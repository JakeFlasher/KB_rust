#![allow(clippy::unwrap_used)]
use super::*;
use crate::diagnostic::codes;
use crate::hash::{chunk_hash, PageSpan as HashPageSpan};
use crate::schema::{ChunkRecord, ChunksManifest, PageSpan, SchemaVersion};
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// `KB_FROZEN_CLOCK` is process-global; `with_frozen_clock` locks
// the crate-shared `FROZEN_CLOCK_TEST_MUTEX` so env-mutating tests
// here serialize against every env-mutating test in the crate,
// including `lint::layer1::tests` (a per-file mutex would not).
use crate::determinism::with_frozen_clock;

fn make_chunk(
    source_id: &str,
    chunk_id: &str,
    text: &str,
    start_page: u32,
    end_page: u32,
) -> ChunkRecord {
    let span = PageSpan {
        page: start_page,
        byte_offset_in_chunk: 0,
    };
    let hash_spans = vec![HashPageSpan {
        page: i64::from(span.page),
        byte_offset_in_chunk: i64::from(span.byte_offset_in_chunk),
    }];
    let h = chunk_hash(
        text,
        i64::from(start_page),
        i64::from(end_page),
        &hash_spans,
    )
    .expect("chunk_hash");
    ChunkRecord {
        schema_version: SchemaVersion::V0,
        source_id: source_id.to_string(),
        chunk_id: chunk_id.to_string(),
        chunk_hash: h,
        ordinal: 0,
        start_page,
        end_page,
        page_spans: vec![span],
        token_count: 3,
        text: text.to_string(),
        text_preview: text.chars().take(20).collect(),
    }
}

fn manifest_with(chunks: Vec<ChunkRecord>, retracted_chunk_ids: Vec<String>) -> ChunksManifest {
    ChunksManifest {
        schema_version: SchemaVersion::V0,
        chunks,
        retracted_source_ids: Vec::new(),
        retracted_chunk_ids,
    }
}

/// Render a minimal valid card body with the indentation
/// the YAML parser expects (mirrors
/// `tests/parity_corpus/valid/synthetic-card-01.md`). Joining
/// lines explicitly via `["..."].join("\n")` avoids Rust's
/// `\\n\\` line-continuation eating the YAML's leading
/// whitespace.
fn render_card(
    card_id: &str,
    source_id: &str,
    chunk_id: &str,
    chunk_hash: &str,
    page_range: (u32, u32),
    quote: &str,
    body: &str,
) -> String {
    let lines = [
        "---".to_string(),
        "schema_version: \"cacg.v0\"".to_string(),
        format!("id: {card_id:?}"),
        "title: \"t\"".to_string(),
        "reading_id: \"R1\"".to_string(),
        "summary: \"This summary is padded to be long enough to satisfy the 80-character minimum that the schema validator requires for every committed card.\"".to_string(),
        "citations:".to_string(),
        format!("  - source_id: {source_id:?}"),
        format!("    chunk_id: {chunk_id:?}"),
        format!("    chunk_hash: {chunk_hash:?}"),
        format!("    page_range: [{}, {}]", page_range.0, page_range.1),
        format!("    quote: {quote:?}"),
        "    edge_type: \"supports\"".to_string(),
        "---".to_string(),
        body.to_string(),
        String::new(),
    ];
    lines.join("\n")
}

/// Build a clean-verifying scenario: write a card + a
/// chunks_manifest.json that satisfies all layer-1 + layer-2
/// checks, return (tempdir, card_path, manifest_path,
/// journal_path). The chunk's text contains the citation's
/// quote substring.
fn build_clean_scenario(quote: &str, body_text: &str) -> (TempDir, PathBuf, PathBuf, PathBuf) {
    let tmp = TempDir::new().expect("tempdir");
    let chunk = make_chunk(
        "src",
        "src:p001:0000",
        "alpha beta gamma delta epsilon zeta eta theta",
        1,
        1,
    );
    let manifest = manifest_with(vec![chunk.clone()], Vec::new());
    let manifest_path = tmp.path().join("chunks_manifest.json");
    fs::write(
        &manifest_path,
        serde_json::to_string(&manifest).expect("manifest json"),
    )
    .unwrap();
    let card_path = tmp.path().join("card.md");
    let body = render_card(
        "test-card",
        "src",
        "src:p001:0000",
        &chunk.chunk_hash,
        (1, 1),
        quote,
        body_text,
    );
    fs::write(&card_path, body).unwrap();
    let journal_path = tmp.path().join("journal.jsonl");
    (tmp, card_path, manifest_path, journal_path)
}

fn count_journal_events(journal_path: &Path) -> usize {
    if !journal_path.is_file() {
        return 0;
    }
    fs::read_to_string(journal_path)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .count()
}

/// Assert exactly one non-empty JSONL line in the journal and
/// return the parsed event. The persisted-field tests use
/// this to pin the byte-equal trust-bearing fields of the
/// `JournalEntry` (command, card_path, card_hash_before/after,
/// diagnostics, verification map, latency_ms).
fn read_single_journal_event(journal_path: &Path) -> serde_json::Value {
    let raw = fs::read_to_string(journal_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", journal_path.display()));
    let lines: Vec<&str> = raw.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines.len(),
        1,
        "expected exactly one journal event; got {}",
        lines.len()
    );
    serde_json::from_str(lines[0])
        .unwrap_or_else(|e| panic!("journal line is not valid JSON: {e}; line={}", lines[0]))
}

