//! `iai-callgrind` instruction-count benchmark for `lint_card`.
//!
//! Uses the golden parity corpus card as the benchmark target.
//! Run with `cargo bench -p cacg-core --bench lint_iai` (needs `valgrind`).

use std::hint::black_box;
use std::path::PathBuf;

use cacg_core::lint::layer1::lint_card;
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
fn lint_one_golden_card() {
    let corpus = corpus_dir();
    let card = corpus.join("cards/reading_01/01-content-addressable-identity.md");
    let chunks = corpus.join("out_python/chunks_manifest.json");
    let journal = corpus.join("cards/reading_01/01-content-addressable-identity.history.jsonl");

    let _ = black_box(lint_card(
        black_box(&card),
        black_box(&chunks),
        black_box(&journal),
        black_box(None),
        black_box(None),
    ));
}

library_benchmark_group!(name = lint; benchmarks = lint_one_golden_card);

main!(library_benchmark_groups = lint);
