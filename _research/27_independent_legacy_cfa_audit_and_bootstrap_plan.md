# Independent Legacy CFA KB Audit and Bootstrap Plan

Date: 2026-05-28
Legacy KB audited: `/home/jakeshea/CFA_reading/CFA_reading` (HEAD `856c4f3`)
Target framework workspace: `/home/jakeshea/knowledge_base_framework_discovery`

This document is an **independent** re-research of the legacy CFA KB and a fresh
bootstrap plan for migrating PDF sources into the new Rust CACG framework. It was
produced from 13 read-only subagent audits that were explicitly forbidden from
reading prior research (`_research/23-26`), the existing staged copies
(`sources/cfa/`, `cards/cfa/`, `out/cfa/`), or the
now-retired Python-port archive that formerly lived at `tools/python_legacy/`.
The purpose is to cross-check the prior
bootstrap by deriving the plan from raw legacy state.

## 1. Executive Summary

The legacy KB is a 1.5 GB tree at `/home/jakeshea/CFA_reading/CFA_reading` with
**110 PDFs** (intentionally gitignored), **2 EPUBs**, **275 authored cards on
disk** (manifest reports 274; the 08_CB subcorpus has 51 on disk vs 50 in the
manifest), and a strict 9-rule governance regime captured in a 73-row source
matrix (`_corpus_planning/05_source_matrix.md`) backed by a per-vertical FM006
stance vocabulary (40+ stances) and 14 RULE-* identifiers in the rule registry.

The first migration step must be a **source-corpus bootstrap**, not card
migration. The hard reason: the target Rust framework's `kb ingest` is
**single-PDF-only** and refuses to publish into a non-empty `out/` directory
(`CACG-INGEST-003`); composing the full source corpus therefore requires a
per-source ingest loop plus a deterministic manifest merge that must be built
outside the existing CLI. Card migration is downstream of that contract because
every CACG card requires a `chunk_id` + `chunk_hash` + verbatim quote bound to
real ingested chunks, and the legacy corpus has zero verbatim quotes — all
citations are paraphrase + page-span. Quote rebinding cannot happen until the
chunks exist.

Recommended first slice after bootstrap: **`10_behavioral_finance`** — smallest
on-disk corpus (5 authored cards), no notes-taint, no regulatory freshness
issues, only two source-coordinate questions.

## 2. Research Method

Thirteen read-only subagents audited disjoint slices of the legacy KB and target
framework in parallel. All were instructed not to read prior research,
prior-staged sources, or the legacy Python port, so the resulting plan is
derivable from the raw legacy state.

| # | Scope |
|---|---|
| 1 | `01_Quantitative_Methods/` + `02_Economics/` |
| 2 | `03_Financial_Reporting_Analysis/` + `05_Equity/` |
| 3 | `06_Fixed_Income_and_Credit/` + `07_Derivatives_and_Volatility/` |
| 4 | `09_Portfolio_Management/` + `10_Behavioral_Finance/` + `11_Risk_Management/` |
| 5 | `Convertible_Bonds/` + `20_Chinese_Convertible_Bonds_Research/` |
| 6 | `CFA_Program_Curriculum/` + `Trading_Price_Action/` + `deferred_books/` |
| 7 | `_corpus_planning/` + `REMAINING_BOOKS.md` + `DEFERRED_TOPICS.md` |
| 8 | `.claude/knowledge/` authored cards + `kb_manifest.json` |
| 9 | `CLAUDE.md` + `README.md` + `STATUS.md` + `CHANGELOG.md` + `.archive/` |
| 10 | `scripts/kb/` + `tools/` |
| 11 | `notes/` + `volumes/` |
| 12 | `.humanize/` + `.agents/` + `.codex` + `.claude` workflow non-knowledge |
| 13 | New Rust framework: crates, CLI, manifest schemas, determinism, gaps |
| 14 | Legacy git state + generated-artifact census + disk-usage breakdown |

## 3. Legacy KB Scope

### 3.1 Source corpus on disk (110 PDFs + 2 EPUBs)

