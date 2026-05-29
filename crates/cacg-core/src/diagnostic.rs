//! Diagnostic struct + CACG-* code constants.
//!
//! Mirrors Python `legacy_python_oracle/src/cacg/diagnostic.py`. The HYBRID parity policy
//! (`docs/diagnostic-parity.md`) requires `code`, `severity`, and `file`
//! to be byte-identical across Python and Rust; `message` strings are
//! covered by snapshot tests with a declared whitelist of intentional
//! divergences (currently empty).

use serde::{Deserialize, Serialize};

/// Severity classifier for a diagnostic, matching Python's `Literal[...]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Hard failure -- the operation cannot succeed.
    Error,
    /// Non-blocking advisory.
    Warning,
    /// Informational message.
    Info,
}

/// A structured diagnostic carrying a stable CACG-* code, severity,
/// message, optional file path, optional line number, and free-form hints.
///
/// Serde-derives produce the same JSON shape Python's `Diagnostic.to_dict`
/// emits: `{"code", "severity", "message"}` always present; `file`,
/// `line`, `hints` only when non-default.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// The CACG-* code identifying this diagnostic class.
    pub code: String,
    /// Severity level.
    pub severity: Severity,
    /// Human-readable diagnostic message.
    pub message: String,
    /// Optional file path the diagnostic refers to.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub file: Option<String>,
    /// Optional 1-based line number within `file`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub line: Option<u32>,
    /// Free-form hints (BM25 chunk_id+score, semantic verdict, etc.).
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub hints: Vec<serde_json::Value>,
}

impl Diagnostic {
    /// Construct a Diagnostic with just code + severity + message.
    /// File / line / hints are None / empty.
    #[must_use]
    pub fn new(code: impl Into<String>, severity: Severity, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            severity,
            message: message.into(),
            file: None,
            line: None,
            hints: Vec::new(),
        }
    }

    /// Builder: attach a file path.
    #[must_use]
    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }
}

/// CACG-* code constants module. String values mirror Python
/// `legacy_python_oracle/src/cacg/lint/codes.py` byte-for-byte (asserted by the snapshot
/// test in `tests/diagnostic_codes_parity.rs`). The Rust names follow
/// the numeric `<CATEGORY>_NNN` convention; trailing comments cross-
/// reference Python's descriptive constant name where relevant.
#[allow(missing_docs)]
pub mod codes {
    // Frontmatter / schema (CACG-FM-*).
    pub const FM_001: &str = "CACG-FM-001"; // FM_MISSING_REQUIRED
    pub const FM_002: &str = "CACG-FM-002"; // FM_UNKNOWN_SCHEMA
    pub const FM_003: &str = "CACG-FM-003"; // FM_EXTRA_FIELD
    pub const FM_004: &str = "CACG-FM-004"; // FM_MISSING_DELIMITER
    pub const FM_005: &str = "CACG-FM-005"; // FM_FORBIDDEN_YAML
    pub const FM_006: &str = "CACG-FM-006"; // FM_PARSE_ERROR (also: duplicate YAML keys)
    pub const FM_007: &str = "CACG-FM-007"; // FM_NOT_MAPPING
    pub const FM_008: &str = "CACG-FM-008"; // FM_OTHER_VALIDATION

    // Citation structure (CACG-CITE-*).
    pub const CITE_001: &str = "CACG-CITE-001"; // CITE_MALFORMED_CHUNK_ID
    pub const CITE_002: &str = "CACG-CITE-002"; // CITE_MALFORMED_HASH
    pub const CITE_003: &str = "CACG-CITE-003"; // CITE_REVERSED_PAGE_RANGE
    pub const CITE_004: &str = "CACG-CITE-004"; // CITE_CHUNK_NOT_IN_MANIFEST
    pub const CITE_005: &str = "CACG-CITE-005"; // CITE_PAGE_DISJOINT
    pub const CITE_006: &str = "CACG-CITE-006"; // CITE_SOURCE_NOT_IN_MANIFEST

    // Card-hash (CACG-HASH-*).
    pub const HASH_001: &str = "CACG-HASH-001"; // HASH_CHUNK_MISMATCH
    pub const HASH_002: &str = "CACG-HASH-002"; // HASH_CARD_STALE
    pub const HASH_003: &str = "CACG-HASH-003"; // HASH_MANIFEST_TAMPER

    // Index publish atomicity (CACG-IDX-*). IDX_001..003 are reserved by
    // Python's catalogue but not assigned in `lint/codes.py`; only the
    // operationally-used codes 004..008 land here.
    pub const IDX_004: &str = "CACG-IDX-004"; // IDX_PUBLISH_FAILED
    pub const IDX_005: &str = "CACG-IDX-005"; // IDX_OUT_INSIDE_CARDS
    pub const IDX_006: &str = "CACG-IDX-006"; // IDX_SIDECAR_COLLISION
    pub const IDX_007: &str = "CACG-IDX-007"; // IDX_INDEX_SIDECAR_COLLISION
    pub const IDX_008: &str = "CACG-IDX-008"; // IDX_CANONICAL_NOT_FILE

