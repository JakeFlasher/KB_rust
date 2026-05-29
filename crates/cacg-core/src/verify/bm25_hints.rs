//! BM25 "did you mean..." hints for failed Layer-2 verifications.
//!
//! Strict policy: hints are diagnostic-only and never grant verification.
//! Every [`Hint`] carries `hint_only = true` so no consumer (CLI, future MCP
//! server) can mistake a hint for authority.
//!
//! Scope: hints are scored over the chunks the caller supplies -- in the
//! verify path, the chunks sharing the failed citation's `source_id`, not the
//! whole corpus. Unlike `kb search`, there is no lexical-overlap match gate:
//! every chunk is scored and may appear in the ranked top-`k`, including
//! zero-score chunks.
//!
//! A [`Bm25HintCache`] rebuilds each source's hint corpus at most once per
//! `(source_id, chunks_signature)` within a single `kb verify` run, so N
//! failed citations against one source do not rebuild the BM25 index N times.
//!
//! Byte-equal with Python `cacg.verify.bm25_hints`; audit at
//! `_research/15_bm25_retrieval_audit.md`.

use std::collections::HashMap;

use serde::Serialize;

use crate::bm25::Bm25Okapi;
use crate::normalize::tokenize_for_lookup;
use crate::schema::ChunkRecord;

/// Number of hints emitted per failed citation by the verify path
/// (Python `bm25_hints.top_k` default `k`).
pub const DEFAULT_HINT_COUNT: usize = 3;

/// One BM25 "did you mean..." hint for a failed Layer-2 citation.
///
/// Mirrors the hint dict returned by Python `bm25_hints.top_k`. Serializes to
/// the same four-key object Python attaches to a `CACG-VERIFY-001`
/// diagnostic's `hints` array.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Hint {
    /// Always `true` -- a hint is advisory and never grants verification.
    pub hint_only: bool,
    /// The suggested chunk's identifier.
    pub chunk_id: String,
    /// BM25 score of the chunk against the citation quote, rounded to 6
    /// decimal places (Python `round(score, 6)`).
    pub score: f64,
    /// Short preview of the suggested chunk's text.
    pub text_preview: String,
}

/// Process-local cache of per-source BM25 hint corpora.
///
/// Mirrors Python `BM25HintCache`: within one `kb verify` run, each source's
/// hint corpus is built at most once per `(source_id, chunks_signature)`.
/// Create one cache per verify run to bound memory; it is not shared across
/// runs.
#[derive(Debug, Default)]
pub struct Bm25HintCache {
    cache: HashMap<(String, String), (Bm25Okapi, Vec<ChunkRecord>)>,
}

impl Bm25HintCache {
    /// Create an empty cache.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of distinct `(source_id, chunks signature)` corpora
    /// currently materialized in the cache. Tests use this to prove
    /// that repeated citations against the same source reuse one
    /// build rather than rebuilding the corpus N times.
    #[must_use]
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// True when no corpus has been built yet.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Return the cached BM25 index + chunk snapshot for
    /// `(source_id, chunks)`, building and caching it on first request.
    ///
    /// The corpus is `tokenize_for_lookup` applied to each chunk's `text`.
    /// The cache owns a cloned snapshot of the chunks so a later retraction
    /// that changes the live chunk slice cannot perturb an already-cached
    /// entry, and so hint payload assembly is stable.
    fn get_or_build(
        &mut self,
        source_id: &str,
        chunks: &[&ChunkRecord],
    ) -> &(Bm25Okapi, Vec<ChunkRecord>) {
        let key = (source_id.to_string(), chunks_signature(chunks));
        self.cache.entry(key).or_insert_with(|| {
            let corpus: Vec<Vec<String>> = chunks
                .iter()
                .map(|c| tokenize_for_lookup(&c.text))
                .collect();
            let snapshot: Vec<ChunkRecord> = chunks.iter().map(|c| (*c).clone()).collect();
            (Bm25Okapi::new(&corpus), snapshot)
        })
    }
}

/// The cache signature for a chunk sequence: every chunk's `chunk_hash`
/// joined by `|`, byte-equal with Python `_chunks_signature`.
///
/// Sensitive to chunk count, order, and content (because `chunk_hash` is
/// content-addressable), so any retract-chunk between verify calls changes
/// the signature and invalidates that source's cache entry.
#[must_use]
pub fn chunks_signature(chunks: &[&ChunkRecord]) -> String {
    chunks
        .iter()
        .map(|c| c.chunk_hash.as_str())
        .collect::<Vec<&str>>()
        .join("|")
}

