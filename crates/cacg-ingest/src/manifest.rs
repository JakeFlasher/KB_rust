//! `SourcesManifest` + `ChunksManifest` builder and pair-atomic publish.
//!
//! Wraps a Pdfium extraction (per-page text + raw PDF bytes) into the
//! two manifests the CACG trust kernel consumes, then publishes them
//! pair-atomically via `cacg_core::atomic_publish`.
//!
//! `parser_name` / `parser_version` are a declared divergence from
//! the Python oracle: this binary records its true `pdfium-render`
//! identity, not Python's `pypdfium2` strings. This is the DEC-2
//! resolution recorded in
//! `plans/cacg-rust-port-m4-ingest-and-migration-pilot-plan.md`; the
//! `sources_manifest.json` parity comparison whitelists those two
//! fields. `source_sha256`, `extracted_at` (under
//! `KB_FROZEN_CLOCK=1`), and every other field stay byte-equal with
//! the Python oracle.
//!
//! **Out of scope this round** (left for a follow-up under task-m4-3):
//! merging with prior `sources_manifest.json` / `chunks_manifest.json`
//! when a second ingest lands in a non-empty `out_dir`. The Python
//! `publish_sources_and_chunks` loads the prior manifests, strips
//! entries matching the new `source_id`, and appends the new
//! records. This builder + publisher only handles the
//! first-ingest-in-an-empty-out_dir case.

use std::path::{Path, PathBuf};

use serde_json::Value;
use thiserror::Error;

use cacg_core::diagnostic::codes as cc;

use cacg_core::atomic_publish::{atomic_publish, DefaultFs, PublishError, PublishMember};
use cacg_core::canonical_json::canonical_json;
use cacg_core::determinism::DeterminismContext;
use cacg_core::hash::source_sha256;
use cacg_core::schema::{ChunksManifest, SchemaVersion, SourceRecord, SourcesManifest};

use crate::chunker::{chunk_pages, ChunkConfig, ChunkerError};

/// Parser-identity name the Rust binary records in `SourceRecord`.
/// Honest about the binding crate (not Python's "pypdfium2").
pub const PARSER_NAME: &str = "pdfium-render";

/// Parser-identity version the Rust binary records in `SourceRecord`.
///
/// MUST be kept in lockstep with the workspace `pdfium-render` pin in
/// the root `Cargo.toml` so the `sources_manifest.json` field stays
/// accurate. (The Pdfium *binary* version is a separate concern,
/// pinned per `docs/pdfium-binary-provisioning.md` when the AC-5
/// parity checkpoint resolves.)
pub const PARSER_VERSION: &str = "0.9.1";

