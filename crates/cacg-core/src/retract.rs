//! Single-chunk retraction primitive.
//!
//! Byte-equal port of `legacy_python_oracle/src/cacg/retract.py::retract_chunk` (the
//! 2nd of 9 M3 authoring-tail verbs). Adds a `chunk_id` to
//! `chunks_manifest.retracted_chunk_ids`, removes the matching
//! record from `chunks_manifest.chunks` (so the schema
//! disjointness invariant holds), atomically rewrites the
//! manifest via the project's `.tmp` → `.bak` → `replace`
//! discipline, and — when a `cards_dir` is supplied AND a
//! sibling `cards_manifest.json` exists — cascades the
//! retraction into `cards_manifest.dependency_retracted_cards`
//! for every card citing the retracted chunk.
//!
//! The atomic-update discipline lives in [`atomic_chunks_manifest_update`]
//! and matches Python `cacg.retract._atomic_chunks_manifest_update`:
//! refuse to clobber pre-existing `.tmp` / `.bak` sidecars
//! (`CACG-MAN-002`); round-trip validate the `.tmp` bytes
//! through schema validation before commit; on commit failure
//! restore from `.bak` (or unlink `.tmp` if no prior).
//!
//! The cascade is pair-atomic via [`update_cards_manifest_cascade`]
//! and matches Python `cacg.index._publish_pair`: both
//! `cards_manifest.json` and `INDEX.md` are written through
//! [`atomic_publish`] with `CACG-IDX-007` (sidecar collision) and
//! `CACG-IDX-008` (non-file canonical) as the diagnostic codes.
//! On any cascade-side failure the retract primitive raises
//! `RetractError::CascadePublish` whose message starts with
//! `CACG-RET-003:` per Python's wire surface.

use std::fs;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::atomic_publish::{atomic_publish, DefaultFs, PublishError, PublishMember};
use crate::canonical_json::canonical_json;
use crate::card_loader::load_card;
use crate::diagnostic::codes as cc;
use crate::index::{build_index_md, CardsManifest};
use crate::schema::{ChunksManifest, SchemaVersion};

