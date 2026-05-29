//! CACG Layer-3 semantic verifier — B1 cache-as-oracle runtime.
//!
//! Mirrors `legacy_python_oracle/src/cacg/verify/semantic.py` on the public boundary
//! contracts: `claim_window_hash`, cache loading and validation,
//! lookup verdicts. See `_research/m5b_semantic_py_spec.md` for the
//! line-by-line oracle reference.
//!
//! B1 is the cache-as-oracle dict lookup path; it ships under the
//! default `b1-cache-as-oracle` feature and is deterministic. B2
//! (LLM-judge via Claude Haiku) is async and opt-in via the
//! `b2-llm-judge` feature flag; under that feature it pulls
//! reqwest + tokio + async-trait into the dependency graph and
//! exposes the [`b2`] module (`LlmJudgeClient` trait,
//! `MockJudgeClient`, `HaikuClient`, `JudgeError`). Without the
//! feature, the [`b2`] module is compiled out entirely and those
//! dependencies stay out of the resolved closure.
//!
//! Cache loading rejects duplicate JSON object keys at every level
//! BEFORE materialization via a custom `Deserialize` impl that walks
//! the `MapAccess` with a `BTreeSet<String>` of seen keys. A
//! post-parse `serde_json::Value` scan is insufficient because
//! duplicate keys are already lost by then. See
//! `BL-20260518-reject-duplicate-keys-at-trust-boundary`.
//!
//! Pydantic-style validators are reproduced as
//! `validate_structurally` methods covering every `@field_validator`
//! / `@model_validator` from the Python source. See
//! `BL-20260522-port-pydantic-validators-not-just-fields`.
//!
//! `SemanticCache::load(&Path)` pre-checks `is_file()` before reading.
//! See `BL-20260518-shape-check-fs-inputs`.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic, missing_docs)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::doc_markdown,
    clippy::doc_lazy_continuation,
    clippy::missing_panics_doc,
    clippy::similar_names,
    clippy::float_cmp,
    clippy::redundant_closure_for_method_calls,
    clippy::needless_borrows_for_generic_args,
    clippy::must_use_candidate,
    clippy::should_implement_trait
)]

use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use serde::de::{self, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Serialize};

// The verdict / mode / evaluator types live in cacg-core so the
// trust kernel never gains a transitive dependency on this crate
// (or its B2 reqwest+tokio surface). `cacg-semantic` re-exports them
// so callers like `cacg_semantic::SemanticVerdict` still resolve.
pub use cacg_core::verify::semantic_spec::{
    SemanticEvaluationError, SemanticEvaluator, SemanticMode, SemanticVerdict, SemanticVerdictKind,
};

/// Crate-wide test-only synchronization for the process-global
/// `CACG_SEMANTIC_LOAD_TRACE` environment variable. Held by
/// `SemanticCache::load` under `cfg(test)` so that no other test in
/// this crate may execute `load` concurrently with a test that has
/// the env var set. Release builds compile WITHOUT this lock; the
/// production load path is unchanged.
///
/// Load-trace unit tests must NOT call `SemanticCache::load` while
/// holding this lock — that would deadlock on the non-reentrant
/// mutex. Tests acquire this lock, then call `load_inner` +
/// `emit_load_trace` directly to exercise the same code paths
/// without recursing back through the lock-taking wrapper.
///
/// See `BL-20260522-one-mutex-per-process-global-in-tests`.
#[cfg(test)]
pub(crate) static LOAD_TRACE_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Schema version literal for the on-disk semantic cache. Matches
/// Python's `SCHEMA_VERSION = "cacg.v0"` at `legacy_python_oracle/src/cacg/schema.py:17`.
pub const SEMANTIC_CACHE_SCHEMA_VERSION: &str = "cacg.v0";

#[cfg(feature = "b2-llm-judge")]
pub mod b2;

/// Crate version helper retained for cross-crate identification.
#[must_use]
pub fn cacg_semantic_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// ----------------------------------------------------------------------
// On-disk cache types
// ----------------------------------------------------------------------

/// One semantic verdict recorded for a `(chunk_hash, claim_window_hash)`
/// pair. Custom `Deserialize` impl rejects duplicate JSON object keys
/// at this level via a `BTreeSet<String>` of seen keys.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SemanticCacheEntry {
    /// SHA-256 of the chunk text (64 lowercase hex chars).
    pub chunk_hash: String,
    /// SHA-256 of the normalized claim window (64 lowercase hex chars).
    pub claim_window_hash: String,
    /// Recorded verdict.
    pub verdict: SemanticVerdictKind,
    /// Recorded score (`[0.0, 1.0]`, finite).
    pub score: f64,
}

impl SemanticCacheEntry {
    fn validate_structurally(&self) -> Result<(), SemanticError> {
        validate_64_hex("chunk_hash", &self.chunk_hash)?;
        validate_64_hex("claim_window_hash", &self.claim_window_hash)?;
        validate_score("score", self.score)?;
        Ok(())
    }
}

impl<'de> Deserialize<'de> for SemanticCacheEntry {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(SemanticCacheEntryVisitor)
    }
}

struct SemanticCacheEntryVisitor;

impl<'de> Visitor<'de> for SemanticCacheEntryVisitor {
    type Value = SemanticCacheEntry;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a semantic cache entry object")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
        let mut chunk_hash: Option<String> = None;
        let mut claim_window_hash: Option<String> = None;
        let mut verdict: Option<SemanticVerdictKind> = None;
        let mut score: Option<f64> = None;
        let mut seen: BTreeSet<String> = BTreeSet::new();

        while let Some(key) = map.next_key::<String>()? {
            if !seen.insert(key.clone()) {
                return Err(de::Error::custom(format!(
                    "duplicate key {key:?} in semantic cache entry",
                )));
            }
            match key.as_str() {
                "chunk_hash" => chunk_hash = Some(map.next_value()?),
                "claim_window_hash" => claim_window_hash = Some(map.next_value()?),
                "verdict" => verdict = Some(map.next_value()?),
                "score" => score = Some(map.next_value()?),
                other => {
                    return Err(de::Error::custom(format!(
                        "unknown field {other:?} in semantic cache entry",
                    )));
                }
            }
        }

        Ok(SemanticCacheEntry {
            chunk_hash: chunk_hash.ok_or_else(|| de::Error::missing_field("chunk_hash"))?,
            claim_window_hash: claim_window_hash
                .ok_or_else(|| de::Error::missing_field("claim_window_hash"))?,
            verdict: verdict.ok_or_else(|| de::Error::missing_field("verdict"))?,
            score: score.ok_or_else(|| de::Error::missing_field("score"))?,
        })
    }
}

/// The complete on-disk semantic cache. Custom `Deserialize` impl
/// rejects duplicate JSON object keys at the top level; entries[i]
/// duplicate-key rejection is delegated to
/// [`SemanticCacheEntry::deserialize`].
#[derive(Clone, Debug, Serialize)]
pub struct SemanticCache {
    /// Must equal [`SEMANTIC_CACHE_SCHEMA_VERSION`].
    pub schema_version: String,
    /// Cache entries in load order.
    pub entries: Vec<SemanticCacheEntry>,
    /// O(1) lookup index keyed on `(chunk_hash, claim_window_hash)`.
    #[serde(skip)]
    index: HashMap<(String, String), usize>,
}

