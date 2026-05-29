---
schema_version: "cacg.v0"
id: "be-noise-trader-equilibrium"
title: "Noise-Trader Risk Equilibrium"
reading_id: "10_behavioral_finance"
summary: "Noise-Trader Risk Equilibrium: framing the noise-trader-risk equilibrium that pairs limits-of-arbitrage with sentiment-driven demand — DeLong-Shleifer-Summers-Waldmann clientele-weighted price formation, partially price-inelastic noise-trader demand, and the bounded-deviation equilibrium that allows non-zero sustained mispricing under rational arbitrage"
tags: ["behavioral-finance", "noise-traders", "mispricing"]
citations:
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p037:0039"
    chunk_hash: "bb3b7e3fd9ba5567928c652b9224321819f67d983ece824b9fad0f86fa699378"
    page_range: [37, 38]
    quote: "must be borne by any arbitrageur with a short time horizon"
    edge_type: "supports"
  - source_id: "econ_hart_mascolell_2013_simple_adaptive_strategies"
    chunk_id: "econ_hart_mascolell_2013_simple_adaptive_strategies:p022:0024"
    chunk_hash: "5032f88e4e140497d7e576db9520a61ecac25d2c07149a1c98500f136f922c8c"
    page_range: [22, 22]
    quote: "Switch next period to a different action with a probability that is proportional to the regret"
    edge_type: "supports"
card_hash: "75902330346e90d1850bb8d0e6a6b8bbc1106aa7ef325f73a6be44903848a1ee"
---
# Noise-Trader Risk Equilibrium

## Intuition

A **noise trader** is an investor whose demand depends on sentiment, narrative, or non-fundamental signals rather than on a coherent forecast of fundamental value. Shleifer ASSERTS that noise traders are not a marginal-share curiosity but a structural participant: their demand is partially price-inelastic and persists across rational-arbitrage rebalancing, so the equilibrium price reflects a clientele-weighted average of sentiment-driven and fundamentally-anchored demand. **Source:** Shleifer (2000) Ch.2 pp.28-35.