/// Errors returned by [`retract_chunk`]. The CLI `dispatch_retract_chunk`
/// maps these 1:1 to the byte-equal stderr surfaces Python
/// `_cmd_retract_chunk` emits.
#[derive(Debug, Error)]
pub enum RetractError {
    /// `chunks_manifest.json` is not a regular file in `out_dir`.
    /// Dispatcher emits a `CACG-CLI-001:` line citing the path
    /// and pointing the operator at `kb ingest`.
    #[error("chunks_manifest_missing: {0}")]
    ChunksManifestMissing(PathBuf),
    /// Caller asked to retract a chunk that is already in
    /// `retracted_chunk_ids` AND no card-cascade was requested
    /// (so this would be a no-op). Retraction is append-only
    /// and not idempotent.
    #[error("already_retracted: {0}")]
    AlreadyRetracted(String),
    /// Caller asked to retract a chunk that is already in
    /// `retracted_chunk_ids` AND the cards-side cascade is already
    /// up-to-date (so the whole operation would be a no-op).
    /// Mirrors Python `retract.py:608-611`.
    #[error("already_retracted_no_op: {0}")]
    AlreadyRetractedNoOp(String),
    /// Caller asked to retract a chunk that is not present in
    /// `chunks_manifest.chunks` (active list).
    #[error("unknown_chunk: {0}")]
    UnknownChunk(String),
    /// `chunks_manifest.json` failed schema validation on load OR
    /// `cards_manifest.json` failed schema validation on load.
    /// Python maps both of these to the same `CACG-MAN-001:
    /// chunks_manifest.json is invalid:` prefix (even when the
    /// underlying validation was on the cards manifest), so we
    /// collapse them into one variant.
    #[error("{0}")]
    ManifestInvalid(String),
    /// `.tmp` or `.bak` sidecar already exists at the chunks-manifest
    /// publish boundary; refusing to clobber (matches Python
    /// `FileExistsError` / `CACG-MAN-002` surface).
    #[error("preexisting_sidecars: {0:?}")]
    PreexistingSidecars(Vec<PathBuf>),
    /// Cards-manifest cascade publish failed after the chunks-manifest
    /// rewrite succeeded. The carrying string is the full Python wire
    /// message including the `CACG-RET-003:` prefix and the
    /// `Underlying: ...` suffix; the dispatcher prepends
    /// `CACG-CLI-001: ` to match Python's `_cmd_retract_chunk`
    /// `except RetractError` arm.
    #[error("{0}")]
    CascadePublish(String),
    /// I/O failure outside the publish path. Dispatcher maps to
    /// `CACG-IDX-004` (Python's bare-`Exception` arm).
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

/// Report emitted by a successful [`retract_chunk`] run.
#[derive(Debug, Clone)]
pub struct RetractChunkReport {
    /// The `chunk_id` that was retracted.
    pub chunk_id: String,
    /// Number of chunks remaining in the active `chunks` list
    /// after the retraction.
    pub chunks_remaining: usize,
    /// Total size of the `retracted_chunk_ids` list after the
    /// retraction.
    pub retracted_chunk_ids_total: usize,
}

/// Filename Python's `cacg.skill_router` treats as a router file.
/// Cards-manifest cascade walkers MUST skip these (Python
/// `retract.py:66, 90`).
const SKILL_ROUTER_FILENAME: &str = "SKILL.md";

/// Retract a single chunk. See module-level docs for the full
/// behavioral contract.
///
/// # Errors
///
/// Any [`RetractError`] variant. The caller (CLI dispatcher)
/// maps these 1:1 to Python `_cmd_retract_chunk`'s stderr
/// surface for byte-equal parity.
pub fn retract_chunk(
    chunk_id: &str,
    out_dir: &Path,
    cards_dir: Option<&Path>,
) -> Result<RetractChunkReport, RetractError> {
    let chunks_manifest_path = out_dir.join("chunks_manifest.json");
    let cards_manifest_path = out_dir.join("cards_manifest.json");
    if !chunks_manifest_path.is_file() {
        return Err(RetractError::ChunksManifestMissing(out_dir.to_path_buf()));
    }
    let prior_bytes = fs::read(&chunks_manifest_path)?;
    let prior: ChunksManifest = serde_json::from_slice(&prior_bytes)
        .map_err(|e| RetractError::ManifestInvalid(e.to_string()))?;
    prior
        .validate_structurally()
        .map_err(|d| RetractError::ManifestInvalid(d.message))?;

    let already_retracted = prior.retracted_chunk_ids.iter().any(|c| c == chunk_id);

    let mut chunks_was_updated = false;
    let new_manifest: ChunksManifest = if already_retracted {
        if cards_dir.is_none() || !cards_manifest_path.is_file() {
            return Err(RetractError::AlreadyRetracted(chunk_id.to_owned()));
        }
        prior.clone()
    } else {
        let active = prior.chunks.iter().any(|c| c.chunk_id == chunk_id);
        if !active {
            return Err(RetractError::UnknownChunk(chunk_id.to_owned()));
        }
        let new_chunks: Vec<_> = prior
            .chunks
            .iter()
            .filter(|c| c.chunk_id != chunk_id)
            .cloned()
            .collect();
        let mut new_retracted: Vec<String> = prior.retracted_chunk_ids.clone();
        new_retracted.push(chunk_id.to_owned());
        new_retracted.sort();
        new_retracted.dedup();
        let nm = ChunksManifest {
            schema_version: SchemaVersion::V0,
            chunks: new_chunks,
            retracted_source_ids: prior.retracted_source_ids.clone(),
            retracted_chunk_ids: new_retracted,
        };
        atomic_chunks_manifest_update(&chunks_manifest_path, &nm)?;
        chunks_was_updated = true;
        nm
    };

    if let Some(cards_dir) = cards_dir {
        if cards_manifest_path.is_file() {
            let cascade_changed = update_cards_manifest_cascade(
                cards_dir,
                out_dir,
                &cards_manifest_path,
                &new_manifest.retracted_source_ids,
                &new_manifest.retracted_chunk_ids,
                chunk_id,
            )?;
            if !chunks_was_updated && !cascade_changed {
                return Err(RetractError::AlreadyRetractedNoOp(chunk_id.to_owned()));
            }
        }
    }

    Ok(RetractChunkReport {
        chunk_id: chunk_id.to_owned(),
        chunks_remaining: new_manifest.chunks.len(),
        retracted_chunk_ids_total: new_manifest.retracted_chunk_ids.len(),
    })
}

/// Atomic single-file rewrite of `chunks_manifest.json` via
/// the project-wide `atomic_publish` discipline. Routing through
/// the centralized publisher (rather than hand-rolling
/// tmp/bak/rename) is enforced by the
/// `xtask lint-rename-outside-publisher` static gate;
/// `cacg.retract._atomic_chunks_manifest_update` is the Python
/// oracle. The publisher's rollback semantics
/// (`PreexistingSidecars` / `NonFileCanonical` / I/O failure
/// rolls back to the prior canonical) match Python's
/// `FileExistsError` / restore-from-bak / re-raise shape.
///
/// Round-trip validation: we ALSO parse the canonical payload
/// through the Rust schema before handing it to the publisher
/// so a programmer error that builds an invariant-invalid
/// manifest surfaces here, not at the next `kb lint` /
/// `kb verify` run. Mirrors Python's `model_validate_json` on
/// the .tmp bytes (`cacg.retract:330-337`).
fn atomic_chunks_manifest_update(
    chunks_manifest_path: &Path,
    new_manifest: &ChunksManifest,
) -> Result<(), RetractError> {
    let value = serde_json::to_value(new_manifest)
        .map_err(|e| RetractError::ManifestInvalid(e.to_string()))?;
    let payload =
        canonical_json(&value).map_err(|e| RetractError::ManifestInvalid(format!("{e:?}")))?;
    let parsed: ChunksManifest = serde_json::from_slice(payload.as_bytes())
        .map_err(|e| RetractError::ManifestInvalid(e.to_string()))?;
    parsed
        .validate_structurally()
        .map_err(|d| RetractError::ManifestInvalid(d.message))?;

    let tmp = with_extra_suffix(chunks_manifest_path, ".tmp");
    let bak = with_extra_suffix(chunks_manifest_path, ".bak");
    let member = PublishMember {
        canonical_path: chunks_manifest_path.to_path_buf(),
        tmp_path: tmp,
        bak_path: bak,
        bytes: payload.into_bytes(),
    };
    let fs_syscalls = DefaultFs;
    match atomic_publish(&[member], &fs_syscalls, cc::MAN_002, cc::MAN_003) {
        Ok(()) => Ok(()),
        Err(PublishError::PreexistingSidecars { paths, .. }) => {
            Err(RetractError::PreexistingSidecars(paths))
        }
        Err(e) => Err(RetractError::Io(std::io::Error::other(e.to_string()))),
    }
}

/// Append a suffix to a path's filename. `with_extra_suffix(
/// "foo/bar.json", ".tmp")` → `foo/bar.json.tmp`. Pure helper.
fn with_extra_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("file")
        .to_owned();
    name.push_str(suffix);
    let mut out = path.to_path_buf();
    out.set_file_name(name);
    out
}

