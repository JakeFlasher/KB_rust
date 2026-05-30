---
schema_version: "cacg.v0"
id: "mt-liquidity-measures-spread-depth-resiliency"
title: "Measuring Liquidity: Quoted, Effective, and Realized Spread; Depth; Resiliency"
reading_id: "14_microstructure_and_trading"
summary: "Liquidity has three observable dimensions — spread (small round-trip cost), depth (size tradable at a quote), and resiliency (speed of price reversion) — and quoted, effective, and realized spreads each measure a different slice of the round-trip cost."
tags: ["microstructure", "liquidity", "bid-ask-spread", "effective-spread", "realized-spread", "market-depth"]
citations:
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p052:0072"
    chunk_hash: "e681dc23b0690279651a32a88443f95ab78c8a496c263b2db878959cabf43993"
    page_range: [52, 53]
    quote: "the price at which a market order executes and the midquote on the market the instant"
    edge_type: "defines"
---
# Measuring Liquidity: Quoted, Effective, and Realized Spread; Depth; Resiliency

## Intuition

Liquidity is not a single number; it is how cheaply, in what size, and how
durably you can convert a position into cash without moving the price against
yourself. Foucault, Pagano, and Röell frame it the way an engineer rates a car:
several distinct performance axes, not one. Three observable axes recur. The
**spread** answers "what does a tiny round-trip cost?" — buy at the ask, sell
instantly at the bid, and the gap is what you forfeit. **Depth** answers "how
big a trade can I do at that quote?" — a market can show a tight top-of-book
spread yet have only a few shares behind it, so a larger order walks up the
book and pays more. **Resiliency** answers "how fast does the quote heal after
I hit it?" — after a large order depletes one side, liquidity suppliers
eventually repost, and the speed of that replenishment is its own dimension.

The same trade can look cheap on one axis and expensive on another. A
high-frequency-rebalancing fund cares most about spread; a pension liquidating
a block cares most about depth and resiliency. That is why one liquidity
statistic never settles the question.

```
   price
     ^
 ask |======  <- top-of-book ask (quoted spread = ask - bid)
     |  ||         depth = size resting at/near these quotes
 mid |- - - <- midquote m = (a+b)/2  (the "fair" benchmark)
     |  ||
 bid |======  <- top-of-book bid
     +----------------------------> size walked into the book
            (deeper book => spread s(q) rises only slowly with size q)

   after a large buy depletes the ask:   resiliency = how fast the ask reposts
   t0: ask gapped up        t0+Δ: ask back near old level (high resiliency)
```

**Source:** Foucault, Pagano & Röell (2013) ch.2 §2.1–2.2 pp.47–54

## Definition

Let `a` be the best ask, `b` the best bid, and the **midquote** `m = (a+b)/2`
the proxy for a perfectly-liquid price.

- **Quoted (relative) spread:** `s ≡ (a − b)/m`. The cost of a round-trip small
  enough to fill entirely at the Best Bid and Offer (BBO). For larger size `q`,
  use the weighted-average quoted spread `s(q) ≡ (ā(q) − b̄(q))/m`, where `ā(q)`
  and `b̄(q)` are the average fill prices for buy and sell market orders of size
  `q`; `s(q)` reduces to `s` as `q → 0`.
- **Effective half-spread:** `S_e ≡ d·(p − m)`, with order-direction indicator
  `d = +1` for buyer-initiated and `d = −1` for seller-initiated trades, `p` the
  execution price, and `m` the midquote *just before* the trade. It is measured
  from actual fills, so it captures price improvement and hidden liquidity.
- **Realized half-spread:** `S_r = d_t·(p_t − m_{t+Δ})`, the liquidity
  supplier's profit if the position is unwound at the post-trade midquote
  `m_{t+Δ}`, with `Δ` chosen long enough for quotes to absorb the trade's price
  impact.
- **Depth:** the size executable at (or near) a given quote; the deeper the
  book, the milder the rise of `s(q)` with `q`.
- **Resiliency:** the speed at which liquidity returns to normal after a trade.

**Source:** Foucault, Pagano & Röell (2013) ch.2 §2.2.1–2.2.3, §2.4 pp.50–54, p.68–69

## Mathematical Reasoning

Substituting the effective half-spread `S_e = d_t(p_t − m_t)` into the realized
half-spread gives the central decomposition:

```
S_r = d_t (p_t − m_{t+Δ})
    = d_t (p_t − m_t)  −  d_t (m_{t+Δ} − m_t)
    = (effective half-spread)  −  (post-trade midquote drift in trade direction)
```

