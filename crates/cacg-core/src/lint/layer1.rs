//! Layer-1 mechanical lint pass mirroring Python
//! `legacy_python_oracle/src/cacg/lint/layer1.py::run_layer1_checks` byte-equal on the
//! trust-bearing CACG-* subset.
//!
//! Boundary with the M1 schema / frontmatter modules:
//!
//! * Pydantic-equivalent field validation (chunk_hash 64-hex, page_range
//!   ordering, schema_version, etc.) is handled upstream in
//!   `crate::frontmatter::parse_card` and surfaces as `CACG-FM-*` /
//!   `CACG-CITE-002` / `CACG-CITE-003` via `CardLoadError`.
//! * Layer-1 lint adds cross-reference checks against `chunks_manifest`:
//!   chunk_id presence, source/chunk retraction, source-id agreement,
//!   stored-hash drift, page-span intersection, and authorization.
//! * `CACG-HASH-002` (stale `card_hash`) is layer-1's only card-level
//!   diagnostic.
//!
//! Per the AC-2.1 carve-out: the new `cacg-core::lint::layer1` surface
//! never emits `CACG-SUM-*`, `CACG-SKILL-*`, `CACG-DEP-*`, or
//! `CACG-ROLE-*` from its own code paths. Carry-through `CACG-SUM-*`
//! diagnostics from `CardLoadError` are produced by the existing M1
//! frontmatter parity surface — a different surface — and pass through
//! to the caller as part of the loader-error short-circuit.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::OnceLock;
use std::time::Instant;

use regex::Regex;
use serde_json::Value;
use thiserror::Error;

use crate::card_loader::{load_card, CardDoc};
use crate::chunks_index::{ChunksIndex, ChunksIndexLoadError};
use crate::determinism::DeterminismContext;
use crate::diagnostic::{codes, Diagnostic, Severity};
use crate::hash::card_hash;
use crate::journal::{append_entry, JournalEntry, JournalError};
use crate::schema::CardFrontmatter;
use crate::source_matrix::{is_citation_authorized, AuthSpec};

/// Result of a layer-1 lint pass over one card. Mirrors Python
/// `run_layer1_checks`'s `(diagnostics, card_hash_before, doc_or_none)`
/// triple.
#[derive(Debug, Default)]
pub struct Layer1Result {
    /// Diagnostics emitted in source-order. Trust-bearing codes only.
    pub diagnostics: Vec<Diagnostic>,
    /// The `card_hash` field as it appeared in the card's frontmatter
    /// at load time. `None` when load failed or the field was absent.
    pub card_hash_before: Option<String>,
    /// The parsed card document. `Some(_)` when load succeeded; `None`
    /// when load failed.
    pub doc: Option<CardDoc>,
}

fn chunk_id_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[a-z0-9_]+:p\d+:\d+$").expect("chunk_id regex is valid"))
}

fn hex64_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[0-9a-f]{64}$").expect("hex64 regex is valid"))
}

const PLACEHOLDER_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// Run the layer-1 lint pass for a single card. Mirrors Python
/// `legacy_python_oracle/src/cacg/lint/layer1.py::run_layer1_checks` per the audit at
/// `_research/12_lint_diagnostic_order_audit.md`.
///
/// Per-card emission order:
///
/// 1. Load card. Loader failure short-circuits with loader diagnostics.
/// 2. Set `card_hash_before` from `doc.frontmatter.card_hash`.
/// 3. Build or borrow `ChunksIndex`. Per-card load failure short-circuits
///    with `CACG-MAN-001`.
/// 4. If `auth.load_error` is set, append one per-card `CACG-AUTH-000`.
///    Does NOT suppress citation checks.
/// 5. Run the per-citation loop (`check_citations`). Each failing
///    citation emits exactly one diagnostic and moves on.
/// 6. Run the card-hash staleness check (`CACG-HASH-002`). Cards with
///    no stored `card_hash` pass silently.
///
/// `run_layer1_checks` itself does NOT append to the journal; the CLI
/// wrapper (task-vh-7) owns journal cardinality.
#[must_use]
pub fn run_layer1_checks(
    card_path: &Path,
    chunks_manifest_path: &Path,
    chunks_index: Option<&ChunksIndex>,
    auth: Option<&AuthSpec>,
) -> Layer1Result {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    let card_hash_before;

    let doc = match load_card(card_path) {
        Ok(d) => d,
        Err(err) => {
            // Loader diagnostics already carry the `file` field via
            // `card_loader::load_card`. Carry-through frontmatter
            // diagnostics (CACG-FM-*, CACG-CITE-002, CACG-CITE-003,
            // CACG-SUM-001..004) are emitted by the M1 frontmatter
            // surface, NOT by run_layer1_checks itself, so AC-2.1's
            // carve-out is preserved.
            diagnostics.extend(err.diagnostics);
            return Layer1Result {
                diagnostics,
                card_hash_before: None,
                doc: None,
            };
        }
    };
    card_hash_before = doc.frontmatter.card_hash.clone();

    let file_path = card_path.display().to_string();

    let owned_index;
    let idx: &ChunksIndex = match chunks_index {
        Some(i) => i,
        None => match ChunksIndex::from_path(chunks_manifest_path) {
            Ok(i) => {
                owned_index = i;
                &owned_index
            }
            Err(err) => {
                diagnostics.push(
                    Diagnostic::new(codes::MAN_001, Severity::Error, format_manifest_error(&err))
                        .with_file(file_path.clone()),
                );
                return Layer1Result {
                    diagnostics,
                    card_hash_before,
                    doc: Some(doc),
                };
            }
        },
    };

    if let Some(a) = auth {
        if let Some(load_err) = &a.load_error {
            diagnostics.push(
                Diagnostic::new(codes::AUTH_000, Severity::Error, load_err.clone())
                    .with_file(file_path.clone()),
            );
        }
    }

    check_citations(&doc, idx, &file_path, auth, &mut diagnostics);

    if let Some(d) = check_card_hash(&doc, &file_path) {
        diagnostics.push(d);
    }

    Layer1Result {
        diagnostics,
        card_hash_before,
        doc: Some(doc),
    }
}

