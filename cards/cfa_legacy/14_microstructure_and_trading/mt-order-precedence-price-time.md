---
schema_version: "cacg.v0"
id: "mt-order-precedence-price-time"
title: "Order Precedence Rules: Price Priority and Time Precedence"
reading_id: "14_microstructure_and_trading"
summary: "Order-matching markets rank buy and sell orders hierarchically: price priority is the universal primary rule, and secondary rules such as time precedence, display, and size break ties to give a deterministic matching order."
tags: ["microstructure", "order-driven", "price-priority", "time-precedence", "limit-order-book"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p130:0206"
    chunk_hash: "032e983d01b658c21cfa0b34461bebe56b2c2217b8204301e8ab1ae7adc20336"
    page_range: [130, 130]
    quote: "All order-matching markets use price priority as their primary order"
    edge_type: "defines"
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p026:0024"
    chunk_hash: "df5a89d09107aa1346d34329ccd7ccb652eaa5a208e7bce619edd9fbfad41e09"
    page_range: [27, 27]
    quote: "markets prioritise MOs over LOs and then use a price-time priority"
    edge_type: "supports"
---
# Order Precedence Rules: Price Priority and Time Precedence

## Intuition
An order-driven market has no single dealer deciding who trades with whom. Instead, the market needs an impartial, mechanical rule to decide which of the standing orders gets to trade first when a counterparty arrives. Order precedence rules are exactly that rule: they rank every buy order and every sell order so the match is deterministic and reproducible, never discretionary.

The ranking is hierarchical. The first and universal cut is *price*: a buyer willing to pay more, or a seller willing to accept less, is more eager to trade and is rewarded with first place. Only when two orders tie on price does the market reach for a tie-breaker — most commonly the *time* the order arrived, but also whether the order was displayed, and sometimes its size.

```
INCOMING SELL ORDER  ->  match against BUY side, top of book first

BUY SIDE (ranked by precedence; top = trades first)
  Bid 103.20  Ann   (arrived 9:31)  <- highest price -> price priority
  Bid 103.20  Bob   (arrived 9:34)  <- same price, later -> loses on TIME
  Bid 103.15  Cal   (arrived 9:30)  <- lower price -> behind both above
            ^primary cut: PRICE      ^secondary cut: TIME (break ties only)
```

Because higher bids and lower offers naturally walk toward each other, price priority is self-enforcing: a trader who wants to jump the queue simply has to improve the price. The tie-breakers are not self-enforcing and must be administered by the market's matching engine.
**Source:** Harris (2003) ch.6 §6.2.1 pp.130-131

## Definition
To arrange trades, an order-matching market ranks all buy orders and all sell orders separately *in order of increasing precedence* and matches the highest-precedence orders first. The rules are applied hierarchically: the *primary* precedence rule ranks all orders; if two or more orders tie on the primary rule, *secondary* precedence rules are applied, one at a time, until every order is uniquely ranked. Every rule-based order-matching system must have at least one secondary rule.

- **Price priority (primary, universal):** buy orders bidding the highest price and sell orders offering the lowest price rank highest on their respective sides. Market orders rank highest of all because the price at which they may trade is unconstrained.
- **Time precedence (secondary):** ranks same-priced orders by submission time, earliest first. *Strict* time precedence ranks every order at a price by arrival time; *floor* time precedence gives only the first arrival at a price precedence, leaving the rest at parity. A system using only price priority plus strict time precedence is a *pure price-time precedence* system.
- **Display precedence (secondary):** displayed orders beat undisclosed (hidden) orders at the same price.
- **Size precedence / pro rata (secondary):** ranks same-priced orders by size (direction varies by market), or splits a fill in proportion to size when orders are at parity.

**Source:** Harris (2003) ch.6 §6.2.1 pp.130-131

## Mathematical Reasoning
Model the buy side as a set of orders, each carrying a price \(p_i\), arrival time \(t_i\), display flag, and size. Precedence defines a strict total order \(\succ\) on these orders. The primary rule is lexicographic-first on price: for two buy orders, \(p_i > p_j \implies i \succ j\). The tie-break set \(\{p_i = p_j\}\) is then resolved by the secondary rule; under strict time precedence, \(p_i = p_j\) and \(t_i < t_j \implies i \succ j\). Composing the rules as an ordered tuple \((\text{price}, \text{time}, \dots)\) compared lexicographically guarantees that, provided the final secondary rule never ties (strict time, with unique timestamps, does not), \(\succ\) is total — so the matching order is unique and the engine is deterministic.

The economic comparative static is that price priority is *incentive-compatible* in a way the tie-breakers are not. A trader can move from rank \(j\) to rank \(i\) ahead of a same-priced order only by improving the price by at least one tick; the cost of jumping the queue is therefore the minimum price increment. As the tick \(\to 0\), this cost \(\to 0\), so time precedence loses its bite: cheaply improving the price lets anyone leapfrog the standing order, eroding the value of being early. A larger tick raises the cost of overtaking and thus strengthens time precedence — but if the tick is too large, traders are reluctant to improve at all. The matching mechanism (best price, then oldest, then walk deeper into the book) follows directly from applying \(\succ\) repeatedly against an incoming marketable order.
**Sources:** Harris (2003) ch.6 §6.1.1, §6.2.1 pp.130-131; Cartea, Jaimungal & Penalva (2015) §1.3 pp.27-28

## Boundary Notes
- **Universality of price priority, plurality of tie-breaks.** *Every* order-matching market uses price priority as primary; the secondary rule is what differs. U.S. continuous markets historically layered public-order precedence then time; futures markets often use time or a *mix* of pro-rata and time precedence (no clean time-priority rule). So "price-time priority" is the common case, not a law.
- **Pro-rata vs. time.** Under a pure pro-rata rule there is *no* time-priority component: same-priced orders share the incoming fill in proportion to posted size. This removes the queue-position race that time precedence creates and changes optimal order-placement behavior.
- **Floor vs. strict time.** Floor time precedence ranks only the first arrival at a price; remaining same-priced orders sit at parity and need a further tie-breaker, so a market can layer multiple secondary rules.
- **Special participant priority.** Some exchanges grant extra precedence to designated market makers or supply-side traders, overriding the generic hierarchy for those orders.
- **Where it breaks as intuition.** Size-restricted orders (all-or-none, minimum-fill) usually carry *lower* precedence because they are harder to fill, so "earlier or better-priced always trades first" is not exact once restrictions enter.

**Sources:** Harris (2003) ch.6 §6.1.1, §6.2.1 pp.130-131; Cartea, Jaimungal & Penalva (2015) §1.3 pp.27-28

## See Also
- [`mt-limit-order-book-mechanics`](./mt-limit-order-book-mechanics.md) -- the data structure these precedence rules rank and consume
- [`mt-order-types-market-limit-stop`](./mt-order-types-market-limit-stop.md) -- why market orders sit atop price priority while limit orders compete on price
- [`mt-execution-systems-quote-vs-order-driven`](./mt-execution-systems-quote-vs-order-driven.md) -- precedence rules are the defining machinery of order-driven (vs. quote-driven) systems
- [`mt-call-vs-continuous-auction`](./mt-call-vs-continuous-auction.md) -- how the same precedence ranking feeds single-price call vs. continuous matching

## Escalate to Raw When
Harris ch.6 works a full pure price-time ranking example (tables 6-1/6-2) and proves how matching by price priority yields a single market-clearing price (pp.135-139); re-read there for the worked ranking and the clearing-price argument this card only sketches. For the electronic LOB matching engine, the "walking the book" mechanics, and the pro-rata-vs-time-priority contrast in futures money markets, re-read Cartea, Jaimungal & Penalva §1.3-1.4.
