#![allow(clippy::unwrap_used)]
//! Cross-implementation parity gate for
//! `cacg_core::verify::fuzzy::ratcliff_obershelp_ratio`: every fixture
//! in `tests/parity_corpus/ratcliff_obershelp/oracle.json` must match
//! Python `difflib.SequenceMatcher(None, a, b).ratio()` within `1e-9`.
//!
//! The fixture is built by `legacy_python_oracle/scripts/build_ratcliff_obershelp_oracle.py`
//! (deterministic seed; re-running the script produces a byte-identical
//! file). Each entry carries `(a, b, python_ratio)`; the test asserts
//! Rust's computed ratio is within 1e-9 of `python_ratio`.

use std::path::PathBuf;

use cacg_core::verify::fuzzy::ratcliff_obershelp_ratio;
use serde_json::Value;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

#[test]
fn rust_matches_python_within_1e_minus_9_on_every_oracle_case() {
    let path = workspace_root().join("tests/parity_corpus/ratcliff_obershelp/oracle.json");
    let raw =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let payload: Value = serde_json::from_str(&raw).expect("oracle JSON must parse");
    assert_eq!(payload["schema_version"], "cacg.v0");
    assert_eq!(payload["autojunk"], true);
    let entries = payload["entries"]
        .as_array()
        .expect("oracle.json must carry an `entries` array");
    assert!(
        entries.len() >= 50,
        "oracle must carry >=50 cases for adequate fuzzy parity coverage; got {}",
        entries.len()
    );

    let mut failures: Vec<String> = Vec::new();
    for entry in entries {
        let idx = entry["index"].as_u64().unwrap_or_default();
        let a = entry["a"].as_str().expect("each entry has an `a` string");
        let b = entry["b"].as_str().expect("each entry has a `b` string");
        let py = entry["python_ratio"]
            .as_f64()
            .expect("each entry has a `python_ratio` float");
        let rs = ratcliff_obershelp_ratio(a, b);
        let delta = (rs - py).abs();
        if delta >= 1e-9 {
            failures.push(format!(
                "case {idx}: rust={rs:.18} python={py:.18} delta={delta:.18}; a={a:?} b={b:?}"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "ratcliff_obershelp_ratio diverges from Python on {} case(s):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}

#[test]
fn oracle_fixture_has_expected_shape() {
    // Structural sanity for the committed fixture (case_count matches
    // entries length, schema_version is the expected literal, every
    // entry has the required fields). The Python script's `json.dumps`
    // formatting is NOT held to Rust's `canonical_json` byte equality;
    // the trust contract is numerical parity within 1e-9, asserted by
    // the test above.
    let path = workspace_root().join("tests/parity_corpus/ratcliff_obershelp/oracle.json");
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
            entry["python_ratio"].is_number(),
            "every entry must have a numeric `python_ratio`"
        );
    }
}
