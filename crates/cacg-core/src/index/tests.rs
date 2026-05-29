#![allow(clippy::unwrap_used)]
use super::delta::replace_card_hash_line;
use super::rollback::{
    rollback_after_partial_index_commit, rollback_committed_cards, CommittedCard,
};
use super::*;
use crate::diagnostic::codes;
use crate::history::HistoryEvent;
use tempfile::TempDir;

/// A valid `CardManifestEntry` for the given id (64-hex `card_hash`,
/// non-empty fields, empty `source_ids`).
fn entry(id: &str) -> CardManifestEntry {
    CardManifestEntry {
        schema_version: SchemaVersion::V0,
        path: format!("cards/{id}.md"),
        id: id.to_string(),
        title: format!("Title {id}"),
        reading_id: "reading_01".to_string(),
        summary: format!("Summary for {id}"),
        card_hash: "0".repeat(64),
        citation_count: 0,
        source_ids: Vec::new(),
    }
}

fn manifest(
    cards: Vec<CardManifestEntry>,
    retracted: &[&str],
    dependency: &[&str],
) -> CardsManifest {
    CardsManifest {
        schema_version: SchemaVersion::V0,
        cards,
        retracted_cards: retracted.iter().map(|s| (*s).to_string()).collect(),
        dependency_retracted_cards: dependency.iter().map(|s| (*s).to_string()).collect(),
    }
}

#[test]
fn cards_manifest_validate_accepts_a_well_formed_manifest() {
    let m = manifest(vec![entry("a"), entry("b")], &["x"], &["a"]);
    assert!(m.validate_structurally().is_ok());
}

#[test]
fn cards_manifest_validate_rejects_duplicate_active_card_ids() {
    let m = manifest(vec![entry("a"), entry("a")], &[], &[]);
    let d = m
        .validate_structurally()
        .expect_err("duplicate id must reject");
    assert_eq!(d.code, codes::MAN_001);
    assert!(d.message.contains("duplicate card_id"), "got {}", d.message);
}

#[test]
fn cards_manifest_validate_rejects_active_retracted_overlap() {
    let m = manifest(vec![entry("a")], &["a"], &[]);
    let d = m
        .validate_structurally()
        .expect_err("active∩retracted must reject");
    assert_eq!(d.code, codes::MAN_001);
    assert!(
        d.message.contains("`cards` and `retracted_cards`"),
        "got {}",
        d.message
    );
}

#[test]
fn cards_manifest_validate_rejects_direct_dependency_overlap() {
    let m = manifest(vec![entry("a")], &["x"], &["a", "x"]);
    let d = m
        .validate_structurally()
        .expect_err("retracted∩dependency must reject");
    assert_eq!(d.code, codes::MAN_001);
}

#[test]
fn cards_manifest_validate_rejects_dependency_not_in_active_cards() {
    let m = manifest(vec![entry("a")], &[], &["ghost"]);
    let d = m
        .validate_structurally()
        .expect_err("dependency not ⊆ active must reject");
    assert_eq!(d.code, codes::MAN_001);
    assert!(d.message.contains("not in `cards`"), "got {}", d.message);
}

#[test]
fn cards_manifest_validate_rejects_unsorted_retraction_list() {
    let m = manifest(vec![entry("a")], &["z", "y"], &[]);
    let d = m
        .validate_structurally()
        .expect_err("unsorted retraction list must reject");
    assert_eq!(d.code, codes::MAN_001);
}

#[test]
fn cards_manifest_validate_rejects_bad_card_hash() {
    let mut bad = entry("a");
    bad.card_hash = "not-64-hex".to_string();
    let d = manifest(vec![bad], &[], &[])
        .validate_structurally()
        .expect_err("non-64-hex card_hash must reject");
    assert_eq!(d.code, codes::MAN_001);
    assert!(d.message.contains("card_hash"), "got {}", d.message);
}

#[test]
fn cards_manifest_validate_rejects_unsorted_entry_source_ids() {
    let mut e = entry("a");
    e.source_ids = vec!["s2".to_string(), "s1".to_string()];
    let d = manifest(vec![e], &[], &[])
        .validate_structurally()
        .expect_err("unsorted source_ids must reject");
    assert_eq!(d.code, codes::MAN_001);
    assert!(d.message.contains("source_ids"), "got {}", d.message);
}

#[test]
fn cards_manifest_deny_unknown_fields_rejects_extras() {
    let json = r#"{"schema_version":"cacg.v0","cards":[],"retracted_cards":[],"dependency_retracted_cards":[],"surprise":1}"#;
    assert!(
        serde_json::from_str::<CardsManifest>(json).is_err(),
        "an unknown top-level field must be rejected"
    );
}

#[test]
fn cards_manifest_retraction_lists_default_to_empty() {
    // A pre-Phase-4 manifest that omits the retraction lists loads
    // with `[]` (Python `Field(default_factory=list)` parity).
    let json = r#"{"schema_version":"cacg.v0","cards":[]}"#;
    let m: CardsManifest =
        serde_json::from_str(json).expect("omitted retraction lists must default to []");
    assert!(m.retracted_cards.is_empty() && m.dependency_retracted_cards.is_empty());
    assert!(m.validate_structurally().is_ok());
}

#[test]
fn cards_manifest_validate_rejects_duplicate_retraction_entries() {
    // A `retracted_cards` list with a repeated id violates the
    // uniqueness invariant `validate_retracted_list` enforces.
    let m = manifest(vec![entry("a")], &["dup", "dup"], &[]);
    let d = m
        .validate_structurally()
        .expect_err("duplicate retracted id must reject");
    assert_eq!(d.code, codes::MAN_001);
    assert!(
        d.message.contains("duplicate retracted id"),
        "got {}",
        d.message
    );
}

#[test]
fn cards_manifest_validate_rejects_empty_retraction_entry() {
    // An empty-string id inside `retracted_cards` is rejected.
    let m = manifest(vec![entry("a")], &[""], &[]);
    let d = m
        .validate_structurally()
        .expect_err("empty retracted id must reject");
    assert_eq!(d.code, codes::MAN_001);
    assert!(
        d.message.contains("must be non-empty strings"),
        "got {}",
        d.message
    );
}

#[test]
fn cards_manifest_validate_rejects_duplicate_entry_source_ids() {
    // `CardManifestEntry.source_ids` must be unique; a repeated
    // source_id is a `CACG-MAN-001` violation.
    let mut e = entry("a");
    e.source_ids = vec!["s1".to_string(), "s1".to_string()];
    let d = manifest(vec![e], &[], &[])
        .validate_structurally()
        .expect_err("duplicate source_ids must reject");
    assert_eq!(d.code, codes::MAN_001);
    assert!(d.message.contains("must be unique"), "got {}", d.message);
}

#[test]
fn cards_manifest_validate_rejects_empty_entry_source_id() {
    // An empty-string entry inside `CardManifestEntry.source_ids`
    // is rejected.
    let mut e = entry("a");
    e.source_ids = vec![String::new()];
    let d = manifest(vec![e], &[], &[])
        .validate_structurally()
        .expect_err("empty source_id must reject");
    assert_eq!(d.code, codes::MAN_001);
    assert!(d.message.contains("non-empty strings"), "got {}", d.message);
}

#[test]
fn cards_manifest_validate_rejects_empty_required_entry_field() {
    // Every `CardManifestEntry` string field must be non-empty;
    // an empty `title` is a `CACG-MAN-001` violation.
    let mut e = entry("a");
    e.title = String::new();
    let d = manifest(vec![e], &[], &[])
        .validate_structurally()
        .expect_err("empty title must reject");
    assert_eq!(d.code, codes::MAN_001);
    assert!(
        d.message.contains("title: must be a non-empty string"),
        "got {}",
        d.message
    );
}

