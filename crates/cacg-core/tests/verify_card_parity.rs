#![allow(clippy::unwrap_used)]
//! Layer-2 `verify_card` parity gate.
//!
//! Reads `tests/parity_corpus/verify_card_layer2/oracle.json`
//! (generated deterministically by
//! `legacy_python_oracle/scripts/build_verify_card_layer2_oracle.py`) and, for every
//! curated scenario, reconstructs the Rust inputs, runs Rust
//! `cacg_core::verify::layer2::verify_card`, and compares against the
//! Python output recorded in the oracle.
//!
//! Per-entry assertions (per the HYBRID parity policy in
//! `docs/diagnostic-parity.md`):
//!
//! * `failed` and `used_fuzzy` match Python.
//! * Number of diagnostics matches.
//! * Per diagnostic: `code`, `severity`, and `file` are byte-equal
//!   with Python.
//! * For diagnostics whose code is `CACG-VERIFY-001`, the full
//!   `message` text is byte-equal with Python (the principal hard-
//!   failure code carries the page-window / quote-absent narrative
//!   the consumer surfaces verbatim).
//!
//! Additional smoke checks: every committed valid card under
//! `tests/parity_corpus/valid/*.md` verifies clean against
//! `tests/parity_corpus/out_python/chunks_manifest.json` under both
//! exact and fuzzy modes. These are NOT the trust-bearing parity
//! gate — they catch regressions on the happy path while the
//! oracle-driven gate above pins per-code behavior.

use std::path::PathBuf;

use cacg_core::card_loader::{load_card, CardDoc};
use cacg_core::chunks_index::ChunksIndex;
use cacg_core::diagnostic::{Diagnostic, Severity};
use cacg_core::schema::{CardFrontmatter, ChunksManifest, SourceMatrix};
use cacg_core::source_matrix::AuthSpec;
use cacg_core::verify::layer2::verify_card;
use serde::Deserialize;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Deserialize)]
struct OracleEntry {
    index: u32,
    name: String,
    card_frontmatter: CardFrontmatter,
    chunks_manifest: ChunksManifest,
    #[serde(default)]
    source_matrix: Option<SourceMatrix>,
    fuzzy_enabled: bool,
    auth_enabled: bool,
    #[serde(default)]
    auth_load_error: Option<String>,
    file_path: String,
    python_failed: bool,
    python_used_fuzzy: bool,
    python_diagnostics: Vec<OracleDiagnostic>,
}

#[derive(Debug, Deserialize)]
struct OracleDiagnostic {
    code: String,
    severity: String,
    file: Option<String>,
    message: String,
    /// BM25 "did you mean" hint dictionaries Python attaches to a
    /// `CACG-VERIFY-001` failure; empty for every other code.
    #[serde(default)]
    hints: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct OracleEnvelope {
    case_count: u32,
    entries: Vec<OracleEntry>,
}

fn severity_str(s: Severity) -> &'static str {
    match s {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "info",
    }
}

fn auth_spec_for(entry: &OracleEntry) -> Option<AuthSpec> {
    if !entry.auth_enabled {
        return None;
    }
    Some(AuthSpec {
        matrix: entry.source_matrix.clone(),
        load_error: entry.auth_load_error.clone(),
    })
}

