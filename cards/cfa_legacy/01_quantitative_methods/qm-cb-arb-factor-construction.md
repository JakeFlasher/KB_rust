---
schema_version: "cacg.v0"
id: "qm-cb-arb-factor-construction"
title: "CB-Arb Factor Construction"
reading_id: "01_quantitative_methods"
summary: "CB-arb cross-sectional factor scoring uses linear regression (OLS), ridge / lasso shrinkage for high-dimensional feature spaces, and boosting (AdaBoost / gradient boosting) for non-linear loadings. ESL Ch.3 supplies linear methods; ESL Ch.10 supplies boosting; Greene Ch.4 supplies the OLS foundation; Wooldridge Ch.7 supplies categorical dummies."
tags: ["quantitative-methods", "cb-arb"]
citations:
  - source_id: "qm_eslii_2009_2ed"
    chunk_id: "qm_eslii_2009_2ed:p062:0076"
    chunk_hash: "2f0a8f3eab3b538b8cd8ea3f9a0b63f831fdab18194e3c51bdfc5742c04b6aa5"
    page_range: [62, 63]
    quote: "The linear model either assumes that the regression function E(Y |X) is linear, or that the linear model is a reasonable approximation."
    edge_type: "defines"
  - source_id: "qm_eslii_2009_2ed"
    chunk_id: "qm_eslii_2009_2ed:p063:0077"
    chunk_hash: "fd2ef7cd3789673981b08c85ca5627b1ad31021b1f314d29c5053db087fb26c2"
    page_range: [63, 64]
    quote: "The most popular estimation method is least squares, in which we pick the coefficients β"
    edge_type: "defines"
  - source_id: "qm_eslii_2009_2ed"
    chunk_id: "qm_eslii_2009_2ed:p356:0450"
    chunk_hash: "e39f0a3a81ac8475188e5d95accd81c4e6def31e0e67e2174b186728787fd8ab"
    page_range: [356, 357]
    quote: "The purpose of boosting is to sequentially apply the weak classification algorithm to repeatedly modified versions of the data, thereby producing a sequence of weak classifiers"
    edge_type: "supports"
  - source_id: "qm_greene_2019_econometric_analysis_8ed"
    chunk_id: "qm_greene_2019_econometric_analysis_8ed:p144:0205"
    chunk_hash: "19fb0306d69b907475b4d54430885d103309f48c4859f518559703a2e5c413cb"
    page_range: [144, 144]
    quote: "CHAPTER 4 ✦ Estimating the Regression Model by Least Squares"
    edge_type: "supports"
card_hash: "320e81255ee9c1e5eb334910b5d24dd681caec32c72f2ee9d95735681756de2b"
---
# CB-Arb Factor Construction

## Intuition

The practitioner-quoted CB-arb relative-value screens described in
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md)
(adjusted conversion premium, credit/equity ratio comparison,
implied-vs-realized vol gap, parity-spread to bond-floor) consume a
cross-sectional ranker that scores each convertible on a universe
by its deviation from an expected fair-value benchmark. The
statistical-learning toolkit provides two layers of machinery for the
ranker-construction step: (i) the linear regression of issuer-specific
features on a target return / mispricing label, and (ii) shrinkage of
the coefficient vector via ridge or lasso penalties when the feature
dimension is high relative to the issuer count. The ESL framing
decomposes each step into a symbolic estimator and a
regularization-versus-bias trade-off without prescribing any
particular weighting scheme, which is what makes the toolkit reusable
across cross-sections (see
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md)
for the CB-specific feature set). **Source:**
01_Quantitative_Methods/ESLII_print12_toc.pdf pp.43-99.

Boosting (ESL Ch.10) provides a third layer when the linear-loading
specification is too restrictive: the algorithm composes a sequence
of weak classifiers and combines them into a strong cross-sectional
ranker. ESL Ch.10 develops boosting as a general cross-sectional
classification toolkit anchored on synthetic data and standard
machine-learning benchmark sets, not on a CB-arb-specific example.
The CB-arb adaptation (treating each convertible as an issuer-level
observation, stacking cheapness / momentum / quality features as the
predictor matrix) is documented in the CB vertical's practitioner
literature; the cross-sectional ranker output feeds the screens in
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md).
**Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.337-384.

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

