//! Shared single-card verify runner used by `kb verify` and
//! `kb verify --round-summary`.
//!
//! Centralizing the verify-one-card flow guarantees the
//! "exactly one journal event per cited card per invocation"
//! contract across both call sites. Without this helper, each
//! entry point would re-implement the layer-1 + layer-2 +
//! retraction-enforcement sequence plus the journal append,
//! opening the door to double-append and missing-append bugs.
//!
//! Mirrors Python `legacy_python_oracle/src/cacg/verify/runner.py::verify_one_card`
//! field-for-field on the trust-bearing surface (diagnostic codes,
//! emission order, journal payload shape). The optional 9th
//! `semantic: Option<&dyn SemanticEvaluator>` parameter threads a
//! caller-supplied Layer-3 evaluator (B1 cache or B2 LLM-judge from
//! `cacg-semantic`) into `verify_card`; the evaluator fires per the
//! semantic negative-firing contract enforced inside Layer-2 (only
//! on `CACG-VERIFY-001` failure, suppressed on AUTH / HASH / CITE /
//! RETR / CACG-MAN / missing-chunk short-circuits).
//!
//! The runner constructs one `Bm25HintCache` per verify pass and
//! threads it into `verify_card`, so a failed `CACG-VERIFY-001`
//! citation carries BM25 "did you mean" hints byte-equal with Python.

mod journal;
mod preflight;

use std::path::Path;
use std::time::Instant;

use crate::chunks_index::ChunksIndex;
use crate::diagnostic::{Diagnostic, Severity};
use crate::journal::JournalError;
use crate::lint::layer1::run_layer1_checks;
use crate::retraction::RetractionSpec;
use crate::source_matrix::AuthSpec;
use crate::verify::bm25_hints::Bm25HintCache;
use crate::verify::layer2::verify_card;
use crate::verify::semantic_spec::SemanticEvaluator;

use journal::{append_verify_event, retraction_diagnostic};
use preflight::{missing_card_preflight, resolve_card_doc, resolve_chunks_index};

/// Exit code surfaced by `kb verify` when every cited card
/// verifies. Mirrors Python `runner.EXIT_VERIFIED`.
pub const EXIT_VERIFIED: i32 = 0;

/// Exit code surfaced by `kb verify` when at least one card or
/// citation fails to verify. Mirrors Python `runner.EXIT_FAILED`.
pub const EXIT_FAILED: i32 = 1;

/// Result of a single-card verify pass. Mirrors Python
/// `VerifyOneCardResult`.
#[derive(Debug, Clone)]
pub struct VerifyOneCardResult {
    /// `true` iff `(!layer1_failed) && layer2_passed && (!retracted_failed)`.
    pub verified: bool,
    /// `true` iff Layer-1 either passed or was skipped via
    /// `skip_lint`.
    pub layer1: bool,
    /// `true` iff Layer-2 passed (no `Diagnostic` of severity
    /// `Error` from `verify_card`).
    pub layer2: bool,
    /// `true` iff at least one citation's verified state was
    /// reached via the fuzzy fallback.
    pub fuzzy: bool,
    /// Every diagnostic emitted across layer-1 + layer-2 +
    /// retraction enforcement, in emission order.
    pub diagnostics: Vec<Diagnostic>,
    /// The card's `card_hash` field as it appeared in the loaded
    /// frontmatter. `None` when the card failed to load.
    pub card_hash: Option<String>,
}

/// Error returned by [`verify_one_card`]. The runner funnels every
/// diagnostic-emission codepath through the same journal append
/// and returns `Ok(VerifyOneCardResult)`; only a journal-append
/// failure surfaces here.
pub type RunnerError = JournalError;

