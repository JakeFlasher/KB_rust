#![allow(clippy::unwrap_used)]
//! Cross-implementation parity gate for the BM25 verify-hint engine.
//!
//! `cacg_core::verify::bm25_hints::{top_k, chunks_signature}` must reproduce
//! Python `cacg.verify.bm25_hints` on every fixture case in
//! `tests/parity_corpus/bm25_hints/oracle.json`. Each case is checked over
//! both the no-cache and `Bm25HintCache` paths -- the cache is an
//! optimization and must never change the hint output.
//!
//! The fixture is built by `legacy_python_oracle/scripts/build_bm25_hints_oracle.py`
//! (deterministic; re-running produces a byte-identical file).

use std::path::PathBuf;

use cacg_core::schema::{ChunkRecord, PageSpan, SchemaVersion};
use cacg_core::verify::bm25_hints::{chunks_signature, top_k, Bm25HintCache};
use serde_json::Value;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

/// Build a `ChunkRecord` from the four fields the hint engine reads; the
/// remaining fields are filler (the engine never inspects them).
fn chunk_record(
    idx: usize,
    chunk_id: &str,
    chunk_hash: &str,
    text: &str,
    text_preview: &str,
) -> ChunkRecord {
    ChunkRecord {
        schema_version: SchemaVersion::V0,
        source_id: "src".to_string(),
        chunk_id: chunk_id.to_string(),
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
        text_preview: text_preview.to_string(),
    }
}

#[test]
fn bm25_hints_match_python() {
    let path = workspace_root().join("tests/parity_corpus/bm25_hints/oracle.json");
    let raw =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let oracle: Value = serde_json::from_str(&raw).expect("oracle.json must parse");
    assert_eq!(oracle["schema_version"], "cacg.v0");
    let cases = oracle["cases"]
        .as_array()
        .expect("`cases` must be an array");
    assert!(
        cases.len() >= 12,
        "expected >=12 bm25_hints cases (ranking, k-cap, ties, casefold, \
         punctuation, empties); got {}",
        cases.len()
    );

    let mut failures: Vec<String> = Vec::new();
    for case in cases {
        let name = case["name"].as_str().unwrap_or("<unnamed>");
        let quote = case["quote"].as_str().expect("each case has a `quote`");
        let k = usize::try_from(case["k"].as_u64().expect("each case has a `k`")).unwrap();

        let chunks: Vec<ChunkRecord> = case["chunks"]
            .as_array()
            .expect("`chunks` must be an array")
            .iter()
            .enumerate()
            .map(|(i, c)| {
                chunk_record(
                    i,
                    c["chunk_id"].as_str().expect("chunk_id"),
                    c["chunk_hash"].as_str().expect("chunk_hash"),
                    c["text"].as_str().expect("text"),
                    c["text_preview"].as_str().expect("text_preview"),
                )
            })
            .collect();
        // The hint engine takes `&[&ChunkRecord]` -- the shape
        // `ChunksIndex::chunks_by_source` yields.
        let chunk_refs: Vec<&ChunkRecord> = chunks.iter().collect();

        // `chunks_signature` parity.
        let want_sig = case["chunks_signature"].as_str().expect("chunks_signature");
        let got_sig = chunks_signature(&chunk_refs);
        if got_sig != want_sig {
            failures.push(format!(
                "case {name:?}: chunks_signature {got_sig:?} != {want_sig:?}"
            ));
        }

        // The no-cache and cache paths must agree with each other...
        let no_cache = top_k(quote, &chunk_refs, k, None, None);
        let mut cache = Bm25HintCache::new();
        let cached = top_k(quote, &chunk_refs, k, Some(&mut cache), Some("src"));
        if no_cache != cached {
            failures.push(format!(
                "case {name:?}: cache path diverged from the no-cache path"
            ));
        }

        // ...and both must match the Python oracle.
        let want_hints = case["hints"].as_array().expect("`hints` must be an array");
        if no_cache.len() != want_hints.len() {
            failures.push(format!(
                "case {name:?}: hint count {} != {}",
                no_cache.len(),
                want_hints.len()
            ));
            continue;
        }
        for (i, (got, want)) in no_cache.iter().zip(want_hints).enumerate() {
            if !got.hint_only || want["hint_only"].as_bool() != Some(true) {
                failures.push(format!(
                    "case {name:?} hint {i}: hint_only must be true on both sides"
                ));
            }
            let want_id = want["chunk_id"].as_str().expect("chunk_id");
            if got.chunk_id != want_id {
                failures.push(format!(
                    "case {name:?} hint {i}: chunk_id {:?} != {want_id:?}",
                    got.chunk_id
                ));
            }
            let want_preview = want["text_preview"].as_str().expect("text_preview");
            if got.text_preview != want_preview {
                failures.push(format!(
                    "case {name:?} hint {i}: text_preview {:?} != {want_preview:?}",
                    got.text_preview
                ));
            }
            let want_score = want["score"].as_f64().expect("score is a number");
            let got_rounded = format!("{:.6}", got.score);
            let want_rounded = format!("{want_score:.6}");
            if got_rounded != want_rounded {
                failures.push(format!(
                    "case {name:?} hint {i}: 6-decimal score {got_rounded} != {want_rounded}"
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} bm25_hints parity failure(s):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}
