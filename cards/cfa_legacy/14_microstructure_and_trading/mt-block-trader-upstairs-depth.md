---
schema_version: "cacg.v0"
id: "mt-block-trader-upstairs-depth"
title: "Block Traders and Upstairs Markets: Supplying Depth to Large Orders"
reading_id: "14_microstructure_and_trading"
summary: "Upstairs block dealers and brokers supply depth to large uninformed traders by screening counterparties for information and honesty, since exposing a block invites front-running and adverse selection."
tags: ["microstructure", "block-trading", "upstairs-market", "adverse-selection", "liquidity-supply", "order-exposure"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p341:0568"
    chunk_hash: "15aec3aca166a8b7f5c76e17061c97b3a5153388b4b356c5f05f7a2a4d3779df"
    page_range: [341, 342]
    quote: "Good block traders must carefully set the prices at which they facilitate trades with their clients to ensure that they will be able to trade out of their positions at a profit."
    edge_type: "defines"
---
# Block Traders and Upstairs Markets: Supplying Depth to Large Orders

## Intuition

A trader who wants to buy several days' worth of normal volume cannot just hit
the public book — the displayed depth is far too thin, and lifting visible offers
one after another will walk the price up against him. Such block initiators turn
instead to **block traders** working the **upstairs market**: dealers and brokers
who arrange large trades by telephone, away from the public order-driven book.
The upstairs market exists precisely because exchanges and screen-based systems
exchange only price and size, but block trading requires exchanging a richer kind
of information — *who* the initiator is, *why* they trade, and *how much* they
truly want.

The block trader's job is to find **latent demand** — responsive traders who would
trade if asked but have posted no order — and to do so without spoiling the market.
The central tension is **information leakage**: every counterparty shown the order
is a potential front-runner. If the order is widely "shopped," it hangs over the
market and prices run away before it can be filled.

```
       LARGE UNINFORMED INITIATOR
                  |  (cannot find size on public book)
                  v
        +-------------------+
        |   BLOCK TRADER    |  screens: informed? honest about size?
        | (dealer / broker) |
        +-------------------+
          /              \
   take position      assemble counterparties
   (block dealer)     (block broker)
          \              /
                  v
     RESPONSIVE LIQUIDITY SUPPLIERS  <-- shown SELECTIVELY,
     (latent demand discovered)          most-trustworthy first
```

Upstairs traders thus "primarily supply depth" — the ability to trade large size at
a contained, predictable price concession.

**Source:** Harris (2003) ch.15 §15.2, §15.4 pp.339-341

## Definition

A **block trade** is any trade resulting from an order too large to fill easily
through normal procedures — typically more than a normal day's volume, or by common
practice more than a quarter of average daily volume in an active stock. The
**block initiator** originates the large order; **block liquidity suppliers** (dealers
or large buy-side traders) fill it.

**Block traders** intermediate and divide into two roles:

- **Block dealers** (a.k.a. block positioners / facilitators) fill the client order
  *from their own account*, taking the position and later trading out of it.
- **Block brokers** (a.k.a. block assemblers) find and organize *other* traders
  willing to fill the order, charging commission.

The **upstairs market** is the telephone-based block market in wirehouse trading
desks; it serves large traders who cannot convey credible information about their
trading motives and intentions to the regular market.

**Source:** Harris (2003) ch.15 §15.1, §15.4 pp.339, 341

## Mathematical Reasoning

Block initiators face four coupled frictions, each a screening or signaling problem
rather than an arithmetic one:

1. **Latent demand.** Most liquidity suppliers are *responsive*, not posting orders.
   The probability that a given called counterparty will fill is low, so the block
   trader's productivity rises super-linearly in the number of maintained
   relationships: with *n* clients there are on the order of *n(n−1)/2* possible
   pairings to match across, a network-externality argument for why hard-working,
   well-connected brokers dominate.

2. **Order exposure.** Each counterparty shown the block is a potential
   front-runner. Same-side traders accelerate and opposite-side traders retard their
   trading, both of which amplify the block's price impact. Hence the broker shows
   the order to the *most likely fillers first* and avoids those who have leaked or
   front-run before — a repeated-game mechanism in which the threat of exclusion
   sustains trustworthy behavior.

3. **Price discrimination.** A supplier fears prices will keep moving against him as
   more size follows, so he wants to know the *true total size* before quoting.
   Credible size revelation is impossible under anonymity (lying carries no penalty
   when reputation cannot be built), so block trading is necessarily a
   *named-counterparty* setting where an honest reputation, or an audit of intentions,
   substitutes for the missing commitment.

4. **Asymmetric information.** Large traders are *suspected* informed — they spread
   research costs over larger portfolios and informed traders want maximum size.
   Suppliers therefore demand a price concession analogous to the adverse-selection
   component of a dealer's bid/ask spread: the concession lets uninformed-flow gains
   offset informed-flow losses. The initiator's countermove is to *de-anonymize* and
   prove he is uninformed, inverting the usual value of an "informed" reputation.

Comparative advantage then partitions the market: **dealers** dominate small/urgent
blocks (they can take a position before placing it and trade out patiently);
**brokers** dominate the very largest blocks (dealers will not warehouse that much
inventory risk). A broker who "puts a finger in the guillotine" by trading alongside
the suppliers signals incentive alignment.

**Source:** Harris (2003) ch.15 §15.2.1-15.2.4, §15.4.3 pp.340-341

## Boundary Notes

- **Holds when** counterparties are *named* and *repeated*, so reputations for honesty
  and for being uninformed are enforceable. In purely anonymous markets the screening
  mechanism collapses — traders lie with impunity — which is why blocks migrate
  upstairs rather than to the anonymous book.
- **Sunshine trading** (publicly announcing identity, size, and motive) is the
  alternative to upstairs intermediation, but it works only for the largest,
  best-known, demonstrably uninformed traders (the textbook LOR portfolio-insurance
  case); for everyone else it merely hands free trading options to front-runners.
- **Distinguish depth from immediacy:** market makers primarily supply *immediacy*
  (instant small fills); upstairs traders primarily supply *depth* (large size at
  contained concession). The block dealer's concession must cover the *expected
  liquidation cost* of the position he takes, not a one-period quote.
- Does *not* describe Chinese-CB execution mechanics (T+0, call-auction, soft-call);
  this is generic Western equity-block microstructure (folder is deferred per the
  CB-focus pivot).

**Source:** Harris (2003) ch.15 §15.2.5, §15.3, §19.6 pp.339-341

## See Also

- [`mt-liquidity-supplier-taxonomy`](./mt-liquidity-supplier-taxonomy.md) -- block dealers/brokers as one branch of the liquidity-supplier taxonomy
- [`mt-market-impact-price-concession`](./mt-market-impact-price-concession.md) -- the price concession block suppliers demand for size and adverse selection
- [`mt-lit-dark-pool-optimal-execution`](./mt-lit-dark-pool-optimal-execution.md) -- non-display venues (crossing networks) as the modern order-exposure solution
- [`mt-market-transparency-dark-pools`](./mt-market-transparency-dark-pools.md) -- transparency trade-offs that motivate upstairs/dark execution

## Escalate to Raw When

Harris develops each of the four block-trading problems with worked institutional
examples (the IBM "doghouse" deception, 13F-Holdings audits to verify honesty,
hot-IPO allocation as a penalty device, the LOR sunshine-trading case) and the
broker/dealer comparative-advantage split that this card only sketches. Re-read
ch.15 §15.2-15.4 (pp.339-342) when you need the exact screening mechanisms, the
front-running strategy table (Table 15-1), or the precise statistical definitions of
a block by exchange.