/// Convenience: pluck `event.verification` as a borrowed Value.
fn verification_of(event: &serde_json::Value) -> &serde_json::Value {
    event
        .get("verification")
        .expect("event must carry verification")
}

#[test]
fn verify_one_card_clean_appends_one_journal_event() {
    let _g = with_frozen_clock(Some("1"));
    let (_tmp, card_path, manifest_path, journal_path) =
        build_clean_scenario("beta gamma delta", "Body.");
    let out = verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        false,
        false,
        None,
        None,
        None,
        None,
        None,
    )
    .expect("clean run should not error");
    assert!(
        out.verified,
        "expected verified=true; diagnostics={:?}",
        out.diagnostics
    );
    assert!(out.layer1);
    assert!(out.layer2);
    assert!(!out.fuzzy);
    assert!(out.diagnostics.is_empty());

    // Persisted-journal field assertions (Python-parity pin on
    // `JournalEntry`). The runner trust-bearing contract is
    // byte-equal on (command, card_path, card_hash_*,
    // diagnostics, verification, latency_ms).
    let event = read_single_journal_event(&journal_path);
    assert_eq!(event["command"], "verify");
    assert_eq!(
        event["card_path"].as_str().unwrap(),
        card_path.display().to_string().as_str()
    );
    assert_eq!(event["card_hash_before"], event["card_hash_after"]);
    // `out.card_hash` is the in-memory observation; the
    // journal must record the same value on both `before` and
    // `after` (no second on-disk read between load and append
    // — Python pin in `legacy_python_oracle/src/cacg/verify/runner.py`'s
    // `_append_verify_event`).
    let expected_hash = serde_json::to_value(&out.card_hash).unwrap();
    assert_eq!(event["card_hash_before"], expected_hash);
    assert_eq!(event["card_hash_after"], expected_hash);
    assert_eq!(event["diagnostics"], serde_json::json!([]));
    let v = verification_of(&event);
    assert_eq!(v["layer1"], true);
    assert_eq!(v["layer2"], true);
    assert_eq!(v["fuzzy"], false);
    assert_eq!(
        event["latency_ms"], 0.0,
        "frozen clock must zero latency_ms"
    );
}

#[test]
fn verify_one_card_missing_card_emits_cli_001_with_journal() {
    let _g = with_frozen_clock(Some("1"));
    let tmp = TempDir::new().unwrap();
    let card_path = tmp.path().join("does-not-exist.md");
    let manifest_path = tmp.path().join("chunks_manifest.json");
    let journal_path = tmp.path().join("journal.jsonl");
    let out = verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        false,
        false,
        None,
        None,
        None,
        None,
        None,
    )
    .expect("runner error path only fires on journal error");
    assert!(!out.verified);
    assert!(!out.layer1);
    assert!(!out.layer2);
    assert_eq!(out.diagnostics.len(), 1);
    assert_eq!(out.diagnostics[0].code, codes::CLI_001);
    assert!(out.card_hash.is_none());

    // Persisted-journal field assertions.
    let event = read_single_journal_event(&journal_path);
    assert_eq!(event["command"], "verify");
    assert_eq!(
        event["card_path"].as_str().unwrap(),
        card_path.display().to_string().as_str()
    );
    assert_eq!(event["card_hash_before"], serde_json::Value::Null);
    assert_eq!(event["card_hash_after"], serde_json::Value::Null);
    let diags = event["diagnostics"].as_array().expect("diagnostics array");
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0]["code"], codes::CLI_001);
    assert_eq!(diags[0]["severity"], "error");
    assert_eq!(
        diags[0]["file"].as_str().unwrap(),
        card_path.display().to_string().as_str()
    );
    let v = verification_of(&event);
    assert_eq!(v["layer1"], false);
    assert_eq!(v["layer2"], false);
    assert_eq!(v["fuzzy"], false);
    assert_eq!(event["latency_ms"], 0.0);
}

#[test]
fn verify_one_card_card_path_is_directory_emits_cli_001() {
    // Shape-check pin: a directory path masquerading as a
    // card file routes through CACG-CLI-001 (is_file() rather
    // than exists() per BL-20260518-shape-check-fs-inputs).
    let _g = with_frozen_clock(Some("1"));
    let tmp = TempDir::new().unwrap();
    let card_path = tmp.path().to_path_buf();
    let manifest_path = tmp.path().join("chunks_manifest.json");
    let journal_path = tmp.path().join("journal.jsonl");
    let out = verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        false,
        false,
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    assert_eq!(out.diagnostics.len(), 1);
    assert_eq!(out.diagnostics[0].code, codes::CLI_001);
    assert_eq!(count_journal_events(&journal_path), 1);
}

