# Legacy CFA KB Bootstrap Execution Record

Date: 2026-05-28
Legacy KB: `/home/jakeshea/CFA_reading/CFA_reading`
Workspace: `/home/jakeshea/knowledge_base_framework_discovery`

This records the completed source-staging portion of the first bootstrap step after the
in-depth legacy audit in `23_legacy_cfa_kb_bootstrap_migration_plan.md`. Per-source
ingestion and deterministic manifest merge remain pending.

## What Was Created

- `sources/cfa_legacy/_registry/snapshot.json`
- `sources/cfa_legacy/_registry/source_inventory.json`
- `sources/cfa_legacy/_registry/legacy_path_map.json`
- `sources/cfa_legacy/_registry/excluded_sources.json`
- `sources/cfa_legacy/_registry/source_matrix.json`
- `sources/cfa_legacy/_registry/source_matrix.pretty.json`
- `sources/cfa_legacy/_registry/ingest_plan.json`
- `sources/cfa_legacy/_registry/run_ingest_per_source.sh`
- `sources/cfa_legacy/_registry/run_ingest_per_source.py`
- `sources/cfa_legacy/_registry/merge_ingest_manifests.py`
- `sources/cfa_legacy/_registry/ingest_merge_report.json`
- `sources/cfa_legacy/_registry/rename_decisions.md`
- `sources/cfa_legacy/_registry/legacy_content_manifest.json`
- `sources/cfa_legacy/excluded/deferred_books_inventory.json`
- `sources/cfa_legacy/excluded/notes_inventory.json`
- `sources/cfa_legacy/excluded/epub_blacklist.json`
- `sources/cfa_legacy/excluded/dynamic_tree_quarantine_manifest.json`
- `sources/cfa_legacy/excluded/legacy_notes_taint_manifest.json`
- `sources/cfa_legacy/pdfs/**`: 70 canonical PDF copies
- `out/cfa_legacy/source_matrix.json`

No files in the legacy KB were edited.

## Snapshot Counts

The bootstrap snapshot records:

- 70 active PDF rows from the source matrix.
- 42 excluded source-like files.
- 274 active legacy card files.
- 59 non-card legacy knowledge markdown files.
- 353 legacy content manifest entries.
- 20 legacy volume markdown files.
- 73 total source-matrix rows.
- 16 generated CACG reading IDs.
- 783 quarantined dynamic/runtime/tooling files.
- 27 notes-taint candidate markdown files, including 8 active-card candidates.
- 70 completed per-source ingest directories.
- 57,603 merged chunks.

Legacy git state at snapshot:

- `856c4f3cfa9228ac6c4fd4a23e60ee90556b4225`
- Dirty/untracked legacy status noted by the snapshot: `?? .humanize/`

## Validation Completed

The generated bootstrap was validated for:

- 70 active rows in `legacy_path_map.json`.
- Unique, Rust-compatible snake_case `source_id` values.
- Copied file SHA256 values matching both the original legacy files and the matrix
  hashes.
- `out/cfa_legacy/source_matrix.json` containing only known source IDs.
- All generated matrix reading IDs matching the planned CACG reading namespaces.

Independent sub-agent review then found the active PDF source boundary sound and confirmed
that Pdfium is now discoverable through `/usr/lib/libpdfium.so` after installing
`libpdfium-nojs`. The review also required pre-ingest fixes for runner idempotency,
deterministic timestamps, metadata freshness flags, and aggregate manifest merging; those
fixes are now reflected in the registry scripts and metadata artifacts.

## Ingest Status

The per-source ingest runner is prepared at
`sources/cfa_legacy/_registry/run_ingest_per_source.sh`. It now sets
`KB_FROZEN_CLOCK=1` by default, ensures `/usr/lib` is visible in `LD_LIBRARY_PATH`, skips
already complete per-source outputs, and fails closed on partial output directories.

The deterministic merge utility is prepared at
`sources/cfa_legacy/_registry/merge_ingest_manifests.py`.

Probe command attempted:

```bash
cargo run -p cacg-cli --bin kb -- ingest sources/cfa_legacy/pdfs/china_convertible_bonds/china_cb_hkex_ch16_convertible_equity.pdf --source-id china_cb_hkex_ch16_convertible_equity --out out/cfa_legacy/ingest_probe
```

Initial observed blocker before `libpdfium-nojs` was installed:

```text
CACG-INGEST-001: pdfium bind failed: LoadLibraryError(DlOpen { desc: "libpdfium.so: cannot open shared object file: No such file or directory" })
```

After `libpdfium-nojs` was installed, a single-source probe succeeded, then the full
70-source ingest completed. The deterministic merge produced:

- `out/cfa_legacy/sources_manifest.json`: 70 sources.
- `out/cfa_legacy/chunks_manifest.json`: 57,603 chunks.

Validation passed:

- all 70 per-source directories contain `sources_manifest.json` and
  `chunks_manifest.json`;
- all 57,603 chunk IDs are unique;
- every active source has at least one chunk;
- merged source hashes match `legacy_path_map.json`;
- all source records have frozen `extracted_at = 1970-01-01T00:00:00Z`;
- a second merge run returned `unchanged` for both merged manifests.

Runtime provenance:

- system library: `/usr/lib/libpdfium.so`;
- package: `libpdfium-nojs 7778.r8.72ea487e43-1`;
- library SHA256:
  `c110f5240692b1915ad090d4f7e9bc6afa429db41341f2339d866d48402edbe5`;
- Rust parser record in manifests: `pdfium-render 0.9.1`.

Residual reproducibility risk: this Arch Pdfium build is operational, but it is not the
project's documented pinned Pdfium binary, so chunk text and chunk hashes should be
treated as tied to the runtime provenance above.
