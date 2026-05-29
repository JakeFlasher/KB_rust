---
schema_version: "cacg.v0"
id: "qm-aic-bic-model-selection"
title: "AIC and BIC for Regression Model Selection"
reading_id: "01_quantitative_methods"
summary: "Information criteria AIC and BIC compare regression models by trading off in-sample residual fit against a parameter-count penalty; AIC's 2(k+1) penalty is constant in n while BIC's ln(n)(k+1) penalty grows with n, so BIC favours more parsimonious models. AIC/BIC are not in CFA L1 2022 Quantitative Methods."
tags: ["quantitative-methods", "aic-bic"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p467:0591"
    chunk_hash: "249999c640c37b2adc76943c754b24467d1468b271b6167a45cf8961437590ce"
    page_range: [467, 468]
    quote: "the coefficient of determination, the F-statistic for the test of fit, and the standard error of the regression"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p467:0591"
    chunk_hash: "249999c640c37b2adc76943c754b24467d1468b271b6167a45cf8961437590ce"
    page_range: [467, 468]
    quote: "Whereas the coefficient of determination—the portion of the variation of the dependent variable explained by the independent variable—is descriptive, it is not a statistical test."
    edge_type: "supports"
card_hash: "1aea6002c5a957326d2711d017bdbd1e5ed955dc1ac1c08dec449e3420164600"
---
# AIC and BIC for Regression Model Selection

## Intuition

Information criteria rank competing regression models by combining
two terms: a goodness-of-fit term that rewards smaller in-sample
residual variation, and a complexity-penalty term that punishes
additional parameters. Lower values are better — the model that
trades off the two terms most efficiently wins. The Akaike
Information Criterion (AIC) and the Bayesian (Schwarz) Information
Criterion (BIC) differ in how their penalty grows with the sample
size `n`; AIC's penalty is constant in `n`, while BIC's penalty
scales with `ln(n)`. That single difference makes BIC favour smaller
(more parsimonious) models when `n` is large. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

Information criteria are NOT a hypothesis test. They do not produce
a p-value or a reject / fail-to-reject verdict; they simply assign
each candidate model a number, and the analyst picks the smallest.
This makes them workable for comparing non-nested models (where the
nested-F-test machinery does not apply) and for choosing among more
than two candidates simultaneously. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

```
<!-- primitive: model-selection-penalty-curve source: _diagram_primitives.md -->
   penalty term
       ^
       |                                   BIC: ln(n)·(k+1)
       |                                  /  (steeper for large n)
       |                                 /
       |                                /
       |                               /     AIC: 2(k+1)
       |                              /     /
       |                             /     /
       |                            /     /
       |                           /     /
       |                          /     /
       |                         /     /
       |                        /     /
       |                       /     /
       |                      /     /
       |                     /     /
       +--------------------+-----+----------> complexity k
       0                   k*_BIC  k*_AIC
                          (BIC prefers fewer regressors than AIC)
```

## Definition

For a linear regression
`Y_i = b_0 + b_1·x_{1i} + ... + b_k·x_{ki} + ε_i` fitted on `n`
observations with error sum of squares `SSE` and `k` slope
predictors, the **Akaike Information Criterion** is
`AIC = n · ln(SSE / n) + 2 · (k + 1)`, where the `(k + 1)` term
counts the slope predictors plus the intercept (some references
absorb the constant `1` differently; the source uses the explicit
`k + 1` form). **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The **Schwarz Bayesian Information Criterion** is
`BIC = n · ln(SSE / n) + ln(n) · (k + 1)`. The goodness-of-fit term
`n · ln(SSE / n)` is shared with AIC; the penalty term `ln(n) · (k + 1)`
is the only difference. Whenever `ln(n) > 2` the BIC penalty exceeds
the AIC penalty for every additional regressor, so BIC ranks
parsimonious models higher than AIC does at every sample size in
that regime. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The decision rule for both criteria is **lower is better**: among
candidate models, pick the one with the smallest AIC (or smallest
BIC). The two criteria can disagree on the winner; AIC tends to
favour models with stronger predictive fit, BIC tends to favour
models with fewer regressors. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## Mathematical Reasoning

The information-criterion forms (source ASSERTS) — the AIC
`n · ln(SSE / n) + 2 · (k + 1)` and the BIC
`n · ln(SSE / n) + ln(n) · (k + 1)` — share the goodness-of-fit term
`n · ln(SSE / n)` that decreases as the residual sum of squares
shrinks, and differ only in the parameter-penalty multiplier
(`2` for AIC, `ln(n)` for BIC). The source asserts both formulas
without deriving them from a deeper likelihood-theoretic foundation;
the card states the formulas at the same depth and labels the gap
(a likelihood-theoretic derivation belongs in a raw econometric
reference rather than in this card). **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The penalty-growth contrast (source ASSERTS) — AIC's
`2 · (k + 1)` is linear in `k` but constant in `n`, while BIC's
`ln(n) · (k + 1)` is linear in `k` AND grows with `n` — is the
mathematical content driving every practical guideline that follows.
For fixed `k`, BIC's penalty exceeds AIC's exactly when `ln(n) > 2`;
beyond that threshold the BIC penalty per regressor is `ln(n) / 2`
times the AIC penalty, which is monotone-growing in `n`. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The parsimony-vs-fit consequence of the penalty-growth contrast
(source ASSERTS) is that BIC's larger penalty per regressor at
typical sample sizes makes the BIC-optimal model carry fewer
regressors than the AIC-optimal model whenever the two criteria
disagree. The source asserts that BIC penalises harder as `n` grows
and so favours parsimony, while AIC tends to favour predictive fit;
both characterisations follow directly from the explicit penalty
formulas and the linear-vs-log scaling identified above without
invoking deeper asymptotic-consistency results that the source does
not state. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## See Also

- [`qm-goodness-of-fit-r2-adj-r2`](qm-goodness-of-fit-r2-adj-r2.md)
  — defines the adjusted-`R²` degree-of-freedom-penalty alternative
  that this card's information criteria compete with; both approaches
  trade off fit against parameter count, but use different penalty
  structures (`(n − 1) / (n − k − 1)` rescaling vs explicit
  log-likelihood penalty)
- [`qm-anova-table`](qm-anova-table.md) — provides the `SSE` quantity
  that feeds both `n · ln(SSE / n)` goodness-of-fit terms in this
  card's AIC and BIC formulas; the ANOVA partition is the upstream
  numerical input

## Escalate to Raw When

Open the underlying source or a more rigorous econometric reference
when any of the criteria below applies. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- Heteroskedastic or autocorrelated errors — the source states the
  `SSE`-based AIC and BIC formulas in the OLS-residual setting on
  `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360`; the diagnostic detection of
  heteroskedasticity / autocorrelation is the
  [`qm-regression-assumption-violations`](qm-regression-assumption-violations.md)
  sibling card's domain, and any non-classical model-selection
  criterion lives in a raw econometric reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- Non-linear-in-parameters or non-real-valued-target regression
  models — the `SSE`-based goodness-of-fit term on
  `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` presumes the linear-regression
  setting that the source covers; analogous criteria for other model
  classes are outside the source' scope and belong to a raw
  econometric reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- Bayesian model comparison — the source presents BIC on
  `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` as a penalty-based ranking rule;
  full Bayesian model-comparison machinery is outside the source'
  scope and belongs to a raw Bayesian reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- Out-of-sample predictive interest — the source' AIC and BIC on
  `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` are in-sample penalty-adjusted
  fit measures; out-of-sample evaluation methodology is outside the
  notes' scope and belongs to a raw econometric reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