#[test]
fn card_manifest_entry_deny_unknown_fields_rejects_extras() {
    // `#[serde(deny_unknown_fields)]` must propagate into the
    // nested `CardManifestEntry` (Python `_StrictModel` parity): an
    // unknown field on an entry object is rejected at parse time.
    let base = r#"{"schema_version":"cacg.v0","path":"cards/a.md","id":"a","title":"T","reading_id":"reading_01","summary":"S","card_hash":"0000000000000000000000000000000000000000000000000000000000000000","citation_count":0,"source_ids":[]}"#;
    serde_json::from_str::<CardManifestEntry>(base)
        .expect("the base entry without extras must deserialize");
    let with_extra = r#"{"schema_version":"cacg.v0","path":"cards/a.md","id":"a","title":"T","reading_id":"reading_01","summary":"S","card_hash":"0000000000000000000000000000000000000000000000000000000000000000","citation_count":0,"source_ids":[],"surprise":1}"#;
    assert!(
        serde_json::from_str::<CardManifestEntry>(with_extra).is_err(),
        "an unknown field on a CardManifestEntry must be rejected"
    );
}

#[test]
fn rejects_non_directory_cards_dir() {
    let tmp = TempDir::new().unwrap();
    let not_a_dir = tmp.path().join("not_a_dir");
    std::fs::write(&not_a_dir, b"file, not dir").unwrap();
    let ctx = DeterminismContext::frozen();
    let r = build_index(&not_a_dir, &tmp.path().join("out"), &ctx);
    assert!(matches!(r, Err(IndexError::NonDirCardsDir(_))), "got {r:?}");
}

#[test]
fn empty_cards_dir_produces_empty_manifests() {
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards");
    std::fs::create_dir_all(&cards).unwrap();
    let out = tmp.path().join("out");
    let ctx = DeterminismContext::frozen();
    build_index(&cards, &out, &ctx).expect("build_index ok on empty dir");
    let manifest = std::fs::read_to_string(out.join("cards_manifest.json")).unwrap();
    let summaries = std::fs::read_to_string(out.join("summaries.json")).unwrap();
    // Manifests must include the schema_version key and an empty
    // cards/summaries array.
    assert!(manifest.contains("\"cards\":[]"));
    assert!(manifest.contains("\"schema_version\":\"cacg.v0\""));
    assert!(summaries.contains("\"summaries\":[]"));
}

#[test]
fn parity_corpus_smoke_under_frozen_context() {
    // Use the committed parity corpus to exercise the full path.
    let mut corpus_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    corpus_root.pop();
    corpus_root.pop();
    let cards_src = corpus_root.join("tests/parity_corpus/cards/reading_01");
    if !cards_src.is_dir() {
        eprintln!("skipping parity-corpus smoke: cards dir not present at {cards_src:?}");
        return;
    }
    // Copy the corpus into a tempdir so the test owns its own
    // sibling history files. Skip committed `.history.jsonl` from
    // the source so the test starts from a clean slate.
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards/reading_01");
    std::fs::create_dir_all(&cards).unwrap();
    for entry in std::fs::read_dir(&cards_src).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name();
        if entry.file_type().unwrap().is_file()
            && !name.to_string_lossy().ends_with(".history.jsonl")
        {
            let src = entry.path();
            let dst = cards.join(&name);
            std::fs::copy(&src, &dst).unwrap();
        }
    }
    let out = tmp.path().join("out");
    let ctx = DeterminismContext::frozen();
    build_index(&cards, &out, &ctx).expect("build_index ok on parity corpus");

    assert!(out.join("cards_manifest.json").is_file());
    assert!(out.join("summaries.json").is_file());
    assert!(out.join("INDEX.md").is_file());
    // Per-card history sidecars must NOT exist for this corpus
    // (frontmatter card_hash matches the recomputed hash, so the
    // history append is correctly skipped — mirrors Python).
    let any_history = std::fs::read_dir(&cards)
        .unwrap()
        .filter_map(Result::ok)
        .any(|e| {
            e.path()
                .file_name()
                .map(|n| n.to_string_lossy().ends_with(".history.jsonl"))
                .unwrap_or(false)
        });
    assert!(
        !any_history,
        "no .history.jsonl should be emitted for the parity corpus"
    );
}

#[test]
fn two_runs_under_frozen_context_produce_byte_identical_manifests() {
    let mut corpus_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    corpus_root.pop();
    corpus_root.pop();
    let cards_src = corpus_root.join("tests/parity_corpus/cards/reading_01");
    if !cards_src.is_dir() {
        return;
    }

    fn copy_corpus(src: &Path, dst: &Path) {
        std::fs::create_dir_all(dst).unwrap();
        for entry in std::fs::read_dir(src).unwrap() {
            let entry = entry.unwrap();
            let name = entry.file_name();
            if entry.file_type().unwrap().is_file()
                && !name.to_string_lossy().ends_with(".history.jsonl")
            {
                let s = entry.path();
                let d = dst.join(&name);
                std::fs::copy(&s, &d).unwrap();
            }
        }
    }

    // ONE shared cards_dir (matches the parity harness + the
    // R29 CLI test's invocation pattern). Two separate out_dirs.
    let shared_tmp = TempDir::new().unwrap();
    let cards = shared_tmp.path().join("cards/reading_01");
    copy_corpus(&cards_src, &cards);

    let tmp_a = TempDir::new().unwrap();
    let out_a = tmp_a.path().join("out");
    let ctx_a = DeterminismContext::frozen();
    build_index(&cards, &out_a, &ctx_a).expect("build_index A");

    let tmp_b = TempDir::new().unwrap();
    let out_b = tmp_b.path().join("out");
    let ctx_b = DeterminismContext::frozen();
    build_index(&cards, &out_b, &ctx_b).expect("build_index B");

    // Byte-compare the 3 out_dir artifacts.
    for name in ["cards_manifest.json", "summaries.json", "INDEX.md"] {
        let a = std::fs::read(out_a.join(name)).unwrap();
        let b = std::fs::read(out_b.join(name)).unwrap();
        assert_eq!(
            a, b,
            "{name} must be byte-identical between two frozen runs"
        );
    }
    // The committed corpus's frontmatter card_hash values are
    // correct, so no .history.jsonl files should be emitted by
    // either run (mirrors Python).
    let any_history = std::fs::read_dir(&cards)
        .unwrap()
        .filter_map(Result::ok)
        .any(|e| {
            e.path()
                .file_name()
                .map(|n| n.to_string_lossy().ends_with(".history.jsonl"))
                .unwrap_or(false)
        });
    assert!(
        !any_history,
        "no .history.jsonl should be emitted for the parity corpus"
    );
}

fn copy_clean_card_with_stale_hash(dst_dir: &Path) -> PathBuf {
    // Use the committed clean-hash card as the seed; only flip
    // the stored `card_hash` to 64 zeros so the recomputed hash
    // mismatches and forces the rewrite + history-emit branch.
    let mut src = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    src.pop();
    src.pop();
    src.push("tests/parity_corpus/cards/reading_01/01-content-addressable-identity.md");
    let original = std::fs::read_to_string(&src).expect("read seed clean card");
    let stale = original.replace(
        "card_hash: \"714988273af6b3805ddf8a7c01226a17528a2b8090bed27d23e2fadbc1fd367a\"",
        "card_hash: \"0000000000000000000000000000000000000000000000000000000000000000\"",
    );
    std::fs::create_dir_all(dst_dir).unwrap();
    let dst = dst_dir.join("01-content-addressable-identity.md");
    std::fs::write(&dst, stale).unwrap();
    dst
}

