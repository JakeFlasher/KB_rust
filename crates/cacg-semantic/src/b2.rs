//! B2 LLM-judge async client surface.
//!
//! Feature-gated under `b2-llm-judge`. This module is compiled
//! only when the feature is active; the trust kernel and the
//! default `cacg-cli` binary never see this code or its transitive
//! dependencies (reqwest + tokio + async-trait).
//!
//! Three public items:
//!
//! - [`LlmJudgeClient`] — the async boundary trait. Callers hold
//!   `Box<dyn LlmJudgeClient + Send + Sync>` and `await
//!   client.judge(...)`. The trait is object-safe via the
//!   `async_trait::async_trait` macro.
//! - [`MockJudgeClient`] — a configurable test implementation
//!   producing canned verdicts or errors via an internal closure,
//!   with an atomic call counter for cardinality assertions and a
//!   [`MockJudgeClient::panicking`] constructor used to prove that
//!   the default verify path (no `--semantic-judge`) makes ZERO
//!   outbound HTTP calls.
//! - [`HaikuClient`] — the production implementation. Posts a
//!   strict-JSON-verdict prompt to the Anthropic `/v1/messages`
//!   endpoint via `reqwest`, parses the returned verdict, and
//!   maps transport / parse / range failures onto [`JudgeError`].
//!   The current `judge` body issues a single POST with no
//!   retry / backoff; the only transport error class surfaced as
//!   [`JudgeError::Timeout`] is the one `reqwest::Error::is_timeout()`
//!   identifies (every other reqwest failure becomes
//!   [`JudgeError::Http`]). Structured retry / backoff policy is
//!   not yet implemented. The HTTP path is validated end-to-end
//!   under deterministic conditions by the isolated wiremock
//!   integration test in `cacg-cli`'s feature-gated
//!   `tests/kb_verify_semantic_b2_wiremock.rs`, which pins the
//!   Anthropic request shape (method, path, `x-api-key`,
//!   `anthropic-version`, JSON body) and the success +
//!   non-2xx response paths; the in-crate unit tests below cover
//!   the free [`parse_haiku_verdict`] helper and the constructor
//!   paths.
//!
//! The [`SemanticVerdict::mode`] returned by both clients is
//! [`SemanticMode::LlmJudge`].

use std::env;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::{
    SemanticEvaluationError, SemanticEvaluator, SemanticMode, SemanticVerdict, SemanticVerdictKind,
};

/// Default upstream endpoint for the Anthropic Messages API.
pub const DEFAULT_API_BASE_URL: &str = "https://api.anthropic.com/v1/messages";

/// Default Claude Haiku model identifier used by [`HaikuClient`].
pub const DEFAULT_MODEL: &str = "claude-haiku-4-5-20251001";

/// Default request timeout for [`HaikuClient`].
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Errors that the B2 LLM-judge async client may return.
///
/// `Display` strings are stable and safe to embed in
/// `CACG-VERIFY-002` diagnostic messages without leaking secrets:
/// the API key never appears in any variant.
#[derive(Debug, thiserror::Error)]
pub enum JudgeError {
    /// The `ANTHROPIC_API_KEY` environment variable was not set or
    /// not valid UTF-8 at the point a key was required.
    #[error("ANTHROPIC_API_KEY environment variable is not set")]
    MissingApiKey,

    /// Transport-level reqwest error (DNS resolution, TLS
    /// handshake, connection refused, response-body read).
    /// Mapped from [`reqwest::Error`] when the error is NOT a
    /// timeout — timeouts are surfaced separately as
    /// [`JudgeError::Timeout`].
    #[error("LLM-judge HTTP transport error: {0}")]
    Http(reqwest::Error),

    /// Request exceeded the configured timeout window.
    #[error("LLM-judge request timed out")]
    Timeout,

    /// Response payload was malformed: unparseable JSON, missing
    /// fields, unknown verdict string, or out-of-range score.
    #[error("LLM-judge malformed response: {0}")]
    MalformedResponse(String),

    /// Tokio async runtime failed to build (exceedingly rare;
    /// configuration-level infrastructure error).
    #[error("LLM-judge runtime build failed: {0}")]
    RuntimeBuild(String),