fn format_manifest_error(err: &ChunksIndexLoadError) -> String {
    match err {
        ChunksIndexLoadError::DuplicateChunkId(ids) => format!(
            "chunks_manifest.json has duplicate chunk_id(s): {ids:?}; refusing to build an ambiguous index"
        ),
        ChunksIndexLoadError::NonFile(_)
        | ChunksIndexLoadError::Io { .. }
        | ChunksIndexLoadError::Parse(_)
        | ChunksIndexLoadError::Validate(_)
        | ChunksIndexLoadError::HashRecompute { .. }
        | ChunksIndexLoadError::MissingChunk(_) => {
            format!("chunks_manifest.json is invalid: {err}")
        }
    }
}

fn check_citations(
    doc: &CardDoc,
    idx: &ChunksIndex,
    file_path: &str,
    auth: Option<&AuthSpec>,
    out: &mut Vec<Diagnostic>,
) {
    let reading_id = doc.frontmatter.reading_id.as_str();
    for (i, cit) in doc.frontmatter.citations.iter().enumerate() {
        let loc = format!("citations[{i}]");

        // CITE-001 chunk_id syntax (covers placeholder strings emitted by `kb new`).
        if !chunk_id_re().is_match(&cit.chunk_id) {
            out.push(
                Diagnostic::new(
                    codes::CITE_001,
                    Severity::Error,
                    format!("{loc}: malformed chunk_id '{cid}'", cid = cit.chunk_id),
                )
                .with_file(file_path.to_string()),
            );
            continue;
        }

        // CITE-002 placeholder all-zero hash (template emits "0"*64).
        if cit.chunk_hash == PLACEHOLDER_HASH {
            out.push(
                Diagnostic::new(
                    codes::CITE_002,
                    Severity::Error,
                    format!(
                        "{loc}: placeholder chunk_hash (all-zero) is not a real chunk reference"
                    ),
                )
                .with_file(file_path.to_string()),
            );
            continue;
        }

        // CITE-002 non-64-hex hash (defense-in-depth; usually unreachable
        // for file-loaded cards because schema validation maps the same
        // failure earlier).
        if !hex64_re().is_match(&cit.chunk_hash) {
            out.push(
                Diagnostic::new(
                    codes::CITE_002,
                    Severity::Error,
                    format!("{loc}: chunk_hash must be 64-hex"),
                )
                .with_file(file_path.to_string()),
            );
            continue;
        }

        // RETR-002 / RETR-003 fire BEFORE manifest-presence (CITE-004)
        // because retracted chunks are removed from active
        // `chunks_manifest.chunks`. If we checked manifest-presence
        // first, retracted citations would emit CITE-004 instead of the
        // more specific retraction code.
        if idx.is_source_retracted(&cit.source_id) {
            out.push(
                Diagnostic::new(
                    codes::RETR_002,
                    Severity::Error,
                    format!(
                        "{loc}: source_id '{sid}' has been retracted (in chunks_manifest.retracted_source_ids); refusing to verify",
                        sid = cit.source_id
                    ),
                )
                .with_file(file_path.to_string()),
            );
            continue;
        }
        if idx.is_retracted(&cit.chunk_id) {
            out.push(
                Diagnostic::new(
                    codes::RETR_003,
                    Severity::Error,
                    format!(
                        "{loc}: chunk_id '{cid}' has been retracted (in chunks_manifest.retracted_chunk_ids); refusing to verify",
                        cid = cit.chunk_id
                    ),
                )
                .with_file(file_path.to_string()),
            );
            continue;
        }

        // CITE-004 chunk_id presence in manifest.
        let chunk = match idx.get(&cit.chunk_id) {
            Some(c) => c,
            None => {
                out.push(
                    Diagnostic::new(
                        codes::CITE_004,
                        Severity::Error,
                        format!(
                            "{loc}: chunk_id '{cid}' not present in chunks manifest",
                            cid = cit.chunk_id
                        ),
                    )
                    .with_file(file_path.to_string()),
                );
                continue;
            }
        };

        // CITE-006 source_id agreement (defensive: chunk_id should
        // already imply the source via its prefix, but the citation
        // carries an explicit field).
        if cit.source_id != chunk.source_id {
            out.push(
                Diagnostic::new(
                    codes::CITE_006,
                    Severity::Error,
                    format!(
                        "{loc}: source_id '{cit_sid}' disagrees with chunk source_id '{chunk_sid}'",
                        cit_sid = cit.source_id,
                        chunk_sid = chunk.source_id
                    ),
                )
                .with_file(file_path.to_string()),
            );
            continue;
        }

        // HASH-001 stored-hash drift detection.
        if cit.chunk_hash != chunk.chunk_hash {
            out.push(
                Diagnostic::new(
                    codes::HASH_001,
                    Severity::Error,
                    format!(
                        "{loc}: chunk_hash mismatch (citation={cit_h}, manifest={chunk_h}); source may have drifted",
                        cit_h = cit.chunk_hash,
                        chunk_h = chunk.chunk_hash
                    ),
                )
                .with_file(file_path.to_string()),
            );
            continue;
        }

        // CITE-005 page-range intersection. Uses the actual set of
        // pages from chunk.page_spans, NOT a start_page..end_page
        // interval approximation — non-contiguous chunks (e.g.,
        // page_spans=[1,3] should reject page_range=[2,2]) require the
        // set-based check.
        let chunk_pages: std::collections::BTreeSet<i64> =
            chunk.page_spans.iter().map(|s| i64::from(s.page)).collect();
        let start = cit.page_range.first().copied().unwrap_or(0);
        let end = cit.page_range.get(1).copied().unwrap_or(start);
        let mut overlap = false;
        for p in start..=end {
            if chunk_pages.contains(&p) {
                overlap = true;
                break;
            }
        }
        if !overlap {
            let actual: Vec<i64> = chunk_pages.into_iter().collect();
            out.push(
                Diagnostic::new(
                    codes::CITE_005,
                    Severity::Error,
                    format!(
                        "{loc}: page_range [{start}, {end}] is disjoint from chunk pages {actual:?}"
                    ),
                )
                .with_file(file_path.to_string()),
            );
            continue;
        }

        // AUTH-001 / AUTH-002 layer-1 check (opt-in via --source-matrix).
        if let Some(a) = auth {
            if let Some(matrix) = &a.matrix {
                let (ok, code) = is_citation_authorized(matrix, reading_id, &cit.source_id);
                if !ok {
                    let resolved_code = code.unwrap_or(codes::AUTH_002);
                    let msg = if resolved_code == codes::AUTH_001 {
                        format!(
                            "{loc}: reading_id '{reading_id}' not present in source_matrix; refusing to verify"
                        )
                    } else {
                        // qg-allow: deep-nesting — citation-auth else arm
                        format!(
                            "{loc}: source_id '{sid}' not authorized for reading_id '{reading_id}' by source_matrix",
                            sid = cit.source_id
                        )
                    };
                    out.push(
                        Diagnostic::new(resolved_code, Severity::Error, msg)
                            .with_file(file_path.to_string()),
                    );
                }
            }
        }
    }
}