/// Errors returned by the manifest builder + publisher.
#[derive(Debug, Error)]
pub enum ManifestError {
    /// `source_id` was empty.
    #[error("source_id must be non-empty")]
    EmptySourceId,
    /// `source_path` was empty.
    #[error("source_path must be non-empty")]
    EmptySourcePath,
    /// Extraction produced zero pages.
    #[error("page_texts must contain at least one page")]
    NoPages,
    /// Page count overflowed `u32`.
    #[error("page count overflow (more than u32::MAX pages)")]
    PageCountOverflow,
    /// The chunker rejected the page texts.
    #[error("chunker failed: {0}")]
    Chunker(#[from] ChunkerError),
    /// Canonical-JSON serialization failed (in practice unreachable
    /// for the structurally-valid values this module constructs).
    #[error("canonical-JSON serialization failed: {0}")]
    Canonical(String),
    /// `cacg_core::atomic_publish` failed (rolled back to the prior
    /// canonical state before returning).
    #[error(transparent)]
    Publish(#[from] PublishError),
    /// `publish_manifests` refused to write into a non-empty `out_dir`
    /// because merging with prior manifests is not yet implemented.
    /// Until the merge-with-prior support lands, callers must publish
    /// into a fresh `out_dir` (or clear the prior manifests
    /// themselves). Carries `CACG-INGEST-003` so operators see the
    /// same code surface they would for any ingest-publish failure.
    #[error("{code}: refusing to publish into {out_dir:?}: prior sources_manifest.json or chunks_manifest.json already present, and merge with prior manifests is not yet implemented", code = cc::INGEST_003)]
    PriorManifestsPresent {
        /// The `out_dir` that already contains prior manifests.
        out_dir: PathBuf,
    },
}

/// Output of the builder: the two manifests, ready to serialize +
/// publish (or to inspect from a test).
#[derive(Debug, Clone)]
pub struct IngestOutput {
    /// The `sources_manifest.json` payload.
    pub sources: SourcesManifest,
    /// The `chunks_manifest.json` payload.
    pub chunks: ChunksManifest,
}

/// Build the manifest pair from a Pdfium extraction, using the
/// default chunker tunables.
///
/// Thin wrapper over [`build_manifests_with_config`] that passes
/// `ChunkConfig::default()`. Callers that need to honor a
/// user-supplied `--config` YAML must use the `_with_config` form.
///
/// - `source_id`: stable identifier stored in both manifests; must
///   match the prefix the chunker uses on `chunk_id`.
/// - `source_path`: filesystem path of the input PDF, recorded
///   verbatim in `SourceRecord.source_path`.
/// - `pdf_bytes`: raw PDF bytes, hashed into `source_sha256`.
/// - `page_texts`: 1-indexed per-page text returned by
///   `extract_pages`; both `page_count` and the chunker consume it.
/// - `determinism`: governs `extracted_at` (frozen → epoch).
///
/// # Errors
///
/// - [`ManifestError::EmptySourceId`] / [`ManifestError::EmptySourcePath`].
/// - [`ManifestError::NoPages`] when `page_texts` is empty.
/// - [`ManifestError::PageCountOverflow`] when `page_texts.len() > u32::MAX`.
/// - [`ManifestError::Chunker`] forwarding any chunker validation error.
pub fn build_manifests(
    source_id: &str,
    source_path: &str,
    pdf_bytes: &[u8],
    page_texts: &[(u32, &str)],
    determinism: &DeterminismContext,
) -> Result<IngestOutput, ManifestError> {
    build_manifests_with_config(
        source_id,
        source_path,
        pdf_bytes,
        page_texts,
        determinism,
        &ChunkConfig::default(),
    )
}

/// Build the manifest pair from a Pdfium extraction, honoring a
/// caller-supplied [`ChunkConfig`].
///
/// Same contract as [`build_manifests`] but threads `chunk_config`
/// into the chunker so a user-supplied `--config` YAML's
/// `target_tokens` / `overlap_tokens` / `max_pages_per_chunk`
/// overrides take effect. Mirrors Python `_cmd_ingest`'s
/// `chunk_pages(..., target_tokens=cfg.target_tokens, ...)` call.
///
/// # Errors
///
/// Identical to [`build_manifests`].
pub fn build_manifests_with_config(
    source_id: &str,
    source_path: &str,
    pdf_bytes: &[u8],
    page_texts: &[(u32, &str)],
    determinism: &DeterminismContext,
    chunk_config: &ChunkConfig,
) -> Result<IngestOutput, ManifestError> {
    build_manifests_with_parser(
        source_id,
        source_path,
        pdf_bytes,
        page_texts,
        determinism,
        chunk_config,
        PARSER_NAME,
        PARSER_VERSION,
    )
}

/// Build the manifest pair with a caller-supplied parser identity.
///
/// The PDF path records the `pdfium-render` identity via
/// [`build_manifests_with_config`]; the utterances path records its
/// own honest identity (`cacg-utterances` + crate version). Everything
/// else — source hashing, chunking, manifest shape — is shared.
///
/// # Errors
///
/// Identical to [`build_manifests`].
#[allow(clippy::too_many_arguments)]
pub fn build_manifests_with_parser(
    source_id: &str,
    source_path: &str,
    source_bytes: &[u8],
    page_texts: &[(u32, &str)],
    determinism: &DeterminismContext,
    chunk_config: &ChunkConfig,
    parser_name: &str,
    parser_version: &str,
) -> Result<IngestOutput, ManifestError> {
    if source_id.is_empty() {
        return Err(ManifestError::EmptySourceId);
    }
    if source_path.is_empty() {
        return Err(ManifestError::EmptySourcePath);
    }
    if page_texts.is_empty() {
        return Err(ManifestError::NoPages);
    }

    let page_count =
        u32::try_from(page_texts.len()).map_err(|_| ManifestError::PageCountOverflow)?;

    let source = SourceRecord {
        schema_version: SchemaVersion::V0,
        source_id: source_id.to_owned(),
        source_path: source_path.to_owned(),
        source_sha256: source_sha256(source_bytes),
        parser_name: parser_name.to_owned(),
        parser_version: parser_version.to_owned(),
        page_count,
        extracted_at: determinism.now_iso(),
    };
    let sources = SourcesManifest {
        schema_version: SchemaVersion::V0,
        sources: vec![source],
    };

    let chunk_records = chunk_pages(source_id, page_texts, chunk_config)?;
    let chunks = ChunksManifest {
        schema_version: SchemaVersion::V0,
        chunks: chunk_records,
        retracted_source_ids: Vec::new(),
        retracted_chunk_ids: Vec::new(),
    };

    Ok(IngestOutput { sources, chunks })
}

/// Errors rejecting a fail-closed `--append` re-ingest.
#[derive(Debug, Error)]
pub enum AppendError {
    /// The out dir has no prior manifests to append to.
    #[error("append requires prior manifests in the out dir; run a plain ingest first")]
    NoPrior,
    /// The prior manifests hold chunks of a different source.
    #[error("append target holds source {found:?}, not {expected:?}; append is single-source")]
    SourceMismatch {
        /// The source id found in the prior manifest.
        found: String,
        /// The source id being ingested.
        expected: String,
    },
    /// The source was retracted; appending to it is forbidden.
    #[error("source {0:?} is retracted; refusing to append")]
    SourceRetracted(String),
    /// A previously-published chunk did not re-derive byte-identical
    /// from the new stream — the new stream is NOT an append (an
    /// utterance was edited, removed, or inserted mid-stream).
    /// NO silent re-anchor: the operator must re-scrape to a NEW
    /// source_id or resolve the upstream edit explicitly.
    #[error(
        "{count} previously-published chunk(s) did not re-derive byte-identical \
             (first: {first}); the stream is not an append — no silent re-anchor"
    )]
    PriorChunkChanged {
        /// How many prior chunks failed to re-derive.
        count: usize,
        /// The first offending chunk id.
        first: String,
    },
    /// The new stream adds nothing beyond the prior state.
    #[error("the stream adds no new chunks beyond the {0} already published; nothing to append")]
    NothingToAppend(usize),
    /// Prior manifest bytes failed to parse/validate.
    #[error("prior manifest invalid: {0}")]
    PriorInvalid(String),
}

