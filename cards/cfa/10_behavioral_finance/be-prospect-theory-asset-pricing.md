---
schema_version: "cacg.v0"
id: "be-prospect-theory-asset-pricing"
title: "Prospect-Theory Preferences in Asset Pricing"
reading_id: "10_behavioral_finance"
summary: "Gain-loss (prospect-theory) utility in equilibrium (Benartzi-Thaler, Barberis-Huang-Santos): investors derive utility from annual gains and losses in financial wealth; loss aversion delivers a high equity premium, and time-varying effective loss aversion (high after losses, low after gains) delivers excess volatility and return predictability."
tags: ["behavioral-finance", "prospect-theory", "asset-pricing", "equity-premium", "loss-aversion"]
citations:
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p152:0236"
    chunk_hash: "b91ee5c745c339706e7fd813b30865e26aa113a9de40cffcfc4ff0fb3f96ba48"
    page_range: [152, 152]
    quote: "the stock market earns a high average return so that it can be competitive with the bond market in the eyes of prospect theory investors"
    edge_type: "supports"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p153:0237"
    chunk_hash: "ba6b4760aa619daf726b5fb41dcb3723856ebecbac6c7b7ea51f6a645e68e725"
    page_range: [153, 153]
    quote: "this term captures the idea that the investor also derives utility from annual gains and losses in her financial wealth"
    edge_type: "defines"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p153:0237"
    chunk_hash: "ba6b4760aa619daf726b5fb41dcb3723856ebecbac6c7b7ea51f6a645e68e725"
    page_range: [153, 153]
    quote: "They find that, for values of loss aversion λ drawn from experimental studies, and for values of b0 that put substantial weight on the gain-loss utility term, the equilibrium equity premium is large – as high as 6% per year for high values of b0."
    edge_type: "supports"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p153:0238"
    chunk_hash: "b91209ccc8c1aefdf16712839834d56244f71c007ae42ff5cbf88c86efc60c6a"
    page_range: [154, 154]
    quote: "if she has experienced gains in recent years, she is less loss averse going forward, while if she has experienced losses, she is more loss averse"
    edge_type: "supports"
card_hash: "da5cb6d4de70b799d4496d7b40326ebe9ead80308d933bfdd5d8fe016e952e4b"
---
# Prospect-Theory Preferences in Asset Pricing

## Intuition

Most asset-pricing models assume investors evaluate risk via Expected Utility over consumption, with a function that is increasing and concave. Prospect-theory asset pricing instead lets investors derive utility from *gains and losses in financial wealth* relative to a reference point, with loss aversion. Benartzi and Thaler first applied this to the equity premium puzzle: if people evaluate their stock investments over an annual horizon and are loss averse, the high volatility of annual stock returns makes stocks feel painful, so a high average return is required to compensate -- the equity premium becomes a natural consequence of loss aversion plus annual evaluation rather than a puzzle.
**Source:** Barberis (2018) §7.4 pp.152.

Barberis, Huang and Santos build this into a full equilibrium. The representative investor's objective adds a gain-loss term over annual changes in financial wealth on top of standard consumption utility. The investor recognizes that holding stocks exposes her to large annual fluctuations in financial wealth which, given loss aversion, are unpleasant; as compensation she demands a high average stock return. Calibrated to experimental loss-aversion values, the model produces an equity premium as high as 6% per year.
**Source:** Barberis (2018) §7.4 pp.153.

The model's second move generates *excess volatility and predictability* by making effective loss aversion time-varying. Motivated by Thaler-Johnson's "house money" evidence, the degree of loss aversion `lambda` is not constant: after recent gains the investor feels she is "playing with house money," becomes less loss averse, willing to pay even more for stocks (pushing prices up and amplifying volatility); after losses she becomes more loss averse, pushing prices down and demanding higher future returns. This countercyclical effective risk aversion delivers both excess volatility and time-series predictability.
**Source:** Barberis (2018) §7.4 pp.153, 154.

## Definition

**Gain-loss utility (prospect-theory term)** is utility derived from annual gains and losses `X_{t+1}` in the investor's financial wealth, on top of consumption utility, with a loss-averse (kinked) value function.
**Source:** Barberis (2018) §7.4 pp.153.

