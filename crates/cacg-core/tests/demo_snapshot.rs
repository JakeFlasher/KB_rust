#![allow(clippy::unwrap_used)]
//! Snapshot-stable proof that `cacg_core::demo::demo_run` produces
//! byte-identical output across runs against the committed golden card.
//!
//! Snapshots live under `crates/cacg-core/tests/snapshots/` (committed
//! to git so a CI run on a fresh checkout reproduces the comparison).

use std::path::PathBuf;

use cacg_core::demo::demo_run;
use cacg_core::determinism::DeterminismContext;
use tempfile::TempDir;

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

#[test]
fn demo_run_against_golden_card() {
    let card = workspace_root().join("tests/golden/01-content-addressable-identity.md");
    assert!(card.is_file(), "golden card must exist at {card:?}");
    let ctx = DeterminismContext::frozen();
    let tempdir = TempDir::new().expect("tempdir");
    let report = demo_run(&card, &ctx, tempdir.path()).expect("demo_run ok");
    insta::assert_snapshot!(report);
}
