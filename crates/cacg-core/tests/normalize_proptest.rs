#![allow(clippy::unwrap_used)]
//! Seeded proptest gate: generate 10,000 strings from a fixed seed,
//! normalize each via the production `normalize_text` AND a test-local
//! independent reference normalizer, and assert byte-equality per case
//! plus idempotence.
//!
//! The reference normalizer matches the 5-step pipeline of the original
//! Python `cacg.normalize.normalize_text` but is independently
//! implemented — it must NOT delegate to `cacg_core::normalize::normalize_text`
//! or `cacg_core::normalize::is_python_re_whitespace`.

use cacg_core::normalize::{is_python_re_whitespace, normalize_text};
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::{Config, RngAlgorithm, TestRng, TestRunner};
use unicode_normalization::UnicodeNormalization;

const FIXED_SEED: [u8; 32] = [
    0x0C, 0xAC, 0x60, 0x07, 0x0C, 0xAC, 0x60, 0x07, // 'cac6 007' x4
    0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, // AC-C3 marker
    0x00, 0x0B, 0x00, 0x0B, 0x00, 0x0B, 0x00, 0x0B, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00,
];
const CASES: usize = 10_000;

/// All 29 codepoints Python `re.compile(...)` `\s` matches on a `str`
/// pattern. Round 14 (Codex R13 Mainline Gap 1) makes this explicit so
/// the proptest strategy, predicate cross-check, and per-codepoint
/// unit assertions cover the full set rather than a hand-picked subset.
///
/// Order matches the contract spec for diff-readability.
const PYTHON_RE_WHITESPACE: &[(u32, &str)] = &[
    // U+0009..U+000D: ASCII control whitespace (5)
    (0x0009, "HT"),
    (0x000A, "LF"),
    (0x000B, "VT"),
    (0x000C, "FF"),
    (0x000D, "CR"),
    // U+001C..U+001F: file-separator controls (4) -- the Python-vs-Rust
    // divergence Codex R12 caught.
    (0x001C, "FS"),
    (0x001D, "GS"),
    (0x001E, "RS"),
    (0x001F, "US"),
    // U+0020: ASCII space
    (0x0020, "SP"),
    // U+0085: NEL (Next Line)
    (0x0085, "NEL"),
    // U+00A0: NBSP (Non-breaking space)
    (0x00A0, "NBSP"),
    // U+1680: Ogham space mark
    (0x1680, "OGHAM"),
    // U+2000..U+200A: various typographic spaces (11)
    (0x2000, "EN-QUAD"),
    (0x2001, "EM-QUAD"),
    (0x2002, "EN-SPACE"),
    (0x2003, "EM-SPACE"),
    (0x2004, "THREE-PER-EM"),
    (0x2005, "FOUR-PER-EM"),
    (0x2006, "SIX-PER-EM"),
    (0x2007, "FIGURE-SPACE"),
    (0x2008, "PUNCT-SPACE"),
    (0x2009, "THIN-SPACE"),
    (0x200A, "HAIR-SPACE"),
    // U+2028: Line separator (LS)
    (0x2028, "LS"),
    // U+2029: Paragraph separator (PS)
    (0x2029, "PS"),
    // U+202F: Narrow no-break space (NNBSP)
    (0x202F, "NNBSP"),
    // U+205F: Medium mathematical space
    (0x205F, "MMSP"),
    // U+3000: Ideographic space
    (0x3000, "IDEOGRAPHIC"),
];

/// Independent Python `\s` whitespace predicate — the 29 codepoints
/// that Python's `re` module treats as whitespace on `str` patterns.
/// Must NOT delegate to `cacg_core::normalize::is_python_re_whitespace`.
fn ref_is_python_ws(c: char) -> bool {
    matches!(
        c,
        '\u{0009}'
            | '\u{000A}'
            | '\u{000B}'
            | '\u{000C}'
            | '\u{000D}'
            | '\u{001C}'
            | '\u{001D}'
            | '\u{001E}'
            | '\u{001F}'
            | '\u{0020}'
            | '\u{0085}'
            | '\u{00A0}'
            | '\u{1680}'
            | '\u{2000}'
            ..='\u{200A}' | '\u{2028}' | '\u{2029}' | '\u{202F}' | '\u{205F}' | '\u{3000}'
    )
}