**Effective loss aversion `lambda(z_t)`** is the investor's degree of loss aversion at time `t`, allowed to vary with the cumulated past gains/losses `z_t`: higher `z_t` (more prior losses) means a higher `lambda`.
**Source:** Barberis (2018) §7.4 pp.154.

**Equity premium from loss aversion** is the high required average return on stocks that compensates a loss-averse investor for the unpleasant annual wealth fluctuations stocks generate.
**Source:** Barberis (2018) §7.4 pp.152, 153.

**Narrow framing / annual evaluation** is the assumption that the investor tracks gains and losses in financial wealth on an annual basis (e.g., tax-year or brokerage-statement horizon) rather than over a lifetime.
**Source:** Barberis (2018) §7.4 pp.152, 153.

## Mathematical Reasoning

The Barberis-Huang-Santos representative investor maximizes a two-component objective combining lifetime consumption utility with an annual gain-loss term,

```
  E sum_{t=0}^{inf} [ rho^t * C_t^{1-gamma}/(1-gamma) + b_0 * Cbar_t^{-gamma} * rho^{t+1} * v(X_{t+1}) ]
```

where `X_{t+1}` is the gain or loss in financial wealth, `b_0` is the weight on the gain-loss term, and `Cbar_t^{-gamma}` is a consumption scaling that keeps the two components similarly important as wealth grows.
**Source:** Barberis (2018) §7.4 pp.152, 153.

The gain or loss is financial wealth at `t+1` minus financial wealth at `t` scaled up by the gross risk-free rate: `X_{t+1} = S_t (R_{t+1} - R_{f,t})`, where `S_t` is the value of risky holdings. The value function is piecewise-linear (diminishing sensitivity and probability weighting are dropped for tractability), `v(X) = X` for `X >= 0` and `v(X) = lambda X` for `X < 0`, with loss aversion `lambda > 1`.
**Source:** Barberis (2018) §7.4 pp.153.

To generate fluctuations, loss aversion is made state-dependent:

```
  v(X, z_t) = X            for X >= 0
            = lambda(z_t) X for X < 0,
  with  lambda(z_t) = lambda + k (z_t - 1),  k > 0,
```

where `z_t` summarizes past gains and losses (higher `z_t` = larger prior losses). After good cash-flow news the investor is in a "prior gains" state, `lambda(z_t)` falls, she pays an even higher price (excess volatility) and accepts a lower forward return (predictability); after bad news the opposite holds.
**Source:** Barberis (2018) §7.4 pp.154.

The source notes loss aversion and risk-aversion-after-losses are not contradictory: facing the *possibility* of a loss, a person may take risk to avoid it (Kahneman-Tversky), while a person who has *already realized* a loss becomes more cautious (Barberis-Huang-Santos, Imas).
**Source:** Barberis (2018) §7.4 pp.155.

## See Also

- [be-cumulative-prospect-theory-risk](./be-cumulative-prospect-theory-risk.md#mathematical-reasoning) -- the value function, loss aversion, and weighting underlying gain-loss utility.
- [be-myopic-loss-aversion-equity-premium](./be-myopic-loss-aversion-equity-premium.md#intuition) -- the Benartzi-Thaler annual-evaluation argument in depth.
- [be-prospect-theory-three-characteristic-pricing](./be-prospect-theory-three-characteristic-pricing.md#intuition) -- the cross-sectional counterpart (volatility, skewness, overhang).
- [be-three-frameworks-behavioral-asset-pricing](./be-three-frameworks-behavioral-asset-pricing.md#intuition) -- gain-loss utility as the average-returns framework.
- [be-asset-pricing-anomalies-catalog](./be-asset-pricing-anomalies-catalog.md#intuition) -- the equity-premium and excess-volatility facts targeted.

## Escalate to Raw When

- You need the De Giorgi-Legg treatment adding probability weighting to the aggregate market (negative skew, higher premium) (p.146).
- You need the static and dynamic cross-sectional prospect-theory models (Sections 7.2-7.3, pp.137-143).
- You need the mental-accounting and reference-point-determination discussion motivating utility over financial-wealth gains (pp.147-148).