#[test]
fn first_event_history_byte_matches_python_oracle_on_stale_hash_card() {
    // Seed: copy the clean-hash card with `card_hash` flipped to 64
    // zeros. Run `build_index` once. The Rust-emitted
    // `.history.jsonl` must byte-match the Python-emitted oracle
    // committed under tests/parity_corpus/out_python/stale_hash/.
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards");
    copy_clean_card_with_stale_hash(&cards);
    let out = tmp.path().join("out");
    let ctx = DeterminismContext::frozen();
    crate::history::reset_history_cache();
    build_index(&cards, &out, &ctx).expect("build_index ok on stale-hash card");

    let history_path = cards.join("01-content-addressable-identity.history.jsonl");
    assert!(history_path.is_file(), "history sidecar must be emitted");
    let rust_bytes = std::fs::read(&history_path).expect("read rust history");

    let mut oracle_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    oracle_path.pop();
    oracle_path.pop();
    oracle_path.push(
        "tests/parity_corpus/out_python/stale_hash/reading_01/01-content-addressable-identity.history.jsonl",
    );
    let oracle_bytes = std::fs::read(&oracle_path).expect("read python oracle");
    assert_eq!(
        rust_bytes, oracle_bytes,
        "first-event .history.jsonl bytes must match Python oracle"
    );
}

#[test]
fn prev_card_hash_sourced_from_staged_prev_hash_not_from_last_event() {
    // The stored frontmatter hash being replaced is the 64-zero
    // stale value. The emitted event's `prev_card_hash` must
    // equal that stored value, NOT the recomputed canonical hash
    // (which is what a buggy implementation might use if it read
    // `prev_card_hash` from the manifest entry).
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards");
    copy_clean_card_with_stale_hash(&cards);
    let out = tmp.path().join("out");
    let ctx = DeterminismContext::frozen();
    crate::history::reset_history_cache();
    build_index(&cards, &out, &ctx).expect("build_index ok");

    let history_path = cards.join("01-content-addressable-identity.history.jsonl");
    let line = std::fs::read_to_string(&history_path).unwrap();
    let event: HistoryEvent = serde_json::from_str(line.trim()).unwrap();
    assert_eq!(
        event.prev_card_hash.as_deref(),
        Some("0000000000000000000000000000000000000000000000000000000000000000"),
        "prev_card_hash must come from the stale stored hash"
    );
    assert_eq!(
        event.new_card_hash,
        "714988273af6b3805ddf8a7c01226a17528a2b8090bed27d23e2fadbc1fd367a"
    );
    assert!(!event.is_retracted, "is_retracted must be false");
}

#[test]
fn cited_chunk_set_delta_and_field_changes_are_sorted_on_first_event() {
    // First event has no prior snapshot, so:
    //   added = sorted(new_chunk_ids)
    //   removed = []
    //   frontmatter_field_changes = sorted(new keys)
    // The committed corpus card has one citation and 8 non-card_hash
    // frontmatter keys (schema_version, id, title, reading_id,
    // summary, citations, tags, card_edges); `tags` and `card_edges`
    // are empty additive lists treated as absence, leaving
    // 6 changed keys.
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards");
    copy_clean_card_with_stale_hash(&cards);
    let out = tmp.path().join("out");
    let ctx = DeterminismContext::frozen();
    crate::history::reset_history_cache();
    build_index(&cards, &out, &ctx).expect("build_index ok");

    let history_path = cards.join("01-content-addressable-identity.history.jsonl");
    let line = std::fs::read_to_string(&history_path).unwrap();
    let event: HistoryEvent = serde_json::from_str(line.trim()).unwrap();

    let added = event
        .cited_chunk_set_delta
        .get("added")
        .expect("added present");
    let removed = event
        .cited_chunk_set_delta
        .get("removed")
        .expect("removed present");
    assert_eq!(added.as_slice(), &["sample:p001:0000".to_string()]);
    assert!(removed.is_empty());
    let mut prev = String::new();
    for k in added {
        assert!(*k >= prev, "added must be sorted: {prev} > {k}");
        prev = k.clone();
    }

    let expected_keys = vec![
        "citations".to_string(),
        "id".to_string(),
        "reading_id".to_string(),
        "schema_version".to_string(),
        "summary".to_string(),
        "title".to_string(),
    ];
    assert_eq!(event.frontmatter_field_changes, expected_keys);

    assert_eq!(
        event.cited_chunk_ids_snapshot,
        vec!["sample:p001:0000".to_string()]
    );
}

#[test]
fn second_run_emits_zero_sidecars_after_rewrite() {
    // After the first run rewrites the card's frontmatter hash, a
    // second `build_index` over the same cards directory MUST NOT
    // append another history event -- the file is now self-
    // consistent. This is the rewrite-idempotency contract: a
    // buggy implementation that fails to rewrite the card would
    // emit a second event on the next run.
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards");
    copy_clean_card_with_stale_hash(&cards);
    let out_a = tmp.path().join("out_a");
    let ctx = DeterminismContext::frozen();
    crate::history::reset_history_cache();
    build_index(&cards, &out_a, &ctx).expect("first build_index");
    let history_path = cards.join("01-content-addressable-identity.history.jsonl");
    let after_first = std::fs::read(&history_path).expect("history after run 1");

    let out_b = tmp.path().join("out_b");
    crate::history::reset_history_cache();
    build_index(&cards, &out_b, &ctx).expect("second build_index");
    let after_second = std::fs::read(&history_path).expect("history after run 2");

    assert_eq!(
        after_first, after_second,
        "second run must NOT append another event"
    );
    let line_count = after_first.iter().filter(|&&b| b == b'\n').count();
    assert_eq!(line_count, 1, "history file must contain exactly one event");
}

#[test]
fn subsequent_event_deltas_compute_against_last_history_snapshot() {
    // Force two consecutive stale-hash events on the same card,
    // each with a DIFFERENT canonical `new_card_hash` so the
    // duplicate-suppression introduced in R17 (Codex review
    // finding: skip history append when last event already
    // records `entry.new_hash`) does NOT collapse the second
    // event. The second event's deltas must compute against the
    // first event's snapshot (which carries `cited_chunk_ids =
    // [sample:p001:0000]` and the post-rewrite frontmatter), so
    // `cited_chunk_set_delta = {"added":[], "removed":[]}` and
    // `frontmatter_field_changes = []` because the frontmatter
    // (minus card_hash) is unchanged between the two events --
    // ONLY the body is edited to drive the canonical hash to a
    // new value.
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards");
    let card_path = copy_clean_card_with_stale_hash(&cards);
    let out_a = tmp.path().join("out_a");
    let ctx = DeterminismContext::frozen();
    crate::history::reset_history_cache();
    build_index(&cards, &out_a, &ctx).expect("first build_index");

    // Edit the body AND re-stale the card_hash. The body edit
    // shifts the canonical hash to a new value (so the R17
    // duplicate-suppression sees `last_event.new_card_hash !=
    // entry.new_hash` and proceeds with the append); the
    // card_hash re-stale forces the per-card loop to enter the
    // rewrite branch. Frontmatter (minus card_hash) is
    // unchanged across runs because body is NOT part of the
    // frontmatter snapshot.
    let after_first = std::fs::read_to_string(&card_path).unwrap();
    let restaled = after_first.replace(
        "card_hash: \"714988273af6b3805ddf8a7c01226a17528a2b8090bed27d23e2fadbc1fd367a\"",
        "card_hash: \"0000000000000000000000000000000000000000000000000000000000000000\"",
    );
    // Append a sentinel to the body so the canonical hash
    // changes between runs. The append happens AFTER the
    // closing `---` so the YAML frontmatter is untouched.
    let body_edited = format!("{restaled}\nappended body line for R17 delta test\n");
    std::fs::write(&card_path, body_edited).unwrap();

    let out_b = tmp.path().join("out_b");
    crate::history::reset_history_cache();
    build_index(&cards, &out_b, &ctx).expect("second build_index");

    let history_path = cards.join("01-content-addressable-identity.history.jsonl");
    let lines: Vec<HistoryEvent> = std::fs::read_to_string(&history_path)
        .unwrap()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(lines.len(), 2, "expected exactly two history events");

    let second = &lines[1];
    let added = second
        .cited_chunk_set_delta
        .get("added")
        .expect("added present");
    let removed = second
        .cited_chunk_set_delta
        .get("removed")
        .expect("removed present");
    assert!(
        added.is_empty(),
        "added must be empty since chunk set is unchanged: {added:?}"
    );
    assert!(
        removed.is_empty(),
        "removed must be empty since chunk set is unchanged: {removed:?}"
    );
    assert!(
        second.frontmatter_field_changes.is_empty(),
        "frontmatter changes must be empty since fm (minus card_hash) is unchanged: {:?}",
        second.frontmatter_field_changes
    );

    // prev_card_hash must be the just-set 64-zero stale value, NOT
    // the new_card_hash of the first event.
    assert_eq!(
        second.prev_card_hash.as_deref(),
        Some("0000000000000000000000000000000000000000000000000000000000000000")
    );
    // Chain must validate: prev_checksum equals first event's event_checksum.
    assert_eq!(
        second.prev_checksum.as_deref(),
        Some(lines[0].event_checksum.as_str())
    );
}