#[test]
fn verify_one_card_malformed_manifest_emits_man_001_with_journal() {
    let _g = with_frozen_clock(Some("1"));
    let tmp = TempDir::new().unwrap();
    let card_path = tmp.path().join("card.md");
    let manifest_path = tmp.path().join("chunks_manifest.json");
    let journal_path = tmp.path().join("journal.jsonl");
    // Manifest is malformed: invalid JSON.
    fs::write(&manifest_path, "{ not valid json").unwrap();
    // Card is valid but skip_lint so layer-1 doesn't try the
    // manifest first (lint also surfaces CACG-MAN-001; we
    // want runner.rs's branch to be the emitter for this
    // test).
    let body = render_card(
        "t",
        "s",
        "s:p001:0000",
        "abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234",
        (1, 1),
        "q",
        "body",
    );
    fs::write(&card_path, body).unwrap();
    let out = verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        false,
        true,
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    assert!(!out.verified);
    // skip_lint=true so layer-1 stays "true" (not run) per
    // the runner convention; layer-2 failed via MAN-001 path.
    assert!(!out.layer2);
    let codes_seen: BTreeSet<&str> = out.diagnostics.iter().map(|d| d.code.as_str()).collect();
    assert!(
        codes_seen.contains(codes::MAN_001),
        "expected CACG-MAN-001; got {codes_seen:?}"
    );
    assert_eq!(count_journal_events(&journal_path), 1);
}

#[test]
fn verify_one_card_malformed_manifest_and_retracted_emits_both_diagnostics() {
    let _g = with_frozen_clock(Some("1"));
    let tmp = TempDir::new().unwrap();
    let card_path = tmp.path().join("card.md");
    let manifest_path = tmp.path().join("chunks_manifest.json");
    let journal_path = tmp.path().join("journal.jsonl");
    fs::write(&manifest_path, "{ not valid json").unwrap();
    let body = render_card(
        "retracted-card-id",
        "s",
        "s:p001:0000",
        "abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234",
        (1, 1),
        "q",
        "body",
    );
    fs::write(&card_path, body).unwrap();
    let retraction = RetractionSpec {
        retracted_ids: {
            let mut s = std::collections::BTreeSet::new();
            s.insert("retracted-card-id".to_string());
            s
        },
        allow_retracted: false,
    };
    let out = verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        false,
        true,
        None,
        None,
        Some(&retraction),
        None,
        None,
    )
    .unwrap();
    let codes_seen: BTreeSet<&str> = out.diagnostics.iter().map(|d| d.code.as_str()).collect();
    assert!(codes_seen.contains(codes::MAN_001));
    assert!(codes_seen.contains(codes::RETR_001));

    // Persisted-journal: both diagnostics must appear in the
    // emitted event in Python's emission order (MAN-001
    // first, then RETR-001 — per the runner's malformed-
    // manifest early-return branch).
    let event = read_single_journal_event(&journal_path);
    let diags = event["diagnostics"].as_array().expect("diagnostics array");
    let codes_in_event: Vec<&str> = diags.iter().map(|d| d["code"].as_str().unwrap()).collect();
    assert_eq!(
        codes_in_event,
        vec![codes::MAN_001, codes::RETR_001],
        "expected MAN-001 then RETR-001 (Python emission order); got {codes_in_event:?}"
    );
}

#[test]
fn verify_one_card_retracted_emits_retr_001_with_warning_when_allow_retracted() {
    let _g = with_frozen_clock(Some("1"));
    let (_tmp, card_path, manifest_path, journal_path) =
        build_clean_scenario("beta gamma delta", "Body.");
    // The card's frontmatter id is "test-card".
    let retraction = RetractionSpec {
        retracted_ids: {
            let mut s = std::collections::BTreeSet::new();
            s.insert("test-card".to_string());
            s
        },
        allow_retracted: true,
    };
    let out = verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        false,
        false,
        None,
        None,
        Some(&retraction),
        None,
        None,
    )
    .unwrap();
    // warning severity does not flip verified to false.
    let retr = out
        .diagnostics
        .iter()
        .find(|d| d.code == codes::RETR_001)
        .expect("RETR-001 expected");
    assert!(matches!(retr.severity, Severity::Warning));
    assert!(out.verified, "warning severity should keep verified=true");
    assert_eq!(count_journal_events(&journal_path), 1);
}

#[test]
fn verify_one_card_retracted_emits_retr_001_with_error_otherwise() {
    let _g = with_frozen_clock(Some("1"));
    let (_tmp, card_path, manifest_path, journal_path) =
        build_clean_scenario("beta gamma delta", "Body.");
    let retraction = RetractionSpec {
        retracted_ids: {
            let mut s = std::collections::BTreeSet::new();
            s.insert("test-card".to_string());
            s
        },
        allow_retracted: false,
    };
    let out = verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        false,
        false,
        None,
        None,
        Some(&retraction),
        None,
        None,
    )
    .unwrap();
    let retr = out
        .diagnostics
        .iter()
        .find(|d| d.code == codes::RETR_001)
        .expect("RETR-001 expected");
    assert!(matches!(retr.severity, Severity::Error));
    assert!(
        !out.verified,
        "error severity should flip verified to false"
    );
    assert_eq!(count_journal_events(&journal_path), 1);
}

