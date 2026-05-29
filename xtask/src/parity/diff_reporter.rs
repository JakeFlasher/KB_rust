//! Structured field-level diff reporter for the parity harness.
//!
//! Replaces the legacy byte-offset-only diagnostic. The byte-offset
//! summary is preserved on `ArtifactComparison::diff_summary` for
//! schema back-compat; this module produces the structured payload
//! that lives on `ArtifactComparison::diff_detail`.
//!
//! Two layers report independently:
//! 1. A byte-equality check always runs first. If bytes match, the
//!    reporter has nothing to add.
//! 2. When bytes differ, the artifact class is inferred from the
//!    file-name suffix (`.json`, `.jsonl`, otherwise UTF-8 text or
//!    binary). The JSON layer walks paired `serde_json::Value`s by
//!    sorted-key traversal and reports the first differing path with
//!    each side's emitted value. JSONL is line-split first, then the
//!    differing line's JSON content is walked. Non-JSON UTF-8 falls
//!    through to a unified text diff; non-UTF-8 binary keeps only the
//!    byte-offset summary.
//!
//! The reporter NEVER propagates a parse failure. A `.json` artifact
//! whose bytes are not valid JSON falls back to text (if UTF-8) or
//! binary (if not), so the harness gate never crashes on an invalid
//! upstream artifact -- it merely reports it as text or binary.

use serde::Serialize;
use serde_json::Value;

/// Upper bound on lines emitted in a single `unified_diff` snippet.
/// Reached lines are followed by the literal marker `... (truncated)`
/// so a reader knows the snippet was capped, not naturally short.
const UNIFIED_DIFF_LINE_CAP: usize = 32;

/// Path segment within a JSON walk. Object keys carry the key name;
/// array elements carry the index. The rendered form is
/// `cards[3].title` (no dot before `[`, dot before subsequent keys),
/// or the literal `<root>` for top-level scalar mismatches.
#[derive(Debug, Clone, PartialEq)]
enum PathSegment {
    Key(String),
    Index(usize),
}

fn render_path(segments: &[PathSegment]) -> String {
    if segments.is_empty() {
        return "<root>".to_string();
    }
    let mut s = String::new();
    for (i, seg) in segments.iter().enumerate() {
        match seg {
            PathSegment::Key(k) => {
                if i > 0 {
                    s.push('.');
                }
                s.push_str(k);
            }
            PathSegment::Index(idx) => {
                s.push('[');
                s.push_str(&idx.to_string());
                s.push(']');
            }
        }
    }
    s
}

/// Structured diff payload. Variants carry the artifact-class kind
/// plus the smallest enough information for a reviewer to act on the
/// diff without rerunning the harness.
///
/// The enum is `#[serde(tag = "kind", rename_all = "snake_case")]` so
/// the JSON shape in the perf report is documented and stable. Every
/// byte-different non-missing variant carries the comparison's
/// `artifact_path` so a reader of `diff_detail` alone (without the
/// surrounding `ArtifactComparison` sibling fields) can identify
/// which artifact the structured payload describes.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StructuredDiff {
    /// Both artifact byte streams are byte-equal. Callers typically
    /// elide this and store `None` in `diff_detail` instead, but the
    /// variant exists so `field_level_diff` is total over its input.
    ByteEqual,
    /// JSON artifact, first differing field. `path` is rendered via
    /// `render_path`; `py_value` / `rs_value` are the parsed values
    /// at that path; `unified_diff` is a pretty-printed `diff -u`
    /// snippet over the whole document; `byte_offset` preserves the
    /// first-divergence offset that the legacy `diff_summary` string
    /// also records, so the two layers can be cross-referenced.
    JsonField {
        artifact_path: String,
        path: String,
        py_value: Value,
        rs_value: Value,
        unified_diff: String,
        byte_offset: usize,
    },
    /// JSONL artifact, first differing line (1-indexed). When the
    /// line parses on both sides, `path` and `py_value` / `rs_value`
    /// resolve within that line's JSON; otherwise `path` is
    /// `<unparseable_line>` and the values are string-wrapped.
    JsonlLine {
        artifact_path: String,
        line: usize,
        path: String,
        py_value: Value,
        rs_value: Value,
        unified_diff: String,
    },
    /// UTF-8 text artifact that is not JSON or JSONL.
    Text {
        artifact_path: String,
        unified_diff: String,
        byte_offset: usize,
    },
    /// Non-UTF-8 binary artifact. Only byte-offset + per-side length
    /// is meaningful; the bytes themselves are not rendered.
    Binary {
        artifact_path: String,
        byte_offset: usize,
        py_len: usize,
        rs_len: usize,
    },
    /// Bytes differ but both sides parse to equal JSON values. The
    /// drift is presentation-level only (sort order, whitespace,
    /// numeric form). Byte-offset is preserved so a reviewer can
    /// locate the divergence without re-parsing.
    SemanticEqualButBytesDiffer {
        artifact_path: String,
        byte_offset: usize,
        py_len: usize,
        rs_len: usize,
    },
    /// Expected artifact missing on disk; only the Rust path is present.
    MissingExpected { rust_path: String },
    /// Rust artifact missing on disk; only the expected path is present.
    MissingRust { expected_path: String },
    /// Both sides missing on disk -- usually means a harness setup
    /// failure rather than a parity gap.
    BothMissing {
        expected_path: String,
        rust_path: String,
    },
}

