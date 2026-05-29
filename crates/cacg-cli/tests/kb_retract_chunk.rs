#![allow(clippy::unwrap_used)]
//! Integration tests for the native `kb retract-chunk` CLI verb.
//!
//! Spawns the built `kb` binary and exercises:
//!   - happy path: retract a chunk; assert stdout summary +
//!     `chunks_manifest` mutation.
//!   - `CACG-CLI-001` `chunks_manifest` missing (exit 1, exact wire text).
//!   - `CACG-CLI-001` unknown `chunk_id` (exit 1, exact wire text incl.
//!     `py_repr`-quoted `chunk_id`).
//!   - `CACG-CLI-001` already-retracted without cascade (exit 1).
//!   - `CACG-MAN-002` pre-existing `.tmp`/`.bak` sidecar (exit 1,
//!     exact wire text incl. Python list-of-str sidecar formatting).
//!   - byte-equality with Python `kb retract-chunk` (skips when
//!     `legacy_python_oracle/.venv/bin/python` is absent; `CACG_REQUIRE_PYTHON=1` makes
//!     the skip a hard failure).
//!   - cascade happy path: `chunks_manifest` + `cards_manifest` +
//!     INDEX.md all updated; SKILL.md routers skipped.
//!   - cascade already-retracted no-op: exit 1 with
//!     "already retracted and the dependency cascade is already
//!     up-to-date" wire text.
//!   - cascade byte-equality with Python.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn skip_if_python_unavailable(venv_python: &Path) -> bool {
    if venv_python.is_file() {
        return false;
    }
    assert!(
        !(std::env::var("CACG_REQUIRE_PYTHON").as_deref() == Ok("1")),
        "CACG_REQUIRE_PYTHON=1 set but project .venv is absent at {}",
        venv_python.display()
    );
    eprintln!(
        "skipping: project .venv not present at {} \
         (set CACG_REQUIRE_PYTHON=1 to make this a hard failure).",
        venv_python.display()
    );
    true
}

/// Write a minimal `chunks_manifest.json` into `out_dir` carrying
/// the given `chunk_ids` as active chunks. Used by every test
/// here so each test starts from a known good corpus state.
fn write_chunks_manifest(out_dir: &Path, chunk_ids: &[&str]) {
    let records: Vec<serde_json::Value> = chunk_ids
        .iter()
        .enumerate()
        .map(|(i, cid)| {
            let source_id = cid.split(':').next().unwrap_or("src");
            serde_json::json!({
                "schema_version": "cacg.v0",
                "source_id": source_id,
                "chunk_id": cid,
                "ordinal": i,
                "start_page": 1,
                "end_page": 1,
                "page_spans": [{"page": 1, "byte_offset_in_chunk": 0}],
                "text": format!("text for {cid}"),
                "text_preview": format!("text for {cid}"),
                "token_count": 3,
                "chunk_hash": "0".repeat(64),
            })
        })
        .collect();
    let manifest = serde_json::json!({
        "schema_version": "cacg.v0",
        "chunks": records,
        "retracted_source_ids": [],
        "retracted_chunk_ids": [],
    });
    let body = serde_json::to_string(&manifest).expect("serialize");
    fs::write(out_dir.join("chunks_manifest.json"), body).expect("write");
}

#[test]
fn kb_retract_chunk_happy_path_mutates_manifest_and_prints_summary() {
    let tmp = tempfile::tempdir().expect("tempdir");
    write_chunks_manifest(tmp.path(), &["src:p001:0000", "src:p001:0001"]);

    let output = Command::new(kb_bin())
        .args(["retract-chunk", "src:p001:0000", "--out"])
        .arg(tmp.path())
        .output()
        .expect("spawn kb");

    assert!(
        output.status.success(),
        "exit={:?} stderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("utf-8 stdout");
    assert!(
        stdout.contains("retracted chunk_id:            src:p001:0000"),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains("active chunks remaining:       1"),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains("retracted_chunk_ids total:     1"),
        "stdout: {stdout}"
    );

    // The chunks_manifest on disk must reflect the retraction.
    let manifest_bytes = fs::read(tmp.path().join("chunks_manifest.json")).unwrap();
    let manifest: serde_json::Value = serde_json::from_slice(&manifest_bytes).expect("parse");
    let chunks = manifest["chunks"].as_array().expect("chunks array");
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0]["chunk_id"], "src:p001:0001");
    assert_eq!(
        manifest["retracted_chunk_ids"],
        serde_json::json!(["src:p001:0000"])
    );
}

