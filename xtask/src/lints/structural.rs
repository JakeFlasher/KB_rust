//! Structural quality checks: function length, nesting depth, module size.
//!
//! Thresholds matching the quality gate policy:
//! - function body ≤ 300 lines
//! - brace-nesting ≤ 6 levels (matching `clippy.toml` `excessive-nesting-threshold`)
//! - module production lines ≤ 1500
//!
//! Excludes `casefold_table.rs` (generated data) and `#[cfg(test)]`
//! blocks from line counts. Production functions with justified deep
//! nesting use `// qg-allow: deep-nesting` annotation.

use std::io;
use std::path::{Path, PathBuf};

use super::{has_comment_marker, Violation};

const RULE_MODULE_SIZE: &str = "structural-module-size";
const RULE_FUNCTION_LENGTH: &str = "structural-function-length";
const RULE_NESTING_DEPTH: &str = "structural-nesting-depth";

const DEFAULT_MODULE_LINE_LIMIT: usize = 1500;
const DEFAULT_FUNCTION_LINE_LIMIT: usize = 300;
const DEFAULT_NESTING_DEPTH_LIMIT: usize = 6;

const GENERATED_FILE_EXEMPTIONS: &[&str] = &["casefold_table.rs"];

pub fn default_scan_roots() -> Vec<PathBuf> {
    vec![PathBuf::from("crates")]
}

pub fn lint(roots: &[PathBuf], warn_mode: bool) -> io::Result<(Vec<Violation>, bool)> {
    let mut violations = Vec::new();
    for root in roots {
        walk_dir(root, &mut violations)?;
    }
    let has_violations = !violations.is_empty();
    if warn_mode && has_violations {
        Ok((violations, false))
    } else {
        Ok((violations, has_violations))
    }
}

fn walk_dir(dir: &Path, violations: &mut Vec<Violation>) -> io::Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    for path in collect_rs_files(dir)? {
        if is_test_path(&path) {
            continue;
        }
        if is_generated_file(&path) {
            continue;
        }
        scan_file(&path, violations)?;
    }
    Ok(())
}

fn collect_rs_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_rs_files_inner(dir, &mut files)?;
    Ok(files)
}

fn collect_rs_files_inner(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if !dir.is_dir() {
        if dir.extension().is_some_and(|e| e == "rs") {
            files.push(dir.to_path_buf());
        }
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files_inner(&path, files)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            files.push(path);
        }
    }
    Ok(())
}

fn is_test_path(path: &Path) -> bool {
    path.components().any(|c| {
        let s = c.as_os_str().to_string_lossy();
        s == "tests" || s == "benches"
    }) || path.file_stem().is_some_and(|s| s == "tests")
}

fn is_generated_file(path: &Path) -> bool {
    path.file_name().is_some_and(|name| {
        let name = name.to_string_lossy();
        GENERATED_FILE_EXEMPTIONS
            .iter()
            .any(|e| name.as_ref() == *e)
    })
}

fn scan_file(path: &Path, violations: &mut Vec<Violation>) -> io::Result<()> {
    let content = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();

    let prod_lines = count_production_lines(&lines);
    if prod_lines > DEFAULT_MODULE_LINE_LIMIT {
        violations.push(Violation {
            file: path.to_path_buf(),
            line: 1,
            rule: RULE_MODULE_SIZE,
            message: format!(
                "module has {prod_lines} production lines (limit: {DEFAULT_MODULE_LINE_LIMIT})"
            ),
        });
    }

    check_function_lengths(path, &lines, violations);
    check_nesting_depth(path, &lines, violations);

    Ok(())
}

fn count_production_lines(lines: &[&str]) -> usize {
    let mut count = 0;
    let mut in_test_block = false;
    let mut brace_depth: usize = 0;
    let mut test_start_depth: Option<usize> = None;

    for line in lines {
        let trimmed = line.trim();

        for ch in trimmed.chars() {
            if ch == '{' {
                brace_depth += 1;
            } else if ch == '}' {
                brace_depth = brace_depth.saturating_sub(1);
            }
        }

        if trimmed == "#[cfg(test)]" {
            in_test_block = true;
            test_start_depth = Some(brace_depth);
            continue;
        }

        if in_test_block {
            if let Some(start_depth) = test_start_depth {
                if brace_depth <= start_depth && trimmed.contains('}') {
                    in_test_block = false;
                    test_start_depth = None;
                }
            }
            continue;
        }

        if !trimmed.is_empty() {
            count += 1;
        }
    }
    count
}