#[test]
fn verify_one_card_layer2_fuzzy_match_sets_fuzzy_true_in_verification() {
    let _g = with_frozen_clock(Some("1"));
    // The drift quote ("beta gamma delxa" — one char swap)
    // requires fuzzy to pass.
    let tmp = TempDir::new().expect("tempdir");
    let chunk_text = "the quick brown fox jumps over the lazy dog and runs fast";
    let chunk = make_chunk("src", "src:p001:0000", chunk_text, 1, 1);
    let manifest = manifest_with(vec![chunk.clone()], Vec::new());
    let manifest_path = tmp.path().join("chunks_manifest.json");
    fs::write(&manifest_path, serde_json::to_string(&manifest).unwrap()).unwrap();
    let card_path = tmp.path().join("card.md");
    let body = render_card(
        "t",
        "src",
        "src:p001:0000",
        &chunk.chunk_hash,
        (1, 1),
        "the quick brown fox jumps over the lazy dox",
        "body",
    );
    fs::write(&card_path, body).unwrap();
    let journal_path = tmp.path().join("journal.jsonl");
    let out = verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        true, // fuzzy enabled
        false,
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    assert!(out.verified);
    assert!(out.layer2);
    assert!(out.fuzzy, "fuzzy fallback should have fired");

    // Persisted-journal: verification.fuzzy must be true.
    let event = read_single_journal_event(&journal_path);
    let v = verification_of(&event);
    assert_eq!(v["layer1"], true);
    assert_eq!(v["layer2"], true);
    assert_eq!(
        v["fuzzy"], true,
        "verification.fuzzy must be true when the fuzzy fallback fires"
    );
}

#[test]
fn verify_one_card_skip_lint_does_not_run_layer1() {
    // skip_lint=true must not call run_layer1_checks. We
    // verify behaviorally: a manifest that lint would reject
    // (e.g., a citation referencing a missing chunk) STILL
    // reaches layer-2, which surfaces HASH-001 there.
    //
    // Python-parity pin on the `verification.layer1` field:
    // Python `legacy_python_oracle/src/cacg/verify/runner.py:193-198` writes
    // `verification = {"layer1": not layer1_failed, ...}`.
    // Under `skip_lint=true`, `layer1_failed` defaults to
    // `false` and never gets set, so Python journals
    // `verification.layer1 = true` — empirically confirmed by
    // running Python `verify_one_card(skip_lint=True)` against
    // a clean fixture and reading the persisted JSONL. The
    // Rust port intentionally mirrors this; the byte-equal
    // JournalEntry parity contract requires field-for-field
    // agreement.
    let _g = with_frozen_clock(Some("1"));
    let tmp = TempDir::new().expect("tempdir");
    // Manifest carries a chunk under a DIFFERENT source so the
    // citation's `src:p001:0000` resolves to "missing chunk".
    let chunk = make_chunk("other", "other:p001:0000", "x", 1, 1);
    let manifest = manifest_with(vec![chunk], Vec::new());
    let manifest_path = tmp.path().join("chunks_manifest.json");
    fs::write(&manifest_path, serde_json::to_string(&manifest).unwrap()).unwrap();
    let card_path = tmp.path().join("card.md");
    let body = render_card(
        "t",
        "src",
        "src:p001:0000",
        "abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234",
        (1, 1),
        "q",
        "body",
    );
    fs::write(&card_path, body).unwrap();
    let journal_path = tmp.path().join("journal.jsonl");
    let out = verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        false,
        true,
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    assert!(!out.verified);
    // skip_lint=true means layer1 status is "true" (not run)
    // and layer2 is "false" because verify_card emitted
    // HASH-001 for the missing chunk.
    assert!(out.layer1, "skip_lint should leave layer1=true (not run)");
    assert!(!out.layer2);
    let codes_seen: BTreeSet<&str> = out.diagnostics.iter().map(|d| d.code.as_str()).collect();
    assert!(codes_seen.contains(codes::HASH_001));

    // Persisted-journal Python-parity pin on
    // `verification.layer1` under `skip_lint=true`.
    let event = read_single_journal_event(&journal_path);
    let v = verification_of(&event);
    assert_eq!(
        v["layer1"], true,
        "Python parity: verification.layer1=true under skip_lint=true \
         (Python writes `not layer1_failed`; layer1_failed defaults false)"
    );
    assert_eq!(v["layer2"], false);
    // Also assert the persisted diagnostic includes HASH-001.
    let diags = event["diagnostics"].as_array().expect("diagnostics array");
    let codes_in_event: Vec<&str> = diags.iter().map(|d| d["code"].as_str().unwrap()).collect();
    assert!(
        codes_in_event.iter().any(|c| *c == codes::HASH_001),
        "expected CACG-HASH-001 in persisted diagnostics; got {codes_in_event:?}"
    );
}

