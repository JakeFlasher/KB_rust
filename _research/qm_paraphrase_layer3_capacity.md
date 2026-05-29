# QM Vertical — Layer-3 Capacity (Paraphrase Fuzzy Sweep)

_captured against HEAD `4d0081a791fe` on `2026-05-24`_

This is the post-M4 capacity-planning measurement that
tightens the bound the M4b shadow tally left open. The
shadow tally (`_research/qm_paraphrase_tally.md`) ran
strict-only on cost grounds; the fuzzy-only-pass count
was therefore bounded only `0 ≤ k ≤ 222`. The Round-19
review P2-B and the Round-25 review P3-D both flagged
this as a Layer-3 capacity-planning unknown.

This sample sweep runs `cacg.verify.layer2.verify_citation`
with `fuzzy_enabled=True` against EVERY overlapping chunk
for a deterministic random sample of 30 of the
222 in-vertical paraphrase annotations (seed
= 42, byte-stable across re-runs).

## Aggregate

- Sample size: **30** of 222 in-vertical annotations
- Sample wall-clock: **1085.1s**
- FUZZY-pass in sample: **0** (0.0%)
- FAIL in sample:       **30** (100.0%)

## Population estimate (95% Wilson-score)

Extrapolated to the 222-annotation paraphrase population that `_research/qm_paraphrase_tally.md` reports as strict-failed:

- Fuzzy-save rate (95% CI): **[0.0%, 11.4%]**
- Projected fuzzy-saves: **[0, 25] of 222 annotations**
- Projected Layer-3-required: **[197, 222] of 222 annotations**

## Per-source breakdown (sample)

| source_id          | sample | fuzzy | fail |
|--------------------|-------:|------:|-----:|
| `qm_afts_trim` | 3 | 0 | 3 |
| `qm_eslii_ch7_trim` | 1 | 0 | 1 |
| `qm_greene_trim` | 1 | 0 | 1 |
| `qm_notes_trim` | 25 | 0 | 25 |

## Sample fuzzy-fails (up to 5)

- `qm-regression-hypothesis-tests` ann#10 (2 chunks tried):
  - paraphrase: "`;   diagnostic detection lives in   [`qm-regression-assumption-violations`](qm-regression-assumption-violations.md),   and any non-classical robust-variance machinery is outside the   notes' scope an"
- `qm-arch-conditional-heteroskedasticity` ann#3 (3 chunks tried):
  - paraphrase: 'Rejecting the null is the verdict that conditional heteroskedasticity is present in the residual process'
- `qm-aic-bic-model-selection` ann#6 (3 chunks tried):
  - paraphrase: "For fixed `k`, BIC's penalty exceeds AIC's exactly when `ln(n) > 2`; beyond that threshold the BIC penalty per regressor is `ln(n) / 2` times the AIC penalty, which is monotone-growing in `n`"
- `qm-structured-data-ml` ann#11 (3 chunks tried):
  - paraphrase: "K-fold — bootstrap   validation, nested cross-validation, leave-one-out cross-validation,   and time-series-aware splits (rolling-window, expanding-window)   are outside the notes' span and belong to "
- `qm-goodness-of-fit-r2-adj-r2` ann#8 (2 chunks tried):
  - paraphrase: 'A negative `R̄²` is a diagnostic that the predictors carry less information than the unconditional mean — a regression-validity warning'

## Interpretation

AC-8 was "measure, don't gate." The shadow tally's
`0 ≤ fuzzy-saves ≤ 222` bound is now tightened to a real
Wilson-score interval. The complementary
`Layer-3-required` count is the strict-and-fuzzy-failed
residual — the citations that semantic verification
would have to ground if the migration ever switched
from verbatim quotes to paraphrase quotes.

The fuzzy matcher is `cacg.verify.fuzzy.fuzzy_match` —
a Levenshtein-bounded sliding-window check with a
documented edit-distance budget. Citations that fail
BOTH strict substring AND fuzzy require Layer-3
(embedding-cache or LLM-judge) coverage, which carries
operational cost. A real deployment's capacity plan
should size against the upper bound here.