| Subcorpus / area | PDFs | Notable |
|---|---:|---|
| `01_Quantitative_Methods/` | 7 | 1 PDF is image-only scan, no text layer (Wooldridge Cross/Panel 2e) |
| `02_Economics/` | 5 | MWG 108 MB; Hart/Mas-Colell is the 2013 World Scientific BOOK, not the 2000 paper |
| `03_Financial_Reporting_Analysis/` | 8 | 3 HKICPA standards; `Stickney_..._8ed.pdf` is actually Wahlen 10e (2023) |
| `05_Equity/` | 1 | `Damodaran_..._4ed.pdf` is the 2025 4e, not 2012 |
| `06_Fixed_Income_and_Credit/` | 7 | `The handbook of fixed income securities..pdf` is 9e (2021), not 8e (2012) |
| `07_Derivatives_and_Volatility/` | 0 | Cards depend on shared Hull/Glasserman in `Convertible_Bonds/classic_pricing_books_english/` |
| `09_Portfolio_Management/` | 2 | Cochrane, Pedersen |
| `10_Behavioral_Finance/` | 2 | Shleifer 2000, Gennaioli/Shleifer 2018 |
| `11_Risk_Management/` | 1 | McNeil/Frey/Embrechts QRM 2015 Revised |
| `Convertible_Bonds/classic_books_CB_english/` | 5 + 1 EPUB | Calamos, DeSpiegeleer, Maitland (EPUB), Philips, Thorp, Zubulake |
| `Convertible_Bonds/classic_pricing_books_english/` | 4 | Glasserman, Hull, Koziol, Lando — shared sources for 07/06/08 |
| `Convertible_Bonds/targeted_books_Chinese/` | 2 + 1 EPUB + 1 PDF/EPUB pair | All four are libgen/z-library-tagged with non-ASCII filenames |
| `20_Chinese_Convertible_Bonds_Research/` | 19 | Across `exchange_rules/CSDC/CSRC/PBOC/SSE/SZSE`, `court_judicial/`, `sell_side_research/`, `industry_reports/`, `offshore_HKEX/`, `academic_papers/` |
| `CFA_Program_Curriculum/` | 3 | L1 2022 (4353 pp), L2 2023 (3369 pp), L3 2022 (3863 pp) — combined volumes with per-volume page numbering |
| `Trading_Price_Action/` | 4 | Brooks 2009 + 2012 trilogy; `..._Reversals.pdf` is Portuguese |
| `deferred_books/` | 28 + 1 HTML | Hard-blocked under Critical Rule 8; spans 11 subfolders |
| **Total** | **~110** | + 2 EPUBs |

### 3.2 Authored knowledge

- **275 cards on disk** under `.claude/knowledge/<NN>_*/` across 11 numbered
  subcorpora (manifest reports 274; drift in `08_convertible_bonds/` is 51 on
  disk vs 50 in `kb_manifest.json`). `retracted_cards.json` is empty.
- **57 auxiliary `_*.md` files** per subcorpus (`_source_role_map.md`,
  `_dependency_order.md`, `_chapter_overviews.md`, `_style_guide.md`,
  `_diagram_primitives.md`, plus one `_future_03_resolution.md`).
- **20 volume artifacts** in `volumes/`: 10 `*-volume-draft.md` + 10 paired
  `cfa-quick-read-*-volume-outline.md`. Volumes are derived deliverable prose
  citing both cards (`[Backing card](...)` links) and primary PDFs.
- **11 skill routers** under `.claude/skills/` (1 meta-router + 10 specialty
  `understanding-*` skills).

### 3.3 Governance

- **9 Critical Rules** in `CLAUDE.md` (no calc examples, citation-mandatory,
  ASCII-only diagrams, OK/GOOD-only sources, topic-first layout, no invented
  math, archive informational-only, deferred-folder hard-block, notes/scripts
  hard-block — Rule 9 added 2026-05-25).
- **73-row source matrix** in `_corpus_planning/05_source_matrix.md` with 10
  columns. 70 rows are `quotable: yes`, 3 are `quotable: no` (2 EPUBs + 1
  unreadable scan).
- **14 RULE-\* IDs** in `_corpus_planning/06_rule_registry.md` mapping
  style-guide rules to ~49 linter diagnostic codes across 14 prefixes.
- **40+ per-vertical FM006 stance values** (e.g. `primary-cfa`,
  `primary-damodaran`, `primary-anduquan-3ed`) coupled to specific PDFs via
  `_lint_fm006_dispatch.py`.

### 3.4 Tooling

`scripts/kb/` is a Python-stdlib-only gate stack (~6000 LOC):
`lint_cards.py` (1804 LOC, master linter), `build_manifest.py` (1463 LOC, SQLite
FTS5 manifest + defense-in-depth `_validate_rows`), `check_source_matrix.py`,
`check_volume_citations.py`, `check_rule_registry.py`, `build_html_cards.py`,
`build_html_volumes.py`, plus `_lint_density.py`, `_lint_doc_overview.py`,
`_lint_misc_gates.py`, `_lint_fm006_dispatch.py`, `_shared_validation.py`,
`_volume_resolver.py`, `metadata.py`. `tools/audit_pdf_quality.py` (~430 LOC)
runs the `SCAN/POOR/OK/GOOD` chars-per-page gate that admits PDFs into the
matrix. 13 test files + 80+ lint fixtures.

