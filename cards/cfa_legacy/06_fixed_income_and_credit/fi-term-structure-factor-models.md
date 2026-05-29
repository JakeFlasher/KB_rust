---
schema_version: "cacg.v0"
id: "fi-term-structure-factor-models"
title: "Term-Structure Factor Models (PCA Decomposition)"
reading_id: "06_fixed_income_and_credit"
summary: "Principal Component Analysis on the historical covariance of cross-tenor yield changes extracts three orthogonal factors — level, slope, curvature — that explain the bulk of curve variance. The factor-mimicking-portfolio basis is the natural coordinate system for curve trades and a more parsimonious hedge target than the full key-rate vector."
tags: ["fixed-income", "term-structure"]
citations:
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p187:0232"
    chunk_hash: "6867b6cbe721a0e39840e3e22ee1b594848df99556900f19afb99a61153421c0"
    page_range: [187, 187]
    quote: "The final section of the chapter introduces principal component analysis, which is an empirical description of how rates move together across the curve"
    edge_type: "defines"
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p169:0207"
    chunk_hash: "69b5156567502fd736f9052245a71fb53212097df66434b7d6daf4189d17802f"
    page_range: [169, 169]
    quote: "The risk that rates along the term structure move by different amounts is known as curve risk"
    edge_type: "supports"
  - source_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed"
    chunk_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed:p232:0301"
    chunk_hash: "61b13947189302dbb81be3016ca79aac47e1f382328098d80eb8092aa5c0c048"
    page_range: [232, 233]
    quote: "Clearly, if one wishes to directly model this instantaneous forward rate, there is no liberty in selecting the drift of its process, as it is completely determined by the chosen volatility coefficient"
    edge_type: "supports"
card_hash: "059cf68847a77973c4d1fb3df4511bf75372da3d184aa2605d660dc7b43b4cdb"
---
# Term-Structure Factor Models (PCA Decomposition)

## Intuition

Historical curve moves are not random across tenors — they exhibit strong cross-tenor correlation. If the 2y rises 10 basis points today, the 5y and 10y typically rise too, by amounts that are correlated but not identical. Principal Component Analysis (PCA) on the historical covariance matrix of yield changes across tenors decomposes the curve's joint variability into a small number of independent factors. Empirically the first three factors — **level**, **slope**, and **curvature** — explain the bulk of the variance; subsequent factors are noise. The decomposition gives the practitioner a parsimonious basis in which to express curve views and hedge curve risk. **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.

```
empirical PCA decomposition of curve yield changes
   eigenvalues of the cross-tenor yield-change covariance
   matrix sorted by magnitude:
       λ_1 >> λ_2 > λ_3 >> λ_4 ≥ ... ≥ λ_N
   factor share of variance (typical Treasury panel):
       factor 1 (level):       dominant share
       factor 2 (slope):       large minority share
       factor 3 (curvature):   small minority share
       factors 4 and beyond:   residual noise
   factor loadings (the eigenvectors) across tenors:
       level loading:    ≈ constant across tenors
       slope loading:    negative at short tenors,
                          positive at long tenors
       curvature loading: positive at short and long
                          tenors, negative at the middle
   level shift             slope tilt             curvature bend
     loading                 loading                  loading
       ^   ^^^^^^^^^           ^         /              ^   .  .
       |  '         '          |        /               |  '    '
       |  '         '          |       /                | '      '
       |  '         '          |  --- /                 |---       ---
       |  '         '          | /                      |
       +------+----> tenor    +------+----> tenor      +------+----> tenor
```

## Definition

A **factor model** for the term structure decomposes the yield-change vector `Δy = (Δy_1, ..., Δy_N)` across `N` tenors as `Δy ≈ L · ε_1 + S · ε_2 + C · ε_3 + residual` where `L`, `S`, `C` are the per-tenor loading vectors and `ε_1, ε_2, ε_3` are the factor realizations. **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.

**Principal Component Analysis (PCA)** is the procedure that extracts the factors from the empirical covariance matrix of historical `Δy`. The factors are constructed to be orthogonal (uncorrelated by construction) and ordered by decreasing variance. Each factor's loading vector is the corresponding eigenvector of the covariance matrix. **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.

The **level factor** has a loading vector approximately constant across tenors. A unit-level-factor move shifts all tenors by roughly the same amount — a parallel shift. **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.

The **slope factor** has a loading vector that is negative at short tenors and positive at long tenors (or the mirror). A unit-slope-factor move tilts the curve: short tenors move in one direction and long tenors in the opposite, with intermediate tenors barely moving. **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.

The **curvature factor** has a loading vector that is positive at the short and long ends and negative in the middle (or the mirror). A unit-curvature-factor move bends the middle of the curve relative to the wings. **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.

