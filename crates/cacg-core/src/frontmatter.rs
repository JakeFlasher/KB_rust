//! Card-frontmatter parser mirroring Python `legacy_python_oracle/src/cacg/frontmatter.py::parse_card`.
//!
//! Five-step pipeline:
//! 1. Regex-split the card text via `_FRONTMATTER_RE`. Missing delimiters
//!    → CACG-FM-004.
//! 2. yaml-rust2 event prescan over the frontmatter text: reject
//!    anchors / aliases / explicit tags. Mirrors Python
//!    `_NoDuplicateKeysSafeLoader::compose_node`. Returns CACG-FM-005 on
//!    forbidden construct; CACG-FM-006 on other YAML parse errors.
//! 3. serde_yaml_bw deserialize into `CardFrontmatter`. The
//!    `DuplicateKeyStrategy::Error` default already catches duplicate
//!    keys (CACG-FM-006). `deny_unknown_fields` catches typos
//!    (CACG-FM-003). Missing required fields → CACG-FM-001.
//!    schema_version literal mismatch → CACG-FM-002. Root not a mapping
//!    → CACG-FM-007. Other validation → CACG-FM-008.
//! 4. Cross-field validation via `CardFrontmatter::validate` (chunk_hash
//!    64-hex / page_range / quote-after-normalize / summary length /
//!    tags / card_edges duplicate-pair).
//! 5. Return `(frontmatter, body)` or `Vec<Diagnostic>`.

use std::sync::OnceLock;

use regex::Regex;
use yaml_rust2::parser::{Event, Parser};
use yaml_rust2::yaml::Yaml;
use yaml_rust2::YamlLoader;

use crate::diagnostic::{codes, Diagnostic, Severity};
use crate::schema::CardFrontmatter;

/// Frontmatter delimiter regex (matches Python `_FRONTMATTER_RE`).
fn frontmatter_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?s)\A---\s*\n(.*?)\n---\s*\n(.*)\z").expect("frontmatter regex is valid")
    })
}

/// Parse a card's full text into a validated `CardFrontmatter` + raw body.
///
/// Returns `Ok((frontmatter, body))` on success or `Err(diagnostics)` on
/// any parse / validation failure. The diagnostics carry CACG-* codes
/// matching Python `legacy_python_oracle/src/cacg/frontmatter.py::parse_card` byte-for-byte
/// per the HYBRID parity policy (code + severity + file always
/// byte-equal; message via whitelist).
///
/// # Errors
///
/// Returns a non-empty `Vec<Diagnostic>` when any step fails. The
/// caller (typically the kb CLI handler) walks the diagnostics and
/// prints them to stderr.
pub fn parse_card(text: &str) -> Result<(CardFrontmatter, String), Vec<Diagnostic>> {
    // Step 1: regex-split frontmatter from body.
    let Some(caps) = frontmatter_re().captures(text) else {
        return Err(vec![Diagnostic::new(
            codes::FM_004,
            Severity::Error,
            "card is missing YAML frontmatter delimited by '---'".to_string(),
        )]);
    };
    let fm_text = caps.get(1).map_or("", |m| m.as_str());
    let body = caps.get(2).map_or("", |m| m.as_str()).to_string();

    // Step 2: yaml-rust2 event prescan -- reject anchors / aliases /
    // explicit tags. serde_yaml_bw's `alias_limit=0` default catches
    // aliases too, but rejecting the upstream events keeps the
    // trust-boundary path explicit + maps to CACG-FM-005 cleanly.
    if let Err(d) = yaml_event_prescan(fm_text) {
        return Err(vec![d]);
    }

    // Step 2b (Round 16 / Codex R15-REVIEW-1): strict YAML scalar-tag
    // prescan. Pydantic with `field: str` rejects YAML scalars whose
    // tag is `!!int` / `!!bool` / `!!null` / `!!seq`. serde_yaml_bw
    // happily coerces ints / bools / null to string form, so without
    // this prescan the AC-C4 trust boundary lets type-mismatched
    // input flow through. We walk the parsed Yaml tree against the
    // known CardFrontmatter shape and reject scalar-type mismatches
    // with the documented CACG-* code Python's _map_pydantic_errors
    // would emit.
    if let Err(d) = strict_shape_check(fm_text) {
        return Err(vec![d]);
    }

    // Step 3: serde_yaml_bw deserialize.
    let de_result: Result<CardFrontmatter, serde_yaml_bw::Error> = serde_yaml_bw::from_str(fm_text);
    let frontmatter = match de_result {
        Ok(fm) => fm,
        Err(e) => {
            return Err(vec![map_serde_error(&e)]);
        }
    };

    // Step 4: cross-field validation.
    if let Err(d) = frontmatter.validate() {
        return Err(vec![d]);
    }

    Ok((frontmatter, body))
}

