//! AC-C4 schema-fixture commands: `gen-schema-fixtures` regenerates
//! the generated_pydantic_errors manifest via Python; `audit-schema-fixtures`
//! validates the committed manifest's invariants against an explicit
//! 80-row required matrix.

use std::io;
use std::path::{Path, PathBuf};

/// Audit report counts.
#[derive(Debug)]
pub struct AuditReport {
    /// Total fixture entries in the manifest.
    pub total: usize,
    /// Number of fixtures with `oracle_layer == "parse"`.
    pub parse_count: usize,
    /// Number of fixtures with `oracle_layer == "lint"`.
    pub lint_count: usize,
}

/// One row in the required-fixture coverage matrix.
///
/// Tuple shape: `(name, field, category, oracle_layer, expected_code)`.
/// `audit()` asserts the manifest's row set matches this list exactly.
/// Codex R16-REVIEW-1: the audit must fail if any row is missing OR if
/// any unexpected row appears (so a regenerator that drops a row and
/// substitutes a fake one is still caught).
type RowTuple = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
);

/// Required fixture rows. Mirrors the committed
/// `tests/parity_corpus/generated_pydantic_errors/manifest.json`
/// row set exactly. Updating this table is the documented path for
/// adding/removing fixtures (the manifest + this matrix must move
/// together; the audit fails-loud on drift).
const REQUIRED_FIXTURE_ROWS: &[RowTuple] = &[
    // CACG-FM-001 missing required field (6 rows)
    (
        "missing-required-schema_version",
        "schema_version",
        "missing",
        "parse",
        "CACG-FM-001",
    ),
    (
        "missing-required-id",
        "id",
        "missing",
        "parse",
        "CACG-FM-001",
    ),
    (
        "missing-required-title",
        "title",
        "missing",
        "parse",
        "CACG-FM-001",
    ),
    (
        "missing-required-reading_id",
        "reading_id",
        "missing",
        "parse",
        "CACG-FM-001",
    ),
    (
        "missing-required-summary",
        "summary",
        "missing",
        "parse",
        "CACG-FM-001",
    ),
    (
        "missing-required-citations",
        "citations",
        "missing",
        "parse",
        "CACG-FM-001",
    ),
    // CACG-FM-002 unknown schema_version (7 rows: 1 original + 6 variants)
    (
        "wrong-schema-version",
        "schema_version",
        "literal_error",
        "parse",
        "CACG-FM-002",
    ),
    (
        "schema-version-cacg-v1",
        "schema_version",
        "literal_error",
        "parse",
        "CACG-FM-002",
    ),
    (
        "schema-version-cacg-v0-0",
        "schema_version",
        "literal_error",
        "parse",
        "CACG-FM-002",
    ),
    (
        "schema-version-CACG-v0",
        "schema_version",
        "literal_error",
        "parse",
        "CACG-FM-002",
    ),
    (
        "schema-version-v0",
        "schema_version",
        "literal_error",
        "parse",
        "CACG-FM-002",
    ),
    (
        "schema-version-empty",
        "schema_version",
        "literal_error",
        "parse",
        "CACG-FM-002",
    ),
    (
        "schema-version-cacg-v0",
        "schema_version",
        "literal_error",
        "parse",
        "CACG-FM-002",
    ),
    // CACG-FM-003 extra/unknown field
    (
        "extra-field",
        "extra_unknown_field",
        "extra",
        "parse",
        "CACG-FM-003",
    ),
    // CACG-FM-004 missing delimiters
    (
        "frontmatter-missing-delimiters",
        "<root>",
        "yaml_shape",
        "parse",
        "CACG-FM-004",
    ),
    // CACG-FM-005 forbidden YAML construct
    (
        "yaml-anchor",
        "reading_id",
        "yaml_error",
        "parse",
        "CACG-FM-005",
    ),
    ("yaml-alias", "title", "yaml_error", "parse", "CACG-FM-005"),
    // CACG-FM-006 YAML parse error / duplicate keys
    (
        "yaml-duplicate-key-id",
        "id",
        "yaml_error",
        "parse",
        "CACG-FM-006",
    ),
    // CACG-FM-007 root not mapping
    (
        "frontmatter-root-is-array",
        "<root>",
        "yaml_shape",
        "parse",
        "CACG-FM-007",
    ),
    // CACG-FM-008 catch-all (per-field empties + types + card_hash + card_edges)
    (
        "empty-citations",
        "citations",
        "too_short",
        "parse",
        "CACG-FM-008",
    ),
    ("id-empty", "id", "string_too_short", "parse", "CACG-FM-008"),
    (
        "title-empty",
        "title",
        "string_too_short",
        "parse",
        "CACG-FM-008",
    ),
    (
        "reading_id-empty",
        "reading_id",
        "string_too_short",
        "parse",
        "CACG-FM-008",
    ),
    (
        "id-wrong-type-int",
        "id",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "id-wrong-type-bool",
        "id",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "id-wrong-type-null",
        "id",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "id-wrong-type-array",
        "id",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "id-wrong-type-object",
        "id",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "title-wrong-type-int",
        "title",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "title-wrong-type-bool",
        "title",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "title-wrong-type-null",
        "title",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "title-wrong-type-array",
        "title",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "title-wrong-type-object",
        "title",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "reading_id-wrong-type-int",
        "reading_id",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "reading_id-wrong-type-bool",
        "reading_id",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "reading_id-wrong-type-null",
        "reading_id",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "reading_id-wrong-type-array",
        "reading_id",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "reading_id-wrong-type-object",
        "reading_id",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "summary-wrong-type-int",
        "summary",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "summary-wrong-type-bool",
        "summary",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "summary-wrong-type-null",
        "summary",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "summary-wrong-type-array",
        "summary",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "summary-wrong-type-object",
        "summary",
        "type_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "citation-quote-empty",
        "citations.0.quote",
        "value_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "citation-edge-type-invalid-literal",
        "citations.0.edge_type",
        "literal_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "citation-empty-source-id",
        "citations.0.source_id",
        "value_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "card-edges-invalid-edge-type",
        "card_edges",
        "card_edges_validator",
        "parse",
        "CACG-FM-008",
    ),
    (
        "card-edges-duplicate-target-edgetype",
        "card_edges",
        "card_edges_validator",
        "parse",
        "CACG-FM-008",
    ),
    (
        "card-edges-empty-target",
        "card_edges",
        "card_edges_validator",
        "parse",
        "CACG-FM-008",
    ),
    (
        "card-edges-not-list",
        "card_edges",
        "card_edges_validator",
        "parse",
        "CACG-FM-008",
    ),
    (
        "card-hash-too-short",
        "card_hash",
        "value_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "card-hash-non-hex",
        "card_hash",
        "value_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "card-hash-63-chars",
        "card_hash",
        "value_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "card-hash-65-chars",
        "card_hash",
        "value_error",
        "parse",
        "CACG-FM-008",
    ),
    (
        "card-hash-wrong-type-int",
        "card_hash",
        "value_error",
        "parse",
        "CACG-FM-008",
    ),
    // CACG-CITE-002 chunk_hash format
    (
        "citation-bad-chunk-hash",
        "citations.0.chunk_hash",
        "value_error",
        "parse",
        "CACG-CITE-002",
    ),
    // CACG-CITE-003 page_range (parse-time)
    (
        "citation-reversed-page-range",
        "citations.0.page_range",
        "value_error",
        "parse",
        "CACG-CITE-003",
    ),
    (
        "citation-page-range-coercion-true-false",
        "citations.0.page_range",
        "value_error",
        "parse",
        "CACG-CITE-003",
    ),
    (
        "citation-page-range-coercion-1.5-2.0",
        "citations.0.page_range",
        "value_error",
        "parse",
        "CACG-CITE-003",
    ),
    (
        "citation-page-range-coercion-1-2",
        "citations.0.page_range",
        "value_error",
        "parse",
        "CACG-CITE-003",
    ),
    (
        "citation-page-range-zero-start",
        "citations.0.page_range",
        "value_error",
        "parse",
        "CACG-CITE-003",
    ),
    (
        "citation-page-range-single-element",
        "citations.0.page_range",
        "value_error",
        "parse",
        "CACG-FM-001",
    ),
    (
        "citation-page-range-three-elements",
        "citations.0.page_range",
        "value_error",
        "parse",
        "CACG-CITE-003",
    ),
    // CACG-SUM-001 / SUM-002 summary length
    (
        "summary-too-short",
        "summary",
        "string_too_short",
        "parse",
        "CACG-SUM-001",
    ),
    (
        "summary-too-long",
        "summary",
        "string_too_long",
        "parse",
        "CACG-SUM-002",
    ),
    // CACG-SUM-003 tags slug + element + container
    (
        "tags-uppercase-not-slug",
        "tags",
        "tags_validator",
        "parse",
        "CACG-SUM-003",
    ),
    (
        "tags-too-short",
        "tags",
        "tags_validator",
        "parse",
        "CACG-SUM-003",
    ),
    (
        "tags-too-long",
        "tags",
        "tags_validator",
        "parse",
        "CACG-SUM-003",
    ),
    (
        "tags-duplicate",
        "tags",
        "tags_validator",
        "parse",
        "CACG-SUM-003",
    ),
    (
        "tags-non-string",
        "tags",
        "tags_validator",
        "parse",
        "CACG-SUM-003",
    ),
    (
        "tags-not-list",
        "tags",
        "tags_validator",
        "parse",
        "CACG-SUM-003",
    ),
    // CACG-SUM-004 tags count
    (
        "tags-too-many",
        "tags",
        "tags_validator",
        "parse",
        "CACG-SUM-004",
    ),
    // Lint-layer (8 rows; oracle_layer="lint")
    (
        "citation-source-id-spaces",
        "citations.0.source_id",
        "value_error",
        "lint",
        "CACG-CITE-006",
    ),
    (
        "citation-chunk-id-no-colons",
        "citations.0.chunk_id",
        "value_error",
        "lint",
        "CACG-CITE-001",
    ),
    (
        "citation-chunk-id-non-numeric-ordinal",
        "citations.0.chunk_id",
        "value_error",
        "lint",
        "CACG-CITE-001",
    ),
    (
        "card-hash-all-zero-placeholder",
        "card_hash",
        "value_error",
        "lint",
        "CACG-HASH-002",
    ),
    (
        "citation-source-chunk-mismatch",
        "citations.0.source_id",
        "value_error",
        "lint",
        "CACG-CITE-004",
    ),
    (
        "citation-chunk-id-prefix-divergent",
        "citations.0.source_id",
        "value_error",
        "lint",
        "CACG-CITE-001",
    ),
    (
        "citation-page-range-disjoint",
        "citations.0.page_range",
        "lint_value_error",
        "lint",
        "CACG-CITE-005",
    ),
    (
        "citation-chunk-id-not-in-manifest",
        "citations.0.chunk_id",
        "lint_value_error",
        "lint",
        "CACG-CITE-004",
    ),
];

