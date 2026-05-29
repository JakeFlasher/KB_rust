#![allow(clippy::unwrap_used)]
//! End-to-end chunker byte-parity test: extract `cfa_vol1_trim.pdf` via
//! `cacg_ingest::extract_pages`, run the Rust chunker on the extracted
//! pages, and assert the resulting `ChunkRecord` list is byte-equal
//! field-for-field with the committed Python oracle at
//! `tests/parity_corpus/out_python/pdfs/cfa_vol1_trim/chunks_manifest.json`.
//!
//! Requires the `ingest` Cargo feature and a discoverable `libpdfium`
//! (see `docs/pdfium-binary-provisioning.md`). When `libpdfium` is
//! absent the test skips with a notice, matching the ergonomics of
//! the existing `extract_pages_pdfium.rs` fixture tests. Set
//! `CACG_REQUIRE_PDFIUM=1` to make a missing `libpdfium` a hard
//! failure.

#![cfg(feature = "ingest")]

use std::path::{Path, PathBuf};

use cacg_core::schema::ChunksManifest;
use cacg_ingest::chunker::{chunk_pages, ChunkConfig};
use cacg_ingest::extract_pages;
use pdfium_render::prelude::Pdfium;

fn pdfium_available() -> bool {
    Pdfium::bind_to_system_library().is_ok()
}

fn parity_pdfs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("tests")
        .join("parity_corpus")
        .join("pdfs")
}

fn oracle_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
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
fn chunker_byte_equal_with_python_oracle_for_cfa_vol1_trim() {
    if skip_if_pdfium_unavailable() {
        return;
    }
    let pdf = parity_pdfs_dir().join("cfa_vol1_trim.pdf");
    let oracle = oracle_dir().join("chunks_manifest.json");
    if skip_if_path_missing(&pdf) || skip_if_path_missing(&oracle) {
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

    let rust_chunks = chunk_pages("cfa_vol1_trim", &pages, &ChunkConfig::default())
        .expect("chunker must succeed on a valid extraction");

    let oracle_bytes = std::fs::read(&oracle).expect("oracle manifest read must succeed");
    let oracle_manifest: ChunksManifest =
        serde_json::from_slice(&oracle_bytes).expect("oracle manifest must parse");
    let py_chunks = oracle_manifest.chunks;

    assert_eq!(
        rust_chunks.len(),
        py_chunks.len(),
        "chunk count: rust={} vs python={}",
        rust_chunks.len(),
        py_chunks.len()
    );

    for (i, (rust, py)) in rust_chunks.iter().zip(py_chunks.iter()).enumerate() {
        assert_eq!(rust.source_id, py.source_id, "chunk[{i}] source_id");
        assert_eq!(rust.chunk_id, py.chunk_id, "chunk[{i}] chunk_id");
        assert_eq!(rust.ordinal, py.ordinal, "chunk[{i}] ordinal");
        assert_eq!(rust.start_page, py.start_page, "chunk[{i}] start_page");
        assert_eq!(rust.end_page, py.end_page, "chunk[{i}] end_page");
        assert_eq!(rust.token_count, py.token_count, "chunk[{i}] token_count");
        // Compare text + preview before page_spans because a text
        // mismatch is the most informative failure for diagnosing
        // tokenization or normalization drift.
        assert_eq!(rust.text, py.text, "chunk[{i}] text");
        assert_eq!(
            rust.text_preview, py.text_preview,
            "chunk[{i}] text_preview"
        );
        assert_eq!(
            rust.page_spans.len(),
            py.page_spans.len(),
            "chunk[{i}] page_spans len"
        );
        for (j, (rs, ps)) in rust.page_spans.iter().zip(py.page_spans.iter()).enumerate() {
            assert_eq!(rs.page, ps.page, "chunk[{i}].page_spans[{j}].page");
            assert_eq!(
                rs.byte_offset_in_chunk, ps.byte_offset_in_chunk,
                "chunk[{i}].page_spans[{j}].byte_offset_in_chunk"
            );
        }
        // chunk_hash binds (text, start_page, end_page, page_spans);
        // if all those match field-by-field above, the hash must too.
        // Asserting the hash explicitly catches any envelope-canonicalization
        // regression in cacg_core::hash.
        assert_eq!(rust.chunk_hash, py.chunk_hash, "chunk[{i}] chunk_hash");
    }
}
