---
schema_version: "cacg.v0"
id: "mt-index-portfolio-markets-design"
title: "Index and Portfolio Markets: Index Construction and ETF/Index-Product Design"
reading_id: "14_microstructure_and_trading"
summary: "Index products price an average of component prices divided by a constant divisor; their order-flow concentration makes index markets more liquid than the cash market and lets them lead price discovery."
tags: ["microstructure", "index-products", "etf-design", "price-discovery", "divisor"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p496:0833"
    chunk_hash: "96c7a5a35939a65a87ae0bacef9316a714cac2d9f5d55b5914c90f91959ed35b"
    page_range: [497, 497]
    quote: "Index markets are far more liquid than the underlying cash markets upon which their products are based."
    edge_type: "defines"
  - source_id: "mt_hasbrouck_2007_empirical_market_microstructure"
    chunk_id: "mt_hasbrouck_2007_empirical_market_microstructure:p112:0143"
    chunk_hash: "0a46b8a9d76c91c57ac492896a6dcfee4ec73e1c160c2b028cc216d348512654"
    page_range: [112, 112]
    quote: "The venue with the largest information share is in some ways"
    edge_type: "supports"
card_hash: "194ee2c6941b31e463dcd6d0e4da53d82db80e24e5e92e038e96679df71e73da"
---
# Index and Portfolio Markets: Index Construction and ETF/Index-Product Design

## Intuition
An index is just a published average of the prices (or capital values) of a fixed list of component instruments. An *index product* — an index future, an index option, or a share in an index fund — lets a trader take or hedge "index risk" in a single transaction instead of trading every component. Because almost everyone wants the same broad-based exposure, order flow concentrates in these products, and the index market becomes the cheapest, most liquid place to express a view on the whole market.

The surprising consequence is directional: even though the index is mechanically *computed* from the cash prices of its components, the index *market* is usually where the market-wide news arrives first. Index traders care only about the price of index risk; traders in the individual stocks must disentangle that common factor from a flood of firm-specific risk, so they price the common factor more slowly. The index product therefore tends to move first, and the cash index follows — the famous "tail that wags the dog."

```
  cash-market news  ----+
  (firm-specific noisy) |        +---------------------+
                        \        |  INDEX PRODUCT MKT  |   <-- order flow
  market-wide news ------+-----> |  (futures/options/  |       concentrates
  (the common factor)            |   index-fund share) |       here; quotes tight
                                 +----------+----------+
                                            |  leads
                                            v
                                 +---------------------+
                                 |   CASH INDEX        |   follows (the "dog")
                                 |  (component stocks) |
                                 +---------------------+
```

**Source:** Harris (2003) ch.23 §23.0, §23.4 pp.484-491.

## Definition
An *index* is a number proportional to a summary statistic of its components' prices, scaled by a constant *divisor*. A **price-weighted index** is proportional to the sum of component prices; the highest-priced names dominate (e.g., DJIA, Nikkei 225). A **value-weighted (capitalization-weighted) index** is proportional to total capital value of the components; the largest-cap names dominate (e.g., S&P 500). Equal-weighted and geometrically weighted variants also exist but are mainly academic.

The **divisor** is chosen so the index begins at an arbitrary base value, and is adjusted only when a structural change must leave the index level continuous — adding/deleting a component, or (for a price-weighted index) a stock split.

**Index products** are the three tradable forms of index risk: index futures contracts, cash-settled index option contracts, and ownership shares in index funds (portfolios constructed to replicate a price index). An index fund holds the components in the weights implied by the index so its returns track the index. **Source:** Harris (2003) ch.23 §23.1, §23.5 pp.485-491.

## Mathematical Reasoning
Let the components be priced p_i with shares-outstanding q_i, and let D be the divisor.

- Price-weighted: I = (Σ_i p_i) / D. Each component's marginal influence is ∂I/∂p_i = 1/D, identical across names, so a high-*priced* stock moves the index most for a given percentage move.
- Value-weighted: I = (Σ_i p_i q_i) / D. Here ∂I/∂p_i = q_i / D, so influence scales with shares outstanding (capitalization), and the largest-cap names dominate.

*Divisor continuity.* Replace a low-priced component with a high-priced one in a price-weighted index: the raw sum Σ p_i jumps upward although no economic value changed. To hold I fixed across the swap, D must rise in proportion — formally, D_new / D_old = (Σ p_i)_new / (Σ p_i)_old evaluated at the instant of substitution. A value-weighted divisor must likewise rise when a high-cap name replaces a low-cap one, but need *not* change at a split: a 2-for-1 split halves p_i and doubles q_i, leaving p_i q_i and hence Σ p_i q_i unchanged.

*Why the index leads (comparative statics of information).* Write each component price's innovation as a common (market-wide) shock plus an idiosyncratic shock. The index product isolates exposure to the common factor, so its quoted price responds to common-factor news with little contamination. Each individual stock's price must aggregate both shocks; the signal-to-noise ratio for the common factor in any single name is low, so its price adjusts to common news more sluggishly. Aggregating the slower adjustments still trails the direct, low-noise channel of the index product. In a vector-error-correction price-discovery model this is exactly the statement that the index venue carries the larger *information share* — the larger share of the random-walk innovation variance of the common efficient price. **Source:** Harris (2003) ch.23 §23.1, §23.4 pp.485-491; Hasbrouck (2007) §10.4 pp.101-102.

## Boundary Notes
- The leadership claim is empirical and statistical, not a no-arbitrage identity: index-arbitrage links the two markets, but Harris notes arbitrageurs do only a small fraction of NYSE volume, so the cash index can lag rather than instantly equalize.
- Index-market liquidity rests on two assumptions that can break: (i) *few traders are well informed about broad-based index values*, so dealers face little adverse selection and quote tight; (ii) order flow concentrates in the *same* product so dealers turn inventory fast. If a market-wide informed trader appears, or order flow fragments across competing products, spreads widen and the "tail wags the dog" effect weakens.
- The information-share interpretation depends on the causal ordering imposed when prices move (near-)contemporaneously; Hasbrouck recommends reporting min/max information shares over all causal permutations, so a single "leader" attribution is a bound, not a point identity.
- Futures vs. fund shares differ in cost structure: futures need position rollover at expiry (recurring cost) while index-fund shares avoid that but require fund-level replication. **Source:** Harris (2003) ch.23 §23.4-§23.5 pp.489-491; Hasbrouck (2007) §10.4 pp.101-102.

## See Also
- [`mt-information-shares-price-discovery`](./mt-information-shares-price-discovery.md) -- formalizes the "leads the cash index" claim as the venue with the larger information share.
- [`mt-liquidity-depth-immediacy-width`](./mt-liquidity-depth-immediacy-width.md) -- why concentrated, low-adverse-selection index order flow yields tight, deep quotes.
- [`mt-value-traders-arbitrageurs`](./mt-value-traders-arbitrageurs.md) -- index arbitrageurs are the link that ties index-product prices back to the cash basket.
- [`mt-call-vs-continuous-auction`](./mt-call-vs-continuous-auction.md) -- venue-design choices that shape where price discovery concentrates.

## Escalate to Raw When
Harris details the *institutional* taxonomy of major U.S. broad-based index products, program-trade definitions and volume shares, and index-fund management mechanics (ch.23 §23.2-§23.5, pp.486-492) that this card only summarizes; re-read there for specific products and the rollover/replication cost arguments. For the *formal* random-walk variance decomposition behind "information share" — the VECM setup, Cholesky ordering, and the di²/σw² definition — re-read Hasbrouck §10.4 pp.101-102, which this card states without re-deriving.