/// Exact total fixture count required by AC-C4.
const REQUIRED_TOTAL: usize = 80;
/// Exact parse-layer count required by AC-C4.
const REQUIRED_PARSE_COUNT: usize = 72;
/// Exact lint-layer count required by AC-C4.
const REQUIRED_LINT_COUNT: usize = 8;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p
}

/// Regenerate the generated_pydantic_errors manifest via Rust-native
/// card generation and parsing. No Python spawn required.
pub fn gen() -> io::Result<()> {
    let repo = workspace_root();
    let manifest_path = repo.join("tests/parity_corpus/generated_pydantic_errors/manifest.json");

    let chunks_manifest_path = repo.join("tests/parity_corpus/out_python/chunks_manifest.json");
    let chunk_hash;
    let chunk_id;
    let source_id;
    let quote;
    if chunks_manifest_path.is_file() {
        let cm_text = std::fs::read_to_string(&chunks_manifest_path)?;
        let cm: serde_json::Value = serde_json::from_str(&cm_text)
            .map_err(|e| io::Error::other(format!("parse chunks_manifest: {e}")))?;
        let chunk = &cm["chunks"][0];
        chunk_hash = chunk["chunk_hash"]
            .as_str()
            .unwrap_or(&"0".repeat(64))
            .to_string();
        chunk_id = chunk["chunk_id"]
            .as_str()
            .unwrap_or("sample:p001:0000")
            .to_string();
        source_id = chunk["source_id"].as_str().unwrap_or("sample").to_string();
        let raw_text = chunk["text"].as_str().unwrap_or("fallback quote");
        quote = raw_text
            .chars()
            .take(60)
            .collect::<String>()
            .trim()
            .replace('"', "'");
    } else {
        chunk_hash = "0".repeat(64);
        chunk_id = "sample:p001:0000".to_string();
        source_id = "sample".to_string();
        quote = "Content-addressable identity is the verification primitive a".to_string();
    }

    let template = valid_card_template(&chunk_hash, &chunk_id, &source_id, &quote);
    let fixtures = generate_all_fixtures(&template);

    let mut manifest_entries: Vec<serde_json::Value> = Vec::new();
    for f in &fixtures {
        let actual_code = run_fixture_through_parser(&f.card_text, &f.oracle_layer, &repo);
        manifest_entries.push(serde_json::json!({
            "actual_code": actual_code,
            "card_text": f.card_text,
            "category": f.category,
            "expected_code": f.expected_code,
            "field": f.field,
            "name": f.name,
            "oracle_layer": f.oracle_layer,
        }));
    }

    let manifest = serde_json::json!({
        "schema_version": "cacg.v0.parity",
        "fixtures": manifest_entries,
    });
    let manifest_bytes = cacg_core::canonical_json::canonical_json(&manifest)
        .map_err(|e| io::Error::other(format!("canonical_json: {e}")))?;
    std::fs::create_dir_all(manifest_path.parent().unwrap())?;
    std::fs::write(&manifest_path, format!("{manifest_bytes}\n"))?;

    eprintln!(
        "xtask gen-schema-fixtures: {} fixtures regenerated (Rust-native)",
        fixtures.len()
    );
    Ok(())
}

