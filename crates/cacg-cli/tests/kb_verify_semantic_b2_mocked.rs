#![allow(clippy::unwrap_used)]
//! Mocked-CI tests for the B2 LLM-judge wiring.
//!
//! Drives `cacg_core::verify::verify_one_card` directly with a
//! `B2Evaluator<MockJudgeClient>` adapter to prove the synchronous
//! [`SemanticEvaluator`] boundary correctly forwards quote +
//! `chunk_text` into the async judge, and that the resulting
//! `CACG-VERIFY-002` diagnostic carries the per-verdict severity,
//! `mode=llm-judge` message tag, and `semantic_*` hint payload.
//!
//! Zero live Anthropic API calls — `MockJudgeClient::always_returns`
//! / `always_errors_*` produce canned responses and the
//! `MockJudgeClient::call_count()` counter proves the judge was
//! invoked exactly the expected number of times.
//!
//! The synthetic card fixture used here is built inline: each
//! test creates a tmpdir card whose citation pins a real
//! `chunk_hash` value from the committed parity-corpus
//! `chunks_manifest.json` (so Layer-1 hash + manifest-tamper
//! checks pass), but whose `quote` is either absent from the
//! pinned chunk text (forcing Layer-2 failure + Layer-3 firing)
//! or a substring/near-match of it (Layer-2 success, Layer-3
//! suppressed). The same fixture pattern is reused by the
//! library-level evaluator tests in
//! `crates/cacg-cli/src/round_summary.rs::tests` for batch
//! round-summary verification.

#![cfg(feature = "b2-llm-judge")]

use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use cacg_core::diagnostic::{codes, Severity};
use cacg_core::verify::{
    verify_one_card, SemanticEvaluator, SemanticMode, SemanticVerdict, SemanticVerdictKind,
};
use cacg_semantic::b2::{
    B2Evaluator, HaikuClient, JudgeError, LlmJudgeClient, MockJudgeClient, DEFAULT_API_BASE_URL,
    DEFAULT_MODEL, DEFAULT_TIMEOUT,
};
use tempfile::TempDir;

/// Committed parity-corpus chunks_manifest path. The synthetic
/// card pins a chunk hash from this manifest so Layer-1 +
/// manifest-tamper checks pass.
fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn build_scenario(tmp: &TempDir) -> (PathBuf, PathBuf, PathBuf) {
    let cards_dir = tmp.path().join("cards/reading_01");
    fs::create_dir_all(&cards_dir).unwrap();

    let chunks_path = workspace_root().join("tests/parity_corpus/out_python/chunks_manifest.json");
    assert!(
        chunks_path.is_file(),
        "committed parity-corpus chunks_manifest must exist",
    );

    // Real chunk_hash from the committed manifest.
    let chunk_a_hash = "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895";

    let card_body = format!(
        "---\n\
        schema_version: \"cacg.v0\"\n\
        id: \"synthetic-b2-card-a\"\n\
        title: \"Synthetic B2 Card A\"\n\
        reading_id: \"reading_01\"\n\
        summary: \"Synthetic card pinned to a real committed chunk_hash but quoting a phrase intentionally absent from the chunk text so Layer-2 fails and the B2 evaluator fires.\"\n\
        citations:\n\
        \x20\x20- source_id: \"sample\"\n\
        \x20\x20\x20\x20chunk_id: \"sample:p001:0000\"\n\
        \x20\x20\x20\x20chunk_hash: \"{chunk_a_hash}\"\n\
        \x20\x20\x20\x20page_range: [1, 2]\n\
        \x20\x20\x20\x20quote: \"this phrase is intentionally not a substring of the pinned chunk text\"\n\
        \x20\x20\x20\x20edge_type: \"supports\"\n\
        ---\n\
        Body.\n"
    );
    let card_path = cards_dir.join("synthetic-b2-card-a.md");
    fs::write(&card_path, card_body).unwrap();

    let journal_path = tmp.path().join("lint_journal.jsonl");
    (card_path, chunks_path, journal_path)
}

fn llm_judge_verdict(kind: SemanticVerdictKind, score: f64) -> SemanticVerdict {
    SemanticVerdict {
        kind,
        score,
        reasoning: Some("mocked".into()),
        mode: SemanticMode::LlmJudge,
    }
}

