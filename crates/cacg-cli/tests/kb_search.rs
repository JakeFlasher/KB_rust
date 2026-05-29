#![allow(clippy::unwrap_used)]
//! Integration parity gate for the `kb search` CLI dispatcher.
//!
//! The Rust `kb search` verb must be byte-equal with Python
//! `legacy_python_oracle/src/cacg/cli.py::_cmd_search` on the in-memory BM25 backend.
//! `legacy_python_oracle/scripts/build_kb_search_oracle.py` captures Python `kb search`
//! stdout + exit code over the sidecar-free summaries triplet in
//! `tests/parity_corpus/kb_search/`; this test byte-compares the Rust
//! `kb` binary against that committed `oracle.json`.
//!
//! This committed-oracle test complements the live `kb_search_parity_corpus`
//! row in `xtask::parity`, which spawns Python and Rust `kb search`
//! side-by-side and byte-compares them. The oracle pins the Python
//! reference bytes at fixture-build time so any drift surfaces as a
//! reviewable committed diff; and, unlike the matrix row, this file
//! also exercises the `kb search` CLI error surface: the `CACG-CLI-001`
//! (missing summaries) vs `CACG-MAN-001` (malformed summaries /
//! malformed sibling `cards_manifest.json`) exit-code split, the
//! `CACG-AUTH-000` malformed-source-matrix path, and the non-file /
//! absent sibling `cards_manifest.json` tolerance (the retraction set
//! degrades to empty, mirroring Python `_cmd_search`'s `is_file()` gate).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

/// The committed sidecar-free corpus directory: `summaries.json` /
/// `source_matrix.json` / `cards_manifest.json` with no
/// `summaries.sqlite` sibling, so both implementations exercise the
/// in-memory backend.
fn corpus_dir() -> PathBuf {
    workspace_root().join("tests/parity_corpus/kb_search")
}

fn load_oracle() -> Value {
    let path = corpus_dir().join("oracle.json");
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let oracle: Value = serde_json::from_str(&raw).expect("oracle.json must parse");
    assert_eq!(oracle["schema_version"], "cacg.v0");
    oracle
}

/// Captured Python `expected_stdout` for the named oracle case.
fn oracle_case_stdout(oracle: &Value, name: &str) -> String {
    oracle["cases"]
        .as_array()
        .expect("`cases` array")
        .iter()
        .find(|c| c["name"] == name)
        .unwrap_or_else(|| panic!("oracle case {name:?} not found"))["expected_stdout"]
        .as_str()
        .expect("expected_stdout is a string")
        .to_string()
}

/// Spawn the Rust `kb search` binary over the committed sidecar-free
/// corpus, returning `(stdout, exit_code)`.
fn run_kb_search(query: &str, json: bool, top_k: Option<i64>) -> (String, i32) {
    let mut cmd = Command::new(kb_bin());
    cmd.arg("search")
        .arg(query)
        .arg("--summaries")
        .arg(corpus_dir().join("summaries.json"))
        .arg("--source-matrix")
        .arg(corpus_dir().join("source_matrix.json"))
        .env("KB_FROZEN_CLOCK", "1");
    if json {
        cmd.arg("--json");
    }
    if let Some(k) = top_k {
        cmd.arg("--top-k").arg(k.to_string());
    }
    let out = cmd.output().expect("spawn kb search");
    let stdout = String::from_utf8(out.stdout).expect("kb search stdout is UTF-8");
    let code = out.status.code().expect("kb search exits with a code");
    (stdout, code)
}

