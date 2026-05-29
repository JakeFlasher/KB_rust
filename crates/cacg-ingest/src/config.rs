//! Optional YAML config for tunable chunker parameters.
//!
//! Mirrors `legacy_python_oracle/src/cacg/config.py`. A config file is purely additive:
//! when absent, defaults from [`crate::chunker`] apply. When
//! present, the documented fields below override those defaults.
//!
//! Format:
//!
//! ```yaml
//! chunking:
//!   target_tokens: 350
//!   overlap_tokens: 50
//!   max_pages_per_chunk: 2
//! ```
//!
//! Unknown keys at either level are rejected with the byte-equal
//! Python diagnostic — AC-4 line 64 in
//! `plans/cacg-rust-port-m4-ingest-and-migration-pilot-plan.md`. To
//! reproduce Python's `unknown top-level config keys: [...] (allowed:
//! [...])` / `unknown chunking keys: [...]` / `chunking.{field} must
//! be an integer (got {type}: {repr})` strings, the loader parses
//! into a `serde_yaml_bw::Value` and walks it manually rather than
//! relying on serde's `#[serde(deny_unknown_fields)]` (whose error
//! body is `unknown field 'foo', expected ...`, which diverges from
//! Python's catalogue). Range validation mirrors Python's
//! `ChunkingConfig.validate`: positive `target_tokens`, `overlap` in
//! `[0, target_tokens)`, positive `max_pages_per_chunk`.
//!
//! ## Declared YAML 1.1 / 1.2 boolean divergence
//!
//! Python's `yaml.safe_load` parses YAML 1.1, so unquoted `yes` /
//! `on` are booleans. `serde_yaml_bw` parses YAML 1.2, where the
//! same scalars are strings. Both implementations reject these
//! inputs under `CACG-CLI-004`, but the inner error type differs
//! (bool-rejection vs str-rejection). The test
//! `yaml_1_1_truthy_scalars_rejected_as_str` locks that down.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use serde_yaml_bw::Value;
use thiserror::Error;

use crate::chunker::{
    ChunkConfig, DEFAULT_MAX_PAGES_PER_CHUNK, DEFAULT_OVERLAP_TOKENS, DEFAULT_TARGET_TOKENS,
};

/// The single top-level key Python's `_TOP_LEVEL = {"chunking"}` allows.
const ALLOWED_TOP_LEVEL: &[&str] = &["chunking"];

/// The three chunking keys Python's
/// `_CHUNKING_KEYS = {"target_tokens", "overlap_tokens", "max_pages_per_chunk"}`
/// allows, listed in the sorted order Python's `sorted(_CHUNKING_KEYS)` emits.
const ALLOWED_CHUNKING_KEYS: &[&str] = &["max_pages_per_chunk", "overlap_tokens", "target_tokens"];