/// Validate that `new_output` is a true APPEND over `prior`: every
/// previously-published active chunk re-derives byte-identical
/// (same `chunk_id`, same `chunk_hash`) from the new stream, and at
/// least one new chunk exists. On success, returns the merged
/// manifest pair: the new chunks with the prior retractions carried
/// over (retracted chunk ids stay retracted — their re-derived
/// chunks are dropped from the active list).
///
/// # Errors
///
/// Any [`AppendError`]; nothing is mutated on failure.
pub fn validate_append(
    prior: &ChunksManifest,
    new_output: &IngestOutput,
    source_id: &str,
) -> Result<(IngestOutput, usize), AppendError> {
    if let Some(other) = prior
        .chunks
        .iter()
        .map(|c| c.source_id.as_str())
        .find(|s| *s != source_id)
    {
        return Err(AppendError::SourceMismatch {
            found: other.to_owned(),
            expected: source_id.to_owned(),
        });
    }
    if prior.retracted_source_ids.iter().any(|s| s == source_id) {
        return Err(AppendError::SourceRetracted(source_id.to_owned()));
    }

    let new_by_id: std::collections::HashMap<&str, &str> = new_output
        .chunks
        .chunks
        .iter()
        .map(|c| (c.chunk_id.as_str(), c.chunk_hash.as_str()))
        .collect();
    let mut changed: Vec<&str> = Vec::new();
    for c in &prior.chunks {
        match new_by_id.get(c.chunk_id.as_str()) {
            Some(h) if *h == c.chunk_hash => {}
            _ => changed.push(&c.chunk_id),
        }
    }
    if !changed.is_empty() {
        return Err(AppendError::PriorChunkChanged {
            count: changed.len(),
            first: changed[0].to_owned(),
        });
    }

    let retracted: std::collections::HashSet<&str> = prior
        .retracted_chunk_ids
        .iter()
        .map(String::as_str)
        .collect();
    let prior_active: std::collections::HashSet<&str> =
        prior.chunks.iter().map(|c| c.chunk_id.as_str()).collect();
    let merged_chunks: Vec<_> = new_output
        .chunks
        .chunks
        .iter()
        .filter(|c| !retracted.contains(c.chunk_id.as_str()))
        .cloned()
        .collect();
    let appended = merged_chunks
        .iter()
        .filter(|c| !prior_active.contains(c.chunk_id.as_str()))
        .count();
    if appended == 0 {
        return Err(AppendError::NothingToAppend(prior.chunks.len()));
    }

    let merged = IngestOutput {
        sources: new_output.sources.clone(),
        chunks: ChunksManifest {
            schema_version: SchemaVersion::V0,
            chunks: merged_chunks,
            retracted_source_ids: prior.retracted_source_ids.clone(),
            retracted_chunk_ids: prior.retracted_chunk_ids.clone(),
        },
    };
    Ok((merged, appended))
}

