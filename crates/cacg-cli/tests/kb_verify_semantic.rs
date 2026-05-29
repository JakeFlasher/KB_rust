#![allow(clippy::unwrap_used)]
//! Integration tests for `kb verify --semantic <cache>` wiring.
//!
//! Spawns the real `kb` binary and asserts the CLI's pre-flight on
//! the supplied cache path:
//!
//! * Missing path → `CACG-MAN-001: semantic cache not found …` + exit 1.
//! * Directory in place of file → `CACG-MAN-001` + exit 1.
//! * Malformed JSON → `CACG-MAN-001: cannot load semantic cache …` + exit 1.
//! * `--semantic-judge` alone → constructs a B2 evaluator and proceeds
//!   through normal verification; missing `ANTHROPIC_API_KEY` does NOT
//!   short-circuit at startup any more (it surfaces at Layer-3 firing
//!   time as `CACG-VERIFY-002` via `SemanticEvaluationError`).
//!
//! The "happy path" (a well-formed cache + a Layer-2 failure that the
//! cache resolves) needs a fully-built `chunks_manifest` + card +
//! source-matrix and is exercised library-side by the runner mock-
//! evaluator tests in
//! `crates/cacg-core/src/verify/runner.rs::tests::semantic_*`.

use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

fn write(path: &Path, body: &[u8]) -> PathBuf {
    std::fs::write(path, body).unwrap();
    path.to_path_buf()
}

/// Spawn `kb verify <card> --chunks-manifest <m> --source-matrix <s>
/// --semantic <cache>` and return the captured `Output`. The card and
/// manifests are tempdir-relative names; the test only cares that the
/// `--semantic <cache>` pre-flight fires before any verify pipeline
/// load.
fn spawn_verify_semantic(dir: &Path, card: &Path, cache: &Path) -> std::process::Output {
    Command::new(kb_bin())
        .arg("verify")
        .arg(card)
        .arg("--chunks-manifest")
        .arg(dir.join("chunks_manifest.json"))
        .arg("--source-matrix")
        .arg(dir.join("source_matrix.json"))
        .arg("--semantic")
        .arg(cache)
        .output()
        .expect("spawn kb verify --semantic")
}

#[test]
fn missing_semantic_cache_path_is_cacg_man_001() {
    let tmp = TempDir::new().unwrap();
    let card = write(&tmp.path().join("card.md"), b"placeholder card body");
    let cache = tmp.path().join("does_not_exist.json");
    let output = spawn_verify_semantic(tmp.path(), &card, &cache);
    let code = output.status.code().expect("exit code");
    assert_eq!(code, 1, "exit 1 for missing semantic cache");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-MAN-001"),
        "stderr must include CACG-MAN-001; got: {stderr}",
    );
    assert!(
        stderr.contains("semantic cache not found or not a regular file"),
        "stderr must include the missing-cache message; got: {stderr}",
    );
}

#[test]
fn directory_in_place_of_semantic_cache_is_cacg_man_001() {
    let tmp = TempDir::new().unwrap();
    let card = write(&tmp.path().join("card.md"), b"placeholder card body");
    let cache_dir = tmp.path().join("cache_dir");
    std::fs::create_dir(&cache_dir).unwrap();
    let output = spawn_verify_semantic(tmp.path(), &card, &cache_dir);
    let code = output.status.code().expect("exit code");
    assert_eq!(code, 1);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("CACG-MAN-001"));
    assert!(
        stderr.contains("semantic cache not found or not a regular file"),
        "stderr should include the regular-file message; got: {stderr}",
    );
}

#[test]
fn malformed_semantic_cache_is_cacg_man_001() {
    let tmp = TempDir::new().unwrap();
    let card = write(&tmp.path().join("card.md"), b"placeholder card body");
    let cache = write(&tmp.path().join("cache.json"), b"{ not valid json");
    let output = spawn_verify_semantic(tmp.path(), &card, &cache);
    let code = output.status.code().expect("exit code");
    assert_eq!(code, 1, "exit 1 for malformed semantic cache");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-MAN-001"),
        "stderr must include CACG-MAN-001; got: {stderr}",
    );
    assert!(
        stderr.contains("cannot load semantic cache"),
        "stderr must include the load-failure prefix; got: {stderr}",
    );
}

