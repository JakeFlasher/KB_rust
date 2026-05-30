---
schema_version: "cacg.v0"
id: "mt-order-types-market-limit-stop"
title: "Order Types: Market, Limit, and Stop Orders"
reading_id: "14_microstructure_and_trading"
summary: "Market orders demand immediacy at the best available price and pay the spread; limit orders cap the trade price and supply liquidity as standing options; stop orders convert to market or limit orders once a trigger price trades."
tags: ["microstructure", "order-types", "market-order", "limit-order", "stop-order"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p083:0125"
    chunk_hash: "687cb87c812c0f386067c3e1dac4b7387c57f66f946c2c2ed25cc0faa97ed392"
    page_range: [84, 84]
    quote: "A market order is an instruction to trade at the best price currently available"
    edge_type: "defines"
---
# Order Types: Market, Limit, and Stop Orders

## Intuition
An order is an instruction a trader gives a broker or exchange about *whether*, *at what price*, and *under what condition* to trade. The three canonical building blocks differ along a single axis: how much certainty of execution the trader is willing to trade away for control over price. A **market order** wants to trade *now* and accepts whatever the book offers. A **limit order** wants a *price no worse than X* and accepts that it may never trade. A **stop order** wants to *do nothing until the market moves to a trigger*, then behaves like one of the first two.

A market buyer crosses the spread, lifting the best (lowest) offer; a market seller hits the best (highest) bid. Because the round-trip pays the full bid/ask spread, a market trader effectively pays roughly half the spread per trade as the price of immediacy. A limit trader does the opposite: by standing in the book at a chosen price, she *offers* immediacy to others and earns the spread instead of paying it — but only if someone arrives to take her order.

```
                  CERTAINTY OF EXECUTION  <-------------->  CONTROL OF PRICE
   MARKET ORDER  |#######################################|                  |
                 |  trade now, price = best available, pay the spread       |
   LIMIT ORDER   |                  |#########################################|
                 |  trade only at limit price or better, may never fill     |
   STOP ORDER    |  dormant until trigger price trades, then -> MKT (or LMT) |
                 +----------------------------------------------------------+
   trigger: BUY stop activates when price RISES to stop; SELL stop when price FALLS
```

**Source:** Harris (2003) ch.4 §§4.3-4.5 pp.84-91

## Definition
- **Market order**: an instruction to trade at the best price currently available in the market; it demands liquidity and usually fills quickly, but at an uncertain price that depends on order size and available liquidity (Harris §4.3).
- **Limit order**: an instruction to trade at the best price available *but only if it is no worse than the limit price* — for a buy, the trade price must be at or below the limit; for a sell, at or above. If no counterparty is immediately willing, it stands in the **limit order book** until it trades, expires, or is cancelled (Harris §4.4).
- **Marketable limit order**: a limit order whose limit price is at or beyond the opposite quote (buy at/above best offer, sell at/below best bid), so it can execute immediately — like a market order but with a cap on price concessions.
- **Stop order**: an order carrying a *stop instruction* that prevents execution until price reaches a trader-specified **stop price**; a buy stop activates when price rises to the stop, a sell stop when price falls to the stop. Once triggered it is treated as an ordinary order (most commonly a market order; a **stop-limit order** activates a limit order). Once active it remains valid even if price crosses back over the stop.

**Source:** Harris (2003) ch.4 §§4.3.1, 4.4, 4.5, 4.5.1 pp.84-91

## Mathematical Reasoning
Let the inside quotes be best bid `b` and best offer `a`, with `a > b`, midpoint `m = (a+b)/2`, and spread `s = a - b`. For an uninformed small market buy, execution is at `a`, so the cost relative to the value estimate `m` is `a - m = s/2`; a small market sell receives `b`, costing `m - b = s/2`. A round trip (buy then sell at unchanged quotes) loses `a - b = s`, i.e. `s/2` per leg — the **price of immediacy**. Price improvement, when a counterparty steps inside the quote, only lowers this cost, so the per-trade liquidity cost is `<= s/2` plus, for large orders, additional **market impact** beyond the inside size.

The execution comparison is an inequality on attainable prices. A buy limit at `L` executes only at price `p <= L`; a sell limit only at `p >= L`. Hence a limit order weakly dominates a market order on price but only conditionally executes — it bears **fill risk** in exchange for eliminating **execution-price uncertainty**. A market order bears the reverse trade-off: guaranteed fill, uncertain price.

A standing limit order is an *option to trade* held by the rest of the market: a standing sell limit is a call (others may buy at the limit), a standing buy limit is a put (others may sell at the limit), with the limit price as the strike. Its value to the market rises with (i) how close the limit price sits to the market, (ii) how long the order will stand, and (iii) price volatility — the same comparative statics as option value increasing in moneyness, time, and volatility. The writer grants this option for free rather than selling it, which is why volatile markets push limit prices farther from the touch and widen spreads.

For a stop order, define the trigger as a first-passage condition: a sell stop at `K` activates at the first time `t` with traded price `P_t <= K`; a buy stop at `K` at the first `P_t >= K`. Activation therefore *adds same-direction order flow exactly when price has already moved that way* — selling pressure into falling markets, buying pressure into rising markets — so stop flow is momentum-amplifying and demands liquidity precisely when it is scarcest. Crucially, the activation price `K` is not the execution price: the order then trades at the next available price, which in a fast move can be materially worse than `K`. Guaranteeing `K` requires a true option contract, not a stop.

**Source:** Harris (2003) ch.4 §§4.3.1, 4.4.2, 4.5, 4.5.2 pp.84-91

## Boundary Notes
- **Stop vs limit are not the same despite both quoting a price.** A limit price *constrains the execution price* (trade only at limit or better); a stop price *only activates the order* and does not bound the fill. A sell stop is typically placed *below* the current price; a sell limit *above* it.
- **Stop does not guarantee the trigger price.** In a gap or fast market the triggered market order can fill well past the stop; only a purchased option contract transfers that price risk to a writer.
- **Liquidity sign flips by order type.** Market orders (and triggered stop-market orders) *demand* liquidity; standing limit orders *supply* it. A market-if-touched order is the contrarian mirror of a stop (buy on falls, sell on rises) and stabilizes rather than destabilizes price.
- **Modern electronic framing matches the taxonomy.** In limit-order-book markets the same two primitives appear as aggressive **Market Orders (MOs)** that execute immediately against the book and passive **Limit Orders (LOs)** posted at a price that waits to be matched or cancelled — confirming the demand-vs-supply-of-liquidity split holds in algorithmic venues, not just floor markets.
- **Scope:** this card covers the three core order *types* and their liquidity roles; precedence among resting orders, full book mechanics, and venue fee structures belong to the sibling cards.

**Source:** Harris (2003) ch.4 §§4.5.1, 4.6 pp.91-93; Cartea, Jaimungal & Penalva (2015) §1.3 pp.9-10

## See Also
- [`mt-bid-ask-spread-immediacy-price`](./mt-bid-ask-spread-immediacy-price.md) -- quantifies the half-spread that market orders pay for immediacy and limit orders earn
- [`mt-limit-order-book-mechanics`](./mt-limit-order-book-mechanics.md) -- how standing limit orders are stored, matched, and consumed by incoming market orders
- [`mt-order-precedence-price-time`](./mt-order-precedence-price-time.md) -- the rules deciding which resting limit order fills first

## Escalate to Raw When
Harris §4.4.2 develops the full limit-order-as-option argument (how it differs from a contract, and the joint dependence of option value on limit price, standing time, and volatility) that this card only sketches; re-read pp.86-88. For the destabilizing momentum dynamics of stop orders and the contrast with market-if-touched and contrarian limit strategies, re-read §§4.5.2-4.6 pp.91-93. For the electronic matching-engine view of MOs interacting with the LOB, see Cartea, Jaimungal & Penalva (2015) §1.3 pp.9-10.
