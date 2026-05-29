//! Reject trust-critical implementation calls inside `cacg-cli`.
//!
//! The trust kernel lives in `cacg-core`. `cacg-cli` MUST stay a thin
//! clap dispatcher: parse args, construct a `DeterminismContext`,
//! delegate to `cacg_core::*` library functions. A direct call to
//! SHA-256, `serde_json::to_string`, an `OpenOptions::new()` builder,
//! a `rusqlite::Connection::open`, or a hand-rolled JSON-key sort in
//! `crates/cacg-cli/src/**` means trust logic has crept out of the
//! kernel — every future trust regression then has to be guarded in
//! two places at once.
//!
//! This lint is the static-grep regression guard for that invariant.
//! Allowlists:
//! - The lint's own source file (`xtask/src/lints/trust_leak.rs`)
//!   contains the needle patterns it scans for and unit-test fixtures.
//! - Any path containing `/tests/` — test code is allowed to use the
//!   underlying APIs for fixture setup; the regression guard targets
//!   production CLI dispatcher code only.
//! - Lines starting with `//` (after trim) are exempt so module-doc
//!   and inline comments that mention the patterns are not flagged.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::Violation;

const RULE: &str = "trust-leak";

/// File-path suffixes intentionally allowed to contain the needle
/// patterns this lint scans for.
const ALLOWED_FILE_SUFFIXES: &[&str] = &["xtask/src/lints/trust_leak.rs"];

/// Production default scan root: `crates/cacg-cli/src`. The lint
/// targets the CLI binary specifically; `cacg-core` IS the trust
/// kernel and is supposed to use these APIs.
#[must_use]
pub fn default_scan_roots() -> Vec<PathBuf> {
    vec![PathBuf::from("crates/cacg-cli/src")]
}

