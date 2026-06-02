---
schema_version: "cacg.v0"
id: "be-prospect-theory-three-characteristic-pricing"
title: "Prospect-Theory Three-Characteristic Pricing"
reading_id: "10_behavioral_finance"
summary: "In an economy of narrowly-framing prospect-theory investors, a stock's required return depends on three characteristics of its own return distribution: volatility (loss aversion raises it), skewness (probability weighting lowers it), and prior gain overhang (diminishing sensitivity raises it)."
tags: ["behavioral-finance", "prospect-theory", "asset-pricing", "narrow-framing"]
citations:
  - source_id: "bf_barberis_jin_wang_2021_pt_anomalies"
    chunk_id: "bf_barberis_jin_wang_2021_pt_anomalies:p002:0002"
    chunk_hash: "7ed0f7e9f963c849ac5cbac781b5dbe963aca72620dc92dc79f72d2310b6ec67"
    page_range: [2, 3]
    quote: "the price of an asset will depend in part on three asset"
    edge_type: "defines"
  - source_id: "bf_barberis_jin_wang_2021_pt_anomalies"
    chunk_id: "bf_barberis_jin_wang_2021_pt_anomalies:p010:0012"
    chunk_hash: "2e5814c345a506a59e5bf1bf184c2b0ab9bf059b3cb9858af89fabbc65e5420e"
    page_range: [11, 11]
    quote: "they are loss-averse, they dislike assets with volatile returns; all else equal, they require a higher average return on these"
    edge_type: "supports"
  - source_id: "bf_barberis_jin_wang_2021_pt_anomalies"
    chunk_id: "bf_barberis_jin_wang_2021_pt_anomalies:p012:0014"
    chunk_hash: "4db0e27143c48c34860c97d35a90456fe3946a57ac21a4a902ac869d48f105b5"
    page_range: [12, 12]
    quote: "it captures both prospect theory and narrow framing"
    edge_type: "supports"
card_hash: "f97bc617068a05cfc3ae592336081b096ee1d5af6480b07bda4425b51b256b34"
---
# Prospect-Theory Three-Characteristic Pricing

## Intuition

Under mean-variance preferences, expected returns are described by the CAPM and depend only on beta. The Barberis-Jin-Wang model asks instead what determines average returns when investors evaluate each stock through the lens of prospect theory rather than through its contribution to portfolio variance. Because such investors engage in *narrow framing* -- they evaluate a stock to some extent in isolation, separately from their other holdings -- the relevant object is the distribution of gains and losses on that one stock, not the joint distribution of the whole portfolio.
**Source:** Barberis, Jin & Wang (2021) §II.A pp.10-11.

Working through prospect theory's elements on a single stock's own return distribution isolates exactly three characteristics that the required return responds to. Loss aversion makes volatile stocks unappealing, so investors charge a *higher* average return on them. Probability weighting overweights the tails of the distribution the investor is thinking about, so positively skewed stocks are attractive lottery tickets and command a *lower* average return. Diminishing sensitivity (concavity over gains, convexity over losses) means a stock currently trading at a gain places the investor in the risk-averse concave region, so he demands a *higher* return; a stock at a paper loss places him in the risk-seeking convex region, so he accepts a *lower* return.
**Source:** Barberis, Jin & Wang (2021) §II.A pp.10-11.

The three forces frequently push in opposite directions, which is why a verbal argument cannot settle prospect theory's prediction for any given anomaly -- a quantitative model that combines all three is required. The third characteristic, the prior gain or loss, is measured by the asset's *capital gain overhang* (Grinblatt and Han (2005)): the average paper gain or loss since purchase across the investors who hold the asset.
**Source:** Barberis, Jin & Wang (2021) §I-II pp.3, 10-11.

## Definition

**Narrow framing** is the empirical phenomenon whereby, when an individual considers taking on a new risk, he evaluates it to some extent in isolation, separately from his other risks, rather than merging it into the distribution of his total wealth.
**Source:** Barberis, Jin & Wang (2021) §I.B pp.9.