fn check_card_hash(doc: &CardDoc, file_path: &str) -> Option<Diagnostic> {
    let stored = doc.frontmatter.card_hash.as_ref()?;
    let fm_map = frontmatter_to_map(&doc.frontmatter).ok()?;
    let recomputed = card_hash(&fm_map, &doc.body_raw).ok()?;
    if stored != &recomputed {
        return Some(
            Diagnostic::new(
                codes::HASH_002,
                Severity::Error,
                format!(
                    "card_hash is stale: stored={stored_prefix}..., recomputed={actual_prefix}...; run `kb index` after editing the card",
                    stored_prefix = &stored[..12.min(stored.len())],
                    actual_prefix = &recomputed[..12.min(recomputed.len())]
                ),
            )
            .with_file(file_path.to_string()),
        );
    }
    None
}

#[allow(clippy::wildcard_enum_match_arm)]
fn frontmatter_to_map(
    fm: &CardFrontmatter,
) -> Result<serde_json::Map<String, serde_json::Value>, serde_json::Error> {
    let value = serde_json::to_value(fm)?;
    Ok(match value {
        serde_json::Value::Object(map) => map,
        _ => serde_json::Map::new(),
    })
}

/// Wall-clock latency in ms; `0.0` when `KB_FROZEN_CLOCK=1` is set.
/// Mirrors Python `legacy_python_oracle/src/cacg/lint/layer1.py::freeze_aware_latency`.
/// Shared with the future Layer-2 verify runner (task-vh-15).
#[must_use]
pub fn freeze_aware_latency(start: Instant) -> f64 {
    if DeterminismContext::from_env().is_frozen() {
        0.0
    } else {
        start.elapsed().as_secs_f64() * 1000.0
    }
}

/// Outcome of [`lint_card`]: the diagnostics, the failure flag, and the
/// parsed [`CardDoc`] (`Some` iff `load_card` succeeded).
///
/// Mirrors Python `lint_card`'s `(diagnostics, failed, doc)` tuple.
#[derive(Debug)]
pub struct LintCardOutcome {
    /// Diagnostics emitted by `run_layer1_checks`.
    pub diagnostics: Vec<Diagnostic>,
    /// `true` iff any diagnostic has `severity == Error`.
    pub failed: bool,
    /// Parsed card document; `Some` when load succeeded.
    pub doc: Option<CardDoc>,
}