fn run_with_adapter(adapter: &dyn SemanticEvaluator) -> cacg_core::verify::VerifyOneCardResult {
    let tmp = TempDir::new().unwrap();
    let (card_path, chunks_path, journal_path) = build_scenario(&tmp);
    verify_one_card(
        &card_path,
        &chunks_path,
        &journal_path,
        false,
        false,
        None,
        None,
        None,
        Some(adapter),
        None,
    )
    .expect("runner journal append must not fail")
}

#[test]
fn b2_pass_verdict_emits_verify_002_warning_with_llm_judge_mode() {
    let mock = Arc::new(MockJudgeClient::always_returns(llm_judge_verdict(
        SemanticVerdictKind::Pass,
        0.91,
    )));
    let mock_for_inspection = mock.clone();
    let adapter = B2Evaluator::new(mock).expect("B2Evaluator runtime must build");

    let result = run_with_adapter(&adapter);

    let verify_002 = result
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 must be emitted when the B2 judge returns a verdict");
    assert!(
        matches!(verify_002.severity, Severity::Warning),
        "pass verdict on top of failing Layer-2 must be Warning; got {:?}",
        verify_002.severity,
    );
    assert!(
        verify_002.message.contains("verdict=pass"),
        "message must include the verdict tag; got: {}",
        verify_002.message,
    );
    assert!(
        verify_002.message.contains("mode=llm-judge"),
        "message must tag llm-judge mode; got: {}",
        verify_002.message,
    );
    assert_eq!(
        mock_for_inspection.call_count(),
        1,
        "the mock judge must be invoked exactly once on the single failing citation",
    );
}

#[test]
fn b2_fail_verdict_emits_verify_002_error_with_llm_judge_mode() {
    let mock = Arc::new(MockJudgeClient::always_returns(llm_judge_verdict(
        SemanticVerdictKind::Fail,
        0.18,
    )));
    let mock_for_inspection = mock.clone();
    let adapter = B2Evaluator::new(mock).expect("B2Evaluator runtime must build");

    let result = run_with_adapter(&adapter);

    let verify_002 = result
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 must be emitted on fail verdict");
    assert!(
        matches!(verify_002.severity, Severity::Error),
        "fail verdict must surface as Error severity; got {:?}",
        verify_002.severity,
    );
    assert!(verify_002.message.contains("verdict=fail"));
    assert!(verify_002.message.contains("mode=llm-judge"));
    assert_eq!(mock_for_inspection.call_count(), 1);
}

#[test]
fn b2_judge_error_emits_verify_002_error_diagnostic() {
    // MissingApiKey is the most common B2 misconfiguration surface
    // — the production CLI uses `with_components_using_env_key()`
    // which carries an empty key when ANTHROPIC_API_KEY is absent;
    // the lazy short-circuit returns MissingApiKey on first judge.
    // The mock here returns the same JudgeError variant directly.
    let mock = Arc::new(MockJudgeClient::always_errors_missing_api_key());
    let mock_for_inspection = mock.clone();
    let adapter = B2Evaluator::new(mock).expect("B2Evaluator runtime must build");

    let result = run_with_adapter(&adapter);

    let verify_002 = result
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 must be emitted on B2 judge error");
    assert!(
        matches!(verify_002.severity, Severity::Error),
        "B2 evaluator errors must surface as Error severity; got {:?}",
        verify_002.severity,
    );
    assert!(
        verify_002.message.contains("evaluator error"),
        "error-branch message must identify itself as an evaluator error; got: {}",
        verify_002.message,
    );
    assert!(
        verify_002.message.contains("mode=llm-judge"),
        "error-branch message must tag llm-judge mode; got: {}",
        verify_002.message,
    );
    assert!(
        verify_002.message.contains("ANTHROPIC_API_KEY"),
        "the JudgeError::MissingApiKey Display string must reach the diagnostic; got: {}",
        verify_002.message,
    );
    assert_eq!(
        mock_for_inspection.call_count(),
        1,
        "the mock judge must be invoked exactly once even when it returns an error",
    );
}

