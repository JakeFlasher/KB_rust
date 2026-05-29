#![allow(clippy::unwrap_used)]
//! Integration tests for the `kb verify --round-summary` dispatcher.
//!
//! These tests spawn the real `kb` binary against tempfile-backed
//! summaries and assert the end-to-end CLI surface: exit codes,
//! stdout/stderr shape, and CACG-RS-NNN emission. They complement
//! the library-level state-machine tests in
//! `crates/cacg-cli/src/round_summary.rs::tests::verify_round_summary_*`
//! by proving the wiring through `dispatch_verify` -> the new
//! `dispatch_verify_round_summary` helper actually takes effect.
//!
//! The structural branches short-circuit BEFORE any per-batch
//! manifest load, so these tests intentionally point at a
//! nonexistent `chunks_manifest.json` and `source_matrix.json`.
//! The per-card verify pipeline (`VERIFIED` / `STALE` / `MISSING`
//! flows against real cards) is covered byte-equal against Python
//! by `tests/round_summary_fixtures_parity.rs` and its committed
//! `tests/round_summary_fixtures/*.expected.json` oracles.

use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

const NA_SENTINEL: &str = "N/A -- task not KB-relevant this round";

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

fn write_summary(dir: &Path, body: &str) -> PathBuf {
    let path = dir.join("round.md");
    std::fs::write(&path, body).unwrap();
    path
}

/// Spawn `kb verify --round-summary <summary> --chunks-manifest <m>
/// --source-matrix <s>` and return the captured `Output`. The
/// chunks-manifest / source-matrix paths point at tempdir-relative
/// names that do NOT exist; the structural branches short-circuit
/// before any per-batch manifest load is attempted.
fn spawn_verify_round_summary(dir: &Path, summary: &Path) -> std::process::Output {
    Command::new(kb_bin())
        .arg("verify")
        .arg("--round-summary")
        .arg(summary)
        .arg("--chunks-manifest")
        .arg(dir.join("chunks_manifest.json"))
        .arg("--source-matrix")
        .arg(dir.join("source_matrix.json"))
        .output()
        .expect("spawn kb verify --round-summary")
}

#[test]
fn missing_summary_path_is_cacg_cli_001() {
    let tmp = TempDir::new().unwrap();
    let missing = tmp.path().join("does_not_exist.md");
    let output = spawn_verify_round_summary(tmp.path(), &missing);
    let code = output.status.code().expect("exit code");
    assert_eq!(
        code,
        1,
        "exit 1 for missing summary; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-CLI-001"),
        "stderr must include CACG-CLI-001; got: {stderr}",
    );
    assert!(
        stderr.contains("not found or not a regular file"),
        "stderr must include the not-a-regular-file message; got: {stderr}",
    );
}

#[test]
fn directory_in_place_of_summary_is_cacg_cli_001() {
    let tmp = TempDir::new().unwrap();
    let as_dir = tmp.path().join("round.md");
    std::fs::create_dir(&as_dir).unwrap();
    let output = spawn_verify_round_summary(tmp.path(), &as_dir);
    let code = output.status.code().expect("exit code");
    assert_eq!(code, 1);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("CACG-CLI-001"));
}

#[test]
fn missing_section_on_kb_relevant_emits_cacg_rs_001_exit_2() {
    let tmp = TempDir::new().unwrap();
    let summary = write_summary(
        tmp.path(),
        "edited cards/qm/foo.md but no section heading\n",
    );
    let output = spawn_verify_round_summary(tmp.path(), &summary);
    let code = output.status.code().expect("exit code");
    assert_eq!(
        code,
        2,
        "missing section + kb_relevant must exit 2; stderr={}",
        String::from_utf8_lossy(&output.stderr),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-RS-001"),
        "stderr must include CACG-RS-001; got: {stderr}",
    );
    assert!(
        stderr.contains("Knowledge Consulted section missing on KB-relevant work"),
        "stderr must include the diagnostic message text; got: {stderr}",
    );
}

#[test]
fn missing_section_on_non_kb_relevant_is_silent_exit_0() {
    let tmp = TempDir::new().unwrap();
    let summary = write_summary(tmp.path(), "round about non-kb work; no relevant paths.\n");
    let output = spawn_verify_round_summary(tmp.path(), &summary);
    let code = output.status.code().expect("exit code");
    assert_eq!(code, 0);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("(no Knowledge Consulted section; round not KB-relevant)"),
        "stdout must include the non-KB-relevant message; got: {stdout}",
    );
}

