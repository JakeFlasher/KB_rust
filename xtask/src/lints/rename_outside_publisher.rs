//! Reject raw `std::fs::rename` (or `fs::rename`) call sites outside
//! the centralized atomic publisher.
//!
//! Rationale: the trust kernel's multi-file atomic publish discipline
//! lives in `cacg_core::atomic_publish`. Any other module that calls
//! `std::fs::rename` directly bypasses the publisher's preflight
//! refusal of pre-existing sidecars, the `.bak` snapshot + rollback,
//! and the success-cleanup contract. A bare rename can leave a
//! half-committed pair on disk if a crash happens between the rename
//! and any follow-up bookkeeping, breaking the byte-equal parity gate
//! against Python's `os.replace`-based publisher.
//!
//! This lint is the static-grep regression guard for that invariant:
//! `cacg_core::atomic_publish::DefaultFs::rename` is the only allowed
//! call site for the raw syscall; everywhere else must call
//! `cacg_core::atomic_publish::atomic_publish` (or pass a
//! `cacg_core::atomic_publish::FsSyscalls` impl that delegates to
//! `DefaultFs`).
//!
//! Comments and module-doc references to `fs::rename` are exempted
//! by the `//` prefix check (mirrors `lints::platform_cfg`).
//!
//! Scope: every `.rs` file under the supplied root directories is
//! scanned recursively. [`default_scan_roots`] returns the production
//! default (`crates` + `xtask/src`) so a regression introduced in any
//! current or future workspace crate is caught by one invocation.
//! `crates/cacg-core/src/atomic_publish.rs` is allowlisted because
//! it IS the publisher and must perform the syscall;
//! `crates/cacg-search/src/sqlite.rs` is allowlisted because the
//! `summaries.sqlite` FTS5 sidecar is a non-trust performance artifact
//! built in place by `rusqlite` — there is no caller-supplied byte
//! buffer for the byte-based `atomic_publish` API to write — and it is
//! published by a single-file atomic rename mirroring Python
//! `search_sqlite.py`'s `os.replace`.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::Violation;

const RULE: &str = "rename-outside-publisher";

/// File-path suffixes (relative to repo root) where rename pattern
/// literals are intentionally present and therefore must not trigger
/// the lint. `crates/cacg-core/src/atomic_publish.rs` IS the
/// publisher and must perform the syscall; `xtask/src/lints/rename_outside_publisher.rs`
/// is THIS lint's source file, which contains the needle strings the
/// lint scans for plus unit-test fixtures that exercise the matcher
/// against synthetic source bytes. `crates/cacg-search/src/sqlite.rs`
/// publishes the `summaries.sqlite` FTS5 sidecar — a NON-TRUST
/// performance artifact, explicitly excluded from the byte-determinism
/// contract: `rusqlite` builds the database in place, so there is no
/// caller-supplied byte buffer for the byte-based `atomic_publish` API
/// to write, and the single-file `.tmp`→canonical rename is atomic on
/// POSIX and mirrors Python `search_sqlite.py`'s `os.replace`.
const ALLOWED_FILE_SUFFIXES: &[&str] = &[
    "crates/cacg-core/src/atomic_publish.rs",
    "crates/cacg-search/src/sqlite.rs",
    "xtask/src/lints/rename_outside_publisher.rs",
];

/// Production default scan roots: every workspace crate's source +
/// test trees (via the `crates` parent dir) plus `xtask/src`. The CLI
/// command uses this when `--root` is not supplied; the regression
/// test for non-core crate coverage calls the same helper with a
/// tempdir-relative prefix so the test exercises the EXACT production
/// default-root construction.
#[must_use]
pub fn default_scan_roots() -> Vec<PathBuf> {
    vec![PathBuf::from("crates"), PathBuf::from("xtask/src")]
}

/// Walk every `.rs` file under `roots` and return every violation found.
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
                if file_type.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    scan_file(&path, &mut violations)?;
                }
            }
        }
    }
    Ok(violations)
}

fn is_allowlisted(path: &Path) -> bool {
    let s = path.to_string_lossy();
    ALLOWED_FILE_SUFFIXES
        .iter()
        .any(|suffix| s.ends_with(suffix))
}

