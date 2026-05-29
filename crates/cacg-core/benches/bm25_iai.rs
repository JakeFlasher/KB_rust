//! `iai-callgrind` instruction-count budget for the in-house BM25
//! Okapi scorer (`cacg_core::bm25::Bm25Okapi`).
//!
//! Instruction counts are hardware-stable — unlike wall-clock — so
//! this is the CI-safe perf gate for the retrieval hot path: it
//! detects an instruction-count regression in BM25 index construction
//! plus scoring, the path both `kb search` and the verify-failure
//! hints exercise. It needs `valgrind` on the runner; run with
//! `cargo bench -p cacg-core --bench bm25_iai`. The benchmark requires
//! no network and allocates a fixed, bounded corpus.

use std::hint::black_box;

use cacg_core::bm25::Bm25Okapi;
use iai_callgrind::{library_benchmark, library_benchmark_group, main};

/// A small deterministic corpus: 24 documents of 12 tokens each drawn
/// from a fixed vocabulary, mirroring the `title + summary + tags`
/// token shape `cacg-search` feeds the scorer.
fn sample_corpus() -> Vec<Vec<String>> {
    let vocab = [
        "intrinsic",
        "valuation",
        "discounted",
        "cash",
        "flow",
        "equity",
        "risk",
        "premium",
        "earnings",
        "multiples",
        "relative",
        "capital",
    ];
    (0..24usize)
        .map(|doc| {
            (0..12usize)
                .map(|tok| vocab[(doc + tok) % vocab.len()].to_string())
                .collect()
        })
        .collect()
}

// Measure BM25 index construction plus a representative `get_scores`
// call — the deterministic retrieval hot path. (`#[library_benchmark]`
// rejects `///` doc comments on the benchmarked fn, so this is a plain
// comment.)
#[library_benchmark]
fn bm25_index_and_score() -> Vec<f64> {
    let corpus = black_box(sample_corpus());
    let index = Bm25Okapi::new(black_box(&corpus));
    let query = black_box(vec![
        "valuation".to_string(),
        "equity".to_string(),
        "risk".to_string(),
    ]);
    black_box(index.get_scores(black_box(&query)))
}

library_benchmark_group!(name = bm25; benchmarks = bm25_index_and_score);

main!(library_benchmark_groups = bm25);