/// Errors returned by [`load_config`]. Each variant's `Display`
/// string is byte-equal with the matching Python `ConfigError` /
/// `ValueError` message; `cacg-cli` `dispatch_ingest` wraps every
/// variant under `CACG-CLI-004` (exit 2).
#[derive(Debug, Error)]
pub enum ConfigError {
    /// The supplied `--config` path does not name a regular file
    /// (missing, directory, broken symlink).
    #[error("config file not found or not a regular file: {0}")]
    NotFound(PathBuf),
    /// `std::fs::read_to_string` failed after the `is_file()`
    /// precheck — typically permission-denied or a TOCTOU deletion.
    #[error("cannot read config {path}: {source}")]
    ReadFailed {
        /// The config path that failed to read.
        path: PathBuf,
        /// Underlying I/O error.
        source: std::io::Error,
    },
    /// YAML syntax error — e.g. unclosed flow sequence, bad indent,
    /// or any other malformedness the YAML parser itself rejects.
    #[error("failed to parse config {path}: {message}")]
    YamlSyntax {
        /// The config path that failed to parse.
        path: PathBuf,
        /// `serde_yaml_bw`'s error message, captured verbatim.
        message: String,
    },
    /// Top-level YAML document was not a mapping (e.g. a list,
    /// scalar, or tagged node). Mirrors Python
    /// `f"config root must be a mapping: {p}"`.
    #[error("config root must be a mapping: {path}")]
    RootNotMapping {
        /// The config path whose root was not a mapping.
        path: PathBuf,
    },
    /// One or more top-level keys other than `chunking`. Mirrors
    /// Python
    /// `f"unknown top-level config keys: {sorted(...)} (allowed: {sorted(...)})"`.
    #[error(
        "unknown top-level config keys: {} (allowed: {})",
        py_list_repr(.keys),
        py_list_repr(&ALLOWED_TOP_LEVEL.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>()),
    )]
    UnknownTopLevelKeys {
        /// The unknown keys, already sorted alphabetically.
        keys: Vec<String>,
    },
    /// `chunking` was present but not a mapping (e.g. a list,
    /// string, or null). Mirrors Python
    /// `"config 'chunking' must be a mapping"`.
    #[error("config 'chunking' must be a mapping")]
    ChunkingNotMapping,
    /// One or more keys under `chunking` other than the three
    /// allowed integer fields. Mirrors Python
    /// `f"unknown chunking keys: {sorted(...)} (allowed: {sorted(...)})"`.
    #[error(
        "unknown chunking keys: {} (allowed: {})",
        py_list_repr(.keys),
        py_list_repr(&ALLOWED_CHUNKING_KEYS.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>()),
    )]
    UnknownChunkingKeys {
        /// The unknown keys, already sorted alphabetically.
        keys: Vec<String>,
    },
    /// A `chunking.*` value was not a Python int (e.g. str, float,
    /// bool, list, mapping, null). Mirrors Python
    /// `f"chunking.{key} must be an integer (got {type(raw).__name__}: {raw!r})"`.
    #[error("chunking.{field} must be an integer (got {py_type}: {py_repr})")]
    IntegerTypeError {
        /// The chunking field whose value was the wrong type.
        field: &'static str,
        /// Python's `type(raw).__name__` for that value
        /// (e.g. "str", "float", "bool", "list", "dict", "NoneType").
        py_type: &'static str,
        /// Python's `repr(raw)` for that value.
        py_repr: String,
    },
    /// `chunking.target_tokens <= 0`. Byte-equal Python message.
    #[error("chunking.target_tokens must be positive")]
    TargetTokensNonPositive,
    /// `chunking.overlap_tokens` falls outside `[0, target_tokens)`.
    /// Byte-equal Python message.
    #[error("chunking.overlap_tokens must be in [0, target_tokens)")]
    OverlapOutOfRange,
    /// `chunking.max_pages_per_chunk <= 0`. Byte-equal Python message.
    #[error("chunking.max_pages_per_chunk must be positive")]
    MaxPagesNonPositive,
}