#[test]
fn kb_retract_chunk_missing_manifest_surfaces_cacg_cli_001() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let output = Command::new(kb_bin())
        .args(["retract-chunk", "any-chunk", "--out"])
        .arg(tmp.path())
        .output()
        .expect("spawn kb");
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Exact wire text — Python `cli.py:1228` emits
    // `f"CACG-CLI-001: {exc}"` where `exc` is the bare path-display
    // (no quotes). Regression-locks the Round-30 finding that
    // `{path:?}` produced quoted Debug output instead.
    let expected = format!(
        "CACG-CLI-001: chunks_manifest.json not found in {}; \
         run `kb ingest` before retracting a chunk\n",
        tmp.path().display()
    );
    assert_eq!(stderr, expected, "stderr was: {stderr}");
}

#[test]
fn kb_retract_chunk_unknown_chunk_id_surfaces_cacg_cli_001() {
    let tmp = tempfile::tempdir().expect("tempdir");
    write_chunks_manifest(tmp.path(), &["src:p001:0000"]);
    let output = Command::new(kb_bin())
        .args(["retract-chunk", "nope:p001:0000", "--out"])
        .arg(tmp.path())
        .output()
        .expect("spawn kb");
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Exact wire text — Python `retract.py:541-544` raises with the
    // chunk_id Python-`repr()`-quoted (single quotes), wrapped via
    // `cli.py:1228` as `CACG-CLI-001: {exc}`. The Rust dispatcher must
    // call `py_repr` (single quotes for this ASCII id) to match.
    let expected = "CACG-CLI-001: chunk 'nope:p001:0000' is not present in \
                    chunks_manifest.chunks; refusing to retract an unknown \
                    chunk\n";
    assert_eq!(stderr, expected, "stderr was: {stderr}");
}

#[test]
fn kb_retract_chunk_already_retracted_without_cascade_surfaces_cacg_cli_001() {
    let tmp = tempfile::tempdir().expect("tempdir");
    write_chunks_manifest(tmp.path(), &["src:p001:0000"]);
    // First retraction succeeds.
    let first = Command::new(kb_bin())
        .args(["retract-chunk", "src:p001:0000", "--out"])
        .arg(tmp.path())
        .output()
        .expect("spawn kb (first)");
    assert!(first.status.success(), "first run failed");

    // Second retraction (no --cards-dir) must reject as
    // already-retracted.
    let second = Command::new(kb_bin())
        .args(["retract-chunk", "src:p001:0000", "--out"])
        .arg(tmp.path())
        .output()
        .expect("spawn kb (second)");
    assert_eq!(second.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&second.stderr);
    let expected = "CACG-CLI-001: chunk 'src:p001:0000' is already retracted; \
                    retraction is append-only and not idempotent\n";
    assert_eq!(stderr, expected, "stderr was: {stderr}");
}

