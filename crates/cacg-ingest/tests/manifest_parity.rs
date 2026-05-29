#![allow(clippy::unwrap_used)]
//! End-to-end manifest-builder + pair-atomic publish parity test.
//!
//! Extract `cfa_vol1_trim.pdf` via `cacg_ingest::extract_pages`, run
//! the chunker on the extracted page text, build both manifests via
//! `cacg_ingest::manifest::build_manifests` under a frozen
//! `DeterminismContext`, pair-atomically publish into a tempdir via
//! `publish_manifests`, then read both files back from disk and
//! compare them to the committed Python oracle at
//! `tests/parity_corpus/out_python/pdfs/cfa_vol1_trim/`.
//!
//! Comparison rules:
//! - `chunks_manifest.json`: full byte-equality with the oracle (the
//!   chunker is byte-equal per `chunker_parity.rs`, and the
//!   `ChunksManifest` envelope here adds only empty `retracted_*_ids`
//!   plus the `schema_version`, which the Python oracle also emits).
//! - `sources_manifest.json`: byte-equality EXCEPT for
//!   `parser_name` and `parser_version`, which are the DEC-2
//!   declared divergence (Rust records its true `pdfium-render`
//!   identity, Python records `pypdfium2`).
//!
//! Requires the `ingest` Cargo feature and a discoverable
//! `libpdfium` (see `docs/pdfium-binary-provisioning.md`). Skips
//! cleanly when libpdfium is absent; honors `CACG_REQUIRE_PDFIUM=1`
//! as a hard-failure gate.

#![cfg(feature = "ingest")]

use std::path::{Path, PathBuf};

use cacg_core::determinism::DeterminismContext;
use cacg_core::schema::SourcesManifest;
use cacg_ingest::extract_pages;
use cacg_ingest::manifest::{build_manifests, publish_manifests};
use pdfium_render::prelude::Pdfium;

fn pdfium_available() -> bool {
    Pdfium::bind_to_system_library().is_ok()
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn fixture_pdf() -> PathBuf {
    workspace_root()
        .join("tests")
        .join("parity_corpus")
        .join("pdfs")
        .join("cfa_vol1_trim.pdf")
}

fn oracle_dir() -> PathBuf {
    workspace_root()
        .join("tests")
        .join("parity_corpus")
        .join("out_python")
        .join("pdfs")
        .join("cfa_vol1_trim")
}

fn skip_if_pdfium_unavailable() -> bool {
    if pdfium_available() {
        return false;
    }
    assert!(
        !(std::env::var("CACG_REQUIRE_PDFIUM").as_deref() == Ok("1")),
        "CACG_REQUIRE_PDFIUM=1 set but libpdfium is not available; \
             see docs/pdfium-binary-provisioning.md for setup."
    );
    eprintln!(
        "skipping: libpdfium not available on this runner; \
         see docs/pdfium-binary-provisioning.md \
         (set CACG_REQUIRE_PDFIUM=1 to make this a hard failure)."
    );
    true
}

fn skip_if_path_missing(p: &Path) -> bool {
    if p.exists() {
        return false;
    }
    eprintln!("skipping: {} not found", p.display());
    true
}

#[test]
fn manifests_byte_equal_with_python_oracle_for_cfa_vol1_trim() {
    if skip_if_pdfium_unavailable() {
        return;
    }
    let pdf = fixture_pdf();
    let oracle_sources = oracle_dir().join("sources_manifest.json");
    let oracle_chunks = oracle_dir().join("chunks_manifest.json");
    if skip_if_path_missing(&pdf)
        || skip_if_path_missing(&oracle_sources)
        || skip_if_path_missing(&oracle_chunks)
    {
        return;
    }

    let pdf_bytes = std::fs::read(&pdf).expect("PDF fixture read must succeed");
    let pages_owned = extract_pages(&pdf_bytes).expect("pdfium extraction must succeed");
    let pages: Vec<(u32, &str)> = pages_owned
        .iter()
        .enumerate()
        .map(|(idx, text)| {
            let page_num = u32::try_from(idx + 1).expect("page index fits in u32");
            (page_num, text.as_str())
        })
        .collect();

    let det = DeterminismContext::frozen();
    // source_path mirrors the relative path the Python oracle records.
    let source_path = "tests/parity_corpus/pdfs/cfa_vol1_trim.pdf";
    let output = build_manifests("cfa_vol1_trim", source_path, &pdf_bytes, &pages, &det)
        .expect("manifest builder must succeed");

    let tmp = tempfile::tempdir().expect("tempdir");
    publish_manifests(tmp.path(), &output).expect("pair-atomic publish must succeed");

    let rust_chunks_bytes =
        std::fs::read(tmp.path().join("chunks_manifest.json")).expect("read published chunks");
    let oracle_chunks_bytes = std::fs::read(&oracle_chunks).expect("read oracle chunks");

    assert_eq!(
        rust_chunks_bytes, oracle_chunks_bytes,
        "chunks_manifest.json must be byte-equal with the Python oracle"
    );

    // sources_manifest.json: parser_name + parser_version are a
    // declared divergence (DEC-2); compare every other field.
    let rust_sources_bytes =
        std::fs::read(tmp.path().join("sources_manifest.json")).expect("read published sources");
    let rust_sources: SourcesManifest =
        serde_json::from_slice(&rust_sources_bytes).expect("rust sources parse");
    let oracle_sources_bytes = std::fs::read(&oracle_sources).expect("read oracle sources");
    let oracle_sources_parsed: SourcesManifest =
        serde_json::from_slice(&oracle_sources_bytes).expect("oracle sources parse");

    assert_eq!(
        rust_sources.sources.len(),
        oracle_sources_parsed.sources.len(),
        "sources count must match"
    );
    for (rust_s, py_s) in rust_sources
        .sources
        .iter()
        .zip(oracle_sources_parsed.sources.iter())
    {
        assert_eq!(rust_s.source_id, py_s.source_id, "source_id");
        assert_eq!(rust_s.source_path, py_s.source_path, "source_path");
        assert_eq!(rust_s.source_sha256, py_s.source_sha256, "source_sha256");
        assert_eq!(rust_s.page_count, py_s.page_count, "page_count");
        assert_eq!(rust_s.extracted_at, py_s.extracted_at, "extracted_at");
        // parser_name + parser_version: declared divergence per DEC-2.
        // Just sanity-check the Rust identity is what we expect.
        assert_eq!(rust_s.parser_name, "pdfium-render");
        assert_eq!(rust_s.parser_version, "0.9.1");
        // And the Python oracle still records its own (so we don't
        // accidentally regress the test fixture):
        assert_eq!(py_s.parser_name, "pypdfium2");
        assert!(
            py_s.parser_version.starts_with("5.8.0"),
            "oracle parser_version: {}",
            py_s.parser_version
        );
    }
}