/// Errors raised by [`lint_card`] when the journal append itself fails.
/// Diagnostic-level lint failures are NOT errors; they appear in
/// [`LintCardOutcome::diagnostics`].
#[derive(Debug, Error)]
pub enum LintCardError {
    /// The journal append refused or failed at the I/O layer.
    #[error("lint journal append failed: {0}")]
    Journal(#[from] JournalError),
}

/// Aggregate result from [`lint_directory`]: every per-card diagnostic
/// concatenated in walk order, and the OR of every per-card failure
/// flag.
#[derive(Debug, Default)]
pub struct LintDirectoryOutcome {
    /// Diagnostics from every card visit (plus the preflight
    /// `CACG-MAN-001` row if `cards_manifest.json` is malformed).
    pub diagnostics: Vec<Diagnostic>,
    /// `true` iff any per-card lint failed OR the cards-manifest
    /// preflight emitted an error.
    pub failed: bool,
}

/// Errors raised by [`lint_directory`] when the directory itself
/// cannot be walked. Per-card lint failures live in
/// [`LintDirectoryOutcome::diagnostics`], not here.
#[derive(Debug, Error)]
pub enum LintDirectoryError {
    /// `cards_dir` is not a directory (missing path, regular file,
    /// broken symlink, etc.). The CLI dispatcher maps this to
    /// `CACG-CLI-001`.
    #[error("cards_dir {0:?} is not a directory")]
    NonDir(std::path::PathBuf),
    /// I/O failure walking the cards directory.
    #[error("cards directory I/O error at {path:?}: {source}")]
    Io {
        /// The path that errored.
        path: std::path::PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// A per-card [`lint_card`] call returned a journal-append failure.
    #[error("lint journal append failed during batch lint: {0}")]
    Journal(#[from] LintCardError),
}

/// Journal-writing wrapper around [`run_layer1_checks`].
///
/// Mirrors Python `legacy_python_oracle/src/cacg/lint/layer1.py::lint_card` byte-equal: the
/// per-card journal cardinality contract is exactly one
/// `command="lint"` entry per call, regardless of whether the lint
/// passed or failed.
///
/// `card_hash_before == card_hash_after` because lint does NOT mutate
/// the card. `verification` carries `layer1: !failed`, `layer2: false`,
/// `fuzzy: false`. `latency_ms` is `0.0` under `KB_FROZEN_CLOCK=1`.
///
/// # Errors
///
/// Returns [`LintCardError::Journal`] when the journal append refuses
/// or fails at the I/O layer. Lint-level diagnostic errors are NOT
/// errors of this function; they're carried in
/// `LintCardOutcome.diagnostics`.
pub fn lint_card(
    card_path: &Path,
    chunks_manifest_path: &Path,
    journal_path: &Path,
    chunks_index: Option<&ChunksIndex>,
    auth: Option<&AuthSpec>,
) -> Result<LintCardOutcome, LintCardError> {
    let start = Instant::now();
    let result = run_layer1_checks(card_path, chunks_manifest_path, chunks_index, auth);
    let failed = result
        .diagnostics
        .iter()
        .any(|d| matches!(d.severity, Severity::Error));

    let ctx = DeterminismContext::from_env();
    let diagnostics_json: Vec<Value> = result
        .diagnostics
        .iter()
        .map(|d| serde_json::to_value(d).unwrap_or(Value::Null))
        .collect();
    let mut verification: BTreeMap<String, bool> = BTreeMap::new();
    verification.insert("layer1".to_string(), !failed);
    verification.insert("layer2".to_string(), false);
    verification.insert("fuzzy".to_string(), false);

    let entry = JournalEntry {
        command: "lint".to_string(),
        card_path: card_path.display().to_string(),
        card_hash_before: result.card_hash_before.clone(),
        // Lint does NOT mutate the card; before == after.
        card_hash_after: result.card_hash_before.clone(),
        diagnostics: diagnostics_json,
        verification,
        latency_ms: freeze_aware_latency(start),
    };
    let event_id = ctx.new_uuid();
    let timestamp = ctx.now_iso();
    append_entry(journal_path, &entry, &event_id, &timestamp)?;

    Ok(LintCardOutcome {
        diagnostics: result.diagnostics,
        failed,
        doc: result.doc,
    })
}

/// Batch wrapper around [`lint_card`]: walk `cards_dir` recursively
/// for `*.md` files (skipping `SKILL.md` router files), preload
/// `ChunksIndex` once, then run `lint_card` per card so each card
/// produces exactly one `command="lint"` journal event.
///
/// Mirrors Python `legacy_python_oracle/src/cacg/lint/layer1.py::lint_directory:396-535`
/// on the trust-bearing subset only — the auxiliary
/// `validate_skill_routers` / `validate_role_maps` / `validate_card_dag`
/// aggregators are intentionally OMITTED per AC-2.1 (the carve-out
/// doc + harness annotation land in task-vh-8). The pre-Round-6
/// reconnaissance confirmed both committed corpora
/// (`tests/parity_corpus/{valid,adversarial}/`) emit only
/// trust-bearing codes through this path, so byte-equal parity holds
/// without the annotation system.
///
/// # Errors
///
/// Returns [`LintDirectoryError::NonDir`] when `cards_dir` does not
/// exist or is not a directory (caller dispatcher maps to
/// `CACG-CLI-001`); [`LintDirectoryError::Io`] on a filesystem walk
/// failure; or [`LintDirectoryError::Journal`] when a per-card journal
/// append refuses or fails.
pub fn lint_directory(
    cards_dir: &Path,
    chunks_manifest_path: &Path,
    journal_path: &Path,
    auth: Option<&AuthSpec>,
) -> Result<LintDirectoryOutcome, LintDirectoryError> {
    if !cards_dir.is_dir() {
        return Err(LintDirectoryError::NonDir(cards_dir.to_path_buf()));
    }

    // Preload the chunks index once per batch. On load failure the
    // index is left as None so each per-card `lint_card` call retries
    // the manifest load and surfaces a per-card `CACG-MAN-001` journal
    // event (preserves per-card cardinality on the failure path,
    // mirroring Python `lint_directory` lines 432-435).
    let preloaded_index = ChunksIndex::from_path(chunks_manifest_path).ok(); // qg-allow: intentional-discard — chunks index is optional for linting

    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    let mut failed = false;

    // Preflight: if `cards_manifest.json` sits next to the chunks
    // manifest and is present-but-invalid — unreadable, unparseable,
    // OR structurally invariant-invalid — emit one `CACG-MAN-001` and
    // continue. The clean / absent paths produce no preflight
    // diagnostic. `serde_json::from_str` enforces only JSON shape; the
    // ported `CardsManifest::validate_structurally` mirrors the Python
    // Pydantic invariant validators, so an invariant-invalid manifest
    // (that Python's `CardsManifest.model_validate_json` rejects) takes
    // the same preflight path as an unparseable one. Mirrors Python
    // `lint_directory:447-490`.
    if let Some(parent) = chunks_manifest_path.parent() {
        let cards_manifest_path = parent.join("cards_manifest.json");
        if cards_manifest_path.is_file() {
            match std::fs::read_to_string(&cards_manifest_path) {
                Ok(raw) => {
                    let validated = serde_json::from_str::<crate::index::CardsManifest>(&raw)
                        .map_err(|e| e.to_string())
                        .and_then(|m| m.validate_structurally().map_err(|d| d.message));
                    if let Err(err) = validated {
                        diagnostics.push(
                            Diagnostic::new(
                                codes::MAN_001,
                                Severity::Error,
                                format!(
                                    "cards_manifest.json could not be read or validated; DAG and SKILL.md checks fall back to filesystem walk: {err}"
                                ),
                            )
                            .with_file(cards_manifest_path.display().to_string()),
                        );
                        failed = true;
                    }
                }
                Err(err) => {
                    diagnostics.push(
                        Diagnostic::new(
                            codes::MAN_001,
                            Severity::Error,
                            format!(
                                "cards_manifest.json could not be read or validated; DAG and SKILL.md checks fall back to filesystem walk: {err}"
                            ),
                        )
                        .with_file(cards_manifest_path.display().to_string()),
                    );
                    failed = true;
                }
            }
        }
    }

    // Walk recursively for `*.md` files and sort the resulting paths
    // (mirrors Python's `sorted(cards_dir_p.rglob("*.md"))`).
    let mut card_paths: Vec<std::path::PathBuf> = Vec::new();
    walk_md_files(cards_dir, &mut card_paths)?;
    card_paths.sort();

    for card_path in &card_paths {
        // Skip `SKILL.md` files (mirrors Python `is_skill_router_path`).
        // The auxiliary SKILL-* aggregator is OUT OF SCOPE per AC-2.1,
        // but we still skip the file so it doesn't get linted as a
        // card and emit spurious frontmatter diagnostics.
        if card_path.file_name() == Some(std::ffi::OsStr::new("SKILL.md")) {
            continue;
        }
        let outcome = lint_card(
            card_path,
            chunks_manifest_path,
            journal_path,
            preloaded_index.as_ref(),
            auth,
        )?;
        diagnostics.extend(outcome.diagnostics);
        if outcome.failed {
            failed = true;
        }
    }

    Ok(LintDirectoryOutcome {
        diagnostics,
        failed,
    })
}

fn walk_md_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) -> Result<(), LintDirectoryError> {
    let entries = std::fs::read_dir(dir).map_err(|source| LintDirectoryError::Io {
        path: dir.to_path_buf(),
        source,
    })?;
    for entry in entries {
        let entry = entry.map_err(|source| LintDirectoryError::Io {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        let ft = entry.file_type().map_err(|source| LintDirectoryError::Io {
            path: path.clone(),
            source,
        })?;
        if ft.is_dir() {
            walk_md_files(&path, out)?;
        } else if ft.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            out.push(path);
        }
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::determinism::with_frozen_clock;
    use crate::schema::{ChunkRecord, ChunksManifest, PageSpan as SchemaPageSpan, SchemaVersion};
    use std::fs;
    use std::path::PathBuf;

    struct Fixture {
        _dir: tempfile::TempDir,
        card_path: PathBuf,
        manifest_path: PathBuf,
    }

    fn workspace_root() -> PathBuf {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.pop();
        p.pop();
        p
    }

    fn write_card(dir: &Path, name: &str, body: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, body).unwrap();
        path
    }

    fn write_manifest(dir: &Path, manifest: &ChunksManifest) -> PathBuf {
        let path = dir.join("chunks_manifest.json");
        fs::write(&path, serde_json::to_string(manifest).unwrap()).unwrap();
        path
    }

    fn sample_chunk(chunk_id: &str, source_id: &str, text: &str) -> ChunkRecord {
        let span = SchemaPageSpan {
            page: 1,
            byte_offset_in_chunk: 0,
        };
        let hash_span = crate::hash::PageSpan {
            page: 1,
            byte_offset_in_chunk: 0,
        };
        let h = crate::hash::chunk_hash(text, 1, 1, &[hash_span]).unwrap();
        ChunkRecord {
            schema_version: SchemaVersion::V0,
            source_id: source_id.to_string(),
            chunk_id: chunk_id.to_string(),
            chunk_hash: h,
            ordinal: 0,
            start_page: 1,
            end_page: 1,
            page_spans: vec![span],
            token_count: 3,
            text: text.to_string(),
            text_preview: text.to_string(),
        }
    }

    fn manifest_with(chunks: Vec<ChunkRecord>) -> ChunksManifest {
        ChunksManifest {
            schema_version: SchemaVersion::V0,
            chunks,
            retracted_source_ids: Vec::new(),
            retracted_chunk_ids: Vec::new(),
        }
    }

    fn card_md(chunk_hash: &str, chunk_id: &str, source_id: &str) -> String {
        // chunk_hash is YAML-quoted because all-digit strings would
        // otherwise parse as integers (the actual M0 corpus quotes
        // hashes too — see `tests/parity_corpus/valid/01-*.md`).
        format!(
            r#"---
schema_version: cacg.v0
id: test-card
title: Test Card
reading_id: reading_01
summary: This is a summary that is at least eighty characters long so the schema validator passes the SUM-001 minimum-length check, padded a bit.
citations:
  - source_id: "{source_id}"
    chunk_id: "{chunk_id}"
    chunk_hash: "{chunk_hash}"
    page_range: [1, 1]
    quote: "alpha beta gamma"
---
# Test Card

This is the body.
"#
        )
    }

    fn build_fixture(chunk_text: &str, chunk_id: &str, source_id: &str) -> Fixture {
        let dir = tempfile::tempdir().unwrap();
        let chunk = sample_chunk(chunk_id, source_id, chunk_text);
        let chunk_hash = chunk.chunk_hash.clone();
        let manifest = manifest_with(vec![chunk]);
        let manifest_path = write_manifest(dir.path(), &manifest);
        let card_path = write_card(
            dir.path(),
            "card.md",
            &card_md(&chunk_hash, chunk_id, source_id),
        );
        Fixture {
            _dir: dir,
            card_path,
            manifest_path,
        }
    }

    fn run(fixture: &Fixture, auth: Option<&AuthSpec>) -> Layer1Result {
        run_layer1_checks(&fixture.card_path, &fixture.manifest_path, None, auth)
    }

    #[test]
    fn clean_card_produces_no_diagnostics() {
        let fx = build_fixture("alpha beta gamma", "src:p001:0000", "src");
        let result = run(&fx, None);
        assert!(
            result.diagnostics.is_empty(),
            "clean card must produce no diagnostics, got {:?}",
            result.diagnostics
        );
        assert!(result.doc.is_some());
    }

    #[test]
    fn card_load_failure_short_circuits() {
        let dir = tempfile::tempdir().unwrap();
        let card_path = dir.path().join("nope.md");
        let manifest_path = dir.path().join("manifest.json");
        let result = run_layer1_checks(&card_path, &manifest_path, None, None);
        assert!(result.doc.is_none());
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.diagnostics[0].code, codes::CLI_001);
    }

    #[test]
    fn manifest_load_failure_emits_man_001_and_short_circuits() {
        let dir = tempfile::tempdir().unwrap();
        let card_path = write_card(
            dir.path(),
            "card.md",
            &card_md(
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "src:p001:0000",
                "src",
            ),
        );
        let manifest_path = dir.path().join("missing_manifest.json");
        let result = run_layer1_checks(&card_path, &manifest_path, None, None);
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.diagnostics[0].code, codes::MAN_001);
        assert!(result.doc.is_some());
        assert!(result.card_hash_before.is_none());
    }