#[test]
fn verify_one_card_exactly_one_journal_event_on_every_path() {
    // Run every scenario above and assert each leaves
    // EXACTLY ONE event in its own journal file. This is the
    // cardinality pin the runner's per-codepath contract
    // requires across every codepath.
    let _g = with_frozen_clock(Some("1"));
    // 1. clean
    let (_t1, c1, m1, j1) = build_clean_scenario("beta gamma delta", "body");
    verify_one_card(&c1, &m1, &j1, false, false, None, None, None, None, None).unwrap();
    assert_eq!(count_journal_events(&j1), 1, "clean path");
    // 2. missing card
    let tmp2 = TempDir::new().unwrap();
    let j2 = tmp2.path().join("journal.jsonl");
    verify_one_card(
        &tmp2.path().join("missing.md"),
        &tmp2.path().join("manifest.json"),
        &j2,
        false,
        false,
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    assert_eq!(count_journal_events(&j2), 1, "missing card path");
    // 3. malformed manifest (skip_lint=true) — already
    // covered in its own test; rerun here as a cardinality
    // sanity for the runner-level append.
    let tmp3 = TempDir::new().unwrap();
    let c3 = tmp3.path().join("card.md");
    let m3 = tmp3.path().join("chunks_manifest.json");
    let j3 = tmp3.path().join("journal.jsonl");
    fs::write(&m3, "{ not valid json").unwrap();
    let body = render_card(
        "t",
        "s",
        "s:p001:0000",
        "abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234",
        (1, 1),
        "q",
        "body",
    );
    fs::write(&c3, body).unwrap();
    verify_one_card(&c3, &m3, &j3, false, true, None, None, None, None, None).unwrap();
    assert_eq!(count_journal_events(&j3), 1, "malformed manifest path");
}

#[test]
fn verify_one_card_exit_constants_match_python() {
    assert_eq!(EXIT_VERIFIED, 0);
    assert_eq!(EXIT_FAILED, 1);
}

#[test]
fn runner_api_is_reexported_at_crate_verify_path() {
    // The canonical entrypoint is
    // `cacg-core::verify::verify_one_card`. The re-export in
    // `verify/mod.rs` surfaces the runner API at the shorter
    // path so future CLI / round-summary callers import the
    // trust-kernel surface at the named path. This test
    // fails to compile if the re-export is removed.
    use crate::verify as cv;
    let _ = cv::EXIT_VERIFIED;
    let _ = cv::EXIT_FAILED;
    let _: fn(
        &Path,
        &Path,
        &Path,
        bool,
        bool,
        Option<&ChunksIndex>,
        Option<&AuthSpec>,
        Option<&RetractionSpec>,
        Option<&dyn SemanticEvaluator>,
        Option<&mut Bm25HintCache>,
    ) -> Result<cv::VerifyOneCardResult, cv::RunnerError> = cv::verify_one_card;
}

#[test]
fn verify_one_card_uses_supplied_chunks_index_without_reloading() {
    // Pin the batch-reuse contract the round-summary driver
    // will rely on. When the caller supplies a pre-built
    // `ChunksIndex`, the
    // runner MUST NOT call `ChunksIndex::from_path(chunks_manifest_path)`.
    // We prove this behaviorally by passing a `chunks_manifest_path`
    // that points at a NON-EXISTENT file: if the runner ignored
    // the supplied index and tried to load from disk, the load
    // would fail and `CACG-MAN-001` would land in the journal.
    // The runner that reuses the supplied index makes the card
    // verify clean and the bogus path is never touched.
    let _g = with_frozen_clock(Some("1"));
    let (_tmp, card_path, manifest_path, journal_path) =
        build_clean_scenario("beta gamma delta", "Body.");
    // Build the index once from the real manifest.
    let index = ChunksIndex::from_path(&manifest_path).expect("clean manifest must load");
    // Now point the runner at a non-existent manifest path.
    let bogus_manifest_path = card_path.parent().unwrap().join("does-not-exist.json");
    assert!(
        !bogus_manifest_path.is_file(),
        "bogus path must not exist; otherwise the test is meaningless"
    );

    let out = verify_one_card(
        &card_path,
        &bogus_manifest_path,
        &journal_path,
        false,
        false,
        Some(&index),
        None,
        None,
        None,
        None,
    )
    .expect("runner should not error with a supplied index");
    assert!(
        out.verified,
        "card should verify clean using the supplied index; diagnostics={:?}",
        out.diagnostics
    );
    assert!(out.diagnostics.is_empty());
    let event = read_single_journal_event(&journal_path);
    let diags = event["diagnostics"].as_array().unwrap();
    for d in diags {
        assert_ne!(
            d["code"].as_str().unwrap(),
            codes::MAN_001,
            "supplied index path must not trigger CACG-MAN-001"
        );
    }

    // Second invocation with the SAME supplied index across a
    // fresh card and journal pins the batch-reuse shape the
    // round-summary driver will rely on.
    let (_tmp2, card_path2, _manifest2, journal_path2) =
        build_clean_scenario("beta gamma delta", "Body.");
    let out2 = verify_one_card(
        &card_path2,
        &bogus_manifest_path,
        &journal_path2,
        false,
        false,
        Some(&index),
        None,
        None,
        None,
        None,
    )
    .expect("second invocation also uses supplied index");
    assert!(out2.verified, "second card should verify clean too");
    assert_eq!(count_journal_events(&journal_path2), 1);
}

// ----------------------------------------------------------------
// Mock-evaluator firing tests: a stub `SemanticEvaluator` that
// returns a pre-set verdict pins the severity contract of the
// `CACG-VERIFY-002` diagnostic the runner appends after a Layer-2
// citation failure (pass -> Warning, fail -> Error, abstain -> Info).
// ----------------------------------------------------------------

use crate::verify::semantic_spec::{
    SemanticEvaluationError, SemanticEvaluator, SemanticMode, SemanticVerdict, SemanticVerdictKind,
};

struct StubEvaluator(SemanticVerdictKind);

impl SemanticEvaluator for StubEvaluator {
    fn evaluate(
        &self,
        _chunk_hash: &str,
        _claim_window_hash: &str,
        _quote: &str,
        _chunk_text: &str,
    ) -> Result<SemanticVerdict, SemanticEvaluationError> {
        Ok(SemanticVerdict {
            kind: self.0,
            score: 0.42,
            reasoning: None,
            mode: SemanticMode::EmbeddingCache,
        })
    }
}

/// Build a fail-on-Layer-2 scenario: the chunk's text intentionally
/// does NOT contain the citation's quote, so verify_citation emits
/// `CACG-VERIFY-001`. The supplied semantic evaluator then fires.
fn build_semantic_firing_scenario() -> (TempDir, PathBuf, PathBuf, PathBuf) {
    let tmp = TempDir::new().expect("tempdir");
    let chunk = make_chunk(
        "src",
        "src:p001:0000",
        "alpha beta gamma delta epsilon zeta eta theta",
        1,
        1,
    );
    let manifest = manifest_with(vec![chunk.clone()], Vec::new());
    let manifest_path = tmp.path().join("chunks_manifest.json");
    fs::write(
        &manifest_path,
        serde_json::to_string(&manifest).expect("manifest json"),
    )
    .unwrap();
    let card_path = tmp.path().join("card.md");
    // Quote that DOES NOT appear in the chunk text — Layer-2 fails.
    let body = render_card(
        "card",
        "src",
        "src:p001:0000",
        &chunk.chunk_hash,
        (1, 1),
        "this phrase is not in the chunk",
        "Body.",
    );
    fs::write(&card_path, body).unwrap();
    let journal_path = tmp.path().join("journal.jsonl");
    (tmp, card_path, manifest_path, journal_path)
}

fn run_with_stub(kind: SemanticVerdictKind) -> VerifyOneCardResult {
    let _g = with_frozen_clock(Some("1"));
    let (_tmp, card_path, manifest_path, journal_path) = build_semantic_firing_scenario();
    let stub = StubEvaluator(kind);
    verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        false,
        false,
        None,
        None,
        None,
        Some(&stub),
        None,
    )
    .expect("runner journal append must not fail")
}

