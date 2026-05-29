#![allow(
    clippy::map_unwrap_or,
    clippy::wildcard_enum_match_arm,
    clippy::unwrap_used,
    clippy::unnecessary_map_or
)]
//! Golden-fixture parity test for the `kb verify --round-summary`
//! dispatcher.
//!
//! Each markdown fixture under `tests/round_summary_fixtures/*.md` is
//! paired with a `*.expected.json` produced by:
//!
//! 1. running the Python `cacg.cli.verify --round-summary` dispatcher
//!    and capturing its stdout / stderr / `exit_code`, AND
//! 2. computing the lowercase hex SHA-256 of the paired markdown
//!    bytes (the `fixture_sha256` field).
//!
//! This test spawns the native Rust `kb` binary against each fixture
//! and asserts two regression gates in order:
//!
//! * POSITIVE — Rust stdout / stderr / `exit_code` byte-equal the
//!   Python dispatcher's recorded output.
//! * NEGATIVE — the SHA-256 of the fixture bytes equals the
//!   recorded `fixture_sha256`. ANY edit to the fixture markdown
//!   body (even prose changes that leave dispatcher output
//!   unchanged) makes the fingerprint check FAIL before the test
//!   ever spawns `kb`, satisfying the regression-gate contract that
//!   a fixture-body edit without a regenerated expected JSON must
//!   fail loudly.
//!
//! Regenerate the expected files by running the Python dispatcher
//! against the workspace and hashing the paired fixture bytes; see
//! the equivalent Python script logic recorded in the Round-4 loop
//! state.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use cacg_core::hash::source_sha256;
use serde_json::Value;
use tempfile::TempDir;

#[derive(Debug)]
struct Expected {
    stdout: String,
    stderr: String,
    exit_code: i64,
    fixture_sha256: String,
}

fn parse_expected(raw: &str, where_path: &str) -> Expected {
    let v: Value =
        serde_json::from_str(raw).unwrap_or_else(|e| panic!("parse {where_path} as JSON: {e}"));
    let obj = v.as_object().unwrap_or_else(|| {
        panic!("{where_path}: expected a JSON object; regenerate expected.json")
    });
    let stdout = obj
        .get("stdout")
        .and_then(Value::as_str)
        .unwrap_or_else(|| {
            panic!("{where_path}: missing string field `stdout`; regenerate expected.json")
        })
        .to_string();
    let stderr = obj
        .get("stderr")
        .and_then(Value::as_str)
        .unwrap_or_else(|| {
            panic!("{where_path}: missing string field `stderr`; regenerate expected.json")
        })
        .to_string();
    let exit_code = obj
        .get("exit_code")
        .and_then(Value::as_i64)
        .unwrap_or_else(|| {
            panic!("{where_path}: missing integer field `exit_code`; regenerate expected.json")
        });
    let fixture_sha256 = obj
        .get("fixture_sha256")
        .and_then(Value::as_str)
        .unwrap_or_else(|| {
            panic!(
                "{where_path}: missing string field `fixture_sha256`; regenerate expected.json with the four-field envelope {{exit_code, fixture_sha256, stderr, stdout}}",
            )
        })
        .to_string();
    Expected {
        stdout,
        stderr,
        exit_code,
        fixture_sha256,
    }
}

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

fn fixtures_dir() -> PathBuf {
    workspace_root().join("tests/round_summary_fixtures")
}

fn fixture_paths(name: &str) -> (PathBuf, PathBuf) {
    (
        fixtures_dir().join(format!("{name}.md")),
        fixtures_dir().join(format!("{name}.expected.json")),
    )
}

fn run_fixture(name: &str) -> (String, String, i64) {
    let workspace = workspace_root();
    let (fixture, _) = fixture_paths(name);
    let tmp = TempDir::new().expect("tempdir");
    let journal = tmp.path().join("lint_journal.jsonl");
    let output = Command::new(kb_bin())
        .current_dir(&workspace)
        .arg("verify")
        .arg("--round-summary")
        .arg(&fixture)
        .arg("--chunks-manifest")
        .arg("tests/parity_corpus/out_python/chunks_manifest.json")
        .arg("--source-matrix")
        .arg("tests/parity_corpus/out_python/source_matrix.json")
        .arg("--journal")
        .arg(&journal)
        .output()
        .expect("spawn kb verify --round-summary");
    let stdout = String::from_utf8(output.stdout).expect("kb stdout is utf-8");
    let stderr = String::from_utf8(output.stderr).expect("kb stderr is utf-8");
    let code = i64::from(output.status.code().unwrap_or(-1));
    (stdout, stderr, code)
}

