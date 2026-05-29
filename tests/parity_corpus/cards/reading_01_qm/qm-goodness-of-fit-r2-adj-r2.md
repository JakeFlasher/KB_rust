---
schema_version: "cacg.v0"
id: "qm-goodness-of-fit-r2-adj-r2"
title: "Goodness of Fit: R² and Adjusted R²"
reading_id: "reading_01_qm"
summary: "framing the coefficient of determination R² and its degree-of-freedom-adjusted counterpart adjusted R² for a fitted regression — what fraction of variation the regressors explain, why R² mechanically rises with added regressors, and how the adjusted form penalises that mechanical rise"
tags: ["definition", "goodness-of-fit"]
card_edges:
  - target: "qm-multiple-linear-regression-foundations"
    edge_type: "extends"
citations:
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p003:0001"
    chunk_hash: "3b9eef70d0ea521b06ea524550cf336c4520f46b5c118e5afaa7f5825071f2c5"
    page_range: [3, 4]
    quote: "Measures of Goodness of Fit UUT Yi- Yi B 4i-4 GOLDEN EDUCATIC I. Coefficient of determination ( R2 )"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p004:0002"
    chunk_hash: "39655e5140c045f186eb0642098261d7c728237518b2a3c111a3bebd5c8f0cd3"
    page_range: [4, 5]
    quote: "X t Adding a new independent variable may either increase or t= b1-0 |>1 Sb decrease the R2"
    edge_type: "supports"
card_hash: "8ce102412dd76af0e0de1b90e6bf2bccf8386fc85e974db92a71a9db101374dc"
---
framing the coefficient of determination R² and its degree-of-freedom-adjusted counterpart adjusted R² for a fitted regression — what fraction of variation the regressors explain, why R² mechanically rises with added regressors, and how the adjusted form penalises that mechanical rise

## Original Card (preserved verbatim)

## Intuition

