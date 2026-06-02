---
schema_version: "cacg.v0"
id: "be-destabilizing-arbitrage-positive-feedback"
title: "Destabilizing Arbitrage With Positive-Feedback Traders"
reading_id: "10_behavioral_finance"
summary: "With positive-feedback (trend-chasing) traders present, informed arbitrageurs who anticipate future noise demand rationally buy early to ride and amplify a price bubble rather than correct it, so adding arbitrageurs pushes prices further from fundamentals (DSSW 1990)."
tags: ["behavioral-finance", "positive-feedback", "arbitrage", "bubbles", "noise-trader-risk"]
citations:
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p164:0185"
    chunk_hash: "a99cca5806cc77130203e9890989089843ec535b24096b80ae2e8832321b8152"
    page_range: [165, 165]
    quote: "When arbitrageurs receive good news, they recognize that the initial price increase will stimulate buying by positive feedback traders tomorrow."
    edge_type: "supports"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p163:0184"
    chunk_hash: "cb806cf8956744b1516d01a7aa44d139cdf16e98453ce1e3e6efe03ee9410ff7"
    page_range: [164, 164]
    quote: "It can result from extrapolative expectations about prices, or trend chasing"
    edge_type: "defines"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p175:0197"
    chunk_hash: "c2c1437646f43ad29db3cdb6a04624208b0a66e5da58c424e63c528674ba2738"
    page_range: [176, 176]
    quote: "the way arbitrageurs make money in this model is through short-term trading: they buy in period"
    edge_type: "supports"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p172:0194"
    chunk_hash: "fde555b6f03ebbf2051b89ae07d23ba5576174159d5118bb49bb7f86997f5d8c"
    page_range: [172, 172]
    quote: "the addition of arbitrageurs can push prices away from"
    edge_type: "supports"
card_hash: "5d0106aedef1e7cfc6b378bcf62afb28d3a7caf31e7e11c0268b2e55a3a75a29"
---
# Destabilizing Arbitrage With Positive-Feedback Traders

## Intuition

The benign view of arbitrage running through most of behavioral finance is that arbitrageurs lean against noise-trader demand and so stabilize prices, even if imperfectly. De Long, Shleifer, Summers, and Waldmann (1990) identify an exception: when **positive-feedback** traders — investors who buy after prices rise and sell after prices fall — are present, rational arbitrageurs who can ANTICIPATE future noise demand will not lean against an emerging bubble. They will instead jump in early, push the price up today, and thereby trigger tomorrow's trend-chasing buying, riding the bubble up and selling out near the top. The crucial premise is anticipation: arbitrageurs here have superior information about *future* noise-trader demand, which the standard model assumes away. **Source:** Shleifer (2000) Ch.6 pp.156-156.

