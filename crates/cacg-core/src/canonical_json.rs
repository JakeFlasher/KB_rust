//! Canonical JSON writer matching Python `json.dumps(value, sort_keys=True,
//! separators=(",",":"), ensure_ascii=False)` byte-for-byte.
//!
//! The full spec is at `_research/08_canonical_json_spec.md`. This module
//! is the Rust half of the byte-equal trust contract; the Python half is
//! `cacg.hash.canonical_json` in `legacy_python_oracle/src/cacg/hash.py`. The 233 happy-path
//! fixtures at `tests/parity_corpus/canonical_json/manifest.json` and the
//! 12 reject fixtures at `tests/parity_corpus/canonical_json_reject/manifest.json`
//! are the merge-block gate that both implementations satisfy.
//!
//! Public API:
//! - [`canonical_json`] writes a [`serde_json::Value`] to its canonical form.
//! - [`canonical_json_from_str`] parses raw JSON text via a strict loader
//!   that rejects duplicate object keys and unpaired surrogates, then
//!   canonicalizes the resulting Value.
//! - [`CanonicalError`] enumerates every rejection class defined in spec §4.

use std::fmt::Write as _;

use serde_json::Value;
use thiserror::Error;

/// Errors raised at the canonical-JSON writer entry point.
///
/// Every variant maps to a spec §4 rejection class. No partial output is
/// emitted on any failure path.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CanonicalError {
    /// `NaN`, `+Infinity`, `-Infinity` are not admissible.
    #[error("non-finite float (NaN/+Inf/-Inf) not admissible in canonical JSON")]
    NonFiniteFloat,
    /// Object keys must be UTF-8 strings.
    #[error("non-string object key not admissible in canonical JSON")]
    NonStringKey,
    /// Duplicate object keys are rejected by the strict loader.
    #[error("duplicate object key {key:?} rejected by strict canonical loader")]
    DuplicateKey {
        /// The colliding key (first detected duplicate).
        key: String,
    },
    /// A lone high or low surrogate is not admissible (Rust strings must be
    /// valid UTF-8; serde_json maps `\uD800..\uDFFF` to a parse error which
    /// we surface as this variant).
    #[error("unpaired surrogate not admissible in canonical JSON")]
    UnpairedSurrogate {
        /// Codepoint of the offending surrogate half, or 0 if the parser
        /// could not isolate a specific codepoint from the error.
        codepoint: u32,
    },
    /// Types outside the CACG artifact JSON domain (bytes, datetime, set,
    /// custom objects) reach the writer only via a misuse upstream of
    /// serde_json::Value (e.g., a hand-rolled deserializer). serde_json's
    /// Value type cannot natively carry these, so this variant is
    /// largely a type-level guarantee.
    #[error("unsupported type {type_name} not admissible in canonical JSON")]
    UnsupportedType {
        /// The Rust type name that could not be canonicalized.
        type_name: &'static str,
    },
}

/// Serialize a [`serde_json::Value`] to its canonical byte form.
///
/// Returns `Ok(bytes)` on success, or [`CanonicalError`] on any of the
/// spec §4 rejection classes.
///
/// # Errors
///
/// Returns [`CanonicalError::NonFiniteFloat`] if the input contains a
/// non-finite f64 (only reachable via a misuse upstream of serde_json,
/// since [`serde_json::Number::from_f64`] returns `None` for NaN/Inf).
pub fn canonical_json(value: &Value) -> Result<String, CanonicalError> {
    let mut out = String::with_capacity(64);
    write_value(&mut out, value)?;
    Ok(out)
}

/// Parse a raw JSON string with strict-loader semantics (duplicate-key
/// rejection + unpaired-surrogate rejection) and canonicalize the
/// resulting value.
///
/// This is the trust-boundary entry point: it MUST reject every input
/// shape that Python `json.loads` silently coerces (last-wins on
/// duplicate keys; invalid-UTF-8 escape sequences).
///
/// # Errors
///
/// Returns [`CanonicalError::DuplicateKey`] when the input JSON contains
/// the same key twice in one object, [`CanonicalError::UnpairedSurrogate`]
/// when the input contains a lone surrogate escape, or
/// [`CanonicalError::UnsupportedType`] when the input is not valid JSON
/// at all (carries `type_name = "json-parse-error"`).
pub fn canonical_json_from_str(s: &str) -> Result<String, CanonicalError> {
    let value = strict_parse(s)?;
    canonical_json(&value)
}