/// Top-level entry: produce a `StructuredDiff` for two in-memory
/// artifact byte streams. The function does not perform any I/O and
/// does not panic on malformed JSON -- invalid `.json` / `.jsonl`
/// bytes fall back to text or binary handling. The `name` argument is
/// recorded as `artifact_path` on every non-missing byte-different
/// variant so a reader of `diff_detail` alone (without the surrounding
/// `ArtifactComparison` sibling fields) can identify the artifact.
pub fn field_level_diff(name: &str, py_bytes: &[u8], rs_bytes: &[u8]) -> StructuredDiff {
    if py_bytes == rs_bytes {
        return StructuredDiff::ByteEqual;
    }
    if name.ends_with(".json") {
        diff_json(name, py_bytes, rs_bytes)
    } else if name.ends_with(".jsonl") {
        diff_jsonl(name, py_bytes, rs_bytes)
    } else {
        diff_text_or_binary(name, py_bytes, rs_bytes)
    }
}

/// First byte offset at which `py` and `rs` disagree. If one is a
/// strict prefix of the other, this returns the prefix length. If
/// both are empty, returns `0`.
fn first_byte_diff_offset(py: &[u8], rs: &[u8]) -> usize {
    py.iter()
        .zip(rs.iter())
        .position(|(a, b)| a != b)
        .unwrap_or_else(|| py.len().min(rs.len()))
}

fn diff_json(name: &str, py_bytes: &[u8], rs_bytes: &[u8]) -> StructuredDiff {
    let byte_offset = first_byte_diff_offset(py_bytes, rs_bytes);
    let py_parse: Result<Value, _> = serde_json::from_slice(py_bytes);
    let rs_parse: Result<Value, _> = serde_json::from_slice(rs_bytes);
    match (py_parse, rs_parse) {
        (Ok(py_val), Ok(rs_val)) if py_val == rs_val => {
            StructuredDiff::SemanticEqualButBytesDiffer {
                artifact_path: name.to_string(),
                byte_offset,
                py_len: py_bytes.len(),
                rs_len: rs_bytes.len(),
            }
        }
        (Ok(py_val), Ok(rs_val)) => {
            let mut segments: Vec<PathSegment> = Vec::new();
            let (py_at, rs_at) = walk_first_diff(&py_val, &rs_val, &mut segments);
            let py_pretty = serde_json::to_string_pretty(&py_val).unwrap_or_default();
            let rs_pretty = serde_json::to_string_pretty(&rs_val).unwrap_or_default();
            let unified_diff = unified_line_diff(&py_pretty, &rs_pretty);
            StructuredDiff::JsonField {
                artifact_path: name.to_string(),
                path: render_path(&segments),
                py_value: py_at,
                rs_value: rs_at,
                unified_diff,
                byte_offset,
            }
        }
        _ => diff_text_or_binary(name, py_bytes, rs_bytes),
    }
}

