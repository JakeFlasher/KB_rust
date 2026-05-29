#![allow(
    clippy::unwrap_used,
    clippy::ptr_arg,
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
//! Two independent runs over the same corpus under a frozen
//! [`DeterminismContext`] must produce byte-identical artifacts.
//!
//! Proof model: build two distinct `DeterminismContext::frozen()`
//! instances, thread each through `journal::append_entry` and
//! `history::append_history_event` over the same input corpus, and
//! byte-compare the resulting files. Any nondeterminism leakage
//! (e.g., a regression that bypasses the context and calls
//! `time::OffsetDateTime::now_utc` directly) would produce divergent
//! byte streams and trip this test.
//!
//! The full end-to-end byte-equal proof — run the `kb` binary twice
//! under `KB_FROZEN_CLOCK=1` and byte-compare the resulting
//! `cards_manifest.json` / `summaries.json` / `lint_journal.jsonl` /
//! per-card `history.jsonl` — belongs in the `cacg-cli` integration
//! suite once that binary is implemented. Until then, this test
//! exercises the trust-kernel primitives (`journal::append_entry`
//! + `history::append_history_event`) directly with two independent
//! frozen contexts, proving the equivalent invariant at the layer
//! that owns the byte contract.

use std::collections::BTreeMap;
use std::path::PathBuf;

use cacg_core::determinism::DeterminismContext;
use cacg_core::history::{
    append_history_event, history_path_for, reset_history_cache, HistoryEntry,
    RETRACTION_FRONTMATTER_MARKER,
};
use cacg_core::journal::{append_entry, reset_append_cache, JournalEntry};
use serde_json::Map;
use tempfile::TempDir;

fn frozen_verification() -> BTreeMap<String, bool> {
    let mut v = BTreeMap::new();
    v.insert("fuzzy".to_string(), false);
    v.insert("layer1".to_string(), true);
    v.insert("layer2".to_string(), false);
    v
}

fn nth_journal_entry(i: usize) -> JournalEntry {
    JournalEntry {
        command: "lint".to_string(),
        card_path: format!("cards/reading-01/card-{i:03}.md"),
        card_hash_before: None,
        card_hash_after: Some(format!("{:0>64x}", i + 1)),
        diagnostics: Vec::new(),
        verification: frozen_verification(),
        latency_ms: 0.0,
    }
}

fn build_journal_with_context(ctx: &DeterminismContext, dir: &PathBuf, n: usize) -> Vec<u8> {
    let path = dir.join("lint_journal.jsonl");
    reset_append_cache();
    for i in 0..n {
        let entry = nth_journal_entry(i);
        let event_id = ctx.new_uuid();
        let timestamp = ctx.now_iso();
        append_entry(&path, &entry, &event_id, &timestamp)
            .unwrap_or_else(|e| panic!("append_entry({i}) failed: {e:?}"));
    }
    std::fs::read(&path).expect("read journal bytes")
}

#[test]
fn two_independent_frozen_contexts_produce_byte_identical_journals() {
    const N: usize = 10;
    let dir_a = TempDir::new().unwrap();
    let dir_b = TempDir::new().unwrap();
    let ctx_a = DeterminismContext::frozen();
    let ctx_b = DeterminismContext::frozen();
    assert!(ctx_a.is_frozen());
    assert!(ctx_b.is_frozen());

    let bytes_a = build_journal_with_context(&ctx_a, &dir_a.path().to_path_buf(), N);
    let bytes_b = build_journal_with_context(&ctx_b, &dir_b.path().to_path_buf(), N);

    assert_eq!(
        bytes_a, bytes_b,
        "two independent frozen contexts must produce byte-identical journals over the same corpus"
    );
}

fn nth_history_entry(i: usize, prev: Option<&str>) -> HistoryEntry {
    let new_hash = format!("{:0>64x}", i + 1);
    let mut delta = BTreeMap::new();
    delta.insert("added".to_string(), Vec::new());
    delta.insert("removed".to_string(), Vec::new());
    let is_retracted = i % 4 == 0;
    HistoryEntry {
        prev_card_hash: prev.map(str::to_string),
        new_card_hash: new_hash,
        cited_chunk_set_delta: delta,
        frontmatter_field_changes: if is_retracted {
            vec![RETRACTION_FRONTMATTER_MARKER.to_string()]
        } else {
            Vec::new()
        },
        cited_chunk_ids_snapshot: Vec::new(),
        frontmatter_snapshot: Map::new(),
        is_retracted,
    }
}

fn build_history_with_context(ctx: &DeterminismContext, dir: &PathBuf, n: usize) -> Vec<u8> {
    let card = dir.join("card.md");
    let path = history_path_for(&card);
    reset_history_cache();
    let mut prev: Option<String> = None;
    for i in 0..n {
        let entry = nth_history_entry(i, prev.as_deref());
        prev = Some(entry.new_card_hash.clone());
        let timestamp = ctx.now_iso();
        append_history_event(&path, &entry, &timestamp)
            .unwrap_or_else(|e| panic!("append_history_event({i}) failed: {e:?}"));
    }
    std::fs::read(&path).expect("read history bytes")
}

#[test]
fn two_independent_frozen_contexts_produce_byte_identical_histories() {
    const N: usize = 8;
    let dir_a = TempDir::new().unwrap();
    let dir_b = TempDir::new().unwrap();
    let ctx_a = DeterminismContext::frozen();
    let ctx_b = DeterminismContext::frozen();

    let bytes_a = build_history_with_context(&ctx_a, &dir_a.path().to_path_buf(), N);
    let bytes_b = build_history_with_context(&ctx_b, &dir_b.path().to_path_buf(), N);

    assert_eq!(
        bytes_a, bytes_b,
        "two independent frozen contexts must produce byte-identical histories over the same corpus"
    );
}

#[test]
fn frozen_context_threads_through_journal_with_zero_uuid_and_epoch_timestamp() {
    // Sanity: the journal lines produced via the context contain the
    // exact frozen UUID + frozen timestamp the trust kernel pins.
    let dir = TempDir::new().unwrap();
    let ctx = DeterminismContext::frozen();
    let bytes = build_journal_with_context(&ctx, &dir.path().to_path_buf(), 1);
    let text = String::from_utf8(bytes).expect("utf8");
    assert!(
        text.contains("\"event_id\":\"00000000-0000-0000-0000-000000000000\""),
        "frozen context must produce the all-zero UUID: {text}"
    );
    assert!(
        text.contains("\"timestamp\":\"1970-01-01T00:00:00Z\""),
        "frozen context must produce the epoch timestamp: {text}"
    );
}

#[test]
fn live_context_threads_through_journal_with_nonzero_values() {
    // Sanity: live context produces a 36-byte UUID + a 20-byte
    // ISO timestamp that is NOT the frozen literal. This rules out
    // a bug where `now_iso` accidentally returns FROZEN_TIMESTAMP in
    // live mode.
    let dir = TempDir::new().unwrap();
    let ctx = DeterminismContext::live();
    let bytes = build_journal_with_context(&ctx, &dir.path().to_path_buf(), 1);
    let text = String::from_utf8(bytes).expect("utf8");
    assert!(
        !text.contains("\"event_id\":\"00000000-0000-0000-0000-000000000000\""),
        "live context must NOT produce the frozen UUID literally: {text}"
    );
    assert!(
        !text.contains("\"timestamp\":\"1970-01-01T00:00:00Z\""),
        "live context must NOT produce the frozen timestamp literally: {text}"
    );
}
