//! Reject source-level platform branches that bypass the documented matrix.
//!
//! The trust kernel's documented platform matrix is Linux x86_64 + Linux
//! aarch64 first-class, macOS best-effort (build-only), Windows non-goal.
//! Source files that introduce `#[cfg(target_os = "windows")]` (or other
//! Windows-targeting cfg branches) silently broaden the matrix and are
//! rejected here at lint time. macOS-specific cfg branches are also flagged
//! because the macOS check is build-only; no runtime macOS-specific code
//! is permitted in the trust kernel.
//!
//! Scope: `crates/cacg-core/src/**/*.rs` only. The `cacg-ingest` crate
//! carries the only intentional FFI surface and uses different lints.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::Violation;

const RULE: &str = "platform-cfg-bypass";

/// Walk the source tree under `root` and return every violation found.
pub fn lint(root: &Path) -> io::Result<Vec<Violation>> {
    let mut violations = Vec::new();
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
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
            if file_type.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                scan_file(&path, &mut violations)?;
            }
        }
    }
    Ok(violations)
}

fn scan_file(path: &Path, violations: &mut Vec<Violation>) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    for (idx, line) in contents.lines().enumerate() {
        // Comments are exempt: lines starting with `//` (after leading
        // whitespace) are not source-level cfg branches.
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") {
            continue;
        }
        let lower = line.to_lowercase();
        let triggers = [
            ("target_os = \"windows\"", "Windows targeting bypasses the documented platform matrix (Linux first-class only)"),
            ("target_os=\"windows\"", "Windows targeting bypasses the documented platform matrix"),
            ("target_family = \"windows\"", "Windows family targeting bypasses the documented platform matrix"),
            ("cfg(windows)", "`cfg(windows)` predicate bypasses the documented platform matrix"),
            ("target_os = \"macos\"", "macOS runtime branching is not permitted in the trust kernel (build-only matrix)"),
            ("target_os=\"macos\"", "macOS runtime branching is not permitted in the trust kernel"),
        ];
        for (needle, message) in triggers {
            if lower.contains(needle) {
                violations.push(Violation {
                    file: path.to_path_buf(),
                    line: idx + 1,
                    rule: RULE,
                    message: format!("{message}: {}", line.trim()),
                });
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(content: &str, ext: &str) -> tempfile_alt::Dir {
        // Simple tmpdir implementation that mirrors what `tempfile` would
        // give us without pulling the dependency into the dev surface.
        let dir = tempfile_alt::Dir::new();
        let path = dir.path.join(format!("subject.{ext}"));
        std::fs::write(&path, content).unwrap();
        dir
    }

    #[test]
    fn flags_windows_cfg_branch() {
        let dir = tmp(
            "#[cfg(target_os = \"windows\")]\nfn windows_only() {}\n",
            "rs",
        );
        let violations = lint(&dir.path).expect("lint did not error");
        assert_eq!(
            violations.len(),
            1,
            "expected one violation, got {violations:?}"
        );
        assert_eq!(violations[0].rule, RULE);
        assert_eq!(violations[0].line, 1);
    }

    #[test]
    fn flags_cfg_windows_short_form() {
        let dir = tmp("#[cfg(windows)]\nfn alt() {}\n", "rs");
        let violations = lint(&dir.path).expect("lint did not error");
        assert_eq!(
            violations.len(),
            1,
            "expected one violation, got {violations:?}"
        );
    }

    #[test]
    fn flags_macos_runtime_branch() {
        let dir = tmp("#[cfg(target_os = \"macos\")]\nfn mac_only() {}\n", "rs");
        let violations = lint(&dir.path).expect("lint did not error");
        assert_eq!(
            violations.len(),
            1,
            "expected one violation, got {violations:?}"
        );
    }

    #[test]
    fn ignores_comments() {
        let dir = tmp(
            "// #[cfg(target_os = \"windows\")] would be flagged in source\nfn ok() {}\n",
            "rs",
        );
        let violations = lint(&dir.path).expect("lint did not error");
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn ignores_non_rust_files() {
        let dir = tmp("#[cfg(target_os = \"windows\")]\n", "txt");
        let violations = lint(&dir.path).expect("lint did not error");
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn clean_file_passes() {
        let dir = tmp(
            "//! Module doc.\n#![forbid(unsafe_code)]\nfn add(a: i32, b: i32) -> i32 { a + b }\n",
            "rs",
        );
        let violations = lint(&dir.path).expect("lint did not error");
        assert_eq!(violations.len(), 0);
    }

    // Tiny stdlib-only tempdir to avoid pulling `tempfile` into the
    // workspace dependency graph for a single test helper.
    mod tempfile_alt {
        use std::env;
        use std::path::PathBuf;
        use std::sync::atomic::{AtomicUsize, Ordering};

        static COUNTER: AtomicUsize = AtomicUsize::new(0);

        pub struct Dir {
            pub path: PathBuf,
        }

        impl Dir {
            pub fn new() -> Self {
                let n = COUNTER.fetch_add(1, Ordering::SeqCst);
                let pid = std::process::id();
                let path = env::temp_dir().join(format!("xtask-lint-{pid}-{n}"));
                std::fs::create_dir_all(&path).unwrap();
                Self { path }
            }
        }

        impl Drop for Dir {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.path);
            }
        }
    }
}
