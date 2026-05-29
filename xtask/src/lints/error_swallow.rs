//! Reject error-discarding patterns in production code.
//!
//! Targets `.unwrap_or_default()` and `.ok()` patterns that silently
//! discard `Result::Err`. Allowlist via `// qg-allow: intentional-discard`.
//!
//! Exemptions:
//! - `#[cfg(test)]` modules and `tests/` directories
//! - Lines containing `// qg-allow: intentional-discard`
//! - `xtask/` (relaxed scope)

use std::io;
use std::path::{Path, PathBuf};

use super::{has_comment_marker, Violation};

const RULE_UNWRAP_DEFAULT: &str = "error-swallow-unwrap-or-default";
const RULE_OK_DISCARD: &str = "error-swallow-ok-discard";

pub fn default_scan_roots() -> Vec<PathBuf> {
    vec![PathBuf::from("crates")]
}

pub fn lint(roots: &[PathBuf]) -> io::Result<Vec<Violation>> {
    let mut violations = Vec::new();
    for root in roots {
        walk_dir(root, &mut violations)?;
    }
    Ok(violations)
}

fn walk_dir(dir: &Path, violations: &mut Vec<Violation>) -> io::Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    for path in collect_rs_files(dir)? {
        if is_test_path(&path) {
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

fn scan_file(path: &Path, violations: &mut Vec<Violation>) -> io::Result<()> {
    let content = std::fs::read_to_string(path)?;
    let mut in_test_block = false;
    let mut brace_depth_at_test_start: Option<usize> = None;
    let mut brace_depth: usize = 0;

    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        for ch in line.chars() {
            match ch {
                '{' => brace_depth += 1,
                '}' => brace_depth = brace_depth.saturating_sub(1),
                _ => {}
            }
        }

        if trimmed == "#[cfg(test)]" {
            in_test_block = true;
            brace_depth_at_test_start = Some(brace_depth);
            continue;
        }

        if in_test_block {
            if let Some(start_depth) = brace_depth_at_test_start {
                if brace_depth <= start_depth && trimmed.contains('}') {
                    in_test_block = false;
                    brace_depth_at_test_start = None;
                }
            }
            continue;
        }

        if trimmed.starts_with("//") {
            continue;
        }

        if has_comment_marker(line, "qg-allow: intentional-discard") {
            continue;
        }

        if line.contains(".unwrap_or_default()") {
            violations.push(Violation {
                file: path.to_path_buf(),
                line: line_idx + 1,
                rule: RULE_UNWRAP_DEFAULT,
                message: "`.unwrap_or_default()` silently discards errors; propagate with `?` or use `// qg-allow: intentional-discard <reason>`".to_string(),
            });
        }

        // Flag .ok() that discards Result::Err, but NOT .ok()? which is a
        // legitimate Option-returning conversion pattern.
        if line.contains(".ok()") && !line.contains(".ok()?") {
            // Also skip .ok().is_some(), .ok().is_none() — these are Option
            // queries, not error discard.
            let has_option_query = line.contains(".ok().is_some()")
                || line.contains(".ok().is_none()")
                || line.contains(".ok().map(")
                || line.contains(".ok().and_then(");
            if !has_option_query {
                violations.push(Violation {
                    file: path.to_path_buf(),
                    line: line_idx + 1,
                    rule: RULE_OK_DISCARD,
                    message: "`.ok()` silently discards errors; propagate with `?` or use `// qg-allow: intentional-discard <reason>`".to_string(),
                });
            }
        }
    }
    Ok(())
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
    fn clean_code_passes() {
        let dir = tempfile::tempdir().unwrap();
        write_temp(
            dir.path(),
            "src/lib.rs",
            "fn main() {\n    let x = foo()?;\n}\n",
        );
        let violations = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(violations.is_empty());
    }

    #[test]
    fn unwrap_or_default_in_production_fails() {
        let dir = tempfile::tempdir().unwrap();
        write_temp(
            dir.path(),
            "src/lib.rs",
            "fn f() {\n    let x = result.unwrap_or_default();\n}\n",
        );
        let violations = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule, RULE_UNWRAP_DEFAULT);
    }

    #[test]
    fn ok_discard_in_production_fails() {
        let dir = tempfile::tempdir().unwrap();
        write_temp(
            dir.path(),
            "src/lib.rs",
            "fn f() {\n    let x = result.ok();\n}\n",
        );
        let violations = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].rule, RULE_OK_DISCARD);
    }

    #[test]
    fn ok_question_mark_is_exempt() {
        let dir = tempfile::tempdir().unwrap();
        write_temp(
            dir.path(),
            "src/lib.rs",
            "fn f() -> Option<T> {\n    let x = result.ok()?;\n}\n",
        );
        let violations = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(violations.is_empty());
    }

    #[test]
    fn test_code_is_exempt() {
        let dir = tempfile::tempdir().unwrap();
        write_temp(
            dir.path(),
            "src/lib.rs",
            "fn prod() {}\n\n#[cfg(test)]\nmod tests {\n    fn t() { x.unwrap_or_default(); }\n}\n",
        );
        let violations = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(violations.is_empty());
    }

    #[test]
    fn qg_allow_annotation_suppresses() {
        let dir = tempfile::tempdir().unwrap();
        write_temp(
            dir.path(),
            "src/lib.rs",
            "fn f() {\n    let x = r.unwrap_or_default(); // qg-allow: intentional-discard reason\n}\n",
        );
        let violations = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(violations.is_empty());
    }
}
