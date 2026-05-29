//! Reject direct calls to nondeterministic APIs outside the
//! centralized [`cacg_core::determinism::DeterminismContext`].
//!
//! The trust kernel's byte-equal contract under `KB_FROZEN_CLOCK=1`
//! is only honored if every timestamp / event-id / temp-name origin
//! routes through the context. A bare call to
//! `OffsetDateTime::now_utc()`, `Uuid::new_v4()`, `Instant::now()`,
//! `SystemTime::now()`, `chrono::Utc::now()`, `chrono::Local::now()`,
//! or `tempfile::tempfile()` bypasses the context, breaks two-run
//! byte-equality under the frozen flag, and leaks
//! nondeterministic suffixes into committed artifacts.
//!
//! This lint is the static-grep regression guard for that invariant.
//! `cacg_core::determinism` IS the centralized source-of-truth and
//! must invoke the real APIs; everywhere else routes through the
//! context.
//!
//! Scope: every `.rs` file under the supplied root directories is
//! scanned recursively. [`default_scan_roots`] returns the production
//! default (`crates` + `xtask/src`) so a regression in any current
//! or future workspace crate is caught by one invocation.
//!
//! Allowlists:
//! - `crates/cacg-core/src/determinism.rs` — the determinism module
//!   itself owns the underlying API calls.
//! - `xtask/src/lints/determinism.rs` — this lint's own source file
//!   contains the needle patterns it scans for.
//! - Any path containing `/tests/` — integration / unit test files
//!   are allowed to use nondeterministic APIs for fixture setup
//!   (random TempDirs, etc.). The lint enforces production-code
//!   discipline only; test code's randomness never reaches committed
//!   artifacts because tests run against tempdirs.
//! - Lines starting with `//` (after trim) are exempt so module-doc
//!   and inline comments that reference the patterns are not flagged.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::Violation;

const RULE: &str = "determinism-bypass";

/// File-path suffixes (relative to repo root) where the nondeterministic
/// API patterns are intentionally present and therefore must not
/// trigger the lint.
const ALLOWED_FILE_SUFFIXES: &[&str] = &[
    "crates/cacg-core/src/determinism.rs",
    "xtask/src/lints/determinism.rs",
    // The parity harness legitimately uses `Instant::now()` to measure
    // wall-clock duration of the Python and Rust subprocess invocations
    // for the perf report. The measured duration is informational
    // (perf-reports JSON), not consumed by any byte-equal artifact, so
    // it cannot break the determinism invariant.
    "xtask/src/parity.rs",
    // `cacg-core::lint::layer1::freeze_aware_latency` measures wall-clock
    // elapsed time via `Instant::now()` to populate the lint journal's
    // `latency_ms` field. Under `KB_FROZEN_CLOCK=1` the helper returns
    // exactly `0.0`, so the byte-equal parity contract is preserved at
    // the call site (mirrors Python `time.perf_counter()` zeroing under
    // the same env per `legacy_python_oracle/src/cacg/lint/layer1.py:280-282`). The `Instant`
    // value never lands in a committed artifact in frozen mode.
    "crates/cacg-core/src/lint/layer1.rs",
    // `cacg-core::verify::runner::verify_one_card` measures wall-
    // clock elapsed time via `Instant::now()` and routes the
    // value through `freeze_aware_latency` before populating the
    // verify journal's `latency_ms` field — same shape as the
    // lint runner above. Under `KB_FROZEN_CLOCK=1` the helper
    // returns exactly `0.0`, so the byte-equal parity contract is
    // preserved at the call site (mirrors Python
    // `legacy_python_oracle/src/cacg/verify/runner.py::verify_one_card`'s
    // `time.perf_counter()` usage under the same env).
    // After module decomposition (Round 0), runner.rs became runner/mod.rs.
    "crates/cacg-core/src/verify/runner/mod.rs",
    // The gate harness legitimately uses `Instant::now()` to measure
    // wall-clock duration of each quality check for the gate report.
    "xtask/src/gate.rs",
];

