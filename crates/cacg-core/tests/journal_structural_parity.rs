#![allow(
    clippy::unwrap_used,
    clippy::doc_markdown,
    clippy::doc_lazy_continuation,
    clippy::needless_borrows_for_generic_args,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::unnecessary_cast,
    clippy::manual_range_contains,
    clippy::iter_cloned_collect,
    clippy::explicit_deref_methods,
    clippy::explicit_auto_deref,
    clippy::too_many_lines
)]
//! AC-C5 Pydantic-equivalent structural validation gate (Codex R19 gap 1).
//!
//! Python `LintEvent` declares `latency_ms: float = Field(ge=0)` and
//! `diagnostics: list[dict[str, Any]]`. A checksum-valid event with
//! `latency_ms: -1.0` or `diagnostics: ["not-a-dict"]` passes Serde
//! but Python rejects via Pydantic validators. The Rust port must
//! mirror that semantic so the byte-equal trust boundary stays
//! consistent.
//!
//! Also exercises the concurrent-append discipline (Codex R19 gap 3):
//! 4 threads × 25 events = 100 events written to the same journal
//! under the flock + per-path cache critical section; resulting file
//! must have a valid chain with seq 0..99 contiguous.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, Barrier};
use std::thread;

use cacg_core::canonical_json::canonical_json;
use cacg_core::hash::compute_event_checksum;
use cacg_core::journal::{
    append_entry, reset_append_cache, validate_jsonl, JournalEntry, JournalError,
};
use serde_json::{json, Value};
use tempfile::TempDir;

const FROZEN_EVENT_ID: &str = "00000000-0000-0000-0000-000000000000";
const FROZEN_TIMESTAMP: &str = "1970-01-01T00:00:00Z";

fn frozen_verification() -> BTreeMap<String, bool> {
    let mut v = BTreeMap::new();
    v.insert("fuzzy".to_string(), false);
    v.insert("layer1".to_string(), true);
    v.insert("layer2".to_string(), false);
    v
}

/// Build a checksum-valid LintEvent line with the given `latency_ms` +
/// `diagnostics`. We construct the canonical payload by hand so the
/// `event_checksum` is correct -- the test then verifies that
/// `validate_jsonl` REJECTS it for structural reasons that Serde
/// can't catch but Python's Pydantic does.
fn checksum_valid_line(
    seq: u64,
    card_path: &str,
    latency_ms: f64,
    diagnostics: Vec<Value>,
    prev_checksum: Option<&str>,
) -> String {
    let mut payload = serde_json::Map::new();
    payload.insert("card_hash_after".into(), Value::Null);
    payload.insert("card_hash_before".into(), Value::Null);
    payload.insert("card_path".into(), Value::String(card_path.to_string()));
    payload.insert("command".into(), Value::String("lint".to_string()));
    payload.insert("diagnostics".into(), Value::Array(diagnostics));
    payload.insert(
        "event_id".into(),
        Value::String(FROZEN_EVENT_ID.to_string()),
    );
    payload.insert(
        "latency_ms".into(),
        Value::Number(serde_json::Number::from_f64(latency_ms).unwrap()),
    );
    payload.insert(
        "prev_checksum".into(),
        prev_checksum.map_or(Value::Null, |s| Value::String(s.to_string())),
    );
    payload.insert(
        "schema_version".into(),
        Value::String("cacg.v0".to_string()),
    );
    payload.insert("seq".into(), Value::Number(seq.into()));
    payload.insert(
        "timestamp".into(),
        Value::String(FROZEN_TIMESTAMP.to_string()),
    );
    payload.insert(
        "verification".into(),
        json!({"fuzzy": false, "layer1": true, "layer2": false}),
    );
    let checksum = compute_event_checksum(&payload).expect("checksum");
    payload.insert("event_checksum".into(), Value::String(checksum));
    canonical_json(&Value::Object(payload)).expect("canonical")
}

#[test]
fn validate_rejects_checksum_valid_negative_latency_ms() {
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("lint_journal.jsonl");
    let line = checksum_valid_line(0, "x.md", -1.0, vec![], None);
    std::fs::write(&path, format!("{line}\n")).unwrap();
    let bad = validate_jsonl(&path).expect("validate");
    assert_eq!(
        bad,
        vec![1],
        "checksum-valid line with latency_ms=-1.0 MUST fail Pydantic-equivalent validation"
    );
}

