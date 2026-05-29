//! Retrieval-quality eval gate (`xtask retrieval-eval`).
//!
//! Runs the committed eval cases (`tests/retrieval_eval/eval_cases.json`)
//! through the in-memory `cacg-search` BM25 backend and asserts each
//! case's `expected_card_ids` land within its configured top-k. The
//! gate is a recall guard: it fails (non-zero exit) when an expected
//! hit drops out of the top-k.
//!
//! The check is card-id MEMBERSHIP within the top-k, never a score
//! comparison — so a float score-presentation change can neither mask
//! nor trigger a regression; only a genuine rank/order change that
//! pushes an expected card past `k` fails the gate. The in-memory
//! backend is fully deterministic, so the eval reads no clock and is
//! reproducible under `KB_FROZEN_CLOCK=1` (and without it).

use std::path::Path;

use anyhow::{bail, Context, Result};
use cacg_core::source_matrix::load_source_matrix;
use cacg_search::SummariesIndex;
use serde::Deserialize;

/// Eval-suite schema version this runner understands.
const EVAL_SCHEMA_VERSION: &str = "cacg.v0.eval1";

/// The committed eval-suite fixture. `deny_unknown_fields`
/// enforces the schema-separation contract: a
/// semantic-eval-shaped fixture (with `cache` at the top level
/// + per-case `chunk_hash` / `claim_window_hash` /
/// `expected_verdict`) must fail to parse here. The `description`
/// field is the only legitimate top-level informational
/// addition this fixture carries; it remains tolerated for
/// parity with the existing committed fixture.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EvalSuite {
    /// Fixture schema version; must equal [`EVAL_SCHEMA_VERSION`].
    schema_version: String,
    /// Optional human-readable description of the suite.
    /// Tolerated for compatibility with the committed fixture's
    /// header comment; the runner does not consume the text.
    #[serde(default)]
    #[allow(dead_code)]
    description: Option<String>,
    /// The eval cases, run in order.
    cases: Vec<EvalCase>,
}

/// One eval case: a query plus the card-ids expected within `k`.
/// `deny_unknown_fields` enforces the schema-separation
/// contract: a semantic-eval-shaped case (with `chunk_hash`,
/// `claim_window_hash`, `expected_verdict`) must fail to parse
/// here.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EvalCase {
    /// Stable case name, printed in the report.
    name: String,
    /// Informational label for the field the query targets
    /// (`title` / `summary` / `tag` / `none`).
    match_field: String,
    /// `true` iff the corpus is synthetic rather than real-source.
    #[serde(default)]
    synthetic: bool,
    /// Workspace-relative path to the `summaries.json` to search.
    summaries: String,
    /// Workspace-relative path to the `source_matrix.json`.
    source_matrix: String,
    /// The `kb search` query.
    query: String,
    /// Top-k cutoff for the expected-hit check.
    k: i64,
    /// Card-ids that MUST appear within the top-k. Empty marks a
    /// zero-result case — the search must then return nothing.
    expected_card_ids: Vec<String>,
}

/// One case's evaluated outcome.
struct CaseOutcome {
    ok: bool,
    expected_found: usize,
    expected_total: usize,
    detail: String,
}

/// Run the eval suite at `fixtures`. Returns `Ok(true)` when every
/// case passes the expected-hit-at-k gate and `Ok(false)` when one or
/// more fail (the caller maps that to a non-zero exit).
///
/// # Errors
///
/// Returns an error when the fixture file cannot be read or parsed,
/// the schema version is unrecognized, or a referenced corpus cannot
/// be loaded.
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

    let mut passed = 0usize;
    let mut expected_found = 0usize;
    let mut expected_total = 0usize;
    let mut all_ok = true;

    for case in &suite.cases {
        let outcome = run_case(case)?;
        expected_found += outcome.expected_found;
        expected_total += outcome.expected_total;
        let verdict = if outcome.ok { "PASS" } else { "FAIL" };
        let corpus_kind = if case.synthetic { "synthetic" } else { "real" };
        if outcome.ok {
            passed += 1;
        } else {
            all_ok = false;
        }
        eprintln!(
            "  {verdict}  {name:<26} [{field}/{kind}] hit-at-{k}: {found}/{total} {detail}",
            name = case.name,
            field = case.match_field,
            kind = corpus_kind,
            k = case.k,
            found = outcome.expected_found,
            total = outcome.expected_total,
            detail = outcome.detail,
        );
    }
    eprintln!(
        "xtask retrieval-eval: {passed}/{} cases passed; expected-hit-at-k = {expected_found}/{expected_total}",
        suite.cases.len()
    );
    Ok(all_ok)
}

