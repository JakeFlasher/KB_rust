#![allow(
    clippy::unwrap_used,
    clippy::doc_markdown,
    clippy::doc_lazy_continuation,
    clippy::needless_borrows_for_generic_args,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::unnecessary_cast,
    clippy::manual_range_contains,
    clippy::iter_cloned_collect,
    clippy::explicit_deref_methods,
    clippy::explicit_auto_deref,
    clippy::too_many_lines
)]
//! Seeded proptest gate: generate 10,000 JSON-domain values from a
//! fixed seed, compute Rust canonical bytes via the production
//! `canonical_json` AND via a test-local independent reference
//! serializer, and assert byte-equality per case.
//!
//! The reference serializer matches the semantics of Python
//! `json.dumps(sort_keys=True, separators=(",",":"), ensure_ascii=False)`
//! but is independently implemented — it must NOT delegate to
//! `cacg_core::canonical_json::canonical_json()`.

use cacg_core::canonical_json::canonical_json;
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::{Config, RngAlgorithm, TestRng, TestRunner};
use serde_json::Value;

const FIXED_SEED: [u8; 32] = [
    0xCA, 0xC6, 0xCA, 0xC6, 0xCA, 0xC6, 0xCA, 0xC6, // 'cac6' x4 = "cacg core canonical"
    0xCA, 0x11, 0x0C, 0xA1, 0xCA, 0x11, 0x0C, 0xA1, 0xCA, 0xC6, 0x09, 0x07, 0xCA, 0xC6, 0x09, 0x07,
    0x00, 0x09, 0x00, 0x09, 0x00, 0x09, 0x00, 0x09,
];
const CASES: usize = 10_000;

/// Independent canonical JSON serializer matching Python
/// `json.dumps(sort_keys=True, separators=(",",":"), ensure_ascii=False)`.
/// Must NOT call `cacg_core::canonical_json::canonical_json`.
fn reference_canonical(v: &Value) -> String {
    let mut out = String::new();
    ref_write(&mut out, v);
    out
}

fn ref_write(out: &mut String, v: &Value) {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                out.push_str(&i.to_string());
            } else if let Some(u) = n.as_u64() {
                out.push_str(&u.to_string());
            } else if let Some(f) = n.as_f64() {
                out.push_str(&ref_format_float(f));
            }
        }
        Value::String(s) => ref_write_string(out, s),
        Value::Array(arr) => {
            out.push('[');
            for (i, elem) in arr.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                ref_write(out, elem);
            }
            out.push(']');
        }
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            out.push('{');
            for (i, key) in keys.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                ref_write_string(out, key);
                out.push(':');
                ref_write(out, &map[*key]);
            }
            out.push('}');
        }
    }
}