impl PartialEq for SemanticCache {
    fn eq(&self, other: &Self) -> bool {
        self.schema_version == other.schema_version && self.entries == other.entries
    }
}

impl SemanticCache {
    /// Read a semantic cache from a JSON string. Pre-checks every
    /// structural invariant (schema_version literal, hash format,
    /// score range, semantic-key uniqueness) and the duplicate-key
    /// constraint from
    /// `BL-20260518-reject-duplicate-keys-at-trust-boundary`.
    ///
    /// The duplicate-key check runs in two passes. The first pass is
    /// a RECURSIVE raw-JSON scanner ([`reject_duplicate_json_keys`])
    /// that visits every map at every depth (including maps under
    /// unknown fields that the typed deserializer would reject
    /// without inspecting their interior). The second pass is the
    /// typed deserialization that turns the JSON into the strongly
    /// typed structs and runs the structural validators. The
    /// typed-visitor duplicate-key checks remain as defense in depth.
    pub fn from_str(json: &str) -> Result<Self, SemanticError> {
        reject_duplicate_json_keys(json)?;
        let mut cache: SemanticCache =
            serde_json::from_str(json).map_err(|e| SemanticError::Json(e.to_string()))?;
        cache.validate_structurally()?;
        cache.index = build_index(&cache.entries)?;
        Ok(cache)
    }

    /// Serialize the cache to canonical JSON via
    /// `cacg_core::canonical_json::canonical_json`. The private
    /// `index` is skipped (already marked `#[serde(skip)]`); entry
    /// order is preserved verbatim (Python's `dump_semantic_cache`
    /// does not sort `entries`).
    pub fn to_canonical_json(&self) -> Result<String, SemanticError> {
        let value = serde_json::to_value(self)
            .map_err(|e| SemanticError::CanonicalSerialize(e.to_string()))?;
        cacg_core::canonical_json::canonical_json(&value)
            .map_err(|e| SemanticError::CanonicalSerialize(e.to_string()))
    }

    /// Read a semantic cache from disk. Routes directories and
    /// missing paths through [`SemanticError::NotRegularFile`] per
    /// `BL-20260518-shape-check-fs-inputs` BEFORE attempting any
    /// `read_to_string` (which would otherwise leak `IsADirectoryError`
    /// equivalents up the stack).
    ///
    /// Test instrumentation: when the `CACG_SEMANTIC_LOAD_TRACE`
    /// environment variable is set to a writable file path, every
    /// invocation of this function appends exactly one line to that
    /// file:
    ///   - `load_ok <path> <entry_count>` on a successful load.
    ///   - `load_err <path>` on any error path.
    /// The trace is silent (no file touched) when the env var is
    /// absent, so production runs see zero overhead. The line format
    /// is deterministic so subprocess tests can assert exact-line
    /// counts.
    pub fn load(path: &Path) -> Result<Self, SemanticError> {
        // Under `cfg(test)` only, serialize every public load against
        // load-trace tests that mutate the process-global env var.
        // The lock is non-reentrant; load-trace tests therefore call
        // `load_inner` + `emit_load_trace` directly (NOT this wrapper)
        // while holding the same lock. `unwrap_or_else(into_inner)`
        // recovers from poisoning so a panicking sibling test does not
        // cascade-fail subsequent runs in the same process.
        #[cfg(test)]
        let _trace_guard = LOAD_TRACE_TEST_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let result = Self::load_inner(path);
        Self::emit_load_trace(path, result.as_ref().ok().map(|c| c.entries.len()));
        result
    }

    fn load_inner(path: &Path) -> Result<Self, SemanticError> {
        if !path.is_file() {
            return Err(SemanticError::NotRegularFile(path.to_path_buf()));
        }
        let raw = fs::read_to_string(path).map_err(|source| SemanticError::Read {
            path: path.to_path_buf(),
            source,
        })?;
        Self::from_str(&raw)
    }

    fn emit_load_trace(path: &Path, success_entries: Option<usize>) {
        let Ok(trace_path) = std::env::var("CACG_SEMANTIC_LOAD_TRACE") else {
            return;
        };
        let line = if let Some(count) = success_entries {
            format!("load_ok {} {}\n", path.display(), count)
        } else {
            format!("load_err {}\n", path.display())
        };
        if let Ok(mut f) = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&trace_path)
        {
            use std::io::Write as _;
            let _ = f.write_all(line.as_bytes());
        }
    }

    /// O(1) lookup. Cache miss returns
    /// [`SemanticVerdict::abstain_embedding_cache`]; cache hit
    /// reconstructs a `SemanticVerdict` from the matched entry's
    /// `(verdict, score)` with mode `embedding-cache`.
    #[must_use]
    pub fn lookup(&self, chunk_hash: &str, claim_window_hash: &str) -> SemanticVerdict {
        let Some(&idx) = self
            .index
            .get(&(chunk_hash.to_string(), claim_window_hash.to_string()))
        else {
            return SemanticVerdict::abstain_embedding_cache();
        };
        let entry = &self.entries[idx];
        SemanticVerdict {
            kind: entry.verdict,
            score: entry.score,
            reasoning: None,
            mode: SemanticMode::EmbeddingCache,
        }
    }

    fn validate_structurally(&self) -> Result<(), SemanticError> {
        if self.schema_version != SEMANTIC_CACHE_SCHEMA_VERSION {
            return Err(SemanticError::UnknownSchemaVersion(
                self.schema_version.clone(),
            ));
        }
        for entry in &self.entries {
            entry.validate_structurally()?;
        }
        Ok(())
    }
}

impl SemanticEvaluator for SemanticCache {
    fn evaluate(
        &self,
        chunk_hash: &str,
        claim_window_hash: &str,
        _quote: &str,
        _chunk_text: &str,
    ) -> Result<SemanticVerdict, SemanticEvaluationError> {
        // B1 (cache-as-oracle) is infallible — cache misses produce
        // `abstain_embedding_cache()`, never an error. `quote` and
        // `chunk_text` are ignored because the verdict is keyed
        // entirely by the pre-computed `(chunk_hash, claim_window_hash)`
        // pair.
        Ok(self.lookup(chunk_hash, claim_window_hash))
    }
}

impl<'de> Deserialize<'de> for SemanticCache {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(SemanticCacheVisitor)
    }
}

struct SemanticCacheVisitor;

impl<'de> Visitor<'de> for SemanticCacheVisitor {
    type Value = SemanticCache;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a semantic cache object")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
        let mut schema_version: Option<String> = None;
        let mut entries: Option<Vec<SemanticCacheEntry>> = None;
        let mut seen: BTreeSet<String> = BTreeSet::new();

        while let Some(key) = map.next_key::<String>()? {
            if !seen.insert(key.clone()) {
                return Err(de::Error::custom(format!(
                    "duplicate key {key:?} in semantic cache",
                )));
            }
            match key.as_str() {
                "schema_version" => schema_version = Some(map.next_value()?),
                "entries" => entries = Some(map.next_value()?),
                other => {
                    return Err(de::Error::custom(format!(
                        "unknown field {other:?} in semantic cache",
                    )));
                }
            }
        }