fn diff_jsonl(name: &str, py_bytes: &[u8], rs_bytes: &[u8]) -> StructuredDiff {
    let py_text = match std::str::from_utf8(py_bytes) {
        Ok(s) => s,
        Err(_) => return diff_text_or_binary(name, py_bytes, rs_bytes),
    };
    let rs_text = match std::str::from_utf8(rs_bytes) {
        Ok(s) => s,
        Err(_) => return diff_text_or_binary(name, py_bytes, rs_bytes),
    };
    let py_lines: Vec<&str> = py_text.lines().collect();
    let rs_lines: Vec<&str> = rs_text.lines().collect();
    let max = py_lines.len().max(rs_lines.len());
    for i in 0..max {
        let py_line = py_lines.get(i).copied();
        let rs_line = rs_lines.get(i).copied();
        if py_line == rs_line {
            continue;
        }
        let py_str = py_line.unwrap_or("");
        let rs_str = rs_line.unwrap_or("");
        let unified_diff = unified_line_diff(py_str, rs_str);
        let py_parse: Result<Value, _> = serde_json::from_str(py_str);
        let rs_parse: Result<Value, _> = serde_json::from_str(rs_str);
        return match (py_parse, rs_parse) {
            (Ok(py_val), Ok(rs_val)) => {
                let mut segments: Vec<PathSegment> = Vec::new();
                let (py_at, rs_at) = walk_first_diff(&py_val, &rs_val, &mut segments);
                StructuredDiff::JsonlLine {
                    artifact_path: name.to_string(),
                    line: i + 1,
                    path: render_path(&segments),
                    py_value: py_at,
                    rs_value: rs_at,
                    unified_diff,
                }
            }
            _ => StructuredDiff::JsonlLine {
                artifact_path: name.to_string(),
                line: i + 1,
                path: "<unparseable_line>".to_string(),
                py_value: Value::String(py_str.to_string()),
                rs_value: Value::String(rs_str.to_string()),
                unified_diff,
            },
        };
    }
    diff_text_or_binary(name, py_bytes, rs_bytes)
}

fn diff_text_or_binary(name: &str, py_bytes: &[u8], rs_bytes: &[u8]) -> StructuredDiff {
    let byte_offset = first_byte_diff_offset(py_bytes, rs_bytes);
    match (std::str::from_utf8(py_bytes), std::str::from_utf8(rs_bytes)) {
        (Ok(py_text), Ok(rs_text)) => StructuredDiff::Text {
            artifact_path: name.to_string(),
            unified_diff: unified_line_diff(py_text, rs_text),
            byte_offset,
        },
        _ => StructuredDiff::Binary {
            artifact_path: name.to_string(),
            byte_offset,
            py_len: py_bytes.len(),
            rs_len: rs_bytes.len(),
        },
    }
}