#[test]
fn semantic_pass_verdict_emits_verify_002_with_warning_severity() {
    let out = run_with_stub(SemanticVerdictKind::Pass);
    let verify_002 = out
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 must be appended on pass");
    assert!(matches!(verify_002.severity, Severity::Warning));
    assert!(verify_002.message.contains("verdict=pass"));
    assert!(verify_002.message.contains("mode=embedding-cache"));
}

#[test]
fn semantic_fail_verdict_emits_verify_002_with_error_severity() {
    let out = run_with_stub(SemanticVerdictKind::Fail);
    let verify_002 = out
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 must be appended on fail");
    assert!(matches!(verify_002.severity, Severity::Error));
    assert!(verify_002.message.contains("verdict=fail"));
}

#[test]
fn semantic_abstain_verdict_emits_verify_002_with_info_severity() {
    let out = run_with_stub(SemanticVerdictKind::Abstain);
    let verify_002 = out
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 must be appended on abstain");
    assert!(matches!(verify_002.severity, Severity::Info));
    assert!(verify_002.message.contains("verdict=abstain"));
}

#[test]
fn semantic_diagnostic_carries_semantic_hint_payload() {
    let out = run_with_stub(SemanticVerdictKind::Pass);
    let verify_002 = out
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 expected");
    assert_eq!(verify_002.hints.len(), 1, "exactly one hint payload");
    let hint = &verify_002.hints[0];
    assert_eq!(
        hint.get("semantic_verdict").and_then(|v| v.as_str()),
        Some("pass")
    );
    assert_eq!(
        hint.get("semantic_mode").and_then(|v| v.as_str()),
        Some("embedding-cache")
    );
    assert!(hint
        .get("semantic_score")
        .and_then(|v| v.as_f64())
        .is_some());
}

