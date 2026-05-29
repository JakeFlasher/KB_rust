#![allow(clippy::unwrap_used)]
//! Cross-implementation parity gate for the in-memory search backend.
//!
//! `cacg_search::SummariesIndex` must reproduce Python
//! `cacg.search.SummariesIndex` — both the `from_path` load/validation
//! surface and the `search` score-then-filter-then-cap path — on every
//! fixture case in `tests/parity_corpus/summaries_search/oracle.json`.
//!
//! The fixture is built by `legacy_python_oracle/scripts/build_summaries_search_oracle.py`
//! (deterministic; re-running produces a byte-identical file).

use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;

use cacg_core::schema::{SchemaVersion, SourceMatrix};
use cacg_search::{SummariesIndex, SummariesLoadError};
use serde_json::Value;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn load_oracle() -> Value {
    let path = workspace_root().join("tests/parity_corpus/summaries_search/oracle.json");
    let raw =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let oracle: Value = serde_json::from_str(&raw).expect("oracle.json must parse");
    assert_eq!(oracle["schema_version"], "cacg.v0");
    oracle
}

fn write_temp(content: &str) -> tempfile::NamedTempFile {
    let mut f = tempfile::NamedTempFile::new().expect("tempfile");
    f.write_all(content.as_bytes())
        .expect("write summaries.json");
    f.flush().expect("flush");
    f
}

/// Coarse outcome category, matching the oracle's `outcome` field.
fn outcome_of(result: &Result<SummariesIndex, SummariesLoadError>) -> &'static str {
    match result {
        Ok(_) => "ok",
        Err(SummariesLoadError::LegacyMissingSourceIds) => "sum_007",
        Err(_) => "rejected",
    }
}

#[test]
fn from_path_load_validation_matches_python() {
    let oracle = load_oracle();
    let cases = oracle["load_cases"]
        .as_array()
        .expect("`load_cases` must be an array");
    assert!(
        cases.len() >= 8,
        "expected >=8 load cases (ok / sum_007 / rejected coverage); got {}",
        cases.len()
    );

    let mut failures: Vec<String> = Vec::new();
    for case in cases {
        let name = case["name"].as_str().unwrap_or("<unnamed>");
        let summaries_json = case["summaries_json"].as_str().expect("summaries_json");
        let want = case["outcome"].as_str().expect("outcome");
        let f = write_temp(summaries_json);
        let got = outcome_of(&SummariesIndex::from_path(f.path()));
        if got != want {
            failures.push(format!("load case {name:?}: rust={got} python={want}"));
        }
    }
    assert!(
        failures.is_empty(),
        "{} load-validation parity failure(s):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}

fn source_matrix_from(value: &Value) -> Option<SourceMatrix> {
    let obj = value.as_object()?;
    let mut allowed: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (reading, srcs) in obj {
        let list = srcs
            .as_array()
            .expect("matrix value is an array")
            .iter()
            .map(|s| s.as_str().expect("source_id is a string").to_string())
            .collect();
        allowed.insert(reading.clone(), list);
    }
    Some(SourceMatrix {
        schema_version: SchemaVersion::V0,
        allowed,
    })
}

#[test]
fn search_matches_python() {
    let oracle = load_oracle();
    let corpus = oracle["search_corpus"].as_str().expect("search_corpus");
    let corpus_file = write_temp(corpus);
    let index =
        SummariesIndex::from_path(corpus_file.path()).expect("search_corpus must load cleanly");
    let cases = oracle["search_cases"]
        .as_array()
        .expect("`search_cases` must be an array");
    assert!(
        cases.len() >= 8,
        "expected >=8 search cases; got {}",
        cases.len()
    );

    let mut failures: Vec<String> = Vec::new();
    for case in cases {
        let name = case["name"].as_str().unwrap_or("<unnamed>");
        let query = case["query"].as_str().expect("query");
        let top_k = case["top_k"].as_i64().expect("top_k is an integer");
        let matrix = source_matrix_from(&case["source_matrix"]);
        let retracted: Vec<String> = case["retracted_card_ids"]
            .as_array()
            .expect("retracted_card_ids array")
            .iter()
            .map(|v| v.as_str().expect("retracted id is a string").to_string())
            .collect();

        let hits = index.search(query, top_k, matrix.as_ref(), &retracted);
        let want = case["hits"].as_array().expect("`hits` array");
        if hits.len() != want.len() {
            failures.push(format!(
                "search case {name:?}: hit count rust={} python={}; rust_ids={:?}",
                hits.len(),
                want.len(),
                hits.iter().map(|h| &h.card_id).collect::<Vec<_>>()
            ));
            continue;
        }
        for (i, (got, exp)) in hits.iter().zip(want).enumerate() {
            for (field, got_val) in [
                ("card_id", got.card_id.as_str()),
                ("reading_id", got.reading_id.as_str()),
                ("title", got.title.as_str()),
                ("summary", got.summary.as_str()),
                ("path", got.path.as_str()),
                ("card_hash", got.card_hash.as_str()),
            ] {
                let exp_val = exp
                    .get(field)
                    .and_then(Value::as_str)
                    .unwrap_or("<missing>");
                if got_val != exp_val {
                    failures.push(format!(
                        "search case {name:?} hit[{i}].{field}: rust={got_val:?} python={exp_val:?}"
                    ));
                }
            }
            let exp_tags: Vec<&str> = exp["tags"]
                .as_array()
                .expect("tags array")
                .iter()
                .map(|t| t.as_str().expect("tag is a string"))
                .collect();
            if got
                .tags
                .iter()
                .map(String::as_str)
                .ne(exp_tags.iter().copied())
            {
                failures.push(format!(
                    "search case {name:?} hit[{i}].tags: rust={:?} python={exp_tags:?}",
                    got.tags
                ));
            }
            let exp_score = exp["score"].as_f64().expect("score is a number");
            if format!("{:.6}", got.score) != format!("{exp_score:.6}") {
                failures.push(format!(
                    "search case {name:?} hit[{i}].score: rust={:.6} python={exp_score:.6}",
                    got.score
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} search parity failure(s):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}