#[cfg(feature = "b2-llm-judge")]
#[test]
fn semantic_judge_alone_constructs_b2_evaluator_and_proceeds_past_startup() {
    // With the B2 wiring landed, `--semantic-judge` no longer
    // short-circuits at startup with a not-yet-supported stub. The
    // CLI now constructs a B2 evaluator and proceeds to the normal
    // verify pipeline. Any startup-time failure (cache load, file
    // shape) is therefore the CARD pipeline's responsibility,
    // not the semantic flag's. A placeholder card with no YAML
    // frontmatter fails at the frontmatter loader with
    // `CACG-FM-004` — and crucially the stderr MUST NOT mention the
    // old "--semantic-judge is not yet supported" stub text.
    let tmp = TempDir::new().unwrap();
    let card = write(&tmp.path().join("card.md"), b"placeholder card body");
    let output = Command::new(kb_bin())
        .arg("verify")
        .arg(&card)
        .arg("--chunks-manifest")
        .arg(tmp.path().join("chunks_manifest.json"))
        .arg("--source-matrix")
        .arg(tmp.path().join("source_matrix.json"))
        .arg("--semantic-judge")
        .output()
        .expect("spawn kb verify --semantic-judge");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("not yet supported"),
        "stderr must NOT mention 'not yet supported' — the stub has been replaced; got: {stderr}",
    );
    assert!(
        !stderr.contains("CACG-MAN-001: --semantic-judge",),
        "stderr must NOT carry the old stub's CACG-MAN-001 line; got: {stderr}",
    );
    // The placeholder card has no frontmatter, so the pipeline
    // surfaces the expected frontmatter error, proving the verify
    // path proceeded past `build_semantic_evaluator`.
    assert!(
        stderr.contains("CACG-FM-004"),
        "stderr must surface the frontmatter loader's CACG-FM-004 error \
         (proving the verify pipeline proceeded past --semantic-judge \
         construction); got: {stderr}",
    );
}

#[cfg(feature = "b2-llm-judge")]
#[test]
fn semantic_and_semantic_judge_together_fail_via_clap_mutex() {
    let tmp = TempDir::new().unwrap();
    let card = write(&tmp.path().join("card.md"), b"placeholder card body");
    let output = Command::new(kb_bin())
        .arg("verify")
        .arg(&card)
        .arg("--chunks-manifest")
        .arg(tmp.path().join("chunks_manifest.json"))
        .arg("--source-matrix")
        .arg(tmp.path().join("source_matrix.json"))
        .arg("--semantic")
        .arg(tmp.path().join("cache.json"))
        .arg("--semantic-judge")
        .output()
        .expect("spawn kb verify --semantic --semantic-judge");
    let code = output.status.code().expect("exit code");
    assert_eq!(
        code,
        2,
        "exit 2 for clap mutex violation; stderr={}",
        String::from_utf8_lossy(&output.stderr),
    );
}

#[cfg(not(feature = "b2-llm-judge"))]
#[test]
fn semantic_judge_rejected_by_clap_without_b2_feature() {
    // Under default features (without `b2-llm-judge`), the
    // `--semantic-judge` flag is invisible to clap and must be
    // rejected at argv parse time with exit code 2 ("unexpected
    // argument" / "unrecognized" error), NOT handled by any
    // runtime stub. The absence of `CACG-MAN-001` in stderr
    // proves the rejection happened before the dispatcher's
    // semantic-evaluator construction ran.
    let tmp = TempDir::new().unwrap();
    let card = write(&tmp.path().join("card.md"), b"placeholder card body");
    let output = Command::new(kb_bin())
        .arg("verify")
        .arg(&card)
        .arg("--chunks-manifest")
        .arg(tmp.path().join("chunks_manifest.json"))
        .arg("--source-matrix")
        .arg(tmp.path().join("source_matrix.json"))
        .arg("--semantic-judge")
        .output()
        .expect("spawn kb verify --semantic-judge");
    let code = output.status.code().expect("exit code");
    assert_eq!(
        code,
        2,
        "exit 2 expected for clap unknown-argument error; stderr={}",
        String::from_utf8_lossy(&output.stderr),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unexpected argument") || stderr.contains("unrecognized"),
        "stderr should signal clap-level parse error; got: {stderr}",
    );
    assert!(
        !stderr.contains("CACG-MAN-001"),
        "stderr must NOT contain CACG-MAN-001 (proves rejection happens at argv parse, not runtime); got: {stderr}",
    );
}
