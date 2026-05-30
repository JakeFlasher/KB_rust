---
schema_version: "cacg.v0"
id: "mt-informed-traders-price-efficiency"
title: "Informed Traders, Fundamental Value, and Price Efficiency"
reading_id: "14_microstructure_and_trading"
summary: "Informed traders trade on estimates of fundamental value, buying undervalued and selling overvalued instruments; this trading makes prices informative and, in efficient markets, makes price changes an unpredictable random walk."
tags: ["microstructure", "informed-trading", "price-efficiency", "fundamental-value", "random-walk"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p236:0384"
    chunk_hash: "37b336887687f79defe862d9dcb76bff59d8a9733bf1e2af166f04813ba498b0"
    page_range: [236, 236]
    quote: "They consider instruments to be undervalued if prices are less than their estimates of fundamental value, and overvalued if prices are greater."
    edge_type: "defines"
card_hash: "0bcc950137cfcbc197af4e37cbf8f5f7e0fda42ffec806aa3af538eb8e0b59fd"
---
# Informed Traders, Fundamental Value, and Price Efficiency

## Intuition
An instrument's *fundamental value* (or intrinsic value) is the "true value" — the price everyone would agree on if all relevant information were known, analyzed correctly, and valued the same way by every trader. Nobody actually observes this value; the *market value* is just the price at which the instrument currently trades. The gap between them is **noise**. Informed traders are the people who spend effort estimating fundamental value and then trade against the noise: they buy what looks cheap and sell what looks rich, betting that price will drift back toward value.

The deep point is what this trading *does to prices*. Each time an informed trader buys an underpriced instrument, demand pushes its price up toward value; each time one sells an overpriced instrument, supply pushes its price down toward value. Aggregated across many informed traders acting on many scraps of information, the market acts as an information-aggregation engine that produces prices closer to fundamental value than any single analyst could reach alone. Prices become **informative**.

```
   fundamental value V (unobserved "truth")
        |
        |  noise = P - V
        v
   market price P  <----[informed buying when P<V]----  pushes P up toward V
                  <----[informed selling when P>V]---->  pushes P down toward V
        |
   (more informed trading => smaller noise => more informative price)
```

When markets are efficient, prices sit very close to fundamental value. Because new information arrives unpredictably, value itself jumps unpredictably — and so does price. To the statistician, price then looks like a *random walk*: future changes cannot be forecast from past prices.

**Source:** Harris (2003) ch.10 §10.1 pp.236-237

## Definition
- **Fundamental (intrinsic) value V**: the expected present value of all present and future benefits and costs of holding the instrument — the value all traders would agree on under full information and correct analysis. It depends only on information *currently* available, so it is the best estimate of the (unobservable) perfect-foresight value, not the perfect-foresight value itself.
- **Market value P**: the price at which the instrument can currently be bought or sold.
- **Noise**: the difference P − V between market price and fundamental value.
- **Informed trader**: a profit-motivated speculator who acquires and acts on information about fundamental value, deeming an instrument *undervalued* when P < V and *overvalued* when P > V, and trading accordingly. Subtypes: value traders, news traders, information-oriented technical traders, and arbitrageurs.
- **Informative price**: a price near its corresponding fundamental value (P ≈ V); a price is *completely* informative when P = V.

Only informed traders move price toward fundamental value; every other trader (utilitarian, futile, order-anticipating, bluffing) adds noise and is termed a **noise trader**.

**Source:** Harris (2003) ch.10 §10.1-§10.2 pp.236-237

## Mathematical Reasoning
Let V_t denote fundamental value and P_t the market price, with noise e_t = P_t − V_t.

1. **Trading direction.** Informed traders buy when P_t < V_t (e_t < 0) and sell when P_t > V_t (e_t > 0). Buying pressure raises P_t and selling pressure lowers it, so order flow is signed to shrink |e_t| — a negative-feedback mechanism that drives P_t toward V_t. Efficiency is the limiting state in which |e_t| is small.

2. **Unpredictability of value changes.** Because V_t already embeds all currently available information, any *predictable* component of the change ΔV_{t+1} = V_{t+1} − V_t would itself be current information and so would already be in V_t — a contradiction. Hence E[ΔV_{t+1} | information_t] = 0: fundamental-value changes are unforecastable. Value moves only on the arrival of unexpected news.

3. **Random walk of prices.** In an efficient market P_t ≈ V_t, so price changes inherit the unpredictability of value changes: E[ΔP_{t+1} | information_t] ≈ 0. A process whose increments cannot be predicted from the past is a random walk; thus efficient-market prices "wander up or down at random." Technical trading on past prices alone cannot be profitable, because there is no forecastable structure to exploit.

No worked arithmetic is performed; the argument is the comparative-static logic that informed trading is the negative-feedback channel collapsing noise, and that efficiency plus unpredictable news jointly imply unpredictable price changes.

**Source:** Harris (2003) ch.10 §10.1 pp.236-237

## Boundary Notes
- **Value ≠ perfect foresight.** Fundamental value conditions only on *currently available* information; later news can move it, so an informed trader who estimates V correctly can still lose if news pushes value the other way. Such losses tend to be short-term — in the long run prices usually revert toward value.
- **Estimation risk.** Informed traders also lose when they estimate V *poorly*: they overpay for instruments they wrongly think cheap and undersell ones they wrongly think rich. Persistent poor estimators exit the market.
- **Limits to efficiency.** Prices cannot be *completely* informative — if they were, no one could profit from informed trading and thus no one would pay to gather information (the Harris/Grossman-Stiglitz tension). Some residual noise must persist to compensate information production. Momentum-type evidence (winners keep winning, losers keep losing) suggests markets are not perfectly efficient.
- **Who counts as informed.** Order anticipators and bluffers use information, but not about *fundamental* value; their information concerns other traders' intended trades, so they are classified as uninformed (noise) traders even though they trade on "information."

**Source:** Harris (2003) ch.10 §10.1-§10.5 pp.236-247

## See Also
- [`mt-order-flow-information-content`](./mt-order-flow-information-content.md) -- how the signed order flow generated here transmits private information into price.
- [`mt-spread-equilibrium-timing-option`](./mt-spread-equilibrium-timing-option.md) -- liquidity suppliers widen spreads to recover the losses they take to informed traders.
- [`mt-order-anticipators-front-running`](./mt-order-anticipators-front-running.md) -- the contrasting "informed-about-trades-not-value" speculator type.
- [`mt-market-manipulation-bluffing`](./mt-market-manipulation-bluffing.md) -- bluffers who fake fundamental information and add noise rather than reduce it.

## Escalate to Raw When
Harris develops, beyond this card's sketch: the full taxonomy and mechanics of each informed-trader subtype (value, news, information-oriented technical, arbitrageur) in §10.2-§10.5, the formal "why prices cannot be completely informative" limits-to-efficiency argument, and the weak/semi-strong/strong efficiency forms with their evidence. Re-read Harris (2003) ch.10 (printed pp.222-247) for the noise-trader rationale, the Fischer Black "noise" framing, and the random-walk / market-efficiency-forms treatment before relying on any of those finer claims.