    /// Explicit test injection or bounded last-resort catch-all.
    /// Production usage is confined to `MockJudgeClient` test doubles.
    // qg-allow: intentional-catch-all — bounded to test mocks and mock-driven integration tests
    #[error("LLM-judge: {0}")]
    Other(String),
}

/// Async boundary for Layer-3 LLM judging.
///
/// Implementations live in this crate ([`HaikuClient`],
/// [`MockJudgeClient`]) and stay out of the trust kernel. Callers
/// hold a trait object: `Box<dyn LlmJudgeClient + Send + Sync>`.
#[async_trait::async_trait]
pub trait LlmJudgeClient: Send + Sync {
    /// Decide whether `quote` is semantically supported by
    /// `chunk_text`. The two hashes are passed through to allow
    /// observability hooks (e.g. trace logs keyed by the same
    /// `(chunk_hash, claim_window_hash)` pair the cache uses) but
    /// are not part of the prompt the LLM sees.
    async fn judge(
        &self,
        chunk_hash: &str,
        claim_window_hash: &str,
        quote: &str,
        chunk_text: &str,
    ) -> Result<SemanticVerdict, JudgeError>;
}

// ----------------------------------------------------------------------
// MockJudgeClient
// ----------------------------------------------------------------------

type JudgeProducer = Box<dyn Fn() -> Result<SemanticVerdict, JudgeError> + Send + Sync>;

/// Test implementation of [`LlmJudgeClient`].
///
/// Stores a closure (`Box<dyn Fn() -> Result<...> + Send + Sync>`)
/// that produces the next verdict (or error) on each call. The
/// internal counter is `AtomicUsize` so concurrent callers can
/// read `call_count()` safely.
pub struct MockJudgeClient {
    producer: JudgeProducer,
    count: AtomicUsize,
}

impl MockJudgeClient {
    /// Construct from an arbitrary producer closure. Use this when
    /// the convenience constructors below do not cover the test
    /// scenario (for example, a stateful counter that returns a
    /// different verdict per call).
    pub fn from_fn<F>(producer: F) -> Self
    where
        F: Fn() -> Result<SemanticVerdict, JudgeError> + Send + Sync + 'static,
    {
        Self {
            producer: Box::new(producer),
            count: AtomicUsize::new(0),
        }
    }

    /// Every call returns a clone of `verdict`.
    #[must_use]
    pub fn always_returns(verdict: SemanticVerdict) -> Self {
        Self::from_fn(move || Ok(verdict.clone()))
    }

    /// Every call returns `JudgeError::MissingApiKey`.
    #[must_use]
    pub fn always_errors_missing_api_key() -> Self {
        Self::from_fn(|| Err(JudgeError::MissingApiKey))
    }

    /// Every call returns `JudgeError::Timeout`.
    #[must_use]
    pub fn always_errors_timeout() -> Self {
        Self::from_fn(|| Err(JudgeError::Timeout))
    }

    /// Every call returns `JudgeError::MalformedResponse(detail)`.
    #[must_use]
    pub fn always_errors_malformed(detail: impl Into<String>) -> Self {
        let detail = detail.into();
        Self::from_fn(move || Err(JudgeError::MalformedResponse(detail.clone())))
    }

    /// Every call returns `JudgeError::Other(message)`.
    #[must_use]
    pub fn always_errors_other(message: impl Into<String>) -> Self {
        let message = message.into();
        Self::from_fn(move || Err(JudgeError::Other(message.clone())))
    }

    /// Every call panics. Used to prove that the default verify
    /// path (no `--semantic-judge`) never invokes the judge — if a
    /// regression accidentally fires Layer-3 on the default path,
    /// the panic surfaces immediately and the test fails.
    #[must_use]
    pub fn panicking() -> Self {
        Self::from_fn(|| {
            panic!(
                "MockJudgeClient::panicking() was invoked — \
                 default-path-zero-network contract violated",
            );
        })
    }

    /// Number of `judge` invocations observed so far.
    #[must_use]
    pub fn call_count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

impl fmt::Debug for MockJudgeClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MockJudgeClient")
            .field("call_count", &self.call_count())
            .finish_non_exhaustive()
    }
}