/// Evaluate one case: load its corpus, run the query, and check every
/// expected card-id is within the top-k (or, for a zero-result case,
/// that the search returns nothing).
fn run_case(case: &EvalCase) -> Result<CaseOutcome> {
    let index = SummariesIndex::from_path(Path::new(&case.summaries)).with_context(|| {
        format!(
            "case {:?}: cannot load summaries {}",
            case.name, case.summaries
        )
    })?;
    let matrix = load_source_matrix(Path::new(&case.source_matrix)).with_context(|| {
        format!(
            "case {:?}: cannot load source-matrix {}",
            case.name, case.source_matrix
        )
    })?;
    let hits = index.search(&case.query, case.k, Some(&matrix), &[]);
    let hit_ids: Vec<&str> = hits.iter().map(|h| h.card_id.as_str()).collect();

    if case.expected_card_ids.is_empty() {
        let ok = hits.is_empty();
        return Ok(CaseOutcome {
            ok,
            expected_found: 0,
            expected_total: 0,
            detail: if ok {
                "(zero-result, as expected)".to_string()
            } else {
                format!("expected zero results, got {hit_ids:?}")
            },
        });
    }

    let mut found = 0usize;
    let mut missing: Vec<&str> = Vec::new();
    for want in &case.expected_card_ids {
        if hit_ids.contains(&want.as_str()) {
            found += 1;
        } else {
            missing.push(want.as_str());
        }
    }
    let ok = missing.is_empty();
    Ok(CaseOutcome {
        ok,
        expected_found: found,
        expected_total: case.expected_card_ids.len(),
        detail: if ok {
            String::new()
        } else {
            format!("MISSING {missing:?} from top-{} {hit_ids:?}", case.k)
        },
    })
}

#[cfg(test)]
mod tests {
    use super::{run_case, EvalCase};

    /// Resolve a workspace-root-relative path (tests run with the
    /// crate dir as cwd, so `CARGO_MANIFEST_DIR/..` is the root).
    fn workspace_path(rel: &str) -> String {
        let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.pop();
        p.push(rel);
        p.to_string_lossy().into_owned()
    }

    fn cfa_case(name: &str, query: &str, k: i64, expected: &[&str]) -> EvalCase {
        EvalCase {
            name: name.to_string(),
            match_field: "title".to_string(),
            synthetic: false,
            summaries: workspace_path("tests/parity_corpus/cfa_first_bite/summaries.json"),
            source_matrix: workspace_path("tests/parity_corpus/cfa_first_bite/source_matrix.json"),
            query: query.to_string(),
            k,
            expected_card_ids: expected.iter().map(|s| (*s).to_string()).collect(),
        }
    }

    #[test]
    fn case_passes_when_expected_card_is_in_top_k() {
        let c = cfa_case(
            "ok",
            "valuation",
            3,
            &["intrinsic-valuation-discounted-cash-flows"],
        );
        assert!(run_case(&c).unwrap().ok);
    }

    #[test]
    fn gate_fails_when_an_expected_hit_drops_out_of_top_k() {
        // `equity-risk-and-expected-return` does not match "valuation",
        // so it can never be in the top-k for that query — the gate
        // must fail the case.
        let c = cfa_case("miss", "valuation", 3, &["equity-risk-and-expected-return"]);
        let outcome = run_case(&c).unwrap();
        assert!(!outcome.ok, "a missing expected hit must fail the gate");
        assert!(outcome.detail.contains("MISSING"), "got {}", outcome.detail);
    }

    #[test]
    fn zero_result_case_fails_when_the_query_actually_matches() {
        // Declares no expected hits, but "valuation" does match cards —
        // a non-empty result must reject the zero-result case.
        let c = cfa_case("bad-zero", "valuation", 3, &[]);
        assert!(
            !run_case(&c).unwrap().ok,
            "a non-empty result must fail a zero-result case"
        );
    }

    #[test]
    fn zero_result_case_passes_on_a_genuinely_empty_result() {
        let c = cfa_case("zero", "zzznomatchxyzzy", 3, &[]);
        assert!(run_case(&c).unwrap().ok);
    }

    /// Symmetric schema-separation gate: a semantic-eval-
    /// shaped fixture (with `cache` at the top level + per-case
    /// `chunk_hash` / `claim_window_hash` / `expected_verdict`)
    /// must fail to parse here, mirroring the rejection of
    /// retrieval-eval shape over in `semantic_eval::tests`.
    #[test]
    fn parsing_rejects_semantic_eval_shape() {
        let json = r#"{
            "schema_version": "cacg.v0.eval1",
            "cache": "out/semantic_cache.json",
            "cases": [
                {
                    "name": "semantic_eval_case",
                    "chunk_hash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    "claim_window_hash": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
                    "expected_verdict": "pass"
                }
            ]
        }"#;
        let err = serde_json::from_str::<super::EvalSuite>(json).unwrap_err();
        assert!(
            err.to_string().contains("unknown field"),
            "semantic-eval-shaped fixture must fail with `unknown field` under retrieval-eval parser; got: {err}",
        );
    }
}