#[test]
fn history_checksum_chain_validates_after_stale_hash_events() {
    // After the per-card history append, the chain prev_checksum
    // -> event_checksum must validate via the existing
    // `cacg_core::history::validate_history` chain validator. Any
    // tampered field would surface as a bad-line index.
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards");
    copy_clean_card_with_stale_hash(&cards);
    let out = tmp.path().join("out");
    let ctx = DeterminismContext::frozen();
    crate::history::reset_history_cache();
    build_index(&cards, &out, &ctx).expect("build_index ok");

    let history_path = cards.join("01-content-addressable-identity.history.jsonl");
    let bad = crate::history::validate_history(&history_path).unwrap();
    assert!(bad.is_empty(), "history chain must validate clean: {bad:?}");
}

#[test]
fn replace_card_hash_line_preserves_indentation_and_other_lines() {
    // Unit-level guard on the in-place rewrite helper: only the
    // card_hash line changes; leading whitespace + other lines are
    // preserved.
    let text = "---\n\
                schema_version: \"cacg.v0\"\n\
                id: \"foo\"\n\
                card_hash: \"AAAA\"\n\
                ---\nbody line\n";
    let out = replace_card_hash_line(text, "BBBB").expect("rewrite hits the line");
    assert!(out.contains("card_hash: \"BBBB\""));
    assert!(!out.contains("AAAA"));
    assert!(out.starts_with("---\n"));
    assert!(out.contains("schema_version: \"cacg.v0\""));
    assert!(out.contains("id: \"foo\""));
    assert!(out.ends_with("body line\n"));
}

#[test]
fn invalid_existing_history_blocks_card_rewrite_and_returns_error() {
    // Failure-atomicity preflight: a tampered `<card>.history.jsonl`
    // must be detected BEFORE the card frontmatter is rewritten,
    // so a later index pass cannot see a fresh stored hash and
    // silently skip the missing audit event. Mirrors Python's
    // `CACG-IDX-004` behavior (refuse to commit on tampered
    // history) and prevents the audit-trail loss on the stale-
    // card tampered-history failure path.
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards");
    let card_path = copy_clean_card_with_stale_hash(&cards);
    let original_card_text = std::fs::read_to_string(&card_path).unwrap();
    let bad_history = cards.join("01-content-addressable-identity.history.jsonl");
    std::fs::write(&bad_history, b"not json\n").unwrap();
    let original_history_bytes = std::fs::read(&bad_history).unwrap();

    let out = tmp.path().join("out");
    let ctx = DeterminismContext::frozen();
    crate::history::reset_history_cache();
    let result = build_index(&cards, &out, &ctx);
    assert!(
        matches!(result, Err(IndexError::History(_))),
        "build_index must reject invalid existing history; got: {result:?}"
    );

    // Card frontmatter MUST still carry the stale stored hash:
    // the preflight rejected before any rewrite touched disk.
    let card_after = std::fs::read_to_string(&card_path).unwrap();
    assert_eq!(
        card_after, original_card_text,
        "card frontmatter must remain unchanged when preflight rejects"
    );
    assert!(card_after.contains(
        "card_hash: \"0000000000000000000000000000000000000000000000000000000000000000\""
    ));

    // The invalid history file is untouched.
    let history_after = std::fs::read(&bad_history).unwrap();
    assert_eq!(
        history_after, original_history_bytes,
        "tampered history must remain unchanged when preflight rejects"
    );

    // Manifests must not be published either (the preflight runs
    // before manifest atomic-publish, so the out_dir is empty or
    // does not contain the new manifests).
    let manifest_path = out.join("cards_manifest.json");
    assert!(
        !manifest_path.exists(),
        "manifest must not be published when preflight rejects"
    );
}

#[test]
fn build_index_skips_history_append_when_last_event_already_records_new_hash() {
    // Codex R16 review finding: a repository that already ran the
    // previous Rust `kb index` on a stale card can have its
    // history sidecar ending with `new_card_hash ==
    // entry.new_hash` while the card frontmatter is still stale
    // (either an older Rust version wrote the history but failed
    // to rewrite the frontmatter, or external tampering reverted
    // the frontmatter). Re-running `build_index` in that state
    // must repair the card frontmatter WITHOUT appending a
    // duplicate audit event for the same canonical hash.
    //
    // Test flow:
    //   1. Stale-hash card -> first build_index appends the
    //      canonical history event AND repairs the frontmatter.
    //   2. Manually re-stale the card_hash field on disk (no
    //      body / frontmatter change), simulating the upgrade
    //      or tamper scenario.
    //   3. Second build_index: the recomputed canonical hash
    //      MATCHES the last event's `new_card_hash`, so the
    //      duplicate append is suppressed.
    //   4. Assertions: card frontmatter rewritten back to the
    //      canonical hash; history file BYTE-EQUAL to the
    //      post-first-run state (no second event appended).
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards");
    let card_path = copy_clean_card_with_stale_hash(&cards);
    let out_a = tmp.path().join("out_a");
    let ctx = DeterminismContext::frozen();
    crate::history::reset_history_cache();
    build_index(&cards, &out_a, &ctx).expect("first build_index");

    let history_path = cards.join("01-content-addressable-identity.history.jsonl");
    let history_after_first = std::fs::read(&history_path).unwrap();
    let card_after_first = std::fs::read_to_string(&card_path).unwrap();

    // Re-stale the card_hash (no body / frontmatter change). The
    // recomputed canonical hash is identical to the first run's
    // because everything except `card_hash` is unchanged.
    let restaled = replace_card_hash_line(
        &card_after_first,
        "0000000000000000000000000000000000000000000000000000000000000000",
    )
    .expect("first-run text must have a card_hash to replace");
    std::fs::write(&card_path, restaled).unwrap();

    // Second run.
    let out_b = tmp.path().join("out_b");
    crate::history::reset_history_cache();
    build_index(&cards, &out_b, &ctx).expect("second build_index");

    // Card frontmatter MUST be restored to the canonical hash.
    let card_after_second = std::fs::read_to_string(&card_path).unwrap();
    assert_eq!(
        card_after_second, card_after_first,
        "second run must rewrite the card frontmatter back to the canonical hash"
    );
    // History file MUST be byte-equal to the post-first-run
    // state -- the duplicate append is suppressed.
    let history_after_second = std::fs::read(&history_path).unwrap();
    assert_eq!(
        history_after_second, history_after_first,
        "second run must NOT append a duplicate history event when the last event already records `new_card_hash == entry.new_hash`"
    );
    // Defensive: exactly one event in the history file.
    let line_count = history_after_second.iter().filter(|&&b| b == b'\n').count();
    assert_eq!(
        line_count, 1,
        "history file must contain exactly one event after the upgrade-scenario re-run"
    );
}