fn load_expected(name: &str) -> Expected {
    let (_, expected_path) = fixture_paths(name);
    let raw = fs::read_to_string(&expected_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", expected_path.display()));
    parse_expected(&raw, &expected_path.display().to_string())
}

fn assert_parity(name: &str) {
    let (fixture, _) = fixture_paths(name);
    assert!(
        fixture.is_file(),
        "fixture not found: {}",
        fixture.display(),
    );
    let expected = load_expected(name);

    // Negative gate: the fixture markdown bytes must hash-match the
    // recorded `fixture_sha256`. This runs BEFORE the dispatcher is
    // spawned so any edit to fixture prose - even prose that leaves
    // dispatcher output unchanged - fails loudly without polluting
    // the failure message with output diffs.
    let bytes =
        fs::read(&fixture).unwrap_or_else(|e| panic!("read fixture {}: {e}", fixture.display()));
    let actual_sha256 = source_sha256(&bytes);
    assert_eq!(
        actual_sha256,
        expected.fixture_sha256,
        "fixture markdown body changed on {name}; regenerate \
         tests/round_summary_fixtures/{name}.expected.json against \
         the Python `kb verify --round-summary` dispatcher \
         (expected SHA-256 {expected_sha}, actual {actual_sha256})",
        expected_sha = expected.fixture_sha256,
    );

    // Positive gate: byte-equal Python ↔ Rust dispatcher output.
    let (stdout, stderr, code) = run_fixture(name);
    assert_eq!(
        code, expected.exit_code,
        "exit_code drift on {name}: rust={code} python={}; stdout={stdout:?}; stderr={stderr:?}",
        expected.exit_code,
    );
    assert_eq!(
        stdout, expected.stdout,
        "stdout drift on {name}\n--- rust stdout ---\n{stdout}\n--- python stdout ---\n{}",
        expected.stdout,
    );
    assert_eq!(
        stderr, expected.stderr,
        "stderr drift on {name}\n--- rust stderr ---\n{stderr}\n--- python stderr ---\n{}",
        expected.stderr,
    );
}

#[test]
fn fixture_01_clean_na() {
    assert_parity("01_clean_na");
}

#[test]
fn fixture_02_missing_section_non_kb() {
    assert_parity("02_missing_section_non_kb");
}

#[test]
fn fixture_03_missing_section_kb_relevant() {
    assert_parity("03_missing_section_kb_relevant");
}

#[test]
fn fixture_04_na_on_kb_relevant() {
    assert_parity("04_na_on_kb_relevant");
}

#[test]
fn fixture_05_empty_section_kb_relevant() {
    assert_parity("05_empty_section_kb_relevant");
}

#[test]
fn fixture_06_sentinel_collision() {
    assert_parity("06_sentinel_collision");
}

#[test]
fn fixture_07_verified_single_card() {
    assert_parity("07_verified_single_card");
}

#[test]
fn fixture_08_missing_card() {
    assert_parity("08_missing_card");
}

#[test]
fn fixture_09_multiple_sections() {
    assert_parity("09_multiple_sections");
}

#[test]
fn every_markdown_fixture_has_paired_expected_json() {
    // Negative gate from the regression-gate contract: any new fixture
    // committed under `tests/round_summary_fixtures/*.md` MUST land
    // with its paired `.expected.json`. A drift between the two sets
    // means either a fixture was added without regenerating its
    // expected output, or an expected file was committed without its
    // source fixture.
    let dir = fixtures_dir();
    assert!(dir.is_dir(), "fixtures dir missing: {}", dir.display());
    let mut md_basenames: BTreeSet<String> = BTreeSet::new();
    let mut json_basenames: BTreeSet<String> = BTreeSet::new();
    for entry in fs::read_dir(&dir).expect("readdir") {
        let path = entry.expect("entry").path();
        if !path.is_file() {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        if let Some(stem) = name.strip_suffix(".md") {
            md_basenames.insert(stem.to_string());
        } else if let Some(stem) = name.strip_suffix(".expected.json") {
            json_basenames.insert(stem.to_string());
        }
    }
    let md_without_json: Vec<String> = md_basenames.difference(&json_basenames).cloned().collect();
    let json_without_md: Vec<String> = json_basenames.difference(&md_basenames).cloned().collect();
    assert!(
        md_without_json.is_empty() && json_without_md.is_empty(),
        "fixtures dir drift: md without expected={md_without_json:?}; expected without md={json_without_md:?}",
    );
    assert!(
        md_basenames.len() >= 5 && md_basenames.len() <= 10,
        "regression gate requires 5-10 fixtures; got {} ({:?})",
        md_basenames.len(),
        md_basenames,
    );
}

#[test]
fn parsing_each_expected_json_yields_the_documented_schema() {
    // Defense in depth: confirm each expected JSON file actually
    // deserializes into the Expected struct, catching format drift
    // before it cascades into confusing parity failures.
    let dir = fixtures_dir();
    for entry in fs::read_dir(&dir).expect("readdir") {
        let path = entry.expect("entry").path();
        if !path
            .file_name()
            .and_then(|n| n.to_str())
            .map_or(false, |n| n.ends_with(".expected.json"))
        {
            continue;
        }
        let raw = fs::read_to_string(&path).unwrap();
        let _parsed = parse_expected(&raw, &path.display().to_string());
    }
}
