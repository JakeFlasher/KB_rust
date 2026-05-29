# Legacy CFA KB Bootstrap Migration Plan

Date: 2026-05-28
Legacy KB audited: `/home/jakeshea/CFA_reading/CFA_reading`
Target framework workspace: `/home/jakeshea/knowledge_base_framework_discovery`

This report consolidates a local audit plus 11 read-only subagent audits. No legacy files
were edited. The purpose is to define the first cleanup/bootstrap step before full CACG
migration of the legacy CFA knowledge base and its PDF source corpus.

## Executive Decision

The first step should not be card migration. It should be a source-corpus bootstrap:

1. Snapshot the legacy tree at its current git state.
2. Build a deterministic source inventory from `_corpus_planning/05_source_matrix.md`.
3. Copy only admitted, quotable PDF sources into a clean `sources/` staging tree in this
   workspace, using canonical filenames and stable `source_id` values.
4. Preserve a machine-readable legacy-path map before any rename.
5. Ingest each source PDF into a fresh per-source output directory, then merge manifests
   deterministically because current Rust `kb ingest` cannot append into an existing
   `out/`.
6. Generate `out/source_matrix.json` from the same inventory.

Only after this bootstrap exists should we transform the 274 legacy cards into `cacg.v0`.

## Research Method

Completed subagent audits:

1. Top-level legacy docs and corpus governance.
2. `.claude` runtime state, generated artifacts, manifests, and skills.
3. `notes/` user-dynamic and OCR artifacts.
4. `scripts/` and `tools/` legacy tooling.
5. `01_Quantitative_Methods` and `02_Economics` PDFs.
6. `03_Financial_Reporting_Analysis` and `05_Equity` PDFs.
7. `06_Fixed_Income_and_Credit` and `07_Derivatives_and_Volatility`.
8. `09_Portfolio_Management_and_Asset_Pricing`, `10_Behavioral_Finance`, `11_Risk_Management`.
9. `20_Chinese_Convertible_Bonds_Research` and `Convertible_Bonds`.
10. `CFA_Program_Curriculum`, `Trading_Price_Action`, `deferred_books`.
11. Current Rust framework layout and migration constraints.

Local cross-checks:

- `05_source_matrix.md`: 73 registered source rows: 70 `quotable: yes`, 3 `quotable: no`.
- Legacy on-disk PDFs: 110 total.
- Non-deferred, non-notes PDFs on disk: 72.
- Matrix-admitted quotable PDFs: 70.
- Legacy authored cards: 274 active card files under `.claude/knowledge`.
- Total `.claude/knowledge` markdown: 333 files: 274 active cards plus 59 non-card
  markdown files, including 57 auxiliary `_*.md` files.

## Current Legacy KB Scope

Current active card inventory is 274 cards across 11 verticals:

| Subcorpus | Cards |
|---|---:|
| `01_quantitative_methods` | 17 |
| `02_economics` | 29 |
| `03_financial_reporting_analysis` | 29 |
| `05_equity` | 24 |
| `06_fixed_income_and_credit` | 36 |
| `07_derivatives_and_volatility` | 20 |
| `08_convertible_bonds` | 50 |
| `09_portfolio_management_and_asset_pricing` | 24 |
| `10_behavioral_finance` | 5 |
| `11_risk_management` | 24 |
| `17_cross_cutting` | 16 |

Prior research documents mentioning 219, 258, or 268 cards are stale for the current
legacy tree. The current `.claude/kb_manifest.json` also reports 274 entries and
`.claude/retracted_cards.json` is empty.

## Canonical Inputs vs Exclusions

Migrate as canonical inputs:

- 70 `quotable: yes` PDFs from `_corpus_planning/05_source_matrix.md`.
- `.claude/knowledge/**/*.md` cards as legacy authored knowledge input.
- Per-subcorpus `_source_role_map.md`, `_dependency_order.md`,
  `_chapter_overviews.md`, `_style_guide.md`, and `_diagram_primitives.md` as migration
  metadata.
- `volumes/*.md` as derived deliverable prose to preserve for later volume migration.
- `_corpus_planning/05_source_matrix.md` and `_corpus_planning/06_rule_registry.md` as
  governance metadata.