#[test]
fn kb_search_is_byte_equal_with_python_oracle() {
    let oracle = load_oracle();
    let cases = oracle["cases"].as_array().expect("`cases` array");
    assert!(
        cases.len() >= 8,
        "expected >=8 oracle cases (human/json success, zero-result, \
         negative top_k clamp, top_k cap); got {}",
        cases.len()
    );

    let mut failures: Vec<String> = Vec::new();
    for case in cases {
        let name = case["name"].as_str().unwrap_or("<unnamed>");
        let query = case["query"].as_str().expect("query");
        let json = case["json"].as_bool().expect("json flag");
        // `top_k` is `null` for the default-cap cases and an integer
        // for the explicit ones; `as_i64()` yields `None` for both
        // `null` and missing, which `run_kb_search` maps to "omit
        // `--top-k`" — exactly what the oracle build script did.
        let top_k = case["top_k"].as_i64();
        let want_stdout = case["expected_stdout"].as_str().expect("expected_stdout");
        let want_exit = case["expected_exit"].as_i64().expect("expected_exit");

        let (got_stdout, got_exit) = run_kb_search(query, json, top_k);
        if got_stdout != want_stdout {
            failures.push(format!(
                "case {name:?} stdout mismatch:\n    rust:   {got_stdout:?}\n    python: {want_stdout:?}"
            ));
        }
        if i64::from(got_exit) != want_exit {
            failures.push(format!(
                "case {name:?} exit mismatch: rust={got_exit} python={want_exit}"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} kb search parity failure(s):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}

#[test]
fn kb_search_missing_summaries_is_cacg_cli_001() {
    // A non-existent `--summaries` path is a CLI-surface error
    // (`CACG-CLI-001`), distinct from a present-but-malformed file.
    let tmp = tempfile::tempdir().expect("tempdir");
    let output = Command::new(kb_bin())
        .arg("search")
        .arg("synthetic")
        .arg("--summaries")
        .arg(tmp.path().join("does-not-exist-summaries.json"))
        .arg("--source-matrix")
        .arg(corpus_dir().join("source_matrix.json"))
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb search");
    assert_eq!(
        output.status.code(),
        Some(1),
        "missing summaries must exit 1; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-CLI-001"),
        "missing summaries must surface CACG-CLI-001; got: {stderr}"
    );
}

#[test]
fn kb_search_malformed_summaries_is_cacg_man_001() {
    // A present-but-malformed `summaries.json` is a manifest error
    // (`CACG-MAN-001`), NOT a CLI-surface error: the path exists, so
    // the failure is downstream of the `is_file()` shape check.
    let output = Command::new(kb_bin())
        .arg("search")
        .arg("synthetic")
        .arg("--summaries")
        .arg(corpus_dir().join("malformed_summaries.json"))
        .arg("--source-matrix")
        .arg(corpus_dir().join("source_matrix.json"))
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb search");
    assert_eq!(
        output.status.code(),
        Some(1),
        "malformed summaries must exit 1; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-MAN-001"),
        "malformed summaries must surface CACG-MAN-001; got: {stderr}"
    );
}

#[test]
fn kb_search_malformed_source_matrix_is_cacg_auth_000() {
    // A present-but-malformed `--source-matrix` routes through the
    // existing authorization diagnostic path (`CACG-AUTH-000`).
    let tmp = tempfile::tempdir().expect("tempdir");
    let bad_matrix = tmp.path().join("source_matrix.json");
    fs::write(&bad_matrix, "{ not valid json\n").expect("write malformed source_matrix");
    let output = Command::new(kb_bin())
        .arg("search")
        .arg("synthetic")
        .arg("--summaries")
        .arg(corpus_dir().join("summaries.json"))
        .arg("--source-matrix")
        .arg(&bad_matrix)
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb search");
    assert_eq!(
        output.status.code(),
        Some(1),
        "malformed source-matrix must exit 1; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-AUTH-000"),
        "malformed source-matrix must route through the auth diagnostic path \
         (CACG-AUTH-000); got: {stderr}"
    );
}

