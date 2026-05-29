#![allow(clippy::unwrap_used)]
//! Parity gate for `cacg_core::chunks_index::ChunksIndex` against the
//! committed Python-built `tests/parity_corpus/out_python/chunks_manifest.json`.
//!
//! Coverage:
//!
//! * Load through `from_path`.
//! * Every chunk's hash recomputes cleanly (Python's `chunk_hash` and
//!   Rust's `chunk_hash` already established byte-equality in M1; this
//!   test re-asserts it through the index API).
//! * `chunks_by_source` preserves manifest insertion order.
//! * `get` returns the same `chunk_ids` that the manifest carries.
//! * Tamper mutation surfaces as a `HashMismatch` carrying the
//!   recomputed actual hash.

use std::path::PathBuf;

use cacg_core::chunks_index::{ChunksIndex, ChunksIndexLoadError};
use cacg_core::schema::ChunksManifest;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn fixture_path() -> PathBuf {
    workspace_root().join("tests/parity_corpus/out_python/chunks_manifest.json")
}

#[test]
fn loads_committed_python_fixture() {
    let idx = ChunksIndex::from_path(fixture_path()).expect("committed fixture must load");
    let raw = std::fs::read_to_string(fixture_path()).unwrap();
    let manifest: ChunksManifest = serde_json::from_str(&raw).unwrap();
    assert!(
        !manifest.chunks.is_empty(),
        "committed fixture must carry at least one chunk"
    );
    for chunk in &manifest.chunks {
        let looked_up = idx
            .get(&chunk.chunk_id)
            .unwrap_or_else(|| panic!("get({}) must succeed", chunk.chunk_id));
        assert_eq!(looked_up.chunk_id, chunk.chunk_id);
        assert_eq!(looked_up.chunk_hash, chunk.chunk_hash);
    }
}

#[test]
fn chunks_by_source_preserves_manifest_order() {
    let idx = ChunksIndex::from_path(fixture_path()).unwrap();
    let raw = std::fs::read_to_string(fixture_path()).unwrap();
    let manifest: ChunksManifest = serde_json::from_str(&raw).unwrap();
    let mut by_source: std::collections::BTreeMap<&str, Vec<&str>> =
        std::collections::BTreeMap::new();
    for c in &manifest.chunks {
        by_source
            .entry(c.source_id.as_str())
            .or_default()
            .push(c.chunk_id.as_str());
    }
    for (source_id, expected_order) in &by_source {
        let actual: Vec<&str> = idx
            .chunks_by_source(source_id)
            .into_iter()
            .map(|c| c.chunk_id.as_str())
            .collect();
        assert_eq!(
            &actual, expected_order,
            "chunks_by_source({source_id:?}) must mirror manifest order"
        );
    }
}

#[test]
fn tamper_status_true_for_every_committed_chunk() {
    let idx = ChunksIndex::from_path(fixture_path()).unwrap();
    let raw = std::fs::read_to_string(fixture_path()).unwrap();
    let manifest: ChunksManifest = serde_json::from_str(&raw).unwrap();
    for chunk in &manifest.chunks {
        let status = idx
            .tamper_status(&chunk.chunk_id)
            .unwrap_or_else(|e| panic!("tamper_status({}): {e}", chunk.chunk_id));
        assert!(
            status,
            "{}: committed chunk_hash should match recomputation",
            chunk.chunk_id
        );
        idx.tamper_check(&chunk.chunk_id)
            .unwrap_or_else(|e| panic!("tamper_check({}): {e:?}", chunk.chunk_id));
    }
}

#[test]
fn tamper_check_surfaces_recomputed_actual_on_mutation() {
    let raw = std::fs::read_to_string(fixture_path()).unwrap();
    let mut manifest: ChunksManifest = serde_json::from_str(&raw).unwrap();
    let target_id = manifest.chunks[0].chunk_id.clone();
    let expected = manifest.chunks[0].chunk_hash.clone();
    manifest.chunks[0].text.push_str(" __tampered__");
    let idx = ChunksIndex::from_manifest(manifest).expect("rebuild");
    let mismatch = idx
        .tamper_check(&target_id)
        .expect_err("mutated chunk must mismatch");
    assert_eq!(mismatch.chunk_id, target_id);
    assert_eq!(mismatch.expected, expected);
    assert_ne!(mismatch.actual, expected);
    assert_eq!(mismatch.actual.len(), 64, "actual must be 64-hex SHA256");
}

#[test]
fn schema_invalid_manifest_yields_validate_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("manifest.json");
    let bad = serde_json::json!({
        "schema_version": "cacg.v0",
        "chunks": [{
            "schema_version": "cacg.v0",
            "source_id": "src",
            "chunk_id": "src:p001:0000",
            "chunk_hash": "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895",
            "ordinal": 0,
            "start_page": 1,
            "end_page": 1,
            "page_spans": [{"page": 1, "byte_offset_in_chunk": 0}],
            "token_count": 1,
            "text": "x",
            "text_preview": "x"
        }],
        "retracted_source_ids": ["src"],
        "retracted_chunk_ids": []
    });
    std::fs::write(&path, serde_json::to_string(&bad).unwrap()).unwrap();
    let err = ChunksIndex::from_path(&path).expect_err("disjointness violation must error");
    match err {
        ChunksIndexLoadError::Validate(_) => {}
        ChunksIndexLoadError::NonFile(_)
        | ChunksIndexLoadError::Io { .. }
        | ChunksIndexLoadError::Parse(_)
        | ChunksIndexLoadError::DuplicateChunkId(_)
        | ChunksIndexLoadError::HashRecompute { .. }
        | ChunksIndexLoadError::MissingChunk(_) => {
            panic!("expected Validate, got {err:?}")
        }
    }
}
