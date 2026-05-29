//! Reject verify-path runner bypasses in `cacg-cli` round-summary
//! sources.
//!
//! `cacg-core::verify::verify_one_card` is the canonical single-card
//! runner that owns the "exactly one `command="verify"` journal
//! event per cited card per invocation" contract. Any direct call to
//! `cacg_core::journal::append_entry(...)` or
//! `cacg_core::verify::layer2::verify_card(...)` from the CLI's
//! round-summary surface bypasses that cardinality contract and
//! risks double-appended or missing journal events across the
//! batch path. The static-grep guard below catches such bypasses
//! before merge.
//!
//! Scope: filenames whose basename matches `round_summary*.rs`
//! inside any of the configured scan roots (default:
//! `crates/cacg-cli/src`). Other CLI files (e.g., `main.rs`) are
//! intentionally not scanned — the lint targets the bounded
//! round-summary surface where the verify-batch driver lives.
//!
//! Allowlists:
//! - The lint's own source file
//!   (`xtask/src/lints/runner_bypass.rs`) contains the needle
//!   patterns and unit-test fixtures.
//! - Any path containing `/tests/` — test code is allowed to call
//!   the underlying primitives directly for fixture setup.
//! - Lines starting with `//` (after trim) are exempt so module-
//!   doc and inline comments that mention the patterns are not
//!   flagged.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::Violation;

const RULE: &str = "runner-bypass";

/// File-path suffixes intentionally allowed to contain the needle
/// patterns this lint scans for.
const ALLOWED_FILE_SUFFIXES: &[&str] = &["xtask/src/lints/runner_bypass.rs"];

/// Production default scan root: `crates/cacg-cli/src`. The lint
/// recurses below this root but filters file scanning to filenames
/// whose basename starts with `round_summary`.
#[must_use]
pub fn default_scan_roots() -> Vec<PathBuf> {
    vec![PathBuf::from("crates/cacg-cli/src")]
}

/// Identifier patterns this lint flags as direct call expressions.
/// Each entry pairs the bare identifier with the human-readable
/// description. The match is identifier-aware (see
/// `line_has_forbidden_call`) so any of these forms surface:
///
/// * `verify_card(...)` — direct, no whitespace.
/// * `verify_card (...)` — direct, with one or more spaces /
///   tabs between the identifier and `(`.
/// * `cacg_core::verify::layer2::verify_card(...)` — qualified.
/// * `cacg_core::verify::layer2::verify_card (...)` — qualified
///   plus whitespace.
/// * Same five shapes for `append_entry`.
///
/// The matcher does NOT trigger on `verify_card_helper(...)`
/// (different identifier — the `_` following the needle is an
/// identifier-continuation byte, not whitespace or `(`), nor on
/// `_verify_card(...)` (the leading `_` makes the left boundary
/// fail), nor on `.verify_card_field` (the byte after the needle
/// is `_`, an identifier-continuation char).
const FORBIDDEN_IDENTS: &[(&str, &str)] = &[
    (
        "append_entry",
        "direct journal append bypasses cacg_core::verify::verify_one_card; the runner owns the exactly-one-event-per-card-per-invocation contract",
    ),
    (
        "verify_card",
        "direct verify_card call bypasses cacg_core::verify::verify_one_card; route through the runner so journal cardinality holds",
    ),
];

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Walk every `.rs` file under `roots` and return every violation
/// found inside files whose basename matches the round-summary
/// pattern.
pub fn lint(roots: &[PathBuf]) -> io::Result<Vec<Violation>> {
    let mut violations = Vec::new();
    for root in roots {
        let mut stack: Vec<PathBuf> = vec![root.clone()];
        while let Some(dir) = stack.pop() {
            let entries = match fs::read_dir(&dir) {
                Ok(e) => e,
                Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
                Err(err) => return Err(err),
            };
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file_type = entry.file_type()?;
                if file_type.is_dir() {
                    stack.push(path);
                    continue;
                }
                if file_type.is_file() && is_round_summary_rs(&path) {
                    scan_file(&path, &mut violations)?;
                }
            }
        }
    }
    Ok(violations)
}

/// True iff the file's extension is `.rs` AND its basename
/// starts with `round_summary` (e.g., `round_summary.rs`,
/// `round_summary_helper.rs`). The lint is scoped to this
/// bounded surface.
fn is_round_summary_rs(path: &Path) -> bool {
    if path.extension().and_then(|s| s.to_str()) != Some("rs") {
        return false;
    }
    match path.file_name().and_then(|s| s.to_str()) {
        Some(name) => name.starts_with("round_summary"),
        None => false,
    }
}

