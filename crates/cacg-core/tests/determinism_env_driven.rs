#![allow(clippy::unwrap_used)]
//! End-to-end proof that `KB_FROZEN_CLOCK=1` actually drives frozen
//! mode through `DeterminismContext::from_env()` and that artifacts
//! emitted by the trust-kernel primitives carry the frozen UUID +
//! frozen timestamp literally.
//!
//! Mechanics mirror `journal_subprocess_concurrent.rs`:
//! - The worker test is libtest-discovered; when run normally
//!   (without `CACG_FROZEN_ENV_OUTPUT_PATH`), it no-ops and passes.
//! - When invoked via the parent test's `current_exe()` with the
//!   env vars set, the worker constructs `DeterminismContext::from_env()`,
//!   asserts it is in frozen mode, then appends one journal event +
//!   one history event through the context.
//! - The parent test reads the worker's emitted bytes and asserts
//!   the frozen UUID `"00000000-0000-0000-0000-000000000000"` plus
//!   the frozen timestamp `"1970-01-01T00:00:00Z"` are present
//!   literally.
//!
//! This proves three things that the in-process
//! `DeterminismContext::frozen()` tests cannot:
//!   1. `from_env()` reads the real environment variable.
//!   2. `KB_FROZEN_CLOCK=1` is the recognized literal trigger.
//!   3. Context-threaded journal/history bytes match the frozen
//!      literals end-to-end under env-driven frozen mode.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus};
use std::time::{Duration, Instant};

use cacg_core::determinism::DeterminismContext;
use cacg_core::history::{
    append_history_event, history_path_for, reset_history_cache, HistoryEntry,
};
use cacg_core::journal::{append_entry, reset_append_cache, JournalEntry};
use serde_json::Map;
use tempfile::TempDir;

const ENV_OUTPUT_PATH: &str = "CACG_FROZEN_ENV_OUTPUT_PATH";
const KB_FROZEN_CLOCK: &str = "KB_FROZEN_CLOCK";
const WORKER_TEST_NAME: &str = "from_env_worker_writes_journal_and_history_when_env_set";
const WORKER_TIMEOUT: Duration = Duration::from_secs(60);

fn frozen_verification() -> BTreeMap<String, bool> {
    let mut v = BTreeMap::new();
    v.insert("fuzzy".to_string(), false);
    v.insert("layer1".to_string(), true);
    v.insert("layer2".to_string(), false);
    v
}

/// Worker test: when libtest invokes us with `CACG_FROZEN_ENV_OUTPUT_PATH`
/// set AND `KB_FROZEN_CLOCK=1` on the env, construct a context via
/// `from_env`, assert it picked up frozen mode, and append one
/// journal + one history event into the output directory. When
/// invoked normally (no env vars), no-op and pass.
#[test]
fn from_env_worker_writes_journal_and_history_when_env_set() {
    let output_dir = match std::env::var(ENV_OUTPUT_PATH) {
        Ok(p) => PathBuf::from(p),
        Err(_) => return, // harness-discovered no-op; parent has not invoked us.
    };
    let ctx = DeterminismContext::from_env();
    assert!(
        ctx.is_frozen(),
        "child must observe KB_FROZEN_CLOCK=1 and produce a frozen context"
    );

    // Journal write.
    reset_append_cache();
    let journal_path = output_dir.join("lint_journal.jsonl");
    let journal_entry = JournalEntry {
        command: "lint".to_string(),
        card_path: "cards/reading-01/env-driven.md".to_string(),
        card_hash_before: None,
        card_hash_after: Some("a".repeat(64)),
        diagnostics: Vec::new(),
        verification: frozen_verification(),
        latency_ms: 0.0,
    };
    let event_id = ctx.new_uuid();
    let timestamp = ctx.now_iso();
    append_entry(&journal_path, &journal_entry, &event_id, &timestamp)
        .expect("worker: append_entry through context");

    // History write.
    reset_history_cache();
    let card_path = output_dir.join("card.md");
    let history_path = history_path_for(&card_path);
    let mut delta = BTreeMap::new();
    delta.insert("added".to_string(), Vec::new());
    delta.insert("removed".to_string(), Vec::new());
    let history_entry = HistoryEntry {
        prev_card_hash: None,
        new_card_hash: "b".repeat(64),
        cited_chunk_set_delta: delta,
        frontmatter_field_changes: Vec::new(),
        cited_chunk_ids_snapshot: Vec::new(),
        frontmatter_snapshot: Map::new(),
        is_retracted: false,
    };
    let history_timestamp = ctx.now_iso();
    append_history_event(&history_path, &history_entry, &history_timestamp)
        .expect("worker: append_history_event through context");
}

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
fn from_env_with_kb_frozen_clock_produces_frozen_artifacts() {
    let dir = TempDir::new().expect("tempdir");
    let current_exe = std::env::current_exe().expect("current_exe");

    let child = Command::new(&current_exe)
        .args(["--exact", WORKER_TEST_NAME, "--nocapture"])
        .env(ENV_OUTPUT_PATH, dir.path())
        .env(KB_FROZEN_CLOCK, "1")
        .spawn()
        .expect("spawn worker subprocess");
    let status = wait_with_timeout(child, WORKER_TIMEOUT)
        .unwrap_or_else(|| panic!("worker did not exit within {WORKER_TIMEOUT:?}"));
    assert!(
        status.success(),
        "worker subprocess exited non-success: {status}"
    );

    let journal_text =
        std::fs::read_to_string(dir.path().join("lint_journal.jsonl")).expect("read journal");
    assert!(
        journal_text.contains("\"event_id\":\"00000000-0000-0000-0000-000000000000\""),
        "env-driven frozen context MUST produce the all-zero UUID in journal bytes; got: {journal_text}"
    );
    assert!(
        journal_text.contains("\"timestamp\":\"1970-01-01T00:00:00Z\""),
        "env-driven frozen context MUST produce the epoch timestamp in journal bytes; got: {journal_text}"
    );

    let history_text =
        std::fs::read_to_string(dir.path().join("card.history.jsonl")).expect("read history");
    assert!(
        history_text.contains("\"timestamp\":\"1970-01-01T00:00:00Z\""),
        "env-driven frozen context MUST produce the epoch timestamp in history bytes; got: {history_text}"
    );
}
