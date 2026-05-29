---
schema_version: "cacg.v0"
id: "pm-capm-and-sml"
title: "CAPM and the Security Market Line"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "CAPM and the Security Market Line: framing the CAPM single-factor pricing equation and the security market line as its geometric expression"
tags: ["portfolio-management", "capm", "sml"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3180:4744"
    chunk_hash: "b2edaa29233287111ac22275536466555f019c65379d8d16ac84fee6a8a2eec8"
    page_range: [3180, 3181]
    quote: "The security market line (SML) is a graphical representation of the capital asset pricing model with beta, reflecting systematic risk, on the x-axis and expected return on the y-axis."
    edge_type: "defines"
card_hash: "b2f2f086f68f5cd05ad10407998b7e6762d012fb65b49f308e5ca9083e117950"
---
# CAPM and the Security Market Line

## Intuition

The CAPM is the equilibrium implication of two-fund separation: in
the world where every investor holds the same tangency portfolio `M`,
each individual asset's expected excess return must be proportional
to its covariance with `M`, scaled by the market risk premium per
unit of market variance. Equivalently, the expected excess return on
any asset is its beta times the market's expected excess return. The
security market line is the same statement in `(beta, E[R])` space —
a straight line through `(0, Rf)` and `(1, E[R_M])`. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.540-565.

```
<!-- primitive: security-market-line source: _diagram_primitives.md -->
E[R]
   ^                                     SML  /
   |                                         /
   |                                        /
   |                                  *    /  <-- mispriced asset
   |                                      /        (above SML = under-
   |                              *      /         priced; below SML =
   |                           E[Rm] *  /          overpriced)
   |                                   /
   |                                  /
   |                                 /
   |                                /
   |                          Rf  *
   |                              |
   +-----------+------+------+------------> beta
              0     0.5    1.0   1.5   2.0
```

The investor uses the SML in two ways. As a benchmark: any asset
plotting above the SML offers a positive expected excess return per
unit of beta beyond the equilibrium, suggesting it is underpriced.
As a discount-rate engine: the SML's expected return is the
opportunity cost of capital for any project or asset with the given
beta. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.540-565.

## Definition

The CAPM equation expresses each asset's expected excess return as a
linear function of its beta to the market portfolio. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.540-565.

```
E[r_i] = Rf + beta_i · (E[r_M] - Rf)
```

The beta of asset `i` to the market is the covariance of `r_i` with
`r_M` divided by the variance of `r_M`. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.540-565.

```
beta_i = cov(r_i, r_M) / var(r_M)
```

The market risk premium `(E[r_M] - Rf)` is the slope of the SML in
`(beta, E[R])` space; the SML's intercept on the vertical axis is
`Rf`. The SML passes through two reference points by construction:
the risk-free rate at `beta = 0` and the market portfolio at
`beta = 1`. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.540-565.

## Mathematical Reasoning

CAPM moves the risk-return tradeoff from total volatility to
systematic risk. Beta measures how much asset `i` co-moves with the
market portfolio relative to the market's own variance. Because
diversifiable risk can be removed in a broad portfolio, CAPM assigns
an expected-return premium to beta risk rather than to standalone
variance. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.540-565.

The SML and the CML are different geometric objects with the same
algebraic content. The CML lives in `(sigma, E[R])` space and applies
only to portfolios on the efficient frontier (well-diversified
combinations of `Rf` and `M`). The SML lives in `(beta, E[R])` space
and applies to every asset, well-diversified or not. The SML uses
beta — the systematic-risk component — rather than total volatility,
because under CAPM equilibrium only systematic risk earns a premium.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.540-565.

A specific illustration: an asset with `beta_i = 0` has no systematic
risk and earns the risk-free rate at equilibrium. An asset with
`beta_i = 1` earns the market's expected return. An asset with
`beta_i > 1` earns a premium above the market commensurate with the
amplified systematic exposure. The proportionality is exact under
the CAPM assumptions; deviations from the SML in observed data are
either pricing errors (mispricings) or evidence of model
misspecification (e.g. additional priced factors). **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.540-565.

The CAPM rests on assumptions: investors are risk-averse utility
maximizers, markets are frictionless, borrowing and lending at the
risk-free rate is possible, investors share a single-period horizon,
investors have homogeneous expectations, investments are infinitely
divisible, and investors are price takers. Empirical violations of
these assumptions motivate APT and practical multifactor models in
the extension batch. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.540-565.

## See Also

- [`pm-capital-market-line.md`](pm-capital-market-line.md) — two-fund separation that produces the CAPM equilibrium
- [`pm-beta-and-factor-exposure.md`](pm-beta-and-factor-exposure.md) — beta as a single-factor exposure measure; systematic / idiosyncratic decomposition
- [`fi-credit-spread-machinery.md`](../06_fixed_income_and_credit/fi-credit-spread-machinery.md) — separates default / liquidity spread mechanics from this card's systematic-risk discount-rate framing
- [`deriv-greeks-overview.md`](../07_derivatives_and_volatility/deriv-greeks-overview.md) — option Greeks decompose sensitivity to underlying; CAPM beta is the equity analogue at the asset level

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R50 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.540-565.

- The CAPM assumption set in detail and the empirical evidence on
  each assumption (size effect, value effect, momentum effect — all
  documented violations of the single-factor model). **Source:** CFA
  L1 Curriculum (2022) Vol.6/pp.540-565.
- Beta-instability across regimes; how beta varies between
  expansions and recessions and why this matters for the discount-
  rate use of the SML. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.555-572.
- The Roll critique on identifying the true market portfolio; the
  reading covers at intuition level. **Source:** CFA L1 Curriculum
  (2022) Vol.6/pp.540-565.
