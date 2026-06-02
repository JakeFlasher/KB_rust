# Legacy CFA Bootstrap Verification and Refined Plan

Date: 2026-05-28
Verification baseline: `_research/27_independent_legacy_cfa_audit_and_bootstrap_plan.md` (commit `ff90c01`)
Bootstrap-of-record: `_research/{23,24,25,26}_*.md` + on-disk artifacts under `sources/cfa/`, `out/cfa/`, `cards/cfa/`

This document is Phase B of a two-phase cross-check. Phase A (doc 27) produced
an independent plan from 13 read-only audits explicitly forbidden from reading
the prior bootstrap. Phase B (this doc) dispatched 8 verification subagents
that compared the existing bootstrap against the independent plan and the
actual disk state. Their findings drive the refined plan below.

## 1. Executive Summary

The existing bootstrap is **substantially correct and operationally sound**.
All 70 active PDFs are staged with SHA256-verified copies, ingested into 70
per-source directories, deterministically merged into `out/cfa/{sources,chunks}_manifest.json`
(70 sources, 57,603 chunks, byte-stable on rerun), authorized through a
cacg.v0-compliant `source_matrix.json` (16 reading_ids, 116 mappings), and
the first migration slice (`10_behavioral_finance`, 5 cards) is fully emitted
and passes `kb lint --all-readings` + per-card `kb verify` with zero
diagnostics.

The independent re-research surfaced **two material gaps** and **several minor
deviations**:

- **Material gap 1**: CFA L1 combined-volume offset map is **not built**.
  This blocks 168 active cards across 10 readings — every reading except 04
  (Corp Finance), 10 (BF, already done), 12 (Alt Inv), 13/15/16
  (Wealth/Performance/ESG). This is the single largest remaining blocker.
- **Material gap 2**: Notes-taint scrub is **not done** for the 7 bare-alias
  citations in `cc-material-info-and-dissemination-delay.md` + 1 in
  `17_cross_cutting/_chapter_overviews.md`, and the new framework's lint has
  no bare-alias rejection regex. The current preflight quarantine catches
  these by happy accident (broader manifest markers), not by defense-in-depth.

Minor deviations: `shared_anchors/` directory not created; Tsay 2e/3e edition
unflagged; Fabozzi 9e edition not in source_id token; `dynamic_tree_quarantine_manifest.json`
missing `node_modules/`, `.pytest_cache/`, `.humanize/`; Pdfium runtime is
`libpdfium-nojs 7778` not the documented pin `149.0.7825.0`; snapshot.json
lacks `legacy_git_unpushed_count` field and Merkle root.

**Recommended next slice**: `09_portfolio_management_and_asset_pricing` (24
cards, 2 PDFs, no regulatory issues, 1 quarantined). It exercises the
single-volume offset map path that the BF slice proved, before tackling the
multi-volume CFA L1 offset map.

## 2. Verification Findings — Per Subsystem

### 2.1 PDF staging — PARTIAL-PASS

**Confirmed (Agent A)**:
- 70/70 PDFs present under `sources/cfa/pdfs/` across 12 subdirs.
- 5/5 spot-checked SHA256 values match across disk ↔ legacy_path_map ↔ matrix.
- 0/70 `source_id` violations of `^[a-z0-9][a-z0-9_]*$`; 0 duplicates.
- All 70 staged filenames are ASCII-only snake_case.
- All known exclusions (28 deferred; 2 EPUBs; SCAN-only Wooldridge; Portuguese
  Brooks) confirmed absent from `pdfs/`.

**Deviations from doc 27**:
| Item | Status | Severity |
|---|---|---|
| `shared_anchors/` subdir for cross-vertical Hull/Glasserman/Lando | Missing; placed under `convertible_bonds/` instead | Medium — affects 07_derivatives migration |
| `07_derivatives_and_volatility/` subdir | Missing | Low — doc 27 said "may be empty" |
| Tsay AFTS edition correction (2e 2005 vs 3e 2010) | Not applied; bootstrap kept matrix's `2010` | Medium — citation-page risk |
| Fabozzi 9e (2021) edition in source_id | `fi_fabozzi_handbook_fixed_income_securities` lacks `_9e_2021` token | Low — inconsistent with other corrected files |
| Edition suffix convention | Bootstrap uses `_8ed/_10ed`; doc 27 examples use `_8e/_10e` | Cosmetic |
| Brooks prefix | Bootstrap uses `tpa_`; doc 27 said `brooks_` | Cosmetic |

### 2.2 Registry and source matrix — PASS (3 minor deviations)