    #[test]
    fn preflight_rejects_structurally_invalid_cards_manifest() {
        // A sibling `cards_manifest.json` that deserializes but
        // violates a `CardsManifest` invariant (here: a card_id in
        // both `cards` and `retracted_cards`) must take the SAME
        // preflight path as an unparseable manifest: exactly one
        // `CACG-MAN-001`, `failed` set, and the filesystem walk still
        // runs. Before Round 9 the preflight only checked
        // `serde_json::from_str` and let this manifest through.
        let dir = tempfile::tempdir().unwrap();
        let cards_dir = dir.path().join("cards");
        fs::create_dir_all(&cards_dir).unwrap();
        let manifest_path = write_manifest(dir.path(), &manifest_with(vec![]));
        let card_hash = "0".repeat(64);
        let invalid = serde_json::json!({
            "schema_version": "cacg.v0",
            "cards": [{
                "schema_version": "cacg.v0",
                "path": "cards/x.md",
                "id": "overlap-card",
                "title": "Title X",
                "reading_id": "reading_01",
                "summary": "Summary X",
                "card_hash": card_hash,
                "citation_count": 0,
                "source_ids": [],
            }],
            "retracted_cards": ["overlap-card"],
            "dependency_retracted_cards": [],
        });
        fs::write(
            dir.path().join("cards_manifest.json"),
            serde_json::to_string(&invalid).unwrap(),
        )
        .unwrap();
        let journal_path = dir.path().join("journal.jsonl");
        let outcome = lint_directory(&cards_dir, &manifest_path, &journal_path, None)
            .expect("lint_directory must not error: cards_dir exists");
        let man_001: Vec<_> = outcome
            .diagnostics
            .iter()
            .filter(|d| d.code == codes::MAN_001)
            .collect();
        assert_eq!(
            man_001.len(),
            1,
            "an invariant-invalid manifest must emit exactly one preflight CACG-MAN-001; got {:?}",
            outcome.diagnostics
        );
        assert!(
            outcome.failed,
            "an invariant-invalid cards_manifest.json must set failed"
        );
    }