#[test]
fn rewrite_card_hash_in_place_refuses_to_clobber_concurrent_edit() {
    // Codex R16 review finding: the R16 fix made the function
    // commit `original_text + new_hash`, which solved the double-
    // read-stale-hash hazard but silently clobbered any
    // concurrent edit made between the parse loop and the
    // rewrite call (atomic_publish renames the edited canonical
    // to `.bak`, replaces with `original_text + new_hash`, and
    // drops `.bak` on success -- losing the user's edit). The
    // R17 fix re-reads on-disk bytes inside the function and
    // refuses to publish when they differ from `original_text`,
    // preserving the concurrent edit verbatim.
    let tmp = TempDir::new().unwrap();
    let card_path = tmp.path().join("card.md");
    // The text the parse loop captured (T0 snapshot fed to
    // recompute_card_hash).
    let original_text = "---\n\
                         schema_version: \"cacg.v0\"\n\
                         id: \"x\"\n\
                         card_hash: \"OLD\"\n\
                         ---\n\
                         body line A\n";
    // The edited text on disk at the time `rewrite_card_hash_in_place`
    // runs (concurrent edit between parse and commit).
    let edited_text = "---\n\
                       schema_version: \"cacg.v0\"\n\
                       id: \"x\"\n\
                       card_hash: \"OLD\"\n\
                       ---\n\
                       body line B (concurrent edit)\n";
    std::fs::write(&card_path, edited_text).unwrap();

    let result = delta::rewrite_card_hash_in_place(&card_path, "NEWHASH", original_text);
    assert!(
        matches!(result, Err(IndexError::Canonical(_))),
        "rewrite must refuse when on-disk text differs from original_text; got: {result:?}"
    );
    // The concurrent edit MUST be preserved byte-for-byte.
    let after = std::fs::read_to_string(&card_path).unwrap();
    assert_eq!(
        after, edited_text,
        "edited text must be preserved verbatim when the rewrite refuses: got\n{after}\nexpected\n{edited_text}"
    );
    // No `.tmp`/`.bak` leftovers (the function returned BEFORE
    // calling atomic_publish, so the publisher's `.tmp` staging
    // never ran).
    let tmp_path = card_path.with_extension("md.tmp");
    let bak_path = card_path.with_extension("md.bak");
    assert!(!tmp_path.exists(), "no leftover {tmp_path:?}");
    assert!(!bak_path.exists(), "no leftover {bak_path:?}");
}

#[test]
fn replace_card_hash_line_returns_none_when_no_card_hash_field() {
    let text = "---\n\
                schema_version: \"cacg.v0\"\n\
                id: \"foo\"\n\
                ---\nbody\n";
    assert!(replace_card_hash_line(text, "BBBB").is_none());
}

/// Author a syntactically-valid card whose frontmatter omits the
/// `card_hash:` field entirely (mirrors a freshly authored card
/// from `kb new`). Returns the destination card path.
fn copy_clean_card_without_card_hash_field(dst_dir: &Path) -> PathBuf {
    let mut src = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    src.pop();
    src.pop();
    src.push("tests/parity_corpus/cards/reading_01/synthetic-card-01.md");
    let original = std::fs::read_to_string(&src).expect("read seed clean card");
    // Strip the entire `card_hash: "..."` line including its
    // trailing newline so the result is a valid frontmatter that
    // simply omits the field.
    let mut filtered = String::with_capacity(original.len());
    for raw in original.split_inclusive('\n') {
        if raw.trim_start().starts_with("card_hash:") {
            continue;
        }
        filtered.push_str(raw);
    }
    std::fs::create_dir_all(dst_dir).unwrap();
    let dst = dst_dir.join("synthetic-card-01.md");
    std::fs::write(&dst, filtered).unwrap();
    dst
}

#[test]
fn build_index_inserts_card_hash_when_absent_and_is_idempotent() {
    // Idempotency contract: a freshly authored card whose
    // frontmatter omits the `card_hash:` field must (i) gain a
    // `card_hash:` line on the first index, (ii) emit exactly one
    // history event, (iii) be a no-op on the second index --
    // matching Python's `_render_with_hash` behavior, which always
    // emits the field regardless of whether the source omitted it.
    // The pre-fix in-place rewrite silently no-op'd on missing
    // line; the next `kb index` re-entered the stale branch and
    // appended a duplicate history event for the same unchanged
    // hash.
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards");
    let card_path = copy_clean_card_without_card_hash_field(&cards);
    let card_before = std::fs::read_to_string(&card_path).unwrap();
    assert!(
        !card_before.contains("card_hash:"),
        "seed card must omit card_hash for this test to be meaningful"
    );

    let out = tmp.path().join("out");
    let ctx = DeterminismContext::frozen();
    crate::history::reset_history_cache();
    build_index(&cards, &out, &ctx).expect("first build_index ok");

    // (i) Card frontmatter now has a `card_hash: "..."` line.
    let card_after_first = std::fs::read_to_string(&card_path).unwrap();
    assert!(
        card_after_first.contains("card_hash: \""),
        "first build_index must INSERT a card_hash line; card text:\n{card_after_first}"
    );

    // (ii) Exactly one history event landed.
    let history_path = cards.join("synthetic-card-01.history.jsonl");
    assert!(history_path.is_file(), "history sidecar must exist");
    let history_bytes_after_first = std::fs::read(&history_path).unwrap();
    let line_count = history_bytes_after_first
        .iter()
        .filter(|&&b| b == b'\n')
        .count();
    assert_eq!(
        line_count, 1,
        "first build_index must emit exactly one history event"
    );

    // (iii) Second run is a no-op: card text unchanged AND the
    // history file size does not grow.
    crate::history::reset_history_cache();
    build_index(&cards, &out, &ctx).expect("second build_index ok");
    let card_after_second = std::fs::read_to_string(&card_path).unwrap();
    assert_eq!(
        card_after_second, card_after_first,
        "second build_index must not modify the card text"
    );
    let history_bytes_after_second = std::fs::read(&history_path).unwrap();
    assert_eq!(
        history_bytes_after_second, history_bytes_after_first,
        "second build_index must not grow the history sidecar (idempotency)"
    );
}

#[test]
fn build_index_rolls_back_card_rewrite_when_history_append_fails() {
    // Failure-atomicity at the rewrite+history step: if
    // `append_per_card_history` fails AFTER `rewrite_card_hash_in_place`
    // has committed the new hash, the card text must be rolled back
    // and the history sidecar must be left unchanged so the next
    // `kb index` run still sees `prev_hash != new_hash` and retries
    // the audit-trail write. Mirrors Python's transaction at
    // `legacy_python_oracle/src/cacg/index.py` lines 917-1063 collapsed to per-card
    // granularity (BL-20260518-multi-file-replace-rollback).
    //
    // Failure injection: pre-create the history file's lock-path
    // (`<card>.history.jsonl.lock`) as a directory. The validate-
    // history preflight only touches `<card>.history.jsonl` (which
    // does not yet exist), so it passes. Then the per-card loop
    // calls `append_history_event`, which tries to `open(WRONLY |
    // CREATE)` the lock path -- opening a directory for write
    // returns EISDIR, surfacing as `HistoryError::Io`. This
    // exercises the rollback path.
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards");
    let card_path = copy_clean_card_with_stale_hash(&cards);
    let original_card_text = std::fs::read_to_string(&card_path).unwrap();
    let history_path = cards.join("01-content-addressable-identity.history.jsonl");
    // History file itself does NOT pre-exist; the lock path does
    // (as a directory) so the append step fails.
    assert!(!history_path.exists(), "history sidecar must not pre-exist");
    let lock_path = cards.join("01-content-addressable-identity.history.jsonl.lock");
    std::fs::create_dir_all(&lock_path).unwrap();

    let out = tmp.path().join("out");
    let ctx = DeterminismContext::frozen();
    crate::history::reset_history_cache();
    let result = build_index(&cards, &out, &ctx);
    assert!(
        matches!(result, Err(IndexError::History(_))),
        "build_index must propagate the history-append error; got: {result:?}"
    );

    // Card text MUST be rolled back to the stale-hash original.
    let card_after = std::fs::read_to_string(&card_path).unwrap();
    assert_eq!(
        card_after, original_card_text,
        "card text must be restored to its pre-rewrite state"
    );
    assert!(
        card_after.contains(
            "card_hash: \"0000000000000000000000000000000000000000000000000000000000000000\""
        ),
        "rolled-back card must still carry the stale stored hash"
    );

    // History sidecar MUST NOT exist (it did not pre-exist; the
    // failed append + rollback must leave the disk in the prior
    // state). The lock-path directory we created is left as-is
    // because the rollback does not touch the lock.
    assert!(
        !history_path.exists(),
        "history sidecar must not be left behind after rollback"
    );
    assert!(
        lock_path.is_dir(),
        "pre-existing lock-path directory must be preserved"
    );

    // No `.tmp` / `.bak` leftovers next to the card.
    let tmp_path = card_path.with_extension("md.tmp");
    let bak_path = card_path.with_extension("md.bak");
    assert!(!tmp_path.exists(), "no leftover {tmp_path:?}");
    assert!(!bak_path.exists(), "no leftover {bak_path:?}");

    // Manifest artifacts MUST NOT exist after rollback: the
    // seed corpus's `out_dir` started empty, so the rollback
    // path must UNLINK the newly-published cards_manifest.json
    // / summaries.json / INDEX.md (NOT leave them on disk with
    // freshly-published bytes that no longer match the rolled-
    // back card state). Mirrors Python's
    // `_IndexPairResult`-driven manifest-pair restore branch.
    assert!(
        !out.join("cards_manifest.json").exists(),
        "cards_manifest.json must not be left behind after rollback"
    );
    assert!(
        !out.join("summaries.json").exists(),
        "summaries.json must not be left behind after rollback"
    );
    assert!(
        !out.join("INDEX.md").exists(),
        "INDEX.md must not be left behind after rollback"
    );
    // No leftover manifest `.tmp` / `.bak` sidecars either.
    for sidecar in [
        "cards_manifest.json.tmp",
        "cards_manifest.json.bak",
        "summaries.json.tmp",
        "summaries.json.bak",
        "INDEX.md.tmp",
        "INDEX.md.bak",
    ] {
        let p = out.join(sidecar);
        assert!(!p.exists(), "no leftover {p:?} after rollback");
    }
}