/// Return up to `k` BM25 hints ranking `chunks` against a failed citation
/// `quote`, byte-equal with Python `bm25_hints.top_k`.
///
/// Returns an empty vector when `chunks` is empty or `quote` tokenizes to
/// nothing. When both `cache` and `source_id` are supplied, the BM25 corpus
/// for the `(source_id, chunks_signature)` pair is built at most once per
/// cache lifetime; otherwise it is built per call. The output is identical
/// either way -- the cache is purely an optimization.
///
/// Hints rank by BM25 score descending, ties broken by lower original chunk
/// index. No lexical-overlap filter is applied: a zero-score chunk can still
/// appear (this matches the Python reference and is intentional -- hints are
/// advisory).
#[must_use]
pub fn top_k(
    quote: &str,
    chunks: &[&ChunkRecord],
    k: usize,
    cache: Option<&mut Bm25HintCache>,
    source_id: Option<&str>,
) -> Vec<Hint> {
    if chunks.is_empty() {
        return Vec::new();
    }
    let query = tokenize_for_lookup(quote);
    if query.is_empty() {
        return Vec::new();
    }
    if let (Some(cache), Some(source_id)) = (cache, source_id) {
        let (bm25, snapshot) = cache.get_or_build(source_id, chunks);
        let snapshot_refs: Vec<&ChunkRecord> = snapshot.iter().collect();
        rank_hints(bm25, &snapshot_refs, &query, k)
    } else {
        let corpus: Vec<Vec<String>> = chunks
            .iter()
            .map(|c| tokenize_for_lookup(&c.text))
            .collect();
        let bm25 = Bm25Okapi::new(&corpus);
        rank_hints(&bm25, chunks, &query, k)
    }
}

/// Score `chunks` with `bm25`, rank, and assemble the top-`k` hint payloads.
fn rank_hints(bm25: &Bm25Okapi, chunks: &[&ChunkRecord], query: &[String], k: usize) -> Vec<Hint> {
    // A corpus that contributes no tokens (every chunk tokenizes to
    // nothing) is the shape `rank_bm25.BM25Okapi` raises on. Hints are
    // advisory and must never crash verify -- the scorer is total by
    // design -- and a hint pointing at an empty chunk is not useful, so
    // a token-empty corpus yields no hints.
    if bm25.is_token_empty() {
        return Vec::new();
    }
    let scores = bm25.get_scores(query);
    // sorted(enumerate(scores), key=lambda x: (-x[1], x[0])) -- score
    // descending, then original chunk index ascending.
    let mut ranked: Vec<usize> = (0..scores.len()).collect();
    ranked.sort_by(|&a, &b| {
        scores[b]
            .partial_cmp(&scores[a])
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(a.cmp(&b))
    });
    ranked
        .into_iter()
        .take(k)
        .map(|idx| Hint {
            hint_only: true,
            chunk_id: chunks[idx].chunk_id.clone(),
            score: round6(scores[idx]),
            text_preview: chunks[idx].text_preview.clone(),
        })
        .collect()
}

