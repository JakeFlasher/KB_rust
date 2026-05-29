#![allow(clippy::unwrap_used)]
//! Integration tests for the native `kb new` CLI verb.
//!
//! Spawns the built `kb` binary and exercises:
//!   - happy path: scaffold a new card; assert the written
//!     bytes equal Python `kb new`'s output byte-for-byte.
//!   - `CACG-CLI-003` invalid slug + invalid `reading_id`
//!     (exit 2).
//!   - `CACG-CLI-005` exists + no `--force` (exit 1) and
//!     `--force` overwrite path (exit 0).
//!   - `CACG-CLI-001` `cards_dir` is a regular file (exit 1).
//!   - `--title` override path.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

/// Returns `true` when the test should skip because the
/// project's `legacy_python_oracle/.venv/bin/python` is unavailable. Honors
/// `CACG_REQUIRE_PYTHON=1` as a hard-failure gate so CI
/// runs that have provisioned a venv can enforce the
/// PY-IS-ORACLE byte-equality assertion instead of silently
/// no-op-ing through it (Round-28 review P2-1; mirrors the
/// existing `CACG_REQUIRE_PDFIUM=1` pattern at
/// `crates/cacg-cli/tests/kb_ingest.rs`).
fn skip_if_python_unavailable(venv_python: &Path) -> bool {
    if venv_python.is_file() {
        return false;
    }
    assert!(
        !(std::env::var("CACG_REQUIRE_PYTHON").as_deref() == Ok("1")),
        "CACG_REQUIRE_PYTHON=1 set but project .venv is absent at {}",
        venv_python.display()
    );
    eprintln!(
        "skipping: project .venv not present at {} \
         (set CACG_REQUIRE_PYTHON=1 to make this a hard failure).",
        venv_python.display()
    );
    true
}

#[test]
fn kb_new_happy_path_writes_card_and_prints_summary() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let cards_dir = tmp.path().join("cards");

    let output = Command::new(kb_bin())
        .arg("new")
        .arg("reading_01")
        .arg("my-card")
        .arg("--cards-dir")
        .arg(&cards_dir)
        .output()
        .expect("spawn kb");

    assert!(
        output.status.success(),
        "exit={:?} stderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8(output.stdout).expect("utf-8 stdout");
    assert!(stdout.contains("card: "), "stdout: {stdout}");
    assert!(
        stdout.contains("reading_id: reading_01"),
        "stdout: {stdout}"
    );
    assert!(stdout.contains("slug: my-card"), "stdout: {stdout}");
    assert!(stdout.contains("status: scaffold"), "stdout: {stdout}");

    let card_path = cards_dir.join("reading_01").join("my-card.md");
    assert!(card_path.is_file(), "card file must exist at {card_path:?}");
    let card_text = fs::read_to_string(&card_path).expect("read card");
    assert!(card_text.contains("schema_version: \"cacg.v0\""));
    assert!(card_text.contains("id: \"my-card\""));
    assert!(card_text.contains("title: \"My Card\""));
    assert!(card_text.contains("reading_id: \"reading_01\""));
    // The "tripwire" zero-hash is the load-bearing scaffold
    // assertion — without it the linter would accept the card.
    assert!(card_text.contains(&format!("chunk_hash: \"{}\"", "0".repeat(64))));
}

#[test]
fn kb_new_output_is_byte_equal_with_python_kb_new() {
    let venv_python = workspace_root().join("legacy_python_oracle/.venv/bin/python");
    if skip_if_python_unavailable(&venv_python) {
        return;
    }

    // Each parametric slug exercises a distinct corner of the
    // Python `str.title()` contract. `byte-parity-card` covers
    // the hyphen-separator path; `v2test` covers the
    // digit-letter boundary the Round-28 review P1 caught
    // diverging; `abc123def` covers the multi-digit-letter
    // boundary path. All three MUST be byte-equal with Python's
    // scaffold output.
    for slug in ["byte-parity-card", "v2test", "abc123def"] {
        let tmp = tempfile::tempdir().expect("tempdir");
        let py_dir = tmp.path().join("py");
        let rs_dir = tmp.path().join("rs");

        let py = Command::new(&venv_python)
            .args(["-m", "cacg.cli", "new", "reading_01", slug, "--cards-dir"])
            .arg(&py_dir)
            .env("PYTHONPATH", workspace_root().join("src"))
            .output()
            .expect("spawn python");
        assert!(
            py.status.success(),
            "python kb new {slug:?} failed: stderr={}",
            String::from_utf8_lossy(&py.stderr)
        );

        let rs = Command::new(kb_bin())
            .args(["new", "reading_01", slug, "--cards-dir"])
            .arg(&rs_dir)
            .output()
            .expect("spawn kb");
        assert!(
            rs.status.success(),
            "rust kb new {slug:?} failed: stderr={}",
            String::from_utf8_lossy(&rs.stderr)
        );

        let py_card = py_dir.join("reading_01").join(format!("{slug}.md"));
        let rs_card = rs_dir.join("reading_01").join(format!("{slug}.md"));
        let py_bytes = fs::read(&py_card).expect("read py card");
        let rs_bytes = fs::read(&rs_card).expect("read rs card");
        assert_eq!(
            py_bytes,
            rs_bytes,
            "scaffold card bytes must be byte-equal between Python \
             and Rust kb new for slug {slug:?}; py_len={} rs_len={}",
            py_bytes.len(),
            rs_bytes.len(),
        );
    }
}

