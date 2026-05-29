//! Canonical card writer + scaffold-card template.
//!
//! Byte-equal port of `legacy_python_oracle/src/cacg/frontmatter.py::render_card_text`
//! + `legacy_python_oracle/src/cacg/card_template.py::build_card_text`. The
//! `kb new` CLI verb (and any future scaffolder) calls
//! [`build_card_text`] to emit a `.md` file with a deliberate
//! placeholder citation that fails the linter — the "tripwire"
//! that surfaces the citation-resolution contract at authoring
//! time.
//!
//! Key-order constants here MUST stay byte-equal with the
//! Python `_CARD_FRONTMATTER_KEY_ORDER` / `_CITATION_KEY_ORDER` /
//! `_CARD_EDGE_KEY_ORDER` tuples; the order is part of the
//! canonical card-write contract (`write_card -> parse_card ->
//! write_card` must be the identity).

use serde_json::{json, Value};

const CARD_FRONTMATTER_KEY_ORDER: &[&str] = &[
    "schema_version",
    "id",
    "title",
    "reading_id",
    "summary",
    "tags",
    "card_edges",
    "citations",
    "card_hash",
];

const CITATION_KEY_ORDER: &[&str] = &[
    "source_id",
    "chunk_id",
    "chunk_hash",
    "page_range",
    "quote",
    "edge_type",
];

const CARD_EDGE_KEY_ORDER: &[&str] = &["target", "edge_type"];

const SCHEMA_VERSION: &str = "cacg.v0";

const DEFAULT_BODY: &str = "\
Write the card body here. Each citation in the frontmatter pins a chunk
by id and hash; the linter rejects citations whose chunk_id does not exist
in the chunk manifest or whose chunk_hash does not match the manifest entry.

Refer to docs/lint-codes.md for the full diagnostic-code catalogue.
";

/// Render a frontmatter dictionary + body into the canonical
/// card byte form. Byte-equal with Python
/// `frontmatter.render_card_text`:
///   - Frontmatter keys emitted in `CARD_FRONTMATTER_KEY_ORDER`,
///     null + absent keys dropped.
///   - `tags` and `card_edges` are additive Phase-3 fields:
///     emitted only when non-empty (so pre-Phase-3 cards
///     re-emit byte-identically).
///   - `citations` and `card_edges` use block-style YAML;
///     `page_range` uses flow-style `[lo, hi]`; `tags` uses
///     flow-style `["a", "b"]`.
///   - Strings are double-quoted with `\\`, `\"`, `\n` escapes.
///   - The block is wrapped in `---\n` / `\n---\n` and the
///     body is appended verbatim.
#[must_use]
pub fn render_card_text(frontmatter: &Value, body: &str) -> String {
    let fm = frontmatter
        .as_object()
        .expect("render_card_text frontmatter must be a JSON object");
    let mut lines: Vec<String> = vec!["---".to_owned()];
    for key in CARD_FRONTMATTER_KEY_ORDER {
        let Some(value) = fm.get(*key) else { continue };
        if value.is_null() {
            continue;
        }
        match *key {
            "citations" => {
                lines.push("citations:".to_owned());
                let arr = value.as_array().expect("citations must be an array");
                for cit in arr {
                    lines.extend(render_citation_block(cit));
                }
            }
            "card_edges" => {
                let arr = value.as_array().expect("card_edges must be an array");
                if arr.is_empty() {
                    continue;
                }
                lines.push("card_edges:".to_owned());
                for edge in arr {
                    lines.extend(render_card_edge_block(edge));
                }
            }
            "tags" => {
                let arr = value.as_array().expect("tags must be an array");
                if arr.is_empty() {
                    continue;
                }
                lines.push(format!("tags: {}", render_string_list(arr)));
            }
            _ => {
                lines.push(format!("{key}: {}", render_scalar(value)));
            }
        }
    }
    lines.push("---".to_owned());
    let mut out = lines.join("\n");
    out.push('\n');
    out.push_str(body);
    out
}

/// Render the canonical card-scaffold template for `kb new`.
/// Byte-equal with Python `card_template.build_card_text`.
/// The single placeholder citation uses a zero-`chunk_hash` so
/// the linter rejects the freshly-created card until the
/// author fills in real values — the deliberate tripwire.
#[must_use]
pub fn build_card_text(card_id: &str, title: &str, reading_id: &str) -> String {
    let frontmatter = json!({
        "schema_version": SCHEMA_VERSION,
        "id": card_id,
        "title": title,
        "reading_id": reading_id,
        "summary": "",
        "citations": [
            {
                "source_id": "REPLACE_WITH_SOURCE_ID",
                "chunk_id": "REPLACE_WITH_SOURCE_ID:p001:0000",
                "chunk_hash": "0".repeat(64),
                "page_range": [1, 1],
                "quote": "Replace with a quote that appears in the cited chunk.",
                "edge_type": "supports",
            }
        ],
    });
    render_card_text(&frontmatter, DEFAULT_BODY)
}

