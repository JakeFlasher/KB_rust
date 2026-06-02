---
schema_version: "cacg.v0"
id: "be-extrapolative-beliefs-asset-prices"
title: "Extrapolative Beliefs and Asset Prices"
reading_id: "10_behavioral_finance"
summary: "In an equilibrium with extrapolators (who expect future price changes as a weighted average of recent past changes) and boundedly-rational fundamental traders, prices are anchored to fundamentals plus an extrapolation term, generating bubbles, excess volatility, and time-series predictability."
tags: ["behavioral-finance", "extrapolation", "asset-pricing", "bubbles", "noise-traders"]
citations:
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p102:0152"
    chunk_hash: "304dac5016490991166878ce679f3d5288fcceceb08d2080072e52d777a193e4"
    page_range: [102, 102]
    quote: "expectation of the future return of an asset, asset class, or fund is a weighted average of the past returns of the"
    edge_type: "defines"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p103:0154"
    chunk_hash: "2f53b4b3083c9e0ce6c2f30bd2e678869202a74e87dc4825e55fbf0e767ffaf1"
    page_range: [103, 103]
    quote: "a weighted average of past price changes that puts more weight on the more recent past"
    edge_type: "supports"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p104:0155"
    chunk_hash: "731b939f435403036bc967f9463faaad24929d6110b4d8dd1844f63e40cb3ab4"
    page_range: [104, 104]
    quote: "if past price changes have been strongly positive, extrapolators become more bullish about the future price change and therefore increase their demand for the risky asset, pushing its price higher"
    edge_type: "supports"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p102:0153"
    chunk_hash: "651c8dc27d67bc6ba75bd8a068174b306c44f8adde91bf49cb815f8d79544a32"
    page_range: [102, 102]
    quote: "the average belief of the surveyed investors about the future stock market return is a positive function of recent past stock market returns"
    edge_type: "supports"
card_hash: "e691b6928f367b6c5e70c9822ab5cea37160407bee1c25001c14c21bf01839dc"
---
# Extrapolative Beliefs and Asset Prices

## Intuition

One of the most useful ideas in behavioral finance is that people hold *extrapolative* beliefs: their estimate of a quantity's future value is a positive function of its recent past values. Applied to returns, investors expect an asset whose price recently rose to keep rising. A single, simple assumption -- return extrapolation -- can explain a strikingly wide range of facts: medium-term momentum, long-term reversal and the value premium in the cross-section; excess volatility and time-series predictability in aggregate asset classes; and the formation and collapse of bubbles. Because it applies naturally in any asset class, it also explains why these patterns appear well beyond the stock market.
**Source:** Barberis (2018) §4-4.1 pp.101.

Survey data give direct evidence. When investors are asked to forecast the stock market's return over the next six months or year, the average forecast is a positive function of recent past returns -- exactly extrapolation. The same surveys reveal *over*-extrapolation: the average forecast is *negatively* related to the subsequently realized return, so the extrapolative beliefs are systematically incorrect. People become bullish precisely when future returns will be low.
**Source:** Barberis (2018) §4.1 pp.101.

In equilibrium, extrapolators trade against "fundamental traders" (arbitrageurs). After a large positive cash-flow shock, the price rises; extrapolators then extrapolate that rise, become bullish, and push the price above fundamentals, creating a bubble that later reverses when fundamental traders pull it back. This same overshoot-and-correction dynamic generates excess volatility (prices move more than fundamentals) and predictability (overshooting prices forecast low subsequent returns).
**Source:** Barberis (2018) §4.1 pp.104.

## Definition

**Return extrapolation** is the idea that an investor's expectation of an asset's future return (or future price change) is a weighted average of the asset's past returns, with positive weights that are larger for more recent past returns.
**Source:** Barberis (2018) §4.1 pp.101, 103.

**Extrapolators** are the investor type whose belief about the next period's price change `X_t` is the recency-weighted average of past price changes; their share demand rises with `X_t`.
**Source:** Barberis (2018) §4.1 pp.103, 104.

**Fundamental traders** are the boundedly-rational arbitrageurs whose demand is higher the lower the price is relative to the expected cash flow `D_t`, anchoring the price toward a sensible present value.
**Source:** Barberis (2018) §4.1 pp.104.

**Over-extrapolation** is the empirical finding that investors' extrapolative forecasts are negatively related to subsequently realized returns -- they are bullish exactly when future returns are low.
**Source:** Barberis (2018) §4.1 pp.101.

## Mathematical Reasoning

The economy has dates `t = 0,...,T`, a risk-free asset with zero net return, and a risky asset in fixed supply `Q` paying a single terminal cash flow `D_T = D_0 + e_1 + ... + e_T` with `e_t ~ N(0, sigma_e^2)` i.i.d. Extrapolators' belief about the next price change is

```
  E^e_t(P_{t+1} - P_t) = X_t = (1 - theta) * sum_{k=1}^{t-1} theta^{k-1} (P_{t-k} - P_{t-k-1}) + theta^{t-1} X_1
```

a geometric, recency-weighted average of past price changes with decay parameter `theta` in `(0,1)`.
**Source:** Barberis (2018) §4.1 pp.103.

With CARA risk aversion `gamma` and Normal beliefs of variance `sigma_e^2`, extrapolators' per-capita share demand is `N^e_t = X_t / (gamma * sigma_e^2)` -- bullishness scaled by risk aversion and perceived risk. Boundedly-rational fundamental traders (who do not understand extrapolator demand) have demand `N^f_t = [D_t - (T-t-1) gamma sigma_e^2 Q - P_t] / (gamma sigma_e^2)`, higher the cheaper the price relative to `D_t`.
**Source:** Barberis (2018) §4.1 pp.103, 104.

Substituting both into market clearing `mu^e N^e_t + mu^f N^f_t = Q` with population shares `mu^e, mu^f = 1 - mu^e` gives the equilibrium price

```
  P_t = D_t + (mu^e/mu^f) X_t - gamma sigma_e^2 Q (T - t - 1 + 1/mu^f),   t = 1,...,T-1.
```

The first term anchors price to the expected final cash flow; the second is the extrapolation term, increasing in `X_t` (strong recent gains push the price up via bullish extrapolators); the third is a risk discount. The `X_t` term is the engine of bubbles, excess volatility, and predictability.
**Source:** Barberis (2018) §4.1 pp.104.

## See Also

- [be-noise-trader-equilibrium](./be-noise-trader-equilibrium.md#mathematical-reasoning) -- the two-type (sentiment trader vs. arbitrageur) equilibrium structure this model instantiates.
- [be-three-frameworks-behavioral-asset-pricing](./be-three-frameworks-behavioral-asset-pricing.md#intuition) -- extrapolation as the first of the three frameworks.
- [be-asset-pricing-anomalies-catalog](./be-asset-pricing-anomalies-catalog.md#intuition) -- the bubbles, excess-volatility, and predictability facts this model targets.
- [be-extrapolation-from-recent-data](./be-extrapolation-from-recent-data.md#intuition) -- the psychological roots of extrapolative belief formation.
- [be-diagnostic-expectations](./be-diagnostic-expectations.md#intuition) -- a related overreaction mechanism for forming forward-looking beliefs.

## Escalate to Raw When

- You need the simulated price path after the `e_2 = 6` two-standard-deviation shock and the parameter values in Table 2 (pp.104-105).
- You need the derivation of equations (3) and (4) from the CARA optimization (Appendix A, p.162).
- You need the treatment of extrapolation of fundamentals, experience effects, or the survey-data sources (Sections 4.2-4.4, pp.103-110).
