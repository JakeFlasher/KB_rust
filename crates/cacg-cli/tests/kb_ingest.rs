#![allow(clippy::unwrap_used)]
//! Integration tests for the native `kb ingest` CLI verb.
//!
//! Spawns the built `kb` binary and exercises:
//!   - happy path: ingest `cfa_vol1_trim.pdf` into a tempdir; assert
//!     exit 0, stdout summary, and the published manifest pair.
//!   - CACG-CLI-001 missing PDF (exit 1) + non-directory --out (exit 1).
//!   - CACG-CLI-003 invalid --source-id (exit 2).
//!   - CACG-CLI-004 `--config` end-to-end:
//!       * valid YAML overrides flow through to the chunker (more
//!         chunks under a tight `target_tokens` budget);
//!       * missing config file → CACG-CLI-004 + exit 2;
//!       * unknown key → CACG-CLI-004 + exit 2;
//!       * out-of-range value → CACG-CLI-004 + exit 2.
//!   - CACG-INGEST-001 garbage PDF (exit 1).
//!   - CACG-INGEST-003 second-publish-into-same-out_dir refusal (exit 1).
//!
//! Pdfium-bound tests (happy path, garbage PDF, second publish,
//! `--config` overrides) probe `Pdfium::bind_to_system_library()`
//! first; skip with a notice when libpdfium is absent. Set
//! `CACG_REQUIRE_PDFIUM=1` to make a missing libpdfium a hard failure
//! (`docs/pdfium-binary-provisioning.md` "Strict Mode (CI Gate)").

#![cfg(feature = "ingest")]

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use pdfium_render::prelude::Pdfium;

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn fixture_pdf() -> PathBuf {
    workspace_root()
        .join("tests")
        .join("parity_corpus")
        .join("pdfs")
        .join("cfa_vol1_trim.pdf")
}

fn pdfium_available() -> bool {
    Pdfium::bind_to_system_library().is_ok()
}

fn skip_if_pdfium_unavailable() -> bool {
    if pdfium_available() {
        return false;
    }
    assert!(
        !(std::env::var("CACG_REQUIRE_PDFIUM").as_deref() == Ok("1")),
        "CACG_REQUIRE_PDFIUM=1 set but libpdfium is not available; \
             see docs/pdfium-binary-provisioning.md."
    );
    eprintln!(
        "skipping: libpdfium not available on this runner; \
         see docs/pdfium-binary-provisioning.md \
         (set CACG_REQUIRE_PDFIUM=1 to make this a hard failure)."
    );
    true
}

#[test]
fn kb_ingest_cfa_vol1_trim_writes_manifests_and_prints_summary() {
    if skip_if_pdfium_unavailable() {
        return;
    }
    let pdf = fixture_pdf();
    if !pdf.is_file() {
        eprintln!("skipping: fixture {} not found", pdf.display());
        return;
    }

    let out_dir = tempfile::tempdir().expect("tempdir");
    let output = Command::new(kb_bin())
        .arg("ingest")
        .arg(&pdf)
        .arg("--out")
        .arg(out_dir.path())
        .arg("--source-id")
        .arg("cfa_vol1_trim")
        .output()
        .expect("spawn kb");

    assert!(
        output.status.success(),
        "exit={:?} stderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8(output.stdout).expect("utf-8 stdout");
    assert!(stdout.contains("sources_manifest:"), "stdout: {stdout}");
    assert!(stdout.contains("chunks_manifest:"), "stdout: {stdout}");
    // cfa_vol1_trim has 34 chunks per the committed oracle.
    assert!(
        stdout.contains("chunks_count:     34"),
        "expected 34 chunks; stdout: {stdout}"
    );
    assert!(
        stdout.contains("source_id:        cfa_vol1_trim"),
        "stdout: {stdout}"
    );

    // The published manifest pair must exist on disk.
    assert!(out_dir.path().join("sources_manifest.json").is_file());
    assert!(out_dir.path().join("chunks_manifest.json").is_file());
}

#[test]
fn kb_ingest_missing_pdf_surfaces_cacg_cli_001_with_exit_1() {
    let out_dir = tempfile::tempdir().expect("tempdir");
    let output = Command::new(kb_bin())
        .arg("ingest")
        .arg("/tmp/this-pdf-does-not-exist-9eaad0c.pdf")
        .arg("--out")
        .arg(out_dir.path())
        .output()
        .expect("spawn kb");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-CLI-001: pdf not found or not a regular file"),
        "stderr: {stderr}"
    );
}