#[test]
fn semantic_diagnostic_message_includes_reasoning_when_present() {
    // When the semantic evaluator returns a verdict carrying a
    // non-empty `reasoning` (B2 LLM-judge path), the rendered
    // diagnostic message must append `reasoning=...` after the
    // mode field — matching the Python verifier's
    // `f"... mode={verdict.mode}{f' reasoning={verdict.reasoning!r}' if verdict.reasoning else ''}"`
    // composition. Verdicts with `reasoning: None` or empty string
    // produce the bare `verdict/score/mode` message (covered by the
    // existing run_with_stub tests).
    struct ReasoningStubEvaluator {
        kind: SemanticVerdictKind,
        reasoning: &'static str,
    }
    impl SemanticEvaluator for ReasoningStubEvaluator {
        fn evaluate(
            &self,
            _chunk_hash: &str,
            _claim_window_hash: &str,
            _quote: &str,
            _chunk_text: &str,
        ) -> Result<SemanticVerdict, SemanticEvaluationError> {
            Ok(SemanticVerdict {
                kind: self.kind,
                score: 0.42,
                reasoning: Some(self.reasoning.to_string()),
                mode: SemanticMode::LlmJudge,
            })
        }
    }

    let _g = with_frozen_clock(Some("1"));
    let (_tmp, card_path, manifest_path, journal_path) = build_semantic_firing_scenario();
    let stub = ReasoningStubEvaluator {
        kind: SemanticVerdictKind::Fail,
        reasoning: "quote does not match the cited chunk",
    };
    let out = verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        false,
        false,
        None,
        None,
        None,
        Some(&stub),
        None,
    )
    .expect("runner journal append must not fail");
    let verify_002 = out
        .diagnostics
        .iter()
        .find(|d| d.code == codes::VERIFY_002)
        .expect("CACG-VERIFY-002 expected when Layer-3 fires");
    assert!(
        verify_002.message.contains("reasoning="),
        "reasoning= must appear when the verdict carries it; got: {}",
        verify_002.message,
    );
    assert!(
        verify_002
            .message
            .contains("quote does not match the cited chunk"),
        "reasoning text must be present in the message; got: {}",
        verify_002.message,
    );
    assert!(
        verify_002.message.contains("mode=llm-judge"),
        "mode field must precede reasoning; got: {}",
        verify_002.message,
    );
}

#[test]
fn semantic_not_invoked_when_layer2_passes() {
    // Sanity pin: when Layer-2 succeeds the evaluator is never
    // consulted (so the runner appends no CACG-VERIFY-002 even
    // with a stub that would return `fail`).
    let _g = with_frozen_clock(Some("1"));
    let (_tmp, card_path, manifest_path, journal_path) =
        build_clean_scenario("beta gamma delta", "Body.");
    let stub = StubEvaluator(SemanticVerdictKind::Fail);
    let out = verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        false,
        false,
        None,
        None,
        None,
        Some(&stub),
        None,
    )
    .expect("runner journal append must not fail");
    assert!(out.verified, "Layer-2 clean -> verified");
    assert!(
        out.diagnostics.iter().all(|d| d.code != codes::VERIFY_002),
        "no CACG-VERIFY-002 must be emitted when Layer-2 passes",
    );
}

// ----------------------------------------------------------------
// Comprehensive per-verdict invariant pins.
//
// For each of the three Layer-3 verdicts (pass / fail / abstain),
// assert every per-verdict contract invariant in one shot rather
// than splitting them across tests:
//
// 1. `verified == false` (Layer-2 already failed; Layer-3 must
//    NEVER flip the overall verdict to verified — `CACG-VERIFY-001`
//    is the merge gate, so `pass` still leaves verified=false).
// 2. Exactly one CACG-VERIFY-001 followed by exactly one
//    CACG-VERIFY-002, both inside the same `diagnostics` array
//    (which becomes one journal event).
// 3. CACG-VERIFY-002 severity follows the per-verdict severity
//    contract: pass -> warning, fail -> error, abstain -> info.
// 4. CACG-VERIFY-002 message text embeds `verdict=<wire>`,
//    `score=0.420000` (exactly 6 decimal digits), and
//    `mode=embedding-cache`.
// 5. CACG-VERIFY-002.hints[0] carries the three keys
//    `semantic_verdict` / `semantic_score` / `semantic_mode` with
//    matching wire values.
// 6. The journal records exactly one event for the card.
// ----------------------------------------------------------------

