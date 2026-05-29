//! Deterministic text normalization byte-equal to Python `cacg.normalize`.
//!
//! All hash inputs and quote comparisons flow through [`normalize_text`].
//! Mismatch between authoring-time and verify-time normalization is the
//! dominant silent-failure mode in citation pipelines, so this module
//! pins the contract.
//!
//! Spec: `legacy_python_oracle/src/cacg/normalize.py`. Five-step pipeline:
//! 1. Unicode NFC (canonical composition; collapses `e` + combining acute
//!    to the single codepoint `é`).
//! 2. Replace seven Latin ligatures with their multi-character forms.
//! 3. Rejoin hyphenated line breaks: `word-\s*\n\s*next` → `wordnext`.
//! 4. Collapse all runs of Unicode whitespace to a single space.
//! 5. Strip leading and trailing whitespace.
//!
//! Any change to this pipeline is a breaking change to every existing
//! chunk/card hash on the corpus.

use unicode_normalization::UnicodeNormalization;

/// Latin ligature codepoints and their multi-character expansions.
///
/// Mirrors `_LIGATURES` in `legacy_python_oracle/src/cacg/normalize.py:12-20`.
const LIGATURES: &[(char, &str)] = &[
    ('\u{FB00}', "ff"),  // ﬀ
    ('\u{FB01}', "fi"),  // ﬁ
    ('\u{FB02}', "fl"),  // ﬂ
    ('\u{FB03}', "ffi"), // ﬃ
    ('\u{FB04}', "ffl"), // ﬄ
    ('\u{FB05}', "ft"),  // ﬅ
    ('\u{FB06}', "st"),  // ﬆ
];

/// Apply the canonical 5-step normalization pipeline.
///
/// Byte-identical to Python `cacg.normalize.normalize_text` for every
/// input in the CACG corpus -- the 54 `unicode_edge` fixtures at
/// `tests/parity_corpus/unicode_edge/cards/` and the paired
/// `out_python/unicode_edge/<name>/normalize.json` oracles enforce the
/// byte-equal contract.
#[must_use]
pub fn normalize_text(text: &str) -> String {
    // Step 1: Unicode NFC (canonical composition).
    let nfc: String = text.nfc().collect();

    // Step 2: Latin ligature replacement (only if any present, mirroring
    // Python's `if any(ch in normalized for ch in _LIGATURES)` early-out).
    let after_ligatures = if nfc.chars().any(is_ligature_codepoint) {
        let mut s = nfc;
        for (src, dst) in LIGATURES {
            if s.contains(*src) {
                s = s.replace(*src, dst);
            }
        }
        s
    } else {
        nfc
    };

    // Step 3: hyphen-linebreak rejoin -- remove every occurrence of
    // `-\s*\n\s*` (a dash optionally surrounded by whitespace that
    // contains at least one newline). Char-by-char walker matches
    // Python's `re.compile(r"-\s*\n\s*").sub("", text)` greedy
    // semantics: when a dash is followed (after any whitespace that
    // is NOT a newline) by a newline, consume from the dash through
    // all trailing whitespace.
    let after_hyphen = remove_hyphen_linebreak(&after_ligatures);

    // Step 4: collapse runs of whitespace into a single space.
    let collapsed = collapse_whitespace(&after_hyphen);

    // Step 5: strip leading + trailing whitespace. Use the same
    // `is_python_re_whitespace` predicate as steps 3+4 -- Rust's
    // `&str::trim()` uses `char::is_whitespace()` which excludes
    // U+001C..U+001F; while the collapse step typically converts
    // those to ASCII spaces (which `.trim()` would handle), we use
    // the consistent predicate to keep the contract explicit.
    strip_python_re_whitespace(&collapsed).to_string()
}

#[inline]
fn is_ligature_codepoint(c: char) -> bool {
    matches!(
        c,
        '\u{FB00}' | '\u{FB01}' | '\u{FB02}' | '\u{FB03}' | '\u{FB04}' | '\u{FB05}' | '\u{FB06}'
    )
}

