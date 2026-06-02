---
schema_version: "cacg.v0"
id: "deriv-delta-and-hedging"
title: "Delta and Hedging"
reading_id: "07_derivatives_and_volatility"
summary: "Delta hedging maintains a position in the underlying whose delta exactly offsets the option position's delta. A writer of a European call with delta N(d₁) holds N(d₁) shares long; small underlying moves are net-flat, gamma drives re-hedging needs. Under continuous-time BSM with continuous re-hedging the hedged portfolio earns the risk-free rate; discrete re-hedging realizes a gamma-theta tradeoff."
tags: ["derivatives", "delta-hedging"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p430:0638"
    chunk_hash: "5d9576ca0062eea9cb551960b4e5fd01ac17db4952435fec1d83c094d8fdfded"
    page_range: [431, 431]
    quote: "Delta neutrality provides protection against relatively small stock price moves between rebalancing."
    edge_type: "defines"
card_hash: "19d5e0b55764120f012f66f780efc42c73431dd142c1ed449e127d6b36511352"
---
# Delta and Hedging

## Intuition

A short option position is exposed to the underlying's price
moves: when the underlying rises, a short call loses money. The
writer can offset this exposure by holding `Δ` shares of the
underlying long; the long-share P&L cancels the short-option
delta P&L over an instantaneous move. Because gamma is non-
zero, the hedge ratio `Δ` itself moves with the underlying, so
the writer must re-hedge dynamically. Between rebalancings, the
short-option position accumulates a residual gamma exposure
that the BSM PDE pairs with theta decay so that the hedged
position earns the risk-free rate at instantaneous frequency.
**Source:** Hull §19 pp.430-445.

```
delta hedging mechanic

    short 1 call               long Delta shares
   (delta = -N(d_1))           (delta = +N(d_1))
            |                          |
            +-------------+------------+
                          |
                  net delta is zero
                          |
                          v
            small move in S: pnl is negligible
            large move in S: gamma cost
                            (re-hedge needed)
```

## Definition

**Delta hedging** is the practice of holding a position in the
underlying whose delta exactly offsets the delta of the option
position. For a writer of one European call with delta `N(d_1)`,
the delta-neutral hedge is to hold `N(d_1)` shares long. After
an instantaneous underlying move `dS`, the option-side P&L is
approximately `-N(d_1) · dS` (short call) and the share-side
P&L is approximately `N(d_1) · dS`, so the net is approximately
zero. **Source:** Hull §19 pp.430-440.

**Dynamic re-hedging** is the periodic rebalancing of the share
position to maintain delta-neutrality as `Δ` itself drifts with
the underlying. Under continuous-time BSM with continuous
re-hedging the hedged portfolio earns the risk-free rate
exactly; in practice the re-hedging frequency trades off gamma
loss against transaction costs. The standard practitioner
result is that the daily P&L of a delta-hedged short option is
approximately
`P&L = (1/2) · Γ · (dS)^2 · scaling - Θ · dt`,
the "gamma-theta tradeoff": realized variance pays for theta
decay if implied vol equals realized vol. **Source:** Hull §19
pp.440-450.

## Mathematical Reasoning

The replication argument behind BSM is exactly the delta-
hedging argument: a self-financing portfolio long the option
and short `Δ` units of underlying has its `dW` term cancelled
by Itô's lemma applied to the option-price function with
`Δ = ∂V / ∂S`. The remaining deterministic instantaneous
return must equal the risk-free rate by no-arbitrage; this
identity is the BSM PDE. The BSM closed form is the unique
arbitrage-free price consistent with continuous-time
delta-hedging. **Source:** Hull §19 pp.430-445; Hull §15
pp.346-360.

In discrete time the delta-hedge is approximate. The portfolio
P&L over one period `dt` includes a second-order gamma term:
expansion of the option-price function around the current spot
gives
`dV = Δ · dS + (1/2) · Γ · (dS)^2 + Θ · dt + higher-order`,
and after offsetting `Δ · dS` with the share hedge the residual
is `(1/2) · Γ · (dS)^2 + Θ · dt`. Under BSM with `dS / S ≈ σ ·
sqrt(dt) · ε` for `ε ~ N(0, 1)`, the expected gamma term is
`(1/2) · Γ · S^2 · σ^2 · dt`, which by the BSM PDE equals
`-Θ · dt + r · (V - Δ · S) · dt`. The path-dependent variation
around this expectation is the "gamma P&L" that determines the
hedge's profitability when realized vol differs from implied
vol. **Source:** Hull §19 pp.440-455.

The transaction-cost boundary follows from the dilemma between
re-hedging often (low gamma loss but high transaction cost) and
re-hedging seldom (high gamma exposure but low transaction
cost). Practitioner approaches include rebalance-when-delta-
moves-by-X-thresholds, time-based grid rebalancing, and stop-
loss / take-profit triggers; each leaves residual hedging error
that compounds across the option's life and that this card
treats only at the order-of-magnitude level. **Source:** Hull
§19 pp.445-455.

## See Also

- [`deriv-greeks-overview.md`](deriv-greeks-overview.md) — taxonomy of all Greeks; delta and gamma are the two used in dynamic hedging
- [`deriv-vega-and-theta.md`](deriv-vega-and-theta.md) — the gamma-theta-vega relationship that quantifies the hedge's daily P&L
- [`deriv-bsm-formula.md`](deriv-bsm-formula.md) — closed-form delta `N(d_1)` and gamma `N'(d_1) / (S · σ · sqrt(T))`

## Escalate to Raw When

Open Hull chapter 19 directly when any of the criteria below
applies. **Source:** Hull §19 pp.430-455.

- Multi-asset hedging (basket options, correlation hedging,
  best-of / worst-of payoffs) is needed; those require Greek
  decomposition across multiple risk factors. **Source:** Hull
  §27 pp.626-660.
- Portfolio-level vega and gamma management at the desk level
  is needed; that introduces volatility-sigma carry, smile-
  consistent re-hedging, and the sticky-strike vs sticky-delta
  conventions. **Source:** Hull §20 pp.460-485.
- Stochastic-vol or jump-aware hedging is needed; the BSM
  delta-only argument breaks down and the writer must hedge
  vega and jump risk separately. **Source:** Hull §27
  pp.626-660.
