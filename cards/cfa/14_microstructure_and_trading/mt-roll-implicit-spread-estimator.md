---
schema_version: "cacg.v0"
id: "mt-roll-implicit-spread-estimator"
title: "Roll's Implicit Bid-Ask Spread Estimator from Serial Covariance"
reading_id: "14_microstructure_and_trading"
summary: "Bid-ask bounce induces negative first-order serial covariance in trade-price changes; Roll (1984) inverts that covariance to recover an implied half-spread c = sqrt(-gamma_1), biased once order flow carries information."
tags: ["microstructure", "bid-ask-spread", "serial-covariance", "roll-model", "transaction-costs"]
citations:
  - source_id: "mt_hasbrouck_2007_empirical_market_microstructure"
    chunk_id: "mt_hasbrouck_2007_empirical_market_microstructure:p041:0048"
    chunk_hash: "e1af5d258054bb979ff6e00cb466ac03300bdfca80af6e0c84c5c7efc78ec30d"
    page_range: [41, 41]
    quote: "estimated from the variance and first-order autocovariance of the price"
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p060:0084"
    chunk_hash: "7fe6a9b8957b3a94696cb9130dafe20cf5564b34c8f3fe20a0e95267d328525c"
    page_range: [60, 61]
    quote: "Roll (1984) sets out an ingenious method for measuring the bid-ask spread based on transaction prices alone."
    edge_type: "supports"
card_hash: "05eef12a3c33d3e17192329711903b88f7e402af5da54614172b71a0dccc2b15"
---
# Roll's Implicit Bid-Ask Spread Estimator from Serial Covariance

## Intuition
When a dealer posts a constant bid `b = m - c` and ask `a = m + c` around an
unobserved efficient price `m`, every executed trade prints either at the ask
(a customer buy) or at the bid (a customer sell). If the efficient price itself
never moved, the printed price would simply bounce between `m - c` and `m + c`
as order direction flips. This "bid-ask bounce" is mechanical: a buy followed
by a sell prints high-then-low, manufacturing a price reversal that has nothing
to do with fundamental news. Roll's (1984) insight is that this reversal leaves
a fingerprint in the data — negative first-order serial covariance in the
trade-to-trade price changes — and that fingerprint can be inverted to recover
the half-spread `c` using transaction prices alone, with no quote data at all.

```
   ask  m+c  -----o---------o-----------o----      o = trade at ask (buy)
                   |        / \         /
   mid    m  - - - | - - - / - \ - - - / - - - -   (unobserved efficient price)
                   |      /     \     /
   bid  m-c  ------x-----x-------x----            x = trade at bid (sell)
              t:   1     2       3    4
        prints:  buy   sell    buy   ...
   change p2-p1 < 0  then  p3-p2 > 0  => reversal => Cov(dp_t, dp_{t-1}) < 0
```

The estimator is one of microstructure's most-used liquidity proxies precisely
because it asks nothing of the analyst beyond a series of transaction prices.
**Source:** Hasbrouck (2007) §3.4 pp.40-41; Foucault, Pagano & Röell (2013) ch.2 pp.60-61.

## Definition
Let the efficient (martingale) price follow a random walk `m_t = m_{t-1} + u_t`,
with `u_t` i.i.d., mean zero, variance `sigma_u^2`. The transaction price is

    p_t = m_t + q_t * c,

