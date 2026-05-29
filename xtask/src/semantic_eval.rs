//! Layer-3 semantic-eval gate (`xtask semantic-eval`).
//!
//! Runs the committed eval cases (default
//! `tests/semantic_eval/eval_cases.json`) through the QM B1
//! semantic cache and asserts each case's `expected_verdict`
//! matches the cache's actual lookup verdict. The gate is a
//! Layer-3 regression guard: it fails (non-zero exit) when a
//! committed `(chunk_hash, claim_window_hash)` pair's verdict
//! drifts away from its frozen label.
//!
//! Schema isolation: the fixture schema lives at its own
//! `schema_version` literal (`cacg.v0.semantic-eval1`) and uses
//! `#[serde(deny_unknown_fields)]` so retrieval-eval-shaped
//! entries (with `query`, `k`, `expected_card_ids`) fail to
//! parse. The sibling retrieval-eval module enforces the
//! symmetric guard so neither suite's fixture can quietly drift
//! into the other's shape.
//!
//! The check is a direct cache lookup, never a fuzzy comparison
//! — so a float score-presentation change neither masks nor
//! triggers a regression; only a genuine verdict change in the
//! committed cache fails the gate. The cache load is fully
//! deterministic (the same `SemanticCache::load` used by the
//! runtime `kb verify --semantic` path), so the eval reads no
//! clock and is reproducible.

use std::path::Path;

use anyhow::{bail, Context, Result};
use cacg_core::verify::SemanticVerdictKind;
use cacg_semantic::SemanticCache;
use serde::Deserialize;

/// Eval-suite schema version this runner understands. Distinct
/// from the retrieval-eval literal (`cacg.v0.eval1`) so a
/// retrieval-eval fixture cannot pass this parser.
const EVAL_SCHEMA_VERSION: &str = "cacg.v0.semantic-eval1";

/// The committed eval-suite fixture. Strict-shape deserializer:
/// any unknown field fails parsing, which closes the
/// schema-separation negative gate against retrieval-eval-
/// shaped entries.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EvalSuite {
    /// Fixture schema version; must equal [`EVAL_SCHEMA_VERSION`].
    schema_version: String,
    /// Human-readable description (informational; not validated
    /// beyond shape).
    #[allow(dead_code)]
    description: String,
    /// Workspace-relative path to the QM B1 `semantic_cache.json`
    /// the eval runs against.
    cache: String,
    /// The eval cases, run in order.
    cases: Vec<EvalCase>,
}

/// One eval case: a (chunk_hash, claim_window_hash) pair plus
/// the expected verdict.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EvalCase {
    /// Stable case name, printed in the report.
    name: String,
    /// 64-hex SHA-256 of the chunk envelope (the cache key's
    /// first component).
    chunk_hash: String,
    /// 64-hex SHA-256 of the normalized claim window (the cache
    /// key's second component).
    claim_window_hash: String,
    /// Expected verdict for this lookup. Must be one of
    /// `"pass"`, `"fail"`, or `"abstain"`. Any other value fails
    /// parsing (the `ExpectedVerdict` enum is `deny_unknown_fields`
    /// by virtue of being a closed enum).
    expected_verdict: ExpectedVerdict,
}

/// The three verdict outcomes a cache lookup can yield. Mirrors
/// `cacg_core::verify::SemanticVerdictKind` but is owned by the
/// eval module so the fixture format stays decoupled from the
/// runtime enum's serde representation.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum ExpectedVerdict {
    Pass,
    Fail,
    Abstain,
}

impl ExpectedVerdict {
    fn matches(self, actual: SemanticVerdictKind) -> bool {
        matches!(
            (self, actual),
            (Self::Pass, SemanticVerdictKind::Pass)
                | (Self::Fail, SemanticVerdictKind::Fail)
                | (Self::Abstain, SemanticVerdictKind::Abstain)
        )
    }

    fn label(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Abstain => "abstain",
        }
    }
}

fn actual_label(kind: SemanticVerdictKind) -> &'static str {
    match kind {
        SemanticVerdictKind::Pass => "pass",
        SemanticVerdictKind::Fail => "fail",
        SemanticVerdictKind::Abstain => "abstain",
    }
}

