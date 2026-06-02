---
schema_version: "cacg.v0"
id: "be-overconfidence-self-attribution-prices"
title: "Overconfidence and Biased Self-Attribution in Prices"
reading_id: "10_behavioral_finance"
summary: "Daniel-Hirshleifer-Subrahmanyam: overconfident investors overweight private signals, causing mispricing that corrects on public news (excess volatility, reversal); adding biased self-attribution -- confidence rising on confirming public news but not falling on disconfirming news -- produces short-run momentum then long-run reversal."
tags: ["behavioral-finance", "overconfidence", "self-attribution", "momentum", "reversal"]
citations:
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p124:0188"
    chunk_hash: "2e2acaa7ae35ba4a4503920cd243561bb81d66d02274300a83f4f47501a69c9c"
    page_range: [124, 124]
    quote: "the asset will be misvalued: overvalued if the signal is good and undervalued if it is bad."
    edge_type: "defines"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p124:0188"
    chunk_hash: "2e2acaa7ae35ba4a4503920cd243561bb81d66d02274300a83f4f47501a69c9c"
    page_range: [124, 124]
    quote: "the tendency, driven by a desire to maintain a positive self-image, to give oneself credit for a good outcome but to blame a bad outcome on extraneous bad luck."
    edge_type: "supports"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p124:0188"
    chunk_hash: "2e2acaa7ae35ba4a4503920cd243561bb81d66d02274300a83f4f47501a69c9c"
    page_range: [124, 124]
    quote: "Biased updating of this kind generates both medium-term momentum and"
    edge_type: "supports"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p121:0183"
    chunk_hash: "fd023d214263db1e9ad5375a023538fc7ce195cdf897afc371bc5c32366fffac"
    page_range: [121, 121]
    quote: "the very high trading volume in financial markets"
    edge_type: "supports"
card_hash: "6ddc6fa1bc62d41b7ba19f3c2f0385b2968a45f605da5eaeaea43d27148da9b9"
---
# Overconfidence and Biased Self-Attribution in Prices

## Intuition

Overconfidence in finance means investors overestimate the precision of their own information signals (and underestimate the precision of others' signals). Daniel, Hirshleifer and Subrahmanyam show this single bias explains several aggregate facts. In the simplest version, a representative investor does private research at time 1 and obtains a signal about a firm's cash flow. Because he is overconfident, he overweights the signal and the asset becomes misvalued -- overvalued on a good signal, undervalued on a bad one. When the cash flow is finally announced and the misvaluation is corrected, there is a price reversal on average. This excess movement at time 1 followed by correction generates excess volatility, return predictability, long-term reversal, and the value premium.
**Source:** Barberis (2018) §6.2 pp.124.

The richer version adds *biased self-attribution* to generate momentum. Confidence in the private signal now varies over time, and it does so asymmetrically. If subsequent public information confirms the private signal, the investor grows *even more* confident in it; if public information contradicts the private signal, his confidence stays unchanged. This asymmetry is motivated by the desire to protect a positive self-image: credit good outcomes to one's skill, blame bad ones on luck.
**Source:** Barberis (2018) §6.2 pp.124.

The asymmetric updating produces a hump-shaped price path. Suppose the private signal is good, pushing the price up. If the public signal is also good, the investor's confidence rises and he pushes the price up *further*; if the public signal is bad, confidence is unchanged and the price barely moves. So on average the price keeps rising after the initial move -- short-run/medium-term momentum -- before the eventual cash-flow announcement pulls it back to fundamentals, producing long-run reversal.
**Source:** Barberis (2018) §6.2 pp.124, 123.

## Definition

**Overconfidence (overprecision)** is overestimating the precision of one's own judgments or information signals; in the trading context it also covers underestimating the precision of other people's signals.
**Source:** Barberis (2018) §5, §6.2 pp.121, 124.

**Biased self-attribution** is the tendency to give oneself credit for good outcomes but to blame bad outcomes on extraneous bad luck, so that confidence in a private signal rises on confirming public news but does not fall on disconfirming public news.
**Source:** Barberis (2018) §6.2 pp.124.

**Overreaction (and its correction)** is the time-1 misvaluation from overweighting the private signal, undone when the true cash flow is revealed -- the source of excess volatility and long-term reversal.
**Source:** Barberis (2018) §6.2 pp.124.

**Momentum** here is the continuation of the initial price move produced by asymmetric confidence updating before the final correction.
**Source:** Barberis (2018) §6.2 pp.124.

## Mathematical Reasoning

The base model has three dates `t = 0, 1, 2`, a risk-free asset, and a risky asset claiming a cash flow at `t = 2`, with a risk-neutral representative investor. At `t = 1` he observes a private signal of the cash flow. Overconfidence is the assumption that he *overestimates the signal's precision*, so the time-1 price overshoots: it lies above fundamentals on a good signal and below on a bad one. At `t = 2` the cash flow is announced and the price reverts, so the time-1 to time-2 return is negatively correlated with the time-0 to time-1 return -- reversal and excess volatility.
**Source:** Barberis (2018) §6.2 pp.124.

The extended model adds dates: private signal at `t = 1`, public signal at `t = 2`, cash flow at `t = 3`. Confidence in the private signal is updated asymmetrically -- increased when the public signal at `t = 2` agrees with the private signal, left unchanged when it disagrees. Tracing the good-private-signal case:

```
  t=1  good private signal           -> price rises (overshoot)
  t=2  public signal good            -> confidence UP   -> price rises further
       public signal bad             -> confidence SAME -> price ~ flat
       => on average, continued rise  -> MOMENTUM
  t=3  cash flow announced           -> correction      -> long-run REVERSAL
```

Because the upward push at `t = 2` happens only in the confirming case but no symmetric downward push happens in the disconfirming case, the average post-event drift is positive, yielding momentum; the terminal correction yields reversal.
**Source:** Barberis (2018) §6.2 pp.124.

Overconfidence also underlies the *volume* facts: a model where investors overweight their own signals and dismiss others' generates substantial disagreement and hence heavy trading, which is why overconfidence became central to behavioral finance.
**Source:** Barberis (2018) §5, §5.1 pp.121.

## See Also

- [be-overconfidence-disagreement-short-sale](./be-overconfidence-disagreement-short-sale.md#intuition) -- the volume/overpricing side of the overconfidence framework, with a short-sale constraint.
- [be-three-frameworks-behavioral-asset-pricing](./be-three-frameworks-behavioral-asset-pricing.md#intuition) -- overconfidence as the second framework.
- [be-asset-pricing-anomalies-catalog](./be-asset-pricing-anomalies-catalog.md#intuition) -- the momentum, reversal, and volatility facts this model targets.
- [be-investor-overreaction](./be-investor-overreaction.md#intuition) -- overreaction as a cross-cutting mispricing mechanism.
- [be-confirmation-prior-biased-inference](./be-confirmation-prior-biased-inference.md#intuition) -- a related asymmetric belief-updating bias.

## Escalate to Raw When

- You need the contrasting Barberis-Shleifer-Vishny (1998) regime-switching and Rabin (2002) law-of-small-numbers models of under- and over-reaction (pp.122-123).
- You need the empirical tests linking measured overconfidence to trading behavior (Grinblatt-Keloharju; Barber-Odean) (pp.114-115).
- You need the precise functional forms and parameters of the Daniel-Hirshleifer-Subrahmanyam model from the original papers (cited but not reproduced).
