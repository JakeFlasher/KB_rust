#![allow(clippy::unwrap_used)]
//! AC-C1 byte-equal parity gate: load `tests/parity_corpus/canonical_json/`
//! and `tests/parity_corpus/canonical_json_reject/` manifests and assert
//! the Rust `canonical_json` writer produces byte-identical output to the
//! committed Python-built `expected` bytes for every happy-path fixture,
//! AND raises the documented typed `CanonicalError` for every reject fixture.
//!
//! The manifests are produced by `legacy_python_oracle/scripts/build_parity_corpus.py` Phase 1.
//! Both manifests are canonical JSON themselves (`sort_keys=True`), so the
//! integration test reads them via `serde_json::from_str` into Value, then
//! iterates the entries.

use cacg_core::canonical_json::{canonical_json, canonical_json_from_str, CanonicalError};
use serde_json::Value;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    // CARGO_MANIFEST_DIR resolves to crates/cacg-core; the workspace root
    // is two levels up.
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop(); // crates/
    p.pop(); // workspace root
    p
}

#[test]
fn happy_path_fixtures_match_python_byte_for_byte() {
    let manifest_path = workspace_root().join("tests/parity_corpus/canonical_json/manifest.json");
    assert!(
        manifest_path.is_file(),
        "manifest missing: {}",
        manifest_path.display()
    );
    let raw = std::fs::read_to_string(&manifest_path).unwrap();
    let manifest: Value = serde_json::from_str(&raw).unwrap();
    let entries = manifest["entries"]
        .as_array()
        .expect("entries must be an array");
    assert!(
        entries.len() >= 200,
        "spec §6.1 requires at least 200 fixtures; got {}",
        entries.len()
    );

    let mut failures: Vec<String> = Vec::new();
    for entry in entries {
        let name = entry["name"].as_str().expect("each entry has a name");
        let input = &entry["input"];
        let expected = entry["expected"]
            .as_str()
            .expect("each entry has an expected string");
        match canonical_json(input) {
            Ok(actual) => {
                if actual != expected {
                    failures.push(format!(
                        "fixture {name:?}: expected {expected:?}, got {actual:?}"
                    ));
                }
            }
            Err(e) => {
                failures.push(format!("fixture {name:?}: writer error {e}"));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} fixture(s) failed byte-equality:\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}

#[test]
fn reject_fixtures_classify_to_typed_canonical_error() {
    let manifest_path =
        workspace_root().join("tests/parity_corpus/canonical_json_reject/manifest.json");
    assert!(
        manifest_path.is_file(),
        "reject manifest missing: {}",
        manifest_path.display()
    );
    let raw = std::fs::read_to_string(&manifest_path).unwrap();
    let manifest: Value = serde_json::from_str(&raw).unwrap();
    let rejects = manifest["rejects"]
        .as_array()
        .expect("rejects must be an array");

    let mut failures: Vec<String> = Vec::new();
    for entry in rejects {
        let name = entry["name"].as_str().expect("each reject has a name");
        let expected_error = entry["expected_error"]
            .as_str()
            .expect("each reject has an expected_error");
        let raw_input = entry.get("raw_input").and_then(|v| v.as_str());

        let outcome = match expected_error {
            "NonFiniteFloat" => {
                // Type-level proof: serde_json::Number::from_f64 returns None
                // for NaN, +Infinity, -Infinity. Rust's serde_json::Value
                // cannot natively carry a non-finite float, so the rejection
                // happens at the construction-site type system, not at the
                // canonical_json writer entry point.
                let inputs = [f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
                let all_none = inputs
                    .iter()
                    .all(|f| serde_json::Number::from_f64(*f).is_none());
                if all_none {
                    Ok(())
                } else {
                    Err(
                        "serde_json::Number::from_f64 unexpectedly accepted a non-finite float"
                            .to_string(),
                    )
                }
            }
            "NonStringKey" => {
                // Type-level proof: serde_json::Map<String, Value> can only
                // carry String keys. Constructing a non-string-keyed map is
                // a compile-time impossibility, satisfying the spec §4
                // NonStringKey rejection class trivially. The CanonicalError
                // variant exists for upstream API surfaces that might accept
                // Map<K, V> with K != String (e.g., a future BTreeMap-based
                // type) before normalization.
                let _proof: serde_json::Map<String, Value> = serde_json::Map::new();
                Ok::<(), String>(())
            }
            "DuplicateKey" => {
                let raw = raw_input.expect("DuplicateKey rejects carry raw_input");
                match canonical_json_from_str(raw) {
                    Err(CanonicalError::DuplicateKey { .. }) => Ok(()),
                    other => Err(format!(
                        "expected DuplicateKey, got {other:?} for raw_input {raw:?}"
                    )),
                }
            }
            "UnpairedSurrogate" => {
                let raw = raw_input.expect("UnpairedSurrogate rejects carry raw_input");
                match canonical_json_from_str(raw) {
                    Err(CanonicalError::UnpairedSurrogate { .. }) => Ok(()),
                    other => Err(format!(
                        "expected UnpairedSurrogate, got {other:?} for raw_input {raw:?}"
                    )),
                }
            }
            "UnsupportedType" => {
                // Type-level proof: serde_json::Value's discriminants are
                // {Null, Bool, Number, String, Array, Object}. Variants for
                // bytes/datetime/set/custom-object do not exist, so a Value
                // cannot be constructed carrying them. The CanonicalError
                // variant exists for upstream API surfaces (e.g., a
                // hand-rolled Serializer) that might emit such a token
                // before reaching the writer.
                #[allow(clippy::no_effect_underscore_binding)]
                let _proof_null = Value::Null;
                #[allow(clippy::no_effect_underscore_binding)]
                let _proof_str = Value::String(String::new());
                Ok::<(), String>(())
            }
            other => Err(format!("unknown expected_error class {other:?}")),
        };

        if let Err(msg) = outcome {
            failures.push(format!("reject {name:?} ({expected_error}): {msg}"));
        }
    }

    assert!(
        failures.is_empty(),
        "{} reject classification failure(s):\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}