#[test]
fn kb_retract_chunk_preexisting_sidecar_surfaces_cacg_man_002() {
    let tmp = tempfile::tempdir().expect("tempdir");
    write_chunks_manifest(tmp.path(), &["src:p001:0000"]);
    // Drop a stale `.tmp` to trip the no-clobber preflight.
    fs::write(
        tmp.path().join("chunks_manifest.json.tmp"),
        b"stale sidecar",
    )
    .expect("write stale");

    let output = Command::new(kb_bin())
        .args(["retract-chunk", "src:p001:0000", "--out"])
        .arg(tmp.path())
        .output()
        .expect("spawn kb");
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Exact wire text — Python `retract.py:318-322` raises
    // `FileExistsError(f"CACG-MAN-002: ... sidecar(s): {[str(p) for p
    // in existing_sidecars]}; remove them and re-run")` and
    // `cli.py:1234` prints the bare exception string. The Rust
    // dispatcher must format `paths` as Python's list-of-str repr
    // (`['<p>']`) — `{paths:?}` Debug output (`[<p>]`) would diverge.
    let stale_sidecar = tmp.path().join("chunks_manifest.json.tmp");
    let expected = format!(
        "CACG-MAN-002: refusing to clobber existing chunks_manifest \
         sidecar(s): ['{}']; remove them and re-run\n",
        stale_sidecar.display()
    );
    assert_eq!(stderr, expected, "stderr was: {stderr}");
}

#[test]
fn kb_retract_chunk_output_is_byte_equal_with_python_kb_retract_chunk() {
    let venv_python = workspace_root().join("legacy_python_oracle/.venv/bin/python");
    if skip_if_python_unavailable(&venv_python) {
        return;
    }
    let tmp = tempfile::tempdir().expect("tempdir");
    let py_out = tmp.path().join("py");
    let rs_out = tmp.path().join("rs");
    fs::create_dir_all(&py_out).expect("mkdir py");
    fs::create_dir_all(&rs_out).expect("mkdir rs");
    write_chunks_manifest(
        &py_out,
        &["src:p001:0000", "src:p001:0001", "src:p001:0002"],
    );
    write_chunks_manifest(
        &rs_out,
        &["src:p001:0000", "src:p001:0001", "src:p001:0002"],
    );

    let py = Command::new(&venv_python)
        .args(["-m", "cacg.cli", "retract-chunk", "src:p001:0001", "--out"])
        .arg(&py_out)
        .env("PYTHONPATH", workspace_root().join("src"))
        .output()
        .expect("spawn python");
    assert!(
        py.status.success(),
        "python kb retract-chunk failed: stderr={}",
        String::from_utf8_lossy(&py.stderr)
    );

    let rs = Command::new(kb_bin())
        .args(["retract-chunk", "src:p001:0001", "--out"])
        .arg(&rs_out)
        .output()
        .expect("spawn kb");
    assert!(
        rs.status.success(),
        "rust kb retract-chunk failed: stderr={}",
        String::from_utf8_lossy(&rs.stderr)
    );

    let py_manifest = fs::read(py_out.join("chunks_manifest.json")).unwrap();
    let rs_manifest = fs::read(rs_out.join("chunks_manifest.json")).unwrap();
    assert_eq!(
        py_manifest,
        rs_manifest,
        "chunks_manifest bytes must be byte-equal after retract-chunk; \
         py_len={} rs_len={}",
        py_manifest.len(),
        rs_manifest.len()
    );
}

// ============================================================
// Cascade integration tests (--cards-dir / cards_manifest +
// INDEX.md pair-publish + SKILL.md skip + no-op detection).
// ============================================================

/// Write a minimal `cards_manifest.json` that the cascade can mutate.
/// Entries cite chunks defined in the corresponding `chunks_manifest`.
fn write_cards_manifest(out_dir: &Path, entries: &[(&str, &str, &str)]) {
    // entries: (card_id, reading_id, card_path_relative_to_cards_root)
    let records: Vec<serde_json::Value> = entries
        .iter()
        .map(|(card_id, reading_id, path)| {
            serde_json::json!({
                "schema_version": "cacg.v0",
                "id": card_id,
                "title": format!("Card {card_id}"),
                "reading_id": reading_id,
                "path": path,
                "summary": format!("summary for {card_id}"),
                "citation_count": 1,
                "card_hash": "0".repeat(64),
                "source_ids": ["src"],
            })
        })
        .collect();
    let manifest = serde_json::json!({
        "schema_version": "cacg.v0",
        "cards": records,
        "retracted_cards": [],
        "dependency_retracted_cards": [],
    });
    let body = serde_json::to_string(&manifest).expect("serialize");
    fs::write(out_dir.join("cards_manifest.json"), body).expect("write");
}

