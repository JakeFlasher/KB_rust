//! Build canonical card manifests from a directory of `.md` cards.
//!
//! Walks every `*.md` file under `cards_dir`, parses the strict
//! `cacg.v0` frontmatter via [`crate::frontmatter::parse_card`], and
//! emits four artifact families to disk:
//!
//! - `<out_dir>/cards_manifest.json` — sorted list of [`CardManifestEntry`].
//! - `<out_dir>/summaries.json` — sorted list of [`SummaryEntry`].
//! - `<out_dir>/INDEX.md` — markdown rendering grouped by reading_id.
//! - `<card_path>.history.jsonl` (sibling per card) — one history event
//!   when the recomputed `card_hash` differs from the frontmatter's
//!   stored `card_hash` (mirrors Python's `prev_hash != new_hash`
//!   skip semantics; the parity corpus's frontmatter hashes are
//!   correct, so no history file is written on a fresh index).
//!
//! Determinism: every timestamp + event-id origin threads through the
//! caller-supplied [`crate::determinism::DeterminismContext`]. Under
//! frozen mode (`KB_FROZEN_CLOCK=1`) two independent runs over the
//! same input corpus produce byte-identical artifacts.
//!
//! Atomicity: the two-file pair (`cards_manifest.json` +
//! `summaries.json`) commits through [`crate::atomic_publish::atomic_publish`]
//! so a crash between the two `os.rename` calls cannot leave a half-
//! committed pair on disk.

mod delta;
mod history;
mod manifest;
mod rollback;

use std::path::{Path, PathBuf};

use crate::atomic_publish::{atomic_publish, DefaultFs, PublishMember};
use crate::determinism::DeterminismContext;
use crate::frontmatter::parse_card;
use crate::history::{
    history_path_for, last_event_new_card_hash, truncate_history_after_append_if_unchanged,
    HistoryError,
};
use crate::schema::{CardFrontmatter, SchemaVersion};

use delta::{recompute_card_hash, rewrite_card_hash_in_place};
use history::append_per_card_history;
use manifest::canonical_serialize;
use rollback::{read_prior_bytes, rollback_after_partial_index_commit, CommittedCard};

// Re-export public types so `cacg_core::index::Foo` keeps working.
pub(crate) use manifest::build_index_md;
pub use manifest::{CardManifestEntry, CardsManifest, IndexError, SummariesManifest, SummaryEntry};

const SIDECAR_DIAG: &str = "CACG-IDX-007";
const NON_FILE_DIAG: &str = "CACG-IDX-008";

/// Walk `cards_dir`, build the manifests, and publish them atomically
/// into `out_dir`. Per-card history events land alongside each card
/// only when the card's frontmatter `card_hash` differs from the
/// recomputed hash (mirroring Python's `prev_hash != new_hash` skip
/// semantics). Also emits `INDEX.md` next to the manifests.
///
/// Note: does NOT emit `lint_journal.jsonl` (that's `kb lint`'s
/// territory; Python's `kb index` does not write to it either).
fn parse_and_index_card(card_path: &Path) -> Result<IndexedCard, IndexError> {
    let text = std::fs::read_to_string(card_path).map_err(|source| IndexError::Io {
        path: card_path.to_path_buf(),
        source,
    })?;
    let (fm, body) = parse_card(&text).map_err(|diagnostics| IndexError::InvalidFrontmatter {
        path: card_path.to_path_buf(),
        diagnostics,
    })?;
    fm.validate().map_err(|d| IndexError::InvalidFrontmatter {
        path: card_path.to_path_buf(),
        diagnostics: vec![d],
    })?;

    let prev_hash = fm.card_hash.clone();
    let new_hash = recompute_card_hash(&fm, &body)?;

    let citation_count = u32::try_from(fm.citations.len()).unwrap_or(u32::MAX);
    let mut source_ids: Vec<String> = fm.citations.iter().map(|c| c.source_id.clone()).collect();
    source_ids.sort();
    source_ids.dedup();

    let path_str = card_path.to_string_lossy().to_string();

    let manifest_entry = CardManifestEntry {
        schema_version: SchemaVersion::V0,
        path: path_str.clone(),
        id: fm.id.clone(),
        title: fm.title.clone(),
        reading_id: fm.reading_id.clone(),
        summary: fm.summary.clone(),
        card_hash: new_hash.clone(),
        citation_count,
        source_ids: source_ids.clone(),
    };
    let summary_entry = SummaryEntry {
        schema_version: SchemaVersion::V0,
        id: fm.id.clone(),
        title: fm.title.clone(),
        reading_id: fm.reading_id.clone(),
        summary: fm.summary.clone(),
        tags: fm.tags.clone(),
        source_ids,
        path: path_str,
        card_hash: new_hash.clone(),
    };
    Ok(IndexedCard {
        card_path: card_path.to_path_buf(),
        prev_hash,
        new_hash,
        frontmatter: fm,
        manifest_entry,
        summary_entry,
        original_text: text,
    })
}