A **factor-mimicking portfolio** is a portfolio of bonds whose PnL responds exclusively to one factor. The factor-mimicking portfolio for the slope factor (for example) has zero DV01-equivalent to the level and curvature factors and unit DV01 to the slope factor. **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.

## Mathematical Reasoning

The empirical covariance matrix `Σ` of cross-tenor yield changes has eigenvalues `λ_1 ≥ λ_2 ≥ ... ≥ λ_N ≥ 0` and corresponding eigenvectors `v_1, ..., v_N`. PCA's spectral decomposition writes `Σ = Σ_k λ_k · v_k · v_k^T`. The first-three-factor approximation truncates at `k = 3` and discards the residual. **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.

The variance share of factor `k` is `λ_k / Σ_j λ_j`. For Treasury panels the empirical shares are approximately the level factor dominant, the slope factor a large minority, the curvature factor a small minority, and remaining factors residual noise — Tuckman documents this stability across multiple decades and across the US, UK, German, and Japanese curves. The level factor's near-monopoly on variance explains why the L1 parallel-shift duration framework is a useful first-order approximation. **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.

The link to key-rate / partial-DV01 hedging from [`fi-key-rate-and-partial-duration.md`](./fi-key-rate-and-partial-duration.md#mathematical-reasoning) is the factor-mimicking-portfolio construction: rather than hedging a portfolio against `N` independent key-rate moves (which over-specifies the hedge given that historical moves are highly correlated), the practitioner hedges against the 2-3 PCA factors that explain the bulk of variance. This yields a more parsimonious hedge that exploits the empirical correlation structure. **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.

The butterfly construction from [`fi-butterfly-and-curve-trades.md`](./fi-butterfly-and-curve-trades.md#mathematical-reasoning) lives naturally in this factor basis: a duration-neutral 2-10-30 butterfly is by construction orthogonal to the level factor (DV01-zero to a parallel shift) and has loadings on the slope and curvature factors that depend on the wing-vs-body weighting. The PCA basis is therefore the natural coordinate system for curve trades. **Source:** Tuckman & Serrat 3e (2011) Ch.5 pp.153-169.

A critical caveat: PCA loadings are stable over long panels but exhibit regime shifts. Tuckman documents that during quantitative-easing regimes the slope factor's loading at the long end weakens (because central-bank purchases pin long-end yields), and during stress events the level factor's variance share rises (correlation across tenors approaches one). Practitioners therefore recompute the PCA periodically over rolling windows rather than relying on a once-and-done estimation. **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.

The connection to the HJM forward-rate framework of [`fi-hjm-forward-rate-framework.md`](./fi-hjm-forward-rate-framework.md#mathematical-reasoning) is conceptual: HJM specifies the forward-rate volatility structure as a continuous function of tenor; the PCA factors are the empirical analogue. A consistent multi-factor HJM model has 2-3 factors whose volatility loading vectors match the empirical level / slope / curvature loadings. **Source:** Brigo+Mercurio (2006) Ch.5 pp.155-190.

## See Also

- [`fi-key-rate-and-partial-duration.md`](fi-key-rate-and-partial-duration.md) — partial-DV01 framework that the factor-mimicking portfolio parsimonizes
- [`fi-butterfly-and-curve-trades.md`](fi-butterfly-and-curve-trades.md) — curve trades expressed in the level / slope / curvature factor basis
- [`fi-hjm-forward-rate-framework.md`](fi-hjm-forward-rate-framework.md) — continuous-time multi-factor forward-rate model whose volatility loadings mirror the empirical PCA loadings
- [`fi-duration-and-convexity.md`](fi-duration-and-convexity.md) — L1 single-factor framework that the level factor justifies as a first-order approximation

## Escalate to Raw When

Open Tuckman & Serrat 3e Ch.6 (Empirical Approaches to Risk Metrics
and Hedging) directly when any of the criteria below applies.
**Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.

- The card user needs the exact panel of Treasury yield changes
  and the corresponding eigenvalue / eigenvector estimates over
  a specific historical window — Tuckman provides the methodology
  but the empirical output requires the dated data series.
  **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.
- A specific regime-shift period (e.g. a quantitative-easing
  episode where the slope-factor loading flattened at the long
  end) requires the dated rolling-window PCA decomposition.
  **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.
- The L2 multi-factor HJM forward-rate model with explicit
  volatility-loading calibration to the empirical PCA factors is
  required — escalate to Brigo+Mercurio for the continuous-time
  framework; Tuckman provides the empirical motivation but not
  the full HJM calibration recipe.
  **Source:** Brigo+Mercurio (2006) Ch.5 pp.155-190.
- A non-Treasury curve (the swap curve, a corporate-spread
  curve, a sovereign-spread curve) requires a separate PCA on
  that panel — out of this card's Treasury-curve framing.
  **Source:** Tuckman & Serrat 3e (2011) Ch.6 pp.171-199.
