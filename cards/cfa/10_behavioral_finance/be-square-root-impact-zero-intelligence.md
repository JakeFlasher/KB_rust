---
schema_version: "cacg.v0"
id: "be-square-root-impact-zero-intelligence"
title: "Square-Root Market Impact and Zero-Intelligence Order Flow"
reading_id: "10_behavioral_finance"
summary: "Market microstructure from order flow: the universal empirical square-root law says a metaorder's price impact grows as the square root of its volume, and minimal zero-intelligence limit-order-book models (random Poisson order flows) are the agent-based test bed used to rationalize it."
tags: ["behavioral-finance", "market-microstructure", "square-root-impact", "zero-intelligence", "limit-order-book"]
citations:
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p419:0616"
    chunk_hash: "6ef2f14d688fb60ce7cddd89b693fa16ab783965804a1957e91f7abcb0fb9082"
    page_range: [420, 420]
    quote: "there is now overwhelming empirical evidence ruling out the simple linear impact law, and suggesting instead a concave, square-root-like growth of impact with volume, often dubbed the"
    edge_type: "defines"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p423:0623"
    chunk_hash: "be5011c5a6aaf5697993a29f682e4780ba95ef24808c74a9c41873c06028cf8e"
    page_range: [424, 424]
    quote: "model, order flows are completely random and assumed to be governed by the following stochastic"
    edge_type: "defines"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p420:0617"
    chunk_hash: "1bbd5075653de38b09976960373fbde5637104b17e6b5c97f15492e36fa6be23"
    page_range: [420, 420]
    quote: "This square-root impact law is extremely well established empirically but extremely surprising"
    edge_type: "supports"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p423:0623"
    chunk_hash: "be5011c5a6aaf5697993a29f682e4780ba95ef24808c74a9c41873c06028cf8e"
    page_range: [424, 424]
    quote: "m(t)), buy (resp. sell) limit orders arrive as a Poisson process with rate"
    edge_type: "supports"
card_hash: "3ab38871d53feb28f314a69058ebf6410a38ad23e0b1545cfc9e3c0f00cb0f48"
---
# Square-Root Market Impact and Zero-Intelligence Order Flow

## Intuition

How much does trading a quantity move the price? The naive guess, and the prediction of the classic Kyle (1985) model, is that impact is LINEAR in volume. Two decades of empirical work overturn this: the price impact of a **metaorder** (the full sequence of trades from a single investment decision) grows instead as the SQUARE ROOT of its volume. This square-root impact law is strikingly universal — it holds across equities, futures, FX, options, and Bitcoin, across pre- and post-HFT epochs, and across market participants and execution styles — which earns it the status of a near-physical "law." Its concavity means the second half of a large order moves the price much less than the first half. **Source:** Bouchaud (2018) §3 pp.401-402.

To explain a law this universal, one wants minimal, robust ingredients, which is where **zero-intelligence** agent-based models enter. These strip away all strategic behavior: orders arrive at random (Poisson) rates and the limit order book evolves mechanically. The Santa-Fe zero-intelligence model treats limit orders as particles deposited on a price lattice, market orders as annihilations at the best quote, and cancellations as evaporations. The exercise reveals both what such minimalism can capture (spread, book shape) and what it cannot (zero-intelligence flow generically FAILS to reproduce the basic random-walk property of prices). **Source:** Bouchaud (2018) §1, §4 pp.395-406.

The deeper resolution invokes a "latent" order book: most liquidity is hidden and only reveals itself as the price approaches it, giving a V-shaped latent liquidity profile that grows linearly away from the current price. Resistance to further moves rises with executed volume, which yields the concave square-root impact. The square-root law and the impossibility of frictionless arbitrage are thus two faces of the same liquidity constraint. **Source:** Bouchaud (2018) §1, §3 pp.395-401.

## Definition

**Metaorder** is the sequence of child trades originating from a single investment decision; its total volume `Q` is the natural unit for measuring price impact, since the "market price" is meaningful only for infinitesimal volumes. **Source:** Bouchaud (2018) §3.1 pp.401-401.