#[test]
fn rust_matches_python_on_every_oracle_case() {
    let path = workspace_root().join("tests/parity_corpus/verify_card_layer2/oracle.json");
    let raw =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let envelope: OracleEnvelope =
        serde_json::from_str(&raw).expect("oracle JSON must parse against OracleEnvelope");
    assert_eq!(
        envelope.case_count as usize,
        envelope.entries.len(),
        "case_count must equal entries.len()"
    );
    assert!(
        envelope.entries.len() >= 15,
        "oracle must carry >=15 curated scenarios; got {}",
        envelope.entries.len()
    );

    let mut failures: Vec<String> = Vec::new();
    for entry in &envelope.entries {
        let index = ChunksIndex::from_manifest(entry.chunks_manifest.clone())
            .expect("oracle chunks_manifest is well-formed by construction");
        let doc = CardDoc {
            path: PathBuf::from(&entry.file_path),
            frontmatter: entry.card_frontmatter.clone(),
            body_raw: String::new(),
            body_normalized: String::new(),
        };
        let auth = auth_spec_for(entry);
        let out = verify_card(
            &doc,
            &index,
            entry.fuzzy_enabled,
            Some(entry.file_path.as_str()),
            auth.as_ref(),
            None,
            None,
        );
        let mut entry_failures: Vec<String> = Vec::new();
        if out.failed != entry.python_failed {
            entry_failures.push(format!(
                "failed: rust={} python={}",
                out.failed, entry.python_failed
            ));
        }
        if out.used_fuzzy != entry.python_used_fuzzy {
            entry_failures.push(format!(
                "used_fuzzy: rust={} python={}",
                out.used_fuzzy, entry.python_used_fuzzy
            ));
        }
        if out.diagnostics.len() == entry.python_diagnostics.len() {
            for (i, (rs, py)) in out
                .diagnostics
                .iter()
                .zip(entry.python_diagnostics.iter())
                .enumerate()
            {
                compare_diagnostic(i, rs, py, &mut entry_failures);
            }
        } else {
            entry_failures.push(format!(
                "diagnostics.len(): rust={} python={}; rust_codes={:?}, python_codes={:?}",
                out.diagnostics.len(),
                entry.python_diagnostics.len(),
                out.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>(),
                entry
                    .python_diagnostics
                    .iter()
                    .map(|d| &d.code)
                    .collect::<Vec<_>>(),
            ));
        }
        if !entry_failures.is_empty() {
            failures.push(format!(
                "case {} ({}):\n    {}",
                entry.index,
                entry.name,
                entry_failures.join("\n    "),
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "verify_card diverges from Python on {} case(s):\n  {}",
        failures.len(),
        failures.join("\n  "),
    );
}

fn compare_diagnostic(
    i: usize,
    rs: &Diagnostic,
    py: &OracleDiagnostic,
    failures: &mut Vec<String>,
) {
    let rs_severity = severity_str(rs.severity);
    if rs.code != py.code {
        failures.push(format!(
            "diagnostics[{i}].code: rust={:?} python={:?}",
            rs.code, py.code
        ));
    }
    if rs_severity != py.severity {
        failures.push(format!(
            "diagnostics[{i}].severity: rust={:?} python={:?}",
            rs_severity, py.severity
        ));
    }
    if rs.file != py.file {
        failures.push(format!(
            "diagnostics[{i}].file: rust={:?} python={:?}",
            rs.file, py.file
        ));
    }
    if rs.code == "CACG-VERIFY-001" && rs.message != py.message {
        failures.push(format!(
            "diagnostics[{i}].message (VERIFY-001 byte-equal contract):\n      \
             rust  = {:?}\n      python= {:?}",
            rs.message, py.message
        ));
    }
    // The BM25 "did you mean" hints riding on a `CACG-VERIFY-001`
    // failure MUST be byte-equal with Python. The former empty-hints
    // carve-out is closed; every diagnostic's `hints` array (empty for
    // non-VERIFY-001 codes) is compared.
    compare_hints(i, &rs.hints, &py.hints, failures);
}

/// Compare the BM25 hint payloads attached to one diagnostic.
///
/// `hint_only`, `chunk_id`, and `text_preview` are compared exactly;
/// the 6-decimal-rounded `score` is compared via its fixed-precision
/// rendering so a last-ULP cross-language difference cannot flake.
fn compare_hints(
    i: usize,
    rs: &[serde_json::Value],
    py: &[serde_json::Value],
    failures: &mut Vec<String>,
) {
    if rs.len() != py.len() {
        failures.push(format!(
            "diagnostics[{i}].hints: count rust={} python={}",
            rs.len(),
            py.len()
        ));
        return;
    }
    for (j, (r, p)) in rs.iter().zip(py).enumerate() {
        for key in ["hint_only", "chunk_id", "text_preview"] {
            if r.get(key) != p.get(key) {
                failures.push(format!(
                    "diagnostics[{i}].hints[{j}].{key}: rust={:?} python={:?}",
                    r.get(key),
                    p.get(key)
                ));
            }
        }
        match (
            r.get("score").and_then(serde_json::Value::as_f64),
            p.get("score").and_then(serde_json::Value::as_f64),
        ) {
            (Some(rs_score), Some(py_score)) => {
                if format!("{rs_score:.6}") != format!("{py_score:.6}") {
                    failures.push(format!(
                        "diagnostics[{i}].hints[{j}].score: rust={rs_score:.6} python={py_score:.6}"
                    ));
                }
            }
            _ => failures.push(format!(
                "diagnostics[{i}].hints[{j}].score: not a number on one side"
            )),
        }
    }
}

#[test]
fn oracle_covers_two_distinct_verify_001_branches() {
    // Pin that the curated oracle exercises BOTH `CACG-VERIFY-001`
    // message branches the Layer-2 surface emits:
    //
    //   1. "quote not found in pinned chunk ..." (plain absent
    //      branch).
    //   2. "quote IS in pinned chunk ... but NOT within cited
    //      page_range ..." (page-window branch).
    //
    // The branches diverge in the failure-path message in
    // `verify_citation`, and the byte-equal hints comparison in
    // `compare_diagnostic` runs on every VERIFY-001 case the
    // oracle carries. Without this fixture-shape pin, a future
    // edit could collapse the oracle to only one branch and the
    // hints comparison would silently lose coverage on the
    // other.
    let path = workspace_root().join("tests/parity_corpus/verify_card_layer2/oracle.json");
    let raw = std::fs::read_to_string(&path).unwrap();
    let envelope: OracleEnvelope = serde_json::from_str(&raw).unwrap();
    let mut has_not_found = false;
    let mut has_page_window = false;
    for entry in &envelope.entries {
        for d in &entry.python_diagnostics {
            if d.code != "CACG-VERIFY-001" {
                continue;
            }
            if d.message.contains("quote not found in pinned chunk") {
                has_not_found = true;
            }
            if d.message.contains("quote IS in pinned chunk")
                && d.message.contains("NOT within cited page_range")
            {
                has_page_window = true;
            }
        }
    }
    assert!(
        has_not_found,
        "oracle fixture is missing the plain VERIFY-001 \"quote not found in pinned chunk\" branch"
    );
    assert!(
        has_page_window,
        "oracle fixture is missing the VERIFY-001 page-window \"quote IS in pinned chunk ... NOT within cited page_range\" branch"
    );
}

#[test]
fn oracle_covers_every_documented_diagnostic_code() {
    // Pin that the curated oracle exercises every CACG-* code the
    // Layer-2 surface is expected to emit. Future fixture edits
    // that drop a category surface here.
    let path = workspace_root().join("tests/parity_corpus/verify_card_layer2/oracle.json");
    let raw = std::fs::read_to_string(&path).unwrap();
    let envelope: OracleEnvelope = serde_json::from_str(&raw).unwrap();
    let mut codes_seen: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for entry in &envelope.entries {
        for d in &entry.python_diagnostics {
            codes_seen.insert(d.code.clone());
        }
    }
    let required = [
        "CACG-HASH-001",
        "CACG-CITE-006",
        "CACG-HASH-003",
        "CACG-RETR-002",
        "CACG-RETR-003",
        "CACG-AUTH-000",
        "CACG-AUTH-001",
        "CACG-AUTH-002",
        "CACG-VERIFY-001",
    ];
    let missing: Vec<&str> = required
        .iter()
        .filter(|c| !codes_seen.contains(**c))
        .copied()
        .collect();
    assert!(
        missing.is_empty(),
        "oracle fixture is missing curated coverage for code(s): {missing:?}; \
         every Layer-2 trust-bearing code MUST appear in at least one case"
    );
}

// Additional smoke checks against the committed valid corpus:
// these are NOT the trust-bearing parity gate (the oracle-driven
// test above is); they catch regressions on the happy path.

fn valid_card_paths() -> Vec<PathBuf> {
    let root = workspace_root().join("tests/parity_corpus/valid");
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("read_dir {}: {e}", root.display()))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "md"))
        .collect();
    paths.sort();
    paths
}

