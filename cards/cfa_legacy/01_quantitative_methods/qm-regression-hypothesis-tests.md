---
schema_version: "cacg.v0"
id: "qm-regression-hypothesis-tests"
title: "Hypothesis Tests on a Fitted Regression"
reading_id: "01_quantitative_methods"
summary: "CFA L1 Reading 7 introduces the F-distributed test for the joint significance of regression slopes and the t-test on an individual slope coefficient (with `n − k − 1` df). At the L1 level, partial (nested) F-tests on multi-coefficient restrictions are not developed; the legacy card overreaches to multiple-regression machinery that is L2 / raw-econometric content."
tags: ["quantitative-methods", "regression-hypothesis"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p467:0591"
    chunk_hash: "249999c640c37b2adc76943c754b24467d1468b271b6167a45cf8961437590ce"
    page_range: [467, 468]
    quote: "In general, we use an F-distributed test statistic to compare two variances."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p471:0597"
    chunk_hash: "3c03c0235957c7c17abe947e5b8f605d9a488840ae05113b5a908fcb964d5ce7"
    page_range: [471, 472]
    quote: "To test a hypothesis about a slope, we calculate the test statistic by subtracting the hypothesized population slope"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p471:0597"
    chunk_hash: "3c03c0235957c7c17abe947e5b8f605d9a488840ae05113b5a908fcb964d5ce7"
    page_range: [471, 472]
    quote: "If the calculated t-statistic is outside the bounds of the critical t-values, we reject the null hypothesis"
    edge_type: "supports"
card_hash: "6cafb5b9d2412eaecc5ed561b8a7182a0a5b29670b73d8c84386e68cccfbcd2e"
---
# Hypothesis Tests on a Fitted Regression

## Intuition

A fitted regression produces estimates `b̂_1, ..., b̂_k` of the
population slope coefficients, but each estimate carries sampling
noise: a different sample would give different numbers. The
inferential question is whether the estimate's distance from a
hypothesised value is large enough — relative to its standard error —
to reject the hypothesised value at a chosen significance level. The
two-tailed t-test is the workhorse for per-coefficient inference; the
F-test is the workhorse for joint inference across multiple
coefficients. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The source pairs each test statistic with a specific reference
distribution under the classical-assumption set: the per-coefficient
t-statistic is referenced against Student-t with `n − k − 1` degrees
of freedom; the overall F-statistic is referenced against
`F(k, n − k − 1)`; the partial F-statistic is referenced against
`F(q, n − k − 1)`. The source asserts the statistic-and-distribution
pairing; the underlying sampling-distribution algebra is outside
the source' span. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

```
<!-- primitive: hypothesis-test-tail-regions source: _diagram_primitives.md -->
   density f(t)
       ^
       |             ___
       |           /     \
       |          /       \
       |         /         \
       |        /           \
       |  *****/             \*****            two-tailed
       |  ****/   accept H_0  \****            rejection at
       |  ***/                 \***            ± t_crit
       |  **/  (1 − α area)     \**
       |  *|                     |*
       +--+----+--------+--------+----+-----------> t
         -∞  −t_crit    0    +t_crit   +∞
         (reject)                       (reject)
```

## Definition

For the regression
`Y_i = b_0 + b_1·x_{1i} + ... + b_k·x_{ki} + ε_i` fitted on `n`
observations with `k` slope predictors, the **per-coefficient
t-test** of the null `H_0: b_j = b_{j,H_0}` against the two-sided
alternative `H_A: b_j ≠ b_{j,H_0}` uses the test statistic
`t = (b̂_j − b_{j,H_0}) / SE(b̂_j)`, where `SE(b̂_j)` is the
standard error of the OLS slope estimate. Under `H_0`, `t` is
distributed as Student-t with `n − k − 1` degrees of freedom. The
two-tailed rejection region at significance `α` is
`|t| > t_{1 − α/2, n − k − 1}`. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The **overall F-test** of joint significance tests
`H_0: b_1 = b_2 = ... = b_k = 0` against the alternative that at
least one slope is non-zero. The test statistic is `F = MSR / MSE`
from the ANOVA table, distributed under `H_0` as `F(k, n − k − 1)`.
The test is right-tailed: reject `H_0` when
`F > F_{1 − α, k, n − k − 1}`. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The **partial F-test** (nested F-test) compares an unrestricted
regression with `k` predictors to a restricted regression with
`k − q` predictors (the restriction zeroes `q` specific coefficients).
With `SSE_U` from the unrestricted fit and `SSE_R` from the restricted
fit, the test statistic is
`F_partial = ((SSE_R − SSE_U) / q) / (SSE_U / (n − k − 1))`,
distributed under `H_0: <the q restricted coefficients are zero>` as
`F(q, n − k − 1)`. The test is right-tailed at significance `α`.
**Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## Mathematical Reasoning

The per-coefficient t-statistic (source ASSERTS) is the source'
formula `t = (b̂_j − b_{j,H_0}) / SE(b̂_j)`, paired with the
Student-t reference distribution at `n − k − 1` degrees of freedom
under the classical-assumption set. The source asserts the test
statistic and its reference distribution; the formal Student-t
construction (Gaussian numerator over independent chi-square
denominator) is outside the source' scope and belongs to a raw
econometric reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The overall F-test (source ASSERTS) is the source' joint-significance
statistic `F = MSR / MSE` with the `F(k, n − k − 1)` reference
distribution under the joint null `H_0: all b_j = 0`. The source
assert the test statistic, the reference distribution, and the
right-tailed rejection rule; the formal derivation of the F-reference
distribution from the underlying sum-of-squares structure is outside
the source' scope. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The partial F-test (source ASSERTS) is the source' nested-comparison
statistic `F_partial = ((SSE_R − SSE_U) / q) / (SSE_U / (n − k − 1))`
with the `F(q, n − k − 1)` reference distribution under the
restricted-coefficient null. The source asserts the statistic and its
reference distribution; relationships between this statistic and
the per-coefficient t-test (e.g., the `q = 1` reduction) are outside
the source' scope. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The two-tailed vs one-tailed distinction (source ASSERTS) is that
two-tailed is the default for "is `b_j` different from `b_{j,H_0}`?"
questions; one-tailed is appropriate only when a priori the
alternative is directional (the slope is theoretically positive, or
theoretically negative). The rejection region shifts to one tail;
the critical value at the same `α` becomes smaller (in absolute
value) than the two-tailed value, so a one-tailed test has higher
power against the specified direction. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## See Also

- [`qm-multiple-linear-regression-foundations`](qm-multiple-linear-regression-foundations.md)
  — establishes the classical Gauss-Markov + normality assumption set
  whose validity makes the t-distribution and F-distribution reference
  distributions of this card's tests exact in finite samples
- [`qm-anova-table`](qm-anova-table.md) — provides the `MSR`, `MSE`,
  and `SSE` quantities that feed every test statistic in this card
  (the overall F is `MSR / MSE` and the partial F reads
  `(SSE_R − SSE_U)` off ANOVA tables of nested fits)
- [`qm-aic-bic-model-selection`](qm-aic-bic-model-selection.md) — an
  alternative model-comparison apparatus that does not produce a
  p-value: AIC and BIC use information-criterion ranking rather than
  hypothesis-test decision rules and so can be used to compare
  non-nested models that the partial F-test cannot reach

## Escalate to Raw When

Open the underlying source or a more rigorous econometric reference
when any of the criteria below applies. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- The errors are heteroskedastic or autocorrelated — the source pairs
  the t and F reference distributions with the homoskedastic /
  independent-error setting on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360`;
  diagnostic detection lives in
  [`qm-regression-assumption-violations`](qm-regression-assumption-violations.md),
  and any non-classical robust-variance machinery is outside the
  notes' scope and belongs to a raw econometric reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- Small-sample inference under non-normality — the source' t and F
  results on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` are stated under the
  classical normal-error assumption; finite-sample inference under
  non-normal residuals is outside the source' scope and belongs to a
  raw econometric reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- Joint linear restrictions beyond zeroing a subset of coefficients
  — the source' partial F-test on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360`
  treats the subset-zero restriction; general linear restrictions on
  the parameter vector are outside the source' scope and belong to a
  raw econometric reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- Nonlinear regression models — the source' t and F reference
  distributions on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` presume the
  linear-in-parameters regression that the source covers; the
  nonlinear-regression analogue is outside the source' scope and
  belongs to a raw econometric reference. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
