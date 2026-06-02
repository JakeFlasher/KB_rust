---
schema_version: "cacg.v0"
id: "qm-panel-cb-factor-inference"
title: "Panel-Data CB-Arb Factor Inference"
reading_id: "01_quantitative_methods"
summary: "Panel-data CB-arb factor inference adjudicates pooled OLS vs fixed-effects (within) vs random-effects (GLS) estimators of the slope on (x_{it}, y_{it}). Greene Ch.11 derives the within transformation and the Hausman specification test; Wooldridge Ch.13–14 cross-presents the same machinery at undergraduate depth plus clustered standard errors for within-unit serial correlation."
tags: ["quantitative-methods", "panel-cb"]
citations:
  - source_id: "qm_greene_2019_econometric_analysis_8ed"
    chunk_id: "qm_greene_2019_econometric_analysis_8ed:p413:0633"
    chunk_hash: "dc4a2d92d0ce811e2291d7796cc18a4b614cafcaf999271452c022274c9529c4"
    page_range: [414, 414]
    quote: "The analysis of panel data allows the model builder to learn about economic processes while accounting for both heterogeneity"
    edge_type: "defines"
  - source_id: "qm_greene_2019_econometric_analysis_8ed"
    chunk_id: "qm_greene_2019_econometric_analysis_8ed:p455:0703"
    chunk_hash: "de5e7975456de619c55caee5ca65fc4aaa7fe1693422816b29fd6ac169f56cf3"
    page_range: [455, 456]
    quote: "The Hausman test is a useful device for determining the preferred specification of the common effects model."
    edge_type: "defines"
  - source_id: "qm_wooldridge_intro_econometrics_8ed"
    chunk_id: "qm_wooldridge_intro_econometrics_8ed:p482:1050"
    chunk_hash: "5bf19c5699520f6935d75b337c4eb54bd131a7739ad29abe0c0c64dca1de28ba"
    page_range: [482, 482]
    quote: "Because i denotes different cities, we call ai an unobserved city effect or a city fixed effect"
    edge_type: "supports"
  - source_id: "qm_wooldridge_intro_econometrics_8ed"
    chunk_id: "qm_wooldridge_intro_econometrics_8ed:p506:1107"
    chunk_hash: "cd9b0ea7cedfcb0d87ad0619336013664f86f172c5ee5bcf010b48abb442d7fe"
    page_range: [506, 506]
    quote: "A pooled OLS estimator that is based on the time-demeaned variables is called the fixed effects estimator or the within estimator."
    edge_type: "supports"
card_hash: "c6917c690d17b9754b03b12e20436599ce8bb0df03d3876ddc1708ae58494fe7"
---
# Panel-Data CB-Arb Factor Inference

## Intuition

A CB-arb cross-issuer factor signal must contend with the fact that
the conversion-premium, credit / equity ratio, and bond-floor distance
of any one issuer carry persistent, issuer-specific level effects that
are unrelated to the cross-sectional factor scores being tested. The
pooled-OLS estimator that the upstream factor-construction pipeline
(see [`qm-cb-arb-factor-construction`](qm-cb-arb-factor-construction.md))
fits to the stacked `(x_{it}, y_{it})` panel inherits an
omitted-variable bias whenever those issuer-specific levels are
correlated with the regressors — the unit-fixed-effects estimator
removes that bias by transforming each observation into a deviation
from its own issuer mean, sweeping the unobserved intercept `α_i`
out of the score equation; the practitioner-quoted CB-arb
relative-value pipeline at
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md)
benefits from this issuer-fixed-effects within-transformation
whenever the cross-sectional signal is estimated on data with
persistent issuer heterogeneity.
**Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

Greene Ch.11.3 illustrates the panel-data machinery on a
manufacturing-productivity panel of firms observed over multiple
years, where the within estimator strips out time-invariant firm
characteristics (managerial capability, capital vintage) that would
otherwise confound the marginal-productivity slope; the same
fixed-effects-by-issuer machinery applies to controlling for
unobserved issuer heterogeneity in CB-arb relative-value factor
inference (the CB-arb application grounding is supplied by the
explicit cross-link to
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md)
above and the downstream signal-validation discipline at
[`qm-signal-validation-oos-discipline`](qm-signal-validation-oos-discipline.md)
gates which fixed-effects-estimated factor scores survive the
out-of-sample test).
**Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