**Confirmed (Agent B)**:
- All 11 doc-27-required registry files present in `sources/cfa/_registry/`.
- `out/cfa/source_matrix.json` is byte-identical to the registry copy and
  fully satisfies the cacg.v0 `SourceMatrix` schema in
  `crates/cacg-core/src/schema.rs:758-799`: literal `schema_version: "cacg.v0"`,
  non-empty reading_id keys, non-empty unique source_id lists, zero regex
  violations, zero orphans vs `source_inventory.json`.
- 16 reading_ids: `01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 15, 16, 17`.
- 116 total reading × source_id mappings across 70 unique source_ids.
- `excluded_sources.json` has 42 rows distributed: 36 deferred + 2 epub_blacklist
  + 2 notes_user_volatile + 1 scan_nonquotable + 1 unregistered_pdf (Brooks Portuguese).

**Deviations from doc 27**:
- Reading_id set is 16, not 11 — bootstrap added `04, 12, 13, 15, 16`. Defensible:
  the matrix preamble pre-authorized CFA L1 for 04 and 12 via v12 Track-B (a),
  and L2/L3 are admit-only-defensive for 13/15/16. Doc 27 considered only the
  11 readings with active cards.
- Bootstrap does not use a shared `cfa_program_curriculum` namespace — inlines
  `cfa_2022_l1_combined`, `cfa_2023_l2_combined`, `cfa_2022_l3_combined` as
  per-reading source_ids.
- `snapshot.json` lacks an explicit `legacy_git_unpushed_count` field (the 30-
  commit unpushed gap noted in doc 27 is not asserted by the snapshot) and has
  no Merkle root over the inventoried files.

### 2.3 Ingest output (manifests) — PASS

**Confirmed (Agent C)** by computational re-check, not eyeball:
- 70 per-source ingest dirs, all complete (both sources + chunks manifests present).
- Merged `sources_manifest.json`: 70 sources, all unique, all matching
  `legacy_path_map.json`, all `extracted_at: "1970-01-01T00:00:00Z"`, all
  `parser_name: "pdfium-render"`, `parser_version: "0.9.1"`, all
  `source_sha256` 64-hex lowercase, sorted by source_id.
- Merged `chunks_manifest.json`: 57,603 chunks, all `chunk_id` unique, all
  referencing valid sources, every source has chunks, 0 schema violations on
  `deny_unknown_fields`, all `chunk_hash` 64-hex, all `page_spans` monotonic
  and within `[start_page, end_page]`, retracted arrays empty.
- 200-random-sample chunk hash recomputation: 0/200 mismatches against the
  canonical-JSON envelope `{end_page, page_spans, start_page, text}`.
- 3/3 source SHA256 cross-check: manifest = pathmap = on-disk PDF.
- Live re-run of `merge_ingest_manifests.py` returns `unchanged` for both
  manifests. Independent byte-for-byte reconstruction matches on-disk
  (sources: 26,700 B; chunks: 144,654,624 B).
- Max ordinal 6,409 < 9999 ceiling. Safe.
- Chunk-count distribution: min 1, max 6,410, median 472, mean 823. Top 5 are
  the three CFA combined volumes plus Kieso and Fabozzi.

**Advisory items (not failures)**:
- 15 outlier chunks in `qm_eslii_2009_2ed` exceed 5,000 tokens (max 15,483 on
  2-page span). Likely table/equation pages the chunker couldn't subdivide.
  Citation snippets from these chunks will be unwieldy.
- `chunk_id` width is 3-4 digits for page numbers (3 for pp 1-999, 4 for ≥1000).
  Spec wording in doc 23/27 implied a fixed 3-digit width via `:p<NNN>:`.
  Downstream regex consumers should expect `:p\d{3,}:`. Documentation gap, not
  a bug — uniqueness holds.

### 2.4 First-slice BF cards — PASS

**Confirmed (Agent D)** by running the framework's own gates and quote-by-quote
verbatim verification:
- 5/5 cards + 5/5 sidecars present.
- 5/5 frontmatters fully cacg.v0-compliant (`schema_version`, `id`, `title`,
  `reading_id`, `summary` ∈ [80, 400], `citations` ≥ 1 with valid `chunk_id`/
  `chunk_hash`/`page_range`/`quote`/`edge_type`, valid `tags`, `card_hash`).
- 8/8 citations: chunk_hash matches manifest; quote substring appears verbatim
  in chunk's `text` (NFC-equivalent); page_range overlaps chunk's `[start_page,
  end_page]`. Includes Unicode curly quotes in `be-sentiment-vs-fundamentals`
  cit[1] — handled correctly.
