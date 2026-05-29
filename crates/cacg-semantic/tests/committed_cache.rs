#![allow(clippy::unwrap_used)]
//! Integration test against the committed B1 semantic cache.
//!
//! Asserts that `out/semantic_cache.json` at the workspace root:
//!
//! 1. Loads cleanly through `cacg_semantic::SemanticCache::load`,
//!    proving the full Pydantic-ported validator stack
//!    (`cacg.v0` schema, 64-hex hash format, score range,
//!    `(chunk_hash, claim_window_hash)` uniqueness) accepts the
//!    committed bytes.
//! 2. Carries exactly 227 entries (222 paraphrase + 5 negative)
//!    per the frozen-count contract enforced by both the
//!    builder and the `xtask audit-semantic-cache-provenance`
//!    command.
//! 3. Round-trips byte-for-byte through `to_canonical_json()`:
//!    the canonical serializer emits the same bytes the
//!    committed file holds (modulo the trailing newline the
//!    builder appends).
//! 4. `lookup()` returns the recorded verdict for at least one
//!    `pass` entry and one `verdict == "fail" && score == 0.0`
//!    negative-fixture entry — the runtime read path actually
//!    finds the committed entries by key.

use std::path::{Path, PathBuf};

use cacg_semantic::{SemanticCache, SemanticMode, SemanticVerdictKind};

fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}

fn committed_cache_path() -> PathBuf {
    workspace_root().join("out/semantic_cache.json")
}

#[test]
fn committed_cache_loads_cleanly() {
    let cache = SemanticCache::load(&committed_cache_path()).expect("committed cache must load");
    assert_eq!(cache.schema_version, "cacg.v0");
    assert_eq!(
        cache.entries.len(),
        227,
        "committed cache must hold exactly 227 entries (222 paraphrase + 5 negative)",
    );
}

#[test]
fn committed_cache_round_trips_canonical_json_byte_equal() {
    let path = committed_cache_path();
    let committed_bytes = std::fs::read(&path).expect("read committed cache");
    let cache = SemanticCache::load(&path).expect("committed cache must load");
    let rendered = cache
        .to_canonical_json()
        .expect("to_canonical_json must succeed on a loaded cache");
    let rendered_bytes_with_newline = {
        let mut s = rendered.into_bytes();
        s.push(b'\n');
        s
    };
    assert_eq!(
        rendered_bytes_with_newline, committed_bytes,
        "canonical-serializer round-trip must match the committed bytes byte-for-byte",
    );
}

#[test]
fn committed_cache_lookup_returns_pass_for_known_pass_entry() {
    let cache = SemanticCache::load(&committed_cache_path()).expect("committed cache must load");
    let pass_entry = cache
        .entries
        .iter()
        .find(|e| matches!(e.verdict, SemanticVerdictKind::Pass))
        .expect("committed cache must include at least one pass entry");
    let verdict = cache.lookup(&pass_entry.chunk_hash, &pass_entry.claim_window_hash);
    assert!(
        matches!(verdict.kind, SemanticVerdictKind::Pass),
        "lookup returned non-pass verdict for a known pass entry: kind={:?}",
        verdict.kind,
    );
    assert!(
        matches!(verdict.mode, SemanticMode::EmbeddingCache),
        "lookup mode must be EmbeddingCache; got {:?}",
        verdict.mode,
    );
}

#[test]
fn committed_cache_lookup_returns_fail_for_negative_fixture_entry() {
    // Negative fixtures are pinned by the builder with
    // `verdict=fail` AND `score=0.0`; pick the first matching
    // entry instead of relying on a hardcoded key (the
    // builder regenerates keys deterministically but a future
    // negative-fixture edit would shift them).
    let cache = SemanticCache::load(&committed_cache_path()).expect("committed cache must load");
    let negative_entry = cache
        .entries
        .iter()
        .find(|e| matches!(e.verdict, SemanticVerdictKind::Fail) && e.score == 0.0)
        .expect(
            "committed cache must include at least one negative fixture (verdict=fail, score=0.0)",
        );
    let verdict = cache.lookup(
        &negative_entry.chunk_hash,
        &negative_entry.claim_window_hash,
    );
    assert!(
        matches!(verdict.kind, SemanticVerdictKind::Fail),
        "lookup returned non-fail verdict for a known negative-fixture entry: kind={:?}",
        verdict.kind,
    );
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(verdict.score, 0.0);
    }
}

#[test]
fn committed_cache_path_exists_at_documented_location() {
    // Belt-and-braces: assert the committed cache lives at the
    // documented `out/semantic_cache.json` path at the workspace
    // root (the rebuild ceremony's canonical output location).
    let p: &Path = &committed_cache_path();
    assert!(
        p.is_file(),
        "committed cache must exist at {} (the documented \
         rebuild-ceremony output path)",
        p.display(),
    );
}
