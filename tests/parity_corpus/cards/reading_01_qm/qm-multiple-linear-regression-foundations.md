---
schema_version: "cacg.v0"
id: "qm-multiple-linear-regression-foundations"
title: "Multiple Linear Regression Foundations"
reading_id: "reading_01_qm"
summary: "framing what a multiple linear regression IS as a statistical primitive — the population conditional-mean equation, the parameter identifiers, the OLS estimator's classical Gauss-Markov assumptions, and the linearity-in-parameters distinction that separates linear from nonlinear regression"
tags: ["definition", "regression"]
citations:
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p001:0000"
    chunk_hash: "93c56ed12de8411fad7822569e4e1a91752744ca27a3ede32b1d3e12cc83e0cd"
    page_range: [1, 2]
    quote: "U 1. Muttiple liner regression (1)Yi=bo+bix1+b2xi++bkxki+Ei bo: intercept term b:slope coefficent lexpectedincrease in"
    edge_type: "supports"
card_hash: "c50783bc4879022425dfdab15121b545d7034980d092d76655de22764f11241d"
---
framing what a multiple linear regression IS as a statistical primitive — the population conditional-mean equation, the parameter identifiers, the OLS estimator's classical Gauss-Markov assumptions, and the linearity-in-parameters distinction that separates linear from nonlinear regression

## Original Card (preserved verbatim)

## Intuition

A multiple linear regression posits that the conditional mean of a
target variable `Y` is a linear function of `k` explanatory variables
`(x_1, ..., x_k)` with an additive zero-mean error term `ε`. The
regression is "multiple" because more than one predictor enters
simultaneously; it is "linear" not in the predictors themselves but
in the parameters `(b_0, b_1, ..., b_k)` — quadratic or cross-product
predictor terms remain admissible as long as the parameters multiply
functions of the predictors linearly. **Source:** notes/CFA_note_2.ocr.pdf pp.1-2;
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-320.

The intuition driving the entire OLS edifice is identification: the
classical assumptions (linearity, exogeneity, homoskedasticity,
normality of errors, and independence of predictors from the error
term) collectively guarantee that the ordinary-least-squares
estimator recovers the population parameters with a clean
unbiased-and-efficient sampling distribution. Each assumption
disabled changes a property of the estimator (bias, efficiency,
inference validity), and the diagnostic toolkit downstream is
organized around which assumption a deviation breaks. **Source:**
notes/CFA_note_2.ocr.pdf pp.1-2; CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.320-340.

```
                  population conditional-mean equation
                  ──────────────────────────────────────
   E[Y | x_1, ..., x_k] = b_0 + b_1·x_1 + b_2·x_2 + ... + b_k·x_k

                   observed data-generating equation
                   ─────────────────────────────────
      Y_i = b_0 + b_1·x_{1i} + b_2·x_{2i} + ... + b_k·x_{ki} + ε_i

       ┌──────────────┐                           ┌────────────┐
       │  intercept   │     slope coefficients    │  residual  │
       │     b_0      │   b_1 ... b_k = marginal  │    ε_i     │
       │ (Y when      │   effect of each x_j on   │ (zero-mean,│
       │  every x = 0)│   Y holding the other     │  i.i.d.    │
       │              │   predictors fixed        │  Gaussian) │
       └──────────────┘                           └────────────┘
```

## Definition

Multiple linear regression is the parametric model
`Y_i = b_0 + b_1 · x_{1i} + b_2 · x_{2i} + ... + b_k · x_{ki} + ε_i`
where `Y_i` is the i-th observation of the dependent (target)
variable, `x_{ji}` is the i-th observation of the j-th independent
(explanatory) predictor, `b_0` is the intercept, `(b_1, ..., b_k)`
are slope coefficients each interpreted as the expected change in
`Y` per unit increase in the corresponding `x_j` holding all other
predictors fixed, and `ε_i` is the residual capturing every effect
on `Y` not accounted for by the systematic part. **Source:**
notes/CFA_note_2.ocr.pdf pp.1-2; CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-310.