#[test]
fn kb_ingest_out_is_a_regular_file_surfaces_cacg_cli_001_with_exit_1() {
    let pdf = fixture_pdf();
    if !pdf.is_file() {
        eprintln!("skipping: fixture {} not found", pdf.display());
        return;
    }

    let tmp = tempfile::tempdir().expect("tempdir");
    let out_file = tmp.path().join("out_is_a_file");
    fs::write(&out_file, b"not a directory").expect("write");

    let output = Command::new(kb_bin())
        .arg("ingest")
        .arg(&pdf)
        .arg("--out")
        .arg(&out_file)
        .output()
        .expect("spawn kb");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-CLI-001: out path is not a directory"),
        "stderr: {stderr}"
    );
}

#[test]
fn kb_ingest_invalid_source_id_surfaces_cacg_cli_003_with_exit_2() {
    let pdf = fixture_pdf();
    if !pdf.is_file() {
        eprintln!("skipping: fixture {} not found", pdf.display());
        return;
    }
    let out_dir = tempfile::tempdir().expect("tempdir");

    // Uppercase + dot violate the ^[a-z0-9][a-z0-9_]*$ pattern.
    let output = Command::new(kb_bin())
        .arg("ingest")
        .arg(&pdf)
        .arg("--out")
        .arg(out_dir.path())
        .arg("--source-id")
        .arg("Bad.Source")
        .output()
        .expect("spawn kb");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Byte-equal Python `_cmd_ingest`'s `!r`-formatted diagnostic:
    // the source_id must appear single-quoted (Python repr) to match
    // a future xtask kb_ingest parity row's expectations.
    assert!(
        stderr.contains("CACG-CLI-003: source_id must match '^[a-z0-9][a-z0-9_]*$': 'Bad.Source'"),
        "stderr: {stderr}"
    );
}

#[test]
fn kb_ingest_config_tight_target_tokens_produces_more_chunks_than_default() {
    if skip_if_pdfium_unavailable() {
        return;
    }
    let pdf = fixture_pdf();
    if !pdf.is_file() {
        eprintln!("skipping: fixture {} not found", pdf.display());
        return;
    }

    // Baseline run: defaults yield the committed 34-chunk oracle.
    let default_out = tempfile::tempdir().expect("tempdir");
    let default_run = Command::new(kb_bin())
        .arg("ingest")
        .arg(&pdf)
        .arg("--out")
        .arg(default_out.path())
        .arg("--source-id")
        .arg("cfa_vol1_trim")
        .output()
        .expect("spawn kb (defaults)");
    assert!(
        default_run.status.success(),
        "default run failed: stderr={}",
        String::from_utf8_lossy(&default_run.stderr)
    );
    let default_stdout = String::from_utf8(default_run.stdout).expect("utf-8 stdout (defaults)");
    let default_chunks =
        parse_chunks_count(&default_stdout).expect("default stdout must contain chunks_count");

    // Config run: tight target_tokens=100 (vs 350 default) must
    // produce strictly more chunks on the same PDF.
    let cfg_tmp = tempfile::tempdir().expect("tempdir (cfg)");
    let cfg = cfg_tmp.path().join("tight.yaml");
    fs::write(
        &cfg,
        b"chunking:\n  target_tokens: 100\n  overlap_tokens: 10\n",
    )
    .expect("write cfg");

    let tight_out = tempfile::tempdir().expect("tempdir (tight)");
    let tight_run = Command::new(kb_bin())
        .arg("ingest")
        .arg(&pdf)
        .arg("--out")
        .arg(tight_out.path())
        .arg("--source-id")
        .arg("cfa_vol1_trim")
        .arg("--config")
        .arg(&cfg)
        .output()
        .expect("spawn kb (tight)");
    assert!(
        tight_run.status.success(),
        "tight run failed: stderr={}",
        String::from_utf8_lossy(&tight_run.stderr)
    );
    let tight_stdout = String::from_utf8(tight_run.stdout).expect("utf-8 stdout (tight)");
    let tight_chunks =
        parse_chunks_count(&tight_stdout).expect("tight stdout must contain chunks_count");

    assert!(
        tight_chunks > default_chunks,
        "tight target_tokens=100 must yield more chunks than default \
         (target_tokens=350); got tight={tight_chunks} default={default_chunks}"
    );
}

#[test]
fn kb_ingest_missing_config_file_surfaces_cacg_cli_004_with_exit_2() {
    let pdf = fixture_pdf();
    if !pdf.is_file() {
        eprintln!("skipping: fixture {} not found", pdf.display());
        return;
    }
    let out_dir = tempfile::tempdir().expect("tempdir");

    let output = Command::new(kb_bin())
        .arg("ingest")
        .arg(&pdf)
        .arg("--out")
        .arg(out_dir.path().join("publish"))
        .arg("--config")
        .arg("/tmp/no-such-config-d3f5a1.yaml")
        .output()
        .expect("spawn kb");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-CLI-004:"),
        "stderr must carry CACG-CLI-004; got: {stderr}"
    );
    assert!(
        stderr.contains("config file not found or not a regular file"),
        "stderr must carry the not-found detail; got: {stderr}"
    );
}