- 5/5 cards authorized: only cite source_ids in `source_matrix.json` for
  `10_behavioral_finance`.
- `kb lint --all-readings`: exit 0, empty stdout.
- `kb verify` per card: exit 0 × 5; `layer1: true, layer2: true` in
  `lint_journal.jsonl`.
- `cards_manifest.json` has 5 entries with valid `card_hash` and
  sorted-unique `source_ids`; `summaries.json` matches; `INDEX.md` lists
  all 5 with matching truncated hashes.
- Card bodies migrate byte-identical (modulo frontmatter and trailing-newline
  normalization) from legacy authoring.

**Edge note**: `be-limits-of-arbitrage` summary is exactly 400 characters
(truncated with `...` suffix). One byte over would have produced
`CACG-SUM-002`. The migrator handled the limit correctly.

### 2.5 Notes-taint handling — TAINT-PARTIALLY-HANDLED

**Confirmed (Agent E)**:
- All 8 notes-tainted active cards are correctly in `quarantine_notes_taint`
  bucket in `card_migration_queue.json`.
- `legacy_notes_taint_manifest.json` uses 19 markers including substring
  `CFA_note`, which catches the bare-alias `CFA_note_2 (2026 OCR)` *de facto*.
- `cc-material-info-and-dissemination-delay.md` is in the 8-card quarantine
  because of this manifest, not because the legacy lint rules
  `NOTES-001/002` caught it.

**Confirmed gaps**:
- `cc-material-info-and-dissemination-delay.md` still contains 7 bare-alias
  citations (L26, L38, L88, L99, L115, L128, L172). `17_cross_cutting/_chapter_overviews.md`
  contains 1 more at L222. **None are scrubbed.**
- Policy escape hatches **not scrubbed**:
  - `17_cross_cutting/_style_guide.md` L151: teaches the bare-alias citation
    pattern (`**Source:** CFA_note_2 (2026 OCR) pp.<P-Q>`).
  - `05_equity/_source_role_map.md` L53: admits a `primary-cfa` notes-derived
    stance.
  Both are `_*.md` files excluded from `active_card_paths()` (which skips
  underscore-prefixed names) and only flagged as `migration_metadata_review`,
  not actually scrubbed.
- **No bare-alias regex in the new framework's lint.** The cacg-core lint
  rules do not check for `CFA_note_\d+\s*\(` patterns. If the upstream taint
  manifest were ever regenerated against a different marker set, the alias
  could slip through. No defense-in-depth.

### 2.6 Excluded set completeness — INCOMPLETE (low severity)

**Confirmed (Agent F)** for source-file exclusions: every PDF/EPUB on legacy
disk is accounted for in either `legacy_path_map.json` (70 active) or
`excluded_sources.json` (42 excluded). Clean bijection. 0 overlap.

**Gaps in `dynamic_tree_quarantine_manifest.json`**:
| Tree | Files | In manifest? |
|---|---:|---|
| `.claude/dist*` | 272 | YES |
| `.claude/kb_manifest.*` | 2 | YES |
| `.claude/retracted_cards.json` | 1 | YES |
| `__pycache__` | 91 | YES |
| `node_modules/` | 171 (17 MB) | **NO — entire tree missing** |
| `.pytest_cache/` | 5 | **NO** |
| `.humanize/{rlcr,skill,ideas,discovery_rounds,codex-*}` | 1,397 (12 MB) | **NO — entire subtree missing** |

Total unrecorded: ~1,573 files. As a downstream effect, 6 HTML files under
`node_modules/playwright-core/lib/vite/` are the only "source-like" extensions
on legacy disk that escape every manifest. They are runtime/tooling files, so
no active CACG source is at risk of contamination, but the quarantine manifest
fails to assert what doc 27 expected.

**Doc 27 has a stale count**: lines 84 and 171 say "28 deferred + 1 HTML";
actual is 36 PDFs + 1 HTML. The manifests are correct; the plan figure is
wrong (likely a partial-pivot count from an earlier corpus state).

### 2.7 Page-coordinate offset maps — INCOMPLETE

**Confirmed (Agent H)**:
- `sources/cfa/_registry/page_coordinate_maps/` contains exactly **one
  file**: `10_behavioral_finance.json` with 2 source entries.
- `page_offset_worklist.json` has 63 sources across 4 buckets:

| Bucket | Sources | Cards blocked | Occurrences |
|---|---:|---:|---:|
| `build_cfa_volume_to_pdf_offset_map` | **1** (`cfa_2022_l1_combined`) | **168** | **216** |
| `verify_book_to_pdf_page_offset` | 40 | 349 | 430 |
| `verify_current_version_and_page_mapping` | 6 | 10 | 13 |
| `verify_pdf_page_mapping_for_regulatory_or_research_pdf` | 16 | 34 | 34 |

The **single highest-leverage gap** is the CFA L1 combined-PDF offset table.
168 cards across 10 readings cannot migrate until it exists.

CFA L2 (3,369 pp) and CFA L3 (3,863 pp) are ingested but cited by **zero
active legacy cards** — their offset tables are NOT required to unblock
current card migration. Build only on demand if L2/L3 cards are ever authored.

The existing offset-map schema (`cfa.page_coordinate_map.v1`) carries
`pdf_coordinate_rule: "pdf_page = legacy_page + N"` as a single global affine
transform per source. For CFA L1 this needs extension to a **per-volume table**
because the volume boundaries reset the printed page number. Either (a) array
of `{volume, legacy_page_start, legacy_page_end, pdf_page_start, pdf_page_end}`
entries per source, or (b) one source-row per volume with the volume number
encoded in the source_id or sub-key.

### 2.8 Doc 27 vs docs 23-26 gap analysis (synthesis)

**Confirmed by Agent G**:

#### Agreement (load-bearing)

- Source-corpus bootstrap before card migration.
- 70 active quotable PDFs from a 73-row matrix.
- `source_id` regex `^[a-z0-9][a-z0-9_]*$`.
- `kb ingest` single-PDF + non-empty-`out/` refusal forces external merge.
- No verbatim quotes anywhere in legacy cards.
- Behavioral finance as first slice (5 cards, no taint, no regulatory).
- Combined-volume offset problem isolated to CFA L1/L2/L3.
- Notes-taint quarantine of exactly 8 active cards.
- Determinism gate (`KB_FROZEN_CLOCK=1`, atomic writes, sorted/compact JSON,
  Pdfium SHA-pin).

#### Divergences in fact

- **08_CB card count**: doc 23 says 50, doc 27 says 51 on disk vs 50 in
  manifest. Resolution: the unprefixed `README.md` in `08_convertible_bonds/`
  inflates the disk count by 1 if `*.md` is counted naively. Real card count
  is **49 `cb-*.md` cards + 1 `README.md` ≠ card**. The manifest 50 is
  consistent with 49 cards + 1 README (or 50 cards if README is treated as a
  card). The doc 23/24 figure of 50 silently elides the discrepancy.
- **Tsay edition**: doc 27 claims 2e 2005; bootstrap kept matrix's 2010
  view. Not corrected.
- **ISLP, ESL editions**: doc 27 flagged as 2023 Python (vs 2021 R) and ESL
  2e corrected 12th printing 2017. Bootstrap silently inherited matrix names
  via `qm_islp_2023_python` (Python tag in source_id is correct) and
  `qm_eslii_2009_2ed` (2009 copyright year preserved; matrix metadata was
  already correct).
- **Hart/Mas-Colell**: bootstrap source_id is `econ_hart_mascolell_regret_matching`
  (no year); doc 27 convention requires `econ_hart_mascolell_2013_simple_adaptive_strategies`.
- **Deferred-books count**: doc 27 says 28 + 1 HTML; actual on disk is 36 + 1
  HTML; bootstrap manifest is correct.

#### Divergences in recommendation

- `source_id` naming convention: doc 27 is more aggressive (`{prefix}_{author}_
  {year}_{title}_{edition}` mandatory); bootstrap is inconsistent — some IDs
  include year+edition (Hull, Wahlen, Damodaran), others don't (Fabozzi,
  Hart/Mas-Colell, HKICPA where edition isn't part of the natural name).
- `shared_anchors/` directory: doc 27 mandates; bootstrap omits.
- Bare-alias notes lint extension: doc 27 mandates; bootstrap doesn't.
- `_legacy_reference/` separate sibling tree: doc 27 prescribes; bootstrap
  folds governance docs into `legacy_content_manifest.json`.
- Migration order post-BF: doc 27 prescribes a full sequence; doc 25 stops
  after recommending BF.

#### Scope items raised only by doc 27

- 30 unpushed local commits + dirty `.humanize/` as a migration baseline question.
- 40+ FM006 per-vertical stance vocabulary as a CFA-specific governance surface.
- Subcorpora 14 Microstructure + 18/19 as schema-grid cleanup.
- Chinese-CB scope-filter as a first-class concept.
- Skill routers + `.humanize/bitlesson.md` as carry-forward candidates.