/// Pair-atomic cards-manifest cascade. Walks every `.md` under
/// `cards_dir` (skipping `SKILL.md` routers), finds cards whose
/// citations cite any retracted `source_id` or `chunk_id`, and
/// pair-atomically publishes the rewritten `cards_manifest.json`
/// PLUS the re-rendered `INDEX.md` via [`atomic_publish`].
/// Mirrors Python `cacg.index._publish_pair`
/// (`legacy_python_oracle/src/cacg/index.py:1191-1283`).
///
/// Returns `Ok(true)` iff `dependency_retracted_cards` changed
/// (so the caller can detect the all-no-op case Python raises
/// `RetractError("...already up-to-date")` for). Returns
/// `Ok(false)` when the recomputed cascade is byte-identical to
/// the prior — the no-op path skips the publish entirely,
/// matching Python `retract.py:580`.
///
/// On any cascade-side publish failure the function returns
/// `RetractError::CascadePublish` whose Display string starts
/// with `CACG-RET-003:` per Python's wire surface
/// (`retract.py:601-606`).
fn update_cards_manifest_cascade(
    cards_dir: &Path,
    out_dir: &Path,
    cards_manifest_path: &Path,
    retracted_source_ids: &[String],
    retracted_chunk_ids: &[String],
    cli_chunk_id: &str,
) -> Result<bool, RetractError> {
    let cascade = compute_cascade(cards_dir, retracted_source_ids, retracted_chunk_ids);
    let prior_bytes = fs::read(cards_manifest_path)?;
    let mut prior_cards: CardsManifest = serde_json::from_slice(&prior_bytes)
        .map_err(|e| RetractError::ManifestInvalid(e.to_string()))?;
    prior_cards
        .validate_structurally()
        .map_err(|d| RetractError::ManifestInvalid(d.message))?;

    let retracted_set: std::collections::HashSet<&str> = prior_cards
        .retracted_cards
        .iter()
        .map(String::as_str)
        .collect();
    let active_ids: std::collections::HashSet<&str> =
        prior_cards.cards.iter().map(|e| e.id.as_str()).collect();
    let mut new_dep_retracted: Vec<String> = cascade
        .iter()
        .filter(|cid| !retracted_set.contains(cid.as_str()) && active_ids.contains(cid.as_str()))
        .cloned()
        .collect();
    new_dep_retracted.sort();
    new_dep_retracted.dedup();

    let mut prior_dep_sorted = prior_cards.dependency_retracted_cards.clone();
    prior_dep_sorted.sort();
    if new_dep_retracted == prior_dep_sorted {
        return Ok(false);
    }
    prior_cards.dependency_retracted_cards = new_dep_retracted;
    prior_cards.retracted_cards.sort();

    let manifest_value = serde_json::to_value(&prior_cards)
        .map_err(|e| RetractError::ManifestInvalid(e.to_string()))?;
    let manifest_payload = canonical_json(&manifest_value)
        .map_err(|e| RetractError::ManifestInvalid(format!("{e:?}")))?;
    let parsed: CardsManifest = serde_json::from_slice(manifest_payload.as_bytes())
        .map_err(|e| RetractError::ManifestInvalid(e.to_string()))?;
    parsed
        .validate_structurally()
        .map_err(|d| RetractError::ManifestInvalid(d.message))?;

    let index_md = build_index_md(&prior_cards.cards);
    let index_path = out_dir.join("INDEX.md");
    let cards_tmp = with_extra_suffix(cards_manifest_path, ".tmp");
    let cards_bak = with_extra_suffix(cards_manifest_path, ".bak");
    let index_tmp = with_extra_suffix(&index_path, ".tmp");
    let index_bak = with_extra_suffix(&index_path, ".bak");

    let members = [
        PublishMember {
            canonical_path: cards_manifest_path.to_path_buf(),
            tmp_path: cards_tmp,
            bak_path: cards_bak,
            bytes: manifest_payload.into_bytes(),
        },
        PublishMember {
            canonical_path: index_path,
            tmp_path: index_tmp,
            bak_path: index_bak,
            bytes: index_md.into_bytes(),
        },
    ];
    let fs_syscalls = DefaultFs;
    match atomic_publish(&members, &fs_syscalls, cc::IDX_007, cc::IDX_008) {
        Ok(()) => Ok(true),
        Err(e) => Err(RetractError::CascadePublish(format!(
            "CACG-RET-003: chunks_manifest update succeeded but \
             cards_manifest dependency-cascade update failed; \
             re-run `kb retract-chunk {cli_chunk_id}` with \
             --cards-dir to retry. Underlying: {}",
            python_publish_error_repr(&e)
        ))),
    }
}

