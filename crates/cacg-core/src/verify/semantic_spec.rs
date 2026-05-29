//! Layer-3 semantic evaluator boundary.
//!
//! `cacg-core` is the trust kernel and owns the small data types and
//! the trait that `cacg-core::verify::verify_one_card` calls when a
//! Layer-2 citation fails and the caller has supplied a Layer-3
//! evaluator. Concrete evaluator implementations (cache-as-oracle B1,
//! LLM-judge B2) live in the optional `cacg-semantic` crate so the
//! trust kernel never gains a transitive dependency on async/network
//! surfaces.
//!
//! Mirrors `legacy_python_oracle/src/cacg/verify/semantic.py` on the wire-string surface
//! (`pass` / `fail` / `abstain`, `embedding-cache` / `llm-judge`).
//! See `_research/m5b_semantic_py_spec.md` §1 + §6 for the
//! line-by-line oracle.

use serde::{Deserialize, Serialize};

/// Closed-set verdict kind. Wire strings (`pass`, `fail`, `abstain`)
/// match Python `Literal["pass", "fail", "abstain"]`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SemanticVerdictKind {
    /// The verifier believes the quote is semantically supported.
    Pass,
    /// The verifier believes the quote is semantically unsupported.
    Fail,
    /// No verdict available (cache miss or deliberate abstain).
    Abstain,
}

impl SemanticVerdictKind {
    /// Lowercase wire string (`"pass"`, `"fail"`, `"abstain"`).
    #[must_use]
    pub fn as_wire(&self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Abstain => "abstain",
        }
    }
}

/// Which engine produced the verdict. Wire strings match Python
/// `SemanticMode = Literal["embedding-cache", "llm-judge"]`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SemanticMode {
    /// Cache-as-oracle (B1) — deterministic, offline.
    #[serde(rename = "embedding-cache")]
    EmbeddingCache,
    /// LLM-judge (B2) — feature-gated, async, network egress.
    #[serde(rename = "llm-judge")]
    LlmJudge,
}

impl SemanticMode {
    /// Wire string (`"embedding-cache"` or `"llm-judge"`).
    #[must_use]
    pub fn as_wire(&self) -> &'static str {
        match self {
            Self::EmbeddingCache => "embedding-cache",
            Self::LlmJudge => "llm-judge",
        }
    }
}

/// Verdict carrier returned by [`SemanticEvaluator::evaluate`].
#[derive(Clone, Debug, PartialEq)]
pub struct SemanticVerdict {
    /// Verdict outcome.
    pub kind: SemanticVerdictKind,
    /// Confidence score in `[0.0, 1.0]`.
    pub score: f64,
    /// Optional human-readable reasoning (always None for B1).
    pub reasoning: Option<String>,
    /// Engine that produced the verdict.
    pub mode: SemanticMode,
}

impl SemanticVerdict {
    /// The canonical cache-miss verdict: abstain with score 0.0,
    /// reasoning None, mode `embedding-cache`. Matches Python's
    /// `_cache_lookup` miss-return convention at
    /// `semantic.py:177-187`.
    #[must_use]
    pub fn abstain_embedding_cache() -> Self {
        Self {
            kind: SemanticVerdictKind::Abstain,
            score: 0.0,
            reasoning: None,
            mode: SemanticMode::EmbeddingCache,
        }
    }
}

/// Displayable evaluator error surfaced by [`SemanticEvaluator::evaluate`].
///
/// Holds only a `String` so the trust kernel never gains a
/// transitive dependency on concrete error types from the
/// semantic layer (`reqwest::Error`, `cacg_semantic::JudgeError`,
/// etc.). Adapters in `cacg-semantic` convert their crate-local
/// errors into this type via
/// `SemanticEvaluationError::new(err.to_string())`. The message
/// is embedded verbatim into the `CACG-VERIFY-002` diagnostic the
/// runner emits on the error branch, so adapter `Display` impls
/// must be stable and secret-free.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticEvaluationError {
    message: String,
}

impl SemanticEvaluationError {
    /// Construct from any displayable message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Borrow the underlying displayable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for SemanticEvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for SemanticEvaluationError {}

/// Layer-3 evaluator boundary. Implementations live in
/// `cacg-semantic` (B1 cache, B2 LLM-judge) and are passed to
/// `cacg_core::verify::verify_one_card` as `Option<&dyn SemanticEvaluator>`
/// so the trust kernel never gains a transitive dependency on the
/// semantic layer.
pub trait SemanticEvaluator: Send + Sync {
    /// Evaluate a `(chunk_hash, claim_window_hash)` pair given the
    /// raw `quote` and the resolved `chunk_text`. The B1 cache
    /// impl ignores `quote` and `chunk_text` (it answers from the
    /// pre-computed `(chunk_hash, claim_window_hash)` index); the
    /// B2 judge impl forwards `quote` + `chunk_text` to the LLM.
    /// On success the runner appends the verdict as a
    /// `CACG-VERIFY-002` diagnostic alongside the parent
    /// `CACG-VERIFY-001`; on error it appends `CACG-VERIFY-002`
    /// with severity `error` carrying the displayable
    /// `SemanticEvaluationError`.
    fn evaluate(
        &self,
        chunk_hash: &str,
        claim_window_hash: &str,
        quote: &str,
        chunk_text: &str,
    ) -> Result<SemanticVerdict, SemanticEvaluationError>;
}

/// SHA-256 (lowercase hex) over `normalize_text(quote).into_bytes()`.
///
/// Byte-equal with Python `cacg.verify.semantic._claim_window_hash`
/// because the underlying [`crate::normalize::normalize_text`] is
/// already proven byte-identical with Python's `normalize_text`
/// (the 54 `unicode_edge` parity fixtures + paired oracles enforce
/// it). Re-exported as `cacg_semantic::claim_window_hash` so callers
/// of the semantic layer keep the historical name.
#[must_use]
pub fn claim_window_hash(quote: &str) -> String {
    let normalized = crate::normalize::normalize_text(quote);
    crate::hash::source_sha256(normalized.as_bytes())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn verdict_kind_wire_strings_match_python() {
        assert_eq!(SemanticVerdictKind::Pass.as_wire(), "pass");
        assert_eq!(SemanticVerdictKind::Fail.as_wire(), "fail");
        assert_eq!(SemanticVerdictKind::Abstain.as_wire(), "abstain");
    }

    #[test]
    fn mode_wire_strings_match_python() {
        assert_eq!(SemanticMode::EmbeddingCache.as_wire(), "embedding-cache");
        assert_eq!(SemanticMode::LlmJudge.as_wire(), "llm-judge");
    }

    #[test]
    fn abstain_embedding_cache_constructor_matches_python_miss_convention() {
        let v = SemanticVerdict::abstain_embedding_cache();
        assert_eq!(v.kind, SemanticVerdictKind::Abstain);
        assert_eq!(v.score, 0.0);
        assert!(v.reasoning.is_none());
        assert_eq!(v.mode, SemanticMode::EmbeddingCache);
    }
}
