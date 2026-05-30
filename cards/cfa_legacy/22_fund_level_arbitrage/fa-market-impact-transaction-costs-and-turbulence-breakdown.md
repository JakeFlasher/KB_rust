---
schema_version: "cacg.v0"
id: "fa-market-impact-transaction-costs-and-turbulence-breakdown"
title: "Market Impact, Best-Execution Tension & Turbulence Breakdown"
reading_id: "22_fund_level_arbitrage"
summary: "Order-size price impact rises with size and falls with liquidity; best execution can fight tracking-error minimization; and in turbulence price discovery fails, so an apparent premium may be a measurement-error gap rather than a riskless arbitrage."
tags: ["market-impact", "best-execution", "arbitrage-breakdown"]
citations:
  - source_id: "fa_weiner_2021_etf_portfolio"
    chunk_id: "fa_weiner_2021_etf_portfolio:p125:0122"
    chunk_hash: "ac509b1b0c24b8a6be1448f449697156bf4c5a69c481ddb776b102fb74aeefab"
    page_range: [126, 126]
    quote: "In general, larger orders will have greater market impact, and securities with lower average daily volumes will have greater market impact."
    edge_type: "defines"
  - source_id: "fa_weiner_2021_etf_portfolio"
    chunk_id: "fa_weiner_2021_etf_portfolio:p124:0121"
    chunk_hash: "7822725dce0e6d20946459d527d65ed8570f055c92a7893a6bbcd293e36ed64d"
    page_range: [125, 125]
    quote: "designing the trade to help minimize the market impact of the transaction."
    edge_type: "supports"
  - source_id: "fa_weiner_2021_etf_portfolio"
    chunk_id: "fa_weiner_2021_etf_portfolio:p275:0275"
    chunk_hash: "8de37b358fcce0998b1dde16db8eeb588046333fbb6de5e2b687b03ed50893a7"
    page_range: [276, 276]
    quote: "we tend to think of arbitrage as a riskless attainable profit."
    edge_type: "defines"
  - source_id: "fa_weiner_2021_etf_portfolio"
    chunk_id: "fa_weiner_2021_etf_portfolio:p273:0273"
    chunk_hash: "77e80f7471ed3485141c0de91c6c24f2a5f76d5a4ff3b858988d33c5d31f2d04"
    page_range: [274, 274]
    quote: "In practice, what happens is that IVs are calculated using the stale prices."
    edge_type: "supports"
  - source_id: "fa_madhavan_2016_etfs_new_dynamics"
    chunk_id: "fa_madhavan_2016_etfs_new_dynamics:p118:0141"
    chunk_hash: "1a8d3763e54c806fa389f609413580a43e36eab567dc9b9934950bbac842af2e"
    page_range: [118, 118]
    quote: "both of which suggest that bond ETF premiums really reflected price discovery, not dislocation."
    edge_type: "supports"
card_hash: "95682e7c061f48c2ab619550afb32426e1b869d946217bc3f0a07e23b1037a42"
---
# Market Impact, Best-Execution Tension & Turbulence Breakdown

## Intuition
A trade does not execute against a single quoted price; pushing a large order into a finite book moves the price against you. Market impact is the price an order pays for its own size — and it grows with order size and shrinks with the security's traded liquidity. So the portfolio manager faces a deeper tension than "minimize cost." The cheapest way to *match the index* (a market-on-close order at the index's reference price) can be the *worst* way to serve the shareholder when the security is thin: it concentrates the whole order at one moment and pushes the print. Best execution and tracking-error minimization can therefore point in opposite directions, and fiduciary duty resolves the tie in favor of the shareholder, accepting more tracking error if execution is better.

```
order size  ->  market impact (monotone up)
liquidity   ->  market impact (monotone down)

         best execution                tracking-error min
        (work the order,                (MOC at index ref price,
         pockets of liquidity)           one moment, thin book)
                \                              /
                 \---- TENSION (fiduciary) ---/
                          |
                resolve toward shareholder
                (accept higher TE if execution better)
```
**Source:** Weiner (2021) ch.8 pp.125-126.

## Definition
- **Market impact:** the effect on the price of an asset that results from an order to buy or sell that asset. It is increasing in order size and decreasing in the security's average daily volume (liquidity).
- **Best execution:** a federal fiduciary standard — execute so the client's total costs or proceeds are the most favorable under the circumstances. It is *not* merely minimizing a single trade's commission; it includes designing the trade to minimize market impact.
- **Best-execution vs tracking-error tension:** transacting to minimize tracking error to the index (e.g., a market-on-close print at the index reference price) may not benefit the shareholder; improving index-relative performance at the expense of overall performance is not a trade-off a PM should make.
- **Turbulence / arbitrage breakdown:** in stress, price discovery can fail so that the indicative value (IV/NAV) is built on stale prices. An apparent premium or discount versus IV is then a *measurement-error gap*, not a riskless attainable profit — there is no simultaneous buy-low/sell-high, so no true arbitrage exists despite the visible spread.
**Source:** Weiner (2021) ch.8 pp.125-126; ch.18 pp.273-276.