/// Independent 5-step normalizer matching Python `cacg.normalize.normalize_text`.
/// Must NOT call `cacg_core::normalize::normalize_text` or any production normalize function.
///
/// Steps:
/// 1. NFC normalization
/// 2. Replace 7 Latin ligatures (U+FB00..U+FB06) with ASCII equivalents
/// 3. Remove hyphenated line breaks: `-\s*\n\s*` → empty
/// 4. Collapse runs of Python-whitespace to a single ASCII space
/// 5. Strip leading/trailing Python-whitespace
fn reference_normalize(input: &str) -> String {
    // Step 1: NFC normalization (before ligature decomposition — the
    // Python contract applies unicodedata.normalize("NFC", ...) first)
    let mut s: String = input.nfc().collect();

    // Step 2: ligature decomposition
    let ligatures = [
        ('\u{FB00}', "ff"),
        ('\u{FB01}', "fi"),
        ('\u{FB02}', "fl"),
        ('\u{FB03}', "ffi"),
        ('\u{FB04}', "ffl"),
        ('\u{FB05}', "ft"),
        ('\u{FB06}', "st"),
    ];
    for (lig, rep) in &ligatures {
        s = s.replace(*lig, rep);
    }

    // Step 3: remove hyphenated line breaks: `-\s*\n\s*` → ""
    let chars: Vec<char> = s.chars().collect();
    let mut step3 = String::with_capacity(s.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '-' {
            let mut j = i + 1;
            while j < chars.len() && ref_is_python_ws(chars[j]) && chars[j] != '\n' {
                j += 1;
            }
            if j < chars.len() && chars[j] == '\n' {
                j += 1;
                while j < chars.len() && ref_is_python_ws(chars[j]) {
                    j += 1;
                }
                i = j;
                continue;
            }
        }
        step3.push(chars[i]);
        i += 1;
    }

    // Step 4: collapse whitespace runs to single ASCII space
    let mut step4 = String::with_capacity(step3.len());
    let mut in_ws = false;
    for c in step3.chars() {
        if ref_is_python_ws(c) {
            if !in_ws {
                step4.push(' ');
                in_ws = true;
            }
        } else {
            step4.push(c);
            in_ws = false;
        }
    }

    // Step 5: strip leading/trailing whitespace
    let trimmed = step4
        .trim_start_matches(|c: char| ref_is_python_ws(c))
        .trim_end_matches(|c: char| ref_is_python_ws(c));
    trimmed.to_string()
}

