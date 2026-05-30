---
schema_version: "cacg.v0"
id: "mt-execution-systems-quote-vs-order-driven"
title: "Execution Systems: Quote-Driven vs Order-Driven vs Brokered Markets"
reading_id: "14_microstructure_and_trading"
summary: "Markets are classified by their execution system: quote-driven (dealer) markets where dealers supply all liquidity, order-driven markets that match public orders by precedence rules, and brokered search markets; real markets are usually hybrids."
tags: ["microstructure", "market-structure", "execution-systems", "dealer-market", "order-driven", "brokered-market"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p105:0164"
    chunk_hash: "887b3665796d5c50e5b5d7b1751dbf28c733c98f1dc52c3831b77f8e1871dfc7"
    page_range: [105, 105]
    quote: "Traders use the hours before the open to collect and submit orders."
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p027:0034"
    chunk_hash: "053bbf912c7f45f6406294a554b5486cc7fc251078dd96a415be97ae051eef96"
    page_range: [28, 28]
    quote: "In dealer markets, the final investors do not trade directly with each other"
    edge_type: "supports"
---
# Execution Systems: Quote-Driven vs Order-Driven vs Brokered Markets

## Intuition
The single most defining feature of any market is its *execution system* — the procedure
by which it matches a willing buyer to a willing seller. Because this procedure is what a
market structure is *about*, analysts classify markets first by execution system, and only
then by their finer trading rules. Harris reduces the bewildering variety of real venues to
three prototypes plus their blends: **quote-driven** (dealer) markets, **order-driven**
markets, and **brokered** markets, with **hybrid** markets mixing the three.

The distinction turns on one question: *who arranges the trade and who supplies the
liquidity?* In a quote-driven market a dealer stands between every pair of public traders —
you buy from the dealer's inventory or sell into it, at quotes the dealer posts. In an
order-driven market public buyers and sellers trade *directly* with one another, their
orders ranked and matched by impersonal precedence and pricing rules. In a brokered market
nobody posts standing liquidity at all; a broker must actively *search* for a counterparty,
which is what you do when the asset is unique or trades too rarely for dealers to inventory.

```
            WHO ARRANGES THE TRADE / SUPPLIES LIQUIDITY?
   +---------------------+---------------------+---------------------+
   |   QUOTE-DRIVEN       |   ORDER-DRIVEN       |   BROKERED          |
   |   (dealer market)    |   (auction market)   |   (search market)   |
   +---------------------+---------------------+---------------------+
   | Buyer -- Dealer --   | Buyer -------------  | Buyer  ··· Broker   |
   |           |   Seller |        match by      |      searches  ···  |
   |     posts quotes,    |   precedence+pricing |     ··· Seller      |
   |   holds inventory    |       rules          |   no standing book  |
   | public CANNOT trade  | public trade DIRECTLY| liquidity is hidden |
   |   with each other    |   with each other    |  / latent, found    |
   | e.g. bond, FX,       | e.g. futures, most   | e.g. blocks, real   |
   |      Nasdaq          |      stock exchanges |      estate         |
   +---------------------+---------------------+---------------------+
                  HYBRID = some combination of the three
                  (NYSE: order-driven + specialist obligation;
                   Nasdaq: dealer + displayed/executable limit orders)
```

**Source:** Harris (2003) *Trading and Exchanges* §5.3 pp.105-106.

## Definition
- **Execution system.** The set of procedures a market uses to match buyers to sellers;
  the defining characteristic by which markets are classified.
- **Quote-driven (dealer) market.** Dealers participate in (essentially) every trade;
  public traders trade with dealers, not with each other, at the bid and ask prices the
  dealers quote. Dealers supply all (or most) of the liquidity. Examples: most bond and
  currency markets, Nasdaq, the LSE.
- **Order-driven market.** Buyers and sellers trade directly without dealer
  intermediation; *order precedence rules* determine which buyer trades with which seller
  and *trade pricing rules* determine the price. Most are auction markets. Dealers may
  trade but on an equal footing and cannot choose counterparties. Examples: futures
  exchanges, most stock and options exchanges, ECNs.
- **Brokered market.** Brokers actively search to match buyers and sellers; the
  distinguishing feature is the broker's role in *finding* liquidity that traders will not
  post publicly (concealed or latent traders). Arises when the item is unique or trades
  infrequently — large blocks, real estate, whole businesses.
- **Hybrid market.** Mixes characteristics of the above (e.g., NYSE order-driven with a
  liquidity-of-last-resort specialist; Nasdaq dealer-based with displayed/executable
  public limit orders).

**Source:** Harris (2003) *Trading and Exchanges* §§5.3.1-5.3.4 pp.105-108.

## Mathematical Reasoning
The taxonomy is a *partition by liquidity-provision mechanism* rather than a numerical
model, but it carries sharp structural implications that the cited authors derive
qualitatively.

1. **Counterparty-choice axis.** Let the matching map send each marketable order to a
   counterparty. In dealer markets the dealer *chooses* whom to trade with (screening for
   creditworthiness and against informed flow); in order-driven markets the precedence
   rules *impose* the counterparty, so a trader "cannot choose with whom they trade." This
   non-discretion is precisely why order-driven venues need elaborate clearing/settlement
   guarantees: traders meet strangers with no bilateral credit relationship.

2. **Liquidity-source axis.** Define liquidity as standing willingness to trade. In
   quote-driven markets that willingness is *concentrated* in dealers (one quote schedule);
   in order-driven markets it is *dispersed* across the consolidated book ranked by price
   priority — "higher bids and cheaper offers are more likely to be executed"; in brokered
   markets it is *latent* and must be searched out. The three are therefore points on a
   continuum from full intermediation (dealer) to direct interaction (limit-order book),
   with the brokered case as the limit where no standing book is viable at all.

3. **Hybridization.** Since the prototypes differ only along the orthogonal axes above,
   a venue can occupy intermediate positions: imposing a dealer obligation onto an
   order-driven book (NYSE specialist) or grafting displayed limit orders onto a dealer
   book (Nasdaq). Hybrids are thus the generic case, the pure prototypes the corner cases.

No worked arithmetic is required; the content is the classification logic and its
consequences for counterparty choice, liquidity provision, and settlement risk.

**Source:** Harris (2003) §§5.3.1-5.3.3 pp.105-108; Foucault, Pagano & Röell (2013)
*Market Liquidity* §1.2 pp.21-28.

## Boundary Notes
- **"Pure" prototypes are idealizations.** Harris stresses real venues are usually hybrid;
  even Nasdaq lets dealers broker public-to-public trades, and order-driven books admit
  dealers as ordinary participants. Treat the three types as analytic anchors, not bins.
- **The classification is about the *primary* mechanism, not the asset.** The same security
  (a stock) can trade order-driven for small sizes yet brokered for large blocks, because
  dealers will not inventory and traders will not post standing orders at block size.
- **Order-driven vs call/continuous is a *further* sub-split.** Order-driven markets vary in
  *when* they match (single-price call vs continuous two-sided auction vs crossing network);
  that timing distinction is a separate card and does not change the order-driven label.
- **Foucault et al. compress the same map to two prototypes** — limit-order (auction) markets
  vs dealer markets — folding brokered search into the OTC/dealer end. The two-way vs
  three-way framing is a modeling choice, not a contradiction; Harris's brokered category is
  the search-market refinement of the illiquid OTC tail.
- **Where it breaks:** the partition blurs precisely at the regulatory boundary between
  "exchange" and "broker," which Harris notes is increasingly unclear as exchanges adopt
  order-matching and ECNs/brokerages replicate exchange functions.

**Source:** Harris (2003) §§5.3.1-5.3.5 pp.105-108; Foucault, Pagano & Röell (2013) §1.2
pp.21-28.

## See Also
- [`mt-order-precedence-price-time`](./mt-order-precedence-price-time.md) -- the precedence
  rules that *define* how order-driven markets choose which orders match.
- [`mt-dealer-inventory-control-price-discovery`](./mt-dealer-inventory-control-price-discovery.md)
  -- how the dealers who *are* a quote-driven market manage inventory and set quotes.
- [`mt-call-vs-continuous-auction`](./mt-call-vs-continuous-auction.md) -- the timing
  sub-split *within* order-driven markets (batch call vs continuous auction).
- [`mt-institutional-setting-market-types`](./mt-institutional-setting-market-types.md) --
  the broader institutional taxonomy of market types and venues.

## Escalate to Raw When
Harris §5.3 only sketches the three execution systems and gives illustrative venues; for
the full treatment of *how* each system's trading rules shape liquidity and strategy,
re-read Harris ch.6 (order-driven precedence/pricing rules) and ch.15 (block/brokered
markets). For the formal comparison of trading costs and price discovery between limit-order
and dealer markets, escalate to Foucault, Pagano & Röell (2013) §1.2-1.3 and the model
chapters they preview, which this card only summarizes qualitatively.