The classical Gauss-Markov assumptions that make OLS the best
linear unbiased estimator are linearity (the conditional mean is
truly linear in the parameters), homoskedasticity (`Var(ε_i)` is
constant across observations), independence (`ε_i` is uncorrelated
across observations and the predictors are uncorrelated with the
error), normality (`ε_i ~ N(0, σ²)` for finite-sample inference
validity), and no-perfect-multicollinearity (the predictors carry
distinct identifying variation). **Source:** notes/CFA_note_2.ocr.pdf pp.1-2;
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.310-340.

The independent variables are conventionally treated as fixed
(non-random) in the classical setup; in the more permissive
stochastic-regressors framework the same OLS properties survive
under the exogeneity condition `E[ε_i | X] = 0`. **Source:**
notes/CFA_note_2.ocr.pdf pp.1-2; CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.340-360.

## Mathematical Reasoning

The OLS estimator (source ASSERTS) chooses the parameter vector
`b̂ = (b̂_0, ..., b̂_k)` to minimize the sum of squared residuals
`Σ_{i=1}^{n} (Y_i − b̂_0 − Σ_j b̂_j · x_{ji})²` over the sample of
`n` observations. The notes assert the closed-form matrix solution
`b̂ = (X^T X)^{-1} X^T Y` for the OLS estimator, valid when
`(X^T X)` is invertible — which fails precisely when the predictors
are perfectly collinear. The notes do not derive the normal-equation
first-order conditions; the card states the closed-form result and
labels the gap. **Source:** notes/CFA_note_2.ocr.pdf pp.1-2;
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-330.

The Gauss-Markov property (source ASSERTS) is that under the
classical assumptions, `b̂` is unbiased (`E[b̂] = b`), has variance
`Var(b̂) = σ² · (X^T X)^{-1}`, and is efficient (smallest variance
among unbiased linear estimators); under normality of the errors,
`b̂` is also Gaussian, which is the property that subsequent t-tests
and F-tests for the regression coefficients depend on for valid
finite-sample inference. The notes assert these properties without
proof. **Source:** notes/CFA_note_2.ocr.pdf pp.1-2;
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.330-360.

The partial-derivative interpretation (source ASSERTS) of each slope
coefficient `b_j` is the expected change in `Y` per unit change in
`x_j` holding the other predictors fixed; this is the property that
breaks under multicollinearity: when two predictors carry nearly
identical information, the partial-derivative interpretation becomes
ill-defined and the OLS estimates of the affected coefficients have
inflated variance even though the joint fit remains adequate. The
notes state the interpretation and the multicollinearity caveat
without deriving the variance-inflation algebra. **Source:**
notes/CFA_note_2.ocr.pdf pp.1-2;
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.350-360.

## See Also

- [`eq-equity-cost-of-capital-estimation`](../05_equity/eq-equity-cost-of-capital-estimation.md) — CAPM as a univariate linear regression of stock excess return on market excess return; the slope coefficient is the security's beta and inherits all OLS assumptions
- [`eq-fama-french-construction-at-security-level`](../05_equity/eq-fama-french-construction-at-security-level.md) — Fama-French factor model as a multi-predictor linear regression with HML / SMB / market factors; a direct multivariate application of this card's machinery

## Escalate to Raw When

Open CFA L1 Vol.1 Reading 8 directly or the more rigorous
econometric references when any of the criteria below applies.
**Source:** notes/CFA_note_2.ocr.pdf pp.1-2; CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf
Vol.1/pp.300-360.

- The model is *nonlinear in the parameters* (logistic regression,
  Cox proportional hazards) — the closed-form OLS solution and the
  Gauss-Markov properties no longer apply. **Source:** CFA L1
  Curriculum (2022) Vol.1/pp.300-310.
- The error structure is heteroskedastic or autocorrelated and the
  inference needs robust standard errors (HAC, Newey-West) — the
  regression-assumption-violations sibling card holds the
  diagnostic-and-correction layer. **Source:** CFA L1 Curriculum
  (2022) Vol.1/pp.330-360.
- The data are panel (cross-section × time) and within / between
  variation must be separated — this card models only the pooled
  cross-section. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf
  Vol.1/pp.340-360.
- The dependent variable is censored, truncated, or count-typed —
  Tobit, Heckman, or Poisson models are needed; OLS gives biased
  estimates. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-310.