```
<!-- primitive: panel-fixed-effects-decomp source: _diagram_primitives.md -->
   y_{it} = α_i + β · x_{it} + ε_{it}
   ───────────────────────────────────────────────────────
                       ▲          ▲           ▲
                       │          │           │
        unit-fixed-effect      common      idiosyncratic
        (issuer / firm /       slope       within-unit
         person α_i)           β            shock ε_{it}

   Within transformation (sweeps α_i out):
   (y_{it} − ȳ_i) = β · (x_{it} − x̄_i) + (ε_{it} − ε̄_i)

   ───────────────────────────────────────────────────────
        Unit i = 1    │    Unit i = 2    │    Unit i = 3
                      │                  │
        ●     ●       │      ●           │           ●
          ●           │   ●     ●        │     ●  ●
                      │                  │
        ─── α_1 ───   │   ─── α_2 ───   │   ─── α_3 ───
                      │                  │
        within-i β identified after subtracting α_i
```

## Definition

The panel-data regression `y_{it} = α_i + β · x_{it} + ε_{it}`
indexed over cross-sectional units `i ∈ {1, ..., N}` and time
periods `t ∈ {1, ..., T}` admits three canonical estimators of the
slope `β`: the pooled-OLS estimator
treats `α_i = α` constant across units and fits the stacked sample
directly; the unit-fixed-effects (within) estimator allows each `α_i`
to be a free parameter and either subtracts the within-unit mean
from both `y_{it}` and `x_{it}` (the within transformation) or
introduces `N` unit dummies (the LSDV equivalent); the random-effects
estimator treats `α_i` as a unit-specific random draw from a
distribution with `E[α_i | x_{it}] = 0` and uses the GLS-weighted
sample. Greene Ch.11.4 derives the algebra of the within
transformation and proves that the fixed-effects slope estimator is
unbiased and consistent under strict exogeneity `E[ε_{it} | x_{i,1},
..., x_{i,T}, α_i] = 0`, regardless of whether `α_i` is correlated
with `x_{it}`. The random-effects estimator is more efficient under
the additional zero-correlation assumption but inconsistent under
its violation.
**Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