/// Format a [`PublishError`] the way Python would format the
/// underlying exception in the `CACG-RET-003: ... Underlying:`
/// suffix. `_publish_pair` raises `FileExistsError` (sidecar
/// collision → `CACG-IDX-007`, non-file canonical →
/// `CACG-IDX-008`) or a raw `OSError`. The bare-`Exception`
/// pattern in Python is `str(exc)`, which for `FileExistsError`
/// is the message string as constructed at `index.py:1223-1227`
/// or `index.py:1239-1244`.
fn python_publish_error_repr(e: &PublishError) -> String {
    match e {
        PublishError::PreexistingSidecars { diagnostic, paths } => {
            format!(
                "{diagnostic}: refusing to clobber existing index sidecar(s): {}; \
                 remove them and re-run kb index",
                py_list_of_str_repr(paths)
            )
        }
        PublishError::NonFileCanonical { diagnostic, paths } => {
            format!(
                "{diagnostic}: refusing to overwrite non-file canonical target(s): {}; \
                 the existing path must be a regular file (or absent) for \
                 kb index to publish",
                py_list_of_str_repr(paths)
            )
        }
        PublishError::Io { source } => source.to_string(),
    }
}

/// Format `paths` as Python's `[str(p) for p in paths]` → `str()`:
/// `['<p1>', '<p2>']` with single quotes around each path and
/// `, ` separating entries.
#[must_use]
pub fn py_list_of_str_repr(paths: &[PathBuf]) -> String {
    let mut out = String::from("[");
    let mut first = true;
    for p in paths {
        if !first {
            out.push_str(", ");
        }
        first = false;
        out.push('\'');
        out.push_str(&p.display().to_string());
        out.push('\'');
    }
    out.push(']');
    out
}

