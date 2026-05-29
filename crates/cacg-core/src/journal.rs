//! Append-only JSONL `lint_journal` with `prev_checksum` chain.
//!
//! Mirrors Python `legacy_python_oracle/src/cacg/journal.py` + `legacy_python_oracle/src/cacg/schema.py::LintEvent`.
//! Each line is a canonical-JSON-serialized `LintEvent`; the `seq` field
//! is monotonic per file; each event's `event_checksum` is SHA-256 over
//! canonical-JSON(event without `event_checksum`); `prev_checksum`
//! references the prior event's `event_checksum` so any same-`seq`
//! rewrite breaks the chain.
//!
//! `BL-20260518-checksum-chain-jsonl` directly applies: this module is
//! the Rust port of the lesson's documented Python implementation.
//!
//! Concurrency / atomicity discipline (AC-T11, AC-C5 closure):
//! - POSIX advisory `flock` (`rustix::fs::flock` with `LOCK_EX`) on a
//!   sidecar `{path}.lock` file wraps the validate + tail-scan +
//!   cache-check + write + cache-update critical section so parallel
//!   CLI invocations cannot interleave their `prev_checksum` /
//!   `event_checksum` derivations and corrupt the chain.
//! - The append itself is one `rustix::io::write` call on an
//!   `O_APPEND | O_WRONLY | O_CREAT` fd; POSIX guarantees a write of
//!   `<= PIPE_BUF` bytes (4096 on Linux) on an `O_APPEND` fd is atomic
//!   with respect to the kernel-internal seek+write pair, so a worker
//!   killed mid-syscall either wrote nothing or the full line. The
//!   single-syscall shape is enforced by the
//!   `single_write_helper_invokes_writer_exactly_once_with_newline_terminated_payload`
//!   regression unit test.
//! - A process-local `APPEND_CACHE`
//!   (`Mutex<BTreeMap<PathBuf, AppendCacheEntry>>`) short-circuits the
//!   O(file) `validate_jsonl` + `scan_tail` work on cache hits; the
//!   `(st_dev, st_ino, st_size, st_mtime_ns)` fingerprint invalidates
//!   the entry whenever an external write changes size or mtime, so
//!   tamper tests still fall through to the full validate path. The
//!   cache is in-process only; cross-process serialization is provided
//!   by the on-disk `flock`, exercised by
//!   `crates/cacg-core/tests/journal_subprocess_concurrent.rs`.
//!
//! Public API:
//! - [`LintEvent`] — the persisted on-disk event shape.
//! - [`JournalEntry`] — the input the caller hands to `append_entry`
//!   (subset without chain/timestamp/event_id fields).
//! - [`JournalError`] — the typed error returned by both entry points.
//! - [`validate_jsonl`] — 1-indexed bad-line scanner; mirrors Python's
//!   `validate_jsonl(path) -> list[int]`.
//! - [`append_entry`] — `flock`-serialized, single-syscall O_APPEND
//!   atomic append with pre-append structural + chain validation.
//! - [`reset_append_cache`] — test-only escape hatch that clears the
//!   process-local cache, mirroring Python's `reset_append_cache()`.

use std::collections::BTreeMap;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rustix::fs::{flock, FlockOperation, Mode, OFlags};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::canonical_json::canonical_json;
use crate::diagnostic::codes;
use crate::hash::compute_event_checksum;
use crate::schema::SchemaVersion;

/// One entry of the process-local append cache.
///
/// Stored per (canonicalized) journal path; invalidated when the
/// fingerprint `(dev, ino, size, mtime_ns)` changes. Mirrors Python
/// `cacg.journal._AppendCache` exactly. Shared with `cacg_core::history`
/// (each module owns its own `Mutex<BTreeMap<...>>` static but reuses
/// the same entry shape + fingerprint discipline).
#[derive(Clone, Debug)]
pub(crate) struct AppendCacheEntry {
    pub(crate) st_dev: u64,
    pub(crate) st_ino: u64,
    pub(crate) st_size: u64,
    pub(crate) st_mtime_ns: i64,
    pub(crate) next_seq: u64,
    pub(crate) last_checksum: Option<String>,
}

