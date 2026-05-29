//! Card-loading, manifest-resolution, and precondition checks
//! extracted from `verify_one_card`.
//!
//! Every function here is `pub(super)` — called only from the
//! orchestrator in `mod.rs`.

use std::path::Path;
use std::time::Instant;

use crate::card_loader::{load_card, CardDoc, CardLoadError};
use crate::chunks_index::ChunksIndex;
use crate::diagnostic::{codes, Diagnostic, Severity};
use crate::journal::JournalError;
use crate::retraction::RetractionSpec;

use super::journal::{append_verify_event, retraction_diagnostic};
use super::VerifyOneCardResult;

/// Check whether the card path points at an existing regular file.
/// Returns `Some(result)` with a `CACG-CLI-001` diagnostic when the
/// path is missing, a directory, or a dangling symlink; returns
/// `None` when the path is valid and the caller should proceed.
pub(super) fn missing_card_preflight(
    card_path: &Path,
    journal_path: &Path,
    start: Instant,
) -> Result<Option<VerifyOneCardResult>, JournalError> {
    if card_path.is_file() {
        return Ok(None);
    }
    let diag = Diagnostic::new(
        codes::CLI_001,
        Severity::Error,
        format!("file not found: {}", card_path.display()),
    )
    .with_file(card_path.display().to_string());
    let diagnostics = vec![diag];
    append_verify_event(
        journal_path,
        card_path,
        None,
        &diagnostics,
        false,
        false,
        false,
        start,
    )?;
    Ok(Some(VerifyOneCardResult {
        verified: false,
        layer1: false,
        layer2: false,
        fuzzy: false,
        diagnostics,
        card_hash: None,
    }))
}

/// Attempt to load the card document when Layer-1 did not supply one
/// (either `skip_lint=true` or Layer-1's `load_card` failed earlier).
///
/// Returns `Ok(Some(doc))` on success, `Ok(None)` when the load
/// failed (diagnostics + journal event already emitted), or `Err` on
/// journal I/O failure.
pub(super) fn resolve_card_doc(
    card_path: &Path,
    journal_path: &Path,
    diagnostics: &mut Vec<Diagnostic>,
    start: Instant,
) -> Result<Option<CardDoc>, JournalError> {
    match load_card(card_path) {
        Ok(d) => Ok(Some(d)),
        Err(CardLoadError { diagnostics: dx }) => {
            diagnostics.extend(dx);
            append_verify_event(
                journal_path,
                card_path,
                None,
                diagnostics,
                false,
                false,
                false,
                start,
            )?;
            Ok(None)
        }
    }
}

/// Outcome of [`resolve_chunks_index`]: either a borrowed reference
/// to the caller-supplied index, or an owned per-card index loaded
/// from disk.
pub(super) enum ResolvedIndex<'a> {
    Borrowed(&'a ChunksIndex),
    Owned(ChunksIndex),
}

impl<'a> ResolvedIndex<'a> {
    pub(super) fn as_ref(&self) -> &ChunksIndex {
        match self {
            ResolvedIndex::Borrowed(idx) => idx,
            ResolvedIndex::Owned(idx) => idx,
        }
    }
}

/// Resolve the chunks index for Layer-2. When `chunks_index` is
/// `Some`, borrow it directly. Otherwise load from disk; on failure
/// emit `CACG-MAN-001` (plus any retraction diagnostic) and append
/// a journal event.
///
/// Returns `Ok(Some(resolved))` when an index is available, or
/// `Ok(None)` when the manifest load failed (journal event already
/// emitted).
pub(super) fn resolve_chunks_index<'a>(
    chunks_index: Option<&'a ChunksIndex>,
    chunks_manifest_path: &Path,
    card_path: &Path,
    journal_path: &Path,
    doc: Option<&CardDoc>,
    card_hash_value: Option<&str>,
    retraction: Option<&RetractionSpec>,
    diagnostics: &mut Vec<Diagnostic>,
    start: Instant,
) -> Result<Option<ResolvedIndex<'a>>, JournalError> {
    match chunks_index {
        Some(idx) => Ok(Some(ResolvedIndex::Borrowed(idx))),
        None => match ChunksIndex::from_path(chunks_manifest_path) {
            Ok(idx) => Ok(Some(ResolvedIndex::Owned(idx))),
            Err(e) => {
                diagnostics.push(
                    Diagnostic::new(codes::MAN_001, Severity::Error, e.to_string())
                        .with_file(card_path.display().to_string()),
                );
                // A retracted card whose chunks manifest is
                // also malformed must still surface
                // CACG-RETR-001 — the two diagnostics are
                // independent.
                if let Some(retr_d) = retraction_diagnostic(doc, card_path, retraction) {
                    diagnostics.push(retr_d);
                }
                append_verify_event(
                    journal_path,
                    card_path,
                    card_hash_value,
                    diagnostics,
                    false,
                    false,
                    false,
                    start,
                )?;
                Ok(None)
            }
        },
    }
}
