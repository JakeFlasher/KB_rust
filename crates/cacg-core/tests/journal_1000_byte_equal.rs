#![allow(clippy::unwrap_used)]
//! 1000-event byte-equal gate: generates 1000 deterministic
//! `JournalEntry` payloads, appends them via Rust
//! `cacg_core::journal::append_entry`, and byte-compares the resulting
//! file against the committed Python-produced fixture at
//! `tests/parity_corpus/proptest_oracles/journal_1000_byte_equal/expected_lint_journal.jsonl`.
//!
//! The committed fixture was captured once from the Python oracle
//! (`legacy_python_oracle/scripts/journal_oracle.py` under
//! `KB_FROZEN_CLOCK=1`) and frozen. Any Rust regression in append
//! semantics, checksum chaining, field ordering, or canonical JSON
//! will diverge from the committed bytes.

use std::collections::BTreeMap;
use std::path::PathBuf;

use cacg_core::journal::{append_entry, reset_append_cache, validate_jsonl, JournalEntry};
use serde_json::Value;
use sha2::{Digest, Sha256};
use tempfile::TempDir;

const FROZEN_EVENT_ID: &str = "00000000-0000-0000-0000-000000000000";
const FROZEN_TIMESTAMP: &str = "1970-01-01T00:00:00Z";
const N_EVENTS: usize = 1000;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn journal_entry_from_json(v: &Value) -> JournalEntry {
    let verification: BTreeMap<String, bool> = v["verification"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, val)| (k.clone(), val.as_bool().unwrap()))
        .collect();
    JournalEntry {
        command: v["command"].as_str().unwrap().to_string(),
        card_path: v["card_path"].as_str().unwrap().to_string(),
        card_hash_before: v["card_hash_before"]
            .as_str()
            .map(std::string::ToString::to_string),
        card_hash_after: v["card_hash_after"]
            .as_str()
            .map(std::string::ToString::to_string),
        diagnostics: Vec::new(),
        verification,
        latency_ms: v["latency_ms"].as_f64().unwrap(),
    }
}

fn load_committed_entries() -> Vec<JournalEntry> {
    let entries_path = workspace_root()
        .join("tests/parity_corpus/proptest_oracles/journal_1000_byte_equal/entries.jsonl");
    assert!(
        entries_path.is_file(),
        "committed entries.jsonl missing: {}",
        entries_path.display()
    );
    let text = std::fs::read_to_string(&entries_path).expect("read entries.jsonl");
    text.lines()
        .map(|line| {
            let v: Value = serde_json::from_str(line).expect("parse entry JSON");
            journal_entry_from_json(&v)
        })
        .collect()
}

#[test]
fn rust_matches_committed_python_fixture_byte_for_byte() {
    let fixture_path = workspace_root().join(
        "tests/parity_corpus/proptest_oracles/journal_1000_byte_equal/expected_lint_journal.jsonl",
    );
    assert!(
        fixture_path.is_file(),
        "committed fixture missing: {}",
        fixture_path.display()
    );

    let entries = load_committed_entries();
    assert_eq!(
        entries.len(),
        N_EVENTS,
        "committed corpus must have {N_EVENTS} entries"
    );

    let rust_dir = TempDir::new().expect("tempdir");
    let rust_path = rust_dir.path().join("lint_journal.jsonl");
    reset_append_cache();
    for (i, entry) in entries.iter().enumerate() {
        let seq = append_entry(&rust_path, entry, FROZEN_EVENT_ID, FROZEN_TIMESTAMP)
            .unwrap_or_else(|e| panic!("append_entry failed at i={i}: {e:?}"));
        assert_eq!(seq, i as u64, "expected seq={i}, got {seq}");
    }

    let bad = validate_jsonl(&rust_path).expect("validate_jsonl");
    assert!(
        bad.is_empty(),
        "Rust-built 1000-event journal must validate clean; bad={bad:?}"
    );

    let rust_bytes = std::fs::read(&rust_path).expect("read rust journal");
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
        panic!("byte streams differ but no line-level divergence found (trailing newline?)");
    }
}

#[test]
fn committed_fixture_integrity() {
    let base =
        workspace_root().join("tests/parity_corpus/proptest_oracles/journal_1000_byte_equal");
    let entries_path = base.join("entries.jsonl");
    let expected_path = base.join("expected_lint_journal.jsonl");
    let manifest_path = base.join("manifest.json");

    assert!(entries_path.is_file(), "entries.jsonl missing");
    assert!(
        expected_path.is_file(),
        "expected_lint_journal.jsonl missing"
    );
    assert!(manifest_path.is_file(), "manifest.json missing");

    let entries_bytes = std::fs::read(&entries_path).expect("read entries");
    let expected_bytes = std::fs::read(&expected_path).expect("read expected");

    let entries_sha = format!("{:x}", Sha256::digest(&entries_bytes));
    let expected_sha = format!("{:x}", Sha256::digest(&expected_bytes));

    assert_eq!(
        entries_sha, "02a7e575ee495044988ba46c4dc937e858929562be96c430d1421951ef0e5b30",
        "entries.jsonl SHA-256 drift"
    );
    assert_eq!(
        expected_sha, "d435abf2c87813ea3cbfe8afe9401f2764e4be4da8e80189d421b48f439869bf",
        "expected_lint_journal.jsonl SHA-256 drift"
    );

    #[allow(clippy::naive_bytecount)]
    let entries_lines = entries_bytes.iter().filter(|&&b| b == b'\n').count();
    #[allow(clippy::naive_bytecount)]
    let expected_lines = expected_bytes.iter().filter(|&&b| b == b'\n').count();
    assert_eq!(entries_lines, N_EVENTS, "entries.jsonl line count");
    assert_eq!(
        expected_lines, N_EVENTS,
        "expected_lint_journal.jsonl line count"
    );
}