#[test]
fn b2_judge_timeout_error_emits_verify_002_error_with_stable_message() {
    // Exercises the Timeout variant — the message text comes from
    // JudgeError::Display ("LLM-judge request timed out") and must
    // reach the diagnostic verbatim so future error-class analysis
    // can grep on stable substrings.
    let mock = Arc::new(MockJudgeClient::always_errors_timeout());
    let adapter = B2Evaluator::new(mock).expect("B2Evaluator runtime must build");

    let result = run_with_adapter(&adapter);
    let verify_002 = result
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 must be emitted on B2 timeout");
    assert!(matches!(verify_002.severity, Severity::Error));
    assert!(
        verify_002.message.contains("timed out"),
        "timeout message must reach the diagnostic verbatim; got: {}",
        verify_002.message,
    );
}

#[test]
fn b2_judge_malformed_response_error_emits_verify_002_error() {
    let mock = Arc::new(MockJudgeClient::always_errors_malformed(
        "score field missing or not a number",
    ));
    let adapter = B2Evaluator::new(mock).expect("B2Evaluator runtime must build");

    let result = run_with_adapter(&adapter);
    let verify_002 = result
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 must be emitted on B2 malformed response");
    assert!(matches!(verify_002.severity, Severity::Error));
    assert!(
        verify_002.message.contains("malformed response"),
        "malformed-response message must reach the diagnostic; got: {}",
        verify_002.message,
    );
    assert!(
        verify_002.message.contains("score field missing"),
        "the inner detail string must be preserved; got: {}",
        verify_002.message,
    );
}

#[test]
fn b2_judge_other_error_emits_verify_002_error_with_stable_prefix() {
    let mock = Arc::new(MockJudgeClient::always_errors_other(
        "synthetic test-only error",
    ));
    let adapter = B2Evaluator::new(mock).expect("B2Evaluator runtime must build");

    let result = run_with_adapter(&adapter);
    let verify_002 = result
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 must be emitted on B2 other error");
    assert!(matches!(verify_002.severity, Severity::Error));
    // JudgeError::Other displays as "LLM-judge: {0}".
    assert!(verify_002.message.contains("LLM-judge"));
    assert!(verify_002.message.contains("synthetic test-only error"));
}

#[test]
fn b2_evaluator_forwards_quote_and_chunk_text_into_judge_call() {
    // A non-trivial mock that captures the arguments and asserts
    // they reach the LLM. We use a dedicated impl rather than the
    // generic MockJudgeClient so we can inspect call arguments.
    use std::sync::Mutex;

    #[derive(Default)]
    struct CapturingClient {
        captured_quote: Mutex<Option<String>>,
        captured_chunk_text: Mutex<Option<String>>,
    }

    #[async_trait::async_trait]
    impl LlmJudgeClient for CapturingClient {
        async fn judge(
            &self,
            _chunk_hash: &str,
            _claim_window_hash: &str,
            quote: &str,
            chunk_text: &str,
        ) -> Result<SemanticVerdict, JudgeError> {
            *self.captured_quote.lock().unwrap() = Some(quote.to_string());
            *self.captured_chunk_text.lock().unwrap() = Some(chunk_text.to_string());
            Ok(llm_judge_verdict(SemanticVerdictKind::Abstain, 0.0))
        }
    }

    let capturing = Arc::new(CapturingClient::default());
    let adapter = B2Evaluator::new(capturing.clone()).expect("runtime");

    let _result = run_with_adapter(&adapter);

    let q = capturing.captured_quote.lock().unwrap().clone();
    let c = capturing.captured_chunk_text.lock().unwrap().clone();
    assert!(
        q.as_deref()
            .map(|s| s.contains("intentionally not a substring"))
            == Some(true),
        "quote must reach judge() verbatim from the citation; got: {q:?}",
    );
    assert!(
        c.is_some() && !c.as_deref().unwrap().is_empty(),
        "chunk_text must reach judge() and must be non-empty; got: {c:?}",
    );
}

// ---------------------------------------------------------------
// Default-path-zero-network proof tests.
//
// These tests prove the negative-firing contract: when Layer-2
// succeeds (exact substring containment OR bounded fuzzy match),
// `verify_citation` MUST NOT invoke the supplied evaluator —
// even though it was passed in. We wire a `MockJudgeClient::panicking()`
// through `B2Evaluator`; if anything in the runtime accidentally
// invokes the judge, the panic surfaces and the test fails.
// ---------------------------------------------------------------

