#![allow(clippy::unwrap_used)]
//! Byte-equal round-trip parity for the manifest / matrix schema types.
//!
//! Each committed Python-built fixture under `tests/parity_corpus/out_python/`
//! is deserialized through the matching Rust struct and reserialized via
//! `canonical_json::canonical_json` -- the output must byte-equal the
//! original file content. Cross-field validators (`validate_structurally`)
//! are exercised on each fixture so a malformed manifest would surface
//! locally.

use std::path::PathBuf;

use cacg_core::canonical_json::canonical_json;
use cacg_core::schema::{ChunksManifest, SourceMatrix, SourcesManifest};
use serde_json::Value;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn read_fixture(rel: &str) -> String {
    let path = workspace_root().join(rel);
    assert!(path.is_file(), "missing fixture: {}", path.display());
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn chunks_manifest_round_trips_byte_equal() {
    let raw = read_fixture("tests/parity_corpus/out_python/chunks_manifest.json");
    let typed: ChunksManifest = serde_json::from_str(&raw).expect("typed deserialize");
    typed
        .validate_structurally()
        .expect("committed fixture must pass structural validation");
    let value: Value = serde_json::to_value(&typed).expect("to_value");
    let written = canonical_json(&value).expect("canonical_json");
    assert_eq!(
        written, raw,
        "chunks_manifest round-trip drift\nleft (rust)  = {written}\nright (file) = {raw}"
    );
}

#[test]
fn source_matrix_round_trips_byte_equal() {
    let raw = read_fixture("tests/parity_corpus/out_python/source_matrix.json");
    let typed: SourceMatrix = serde_json::from_str(&raw).expect("typed deserialize");
    typed
        .validate_structurally()
        .expect("committed fixture must pass structural validation");
    let value: Value = serde_json::to_value(&typed).expect("to_value");
    let written = canonical_json(&value).expect("canonical_json");
    assert_eq!(
        written, raw,
        "source_matrix round-trip drift\nleft (rust)  = {written}\nright (file) = {raw}"
    );
}

#[test]
fn sources_manifest_round_trips_byte_equal() {
    let raw = read_fixture("tests/parity_corpus/out_python/sources_manifest.json");
    let typed: SourcesManifest = serde_json::from_str(&raw).expect("typed deserialize");
    typed
        .validate_structurally()
        .expect("committed fixture must pass structural validation");
    let value: Value = serde_json::to_value(&typed).expect("to_value");
    let written = canonical_json(&value).expect("canonical_json");
    assert_eq!(
        written, raw,
        "sources_manifest round-trip drift\nleft (rust)  = {written}\nright (file) = {raw}"
    );
}

#[test]
fn chunks_manifest_rejects_unknown_field() {
    let raw = read_fixture("tests/parity_corpus/out_python/chunks_manifest.json");
    let mut v: Value = serde_json::from_str(&raw).unwrap();
    v.as_object_mut()
        .unwrap()
        .insert("typo_field".into(), Value::String("oops".into()));
    let serialized = canonical_json(&v).unwrap();
    let parsed: Result<ChunksManifest, _> = serde_json::from_str(&serialized);
    assert!(
        parsed.is_err(),
        "extra field at chunks_manifest root must be rejected (deny_unknown_fields)"
    );
}

#[test]
fn source_matrix_rejects_unknown_field() {
    let raw = read_fixture("tests/parity_corpus/out_python/source_matrix.json");
    let mut v: Value = serde_json::from_str(&raw).unwrap();
    v.as_object_mut()
        .unwrap()
        .insert("typo_field".into(), Value::String("oops".into()));
    let serialized = canonical_json(&v).unwrap();
    let parsed: Result<SourceMatrix, _> = serde_json::from_str(&serialized);
    assert!(
        parsed.is_err(),
        "extra field at source_matrix root must be rejected (deny_unknown_fields)"
    );
}

#[test]
fn chunks_manifest_validators_catch_unsorted_retracted() {
    let bad = serde_json::json!({
        "schema_version": "cacg.v0",
        "chunks": [],
        "retracted_source_ids": ["zzz", "aaa"],
        "retracted_chunk_ids": []
    });
    let typed: ChunksManifest = serde_json::from_value(bad).expect("deserialize");
    let err = typed
        .validate_structurally()
        .expect_err("unsorted retracted_source_ids must fail validation");
    assert_eq!(err.code, "CACG-MAN-001");
}

#[test]
fn chunks_manifest_validators_catch_disjointness_violation() {
    let bad = serde_json::json!({
        "schema_version": "cacg.v0",
        "chunks": [{
            "schema_version": "cacg.v0",
            "source_id": "sample",
            "chunk_id": "sample:p001:0000",
            "chunk_hash": "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895",
            "ordinal": 0,
            "start_page": 1,
            "end_page": 1,
            "page_spans": [{"page": 1, "byte_offset_in_chunk": 0}],
            "token_count": 1,
            "text": "x",
            "text_preview": "x"
        }],
        "retracted_source_ids": ["sample"],
        "retracted_chunk_ids": []
    });
    let typed: ChunksManifest = serde_json::from_value(bad).expect("deserialize");
    let err = typed
        .validate_structurally()
        .expect_err("active source_id present in retracted_source_ids must fail validation");
    assert_eq!(err.code, "CACG-MAN-001");
    assert!(err.message.contains("sample"));
}

#[test]
fn chunk_record_page_span_non_monotonic_rejected() {
    let bad = serde_json::json!({
        "schema_version": "cacg.v0",
        "chunks": [{
            "schema_version": "cacg.v0",
            "source_id": "sample",
            "chunk_id": "sample:p001:0000",
            "chunk_hash": "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895",
            "ordinal": 0,
            "start_page": 1,
            "end_page": 2,
            "page_spans": [
                {"page": 1, "byte_offset_in_chunk": 5},
                {"page": 2, "byte_offset_in_chunk": 3}
            ],
            "token_count": 1,
            "text": "abcdefghij",
            "text_preview": "abc"
        }],
        "retracted_source_ids": [],
        "retracted_chunk_ids": []
    });
    let typed: ChunksManifest = serde_json::from_value(bad).expect("deserialize");
    let err = typed
        .validate_structurally()
        .expect_err("non-monotonic byte_offset_in_chunk must fail");
    assert_eq!(err.code, "CACG-MAN-001");
}

#[test]
fn source_matrix_rejects_empty_source_id() {
    let bad = serde_json::json!({
        "schema_version": "cacg.v0",
        "allowed": {"reading_01": ["", "sample"]}
    });
    let typed: SourceMatrix = serde_json::from_value(bad).expect("deserialize");
    let err = typed
        .validate_structurally()
        .expect_err("empty source_id must fail validation");
    assert_eq!(err.code, "CACG-AUTH-000");
}

#[test]
fn source_matrix_rejects_duplicate_source_id() {
    let bad = serde_json::json!({
        "schema_version": "cacg.v0",
        "allowed": {"reading_01": ["sample", "sample"]}
    });
    let typed: SourceMatrix = serde_json::from_value(bad).expect("deserialize");
    let err = typed
        .validate_structurally()
        .expect_err("duplicate source_id must fail validation");
    assert_eq!(err.code, "CACG-AUTH-000");
}