This inverts the usual stabilizing role and CONTRASTS sharply with the two-model mispricing taxonomy of [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#intuition), where arbitrageurs trade against both overreaction and underreaction. Here informed trading *causes* the overshoot: part of the price rise is rational anticipation, part is the positive feedbackers' mechanical reaction to the arbitrageurs' own trades. Shleifer connects this to a literary tradition — Soros's "betting on future crowd behavior" rather than fundamentals, Bagehot, Kindleberger's insiders who "destabilize by driving the price up and up." **Source:** Shleifer (2000) Ch.6 pp.156-157.

The phenomenon is built on the noise-trader-risk apparatus of [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#intuition): noise (here positive-feedback) demand is the source of mispricing, but unlike the static case the demand is *predictable in direction*, which is exactly what makes anticipatory arbitrage profitable and destabilizing. **Source:** Shleifer (2000) Ch.6 pp.156-156.

## Definition

**Positive-feedback trader** is an investor whose demand responds to *past* price changes: they buy securities after prices rise and sell after prices fall (trend-chasing, extrapolative expectations, stop-loss orders, portfolio insurance, margin-call liquidation). **Source:** Shleifer (2000) Ch.6 pp.154-155.

**Destabilizing rational arbitrage** is the equilibrium pattern in which adding informed, profit-maximizing arbitrageurs moves prices *further* from fundamentals, because the arbitrageurs trade in anticipation of, and so amplify, positive-feedback demand. Shleifer ASSERTS this requires arbitrageurs to have superior information about future noise demand — the one assumption that overturns the book's general benign view of arbitrage. **Source:** Shleifer (2000) Ch.6 pp.156-156.

**Anticipatory trade** is the arbitrageurs' early purchase made because they recognize that today's price increase will stimulate positive-feedback buying tomorrow; the trade is profitable on average yet drives the period-1 price above its fundamental value. **Source:** Shleifer (2000) Ch.6 pp.156-156.

## Mathematical Reasoning

The model has four periods (0,1,2,3) and two assets (cash, stock). Stock liquidates in period 3 paying `Phi + theta`, where `theta ~ N(0, sigma_theta^2)` is unpredictable and `Phi in {phi, 0, -phi}` is a fundamental shock made public in period 2, with a signal in period 1. Three investor types coexist: positive-feedback traders (measure 1), arbitrageurs (measure `mu`), and passive investors (measure `1 - mu`). **Source:** Shleifer (2000) Ch.6 pp.158-158.

Positive-feedback demand reacts to the *past* price change: `D_2^f = beta(p_1 - p_0) = beta·p_1` with `p_0 = 0`, where `beta > 0` is the positive-feedback coefficient. They place market orders today in response to yesterday's price move and cannot respond instantaneously. Arbitrageurs maximize mean-variance utility with risk aversion `gamma`, giving period-2 demand `D_2^a = alpha(Phi - p_2)` with `alpha = 1/(2·gamma·sigma_theta^2)`; passive investors have `D_2^i = alpha(Phi - p_2)`. Stability requires `alpha > beta`. **Source:** Shleifer (2000) Ch.6 pp.160-161.

With a **noiseless** period-1 signal (`epsilon = Phi`), arbitrage makes period-1 prices rise one-for-one with expected period-2 prices, and the equilibrium gives `p_1 = p_2 = (alpha·phi)/(alpha - beta)` if `mu > 0`, versus `p_1 = 0, p_2 = phi` if `mu = 0`. So if `beta > 0`, the price with arbitrageurs is strictly further from fundamentals in all periods than without them. **Source:** Shleifer (2000) Ch.6 pp.163-163.

```
   noisy-signal price path (Fig. 6.2 schematic)
   p2 = (beta/alpha)p1 + phi
   price
        |              * (2) arb sells/shorts; feedback buys
        |             / \
        |   (1)*    /     \
        |     /  \ /        \
   phi  |    /    o----------o (3) cover at fundamental phi
        |   /   (no-arb path: dampened hump through phi)
     0  *--/------------------------> period  0   1   2   3
        * = price WITH arbitrageurs   o = price WITHOUT
```

With an **imperfectly informative** (noisy) signal the round trip carries risk, period-1 demand is `D_1^a = [(p_2a + p_2b) - 2p_1] / [gamma·(p_2a - p_2b)^2]`, and the period-1 price solves equation (6.19). Arbitrageurs bet on `Phi` being high, drive `p_1` above zero, which raises positive-feedback demand in period 2 in both states; in period 2 they UNLOAD and short while feedback buying keeps the price above fundamentals, then COVER in period 3. The money is made by short-term trading, and the introduction of arbitrageurs always destabilizes period-2 prices. **Source:** Shleifer (2000) Ch.6 pp.167-167.

## See Also

- [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#intuition) — the noise-trader-risk base on which predictable (positive-feedback) demand is layered.
- [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#intuition) — the contrasting taxonomy where arbitrage corrects rather than amplifies mispricing.
- [`be-momentum-anomaly.md`](./be-momentum-anomaly.md#intuition) — return continuation from underreaction, distinct from the bubble continuation positive feedback produces.
- [`be-fund-flow-pressure.md`](./be-fund-flow-pressure.md#intuition) — the other channel (Ch.4) where arbitrage fails to stabilize, via forced liquidation rather than anticipatory amplification.

## Escalate to Raw When

- The full comparative-statics on `mu` (number of arbitrageurs) and the discontinuity between the noiseless and noisy-signal solutions require the original DSSW (1990) algebra, not the card's symbolic summary. **Source:** Shleifer (2000) Ch.6 pp.163-167.
- A historical bubble (1960s conglomerates, 1970s REITs, 1998 Internet stocks) must be mapped onto the model's period structure with its specific narrative detail. **Source:** Shleifer (2000) Ch.6 pp.156-158.
- The period-1 demand derivation `D_1^a` and certainty-equivalent wealth expressions (6.16)–(6.18) are needed for the noisy-signal equilibrium price (6.19). **Source:** Shleifer (2000) Ch.6 pp.165-167.