/// Process-local trust mark for the most recent successful append.
///
/// Cache HIT (fingerprint matches): skip the O(file) `validate_jsonl`
/// + tail-scan and reuse the cached `(next_seq, last_checksum)`.
/// Takes an N-event batch from O(N²) to O(N).
///
/// Cache MISS (fingerprint differs or absent): full validate + tail
/// scan. Any external write changes `st_mtime_ns` (or `st_size`),
/// tripping the fingerprint check so tamper tests still surface
/// invalid lines.
///
/// Mirrors Python `cacg.journal._append_cache`. Mutex serializes
/// in-process access; the per-path `flock` serializes cross-process.
static APPEND_CACHE: Mutex<BTreeMap<PathBuf, AppendCacheEntry>> = Mutex::new(BTreeMap::new());

/// Drop all cached trust marks. Test-only escape hatch mirroring
/// Python `cacg.journal.reset_append_cache`. Concurrent-append tests
/// invoke this before each scenario so a prior test's cache state
/// doesn't bleed in.
pub fn reset_append_cache() {
    APPEND_CACHE
        .lock()
        .expect("APPEND_CACHE mutex poisoned")
        .clear();
}

pub(crate) fn cache_key(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

pub(crate) fn fingerprint(path: &Path) -> Option<(u64, u64, u64, i64)> {
    let m = std::fs::metadata(path).ok()?;
    Some((m.dev(), m.ino(), m.size(), m.mtime_nsec()))
}

/// One persisted journal event. Mirrors Python
/// `cacg.schema.LintEvent` lines 403-416 with `deny_unknown_fields`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LintEvent {
    /// Schema version (always `cacg.v0`).
    pub schema_version: SchemaVersion,
    /// Monotonic sequence number, 0 on the first event of a file.
    pub seq: u64,
    /// Event UUID; under `KB_FROZEN_CLOCK=1` collapses to all-zero.
    pub event_id: String,
    /// Event timestamp ISO-8601; under `KB_FROZEN_CLOCK=1` collapses to epoch.
    pub timestamp: String,
    /// The `kb` subcommand that emitted the event (`lint`, `verify`, ...).
    pub command: String,
    /// Path of the card the event describes.
    pub card_path: String,
    /// Card hash before the operation (None when not applicable).
    #[serde(default)]
    pub card_hash_before: Option<String>,
    /// Card hash after the operation (None when not applicable).
    #[serde(default)]
    pub card_hash_after: Option<String>,
    /// Diagnostic shapes as opaque JSON (no per-field validation here).
    pub diagnostics: Vec<Value>,
    /// `{ "layer1": bool, "layer2": bool, "fuzzy": bool }` verification flags.
    pub verification: BTreeMap<String, bool>,
    /// Latency in milliseconds; collapses to 0.0 under frozen clock.
    pub latency_ms: f64,
    /// Prior event's `event_checksum`, or None on the first line.
    #[serde(default)]
    pub prev_checksum: Option<String>,
    /// SHA-256 over canonical-JSON of this event minus `event_checksum`.
    pub event_checksum: String,
}

/// Input shape callers hand to [`append_entry`]. The chain / id /
/// timestamp fields are populated by the appender, so the caller
/// only provides the per-event payload.
#[derive(Debug, Clone)]
pub struct JournalEntry {
    /// The `kb` subcommand emitting the event.
    pub command: String,
    /// Card path under the corpus tree.
    pub card_path: String,
    /// Card hash before the op (None when N/A).
    pub card_hash_before: Option<String>,
    /// Card hash after the op (None when N/A).
    pub card_hash_after: Option<String>,
    /// Diagnostic JSON values from the layer-1 / layer-2 path.
    pub diagnostics: Vec<Value>,
    /// Verification flags.
    pub verification: BTreeMap<String, bool>,
    /// Latency in ms (rounded to 6 decimals upstream; under
    /// `KB_FROZEN_CLOCK=1` always `0.0`).
    pub latency_ms: f64,
}