Do not ingest as active CACG sources:

- `deferred_books/`: 36 PDFs, hard-blocked by legacy rules.
- `notes/`: 2 PDFs plus OCR JSON/PNG artifacts. These are user-volatile under current
  legacy rules. Preserve separately, but do not authorize in `source_matrix.json`.
- EPUBs: 2 blacklisted files, non-quotable because page anchors are unstable.
- `01_Quantitative_Methods/Econometric Analysis of Cross Section and Panel Data...pdf`:
  registered but `quotable: no` due SCAN quality.
- `Trading_Price_Action/Brooks_2012_Trading_Price_Action_Reversals.pdf`: present on disk
  but not matrix-admitted.
- Generated artifacts: `.claude/dist`, `.claude/dist_volumes`, `.claude/kb_manifest.*`,
  zero-byte `.claude/dist/manifest.sqlite`, caches, `node_modules`, `__pycache__`,
  `.pytest_cache`, and OCR intermediate images/JSON.
- Legacy `scripts/kb` as production code. Mine it for tests/rules only.

## Source Corpus Findings

The admitted 70-PDF source set covers:

- CFA combined curriculum PDFs: 3.
- QM/Economics/FRA/Equity/FI/PM/BF/RM sources: 33 admitted PDFs.
- Convertible-bond and China CB research sources: 31 admitted PDFs.
- Trading price action: 3 admitted Brooks PDFs.

Known cleanup issues:

- Many filenames include acquisition metadata such as `z-library`, `libgen`, doubled
  spaces, brackets, and long non-ASCII titles.
- `source_id` in current Rust ingest must match `^[a-z0-9][a-z0-9_]*$`; use snake_case,
  not hyphenated IDs.
- `03_Financial_Reporting_Analysis/Stickney_Brown_Wahlen_2013_FRA_and_Valuation_8ed.pdf`
  appears to be misnamed; audit found it identifies as Wahlen, Baginski, Bradshaw,
  10th edition, copyright 2023.
- `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` appears to be a 2025 fourth
  edition, while the matrix says 2012.
- `06_Fixed_Income_and_Credit/The handbook of fixed income securities..pdf` may be
  edition-mismatched; audit found metadata suggesting Ninth Edition while the matrix
  labels 2012 8ed.
- `07_Derivatives_and_Volatility` has no local PDFs despite 20 cards; those cards depend
  on shared Hull/Glasserman/CFA sources elsewhere.
- Behavioral Finance has 5 current cards but a stale 15-card skeleton in planning docs.
- `.claude/dist` is stale: 268 generated HTML card pages for 274 markdown cards.

Current-version-sensitive regulatory sources:

- HKICPA/HKFRS sources should be treated as versioned standards, not static book anchors.
  Official HKICPA pages indicate HKFRS 9/HKFRS 7 amendments in August 2024 and February
  2025, and HKAS 32 search results show a later revision than the local 2022 file.
  Before final ingestion, either refresh these PDFs or explicitly mark them as
  historical version snapshots.

Official references checked:

- HKICPA HKFRS 9 page:
  https://www.hkicpa.org.hk/en/Standards-setting/Standards/New-and-major-standards/New-and-Major-Standards/HKFRS-9-Financial-Instruments
- HKICPA HKAS 32 PDF search result:
  https://www.hkicpa.org.hk/-/media/HKICPA-Website/Members-Handbook/volumeII/HKAS%2032_2023.pdf
- IFRS 7 standard page:
  https://www.ifrs.org/issued-standards/list-of-standards/ifrs-7-financial-instruments-disclosures/

## Current Framework Constraints

The Rust framework expects:

```text
cards/
  <reading_id>/
    <slug>.md
    <slug>.history.jsonl

out/
  sources_manifest.json
  chunks_manifest.json
  cards_manifest.json
  summaries.json
  INDEX.md
  summaries.sqlite
  source_matrix.json
  lint_journal.jsonl
```

Current native commands:

- Implemented: `ingest`, `new`, `lint`, `verify`, `index`, `retract-chunk`, `search`,
  `show`.
- Stubbed: `history`, `retract`, `retract-source`, `scaffold-matrix`,
  `scaffold-role-map`, `migrate-summaries`.