#[async_trait::async_trait]
impl LlmJudgeClient for MockJudgeClient {
    async fn judge(
        &self,
        _chunk_hash: &str,
        _claim_window_hash: &str,
        _quote: &str,
        _chunk_text: &str,
    ) -> Result<SemanticVerdict, JudgeError> {
        self.count.fetch_add(1, Ordering::SeqCst);
        (self.producer)()
    }
}

// ----------------------------------------------------------------------
// HaikuClient
// ----------------------------------------------------------------------

/// Production implementation of [`LlmJudgeClient`] backed by
/// Anthropic's Claude Haiku via the `/v1/messages` endpoint.
///
/// The client is reusable across many `judge` calls — `reqwest`
/// pools connections internally. Live Anthropic API calls are
/// forbidden in standard CI gates. The HTTP path is exercised
/// instead by `cacg-cli`'s isolated wiremock integration test
/// (`tests/kb_verify_semantic_b2_wiremock.rs`, feature-gated),
/// which validates the Anthropic request shape and the
/// success + non-2xx response paths against an in-process
/// `MockServer`. The in-crate unit tests below cover only the
/// free [`parse_haiku_verdict`] helper and the `from_env` /
/// `with_components` constructors.
#[derive(Debug, Clone)]
pub struct HaikuClient {
    client: reqwest::Client,
    api_key: String,
    model: String,
    base_url: String,
    timeout: Duration,
}

impl HaikuClient {
    /// Construct from `ANTHROPIC_API_KEY` in the process
    /// environment, using the default model, base URL, and timeout.
    pub fn from_env() -> Result<Self, JudgeError> {
        let api_key = env::var("ANTHROPIC_API_KEY").map_err(|_| JudgeError::MissingApiKey)?;
        if api_key.is_empty() {
            return Err(JudgeError::MissingApiKey);
        }
        Ok(Self::with_components(
            api_key,
            DEFAULT_MODEL.to_string(),
            DEFAULT_API_BASE_URL.to_string(),
            DEFAULT_TIMEOUT,
        ))
    }

    /// Explicit constructor. Does NOT read any process-global
    /// environment variable. Useful for wiremock harnesses
    /// (override `base_url`) and for in-process unit tests
    /// (`api_key` can be a non-secret placeholder).
    #[must_use]
    pub fn with_components(
        api_key: String,
        model: String,
        base_url: String,
        timeout: Duration,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("reqwest client builder must succeed with rustls-tls");
        Self {
            client,
            api_key,
            model,
            base_url,
            timeout,
        }
    }

    /// Build a client from process env using the defaults, but do
    /// NOT fail when `ANTHROPIC_API_KEY` is absent or empty —
    /// surface the missing-key state at `judge()` time as a
    /// [`JudgeError::MissingApiKey`]. This is the constructor the
    /// CLI uses so a missing key produces a `CACG-VERIFY-002`
    /// diagnostic at Layer-3 firing time rather than a startup
    /// `CACG-MAN-001`.
    #[must_use]
    pub fn with_components_using_env_key() -> Self {
        let api_key = env::var("ANTHROPIC_API_KEY").unwrap_or_default(); // qg-allow: intentional-discard — missing key deferred to judge() time
        Self::with_components(
            api_key,
            DEFAULT_MODEL.to_string(),
            DEFAULT_API_BASE_URL.to_string(),
            DEFAULT_TIMEOUT,
        )
    }

    /// Configured model identifier.
    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Configured upstream base URL.
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Configured request timeout.
    #[must_use]
    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}