/// Walk every `.md` under `cards_dir`; return `card_ids` whose
/// citations cite any chunk in `retracted_chunk_ids` OR whose
/// `source_id` is in `retracted_source_ids`. Mirrors the
/// `_compute_cascade_for_sources ∪ _compute_cascade_for_chunks`
/// union in Python (`cacg.retract:48-100`). Skips `SKILL.md`
/// router files (Python `retract.py:66, 90`).
fn compute_cascade(
    cards_dir: &Path,
    retracted_source_ids: &[String],
    retracted_chunk_ids: &[String],
) -> std::collections::HashSet<String> {
    let mut out: std::collections::HashSet<String> = std::collections::HashSet::new();
    if retracted_source_ids.is_empty() && retracted_chunk_ids.is_empty() {
        return out;
    }
    let src_set: std::collections::HashSet<&str> =
        retracted_source_ids.iter().map(String::as_str).collect();
    let chunk_set: std::collections::HashSet<&str> =
        retracted_chunk_ids.iter().map(String::as_str).collect();

    let walker = walk_md(cards_dir);
    for card_path in walker {
        if card_path.file_name() == Some(std::ffi::OsStr::new(SKILL_ROUTER_FILENAME)) {
            continue;
        }
        let Ok(doc) = load_card(&card_path) else {
            continue;
        };
        for cit in &doc.frontmatter.citations {
            if src_set.contains(cit.source_id.as_str()) || chunk_set.contains(cit.chunk_id.as_str())
            {
                out.insert(doc.frontmatter.id.clone());
                break;
            }
        }
    }
    out
}