fn is_fn_keyword_line(trimmed: &str) -> bool {
    let prefixes = [
        "fn ",
        "pub fn ",
        "pub(crate) fn ",
        "pub(super) fn ",
        "async fn ",
        "pub async fn ",
        "pub(crate) async fn ",
        "pub(super) async fn ",
        "unsafe fn ",
        "pub unsafe fn ",
    ];
    prefixes.iter().any(|p| trimmed.starts_with(p))
}

fn check_function_lengths(path: &Path, lines: &[&str], violations: &mut Vec<Violation>) {
    let mut in_test_block = false;
    let mut brace_depth: usize = 0;
    let mut test_start_depth: Option<usize> = None;

    let mut pending_fn_name: Option<String> = None;
    let mut pending_fn_start: usize = 0;

    let mut fn_name: Option<String> = None;
    let mut fn_start_line: usize = 0;
    let mut fn_brace_depth: Option<usize> = None;
    let mut fn_line_count: usize = 0;

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        let old_depth = brace_depth;
        for ch in trimmed.chars() {
            if ch == '{' {
                brace_depth += 1;
            } else if ch == '}' {
                brace_depth = brace_depth.saturating_sub(1);
            }
        }

        if trimmed == "#[cfg(test)]" {
            in_test_block = true;
            test_start_depth = Some(brace_depth);
            continue;
        }

        if in_test_block {
            if let Some(start_depth) = test_start_depth {
                if brace_depth <= start_depth && trimmed.contains('}') {
                    in_test_block = false;
                    test_start_depth = None;
                }
            }
            continue;
        }

        if fn_brace_depth.is_some() {
            fn_line_count += 1;
            if let Some(fd) = fn_brace_depth {
                if brace_depth <= fd {
                    if fn_line_count > DEFAULT_FUNCTION_LINE_LIMIT {
                        violations.push(Violation {
                            file: path.to_path_buf(),
                            line: fn_start_line + 1,
                            rule: RULE_FUNCTION_LENGTH,
                            message: format!(
                                "function `{}` body is {} lines (limit: {DEFAULT_FUNCTION_LINE_LIMIT})",
                                fn_name.as_deref().unwrap_or("<unknown>"),
                                fn_line_count
                            ),
                        });
                    }
                    fn_brace_depth = None;
                    fn_name = None;
                    fn_line_count = 0;
                }
            }
        } else if let Some(ref pname) = pending_fn_name {
            if trimmed.contains('{') {
                fn_name = Some(pname.clone());
                fn_start_line = pending_fn_start;
                fn_brace_depth = Some(old_depth);
                fn_line_count = 0;
                pending_fn_name = None;
            }
        } else if is_fn_keyword_line(trimmed) {
            if trimmed.contains('{') {
                let name = extract_fn_name(trimmed);
                fn_name = Some(name);
                fn_start_line = line_idx;
                fn_brace_depth = Some(old_depth);
                fn_line_count = 0;
            } else {
                pending_fn_name = Some(extract_fn_name(trimmed));
                pending_fn_start = line_idx;
            }
        }
    }
}

/// Scan past a raw string body: `i` enters pointing one past the opening `"`,
/// returns pointing one past the closing delimiter (`"` + `hashes` `#` chars).
fn skip_raw_string(bytes: &[u8], mut i: usize, hashes: usize) -> usize {
    while i < bytes.len() {
        if bytes[i] == b'"' && has_closing_hashes(bytes, i, hashes) {
            return i + 1 + hashes;
        }
        i += 1;
    }
    i
}

