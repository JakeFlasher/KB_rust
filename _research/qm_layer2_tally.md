# QM Vertical — Layer-2 Per-Citation Tally

_captured against HEAD `1fe8ee9021e4` on `2026-05-23`_

Captured by `scripts/qm_vertical_layer2_tally.py` over the
17 cards under `tests/parity_corpus/cards/reading_01_qm/`
verified against `tests/parity_corpus/out_python/qm_vertical/chunks_manifest.json`.
Each citation is run through `cacg.verify.layer2.verify_citation`
first with `fuzzy_enabled=False` (strict substring match), then
again with `fuzzy_enabled=True` on any strict-fail; the verdict
buckets the citation into `strict` / `fuzzy` / `fail`.

## Aggregate

- Total citations: **391**
- STRICT pass: **391** (100.0%)
- FUZZY pass:  **0** (0.0%)
- FAIL:        **0** (0.0%)

## Per-card breakdown

| card_id | citations | strict | fuzzy | fail |
|---------|-----------|--------|-------|------|
| `qm-aic-bic-model-selection` | 3 | 3 | 0 | 0 |
| `qm-anova-table` | 2 | 2 | 0 | 0 |
| `qm-arch-conditional-heteroskedasticity` | 3 | 3 | 0 | 0 |
| `qm-cb-arb-factor-construction` | 72 | 72 | 0 | 0 |
| `qm-decision-trees-and-roots` | 3 | 3 | 0 | 0 |
| `qm-goodness-of-fit-r2-adj-r2` | 2 | 2 | 0 | 0 |
| `qm-influence-analysis-leverage` | 2 | 2 | 0 | 0 |
| `qm-multiple-linear-regression-foundations` | 1 | 1 | 0 | 0 |
| `qm-panel-cb-factor-inference` | 98 | 98 | 0 | 0 |
| `qm-penalized-regression-lasso` | 2 | 2 | 0 | 0 |
| `qm-projection-and-dimensionality-reduction` | 3 | 3 | 0 | 0 |
| `qm-regression-assumption-violations` | 4 | 4 | 0 | 0 |
| `qm-regression-hypothesis-tests` | 2 | 2 | 0 | 0 |
| `qm-signal-validation-oos-discipline` | 49 | 49 | 0 | 0 |
| `qm-structured-data-ml` | 3 | 3 | 0 | 0 |
| `qm-time-series-foundations` | 4 | 4 | 0 | 0 |
| `qm-volatility-model-garch-multivariate` | 138 | 138 | 0 | 0 |

## Per-source breakdown

| source_id | citations | strict | fuzzy | fail |
|-----------|-----------|--------|-------|------|
| `qm_afts_trim` | 138 | 138 | 0 | 0 |
| `qm_eslii_ch3_trim` | 72 | 72 | 0 | 0 |
| `qm_eslii_ch7_trim` | 49 | 49 | 0 | 0 |
| `qm_greene_trim` | 98 | 98 | 0 | 0 |
| `qm_notes_trim` | 34 | 34 | 0 | 0 |

## AC-8 contract

AC-8 mandates that the tally be captured, NOT that the
pass rate exceed any threshold. Layer-2 failures driven
by legitimate paraphrase (the migration script extracts
a short verbatim sentence from each chunk's text, but
the card's prose paraphrases that content; only the
citation `quote` is a verbatim substring) are expected.
The detailed analysis lives in
`_research/18_cfa_real_migration_findings.md` (task-m4-10).

