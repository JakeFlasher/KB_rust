//! CACG PDF ingest path.
//!
//! Wraps `pdfium-render` for per-page text extraction, hands off to the
//! cacg-core chunker + atomic publisher. Behind the `ingest` Cargo
//! feature so non-ingest builds skip the Pdfium native dependency
//! entirely.

// cacg-ingest carries the only unsafe-Rust surface in the workspace
// (the Pdfium C++ FFI binding). The forbid-unsafe-code attribute is
// therefore intentionally omitted here. All other crates carry it.

#![warn(clippy::pedantic, missing_docs)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::doc_markdown,
    clippy::doc_lazy_continuation,
    clippy::missing_panics_doc,
    clippy::similar_names,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::manual_range_contains
)]

use thiserror::Error;

pub mod chunker;
pub mod config;
pub mod manifest;
#[cfg(feature = "ingest")]
mod pdf;
pub mod utterances;

/// Error type for the PDF ingest path. Diagnostic strings here are
/// kept byte-equal with `cacg_core::diagnostic::codes::INGEST_*` and
/// with the Python `legacy_python_oracle/src/cacg/lint/codes.py` catalogue.
#[derive(Debug, Error)]
pub enum IngestError {
    /// Source PDF could not be loaded or extracted (truncated,
    /// garbage-byte, encrypted, zero-page, or a native panic crossing
    /// the FFI boundary). `CACG-INGEST-001`.
    #[error("CACG-INGEST-001: malformed PDF: {detail}")]
    Corrupt {
        /// Human-readable detail describing the load or extraction failure.
        detail: String,
    },

    /// Extraction succeeded but produced no chunks. `CACG-INGEST-002`.
    #[error("CACG-INGEST-002: PDF produced no chunks")]
    Empty,

    /// Manifest pair-atomic publish failed (e.g. OS error during atomic
    /// rename). `CACG-INGEST-003`.
    #[error("CACG-INGEST-003: manifest publish failed: {detail}")]
    PublishFailed {
        /// Human-readable detail describing the publish failure.
        detail: String,
    },
}

/// Extract per-page text from a PDF.
///
/// Returns a vector with one entry per page; element index `i` holds
/// the extracted text of page `i + 1` (CACG pages are 1-indexed).
///
/// Available only with the `ingest` Cargo feature enabled. Every
/// per-page Pdfium call is wrapped in `std::panic::catch_unwind` so a
/// native panic crossing the C++ FFI boundary surfaces as
/// `CACG-INGEST-001` rather than aborting the process. Pdfium's own
/// `Result` errors are also mapped to `IngestError::Corrupt`.
///
/// The Pdfium native shared library must be discoverable at runtime
/// (e.g. `libpdfium.so` on Linux on `LD_LIBRARY_PATH`). A pinned
/// binary-provisioning mechanism reproducible on a fresh checkout and
/// in CI is a separate deliverable.
#[cfg(feature = "ingest")]
pub fn extract_pages(pdf_bytes: &[u8]) -> Result<Vec<String>, IngestError> {
    pdf::extract_pages_impl(pdf_bytes)
}

/// Version of the cacg-ingest crate, as set by Cargo at compile time.
#[must_use]
pub fn cacg_ingest_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{cacg_ingest_version, IngestError};

    #[test]
    fn version_is_nonempty() {
        assert!(!cacg_ingest_version().is_empty());
    }

    /// Locks the doc-comment claim that `IngestError` display strings are
    /// byte-equal with the `cacg_core::diagnostic::codes::INGEST_*`
    /// catalogue. If the catalogue strings drift, this test must fail
    /// so the divergence is caught at the source of truth boundary.
    #[test]
    fn ingest_error_display_codes_match_diagnostic_catalogue() {
        use cacg_core::diagnostic::codes;

        let corrupt = IngestError::Corrupt {
            detail: "test detail".into(),
        };
        let empty = IngestError::Empty;
        let publish = IngestError::PublishFailed {
            detail: "test detail".into(),
        };

        assert!(
            format!("{corrupt}").starts_with(codes::INGEST_001),
            "Corrupt display must start with {}",
            codes::INGEST_001
        );
        assert!(
            format!("{empty}").starts_with(codes::INGEST_002),
            "Empty display must start with {}",
            codes::INGEST_002
        );
        assert!(
            format!("{publish}").starts_with(codes::INGEST_003),
            "PublishFailed display must start with {}",
            codes::INGEST_003
        );
    }
}
