# Legacy CFA Card Migration Preflight

Date: 2026-05-28

## Scope

This note records the first card-migration preflight after the legacy CFA PDF
sources were normalized, ingested, and merged into the CACG workspace. It does
not emit CACG cards. The purpose is to prove which legacy dynamic cards can
advance to page-offset and quote selection without mixing in notes-derived
material.

## Generated Artifacts

- `sources/cfa/_registry/build_card_migration_preflight.py`
- `sources/cfa/_registry/card_migration_queue.json`
- `sources/cfa/_registry/legacy_source_ref_map.json`
- `sources/cfa/_registry/page_offset_worklist.json`
- `sources/cfa/_registry/card_migration_preflight_summary.md`

The preflight generator now follows the project determinism convention:
`KB_FROZEN_CLOCK=1` writes `1970-01-01T00:00:00Z` into generated metadata.

## Findings

- Active legacy cards scanned: 274.
- Cards with source paths resolved and source-matrix authorization passed: 266.
- Cards quarantined because they are active notes-taint candidates: 8.
- Source resolution blockers: 0.
- Source authorization blockers: 0.
- Total legacy source-reference occurrences: 693.
- Unique legacy source paths referenced by active cards: 63.
- Unique resolved source IDs referenced by active cards: 63.
- Candidate chunk overlap check: 693 of 693 references overlap at least one
  ingested chunk when legacy page numbers are treated as PDF pages.

The overlap check is only a candidate retrieval guard. It is not proof that
legacy book pages equal PDF page coordinates. The next pass must verify page
offsets and choose verbatim quote evidence from the ingested chunks.

## Page Offset Worklist

The source-level worklist contains 63 source entries:

- `build_cfa_volume_to_pdf_offset_map`: 1 source.
- `verify_book_to_pdf_page_offset`: 40 sources.
- `verify_current_version_and_page_mapping`: 6 sources.
- `verify_pdf_page_mapping_for_regulatory_or_research_pdf`: 16 sources.

The CFA combined curriculum PDF is the largest offset-risk source because the
legacy cards cite volume and book pages while the ingested chunks are indexed
by merged-PDF page coordinates.

## Quarantine Boundary

The following active cards remain out of the migration lane until the notes
provenance question is resolved:

- `.claude/knowledge/02_economics/ec-currency-exchange-rates-and-parity.md`
- `.claude/knowledge/09_portfolio_management_and_asset_pricing/pm-tracking-error-and-active-risk.md`
- `.claude/knowledge/11_risk_management/rm-historical-simulation-var.md`
- `.claude/knowledge/11_risk_management/rm-monte-carlo-var.md`
- `.claude/knowledge/11_risk_management/rm-parametric-var.md`
- `.claude/knowledge/11_risk_management/rm-risk-objectives-and-tolerance.md`
- `.claude/knowledge/11_risk_management/rm-sensitivity-versus-simulation.md`
- `.claude/knowledge/17_cross_cutting/cc-material-info-and-dissemination-delay.md`

## Recommended First Migration Slice

Start with `10_behavioral_finance`.

Reasons:

- It has only 5 active cards.
- All 5 are in `ready_for_offset_and_quote_mapping`.
- It has no active notes-taint quarantine.
- It avoids the CFA combined-volume offset problem.
- It avoids current-version-sensitive and regulatory/current-web artifacts.
- It uses only two offset-worklist sources:
  - `bf_shleifer_2000_inefficient_markets`, cited over legacy pages 28-174.
  - `econ_hart_mascolell_regret_matching`, cited over legacy pages 1-50.

The next implementation step should build verified page-coordinate maps for
those two PDFs, select chunk-level quote evidence for the five behavioral
finance cards, then emit the first CACG card batch with lint/verify checks.

## Independent Sub-Agent Review

Six read-only review tracks were run before continuing beyond preflight:

- Determinism/idempotency.
- Source resolution and source-matrix authorization.
- Dynamic-content and notes-taint boundary.
- Page-offset worklist and first-slice choice.
- CACG card schema compatibility.
- Ingest integrity and Pdfium runtime provenance.

### Findings

- Source resolution and authorization claims were validated: 693 references,
  63 referenced source paths, 0 unresolved references, and 0 unauthorized
  card/source pairs.
- Dynamic quarantine is conditionally sufficient: the 8 tainted active cards
  are correctly bucketed, and no ready-bucket cards point into notes/scripts or
  other dynamic trees.
- `deliverable_ready` is unsafe as a migration gate because it is legacy
  frontmatter metadata and remains true on the 8 quarantined cards.
- Candidate chunk overlaps are retrieval guards only. They do not prove that
  legacy book pages equal PDF page coordinates.
- CACG emission is blocked until citations have verified `chunk_id`,
  `chunk_hash`, exact quote text, and valid page ranges.
- Legacy edge vocabulary includes relationships outside CACG `card_edges`;
  edges must be normalized or omitted during emission.
- Re-ingest reproducibility depends on the recorded system Pdfium binary unless
  the runner enforces the recorded library hash.

### Fixes Applied Before Continuing

- `build_card_migration_preflight.py` now defaults to frozen generated metadata
  for byte-stable direct reruns.
- Generated preflight files now share an `artifact_set_id` and include input
  fingerprints for the source matrix, path map, notes-taint manifest, source
  manifest, chunk manifest, and active legacy-card set.
- The generator writes JSON and Markdown outputs via temp-file plus atomic
  rename.
- Source page counts now come from `sources_manifest.json`, not max chunk page.
- Queue entries preserve `deliverable_ready` only as legacy metadata and add:
  - `deliverable_ready_semantics`
  - `eligible_for_offset_quote_mapping`
  - `eligible_for_cacg_emission: false`
  - `cacg_emission_blockers`
- The ingest runner now checks the recorded `/usr/lib/libpdfium.so` SHA-256
  before re-ingest unless explicitly bypassed.
- The merge script now recomputes source SHA-256 and every chunk hash before
  accepting per-source manifests into the merged manifest.

### Post-Fix Validation

- Direct reruns of `build_card_migration_preflight.py` are byte-stable.
- All preflight JSON artifacts have matching `artifact_set_id`.
- 0 cards are marked `eligible_for_cacg_emission`.
- 266 cards are marked `eligible_for_offset_quote_mapping`.
- 0 source page-count mismatches against `sources_manifest.json`.
- 0 candidate chunk-overlap gaps across 693 source refs.
- `run_ingest_per_source.py --dry-run --limit 1` passed the recorded Pdfium hash
  check.
- `merge_ingest_manifests.py --require-count 70` recomputed source/chunk hashes
  and left merged manifests unchanged.