fn render_citation_block(cit: &Value) -> Vec<String> {
    let cit = cit
        .as_object()
        .expect("citation entry must be a JSON object");
    let mut lines: Vec<String> = Vec::new();
    let mut first = true;
    for key in CITATION_KEY_ORDER {
        let Some(v) = cit.get(*key) else { continue };
        let rendered = if *key == "page_range" {
            let pr = v.as_array().expect("page_range must be a 2-element array");
            let lo = pr[0].as_i64().expect("page_range[0] int");
            let hi = pr[1].as_i64().expect("page_range[1] int");
            format!("[{lo}, {hi}]")
        } else {
            render_scalar(v)
        };
        let prefix = if first { "  - " } else { "    " };
        lines.push(format!("{prefix}{key}: {rendered}"));
        first = false;
    }
    lines
}

fn render_card_edge_block(edge: &Value) -> Vec<String> {
    let edge = edge
        .as_object()
        .expect("card_edges entry must be a JSON object");
    let mut lines: Vec<String> = Vec::new();
    let mut first = true;
    for key in CARD_EDGE_KEY_ORDER {
        let Some(v) = edge.get(*key) else { continue };
        let prefix = if first { "  - " } else { "    " };
        lines.push(format!("{prefix}{key}: {}", render_scalar(v)));
        first = false;
    }
    lines
}

fn render_string_list(values: &[Value]) -> String {
    let mut out = String::from("[");
    for (i, v) in values.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&render_scalar(v));
    }
    out.push(']');
    out
}