fn write_value(out: &mut String, v: &Value) -> Result<(), CanonicalError> {
    match v {
        Value::Null => {
            out.push_str("null");
            Ok(())
        }
        Value::Bool(true) => {
            out.push_str("true");
            Ok(())
        }
        Value::Bool(false) => {
            out.push_str("false");
            Ok(())
        }
        Value::Number(n) => write_number(out, n),
        Value::String(s) => {
            write_string(out, s);
            Ok(())
        }
        Value::Array(a) => write_array(out, a),
        Value::Object(o) => write_object(out, o),
    }
}

fn write_number(out: &mut String, n: &serde_json::Number) -> Result<(), CanonicalError> {
    if let Some(i) = n.as_i64() {
        // i64 covers signed range; serde_json::Number::as_i64 returns Some
        // only for integers within i64::MIN..=i64::MAX.
        write!(out, "{i}").expect("writing to String never fails");
        return Ok(());
    }
    if let Some(u) = n.as_u64() {
        // u64::MAX > i64::MAX; values above i64::MAX surface here.
        write!(out, "{u}").expect("writing to String never fails");
        return Ok(());
    }
    if let Some(f) = n.as_f64() {
        if !f.is_finite() {
            return Err(CanonicalError::NonFiniteFloat);
        }
        out.push_str(&format_float_python(f));
        return Ok(());
    }
    // serde_json::Number cannot natively carry NaN/Inf, and as_i64/as_u64/as_f64
    // collectively cover every constructable shape. This branch is defensive.
    Err(CanonicalError::UnsupportedType {
        type_name: "unrepresentable-number",
    })
}

fn write_string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{0009}' => out.push_str("\\t"),
            '\u{000A}' => out.push_str("\\n"),
            '\u{000C}' => out.push_str("\\f"),
            '\u{000D}' => out.push_str("\\r"),
            c if (c as u32) < 0x20 => {
                // Spec §3.4: U+0000..U+0007, U+000B, U+000E..U+001F render
                // as `\u00XX` (4-hex lowercase) per Python's default
                // ensure_ascii=False behavior on control codepoints.
                write!(out, "\\u{:04x}", c as u32).expect("writing to String never fails");
            }
            // U+007F DELETE passes through (Python json.dumps does NOT escape
            // it under ensure_ascii=False). U+2028 / U+2029 pass through
            // (documented divergence from RFC 8259 §7's recommendation,
            // mirroring Python). Everything else: literal UTF-8 bytes.
            c => out.push(c),
        }
    }
    out.push('"');
}

fn write_array(out: &mut String, a: &[Value]) -> Result<(), CanonicalError> {
    out.push('[');
    for (i, item) in a.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        write_value(out, item)?;
    }
    out.push(']');
    Ok(())
}

fn write_object(
    out: &mut String,
    o: &serde_json::Map<String, Value>,
) -> Result<(), CanonicalError> {
    out.push('{');
    // Spec §3.3: object keys sorted codepoint-by-codepoint. Rust
    // String::cmp is byte-by-byte over UTF-8 which is codepoint-by-codepoint
    // for valid UTF-8 (UTF-8 byte ordering matches codepoint ordering).
    let mut entries: Vec<(&String, &Value)> = o.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));
    for (i, (k, v)) in entries.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        write_string(out, k);
        out.push(':');
        write_value(out, v)?;
    }
    out.push('}');
    Ok(())
}

/// Python-`repr`-compatible f64 formatter.
///
/// Spec §3.7 pins this to Python's `repr(float)` behavior:
/// - Whole-number floats keep trailing `.0` (e.g., `2.0` not `2`).
/// - Negative zero renders as `-0.0`.
/// - Exponential form for `abs(x) < 1e-4` and `abs(x) >= 1e16`; decimal
///   form otherwise.
/// - Exponent uses lowercase `e`, explicit sign, and at least 2-digit
///   padded exponent (e.g., `1e-06` not `1e-6`).
///
/// Implementation delegates to `ryu` for the shortest round-trip
/// representation, then post-processes for Python-specific cosmetic
/// rules.
fn format_float_python(f: f64) -> String {
    // Zero is sign-preserving but otherwise renders as `0.0` / `-0.0`.
    if f == 0.0 {
        return if f.is_sign_negative() {
            "-0.0".to_string()
        } else {
            "0.0".to_string()
        };
    }

    let abs = f.abs();

    // Python repr thresholds: decimal form for 1e-4 <= abs(x) < 1e16;
    // exp form otherwise. ryu picks form on a shortest-string heuristic
    // that disagrees with Python at the boundary (ryu emits "0.00001"
    // for 1e-5 because it's the same length as "1e-5" and decimal is
    // preferred lexicographically) — so we route by abs(x) ourselves.
    if abs >= 1e-4 && abs < 1e16 {
        // Decimal form. ryu's output matches Python here:
        //   2.0_f64 -> "2.0" (trailing .0 preserved for whole floats)
        //   0.5_f64 -> "0.5"
        //   0.0001_f64 -> "0.0001"
        //   9999.999999_f64 -> "9999.999999"
        let mut buf = ryu::Buffer::new();
        return buf.format(f).to_string();
    }

    // Exp form. Rust's built-in `{:e}` emits e.g. "1e-5" for 1e-5
    // (no leading zero on exponent, no explicit + sign on positive
    // exponents). Python wants "1e-05" with explicit sign + 2-digit
    // padded exponent. Normalize via `normalize_python_exponent`.
    //
    // Whole-number floats in exp form: format!("{:e}", 1e16) = "1e16",
    // Python repr(1e16) = "1e+16" — the normalizer handles both the
    // sign and the padding.
    normalize_python_exponent(&format!("{f:e}"))
}