/// Copy a parity-corpus card into `dst_dir` with its `card_hash:`
/// line rewritten to 64 zeros so `build_index` enters the stale
/// branch. Uses the in-crate `replace_card_hash_line` helper so
/// the test does not depend on each seed card's canonical hash.
fn copy_card_with_stale_hash(dst_dir: &Path, slug: &str) -> PathBuf {
    let mut src = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    src.pop();
    src.pop();
    src.push(format!("tests/parity_corpus/cards/reading_01/{slug}.md"));
    let original = std::fs::read_to_string(&src).expect("read seed clean card");
    let stale = replace_card_hash_line(
        &original,
        "0000000000000000000000000000000000000000000000000000000000000000",
    )
    .expect("seed card must have a card_hash line");
    std::fs::create_dir_all(dst_dir).unwrap();
    let dst = dst_dir.join(format!("{slug}.md"));
    std::fs::write(&dst, stale).unwrap();
    dst
}

#[test]
fn build_index_surfaces_cacg_idx_008_when_output_artifact_is_a_directory() {
    // The R12 manifest-prior-bytes snapshot path must NOT pre-
    // empt `atomic_publish`'s `NonFileCanonical` (`CACG-IDX-008`)
    // shape-check. If a published-artifact path exists but is
    // not a regular file (e.g., `out_dir/cards_manifest.json`
    // is a directory), `std::fs::read(path)` would have failed
    // with EISDIR and surfaced as `IndexError::Io`, masking the
    // documented `CACG-IDX-008` diagnostic. The R15 fix uses
    // `std::fs::metadata` to detect this case in
    // `read_prior_bytes` and returns `Ok(None)`, letting
    // `atomic_publish` run its preflight and emit the canonical
    // error.
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards");
    let card_path = copy_clean_card_with_stale_hash(&cards);
    let original_card_text = std::fs::read_to_string(&card_path).unwrap();
    let out = tmp.path().join("out");
    // Pre-create `out_dir/cards_manifest.json` as a DIRECTORY so
    // the manifest publish's preflight rejects it.
    std::fs::create_dir_all(out.join("cards_manifest.json")).unwrap();

    let ctx = DeterminismContext::frozen();
    crate::history::reset_history_cache();
    let result = build_index(&cards, &out, &ctx);

    // The error MUST be the publisher's `NonFileCanonical`
    // carrying the `CACG-IDX-008` diagnostic, NOT
    // `IndexError::Io` (EISDIR).
    match result {
        Err(IndexError::Publish(
            crate::atomic_publish::PublishError::NonFileCanonical { diagnostic, paths },
        )) => {
            assert_eq!(diagnostic, "CACG-IDX-008");
            assert!(
                paths.iter().any(|p| p.ends_with("cards_manifest.json")),
                "NonFileCanonical paths must include the non-file canonical: {paths:?}"
            );
        }
        other => panic!(
            "expected IndexError::Publish(PublishError::NonFileCanonical {{diagnostic:\"CACG-IDX-008\", ...}}); got: {other:?}",
        ),
    }

    // Card text MUST be unchanged: the publish failed before any
    // per-card rewrite was attempted.
    let card_after = std::fs::read_to_string(&card_path).unwrap();
    assert_eq!(
        card_after, original_card_text,
        "stale card text must remain unchanged when manifest publish refuses"
    );
    // The pre-existing directory at the canonical path remains
    // as-is (the publisher's preflight rejected before any
    // rename was attempted).
    assert!(
        out.join("cards_manifest.json").is_dir(),
        "pre-existing directory at canonical path must be preserved"
    );
}

#[test]
fn rollback_after_partial_index_commit_restores_manifests_on_no_prior_run() {
    // Codex R23 review (P2): pre-append `?` propagations in
    // the per-card loop (`std::fs::read`, `std::fs::metadata`,
    // `last_event_new_card_hash`) used to skip the rollback
    // chain entirely after manifest publish. R24 extracted
    // `rollback_after_partial_index_commit` (which centralizes
    // `rollback_committed_cards` + 3 × `rollback_manifest`) and
    // routes all post-publish failure paths through it. This
    // test directly exercises the helper with the no-prior-run
    // case (all three `prior_bytes` are `None`): the manifest
    // artifacts published before the failure must be unlinked,
    // mirroring the R12 no-prior-history semantic on the
    // manifest level.
    let tmp = TempDir::new().unwrap();
    let out = tmp.path().join("out");
    std::fs::create_dir_all(&out).unwrap();

    // Simulate post-publish state: 3 manifest artifacts on
    // disk with arbitrary "new bytes" content (the bytes the
    // failing run wrote before its pre-append error fired).
    let manifest_path = out.join("cards_manifest.json");
    let summaries_path = out.join("summaries.json");
    let index_md_path = out.join("INDEX.md");
    std::fs::write(&manifest_path, b"new_cards_manifest_bytes").unwrap();
    std::fs::write(&summaries_path, b"new_summaries_bytes").unwrap();
    std::fs::write(&index_md_path, b"new_index_md_bytes").unwrap();

    rollback_after_partial_index_commit(
        &[],
        &manifest_path,
        &summaries_path,
        &index_md_path,
        None,
        None,
        None,
    );

    // All three published artifacts MUST be unlinked because
    // no prior version existed (None for each `prior_bytes`).
    assert!(
        !manifest_path.exists(),
        "cards_manifest.json must be unlinked when no prior version existed"
    );
    assert!(
        !summaries_path.exists(),
        "summaries.json must be unlinked when no prior version existed"
    );
    assert!(
        !index_md_path.exists(),
        "INDEX.md must be unlinked when no prior version existed"
    );
}

