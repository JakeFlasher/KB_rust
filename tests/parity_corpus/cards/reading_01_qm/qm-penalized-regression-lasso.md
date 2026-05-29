---
schema_version: "cacg.v0"
id: "qm-penalized-regression-lasso"
title: "LASSO Penalised Regression"
reading_id: "reading_01_qm"
summary: "framing the LASSO penalised-regression objective for a linear regression — the L1 penalty added to the OLS sum-of-squared-residuals, its hyperparameter `λ` that controls penalty strength, and the simultaneous coefficient-shrinkage / automatic-feature-selection mechanism that makes LASSO useful when the predictor count is large relative to the sample size"
tags: ["definition", "regression", "shrinkage"]
citations:
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p015:0014"
    chunk_hash: "c9db4c36421e7e9d9e642399ee10f44783f7a04a7c7d7fa986421ec30d57847d"
    page_range: [15, 16]
    quote: "Repeat this process k times. The average of the k validation errors is then taken as a reasonable estimate of the"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p016:0015"
    chunk_hash: "e56bcc63ab3914c54f5ef14bd84e318cdbe2637830d04f75190a6f853a42bd18"
    page_range: [16, 17]
    quote: "Root node: IOG > 10% Decision node: No Yes Terminal node: Free cash FCFG > 10% FCFG > 20% flow No Yes No Yes invest"
    edge_type: "supports"
card_hash: "13af9b7736d252d6ba841ecd0ffe4393ad25afefcc519e5ebf016b6cb9dc7c70"
---
framing the LASSO penalised-regression objective for a linear regression — the L1 penalty added to the OLS sum-of-squared-residuals, its hyperparameter `λ` that controls penalty strength, and the simultaneous coefficient-shrinkage / automatic-feature-selection mechanism that makes LASSO useful when the predictor count is large relative to the sample size

## Original Card (preserved verbatim)

## Intuition