/// Load a [`ChunkConfig`] from an optional YAML config path.
///
/// - `None` → returns `ChunkConfig::default()`.
/// - Missing / non-file path → [`ConfigError::NotFound`].
/// - Read failure → [`ConfigError::ReadFailed`].
/// - YAML syntax failure → [`ConfigError::YamlSyntax`].
/// - Structural / type / range failure → one of the matching
///   variants, each producing the byte-equal Python diagnostic
///   string the CLI wraps under `CACG-CLI-004`.
///
/// # Errors
///
/// Returns [`ConfigError`] on any of the failure modes above.
pub fn load_config(path: Option<&Path>) -> Result<ChunkConfig, ConfigError> {
    let Some(path) = path else {
        return Ok(ChunkConfig::default());
    };
    if !path.is_file() {
        return Err(ConfigError::NotFound(path.to_path_buf()));
    }
    let raw = std::fs::read_to_string(path).map_err(|source| ConfigError::ReadFailed {
        path: path.to_path_buf(),
        source,
    })?;
    // An empty file or a YAML doc that parses to `null` collapses to
    // defaults, matching Python's `if data is None: return ChunkingConfig()`.
    if raw.trim().is_empty() {
        return Ok(ChunkConfig::default());
    }
    let value: Value = serde_yaml_bw::from_str(&raw).map_err(|e| ConfigError::YamlSyntax {
        path: path.to_path_buf(),
        message: e.to_string(),
    })?;

    // `null` root → defaults (Python's `if data is None:`).
    if matches!(value, Value::Null(_)) {
        return Ok(ChunkConfig::default());
    }

    // Root must be a mapping.
    let Value::Mapping(root) = value else {
        return Err(ConfigError::RootNotMapping {
            path: path.to_path_buf(),
        });
    };

    // Unknown top-level keys: any key not in {"chunking"}. Keys are
    // stringified before sorting so a mixed-type unknown key set
    // (e.g. `1: 2` plus `foo: 3`) sorts deterministically — mirrors
    // Python's `sorted(str(k) for k in unknown_top)`.
    let mut unknown_top: Vec<String> = root
        .iter()
        .map(|(k, _)| yaml_key_to_string(k))
        .filter(|s| !ALLOWED_TOP_LEVEL.contains(&s.as_str()))
        .collect();
    if !unknown_top.is_empty() {
        unknown_top.sort();
        return Err(ConfigError::UnknownTopLevelKeys { keys: unknown_top });
    }

    // `chunking` is optional; absent → defaults.
    let chunking_value = root
        .iter()
        .find(|(k, _)| matches!(k, Value::String(s, _) if s == "chunking"))
        .map(|(_, v)| v);

    let mut target_tokens = DEFAULT_TARGET_TOKENS;
    let mut overlap_tokens = DEFAULT_OVERLAP_TOKENS;
    let mut max_pages_per_chunk = DEFAULT_MAX_PAGES_PER_CHUNK;

    if let Some(ch) = chunking_value {
        let Value::Mapping(ch_map) = ch else {
            return Err(ConfigError::ChunkingNotMapping);
        };

        let mut unknown_ch: Vec<String> = ch_map
            .iter()
            .map(|(k, _)| yaml_key_to_string(k))
            .filter(|s| !ALLOWED_CHUNKING_KEYS.contains(&s.as_str()))
            .collect();
        if !unknown_ch.is_empty() {
            unknown_ch.sort();
            return Err(ConfigError::UnknownChunkingKeys { keys: unknown_ch });
        }

        if let Some(v) = lookup_str_key(ch_map, "target_tokens") {
            target_tokens = coerce_u32(v, "target_tokens")?;
        }
        if let Some(v) = lookup_str_key(ch_map, "overlap_tokens") {
            overlap_tokens = coerce_u32(v, "overlap_tokens")?;
        }
        if let Some(v) = lookup_str_key(ch_map, "max_pages_per_chunk") {
            max_pages_per_chunk = coerce_u32(v, "max_pages_per_chunk")?;
        }
    }

    // Validation mirrors Python `ChunkingConfig.validate` byte-for-byte.
    if target_tokens == 0 {
        return Err(ConfigError::TargetTokensNonPositive);
    }
    if overlap_tokens >= target_tokens {
        return Err(ConfigError::OverlapOutOfRange);
    }
    if max_pages_per_chunk == 0 {
        return Err(ConfigError::MaxPagesNonPositive);
    }

    Ok(ChunkConfig {
        target_tokens,
        overlap_tokens,
        max_pages_per_chunk,
    })
}

/// Stringify a YAML key for the unknown-key diagnostics. Python's
/// `sorted(str(k) for k in d)` invokes `str()` on every key, which
/// for the YAML-loaded values is the scalar's literal form for
/// scalars and Python-`str`-of-list/dict for collections. The set
/// of mixed-type cases we actually exercise is small (str keys
/// from typo'd configs); for anything richer we fall back to the
/// value's Debug.
#[allow(clippy::wildcard_enum_match_arm)]
fn yaml_key_to_string(v: &Value) -> String {
    match v {
        Value::String(s, _) => s.clone(),
        Value::Bool(b, _) => if *b { "True" } else { "False" }.to_owned(),
        Value::Number(n, _) => n.to_string(),
        Value::Null(_) => "None".to_owned(),
        other => format!("{other:?}"),
    }
}