/// Check whether position `quote_pos` is followed by exactly `hashes` `#` chars.
fn has_closing_hashes(bytes: &[u8], quote_pos: usize, hashes: usize) -> bool {
    let mut n = 0;
    while quote_pos + 1 + n < bytes.len() && bytes[quote_pos + 1 + n] == b'#' && n < hashes {
        n += 1;
    }
    n == hashes
}

fn count_code_braces(line: &str) -> (usize, usize) {
    let mut opens = 0usize;
    let mut closes = 0usize;
    let mut in_string = false;
    let mut in_char = false;
    let mut escape = false;
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let ch = bytes[i];
        if escape {
            escape = false;
            i += 1;
            continue;
        }
        if ch == b'\\' && (in_string || in_char) {
            escape = true;
            i += 1;
            continue;
        }
        if in_string {
            if ch == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        if in_char {
            if ch == b'\'' {
                in_char = false;
            }
            i += 1;
            continue;
        }
        if ch == b'"' {
            // Check for raw string: r"...", r#"..."#, r##"..."##, etc.
            if let Some(hashes) = raw_string_hashes(bytes, i) {
                i = skip_raw_string(bytes, i + 1, hashes);
                continue; // Don't count any braces from the raw string
            }
            // Regular string
            in_string = true;
        } else if ch == b'\'' {
            // Distinguish char literals ('x', '\n', '\u{XXXX}') from lifetimes ('a, 'static)
            if i + 2 < bytes.len() && bytes[i + 2] == b'\'' {
                // 'x' pattern -- simple char literal
                in_char = true;
            } else if i + 3 < bytes.len() && bytes[i + 1] == b'\\' && bytes[i + 3] == b'\'' {
                // '\n' pattern -- escaped char literal
                in_char = true;
            } else if i + 1 < bytes.len() && bytes[i + 1] == b'\\' {
                // Possibly '\u{XXXX}' or '\x41' -- scan forward for closing '
                // to avoid counting braces inside Unicode escapes.
                let mut k = i + 2;
                while k < bytes.len() && bytes[k] != b'\'' {
                    k += 1;
                }
                if k < bytes.len() {
                    // Found closing quote -- skip everything between the quotes
                    i = k + 1;
                    continue;
                }
            }
            // Otherwise it's a lifetime or label -- skip the quote
        } else if ch == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            break;
        } else if ch == b'{' {
            opens += 1;
        } else if ch == b'}' {
            closes += 1;
        }
        i += 1;
    }
    (opens, closes)
}

/// If position `quote_pos` is the opening `"` of a raw string (`r"`, `r#"`, etc.),
/// return the number of `#` delimiters. Otherwise return `None`.
fn raw_string_hashes(bytes: &[u8], quote_pos: usize) -> Option<usize> {
    let mut hashes = 0;
    let mut j = quote_pos;
    while j > 0 && bytes[j - 1] == b'#' {
        hashes += 1;
        j -= 1;
    }
    if j > 0 && bytes[j - 1] == b'r' {
        Some(hashes)
    } else {
        None
    }
}

fn check_nesting_depth(path: &Path, lines: &[&str], violations: &mut Vec<Violation>) {
    let mut in_test_block = false;
    let mut brace_depth: usize = 0;
    let mut test_start_depth: Option<usize> = None;
    let mut reported_lines: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut in_multiline_string = false;

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed == "#[cfg(test)]" {
            in_test_block = true;
            test_start_depth = Some(brace_depth);
        }

        if in_multiline_string {
            if !trimmed.ends_with('\\') {
                in_multiline_string = false;
            }
            continue;
        }

        if trimmed.contains('"') && trimmed.ends_with('\\') {
            in_multiline_string = true;
        }

        let has_nesting_allow = has_comment_marker(line, "qg-allow: deep-nesting")
            || lines
                .get(line_idx + 1)
                .is_some_and(|next| has_comment_marker(next, "qg-allow: deep-nesting"));
        let (opens, closes) = count_code_braces(trimmed);

        for _ in 0..opens {
            brace_depth += 1;
            if !in_test_block
                && !has_nesting_allow
                && brace_depth > DEFAULT_NESTING_DEPTH_LIMIT
                && !reported_lines.contains(&line_idx)
            {
                violations.push(Violation {
                    file: path.to_path_buf(),
                    line: line_idx + 1,
                    rule: RULE_NESTING_DEPTH,
                    message: format!(
                        "brace nesting depth {brace_depth} exceeds limit {DEFAULT_NESTING_DEPTH_LIMIT}"
                    ),
                });
                reported_lines.insert(line_idx);
            }
        }
        for _ in 0..closes {
            brace_depth = brace_depth.saturating_sub(1);
        }

        if in_test_block {
            if let Some(start_depth) = test_start_depth {
                if brace_depth <= start_depth && closes > 0 {
                    in_test_block = false;
                    test_start_depth = None;
                }
            }
        }
    }
}

