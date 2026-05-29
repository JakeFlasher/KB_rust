//! Pdfium-render wrapper.
//!
//! Only compiled with the `ingest` Cargo feature. This is the single
//! Rust file in the workspace permitted to perform the C++ FFI call
//! into Pdfium; each per-page extraction call is run inside
//! `std::panic::catch_unwind` so a Rust-level panic from the FFI
//! surfaces as `CACG-INGEST-001` (naming the offending page) instead
//! of aborting the process.
//!
//! **Residual risk: C++-level abort.** `std::panic::catch_unwind`
//! catches Rust panics only. If Pdfium itself calls C++ `abort()`,
//! raises an uncaught C++ exception, or hits a hard SIGSEGV, the
//! process terminates with no Rust-level recovery. The realistic
//! mitigation if this surfaces in practice is `panic = "abort"` on
//! the ingest binary plus subprocess isolation for untrusted PDFs;
//! today we accept the residual risk and rely on pinning a known-good
//! Pdfium binary build.
//!
//! The Pdfium native shared library is bound at runtime via
//! `Pdfium::bind_to_system_library`; binary provisioning (a pinned
//! Pdfium build SHA reproducible across fresh checkouts and CI) is a
//! separate deliverable.
//!
//! ## Per-page normalization
//!
//! Each page's raw Pdfium text is passed through
//! `cacg_core::normalize::normalize_text` before being returned —
//! byte-equal with Python `cacg.pdf.extract_pages_normalized`'s
//! `normalize_text(raw)` call. The normalization pipeline is the
//! NFC + ligature-expand + hyphen-linebreak-rejoin + whitespace
//! collapse pipeline pinned in `cacg_core::normalize`; skipping
//! it here produces chunk-boundary divergence from the Python
//! oracle (different per-page byte lengths shift the chunker's
//! token-budget decisions). AC-5 caught this in Round 9.

use std::panic::{catch_unwind, AssertUnwindSafe};

use pdfium_render::prelude::*;

use cacg_core::normalize::normalize_text;

use crate::IngestError;

/// Extract per-page text using pdfium-render.
///
/// The outer `catch_unwind` contains any Rust panic from the bind +
/// load setup; each inner `catch_unwind` contains a Rust panic from a
/// single page's text extraction. Either way, a caught panic surfaces
/// as `IngestError::Corrupt` (`CACG-INGEST-001`) — the per-page form
/// names the offending page number so a single bad page does not
/// silently mask the rest of a document. Pdfium's own `Result` errors
/// are mapped to `IngestError::Corrupt` with their error string.
#[allow(clippy::excessive_nesting)]
pub(crate) fn extract_pages_impl(pdf_bytes: &[u8]) -> Result<Vec<String>, IngestError> {
    let owned = pdf_bytes.to_vec();
    let outcome = catch_unwind(AssertUnwindSafe(|| -> Result<Vec<String>, IngestError> {
        let bindings = Pdfium::bind_to_system_library().map_err(|err| IngestError::Corrupt {
            detail: format!("pdfium bind failed: {err}"),
        })?;
        let pdfium = Pdfium::new(bindings);

        let document =
            pdfium
                .load_pdf_from_byte_vec(owned, None)
                .map_err(|err| IngestError::Corrupt {
                    detail: format!("pdfium load failed: {err}"),
                })?;

        let pages = document.pages();
        let page_count = pages.len();
        if page_count == 0 {
            return Err(IngestError::Corrupt {
                detail: "PDF contains zero pages".into(),
            });
        }

        let cap = usize::try_from(page_count).unwrap_or(0);
        let mut out: Vec<String> = Vec::with_capacity(cap);
        for (idx, page) in pages.iter().enumerate() {
            let page_num = idx + 1;
            let page_outcome = catch_unwind(AssertUnwindSafe(|| -> Result<String, IngestError> {
                let text = page.text().map_err(|err| IngestError::Corrupt {
                    detail: format!("page {page_num}: pdfium text extraction failed: {err}"),
                })?;
                // pdfium-render's `text().all()` calls
                // `FPDFText_GetBoundedText` over the page's
                // geometric rectangle, which silently drops
                // characters whose glyph origin falls outside
                // the page rect (typically text in margin
                // areas Pdfium still indexes). Python's
                // pypdfium2 `get_textpage().get_text_range()`
                // calls `FPDFText_GetText(text_page, 0, count)`
                // — index-range, not geometry-bound, so every
                // indexed character is preserved.
                //
                // To match Python byte-for-byte we walk every
                // char via `chars().iter()` (each call hits
                // `FPDFText_GetUnicode(handle, index)`, the
                // same per-char accessor `FPDFText_GetText`
                // walks internally) and concatenate. Chars
                // for which Pdfium has no Unicode info are
                // skipped — matching the
                // `FPDFText_GetText` docs' "ignores characters
                // without unicode information" semantic.
                //
                // AC-5 caught the boundary-vs-index divergence
                // in Round 9; this is the byte-equal fix.
                let raw_chars = text.chars();
                let mut raw = String::new();
                for c in raw_chars.iter() {
                    if let Some(ch) = c.unicode_char() {
                        // Pdfium emits U+0000 for chars
                        // without Unicode information;
                        // `FPDFText_GetText` ignores them
                        // (per its docstring "ignores
                        // characters without unicode
                        // information"). Mirror that ignore
                        // in the per-char loop so the
                        // output stays byte-equal with
                        // Python (which also drops cp==0
                        // in its mirrored per-char loop).
                        // M4-Round-15 caught a NULL leak
                        // on `qm_eslii_trim.pdf`.
                        if ch == '\u{0000}' {
                            // qg-allow: deep-nesting — PDF null-byte filter
                            continue;
                        }
                        // Pdfium's per-string accessor
                        // (`FPDFText_GetText`, used by pypdfium2)
                        // post-processes the soft-hyphen marker
                        // U+0002 (STX) into the non-character
                        // U+FFFE. The per-char accessor
                        // (`FPDFText_GetUnicode`, used by
                        // `chars().iter()` above) returns the
                        // raw U+0002. Mirror the per-string
                        // transform so the extracted text is
                        // byte-equal with Python.
                        // AC-5 caught this in Round 9 against
                        // `cfa_vol1_trim.pdf` (36 occurrences,
                        // 23 of 34 chunks affected). STX is a
                        // C0 control that never appears in
                        // legitimate text, so the mapping is
                        // safe in practice.
                        let ch = if ch == '\u{0002}' { '\u{FFFE}' } else { ch }; // qg-allow: deep-nesting
                        raw.push(ch);
                    }
                }
                // Byte-equal with Python
                // `cacg.pdf.extract_pages_normalized`'s
                // `normalize_text(raw)` (cacg/pdf.py:80).
                Ok(normalize_text(&raw))
            }));
            match page_outcome {
                Ok(Ok(text)) => out.push(text),
                Ok(Err(err)) => return Err(err),
                Err(_panic) => {
                    return Err(IngestError::Corrupt {
                        detail: format!(
                            "page {page_num}: pdfium native panic crossed the FFI boundary"
                        ),
                    });
                }
            }
        }
        Ok(out)
    }));

    match outcome {
        Ok(extraction) => extraction,
        Err(_panic) => Err(IngestError::Corrupt {
            detail: "pdfium native panic crossed the FFI boundary (bind/load)".into(),
        }),
    }
}
