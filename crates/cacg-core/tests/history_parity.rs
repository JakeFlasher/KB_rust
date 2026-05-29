#![allow(clippy::unwrap_used)]
//! Per-card `.history.jsonl` parity gate.
//!
//! Verifies that `cacg_core::history::append_history_event` produces
//! byte-identical output to the committed Python-produced fixtures at
//! `tests/parity_corpus/proptest_oracles/history_parity/`.
//!
//! The committed fixtures were captured once from the Python oracle
//! (`legacy_python_oracle/scripts/history_oracle.py` under
//! `KB_FROZEN_CLOCK=1`) and frozen. Any Rust regression in append
//! semantics, checksum chaining, field ordering, retraction tombstone
//! shape, or canonical JSON will diverge from the committed bytes.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use cacg_core::history::{
    append_history_event, history_path_for, reset_history_cache, validate_history, HistoryEntry,
    HistoryError, RETRACTION_FRONTMATTER_MARKER,
};
use serde_json::{Map, Value};
use tempfile::TempDir;

const FROZEN_TIMESTAMP: &str = "1970-01-01T00:00:00Z";
const N_EVENTS: usize = 50;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn history_entry_from_json(v: &Value) -> HistoryEntry {
    let delta: BTreeMap<String, Vec<String>> = v["cited_chunk_set_delta"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, val)| {
            let arr: Vec<String> = val
                .as_array()
                .unwrap()
                .iter()
                .map(|s| s.as_str().unwrap().to_string())
                .collect();
            (k.clone(), arr)
        })
        .collect();
    let frontmatter_field_changes: Vec<String> = v["frontmatter_field_changes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect();
    let cited_chunk_ids_snapshot: Vec<String> = v["cited_chunk_ids_snapshot"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect();
    let frontmatter_snapshot: Map<String, Value> =
        v["frontmatter_snapshot"].as_object().unwrap().clone();
    HistoryEntry {
        prev_card_hash: v["prev_card_hash"]
            .as_str()
            .map(std::string::ToString::to_string),
        new_card_hash: v["new_card_hash"].as_str().unwrap().to_string(),
        cited_chunk_set_delta: delta,
        frontmatter_field_changes,
        cited_chunk_ids_snapshot,
        frontmatter_snapshot,
        is_retracted: v["is_retracted"].as_bool().unwrap(),
    }
}

fn load_committed_history_entries() -> Vec<HistoryEntry> {
    let entries_path =
        workspace_root().join("tests/parity_corpus/proptest_oracles/history_parity/entries.jsonl");
    assert!(
        entries_path.is_file(),
        "committed entries.jsonl missing: {}",
        entries_path.display()
    );
    let text = std::fs::read_to_string(&entries_path).expect("read entries.jsonl");
    text.lines()
        .map(|line| {
            let v: Value = serde_json::from_str(line).expect("parse entry JSON");
            history_entry_from_json(&v)
        })
        .collect()
}

#[test]
fn rust_matches_committed_python_fixture_50_events() {
    let fixture_path = workspace_root()
        .join("tests/parity_corpus/proptest_oracles/history_parity/expected_card.history.jsonl");
    assert!(
        fixture_path.is_file(),
        "committed fixture missing: {}",
        fixture_path.display()
    );

    let entries = load_committed_history_entries();
    assert_eq!(
        entries.len(),
        N_EVENTS,
        "committed corpus must have {N_EVENTS} entries"
    );

    let rust_dir = TempDir::new().expect("tempdir");
    let rust_path = rust_dir.path().join("card.history.jsonl");
    reset_history_cache();
    for (i, entry) in entries.iter().enumerate() {
        let seq = append_history_event(&rust_path, entry, FROZEN_TIMESTAMP)
            .unwrap_or_else(|e| panic!("append_history_event failed at i={i}: {e:?}"));
        assert_eq!(seq, i as u64);
    }
    let bad = validate_history(&rust_path).expect("validate_history");
    assert!(
        bad.is_empty(),
        "Rust-built 50-event history must validate clean; bad={bad:?}"
    );

    let rust_bytes = std::fs::read(&rust_path).expect("read rust history");
    let fixture_bytes = std::fs::read(&fixture_path).expect("read committed fixture");
    assert_eq!(
        rust_bytes.len(),
        fixture_bytes.len(),
        "byte-length mismatch: rust={}, fixture={}",
        rust_bytes.len(),
        fixture_bytes.len()
    );
    if rust_bytes != fixture_bytes {
        let rust_text = String::from_utf8_lossy(&rust_bytes);
        let fix_text = String::from_utf8_lossy(&fixture_bytes);
        let rust_lines: Vec<&str> = rust_text.lines().collect();
        let fix_lines: Vec<&str> = fix_text.lines().collect();
        for (i, (r, f)) in rust_lines.iter().zip(fix_lines.iter()).enumerate() {
            assert!(
                r == f,
                "first divergence at line {} (1-indexed)\n  rust:    {r}\n  fixture: {f}",
                i + 1
            );
        }
        panic!("byte streams differ but no line-level divergence found");
    }
}

#[test]
fn retraction_tombstone_matches_committed_fixture() {
    let fixture_path = workspace_root().join(
        "tests/parity_corpus/proptest_oracles/history_parity/expected_retraction_tombstone.history.jsonl",
    );
    assert!(
        fixture_path.is_file(),
        "committed tombstone fixture missing: {}",
        fixture_path.display()
    );

    let card_hash = "a".repeat(64);
    let mut delta = BTreeMap::new();
    delta.insert("added".to_string(), Vec::new());
    delta.insert("removed".to_string(), Vec::new());
    let entry = HistoryEntry {
        prev_card_hash: Some(card_hash.clone()),
        new_card_hash: card_hash,
        cited_chunk_set_delta: delta,
        frontmatter_field_changes: vec![RETRACTION_FRONTMATTER_MARKER.to_string()],
        cited_chunk_ids_snapshot: Vec::new(),
        frontmatter_snapshot: Map::new(),
        is_retracted: true,
    };

    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("card.history.jsonl");
    reset_history_cache();
    let seq = append_history_event(&path, &entry, FROZEN_TIMESTAMP).expect("append");
    assert_eq!(seq, 0);

    let rust_bytes = std::fs::read(&path).unwrap();
    let fixture_bytes = std::fs::read(&fixture_path).unwrap();
    assert_eq!(
        rust_bytes,
        fixture_bytes,
        "tombstone byte mismatch\n  rust:    {}\n  fixture: {}",
        String::from_utf8_lossy(&rust_bytes),
        String::from_utf8_lossy(&fixture_bytes)
    );
    let text = String::from_utf8_lossy(&rust_bytes);
    assert!(
        text.contains(RETRACTION_FRONTMATTER_MARKER),
        "tombstone bytes must contain __cacg_retracted__ marker; got: {text}"
    );
    assert!(
        text.contains("\"is_retracted\":true"),
        "tombstone bytes must contain is_retracted:true; got: {text}"
    );
}

#[test]
fn chain_prev_card_hash_and_new_card_hash_thread_through() {
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("card.history.jsonl");
    reset_history_cache();
    let h1 = "1".repeat(64);
    let h2 = "2".repeat(64);
    let mut delta = BTreeMap::new();
    delta.insert("added".to_string(), Vec::new());
    delta.insert("removed".to_string(), Vec::new());

    let e1 = HistoryEntry {
        prev_card_hash: None,
        new_card_hash: h1.clone(),
        cited_chunk_set_delta: delta.clone(),
        frontmatter_field_changes: Vec::new(),
        cited_chunk_ids_snapshot: Vec::new(),
        frontmatter_snapshot: Map::new(),
        is_retracted: false,
    };
    let e2 = HistoryEntry {
        prev_card_hash: Some(h1.clone()),
        new_card_hash: h2.clone(),
        cited_chunk_set_delta: delta,
        frontmatter_field_changes: Vec::new(),
        cited_chunk_ids_snapshot: Vec::new(),
        frontmatter_snapshot: Map::new(),
        is_retracted: false,
    };
    let s1 = append_history_event(&path, &e1, FROZEN_TIMESTAMP).expect("append 1");
    let s2 = append_history_event(&path, &e2, FROZEN_TIMESTAMP).expect("append 2");
    assert_eq!(s1, 0);
    assert_eq!(s2, 1);
    let bad = validate_history(&path).expect("validate");
    assert!(
        bad.is_empty(),
        "2-event chain must validate clean; bad={bad:?}"
    );

    let text = std::fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 2);
    let l1: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(l1["prev_card_hash"], Value::Null);
    assert_eq!(l1["new_card_hash"], Value::String(h1.clone()));
    assert_eq!(l1["seq"], Value::Number(0.into()));
    assert_eq!(l1["prev_checksum"], Value::Null);
    let l2: Value = serde_json::from_str(lines[1]).unwrap();
    assert_eq!(l2["prev_card_hash"], Value::String(h1));
    assert_eq!(l2["new_card_hash"], Value::String(h2));
    assert_eq!(l2["seq"], Value::Number(1.into()));
    assert_eq!(l2["prev_checksum"], l1["event_checksum"]);
}

