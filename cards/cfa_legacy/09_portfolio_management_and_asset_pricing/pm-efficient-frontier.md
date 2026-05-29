---
schema_version: "cacg.v0"
id: "pm-efficient-frontier"
title: "Efficient Frontier"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Efficient Frontier: framing mean-variance optimization and the efficient frontier as the geometric locus of optimal risk-return tradeoffs"
tags: ["portfolio-management", "efficient-frontier", "markowitz"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3130:4675"
    chunk_hash: "228b2e1be10ba23352d4e81bcdae0bccf51f8f261f23bb62f75fe646f04e7b3c"
    page_range: [3130, 3131]
    quote: "The curve that lies above and to the right of the global minimum-variance portfolio is referred to as the Markowitz efficient frontier"
    edge_type: "defines"
card_hash: "9377cd56d49a691633400ed844c7fe1f0d91d22c6019acebe59768bca7aaadf0"
---
# Efficient Frontier

## Intuition

Among the universe of feasible portfolios, an investor would never
hold one that has both lower expected return and higher variance than
another. The efficient frontier is the set of portfolios that survive
this dominance check — for each level of expected return, the
portfolio with minimum variance; equivalently, for each level of
variance, the portfolio with maximum expected return. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.490-519.

```
<!-- primitive: efficient-frontier source: _diagram_primitives.md -->
E[R]
   ^                                   efficient frontier
   |                              . - *
   |                          . -
   |                       .                         (max-Sharpe
   |                    .                              tangency M
   |                 *  <-- max-Sharpe tangent          if Rf added)
   |               .
   |             *  <-- global minimum-variance (GMV)
   |            .
   |           .       . . . . . . . . inefficient
   |          .      . dominated portfolios .
   |         .
   +-----------------------------------------> sigma
                                                (volatility)
```

The frontier curve in `(sigma, E[R])` space is the boundary of the
feasible region. Points strictly inside the boundary are dominated.
The investor's risk preference selects a single point on the
frontier — never inside it — because moving from inside to the
frontier strictly improves at least one of `(higher E[R], lower
sigma)`. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.490-519.

## Definition

The efficient frontier is the upper branch of the minimum-variance
frontier. For each target expected return, the minimum-variance
frontier keeps the feasible portfolio with the lowest variance; the
efficient frontier then discards the lower-return branch below the
global minimum-variance portfolio. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.490-519.

```
given target E[r_p]:
choose feasible w with minimum sigma_p^2 = w' Sigma w
subject to w' ones = 1
```

Sweeping the target-return parameter traces out the minimum-variance
frontier. The portfolio with the smallest variance over all risky
portfolios is the global minimum-variance portfolio (GMV). The
efficient frontier begins at the GMV and moves upward and to the
right. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.490-519.

```
GMV problem:
choose feasible w with the smallest sigma_p^2
```

The efficient frontier uses only risky assets. Adding a risk-free
asset creates capital allocation lines from `Rf` to candidate risky
portfolios; the steepest attainable line is tangent to the risky-asset
frontier and identifies the optimal risky portfolio. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.490-519.

## Mathematical Reasoning

The frontier starts from the investment opportunity set: all feasible
portfolios formed from the available risky assets. Adding an asset
class whose returns are not perfectly correlated with the existing
set expands the opportunity set northwest, improving the available
risk-return tradeoff. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.490-519.

The minimum-variance frontier removes portfolios that have avoidable
risk at the same expected return. The efficient frontier removes the
lower branch of the minimum-variance frontier because, for the same
volatility as a lower-branch portfolio, an upper-branch portfolio
offers higher expected return. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.490-519.

As an investor moves upward and rightward from the GMV along the
efficient frontier, expected return rises with volatility, but the
incremental return per additional unit of volatility declines. This
declining slope is why the investor's risk preference still matters:
the frontier gives candidates, not a single universal risky-asset
portfolio until the risk-free asset is added. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.490-519.

## See Also

- [`pm-diversification-and-correlation.md`](pm-diversification-and-correlation.md) — covariance matrix as the fundamental input to the optimization
- [`pm-capital-market-line.md`](pm-capital-market-line.md) — frontier with risk-free asset; tangency portfolio M

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R49 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.490-519.

- Specific quadratic-programming algorithms for computing frontier
  weights. Algorithm-implementation detail belongs in future-01
  quantitative methods. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.490-519.
- Investor utility / indifference-curve overlays for choosing a
  single point from the frontier; the current card stops at the
  frontier geometry. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.490-519.
- Estimation-error robustness of optimal weights — small changes in
  expected-return inputs produce large weight changes (a known
  practitioner concern). The reading discusses at intuition level;
  detailed treatment belongs in future-01. **Source:** CFA L1
  Curriculum (2022) Vol.6/pp.490-519.
