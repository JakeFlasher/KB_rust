# Deferred Sources

These PDFs are preserved for context but hard-blocked from active CACG
citations.

## Contents

This directory contains:

- `21_sota_acquisitions_2026_non_china/` — non-Chinese-CB PDFs from the
  2026-05-21 SOTA acquisition pass. They are retained because they contain
  actual source assets, but they remain outside the active source matrix.

## Re-activation

To lift a deferral in a future migration:

1. Move or copy the PDF into `sources/cfa/pdfs/<topic>/`.
2. Add the source to `_registry/library_catalog.json` with SHA-256 and
   page-count metadata.
3. Add or update the corresponding `sources/cfa/_registry/` source
   matrix entries.
4. Re-ingest and rebuild `out/cfa/` before any card cites it.

## Active source folders

The active source set lives under `sources/cfa/pdfs/` and is described by the
committed `sources/cfa/_registry/` manifests (`source_matrix.json`,
`legacy_path_map.json`, `library_catalog.json`).