### 3.5 User-dynamic content

- `notes/` (513 MB, 302 files): 2 PDFs (`CFA_note_2.pdf` + `CFA_note_2.ocr.pdf`)
  plus 298 MB of paddleocr per-page artifacts plus 2 unreferenced HTML
  deliverables (`convertible_bond_basics/`, `ipo_dark_pool_second_peak/`). Hard-
  blocked from citation by `NOTES-001/002`. **Taint residue**: 7 inline
  citations in `cc-material-info-and-dissemination-delay.md` use the
  bare-alias form `CFA_note_2 (2026 OCR) pp.115-116` which the path-prefix
  check does not catch.
- `scripts/` (5.3 MB): Hard-blocked from citation by `SCRIPTS-001/002`. May be
  referenced as tooling in non-`**Source:**` prose only.

### 3.6 Legacy git state

HEAD `856c4f3cfa9228ac6c4fd4a23e60ee90556b4225` ("fix: include zero in bar
chart y-axis scale..."). Branch `master` is 30 commits ahead of
`origin/master` — local-only work, not pushed. Working tree clean except for
`?? .humanize/` (the project-local `WORKFLOW.md` + `config.json`). All binary
corpus (`*.pdf`, `*.epub`, etc.), `node_modules/`, `notes/CFA_note_2_ocr/`,
`.claude/dist*`, `.claude/kb_manifest.*`, `.claude/retracted_cards.json` are
gitignored.

## 4. Canonical Inputs vs Exclusions

### 4.1 Migrate as canonical inputs

| What | Count | Why |
|---|---:|---|
| `quotable: yes` PDFs from source matrix | 70 | The authoritative ingest set; every citation in every authored card resolves to a matrix row |
| `.claude/knowledge/**/*.md` cards | 275 | Authored knowledge layer; needs quote rebinding to migrate, not raw copy |
| Per-subcorpus `_*.md` aux files | 57 | Style guides, role maps, dependency order, chapter overviews, diagram primitives — drive the linter behavior |
| `_corpus_planning/05_source_matrix.md` | 1 | THE governance contract |
| `_corpus_planning/06_rule_registry.md` | 1 | Linter ↔ style-guide ↔ rule-id binding |
| `_corpus_planning/04_pdf_quality_audit.md` | 1 | SCAN/POOR/OK/GOOD ladder methodology |
| `CLAUDE.md`, `README.md`, `STATUS.md`, `CHANGELOG.md` | 4 | Governance, history; carry forward as a `_legacy_reference/` bundle |
| `volumes/*.md` | 20 | Derived prose; migrate as derived output, not source |
| `.humanize/bitlesson.md` | 1 | 180KB of post-mortem lessons; carry forward generalizable entries |
| `.claude/skills/SKILL.md` files | 11 | Optional — useful routing pattern, but content is CFA-domain-specific |

### 4.2 Do NOT ingest as active CACG sources

| What | Count | Reason |
|---|---:|---|
| `deferred_books/` PDFs | 28 + 1 HTML | Hard-blocked by Critical Rule 8 |
| `notes/` content | 2 PDFs + 298 MB OCR | User-volatile, Critical Rule 9 |
| EPUBs | 2 | Critical Rule 4 / DEC-2 — no stable page anchors |
| `01_Quantitative_Methods/Econometric Analysis of Cross Section and Panel Data, 2ed...pdf` | 1 | SCAN 0.0 cpp; `quotable: no` until replaced |
| `Trading_Price_Action/Brooks_2012_..._Reversals.pdf` | 1 | Portuguese translation; quarantine pending English Wiley edition |
| `.claude/dist*` | ~270 HTML | Generated render output |
| `.claude/kb_manifest.{json,sqlite}` | 2 | Generated manifests |
| `.claude/retracted_cards.json` | 1 | Generated (empty `[]`) |
| `node_modules/` | 169 files | Playwright deps |
| `.pytest_cache/`, `__pycache__/` | n/a | Build caches |
| `scripts/kb/`, `tools/` as production code | — | Mine for rules/fixtures only |
| `.humanize/{rlcr,skill,ideas,discovery_rounds,codex-*}/` | 1397 files | Workflow ephemera |

### 4.3 Known cleanup issues requiring decisions before final ingest

1. **Filename rot**: 6+ PDFs carry `z-library.sk` / `1lib.sk` / `libgen.li`
   marks; ~5 files have doubled spaces or trailing double-period; 4 Chinese
   files have non-ASCII names with the typo `饕饕` (should be `饕餮`).
2. **Edition/title mismatches**:
   - `03_FRA/Stickney_..._8ed.pdf` is actually **Wahlen 10e (2023)**, not
     Stickney 8e (2013).
   - `05_Equity/Damodaran_..._4ed.pdf` is the **2025 4e** (Wiley, ISBN
     978-1-394-25460-6), not 2012.
   - `06_FI/The handbook of fixed income securities..pdf` is **Fabozzi 9e
     (2021)**, not 8e (2012). Confirmed via PDF metadata Title field.
   - `01_QM/ESLII_print12_toc.pdf` is ESL 2e (corrected 12th printing 2017).
   - `01_QM/ISLP_website.pdf` is the **2023 Python edition**, not ISL 2e (R, 2021).
   - `01_QM/Tsay_AFTS.pdf` is **2e (2005)**, not 3e (2010) as planning suggests.
3. **Regulatory freshness**:
   - `HKICPA_HKFRS_7_..._2018.pdf` is stale — IASB amended IFRS 7 in May 2024;
     HKICPA's HKFRS 7 was consolidated Aug 2024.
   - `HKICPA_HKAS_32_..._2022.pdf` may be superseded by 2024 FICE amendments.
   - `csrc-disclosure-standard-no-60.pdf` has no date in filename; multiple
     revisions exist.
   - `sse-cb-rules-compilation.pdf` is undated; an SSE bundle.
   - `hkex-mb-listing-rules-ch16-...pdf` and `ch28-...pdf` are 23–26 KB
     each — likely 1–2 page excerpts, not full chapters.
4. **Combined-volume page-coordinate problem**: CFA L1/L2/L3 combined PDFs use
   per-volume printed page numbers that reset at each volume boundary. The
   legacy KB enforces `Vol.<N>/pp.<P-Q>` disambiguation (`CITE-005`/`AC-2.1`);
   the new framework needs a volume-to-PDF-page offset table per combined PDF.
5. **No verbatim quotes anywhere**: All 275 cards are paraphrase + citation
   tail. The CACG `cacg.v0` schema requires `chunk_id` + `chunk_hash` + exact
   quote per citation. Migration must rebind quotes from ingested chunks.
6. **08_CB card-count drift**: 51 on disk vs 50 in manifest. One file is extra
   relative to the manifest; identify before migration begins.
7. **Notes-taint residue**: 7 inline `CFA_note_2 (2026 OCR) pp.115-116`
   citations in `cc-material-info-and-dissemination-delay.md` evade the
   path-prefix lint and must be scrubbed before migration.

## 5. Target Framework Constraints

The Rust framework defines a workspace with 5 crates (`cacg-core`, `cacg-ingest`,
`cacg-search`, `cacg-semantic`, `cacg-cli`). The `kb` binary exposes 14
subcommands; **5 are fully implemented** (`ingest`, `new`, `lint`, `verify`,
`index`, `search`, `show`) with `retract-chunk` partial; **7 are stubs** that
exit 1 with `CACG-CLI-NOT-IMPLEMENTED-*`: `history`, `retract`,
`retract-source`, `scaffold-matrix`, `scaffold-role-map`, `migrate-summaries`.

### 5.1 Bootstrap-blocking gaps

1. **`kb ingest` is single-PDF-only and refuses to publish into a non-empty
   `out/`** (`CACG-INGEST-003: PriorManifestsPresent`). The source comment in
   `crates/cacg-ingest/src/manifest.rs:269-308` is explicit: merge with prior
   manifests is "out of scope this round (left for a follow-up under
   task-m4-3)". Multi-source ingest therefore requires either (a) ingest per
   source into a fresh `out_dir` and externally merge, or (b) port the Python
   merge logic.
