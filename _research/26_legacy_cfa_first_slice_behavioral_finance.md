# Legacy CFA First CACG Slice: Behavioral Finance

Date: 2026-05-28

## Scope

This note records the first actual CACG card emission from the legacy CFA
knowledge base. The slice is `10_behavioral_finance`, selected because the
preflight found 5 active cards, no notes-tainted cards, no current-version
source dependencies, and only two source-coordinate questions.

## Emitted Cards

Cards emitted under `cards/cfa_legacy/10_behavioral_finance/`:

- `be-limits-of-arbitrage`
- `be-noise-trader-equilibrium`
- `be-regret-matching-foundations`
- `be-sentiment-vs-fundamentals`
- `be-two-model-mispricing`

The batch has 8 total citations:

- 5 citations to `bf_shleifer_2000_inefficient_markets`.
- 3 citations to `econ_hart_mascolell_regret_matching`.

## Coordinate Verification

The source-coordinate map is stored at
`sources/cfa_legacy/_registry/page_coordinate_maps/10_behavioral_finance.json`.

For `bf_shleifer_2000_inefficient_markets`, legacy card pages are printed book
pages. The verified rule for this first slice is:

```text
pdf_page = legacy_page + 9
```

Evidence points:

- Legacy page 28 maps to PDF page 37.
- Legacy page 52 maps to PDF page 61.
- Legacy page 89 maps to PDF page 98.
- Legacy page 111 maps to PDF page 120.
- Legacy page 112 maps to PDF page 121.
- Legacy page 153 maps to PDF page 162.
- Legacy page 174 maps to PDF page 183.

For `econ_hart_mascolell_regret_matching`, the first-slice references are used
as physical PDF pages because the cited overview material is in the PDF
front-matter page window:

```text
pdf_page = legacy_page
```

Evidence points:

- PDF page 22 contains the regret-matching rule overview.
- PDF page 23 contains the Regret Matching Theorem overview.
- PDF page 35 contains the dynamics/equilibria overview.

## Generated Artifacts

- `sources/cfa_legacy/_registry/emit_behavioral_finance_first_slice.py`
- `sources/cfa_legacy/_registry/page_coordinate_maps/10_behavioral_finance.json`
- `sources/cfa_legacy/_registry/behavioral_finance_first_slice_citation_plan.json`
- `sources/cfa_legacy/_registry/behavioral_finance_first_slice_migration_report.json`
- `out/cfa_legacy/cards_manifest.json`
- `out/cfa_legacy/summaries.json`
- `out/cfa_legacy/INDEX.md`

## Validation

Commands run:

```bash
python3 -m py_compile sources/cfa_legacy/_registry/emit_behavioral_finance_first_slice.py
python3 sources/cfa_legacy/_registry/emit_behavioral_finance_first_slice.py --plan-only
python3 sources/cfa_legacy/_registry/emit_behavioral_finance_first_slice.py
env KB_FROZEN_CLOCK=1 target/debug/kb index cards --out out/cfa_legacy
env KB_FROZEN_CLOCK=1 target/debug/kb lint --all-readings --cards-dir cards --chunks-manifest out/cfa_legacy/chunks_manifest.json --source-matrix out/cfa_legacy/source_matrix.json
env KB_FROZEN_CLOCK=1 target/debug/kb verify <each emitted card> --chunks-manifest out/cfa_legacy/chunks_manifest.json --source-matrix out/cfa_legacy/source_matrix.json
```

Results:

- `kb index`: pass.
- `kb lint --all-readings`: pass.
- `kb verify`: pass for all 5 emitted cards.
- Second frozen `kb index` run was byte-stable for card files, history files,
  `cards_manifest.json`, `summaries.json`, and `INDEX.md`.

## Remaining Gates

This slice proves the first end-to-end migration path, but it does not remove
the global migration gates:

- The remaining 261 ready-bucket cards still need source-specific page
  coordinate maps and quote binding.
- The 8 notes-tainted active cards remain quarantined.
- CFA combined-volume cards still need a volume-to-merged-PDF offset map.
- Current-version and regulatory PDFs still need explicit freshness checks
  before card emission.

