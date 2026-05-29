#![allow(clippy::unwrap_used)]
//! AC-5 Pdfium parity checkpoint: locks the BYTE-EQUAL outcome.
//!
//! Spawns the built `kb ingest` against the committed
//! `tests/parity_corpus/pdfs/cfa_vol1_trim.pdf` fixture under
//! `KB_FROZEN_CLOCK=1` and compares the published manifest pair to
//! the committed Python oracle at
//! `tests/parity_corpus/out_python/pdfs/cfa_vol1_trim/`.
//!
//! Per AC-5 BYTE-EQUAL:
//!   - `chunks_manifest.json` is byte-for-byte identical.
//!   - `sources_manifest.json` is byte-for-byte identical EXCEPT for
//!     the two DEC-2-whitelisted fields: `parser_name`
//!     (`"pdfium-render"` vs Python's `"pypdfium2"`) and
//!     `parser_version` (the workspace `pdfium-render` pin vs
//!     pypdfium2's `<pyv>+pdfium<bin>` string). The whitelist is
//!     applied symmetrically by stripping both fields from each
//!     side, re-canonicalizing via
//!     `cacg_core::canonical_json::canonical_json`, and asserting
//!     RAW BYTE equality of the result. Plain `serde_json::Value`
//!     equality would be strictly weaker (a future ingest path
//!     emitting `30.0` instead of `30`, `A` instead of `A`, or
//!     a trailing newline would round-trip-equal but fail the AC-5
//!     byte-for-byte contract); the canonicalize-then-bytes pattern
//!     enforces the real AC.
//!
//! Skips when libpdfium is unavailable (same convention as
//! `kb_ingest.rs`). Set `CACG_REQUIRE_PDFIUM=1` to escalate a
//! missing libpdfium into a hard failure.
//!
//! ## Test isolation: pdfium-render single-bind constraint
//!
//! pdfium-render's `Pdfium::bind_to_system_library()` may only be
//! called once per process; a second call asserts and the test
//! harness SEGVs. The cacg-ingest workspace has at least four
//! pdfium-bound test files now
//! (`extract_pages_pdfium.rs`, `chunker_parity.rs`,
//! `manifest_parity.rs`, plus this one); running them in one
//! `cargo test` invocation requires `--test-threads=1` so each
//! test file gets its own process. A future tightening (a once-cell
//! binding shared across tests, or a `serial_test`-style mutex) is
//! the proper fix but is out of scope for AC-5.

#![cfg(feature = "ingest")]

use std::path::PathBuf;
use std::process::Command;

use cacg_core::parity::canonicalize_sources_minus_dec2_whitelist;
use pdfium_render::prelude::Pdfium;
use serde_json::Value;

fn kb_bin() -> &'static str {
    env!("CARGO_BIN_EXE_kb")
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn pdfium_available() -> bool {
    Pdfium::bind_to_system_library().is_ok()
}

fn skip_if_pdfium_unavailable() -> bool {
    if pdfium_available() {
        return false;
    }
    assert!(
        !(std::env::var("CACG_REQUIRE_PDFIUM").as_deref() == Ok("1")),
        "CACG_REQUIRE_PDFIUM=1 set but libpdfium is not available; \
             see docs/pdfium-binary-provisioning.md."
    );
    eprintln!(
        "skipping: libpdfium not available on this runner; \
         see docs/pdfium-binary-provisioning.md \
         (set CACG_REQUIRE_PDFIUM=1 to make this a hard failure)."
    );
    true
}

