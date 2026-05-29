#![allow(clippy::unwrap_used)]
//! Integration coverage for the `kb show` dispatcher's `CACG-SHOW-003`
//! `--path` hardening.
//!
//! `kb show --path` rejects a `..`-traversal or absolute path with
//! `CACG-SHOW-003` before any filesystem read. Python `_cmd_show`
//! performs the identical check, so the behavior is byte-equal across
//! both implementations. These tests pin the Rust dispatcher's
//! behavior directly.
//!
//! Every full `kb show` surface (active / retracted / unauthorized /
//! `--path` match + mismatch / missing-card) is byte-parity-tested by
//! the `kb_show_parity` matrix row; this file is scoped to the
//! `CACG-SHOW-003` hardening only.

use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

/// Run `kb show <card_id> --path <path>` over the committed CFA
/// first-bite corpus, with `cwd` = workspace root so the
/// workspace-relative manifest / matrix / `--path` arguments resolve.
fn kb_show(card_id: &str, path: &str) -> std::process::Output {
    Command::new(kb_bin())
        .current_dir(workspace_root())
        .arg("show")
        .arg(card_id)
        .arg("--cards-manifest")
        .arg("tests/parity_corpus/cfa_first_bite/cards_manifest.json")
        .arg("--source-matrix")
        .arg("tests/parity_corpus/cfa_first_bite/source_matrix.json")
        .arg("--path")
        .arg(path)
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb show")
}

#[test]
fn kb_show_path_traversal_rejected_with_cacg_show_003() {
    let out = kb_show("intrinsic-valuation-discounted-cash-flows", "../evil.md");
    assert_eq!(
        out.status.code(),
        Some(1),
        "a `..`-traversal --path must exit 1"
    );
    assert!(
        out.stdout.is_empty(),
        "a rejected --path must write nothing to stdout; got {:?}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("CACG-SHOW-003"),
        "a `..`-traversal --path must be rejected with CACG-SHOW-003; got: {stderr}"
    );
}

#[test]
fn kb_show_absolute_path_rejected_with_cacg_show_003() {
    let out = kb_show(
        "intrinsic-valuation-discounted-cash-flows",
        "/tmp/evil-absolute.md",
    );
    assert_eq!(out.status.code(), Some(1), "an absolute --path must exit 1");
    assert!(
        out.stdout.is_empty(),
        "a rejected --path must write nothing to stdout; got {:?}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("CACG-SHOW-003"),
        "an absolute --path must be rejected with CACG-SHOW-003; got: {stderr}"
    );
}

#[test]
fn kb_show_normal_relative_path_proceeds_past_show_003() {
    // A `..`-free relative `--path` is NOT a CACG-SHOW-003 case: it
    // proceeds past the traversal guard to the id / card_hash
    // cross-check and, pointed at the card's own file, succeeds.
    let out = kb_show(
        "intrinsic-valuation-discounted-cash-flows",
        "tests/parity_corpus/cfa_first_bite/cards/05_Equity/intrinsic-valuation-discounted-cash-flows.md",
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("CACG-SHOW-003"),
        "a normal relative --path must NOT trigger CACG-SHOW-003; got: {stderr}"
    );
    assert_eq!(
        out.status.code(),
        Some(0),
        "a --path pointing at the card's own file must succeed; stderr={stderr}"
    );
}

/// Run `kb show` over a `cards_manifest.json` whose bytes are
/// `manifest_json` (written to a fresh tempdir). The `card_id` is
/// irrelevant — `CardsManifest` structural validation runs before
/// resolution. `cwd` = workspace root so `--source-matrix` resolves.
fn kb_show_over_manifest(manifest_json: &str) -> std::process::Output {
    let tmp = tempfile::tempdir().expect("tempdir");
    let manifest = tmp.path().join("cards_manifest.json");
    std::fs::write(&manifest, manifest_json).expect("write cards_manifest.json");
    Command::new(kb_bin())
        .current_dir(workspace_root())
        .arg("show")
        .arg("any-card-id")
        .arg("--cards-manifest")
        .arg(&manifest)
        .arg("--source-matrix")
        .arg("tests/parity_corpus/kb_show/source_matrix.json")
        .env("KB_FROZEN_CLOCK", "1")
        .output()
        .expect("spawn kb show")
}

#[test]
fn kb_show_duplicate_active_card_id_manifest_is_cacg_man_001() {
    // A `cards_manifest.json` with a duplicate active `cards[*].id` is
    // a `CardsManifest` invariant violation. Python rejects it at the
    // `CardsManifest` load boundary with `CACG-MAN-001`; Rust must too,
    // before any card resolution.
    let manifest = r#"{"schema_version":"cacg.v0","cards":[{"schema_version":"cacg.v0","path":"cards/a.md","id":"dup-card","title":"T","reading_id":"reading_01","summary":"S","card_hash":"0000000000000000000000000000000000000000000000000000000000000000","citation_count":0,"source_ids":[]},{"schema_version":"cacg.v0","path":"cards/b.md","id":"dup-card","title":"T2","reading_id":"reading_01","summary":"S2","card_hash":"0000000000000000000000000000000000000000000000000000000000000000","citation_count":0,"source_ids":[]}],"retracted_cards":[],"dependency_retracted_cards":[]}"#;
    let out = kb_show_over_manifest(manifest);
    assert_eq!(
        out.status.code(),
        Some(1),
        "a duplicate-active-id manifest must exit 1"
    );
    assert!(
        out.stdout.is_empty(),
        "an invalid manifest must write nothing to stdout; got {:?}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("CACG-MAN-001"),
        "a duplicate-active-id manifest must be rejected with CACG-MAN-001; got: {stderr}"
    );
}

#[test]
fn kb_show_active_retracted_overlap_manifest_is_cacg_man_001() {
    // A card_id in BOTH `cards` and `retracted_cards` violates the
    // `CardsManifest` retraction-disjointness invariant. `kb show` must
    // reject the manifest with `CACG-MAN-001` and write NOTHING to
    // stdout — the validation gate runs before the retraction gate, so
    // an invalid manifest cannot leak retraction state (a `STATUS:`
    // line or `CACG-SHOW-001`).
    let manifest = r#"{"schema_version":"cacg.v0","cards":[{"schema_version":"cacg.v0","path":"cards/x.md","id":"overlap-card","title":"T","reading_id":"reading_01","summary":"S","card_hash":"0000000000000000000000000000000000000000000000000000000000000000","citation_count":0,"source_ids":[]}],"retracted_cards":["overlap-card"],"dependency_retracted_cards":[]}"#;
    let out = kb_show_over_manifest(manifest);
    assert_eq!(
        out.status.code(),
        Some(1),
        "an active/retracted-overlap manifest must exit 1"
    );
    assert!(
        out.stdout.is_empty(),
        "an invalid manifest must not leak retraction state to stdout; got {:?}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("CACG-MAN-001"),
        "an active/retracted-overlap manifest must be rejected with CACG-MAN-001; got: {stderr}"
    );
}
