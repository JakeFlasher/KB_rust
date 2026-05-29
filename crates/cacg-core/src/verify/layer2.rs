//! Layer-2 mechanical content verification.
//!
//! `verify_card` is the canonical verify oracle: for every citation
//! on a card, look up the pinned chunk in the loaded `ChunksIndex`,
//! re-check the trust-boundary invariants (chunk lookup, source
//! agreement, manifest-tamper memoization), slice the chunk's
//! normalized text by code-point offsets within the cited page
//! range, and require exact substring containment of the normalized
//! citation quote. An opt-in fuzzy mode (`fuzzy_enabled = true`)
//! allows tiny Levenshtein-bounded drift via the dual-metric
//! [`crate::verify::fuzzy::fuzzy_match`] (Ratcliff-Obershelp ratio
//! gate + bounded Wagner-Fischer Levenshtein gate).
//!
//! The implementation mirrors Python
//! `legacy_python_oracle/src/cacg/verify/layer2.py::verify_card` field-for-field on the
//! trust-bearing diagnostic surface:
//!
//! * `CACG-HASH-001` (chunk missing or declared hash mismatch).
//! * `CACG-CITE-006` (citation source_id disagrees with manifest
//!   chunk).
//! * `CACG-HASH-003` (manifest text + page metadata no longer
//!   hashes to the stored `chunk_hash`).
//! * `CACG-RETR-002` / `CACG-RETR-003` (source / chunk retracted).
//! * `CACG-AUTH-000` / `CACG-AUTH-001` / `CACG-AUTH-002` (source-
//!   matrix load failure / unknown reading_id / disallowed
//!   source_id).
//! * `CACG-VERIFY-001` (containment failure; both exact and fuzzy
//!   gates rejected).
//!
//! A failed `CACG-VERIFY-001` citation carries BM25 "did you mean"
//! hints (`Diagnostic::hints`), scored over the chunks sharing the
//! citation's `source_id` — byte-equal with Python (the M5 retrieval
//! phase closed the former BM25-hints diagnostic-parity divergence).
//!
//! Layer-3 semantic verification is integrated via the optional
//! `semantic: Option<&dyn SemanticEvaluator>` parameter on
//! `verify_card` / `verify_citation`. When supplied, the evaluator
//! runs once per citation that reaches the `CACG-VERIFY-001` failure
//! branch, and the resulting `CACG-VERIFY-002` rides on the same
//! `diagnostics` array. AUTH-* / HASH-* / CITE-* / RETR-* /
//! CACG-MAN-* / missing-chunk short-circuits suppress the evaluator
//! (negative-firing contract). Concrete evaluator implementations
//! (B1 cache, B2 LLM-judge) live in `cacg-semantic` so the trust
//! kernel stays free of async / network surface.

use crate::card_loader::CardDoc;
use crate::chunks_index::ChunksIndex;
use crate::diagnostic::{codes, Diagnostic, Severity};
use crate::normalize::normalize_text;
use crate::schema::{ChunkRecord, Citation, PageSpan};
use crate::source_matrix::{is_citation_authorized, AuthSpec};
use crate::verify::bm25_hints::{self, Bm25HintCache};
use crate::verify::fuzzy::{fuzzy_match, DEFAULT_MAX_DIST, DEFAULT_MIN_RATIO};
use crate::verify::semantic_spec::{
    claim_window_hash, SemanticEvaluator, SemanticMode, SemanticVerdictKind,
};

/// Per-citation verification verdict. Mirrors Python `VerifyResult`.
#[derive(Debug, Clone)]
pub struct VerifyCitationResult {
    /// `true` iff the citation's normalized quote was found inside
    /// the cited page-window of the pinned chunk (exact match) OR
    /// the dual-metric fuzzy gate accepted a near-match.
    pub verified: bool,
    /// `true` iff the verification path went through the fuzzy
    /// fallback (always `false` when `verified == false`).
    pub fuzzy: bool,
    /// Diagnostics emitted for this citation. Empty on success.
    pub diagnostics: Vec<Diagnostic>,
}

/// Aggregate verdict for a card. Mirrors the tuple return shape of
/// Python `verify_card`: `(diagnostics, failed, used_fuzzy)`.
#[derive(Debug, Clone)]
pub struct VerifyCardOutput {
    /// Every diagnostic emitted while verifying the card, in
    /// emission order. Includes AUTH, retraction, per-citation
    /// trust-boundary, and `VERIFY-001` diagnostics as applicable.
    pub diagnostics: Vec<Diagnostic>,
    /// `true` iff at least one citation failed to verify OR an
    /// AUTH / retraction diagnostic was emitted.
    pub failed: bool,
    /// `true` iff at least one citation's verified state was
    /// reached via the fuzzy fallback.
    pub used_fuzzy: bool,
}