/// Walk paired JSON `Value` trees in canonical-JSON sort order (object
/// keys via the union of both sides' key sets in lexicographic order)
/// and return the first differing pair of values along with the path
/// reached. If a child object/array branch terminates before its
/// sibling at the same key (different lengths), the missing side is
/// reported as `Value::Null`. The caller threads the path vec through
/// recursion to avoid reallocation on the common pass.
fn walk_first_diff(py: &Value, rs: &Value, path: &mut Vec<PathSegment>) -> (Value, Value) {
    if py == rs {
        return (py.clone(), rs.clone());
    }
    match (py, rs) {
        (Value::Object(p_obj), Value::Object(r_obj)) => {
            // Union of keys in sorted order. serde_json's default
            // `Map<String, Value>` is a `BTreeMap`, so `.keys()` is
            // already alphabetical, but we still merge both sides
            // explicitly for the union -- a side-only key is a real
            // mismatch and must be detected.
            let mut keys: std::collections::BTreeSet<&String> = std::collections::BTreeSet::new();
            for k in p_obj.keys() {
                keys.insert(k);
            }
            for k in r_obj.keys() {
                keys.insert(k);
            }
            for k in keys {
                // Compare `Option<&Value>` so a key that's missing
                // on one side AND explicit `null` on the other side
                // is detected as a real divergence at this key
                // path (NOT collapsed to `Null == Null` and reported
                // at `<root>`). The four arms below handle:
                //   * present-both-equal -> continue
                //   * present-both-different -> recurse with the values
                //   * side-only: push the key path and return the
                //     actual value paired with `Value::Null` (the
                //     missing side renders as null at the leaf; the
                //     PATH correctly identifies the asymmetric key)
                //   * `(None, None)` is unreachable because the
                //     union iteration only visits keys present on at
                //     least one side.
                let p_child = p_obj.get(k);
                let r_child = r_obj.get(k);
                match (p_child, r_child) {
                    (Some(p), Some(r)) if p == r => continue,
                    (Some(p), Some(r)) => {
                        path.push(PathSegment::Key(k.clone()));
                        return walk_first_diff(p, r, path);
                    }
                    (Some(p), None) => {
                        path.push(PathSegment::Key(k.clone()));
                        return (p.clone(), Value::Null);
                    }
                    (None, Some(r)) => {
                        path.push(PathSegment::Key(k.clone()));
                        return (Value::Null, r.clone());
                    }
                    (None, None) => unreachable!(
                        "union iteration visits only keys present on at least one side",
                    ),
                }
            }
            // py != rs at top but every union key is equal: shouldn't
            // happen, but return the values themselves at the current
            // path as a safe fallback.
            (py.clone(), rs.clone())
        }
        (Value::Array(p_arr), Value::Array(r_arr)) => {
            let max_len = p_arr.len().max(r_arr.len());
            for i in 0..max_len {
                // Compare `Option<&Value>` so an index that's missing
                // on one side AND explicit `null` on the other side is
                // detected as a real divergence at this index (NOT
                // collapsed to `Null == Null` and reported at `<root>`).
                // Mirrors the object-key 4-arm match above.
                let p_child = p_arr.get(i);
                let r_child = r_arr.get(i);
                match (p_child, r_child) {
                    (Some(p), Some(r)) if p == r => continue,
                    (Some(p), Some(r)) => {
                        path.push(PathSegment::Index(i));
                        return walk_first_diff(p, r, path);
                    }
                    (Some(p), None) => {
                        path.push(PathSegment::Index(i));
                        return (p.clone(), Value::Null);
                    }
                    (None, Some(r)) => {
                        path.push(PathSegment::Index(i));
                        return (Value::Null, r.clone());
                    }
                    (None, None) => {
                        unreachable!("0..max_len visits only indices present on at least one side",)
                    }
                }
            }
            (py.clone(), rs.clone())
        }
        _ => (py.clone(), rs.clone()),
    }
}