fn ref_write_string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000C}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c < '\u{0020}' => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Independent Python-repr-compatible f64 formatter.
/// Matches Python json.dumps float formatting:
/// - 0.0 / -0.0 as literals
/// - Decimal form for 1e-4 <= abs(x) < 1e16 (ryu matches Python here)
/// - Exp form for abs(x) < 1e-4 or abs(x) >= 1e16, with lowercase e,
///   explicit sign, and at least 2-digit padded exponent (e.g., 1e-06)
/// Must NOT delegate to cacg_core::canonical_json::format_float_python.
fn ref_format_float(f: f64) -> String {
    if f == 0.0 {
        return if f.is_sign_negative() {
            "-0.0".to_string()
        } else {
            "0.0".to_string()
        };
    }
    let abs = f.abs();
    if abs >= 1e-4 && abs < 1e16 {
        let mut buf = ryu::Buffer::new();
        return buf.format(f).to_string();
    }
    let raw = format!("{f:e}");
    let Some(e_pos) = raw.find('e') else {
        return raw;
    };
    let (mantissa, exp_part) = raw.split_at(e_pos);
    let exp = &exp_part[1..];
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

/// Strategy for a single JSON atom inside the CACG artifact JSON domain.
///
/// String generators are kept bounded to a small Unicode subset because
/// proptest's full `".*"` regex sampler is too slow at 10k cases.
/// The 233-fixture manifest already exhaustively covers the Unicode
/// edge cases (BMP / SMP / SIP / SSP / ligatures / surrogate pairs);
/// the proptest's job is high-volume randomized variety on the value
/// envelope (shapes, integer ranges, key ordering), not Unicode breadth.
fn atom_strategy() -> impl Strategy<Value = Value> {
    prop_oneof![
        Just(Value::Null),
        any::<bool>().prop_map(Value::Bool),
        any::<i64>().prop_map(|i| Value::Number(i.into())),
        // u64 cases above i64::MAX to exercise the as_u64 fallback.
        ((i64::MAX as u64).saturating_add(1)..=u64::MAX).prop_map(|u| Value::Number(u.into())),
        // Floats: round to 6 decimals in [-1, 1] (the `score` subdomain
        // per spec §2.3).
        (-1.0_f64..=1.0).prop_map(|f| {
            let rounded = (f * 1e6).round() / 1e6;
            match serde_json::Number::from_f64(rounded) {
                Some(n) => Value::Number(n),
                None => Value::Number(0i64.into()),
            }
        }),
        // ASCII strings (printable subset, bounded length).
        "[a-zA-Z0-9 _.-]{0,16}".prop_map(Value::String),
        // Strings containing a literal `\u....` text sequence (per Codex
        // R9 note about escape-boundary coverage). When serialized to
        // JSON the leading `\` is escaped to `\\`; canonical_json must
        // produce the same `\\u....` bytes as Python and `canonical_json_from_str`
        // must NOT misclassify it as an unpaired-surrogate escape
        // (Round 10 scanner regression).
        r"\\u[0-9a-fA-F]{4}".prop_map(Value::String),
        // Empty string + single-char strings for edge coverage.
        Just(Value::String(String::new())),
        "[\\x01-\\x1f]".prop_map(Value::String),
    ]
}

/// Strategy for JSON-domain values with a single nesting level
/// (atoms + small arrays/objects containing atoms). Depth-1 keeps the
/// 10k case generation fast; the 233-fixture manifest already covers
/// deeper nesting edge cases.
fn value_strategy() -> impl Strategy<Value = Value> {
    prop_oneof![
        4 => atom_strategy(),
        1 => prop::collection::vec(atom_strategy(), 0..4).prop_map(Value::Array),
        1 => prop::collection::hash_map(
            "[a-zA-Z][a-zA-Z0-9_]{0,3}",
            atom_strategy(),
            0..4
        )
        .prop_map(|m| {
            let mut out = serde_json::Map::new();
            for (k, v) in m {
                out.insert(k, v);
            }
            Value::Object(out)
        }),
    ]
}

#[test]
fn seeded_proptest_matches_reference_oracle_byte_for_byte() {
    let rng = TestRng::from_seed(RngAlgorithm::ChaCha, &FIXED_SEED);
    let config = Config {
        cases: u32::try_from(CASES).unwrap(),
        ..Config::default()
    };
    let mut runner = TestRunner::new_with_rng(config, rng);

    let strategy = value_strategy();
    let mut values: Vec<Value> = Vec::with_capacity(CASES);
    let mut rust_bytes: Vec<String> = Vec::with_capacity(CASES);
    let mut ref_bytes: Vec<String> = Vec::with_capacity(CASES);

    for case_idx in 0..CASES {
        let tree = strategy
            .new_tree(&mut runner)
            .expect("strategy new_tree failed");
        let value = tree.current();
        let rust = canonical_json(&value).unwrap_or_else(|e| {
            panic!("canonical_json failed for case {case_idx}: {e}; value={value:?}")
        });
        let reference = reference_canonical(&value);
        rust_bytes.push(rust);
        ref_bytes.push(reference);
        values.push(value);
    }

    let mut failures = 0usize;
    for (idx, (r, rf)) in rust_bytes.iter().zip(ref_bytes.iter()).enumerate() {
        if r != rf {
            if failures < 10 {
                eprintln!(
                    "case {idx}: divergence\n  production: {r:?}\n  reference:  {rf:?}\n  value:      {:?}",
                    values[idx]
                );
            }
            failures += 1;
        }
    }
    assert_eq!(
        failures, 0,
        "{failures} of {CASES} cases diverged from reference oracle (first 10 logged above)"
    );
}