**Square-root impact law** is the empirical relation that a metaorder's impact scales as `I(Q,T) ~ Y sigma_T (Q/V_T)^delta` with exponent `delta` in roughly 0.4-0.7 (vs. Kyle's `delta = 1`), `sigma_T` the contemporaneous volatility, `V_T` the average traded volume, and `Y ~ 0.5` a numerical coefficient of order unity. **Source:** Bouchaud (2018) §3.2 pp.402-402.

**Zero-intelligence (Santa-Fe) model** is a limit-order-book model in which buy/sell limit orders, market orders, and cancellations all arrive as independent Poisson processes with no strategic optimization, used as a minimal test bed for emergent microstructure. **Source:** Bouchaud (2018) §4 pp.406-406.

## Mathematical Reasoning

The square-root law is `I(Q,T) ~ Y sigma_T (Q/V_T)^delta` for `Q << V_T`, dimensionally consistent because `Q` and `V_T` cancel while impact and volatility are both expressed as price percentages. Kyle's linear model is the `delta = 1` special case with constant "lambda" `= Y sigma_T / V_T`. Concavity (`delta < 1`) implies the second `Q/2` of a metaorder has impact only ~40% of the first `Q/2`; for `delta = 1/2` exactly, `sqrt(2) - 1 ~ 0.414` times less. This requires a market memory longer than the execution time and liquidity that REPLENISHES (resistance increases) as the order proceeds. **Source:** Bouchaud (2018) §3.2-3.3 pp.402-403.

In the zero-intelligence model, orders are unit-size particles on a tick lattice. The leftmost sell particle defines the ask `a(t)`, the rightmost buy the bid `b(t)`, mid-price `m(t) = (a(t)+b(t))/2`, spread `s(t) = a(t)-b(t)`. The Poisson dynamics:

```
   limit orders : rate lambda per price level (buy below m, sell above m)
   market orders: rate mu  (annihilate the best opposite quote)
   cancellations: rate nu  (each resting order evaporates)
   all event types mutually independent
```

The stationary queue-size distribution far from `m(t)` is Poisson, `P_st(V) = e^{-V*} (V*)^V / V!` with `V* = lambda/nu`. The equilibrium spread is approximately `s_eq ~ vartheta[1 + 2(mu + nu)/lambda]`, so a larger market-order flux `mu` widens the spread. Only the RATIOS of `lambda, mu, nu` matter (they carry units of inverse time). **Source:** Bouchaud (2018) §4.1-4.2 pp.405-408.

Simulating the model reproduces the mean spread well but predicts significant MEAN REVERSION in prices (except when the cancellation memory `T_m = 1/nu` is very short), too little volatility for large-tick and too much for small-tick stocks, and spurious arbitrage opportunities for market-making — weaknesses that point to the missing ingredients (long-range order-flow correlation, latent liquidity) needed to recover the random walk and the square-root law. **Source:** Bouchaud (2018) §4.3 pp.408-411.

## See Also

- [be-limits-of-arbitrage](./be-limits-of-arbitrage.md#intuition) — the liquidity/impact constraints that make frictionless arbitrage impossible, of which square-root impact is the microstructural face.
- [be-stylized-facts-financial-markets](./be-stylized-facts-financial-markets.md#intuition) — the random-walk-with-long-memory facts that zero-intelligence flow generically fails to reproduce.
- [be-fundamentalist-chartist-ham](./be-fundamentalist-chartist-ham.md#intuition) — the mesoscale HAMs that complement these microscale order-flow models.

## Escalate to Raw When

- The latent-order-book derivation of the square-root law and the V-shaped liquidity profile (Sections 5-6) require the source's full argument. **Source:** Bouchaud (2018) §1, §6 pp.395-426.
- Precise empirical exponents `delta`, the coefficient `Y`, or the gap-to-spread and signature-plot simulation results must be quoted from the data. **Source:** Bouchaud (2018) §3.2, §4.3 pp.402-411.
- The full Santa-Fe stochastic-process specification, queue-size approximations, and market-making profit analysis are needed beyond the schematic. **Source:** Bouchaud (2018) §4 pp.405-411.
