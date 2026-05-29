#![allow(clippy::unwrap_used)]
//! AC-C5 parity gate: `cacg_core::journal::validate_jsonl` matches
//! Python `cacg.journal.validate_jsonl` byte-for-byte against the 3
//! committed M0 corpus scenarios.
//!
//! Each scenario commits a `lint_journal.jsonl` with a single
//! tampered/truncated/duplicate-seq line on line 2 and an
//! `expected.json` declaring `bad_lines == [2]`. The Rust port must
//! return the same `vec![2]`.
//!
//! Plan reference: AC-C5 at `plans/cacg-rust-port-trust-kernel-first-plan.md`.

use std::path::PathBuf;

use cacg_core::journal::{validate_jsonl, JournalError};

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn scenario_path(scenario: &str) -> PathBuf {
    workspace_root().join(format!(
        "tests/parity_corpus/{scenario}/scenario-01/input/lint_journal.jsonl"
    ))
}

#[test]
fn tamper_recovery_scenario_reports_line_2_bad() {
    let path = scenario_path("tamper_recovery");
    assert!(path.is_file(), "missing fixture: {}", path.display());
    let bad = validate_jsonl(&path).expect("validate_jsonl");
    assert_eq!(
        bad,
        vec![2],
        "tamper_recovery scenario: expected bad_lines == [2], got {bad:?}"
    );
}

#[test]
fn truncated_journal_scenario_reports_line_2_bad() {
    let path = scenario_path("truncated_journal");
    assert!(path.is_file(), "missing fixture: {}", path.display());
    let bad = validate_jsonl(&path).expect("validate_jsonl");
    assert_eq!(
        bad,
        vec![2],
        "truncated_journal scenario: expected bad_lines == [2], got {bad:?}"
    );
}

#[test]
fn concurrent_journal_scenario_reports_line_2_bad() {
    let path = scenario_path("concurrent_journal");
    assert!(path.is_file(), "missing fixture: {}", path.display());
    let bad = validate_jsonl(&path).expect("validate_jsonl");
    assert_eq!(
        bad,
        vec![2],
        "concurrent_journal scenario: expected bad_lines == [2], got {bad:?}"
    );
}

#[test]
fn missing_file_returns_empty() {
    let path = PathBuf::from("/tmp/cacg-core-journal-test-does-not-exist-abc123");
    assert!(!path.exists());
    let bad = validate_jsonl(&path).expect("validate_jsonl on missing path");
    assert!(bad.is_empty());
}

#[test]
fn non_file_path_errors_non_file() {
    let path = std::env::temp_dir();
    assert!(path.is_dir());
    let r = validate_jsonl(&path);
    assert!(matches!(r, Err(JournalError::NonFile(_))));
}