Bulk migration implications:

- `kb ingest` is native but only writes a single source into a fresh `--out`.
  It refuses when `sources_manifest.json` or `chunks_manifest.json` already exists.
- Multi-PDF migration therefore needs a merge step outside the current CLI.
- `source_matrix.json` must be generated manually or by a new bootstrap tool.
- Cards must include 80-400 character summaries and at least one citation.
- `kb index` should compute and write `card_hash`; migration scripts should emit cards
  without trying to precompute hashes.
- `source_matrix.json` is mandatory for `lint`, `verify`, `search`, and `show`.
- `search` excludes cards whose `source_ids` are empty or unauthorized.

## Proposed Target Layout

Use canonical copies in the new workspace, not in-place renames in the legacy tree:

```text
sources/
  cfa_legacy/
    _registry/
      source_inventory.json
      legacy_path_map.json
      source_matrix.json
      excluded_sources.json
      rename_decisions.md
      run_ingest_per_source.py
      merge_ingest_manifests.py
    pdfs/
      01_quantitative_methods/
      02_economics/
      03_financial_reporting_analysis/
      05_equity/
      06_fixed_income_and_credit/
      cfa_program_curriculum/
      convertible_bonds/
      china_convertible_bonds/
      portfolio_management_and_asset_pricing/
      behavioral_finance/
      risk_management/
      trading_price_action/
    excluded/
      deferred_books_inventory.json
      notes_inventory.json
      epub_blacklist.json
      dynamic_tree_quarantine_manifest.json
      legacy_notes_taint_manifest.json

cards/
  cfa_legacy/
    <reading_id>/

out/
  cfa_legacy/
    ingest_per_source/<source_id>/
    sources_manifest.json
    chunks_manifest.json
    source_matrix.json
    cards_manifest.json
    summaries.json
    INDEX.md
```

Recommended `source_id` convention:

- Use lowercase snake_case.
- Prefix where useful to prevent collisions: `qm_`, `econ_`, `fra_`, `fi_`, `cb_`,
  `china_cb_`, `cfa_`.
- Keep IDs shorter than filenames but bibliographically stable:
  `qm_tsay_2010_afts`, `fra_penman_5ed`, `cfa_2022_l1_combined`,
  `cb_calamos_2003`, `china_cb_szse_trading_rules_2022`.

Recommended filename convention:

- ASCII canonical names for copied PDFs when practical:
  `{author_or_issuer}_{year}_{short_title}_{edition_or_rev}.pdf`.
- For Chinese sources, prefer ASCII canonical filenames plus a registry field for
  original Chinese title.
- Never lose the exact old path: every copy/rename must be reversible through
  `legacy_path_map.json`.

## First-Step Bootstrap Plan

### Phase 1: Snapshot and inventory

Create a read-only snapshot record:

- legacy root absolute path,
- legacy git commit hash if available,
- inventory timestamp,
- total PDF/EPUB/card counts,
- SHA256 for every source-like file,
- row extraction from `_corpus_planning/05_source_matrix.md`.

Acceptance criteria:

- `sources/cfa_legacy/_registry/source_inventory.json` contains all 73 matrix rows.
- `excluded_sources.json` explains each excluded source: `deferred`, `notes`,
  `epub_blacklist`, `scan_nonquotable`, or `unregistered_pdf`.
- The inventory proves the active ingest set is exactly 70 PDFs.

### Phase 2: Canonical copy plan

Generate `legacy_path_map.json` with one row per active copied matrix source. The
non-active matrix rows and source-like exclusions are tracked in `excluded_sources.json`,
which is the hash authority for excluded files.

```json
{
  "legacy_path": "03_Financial_Reporting_Analysis/Penman_Financial_Statement_Analysis_and_Security_Valuation_5ed.pdf",
  "canonical_path": "sources/cfa_legacy/pdfs/03_financial_reporting_analysis/Penman_2013_FSA_and_Security_Valuation_5ed.pdf",
  "source_id": "fra_penman_5ed",
  "quotable": true,
  "source_sha256": "...",
  "subcorpora": ["03"],
  "status": "active"
}
```

