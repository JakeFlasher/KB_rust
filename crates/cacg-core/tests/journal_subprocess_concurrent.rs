#![allow(clippy::unwrap_used)]
//! AC-C5 cross-process concurrent-appender regression test (Codex R20 gap 1).
//!
//! Round 20 added an in-process multi-thread appender test, which
//! exercises the `APPEND_CACHE` mutex + flock path but NOT the
//! cross-process trust boundary. Real CACG users invoke `kb lint` from
//! parallel shells; each process has its own empty `APPEND_CACHE`
//! while sharing the on-disk `{path}.lock`. The flock `LOCK_EX` must
//! serialize these independent processes -- only the kernel-level lock
//! can, because the per-process cache is not shared.
//!
//! Mirrors Python `legacy_python_oracle/tests/test_phase4_journal_flock.py::test_concurrent_journal_appenders_produce_valid_chain`:
//! 4 worker processes each append 25 events; after all 4 join, the
//! resulting 100-line journal must validate clean and have seqs 0..99
//! contiguous.
//!
//! Test harness mechanics:
//! - The `worker_appends_events_when_env_set` test is libtest-discovered
//!   like any other test. When run normally (env vars absent), it
//!   no-ops and passes. When run via the parent's
//!   `Command::new(current_exe).args(["--exact", "...", "--nocapture"])`,
//!   it sees the env vars, runs the append loop, and exits.
//! - The `cross_process_appenders_produce_valid_chain` test is the
//!   parent: it spawns 4 of those filtered child invocations with
//!   distinct worker IDs and a shared `CACG_JOURNAL_WORKER_PATH`,
//!   waits for all four to complete (with a 60-second hard deadline),
//!   then asserts chain integrity.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus};
use std::time::{Duration, Instant};

use cacg_core::journal::{append_entry, validate_jsonl, JournalEntry};
use serde_json::Value;
use tempfile::TempDir;

const FROZEN_EVENT_ID: &str = "00000000-0000-0000-0000-000000000000";
const FROZEN_TIMESTAMP: &str = "1970-01-01T00:00:00Z";
const ENV_PATH: &str = "CACG_JOURNAL_WORKER_PATH";
const ENV_WORKER_ID: &str = "CACG_JOURNAL_WORKER_ID";
const ENV_WORKER_COUNT: &str = "CACG_JOURNAL_WORKER_COUNT";
const WORKER_TEST_NAME: &str = "worker_appends_events_when_env_set";
const WORKER_COUNT: usize = 4;
const EVENTS_PER_WORKER: usize = 25;
const WORKER_TIMEOUT: Duration = Duration::from_secs(60);

fn frozen_verification() -> BTreeMap<String, bool> {
    let mut v = BTreeMap::new();
    v.insert("layer1".to_string(), true);
    v.insert("layer2".to_string(), true);
    v
}

/// Worker test: when libtest invokes us with `CACG_JOURNAL_WORKER_PATH`
/// set, append K events to the shared journal under that path. When
/// invoked normally (no env vars), no-op and pass -- this keeps
/// `cargo test -p cacg-core --test journal_subprocess_concurrent` clean.
#[test]
fn worker_appends_events_when_env_set() {
    let path = match std::env::var(ENV_PATH) {
        Ok(p) => PathBuf::from(p),
        Err(_) => return, // harness-discovered no-op; parent has not invoked us.
    };
    let worker_id = std::env::var(ENV_WORKER_ID)
        .unwrap_or_else(|_| panic!("{ENV_WORKER_ID} required when {ENV_PATH} is set"));
    let k: usize = std::env::var(ENV_WORKER_COUNT)
        .unwrap_or_else(|_| panic!("{ENV_WORKER_COUNT} required when {ENV_PATH} is set"))
        .parse()
        .expect("CACG_JOURNAL_WORKER_COUNT must be a usize");

    for i in 0..k {
        let entry = JournalEntry {
            command: "lint".to_string(),
            card_path: format!("cards/{worker_id}/event-{i:03}.md"),
            card_hash_before: None,
            card_hash_after: Some("a".repeat(64)),
            diagnostics: Vec::new(),
            verification: frozen_verification(),
            latency_ms: 1.0,
        };
        append_entry(&path, &entry, FROZEN_EVENT_ID, FROZEN_TIMESTAMP)
            .unwrap_or_else(|e| panic!("worker {worker_id} append_entry({i}) failed: {e:?}"));
    }
}

/// Poll `child.try_wait` until either the child exits or `timeout`
/// elapses. On timeout, attempt to kill the child + reap it. Returns
/// `Some(status)` on natural exit; `None` on timeout.
fn wait_with_timeout(mut child: Child, timeout: Duration) -> Option<ExitStatus> {
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Some(status),
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return None;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
        }
    }
}

#[test]
fn cross_process_appenders_produce_valid_chain() {
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("lint_journal.jsonl");
    let current_exe = std::env::current_exe().expect("current_exe");

    let mut children: Vec<Child> = Vec::with_capacity(WORKER_COUNT);
    for wid in 0..WORKER_COUNT {
        let child = Command::new(&current_exe)
            .args(["--exact", WORKER_TEST_NAME, "--nocapture"])
            .env(ENV_PATH, &path)
            .env(ENV_WORKER_ID, format!("w{wid}"))
            .env(ENV_WORKER_COUNT, EVENTS_PER_WORKER.to_string())
            .spawn()
            .unwrap_or_else(|e| panic!("spawn worker {wid} failed: {e}"));
        children.push(child);
    }

    for (wid, child) in children.into_iter().enumerate() {
        let status = wait_with_timeout(child, WORKER_TIMEOUT)
            .unwrap_or_else(|| panic!("worker {wid} did not exit within {WORKER_TIMEOUT:?}"));
        assert!(
            status.success(),
            "worker {wid} exited with non-success status: {status}"
        );
    }

    let text = std::fs::read_to_string(&path).expect("read journal");
    let nonblank: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(
        nonblank.len(),
        WORKER_COUNT * EVENTS_PER_WORKER,
        "expected {} events; got {}",
        WORKER_COUNT * EVENTS_PER_WORKER,
        nonblank.len()
    );

    let bad = validate_jsonl(&path).expect("validate_jsonl");
    assert!(
        bad.is_empty(),
        "cross-process journal MUST validate clean; bad_lines={bad:?}"
    );

    let mut seqs: Vec<u64> = Vec::with_capacity(WORKER_COUNT * EVENTS_PER_WORKER);
    for line in &nonblank {
        let event: Value = serde_json::from_str(line).expect("event parses");
        seqs.push(event["seq"].as_u64().expect("seq is u64"));
    }
    seqs.sort_unstable();
    let expected: Vec<u64> = (0..(WORKER_COUNT * EVENTS_PER_WORKER) as u64).collect();
    assert_eq!(
        seqs, expected,
        "cross-process seqs MUST be 0..{} contiguous (kernel flock serialized N independent processes)",
        WORKER_COUNT * EVENTS_PER_WORKER - 1
    );
}