/// Hand-rolled line-by-line diff. Not LCS-aware -- aligns lines by
/// index and emits `-` for the Python side, `+` for the Rust side,
/// and a space prefix for matching lines. Capped at
/// `UNIFIED_DIFF_LINE_CAP` lines with an explicit `... (truncated)`
/// marker so callers can tell capped output apart from a naturally
/// short diff.
fn unified_line_diff(py: &str, rs: &str) -> String {
    let py_lines: Vec<&str> = py.lines().collect();
    let rs_lines: Vec<&str> = rs.lines().collect();
    let max = py_lines.len().max(rs_lines.len());
    let mut out: Vec<String> = Vec::new();
    for i in 0..max {
        if out.len() >= UNIFIED_DIFF_LINE_CAP {
            out.push("... (truncated)".to_string());
            break;
        }
        match (py_lines.get(i), rs_lines.get(i)) {
            (Some(p), Some(r)) if p == r => out.push(format!(" {p}")),
            (Some(p), Some(r)) => {
                out.push(format!("-{p}"));
                if out.len() >= UNIFIED_DIFF_LINE_CAP {
                    out.push("... (truncated)".to_string());
                    break;
                }
                out.push(format!("+{r}"));
            }
            (Some(p), None) => out.push(format!("-{p}")),
            (None, Some(r)) => out.push(format!("+{r}")),
            (None, None) => break,
        }
    }
    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn byte_equal_inputs_return_byte_equal() {
        let py = br#"{"a": 1}"#;
        let rs = br#"{"a": 1}"#;
        let d = field_level_diff("artifact.json", py, rs);
        assert_eq!(d, StructuredDiff::ByteEqual);
    }

    #[test]
    fn json_field_diff_surfaces_path_and_values() {
        // Python emits one diagnostics[0].message; Rust emits another.
        // The 8-byte payload difference (`ABCDEFGH` vs `12345678`) is
        // the positive-test driver: the reporter must surface
        // `kind=json_field`, `artifact_path=lint.json`,
        // `path=diagnostics[0].message`, both side values, and a unified
        // diff snippet.
        let py = br#"{"diagnostics":[{"code":"X","message":"ABCDEFGH"}]}"#;
        let rs = br#"{"diagnostics":[{"code":"X","message":"12345678"}]}"#;
        let d = field_level_diff("lint.json", py, rs);
        match d {
            StructuredDiff::JsonField {
                ref artifact_path,
                ref path,
                ref py_value,
                ref rs_value,
                ref unified_diff,
                byte_offset: _,
            } => {
                assert_eq!(artifact_path, "lint.json");
                assert_eq!(path, "diagnostics[0].message");
                assert_eq!(py_value, &Value::String("ABCDEFGH".to_string()));
                assert_eq!(rs_value, &Value::String("12345678".to_string()));
                assert!(unified_diff.contains("ABCDEFGH"));
                assert!(unified_diff.contains("12345678"));
            }
            other => panic!("expected JsonField, got {other:?}"),
        }
    }

    #[test]
    fn jsonl_first_line_diff_reports_line_and_path() {
        let py = b"{\"seq\":0,\"x\":1}\n{\"seq\":1,\"x\":2}\n";
        let rs = b"{\"seq\":0,\"x\":9}\n{\"seq\":1,\"x\":2}\n";
        let d = field_level_diff("events.jsonl", py, rs);
        match d {
            StructuredDiff::JsonlLine {
                ref artifact_path,
                line,
                ref path,
                ref py_value,
                ref rs_value,
                ..
            } => {
                assert_eq!(artifact_path, "events.jsonl");
                assert_eq!(line, 1, "first differing line is 1-indexed");
                assert_eq!(path, "x");
                assert_eq!(py_value, &json!(1));
                assert_eq!(rs_value, &json!(9));
            }
            other => panic!("expected JsonlLine, got {other:?}"),
        }
    }

    #[test]
    fn text_diff_one_char_difference_reports_unified_diff() {
        let py = b"line one\nline two\nline three\n";
        let rs = b"line one\nline TWO\nline three\n";
        let d = field_level_diff("INDEX.md", py, rs);
        match d {
            StructuredDiff::Text {
                ref artifact_path,
                ref unified_diff,
                ..
            } => {
                assert_eq!(artifact_path, "INDEX.md");
                assert!(unified_diff.contains("-line two"));
                assert!(unified_diff.contains("+line TWO"));
                assert!(unified_diff.contains(" line three"));
            }
            other => panic!("expected Text, got {other:?}"),
        }
    }

    #[test]
    fn semantic_equal_but_bytes_differ_surfaces_byte_offset() {
        // Same value, different whitespace + key order at the source
        // text level. serde_json parses both to identical Value trees
        // (since Map is a BTreeMap, key order is normalized on
        // parse), so the structured diff must report the byte-offset
        // semantic-equal marker rather than walking a non-existent
        // field-level diff.
        let py = br#"{"a":1,"b":2}"#;
        let rs = br#"{ "b": 2, "a": 1 }"#;
        let d = field_level_diff("artifact.json", py, rs);
        match d {
            StructuredDiff::SemanticEqualButBytesDiffer {
                ref artifact_path,
                byte_offset,
                py_len,
                rs_len,
            } => {
                assert_eq!(artifact_path, "artifact.json");
                assert!(py_len != rs_len, "lengths must differ");
                assert!(byte_offset < py_len.min(rs_len) + 1);
            }
            other => panic!("expected SemanticEqualButBytesDiffer, got {other:?}"),
        }
    }

    #[test]
    fn non_utf8_binary_diff_reports_binary_kind() {
        let py: &[u8] = &[0xff, 0xfe, 0xfd, 0x00];
        let rs: &[u8] = &[0xff, 0xfe, 0xfd, 0x01];
        let d = field_level_diff("blob.bin", py, rs);
        match d {
            StructuredDiff::Binary {
                ref artifact_path,
                byte_offset,
                py_len,
                rs_len,
            } => {
                assert_eq!(artifact_path, "blob.bin");
                assert_eq!(byte_offset, 3);
                assert_eq!(py_len, 4);
                assert_eq!(rs_len, 4);
            }
            other => panic!("expected Binary, got {other:?}"),
        }
    }

    #[test]
    fn oversized_text_diff_is_truncated_with_marker() {
        // Construct two texts that differ on every line. The truncation
        // cap must produce a snippet ending with `... (truncated)` and
        // strictly fewer than the unlimited line count.
        let mut py_buf = String::new();
        let mut rs_buf = String::new();
        for i in 0..100 {
            py_buf.push_str(&format!("py-line-{i}\n"));
            rs_buf.push_str(&format!("rs-line-{i}\n"));
        }
        let d = field_level_diff("big.md", py_buf.as_bytes(), rs_buf.as_bytes());
        match d {
            StructuredDiff::Text {
                ref unified_diff, ..
            } => {
                assert!(unified_diff.ends_with("... (truncated)"));
                let line_count = unified_diff.lines().count();
                assert!(
                    line_count <= UNIFIED_DIFF_LINE_CAP + 1,
                    "expected at most {} lines (cap + marker); got {line_count}",
                    UNIFIED_DIFF_LINE_CAP + 1
                );
            }
            other => panic!("expected Text, got {other:?}"),
        }
    }

    #[test]
    fn invalid_json_in_json_file_falls_back_to_text_without_panic() {
        // A `.json`-suffixed artifact whose bytes do not parse must
        // not propagate the serde_json error. The reporter should
        // fall through to text (if UTF-8) or binary handling.
        let py = b"{not valid json";
        let rs = b"{also not valid";
        let d = field_level_diff("broken.json", py, rs);
        match d {
            StructuredDiff::Text { .. } => {}
            other => panic!("expected Text fallback for invalid JSON, got {other:?}"),
        }
    }

    #[test]
    fn invalid_json_non_utf8_falls_back_to_binary() {
        // A `.json` artifact whose bytes are not even valid UTF-8 has
        // to fall through past text to binary.
        let py: &[u8] = &[0xff, 0xfe, 0x00, 0x01];
        let rs: &[u8] = &[0xff, 0xfe, 0x00, 0x02];
        let d = field_level_diff("blob.json", py, rs);
        match d {
            StructuredDiff::Binary { .. } => {}
            other => panic!("expected Binary fallback for non-UTF-8 .json, got {other:?}"),
        }
    }

    #[test]
    fn json_array_length_mismatch_reports_index_boundary() {
        let py = br#"{"cards":[{"id":"a"},{"id":"b"}]}"#;
        let rs = br#"{"cards":[{"id":"a"},{"id":"b"},{"id":"c"}]}"#;
        let d = field_level_diff("manifest.json", py, rs);
        match d {
            StructuredDiff::JsonField {
                ref artifact_path,
                ref path,
                ref py_value,
                ref rs_value,
                ..
            } => {
                assert_eq!(artifact_path, "manifest.json");
                assert_eq!(path, "cards[2]");
                assert_eq!(py_value, &Value::Null);
                assert_eq!(rs_value, &json!({"id": "c"}));
            }
            other => panic!("expected JsonField for array-length boundary, got {other:?}"),
        }
    }

    #[test]
    fn json_object_missing_key_vs_explicit_null_reports_correct_path() {
        // Codex R15 review finding: when one side has `{"foo": null}`
        // and the other has `{}`, the prior `walk_first_diff` arm
        // materialized both children to `Value::Null` and considered
        // them equal at key `foo`, falling through to report the
        // diff at `<root>` (or at a later unrelated field). The R16
        // fix compares `Option<&Value>` so the missing-vs-null
        // asymmetry is detected; the diff path correctly names
        // `foo` even when the leaf values both render as `null`.
        //
        // Three scenarios exercise the new 4-arm match:
        //   1. Python has `{"foo": null}`; Rust has `{}` -- path=foo,
        //      py_value=null, rs_value=null (asymmetric key with
        //      null on one side).
        //   2. Python has `{}`; Rust has `{"foo": null}` -- symmetric:
        //      path=foo, py_value=null, rs_value=null.
        //   3. Python has `{"foo": "x"}`; Rust has `{}` -- path=foo,
        //      py_value="x", rs_value=null (value-vs-missing, the
        //      informative case).
        let py = br#"{"foo":null}"#;
        let rs = br#"{}"#;
        let d = field_level_diff("schema.json", py, rs);
        match d {
            StructuredDiff::JsonField {
                ref path,
                ref py_value,
                ref rs_value,
                ..
            } => {
                assert_eq!(
                    path, "foo",
                    "missing-vs-null asymmetry must report path `foo`, not `<root>`"
                );
                assert_eq!(py_value, &Value::Null);
                assert_eq!(rs_value, &Value::Null);
            }
            other => panic!("expected JsonField for missing-vs-null, got {other:?}"),
        }

        // Symmetric: swap python and rust sides.
        let py = br#"{}"#;
        let rs = br#"{"foo":null}"#;
        let d = field_level_diff("schema.json", py, rs);
        match d {
            StructuredDiff::JsonField {
                ref path,
                ref py_value,
                ref rs_value,
                ..
            } => {
                assert_eq!(path, "foo");
                assert_eq!(py_value, &Value::Null);
                assert_eq!(rs_value, &Value::Null);
            }
            other => panic!("expected JsonField for null-vs-missing, got {other:?}"),
        }

        // Value-vs-missing: the more informative variant.
        let py = br#"{"foo":"x"}"#;
        let rs = br#"{}"#;
        let d = field_level_diff("schema.json", py, rs);
        match d {
            StructuredDiff::JsonField {
                ref path,
                ref py_value,
                ref rs_value,
                ..
            } => {
                assert_eq!(path, "foo");
                assert_eq!(py_value, &Value::String("x".to_string()));
                assert_eq!(rs_value, &Value::Null);
            }
            other => panic!("expected JsonField for value-vs-missing, got {other:?}"),
        }
    }

    #[test]
    fn walk_first_diff_reports_array_missing_vs_null_at_correct_index() {
        // Codex R22 review (P3): the `walk_first_diff` array branch
        // had the same missing-vs-explicit-null collapse bug that
        // R16 fixed in the object-key branch. For inputs like
        // `[null]` vs `[]`, the array iteration materialized the
        // missing side as `Value::Null` and treated the pair as
        // equal, so the walker fell through and reported the diff
        // at `<root>` instead of at index 0. R23 applies the same
        // 4-arm `Option<&Value>` match the R16 fix established for
        // objects.
        //
        // Three sub-cases exercise the new array-branch arms:
        //   1. Python `[null]`; Rust `[]` -- path=[0],
        //      py_value=null, rs_value=null (asymmetric index with
        //      null on one side).
        //   2. Python `[]`; Rust `[null]` -- symmetric: path=[0],
        //      py_value=null, rs_value=null.
        //   3. Python `["x"]`; Rust `[]` -- path=[0],
        //      py_value="x", rs_value=null (value-vs-missing, the
        //      informative case).
        let py = br#"[null]"#;
        let rs = br#"[]"#;
        let d = field_level_diff("schema.json", py, rs);
        match d {
            StructuredDiff::JsonField {
                ref path,
                ref py_value,
                ref rs_value,
                ..
            } => {
                assert_eq!(
                    path, "[0]",
                    "missing-vs-null array element must report path `[0]`, not `<root>`"
                );
                assert_eq!(py_value, &Value::Null);
                assert_eq!(rs_value, &Value::Null);
            }
            other => panic!("expected JsonField for `[null]` vs `[]`, got {other:?}"),
        }

        // Symmetric: swap python and rust sides.
        let py = br#"[]"#;
        let rs = br#"[null]"#;
        let d = field_level_diff("schema.json", py, rs);
        match d {
            StructuredDiff::JsonField {
                ref path,
                ref py_value,
                ref rs_value,
                ..
            } => {
                assert_eq!(path, "[0]");
                assert_eq!(py_value, &Value::Null);
                assert_eq!(rs_value, &Value::Null);
            }
            other => panic!("expected JsonField for `[]` vs `[null]`, got {other:?}"),
        }

        // Value-vs-missing: the informative variant.
        let py = br#"["x"]"#;
        let rs = br#"[]"#;
        let d = field_level_diff("schema.json", py, rs);
        match d {
            StructuredDiff::JsonField {
                ref path,
                ref py_value,
                ref rs_value,
                ..
            } => {
                assert_eq!(path, "[0]");
                assert_eq!(py_value, &Value::String("x".to_string()));
                assert_eq!(rs_value, &Value::Null);
            }
            other => panic!("expected JsonField for `[\"x\"]` vs `[]`, got {other:?}"),
        }
    }

    #[test]
    fn root_scalar_diff_renders_root_path() {
        // Top-level scalar diff -- the path renders as `<root>` and
        // the values are the scalars themselves.
        let py = b"42";
        let rs = b"43";
        let d = field_level_diff("scalar.json", py, rs);
        match d {
            StructuredDiff::JsonField {
                ref artifact_path,
                ref path,
                ref py_value,
                ref rs_value,
                ..
            } => {
                assert_eq!(artifact_path, "scalar.json");
                assert_eq!(path, "<root>");
                assert_eq!(py_value, &json!(42));
                assert_eq!(rs_value, &json!(43));
            }
            other => panic!("expected JsonField for root scalar diff, got {other:?}"),
        }
    }

    #[test]
    fn structured_diff_serializes_with_tagged_kind() {
        let d = StructuredDiff::Binary {
            artifact_path: "blob.bin".to_string(),
            byte_offset: 3,
            py_len: 4,
            rs_len: 4,
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("\"kind\":\"binary\""));
        assert!(json.contains("\"artifact_path\":\"blob.bin\""));
        assert!(json.contains("\"byte_offset\":3"));
    }

    #[test]
    fn artifact_path_propagated_through_invalid_json_fallback() {
        // A `.json` artifact whose bytes do not parse falls back to
        // Text. The fallback path must still carry the artifact name
        // so the structured payload identifies which artifact is
        // unparseable.
        let py = b"{not valid json";
        let rs = b"{also not valid";
        let d = field_level_diff("broken.json", py, rs);
        match d {
            StructuredDiff::Text {
                ref artifact_path, ..
            } => assert_eq!(artifact_path, "broken.json"),
            other => panic!("expected Text fallback, got {other:?}"),
        }
    }

    #[test]
    fn render_path_examples() {
        assert_eq!(render_path(&[]), "<root>");
        assert_eq!(
            render_path(&[PathSegment::Key("cards".to_string()), PathSegment::Index(3)]),
            "cards[3]"
        );
        assert_eq!(
            render_path(&[
                PathSegment::Key("cards".to_string()),
                PathSegment::Index(3),
                PathSegment::Key("title".to_string())
            ]),
            "cards[3].title"
        );
        assert_eq!(
            render_path(&[
                PathSegment::Key("diagnostics".to_string()),
                PathSegment::Index(0),
                PathSegment::Key("message".to_string())
            ]),
            "diagnostics[0].message"
        );
    }

    #[test]
    fn missing_side_variants_serialize_with_their_kinds() {
        let m_py = StructuredDiff::MissingExpected {
            rust_path: "rs.json".to_string(),
        };
        let m_rs = StructuredDiff::MissingRust {
            expected_path: "py.json".to_string(),
        };
        let m_both = StructuredDiff::BothMissing {
            expected_path: "py.json".to_string(),
            rust_path: "rs.json".to_string(),
        };
        assert!(serde_json::to_string(&m_py)
            .unwrap()
            .contains("\"kind\":\"missing_expected\""));
        assert!(serde_json::to_string(&m_rs)
            .unwrap()
            .contains("\"kind\":\"missing_rust\""));
        assert!(serde_json::to_string(&m_both)
            .unwrap()
            .contains("\"kind\":\"both_missing\""));
    }
}