/// Build a Layer-2-success card whose `quote` IS a substring of
/// the pinned chunk text. The same `chunk_a_hash` from
/// [`build_scenario`] is reused (page_range covers both pages of
/// the chunk to avoid page-window slicing rejecting the match).
fn build_layer2_exact_match_scenario(tmp: &TempDir) -> (PathBuf, PathBuf, PathBuf) {
    let cards_dir = tmp.path().join("cards/reading_01");
    fs::create_dir_all(&cards_dir).unwrap();

    let chunks_path = workspace_root().join("tests/parity_corpus/out_python/chunks_manifest.json");
    assert!(chunks_path.is_file());

    let chunk_a_hash = "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895";
    // A verbatim substring near the start of the chunk text (page 1).
    let exact_substring = "Content-addressable identity is the verification primitive";

    let card_body = format!(
        "---\n\
        schema_version: \"cacg.v0\"\n\
        id: \"synthetic-layer2-exact-card\"\n\
        title: \"Synthetic Layer-2 Exact-Match Card\"\n\
        reading_id: \"reading_01\"\n\
        summary: \"Synthetic card whose quote is a verbatim substring of the pinned chunk text, forcing Layer-2 exact-substring containment to SUCCEED so Layer-3 must NOT fire.\"\n\
        citations:\n\
        \x20\x20- source_id: \"sample\"\n\
        \x20\x20\x20\x20chunk_id: \"sample:p001:0000\"\n\
        \x20\x20\x20\x20chunk_hash: \"{chunk_a_hash}\"\n\
        \x20\x20\x20\x20page_range: [1, 2]\n\
        \x20\x20\x20\x20quote: \"{exact_substring}\"\n\
        \x20\x20\x20\x20edge_type: \"supports\"\n\
        ---\n\
        Body.\n"
    );
    let card_path = cards_dir.join("synthetic-layer2-exact-card.md");
    fs::write(&card_path, card_body).unwrap();

    let journal_path = tmp.path().join("lint_journal.jsonl");
    (card_path, chunks_path, journal_path)
}

/// Build a Layer-2-fuzzy-match-success card whose `quote` is a
/// bounded-Levenshtein near-match of a chunk substring. With
/// `fuzzy_enabled = true`, Layer-2 accepts; Layer-3 must NOT fire.
fn build_layer2_fuzzy_match_scenario(tmp: &TempDir) -> (PathBuf, PathBuf, PathBuf) {
    let cards_dir = tmp.path().join("cards/reading_01");
    fs::create_dir_all(&cards_dir).unwrap();

    let chunks_path = workspace_root().join("tests/parity_corpus/out_python/chunks_manifest.json");
    assert!(chunks_path.is_file());

    let chunk_a_hash = "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895";
    // Original substring (verbatim from the chunk):
    //   "Hash pinning protects against silent text drift."
    // Fuzzy near-match: a single character substitution
    // ("protects" → "protecs") plus a missing period. Falls
    // inside the DEFAULT_MAX_DIST / DEFAULT_MIN_RATIO bound the
    // verifier uses.
    let fuzzy_quote = "Hash pinning protecs against silent text drift";

    let card_body = format!(
        "---\n\
        schema_version: \"cacg.v0\"\n\
        id: \"synthetic-layer2-fuzzy-card\"\n\
        title: \"Synthetic Layer-2 Fuzzy-Match Card\"\n\
        reading_id: \"reading_01\"\n\
        summary: \"Synthetic card whose quote is a bounded-Levenshtein near-match of a chunk substring; fuzzy_enabled must accept and Layer-3 must NOT fire.\"\n\
        citations:\n\
        \x20\x20- source_id: \"sample\"\n\
        \x20\x20\x20\x20chunk_id: \"sample:p001:0000\"\n\
        \x20\x20\x20\x20chunk_hash: \"{chunk_a_hash}\"\n\
        \x20\x20\x20\x20page_range: [1, 2]\n\
        \x20\x20\x20\x20quote: \"{fuzzy_quote}\"\n\
        \x20\x20\x20\x20edge_type: \"supports\"\n\
        ---\n\
        Body.\n"
    );
    let card_path = cards_dir.join("synthetic-layer2-fuzzy-card.md");
    fs::write(&card_path, card_body).unwrap();

    let journal_path = tmp.path().join("lint_journal.jsonl");
    (card_path, chunks_path, journal_path)
}