A cross-sectional factor model is the linear specification
`Y_i = b_0 + b_1 · x_{1i} + b_2 · x_{2i} + ... + b_k · x_{ki} + ε_i`
where `Y_i` is the realized return or mispricing label on observation
`i`, `x_{ji}` is the value of the `j`-th factor on observation `i`,
`(b_1, ..., b_k)` are the loading coefficients to be estimated, and
`ε_i` is the residual. The OLS estimator `b̂ = (XᵀX)⁻¹ XᵀY` estimates
the loading coefficients under the classical Gauss-Markov assumptions
(see
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md)
for the convertible-issuer cross-section instance: `Y_i` is a
parity-spread or implied-vs-realized vol gap label on convertible
`i`, `x_{ji}` is one of the four practitioner-quoted CB-arb factor
families on convertible `i`). **Source:**
01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.143-180.

When the feature count `k` is large relative to the observation count
`n`, OLS becomes ill-conditioned and the coefficient estimates
inflate; the ridge estimator augments the objective with an L2
penalty `min_b Σ_i (Y_i − x_iᵀ b)² + λ · Σ_j b_j²` and the lasso
estimator augments with an L1 penalty `min_b Σ_i (Y_i − x_iᵀ b)² +
λ · Σ_j |b_j|`. The L1 penalty has the additional effect of forcing
some coefficients exactly to zero, performing simultaneous
regularization and feature selection. The hyperparameter `λ` is held
abstract here; the sibling signal-validation card will cross-reference
the K-fold cross-validation discipline that selects it
out-of-sample. The CB-arb application of high-dimensional feature
spaces (issuer-level cheapness scores aggregated across multiple
look-back windows) is documented at the practitioner cross-link
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md).
**Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.43-99.

The boosting estimator generalises beyond linear loadings by composing
a sequence of weak classifiers `(h_1, h_2, ..., h_M)` and combining
them into a strong cross-sectional ranker
`F_M(x) = Σ_{m=1}^{M} α_m · h_m(x)` where each round `m` re-weights
the observations that the prior rounds misclassified. The CB-arb
construction step uses boosting as a classifier for the cheapness /
momentum / quality factor families when the linear-loading
specification is too restrictive; the CB-specific feature engineering
and screen-output interpretation live in
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md).
**Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.337-384.

## Mathematical Reasoning

The OLS estimator (source ASSERTS) is the closed-form minimiser of
the sum of squared residuals `Σ_i (Y_i − x_iᵀ b)²` over the
parameter vector `b ∈ ℝ^{k+1}`. The first-order conditions yield the
normal equations `XᵀX b̂ = XᵀY`, whose solution is `b̂ = (XᵀX)⁻¹ XᵀY`
when `(XᵀX)` is invertible. The Gauss-Markov assumptions guarantee
unbiasedness `E[b̂] = b` and minimum variance `Var(b̂) = σ² (XᵀX)⁻¹`
among linear unbiased estimators; the classical Wald inference toolkit
extends to the cross-sectional factor setting unchanged. **Source:**
01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.143-180.

The ridge estimator (source ASSERTS) modifies the OLS first-order
conditions to `(XᵀX + λ · I) b̂_{ridge} = XᵀY`, whose closed-form
solution is `b̂_{ridge} = (XᵀX + λ · I)⁻¹ XᵀY`. The `λ · I` term
inflates the diagonal of the cross-product matrix and is what
guarantees invertibility even when `k > n` or when the predictor
columns are nearly collinear. The lasso estimator has no closed-form
solution because the L1 penalty is non-differentiable at zero;
coordinate-descent and least-angle-regression algorithms compute the
solution path over `λ`. The shrinkage-and-selection effect is the
key property that makes lasso useful for high-dimensional CB-arb
feature spaces. **Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.43-99.