#[test]
fn history_path_for_card_md_yields_history_jsonl_sibling() {
    let p = Path::new("tests/parity_corpus/cards/reading_01/tamper_recovery-card-a.md");
    let h = history_path_for(p);
    assert_eq!(
        h,
        PathBuf::from("tests/parity_corpus/cards/reading_01/tamper_recovery-card-a.history.jsonl")
    );
}

#[test]
fn append_refuses_when_existing_has_invalid_line() {
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("card.history.jsonl");
    reset_history_cache();
    std::fs::write(&path, b"this is not valid json\n").unwrap();
    let entry = HistoryEntry {
        prev_card_hash: None,
        new_card_hash: "x".repeat(64),
        cited_chunk_set_delta: BTreeMap::new(),
        frontmatter_field_changes: Vec::new(),
        cited_chunk_ids_snapshot: Vec::new(),
        frontmatter_snapshot: Map::new(),
        is_retracted: false,
    };
    let r = append_history_event(&path, &entry, FROZEN_TIMESTAMP);
    match r {
        Err(HistoryError::InvalidExisting { bad_lines, .. }) => {
            assert_eq!(bad_lines, vec![1]);
        }
        other => panic!("expected InvalidExisting, got {other:?}"),
    }
}

#[test]
fn validate_rejects_chain_tamper() {
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("card.history.jsonl");
    reset_history_cache();
    let mut delta = BTreeMap::new();
    delta.insert("added".to_string(), Vec::new());
    delta.insert("removed".to_string(), Vec::new());
    let e1 = HistoryEntry {
        prev_card_hash: None,
        new_card_hash: "1".repeat(64),
        cited_chunk_set_delta: delta.clone(),
        frontmatter_field_changes: Vec::new(),
        cited_chunk_ids_snapshot: Vec::new(),
        frontmatter_snapshot: Map::new(),
        is_retracted: false,
    };
    let e2 = HistoryEntry {
        prev_card_hash: Some("1".repeat(64)),
        new_card_hash: "2".repeat(64),
        cited_chunk_set_delta: delta,
        frontmatter_field_changes: Vec::new(),
        cited_chunk_ids_snapshot: Vec::new(),
        frontmatter_snapshot: Map::new(),
        is_retracted: false,
    };
    append_history_event(&path, &e1, FROZEN_TIMESTAMP).expect("append 1");
    reset_history_cache();
    append_history_event(&path, &e2, FROZEN_TIMESTAMP).expect("append 2");

    let mut text = std::fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 2);
    let tampered = text.replacen(r#""event_checksum":"#, r#""event_checksum":"0000"#, 1);
    assert!(tampered != text, "tamper failed -- text unchanged");
    text = tampered;
    std::fs::write(&path, &text).unwrap();
    reset_history_cache();
    let bad = validate_history(&path).expect("validate_history after tamper");
    assert!(
        !bad.is_empty(),
        "tampered chain must surface at least one bad line"
    );
}

