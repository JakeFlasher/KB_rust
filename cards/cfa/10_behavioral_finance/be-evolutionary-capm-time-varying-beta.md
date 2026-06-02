---
schema_version: "cacg.v0"
id: "be-evolutionary-capm-time-varying-beta"
title: "Evolutionary CAPM With Time-Varying Beta"
reading_id: "10_behavioral_finance"
summary: "An evolutionary multi-asset HAM in which adaptive type fractions and a consensus belief yield a dynamic CAPM relation whose ex-ante betas vary over time as investor sentiment shifts, replacing the static efficient-market beta with an endogenous, adaptive one."
tags: ["behavioral-finance", "heterogeneous-agents", "evolutionary-capm", "time-varying-beta", "consensus-belief"]
citations:
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p307:0437"
    chunk_hash: "f1b397743550bb70a33ef5089fbab225d52975b978283694554e90e9ff12ad25"
    page_range: [307, 307]
    quote: "through the construction of a consensus belief, Chiarella et al. (2013b) develop a dynamic CAPM relationship between the market-average expected returns of the risky assets and their ex-ante betas in temporary equilibrium."
    edge_type: "defines"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p307:0437"
    chunk_hash: "f1b397743550bb70a33ef5089fbab225d52975b978283694554e90e9ff12ad25"
    page_range: [307, 307]
    quote: "changes in the market portfolio and risk-return relationships may occur due to changes of investor sentiment (such as chartists acting more strongly as momentum traders)."
    edge_type: "supports"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p312:0446"
    chunk_hash: "b0929b794a4005f1d5d8b6f6bfaa4cfa4f97b48a01c43b5de688ebb1d7b437b9"
    page_range: [312, 312]
    quote: "large literature on time-varying betas has been developed within the conditional CAPM"
    edge_type: "supports"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p312:0446"
    chunk_hash: "b0929b794a4005f1d5d8b6f6bfaa4cfa4f97b48a01c43b5de688ebb1d7b437b9"
    page_range: [312, 312]
    quote: "The CAPM relation (26) is evolutionary, since asset and market returns, as well as the corresponding consensus beliefs, co-evolve endogenously, based on the dynamic HAM with expectations feedback."
    edge_type: "supports"
card_hash: "1cff0c93930528173159e36848ef1045e21a9b58915e5e830da8c6999a0a4048"
---
# Evolutionary CAPM With Time-Varying Beta

## Intuition

The classical CAPM delivers a single, static beta: an asset's risk premium is proportional to its covariance with a fixed market portfolio. The **evolutionary CAPM** keeps the geometry of the CAPM relation but makes it dynamic. When mean-variance investors hold heterogeneous, evolving beliefs about means, variances, and covariances, the market temporary equilibrium can be rewritten AS IF a single representative agent held a **consensus belief** — a population-weighted aggregate of the heterogeneous beliefs. The result is a CAPM-like relation in which the betas are ex-ante, belief-dependent quantities that move as the composition and beliefs of the population shift. **Source:** Dieci and He (2018) §4.2 pp.289-289.

The behavioral payoff is a structural explanation for the long-debated "time-varying betas" and the instability of risk-return relationships. As investor sentiment changes — for instance when chartists begin to act more strongly as momentum traders — the consensus belief shifts, the market portfolio weights change, and systematic risk-return relationships move with them. Beta is no longer a fixed structural parameter of an asset; it is an emergent property of an adaptive market. **Source:** Dieci and He (2018) §4.2 pp.289-289.

This framework lets one compare theoretical ex-ante betas against the rolling-window beta estimates practitioners actually use, and it shows that asset diversification by heterogeneous investors can produce aggregate risk-return relationships that differ substantially from standard mean-variance equilibrium predictions. The static efficient market is replaced by an adaptive one. **Source:** Dieci and He (2018) §4.2 pp.294-295.

## Definition

**Consensus belief** is the population-weighted aggregate of heterogeneous investors' first and second moment beliefs, `{E_{a,t}, Omega_{a,t}}`, such that the temporary equilibrium price equals that of a homogeneous agent with average risk aversion `theta_{a,t}` holding the consensus belief. **Source:** Dieci and He (2018) §4.2 pp.289-290.

