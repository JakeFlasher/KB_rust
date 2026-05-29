//! Layer-2 verify oracle + fuzzy primitives.
//!
//! `fuzzy` ships the oracle-critical metrics the dual-metric verify
//! gate combines: Ratcliff-Obershelp gestalt-pattern ratio mirroring
//! Python `difflib.SequenceMatcher.ratio()`, and a forthcoming bounded
//! Wagner-Fischer Levenshtein DP. Both metrics own their semantics
//! (no third-party crate for oracle-critical paths) and are
//! cross-validated byte-for-byte against the Python reference.
//!
//! `bm25_hints` produces the advisory BM25 "did you mean..." hints attached
//! to failed Layer-2 citations, scored over the in-house BM25 index.

pub mod bm25_hints;
pub mod fuzzy;
pub mod layer2;
pub mod runner;
pub mod semantic_spec;

// Re-export the runner API at the canonical
// `cacg_core::verify::*` path. The deeper `runner` module stays
// available for callers that want to be explicit; downstream CLI
// dispatchers and the round-summary driver should prefer the
// shorter path so the trust-kernel entrypoint
// (`cacg-core::verify::verify_one_card`) is the named surface.
pub use runner::{verify_one_card, RunnerError, VerifyOneCardResult, EXIT_FAILED, EXIT_VERIFIED};
pub use semantic_spec::{
    claim_window_hash, SemanticEvaluationError, SemanticEvaluator, SemanticMode, SemanticVerdict,
    SemanticVerdictKind,
};
