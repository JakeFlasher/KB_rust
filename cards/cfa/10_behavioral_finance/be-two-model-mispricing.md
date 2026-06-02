---
schema_version: "cacg.v0"
id: "be-two-model-mispricing"
title: "Two-Model Mispricing Taxonomy"
reading_id: "10_behavioral_finance"
summary: "Two-Model Mispricing Taxonomy: framing the two-model partition of behavioral-finance mispricing — Barberis-Shleifer-Vishny conservatism-driven underreaction vs Daniel-Hirshleifer-Subrahmanyam representativeness-driven overreaction — and the cross-sectional anomaly signatures each model predicts"
tags: ["behavioral-finance", "underreaction", "overreaction"]
citations:
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p120:0137"
    chunk_hash: "3957b4864cb441abe4ac95320470e0f9e4d26e0d37af6b30f88c75b0b36cacb2"
    page_range: [121, 121]
    quote: "regularities, already mentioned in Chapter 1, are conveniently called underreaction and overreaction"
    edge_type: "defines"
card_hash: "56daf0c55f4909f4a950b840b3bd75f77fbf873fd717c4498c85b3eb2a06d255"
---
# Two-Model Mispricing Taxonomy

## Intuition

Shleifer ASSERTS that the empirical behavioral-finance anomalies do not arise from a single cognitive bias but from two structurally distinct mispricing mechanisms that often coexist in the same market: **representativeness-driven overreaction** (Daniel-Hirshleifer-Subrahmanyam framework) and **conservatism-driven underreaction** (Barberis-Shleifer-Vishny framework). The two models predict opposite directional patterns at the same forecast horizon, and the realised anomaly inventory partitions cleanly along this axis. **Source:** Shleifer (2000) Ch.5 pp.112-125.

The taxonomy EXPLAINS why some anomalies (value premium, long-horizon reversal) show **mean-reversion** signatures while others (momentum, post-earnings drift) show **continuation** signatures: the former are corrections of prior overreaction (the price overshot fundamental value and now reverts); the latter are gradual corrections of prior underreaction (the price under-adjusted to new information and continues to drift toward fundamental value). The behavioral-bias-taxonomy diagram below shows the cognitive-source partition that drives the two models. **Source:** Shleifer (2000) Ch.5 pp.125-140.

```
<!-- primitive: behavioral-bias-taxonomy source: _diagram_primitives.md -->
                          +-----------------------+
                          |   Behavioral biases   |
                          +-----------+-----------+
                                      |
              +-----------------------+------------------------+
              |                       |                        |
       +------+------+         +------+-------+         +------+--------+
       | Heuristic   |         | Framing /    |         | Emotional /   |
       | (cognitive) |         | reference    |         | social        |
       +------+------+         +------+-------+         +------+--------+
              |                       |                        |
   +----------+----------+   +--------+--------+    +----------+----------+
   | representativeness  |   | anchoring       |    | overconfidence      |
   | availability        |   | framing effect  |    | loss aversion       |
   | recency / hindsight |   | mental accounts |    | herding / consensus |
   | gambler's fallacy   |   | status quo bias |    | regret aversion     |
   +---------------------+   +-----------------+    +---------------------+

   downstream pricing effect: each bias drives a specific deviation
   from the rational-expectations benchmark documented in card bodies.
```

Shleifer's central methodological point ASSERTS that the two models are not mutually exclusive: a market can exhibit overreaction at short horizons (representativeness-driven momentum continuation that overshoots) and underreaction at longer horizons (conservatism-driven slow drift toward the fundamental anchor) simultaneously. The observed signature depends on the forecast horizon, the salience of the underlying information, and the clientele weights from [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#definition). **Source:** Shleifer (2000) Ch.5 pp.140-153.

## Definition

**Representativeness-driven overreaction** is the mispricing pattern where investors over-weight recent observations relative to base rates, mistaking a salient short pattern for a structural shift. Shleifer EXPLAINS the cognitive mechanism (Tversky-Kahneman representativeness heuristic): when an outcome is "representative" of a category (rapid recent growth representative of "high-growth firm"), agents over-attribute the outcome to the category and under-attribute it to chance. The pricing signature is initial-period extrapolation followed by long-horizon reversal as the category-attribution proves incorrect. **Source:** Shleifer (2000) Ch.5 pp.112-125.

**Conservatism-driven underreaction** is the mispricing pattern where investors update beliefs too slowly relative to a Bayesian benchmark, anchoring on prior estimates and discounting new information. Shleifer EXPLAINS the cognitive mechanism (Edwards conservatism finding): when an unexpected event arrives (positive earnings surprise, unanticipated dividend cut), agents adjust their forecast in the correct direction but by a magnitude that under-shoots the Bayesian update. The pricing signature is gradual continuation in the direction of the news as the under-shoot is corrected over subsequent periods. **Source:** Shleifer (2000) Ch.5 pp.125-135.

