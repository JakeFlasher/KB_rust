#![allow(clippy::unwrap_used)]
//! End-to-end locked transcript test for the
//! `kb verify --round-summary --semantic <cache>` pipeline.
//!
//! Exercises the full path: Layer-1 chunk-hash lookup passes
//! against the committed parity-corpus manifest → Layer-2
//! substring containment fails (synthetic quote intentionally
//! absent from the chunk text) → `CACG-VERIFY-001` emits →
//! Layer-3 cache hit on the (`chunk_hash`, `claim_window_hash`)
//! pair produces a `pass` verdict → `CACG-VERIFY-002` emits
//! alongside `CACG-VERIFY-001` in the same diagnostics array,
//! severity `warning` (the per-verdict severity contract:
//! `pass` → Warning because Layer-2 already failed).
//!
//! The primary lock is a byte-equal assertion on a
//! deterministic transcript string covering:
//!
//! - `exit_code` integer
//! - `stdout` bytes (lossy → String, then path-normalized)
//! - `stderr` bytes (lossy → String, then path-normalized)
//! - `journal` projection (one canonical JSON object per
//!   event, keys: `card_path`, `verification`, `diagnostics`
//!   — all path-normalized; volatile fields like
//!   `card_hash_*` and `latency_ms` excluded)
//! - `load_trace` content from
//!   `CACG_SEMANTIC_LOAD_TRACE` (one `load_ok <path> 1`
//!   line — the cache loaded exactly once at startup)
//!
//! Path-volatility is removed via the placeholders `<TMP>`
//! (replaces every occurrence of the test's tempdir path)
//! and `<WORKSPACE>` (replaces every occurrence of the
//! workspace root path). All other bytes are preserved
//! exactly.
//!
//! Any cross-AC drift surfaces as a transcript-mismatch
//! failure: severity change, exit-code-ladder change,
//! per-card diagnostic count change, message-format
//! change (decimal precision, missing verdict/mode tag),
//! stdout/stderr routing change for non-Verified verdicts,
//! per-cite cache reload (would emit a second `load_ok`
//! line in the trace), or any new informational line on
//! either stream.
//!
//! Secondary assertions below the transcript lock fire
//! only as diagnostic hints when the transcript lock
//! fails — they help pinpoint WHICH locked observable
//! drifted.
//!
//! The fixture is built at test time rather than committed
//! as a static JSON file because the cache key depends on
//! `claim_window_hash(quote)`. Computing the key inline
//! keeps the semantic-hit scenario deterministic without a
//! committed-hash maintenance burden: changing the synthetic
//! quote in this file automatically updates the cache key
//! the test verifies against. This does NOT itself catch
//! drift in the normalization pipeline — the test uses the
//! same `claim_window_hash` for both the build and the
//! verify sides, so a divergent normalization would still
//! agree with itself. Hash normalization parity is covered
//! by the Python-derived golden-vector tests in
//! `cacg-semantic` (the 5 hash classes that pin the
//! Python ↔ Rust contract for `claim_window_hash`).

use std::path::PathBuf;
use std::process::Command;

use cacg_core::verify::claim_window_hash;
use serde_json::{json, Value};
use tempfile::TempDir;

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

/// The well-known `chunk_hash` + `chunk_id` from the committed
/// parity-corpus `chunks_manifest` used by the other synthetic
/// round-summary semantic tests. Layer-1 hash + manifest-
/// tamper checks pass against this.
const CHUNK_A_HASH: &str = "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895";
const CHUNK_A_ID: &str = "sample:p001:0000";
const SOURCE_ID: &str = "sample";

/// Synthetic quote intentionally NOT a substring of `chunk_a`'s
/// text (which starts with "Content-addressable identity is
/// the verification primitive..."). Layer-2 substring
/// containment therefore fails → `CACG-VERIFY-001` emits →
/// Layer-3 fires.
const SYNTHETIC_QUOTE: &str =
    "locked transcript synthetic quote intentionally absent from the pinned chunk";