struct Fixture {
    name: String,
    category: String,
    field: String,
    expected_code: String,
    oracle_layer: String,
    card_text: String,
}

fn valid_card_template(
    chunk_hash: &str,
    chunk_id: &str,
    source_id: &str,
    quote: &str,
) -> Vec<String> {
    vec![
        "---".to_string(),
        "schema_version: \"cacg.v0\"".to_string(),
        "id: \"test-card\"".to_string(),
        "title: \"Test Card\"".to_string(),
        "reading_id: \"reading_01\"".to_string(),
        "summary: \"A bounded summary of at least eighty characters to satisfy the SUM-001 / SUM-002 minimum length validator constraint for parity-corpus fixture generation.\"".to_string(),
        "citations:".to_string(),
        format!("  - source_id: \"{source_id}\""),
        format!("    chunk_id: \"{chunk_id}\""),
        format!("    chunk_hash: \"{chunk_hash}\""),
        "    page_range: [1, 2]".to_string(),
        format!("    quote: \"{quote}\""),
        "    edge_type: \"supports\"".to_string(),
        "---".to_string(),
        String::new(),
        "Body text.".to_string(),
    ]
}

fn lines_to_card(lines: &[String]) -> String {
    lines.join("\n") + "\n"
}

fn remove_field(template: &[String], field: &str) -> Vec<String> {
    if field == "citations" {
        let mut cleaned = Vec::new();
        let mut skip = false;
        for line in template {
            if line.starts_with("citations:") {
                skip = true;
                continue;
            }
            if skip && (line.starts_with("  ") || line.starts_with('\t')) {
                continue;
            }
            skip = false;
            cleaned.push(line.clone());
        }
        cleaned
    } else {
        template
            .iter()
            .filter(|l| !l.starts_with(&format!("{field}:")))
            .cloned()
            .collect()
    }
}

fn replace_field(template: &[String], field: &str, new_value: &str) -> Vec<String> {
    template
        .iter()
        .map(|line| {
            if line.starts_with(&format!("{field}:"))
                || line
                    .trim_start_matches("    ")
                    .starts_with(&format!("{field}:"))
                || line
                    .trim_start_matches("  - ")
                    .starts_with(&format!("{field}:"))
            {
                let prefix_end = line.find(':').unwrap() + 2;
                let prefix = &line[..prefix_end];
                format!("{prefix}{new_value}")
            } else {
                line.clone()
            }
        })
        .collect()
}

fn insert_before_closing(template: &[String], line_to_insert: &str) -> Vec<String> {
    let mut result = template.to_vec();
    let closing = result
        .iter()
        .rposition(|l| l == "---")
        .unwrap_or(result.len());
    result.insert(closing, line_to_insert.to_string());
    result
}

fn insert_before_citations(template: &[String], line_to_insert: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in template {
        if line.starts_with("citations:") {
            result.push(line_to_insert.to_string());
        }
        result.push(line.clone());
    }
    result
}