**Cross-sectional anomaly partition** is the empirical taxonomy of observed anomalies by mechanism. Shleifer DOCUMENTS that the value premium and long-horizon reversal in cross-sectional returns map to representativeness-driven overreaction; the momentum effect and post-earnings-announcement drift map to conservatism-driven underreaction. Cards [`be-value-anomaly.md`](./be-value-anomaly.md#intuition), [`be-momentum-anomaly.md`](./be-momentum-anomaly.md#intuition), and [`be-investor-overreaction.md`](./be-investor-overreaction.md#intuition) develop each anomaly's specific evidence base. **Source:** Shleifer (2000) Ch.5 pp.140-153 + Ch.6 pp.154-174.

**Coexistence regime** is the empirically common case where the same security exhibits underreaction at short horizons (gradual drift after news) and overreaction at long horizons (cumulative drift eventually overshoots). The two-model partition does not assume a single mechanism dominates a given market — it provides the descriptive vocabulary for diagnosing which mechanism is active at which horizon. **Source:** Shleifer (2000) Ch.5 pp.135-153.

## Mathematical Reasoning

The Barberis-Shleifer-Vishny conservatism model EXPLAINS the gradual-drift signature with a symbolic underreaction operator: the agent's posterior belief `B(t+1)` after observing signal `s_t` is `B(t+1) = (1 − λ) · B(t) + λ · B_Bayes(t+1)` with `λ < 1`, where `B_Bayes(t+1)` is the Bayesian posterior and `λ` is the conservatism-discount factor. Lower `λ` corresponds to stronger conservatism (slower belief revision) and produces longer continuation drift. **Source:** Shleifer (2000) Ch.5 pp.125-130.

The Daniel-Hirshleifer-Subrahmanyam overreaction model EXPLAINS the overshoot-and-reversal signature with a symbolic overconfidence-amplifier: the agent's posterior places weight `α > 1` on private signals and weight `< 1` on public corrections, so initial-period reactions to private information are amplified relative to the Bayesian benchmark and subsequent-period corrections are damped. The amplification produces initial-period continuation (the overreaction phase) followed by long-horizon reversal as the under-weighted public-correction signals accumulate. **Source:** Shleifer (2000) Ch.5 pp.120-130.

The two-model partition PREDICTS distinct horizon signatures: under conservatism, the cumulative return `R(t, t+h)` is monotonically increasing in `h` until the Bayesian-target is reached, with no overshoot; under representativeness-driven overreaction, `R(t, t+h)` rises through an intermediate peak (the overshoot horizon) and then declines toward the eventual reversal level. The intermediate peak under overreaction is the empirical signature of the momentum-reversal sequence documented in [`be-investor-overreaction.md`](./be-investor-overreaction.md#mathematical-reasoning). **Source:** Shleifer (2000) Ch.5 pp.140-153.

The bounded-deviation arbitrage equilibrium from [`be-limits-of-arbitrage.md`](./be-limits-of-arbitrage.md#mathematical-reasoning) APPLIES to both models: arbitrageurs trade against the mispricing in both directions (selling overreaction-driven overshoots, buying underreaction-driven gaps) but the same gate conditions (noise-trader risk, agency cost, capital constraint) bound the arbitrage capacity in both cases. The clientele-weighted price formation from [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#mathematical-reasoning) supplies the equilibrium-`w` dynamics for both models. **Source:** Shleifer (2000) Ch.4 pp.89-111 + Ch.5 pp.112-153.

## See Also

- [`be-limits-of-arbitrage.md`](./be-limits-of-arbitrage.md#mathematical-reasoning) — the four-condition gate that bounds arbitrage against both overreaction and underreaction mispricing.
- [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#mathematical-reasoning) — the clientele-weighted price-formation identity that supplies the equilibrium dynamics for both models.
- [`be-sentiment-vs-fundamentals.md`](./be-sentiment-vs-fundamentals.md#intuition) — the divergence-path framing whose shape signature (bounded peak then convergence) maps to the overreaction-overshoot prediction.
- [`be-regret-matching-foundations.md`](./be-regret-matching-foundations.md#definition) — the adaptive-strategies framework that supplies the path-dependent equilibrium dynamics shared by both models.

## Escalate to Raw When

Open Shleifer 2000 *Inefficient Markets* Ch.5 directly when any of the criteria below applies. **Source:** Shleifer (2000) Ch.5 pp.112-153.

- The Barberis-Shleifer-Vishny conservatism model requires the original 1998 derivation with the explicit two-state Markov-chain belief evolution rather than the symbolic operator summary in this card. **Source:** Shleifer (2000) Ch.5 pp.125-130.
- The Daniel-Hirshleifer-Subrahmanyam overreaction model requires the original 1998 derivation with the explicit private/public signal weighting algebra rather than the symbolic amplifier summary. **Source:** Shleifer (2000) Ch.5 pp.120-130.
- A specific empirical anomaly under analysis appears to require a hybrid model (coexistence regime) where neither conservatism nor representativeness alone is sufficient — open Ch.5 for the dual-process discussion. **Source:** Shleifer (2000) Ch.5 pp.140-153.