/// Production default scan roots: every workspace crate (via the
/// `crates` parent dir) plus `xtask/src`. The CLI dispatcher uses
/// this when `--root` is empty; the non-core crate regression test
/// uses the same helper so the CLI and test cannot desynchronize.
#[must_use]
pub fn default_scan_roots() -> Vec<PathBuf> {
    vec![PathBuf::from("crates"), PathBuf::from("xtask/src")]
}

/// Needle patterns this lint flags. Each entry pairs the literal
/// substring to search for with the human-readable description of
/// why it is a violation.
///
/// Coverage rationale for the tempfile family:
/// - `tempfile::tempfile(` creates a random-suffix anonymous tempfile.
/// - `NamedTempFile::new(` / `NamedTempFile::new_in(` create a
///   random-suffix NAMED file in the process or in a chosen dir.
/// - `.tempfile(` / `.tempfile_in(` catch the `Builder` chain forms
///   (`tempfile::Builder::new().tempfile()`, etc.).
///
/// `TempDir::new(` and `tempfile::tempdir(` are intentionally NOT
/// triggers because `TempDir` only randomizes the DIRECTORY name;
/// any files created inside it have caller-controlled deterministic
/// names. Tests use `TempDir` extensively for fixture isolation; the
/// random suffix never reaches a committed artifact because the
/// tempdir is dropped at the end of the test.
const TRIGGERS: &[(&str, &str)] = &[
    ("OffsetDateTime::now_utc(", "use DeterminismContext::now_iso instead of OffsetDateTime::now_utc"),
    ("Uuid::new_v4(", "use DeterminismContext::new_uuid instead of Uuid::new_v4"),
    ("Instant::now(", "Instant::now bypasses DeterminismContext; route per-call latency through the context"),
    ("SystemTime::now(", "SystemTime::now bypasses DeterminismContext; route timestamps through the context"),
    ("chrono::Utc::now(", "chrono is not the project's clock provider; use DeterminismContext::now_iso"),
    ("chrono::Local::now(", "chrono is not the project's clock provider; use DeterminismContext::now_iso"),
    ("tempfile::tempfile(", "use DeterminismContext::next_temp_name + manual open instead of tempfile::tempfile (random suffix)"),
    ("NamedTempFile::new(", "use DeterminismContext::next_temp_name + manual open instead of NamedTempFile::new (random suffix)"),
    ("NamedTempFile::new_in(", "use DeterminismContext::next_temp_name + manual open instead of NamedTempFile::new_in (random suffix)"),
    (".tempfile(", "use DeterminismContext::next_temp_name + manual open instead of Builder::tempfile (random suffix)"),
    (".tempfile_in(", "use DeterminismContext::next_temp_name + manual open instead of Builder::tempfile_in (random suffix)"),
];

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
    if ALLOWED_FILE_SUFFIXES
        .iter()
        .any(|suffix| s.ends_with(suffix))
    {
        return true;
    }
    // Any path containing `/tests/` is test code; allow nondeterministic
    // APIs there because test fixtures never reach committed artifacts.
    s.contains("/tests/")
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
        for (needle, message) in TRIGGERS {
            if line.contains(needle) {
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
    use tempfile::TempDir;

    fn write(dir: &Path, name: &str, body: &str) -> PathBuf {
        let p = dir.join(name);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, body).unwrap();
        p
    }

    #[test]
    fn clean_file_passes() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "ok.rs",
            "fn main() { println!(\"deterministic\"); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(v.is_empty(), "expected 0 violations, got: {v:?}");
    }

    #[test]
    fn flags_offset_datetime_now_utc() {
        let dir = TempDir::new().unwrap();
        let p = write(
            dir.path(),
            "bad.rs",
            "fn t() { let _ = OffsetDateTime::now_utc(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert_eq!(v[0].file, p);
        assert!(v[0].message.contains("OffsetDateTime"));
    }

    #[test]
    fn flags_uuid_new_v4() {
        let dir = TempDir::new().unwrap();
        write(dir.path(), "bad.rs", "fn t() { let _ = Uuid::new_v4(); }\n");
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("Uuid"));
    }

    #[test]
    fn flags_chrono_utc_now() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "bad.rs",
            "fn t() { let _ = chrono::Utc::now(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("chrono"));
    }

    #[test]
    fn flags_instant_now_and_system_time_now() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "lat.rs",
            "fn a() { let _ = Instant::now(); }\nfn b() { let _ = SystemTime::now(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 2, "got: {v:?}");
    }

    #[test]
    fn flags_tempfile_tempfile() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "tmp.rs",
            "fn t() { let _ = tempfile::tempfile().unwrap(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("tempfile"));
    }

    #[test]
    fn flags_tempfile_named_new() {
        // Both fully-qualified and imported forms must be rejected.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "qualified.rs",
            "fn t() { let _ = tempfile::NamedTempFile::new().unwrap(); }\n",
        );
        write(
            dir.path(),
            "imported.rs",
            "use tempfile::NamedTempFile;\nfn t() { let _ = NamedTempFile::new().unwrap(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 2, "expected 2 violations, got: {v:?}");
        for hit in &v {
            assert!(hit.message.contains("NamedTempFile"));
        }
    }

    #[test]
    fn flags_tempfile_named_new_in() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "in_dir.rs",
            "fn t() { let _ = tempfile::NamedTempFile::new_in(\"/tmp\").unwrap(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("NamedTempFile"));
    }

    #[test]
    fn flags_builder_tempfile_chain() {
        // The Builder chain form must trigger via the `.tempfile(`
        // substring match.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "builder.rs",
            "fn t() { let _ = tempfile::Builder::new().prefix(\"foo\").tempfile().unwrap(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("Builder"));
    }

    #[test]
    fn flags_builder_tempfile_in_chain() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "builder_in.rs",
            "fn t() { let _ = tempfile::Builder::new().tempfile_in(\"/tmp\").unwrap(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("Builder"));
    }

    #[test]
    fn allows_in_determinism_module() {
        // Synthesize the allowlisted-suffix layout.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "crates/cacg-core/src/determinism.rs",
            "fn t() { let _ = OffsetDateTime::now_utc(); let _ = Uuid::new_v4(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(
            v.is_empty(),
            "determinism module must be allowlisted; got: {v:?}"
        );
    }

    #[test]
    fn allows_in_tests_directories() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "crates/cacg-core/tests/some_test.rs",
            "fn t() { let _ = OffsetDateTime::now_utc(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(v.is_empty(), "tests/ paths must be allowlisted; got: {v:?}");
    }

    #[test]
    fn ignores_comments() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "doc.rs",
            "// uses OffsetDateTime::now_utc internally\n/// See Uuid::new_v4 docs\nfn ok() {}\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(v.is_empty(), "comments must be exempt; got: {v:?}");
    }

    #[test]
    fn flags_nondeterministic_call_in_non_core_crate_under_default_roots() {
        // Synthesize a workspace-like tempdir layout with a non-core
        // crate source file containing nondeterministic API calls,
        // including the named-temp triggers. Resolve the production
        // default roots against the tempdir prefix and confirm the
        // synthetic file IS flagged for every distinct violation.
        let workspace = TempDir::new().unwrap();
        write(
            workspace.path(),
            "crates/cacg-cli/src/main.rs",
            concat!(
                "use tempfile::NamedTempFile;\n",
                "fn main() {\n",
                "    let _ = OffsetDateTime::now_utc();\n",
                "    let _ = NamedTempFile::new().unwrap();\n",
                "    let _ = tempfile::Builder::new().tempfile().unwrap();\n",
                "}\n",
            ),
        );
        // Also create the allowlisted determinism module to confirm
        // the allowlist still works under the broader scope.
        write(
            workspace.path(),
            "crates/cacg-core/src/determinism.rs",
            "fn d() { let _ = OffsetDateTime::now_utc(); let _ = Uuid::new_v4(); }\n",
        );
        let roots: Vec<PathBuf> = default_scan_roots()
            .iter()
            .map(|r| workspace.path().join(r))
            .collect();
        let v = lint(&roots).unwrap();
        assert_eq!(
            v.len(),
            3,
            "expected 3 violations (OffsetDateTime + NamedTempFile + Builder.tempfile); got: {v:?}"
        );
        for hit in &v {
            assert!(
                hit.file.to_string_lossy().contains("cacg-cli"),
                "every violation must be in the non-core crate; got {hit:?}"
            );
            assert_eq!(hit.rule, RULE);
        }
    }
}