/// Errors raised by [`validate_jsonl`] and [`append_entry`].
#[derive(Debug, Error)]
pub enum JournalError {
    /// The provided path exists but is not a regular file (e.g.,
    /// a directory was passed).
    #[error("journal path {0:?} exists and is not a regular file")]
    NonFile(PathBuf),
    /// I/O failure reading or writing the journal.
    #[error("journal I/O error at {path:?}: {source}")]
    Io {
        /// Path that errored.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// `validate_jsonl` reported existing invalid lines; appender refuses.
    #[error("existing journal {path:?} has invalid line(s) {bad_lines:?}; refusing to append")]
    InvalidExisting {
        /// Path of the journal file with bad lines.
        path: PathBuf,
        /// 1-indexed list of bad line numbers.
        bad_lines: Vec<usize>,
    },
    /// `canonical_json` (or similar) failed for the payload being
    /// canonicalized; should be impossible for well-formed input but
    /// surfaces here as a defensive measure.
    #[error("canonical_json failure for journal payload: {0}")]
    Canonical(String),
    /// The supplied `JournalEntry` fails Pydantic-equivalent shape
    /// validation (e.g., `latency_ms < 0.0`, non-finite, or
    /// `diagnostics` contains a non-object element). Returned by
    /// [`append_entry`] before any I/O so a malformed call cannot
    /// pollute the journal.
    #[error("journal entry shape rejected: {0}")]
    InvalidEntry(&'static str),
}

impl LintEvent {
    /// Pydantic-equivalent structural validation (Codex R19 gap 1).
    ///
    /// Python `LintEvent` declares `seq: int = Field(ge=0)`,
    /// `diagnostics: list[dict[str, Any]]`, and `latency_ms: float =
    /// Field(ge=0)`. Serde's `deny_unknown_fields` covers field-set
    /// validation but does NOT enforce `ge=0` on numbers OR that
    /// `diagnostics` elements are JSON objects. This helper closes
    /// the gap. Returns an error string when an invariant fails.
    pub fn structural_validate(&self) -> Result<(), &'static str> {
        if !self.latency_ms.is_finite() {
            return Err("latency_ms must be finite (NaN/Infinity rejected)");
        }
        if self.latency_ms < 0.0 {
            return Err("latency_ms must be >= 0.0");
        }
        for value in &self.diagnostics {
            if !value.is_object() {
                return Err("diagnostics elements must be JSON objects");
            }
        }
        Ok(())
    }
}

impl JournalEntry {
    /// Pydantic-equivalent validation for the input shape callers
    /// hand to [`append_entry`]. Same constraints as
    /// [`LintEvent::structural_validate`] but checked on the
    /// pre-payload `JournalEntry` so a malformed call is rejected
    /// before any disk I/O.
    pub fn structural_validate(&self) -> Result<(), &'static str> {
        if !self.latency_ms.is_finite() {
            return Err("JournalEntry.latency_ms must be finite");
        }
        if self.latency_ms < 0.0 {
            return Err("JournalEntry.latency_ms must be >= 0.0");
        }
        for value in &self.diagnostics {
            if !value.is_object() {
                return Err("JournalEntry.diagnostics elements must be JSON objects");
            }
        }
        Ok(())
    }
}

