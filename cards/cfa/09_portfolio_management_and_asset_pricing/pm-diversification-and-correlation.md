---
schema_version: "cacg.v0"
id: "pm-diversification-and-correlation"
title: "Diversification and Correlation"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Diversification and Correlation: framing covariance, correlation, and naive vs Markowitz diversification as the geometric basis for the efficient frontier"
tags: ["portfolio-management", "diversification", "correlation"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3115:4655"
    chunk_hash: "68277a1919303ebc33b03b3c5e4e9b10a06bc65b1b9d7c0af2335e3be017ba44"
    page_range: [3115, 3116]
    quote: "The correlation coefficient between two assets determines the effect on portfolio risk when the two assets are combined."
    edge_type: "defines"
card_hash: "d8eca79184c30e7fedee7a44ec010a33b3f7414d3b2855c1f8da18cd936be3be"
---
# Diversification and Correlation

## Intuition

Combining two assets whose returns are imperfectly correlated produces
a portfolio whose standard deviation is less than the weighted-average
of the two asset standard deviations. The reduction is the
diversification benefit and depends on the correlation coefficient:
when correlation equals one (perfect comovement) there is no benefit,
and when correlation equals minus one the portfolio variance can be
driven to zero with the right weights. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.460-500.

```
   sigma_p (portfolio std-dev)
       ^
       |  rho = +1 (no benefit; linear combination)
       |  ----------------
       |  *               *
       |   \             /
       |    \           /
       |     \   *     /     rho = 0 (some benefit)
       |      \ /  *  /
       |       *      *      rho < 0 (more benefit)
       |        \    /
       |         \  /
       |          *          rho = -1 (zero variance achievable
       +----+---+---+--->     for one weight pair)
            asset A      asset B
       (varying weight from pure B to pure A along curve)
```

The geometric picture: as correlation falls, the curve from `B` (pure
asset B) to `A` (pure asset A) bows further left into lower-variance
territory. The investor exploits this geometry to find the
minimum-variance combination — the input to the efficient frontier
construction. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.460-500.

## Definition

The covariance between two return series is the expected product of
their deviations from their respective means. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.460-500.

```
cov(r_A, r_B) = E[(r_A - E[r_A]) (r_B - E[r_B])]
```

The correlation coefficient is the covariance normalized by the
product of standard deviations; it is bounded in the closed interval
`[-1, +1]`. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.460-500.

```
rho(r_A, r_B) = cov(r_A, r_B) / (sigma_A · sigma_B)
```

For a two-asset portfolio with weights `w_A` and `w_B = 1 - w_A`, the
portfolio variance is the algebraic combination of asset variances
weighted by the squared weights, plus a covariance cross-term.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.460-500.

```
sigma_p^2 = w_A^2 sigma_A^2 + w_B^2 sigma_B^2 + 2 w_A w_B rho sigma_A sigma_B
```

For a portfolio of arbitrarily many assets, the variance is a
quadratic form in the weight vector against the covariance matrix.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.460-500.

```
sigma_p^2 = w' Sigma w
```

## Mathematical Reasoning

The two-asset variance formula reveals the core of the diversification
benefit. Treating `sigma_p^2` as a function of `rho` while holding
weights and asset volatilities fixed, the formula is linear in `rho`
and increasing. Therefore lower correlation reduces portfolio
variance. When `rho < +1`, the portfolio standard deviation is less
than the weighted-average standard deviation of the two assets.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.460-500.

When `rho` equals one, the portfolio standard deviation collapses to
the weighted average of the constituent standard deviations and there
is no diversification benefit. When `rho` equals zero, the covariance
cross-term drops out and risk falls below the perfect-correlation
case. When `rho` equals minus one, a weight pair can drive portfolio
variance to zero; for negative correlations above minus one, risk is
reduced further but is generally not eliminated. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.460-500.

The naive-diversification limit is the asymptotic behavior of an
equally-weighted portfolio as the asset count grows. Decompose the
quadratic form into diagonal and off-diagonal contributions: the
diagonal term carries the average variance scaled by one over the
asset count and vanishes asymptotically; the off-diagonal term
carries the average covariance and persists as the residual systematic
component. Naive diversification eliminates idiosyncratic variance
but cannot eliminate the average covariance. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.500-519.

```
average variance contribution: bar_var / asset_count   ->   0  as count grows
average covariance contribution: bar_cov                ->   bar_cov  (limit)
```

This decomposition is the algebraic underpinning of the
systematic-vs-idiosyncratic distinction that the CAPM single-factor
framework formalizes. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.500-519.

## See Also

- [`pm-return-and-risk-fundamentals.md`](pm-return-and-risk-fundamentals.md) — variance and standard deviation as the inputs to covariance / correlation
- [`pm-efficient-frontier.md`](pm-efficient-frontier.md) — Markowitz mean-variance optimization that uses this covariance structure as input

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R49 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.460-500.

- Sample covariance estimator construction; the relationship between
  sample covariance and population covariance under stationarity
  assumptions; small-sample bias treatment. **Source:** CFA L1
  Curriculum (2022) Vol.6/pp.460-500.
- Multivariate normal density theory and the role of the covariance
  matrix in joint-distribution shape; when this matters for downside
  measures beyond variance. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.460-500.
- Specific examples of cross-asset correlation regimes during stress
  episodes (the reading discusses correlation breakdown during
  drawdowns). **Source:** CFA L1 Curriculum (2022) Vol.6/pp.500-519.
