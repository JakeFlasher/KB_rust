---
schema_version: "cacg.v0"
id: "mt-microprice-midprice-spread"
title: "Midprice, Microprice, Quoted Spread and Market-Quality Measures"
reading_id: "14_microstructure_and_trading"
summary: "The quoted spread is ask minus bid; the midprice averages the two as a frictionless-price proxy, while the volume-weighted microprice leans toward the lighter side of the book — heavy bid depth pulls it up toward the ask — signaling directional order-flow pressure."
tags: ["microstructure", "limit-order-book", "spread", "microprice", "liquidity", "order-imbalance"]
citations:
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p063:0080"
    chunk_hash: "0a906b59e3d16c45f0e88a27d4b22c5f01cf7e6051b1e8a04eb1839550711fcd"
    page_range: [63, 63]
    quote: "The microprice is similar to the midprice, but it incorporates information on order imbalance"
    edge_type: "defines"
card_hash: "cbf75f9a104c49d1ed21d0499ccff48621d420951122c198ff7570ffef0d5e7f"
---
# Midprice, Microprice, Quoted Spread and Market-Quality Measures

## Intuition
A limit order book (LOB) does not display a single price; it displays a best bid and a best ask separated by a gap. Three summary statistics compress that gap into usable signals. The **quoted spread** is the width of the gap (ask minus bid) — the round-trip cost a small market order pays for immediacy. The **midprice** is the arithmetic center of the gap, used as the cleanest proxy for the asset's "true" frictionless value: the price that would prevail if there were no spread to cross. The **microprice** refines the midprice by tilting the center toward whichever side of the book is *lighter* in posted volume, because that is the side the price is more likely to move toward.

```
        bid (P^b)        midprice          ask (P^a)
  --------|------------------|------------------|--------
          |<----- 1/2 spread ---->|<-- 1/2 spread -->|
          |
  microprice sits HERE when ask-side volume is thin
  (heavy bids, light asks => buying pressure => lean up)
          |          ^microprice
```

The intuition for the lean: if there is a large posted volume on the bid and only a thin offer on the ask, the next executions are more likely to clear the thin ask and walk the price up, so the volume-weighted center moves toward the ask. The microprice thus encodes the same order-imbalance information traders watch to anticipate the next tick.

**Source:** Cartea, Jaimungal & Penalva (2015) §1.4 pp.16-18.

## Definition
Let `P^b_t` and `P^a_t` be the best bid and best ask at time `t`, and let `V^b_t` and `V^a_t` be the volumes (share quantities) posted at the best bid and best ask.

- **Quoted spread:** `QuotedSpread_t = P^a_t − P^b_t`. The minimum positive value is one tick; when bid equals ask the market is *locked* (zero spread), an unstable transient state.
- **Midprice:** `Midprice_t = ½(P^a_t + P^b_t)` — the arithmetic average of bid and ask, used to proxy the transaction-cost-free price.
- **Microprice** (also called the weighted-midprice): the average of bid and ask weighted by the *opposite-side* relative volumes,

```
              V^b_t                 V^a_t
microprice = -------------- P^a_t + -------------- P^b_t
             V^b_t + V^a_t          V^b_t + V^a_t
```

so that the heavier bid volume puts more weight on the ask price, pulling the microprice up.

**Source:** Cartea, Jaimungal & Penalva (2015) §1.4 pp.17-18, §3.2 p.63.

## Mathematical Reasoning
Write the bid-side volume share as `w = V^b/(V^b+V^a) ∈ [0,1]`. Then

```
microprice = w·P^a + (1−w)·P^b = P^b + w·(P^a − P^b) = P^b + w·QuotedSpread.
```

This makes the comparative statics transparent:

- When the book is *balanced* (`V^b = V^a`, so `w = ½`) the microprice collapses to the midprice `P^b + ½·spread`. The midprice is therefore the equal-weight special case of the microprice.
- The microprice is bounded inside the spread: `P^b ≤ microprice ≤ P^a`, since `w ∈ [0,1]`. It never quotes outside the BBO.
- `∂(microprice)/∂w = QuotedSpread > 0`: as bid-side volume share rises (more posted buyers relative to sellers), the microprice rises toward the ask. Heavy bids / thin asks ⇒ `w → 1` ⇒ microprice `→ P^a` (upward pressure); the symmetric case pushes it toward `P^b`.
- The lean is scaled by the spread: in a tight one-tick market the imbalance can only move the microprice a fraction of a tick, whereas in a wide-spread illiquid name the same imbalance translates into a large absolute displacement.

Market quality is then read jointly from these statistics plus **depth** (posted volume at and behind the BBO): a liquid market shows a narrow quoted spread, large depth at many price levels, and a microprice that hugs the midprice; an illiquid market shows a wide spread, thin and gappy depth, and a microprice that swings.

**Source:** Cartea, Jaimungal & Penalva (2015) §1.4 pp.17-18.

## Boundary Notes
- The midprice is only a *proxy* for the frictionless value; it ignores volume entirely, so it can sit at the arithmetic center of a badly imbalanced book that is about to move. The microprice corrects exactly this blind spot by incorporating order-imbalance information.
- All three measures use only the *best* (level-1) bid and ask volumes; deeper-book imbalance is not captured by the standard microprice formula above.
- The quoted spread is the cost of immediacy for *small* market orders only. Larger orders walk the book and pay more than the quoted spread; the relevant frictions then are depth and price impact, not the level-1 spread. Harris frames the spread as "the price impatient traders pay for immediacy," with buyers lifting the offer and sellers hitting the bid.
- A locked (zero-spread) or crossed book breaks the clean `P^b ≤ microprice ≤ P^a` ordering and is treated as a transient anomaly, not a steady state.

**Source:** Harris (2003) ch.14 p.297 (bid/ask spread as the price impatient traders pay for immediacy; buyers buy at the ask, sellers sell at the bid); Cartea, Jaimungal & Penalva (2015) §1.4 p.17.

## See Also
- [`mt-limit-order-book-mechanics`](./mt-limit-order-book-mechanics.md) -- supplies the bid/ask/depth primitives these statistics summarize.
- [`mt-order-imbalance-signal`](./mt-order-imbalance-signal.md) -- the volume-imbalance signal that drives the microprice lean.
- [`mt-liquidity-measures-spread-depth-resiliency`](./mt-liquidity-measures-spread-depth-resiliency.md) -- extends spread/depth into the full liquidity taxonomy.
- [`mt-market-quality-volatility-origins`](./mt-market-quality-volatility-origins.md) -- connects these measures to market-quality and volatility.

## Escalate to Raw When
The source only asserts (does not derive) that the microprice "indicates the buy (sell) pressure in the market" and defers the formal exploration of how relative bid/ask volumes drive price moves to Chapter 12 (volume-imbalance trading strategies); re-read Cartea, Jaimungal & Penalva (2015) §3.2 (microprice as weighted-midprice, empirical AAPL ITCH study, autocorrelation/mean-reversion) and Chapter 12 for the modelled dynamics rather than relying on this card's static comparative statics.
