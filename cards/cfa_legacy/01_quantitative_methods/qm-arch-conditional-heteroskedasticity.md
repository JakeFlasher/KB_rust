---
schema_version: "cacg.v0"
id: "qm-arch-conditional-heteroskedasticity"
title: "ARCH(1) Conditional Heteroskedasticity"
reading_id: "01_quantitative_methods"
summary: "ARCH(1) models the squared regression residual at time t as a linear function of its own lag, e_t^2 = a_0 + a_1 e_{t-1}^2 + u_t; significance of a_1 is the conditional-heteroskedasticity verdict and the prescribed estimator response is to switch from OLS to GLS. ARCH is not in CFA L1 2022 Quantitative Methods."
tags: ["quantitative-methods", "arch-conditional"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p459:0583"
    chunk_hash: "d1861ed647fb04131013ea49936e275d653994f4bd336d765db641da4d156b63"
    page_range: [459, 460]
    quote: "2 Homoskedasticity: The variance of the regression residuals is the same for all observations."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p462:0586"
    chunk_hash: "85f25d69b36ad1a3a7bcc6128f7c9abd19c04544ed8d45cc6c05d05dfa3f4f4b"
    page_range: [462, 463]
    quote: "the clustering of residuals in two groups with much different variances clearly indicates the existence of distinct regimes"
    edge_type: "supports"
card_hash: "9a03bcbb82b12365909d852c74daf7fb15962782ee9a9236e9e8a324da40aa52"
---
# ARCH(1) Conditional Heteroskedasticity

## Intuition

The aggregate-residual heteroskedasticity test in the sibling
[`qm-regression-assumption-violations`](qm-regression-assumption-violations.md)
card detects whether residual variance depends on the original
PREDICTORS. The time-series counterpart asks a different question:
whether the residual variance at time `t` depends on what the
residual variance was at time `t − 1`. When it does, the residual
process has "volatility clustering" — periods of large squared
residuals tend to cluster together, and periods of small squared
residuals tend to cluster together. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

ARCH (Autoregressive Conditional Heteroskedasticity) is the simplest
parametric model of that clustering: it regresses the squared
residual at time `t` on the squared residual at time `t − 1`. The
slope coefficient on the lag, if significantly different from zero,
is the diagnostic that conditional heteroskedasticity is present.
When it is, the unconditional variance is no longer the correct
input to a risk forecast at time `t`; the conditional variance
`σ_t²` is, and the OLS estimator that assumes constant residual
variance is inefficient — generalised least squares (GLS) becomes
the appropriate estimator. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

```
   ε_t² (squared residual at time t)
       ^
       |                                       .
       |                                  .
       |                              .
       |                          .
       |                      .              σ_t² conditional-variance
       |                  .                  line under ARCH(1):
       |              .                      σ_t² = a_0 + a_1 · ε_{t-1}²
       |          .
       |       .                              slope a_1  >  0
       |    .                                 ⇒ large ε_{t-1}² predicts
       | a_0                                    large ε_t² ("volatility
       | .                                      clustering")
       |
       +-------------------------------------> ε_{t-1}² (squared residual
                                               at the previous period)

   under conditional homoskedasticity (a_1 = 0):  σ_t² ≡ a_0  (flat)
   under ARCH (a_1 > 0):                          σ_t² rises with ε_{t-1}²
```

## Definition

For a time-series regression residual process `{ε_t}` from a fitted
linear regression (the time-series foundations card's AR or trend
model providing the residual stream), the **ARCH(1) model** is the
auxiliary regression
`ε_t² = a_0 + a_1 · ε_{t-1}² + u_t`,
where `a_0` is the intercept, `a_1` is the slope on the lagged
squared residual, and `u_t` is the auxiliary-regression error. The
ARCH(1) coefficient `a_1` is fitted by OLS on the squared residual
series. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The **ARCH detection rule** is to test `H_0: a_1 = 0` against
`H_A: a_1 ≠ 0` using the standard t-statistic on the OLS estimate
`â_1`. Rejecting the null is the verdict that conditional
heteroskedasticity is present in the residual process. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The **conditional-variance forecast** under ARCH(1) is
`σ_t² = a_0 + a_1 · ε_{t-1}²`, which is time-varying (it depends
on the most recent squared residual rather than being a constant).
The source asserts that under conditional heteroskedasticity the
analyst's risk forecast at time `t` should use the conditional
variance `σ_t²` rather than the unconditional variance. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The **estimator switch** when ARCH is present is from OLS to
generalised least squares (GLS); the source asserts this substitution
as the appropriate estimator response under conditional
heteroskedasticity without specifying the GLS weighting scheme or
the standard-error correction details. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## Mathematical Reasoning

The ARCH(1) recursion (source ASSERTS) `ε_t² = a_0 + a_1 · ε_{t-1}² + u_t`
is the source' auxiliary regression for the second moment of the
residual process. The source asserts the recursion form; the
interpretation of `a_1 ≠ 0` as significant lag-dependence is the
content of the detection test below. The reduction `a_1 = 0 →
constant variance` and the deeper estimator-foundations question
(maximum-likelihood vs OLS-on-squared-residuals) are outside the
notes' span and belong to a raw time-series-volatility reference.
**Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The detection test (source ASSERTS) is the source' significance test
on `â_1`: a significant coefficient on the lagged squared residual
is the conditional-heteroskedasticity verdict. The source pairs this
test with Breusch–Pagan as the predictor-conditioned analogue in
the sibling assumption-violations card; ARCH is the time-conditional
version. The specific reference distribution / asymptotic argument
for `â_1` in the squared-residual auxiliary regression is outside
the source' span. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The conditional-variance forecasting input (source ASSERTS) is the
notes' guidance that under conditional heteroskedasticity the
time-`t` risk projection uses the conditional variance
`σ_t² = a_0 + a_1 · ε_{t-1}²` — a time-varying quantity that depends
on the most recent squared residual — rather than the unconditional
variance. The source states this as a forecasting-input choice; the
algebraic relationship between conditional and unconditional
variance is outside the source' span. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The OLS-to-GLS switch (source ASSERTS) is the source' prescription
that generalised least squares becomes the appropriate estimator
when ARCH conditional heteroskedasticity is present. The source
assert the substitution at the estimator-name level; the detailed
weighting scheme and standard-error correction are outside the
notes' span and belong to a raw econometric reference. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## See Also

- [`qm-time-series-foundations`](qm-time-series-foundations.md) —
  provides the AR(p) and trend models whose fitted residuals
  `{ε_t}` are the inputs to this card's ARCH(1) auxiliary
  regression
- [`qm-regression-assumption-violations`](qm-regression-assumption-violations.md)
  — covers the Breusch–Pagan predictor-conditioned heteroskedasticity
  test, of which this card's ARCH(1) detection is the
  time-conditional analogue

## Escalate to Raw When

Open the underlying source or a more rigorous econometric reference
when any of the criteria below applies. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- The residual process exhibits more complex volatility memory than
  ARCH(1) captures — the source on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360`
  cover the single-lag ARCH(1) form only; multi-lag and lagged-
  variance extensions are outside the source' scope and belong to a
  raw time-series-volatility reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The volatility response to positive and negative residual shocks
  is asymmetric — the source' ARCH(1) on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` is symmetric in the sign of `ε_{t-1}`; asymmetric-volatility
  extensions are outside the source' scope and belong to a raw
  time-series-volatility reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analyst needs full likelihood-based estimation of the ARCH
  parameters — the source on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` describe
  the OLS regression on squared residuals as the detection device;
  full maximum-likelihood ARCH-family estimation is outside the
  notes' scope and belongs to a raw time-series-volatility reference.
  **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analysis requires multivariate ARCH (cross-asset volatility
  spillover) — the source' ARCH on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360`
  is single-series; multivariate-ARCH extensions are outside the
  notes' scope and belong to a raw time-series-volatility reference.
  **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
