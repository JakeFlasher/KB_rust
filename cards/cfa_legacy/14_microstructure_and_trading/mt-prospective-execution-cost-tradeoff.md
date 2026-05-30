---
schema_version: "cacg.v0"
id: "mt-prospective-execution-cost-tradeoff"
title: "Prospective Execution: The Market-Order vs Limit-Order Cost/Opportunity Trade-off"
reading_id: "14_microstructure_and_trading"
summary: "A forward-looking trader chooses between a market order (certain fill, positive execution cost, zero opportunity cost) and a limit order (possibly negative execution cost but volatile opportunity cost if the order goes unfilled)."
tags: ["microstructure", "execution-cost", "limit-order", "market-order", "opportunity-cost", "implementation-shortfall"]
citations:
  - source_id: "mt_hasbrouck_2007_empirical_market_microstructure"
    chunk_id: "mt_hasbrouck_2007_empirical_market_microstructure:p156:0201"
    chunk_hash: "1382580b0feac0877de9bf267d2034feba53f773fd14d3f51041035b56e10fa1"
    page_range: [156, 157]
    quote: "This will give a positive execution cost, but zero opportunity cost."
    edge_type: "defines"
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p065:0083"
    chunk_hash: "2634d28c6463fc4f918ddbfe3793134af283c138470aa5d84f5a6b58af91b9c1"
    page_range: [66, 66]
    quote: "trade-off we face is immediacy versus cost of execution"
    edge_type: "supports"
---
# Prospective Execution: The Market-Order vs Limit-Order Cost/Opportunity Trade-off

## Intuition
When a trader looks *forward* at how to fill a desired position, the central question is not merely "what did this trade cost?" (retrospective) but "what mix of order types should I use, given that fills are uncertain?" The implementation shortfall framework splits the realized loss-versus-benchmark into two pieces — an **execution cost** (paid on shares actually filled, at prices away from the benchmark midpoint) and an **opportunity cost** (the benchmark-price drift on shares left *unfilled*). Prospectively, the trader is choosing the joint distribution of these two costs.

The cleanest way to see the trade-off is to take the pre-trade quote midpoint as the benchmark and ask how a buy order behaves under two pure strategies:

```
                 EXECUTION COST        OPPORTUNITY COST
                 (filled shares)       (unfilled shares)
  ----------------------------------------------------------
  MARKET ORDER   positive, certain     zero
  (cross spread) (pay ~ half-spread)   (you always get filled)
  ----------------------------------------------------------
  LIMIT ORDER    if hit: NEGATIVE      if NOT hit: positive &
  (post inside)  (buy below mid)        HIGHLY VOLATILE (price ran)
  ----------------------------------------------------------
```

A market order guarantees the fill but pays the spread; you keep no opportunity cost because nothing is left undone. A passive limit order can actually *earn* the spread (a negative execution cost) — but the very price moves that prevent it from filling are precisely the moves that make the opportunity cost large and unpredictable. Cartea, Jaimungal & Penalva frame the same choice as "immediacy versus cost of execution": the fastest fill crosses the spread, the cheapest fill waits and risks non-execution.

**Source:** Hasbrouck (2007) §14.2 *The Implementation Shortfall* pp.156-157; Cartea, Jaimungal & Penalva (2015) ch.3 (Empirical and Statistical Evidence) p.66.

## Definition
Let `v` be the desired (target) post-trade holding, `n0` the initial holding, `n1` the realized holding, `p` the share-weighted execution price, and `π0`, `π1` benchmark prices before and after. The implementation shortfall decomposes into:

- **Execution cost** `(n1 − n0)(p − π0)` — cost of *actual* executions accomplished at trade prices rather than the benchmark. Negative if the stock is bought below the benchmark.
- **Opportunity cost** `(v − n1)(π1 − π0)` — driven by the divergence between actual (`n1`) and desired (`v`) holdings multiplied by the benchmark-price change. Negative if an intended purchase was *not* completed for a stock that subsequently declined.

A **market order** is the order type that "will achieve execution with certainty, but at a price generally above the midpoint" (for a buy). A **limit order** posts a price away from the midpoint and fills only if the market trades through it.

**Source:** Hasbrouck (2007) §14.1-14.2 (eq. 14.1, implementation-shortfall decomposition) pp.144-157.

## Mathematical Reasoning
Take the quote midpoint as benchmark `π0` and consider a buy.