/// Scan an existing journal for invalid lines.
///
/// Returns 1-indexed line numbers of lines that fail any of:
/// - Non-JSON content (after `.trim()`).
/// - Pydantic-equivalent shape validation (missing required field;
///   unknown field; non-`cacg.v0` schema_version; etc.).
/// - Non-monotonic `seq` (expected starts at 0 and increments by 1).
/// - `event_checksum` mismatch (recomputed != stored).
/// - Broken `prev_checksum` chain (current `prev_checksum` !=
///   prior event's `event_checksum`).
///
/// Blank lines are skipped. A missing file returns `Ok(vec![])`.
///
/// # Errors
///
/// Returns [`JournalError::NonFile`] if the path exists but is not a
/// regular file (e.g., a directory). Returns [`JournalError::Io`] for
/// read failures.
pub fn validate_jsonl(path: &Path) -> Result<Vec<usize>, JournalError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    if !path.is_file() {
        return Err(JournalError::NonFile(path.to_path_buf()));
    }
    let text = std::fs::read_to_string(path).map_err(|source| JournalError::Io {
        path: path.to_path_buf(),
        source,
    })?;

    let mut bad: Vec<usize> = Vec::new();
    let mut expected_seq: u64 = 0;
    let mut expected_prev_checksum: Option<String> = None;
    for (n, raw) in text.lines().enumerate().map(|(i, l)| (i + 1, l)) {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        // Strict deserialize: deny_unknown_fields + tagged SchemaVersion
        // enforce most of the Pydantic shape contract for free.
        let event: LintEvent = match serde_json::from_str(line) {
            Ok(e) => e,
            Err(_) => {
                bad.push(n);
                continue;
            }
        };
        // Codex R19 gap 1: Pydantic-equivalent structural validation.
        // A checksum-valid event with `latency_ms: -1.0` or a non-object
        // `diagnostics` element passes Serde but Python rejects via
        // `Field(ge=0)` + `list[dict[str, Any]]`. Mirror Python.
        if event.structural_validate().is_err() {
            bad.push(n);
            continue;
        }
        if event.seq != expected_seq {
            bad.push(n);
            // Advance both anchors so subsequent lines are scored
            // against this event's anchor (matches Python's
            // continue-after-bad-seq behavior).
            expected_seq = event.seq + 1;
            expected_prev_checksum = Some(event.event_checksum.clone());
            continue;
        }
        // Recompute event_checksum over the payload with
        // `event_checksum` field removed; mirrors Python's
        // `compute_event_checksum(payload)`.
        let payload_value = match serde_json::to_value(&event) {
            Ok(v) => v,
            Err(_) => {
                bad.push(n);
                continue;
            }
        };
        #[allow(clippy::wildcard_enum_match_arm)]
        let payload_map = match payload_value {
            Value::Object(m) => m,
            _ => {
                bad.push(n);
                continue;
            }
        };
        let recomputed = match compute_event_checksum(&payload_map) {
            Ok(s) => s,
            Err(_) => {
                bad.push(n);
                continue;
            }
        };
        if recomputed != event.event_checksum || event.prev_checksum != expected_prev_checksum {
            bad.push(n);
        }
        expected_seq = event.seq + 1;
        expected_prev_checksum = Some(event.event_checksum.clone());
    }
    Ok(bad)
}