/// Needle patterns this lint flags. Each entry pairs the literal
/// substring with the human-readable description.
const TRIGGERS: &[(&str, &str)] = &[
    (
        "Sha256",
        "direct SHA-256 use bypasses cacg_core::hash; route through cacg_core::hash::{card_hash, source_sha256, chunk_hash, compute_event_checksum}",
    ),
    (
        "serde_json::to_string(",
        "direct JSON serialization bypasses cacg_core::canonical_json; route through cacg_core::canonical_json::canonical_json",
    ),
    (
        "serde_json::to_writer(",
        "direct JSON serialization bypasses cacg_core::canonical_json; route through cacg_core::canonical_json::canonical_json",
    ),
    (
        "OpenOptions::new(",
        "direct file-open builders bypass cacg_core::{journal, history, atomic_publish}; route through the trust-kernel append/publish primitives",
    ),
    (
        ".append(true)",
        "append-mode writes bypass cacg_core::{journal, history} append discipline; route through cacg_core::journal::append_entry or cacg_core::history::append_history_event",
    ),
    (
        "sort_keys",
        "hand-rolled canonical-JSON key sorting bypasses cacg_core::canonical_json; route through cacg_core::canonical_json::canonical_json",
    ),
    (
        ".sort_by_key(",
        "hand-rolled key sort over JSON-shaped data bypasses cacg_core::canonical_json; route through cacg_core::canonical_json::canonical_json",
    ),
    (
        "rusqlite::Connection::open(",
        "direct SQLite access belongs in cacg-search, not the CLI dispatcher",
    ),
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
    // Any path containing `/tests/` is test code; allow direct
    // trust-kernel API calls there because test fixtures never
    // reach committed artifacts.
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
    fn clean_dispatcher_file_passes() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "main.rs",
            "use cacg_core::index::build_index;\nfn main() { println!(\"thin\"); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(v.is_empty(), "got: {v:?}");
    }

    #[test]
    fn flags_sha256_direct_use() {
        let dir = TempDir::new().unwrap();
        write(dir.path(), "bad.rs", "use sha2::Sha256;\n");
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("SHA-256"));
        assert_eq!(v[0].rule, RULE);
    }

    #[test]
    fn flags_sha256_brace_import_with_digest_call() {
        // The brace-import form `use sha2::{Digest, Sha256};` plus a
        // bare `Sha256::digest(...)` call doesn't contain the literal
        // `sha2::Sha256` substring on either line, so the original
        // qualified-only trigger missed it. The broader `Sha256`
        // substring catches both the import line AND the use line.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "bad.rs",
            "use sha2::{Digest, Sha256};\nfn h(b: &[u8]) { let _ = Sha256::digest(b); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(
            v.len(),
            2,
            "expected 2 violations (one per Sha256-bearing line); got: {v:?}"
        );
        for hit in &v {
            assert!(hit.message.contains("SHA-256"));
        }
    }

    #[test]
    fn flags_sort_by_key_hand_roll() {
        // The plan explicitly names `sort_by_key` as a forbidden
        // hand-rolled canonical-JSON key sort. A Rust caller doing
        // `keys.sort_by_key(|k| k.clone())` outside cacg_core::canonical_json
        // is the regression target.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "bad.rs",
            "fn canon(mut keys: Vec<String>) {\n    keys.sort_by_key(|k| k.clone());\n}\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("canonical_json"));
        assert_eq!(v[0].rule, RULE);
    }

    #[test]
    fn flags_serde_json_to_string() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "bad.rs",
            "fn t() { let _ = serde_json::to_string(&value); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("canonical_json"));
    }

    #[test]
    fn flags_serde_json_to_writer() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "bad.rs",
            "fn t(mut f: std::fs::File) { serde_json::to_writer(&mut f, &value).unwrap(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("canonical_json"));
    }

    #[test]
    fn flags_open_options_new() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "bad.rs",
            "fn t() { let _ = std::fs::OpenOptions::new().create(true).open(\"x\"); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("trust-kernel append/publish"));
    }

    #[test]
    fn flags_append_mode_write() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "bad.rs",
            "fn t() { let mut o = std::fs::File::open(\"x\"); o.append(true); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("append"));
    }

    #[test]
    fn flags_sort_keys_hand_roll() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "bad.rs",
            "fn t() { let opts = SerializerOpts { sort_keys: true }; }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("canonical_json"));
    }

    #[test]
    fn flags_rusqlite_connection_open() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "bad.rs",
            "fn t() { let conn = rusqlite::Connection::open(\"db.sqlite\").unwrap(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert_eq!(v.len(), 1, "got: {v:?}");
        assert!(v[0].message.contains("cacg-search"));
    }

    #[test]
    fn ignores_comments() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "doc.rs",
            "// uses sha2::Sha256 internally\n/// Calls serde_json::to_string for debugging\nfn ok() {}\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(v.is_empty(), "comments must be exempt; got: {v:?}");
    }

    #[test]
    fn allows_in_tests_directory() {
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "crates/cacg-cli/tests/some_test.rs",
            "fn t() { let _ = sha2::Sha256::new(); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(v.is_empty(), "tests/ paths must be allowlisted; got: {v:?}");
    }

    #[test]
    fn allows_in_trust_leak_own_source() {
        // Synthesize the allowlisted-suffix layout.
        let dir = TempDir::new().unwrap();
        write(
            dir.path(),
            "xtask/src/lints/trust_leak.rs",
            "fn t() { let _ = sha2::Sha256::new(); let _ = serde_json::to_string(&v); }\n",
        );
        let v = lint(&[dir.path().to_path_buf()]).unwrap();
        assert!(
            v.is_empty(),
            "the lint's own source must be allowlisted; got: {v:?}"
        );
    }

    #[test]
    fn flags_trust_leak_in_cacg_cli_under_default_roots() {
        // Synthesize a non-existent crates/cacg-cli/src tree under a
        // tempdir + resolve default_scan_roots() against the tempdir
        // prefix. Confirms the production default-root construction
        // catches forbidden patterns in the CLI dispatcher. The broad
        // `Sha256` trigger flags both the `use` import line and the
        // `Sha256::new()` use line, so 2 violations are expected.
        let workspace = TempDir::new().unwrap();
        write(
            workspace.path(),
            "crates/cacg-cli/src/main.rs",
            "use sha2::Sha256;\nfn main() { let _ = Sha256::new(); }\n",
        );
        let roots: Vec<PathBuf> = default_scan_roots()
            .iter()
            .map(|r| workspace.path().join(r))
            .collect();
        let v = lint(&roots).unwrap();
        assert_eq!(v.len(), 2, "got: {v:?}");
        for hit in &v {
            assert!(hit.file.to_string_lossy().contains("cacg-cli"));
            assert_eq!(hit.rule, RULE);
        }
    }
}
