---
schema_version: "cacg.v0"
id: "mt-bid-ask-spread-immediacy-price"
title: "The Bid-Ask Spread as the Price of Immediacy"
reading_id: "14_microstructure_and_trading"
summary: "A market-order trader who completes a round trip pays the full quoted spread, i.e. one half-spread per side relative to the midpoint value estimate; the spread is what impatient liquidity demanders pay patient liquidity suppliers for immediacy."
tags: ["microstructure", "bid-ask-spread", "immediacy", "liquidity", "market-orders", "transaction-cost"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p084:0126"
    chunk_hash: "d4efd25088deb3742ece976430ab47240475f3c495ae1e0469eca323d11137c0"
    page_range: [84, 84]
    quote: "The difference of 1—which is half the bid/ask spread—is what she paid for liquidity."
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p052:0072"
    chunk_hash: "e681dc23b0690279651a32a88443f95ab78c8a496c263b2db878959cabf43993"
    page_range: [53, 53]
    quote: "the difference between the price at which a market order executes and the midquote on the market the instant before"
    edge_type: "supports"
---
# The Bid-Ask Spread as the Price of Immediacy

## Intuition
Trading is a search problem: a buyer must find a seller and vice versa. Patient
traders post standing limit orders and wait, capturing better prices because they
"search longer and harder." Impatient traders refuse to wait — they hit the
standing quotes with market orders to trade *now*. The bid-ask spread is the price
of that impatience: it is what a liquidity *demander* hands to a liquidity
*supplier* in exchange for immediacy.

Picture a single instrument with a best bid b and a best offer (ask) a. A trader
who buys immediately pays a (lifts the offer); a trader who sells immediately
receives b (hits the bid). With no other information, the best estimate of value is
the midpoint m = (a + b)/2. So the buyer overpays by a − m and the seller undersells
by m − b — each side gives up *half the spread* S/2 relative to the midpoint, where
S = a − b is the quoted spread.

```
             BID b          MID m          ASK a
              |              |              |
   seller hits|              |              |lifts buyer
   bid -> gets|<-- half ---->|<-- half ---->|-> pays
         b    |   spread     |   spread     |   a
              | (S/2 below)  | (S/2 above)  |
       SELLER's cost                   BUYER's cost
              \________ full spread S = a - b ________/
                  (cost of a market-order round trip)
```

A trader who buys with a market order and then sells with a market order — a full
round trip — loses the entire spread. The spread is therefore a transitory cost of
demanding liquidity twice, not a change in fundamental value.

**Source:** Harris (2003) §4.3.1 "Market Orders Pay the Spread" pp.84; §1.4 p.6.

## Definition
A **market order** is an instruction to trade at the best price currently
available; impatient traders use it to *demand liquidity*. A small market buy
executes at the best (lowest) ask; a small market sell executes at the best
(highest) bid.

The **bid-ask spread** S = ask − bid. The **midpoint** (midquote) m = (bid + ask)/2
is the best uninformed estimate of value. The **half-spread** is S/2 = ask − m =
m − bid: the per-side cost of demanding immediacy relative to the midpoint.

The **effective half-spread** generalizes this to realized executions: it is the
signed difference between the actual execution price and the midquote prevailing
just before the trade, S_e = d·(p − m), where d = +1 for buyer-initiated and
d = −1 for seller-initiated trades.

**Source:** Harris (2003) §4.3 pp.83-84; Foucault, Pagano & Röell (2013) §2.2.2
eq.(2.3) p.53.

## Mathematical Reasoning
Let a small uninformed trader complete a round trip with two market orders, the
quotes unchanged between them. The buy executes at the ask a; the sell executes at
the bid b. The realized round-trip loss (excluding commissions) is

    L_round = a − b = S.

Allocating this symmetrically over the two trades gives a per-trade cost of S/2.
Equivalently, measure each leg against the midpoint m = (a + b)/2:

    buy cost  = a − m = S/2,
    sell cost = m − b = S/2,

so each market order forfeits exactly the half-spread relative to the best value
estimate. This is the deviation captured by the effective half-spread S_e = d·(p − m):
for a marketable buy, p = a and d = +1 so S_e = a − m = S/2; for a marketable sell,
p = b and d = −1 so S_e = −(b − m) = S/2. Thus the effective half-spread reduces to
the quoted half-spread when execution occurs exactly at the posted quote, and it
is positive precisely because available liquidity is finite (any slippage beyond
the quote only widens it).

No fundamental value moved; the cost is purely the transitory price of immediacy,
transferred from the impatient market-order trader to the patient limit-order
trader who supplied the standing quote.

**Source:** Harris (2003) §4.3.1 pp.84; Foucault, Pagano & Röell (2013) §2.2.2
eq.(2.3)-(2.4) p.53.

## Boundary Notes
- The "pay half the spread per side" result assumes a *small* order that executes
  entirely at the posted best quote and assumes quotes are unchanged across the
  round trip. Large orders walk the book and incur additional market impact beyond
  the half-spread.
- The argument treats the trader as *uninformed*, so the midpoint is the best value
  estimate; if the order conveys information, the post-trade fair value itself
  shifts and the effective spread blends a transitory liquidity cost with a
  permanent information component.
- **Price improvement** breaks the equality the other way: in negotiable markets a
  supplier may step in front of the quote and fill a market order *inside* the
  spread, so the realized cost is below the quoted half-spread.
- The spread measures the cost of immediacy for *small* orders only; it is one of
  several liquidity dimensions (width, depth, resiliency), not the whole of
  liquidity.

**Source:** Harris (2003) §4.3.1-4.3.2 pp.84; Foucault, Pagano & Röell (2013)
§2.2.2 p.53.

## See Also
- [`mt-liquidity-measures-spread-depth-resiliency`](./mt-liquidity-measures-spread-depth-resiliency.md) -- the spread is the *width* dimension of liquidity; this card isolates its immediacy-pricing role.
- [`mt-order-types-market-limit-stop`](./mt-order-types-market-limit-stop.md) -- market orders demand liquidity (pay the spread); limit orders supply it (earn the spread).
- [`mt-liquidity-depth-immediacy-width`](./mt-liquidity-depth-immediacy-width.md) -- decomposes the immediacy/width/depth tradeoffs the spread sits within.
- [`mt-spread-equilibrium-timing-option`](./mt-spread-equilibrium-timing-option.md) -- how patient limit-order traders set the spread that demanders pay.

## Escalate to Raw When
Harris §4.3 (pp.83-84) works the Amy round-trip narrative and the midpoint-deviation
argument in full, plus the price-improvement and market-impact qualifications in
§4.3.2 that this card only names. For the formal effective- vs. realized-spread
distinction and the decomposition into transitory liquidity cost vs. permanent
price impact (which separates immediacy pricing from information), re-read Foucault,
Pagano & Röell (2013) §2.2.2-2.2.3 pp.50-54.
