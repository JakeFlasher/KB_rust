#![allow(clippy::unwrap_used)]
//! Integration tests for `kb verify --round-summary <summary>
//! --semantic <cache>` — the batch + semantic wiring.
//!
//! Spawns the real `kb` binary against the committed parity-corpus
//! cards and asserts:
//!
//! * The semantic cache is loaded EXACTLY ONCE at dispatcher
//!   startup (a malformed / missing cache emits exactly one
//!   `CACG-MAN-001` and no per-card verification ever runs).
//! * Without `--semantic`, the round-summary path does NOT attempt
//!   any cache load — no `CACG-MAN-001` appears in stderr.
//! * A summary that cites card-A three times and card-B once
//!   produces exactly four journal events (proving the per-cite
//!   `verify_one_card` invocation contract: repeated cards are
//!   visited independently, not collapsed by a within-run cache).
//!
//! These tests use the committed cards under
//! `tests/parity_corpus/cards/reading_01/` and the committed
//! `chunks_manifest` under `tests/parity_corpus/out_python/`.

use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn write_summary(dir: &Path, body: &str) -> PathBuf {
    let path = dir.join("round.md");
    std::fs::write(&path, body).unwrap();
    path
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

/// Spawn `kb verify --round-summary <summary>` against the
/// parity-corpus `chunks_manifest` / `source_matrix`, optionally with
/// `--semantic <cache>`. The `current_dir` is set to the workspace
/// root so the cited card paths resolve.
fn spawn_round_summary(
    summary: &Path,
    journal: &Path,
    semantic_cache: Option<&Path>,
) -> std::process::Output {
    let mut cmd = Command::new(kb_bin());
    cmd.current_dir(workspace_root())
        .arg("verify")
        .arg("--round-summary")
        .arg(summary)
        .arg("--chunks-manifest")
        .arg("tests/parity_corpus/out_python/chunks_manifest.json")
        .arg("--source-matrix")
        .arg("tests/parity_corpus/out_python/source_matrix.json")
        .arg("--journal")
        .arg(journal);
    if let Some(cache) = semantic_cache {
        cmd.arg("--semantic").arg(cache);
    }
    cmd.output()
        .expect("spawn kb verify --round-summary [--semantic]")
}

/// A round summary that cites `01-content-addressable-identity.md`
/// three times and `02-determinism-is-a-kept-promise.md` once.
/// Both cards live in the committed parity-corpus cards directory.
const SUMMARY_CARD_A_THRICE_CARD_B_ONCE: &str = "## Knowledge Consulted\n\n\
    - tests/parity_corpus/cards/reading_01/01-content-addressable-identity.md\n\
    - tests/parity_corpus/cards/reading_01/01-content-addressable-identity.md\n\
    - tests/parity_corpus/cards/reading_01/01-content-addressable-identity.md\n\
    - tests/parity_corpus/cards/reading_01/02-determinism-is-a-kept-promise.md\n";

#[test]
fn round_summary_semantic_card_a_thrice_card_b_once_emits_four_journal_events() {
    let tmp = TempDir::new().unwrap();
    let summary = write_summary(tmp.path(), SUMMARY_CARD_A_THRICE_CARD_B_ONCE);
    let journal = tmp.path().join("lint_journal.jsonl");
    // Use the committed semantic-parity cache. It contains a fail
    // entry for the (chunk_hash, claim_window_hash) of the
    // semantic-parity-card. Neither cited card here happens to
    // produce that exact (chunk_hash, claim_window_hash) pair, so
    // all four cite-side `verify_one_card` invocations Layer-2
    // pass cleanly and Layer-3 does not fire. The journal-event
    // contract (one event per cite) is what this test pins.
    let cache = workspace_root().join("tests/parity_corpus/semantic/semantic_cache.json");
    let output = spawn_round_summary(&summary, &journal, Some(&cache));

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let code = output.status.code().expect("exit code");

    assert!(
        !stderr.contains("CACG-MAN-001"),
        "no startup MAN-001 expected with a valid cache; stderr={stderr}",
    );
    assert_eq!(
        count_journal_events(&journal),
        4,
        "card-A cited 3x + card-B 1x must produce exactly 4 journal events; \
         stdout={stdout} stderr={stderr}",
    );
    // All four cited cards verify cleanly (Layer-2 passes).
    assert_eq!(
        code, 0,
        "all four citations verify cleanly; stdout={stdout} stderr={stderr}",
    );
    // Each cited path appears as `VERIFIED` on stdout once per
    // visit; the dispatcher prints one line per `PathVerdict`.
    let verified_lines = stdout.lines().filter(|l| l.ends_with("VERIFIED")).count();
    assert_eq!(
        verified_lines, 4,
        "expected 4 VERIFIED lines (one per cite); got: {stdout}",
    );
}

#[test]
fn round_summary_without_semantic_does_not_attempt_cache_load() {
    let tmp = TempDir::new().unwrap();
    let summary = write_summary(tmp.path(), SUMMARY_CARD_A_THRICE_CARD_B_ONCE);
    let journal = tmp.path().join("lint_journal.jsonl");
    let output = spawn_round_summary(&summary, &journal, None);

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let code = output.status.code().expect("exit code");

    // No `--semantic` flag → dispatcher must NOT attempt any
    // cache load. `CACG-MAN-001` is the cache-load failure code;
    // its absence proves the load was never attempted.
    assert!(
        !stderr.contains("CACG-MAN-001"),
        "without --semantic, dispatcher must not emit CACG-MAN-001; got stderr={stderr}",
    );
    // The four cites still verify cleanly via Layer-2; the journal
    // cardinality contract is independent of the semantic backend.
    assert_eq!(
        count_journal_events(&journal),
        4,
        "card-A cited 3x + card-B 1x must still produce 4 journal events \
         without --semantic; stdout={stdout} stderr={stderr}",
    );
    assert_eq!(code, 0, "verify clean; stdout={stdout} stderr={stderr}");
}

#[test]
fn round_summary_semantic_valid_cache_loads_exactly_once() {
    // Set `CACG_SEMANTIC_LOAD_TRACE` to a tmpfile and run the same
    // card-A 3x + card-B 1x summary fixture with a VALID cache. The
    // dispatcher must load the cache exactly once at startup, NOT
    // per cited card. A regression that reloaded the cache per
    // visit would emit 4 trace lines instead of 1.
    let tmp = TempDir::new().unwrap();
    let summary = write_summary(tmp.path(), SUMMARY_CARD_A_THRICE_CARD_B_ONCE);
    let journal = tmp.path().join("lint_journal.jsonl");
    let cache = workspace_root().join("tests/parity_corpus/semantic/semantic_cache.json");
    let trace = tmp.path().join("semantic_load_trace.txt");

    let mut cmd = Command::new(kb_bin());
    cmd.current_dir(workspace_root())
        .env("CACG_SEMANTIC_LOAD_TRACE", &trace)
        .arg("verify")
        .arg("--round-summary")
        .arg(&summary)
        .arg("--chunks-manifest")
        .arg("tests/parity_corpus/out_python/chunks_manifest.json")
        .arg("--source-matrix")
        .arg("tests/parity_corpus/out_python/source_matrix.json")
        .arg("--journal")
        .arg(&journal)
        .arg("--semantic")
        .arg(&cache);
    let output = cmd
        .output()
        .expect("spawn kb verify --round-summary --semantic");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let code = output.status.code().expect("exit code");

    // The trace file must exist (subprocess wrote to it).
    let trace_content = std::fs::read_to_string(&trace)
        .expect("CACG_SEMANTIC_LOAD_TRACE file must be created by the subprocess");
    let load_ok_count = trace_content
        .lines()
        .filter(|l| l.starts_with("load_ok "))
        .count();
    let load_err_count = trace_content
        .lines()
        .filter(|l| l.starts_with("load_err "))
        .count();
    assert_eq!(
        load_ok_count, 1,
        "exactly one successful cache load (startup, not per-card); got trace={trace_content:?}, stderr={stderr}",
    );
    assert_eq!(
        load_err_count, 0,
        "no failed loads expected on a valid cache; got trace={trace_content:?}",
    );
    // Journal-cardinality contract: 4 cites → 4 journal events.
    assert_eq!(
        count_journal_events(&journal),
        4,
        "card-A cited 3x + card-B 1x must produce exactly 4 journal events; \
         stderr={stderr}",
    );
    assert_eq!(code, 0, "verify clean; stderr={stderr}");
}

#[test]
fn round_summary_with_missing_semantic_cache_emits_single_man_001() {
    let tmp = TempDir::new().unwrap();
    let summary = write_summary(tmp.path(), SUMMARY_CARD_A_THRICE_CARD_B_ONCE);
    let journal = tmp.path().join("lint_journal.jsonl");
    // Cache path deliberately points at a nonexistent file. The
    // dispatcher's `build_semantic_evaluator` MUST emit
    // `CACG-MAN-001` exactly once at startup and return non-zero
    // BEFORE the per-cite loop runs.
    let cache = tmp.path().join("does_not_exist.json");
    let output = spawn_round_summary(&summary, &journal, Some(&cache));

    let stderr = String::from_utf8_lossy(&output.stderr);
    let code = output.status.code().expect("exit code");

    // Exactly one CACG-MAN-001 emission. If the dispatcher
    // accidentally retried per-card, this count would be N (one
    // per cite). Single startup emission proves the cache-load-
    // once contract.
    let man_001_count = stderr.matches("CACG-MAN-001").count();
    assert_eq!(
        man_001_count, 1,
        "missing cache must emit exactly one CACG-MAN-001 (startup, \
         not per-card); got {man_001_count} in stderr={stderr}",
    );
    assert!(
        stderr.contains("semantic cache not found or not a regular file"),
        "stderr should include the missing-cache message; got: {stderr}",
    );
    // Per-card loop never ran → journal is empty (or absent).
    assert_eq!(
        count_journal_events(&journal),
        0,
        "no per-card verification ran; journal must be empty; stderr={stderr}",
    );
    assert_eq!(code, 1, "exit 1 on cache-load failure; stderr={stderr}");
}
