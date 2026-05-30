---
schema_version: "cacg.v0"
id: "mt-market-impact-price-concession"
title: "Market Impact and Price Concession on Large Orders"
reading_id: "14_microstructure_and_trading"
summary: "Large market orders move prices because counterparties are scarce and fear adverse selection; the premium buyers pay and discount sellers grant is the price concession, making market impact the dominant cost of trading size."
tags: ["microstructure", "market-impact", "price-concession", "liquidity", "transaction-costs"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p084:0127"
    chunk_hash: "9199bc7f8ae70d10e50c16abda145fad0a366005c5b94261f9f5d51eb10b731a"
    page_range: [85, 85]
    quote: "Large sellers offer prices down to encourage buyers to buy from them."
    edge_type: "defines"
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p098:0130"
    chunk_hash: "00ec26db21e99d1345cf5b494c433d9830eec65f0a67c7d6e04e75cf8d58e264"
    page_range: [98, 99]
    quote: "order will have an adverse price impact: increasing the price when buying"
    edge_type: "supports"
---
# Market Impact and Price Concession on Large Orders

## Intuition

A small market order trades against liquidity that is already sitting at the
best quotes; it pays half the bid/ask spread and goes away. A *large* market
order is different in kind, not just degree. The trader willing to take the
other side of a very large trade is hard to find — the natural counterparty may
not want the instrument at all, and any counterparty who does step up worries
that whoever is demanding so much size may know something they do not. To pull
reluctant counterparties out, the impatient trader must sweeten the deal: a
large buyer bids the price up to coax sellers out, a large seller offers the
price down to coax buyers in. That sweetener — the premium the buyer pays above
the prior price, the discount the seller accepts below it — is the **price
concession**. The act of moving the price to fill your own order is **market
impact** (equivalently, price impact).

```
            price
              ^
   ask quote  |======X  <- large BUY walks UP the book, paying concession
              |      /
   midprice   |.....*.................. prior reference price
              |    \
   bid quote  |======X  <- large SELL walks DOWN the book, granting concession
              +------------------------> cumulative quantity executed
                        |--- one order ---|
   concession = signed gap between fill prices and the prior reference price
```

Because the concession grows with order size, market impact is typically the
single largest cost of trading large orders — larger than commissions, and for
big institutional orders larger than the visible half-spread. Traders submitting
large market orders often pay well more than half the bid/ask spread for
immediacy. Whether the impact is large or small depends entirely on available
liquidity: in thin markets even modest orders move prices a lot (and truly large
orders may be impossible to fill), while in deep, active markets very large
orders can execute with little impact.

**Source:** Harris (2003) §4.3.3 Market Impact pp.85-86.

## Definition

Let a marketable order demand quantity $Q$ on one side. Decompose the available
liquidity into a price/quantity schedule at the moment of submission.

- **Price concession:** the signed difference between the prices at which the
  order's shares actually fill and the prior reference (e.g., the midprice or
  the touch before the order arrived) — a premium for a buyer, a discount for a
  seller, demanded by counterparties as the inducement to supply liquidity.
- **Market impact (price impact):** the price movement *caused by the order
  itself* as it consumes liquidity and "walks the book." Cartea, Jaimungal, and
  Penalva define it directly: a large order has an adverse price impact,
  "increasing the price when buying aggressively and lowering it when selling."
- **Zero-impact size:** the quantity available at the best quote (the displayed
  depth at the touch) sets the largest order that fills with no walk and hence
  no impact; beyond it, the order steps through successive price levels.

Market impact is therefore the price-dimension consequence of demanding more
immediacy than the resting liquidity at the touch can supply.

**Source:** Harris (2003) §4.3.3 pp.85-86; Cartea, Jaimungal & Penalva (2015)
§4.3.4 Price Impact p.98.

## Mathematical Reasoning

Model the consolidated offer side of the book as an increasing marginal-price
schedule $p(q)$, $q \in [0, Q]$, with $p(0)$ the best offer and $p'(q) \ge 0$
(deeper levels are more expensive). A market buy of size $Q$ that walks the book
pays total cost

$$
C(Q) = \int_0^{Q} p(q)\,dq,
$$