/// Strategy for a single string in the CACG normalize domain.
///
/// The generator exercises every class of input the 5-step pipeline
/// touches: the 7 Latin ligatures, combining marks (NFC composition),
/// the full Python `\s` codepoint set (including U+001C..U+001F which
/// Rust's `char::is_whitespace()` excludes), hyphen-linebreak probes,
/// and plain ASCII. Strings are bounded length 0..=20 so the 10k-case
/// run finishes in well under a second.
fn string_strategy() -> impl Strategy<Value = String> {
    // Whitespace branch: pick an index into PYTHON_RE_WHITESPACE.
    // `(0..N).prop_map(...)` is a fast O(1) range-selector strategy
    // (proptest's range-strategy doesn't allocate per case, unlike
    // `prop::sample::select` which clones from a Vec). All 29
    // codepoints are reachable across the 10k-case run; Codex R13
    // Mainline Gap 1's contract requirement is satisfied structurally
    // (via the per-codepoint direct-assertion tests below) AND
    // empirically (the proptest exercises a random subset per run).
    let whitespace_strategy = (0..PYTHON_RE_WHITESPACE.len()).prop_map(|idx| {
        char::from_u32(PYTHON_RE_WHITESPACE[idx].0)
            .expect("PYTHON_RE_WHITESPACE codepoints are all valid")
            .to_string()
    });
    let char_strategy = prop_oneof![
        // ASCII printable + a few punctuation choices.
        "[a-zA-Z0-9.,;:!?_-]",
        // Every Python-`\s` codepoint, table-driven (Round 14).
        whitespace_strategy,
        // The 7 Latin ligatures.
        Just("\u{FB00}".to_string()), // ﬀ
        Just("\u{FB01}".to_string()), // ﬁ
        Just("\u{FB02}".to_string()), // ﬂ
        Just("\u{FB03}".to_string()), // ﬃ
        Just("\u{FB04}".to_string()), // ﬄ
        Just("\u{FB05}".to_string()), // ﬅ
        Just("\u{FB06}".to_string()), // ﬆ
        // Combining marks (NFC composition probes).
        Just("e\u{0301}".to_string()), // e + combining acute -> é
        Just("a\u{0300}".to_string()), // a + combining grave -> à
        Just("u\u{0308}".to_string()), // u + combining diaeresis -> ü
        // Multi-codepoint hyphen-linebreak probes (matches `-\s*\n\s*`).
        Just("-\n".to_string()),
        Just("-  \n  ".to_string()),
        Just("- \n\n".to_string()),
    ];
    prop::collection::vec(char_strategy, 0..=20).prop_map(|parts| parts.concat())
}

#[test]
fn seeded_proptest_matches_reference_oracle_and_is_idempotent() {
    let rng = TestRng::from_seed(RngAlgorithm::ChaCha, &FIXED_SEED);
    let config = Config {
        cases: u32::try_from(CASES).unwrap(),
        ..Config::default()
    };
    let mut runner = TestRunner::new_with_rng(config, rng);

    let strategy = string_strategy();
    let mut inputs: Vec<String> = Vec::with_capacity(CASES);
    let mut rust_outputs: Vec<String> = Vec::with_capacity(CASES);
    let mut ref_outputs: Vec<String> = Vec::with_capacity(CASES);

    for case_idx in 0..CASES {
        let tree = strategy
            .new_tree(&mut runner)
            .expect("strategy new_tree failed");
        let input = tree.current();
        let rust = normalize_text(&input);

        let rust_twice = normalize_text(&rust);
        assert_eq!(
            rust, rust_twice,
            "idempotence violated for case {case_idx}: input={input:?}\n  \
             once  = {rust:?}\n  twice = {rust_twice:?}"
        );

        let reference = reference_normalize(&input);
        inputs.push(input);
        rust_outputs.push(rust);
        ref_outputs.push(reference);
    }

    let mut failures = 0usize;
    for (idx, (r, rf)) in rust_outputs.iter().zip(ref_outputs.iter()).enumerate() {
        if r != rf {
            if failures < 10 {
                eprintln!(
                    "case {idx}: divergence\n  production: {r:?}\n  reference:  {rf:?}\n  input:      {:?}",
                    inputs[idx]
                );
            }
            failures += 1;
        }
    }
    assert_eq!(
        failures, 0,
        "{failures} of {CASES} cases diverged from reference normalize oracle (first 10 logged above)"
    );
}