    // CLI-surface entry (CACG-CLI-*).
    pub const CLI_001: &str = "CACG-CLI-001"; // CLI_FILE_NOT_FOUND
    pub const CLI_002: &str = "CACG-CLI-002"; // CLI_NOT_IMPLEMENTED
    pub const CLI_003: &str = "CACG-CLI-003"; // CLI_BAD_ARG
    pub const CLI_004: &str = "CACG-CLI-004"; // CLI_CONFIG_ERROR
    pub const CLI_005: &str = "CACG-CLI-005"; // CLI_EXISTS

    // PDF ingestion path (CACG-INGEST-*).
    pub const INGEST_001: &str = "CACG-INGEST-001"; // INGEST_CORRUPT
    pub const INGEST_002: &str = "CACG-INGEST-002"; // INGEST_EMPTY
    pub const INGEST_003: &str = "CACG-INGEST-003"; // INGEST_PUBLISH_FAILED

    // Manifest parse / validation failures (CACG-MAN-*).
    pub const MAN_001: &str = "CACG-MAN-001"; // MAN_MALFORMED
    pub const MAN_002: &str = "CACG-MAN-002"; // MAN_SIDECAR_COLLISION
    pub const MAN_003: &str = "CACG-MAN-003"; // MAN_CANONICAL_NOT_FILE

    // Lint journal integrity (CACG-JNL-*).
    pub const JNL_001: &str = "CACG-JNL-001"; // JNL_TAMPERED

    // Source-authorization matrix (CACG-AUTH-*).
    pub const AUTH_000: &str = "CACG-AUTH-000"; // AUTH_MATRIX_INVALID
    pub const AUTH_001: &str = "CACG-AUTH-001"; // AUTH_READING_UNKNOWN
    pub const AUTH_002: &str = "CACG-AUTH-002"; // AUTH_SOURCE_UNAUTHORIZED

    // Retraction (CACG-RETR-*).
    pub const RETR_001: &str = "CACG-RETR-001"; // RETR_CARD_RETRACTED
    pub const RETR_002: &str = "CACG-RETR-002"; // RETR_SOURCE_RETRACTED
    pub const RETR_003: &str = "CACG-RETR-003"; // RETR_CHUNK_RETRACTED

    // Summary / tags surface (CACG-SUM-*).
    pub const SUM_001: &str = "CACG-SUM-001"; // SUM_MISSING (also: oversized lower-bound)
    pub const SUM_002: &str = "CACG-SUM-002"; // SUM_OVERSIZED
    pub const SUM_003: &str = "CACG-SUM-003"; // SUM_TAG_NON_CONFORMING
    pub const SUM_004: &str = "CACG-SUM-004"; // SUM_TAGS_OVERSIZED

    // SKILL.md router (CACG-SKILL-*).
    pub const SKILL_001: &str = "CACG-SKILL-001"; // SKILL_NAME_COLLISION
    pub const SKILL_002: &str = "CACG-SKILL-002"; // SKILL_ROUTES_TO_UNKNOWN
    pub const SKILL_003: &str = "CACG-SKILL-003"; // SKILL_SCHEMA_VIOLATION

    // Dependency DAG (CACG-DEP-*).
    pub const DEP_001: &str = "CACG-DEP-001"; // DEP_UNKNOWN_TARGET
    pub const DEP_002: &str = "CACG-DEP-002"; // DEP_CYCLE
    pub const DEP_003: &str = "CACG-DEP-003"; // DEP_ORPHAN
    pub const DEP_004: &str = "CACG-DEP-004"; // DEP_DANGLING_RETRACTED

    // Per-reading role map (CACG-ROLE-*).
    pub const ROLE_001: &str = "CACG-ROLE-001"; // ROLE_MAP_ASYMMETRY
    pub const ROLE_002: &str = "CACG-ROLE-002"; // ROLE_MISSING_ENTRY
    pub const ROLE_003: &str = "CACG-ROLE-003"; // ROLE_VOCAB_VIOLATION

    // Layer-2 / Layer-3 verifier (CACG-VERIFY-*). VERIFY-001 fires on
    // Layer-2 exact / fuzzy match failure; VERIFY-002 carries the
    // Layer-3 semantic verdict as a journal-event hint payload.
    pub const VERIFY_001: &str = "CACG-VERIFY-001"; // layer-2 verify failure
    pub const VERIFY_002: &str = "CACG-VERIFY-002"; // VERIFY_SEMANTIC
}