fn generate_all_fixtures(template: &[String]) -> Vec<Fixture> {
    let mut fixtures = Vec::new();

    let required_fields = [
        "schema_version",
        "id",
        "title",
        "reading_id",
        "summary",
        "citations",
    ];
    for field in &required_fields {
        let lines = remove_field(template, field);
        fixtures.push(Fixture {
            name: format!("missing-required-{field}"),
            category: "missing".to_string(),
            field: field.to_string(),
            expected_code: "CACG-FM-001".to_string(),
            oracle_layer: "parse".to_string(),
            card_text: lines_to_card(&lines),
        });
    }

    let lines = insert_before_closing(template, "extra_unknown_field: \"oops\"");
    fixtures.push(Fixture {
        name: "extra-field".to_string(),
        category: "extra".to_string(),
        field: "extra_unknown_field".to_string(),
        expected_code: "CACG-FM-003".to_string(),
        oracle_layer: "parse".to_string(),
        card_text: lines_to_card(&lines),
    });

    let lines = replace_field(template, "schema_version", "\"cacg.v999\"");
    fixtures.push(Fixture {
        name: "wrong-schema-version".to_string(),
        category: "literal_error".to_string(),
        field: "schema_version".to_string(),
        expected_code: "CACG-FM-002".to_string(),
        oracle_layer: "parse".to_string(),
        card_text: lines_to_card(&lines),
    });

    let lines = replace_field(template, "summary", "\"too short\"");
    fixtures.push(Fixture {
        name: "summary-too-short".to_string(),
        category: "string_too_short".to_string(),
        field: "summary".to_string(),
        expected_code: "CACG-SUM-001".to_string(),
        oracle_layer: "parse".to_string(),
        card_text: lines_to_card(&lines),
    });

    let long_summary = format!("\"{}\"", "x".repeat(500));
    let lines = replace_field(template, "summary", &long_summary);
    fixtures.push(Fixture {
        name: "summary-too-long".to_string(),
        category: "string_too_long".to_string(),
        field: "summary".to_string(),
        expected_code: "CACG-SUM-002".to_string(),
        oracle_layer: "parse".to_string(),
        card_text: lines_to_card(&lines),
    });

    let lines = replace_field(
        template,
        "chunk_hash",
        "\"not_64_hex_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"",
    );
    fixtures.push(Fixture {
        name: "citation-bad-chunk-hash".to_string(),
        category: "value_error".to_string(),
        field: "citations.0.chunk_hash".to_string(),
        expected_code: "CACG-CITE-002".to_string(),
        oracle_layer: "parse".to_string(),
        card_text: lines_to_card(&lines),
    });

    let lines = replace_field(template, "page_range", "[5, 2]");
    fixtures.push(Fixture {
        name: "citation-reversed-page-range".to_string(),
        category: "value_error".to_string(),
        field: "citations.0.page_range".to_string(),
        expected_code: "CACG-CITE-003".to_string(),
        oracle_layer: "parse".to_string(),
        card_text: lines_to_card(&lines),
    });

    // empty-citations
    let mut cleaned = Vec::new();
    let mut skip = false;
    for line in template {
        if line.starts_with("citations:") {
            cleaned.push("citations: []".to_string());
            skip = true;
            continue;
        }
        if skip && (line.starts_with("  ") || line.starts_with('\t')) {
            continue;
        }
        skip = false;
        cleaned.push(line.clone());
    }
    fixtures.push(Fixture {
        name: "empty-citations".to_string(),
        category: "too_short".to_string(),
        field: "citations".to_string(),
        expected_code: "CACG-FM-008".to_string(),
        oracle_layer: "parse".to_string(),
        card_text: lines_to_card(&cleaned),
    });

    // Empty-string top-level fields
    for (field, code) in [
        ("id", "CACG-FM-008"),
        ("title", "CACG-FM-008"),
        ("reading_id", "CACG-FM-008"),
    ] {
        let lines = replace_field(template, field, "\"\"");
        fixtures.push(Fixture {
            name: format!("{field}-empty"),
            category: "string_too_short".to_string(),
            field: field.to_string(),
            expected_code: code.to_string(),
            oracle_layer: "parse".to_string(),
            card_text: lines_to_card(&lines),
        });
    }

    // page_range coercion variants
    for (val, suffix) in [
        ("[true, false]", "true-false"),
        ("[1.5, 2.0]", "1.5-2.0"),
        ("[\"1\", \"2\"]", "1-2"),
    ] {
        let lines = replace_field(template, "page_range", val);
        fixtures.push(Fixture {
            name: format!("citation-page-range-coercion-{suffix}"),
            category: "value_error".to_string(),
            field: "citations.0.page_range".to_string(),
            expected_code: "CACG-CITE-003".to_string(),
            oracle_layer: "parse".to_string(),
            card_text: lines_to_card(&lines),
        });
    }

    // YAML duplicate key
    let mut lines = template.to_vec();
    lines.insert(2, "id: \"duplicate-id-second-value\"".to_string());
    fixtures.push(Fixture {
        name: "yaml-duplicate-key-id".to_string(),
        category: "yaml_error".to_string(),
        field: "id".to_string(),
        expected_code: "CACG-FM-006".to_string(),
        oracle_layer: "parse".to_string(),
        card_text: lines_to_card(&lines),
    });

    // YAML anchor
    let lines = replace_field(template, "reading_id", "&anchor_name \"reading_01\"");
    fixtures.push(Fixture {
        name: "yaml-anchor".to_string(),
        category: "yaml_error".to_string(),
        field: "reading_id".to_string(),
        expected_code: "CACG-FM-005".to_string(),
        oracle_layer: "parse".to_string(),
        card_text: lines_to_card(&lines),
    });

    // YAML alias
    let mut lines = template.to_vec();
    let closing = lines.iter().rposition(|l| l == "---").unwrap();
    lines.insert(closing, "_anchor: &foo \"value\"".to_string());
    let lines: Vec<String> = lines
        .iter()
        .map(|l| {
            if l.starts_with("title:") {
                "title: *foo".to_string()
            } else {
                l.clone()
            }
        })
        .collect();
    fixtures.push(Fixture {
        name: "yaml-alias".to_string(),
        category: "yaml_error".to_string(),
        field: "title".to_string(),
        expected_code: "CACG-FM-005".to_string(),
        oracle_layer: "parse".to_string(),
        card_text: lines_to_card(&lines),
    });

    // Wrong-type variants for string fields
    for field in ["id", "title", "reading_id", "summary"] {
        for (val, label) in [
            ("123", "int"),
            ("true", "bool"),
            ("null", "null"),
            ("[]", "array"),
            ("{}", "object"),
        ] {
            let lines = replace_field(template, field, val);
            fixtures.push(Fixture {
                name: format!("{field}-wrong-type-{label}"),
                category: "type_error".to_string(),
                field: field.to_string(),
                expected_code: "CACG-FM-008".to_string(),
                oracle_layer: "parse".to_string(),
                card_text: lines_to_card(&lines),
            });
        }
    }

    // Citation field attacks
    let citation_attacks: Vec<(&str, &str, &str, &str, &str)> = vec![
        (
            "source_id",
            "\"!!invalid id with spaces!!\"",
            "source-id-spaces",
            "value_error",
            "CACG-CITE-006",
        ),
        (
            "chunk_id",
            "\"malformed-id-no-colons\"",
            "chunk-id-no-colons",
            "value_error",
            "CACG-CITE-001",
        ),
        (
            "chunk_id",
            "\"sample:p001:not-a-number\"",
            "chunk-id-non-numeric-ordinal",
            "value_error",
            "CACG-CITE-001",
        ),
        (
            "page_range",
            "[0, 5]",
            "page-range-zero-start",
            "value_error",
            "CACG-CITE-003",
        ),
        (
            "page_range",
            "[1]",
            "page-range-single-element",
            "value_error",
            "CACG-FM-001",
        ),
        (
            "page_range",
            "[1, 2, 3]",
            "page-range-three-elements",
            "value_error",
            "CACG-CITE-003",
        ),
        ("quote", "\"\"", "quote-empty", "value_error", "CACG-FM-008"),
        (
            "edge_type",
            "\"made_up_edge_type\"",
            "edge-type-invalid-literal",
            "literal_error",
            "CACG-FM-008",
        ),
    ];
    for (field_path, value, suffix, category, code) in &citation_attacks {
        let lines = replace_field(template, field_path, value);
        fixtures.push(Fixture {
            name: format!("citation-{suffix}"),
            category: category.to_string(),
            field: format!("citations.0.{field_path}"),
            expected_code: code.to_string(),
            oracle_layer: if code.starts_with("CACG-CITE-006")
                || code.starts_with("CACG-CITE-001")
                || code.starts_with("CACG-CITE-004")
            {
                "lint".to_string()
            } else {
                "parse".to_string()
            },
            card_text: lines_to_card(&lines),
        });
    }

    // Tags validators
    let tags_too_long = format!("tags: [\"{}\"]", "x".repeat(50));
    let tags_too_many = format!(
        "tags: {}",
        serde_json::to_string(&(0..20).map(|i| format!("t{i}")).collect::<Vec<_>>()).unwrap()
    );
    let tag_attacks: Vec<(&str, &str, &str)> = vec![
        (
            "tags: [\"UPPERCASE\"]",
            "tags-uppercase-not-slug",
            "CACG-SUM-003",
        ),
        ("tags: [\"a\"]", "tags-too-short", "CACG-SUM-003"),
        (&tags_too_long, "tags-too-long", "CACG-SUM-003"),
        ("tags: [\"dup\", \"dup\"]", "tags-duplicate", "CACG-SUM-003"),
        (&tags_too_many, "tags-too-many", "CACG-SUM-004"),
        ("tags: [123]", "tags-non-string", "CACG-SUM-003"),
        ("tags: \"not-a-list\"", "tags-not-list", "CACG-SUM-003"),
    ];
    for (attack_line, name, code) in &tag_attacks {
        let lines = insert_before_citations(template, attack_line.trim());
        fixtures.push(Fixture {
            name: name.to_string(),
            category: "tags_validator".to_string(),
            field: "tags".to_string(),
            expected_code: code.to_string(),
            oracle_layer: "parse".to_string(),
            card_text: lines_to_card(&lines),
        });
    }

    // Card-edges attacks
    let edge_attacks: Vec<(&str, &str, &str)> = vec![
        ("card_edges:\n  - target: \"neighbor-card\"\n    edge_type: \"made_up_type\"", "card-edges-invalid-edge-type", "CACG-FM-008"),
        ("card_edges:\n  - target: \"neighbor\"\n    edge_type: \"depends_on\"\n  - target: \"neighbor\"\n    edge_type: \"depends_on\"", "card-edges-duplicate-target-edgetype", "CACG-FM-008"),
        ("card_edges:\n  - target: \"\"\n    edge_type: \"depends_on\"", "card-edges-empty-target", "CACG-FM-008"),
        ("card_edges: \"not-a-list\"", "card-edges-not-list", "CACG-FM-008"),
    ];
    for (attack, name, code) in &edge_attacks {
        let lines = insert_before_citations(template, attack);
        fixtures.push(Fixture {
            name: name.to_string(),
            category: "card_edges_validator".to_string(),
            field: "card_edges".to_string(),
            expected_code: code.to_string(),
            oracle_layer: "parse".to_string(),
            card_text: lines_to_card(&lines),
        });
    }

    // Frontmatter shape errors
    fixtures.push(Fixture {
        name: "frontmatter-missing-delimiters".to_string(),
        category: "yaml_shape".to_string(),
        field: "<root>".to_string(),
        expected_code: "CACG-FM-004".to_string(),
        oracle_layer: "parse".to_string(),
        card_text: "schema_version: cacg.v0\nid: foo\n\nBody text without ---\n".to_string(),
    });
    fixtures.push(Fixture {
        name: "frontmatter-root-is-array".to_string(),
        category: "yaml_shape".to_string(),
        field: "<root>".to_string(),
        expected_code: "CACG-FM-007".to_string(),
        oracle_layer: "parse".to_string(),
        card_text: "---\n- not\n- a\n- mapping\n---\n\nBody.\n".to_string(),
    });

    // Card-hash attacks
    let hash_non_hex = format!("\"{}\"", "g".repeat(64));
    let hash_all_zero = format!("\"{}\"", "0".repeat(64));
    let hash_63 = format!("\"{}\"", "a".repeat(63));
    let hash_65 = format!("\"{}\"", "a".repeat(65));
    let hash_attacks: Vec<(&str, &str, &str)> = vec![
        ("\"too-short\"", "too-short", "CACG-FM-008"),
        (&hash_non_hex, "non-hex", "CACG-FM-008"),
        (&hash_all_zero, "all-zero-placeholder", "CACG-HASH-002"),
        (&hash_63, "63-chars", "CACG-FM-008"),
        (&hash_65, "65-chars", "CACG-FM-008"),
        ("123", "wrong-type-int", "CACG-FM-008"),
    ];
    for (bad_hash, label, code) in &hash_attacks {
        let lines = insert_before_citations(template, &format!("card_hash: {bad_hash}"));
        fixtures.push(Fixture {
            name: format!("card-hash-{label}"),
            category: "value_error".to_string(),
            field: "card_hash".to_string(),
            expected_code: code.to_string(),
            oracle_layer: if *code == "CACG-HASH-002" {
                "lint"
            } else {
                "parse"
            }
            .to_string(),
            card_text: lines_to_card(&lines),
        });
    }

    // Schema version variants
    for bad_version in ["cacg.v1", "cacg.v0.0", "CACG.v0", "v0", "", "cacg.v0 "] {
        let lines = replace_field(template, "schema_version", &format!("\"{bad_version}\""));
        let safe_name = if bad_version.is_empty() {
            "empty"
        } else {
            bad_version
        };
        fixtures.push(Fixture {
            name: format!(
                "schema-version-{}",
                safe_name.trim().replace('.', "-").replace(' ', "-")
            ),
            category: "literal_error".to_string(),
            field: "schema_version".to_string(),
            expected_code: "CACG-FM-002".to_string(),
            oracle_layer: "parse".to_string(),
            card_text: lines_to_card(&lines),
        });
    }

    // Source/chunk mismatch fixtures
    let mismatch_attacks: Vec<(&str, &str, &str, &str)> = vec![
        (
            "\"source_a\"",
            "\"different_source:p001:0000\"",
            "source-chunk-mismatch",
            "CACG-CITE-004",
        ),
        (
            "\"sample\"",
            "\"DIFFERENT_PREFIX:p001:0000\"",
            "chunk-id-prefix-divergent",
            "CACG-CITE-001",
        ),
        (
            "\"\"",
            "\"sample:p001:0000\"",
            "empty-source-id",
            "CACG-FM-008",
        ),
    ];
    for (src_id, chk_id, label, code) in &mismatch_attacks {
        let lines: Vec<String> = template
            .iter()
            .map(|line| {
                if line.contains("source_id:") && line.trim().starts_with("- source_id:")
                    || line.trim().starts_with("source_id:")
                {
                    line.split(':').next().unwrap().to_string() + ": " + src_id
                } else if line.contains("chunk_id:") {
                    line.split(':').next().unwrap().to_string() + ": " + chk_id
                } else {
                    line.clone()
                }
            })
            .collect();
        fixtures.push(Fixture {
            name: format!("citation-{label}"),
            category: "value_error".to_string(),
            field: "citations.0.source_id".to_string(),
            expected_code: code.to_string(),
            oracle_layer: if code.starts_with("CACG-CITE") {
                "lint"
            } else {
                "parse"
            }
            .to_string(),
            card_text: lines_to_card(&lines),
        });
    }

    // citation-page-range-disjoint (lint)
    let lines = replace_field(template, "page_range", "[100, 200]");
    fixtures.push(Fixture {
        name: "citation-page-range-disjoint".to_string(),
        category: "lint_value_error".to_string(),
        field: "citations.0.page_range".to_string(),
        expected_code: "CACG-CITE-005".to_string(),
        oracle_layer: "lint".to_string(),
        card_text: lines_to_card(&lines),
    });

    // citation-chunk-id-not-in-manifest (lint)
    let lines = replace_field(template, "chunk_id", "\"sample:p999:9999\"");
    fixtures.push(Fixture {
        name: "citation-chunk-id-not-in-manifest".to_string(),
        category: "lint_value_error".to_string(),
        field: "citations.0.chunk_id".to_string(),
        expected_code: "CACG-CITE-004".to_string(),
        oracle_layer: "lint".to_string(),
        card_text: lines_to_card(&lines),
    });

    fixtures
}