fn is_allowlisted(path: &Path) -> bool {
    let s = path.to_string_lossy();
    if ALLOWED_FILE_SUFFIXES
        .iter()
        .any(|suffix| s.ends_with(suffix))
    {
        return true;
    }
    // Any path containing `/tests/` is test code; allow direct
    // trust-kernel primitive calls there because test fixtures
    // never reach committed runtime artifacts.
    s.contains("/tests/")
}

/// Build a 1-indexed line-start table. `line_starts[0]` is the byte
/// offset where line 1 begins (always 0); `line_starts[i]` (for
/// `i > 0`) is the byte offset where line `i+1` begins (the byte
/// after the `i`-th newline). The table is used to map any byte
/// offset back to a 1-based line number via binary search.
fn build_line_starts(bytes: &[u8]) -> Vec<usize> {
    let mut starts = vec![0usize];
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'\n' {
            starts.push(i + 1);
        }
    }
    starts
}

/// Map a byte offset to its 1-based line number using the
/// monotonic `line_starts` table.
fn line_number_at(line_starts: &[usize], offset: usize) -> usize {
    // `partition_point` returns the count of elements strictly less
    // than or equal to the predicate-flip; we want the largest index
    // `j` such that `line_starts[j] <= offset`, plus 1 for 1-based.
    match line_starts.binary_search(&offset) {
        Ok(j) => j + 1,
        Err(j) => j, // `j` is the insert position; the line begins at j-1, so 1-based = j
    }
}

/// Extract the full text of the line containing `offset` (without
/// its trailing newline). Used to populate the violation message
/// and to apply the comment-line exemption against the trimmed
/// start of the identifier's line.
fn line_text_at<'a>(contents: &'a str, line_starts: &[usize], offset: usize) -> &'a str {
    let bytes = contents.as_bytes();
    let line_idx = line_number_at(line_starts, offset) - 1;
    let start = line_starts[line_idx];
    let end = if line_idx + 1 < line_starts.len() {
        line_starts[line_idx + 1] - 1 // strip the trailing '\n'
    } else {
        bytes.len()
    };
    // Walk back from `end` if there's a trailing '\r' (CRLF).
    let end = if end > start && bytes[end.saturating_sub(1)] == b'\r' {
        end - 1
    } else {
        end
    };
    // SAFETY: contents is &str, all indices land at UTF-8 boundaries
    // (line breaks are ASCII bytes).
    std::str::from_utf8(&bytes[start..end]).unwrap_or("")
}

/// Return the byte offset of the first NON-trivia byte at or after
/// `start`. "Trivia" is the full Rust inter-token grammar that can
/// legally appear between a callee path and its call paren:
///
/// * ASCII whitespace (space, tab, `\n`, `\r`, FF, VT).
/// * Line comments: `//` through (but not past) the next `\n`, or
///   end-of-file.
/// * Block comments: `/* ... */`, NESTING-aware — Rust uniquely
///   allows `/* a /* b */ c */` as a single comment. An
///   unterminated block comment consumes to end-of-file (such a
///   file does not compile; no `(` follows, so no violation).
///
/// The three kinds interleave freely; the scanner loops until it
/// reaches a byte that begins no trivia construct.
///
/// String / char literals are intentionally NOT tracked: a string
/// literal cannot legally appear between a function path and its
/// call paren (it is a syntax error), so none can occur in the
/// region this scanner walks.
fn skip_trivia(bytes: &[u8], start: usize) -> usize {
    let mut j = start;
    loop {
        // ASCII whitespace.
        while j < bytes.len() && bytes[j].is_ascii_whitespace() {
            j += 1;
        }
        // Line comment: `//` ... `\n`.
        if j + 1 < bytes.len() && bytes[j] == b'/' && bytes[j + 1] == b'/' {
            j += 2;
            while j < bytes.len() && bytes[j] != b'\n' {
                j += 1;
            }
            continue;
        }
        // Block comment: `/* ... */`, nesting-aware.
        if j + 1 < bytes.len() && bytes[j] == b'/' && bytes[j + 1] == b'*' {
            j += 2;
            let mut depth = 1usize;
            while j < bytes.len() && depth > 0 {
                if j + 1 < bytes.len() && bytes[j] == b'/' && bytes[j + 1] == b'*' {
                    depth += 1;
                    j += 2;
                } else if j + 1 < bytes.len() && bytes[j] == b'*' && bytes[j + 1] == b'/' {
                    depth -= 1;
                    j += 2;
                } else {
                    j += 1;
                }
            }
            continue;
        }
        break;
    }
    j
}