#[test]
fn kb_ingest_unknown_config_key_surfaces_cacg_cli_004_with_exit_2() {
    let pdf = fixture_pdf();
    if !pdf.is_file() {
        eprintln!("skipping: fixture {} not found", pdf.display());
        return;
    }
    let out_dir = tempfile::tempdir().expect("tempdir");
    let cfg = out_dir.path().join("typo.yaml");
    // `target_tokens` at the top level (not under `chunking:`) is the
    // exact typo `_TOP_LEVEL = {"chunking"}` catches in Python.
    fs::write(&cfg, b"target_tokens: 100\n").expect("write cfg");

    let output = Command::new(kb_bin())
        .arg("ingest")
        .arg(&pdf)
        .arg("--out")
        .arg(out_dir.path().join("publish"))
        .arg("--config")
        .arg(&cfg)
        .output()
        .expect("spawn kb");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-CLI-004:"),
        "stderr must carry CACG-CLI-004; got: {stderr}"
    );
    // Byte-equal Python `_TOP_LEVEL` rejection surface: name the
    // offending key + the single allowed key. AC-4 line 64.
    assert!(
        stderr.contains("unknown top-level config keys: ['target_tokens'] (allowed: ['chunking'])"),
        "stderr must carry the byte-equal Python unknown-key diagnostic; got: {stderr}"
    );
}

#[test]
fn kb_ingest_out_of_range_overlap_surfaces_cacg_cli_004_with_exit_2() {
    let pdf = fixture_pdf();
    if !pdf.is_file() {
        eprintln!("skipping: fixture {} not found", pdf.display());
        return;
    }
    let out_dir = tempfile::tempdir().expect("tempdir");
    let cfg = out_dir.path().join("bad_overlap.yaml");
    // overlap_tokens >= target_tokens is the canonical validation
    // failure on the same `ChunkingConfig.validate` boundary.
    fs::write(
        &cfg,
        b"chunking:\n  target_tokens: 50\n  overlap_tokens: 50\n",
    )
    .expect("write cfg");

    let output = Command::new(kb_bin())
        .arg("ingest")
        .arg(&pdf)
        .arg("--out")
        .arg(out_dir.path().join("publish"))
        .arg("--config")
        .arg(&cfg)
        .output()
        .expect("spawn kb");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-CLI-004:"),
        "stderr must carry CACG-CLI-004; got: {stderr}"
    );
    assert!(
        stderr.contains("overlap_tokens"),
        "stderr must mention the failing field; got: {stderr}"
    );
}

/// Extract the integer that follows `chunks_count:` on the
/// `kb ingest` stdout summary, returning `None` if absent.
fn parse_chunks_count(stdout: &str) -> Option<usize> {
    stdout
        .lines()
        .find_map(|l| l.strip_prefix("chunks_count:"))
        .and_then(|rest| rest.trim().parse().ok())
}

#[test]
fn kb_ingest_garbage_bytes_surfaces_cacg_ingest_001_with_exit_1() {
    if skip_if_pdfium_unavailable() {
        return;
    }
    let tmp = tempfile::tempdir().expect("tempdir");
    let garbage = tmp.path().join("garbage.pdf");
    fs::write(&garbage, b"not a pdf file").expect("write garbage");

    let output = Command::new(kb_bin())
        .arg("ingest")
        .arg(&garbage)
        .arg("--out")
        .arg(tmp.path().join("publish"))
        .output()
        .expect("spawn kb");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("CACG-INGEST-001"), "stderr: {stderr}");
}

#[test]
fn kb_ingest_second_publish_into_same_out_dir_surfaces_cacg_ingest_003() {
    if skip_if_pdfium_unavailable() {
        return;
    }
    let pdf = fixture_pdf();
    if !pdf.is_file() {
        eprintln!("skipping: fixture {} not found", pdf.display());
        return;
    }

    let out_dir = tempfile::tempdir().expect("tempdir");

    // First publish: succeeds.
    let first = Command::new(kb_bin())
        .arg("ingest")
        .arg(&pdf)
        .arg("--out")
        .arg(out_dir.path())
        .arg("--source-id")
        .arg("cfa_vol1_trim")
        .output()
        .expect("spawn kb (first)");
    assert!(
        first.status.success(),
        "first publish failed: stderr={}",
        String::from_utf8_lossy(&first.stderr)
    );

    // Second publish into the same out_dir: must fail loudly under
    // the not-yet-implemented merge gate.
    let second = Command::new(kb_bin())
        .arg("ingest")
        .arg(&pdf)
        .arg("--out")
        .arg(out_dir.path())
        .arg("--source-id")
        .arg("cfa_vol1_trim")
        .output()
        .expect("spawn kb (second)");
    assert_eq!(second.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&second.stderr);
    assert!(stderr.contains("CACG-INGEST-003"), "stderr: {stderr}");
}
