---
schema_version: "cacg.v0"
id: "pm-capital-market-line"
title: "Capital Market Line"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Capital Market Line: framing the capital market line geometry — the risk-free asset, the market portfolio, lending / borrowing portfolios, and the CML slope"
tags: ["portfolio-management", "cml", "market-price-of-risk"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3164:4718"
    chunk_hash: "3c7f42e67ff7dbb0e806085622090dd4707604743f3b908044b172dc934b871a"
    page_range: [3164, 3164]
    quote: "The y-intercept is the risk-free rate, and the slope of the line referred to as the market price of risk is [E(Rm) – Rf ]/σm."
    edge_type: "defines"
card_hash: "d3ddc4d1b2995abf17a8278e9923f0fbce00c34ef23d59977e039496c9ae3a15"
---
# Capital Market Line

## Intuition

When a risk-free asset is available, the upper boundary of the
feasible set in `(sigma, E[R])` space changes from a hyperbolic curve
(the unconstrained efficient frontier) to a straight ray emanating
from the risk-free point and tangent to the curve at one specific
portfolio. That tangent ray is the capital market line; the tangency
point is the market portfolio `M`. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.520-545.

```
<!-- primitive: capital-market-line source: _diagram_primitives.md -->
E[R]
   ^                                          CML
   |                                          /
   |                                         /
   |                               * M <----/    market / tangency
   |                            . /
   |                          ./
   |                        ./
   |                      ./   <-- lending segment (combine with Rf)
   |                    ./
   |                  ./
   |                ./
   |              ./
   |        Rf  *  <--  risk-free rate (beta = 0, sigma = 0)
   |
   +--------------------------------------> sigma
                                          (volatility)
```

Every rational mean-variance investor holds a combination of `Rf`
and `M`. The investor's risk tolerance selects the position along
the CML — fully invested in `Rf` (zero variance) at one extreme,
holding `M` at the tangency, or borrowing at `Rf` to lever up `M`
beyond the tangency in the borrowing segment. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.520-545.

## Definition

The CML equation expresses the tradeoff: every CML portfolio's
expected return is the risk-free rate plus a market-Sharpe-ratio
times that portfolio's standard deviation. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.520-545.

```
E[r_p] = Rf + (E[r_M] - Rf) · sigma_p / sigma_M
```

The slope `(E[r_M] - Rf) / sigma_M` is the Sharpe ratio of the market
portfolio — the highest Sharpe ratio achievable in the unconstrained
mean-variance world. The tangency portfolio `M` is the unique
efficient-frontier portfolio that maximizes this Sharpe ratio.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.520-545.

The CML's two segments are: the lending segment between `Rf` and `M`
(positive weight on `Rf` and `M`); the borrowing segment beyond `M`
(negative weight on `Rf` — borrowing — and weight greater than one on
`M`). The boundary case is `M` itself (zero weight on `Rf`).
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.520-545.

## Mathematical Reasoning

The CML's straight-line form follows from combining one risk-free
asset with the market portfolio. The risk-free asset has zero
standard deviation and zero covariance with the market portfolio, so
the portfolio's risk is proportional to the weight in `M` while the
portfolio's expected return is the weighted average of `Rf` and
`E[r_M]`. Substituting the weight relation gives the CML equation.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.520-545.

The tangency property follows from comparing capital allocation
lines. A line from `Rf` to a non-tangency efficient portfolio is
dominated by the steeper tangent line through `M`; the tangent CML
offers a higher expected return for the same volatility. **Source:**
CFA L1 Curriculum (2022) Vol.6/pp.520-545.

The two-fund separation theorem states the same result in portfolio
choice terms: efficient investors hold a combination of the risk-free
asset and one optimal risky portfolio. The investor's risk tolerance
enters through the weight on `Rf` versus `M`; the risky-asset block is
the same market portfolio in the CFA capital-market-theory setting.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.520-545.

When the borrowing rate exceeds the lending rate, the CML becomes
kinked at `M`. The lending segment from `Rf` to `M` uses the slope
`(E[r_M] - Rf) / sigma_M`; the borrowing segment to the right of `M`
uses the smaller slope `(E[r_M] - Rb) / sigma_M`. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.540-565.

## See Also

- [`pm-efficient-frontier.md`](pm-efficient-frontier.md) — risky-asset-only frontier; mean-variance optimization
- [`pm-capm-and-sml.md`](pm-capm-and-sml.md) — CAPM derivation that follows from the CML separation result

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R50 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.520-545.

- Two-fund separation proof at the algebraic level; the card keeps
  only the L1 result and its CML interpretation. **Source:** CFA L1
  Curriculum (2022) Vol.6/pp.520-545.
- Different lending and borrowing rates, including the two CML
  equations and kinked-line exhibit. **Source:** CFA L1 Curriculum
  (2022) Vol.6/pp.540-565.
- The empirical identification of `M` — what real-world index
  approximates the theoretical market portfolio. The reading
  discusses index-construction tradeoffs at intuition level.
  **Source:** CFA L1 Curriculum (2022) Vol.6/pp.520-545.