        Ok(SemanticCache {
            schema_version: schema_version
                .ok_or_else(|| de::Error::missing_field("schema_version"))?,
            entries: entries.unwrap_or_default(), // qg-allow: intentional-discard — serde: missing entries field defaults to empty
            index: HashMap::new(),
        })
    }
}

// ----------------------------------------------------------------------
// Errors
// ----------------------------------------------------------------------

/// Failure modes surfaced by [`SemanticCache::load`] /
/// [`SemanticCache::from_str`] and the helper validators. Each variant
/// is the Rust counterpart of one Python `_build_semantic_spec`
/// failure row; the CLI dispatcher maps every variant onto
/// `CACG-MAN-001` (matching Python).
#[derive(Debug)]
pub enum SemanticError {
    /// `SemanticCache::load` was handed a path that is not a regular
    /// file (missing, directory, broken symlink).
    NotRegularFile(PathBuf),
    /// `std::fs::read_to_string` failed after the shape check passed.
    Read {
        /// Path that failed to read.
        path: PathBuf,
        /// Underlying I/O error.
        source: std::io::Error,
    },
    /// `serde_json` rejected the input — includes malformed JSON,
    /// duplicate JSON object keys at any level, unknown fields, and
    /// missing required fields. The carried `String` is the
    /// underlying serde error message verbatim.
    Json(String),
    /// `schema_version` did not equal `"cacg.v0"`.
    UnknownSchemaVersion(String),
    /// A hash field was not exactly 64 lowercase hex characters.
    InvalidHashField {
        /// Field name (`chunk_hash` or `claim_window_hash`).
        field: &'static str,
        /// Offending value.
        value: String,
    },
    /// A score field was non-finite or outside `[0.0, 1.0]`.
    InvalidScore {
        /// Field name (currently always `"score"`).
        field: &'static str,
        /// Offending value.
        value: f64,
        /// Reason text (`not finite` or `out of range`).
        reason: &'static str,
    },
    /// Two entries shared the same `(chunk_hash, claim_window_hash)`
    /// pair.
    DuplicateSemanticKey {
        /// First component of the colliding pair.
        chunk_hash: String,
        /// Second component of the colliding pair.
        claim_window_hash: String,
    },
    /// `to_canonical_json` failed (either the serde-value conversion
    /// or `cacg_core::canonical_json` rejected the input). Carries
    /// the underlying error message verbatim.
    CanonicalSerialize(String),
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRegularFile(p) => {
                write!(f, "semantic cache path is not a regular file: {}", p.display())
            }
            Self::Read { path, source } => {
                write!(f, "failed to read semantic cache {}: {source}", path.display())
            }
            Self::Json(msg) => write!(f, "semantic cache JSON rejected: {msg}"),
            Self::UnknownSchemaVersion(v) => write!(
                f,
                "semantic cache schema_version {v:?} is not {SEMANTIC_CACHE_SCHEMA_VERSION:?}",
            ),
            Self::InvalidHashField { field, value } => write!(
                f,
                "semantic cache field {field} is not a 64-lowercase-hex SHA-256: {value:?}",
            ),
            Self::InvalidScore { field, value, reason } => write!(
                f,
                "semantic cache field {field} value {value} is {reason}",
            ),
            Self::DuplicateSemanticKey {
                chunk_hash,
                claim_window_hash,
            } => write!(
                f,
                "semantic cache has duplicate entry for (chunk_hash={chunk_hash:?}, claim_window_hash={claim_window_hash:?})",
            ),
            Self::CanonicalSerialize(msg) => {
                write!(f, "semantic cache canonical serialization failed: {msg}")
            }
        }
    }
}

impl std::error::Error for SemanticError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::NotRegularFile(_)
            | Self::Json(_)
            | Self::UnknownSchemaVersion(_)
            | Self::InvalidHashField { .. }
            | Self::InvalidScore { .. }
            | Self::DuplicateSemanticKey { .. }
            | Self::CanonicalSerialize(_) => None,
        }
    }
}

// ----------------------------------------------------------------------
// claim_window_hash
// ----------------------------------------------------------------------

/// SHA-256 (lowercase hex) over `normalize_text(quote).into_bytes()`.
///
/// Byte-equal with Python `cacg.verify.semantic._claim_window_hash`
/// because the underlying `cacg_core::normalize::normalize_text` is
/// already proven byte-identical with Python's `normalize_text`
/// (the 54 `unicode_edge` parity fixtures + paired oracles enforce
/// it).
#[must_use]
pub fn claim_window_hash(quote: &str) -> String {
    let normalized = cacg_core::normalize::normalize_text(quote);
    cacg_core::hash::source_sha256(normalized.as_bytes())
}

// ----------------------------------------------------------------------
// Internal validators
// ----------------------------------------------------------------------

fn validate_64_hex(field: &'static str, value: &str) -> Result<(), SemanticError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(SemanticError::InvalidHashField {
            field,
            value: value.to_string(),
        });
    }
    Ok(())
}

fn validate_score(field: &'static str, value: f64) -> Result<(), SemanticError> {
    if !value.is_finite() {
        return Err(SemanticError::InvalidScore {
            field,
            value,
            reason: "not finite",
        });
    }
    if !(0.0..=1.0).contains(&value) {
        return Err(SemanticError::InvalidScore {
            field,
            value,
            reason: "out of range",
        });
    }
    Ok(())
}

fn build_index(
    entries: &[SemanticCacheEntry],
) -> Result<HashMap<(String, String), usize>, SemanticError> {
    let mut index = HashMap::with_capacity(entries.len());
    for (i, entry) in entries.iter().enumerate() {
        let key = (entry.chunk_hash.clone(), entry.claim_window_hash.clone());
        if index.insert(key.clone(), i).is_some() {
            return Err(SemanticError::DuplicateSemanticKey {
                chunk_hash: key.0,
                claim_window_hash: key.1,
            });
        }
    }
    Ok(index)
}

/// Walk the raw JSON tree and reject duplicate object keys at every
/// depth before typed deserialization runs. This is the trust-
/// boundary half of `BL-20260518-reject-duplicate-keys-at-trust-
/// boundary`: the typed-visitor approach used at the
/// `SemanticCache` / `SemanticCacheEntry` levels CANNOT detect
/// duplicates inside unknown nested objects (the typed visitor would
/// reject "unknown field" before walking into the value), and the
/// post-parse `serde_json::Value` scan is insufficient because
/// `serde_json::Map` collapses duplicates last-write-wins by the
/// time it materializes. The recursive Visitor below sees every
/// `MapAccess::next_key` event AT EVERY DEPTH, including under
/// unknown fields, and rejects the first collision with a stable
/// `"duplicate key ... in semantic cache JSON"` message.
fn reject_duplicate_json_keys(json: &str) -> Result<(), SemanticError> {
    let mut deserializer = serde_json::Deserializer::from_str(json);
    DuplicateKeyChecker
        .deserialize(&mut deserializer)
        .map_err(|e| SemanticError::Json(e.to_string()))?;
    deserializer
        .end()
        .map_err(|e| SemanticError::Json(e.to_string()))?;
    Ok(())
}