The market clears with the noise-trader presence by forcing a wedge between price `P` and fundamental value `V`. Rational arbitrageurs trade against the wedge but cannot fully eliminate it, because (i) the noise-trader demand is partially price-inelastic so additional supply does not fully offset it, and (ii) arbitrage capital is bounded — see [`be-limits-of-arbitrage.md`](./be-limits-of-arbitrage.md#mathematical-reasoning). The investor-clientele-tiers diagram below shows the segmentation. **Source:** Shleifer (2000) Ch.2 pp.35-45.

```
<!-- primitive: investor-clientele-tiers source: _diagram_primitives.md -->
   +---------------------+---------+-----------+-------------+-------------+
   | Clientele tier      | Horizon | Risk tol. | Cap. constr.| Price role  |
   +---------------------+---------+-----------+-------------+-------------+
   | Noise traders       | Short   | High      | Low         | Distorting  |
   |  (retail, narrative-|         | (sentim.- |  (margin    |  push: drive|
   |   driven, fad)      |         |  driven)  |   sensitive)|  P off V    |
   +---------------------+---------+-----------+-------------+-------------+
   | Arbitrageurs        | Medium  | Bounded   | Medium      | Restoring   |
   |  (prop., hedge fund,|         | (drawdown |  (margin +  |  pull: push |
   |   stat. arb.)       |         |  bounded) |   redempt.) |  P toward V |
   +---------------------+---------+-----------+-------------+-------------+
   | Fundamental holders | Long    | Patient   | High        | Anchoring   |
   |  (pension, endow.,  |         | (long     |  (rebal.    |  weight: V  |
   |   value-style mgr)  |         |  horizon) |   capacity) |  reference  |
   +---------------------+---------+-----------+-------------+-------------+

   Equilibrium P forms as a clientele-weighted average; the mispricing
   (P - V) widens when noise-trader weight rises and arbitrageurs hit
   capital / agency constraints. Cards MUST NOT overlay numerical
   weights / horizons / risk-tolerance magnitudes.
```

Shleifer's framing EXPLAINS that the noise-trader presence is not a transient anomaly but a stable feature of markets where retail flow, narrative-driven momentum, and clientele-segmentation effects are present. The DeLong-Shleifer-Summers-Waldmann model derives an equilibrium in which noise-traders earn positive excess returns on average because the same risk-aversion that limits arbitrage capacity allows noise-traders to bear additional risk and capture a compensating premium — a structural rather than a calibration result. **Source:** Shleifer (2000) Ch.2 pp.42-52.

## Definition

The **noise-trader risk** is the variance contribution to next-period price from sentiment-driven demand shifts, distinct from variance from fundamental shocks. Shleifer DOCUMENTS that noise-trader risk is empirically large and persistent: the variance of price changes around large index-rebalancing dates, around earnings-announcement clusters with concentrated retail participation, and around narrative-driven episodes (sector rotations, crypto cycles, meme rallies) consistently exceeds the fundamental-volatility benchmark by a wide structural margin. **Source:** Shleifer (2000) Ch.2 pp.35-42.

The **clientele weight** `w` is the noise-trader share of aggregate demand at the equilibrium clearing price. The clientele weight is endogenous to the arbitrage capacity available at the date: when arbitrage capital is plentiful (after a long no-shock period), `w` is small; when arbitrage capital is depleted (after a sequence of mark-to-market losses on prior trades), `w` rises and the equilibrium price drifts further from `V`. Shleifer EXPLAINS that the clientele weight is therefore not a market parameter but a state-contingent equilibrium quantity. **Source:** Shleifer (2000) Ch.2 pp.42-48.

The **fundamental anchor** `V` is the rational-expectations price under full information and unlimited arbitrage. In the Shleifer framework `V` is observable to fundamental holders (long-horizon institutions with reference-portfolio mandates) but is not a market price — the observed market price `P` aggregates `V` with the noise-trader demand. The anchor's role in the equilibrium is to provide the convergence target that bounded-arbitrage trades toward; without `V` as a reference, the noise-trader-only equilibrium would be indeterminate. **Source:** Shleifer (2000) Ch.2 pp.48-52.

## Mathematical Reasoning

The clientele-weighted price-formation identity EXPLAINS the structural relationship between equilibrium price and the constituent demand shares: symbolically, `P(t) = w · P_sentiment(t) + (1 − w) · V_fundamental(t)` where `w ∈ [0, 1]` is the clientele-weighted noise-trader share, `P_sentiment(t)` is the sentiment-driven shadow price (the price that would prevail under noise-trader demand alone), and `V_fundamental(t)` is the rational-expectations fundamental value. **Source:** Shleifer (2000) Ch.2 pp.28-42.

The identity DOCUMENTS three structural properties: the mispricing magnitude `|P(t) − V(t)| = w · |P_sentiment(t) − V_fundamental(t)|` scales linearly in the clientele weight `w` (a larger noise-trader share widens the mispricing for any given sentiment-fundamental gap); under `w = 0` the equilibrium collapses to the rational-expectations price; under `w = 1` the equilibrium becomes the sentiment-only price with no arbitrage anchoring. The intermediate-`w` regime is the realistic case in observed markets. **Source:** Shleifer (2000) Ch.2 pp.28-52.

The Hart+Mas-Colell adaptive-strategies / regret-matching framework APPLIES to the dynamic determination of `w` as a function of past-period realised arbitrage P&L: clientele weights evolve via regret-matching when each clientele's capital allocation responds to recent realised drawdowns. The supporting reference frames why the equilibrium-`w` is path-dependent rather than fixed — drawdowns shift capital toward whichever clientele performed best in the recent window, which is the noise-trader side after a sentiment-rally and the arbitrage side after a convergence episode. **Source:** Hart + Mas-Colell *Simple Adaptive Strategies* pp.5-30 + Shleifer (2000) Ch.2 pp.48-52.

The equilibrium-`w` dynamics PREDICT that the realised mispricing magnitude is auto-correlated at moderate horizons: a period of sentiment-rally that depletes arbitrage capital leaves `w` high entering the next period, which compounds the mispricing further before the inevitable convergence episode pulls `w` back down. The Shleifer 2000 anomaly inventory under [`be-value-anomaly.md`](./be-value-anomaly.md#mathematical-reasoning) and [`be-momentum-anomaly.md`](./be-momentum-anomaly.md#mathematical-reasoning) is partly an empirical signature of this `w`-dynamics — the same structural mechanism produces both the value premium (slow correction after extended growth-style rallies) and the momentum effect (continuation of recent winners under a rising-`w` regime). **Source:** Shleifer (2000) Ch.2 pp.42-52 + Ch.5 pp.112-153 + Ch.6 pp.154-174.

## See Also

- [`be-limits-of-arbitrage.md`](./be-limits-of-arbitrage.md#mathematical-reasoning) — the arbitrage-activation gate that explains why bounded arbitrage capacity cannot drive `w` to zero.
- [`be-sentiment-vs-fundamentals.md`](./be-sentiment-vs-fundamentals.md#intuition) — the divergence-path framing that visualises `|P − V|` as a function of `w` and sentiment-shock magnitude.
- [`be-regret-matching-foundations.md`](./be-regret-matching-foundations.md#definition) — the Hart+Mas-Colell regret-matching foundation that drives the `w`-dynamics path-dependence.
- [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#definition) — the overreaction / underreaction taxonomy that maps the noise-trader-driven mispricing to specific cognitive-source partitions.

## Escalate to Raw When

Open Shleifer 2000 *Inefficient Markets* Ch.2 directly when any of the criteria below applies. **Source:** Shleifer (2000) Ch.2 pp.28-52.

- The equilibrium-`w` derivation under a specific noise-trader-demand-elasticity assumption requires the original DeLong-Shleifer-Summers-Waldmann (1990) algebra rather than the bounded-deviation summary in this card. **Source:** Shleifer (2000) Ch.2 pp.28-42.
- A specific clientele-segmentation case study (closed-end fund discount, small-cap premium, narrative-driven rotation) requires the original Shleifer Ch.3 case-study narrative rather than the symbolic identity. **Source:** Shleifer (2000) Ch.3 pp.53-88.
- The Hart+Mas-Colell regret-matching dynamics on `w` require the original adaptive-strategies derivation rather than the verbal summary in this card. **Source:** Hart + Mas-Colell *Simple Adaptive Strategies* pp.5-30.