/// Run the eval suite at `fixtures`. Returns `Ok(true)` when
/// every case's expected verdict matches the cache's actual
/// lookup verdict, and `Ok(false)` when one or more cases
/// mismatch (the caller maps that to a non-zero exit).
///
/// # Errors
///
/// Returns an error when the fixture file cannot be read or
/// parsed, the schema version is unrecognized, or the
/// referenced cache cannot be loaded.
pub fn run(fixtures: &Path) -> Result<bool> {
    let raw = std::fs::read_to_string(fixtures)
        .with_context(|| format!("cannot read eval fixtures {}", fixtures.display()))?;
    let suite: EvalSuite = serde_json::from_str(&raw)
        .with_context(|| format!("cannot parse eval fixtures {}", fixtures.display()))?;
    if suite.schema_version != EVAL_SCHEMA_VERSION {
        bail!(
            "eval fixtures schema_version {:?} unrecognized (expected {EVAL_SCHEMA_VERSION:?})",
            suite.schema_version
        );
    }
    if suite.cases.is_empty() {
        bail!("eval fixtures define no cases");
    }

    let cache_path = Path::new(&suite.cache);
    let cache = SemanticCache::load(cache_path)
        .with_context(|| format!("cannot load semantic cache {}", cache_path.display()))?;

    let mut passed = 0usize;
    let total = suite.cases.len();
    let mut all_ok = true;

    for case in &suite.cases {
        let verdict = cache.lookup(&case.chunk_hash, &case.claim_window_hash);
        let matched = case.expected_verdict.matches(verdict.kind);
        let expected = case.expected_verdict.label();
        let actual = actual_label(verdict.kind);
        eprintln!(
            "{}",
            format_case_line(&case.name, expected, actual, matched)
        );
        if matched {
            passed += 1;
        } else {
            all_ok = false;
        }
    }
    eprintln!("xtask semantic-eval: {passed}/{total} cases match expected verdict");
    Ok(all_ok)
}

