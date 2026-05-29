---
schema_version: "cacg.v0"
id: "qm-anova-table"
title: "ANOVA Table for a Fitted Regression"
reading_id: "01_quantitative_methods"
summary: "The ANOVA decomposition for a fitted regression partitions total variation SST into the regression sum of squares SSR plus the error sum of squares SSE; mean squares MSR and MSE are formed by dividing each component by its degrees of freedom, and their ratio F = MSR / MSE is the joint-significance test statistic for the slope coefficients."
tags: ["quantitative-methods", "anova-table"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p466:0590"
    chunk_hash: "e13883fc1bd038a146b6164a92314332031530db31c5c89ddb29e411d215c66b"
    page_range: [466, 467]
    quote: "SST = SSR + SSE, meaning total variation in Y equals explained variation in Y plus unexplained variation in Y."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p468:0593"
    chunk_hash: "84514b961acad885fcdfa5536135782a7ad15e3a99672f9c6d4d57426a9909fe"
    page_range: [468, 469]
    quote: "We often represent the sums of squares from a regression model in an analysis of variance (ANOVA) table"
    edge_type: "supports"
card_hash: "ae3de0de0297bc7ffe3666aaec110a3099b1f0100b9df226460a05334f64cc71"
---
# ANOVA Table for a Fitted Regression

## Intuition

The ANOVA table answers a specific question about a fitted regression:
of the total variation in the dependent variable `Y` across the `n`
observations, how much is explained by the predictors and how much
remains as residual noise? The partition is exact (the two pieces sum
to the total) and feeds the F-test for the joint hypothesis that all
slope coefficients are simultaneously zero. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The decomposition partitions each squared deviation
`(Y_i − Ȳ)²` into an explained component (squared distance from the
fitted value `Ŷ_i` to the sample mean `Ȳ`) and a residual component
(squared distance from `Y_i` to `Ŷ_i`). The source asserts the
partition at the sum-of-squares level; the algebraic justification
behind the partition belongs to a raw econometric reference.
**Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

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
an intercept term. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The mean squares are `MSR = SSR / k` and `MSE = SSE / (n − k − 1)`,
where `k` is the number of slope predictors (the intercept is not
counted in `k`). Their ratio is the **regression F-statistic**
`F = MSR / MSE`, distributed under the joint null hypothesis
`H_0: b_1 = b_2 = ... = b_k = 0` as `F(k, n − k − 1)`. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## Mathematical Reasoning

The identity `SST = SSR + SSE` (source DECOMPOSES) is the source'
partition of the total variation of `Y` into the explained
(regression) and unexplained (residual) sums of squares. The source
state the decomposition without spelling out the underlying
orthogonality argument; the card states the partition at the same
depth and labels the gap (the algebraic justification of why the
cross term vanishes belongs to a raw econometric reference).
**Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The F-statistic `F = MSR / MSE` (source ASSERTS) is the source'
joint-significance statistic comparing the per-degree-of-freedom
explained variation against the per-degree-of-freedom residual
variation. The source pairs the statistic with the joint null
`H_0: b_1 = ... = b_k = 0` and the right-tailed rejection rule for
overall significance; the formal distributional derivation that
links the SS-partition to the F-reference distribution is outside
the source' scope. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The mean-square denominators (source ASSERTS) — `MSR = SSR / k` and
`MSE = SSE / (n − k − 1)` — are the source' formulas for the per-
degree-of-freedom averages that feed the F-ratio. The source states
the denominators `k` and `n − k − 1` without deriving them; the
geometric / dimensionality argument that justifies these specific
splits is outside the source' scope and belongs to a raw econometric
reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## See Also

- [`qm-multiple-linear-regression-foundations`](qm-multiple-linear-regression-foundations.md)
  — provides the underlying linear-regression model `Y_i = b_0 + b_1 x_{1i} + ... + b_k x_{ki} + ε_i`
  whose fitted values `Ŷ_i` feed every quantity in this card's SST /
  SSR / SSE decomposition

## Escalate to Raw When

Open the underlying source or a more rigorous econometric reference
when any of the criteria below applies. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- The regression omits the intercept term — the source states the
  ANOVA decomposition in the with-intercept setting on
  `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360`; the no-intercept variant is
  outside the source' scope, so a raw econometric reference is needed
  for that boundary. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The errors are heteroskedastic or autocorrelated — the source pairs
  the F-statistic reference distribution with the homoskedastic /
  independent-error setting on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360`;
  diagnostic detection lives in
  [`qm-regression-assumption-violations`](qm-regression-assumption-violations.md),
  and any non-classical correction lives in a raw econometric
  reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The model is nonlinear in the parameters — the source' SS partition
  on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` is stated for linear-in-
  parameters regression; the nonlinear-regression analogue is outside
  the source' scope and belongs to a raw econometric reference.
  **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The hypothesis is one-sided or about a specific coefficient — the
  notes' F-test on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` is the joint
  overall-zero test; per-slope and partial-restriction inference
  lives in
  [`qm-regression-hypothesis-tests`](qm-regression-hypothesis-tests.md),
  and any one-sided variants beyond the two-sided default belong to
  a raw econometric reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