/// Sidecar paths for one canonical manifest file: `.tmp` + `.bak`,
/// both in the same directory so POSIX `rename` is atomic.
fn sidecars(canonical: &Path) -> (PathBuf, PathBuf) {
    let mut tmp = canonical.to_path_buf();
    let mut bak = canonical.to_path_buf();
    let tmp_name = format!(
        "{}.tmp",
        canonical
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("manifest.json")
    );
    let bak_name = format!(
        "{}.bak",
        canonical
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("manifest.json")
    );
    tmp.set_file_name(tmp_name);
    bak.set_file_name(bak_name);
    (tmp, bak)
}

/// Serialize an [`IngestOutput`] to the canonical-JSON bytes the
/// publisher will commit. Exposed for tests / callers that want the
/// bytes without writing to disk.
///
/// Returns `(sources_bytes, chunks_bytes)`.
pub fn render_manifests(output: &IngestOutput) -> Result<(Vec<u8>, Vec<u8>), ManifestError> {
    let sources_value: Value = serde_json::to_value(&output.sources)
        .map_err(|e| ManifestError::Canonical(format!("sources: {e}")))?;
    let chunks_value: Value = serde_json::to_value(&output.chunks)
        .map_err(|e| ManifestError::Canonical(format!("chunks: {e}")))?;
    let sources_bytes = canonical_json(&sources_value)
        .map_err(|e| ManifestError::Canonical(format!("sources: {e:?}")))?
        .into_bytes();
    let chunks_bytes = canonical_json(&chunks_value)
        .map_err(|e| ManifestError::Canonical(format!("chunks: {e:?}")))?
        .into_bytes();
    Ok((sources_bytes, chunks_bytes))
}

/// Pair-atomically publish `sources_manifest.json` and
/// `chunks_manifest.json` into `out_dir` via
/// `cacg_core::atomic_publish`.
///
/// On any I/O failure the publisher rolls back to the prior canonical
/// state (if any) before returning [`ManifestError::Publish`]. Empty
/// `out_dir` is created. Existing `.tmp`/`.bak` sidecars or non-file
/// canonicals are rejected with the project's `CACG-MAN-002` /
/// `CACG-MAN-003` diagnostic codes respectively.
///
/// **Limitation**: this round does not merge with prior manifests
/// already present in `out_dir`. Until merge support lands, this
/// function rejects a non-empty `out_dir` outright with
/// [`ManifestError::PriorManifestsPresent`] (`CACG-INGEST-003`) so
/// no silent data loss can occur when a user runs `kb ingest` twice
/// into the same directory. Callers must publish into a fresh
/// `out_dir` (or remove the prior manifests themselves).
///
/// # Errors
///
/// - [`ManifestError::PriorManifestsPresent`] if a prior
///   `sources_manifest.json` or `chunks_manifest.json` exists in
///   `out_dir`.
/// - [`ManifestError::Canonical`] if serialization fails.
/// - [`ManifestError::Publish`] forwarding any
///   [`cacg_core::atomic_publish::PublishError`].
pub fn publish_manifests(out_dir: &Path, output: &IngestOutput) -> Result<(), ManifestError> {
    publish_manifests_with_locator(out_dir, output, None)
}

