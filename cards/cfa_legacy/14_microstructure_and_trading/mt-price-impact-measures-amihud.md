---
schema_version: "cacg.v0"
id: "mt-price-impact-measures-amihud"
title: "Price-Impact Liquidity Measures: Kyle-Lambda Regression and the Amihud Illiquidity Ratio"
reading_id: "14_microstructure_and_trading"
summary: "Illiquidity is gauged by price impact: lambda from regressing midquote changes on signed order imbalance, and the Amihud ratio of absolute return to volume when order-flow signing is unavailable."
tags: ["microstructure", "liquidity", "price-impact", "amihud", "kyle-lambda", "order-imbalance"]
citations:
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p059:0082"
    chunk_hash: "6d3c244590607db8209d8c57e64959ac718e875a460a1fc07a94c0f9d108a260"
    page_range: [59, 59]
    quote: "the ratio It of the absolute return for a stock"
    edge_type: "defines"
  - source_id: "mt_hasbrouck_2007_empirical_market_microstructure"
    chunk_id: "mt_hasbrouck_2007_empirical_market_microstructure:p104:0132"
    chunk_hash: "4bc79874655841d4c97cc2f83ab11df197f54588250fc7980ad2caccd2f9f304"
    page_range: [104, 104]
    quote: "Amihud (2002) suggests the illiquidity"
    edge_type: "supports"
---
# Price-Impact Liquidity Measures: Kyle-Lambda Regression and the Amihud Illiquidity Ratio

## Intuition
Illiquidity has a price-impact face: a market order shoves the price in the direction of the trade, and the harder it shoves per dollar transacted, the less liquid the market. The midprice tends to rise when buy orders arrive and fall when sell orders arrive, with the move growing in the size of the buying or selling pressure. So if we could watch the *signed* net demand over a fixed interval, the slope of price change on that net demand is a direct read on illiquidity — a steeper slope means a shallower, more impactable book.

```
   Δm (midprice change)
     ^
     |              .  slope = λ  (impact per unit imbalance)
     |           .                steeper λ  ->  more illiquid
     |        .                   flatter λ  ->  deeper market
  ---+-----.------------------> q  (signed order imbalance:
     |  .                            buy $ minus sell $)
     .
```

The catch: signing order flow (knowing which side was the aggressor) requires fine data, often inferred via the Lee-Ready algorithm. When that is unavailable, the Amihud (2002) workaround uses what *is* always observable — the day's absolute return and its trading volume — to proxy "price move per unit of trading," trading some precision for universal applicability across stocks, periods, and thin emerging markets.

**Source:** Foucault, Pagano & Roell (2013) §2.3.2 pp.56-58.

## Definition
Price-impact measure (Kyle-type lambda): over a fixed interval (a half-hour, a day), regress the midprice change on the order imbalance,

  Delta m_t = lambda * q_t + eps_t,

where Delta m_t is the midprice change and q_t is the order imbalance = dollar value of executed buy market orders minus that of sell market orders in the same interval (distinct from trading volume, the sum of both). The reciprocal 1/lambda is a measure of market depth: lower lambda means prices are less sensitive to imbalance.

Amihud (2002) illiquidity ratio: when signed flow is unavailable, use

  I_t = |r_t| / Vol_t,

the absolute return divided by the monetary trading volume over the period (averaged over days with nonzero volume). Its reciprocal Vol_t / |r_t| is the Amivest liquidity ratio; a low Amivest value signals illiquidity.

**Source:** Foucault, Pagano & Roell (2013) §2.3.2 pp.57-58 (eqs. 2.8-2.10).

## Mathematical Reasoning
The linear specification Delta m_t = lambda q_t + eps_t makes lambda the OLS slope of price change on signed imbalance; under the model lambda is positive, so net buying pressure raises the midprice and net selling lowers it. Because depth is the order flow required to move price by a fixed amount, depth and lambda are reciprocal: a deep market absorbs large q_t with small Delta m_t (small lambda), a thin market does the opposite (large lambda). The empirical regression operationalizes this by reading lambda off the data rather than from a structural model; Stoll (2000) augments it with a lagged imbalance term q_{t-1} to capture next-period reversal of transitory impact.

The Amihud ratio is a substitution under a correlation argument: trading volume and order imbalance are distinct (volume sums both sides, imbalance differences them) but tend to co-move, since high-imbalance days are often high-volume days. Replacing signed q_t with unsigned Vol_t and Delta m_t with |r_t| yields a slope-like object |Delta m_t| regressed on Vol_t, or equivalently the ratio |r_t|/Vol_t — price movement per unit of volume. Because volume sits in the denominator, the illiquidity ratio is comparatively more stable than its Amivest inverse, both being prone to extreme values when |r_t| or Vol_t is near zero. Cross-sectionally these volume-based proxies are only moderately positively correlated with high-frequency lambda estimates, with the illiquidity ratio the better of the two.

**Source:** Foucault, Pagano & Roell (2013) §2.3.2 pp.57-58; Hasbrouck (2007) §9.7 p.93.

## Boundary Notes
- Lambda requires signed market-order data; without it one infers signs (Lee-Ready) or falls back to Amihud's volume proxy. The fallback conflates imbalance with volume, valid only insofar as the two co-move.
- The half-normal relation linking |r_t| and |Vol_t| to lambda holds under bivariate-normality assumptions that Hasbrouck notes are poor for most return/volume samples, and break under time-aggregation — so the Amihud ratio is a heuristic proxy, not an unbiased lambda estimator.
- Both Amihud and Amivest ratios are sensitive to extreme values; the illiquidity ratio is steadier because volume is in the denominator.
- Distinguish from raw volume / turnover as liquidity gauges: volume rises with information arrival (high-volatility, wide-spread times), so turnover can be high precisely when trading costs are high (Fleming 2003 on Treasuries). Price-impact and Amihud measures avoid that trap by referencing the price move itself.

**Source:** Foucault, Pagano & Roell (2013) §2.3.2-2.3.3 pp.57-58; Hasbrouck (2007) §9.7 p.93.

## See Also
- [`mt-kyle-lambda-market-depth-price-impact`](./mt-kyle-lambda-market-depth-price-impact.md) -- structural origin of lambda as the inverse-depth coefficient in Kyle's model
- [`mt-liquidity-measures-spread-depth-resiliency`](./mt-liquidity-measures-spread-depth-resiliency.md) -- spread/depth/resiliency taxonomy these price-impact measures sit within
- [`mt-hasbrouck-var-trades-quotes`](./mt-hasbrouck-var-trades-quotes.md) -- VAR price-impact regression of quotes on signed trades (the Chapter 5 approach)
- [`mt-liquidity-premium-asset-pricing`](./mt-liquidity-premium-asset-pricing.md) -- Amihud ratio as a priced illiquidity factor (Acharya-Pedersen)

## Escalate to Raw When
The source asserts the lambda-regression and Amihud forms and reports Stoll's (2000) cross-sectional lambda magnitudes and Hasbrouck's correlation finding, but does not re-derive Kyle's lambda from first principles here — for the structural derivation see FPR Chapter 3 and the VAR price-impact regression in Chapter 5. For the bivariate half-normal distributional argument linking |r| and |Vol| to lambda, and the Amihud-vs-Amivest stability comparison, re-read Hasbrouck (2007) §9.7 pp.92-93. For asset-pricing use of the illiquidity ratio, see Acharya & Pedersen (2005) as cited there.
