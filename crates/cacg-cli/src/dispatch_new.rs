//! `kb new` dispatch module.

use std::process::ExitCode;

use cacg_cli::NewArgs;

use crate::dispatch_show::py_repr;

/// `kb new <reading_id> <slug>` dispatcher. Mirrors Python
/// `_cmd_new` (`legacy_python_oracle/src/cacg/cli.py:722`): validates the slug regex
/// `^[a-z0-9][a-z0-9_-]*$`, derives the title (from `--title` or
/// the slug-to-title-case fallback), creates
/// `<cards_dir>/<reading_id>/<slug>.md` with the canonical
/// scaffold template emitted by
/// `cacg_core::card_template::build_card_text`. The scaffold's
/// single citation uses a zero-`chunk_hash` so the linter rejects
/// the freshly-created card until the author fills in real
/// values — the deliberate authoring tripwire.
///
/// Exit-code surface byte-equal with Python `_cmd_new`:
///   - `CACG-CLI-001` (exit 1): cards_dir is a regular file,
///     `mkdir(parents=True)` fails, or the file write fails.
///   - `CACG-CLI-003` (exit 2): `reading_id` or `slug` fails
///     the `^[a-z0-9][a-z0-9_-]*$` regex.
///   - `CACG-CLI-005` (exit 1): the target card file already
///     exists and `--force` was not supplied.
pub(crate) fn dispatch_new(args: &NewArgs) -> ExitCode {
    let slug_re = "^[a-z0-9][a-z0-9_-]*$";
    if !is_valid_slug(&args.reading_id) {
        eprintln!(
            "CACG-CLI-003: reading_id must match {}: {}",
            py_repr(slug_re),
            py_repr(&args.reading_id)
        );
        return ExitCode::from(2);
    }
    if !is_valid_slug(&args.slug) {
        eprintln!(
            "CACG-CLI-003: slug must match {}: {}",
            py_repr(slug_re),
            py_repr(&args.slug)
        );
        return ExitCode::from(2);
    }

    let title = args
        .title
        .clone()
        .unwrap_or_else(|| derive_title_from_slug(&args.slug));

    // is_file()-only check: matches Python's `cards_root.exists()
    // and not cards_root.is_dir()` shape (BL-20260518-shape-check-fs-inputs).
    if args.cards_dir.exists() && !args.cards_dir.is_dir() {
        eprintln!(
            "CACG-CLI-001: cards_dir is not a directory: {}",
            args.cards_dir.display()
        );
        return ExitCode::FAILURE;
    }
    let card_dir = args.cards_dir.join(&args.reading_id);
    if let Err(e) = std::fs::create_dir_all(&card_dir) {
        eprintln!(
            "CACG-CLI-001: cannot create cards directory {}: {e}",
            card_dir.display()
        );
        return ExitCode::FAILURE;
    }
    let out_path = card_dir.join(format!("{}.md", args.slug));
    if out_path.exists() && !args.force {
        eprintln!(
            "CACG-CLI-005: card already exists: {} (use --force to overwrite)",
            out_path.display()
        );
        return ExitCode::FAILURE;
    }

    let text = cacg_core::card_template::build_card_text(&args.slug, &title, &args.reading_id);
    if let Err(e) = std::fs::write(&out_path, &text) {
        eprintln!(
            "CACG-CLI-001: cannot write card {}: {e}",
            out_path.display()
        );
        return ExitCode::FAILURE;
    }

    println!("card: {}", out_path.display());
    println!("reading_id: {}", args.reading_id);
    println!("slug: {}", args.slug);
    println!("status: scaffold (lint will fail until citations are filled in)");
    ExitCode::SUCCESS
}

/// Validate a slug against the Python `_SLUG_RE` pattern
/// `^[a-z0-9][a-z0-9_-]*$`. The Python `kb new` regex is more
/// permissive than `kb ingest`'s source_id regex (which is
/// `^[a-z0-9][a-z0-9_]*$` — no hyphen); this is intentional
/// (cards use kebab-case slugs).
fn is_valid_slug(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
        return false;
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
}

/// Derive a title from a slug, byte-equal with Python's
/// `slug.replace("-", " ").replace("_", " ").title()`.
///
/// Python `str.title()` capitalizes after every NON-CASED
/// character (per the Unicode `Cased` property: cased ⇔
/// `Lu | Ll | Lt`). Digits are UNCASED, so a digit-then-letter
/// boundary starts a new "word" in Python — `"v2test".title()`
/// is `"V2Test"`, not `"V2test"`. The Round-28 review (P1)
/// caught a divergence where the prior implementation used
/// `is_alphanumeric` (which counts digits as alphanumeric and
/// kept `prev_is_*` True after the digit, suppressing the
/// uppercase on the next letter). The fix below tracks
/// `prev_is_cased` instead, which for the ASCII subset the
/// slug regex permits (`^[a-z0-9][a-z0-9_-]*$`) is exactly
/// the predicate Python's `str.iscased()` checks.
pub(crate) fn derive_title_from_slug(slug: &str) -> String {
    let spaced: String = slug
        .chars()
        .map(|c| if c == '-' || c == '_' { ' ' } else { c })
        .collect();
    let mut out = String::with_capacity(spaced.len());
    let mut prev_is_cased = false;
    for ch in spaced.chars() {
        if prev_is_cased {
            out.extend(ch.to_lowercase());
        } else if ch.is_alphabetic() {
            out.extend(ch.to_uppercase());
        } else {
            out.push(ch);
        }
        // "Cased" for our ASCII subset: any ASCII letter is
        // cased; digits / spaces / hyphens / underscores are
        // not. Matches Python `str.iscased()` exactly within
        // the slug-regex character set.
        prev_is_cased = ch.is_alphabetic();
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::derive_title_from_slug;

    /// Round-28 review P1 regression: Python `str.title()`
    /// starts a new word after every non-CASED character, and
    /// digits are non-cased. Locks the byte-equal contract
    /// for every digit-letter boundary in the slug-regex
    /// character set.
    #[test]
    fn derive_title_matches_python_on_digit_letter_boundaries() {
        // Direct byte-equality checks against Python `str.title()`
        // output. The cases cover the full digit-vs-letter
        // adjacency space:
        for (slug_minus_seps, expected) in [
            ("v2test", "V2Test"),
            ("a1b", "A1B"),
            ("1abc", "1Abc"),
            ("abc123def", "Abc123Def"),
            ("foo2bar3baz", "Foo2Bar3Baz"),
        ] {
            assert_eq!(
                derive_title_from_slug(slug_minus_seps),
                expected,
                "digit-letter boundary case {slug_minus_seps:?}: \
                 Python str.title() returns {expected:?}",
            );
        }
    }

    #[test]
    fn derive_title_matches_python_on_hyphen_underscore_seps() {
        for (slug, expected) in [
            ("byte-parity-card", "Byte Parity Card"),
            ("my_card_id", "My Card Id"),
            ("mixed-sep_styles", "Mixed Sep Styles"),
        ] {
            assert_eq!(derive_title_from_slug(slug), expected);
        }
    }

    #[test]
    fn derive_title_lowercases_within_words() {
        // Title() forces the second-and-later letter of each
        // word to lowercase even if the input had uppercase.
        // The slug regex forbids uppercase so this is not
        // reachable from kb new today, but the helper is
        // documented as a byte-equal port and the contract
        // should hold under any input.
        assert_eq!(derive_title_from_slug("ABC"), "Abc");
        assert_eq!(derive_title_from_slug("aBcDe"), "Abcde");
    }
}
