---
schema_version: "cacg.v0"
id: "mt-competition-within-among-markets"
title: "Competition Within and Among Markets: Floor vs Automated Trading Systems"
reading_id: "14_microstructure_and_trading"
summary: "Trading venues compete for order flow and listings on cost, speed, transparency, and fairness; cheap automation shifted the cost structure and let electronic systems erode floor-based incumbents."
tags: ["microstructure", "market-structure", "trading-venues", "automation", "competition"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p562:0949"
    chunk_hash: "bf7b943c8dae901198aecd62854021d9f0c112c9e98ba3031a4d6727267a7c69"
    page_range: [562, 562]
    quote: "low operating costs, while floor-based systems are quite costly to operate."
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p037:0049"
    chunk_hash: "1eedf05f41c6f0360ac0c7c64a6426b10feb73b7bd1ecee6982b42d75ea40721"
    page_range: [37, 38]
    quote: "The intensification of competition between trading platforms is the result of three forces:"
    edge_type: "supports"
card_hash: "4a74afe432aa844e381c89e5aa87061eea98d8f91821a1688d0be03bfcd13d5f"
---
# Competition Within and Among Markets: Floor vs Automated Trading Systems

## Intuition

Securities trading happens somewhere, and that "somewhere" is itself a product
sold by a competing supplier. Exchanges, ECNs, brokers, and dealers compete for
order flow (and, separately, for listings and for the sale of market data) by
offering different bundles of cost, speed, transparency, and operational
fairness. Competition operates on two axes: *within* a single market (traders
on the same venue compete to supply liquidity at the best price) and *among*
markets (venues compete for the order flow itself). The decisive shift of the
last few decades is that automation collapsed the cost of building and running
a venue, so the *among-markets* competition intensified and order flow migrated
toward whichever structure delivered better execution.

The core comparison is floor-based vs automated (screen-based) trading systems.
A floor-based oral auction and an automated rule-based auction are, structurally,
almost the same order-driven market — both match buys to sells under
price-time-style precedence rules — differing mainly in the *technology* that
arranges the match: humans exchanging information on a floor versus computers
matching to predetermined rules. Harris weighs them on fairness, convenience,
capacity, speed, efficiency, and cost; the asymmetry that drives outcomes is
cost.

```
   COMPETITION FOR ORDER FLOW (among markets)
   +--------------+         +------------------+
   | FLOOR-BASED  |         |  AUTOMATED / LOB  |
   | high op cost |  <--->  |  low op cost      |
   | labor-heavy  | order   |  fixed-cost-heavy |
   | skill/honor  | flow    |  rule-exact,fast  |
   +--------------+ migrates+------------------+
        incumbent      to lower-latency / lower-cost venue
```

**Source:** Harris (2003) ch.27 "Floor versus Automated Trading Systems"
pp.543-554.

## Definition

*Competition among markets* is rivalry between market centers (exchanges, ECNs,
brokers, dealers) to attract order flow, listings, and data sales by offering
services their clienteles value. Harris splits these services into *private*
services (benefit only the user, e.g., order-routing; easy to meter and charge
for) and *public* services (benefit everyone regardless of where they trade,
e.g., price-continuity promotion and trading-rule enforcement; hard to charge
for). *Competition within a market* is rivalry among traders on the same venue,
chiefly liquidity suppliers competing to post the best quotes.

A *floor-based trading system* arranges trades by traders personally exchanging
information in a physical location (oral auction). An *automated (electronic)
trading system* accepts, validates, processes, and matches orders by computer
according to predetermined rules, routing via high-speed lines; once built, it
runs with little human intervention.

**Source:** Harris (2003) ch.26 §26.5 "Regulatory Services" pp.538-539; ch.27
pp.543-545; Foucault, Pagano & Röell (2013) §1.4.2-1.4.3 pp.34-38.

## Mathematical Reasoning

The mechanism is a cost-structure argument, not arithmetic. Let a venue's total
cost be C = F + v·Q, with F the fixed (setup) cost, v the variable (per-trade)
operating cost, and Q traded volume. Floor and screen systems both carry high
F, but they differ sharply in v: a floor market is labor-intensive (brokers,
reporters, supervising officials), so v is large; an automated system, once
built, has "small operating costs because everything is automated," so v is
near zero (telecom and data-backup only). Average cost per trade C/Q = F/Q + v
therefore falls faster in Q for the automated system, and its asymptotic floor
(v) is far lower. Hence economies of scale are stronger and the marginal cost
of an extra trade is lower under automation.

Comparative statics follow. As communication/computing technology advances, F
for an electronic platform falls toward a low fixed sum, lowering the entry
barrier; many entrants (ECNs, ATSs, MTFs) appear. With smart order-routing,
the search cost of comparing prices across venues drops, so order flow becomes
more elastic to spread and fee differences. A venue facing a rival with lower
v can be undercut on fees, so order flow migrates to the lower-cost / lower-
latency structure — the empirically "mixed but tilting toward electronic"
outcome (Paris Bourse over London for French stocks; DTB over LIFFE for German
T-bond futures; NYSE Arca eventually displacing the NYSE floor). Incumbents
respond by merging (to spread F and capture the liquidity externality) and by
automating.

This same low-v / low-latency advantage is precisely what later enabled
algorithmic and high-frequency strategies, since matching now occurs at
millisecond latency under transparent rules.

**Source:** Harris (2003) ch.27 §27.7 "Cost" pp.547-554; Foucault, Pagano &
Röell (2013) §1.4.2 pp.35-37.

## Boundary Notes

The cost argument assumes that automation does not destroy the value the floor
adds. Harris is careful: floor structures *may* encourage liquidity supply
(skilled brokers, relationship-based order handling, market-reporter judgment),
in which case eliminating the floor would be foolish; the verdict is genuinely
mixed, not a clean win for screens. The migration examples are confounded —
Paris's gain coincided with the 1994 repeal of a French transaction tax, and the
LIFFE→DTB move was partly a German repatriation effort — so they do not prove
automation dominates on its merits alone. Floor markets retain skill-and-honor
fairness but are vulnerable to documented scandals (front running, prearranged
trades) that automated systems structurally preclude *within* the system (though
fraud can still occur "on the side").

A separate boundary: unregulated among-markets competition can *underprovide*
public/regulatory services and can erode meaningful minimum price increments
(venues undercut each other), weakening secondary precedence and the incentive
to expose limit orders. So more competition is not unambiguously welfare-
improving; this is why Reg NMS (US, 2005) and MiFID (EU, 2007) re-engineered the
competition rather than merely deregulating it.

**Source:** Harris (2003) ch.26 §26.5.2-26.5.3 pp.538-539; ch.27 §27.1
"Fairness" pp.545-547; Foucault, Pagano & Röell (2013) §1.4.2 pp.36-37.

## See Also

- [`mt-market-fragmentation`](./mt-market-fragmentation.md) -- the flip side of
  among-markets competition: when does order flow split across venues vs
  consolidate.
- [`mt-limit-order-book-mechanics`](./mt-limit-order-book-mechanics.md) -- the
  electronic LOB is the dominant automated structure that displaced floors.
- [`mt-bubbles-crashes-circuit-breakers`](./mt-bubbles-crashes-circuit-breakers.md)
  -- automation's low-latency matching raises capacity/reliability and crash-
  control stakes.

## Escalate to Raw When

Harris ch.27 lays out the full six-dimension comparison (fairness, convenience,
capacity, speed, efficiency, cost) with detailed reliability/security/speed
requirements and the floor labor-cost breakdown that this card only summarizes;
re-read pp.545-554 for the dimension-by-dimension argument. For the empirical
competition history and the Reg NMS / MiFID order-protection-rule mechanics —
which this card only names — see Foucault, Pagano & Röell (2013) §1.4.2 pp.35-37
and the cross-referenced Chapter 6/7 analyses of order-protection rules and the
liquidity externality.
