#![allow(clippy::unwrap_used)]
//! Parity gate for `cacg_core::card_loader::load_card` against the
//! committed corpus.
//!
//! Coverage:
//!
//! * `tests/parity_corpus/valid/*.md` — every card must load cleanly
//!   and the returned [`CardDoc`] must carry a non-empty
//!   `body_normalized` (the verify hot path reads from this field).
//! * `tests/parity_corpus/adversarial/01-malformed-hash.md` and
//!   `02-reversed-page-range.md` — parse-layer failures that must
//!   produce `CardLoadError` with the documented CACG-CITE-* code.
//! * `tests/parity_corpus/adversarial/03..12` — lint-layer adversarials
//!   that parse cleanly here (the corresponding lint failures land in
//!   the AC-2 work, not this AC-1 loader test).

use std::path::PathBuf;

use cacg_core::card_loader::load_card;
use cacg_core::diagnostic::codes;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

#[test]
fn valid_cards_load_with_normalized_body() {
    let dir = workspace_root().join("tests/parity_corpus/valid");
    assert!(dir.is_dir(), "missing fixture dir: {}", dir.display());
    let mut count = 0;
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let doc =
            load_card(&path).unwrap_or_else(|e| panic!("{} should load: {e}", path.display()));
        assert_eq!(doc.path, path);
        assert!(
            !doc.body_normalized.is_empty(),
            "{}: body_normalized must be non-empty",
            path.display()
        );
        assert!(!doc.frontmatter.citations.is_empty());
        count += 1;
    }
    assert!(count >= 2, "expected at least 2 valid cards, got {count}");
}

#[test]
fn adversarial_parse_layer_cards_surface_documented_codes() {
    let dir = workspace_root().join("tests/parity_corpus/adversarial");
    let parse_failures = [
        ("01-malformed-hash.md", codes::CITE_002),
        ("02-reversed-page-range.md", codes::CITE_003),
    ];
    for (name, expected) in parse_failures {
        let path = dir.join(name);
        let err = load_card(&path).expect_err(&format!("{name} should fail parse-layer load"));
        assert!(
            err.diagnostics.iter().any(|d| d.code == expected),
            "{name}: expected diagnostic with code {expected}, got {:?}",
            err.diagnostics
                .iter()
                .map(|d| d.code.as_str())
                .collect::<Vec<_>>()
        );
        assert!(
            err.diagnostics[0].file.is_some(),
            "{name}: diagnostic must carry the file path"
        );
    }
}

#[test]
fn lint_layer_adversarials_parse_cleanly() {
    // These adversarials fail at the lint layer (AC-2 work), not the
    // loader. The loader must let them through.
    let dir = workspace_root().join("tests/parity_corpus/adversarial");
    let lint_layer = [
        "03-chunk-not-in-manifest.md",
        "04-stale-card-hash.md",
        "05-chunk-hash-drift.md",
        "06-page-disjoint.md",
        "07-fake-quote.md",
        "08-auth-unknown-reading.md",
        "09-auth-unauthorized-source.md",
        "10-retracted-card.md",
        "11-retracted-source-cited.md",
        "12-retracted-chunk-cited.md",
    ];
    for name in lint_layer {
        let path = dir.join(name);
        let doc = load_card(&path)
            .unwrap_or_else(|e| panic!("{} should parse cleanly: {e}", path.display()));
        assert_eq!(doc.path, path);
    }
}