/// Walk yaml-rust2's event stream over the frontmatter text, rejecting
/// anchors / aliases / explicit (non-default) tags as Python's
/// `_NoDuplicateKeysSafeLoader::compose_node` override does.
#[allow(clippy::wildcard_enum_match_arm)]
fn yaml_event_prescan(fm_text: &str) -> Result<(), Diagnostic> {
    let mut parser = Parser::new_from_str(fm_text);
    loop {
        let (event, _marker) = match parser.next_token() {
            Ok(t) => t,
            Err(e) => {
                // yaml-rust2 errors with "unknown anchor" when an
                // alias `*foo` references an undefined anchor. Python
                // catches the alias at compose_node time with
                // CACG-FM-005; mirror that here.
                let emsg = e.to_string();
                if emsg.contains("unknown anchor") || emsg.contains("anchor") {
                    return Err(Diagnostic::new(
                        codes::FM_005,
                        Severity::Error,
                        format!("forbidden YAML construct: alias references unknown anchor: {e}"),
                    ));
                }
                return Err(Diagnostic::new(
                    codes::FM_006,
                    Severity::Error,
                    format!("YAML parse error: {e}"),
                ));
            }
        };
        match event {
            Event::StreamEnd => return Ok(()),
            Event::Alias(_) => {
                return Err(Diagnostic::new(
                    codes::FM_005,
                    Severity::Error,
                    "forbidden YAML construct: alias".to_string(),
                ));
            }
            Event::Scalar(_value, _style, anchor_id, ref tag) => {
                if anchor_id != 0 {
                    return Err(Diagnostic::new(
                        codes::FM_005,
                        Severity::Error,
                        "forbidden YAML construct: anchor on scalar".to_string(),
                    ));
                }
                if let Some(t) = tag {
                    if !is_default_yaml_tag(t) {
                        return Err(Diagnostic::new(
                            codes::FM_005,
                            Severity::Error,
                            format!(
                                "forbidden YAML construct: explicit tag {}{}",
                                t.handle, t.suffix
                            ),
                        ));
                    }
                }
            }
            Event::SequenceStart(anchor_id, ref tag) => {
                if anchor_id != 0 {
                    return Err(Diagnostic::new(
                        codes::FM_005,
                        Severity::Error,
                        "forbidden YAML construct: anchor on sequence".to_string(),
                    ));
                }
                if let Some(t) = tag {
                    if !is_default_yaml_tag(t) {
                        return Err(Diagnostic::new(
                            codes::FM_005,
                            Severity::Error,
                            format!(
                                "forbidden YAML construct: explicit tag {}{}",
                                t.handle, t.suffix
                            ),
                        ));
                    }
                }
            }
            Event::MappingStart(anchor_id, ref tag) => {
                if anchor_id != 0 {
                    return Err(Diagnostic::new(
                        codes::FM_005,
                        Severity::Error,
                        "forbidden YAML construct: anchor on mapping".to_string(),
                    ));
                }
                if let Some(t) = tag {
                    if !is_default_yaml_tag(t) {
                        return Err(Diagnostic::new(
                            codes::FM_005,
                            Severity::Error,
                            format!(
                                "forbidden YAML construct: explicit tag {}{}",
                                t.handle, t.suffix
                            ),
                        ));
                    }
                }
            }
            _ => {}
        }
    }
}