pub fn build_index(
    cards_dir: &Path,
    out_dir: &Path,
    ctx: &DeterminismContext,
) -> Result<(), IndexError> {
    if !cards_dir.is_dir() {
        return Err(IndexError::NonDirCardsDir(cards_dir.to_path_buf()));
    }
    std::fs::create_dir_all(out_dir).map_err(|source| IndexError::Io {
        path: out_dir.to_path_buf(),
        source,
    })?;

    let card_paths = collect_card_paths(cards_dir, out_dir)?;

    // Per-card row: keeps `prev_hash` (stale stored hash from
    // frontmatter), the parsed frontmatter (used for history-event
    // construction: chunk-id set, frontmatter snapshot), the manifest
    // entry + summary entry (Python-byte-equal artifact shape). The
    // tuple stays large to avoid re-parsing the card later.
    let mut indexed: Vec<IndexedCard> = Vec::with_capacity(card_paths.len());
    for card_path in &card_paths {
        indexed.push(parse_and_index_card(card_path)?);
    }

    indexed.sort_by(|a, b| {
        (&a.manifest_entry.reading_id, &a.manifest_entry.id)
            .cmp(&(&b.manifest_entry.reading_id, &b.manifest_entry.id))
    });

    // Failure-atomicity preflight: before mutating any disk state
    // (manifest publish or per-card frontmatter rewrite), validate the
    // sibling history file for every card whose stored `card_hash`
    // differs from the recomputed value. A tampered history file
    // surfaces here instead of after a card rewrite has already
    // succeeded. Without this gate, a card could end up with a fresh
    // `card_hash` on disk + no audit event because the subsequent
    // append failed -- a later index pass would then see a clean hash
    // and silently skip emitting the missing event. Mirrors Python's
    // `CACG-IDX-004: refusing to commit kb index on top of a tampered
    // history`.
    for entry in &indexed {
        if entry.prev_hash.as_deref() == Some(entry.new_hash.as_str()) {
            continue;
        }
        let history_path = history_path_for(&entry.card_path);
        let bad_lines =
            crate::history::validate_history(&history_path).map_err(IndexError::History)?;
        if !bad_lines.is_empty() {
            return Err(IndexError::History(HistoryError::InvalidExisting {
                path: history_path,
                bad_lines,
            }));
        }
    }

    let cards_manifest = CardsManifest {
        schema_version: SchemaVersion::V0,
        cards: indexed.iter().map(|t| t.manifest_entry.clone()).collect(),
        retracted_cards: Vec::new(),
        dependency_retracted_cards: Vec::new(),
    };
    let summaries_manifest = SummariesManifest {
        schema_version: SchemaVersion::V0,
        summaries: indexed.iter().map(|t| t.summary_entry.clone()).collect(),
    };

    let cards_bytes = canonical_serialize(&cards_manifest)?;
    let summaries_bytes = canonical_serialize(&summaries_manifest)?;

    let manifest_path = out_dir.join("cards_manifest.json");
    let summaries_path = out_dir.join("summaries.json");
    let index_md_path = out_dir.join("INDEX.md");

    // Capture prior bytes of the 3 manifest artifacts BEFORE the
    // atomic_publish that commits them. These snapshots feed the
    // per-card rollback chain below: if any card's iteration fails
    // after this publish has succeeded, the rollback restores each
    // manifest to its pre-call state. `None` means the artifact did
    // not pre-exist; rollback unlinks the newly-published file in
    // that case (BL-20260518-no-prior-pair-atomic). Python achieves
    // the same with `_IndexPairResult` deferred `.bak` cleanup
    // (`legacy_python_oracle/src/cacg/index.py` lines 941-944 + 1027-1050); the Rust port
    // captures the bytes in memory to avoid widening the
    // `atomic_publish` API, which is shared with other callers.
    let manifest_prior_bytes = read_prior_bytes(&manifest_path)?;
    let summaries_prior_bytes = read_prior_bytes(&summaries_path)?;
    let index_md_prior_bytes = read_prior_bytes(&index_md_path)?;

    let index_md_bytes = build_index_md(&cards_manifest.cards);
    let members = vec![
        PublishMember {
            tmp_path: manifest_path.with_extension("json.tmp"),
            bak_path: manifest_path.with_extension("json.bak"),
            canonical_path: manifest_path.clone(),
            bytes: cards_bytes.into_bytes(),
        },
        PublishMember {
            tmp_path: summaries_path.with_extension("json.tmp"),
            bak_path: summaries_path.with_extension("json.bak"),
            canonical_path: summaries_path.clone(),
            bytes: summaries_bytes.into_bytes(),
        },
        PublishMember {
            tmp_path: index_md_path.with_extension("md.tmp"),
            bak_path: index_md_path.with_extension("md.bak"),
            canonical_path: index_md_path.clone(),
            bytes: index_md_bytes.into_bytes(),
        },
    ];
    // Atomic-publish cards_manifest.json + summaries.json + INDEX.md
    // as a 3-file group.
    atomic_publish(&members, &DefaultFs, SIDECAR_DIAG, NON_FILE_DIAG)?;

    // Per-card frontmatter rewrite + history append: only when the
    // stored `prev_hash` (frontmatter's `card_hash` value as read from
    // disk) differs from `new_hash` (the recomputed canonical hash).
    // On a corpus whose frontmatter hashes are already correct, this
    // loop is a no-op -- matching Python's `cards_updated: 0`. On a
    // stale corpus, this rewrites the card's `card_hash` line in place
    // AND appends one history event with deltas computed against the
    // LAST persisted history snapshot (matches Python's delta-against-
    // last-event semantics).
    //
    // Multi-card transaction discipline (mirrors Python's
    // `legacy_python_oracle/src/cacg/index.py` lines 922-1063; BL-20260518-multi-file-
    // replace-rollback applied across the per-card loop): the
    // `committed` accumulator captures each card's pre-iteration
    // state once its rewrite + history append both succeed. On ANY
    // failure path AFTER the manifest publish (this iteration's
    // pre-append I/O, history-append, or rewrite), the `rollback`
    // closure walks `committed` in REVERSE order to restore each
    // prior card's text + history sidecar, then restores the 3
    // manifest artifacts to their pre-`build_index` bytes -- so
    // `kb index` returning Err always leaves the disk in its
    // pre-call state.
    let mut committed: Vec<CommittedCard> = Vec::new();

    let manifest_prior = manifest_prior_bytes.as_deref();
    let summaries_prior = summaries_prior_bytes.as_deref();
    let index_md_prior = index_md_prior_bytes.as_deref();
    // Helper closure: centralizes the 6-arg rollback boilerplate that
    // every post-publish failure path needs. Borrows the manifest paths
    // + prior-bytes captures so callers only thread the (mutable)
    // `committed` accumulator.
    let rollback = |committed: &[CommittedCard]| {
        rollback_after_partial_index_commit(
            committed,
            &manifest_path,
            &summaries_path,
            &index_md_path,
            manifest_prior,
            summaries_prior,
            index_md_prior,
        );
    };

    for entry in &indexed {
        if entry.prev_hash.as_deref() == Some(entry.new_hash.as_str()) {
            continue;
        }

        // Pre-append I/O: read prior card bytes (for the rollback's
        // text restore), stat the history sidecar (for the unlink-on-
        // truncate-to-zero heuristic), and probe the last persisted
        // history event (for the duplicate-suppression check below).
        // Each error routes through `rollback` so manifests + prior
        // committed cards return to their pre-call state.
        let prior_card_text = match std::fs::read(&entry.card_path) {
            Ok(bytes) => bytes,
            Err(source) => {
                rollback(&committed);
                return Err(IndexError::Io {
                    path: entry.card_path.clone(),
                    source,
                });
            }
        };
        let history_path = history_path_for(&entry.card_path);
        // `history_existed_before` is the pre-flock existence
        // observation. The rollback uses it to decide whether to
        // unlink an empty history file after a successful truncate
        // (BL-20260518-no-clobber-foreign-sidecars: don't delete a
        // pre-existing user-owned zero-byte file). The under-lock
        // pre-write size is captured by
        // `append_history_event_with_outcome` and threaded through
        // `AppendOutcome.prior_size_under_lock` so the truncate
        // target is concurrency-safe.
        let history_existed_before = match std::fs::metadata(&history_path) {
            Ok(_) => true,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => false,
            Err(source) => {
                rollback(&committed);
                return Err(IndexError::Io {
                    path: history_path.clone(),
                    source,
                });
            }
        };

        // Decide BEFORE either I/O whether the history append is
        // needed at all: when the last persisted history event already
        // records `new_card_hash == entry.new_hash`, a prior partial
        // commit (crash between history-append and card-rewrite, an
        // older Rust version's bug, or external tampering) left the
        // card stale + history canonical. Skip the duplicate audit
        // event; only the frontmatter needs repair below.
        let history_already_has_new_hash = match last_event_new_card_hash(&history_path) {
            Ok(Some(last)) => last == entry.new_hash,
            Ok(None) => false,
            Err(history_err) => {
                rollback(&committed);
                return Err(IndexError::History(history_err));
            }
        };

        // Order: append history FIRST, rewrite card SECOND. If the
        // process crashes between the two operations after history has
        // been appended, the next index run sees: stale card on disk +
        // history ending with `new_card_hash == entry.new_hash`. The
        // `history_already_has_new_hash` check above then suppresses
        // the duplicate append while the rewrite below repairs the
        // frontmatter. The audit event survives the crash because it
        // landed on disk before the rewrite was attempted.
        //
        // The reverse order (rewrite first) would leave: card with
        // canonical hash + no history event. The next run sees
        // `prev_hash == new_hash` and skips the stale branch entirely,
        // permanently dropping the audit event.
        let append_outcome: Option<crate::history::AppendOutcome> = if !history_already_has_new_hash
        {
            match append_per_card_history(entry, ctx) {
                Ok(outcome) => Some(outcome),
                Err(history_err) => {
                    // History-failure rollback: nothing was
                    // committed for this card (the rewrite hasn't
                    // run yet).
                    rollback(&committed);
                    return Err(history_err);
                }
            }
        } else {
            None
        };

        if let Err(rewrite_err) =
            rewrite_card_hash_in_place(&entry.card_path, &entry.new_hash, &entry.original_text)
        {
            // Rewrite-failure rollback: `atomic_publish` handles its
            // own `.tmp`/`.bak` cleanup on failure, so this card's
            // text is unchanged. If we appended a history event above,
            // safely undo it via `truncate_history_after_append_if_unchanged`,
            // which re-acquires the history flock + revalidates the
            // file size against `outcome.post_write_size` before
            // truncating to `outcome.prior_size_under_lock` -- if
            // another writer has appended after our event, the helper
            // returns `Ok(false)` and the history is left alone so the
            // other writer's chain is preserved. When the helper
            // truncates AND we wrote to a previously-empty file
            // (`!history_existed_before` AND the under-lock signal
            // `target_size == 0`), the helper unlinks the now-empty
            // file UNDER THE FLOCK so a concurrent writer cannot
            // append in the gap (BL-20260518-no-clobber-foreign-sidecars
            // is enforced by the `!history_existed_before` arg).
            if let Some(outcome) = append_outcome {
                let _ = truncate_history_after_append_if_unchanged(
                    &history_path,
                    outcome.prior_size_under_lock,
                    outcome.post_write_size,
                    !history_existed_before,
                );
            }
            rollback(&committed);
            return Err(rewrite_err);
        }

        // Both the history append (if not suppressed) and the rewrite
        // committed for this card. Track its pre-iteration state so a
        // LATER iteration's failure can roll it back.
        // `history_append_outcome` is `Some(outcome)` when we actually
        // appended (carrying the under-lock sizes for the safe
        // truncate-only-our-event rollback) and `None` when the append
        // was suppressed by the idempotency check.
        committed.push(CommittedCard {
            card_path: entry.card_path.clone(),
            prior_card_text,
            history_path,
            history_existed_before,
            history_append_outcome: append_outcome,
        });
    }

    Ok(())
}

