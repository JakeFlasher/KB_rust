---
schema_version: "cacg.v0"
id: "qm-anova-table"
title: "ANOVA Table for a Fitted Regression"
reading_id: "reading_01_qm"
summary: "framing the ANOVA partition for a fitted regression — splitting total variation of the dependent variable into an explained component, an unexplained component, and the F-statistic that tests whether the regressors jointly explain a non-zero share of total variation"
tags: ["anova", "definition"]
card_edges:
  - target: "qm-multiple-linear-regression-foundations"
    edge_type: "extends"
citations:
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p001:0000"
    chunk_hash: "93c56ed12de8411fad7822569e4e1a91752744ca27a3ede32b1d3e12cc83e0cd"
    page_range: [1, 2]
    quote: "U 1. Muttiple liner regression (1)Yi=bo+bix1+b2xi++bkxki+Ei bo: intercept term b:slope coefficent lexpectedincrease in"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p003:0001"
    chunk_hash: "3b9eef70d0ea521b06ea524550cf336c4520f46b5c118e5afaa7f5825071f2c5"
    page_range: [3, 4]
    quote: "Measures of Goodness of Fit UUT Yi- Yi B 4i-4 GOLDEN EDUCATIC I. Coefficient of determination ( R2 )"
    edge_type: "supports"
card_hash: "95129c76da0af773fe69e3c29f2b60bc10e8d23d8908b815fcaf21c887006b5d"
---
framing the ANOVA partition for a fitted regression — splitting total variation of the dependent variable into an explained component, an unexplained component, and the F-statistic that tests whether the regressors jointly explain a non-zero share of total variation

## Original Card (preserved verbatim)

## Intuition

The ANOVA table answers a specific question about a fitted regression:
of the total variation in the dependent variable `Y` across the `n`
observations, how much is explained by the predictors and how much
remains as residual noise? The partition is exact (the two pieces sum
to the total) and feeds the F-test for the joint hypothesis that all
slope coefficients are simultaneously zero. **Source:**
notes/CFA_note_2.ocr.pdf pp.2-3.

The decomposition partitions each squared deviation
`(Y_i − Ȳ)²` into an explained component (squared distance from the
fitted value `Ŷ_i` to the sample mean `Ȳ`) and a residual component
(squared distance from `Y_i` to `Ŷ_i`). The notes assert the
partition at the sum-of-squares level; the algebraic justification
behind the partition belongs to a raw econometric reference.
**Source:** notes/CFA_note_2.ocr.pdf pp.2-3.

```
<!-- primitive: anova-decomposition-box source: _diagram_primitives.md -->
   Total variation of Y                Σ (Y_i − Ȳ)²
   ┌─────────────────────────────────────────────────┐
   │                       SST                       │
   └─────────────────────────────────────────────────┘
                          ║
              ┌───────────╨───────────┐
              ▼                       ▼
       ┌─────────────┐         ┌─────────────┐
       │     SSR     │    +    │     SSE     │
       │ Σ(Ŷ_i − Ȳ)² │         │ Σ(Y_i − Ŷ_i)²│
       │  (explained)│         │ (residual)  │
       └─────────────┘         └─────────────┘
        MSR = SSR / k          MSE = SSE / (n − k − 1)

           F = MSR / MSE  ~  F(k, n − k − 1) under H_0
```

## Definition

The ANOVA decomposition for a linear regression
`Y_i = b_0 + b_1·x_{1i} + ... + b_k·x_{ki} + ε_i` fitted on `n`
observations partitions the **total sum of squares**
`SST = Σ_{i=1}^{n} (Y_i − Ȳ)²` into the **regression sum of squares**
`SSR = Σ_{i=1}^{n} (Ŷ_i − Ȳ)²` and the **error sum of squares**
`SSE = Σ_{i=1}^{n} (Y_i − Ŷ_i)²`, where `Ȳ` is the sample mean of
`Y`, `Ŷ_i` is the OLS-fitted value for observation `i`, and the
identity `SST = SSR + SSE` holds exactly when the regression includes
an intercept term. **Source:** notes/CFA_note_2.ocr.pdf pp.2-3.

