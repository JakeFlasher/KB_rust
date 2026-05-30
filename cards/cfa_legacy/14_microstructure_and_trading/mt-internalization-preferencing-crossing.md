---
schema_version: "cacg.v0"
id: "mt-internalization-preferencing-crossing"
title: "Internalization, Preferencing, and Crossing Networks"
reading_id: "14_microstructure_and_trading"
summary: "Internalization (in-house dealer fills), preferencing (routing to a paying dealer at NBBO), and internal crossing all arrange trades away from organized markets, diverting benign flow from public price discovery and raising cream-skimming and best-execution concerns."
tags: ["microstructure", "internalization", "payment-for-order-flow", "best-execution", "cream-skimming", "fragmentation"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p526:0885"
    chunk_hash: "eba6151badb640a4b2c91dadd023d203d56e75d01412c392222fee7d53cd1760"
    page_range: [527, 527]
    quote: "Internalization, order preferencing, and internal order crossing all arrange trades away from organized markets."
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p030:0037"
    chunk_hash: "c0a37bc3d756fb9d3e636b5626ed5a8171e11bd47490b4bf92428a1900f64c2a"
    page_range: [30, 30]
    quote: "a broker commits to route his orders to a specific dealer, and the dealer commits to execute them at the best quoted price in the market or even to improve systematically upon these prices."
    edge_type: "supports"
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p532:0895"
    chunk_hash: "b74e43efc3ac5294287ee40fd8b8a595f8bce4e72997d2b11b004aa0470d35d8"
    page_range: [532, 532]
    quote: "Dealers expose themselves to well-informed traders and to large traders when they offer firm quotes that any trader can take."
    edge_type: "supports"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p230:0349"
    chunk_hash: "c57283cee60e3c63c11a0825a25670b6b1c6bd0006601d5b854c770b896d8331"
    page_range: [230, 231]
    quote: "If he does internalize, he will refrain from bidding in the main market; otherwise, he would be competing against himself on the internalized portion."
    edge_type: "supports"
card_hash: "8a2320aec86622e5bc555f2ff35dfd2e9c28b68953fe589ac30c6c69c79efd1b"
---
# Internalization, Preferencing, and Crossing Networks

## Intuition
A retail market order does not always reach a public exchange. A broker can dispose of it three ways without ever displaying it to the wider market. (1) **Internalize** it: a broker that also acts as a dealer fills the order out of its own book. (2) **Preference** it: route it to a chosen wholesale dealer with whom the broker has a standing relationship, typically in exchange for a payment for order flow (PFOF), with the dealer promising to fill at the national best bid or offer (NBBO). (3) **Cross** it internally: match it against another of the broker's own clients (or in a crossing network) at a reference price taken from elsewhere. All three keep the trade away from the central, displayed market.

```
       retail market order
              |
   +----------+-----------+-----------------+
   |          |           |                 |
INTERNALIZE  PREFERENCE   CROSS          (display on
broker fills route to    match client    public
own book   paying dealer  vs client      limit-order
           at NBBO        at ref. price   book)
   |          |           |                 |
   +----------+-----------+                 |
        flow NEVER hits             flow competes openly,
        the public book             helps set the price
```

The economic tension: dealers happily pay for *benign* (uninformed, small) orders because they can be filled profitably inside a wide spread, while leaving informed and large orders for the public book. Skimming the most desirable orders this way is **cream-skimming**, and it is what makes the practices controversial.

**Source:** Harris (2003) ch.25 §25, §25.2.2 pp.527, 532.

## Definition
**Internalization** — a broker-dealer fills its own clients' orders against its own account, acting in a dual broker/dealer capacity. **Order preferencing** — "the routing of order flow by a broker to a preferred dealer," normally based on the broker's relationship with the dealer rather than on quoted prices or current market conditions; most commonly small retail stock and option orders. **Payment for order flow** — pecuniary and nonpecuniary inducements dealers offer brokers in exchange for their order flows (e.g., roughly 1 cent per share in U.S. stocks). **Internal order crossing** — a broker arranging trades among its own clients, or routing limit orders to an ECN/crossing network. A **crossing network** is the only order-driven market that is not an auction market: trades occur at a derivative price taken from another market (e.g., the primary-market bid/ask midpoint), so it discovers only *whether* traders will trade at that price, not the price itself. **Best execution** in U.S. equities generally means filling marketable orders at the NBBO (or with price improvement) and ensuring standing limit orders execute at least as soon as they would in the primary market.

**Source:** Harris (2003) §25.1 pp.527-528 (internalization, preferencing, best execution); ch.6 (crossing networks as the only non-auction order-driven market) pp.145-147.

## Mathematical Reasoning
The mechanism is a competitive-equilibrium / adverse-selection argument, not arithmetic.

Adverse-selection spread: dealers offering firm public quotes face informed and large traders, so the equilibrium spread must widen to cover those expected losses; let it be `S_public`. If a dealer can instead screen flow and trade only with small uninformed clients (cream-skimming), its expected loss per trade falls, so the spread it *could* sustain on that benign subset, `S_benign`, satisfies `S_benign < S_public`. The dealer captures the gap and rebates part of it to the broker as PFOF.

Net-cost invariance under perfect competition: for a small market order, total cost = bid/ask spread + commission. In perfectly competitive wholesale and retail order-flow markets, any excess dealer profit is competed away into PFOF, and any excess broker profit is competed away into lower commissions and ancillary services. Holding all else constant, demanding greater price improvement raises commissions by a corresponding amount, so net transaction cost is invariant to how best execution is defined — `(spread) + (commission)` is pinned by competition, not by the routing rule. The harm therefore appears only where competition is imperfect (dealers/brokers with market power retain excess profit).

Anticompetitive comparative static: aggressive quoting is worthwhile only when an aggressive quote *attracts* order flow. Diverting flow to internalizers/preferred dealers means the trader posting the best price no longer wins the order, weakening the incentive to quote aggressively, so equilibrium spreads rise relative to a regime that routes every order to the best displayed price — and this holds *even if all traders were equally informed and equal-sized*. In Foucault-Pagano-Röell's dual-capacity model the same logic appears formally: an internalizing dealer refrains from bidding on the internalized portion (else he competes against himself), which raises the price impact of the routed remainder versus a ban on internalization.

**Source:** Harris (2003) §25.2 pp.530, §25.3 p.533; Foucault, Pagano & Röell (2013) §7.2.3 "Internalization and Market Power" pp.230-231.

## Boundary Notes
- **When the practices are benign:** under genuinely perfect competition in both wholesale and retail order-flow markets, low commissions exactly offset poor execution, so net prices for small market orders do not depend on the best-execution standard. The harm is conditional on market power, which "in no market is competition perfect."
- **Who gains, who loses:** internalization and preferencing can *benefit small uninformed market-order traders* (lower commissions, NBBO fills) but *harm limit-order traders* — preferenced limit orders parked at the NBBO get matched only when prices move toward them (adverse selection), shifting the dealer-vs-public-liquidity competition toward dealers.
- **Crossing-network limits:** because it uses a derivative price, a crossing network never clears excess demand; it fills only a fraction of volume, allocates the over-subscribed side by precedence rules, and depends on a credible external price — primary markets complain it skims the easiest-to-fill orders (cream).
- **Contrast with consolidation:** the alternative is a single consolidated limit order book; whether to consolidate is a genuine policy trade-off, since internal crossers may supply services they would not otherwise provide.
- **Scope:** Harris draws on U.S. equity examples, but states the forces "are present in all markets." Do not transplant the specific NBBO / SEC Rule 11Ac1-5/-6 institutional details to non-U.S. venues without re-checking.

**Source:** Harris (2003) ch.25 intro p.527 ("present in all markets"), §25.4-§25.5 pp.533-535.

## See Also
- [`mt-market-fragmentation`](./mt-market-fragmentation.md) -- why diverting flow off-exchange fragments the market and the consolidation/fragmentation trade-off
- [`mt-market-transparency-dark-pools`](./mt-market-transparency-dark-pools.md) -- non-display venues and the order-exposure problem that crossing networks share
- [`mt-call-vs-continuous-auction`](./mt-call-vs-continuous-auction.md) -- crossing networks as call markets that import prices rather than discover them

## Escalate to Raw When
Harris ch.25 (pp.527-536) is the load-bearing source for definitions, the net-cost-invariance argument, the adverse-selection-on-preferenced-limit-orders result, and the anticompetitive comparative static; re-read it before asserting any welfare claim beyond the qualitative direction sketched here. For the *formal* internalization model (closed-form price impact with vs. without internalization, the optimal internalized fraction), go to Foucault, Pagano & Röell (2013) §7.2.3 "Internalization and Market Power" pp.230-231 — this card only sketches its conclusion. Re-read Harris's crossing-networks section for the allocation/precedence mechanics, which this card summarizes.
