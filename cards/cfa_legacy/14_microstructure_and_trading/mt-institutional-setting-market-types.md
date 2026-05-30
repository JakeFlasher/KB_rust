---
schema_version: "cacg.v0"
id: "mt-institutional-setting-market-types"
title: "The Institutional Setting: Limit-Order, Dealer, and Hybrid Markets"
reading_id: "14_microstructure_and_trading"
summary: "Trading venues reduce to two prototype mechanisms — limit-order (auction) markets where investors interact directly through a consolidated book, and dealer markets where intermediaries quote two-sided prices — plus hybrids combining both under distinct transparency regimes."
tags: ["microstructure", "limit-order-market", "dealer-market", "hybrid-market", "market-structure", "transparency"]
citations:
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p021:0025"
    chunk_hash: "f6017404046c9f32b07ce337bd816889212966309cf71b24d9dadaf447a57492"
    page_range: [22, 22]
    quote: "Limit order or auction markets are centralized trading mechanisms in which potential"
    edge_type: "defines"
card_hash: "7b82a06afabd34db2f2a9c3623a8a8f4cf9d3ec1b1f6151699aeec212b711c32"
---
# The Institutional Setting: Limit-Order, Dealer, and Hybrid Markets

## Intuition
Real-world securities markets look bewilderingly diverse — floor auctions, electronic books, OTC phone-and-screen networks, periodic crossing pools — and the rules can be combined in essentially unbounded ways, so a complete taxonomy is hopeless. The productive move is to recognize that almost every trading venue is a variation on just two *prototype* mechanisms, and then to ask which one (or which blend) a given venue implements. That single framing organizes the entire institutional landscape.

In a **limit-order (auction) market**, the final investors interact *directly*: anyone can supply or demand liquidity by posting orders, which are consolidated into a single limit-order book (LOB) ranked by price priority. In a **dealer market**, there is a sharp role split — specialized intermediaries (dealers / market makers) post the bid and ask quotes, and the public can only trade against those quotes, never directly with one another. A **hybrid market** stitches the two together (e.g., a public book plus designated dealers, or a specialist who must better the book).

```
            LIMIT-ORDER MARKET                 DEALER MARKET
        investors trade DIRECTLY            investors trade via DEALERS
        +------------------------+          +------------------------+
        |   buy orders | sell    |          |  Dealer A: bid/ask     |
        |   (bids)     | orders  |          |  Dealer B: bid/ask     |
        |   ----- consolidated --|          |  Dealer C: bid/ask     |
        |   into ONE book by     |          |  (NOT consolidated;    |
        |   PRICE PRIORITY       |          |   no price priority)   |
        +------------------------+          +------------------------+
                  \                                   /
                   \           HYBRID                /
                    +---- book + designated dealers, ----+
                         specialist must improve on book
```

The choice is not cosmetic: it governs *who* supplies liquidity, *whether* a single consolidated price exists, and *how much* trading information participants can see — and those properties feed directly into trading costs and price discovery, which the rest of the microstructure literature studies.
**Source:** Foucault, Pagano & Röell (2013) §1.1–1.2 pp.16–28

## Definition
Foucault, Pagano & Röell define the two prototypes precisely:

- **Limit-order (auction) market.** A centralized trading mechanism in which participants reveal trading interest by submitting orders that are matched directly by the platform. Bids and offers are consolidated in a limit-order book (LOB) according to **price priority**, so that higher bids and cheaper offers are more likely to execute. Each participant chooses whether to provide or demand liquidity. Examples: BATS (US), Chi-X (Europe).
- **Dealer market.** Final investors do not trade with each other; they must contact a **dealer** (market maker) who quotes an ask (the price at which the public buys) and a bid (the price at which the public sells), and trade at that quote. There is a sharp distinction between liquidity *suppliers* (dealers) and liquidity *demanders* (investors); quotes are *not* consolidated to enforce price priority. Often OTC. Example: US/European corporate bond markets.
- **Hybrid market.** A venue comprising both a limit-order platform and a dealer segment, or mixing features of both — e.g., a public LOB alongside designated dealers, or a quote-driven market that adds a routing facility to a book.