/// Locked cache verdict score. Six-decimal-formatted in the
/// CACG-VERIFY-002 message → the lock pins exactly
/// `score=0.950000`.
const LOCKED_VERDICT_SCORE: f64 = 0.95;

/// Apply path normalization: replace tempdir and workspace
/// root paths with stable placeholders so the transcript
/// is byte-stable across runs.
fn normalize_paths(text: &str, tmp: &str, workspace: &str) -> String {
    text.replace(tmp, "<TMP>").replace(workspace, "<WORKSPACE>")
}

/// Project one journal event JSON object into a deterministic
/// shape with only the load-bearing fields:
/// `card_path`, `verification`, `diagnostics`.
/// Volatile fields (`card_hash_*`, `latency_ms`, `timestamp`,
/// `command`) are excluded — the transcript lock catches
/// drift on the locked-transcript-relevant surface, and the
/// excluded fields are covered by other gates.
fn project_journal_event(event: &Value) -> Value {
    json!({
        "card_path": event.get("card_path").cloned().unwrap_or(Value::Null),
        "verification": event.get("verification").cloned().unwrap_or(Value::Null),
        "diagnostics": event.get("diagnostics").cloned().unwrap_or(Value::Null),
    })
}

#[test]
#[allow(clippy::too_many_lines)]
fn round_summary_semantic_e2e_locked_transcript() {
    let tmp = TempDir::new().expect("tempdir");
    let tmp_str = tmp.path().to_str().expect("utf-8 tempdir path").to_string();
    let workspace_str = workspace_root()
        .to_str()
        .expect("utf-8 workspace path")
        .to_string();

    // Compute the cache key for our synthetic quote via the
    // in-Rust normalization + SHA-256 pipeline. Hard-coding
    // a precomputed hash would silently drift if the
    // normalization ever changed; this guarantees the cache
    // ALWAYS hits the entry the synthetic card cites.
    let claim_hash = claim_window_hash(SYNTHETIC_QUOTE);

    // ---- Build the synthetic semantic cache (1 entry) ----
    let cache_path = tmp.path().join("locked_semantic_cache.json");
    let cache_body = json!({
        "schema_version": "cacg.v0",
        "entries": [
            {
                "chunk_hash": CHUNK_A_HASH,
                "claim_window_hash": claim_hash,
                "verdict": "pass",
                "score": LOCKED_VERDICT_SCORE,
            }
        ]
    });
    std::fs::write(&cache_path, serde_json::to_string(&cache_body).unwrap()).expect("write cache");

    // ---- Build the synthetic card pinning the chunk ----
    let cards_dir = tmp.path().join("cards/reading_01");
    std::fs::create_dir_all(&cards_dir).expect("mkdir cards");
    let card_body = format!(
        "---\n\
        schema_version: \"cacg.v0\"\n\
        id: \"locked-transcript-card\"\n\
        title: \"Locked Transcript Card\"\n\
        reading_id: \"reading_01\"\n\
        summary: \"Synthetic card whose citation quote is intentionally absent from the pinned chunk text, forcing Layer-2 failure and Layer-3 cache-hit firing.\"\n\
        citations:\n\
        \x20\x20- source_id: \"{SOURCE_ID}\"\n\
        \x20\x20\x20\x20chunk_id: \"{CHUNK_A_ID}\"\n\
        \x20\x20\x20\x20chunk_hash: \"{CHUNK_A_HASH}\"\n\
        \x20\x20\x20\x20page_range: [1, 2]\n\
        \x20\x20\x20\x20quote: \"{SYNTHETIC_QUOTE}\"\n\
        \x20\x20\x20\x20edge_type: \"supports\"\n\
        ---\n\
        Body.\n"
    );
    let card_relative = "cards/reading_01/locked-transcript-card.md";
    let card_path = tmp.path().join(card_relative);
    std::fs::write(&card_path, card_body).expect("write card");

    // ---- Build the round summary citing the card ----
    let summary_path = tmp.path().join("summary.md");
    let summary_body = format!(
        "## Knowledge Consulted\n\n\
        - {card_relative}\n",
    );
    std::fs::write(&summary_path, summary_body).expect("write summary");

    let journal_path = tmp.path().join("lint_journal.jsonl");
    let load_trace_path = tmp.path().join("semantic_load_trace.txt");
    let chunks_manifest =
        workspace_root().join("tests/parity_corpus/out_python/chunks_manifest.json");
    assert!(
        chunks_manifest.is_file(),
        "committed parity-corpus chunks_manifest must exist for the locked-transcript fixture",
    );
    let source_matrix = workspace_root().join("tests/parity_corpus/out_python/source_matrix.json");

    // ---- Run kb verify --round-summary --semantic <cache> ----
    // current_dir = tempdir so the round-summary path resolver
    // finds the synthetic card via the tempdir-relative cite.
    // KB_FROZEN_CLOCK=1 keeps timestamps + latency deterministic.
    // CACG_SEMANTIC_LOAD_TRACE captures cache-load cardinality
    // so the per-cite cache-reload drift claim is actually
    // backed by an assertion (exactly one `load_ok` line).
    let output = Command::new(kb_bin())
        .current_dir(tmp.path())
        .arg("verify")
        .arg("--round-summary")
        .arg(&summary_path)
        .arg("--chunks-manifest")
        .arg(&chunks_manifest)
        .arg("--source-matrix")
        .arg(&source_matrix)
        .arg("--journal")
        .arg(&journal_path)
        .arg("--semantic")
        .arg(&cache_path)
        .env("KB_FROZEN_CLOCK", "1")
        .env("CACG_SEMANTIC_LOAD_TRACE", &load_trace_path)
        .output()
        .expect("spawn kb verify --round-summary --semantic");

    let exit_code = output.status.code().expect("exit code");
    let stdout_raw = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr_raw = String::from_utf8_lossy(&output.stderr).into_owned();

    // ---- Read + parse the journal ----
    let journal_raw = std::fs::read_to_string(&journal_path).expect("journal file must be created");
    let journal_lines: Vec<&str> = journal_raw.lines().filter(|l| !l.is_empty()).collect();
    let journal_projection_lines: Vec<String> = journal_lines
        .iter()
        .map(|line| {
            let event: Value =
                serde_json::from_str(line).expect("journal event must parse as JSON");
            let projected = project_journal_event(&event);
            // Canonical: pretty-print with stable key order
            // (serde_json::to_string_pretty preserves the
            // explicit insertion order from project_journal_event).
            serde_json::to_string_pretty(&projected).unwrap()
        })
        .collect();
    let journal_normalized = normalize_paths(
        &journal_projection_lines.join("\n---\n"),
        &tmp_str,
        &workspace_str,
    );

    // ---- Read the load-trace ----
    let load_trace_raw = std::fs::read_to_string(&load_trace_path)
        .expect("load-trace file must be created by CACG_SEMANTIC_LOAD_TRACE");
    let load_trace_normalized = normalize_paths(&load_trace_raw, &tmp_str, &workspace_str);

    // ---- Compose the actual transcript ----
    let stdout_normalized = normalize_paths(&stdout_raw, &tmp_str, &workspace_str);
    let stderr_normalized = normalize_paths(&stderr_raw, &tmp_str, &workspace_str);
    let actual_transcript = format!(
        "exit_code={exit_code}\n\
        stdout:\n\
        {stdout_normalized}\
        stderr:\n\
        {stderr_normalized}\
        journal:\n\
        {journal_normalized}\n\
        load_trace:\n\
        {load_trace_normalized}",
    );

    // ---- The EXPECTED transcript ----
    //
    // Fields appear in alphabetical order because the
    // journal entries were re-serialized through
    // `serde_json::to_string_pretty` over a
    // `serde_json::Value`, which sorts keys alphabetically.
    // The BM25 hint scores (-0.167814, -0.181373) are
    // deterministic over the committed parity-corpus
    // chunks_manifest — they reflect BM25's stable
    // tokenization + IDF over the two-chunk corpus and
    // the synthetic quote. The two hint entries are the
    // top-2 hits (the cited chunk + the other chunk in
    // the same source). All bytes are byte-stable; any
    // drift on a locked observable fails this string
    // compare.
    //
    // `card_relative` is included in the format args so
    // the macro doesn't fail with an "unused variable"
    // warning — it appears as the path in the STALE line.
    let _ = card_relative; // documented above.
    let expected_transcript = format!(
"exit_code=1
stdout:
stderr:
cards/reading_01/locked-transcript-card.md: STALE (verify failed: CACG-VERIFY-001)
journal:
{{
  \"card_path\": \"<TMP>/cards/reading_01/locked-transcript-card.md\",
  \"diagnostics\": [
    {{
      \"code\": \"CACG-VERIFY-001\",
      \"file\": \"<TMP>/cards/reading_01/locked-transcript-card.md\",
      \"hints\": [
        {{
          \"chunk_id\": \"sample:p002:0001\",
          \"hint_only\": true,
          \"score\": -0.167814,
          \"text_preview\": \"Tens of thousands of tokens compress to one focused result. Round contracts pin the unit of work. One mainline objective\"
        }},
        {{
          \"chunk_id\": \"sample:p001:0000\",
          \"hint_only\": true,
          \"score\": -0.181373,
          \"text_preview\": \"Content-addressable identity is the verification primitive at the core of the framework. Every chunk hashes to a stable \"
        }}
      ],
      \"message\": \"citations[0]: quote not found in pinned chunk {CHUNK_A_ID} (fuzzy=False)\",
      \"severity\": \"error\"
    }},
    {{
      \"code\": \"CACG-VERIFY-002\",
      \"file\": \"<TMP>/cards/reading_01/locked-transcript-card.md\",
      \"hints\": [
        {{
          \"semantic_mode\": \"embedding-cache\",
          \"semantic_score\": 0.95,
          \"semantic_verdict\": \"pass\"
        }}
      ],
      \"message\": \"semantic verdict=pass score=0.950000 mode=embedding-cache\",
      \"severity\": \"warning\"
    }}
  ],
  \"verification\": {{
    \"fuzzy\": false,
    \"layer1\": true,
    \"layer2\": false
  }}
}}
load_trace:
load_ok <TMP>/locked_semantic_cache.json 1
"
    );

    // -----------------------------------------------------
    // PRIMARY LOCK: byte-equal transcript.
    // Any cross-AC drift on a locked observable fails here.
    // -----------------------------------------------------
    assert_eq!(
        actual_transcript, expected_transcript,
        "LOCKED TRANSCRIPT BYTE-EQUAL DRIFT.\n\n\
         === ACTUAL ===\n{actual_transcript}\n\
         === EXPECTED ===\n{expected_transcript}",
    );

    // -----------------------------------------------------
    // Secondary diagnostic assertions: redundant when the
    // primary lock holds; they help pinpoint WHICH locked
    // observable drifted if the primary assertion ever
    // fails. They are not load-bearing (the byte-equal
    // transcript covers each).
    // -----------------------------------------------------

    assert_eq!(exit_code, 1, "exit-code secondary assertion");
    assert_eq!(
        journal_lines.len(),
        1,
        "journal cardinality secondary assertion",
    );
    // Load-trace must be exactly one `load_ok` line and zero
    // `load_err` lines — proves cache-load cardinality.
    let load_ok_count = load_trace_raw
        .lines()
        .filter(|l| l.starts_with("load_ok "))
        .count();
    let load_err_count = load_trace_raw
        .lines()
        .filter(|l| l.starts_with("load_err "))
        .count();
    assert_eq!(load_ok_count, 1, "load_ok count secondary assertion");
    assert_eq!(load_err_count, 0, "load_err count secondary assertion");
}
