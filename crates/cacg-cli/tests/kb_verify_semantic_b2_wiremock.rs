#![allow(clippy::unwrap_used)]
//! Wiremock-based isolated integration tests for the B2 `HaikuClient`
//! HTTP transport layer.
//!
//! Standard CI gates forbid live Anthropic API calls; this file
//! pairs the production `HaikuClient::with_components(...)` against
//! an in-process `wiremock::MockServer` to exercise the real reqwest
//! transport path under deterministic conditions. The wiremock
//! matchers pin the canonical Anthropic Messages API request shape
//! the client must send (method, path, headers, JSON body fields);
//! the canned responses validate the parser's end-to-end behavior
//! with a real HTTP roundtrip.
//!
//! Two tests:
//!
//! 1. `haiku_client_success_response_yields_pass_verdict_with_llm_judge_mode`
//!    — a 200 OK with an Anthropic-shaped envelope carrying strict-JSON
//!    verdict text in `content[0].text`. Asserts the client sent
//!    `POST /v1/messages` with the expected headers + body shape, and
//!    parsed the response into `SemanticVerdict { kind: Pass, score,
//!    mode: LlmJudge }`. Uses `client.judge(...).await` directly (no
//!    `B2Evaluator`) since the test is already in an async context
//!    and the focus is the HTTP transport layer.
//!
//! 2. `haiku_client_non_2xx_response_emits_verify_002_error_without_leaking_canary`
//!    — a 500 error whose body does not echo the `api_key` canary.
//!    Drives `B2Evaluator<HaikuClient>` through `verify_one_card`
//!    (synchronous) via `tokio::task::spawn_blocking` to bridge the
//!    outer multi-threaded tokio runtime hosting wiremock with the
//!    inner current-thread runtime the `B2Evaluator` owns. Asserts
//!    `CACG-VERIFY-002` severity Error + `mode=llm-judge` + canary
//!    absent from both the diagnostic message and every hint.
//!
//! These two tests together cover the wiremock isolation contract:
//! standard CI exercises only mocks (`MockJudgeClient` or wiremock);
//! the production `HaikuClient` HTTP path is validated end-to-end
//! without ever talking to the real Anthropic API.

#![cfg(feature = "b2-llm-judge")]

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use cacg_core::diagnostic::{codes, Severity};
use cacg_core::verify::{verify_one_card, SemanticEvaluator, SemanticMode, SemanticVerdictKind};
use cacg_semantic::b2::{B2Evaluator, HaikuClient, DEFAULT_MODEL};
use serde_json::json;
use tempfile::TempDir;
use wiremock::matchers::{header, method, path};
use wiremock::{Match, Mock, MockServer, Request, ResponseTemplate};

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

