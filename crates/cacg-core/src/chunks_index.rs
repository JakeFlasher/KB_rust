//! Process-local index over `chunks_manifest.json` shared by lint and
//! verify. Mirrors Python `legacy_python_oracle/src/cacg/chunks_index.py` byte-equal in
//! semantics.
//!
//! Built so a batch operation (`kb verify --round-summary` over 1000
//! cards, `kb lint --all-readings` over a full corpus) loads and
//! validates the manifest exactly once, then services every per-card
//! lookup from in-memory dicts. Per-card cost becomes O(citations)
//! instead of O(citations + manifest size).
//!
//! Boundary discipline (mirroring Python):
//!
//! 1. Loading + structural-validating `chunks_manifest.json`.
//! 2. Rejecting duplicate `chunk_id` at the trust boundary
//!    (`BL-20260518-reject-duplicate-keys-at-trust-boundary`).
//! 3. Memoizing the layer-2 tamper-status recomputation per `chunk_id`
//!    for the lifetime of the index.
//!
//! Concurrency posture: the tamper cache lives behind a `RefCell` so a
//! shared `&ChunksIndex` can mutate its memo table on read. Single-
//! threaded today; if a later phase introduces Rayon-driven parallel
//! verify, swap the `RefCell` for a `Mutex<BTreeMap<...>>` — the public
//! API will not change.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::canonical_json::CanonicalError;
use crate::diagnostic::Diagnostic;
use crate::hash::{chunk_hash, PageSpan as HashPageSpan};
use crate::schema::{ChunkRecord, ChunksManifest, PageSpan as SchemaPageSpan};

/// Raised when `chunks_manifest.json` cannot be read or fails any
/// trust-boundary validation step. Downstream `cacg-cli` callers map
/// these to `CACG-MAN-001` per-card so the journal records exactly one
/// event per card; the file-shape failures may also map to
/// `CACG-CLI-001` depending on the caller's context.
#[derive(Debug, Error)]
pub enum ChunksIndexLoadError {
    /// Path is not a regular file (missing, directory, broken symlink).
    #[error("chunks_manifest path is not a regular file: {0}")]
    NonFile(PathBuf),
    /// I/O failure reading the manifest bytes.
    #[error("chunks_manifest.json is invalid: {path}: {source}")]
    Io {
        /// Manifest path.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// JSON parse / deny-unknown-fields failure.
    #[error("chunks_manifest.json is invalid: {0}")]
    Parse(String),
    /// Structural validation failure (page-span monotonicity, active /
    /// retracted disjointness, sorted-unique retracted lists, etc.).
    #[error("chunks_manifest.json is invalid: {0}")]
    Validate(String),
    /// Duplicate `chunk_id` rejected at the trust boundary. Carries
    /// the offending ids sorted-unique for diagnostic formatting.
    #[error(
        "chunks_manifest.json has duplicate chunk_id(s): {0:?}; refusing to build an ambiguous index"
    )]
    DuplicateChunkId(Vec<String>),
    /// `chunk_hash` recomputation failed during tamper check
    /// (canonical-JSON serialization error). The wrapped error is
    /// effectively unreachable for valid Pydantic-shape data.
    #[error("chunk_hash recompute failed for {chunk_id}: {source}")]
    HashRecompute {
        /// The chunk whose hash recompute failed.
        chunk_id: String,
        /// Underlying canonical-JSON error.
        #[source]
        source: CanonicalError,
    },
    /// `chunk_id` not present in the indexed manifest. Mirrors Python's
    /// `KeyError` fail-loud behavior on `tamper_status` / `tamper_check`
    /// against an unknown id.
    #[error("chunk_id {0:?} is not in the manifest")]
    MissingChunk(String),
}

/// Tamper-check failure payload. Returned by [`ChunksIndex::tamper_check`]
/// when a chunk's recomputed hash does not match its stored
/// `chunk_hash`. Lint surfaces this as `CACG-HASH-003` or `CACG-CITE-005`
/// depending on the call site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashMismatch {
    /// The chunk whose hash failed to match.
    pub chunk_id: String,
    /// The hash recorded in the manifest.
    pub expected: String,
    /// The hash recomputed from the chunk's current text + page-span
    /// envelope.
    pub actual: String,
}