where `q_t = +1` for a customer buy (lifting the ask) and `q_t = -1` for a
customer sell (hitting the bid), so the quoted spread is `a_t - b_t = 2c`. Roll
assumes buys and sells are equally likely, serially independent, and independent
of the efficient-price innovations `u_t`. Define the autocovariances of price
changes `dp_t = p_t - p_{t-1}` as `gamma_k = Cov(dp_t, dp_{t-k})`. The two
identifying moments are the variance `gamma_0` and the first-order autocovariance
`gamma_1`. The implied half-spread (Roll's measure) is `c = sqrt(-gamma_1)`, and
the full implied spread is `2c`.
**Source:** Hasbrouck (2007) §3.4 pp.40-41.

## Mathematical Reasoning
Differencing the price equation gives `dp_t = u_t + c(q_t - q_{t-1})`. Taking the
variance and exploiting that `q_t^2 = 1`, that distinct `q`'s are mean-zero and
independent, and that `q` is independent of `u`:

    gamma_0 = Var(dp_t) = 2 c^2 + sigma_u^2,

since `E[(q_t - q_{t-1})^2] = E[q_t^2] - 2 E[q_t q_{t-1}] + E[q_{t-1}^2] = 2`.
The first-order autocovariance keeps only the surviving cross term in
`q_{t-1}^2`:

    gamma_1 = Cov(dp_{t-1}, dp_t) = -c^2,

and all autocovariances of order two or higher vanish. Inverting these two
moment conditions yields the closed form

    c = sqrt(-gamma_1),     sigma_u^2 = gamma_0 + 2 gamma_1.

The negative sign on `gamma_1` is the heart of the construction: only a genuine
reversal mechanism (the bounce) can drive `gamma_1 < 0`, so `-gamma_1 >= 0` and
the square root is real under the model. Two comparative statics follow
directly from the structural derivation. (i) If trade directions are positively
autocorrelated, `Corr(q_t, q_{t-1}) = rho > 0`, the bounce is attenuated and the
naive estimator is biased *downward*, `c_hat < c`. (ii) If buys carry good news
so `Corr(q_t, u_t) = rho > 0` (informed order flow), then
`gamma_1 = -c(c + rho*sigma_u)` is more negative than `-c^2`, and the naive
estimator is biased *upward*, `c_hat > c`. The estimator therefore confounds the
pure transaction-cost component `c` with information effects whenever the
independence assumptions fail.
**Source:** Hasbrouck (2007) §3.4, Exercises 4.2-4.3 pp.40-41; Foucault, Pagano & Röell (2013) ch.2 pp.60-61.

## Boundary Notes
- **When it holds:** constant spread `2c`, serially independent and balanced
  order flow, and trade direction independent of efficient-price innovations
  (purely noninformational `c`). Under these assumptions `c = sqrt(-gamma_1)`
  identifies the half-spread exactly.
- **When it breaks:** real order flow is positively autocorrelated
  (`Corr(q_t, q_{t-1}) ~ 0.34` in Hasbrouck's PCO sample), spreads vary over
  time, and quote midpoints move with the most recent trade direction
  (asymmetric information). Each violation biases `c_hat`: serial correlation in
  `q` biases it down, information in order flow biases it up.
- **Estimation pathology:** the sample `gamma_1_hat` can come out *positive* even
  when the model is correctly specified (Harris 1990), leaving `-gamma_1_hat < 0`
  and `sqrt(-gamma_1_hat)` undefined; Hasbrouck (2005) suggests a Bayesian fix.
- **Contrast:** the basic estimator lumps all transitory cost into a single `c`;
  the generalized Roll decomposition separates adverse-selection from
  inventory/processing components by modeling the midpoint response to `q`.
**Source:** Hasbrouck (2007) §3.4 & §4.5 pp.40-41; Foucault, Pagano & Röell (2013) ch.2 pp.60-61.

## See Also
- [`mt-random-walk-efficient-price`](./mt-random-walk-efficient-price.md) -- the martingale `m_t` that Roll's transaction price is built around
- [`mt-generalized-roll-spread-decomposition`](./mt-generalized-roll-spread-decomposition.md) -- relaxes independence to split the spread into adverse-selection vs noninformational parts
- [`mt-effective-cost-trade-benchmark`](./mt-effective-cost-trade-benchmark.md) -- the realized effective half-spread that Roll's measure proxies when quotes are unavailable
- [`mt-liquidity-measures-spread-depth-resiliency`](./mt-liquidity-measures-spread-depth-resiliency.md) -- situates Roll's covariance proxy among the broader spread/depth/resiliency liquidity measures
- [`fa-liquidity-measurement-and-price-impact`](../22_fund_level_arbitrage/fa-liquidity-measurement-and-price-impact.md) — cross-set: Roll / Kyle / Amihud price-impact and implied-spread estimators (reading-14 primary derivations; reading-22 ETF liquidity-measurement application).
## Escalate to Raw When
Hasbrouck §3.4 pp.40-41 derives `gamma_0` and `gamma_1` term-by-term from the
cross-product expansion (this card states the surviving terms but does not write
out every vanishing one); re-read it for the full algebra. Hasbrouck Exercises
4.2-4.3 prove the directions of the two biases — this card asserts `c_hat < c`
and `c_hat > c` but only sketches why; the source poses the full derivation.
For the order-flow-balance (`eta`) and serial-correlation (`delta`) adjustment
factors that rescale Roll's estimate, re-read Foucault, Pagano & Röell ch.2
pp.60-64, which this card only gestures at.
