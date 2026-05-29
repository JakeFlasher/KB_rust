//! `iai-callgrind` instruction-count benchmark for `build_index`.
//!
//! Uses a 2-card golden corpus from the parity test suite.
//! Run with `cargo bench -p cacg-core --bench index_iai` (needs `valgrind`).

use std::hint::black_box;
use std::path::PathBuf;

use cacg_core::determinism::DeterminismContext;
use cacg_core::index::build_index;
use iai_callgrind::{library_benchmark, library_benchmark_group, main};

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Invariant: cacg-core is under crates/")
        .parent()
        .expect("Invariant: crates/ is under workspace root")
        .join("tests/parity_corpus")
}

#[library_benchmark]
fn build_index_golden_corpus() {
    let corpus = corpus_dir();
    let cards_dir = corpus.join("cards/reading_01");
    let out_dir = std::env::temp_dir().join("cacg_bench_index");
    let _ = std::fs::remove_dir_all(&out_dir);
    std::fs::create_dir_all(&out_dir).expect("Invariant: can create temp dir");

    std::env::set_var("KB_FROZEN_CLOCK", "1");
    let ctx = DeterminismContext::from_env();

    let _ = black_box(build_index(
        black_box(&cards_dir),
        black_box(&out_dir),
        black_box(&ctx),
    ));

    let _ = std::fs::remove_dir_all(&out_dir);
}

library_benchmark_group!(name = index; benchmarks = build_index_golden_corpus);

main!(library_benchmark_groups = index);