/// Synthetic Layer-2-failing scenario: a card pinning a real
/// `chunk_hash` from the committed parity-corpus manifest but
/// quoting text intentionally absent from the chunk text. Layer-2
/// fails (`CACG-VERIFY-001`); Layer-3 fires unconditionally. Used
/// only by the error-path wiremock test — the success-path test
/// calls `client.judge(...)` directly without going through
/// `verify_one_card`.
fn build_layer2_failing_scenario(tmp: &TempDir) -> (PathBuf, PathBuf, PathBuf) {
    let cards_dir = tmp.path().join("cards/reading_01");
    fs::create_dir_all(&cards_dir).unwrap();

    let chunks_path = workspace_root().join("tests/parity_corpus/out_python/chunks_manifest.json");
    assert!(chunks_path.is_file());

    let chunk_a_hash = "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895";
    let card_body = format!(
        "---\n\
        schema_version: \"cacg.v0\"\n\
        id: \"synthetic-wiremock-card\"\n\
        title: \"Synthetic Wiremock Card\"\n\
        reading_id: \"reading_01\"\n\
        summary: \"Synthetic card whose quote is intentionally absent from the pinned chunk text, forcing Layer-2 failure so the supplied B2 evaluator (driving wiremock-served HTTP) is invoked.\"\n\
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
    let card_path = cards_dir.join("synthetic-wiremock-card.md");
    fs::write(&card_path, card_body).unwrap();

    let journal_path = tmp.path().join("lint_journal.jsonl");
    (card_path, chunks_path, journal_path)
}

fn run_verify_one_card_sync(
    adapter: B2Evaluator,
    card_path: &Path,
    chunks_path: &Path,
    journal_path: &Path,
) -> cacg_core::verify::VerifyOneCardResult {
    verify_one_card(
        card_path,
        chunks_path,
        journal_path,
        false, // fuzzy
        false, // skip_lint
        None,
        None,
        None,
        Some(&adapter as &dyn SemanticEvaluator),
        None,
    )
    .expect("runner journal append must not fail")
}

/// Custom wiremock matcher: parses the request body as JSON and
/// verifies the Anthropic Messages API request shape the
/// production `HaikuClient::judge` body must emit.
///
/// Asserts:
/// - `model` equals [`DEFAULT_MODEL`]
/// - `max_tokens` equals 256
/// - `messages[0].role == "user"`
/// - `messages[0].content` contains both the configured quote
///   substring AND the configured chunk-text substring
///
/// If any assertion fails (or the body is not valid JSON), the
/// matcher returns false and wiremock falls through to the
/// default no-match handler (HTTP 404). `client.judge(...)` then
/// surfaces a `MalformedResponse` and the test's verdict
/// assertion fails with diagnostic clarity.
struct AnthropicRequestBodyMatcher {
    expected_quote: &'static str,
    expected_chunk_text: &'static str,
}

impl Match for AnthropicRequestBodyMatcher {
    fn matches(&self, request: &Request) -> bool {
        let Ok(body): Result<serde_json::Value, _> = serde_json::from_slice(&request.body) else {
            return false;
        };
        if body.get("model").and_then(|m| m.as_str()) != Some(DEFAULT_MODEL) {
            return false;
        }
        if body.get("max_tokens").and_then(|t| t.as_u64()) != Some(256) {
            return false;
        }
        let messages = match body.get("messages").and_then(|m| m.as_array()) {
            Some(arr) if !arr.is_empty() => arr,
            _ => return false,
        };
        let first = &messages[0];
        if first.get("role").and_then(|r| r.as_str()) != Some("user") {
            return false;
        }
        let content = match first.get("content").and_then(|c| c.as_str()) {
            Some(s) => s,
            None => return false,
        };
        content.contains(self.expected_quote) && content.contains(self.expected_chunk_text)
    }
}

#[tokio::test]
async fn haiku_client_success_response_yields_pass_verdict_with_llm_judge_mode() {
    let server = MockServer::start().await;
    let canary = "sk-ant-wiremock-canary-XXXX";
    // Identifiable substrings the body matcher will search for in
    // `messages[0].content` — chosen to be distinct from the
    // surrounding prompt boilerplate so a regression that drops
    // either one can't accidentally satisfy the substring check
    // via the prompt template text.
    let quote = "synthetic quote text for wiremock body validation";
    let chunk_text = "synthetic chunk text for wiremock body validation";

    // The matchers pin the canonical Anthropic Messages API request
    // shape the production HaikuClient::judge body must emit:
    // POST /v1/messages + x-api-key + anthropic-version +
    // content-type headers AND the JSON body (model, max_tokens,
    // messages[0].role, messages[0].content contains quote + chunk
    // text). The response is an Anthropic-shaped envelope carrying
    // strict-JSON verdict text — exercises the full
    // parse_haiku_verdict path through a real HTTP roundtrip.
    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .and(header("x-api-key", canary))
        .and(header("anthropic-version", "2023-06-01"))
        .and(header("content-type", "application/json"))
        .and(AnthropicRequestBodyMatcher {
            expected_quote: quote,
            expected_chunk_text: chunk_text,
        })
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "content": [
                {
                    "type": "text",
                    "text":
                        "{\"verdict\":\"pass\",\"score\":0.87,\"reasoning\":\"matches\"}"
                }
            ]
        })))
        .mount(&server)
        .await;

    let base_url = format!("{}/v1/messages", server.uri());
    let client = HaikuClient::with_components(
        canary.to_string(),
        DEFAULT_MODEL.to_string(),
        base_url,
        Duration::from_secs(5),
    );

    let verdict = (&client as &dyn cacg_semantic::b2::LlmJudgeClient)
        .judge(
            "a".repeat(64).as_str(),
            "b".repeat(64).as_str(),
            quote,
            chunk_text,
        )
        .await
        .expect("HTTP success path must return Ok");

    assert_eq!(verdict.kind, SemanticVerdictKind::Pass);
    assert!(
        (verdict.score - 0.87).abs() < 1e-9,
        "score must round-trip through HTTP exactly; got {}",
        verdict.score,
    );
    assert_eq!(verdict.mode, SemanticMode::LlmJudge);
    assert_eq!(verdict.reasoning.as_deref(), Some("matches"));
}

