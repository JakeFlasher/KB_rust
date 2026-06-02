---
schema_version: "cacg.v0"
id: "mt-spread-equilibrium-timing-option"
title: "Spread Anatomy: Limit-Order Timing Option and Adverse Selection"
reading_id: "14_microstructure_and_trading"
summary: "The equilibrium bid/ask spread compensates limit-order suppliers for the free timing option they grant market-order traders (a form of adverse selection) plus limit-order management costs, widening with volatility and slow cancellation."
tags: ["microstructure", "bid-ask-spread", "limit-order", "adverse-selection", "timing-option"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p319:0528"
    chunk_hash: "960f30269cc0ad32207f550ffa9a327961fd2e2c20ba56a3950e53b12b23101f"
    page_range: [320, 320]
    quote: "limit order traders give a valuable timing option to market order traders when they do not continuously update their limit order prices"
    edge_type: "defines"
card_hash: "65b8b25cdd02c167ba56d16d6cf6d80e95616ee0acfe5aaff88dc61b3f20b2a3"
---
# Spread Anatomy: Limit-Order Timing Option and Adverse Selection

## Intuition

When you post a standing limit order, you are not just naming a price — you are handing other traders a *free, exercisable option to trade against you at that price*. A resting sell limit at 20.00 is a call written to the market: anyone may "buy at 20.00" whenever it suits them. Crucially, you cannot reprice that order instantaneously. Between the moment you post and the moment you can cancel, the fair value of the asset drifts. A patient, attentive market-order trader watches that drift and pulls the trigger only when the move has gone *his* way — buying from you precisely when the asset has become worth more than your stale limit price. That selectivity is the *timing option*.

```
       value path during your "cannot-reprice" window
   v ^
     |        ........  value rose  --> market BUYER lifts your
     |      ..        \                 stale sell limit (you lose)
 20 -+----o------------\----------------- your resting SELL limit @ 20
     |    .             ..  value fell --> nobody lifts; you are stuck
     |   .                \                holding, wishing you had sold
     +----+----+----+----+----+--> time
        post              cancel-effective
        order             (delayed)
   Filled mostly on the UP moves you'd rather not trade on
   -> classic adverse selection against the liquidity supplier
```

The asymmetry — you trade exactly when you wish you hadn't, and fail to trade when you wish you had — is a *form of adverse selection*. The market-order trader behaves like a better-informed counterparty not because he has private information, but because he has *more current* information at the instant of trade than you had when you set the price. For liquidity supply to survive in equilibrium, the spread must be wide enough to pay limit-order traders back for the options they give away.

**Source:** Harris (2003) *Trading and Exchanges* ch.14 §14.4.2.2 pp.307-308.

## Definition

Setup (Harris's simple spread model): traders choose between a *market order* (immediate execution, pays the spread) and a *limit order* (offers liquidity, may not fill). In equilibrium traders must be indifferent between the two strategies, which pins down the spread.

- **Timing option**: the right, granted by a standing limit order, for a market-order trader to wait, observe value changes, and execute against the limit price only when doing so is profitable for him (and adverse for the limit-order trader).
- **Adverse selection (here)**: the limit-order trader systematically transacts on the value moves unfavorable to him and misses the favorable ones, so his realized fill price is worse than the fair value conditional on trading.
- **Limit-order management cost**: fees plus opportunity/attention cost of cancelling and resubmitting orders; because these are positive, traders reprice only when value diverges materially, leaving the timing option live.
- **Equilibrium spread**: the round-trip cost of two market orders, set so liquidity suppliers are exactly compensated.

**Source:** Harris (2003) ch.14 §14.4.2.1-§14.4.2.2 pp.306-308.

## Mathematical Reasoning

Harris derives the equilibrium spread as a sum of indifference conditions, with no numeric plug-in required for the structural result.

The round-trip cost of liquidity demand equals the spread (two market orders close a position). For traders to be indifferent between supplying and demanding liquidity, the spread must reimburse limit-order traders for (i) the management cost they bear and market-order traders do not, and (ii) the timing option they write. Harris states the structural identity directly:

> the equilibrium spread must equal the expected cost of managing the limit orders plus twice the value of the timing option.

The factor of *two* arises because the round-trip spread reflects a buy-side and a sell-side limit order, each of which writes a timing option. Comparative statics follow from the option's value:

- **Speed asymmetry**: the timing option is most valuable when market-order traders can react faster than limit-order traders can reprice. If limit traders could cancel before any exploitable move, option value -> 0 and the spread compresses toward the management-cost floor.
- **Cancellation latency**: slower order cancellation -> longer exposure window -> more value can accumulate -> wider equilibrium spread. The spread "depends on the average time it takes limit order traders to successfully cancel their orders."
- **Volatility**: holding cancellation slow, higher instrument volatility raises the probability and magnitude of an exploitable value move within the window, raising option value -> wider spread for volatile instruments.
- **Competition among option-exercisers**: when many market-order traders chase the same opportunity, each must act instantly and may lose it to a quicker trader, which *reduces* the per-trader value extracted but does not flip the sign — liquidity suppliers are more willing to post in active markets.

Direction of all effects: spread is increasing in volatility, increasing in cancellation latency, increasing in management cost, and increasing in the speed advantage of market-order traders.

**Source:** Harris (2003) ch.14 §14.4.2.2 pp.307-309.

## Boundary Notes

- **Assumptions**: equal-sized orders; traders are equally (or commonly) informed about value, so this is *not* the Glosten-Milgrom private-information channel — the "adverse selection" here is purely a *repricing-latency* phenomenon. Harris notes the equilibrium-spread results survive merely if all traders are *equally* well informed.
- **When the channel vanishes**: if limit-order traders could reprice continuously and instantaneously (zero attention/order-handling latency), the timing option is worthless and this component of the spread disappears — only management cost and time-value/risk-aversion terms remain.
- **Volatility caveat**: Harris flags that results involving volatility use the volatility of the *estimate* of value, which by a statistical bound is below the volatility of the true value being estimated.
- **Contrast with inventory and order-processing components**: the timing-option/adverse-selection term is distinct from dealer inventory-control compensation and from fixed per-trade processing costs; it is the piece that scales with latency and volatility.
- **Empirical inference**: since inside-quote sizes are typically small, small traders predominantly set inside spreads, so limit-order execution risk tends to dominate market-order price risk in setting the inside spread.

**Source:** Harris (2003) ch.14 §14.4.2.2-§14.4.2.4 pp.308-310.

## See Also

- [`mt-spread-decomposition-components`](./mt-spread-decomposition-components.md) -- the timing-option/adverse-selection term as one additive piece of the total spread.
- [`mt-limit-order-book-equilibrium`](./mt-limit-order-book-equilibrium.md) -- how standing limit orders aggregate into book depth under the same option-writing logic.
- [`mt-bid-ask-spread-immediacy-price`](./mt-bid-ask-spread-immediacy-price.md) -- the spread as the price of immediacy: market traders buy time, limit traders sell it.
- [`mt-dealer-inventory-control-price-discovery`](./mt-dealer-inventory-control-price-discovery.md) -- the complementary inventory-risk spread channel.

## Escalate to Raw When

You need the formal indifference algebra behind "expected management cost plus twice the option value," the worked Lisbet/Mark/Tim/Dieter timing-option example that decomposes a fill into timing-option vs quote-matching profit, or the precise statement of the estimate-volatility bound. Re-read Harris (2003) ch.14 §14.4.2 (pp.306-310), especially the boxed "A Simple Timing Option Example" on pp.308-309.
