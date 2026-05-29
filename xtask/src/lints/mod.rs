//! Static-grep lints over the workspace source tree.
//!
//! Each lint walks a target directory, scans `.rs` files, and emits
//! structured violations. The xtask binary dispatches them via clap
//! subcommands so CI can invoke them as merge-blocking checks.

pub mod determinism;
pub mod error_swallow;
pub mod platform_cfg;
pub mod rename_outside_publisher;
pub mod runner_bypass;
pub mod structural;
pub mod trust_leak;
pub mod unwrap;
pub mod workflow_integrity;
pub mod workflow_labels;

use std::path::PathBuf;

/// One violation surfaced by a lint.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Violation {
    /// Repo-relative path to the offending file.
    pub file: PathBuf,
    /// 1-indexed line number.
    pub line: usize,
    /// Short, fixed identifier for the rule that fired.
    pub rule: &'static str,
    /// Human-readable explanation of why the rule fired.
    pub message: String,
}

/// Returns true if `marker` appears in a line comment (after `//` outside of string/char literals).
///
/// Plain `line.contains("// qg-allow: ...")` matches inside string literals, allowing
/// suppression bypass. This helper only recognises markers in actual comment positions.
pub fn has_comment_marker(line: &str, marker: &str) -> bool {
    let bytes = line.as_bytes();
    let mut in_string = false;
    let mut in_char = false;
    let mut i = 0;
    while i < bytes.len() {
        let ch = bytes[i];
        if in_string {
            if ch == b'\\' {
                i += 1; // skip escaped char
            } else if ch == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        if in_char {
            if ch == b'\\' {
                i += 1; // skip escaped char
            } else if ch == b'\'' {
                in_char = false;
            }
            i += 1;
            continue;
        }
        if ch == b'"' {
            in_string = true;
        } else if ch == b'\'' {
            in_char = true;
        } else if ch == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            // Found a real comment -- check if marker is in the rest of the line
            return line[i..].contains(marker);
        }
        i += 1;
    }
    false
}