#[test]
fn clean_na_acknowledged_exit_0() {
    let tmp = TempDir::new().unwrap();
    let body = format!("# round summary\n\n## Knowledge Consulted\n\n{NA_SENTINEL}\n");
    let summary = write_summary(tmp.path(), &body);
    let output = spawn_verify_round_summary(tmp.path(), &summary);
    let code = output.status.code().expect("exit code");
    assert_eq!(code, 0);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("N/A acknowledged"),
        "stdout must include N/A acknowledged; got: {stdout}",
    );
}

#[test]
fn na_sentinel_on_kb_relevant_emits_cacg_rs_003() {
    let tmp = TempDir::new().unwrap();
    let body =
        format!("edited cards/qm/foo.md this round.\n\n## Knowledge Consulted\n\n{NA_SENTINEL}\n",);
    let summary = write_summary(tmp.path(), &body);
    let output = spawn_verify_round_summary(tmp.path(), &summary);
    let code = output.status.code().expect("exit code");
    assert_eq!(code, 1);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("(section): STALE"),
        "stderr must include the (section)/STALE prefix; got: {stderr}",
    );
    assert!(
        stderr.contains("CACG-RS-003"),
        "stderr must include CACG-RS-003; got: {stderr}",
    );
}

#[test]
fn empty_section_on_kb_relevant_emits_cacg_rs_004() {
    let tmp = TempDir::new().unwrap();
    let body = "edited .claude/knowledge/qm/foo.md\n\n## Knowledge Consulted\n\n## Other\nbody\n";
    let summary = write_summary(tmp.path(), body);
    let output = spawn_verify_round_summary(tmp.path(), &summary);
    let code = output.status.code().expect("exit code");
    assert_eq!(code, 1);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("(section): STALE"),
        "stderr must include the (section)/STALE prefix; got: {stderr}",
    );
    assert!(
        stderr.contains("CACG-RS-004"),
        "stderr must include CACG-RS-004; got: {stderr}",
    );
}

#[test]
fn sentinel_collision_emits_cacg_rs_002() {
    let tmp = TempDir::new().unwrap();
    let body = format!("## Knowledge Consulted\n\n{NA_SENTINEL}\n- cards/qm/foo.md\n",);
    let summary = write_summary(tmp.path(), &body);
    let output = spawn_verify_round_summary(tmp.path(), &summary);
    let code = output.status.code().expect("exit code");
    assert_eq!(code, 1);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("(section): STALE"),
        "stderr must include the (section)/STALE prefix; got: {stderr}",
    );
    assert!(
        stderr.contains("CACG-RS-002"),
        "stderr must include CACG-RS-002; got: {stderr}",
    );
}

// ---------- Dispatcher-level edge cases (empty md, duplicates, very-large) ----------

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn spawn_against_parity_corpus(summary: &Path, journal: &Path) -> std::process::Output {
    Command::new(kb_bin())
        .current_dir(workspace_root())
        .arg("verify")
        .arg("--round-summary")
        .arg(summary)
        .arg("--chunks-manifest")
        .arg("tests/parity_corpus/out_python/chunks_manifest.json")
        .arg("--source-matrix")
        .arg("tests/parity_corpus/out_python/source_matrix.json")
        .arg("--journal")
        .arg(journal)
        .output()
        .expect("spawn kb verify --round-summary against parity corpus")
}

fn count_journal_events(journal: &Path) -> usize {
    if !journal.is_file() {
        return 0;
    }
    std::fs::read_to_string(journal)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .count()
}

#[test]
fn empty_markdown_file_is_silent_exit_0_with_no_journal_events() {
    let tmp = TempDir::new().unwrap();
    let summary = write_summary(tmp.path(), "");
    let journal = tmp.path().join("lint_journal.jsonl");
    let output = spawn_against_parity_corpus(&summary, &journal);
    let code = output.status.code().expect("exit code");
    assert_eq!(
        code,
        0,
        "empty markdown must exit 0; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("(no Knowledge Consulted section; round not KB-relevant)"),
        "empty markdown must surface the non-KB-relevant message; got: {stdout}",
    );
    assert_eq!(
        count_journal_events(&journal),
        0,
        "empty markdown must NOT generate any journal events",
    );
}