Taking expectations,

```
E(S_r) = E(S_e) − E( d_t (m_{t+Δ} − m_t) ).
```

The term `E( d_t (m_{t+Δ} − m_t) )` is the **price-impact / adverse-selection**
component: if order direction is positively correlated with the subsequent
midquote move (buys tend to be followed by higher midquotes), this term is
positive, so `E(S_r) < E(S_e)`. The liquidity supplier's *realized* gain is
strictly less than the *effective* spread the demander paid, because the
supplier is left holding inventory whose fair value has moved against them.

This yields a no-free-lunch lower bound on the spread: if the effective spread
were small enough that `E(S_e) < E( d_t(m_{t+Δ} − m_t) )`, suppliers would lose
money on average and exit. Hence the effective spread cannot fall below the
expected adverse price move — it must at least compensate suppliers for being
picked off. So the ordering between realized and effective spread is an
*expectation-level* result that holds only under the stated condition
`E(d_t(m_{t+Δ} − m_t)) > 0` — it is not an unconditional, trade-by-trade
inequality:

```
E(realized spread)  <  E(effective spread)   [iff E(d_t(m_{t+Δ}−m_t)) > 0]
        ^smaller in expectation, net of impact

  effective spread   (≈ or vs)   quoted spread
        ^net of impact           ^hypothetical, pre-improvement
```

Effective spread can sit *below* the quoted spread when fills receive price
improvement (hidden orders, dealer improvement), and *above* it when size
exhausts the BBO and walks the book.

**Source:** Foucault, Pagano & Röell (2013) ch.2 §2.2.2–2.2.3 eqs.(2.3)–(2.6) pp.51–54

## Boundary Notes

- The quoted spread requires live quote/limit-order-book data; for size beyond
  the BBO it needs depth at price points past the top of book, which is not
  always available — hence the fallback to transaction-based measures.
- The effective spread is a *retrospective* measure (built from past fills),
  whereas the quoted spread is *prospective* (prices traders can act on now).
- Computing the effective spread needs a signed direction `d`; when datasets do
  not record initiation, trades are signed by classification rules (e.g.,
  Lee-Ready), which misclassify midpoint trades, small trades, and very active
  large-cap names, injecting noise into the estimate.
- The realized spread is sensitive to the choice of horizon `Δ`: too short and
  quotes have not yet absorbed the impact; the appropriate `Δ` depends on how
  fast the market re-quotes (small in transparent, active markets).
- Order splitting breaks the link between recorded transactions and parent
  orders, so a non-participant analyst may only be able to measure effective
  spreads for small, identifiable trades.
- Resiliency is a dynamic axis the static spread/depth snapshots miss: in highly
  resilient markets a broker can accelerate a large order and cut both execution
  and opportunity cost; in low-resiliency markets they must trade slowly.

**Source:** Foucault, Pagano & Röell (2013) ch.2 §2.2.2–2.2.3, §2.4, box 2.1 pp.51–54, p.68–69

## See Also

- [`mt-liquidity-depth-immediacy-width`](./mt-liquidity-depth-immediacy-width.md) -- decomposes the depth/width/immediacy axes this card summarizes
- [`mt-effective-cost-trade-benchmark`](./mt-effective-cost-trade-benchmark.md) -- effective spread as a per-trade execution-quality benchmark
- [`mt-roll-implicit-spread-estimator`](./mt-roll-implicit-spread-estimator.md) -- recovering the implied spread from transaction-price serial covariance when quotes are unavailable
- [`mt-price-impact-measures-amihud`](./mt-price-impact-measures-amihud.md) -- the adverse-selection / price-impact term that drives realized below effective spread
- [`fa-market-liquidity-dimensions-and-no-arbitrage`](../22_fund_level_arbitrage/fa-market-liquidity-dimensions-and-no-arbitrage.md) — cross-set: the spread / depth / resiliency liquidity dimensions (reading-14 primary; reading-22 no-arbitrage framing).
## Escalate to Raw When

The card sketches the realized-vs-effective decomposition and the supplier
break-even bound but does not reproduce the full table comparing realized and
effective spreads across trade sizes (Microsoft example) nor the formal `s(q)`
weighted-average construction beyond stating its limit behavior. For the exact
Lee-Ready classification algorithm and its measured error rates, the precise
choice of `Δ`, and the implementation-shortfall treatment that folds in
opportunity cost, re-read Foucault, Pagano & Röell (2013) ch.2 §2.2–2.4
pp.50–69 (box 2.1 for trade signing; §2.4 for resiliency and shortfall).
