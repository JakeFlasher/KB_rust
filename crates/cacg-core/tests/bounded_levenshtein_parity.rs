#![allow(
    clippy::unwrap_used,
    clippy::doc_markdown,
    clippy::doc_lazy_continuation,
    clippy::needless_borrows_for_generic_args,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::unnecessary_cast,
    clippy::manual_range_contains,
    clippy::iter_cloned_collect,
    clippy::explicit_deref_methods,
    clippy::explicit_auto_deref,
    clippy::too_many_lines
)]
//! Cross-implementation parity gate for
//! `cacg_core::verify::fuzzy::bounded_levenshtein`: every fixture in
//! `tests/parity_corpus/bounded_levenshtein/oracle.json` must agree
//! with the Python reference distance from
//! `cacg.verify.fuzzy.levenshtein`.
//!
//! For every `(a, b, python_distance, thresholds)` row, and every
//! threshold `t` in `thresholds`, the Rust port must satisfy:
//!   - `bounded_levenshtein(a, b, t).is_some() == (python_distance <= t)`
//!   - When the result is `Some(d)`, `d == python_distance`.
//!
//! The fixture is built by `legacy_python_oracle/scripts/build_bounded_levenshtein_oracle.py`
//! (deterministic seed; re-running the script produces a byte-identical
//! file).

use std::path::PathBuf;

use cacg_core::verify::fuzzy::bounded_levenshtein;
use serde_json::Value;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

#[test]
fn rust_matches_python_truth_value_and_distance_on_every_oracle_case() {
    let path = workspace_root().join("tests/parity_corpus/bounded_levenshtein/oracle.json");
    let raw =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let payload: Value = serde_json::from_str(&raw).expect("oracle JSON must parse");
    assert_eq!(payload["schema_version"], "cacg.v0");
    let entries = payload["entries"]
        .as_array()
        .expect("oracle.json must carry an `entries` array");
    assert!(
        entries.len() >= 60,
        "oracle must carry >=60 cases for adequate bounded-Levenshtein parity coverage; got {}",
        entries.len()
    );

    let mut failures: Vec<String> = Vec::new();
    for entry in entries {
        let idx = entry["index"].as_u64().unwrap_or_default();
        let a = entry["a"].as_str().expect("each entry has an `a` string");
        let b = entry["b"].as_str().expect("each entry has a `b` string");
        let py_dist = entry["python_distance"]
            .as_u64()
            .expect("each entry has a `python_distance` integer");
        let thresholds = entry["bounded_max_edits_list"]
            .as_array()
            .expect("each entry has a `bounded_max_edits_list` array");
        for t in thresholds {
            let t = u32::try_from(
                t.as_u64()
                    .expect("each threshold must be a non-negative integer"),
            )
            .expect("thresholds fit in u32");
            let rs = bounded_levenshtein(a, b, t);
            let expected_some = (py_dist as u64) <= u64::from(t);
            if rs.is_some() != expected_some {
                failures.push(format!(
                    "case {idx} threshold={t}: rust_is_some={} expected_is_some={} \
                     python_distance={py_dist}; a={a:?} b={b:?}",
                    rs.is_some(),
                    expected_some,
                ));
                continue;
            }
            if let Some(d) = rs {
                if u64::from(d) != py_dist {
                    failures.push(format!(
                        "case {idx} threshold={t}: rust_distance={d} python_distance={py_dist}; \
                         a={a:?} b={b:?}",
                    ));
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "bounded_levenshtein diverges from Python on {} case(s):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}

#[test]
fn oracle_fixture_has_expected_shape() {
    let path = workspace_root().join("tests/parity_corpus/bounded_levenshtein/oracle.json");
    let raw = std::fs::read_to_string(&path).unwrap();
    let payload: Value = serde_json::from_str(&raw).unwrap();
    let entries = payload["entries"].as_array().unwrap();
    assert_eq!(
        usize::try_from(payload["case_count"].as_u64().unwrap()).unwrap(),
        entries.len(),
        "case_count must equal entries.len()"
    );
    for entry in entries {
        assert!(entry["a"].is_string(), "every entry must have a string `a`");
        assert!(entry["b"].is_string(), "every entry must have a string `b`");
        assert!(
            entry["python_distance"].is_number(),
            "every entry must have a numeric `python_distance`"
        );
        let thresholds = entry["bounded_max_edits_list"]
            .as_array()
            .expect("every entry must have a `bounded_max_edits_list` array");
        assert!(!thresholds.is_empty(), "thresholds list must be non-empty");
        for t in thresholds {
            assert!(t.is_u64(), "every threshold must be a non-negative integer");
        }
    }
}