/// Deterministic scalar rendering byte-equal with Python
/// `frontmatter._render_scalar`. Strings are double-quoted with
/// `\\`, `\"`, `\n` escapes (in that REPLACE order — backslash
/// first so the escapes for `"` and `\n` aren't double-escaped).
/// Ints / bools follow Python's `str()` conventions.
///
/// **Float caveat (latent)**: Python `repr(float)` uses
/// shortest round-trip with a 2-digit minimum exponent
/// (e.g. `1e-07`); serde_json's `Number::to_string` uses
/// Rust's `<f64 as Display>` which uses a 1-digit exponent
/// (`1e-7`). The cacg.v0 schema carries no floats today so
/// this is unreachable; if a future schema additive
/// introduces a float field, this branch needs a manual
/// Python-shape formatter. Round-28 review P2-3.
///
/// **Variant fallback (panic, not silent divergence)**:
/// Python `_render_scalar`'s catch-all is `f'"{str(value)}"'`
/// — a defensive path that for `None` produces `'"None"'`,
/// for `[1,2]` produces `'"[1, 2]"'`, etc. Rust's serde_json
/// `Display` of `Value::Null` is `null` and of arrays/objects
/// is compact JSON (`[1,2]`), both byte-divergent from
/// Python's `str(...)`. Today every cacg.v0 frontmatter field
/// is a typed string/int/bool/array/object that takes a
/// dedicated arm above; the catch-all is unreachable.
/// Panicking here (rather than emitting silently-divergent
/// bytes) ensures a future schema additive that lets a null
/// or unstructured value slip through fails LOUDLY at
/// authoring time instead of silently breaking parity in
/// production. Round-28 review P2-2.
#[allow(clippy::wildcard_enum_match_arm)]
fn render_scalar(value: &Value) -> String {
    match value {
        Value::String(s) => {
            let escaped = s
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n");
            format!("\"{escaped}\"")
        }
        Value::Bool(b) => if *b { "true" } else { "false" }.to_owned(),
        Value::Number(n) => n.to_string(),
        Value::Null | Value::Array(_) | Value::Object(_) => panic!(
            "card_template::render_scalar reached the defensive fallback for \
             a {value_type} value ({value}); the cacg.v0 schema must take a \
             typed arm above. Either add a dedicated branch (preferred) or, \
             if this really IS dead code, replace the panic with the \
             byte-equal Python `f'\"{{str(value)}}\"'` rendering for the \
             specific variant.",
            value_type = match value {
                Value::Null => "null",
                Value::Array(_) => "array",
                Value::Object(_) => "object",
                _ => unreachable!(),
            },
        ),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn scalar_string_escapes_backslash_quote_and_newline() {
        let v = Value::String("a\\b\"c\nd".to_owned());
        assert_eq!(render_scalar(&v), "\"a\\\\b\\\"c\\nd\"");
    }

    #[test]
    fn scalar_int_emits_as_decimal() {
        assert_eq!(render_scalar(&json!(42)), "42");
        assert_eq!(render_scalar(&json!(-7)), "-7");
    }

    #[test]
    fn scalar_bool_emits_lowercase_yaml_literals() {
        assert_eq!(render_scalar(&json!(true)), "true");
        assert_eq!(render_scalar(&json!(false)), "false");
    }

    #[test]
    fn build_card_text_emits_expected_lines() {
        let text = build_card_text("my-card", "My Card", "reading_01");
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines[0], "---");
        assert!(lines.contains(&"schema_version: \"cacg.v0\""));
        assert!(lines.contains(&"id: \"my-card\""));
        assert!(lines.contains(&"title: \"My Card\""));
        assert!(lines.contains(&"reading_id: \"reading_01\""));
        assert!(lines.contains(&"summary: \"\""));
        assert!(lines.contains(&"citations:"));
        assert!(text.contains("Write the card body here."));
    }

    #[test]
    fn build_card_text_zero_chunk_hash_is_64_hex_zeros() {
        let text = build_card_text("c", "t", "r");
        let zero64 = "0".repeat(64);
        let needle = format!("chunk_hash: \"{zero64}\"");
        assert!(text.contains(&needle), "expected {needle:?} in output");
    }

    #[test]
    fn build_card_text_omits_empty_tags_and_card_edges() {
        let text = build_card_text("c", "t", "r");
        assert!(!text.contains("tags:"));
        assert!(!text.contains("card_edges:"));
    }

    #[test]
    fn render_card_text_emits_tags_when_non_empty() {
        let fm = json!({
            "schema_version": "cacg.v0",
            "id": "c",
            "title": "t",
            "reading_id": "r",
            "summary": "s",
            "tags": ["alpha", "beta"],
            "citations": [],
        });
        let text = render_card_text(&fm, "body\n");
        assert!(text.contains("tags: [\"alpha\", \"beta\"]"));
    }

    #[test]
    fn render_card_text_emits_card_edges_when_non_empty() {
        let fm = json!({
            "schema_version": "cacg.v0",
            "id": "c",
            "title": "t",
            "reading_id": "r",
            "summary": "s",
            "card_edges": [
                {"target": "other-card", "edge_type": "extends"},
            ],
            "citations": [],
        });
        let text = render_card_text(&fm, "body\n");
        assert!(text.contains("card_edges:"));
        assert!(text.contains("  - target: \"other-card\""));
        assert!(text.contains("    edge_type: \"extends\""));
    }

    #[test]
    fn render_card_text_emits_page_range_in_flow_style() {
        let fm = json!({
            "schema_version": "cacg.v0",
            "id": "c",
            "title": "t",
            "reading_id": "r",
            "summary": "s",
            "citations": [
                {
                    "source_id": "src",
                    "chunk_id": "src:p001:0000",
                    "chunk_hash": "0".repeat(64),
                    "page_range": [3, 7],
                    "quote": "q",
                    "edge_type": "supports",
                }
            ],
        });
        let text = render_card_text(&fm, "body\n");
        assert!(text.contains("    page_range: [3, 7]"));
    }

    #[test]
    fn render_card_text_drops_null_card_hash() {
        let mut fm = json!({
            "schema_version": "cacg.v0",
            "id": "c",
            "title": "t",
            "reading_id": "r",
            "summary": "s",
            "citations": [],
            "card_hash": null,
        });
        let text = render_card_text(&fm, "body\n");
        assert!(!text.contains("card_hash"));
        // And the absent vs explicit-null shapes are equivalent.
        fm.as_object_mut().unwrap().remove("card_hash");
        let text2 = render_card_text(&fm, "body\n");
        assert_eq!(text, text2);
    }

    #[test]
    fn render_card_text_emits_keys_in_canonical_order() {
        // schema_version, id, title, reading_id, summary, tags,
        // card_edges, citations, card_hash. Construct in REVERSE
        // order in the input dict so the test proves the writer's
        // key order is independent of the dict's insertion order.
        let fm = json!({
            "card_hash": "0".repeat(64),
            "citations": [{
                "source_id": "s", "chunk_id": "s:p001:0000",
                "chunk_hash": "0".repeat(64), "page_range": [1, 1],
                "quote": "q", "edge_type": "supports",
            }],
            "tags": ["t"],
            "summary": "summary text",
            "reading_id": "r",
            "title": "T",
            "id": "id",
            "schema_version": "cacg.v0",
        });
        let text = render_card_text(&fm, "body\n");
        let lines: Vec<&str> = text.lines().collect();
        // Find the index of each emitted key and assert the
        // sequence matches CARD_FRONTMATTER_KEY_ORDER.
        let expected_seq = [
            "schema_version:",
            "id:",
            "title:",
            "reading_id:",
            "summary:",
            "tags:",
            "citations:",
            "card_hash:",
        ];
        let mut last_idx: i32 = -1;
        for needle in &expected_seq {
            let idx = lines
                .iter()
                .position(|l| l.starts_with(needle))
                .unwrap_or_else(|| panic!("missing key {needle}"));
            assert!(
                (idx as i32) > last_idx,
                "key {needle} appears out of canonical order"
            );
            last_idx = idx as i32;
        }
    }
}