**Evolutionary CAPM** is the dynamic, belief-dependent CAPM relation derived under the consensus belief, in which asset returns, the market portfolio, and the consensus beliefs co-evolve endogenously through the HAM's expectation-feedback mechanism. **Source:** Dieci and He (2018) §4.2 pp.294-294.

**Ex-ante beta** `beta_{j,t}` is the consensus-belief covariance of asset `j`'s return with the market return divided by the consensus-belief variance of the market return; it varies over time as the consensus beliefs and market weights change, distinct from the ex-post rolling-window beta. **Source:** Dieci and He (2018) §4.2 pp.294-294.

## Mathematical Reasoning

With `H` agent-types, each of risk aversion `theta_h` maximizing CARA utility `u_h(w) = -exp(-theta_h w)`, the optimal demand for the `N` risky assets is the mean-variance vector `z_{h,t} = theta_h^{-1} Omega_{h,t}^{-1}[E_{h,t}(x_{t+1}) - R_f p_t]`. Market clearing `sum_h n_{h,t} z_{h,t} = z_t^s` solves for temporary-equilibrium prices that can be written via the consensus belief, with `Omega_{a,t} = theta_{a,t}^{-1}(sum_h n_{h,t} theta_h^{-1} Omega_{h,t}^{-1})^{-1}` and `E_{a,t}(x_{t+1}) = theta_{a,t} Omega_{a,t} sum_h n_{h,t} theta_h^{-1} Omega_{h,t}^{-1} E_{h,t}(x_{t+1})`. **Source:** Dieci and He (2018) §4.2 pp.289-290.

Type fractions evolve by the discrete-choice (logit) rule on a risk-adjusted fitness `v_{h,t-1}`, `n_{h,t} = exp(eta v_{h,t-1})/Z_t`, with `eta > 0` the intensity of choice. Fundamentalists hold beliefs that revert toward fundamental values `p*_t`; trend followers extrapolate moving averages with strength `gamma_j`. The stochastic nonlinear multi-asset HAM with two belief-types yields a recursive asset-price equation whose deterministic skeleton has a unique steady state `F* = (p*, p*, 0, n_f*)`. **Source:** Dieci and He (2018) §4.2 pp.290-293.

Defining market wealth `W_{m,t} = p_t^T s` and `W_{m,t+1} = x_{t+1}^T s`, asset and market returns `r_{j,t+1}, r_{m,t+1}` under the consensus belief satisfy the CAPM-like relation:

```
   E_{a,t}(r_{t+1}) - r_f * 1  =  beta_{a,t} * [ E_{a,t}(r_{m,t+1}) - r_f ]
   beta_{j,t} = Cov_{a,t}(r_{m,t+1}, r_{j,t+1}) / Var_{a,t}(r_{m,t+1})
```

This relation is EVOLUTIONARY because `beta_{a,t}` is built from the consensus moments `E_{a,t}, Omega_{a,t}`, which themselves co-evolve with prices and type fractions. Simulations show the ex-ante betas vary significantly and at substantially different levels across subperiods, and 100- and 300-period rolling betas reveal systematic changes in risk-return relationships qualitatively similar to the ex-ante betas. **Source:** Dieci and He (2018) §4.2 pp.294-295.

## See Also

- [be-brock-hommes-switching](./be-brock-hommes-switching.md#intuition) — the logit fitness-switching that drives the evolving type fractions feeding the consensus belief.
- [be-fundamentalist-chartist-ham](./be-fundamentalist-chartist-ham.md#intuition) — the two-type belief structure (fundamentalists vs trend followers) extended here to multiple assets.
- [be-three-frameworks-behavioral-asset-pricing](./be-three-frameworks-behavioral-asset-pricing.md#intuition) — the broader behavioral asset-pricing frameworks within which an adaptive-CAPM sits.

## Escalate to Raw When

- The full derivation of the consensus-belief aggregation, the fitness measures `v_{f,t}, v_{c,t}`, and the optimal portfolios (22)-(23) is needed. **Source:** Dieci and He (2018) §4.2 pp.290-293.
- The local-stability thresholds `eta-hat_j` (Eq. 25) and the Neimark-Sacker condition for the multi-asset system must be quoted exactly. **Source:** Dieci and He (2018) §4.2 pp.293-293.
- The specific parameterization and the figures showing ex-ante vs rolling-window betas (Fig. 8) require the source plots. **Source:** Dieci and He (2018) §4.2 pp.294-295.
