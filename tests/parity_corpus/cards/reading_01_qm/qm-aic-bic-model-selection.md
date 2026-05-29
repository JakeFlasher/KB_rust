---
schema_version: "cacg.v0"
id: "qm-aic-bic-model-selection"
title: "AIC and BIC for Regression Model Selection"
reading_id: "reading_01_qm"
summary: "framing the Akaike and Schwarz information-criterion approach to comparing competing regression models — penalty-based ranking of nested or non-nested candidates by trading off in-sample residual fit against parameter count, and the AIC-versus-BIC contrast that follows from their different penalty growth rates"
tags: ["definition", "model-selection"]
card_edges:
  - target: "qm-anova-table"
    edge_type: "extends"
  - target: "qm-goodness-of-fit-r2-adj-r2"
    edge_type: "extends"
citations:
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p004:0002"
    chunk_hash: "39655e5140c045f186eb0642098261d7c728237518b2a3c111a3bebd5c8f0cd3"
    page_range: [4, 5]
    quote: "X t Adding a new independent variable may either increase or t= b1-0 |>1 Sb decrease the R2"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p005:0003"
    chunk_hash: "3afba34490df4ebc84f652a34c9d0a35ce9a41914d6afdce25aa4a38459e4389"
    page_range: [5, 6]
    quote: "Goodness of Fit ttR{2 2 AIC (Akaike's information criterion) 4 AIC = n x ln SE +2(k+1) n Where: n = number of"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p006:0004"
    chunk_hash: "4ed9fdb76ece800b7858c72ac5a20e57c71b4ec14093b7f3938aece82f48fda6"
    page_range: [6, 7]
    quote: "Compared to AIC, BIC assesses a greater penalty for adding independent variables"
    edge_type: "supports"
card_hash: "f5505eed31200f4c27dd03a8078494757a7703ea078b1e4264b99b6abac2a324"
---
framing the Akaike and Schwarz information-criterion approach to comparing competing regression models — penalty-based ranking of nested or non-nested candidates by trading off in-sample residual fit against parameter count, and the AIC-versus-BIC contrast that follows from their different penalty growth rates

## Original Card (preserved verbatim)

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
notes/CFA_note_2.ocr.pdf pp.5-6.

Information criteria are NOT a hypothesis test. They do not produce
a p-value or a reject / fail-to-reject verdict; they simply assign
each candidate model a number, and the analyst picks the smallest.
This makes them workable for comparing non-nested models (where the
nested-F-test machinery does not apply) and for choosing among more
than two candidates simultaneously. **Source:**
notes/CFA_note_2.ocr.pdf pp.5-6.

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
absorb the constant `1` differently; the notes use the explicit
`k + 1` form). **Source:** notes/CFA_note_2.ocr.pdf pp.5-6.

The **Schwarz Bayesian Information Criterion** is
`BIC = n · ln(SSE / n) + ln(n) · (k + 1)`. The goodness-of-fit term
`n · ln(SSE / n)` is shared with AIC; the penalty term `ln(n) · (k + 1)`
is the only difference. Whenever `ln(n) > 2` the BIC penalty exceeds
the AIC penalty for every additional regressor, so BIC ranks
parsimonious models higher than AIC does at every sample size in
that regime. **Source:** notes/CFA_note_2.ocr.pdf pp.5-6.

The decision rule for both criteria is **lower is better**: among
candidate models, pick the one with the smallest AIC (or smallest
BIC). The two criteria can disagree on the winner; AIC tends to
favour models with stronger predictive fit, BIC tends to favour
models with fewer regressors. **Source:** notes/CFA_note_2.ocr.pdf
pp.5-6.

## Mathematical Reasoning

The information-criterion forms (source ASSERTS) — the AIC
`n · ln(SSE / n) + 2 · (k + 1)` and the BIC
`n · ln(SSE / n) + ln(n) · (k + 1)` — share the goodness-of-fit term
`n · ln(SSE / n)` that decreases as the residual sum of squares
shrinks, and differ only in the parameter-penalty multiplier
(`2` for AIC, `ln(n)` for BIC). The notes assert both formulas
without deriving them from a deeper likelihood-theoretic foundation;
the card states the formulas at the same depth and labels the gap
(a likelihood-theoretic derivation belongs in a raw econometric
reference rather than in this card). **Source:**
notes/CFA_note_2.ocr.pdf pp.5-6.

The penalty-growth contrast (source ASSERTS) — AIC's
`2 · (k + 1)` is linear in `k` but constant in `n`, while BIC's
`ln(n) · (k + 1)` is linear in `k` AND grows with `n` — is the
mathematical content driving every practical guideline that follows.
For fixed `k`, BIC's penalty exceeds AIC's exactly when `ln(n) > 2`;
beyond that threshold the BIC penalty per regressor is `ln(n) / 2`
times the AIC penalty, which is monotone-growing in `n`. **Source:**
notes/CFA_note_2.ocr.pdf pp.5-6.

The parsimony-vs-fit consequence of the penalty-growth contrast
(source ASSERTS) is that BIC's larger penalty per regressor at
typical sample sizes makes the BIC-optimal model carry fewer
regressors than the AIC-optimal model whenever the two criteria
disagree. The notes assert that BIC penalises harder as `n` grows
and so favours parsimony, while AIC tends to favour predictive fit;
both characterisations follow directly from the explicit penalty
formulas and the linear-vs-log scaling identified above without
invoking deeper asymptotic-consistency results that the notes do
not state. **Source:** notes/CFA_note_2.ocr.pdf pp.5-6.

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
notes/CFA_note_2.ocr.pdf pp.5-6.

- Heteroskedastic or autocorrelated errors — the notes state the
  `SSE`-based AIC and BIC formulas in the OLS-residual setting on
  `notes/CFA_note_2.ocr.pdf pp.5-6`; the diagnostic detection of
  heteroskedasticity / autocorrelation is the
  [`qm-regression-assumption-violations`](qm-regression-assumption-violations.md)
  sibling card's domain, and any non-classical model-selection
  criterion lives in a raw econometric reference. **Source:**
  notes/CFA_note_2.ocr.pdf pp.5-6.
- Non-linear-in-parameters or non-real-valued-target regression
  models — the `SSE`-based goodness-of-fit term on
  `notes/CFA_note_2.ocr.pdf pp.5-6` presumes the linear-regression
  setting that the notes cover; analogous criteria for other model
  classes are outside the notes' scope and belong to a raw
  econometric reference. **Source:** notes/CFA_note_2.ocr.pdf pp.5-6.
- Bayesian model comparison — the notes present BIC on
  `notes/CFA_note_2.ocr.pdf pp.5-6` as a penalty-based ranking rule;
  full Bayesian model-comparison machinery is outside the notes'
  scope and belongs to a raw Bayesian reference. **Source:**
  notes/CFA_note_2.ocr.pdf pp.5-6.
- Out-of-sample predictive interest — the notes' AIC and BIC on
  `notes/CFA_note_2.ocr.pdf pp.5-6` are in-sample penalty-adjusted
  fit measures; out-of-sample evaluation methodology is outside the
  notes' scope and belongs to a raw econometric reference. **Source:**
  notes/CFA_note_2.ocr.pdf pp.5-6.