#[async_trait::async_trait]
impl LlmJudgeClient for HaikuClient {
    async fn judge(
        &self,
        _chunk_hash: &str,
        _claim_window_hash: &str,
        quote: &str,
        chunk_text: &str,
    ) -> Result<SemanticVerdict, JudgeError> {
        // Lazy empty-key check: a client built via
        // `with_components_using_env_key()` carries an empty
        // `api_key` when `ANTHROPIC_API_KEY` is absent. Short-
        // circuit BEFORE any HTTP work so the CLI surfaces the
        // missing key as a Layer-3 `CACG-VERIFY-002` diagnostic
        // instead of a startup `CACG-MAN-001`.
        if self.api_key.is_empty() {
            return Err(JudgeError::MissingApiKey);
        }
        let prompt = format!(
            "You are a semantic-grounding judge. Decide whether the QUOTE \
             is supported by the CHUNK_TEXT.\n\
             \n\
             CHUNK_TEXT:\n{chunk_text}\n\
             \n\
             QUOTE:\n{quote}\n\
             \n\
             Respond with strict JSON: \
             {{\"verdict\": \"pass\" | \"fail\" | \"abstain\", \
             \"score\": <number in 0.0..=1.0>, \
             \"reasoning\": \"<short justification>\"}}.\n\
             Do not include any text outside the JSON object."
        );

        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": 256,
            "messages": [
                {"role": "user", "content": prompt}
            ]
        });

        let response = self
            .client
            .post(&self.base_url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|e| format!("<body unreadable: {e}>"));
            return Err(JudgeError::MalformedResponse(format!(
                "HTTP {status}: {text}",
            )));
        }

        let payload: serde_json::Value = response.json().await.map_err(|e| {
            JudgeError::MalformedResponse(format!("response JSON parse error: {e}",))
        })?;

        parse_haiku_verdict(&payload)
    }
}

fn map_reqwest_error(e: reqwest::Error) -> JudgeError {
    if e.is_timeout() {
        JudgeError::Timeout
    } else {
        JudgeError::Http(e)
    }
}

/// Parse the Anthropic Messages API envelope into a
/// [`SemanticVerdict`]. The response shape is
/// `{ "content": [ { "type": "text", "text": "<verdict JSON>" }, ...] }`,
/// where `<verdict JSON>` is the strict-JSON object the prompt
/// requested.
fn parse_haiku_verdict(payload: &serde_json::Value) -> Result<SemanticVerdict, JudgeError> {
    let text = payload
        .get("content")
        .and_then(|c| c.get(0))
        .and_then(|first| first.get("text"))
        .and_then(|t| t.as_str())
        .ok_or_else(|| {
            JudgeError::MalformedResponse("response missing content[0].text".to_string())
        })?;

    let verdict_json: serde_json::Value = serde_json::from_str(text).map_err(|e| {
        JudgeError::MalformedResponse(format!("LLM response body is not strict JSON: {e}",))
    })?;

    let verdict_str = verdict_json
        .get("verdict")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            JudgeError::MalformedResponse("verdict field missing or not a string".to_string())
        })?;
    let kind = match verdict_str {
        "pass" => SemanticVerdictKind::Pass,
        "fail" => SemanticVerdictKind::Fail,
        "abstain" => SemanticVerdictKind::Abstain,
        other => {
            return Err(JudgeError::MalformedResponse(format!(
                "unknown verdict string: {other:?}",
            )));
        }
    };

    let score = verdict_json
        .get("score")
        .and_then(serde_json::Value::as_f64)
        .ok_or_else(|| {
            JudgeError::MalformedResponse("score field missing or not a number".to_string())
        })?;
    if !score.is_finite() || !(0.0..=1.0).contains(&score) {
        return Err(JudgeError::MalformedResponse(format!(
            "score {score} is not finite in [0.0, 1.0]",
        )));
    }

    let reasoning = verdict_json
        .get("reasoning")
        .and_then(|r| r.as_str())
        .map(str::to_string);

    Ok(SemanticVerdict {
        kind,
        score,
        reasoning,
        mode: SemanticMode::LlmJudge,
    })
}

// ----------------------------------------------------------------------
// B2Evaluator — sync `SemanticEvaluator` adapter over `LlmJudgeClient`
// ----------------------------------------------------------------------

/// Sync [`SemanticEvaluator`] adapter that owns a
/// [`LlmJudgeClient`] trait object and a current-thread Tokio
/// runtime, bridging the async `judge` call into the sync
/// `evaluate` boundary via `Runtime::block_on`.
///
/// Plugged into `verify_one_card` via `Option<&dyn SemanticEvaluator>`
/// just like the B1 `SemanticCache`. Errors from the underlying
/// client are mapped onto [`SemanticEvaluationError`] (a small
/// core-owned wrapper holding only the displayable error text);
/// the trust kernel never sees `JudgeError` or `reqwest::Error`.
pub struct B2Evaluator {
    client: Arc<dyn LlmJudgeClient + Send + Sync>,
    runtime: tokio::runtime::Runtime,
}