#[test]
fn rollback_after_partial_index_commit_restores_manifests_to_prior_bytes() {
    // Companion to the no-prior-run test: when `prior_bytes`
    // is `Some(...)`, the helper must re-publish the prior
    // bytes via `atomic_publish` so the manifests return to
    // their pre-call state (mirrors the R12 manifest-snapshot
    // restore for the with-prior case).
    let tmp = TempDir::new().unwrap();
    let out = tmp.path().join("out");
    std::fs::create_dir_all(&out).unwrap();

    let manifest_path = out.join("cards_manifest.json");
    let summaries_path = out.join("summaries.json");
    let index_md_path = out.join("INDEX.md");
    // Disk currently holds the post-publish bytes.
    std::fs::write(&manifest_path, b"new_cards_manifest_bytes").unwrap();
    std::fs::write(&summaries_path, b"new_summaries_bytes").unwrap();
    std::fs::write(&index_md_path, b"new_index_md_bytes").unwrap();

    let prior_manifest = b"prior_cards_manifest_bytes_v0".to_vec();
    let prior_summaries = b"prior_summaries_bytes_v0".to_vec();
    let prior_index_md = b"prior_index_md_bytes_v0".to_vec();
    rollback_after_partial_index_commit(
        &[],
        &manifest_path,
        &summaries_path,
        &index_md_path,
        Some(&prior_manifest),
        Some(&prior_summaries),
        Some(&prior_index_md),
    );

    assert_eq!(
        std::fs::read(&manifest_path).unwrap(),
        prior_manifest,
        "cards_manifest.json must be restored to prior bytes"
    );
    assert_eq!(
        std::fs::read(&summaries_path).unwrap(),
        prior_summaries,
        "summaries.json must be restored to prior bytes"
    );
    assert_eq!(
        std::fs::read(&index_md_path).unwrap(),
        prior_index_md,
        "INDEX.md must be restored to prior bytes"
    );
}

#[test]
fn rollback_committed_cards_preserves_history_when_card_restore_fails() {
    // Codex R21 review (P2): if `atomic_publish`'s card-text
    // restore fails inside `rollback_committed_cards` (e.g., a
    // foreign `<card>.md.tmp`/`<card>.md.bak` sidecar appeared
    // between our append and the rollback, or a permission
    // problem blocks the replace), the prior R21 code
    // swallowed the error AND still truncated the history.
    // Net result: card stays at the post-rewrite `new_hash`
    // content + history sidecar is truncated. Next index run
    // sees `prev_hash == new_hash` and skips the stale branch
    // -- audit event permanently lost.
    //
    // R22 fix: capture the `atomic_publish` result and SKIP
    // the history truncate when the restore failed. This test
    // triggers the restore-failure path by pre-creating
    // `<card>.md.bak` as a foreign file so the publisher's
    // sidecar preflight returns `PublishError::PreexistingSidecars`.
    // Asserts: card text unchanged (restore failed); history
    // file byte-equal (truncate suppressed); foreign `.md.bak`
    // preserved verbatim (BL-20260518-no-clobber-foreign-sidecars).
    let tmp = TempDir::new().unwrap();
    let card_path = tmp.path().join("card.md");
    // Card on disk: contains the NEW (canonical) hash --
    // simulating a card that was successfully rewritten by
    // this iteration AND a successful history append, then a
    // LATER iteration failed and triggered rollback_committed_cards.
    let post_rewrite_card_text =
        b"---\nschema_version: \"cacg.v0\"\nid: \"x\"\ncard_hash: \"NEW\"\n---\nbody\n";
    std::fs::write(&card_path, post_rewrite_card_text).unwrap();
    let prior_card_text =
        b"---\nschema_version: \"cacg.v0\"\nid: \"x\"\ncard_hash: \"OLD\"\n---\nbody\n".to_vec();

    // Pre-create `.md.bak` as a foreign file so atomic_publish's
    // sidecar preflight refuses.
    let bak_path = card_path.with_extension("md.bak");
    let foreign_bak_bytes = b"USER_OWNED_BAK_DO_NOT_CLOBBER\n";
    std::fs::write(&bak_path, foreign_bak_bytes).unwrap();

    // History file already records the canonical event.
    let history_path = tmp.path().join("card.history.jsonl");
    let history_bytes = b"{\"new_card_hash\":\"NEW\",\"seq\":0}\n";
    std::fs::write(&history_path, history_bytes).unwrap();

    let committed = vec![CommittedCard {
        card_path: card_path.clone(),
        prior_card_text,
        history_path: history_path.clone(),
        history_existed_before: false,
        history_append_outcome: Some(crate::history::AppendOutcome {
            seq: 0,
            prior_size_under_lock: 0,
            post_write_size: history_bytes.len() as u64,
        }),
    }];

    rollback_committed_cards(&committed);

    // Card text must be unchanged: atomic_publish refused due
    // to the foreign `.md.bak`, so the canonical path was
    // never touched.
    let card_after = std::fs::read(&card_path).unwrap();
    assert_eq!(
        card_after,
        post_rewrite_card_text,
        "card text must be unchanged when restore fails (atomic_publish refused due to foreign .md.bak)"
    );

    // History file must be unchanged: the truncate was
    // suppressed because the card restore failed. The audit
    // event is preserved.
    let history_after = std::fs::read(&history_path).unwrap();
    assert_eq!(
        history_after, history_bytes,
        "history file must be preserved (truncate suppressed) when card restore fails"
    );
    // For completeness: the history file still exists (not
    // unlinked) and contains the canonical event.
    assert!(
        history_path.is_file(),
        "history sidecar must still exist when card restore fails"
    );

    // User's foreign `.md.bak` MUST be preserved verbatim.
    let bak_after = std::fs::read(&bak_path).unwrap();
    assert_eq!(
        bak_after, foreign_bak_bytes,
        "foreign .md.bak must be preserved (BL-20260518-no-clobber-foreign-sidecars)"
    );
}

#[test]
fn rollback_committed_cards_preserves_concurrently_created_history_sidecar() {
    // Codex R20 review (P2): the post-truncate unlink decision
    // relied solely on the pre-flock `history_existed_before`
    // heuristic. Under a concurrent-create race (another
    // `kb index` creates + appends to the history file between
    // our pre-flock check and our flock acquisition), the
    // pre-flock check returns false, but the file actually
    // exists with the concurrent writer's event under our
    // lock. R21 added the `outcome.prior_size_under_lock == 0`
    // guard so the unlink does NOT fire in this scenario.
    //
    // This test simulates the race by constructing a
    // `CommittedCard` with `history_existed_before = false`
    // (the stale pre-flock observation) AND an `AppendOutcome`
    // whose `prior_size_under_lock > 0` (the concurrent
    // writer's event size under our lock). The history file
    // is pre-written with both the concurrent writer's bytes
    // and our event bytes (matching `post_write_size`). After
    // `rollback_committed_cards`, the file must:
    //   * Still exist (NOT unlinked).
    //   * Contain ONLY the concurrent writer's bytes (truncated
    //     to `prior_size_under_lock`).
    let tmp = TempDir::new().unwrap();
    let card_path = tmp.path().join("card.md");
    let prior_card_text = b"---\nschema_version: \"cacg.v0\"\nid: \"x\"\n---\nbody\n";
    std::fs::write(&card_path, prior_card_text).unwrap();

    let history_path = tmp.path().join("card.history.jsonl");
    // The concurrent writer's event landed under our flock
    // BEFORE our append. The bytes are arbitrary for this
    // test; only the size matters.
    let concurrent_event_bytes = b"{\"concurrent_writer\":\"appended_first\",\"seq\":0}\n";
    let our_event_bytes = b"{\"this_process\":\"appended_second\",\"seq\":1}\n";
    let mut combined: Vec<u8> = Vec::new();
    combined.extend_from_slice(concurrent_event_bytes);
    combined.extend_from_slice(our_event_bytes);
    std::fs::write(&history_path, &combined).unwrap();

    let committed = vec![CommittedCard {
        card_path: card_path.clone(),
        prior_card_text: prior_card_text.to_vec(),
        history_path: history_path.clone(),
        history_existed_before: false, // pre-flock observation: stale under race
        history_append_outcome: Some(crate::history::AppendOutcome {
            seq: 1,
            prior_size_under_lock: concurrent_event_bytes.len() as u64,
            post_write_size: combined.len() as u64,
        }),
    }];

    rollback_committed_cards(&committed);

    // History file MUST still exist -- the concurrent writer's
    // event must NOT be deleted by our rollback.
    assert!(
        history_path.is_file(),
        "concurrent writer's history sidecar must not be unlinked by rollback"
    );
    // The file must be truncated to `prior_size_under_lock`,
    // preserving the concurrent writer's event verbatim and
    // removing only our event.
    let after = std::fs::read(&history_path).unwrap();
    assert_eq!(
        after, concurrent_event_bytes,
        "rollback must preserve the concurrent writer's bytes verbatim"
    );
}