The Hausman specification test adjudicates the fixed-vs-random
choice by computing a quadratic-form statistic in the difference
between the two slope estimators, `H = (β̂_FE − β̂_RE)' · (V̂_FE −
V̂_RE)^{-1} · (β̂_FE − β̂_RE)`, which is asymptotically chi-square
distributed with degrees of freedom equal to the number of slope
parameters under the null of zero correlation between `α_i` and
`x_{it}`. Greene Ch.11.5 derives the variance-difference matrix
algebra and gives the null-distribution argument. Rejection of the
null is the operational signal that issuer-fixed-effects must be
retained for unbiased CB-arb factor-slope inference; the practitioner
application of this gate to the CB-arb cross-issuer factor pipeline
is documented at
[`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md)
where the relative-value-screen scoring sits downstream of the
issuer-fixed-effects choice made here.
**Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

The Wooldridge Intro 8e treatment reframes the same machinery at
undergraduate depth: the within (or "fixed effects") estimator is
introduced via first-differencing for the two-period case and via
the within-mean subtraction for the general T-period case; the
between estimator (cross-sectional regression on unit means) is
contrasted with the within estimator as the variance-decomposition
complement; the random-effects estimator is positioned as the GLS-
weighted combination of within and between variation under the
exogeneity-of-`α_i` assumption. Wooldridge Intro Ch.13-14 also
introduces the clustered-standard-errors correction that adjusts
the OLS variance estimator for within-unit serial correlation in
the residuals.
**Source:** 01_Quantitative_Methods/Introductory Econometrics A Modern Approach, 8e (Jeffrey M. Wooldridge) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.421-485.

## Mathematical Reasoning

The pooled-OLS estimator (source ASSERTS) of the stacked panel
`y_{it} = α + β · x_{it} + (α_i − α + ε_{it})` is biased and
inconsistent whenever `Cov(x_{it}, α_i) ≠ 0` because the composite
error `u_{it} = (α_i − α) + ε_{it}` is correlated with the
regressor. Greene derives the within (fixed-effects) estimator
`β̂_FE = [Σ_i Σ_t (x_{it} − x̄_i)(x_{it} − x̄_i)']^{-1} · Σ_i Σ_t
(x_{it} − x̄_i)(y_{it} − ȳ_i)` by least-squares minimisation of the
within-transformed residual sum of squares; the unbiasedness of
`β̂_FE` under strict exogeneity follows from the orthogonality of
the demeaned regressor with the demeaned idiosyncratic shock.
**Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

The random-effects estimator (source ASSERTS) imposes
`E[α_i | x_{it}] = 0` and `Var(α_i) = σ_α²` and applies GLS to the
composite-error specification with covariance matrix `Σ = σ_ε² · I_T
+ σ_α² · ι_T · ι_T'` where `ι_T` is the `T × 1` vector of ones; the
GLS transformation subtracts a fraction `θ` of the within-unit mean
from each observation, with `θ = 1 − √(σ_ε² / (σ_ε² + T · σ_α²))`.
The random-effects estimator is more efficient than fixed effects
under the zero-correlation assumption but inconsistent if the
assumption fails.
**Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

The Hausman specification test (source ASSERTS) exploits the fact
that under the null of `Cov(α_i, x_{it}) = 0` both `β̂_FE` and
`β̂_RE` are consistent but `β̂_RE` is efficient, so the difference
`β̂_FE − β̂_RE` has variance `V̂_FE − V̂_RE` (Hausman's lemma); under
the alternative `Cov(α_i, x_{it}) ≠ 0`, `β̂_FE` remains consistent
while `β̂_RE` is inconsistent so the difference has nontrivial
probability limit. The statistic `H = (β̂_FE − β̂_RE)' · (V̂_FE −
V̂_RE)^{-1} · (β̂_FE − β̂_RE)` is asymptotically `χ²_k` under the
null, with `k` equal to the number of time-varying regressors.
**Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

The between estimator (source ASSERTS) regresses unit-time-averaged
responses `ȳ_i` on unit-time-averaged regressors `x̄_i` across the
`N` cross-sectional units, exploiting only cross-unit variation and
discarding the within-unit information. The OLS estimator of the
pooled panel decomposes as a matrix-weighted average of the within
and between estimators, with weights determined by the relative
within-unit and between-unit variation in the regressors. Wooldridge
Intro Ch.14 motivates the decomposition for the two-period case via
first-differencing.
**Source:** 01_Quantitative_Methods/Introductory Econometrics A Modern Approach, 8e (Jeffrey M. Wooldridge) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.421-485.

The clustered-standard-errors correction (source ASSERTS) replaces
the OLS variance estimator with a sandwich form that aggregates the
score contributions within each unit cluster before averaging
across clusters: `V̂_cluster = (X'X)^{-1} · (Σ_i X_i' · ε̂_i · ε̂_i'
· X_i) · (X'X)^{-1}`, where `X_i` is the `T × k` matrix of regressors
for unit `i` and `ε̂_i` is the `T × 1` residual vector. The correction
delivers consistent standard errors under arbitrary within-unit
serial correlation of the idiosyncratic shock, as long as the
number of clusters grows.
**Source:** 01_Quantitative_Methods/Introductory Econometrics A Modern Approach, 8e (Jeffrey M. Wooldridge) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.421-485.

## See Also

- [`qm-cb-arb-factor-construction`](qm-cb-arb-factor-construction.md) — the upstream factor-construction step whose stacked-panel pooled-OLS estimator inherits the omitted-variable bias addressed here by the within (unit-fixed-effects) estimator
- [`qm-signal-validation-oos-discipline`](qm-signal-validation-oos-discipline.md) — the downstream out-of-sample validation discipline that gates which fixed-effects-estimated factor scores survive the K-fold cross-validation test
- [`cb-relative-value-screens`](../08_convertible_bonds/cb-relative-value-screens.md) — the practitioner-quoted CB-arb cross-issuer relative-value screen whose factor-slope inference benefits from the issuer-fixed-effects within transformation when issuer-specific intercepts are correlated with the relative-value regressors

## Escalate to Raw When

Open Greene 8e or Wooldridge Intro 8e directly when any of the
criteria below applies. **Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.

- The panel inference must handle a dynamic-panel specification with
  a lagged dependent variable on the right-hand side — the
  Arellano-Bond / system-GMM machinery is out of scope per the v7+
  CB-arb extension boundary discipline (see frontmatter `Out of
  scope:` field for the chapter-level boundary specification).
  **Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.
- The panel response is binary, ordinal, or censored (logit / probit
  / Tobit panel) — the nonlinear panel-likelihood machinery is out
  of scope per the v7+ CB-arb extension boundary policy.
  **Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.
- The cross-sectional unit count is small (single-digit issuers)
  and asymptotic-cluster justification fails — bootstrap or
  small-cluster-corrected inference depth is out of scope.
  **Source:** 01_Quantitative_Methods/Introductory Econometrics A Modern Approach, 8e (Jeffrey M. Wooldridge) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.421-485.
- The deeper graduate-depth fixed/random-effects asymptotic theory
  is required (the would-be Wooldridge Cross/Panel 2e MIT Press 2010
  primary anchor) — the on-disk PDF is non-quotable per Critical
  Rule 4 (SCAN-quality OCR scan); the re-activation trigger is a
  clean publisher PDF acquisition. **Source:** 01_Quantitative_Methods/William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf pp.413-470.
