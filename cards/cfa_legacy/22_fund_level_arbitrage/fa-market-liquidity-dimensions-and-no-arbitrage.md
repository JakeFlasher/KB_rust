---
schema_version: "cacg.v0"
id: "fa-market-liquidity-dimensions-and-no-arbitrage"
title: "Market-Liquidity Dimensions (Tightness, Depth, Resiliency) & the No-Arbitrage Baseline"
reading_id: "22_fund_level_arbitrage"
summary: "Market microstructure identifies three core liquidity facets — tightness, depth, resiliency (with immediacy a related condition from Black's definition). The no-arbitrage / law-of-one-price / EMH baseline that pins price to fundamental value rests on two hidden assumptions: unlimited arbitrage capital and no risk. Relax either and mispricing persists."
tags: ["tightness-depth-resiliency", "no-arbitrage", "law-of-one-price"]
citations:
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p026:0020"
    chunk_hash: "502c67e0e528c16bc0329e191dc0fe797a7ccff27590422d5750e2d515cad07a"
    page_range: [26, 26]
    quote: "the theory rests on the critical assumptions of"
    edge_type: "defines"
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p024:0017"
    chunk_hash: "450bbccdf99bc480c528940c7f34434b4d754d522cec17c51f2f1fe682eb76e7"
    page_range: [24, 24]
    quote: "such as the cost of changing positions (tightness), the trade size or thickness of the order book-profile (order book refers to a data set which provides traders with bid-ask prices and volume offered per price) required for changing prices (market depth), and the required period of time to recover from price fluctuation caused by a sudden shock or to reach a new equilibrium (market resiliency)."
    edge_type: "supports"
---
# Market-Liquidity Dimensions (Tightness, Depth, Resiliency) & the No-Arbitrage Baseline

## Intuition
A market is "liquid" if a large volume of trades can be executed immediately with minimum effect on price. But liquidity is not one number — market microstructure identifies three core facets: how cheaply you can flip a position (tightness), how much size the order book can absorb before the price moves (depth), and how fast the price snaps back after a shock (resiliency). A closely related condition, *immediacy* — whether a counterparty is available at all right now — appears separately in the book (Demsetz's "need for immediacy," and as one leg of Black's broader four-condition definition of a liquid market), not as a fourth member of the tightness/depth/resiliency triad. The eccentric-billionaire-with-the-purple-convertible thought experiment shows that even when both parties agree on fundamental value, if they will only trade on different days the transaction price is simply not well defined.

Layered on top of this is the textbook no-arbitrage world: if traders could costlessly buy cheap and sell dear, mispricing would vanish instantly and price would equal fundamental value. The hinge of this whole card is that that frictionless picture quietly assumes two things — that arbitrageurs have unlimited capital and bear no risk. Knock out either leg and the convergence machine stalls, so mispricing can persist.

```
        IDEAL (no-arbitrage baseline)            REAL (assumptions relaxed)
   +------------------------------+        +------------------------------+
   |  Law of one price  +  EMH    |        |  capital is LIMITED          |
   |  price == fundamental value  |        |  arbitrage is RISKY          |
   +--------------+---------------+        +--------------+---------------+
   hidden assumptions:                                     |
     (1) unlimited arbitrage capital                       v
     (2) no risk                              mispricing PERSISTS (gap > 0)
                  |                                         |
                  v                                         v
        |P - fundamental| -> 0                  |P - fundamental| stays > 0
   core microstructure facets:  tightness | depth | resiliency   (+ immediacy: a counterparty now?)
```
**Source:** van der Merwe (2015) pp.23-26.