/// Format one per-case report line for the eval runner.
///
/// Always emits both `expected=<v>` and `actual=<v>` labels so
/// the report is self-describing on both matching and
/// mismatching rows. The `PASS` / `FAIL` marker carries the
/// match status; `expected == actual` on a matching row,
/// `expected != actual` on a mismatch.
fn format_case_line(name: &str, expected: &str, actual: &str, matched: bool) -> String {
    let marker = if matched { "PASS" } else { "FAIL" };
    format!("  {marker}  {name:<32} expected={expected} actual={actual}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_accepts_minimal_semantic_eval() {
        let json = r#"{
            "schema_version": "cacg.v0.semantic-eval1",
            "description": "minimal",
            "cache": "out/semantic_cache.json",
            "cases": [
                {
                    "name": "case_a",
                    "chunk_hash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    "claim_window_hash": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
                    "expected_verdict": "pass"
                }
            ]
        }"#;
        let suite: EvalSuite = serde_json::from_str(json).expect("parse minimal");
        assert_eq!(suite.schema_version, EVAL_SCHEMA_VERSION);
        assert_eq!(suite.cases.len(), 1);
        assert_eq!(suite.cases[0].expected_verdict, ExpectedVerdict::Pass);
    }

    #[test]
    fn parsing_rejects_retrieval_eval_shape() {
        // Retrieval-eval shape: cases have `query`, `k`,
        // `expected_card_ids`. With `deny_unknown_fields`, those
        // unknown fields must cause parsing to fail.
        let json = r#"{
            "schema_version": "cacg.v0.semantic-eval1",
            "description": "retrieval-eval shape under semantic-eval schema",
            "cache": "out/semantic_cache.json",
            "cases": [
                {
                    "name": "title_match_intrinsic",
                    "match_field": "title",
                    "synthetic": false,
                    "summaries": "tests/parity_corpus/cfa_first_bite/summaries.json",
                    "source_matrix": "tests/parity_corpus/cfa_first_bite/source_matrix.json",
                    "query": "intrinsic",
                    "k": 3,
                    "expected_card_ids": ["intrinsic-valuation-discounted-cash-flows"]
                }
            ]
        }"#;
        let err = serde_json::from_str::<EvalSuite>(json).unwrap_err();
        assert!(
            err.to_string().contains("unknown field"),
            "retrieval-eval-shaped case must fail with `unknown field`; got: {err}",
        );
    }

    #[test]
    fn parsing_rejects_unknown_verdict() {
        let json = r#"{
            "schema_version": "cacg.v0.semantic-eval1",
            "description": "unknown verdict",
            "cache": "out/semantic_cache.json",
            "cases": [
                {
                    "name": "garbage",
                    "chunk_hash": "0000000000000000000000000000000000000000000000000000000000000001",
                    "claim_window_hash": "0000000000000000000000000000000000000000000000000000000000000002",
                    "expected_verdict": "garbage"
                }
            ]
        }"#;
        let err = serde_json::from_str::<EvalSuite>(json).unwrap_err();
        assert!(
            err.to_string().contains("unknown variant") || err.to_string().contains("garbage"),
            "unknown verdict must fail parsing; got: {err}",
        );
    }

    #[test]
    fn run_rejects_wrong_schema_version() {
        // Use the retrieval-eval schema version literal to prove
        // a retrieval-eval fixture file cannot accidentally be
        // routed through this gate.
        let tmp = tempfile::TempDir::new().unwrap();
        let path = tmp.path().join("wrong-schema.json");
        let json = r#"{
            "schema_version": "cacg.v0.eval1",
            "description": "wrong schema version",
            "cache": "out/semantic_cache.json",
            "cases": [
                {
                    "name": "x",
                    "chunk_hash": "0000000000000000000000000000000000000000000000000000000000000001",
                    "claim_window_hash": "0000000000000000000000000000000000000000000000000000000000000002",
                    "expected_verdict": "abstain"
                }
            ]
        }"#;
        std::fs::write(&path, json).unwrap();
        let err = run(&path).unwrap_err();
        assert!(
            format!("{err:#}").contains("schema_version")
                && format!("{err:#}").contains("unrecognized"),
            "wrong schema_version must surface in the error: {err:#}",
        );
    }

    #[test]
    fn format_case_line_includes_expected_and_actual_on_match() {
        let line = format_case_line("case_a", "pass", "pass", true);
        assert!(
            line.contains("PASS"),
            "match marker should be PASS; got: {line}"
        );
        assert!(line.contains("case_a"));
        assert!(
            line.contains("expected=pass"),
            "missing expected= label: {line}"
        );
        assert!(
            line.contains("actual=pass"),
            "missing actual= label: {line}"
        );
    }

    #[test]
    fn format_case_line_includes_expected_and_actual_on_mismatch() {
        let line = format_case_line("case_b", "pass", "fail", false);
        assert!(
            line.contains("FAIL"),
            "mismatch marker should be FAIL; got: {line}"
        );
        assert!(line.contains("case_b"));
        assert!(
            line.contains("expected=pass"),
            "missing expected= label: {line}"
        );
        assert!(
            line.contains("actual=fail"),
            "missing actual= label: {line}"
        );
    }

    #[test]
    fn run_executes_against_a_synthetic_fixture_with_absolute_cache_path() {
        // End-to-end smoke: synthesize a fixture whose `cache`
        // field points at the committed workspace cache via an
        // ABSOLUTE path (avoids any cwd dependency). The case
        // uses an all-zero synthetic 64-hex pair that is NOT a
        // key in the cache, so the cache lookup returns the
        // abstain-embedding-cache sentinel — matching the
        // case's `expected_verdict: "abstain"`.
        let mut workspace_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        workspace_root.pop();
        let cache_path = workspace_root.join("out/semantic_cache.json");
        if !cache_path.is_file() {
            // Cache not committed in this workspace state. The
            // live `cargo xtask semantic-eval` runs cover the
            // happy path; skip cleanly.
            eprintln!(
                "committed cache not present at {}; smoke test skipped",
                cache_path.display(),
            );
            return;
        }
        let json = format!(
            r#"{{
                "schema_version": "cacg.v0.semantic-eval1",
                "description": "synthetic smoke",
                "cache": "{}",
                "cases": [
                    {{
                        "name": "abstain_synthetic_key",
                        "chunk_hash": "0000000000000000000000000000000000000000000000000000000000000001",
                        "claim_window_hash": "0000000000000000000000000000000000000000000000000000000000000002",
                        "expected_verdict": "abstain"
                    }}
                ]
            }}"#,
            cache_path.display(),
        );
        let tmp = tempfile::TempDir::new().unwrap();
        let path = tmp.path().join("synthetic-semantic-eval.json");
        std::fs::write(&path, json).unwrap();
        let ok = run(&path).expect("synthetic fixture must parse and run");
        assert!(
            ok,
            "synthetic abstain case must match (the chunk_hash isn't in the cache)",
        );
    }
}