#[test]
fn empty_new_card_hash_matches_committed_fixture() {
    let fixture_path = workspace_root().join(
        "tests/parity_corpus/proptest_oracles/history_parity/expected_empty_new_card_hash.history.jsonl",
    );
    assert!(fixture_path.is_file(), "committed fixture missing");

    let mut delta = BTreeMap::new();
    delta.insert("added".to_string(), Vec::new());
    delta.insert("removed".to_string(), Vec::new());
    let entry = HistoryEntry {
        prev_card_hash: Some("a".repeat(64)),
        new_card_hash: String::new(),
        cited_chunk_set_delta: delta,
        frontmatter_field_changes: Vec::new(),
        cited_chunk_ids_snapshot: Vec::new(),
        frontmatter_snapshot: Map::new(),
        is_retracted: false,
    };

    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("card.history.jsonl");
    reset_history_cache();
    let seq = append_history_event(&path, &entry, FROZEN_TIMESTAMP).expect("append");
    assert_eq!(seq, 0);

    let rust_bytes = std::fs::read(&path).unwrap();
    let fixture_bytes = std::fs::read(&fixture_path).unwrap();
    assert_eq!(
        rust_bytes,
        fixture_bytes,
        "empty new_card_hash mismatch\n  rust:    {}\n  fixture: {}",
        String::from_utf8_lossy(&rust_bytes),
        String::from_utf8_lossy(&fixture_bytes)
    );
}