#[test]
fn b2_panicking_mock_with_layer2_exact_match_does_not_invoke_judge() {
    let mock = Arc::new(MockJudgeClient::panicking());
    let adapter = B2Evaluator::new(mock).expect("B2Evaluator runtime must build");

    let tmp = TempDir::new().unwrap();
    let (card_path, chunks_path, journal_path) = build_layer2_exact_match_scenario(&tmp);
    let result = verify_one_card(
        &card_path,
        &chunks_path,
        &journal_path,
        false, // fuzzy = false; exact-substring containment is the path under test
        false, // skip_lint = false
        None,
        None,
        None,
        Some(&adapter as &dyn SemanticEvaluator),
        None,
    )
    .expect("runner journal append must not fail");

    assert!(
        result.verified,
        "Layer-2 exact-substring match must succeed; got verified={} diagnostics={:?}",
        result.verified, result.diagnostics,
    );
    assert!(
        !result
            .diagnostics
            .iter()
            .any(|d| d.code == codes::VERIFY_002),
        "CACG-VERIFY-002 must NOT be emitted when Layer-2 succeeds; got: {:?}",
        result.diagnostics,
    );
    // Reaching this assertion without panicking is the actual
    // contract proof: the panicking mock would have surfaced if
    // the runtime accidentally invoked it.
}

#[test]
fn b2_panicking_mock_with_layer2_fuzzy_match_does_not_invoke_judge() {
    let mock = Arc::new(MockJudgeClient::panicking());
    let adapter = B2Evaluator::new(mock).expect("B2Evaluator runtime must build");

    let tmp = TempDir::new().unwrap();
    let (card_path, chunks_path, journal_path) = build_layer2_fuzzy_match_scenario(&tmp);
    let result = verify_one_card(
        &card_path,
        &chunks_path,
        &journal_path,
        true,  // fuzzy = true; near-match must be accepted
        false, // skip_lint = false
        None,
        None,
        None,
        Some(&adapter as &dyn SemanticEvaluator),
        None,
    )
    .expect("runner journal append must not fail");

    assert!(
        result.verified,
        "Layer-2 fuzzy match must succeed under fuzzy_enabled=true; got verified={} diagnostics={:?}",
        result.verified, result.diagnostics,
    );
    assert!(
        !result
            .diagnostics
            .iter()
            .any(|d| d.code == codes::VERIFY_002),
        "CACG-VERIFY-002 must NOT be emitted when Layer-2 (fuzzy) succeeds; got: {:?}",
        result.diagnostics,
    );
}

#[test]
fn b2_panicking_mock_panics_when_directly_invoked() {
    // Sanity check that the panicking-mock canary is live. If the
    // panicking constructor is ever weakened to a silent no-op
    // (or the adapter's block_on swallows panics), the
    // negative-firing tests above would silently regress. This
    // test catches that regression by asserting the panic.
    let mock = Arc::new(MockJudgeClient::panicking());
    let adapter = B2Evaluator::new(mock).expect("B2Evaluator runtime must build");

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = (&adapter as &dyn SemanticEvaluator).evaluate(
            "a".repeat(64).as_str(),
            "b".repeat(64).as_str(),
            "quote",
            "chunk text",
        );
    }));
    assert!(
        result.is_err(),
        "directly invoking the panicking adapter must panic",
    );
}