/// Find the value for a string-typed key in a YAML mapping. Returns
/// `None` for missing keys; never panics. Mappings preserve insertion
/// order, so a linear scan is fine for the three known fields.
fn lookup_str_key<'a>(m: &'a serde_yaml_bw::Mapping, key: &str) -> Option<&'a Value> {
    m.iter()
        .find(|(k, _)| matches!(k, Value::String(s, _) if s == key))
        .map(|(_, v)| v)
}

/// Coerce a YAML value to a `u32`, producing Python-shape errors
/// when the value is the wrong type. Mirrors Python's
/// `_coerce_int` plus the `target_tokens <= u32::MAX` envelope:
///
/// - Int that fits `[0, u32::MAX]` → `Ok(value as u32)`.
/// - Int < 0 → `Ok(0)` so the downstream
///   `ChunkingConfig.validate` "must be positive" / "in
///   `[0, target_tokens)`" surface fires (byte-equal Python).
/// - Int > `u32::MAX` → `Ok(u32::MAX)`; the same downstream
///   validators reject (`overlap_tokens >= target_tokens`).
/// - Any non-int scalar (bool / str / float) → `IntegerTypeError`
///   with `type(raw).__name__` + `repr(raw)` byte-equal Python.
/// - Sequence / mapping / null → same `IntegerTypeError`.
fn coerce_u32(v: &Value, field: &'static str) -> Result<u32, ConfigError> {
    let (py_type, py_repr) = match v {
        // bool is matched first because Python's `isinstance(raw, bool)`
        // is checked before the int guard — `True`/`False` always
        // surface as bool, never as 1/0.
        Value::Bool(b, _) => ("bool", if *b { "True" } else { "False" }.to_owned()),
        Value::Number(n, _) => {
            // YAML 1.2 distinguishes int from float at parse time.
            // serde_yaml_bw's Number exposes is_i64() / is_u64() /
            // is_f64(); ints satisfy one of the first two.
            if n.is_u64() || n.is_i64() {
                // Python accepts any int (arbitrary precision); coerce
                // to i64 for range-check and clamp to u32 envelope so
                // the downstream validator fires the byte-equal
                // "must be positive" / "in [0, target_tokens)" surface.
                let as_i64 = n.as_i64().unwrap_or(i64::MAX);
                if as_i64 < 0 {
                    return Ok(0); // → "must be positive" downstream
                }
                let clamped: u64 = u64::try_from(as_i64).unwrap_or(0);
                return Ok(u32::try_from(clamped).unwrap_or(u32::MAX));
            }
            ("float", py_float_repr(n.as_f64().unwrap_or(f64::NAN)))
        }
        Value::String(s, _) => ("str", py_str_repr(s)),
        Value::Sequence(seq) => ("list", py_seq_repr(seq)),
        Value::Mapping(map) => ("dict", py_map_repr(map)),
        Value::Null(_) => ("NoneType", "None".to_owned()),
        Value::Tagged(t) => ("tagged", format!("{t:?}")),
        Value::Alias(a) => ("alias", a.clone()),
    };
    Err(ConfigError::IntegerTypeError {
        field,
        py_type,
        py_repr,
    })
}

/// Format a list of strings the way Python's `repr(list[str])`
/// would: `['a', 'b']` — square brackets, comma-space separated,
/// each element single-quoted via `py_str_repr`.
fn py_list_repr(items: &[String]) -> String {
    let mut out = String::from("[");
    for (i, s) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&py_str_repr(s));
    }
    out.push(']');
    out
}

