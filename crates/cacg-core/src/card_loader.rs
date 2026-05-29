//! Strict card loader mirroring Python `legacy_python_oracle/src/cacg/card_loader.py`.
//!
//! The loader is the boundary the rest of the verify hot path imports.
//! It wraps `crate::frontmatter::parse_card` with file-shape preflight
//! and UTF-8 decoding, so caller-facing failures arrive as
//! [`CardLoadError`] carrying CACG-* diagnostics rather than
//! filesystem / encoding panics.
//!
//! Trust-boundary discipline: every user-supplied path goes through
//! `Path::is_file()` before `read_to_string` (per BitLesson
//! `BL-20260518-shape-check-fs-inputs`), so a directory or broken
//! symlink at a `kb lint <card>` invocation surfaces as a
//! `CACG-CLI-001` diagnostic + journal entry instead of a Python
//! traceback leak.

use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::diagnostic::{codes, Diagnostic, Severity};
use crate::frontmatter::parse_card;
use crate::normalize::normalize_text;
use crate::schema::CardFrontmatter;

/// A parsed and validated card: frontmatter plus the raw and normalized
/// body bytes. Returned by [`load_card`]. Mirrors Python `cacg.frontmatter.CardDoc`
/// plus the extra `path` + `body_normalized` fields the verify hot path
/// needs eagerly so callers don't recompute normalization per-citation.
#[derive(Debug, Clone)]
pub struct CardDoc {
    /// The on-disk path the card was loaded from.
    pub path: PathBuf,
    /// Validated frontmatter.
    pub frontmatter: CardFrontmatter,
    /// Body text exactly as it appeared after the closing `---\n`.
    pub body_raw: String,
    /// Body text with `normalize::normalize_text` applied (NFKC + ASCII
    /// punctuation map + whitespace collapse). Layer-2 verify operates
    /// on this form.
    pub body_normalized: String,
}

/// Errors raised by [`load_card`]. The `diagnostics` field carries
/// one or more [`Diagnostic`]s already mapped to stable CACG-* codes
/// so the CLI handler can stream them to stderr without re-deriving.
#[derive(Debug, Error)]
#[error("{}", join_messages(.diagnostics))]
pub struct CardLoadError {
    /// Diagnostics describing the failure (parse, validate, or I/O).
    pub diagnostics: Vec<Diagnostic>,
}

fn join_messages(items: &[Diagnostic]) -> String {
    items
        .iter()
        .map(|d| d.message.as_str())
        .collect::<Vec<_>>()
        .join("; ")
}

impl CardLoadError {
    fn single(code: &str, message: impl Into<String>, path: &Path) -> Self {
        Self {
            diagnostics: vec![Diagnostic::new(code, Severity::Error, message)
                .with_file(path.display().to_string())],
        }
    }
}

/// Read, parse, and validate a card. Returns a populated [`CardDoc`]
/// on success or [`CardLoadError`] carrying CACG-* diagnostics on any
/// preflight / parse / validation failure.
///
/// On the failure paths:
///
/// * Path is not a regular file (directory, broken symlink, missing) →
///   `CACG-CLI-001`.
/// * Read fails with I/O error or invalid UTF-8 →
///   `CACG-CLI-001`.
/// * Frontmatter parse or validation fails → the diagnostics produced
///   by [`crate::frontmatter::parse_card`], with the `file` field
///   populated to the card's path.
///
/// # Errors
///
/// Returns `Err(CardLoadError)` when any of the above failure paths
/// fires. The error carries one or more diagnostics; the caller emits
/// them and exits non-zero.
pub fn load_card(path: impl AsRef<Path>) -> Result<CardDoc, CardLoadError> {
    let path = path.as_ref();
    if !path.is_file() {
        return Err(CardLoadError::single(
            codes::CLI_001,
            format!("file not found or not a regular file: {}", path.display()),
            path,
        ));
    }
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            return Err(CardLoadError::single(
                codes::CLI_001,
                format!("cannot read card {}: {e}", path.display()),
                path,
            ));
        }
    };
    match parse_card(&text) {
        Ok((frontmatter, body_raw)) => {
            let body_normalized = normalize_text(&body_raw);
            Ok(CardDoc {
                path: path.to_path_buf(),
                frontmatter,
                body_raw,
                body_normalized,
            })
        }
        Err(mut diagnostics) => {
            let file_str = path.display().to_string();
            for d in &mut diagnostics {
                if d.file.is_none() {
                    d.file = Some(file_str.clone());
                }
            }
            Err(CardLoadError { diagnostics })
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn missing_path_emits_cli_001() {
        let nonexistent = PathBuf::from("/nonexistent/__rust_card_loader_probe__.md");
        let err = load_card(&nonexistent).expect_err("missing file must error");
        assert_eq!(err.diagnostics.len(), 1);
        assert_eq!(err.diagnostics[0].code, codes::CLI_001);
        assert_eq!(
            err.diagnostics[0].file.as_deref(),
            Some(nonexistent.display().to_string().as_str())
        );
    }

    #[test]
    fn directory_path_emits_cli_001() {
        let dir = tempfile::tempdir().expect("tempdir");
        let err = load_card(dir.path()).expect_err("directory must error with CLI-001");
        assert_eq!(err.diagnostics[0].code, codes::CLI_001);
    }

    #[test]
    fn invalid_utf8_emits_cli_001() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("card.md");
        // Invalid UTF-8 byte sequence (lone continuation byte).
        std::fs::write(&path, &[0xC3, 0x28]).expect("write");
        let err = load_card(&path).expect_err("invalid utf-8 must error");
        assert_eq!(err.diagnostics[0].code, codes::CLI_001);
    }
}