## Mathematical Reasoning
Let `Q` be order size and `L` a liquidity proxy (e.g., average daily volume). Market impact `I` obeys the comparative statics
```
dI/dQ > 0      (impact monotone increasing in order size)
dI/dL < 0      (impact monotone decreasing in liquidity)
```
Total execution cost decomposes as `C = commissions + spread + I(Q, L)`, so even a "cheap" commission does not bound `C` when `Q` is large relative to `L`. Best execution minimizes `C`, not just the commission term.

The tension is an objective conflict: let `TE` be tracking error to the index and `S` be shareholder net proceeds. The MOC-at-reference policy minimizes `TE` but, in a thin book, raises `I` and lowers `S`. Fiduciary duty optimizes `S` subject to accepting larger `TE` — i.e., it does not minimize `TE` when doing so degrades `S`.

The turbulence breakdown is a statement about what the observed price gap *measures*. Write reported value `IV = sum_i w_i * p_i`, where some `p_i` are stale (last trade at time `t - tau`, `tau > 0`). The true value uses contemporaneous prices `p_i*`. The visible gap between secondary price `P` and `IV`,
```
P - IV = (P - V_true) + (V_true - IV),
```
splits into a *dislocation* term `(P - V_true)` and a *measurement-error / staleness* term `(V_true - IV)`. Arbitrage requires a riskless, simultaneously attainable profit — i.e., it lives only in `(P - V_true)`. When discovery fails, the gap is dominated by `(V_true - IV)`, which is not tradeable, so no arbitrage exists even though `|P - IV|` is large.
**Source:** Weiner (2021) ch.8 pp.125-126; ch.18 pp.274-276. Supporting (the bond-ETF stress case where a >1% premium reflected price discovery, not dislocation): Madhavan (2016) §8.6 pp.117-118.

## Boundary Notes
- The "apparent premium is a measurement-error gap, not a tradable arbitrage" conclusion is scope-limited.
- It holds only in the turbulence / price-discovery-failure regime, where the IV/NAV is built on stale `p_i` and the gap is dominated by the staleness term `(V_true - IV)`.
- In normal conditions price discovery is intact, so the same secondary-price-vs-fair-value gap `(P - V_true)` can be a genuine, simultaneously attainable arbitrage rather than noise.
- The model also assumes the staleness is the binding friction; if the creation/redemption channel is itself broken, a real dislocation can persist even outside the stale-price regime.
**Source:** Weiner (2021) pp.271-277.

## See Also
- [`fa-volume-neq-liquidity-idts-ebils-components`](./fa-volume-neq-liquidity-idts-ebils-components.md) — market impact depends on real liquidity, not raw printed volume; the components decomposition supplies the `L` here.
- [`fa-nav-staleness-and-arbitrage-speed`](./fa-nav-staleness-and-arbitrage-speed.md) — the staleness term `(V_true - IV)` and how it gates arbitrage speed.
- [`fa-true-vs-reported-premium-price-discovery-share`](./fa-true-vs-reported-premium-price-discovery-share.md) — separating reported premium into a measurement gap versus genuine price-discovery signal.
- [`fa-limits-to-arbitrage-when-creation-channel-breaks`](./fa-limits-to-arbitrage-when-creation-channel-breaks.md) — the complementary breakdown on the creation/redemption side in stress.
- [`fa-tracking-error-attribution-and-tco`](./fa-tracking-error-attribution-and-tco.md) — transaction costs as a one-way contributor to tracking error.
- [`mt-market-impact-price-concession`](../14_microstructure_and_trading/mt-market-impact-price-concession.md) (reading 14) derives the same dI/dQ>0, dI/dL<0 impact comparative statics from the limit-order book; this card states the result and applies it to ETF best-execution.
- [`mt-implementation-shortfall`](../14_microstructure_and_trading/mt-implementation-shortfall.md) (reading 14) is the master execution-cost measure whose impact term is the cost priced here (note its "tracking error" denotes a dollar opportunity-cost level, not the excess-return volatility meant in this card).
- `mt-buy-side-trader-best-execution` (reading 14) owns the "best execution is relative, not best-price-in-the-abstract" fiduciary framing this card invokes.
- `pa-transaction-based-attribution-and-trading-cost` (reading 15) is where the transaction cost this card minimizes is later absorbed into an active value-add attribution.

Legacy cross-refs (other tree, prose only): `pm-active-risk-and-tracking-error` (reading 09) frames the index-relative active-risk objective that best execution overrides here; `bf-limits-to-arbitrage` (reading 10) carries the same "apparent mispricing is not always a free lunch" lesson in a fundamentals-vs-sentiment idiom.

## Escalate to Raw When
Go to Weiner ch.8 for the worked best-execution-vs-tracking-error scenario: the somewhat-illiquid security needing a market-on-close fill, and the acquisition example where the PM weighs a round-trip in the acquired ticker against simply holding cash through the rebalance (with the per-share commission/spread and share-count figures that quantify the round-trip cost). Go to ch.18 for the COVID-19 turbulence case studies — the bond-ETF (AGG) IV dislocation, the August 24, 2015 equity-ETF prints far below intrinsic value, and the circuit-breaker mechanics — where the concrete dollar gaps and basis-point spreads are spelled out. Use Madhavan §8.6 for the bond-ETF "Taper Tantrum" empirics and the two methods that attribute the premium to price discovery rather than dislocation.
**Source:** Weiner (2021) ch.8 pp.130-132; ch.18 pp.271-277; Madhavan (2016) §8.6 pp.117-118.
