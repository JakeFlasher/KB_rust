#![allow(clippy::unwrap_used)]
//! Byte-equal parity: every CACG-* constant exposed by Rust
//! `cacg_core::diagnostic::codes` matches the corresponding Python
//! constant in `legacy_python_oracle/src/cacg/lint/codes.py` (and the parent `cacg.diagnostic`
//! catalogue). The string VALUES are the contract; the Rust naming
//! convention (`<CATEGORY>_NNN`) differs from Python's descriptive
//! naming (`<CATEGORY>_<MEANING>`), so we test value-equality keyed by
//! Python's descriptive name.

use cacg_core::diagnostic::codes;

#[test]
fn rust_constants_byte_equal_python_lint_codes_py() {
    let pairs: &[(&str, &str, &str)] = &[
        ("FM_MISSING_REQUIRED", codes::FM_001, "CACG-FM-001"),
        ("FM_UNKNOWN_SCHEMA", codes::FM_002, "CACG-FM-002"),
        ("FM_EXTRA_FIELD", codes::FM_003, "CACG-FM-003"),
        ("FM_MISSING_DELIMITER", codes::FM_004, "CACG-FM-004"),
        ("FM_FORBIDDEN_YAML", codes::FM_005, "CACG-FM-005"),
        ("FM_PARSE_ERROR", codes::FM_006, "CACG-FM-006"),
        ("FM_NOT_MAPPING", codes::FM_007, "CACG-FM-007"),
        ("FM_OTHER_VALIDATION", codes::FM_008, "CACG-FM-008"),
        ("CITE_MALFORMED_CHUNK_ID", codes::CITE_001, "CACG-CITE-001"),
        ("CITE_MALFORMED_HASH", codes::CITE_002, "CACG-CITE-002"),
        ("CITE_REVERSED_PAGE_RANGE", codes::CITE_003, "CACG-CITE-003"),
        (
            "CITE_CHUNK_NOT_IN_MANIFEST",
            codes::CITE_004,
            "CACG-CITE-004",
        ),
        ("CITE_PAGE_DISJOINT", codes::CITE_005, "CACG-CITE-005"),
        (
            "CITE_SOURCE_NOT_IN_MANIFEST",
            codes::CITE_006,
            "CACG-CITE-006",
        ),
        ("HASH_CHUNK_MISMATCH", codes::HASH_001, "CACG-HASH-001"),
        ("HASH_CARD_STALE", codes::HASH_002, "CACG-HASH-002"),
        ("HASH_MANIFEST_TAMPER", codes::HASH_003, "CACG-HASH-003"),
        ("IDX_PUBLISH_FAILED", codes::IDX_004, "CACG-IDX-004"),
        ("IDX_OUT_INSIDE_CARDS", codes::IDX_005, "CACG-IDX-005"),
        ("IDX_SIDECAR_COLLISION", codes::IDX_006, "CACG-IDX-006"),
        (
            "IDX_INDEX_SIDECAR_COLLISION",
            codes::IDX_007,
            "CACG-IDX-007",
        ),
        ("IDX_CANONICAL_NOT_FILE", codes::IDX_008, "CACG-IDX-008"),
        ("CLI_FILE_NOT_FOUND", codes::CLI_001, "CACG-CLI-001"),
        ("CLI_NOT_IMPLEMENTED", codes::CLI_002, "CACG-CLI-002"),
        ("CLI_BAD_ARG", codes::CLI_003, "CACG-CLI-003"),
        ("CLI_CONFIG_ERROR", codes::CLI_004, "CACG-CLI-004"),
        ("CLI_EXISTS", codes::CLI_005, "CACG-CLI-005"),
        ("INGEST_CORRUPT", codes::INGEST_001, "CACG-INGEST-001"),
        ("INGEST_EMPTY", codes::INGEST_002, "CACG-INGEST-002"),
        (
            "INGEST_PUBLISH_FAILED",
            codes::INGEST_003,
            "CACG-INGEST-003",
        ),
        ("MAN_MALFORMED", codes::MAN_001, "CACG-MAN-001"),
        ("MAN_SIDECAR_COLLISION", codes::MAN_002, "CACG-MAN-002"),
        ("MAN_CANONICAL_NOT_FILE", codes::MAN_003, "CACG-MAN-003"),
        ("JNL_TAMPERED", codes::JNL_001, "CACG-JNL-001"),
        ("AUTH_MATRIX_INVALID", codes::AUTH_000, "CACG-AUTH-000"),
        ("AUTH_READING_UNKNOWN", codes::AUTH_001, "CACG-AUTH-001"),
        ("AUTH_SOURCE_UNAUTHORIZED", codes::AUTH_002, "CACG-AUTH-002"),
        ("RETR_CARD_RETRACTED", codes::RETR_001, "CACG-RETR-001"),
        ("RETR_SOURCE_RETRACTED", codes::RETR_002, "CACG-RETR-002"),
        ("RETR_CHUNK_RETRACTED", codes::RETR_003, "CACG-RETR-003"),
        ("SUM_MISSING", codes::SUM_001, "CACG-SUM-001"),
        ("SUM_OVERSIZED", codes::SUM_002, "CACG-SUM-002"),
        ("SUM_TAG_NON_CONFORMING", codes::SUM_003, "CACG-SUM-003"),
        ("SUM_TAGS_OVERSIZED", codes::SUM_004, "CACG-SUM-004"),
        ("SKILL_NAME_COLLISION", codes::SKILL_001, "CACG-SKILL-001"),
        (
            "SKILL_ROUTES_TO_UNKNOWN",
            codes::SKILL_002,
            "CACG-SKILL-002",
        ),
        ("SKILL_SCHEMA_VIOLATION", codes::SKILL_003, "CACG-SKILL-003"),
        ("DEP_UNKNOWN_TARGET", codes::DEP_001, "CACG-DEP-001"),
        ("DEP_CYCLE", codes::DEP_002, "CACG-DEP-002"),
        ("DEP_ORPHAN", codes::DEP_003, "CACG-DEP-003"),
        ("DEP_DANGLING_RETRACTED", codes::DEP_004, "CACG-DEP-004"),
        ("ROLE_MAP_ASYMMETRY", codes::ROLE_001, "CACG-ROLE-001"),
        ("ROLE_MISSING_ENTRY", codes::ROLE_002, "CACG-ROLE-002"),
        ("ROLE_VOCAB_VIOLATION", codes::ROLE_003, "CACG-ROLE-003"),
        ("VERIFY_LAYER2", codes::VERIFY_001, "CACG-VERIFY-001"),
        ("VERIFY_SEMANTIC", codes::VERIFY_002, "CACG-VERIFY-002"),
    ];
    let mut failures = Vec::new();
    for (py_name, rust_value, expected) in pairs {
        if *rust_value != *expected {
            failures.push(format!(
                "Python {py_name}: expected {expected:?}, Rust got {rust_value:?}"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "Rust diagnostic constants drift from Python:\n  {}",
        failures.join("\n  ")
    );
}