    #[test]
    fn auth_000_fires_when_matrix_has_load_error() {
        let fx = build_fixture("alpha beta gamma", "src:p001:0000", "src");
        let auth = AuthSpec {
            matrix: None,
            load_error: Some("CACG-AUTH-000 fixture failure".to_string()),
        };
        let result = run(&fx, Some(&auth));
        assert!(result.diagnostics.iter().any(|d| d.code == codes::AUTH_000));
        // AUTH-000 does NOT suppress citation checks; clean fixture
        // still emits no citation-loop diagnostics.
        let non_auth_000: Vec<_> = result
            .diagnostics
            .iter()
            .filter(|d| d.code != codes::AUTH_000)
            .collect();
        assert!(non_auth_000.is_empty());
    }

    #[test]
    fn malformed_chunk_id_emits_cite_001_and_short_circuits() {
        let fx = build_fixture("alpha beta gamma", "INVALID:CHUNKID:!!", "src");
        let result = run(&fx, None);
        // The citation also has a real chunk_hash (auto-computed),
        // so without short-circuiting we'd see two diagnostics. The
        // contract is one per citation.
        let codes_seen: Vec<&str> = result.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes_seen, vec![codes::CITE_001]);
    }

    #[test]
    fn placeholder_hash_emits_cite_002() {
        let dir = tempfile::tempdir().unwrap();
        // We need a manifest that has the chunk_id but the citation's
        // hash is the all-zero placeholder; the citation must come
        // BEFORE the chunk lookup so we hit the placeholder branch.
        let chunk = sample_chunk("src:p001:0000", "src", "alpha beta gamma");
        let manifest = manifest_with(vec![chunk]);
        let manifest_path = write_manifest(dir.path(), &manifest);
        let card_path = write_card(
            dir.path(),
            "card.md",
            &card_md(PLACEHOLDER_HASH, "src:p001:0000", "src"),
        );
        let result = run_layer1_checks(&card_path, &manifest_path, None, None);
        let codes_seen: Vec<&str> = result.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes_seen, vec![codes::CITE_002]);
        assert!(result.diagnostics[0]
            .message
            .contains("placeholder chunk_hash"));
    }

    #[test]
    fn retracted_source_emits_retr_002_before_cite_004() {
        let dir = tempfile::tempdir().unwrap();
        let chunk = sample_chunk("other:p001:0000", "other_source", "x");
        let manifest = ChunksManifest {
            schema_version: SchemaVersion::V0,
            chunks: vec![chunk],
            retracted_source_ids: vec!["src".to_string()],
            retracted_chunk_ids: Vec::new(),
        };
        let manifest_path = write_manifest(dir.path(), &manifest);
        // Cite an unretracted chunk_id but retracted source — but cite
        // a valid format hash. Use a known-good 64-hex value.
        let card_path = write_card(
            dir.path(),
            "card.md",
            &card_md(
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "src:p001:0000",
                "src",
            ),
        );
        let result = run_layer1_checks(&card_path, &manifest_path, None, None);
        let codes_seen: Vec<&str> = result.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes_seen, vec![codes::RETR_002]);
    }

    #[test]
    fn retracted_chunk_emits_retr_003_before_cite_004() {
        let dir = tempfile::tempdir().unwrap();
        let chunk = sample_chunk("other:p001:0000", "other_source", "x");
        let manifest = ChunksManifest {
            schema_version: SchemaVersion::V0,
            chunks: vec![chunk],
            retracted_source_ids: Vec::new(),
            retracted_chunk_ids: vec!["src:p001:0000".to_string()],
        };
        let manifest_path = write_manifest(dir.path(), &manifest);
        let card_path = write_card(
            dir.path(),
            "card.md",
            &card_md(
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "src:p001:0000",
                "src",
            ),
        );
        let result = run_layer1_checks(&card_path, &manifest_path, None, None);
        let codes_seen: Vec<&str> = result.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes_seen, vec![codes::RETR_003]);
    }

    #[test]
    fn chunk_not_in_manifest_emits_cite_004() {
        let dir = tempfile::tempdir().unwrap();
        let chunk = sample_chunk("other:p001:0000", "other_source", "x");
        let manifest = manifest_with(vec![chunk]);
        let manifest_path = write_manifest(dir.path(), &manifest);
        let card_path = write_card(
            dir.path(),
            "card.md",
            &card_md(
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "src:p001:0000",
                "src",
            ),
        );
        let result = run_layer1_checks(&card_path, &manifest_path, None, None);
        let codes_seen: Vec<&str> = result.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes_seen, vec![codes::CITE_004]);
    }

    #[test]
    fn source_mismatch_emits_cite_006() {
        let dir = tempfile::tempdir().unwrap();
        // Manifest chunk with source_id "src" but citation claims
        // source_id "other".
        let chunk = sample_chunk("src:p001:0000", "src", "alpha");
        let chunk_hash = chunk.chunk_hash.clone();
        let manifest = manifest_with(vec![chunk]);
        let manifest_path = write_manifest(dir.path(), &manifest);
        let card_path = write_card(
            dir.path(),
            "card.md",
            &card_md(&chunk_hash, "src:p001:0000", "other"),
        );
        let result = run_layer1_checks(&card_path, &manifest_path, None, None);
        let codes_seen: Vec<&str> = result.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes_seen, vec![codes::CITE_006]);
    }

    #[test]
    fn hash_drift_emits_hash_001() {
        let dir = tempfile::tempdir().unwrap();
        let chunk = sample_chunk("src:p001:0000", "src", "alpha");
        let manifest = manifest_with(vec![chunk]);
        let manifest_path = write_manifest(dir.path(), &manifest);
        // Citation has the correct chunk_id but wrong hash.
        let drift_hash = "1111111111111111111111111111111111111111111111111111111111111111";
        let card_path = write_card(
            dir.path(),
            "card.md",
            &card_md(drift_hash, "src:p001:0000", "src"),
        );
        let result = run_layer1_checks(&card_path, &manifest_path, None, None);
        let codes_seen: Vec<&str> = result.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes_seen, vec![codes::HASH_001]);
    }

    #[test]
    fn auth_001_when_reading_unknown() {
        let fx = build_fixture("alpha", "src:p001:0000", "src");
        let matrix: crate::schema::SourceMatrix = serde_json::from_value(serde_json::json!({
            "schema_version": "cacg.v0",
            "allowed": {"reading_02": ["src"]}
        }))
        .unwrap();
        let auth = AuthSpec {
            matrix: Some(matrix),
            load_error: None,
        };
        let result = run(&fx, Some(&auth));
        let codes_seen: Vec<&str> = result.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes_seen, vec![codes::AUTH_001]);
    }

    #[test]
    fn auth_002_when_source_unauthorized() {
        let fx = build_fixture("alpha", "src:p001:0000", "src");
        let matrix: crate::schema::SourceMatrix = serde_json::from_value(serde_json::json!({
            "schema_version": "cacg.v0",
            "allowed": {"reading_01": ["other_source"]}
        }))
        .unwrap();
        let auth = AuthSpec {
            matrix: Some(matrix),
            load_error: None,
        };
        let result = run(&fx, Some(&auth));
        let codes_seen: Vec<&str> = result.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert_eq!(codes_seen, vec![codes::AUTH_002]);
    }

    #[test]
    fn card_hash_002_fires_when_stored_differs_from_recomputed() {
        let dir = tempfile::tempdir().unwrap();
        let chunk = sample_chunk("src:p001:0000", "src", "alpha");
        let chunk_hash = chunk.chunk_hash.clone();
        let manifest = manifest_with(vec![chunk]);
        let manifest_path = write_manifest(dir.path(), &manifest);
        let card_body = format!(
            r#"---
schema_version: cacg.v0
id: test-card
title: Test Card
reading_id: reading_01
summary: This is a summary that is at least eighty characters long so the schema validator passes the SUM-001 minimum-length check, padded a bit.
card_hash: "1111111111111111111111111111111111111111111111111111111111111111"
citations:
  - source_id: "src"
    chunk_id: "src:p001:0000"
    chunk_hash: "{chunk_hash}"
    page_range: [1, 1]
    quote: "alpha"
---
# Test Card

This is the body.
"#
        );
        let card_path = write_card(dir.path(), "card.md", &card_body);
        let result = run_layer1_checks(&card_path, &manifest_path, None, None);
        let hash_diags: Vec<&Diagnostic> = result
            .diagnostics
            .iter()
            .filter(|d| d.code == codes::HASH_002)
            .collect();
        assert_eq!(hash_diags.len(), 1, "expected exactly one HASH-002");
        assert!(result.card_hash_before.is_some());
    }

    #[test]
    fn absent_card_hash_passes_silently() {
        let fx = build_fixture("alpha", "src:p001:0000", "src");
        let result = run(&fx, None);
        assert!(result.diagnostics.iter().all(|d| d.code != codes::HASH_002));
    }

    #[test]
    fn batch_shared_chunks_index_skips_manifest_load() {
        let fx = build_fixture("alpha", "src:p001:0000", "src");
        let raw = std::fs::read_to_string(&fx.manifest_path).unwrap();
        let manifest: ChunksManifest = serde_json::from_str(&raw).unwrap();
        let index = ChunksIndex::from_manifest(manifest).unwrap();
        // Pass a deliberately-invalid chunks_manifest_path; the
        // batch-shared index must be used and the invalid path
        // ignored.
        let bogus_manifest = PathBuf::from("/nonexistent/__bogus__.json");
        let result = run_layer1_checks(&fx.card_path, &bogus_manifest, Some(&index), None);
        assert!(
            result.diagnostics.is_empty(),
            "batch-shared index path must skip manifest load, got {:?}",
            result.diagnostics
        );
    }

    // The tests below mutate the process-global `KB_FROZEN_CLOCK` env
    // var that `freeze_aware_latency` and the journal append observe.
    // `with_frozen_clock` (imported above) locks the crate-shared
    // `FROZEN_CLOCK_TEST_MUTEX`, so they serialize against every
    // env-mutating test in the crate — including `verify::runner::tests`,
    // which a per-file mutex would not cover.

    #[test]
    fn freeze_aware_latency_returns_zero_when_frozen() {
        let _g = with_frozen_clock(Some("1"));
        let start = Instant::now();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let latency = freeze_aware_latency(start);
        assert_eq!(latency, 0.0);
    }

    #[test]
    fn freeze_aware_latency_measures_elapsed_when_unfrozen() {
        let _g = with_frozen_clock(None);
        let start = Instant::now();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let latency = freeze_aware_latency(start);
        assert!(latency > 0.0, "expected nonzero latency under live clock");
    }

    #[test]
    fn lint_card_appends_exactly_one_journal_event_clean_card() {
        let _g = with_frozen_clock(Some("1"));
        let fx = build_fixture("alpha beta gamma", "src:p001:0000", "src");
        let journal = fx._dir.path().join("lint_journal.jsonl");
        let outcome =
            lint_card(&fx.card_path, &fx.manifest_path, &journal, None, None).expect("lint_card");
        assert!(!outcome.failed);
        assert!(outcome.diagnostics.is_empty());
        let body = std::fs::read_to_string(&journal).unwrap();
        let lines: Vec<&str> = body.lines().collect();
        assert_eq!(lines.len(), 1, "expected exactly one lint journal entry");
        let parsed: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(parsed["command"], "lint");
        assert_eq!(parsed["latency_ms"], 0.0);
        assert_eq!(parsed["verification"]["layer1"], true);
        assert_eq!(parsed["verification"]["layer2"], false);
        assert_eq!(parsed["verification"]["fuzzy"], false);
        // card_hash_before == card_hash_after (lint doesn't mutate).
        assert_eq!(parsed["card_hash_before"], parsed["card_hash_after"]);
    }

    #[test]
    fn lint_card_appends_exactly_one_journal_event_failing_card() {
        let _g = with_frozen_clock(Some("1"));
        let dir = tempfile::tempdir().unwrap();
        let card_path = write_card(
            dir.path(),
            "card.md",
            // Non-existent chunk → CITE-004.
            &card_md(
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                "src:p001:0000",
                "src",
            ),
        );
        let manifest = manifest_with(vec![]);
        let manifest_path = write_manifest(dir.path(), &manifest);
        let journal = dir.path().join("lint_journal.jsonl");
        let outcome =
            lint_card(&card_path, &manifest_path, &journal, None, None).expect("lint_card");
        assert!(outcome.failed);
        let body = std::fs::read_to_string(&journal).unwrap();
        assert_eq!(body.lines().count(), 1);
        let parsed: serde_json::Value = serde_json::from_str(body.lines().next().unwrap()).unwrap();
        assert_eq!(parsed["verification"]["layer1"], false);
        assert_eq!(parsed["latency_ms"], 0.0);
        // Diagnostic shows the CITE-004 we expect.
        let diags = parsed["diagnostics"].as_array().unwrap();
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0]["code"], "CACG-CITE-004");
    }

    #[test]
    fn auxiliary_codes_never_emitted_from_lint_pass_surface() {
        // Run lint over every committed valid card; the run_layer1_checks
        // surface MUST NOT emit any CACG-SUM-* / CACG-SKILL-* / CACG-DEP-*
        // / CACG-ROLE-* code from its own paths.
        //
        // (Loader-carried CACG-SUM-* diagnostics from the M1 frontmatter
        // surface would only fire on cards that fail summary length /
        // tag-slug validation; the committed valid cards pass those
        // checks, so no SUM-* should appear. This negative covers both
        // "surface emits SUM-*" and "loader leaks SUM-* on valid cards".)
        let dir = workspace_root().join("tests/parity_corpus/valid");
        if !dir.is_dir() {
            // Parity corpus might not be materialized in some CI shapes.
            return;
        }
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            // Use the corpus's own chunks_manifest if present.
            let manifest_path =
                workspace_root().join("tests/parity_corpus/out_python/chunks_manifest.json");
            if !manifest_path.is_file() {
                continue;
            }
            let result = run_layer1_checks(&path, &manifest_path, None, None);
            for d in &result.diagnostics {
                assert!(
                    !d.code.starts_with("CACG-SUM-"),
                    "{}: SUM-* not allowed from lint-pass surface",
                    path.display()
                );
                assert!(
                    !d.code.starts_with("CACG-SKILL-"),
                    "{}: SKILL-* not allowed from lint-pass surface",
                    path.display()
                );
                assert!(
                    !d.code.starts_with("CACG-DEP-"),
                    "{}: DEP-* not allowed from lint-pass surface",
                    path.display()
                );
                assert!(
                    !d.code.starts_with("CACG-ROLE-"),
                    "{}: ROLE-* not allowed from lint-pass surface",
                    path.display()
                );
            }
        }
    }
}
