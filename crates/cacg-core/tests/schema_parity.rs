#![allow(clippy::unwrap_used)]
//! AC-C4 byte-equal parity gate: `cacg_core::frontmatter::parse_card`
//! matches Python `cacg.frontmatter.parse_card` pass/fail across the
//! full M0 corpus.
//!
//! Coverage:
//! - 5 valid cards in `tests/parity_corpus/valid/*.md` -- must parse Ok.
//! - 54 `unicode_edge` cards in `tests/parity_corpus/unicode_edge/cards/*.md` -- must parse Ok.
//! - 12 adversarial cards in `tests/parity_corpus/adversarial/*.md` --
//!   the first 2 (01-malformed-hash, 02-reversed-page-range) are
//!   parse-layer failures that must Err with the documented code;
//!   the remaining 10 are lint-layer failures (`chunks_manifest` /
//!   `card_hash` / auth / retraction) that parse cleanly here.
//! - 72 parse-layer fixtures in
//!   `tests/parity_corpus/generated_pydantic_errors/manifest.json`
//!   (where `oracle_layer == "parse"`) -- each fails with the exact
//!   `expected_code`.
//!
//! Plan reference: AC-C4 at `plans/cacg-rust-port-trust-kernel-first-plan.md:60`.

use std::path::PathBuf;

use cacg_core::frontmatter::parse_card;
use serde_json::Value;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

#[test]
fn valid_cards_parse_cleanly() {
    let dir = workspace_root().join("tests/parity_corpus/valid");
    assert!(dir.is_dir(), "missing: {}", dir.display());
    let mut count = 0usize;
    let mut failures: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let text = std::fs::read_to_string(&path).unwrap();
        match parse_card(&text) {
            Ok(_) => count += 1,
            Err(diags) => failures.push(format!(
                "{}: parse_card returned Err({:?})",
                path.display(),
                diags[0]
            )),
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} valid cards failed to parse:\n  {}",
        failures.len(),
        count + failures.len(),
        failures.join("\n  ")
    );
    assert_eq!(count, 5, "expected exactly 5 valid cards; got {count}");
}

#[test]
fn unicode_edge_cards_parse_cleanly() {
    let dir = workspace_root().join("tests/parity_corpus/unicode_edge/cards");
    assert!(dir.is_dir(), "missing: {}", dir.display());
    let mut count = 0usize;
    let mut failures: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let text = std::fs::read_to_string(&path).unwrap();
        match parse_card(&text) {
            Ok(_) => count += 1,
            Err(diags) => failures.push(format!(
                "{}: parse_card returned Err({:?})",
                path.display(),
                diags[0]
            )),
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} unicode_edge cards failed to parse:\n  {}",
        failures.len(),
        count + failures.len(),
        failures.join("\n  ")
    );
    assert_eq!(
        count, 54,
        "expected exactly 54 unicode_edge cards; got {count}"
    );
}

#[test]
fn adversarial_parse_layer_codes() {
    // Only the first 2 adversarial cards fail at parse-time; the rest
    // are lint-layer failures (chunks_manifest / card_hash / auth /
    // retraction) that pass `parse_card` cleanly.
    let dir = workspace_root().join("tests/parity_corpus/adversarial");
    let expected: [(&str, &str); 2] = [
        ("01-malformed-hash.md", "CACG-CITE-002"),
        ("02-reversed-page-range.md", "CACG-CITE-003"),
    ];
    for (name, code) in expected {
        let path = dir.join(name);
        let text = std::fs::read_to_string(&path).unwrap();
        let result = parse_card(&text);
        match result {
            Err(diags) => {
                assert_eq!(
                    diags[0].code, code,
                    "{name}: expected first diagnostic to be {code:?}, got {:?}",
                    diags[0]
                );
            }
            Ok(_) => {
                panic!("{name}: expected Err but parse_card succeeded");
            }
        }
    }

    // The remaining 10 adversarial cards parse cleanly (they fail at
    // lint-time, not parse-time).
    for n in 3..=12 {
        let name = format!("{n:02}");
        let candidates: Vec<_> = std::fs::read_dir(&dir)
            .unwrap()
            .filter_map(std::result::Result::ok)
            .filter(|e| {
                e.path()
                    .file_name()
                    .and_then(|s| s.to_str())
                    .is_some_and(|s| s.starts_with(&name))
            })
            .collect();
        assert_eq!(
            candidates.len(),
            1,
            "expected exactly one adversarial fixture starting with {name:?}; got {}",
            candidates.len()
        );
        let path = candidates[0].path();
        let text = std::fs::read_to_string(&path).unwrap();
        let result = parse_card(&text);
        assert!(
            result.is_ok(),
            "{}: expected parse_card Ok (lint-layer failure), got {:?}",
            path.display(),
            result
        );
    }
}