/// Like [`publish_manifests`], with an optional third atomic member:
/// the sealed `locator_map.json` sidecar the utterances backend emits
/// (chunk_id → utterance identities). All members publish in ONE
/// atomic group — a failure rolls every file back, so the locator can
/// never exist without its manifests or vice versa.
///
/// # Errors
///
/// Identical to [`publish_manifests`]; a pre-existing
/// `locator_map.json` also raises
/// [`ManifestError::PriorManifestsPresent`].
pub fn publish_manifests_with_locator(
    out_dir: &Path,
    output: &IngestOutput,
    locator_bytes: Option<&[u8]>,
) -> Result<(), ManifestError> {
    publish_manifests_impl(out_dir, output, locator_bytes, false)
}

/// Replace-capable publish used by the fail-closed `--append` path
/// AFTER the byte-identical re-derivation proof has passed: the prior
/// canonicals are atomically replaced (with `.bak` rollback on
/// failure) instead of refused. Never use without that proof.
pub fn publish_manifests_replacing(
    out_dir: &Path,
    output: &IngestOutput,
    locator_bytes: Option<&[u8]>,
) -> Result<(), ManifestError> {
    publish_manifests_impl(out_dir, output, locator_bytes, true)
}

fn publish_manifests_impl(
    out_dir: &Path,
    output: &IngestOutput,
    locator_bytes: Option<&[u8]>,
    replace_prior: bool,
) -> Result<(), ManifestError> {
    let fs = DefaultFs;
    // Ensure out_dir exists (mkdir -p semantics; idempotent).
    std::fs::create_dir_all(out_dir)
        .map_err(|e| ManifestError::Publish(PublishError::Io { source: e }))?;

    let sources_path = out_dir.join("sources_manifest.json");
    let chunks_path = out_dir.join("chunks_manifest.json");
    let locator_path = out_dir.join("locator_map.json");

    // Outside the proven append path, refuse to clobber existing
    // manifests rather than silently destroy data.
    if !replace_prior
        && (sources_path.exists()
            || chunks_path.exists()
            || (locator_bytes.is_some() && locator_path.exists()))
    {
        return Err(ManifestError::PriorManifestsPresent {
            out_dir: out_dir.to_path_buf(),
        });
    }

    let (sources_bytes, chunks_bytes) = render_manifests(output)?;

    let (sources_tmp, sources_bak) = sidecars(&sources_path);
    let (chunks_tmp, chunks_bak) = sidecars(&chunks_path);

    let mut members = vec![
        PublishMember {
            canonical_path: sources_path,
            tmp_path: sources_tmp,
            bak_path: sources_bak,
            bytes: sources_bytes,
        },
        PublishMember {
            canonical_path: chunks_path,
            tmp_path: chunks_tmp,
            bak_path: chunks_bak,
            bytes: chunks_bytes,
        },
    ];
    if let Some(bytes) = locator_bytes {
        let (locator_tmp, locator_bak) = sidecars(&locator_path);
        members.push(PublishMember {
            canonical_path: locator_path,
            tmp_path: locator_tmp,
            bak_path: locator_bak,
            bytes: bytes.to_vec(),
        });
    }

    atomic_publish(&members, &fs, cc::MAN_002, cc::MAN_003)?;
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_source_id() {
        let pages: Vec<(u32, &str)> = vec![(1u32, "hello")];
        let det = DeterminismContext::frozen();
        let err = build_manifests("", "a.pdf", b"pdf", &pages, &det).unwrap_err();
        assert!(matches!(err, ManifestError::EmptySourceId));
    }

    #[test]
    fn rejects_empty_source_path() {
        let pages: Vec<(u32, &str)> = vec![(1u32, "hello")];
        let det = DeterminismContext::frozen();
        let err = build_manifests("sid", "", b"pdf", &pages, &det).unwrap_err();
        assert!(matches!(err, ManifestError::EmptySourcePath));
    }

    #[test]
    fn rejects_no_pages() {
        let pages: Vec<(u32, &str)> = Vec::new();
        let det = DeterminismContext::frozen();
        let err = build_manifests("sid", "a.pdf", b"pdf", &pages, &det).unwrap_err();
        assert!(matches!(err, ManifestError::NoPages));
    }

    #[test]
    fn build_manifests_records_expected_source_fields_under_frozen_clock() {
        let pages: Vec<(u32, &str)> = vec![(1u32, "Hello world. Short paragraph.")];
        let det = DeterminismContext::frozen();
        let out = build_manifests("sid", "a.pdf", b"raw-bytes", &pages, &det).unwrap();
        assert_eq!(out.sources.sources.len(), 1);
        let s = &out.sources.sources[0];
        assert_eq!(s.source_id, "sid");
        assert_eq!(s.source_path, "a.pdf");
        assert_eq!(s.parser_name, "pdfium-render");
        assert_eq!(s.parser_version, "0.9.1");
        assert_eq!(s.page_count, 1);
        assert_eq!(s.extracted_at, "1970-01-01T00:00:00Z");
        // source_sha256 = SHA-256("raw-bytes") in hex, 64 chars.
        assert_eq!(s.source_sha256.len(), 64);
        // Chunks manifest is non-empty and uses the source_id as prefix.
        assert!(!out.chunks.chunks.is_empty());
        assert!(out.chunks.chunks[0].chunk_id.starts_with("sid:"));
    }

    #[test]
    fn render_manifests_emits_canonical_sorted_keys() {
        let pages: Vec<(u32, &str)> = vec![(1u32, "Hello world.")];
        let det = DeterminismContext::frozen();
        let out = build_manifests("sid", "a.pdf", b"x", &pages, &det).unwrap();
        let (sources_bytes, _chunks_bytes) = render_manifests(&out).unwrap();
        let sources_str = std::str::from_utf8(&sources_bytes).unwrap();
        // Canonical JSON sorts keys alphabetically; "schema_version"
        // sorts before "sources" at the top level.
        let schema_idx = sources_str.find("\"schema_version\"").unwrap();
        let sources_idx = sources_str.find("\"sources\"").unwrap();
        assert!(schema_idx < sources_idx, "keys must be alphabetical");
        // Inside a source record, "extracted_at" sorts first.
        assert!(
            sources_str.contains("{\"extracted_at\":"),
            "source record must start with extracted_at (alphabetical)"
        );
    }

    #[test]
    fn publish_manifests_writes_both_files_to_tempdir() {
        let pages: Vec<(u32, &str)> = vec![(1u32, "Hello world. Short paragraph here.")];
        let det = DeterminismContext::frozen();
        let out = build_manifests("sid", "a.pdf", b"x", &pages, &det).unwrap();

        let tmp = tempfile::tempdir().expect("tempdir");
        publish_manifests(tmp.path(), &out).expect("publish must succeed");

        let sources_path = tmp.path().join("sources_manifest.json");
        let chunks_path = tmp.path().join("chunks_manifest.json");
        assert!(sources_path.is_file(), "sources_manifest.json must exist");
        assert!(chunks_path.is_file(), "chunks_manifest.json must exist");

        // Sidecars must be cleaned up on success.
        assert!(!tmp.path().join("sources_manifest.json.tmp").exists());
        assert!(!tmp.path().join("sources_manifest.json.bak").exists());
        assert!(!tmp.path().join("chunks_manifest.json.tmp").exists());
        assert!(!tmp.path().join("chunks_manifest.json.bak").exists());
    }

    #[test]
    fn publish_manifests_into_existing_dir_writes_fresh_files() {
        // out_dir already exists (mkdir is idempotent).
        let tmp = tempfile::tempdir().expect("tempdir");
        let pages: Vec<(u32, &str)> = vec![(1u32, "Hello world. Para.")];
        let det = DeterminismContext::frozen();
        let out = build_manifests("sid", "a.pdf", b"x", &pages, &det).unwrap();
        publish_manifests(tmp.path(), &out).expect("publish must succeed");
        assert!(tmp.path().join("sources_manifest.json").is_file());
    }

    #[test]
    fn publish_manifests_refuses_when_prior_manifests_present() {
        // Round-5 limitation: merge isn't implemented, so a second
        // publish into the same out_dir must fail loudly rather than
        // silently destroy the prior data.
        let tmp = tempfile::tempdir().expect("tempdir");
        let pages: Vec<(u32, &str)> = vec![(1u32, "Hello world. Para.")];
        let det = DeterminismContext::frozen();
        let out = build_manifests("sid", "a.pdf", b"x", &pages, &det).unwrap();

        // First publish: succeeds.
        publish_manifests(tmp.path(), &out).expect("first publish must succeed");

        // Second publish into the same out_dir: must error.
        let err = publish_manifests(tmp.path(), &out).expect_err("second publish must fail");
        match err {
            ManifestError::PriorManifestsPresent { out_dir } => {
                assert_eq!(out_dir, tmp.path());
                // The Display surface must carry the CACG-INGEST-003
                // operator code so the diagnostic is grep-friendly.
                let msg = format!("{}", ManifestError::PriorManifestsPresent { out_dir });
                assert!(
                    msg.contains("CACG-INGEST-003"),
                    "error display must surface CACG-INGEST-003; got: {msg}"
                );
            }
            ManifestError::EmptySourceId
            | ManifestError::EmptySourcePath
            | ManifestError::NoPages
            | ManifestError::PageCountOverflow
            | ManifestError::Chunker(_)
            | ManifestError::Canonical(_)
            | ManifestError::Publish(_) => {
                panic!("expected PriorManifestsPresent, got {err:?}")
            }
        }
    }

    #[test]
    fn build_manifests_with_config_honors_smaller_target_tokens() {
        // A tighter target_tokens budget on the same page text must
        // produce strictly MORE chunks than the default. This is the
        // observable difference between build_manifests (defaults)
        // and build_manifests_with_config (caller-supplied) — proves
        // the chunk_config plumbed all the way through.
        let big_page = "Lorem ipsum dolor sit amet. ".repeat(200); // ~1000 tokens
        let pages: Vec<(u32, &str)> = vec![(1u32, big_page.as_str())];
        let det = DeterminismContext::frozen();

        let default_out = build_manifests("sid", "a.pdf", b"x", &pages, &det).unwrap();
        let tight_cfg = ChunkConfig {
            target_tokens: 50,
            overlap_tokens: 5,
            max_pages_per_chunk: 2,
        };
        let tight_out =
            build_manifests_with_config("sid", "a.pdf", b"x", &pages, &det, &tight_cfg).unwrap();

        assert!(
            tight_out.chunks.chunks.len() > default_out.chunks.chunks.len(),
            "tight target_tokens=50 must yield more chunks than default ({}); \
             got tight={} default={}",
            ChunkConfig::default().target_tokens,
            tight_out.chunks.chunks.len(),
            default_out.chunks.chunks.len(),
        );
    }

    #[test]
    fn parser_version_matches_workspace_pdfium_render_pin() {
        // Closes the silent-drift hazard between the hardcoded
        // PARSER_VERSION constant and the workspace pdfium-render
        // pin in the root Cargo.toml. Bumping the pin requires
        // updating PARSER_VERSION in lockstep; this test ensures
        // a missed bump is caught at test time, not at runtime when
        // sources_manifest.json suddenly carries the wrong version.
        let workspace_cargo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("Cargo.toml");
        let contents =
            std::fs::read_to_string(&workspace_cargo).expect("workspace Cargo.toml must exist");
        let line = contents
            .lines()
            .find(|l| {
                let t = l.trim_start();
                t.starts_with("pdfium-render ") || t.starts_with("pdfium-render=")
            })
            .expect("workspace Cargo.toml must declare pdfium-render");
        // Expected shape: `pdfium-render = "0.9.1"`. Extract the
        // first quoted substring.
        let pinned = line
            .split('"')
            .nth(1)
            .expect("pdfium-render line must carry a quoted version");
        assert_eq!(
            pinned, PARSER_VERSION,
            "workspace pdfium-render pin (\"{pinned}\") must match \
             cacg_ingest::manifest::PARSER_VERSION (\"{PARSER_VERSION}\"); \
             bump them in lockstep"
        );
    }
}