fn run_fixture_through_parser(
    card_text: &str,
    oracle_layer: &str,
    workspace_root: &Path,
) -> String {
    use cacg_core::frontmatter::parse_card;
    match oracle_layer {
        "parse" => match parse_card(card_text) {
            Ok(_) => "PASS".to_string(),
            Err(diags) => diags
                .first()
                .map(|d| d.code.clone())
                .unwrap_or_else(|| "UNKNOWN".to_string()),
        },
        "lint" => {
            let tmp_dir =
                std::env::temp_dir().join(format!("cacg-schema-fixture-{}", std::process::id()));
            let _ = std::fs::create_dir_all(&tmp_dir);
            let card_path = tmp_dir.join("fixture.md");
            std::fs::write(&card_path, card_text).expect("write fixture card");
            let chunks_manifest_path =
                workspace_root.join("tests/parity_corpus/out_python/chunks_manifest.json");
            let journal_path = tmp_dir.join("lint_journal.jsonl");
            let chunks_index =
                cacg_core::chunks_index::ChunksIndex::from_path(&chunks_manifest_path).ok();
            let result = cacg_core::lint::layer1::lint_card(
                &card_path,
                &chunks_manifest_path,
                &journal_path,
                chunks_index.as_ref(),
                None,
            );
            let _ = std::fs::remove_dir_all(&tmp_dir);
            match result {
                Ok(outcome) => {
                    if outcome.diagnostics.is_empty() {
                        "PASS".to_string()
                    } else {
                        outcome.diagnostics[0].code.clone()
                    }
                }
                Err(e) => format!("LINT-ERROR: {e:?}"),
            }
        }
        _ => "UNKNOWN-LAYER".to_string(),
    }
}

