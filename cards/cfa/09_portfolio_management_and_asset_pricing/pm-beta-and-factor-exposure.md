---
schema_version: "cacg.v0"
id: "pm-beta-and-factor-exposure"
title: "Beta and Factor Exposure"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Beta and Factor Exposure: framing beta as the single-factor exposure measure; systematic vs idiosyncratic risk decomposition; the natural bridge from CAPM to multifactor framing"
tags: ["portfolio-management", "beta", "systematic-risk"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3175:4735"
    chunk_hash: "fcd8a87a4e7b173bf76680fdee99b7086b4cc7ff25f22ea4515aa41f31a6d9d2"
    page_range: [3175, 3175]
    quote: "beta captures an asset’s systematic risk, or the portion of an asset’s risk that cannot be eliminated by diversification."
    edge_type: "defines"
card_hash: "467b51b9cd59412d7adae627b2bfa36ef7eafbbcd77c9e5fd8aa48fe0f004873"
---
# Beta and Factor Exposure

## Intuition

Beta is a single number that captures one asset's sensitivity to the
market. Move the market by one unit; beta tells you how much the
asset moves on average. The decomposition implicit in beta is:
asset return splits into a systematic component (market-driven) and
an idiosyncratic residual (asset-specific). Diversification removes
the residual; the systematic component is what the investor cannot
diversify away and therefore the source of the equilibrium risk
premium. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.536-540.

```
   r_i (asset return)
       ^
       |       *                     *  *
       |     *    *               *
       |   *        *           *
       |  *            *      *  beta_i = slope of fitted line
       | *               *  *          (single-factor regression)
       |*                  *
       +------------------------> r_M (market return)
                              |  alpha_i = vertical intercept
                              |  epsilon_i = vertical residual
       (each * is one period; line is the OLS-style fit;
        beta is the slope; alpha is the intercept;
        residual is asset-specific noise)
```

In a well-diversified portfolio, the residual variance averages out
toward zero by the diversification argument; only the
beta-weighted systematic variance survives. The investor's portfolio
beta is the weighted average of constituent betas. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.536-545.

## Definition

The single-factor model decomposes asset return into a market-driven
component plus an asset-specific residual. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.536-540.

```
r_i = alpha_i + beta_i · r_M + epsilon_i
```

Beta is the regression coefficient — the covariance with the market
divided by the market's variance. Alpha is the vertical intercept —
the expected return when market return is zero. Epsilon is the
residual — uncorrelated with `r_M` by construction of the
decomposition. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.536-540.

```
beta_i = cov(r_i, r_M) / var(r_M)
alpha_i = E[r_i] - beta_i · E[r_M]
var(r_i) = beta_i^2 · var(r_M) + var(epsilon_i)
```

The variance decomposition splits asset variance into two
non-negative parts: the systematic variance (`beta_i^2 · var(r_M)`)
and the idiosyncratic variance (`var(epsilon_i)`). The two are
orthogonal because `cov(r_M, epsilon_i) = 0` by construction.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.536-540.

## Mathematical Reasoning

For a portfolio with weights `w` over `N` assets, the portfolio's
beta is the weighted average of constituent betas. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.540-545.

```
beta_p = sum_i w_i · beta_i
```

The portfolio's systematic variance is `beta_p^2 · var(r_M)`; the
portfolio's idiosyncratic variance is the weighted-squared average
of the constituent idiosyncratic variances (assuming uncorrelated
residuals across assets). For an equally-weighted portfolio of `N`
assets with uniform residual variance `sigma_eps^2`, the
idiosyncratic component shrinks at rate `1 / N` while the systematic
component is unchanged by diversification. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.536-540.

```
var(epsilon_p) = sigma_eps^2 / N    (equally-weighted, uniform residuals)
                 ->   0   as N grows large
```

This decomposition is the algebraic core of the CAPM's prediction
that only systematic risk earns a premium: in equilibrium investors
hold the market portfolio and bear only `beta_p^2 · var(r_M)` of
risk, having driven idiosyncratic variance to zero through
diversification. The pricing implication is that idiosyncratic
variance contributes to total risk but not to expected return.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.536-545.

The single-factor framing extends naturally to multifactor models:
replace `r_M` with a vector of factor returns and replace the scalar
beta with a vector of factor betas. The variance decomposition then
splits into per-factor systematic variances plus the idiosyncratic
residual. The single-factor case treated here is the L1-core
intuition; deeper multifactor pricing theory belongs in the AC-42
extension card. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.548-551.

A subtle but important point: the regression beta is a
backward-looking estimate from observed returns, while the CAPM beta
is a forward-looking equilibrium quantity. The two coincide under
the CAPM assumptions (homogeneous expectations, stationary
distribution); they diverge when the distribution shifts (regime
changes, structural breaks). The investor must check whether the
estimated beta is a stable proxy for the relevant forward-looking
exposure. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.540-565.

## See Also

- [`pm-capm-and-sml.md`](pm-capm-and-sml.md) — CAPM equilibrium pricing using beta as the systematic-risk price
- [`pm-diversification-and-correlation.md`](pm-diversification-and-correlation.md) — variance decomposition for arbitrary covariance structures
- [`fi-duration-and-convexity.md`](../06_fixed_income_and_credit/fi-duration-and-convexity.md) — duration is the fixed-income analogue of beta — a single-number sensitivity measure
- [`deriv-greeks-overview.md`](../07_derivatives_and_volatility/deriv-greeks-overview.md) — option Greeks decompose sensitivity to underlying; delta is the derivative analogue of beta

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R50 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.536-572.

- Beta-estimation choices such as lookback length, observation
  frequency, and market-index proxy. **Source:** CFA L1 Curriculum
  (2022) Vol.6/pp.536-540.
- Empirical evidence on the size, value, and momentum factors that
  motivated the multifactor extensions; the reading lists the
  documented anomalies and the AC-42 extension card formalizes the
  extension theory. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.548-551.
- Beta-instability detection methods (rolling-window estimation,
  regime-switching specifications); the reading flags instability,
  while method detail belongs in future-01. **Source:** CFA L1
  Curriculum (2022) Vol.6/pp.536-540.