## 3. Overall Verdict on the Bootstrap

The bootstrap is **production-quality for the source-corpus layer**. Every
operational claim in docs 24-26 verifies under independent re-check
(hash-by-hash, count-by-count, lint+verify exit codes, byte-stable merge
reruns). The first slice exists and works.

It is **not yet production-quality for the multi-vertical card layer**. Three
classes of remaining work are required before continuing past BF:

1. **Blocker**: CFA L1 combined-volume offset map (168 cards across 10
   readings).
2. **Quality gate**: notes-taint scrub of 8 alias citations + 2 policy
   files + new framework lint rule.
3. **Hygiene**: stale-count and naming inconsistencies + missing quarantine
   trees + Pdfium pin clarification.

## 4. Refined Plan — Highest-Priority Fixes Before More Card Migration

Each item below has a concrete artifact + acceptance criterion.

### 4.1 Build the CFA L1 combined-volume offset table — HIGHEST LEVERAGE

Artifact: `sources/cfa/_registry/page_coordinate_maps/cfa_2022_l1_combined.json`

Schema extension to `cfa.page_coordinate_map.v2`:

```json
{
  "source_id": "cfa_2022_l1_combined",
  "legacy_coordinate": "printed_volume_page",
  "pdf_coordinate_rule": "per_volume_offset",
  "volume_table": [
    {"volume": 1, "first_legacy_page": 1, "last_legacy_page": 598,
     "first_pdf_page": 7, "pdf_page_offset": 6,
     "verified_evidence": [
       {"legacy_volume": 1, "legacy_page": 1, "pdf_page": 7, "evidence": "..."},
       {"legacy_volume": 1, "legacy_page": 300, "pdf_page": 306, "evidence": "..."}
     ]},
    {"volume": 2, ...},
    ...
  ]
}
```

Build steps:
1. Identify each volume's first PDF page by scanning the 4,353-page PDF for the
   publisher boilerplate `"CFA® Program Curriculum / 2022 • LEVEL I • VOLUME N"`
   cover page text. The text appears at exactly one location per volume.
2. For each volume, anchor `(legacy_page, pdf_page)` at the start, one mid-volume
   reference, and the end (3 evidence points minimum per volume; 6 volumes ×
   3 = 18 evidence points).
3. Implement conversion functions: `vol_page_to_pdf_page(vol, page, map)` and
   inverse. Both unit-tested with frozen-clock determinism.

**Acceptance**: every cited (volume, page) in the 168 blocked cards round-trips
through the table; absolute deviation from a sampled-then-verified pdf page is 0.

### 4.2 Notes-taint scrub + lint extension

Three sub-artifacts:

1. **Scrub `cc-material-info-and-dissemination-delay.md`** in the
   `_legacy_reference/` copy (do NOT edit the legacy KB tree itself). For each
   of the 7 `**Source:** CFA_note_2 (2026 OCR) pp.115-116` lines (L26, L38,
   L88, L99, L115, L128, L172), retain only the paired `CFA_Program_Curriculum
   /.../Vol.6/pp.343-354` co-citation. Same for L222 of
   `17_cross_cutting/_chapter_overviews.md`.
2. **Strip policy escape hatches**:
   - `17_cross_cutting/_style_guide.md` L148-158: remove the `For the notes PDF:`
     example block.
   - `05_equity/_source_role_map.md` L52-53: remove the parenthetical that
     admits `primary-cfa` notes-derived stance.
3. **Add new framework lint rule**: in `crates/cacg-core/src/verify/` (or
   wherever the lint suite lives), add a `CACG-CITE-NOTES-ALIAS` rule with
   regex `\bCFA_note_\d+\s*\(` matched against (a) frontmatter `Primary raw
   source:` / `Supporting sources:` values, and (b) body `**Source:**` /
   `**Sources:**` lines. Failing cards get a non-zero `verify` exit.

**Acceptance**: zero `CFA_note_\d+\(` matches in the scrubbed
`_legacy_reference/` copy; `kb verify` rejects a test fixture card that
contains the alias; doc 25's 8-card quarantine reduces to 7 because
`cc-material-info-and-dissemination-delay.md` becomes citation-clean.

### 4.3 Resolve 08_CB 50-vs-51 drift

Artifact: a one-line decision in `card_migration_preflight_summary.md` (or
similar) recording that the unprefixed `08_convertible_bonds/README.md` is NOT
a card and is preserved as documentation only in the legacy reference bundle.
Audit other subcorpora for unprefixed Markdown:

```
find /home/jakeshea/CFA_reading/CFA_reading/.claude/knowledge -maxdepth 2 -type f \
  -name '*.md' ! -name '_*' ! -name 'INDEX.md' | xargs -I{} basename {} \
  | grep -v ^[a-z][a-z]-
```

**Acceptance**: every `.md` file under `.claude/knowledge/<NN>_*/` is in
exactly one of: active-cards, aux-files (`_*`), or docs (`README.md` /
`INDEX.md`). Counts reconcile across all 3 categories.

### 4.4 Stage `shared_anchors/` for cross-vertical PDFs

Artifact: `sources/cfa/pdfs/shared_anchors/` containing canonical copies
(or symlinks) of:
- `cb_hull_2022_options_futures_derivatives_11ed.pdf`
- `cb_glasserman_2003_monte_carlo_methods.pdf`
- `cb_lando_2004_credit_risk_modeling.pdf`
- `cb_koziol_2004_valuation_strategic_investors.pdf` (also used by 06)

Update `legacy_path_map.json` to redirect their canonical paths from
`convertible_bonds/` to `shared_anchors/`. Update `out/cfa/source_matrix.json`
to authorize each `cb_hull_...` source for both `07_derivatives_and_volatility`
and `08_convertible_bonds`. Per-source ingest dirs and chunks need no change
(source_ids stay the same).

**Acceptance**: 07_derivatives cards can resolve to Hull/Glasserman without
crossing into `convertible_bonds/` namespace; source authorization remains
correct.

### 4.5 Fix `source_id` naming inconsistencies

Three specific renames:

1. `econ_hart_mascolell_regret_matching` → `econ_hart_mascolell_2013_simple_adaptive_strategies`
   (confirmed 2013 World Scientific book identity).
2. `fi_fabozzi_handbook_fixed_income_securities` →
   `fi_fabozzi_2021_handbook_fixed_income_9e` (per matrix's
   `metadata_corrected` flag).
3. `qm_tsay_2010_afts` — verify on-disk edition first; if 2e (2005), rename to
   `qm_tsay_2005_afts_2e`. Add `metadata_corrected` `rename_decisions.md` entry.

These break the 5 already-emitted BF cards if the 5 cite `econ_hart_mascolell_regret_matching`
— they do (3 of the 8 citations). The rename must be **coordinated**: update
the 3 citations in 5 cards' frontmatter, regenerate `card_hash`, regenerate
`cards_manifest.json` + `summaries.json` + `INDEX.md` via `kb index`,
re-verify. Frozen-clock keeps it byte-stable.

**Acceptance**: all source_ids encode year + edition where the corrected
metadata is known; the 5 BF cards still pass `kb lint`/`verify` after rename.

### 4.6 Extend `dynamic_tree_quarantine_manifest.json`

Add three entries:
- `node_modules/` (171 files, `legacy_tooling_not_source`)
- `.pytest_cache/` (5 files, `generated_runtime_artifact`)
- `.humanize/` (1,397 files, `legacy_runtime_artifact`) — directory-level
  entry is fine; per-file enumeration is optional.

**Acceptance**: every `.html` / `.pdf` / `.md` / `.json` under legacy root is
in exactly one of: active source, excluded source, dynamic quarantine, or
notes inventory.

### 4.7 Clarify Pdfium runtime constraint

Artifact: add `re_ingest_blockers` field to `out/cfa/sources_manifest.json`
(or a sibling `provenance.json`) stating:

```json
{
  "pdfium_runtime_pin": {
    "package": "libpdfium-nojs 7778.r8.72ea487e43-1",
    "library_path": "/usr/lib/libpdfium.so",
    "library_sha256": "c110f5240692b1915ad090d4f7e9bc6afa429db41341f2339d866d48402edbe5",
    "documented_canonical_pin": "149.0.7825.0",
    "documented_canonical_sha256": "fcd602cd518476d712f661b08e010700490875288fb17069b5b5a2f8b7724118",
    "warning": "chunk_hash and chunk text bytes are tied to this runtime. Re-ingest on any other host requires the same libpdfium-nojs build OR a switch to the documented canonical pin (and full re-ingest)."
  }
}
```

OR: install Pdfium `149.0.7825.0` and re-ingest all 70 sources before more
cards are emitted. (Higher cost, but produces a more reproducible corpus.)

**Acceptance**: provenance is explicit; any operator running `kb verify` on a
different host can see the constraint.