#[test]
fn b2_panicking_mock_through_round_summary_with_all_layer2_passing_cites_does_not_invoke_judge() {
    // Round-summary (batch) counterpart to the single-card
    // panicking-mock tests above. Lays down BOTH Layer-2-success
    // synthetic cards (exact-match + fuzzy-match) in the same
    // tempdir, writes a summary citing both, and runs
    // `verify_round_summary` with a `B2Evaluator<MockJudgeClient::panicking()>`
    // attached. Every cite passes Layer-2, so the batch path must
    // NOT invoke the supplied evaluator on any iteration. The
    // panicking mock surfaces any accidental invocation as a panic
    // — reaching the assertions without panic is the proof.
    use cacg_cli::round_summary::{verify_round_summary, Verdict};

    let tmp = TempDir::new().unwrap();
    // The two single-card Layer-2-success scenario builders
    // share the same `cards_dir = tmp/cards/reading_01`
    // (create_dir_all is idempotent) and use different card
    // filenames, so they can coexist in one tempdir. Each
    // returns a (card, chunks, journal) triple but only the
    // card path differs across the pair.
    let (card_exact, chunks_path, _journal_unused_1) = build_layer2_exact_match_scenario(&tmp);
    let (card_fuzzy, _chunks_path_2, _journal_unused_2) = build_layer2_fuzzy_match_scenario(&tmp);
    assert_eq!(_chunks_path_2, chunks_path); // sanity: same manifest

    // Cite both cards through tempdir-relative paths. The
    // round-summary resolver receives `cwd_root = Some(tmp.path())`
    // below so these resolve correctly.
    let card_exact_rel = card_exact
        .strip_prefix(tmp.path())
        .expect("card under tempdir")
        .to_str()
        .unwrap()
        .to_string();
    let card_fuzzy_rel = card_fuzzy
        .strip_prefix(tmp.path())
        .expect("card under tempdir")
        .to_str()
        .unwrap()
        .to_string();
    let summary_body = format!(
        "## Knowledge Consulted\n\n\
        - {card_exact_rel}\n\
        - {card_fuzzy_rel}\n",
    );
    let summary_path = tmp.path().join("summary.md");
    fs::write(&summary_path, summary_body).unwrap();

    let journal_path = tmp.path().join("lint_journal.jsonl");

    let mock = Arc::new(MockJudgeClient::panicking());
    let adapter = B2Evaluator::new(mock).expect("B2Evaluator runtime must build");

    let result = verify_round_summary(
        &summary_path,
        &chunks_path,
        &journal_path,
        true,             // fuzzy = true (Card B needs fuzzy match)
        None,             // source_matrix_path
        false,            // allow_retracted
        Some(tmp.path()), // cwd_root
        Some(&adapter as &dyn SemanticEvaluator),
    )
    .expect("verify_round_summary must not error on clean Layer-2-success batch");

    assert_eq!(
        result.paths.len(),
        2,
        "summary cites 2 cards; got {} path verdicts: {:?}",
        result.paths.len(),
        result.paths,
    );
    for pv in &result.paths {
        assert_eq!(
            pv.verdict,
            Verdict::Verified,
            "every cited card must verify on Layer-2 success; got verdict {:?} for path {} (detail: {})",
            pv.verdict, pv.path, pv.detail,
        );
    }
    assert_eq!(
        result.exit_code(),
        0,
        "all-Verified batch must exit 0; got {} (paths={:?})",
        result.exit_code(),
        result.paths,
    );

    let journal_text = fs::read_to_string(&journal_path).expect("journal file must be created");
    let journal_line_count = journal_text.lines().filter(|l| !l.is_empty()).count();
    assert_eq!(
        journal_line_count,
        result.paths.len(),
        "journal must contain exactly one event per cited path; got {} lines for {} cites",
        journal_line_count,
        result.paths.len(),
    );
    assert!(
        !journal_text.contains("CACG-VERIFY-002"),
        "journal must NOT contain CACG-VERIFY-002 when Layer-2 succeeds on every cite (proves the batch path did not invoke the supplied evaluator); got: {journal_text}",
    );
    // Reaching this assertion without the panicking mock
    // panicking IS the negative-firing proof for the batch path.
}

// ---------------------------------------------------------------
// Per-variant diagnostic tests with stable message substrings.
//
// These tests pin the EXACT detail strings `HaikuClient::parse_haiku_verdict`
// produces for each malformed-response sub-class through
// `MockJudgeClient::always_errors_malformed(detail)`, then assert
// the resulting CACG-VERIFY-002 message carries the stable
// per-class substring. Together they cover the negative-path
// contract that malformed B2 payloads (out-of-range score,
// unknown verdict string, non-JSON body) surface as
// CACG-VERIFY-002 with severity Error and a diagnosable text.
// The matching JudgeError variant constructors in
// `parse_haiku_verdict` are exercised at the parser level by the
// b2.rs unit tests; this layer confirms those detail strings
// propagate verbatim through the SemanticEvaluationError
// boundary to the diagnostic surface.
// ---------------------------------------------------------------

