# CFA source corpus (`sources/cfa/`)

The single source-of-record folder for the migrated CFA knowledge base. It holds the
git-tracked **cited PDF corpus**, the **registry** (build/gate/release pipeline + metadata),
and an untracked **navigation archive** of deferred/excluded items. This folder replaces the
former split between `sources/cfa_legacy/` (runtime corpus) and
`sources/cfa_knowledge_library/` (navigation overlay); the two were consolidated here in the
2026-06 namespace consolidation (`cfa_legacy → cfa`), and the overlay's metadata was salvaged
into `_registry/library_catalog.json`.

## Tree

```
sources/cfa/
├── README.md                  # this file
├── .gitignore                 # keeps the deferred/ + excluded/ archive PDFs untracked
│
├── pdfs/                      # the CITED corpus — 91 PDFs, GIT-TRACKED
│   ├── 01_quantitative_methods/ … 22_fund_level_arbitrage/
│   ├── cfa_program_curriculum/  china_convertible_bonds/  convertible_bonds/
│   ├── shared_anchors/          trading_price_action/
│   │   (cards cite these by source_id, never by path; filenames are the canonical
│   │    bootstrap names — see _registry/legacy_path_map.json for source_id → path)
│
├── _registry/                 # the build/gate/release pipeline + metadata (git-tracked)
│   ├── run_corpus_gate.sh      # the re-runnable corpus gate (lint + verify + invariants)
│   ├── build_scope_ledger.py  check_*.py  emit_*_slice.py  resolve_migration_citations.py
│   ├── *_curated_citations.json  source_matrix.json  legacy_path_map.json
│   ├── library_catalog.json   # salvaged navigation catalog (sha256, page_count, audit
│   │                            ratings, review_flags, shared_anchor, subcorpus_authorizations)
│   ├── page_coordinate_maps/  release_baseline/  rule9_canonical.md
│
├── excluded/                  # non-quotable items: JSON quarantine manifests (tracked) +
│   │                            the physical archive PDFs (untracked)
│   ├── *.json                 # deferred_books_inventory / epub_blacklist / notes_inventory /
│   │                            legacy_notes_taint_manifest / dynamic_tree_quarantine_manifest
│   ├── epub_blacklist/  notes_user_volatile/  portuguese_translation/  scan_nonquotable/
│
└── deferred/                  # 36 SOTA-2026 PDFs kept for future re-evaluation (untracked)
    └── 21_sota_acquisitions_2026_non_china/   # per-subfolder README rationale (tracked)
```

The `pdfs/` corpus is tracked so the build is self-contained and re-ingestable from a clean
checkout. The `deferred/` + `excluded/` PDFs/EPUBs are large navigation-only binaries and are
gitignored (`.gitignore`); their provenance is fully recorded in the tracked `*.json`
manifests and `library_catalog.json`.

## Workflow: revising a migrated knowledge card

1. **Locate the card**: `cards/cfa/<reading_id>/<card_id>.md`
2. **Identify cited source**: card frontmatter `citations[].source_id` (e.g.
   `cfa_2022_l1_combined`, `pm_pedersen_2015_efficiently_inefficient`).
3. **Find the PDF**: look up `source_id` in `_registry/library_catalog.json` (or
   `_registry/legacy_path_map.json`) → `library_path`/`canonical_path`, e.g.
   `pdfs/cfa_program_curriculum/cfa_2022_l1_combined.pdf`.
4. **Locate the cited page**:
   - Combined-volume CFA PDFs: use the per-volume offset table at
     `_registry/page_coordinate_maps/cfa_2022_l1_combined.json`
     (`volume_page_map.py` → `vol_page_to_pdf_page(volume, volume_page)`).
   - Single-volume PDFs: `citations[].page_range` is the PDF page index directly.
5. **Verify the quote**: `citations[].quote` must appear verbatim on the cited pages;
   `out/cfa/chunks_manifest.json` resolves `chunk_id` → chunk text byte-by-byte.
6. **Update the card**: edit the `.md`, then `kb index cards --out out/cfa` and
   `kb verify <card>` to regenerate `card_hash` and confirm the citation chain.

## How this folder relates to the rest of the workspace

| Location | Holds | Relationship |
|---|---|---|
| `/home/jakeshea/CFA_reading/` | Legacy KB (immutable, read-only) | Upstream origin of the PDFs + the migrated cards; never modified by this workspace |
| `sources/cfa/pdfs/` | 91 git-tracked cited PDFs | Runtime + reproducible source set for `out/cfa/` manifests |
| `sources/cfa/_registry/` | Build/gate/release pipeline + `library_catalog.json` | Authoritative metadata; `source_matrix.json` is the citation-authorization boundary |
| `out/cfa/sources_manifest.json` | Ingested source metadata (page_count, parser provenance) | Catalog mirrors page_count + ingest-time facts |
| `out/cfa/chunks_manifest.json` | Chunk-level text | Cards cite chunks; PDFs are the upstream |
| `cards/cfa/<reading>/*.md` | Migrated cacg.v0 cards | Cite the corpus via `source_id` |

## Critical caveats for revising cards

1. **CFA L1 combined PDF has 512 pages of non-CFA junk after the Wiley EULA** (PDF pages
   3842–4353, a flawed iLovePDF concatenation). The offset table's V6 `last_pdf_page` is
   capped at 3841; a citation to Vol.6/p.573+ is invalid regardless of how it converts.
2. **Wrong-volume legacy citations are common** (per Doc-29): always cross-check PDF content
   against the cited topic; the offset table mechanically converts whatever you give it.
3. **Soft-hyphen artifacts**: pdf extraction can insert U+FFFE mid-word (e.g. `rela￾tion`);
   ensure verbatim quotes handle or avoid these.
4. **Notes-taint (Critical Rule 9)**: never cite a `notes/` or `scripts/` path or a bare
   `CFA_note_N (OCR date) pp.X-Y` alias. The 6 formerly-quarantined risk/PM cards were
   re-authored from primary sources (QRM 2015 / CFA L1) during the 2026-06 consolidation, so
   the corpus is now fully absorbed (0 quarantined). Scrubbed reference copies of earlier
   notes-derived cards remain under `_legacy_reference/cfa/cards/` for context only.
5. **Deferred + excluded items**: do NOT cite from `deferred/` or `excluded/` in any cacg.v0
   card; `library_catalog.json` + `excluded/*.json` record them for future re-evaluation only.

## Maintenance

`library_catalog.json` carries a per-source `sha256`. If it disagrees with
`out/cfa/sources_manifest.json` `source_sha256`, the tracked PDF corpus has drifted from the
ingested manifests — investigate before emitting any new card. The corpus gate
(`_registry/run_corpus_gate.sh`) is the re-runnable authority; see
`_research/31_v1_release_and_rebuild_recipe.md` for the full rebuild recipe.