/// Write a card .md file with one citation pinning `chunk_id`.
/// Used to exercise the cascade walker. The summary is padded out
/// to >= 80 chars to satisfy the schema's `summary` length floor.
fn write_card_md(
    cards_dir: &Path,
    relative_path: &str,
    card_id: &str,
    reading_id: &str,
    citation_source_id: &str,
    citation_chunk_id: &str,
) {
    let abs_path = cards_dir.join(relative_path);
    if let Some(parent) = abs_path.parent() {
        fs::create_dir_all(parent).expect("mkdir parent");
    }
    let summary = format!(
        "Cascade-walker fixture for card {card_id}; the schema floor on \
         summary length is 80 chars so this string is padded."
    );
    debug_assert!(
        summary.len() >= 80,
        "summary fixture must satisfy length floor"
    );
    let body = format!(
        "---\n\
         schema_version: \"cacg.v0\"\n\
         id: \"{card_id}\"\n\
         title: \"Card {card_id}\"\n\
         reading_id: \"{reading_id}\"\n\
         summary: \"{summary}\"\n\
         citations:\n\
         \x20\x20- source_id: \"{citation_source_id}\"\n\
         \x20\x20\x20\x20chunk_id: \"{citation_chunk_id}\"\n\
         \x20\x20\x20\x20chunk_hash: \"{}\"\n\
         \x20\x20\x20\x20page_range: [1, 1]\n\
         \x20\x20\x20\x20quote: \"text for {citation_chunk_id}\"\n\
         \x20\x20\x20\x20edge_type: \"supports\"\n\
         ---\nBody for card {card_id}, padded with enough text to keep \
         the schema's body-length validator happy.\n",
        "0".repeat(64)
    );
    fs::write(&abs_path, body).expect("write card");
}

#[test]
fn kb_retract_chunk_cascade_updates_cards_manifest_and_index_md() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let out_dir = tmp.path().join("out");
    let cards_dir = tmp.path().join("cards");
    fs::create_dir_all(&out_dir).expect("mkdir out");
    fs::create_dir_all(&cards_dir).expect("mkdir cards");
    write_chunks_manifest(&out_dir, &["src:p001:0000", "src:p001:0001"]);
    // Two cards: alpha cites the chunk we'll retract; beta cites the
    // surviving chunk. Only alpha must end up in
    // dependency_retracted_cards.
    write_card_md(
        &cards_dir,
        "reading_01/alpha.md",
        "alpha",
        "reading_01",
        "src",
        "src:p001:0000",
    );
    write_card_md(
        &cards_dir,
        "reading_01/beta.md",
        "beta",
        "reading_01",
        "src",
        "src:p001:0001",
    );
    write_cards_manifest(
        &out_dir,
        &[
            ("alpha", "reading_01", "cards/reading_01/alpha.md"),
            ("beta", "reading_01", "cards/reading_01/beta.md"),
        ],
    );

    let output = Command::new(kb_bin())
        .args(["retract-chunk", "src:p001:0000", "--out"])
        .arg(&out_dir)
        .arg("--cards-dir")
        .arg(&cards_dir)
        .output()
        .expect("spawn kb");
    assert!(
        output.status.success(),
        "exit={:?} stderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    // chunks_manifest must reflect the retraction.
    let chunks_manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(out_dir.join("chunks_manifest.json")).unwrap()).unwrap();
    assert_eq!(
        chunks_manifest["retracted_chunk_ids"],
        serde_json::json!(["src:p001:0000"])
    );

    // cards_manifest must list alpha (cites the retracted chunk) in
    // dependency_retracted_cards. beta cites the surviving chunk so
    // must NOT be in the cascade.
    let cards_manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(out_dir.join("cards_manifest.json")).unwrap()).unwrap();
    assert_eq!(
        cards_manifest["dependency_retracted_cards"],
        serde_json::json!(["alpha"])
    );

    // INDEX.md MUST be regenerated. Round-30 P1-3 lock: previous
    // implementation never touched INDEX.md.
    let index_md = fs::read_to_string(out_dir.join("INDEX.md"))
        .expect("INDEX.md must be regenerated on cascade");
    assert!(index_md.contains("# Card Index"));
    assert!(index_md.contains("schema_version: cacg.v0"));
    assert!(index_md.contains("## reading_01"));
}