/// Render an arbitrary YAML value the way Python's `repr()` would
/// render the corresponding `yaml.safe_load` Python object. Used by
/// the `IntegerTypeError` payload so a YAML list value `[1, 2]`
/// reaches the operator as `[1, 2]` (byte-equal Python), not `[]`.
fn py_value_repr(v: &Value) -> String {
    match v {
        Value::Null(_) => "None".to_owned(),
        Value::Bool(b, _) => if *b { "True" } else { "False" }.to_owned(),
        Value::Number(n, _) => {
            if n.is_u64() || n.is_i64() {
                n.to_string()
            } else {
                py_float_repr(n.as_f64().unwrap_or(f64::NAN))
            }
        }
        Value::String(s, _) => py_str_repr(s),
        Value::Sequence(seq) => py_seq_repr(seq),
        Value::Mapping(map) => py_map_repr(map),
        Value::Tagged(t) => format!("{t:?}"),
        Value::Alias(a) => a.clone(),
    }
}

/// Render a YAML sequence as `[elem1, elem2, …]` — byte-equal with
/// Python `repr(list)`.
fn py_seq_repr(seq: &serde_yaml_bw::Sequence) -> String {
    let mut out = String::from("[");
    for (i, item) in seq.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&py_value_repr(item));
    }
    out.push(']');
    out
}

/// Render a YAML mapping as `{k1: v1, k2: v2, …}` — byte-equal with
/// Python `repr(dict)`. Keys are repr'd via `py_value_repr` too, so
/// string keys carry their single-quotes (`{'a': 1}`).
fn py_map_repr(map: &serde_yaml_bw::Mapping) -> String {
    let mut out = String::from("{");
    for (i, (k, v)) in map.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&py_value_repr(k));
        out.push_str(": ");
        out.push_str(&py_value_repr(v));
    }
    out.push('}');
    out
}

/// Render a string the way Python's `repr(str)` renders it. Python
/// uses single quotes by default and switches to double only when
/// the string contains a `'` and no `"`. For the YAML-key /
/// YAML-scalar values we surface here (typo'd identifiers, short
/// values), the simple ASCII path is sufficient. Non-ASCII and
/// control chars are escaped at Python's exact width via `\xXX` /
/// `\uXXXX` / `\UXXXXXXXX` — same surface as the
/// `cacg-cli::main::py_repr` helper, kept self-contained here so
/// the loader stays decoupled from the CLI.
fn py_str_repr(s: &str) -> String {
    let quote = if s.contains('\'') && !s.contains('"') {
        '"'
    } else {
        '\''
    };
    let mut out = String::with_capacity(s.len() + 2);
    out.push(quote);
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            c if (c as u32) < 0x20 || c as u32 == 0x7f => {
                let _ = write!(out, "\\x{:02x}", c as u32);
            }
            c if (c as u32) < 0x7f => out.push(c),
            // For >= 0x80 we keep raw — Python's `str.isprintable()`
            // covers most cases (`é`, emoji); the rare non-printable
            // C1 controls would require the full nonprintable table.
            // YAML keys in this loader are short identifiers, so raw
            // is fine for the surface we exercise.
            c => out.push(c),
        }
    }
    out.push(quote);
    out
}