/// Whitespace predicate byte-equal to Python 3 `re.compile(...)` `\s`
/// match on a `str` pattern (UNICODE flag, the default for str patterns).
///
/// Differs from Rust's `char::is_whitespace()` by ALSO matching the four
/// file-separator controls U+001C..U+001F (FS, GS, RS, US) -- Python's
/// `re` treats these as `\s` but Rust's Unicode `White_Space` property
/// does not. Codex Round-12 review (R12 Mainline Gap 1) flagged this
/// divergence; without the fix, `normalize_text("ab")` returned
/// `"a\u{1c}b"` in Rust while Python returned `"a b"`, breaking the
/// AC-C3 byte-equal contract on any card body containing one of those
/// separators.
///
/// Round 14 promotes this predicate from private to `pub` so the
/// AC-C3 proptest at `crates/cacg-core/tests/normalize_proptest.rs`
/// can cross-check the predicate against an explicit 29-codepoint
/// table covering every Python `re` `\s` codepoint.
///
/// Sources:
/// - CPython `Lib/sre_parse.py` whitespace class for `re` UNICODE flag.
/// - Unicode 15.1 `White_Space` property (Rust's set).
/// - Codex empirical probe in Round-12 review.
#[inline]
#[must_use]
pub fn is_python_re_whitespace(c: char) -> bool {
    let cp = c as u32;
    // ASCII control whitespace U+0009..U+000D + file-separators
    // U+001C..U+001F + ASCII space.
    if (0x09..=0x0D).contains(&cp) || (0x1C..=0x1F).contains(&cp) || cp == 0x20 {
        return true;
    }
    // Everything else: defer to Rust's `White_Space` property
    // (NEL, NBSP, Ogham space, U+2000..U+200A, U+2028, U+2029, etc).
    c.is_whitespace()
}

