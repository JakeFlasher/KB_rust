//! Shared parity-comparison helpers.
//!
//! This module contains comparison primitives reused across the
//! parity gates that live in different crates (the `xtask` matrix
//! row and the `cacg-cli` integration test). Lifting them here keeps
//! both gates byte-identical — a change to the DEC-2 whitelist made
//! in one place flows to both, eliminating the silent
//! contract-divergence risk where one gate goes green while the
//! other goes red.
//!
//! All functions in this module compose existing `cacg-core`
//! primitives (`canonical_json`, `schema`) plus `serde_json` —
//! they introduce no new dependency edges that
//! `xtask audit-cacg-core-deps` would flag.

use serde_json::Value;
use thiserror::Error;

use crate::canonical_json::{canonical_json, CanonicalError};

/// Errors returned by [`canonicalize_sources_minus_dec2_whitelist`].
#[derive(Debug, Error)]
pub enum ParityHelperError {
    /// The supplied bytes did not parse as JSON.
    #[error("sources_manifest.json is not valid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    /// The parsed JSON did not contain a `sources` array of objects.
    #[error("sources_manifest.json has no `sources` array of objects")]
    MissingSourcesArray,
    /// A `sources[]` entry was not a JSON object.
    #[error("sources_manifest.json `sources[{0}]` is not a JSON object")]
    NonObjectSourceEntry(usize),
    /// Re-serialization through `cacg_core::canonical_json` failed.
    #[error("canonical_json failed after stripping DEC-2 whitelist: {0:?}")]
    Canonical(CanonicalError),
}

/// Strip the two DEC-2-whitelisted parser-identity fields
/// (`parser_name`, `parser_version`) from every entry of
/// `sources_manifest.json["sources"][]` and return the canonical-JSON
/// bytes of the result.
///
/// This is the **AC-5 BYTE-EQUAL contract** for the `sources_manifest`
/// half: every field that ISN'T whitelisted must be byte-equal
/// between the Python oracle and the Rust output. The whitelist is
/// applied symmetrically (stripped from both sides) and the result
/// is re-serialized through [`canonical_json`] so the comparison is
/// strictly stronger than parsed-`serde_json::Value` field equality
/// (which would silently pass a `30` vs `30.0` numeric drift or an
/// ASCII-as-`A` re-escape).
///
/// # Callers
///
/// - `xtask/src/parity.rs` — the `MatrixRowKind::KbIngest` row body
///   (`kb_ingest_parity_cfa_vol1_trim`).
/// - `crates/cacg-cli/tests/kb_ingest_parity.rs` — the AC-5
///   integration test.
///
/// Adding a third caller is fine; the byte-equal contract is the
/// shared definition of "AC-5 sources parity," not the per-caller
/// detail.
///
/// # Errors
///
/// Returns [`ParityHelperError`] on JSON parse failure, missing /
/// wrong-shape `sources` array, a non-object entry, or canonical-JSON
/// re-serialization failure (in practice unreachable for any
/// structurally-valid manifest that originated from the schema).
pub fn canonicalize_sources_minus_dec2_whitelist(
    bytes: &[u8],
) -> Result<Vec<u8>, ParityHelperError> {
    let mut v: Value = serde_json::from_slice(bytes)?;
    let sources = v
        .get_mut("sources")
        .and_then(Value::as_array_mut)
        .ok_or(ParityHelperError::MissingSourcesArray)?;
    for (idx, entry) in sources.iter_mut().enumerate() {
        let obj = entry
            .as_object_mut()
            .ok_or(ParityHelperError::NonObjectSourceEntry(idx))?;
        obj.remove("parser_name");
        obj.remove("parser_version");
    }
    canonical_json(&v)
        .map(String::into_bytes)
        .map_err(ParityHelperError::Canonical)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn strips_both_whitelisted_fields_from_single_source_entry() {
        let bytes = br#"{"schema_version":"cacg.v0","sources":[{"extracted_at":"1970-01-01T00:00:00Z","parser_name":"pypdfium2","parser_version":"5.8.0+pdfium149.0.7825.0","page_count":30,"schema_version":"cacg.v0","source_id":"sid","source_path":"a.pdf","source_sha256":"deadbeef"}]}"#;
        let stripped = canonicalize_sources_minus_dec2_whitelist(bytes).unwrap();
        let stripped_str = std::str::from_utf8(&stripped).unwrap();
        // Whitelisted fields removed:
        assert!(!stripped_str.contains("parser_name"));
        assert!(!stripped_str.contains("parser_version"));
        assert!(!stripped_str.contains("pypdfium2"));
        // Non-whitelisted fields preserved:
        assert!(stripped_str.contains("extracted_at"));
        assert!(stripped_str.contains("source_sha256"));
        assert!(stripped_str.contains("deadbeef"));
        // Canonical-JSON shape: keys sorted, no spaces, no trailing
        // newline.
        assert!(!stripped_str.ends_with('\n'));
        let parsed: Value = serde_json::from_slice(&stripped).unwrap();
        assert_eq!(parsed["sources"][0]["source_id"], "sid");
    }

    #[test]
    fn applying_helper_symmetrically_yields_byte_equal_on_whitelist_only_diff() {
        let py = br#"{"schema_version":"cacg.v0","sources":[{"extracted_at":"X","page_count":1,"parser_name":"pypdfium2","parser_version":"5.8.0+pdfium149.0.7825.0","schema_version":"cacg.v0","source_id":"s","source_path":"a","source_sha256":"d"}]}"#;
        let rs = br#"{"schema_version":"cacg.v0","sources":[{"extracted_at":"X","page_count":1,"parser_name":"pdfium-render","parser_version":"0.9.1","schema_version":"cacg.v0","source_id":"s","source_path":"a","source_sha256":"d"}]}"#;
        let py_stripped = canonicalize_sources_minus_dec2_whitelist(py).unwrap();
        let rs_stripped = canonicalize_sources_minus_dec2_whitelist(rs).unwrap();
        assert_eq!(
            py_stripped, rs_stripped,
            "DEC-2 whitelist must collapse parser-identity differences",
        );
    }

    #[test]
    fn rejects_non_object_source_entry() {
        let bytes = br#"{"schema_version":"cacg.v0","sources":["not-an-object"]}"#;
        let err = canonicalize_sources_minus_dec2_whitelist(bytes).unwrap_err();
        assert!(matches!(err, ParityHelperError::NonObjectSourceEntry(0)));
    }

    #[test]
    fn rejects_missing_sources_array() {
        let bytes = br#"{"schema_version":"cacg.v0"}"#;
        let err = canonicalize_sources_minus_dec2_whitelist(bytes).unwrap_err();
        assert!(matches!(err, ParityHelperError::MissingSourcesArray));
    }

    #[test]
    fn rejects_malformed_json() {
        let bytes = br#"not json at all"#;
        let err = canonicalize_sources_minus_dec2_whitelist(bytes).unwrap_err();
        assert!(matches!(err, ParityHelperError::InvalidJson(_)));
    }
}