2. **`kb scaffold-matrix` is a stub** — `source_matrix.json` must be authored or
   built by a migration script. Every `kb lint`/`verify`/`search`/`show`
   invocation has `--source-matrix` as a `required = true` clap arg.
3. **`source_id` regex is `^[a-z0-9][a-z0-9_]*$`** — no hyphens. Note
   `reading_id` and `slug` regex `^[a-z0-9][a-z0-9_-]*$` DO allow hyphens.
4. **Card schema cacg.v0 hard requirements**: `summary` length [80, 400]
   characters; `citations` non-empty; each citation must carry `chunk_id`
   (format `<source_id>:p<NNN>:<NNNN>`), `chunk_hash` (64-hex), `page_range`,
   non-empty `quote`. Legacy cards have none of these inline.
5. **Pdfium binary pin**: `pdfium-render 0.9.1` against Pdfium native
   `149.0.7825.0` (SHA-256 pin documented in `docs/pdfium-binary-provisioning.md`).
   Runtime resolves `libpdfium.so` via `LD_LIBRARY_PATH`. CI has no auto-
   provisioning step; chunk-hash byte equality depends on the pinned binary.
6. **Determinism gate**: `KB_FROZEN_CLOCK=1` collapses all timestamps to
   `1970-01-01T00:00:00Z` and UUIDs to nil. All JSON goes through
   `canonical_json` (sort_keys + compact). The whole CI suite runs with
   `KB_FROZEN_CLOCK=1`. `xtask lint-determinism` static-greps for
   non-deterministic calls.