#[test]
fn generated_pydantic_parse_layer_fixtures() {
    let manifest_path =
        workspace_root().join("tests/parity_corpus/generated_pydantic_errors/manifest.json");
    assert!(
        manifest_path.is_file(),
        "missing: {}",
        manifest_path.display()
    );
    let raw = std::fs::read_to_string(&manifest_path).unwrap();
    let manifest: Value = serde_json::from_str(&raw).unwrap();
    let fixtures = manifest["fixtures"].as_array().expect("fixtures array");

    // Codex R15-REVIEW-1: zero-skip gate. Every parse-layer fixture
    // must match Python's expected_code byte-for-byte. Round 16's
    // strict_shape_check prescan in cacg_core::frontmatter closes the
    // 15 fixtures previously deferred to "Pydantic-strict-vs-serde-
    // permissive-coercion" class. The list below is retained as a
    // structural assertion: each of the 15 fixture names MUST appear
    // in the manifest with the expected primary code, so a future
    // regression that drops one fails-loud.
    let r16_regression_targets: &[(&str, &str)] = &[
        ("id-wrong-type-int", "CACG-FM-008"),
        ("id-wrong-type-bool", "CACG-FM-008"),
        ("id-wrong-type-null", "CACG-FM-008"),
        ("title-wrong-type-int", "CACG-FM-008"),
        ("title-wrong-type-bool", "CACG-FM-008"),
        ("title-wrong-type-null", "CACG-FM-008"),
        ("reading_id-wrong-type-int", "CACG-FM-008"),
        ("reading_id-wrong-type-bool", "CACG-FM-008"),
        ("reading_id-wrong-type-null", "CACG-FM-008"),
        ("summary-wrong-type-int", "CACG-FM-008"),
        ("summary-wrong-type-bool", "CACG-FM-008"),
        ("summary-wrong-type-null", "CACG-FM-008"),
        ("tags-non-string", "CACG-SUM-003"),
        ("tags-not-list", "CACG-SUM-003"),
        ("citation-page-range-single-element", "CACG-FM-001"),
    ];

    let mut tested = 0usize;
    let mut regression_seen: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    let mut failures: Vec<String> = Vec::new();
    for fixture in fixtures {
        let oracle_layer = fixture["oracle_layer"].as_str().unwrap_or("");
        if oracle_layer != "parse" {
            continue;
        }
        let name = fixture["name"].as_str().unwrap();
        let card_text = fixture["card_text"].as_str().unwrap();
        let expected_code = fixture["expected_code"].as_str().unwrap();
        let result = parse_card(card_text);
        match result {
            Err(diags) => {
                if diags[0].code != expected_code {
                    failures.push(format!(
                        "fixture {name:?}: expected {expected_code}, got {} ({})",
                        diags[0].code, diags[0].message
                    ));
                }
            }
            Ok(_) => {
                failures.push(format!(
                    "fixture {name:?}: expected Err({expected_code}) but parse_card succeeded"
                ));
            }
        }
        // Track which Round-16 regression targets we observed.
        for (target_name, _) in r16_regression_targets {
            if *target_name == name {
                regression_seen.insert(*target_name);
            }
        }
        tested += 1;
    }

    // Zero-skip gate: exactly 72 parse-layer fixtures tested.
    assert_eq!(
        tested, 72,
        "expected EXACTLY 72 parse-layer fixtures (Codex R15-REVIEW-1 zero-skip); got {tested}"
    );
    // Round-16 regression coverage: all 15 names must be in the manifest.
    for (target_name, _) in r16_regression_targets {
        assert!(
            regression_seen.contains(target_name),
            "R16 regression target {target_name:?} missing from manifest -- a future regenerate must keep this fixture"
        );
    }
    assert!(
        failures.is_empty(),
        "{} of {tested} parse-layer fixtures failed:\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
}