#[test]
fn kb_search_absent_cards_manifest_degrades_to_empty_retraction() {
    // No sibling `cards_manifest.json`: the retraction set degrades to
    // empty and the search still succeeds. The committed corpus's own
    // `cards_manifest.json` has empty retraction lists, so an absent
    // manifest must produce byte-identical output to the
    // `success_human` oracle case.
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::copy(
        corpus_dir().join("summaries.json"),
        tmp.path().join("summaries.json"),
    )
    .expect("copy summaries.json into sibling-free tempdir");
    // Deliberately do NOT copy `cards_manifest.json` into `tmp`.
    let output = Command::new(kb_bin())
        .arg("search")
        .arg("synthetic")
        .arg("--summaries")
        .arg(tmp.path().join("summaries.json"))
        .arg("--source-matrix")
        .arg(corpus_dir().join("source_matrix.json"))
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb search");
    assert_eq!(
        output.status.code(),
        Some(0),
        "absent cards_manifest.json must be tolerated (exit 0); stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("kb search stdout is UTF-8");
    let want = oracle_case_stdout(&load_oracle(), "success_human");
    assert_eq!(
        stdout, want,
        "absent cards_manifest.json must yield the same hits as the committed \
         corpus (whose cards_manifest.json carries empty retraction lists)"
    );
}

#[test]
fn kb_search_malformed_cards_manifest_fails_closed_with_cacg_man_001() {
    // A present-but-malformed sibling `cards_manifest.json` fails
    // closed (`CACG-MAN-001`) rather than silently degrading to an
    // empty retraction set — a broken retraction filter must never
    // be treated as "no retractions".
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::copy(
        corpus_dir().join("summaries.json"),
        tmp.path().join("summaries.json"),
    )
    .expect("copy summaries.json into tempdir");
    fs::write(tmp.path().join("cards_manifest.json"), "{ broken json\n")
        .expect("write malformed cards_manifest.json");
    let output = Command::new(kb_bin())
        .arg("search")
        .arg("synthetic")
        .arg("--summaries")
        .arg(tmp.path().join("summaries.json"))
        .arg("--source-matrix")
        .arg(corpus_dir().join("source_matrix.json"))
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb search");
    assert_eq!(
        output.status.code(),
        Some(1),
        "malformed cards_manifest.json must fail closed (exit 1); stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("CACG-MAN-001"),
        "malformed cards_manifest.json must surface CACG-MAN-001; got: {stderr}"
    );
}

#[test]
fn kb_search_directory_cards_manifest_degrades_to_empty_retraction() {
    // A sibling `cards_manifest.json` that is a DIRECTORY (not a
    // regular file) is treated exactly like an absent manifest: the
    // retraction set degrades to empty and the search succeeds. This
    // mirrors Python `_cmd_search`, which gates the retraction block on
    // `cards_manifest_path.is_file()` (`legacy_python_oracle/src/cacg/cli.py:1645`) —
    // `is_file()` is false for a directory, so Python skips the block
    // and leaves the retraction union empty. A non-file sibling
    // manifest is therefore NOT a `CACG-MAN-001` fail-closed path; only
    // a present, malformed *regular file* fails closed (see
    // `kb_search_malformed_cards_manifest_fails_closed_with_cacg_man_001`).
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::copy(
        corpus_dir().join("summaries.json"),
        tmp.path().join("summaries.json"),
    )
    .expect("copy summaries.json into tempdir");
    // The sibling `cards_manifest.json` is a directory, not a file.
    fs::create_dir(tmp.path().join("cards_manifest.json"))
        .expect("create directory at the cards_manifest.json path");
    let output = Command::new(kb_bin())
        .arg("search")
        .arg("synthetic")
        .arg("--summaries")
        .arg(tmp.path().join("summaries.json"))
        .arg("--source-matrix")
        .arg(corpus_dir().join("source_matrix.json"))
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb search");
    assert_eq!(
        output.status.code(),
        Some(0),
        "a directory at the cards_manifest.json path must be tolerated like an \
         absent manifest (exit 0); stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("kb search stdout is UTF-8");
    let want = oracle_case_stdout(&load_oracle(), "success_human");
    assert_eq!(
        stdout, want,
        "a non-file sibling cards_manifest.json must yield the same hits as the \
         committed corpus (the retraction set degrades to empty)"
    );
}