#[test]
fn omitted_is_retracted_marker_matches_committed_fixture() {
    let fixture_path = workspace_root().join(
        "tests/parity_corpus/proptest_oracles/history_parity/expected_omitted_is_retracted_marker.history.jsonl",
    );
    assert!(fixture_path.is_file(), "committed fixture missing");

    let mut delta = BTreeMap::new();
    delta.insert("added".to_string(), Vec::new());
    delta.insert("removed".to_string(), Vec::new());
    let entry = HistoryEntry {
        prev_card_hash: Some("b".repeat(64)),
        new_card_hash: "c".repeat(64),
        cited_chunk_set_delta: delta,
        frontmatter_field_changes: vec![RETRACTION_FRONTMATTER_MARKER.to_string()],
        cited_chunk_ids_snapshot: Vec::new(),
        frontmatter_snapshot: Map::new(),
        is_retracted: false,
    };

    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("card.history.jsonl");
    reset_history_cache();
    let seq = append_history_event(&path, &entry, FROZEN_TIMESTAMP).expect("append");
    assert_eq!(seq, 0);

    let rust_bytes = std::fs::read(&path).unwrap();
    let fixture_bytes = std::fs::read(&fixture_path).unwrap();
    assert_eq!(
        rust_bytes,
        fixture_bytes,
        "omitted is_retracted marker mismatch\n  rust:    {}\n  fixture: {}",
        String::from_utf8_lossy(&rust_bytes),
        String::from_utf8_lossy(&fixture_bytes)
    );
    let text = String::from_utf8_lossy(&rust_bytes);
    assert!(
        text.contains(RETRACTION_FRONTMATTER_MARKER),
        "marker must appear in output even though is_retracted=false"
    );
    assert!(
        text.contains("\"is_retracted\":false"),
        "is_retracted must be false"
    );
}

#[test]
fn committed_fixture_integrity() {
    let base = workspace_root().join("tests/parity_corpus/proptest_oracles/history_parity");
    let entries_path = base.join("entries.jsonl");
    let expected_path = base.join("expected_card.history.jsonl");
    let tombstone_path = base.join("expected_retraction_tombstone.history.jsonl");
    let manifest_path = base.join("manifest.json");

    assert!(entries_path.is_file(), "entries.jsonl missing");
    assert!(
        expected_path.is_file(),
        "expected_card.history.jsonl missing"
    );
    assert!(
        tombstone_path.is_file(),
        "expected_retraction_tombstone.history.jsonl missing"
    );
    assert!(manifest_path.is_file(), "manifest.json missing");

    let empty_hash_path = base.join("expected_empty_new_card_hash.history.jsonl");
    let omitted_marker_path = base.join("expected_omitted_is_retracted_marker.history.jsonl");

    assert!(
        empty_hash_path.is_file(),
        "empty_new_card_hash fixture missing"
    );
    assert!(
        omitted_marker_path.is_file(),
        "omitted_is_retracted_marker fixture missing"
    );

    let entries_bytes = std::fs::read(&entries_path).expect("read entries");
    let expected_bytes = std::fs::read(&expected_path).expect("read expected");
    let tombstone_bytes = std::fs::read(&tombstone_path).expect("read tombstone");
    let empty_hash_bytes = std::fs::read(&empty_hash_path).expect("read empty_hash");
    let omitted_marker_bytes = std::fs::read(&omitted_marker_path).expect("read omitted_marker");

    assert_eq!(
        format!("{:x}", Sha256::digest(&entries_bytes)),
        "fff2b0fd6862721a42eb1092baee4eb1f81ae86415538046e388e3f59a961935",
        "entries.jsonl SHA-256 drift"
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(&expected_bytes)),
        "06f3fe60f8ef0625929806b83d5023cec761a009fd0f9de9bb4b6581774fa66d",
        "expected_card.history.jsonl SHA-256 drift"
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(&tombstone_bytes)),
        "9d455c58fe598096048db12fb9c26b2b7e6be2d617cbbc8c18e36173281a1515",
        "expected_retraction_tombstone.history.jsonl SHA-256 drift"
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(&empty_hash_bytes)),
        "4fa09630c856b0797814d1267f125f18d2dc7cf28b05d31d8c5b3512df07f249",
        "empty_new_card_hash fixture SHA-256 drift"
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(&omitted_marker_bytes)),
        "5415e0799fafe080d8ea1d08430d536c7187dfdab39a9e319750a26841c6cf08",
        "omitted_is_retracted_marker fixture SHA-256 drift"
    );

    #[allow(clippy::naive_bytecount)]
    let entries_lines = entries_bytes.iter().filter(|&&b| b == b'\n').count();
    #[allow(clippy::naive_bytecount)]
    let expected_lines = expected_bytes.iter().filter(|&&b| b == b'\n').count();
    #[allow(clippy::naive_bytecount)]
    let tombstone_lines = tombstone_bytes.iter().filter(|&&b| b == b'\n').count();
    assert_eq!(entries_lines, N_EVENTS, "entries.jsonl line count");
    assert_eq!(
        expected_lines, N_EVENTS,
        "expected_card.history.jsonl line count"
    );
    assert_eq!(tombstone_lines, 1, "tombstone fixture line count");
}