impl B2Evaluator {
    /// Construct from any `LlmJudgeClient` trait object wrapped in
    /// `Arc` (so the adapter can outlive the original constructor
    /// stack and the client can be shared across multiple
    /// adapters if needed). Builds a current-thread Tokio runtime
    /// internally — single-threaded is sufficient because the
    /// runner calls `evaluate` once per failing citation, never
    /// concurrently.
    pub fn new(client: Arc<dyn LlmJudgeClient + Send + Sync>) -> Result<Self, JudgeError> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| JudgeError::RuntimeBuild(format!("{e}")))?;
        Ok(Self { client, runtime })
    }

    /// Borrow the underlying client. Useful for tests asserting
    /// the configured mock's `call_count()`.
    #[must_use]
    pub fn client(&self) -> &(dyn LlmJudgeClient + Send + Sync) {
        self.client.as_ref()
    }
}

impl std::fmt::Debug for B2Evaluator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("B2Evaluator").finish_non_exhaustive()
    }
}

impl SemanticEvaluator for B2Evaluator {
    fn evaluate(
        &self,
        chunk_hash: &str,
        claim_window_hash: &str,
        quote: &str,
        chunk_text: &str,
    ) -> Result<SemanticVerdict, SemanticEvaluationError> {
        self.runtime
            .block_on(
                self.client
                    .judge(chunk_hash, claim_window_hash, quote, chunk_text),
            )
            .map_err(|judge_err| SemanticEvaluationError::new(judge_err.to_string()))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Module-local lock serializing `ANTHROPIC_API_KEY` mutation
    /// across tests in this module. Different process-global than
    /// the crate-wide `LOAD_TRACE_TEST_LOCK` (which guards
    /// `CACG_SEMANTIC_LOAD_TRACE`), so a fresh module-local lock is
    /// appropriate. See `BL-20260522-one-mutex-per-process-global-in-tests`.
    static API_KEY_LOCK: Mutex<()> = Mutex::new(());

    fn pass_verdict() -> SemanticVerdict {
        SemanticVerdict {
            kind: SemanticVerdictKind::Pass,
            score: 0.95,
            reasoning: Some("supported".into()),
            mode: SemanticMode::LlmJudge,
        }
    }

    #[tokio::test]
    async fn mock_always_returns_verdict_and_increments_call_count() {
        let mock = MockJudgeClient::always_returns(pass_verdict());
        assert_eq!(mock.call_count(), 0);
        let v = mock
            .judge("aaaa", "bbbb", "quote", "chunk")
            .await
            .expect("ok");
        assert_eq!(v.kind, SemanticVerdictKind::Pass);
        assert_eq!(v.score, 0.95);
        assert_eq!(v.mode, SemanticMode::LlmJudge);
        assert_eq!(mock.call_count(), 1);
        let _ = mock.judge("a", "b", "q", "c").await.expect("ok");
        let _ = mock.judge("a", "b", "q", "c").await.expect("ok");
        assert_eq!(mock.call_count(), 3);
    }

    #[tokio::test]
    async fn mock_always_errors_missing_api_key_propagates_error() {
        let mock = MockJudgeClient::always_errors_missing_api_key();
        let err = mock.judge("a", "b", "q", "c").await.unwrap_err();
        assert!(matches!(err, JudgeError::MissingApiKey));
    }

    #[tokio::test]
    async fn mock_always_errors_timeout_propagates_error() {
        let mock = MockJudgeClient::always_errors_timeout();
        let err = mock.judge("a", "b", "q", "c").await.unwrap_err();
        assert!(matches!(err, JudgeError::Timeout));
    }

    #[tokio::test]
    async fn mock_always_errors_malformed_propagates_detail() {
        let mock = MockJudgeClient::always_errors_malformed("oops");
        let err = mock.judge("a", "b", "q", "c").await.unwrap_err();
        match err {
            JudgeError::MalformedResponse(d) => assert_eq!(d, "oops"),
            JudgeError::MissingApiKey
            | JudgeError::Http(_)
            | JudgeError::Timeout
            | JudgeError::RuntimeBuild(_)
            | JudgeError::Other(_) => {
                panic!("expected MalformedResponse, got {err:?}")
            }
        }
    }

    #[tokio::test]
    async fn mock_always_errors_other_propagates_message() {
        let mock = MockJudgeClient::always_errors_other("rationale");
        let err = mock.judge("a", "b", "q", "c").await.unwrap_err();
        match err {
            JudgeError::Other(m) => assert_eq!(m, "rationale"),
            JudgeError::MissingApiKey
            | JudgeError::Http(_)
            | JudgeError::Timeout
            | JudgeError::MalformedResponse(_)
            | JudgeError::RuntimeBuild(_) => {
                panic!("expected Other, got {err:?}")
            }
        }
    }

    #[test]
    fn mock_panicking_panics_on_first_call() {
        let mock = MockJudgeClient::panicking();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio current-thread runtime must build");
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            runtime.block_on(mock.judge("a", "b", "q", "c"))
        }));
        assert!(result.is_err(), "panicking mock must panic on call");
    }

    #[test]
    fn judge_error_display_strings_are_stable() {
        assert_eq!(
            JudgeError::MissingApiKey.to_string(),
            "ANTHROPIC_API_KEY environment variable is not set",
        );
        assert_eq!(
            JudgeError::Timeout.to_string(),
            "LLM-judge request timed out",
        );
        assert_eq!(
            JudgeError::MalformedResponse("unparseable".into()).to_string(),
            "LLM-judge malformed response: unparseable",
        );
        assert_eq!(
            JudgeError::Other("context".into()).to_string(),
            "LLM-judge: context",
        );
    }

    #[test]
    fn judge_error_display_strings_do_not_leak_secrets_for_constructible_variants() {
        // Structural secret-non-leakage assertion. The constructible
        // `JudgeError` variants (`MissingApiKey`, `Timeout`,
        // `MalformedResponse`, `Other`) never carry the api_key —
        // their inputs are: nothing (MissingApiKey, Timeout), or a
        // caller-supplied detail string (MalformedResponse, Other).
        // `HaikuClient::judge` audits show no api_key references in
        // any of these construction paths. The `Http(reqwest::Error)`
        // variant is unreachable without actual HTTP, but reqwest's
        // Error::Display does not include header values by default;
        // the runtime end-to-end proof lives in `cacg-cli`'s
        // feature-gated wiremock test
        // `tests/kb_verify_semantic_b2_wiremock.rs::haiku_client_non_2xx_response_emits_verify_002_error_without_leaking_canary`,
        // which drives a real `HaikuClient` against an in-process
        // wiremock server returning HTTP 500 and asserts the
        // canary api_key never reaches the CACG-VERIFY-002
        // message or any of its hints.
        let canary = "sk-ant-test-leak-canary-XXXX";
        for err in [
            JudgeError::MissingApiKey,
            JudgeError::Timeout,
            JudgeError::MalformedResponse("response body had bad shape".into()),
            JudgeError::Other("config: bad value".into()),
        ] {
            let rendered = err.to_string();
            assert!(
                !rendered.contains(canary),
                "JudgeError variant Display leaked the api_key canary; rendered = {rendered:?}",
            );
        }
    }

    #[test]
    fn mock_judge_client_debug_format_includes_call_count() {
        let mock = MockJudgeClient::always_returns(pass_verdict());
        let formatted = format!("{mock:?}");
        assert!(
            formatted.contains("MockJudgeClient"),
            "Debug must name the struct; got {formatted:?}",
        );
        assert!(
            formatted.contains("call_count: 0"),
            "Debug must include initial call_count; got {formatted:?}",
        );
    }

    #[test]
    fn haiku_client_from_env_returns_missing_api_key_when_var_absent() {
        let _guard = API_KEY_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        std::env::remove_var("ANTHROPIC_API_KEY");
        match HaikuClient::from_env() {
            Err(JudgeError::MissingApiKey) => (),
            other => panic!("expected MissingApiKey, got {other:?}"),
        }
    }

    #[test]
    fn haiku_client_from_env_returns_missing_api_key_when_var_empty() {
        let _guard = API_KEY_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        std::env::set_var("ANTHROPIC_API_KEY", "");
        let result = HaikuClient::from_env();
        std::env::remove_var("ANTHROPIC_API_KEY");
        match result {
            Err(JudgeError::MissingApiKey) => (),
            other => panic!("expected MissingApiKey for empty key, got {other:?}"),
        }
    }

    #[test]
    fn haiku_client_with_components_does_not_read_env() {
        let _guard = API_KEY_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        std::env::remove_var("ANTHROPIC_API_KEY");
        let client = HaikuClient::with_components(
            "explicit-key".to_string(),
            "claude-haiku-test".to_string(),
            "http://localhost:9/v1/messages".to_string(),
            Duration::from_secs(5),
        );
        assert_eq!(client.model(), "claude-haiku-test");
        assert_eq!(client.base_url(), "http://localhost:9/v1/messages");
        assert_eq!(client.timeout(), Duration::from_secs(5));
    }

    #[test]
    fn parse_haiku_verdict_accepts_well_formed_pass() {
        let payload = serde_json::json!({
            "content": [
                {
                    "type": "text",
                    "text": "{\"verdict\":\"pass\",\"score\":0.87,\"reasoning\":\"matches\"}"
                }
            ]
        });
        let v = parse_haiku_verdict(&payload).expect("ok");
        assert_eq!(v.kind, SemanticVerdictKind::Pass);
        assert_eq!(v.score, 0.87);
        assert_eq!(v.reasoning.as_deref(), Some("matches"));
        assert_eq!(v.mode, SemanticMode::LlmJudge);
    }

    #[test]
    fn parse_haiku_verdict_rejects_missing_content() {
        let payload = serde_json::json!({"content": []});
        match parse_haiku_verdict(&payload) {
            Err(JudgeError::MalformedResponse(d)) => {
                assert!(d.contains("content[0].text"), "got: {d}");
            }
            other => panic!("expected MalformedResponse, got {other:?}"),
        }
    }

    #[test]
    fn parse_haiku_verdict_rejects_non_json_text() {
        let payload = serde_json::json!({
            "content": [
                {"type": "text", "text": "not json at all"}
            ]
        });
        match parse_haiku_verdict(&payload) {
            Err(JudgeError::MalformedResponse(d)) => {
                assert!(d.contains("strict JSON"), "got: {d}");
            }
            other => panic!("expected MalformedResponse, got {other:?}"),
        }
    }

    #[test]
    fn parse_haiku_verdict_rejects_unknown_verdict_string() {
        let payload = serde_json::json!({
            "content": [
                {"type": "text", "text": "{\"verdict\":\"maybe\",\"score\":0.5}"}
            ]
        });
        match parse_haiku_verdict(&payload) {
            Err(JudgeError::MalformedResponse(d)) => {
                assert!(d.contains("unknown verdict string"), "got: {d}");
            }
            other => panic!("expected MalformedResponse, got {other:?}"),
        }
    }

    #[test]
    fn parse_haiku_verdict_rejects_out_of_range_score() {
        let payload = serde_json::json!({
            "content": [
                {"type": "text", "text": "{\"verdict\":\"pass\",\"score\":1.5}"}
            ]
        });
        match parse_haiku_verdict(&payload) {
            Err(JudgeError::MalformedResponse(d)) => {
                assert!(d.contains("score 1.5"), "got: {d}");
            }
            other => panic!("expected MalformedResponse, got {other:?}"),
        }
    }

    #[test]
    fn parse_haiku_verdict_accepts_missing_reasoning() {
        let payload = serde_json::json!({
            "content": [
                {"type": "text", "text": "{\"verdict\":\"abstain\",\"score\":0.0}"}
            ]
        });
        let v = parse_haiku_verdict(&payload).expect("ok");
        assert_eq!(v.kind, SemanticVerdictKind::Abstain);
        assert_eq!(v.score, 0.0);
        assert!(v.reasoning.is_none());
    }
}