/// Append a single entry to the journal, returning the assigned `seq`.
///
/// Builds the payload in Python's exact field order via
/// `serde_json::Map` insertion (BTreeMap is canonicalized by
/// `cacg_core::canonical_json`, which sorts keys at serialize time;
/// the input order doesn't matter for byte equality). Computes
/// `event_checksum`, canonicalizes the full event including the
/// checksum, and appends `line + "\n"` to the file.
///
/// `event_id` and `timestamp` are caller-supplied because Round-19
/// keeps the DeterminismContext (UUID / clock injection) out of
/// scope; that ships in task-M1-9. Under `KB_FROZEN_CLOCK=1` callers
/// pass `"00000000-0000-0000-0000-000000000000"` + `"1970-01-01T00:00:00Z"`.
///
/// Pre-append integrity check: if `validate_jsonl(path)` flags any
/// existing line as invalid, this function refuses to append and
/// returns [`JournalError::InvalidExisting`]. Mirrors the Python
/// preflight discipline.
///
/// # Errors
///
/// Returns [`JournalError`] for any of: non-file path, parent-dir
/// creation failure, pre-existing invalid lines, or I/O failure on
/// write.
pub fn append_entry(
    path: &Path,
    entry: &JournalEntry,
    event_id: &str,
    timestamp: &str,
) -> Result<u64, JournalError> {
    // Codex R19 gap 1: structural validation BEFORE any I/O so a
    // malformed call cannot pollute the journal.
    if let Err(msg) = entry.structural_validate() {
        return Err(JournalError::InvalidEntry(msg));
    }
    if path.exists() && !path.is_file() {
        return Err(JournalError::NonFile(path.to_path_buf()));
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|source| JournalError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    // POSIX advisory lock over the entire validate + tail-scan +
    // cache-check + write + cache-update critical section, mirroring
    // Python `_flock_lock`. Without it, parallel CLI invocations
    // could interleave their prev_checksum/event_checksum derivations
    // and corrupt the chain. Lock file = `<path>.lock`.
    let lock_path = lock_path_for(path);
    let lock_fd = rustix::fs::open(
        &lock_path,
        OFlags::WRONLY | OFlags::CREATE,
        Mode::from_bits_truncate(0o644),
    )
    .map_err(|e| JournalError::Io {
        path: lock_path.clone(),
        source: std::io::Error::from(e),
    })?;
    flock(&lock_fd, FlockOperation::LockExclusive).map_err(|e| JournalError::Io {
        path: lock_path.clone(),
        source: std::io::Error::from(e),
    })?;

    // Wrap the rest in a closure so we can release the lock on EVERY
    // exit path. The fd's Drop releases the lock automatically but
    // we explicitly Unlock to be defensive.
    let result = (|| -> Result<u64, JournalError> {
        // Cache check: if the file's fingerprint matches the cached
        // entry, skip the O(file) validate + tail-scan and reuse
        // cached state.
        let key = cache_key(path);
        let fp = fingerprint(path);
        let cached = {
            let map = APPEND_CACHE.lock().expect("APPEND_CACHE mutex poisoned");
            map.get(&key).cloned()
        };
        let (seq, prev_checksum) = if let (Some(c), Some(fp)) = (cached, fp) {
            if (c.st_dev, c.st_ino, c.st_size, c.st_mtime_ns) == fp {
                (c.next_seq, c.last_checksum)
            } else {
                let bad = validate_jsonl(path)?;
                if !bad.is_empty() {
                    return Err(JournalError::InvalidExisting {
                        path: path.to_path_buf(),
                        bad_lines: bad,
                    });
                }
                scan_tail(path)?
            }
        } else {
            let bad = validate_jsonl(path)?;
            if !bad.is_empty() {
                return Err(JournalError::InvalidExisting {
                    path: path.to_path_buf(),
                    bad_lines: bad,
                });
            }
            scan_tail(path)?
        };
        append_payload(
            path,
            entry,
            event_id,
            timestamp,
            seq,
            prev_checksum.as_deref(),
            &key,
        )
    })();

    // Best-effort unlock; fd drop also releases.
    let _ = flock(&lock_fd, FlockOperation::Unlock);
    drop(lock_fd);

    result
}

/// Lock-file path: `{path}.lock`. Mirrors Python's
/// `path.with_suffix(path.suffix + ".lock")` (which appends `.lock`
/// after the existing suffix, not as a replacement). Shared with
/// `cacg_core::history` so each module's flock sidecar follows the
/// same naming convention.
pub(crate) fn lock_path_for(path: &Path) -> PathBuf {
    let mut s = path.as_os_str().to_os_string();
    s.push(".lock");
    PathBuf::from(s)
}

/// Inner-append helper: holds the post-cache path that builds the
/// payload, writes it via single-syscall `rustix::io::write` on an
/// O_APPEND fd, and updates the process-local cache. Called only
/// inside the flock critical section.
fn append_payload(
    path: &Path,
    entry: &JournalEntry,
    event_id: &str,
    timestamp: &str,
    seq: u64,
    prev_checksum: Option<&str>,
    cache_key_path: &Path,
) -> Result<u64, JournalError> {
    // Build the payload as a serde_json::Map. Canonical-JSON sorts
    // keys at serialize time, so the insertion order here is irrelevant
    // for the byte-equal output -- but we keep Python's field order in
    // comments for cross-impl readability.
    let mut payload = serde_json::Map::new();
    payload.insert(
        "card_hash_after".to_string(),
        json_option(entry.card_hash_after.as_deref()),
    );
    payload.insert(
        "card_hash_before".to_string(),
        json_option(entry.card_hash_before.as_deref()),
    );
    payload.insert(
        "card_path".to_string(),
        Value::String(entry.card_path.clone()),
    );
    payload.insert("command".to_string(), Value::String(entry.command.clone()));
    payload.insert(
        "diagnostics".to_string(),
        Value::Array(entry.diagnostics.clone()),
    );
    payload.insert("event_id".to_string(), Value::String(event_id.to_string()));
    payload.insert(
        "latency_ms".to_string(),
        json_float(entry.latency_ms).ok_or_else(|| {
            JournalError::Canonical(format!(
                "latency_ms {} cannot be canonicalized (non-finite?)",
                entry.latency_ms
            ))
        })?,
    );
    payload.insert(
        "prev_checksum".to_string(),
        prev_checksum
            .as_deref()
            .map_or(Value::Null, |s| Value::String(s.to_string())),
    );
    payload.insert(
        "schema_version".to_string(),
        Value::String("cacg.v0".to_string()),
    );
    payload.insert("seq".to_string(), Value::Number(seq.into()));
    payload.insert(
        "timestamp".to_string(),
        Value::String(timestamp.to_string()),
    );
    payload.insert(
        "verification".to_string(),
        serde_json::to_value(&entry.verification).expect("verification serializes"),
    );

    let event_checksum =
        compute_event_checksum(&payload).map_err(|e| JournalError::Canonical(e.to_string()))?;
    payload.insert(
        "event_checksum".to_string(),
        Value::String(event_checksum.clone()),
    );

    let line = canonical_json(&Value::Object(payload))
        .map_err(|e| JournalError::Canonical(e.to_string()))?;

    append_one_line_via_writer(path, &line, default_append_writer).map_err(|source| {
        JournalError::Io {
            path: path.to_path_buf(),
            source,
        }
    })?;

    // Update the process-local append cache so subsequent appends
    // (in this process) can skip the validate + tail-scan O(N) work.
    if let Some(fp) = fingerprint(path) {
        let entry_cache = AppendCacheEntry {
            st_dev: fp.0,
            st_ino: fp.1,
            st_size: fp.2,
            st_mtime_ns: fp.3,
            next_seq: seq + 1,
            last_checksum: Some(event_checksum.clone()),
        };
        APPEND_CACHE
            .lock()
            .expect("APPEND_CACHE mutex poisoned")
            .insert(cache_key_path.to_path_buf(), entry_cache);
    } else {
        // fingerprint failed -> evict
        APPEND_CACHE
            .lock()
            .expect("APPEND_CACHE mutex poisoned")
            .remove(cache_key_path);
    }

    // Silence unused-imports warning for `codes` in the lib path --
    // future history.rs / atomic_publish.rs will consume CACG-JNL-001.
    let _ = codes::CLI_001;

    Ok(seq)
}

/// Build `line + "\n"` and hand it to `writer` exactly once for the
/// AC-T11 single-syscall O_APPEND atomic write contract. POSIX
/// guarantees a write of `<= PIPE_BUF` bytes (4096 on Linux) on an
/// `O_APPEND` fd is atomic with respect to the seek+write pair, so a
/// worker killed mid-syscall either wrote nothing or the full line.
///
/// The closure boundary is the regression guard: a future refactor that
/// splits the append into two writes would break the
/// `single_write_helper_invokes_writer_exactly_once_with_newline_terminated_payload`
/// unit test. Mirrors Python `_append_one_line_atomically` +
/// `test_append_uses_single_os_write_syscall`.
///
/// Returns `std::io::Error` (not a domain-specific error) so both
/// `journal.rs` and `history.rs` can call this helper and wrap the
/// resulting I/O failure into their respective `*Error::Io` variant.
pub(crate) fn append_one_line_via_writer<W>(
    path: &Path,
    line: &str,
    mut writer: W,
) -> Result<(), std::io::Error>
where
    W: FnMut(&Path, &[u8]) -> Result<usize, std::io::Error>,
{
    let mut payload = Vec::with_capacity(line.len() + 1);
    payload.extend_from_slice(line.as_bytes());
    payload.push(b'\n');
    let written = writer(path, &payload)?;
    if written != payload.len() {
        return Err(std::io::Error::other(format!(
            "partial write: {written} of {} bytes",
            payload.len()
        )));
    }
    Ok(())
}

/// Production writer: opens an `O_APPEND | O_WRONLY | O_CREAT` fd via
/// `rustix::fs::open`, performs one `rustix::io::write`, closes the fd
/// on drop. Returns the byte count written by the kernel so the caller
/// can verify against the payload length. Shared with `cacg_core::history`.
pub(crate) fn default_append_writer(path: &Path, bytes: &[u8]) -> Result<usize, std::io::Error> {
    let fd = rustix::fs::open(
        path,
        OFlags::WRONLY | OFlags::APPEND | OFlags::CREATE,
        Mode::from_bits_truncate(0o644),
    )
    .map_err(std::io::Error::from)?;
    let n = rustix::io::write(&fd, bytes).map_err(std::io::Error::from)?;
    drop(fd);
    Ok(n)
}

/// Scan the tail of an existing journal for `(next_seq, prev_checksum)`.
/// Returns `(0, None)` for a missing or empty file.
fn scan_tail(path: &Path) -> Result<(u64, Option<String>), JournalError> {
    if !path.exists() {
        return Ok((0, None));
    }
    let text = std::fs::read_to_string(path).map_err(|source| JournalError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let mut last_seq: i64 = -1;
    let mut last_checksum: Option<String> = None;
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let event: LintEvent = serde_json::from_str(line).map_err(|e| JournalError::Io {
            path: path.to_path_buf(),
            source: std::io::Error::other(format!("parse failure on tail scan: {e}")),
        })?;
        last_seq = event.seq as i64;
        last_checksum = Some(event.event_checksum);
    }
    Ok(((last_seq + 1) as u64, last_checksum))
}

/// Serialize an Option<&str> as JSON: `Some(s)` -> `Value::String(s)`;
/// `None` -> `Value::Null`. Mirrors Python's `Optional[str]` serde behavior.
fn json_option(s: Option<&str>) -> Value {
    s.map_or(Value::Null, |v| Value::String(v.to_string()))
}

/// Serialize a finite f64 as a JSON number; non-finite returns None.
fn json_float(f: f64) -> Option<Value> {
    serde_json::Number::from_f64(f).map(Value::Number)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn validate_missing_file_returns_empty() {
        let p = PathBuf::from("/tmp/cacg-core-journal-test-does-not-exist-xyz");
        let r = validate_jsonl(&p).unwrap();
        assert!(r.is_empty());
    }

    #[test]
    fn validate_non_file_path_errors() {
        let dir = std::env::temp_dir();
        let r = validate_jsonl(&dir);
        assert!(matches!(r, Err(JournalError::NonFile(_))));
    }

    /// AC-T11 single-syscall regression guard (Codex R20 gap 2). Mirrors
    /// Python `legacy_python_oracle/tests/test_phase4_journal_flock.py::test_append_uses_single_os_write_syscall`:
    /// a future refactor that splits `line + '\n'` into two writes would
    /// break the chain-tail atomicity contract, so this test pins the
    /// call shape via a counting closure.
    #[test]
    fn single_write_helper_invokes_writer_exactly_once_with_newline_terminated_payload() {
        use std::cell::RefCell;
        let calls: RefCell<Vec<Vec<u8>>> = RefCell::new(Vec::new());
        let canonical_line = "canonical-line-payload-without-trailing-newline";
        let result = append_one_line_via_writer(
            Path::new("/tmp/cacg-single-write-regression-dummy.jsonl"),
            canonical_line,
            |_p, bytes| {
                calls.borrow_mut().push(bytes.to_vec());
                Ok(bytes.len())
            },
        );
        assert!(result.is_ok(), "helper returned err: {result:?}");
        let calls = calls.borrow();
        assert_eq!(
            calls.len(),
            1,
            "writer must be invoked exactly once per append; got {} calls",
            calls.len()
        );
        let bytes = &calls[0];
        assert!(
            bytes.ends_with(b"\n"),
            "payload must end with newline; got {bytes:?}"
        );
        assert_eq!(
            &bytes[..bytes.len() - 1],
            canonical_line.as_bytes(),
            "payload (minus trailing newline) must equal the canonical line"
        );
    }
}