1. **Market order.** Fill is certain, so `v − n1 = 0` ⇒ opportunity cost `= 0`. Execution price `p ≥ π0` (you lift the ask), so execution cost `(n1 − n0)(p − π0) ≥ 0`. Its *variance* is low: a marketable buy executes at or very near the ask, so `p − π0 ≈` half-spread with little dispersion.

2. **Limit order** posted below `π0` at price `ℓ < π0`.
   - If hit: execution cost `(n1 − n0)(ℓ − π0) < 0` (negative — you bought below the midpoint).
   - If not hit: `n1 = n0`, so opportunity cost `= (v − n0)(π1 − π0)`. Conditional on *non-fill of a buy*, the most likely reason is `π1 ≫ π0` (price rose away from the limit), making this term large and **positive**. Because `π1 − π0` is the price change over a possibly long horizon, its variance is high.

This yields the two comparative-static statements Hasbrouck draws:
- **Expectation vs. expectation.** A market order has high *expected* trading cost; a far-from-market limit order has low (even negative) *expected* execution cost.
- **Expectation vs. volatility.** A market order has low variance of trading cost; an aggressive limit order has low expected shortfall but high volatility, because the unfilled-portion opportunity cost inherits the variance of `π1 − π0`.

The bound `(v − n1)(π1 − π0)` is approximately the *worst* case for opportunity cost: at the new higher price a manager might rationally seek fewer shares, so realized opportunity cost can be below the formula. It is exactly this expectation-versus-volatility trade-off that "drives some of the more formal trading models considered in the next chapter," i.e., dynamic optimal-execution schedules.

**Source:** Hasbrouck (2007) §14.2 pp.156-157 (worst-case bound, low-variance execution cost, high-variance opportunity cost).

## Boundary Notes
- **Benchmark dependence.** The whole decomposition is defined relative to a chosen `π0`. The pre-trade bid-ask midpoint (the "effective cost" benchmark) is one convention; using a five-minutes-after midpoint gives the "realized cost." Sign and magnitude of both components shift with the benchmark.
- **Aggregation breaks down.** Over both sides of a trade, *execution* costs are a zero-sum game when everyone uses the same `π0`; *opportunity* costs are not — they are measured against desired positions, and there is no mechanism ensuring unconsummated demand equals unconsummated supply. Hasbrouck's 100-trader / one-dealer example shows aggregate opportunity cost can be absurdly inflated when `π0` no longer clears the market.
- **Worst-case caveat.** `(v − n1)(π1 − π0)` overstates true opportunity cost when the manager would revise `v` downward after an adverse price move.
- **Liquidity-supplier reframe.** When `v = n0` (agent already at desired position and supplying liquidity), the execution/opportunity split is "somewhat strained" — risk-neutral liquidity suppliers are indifferent to fill and incur no loss on non-execution.
- **Where it holds.** The clean MO-zero-opportunity-cost result assumes the market order fully fills; partial fills reintroduce opportunity cost on the residual.

**Source:** Hasbrouck (2007) §14.2-14.2.2 pp.156-159; Cartea, Jaimungal & Penalva (2015) ch.3 (Empirical and Statistical Evidence) p.66 (immediacy vs cost framing; latency reintroduces fill uncertainty even for marketable orders).

## See Also
- [`mt-implementation-shortfall`](./mt-implementation-shortfall.md) -- the parent decomposition into execution + opportunity cost that this card chooses between prospectively.
- [`mt-almgren-chriss-optimal-execution`](./mt-almgren-chriss-optimal-execution.md) -- the "more formal trading models" that resolve the expectation-vs-volatility trade-off via an optimal schedule.
- [`mt-limit-order-book-equilibrium`](./mt-limit-order-book-equilibrium.md) -- why posted limit prices sit near the midpoint and what fill probability the unfilled-order opportunity cost depends on.

## Escalate to Raw When
Hasbrouck §14 derives the implementation-shortfall identity (eq. 14.1) and proves the zero-sum property of execution costs under a common benchmark; this card only sketches those. Re-read pp.144-159 for the formal portfolio-level definition, the effective-vs-realized cost (Rule 605 / "dash five") benchmark machinery, and the liquidity-supplier (`v = n0`) reinterpretation. For the dynamic resolution of the expectation/volatility trade-off, escalate to the optimal-execution chapter (Hasbrouck ch.15) and Cartea, Jaimungal & Penalva's "Optimal Execution with Limit and Market Orders" chapter.
