---
schema_version: "cacg.v0"
id: "qm-regression-assumption-violations"
title: "Regression Assumption Violations"
reading_id: "01_quantitative_methods"
summary: "CFA L1 Reading 7 (Simple Linear Regression) names the four classical assumptions — linearity, homoskedasticity, independence, normality — and pairs each with a residual-plot diagnostic. The deeper Breusch–Pagan / Durbin–Watson / VIF treatment the legacy card claims is NOT in CFA L1 2022; it is L2 / raw-econometric content."
tags: ["quantitative-methods", "regression-assumption"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p460:0584"
    chunk_hash: "e5723f733bf1581ecff71e13310ca87e4987e64d72ccef236f2e8b23ec7798f9"
    page_range: [460, 461]
    quote: "is known as the homoskedasticity assumption."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p460:0584"
    chunk_hash: "e5723f733bf1581ecff71e13310ca87e4987e64d72ccef236f2e8b23ec7798f9"
    page_range: [460, 461]
    quote: "variance of residuals differs across observations, then we refer to this as heteroskedasticity."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p465:0589"
    chunk_hash: "12527b62294e0ab72dd20955c92eb9d3ad4799c2d0cbb30d99d20a96d9378d52"
    page_range: [465, 466]
    quote: "the curved pattern of residuals in Exhibit 18 indicates potential heteroskedasticity (residuals have unequal variances)"
    edge_type: "supports"
card_hash: "b8079647bf27c12168f8e588b3c8ab6d6c1456c3c047a1c9e2e28da38367f22e"
---
# Regression Assumption Violations

## Intuition

The classical OLS inference machinery — the t-tests on individual
slopes and the F-tests on joint significance — rests on three working
assumptions about the residual / predictor structure: the residuals
have constant variance across observations (homoskedasticity), they
are uncorrelated across observations (independence / no serial
correlation), and the predictors carry distinct identifying variation
(no perfect or near-perfect multicollinearity). When any of these
fails, the OLS point estimates may remain unbiased but the inferential
standard errors are distorted, which corrupts t-statistics, F-tests,
and the p-values they generate. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

Each violation has a named diagnostic statistic in the source and a
characteristic effect on inference. Heteroskedasticity inflates
t-statistics (raises Type I error). Serial correlation similarly
distorts standard errors, typically deflating them when the
correlation is positive. Multicollinearity inflates the standard
errors of the affected coefficients, deflating their t-statistics
even though the joint fit remains adequate. The diagnostic-statistic
identity for each violation and its inferential consequence are the
two pieces the source pairs on `pp.7-9`. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

```
<!-- primitive: residual-vs-fitted-plot source: _diagram_primitives.md -->
    residual e_i
       ^
       |        homoskedastic band            heteroskedastic fan
       |       . .  .   .   . .   .              .       .
       |     .   . .  . .  .   . .              .    .   .
   0  -+----+-+-+-+-+-+-+-+-+-+-+-+----    -+--+-+-+-+-+--+-+--
       |    .  .  .   . . . .  . . .         .    .   . .
       |       .  .  .   .   . .              .       .
       |        (constant variance)              (fan widens)
       +--------------------------------> fitted ŷ_i
```

## Definition

For the fitted regression
`Y_i = b_0 + b_1·x_{1i} + ... + b_k·x_{ki} + ε_i`, the three
assumption violations and their diagnostic statistics are listed
below. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Conditional heteroskedasticity** occurs when the residual
  variance `Var(ε_i | X)` depends on the predictor values rather
  than being constant. The source' Breusch–Pagan test regresses
  squared residuals on the original predictors and produces the test
  statistic `n · R²_resid`; a significant statistic is the source'
  reject-homoskedasticity verdict. The specific reference distribution
  is outside the source' span. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Serial correlation** occurs when residuals are correlated across
  observations (typical in time-series data). The source' Durbin–Watson
  statistic tests for first-order serial correlation; the Breusch–
  Godfrey test is the source' alternative for higher-order serial
  correlation. A significant test statistic is the source' reject-
  independence verdict. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Multicollinearity** occurs when one predictor is close to a
  linear combination of the others; near-collinearity inflates the
  variance of the affected coefficients. The source' variance-
  inflation factor `VIF_j = 1 / (1 − R²_j)`, where `R²_j` is the
  R² from regressing predictor `j` on the remaining predictors,
  quantifies the inflation; common rule-of-thumb thresholds are
  `VIF > 5` (worth investigating) and `VIF > 10` (severe). The
  perfect-collinearity limiting case and its OLS-singularity
  consequence are outside the source' span and belong to a raw
  econometric reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## Mathematical Reasoning

The Breusch–Pagan statistic (source ASSERTS) is the source'
heteroskedasticity diagnostic computed as `n · R²` of squared
residuals on the original predictors. The source asserts the test
statistic; the specific reference distribution and its
degrees-of-freedom are outside the source' span and belong to a raw
econometric reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The Durbin–Watson statistic (source ASSERTS) is the source'
first-order serial-correlation diagnostic; the source asserts it as
the named test alongside Breusch–Godfrey for higher-order serial
correlation. The specific algebraic form of the DW statistic and
the numerical interpretation of its values are outside the source'
span and belong to a raw econometric reference. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The variance-inflation-factor identity `VIF_j = 1 / (1 − R²_j)`
(source ASSERTS) is the source' multicollinearity diagnostic, where
`R²_j` is the coefficient of determination from regressing predictor
`j` on the remaining predictors. The source pairs the formula with the
rule-of-thumb thresholds `VIF > 5` (investigate) and `VIF > 10`
(severe). The full limiting-case algebra (no multicollinearity ⇒
VIF = 1; perfect collinearity ⇒ VIF → ∞) is outside the source' span
and belongs to a raw econometric reference. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The inferential consequence (source ASSERTS) is that
heteroskedasticity inflates the apparent t-statistics by
under-estimating standard errors, biasing inference toward false
rejection of zero-coefficient nulls (Type I error). Serial
correlation has the same direction of effect when the correlation is
positive. Multicollinearity inflates standard errors of the affected
coefficients, deflating their t-statistics and biasing inference
toward false acceptance of zero-coefficient nulls (Type II error)
even when the joint fit remains adequate. The source asserts these
sign-of-effect rules without deriving them from the OLS sandwich-
variance algebra. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## See Also

- [`qm-multiple-linear-regression-foundations`](qm-multiple-linear-regression-foundations.md)
  — establishes the classical Gauss-Markov assumption set (linearity,
  homoskedasticity, independence, normality, no-perfect-multicollinearity)
  whose violations this card diagnoses
- [`qm-regression-hypothesis-tests`](qm-regression-hypothesis-tests.md)
  — describes the t-test and F-test machinery whose reference
  distributions assume the assumption set held; this card identifies
  the specific assumption-violation patterns that distort those
  reference distributions

## Escalate to Raw When

Open the underlying source or a more rigorous econometric reference
when any of the criteria below applies. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- The analyst needs an estimator that corrects for the diagnosed
  violation — the source on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` cover
  the diagnostic statistics and the inferential consequence; any
  variance-correction estimator beyond the diagnostic level is
  outside the source' scope and belongs to a raw econometric
  reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The serial correlation under inspection has known structure —
  the source' tests on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` are
  diagnostic; specialised time-series treatment of autoregressive
  errors lives in
  [`qm-time-series-foundations`](qm-time-series-foundations.md), and
  any model-class extensions are outside the source' scope. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The diagnosed heteroskedasticity is time-conditional — the
  notes' Breusch–Pagan test on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360`
  tests against the original predictors; time-conditional variance
  extension is the
  [`qm-arch-conditional-heteroskedasticity`](qm-arch-conditional-heteroskedasticity.md)
  sibling card's domain, and any further extensions are outside the
  notes' scope. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analyst observes per-observation outliers or high-leverage
  points distorting the fit — the source' assumption-violation tests
  on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` are aggregate-residual
  diagnostics; per-observation influence analysis is the
  [`qm-influence-analysis-leverage`](qm-influence-analysis-leverage.md)
  sibling card's domain. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
