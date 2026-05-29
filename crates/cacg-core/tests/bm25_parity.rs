#![allow(clippy::unwrap_used)]
//! Cross-implementation parity gate for the BM25 retrieval core.
//!
//! `cacg_core::bm25::Bm25Okapi` must reproduce `rank_bm25.BM25Okapi`, and
//! `cacg_core::normalize::{normalize_for_lookup, tokenize_for_lookup}` must
//! reproduce `cacg.normalize.normalize_for_lookup` + its `.split()`
//! tokenizer, on every fixture case in `tests/parity_corpus/bm25/oracle.json`.
//!
//! The fixture is built by `legacy_python_oracle/scripts/build_bm25_oracle.py` (deterministic;
//! re-running produces a byte-identical file). Reference contract:
//! `_research/15_bm25_retrieval_audit.md`.

use std::path::PathBuf;

use cacg_core::bm25::Bm25Okapi;
use cacg_core::normalize::{normalize_for_lookup, tokenize_for_lookup};
use serde_json::Value;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn load_oracle() -> Value {
    let path = workspace_root().join("tests/parity_corpus/bm25/oracle.json");
    let raw =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let oracle: Value = serde_json::from_str(&raw).expect("oracle.json must parse");
    assert_eq!(oracle["schema_version"], "cacg.v0");
    oracle
}

fn str_vec(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("expected a JSON array of strings")
        .iter()
        .map(|v| v.as_str().expect("expected a string").to_string())
        .collect()
}