### 5.2 Required output layout (per the framework)

```
out/
├── sources_manifest.json          # kb ingest (cacg.v0; per-source SHA256, parser provenance)
├── chunks_manifest.json           # kb ingest (cacg.v0; chunk_id, chunk_hash, page_spans, text)
├── cards_manifest.json            # kb index
├── summaries.json                 # kb index
├── INDEX.md                       # kb index
├── summaries.sqlite               # kb index (FTS5 sidecar; non-fatal)
├── lint_journal.jsonl             # kb lint/verify (append-only chained checksum)
├── source_matrix.json             # operator-authored; mandatory
└── role_maps/<reading_id>.json    # optional per-reading
```

## 6. Proposed Target Layout

Canonical copies in the new workspace; no in-place renames in the legacy tree.

```
sources/
  cfa/
    _registry/
      snapshot.json
      source_inventory.json
      legacy_path_map.json
      source_matrix.json
      excluded_sources.json
      rename_decisions.md
      legacy_content_manifest.json
      ingest_plan.json
      run_ingest_per_source.{sh,py}
      merge_ingest_manifests.py
      ingest_merge_report.json
    pdfs/
      01_quantitative_methods/
      02_economics/
      03_financial_reporting_analysis/
      05_equity/
      06_fixed_income_and_credit/
      07_derivatives_and_volatility/      # may be empty; cards use shared
      09_portfolio_management_and_asset_pricing/
      10_behavioral_finance/
      11_risk_management/
      cfa_program_curriculum/
      convertible_bonds/
      china_convertible_bonds/
      trading_price_action/
      shared_anchors/                     # Hull, Glasserman, Lando (cross-vertical)
    excluded/
      deferred_books_inventory.json
      notes_inventory.json
      epub_blacklist.json
      dynamic_tree_quarantine_manifest.json
      legacy_notes_taint_manifest.json

cards/
  cfa/
    <reading_id>/

out/
  cfa/
    ingest_per_source/<source_id>/
      sources_manifest.json
      chunks_manifest.json
    sources_manifest.json                 # merged
    chunks_manifest.json                  # merged
    source_matrix.json
    cards_manifest.json
    summaries.json
    INDEX.md

_legacy_reference/                        # outside framework's source/cards/out
  cfa_governance/                         # CLAUDE/README/STATUS/CHANGELOG snapshots
  cfa_notes_archive/                      # quarantined notes/ tree (hash-manifested)
  cfa_volumes/                            # 20 volume drafts/outlines for later regen
```

Naming conventions:

- **`source_id`**: lowercase snake_case, no hyphens. Pattern
  `{prefix}_{author_or_issuer}_{year}_{short_title}_{edition_or_rev}` where
  prefix is one of `qm_, econ_, fra_, eq_, fi_, deriv_, pm_, bf_, rm_, cfa_, cb_, china_cb_, brooks_`.
  Examples: `qm_tsay_2005_afts_2e`, `fra_wahlen_2023_fra_valuation_10e`,
  `eq_damodaran_2025_investment_valuation_4e`,
  `fi_fabozzi_2021_handbook_fixed_income_9e`, `cfa_l1_2022_combined`,
  `brooks_2009_reading_price_charts`,
  `china_cb_szse_2025_self_reg_guideline_15`.
- **Canonical PDF filename**: ASCII-only,
  `{Author_or_Issuer}_{Year}_{Short_Title}_{Edition}.pdf`. For Chinese-language
  sources, ASCII filename + `title_native` / `author_native` /
  `author_pinyin` fields in the registry.
- **`reading_id`**: matches legacy subcorpus enum (`01_quantitative_methods`,
  ..., `17_cross_cutting`). Hyphens permitted by regex but unused.

## 7. First-Step Bootstrap Plan

### Phase 1 — Snapshot and inventory

Generate `sources/cfa/_registry/snapshot.json` recording:

- legacy absolute path, git HEAD (`856c4f3`), unpushed-commit count (30),
  dirty/untracked summary, tool versions, snapshot UTC timestamp;
- SHA256 for every source-like file (110 PDFs + 2 EPUBs);
- extracted rows from `_corpus_planning/05_source_matrix.md` (all 73 rows;
  10 columns; per-row SHA256 verified against on-disk);
- per-subcorpus card counts including the 50-vs-51 08_CB drift flag;
- generated-artifact and dynamic-tree census for quarantine.

**Acceptance**: `source_inventory.json` reproduces all 73 matrix rows; the
active ingest set is exactly 70 PDFs; 42+ excluded source-like files explained
(deferred, notes, EPUB, scan, unregistered, Portuguese-translation, etc.);
hash-of-hashes Merkle over inventoried files for future drift detection.

### Phase 2 — Canonical copy plan

Generate `legacy_path_map.json` with one row per active source:

```json
{
  "legacy_path": "03_Financial_Reporting_Analysis/Penman_Financial_Statement_Analysis_and_Security_Valuation_5ed.pdf",
  "canonical_path": "sources/cfa/pdfs/03_financial_reporting_analysis/Penman_2013_FSA_and_Security_Valuation_5e.pdf",
  "source_id": "fra_penman_2013_fsa_security_valuation_5e",
  "quotable": true,
  "source_sha256": "...",
  "subcorpora": ["03"],
  "status": "active",
  "rename_reason": "ascii_canonical",
  "edition_flag": null
}
```

For misnamed/uncertain files (Stickney→Wahlen, Damodaran 2012→2025,
Fabozzi 8e→9e), copy with conservative canonical names and flag in
`rename_decisions.md` with the evidence (PDF metadata Title field,
copyright page text). For Chinese-language sources, also persist
`title_native`, `author_native`, `author_pinyin`, `provenance_flag` (libgen /
z-library / publisher).

**Acceptance**: every `active` row has a valid Rust `source_id` (regex
`^[a-z0-9][a-z0-9_]*$`); every legacy path resolves on disk; every copied
SHA256 equals the matrix hash or is explicitly flagged; misnamed files have
explicit `rename_decisions.md` entries.

### Phase 3 — Generate authorization matrix

Transform legacy `subcorpora` into CACG `reading_id`s:

```
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

Emit `out/cfa/source_matrix.json`:

```json
{"schema_version":"cacg.v0","allowed":{"01_quantitative_methods":["..."]}}
```

**Acceptance**: every active card subcorpus has a matrix key; every matrix
source ID exists in the source inventory; non-quotable, deferred, notes, and
unregistered sources are absent; the per-vertical FM006 stance-to-PDF coupling
(preserved separately) admits only sources present in the matrix.

### Phase 4 — Per-source ingest + deterministic merge

Run `kb ingest` once per active PDF into a per-source directory because the
framework refuses to write into a non-empty `out/`:

```
out/cfa/ingest_per_source/<source_id>/
  sources_manifest.json
  chunks_manifest.json
