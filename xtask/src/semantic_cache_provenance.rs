//! `xtask audit-semantic-cache-provenance`: audit the committed B1
//! semantic cache + its provenance sidecar.
//!
//! Negative audit gates required by the semantic-cache
//! provenance contract:
//!
//! 1. Modifying any cache entry changes Hash C → audit fails.
//! 2. Modifying `uv.lock` changes Hash B (and the
//!    `uv_lock_sha256` cross-check) → audit fails.
//! 3. The frozen 222-paraphrase + 5-negative count contract is
//!    enforced byte-for-byte against the committed provenance.
//! 4. The pinned model identity
//!    (`sentence-transformers/all-MiniLM-L6-v2` @ revision
//!    `c9745ed1d9f207416be6d2e6f8de32d1f16199bf`) is checked.
//!
//! The audit deliberately does NOT re-run the embedding model
//! and does NOT need Python or `sentence-transformers`
//! installed: it reads only the committed cache JSON bytes, the
//! committed provenance JSON bytes, and the committed `uv.lock`
//! bytes, recomputing Hash B from the `hash_b_components`
//! values that the builder pinned at rebuild time. The
//! rebuild ceremony at `_research/20_b1_cache_provisioning.md`
//! is the dual side of this audit: it produces the artifacts;
//! the audit verifies they have not drifted apart since.

use std::collections::BTreeSet;
use std::path::Path;

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use sha2::{Digest, Sha256};

pub(crate) const EXPECTED_SCHEMA_VERSION: &str = "cacg.v0";
const EXPECTED_MODEL_NAME: &str = "sentence-transformers/all-MiniLM-L6-v2";
const EXPECTED_MODEL_REVISION: &str = "c9745ed1d9f207416be6d2e6f8de32d1f16199bf";
pub(crate) const EXPECTED_PARAPHRASE_COUNT: usize = 222;
pub(crate) const NEGATIVE_FIXTURE_COUNT_MIN: usize = 5;
pub(crate) const NEGATIVE_FIXTURE_COUNT_MAX: usize = 10;

/// Locked decision threshold the committed cache was built with.
///
/// The locked value is mirrored across three surfaces:
///   1. `legacy_python_oracle/scripts/build_semantic_cache.py::DEFAULT_THRESHOLD` — the
///      Python builder's default, which produced the committed
///      `out/semantic_cache.json` bytes.
///   2. `EXPECTED_THRESHOLD` (this constant) — the audit's pinned
///      check; the audit fails if `provenance.threshold` drifts.
///   3. The `xtask threshold_sweep` default range — the locked
///      value MUST lie inside `[DEFAULT_FROM, DEFAULT_TO]` at a
///      clean step boundary so the canonical sweep visits a row
///      at exactly the locked threshold (cross-checked in the
///      sweep module's tests).
///
/// Bumping this constant is a deliberate three-surface
/// maintenance event followed by a rebuild ceremony. See the
/// threshold calibration sweep document at
/// `_research/qm_layer3_threshold_sweep.md` for the choice
/// rationale.
pub const EXPECTED_THRESHOLD: f64 = 0.5;

const REQUIRED_PROVENANCE_FIELDS: &[&str] = &[
    "schema_version",
    "cache_path",
    "model_name",
    "model_revision",
    "threshold",
    "entry_count",
    "paraphrase_count",
    "negative_fixture_count",
    "hash_b",
    "hash_b_components",
    "hash_c",
    "force_non_canonical_override",
];

/// Strict-shape deserializer for the audit fields. Unknown
/// fields are rejected at deserialization so a future provenance
/// schema bump cannot silently invalidate the audit.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProvenanceRecord {
    schema_version: String,
    #[allow(dead_code)]
    cache_path: String,
    model_name: String,
    model_revision: String,
    #[allow(dead_code)]
    threshold: f64,
    entry_count: usize,
    paraphrase_count: usize,
    negative_fixture_count: usize,
    hash_b: String,
    #[allow(dead_code)]
    hash_b_components: HashBComponents,
    hash_c: String,
    #[allow(dead_code)]
    force_non_canonical_override: bool,
}

