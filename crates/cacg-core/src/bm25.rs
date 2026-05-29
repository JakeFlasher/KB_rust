//! In-house Okapi BM25 scorer, byte-equal with Python `rank_bm25.BM25Okapi`.
//!
//! The CACG retrieval layer (verify-failure hints and `kb search` ranking)
//! must reproduce the Python reference exactly. `rank_bm25` is not portable
//! to the trust kernel (it pulls `numpy`), and the third-party Rust `bm25`
//! crate and `tantivy` diverge in tokenizer, IDF, and scoring. So the scorer
//! is implemented here, in `cacg-core`, where both `cacg-core::verify` and
//! `cacg-search` can consume it without a dependency cycle.
//!
//! Reference: `.venv/.../rank_bm25.py` (`BM25._initialize`,
//! `BM25Okapi._calc_idf`, `BM25Okapi.get_scores`); audit at
//! `_research/15_bm25_retrieval_audit.md`. Okapi parameters are fixed at the
//! `rank_bm25` defaults (`k1 = 1.5`, `b = 0.75`, `epsilon = 0.25`); they are
//! the parity contract and are intentionally not configurable.
//!
//! Determinism: scores are a pure function of the tokenized corpus and query.
//! `get_scores` returns only finite values -- a non-degenerate corpus is
//! provably finite, and the degenerate all-empty corpus is floored to zeros
//! rather than producing `NaN`.

use std::collections::{HashMap, HashSet};

/// Term-frequency saturation parameter (`rank_bm25` `BM25Okapi` `k1` default).
const K1: f64 = 1.5;
/// Document-length normalization parameter (`rank_bm25` `b` default).
const B: f64 = 0.75;
/// Negative-IDF floor multiplier (`rank_bm25` `epsilon` default).
const EPSILON: f64 = 0.25;

/// An Okapi BM25 index built from a tokenized corpus.
///
/// Construct with [`Bm25Okapi::new`] over a slice of token lists (one per
/// document), then call [`Bm25Okapi::get_scores`] with a token list query.
/// Tokens are produced by [`crate::normalize::tokenize_for_lookup`].
#[derive(Debug, Clone)]
pub struct Bm25Okapi {
    corpus_size: usize,
    avgdl: f64,
    doc_len: Vec<f64>,
    doc_freqs: Vec<HashMap<String, u32>>,
    idf: HashMap<String, f64>,
}

impl Bm25Okapi {
    /// Build an Okapi BM25 index over a tokenized `corpus`.
    ///
    /// Mirrors `rank_bm25` `BM25._initialize` + `BM25Okapi._calc_idf`. Unlike
    /// `rank_bm25` -- which raises `ZeroDivisionError` on an empty corpus or a
    /// corpus with no tokens at all -- this constructor is total: an empty
    /// corpus yields an index whose `get_scores` returns no scores, and a
    /// degenerate all-empty corpus is handled without panicking. A caller that
    /// must mirror the Python failure checks [`Bm25Okapi::is_token_empty`] on
    /// the built index.
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // corpus/document sizes are far below 2^53.
    pub fn new(corpus: &[Vec<String>]) -> Self {
        let corpus_size = corpus.len();
        let mut doc_len: Vec<f64> = Vec::with_capacity(corpus_size);
        let mut doc_freqs: Vec<HashMap<String, u32>> = Vec::with_capacity(corpus_size);
        let mut num_doc: u64 = 0;

        // `nd`: number of documents containing each word. Kept in first-
        // occurrence order so the `idf_sum` accumulation below matches Python
        // dict-iteration order exactly -- float addition is not associative,
        // so the summation order is load-bearing for byte-equal parity.
        let mut nd_index: HashMap<String, usize> = HashMap::new();
        let mut nd: Vec<(String, u32)> = Vec::new();

        for doc in corpus {
            doc_len.push(doc.len() as f64);
            num_doc += doc.len() as u64;

            let mut frequencies: HashMap<String, u32> = HashMap::with_capacity(doc.len());
            for tok in doc {
                *frequencies.entry(tok.clone()).or_insert(0) += 1;
            }

            // Increment `nd` once per distinct word, walking the document in
            // first-occurrence order (Python iterates `frequencies.items()`,
            // an insertion-ordered dict).
            let mut seen: HashSet<&str> = HashSet::with_capacity(doc.len());
            for tok in doc {
                if seen.insert(tok.as_str()) {
                    if let Some(&pos) = nd_index.get(tok.as_str()) {
                        nd[pos].1 += 1;
                    } else {
                        // qg-allow: deep-nesting — BM25 scorer no-touch zone
                        nd_index.insert(tok.clone(), nd.len());
                        nd.push((tok.clone(), 1));
                    }
                }
            }
            doc_freqs.push(frequencies);
        }

        let avgdl = if corpus_size == 0 {
            0.0
        } else {
            num_doc as f64 / corpus_size as f64
        };

        // _calc_idf (Okapi variant): idf = ln(N - df + 0.5) - ln(df + 0.5);
        // then every negative idf is floored at `epsilon * average_idf`.
        let n = corpus_size as f64;
        let mut idf: HashMap<String, f64> = HashMap::with_capacity(nd.len());
        let mut idf_sum = 0.0_f64;
        let mut negative: Vec<String> = Vec::new();
        for (word, df) in &nd {
            let df = f64::from(*df);
            let value = (n - df + 0.5).ln() - (df + 0.5).ln();
            idf.insert(word.clone(), value);
            idf_sum += value;
            if value < 0.0 {
                negative.push(word.clone());
            }
        }
        // `average_idf` is consumed only to floor negative idfs. When there
        // are none it is never read -- and skipping it also sidesteps the
        // `idf_sum / 0` that an all-empty corpus would otherwise hit.
        if !negative.is_empty() {
            let average_idf = idf_sum / idf.len() as f64;
            let eps = EPSILON * average_idf;
            for word in negative {
                idf.insert(word, eps);
            }
        }

        Self {
            corpus_size,
            avgdl,
            doc_len,
            doc_freqs,
            idf,
        }
    }