/// Normalize an exponential-form float string so the exponent has an
/// explicit sign and at least 2 digits, matching Python's `repr`.
///
/// Examples: `"1e-6"` -> `"1e-06"`, `"1.5e4"` -> `"1.5e+04"`.
fn normalize_python_exponent(s: &str) -> String {
    let Some(e_pos) = s.find('e') else {
        return s.to_string();
    };
    let (mantissa, exp_part) = s.split_at(e_pos);
    let exp = &exp_part[1..]; // skip the 'e'
    let (sign, digits) = if let Some(rest) = exp.strip_prefix('-') {
        ("-", rest)
    } else if let Some(rest) = exp.strip_prefix('+') {
        ("+", rest)
    } else {
        ("+", exp)
    };
    if digits.len() < 2 {
        format!("{mantissa}e{sign}0{digits}")
    } else {
        format!("{mantissa}e{sign}{digits}")
    }
}

/// Scan a raw JSON string for `\uD800..\uDFFF` escape sequences that
/// are NOT part of a valid surrogate pair (high `\uD800..\uDBFF`
/// followed by low `\uDC00..\uDFFF`).
///
/// The scanner is JSON-string-aware: it only inspects `\uXXXX` when
/// it occurs inside a JSON string AND the preceding `\` is not itself
/// part of an escape (e.g., `\\u1234` is a literal backslash followed
/// by literal `u1234`, NOT a unicode escape). Robust to serde_json
/// version drift -- serde_json's parse-error message text for
/// surrogate rejection varies, so this pre-scan is the reliable path
/// to surface `CanonicalError::UnpairedSurrogate`.
///
/// Returns `Some(codepoint)` if an unpaired surrogate is found,
/// `None` otherwise.
fn scan_unpaired_surrogate(s: &str) -> Option<u32> {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut in_string = false;
    let mut pending_high: Option<u32> = None;

    while i < bytes.len() {
        let b = bytes[i];
        if !in_string {
            if b == b'"' {
                in_string = true;
                pending_high = None;
            }
            i += 1;
            continue;
        }

        // Inside a JSON string. Handle escape sequences explicitly so
        // a `\\` (literal backslash) consumes both bytes -- the
        // subsequent `u1234` is then literal text, not a unicode escape.
        match b {
            b'"' => {
                // End of string. Any pending high surrogate without a
                // following low surrogate is unpaired.
                if let Some(cp) = pending_high {
                    return Some(cp);
                }
                in_string = false;
                i += 1;
            }
            b'\\' => {
                // Escape sequence. Inspect the next byte.
                if i + 1 >= bytes.len() {
                    return None; // truncated; serde_json will surface it
                }
                let next = bytes[i + 1];
                if next == b'u' {
                    // \uXXXX unicode escape (need 4 hex digits after the 'u').
                    if i + 6 > bytes.len() {
                        return None; // truncated
                    }
                    let hex = std::str::from_utf8(&bytes[i + 2..i + 6]).ok()?;
                    let Ok(cp) = u32::from_str_radix(hex, 16) else {
                        // Not valid hex; serde_json will surface it.
                        i += 2;
                        continue;
                    };
                    if (0xD800..=0xDBFF).contains(&cp) {
                        // High surrogate. A previous pending high without
                        // a low between is unpaired.
                        if let Some(prev_hi) = pending_high {
                            // qg-allow: deep-nesting — unpaired high surrogate
                            return Some(prev_hi);
                        }
                        pending_high = Some(cp);
                    } else if (0xDC00..=0xDFFF).contains(&cp) {
                        // qg-allow: deep-nesting — low surrogate branch
                        // Low surrogate. Paired only if a high surrogate
                        // immediately preceded.
                        if pending_high.is_none() {
                            // qg-allow: deep-nesting — unpaired low surrogate
                            return Some(cp);
                        }
                        pending_high = None;
                    } else {
                        // qg-allow: deep-nesting — non-surrogate codepoint
                        // Non-surrogate codepoint. A previous pending high
                        // without a low immediately after is unpaired.
                        if let Some(prev_hi) = pending_high.take() {
                            // qg-allow: deep-nesting — trailing unpaired high
                            return Some(prev_hi);
                        }
                    }
                    i += 6;
                } else {
                    // Other escapes (\", \\, \/, \b, \f, \n, \r, \t) consume
                    // 2 bytes and are NOT unicode escapes. A pending high
                    // surrogate followed by a non-`\u` escape is unpaired.
                    if let Some(prev_hi) = pending_high.take() {
                        return Some(prev_hi);
                    }
                    i += 2;
                }
            }
            _ => {
                // Literal character inside string. A pending high surrogate
                // followed by a literal char (not `\uDC..`) is unpaired.
                if let Some(prev_hi) = pending_high.take() {
                    return Some(prev_hi);
                }
                i += 1;
            }
        }
    }
    None
}