Ordinary least squares minimises the residual sum of squares without
any constraint on the size of the coefficient vector; when the
predictor count `k` is large relative to the sample size `n`, OLS
tends to over-fit, producing many small-but-nonzero coefficients
that capture sample noise rather than signal. LASSO ("least
absolute shrinkage and selection operator") fixes this by adding a
penalty term to the OLS objective: instead of minimising `SSE` alone,
LASSO minimises `SSE + λ · Σ|b_j|`, where `λ ≥ 0` is a hyperparameter
that scales the penalty's bite. **Source:**
notes/CFA_note_2.ocr.pdf pp.16.

The L1 form of the penalty (sum of absolute values, not sum of
squares) has a structural consequence: as `λ` grows, more coefficient
estimates are pushed exactly to zero rather than just shrunk toward
zero. This makes LASSO a feature-selection technique in addition to
a regularisation technique — the fitted model identifies a subset of
the original predictors as the "kept" ones and zeros out the rest.
The result is a sparser, more interpretable model whose coefficient
count adapts to the chosen `λ`. **Source:**
notes/CFA_note_2.ocr.pdf pp.16.

```
<!-- primitive: regression-scatter-and-fit source: _diagram_primitives.md -->
   y
    ^                                       .
    |                                   .  ŷ = b̂_0 + b̂_1·x
    |                              .   /
    |                         .       /  .
    |                    .           / .
    |               .       .       /
    |          .                .  /
    |     .            .          /  .
    |          .          .      /
    |  .             .          /     .
    |       .                  /  .
    | b̂_0  ___________________/
    |                        /
    +-----------------------+----------------------> x
```

## Definition

The **LASSO objective** for a linear regression
`Y_i = b_0 + b_1·x_{1i} + ... + b_k·x_{ki} + ε_i` fitted on `n`
observations is the constrained minimisation
`min_b [ Σ (Y_i − b_0 − Σ_j b_j · x_{ji})² + λ · Σ_j |b_j| ]`,
where `λ ≥ 0` is the penalty hyperparameter. The notes assert the
objective form and the role of `λ` as the penalty-strength
hyperparameter. **Source:** notes/CFA_note_2.ocr.pdf pp.16.

The **L1 penalty term** `λ · Σ |b_j|` is the part of the LASSO
objective that differs from OLS. The penalty sums the absolute
values of the slope coefficients and applies the multiplier `λ`.
The notes assert the penalty form at this level; finer conventions
(e.g., whether the intercept is included in the penalty) are
outside the notes' span. **Source:** notes/CFA_note_2.ocr.pdf
pp.16.

The **shrinkage-and-selection mechanism** is the notes' joint
description of LASSO's two effects: the L1 penalty shrinks
coefficient magnitudes and forces some coefficients exactly to
zero. The notes assert both effects at the descriptive level; the
zeroed predictors form the LASSO-selected feature set and the
remaining predictors form the model. **Source:**
notes/CFA_note_2.ocr.pdf pp.16.

## Mathematical Reasoning

The L1-penalised objective (source ASSERTS) is the notes' substitute
for the OLS minimisation: the same residual-sum-of-squares is
augmented by `λ · Σ |b_j|`. The notes assert the objective form and
the role of `λ` as the penalty-strength hyperparameter; the formal
optimisation argument that links the L1 form to coefficient-exact-zero
solutions is outside the notes' span and belongs to a raw
statistical-learning reference. **Source:**
notes/CFA_note_2.ocr.pdf pp.16.

The shrinkage-vs-selection contrast (source ASSERTS) is the notes'
two-effect description: LASSO simultaneously regularises the fit
(shrinks coefficient magnitudes) and performs feature selection
(zeros some coefficients). The notes assert both effects without
deriving the geometric / KKT-conditions argument that explains why
the L1 penalty produces exact zeros while an L2 (ridge) penalty does
not. **Source:** notes/CFA_note_2.ocr.pdf pp.16.

The role of `λ` (source ASSERTS) is that larger `λ` produces
stronger shrinkage and more zeroed coefficients; smaller `λ`
produces a fit closer to OLS. The notes assert this monotone
relationship without specifying how `λ` is chosen in practice. The
choice of `λ` is a hyperparameter-tuning task outside the notes'
span and belongs to a raw statistical-learning reference. **Source:**
notes/CFA_note_2.ocr.pdf pp.16.

## See Also

- [`qm-multiple-linear-regression-foundations`](qm-multiple-linear-regression-foundations.md)
  — provides the OLS objective `min_b Σ (Y_i − b_0 − Σ_j b_j x_{ji})²`
  that LASSO augments by adding the L1 penalty term; the `λ = 0`
  limit of the LASSO objective recovers the OLS estimator exactly
- [`qm-aic-bic-model-selection`](qm-aic-bic-model-selection.md) —
  an alternative model-comparison apparatus that penalises model
  complexity through information-criterion ranking rather than
  through coefficient shrinkage; both LASSO and AIC/BIC trade off
  fit against parameter count but use different mechanisms
- [`qm-goodness-of-fit-r2-adj-r2`](qm-goodness-of-fit-r2-adj-r2.md)
  — the adjusted-R² degree-of-freedom-penalty alternative to LASSO's
  hyperparameter-controlled penalty; adjusted-R² is a weaker filter
  against junk predictors than LASSO at typical `λ` values

## Escalate to Raw When

Open the underlying source or a more rigorous statistical-learning
reference when any of the criteria below applies. **Source:**
notes/CFA_note_2.ocr.pdf pp.16.

- The analyst needs to choose `λ` on the data — the notes on
  `notes/CFA_note_2.ocr.pdf pp.16` describe LASSO's penalty
  mechanism but not the hyperparameter-tuning workflow that decides
  `λ`; cross-validation and similar tuning machinery are outside the
  notes' span and belong to a raw statistical-learning reference.
  **Source:** notes/CFA_note_2.ocr.pdf pp.16.
- The analyst wants ridge (L2) or elastic-net (L1+L2) regularisation
  instead of pure L1 — the notes on `notes/CFA_note_2.ocr.pdf pp.16`
  cover the L1 form only; other penalty families are outside the
  notes' span and belong to a raw statistical-learning reference.
  **Source:** notes/CFA_note_2.ocr.pdf pp.16.
- The analyst needs valid inference on the LASSO-selected
  coefficients — the notes on `notes/CFA_note_2.ocr.pdf pp.16` do
  not address the post-selection inference problem; corrected
  inference machinery is outside the notes' span and belongs to a
  raw statistical-learning reference. **Source:**
  notes/CFA_note_2.ocr.pdf pp.16.
- The analyst needs the specific algorithm used to fit LASSO — the
  notes on `notes/CFA_note_2.ocr.pdf pp.16` describe the objective
  but not the optimisation algorithm; LARS, coordinate descent, and
  related fitting machinery are outside the notes' span and belong
  to a raw statistical-learning reference. **Source:**
  notes/CFA_note_2.ocr.pdf pp.16.
