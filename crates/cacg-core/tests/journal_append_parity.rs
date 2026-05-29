#![allow(clippy::unwrap_used)]
//! AC-C5 append-side parity gate: build a journal via
//! `cacg_core::journal::append_entry` from scratch under frozen-clock
//! values and assert the resulting bytes match a hand-computed
//! reference. Also exercise the pre-append validate refusal.

use std::collections::BTreeMap;
use std::path::PathBuf;

use cacg_core::journal::{append_entry, validate_jsonl, JournalEntry, JournalError};
use tempfile::TempDir;

const FROZEN_EVENT_ID: &str = "00000000-0000-0000-0000-000000000000";
const FROZEN_TIMESTAMP: &str = "1970-01-01T00:00:00Z";

fn frozen_entry(card_path: &str) -> JournalEntry {
    let mut verification = BTreeMap::new();
    verification.insert("fuzzy".to_string(), false);
    verification.insert("layer1".to_string(), true);
    verification.insert("layer2".to_string(), false);
    JournalEntry {
        command: "lint".to_string(),
        card_path: card_path.to_string(),
        card_hash_before: None,
        card_hash_after: None,
        diagnostics: Vec::new(),
        verification,
        latency_ms: 0.0,
    }
}

#[test]
fn first_event_byte_equal_to_tamper_recovery_corpus_line_1() {
    // The committed tests/parity_corpus/tamper_recovery/scenario-01/
    // input/lint_journal.jsonl line 1 is a frozen-clock LintEvent for
    // `tests/parity_corpus/cards/reading_01/tamper_recovery-card-a.md`.
    // Building the same event from scratch via append_entry must
    // produce byte-identical canonical bytes.
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("lint_journal.jsonl");
    let entry = frozen_entry("tests/parity_corpus/cards/reading_01/tamper_recovery-card-a.md");
    let seq = append_entry(&path, &entry, FROZEN_EVENT_ID, FROZEN_TIMESTAMP).expect("append_entry");
    assert_eq!(seq, 0);
    let actual = std::fs::read_to_string(&path).unwrap();

    // Reference line copied from
    // tests/parity_corpus/tamper_recovery/scenario-01/input/lint_journal.jsonl line 1.
    let expected = concat!(
        "{\"card_hash_after\":null,",
        "\"card_hash_before\":null,",
        "\"card_path\":\"tests/parity_corpus/cards/reading_01/tamper_recovery-card-a.md\",",
        "\"command\":\"lint\",",
        "\"diagnostics\":[],",
        "\"event_checksum\":\"a550837f748206624e901caaa3ee8a0fb6969c2cd23c5baa067ea5a380843a6c\",",
        "\"event_id\":\"00000000-0000-0000-0000-000000000000\",",
        "\"latency_ms\":0.0,",
        "\"prev_checksum\":null,",
        "\"schema_version\":\"cacg.v0\",",
        "\"seq\":0,",
        "\"timestamp\":\"1970-01-01T00:00:00Z\",",
        "\"verification\":{\"fuzzy\":false,\"layer1\":true,\"layer2\":false}}\n"
    );
    assert_eq!(
        actual, expected,
        "first event must byte-equal the Python-built tamper_recovery line 1"
    );
}

#[test]
fn second_event_chains_prev_checksum_correctly() {
    // Build a 2-event journal: append-entry-1 then append-entry-2.
    // The second line's `prev_checksum` MUST equal the first line's
    // `event_checksum`. validate_jsonl on the result returns [].
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("lint_journal.jsonl");
    let entry1 = frozen_entry("tests/parity_corpus/cards/reading_01/tamper_recovery-card-a.md");
    let entry2 = frozen_entry("tests/parity_corpus/cards/reading_01/tamper_recovery-card-b.md");
    let seq1 = append_entry(&path, &entry1, FROZEN_EVENT_ID, FROZEN_TIMESTAMP).expect("append 1");
    let seq2 = append_entry(&path, &entry2, FROZEN_EVENT_ID, FROZEN_TIMESTAMP).expect("append 2");
    assert_eq!(seq1, 0);
    assert_eq!(seq2, 1);
    let bad = validate_jsonl(&path).expect("validate");
    assert!(
        bad.is_empty(),
        "2-event chain must validate clean; got bad_lines = {bad:?}"
    );
    // Confirm 2 lines in file.
    let text = std::fs::read_to_string(&path).unwrap();
    assert_eq!(text.lines().count(), 2);
}

#[test]
fn append_refuses_when_existing_has_invalid_line() {
    // Pre-populate a file with line 1 valid + line 2 tampered
    // (event_checksum changed). append_entry must refuse via
    // JournalError::InvalidExisting.
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("lint_journal.jsonl");
    // Reuse the committed tamper_recovery corpus directly: copy it
    // into the tempdir so the test owns the working copy.
    let workspace = {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.pop();
        p.pop();
        p
    };
    let src =
        workspace.join("tests/parity_corpus/tamper_recovery/scenario-01/input/lint_journal.jsonl");
    std::fs::copy(&src, &path).expect("copy tamper_recovery corpus");

    let entry = frozen_entry("tests/parity_corpus/cards/reading_01/tamper_recovery-card-c.md");
    let r = append_entry(&path, &entry, FROZEN_EVENT_ID, FROZEN_TIMESTAMP);
    match r {
        Err(JournalError::InvalidExisting { bad_lines, .. }) => {
            assert_eq!(
                bad_lines,
                vec![2],
                "expected bad_lines=[2], got {bad_lines:?}"
            );
        }
        other => panic!("expected InvalidExisting error, got {other:?}"),
    }
}
