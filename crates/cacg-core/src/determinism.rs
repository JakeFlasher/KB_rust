//! Explicit clock / event-id / temp-name source-of-truth.
//!
//! Every trust-kernel caller that currently passes a timestamp, event
//! id, UUID-like value, or temp-file name MUST obtain it from a
//! [`DeterminismContext`] instance rather than from a direct call to
//! [`time::OffsetDateTime::now_utc`], [`uuid::Uuid::new_v4`],
//! [`std::time::Instant::now`], `chrono::Utc::now`, or
//! [`tempfile::tempfile`]. The `xtask lint-determinism` static-grep
//! guard enforces this invariant; this module is the only allowlisted
//! caller of the real APIs.
//!
//! Under the `KB_FROZEN_CLOCK=1` environment variable the context
//! collapses every nondeterministic source to a fixed value
//! ([`FROZEN_TIMESTAMP`], [`FROZEN_UUID`], a monotonic counter-suffixed
//! temp name), so two independent runs over the same corpus produce
//! byte-identical artifacts. Otherwise the context delegates to the
//! real syscalls; the wall-clock format is the same
//! `YYYY-MM-DDTHH:MM:SSZ` shape Python's `cacg.clock.now_iso` emits
//! so the trust-kernel byte contract holds across implementations.
//!
//! Mirrors Python `legacy_python_oracle/src/cacg/clock.py`. The Rust port adds
//! [`DeterminismContext::next_temp_name`] because Rust's `tempfile`
//! ecosystem injects random suffixes; Python's `tmp_path = canonical.with_suffix(".tmp")`
//! pattern is deterministic by construction and needs no counterpart.

use std::sync::atomic::{AtomicU64, Ordering};

use time::OffsetDateTime;
use uuid::Uuid;

/// Frozen timestamp returned by [`DeterminismContext::now_iso`] when
/// `KB_FROZEN_CLOCK=1`. Matches Python `cacg.clock.FROZEN_TIMESTAMP`.
pub const FROZEN_TIMESTAMP: &str = "1970-01-01T00:00:00Z";

/// Frozen UUID returned by [`DeterminismContext::new_uuid`] when
/// `KB_FROZEN_CLOCK=1`. Matches Python `cacg.clock.FROZEN_UUID`.
pub const FROZEN_UUID: &str = "00000000-0000-0000-0000-000000000000";

/// Environment variable that toggles frozen mode. Set to the literal
/// string `"1"` to enable.
pub const FROZEN_ENV: &str = "KB_FROZEN_CLOCK";

/// Serializes tests that mutate the process-global `KB_FROZEN_CLOCK`
/// env var. `cargo test` runs tests in parallel within a single
/// process, so EVERY env-mutating test across the whole crate must
/// serialize against this one mutex — a per-module mutex only
/// serializes tests within its own file and still races a sibling
/// module's env test.
#[cfg(test)]
static FROZEN_CLOCK_TEST_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Lock [`FROZEN_CLOCK_TEST_MUTEX`], then set (`Some`) or clear
/// (`None`) `KB_FROZEN_CLOCK`. The returned guard must stay in scope
/// for the whole test so no sibling env-mutating test interleaves.
#[cfg(test)]
pub(crate) fn with_frozen_clock(value: Option<&str>) -> std::sync::MutexGuard<'static, ()> {
    let guard = FROZEN_CLOCK_TEST_MUTEX
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    match value {
        Some(v) => std::env::set_var(FROZEN_ENV, v),
        None => std::env::remove_var(FROZEN_ENV),
    }
    guard
}

/// Provides timestamps, event ids, and temp-file names.
///
/// Constructed once at the program entry point (or per-test) and
/// passed by reference to every primitive that needs nondeterministic
/// input. In frozen mode every method returns a fixed or monotonic
/// value; in live mode methods delegate to the underlying syscalls
/// using the same format conventions as Python.
pub struct DeterminismContext {
    frozen: bool,
    temp_counter: AtomicU64,
}

impl DeterminismContext {
    /// Construct a context whose frozen-ness is determined by the
    /// `KB_FROZEN_CLOCK` environment variable at construction time.
    /// Mirrors Python `cacg.clock.frozen()` semantics but reads the
    /// env once (the env may change between context instances; a
    /// long-running process should construct one context at startup).
    #[must_use]
    pub fn from_env() -> Self {
        let frozen = std::env::var(FROZEN_ENV).map(|v| v == "1").unwrap_or(false);
        Self {
            frozen,
            temp_counter: AtomicU64::new(0),
        }
    }

    /// Construct an explicitly-frozen context. Test escape hatch so a
    /// scenario can pin frozen mode regardless of the runner's env.
    #[must_use]
    pub fn frozen() -> Self {
        Self {
            frozen: true,
            temp_counter: AtomicU64::new(0),
        }
    }

    /// Construct an explicitly-live context. Test escape hatch.
    #[must_use]
    pub fn live() -> Self {
        Self {
            frozen: false,
            temp_counter: AtomicU64::new(0),
        }
    }

    /// `true` iff the context is in frozen mode.
    #[must_use]
    pub fn is_frozen(&self) -> bool {
        self.frozen
    }