impl HashMismatch {
    /// Project the mismatch into a diagnostic carrying `CACG-HASH-003`.
    /// Callers that prefer a different code (e.g., layer-1's
    /// `CACG-CITE-005` for hash drift detected at chunk lookup time)
    /// build their own diagnostic from the struct fields.
    #[must_use]
    pub fn to_diagnostic(&self) -> Diagnostic {
        use crate::diagnostic::{codes, Severity};
        Diagnostic::new(
            codes::HASH_003,
            Severity::Error,
            format!(
                "chunk {0:?} tamper detected: expected chunk_hash {1}, recomputed {2}",
                self.chunk_id, self.expected, self.actual
            ),
        )
    }
}

/// In-memory index over a validated `ChunksManifest`. Owns the manifest
/// data; lookups borrow from the owned chunks. The tamper cache is
/// behind `RefCell` so `&self` reads can populate memoization without
/// requiring `&mut self`.
#[derive(Debug)]
pub struct ChunksIndex {
    manifest: ChunksManifest,
    by_id: BTreeMap<String, usize>,
    by_source: BTreeMap<String, Vec<usize>>,
    retracted_sources: BTreeSet<String>,
    retracted_chunks: BTreeSet<String>,
    tamper_cache: RefCell<BTreeMap<String, bool>>,
}

impl ChunksIndex {
    /// Build an index from an already-loaded [`ChunksManifest`].
    /// Mirrors Python `ChunksIndex.from_manifest`. Runs cross-field
    /// validators first (so duplicate retracted-list entries and active
    /// vs retracted disjointness violations fail-fast), then rejects
    /// duplicate `chunk_id`s, then builds the lookup maps preserving
    /// the manifest's chunk order within each source.
    ///
    /// # Errors
    ///
    /// Returns [`ChunksIndexLoadError::Validate`] if `validate_structurally`
    /// rejects the manifest, or [`ChunksIndexLoadError::DuplicateChunkId`]
    /// when the same `chunk_id` appears more than once.
    pub fn from_manifest(manifest: ChunksManifest) -> Result<Self, ChunksIndexLoadError> {
        if let Err(d) = manifest.validate_structurally() {
            return Err(ChunksIndexLoadError::Validate(d.message));
        }
        // Duplicate-chunk_id rejection BEFORE the by_id insert. Python
        // accumulates a `duplicates: list` and raises with the sorted
        // unique set; we mirror.
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        let mut duplicates: BTreeSet<String> = BTreeSet::new();
        for c in &manifest.chunks {
            if !seen.insert(c.chunk_id.as_str()) {
                duplicates.insert(c.chunk_id.clone());
            }
        }
        if !duplicates.is_empty() {
            return Err(ChunksIndexLoadError::DuplicateChunkId(
                duplicates.into_iter().collect(),
            ));
        }
        let mut by_id: BTreeMap<String, usize> = BTreeMap::new();
        let mut by_source: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (idx, c) in manifest.chunks.iter().enumerate() {
            by_id.insert(c.chunk_id.clone(), idx);
            by_source.entry(c.source_id.clone()).or_default().push(idx);
        }
        let retracted_sources: BTreeSet<String> =
            manifest.retracted_source_ids.iter().cloned().collect();
        let retracted_chunks: BTreeSet<String> =
            manifest.retracted_chunk_ids.iter().cloned().collect();
        Ok(Self {
            manifest,
            by_id,
            by_source,
            retracted_sources,
            retracted_chunks,
            tamper_cache: RefCell::new(BTreeMap::new()),
        })
    }