/// Recursive duplicate-key visitor + `DeserializeSeed` used by
/// [`reject_duplicate_json_keys`]. Walks maps and sequences without
/// retaining any payload state beyond a per-object `BTreeSet<String>`
/// of seen keys, and rejects the first duplicate it finds at any
/// depth.
struct DuplicateKeyChecker;

impl<'de> DeserializeSeed<'de> for DuplicateKeyChecker {
    type Value = ();

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for DuplicateKeyChecker {
    type Value = ();

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("any JSON value (recursive duplicate-key scan)")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        while let Some(key) = map.next_key::<String>()? {
            if !seen.insert(key.clone()) {
                return Err(de::Error::custom(format!(
                    "duplicate key {key:?} in semantic cache JSON",
                )));
            }
            map.next_value_seed(DuplicateKeyChecker)?;
        }
        Ok(())
    }

    fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        while seq.next_element_seed(DuplicateKeyChecker)?.is_some() {}
        Ok(())
    }

    fn visit_bool<E: de::Error>(self, _v: bool) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_i64<E: de::Error>(self, _v: i64) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_u64<E: de::Error>(self, _v: u64) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_f64<E: de::Error>(self, _v: f64) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_str<E: de::Error>(self, _v: &str) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_borrowed_str<E: de::Error>(self, _v: &'de str) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_string<E: de::Error>(self, _v: String) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_bytes<E: de::Error>(self, _v: &[u8]) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(self)
    }
}

