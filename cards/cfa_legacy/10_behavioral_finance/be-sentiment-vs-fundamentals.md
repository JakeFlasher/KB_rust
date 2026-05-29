---
schema_version: "cacg.v0"
id: "be-sentiment-vs-fundamentals"
title: "Sentiment-Driven Mispricing vs Fundamental Anchor"
reading_id: "10_behavioral_finance"
summary: "Sentiment-Driven Mispricing vs Fundamental Anchor: framing the divergence between a sentiment-driven price path `P(t)` and a stable fundamental anchor `V` — the bounded-deviation magnitude, the indefinite convergence horizon, the role of arbitrage capacity in shaping the divergence envelope, and the visual primitive that supports Batch 1 anomaly cards and Batch 2 crisis-narrative cards"
tags: ["behavioral-finance", "sentiment", "mispricing"]
citations:
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p163:0184"
    chunk_hash: "cb806cf8956744b1516d01a7aa44d139cdf16e98453ce1e3e6efe03ee9410ff7"
    page_range: [163, 164]
    quote: "It can result from extrapolative expectations about prices, or trend chasing"
    edge_type: "supports"
  - source_id: "econ_hart_mascolell_2013_simple_adaptive_strategies"
    chunk_id: "econ_hart_mascolell_2013_simple_adaptive_strategies:p035:0042"
    chunk_hash: "9a3492843977401b78f8681d112e53cfcda0cde381a9dea11109bae16c182e23"
    page_range: [35, 35]
    quote: "correlated equilibria are, in contrast, “dynamically easy.”"
    edge_type: "supports"
card_hash: "3ff930a81de0ff961c0905082c8451a46c9d48be23cc0308e6a0b035d673f395"
---
# Sentiment-Driven Mispricing vs Fundamental Anchor

## Intuition

The textbook efficient-market hypothesis ASSERTS that price `P(t)` tracks fundamental value `V(t)` continuously, with deviations decaying rapidly under rational-arbitrage pressure. Shleifer's case-study chapters DOCUMENT that observed market price paths diverge from `V` for sustained periods — closed-end fund discounts, internet-stock cycles, small-cap premium episodes, narrative-driven sector rotations — with the divergence magnitude bounded but the convergence horizon indefinite. **Source:** Shleifer (2000) Ch.3 pp.53-88 + Ch.6 pp.154-174.

The sentiment-vs-fundamentals diagram below visualises the structural shape: `P(t)` follows a sentiment-driven path that opens a wedge against the slowly-drifting fundamental anchor `V`, peaks at an indefinite intermediate horizon, then eventually converges. The divergence-and-convergence pattern is the empirical signature that underlies the Batch 1 anomaly inventory and the Batch 2 crisis-narrative formation. **Source:** Shleifer (2000) Ch.6 pp.154-174.

```
<!-- primitive: sentiment-vs-fundamentals source: _diagram_primitives.md -->
   price P                                    sentiment-driven
   ^                                          path P(t)
   |                              * *
   |                          * *     *
   |                       *             *
   |                    *                   *
   |                  *                       *
   |               *                            * *
   |             *                                   * *  *
   |          *                                              *  convergence
   |        *                                                      to V
   |- - -*- - - - - - - - - - - - - - - - - - - - - - - - - - * -  fundamental
   |    *                                                      anchor V
   |   *                                                       (slow drift)
   |  *
   | *
   +*--------------------------------------------------> time t
   t0       t1: divergence opens       t2: peak       t3: convergence
                                                       horizon (indef.)

   * P deviates from V because noise-trader demand is partially price-inelastic.
   * Arbitrage capacity bounds the maximum divergence (P - V) but cannot
     guarantee convergence within any pre-specified t3 horizon.
   * Cards instantiating this MUST NOT overlay numerical P or V values.
```

