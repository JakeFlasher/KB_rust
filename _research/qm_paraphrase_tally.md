# QM Vertical — Paraphrase Shadow Layer-2 Tally

_captured against HEAD `1fe8ee9021e4` on `2026-05-23`_

This is the COMPANION measurement to
`_research/qm_layer2_tally.md` and addresses Round-19
review P2-B (the primary tally's 100 % strict pass rate
is structurally trivial — it measures only that the
migration script's verbatim-substring quotes are still
verbatim substrings).

This shadow tally uses the LEGACY card's prose preceding
each `**Source:** <pdf> pp.<range>` annotation as the
paraphrased claim, then runs `cacg.verify.layer2.verify_citation`
strict-then-fuzzy against every chunk in the merged QM
`chunks_manifest.json` that overlaps the cited range. A
citation is counted `strict` if any overlapping chunk
contains the paraphrase substring; `fuzzy` if the
Levenshtein matcher accepted it; `fail` otherwise.

## Aggregate

- Total in-vertical annotations: **222**
- STRICT match: **0** (0.0%)
- FUZZY match:  **0** (0.0%)
- FAIL:         **222** (100.0%)
- Skipped (annotation cites out-of-vertical source like Wooldridge): **17**

## Per-card breakdown

| card_id | annotations | strict | fuzzy | fail |
|---------|-------------|--------|-------|------|
| `qm-aic-bic-model-selection` | 13 | 0 | 0 | 13 |
| `qm-anova-table` | 12 | 0 | 0 | 12 |
| `qm-arch-conditional-heteroskedasticity` | 15 | 0 | 0 | 15 |
| `qm-cb-arb-factor-construction` | 7 | 0 | 0 | 7 |
| `qm-decision-trees-and-roots` | 15 | 0 | 0 | 15 |
| `qm-goodness-of-fit-r2-adj-r2` | 14 | 0 | 0 | 14 |
| `qm-influence-analysis-leverage` | 14 | 0 | 0 | 14 |
| `qm-multiple-linear-regression-foundations` | 9 | 0 | 0 | 9 |
| `qm-panel-cb-factor-inference` | 11 | 0 | 0 | 11 |
| `qm-penalized-regression-lasso` | 13 | 0 | 0 | 13 |
| `qm-projection-and-dimensionality-reduction` | 15 | 0 | 0 | 15 |
| `qm-regression-assumption-violations` | 15 | 0 | 0 | 15 |
| `qm-regression-hypothesis-tests` | 14 | 0 | 0 | 14 |
| `qm-signal-validation-oos-discipline` | 11 | 0 | 0 | 11 |
| `qm-structured-data-ml` | 14 | 0 | 0 | 14 |
| `qm-time-series-foundations` | 17 | 0 | 0 | 17 |
| `qm-volatility-model-garch-multivariate` | 13 | 0 | 0 | 13 |

## Per-source breakdown

| source_id | annotations | strict | fuzzy | fail |
|-----------|-------------|--------|-------|------|
| `qm_afts_trim` | 13 | 0 | 0 | 13 |
| `qm_eslii_ch3_trim` | 7 | 0 | 0 | 7 |
| `qm_eslii_ch7_trim` | 11 | 0 | 0 | 11 |
| `qm_greene_trim` | 11 | 0 | 0 | 11 |
| `qm_notes_trim` | 180 | 0 | 0 | 180 |

## Sample failure paraphrases

Up to 5 paraphrases that failed both strict and fuzzy (truncated to 200 chars for display).

- `qm-aic-bic-model-selection` annotation #0 against `qm_notes_trim` chunks (best try: `qm_notes_trim:p004:0002`):
  - paraphrase: 'That single difference makes BIC favour smaller (more parsimonious) models when `n` is large'
- `qm-aic-bic-model-selection` annotation #1 against `qm_notes_trim` chunks (best try: `qm_notes_trim:p004:0002`):
  - paraphrase: 'This makes them workable for comparing non-nested models (where the nested-F-test machinery does not apply) and for choosing among more than two candidates simultaneously'
- `qm-aic-bic-model-selection` annotation #2 against `qm_notes_trim` chunks (best try: `qm_notes_trim:p004:0002`):
  - paraphrase: '*Akaike Information Criterion** is `AIC = n · ln(SSE / n) + 2 · (k + 1)`, where the `(k + 1)` term counts the slope predictors plus the intercept (some references absorb the constant `1` differently; '
- `qm-aic-bic-model-selection` annotation #3 against `qm_notes_trim` chunks (best try: `qm_notes_trim:p004:0002`):
  - paraphrase: 'Whenever `ln(n) > 2` the BIC penalty exceeds the AIC penalty for every additional regressor, so BIC ranks parsimonious models higher than AIC does at every sample size in that regime'
- `qm-aic-bic-model-selection` annotation #4 against `qm_notes_trim` chunks (best try: `qm_notes_trim:p004:0002`):
  - paraphrase: 'The two criteria can disagree on the winner; AIC tends to favour models with stronger predictive fit, BIC tends to favour models with fewer regressors'

## Interpretation

AC-8 is "measure, don't gate." The gap between this
shadow tally and the verbatim-quote primary tally is
the substantive measurement AC-9's findings artifact
(`_research/18_cfa_real_migration_findings.md`,
task-m4-10) will analyze:

  - High STRICT here would mean the legacy paraphrases
    are close enough to verbatim that Layer-2 trivially
    grounds them. (Unlikely; the legacy prose is
    aggressively reworded.)
  - Substantial FUZZY here means Layer-2 + fuzzy is
    sufficient — Layer-3 semantic verification isn't
    needed for the QM vertical.
  - Substantial FAIL here is the expected finding and
    motivates Layer-3 (semantic-judge) coverage for
    a real-corpus deployment.

