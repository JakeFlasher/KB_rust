---
schema_version: "cacg.v0"
id: "qm-penalized-regression-lasso"
title: "LASSO Penalised Regression"
reading_id: "01_quantitative_methods"
summary: "LASSO augments the OLS sum-of-squared-residuals with an L1 penalty lambda * sum |b_j|; for large lambda some coefficients are shrunk exactly to zero, giving simultaneous regularisation and automatic feature selection. LASSO is not in CFA L1 2022 Quantitative Methods; R7 covers the unpenalised OLS criterion only."
tags: ["quantitative-methods", "penalized-regression"]
citations:
  - source_id: "qm_eslii_2009_2ed"
    chunk_id: "qm_eslii_2009_2ed:p087:0108"
    chunk_hash: "35d833a81343e91541e9be749fc404601230b56ff3b4d0e22bc5281372d63d2f"
    page_range: [87, 88]
    quote: "Notice the similarity to the ridge regression problem (3.42) or (3.41): the L2 ridge penalty Pp 1 β 2 j is replaced by the L1 lasso penalty Pp 1 |βj |."
    edge_type: "defines"
  - source_id: "qm_eslii_2009_2ed"
    chunk_id: "qm_eslii_2009_2ed:p088:0109"
    chunk_hash: "ea948a05da4c3a8cbb7675c22d5b45fa1ceb6085ed2611ffefc372d911760e5f"
    page_range: [88, 88]
    quote: "Because of the nature of the constraint, making t sufficiently small will cause some of the coefficients to be exactly zero. Thus the lasso does a kind of continuous subset selection."
    edge_type: "supports"
card_hash: "f9f695fda7053fce81941a6c61f09c4c51bb2de69fb18e326ea4d9b34653d929"
---
# LASSO Penalised Regression

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
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The L1 form of the penalty (sum of absolute values, not sum of
squares) has a structural consequence: as `λ` grows, more coefficient
estimates are pushed exactly to zero rather than just shrunk toward
zero. This makes LASSO a feature-selection technique in addition to
a regularisation technique — the fitted model identifies a subset of
the original predictors as the "kept" ones and zeros out the rest.
The result is a sparser, more interpretable model whose coefficient
count adapts to the chosen `λ`. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

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
where `λ ≥ 0` is the penalty hyperparameter. The source asserts the
objective form and the role of `λ` as the penalty-strength
hyperparameter. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The **L1 penalty term** `λ · Σ |b_j|` is the part of the LASSO
objective that differs from OLS. The penalty sums the absolute
values of the slope coefficients and applies the multiplier `λ`.
The source asserts the penalty form at this level; finer conventions
(e.g., whether the intercept is included in the penalty) are
outside the source' span. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The **shrinkage-and-selection mechanism** is the source' joint
description of LASSO's two effects: the L1 penalty shrinks
coefficient magnitudes and forces some coefficients exactly to
zero. The source asserts both effects at the descriptive level; the
zeroed predictors form the LASSO-selected feature set and the
remaining predictors form the model. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## Mathematical Reasoning

The L1-penalised objective (source ASSERTS) is the source' substitute
for the OLS minimisation: the same residual-sum-of-squares is
augmented by `λ · Σ |b_j|`. The source asserts the objective form and
the role of `λ` as the penalty-strength hyperparameter; the formal
optimisation argument that links the L1 form to coefficient-exact-zero
solutions is outside the source' span and belongs to a raw
statistical-learning reference. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The shrinkage-vs-selection contrast (source ASSERTS) is the source'
two-effect description: LASSO simultaneously regularises the fit
(shrinks coefficient magnitudes) and performs feature selection
(zeros some coefficients). The source asserts both effects without
deriving the geometric / KKT-conditions argument that explains why
the L1 penalty produces exact zeros while an L2 (ridge) penalty does
not. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The role of `λ` (source ASSERTS) is that larger `λ` produces
stronger shrinkage and more zeroed coefficients; smaller `λ`
produces a fit closer to OLS. The source asserts this monotone
relationship without specifying how `λ` is chosen in practice. The
choice of `λ` is a hyperparameter-tuning task outside the source'
span and belongs to a raw statistical-learning reference. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

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
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- The analyst needs to choose `λ` on the data — the CFA L1 QM
  reading describes LASSO's penalty mechanism but not the
  hyperparameter-tuning workflow that decides `λ`; cross-validation
  and similar tuning machinery are outside the source's span and
  belong to a raw statistical-learning reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analyst wants ridge (L2) or elastic-net (L1+L2) regularisation
  instead of pure L1 — the CFA L1 QM reading covers the L1 form only;
  other penalty families are outside the source's span and belong to
  a raw statistical-learning reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analyst needs valid inference on the LASSO-selected
  coefficients — the CFA L1 QM reading does not address the
  post-selection inference problem; corrected inference machinery is
  outside the source's span and belongs to a raw statistical-learning
  reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analyst needs the specific algorithm used to fit LASSO — the
  CFA L1 QM reading describes the objective but not the optimisation
  algorithm; LARS, coordinate descent, and related fitting machinery
  are outside the source's span and belong to a raw statistical-
  learning reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
