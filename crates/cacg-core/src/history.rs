//! Per-card append-only `.history.jsonl` with `prev_checksum` chain.
//!
//! Mirrors Python `legacy_python_oracle/src/cacg/history.py` + `legacy_python_oracle/src/cacg/schema.py::HistoryEvent`.
//! Each card has a sibling history file (`<card>.history.jsonl`); `kb
//! index` appends one event per card hash change, and `kb retract <card>`
//! appends a tombstone event with `is_retracted = true` plus the
//! [`RETRACTION_FRONTMATTER_MARKER`] in `frontmatter_field_changes`.
//!
//! `BL-20260518-checksum-chain-jsonl` directly applies: this module is
//! a sibling Rust port of the lesson's documented Python implementation.
//! The chain-tail integrity discipline + the `validate_*` contract are
//! identical to `cacg_core::journal`; only the event shape differs.
//!
//! Concurrency / atomicity discipline (shared with `cacg_core::journal`):
//! - POSIX `flock` (`rustix::fs::flock` LOCK_EX) on the sidecar
//!   `{path}.lock` wraps the validate + tail-scan + cache-check +
//!   write + cache-update critical section.
//! - The append itself is one `rustix::io::write` on an
//!   `O_APPEND | O_WRONLY | O_CREAT` fd; POSIX-atomic for
//!   `<= PIPE_BUF` payloads. Enforced by the
//!   [`journal::single_write_helper_invokes_writer_exactly_once_with_newline_terminated_payload`]
//!   regression guard (the helper is `pub(crate)` and shared).
//! - Process-local [`HISTORY_APPEND_CACHE`] short-circuits the O(file)
//!   validate + tail-scan on cache hits; the `(st_dev, st_ino, st_size,
//!   st_mtime_ns)` fingerprint invalidates the entry when an external
//!   write changes size or mtime.
//!
//! Public API:
//! - [`HistoryEvent`] — the persisted on-disk event shape.
//! - [`HistoryEntry`] — the input the caller hands to `append_history_event`.
//! - [`HistoryError`] — typed error returned by both entry points.
//! - [`validate_history`] — 1-indexed bad-line scanner.
//! - [`append_history_event`] — flock-serialized, single-syscall append.
//! - [`history_path_for`] — `cards/foo/bar.md` → `cards/foo/bar.history.jsonl`.
//! - [`reset_history_cache`] — test-only escape hatch.
//! - [`RETRACTION_FRONTMATTER_MARKER`] — `"__cacg_retracted__"` constant.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rustix::fs::{flock, FlockOperation, Mode, OFlags};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use thiserror::Error;

use crate::canonical_json::canonical_json;
use crate::hash::compute_event_checksum;
use crate::journal::{
    append_one_line_via_writer, cache_key, default_append_writer, fingerprint, lock_path_for,
    AppendCacheEntry,
};
use crate::schema::SchemaVersion;

/// The tombstone marker string a `kb retract <card>` event places in
/// `frontmatter_field_changes`. Mirrors Python
/// `cacg.retract.RETRACTION_FRONTMATTER_MARKER`.
pub const RETRACTION_FRONTMATTER_MARKER: &str = "__cacg_retracted__";

/// Process-local trust mark for the most recent successful per-path
/// history append. Same shape + invalidation rule as
/// [`crate::journal::APPEND_CACHE`]; kept as a separate `Mutex` here
/// so journal and history caches stay independent (each path's
/// fingerprint is keyed by canonical path; the two modules never
/// share entries).
static HISTORY_APPEND_CACHE: Mutex<BTreeMap<PathBuf, AppendCacheEntry>> =
    Mutex::new(BTreeMap::new());

/// Drop all cached trust marks for history files. Test-only escape
/// hatch mirroring Python `cacg.history.reset_history_cache` (Python's
/// history module doesn't expose a cache, but parity tests need to
/// clear in-process state between scenarios).
pub fn reset_history_cache() {
    HISTORY_APPEND_CACHE
        .lock()
        .expect("HISTORY_APPEND_CACHE mutex poisoned")
        .clear();
}

