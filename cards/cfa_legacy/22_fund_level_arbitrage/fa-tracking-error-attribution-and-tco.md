---
schema_version: "cacg.v0"
id: "fa-tracking-error-attribution-and-tco"
title: "Tracking Difference vs Error & Total Cost of Ownership"
reading_id: "22_fund_level_arbitrage"
summary: "Tracking difference is ex-post cumulative under/outperformance; tracking error is its volatility. PM attributes the difference into misallocation (overweights/underweights summing to zero), cash drag, and a fee/cost residual. Total cost of ownership extends the expense ratio with implicit trading costs, taxes, and securities-lending credits."
tags: ["tracking-error", "total-cost-of-ownership", "misallocation"]
citations:
  - source_id: "fa_weiner_2021_etf_portfolio"
    chunk_id: "fa_weiner_2021_etf_portfolio:p116:0114"
    chunk_hash: "02addd552031cfa6e12d44b12e24d1f1ba97786be5adaa9d80cec5d886f9942b"
    page_range: [117, 117]
    quote: "By definition, the misallocations must sum to zero, and therefore there are always positive and negative misallocations"
    edge_type: "defines"
  - source_id: "fa_weiner_2021_etf_portfolio"
    chunk_id: "fa_weiner_2021_etf_portfolio:p122:0119"
    chunk_hash: "e0eff3a33cf39c75be11f85d96f671764909c1f655d1bca22a8fcfdfba4b92ef"
    page_range: [123, 123]
    quote: "Tracking error, under this construct, would be the standard deviation of the outperformance, which is annualized by multiplying by the square root of 252 (since variance scales linearly)."
    edge_type: "defines"
  - source_id: "fa_madhavan_2016_etfs_new_dynamics"
    chunk_id: "fa_madhavan_2016_etfs_new_dynamics:p080:0095"
    chunk_hash: "6accad22c7f174bf0f75ffc94f2cead1f719b56e4a7c83c675d8150e4cf38905"
    page_range: [81, 81]
    quote: "these reports overstate tracking error to the benchmark index because price changes around dividend ex dates show as volatility."
    edge_type: "supports"
---
# Tracking Difference vs Error & Total Cost of Ownership

## Intuition
A passive ETF promises the index return less fees, but the manager cooks from the same recipe with imperfect ingredients: closing-price-vs-traded-price gaps, round lots, dividend-reinvestment lags, and corporate actions all nudge portfolio weights off index weights. Two distinct quantities measure the gap. *Tracking difference* is the cumulative directional miss over a period (it can be positive or negative). *Tracking error* is the volatility of the daily misses (always non-negative). A fund that lags by exactly one basis point every day has a large tracking difference but near-zero tracking error — consistent, just consistently behind. The manager's job is to attribute the difference into controllable buckets so it can be hunted down: which slice came from fees, which from holding cash idle, which from being over/underweight the wrong names.

```
 index weights w^I_i        portfolio weights w^P_i
        |                          |
        +----------- diff ---------+
                     |
        outperformance alpha_t = (P_t - P_{t-1}) - (I_t - I_{t-1})
                     |
   +-----------------+-----------------+-------------------+
   | misallocation   |   cash drag     |  residual epsilon |
   | sum_i (w^P-w^I)r| (1-sum w^P)*(rc -| -fee + SL - TC    |
   |   sums to 0     |   portfolio ret) |                   |
   +-----------------+-----------------+-------------------+
                     |
   TE = stdev(alpha_t) * sqrt(252)
```

**Source:** Weiner (2021) ch.7 pp.106-119.

## Definition
- **Tracking difference (TD):** the ex-post difference in cumulative performance over a stated period between the fund (measured on NAV) and the index it tracks; signed.
- **Tracking error (TE):** the standard deviation (volatility) of the daily performance differences, annualized; non-negative. Weiner reserves "tracking error" for this second sense and uses "underperformance/overperformance" for the signed first sense.
- **Misallocation:** the PM's overweights and underweights relative to the index; by construction the misallocations sum to zero, so there are always offsetting positive and negative legs.
- **Cash drag:** the (positive or negative) impact of holding cash instead of deploying it pro rata across index holdings; the cash position as a fraction of AUM times the gap between portfolio/index return and the cash return.
- **Total cost of ownership (TCO):** the explicit total expense ratio (TER) plus implicit costs and revenues — trading costs (bid-offer spread, market impact), the tracking difference itself, and taxes on distributions — netted against securities-lending income.