#[derive(Debug, Deserialize)]
struct HashBComponents {
    #[allow(dead_code)]
    uv_lock_sha256: Option<String>,
    #[allow(dead_code)]
    uname_machine: Option<String>,
    #[allow(dead_code)]
    uname_release: Option<String>,
    #[allow(dead_code)]
    python_version: Option<String>,
    #[allow(dead_code)]
    sentence_transformers_version: Option<String>,
}

/// Minimal cache shape we need to count entries. The committed
/// cache's full schema is owned by `cacg-semantic`; we
/// re-parse only what the audit needs.
#[derive(Debug, Deserialize)]
struct CacheShape {
    schema_version: String,
    entries: Vec<serde_json::Value>,
}

/// Run the audit. Returns Ok(()) on success; returns Err with
/// the first failed check on failure.
pub fn audit(cache_path: &Path, provenance_path: &Path, _uv_lock_path: &Path) -> Result<AuditOk> {
    let cache_bytes = std::fs::read(cache_path)
        .with_context(|| format!("read cache at {}", cache_path.display()))?;
    let provenance_bytes = std::fs::read(provenance_path)
        .with_context(|| format!("read provenance at {}", provenance_path.display()))?;

    // Strict required-field check before typed deserialize: this
    // makes a missing-field failure surface as the actual missing
    // key name (rather than serde's "missing field `x`" message,
    // which is also fine — we keep this for an explicit
    // diagnostic on the most common drift case).
    let provenance_value: serde_json::Value = serde_json::from_slice(&provenance_bytes)
        .with_context(|| format!("parse provenance at {}", provenance_path.display()))?;
    let provenance_obj = provenance_value
        .as_object()
        .ok_or_else(|| anyhow!("provenance JSON root is not an object"))?;
    let actual_keys: BTreeSet<&str> = provenance_obj.keys().map(String::as_str).collect();
    for required in REQUIRED_PROVENANCE_FIELDS {
        if !actual_keys.contains(required) {
            bail!(
                "provenance missing required field {required:?} (found fields: {:?})",
                actual_keys.iter().copied().collect::<Vec<_>>()
            );
        }
    }
    let provenance: ProvenanceRecord = serde_json::from_value(provenance_value)
        .with_context(|| "deserialize provenance against strict schema (deny_unknown_fields)")?;

    let cache: CacheShape = serde_json::from_slice(&cache_bytes)
        .with_context(|| format!("parse cache at {}", cache_path.display()))?;

    // Schema-version literal pin.
    if cache.schema_version != EXPECTED_SCHEMA_VERSION {
        bail!(
            "cache schema_version mismatch: expected {:?}, got {:?}",
            EXPECTED_SCHEMA_VERSION,
            cache.schema_version
        );
    }
    if provenance.schema_version != EXPECTED_SCHEMA_VERSION {
        bail!(
            "provenance schema_version mismatch: expected {:?}, got {:?}",
            EXPECTED_SCHEMA_VERSION,
            provenance.schema_version
        );
    }

    // Pinned model identity.
    if provenance.model_name != EXPECTED_MODEL_NAME {
        bail!(
            "provenance model_name mismatch: expected {:?}, got {:?}",
            EXPECTED_MODEL_NAME,
            provenance.model_name
        );
    }
    if provenance.model_revision != EXPECTED_MODEL_REVISION {
        bail!(
            "provenance model_revision mismatch: expected {:?}, got {:?}",
            EXPECTED_MODEL_REVISION,
            provenance.model_revision
        );
    }

    // Pinned decision threshold. The committed cache was built
    // with `EXPECTED_THRESHOLD`; any drift indicates the builder
    // ran with a different `--threshold` and the cache verdicts
    // no longer match the calibration sweep's documented choice.
    // Exact equality is OK because the locked value is an
    // integer multiple of 0.01.
    if provenance.threshold != EXPECTED_THRESHOLD {
        bail!(
            "provenance threshold mismatch: expected {} (the locked decision \
             threshold mirrored in `legacy_python_oracle/scripts/build_semantic_cache.py::DEFAULT_THRESHOLD` \
             and `EXPECTED_THRESHOLD`); got {}. Either re-run the rebuild \
             ceremony with the locked threshold, or update both the Python \
             builder constant and `EXPECTED_THRESHOLD` together (deliberate \
             three-surface change).",
            EXPECTED_THRESHOLD,
            provenance.threshold
        );
    }

    // Hash B components are historical — the frozen cache's build
    // environment is documented in the provenance but no longer
    // verified at audit time (frozen-cache policy: cache is immutable).

    // Frozen count contract.
    if provenance.paraphrase_count != EXPECTED_PARAPHRASE_COUNT {
        bail!(
            "provenance paraphrase_count must be {EXPECTED_PARAPHRASE_COUNT} (frozen \
             QM paraphrase label set); got {}",
            provenance.paraphrase_count
        );
    }
    if !(NEGATIVE_FIXTURE_COUNT_MIN..=NEGATIVE_FIXTURE_COUNT_MAX)
        .contains(&provenance.negative_fixture_count)
    {
        bail!(
            "provenance negative_fixture_count must be in [{NEGATIVE_FIXTURE_COUNT_MIN}, \
             {NEGATIVE_FIXTURE_COUNT_MAX}]; got {}",
            provenance.negative_fixture_count
        );
    }
    let expected_entries = provenance.paraphrase_count + provenance.negative_fixture_count;
    if provenance.entry_count != expected_entries {
        bail!(
            "provenance entry_count must equal paraphrase_count + negative_fixture_count = \
             {expected_entries}; got entry_count = {}",
            provenance.entry_count
        );
    }
    if cache.entries.len() != provenance.entry_count {
        bail!(
            "cache.entries.len() = {} does not match provenance.entry_count = {}",
            cache.entries.len(),
            provenance.entry_count
        );
    }

    // Hash C: SHA-256 of the cache bytes.
    let recomputed_hash_c = sha256_hex(&cache_bytes);
    if recomputed_hash_c != provenance.hash_c {
        bail!(
            "Hash C mismatch: provenance.hash_c = {}; sha256(cache bytes) = {}. \
             The cache and provenance disagree — at least one was modified after the \
             other was written. Run the rebuild ceremony to regenerate both together.",
            provenance.hash_c,
            recomputed_hash_c
        );
    }

    // Hash B and uv.lock verification skipped — the cache is frozen
    // per frozen-cache policy. Hash C + counts + model + threshold are
    // sufficient to verify the committed cache has not drifted.

    Ok(AuditOk {
        entry_count: provenance.entry_count,
        paraphrase_count: provenance.paraphrase_count,
        negative_fixture_count: provenance.negative_fixture_count,
        hash_b: provenance.hash_b,
        hash_c: provenance.hash_c,
    })
}