/// Return the sibling history path for `card_path`. Replaces the final
/// extension with `history.jsonl`; mirrors Python's
/// `Path(card_path).with_suffix(".history.jsonl")`.
///
/// Examples:
/// - `cards/foo/bar.md` → `cards/foo/bar.history.jsonl`
/// - `cards/baz` (no extension) → `cards/baz.history.jsonl`
pub fn history_path_for(card_path: &Path) -> PathBuf {
    card_path.with_extension("history.jsonl")
}

/// One persisted history event. Mirrors Python
/// `cacg.schema.HistoryEvent` (lines 419-440) with `deny_unknown_fields`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryEvent {
    /// Schema version (always `cacg.v0`).
    pub schema_version: SchemaVersion,
    /// Monotonic sequence number; 0 on the first event of a file.
    pub seq: u64,
    /// Event timestamp; collapses to epoch under `KB_FROZEN_CLOCK=1`.
    pub timestamp: String,
    /// Card hash before this event, or None at file genesis.
    #[serde(default)]
    pub prev_card_hash: Option<String>,
    /// Card hash after this event.
    pub new_card_hash: String,
    /// Cited chunk set delta, typically `{"added": [...], "removed": [...]}`.
    pub cited_chunk_set_delta: BTreeMap<String, Vec<String>>,
    /// Sorted list of frontmatter field names that changed in this
    /// event. Retraction tombstones contain `["__cacg_retracted__"]`.
    pub frontmatter_field_changes: Vec<String>,
    /// Snapshot of post-event cited chunk IDs so the NEXT event can
    /// compute a real delta. Empty default for backwards compatibility.
    #[serde(default)]
    pub cited_chunk_ids_snapshot: Vec<String>,
    /// Snapshot of post-event frontmatter so the NEXT event can compute
    /// real field changes. Empty default for backwards compatibility.
    #[serde(default)]
    pub frontmatter_snapshot: Map<String, Value>,
    /// True when this event is the tombstone written by `kb retract`.
    #[serde(default)]
    pub is_retracted: bool,
    /// Prior event's `event_checksum`, or None on the first line.
    #[serde(default)]
    pub prev_checksum: Option<String>,
    /// SHA-256 over canonical-JSON of this event minus `event_checksum`.
    pub event_checksum: String,
}

/// Input shape callers hand to [`append_history_event`]. Mirrors Python's
/// `cacg.history.HistoryEvent` dataclass (7 fields, no chain / seq /
/// timestamp — those are populated by the appender).
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    /// Card hash before the operation, or None at genesis.
    pub prev_card_hash: Option<String>,
    /// Card hash after the operation. Required.
    pub new_card_hash: String,
    /// Cited chunk set delta typically `{"added": [...], "removed": [...]}`.
    pub cited_chunk_set_delta: BTreeMap<String, Vec<String>>,
    /// Sorted list of frontmatter field names that changed.
    pub frontmatter_field_changes: Vec<String>,
    /// Snapshot of post-event cited chunk IDs.
    pub cited_chunk_ids_snapshot: Vec<String>,
    /// Snapshot of post-event frontmatter.
    pub frontmatter_snapshot: Map<String, Value>,
    /// Tombstone marker for `kb retract`-emitted events.
    pub is_retracted: bool,
}