### 4.8 Snapshot completeness

Update `sources/cfa/_registry/snapshot.json` to add:
- `legacy_git_unpushed_count: 30` (verifiable via `git -C ... log
  origin/master..HEAD | wc -l`)
- `legacy_content_merkle_root` over the recursive SHA256 of all
  `legacy_content_manifest.json` entries (single 64-hex digest)

**Acceptance**: snapshot captures the full provenance triple (HEAD, push
state, content Merkle) for future drift detection.

## 5. Refined Migration Sequence After Fixes

Doc 25 recommended starting with BF (done). The next slices, in priority order
once the highest-priority fixes are landed:

### 5.1 Immediate (no CFA L1 dependency)

1. **09_portfolio_management_and_asset_pricing** — 24 cards, 2 single-volume
   PDFs (Cochrane 553 pp, Pedersen 369 pp). 1 quarantined
   (`pm-tracking-error-and-active-risk` notes-taint). 23 emittable. Proves
   the single-volume offset path at scale before tackling L1.
2. **11_risk_management** — 24 cards, 1 single-volume PDF (McNeil/Frey/Embrechts
   QRM 721 pp). 5 quarantined (parametric/historical/MC VaR + sensitivity-vs-
   simulation + risk-objectives). 19 emittable. Confirms single-source
   handling for a large PDF with broad page coverage.

### 5.2 After CFA L1 offset table

3. **17_cross_cutting** — 16 cards. 1 quarantined (`cc-material-info-and-
   dissemination-delay`). After 4.2 scrub, all 16 become eligible. Mixes CFA
   L1 + Ghosh-case material; first slice that uses the L1 offset.
4. **01_quantitative_methods** — 17 cards, 6 single-volume PDFs (Tsay, ESL,
   ISLP, Wooldridge Intro, Greene, Multivariate Tsay). All CFA L1 anchored
   for the QM core. Uses L1 offset + 6 single-volume offsets.
5. **02_economics** — 29 cards, 5 single-volume PDFs (MWG 1001 pp is large but
   single-volume; Romer; Cochrane FTPL; Hart/Mas-Colell; MasColell papers).
   1 quarantined.
6. **05_equity** — 24 cards, 1 single-volume PDF (Damodaran 2025 4e, 1356 pp).
   Needs the citation-coordinate convention (Damodaran calibre-rebuild =
   PDF-page only, no print-page parity). Resolve open-q #3 from doc 27
   (chapter+section vs page-N) before emission.
7. **03_financial_reporting_analysis** — 29 cards, 3 HKICPA standards + 5
   textbooks. Needs (a) regulatory freshness decisions for HKFRS 7 (stale),
   HKAS 32 (possibly stale), HKFRS 9 (current), and (b) Wahlen 10e (already
   renamed). Single-volume offsets only.

### 5.3 After `shared_anchors/` + bulk verification

8. **06_fixed_income_and_credit** — 36 cards, 7 single-volume PDFs (Brigo,
   Crepey, Duffie/Singleton, Davidson/Levin, Veronesi, Fabozzi 9e, Tuckman 3e).
   Also relies on shared Hull/Lando from `shared_anchors/`. Largest non-CB
   single-source pool.
9. **08_convertible_bonds** — 49 cards (per disk count excluding README), 35
   authorized sources (largest set), mixes Chinese-language and Brooks Trading
   sources. Requires CN-language section-marker parser (`§五-§十`,
   `§三十九-§四十`) plus PDF/EPUB-pair decisions for 攻守, plus libgen-source
   licensing decision for 安道全 books. Largest and most heterogeneous; last
   among heavy verticals.
10. **07_derivatives_and_volatility** — 20 cards, zero local PDFs. Cites only
    `shared_anchors/` (Hull, Glasserman). Final slice; lowest-risk if 8 worked.

## 6. Process Improvements From the BF Slice

- **Generalize the per-vertical emitter.** `emit_behavioral_finance_first_slice.py`
  is per-vertical. Promote to `emit_subcorpus_slice.py --reading-id <NN>_<name>`
  driven by the queue + coordinate maps + chunks_manifest. Subsequent slices
  need only the coordinate maps and not new emitter code.
- **Promote the page_coordinate_maps schema to v2** to support per-volume
  affine transforms (required for CFA L1; degenerates to single-rule for
  single-volume PDFs).