/// Walk the parsed Yaml document tree against the CardFrontmatter
/// shape, rejecting scalar-type mismatches at the YAML layer before
/// serde_yaml_bw can coerce them. Mirrors Pydantic's strict-mode
/// scalar checks for `field: str` (rejects `!!int`/`!!bool`/`!!null`),
/// `tags: list[str]` (rejects non-string elements + non-sequence
/// containers), and `page_range: tuple[int, int]` (rejects non-integer
/// elements + single-element lists per Python `tuple_size` -> FM-001).
#[allow(clippy::wildcard_enum_match_arm, clippy::excessive_nesting)]
fn strict_shape_check(fm_text: &str) -> Result<(), Diagnostic> {
    let docs = match YamlLoader::load_from_str(fm_text) {
        Ok(d) => d,
        Err(_) => {
            // yaml-rust2 itself rejected -- defer to event prescan's
            // already-emitted diagnostic (we don't reach here in
            // practice because event_prescan ran first).
            return Ok(());
        }
    };
    let Some(doc) = docs.first() else {
        return Ok(());
    };
    let Yaml::Hash(ref root) = doc else {
        // Root is not a mapping -- serde_yaml_bw will emit the
        // documented FM-007 path; nothing to add here.
        return Ok(());
    };

    // String-typed root fields. Reject any non-String YAML node.
    // Python Pydantic: `id: str = Field(min_length=1)` etc. rejects
    // ints/bools/null/arrays. We map these to CACG-FM-008 per
    // _map_pydantic_errors' catch-all (Python's `etype` for these is
    // "string_type" which is NOT one of the specific branches).
    for field in &["schema_version", "id", "title", "reading_id"] {
        if let Some(value) = root.get(&Yaml::String((*field).to_string())) {
            if !matches!(value, Yaml::String(_)) {
                return Err(Diagnostic::new(
                    codes::FM_008,
                    Severity::Error,
                    format!("validation error at {field}: Input should be a valid string"),
                ));
            }
        }
    }

    // `summary` MUST be a string; non-string maps to FM-008 (NOT
    // SUM-001/SUM-002 -- those are length-bound codes for an actual
    // string value).
    if let Some(value) = root.get(&Yaml::String("summary".to_string())) {
        if !matches!(value, Yaml::String(_)) {
            return Err(Diagnostic::new(
                codes::FM_008,
                Severity::Error,
                "validation error at summary: Input should be a valid string".to_string(),
            ));
        }
    }

    // `card_hash`: String OR Null. Anything else is FM-008.
    if let Some(value) = root.get(&Yaml::String("card_hash".to_string())) {
        if !matches!(value, Yaml::String(_) | Yaml::Null) {
            return Err(Diagnostic::new(
                codes::FM_008,
                Severity::Error,
                "validation error at card_hash: Input should be a valid string or null".to_string(),
            ));
        }
    }

    // `tags`: Array of String elements. Non-array container -> SUM-003.
    // Non-string element -> SUM-003 (Python `_map_pydantic_errors`
    // routes tags-context errors to SUM-003 unless the message mentions
    // a max-entries violation, in which case SUM-004).
    if let Some(value) = root.get(&Yaml::String("tags".to_string())) {
        match value {
            Yaml::Array(items) => {
                for (idx, item) in items.iter().enumerate() {
                    if !matches!(item, Yaml::String(_)) {
                        return Err(Diagnostic::new(
                            codes::SUM_003,
                            Severity::Error,
                            format!(
                                "tag non-conforming: tags[{idx}] must be a string (got {})",
                                yaml_type_name(item)
                            ),
                        ));
                    }
                }
            }
            _ => {
                return Err(Diagnostic::new(
                    codes::SUM_003,
                    Severity::Error,
                    format!(
                        "tag non-conforming: tags must be a sequence (got {})",
                        yaml_type_name(value)
                    ),
                ));
            }
        }
    }

    // `card_edges`: Array of Hash elements. Non-array -> FM-008.
    if let Some(value) = root.get(&Yaml::String("card_edges".to_string())) {
        match value {
            Yaml::Array(items) => {
                for (idx, item) in items.iter().enumerate() {
                    let Yaml::Hash(edge) = item else {
                        return Err(Diagnostic::new(
                            codes::FM_008,
                            Severity::Error,
                            format!(
                                "validation error at card_edges.{idx}: Input should be a mapping (got {})",
                                yaml_type_name(item)
                            ),
                        ));
                    };
                    for sf in &["target", "edge_type"] {
                        if let Some(v) = edge.get(&Yaml::String((*sf).to_string())) {
                            // qg-allow: deep-nesting — YAML edge validation
                            if !matches!(v, Yaml::String(_)) {
                                // qg-allow: deep-nesting — YAML edge type check
                                return Err(Diagnostic::new(
                                    codes::FM_008,
                                    Severity::Error,
                                    format!(
                                        "validation error at card_edges.{idx}.{sf}: Input should be a valid string"
                                    ),
                                ));
                            }
                        }
                    }
                }
            }
            _ => {
                return Err(Diagnostic::new(
                    codes::FM_008,
                    Severity::Error,
                    format!(
                        "validation error at card_edges: Input should be a sequence (got {})",
                        yaml_type_name(value)
                    ),
                ));
            }
        }
    }

    // `citations`: Array of Hash elements. Each citation's string
    // fields must be String; `page_range` must be exactly 2 integers.
    if let Some(value) = root.get(&Yaml::String("citations".to_string())) {
        let Yaml::Array(items) = value else {
            return Err(Diagnostic::new(
                codes::FM_008,
                Severity::Error,
                format!(
                    "validation error at citations: Input should be a sequence (got {})",
                    yaml_type_name(value)
                ),
            ));
        };
        for (idx, item) in items.iter().enumerate() {
            let Yaml::Hash(cit) = item else {
                return Err(Diagnostic::new(
                    codes::FM_008,
                    Severity::Error,
                    format!(
                        "validation error at citations.{idx}: Input should be a mapping (got {})",
                        yaml_type_name(item)
                    ),
                ));
            };
            for sf in &["source_id", "chunk_id", "chunk_hash", "quote", "edge_type"] {
                if let Some(v) = cit.get(&Yaml::String((*sf).to_string())) {
                    if !matches!(v, Yaml::String(_)) {
                        return Err(Diagnostic::new(
                            codes::FM_008,
                            Severity::Error,
                            format!(
                                "validation error at citations.{idx}.{sf}: Input should be a valid string"
                            ),
                        ));
                    }
                }
            }
            // page_range: must be Array of EXACTLY 2 Integer elements.
            if let Some(pr) = cit.get(&Yaml::String("page_range".to_string())) {
                let Yaml::Array(items) = pr else {
                    return Err(Diagnostic::new(
                        codes::CITE_003,
                        Severity::Error,
                        format!(
                            "citations.{idx}.page_range: page_range must be ascending start<=end (Value error, page_range must be a list or tuple; got {})",
                            yaml_type_name(pr)
                        ),
                    ));
                };
                if items.len() == 1 {
                    // Python's Pydantic `tuple[int, int]` for `[1]`
                    // emits `etype == "missing"` -> mapped to FM-001
                    // ("missing required field"). We mirror that here.
                    return Err(Diagnostic::new(
                        codes::FM_001,
                        Severity::Error,
                        format!("missing required field: citations.{idx}.page_range.1"),
                    ));
                }
                if items.len() != 2 {
                    return Err(Diagnostic::new(
                        codes::CITE_003,
                        Severity::Error,
                        format!(
                            "citations.{idx}.page_range: page_range must be ascending start<=end (Value error, page_range must have exactly 2 elements; got {})",
                            items.len()
                        ),
                    ));
                }
                for (ei, e) in items.iter().enumerate() {
                    if !matches!(e, Yaml::Integer(_)) {
                        return Err(Diagnostic::new(
                            codes::CITE_003,
                            Severity::Error,
                            format!(
                                "citations.{idx}.page_range: page_range must be ascending start<=end (Value error, page_range[{ei}] must be a strict integer (not bool, not str, not float); got {})",
                                yaml_type_name(e)
                            ),
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}

/// Human-readable name for a Yaml node variant, used in diagnostic messages.
fn yaml_type_name(y: &Yaml) -> &'static str {
    match y {
        Yaml::Real(_) => "float",
        Yaml::Integer(_) => "int",
        Yaml::String(_) => "str",
        Yaml::Boolean(_) => "bool",
        Yaml::Array(_) => "sequence",
        Yaml::Hash(_) => "mapping",
        Yaml::Alias(_) => "alias",
        Yaml::Null => "null",
        Yaml::BadValue => "bad-value",
    }
}

fn is_default_yaml_tag(tag: &yaml_rust2::parser::Tag) -> bool {
    // Default YAML tags use the `tag:yaml.org,2002:` prefix; libyaml
    // emits these implicitly for primitives. Only EXPLICIT tags
    // (`!CustomTag` style) carry a non-default handle/suffix.
    tag.handle.starts_with("tag:yaml.org,2002:") || tag.handle.is_empty() && tag.suffix.is_empty()
}

/// Map a serde_yaml_bw error to the documented CACG-FM-* / CACG-CITE-* /
/// CACG-SUM-* diagnostic. Mirrors Python `_map_pydantic_errors`.
fn map_serde_error(e: &serde_yaml_bw::Error) -> Diagnostic {
    let msg = e.to_string();
    // serde_yaml_bw's error messages follow patterns:
    //   - "missing field `X` at line L column C"
    //   - "unknown field `X`, expected one of ... at line L column C"
    //   - "unknown variant `X`, expected one of `...` at ..." (literal mismatch)
    //   - "duplicate key `X` at ..."
    //   - "invalid type: expected <T>, found <U> at ..."
    if msg.starts_with("missing field") || msg.contains("missing field") {
        // Extract field name from `missing field \`X\``.
        let field = extract_backtick(&msg).unwrap_or_else(|| "?".to_string());
        return Diagnostic::new(
            codes::FM_001,
            Severity::Error,
            format!("missing required field: {field}"),
        );
    }
    if msg.starts_with("unknown field") || msg.contains("unknown field") {
        let field = extract_backtick(&msg).unwrap_or_else(|| "?".to_string());
        return Diagnostic::new(
            codes::FM_003,
            Severity::Error,
            format!("unknown frontmatter field: {field}"),
        );
    }
    if msg.contains("duplicate key") || msg.contains("duplicate entry") {
        return Diagnostic::new(
            codes::FM_006,
            Severity::Error,
            format!("YAML parse error: {msg}"),
        );
    }
    // page_range field path: any "invalid type ... expected i64" error
    // surfaces from page_range deserialization (the only i64-list field
    // in CardFrontmatter). Python's mapper labels these CACG-CITE-003.
    if msg.contains("expected i64") {
        return Diagnostic::new(
            codes::CITE_003,
            Severity::Error,
            format!("citations.?.page_range: page_range must be ascending start<=end ({msg})"),
        );
    }
    // tags-context errors: "expected a sequence" coming from a tags:
    // string YAML node, OR "invalid type" for tag element strings.
    // Python's mapper labels these CACG-SUM-003.
    if msg.contains("expected a sequence") && msg.to_lowercase().contains("tag") {
        return Diagnostic::new(
            codes::SUM_003,
            Severity::Error,
            format!("tag non-conforming: {msg}"),
        );
    }
    if msg.contains("unknown variant") {
        // schema_version literal mismatch falls here. Disambiguate by
        // looking for "schema_version" in the message context (serde
        // includes a "for variant `X`" or "expected one of" suffix that
        // names the field path via deserialize state). The simplest
        // heuristic: if the error mentions schema_version, FM_002;
        // otherwise FM_008.
        if msg.contains("schema_version") || msg.contains("cacg.v0") {
            return Diagnostic::new(
                codes::FM_002,
                Severity::Error,
                format!("unknown schema_version (must be 'cacg.v0'): {msg}"),
            );
        }
        return Diagnostic::new(
            codes::FM_008,
            Severity::Error,
            format!("validation error: {msg}"),
        );
    }
    // FM-007 fires ONLY for root-level shape errors -- a top-level
    // sequence or scalar instead of a mapping. The serde error message
    // for that case mentions the root struct name `CardFrontmatter`.
    // Other "invalid type" errors at nested locations (e.g.,
    // `tags: "not-a-list"`) fall through to the FM-008 catch-all.
    if msg.contains("invalid type") && msg.contains("expected struct CardFrontmatter") {
        return Diagnostic::new(
            codes::FM_007,
            Severity::Error,
            format!("frontmatter root is not a mapping: {msg}"),
        );
    }
    // Catch-all.
    Diagnostic::new(
        codes::FM_008,
        Severity::Error,
        format!("validation error: {msg}"),
    )
}

fn extract_backtick(msg: &str) -> Option<String> {
    let start = msg.find('`')?;
    let rest = &msg[start + 1..];
    let end = rest.find('`')?;
    Some(rest[..end].to_string())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn missing_delimiters_emits_fm_004() {
        let card = "no frontmatter here\nbody\n";
        let result = parse_card(card);
        assert!(result.is_err());
        let diags = result.unwrap_err();
        assert_eq!(diags[0].code, codes::FM_004);
    }

    #[test]
    fn valid_minimal_card_parses() {
        let card = r#"---
schema_version: "cacg.v0"
id: "sample-card"
title: "Sample Card"
reading_id: "reading_01"
summary: "This summary is at least eighty characters long to satisfy the AC-S1 bounded summary lower bound."
citations:
  - source_id: "sample"
    chunk_id: "sample:p001:0000"
    chunk_hash: "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895"
    page_range: [1, 2]
    quote: "some quoted text"
    edge_type: "supports"
---
Body text.
"#;
        let result = parse_card(card);
        assert!(result.is_ok(), "valid card must parse: {result:?}");
        let (fm, body) = result.unwrap();
        assert_eq!(fm.id, "sample-card");
        assert_eq!(body, "Body text.\n");
    }

    #[test]
    fn unknown_field_emits_fm_003() {
        let card = r#"---
schema_version: "cacg.v0"
id: "x"
title: "x"
reading_id: "x"
summary: "This summary is at least eighty characters long to satisfy the AC-S1 bounded summary lower bound."
unknown_field: "oops"
citations:
  - source_id: "s"
    chunk_id: "s:p001:0000"
    chunk_hash: "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895"
    page_range: [1, 2]
    quote: "q"
    edge_type: "supports"
---
Body
"#;
        let result = parse_card(card);
        assert!(result.is_err());
        let diags = result.unwrap_err();
        assert_eq!(diags[0].code, codes::FM_003);
    }
}
