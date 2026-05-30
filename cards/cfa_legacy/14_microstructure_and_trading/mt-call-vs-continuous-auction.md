---
schema_version: "cacg.v0"
id: "mt-call-vs-continuous-auction"
title: "Call (Single-Price) vs Continuous Auctions and Pricing Rules"
reading_id: "14_microstructure_and_trading"
summary: "Call auctions batch orders and clear all trades at one uniform price; continuous two-sided auctions match arriving orders under a discriminatory rule and quote a bid-ask spread; crossing networks use derivative (reference) pricing."
tags: ["microstructure", "call-auction", "continuous-auction", "pricing-rule", "order-driven", "crossing-network"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p133:0211"
    chunk_hash: "4bc62ac496042105cf42b6427ebbb6400fd6a2e789b50522f8d532c7c476975f"
    page_range: [133, 133]
    quote: "In a single price auction, all trades take place at the same"
    edge_type: "defines"
---
# Call (Single-Price) vs Continuous Auctions and Pricing Rules

## Intuition
Order-driven markets differ chiefly in *when* they match and *how* they price. A
**call market** freezes time: it collects orders, then at the call makes one
attempt to arrange trades, clearing every matched buyer and seller at a single
market-clearing price. A **continuous two-sided auction** runs in real time:
each arriving order is tested against the best standing order on the other side,
so trades happen one at a time at prices that drift as liquidity is consumed. A
**crossing network** does neither — it imports a price from somewhere else and
only asks who is willing to trade at that imported price.

The same orders, fed to these three structures, produce three different
experiences. In the call, one batched clearing price; in the continuous book, a
standing bid and offer separated by a spread that an incoming marketable order
walks through; in the cross, a midpoint or closing reference price with no price
discovery at all.

```
 CALL (single-price)        CONTINUOUS (two-sided)        CROSSING NETWORK
 batch -> 1 clearing px      arrive -> match -> repeat     import ref px
 all trades @ P*             trades @ standing limit pxs   trades @ outside px
   S                         best bid | best offer         no price discovery,
   |\   /D                     20.0   |   20.1             only quantity match
   | \ / .... P* (S=D)        <----- spread ----->         (e.g. midpoint, close)
   |  X                       marketable order walks book
```

**Source:** Harris (2003) Trading and Exchanges ch.6 §6.2.3-6.5 pp.121-145

## Definition
- **Single-price (call) auction.** Orders are collected before the call; "In a
  single price auction, all trades take place at the same marketclearing price."
  The last match that leads to a feasible trade sets that price, and price
  priority guarantees the clearing price is feasible for all higher-priority
  matches. It uses the **uniform pricing rule**.
- **Continuous two-sided auction.** The market keeps an **order book** of
  standing orders sorted by precedence; the highest bid and lowest offer are the
  best bid and best offer. An arriving order is **marketable** if it offers terms
  acceptable to the best opposite order; marketable orders trade immediately,
  non-marketable orders rest in the book. It uses the **discriminatory pricing
  rule**: each trade prices at the *standing* order's limit price.
- **Crossing network.** "Crossing networks are the only order-driven markets that
  are not auction markets"; all trades occur at a price determined elsewhere
  (e.g., the primary-market midpoint or close). It uses the **derivative pricing
  rule** and performs no price discovery — only quantity matching.

**Source:** Harris (2003) Trading and Exchanges §6.2.3, §6.3, §6.4, §6.5 pp.120-145

## Mathematical Reasoning
Let the limit book define an upward-sloping supply schedule S(p) (cumulative size
sellers will sell at price <= p, summed from low prices up) and a downward-sloping
demand schedule D(p) (cumulative size buyers want at price >= p). Feasible volume
at any candidate price is min(S(p), D(p)).

- **Call / uniform rule.** The single clearing price P* is set where supply meets
  demand. Below P* there is excess demand, so volume is supply-constrained and
  rises with price along S; above P* there is excess supply, so volume is
  demand-constrained and falls with price along D. Hence trading volume is
  maximized at the crossing P*, and the single price maximizes total trader
  surplus when the outcome is feasible. Discreteness of prices/quantities means
  exact equality S(P*) = D(P*) rarely holds, leaving residual excess on one side
  resolved by secondary precedence.

- **Continuous / discriminatory rule.** A marketable order matches the
  highest-precedence opposite order; if unfilled, it then matches the next, and
  so on, each fill priced at that standing order's limit. So a single large
  marketable buy that consumes several offers transacts at a *sequence* of
  ascending prices — it walks up the book. The realized average price therefore
  weakly exceeds the inside quote, and the gap is the size-dependent price
  impact. The standing best bid and best offer define a positive bid-ask spread;
  liquidity demanders pay this spread, liquidity suppliers earn it.

- **Crossing / derivative rule.** The clearing price p_cross is exogenous, so
  S and D do not pin it down; quantity matched is min(buy interest, sell interest)
  at p_cross and the oversubscribed side is rationed by precedence. Because price
  is independent of submitted orders, the cross cannot clear excess on its own.

**Source:** Harris (2003) Trading and Exchanges §6.3.2, §6.4, §6.5 pp.122-145

## Boundary Notes
- The uniform rule is a property of *single-price* auctions specifically; "a few
  call markets" instead use the discriminatory rule, so "call market" and
  "uniform pricing" are not synonyms. Likewise continuous matching does not
  *require* a continuous structure — call markets exist too — the pricing rule and
  the timing are distinct design choices.
- All three are **order-driven**: traders cannot choose counterparties, and
  precedence rules (price first, then secondary tie-breaks such as time) arrange
  matches. The continuous and call cases are both two-sided *auctions*; only the
  crossing network is a non-auction order-driven market doing no price discovery.
- Many exchanges combine modes — e.g., opening with a single-price call, then
  continuous trading, and re-opening halts via single-price auctions — so a real
  session may switch pricing rules within the day.
- Worked numeric clearing examples in the source (the 20.0 book) are illustrative;
  this card states the mechanism, not arithmetic.

**Source:** Harris (2003) Trading and Exchanges §6.2.3, §6.3, §6.5 pp.121-145

## See Also
- [`mt-order-precedence-price-time`](./mt-order-precedence-price-time.md) -- the precedence (price, then time) rules every auction here applies to rank and match orders.
- [`mt-execution-systems-quote-vs-order-driven`](./mt-execution-systems-quote-vs-order-driven.md) -- situates these order-driven auctions against quote-driven (dealer) execution.
- [`mt-index-portfolio-markets-design`](./mt-index-portfolio-markets-design.md) -- crossing networks and reference (derivative) pricing as used in portfolio/index trading venues.

## Escalate to Raw When
The source proves (via the supply/demand schedule and trader-surplus tables in
§6.3.2-§6.3.3) *why* the single price maximizes volume and total surplus, and
walks a full continuous-book example showing the discriminatory rule and quote
evolution (§6.4.1). Re-read Harris (2003) pp.121-145 for the surplus-maximization
argument, the discrete-price rationing detail, and the exact crossing-network
reference-price mechanics (POSIT midpoint, after-hours close) before relying on
any of those specifics.