**The three pricing characteristics** are (i) the volatility of the asset's returns, (ii) the skewness of the asset's returns, and (iii) the asset's capital gain overhang -- the average prior gain or loss since purchase across investors holding the asset.
**Source:** Barberis, Jin & Wang (2021) §I pp.1-2.

**Capital gain overhang** `g_i` is the percentage gain or loss since purchase for the average investor in stock `i`; a stock trading above (below) the average purchase price has positive (negative) overhang.
**Source:** Barberis, Jin & Wang (2021) §II.A pp.12.

## Mathematical Reasoning

Each investor at date 0 maximizes a two-component objective: traditional mean-variance preferences plus a narrowly-framed prospect-theory term summed asset by asset,

```
  max  E(W1) - (gamma/2) Var(W1) + b * sum_i V(G_i)
  Theta
```

where `V(G_i)` is the cumulative-prospect-theory value of the gain or loss `G_i` on asset `i`, and `b` controls the weight on the prospect-theory term. The narrow-framing assumption is exactly that utility is derived from asset-level gains/losses `V(G_i)`, summed across assets, not from portfolio-level gains and losses.
**Source:** Barberis, Jin & Wang (2021) §II.A pp.11.

The gain or loss on asset `i` merges a future and a prior component, `G_i = W0*Theta_i*(R_i - R_f) + W_{-1}*Theta_{i,-1}*g_i`. The first term is the potential future gain or loss relative to the risk-free benchmark; the second is the prior gain `g_i` carried in from before date 0. Investors do not segregate prior from future gains -- they integrate them and derive utility `V(.)` from the combined position.
**Source:** Barberis, Jin & Wang (2021) §II.A pp.11-12.

The value-function ingredients map one-to-one onto the three characteristics. Loss aversion (parameter `lambda > 1`) multiplies the loss branch and, applied to a volatile distribution, raises the required return: more volatility means a larger expected loss penalty. Probability weighting `w(P)` overweights tail outcomes; for a positively skewed `P(R_i)` the attractive right tail is overweighted, raising the asset's appeal and lowering its required return. Concavity over gains / convexity over losses (governed by `alpha`) interacts with the sign of `g_i`: positive `g_i` puts the integrated position in the concave gain region (risk-averse, higher required return), negative `g_i` puts it in the convex loss region (risk-seeking, lower required return).
**Source:** Barberis, Jin & Wang (2021) §I.A, §II.A pp.3, 10-11.

```
  Characteristic        PT ingredient            Effect on required return
  ------------------    --------------------     -------------------------
  volatility  up        loss aversion (lambda)   HIGHER
  skewness    up        probability weighting    LOWER
  gain overhang up      diminishing sensitivity  HIGHER
  (sign of g_i)         (concave gains/convex)
```

## See Also

- [be-cumulative-prospect-theory-risk](./be-cumulative-prospect-theory-risk.md#mathematical-reasoning) -- the value function `v(.)`, loss aversion, and rank-dependent weighting `pi_i` underlying `V(G_i)`.
- [be-probability-weighting-inverse-s](./be-probability-weighting-inverse-s.md#intuition) -- why overweighting tails makes positively skewed stocks attractive.
- [be-bjw-anomaly-pricing-model](./be-bjw-anomaly-pricing-model.md#intuition) -- embeds these three characteristics in a full equilibrium to price 23 anomalies.
- [be-prospect-theory-asset-pricing](./be-prospect-theory-asset-pricing.md#intuition) -- the broader research program of prospect-theory cross-sectional pricing.
- [be-myopic-loss-aversion-equity-premium](./be-myopic-loss-aversion-equity-premium.md#intuition) -- narrow framing plus loss aversion applied to the aggregate equity premium.

## Escalate to Raw When

- You need the exact continuous-distribution form of `V(G_i)` as the two cumulative-prospect-theory integrals over losses and gains (equation 11, p.14).
- You need the precise definition of the gain/loss components, the risk-free benchmark choice, or the `W_{-1} approx W0` approximation (pp.11-13).
- You need the empirical magnitudes (e.g., small-cap vs large-cap volatility, skewness, overhang) used to illustrate the three characteristics (pp.3, 27).
