---
schema_version: "cacg.v0"
id: "pm-portfolio-perspective"
title: "Portfolio Perspective"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Portfolio Perspective: framing what a portfolio is from the investor perspective — the rationale for evaluating holdings together rather than security by security, and the steps in a portfolio-management process"
tags: ["portfolio-management", "portfolio-perspective", "diversification"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3042:4550"
    chunk_hash: "0fd3bd67e777e66912203c5079aeec621048c9f5fdfebf87defe69ddda914376"
    page_range: [3042, 3043]
    quote: "Should we invest in individual securities, evaluating each in isolation, or should we take a portfolio approach?"
    edge_type: "defines"
card_hash: "da91e99fde1cf0abb2352aa4263a35398a42948fabe576a16bd39887d6b06816"
---
# Portfolio Perspective

## Intuition

A portfolio is a single decision unit — a basket of holdings that the
investor evaluates together rather than security by security. The
portfolio perspective shifts attention from any one asset's return
distribution to the JOINT distribution of the basket, where co-movements
between holdings determine aggregate risk. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.405-441.

```
   individual holdings              portfolio
   ----------------------          ------------
        a1   a2   a3                +--------+
        |    |    |     ----->      | r_p =  |
        v    v    v                 | sum    |
       r_a  r_b  r_c                | w_i r_i|
                                    +--------+
   evaluated separately:           evaluated as a single
   each return distribution        joint distribution; risk
   stands alone                    depends on covariances
```

The investor cares about the portfolio outcome — the basket's return
and risk — because consumption is funded from the basket as a whole.
A holding's contribution to the basket is what matters, not its
stand-alone characteristics. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.405-441.

## Definition

A portfolio is a vector of weights `w = (w_1, ..., w_N)` over `N`
candidate holdings, with `sum_i w_i = 1`. Each weight represents the
fraction of total invested capital allocated to holding `i`. The
portfolio's return over a period is the weighted sum of individual
holding returns. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.405-441.

```
r_p = sum_i w_i r_i
```

Weights may be non-negative (long-only) or signed (long-short, with
short positions carrying negative weights subject to short-sale
constraints). **Source:** CFA L1 Curriculum (2022) Vol.6/pp.405-441.

The portfolio-management process is the cyclical sequence: planning
(client analysis, IPS authoring), execution (asset allocation,
security selection, trade implementation), and feedback (performance
measurement, rebalancing, IPS revision). Each step constrains the
next; the IPS sets the boundary conditions under which execution and
feedback operate. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.405-441.

## Mathematical Reasoning

The portfolio's expected return is the weighted average of constituent
expected returns. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.442-475.

```
E[r_p] = sum_i w_i E[r_i]
```

This linearity follows from the linearity of expectation and holds
regardless of the joint return distribution. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.442-475.

The portfolio's variance is NOT a weighted average — it depends on
covariances among the holdings. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.442-475.

```
var(r_p) = sum_i sum_j w_i w_j cov(r_i, r_j)
```

When all pairwise covariances are non-positive or when correlations
are strictly less than `1`, the portfolio variance falls below the
weighted-average variance of the constituents. This inequality is the
algebraic core of the diversification benefit and the reason the
investor evaluates holdings jointly rather than individually.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.442-475.

The implication for the investor is that a holding's contribution to
portfolio risk depends on its covariances with the rest of the
basket, not on its own variance in isolation. A high-variance asset
that is uncorrelated with the rest of the portfolio may add little
aggregate risk; a moderate-variance asset that moves in lockstep with
existing holdings may add disproportionately. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.405-441.

## See Also

- [`pm-return-and-risk-fundamentals.md`](pm-return-and-risk-fundamentals.md) — return measures (HPR, arithmetic, geometric) and risk measures (variance, semi-variance) at the holding and portfolio level
- [`pm-diversification-and-correlation.md`](pm-diversification-and-correlation.md) — covariance and correlation as the inputs that determine portfolio variance

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R48 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.405-441.

- Investor-type taxonomy beyond retail / institutional needs detail
  (e.g. defined-benefit pension fund, sovereign wealth fund,
  endowment) — Vol.6 R48 enumerates and the deeper detail belongs in
  future-13. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.
- Active-vs-passive decision framing or market-efficiency forms
  appearing within R48 — these route to dedicated sibling cards
  authored in the second batch. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.420-441.
- Specific portfolio-process step ordering and stakeholder mapping
  beyond the cyclical planning / execution / feedback summary above
  — the reading enumerates substeps that the present card abstracts.
  **Source:** CFA L1 Curriculum (2022) Vol.6/pp.405-441.