/// Strict JSON loader that rejects duplicate object keys and surfaces
/// unpaired-surrogate parse errors as [`CanonicalError::UnpairedSurrogate`].
///
/// Built on top of `serde_json::Deserializer` with a custom Visitor that
/// tracks seen keys per object.
fn strict_parse(s: &str) -> Result<Value, CanonicalError> {
    use serde::de::Deserialize as _;

    // Pre-scan for unpaired-surrogate escapes BEFORE handing to serde_json.
    // serde_json's parser rejects `\uD800..\uDFFF` not followed by a valid
    // low-surrogate pair, but its error message varies across versions
    // ("unexpected end of hex escape" / "lone leading surrogate" / etc).
    // A dedicated pre-scan keeps the typed CanonicalError variant
    // reliable regardless of serde_json version drift.
    if let Some(codepoint) = scan_unpaired_surrogate(s) {
        return Err(CanonicalError::UnpairedSurrogate { codepoint });
    }

    let mut de = serde_json::Deserializer::from_str(s);
    let strict = StrictValue::deserialize(&mut de).map_err(|e| {
        let msg = e.to_string();
        // The pre-scan above already promoted unpaired-surrogate cases to
        // UnpairedSurrogate; any remaining surrogate-mentioning parse
        // error is folded into the same variant defensively.
        if msg.contains("surrogate") || msg.contains("hex escape") {
            CanonicalError::UnpairedSurrogate { codepoint: 0 }
        } else if let Some(key) = msg.strip_prefix("DUPLICATE_KEY:") {
            // Sentinel format raised by StrictValue's MapAccess visitor.
            // The key is everything after the colon and before any
            // trailing position annotation serde_json appends.
            CanonicalError::DuplicateKey {
                key: key.splitn(2, " at line").next().unwrap_or(key).to_string(),
            }
        } else {
            CanonicalError::UnsupportedType {
                type_name: "json-parse-error",
            }
        }
    })?;
    de.end().map_err(|_| CanonicalError::UnsupportedType {
        type_name: "json-trailing-garbage",
    })?;
    Ok(strict.0)
}

/// Newtype carrying a [`serde_json::Value`] parsed under strict semantics.
struct StrictValue(Value);

impl<'de> serde::Deserialize<'de> for StrictValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictValueVisitor)
    }
}

struct StrictValueVisitor;

