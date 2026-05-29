//! Rollback helpers for the per-card transaction in [`super::build_index`].
//!
//! Contains [`CommittedCard`], [`rollback_after_partial_index_commit`],
//! [`rollback_committed_cards`], [`read_prior_bytes`], and
//! [`rollback_manifest`].

use std::path::{Path, PathBuf};

use crate::atomic_publish::{atomic_publish, DefaultFs, PublishMember};
use crate::history::truncate_history_after_append_if_unchanged;

use super::manifest::IndexError;
use super::{NON_FILE_DIAG, SIDECAR_DIAG};

/// Captures the pre-iteration state of a card whose rewrite +
/// history append BOTH committed during the per-card loop in
/// `build_index`. The `committed: Vec<CommittedCard>` accumulator
/// holds one entry per successful iteration; on any later
/// iteration's failure, the accumulator is walked in reverse and
/// each card's text is restored via `atomic_publish` while its
/// history sidecar is truncated (when it pre-existed) or unlinked
/// (when it did not). Mirrors Python's
/// `committed: list[_StagedCardUpdate]` in
/// `legacy_python_oracle/src/cacg/index.py` lines 919-1024.
pub(super) struct CommittedCard {
    pub(super) card_path: PathBuf,
    pub(super) prior_card_text: Vec<u8>,
    pub(super) history_path: PathBuf,
    pub(super) history_existed_before: bool,
    /// `Some(outcome)` when this card's iteration appended a fresh
    /// history event; `None` when the iteration suppressed the
    /// append because the last persisted event already recorded
    /// `entry.new_hash`. The rollback uses
    /// `truncate_history_after_append_if_unchanged` to safely undo
    /// only OUR appended event, leaving any concurrently-appended
    /// events intact.
    pub(super) history_append_outcome: Option<crate::history::AppendOutcome>,
}

/// Centralized rollback chain for any post-manifest-publish failure
/// in `build_index`: walk the `committed` accumulator backwards
/// (restoring each card's text + truncating its history sidecar),
/// then restore the 3 manifest artifacts to their pre-call bytes
/// (or unlink them when no prior version existed). Used by every
/// failure branch that fires AFTER the manifest atomic_publish so
/// `kb index` returning Err always leaves the disk in its pre-call
/// state. Rollback I/O errors are intentionally swallowed; the
/// caller's original error must remain the value returned.
pub(super) fn rollback_after_partial_index_commit(
    committed: &[CommittedCard],
    manifest_path: &Path,
    summaries_path: &Path,
    index_md_path: &Path,
    manifest_prior_bytes: Option<&[u8]>,
    summaries_prior_bytes: Option<&[u8]>,
    index_md_prior_bytes: Option<&[u8]>,
) {
    rollback_committed_cards(committed);
    rollback_manifest(manifest_path, "json.tmp", "json.bak", manifest_prior_bytes);
    rollback_manifest(
        summaries_path,
        "json.tmp",
        "json.bak",
        summaries_prior_bytes,
    );
    rollback_manifest(index_md_path, "md.tmp", "md.bak", index_md_prior_bytes);
}

pub(super) fn rollback_committed_cards(committed: &[CommittedCard]) {
    for c in committed.iter().rev() {
        let restore_members = vec![PublishMember {
            tmp_path: c.card_path.with_extension("md.tmp"),
            bak_path: c.card_path.with_extension("md.bak"),
            canonical_path: c.card_path.clone(),
            bytes: c.prior_card_text.clone(),
        }];
        // If `atomic_publish` fails (e.g., a foreign `.md.tmp`/`.md.bak`
        // sidecar appeared or a permission issue blocked the replace),
        // the card stays at the post-rewrite `new_hash`. We must NOT
        // then truncate the history sidecar -- doing so would leave
        // the card at `new_hash` with NO audit event, and the next
        // `kb index` would see `prev_hash == new_hash` and permanently
        // skip the stale branch (audit trail lost). Preserving the
        // history keeps the canonical event in place so the next
        // index run sees a consistent "card at new_hash + history
        // canonical event" pair.
        let card_restored =
            atomic_publish(&restore_members, &DefaultFs, SIDECAR_DIAG, NON_FILE_DIAG).is_ok();
        if !card_restored {
            continue;
        }
        if let Some(outcome) = c.history_append_outcome {
            // The post-truncate unlink runs UNDER the same flock as
            // the truncate via the helper's `unlink_if_truncated_to_zero`
            // parameter. Passing `!c.history_existed_before` preserves
            // the pre-flock BL-20260518-no-clobber-foreign-sidecars
            // guard; the helper additionally checks `target_size == 0`
            // (under-lock prior-size signal) so the unlink only fires
            // when we wrote to a previously-empty file.
            let _ = truncate_history_after_append_if_unchanged(
                &c.history_path,
                outcome.prior_size_under_lock,
                outcome.post_write_size,
                !c.history_existed_before,
            );
        }
    }
}

/// Read the prior bytes of a published-artifact path for the per-card
/// history-append rollback. Returns `Ok(None)` if the path did not
/// pre-exist (so the rollback will unlink the newly-published file
/// instead of restoring prior bytes -- mirrors
/// BL-20260518-no-prior-pair-atomic for the first-ever-index case).
/// A path that exists but is NOT a regular file (e.g. a directory)
/// also returns `Ok(None)` so the subsequent `atomic_publish` call
/// can run its `NonFileCanonical` preflight and produce the
/// documented `CACG-IDX-008` diagnostic; pre-reading via
/// `std::fs::read(directory)` would otherwise surface as
/// `IndexError::Io` (EISDIR on Linux) BEFORE the publisher gets a
/// chance to emit the shape-check error. Other I/O errors continue
/// to surface as `IndexError::Io` so the caller fails fast.
pub(super) fn read_prior_bytes(path: &Path) -> Result<Option<Vec<u8>>, IndexError> {
    match std::fs::metadata(path) {
        Ok(m) if m.is_file() => match std::fs::read(path) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(source) => Err(IndexError::Io {
                path: path.to_path_buf(),
                source,
            }),
        },
        Ok(_) => Ok(None),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(IndexError::Io {
            path: path.to_path_buf(),
            source,
        }),
    }
}

/// Restore a published manifest artifact to its pre-`build_index`
/// state during per-card history-append rollback. When `prior_bytes`
/// is `Some(bytes)`, re-publishes those bytes via the existing
/// `atomic_publish` framework so the restore is itself durable. When
/// `prior_bytes` is `None`, the artifact did not pre-exist, so the
/// rollback unlinks the newly-published file. Rollback I/O errors
/// are intentionally swallowed; the caller's original `HistoryError`
/// must remain the returned value.
pub(super) fn rollback_manifest(
    canonical: &Path,
    tmp_ext: &str,
    bak_ext: &str,
    prior_bytes: Option<&[u8]>,
) {
    match prior_bytes {
        Some(bytes) => {
            let restore_members = vec![PublishMember {
                tmp_path: canonical.with_extension(tmp_ext),
                bak_path: canonical.with_extension(bak_ext),
                canonical_path: canonical.to_path_buf(),
                bytes: bytes.to_vec(),
            }];
            let _ = atomic_publish(&restore_members, &DefaultFs, SIDECAR_DIAG, NON_FILE_DIAG);
        }
        None => {
            let _ = std::fs::remove_file(canonical);
        }
    }
}