fn remove_hyphen_linebreak(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '-' {
            // Scan forward through whitespace that is NOT a newline.
            let mut j = i + 1;
            while j < chars.len() && is_python_re_whitespace(chars[j]) && chars[j] != '\n' {
                j += 1;
            }
            if j < chars.len() && chars[j] == '\n' {
                // Found the `-\s*\n` head; consume trailing whitespace
                // (greedy match of `\s*` after the newline includes any
                // additional newlines, mirroring Python regex backtracking
                // since `\s+` is whitespace-inclusive).
                let mut k = j + 1;
                while k < chars.len() && is_python_re_whitespace(chars[k]) {
                    k += 1;
                }
                i = k;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

fn collapse_whitespace(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_ws = false;
    for c in text.chars() {
        if is_python_re_whitespace(c) {
            if !in_ws {
                out.push(' ');
                in_ws = true;
            }
        } else {
            out.push(c);
            in_ws = false;
        }
    }
    out
}

/// Strip leading + trailing `is_python_re_whitespace` codepoints.
///
/// Rust's built-in `&str::trim()` uses `char::is_whitespace()` which
/// excludes U+001C..U+001F; Python's `str.strip()` (called from
/// `normalize_text` via `.strip()`) uses the same `\s` set as Python
/// `re`. Mirror that here.
fn strip_python_re_whitespace(s: &str) -> &str {
    let start = s.find(|c| !is_python_re_whitespace(c)).unwrap_or(s.len());
    let end = s.rfind(|c| !is_python_re_whitespace(c)).map_or(start, |i| {
        i + s[i..].chars().next().map_or(0, char::len_utf8)
    });
    &s[start..end]
}

// Generated Unicode full case-folding table: `static CASE_FOLD: &[(char, &str)]`,
// sorted by key for binary search. Regenerate with
// `legacy_python_oracle/.venv/bin/python legacy_python_oracle/scripts/build_casefold_table.py`.
include!("casefold_table.rs");

/// Apply Unicode full case folding, byte-equal with Python `str.casefold()`.
///
/// Each scalar is expanded via the generated `CASE_FOLD` table; scalars with
/// an identity fold (absent from the table) pass through unchanged. This is
/// NOT `char::to_lowercase`: full folding maps `ß` -> `ss`, Greek final sigma
/// `ς` -> `σ`, and `İ` (U+0130) -> `i` + U+0307. Unicode full case folding
/// has no context-sensitive rules, so the per-scalar table reproduces
/// `str.casefold()` over a whole string exactly.
fn casefold(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        if let Ok(idx) = CASE_FOLD.binary_search_by(|&(key, _)| key.cmp(&ch)) {
            out.push_str(CASE_FOLD[idx].1);
        } else {
            out.push(ch);
        }
    }
    out
}

/// Case-insensitive normalization for the BM25 retrieval path, byte-equal
/// with Python `cacg.normalize.normalize_for_lookup`.
///
/// `normalize_text(text)` followed by Unicode full case folding. The casefold
/// step is lossy, so this surface is for lookup/ranking ONLY -- never for
/// hashing or verification, which flow through [`normalize_text`]. NFC is not
/// re-applied after folding (Python does not either).
#[must_use]
pub fn normalize_for_lookup(text: &str) -> String {
    casefold(&normalize_text(text))
}

/// Tokenize text for BM25 lookup: byte-equal with Python
/// `normalize_for_lookup(text).split()`.
///
/// After `normalize_text` the string carries only single U+0020 separators
/// with no leading or trailing whitespace, and case folding introduces no
/// whitespace, so splitting on the space and discarding empty fields
/// reproduces the no-argument `str.split()` exactly.
#[must_use]
pub fn tokenize_for_lookup(text: &str) -> Vec<String> {
    normalize_for_lookup(text)
        .split(' ')
        .filter(|tok| !tok.is_empty())
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn identity_on_clean_ascii() {
        assert_eq!(normalize_text("hello world"), "hello world");
    }

    #[test]
    fn nfc_combining_acute() {
        // NFD: e + combining acute (U+0301) -> NFC: é (U+00E9).
        let input = "cafe\u{0301}";
        assert_eq!(normalize_text(input), "café");
    }

    #[test]
    fn ligature_expansion() {
        // ﬁ (U+FB01) -> "fi" (2 chars); o + fi + ce = "ofice".
        assert_eq!(normalize_text("o\u{FB01}ce"), "ofice");
        // ﬃ (U+FB03) -> "ffi" (3 chars); o + ffi + ce = "office".
        assert_eq!(normalize_text("o\u{FB03}ce"), "office");
        // ﬄ (U+FB04) -> "ffl" (3 chars); ba + ffl + e = "baffle".
        assert_eq!(normalize_text("ba\u{FB04}e"), "baffle");
        // ﬆ (U+FB06) -> "st" (2 chars); be + st = "best".
        assert_eq!(normalize_text("be\u{FB06}"), "best");
        // ﬀ (U+FB00) -> "ff" (2 chars); o + ff = "off".
        assert_eq!(normalize_text("o\u{FB00}"), "off");
    }

    #[test]
    fn hyphen_linebreak_rejoin() {
        assert_eq!(normalize_text("word-\nnext"), "wordnext");
        assert_eq!(normalize_text("word-  \n  next"), "wordnext");
        assert_eq!(normalize_text("word-\n\nnext"), "wordnext");
    }

    #[test]
    fn whitespace_collapse() {
        assert_eq!(normalize_text("a  b\t\tc"), "a b c");
        assert_eq!(normalize_text("a\nb"), "a b");
        assert_eq!(normalize_text("a\u{00A0}b"), "a b"); // U+00A0 NBSP
    }

    #[test]
    fn strip_leading_trailing() {
        assert_eq!(normalize_text("  hello  "), "hello");
        assert_eq!(normalize_text("\n\nhello\t\t"), "hello");
    }

    #[test]
    fn composite_pipeline() {
        // All five steps in one input: NBSP + ligature + hyphen-linebreak
        // + leading/trailing whitespace. `ﬃ` (U+FB03) -> "ffi", so
        // "oﬃce" -> "office".
        let input = "  o\u{FB03}ce sup-\nplies  ";
        assert_eq!(normalize_text(input), "office supplies");
    }

    #[test]
    fn file_separator_codepoints_are_whitespace() {
        // Codex R12-REVIEW-1: Python `re` `\s` matches U+001C..U+001F
        // but Rust `char::is_whitespace()` does NOT. Round 13 fixes
        // the predicate; before the fix, normalize_text("a\u{1c}b")
        // returned "a\u{1c}b"; after the fix, it returns "a b".
        for cp in 0x1C_u32..=0x1F {
            let c = char::from_u32(cp).unwrap();
            let input = format!("a{c}b");
            assert_eq!(
                normalize_text(&input),
                "a b",
                "U+{cp:04X} must be collapsed to single space (Python `re` \\s)"
            );
            // Edge case: leading/trailing file-separator.
            let edge = format!("{c}hello{c}");
            assert_eq!(
                normalize_text(&edge),
                "hello",
                "U+{cp:04X} at edges must strip (collapse+strip equivalent to single space + trim)"
            );
        }
    }

    #[test]
    fn idempotence_on_simple_inputs() {
        // normalize_text(normalize_text(x)) == normalize_text(x)
        // -- the AC-C3 plan-required idempotence property, exercised
        // briefly here as a sanity gate; the full 10k-case proptest
        // lives in tests/normalize_proptest.rs.
        for input in [
            "",
            "hello",
            "  a  b  ",
            "cafe\u{0301}",
            "o\u{FB01}ce",
            "word-\nnext",
            "a\u{1c}b",
            "\u{2028}\u{2029}",
        ] {
            let once = normalize_text(input);
            let twice = normalize_text(&once);
            assert_eq!(
                once, twice,
                "idempotence violated for {input:?}: {once:?} vs {twice:?}"
            );
        }
    }

    #[test]
    fn casefold_table_is_sorted_by_key() {
        // Binary search in `casefold` requires a key-sorted table.
        assert!(
            CASE_FOLD.windows(2).all(|w| w[0].0 < w[1].0),
            "CASE_FOLD must be strictly sorted by key"
        );
    }

    #[test]
    fn normalize_for_lookup_lowercases_ascii() {
        assert_eq!(normalize_for_lookup("Hello WORLD"), "hello world");
    }

    #[test]
    fn normalize_for_lookup_full_folds_sharp_s() {
        // Python `str.casefold()` folds `ß` -> `ss` (full folding);
        // `str.lower()` would leave it as `ß`.
        assert_eq!(normalize_for_lookup("Stra\u{df}e"), "strasse");
    }

    #[test]
    fn normalize_for_lookup_folds_greek_sigmas() {
        // Capital sigma and final sigma both fold to U+03C3.
        assert_eq!(normalize_for_lookup("\u{3a3}"), "\u{3c3}");
        assert_eq!(normalize_for_lookup("\u{3c2}"), "\u{3c3}");
    }

    #[test]
    fn normalize_for_lookup_folds_dotted_i_keeps_dotless_i() {
        // U+0130 (dotted capital I) folds to `i` + U+0307; U+0131
        // (dotless i) folds to itself.
        assert_eq!(normalize_for_lookup("\u{130}"), "i\u{307}");
        assert_eq!(normalize_for_lookup("\u{131}"), "\u{131}");
    }

    #[test]
    fn normalize_for_lookup_runs_normalize_text_before_folding() {
        // Ligature `ﬃ` (U+FB03) is expanded by `normalize_text` to "ffi"
        // BEFORE the casefold step: "OﬃCE" -> "OffiCE" -> "office".
        assert_eq!(normalize_for_lookup("O\u{FB03}CE"), "office");
    }

    #[test]
    fn tokenize_for_lookup_splits_and_folds() {
        assert_eq!(
            tokenize_for_lookup("  The QUICK  brown\tFox  "),
            vec!["the", "quick", "brown", "fox"]
        );
    }

    #[test]
    fn tokenize_for_lookup_empty_inputs_yield_no_tokens() {
        assert!(tokenize_for_lookup("").is_empty());
        assert!(tokenize_for_lookup("   \t\n  ").is_empty());
    }
}