#[test]
fn table_covers_python_re_whitespace_predicate() {
    // Codex R13 Mainline Gap 1: AC-C3's contract claim is "the proptest
    // covers every Python `\s` codepoint". Round 14 makes the table
    // explicit; this test verifies (a) the table has exactly 29 entries
    // (the documented Python `\s` set), AND (b) `is_python_re_whitespace`
    // returns true for every codepoint in the table. A future regression
    // that drops a codepoint from `PYTHON_RE_WHITESPACE` or changes the
    // predicate will fail this test with explicit codepoint name.

    assert_eq!(
        PYTHON_RE_WHITESPACE.len(),
        29,
        "PYTHON_RE_WHITESPACE must have exactly 29 entries (Python `\\s` set); got {}",
        PYTHON_RE_WHITESPACE.len()
    );

    let mut failures: Vec<String> = Vec::new();
    for &(cp, name) in PYTHON_RE_WHITESPACE {
        let c = char::from_u32(cp).expect("table codepoint is valid");
        if !is_python_re_whitespace(c) {
            failures.push(format!(
                "is_python_re_whitespace(U+{cp:04X} {name}) returned false; \
                 predicate diverges from the explicit Python-`\\s` table"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "predicate / table divergence ({} of {} codepoints):\n  {}",
        failures.len(),
        PYTHON_RE_WHITESPACE.len(),
        failures.join("\n  ")
    );
}

#[test]
fn collapse_works_for_every_python_re_whitespace_codepoint() {
    // For each codepoint in PYTHON_RE_WHITESPACE: a string of the form
    // "a<cp>b" must normalize to "a b" (Step 4 collapses any whitespace
    // run -- including the single codepoint -- to a single ASCII space).
    let mut failures: Vec<String> = Vec::new();
    for &(cp, name) in PYTHON_RE_WHITESPACE {
        let c = char::from_u32(cp).unwrap();
        let input = format!("a{c}b");
        let got = normalize_text(&input);
        if got != "a b" {
            failures.push(format!(
                "collapse U+{cp:04X} ({name}): normalize_text({input:?}) = {got:?}, expected {:?}",
                "a b"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} codepoints failed collapse parity:\n  {}",
        failures.len(),
        PYTHON_RE_WHITESPACE.len(),
        failures.join("\n  ")
    );
}

#[test]
fn strip_works_for_every_python_re_whitespace_codepoint() {
    // For each codepoint: "<cp>hello<cp>" must normalize to "hello"
    // (Step 5 strip; or Step 4 collapse-then-strip, both apply).
    let mut failures: Vec<String> = Vec::new();
    for &(cp, name) in PYTHON_RE_WHITESPACE {
        let c = char::from_u32(cp).unwrap();
        let input = format!("{c}hello{c}");
        let got = normalize_text(&input);
        if got != "hello" {
            failures.push(format!(
                "strip U+{cp:04X} ({name}): normalize_text({input:?}) = {got:?}, expected {:?}",
                "hello"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} codepoints failed strip parity:\n  {}",
        failures.len(),
        PYTHON_RE_WHITESPACE.len(),
        failures.join("\n  ")
    );
}

#[test]
fn hyphen_linebreak_works_for_every_python_re_whitespace_codepoint() {
    // For each codepoint that is NOT a newline (since `-\s*\n\s*` needs
    // the codepoint to appear between dash and newline): a string of
    // the form "dash-<cp>\nthen" must normalize to "dashthen" (Step 3
    // removes the dash + intervening whitespace + newline + trailing
    // whitespace window).
    //
    // U+000A is itself `\n`; the regex pattern `-\s*\n\s*` consumes the
    // first newline as the literal `\n`; "dash-\nthen" becomes "dashthen"
    // (no intervening whitespace before the newline). This is also a
    // valid hyphen-linebreak case so we include U+000A in the assertion.
    let mut failures: Vec<String> = Vec::new();
    for &(cp, name) in PYTHON_RE_WHITESPACE {
        let c = char::from_u32(cp).unwrap();
        // Insert the codepoint between dash and the trailing \n.
        // For cp==U+000A (LF itself), the input becomes "dash-\n\nthen"
        // which Python's greedy `-\s*\n\s*` consumes entirely.
        let input = format!("dash-{c}\nthen");
        let got = normalize_text(&input);
        if got != "dashthen" {
            failures.push(format!(
                "hyphen-linebreak U+{cp:04X} ({name}): normalize_text({input:?}) = {got:?}, expected {:?}",
                "dashthen"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} codepoints failed hyphen-linebreak parity:\n  {}",
        failures.len(),
        PYTHON_RE_WHITESPACE.len(),
        failures.join("\n  ")
    );
}