so the average execution price is $\bar p(Q) = C(Q)/Q = \tfrac{1}{Q}\int_0^Q p(q)\,dq$,
which is a quantity-weighted mean of marginal prices and therefore satisfies
$\bar p(Q) \ge p(0)$ whenever $p$ is nondecreasing — the average fill is never
better than the touch, the inequality being strict once the order exceeds
displayed depth. The realized price concession per share is
$\bar p(Q) - p(0) \ge 0$, and it is monotone nondecreasing in $Q$: differentiating,
$\frac{d}{dQ}\bar p(Q) = \frac{p(Q) - \bar p(Q)}{Q} \ge 0$ since the next
marginal price $p(Q)$ weakly exceeds the running average. Comparative statics
follow immediately:

- **Size:** larger $Q$ reaches higher levels, so both total impact and average
  concession weakly rise — impact increases with order size.
- **Liquidity / depth:** flatter, deeper schedules (small $p'$) compress the
  concession; thin schedules (large $p'$, few levels) amplify it. The
  zero-impact threshold is exactly the depth at the touch.
- **Convexity:** when $p$ is convex, doubling size *more* than doubles total
  impact, which is the formal driver behind splitting large orders over time.

A symmetric argument applies to the bid side for a sell: prices walk down and
the discount is again weakly increasing in size. No worked arithmetic is needed;
the inequalities above are the mechanism.

**Source:** Harris (2003) §4.3.3 pp.85-86; Cartea, Jaimungal & Penalva (2015)
§4.3.4 p.98.

## Boundary Notes

- **Cause vs. coincidence.** Market impact is price movement *caused by the
  order itself*; it must be separated from exogenous quote changes occurring
  between submission and execution (which Harris files under *execution price
  uncertainty*, not impact). Conflating the two overstates impact.
- **Permanent vs. temporary.** This card treats the immediate, mechanical
  walk-the-book concession. Part of that move reverts after the order finishes
  (temporary impact, the liquidity-supply premium) and part persists (permanent
  impact, the information the order revealed). The split is the subject of the
  temporary/permanent-impact card; do not assume the full concession is a
  recoverable cost.
- **Holds:** continuous markets where liquidity is finite at each price level
  and counterparties price adverse-selection and inventory risk. **Breaks down**
  as a pure size effect in deep, highly liquid markets, where even very large
  orders execute with negligible impact, and at the other extreme in markets so
  thin that a large order is simply unfillable rather than merely expensive.
- **Marketable limit orders** cap the concession by refusing fills beyond a
  limit price: they trade immediacy completeness for a bound on what the trader
  pays for liquidity.

**Source:** Harris (2003) §4.3.3-4.3.4 pp.85-87; Cartea, Jaimungal & Penalva
(2015) §4.3.4 p.98.

## See Also

- [`mt-liquidity-depth-immediacy-width`](./mt-liquidity-depth-immediacy-width.md) -- depth at the touch sets the zero-impact size; impact is the price cost of demanding immediacy beyond it.
- [`mt-temporary-permanent-price-impact`](./mt-temporary-permanent-price-impact.md) -- decomposes the realized concession into a reverting liquidity premium and a persistent information component.
- [`mt-implementation-shortfall`](./mt-implementation-shortfall.md) -- market impact is the dominant component of the shortfall between decision price and realized execution.
- [`mt-kyle-lambda-market-depth-price-impact`](./mt-kyle-lambda-market-depth-price-impact.md) -- gives the equilibrium linear price-impact coefficient $\lambda$ underlying these schedules.

## Escalate to Raw When

Harris (2003) §4.3.3 (pp.85-86) gives the institutional definition and the
orange-juice-futures worked example showing brokers raising the fill price to
clear size; re-read for the full numerical walk and for how it ties into
execution price uncertainty (§4.3.4) and order splitting. Cartea, Jaimungal &
Penalva (2015) §4.3.4-4.3.5 (pp.98ff.) and Chapter 6 supply the empirical
measurement of impact (walking the LOB, impact on midprice over time) and the
optimal-liquidation theory that formalizes minimizing impact under temporary
and permanent components — go there for the dynamic-strategy results this card
only names.