Within dealer markets, FPR further distinguish a **retail segment** (dealers serve final investors) from a **wholesale / interdealer segment** (dealers trade with one another to share inventory risk); interdealer volume typically dwarfs retail volume.
**Source:** Foucault, Pagano & Röell (2013) §1.2–1.2.3 pp.22–28

## Mathematical Reasoning
The two prototypes differ structurally in their *price-formation map* from order flow to executions, and these structural facts admit comparative reasoning even without numerics.

1. **Consolidation and price priority (limit-order market).** Because all orders enter one book ranked by price, the executable price faced by an incoming marketable order is obtained by *walking the book*: the order consumes resting liquidity from the best price outward. Aggregating across resting volume defines a weighted-average bid–ask spread that is **monotonically increasing in trade size** — larger orders reach into worse-priced layers.

2. **Fragmentation and absence of price priority (dealer market).** Quotes are posted independently by dealers and are *not* forced into a single priority ordering, so an investor who does not observe all quotes can trade with a dealer (e.g., "Beta") even when a strictly better quote exists elsewhere (e.g., "Alpha"). Search is costly. Yet when quotes *are* displayed (e.g., Nasdaq, SEAQ), the **inside spread** ("market touch") formed by consolidating the best bid and best ask across dealers is no wider than — and generally strictly tighter than — any individual dealer's spread: the market as a whole supplies more liquidity than any single dealer. Walking the aggregate dealer supply/demand curve again yields a weighted-average spread increasing in size.

3. **Liquidity-role symmetry vs. asymmetry.** In a limit-order market each participant *endogenously chooses* to post (supply) or take (demand) liquidity; in a dealer market the roles are *fixed* by institutional design (dealers supply, investors demand). This asymmetry is what makes dealer **inventory risk** — and its compensation through the spread — a first-order pricing determinant in dealer markets, a comparative-statics theme developed later in the book.
**Source:** Foucault, Pagano & Röell (2013) §1.2.1–1.2.2 pp.22–27

## Boundary Notes
- **Prototypes, not a partition.** FPR are explicit that a *complete* classification is hopeless; the two-prototype scheme is a modeling device, and most real venues are hybrids. NYSE is cited as running open-outcry, a single-specialist dealer mechanism, and an electronic LOB *simultaneously* per stock, coordinated by priority rules.
- **Price-setting vs. crossing.** Both prototypes possess an internal price-setting mechanism balancing supply and demand. This distinguishes them from **crossing networks** (e.g., POSIT), which set no price of their own but cross accumulated orders at a price *imported* from another venue — so crossing networks are not a third price-forming prototype.
- **Transparency is orthogonal.** A market's transparency (how much trade/quote information participants see) varies widely *within* both prototypes; it is a separate dimension of design, treated in depth in FPR Chapter 8 and routed here to the dark-pools card.
- **Quote firmness caveat.** Some "dealer market" quotes (e.g., indicative FX screen quotes) do not commit the dealer to trade, and OTC corporate bonds may show no real-time quotes at all — so "dealer market" spans a spectrum from firm-and-displayed to indicative-and-opaque.
**Source:** Foucault, Pagano & Röell (2013) §1.1, §1.2–1.2.4 pp.16–29

## See Also
- [`mt-execution-systems-quote-vs-order-driven`](./mt-execution-systems-quote-vs-order-driven.md) -- maps the limit-order/dealer prototypes onto the order-driven vs. quote-driven execution-system vocabulary
- [`mt-limit-order-book-equilibrium`](./mt-limit-order-book-equilibrium.md) -- formalizes price formation and the increasing-with-size spread inside the limit-order-market prototype
- [`mt-market-transparency-dark-pools`](./mt-market-transparency-dark-pools.md) -- develops the orthogonal transparency dimension and venue opacity introduced here

## Escalate to Raw When
FPR §1.2 only sketches the institutional anatomy; for the *equilibrium* logic — how inventory risk and adverse selection translate into the bid–ask spread, and formal comparative statics across the two structures — re-read Chapter 3 (inventory) and the relevant adverse-selection chapters. For the transparency mechanics and welfare/distributional effects only gestured at here, re-read Chapter 8. The empirical comparisons of limit-order vs. dealer markets previewed in §1.3 carry the supporting evidence, and §1.4 carries the political-economy account of *why* venues choose a given structure.