/// Errors raised by [`validate_history`] and [`append_history_event`].
#[derive(Debug, Error)]
pub enum HistoryError {
    /// The provided path exists but is not a regular file.
    #[error("history path {0:?} exists and is not a regular file")]
    NonFile(PathBuf),
    /// I/O failure reading or writing the history file.
    #[error("history I/O error at {path:?}: {source}")]
    Io {
        /// Path that errored.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// `validate_history` reported existing invalid lines; appender refuses.
    #[error("existing history {path:?} has invalid line(s) {bad_lines:?}; refusing to append")]
    InvalidExisting {
        /// Path of the file with bad lines.
        path: PathBuf,
        /// 1-indexed list of bad line numbers.
        bad_lines: Vec<usize>,
    },
    /// `canonical_json` (or similar) failed for the payload.
    #[error("canonical JSON failure: {0}")]
    Canonical(String),
    /// Input entry failed the Pydantic-equivalent structural gate.
    #[error("invalid history entry: {0}")]
    InvalidEntry(&'static str),
}

impl HistoryEntry {
    /// Pydantic-equivalent structural gate. Python's `HistoryEvent`
    /// Pydantic model declares only `seq: int = Field(ge=0)` as a
    /// constraint; `new_card_hash: str` accepts any string (including
    /// the empty string) and the snapshot / delta fields are typed at
    /// the Python level via dict/list generics that serde already
    /// enforces in Rust. Therefore this method currently has no
    /// rejection cases — it stays as `Ok(())` for symmetry with the
    /// `journal::JournalEntry::structural_validate` shape and to give
    /// future Pydantic invariant additions an obvious landing spot
    /// without needing to thread a new method through the appender.
    pub fn structural_validate(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

impl HistoryEvent {
    /// Post-deserialize structural gate. Same null-set of constraints
    /// as `HistoryEntry::structural_validate`; the persisted Pydantic
    /// model has no `min_length` / regex / shape constraints beyond
    /// what serde + the type system already enforce.
    pub fn structural_validate(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

/// Return `(cited_chunk_ids_snapshot, frontmatter_snapshot)` from the
/// last event in the history file, or `(vec![], Map::new())` when the
/// file does not exist or contains no parseable events. Mirrors
/// Python's `cacg.history.last_event_snapshot`.
///
/// Feeds the `kb index` delta computation so consecutive index runs
/// can record actual added/removed `chunk_id`s and frontmatter field
/// changes against the LAST persisted event's snapshot rather than
/// against the in-memory `prior_frontmatter` Python had at write
/// time. Malformed lines are silently skipped (the next index run
/// will surface the integrity issue via `validate_history`'s
/// preflight inside `append_history_event`).
pub fn last_event_snapshot(path: &Path) -> Result<(Vec<String>, Map<String, Value>), HistoryError> {
    if !path.exists() {
        return Ok((Vec::new(), Map::new()));
    }
    if !path.is_file() {
        return Err(HistoryError::NonFile(path.to_path_buf()));
    }
    let text = std::fs::read_to_string(path).map_err(|source| HistoryError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let mut last_chunk_ids: Vec<String> = Vec::new();
    let mut last_frontmatter: Map<String, Value> = Map::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let event: HistoryEvent = match serde_json::from_str(line) {
            Ok(e) => e,
            Err(_) => continue,
        };
        last_chunk_ids = event.cited_chunk_ids_snapshot;
        last_frontmatter = event.frontmatter_snapshot;
    }
    Ok((last_chunk_ids, last_frontmatter))
}

/// Return the `new_card_hash` field of the last persisted history
/// event in `path`, or `None` when the file does not exist OR
/// contains no parseable events. Used by `build_index` to detect
/// the "upgrade from prior Rust kb index" state: when a card's
/// history sidecar already ends with `new_card_hash` equal to the
/// recomputed canonical hash but the card frontmatter is still
/// stale (an older Rust version wrote the history but failed to
/// rewrite the frontmatter, or external tampering reverted the
/// frontmatter), the per-card loop must NOT append a duplicate
/// audit event for the same canonical hash.
pub fn last_event_new_card_hash(path: &Path) -> Result<Option<String>, HistoryError> {
    if !path.exists() {
        return Ok(None);
    }
    if !path.is_file() {
        return Err(HistoryError::NonFile(path.to_path_buf()));
    }
    let text = std::fs::read_to_string(path).map_err(|source| HistoryError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let mut last: Option<String> = None;
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let event: HistoryEvent = match serde_json::from_str(line) {
            Ok(e) => e,
            Err(_) => continue,
        };
        last = Some(event.new_card_hash);
    }
    Ok(last)
}

/// Walk the history file and return 1-indexed line numbers that fail
/// JSON parse, Pydantic-equivalent shape, monotonic seq, or the
/// `prev_checksum`/`event_checksum` chain. Mirrors Python
/// `cacg.history.validate_history`.
pub fn validate_history(path: &Path) -> Result<Vec<usize>, HistoryError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    if !path.is_file() {
        return Err(HistoryError::NonFile(path.to_path_buf()));
    }
    let text = std::fs::read_to_string(path).map_err(|source| HistoryError::Io {
        path: path.to_path_buf(),
        source,
    })?;

    let mut bad: Vec<usize> = Vec::new();
    let mut expected_seq: u64 = 0;
    let mut expected_prev_checksum: Option<String> = None;
    for (n, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let n1 = n + 1;
        let event: HistoryEvent = match serde_json::from_str(line) {
            Ok(e) => e,
            Err(_) => {
                bad.push(n1);
                continue;
            }
        };
        if event.structural_validate().is_err() {
            bad.push(n1);
            continue;
        }
        if event.seq != expected_seq {
            bad.push(n1);
            expected_seq = event.seq + 1;
            expected_prev_checksum = Some(event.event_checksum.clone());
            continue;
        }
        let mut payload = canonical_payload(&event);
        payload.remove("event_checksum");
        let recomputed = match compute_event_checksum(&payload) {
            Ok(c) => c,
            Err(_) => {
                bad.push(n1);
                expected_seq = event.seq + 1;
                expected_prev_checksum = Some(event.event_checksum.clone());
                continue;
            }
        };
        if recomputed != event.event_checksum || event.prev_checksum != expected_prev_checksum {
            bad.push(n1);
        }
        expected_seq = event.seq + 1;
        expected_prev_checksum = Some(event.event_checksum.clone());
    }
    Ok(bad)
}

/// Outcome of a successful `append_history_event_with_outcome` call.
/// Carries the assigned sequence number plus the file size observed
/// UNDER THE LOCK both before and after our write. Callers that need
/// to safely roll back ONLY the event they just appended (e.g.
/// `cacg_core::index::build_index` on rewrite failure after
/// history-append succeeded) feed these sizes to
/// `truncate_history_after_append_if_unchanged`, which uses them to
/// detect whether another writer has appended in the meantime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppendOutcome {
    /// Assigned sequence number of the newly-appended event.
    pub seq: u64,
    /// File size in bytes BEFORE our write, captured under the flock.
    /// The truncate-on-rollback target.
    pub prior_size_under_lock: u64,
    /// File size in bytes AFTER our write, captured under the flock
    /// (`prior_size_under_lock + line_with_newline_len`). The
    /// expected-current-size for the under-lock revalidation in
    /// `truncate_history_after_append_if_unchanged`.
    pub post_write_size: u64,
}

/// Append one entry; returns the assigned `seq`. Mirrors Python
/// `cacg.history.append_history_event` semantics including the
/// pre-append validate refusal, flock LOCK_EX critical section,
/// single-syscall O_APPEND atomic write, and process-local cache
/// short-circuit. Thin wrapper around
/// `append_history_event_with_outcome` for callers that don't need
/// the under-lock size bookkeeping.
pub fn append_history_event(
    path: &Path,
    entry: &HistoryEntry,
    timestamp: &str,
) -> Result<u64, HistoryError> {
    Ok(append_history_event_with_outcome(path, entry, timestamp)?.seq)
}

/// Same as `append_history_event` but returns the full
/// `AppendOutcome` (seq + prior_size_under_lock + post_write_size)
/// so the caller can later safely roll back ONLY this event via
/// `truncate_history_after_append_if_unchanged`.
pub fn append_history_event_with_outcome(
    path: &Path,
    entry: &HistoryEntry,
    timestamp: &str,
) -> Result<AppendOutcome, HistoryError> {
    if let Err(msg) = entry.structural_validate() {
        return Err(HistoryError::InvalidEntry(msg));
    }
    if path.exists() && !path.is_file() {
        return Err(HistoryError::NonFile(path.to_path_buf()));
    }
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|source| HistoryError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    let lock_path = lock_path_for(path);
    if let Some(parent) = lock_path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|source| HistoryError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    let lock_fd = rustix::fs::open(
        &lock_path,
        OFlags::WRONLY | OFlags::CREATE,
        Mode::from_bits_truncate(0o644),
    )
    .map_err(|e| HistoryError::Io {
        path: lock_path.clone(),
        source: std::io::Error::from(e),
    })?;
    flock(&lock_fd, FlockOperation::LockExclusive).map_err(|e| HistoryError::Io {
        path: lock_path.clone(),
        source: std::io::Error::from(e),
    })?;

    // Capture the file size UNDER THE LOCK so a later rollback can
    // distinguish "our event is still the tail" from "another
    // writer appended after us". Other writers must hold the same
    // flock to append, so the size we see here is stable for the
    // duration of our critical section.
    let prior_size_under_lock = if path.exists() {
        match std::fs::metadata(path) {
            Ok(m) => m.len(),
            Err(source) => {
                let _ = flock(&lock_fd, FlockOperation::Unlock);
                drop(lock_fd);
                return Err(HistoryError::Io {
                    path: path.to_path_buf(),
                    source,
                });
            }
        }
    } else {
        0
    };

    let result = (|| -> Result<AppendOutcome, HistoryError> {
        let key = cache_key(path);
        let fp = fingerprint(path);
        let cached = {
            let cache = HISTORY_APPEND_CACHE
                .lock()
                .expect("HISTORY_APPEND_CACHE mutex poisoned");
            cache.get(&key).cloned()
        };
        let (seq, prev_checksum) = if let (Some(c), Some(f)) = (cached.as_ref(), fp) {
            if (c.st_dev, c.st_ino, c.st_size, c.st_mtime_ns) == f {
                (c.next_seq, c.last_checksum.clone())
            } else {
                full_scan(path)?
            }
        } else {
            full_scan(path)?
        };

        let mut payload = Map::new();
        payload.insert(
            "cited_chunk_ids_snapshot".to_string(),
            Value::Array(
                entry
                    .cited_chunk_ids_snapshot
                    .iter()
                    .cloned()
                    .map(Value::String)
                    .collect(),
            ),
        );
        payload.insert(
            "cited_chunk_set_delta".to_string(),
            serde_json::to_value(&entry.cited_chunk_set_delta)
                .map_err(|e| HistoryError::Canonical(e.to_string()))?,
        );
        payload.insert(
            "frontmatter_field_changes".to_string(),
            Value::Array(
                entry
                    .frontmatter_field_changes
                    .iter()
                    .cloned()
                    .map(Value::String)
                    .collect(),
            ),
        );
        payload.insert(
            "frontmatter_snapshot".to_string(),
            Value::Object(entry.frontmatter_snapshot.clone()),
        );
        payload.insert("is_retracted".to_string(), Value::Bool(entry.is_retracted));
        payload.insert(
            "new_card_hash".to_string(),
            Value::String(entry.new_card_hash.clone()),
        );
        payload.insert(
            "prev_card_hash".to_string(),
            entry
                .prev_card_hash
                .as_deref()
                .map_or(Value::Null, |s| Value::String(s.to_string())),
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

        let event_checksum =
            compute_event_checksum(&payload).map_err(|e| HistoryError::Canonical(e.to_string()))?;
        payload.insert(
            "event_checksum".to_string(),
            Value::String(event_checksum.clone()),
        );

        let line = canonical_json(&Value::Object(payload))
            .map_err(|e| HistoryError::Canonical(e.to_string()))?;

        append_one_line_via_writer(path, &line, default_append_writer).map_err(|source| {
            HistoryError::Io {
                path: path.to_path_buf(),
                source,
            }
        })?;

        if let Some(fp) = fingerprint(path) {
            let cache_entry = AppendCacheEntry {
                st_dev: fp.0,
                st_ino: fp.1,
                st_size: fp.2,
                st_mtime_ns: fp.3,
                next_seq: seq + 1,
                last_checksum: Some(event_checksum.clone()),
            };
            HISTORY_APPEND_CACHE
                .lock()
                .expect("HISTORY_APPEND_CACHE mutex poisoned")
                .insert(key, cache_entry);
        } else {
            HISTORY_APPEND_CACHE
                .lock()
                .expect("HISTORY_APPEND_CACHE mutex poisoned")
                .remove(&key);
        }
        // `append_one_line_via_writer` writes the canonical-JSON
        // line + `\n` as a single payload of `line.len() + 1`
        // bytes. The post-write file size is therefore
        // deterministically `prior_size + line.len() + 1`. Capture
        // it under the lock so a later rollback can revalidate
        // before truncating.
        let post_write_size = prior_size_under_lock + line.len() as u64 + 1;
        Ok(AppendOutcome {
            seq,
            prior_size_under_lock,
            post_write_size,
        })
    })();

    let _ = flock(&lock_fd, FlockOperation::Unlock);
    drop(lock_fd);
    result
}

/// Re-acquire the history flock and conditionally truncate the history
/// file at `path` to `target_size`. The truncation runs ONLY when the
/// file's current size equals `expected_current_size` -- i.e., our
/// just-appended event is still the tail and no other writer has
/// appended after us. Returns `Ok(true)` after a successful truncation;
/// `Ok(false)` when the size mismatch means another writer appended
/// (rollback skipped to preserve the chain). Resets the process-local
/// append cache for `path` on truncation so the next append re-scans
/// the now-truncated tail.
///
/// Closes a race where a pre-flock prior_size + naive `set_len` /
/// `remove_file` could erase a concurrent writer's audit event: sizes
/// are captured under the lock (see `AppendOutcome`) and revalidated
/// here under a fresh lock before truncating.
///
/// `unlink_if_truncated_to_zero`: when `true` AND `target_size == 0`
/// (we just truncated to a zero-byte file because nothing existed
/// before our append), unlink the file BEFORE releasing the flock so
/// a concurrent writer cannot append in the gap and have its event
/// deleted by a post-helper unlink. Callers pass `!history_existed_before`
/// (the pre-flock heuristic) for this param so the
/// BL-20260518-no-clobber-foreign-sidecars guard against unlinking
/// a foreign zero-byte file is preserved.
pub fn truncate_history_after_append_if_unchanged(
    path: &Path,
    target_size: u64,
    expected_current_size: u64,
    unlink_if_truncated_to_zero: bool,
) -> Result<bool, HistoryError> {
    if !path.exists() {
        // Nothing to truncate. Either we never wrote, or another
        // writer (or a process restart cleanup) removed the file.
        // Treat as a no-op success.
        return Ok(true);
    }
    if !path.is_file() {
        return Err(HistoryError::NonFile(path.to_path_buf()));
    }
    let lock_path = lock_path_for(path);
    if let Some(parent) = lock_path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|source| HistoryError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    let lock_fd = rustix::fs::open(
        &lock_path,
        OFlags::WRONLY | OFlags::CREATE,
        Mode::from_bits_truncate(0o644),
    )
    .map_err(|e| HistoryError::Io {
        path: lock_path.clone(),
        source: std::io::Error::from(e),
    })?;
    flock(&lock_fd, FlockOperation::LockExclusive).map_err(|e| HistoryError::Io {
        path: lock_path.clone(),
        source: std::io::Error::from(e),
    })?;

    let result: Result<bool, HistoryError> = (|| {
        let current_size = std::fs::metadata(path)
            .map_err(|source| HistoryError::Io {
                path: path.to_path_buf(),
                source,
            })?
            .len();
        if current_size != expected_current_size {
            // Another writer appended after our event. Truncating
            // here would erase their valid event AND break their
            // chain. Leave the file alone.
            return Ok(false);
        }
        let file = std::fs::OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(|source| HistoryError::Io {
                path: path.to_path_buf(),
                source,
            })?;
        file.set_len(target_size)
            .map_err(|source| HistoryError::Io {
                path: path.to_path_buf(),
                source,
            })?;
        // Drop the file handle BEFORE the conditional unlink so the
        // remove_file syscall isn't racing with our own open fd.
        drop(file);
        // When we just truncated to zero AND no prior history file
        // existed before our append, unlink the file under the same
        // flock as the truncate so a concurrent writer cannot append
        // between our truncate and our unlink.
        if unlink_if_truncated_to_zero && target_size == 0 {
            let _ = std::fs::remove_file(path);
        }
        // Reset the process-local cache for this path so the next
        // append re-scans the truncated file (cache holds the
        // pre-truncate fingerprint which no longer matches).
        let key = cache_key(path);
        HISTORY_APPEND_CACHE
            .lock()
            .expect("HISTORY_APPEND_CACHE mutex poisoned")
            .remove(&key);
        Ok(true)
    })();

    let _ = flock(&lock_fd, FlockOperation::Unlock);
    drop(lock_fd);
    result
}

/// `validate_history` + `scan_tail` rolled into one trip for cache-miss
/// path. Returns `(next_seq, last_checksum)` for an already-clean file
/// or `Err(InvalidExisting { bad_lines })` for any invalid line.
fn full_scan(path: &Path) -> Result<(u64, Option<String>), HistoryError> {
    let bad = validate_history(path)?;
    if !bad.is_empty() {
        return Err(HistoryError::InvalidExisting {
            path: path.to_path_buf(),
            bad_lines: bad,
        });
    }
    scan_tail(path)
}

/// Scan the tail of an existing history file for `(next_seq, prev_checksum)`.
fn scan_tail(path: &Path) -> Result<(u64, Option<String>), HistoryError> {
    if !path.exists() {
        return Ok((0, None));
    }
    let text = std::fs::read_to_string(path).map_err(|source| HistoryError::Io {
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
        let event: HistoryEvent = serde_json::from_str(line).map_err(|e| HistoryError::Io {
            path: path.to_path_buf(),
            source: std::io::Error::other(format!("parse failure on tail scan: {e}")),
        })?;
        last_seq = event.seq as i64;
        last_checksum = Some(event.event_checksum);
    }
    Ok(((last_seq + 1) as u64, last_checksum))
}

/// Rebuild the canonical payload from a deserialized `HistoryEvent` for
/// re-checksum + re-canonicalize. The serde_json round-trip happens via
/// `serde_json::to_value`; alphabetical key ordering is enforced by
/// `canonical_json` at write time, so the field set (not the insertion
/// order here) determines byte-equality.
fn canonical_payload(event: &HistoryEvent) -> Map<String, Value> {
    let v = serde_json::to_value(event).expect("HistoryEvent serializes");
    #[allow(clippy::wildcard_enum_match_arm)]
    match v {
        Value::Object(m) => m,
        _ => unreachable!("HistoryEvent serializes to an object"),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn history_path_for_replaces_md_suffix() {
        let p = Path::new("cards/foo/bar.md");
        assert_eq!(
            history_path_for(p),
            PathBuf::from("cards/foo/bar.history.jsonl")
        );
    }

    #[test]
    fn history_path_for_appends_when_no_suffix() {
        let p = Path::new("cards/baz");
        assert_eq!(
            history_path_for(p),
            PathBuf::from("cards/baz.history.jsonl")
        );
    }

    #[test]
    fn validate_missing_file_returns_empty() {
        let p = PathBuf::from("/tmp/cacg-core-history-test-does-not-exist-xyz");
        let r = validate_history(&p).unwrap();
        assert!(r.is_empty());
    }

    #[test]
    fn validate_non_file_path_errors() {
        let dir = std::env::temp_dir();
        let r = validate_history(&dir);
        assert!(matches!(r, Err(HistoryError::NonFile(_))));
    }

    #[test]
    fn truncate_history_after_append_if_unchanged_refuses_when_file_changed() {
        // Codex R19 review (P2) regression: the rollback truncate
        // must refuse when another writer has appended after our
        // event so the concurrent writer's audit event is
        // preserved. This test simulates that scenario directly by
        // constructing a file whose current size exceeds the
        // `expected_current_size` passed to the helper. The helper
        // must return `Ok(false)` and leave the file byte-equal to
        // its pre-call state.
        let tmp = tempfile::TempDir::new().unwrap();
        let path = tmp.path().join("foo.history.jsonl");
        // Write a file with content that is LONGER than what we'll
        // claim as `expected_current_size`. The helper sees the
        // size mismatch and refuses to truncate.
        let initial_bytes = b"line0_pretend_event_appended_by_us\n\
                              line1_concurrent_writer_event\n";
        std::fs::write(&path, initial_bytes).unwrap();
        let real_size = initial_bytes.len() as u64;

        // Claim we expected the file to be size of just `line0`
        // (i.e., the post-write size from BEFORE the concurrent
        // append). The helper must observe `current_size > expected`
        // and refuse.
        let expected_pretend_post_write = b"line0_pretend_event_appended_by_us\n".len() as u64;
        let pretend_target = 0u64; // would truncate everything if helper were unsafe

        let result = truncate_history_after_append_if_unchanged(
            &path,
            pretend_target,
            expected_pretend_post_write,
            false,
        )
        .expect("helper must not propagate I/O error in the size-mismatch path");
        assert!(
            !result,
            "helper must return Ok(false) when current_size != expected_current_size"
        );

        // File content must be BYTE-EQUAL to the pre-call state --
        // the concurrent writer's event must survive.
        let after = std::fs::read(&path).unwrap();
        assert_eq!(
            after, initial_bytes,
            "file content must be unchanged when truncate refuses"
        );
        assert_eq!(
            std::fs::metadata(&path).unwrap().len(),
            real_size,
            "file size must be unchanged when truncate refuses"
        );
    }

    #[test]
    fn truncate_history_after_append_if_unchanged_truncates_when_file_matches() {
        // Positive case: when the file size still equals
        // `expected_current_size`, the helper truncates to
        // `target_size` and returns `Ok(true)`. This validates the
        // happy-path single-writer rollback that produced no
        // concurrent append.
        let tmp = tempfile::TempDir::new().unwrap();
        let path = tmp.path().join("foo.history.jsonl");
        let initial_bytes = b"line0_kept_prior_event\nline1_event_to_remove\n";
        std::fs::write(&path, initial_bytes).unwrap();
        let prior_size = b"line0_kept_prior_event\n".len() as u64;
        let post_write_size = initial_bytes.len() as u64;

        let result =
            truncate_history_after_append_if_unchanged(&path, prior_size, post_write_size, false)
                .expect("helper must succeed in the matching-size happy path");
        assert!(result, "helper must return Ok(true) when sizes match");
        let after = std::fs::read(&path).unwrap();
        assert_eq!(
            after, b"line0_kept_prior_event\n",
            "file must be truncated to `target_size` bytes"
        );
    }

    #[test]
    fn truncate_history_after_append_if_unchanged_unlinks_under_lock_when_truncated_to_zero() {
        // Codex R24 review (P2): a separate `std::fs::remove_file`
        // after the helper returns races a concurrent writer that
        // can acquire the lock and append in the gap. R25 moves
        // the unlink INSIDE the helper, under the same flock as
        // the truncate. Verifies the under-lock unlink branch:
        // when target_size == 0 AND `unlink_if_truncated_to_zero`
        // is true, the file is unlinked before the helper returns.
        let tmp = tempfile::TempDir::new().unwrap();
        let path = tmp.path().join("foo.history.jsonl");
        let initial_bytes = b"our_only_event_to_remove\n";
        std::fs::write(&path, initial_bytes).unwrap();
        let expected_post_write = initial_bytes.len() as u64;

        let result =
            truncate_history_after_append_if_unchanged(&path, 0, expected_post_write, true)
                .expect("helper must succeed in the matching-size happy path");
        assert!(result, "helper must return Ok(true) when sizes match");
        // File MUST be unlinked under the lock. After the helper
        // returns, the path does not exist.
        assert!(
            !path.exists(),
            "file must be unlinked when truncated to zero AND unlink_if_truncated_to_zero=true"
        );
    }

    #[test]
    fn truncate_history_after_append_if_unchanged_does_not_unlink_when_flag_is_false() {
        // Negative case: same scenario but the flag is false (e.g.,
        // caller's pre-flock heuristic said the history file
        // pre-existed). The truncate runs but the unlink does NOT
        // -- preserving the foreign zero-byte file
        // (BL-20260518-no-clobber-foreign-sidecars).
        let tmp = tempfile::TempDir::new().unwrap();
        let path = tmp.path().join("foo.history.jsonl");
        let initial_bytes = b"our_only_event_to_remove\n";
        std::fs::write(&path, initial_bytes).unwrap();
        let expected_post_write = initial_bytes.len() as u64;

        let result =
            truncate_history_after_append_if_unchanged(&path, 0, expected_post_write, false)
                .expect("helper must succeed in the matching-size happy path");
        assert!(result, "helper must return Ok(true) when sizes match");
        // File MUST still exist (truncated to zero bytes, NOT
        // unlinked).
        assert!(
            path.is_file(),
            "file must remain on disk when unlink_if_truncated_to_zero=false"
        );
        assert_eq!(
            std::fs::metadata(&path).unwrap().len(),
            0,
            "file must be truncated to zero bytes"
        );
    }
}