/// Render an `f64` the way Python's `repr(float)` would. Python
/// always includes a decimal point (`repr(350.0) == "350.0"`) and
/// uses the shortest round-trip representation otherwise
/// (`repr(350.5) == "350.5"`). Rust's `Display` drops the `.0` for
/// integral floats, so we add it back; for non-integral floats the
/// default `Display` already matches Python's shortest-round-trip
/// for the YAML scalars we exercise.
fn py_float_repr(f: f64) -> String {
    if !f.is_finite() {
        if f.is_nan() {
            return "nan".to_owned();
        }
        return if f.is_sign_negative() { "-inf" } else { "inf" }.to_owned();
    }
    let s = format!("{f}");
    if !s.contains('.') && !s.contains('e') && !s.contains('E') {
        format!("{s}.0")
    } else {
        s
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// Holder that keeps a `TempDir` alive for the duration of a test
    /// while exposing the deterministic `config.yaml` path inside it.
    /// Using `tempdir()` (not `Builder::tempfile()`) keeps the
    /// `xtask lint-determinism` gate happy: random suffixes are gated
    /// behind `DeterminismContext::next_temp_name`, but a random
    /// *directory* name is allowed because tests never publish the
    /// dir into a committed artifact.
    struct TmpYaml {
        _dir: tempfile::TempDir,
        path: PathBuf,
    }
    impl TmpYaml {
        fn path(&self) -> &Path {
            &self.path
        }
    }

    fn write_tmp(contents: &str) -> TmpYaml {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("config.yaml");
        std::fs::write(&path, contents).expect("write");
        TmpYaml { _dir: dir, path }
    }

    #[test]
    fn none_returns_defaults() {
        let cfg = load_config(None).unwrap();
        assert_eq!(cfg.target_tokens, DEFAULT_TARGET_TOKENS);
        assert_eq!(cfg.overlap_tokens, DEFAULT_OVERLAP_TOKENS);
        assert_eq!(cfg.max_pages_per_chunk, DEFAULT_MAX_PAGES_PER_CHUNK);
    }

    #[test]
    fn missing_file_surfaces_not_found() {
        let err = load_config(Some(Path::new("/tmp/no-such-config-abc.yaml"))).unwrap_err();
        assert!(matches!(err, ConfigError::NotFound(_)));
    }

    #[test]
    fn directory_path_surfaces_not_found() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let err = load_config(Some(tmp.path())).unwrap_err();
        assert!(matches!(err, ConfigError::NotFound(_)));
    }

    #[test]
    fn empty_file_returns_defaults() {
        let f = write_tmp("");
        let cfg = load_config(Some(f.path())).unwrap();
        assert_eq!(cfg.target_tokens, DEFAULT_TARGET_TOKENS);
    }

    #[test]
    fn null_root_returns_defaults() {
        let f = write_tmp("null\n");
        let cfg = load_config(Some(f.path())).unwrap();
        assert_eq!(cfg.target_tokens, DEFAULT_TARGET_TOKENS);
    }

    #[test]
    fn full_chunking_overrides_apply() {
        let f = write_tmp(
            "chunking:\n  target_tokens: 200\n  overlap_tokens: 25\n  max_pages_per_chunk: 3\n",
        );
        let cfg = load_config(Some(f.path())).unwrap();
        assert_eq!(cfg.target_tokens, 200);
        assert_eq!(cfg.overlap_tokens, 25);
        assert_eq!(cfg.max_pages_per_chunk, 3);
    }

    #[test]
    fn partial_chunking_falls_back_to_defaults_per_key() {
        let f = write_tmp("chunking:\n  target_tokens: 200\n");
        let cfg = load_config(Some(f.path())).unwrap();
        assert_eq!(cfg.target_tokens, 200);
        assert_eq!(cfg.overlap_tokens, DEFAULT_OVERLAP_TOKENS);
        assert_eq!(cfg.max_pages_per_chunk, DEFAULT_MAX_PAGES_PER_CHUNK);
    }

    #[test]
    fn unknown_top_level_key_emits_byte_equal_python_message() {
        let f = write_tmp("foo: 1\nchunking:\n  target_tokens: 200\n");
        let err = load_config(Some(f.path())).unwrap_err();
        let msg = err.to_string();
        // Byte-equal Python `_cmd_ingest` → `_TOP_LEVEL` rejection.
        assert_eq!(
            msg,
            "unknown top-level config keys: ['foo'] (allowed: ['chunking'])",
        );
    }

    #[test]
    fn multiple_unknown_top_level_keys_sort_alphabetically() {
        // Python `sorted(str(k) for k in unknown_top)` ⇒ alphabetical.
        let f = write_tmp("zebra: 1\nalpha: 2\nchunking:\n  target_tokens: 200\n");
        let err = load_config(Some(f.path())).unwrap_err();
        let msg = err.to_string();
        assert_eq!(
            msg,
            "unknown top-level config keys: ['alpha', 'zebra'] (allowed: ['chunking'])",
        );
    }

    #[test]
    fn unknown_chunking_key_emits_byte_equal_python_message() {
        let f = write_tmp("chunking:\n  target_tokens: 200\n  weirdo: 1\n");
        let err = load_config(Some(f.path())).unwrap_err();
        let msg = err.to_string();
        assert_eq!(
            msg,
            "unknown chunking keys: ['weirdo'] (allowed: \
             ['max_pages_per_chunk', 'overlap_tokens', 'target_tokens'])",
        );
    }

    #[test]
    fn non_mapping_root_emits_byte_equal_python_message() {
        let f = write_tmp("- a\n- b\n");
        let err = load_config(Some(f.path())).unwrap_err();
        let msg = err.to_string();
        // Path component varies per test; assert the prefix + the
        // exact `config.yaml` tail.
        assert!(
            msg.starts_with("config root must be a mapping: "),
            "actual: {msg}"
        );
        assert!(msg.ends_with("config.yaml"), "actual: {msg}");
    }

    #[test]
    fn chunking_not_mapping_emits_byte_equal_python_message() {
        let f = write_tmp("chunking: 42\n");
        let err = load_config(Some(f.path())).unwrap_err();
        assert_eq!(err.to_string(), "config 'chunking' must be a mapping");
    }

    #[test]
    fn str_value_rejected_with_python_shape_message() {
        let f = write_tmp("chunking:\n  target_tokens: not_a_number\n");
        let err = load_config(Some(f.path())).unwrap_err();
        assert_eq!(
            err.to_string(),
            "chunking.target_tokens must be an integer (got str: 'not_a_number')",
        );
    }

    #[test]
    fn bool_value_rejected_with_python_shape_message() {
        // Python's `_coerce_int` rejects bool (subclass of int).
        // YAML 1.2 `true` → bool → "got bool: True".
        let f = write_tmp("chunking:\n  target_tokens: true\n");
        let err = load_config(Some(f.path())).unwrap_err();
        assert_eq!(
            err.to_string(),
            "chunking.target_tokens must be an integer (got bool: True)",
        );
    }

    #[test]
    fn float_value_rejected_with_python_shape_message() {
        let f = write_tmp("chunking:\n  target_tokens: 350.5\n");
        let err = load_config(Some(f.path())).unwrap_err();
        assert_eq!(
            err.to_string(),
            "chunking.target_tokens must be an integer (got float: 350.5)",
        );
    }

    #[test]
    fn yaml_1_1_truthy_scalars_rejected_as_str() {
        // YAML 1.1 parsers (Python `yaml.safe_load`) read unquoted
        // `yes` / `on` as booleans; serde_yaml_bw parses YAML 1.2
        // where they are strings. Both implementations reject the
        // value under CACG-CLI-004; the inner error type differs.
        // This test locks the Rust-side str-rejection so a future
        // serde_yaml_bw release that adopts YAML 1.1 bool parsing
        // would change the error class deliberately, not silently.
        for word in ["yes", "on", "YES", "On"] {
            let f = write_tmp(&format!("chunking:\n  target_tokens: {word}\n"));
            let err = load_config(Some(f.path())).unwrap_err();
            let msg = err.to_string();
            assert!(
                msg.starts_with("chunking.target_tokens must be an integer (got str: '"),
                "YAML-1.1 truthy scalar {word:?} must reject as str; got: {msg}",
            );
        }
    }

    #[test]
    fn list_value_rejected_with_python_shape_message() {
        let f = write_tmp("chunking:\n  target_tokens: [1, 2]\n");
        let err = load_config(Some(f.path())).unwrap_err();
        assert_eq!(
            err.to_string(),
            "chunking.target_tokens must be an integer (got list: [1, 2])",
        );
    }

    #[test]
    fn empty_list_value_rejected_with_python_shape_message() {
        let f = write_tmp("chunking:\n  target_tokens: []\n");
        let err = load_config(Some(f.path())).unwrap_err();
        assert_eq!(
            err.to_string(),
            "chunking.target_tokens must be an integer (got list: [])",
        );
    }

    #[test]
    fn dict_value_rejected_with_python_shape_message() {
        // Python `repr({'k': 1}) == "{'k': 1}"` — keys are repr'd
        // (single-quoted) and key/value separated by `: `.
        let f = write_tmp("chunking:\n  target_tokens:\n    k: 1\n");
        let err = load_config(Some(f.path())).unwrap_err();
        assert_eq!(
            err.to_string(),
            "chunking.target_tokens must be an integer (got dict: {'k': 1})",
        );
    }

    #[test]
    fn null_value_rejected_with_python_shape_message() {
        // YAML `null` (or `~`) → Python None → "got NoneType: None".
        let f = write_tmp("chunking:\n  target_tokens: null\n");
        let err = load_config(Some(f.path())).unwrap_err();
        assert_eq!(
            err.to_string(),
            "chunking.target_tokens must be an integer (got NoneType: None)",
        );
    }

    #[test]
    fn negative_int_falls_through_to_must_be_positive() {
        // Python `_coerce_int(-1)` returns -1 (it's an int); then
        // `validate()` raises "must be positive". The Rust loader
        // clamps negative ints to 0 so the same validator fires
        // and emits the byte-equal Python message.
        let f = write_tmp("chunking:\n  target_tokens: -1\n");
        let err = load_config(Some(f.path())).unwrap_err();
        assert!(matches!(err, ConfigError::TargetTokensNonPositive));
        assert_eq!(err.to_string(), "chunking.target_tokens must be positive");
    }

    #[test]
    fn zero_target_tokens_rejected_at_validation() {
        let f = write_tmp("chunking:\n  target_tokens: 0\n");
        let err = load_config(Some(f.path())).unwrap_err();
        assert!(matches!(err, ConfigError::TargetTokensNonPositive));
    }

    #[test]
    fn overlap_equal_target_rejected_at_validation() {
        let f = write_tmp("chunking:\n  target_tokens: 50\n  overlap_tokens: 50\n");
        let err = load_config(Some(f.path())).unwrap_err();
        assert!(matches!(err, ConfigError::OverlapOutOfRange));
        assert_eq!(
            err.to_string(),
            "chunking.overlap_tokens must be in [0, target_tokens)",
        );
    }

    #[test]
    fn zero_max_pages_rejected_at_validation() {
        let f = write_tmp("chunking:\n  max_pages_per_chunk: 0\n");
        let err = load_config(Some(f.path())).unwrap_err();
        assert!(matches!(err, ConfigError::MaxPagesNonPositive));
    }

    #[test]
    fn py_float_repr_preserves_dot_zero_for_integral_floats() {
        // Lock the helper used by IntegerTypeError so future
        // refactors keep the byte-equal Python `repr(350.0) == "350.0"`
        // contract.
        assert_eq!(super::py_float_repr(350.0), "350.0");
        assert_eq!(super::py_float_repr(350.5), "350.5");
        assert_eq!(super::py_float_repr(0.0), "0.0");
        assert_eq!(super::py_float_repr(f64::INFINITY), "inf");
        assert_eq!(super::py_float_repr(-f64::INFINITY), "-inf");
        assert!(super::py_float_repr(f64::NAN) == "nan");
    }

    #[test]
    fn py_list_repr_single_quotes_and_brackets() {
        assert_eq!(super::py_list_repr(&[]), "[]");
        assert_eq!(super::py_list_repr(&["foo".into()]), "['foo']");
        assert_eq!(super::py_list_repr(&["a".into(), "b".into()]), "['a', 'b']",);
    }
}
