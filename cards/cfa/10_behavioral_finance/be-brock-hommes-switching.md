---
schema_version: "cacg.v0"
id: "be-brock-hommes-switching"
title: "Brock-Hommes Adaptive Belief Switching"
reading_id: "10_behavioral_finance"
summary: "Brock-Hommes adaptive belief system: agents endogenously switch among forecasting rules according to past realized profit via a discrete-choice (logit) rule, with the intensity-of-choice beta governing how sharply they chase the more profitable strategy."
tags: ["behavioral-finance", "heterogeneous-agents", "discrete-choice", "intensity-of-choice", "adaptive-beliefs"]
citations:
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p284:0403"
    chunk_hash: "dd5faedd6aef948101ce3682e6068425e8df66cd2db49110d8a65fcf69626597"
    page_range: [284, 284]
    quote: "Following Brock and Hommes (1997, 1998), the market fraction of investors choosing strategy h at time t + 1 is determined by"
    edge_type: "defines"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p284:0403"
    chunk_hash: "dd5faedd6aef948101ce3682e6068425e8df66cd2db49110d8a65fcf69626597"
    page_range: [284, 284]
    quote: "measures the intensity of the choice and"
    edge_type: "supports"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p284:0403"
    chunk_hash: "dd5faedd6aef948101ce3682e6068425e8df66cd2db49110d8a65fcf69626597"
    page_range: [284, 284]
    quote: "Let πh,t+1 be the realized profit between t and t +1 of type-h investors, πh,t+1 = zh,t(pt+1 + dt+1 − Rpt) for h = 1, 2."
    edge_type: "supports"
card_hash: "4ee1f59e349a6f5b29c818a43fa955bbb72cdd07c973cd9e92e2cfb1f0ea32b6"
---
# Brock-Hommes Adaptive Belief Switching

## Intuition

The two-type HAM becomes an **adaptive belief system** once the population fractions are allowed to evolve. Brock and Hommes (1997, 1998) make agents boundedly rational evolutionary opportunists: at each date they look back at how each forecasting rule has actually performed and migrate toward whichever rule earned the higher realized profit. Cheap, simple rules of thumb compete in a kind of fitness contest, and the market's composition is itself a dynamic variable rather than a fixed parameter. **Source:** Dieci and He (2018) §2.1 pp.266-266.

The discrete-choice (multinomial logit) rule that governs migration is parameterized by the **intensity of choice** `beta`. When `beta` is small, agents barely respond to performance differences and fractions stay near a 50/50 mix; when `beta` is large, almost everyone piles into the best-performing rule. The intensity of choice is therefore the model's behavioral temperature: it measures how rationally (sharply) agents chase profit. This single parameter turns out to be the knob that destabilizes the market, which is the subject of the rational-route-to-randomness card. **Source:** Dieci and He (2018) §2.1 pp.266-266.

Empirically the switching version fits markets better than fixed-fraction versions: calibrations to the DAX-30 show a market dominated by investors (about 70%) who constantly switch between fundamental and trend-following strategies, with a residual minority who never change. The adaptive behavior of investors — not a static mix — is what generates the observed power-law volatility behavior. **Source:** Dieci and He (2018) §2.2 pp.267-268.

## Definition

**Adaptive belief system** is a HAM in which the market fractions of each forecasting type are updated each period as a function of the rules' recent realized performance, so beliefs co-evolve with prices and with each other. **Source:** Dieci and He (2018) §2.1 pp.265-266.

**Intensity of choice** `beta` is the discrete-choice parameter scaling the sensitivity of type fractions to profit differences: `beta = 0` gives constant equal fractions; `beta -> infinity` makes all agents instantly switch to the single most profitable rule. **Source:** Dieci and He (2018) §2.1 pp.266-266.

**Strategy cost** `C_h >= 0` is a per-period cost of using rule `h` (e.g. the cost of gathering fundamental information), which enters the logit alongside profit so that a cheaper rule is favored at equal gross performance. **Source:** Dieci and He (2018) §2.1 pp.266-266.

## Mathematical Reasoning

Let `pi_{h,t+1} = z_{h,t}(p_{t+1} + d_{t+1} - R p_t)` be the realized profit of type-`h` investors between `t` and `t+1`, where `z_{h,t}` is their order flow (position) and the bracket is the realized excess return. The fraction choosing strategy `h` at `t+1` is the multinomial logit (Gibbs) rule:

```
                       exp[ beta (pi_{h,t+1} - C_h) ]
   n_{h,t+1}  =  ----------------------------------------,   h = 1, 2.
                  sum_i  exp[ beta (pi_{i,t+1} - C_i) ]
```

Here `beta` is the intensity of choice and `C_h` the cost. The map is order-preserving in profit: higher relative `pi_{h}` raises `n_h`, and the responsiveness is controlled entirely by `beta`. **Source:** Dieci and He (2018) §2.1 pp.266-266.

For two types it is convenient to track the difference in fractions through `m_t = tanh[ (beta/2)(z_{1,t-1} - z_{2,t-1})(p_t + d_t - R p_{t-1}) - (beta/2)(C_1 - C_2) ]`, which closes the dynamic system together with the price-adjustment equation, the trend statistic `u_t = delta u_{t-1} + (1 - delta)p_t`, and the sample variance `v_t`. The full discrete-time random dynamic system thus reads (schematically):

```
   p_{t+1} = p_t + mu (q_{1,t} z_{1,t} + q_{2,t} z_{2,t}) + delta_t
   u_t     = delta u_{t-1} + (1 - delta) p_t
   v_t     = delta v_{t-1} + delta(1 - delta)(p_t - u_{t-1})^2
   m_t     = tanh[ (beta/2)(z_{1,t-1} - z_{2,t-1} - (C_1 - C_2))(p_t + d_t - R p_{t-1}) ]
```

The `tanh` form is the binary specialization of the logit; it saturates at +/-1 as `beta` grows, encoding the all-or-nothing herding into the best rule in the high-intensity limit. **Source:** Dieci and He (2018) §2.1 pp.266-266.

## See Also

- [be-fundamentalist-chartist-ham](./be-fundamentalist-chartist-ham.md#intuition) — the underlying two-type demand structure whose fractions this rule updates.
- [be-bifurcation-route-instability](./be-bifurcation-route-instability.md#intuition) — what happens to stability as the intensity of choice rises.
- [be-regret-matching-foundations](./be-regret-matching-foundations.md#intuition) — a related learning dynamic in which past-performance comparisons drive strategy adjustment.
- [be-rationally-heterogeneous-expectations](./be-rationally-heterogeneous-expectations.md#intuition) — the same logit switching used to fit learning-to-forecast laboratory data.

## Escalate to Raw When

- The exact derivation of the `tanh` reduction and the closed-form fraction equations `q_{1,t}, q_{2,t}` from the logit is needed. **Source:** Dieci and He (2018) §2.1 pp.265-266.
- A precise calibrated value of the intensity of choice (the no-switching, pure-switching, and full models) must be read off the estimation table. **Source:** Dieci and He (2018) §2.2 pp.267-268.
- The asymptotic memory specifications and the role of `C_1 - C_2` in selecting between strategies require the source's full treatment. **Source:** Dieci and He (2018) §2.1 pp.266-266.