#[test]
fn b2_diagnostic_propagates_unknown_verdict_string_detail() {
    // Exact text from `parse_haiku_verdict` on
    // `{"verdict": "maybe", "score": 0.5}`.
    let detail = r#"unknown verdict string: "maybe""#;
    let mock = Arc::new(MockJudgeClient::always_errors_malformed(detail));
    let adapter = B2Evaluator::new(mock).expect("B2Evaluator runtime must build");

    let result = run_with_adapter(&adapter);

    let verify_002 = result
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 must be emitted on unknown-verdict malformed response");
    assert!(matches!(verify_002.severity, Severity::Error));
    assert!(
        verify_002.message.contains("malformed response"),
        "diagnostic must carry the JudgeError class prefix; got: {}",
        verify_002.message,
    );
    assert!(
        verify_002.message.contains("unknown verdict string"),
        "diagnostic must surface the stable per-class substring; got: {}",
        verify_002.message,
    );
    assert!(
        verify_002.message.contains("mode=llm-judge"),
        "diagnostic must tag llm-judge mode; got: {}",
        verify_002.message,
    );
}

#[test]
fn b2_diagnostic_propagates_out_of_range_score_detail() {
    // Exact text from `parse_haiku_verdict` on
    // `{"verdict": "pass", "score": 1.5}`.
    let detail = "score 1.5 is not finite in [0.0, 1.0]";
    let mock = Arc::new(MockJudgeClient::always_errors_malformed(detail));
    let adapter = B2Evaluator::new(mock).expect("B2Evaluator runtime must build");

    let result = run_with_adapter(&adapter);

    let verify_002 = result
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 must be emitted on out-of-range-score malformed response");
    assert!(matches!(verify_002.severity, Severity::Error));
    assert!(verify_002.message.contains("malformed response"));
    assert!(
        verify_002.message.contains("score 1.5"),
        "diagnostic must include the offending score value; got: {}",
        verify_002.message,
    );
    assert!(
        verify_002.message.contains("not finite"),
        "diagnostic must include the range-violation substring; got: {}",
        verify_002.message,
    );
    assert!(verify_002.message.contains("mode=llm-judge"));
}

#[test]
fn b2_diagnostic_propagates_non_json_response_detail() {
    // Exact text from `parse_haiku_verdict` on a non-JSON body.
    let detail = "LLM response body is not strict JSON: expected value at line 1 column 1";
    let mock = Arc::new(MockJudgeClient::always_errors_malformed(detail));
    let adapter = B2Evaluator::new(mock).expect("B2Evaluator runtime must build");

    let result = run_with_adapter(&adapter);

    let verify_002 = result
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 must be emitted on non-JSON malformed response");
    assert!(matches!(verify_002.severity, Severity::Error));
    assert!(verify_002.message.contains("malformed response"));
    assert!(
        verify_002.message.contains("strict JSON"),
        "diagnostic must surface the non-JSON stable substring; got: {}",
        verify_002.message,
    );
    assert!(verify_002.message.contains("mode=llm-judge"));
}

#[test]
fn b2_diagnostic_real_haiku_client_with_empty_api_key_surfaces_missing_api_key() {
    // Realistic CLI path: `build_semantic_evaluator` uses
    // `HaikuClient::with_components_using_env_key()`, which carries
    // an empty `api_key` when `ANTHROPIC_API_KEY` is absent. The
    // lazy empty-key check in `HaikuClient::judge` short-circuits
    // with `JudgeError::MissingApiKey` BEFORE any HTTP work.
    // This test exercises the same code path with an explicit
    // empty key + the production defaults (model, base_url,
    // timeout) — no network egress.
    let client = HaikuClient::with_components(
        String::new(),
        DEFAULT_MODEL.to_string(),
        DEFAULT_API_BASE_URL.to_string(),
        DEFAULT_TIMEOUT,
    );
    let adapter = B2Evaluator::new(Arc::new(client)).expect("B2Evaluator runtime must build");

    let result = run_with_adapter(&adapter);

    let verify_002 = result
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 must be emitted when the real HaikuClient lazy-key check fires");
    assert!(matches!(verify_002.severity, Severity::Error));
    assert!(
        verify_002.message.contains("evaluator error"),
        "diagnostic must identify itself as an evaluator error; got: {}",
        verify_002.message,
    );
    assert!(
        verify_002.message.contains("ANTHROPIC_API_KEY"),
        "diagnostic must surface the MissingApiKey stable substring; got: {}",
        verify_002.message,
    );
    assert!(verify_002.message.contains("mode=llm-judge"));
}