The coefficient of determination `R²` reads off the ANOVA partition
directly: it is the fraction of total variation in `Y` that the
fitted regression's predictors explain. A value near `1` means the
predictors line up almost perfectly with the dependent variable; a
value near `0` means they have almost no joint explanatory power. The
`R²` itself does not say whether each predictor individually matters
(that is the t-test's job) or whether the joint fit is statistically
significant (the F-test's job). **Source:** notes/CFA_note_2.ocr.pdf
pp.3-4.

`R²` has one mechanical property that makes it dangerous as a model-
comparison statistic: it can only rise (never fall) when an extra
predictor is added to the regression, because the OLS fit on the
larger predictor set is at least as good as on the smaller one. Two
regressions with different predictor counts cannot be ranked on raw
`R²` alone. The adjusted `R̄²` builds in a degree-of-freedom penalty
that makes adding a junk predictor cost more degrees of freedom than
it adds explained variation; `R̄²` can fall when a weak predictor
enters, and can even go negative if the regression fits worse than
the sample-mean baseline. **Source:** notes/CFA_note_2.ocr.pdf
pp.3-4.

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

The **coefficient of determination** for a regression
`Y_i = b_0 + b_1·x_{1i} + ... + b_k·x_{ki} + ε_i` fitted on `n`
observations is `R² = SSR / SST = 1 − SSE / SST`, where `SST`, `SSR`,
and `SSE` are the total, regression, and error sums of squares from
the ANOVA decomposition. Because `SSR + SSE = SST` (with an
intercept), `R² ∈ [0, 1]` and the two forms are algebraically
identical. **Source:** notes/CFA_note_2.ocr.pdf pp.3-4.

The **adjusted coefficient of determination** introduces a degree-of-
freedom penalty: `R̄² = 1 − (1 − R²) · (n − 1) / (n − k − 1)`, where
`k` is the count of slope predictors and `n − k − 1` is the residual
degrees of freedom. The penalty makes `R̄² ≤ R²` with equality only
when `k = 0`, and lets `R̄²` fall when a new predictor's contribution
to `R²` is smaller than the degree-of-freedom cost it imposes.
**Source:** notes/CFA_note_2.ocr.pdf pp.3-4.

A useful equivalent form is `R̄² = 1 − MSE / (SST / (n − 1))`, where
`MSE = SSE / (n − k − 1)` is the residual mean square and
`SST / (n − 1)` is the sample variance of `Y`. This form makes it
explicit that `R̄²` compares the residual variance estimate against
the unconditional variance of the dependent variable; if the
predictors do not improve on the sample-mean baseline,
`MSE > SST / (n − 1)` and `R̄²` is negative. **Source:**
notes/CFA_note_2.ocr.pdf pp.3-4.

## Mathematical Reasoning

The `R² = SSR / SST = 1 − SSE / SST` identity (source ASSERTS)
follows immediately from the ANOVA partition `SST = SSR + SSE`: the
fraction of total variation captured by the regression is the share
that lives in the explained component. The two forms are algebraically
identical given the partition; the notes assert both forms without
deriving them. **Source:** notes/CFA_note_2.ocr.pdf pp.3-4.

The mechanical-rise property — `R²` never decreases when a regressor
is added (source ASSERTS) — follows from the fact that OLS on the
augmented design matrix can always replicate the smaller-model fit by
zeroing the new coefficient, so the augmented `SSR` is at least the
smaller `SSR` and the augmented `SSE` is at most the smaller `SSE`.
The notes state this consequence without spelling out the
nested-optimization proof. **Source:** notes/CFA_note_2.ocr.pdf
pp.3-4.

The adjusted-R² penalty correction `(n − 1) / (n − k − 1)` (source
ASSERTS) inflates the residual ratio `(1 − R²)` to penalise added
regressors: as `k` grows the multiplier grows, so adding a predictor
whose own contribution to `R²` is smaller than the inflation factor
lowers `R̄²`. The exact rule is that `R̄²` rises when adding a
predictor if and only if the new predictor's partial t-statistic
exceeds `1` in absolute value — a much weaker bar than the
two-sided `|t| > t_{1-α/2}` rejection threshold used for inference,
which is why `R̄²` is a weak filter against junk predictors.
**Source:** notes/CFA_note_2.ocr.pdf pp.3-4.

The non-negativity bound `R̄² ≥ 0` does NOT hold (source ASSERTS):
the equivalent form `R̄² = 1 − MSE / (SST / (n − 1))` makes the
negative case explicit — whenever the residual mean square
`MSE = SSE / (n − k − 1)` exceeds the sample variance
`SST / (n − 1)` of `Y`, the bracketed ratio exceeds `1` and `R̄²`
becomes negative. This happens when `R²` is low relative to `k / n`
(too many regressors for the explanatory power they buy), and the
notes state both the `R²` lower bound `R̄² ≤ R²` and the possibility
of negative `R̄²`. A negative `R̄²` is a diagnostic that the
predictors carry less information than the unconditional mean —
a regression-validity warning. **Source:**
notes/CFA_note_2.ocr.pdf pp.3-4.

## See Also

- [`qm-multiple-linear-regression-foundations`](qm-multiple-linear-regression-foundations.md)
  — gives the OLS estimator that produces `Ŷ_i` and the residuals
  `(Y_i − Ŷ_i)` that drive `SSR` and `SSE` in this card's identities
- [`qm-anova-table`](qm-anova-table.md) — provides the upstream
  `SST = SSR + SSE` decomposition that `R²` reads off, plus the
  `MSE = SSE / (n − k − 1)` mean square that the equivalent-form
  `R̄²` expression rescales against the sample variance of `Y`

## Escalate to Raw When

Open the underlying source or a more rigorous econometric reference
when any of the criteria below applies. **Source:**
notes/CFA_note_2.ocr.pdf pp.3-4.

- Comparing non-nested models — the mechanical-rise / adjusted-R²
  framework relies on the augmented-design-matrix argument that holds
  only for nested regressions. For non-nested model comparison, use
  information-criterion alternatives (AIC, BIC) or out-of-sample
  predictive measures. **Source:** notes/CFA_note_2.ocr.pdf pp.3-4.
- Comparing across samples of different size — the degree-of-freedom
  adjustment uses sample-specific `n`, so two `R̄²` values from
  different `n` are not directly comparable. **Source:**
  notes/CFA_note_2.ocr.pdf pp.3-4.
- Out-of-sample predictive interest — in-sample `R²` and `R̄²` say
  nothing about how the model generalizes; cross-validated `R²` or a
  held-out test sample is the appropriate measure for prediction
  questions. **Source:** notes/CFA_note_2.ocr.pdf pp.3-4.
- Heteroskedastic or autocorrelated errors — the ANOVA partition
  underlying `R²` remains an exact algebraic identity, but the
  inferential link between `R²` and the F-test reference distribution
  is broken; use the regression-assumption-violations sibling card
  for the diagnostic-and-correction layer. **Source:**
  notes/CFA_note_2.ocr.pdf pp.3-4.