/// AC-5 BYTE-EQUAL gate. Locks the Round-9 finding: with
/// `extract_pages_impl` using `chars().iter()` + the
/// soft-hyphen STX→U+FFFE mapping, Rust `kb ingest` reproduces
/// the committed Python oracle byte-for-byte.
#[test]
fn kb_ingest_cfa_vol1_trim_is_byte_equal_with_committed_python_oracle() {
    if skip_if_pdfium_unavailable() {
        return;
    }
    let root = workspace_root();
    let pdf = root
        .join("tests")
        .join("parity_corpus")
        .join("pdfs")
        .join("cfa_vol1_trim.pdf");
    if !pdf.is_file() {
        eprintln!("skipping: fixture {} not found", pdf.display());
        return;
    }
    let oracle_dir = root
        .join("tests")
        .join("parity_corpus")
        .join("out_python")
        .join("pdfs")
        .join("cfa_vol1_trim");
    let oracle_chunks = oracle_dir.join("chunks_manifest.json");
    let oracle_sources = oracle_dir.join("sources_manifest.json");
    if !oracle_chunks.is_file() || !oracle_sources.is_file() {
        eprintln!(
            "skipping: committed Python oracle missing at {}",
            oracle_dir.display()
        );
        return;
    }

    let out_dir = tempfile::tempdir().expect("tempdir");

    // The committed Python oracle was produced with the
    // workspace-relative path `tests/parity_corpus/pdfs/cfa_vol1_trim.pdf`
    // as the recorded `source_path`. The Rust binary records the
    // path it is invoked with verbatim, so we MUST invoke it from
    // the workspace root with the same relative path or
    // `source_path` will diverge from the oracle.
    let status = Command::new(kb_bin())
        .current_dir(&root)
        .env("KB_FROZEN_CLOCK", "1")
        .arg("ingest")
        .arg("tests/parity_corpus/pdfs/cfa_vol1_trim.pdf")
        .arg("--out")
        .arg(out_dir.path())
        .arg("--source-id")
        .arg("cfa_vol1_trim")
        .output()
        .expect("spawn kb");
    assert!(
        status.status.success(),
        "kb ingest failed: exit={:?} stderr={}",
        status.status.code(),
        String::from_utf8_lossy(&status.stderr),
    );

    let oracle_chunks_bytes = std::fs::read(&oracle_chunks).expect("read committed chunks oracle");
    let rust_chunks_bytes = std::fs::read(out_dir.path().join("chunks_manifest.json"))
        .expect("read Rust chunks manifest");

    assert_eq!(
        oracle_chunks_bytes,
        rust_chunks_bytes,
        "AC-5: chunks_manifest.json must be byte-equal with the \
         committed Python oracle. Lengths py={} rs={}.",
        oracle_chunks_bytes.len(),
        rust_chunks_bytes.len(),
    );

    // sources_manifest.json: strip the two DEC-2 whitelisted fields
    // (parser_name, parser_version) from each side, re-canonicalize
    // via cacg_core::canonical_json, and assert RAW BYTE equality on
    // the result. This is the AC-5 contract — strictly stronger than
    // a `serde_json::Value` field-by-field comparison, which would
    // silently pass a future `30` vs `30.0` numeric drift, an
    // ASCII-as-`A` re-escape, or a trailing-whitespace
    // regression.
    let oracle_bytes = std::fs::read(&oracle_sources).expect("read oracle sources");
    let rust_bytes =
        std::fs::read(out_dir.path().join("sources_manifest.json")).expect("read Rust sources");
    let oracle_stripped = canonicalize_dec2_whitelist(&oracle_bytes, "oracle");
    let rust_stripped = canonicalize_dec2_whitelist(&rust_bytes, "rust");
    assert_eq!(
        oracle_stripped, rust_stripped,
        "AC-5: sources_manifest.json must be byte-equal with the \
         committed Python oracle on every field except the DEC-2 \
         whitelist (parser_name, parser_version).",
    );

    // Lock the Rust identity strings so a silent pdfium-render
    // workspace pin bump surfaces here, not only in the
    // manifest-version-pin test inside cacg-ingest. We need the
    // parsed form to assert this, so re-parse on this side only.
    let rust_v: Value = serde_json::from_slice(&rust_bytes).expect("parse Rust sources");
    let rust_record = &rust_v["sources"][0];
    assert_eq!(
        rust_record["parser_name"],
        Value::String("pdfium-render".into()),
        "Rust parser_name must be 'pdfium-render' (DEC-2 declared identity)",
    );
    assert!(
        rust_record["parser_version"]
            .as_str()
            .is_some_and(|s| !s.is_empty()),
        "Rust parser_version must be a non-empty string",
    );
}

/// Strip the DEC-2 whitelist via the shared `cacg_core::parity`
/// helper (the single source of truth for the AC-5 BYTE-EQUAL
/// sources contract — also used by the xtask matrix's
/// `MatrixRowKind::KbIngest` row body). `label` appears in the
/// panic messages so an oracle vs Rust parse failure is locally
/// diagnosable.
fn canonicalize_dec2_whitelist(bytes: &[u8], label: &str) -> Vec<u8> {
    canonicalize_sources_minus_dec2_whitelist(bytes)
        .unwrap_or_else(|e| panic!("{label}: DEC-2 whitelist strip failed: {e}"))
}