    /// Score every document against `query`, byte-equal with
    /// `rank_bm25` `BM25Okapi.get_scores`.
    ///
    /// Returns one raw (un-rounded) score per document, in corpus order. An
    /// empty corpus yields an empty vector; an empty query yields all zeros.
    /// Query tokens absent from the corpus contribute zero; repeated query
    /// tokens are scored once per occurrence (the score accumulates). Every
    /// returned score is finite.
    #[must_use]
    pub fn get_scores(&self, query: &[String]) -> Vec<f64> {
        let mut score = vec![0.0_f64; self.corpus_size];
        for q in query {
            let q_idf = self.idf.get(q).copied().unwrap_or(0.0);
            for ((slot, freqs), &dl) in score.iter_mut().zip(&self.doc_freqs).zip(&self.doc_len) {
                let qf = f64::from(freqs.get(q).copied().unwrap_or(0));
                // `b * dl / avgdl` is evaluated left-to-right to match
                // numpy's `b * doc_len / avgdl`; `avgdl == 0` only on a
                // degenerate all-empty corpus, where it is floored to 0.
                let len_norm = if self.avgdl == 0.0 {
                    0.0
                } else {
                    B * dl / self.avgdl
                };
                let denom = qf + K1 * (1.0 - B + len_norm);
                *slot += q_idf * (qf * (K1 + 1.0) / denom);
            }
        }
        debug_assert!(
            score.iter().all(|s| s.is_finite()),
            "BM25 scores must be finite"
        );
        score
    }

    /// Number of documents in the indexed corpus.
    #[must_use]
    pub fn corpus_size(&self) -> usize {
        self.corpus_size
    }

    /// Average document length (`avgdl`); `0.0` for an empty corpus.
    #[must_use]
    pub fn avgdl(&self) -> f64 {
        self.avgdl
    }

    /// Per-document token counts, in corpus order.
    #[must_use]
    pub fn doc_len(&self) -> &[f64] {
        &self.doc_len
    }

    /// Final IDF weight for `term` after the negative-IDF floor, or `None` if
    /// `term` does not occur in the corpus.
    #[must_use]
    pub fn idf(&self, term: &str) -> Option<f64> {
        self.idf.get(term).copied()
    }