#[test]
fn validate_rejects_checksum_valid_non_object_diagnostics() {
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("lint_journal.jsonl");
    // diagnostics contains a non-object element (string "not-a-dict").
    let line = checksum_valid_line(
        0,
        "x.md",
        0.0,
        vec![Value::String("not-a-dict".to_string())],
        None,
    );
    std::fs::write(&path, format!("{line}\n")).unwrap();
    let bad = validate_jsonl(&path).expect("validate");
    assert_eq!(
        bad,
        vec![1],
        "checksum-valid line with non-object diagnostics MUST fail Pydantic-equivalent validation"
    );
}

#[test]
fn append_refuses_negative_latency_ms_entry() {
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("lint_journal.jsonl");
    reset_append_cache();
    let entry = JournalEntry {
        command: "lint".to_string(),
        card_path: "x.md".to_string(),
        card_hash_before: None,
        card_hash_after: None,
        diagnostics: Vec::new(),
        verification: frozen_verification(),
        latency_ms: -1.0,
    };
    let r = append_entry(&path, &entry, FROZEN_EVENT_ID, FROZEN_TIMESTAMP);
    assert!(
        matches!(r, Err(JournalError::InvalidEntry(_))),
        "append_entry MUST refuse latency_ms < 0.0: got {r:?}"
    );
    // File must not exist (refusal happens BEFORE any I/O).
    assert!(
        !path.exists(),
        "append must not create the journal on refusal"
    );
}

#[test]
fn append_refuses_non_object_diagnostics_entry() {
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("lint_journal.jsonl");
    reset_append_cache();
    let entry = JournalEntry {
        command: "lint".to_string(),
        card_path: "x.md".to_string(),
        card_hash_before: None,
        card_hash_after: None,
        diagnostics: vec![Value::String("not-a-dict".to_string())],
        verification: frozen_verification(),
        latency_ms: 0.0,
    };
    let r = append_entry(&path, &entry, FROZEN_EVENT_ID, FROZEN_TIMESTAMP);
    assert!(
        matches!(r, Err(JournalError::InvalidEntry(_))),
        "append_entry MUST refuse non-object diagnostics elements: got {r:?}"
    );
    assert!(!path.exists());
}

#[test]
fn multi_thread_concurrent_appenders_produce_valid_chain() {
    // Codex R19 gap 3: AC-T11 concurrency. 4 threads each append 25
    // events to the same journal under the flock + per-path cache
    // critical section. After all threads join: file has 100 lines,
    // validate_jsonl returns [], seq values are 0..99 contiguous.
    const THREADS: usize = 4;
    const EVENTS_PER_THREAD: usize = 25;
    let dir = TempDir::new().expect("tempdir");
    let path: Arc<PathBuf> = Arc::new(dir.path().join("lint_journal.jsonl"));
    reset_append_cache();
    let barrier = Arc::new(Barrier::new(THREADS));
    let mut handles = Vec::with_capacity(THREADS);
    for thread_id in 0..THREADS {
        let path = Arc::clone(&path);
        let barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier.wait();
            for i in 0..EVENTS_PER_THREAD {
                let entry = JournalEntry {
                    command: "lint".to_string(),
                    card_path: format!("thread-{thread_id}-event-{i}.md"),
                    card_hash_before: None,
                    card_hash_after: None,
                    diagnostics: Vec::new(),
                    verification: frozen_verification(),
                    latency_ms: 0.0,
                };
                append_entry(&path, &entry, FROZEN_EVENT_ID, FROZEN_TIMESTAMP)
                    .expect("append_entry must succeed under concurrent contention");
            }
        }));
    }
    for h in handles {
        h.join().expect("thread panicked");
    }

    let text = std::fs::read_to_string(&*path).expect("read journal");
    let line_count = text.lines().filter(|l| !l.trim().is_empty()).count();
    assert_eq!(
        line_count,
        THREADS * EVENTS_PER_THREAD,
        "expected {} events; got {line_count}",
        THREADS * EVENTS_PER_THREAD
    );

    let bad = validate_jsonl(&*path).expect("validate_jsonl");
    assert!(
        bad.is_empty(),
        "concurrent-append journal MUST validate cleanly; got bad_lines = {bad:?}"
    );

    // Confirm seq values are 0..99 contiguous.
    let mut seqs: Vec<u64> = Vec::with_capacity(THREADS * EVENTS_PER_THREAD);
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let event: Value = serde_json::from_str(line).expect("event parses");
        seqs.push(event["seq"].as_u64().expect("seq is u64"));
    }
    seqs.sort_unstable();
    let expected: Vec<u64> = (0..(THREADS * EVENTS_PER_THREAD) as u64).collect();
    assert_eq!(
        seqs,
        expected,
        "seq values MUST be 0..{} contiguous (no gaps / duplicates) under flock serialization",
        THREADS * EVENTS_PER_THREAD - 1
    );
}
