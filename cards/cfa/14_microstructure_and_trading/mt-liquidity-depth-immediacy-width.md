---
schema_version: "cacg.v0"
id: "mt-liquidity-depth-immediacy-width"
title: "Liquidity as a Bilateral Search Problem: Depth, Immediacy, Width"
reading_id: "14_microstructure_and_trading"
summary: "Liquidity is the ability to trade large size quickly at low cost; it is the object of a bilateral search and decomposes into three interrelated dimensions — immediacy (time), depth (size), and width (cost)."
tags: ["microstructure", "liquidity", "bilateral-search", "depth", "immediacy", "width"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p412:0696"
    chunk_hash: "68e94584f6fae6c1b898312b197bc3a9b8d28c6a3315bd1ca376227833b1cbdb"
    page_range: [412, 412]
    quote: "\"Quickly\" refers to immediacy; \"size,\" to depth; and \"cost,\" to width"
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p017:0018"
    chunk_hash: "29d0407ec0ec4b01563ce6790858c1d34e3553bf67db9c27606c32f9b14facff"
    page_range: [17, 17]
    quote: "the ability to trade a security quickly at a price close to its consensus value"
    edge_type: "supports"
card_hash: "25d17951d943e89088ea65adb19a6c6e3982f305d49e3dab0c2baae9ace152e0"
---
# Liquidity as a Bilateral Search Problem: Depth, Immediacy, Width

## Intuition

Liquidity is one of the most heavily used and least precisely defined words in trading.
Harris cuts through the confusion by anchoring it in a single mechanism: liquidity is the
*successful outcome of a bilateral search* in which buyers look for sellers and sellers
look for buyers. A trade happens only when a buyer finds a seller (or vice versa) willing
to trade at mutually acceptable terms — so "having liquidity" means the search succeeds
cheaply, quickly, and at the size you want. This is harder than a unilateral search (Fred
shopping for a camera at the lowest posted price), because the counterparty must also want
to trade and may vanish before you commit.

Because a search produces several distinct goods at once, the word "liquidity" hides
*three* dimensions that different traders weigh differently:

```
        BILATERAL SEARCH (buyers seek sellers, sellers seek buyers)
                                |
        +-----------------------+------------------------+
        |                       |                        |
   IMMEDIACY                  DEPTH                     WIDTH
   (TIME)                     (SIZE)                    (COST)
   how fast a trade           size tradeable            cost per unit
   of given size can          at a given price          of a given-size
   be arranged                of liquidity              trade
        |                       |                        |
   impatient/small        large/block             small market-order
   traders care most      traders care most       traders identify
   about this             about this              this with the spread
```

Impatient small traders fixate on immediacy and its cost (width ≈ the bid/ask spread);
large traders fixate on depth — the size they can move without blowing through the book.
Saying a market is "liquid" without naming a dimension is what produces the cross-talk
Harris warns about. **Source:** Harris (2003) ch.19 §19.1–19.2 pp.394–399.

## Definition

Harris's summary definition: *liquidity is the ability to quickly trade large size at low
cost*, where "quickly" maps to immediacy, "size" to depth, and "cost" to width. Formally,
the three dimensions are:

- **Immediacy** — how quickly trades of a *given size* can be arranged *at a given cost*.
  Demanded with market orders.
- **Width** — the cost of doing a trade of a *given size*; the cost per unit of liquidity.
  For small trades traders identify width with the bid/ask spread (plus commissions). Also
  called market *breadth*.
- **Depth** — the size of a trade that can be arranged *at a given cost*; measured in units
  available at a given price of liquidity.

Foucault, Pagano, and Röell give a compatible compact definition of *market liquidity* as
the ability to trade a security quickly at a price close to its consensus value, and stress
this is only one of three interrelated notions of liquidity (market, funding, monetary).
**Sources:** Harris (2003) ch.19 §19.2 pp.398–399; Foucault, Pagano & Röell (2013) §0.4.1
p.9.

## Mathematical Reasoning

Harris frames the search as a *production function*: the primary input is time spent
searching, and the outputs are good prices and adequate size. Holding the technology of the
market fixed, the three dimensions trade off against one another along this function:

| Change | Hold constant | Implication |
|---|---|---|
| Spend more time searching | Size of trade | Expect a better average price |
| Spend more time searching | Price willing to pay/receive | Expect to find more size |
| Increase desired size | Time, price | Expect a worse average price |
| Offer a better price | Size of trade | Expect to spend less time searching |
| Offer a better price | Time searching | Expect to find more size |

These are comparative-statics statements (signs of partial derivatives along the production
function), not numeric predictions: more search-time ⇒ better price *or* more size; more
demanded size ⇒ worse price *or* more time; a better offered price ⇒ less time *or* more
size.

A key structural fact: *width (breadth) and depth are duals.* The problem "minimize the cost
of trading a given size" has the same solution as "maximize the size traded at a given cost."
Depth (size at a given price) and breadth (price at a given size) therefore encode
essentially the same liquidity information from opposite directions — both are solved by
searching efficiently. **Source:** Harris (2003) ch.19 §19.2 pp.398–399.

## Boundary Notes

- The three-dimension decomposition is descriptive of the bilateral-search *mechanics*; it
  does not by itself predict spread *levels*. What *determines* width (adverse selection,
  inventory risk, volatility, order-processing cost) is a separate question handled by the
  spread cards.
- Width-as-spread is a *small-trade* identification. For larger orders, width includes the
  price concession beyond the inner quote, which is governed by depth/market impact rather
  than the posted bid/ask spread alone.
- A fourth, dynamic property — *resiliency* (how fast liquidity returns after it is consumed)
  — is related to the bilateral search but, per Harris, much less directly than immediacy,
  width, and depth; it lives in the liquidity-measures card.
- "Liquidity" in Harris means *market* liquidity. Foucault et al. caution that market,
  funding, and monetary liquidity are distinct (though feedback-linked) notions; conflating
  them is a common source of confusion in policy discussion.
- The framework is structure-agnostic: it applies to quote-driven and order-driven markets,
  but who supplies each dimension (dealers vs. value traders vs. limit-order submitters)
  varies by structure — see the supplier-taxonomy card.
**Sources:** Harris (2003) ch.19 §19.2 pp.398–399; Foucault, Pagano & Röell (2013) §0.4.2–0.4.3 pp.9–10.

## See Also

- [`mt-bid-ask-spread-immediacy-price`](./mt-bid-ask-spread-immediacy-price.md) -- width for small trades: the spread is the price of immediacy.
- [`mt-liquidity-supplier-taxonomy`](./mt-liquidity-supplier-taxonomy.md) -- who supplies immediacy vs. depth (dealers, value traders, limit-order traders).
- [`mt-market-impact-price-concession`](./mt-market-impact-price-concession.md) -- depth in action: the price concession large orders pay.
- [`mt-liquidity-measures-spread-depth-resiliency`](./mt-liquidity-measures-spread-depth-resiliency.md) -- operationalizing these dimensions plus resiliency as measures.

## Escalate to Raw When

Harris ch.19 develops the unilateral-vs-bilateral search analogy and the production-function
trade-off table (Table 19-2) in full; this card only sketches the comparative statics. Re-read
§19.1 for the search-strategy mechanics and §19.2 (esp. the breadth/depth duality discussion)
when you need the precise definitional wording or the supplier-by-dimension mapping in §19.3.
For the market/funding/monetary distinction and the liquidity-spiral feedback, escalate to
Foucault, Pagano & Röell (2013) §0.4.