- **Add edge normalization to the emitter.** Doc 25:113-115 noted CACG
  `card_edges` allows only `depends_on`/`extends` while legacy uses untyped
  `Repo touchpoints` and typed `edges:`. The BF slice avoided this; the next
  slice (`09_pm`) must address it. Choose: (a) drop, (b) map legacy predicates
  to cacg vocabulary where possible (`applies_to` and `prerequisite_for` are
  already in `CitationEdge` but not `CardEdgeType`), or (c) omit `card_edges`
  entirely and rely on `card_edges`-only-from-explicit-`edges:`.
- **Add a per-slice `unblocked_cards.json` checkpoint** after each fix lands,
  so progress is auditable cumulatively.
- **Capture coordinate-verification evidence per PDF once, not per card.**
  The BF map records 7 evidence points for Shleifer used across 5 cards. For
  Penman (21 cards) or McNeil (22 cards), the front-loaded offset table will
  amortize verification cost.
- **Treat `_legacy_reference/` as a real sibling tree** per doc 27:330-334
  instead of folding it under `sources/cfa/`. The current bootstrap
  conflates "sources to migrate" with "legacy governance docs to consult";
  separating them clarifies data flow.
- **Update doc 27** to fix the stale "28 + 1 HTML" deferred count to "36 + 1
  HTML" (the manifests are right; the plan figure is wrong).

## 7. Open Questions Carried Forward From Doc 27

These remain unresolved and gate decisions for slices 4-10:

1. **Edition canonicalization**: adopt on-disk Wahlen 10e (2023), Damodaran
   2025 4e, Fabozzi 9e (2021), Tsay 2e (2005), ISLP (Python, 2023) as
   authoritative? Bootstrap has done 2 (Wahlen, Damodaran), partially done 1
   (Fabozzi: matrix flagged but source_id token missing), not done 1 (Tsay).
2. **Chinese licensing**: 4 Chinese books in `targeted_books_Chinese/` carry
   libgen/z-lib marks. Quarantine `quotable: no`, replace with publisher
   editions, or drop entirely? Affects 08_CB and 20_China_CB migrations.
3. **Portuguese Brooks Reversals**: quarantine pending English edition, or
   accept bilingual citation with `language: pt` flag? Currently excluded as
   `unregistered_pdf` with `language_quality_risk` flag.
4. **Combined-volume citation form**: persist both `volume_page` and `pdf_page`
   in cacg citations, or just one?
5. **Volume drafts disposition**: regenerate from new-framework cards (drop
   legacy drafts), preserve as snapshot, or port + re-link?
6. **08_CB 50-vs-51 drift**: confirmed as unprefixed README.md. Migrate as
   docs-only?
7. **Per-subcorpus FM006 stance vocabulary**: preserve inventory-exact admit-
   sets, or collapse to global enum?
8. **`primary-cfa` admit-only-defensive pattern**: preserve, or migrate only
   active stances?
9. **Edge unification**: cacg `card_edges` is narrower than legacy. Drop, map,
   or omit?
10. **30 unpushed local commits**: migration baseline is local HEAD or remote
    tip?
11. **Subcorpora 14/18/19**: retire from `DEFERRED_TOPICS.md`?
12. **Scope-filter inheritance**: encode Chinese-CB filter as first-class
    scope concept, or orthogonal config?

## 8. Verification Summary Table

| Subsystem | Verdict | Material gaps | Minor deviations |
|---|---|---|---|
| PDF staging | PARTIAL-PASS | None | Tsay edition; Fabozzi source_id; `shared_anchors/`; naming convention drift |
| Registry + matrix | PASS | None | 16 reading_ids (not 11); snapshot field gaps |
| Ingest output (manifests) | PASS | None | Pdfium pin gap; outlier chunks in qm_eslii |
| First-slice BF cards | PASS | None | None |
| Notes-taint handling | TAINT-PARTIALLY-HANDLED | No bare-alias lint rule; 7 alias citations not scrubbed | Style-guide/role-map escape hatches not closed |
| Excluded set | INCOMPLETE | `dynamic_tree_quarantine_manifest.json` missing 3 trees | Doc 27 has stale "28 deferred" count |
| Page-coordinate maps | INCOMPLETE | CFA L1 offset table not built (168 cards blocked) | 61 of 63 single-source maps still TODO |
| Doc 27 vs docs 23-26 | (synthesis) | Migration order post-BF; bare-alias scrub | 6 cosmetic naming drifts; 6 doc-27-only scope items |

---

Phase B verification complete. The bootstrap is operationally sound; the
remaining work is the CFA L1 offset table (highest leverage), the notes-taint
scrub (lowest cost), and the sequential card-migration slices in the
prioritized order in §5.
