#![allow(clippy::unwrap_used)]
//! Integration tests for the `kb verify` single-card dispatcher.
//!
//! These run the real `kb` binary against committed fixtures and
//! assert the end-to-end behavior: exit codes, stderr shape, and
//! per-invocation journal cardinality. They complement the
//! `kb_verify_parity_*` rows in `xtask::parity::matrix` (Python ↔
//! Rust byte parity); this file exercises the Rust dispatcher
//! directly.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
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

fn count_journal_events(journal: &PathBuf) -> usize {
    if !journal.is_file() {
        return 0;
    }
    fs::read_to_string(journal)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .count()
}

#[test]
fn kb_verify_clean_valid_card_succeeds_with_one_journal_event() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let journal = tmp.path().join("lint_journal.jsonl");
    let output = Command::new(kb_bin())
        .arg("verify")
        .arg(corpus_path(
            "tests/parity_corpus/valid/01-content-addressable-identity.md",
        ))
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
        .expect("spawn kb verify");
    assert!(
        output.status.success(),
        "expected exit 0 on a clean valid card; status={:?} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        count_journal_events(&journal),
        1,
        "exactly one verify journal event must be appended"
    );
    // Default-path-zero-network contract: with neither
    // `--semantic` nor `--semantic-judge` set, the verify
    // pipeline must NOT construct any Layer-3 evaluator. Any
    // `CACG-VERIFY-002` in stderr would indicate an accidental
    // semantic firing on the default path.
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("CACG-VERIFY-002"),
        "default verify path must not emit CACG-VERIFY-002 (no semantic evaluator was supplied); got stderr: {stderr}",
    );
}

#[test]
fn kb_verify_missing_chunks_manifest_fails_with_journal_event() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let journal = tmp.path().join("lint_journal.jsonl");
    // Point --chunks-manifest at a non-existent path; verify must
    // fail (exit 1) and still record a journal event so the audit
    // trail is complete.
    let output = Command::new(kb_bin())
        .arg("verify")
        .arg(corpus_path(
            "tests/parity_corpus/valid/01-content-addressable-identity.md",
        ))
        .arg("--chunks-manifest")
        .arg(tmp.path().join("does-not-exist-chunks.json"))
        .arg("--source-matrix")
        .arg(corpus_path(
            "tests/parity_corpus/out_python/source_matrix.json",
        ))
        .arg("--journal")
        .arg(&journal)
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb verify");
    assert!(
        !output.status.success(),
        "expected non-zero exit on a missing chunks-manifest"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-"),
        "stderr must carry a CACG-* diagnostic; got: {stderr}"
    );
    assert_eq!(
        count_journal_events(&journal),
        1,
        "the failure path must still append exactly one journal event"
    );
}

#[test]
fn kb_verify_round_summary_directory_in_place_of_summary_is_cacg_cli_001() {
    // The native --round-summary dispatcher pre-checks
    // `summary_path.is_file()` per BL-20260518-shape-check-fs-inputs;
    // a directory in place of the summary file surfaces CACG-CLI-001
    // with the not-a-regular-file message (Python parity with
    // `_cmd_verify_round_summary`). The legacy CACG-CLI-002 stub no
    // longer fires. End-to-end coverage of the CACG-RS-NNN
    // structural branches lives in `tests/kb_verify_round_summary.rs`.
    let tmp = tempfile::tempdir().expect("tempdir");
    let output = Command::new(kb_bin())
        .arg("verify")
        .arg("--round-summary")
        .arg(tmp.path())
        .arg("--chunks-manifest")
        .arg(corpus_path(
            "tests/parity_corpus/out_python/chunks_manifest.json",
        ))
        .arg("--source-matrix")
        .arg(corpus_path(
            "tests/parity_corpus/out_python/source_matrix.json",
        ))
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb verify --round-summary");
    assert!(
        !output.status.success(),
        "the --round-summary path must exit non-zero on a directory input",
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-CLI-001") && stderr.contains("not found or not a regular file"),
        "stderr must surface the CACG-CLI-001 not-a-regular-file diagnostic; got: {stderr}",
    );
    assert!(
        !stderr.contains("CACG-CLI-002"),
        "the legacy CACG-CLI-002 stub must be gone; got: {stderr}",
    );
}

#[test]
fn kb_verify_skip_lint_clean_card_succeeds() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let journal = tmp.path().join("lint_journal.jsonl");
    let output = Command::new(kb_bin())
        .arg("verify")
        .arg(corpus_path(
            "tests/parity_corpus/valid/01-content-addressable-identity.md",
        ))
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
        .arg("--unsafe-skip-lint")
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb verify --unsafe-skip-lint");
    assert!(
        output.status.success(),
        "expected exit 0 on a clean valid card under --unsafe-skip-lint; status={:?} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(count_journal_events(&journal), 1);
}

#[test]
fn kb_verify_missing_card_short_command_is_cacg_cli_001_not_man_001() {
    // Regression for the AC-1 contracted command shape:
    //   kb verify <missing>.md --source-matrix <matrix>
    // with NO --chunks-manifest. Omitting --chunks-manifest defaults it
    // to `<cwd>/out/chunks_manifest.json`, so the derived sibling
    // cards_manifest.json resolves to `<cwd>/out/cards_manifest.json`.
    // When that sibling is stale/malformed, the pre-fix dispatcher
    // fail-closed with CACG-MAN-001 BEFORE the missing-card preflight,
    // shadowing the contracted CACG-CLI-001. The fix runs an `is_file()`
    // card-path preflight first (BL-20260518-shape-check-fs-inputs), so a
    // missing card path surfaces CACG-CLI-001 regardless of the sibling
    // manifest's health, while an EXISTING card still fail-closes on a
    // malformed manifest (covered elsewhere).
    let tmp = tempfile::tempdir().expect("tempdir");
    let out_dir = tmp.path().join("out");
    fs::create_dir_all(&out_dir).expect("create out/ dir");
    // A malformed sibling cards_manifest.json that WOULD fail-close with
    // CACG-MAN-001 if it were ever loaded for this invocation.
    fs::write(out_dir.join("cards_manifest.json"), "{ this is not valid json")
        .expect("write malformed cards_manifest.json");

    let output = Command::new(kb_bin())
        .current_dir(tmp.path())
        .arg("verify")
        .arg("does-not-exist-card.md")
        .arg("--source-matrix")
        .arg(corpus_path(
            "tests/parity_corpus/out_python/source_matrix.json",
        ))
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb verify");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "a missing card path must exit non-zero; stderr={stderr}"
    );
    assert!(
        stderr.contains("CACG-CLI-001") && stderr.contains("file not found"),
        "missing card under the short command must surface CACG-CLI-001 file-not-found; got: {stderr}"
    );
    assert!(
        !stderr.contains("CACG-MAN-001"),
        "a stale sibling cards_manifest.json must NOT shadow the missing-card diagnostic; got: {stderr}"
    );
}