fn committed_index() -> ChunksIndex {
    let manifest_path =
        workspace_root().join("tests/parity_corpus/out_python/chunks_manifest.json");
    ChunksIndex::from_path(&manifest_path)
        .unwrap_or_else(|e| panic!("load chunks_manifest from {}: {e}", manifest_path.display()))
}

#[test]
fn smoke_every_valid_card_verifies_clean_under_exact_mode() {
    let index = committed_index();
    let paths = valid_card_paths();
    assert!(
        !paths.is_empty(),
        "tests/parity_corpus/valid must contain at least one .md card"
    );
    let mut failures: Vec<String> = Vec::new();
    for path in &paths {
        let doc = load_card(path).unwrap_or_else(|e| panic!("load_card {}: {e}", path.display()));
        let file_path = path.display().to_string();
        let out = verify_card(&doc, &index, false, Some(&file_path), None, None, None);
        if out.failed || !out.diagnostics.is_empty() || out.used_fuzzy {
            failures.push(format!(
                "{}: failed={} diagnostics={} used_fuzzy={}\n  codes: {:?}",
                path.display(),
                out.failed,
                out.diagnostics.len(),
                out.used_fuzzy,
                out.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>(),
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "valid corpus failed exact-mode verify on {} card(s):\n  {}",
        failures.len(),
        failures.join("\n  "),
    );
}

#[test]
fn smoke_every_valid_card_verifies_clean_under_fuzzy_mode() {
    let index = committed_index();
    let paths = valid_card_paths();
    let mut failures: Vec<String> = Vec::new();
    for path in &paths {
        let doc = load_card(path).unwrap_or_else(|e| panic!("load_card {}: {e}", path.display()));
        let file_path = path.display().to_string();
        let out = verify_card(&doc, &index, true, Some(&file_path), None, None, None);
        if out.failed || !out.diagnostics.is_empty() {
            failures.push(format!(
                "{}: failed={} diagnostics={}\n  codes: {:?}",
                path.display(),
                out.failed,
                out.diagnostics.len(),
                out.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>(),
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "valid corpus failed fuzzy-mode verify on {} card(s):\n  {}",
        failures.len(),
        failures.join("\n  "),
    );
}