impl<'de> serde::de::Visitor<'de> for StrictValueVisitor {
    type Value = StrictValue;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("any valid JSON value")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(StrictValue(Value::Null))
    }
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
        Ok(StrictValue(Value::Bool(v)))
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
        Ok(StrictValue(Value::Number(v.into())))
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
        Ok(StrictValue(Value::Number(v.into())))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        serde_json::Number::from_f64(v).map_or_else(
            || Err(E::custom("non-finite float not admissible")),
            |n| Ok(StrictValue(Value::Number(n))),
        )
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
        Ok(StrictValue(Value::String(v.to_string())))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
        Ok(StrictValue(Value::String(v)))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut out = Vec::new();
        while let Some(item) = seq.next_element::<StrictValue>()? {
            out.push(item.0);
        }
        Ok(StrictValue(Value::Array(out)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut out = serde_json::Map::new();
        while let Some(key) = map.next_key::<String>()? {
            let value = map.next_value::<StrictValue>()?;
            if out.contains_key(&key) {
                // Sentinel format that strict_parse() pattern-matches to
                // promote into CanonicalError::DuplicateKey.
                return Err(serde::de::Error::custom(format!("DUPLICATE_KEY:{key}")));
            }
            out.insert(key, value.0);
        }
        Ok(StrictValue(Value::Object(out)))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn writes_primitives() {
        assert_eq!(canonical_json(&Value::Null).unwrap(), "null");
        assert_eq!(canonical_json(&Value::Bool(true)).unwrap(), "true");
        assert_eq!(canonical_json(&Value::Bool(false)).unwrap(), "false");
    }

    #[test]
    fn writes_ints() {
        assert_eq!(canonical_json(&Value::from(0i64)).unwrap(), "0");
        assert_eq!(canonical_json(&Value::from(-1i64)).unwrap(), "-1");
        assert_eq!(canonical_json(&Value::from(42i64)).unwrap(), "42");
    }

    #[test]
    fn writes_floats() {
        assert_eq!(format_float_python(0.0), "0.0");
        assert_eq!(format_float_python(-0.0), "-0.0");
        assert_eq!(format_float_python(1.0), "1.0");
        assert_eq!(format_float_python(2.0), "2.0");
        assert_eq!(format_float_python(0.5), "0.5");
        assert_eq!(format_float_python(0.0001), "0.0001");
        assert_eq!(format_float_python(1e-5), "1e-05");
        assert_eq!(format_float_python(1e-6), "1e-06");
        assert_eq!(format_float_python(9999.999999), "9999.999999");
    }

    #[test]
    fn rejects_nonfinite_via_serde_json_type_level() {
        assert!(serde_json::Number::from_f64(f64::NAN).is_none());
        assert!(serde_json::Number::from_f64(f64::INFINITY).is_none());
        assert!(serde_json::Number::from_f64(f64::NEG_INFINITY).is_none());
    }

    #[test]
    fn rejects_duplicate_keys() {
        let r = canonical_json_from_str(r#"{"a":1,"a":2}"#);
        assert!(matches!(r, Err(CanonicalError::DuplicateKey { .. })));
    }

    #[test]
    fn rejects_unpaired_surrogate() {
        let r = canonical_json_from_str(r#""\uD800""#);
        assert!(matches!(r, Err(CanonicalError::UnpairedSurrogate { .. })));
    }

    #[test]
    fn rejects_unpaired_low_surrogate() {
        let r = canonical_json_from_str(r#""\uDC00""#);
        assert!(matches!(r, Err(CanonicalError::UnpairedSurrogate { .. })));
    }

    #[test]
    fn accepts_paired_surrogate_pair_emoji() {
        // U+1F600 (grinning face emoji) is encoded as the surrogate pair
        // 😀 in JSON. The scanner must see this as paired.
        let r = canonical_json_from_str(r#""😀""#);
        assert!(r.is_ok(), "paired surrogate pair must canonicalize: {r:?}");
        // Round-trip: the canonical output is the emoji as literal UTF-8.
        assert_eq!(r.unwrap(), "\"\u{1f600}\"");
    }

    #[test]
    fn accepts_literal_backslash_u_text() {
        // In JSON `"\\ud800"` is a string of 6 chars: literal `\`, `u`,
        // `d`, `8`, `0`, `0`. The double-backslash escapes the
        // backslash; the subsequent `ud800` is NOT a unicode escape.
        // Round 9's raw-byte scanner falsely rejected this; Round 10's
        // JSON-string-aware scanner must accept.
        let r = canonical_json_from_str(r#""\\ud800""#);
        assert!(
            r.is_ok(),
            "literal backslash followed by ud800 must be accepted: {r:?}"
        );
        // Output canonicalizes back to the same JSON form (Rust escapes
        // the literal backslash as `\\`).
        assert_eq!(r.unwrap(), r#""\\ud800""#);
    }

    #[test]
    fn accepts_literal_backslash_u_in_object_value() {
        // Same false-positive class but inside an object value -- proves
        // the scanner correctly enters/exits string state across the
        // key/value/comma structure.
        let r = canonical_json_from_str(r#"{"k":"\\ud800"}"#);
        assert!(
            r.is_ok(),
            "literal backslash in object value must be accepted: {r:?}"
        );
        assert_eq!(r.unwrap(), r#"{"k":"\\ud800"}"#);
    }

    #[test]
    fn sorts_object_keys_codepoint() {
        let v: Value = serde_json::from_str(r#"{"A":3,"a":1,"z":2}"#).unwrap();
        assert_eq!(canonical_json(&v).unwrap(), r#"{"A":3,"a":1,"z":2}"#);
    }
}