**Source:** Weiner (2021) ch.7 pp.107-119; Madhavan (2016) §5.2.1, §5.4 pp.79-83.

## Mathematical Reasoning
Define daily outperformance as the alpha between portfolio and index log returns:

  alpha_t = (P_t - P_{t-1}) - (I_t - I_{t-1}).

Decomposing through the security weights, Weiner's appendix rearranges alpha into two structural terms plus a cost residual:

  alpha_t = sum_{i in M} (w^P_{i,t} - w^I_{i,t}) r_{i,t}   [misallocation]
            + (1 - sum_i w^P_{i,t}) (r^c_t - r^P_t)         [cash drag]
            + epsilon_t,    where epsilon_t = -fee_t + SL_t - TC_t.

The misallocation weights satisfy the constraint sum_i (w^P_{i,t} - w^I_{i,t}) = 0, so misallocation is a zero-sum reallocation: every overweight is funded by an offsetting underweight. The cash-drag term is keyed to portfolio (not index) weights and vanishes when the portfolio is fully invested (sum_i w^P = 1). Tracking error is then

  TE = stdev(alpha_t) * sqrt(252),

variance scaling linearly in time. Setting the mean to zero (TE = sqrt(E[alpha_t^2]) * sqrt(252)) ensures a fund that consistently lags by a fixed bp still registers non-zero TE. For cost comparison, total cost of ownership decomposes symbolically as

  TCO = TER + trading costs + taxes - securities-lending income,

which is why a fund with the lower TER can be the more expensive holding once the negative legs and lending credit are netted. A measurement caveat: when distributions are omitted from return calculations, price moves around ex-dividend dates masquerade as volatility and inflate the measured TE even though true tracking is unchanged.

**Source:** Weiner (2021) ch.7 appendix pp.120-123; Madhavan (2016) §5.2.1 pp.79-81.

## See Also
- [`fa-market-impact-transaction-costs-and-turbulence-breakdown`](./fa-market-impact-transaction-costs-and-turbulence-breakdown.md) — the trading-cost leg of TCO and of the residual epsilon.
- [`fa-in-kind-basket-design-and-fees`](./fa-in-kind-basket-design-and-fees.md) — fee accrual and securities-lending income that enter the attribution residual.
- [`fa-nav-staleness-and-arbitrage-speed`](./fa-nav-staleness-and-arbitrage-speed.md) — stale/fair-value pricing that distorts measured tracking error.
- [`fa-iiv-iopv-intraday-fair-value`](./fa-iiv-iopv-intraday-fair-value.md) — the fair-value-vs-last-trade pricing choice that can show spurious one-day over/underperformance.
- `pa-active-risk-tracking-error-ex-ante-vs-ex-post` (reading 15) owns tracking error as the volatility of active (excess) return and its ex-ante/ex-post split; here the same annualized statistic is applied to a passive replication target and attributed into misallocation, cash-drag, and a fee/cost residual.
- `pa-transaction-based-attribution-and-trading-cost` (reading 15) is the active-management twin (it absorbs trading cost into a value-add decomposition; this card books it into a passive-replication TCO residual). Note: reading-14's `mt-implementation-shortfall` uses "tracking error" for a dollar opportunity-cost level, NOT the excess-return volatility meant here.

Legacy cross-reference (other tree, prose only): the performance-and-attribution (reading 15) card on tracking error and active risk frames TE as the volatility of active return against a benchmark; here the same statistic is applied to a passive replication target and attributed into misallocation, cash drag, and a fee/cost residual rather than to active factor bets.

## Escalate to Raw When
Go to the raw source when you need the worked numerical attribution — e.g., the SWA ETF example computing the basis-point cash drag from a stated cash position, risk-free rate, and one-day portfolio return, or the quarterly attribution report tallying fees, transaction costs, cash drag, and misallocation into a total under/overperformance figure. Also escalate for the full appendix algebra (the normalization of portfolio weights to 100% and the term-by-term rearrangement into misallocation and cash drag), or for the Madhavan high-yield case study where a lower-TER fund proves more expensive once tracking difference and lending revenue are netted into TCO.

**Source:** Weiner (2021) ch.7 pp.115-123; Madhavan (2016) §5.4 pp.82-83.