#[test]
fn kb_retract_chunk_cascade_skips_skill_md_routers() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let out_dir = tmp.path().join("out");
    let cards_dir = tmp.path().join("cards");
    fs::create_dir_all(&out_dir).expect("mkdir out");
    fs::create_dir_all(&cards_dir).expect("mkdir cards");
    write_chunks_manifest(&out_dir, &["src:p001:0000", "src:p001:0001"]);
    // A SKILL.md router that ALSO cites the retracted chunk. Python's
    // _compute_cascade_for_chunks calls is_skill_router_path to skip
    // these (retract.py:90). Regression-lock for Round-30 P1-1.
    write_card_md(
        &cards_dir,
        "reading_01/SKILL.md",
        "skill-router-01",
        "reading_01",
        "src",
        "src:p001:0000",
    );
    write_card_md(
        &cards_dir,
        "reading_01/alpha.md",
        "alpha",
        "reading_01",
        "src",
        "src:p001:0000",
    );
    write_cards_manifest(
        &out_dir,
        &[("alpha", "reading_01", "cards/reading_01/alpha.md")],
    );

    let output = Command::new(kb_bin())
        .args(["retract-chunk", "src:p001:0000", "--out"])
        .arg(&out_dir)
        .arg("--cards-dir")
        .arg(&cards_dir)
        .output()
        .expect("spawn kb");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let cards_manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(out_dir.join("cards_manifest.json")).unwrap()).unwrap();
    // Only alpha (NOT skill-router-01) must be in the cascade.
    assert_eq!(
        cards_manifest["dependency_retracted_cards"],
        serde_json::json!(["alpha"]),
        "SKILL.md routers must be skipped by the cascade walker"
    );
}

#[test]
fn kb_retract_chunk_already_retracted_no_op_cascade_surfaces_cacg_cli_001() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let out_dir = tmp.path().join("out");
    let cards_dir = tmp.path().join("cards");
    fs::create_dir_all(&out_dir).expect("mkdir out");
    fs::create_dir_all(&cards_dir).expect("mkdir cards");
    write_chunks_manifest(&out_dir, &["src:p001:0000", "src:p001:0001"]);
    write_card_md(
        &cards_dir,
        "reading_01/alpha.md",
        "alpha",
        "reading_01",
        "src",
        "src:p001:0000",
    );
    write_cards_manifest(
        &out_dir,
        &[("alpha", "reading_01", "cards/reading_01/alpha.md")],
    );

    // First run: chunks_manifest + cards_manifest cascade both update.
    let first = Command::new(kb_bin())
        .args(["retract-chunk", "src:p001:0000", "--out"])
        .arg(&out_dir)
        .arg("--cards-dir")
        .arg(&cards_dir)
        .output()
        .expect("spawn kb (first)");
    assert!(
        first.status.success(),
        "first run failed: {}",
        String::from_utf8_lossy(&first.stderr)
    );

    // Second run: chunks-side is already retracted AND cards-side is
    // already up-to-date. Python `retract.py:608-611` raises with the
    // "already retracted and the dependency cascade is already
    // up-to-date" wire text. Regression-lock for Round-30 P1-4.
    let second = Command::new(kb_bin())
        .args(["retract-chunk", "src:p001:0000", "--out"])
        .arg(&out_dir)
        .arg("--cards-dir")
        .arg(&cards_dir)
        .output()
        .expect("spawn kb (second)");
    assert_eq!(second.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&second.stderr);
    let expected = "CACG-CLI-001: chunk 'src:p001:0000' is already retracted \
                    and the dependency cascade is already up-to-date\n";
    assert_eq!(stderr, expected, "stderr was: {stderr}");
}