## Definition
- **Tightness** — the cost of changing positions (the bid-ask spread component you pay to flip).
- **Market depth** — the trade size / thickness of the order-book profile required to change prices; sizing the potential trade matters.
- **Market resiliency** — the time required to recover from a price fluctuation caused by a sudden shock, i.e. to reach a new equilibrium.
- **Immediacy** (a related concept van der Merwe draws from Demsetz and from Black's four-condition definition of a liquid market — *not* part of the tightness/depth/resiliency microstructure triad) — the ability to execute now; absent a contemporaneous counterparty the transaction price is undefined even when fundamental value is agreed.
- **No-arbitrage principle** — a necessary equilibrium condition: the simultaneous purchase and sale of the same or essentially similar securities in two markets for advantageously different prices should be competed away, so prices equal fundamental (equilibrium) values.
- **Law of one price** — corollary of no-arbitrage: assets with similar payoffs should trade at similar prices.
- **Efficient market hypothesis (EMH)** — the "other twin": prices change only in response to fundamental news, so they follow a random walk.

**Source:** van der Merwe (2015) pp.24-25.

## Mathematical Reasoning
Let P be the transaction price and V the fundamental (equilibrium) value. The no-arbitrage baseline asserts convergence:

  P -> V , i.e. |P - V| -> 0 under arbitrage pressure.

This holds because each of a large number of arbitrageurs takes an infinitesimal position against any gap |P - V| > 0, and their collective force drives relative mispricing toward zero. But that force is contingent on two assumptions, call them K (capital) and R (risk):

  K: arbitrage capital is unbounded (no funding constraint);
  R: the arbitrage is riskless (positions are risk-neutral toward V).

Convergence is guaranteed only on the conjunction K ∧ R. The card's hinge is the contrapositive: if ¬K ∨ ¬R, then |P - V| need not go to 0 and a persistent wedge

  Δ = |P - V| > 0

is admissible in equilibrium. Real and especially distressed markets violate K or R, so Δ > 0 is the empirical norm rather than the exception. Comparative statics through the liquidity facets: tighter spreads lower the per-round arbitrage cost (favoring convergence), greater depth lets size trade without moving price, higher resiliency shortens the recovery time of any shock-induced Δ, and immediacy availability is the on/off switch for whether the trade can be put on at all. Degrade any facet and the effective bound on persistent Δ widens.

**Source:** van der Merwe (2015) pp.25-26.

## Boundary Notes
The convergence result |P - V| -> 0 is not unconditional. It holds only on the conjunction of two named hidden assumptions: K (arbitrageurs have unlimited access to capital) and R (the arbitrage is riskless). Relax EITHER — ¬K or ¬R, which is common in real and especially distressed markets — and convergence is no longer guaranteed: a persistent wedge Δ = |P - V| > 0 is admissible in equilibrium. The no-arbitrage / EMH / law-of-one-price baseline of this card is therefore a frictionless idealization, not a description of where price actually settles once capital or risklessness fails.

**Source:** van der Merwe (2015) pp.25-26.

## See Also
- [`fa-shleifer-vishny-limits-to-arbitrage`](./fa-shleifer-vishny-limits-to-arbitrage.md) — formalizes what happens once "unlimited capital" and "no risk" are relaxed; this card is the baseline it extends from.
- [`fa-liquidity-measurement-and-price-impact`](./fa-liquidity-measurement-and-price-impact.md) — operationalizes the tightness/depth facets into measurable price-impact and spread quantities.
- [`fa-nav-staleness-and-arbitrage-speed`](./fa-nav-staleness-and-arbitrage-speed.md) — resiliency/immediacy applied to fund NAV convergence speed.
- [`fa-dual-rail-pricing-nav-vs-market`](./fa-dual-rail-pricing-nav-vs-market.md) — the law-of-one-price wedge made concrete as the fund's market-price-vs-NAV gap.
- `mt-three-dimensions-liquidity` and `mt-liquidity-measures-spread-depth-resiliency` (reading 14) define tightness/depth/resiliency from primary microstructure sources; this card adds the no-arbitrage / law-of-one-price baseline and its two hidden assumptions.

Legacy cross-refs (other tree, prose only): the behavioral-finance limits-of-arbitrage card and noise-trader-equilibrium card extend the same "mispricing persists when arbitrage is bounded" theme, and the convertible-bond arbitrage-strategy card is a concrete relative-value application of the law of one price.

## Escalate to Raw When
Go to the raw source when you need the worked S&P 500 index-versus-futures example that van der Merwe uses to illustrate why an apparent arbitrage can persist: it walks a specific risk-free rate, an index level, a futures price embedding the financing cost, an unexpected rate cut, the resulting fair-value gap, and the convergence trade that closes it. The concrete figures (carry, post-shock index level, futures level, and the resulting mispricing) live only in the book and are deliberately omitted here per the no-worked-arithmetic rule. Also escalate for Black's (1971) full four-condition definition of a liquid market and for the chapter-4 development of Shleifer-Vishny limits to arbitrage referenced at the end of this passage.

**Source:** van der Merwe (2015) pp.25-27.
