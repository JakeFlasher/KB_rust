#![allow(clippy::unwrap_used)]
//! AC-C3 byte-equal parity gate: for every Unicode-edge sample in
//! `tests/parity_corpus/unicode_edge/manifest.json`, assert
//! `cacg_core::normalize::normalize_text(input)` byte-equals the
//! Python-computed `normalized` field at
//! `tests/parity_corpus/out_python/unicode_edge/<name>/normalize.json`.
//!
//! Plan reference: AC-C3 at
//! `plans/cacg-rust-port-trust-kernel-first-plan.md:59`. Spec:
//! `legacy_python_oracle/src/cacg/normalize.py` (the 5-step pipeline).

use std::path::PathBuf;

use cacg_core::normalize::normalize_text;
use serde_json::Value;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

#[test]
fn unicode_edge_samples_match_python() {
    let manifest_path = workspace_root().join("tests/parity_corpus/unicode_edge/manifest.json");
    assert!(
        manifest_path.is_file(),
        "unicode_edge manifest missing: {}",
        manifest_path.display()
    );
    let raw = std::fs::read_to_string(&manifest_path).unwrap();
    let manifest: Value = serde_json::from_str(&raw).unwrap();
    let samples = manifest["samples"]
        .as_array()
        .expect("samples must be an array");
    assert!(
        samples.len() >= 50,
        "spec §AC-D2 requires at least 50 Unicode-edge cards; got {}",
        samples.len()
    );

    let mut failures: Vec<String> = Vec::new();
    let mut checked = 0usize;
    for sample in samples {
        let name = sample["name"].as_str().expect("each sample has a name");
        let input = sample["input"].as_str().expect("each sample has an input");
        // Cross-check against the per-card normalize.json oracle so the
        // gate ALSO proves the corpus-side normalize oracle agrees with
        // the manifest's `expected` field.
        let oracle_path = workspace_root().join(format!(
            "tests/parity_corpus/out_python/unicode_edge/{name}/normalize.json"
        ));
        if !oracle_path.is_file() {
            failures.push(format!(
                "sample {name:?}: missing per-card normalize oracle at {}",
                oracle_path.display()
            ));
            continue;
        }
        let oracle_raw = std::fs::read_to_string(&oracle_path).unwrap();
        let oracle: Value = serde_json::from_str(&oracle_raw).unwrap();
        let expected = oracle["normalized"]
            .as_str()
            .expect("normalize.json must have a `normalized` field");
        let actual = normalize_text(input);
        if actual != expected {
            failures.push(format!(
                "sample {name:?}: expected {expected:?}, got {actual:?}; input={input:?}"
            ));
        }
        checked += 1;
    }
    assert!(
        failures.is_empty(),
        "{} of {checked} unicode_edge samples diverged from Python normalize_text:\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}