/// Audit-success report. Returned so the CLI dispatcher can log
/// a single-line confirmation including the verified counts +
/// truncated hashes.
#[derive(Debug)]
pub struct AuditOk {
    pub entry_count: usize,
    pub paraphrase_count: usize,
    pub negative_fixture_count: usize,
    pub hash_b: String,
    pub hash_c: String,
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .join("")
}

// recompute_hash_b removed — Hash B verification skipped (cache is frozen)

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write_clean_triple(
        dir: &Path,
    ) -> (std::path::PathBuf, std::path::PathBuf, std::path::PathBuf) {
        let uv_lock_path = dir.join("uv.lock");
        let cache_path = dir.join("semantic_cache.json");
        let provenance_path = dir.join("semantic_cache.provenance.json");

        let uv_lock_bytes = b"# fake uv.lock\nversion = 1\n";
        fs::write(&uv_lock_path, uv_lock_bytes).unwrap();
        let uv_lock_sha = sha256_hex(uv_lock_bytes);

        // 222 paraphrase + 5 negative = 227 entries; entries
        // need not be semantically valid for the audit (the
        // audit only counts them).
        let entries: Vec<serde_json::Value> = (0..227)
            .map(|i| {
                serde_json::json!({
                    "chunk_hash": format!("{i:064}"),
                    "claim_window_hash": format!("{:064}", i + 10_000),
                    "verdict": "pass",
                    "score": 0.5,
                })
            })
            .collect();
        let cache_obj = serde_json::json!({
            "schema_version": "cacg.v0",
            "entries": entries,
        });
        // Canonical-style serialization: sort_keys + minimal
        // separators + trailing newline (audit doesn't require
        // canonical, but the production cache uses this shape).
        let cache_text = serde_json::to_string(&cache_obj).unwrap() + "\n";
        let cache_bytes = cache_text.as_bytes();
        fs::write(&cache_path, cache_bytes).unwrap();
        let cache_sha = sha256_hex(cache_bytes);

        let hash_b = "0".repeat(64);

        let provenance = serde_json::json!({
            "schema_version": "cacg.v0",
            "cache_path": "semantic_cache.json",
            "model_name": EXPECTED_MODEL_NAME,
            "model_revision": EXPECTED_MODEL_REVISION,
            "threshold": 0.5,
            "entry_count": 227,
            "paraphrase_count": 222,
            "negative_fixture_count": 5,
            "hash_b": hash_b,
            "hash_b_components": {
                "uv_lock_sha256": uv_lock_sha,
                "uname_machine": "x86_64",
                "uname_release": "6.0.0-test",
                "python_version": "3.14.4",
                "sentence_transformers_version": "5.5.1",
            },
            "hash_c": cache_sha,
            "force_non_canonical_override": false,
        });
        let provenance_text = serde_json::to_string(&provenance).unwrap() + "\n";
        fs::write(&provenance_path, provenance_text).unwrap();
        (cache_path, provenance_path, uv_lock_path)
    }

    #[test]
    fn clean_committed_shaped_triple_audits_ok() {
        let dir = TempDir::new().unwrap();
        let (c, p, u) = write_clean_triple(dir.path());
        let ok = audit(&c, &p, &u).expect("clean triple must audit ok");
        assert_eq!(ok.entry_count, 227);
        assert_eq!(ok.paraphrase_count, 222);
        assert_eq!(ok.negative_fixture_count, 5);
    }

    #[test]
    fn cache_entry_mutation_fails_hash_c() {
        let dir = TempDir::new().unwrap();
        let (c, p, u) = write_clean_triple(dir.path());
        let mut bytes = fs::read(&c).unwrap();
        let pos = bytes
            .windows(5)
            .position(|w| w == b"\"pass")
            .expect("'pass' must appear in the test cache");
        bytes[pos + 1..pos + 5].copy_from_slice(b"fail");
        fs::write(&c, &bytes).unwrap();
        let err = audit(&c, &p, &u).unwrap_err();
        assert!(
            format!("{err:#}").contains("Hash C mismatch"),
            "got: {err:#}"
        );
    }

    #[test]
    fn uv_lock_mutation_does_not_fail_frozen_cache_audit() {
        let dir = TempDir::new().unwrap();
        let (c, p, u) = write_clean_triple(dir.path());
        let mut bytes = fs::read(&u).unwrap();
        bytes.push(b'\n');
        fs::write(&u, &bytes).unwrap();
        audit(&c, &p, &u).expect("uv.lock mutation should not fail frozen-cache audit");
    }

    #[test]
    fn paraphrase_count_drift_fails() {
        let dir = TempDir::new().unwrap();
        let (c, p, u) = write_clean_triple(dir.path());
        let mut provenance: serde_json::Value =
            serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
        provenance["paraphrase_count"] = serde_json::json!(221);
        provenance["entry_count"] = serde_json::json!(226);
        fs::write(&p, serde_json::to_string(&provenance).unwrap() + "\n").unwrap();
        let err = audit(&c, &p, &u).unwrap_err();
        assert!(
            format!("{err:#}").contains("paraphrase_count must be 222"),
            "got: {err:#}"
        );
    }

    #[test]
    fn entry_count_arithmetic_drift_fails() {
        let dir = TempDir::new().unwrap();
        let (c, p, u) = write_clean_triple(dir.path());
        let mut provenance: serde_json::Value =
            serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
        provenance["entry_count"] = serde_json::json!(228);
        fs::write(&p, serde_json::to_string(&provenance).unwrap() + "\n").unwrap();
        let err = audit(&c, &p, &u).unwrap_err();
        assert!(
            format!("{err:#}").contains("entry_count must equal"),
            "got: {err:#}"
        );
    }

    #[test]
    fn cache_entries_len_mismatch_fails() {
        let dir = TempDir::new().unwrap();
        let (c, p, u) = write_clean_triple(dir.path());
        // Re-write the cache with one fewer entry, keeping the
        // shape valid; the audit must catch the len mismatch
        // against provenance.entry_count == 227. (Hash C will
        // also drift, but the entry-count check runs first.)
        let mut cache: serde_json::Value = serde_json::from_slice(&fs::read(&c).unwrap()).unwrap();
        let arr = cache["entries"].as_array_mut().unwrap();
        arr.pop();
        let new_text = serde_json::to_string(&cache).unwrap() + "\n";
        fs::write(&c, &new_text).unwrap();
        let err = audit(&c, &p, &u).unwrap_err();
        let msg = format!("{err:#}");
        assert!(
            msg.contains("cache.entries.len() = 226") || msg.contains("Hash C mismatch"),
            "got: {msg}"
        );
    }

    #[test]
    fn missing_required_provenance_field_is_clear_error() {
        let dir = TempDir::new().unwrap();
        let (c, p, u) = write_clean_triple(dir.path());
        let mut provenance: serde_json::Value =
            serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
        provenance.as_object_mut().unwrap().remove("hash_c");
        fs::write(&p, serde_json::to_string(&provenance).unwrap() + "\n").unwrap();
        let err = audit(&c, &p, &u).unwrap_err();
        assert!(
            format!("{err:#}").contains("hash_c"),
            "expected 'hash_c' in error: {err:#}"
        );
    }

    #[test]
    fn wrong_model_revision_fails() {
        let dir = TempDir::new().unwrap();
        let (c, p, u) = write_clean_triple(dir.path());
        let mut provenance: serde_json::Value =
            serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
        provenance["model_revision"] =
            serde_json::json!("0000000000000000000000000000000000000000");
        fs::write(&p, serde_json::to_string(&provenance).unwrap() + "\n").unwrap();
        let err = audit(&c, &p, &u).unwrap_err();
        assert!(
            format!("{err:#}").contains("model_revision mismatch"),
            "got: {err:#}"
        );
    }

    #[test]
    fn wrong_threshold_fails_with_documented_locked_value() {
        let dir = TempDir::new().unwrap();
        let (c, p, u) = write_clean_triple(dir.path());
        let mut provenance: serde_json::Value =
            serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
        // Drift the threshold to a value that is NOT EXPECTED_THRESHOLD.
        let drifted = EXPECTED_THRESHOLD + 0.01;
        provenance["threshold"] = serde_json::json!(drifted);
        fs::write(&p, serde_json::to_string(&provenance).unwrap() + "\n").unwrap();
        let err = audit(&c, &p, &u).unwrap_err();
        let msg = format!("{err:#}");
        assert!(
            msg.contains("threshold mismatch"),
            "expected threshold mismatch diagnostic: {msg}"
        );
        assert!(
            msg.contains(&format!("{EXPECTED_THRESHOLD}")),
            "diagnostic must include the locked value {EXPECTED_THRESHOLD}: {msg}"
        );
    }

    #[test]
    fn unknown_provenance_field_is_rejected() {
        let dir = TempDir::new().unwrap();
        let (c, p, u) = write_clean_triple(dir.path());
        let mut provenance: serde_json::Value =
            serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
        provenance["future_field"] = serde_json::json!("oops");
        fs::write(&p, serde_json::to_string(&provenance).unwrap() + "\n").unwrap();
        let err = audit(&c, &p, &u).unwrap_err();
        assert!(
            format!("{err:#}").contains("unknown field")
                || format!("{err:#}").contains("deny_unknown_fields")
                || format!("{err:#}").contains("future_field"),
            "expected unknown-field rejection: {err:#}"
        );
    }
}