    /// Load + structurally-validate + index a `chunks_manifest.json`
    /// in one step. Mirrors Python `ChunksIndex.from_path`. Applies
    /// `Path::is_file()` preflight (per `BL-20260518-shape-check-fs-inputs`).
    ///
    /// # Errors
    ///
    /// Returns the appropriate [`ChunksIndexLoadError`] variant on file-
    /// shape, I/O, parse, or validate failure.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ChunksIndexLoadError> {
        let path = path.as_ref();
        if !path.is_file() {
            return Err(ChunksIndexLoadError::NonFile(path.to_path_buf()));
        }
        let raw = std::fs::read_to_string(path).map_err(|source| ChunksIndexLoadError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        let manifest: ChunksManifest =
            serde_json::from_str(&raw).map_err(|e| ChunksIndexLoadError::Parse(e.to_string()))?;
        Self::from_manifest(manifest)
    }

    /// Borrow the underlying validated manifest.
    #[must_use]
    pub fn manifest(&self) -> &ChunksManifest {
        &self.manifest
    }

    /// Look up a chunk by its `chunk_id`. Returns `None` if the id is
    /// not present in the manifest. Mirrors Python's `by_id.get(...)`.
    #[must_use]
    pub fn get(&self, chunk_id: &str) -> Option<&ChunkRecord> {
        self.by_id
            .get(chunk_id)
            .and_then(|i| self.manifest.chunks.get(*i))
    }

    /// All chunks belonging to `source_id`, in manifest-original order.
    /// Empty vector on unknown source. Mirrors Python
    /// `chunks_by_source`.
    #[must_use]
    pub fn chunks_by_source(&self, source_id: &str) -> Vec<&ChunkRecord> {
        match self.by_source.get(source_id) {
            Some(indices) => indices
                .iter()
                .filter_map(|i| self.manifest.chunks.get(*i))
                .collect(),
            None => Vec::new(),
        }
    }

    /// True iff `chunk_id` is in `retracted_chunk_ids`. The retraction
    /// state mirrors layer-2's `RETR_003` check.
    #[must_use]
    pub fn is_retracted(&self, chunk_id: &str) -> bool {
        self.retracted_chunks.contains(chunk_id)
    }

    /// True iff `source_id` is in `retracted_source_ids`. Mirrors
    /// layer-2's `RETR_002` check.
    #[must_use]
    pub fn is_source_retracted(&self, source_id: &str) -> bool {
        self.retracted_sources.contains(source_id)
    }

    /// Return `true` if the chunk's recomputed hash matches its stored
    /// `chunk_hash`, `false` otherwise. Memoized per `chunk_id`.
    ///
    /// # Errors
    ///
    /// Returns [`ChunksIndexLoadError::MissingChunk`] if `chunk_id` is
    /// not present in the manifest, or [`ChunksIndexLoadError::HashRecompute`]
    /// if canonical-JSON serialization of the hash envelope fails (an
    /// effectively-unreachable code path for Pydantic-shape data, but
    /// preserved for API symmetry).
    pub fn tamper_status(&self, chunk_id: &str) -> Result<bool, ChunksIndexLoadError> {
        if let Some(cached) = self.tamper_cache.borrow().get(chunk_id) {
            return Ok(*cached);
        }
        let chunk = self
            .get(chunk_id)
            .ok_or_else(|| ChunksIndexLoadError::MissingChunk(chunk_id.to_string()))?;
        let hash_spans: Vec<HashPageSpan> = chunk
            .page_spans
            .iter()
            .map(schema_to_hash_page_span)
            .collect();
        let recomputed = chunk_hash(
            &chunk.text,
            i64::from(chunk.start_page),
            i64::from(chunk.end_page),
            &hash_spans,
        )
        .map_err(|source| ChunksIndexLoadError::HashRecompute {
            chunk_id: chunk_id.to_string(),
            source,
        })?;
        let matches = recomputed == chunk.chunk_hash;
        self.tamper_cache
            .borrow_mut()
            .insert(chunk_id.to_string(), matches);
        Ok(matches)
    }