    /// `true` when the index holds no tokens at all -- either an empty
    /// corpus or a corpus whose every document is empty.
    ///
    /// This is exactly the shape on which `rank_bm25.BM25Okapi` raises
    /// `ZeroDivisionError`: `_calc_idf` divides `idf_sum` by an empty IDF
    /// map (and `_initialize` divides by `corpus_size` for the empty
    /// corpus). [`Bm25Okapi::new`] is deliberately total and builds such
    /// an index anyway, so a caller that must reproduce the Python
    /// failure -- `SummariesIndex` rejecting an all-blank `summaries.json`,
    /// the verify-hint path skipping an all-blank chunk corpus -- checks
    /// this predicate.
    #[must_use]
    pub fn is_token_empty(&self) -> bool {
        self.idf.is_empty()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::Bm25Okapi;

    fn corpus(docs: &[&[&str]]) -> Vec<Vec<String>> {
        docs.iter()
            .map(|d| d.iter().map(|t| (*t).to_string()).collect())
            .collect()
    }

    fn query(tokens: &[&str]) -> Vec<String> {
        tokens.iter().map(|t| (*t).to_string()).collect()
    }

    #[test]
    fn empty_corpus_yields_no_scores() {
        let bm25 = Bm25Okapi::new(&[]);
        assert!(bm25.get_scores(&query(&["anything"])).is_empty());
        assert_eq!(bm25.corpus_size(), 0);
        assert!((bm25.avgdl() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn empty_query_yields_zero_scores() {
        let bm25 = Bm25Okapi::new(&corpus(&[&["alpha", "beta"], &["gamma"]]));
        let scores = bm25.get_scores(&[]);
        assert_eq!(scores, vec![0.0, 0.0]);
    }

    #[test]
    fn degenerate_all_empty_corpus_is_finite() {
        // Python `BM25Okapi([[]])` raises; the Rust port floors to finite zeros.
        let bm25 = Bm25Okapi::new(&corpus(&[&[], &[]]));
        let scores = bm25.get_scores(&query(&["x"]));
        assert_eq!(scores.len(), 2);
        assert!(scores.iter().all(|s| s.is_finite()));
    }

    #[test]
    fn is_token_empty_flags_the_rank_bm25_raise_condition() {
        // `rank_bm25.BM25Okapi` raises `ZeroDivisionError` for an empty
        // corpus and for a corpus whose every document is empty; the
        // total Rust port builds the index anyway and flags both shapes
        // via `is_token_empty`.
        assert!(Bm25Okapi::new(&[]).is_token_empty());
        assert!(Bm25Okapi::new(&corpus(&[&[], &[]])).is_token_empty());
        // A corpus with at least one token anywhere is not token-empty.
        assert!(!Bm25Okapi::new(&corpus(&[&["alpha"], &[]])).is_token_empty());
    }

    #[test]
    fn unknown_query_token_contributes_zero() {
        let bm25 = Bm25Okapi::new(&corpus(&[&["alpha"], &["beta"]]));
        assert_eq!(bm25.get_scores(&query(&["zzz"])), vec![0.0, 0.0]);
    }

    #[test]
    fn single_document_corpus_scores_finite() {
        // N == 2*df for the lone term, so idf is negative and floored; the
        // score is finite (and, here, negative) rather than panicking.
        let bm25 = Bm25Okapi::new(&corpus(&[&["foo"]]));
        let scores = bm25.get_scores(&query(&["foo"]));
        assert_eq!(scores.len(), 1);
        assert!(scores[0].is_finite());
    }

    #[test]
    #[allow(clippy::float_cmp)] // `x + x == 2.0 * x` is exact in IEEE-754.
    fn repeated_query_token_accumulates_exactly() {
        let bm25 = Bm25Okapi::new(&corpus(&[&["alpha", "beta"], &["alpha", "gamma"]]));
        let once = bm25.get_scores(&query(&["alpha"]));
        let twice = bm25.get_scores(&query(&["alpha", "alpha"]));
        for (o, t) in once.iter().zip(&twice) {
            // The query token is scored once per occurrence: `twice` adds the
            // same per-token term to the same accumulator twice, so it equals
            // `2 * once` bit-for-bit (doubling and `x + x` are both exact).
            assert_eq!(2.0 * o, *t);
        }
    }

    #[test]
    fn idf_and_avgdl_are_exposed() {
        let bm25 = Bm25Okapi::new(&corpus(&[&["a", "b"], &["a"]]));
        assert!((bm25.avgdl() - 1.5).abs() < f64::EPSILON);
        assert!(bm25.idf("a").is_some());
        assert!(bm25.idf("missing").is_none());
    }
}