/// Walk `contents` for occurrences of `ident` whose left + right
/// identifier-boundary checks pass and whose identifier-start line
/// is not a `//`-prefixed comment. Each match pushes one
/// [`Violation`] into `violations`.
fn collect_file_violations(
    path: &Path,
    contents: &str,
    line_starts: &[usize],
    ident: &str,
    message: &str,
    violations: &mut Vec<Violation>,
) {
    let bytes = contents.as_bytes();
    let needle = ident.as_bytes();
    if needle.is_empty() || needle.len() > bytes.len() {
        return;
    }
    let mut i = 0usize;
    while i + needle.len() <= bytes.len() {
        if &bytes[i..i + needle.len()] != needle {
            i += 1;
            continue;
        }
        // Left boundary check.
        let left_ok = i == 0 || !is_ident_byte(bytes[i - 1]);
        if !left_ok {
            i += 1;
            continue;
        }
        // Right boundary: skip the full Rust inter-token trivia
        // grammar (whitespace + line comments + nested block
        // comments), then require `(`. This catches direct calls
        // however the source spaces / comments the callee from
        // the call paren.
        let j = skip_trivia(bytes, i + needle.len());
        if j >= bytes.len() || bytes[j] != b'(' {
            i += 1;
            continue;
        }
        // Comment-line exemption: skip when the identifier's
        // starting line begins with `//` after trim_start.
        let line = line_text_at(contents, line_starts, i);
        if line.trim_start().starts_with("//") {
            i += 1;
            continue;
        }
        violations.push(Violation {
            file: path.to_path_buf(),
            line: line_number_at(line_starts, i),
            rule: RULE,
            message: format!("{message}: {}", line.trim()),
        });
        // Advance past this identifier so we do not double-report a
        // single literal occurrence under multiple shifted starts
        // (which could happen on substrings like `verify_card`
        // matching its own suffix in degenerate cases).
        i += needle.len();
    }
}