#[test]
fn kb_retract_chunk_cascade_byte_equal_with_python_kb_retract_chunk() {
    let venv_python = workspace_root().join("legacy_python_oracle/.venv/bin/python");
    if skip_if_python_unavailable(&venv_python) {
        return;
    }
    let tmp = tempfile::tempdir().expect("tempdir");
    let py_out = tmp.path().join("py-out");
    let rs_out = tmp.path().join("rs-out");
    let py_cards = tmp.path().join("py-cards");
    let rs_cards = tmp.path().join("rs-cards");
    for d in [&py_out, &rs_out, &py_cards, &rs_cards] {
        fs::create_dir_all(d).expect("mkdir");
    }
    for (out_dir, cards_dir) in [(&py_out, &py_cards), (&rs_out, &rs_cards)] {
        write_chunks_manifest(out_dir, &["src:p001:0000", "src:p001:0001"]);
        write_card_md(
            cards_dir,
            "reading_01/alpha.md",
            "alpha",
            "reading_01",
            "src",
            "src:p001:0000",
        );
        write_card_md(
            cards_dir,
            "reading_01/beta.md",
            "beta",
            "reading_01",
            "src",
            "src:p001:0001",
        );
        // SKILL.md router that cites the retracted chunk; both Python
        // and Rust must skip it.
        write_card_md(
            cards_dir,
            "reading_01/SKILL.md",
            "skill-router-01",
            "reading_01",
            "src",
            "src:p001:0000",
        );
        write_cards_manifest(
            out_dir,
            &[
                ("alpha", "reading_01", "cards/reading_01/alpha.md"),
                ("beta", "reading_01", "cards/reading_01/beta.md"),
            ],
        );
    }

    let py = Command::new(&venv_python)
        .args(["-m", "cacg.cli", "retract-chunk", "src:p001:0000", "--out"])
        .arg(&py_out)
        .arg("--cards-dir")
        .arg(&py_cards)
        .env("PYTHONPATH", workspace_root().join("src"))
        .output()
        .expect("spawn python");
    assert!(
        py.status.success(),
        "python kb retract-chunk --cards-dir failed: stderr={}",
        String::from_utf8_lossy(&py.stderr)
    );

    let rs = Command::new(kb_bin())
        .args(["retract-chunk", "src:p001:0000", "--out"])
        .arg(&rs_out)
        .arg("--cards-dir")
        .arg(&rs_cards)
        .output()
        .expect("spawn kb");
    assert!(
        rs.status.success(),
        "rust kb retract-chunk --cards-dir failed: stderr={}",
        String::from_utf8_lossy(&rs.stderr)
    );

    let py_chunks = fs::read(py_out.join("chunks_manifest.json")).unwrap();
    let rs_chunks = fs::read(rs_out.join("chunks_manifest.json")).unwrap();
    assert_eq!(py_chunks, rs_chunks, "chunks_manifest bytes diverge");

    let py_cards_m = fs::read(py_out.join("cards_manifest.json")).unwrap();
    let rs_cards_m = fs::read(rs_out.join("cards_manifest.json")).unwrap();
    assert_eq!(
        py_cards_m,
        rs_cards_m,
        "cards_manifest bytes diverge after cascade; \
         py_len={} rs_len={}",
        py_cards_m.len(),
        rs_cards_m.len()
    );

    let py_index = fs::read(py_out.join("INDEX.md")).unwrap();
    let rs_index = fs::read(rs_out.join("INDEX.md")).unwrap();
    assert_eq!(
        py_index,
        rs_index,
        "INDEX.md bytes diverge after cascade; \
         py_len={} rs_len={}",
        py_index.len(),
        rs_index.len()
    );
}