/// Run Layer 1 + Layer 2 against one card and append exactly one
/// `command="verify"` journal event.
///
/// Per-codepath cardinality is the trust-bearing pin: every
/// failure mode (missing card, layer-1 fail, malformed manifest,
/// layer-2 fail, retracted card) funnels into the same
/// `_append_verify_event` helper before returning, so the journal
/// always carries exactly one event per call.
///
/// # Parameters
///
/// * `card_path` — path to the card on disk. A missing or
///   non-file path emits `CACG-CLI-001` and still records a
///   journal event so the audit trail is complete.
/// * `chunks_manifest_path` — path to `chunks_manifest.json`.
///   Used only when `chunks_index` is `None`.
/// * `journal_path` — JSONL journal sink. Created if missing.
/// * `fuzzy` — opt-in to the dual-metric fuzzy fallback in
///   layer-2's content check.
/// * `skip_lint` — when `true`, skip the Layer-1 pass and run
///   only Layer-2; this is the `kb verify --skip-lint` path.
///   The trust-boundary checks layer-1 normally owns (AUTH +
///   source/chunk retraction) are routed through layer-2's
///   built-in re-enforcement to keep the trust contract intact.
/// * `chunks_index` — pre-built `ChunksIndex` shared across a
///   batch of cards. When `Some(_)`, layer-2 reuses the index
///   (and its memoized tamper cache) instead of re-loading the
///   manifest from disk. When `None`, the runner loads the
///   manifest per-card; on load failure each card still emits
///   one `CACG-MAN-001` journal event so per-card cardinality is
///   preserved.
/// * `auth` — `AuthSpec` for AUTH-* enforcement. Layer-1 owns
///   AUTH on the normal path; layer-2 owns it on `skip_lint`.
/// * `retraction` — `RetractionSpec` for `CACG-RETR-001`
///   per-card retraction enforcement. The runner emits the
///   diagnostic AFTER layer-2 on the successful path (Python
///   pin); on the malformed-manifest early-return path it also
///   emits retraction so the audit trail is complete.
///
/// # Errors
///
/// Returns [`RunnerError`] (a [`JournalError`]) only when the
/// journal append refuses or fails at the I/O layer. Diagnostic-
/// level failures are reported via `VerifyOneCardResult.diagnostics`.
pub fn verify_one_card(
    card_path: &Path,
    chunks_manifest_path: &Path,
    journal_path: &Path,
    fuzzy: bool,
    skip_lint: bool,
    chunks_index: Option<&ChunksIndex>,
    auth: Option<&AuthSpec>,
    retraction: Option<&RetractionSpec>,
    semantic: Option<&dyn SemanticEvaluator>,
    bm25_hint_cache: Option<&mut Bm25HintCache>,
) -> Result<VerifyOneCardResult, RunnerError> {
    let start = Instant::now();

    // Missing-card preflight (BL-20260518-shape-check-fs-inputs:
    // is_file() rather than exists() so a directory or dangling
    // symlink also routes through the CACG-CLI-001 path).
    if let Some(result) = missing_card_preflight(card_path, journal_path, start)? {
        return Ok(result);
    }

    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    let mut layer1_failed = false;
    let mut card_hash_value: Option<String> = None;
    let mut layer1_doc: Option<crate::card_loader::CardDoc> = None;

    if !skip_lint {
        // Capture the card_hash AND the parsed doc that Layer-1
        // saw so the journal records hash even on lint-failure
        // paths AND the layer-2 path can skip a redundant
        // load_card + YAML parse.
        let l1 = run_layer1_checks(card_path, chunks_manifest_path, chunks_index, auth);
        layer1_failed = l1
            .diagnostics
            .iter()
            .any(|d| matches!(d.severity, Severity::Error));
        diagnostics.extend(l1.diagnostics);
        card_hash_value = l1.card_hash_before;
        layer1_doc = l1.doc;
    }

    let mut layer2_passed = false;
    let mut used_fuzzy = false;
    let mut doc: Option<crate::card_loader::CardDoc> = layer1_doc.clone();

    if !layer1_failed {
        if doc.is_none() {
            // Either skip_lint=true (no layer-1 ran) or layer-1
            // didn't surface a doc (load_card failed earlier in
            // run_layer1_checks). Load now and surface the same
            // CardLoadError diagnostic path.
            match resolve_card_doc(card_path, journal_path, &mut diagnostics, start)? {
                Some(d) => {
                    doc = Some(d);
                }
                None => {
                    return Ok(VerifyOneCardResult {
                        verified: false,
                        layer1: false,
                        layer2: false,
                        fuzzy: false,
                        diagnostics,
                        card_hash: None,
                    });
                }
            }
        }
        // doc is Some(_) at this point.
        let loaded_doc = doc.as_ref().expect("doc populated above");
        card_hash_value = loaded_doc.frontmatter.card_hash.clone();

        // Resolve the index for layer-2: caller-supplied
        // batch-shared index OR per-card load.
        let resolved = resolve_chunks_index(
            chunks_index,
            chunks_manifest_path,
            card_path,
            journal_path,
            doc.as_ref(),
            card_hash_value.as_deref(),
            retraction,
            &mut diagnostics,
            start,
        )?;
        let l2_index = match resolved {
            Some(ri) => ri,
            None => {
                return Ok(VerifyOneCardResult {
                    verified: false,
                    layer1: false,
                    layer2: false,
                    fuzzy: false,
                    diagnostics,
                    card_hash: card_hash_value,
                });
            }
        };

        // Layer-2 AUTH routing: when --skip-lint bypassed
        // layer-1, ask layer-2 to enforce AUTH so the source-
        // authorization contract holds at the trust boundary.
        // Otherwise pass auth=None to avoid double-emitting
        // AUTH-* diagnostics.
        let l2_auth = if skip_lint { auth } else { None };
        let card_path_str = card_path.display().to_string();
        // BM25 hint cache: when the caller supplies a shared cache
        // (round-summary batch path), use it so N failed citations
        // against the same source build the BM25 corpus ONCE per
        // batch, mirroring Python `_cmd_verify_round_summary`'s
        // `BM25HintCache()` reuse. When the caller passes None
        // (single-card `kb verify`), allocate a fresh per-card
        // cache so single-card behavior is preserved unchanged.
        let mut owned_hint_cache = Bm25HintCache::new();
        let hint_cache_ref: &mut Bm25HintCache = match bm25_hint_cache {
            Some(shared) => shared,
            None => &mut owned_hint_cache,
        };
        let out = verify_card(
            loaded_doc,
            l2_index.as_ref(),
            fuzzy,
            Some(card_path_str.as_str()),
            l2_auth,
            Some(hint_cache_ref),
            semantic,
        );
        diagnostics.extend(out.diagnostics);
        used_fuzzy = out.used_fuzzy;
        layer2_passed = !out.failed;
    }

    // Retraction enforcement on the successful-verify path. The
    // malformed-manifest early-return above handles its own
    // retraction injection.
    let mut retracted_failed = false;
    if let Some(retr_d) = retraction_diagnostic(doc.as_ref(), card_path, retraction) {
        if matches!(retr_d.severity, Severity::Error) {
            retracted_failed = true;
        }
        diagnostics.push(retr_d);
    }

    let verified = !layer1_failed && layer2_passed && !retracted_failed;
    append_verify_event(
        journal_path,
        card_path,
        card_hash_value.as_deref(),
        &diagnostics,
        !layer1_failed,
        layer2_passed,
        used_fuzzy,
        start,
    )?;
    Ok(VerifyOneCardResult {
        verified,
        layer1: !layer1_failed,
        layer2: layer2_passed,
        fuzzy: used_fuzzy,
        diagnostics,
        card_hash: card_hash_value,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests;
