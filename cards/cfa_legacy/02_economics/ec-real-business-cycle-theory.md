---
schema_version: "cacg.v0"
id: "ec-real-business-cycle-theory"
title: "Real Business Cycle Theory"
reading_id: "02_economics"
summary: "RBC explains short-run output fluctuations as the Walrasian-equilibrium response of an otherwise-Ramsey economy to stochastic productivity (and government-purchase) shocks; the propagation mechanism is intertemporal labor substitution — when productivity is temporarily high, workers shift labor effort across time — with cycles efficient by construction."
tags: ["economics", "real-business"]
citations:
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p213:0293"
    chunk_hash: "fae9b9f42b87fc5f3f928a9340e25b26c7fb46f6a0153294f4dfa14f490e94c5"
    page_range: [213, 214]
    quote: "Real-business-cycle theory focuses on the question of whether a Walrasian model provides a good description of the main features of observed fluctuations."
    edge_type: "defines"
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p217:0299"
    chunk_hash: "6d4baafaee90b7f7b505e92fcd538908546d15123e51d2c07e141ae5f2089760"
    page_range: [217, 218]
    quote: "These responses of labor supply to the relative wage and the interest rate are known as intertemporal substitution in labor supply"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p722:1004"
    chunk_hash: "9a7867565d6faa6d12258726469fc89f903fa11e8ab372fcd1ea4fff592e6a5c"
    page_range: [722, 723]
    quote: "Aggregate demand tells us only the relationship between the price level and the amount of output demanded at those prices."
    edge_type: "supports"
card_hash: "69be1b61ab3e63c2755805b19c31fa3cd8dbbed6dd1276ef4621152d6b853ffe"
---
# Real Business Cycle Theory

## Intuition

The Real Business Cycle (RBC) framework explains short-run output fluctuations as the equilibrium response of an otherwise-Ramsey economy to stochastic shocks — primarily productivity shocks to the production function `Y = A_t · F(K, L)`. Households optimally respond to a temporary productivity shock by reallocating labor across time (intertemporal labor substitution): when productivity is temporarily high (`A_t > A̅`), workers supply more labor today (because the real wage is high) and consume more, both today and in the future. The cycle in output is thus a *real-equilibrium* phenomenon, not a market failure — prices are flexible, markets clear, and the economy is always Pareto efficient. **Source:** Romer (2019) Ch.5 pp.188-237.

```
   RBC propagation: productivity shock to output

   A_t (productivity)
   ^
   | shock at initial period
   |      ___
   |     /   \___
   |    /        \____
   |---/             \____ steady-state A̅
   +---0---------------> t

   Y_t (output) — amplified + persistent
   ^                ___
   |               /   \
   |              /     \___ persistent (capital accumulates)
   |             /          \____
   |----steady state Y*-----------
   +---0---------------> t

   L_t (labor) — intertemporal substitution
   ^      ___
   |     /   \
   |    /     \____ workers reallocate from
   |---/           future to current period
   +---0---------------> t
```

The RBC model's central claim — that observed cycles are *efficient* equilibrium responses to real shocks — is the methodological opposite of the Keynesian / New Keynesian view that nominal rigidities create inefficient gaps between observed output and the flexible-price natural level. The empirical critique of RBC is that productivity shocks alone cannot account for the magnitude of cycles (Solow residuals are too small) and that the model has trouble matching the cross-correlation between hours worked and productivity. Modern macro takes RBC as the **frictionless baseline** that NK models extend with nominal rigidities. **Source:** Romer (2019) Ch.5 pp.225-237.

## Definition

The RBC model embeds a stochastic productivity process in the Ramsey-Cass-Koopmans household optimization. The representative household chooses consumption and labor supply to maximize expected discounted utility over an infinite horizon: **Source:** Romer (2019) pp.188-237.

```
max  E_0  ∑_{t ≥ 0}  β^t · [ u(c_t) − v(L_t) ]                [expected utility]
s.t.  k_{t+1}  =  A_t · F(k_t, L_t) − c_t + (1−δ) · k_t        [accumulation]
      log A_t  =  ρ · log A_{t-1} + ε_t                        [AR(1) shock]
      ε_t ~ N(0, σ_ε^2)                                        [white noise]
```

where `β ∈ (0, 1)` is the discount factor, `u(c)` is consumption utility, `v(L)` is labor disutility (so the planner trades off consumption against leisure `1 − L`), `δ` is depreciation, and the productivity `A_t` follows a first-order autoregressive process with persistence `ρ ∈ (0, 1)`. **Source:** Romer (2019) Ch.5 pp.190-205.

The **two first-order conditions** are the stochastic Euler equation (intertemporal consumption smoothing under uncertainty) and the intratemporal labor-leisure tradeoff: **Source:** Romer (2019) pp.188-237.

```
u'(c_t)  =  β · E_t[u'(c_{t+1}) · (A_{t+1} · F_k(k_{t+1}, L_{t+1}) + 1 − δ)]
v'(L_t)  =  u'(c_t) · A_t · F_L(k_t, L_t)
```

