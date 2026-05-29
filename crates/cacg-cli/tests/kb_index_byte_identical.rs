#![allow(clippy::unwrap_used)]
//! End-to-end byte-identical proof for `kb index` under
//! `KB_FROZEN_CLOCK=1`.
//!
//! Two independent runs of the compiled `kb` binary against the
//! SAME shared `cards_dir` input (with separate `--out` dirs) must
//! produce byte-identical `cards_manifest.json`, `summaries.json`,
//! and `INDEX.md`. The committed parity corpus's frontmatter
//! `card_hash` values match the recomputed hash, so neither run
//! appends a `*.history.jsonl` sidecar — the test asserts that
//! invariant directly. Any determinism leak in the CLI dispatcher
//! or the `cacg_core::index` library would surface here as
//! diverging bytes or an unexpected sidecar.

use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn copy_cards_corpus(src: &Path, dst: &Path) {
    std::fs::create_dir_all(dst).expect("create dst dir");
    for entry in std::fs::read_dir(src).expect("read src corpus") {
        let entry = entry.expect("read_dir entry");
        let file_type = entry.file_type().expect("file_type");
        if file_type.is_file() {
            let name = entry.file_name();
            // Skip pre-existing Python-generated `.history.jsonl`
            // sidecars: this test compares two Rust-only runs from a
            // clean slate, so committed Python history bytes would
            // corrupt the comparison.
            if name.to_string_lossy().ends_with(".history.jsonl") {
                continue;
            }
            std::fs::copy(entry.path(), dst.join(name)).expect("copy file");
        }
    }
}

fn run_kb_index(cards_dir: &Path, out_dir: &Path) {
    let kb_bin = env!("CARGO_BIN_EXE_kb");
    let status = Command::new(kb_bin)
        .arg("index")
        .arg(cards_dir)
        .arg("--out")
        .arg(out_dir)
        .env("KB_FROZEN_CLOCK", "1")
        .status()
        .expect("spawn kb binary");
    assert!(
        status.success(),
        "kb index exited with non-success: {status}"
    );
}

#[test]
fn kb_index_twice_under_kb_frozen_clock_produces_byte_identical_artifacts() {
    let corpus_src = workspace_root().join("tests/parity_corpus/cards/reading_01");
    assert!(
        corpus_src.is_dir(),
        "parity corpus must exist at {corpus_src:?}"
    );

    // ONE shared cards_dir between the two runs (matches the parity
    // harness's invocation pattern + Python's path-as-given manifest
    // semantics). Each run gets its own out_dir.
    let shared_tmp = TempDir::new().expect("shared tempdir");
    let shared_cards = shared_tmp.path().join("cards/reading_01");
    copy_cards_corpus(&corpus_src, &shared_cards);

    let tmp_a = TempDir::new().expect("tempdir A");
    let out_a = tmp_a.path().join("out");

    let tmp_b = TempDir::new().expect("tempdir B");
    let out_b = tmp_b.path().join("out");

    run_kb_index(&shared_cards, &out_a);
    // The first kb index run writes cards_manifest.json + summaries.json
    // + INDEX.md to out_a. It also COULD append per-card history into
    // shared_cards (next to each .md). Both must be cleaned/aligned
    // before the second run so the harness's "fresh state" invariant
    // holds — but with the R38 update the build_index skips history
    // append when the frontmatter card_hash already matches the
    // recomputed hash, so the corpus stays untouched and the second
    // run reads the same input.
    run_kb_index(&shared_cards, &out_b);

    // Compare the 3 out_dir artifacts byte-for-byte.
    for name in ["cards_manifest.json", "summaries.json", "INDEX.md"] {
        let a_bytes =
            std::fs::read(out_a.join(name)).unwrap_or_else(|e| panic!("read out_a/{name}: {e}"));
        let b_bytes =
            std::fs::read(out_b.join(name)).unwrap_or_else(|e| panic!("read out_b/{name}: {e}"));
        assert_eq!(
            a_bytes, b_bytes,
            "kb index {name} must be byte-identical between two frozen runs",
        );
    }

    // Per-card history is now Python-equivalent: skipped when
    // frontmatter card_hash matches the recomputed hash. The committed
    // corpus has correct hashes, so no .history.jsonl should be
    // produced. Confirm.
    let history_names: Vec<String> = std::fs::read_dir(&shared_cards)
        .expect("read shared_cards")
        .filter_map(std::result::Result::ok)
        .filter(|e| {
            e.path()
                .file_name()
                .is_some_and(|n| n.to_string_lossy().ends_with(".history.jsonl"))
        })
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    assert!(
        history_names.is_empty(),
        "no .history.jsonl should be emitted for the parity corpus (frontmatter hashes are correct); got: {history_names:?}"
    );
}