#[test]
fn duplicate_cited_paths_generate_one_journal_event_per_occurrence() {
    // Python parity: the parser preserves duplicate cited paths and
    // the dispatcher calls verify_one_card for every occurrence. The
    // journal records one event per call, so a round-summary citing
    // the same card N times produces N events.
    let tmp = TempDir::new().unwrap();
    let body = "## Knowledge Consulted\n\n\
        - tests/parity_corpus/cards/reading_01/01-content-addressable-identity.md\n\
        - tests/parity_corpus/cards/reading_01/01-content-addressable-identity.md\n\
        - tests/parity_corpus/cards/reading_01/01-content-addressable-identity.md\n";
    let summary = write_summary(tmp.path(), body);
    let journal = tmp.path().join("lint_journal.jsonl");
    let output = spawn_against_parity_corpus(&summary, &journal);
    let code = output.status.code().expect("exit code");
    assert_eq!(
        code,
        0,
        "all three citations verify; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let verified_lines = stdout.lines().filter(|l| l.ends_with(": VERIFIED")).count();
    assert_eq!(
        verified_lines, 3,
        "three duplicate citations must emit three VERIFIED lines; got: {stdout}",
    );
    assert_eq!(
        count_journal_events(&journal),
        3,
        "journal must record one event per occurrence (3 of 3); journal contents:\n{}",
        std::fs::read_to_string(&journal).unwrap_or_default(),
    );
}

#[test]
fn very_large_summary_processes_without_oom() {
    // Bound: 10_001 bullets pointing to non-existent paths. Each
    // citation routes through verify_one_card -> exists() check -> the
    // MISSING verdict path. The dispatcher must complete without
    // panicking or exhausting memory. We assert exit 1 (any-non-
    // VERIFIED) and at least one MISSING line; the journal-event
    // count and per-process time budget are bounded as smoke checks.
    let tmp = TempDir::new().unwrap();
    let mut body = String::with_capacity(700 * 1024);
    body.push_str("## Knowledge Consulted\n\n");
    for i in 0..10_001 {
        body.push_str(&format!("- cards/reading_99/does_not_exist_{i:05}.md\n"));
    }
    let summary = write_summary(tmp.path(), &body);
    let journal = tmp.path().join("lint_journal.jsonl");

    let start = std::time::Instant::now();
    let output = spawn_against_parity_corpus(&summary, &journal);
    let elapsed = start.elapsed();

    let code = output.status.code().expect("exit code");
    assert_eq!(
        code,
        1,
        "any-non-VERIFIED must exit 1; stderr head={}",
        String::from_utf8_lossy(&output.stderr)
            .chars()
            .take(400)
            .collect::<String>(),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(": MISSING (file not found)"),
        "expected at least one MISSING line in stderr; got first 400 bytes: {}",
        stderr.chars().take(400).collect::<String>(),
    );
    let missing_lines = stderr
        .lines()
        .filter(|l| l.ends_with(": MISSING (file not found)"))
        .count();
    assert_eq!(
        missing_lines, 10_001,
        "all 10_001 citations must emit MISSING lines; got {missing_lines}",
    );
    assert_eq!(
        count_journal_events(&journal),
        10_001,
        "journal must record one event per cited card",
    );
    assert!(
        elapsed.as_secs() < 120,
        "very-large summary should complete in <120s on local hardware; took {elapsed:?}",
    );
}

#[test]
fn cacg_cli_002_stub_is_gone() {
    // Regression: the old stub printed `CACG-CLI-002: kb verify
    // --round-summary not yet implemented` and exited 1. The new
    // dispatcher must not surface this code on any structural branch
    // we already cover.
    let tmp = TempDir::new().unwrap();
    let body = format!("## Knowledge Consulted\n\n{NA_SENTINEL}\n");
    let summary = write_summary(tmp.path(), &body);
    let output = spawn_verify_round_summary(tmp.path(), &summary);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stderr.contains("CACG-CLI-002") && !stdout.contains("CACG-CLI-002"),
        "the CACG-CLI-002 stub must be gone; stdout={stdout}, stderr={stderr}",
    );
}
