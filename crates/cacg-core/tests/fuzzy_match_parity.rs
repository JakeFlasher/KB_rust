#![allow(clippy::unwrap_used)]
//! Cross-implementation parity gate for
//! `cacg_core::verify::fuzzy::fuzzy_match`: every fixture in
//! `tests/parity_corpus/fuzzy_match/oracle.json` must produce the
//! same boolean result as Python `cacg.verify.fuzzy.fuzzy_match`
//! under the same `(max_dist, min_ratio)` thresholds.
//!
//! The fixture is built by `legacy_python_oracle/scripts/build_fuzzy_match_oracle.py`
//! (deterministic seed; re-running the script produces a byte-
//! identical file). Each entry carries `(quote, chunk_text,
//! max_dist, min_ratio, python_result)`.

use std::path::PathBuf;

use cacg_core::verify::fuzzy::{bounded_levenshtein, fuzzy_match, ratcliff_obershelp_ratio};
use serde_json::Value;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

#[test]
fn rust_matches_python_on_every_oracle_case() {
    let path = workspace_root().join("tests/parity_corpus/fuzzy_match/oracle.json");
    let raw =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let payload: Value = serde_json::from_str(&raw).expect("oracle JSON must parse");
    assert_eq!(payload["schema_version"], "cacg.v0");
    let entries = payload["entries"]
        .as_array()
        .expect("oracle.json must carry an `entries` array");
    assert!(
        entries.len() >= 40,
        "oracle must carry >=40 cases for adequate fuzzy_match parity coverage; got {}",
        entries.len()
    );

    let mut failures: Vec<String> = Vec::new();
    for entry in entries {
        let idx = entry["index"].as_u64().unwrap_or_default();
        let quote = entry["quote"]
            .as_str()
            .expect("each entry has a `quote` string");
        let chunk = entry["chunk_text"]
            .as_str()
            .expect("each entry has a `chunk_text` string");
        let max_dist = u32::try_from(
            entry["max_dist"]
                .as_u64()
                .expect("each entry has an integer `max_dist`"),
        )
        .expect("max_dist fits in u32");
        let min_ratio = entry["min_ratio"]
            .as_f64()
            .expect("each entry has a numeric `min_ratio`");
        let py = entry["python_result"]
            .as_bool()
            .expect("each entry has a boolean `python_result`");
        let rs = fuzzy_match(quote, chunk, max_dist, min_ratio);
        if rs != py {
            failures.push(format!(
                "case {idx}: rust={rs} python={py}; quote={quote:?} chunk={chunk:?} \
                 max_dist={max_dist} min_ratio={min_ratio}"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "fuzzy_match diverges from Python on {} case(s):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}

#[test]
fn oracle_covers_both_decisive_hard_negative_categories() {
    // For every fixture entry with `python_result == false`, scan
    // the same candidate windows `fuzzy_match` would
    // (`window` in `[max(1, q - max_dist), min(t, q + max_dist)]`,
    // `start` in `[0, t - window]`, by code-point offsets). Classify
    // each candidate as:
    //
    //   - ratio_pass_lev_fail: `ratio >= min_ratio` AND `lev > max_dist`
    //   - ratio_fail_lev_pass: `ratio < min_ratio` AND `lev <= max_dist`
    //
    // Assert both categories appear at least once across the
    // fixture. This pins the contract that the oracle cannot lose
    // either decisive hard-negative category in a future fixture
    // edit; without this guard, every false-result case could
    // silently collapse into the ratio-fail bucket.
    let path = workspace_root().join("tests/parity_corpus/fuzzy_match/oracle.json");
    let raw = std::fs::read_to_string(&path).unwrap();
    let payload: Value = serde_json::from_str(&raw).unwrap();
    let entries = payload["entries"].as_array().unwrap();

    let mut has_ratio_pass_lev_fail = false;
    let mut has_ratio_fail_lev_pass = false;

    for entry in entries {
        let python_result = entry["python_result"].as_bool().unwrap();
        if python_result {
            continue;
        }
        let quote = entry["quote"].as_str().unwrap();
        let chunk = entry["chunk_text"].as_str().unwrap();
        let max_dist = u32::try_from(entry["max_dist"].as_u64().unwrap()).unwrap();
        let min_ratio = entry["min_ratio"].as_f64().unwrap();

        if quote.is_empty() {
            // Empty quote returns true; skipped above already, but
            // be defensive.
            continue;
        }
        let chunk_chars: Vec<char> = chunk.chars().collect();
        let q = quote.chars().count();
        let t = chunk_chars.len();
        let max_dist_usize = max_dist as usize;
        let min_window = q.saturating_sub(max_dist_usize).max(1);
        let max_window = t.min(q + max_dist_usize);
        if min_window > t {
            continue;
        }
        for window in min_window..=max_window {
            for start in 0..=(t - window) {
                let candidate: String = chunk_chars[start..start + window].iter().collect();
                let r = ratcliff_obershelp_ratio(quote, &candidate);
                let l = bounded_levenshtein(quote, &candidate, max_dist);
                if r >= min_ratio && l.is_none() {
                    has_ratio_pass_lev_fail = true;
                }
                if r < min_ratio && l.is_some() {
                    has_ratio_fail_lev_pass = true;
                }
            }
        }
    }

    assert!(
        has_ratio_pass_lev_fail,
        "fixture lost the ratio-pass/Levenshtein-fail decisive category; \
         at least one false-result entry must contain a candidate window where \
         ratio >= min_ratio AND bounded_levenshtein > max_dist"
    );
    assert!(
        has_ratio_fail_lev_pass,
        "fixture lost the ratio-fail/Levenshtein-pass decisive category; \
         at least one false-result entry must contain a candidate window where \
         ratio < min_ratio AND bounded_levenshtein <= max_dist"
    );
}

#[test]
fn oracle_fixture_has_expected_shape() {
    let path = workspace_root().join("tests/parity_corpus/fuzzy_match/oracle.json");
    let raw = std::fs::read_to_string(&path).unwrap();
    let payload: Value = serde_json::from_str(&raw).unwrap();
    let entries = payload["entries"].as_array().unwrap();
    assert_eq!(
        usize::try_from(payload["case_count"].as_u64().unwrap()).unwrap(),
        entries.len(),
        "case_count must equal entries.len()"
    );
    for entry in entries {
        assert!(
            entry["quote"].is_string(),
            "each entry has a string `quote`"
        );
        assert!(
            entry["chunk_text"].is_string(),
            "each entry has a string `chunk_text`"
        );
        assert!(
            entry["max_dist"].is_number(),
            "each entry has a numeric `max_dist`"
        );
        assert!(
            entry["min_ratio"].is_number(),
            "each entry has a numeric `min_ratio`"
        );
        assert!(
            entry["python_result"].is_boolean(),
            "each entry has a boolean `python_result`"
        );
    }
}