/// Per-card row carried through `build_index`. Holds enough context
/// to construct the per-card history event without re-parsing.
/// `original_text` is the byte content of the card file at parse time
/// (the same bytes that fed `recompute_card_hash` to produce
/// `new_hash`). The per-card rewrite path reuses these bytes so the
/// committed `card_hash` always matches the content the manifests +
/// history events describe -- even if the file is concurrently
/// edited between the parse loop and the per-card rewrite. Mirrors
/// Python's `_render_with_hash(doc, new_hash)` (`legacy_python_oracle/src/cacg/index.py`
/// line 1165-1171), which renders from the parse-time `doc` rather
/// than re-reading the file.
struct IndexedCard {
    card_path: PathBuf,
    prev_hash: Option<String>,
    new_hash: String,
    frontmatter: CardFrontmatter,
    manifest_entry: CardManifestEntry,
    summary_entry: SummaryEntry,
    original_text: String,
}

fn collect_card_paths(cards_dir: &Path, out_dir: &Path) -> Result<Vec<PathBuf>, IndexError> {
    let mut out: Vec<PathBuf> = Vec::new();
    let mut stack: Vec<PathBuf> = vec![cards_dir.to_path_buf()];
    let out_canonical = std::fs::canonicalize(out_dir).unwrap_or_else(|_| out_dir.to_path_buf());
    while let Some(dir) = stack.pop() {
        let dir_canonical = std::fs::canonicalize(&dir).unwrap_or_else(|_| dir.clone());
        if dir_canonical == out_canonical {
            continue;
        }
        let entries = std::fs::read_dir(&dir).map_err(|source| IndexError::Io {
            path: dir.clone(),
            source,
        })?;
        for entry in entries {
            let entry = entry.map_err(|source| IndexError::Io {
                path: dir.clone(),
                source,
            })?;
            let p = entry.path();
            let file_type = entry.file_type().map_err(|source| IndexError::Io {
                path: p.clone(),
                source,
            })?;
            if file_type.is_dir() {
                stack.push(p);
            } else if file_type.is_file() && p.extension().and_then(|s| s.to_str()) == Some("md") {
                out.push(p);
            }
        }
    }
    out.sort();
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests;