fn scan_file(path: &Path, violations: &mut Vec<Violation>) -> io::Result<()> {
    if is_allowlisted(path) {
        return Ok(());
    }
    let contents = fs::read_to_string(path)?;
    let line_starts = build_line_starts(contents.as_bytes());
    for (ident, message) in FORBIDDEN_IDENTS {
        collect_file_violations(path, &contents, &line_starts, ident, message, violations);
    }
    // Re-sort violations by (line, original push order) so the
    // output ordering remains stable across both identifiers when
    // both fire on the same file. Stable sort preserves push order
    // within a single line.
    violations.sort_by_key(|v| (v.file.clone(), v.line));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write(dir: &Path, name: &str, body: &str) -> PathBuf {
        let p = dir.join(name);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, body).unwrap();
        p
    }

    #[test]
    fn clean_round_summary_file_passes() {
        // A round-summary source that calls the canonical runner
        // entrypoint passes; no append_entry or verify_card direct
        // calls.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "use cacg_core::verify::verify_one_card;\nfn drive() { let _ = verify_one_card(/*...*/); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(
            v.is_empty(),
            "clean round-summary source must pass; got: {v:?}"
        );
    }

    #[test]
    fn flags_append_entry_in_round_summary() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn t() { cacg_core::journal::append_entry(&path, &entry, &id, &ts).unwrap(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("journal append"));
        assert_eq!(v[0].rule, RULE);
    }

    #[test]
    fn flags_verify_card_in_round_summary() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn t() { let out = verify_card(&doc, &idx, false, None, None); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("verify_card call"));
        assert_eq!(v[0].rule, RULE);
    }

    #[test]
    fn flags_both_triggers_in_round_summary_helper_file() {
        // round_summary_helper.rs matches the basename pattern and
        // is scanned; both triggers on different lines surface as
        // two violations.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary_helper.rs",
            "fn t() {\n    append_entry(&p, &e, &id, &ts).unwrap();\n    let _ = verify_card(&d, &i, false, None, None);\n}\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 2, "got: {v:?}");
    }

    #[test]
    fn ignores_non_round_summary_files() {
        // A file whose basename does NOT start with
        // `round_summary` is not scanned, even if it contains
        // append_entry / verify_card calls. This keeps the lint
        // bounded to the documented surface.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "main.rs",
            "fn t() { append_entry(&p, &e, &id, &ts).unwrap(); verify_card(&d, &i, false, None, None); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(
            v.is_empty(),
            "non-round_summary file must not be scanned; got: {v:?}"
        );
    }

    #[test]
    fn ignores_comments() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "// uses append_entry internally for the helper\n/// see verify_card(...) docs\nfn ok() {}\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(v.is_empty(), "comments must be exempt; got: {v:?}");
    }

    #[test]
    fn allows_in_tests_directory() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "crates/cacg-cli/tests/round_summary_test.rs",
            "fn t() { let out = verify_card(&doc, &idx, false, None, None); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(v.is_empty(), "tests/ paths must be allowlisted; got: {v:?}");
    }

    #[test]
    fn allows_in_runner_bypass_own_source() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "xtask/src/lints/runner_bypass.rs",
            "fn t() { append_entry(); verify_card(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(
            v.is_empty(),
            "the lint's own source must be allowlisted; got: {v:?}"
        );
    }

    #[test]
    fn flags_verify_card_with_space_before_paren() {
        // Rust accepts whitespace between the identifier and the
        // call-paren. The exact-substring matcher missed this
        // form; the identifier-aware matcher must catch it.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn drive() { let _ = verify_card (&doc, &idx, false, None, None); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(
            v.len(),
            1,
            "spaced `verify_card (...)` must fire; got: {v:?}"
        );
        assert!(v[0].message.contains("verify_card call"));
        assert_eq!(v[0].rule, RULE);
    }

    #[test]
    fn flags_append_entry_with_space_before_paren() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn t() { cacg_core::journal::append_entry (&p, &e, &id, &ts).unwrap(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(
            v.len(),
            1,
            "spaced qualified `append_entry (...)` must fire; got: {v:?}"
        );
        assert!(v[0].message.contains("journal append"));
    }

    #[test]
    fn flags_qualified_verify_card_with_space_before_paren() {
        // Fully-qualified path + whitespace before `(`.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn t() { cacg_core::verify::layer2::verify_card (&doc, &idx, false, None, None); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(
            v.len(),
            1,
            "qualified spaced `verify_card (...)` must fire; got: {v:?}"
        );
    }

    #[test]
    fn does_not_flag_verify_card_helper_identifier() {
        // `verify_card_helper(` is a DIFFERENT identifier that
        // happens to start with the needle bytes. The
        // identifier-aware matcher must NOT fire on it (the byte
        // after the needle is `_`, an identifier-continuation
        // char that is neither whitespace nor `(`).
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn verify_card_helper() {}\nfn driver() { verify_card_helper(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(
            v.is_empty(),
            "matcher must not fire on `verify_card_helper`; got: {v:?}"
        );
    }

    #[test]
    fn does_not_flag_underscore_prefixed_identifier() {
        // `_verify_card(` has the needle bytes preceded by `_`
        // (identifier-continuation). The left boundary check must
        // reject this — `_verify_card` is a different identifier.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn _verify_card() {}\nfn driver() { _verify_card(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(
            v.is_empty(),
            "matcher must not fire on `_verify_card`; got: {v:?}"
        );
    }

    #[test]
    fn flags_verify_card_with_newline_before_paren() {
        // The call paren is on a different line from the
        // identifier. Rust accepts this as a function call. The
        // line-by-line scanner missed it; the whole-file scanner
        // must catch it.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn drive() {\n    let _ = verify_card\n        (&doc, &idx, false, None, None);\n}\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(
            v.len(),
            1,
            "newline `verify_card\\n(` must fire; got: {v:?}"
        );
        // The violation must be reported at the line where the
        // identifier starts, not the line where the paren lives.
        assert_eq!(
            v[0].line, 2,
            "violation should reference identifier-start line; got: {v:?}"
        );
        assert!(v[0].message.contains("verify_card call"));
        assert_eq!(v[0].rule, RULE);
    }

    #[test]
    fn flags_append_entry_with_newline_before_paren() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn t() {\n    cacg_core::journal::append_entry\n        (&p, &e, &id, &ts).unwrap();\n}\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(
            v.len(),
            1,
            "qualified `append_entry\\n(` must fire; got: {v:?}"
        );
        assert!(v[0].message.contains("journal append"));
    }

    #[test]
    fn flags_qualified_verify_card_with_newline_before_paren() {
        // Fully qualified path + newline before `(`, in the
        // helper file (basename also matches `round_summary*`).
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary_helper.rs",
            "fn t() {\n    cacg_core::verify::layer2::verify_card\n        (&doc, &idx, false, None, None);\n}\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(
            v.len(),
            1,
            "qualified `verify_card\\n(` in helper file must fire; got: {v:?}"
        );
    }

    #[test]
    fn comment_line_carries_through_to_next_line_paren() {
        // A line that starts with `//` may legally mention the
        // identifier in prose. Even if the next line happens to
        // start with `(...)`, the identifier itself is on the
        // comment line so the comment-line exemption holds.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "// see verify_card\n    (...) for context\nfn ok() {}\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(
            v.is_empty(),
            "identifier on a `//` line must remain exempt; got: {v:?}"
        );
    }

    #[test]
    fn does_not_flag_verify_card_helper_with_newline() {
        // The right-boundary check must keep rejecting
        // `verify_card_helper` (the `_` after the needle is
        // identifier-continuation) even when the call paren is on
        // the next line.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn t() {\n    verify_card_helper\n        ();\n}\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(
            v.is_empty(),
            "must not fire on `verify_card_helper\\n()`; got: {v:?}"
        );
    }

    #[test]
    fn flags_verify_card_with_block_comment_before_paren() {
        // Rust treats `/* ... */` as inter-token trivia, so a
        // block comment between the callee and `(` is still a
        // direct call. The whitespace-only skip missed this.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn drive() { let _ = verify_card /* bypass */ (&doc, &idx, false, None, None); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "`verify_card /* c */ (` must fire; got: {v:?}");
        assert!(v[0].message.contains("verify_card call"));
        assert_eq!(v[0].rule, RULE);
    }

    #[test]
    fn flags_append_entry_with_block_comment_before_paren() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn t() { cacg_core::journal::append_entry /* c */ (&p, &e, &id, &ts).unwrap(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(
            v.len(),
            1,
            "qualified `append_entry /* c */ (` must fire; got: {v:?}"
        );
        assert!(v[0].message.contains("journal append"));
    }

    #[test]
    fn flags_verify_card_with_line_comment_before_paren() {
        // A `//` line comment after the identifier consumes to the
        // newline; the call paren on the next line is still a
        // direct call. The identifier itself is NOT on a comment
        // line, so the full-line-comment exemption does not apply.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn drive() {\n    let _ = verify_card // bypass\n        (&doc, &idx, false, None, None);\n}\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "`verify_card // c\\n(` must fire; got: {v:?}");
        assert_eq!(
            v[0].line, 2,
            "violation references identifier-start line; got: {v:?}"
        );
    }

    #[test]
    fn flags_qualified_verify_card_with_block_comment_in_helper() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary_helper.rs",
            "fn t() { cacg_core::verify::layer2::verify_card /* c */ (&doc, &idx, false, None, None); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(
            v.len(),
            1,
            "qualified `verify_card /* c */ (` in helper file must fire; got: {v:?}"
        );
    }

    #[test]
    fn flags_verify_card_with_nested_block_comment_before_paren() {
        // Rust block comments nest. The trivia scanner's depth
        // counter must consume the WHOLE `/* outer /* inner */
        // outer */` before reaching `(`.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn t() { verify_card /* outer /* inner */ outer */ (&d, &i, false, None, None); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(
            v.len(),
            1,
            "nested-block-comment `verify_card /* .. */ (` must fire; got: {v:?}"
        );
    }

    #[test]
    fn does_not_flag_division_after_identifier() {
        // `verify_card / 2` is integer division, not a call. A `/`
        // that begins neither `//` nor `/*` is not trivia; the
        // trivia scanner stops at `/`, which is not `(`.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "round_summary.rs",
            "fn t() { let x = verify_card / 2; }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(
            v.is_empty(),
            "division after the identifier must not fire; got: {v:?}"
        );
    }

    #[test]
    fn flags_runner_bypass_under_default_roots() {
        // Synthesize the production layout under a tempdir and
        // resolve `default_scan_roots()` against the tempdir
        // prefix. Confirms the default-root construction catches a
        // round-summary bypass.
        let workspace = TempDir::new().unwrap();
        write(
            workspace.path(),
            "crates/cacg-cli/src/round_summary.rs",
            "fn drive() { let out = verify_card(&doc, &idx, false, None, None); }\n",
        );
        let roots: Vec<PathBuf> = default_scan_roots()
            .iter()
            .map(|r| workspace.path().join(r))
            .collect();
        let v = lint(&roots).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].file.to_string_lossy().contains("round_summary.rs"));
        assert_eq!(v[0].rule, RULE);
    }
}