/// Round to 6 decimal places, matching Python `round(x, 6)`.
///
/// Both Rust's `{:.6}` formatting and Python's `round` round half to even,
/// so formatting to six decimals and re-parsing reproduces `round(x, 6)`.
fn round6(x: f64) -> f64 {
    format!("{x:.6}")
        .parse()
        .expect("a fixed-precision float format always re-parses")
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{chunks_signature, top_k, Bm25HintCache, DEFAULT_HINT_COUNT};
    use crate::schema::{ChunkRecord, PageSpan, SchemaVersion};

    /// Build a minimal valid `ChunkRecord`; only `chunk_id`, `chunk_hash`,
    /// `text`, and `text_preview` are read by the hint engine.
    fn chunk(idx: usize, chunk_hash: &str, text: &str) -> ChunkRecord {
        ChunkRecord {
            schema_version: SchemaVersion::V0,
            source_id: "src_a".to_string(),
            chunk_id: format!("src_a:p001:{idx:04}"),
            chunk_hash: chunk_hash.to_string(),
            ordinal: u32::try_from(idx).unwrap(),
            start_page: 1,
            end_page: 1,
            page_spans: vec![PageSpan {
                page: 1,
                byte_offset_in_chunk: 0,
            }],
            token_count: 1,
            text: text.to_string(),
            text_preview: text.to_string(),
        }
    }

    fn corpus() -> Vec<ChunkRecord> {
        (0..5)
            .map(|i| {
                let hash = format!("{i:064x}");
                chunk(i, &hash, &format!("chunk number {i} content text"))
            })
            .collect()
    }

    /// Borrow every chunk -- the hint engine takes `&[&ChunkRecord]`, the
    /// shape `ChunksIndex::chunks_by_source` yields.
    fn refs(chunks: &[ChunkRecord]) -> Vec<&ChunkRecord> {
        chunks.iter().collect()
    }

    #[test]
    fn empty_chunks_yield_no_hints() {
        let mut cache = Bm25HintCache::new();
        assert!(top_k("query", &[], DEFAULT_HINT_COUNT, None, None).is_empty());
        assert!(top_k(
            "query",
            &[],
            DEFAULT_HINT_COUNT,
            Some(&mut cache),
            Some("src_a")
        )
        .is_empty());
    }

    #[test]
    fn empty_quote_yields_no_hints() {
        let chunks = corpus();
        assert!(top_k("", &refs(&chunks), DEFAULT_HINT_COUNT, None, None).is_empty());
        assert!(top_k("   ", &refs(&chunks), DEFAULT_HINT_COUNT, None, None).is_empty());
    }

    #[test]
    fn all_empty_text_chunks_yield_no_hints() {
        // Every chunk's text tokenizes to nothing -- the shape
        // `rank_bm25.BM25Okapi` raises on. The total Rust scorer instead
        // yields no hints (advisory hint generation never crashes
        // verify), via both the uncached and cached `top_k` paths.
        let chunks = vec![
            chunk(0, &format!("{:064x}", 0), "   "),
            chunk(1, &format!("{:064x}", 1), ""),
        ];
        assert!(top_k("query text", &refs(&chunks), DEFAULT_HINT_COUNT, None, None).is_empty());
        let mut cache = Bm25HintCache::new();
        assert!(top_k(
            "query text",
            &refs(&chunks),
            DEFAULT_HINT_COUNT,
            Some(&mut cache),
            Some("src_a"),
        )
        .is_empty());
    }

    #[test]
    fn chunks_signature_joins_hashes_with_pipe() {
        let hash0 = format!("{:064x}", 0);
        let hash1 = format!("{:064x}", 1);
        let chunks = vec![chunk(0, &hash0, "a"), chunk(1, &hash1, "b")];
        assert_eq!(chunks_signature(&refs(&chunks)), format!("{hash0}|{hash1}"));
    }

    #[test]
    fn top_k_caps_at_k() {
        let chunks = corpus();
        let hints = top_k("content", &refs(&chunks), 3, None, None);
        assert_eq!(hints.len(), 3);
        assert!(hints.iter().all(|h| h.hint_only));
    }

    #[test]
    fn cache_reuses_for_same_source_and_signature() {
        let chunks = corpus();
        let mut cache = Bm25HintCache::new();
        for _ in 0..5 {
            let _ = top_k(
                "content",
                &refs(&chunks),
                3,
                Some(&mut cache),
                Some("src_a"),
            );
        }
        assert_eq!(
            cache.cache.len(),
            1,
            "one (source_id, signature) entry expected"
        );
    }

    #[test]
    fn cache_separates_distinct_source_ids() {
        let chunks = corpus();
        let mut cache = Bm25HintCache::new();
        let _ = top_k(
            "content",
            &refs(&chunks),
            3,
            Some(&mut cache),
            Some("src_a"),
        );
        let _ = top_k(
            "content",
            &refs(&chunks),
            3,
            Some(&mut cache),
            Some("src_b"),
        );
        assert_eq!(cache.cache.len(), 2);
    }

    #[test]
    fn cache_rebuilds_on_signature_change() {
        let chunks_v1 = corpus();
        let chunks_v2 = &chunks_v1[..chunks_v1.len() - 1]; // one fewer chunk
        let mut cache = Bm25HintCache::new();
        let _ = top_k(
            "content",
            &refs(&chunks_v1),
            3,
            Some(&mut cache),
            Some("src_a"),
        );
        let _ = top_k(
            "content",
            &refs(chunks_v2),
            3,
            Some(&mut cache),
            Some("src_a"),
        );
        assert_eq!(cache.cache.len(), 2, "a changed signature is a new entry");
    }

    #[test]
    fn cache_and_no_cache_produce_identical_hints() {
        let chunks = corpus();
        let mut cache = Bm25HintCache::new();
        let cached = top_k(
            "content text",
            &refs(&chunks),
            3,
            Some(&mut cache),
            Some("src_a"),
        );
        let uncached = top_k("content text", &refs(&chunks), 3, None, None);
        assert_eq!(cached, uncached);
    }

    #[test]
    fn ties_break_on_lower_chunk_index() {
        // Two identical-text chunks score equally; the lower index wins.
        let chunks = vec![
            chunk(0, &format!("{:064x}", 0), "alpha beta"),
            chunk(1, &format!("{:064x}", 1), "alpha beta"),
        ];
        let hints = top_k("alpha", &refs(&chunks), 2, None, None);
        assert_eq!(hints[0].chunk_id, "src_a:p001:0000");
        assert_eq!(hints[1].chunk_id, "src_a:p001:0001");
    }
}