The Euler equation says marginal utility of current consumption equals discounted expected marginal utility of next-period consumption times the gross return on capital. The intratemporal condition equates the marginal disutility of an extra hour of work to the marginal utility of the consumption that extra hour buys at the real wage `A_t · F_L`. **Source:** Romer (2019) Ch.5 pp.205-220.

The **intertemporal-substitution elasticity** is the key parameter that governs RBC propagation. Define the labor-supply elasticity to a temporary wage change as: **Source:** Romer (2019) pp.188-237.

```
η  =  d log L / d log w        (Frisch elasticity, const. MU of consumption)
```

For RBC to generate substantial cyclical labor variation, `η` must be large — workers must be willing to substantially shift their labor effort across time in response to wage changes. Empirical micro estimates of `η` are systematically smaller than what RBC requires, which is the main empirical challenge to the RBC framework. **Source:** Romer (2019) Ch.5 pp.220-237.

## Mathematical Reasoning

Linearizing the RBC model around the deterministic steady state and solving forward gives the **impulse-response function** for output to a productivity shock `ε_t`. With AR(1) productivity `log A_t = ρ · log A_{t-1} + ε_t`, the response of `log Y_t` to a unit shock at the initial period is the product of two parts: (i) the direct effect of higher `A` on output, which decays at rate `ρ`; and (ii) the indirect effect via capital accumulation, which is hump-shaped (peaks several periods after the shock as capital is gradually accumulated, then decays). The combined response is more persistent than the underlying shock — capital provides a propagation mechanism even when productivity is purely transitory. **Source:** Romer (2019) Ch.5 pp.205-225.

The **intertemporal labor substitution** mechanism is the source of cyclical labor variation in RBC. From the intratemporal FOC `v'(L_t) = u'(c_t) · w_t` (with `w_t = A_t · F_L`), and the Euler equation `u'(c_t) = β · (1+r) · E_t u'(c_{t+1})`, eliminating `u'(c)` gives a condition relating relative labor supply across periods to relative wages: **Source:** Romer (2019) pp.188-237.

```
v'(L_t) / v'(L_{t+1})  ≈  ( w_t / w_{t+1} ) · ( 1 / β(1+r) )
```

A *temporary* productivity boom (`w_t` high but `w_{t+1}` back to normal) prompts a large positive labor-supply response in period `t`. A *permanent* productivity boom (`w` permanently high) has a much smaller labor-supply response (the wealth effect on labor supply offsets the substitution effect). This temporary-vs-permanent distinction is why RBC requires productivity shocks to be substantially transitory (`ρ` not too close to 1) to generate observable labor cycles. **Source:** Romer (2019) Ch.5 pp.220-237.

The **Solow-residual measurement** of productivity shocks `A_t` from the growth-accounting identity `Δ log Y = α · Δ log K + (1 − α) · Δ log L + Δ log A` is the standard empirical input to RBC calibration. The empirical critique is that measured Solow residuals are too small and too correlated with other macro variables (e.g., capacity utilization, demand) to be plausibly identified as pure productivity shocks — calling into question whether RBC's productivity-shock interpretation is the right one. Modern DSGE practice augments RBC with multiple shock sources (preference shocks, government-spending shocks, monetary shocks) and nominal rigidities (NK extensions; sibling `ec-nominal-rigidity-and-NK-monetary`). **Source:** Romer (2019) Ch.5 pp.225-237.

## See Also

- [`ec-ramsey-cass-koopmans-savings`](./ec-ramsey-cass-koopmans-savings.md) — deterministic Ramsey framework that RBC extends with stochastic productivity
- [`ec-aggregate-demand-representative-consumer`](./ec-aggregate-demand-representative-consumer.md) — representative-agent aggregation that underlies RBC's single-household formulation
- [`ec-nominal-rigidity-and-NK-monetary`](./ec-nominal-rigidity-and-NK-monetary.md) — New Keynesian extension that adds price stickiness to the RBC baseline
- [`ec-business-cycles-and-output-gaps`](./ec-business-cycles-and-output-gaps.md) — CFA L1 mechanical business-cycle taxonomy (peak/trough/recovery; leading/coincident/lagging indicators)

## Escalate to Raw When

The full RBC calibration exercise — choosing `β, α, δ, ρ, σ_ε` to match second moments of US data (variance of output, hours, consumption; correlations) — is in Romer Ch.5 pp.205-225 and Kydland-Prescott (1982) original RBC paper. The state-space solution methods for linearized DSGE models (Blanchard-Kahn, undetermined coefficients, perturbation methods) are graduate-macro material out of v10 scope. Calvo-style sticky-price extensions are treated in the sibling `ec-nominal-rigidity-and-NK-monetary`. The empirical challenges (Hansen-Sargent indivisible-labor extension, news-shock literature) are in Romer Ch.5 §5.7 pp.225-237. **Source:** Romer (2019) pp.188-237.