The boosting estimator (source DECOMPOSES) builds the strong ranker
`F_M(x)` additively: at round `m`, the weak learner `h_m` is fitted
to the residuals of the prior ensemble `F_{m-1}(x)`, then combined
with a step-size coefficient `α_m` whose value depends on the chosen
loss function. AdaBoost uses an exponential loss `L(y, F) = exp(−y F)`
that yields `α_m = (1/2) · ln((1 − err_m) / err_m)` where `err_m`
is the weighted error of `h_m`; gradient boosting generalises this
to any differentiable loss by fitting `h_m` to the negative gradient
of the loss at `F_{m-1}`. The CB-arb adaptation (whether to set
classification targets — binary buy/sell signal — or regression
targets — continuous expected-return score — for the cheapness /
momentum / quality labels) is a practitioner-engineering choice
documented at
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md).
**Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.337-384.

The Wooldridge Intro treatment (source ASSERTS) of the cross-section
regression frames the same OLS machinery at undergraduate depth with
explicit dummy-variable encoding for categorical features (industry,
country, credit-rating bucket). The dummy-variable trap warns against
perfect multicollinearity when the dummy set spans the categorical
attribute completely; the standard remedy is to omit one category as
the baseline. CB-arb factor models that include issuer fixed effects
via dummies inherit this trap structure unchanged (see
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md)
for the CB-specific issuer / industry / credit-rating bucket
typology). **Source:** 01_Quantitative_Methods/Introductory Econometrics A Modern Approach, 8e (Jeffrey M. Wooldridge) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.176-235.

## See Also

- [`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md) — the four practitioner-quoted CB-arb relative-value screens (adjusted conversion premium, credit/equity ratio comparison, implied-vs-realized vol gap, parity-spread to bond-floor metric) that consume the factor-construction step's cross-sectional ranker output as input
- [`pm-multifactor-asset-pricing-intuition`](../09_portfolio_management_and_asset_pricing/pm-multifactor-asset-pricing-intuition.md) — the multifactor asset-pricing framing for portfolio-level factor exposure interpretation; the CB-arb factor construction here is an asset-class-specific application of the same conceptual surface

## Escalate to Raw When

Open ESL 2e directly, Greene 8e directly, or the more rigorous
econometric / ML references when any of the criteria below applies.
**Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.43-99.

- The factor feature space includes non-numeric kernel similarity
  measures (text-derived sentiment vectors, graph-based issuer
  similarity scores) — kernel methods are out of scope per the v7+
  CB-arb extension boundary discipline (see `Out of scope:`
  frontmatter for the chapter-level boundary specification).
  **Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.43-99.
- The factor model needs deep neural-network architecture or random
  forests beyond boosting — both are out of scope under the v7+
  CB-arb extension policy (see `Out of scope:` frontmatter for the
  chapter-level boundary specification); consult the relevant raw
  references only if the practitioner deliberately steps outside the
  CB-arb pilot scope. **Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.337-384.
- The factor specification requires unsupervised clustering or
  self-organising maps to discover latent issuer groups — clustering
  is out of scope (see `Out of scope:` frontmatter); route to the
  appropriate unsupervised-learning reference if the use case truly
  requires it. **Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.43-99.
- The factor model needs panel-data fixed/random-effects estimation
  to control for issuer-level unobserved heterogeneity — that
  machinery is in the sibling `qm-panel-cb-factor-inference.md` card
  (see `Out of scope:` frontmatter for the panel-data deferral).
  **Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.143-180.
- The hyperparameter `λ` for shrinkage / boosting needs
  out-of-sample validation — that discipline is in the sibling
  `qm-signal-validation-oos-discipline.md` card (see `Out of scope:`
  frontmatter for the signal-validation deferral).
  **Source:** 01_Quantitative_Methods/ESLII_print12_toc.pdf pp.43-99.