    /// `Ok(())` when the chunk's hash matches; `Err(HashMismatch)` with
    /// the recomputed value when it does not. Routed through
    /// `tamper_status` so the memoized cache is consulted; on a cache
    /// hit the recomputation is skipped but a fresh recompute happens
    /// only on the failure path so the diagnostic carries the actual
    /// recomputed value.
    ///
    /// # Errors
    ///
    /// Returns [`HashMismatch`] when the chunk's text + page-span
    /// envelope no longer hashes to the manifest's stored
    /// `chunk_hash`. Propagates the underlying
    /// [`ChunksIndexLoadError`] (missing chunk or hash recompute
    /// failure) via the same return-value boundary by wrapping it as a
    /// [`HashMismatch`] with `expected` carrying a sentinel — callers
    /// that need to distinguish these cases should call
    /// [`Self::tamper_status`] directly.
    pub fn tamper_check(&self, chunk_id: &str) -> Result<(), HashMismatch> {
        let matches = self.tamper_status(chunk_id).map_err(|e| HashMismatch {
            chunk_id: chunk_id.to_string(),
            expected: String::new(),
            actual: e.to_string(),
        })?;
        if matches {
            return Ok(());
        }
        let chunk = self
            .get(chunk_id)
            .expect("tamper_status would have errored on missing chunk");
        let hash_spans: Vec<HashPageSpan> = chunk
            .page_spans
            .iter()
            .map(schema_to_hash_page_span)
            .collect();
        let recomputed = chunk_hash(
            &chunk.text,
            i64::from(chunk.start_page),
            i64::from(chunk.end_page),
            &hash_spans,
        )
        .unwrap_or_else(|_| String::new());
        Err(HashMismatch {
            chunk_id: chunk_id.to_string(),
            expected: chunk.chunk_hash.clone(),
            actual: recomputed,
        })
    }
}