Shleifer EXPLAINS that the divergence is not a market-microstructure curiosity but a structural feature: it emerges whenever noise-trader demand is partially price-inelastic ([`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#definition)) and arbitrage capacity is bounded ([`be-limits-of-arbitrage.md`](./be-limits-of-arbitrage.md#mathematical-reasoning)). The combination produces a stable equilibrium in which `|P − V|` is non-zero but bounded above by a multiplicative function of arbitrage-capacity tightness — the bounded-deviation inequality from [`be-limits-of-arbitrage.md`](./be-limits-of-arbitrage.md#mathematical-reasoning). **Source:** Shleifer (2000) Ch.6 pp.165-174.

## Definition

**Fundamental anchor `V(t)`** is the rational-expectations price under full information and unlimited arbitrage. Shleifer ASSERTS that `V` is observable to long-horizon institutional investors (pension funds, endowments, value-style managers) through standard cash-flow / discount-rate decomposition, but is not directly recoverable from market prices in the presence of noise-trader demand. The anchor's role is to provide the convergence target that bounded-arbitrage trades toward, not to specify the realised market price. **Source:** Shleifer (2000) Ch.5 pp.112-125.

**Sentiment-driven path `P(t)`** is the realised market price under the joint influence of fundamental flow, noise-trader demand, and arbitrage rebalancing. Shleifer DOCUMENTS that `P(t)` follows a clientele-weighted process from [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#mathematical-reasoning): `P(t) = w(t) · P_sentiment(t) + (1 − w(t)) · V(t)` with the clientele-weight `w(t)` evolving under regret-matching dynamics from [`be-regret-matching-foundations.md`](./be-regret-matching-foundations.md#mathematical-reasoning). **Source:** Shleifer (2000) Ch.5 pp.112-153 + Ch.6 pp.154-167.

**Divergence envelope** is the bounded-deviation region `|P(t) − V(t)| ≤ K · (1/H)` from the limits-of-arbitrage equilibrium ([`be-limits-of-arbitrage.md`](./be-limits-of-arbitrage.md#mathematical-reasoning)) where `K` aggregates noise-trader-risk variance, agency cost severity, and capital constraint tightness, and `H` is the arbitrageur's bounded holding horizon. The envelope is a soft cap: deviations exceeding the envelope attract larger arbitrage flow, but deviations within the envelope persist. **Source:** Shleifer (2000) Ch.5 pp.112-130 + Ch.6 pp.154-174.

**Convergence horizon `t_3`** is the indefinite future date at which the sentiment-driven divergence resolves and `P(t_3) ≈ V(t_3)`. Shleifer ASSERTS that `t_3` is not a fixed parameter but a stopping time determined by the arrival of a convergence-triggering event: a fundamental shock that the noise-trader demand cannot rationalise, an arbitrage-capital influx that breaks the regret-matching equilibrium, or a long-horizon institutional rebalancing flow toward `V`. The indefiniteness of `t_3` is the source of noise-trader risk on the arbitrageur side. **Source:** Shleifer (2000) Ch.6 pp.165-174.

## Mathematical Reasoning

The bounded-deviation envelope EXPLAINS the visual shape of the sentiment-vs-fundamentals diagram: the maximum sustained gap `|P(t) − V(t)|` is bounded above by `K · (1/H)` for every `t`, but the realised gap can drift within the envelope according to the clientele-weight dynamics from [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#mathematical-reasoning). The diagram's peak at `t_2` corresponds to the maximum sustainable `w(t_2)` (the noise-trader weight that the arbitrage capacity can absorb without forcing convergence). **Source:** Shleifer (2000) Ch.5 pp.112-130 + Ch.6 pp.154-174.

The Hart+Mas-Colell regret-matching dynamics APPLY to the path-dependence of `w(t)`: a long sentiment-rally builds up unrealised arbitrage losses, which under regret-matching push the arbitrageur clientele to reduce their capital allocation (their realised drawdown signals high regret on the convergence trade); the reduced arbitrage capacity then allows `w(t)` to rise further, extending the divergence. The dynamics PREDICT that long divergence episodes are followed by sharp convergence episodes when an arbitrage-attracting event eventually breaks the equilibrium. **Source:** Hart + Mas-Colell *Simple Adaptive Strategies* pp.30-50 + Shleifer (2000) Ch.6 pp.154-174.

The divergence-and-convergence pattern PREDICTS three empirically-testable signatures: (i) cross-sectional excess returns on previously-overshooting securities are negative on average over long horizons (the reversal signature of value-style and long-horizon-overreaction anomalies — see [`be-value-anomaly.md`](./be-value-anomaly.md#mathematical-reasoning)); (ii) the realised variance of `P(t)` exceeds the realised variance of `V(t)` by a structural margin attributable to noise-trader risk (the excess-volatility signature documented by Shiller and re-framed by Shleifer); (iii) the convergence-trigger episodes are temporally clustered with sentiment-cycle-end shocks rather than uniformly distributed. **Source:** Shleifer (2000) Ch.6 pp.165-174.

The boundary against subcorpus 09's efficient-markets-and-anomalies framing in `pm-efficient-markets-and-anomalies.md` is firm: the same `P − V` divergence is interpreted in 09 as a factor-priced risk premium (the value/momentum/quality factor exposures earn excess returns because they bear systematic risk), whereas the per-10 framing here ASSERTS the same divergence arises from bounded-arbitrage failure of a behavioral-bias-driven mispricing. The two framings are not mutually exclusive — both can hold for the same anomaly — but the per-10 BOUNDARY-DISCIPLINE in `_style_guide.md` partitions ownership of the mechanistic narrative. **Source:** Shleifer (2000) Ch.6 pp.165-174.

## See Also

- [`be-limits-of-arbitrage.md`](./be-limits-of-arbitrage.md#mathematical-reasoning) — the bounded-deviation arbitrage equilibrium that supplies the divergence-envelope inequality.
- [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#mathematical-reasoning) — the clientele-weighted price-formation identity that supplies the path-of-`P(t)` decomposition.
- [`be-regret-matching-foundations.md`](./be-regret-matching-foundations.md#mathematical-reasoning) — the adaptive-strategies dynamics that drive the path-dependence of the clientele-weight `w(t)`.
- [`be-two-model-mispricing.md`](./be-two-model-mispricing.md#definition) — the overreaction / underreaction taxonomy that specialises the divergence pattern to specific anomaly classes.

## Escalate to Raw When

Open Shleifer 2000 *Inefficient Markets* Ch.5 + Ch.6 directly when any of the criteria below applies. **Source:** Shleifer (2000) Ch.5 pp.112-153 + Ch.6 pp.154-174.

- A specific divergence-vs-convergence episode requires the original Ch.3 / Ch.6 case-study narrative (closed-end fund discount in Ch.3, internet-stock / positive-feedback cycle in Ch.6) rather than the symbolic divergence-envelope summary in this card. **Source:** Shleifer (2000) Ch.3 pp.53-88 + Ch.6 pp.154-174.
- The Shiller excess-volatility argument and its Shleifer-reframing under noise-trader-risk require the original Ch.6 §6.x discussion rather than the symbolic prediction in this card. **Source:** Shleifer (2000) Ch.6 pp.165-174.
- A cross-vertical synthesis card needs the bounded-deviation arbitrage equilibrium combined with Shiller-style excess-volatility evidence — open both Shleifer Ch.5 + Ch.6 for the joint reading. **Source:** Shleifer (2000) Ch.5 pp.112-130 + Ch.6 pp.165-174.