/// Recursive walk of `dir`, yielding every `.md` file path in
/// sorted (`BTreeSet`) order — matches Python's
/// `sorted(cards_dir.rglob("*.md"))`. Skip-on-error matches
/// Python's `try/except: continue`.
fn walk_md(dir: &Path) -> Vec<PathBuf> {
    let mut out: std::collections::BTreeSet<PathBuf> = std::collections::BTreeSet::new();
    let mut stack: Vec<PathBuf> = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = fs::read_dir(&d) else {
            continue;
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().and_then(|s| s.to_str()) == Some("md") {
                out.insert(p);
            }
        }
    }
    out.into_iter().collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use serde_json::json;

    fn write_chunks_manifest(out_dir: &Path, chunks: &[&str]) {
        let records: Vec<_> = chunks
            .iter()
            .enumerate()
            .map(|(i, cid)| {
                let parts: Vec<&str> = cid.split(':').collect();
                let source_id = parts[0];
                json!({
                    "schema_version": "cacg.v0",
                    "source_id": source_id,
                    "chunk_id": cid,
                    "ordinal": i,
                    "start_page": 1,
                    "end_page": 1,
                    "page_spans": [{"page": 1, "byte_offset_in_chunk": 0}],
                    "text": format!("text for {cid}"),
                    "text_preview": format!("text for {cid}"),
                    "token_count": 3,
                    "chunk_hash": "0".repeat(64),
                })
            })
            .collect();
        let v = json!({
            "schema_version": "cacg.v0",
            "chunks": records,
            "retracted_source_ids": [],
            "retracted_chunk_ids": [],
        });
        let body = canonical_json(&v).expect("canonical");
        fs::write(out_dir.join("chunks_manifest.json"), body.as_bytes()).expect("write chunks");
    }

    #[test]
    fn retract_chunk_removes_from_active_and_adds_to_retracted_list() {
        let tmp = tempfile::tempdir().expect("tempdir");
        write_chunks_manifest(tmp.path(), &["src:p001:0000", "src:p001:0001"]);
        let report = retract_chunk("src:p001:0000", tmp.path(), None).unwrap();
        assert_eq!(report.chunk_id, "src:p001:0000");
        assert_eq!(report.chunks_remaining, 1);
        assert_eq!(report.retracted_chunk_ids_total, 1);
        let after: ChunksManifest =
            serde_json::from_slice(&fs::read(tmp.path().join("chunks_manifest.json")).unwrap())
                .unwrap();
        assert_eq!(after.chunks.len(), 1);
        assert_eq!(after.chunks[0].chunk_id, "src:p001:0001");
        assert_eq!(after.retracted_chunk_ids, vec!["src:p001:0000"]);
    }

    #[test]
    fn retract_chunk_rejects_already_retracted_when_no_cascade_requested() {
        let tmp = tempfile::tempdir().expect("tempdir");
        write_chunks_manifest(tmp.path(), &["src:p001:0000"]);
        retract_chunk("src:p001:0000", tmp.path(), None).unwrap();
        let err = retract_chunk("src:p001:0000", tmp.path(), None).unwrap_err();
        assert!(matches!(err, RetractError::AlreadyRetracted(_)));
    }

    #[test]
    fn retract_chunk_rejects_unknown_chunk_id() {
        let tmp = tempfile::tempdir().expect("tempdir");
        write_chunks_manifest(tmp.path(), &["src:p001:0000"]);
        let err = retract_chunk("nope:p001:0000", tmp.path(), None).unwrap_err();
        assert!(matches!(err, RetractError::UnknownChunk(_)));
    }

    #[test]
    fn retract_chunk_rejects_missing_chunks_manifest() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let err = retract_chunk("any", tmp.path(), None).unwrap_err();
        assert!(matches!(err, RetractError::ChunksManifestMissing(_)));
    }

    #[test]
    fn retract_chunk_refuses_to_clobber_preexisting_sidecars() {
        let tmp = tempfile::tempdir().expect("tempdir");
        write_chunks_manifest(tmp.path(), &["src:p001:0000"]);
        fs::write(tmp.path().join("chunks_manifest.json.tmp"), b"stale").unwrap();
        let err = retract_chunk("src:p001:0000", tmp.path(), None).unwrap_err();
        assert!(matches!(err, RetractError::PreexistingSidecars(_)));
    }

    #[test]
    fn retract_chunk_keeps_retracted_list_sorted_and_unique() {
        let tmp = tempfile::tempdir().expect("tempdir");
        write_chunks_manifest(
            tmp.path(),
            &["src:p001:0002", "src:p001:0000", "src:p001:0001"],
        );
        retract_chunk("src:p001:0002", tmp.path(), None).unwrap();
        retract_chunk("src:p001:0000", tmp.path(), None).unwrap();
        let after: ChunksManifest =
            serde_json::from_slice(&fs::read(tmp.path().join("chunks_manifest.json")).unwrap())
                .unwrap();
        assert_eq!(
            after.retracted_chunk_ids,
            vec!["src:p001:0000".to_owned(), "src:p001:0002".to_owned()]
        );
    }

    #[test]
    fn py_list_of_str_repr_matches_python() {
        let paths = vec![
            PathBuf::from("/tmp/a/chunks_manifest.json.tmp"),
            PathBuf::from("/tmp/a/chunks_manifest.json.bak"),
        ];
        assert_eq!(
            py_list_of_str_repr(&paths),
            "['/tmp/a/chunks_manifest.json.tmp', '/tmp/a/chunks_manifest.json.bak']"
        );
        assert_eq!(py_list_of_str_repr(&[]), "[]");
    }
}