// ----------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn hash_a() -> String {
        "a".repeat(64)
    }

    fn hash_b() -> String {
        "b".repeat(64)
    }

    fn minimal_cache_json() -> String {
        let a = hash_a();
        let b = hash_b();
        format!(
            r#"{{
              "schema_version": "cacg.v0",
              "entries": [
                {{
                  "chunk_hash": "{a}",
                  "claim_window_hash": "{b}",
                  "verdict": "pass",
                  "score": 0.92
                }}
              ]
            }}"#,
        )
    }

    // ---------- crate plumbing ----------

    #[test]
    fn version_is_nonempty() {
        assert!(!cacg_semantic_version().is_empty());
    }

    // ---------- claim_window_hash ----------

    #[test]
    fn claim_window_hash_ascii_whitespace_matches_python_oracle() {
        // From the Round-7 spec doc § 2.5 (Python-generated oracle).
        assert_eq!(
            claim_window_hash("  Alpha\t\n beta   gamma  "),
            "a53fb55d1133524d9f58398c09e03737736a84d98059537401af8e5e7520791b",
        );
    }

    // ---------- SemanticVerdict (re-exported from cacg-core) ----------

    #[test]
    fn abstain_embedding_cache_constructor_resolves_via_reexport() {
        // Sanity that the re-export at the crate root still gives the
        // same constructor; the contract itself is exercised in
        // cacg-core::verify::semantic_spec::tests.
        let v = SemanticVerdict::abstain_embedding_cache();
        assert_eq!(v.kind, SemanticVerdictKind::Abstain);
        assert_eq!(v.score, 0.0);
        assert!(v.reasoning.is_none());
        assert_eq!(v.mode, SemanticMode::EmbeddingCache);
    }

    #[test]
    fn semantic_cache_implements_semantic_evaluator() {
        // Confirm SemanticCache satisfies the cacg-core trait. B1
        // ignores `quote` and `chunk_text`; both are passed as empty
        // strings to prove the impl does not depend on them.
        let cache = SemanticCache::from_str(&minimal_cache_json()).unwrap();
        let evaluator: &dyn SemanticEvaluator = &cache;
        let hit = evaluator
            .evaluate(&hash_a(), &hash_b(), "", "")
            .expect("B1 evaluate is infallible");
        assert_eq!(hit.kind, SemanticVerdictKind::Pass);
        let miss = evaluator
            .evaluate(&"c".repeat(64), &"d".repeat(64), "", "")
            .expect("B1 evaluate is infallible");
        assert_eq!(miss.kind, SemanticVerdictKind::Abstain);
    }

    // ---------- SemanticCache::from_str (positive) ----------

    #[test]
    fn from_str_accepts_minimal_valid_cache() {
        let cache = SemanticCache::from_str(&minimal_cache_json()).unwrap();
        assert_eq!(cache.schema_version, "cacg.v0");
        assert_eq!(cache.entries.len(), 1);
        assert_eq!(cache.entries[0].verdict, SemanticVerdictKind::Pass);
    }

    #[test]
    fn from_str_accepts_empty_entries_array() {
        let json = r#"{"schema_version":"cacg.v0","entries":[]}"#;
        let cache = SemanticCache::from_str(json).unwrap();
        assert!(cache.entries.is_empty());
    }

    #[test]
    fn from_str_defaults_entries_when_omitted() {
        // Python's `entries: list[SemanticCacheEntry] = Field(default_factory=list)`
        // makes the field optional; the Rust port honours the same default.
        let json = r#"{"schema_version":"cacg.v0"}"#;
        let cache = SemanticCache::from_str(json).unwrap();
        assert!(cache.entries.is_empty());
    }

    // ---------- SemanticCache::lookup ----------

    #[test]
    fn lookup_returns_pass_on_hit() {
        let cache = SemanticCache::from_str(&minimal_cache_json()).unwrap();
        let v = cache.lookup(&hash_a(), &hash_b());
        assert_eq!(v.kind, SemanticVerdictKind::Pass);
        assert!((v.score - 0.92).abs() < 1e-9);
        assert_eq!(v.mode, SemanticMode::EmbeddingCache);
    }

    #[test]
    fn lookup_returns_abstain_on_miss() {
        let cache = SemanticCache::from_str(&minimal_cache_json()).unwrap();
        let v = cache.lookup(&"c".repeat(64), &"d".repeat(64));
        assert_eq!(v.kind, SemanticVerdictKind::Abstain);
        assert_eq!(v.score, 0.0);
        assert!(v.reasoning.is_none());
        assert_eq!(v.mode, SemanticMode::EmbeddingCache);
    }

    // ---------- Duplicate JSON keys at JSON object level ----------

    #[test]
    fn duplicate_json_key_at_top_level_is_rejected() {
        // The same field name appears twice in the top-level object;
        // serde_json by default would last-write-wins. The custom
        // visitor rejects it before materialization.
        let json = r#"{"schema_version":"cacg.v0","schema_version":"cacg.v0","entries":[]}"#;
        let err = SemanticCache::from_str(json).unwrap_err();
        match err {
            SemanticError::Json(msg) => assert!(
                msg.contains("duplicate key") && msg.contains("schema_version"),
                "expected duplicate-key message; got {msg}",
            ),
            SemanticError::NotRegularFile(_)
            | SemanticError::Read { .. }
            | SemanticError::UnknownSchemaVersion(_)
            | SemanticError::InvalidHashField { .. }
            | SemanticError::InvalidScore { .. }
            | SemanticError::DuplicateSemanticKey { .. }
            | SemanticError::CanonicalSerialize(_) => {
                panic!("expected SemanticError::Json; got {err:?}")
            }
        }
    }

    #[test]
    fn duplicate_json_key_at_entry_level_is_rejected() {
        let a = hash_a();
        let b = hash_b();
        let json = format!(
            r#"{{
              "schema_version": "cacg.v0",
              "entries": [
                {{
                  "chunk_hash": "{a}",
                  "chunk_hash": "{a}",
                  "claim_window_hash": "{b}",
                  "verdict": "pass",
                  "score": 0.92
                }}
              ]
            }}"#,
        );
        let err = SemanticCache::from_str(&json).unwrap_err();
        match err {
            SemanticError::Json(msg) => assert!(
                msg.contains("duplicate key") && msg.contains("chunk_hash"),
                "expected duplicate-key message; got {msg}",
            ),
            SemanticError::NotRegularFile(_)
            | SemanticError::Read { .. }
            | SemanticError::UnknownSchemaVersion(_)
            | SemanticError::InvalidHashField { .. }
            | SemanticError::InvalidScore { .. }
            | SemanticError::DuplicateSemanticKey { .. }
            | SemanticError::CanonicalSerialize(_) => {
                panic!("expected SemanticError::Json; got {err:?}")
            }
        }
    }

    // ---------- Unknown fields ----------

    #[test]
    fn unknown_top_level_field_is_rejected() {
        let json = r#"{"schema_version":"cacg.v0","entries":[],"extra":"nope"}"#;
        let err = SemanticCache::from_str(json).unwrap_err();
        match err {
            SemanticError::Json(msg) => assert!(
                msg.contains("unknown field") && msg.contains("extra"),
                "expected unknown-field message; got {msg}",
            ),
            SemanticError::NotRegularFile(_)
            | SemanticError::Read { .. }
            | SemanticError::UnknownSchemaVersion(_)
            | SemanticError::InvalidHashField { .. }
            | SemanticError::InvalidScore { .. }
            | SemanticError::DuplicateSemanticKey { .. }
            | SemanticError::CanonicalSerialize(_) => {
                panic!("expected SemanticError::Json; got {err:?}")
            }
        }
    }

    #[test]
    fn unknown_entry_field_is_rejected() {
        let a = hash_a();
        let b = hash_b();
        let json = format!(
            r#"{{
              "schema_version": "cacg.v0",
              "entries": [
                {{
                  "chunk_hash": "{a}",
                  "claim_window_hash": "{b}",
                  "verdict": "pass",
                  "score": 0.92,
                  "evidence": "should not parse"
                }}
              ]
            }}"#,
        );
        let err = SemanticCache::from_str(&json).unwrap_err();
        match err {
            SemanticError::Json(msg) => assert!(
                msg.contains("unknown field") && msg.contains("evidence"),
                "expected unknown-field message; got {msg}",
            ),
            SemanticError::NotRegularFile(_)
            | SemanticError::Read { .. }
            | SemanticError::UnknownSchemaVersion(_)
            | SemanticError::InvalidHashField { .. }
            | SemanticError::InvalidScore { .. }
            | SemanticError::DuplicateSemanticKey { .. }
            | SemanticError::CanonicalSerialize(_) => {
                panic!("expected SemanticError::Json; got {err:?}")
            }
        }
    }

    // ---------- Structural validators ----------

    #[test]
    fn schema_version_mismatch_is_rejected() {
        let json = r#"{"schema_version":"cacg.v1","entries":[]}"#;
        let err = SemanticCache::from_str(json).unwrap_err();
        match err {
            SemanticError::UnknownSchemaVersion(v) => assert_eq!(v, "cacg.v1"),
            SemanticError::NotRegularFile(_)
            | SemanticError::Read { .. }
            | SemanticError::Json(_)
            | SemanticError::InvalidHashField { .. }
            | SemanticError::InvalidScore { .. }
            | SemanticError::DuplicateSemanticKey { .. }
            | SemanticError::CanonicalSerialize(_) => {
                panic!("expected UnknownSchemaVersion; got {err:?}")
            }
        }
    }

    #[test]
    fn non_hex_chunk_hash_is_rejected() {
        let bad_hash = "z".repeat(64);
        let b = hash_b();
        let json = format!(
            r#"{{"schema_version":"cacg.v0","entries":[{{"chunk_hash":"{bad_hash}","claim_window_hash":"{b}","verdict":"pass","score":0.5}}]}}"#,
        );
        let err = SemanticCache::from_str(&json).unwrap_err();
        match err {
            SemanticError::InvalidHashField { field, .. } => assert_eq!(field, "chunk_hash"),
            SemanticError::NotRegularFile(_)
            | SemanticError::Read { .. }
            | SemanticError::Json(_)
            | SemanticError::UnknownSchemaVersion(_)
            | SemanticError::InvalidScore { .. }
            | SemanticError::DuplicateSemanticKey { .. }
            | SemanticError::CanonicalSerialize(_) => {
                panic!("expected InvalidHashField; got {err:?}")
            }
        }
    }

    #[test]
    fn uppercase_hex_chunk_hash_is_rejected() {
        let bad_hash = "A".repeat(64);
        let b = hash_b();
        let json = format!(
            r#"{{"schema_version":"cacg.v0","entries":[{{"chunk_hash":"{bad_hash}","claim_window_hash":"{b}","verdict":"pass","score":0.5}}]}}"#,
        );
        let err = SemanticCache::from_str(&json).unwrap_err();
        assert!(matches!(
            err,
            SemanticError::InvalidHashField {
                field: "chunk_hash",
                ..
            }
        ));
    }

    #[test]
    fn short_hash_is_rejected() {
        let bad_hash = "a".repeat(63);
        let b = hash_b();
        let json = format!(
            r#"{{"schema_version":"cacg.v0","entries":[{{"chunk_hash":"{bad_hash}","claim_window_hash":"{b}","verdict":"pass","score":0.5}}]}}"#,
        );
        let err = SemanticCache::from_str(&json).unwrap_err();
        assert!(matches!(
            err,
            SemanticError::InvalidHashField {
                field: "chunk_hash",
                ..
            }
        ));
    }

    #[test]
    fn invalid_verdict_string_is_rejected() {
        let a = hash_a();
        let b = hash_b();
        let json = format!(
            r#"{{"schema_version":"cacg.v0","entries":[{{"chunk_hash":"{a}","claim_window_hash":"{b}","verdict":"maybe","score":0.5}}]}}"#,
        );
        let err = SemanticCache::from_str(&json).unwrap_err();
        match err {
            SemanticError::Json(msg) => assert!(
                msg.contains("unknown variant") || msg.contains("invalid"),
                "expected serde enum-rejection message; got {msg}",
            ),
            SemanticError::NotRegularFile(_)
            | SemanticError::Read { .. }
            | SemanticError::UnknownSchemaVersion(_)
            | SemanticError::InvalidHashField { .. }
            | SemanticError::InvalidScore { .. }
            | SemanticError::DuplicateSemanticKey { .. }
            | SemanticError::CanonicalSerialize(_) => {
                panic!("expected SemanticError::Json; got {err:?}")
            }
        }
    }

    #[test]
    fn score_out_of_range_is_rejected() {
        let a = hash_a();
        let b = hash_b();
        let json = format!(
            r#"{{"schema_version":"cacg.v0","entries":[{{"chunk_hash":"{a}","claim_window_hash":"{b}","verdict":"pass","score":1.5}}]}}"#,
        );
        let err = SemanticCache::from_str(&json).unwrap_err();
        match err {
            SemanticError::InvalidScore { reason, .. } => assert_eq!(reason, "out of range"),
            SemanticError::NotRegularFile(_)
            | SemanticError::Read { .. }
            | SemanticError::Json(_)
            | SemanticError::UnknownSchemaVersion(_)
            | SemanticError::InvalidHashField { .. }
            | SemanticError::DuplicateSemanticKey { .. }
            | SemanticError::CanonicalSerialize(_) => {
                panic!("expected InvalidScore; got {err:?}")
            }
        }
    }

    #[test]
    fn negative_score_is_rejected() {
        let a = hash_a();
        let b = hash_b();
        let json = format!(
            r#"{{"schema_version":"cacg.v0","entries":[{{"chunk_hash":"{a}","claim_window_hash":"{b}","verdict":"pass","score":-0.1}}]}}"#,
        );
        let err = SemanticCache::from_str(&json).unwrap_err();
        assert!(matches!(
            err,
            SemanticError::InvalidScore {
                reason: "out of range",
                ..
            }
        ));
    }

    // ---------- Duplicate semantic keys across entries ----------

    #[test]
    fn duplicate_chunk_claim_pair_across_entries_is_rejected() {
        let a = hash_a();
        let b = hash_b();
        let json = format!(
            r#"{{
              "schema_version":"cacg.v0",
              "entries":[
                {{"chunk_hash":"{a}","claim_window_hash":"{b}","verdict":"pass","score":0.9}},
                {{"chunk_hash":"{a}","claim_window_hash":"{b}","verdict":"fail","score":0.1}}
              ]
            }}"#,
        );
        let err = SemanticCache::from_str(&json).unwrap_err();
        match err {
            SemanticError::DuplicateSemanticKey {
                chunk_hash,
                claim_window_hash,
            } => {
                assert_eq!(chunk_hash, "a".repeat(64));
                assert_eq!(claim_window_hash, "b".repeat(64));
            }
            SemanticError::NotRegularFile(_)
            | SemanticError::Read { .. }
            | SemanticError::Json(_)
            | SemanticError::UnknownSchemaVersion(_)
            | SemanticError::InvalidHashField { .. }
            | SemanticError::InvalidScore { .. }
            | SemanticError::CanonicalSerialize(_) => {
                panic!("expected DuplicateSemanticKey; got {err:?}")
            }
        }
    }

    // ---------- SemanticCache::load shape check ----------

    #[test]
    fn load_rejects_missing_path() {
        let tmp = TempDir::new().unwrap();
        let missing = tmp.path().join("does_not_exist.json");
        let err = SemanticCache::load(&missing).unwrap_err();
        match err {
            SemanticError::NotRegularFile(p) => assert_eq!(p, missing),
            SemanticError::Read { .. }
            | SemanticError::Json(_)
            | SemanticError::UnknownSchemaVersion(_)
            | SemanticError::InvalidHashField { .. }
            | SemanticError::InvalidScore { .. }
            | SemanticError::DuplicateSemanticKey { .. }
            | SemanticError::CanonicalSerialize(_) => {
                panic!("expected NotRegularFile; got {err:?}")
            }
        }
    }

    #[test]
    fn load_rejects_directory_in_place_of_file() {
        let tmp = TempDir::new().unwrap();
        let as_dir = tmp.path().join("cache.json");
        fs::create_dir(&as_dir).unwrap();
        let err = SemanticCache::load(&as_dir).unwrap_err();
        assert!(matches!(err, SemanticError::NotRegularFile(_)));
    }

    #[test]
    fn load_reads_valid_cache_from_disk() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("cache.json");
        fs::write(&path, minimal_cache_json()).unwrap();
        let cache = SemanticCache::load(&path).unwrap();
        assert_eq!(cache.entries.len(), 1);
        let v = cache.lookup(&hash_a(), &hash_b());
        assert_eq!(v.kind, SemanticVerdictKind::Pass);
    }

    // ---------- Malformed JSON ----------

    #[test]
    fn malformed_json_is_rejected() {
        let err = SemanticCache::from_str("{ not json").unwrap_err();
        assert!(matches!(err, SemanticError::Json(_)));
    }

    // ---------- Display + Error trait sanity ----------

    #[test]
    fn semantic_error_display_includes_field_name() {
        let err = SemanticError::InvalidHashField {
            field: "chunk_hash",
            value: "nope".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains("chunk_hash"));
        assert!(s.contains("nope"));
    }

    // ---------- Recursive duplicate-key scanner (every JSON depth) ----------

    #[test]
    fn duplicate_key_inside_unknown_top_level_field_is_rejected_by_recursive_scanner() {
        // The typed visitor would reject the unknown top-level field
        // "extra" without walking into it; the recursive scanner runs
        // first and sees the nested duplicate at any depth.
        let json = r#"{
              "schema_version":"cacg.v0",
              "entries":[],
              "extra":{"a":1,"a":2}
            }"#;
        let err = SemanticCache::from_str(json).unwrap_err();
        match err {
            SemanticError::Json(msg) => {
                assert!(
                    msg.contains("duplicate key") && msg.contains("\"a\""),
                    "expected recursive duplicate-key message; got {msg}",
                );
                assert!(
                    msg.contains("semantic cache JSON"),
                    "expected the recursive-scanner-specific message; got {msg}",
                );
            }
            SemanticError::NotRegularFile(_)
            | SemanticError::Read { .. }
            | SemanticError::UnknownSchemaVersion(_)
            | SemanticError::InvalidHashField { .. }
            | SemanticError::InvalidScore { .. }
            | SemanticError::DuplicateSemanticKey { .. }
            | SemanticError::CanonicalSerialize(_) => {
                panic!("expected SemanticError::Json; got {err:?}")
            }
        }
    }

    #[test]
    fn duplicate_key_inside_entry_level_unknown_nested_field_is_rejected_by_recursive_scanner() {
        let a = hash_a();
        let b = hash_b();
        let json = format!(
            r#"{{
              "schema_version":"cacg.v0",
              "entries":[
                {{
                  "chunk_hash":"{a}",
                  "claim_window_hash":"{b}",
                  "verdict":"pass",
                  "score":0.5,
                  "evidence":{{"k":1,"k":2}}
                }}
              ]
            }}"#,
        );
        let err = SemanticCache::from_str(&json).unwrap_err();
        match err {
            SemanticError::Json(msg) => {
                assert!(
                    msg.contains("duplicate key") && msg.contains("\"k\""),
                    "expected recursive duplicate-key message for nested unknown field; got {msg}",
                );
                assert!(
                    msg.contains("semantic cache JSON"),
                    "expected the recursive-scanner-specific message; got {msg}",
                );
            }
            SemanticError::NotRegularFile(_)
            | SemanticError::Read { .. }
            | SemanticError::UnknownSchemaVersion(_)
            | SemanticError::InvalidHashField { .. }
            | SemanticError::InvalidScore { .. }
            | SemanticError::DuplicateSemanticKey { .. }
            | SemanticError::CanonicalSerialize(_) => {
                panic!("expected SemanticError::Json; got {err:?}")
            }
        }
    }

    #[test]
    fn duplicate_key_deeply_nested_under_unknown_array_is_rejected_by_recursive_scanner() {
        // Belt-and-braces: ensure the scanner recurses into arrays
        // too, not just objects.
        let json = r#"{
              "schema_version":"cacg.v0",
              "entries":[],
              "extra":[1,2,{"x":1,"x":2},3]
            }"#;
        let err = SemanticCache::from_str(json).unwrap_err();
        match err {
            SemanticError::Json(msg) => {
                assert!(
                    msg.contains("duplicate key") && msg.contains("\"x\""),
                    "expected recursive duplicate-key message inside an array element; got {msg}",
                );
            }
            SemanticError::NotRegularFile(_)
            | SemanticError::Read { .. }
            | SemanticError::UnknownSchemaVersion(_)
            | SemanticError::InvalidHashField { .. }
            | SemanticError::InvalidScore { .. }
            | SemanticError::DuplicateSemanticKey { .. }
            | SemanticError::CanonicalSerialize(_) => {
                panic!("expected SemanticError::Json; got {err:?}")
            }
        }
    }

    // ---------- Canonical semantic-cache serializer ----------

    #[test]
    fn to_canonical_json_matches_documented_envelope() {
        let cache = SemanticCache::from_str(&minimal_cache_json()).unwrap();
        // Spec § 4 + Python `json.dumps(..., sort_keys=True)` puts
        // top-level keys in alphabetical order (entries < schema_version),
        // and `cacg_core::canonical_json` does the same. The entry's
        // inner keys sort alphabetically too.
        let a = hash_a();
        let b = hash_b();
        let expected = format!(
            r#"{{"entries":[{{"chunk_hash":"{a}","claim_window_hash":"{b}","score":0.92,"verdict":"pass"}}],"schema_version":"cacg.v0"}}"#,
        );
        assert_eq!(cache.to_canonical_json().unwrap(), expected);
    }

    #[test]
    fn canonical_serialize_round_trip_is_byte_stable() {
        // parse → serialize → parse → serialize: both serializations
        // must produce byte-identical output.
        let initial = SemanticCache::from_str(&minimal_cache_json()).unwrap();
        let first = initial.to_canonical_json().unwrap();
        let reparsed = SemanticCache::from_str(&first).unwrap();
        let second = reparsed.to_canonical_json().unwrap();
        assert_eq!(
            first, second,
            "canonical serialization is not round-trip stable"
        );
    }

    #[test]
    fn canonical_serialize_preserves_entries_order() {
        // Python `dump_semantic_cache` does NOT sort entries; the
        // Rust port must preserve load order verbatim.
        let a = hash_a();
        let b = hash_b();
        let c = "c".repeat(64);
        let d = "d".repeat(64);
        let json = format!(
            r#"{{
              "schema_version":"cacg.v0",
              "entries":[
                {{"chunk_hash":"{c}","claim_window_hash":"{d}","verdict":"fail","score":0.1}},
                {{"chunk_hash":"{a}","claim_window_hash":"{b}","verdict":"pass","score":0.9}}
              ]
            }}"#,
        );
        let cache = SemanticCache::from_str(&json).unwrap();
        let out = cache.to_canonical_json().unwrap();
        // c-d entry must appear BEFORE a-b entry even though c > a
        // alphabetically; the contract is "preserve load order"
        // (Python `dump_semantic_cache` matches).
        let c_pos = out.find(&c).unwrap();
        let a_pos = out.find(&a).unwrap();
        assert!(
            c_pos < a_pos,
            "entries order not preserved: c@{c_pos} should precede a@{a_pos}"
        );
    }

    // ---------- Schema-version evolution ----------

    #[test]
    fn schema_version_evolution_positive_cacg_v0_loads() {
        // Same content as from_str_accepts_minimal_valid_cache but
        // anchored on the schema-version evolution contract.
        let cache = SemanticCache::from_str(&minimal_cache_json()).unwrap();
        assert_eq!(cache.schema_version, SEMANTIC_CACHE_SCHEMA_VERSION);
    }

    #[test]
    fn schema_version_evolution_negative_unknown_version_has_stable_reason_text() {
        let json = r#"{"schema_version":"cacg.v1","entries":[]}"#;
        let err = SemanticCache::from_str(json).unwrap_err();
        let display = err.to_string();
        // The reason text must mention the offending version, the
        // expected version, and the schema_version field name so
        // operators can diagnose the rejection without grep-walking
        // the Rust source.
        assert!(
            display.contains("schema_version"),
            "expected reason text to mention the field; got {display}",
        );
        assert!(
            display.contains("cacg.v1"),
            "expected reason text to mention the offending version; got {display}",
        );
        assert!(
            display.contains("cacg.v0"),
            "expected reason text to mention the expected version; got {display}",
        );
    }

    #[test]
    fn schema_version_evolution_negative_unknown_field_has_distinct_stable_reason_text() {
        let json = r#"{"schema_version":"cacg.v0","entries":[],"extra":"nope"}"#;
        let err = SemanticCache::from_str(json).unwrap_err();
        let display = err.to_string();
        assert!(
            display.contains("unknown field") && display.contains("extra"),
            "expected unknown-field reason text; got {display}",
        );
        // The unknown-field rejection must NOT collide with the
        // schema-version-mismatch reason text.
        assert!(
            !display.contains("schema_version"),
            "unknown-field reason text must be DISTINCT from schema-version-mismatch; got {display}",
        );

        // Belt-and-braces: confirm the two negative-path Display
        // strings are demonstrably different.
        let version_err = SemanticError::UnknownSchemaVersion("cacg.v1".to_string()).to_string();
        assert_ne!(
            display, version_err,
            "schema-version-mismatch and unknown-field rejections must produce distinct reason text",
        );
    }

    // ---------- claim_window_hash 5-class golden vectors ----------

    #[test]
    fn claim_window_hash_ligature_fi_matches_python_oracle() {
        // "ofﬁce file" — the Latin small ligature ﬁ (U+FB01) is
        // replaced with "fi" by the normalize_text ligature step
        // BEFORE NFC composition.
        assert_eq!(
            claim_window_hash("of\u{FB01}ce file"),
            "8169a01c24806359777ff0f5b8ba513d42c9e21e956a79b589ac6cd5cf759937",
        );
    }

    #[test]
    fn claim_window_hash_ligature_equivalence_collapses_to_same_hash() {
        // "ofﬁce" and "office" hash identically after normalize_text.
        let with_ligature = claim_window_hash("of\u{FB01}ce");
        let without = claim_window_hash("office");
        assert_eq!(
            with_ligature, without,
            "ligature ﬁ must collapse to fi before hashing",
        );
        assert_eq!(
            with_ligature,
            "5cc3f82838ba7260203e4590ce03d00e1663d41f6a5167144f5c95d6be2166a0",
        );
    }

    #[test]
    fn claim_window_hash_nfc_combining_marks_matches_python_oracle() {
        // "Cafe\u{0301}" (decomposed: 'C','a','f','e','◌́') NFC-composes
        // to "Café" (composed 'C','a','f','é'). The Python oracle
        // hashes the composed form.
        assert_eq!(
            claim_window_hash("Cafe\u{0301}"),
            "73473dcc12b763085904a5279d048c4d5b3b008c46f1f32443b99de04aa83a14",
        );
    }

    #[test]
    fn claim_window_hash_nfc_combining_marks_equivalence_collapses_to_same_hash() {
        // The decomposed and pre-composed forms must hash identically.
        assert_eq!(
            claim_window_hash("Cafe\u{0301}"),
            claim_window_hash("Caf\u{00E9}"),
        );
    }

    #[test]
    fn claim_window_hash_crlf_lf_equivalence_matches_python_oracle() {
        let crlf = claim_window_hash("line one\r\nline two");
        let lf = claim_window_hash("line one\nline two");
        assert_eq!(crlf, lf, "CRLF and LF must collapse to identical hashes");
        assert_eq!(
            crlf,
            "e490ed577595f61675761642aa202c2f1f59ef292f58c24fe0b431b05eeb86ec",
        );
    }

    #[test]
    fn claim_window_hash_supplementary_plane_matches_python_oracle() {
        // Astral-plane codepoints: 😀 (U+1F600) + 𠀀 (U+20000). The
        // Rust port hashes the normalized UTF-8 bytes, NOT
        // UTF-16-encoded surrogate pairs.
        assert_eq!(
            claim_window_hash("emoji 😀 and CJK 𠀀"),
            "24c4ef6df24054f42f626403e11e78a00c58b009970fcce360c49ee3ea91f85c",
        );
    }

    /// Tests for `CACG_SEMANTIC_LOAD_TRACE` instrumentation.
    ///
    /// `SemanticCache::emit_load_trace` reads the env var at call
    /// time. A concurrent test that toggles the env var would race
    /// against every other test in this crate that calls the public
    /// `SemanticCache::load` — those callers also read the env var
    /// inside the wrapper. The synchronization strategy is therefore
    /// crate-wide, not module-local: tests acquire
    /// `crate::LOAD_TRACE_TEST_LOCK` (held by `SemanticCache::load`
    /// under `cfg(test)`), then call `load_inner` + `emit_load_trace`
    /// directly to exercise the same code paths without recursing
    /// back into the lock-taking wrapper (the mutex is non-reentrant
    /// so a recursive `lock()` would deadlock).
    ///
    /// See `BL-20260522-one-mutex-per-process-global-in-tests`.
    mod load_trace {
        use super::*;
        use tempfile::TempDir;

        /// RAII helper that sets the env var on construction and
        /// removes it on drop, so a test failure does not leak the
        /// env-var setting into a sibling test.
        struct EnvGuard {
            key: &'static str,
        }
        impl EnvGuard {
            fn set(key: &'static str, value: &str) -> Self {
                std::env::set_var(key, value);
                Self { key }
            }
        }
        impl Drop for EnvGuard {
            fn drop(&mut self) {
                std::env::remove_var(self.key);
            }
        }

        fn write_minimal_cache(dir: &Path) -> PathBuf {
            let path = dir.join("cache.json");
            let body = serde_json::json!({
                "schema_version": SEMANTIC_CACHE_SCHEMA_VERSION,
                "entries": [
                    {
                        "chunk_hash": format!("{:064}", 1),
                        "claim_window_hash": format!("{:064}", 2),
                        "verdict": "pass",
                        "score": 0.5,
                    }
                ],
            });
            fs::write(&path, serde_json::to_string(&body).unwrap()).unwrap();
            path
        }

        // Helper that mirrors `SemanticCache::load` (load_inner +
        // emit_load_trace) without re-acquiring the test lock. Tests
        // already hold `crate::LOAD_TRACE_TEST_LOCK` for the full
        // duration of their env-var manipulation, so calling the
        // public `load` wrapper here would deadlock on the non-
        // reentrant mutex.
        fn load_under_held_lock(path: &Path) -> Result<SemanticCache, SemanticError> {
            let result = SemanticCache::load_inner(path);
            SemanticCache::emit_load_trace(path, result.as_ref().ok().map(|c| c.entries.len()));
            result
        }

        #[test]
        fn load_trace_silent_when_env_var_unset() {
            let _guard = crate::LOAD_TRACE_TEST_LOCK
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            std::env::remove_var("CACG_SEMANTIC_LOAD_TRACE");
            let dir = TempDir::new().unwrap();
            let cache = write_minimal_cache(dir.path());
            let trace = dir.path().join("trace.txt");
            load_under_held_lock(&cache).expect("clean cache load");
            assert!(
                !trace.is_file(),
                "trace file must not be created when env var is absent",
            );
        }

        #[test]
        fn load_trace_appends_load_ok_on_success() {
            let _guard = crate::LOAD_TRACE_TEST_LOCK
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let dir = TempDir::new().unwrap();
            let cache = write_minimal_cache(dir.path());
            let trace = dir.path().join("trace.txt");
            let _env = EnvGuard::set(
                "CACG_SEMANTIC_LOAD_TRACE",
                trace.to_str().expect("utf-8 path"),
            );
            load_under_held_lock(&cache).expect("clean cache load");
            let content = fs::read_to_string(&trace).expect("trace must be written");
            assert_eq!(
                content.lines().count(),
                1,
                "exactly one trace line on a single load; got: {content:?}",
            );
            assert!(
                content.starts_with("load_ok "),
                "successful load must emit `load_ok` line; got: {content:?}",
            );
            assert!(
                content.contains(cache.to_str().unwrap()),
                "trace line must include the cache path; got: {content:?}",
            );
            assert!(
                content.trim_end().ends_with(" 1"),
                "trace line must end with the entry count (1); got: {content:?}",
            );
        }

        #[test]
        fn load_trace_appends_load_err_on_failure() {
            let _guard = crate::LOAD_TRACE_TEST_LOCK
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let dir = TempDir::new().unwrap();
            let cache = dir.path().join("does_not_exist.json");
            let trace = dir.path().join("trace.txt");
            let _env = EnvGuard::set(
                "CACG_SEMANTIC_LOAD_TRACE",
                trace.to_str().expect("utf-8 path"),
            );
            let err = load_under_held_lock(&cache).unwrap_err();
            assert!(matches!(err, SemanticError::NotRegularFile(_)));
            let content = fs::read_to_string(&trace).expect("trace must be written");
            assert_eq!(content.lines().count(), 1);
            assert!(
                content.starts_with("load_err "),
                "failed load must emit `load_err` line; got: {content:?}",
            );
        }
    }
}