#[test]
fn kb_new_invalid_reading_id_surfaces_cacg_cli_003_with_exit_2() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let output = Command::new(kb_bin())
        .args(["new", "Bad.Reading", "good-slug", "--cards-dir"])
        .arg(tmp.path())
        .output()
        .expect("spawn kb");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-CLI-003: reading_id must match"),
        "stderr: {stderr}"
    );
    // py_repr of the offending value (single-quoted) is the
    // byte-equal Python format (`{value!r}`).
    assert!(stderr.contains("'Bad.Reading'"), "stderr: {stderr}");
}

#[test]
fn kb_new_invalid_slug_surfaces_cacg_cli_003_with_exit_2() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let output = Command::new(kb_bin())
        .args(["new", "reading_01", "Bad.Slug", "--cards-dir"])
        .arg(tmp.path())
        .output()
        .expect("spawn kb");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-CLI-003: slug must match"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains("'Bad.Slug'"), "stderr: {stderr}");
}

#[test]
fn kb_new_existing_card_without_force_surfaces_cacg_cli_005() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let cards_dir = tmp.path().join("cards");

    // First run: creates the card.
    let first = Command::new(kb_bin())
        .args(["new", "reading_01", "dup-card", "--cards-dir"])
        .arg(&cards_dir)
        .output()
        .expect("spawn kb (first)");
    assert!(first.status.success(), "first run failed");

    // Second run without --force: must refuse with CACG-CLI-005.
    let second = Command::new(kb_bin())
        .args(["new", "reading_01", "dup-card", "--cards-dir"])
        .arg(&cards_dir)
        .output()
        .expect("spawn kb (second)");
    assert_eq!(second.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&second.stderr);
    assert!(
        stderr.contains("CACG-CLI-005: card already exists"),
        "stderr: {stderr}"
    );
}

#[test]
fn kb_new_force_overwrites_existing_card() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let cards_dir = tmp.path().join("cards");

    let first = Command::new(kb_bin())
        .args(["new", "reading_01", "force-card", "--cards-dir"])
        .arg(&cards_dir)
        .output()
        .expect("spawn kb (first)");
    assert!(first.status.success(), "first run failed");

    // Mutate the file so we can prove --force re-wrote it.
    let card = cards_dir.join("reading_01").join("force-card.md");
    fs::write(&card, "MUTATED").expect("write mutated");

    let second = Command::new(kb_bin())
        .args(["new", "reading_01", "force-card", "--cards-dir"])
        .arg(&cards_dir)
        .arg("--force")
        .output()
        .expect("spawn kb (second)");
    assert!(second.status.success(), "force run failed");
    let text = fs::read_to_string(&card).expect("read");
    assert!(
        !text.contains("MUTATED"),
        "card was not overwritten; text starts: {:?}",
        &text[..text.len().min(50)]
    );
    assert!(text.contains("schema_version: \"cacg.v0\""));
}

#[test]
fn kb_new_cards_dir_is_regular_file_surfaces_cacg_cli_001() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let cards_dir_as_file = tmp.path().join("not_a_dir");
    fs::write(&cards_dir_as_file, b"i am a regular file").expect("write");

    let output = Command::new(kb_bin())
        .args(["new", "reading_01", "any-slug", "--cards-dir"])
        .arg(&cards_dir_as_file)
        .output()
        .expect("spawn kb");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-CLI-001: cards_dir is not a directory"),
        "stderr: {stderr}"
    );
}

#[test]
fn kb_new_title_override_takes_precedence_over_slug_derivation() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let cards_dir = tmp.path().join("cards");

    let output = Command::new(kb_bin())
        .args([
            "new",
            "reading_01",
            "auto-slug",
            "--title",
            "Explicit Title",
            "--cards-dir",
        ])
        .arg(&cards_dir)
        .output()
        .expect("spawn kb");
    assert!(output.status.success());

    let card = cards_dir.join("reading_01").join("auto-slug.md");
    let text = fs::read_to_string(&card).expect("read");
    assert!(text.contains("title: \"Explicit Title\""), "card: {text}");
    // The slug-derived "Auto Slug" must NOT appear.
    assert!(!text.contains("title: \"Auto Slug\""));
}