    /// Return an ISO-8601 UTC timestamp. Frozen mode returns
    /// [`FROZEN_TIMESTAMP`]; live mode returns the current wall-clock
    /// formatted as `YYYY-MM-DDTHH:MM:SSZ` (matches Python's
    /// `datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")`).
    #[must_use]
    pub fn now_iso(&self) -> String {
        if self.frozen {
            return FROZEN_TIMESTAMP.to_string();
        }
        let dt = OffsetDateTime::now_utc();
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            dt.year(),
            u8::from(dt.month()),
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second()
        )
    }

    /// Return a UUID-v4 string. Frozen mode returns [`FROZEN_UUID`];
    /// live mode returns a freshly-generated UUID-v4. Matches Python's
    /// `str(uuid.uuid4())` byte shape (hyphenated, lowercase).
    #[must_use]
    pub fn new_uuid(&self) -> String {
        if self.frozen {
            return FROZEN_UUID.to_string();
        }
        Uuid::new_v4().to_string()
    }

    /// Return a deterministic temp-file name suffixed by a process-
    /// local monotonic counter in frozen mode (e.g., `"prefix.0000000000.tmp"`,
    /// `"prefix.0000000001.tmp"`, ...). In live mode the suffix uses
    /// the current wall-clock-nanoseconds-since-epoch so concurrent
    /// invocations cannot collide. Either way the caller's `prefix`
    /// (typically a canonical path stem) appears verbatim at the
    /// head of the returned name.
    ///
    /// This method exists so a caller never has to reach for
    /// [`tempfile::tempfile`] or [`tempfile::NamedTempFile`] (which
    /// would inject random suffixes that break byte-equal parity).
    /// The `xtask lint-determinism` regression guard catches such
    /// bypasses.
    #[must_use]
    pub fn next_temp_name(&self, prefix: &str) -> String {
        if self.frozen {
            let n = self.temp_counter.fetch_add(1, Ordering::Relaxed);
            format!("{prefix}.{n:010}.tmp")
        } else {
            let now_nanos = OffsetDateTime::now_utc().unix_timestamp_nanos();
            let counter = self.temp_counter.fetch_add(1, Ordering::Relaxed);
            format!("{prefix}.{now_nanos:020}.{counter:010}.tmp")
        }
    }
}

impl Default for DeterminismContext {
    fn default() -> Self {
        Self::from_env()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn frozen_context_returns_constants() {
        let ctx = DeterminismContext::frozen();
        assert!(ctx.is_frozen());
        assert_eq!(ctx.now_iso(), FROZEN_TIMESTAMP);
        assert_eq!(ctx.new_uuid(), FROZEN_UUID);
    }

    #[test]
    fn live_context_returns_nonzero_uuid() {
        let ctx = DeterminismContext::live();
        assert!(!ctx.is_frozen());
        let uuid = ctx.new_uuid();
        assert_ne!(uuid, FROZEN_UUID);
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid.chars().filter(|&c| c == '-').count(), 4);
    }

    #[test]
    fn live_context_now_iso_matches_python_format() {
        // 20-byte shape: YYYY-MM-DDTHH:MM:SSZ
        let ctx = DeterminismContext::live();
        let ts = ctx.now_iso();
        assert_eq!(ts.len(), 20, "expected 20-byte ISO format, got {ts:?}");
        assert!(ts.ends_with('Z'));
        assert_eq!(ts.chars().nth(4), Some('-'));
        assert_eq!(ts.chars().nth(7), Some('-'));
        assert_eq!(ts.chars().nth(10), Some('T'));
        assert_eq!(ts.chars().nth(13), Some(':'));
        assert_eq!(ts.chars().nth(16), Some(':'));
    }

    #[test]
    fn frozen_temp_name_is_monotonic_and_deterministic() {
        let ctx = DeterminismContext::frozen();
        let a = ctx.next_temp_name("foo");
        let b = ctx.next_temp_name("foo");
        let c = ctx.next_temp_name("bar");
        assert_eq!(a, "foo.0000000000.tmp");
        assert_eq!(b, "foo.0000000001.tmp");
        assert_eq!(c, "bar.0000000002.tmp");
    }

    #[test]
    fn frozen_temp_name_is_byte_identical_across_independent_contexts() {
        let ctx_a = DeterminismContext::frozen();
        let ctx_b = DeterminismContext::frozen();
        assert_eq!(ctx_a.next_temp_name("p"), ctx_b.next_temp_name("p"));
        assert_eq!(ctx_a.next_temp_name("p"), ctx_b.next_temp_name("p"));
    }

    #[test]
    fn live_temp_name_includes_prefix() {
        let ctx = DeterminismContext::live();
        let name = ctx.next_temp_name("card-X");
        assert!(name.starts_with("card-X."), "got {name:?}");
        assert!(name.ends_with(".tmp"), "got {name:?}");
    }

    #[test]
    fn from_env_without_var_falls_back_to_live() {
        // With `KB_FROZEN_CLOCK` cleared, `from_env()` must default to
        // live mode. `with_frozen_clock(None)` holds the crate-shared
        // mutex AND clears the env var, so this assertion runs
        // deterministically even under parallel tests — no sibling test
        // can set the var while the guard is held. The env-var-SET path
        // is covered by the child-process integration test in
        // `crates/cacg-core/tests/determinism_byte_identical.rs`.
        let _g = with_frozen_clock(None);
        let ctx = DeterminismContext::from_env();
        assert!(!ctx.is_frozen(), "unset env var must yield live mode");
    }
}