fn scan_file(path: &Path, violations: &mut Vec<Violation>) -> io::Result<()> {
    if is_allowlisted(path) {
        return Ok(());
    }
    let contents = fs::read_to_string(path)?;
    for (idx, line) in contents.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") {
            continue;
        }
        // `std::fs::rename(` is the obvious form. The unqualified
        // `fs::rename(` form is also flagged because the lint cannot
        // tell from a single line whether `fs` aliases `std::fs` or
        // (e.g.) `tokio::fs`; both bypass the publisher.
        let triggers = ["std::fs::rename(", "fs::rename("];
        for needle in triggers {
            if line.contains(needle) {
                violations.push(Violation {
                    file: path.to_path_buf(),
                    line: idx + 1,
                    rule: RULE,
                    message: format!(
                        "raw rename bypasses cacg_core::atomic_publish; route file moves through the centralized publisher: {}",
                        line.trim()
                    ),
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
    use tempfile::TempDir;

    fn write(dir: &Path, name: &str, body: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, body).unwrap();
        p
    }

    #[test]
    fn clean_file_passes() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "ok.rs",
            "fn main() {\n    println!(\"hello\");\n}\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(v.is_empty(), "expected 0 violations, got: {v:?}");
    }

    #[test]
    fn flags_raw_rename_call() {
        let dir = TempDir::new().unwrap();
        let p = write(
            dir.path(),
            "bad.rs",
            "fn main() {\n    std::fs::rename(\"a\", \"b\").unwrap();\n}\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "expected 1 violation, got: {v:?}");
        assert_eq!(v[0].file, p);
        assert_eq!(v[0].line, 2);
        assert_eq!(v[0].rule, RULE);
        assert!(v[0].message.contains("rename"));
    }

    #[test]
    fn allows_rename_inside_atomic_publish_module() {
        // Synthesize a directory layout that mirrors the allowlisted
        // path so a synthetic atomic_publish.rs is NOT flagged.
        let dir = TempDir::new().unwrap();
        let nested = dir.path().join("crates/cacg-core/src");
        fs::create_dir_all(&nested).unwrap();
        write(
            &nested,
            "atomic_publish.rs",
            "fn r() { std::fs::rename(\"a\", \"b\").unwrap(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(
            v.is_empty(),
            "atomic_publish.rs must be allowlisted; got: {v:?}"
        );
    }

    #[test]
    fn ignores_comments() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "doc.rs",
            "// std::fs::rename was here\n/// Calls std::fs::rename internally\nfn ok() {}\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(v.is_empty(), "comments must be exempt; got: {v:?}");
    }

    #[test]
    fn flags_raw_rename_in_non_core_crate_under_default_roots() {
        // Synthesize a workspace-like layout containing the same crate
        // tree the real workspace has, then resolve the default scan
        // roots against the tempdir prefix and confirm a `std::fs::rename`
        // in a non-core crate IS flagged.
        //
        // This pins the invariant that ANY workspace crate (current
        // `cacg-cli` / `cacg-ingest` / `cacg-search` / `cacg-semantic`
        // or any future addition) is covered by the default invocation,
        // not just `cacg-core`.
        let workspace = TempDir::new().unwrap();
        let cli_src = workspace.path().join("crates/cacg-cli/src");
        fs::create_dir_all(&cli_src).unwrap();
        let cli_main = cli_src.join("main.rs");
        fs::write(
            &cli_main,
            "fn main() { std::fs::rename(\"a\", \"b\").unwrap(); }\n",
        )
        .unwrap();
        // Also create the allowlisted publisher file so the test
        // confirms the allowlist still works under the broader scope.
        let publisher_dir = workspace.path().join("crates/cacg-core/src");
        fs::create_dir_all(&publisher_dir).unwrap();
        fs::write(
            publisher_dir.join("atomic_publish.rs"),
            "fn pub_rename() { std::fs::rename(\"x\", \"y\").unwrap(); }\n",
        )
        .unwrap();

        // Resolve the production default roots against the tempdir
        // prefix so the same construction the CLI uses is exercised.
        let roots: Vec<PathBuf> = default_scan_roots()
            .iter()
            .map(|r| workspace.path().join(r))
            .collect();
        let v = lint(&roots).unwrap();
        assert_eq!(
            v.len(),
            1,
            "expected exactly 1 violation in the non-core crate; got: {v:?}"
        );
        assert_eq!(v[0].file, cli_main);
        assert_eq!(v[0].rule, RULE);
    }
}