fn extract_fn_name(line: &str) -> String {
    let after_fn = if let Some(pos) = line.find("fn ") {
        &line[pos + 3..]
    } else {
        line
    };
    let name_end = after_fn
        .find(|c: char| c == '(' || c == '<' || c.is_whitespace())
        .unwrap_or(after_fn.len());
    after_fn[..name_end].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_temp(dir: &Path, name: &str, content: &str) -> PathBuf {
        let p = dir.join(name);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&p, content).unwrap();
        p
    }

    #[test]
    fn small_module_passes() {
        let dir = tempfile::tempdir().unwrap();
        write_temp(
            dir.path(),
            "src/lib.rs",
            "fn main() {\n    println!(\"hello\");\n}\n",
        );
        let (violations, should_fail) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(violations.is_empty());
        assert!(!should_fail);
    }

    #[test]
    fn oversized_module_reports_violation() {
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        for i in 0..1600 {
            content.push_str(&format!("let x{i} = {i};\n"));
        }
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            violations.iter().any(|v| v.rule == RULE_MODULE_SIZE),
            "expected module-size violation"
        );
    }

    #[test]
    fn warn_mode_does_not_fail() {
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        for i in 0..1600 {
            content.push_str(&format!("let x{i} = {i};\n"));
        }
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, should_fail) = lint(&[dir.path().to_path_buf()], true).unwrap();
        assert!(!violations.is_empty());
        assert!(!should_fail, "warn mode should not cause failure");
    }

    #[test]
    fn generated_file_is_exempt() {
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        for i in 0..1600 {
            content.push_str(&format!("let x{i} = {i};\n"));
        }
        write_temp(dir.path(), "src/casefold_table.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(violations.is_empty());
    }

    #[test]
    fn test_block_lines_excluded_from_count() {
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        for i in 0..100 {
            content.push_str(&format!("let x{i} = {i};\n"));
        }
        content.push_str("#[cfg(test)]\nmod tests {\n");
        for i in 0..1500 {
            content.push_str(&format!("    let t{i} = {i};\n"));
        }
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            !violations.iter().any(|v| v.rule == RULE_MODULE_SIZE),
            "test lines should not count toward module size"
        );
    }

    #[test]
    fn oversized_function_reports_violation() {
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::from("fn big_function() {\n");
        for i in 0..310 {
            content.push_str(&format!("    let x{i} = {i};\n"));
        }
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            violations.iter().any(|v| v.rule == RULE_FUNCTION_LENGTH),
            "expected function-length violation"
        );
    }

    #[test]
    fn multiline_signature_function_detected() {
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        content.push_str("pub fn big_multiline(\n");
        content.push_str("    arg1: u32,\n");
        content.push_str("    arg2: String,\n");
        content.push_str(") -> Result<(), Error> {\n");
        for i in 0..310 {
            content.push_str(&format!("    let x{i} = {i};\n"));
        }
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            violations
                .iter()
                .any(|v| v.rule == RULE_FUNCTION_LENGTH && v.message.contains("big_multiline")),
            "expected function-length violation for multi-line signature"
        );
    }

    #[test]
    fn nesting_depth_violation_detected() {
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        content.push_str("fn deep() {\n");
        content.push_str("    if true {\n");
        content.push_str("        if true {\n");
        content.push_str("            if true {\n");
        content.push_str("                if true {\n");
        content.push_str("                    if true {\n");
        content.push_str("                        if true {\n");
        content.push_str("                            let x = 1;\n");
        content.push_str("                        }\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            violations.iter().any(|v| v.rule == RULE_NESTING_DEPTH),
            "expected nesting-depth violation for 7-level brace nesting (limit is 6)"
        );
    }

    #[test]
    fn nesting_depth_at_limit_passes() {
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        content.push_str("fn f() {\n");
        content.push_str("    if true {\n");
        content.push_str("        if true {\n");
        content.push_str("            if true {\n");
        content.push_str("                if true {\n");
        content.push_str("                    let x = 1;\n");
        content.push_str("                }\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            !violations.iter().any(|v| v.rule == RULE_NESTING_DEPTH),
            "depth 6 should not trigger (limit is 6)"
        );
    }

    #[test]
    fn qg_allow_deep_nesting_suppresses() {
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        content.push_str("fn deep() {\n");
        content.push_str("    if true {\n");
        content.push_str("        if true {\n");
        content.push_str("            if true {\n");
        content.push_str("                if true {\n");
        content.push_str("                    if true { // qg-allow: deep-nesting\n");
        content.push_str("                        let x = 1;\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            !violations.iter().any(|v| v.rule == RULE_NESTING_DEPTH),
            "qg-allow: deep-nesting should suppress the nesting violation"
        );
    }

    #[test]
    fn unsuppressed_depth_7_fails() {
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        content.push_str("fn deep() {\n");
        content.push_str("    if true {\n");
        content.push_str("        if true {\n");
        content.push_str("            if true {\n");
        content.push_str("                if true {\n");
        content.push_str("                    if true {\n");
        content.push_str("                        if true {\n");
        content.push_str("                            let x = 1;\n");
        content.push_str("                        }\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            violations.iter().any(|v| v.rule == RULE_NESTING_DEPTH),
            "unsuppressed depth 7 must trigger a nesting violation"
        );
    }

    #[test]
    fn previous_line_allow_does_not_suppress() {
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        content.push_str("fn deep() {\n");
        content.push_str("    if true {\n");
        content.push_str("        if true {\n");
        content.push_str("            if true {\n");
        content.push_str("                if true {\n");
        content.push_str("                    // qg-allow: deep-nesting — on wrong line\n");
        content.push_str("                    if true {\n");
        content.push_str("                        if true {\n");
        content.push_str("                            let x = 1;\n");
        content.push_str("                        }\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            violations.iter().any(|v| v.rule == RULE_NESTING_DEPTH),
            "qg-allow on previous line must NOT suppress the brace line"
        );
    }

    #[test]
    fn next_line_allow_suppresses() {
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        content.push_str("fn deep() {\n");
        content.push_str("    if true {\n");
        content.push_str("        if true {\n");
        content.push_str("            if true {\n");
        content.push_str("                if true {\n");
        content.push_str("                    if true {\n");
        content.push_str("                        // qg-allow: deep-nesting\n");
        content.push_str("                        let x = 1;\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            !violations.iter().any(|v| v.rule == RULE_NESTING_DEPTH),
            "qg-allow on the next line (rustfmt-compatible) should suppress the brace line"
        );
    }

    #[test]
    fn qg_allow_inside_string_does_not_suppress() {
        // A line with the marker inside a string literal should NOT suppress.
        // We need depth > 6 to trigger: fn{1} if{2} if{3} if{4} if{5} if{6} then
        // the string-line's `if true {` pushes to 7, which exceeds limit 6.
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        content.push_str("fn deep() {\n"); // depth 1
        content.push_str("    if true {\n"); // depth 2
        content.push_str("        if true {\n"); // depth 3
        content.push_str("            if true {\n"); // depth 4
        content.push_str("                if true {\n"); // depth 5
        content.push_str("                    if true {\n"); // depth 6
        content
            .push_str("                        let s = \"// qg-allow: deep-nesting\"; if true {\n"); // depth 7
        content.push_str("                            let x = 1;\n");
        content.push_str("                        }\n");
        // close all braces
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            violations.iter().any(|v| v.rule == RULE_NESTING_DEPTH),
            "qg-allow inside a string literal must NOT suppress the violation"
        );
    }

    #[test]
    fn nesting_in_test_block_is_exempt() {
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        content.push_str("#[cfg(test)]\nmod tests {\n");
        content.push_str("    fn deep() {\n");
        for _ in 0..8 {
            content.push_str("        {\n");
        }
        content.push_str("            let x = 1;\n");
        for _ in 0..8 {
            content.push_str("        }\n");
        }
        content.push_str("    }\n");
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            !violations.iter().any(|v| v.rule == RULE_NESTING_DEPTH),
            "test code nesting should be exempt"
        );
    }

    #[test]
    fn braces_in_string_not_counted() {
        // Braces inside a string literal must not affect nesting depth.
        // Build code at nesting depth exactly 6 (the limit), with a string
        // containing an extra '{' that would push it to 7 if mis-counted.
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        content.push_str("fn f() {\n"); // depth 1
        content.push_str("    if true {\n"); // depth 2
        content.push_str("        if true {\n"); // depth 3
        content.push_str("            if true {\n"); // depth 4
        content.push_str("                if true {\n"); // depth 5
        content.push_str("                    let s = \"{\"; if true {\n"); // depth 6
        content.push_str("                        let x = 1;\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            !violations.iter().any(|v| v.rule == RULE_NESTING_DEPTH),
            "brace inside string literal should not be counted"
        );
    }

    #[test]
    fn braces_in_line_comment_not_counted() {
        // Braces after `//` must not affect nesting depth.
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        content.push_str("fn f() {\n"); // depth 1
        content.push_str("    if true {\n"); // depth 2
        content.push_str("        if true {\n"); // depth 3
        content.push_str("            if true {\n"); // depth 4
        content.push_str("                if true {\n"); // depth 5
        content.push_str("                    // { { {\n"); // comment braces
        content.push_str("                    if true {\n"); // depth 6
        content.push_str("                        let x = 1;\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            !violations.iter().any(|v| v.rule == RULE_NESTING_DEPTH),
            "braces inside line comments should not be counted"
        );
    }

    #[test]
    fn lifetime_annotation_does_not_hide_brace() {
        // Lifetime annotations ('a) must not confuse the parser into
        // missing real braces on the same line.
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        content.push_str("mod outer {\n"); // depth 1
        content.push_str("    mod inner {\n"); // depth 2
        content.push_str("        fn foo<'a>(x: &'a str) {\n"); // depth 3
        content.push_str("            if true {\n"); // depth 4
        content.push_str("                if true {\n"); // depth 5
        content.push_str("                    if true {\n"); // depth 6
        content.push_str("                        if true {\n"); // depth 7 — violation
        content.push_str("                            let y = x;\n");
        content.push_str("                        }\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            violations.iter().any(|v| v.rule == RULE_NESTING_DEPTH),
            "lifetime annotations must not hide the opening brace — depth 7 should be detected"
        );
    }

    #[test]
    fn raw_string_with_hashes_not_counted() {
        // Braces inside r#"..."# must not affect nesting depth.
        let dir = tempfile::tempdir().unwrap();
        let mut content = String::new();
        content.push_str("fn f() {\n"); // depth 1
        content.push_str("    if true {\n"); // depth 2
        content.push_str("        if true {\n"); // depth 3
        content.push_str("            if true {\n"); // depth 4
        content.push_str("                if true {\n"); // depth 5
        content.push_str("                    let s = r#\"{ { {\"#; if true {\n"); // depth 6
        content.push_str("                        let x = 1;\n");
        content.push_str("                    }\n");
        content.push_str("                }\n");
        content.push_str("            }\n");
        content.push_str("        }\n");
        content.push_str("    }\n");
        content.push_str("}\n");
        write_temp(dir.path(), "src/lib.rs", &content);
        let (violations, _) = lint(&[dir.path().to_path_buf()], false).unwrap();
        assert!(
            !violations.iter().any(|v| v.rule == RULE_NESTING_DEPTH),
            "braces inside r#\"...\"# raw string should not be counted"
        );
    }
}