/// Audit the committed manifest. Returns an [`AuditReport`] on success,
/// or a descriptive error message describing the first invariant
/// violation.
///
/// Gates (in order; Codex R16-REVIEW-1 + R17-REVIEW-1):
/// 1. Per-fixture row-data validation in a loop: required fields
///    (`name`, `field`, `category`, `oracle_layer`, `expected_code`,
///    `actual_code`, `card_text`) all present; no duplicate `name`;
///    `skip_reason` absent; `actual_code == expected_code`; valid
///    `oracle_layer`. Per-fixture issues fail-fast.
/// 2. Row-matrix diff against [`REQUIRED_FIXTURE_ROWS`]: missing rows
///    AND unexpected rows are listed by name. Codex R17-REVIEW-1: this
///    fires BEFORE the count gates so a short manifest reports the
///    dropped row name rather than only "expected EXACTLY 80". The
///    total/per-layer count mismatch summary is appended after the
///    missing/unexpected lists so operators see both signals.
/// 3. Per-row metadata cross-check: field/category/layer/code drift
///    on matched names.
/// 4. Final sanity: exact total `== 80`, exact `parse == 72` +
///    `lint == 8`. Defensive against `REQUIRED_FIXTURE_ROWS` drift
///    from the count constants.
///
/// # Errors
///
/// Returns an `io::Error` whose `to_string()` describes the first
/// invariant violation reached. Per-fixture issues short-circuit
/// before matrix matching; matrix gaps short-circuit before count
/// sanity. Coverage diagnostics (missing/unexpected names) include
/// the appended count summary when applicable.
pub fn audit(manifest_path: &Path) -> io::Result<AuditReport> {
    let manifest_path = if manifest_path.is_absolute() {
        manifest_path.to_path_buf()
    } else {
        workspace_root().join(manifest_path)
    };
    if !manifest_path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("manifest missing: {}", manifest_path.display()),
        ));
    }
    let raw = std::fs::read_to_string(&manifest_path)?;
    let manifest: serde_json::Value = serde_json::from_str(&raw)
        .map_err(|e| io::Error::other(format!("manifest is not valid JSON: {e}")))?;
    let fixtures = manifest["fixtures"]
        .as_array()
        .ok_or_else(|| io::Error::other("manifest.fixtures must be an array"))?;

    // Gate 1: per-fixture required-field validation + counts. Ordered
    // before the total-count gate so per-fixture issues (duplicate
    // name, skip_reason, actual_code != expected_code, missing field)
    // surface with specific messages rather than being masked by a
    // count-mismatch error.
    let mut names: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut parse_count = 0usize;
    let mut lint_count = 0usize;
    let mut manifest_rows: std::collections::BTreeMap<String, ManifestRow> =
        std::collections::BTreeMap::new();
    for fixture in fixtures {
        let name = fixture["name"]
            .as_str()
            .ok_or_else(|| io::Error::other(format!("fixture missing `name`: {fixture:?}")))?;
        if !names.insert(name.to_string()) {
            return Err(io::Error::other(format!("duplicate fixture name: {name}")));
        }
        let field = fixture["field"]
            .as_str()
            .ok_or_else(|| io::Error::other(format!("fixture {name} missing required `field`")))?;
        let category = fixture["category"].as_str().ok_or_else(|| {
            io::Error::other(format!("fixture {name} missing required `category`"))
        })?;
        let oracle_layer = fixture["oracle_layer"].as_str().ok_or_else(|| {
            io::Error::other(format!("fixture {name} missing required `oracle_layer`"))
        })?;
        let expected_code = fixture["expected_code"].as_str().ok_or_else(|| {
            io::Error::other(format!("fixture {name} missing required `expected_code`"))
        })?;
        let actual_code = fixture["actual_code"].as_str().ok_or_else(|| {
            io::Error::other(format!("fixture {name} missing required `actual_code`"))
        })?;
        let _card_text = fixture["card_text"].as_str().ok_or_else(|| {
            io::Error::other(format!("fixture {name} missing required `card_text`"))
        })?;
        if fixture.get("skip_reason").is_some() {
            return Err(io::Error::other(format!(
                "fixture {name} carries `skip_reason`: silent skips not allowed (R12-REVIEW-1)"
            )));
        }
        if actual_code != expected_code {
            return Err(io::Error::other(format!(
                "fixture {name}: actual_code {actual_code} differs from expected_code {expected_code}; \
                 the Python generator's runtime classification drifted from the documented expectation"
            )));
        }
        match oracle_layer {
            "parse" => parse_count += 1,
            "lint" => lint_count += 1,
            other => {
                return Err(io::Error::other(format!(
                    "fixture {name} has invalid oracle_layer {other:?} (expected `parse` or `lint`)"
                )));
            }
        }
        manifest_rows.insert(
            name.to_string(),
            ManifestRow {
                field: field.to_string(),
                category: category.to_string(),
                oracle_layer: oracle_layer.to_string(),
                expected_code: expected_code.to_string(),
            },
        );
    }

    // Gate 2: row-matrix exact match. Codex R17-REVIEW-1: this MUST
    // run BEFORE the total/per-layer count gates so a short manifest
    // (e.g., 79 rows with one dropped) emits the missing row's NAME
    // rather than just "expected EXACTLY 80". AC-C4's audit contract
    // requires the diagnostic to list the missing rows by name in the
    // `(error_type × field)` coverage matrix.
    let mut required_set: std::collections::BTreeMap<String, ManifestRow> =
        std::collections::BTreeMap::new();
    for (name, field, category, oracle_layer, expected_code) in REQUIRED_FIXTURE_ROWS {
        required_set.insert(
            (*name).to_string(),
            ManifestRow {
                field: (*field).to_string(),
                category: (*category).to_string(),
                oracle_layer: (*oracle_layer).to_string(),
                expected_code: (*expected_code).to_string(),
            },
        );
    }
    let required_names: std::collections::BTreeSet<&String> = required_set.keys().collect();
    let manifest_names: std::collections::BTreeSet<&String> = manifest_rows.keys().collect();
    let missing: Vec<&&String> = required_names.difference(&manifest_names).collect();
    let unexpected: Vec<&&String> = manifest_names.difference(&required_names).collect();
    if !missing.is_empty() || !unexpected.is_empty() {
        let mut msg = String::new();
        if !missing.is_empty() {
            msg.push_str(&format!(
                "missing {} required row(s): {}\n",
                missing.len(),
                missing
                    .iter()
                    .map(|n| n.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if !unexpected.is_empty() {
            msg.push_str(&format!(
                "unexpected {} row(s): {}\n",
                unexpected.len(),
                unexpected
                    .iter()
                    .map(|n| n.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        // Append count / layer mismatch summary when applicable so the
        // operator sees BOTH the coverage gap (missing names) AND the
        // structural mismatch (total/per-layer counts).
        if fixtures.len() != REQUIRED_TOTAL {
            msg.push_str(&format!(
                "manifest has {} fixtures; expected EXACTLY {REQUIRED_TOTAL} (Codex R16-REVIEW-1)\n",
                fixtures.len()
            ));
        }
        if parse_count != REQUIRED_PARSE_COUNT {
            msg.push_str(&format!(
                "manifest has {parse_count} parse-layer fixtures; expected EXACTLY {REQUIRED_PARSE_COUNT}\n"
            ));
        }
        if lint_count != REQUIRED_LINT_COUNT {
            msg.push_str(&format!(
                "manifest has {lint_count} lint-layer fixtures; expected EXACTLY {REQUIRED_LINT_COUNT}\n"
            ));
        }
        return Err(io::Error::other(msg.trim_end().to_string()));
    }

    // Per-row metadata cross-check (names match but field/category/layer/code may drift).
    let mut metadata_mismatches: Vec<String> = Vec::new();
    for (name, required) in &required_set {
        let actual = &manifest_rows[name];
        if actual != required {
            metadata_mismatches.push(format!("row {name}: expected {required:?}, got {actual:?}"));
        }
    }
    if !metadata_mismatches.is_empty() {
        return Err(io::Error::other(format!(
            "{} fixture row(s) have metadata divergent from REQUIRED_FIXTURE_ROWS:\n{}",
            metadata_mismatches.len(),
            metadata_mismatches.join("\n")
        )));
    }

    // Gate 4 (final sanity): exact counts. By this point the row
    // matrix matches exactly + every row has correct metadata, so
    // these checks are redundant in steady-state but defend against
    // a future regression where REQUIRED_FIXTURE_ROWS drifts from
    // REQUIRED_TOTAL / REQUIRED_PARSE_COUNT / REQUIRED_LINT_COUNT.
    if fixtures.len() != REQUIRED_TOTAL {
        return Err(io::Error::other(format!(
            "manifest has {} fixtures; expected EXACTLY {REQUIRED_TOTAL} (Codex R16-REVIEW-1)",
            fixtures.len()
        )));
    }
    if parse_count != REQUIRED_PARSE_COUNT {
        return Err(io::Error::other(format!(
            "manifest has {parse_count} parse-layer fixtures; expected EXACTLY {REQUIRED_PARSE_COUNT}"
        )));
    }
    if lint_count != REQUIRED_LINT_COUNT {
        return Err(io::Error::other(format!(
            "manifest has {lint_count} lint-layer fixtures; expected EXACTLY {REQUIRED_LINT_COUNT}"
        )));
    }

    Ok(AuditReport {
        total: fixtures.len(),
        parse_count,
        lint_count,
    })
}

/// One manifest row's metadata (field/category/oracle_layer/expected_code).
/// Used for the row-matrix exact-match comparison.
#[derive(Debug, PartialEq, Eq)]
struct ManifestRow {
    field: String,
    category: String,
    oracle_layer: String,
    expected_code: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_manifest(dir: &tempfile::TempDir, contents: &str) -> PathBuf {
        let path = dir.path().join("manifest.json");
        fs::write(&path, contents).unwrap();
        path
    }

    #[test]
    fn audit_committed_manifest_passes() {
        let manifest =
            workspace_root().join("tests/parity_corpus/generated_pydantic_errors/manifest.json");
        let report = audit(&manifest).expect("committed manifest must audit clean");
        assert_eq!(report.total, REQUIRED_TOTAL);
        assert_eq!(report.parse_count, REQUIRED_PARSE_COUNT);
        assert_eq!(report.lint_count, REQUIRED_LINT_COUNT);
    }

    #[test]
    fn required_matrix_has_exact_count() {
        assert_eq!(
            REQUIRED_FIXTURE_ROWS.len(),
            REQUIRED_TOTAL,
            "REQUIRED_FIXTURE_ROWS must have exactly {REQUIRED_TOTAL} entries"
        );
    }

    #[test]
    fn audit_rejects_seven_row_false_green() {
        // Codex R16-REVIEW-1: the smoking-gun probe. Round-16's audit
        // false-greened a 7-row manifest containing only one fixture
        // per CACG-* code category. Round-17 must reject it because
        // 73 of 80 required rows are missing.
        let dir = tempfile::tempdir().unwrap();
        let path = write_manifest(
            &dir,
            r#"{
                "schema_version": "cacg.v0.parity",
                "fixtures": [
                    {"name":"a","field":"x","category":"missing","oracle_layer":"parse","expected_code":"CACG-FM-001","actual_code":"CACG-FM-001","card_text":"---\n---\nb"},
                    {"name":"b","field":"x","category":"x","oracle_layer":"parse","expected_code":"CACG-FM-002","actual_code":"CACG-FM-002","card_text":"---\n---\nb"},
                    {"name":"c","field":"x","category":"x","oracle_layer":"parse","expected_code":"CACG-FM-003","actual_code":"CACG-FM-003","card_text":"---\n---\nb"},
                    {"name":"d","field":"x","category":"x","oracle_layer":"parse","expected_code":"CACG-FM-008","actual_code":"CACG-FM-008","card_text":"---\n---\nb"},
                    {"name":"e","field":"x","category":"x","oracle_layer":"parse","expected_code":"CACG-CITE-002","actual_code":"CACG-CITE-002","card_text":"---\n---\nb"},
                    {"name":"f","field":"x","category":"x","oracle_layer":"parse","expected_code":"CACG-CITE-003","actual_code":"CACG-CITE-003","card_text":"---\n---\nb"},
                    {"name":"g","field":"x","category":"x","oracle_layer":"parse","expected_code":"CACG-SUM-003","actual_code":"CACG-SUM-003","card_text":"---\n---\nb"}
                ]
            }"#,
        );
        let r = audit(&path);
        assert!(
            r.is_err(),
            "7-row manifest must fail audit (Codex R16-REVIEW-1 false-green)"
        );
        let msg = r.unwrap_err().to_string();
        // Round 18 reorder: matrix-diff fires before count gate, so
        // the 7-row probe now also reports the missing row names AND
        // the count mismatch.
        assert!(
            msg.contains("missing"),
            "audit error must list missing rows: {msg}"
        );
        assert!(
            msg.contains("expected EXACTLY 80") || msg.contains("7 fixtures"),
            "audit error must also signal total-count mismatch: {msg}"
        );
    }

    #[test]
    fn audit_rejects_seventy_nine_row_dropped() {
        // Codex R17-REVIEW-1: a 79-row manifest (any single row dropped)
        // must report the DROPPED ROW NAME, not just "expected EXACTLY 80".
        // AC-C4 requires the audit to list the missing rows in the
        // `(error_type × field)` coverage matrix.
        let dropped = "citation-page-range-single-element";
        let manifest_text = build_manifest_text_minus(&[dropped]);
        let dir = tempfile::tempdir().unwrap();
        let path = write_manifest(&dir, &manifest_text);
        let r = audit(&path);
        assert!(r.is_err());
        let msg = r.unwrap_err().to_string();
        assert!(
            msg.contains(dropped),
            "audit error MUST name the dropped row {dropped:?} (R17-REVIEW-1): {msg}"
        );
        assert!(
            msg.contains("missing 1 required row"),
            "audit error must summarize the missing-row count: {msg}"
        );
        assert!(
            msg.contains("79") && msg.contains("expected EXACTLY 80"),
            "audit error must also append the total-count mismatch: {msg}"
        );
    }

    #[test]
    fn audit_rejects_missing_actual_code() {
        let manifest_text =
            build_manifest_text_with_drop_field("missing-required-id", "actual_code");
        let dir = tempfile::tempdir().unwrap();
        let path = write_manifest(&dir, &manifest_text);
        let r = audit(&path);
        assert!(r.is_err());
        let msg = r.unwrap_err().to_string();
        assert!(msg.contains("actual_code"));
    }

    #[test]
    fn audit_rejects_expected_actual_code_mismatch() {
        // Replace one row's `actual_code` with a different CACG-* value.
        let mut manifest: serde_json::Value =
            serde_json::from_str(&full_manifest_text()).expect("base manifest parses");
        let fixtures = manifest["fixtures"].as_array_mut().unwrap();
        for fixture in fixtures.iter_mut() {
            if fixture["name"] == "missing-required-id" {
                fixture["actual_code"] = serde_json::Value::String("CACG-FM-DRIFT".into());
                break;
            }
        }
        let dir = tempfile::tempdir().unwrap();
        let path = write_manifest(&dir, &serde_json::to_string(&manifest).unwrap());
        let r = audit(&path);
        assert!(r.is_err());
        let msg = r.unwrap_err().to_string();
        assert!(
            msg.contains("actual_code") && msg.contains("expected_code"),
            "audit error must explain code drift: {msg}"
        );
    }

    #[test]
    fn audit_rejects_missing_r16_regression_target() {
        // Drop one of the 15 R16 regression targets specifically and
        // assert the audit names it.
        let target = "citation-page-range-single-element";
        let manifest_text = build_manifest_text_minus(&[target]);
        // Add a synthetic filler to keep the total at 80 (so the
        // total-count gate doesn't short-circuit before the row-matrix
        // check fires).
        let mut manifest: serde_json::Value =
            serde_json::from_str(&manifest_text).expect("text parses");
        let fixtures = manifest["fixtures"].as_array_mut().unwrap();
        fixtures.push(serde_json::json!({
            "name": "synthetic-filler",
            "field": "x",
            "category": "x",
            "oracle_layer": "parse",
            "expected_code": "CACG-FM-008",
            "actual_code": "CACG-FM-008",
            "card_text": "---\n---\nb"
        }));
        let dir = tempfile::tempdir().unwrap();
        let path = write_manifest(&dir, &serde_json::to_string(&manifest).unwrap());
        let r = audit(&path);
        assert!(
            r.is_err(),
            "audit must reject when an R16 regression target is missing"
        );
        let msg = r.unwrap_err().to_string();
        assert!(
            msg.contains(target),
            "audit error must name the missing R16 target {target}: {msg}"
        );
    }

    #[test]
    fn audit_rejects_fake_extra_row_replacement() {
        // Replace a real row with a fake one (same total count, but
        // different name). Audit must list BOTH the missing and the
        // unexpected row.
        let target = "citation-bad-chunk-hash";
        let manifest_text = build_manifest_text_minus(&[target]);
        let mut manifest: serde_json::Value =
            serde_json::from_str(&manifest_text).expect("text parses");
        let fixtures = manifest["fixtures"].as_array_mut().unwrap();
        fixtures.push(serde_json::json!({
            "name": "fake-extra-row-not-in-required-matrix",
            "field": "x",
            "category": "x",
            "oracle_layer": "parse",
            "expected_code": "CACG-FM-008",
            "actual_code": "CACG-FM-008",
            "card_text": "---\n---\nb"
        }));
        let dir = tempfile::tempdir().unwrap();
        let path = write_manifest(&dir, &serde_json::to_string(&manifest).unwrap());
        let r = audit(&path);
        assert!(r.is_err());
        let msg = r.unwrap_err().to_string();
        assert!(
            msg.contains(target),
            "audit error must name the missing row {target}: {msg}"
        );
        assert!(
            msg.contains("fake-extra-row-not-in-required-matrix"),
            "audit error must name the unexpected row: {msg}"
        );
    }

    #[test]
    fn audit_rejects_duplicate_names() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_manifest(
            &dir,
            r#"{
                "schema_version": "cacg.v0.parity",
                "fixtures": [
                    {"name": "dup", "field":"x","category":"x","oracle_layer":"parse","expected_code":"CACG-FM-001","actual_code":"CACG-FM-001","card_text":"x"},
                    {"name": "dup", "field":"x","category":"x","oracle_layer":"parse","expected_code":"CACG-FM-001","actual_code":"CACG-FM-001","card_text":"x"}
                ]
            }"#,
        );
        let r = audit(&path);
        assert!(r.is_err());
        assert!(r.unwrap_err().to_string().contains("duplicate"));
    }

    #[test]
    fn audit_rejects_skip_reason() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_manifest(
            &dir,
            r#"{
                "schema_version": "cacg.v0.parity",
                "fixtures": [
                    {"name":"x","field":"x","category":"x","oracle_layer":"parse","expected_code":"CACG-FM-001","actual_code":"CACG-FM-001","card_text":"x","skip_reason":"..."}
                ]
            }"#,
        );
        let r = audit(&path);
        assert!(r.is_err());
        assert!(r.unwrap_err().to_string().contains("skip_reason"));
    }

    // --- Helpers for negative tests that build mutations of the
    //     committed manifest ---

    fn full_manifest_text() -> String {
        let p =
            workspace_root().join("tests/parity_corpus/generated_pydantic_errors/manifest.json");
        fs::read_to_string(&p).expect("read committed manifest")
    }

    fn build_manifest_text_minus(drop_names: &[&str]) -> String {
        let mut manifest: serde_json::Value =
            serde_json::from_str(&full_manifest_text()).expect("committed manifest parses");
        let fixtures = manifest["fixtures"].as_array_mut().unwrap();
        fixtures.retain(|f| {
            let n = f["name"].as_str().unwrap_or("");
            !drop_names.contains(&n)
        });
        serde_json::to_string(&manifest).unwrap()
    }

    fn build_manifest_text_with_drop_field(name: &str, drop_field: &str) -> String {
        let mut manifest: serde_json::Value =
            serde_json::from_str(&full_manifest_text()).expect("committed manifest parses");
        let fixtures = manifest["fixtures"].as_array_mut().unwrap();
        for fixture in fixtures.iter_mut() {
            if fixture["name"] == name {
                if let Some(obj) = fixture.as_object_mut() {
                    obj.remove(drop_field);
                }
                break;
            }
        }
        serde_json::to_string(&manifest).unwrap()
    }
}