```

A runner (`run_ingest_per_source.{sh,py}`) iterates the active inventory with:

- `KB_FROZEN_CLOCK=1` by default;
- `LD_LIBRARY_PATH` augmented to include `/usr/lib` (or wherever
  `libpdfium.so` lives);
- recorded `libpdfium.so` SHA-256 verified before each ingest;
- already-complete per-source directories skipped (resumable);
- partial per-source directories fail closed for manual inspection.

A merger (`merge_ingest_manifests.py`) deterministically combines per-source
manifests into:

```
out/cfa/sources_manifest.json
out/cfa/chunks_manifest.json
```

with re-verified source SHA-256 and per-chunk hashes. Manifest writes use
temp-file + atomic rename.

**Acceptance**: merged `sources_manifest.json` has exactly 70 sources; merged
`chunks_manifest.json` has non-empty chunks for each source; all `chunk_id`
values unique; retracted lists empty arrays at bootstrap; a second merge run
returns `unchanged` byte-for-byte; all `extracted_at` are
`1970-01-01T00:00:00Z`. Plan-time chunk-count estimate: tens of thousands
across 70 PDFs (the CFA L1 combined alone is 4353 pages; many credit/FI books
exceed 700 pages).

### Phase 5 — Preserve legacy authored knowledge

Copy `.claude/knowledge/` (275 cards + 57 aux files + 11 skill SKILL.md) and
`volumes/` (20 files) into `_legacy_reference/` paths with a content manifest
of paths + SHA-256s. Do not yet rewrite cards into `cacg.v0` form.

**Acceptance**: 275 active card files accounted for; 57 aux files accounted for;
20 volume artifacts accounted for; `.claude/dist*`, `.claude/kb_manifest.*`,
`.claude/retracted_cards.json` excluded; `notes/`, `deferred_books/`,
`scripts/`, `tools/`, `node_modules/`, `.humanize/{rlcr,skill,...}/`,
`.pytest_cache/`, `__pycache__/` quarantined per
`dynamic_tree_quarantine_manifest.json`. Note the 08_CB 50-vs-51 drift in the
manifest so the migrator can resolve before card emission.

### Phase 6 — Notes-taint scrub

Two policy decisions block clean card migration:

1. The 7 surviving `CFA_note_2 (2026 OCR) pp.115-116` bare-alias citations in
   `cc-material-info-and-dissemination-delay.md` (and 1 in
   `17_cross_cutting/_chapter_overviews.md`) bypass `NOTES-001/002` because
   the lint is path-prefix only. Either scrub to leave only the Vol.6
   co-citation, or replace with `[notes-citation removed per Critical Rule 9]`
   per the existing precedent.
2. Two style/role-map files (`17_cross_cutting/_style_guide.md` L151,
   `05_equity/_source_role_map.md` L53) teach the bare-alias citation pattern
   and admit a `primary-cfa` notes-derived stance respectively. Close these
   escape hatches at migration.

Additionally, extend the framework's lint with a bare-alias rejection regex
(e.g. `CFA_note_\d+\s*\(`) so the prefix check cannot be evaded.

### Phase 7 — Card-migration preflight (before any `cacg.v0` emission)

Generate `card_migration_queue.json` recording, per card:

- `legacy_source_paths` extracted from `Primary raw source:` +
  `Supporting sources:` + body `**Source:**` lines;
- `legacy_source_path → source_id` resolution via the `legacy_path_map.json`
  reverse index;
- `subcorpus_authorization` per `source_matrix.json`;
- `eligible_for_offset_quote_mapping` (resolves + authorized + not notes-taint
  candidate);
- `eligible_for_cacg_emission: false` (gated until coordinates verified and
  quote text bound to chunk_id + chunk_hash);
- `cacg_emission_blockers` list (e.g. `volume_page_offset_unverified`,
  `bare_alias_citation_present`).

Expected pre-scrub yield: 266–268 cards `ready_for_offset_and_quote_mapping`,
7–8 quarantined by notes-taint candidacy.

## 8. First Migration Slice After Bootstrap

Start with **`10_behavioral_finance`** because:

- 5 active cards on disk (smallest closed slice that isn't 0).
- Only 2 primary PDF anchors: `bf_shleifer_2000_inefficient_markets`,
  `econ_hart_mascolell_2013_simple_adaptive_strategies` (cross-vertical
  supporting from 02).
- No regulatory freshness sensitivity (theory-only sources).
- No notes-taint quarantine in this subcorpus.
- No combined-volume offset problem (no CFA L1 cards anchored here yet).
- Tests the full pipeline: source resolution → page-offset map → chunk
  binding → quote extraction → `cacg.v0` emission → `kb index`/`lint`/`verify`.

Coordinate question for the slice: Shleifer 2000 cards cite by printed-book
page; Hart/Mas-Colell 2013 cards reference front-matter overview material —
build verified `pdf_page` offset maps for both, then bind the 5 cards' quote
text to specific `chunk_id`s.

Then expand: 09 PM (24 cards, 2 PDFs, no regulatory issues but more cards)
→ 11 Risk (24 cards, 1 large PDF, McNeil/Frey/Embrechts) → 17 Cross-cutting
(after notes-taint scrub) → 01/02/05/03 → 06 FI → 08 CB (largest, most
heterogeneous) → 07 (no local PDFs; depends on shared anchors).

## 9. Why This Is the Right First Step

The hard dependency for trustworthy CACG cards is **real chunk manifests over
real PDFs**. The new framework can already lint, verify, index, search, and
show cards, but card migration without a clean source bootstrap either
produces synthetic chunks or binds citations to unstable legacy paths — that
would preserve the legacy weakness under a new schema. The bootstrap step:

- isolates the source corpus,
- normalizes filenames to ASCII canonical with stable `source_id`s,
- preserves every exclusion with explicit reason,
- works around the current CLI's lack of multi-source ingest merge,
- builds the `source_matrix.json` authorization layer that all downstream
  CLI commands require.

Once complete, card migration becomes a deterministic transform:

```
legacy path + page span → source_id → overlapping chunk_ids → chunk text →
verbatim quote selection (or notes-taint quarantine queue)
```

## 10. Risks and Open Questions for the Human

### Risks

1. **Page-coordinate translation** for legacy-printed-page vs PDF-index pages
   will require per-source offset tables. The CFA combined volumes need a
   per-volume table; many other books need a constant offset for front-matter.
2. **Quote rebinding** will create multiple chunks per legacy citation because
   page spans frequently overlap multiple ~350-token chunks. The migrator
   must select verbatim quote text from the chunk that maximally supports the
   paraphrased card claim.
3. **Pdfium binary provenance** is operationally load-bearing. The
   project's documented pin is `149.0.7825.0`. If the runner uses a different
   system Pdfium, chunk text and chunk hashes are tied to that runtime — full
   reproducibility requires either the pinned binary or recording the system
   library SHA-256 as provenance metadata.
4. **Regulatory-PDF freshness** (HKFRS 7, possibly HKAS 32, CSRC Standard 60,
   SSE compilation, HKEX excerpts) requires explicit freshness flags before
   final ingestion. Snapshot-with-`regulatory_status` field is the cheapest
   path; refresh-then-ingest is cleaner but blocks on acquisition.
5. **Behavioral Finance has only 5 of 15 planned cards** authored. The other
   10 are DAG-declared `(planned)`. The new framework should not gate on
   un-authored cards.
6. **07 Derivatives has 20 cards but zero local PDFs** — relies on shared
   anchors in `Convertible_Bonds/classic_pricing_books_english/`. The
   migration's `shared_anchors/` tree must be present before 07 card
   migration can proceed.
7. **Card schema gap**: legacy cards have zero verbatim quotes. The CACG
   `quote` field cannot be filled from legacy card bodies alone — the
   ingested chunk text is the only quote source.
8. **40+ FM006 stance values + per-vertical admit-sets** are deeply
   CFA-specific. The new framework either externalizes these as
   per-corpus rule config or accepts them as built-in checks.
9. **`kb scaffold-matrix` is a stub** — the migration's `source_matrix.json`
   author script effectively reimplements what scaffold-matrix would do;
   plan to upstream this.
10. **8 of the 9 Critical Rules port cleanly to the new framework as lint
    rules**; Rule 9 (notes/scripts hard-block) needs the bare-alias
    extension noted in Phase 6.

### Open questions

1. **Edition canonicalization**: do we adopt the on-disk Wahlen 10e (2023),
   Damodaran 2025 4e, Fabozzi 9e (2021), Tsay 2e (2005), ISLP (Python, 2023)
   as authoritative, replacing the matrix's older edition strings — or hunt
   the older editions the matrix names?
2. **Chinese licensing**: 4 Chinese books in `targeted_books_Chinese/` carry
   libgen/z-lib marks. Quarantine with `quotable: no`, replace with publisher
   editions, or drop entirely from migration?
3. **Portuguese Brooks Reversals**: quarantine pending English edition, or
   accept bilingual citation with `language: pt` flag?
4. **Combined-volume citation form**: persist both `volume_page` and
   `pdf_page` in cacg citations, or just one? If just one, which is canonical?
5. **Volume drafts disposition**: regenerate from new-framework cards (drop
   legacy drafts), preserve as snapshot, or port + re-link as the framework's
   first derived deliverables?
6. **08_CB 50-vs-51 drift**: which is canonical — manifest or on-disk? Identify
   the extra file and resolve before migration.
7. **Per-subcorpus FM006 stance vocabulary**: preserve inventory-exact
   admit-sets per vertical, or collapse to a flat global enum with
   cross-vertical auditing separate?
8. **`primary-cfa` admit-only-defensive pattern** (10_BF admits CFA L1 but no
   card uses it): preserve admissibility tracking, or migrate only active
   stances?
9. **Edge unification**: legacy cards mix `Repo touchpoints:` (untyped) and
   `edges:` (typed; 33 cards only) lists. Cacg `card_edges` allows only
   `depends_on` and `extends` — the legacy edge predicates need normalization
   or omission.
10. **30 unpushed local commits on `master`**: is the migration baseline the
    local HEAD or the remote tip? Affects snapshot provenance.
11. **Subcorpora 14/18/19**: 14 Microstructure has zero matrix rows; 18/19
    are outside the 01–17 grid. Retire from `DEFERRED_TOPICS.md`?
12. **Scope-filter inheritance**: should the new framework encode the
    2026-05-21 Chinese-CB filter as a first-class scope concept, or treat it
    as orthogonal config layered on a topic-neutral core?

---

Independent re-research complete. The next step (Phase B) is to compare this
plan against the existing bootstrap artifacts in `sources/cfa/`,
`out/cfa/`, `cards/cfa/`, and `_research/23-26` and produce a
refined plan for the remaining migration work.
