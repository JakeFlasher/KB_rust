#![allow(clippy::unwrap_used)]
//! Integration tests for `kb lint --all-readings` (task-vh-7 batch path).
//!
//! These tests run the real `target/debug/kb` binary against the
//! committed parity corpora and assert the end-to-end behavior matches
//! what Python emits: exit codes, stderr shape, and per-card journal
//! cardinality. They complement the `kb_lint_parity_golden` /
//! `kb_lint_parity_adversarial` rows in `xtask::parity::matrix` which
//! drive the per-card single-card path; this file exclusively exercises
//! the `--all-readings` batch path.

use std::path::{Path, PathBuf};
use std::process::Command;

fn workspace_root() -> PathBuf {
    // CARGO_MANIFEST_DIR is `crates/cacg-cli`; pop twice to get the
    // workspace root.
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

fn corpus_path(rel: &str) -> PathBuf {
    workspace_root().join(rel)
}

#[test]
fn kb_lint_all_readings_succeeds_on_valid_corpus() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let journal = tmp.path().join("lint_journal.jsonl");
    let output = Command::new(kb_bin())
        .arg("lint")
        .arg("--all-readings")
        .arg("--cards-dir")
        .arg(corpus_path("tests/parity_corpus/valid"))
        .arg("--chunks-manifest")
        .arg(corpus_path(
            "tests/parity_corpus/out_python/chunks_manifest.json",
        ))
        .arg("--source-matrix")
        .arg(corpus_path(
            "tests/parity_corpus/out_python/source_matrix.json",
        ))
        .arg("--journal")
        .arg(&journal)
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb lint --all-readings");
    assert!(
        output.status.success(),
        "expected exit 0 on the committed valid corpus; got status={:?} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stdout.is_empty(),
        "expected empty stdout on a clean batch lint; got {:?}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        output.stderr.is_empty(),
        "expected empty stderr on a clean batch lint; got {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    let journal_body = std::fs::read_to_string(&journal).expect("read journal");
    assert_eq!(
        journal_body.lines().count(),
        5,
        "expected exactly one journal line per valid card (5 fixtures); journal=\n{journal_body}"
    );
}

#[test]
fn kb_lint_all_readings_fails_on_adversarial_corpus() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let journal = tmp.path().join("lint_journal.jsonl");
    let output = Command::new(kb_bin())
        .arg("lint")
        .arg("--all-readings")
        .arg("--cards-dir")
        .arg(corpus_path("tests/parity_corpus/adversarial"))
        .arg("--chunks-manifest")
        .arg(corpus_path(
            "tests/parity_corpus/out_python/chunks_manifest.json",
        ))
        .arg("--source-matrix")
        .arg(corpus_path(
            "tests/parity_corpus/out_python/source_matrix.json",
        ))
        .arg("--journal")
        .arg(&journal)
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb lint --all-readings");
    assert!(
        !output.status.success(),
        "expected non-zero exit on the adversarial corpus"
    );
    // Every adversarial card emits exactly one journal line; the
    // committed corpus has 12 cards (.md files).
    let journal_body = std::fs::read_to_string(&journal).expect("read journal");
    assert_eq!(
        journal_body.lines().count(),
        12,
        "expected one journal line per adversarial card (12 fixtures); journal=\n{journal_body}"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Spot-check that the trust-bearing codes the smoke test confirmed
    // are present in stderr.
    for expected_code in &[
        "CACG-CITE-002",
        "CACG-CITE-003",
        "CACG-CITE-004",
        "CACG-CITE-005",
        "CACG-HASH-001",
        "CACG-HASH-002",
        "CACG-AUTH-001",
    ] {
        assert!(
            stderr.contains(expected_code),
            "expected stderr to contain {expected_code:?}; got:\n{stderr}"
        );
    }
}

#[test]
fn kb_lint_all_readings_rejects_missing_cards_dir() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let missing = tmp.path().join("no_such_directory");
    let journal = tmp.path().join("lint_journal.jsonl");
    let output = Command::new(kb_bin())
        .arg("lint")
        .arg("--all-readings")
        .arg("--cards-dir")
        .arg(&missing)
        .arg("--chunks-manifest")
        .arg(corpus_path(
            "tests/parity_corpus/out_python/chunks_manifest.json",
        ))
        .arg("--source-matrix")
        .arg(corpus_path(
            "tests/parity_corpus/out_python/source_matrix.json",
        ))
        .arg("--journal")
        .arg(&journal)
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb lint --all-readings");
    assert!(
        !output.status.success(),
        "missing cards_dir must exit non-zero"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-CLI-001"),
        "expected CACG-CLI-001 in stderr; got {stderr}"
    );
    assert!(
        stderr.contains("cards directory not found"),
        "expected directory-not-found message; got {stderr}"
    );
    // No journal file should have been created — the dispatcher exits
    // before lint_directory walks anything.
    assert!(
        !Path::new(&journal).exists(),
        "no journal should be appended when cards_dir is missing"
    );
}

#[test]
fn kb_lint_all_readings_rejects_non_directory_cards_dir() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let regular_file = tmp.path().join("not_a_dir.md");
    std::fs::write(&regular_file, b"this is a regular file, not a directory").unwrap();
    let journal = tmp.path().join("lint_journal.jsonl");
    let output = Command::new(kb_bin())
        .arg("lint")
        .arg("--all-readings")
        .arg("--cards-dir")
        .arg(&regular_file)
        .arg("--chunks-manifest")
        .arg(corpus_path(
            "tests/parity_corpus/out_python/chunks_manifest.json",
        ))
        .arg("--source-matrix")
        .arg(corpus_path(
            "tests/parity_corpus/out_python/source_matrix.json",
        ))
        .arg("--journal")
        .arg(&journal)
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb lint --all-readings");
    assert!(
        !output.status.success(),
        "non-directory cards_dir must exit non-zero"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-CLI-001"),
        "expected CACG-CLI-001 in stderr; got {stderr}"
    );
    assert!(
        stderr.contains("not a directory"),
        "expected not-a-directory message; got {stderr}"
    );
}