The mean squares are `MSR = SSR / k` and `MSE = SSE / (n − k − 1)`,
where `k` is the number of slope predictors (the intercept is not
counted in `k`). Their ratio is the **regression F-statistic**
`F = MSR / MSE`, distributed under the joint null hypothesis
`H_0: b_1 = b_2 = ... = b_k = 0` as `F(k, n − k − 1)`. **Source:**
notes/CFA_note_2.ocr.pdf pp.2-3.

## Mathematical Reasoning

The identity `SST = SSR + SSE` (source DECOMPOSES) is the notes'
partition of the total variation of `Y` into the explained
(regression) and unexplained (residual) sums of squares. The notes
state the decomposition without spelling out the underlying
orthogonality argument; the card states the partition at the same
depth and labels the gap (the algebraic justification of why the
cross term vanishes belongs to a raw econometric reference).
**Source:** notes/CFA_note_2.ocr.pdf pp.2-3.

The F-statistic `F = MSR / MSE` (source ASSERTS) is the notes'
joint-significance statistic comparing the per-degree-of-freedom
explained variation against the per-degree-of-freedom residual
variation. The notes pair the statistic with the joint null
`H_0: b_1 = ... = b_k = 0` and the right-tailed rejection rule for
overall significance; the formal distributional derivation that
links the SS-partition to the F-reference distribution is outside
the notes' scope. **Source:** notes/CFA_note_2.ocr.pdf pp.2-3.

The mean-square denominators (source ASSERTS) — `MSR = SSR / k` and
`MSE = SSE / (n − k − 1)` — are the notes' formulas for the per-
degree-of-freedom averages that feed the F-ratio. The notes state
the denominators `k` and `n − k − 1` without deriving them; the
geometric / dimensionality argument that justifies these specific
splits is outside the notes' scope and belongs to a raw econometric
reference. **Source:** notes/CFA_note_2.ocr.pdf pp.2-3.

## See Also

- [`qm-multiple-linear-regression-foundations`](qm-multiple-linear-regression-foundations.md)
  — provides the underlying linear-regression model `Y_i = b_0 + b_1 x_{1i} + ... + b_k x_{ki} + ε_i`
  whose fitted values `Ŷ_i` feed every quantity in this card's SST /
  SSR / SSE decomposition

## Escalate to Raw When

Open the underlying source or a more rigorous econometric reference
when any of the criteria below applies. **Source:**
notes/CFA_note_2.ocr.pdf pp.2-3.

- The regression omits the intercept term — the notes state the
  ANOVA decomposition in the with-intercept setting on
  `notes/CFA_note_2.ocr.pdf pp.2-3`; the no-intercept variant is
  outside the notes' scope, so a raw econometric reference is needed
  for that boundary. **Source:** notes/CFA_note_2.ocr.pdf pp.2-3.
- The errors are heteroskedastic or autocorrelated — the notes pair
  the F-statistic reference distribution with the homoskedastic /
  independent-error setting on `notes/CFA_note_2.ocr.pdf pp.2-3`;
  diagnostic detection lives in
  [`qm-regression-assumption-violations`](qm-regression-assumption-violations.md),
  and any non-classical correction lives in a raw econometric
  reference. **Source:** notes/CFA_note_2.ocr.pdf pp.2-3.
- The model is nonlinear in the parameters — the notes' SS partition
  on `notes/CFA_note_2.ocr.pdf pp.2-3` is stated for linear-in-
  parameters regression; the nonlinear-regression analogue is outside
  the notes' scope and belongs to a raw econometric reference.
  **Source:** notes/CFA_note_2.ocr.pdf pp.2-3.
- The hypothesis is one-sided or about a specific coefficient — the
  notes' F-test on `notes/CFA_note_2.ocr.pdf pp.2-3` is the joint
  overall-zero test; per-slope and partial-restriction inference
  lives in
  [`qm-regression-hypothesis-tests`](qm-regression-hypothesis-tests.md),
  and any one-sided variants beyond the two-sided default belong to
  a raw econometric reference. **Source:** notes/CFA_note_2.ocr.pdf pp.2-3.