fn schema_to_hash_page_span(span: &SchemaPageSpan) -> HashPageSpan {
    HashPageSpan {
        page: i64::from(span.page),
        byte_offset_in_chunk: i64::from(span.byte_offset_in_chunk),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_chunk(
        source_id: &str,
        chunk_id: &str,
        text: &str,
        start_page: u32,
        end_page: u32,
    ) -> ChunkRecord {
        let span = SchemaPageSpan {
            page: start_page,
            byte_offset_in_chunk: 0,
        };
        let hash_spans = vec![schema_to_hash_page_span(&span)];
        let h = chunk_hash(
            text,
            i64::from(start_page),
            i64::from(end_page),
            &hash_spans,
        )
        .expect("chunk_hash");
        ChunkRecord {
            schema_version: crate::schema::SchemaVersion::V0,
            source_id: source_id.to_string(),
            chunk_id: chunk_id.to_string(),
            chunk_hash: h,
            ordinal: 0,
            start_page,
            end_page,
            page_spans: vec![span],
            token_count: 3,
            text: text.to_string(),
            text_preview: text.to_string(),
        }
    }

    fn manifest_from(chunks: Vec<ChunkRecord>) -> ChunksManifest {
        ChunksManifest {
            schema_version: crate::schema::SchemaVersion::V0,
            chunks,
            retracted_source_ids: Vec::new(),
            retracted_chunk_ids: Vec::new(),
        }
    }

    #[test]
    fn from_manifest_builds_by_id_and_by_source() {
        let c1 = make_chunk("src", "src:p001:0000", "alpha beta gamma", 1, 1);
        let c2 = make_chunk("src", "src:p002:0000", "delta epsilon zeta", 2, 2);
        let c3 = make_chunk("other", "other:p001:0000", "other source content", 1, 1);
        let idx =
            ChunksIndex::from_manifest(manifest_from(vec![c1.clone(), c2.clone(), c3.clone()]))
                .expect("build");
        assert_eq!(idx.get("src:p001:0000").unwrap().text, c1.text);
        assert_eq!(idx.get("src:p002:0000").unwrap().text, c2.text);
        assert_eq!(idx.get("other:p001:0000").unwrap().text, c3.text);
        let src_chunks = idx.chunks_by_source("src");
        assert_eq!(src_chunks.len(), 2);
        assert_eq!(src_chunks[0].chunk_id, "src:p001:0000");
        assert_eq!(src_chunks[1].chunk_id, "src:p002:0000");
        let other_chunks = idx.chunks_by_source("other");
        assert_eq!(other_chunks.len(), 1);
        assert_eq!(other_chunks[0].chunk_id, "other:p001:0000");
        assert!(idx.chunks_by_source("missing").is_empty());
    }

    #[test]
    fn from_manifest_rejects_duplicate_chunk_id() {
        let c1 = make_chunk("src", "src:p001:0000", "alpha beta gamma", 1, 1);
        let c2 = make_chunk("src", "src:p001:0000", "conflict", 1, 1);
        let err = ChunksIndex::from_manifest(manifest_from(vec![c1, c2]))
            .expect_err("duplicate must error");
        match err {
            ChunksIndexLoadError::DuplicateChunkId(ids) => {
                assert_eq!(ids, vec!["src:p001:0000".to_string()]);
            }
            ChunksIndexLoadError::NonFile(_)
            | ChunksIndexLoadError::Io { .. }
            | ChunksIndexLoadError::Parse(_)
            | ChunksIndexLoadError::Validate(_)
            | ChunksIndexLoadError::HashRecompute { .. }
            | ChunksIndexLoadError::MissingChunk(_) => {
                panic!("expected DuplicateChunkId, got {err:?}")
            }
        }
    }

    #[test]
    fn from_path_round_trips_through_disk() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("chunks_manifest.json");
        let c1 = make_chunk("src", "src:p001:0000", "alpha beta gamma", 1, 1);
        let payload = serde_json::to_string(&manifest_from(vec![c1.clone()])).unwrap();
        std::fs::write(&path, payload).unwrap();
        let idx = ChunksIndex::from_path(&path).expect("load");
        assert_eq!(idx.get("src:p001:0000").unwrap().chunk_id, c1.chunk_id);
    }

    #[test]
    fn from_path_missing_file_yields_non_file() {
        let err = ChunksIndex::from_path("/nonexistent/__chunks_index_probe__.json")
            .expect_err("missing must error");
        match err {
            ChunksIndexLoadError::NonFile(_) => {}
            ChunksIndexLoadError::Io { .. }
            | ChunksIndexLoadError::Parse(_)
            | ChunksIndexLoadError::Validate(_)
            | ChunksIndexLoadError::DuplicateChunkId(_)
            | ChunksIndexLoadError::HashRecompute { .. }
            | ChunksIndexLoadError::MissingChunk(_) => {
                panic!("expected NonFile, got {err:?}")
            }
        }
    }

    #[test]
    fn from_path_malformed_json_yields_parse() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("manifest.json");
        std::fs::write(&path, "definitely not json").unwrap();
        let err = ChunksIndex::from_path(&path).expect_err("malformed must error");
        match err {
            ChunksIndexLoadError::Parse(_) => {}
            ChunksIndexLoadError::NonFile(_)
            | ChunksIndexLoadError::Io { .. }
            | ChunksIndexLoadError::Validate(_)
            | ChunksIndexLoadError::DuplicateChunkId(_)
            | ChunksIndexLoadError::HashRecompute { .. }
            | ChunksIndexLoadError::MissingChunk(_) => {
                panic!("expected Parse, got {err:?}")
            }
        }
    }

    #[test]
    fn from_path_schema_violation_yields_parse() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("manifest.json");
        // Missing required `chunks` field.
        std::fs::write(&path, json!({"schema_version": "cacg.v0"}).to_string()).unwrap();
        let err = ChunksIndex::from_path(&path).expect_err("schema invalid must error");
        match err {
            ChunksIndexLoadError::Parse(_) => {}
            ChunksIndexLoadError::NonFile(_)
            | ChunksIndexLoadError::Io { .. }
            | ChunksIndexLoadError::Validate(_)
            | ChunksIndexLoadError::DuplicateChunkId(_)
            | ChunksIndexLoadError::HashRecompute { .. }
            | ChunksIndexLoadError::MissingChunk(_) => {
                panic!("expected Parse on missing required field, got {err:?}")
            }
        }
    }

    #[test]
    fn tamper_status_true_for_well_formed_chunk() {
        let c1 = make_chunk("src", "src:p001:0000", "alpha beta gamma", 1, 1);
        let idx = ChunksIndex::from_manifest(manifest_from(vec![c1])).unwrap();
        assert!(idx.tamper_status("src:p001:0000").unwrap());
    }

    #[test]
    fn tamper_status_false_when_text_mutated() {
        let mut c1 = make_chunk("src", "src:p001:0000", "alpha beta gamma", 1, 1);
        // Mutate text after computing hash; manifest will fail the
        // chunk_record validator structurally because the hash no
        // longer matches text — but ChunkRecord::validate doesn't
        // check that (it checks page-span structure only). Construct
        // by directly setting text post-hash.
        c1.text = "DIFFERENT TEXT".to_string();
        let idx = ChunksIndex::from_manifest(manifest_from(vec![c1])).unwrap();
        assert!(!idx.tamper_status("src:p001:0000").unwrap());
    }

    #[test]
    fn tamper_status_is_memoized() {
        let c1 = make_chunk("src", "src:p001:0000", "alpha beta gamma", 1, 1);
        let idx = ChunksIndex::from_manifest(manifest_from(vec![c1])).unwrap();
        assert!(idx.tamper_status("src:p001:0000").unwrap());
        // Inject a fake cached value to prove the cache wins on the
        // second call (parity with Python's _tamper_cache override
        // trick in test_chunks_index.py).
        idx.tamper_cache
            .borrow_mut()
            .insert("src:p001:0000".to_string(), false);
        assert!(!idx.tamper_status("src:p001:0000").unwrap());
    }

    #[test]
    fn tamper_status_missing_chunk_id_errors() {
        let c1 = make_chunk("src", "src:p001:0000", "alpha beta gamma", 1, 1);
        let idx = ChunksIndex::from_manifest(manifest_from(vec![c1])).unwrap();
        let err = idx
            .tamper_status("src:p999:0000")
            .expect_err("missing chunk must error");
        match err {
            ChunksIndexLoadError::MissingChunk(id) => assert_eq!(id, "src:p999:0000"),
            ChunksIndexLoadError::NonFile(_)
            | ChunksIndexLoadError::Io { .. }
            | ChunksIndexLoadError::Parse(_)
            | ChunksIndexLoadError::Validate(_)
            | ChunksIndexLoadError::DuplicateChunkId(_)
            | ChunksIndexLoadError::HashRecompute { .. } => {
                panic!("expected MissingChunk, got {err:?}")
            }
        }
    }

    #[test]
    fn tamper_check_ok_for_well_formed_chunk() {
        let c1 = make_chunk("src", "src:p001:0000", "alpha beta gamma", 1, 1);
        let idx = ChunksIndex::from_manifest(manifest_from(vec![c1])).unwrap();
        idx.tamper_check("src:p001:0000").expect("clean chunk ok");
    }

    #[test]
    fn tamper_check_returns_mismatch_with_recomputed_actual() {
        let mut c1 = make_chunk("src", "src:p001:0000", "alpha beta gamma", 1, 1);
        let expected = c1.chunk_hash.clone();
        c1.text = "DIFFERENT TEXT".to_string();
        let idx = ChunksIndex::from_manifest(manifest_from(vec![c1])).unwrap();
        let err = idx
            .tamper_check("src:p001:0000")
            .expect_err("mutated chunk must mismatch");
        assert_eq!(err.chunk_id, "src:p001:0000");
        assert_eq!(err.expected, expected);
        assert_ne!(err.actual, expected);
        assert_eq!(err.actual.len(), 64);
    }

    #[test]
    fn is_retracted_and_is_source_retracted_pick_up_manifest_lists() {
        let c1 = make_chunk("src", "src:p001:0000", "alpha beta gamma", 1, 1);
        let manifest = ChunksManifest {
            schema_version: crate::schema::SchemaVersion::V0,
            chunks: vec![c1],
            retracted_source_ids: vec!["retired_source".to_string()],
            retracted_chunk_ids: vec!["retired:chunk:001".to_string()],
        };
        let idx = ChunksIndex::from_manifest(manifest).unwrap();
        assert!(idx.is_retracted("retired:chunk:001"));
        assert!(!idx.is_retracted("src:p001:0000"));
        assert!(idx.is_source_retracted("retired_source"));
        assert!(!idx.is_source_retracted("src"));
    }
}