#[tokio::test(flavor = "multi_thread")]
async fn haiku_client_non_2xx_response_emits_verify_002_error_without_leaking_canary() {
    let server = MockServer::start().await;
    let canary = "sk-ant-wiremock-canary-XXXX";

    // Wiremock returns a 500 with a body that does NOT echo the
    // canary; the canary-absence assertion below is therefore a
    // meaningful check (the upstream body never carried it), not
    // trivially passing.
    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .respond_with(
            ResponseTemplate::new(500)
                .set_body_string("internal server error — body does not echo the canary"),
        )
        .mount(&server)
        .await;

    let base_url = format!("{}/v1/messages", server.uri());
    let client = HaikuClient::with_components(
        canary.to_string(),
        DEFAULT_MODEL.to_string(),
        base_url,
        Duration::from_secs(5),
    );
    let adapter = B2Evaluator::new(Arc::new(client)).expect("B2Evaluator runtime must build");

    // verify_one_card is synchronous and B2Evaluator owns its own
    // current-thread tokio runtime. Driving it from inside this
    // outer multi-threaded tokio test runtime would panic with
    // "Cannot start a runtime from within a runtime"; spawn_blocking
    // moves the synchronous work onto a dedicated blocking thread
    // where the inner runtime can safely block_on its judge call,
    // while the outer runtime continues to host the wiremock server.
    let tmp = TempDir::new().unwrap();
    let (card_path, chunks_path, journal_path) = build_layer2_failing_scenario(&tmp);
    let result = tokio::task::spawn_blocking(move || {
        run_verify_one_card_sync(adapter, &card_path, &chunks_path, &journal_path)
    })
    .await
    .expect("spawn_blocking task must not panic");

    let verify_002 = result
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 must be emitted on HTTP 5xx");
    assert!(
        matches!(verify_002.severity, Severity::Error),
        "HTTP 5xx must surface as severity Error; got {:?}",
        verify_002.severity,
    );
    assert!(
        verify_002.message.contains("mode=llm-judge"),
        "diagnostic must tag llm-judge mode; got: {}",
        verify_002.message,
    );
    assert!(
        !verify_002.message.contains(canary),
        "secret non-leakage VIOLATED: api_key canary appeared in CACG-VERIFY-002 message; got: {}",
        verify_002.message,
    );
    for (i, hint) in verify_002.hints.iter().enumerate() {
        let hint_text = hint.to_string();
        assert!(
            !hint_text.contains(canary),
            "secret non-leakage VIOLATED: api_key canary appeared in CACG-VERIFY-002 hint[{}]; got: {}",
            i,
            hint_text,
        );
    }
}