/// Compute the `[start_char, end_char)` slice of `chunk.text` that
/// the cited page range covers, using `chunk.page_spans`.
///
/// Mirrors Python `layer2._page_range_byte_window`. The schema field
/// `byte_offset_in_chunk` is documented as a CHARACTER offset; the
/// caller slices `chunk.text` by code-point offsets so the result
/// matches Python `chunk_text[w0:w1]` on Unicode chunks.
///
/// Returns `None` when no `page_span` falls in
/// `[page_start, page_end]` — a defensive path that Layer-1's
/// `CACG-CITE-005` intersection check should have rejected before
/// reaching Layer-2.
fn page_range_char_window(
    chunk: &ChunkRecord,
    page_start: u32,
    page_end: u32,
) -> Option<(usize, usize)> {
    let mut spans: Vec<&PageSpan> = chunk.page_spans.iter().collect();
    spans.sort_by_key(|s| s.byte_offset_in_chunk);
    let cited: Vec<usize> = spans
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            if page_start <= s.page && s.page <= page_end {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    if cited.is_empty() {
        return None;
    }
    let first = cited[0];
    let last = *cited
        .last()
        .expect("Invariant: cited is non-empty after is_empty check");
    let window_start = spans[first].byte_offset_in_chunk as usize;
    let window_end = if last + 1 < spans.len() {
        spans[last + 1].byte_offset_in_chunk as usize
    } else {
        chunk.text.chars().count()
    };
    Some((window_start, window_end))
}

/// Slice `text` by code-point offsets `[start, end)`. Caller
/// guarantees `start <= end <= text.chars().count()`.
fn slice_by_chars(text: &str, start: usize, end: usize) -> String {
    text.chars().skip(start).take(end - start).collect()
}

/// Verify one citation against the pinned chunk.
///
/// Mirrors Python `layer2.verify_citation`. The per-citation
/// diagnostic order is:
///
/// 1. chunk missing or declared hash mismatch → `CACG-HASH-001`.
/// 2. source_id disagrees with the manifest chunk → `CACG-CITE-006`.
/// 3. manifest tamper (recomputed hash != stored hash) →
///    `CACG-HASH-003`.
/// 4. exact containment → verified, no diagnostic.
/// 5. fuzzy match (only when `fuzzy_enabled`) → verified, no
///    diagnostic.
/// 6. otherwise → `CACG-VERIFY-001`, carrying up to
///    [`bm25_hints::DEFAULT_HINT_COUNT`] BM25 "did you mean" hints
///    scored over the chunks sharing this citation's `source_id`.
///
/// `hint_cache`, when supplied, builds each source's hint corpus at
/// most once per `kb verify` run. Hints are emitted on the failure
/// branch only and never change the verified verdict.
#[must_use]
pub fn verify_citation(
    citation: &Citation,
    index: &ChunksIndex,
    fuzzy_enabled: bool,
    file_path: Option<&str>,
    citation_index: usize,
    hint_cache: Option<&mut Bm25HintCache>,
    semantic: Option<&dyn SemanticEvaluator>,
) -> VerifyCitationResult {
    // Step 1: chunk lookup. Missing chunk OR declared-hash
    // disagreement both fall through to CACG-HASH-001 here; we do
    // NOT call `tamper_status` on a missing chunk so the "missing
    // manifest entry vs hash mismatch" categories stay distinct.
    let Some(chunk) = index.get(&citation.chunk_id) else {
        return VerifyCitationResult {
            verified: false,
            fuzzy: false,
            diagnostics: vec![diag(
                codes::HASH_001,
                format!(
                    "citations[{citation_index}]: chunk {chunk_id} missing or hash mismatch; \
                     rerun `kb lint` for the structural diagnostic",
                    chunk_id = citation.chunk_id
                ),
                file_path,
            )],
        };
    };
    if chunk.chunk_hash != citation.chunk_hash {
        return VerifyCitationResult {
            verified: false,
            fuzzy: false,
            diagnostics: vec![diag(
                codes::HASH_001,
                format!(
                    "citations[{citation_index}]: chunk {chunk_id} missing or hash mismatch; \
                     rerun `kb lint` for the structural diagnostic",
                    chunk_id = citation.chunk_id
                ),
                file_path,
            )],
        };
    }

    // Step 2: source agreement at the trust boundary (Python
    // catches this via Layer-1 CACG-CITE-006 normally; the repeat
    // here keeps `kb verify --skip-lint` honest).
    if citation.source_id != chunk.source_id {
        return VerifyCitationResult {
            verified: false,
            fuzzy: false,
            diagnostics: vec![diag(
                codes::CITE_006,
                format!(
                    "citations[{citation_index}]: source_id {source_id:?} does not match the \
                     chunk's source_id {chunk_source_id:?} in the manifest; chunk_id alone is \
                     not authoritative across sources",
                    source_id = citation.source_id,
                    chunk_source_id = chunk.source_id
                ),
                file_path,
            )],
        };
    }

    // Step 3: tamper detection. `tamper_status` is the memoized
    // entry point — it caches the boolean per chunk_id so repeat
    // citations to the same chunk are free. Errors from
    // `tamper_status` are unreachable on well-formed input (the
    // chunk lookup above already proved the chunk_id exists);
    // we collapse any unexpected error into the same HASH-003
    // diagnostic surface so the caller still sees a bounded code.
    let tamper_ok = index.tamper_status(&citation.chunk_id).unwrap_or(false);
    if !tamper_ok {
        return VerifyCitationResult {
            verified: false,
            fuzzy: false,
            diagnostics: vec![diag(
                codes::HASH_003,
                format!(
                    "citations[{citation_index}]: chunks_manifest.json text for chunk {chunk_id} \
                     disagrees with its stored chunk_hash; manifest may have been hand-edited",
                    chunk_id = citation.chunk_id
                ),
                file_path,
            )],
        };
    }

    // Step 4: page-window slice + normalized substring containment.
    let normalized_quote = normalize_text(&citation.quote);
    let chunk_text = chunk.text.as_str();
    // Citation page_range is a 2-element Vec<i64>; the schema
    // validator already pins the shape, but we defensively clamp
    // here to keep the function total.
    let page_start = citation.page_range.first().copied().unwrap_or(0).max(0) as u32;
    let page_end = citation.page_range.get(1).copied().unwrap_or(0).max(0) as u32;
    let window = page_range_char_window(chunk, page_start, page_end);
    let restricted_text = match window {
        Some((w0, w1)) => slice_by_chars(chunk_text, w0, w1),
        None => String::new(),
    };

    if restricted_text.contains(&normalized_quote) {
        return VerifyCitationResult {
            verified: true,
            fuzzy: false,
            diagnostics: Vec::new(),
        };
    }

    if fuzzy_enabled
        && fuzzy_match(
            &normalized_quote,
            &restricted_text,
            DEFAULT_MAX_DIST,
            DEFAULT_MIN_RATIO,
        )
    {
        return VerifyCitationResult {
            verified: true,
            fuzzy: true,
            diagnostics: Vec::new(),
        };
    }

    // Step 5: failure path. Distinguish "quote IS in pinned chunk
    // but outside cited page_range" from "quote not found at all".
    //
    // The trailing `(fuzzy=...)` token uses Python-style boolean
    // literals (`True` / `False`) so the message body byte-matches
    // Python's `f"(fuzzy={fuzzy_enabled})"`. Rust's default
    // `Display` for `bool` is lowercase; we route through this
    // helper to keep the parity contract honest on VERIFY-001.
    let fuzzy_repr = if fuzzy_enabled { "True" } else { "False" };
    let quote_in_full_chunk = chunk_text.contains(&normalized_quote);
    let message = if let (true, Some((w0, w1))) = (quote_in_full_chunk, window) {
        format!(
            "citations[{citation_index}]: quote IS in pinned chunk {chunk_id} but NOT within \
             cited page_range [{page_start}, {page_end}] (chunk byte window [{w0}, {w1}); widen \
             page_range or fix quote (fuzzy={fuzzy_repr})",
            chunk_id = citation.chunk_id
        )
    } else {
        format!(
            "citations[{citation_index}]: quote not found in pinned chunk {chunk_id} \
             (fuzzy={fuzzy_repr})",
            chunk_id = citation.chunk_id
        )
    };

    // BM25 "did you mean..." hints over the chunks sharing this
    // citation's `source_id`. Diagnostic-only: the hints ride on the
    // diagnostic but never flip the failed verdict (`verified` stays
    // `false` below). Python `verify_citation` passes the already-
    // `normalize_text`-ed quote to `top_k`; we do the same.
    let source_chunks = index.chunks_by_source(&citation.source_id);
    let hints = bm25_hints::top_k(
        &normalized_quote,
        &source_chunks,
        bm25_hints::DEFAULT_HINT_COUNT,
        hint_cache,
        Some(&citation.source_id),
    );
    let mut verify_diag = diag(codes::VERIFY_001, message, file_path);
    verify_diag.hints = hints
        .iter()
        .map(|h| serde_json::to_value(h).expect("a Hint always serializes to JSON"))
        .collect();
    let mut diagnostics = vec![verify_diag];

    // Layer-3 semantic check. Fires ONLY when Layer-2 exact /
    // fuzzy paths have both failed AND no upstream HASH-* /
    // CITE-* / AUTH-* / RETR-* / CACG-MAN-* short-circuited the
    // citation (the early returns above guarantee that), AND the
    // caller supplied a semantic evaluator. Mirrors Python
    // `layer2.py:272-349` callsite.
    if let Some(evaluator) = semantic {
        let claim_hash = claim_window_hash(&citation.quote);
        let semantic_diag = match evaluator.evaluate(
            &citation.chunk_hash,
            &claim_hash,
            &citation.quote,
            chunk_text,
        ) {
            Ok(verdict) => {
                // Per-verdict severity contract: `pass` -> Warning
                // (Layer-3 agrees but Layer-2 already failed),
                // `fail` -> Error (Layer-3 actively rejects),
                // `abstain` -> Info (cache miss / no signal, not an
                // error).
                let severity = match verdict.kind {
                    SemanticVerdictKind::Pass => Severity::Warning,
                    SemanticVerdictKind::Fail => Severity::Error,
                    SemanticVerdictKind::Abstain => Severity::Info,
                };
                let mut message = format!(
                    "semantic verdict={} score={:.6} mode={}",
                    verdict.kind.as_wire(),
                    verdict.score,
                    verdict.mode.as_wire(),
                );
                if let Some(reasoning) = verdict.reasoning.as_deref().filter(|s| !s.is_empty()) {
                    use std::fmt::Write;
                    let _ = write!(message, " reasoning={reasoning:?}");
                }
                let mut d = Diagnostic::new(codes::VERIFY_002, severity, message);
                if let Some(fp) = file_path {
                    d = d.with_file(fp.to_string());
                }
                let hint_value = serde_json::json!({
                    "semantic_verdict": verdict.kind.as_wire(),
                    "semantic_score": verdict.score,
                    "semantic_mode": verdict.mode.as_wire(),
                });
                d.hints = vec![hint_value];
                d
            }
            Err(eval_err) => {
                // Evaluator failure: surface as CACG-VERIFY-002 with
                // severity=Error. The displayable error text is
                // embedded verbatim; adapter `Display` impls in
                // cacg-semantic are stable and never leak secrets.
                // Mode is `llm-judge` because B1 (cache-as-oracle)
                // is infallible — any error here came from B2.
                let message = format!(
                    "semantic evaluator error mode={}: {eval_err}",
                    SemanticMode::LlmJudge.as_wire(),
                );
                let mut d = Diagnostic::new(codes::VERIFY_002, Severity::Error, message);
                if let Some(fp) = file_path {
                    d = d.with_file(fp.to_string());
                }
                let hint_value = serde_json::json!({
                    "semantic_error": eval_err.to_string(),
                    "semantic_mode": SemanticMode::LlmJudge.as_wire(),
                });
                d.hints = vec![hint_value];
                d
            }
        };
        diagnostics.push(semantic_diag);
    }

    VerifyCitationResult {
        verified: false,
        fuzzy: false,
        diagnostics,
    }
}

/// Verify every citation on a card. Mirrors Python
/// `layer2.verify_card`.
///
/// Diagnostic emission order:
///
/// 1. AUTH branch (only when `auth.is_some()`):
///    a. `auth.load_error.is_some()` → emit one `CACG-AUTH-000`,
///       mark `failed`, continue (subsequent citations still
///       walked for retraction + verify checks).
///    b. Otherwise, for each citation in order, emit
///       `CACG-AUTH-001` or `CACG-AUTH-002` per
///       `is_citation_authorized`.
/// 2. Per-citation: retraction first (`CACG-RETR-002` then
///    `CACG-RETR-003` short-circuit the citation), else
///    `verify_citation`.
#[must_use]
pub fn verify_card(
    doc: &CardDoc,
    index: &ChunksIndex,
    fuzzy_enabled: bool,
    file_path: Option<&str>,
    auth: Option<&AuthSpec>,
    mut hint_cache: Option<&mut Bm25HintCache>,
    semantic: Option<&dyn SemanticEvaluator>,
) -> VerifyCardOutput {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    let mut failed = false;
    let mut used_fuzzy = false;

    // Per-citation "Layer-3 must NOT fire for this citation" mask.
    // The semantic negative-firing contract requires the supplied
    // `SemanticEvaluator` to be skipped when an AUTH-* diagnostic has
    // already short-circuited the citation (HASH-*/CITE-*/missing-chunk
    // short-circuit naturally inside `verify_citation`; RETR-* short-
    // circuits with `continue` in the loop below). `auth.load_error`
    // blocks every citation on the card; per-citation AUTH-001 /
    // AUTH-002 blocks only that index.
    let mut auth_blocks_semantic: Vec<bool> = vec![false; doc.frontmatter.citations.len()];

    // AUTH branch (only fires when caller opted into it; layer-1
    // already emits AUTH-* on the normal path, so the runner
    // passes `auth=None` unless `--skip-lint` was set).
    if let Some(a) = auth {
        if let Some(err) = a.load_error.as_deref() {
            diagnostics.push(diag(codes::AUTH_000, err.to_string(), file_path));
            failed = true;
            for blocked in &mut auth_blocks_semantic {
                *blocked = true;
            }
        } else if let Some(matrix) = a.matrix.as_ref() {
            let reading_id = doc.frontmatter.reading_id.as_str();
            for (i, citation) in doc.frontmatter.citations.iter().enumerate() {
                let (ok, failure_code) =
                    is_citation_authorized(matrix, reading_id, &citation.source_id);
                if !ok {
                    let code = failure_code.unwrap_or(codes::AUTH_002);
                    let msg = if code == codes::AUTH_001 {
                        format!(
                            "citations[{i}]: reading_id {reading_id:?} not present in \
                             source_matrix; refusing to verify"
                        )
                    } else {
                        // qg-allow: deep-nesting — auth-check else arm
                        format!(
                            "citations[{i}]: source_id {source_id:?} not authorized for \
                             reading_id {reading_id:?} by source_matrix",
                            source_id = citation.source_id
                        )
                    };
                    diagnostics.push(diag(code, msg, file_path));
                    failed = true;
                    auth_blocks_semantic[i] = true;
                }
            }
        }
    }

    let retracted_source_ids = &index.manifest().retracted_source_ids;
    let retracted_chunk_ids = &index.manifest().retracted_chunk_ids;

    for (i, citation) in doc.frontmatter.citations.iter().enumerate() {
        if retracted_source_ids
            .iter()
            .any(|s| s == &citation.source_id)
        {
            diagnostics.push(diag(
                codes::RETR_002,
                format!(
                    "citations[{i}]: source_id {source_id:?} has been retracted (in \
                     chunks_manifest.retracted_source_ids); refusing to verify",
                    source_id = citation.source_id
                ),
                file_path,
            ));
            failed = true;
            continue;
        }
        if retracted_chunk_ids.iter().any(|c| c == &citation.chunk_id) {
            diagnostics.push(diag(
                codes::RETR_003,
                format!(
                    "citations[{i}]: chunk_id {chunk_id:?} has been retracted (in \
                     chunks_manifest.retracted_chunk_ids); refusing to verify",
                    chunk_id = citation.chunk_id
                ),
                file_path,
            ));
            failed = true;
            continue;
        }
        // `as_deref_mut` reborrows the optional cache each iteration so
        // every citation on the card shares the one per-run cache.
        // Pass `None` for `semantic` when AUTH already short-circuited
        // this citation so the Layer-3 evaluator never inspects a
        // citation the source-matrix gate rejected.
        let semantic_for_citation = if auth_blocks_semantic.get(i).copied().unwrap_or(false) {
            None
        } else {
            semantic
        };
        let result = verify_citation(
            citation,
            index,
            fuzzy_enabled,
            file_path,
            i,
            hint_cache.as_deref_mut(),
            semantic_for_citation,
        );
        diagnostics.extend(result.diagnostics);
        if result.fuzzy {
            used_fuzzy = true;
        }
        if !result.verified {
            failed = true;
        }
    }

    VerifyCardOutput {
        diagnostics,
        failed,
        used_fuzzy,
    }
}

fn diag(code: &'static str, message: String, file: Option<&str>) -> Diagnostic {
    let d = Diagnostic::new(code, Severity::Error, message);
    if let Some(f) = file {
        d.with_file(f.to_string())
    } else {
        d
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::hash::{chunk_hash, PageSpan as HashPageSpan};
    use crate::schema::{
        CardEdge, CardFrontmatter, ChunksManifest, CitationEdge, SchemaVersion, SourceMatrix,
    };
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    fn hash_pagespan(span: &PageSpan) -> HashPageSpan {
        HashPageSpan {
            page: i64::from(span.page),
            byte_offset_in_chunk: i64::from(span.byte_offset_in_chunk),
        }
    }

    fn make_chunk(
        source_id: &str,
        chunk_id: &str,
        text: &str,
        start_page: u32,
        end_page: u32,
        page_spans: Vec<PageSpan>,
    ) -> ChunkRecord {
        let hash_spans: Vec<HashPageSpan> = page_spans.iter().map(hash_pagespan).collect();
        let h = chunk_hash(
            text,
            i64::from(start_page),
            i64::from(end_page),
            &hash_spans,
        )
        .expect("chunk_hash");
        ChunkRecord {
            schema_version: SchemaVersion::V0,
            source_id: source_id.to_string(),
            chunk_id: chunk_id.to_string(),
            chunk_hash: h,
            ordinal: 0,
            start_page,
            end_page,
            page_spans,
            token_count: 3,
            text: text.to_string(),
            text_preview: text.chars().take(20).collect(),
        }
    }

    fn one_page_chunk(source_id: &str, chunk_id: &str, text: &str, page: u32) -> ChunkRecord {
        make_chunk(
            source_id,
            chunk_id,
            text,
            page,
            page,
            vec![PageSpan {
                page,
                byte_offset_in_chunk: 0,
            }],
        )
    }

    fn manifest_with(
        chunks: Vec<ChunkRecord>,
        retracted_sources: Vec<String>,
        retracted_chunks: Vec<String>,
    ) -> ChunksManifest {
        ChunksManifest {
            schema_version: SchemaVersion::V0,
            chunks,
            retracted_source_ids: retracted_sources,
            retracted_chunk_ids: retracted_chunks,
        }
    }

    fn citation_for(chunk: &ChunkRecord, quote: &str) -> Citation {
        Citation {
            source_id: chunk.source_id.clone(),
            chunk_id: chunk.chunk_id.clone(),
            chunk_hash: chunk.chunk_hash.clone(),
            page_range: vec![i64::from(chunk.start_page), i64::from(chunk.end_page)],
            quote: quote.to_string(),
            edge_type: CitationEdge::Supports,
        }
    }

    fn card_doc_with(reading_id: &str, citations: Vec<Citation>) -> CardDoc {
        CardDoc {
            path: PathBuf::from("/dev/null/test-card.md"),
            frontmatter: CardFrontmatter {
                schema_version: SchemaVersion::V0,
                id: "test-card".to_string(),
                title: "test title".to_string(),
                reading_id: reading_id.to_string(),
                summary: "summary ".repeat(20),
                tags: Vec::new(),
                card_edges: Vec::<CardEdge>::new(),
                citations,
                card_hash: None,
            },
            body_raw: String::new(),
            body_normalized: String::new(),
        }
    }

    #[test]
    fn verify_citation_exact_match_returns_verified_no_diagnostics() {
        let chunk = one_page_chunk("src", "src:p001:0000", "alpha beta gamma", 1);
        let cit = citation_for(&chunk, "beta");
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let r = verify_citation(&cit, &index, false, None, 0, None, None);
        assert!(r.verified);
        assert!(!r.fuzzy);
        assert!(r.diagnostics.is_empty());
    }

    #[test]
    fn verify_citation_chunk_hash_mismatch_emits_hash_001() {
        let chunk = one_page_chunk("src", "src:p001:0000", "alpha beta gamma", 1);
        let mut cit = citation_for(&chunk, "beta");
        cit.chunk_hash = "0".repeat(64); // valid hex but wrong
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let r = verify_citation(&cit, &index, false, None, 0, None, None);
        assert!(!r.verified);
        assert_eq!(r.diagnostics.len(), 1);
        assert_eq!(r.diagnostics[0].code, codes::HASH_001);
    }

    #[test]
    fn verify_citation_missing_chunk_emits_hash_001_not_hash_003() {
        // Pinned regression: a citation referencing an unknown
        // chunk_id must surface CACG-HASH-001, never CACG-HASH-003.
        // The Layer-2 flow checks chunk existence before any
        // tamper_status call, so the missing-chunk branch never
        // reaches the tamper cache.
        let chunk = one_page_chunk("src", "src:p001:0000", "alpha beta gamma", 1);
        let mut cit = citation_for(&chunk, "beta");
        cit.chunk_id = "src:p999:0000".to_string();
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let r = verify_citation(&cit, &index, false, None, 0, None, None);
        assert!(!r.verified);
        assert_eq!(r.diagnostics.len(), 1);
        assert_eq!(r.diagnostics[0].code, codes::HASH_001);
        for d in &r.diagnostics {
            assert_ne!(
                d.code,
                codes::HASH_003,
                "HASH-003 must not fire on missing chunk"
            );
        }
    }

    #[test]
    fn verify_citation_source_id_mismatch_emits_cite_006() {
        let chunk = one_page_chunk("src-actual", "x:p001:0000", "alpha beta gamma", 1);
        let mut cit = citation_for(&chunk, "beta");
        cit.source_id = "src-claimed".to_string();
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let r = verify_citation(&cit, &index, false, None, 0, None, None);
        assert!(!r.verified);
        assert_eq!(r.diagnostics.len(), 1);
        assert_eq!(r.diagnostics[0].code, codes::CITE_006);
    }

    #[test]
    fn verify_citation_tampered_text_emits_hash_003() {
        // Build a chunk, then directly mutate the manifest's text so
        // the recomputed hash diverges from the stored chunk_hash.
        let mut chunk = one_page_chunk("src", "src:p001:0000", "alpha beta gamma", 1);
        let stored_hash = chunk.chunk_hash.clone();
        chunk.text = "alpha beta DELTA".to_string();
        // Citation references the stored hash (so the citation's
        // chunk_hash agrees with the manifest's stored hash, but
        // the manifest's text no longer hashes to that value).
        let cit = Citation {
            source_id: chunk.source_id.clone(),
            chunk_id: chunk.chunk_id.clone(),
            chunk_hash: stored_hash,
            page_range: vec![1, 1],
            quote: "beta".to_string(),
            edge_type: CitationEdge::Supports,
        };
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let r = verify_citation(&cit, &index, false, None, 0, None, None);
        assert!(!r.verified);
        assert_eq!(r.diagnostics.len(), 1);
        assert_eq!(r.diagnostics[0].code, codes::HASH_003);
    }

    #[test]
    fn verify_citation_quote_outside_page_window_emits_verify_001() {
        // Multi-page chunk: page 1 holds "alpha", page 2 holds
        // "omega". Citation claims page_range=[2,2] but quotes
        // "alpha" — quote IS in the chunk but NOT in the page-2
        // window. Verify_001 fires with the more informative
        // "quote IS in pinned chunk but NOT within cited page_range"
        // message shape.
        let text = "alpha\nomega"; // 11 chars, '\n' at index 5
        let chunk = make_chunk(
            "src",
            "src:p001:0000",
            text,
            1,
            2,
            vec![
                PageSpan {
                    page: 1,
                    byte_offset_in_chunk: 0,
                },
                PageSpan {
                    page: 2,
                    byte_offset_in_chunk: 6,
                },
            ],
        );
        let cit = Citation {
            source_id: chunk.source_id.clone(),
            chunk_id: chunk.chunk_id.clone(),
            chunk_hash: chunk.chunk_hash.clone(),
            page_range: vec![2, 2],
            quote: "alpha".to_string(),
            edge_type: CitationEdge::Supports,
        };
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let r = verify_citation(&cit, &index, false, None, 0, None, None);
        assert!(!r.verified);
        assert_eq!(r.diagnostics.len(), 1);
        assert_eq!(r.diagnostics[0].code, codes::VERIFY_001);
        assert!(
            r.diagnostics[0]
                .message
                .contains("quote IS in pinned chunk"),
            "expected page-window message; got {:?}",
            r.diagnostics[0].message
        );
    }

    #[test]
    fn verify_citation_quote_not_in_chunk_emits_verify_001() {
        let chunk = one_page_chunk("src", "src:p001:0000", "alpha beta gamma", 1);
        let mut cit = citation_for(&chunk, "completely absent text");
        cit.quote = "completely absent text".to_string();
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let r = verify_citation(&cit, &index, false, None, 0, None, None);
        assert!(!r.verified);
        assert_eq!(r.diagnostics.len(), 1);
        assert_eq!(r.diagnostics[0].code, codes::VERIFY_001);
        assert!(
            r.diagnostics[0]
                .message
                .contains("quote not found in pinned chunk"),
            "expected plain not-found message; got {:?}",
            r.diagnostics[0].message
        );
    }

    #[test]
    fn verify_citation_fuzzy_match_returns_verified_fuzzy_true() {
        // Long enough quote that a single-character substitution
        // keeps the ratio above 0.95 AND distance within 2.
        let text = "the quick brown fox jumps over the lazy dog and runs fast";
        let chunk = one_page_chunk("src", "src:p001:0000", text, 1);
        let cit = Citation {
            source_id: chunk.source_id.clone(),
            chunk_id: chunk.chunk_id.clone(),
            chunk_hash: chunk.chunk_hash.clone(),
            page_range: vec![1, 1],
            quote: "the quick brown fox jumps over the lazy dox".to_string(),
            edge_type: CitationEdge::Supports,
        };
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let r = verify_citation(&cit, &index, true, None, 0, None, None);
        assert!(
            r.verified,
            "fuzzy match should succeed for single-char drift"
        );
        assert!(r.fuzzy);
        assert!(r.diagnostics.is_empty());
    }

    #[test]
    fn verify_citation_fuzzy_disabled_still_emits_verify_001_on_drift() {
        let text = "the quick brown fox jumps over the lazy dog and runs fast";
        let chunk = one_page_chunk("src", "src:p001:0000", text, 1);
        let cit = Citation {
            source_id: chunk.source_id.clone(),
            chunk_id: chunk.chunk_id.clone(),
            chunk_hash: chunk.chunk_hash.clone(),
            page_range: vec![1, 1],
            quote: "the quick brown fox jumps over the lazy dox".to_string(),
            edge_type: CitationEdge::Supports,
        };
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let r = verify_citation(&cit, &index, false, None, 0, None, None);
        assert!(!r.verified);
        assert_eq!(r.diagnostics.len(), 1);
        assert_eq!(r.diagnostics[0].code, codes::VERIFY_001);
    }

    #[test]
    fn verify_citation_unicode_quote_through_code_point_slicing() {
        // CJK chunk: 床前明月光\n疑是地上霜
        //   page 1 → "床前明月光" (5 chars), page 2 → "疑是地上霜" (5 chars).
        // PageSpan byte_offset_in_chunk values are CHARACTER offsets
        // per the Rust schema's documented naming caveat.
        let text = "床前明月光\n疑是地上霜";
        let chunk = make_chunk(
            "src",
            "src:p001:0000",
            text,
            1,
            2,
            vec![
                PageSpan {
                    page: 1,
                    byte_offset_in_chunk: 0,
                },
                PageSpan {
                    page: 2,
                    byte_offset_in_chunk: 6,
                },
            ],
        );
        let cit = Citation {
            source_id: chunk.source_id.clone(),
            chunk_id: chunk.chunk_id.clone(),
            chunk_hash: chunk.chunk_hash.clone(),
            page_range: vec![2, 2],
            quote: "疑是地上霜".to_string(),
            edge_type: CitationEdge::Supports,
        };
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let r = verify_citation(&cit, &index, false, None, 0, None, None);
        assert!(r.verified, "CJK substring on page-2 window should verify");
        assert!(!r.fuzzy);
    }

    #[test]
    fn verify_card_clean_card_returns_no_diagnostics_and_no_failed() {
        let chunk = one_page_chunk("src", "src:p001:0000", "alpha beta gamma", 1);
        let cit = citation_for(&chunk, "beta");
        let doc = card_doc_with("R1", vec![cit]);
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let out = verify_card(&doc, &index, false, None, None, None, None);
        assert!(out.diagnostics.is_empty());
        assert!(!out.failed);
        assert!(!out.used_fuzzy);
    }

    #[test]
    fn verify_card_auth_load_error_emits_one_auth_000_per_card() {
        let chunk = one_page_chunk("src", "src:p001:0000", "alpha beta gamma", 1);
        let cit = citation_for(&chunk, "beta");
        let doc = card_doc_with("R1", vec![cit]);
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let auth = AuthSpec {
            matrix: None,
            load_error: Some("source_matrix.json is invalid".to_string()),
        };
        let out = verify_card(&doc, &index, false, None, Some(&auth), None, None);
        assert!(out.failed);
        let auth_diags: Vec<_> = out
            .diagnostics
            .iter()
            .filter(|d| d.code == codes::AUTH_000)
            .collect();
        assert_eq!(
            auth_diags.len(),
            1,
            "exactly one CACG-AUTH-000 per card on load_error"
        );
    }

    #[test]
    fn verify_card_auth_unauthorized_emits_auth_002_per_offending_citation() {
        let chunk = one_page_chunk("src-disallowed", "x:p001:0000", "alpha beta gamma", 1);
        let cit = citation_for(&chunk, "beta");
        let doc = card_doc_with("R1", vec![cit]);
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let mut allowed: BTreeMap<String, Vec<String>> = BTreeMap::new();
        allowed.insert("R1".to_string(), vec!["src-allowed".to_string()]);
        let matrix = SourceMatrix {
            schema_version: SchemaVersion::V0,
            allowed,
        };
        let auth = AuthSpec {
            matrix: Some(matrix),
            load_error: None,
        };
        let out = verify_card(&doc, &index, false, None, Some(&auth), None, None);
        assert!(out.failed);
        let auth_diags: Vec<_> = out
            .diagnostics
            .iter()
            .filter(|d| d.code == codes::AUTH_002)
            .collect();
        assert_eq!(auth_diags.len(), 1);
    }

    #[test]
    fn verify_card_auth_unknown_reading_id_emits_auth_001() {
        let chunk = one_page_chunk("src", "x:p001:0000", "alpha beta gamma", 1);
        let cit = citation_for(&chunk, "beta");
        let doc = card_doc_with("unknown-reading", vec![cit]);
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let mut allowed: BTreeMap<String, Vec<String>> = BTreeMap::new();
        allowed.insert("R1".to_string(), vec!["src".to_string()]);
        let matrix = SourceMatrix {
            schema_version: SchemaVersion::V0,
            allowed,
        };
        let auth = AuthSpec {
            matrix: Some(matrix),
            load_error: None,
        };
        let out = verify_card(&doc, &index, false, None, Some(&auth), None, None);
        assert!(out.failed);
        let auth_diags: Vec<_> = out
            .diagnostics
            .iter()
            .filter(|d| d.code == codes::AUTH_001)
            .collect();
        assert_eq!(auth_diags.len(), 1);
    }

    #[test]
    fn verify_card_retracted_source_emits_retr_002_and_skips_citation_check() {
        // The manifest validator enforces disjointness between
        // active chunks' source_ids and `retracted_source_ids`.
        // Stage a second active source so the manifest is valid,
        // and point the citation at the retracted source (with the
        // citation's source_id appearing only in
        // retracted_source_ids — the chunk lookup will fall
        // through to HASH-001 in the absence of the RETR-002
        // short-circuit, so the diagnostic shape is decisively
        // RETR-002 only).
        let active_chunk = one_page_chunk("src-active", "src-active:p001:0000", "alpha", 1);
        let retracted_citation = Citation {
            source_id: "src-retracted".to_string(),
            chunk_id: "src-retracted:p001:0000".to_string(),
            chunk_hash: "0".repeat(64),
            page_range: vec![1, 1],
            quote: "alpha".to_string(),
            edge_type: CitationEdge::Supports,
        };
        let doc = card_doc_with("R1", vec![retracted_citation]);
        let index = ChunksIndex::from_manifest(manifest_with(
            vec![active_chunk],
            vec!["src-retracted".to_string()],
            vec![],
        ))
        .unwrap();
        let out = verify_card(&doc, &index, false, None, None, None, None);
        assert!(out.failed);
        assert_eq!(out.diagnostics.len(), 1);
        assert_eq!(out.diagnostics[0].code, codes::RETR_002);
    }

    #[test]
    fn verify_card_retracted_chunk_emits_retr_003_and_skips_citation_check() {
        // Same disjointness story: retract a chunk_id that's not
        // in the active chunk list. Citation references the
        // retracted chunk_id; RETR-003 short-circuits before any
        // HASH-001 falls through on the missing-chunk path.
        let active_chunk = one_page_chunk("src", "src:p001:0000", "alpha", 1);
        let retracted_citation = Citation {
            source_id: "src".to_string(),
            chunk_id: "src:p999:0000".to_string(),
            chunk_hash: "0".repeat(64),
            page_range: vec![999, 999],
            quote: "alpha".to_string(),
            edge_type: CitationEdge::Supports,
        };
        let doc = card_doc_with("R1", vec![retracted_citation]);
        let index = ChunksIndex::from_manifest(manifest_with(
            vec![active_chunk],
            vec![],
            vec!["src:p999:0000".to_string()],
        ))
        .unwrap();
        let out = verify_card(&doc, &index, false, None, None, None, None);
        assert!(out.failed);
        assert_eq!(out.diagnostics.len(), 1);
        assert_eq!(out.diagnostics[0].code, codes::RETR_003);
    }

    #[test]
    fn verify_card_used_fuzzy_reports_true_when_any_citation_fuzzy_matched() {
        let text = "the quick brown fox jumps over the lazy dog and runs fast";
        let chunk = one_page_chunk("src", "src:p001:0000", text, 1);
        let cit = Citation {
            source_id: chunk.source_id.clone(),
            chunk_id: chunk.chunk_id.clone(),
            chunk_hash: chunk.chunk_hash.clone(),
            page_range: vec![1, 1],
            quote: "the quick brown fox jumps over the lazy dox".to_string(),
            edge_type: CitationEdge::Supports,
        };
        let doc = card_doc_with("R1", vec![cit]);
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let out = verify_card(&doc, &index, true, None, None, None, None);
        assert!(!out.failed);
        assert!(out.used_fuzzy);
        assert!(out.diagnostics.is_empty());
    }

    #[test]
    fn verify_card_emits_bm25_hints_on_verify_001() {
        // A failed Layer-2 citation carries BM25 "did you mean" hints
        // on its `CACG-VERIFY-001` diagnostic, scored
        // over the chunks sharing the citation's `source_id`. Each
        // hint serializes as a four-field object
        // (`hint_only`, `chunk_id`, `score`, `text_preview`), and
        // the diagnostic's serialized form carries a `hints` key.
        let chunk = one_page_chunk("src", "src:p001:0000", "alpha beta gamma", 1);
        let cit = Citation {
            source_id: chunk.source_id.clone(),
            chunk_id: chunk.chunk_id.clone(),
            chunk_hash: chunk.chunk_hash.clone(),
            page_range: vec![1, 1],
            quote: "completely absent text here".to_string(),
            edge_type: CitationEdge::Supports,
        };
        let doc = card_doc_with("R1", vec![cit]);
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let out = verify_card(&doc, &index, false, None, None, None, None);
        assert_eq!(out.diagnostics.len(), 1);
        assert_eq!(out.diagnostics[0].code, codes::VERIFY_001);
        let hints = &out.diagnostics[0].hints;
        assert!(!hints.is_empty(), "VERIFY-001 must carry BM25 hints");
        for h in hints {
            let obj = h.as_object().expect("each hint is a JSON object");
            assert_eq!(
                obj.get("hint_only").and_then(serde_json::Value::as_bool),
                Some(true)
            );
            assert!(obj
                .get("chunk_id")
                .and_then(serde_json::Value::as_str)
                .is_some());
            assert!(obj
                .get("score")
                .and_then(serde_json::Value::as_f64)
                .is_some());
            assert!(obj
                .get("text_preview")
                .and_then(serde_json::Value::as_str)
                .is_some());
        }
        // The serialized diagnostic now carries a populated `hints` key.
        let serialized = serde_json::to_value(&out.diagnostics[0]).unwrap();
        assert!(
            serialized.get("hints").is_some(),
            "VERIFY-001 must serialize a `hints` key when hints are populated; got {serialized}"
        );
    }

    #[test]
    fn verify_card_passing_citation_emits_no_hints() {
        // A passing citation emits no diagnostic at all, so there is
        // no hint payload — hints ride only on failure.
        let chunk = one_page_chunk("src", "src:p001:0000", "alpha beta gamma", 1);
        let cit = citation_for(&chunk, "beta");
        let doc = card_doc_with("R1", vec![cit]);
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let out = verify_card(&doc, &index, false, None, None, None, None);
        assert!(!out.failed);
        assert!(out.diagnostics.is_empty());
    }

    // ----------------------------------------------------------------
    // Layer-3 suppression on per-citation short-circuits.
    //
    // The semantic negative-firing contract: Layer-3 does NOT fire on
    // `HASH-* / CITE-* / AUTH-* / RETR-* / CACG-MAN-* / missing-chunk`
    // failures, nor when Layer-2's exact or fuzzy paths accept. The
    // AUTH branch is the load-bearing one because Layer-2 routes
    // AUTH enforcement here on `--skip-lint`, so an unauthorized
    // citation can still reach `verify_citation` and (without
    // suppression) trip the supplied evaluator. These tests use a
    // counting evaluator to pin both that no `CACG-VERIFY-002` is
    // emitted on the suppressed paths AND that the evaluator is
    // never invoked.
    // ----------------------------------------------------------------

    use crate::verify::semantic_spec::{
        SemanticEvaluationError, SemanticMode, SemanticVerdict, SemanticVerdictKind,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Stub `SemanticEvaluator` that counts invocations and returns a
    /// fixed verdict. The counter is `AtomicUsize` so the trait's
    /// `&self` shape works without `Mutex`.
    struct CountingEvaluator {
        count: AtomicUsize,
        verdict: SemanticVerdictKind,
    }

    impl CountingEvaluator {
        fn new(verdict: SemanticVerdictKind) -> Self {
            Self {
                count: AtomicUsize::new(0),
                verdict,
            }
        }
        fn count(&self) -> usize {
            self.count.load(Ordering::SeqCst)
        }
    }

    impl SemanticEvaluator for CountingEvaluator {
        fn evaluate(
            &self,
            _chunk_hash: &str,
            _claim_window_hash: &str,
            _quote: &str,
            _chunk_text: &str,
        ) -> Result<SemanticVerdict, SemanticEvaluationError> {
            self.count.fetch_add(1, Ordering::SeqCst);
            Ok(SemanticVerdict {
                kind: self.verdict,
                score: 0.42,
                reasoning: None,
                mode: SemanticMode::EmbeddingCache,
            })
        }
    }

    /// Build a card whose only citation would have entered the
    /// CACG-VERIFY-001 branch (so Layer-3 firing IS the question).
    fn build_layer2_failing_card(reading_id: &str, source_id: &str) -> (CardDoc, ChunksIndex) {
        let chunk = one_page_chunk(
            source_id,
            &format!("{source_id}:p001:0000"),
            "alpha beta gamma",
            1,
        );
        // Quote is NOT in the chunk → verify_citation falls into the
        // VERIFY-001 branch where Layer-3 would fire.
        let mut cit = citation_for(&chunk, "not in chunk");
        cit.source_id = source_id.to_string();
        let doc = card_doc_with(reading_id, vec![cit]);
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        (doc, index)
    }

    #[test]
    fn semantic_suppressed_when_auth_load_error_fires() {
        let (doc, index) = build_layer2_failing_card("R1", "src");
        let auth = AuthSpec {
            matrix: None,
            load_error: Some("source_matrix.json is invalid".to_string()),
        };
        let evaluator = CountingEvaluator::new(SemanticVerdictKind::Fail);
        let out = verify_card(
            &doc,
            &index,
            false,
            None,
            Some(&auth),
            None,
            Some(&evaluator),
        );
        assert!(out.failed);
        let codes_seen: Vec<&str> = out.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert!(
            codes_seen.contains(&codes::AUTH_000),
            "AUTH-000 must appear; got {codes_seen:?}",
        );
        assert!(
            codes_seen.contains(&codes::VERIFY_001),
            "CACG-VERIFY-001 must still appear (mechanical Layer-2 diagnostic \
             preservation); got {codes_seen:?}",
        );
        assert!(
            !codes_seen.contains(&codes::VERIFY_002),
            "CACG-VERIFY-002 must NOT fire when AUTH-000 short-circuited the card; got {codes_seen:?}",
        );
        assert_eq!(
            evaluator.count(),
            0,
            "evaluator must not be invoked when AUTH-000 short-circuited the card",
        );
    }

    #[test]
    fn semantic_suppressed_when_auth_001_unknown_reading_id_fires() {
        let (doc, index) = build_layer2_failing_card("unknown-reading", "src");
        let mut allowed: BTreeMap<String, Vec<String>> = BTreeMap::new();
        allowed.insert("R1".to_string(), vec!["src".to_string()]);
        let auth = AuthSpec {
            matrix: Some(SourceMatrix {
                schema_version: SchemaVersion::V0,
                allowed,
            }),
            load_error: None,
        };
        let evaluator = CountingEvaluator::new(SemanticVerdictKind::Fail);
        let out = verify_card(
            &doc,
            &index,
            false,
            None,
            Some(&auth),
            None,
            Some(&evaluator),
        );
        let codes_seen: Vec<&str> = out.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert!(
            codes_seen.contains(&codes::AUTH_001),
            "AUTH-001 must appear; got {codes_seen:?}",
        );
        assert!(
            codes_seen.contains(&codes::VERIFY_001),
            "CACG-VERIFY-001 must still appear (mechanical Layer-2 diagnostic \
             preservation); got {codes_seen:?}",
        );
        assert!(
            !codes_seen.contains(&codes::VERIFY_002),
            "CACG-VERIFY-002 must NOT fire when AUTH-001 short-circuited the citation; got {codes_seen:?}",
        );
        assert_eq!(
            evaluator.count(),
            0,
            "evaluator must not be invoked when AUTH-001 short-circuited the citation",
        );
    }

    #[test]
    fn semantic_suppressed_when_auth_002_unauthorized_source_fires() {
        let (doc, index) = build_layer2_failing_card("R1", "src-disallowed");
        let mut allowed: BTreeMap<String, Vec<String>> = BTreeMap::new();
        allowed.insert("R1".to_string(), vec!["src-allowed".to_string()]);
        let auth = AuthSpec {
            matrix: Some(SourceMatrix {
                schema_version: SchemaVersion::V0,
                allowed,
            }),
            load_error: None,
        };
        let evaluator = CountingEvaluator::new(SemanticVerdictKind::Fail);
        let out = verify_card(
            &doc,
            &index,
            false,
            None,
            Some(&auth),
            None,
            Some(&evaluator),
        );
        let codes_seen: Vec<&str> = out.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert!(
            codes_seen.contains(&codes::AUTH_002),
            "AUTH-002 must appear; got {codes_seen:?}",
        );
        assert!(
            codes_seen.contains(&codes::VERIFY_001),
            "CACG-VERIFY-001 must still appear (mechanical Layer-2 diagnostic \
             preservation); got {codes_seen:?}",
        );
        assert!(
            !codes_seen.contains(&codes::VERIFY_002),
            "CACG-VERIFY-002 must NOT fire when AUTH-002 short-circuited the citation; got {codes_seen:?}",
        );
        assert_eq!(
            evaluator.count(),
            0,
            "evaluator must not be invoked when AUTH-002 short-circuited the citation",
        );
    }

    #[test]
    fn semantic_fires_only_for_auth_passing_citation_in_mixed_card() {
        // Card with two citations: A authorized + Layer-2-failing,
        // B unauthorized + Layer-2-failing. The evaluator must be
        // invoked exactly ONCE (for A) and exactly ONE CACG-VERIFY-002
        // must appear.
        let chunk_a = one_page_chunk("src-a", "src-a:p001:0000", "alpha beta gamma", 1);
        let chunk_b = one_page_chunk("src-b", "src-b:p001:0000", "delta epsilon zeta", 1);
        let mut cit_a = citation_for(&chunk_a, "not in chunk");
        cit_a.source_id = "src-a".to_string();
        let mut cit_b = citation_for(&chunk_b, "also not in chunk");
        cit_b.source_id = "src-b".to_string();
        let doc = card_doc_with("R1", vec![cit_a, cit_b]);
        let index =
            ChunksIndex::from_manifest(manifest_with(vec![chunk_a, chunk_b], vec![], vec![]))
                .unwrap();
        let mut allowed: BTreeMap<String, Vec<String>> = BTreeMap::new();
        allowed.insert("R1".to_string(), vec!["src-a".to_string()]);
        let auth = AuthSpec {
            matrix: Some(SourceMatrix {
                schema_version: SchemaVersion::V0,
                allowed,
            }),
            load_error: None,
        };
        let evaluator = CountingEvaluator::new(SemanticVerdictKind::Fail);
        let out = verify_card(
            &doc,
            &index,
            false,
            None,
            Some(&auth),
            None,
            Some(&evaluator),
        );
        let codes_seen: Vec<&str> = out.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert!(
            codes_seen.contains(&codes::AUTH_002),
            "AUTH-002 must appear for the unauthorized citation; got {codes_seen:?}",
        );
        // Both citations failed Layer-2; CACG-VERIFY-001 must still
        // appear for at least the AUTH-passing one (mechanical
        // Layer-2 diagnostic preservation).
        let verify_001_count = out
            .diagnostics
            .iter()
            .filter(|d| d.code == codes::VERIFY_001)
            .count();
        assert!(
            verify_001_count >= 1,
            "at least one CACG-VERIFY-001 must remain (mechanical Layer-2 \
             diagnostic preservation); got {verify_001_count}, codes={codes_seen:?}",
        );
        let verify_002_count = out
            .diagnostics
            .iter()
            .filter(|d| d.code == codes::VERIFY_002)
            .count();
        assert_eq!(
            verify_002_count, 1,
            "exactly one CACG-VERIFY-002 must appear (for the AUTH-passing citation only); got {verify_002_count}, codes={codes_seen:?}",
        );
        assert_eq!(
            evaluator.count(),
            1,
            "evaluator must be invoked exactly once (for the AUTH-passing Layer-2-failing citation)",
        );
    }

    #[test]
    fn semantic_suppressed_when_hash_001_short_circuits_citation() {
        // The plan enumerates HASH-* as a negative-firing surface.
        // verify_citation already short-circuits internally on
        // HASH-001 before the VERIFY-001 branch, so this test pins
        // the pre-existing natural suppression and guards against
        // regression.
        let chunk = one_page_chunk("src", "src:p001:0000", "alpha beta gamma", 1);
        let mut cit = citation_for(&chunk, "beta");
        cit.chunk_hash = "0".repeat(64); // valid hex but wrong
        let doc = card_doc_with("R1", vec![cit]);
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let evaluator = CountingEvaluator::new(SemanticVerdictKind::Fail);
        let out = verify_card(&doc, &index, false, None, None, None, Some(&evaluator));
        let codes_seen: Vec<&str> = out.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert!(
            codes_seen.contains(&codes::HASH_001),
            "HASH-001 must appear; got {codes_seen:?}",
        );
        assert!(
            !codes_seen.contains(&codes::VERIFY_002),
            "CACG-VERIFY-002 must NOT fire after a HASH-001 short-circuit; got {codes_seen:?}",
        );
        assert_eq!(
            evaluator.count(),
            0,
            "evaluator must not be invoked on HASH-001"
        );
    }

    #[test]
    fn semantic_not_invoked_when_fuzzy_path_accepts() {
        // Plan: "Layer-3 does NOT fire when fuzzy match accepts."
        // The citation's quote differs from the chunk text by one
        // character; with fuzzy enabled, Layer-2 verifies and
        // verify_citation returns BEFORE the VERIFY-001 branch.
        let chunk = one_page_chunk(
            "src",
            "src:p001:0000",
            "the quick brown fox jumps over the lazy dog and runs fast",
            1,
        );
        let mut cit = citation_for(&chunk, "the quick brown fox jumps over the lazy dox");
        cit.source_id = "src".to_string();
        let doc = card_doc_with("R1", vec![cit]);
        let index = ChunksIndex::from_manifest(manifest_with(vec![chunk], vec![], vec![])).unwrap();
        let evaluator = CountingEvaluator::new(SemanticVerdictKind::Fail);
        // fuzzy_enabled = true
        let out = verify_card(&doc, &index, true, None, None, None, Some(&evaluator));
        assert!(!out.failed, "card must verify clean via fuzzy");
        assert!(out.used_fuzzy);
        let codes_seen: Vec<&str> = out.diagnostics.iter().map(|d| d.code.as_str()).collect();
        assert!(
            !codes_seen.contains(&codes::VERIFY_002),
            "CACG-VERIFY-002 must NOT fire when fuzzy accepted; got {codes_seen:?}",
        );
        assert_eq!(
            evaluator.count(),
            0,
            "evaluator must not be invoked when fuzzy accepts the citation",
        );
    }
}