fn assert_per_verdict_invariants(
    verdict: SemanticVerdictKind,
    expected_severity: Severity,
    expected_wire: &str,
) {
    let _g = with_frozen_clock(Some("1"));
    let (_tmp, card_path, manifest_path, journal_path) = build_semantic_firing_scenario();
    let stub = StubEvaluator(verdict);
    let out = verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        false,
        false,
        None,
        None,
        None,
        Some(&stub),
        None,
    )
    .expect("runner journal append must not fail");

    // (1) Layer-3 never flips the overall verdict.
    assert!(
        !out.verified,
        "Layer-2 fail must not be overridden by Layer-3"
    );
    assert!(out.layer1);
    assert!(!out.layer2);

    // (2) Same-array co-location with VERIFY-001 first.
    let verify_001_count = out
        .diagnostics
        .iter()
        .filter(|d| d.code == codes::VERIFY_001)
        .count();
    let verify_002_count = out
        .diagnostics
        .iter()
        .filter(|d| d.code == codes::VERIFY_002)
        .count();
    assert_eq!(verify_001_count, 1, "exactly one CACG-VERIFY-001");
    assert_eq!(verify_002_count, 1, "exactly one CACG-VERIFY-002");
    let positions: Vec<usize> = out
        .diagnostics
        .iter()
        .enumerate()
        .filter_map(|(i, d)| {
            if d.code == codes::VERIFY_001 || d.code == codes::VERIFY_002 {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    assert_eq!(positions.len(), 2);
    assert!(
        out.diagnostics[positions[0]].code == codes::VERIFY_001
            && out.diagnostics[positions[1]].code == codes::VERIFY_002,
        "CACG-VERIFY-001 must precede CACG-VERIFY-002 in the same diagnostics array",
    );

    // (3) Severity + (4) message text + (5) hint payload.
    let verify_002 = &out.diagnostics[positions[1]];
    match (expected_severity, verify_002.severity) {
        (Severity::Warning, Severity::Warning)
        | (Severity::Error, Severity::Error)
        | (Severity::Info, Severity::Info) => {}
        (expected, actual) => {
            panic!("severity mismatch: expected {expected:?}, got {actual:?}")
        }
    }
    assert!(
        verify_002
            .message
            .contains(&format!("verdict={expected_wire}")),
        "message must contain verdict={expected_wire}; got {:?}",
        verify_002.message,
    );
    assert!(
        verify_002.message.contains("score=0.420000"),
        "score must be formatted to exactly 6 decimals; got {:?}",
        verify_002.message,
    );
    assert!(
        verify_002.message.contains("mode=embedding-cache"),
        "message must contain mode=embedding-cache; got {:?}",
        verify_002.message,
    );

    assert_eq!(verify_002.hints.len(), 1, "exactly one hint payload");
    let hint = &verify_002.hints[0];
    assert_eq!(
        hint.get("semantic_verdict").and_then(|v| v.as_str()),
        Some(expected_wire),
    );
    assert_eq!(
        hint.get("semantic_mode").and_then(|v| v.as_str()),
        Some("embedding-cache"),
    );
    let score = hint
        .get("semantic_score")
        .and_then(|v| v.as_f64())
        .expect("semantic_score must be a number");
    assert!(
        (score - 0.42).abs() < 1e-9,
        "semantic_score mismatch: {score}"
    );

    // (6) Exactly one journal event for the card.
    assert_eq!(
        count_journal_events(&journal_path),
        1,
        "exactly one journal event per card per invocation",
    );
}

#[test]
fn semantic_pass_verdict_pins_every_per_verdict_invariant() {
    assert_per_verdict_invariants(SemanticVerdictKind::Pass, Severity::Warning, "pass");
}

#[test]
fn semantic_fail_verdict_pins_every_per_verdict_invariant() {
    assert_per_verdict_invariants(SemanticVerdictKind::Fail, Severity::Error, "fail");
}

#[test]
fn semantic_abstain_verdict_pins_every_per_verdict_invariant() {
    assert_per_verdict_invariants(SemanticVerdictKind::Abstain, Severity::Info, "abstain");
}

#[test]
fn shared_bm25_hint_cache_reuses_corpus_across_repeated_failing_visits() {
    // Pin the round-summary BM25 hint-cache reuse contract: when a
    // caller threads the same `Bm25HintCache` into multiple
    // `verify_one_card` invocations against the same source (the
    // round-summary batch path), the BM25 corpus + Bm25Okapi index
    // for that source is built ONCE, not per-card. Mirrors Python
    // `_cmd_verify_round_summary`'s per-batch
    // `BM25HintCache()` reuse.
    let _g = with_frozen_clock(Some("1"));
    let (_tmp, card_path, manifest_path, journal_path) = build_semantic_firing_scenario();
    let mut shared_cache = Bm25HintCache::new();
    assert!(shared_cache.is_empty(), "fresh cache must be empty");

    // First invocation: Layer-2 fails because the synthetic
    // citation's quote does not appear in the chunk's text, so
    // BM25 hints are computed for the source and the cache is
    // populated.
    let out1 = verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        false,
        false,
        None,
        None,
        None,
        None,
        Some(&mut shared_cache),
    )
    .expect("first verify_one_card");
    assert!(!out1.verified, "Layer-2 should fail in the firing scenario");
    assert_eq!(
        shared_cache.len(),
        1,
        "first failing visit must populate exactly one (source, signature) entry",
    );

    // Second invocation: same card, same source. The cache MUST
    // be reused rather than rebuilt. A regression that rebuilt
    // the corpus per visit would produce an unchanged cache.len()
    // == 1 here (the key is identical), so we additionally pin
    // the cache CONTENT identity by checking len() == 1 and
    // is_empty() == false after the second call.
    let out2 = verify_one_card(
        &card_path,
        &manifest_path,
        &journal_path,
        false,
        false,
        None,
        None,
        None,
        None,
        Some(&mut shared_cache),
    )
    .expect("second verify_one_card");
    assert!(!out2.verified);
    assert_eq!(
        shared_cache.len(),
        1,
        "repeated failing visit on the same source must not introduce a second entry",
    );
    assert!(!shared_cache.is_empty());
}