#[test]
fn normalize_for_lookup_matches_python() {
    let oracle = load_oracle();
    let cases = oracle["normalize_for_lookup"]
        .as_array()
        .expect("normalize_for_lookup must be an array");
    assert!(
        cases.len() >= 12,
        "expected >=12 normalize_for_lookup cases (casefold + tokenizer \
         coverage); got {}",
        cases.len()
    );

    let mut failures: Vec<String> = Vec::new();
    for case in cases {
        let name = case["name"].as_str().unwrap_or("<unnamed>");
        let input = case["input"].as_str().expect("each case has an `input`");
        let expected_norm = case["normalized"]
            .as_str()
            .expect("each case has a `normalized`");
        let expected_tokens = str_vec(&case["tokens"]);

        let actual_norm = normalize_for_lookup(input);
        if actual_norm != expected_norm {
            failures.push(format!(
                "case {name:?}: normalize_for_lookup({input:?}) = {actual_norm:?}, \
                 expected {expected_norm:?}"
            ));
        }
        let actual_tokens = tokenize_for_lookup(input);
        if actual_tokens != expected_tokens {
            failures.push(format!(
                "case {name:?}: tokenize_for_lookup({input:?}) = {actual_tokens:?}, \
                 expected {expected_tokens:?}"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} normalize_for_lookup parity failure(s):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn bm25_scorer_matches_python() {
    let oracle = load_oracle();
    let cases = oracle["bm25"].as_array().expect("bm25 must be an array");
    assert!(
        cases.len() >= 12,
        "expected >=12 bm25 cases (negative-IDF, ties, empty, casefold, \
         ligature, repeated-term coverage); got {}",
        cases.len()
    );

    let mut failures: Vec<String> = Vec::new();
    for case in cases {
        let name = case["name"].as_str().unwrap_or("<unnamed>");

        // End-to-end tokenizer parity: re-tokenize the raw document/query
        // texts with the Rust tokenizer and confirm we reproduce the
        // oracle's resolved token lists before scoring.
        let corpus_texts = str_vec(&case["corpus_texts"]);
        let rust_corpus: Vec<Vec<String>> = corpus_texts
            .iter()
            .map(|t| tokenize_for_lookup(t))
            .collect();
        let oracle_corpus: Vec<Vec<String>> = case["corpus"]
            .as_array()
            .expect("`corpus` array")
            .iter()
            .map(str_vec)
            .collect();
        if rust_corpus != oracle_corpus {
            failures.push(format!(
                "case {name:?}: corpus tokenization diverged: {rust_corpus:?} \
                 vs {oracle_corpus:?}"
            ));
            continue;
        }
        let query_text = case["query_text"].as_str().expect("`query_text`");
        let rust_query = tokenize_for_lookup(query_text);
        let oracle_query = str_vec(&case["query"]);
        if rust_query != oracle_query {
            failures.push(format!(
                "case {name:?}: query tokenization diverged: {rust_query:?} \
                 vs {oracle_query:?}"
            ));
            continue;
        }

        let bm25 = Bm25Okapi::new(&rust_corpus);

        // corpus_size + per-document length
        let oracle_doc_len: Vec<u64> = case["doc_len"]
            .as_array()
            .expect("`doc_len` array")
            .iter()
            .map(|v| v.as_u64().expect("doc_len entry is an integer"))
            .collect();
        if bm25.corpus_size() != oracle_doc_len.len() {
            failures.push(format!(
                "case {name:?}: corpus_size {} != {}",
                bm25.corpus_size(),
                oracle_doc_len.len()
            ));
        }
        for (i, (&rust_dl, &want)) in bm25.doc_len().iter().zip(&oracle_doc_len).enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let want_f = want as f64;
            if (rust_dl - want_f).abs() > f64::EPSILON {
                failures.push(format!("case {name:?}: doc_len[{i}] {rust_dl} != {want}"));
            }
        }

        // avgdl + final IDF map (absent for the empty corpus: avgdl is null).
        if let Some(oracle_avgdl) = case["avgdl"].as_f64() {
            if (bm25.avgdl() - oracle_avgdl).abs() > 1e-12 {
                failures.push(format!(
                    "case {name:?}: avgdl {} != {oracle_avgdl}",
                    bm25.avgdl()
                ));
            }
        }
        if let Some(idf_map) = case["idf"].as_object() {
            for (word, want) in idf_map {
                let want = want.as_f64().expect("idf value is a number");
                match bm25.idf(word) {
                    Some(got) if (got - want).abs() <= 1e-9 => {}
                    other => failures.push(format!(
                        "case {name:?}: idf({word:?}) = {other:?}, expected {want}"
                    )),
                }
            }
        }

        // Raw + 6-decimal-rounded scores.
        let scores = bm25.get_scores(&rust_query);
        let oracle_raw: Vec<f64> = case["scores_raw"]
            .as_array()
            .expect("`scores_raw` array")
            .iter()
            .map(|v| v.as_f64().expect("score is a number"))
            .collect();
        let oracle_rounded: Vec<f64> = case["scores_rounded"]
            .as_array()
            .expect("`scores_rounded` array")
            .iter()
            .map(|v| v.as_f64().expect("rounded score is a number"))
            .collect();
        if scores.len() != oracle_raw.len() {
            failures.push(format!(
                "case {name:?}: score count {} != {}",
                scores.len(),
                oracle_raw.len()
            ));
            continue;
        }
        for (i, &got) in scores.iter().enumerate() {
            assert!(got.is_finite(), "case {name:?}: score[{i}] is not finite");
            if (got - oracle_raw[i]).abs() > 1e-9 {
                failures.push(format!(
                    "case {name:?}: raw score[{i}] {got} diverged from {} (delta {:e})",
                    oracle_raw[i],
                    (got - oracle_raw[i]).abs()
                ));
            }
            let got_6 = format!("{got:.6}");
            let want_6 = format!("{:.6}", oracle_rounded[i]);
            if got_6 != want_6 {
                failures.push(format!(
                    "case {name:?}: 6-decimal score[{i}] {got_6} != {want_6}"
                ));
            }
        }

        // Ranked order: sort by score descending, original index ascending
        // (the `bm25_hints` `(-score, index)` tie-break).
        let mut ranked: Vec<usize> = (0..scores.len()).collect();
        ranked.sort_by(|&a, &b| {
            scores[b]
                .partial_cmp(&scores[a])
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.cmp(&b))
        });
        let oracle_ranked: Vec<usize> = case["ranked_order"]
            .as_array()
            .expect("`ranked_order` array")
            .iter()
            .map(|v| usize::try_from(v.as_u64().expect("index is an integer")).unwrap())
            .collect();
        if ranked != oracle_ranked {
            failures.push(format!(
                "case {name:?}: ranked order {ranked:?} != {oracle_ranked:?}"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} bm25 parity failure(s):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}