Do not copy files whose status is not `active`.

Acceptance criteria:

- Every `active` row has a valid Rust `source_id`.
- Every original path resolves on disk.
- Every copied file's SHA256 equals the source-matrix hash or is explicitly flagged
  for review.
- Misnamed/uncertain files are copied with conservative names and flagged in
  `rename_decisions.md`.

### Phase 3: Generate authorization matrix

Transform legacy `subcorpora` into CACG reading IDs. Recommended mapping:

```text
01 -> 01_quantitative_methods
02 -> 02_economics
03 -> 03_financial_reporting_analysis
05 -> 05_equity
06 -> 06_fixed_income_and_credit
07 -> 07_derivatives_and_volatility
08 -> 08_convertible_bonds
09 -> 09_portfolio_management_and_asset_pricing
10 -> 10_behavioral_finance
11 -> 11_risk_management
17 -> 17_cross_cutting
```

Generate:

```json
{"schema_version":"cacg.v0","allowed":{"01_quantitative_methods":["..."]}}
```

Acceptance criteria:

- Every active card subcorpus has a matrix key.
- Every matrix source ID exists in the source inventory.
- Non-quotable, deferred, notes, and unregistered sources are absent.

Note: the generated authorization matrix may include reading IDs that have source
authorizations but no active legacy card directory yet. In the current legacy CFA bootstrap,
there are 11 active card subcorpora and 16 source-authorization namespaces.

### Phase 4: Ingest per source

Run `kb ingest` once per active PDF into a fresh per-source directory:

```text
out/cfa_legacy/ingest_per_source/<source_id>/
  sources_manifest.json
  chunks_manifest.json
```

Then merge all per-source manifests into:

```text
out/cfa_legacy/sources_manifest.json
out/cfa_legacy/chunks_manifest.json
```

Acceptance criteria:

- Merged `sources_manifest.json` has 70 sources.
- Merged `chunks_manifest.json` has non-empty chunks for each source.
- Source IDs are unique.
- Chunk IDs are unique.
- Retracted lists are sorted empty arrays at bootstrap.
- Per-source ingest is resumable: complete source directories are skipped and partial
  directories fail closed for manual inspection.
- Manifest merge is deterministic byte-for-byte across repeated runs under
  `KB_FROZEN_CLOCK=1`.

### Phase 5: Preserve legacy authored knowledge

Copy `.claude/knowledge` and `volumes` into a legacy-reference area or commit a manifest
of their paths/SHA256s. Do not yet rewrite cards.

Acceptance criteria:

- 274 active card files are accounted for.
- 59 non-card markdown files are accounted for, including 57 auxiliary `_*.md` files.
- `volumes/*.md` are accounted for.
- `.claude/dist*` and legacy runtime manifests are not copied as canonical outputs.
- `notes/`, `deferred_books/`, `scripts/`, `tools/`, and generated `.claude/dist*`
  trees are either excluded or quarantined before card migration.

## Why This Is The Right First Step

The hard dependency for a trustworthy CACG migration is real chunk manifests over real
PDFs. The existing framework can lint, verify, index, search, and show cards, but card
migration without a clean source bootstrap either produces synthetic chunks or binds
citations to unstable legacy paths. That would preserve the legacy weakness under a new
schema.

The bootstrap step isolates the source corpus, normalizes names, creates stable IDs,
records every exclusion, and works around the current CLI's lack of multi-source ingest
merge. Once complete, card migration becomes a deterministic transform:

legacy path + page span -> source_id -> overlapping chunk_ids -> exact/verbatim quotes
or Layer-3 semantic review queue.

## Immediate Risks To Track

- Full migration will create many citations per card because legacy page spans often
  overlap multiple chunks.
- Legacy prose is paraphrase-heavy. Prior QM migration findings showed strict
  substring verification passes only when migration emits verbatim chunk excerpts, not
  when using authored paraphrase as the quote.
- Regulatory PDFs may need refresh before final ingestion.
- `10_behavioral_finance` is partial and has future-card links.
- `07_derivatives_and_volatility` has cards but no local source PDFs.
- The README in the new framework is stale around deleted Python paths; use code and
  current tests as authority.