#[test]
fn build_index_rolls_back_manifests_when_card_rewrite_fails() {
    // Failure injection: pre-create `<card>.md.bak` as a regular
    // file at the path `atomic_publish` would stage to during
    // `rewrite_card_hash_in_place`. The publisher's sidecar
    // preflight catches it and returns
    // `PublishError::PreexistingSidecars`, surfacing as
    // `IndexError::Publish` from the per-card rewrite call. This
    // exercises the rewrite-failure branch of the per-card
    // transaction. The expected outcome: the 3 manifest
    // artifacts published BEFORE the per-card loop must be
    // rolled back (Codex R12 review finding 1), the user-owned
    // `.bak` must be preserved verbatim
    // (BL-20260518-no-clobber-foreign-sidecars), and the stale
    // card must keep its pre-`build_index` text.
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards");
    let card_path = copy_card_with_stale_hash(&cards, "01-content-addressable-identity");
    let original_card_text = std::fs::read_to_string(&card_path).unwrap();
    // User-owned `.bak` sidecar that the rewrite must refuse to
    // clobber.
    let user_bak = card_path.with_extension("md.bak");
    let user_bak_bytes = b"USER_OWNED_BAK_DO_NOT_CLOBBER\n";
    std::fs::write(&user_bak, user_bak_bytes).unwrap();

    let out = tmp.path().join("out");
    let ctx = DeterminismContext::frozen();
    crate::history::reset_history_cache();
    let result = build_index(&cards, &out, &ctx);
    assert!(
        matches!(result, Err(IndexError::Publish(_))),
        "build_index must propagate the sidecar-clobber error; got: {result:?}"
    );

    // User's `.bak` MUST be preserved byte-for-byte.
    assert!(
        user_bak.is_file(),
        "user-owned {user_bak:?} must not be deleted by rollback"
    );
    let user_bak_after = std::fs::read(&user_bak).unwrap();
    assert_eq!(
        user_bak_after, user_bak_bytes,
        "user-owned .bak bytes must be preserved verbatim"
    );

    // Stale card's text MUST be unchanged.
    let card_after = std::fs::read_to_string(&card_path).unwrap();
    assert_eq!(
        card_after, original_card_text,
        "stale card text must remain unchanged when rewrite is refused"
    );

    // The 3 manifest artifacts MUST NOT exist after rollback.
    // The `out_dir` started empty, so each manifest's prior
    // bytes were `None`; the rollback unlinked the newly-
    // published files.
    assert!(
        !out.join("cards_manifest.json").exists(),
        "cards_manifest.json must not be left behind after rewrite-failure rollback"
    );
    assert!(
        !out.join("summaries.json").exists(),
        "summaries.json must not be left behind after rewrite-failure rollback"
    );
    assert!(
        !out.join("INDEX.md").exists(),
        "INDEX.md must not be left behind after rewrite-failure rollback"
    );
    // No leftover history sidecar either (none should have been
    // touched on the rewrite-failure path).
    let history_path = cards.join("01-content-addressable-identity.history.jsonl");
    assert!(
        !history_path.exists(),
        "history sidecar must not be created when rewrite fails"
    );
}

#[test]
fn build_index_rolls_back_prior_cards_on_later_history_failure() {
    // Multi-card transaction: 2 stale cards in the corpus.
    // Inject a history-append failure on the SECOND card by
    // pre-creating its `<card>.history.jsonl.lock` as a
    // directory (so `rustix::fs::open(WRONLY|CREATE)` fails with
    // EISDIR after the preflight). The expected outcome: the
    // FIRST card was rewritten + history-appended in iteration 0
    // and is now in `committed`; iteration 1's history failure
    // triggers the rollback walk, restoring BOTH cards' text
    // and BOTH cards' history sidecars + the 3 manifest
    // artifacts (Codex R12 review finding 2). Without this, the
    // corpus would be left with card 1 holding a new hash + one
    // committed history event and card 2 rolled back -- a
    // partial commit.
    let tmp = TempDir::new().unwrap();
    let cards = tmp.path().join("cards");
    let card1 = copy_card_with_stale_hash(&cards, "01-content-addressable-identity");
    let card2 = copy_card_with_stale_hash(&cards, "02-determinism-is-a-kept-promise");
    let original_card1_text = std::fs::read_to_string(&card1).unwrap();
    let original_card2_text = std::fs::read_to_string(&card2).unwrap();

    // Inject failure on card 2's history-append step.
    let history_path_1 = cards.join("01-content-addressable-identity.history.jsonl");
    let history_path_2 = cards.join("02-determinism-is-a-kept-promise.history.jsonl");
    let lock_path_2 = cards.join("02-determinism-is-a-kept-promise.history.jsonl.lock");
    std::fs::create_dir_all(&lock_path_2).unwrap();

    let out = tmp.path().join("out");
    let ctx = DeterminismContext::frozen();
    crate::history::reset_history_cache();
    let result = build_index(&cards, &out, &ctx);
    assert!(
        matches!(result, Err(IndexError::History(_))),
        "build_index must propagate the history-append error; got: {result:?}"
    );

    // BOTH cards' text MUST be restored to the stale-hash
    // original. Card 1 was committed in iteration 0 (its
    // rewrite + history both succeeded); the iteration-1 failure
    // triggered the rollback walk which restored card 1 via
    // `atomic_publish` and truncated its history. Card 2's
    // rewrite committed before its history failed, so it gets
    // restored too.
    let card1_after = std::fs::read_to_string(&card1).unwrap();
    assert_eq!(
        card1_after, original_card1_text,
        "card 1's text must be restored (it was committed in iteration 0)"
    );
    let card2_after = std::fs::read_to_string(&card2).unwrap();
    assert_eq!(
        card2_after, original_card2_text,
        "card 2's text must be restored (its rewrite committed before history failed)"
    );

    // NEITHER history sidecar should exist on disk: card 1's
    // history was appended (one event landed) and then
    // truncated/unlinked by the rollback; card 2's history was
    // never written.
    assert!(
        !history_path_1.exists(),
        "card 1's history sidecar must be unlinked by the rollback"
    );
    assert!(
        !history_path_2.exists(),
        "card 2's history sidecar must not exist"
    );

    // Manifests MUST be rolled back.
    assert!(
        !out.join("cards_manifest.json").exists(),
        "cards_manifest.json must not be left behind after multi-card rollback"
    );
    assert!(
        !out.join("summaries.json").exists(),
        "summaries.json must not be left behind after multi-card rollback"
    );
    assert!(
        !out.join("INDEX.md").exists(),
        "INDEX.md must not be left behind after multi-card rollback"
    );

    // Lock-path directory we created remains as-is (the
    // rollback does not touch the lock).
    assert!(
        lock_path_2.is_dir(),
        "pre-existing lock-path directory must be preserved"
    );

    // No leftover `.tmp`/`.bak` sidecars next to either card.
    for path in [&card1, &card2] {
        let tmp = path.with_extension("md.tmp");
        let bak = path.with_extension("md.bak");
        assert!(!tmp.exists(), "no leftover {tmp:?}");
        assert!(!bak.exists(), "no leftover {bak:?}");
    }
}
