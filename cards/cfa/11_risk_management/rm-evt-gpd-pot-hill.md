---
schema_version: "cacg.v0"
id: "rm-evt-gpd-pot-hill"
title: "Applied EVT: GPD, Peaks-Over-Threshold, and the Hill Estimator"
reading_id: "11_risk_management"
summary: "McNeil's applied/statistical EVT mechanism for fat tails: GEV for block maxima, Pickands-Balkema-de Haan giving the GPD as the limiting excess-over-threshold law, the mean-excess diagnostic, the POT point-process model, and the Hill tail-index estimator feeding deep-tail VaR/ES, per McNeil et al. (2015) Ch.5."
tags: ["risk-management", "extreme-value-theory", "fat-tails"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p170:0241"
    chunk_hash: "7f1251c7927b7eddfadb00e6f839325b36adb4872eb6b30ee8871f254df2b0fe"
    page_range: [170, 170]
    quote: "the GPD is the canonical distribution for modelling excess losses over high thresholds"
    edge_type: "defines"
card_hash: "cc64f596bc81b6ce779bb9ad431b6b73aa7730cb87b8df7fd8cc8a7f9da94ce9"
---
# Applied EVT: GPD, Peaks-Over-Threshold, and the Hill Estimator

## Intuition
This card is the **applied / statistical EVT** mechanism for fat tails — McNeil's
program of *fitting limit laws directly to the tail* of realized losses, as opposed
to the other three routes by which fat tails enter risk: heavy-tail probability /
ruin asymptotics (Cramer-Lundberg, Fisher-Tippett extreme-value types), Levy /
stable-Paretian scaling, and crash-as-criticality. Here fat tails are an empirical
object we *estimate*: we do not assume a parametric loss law globally, we let
extreme-value limit theorems tell us the *shape of the tail alone*, and we read the
tail heaviness off a single shape parameter ξ. Larger ξ means a heavier tail and a
larger gap between VaR and expected shortfall far out in the tail.

```
   loss data (mostly central)        keep only exceedances over u
   x x x x x x x x x x x x  ──────►        o   o     o   o
   --------------------|----u----            \  |   /   /
                       threshold u            GPD fit on excesses Y = X - u
                                              tail-index / shape ξ  ──► VaR_α, ES_α
```

**Source:** McNeil et al. (2015) Ch.5 §5.2 printed pp.147–149 (PDF pp.167–170).

## Definition
Two complementary model families:

- **Block maxima (GEV).** The only possible non-degenerate limit laws for normalized
  maxima M_n = max(X_1,…,X_n) form the generalized extreme value (GEV) family
  H_ξ, indexed by a shape parameter ξ (Frechet ξ>0, Gumbel ξ=0, Weibull ξ<0).
- **Excess distribution.** For threshold u, the excess df is
  F_u(x) = P(X − u ≤ x | X > u) = (F(x+u) − F(u))/(1 − F(u)).
- **Mean-excess function.** e(u) = E(X − u | X > u), the mean of F_u as a function
  of u.
- **Generalized Pareto distribution (GPD)** G_{ξ,β}, the limiting model for the
  excess distribution over a high threshold.
- **Tail index / Hill.** When F̄(x) = L(x)x^{−α} with L slowly varying, the tail
  index is α = 1/ξ, estimated by the Hill estimator from the upper order statistics.

**Source:** McNeil et al. (2015) Ch.5 §5.2.1 printed pp.148–149 (PDF pp.169–170).

## Mathematical Reasoning
The engine is the **Pickands-Balkema-de Haan theorem**: there exists a
positive-measurable scaling function β(u) such that
sup_{0≤x<x_F−u} |F_u(x) − G_{ξ,β(u)}(x)| → 0 as u → x_F **iff** F ∈ MDA(H_ξ).
So the distributions whose normalized maxima converge to a GEV law are exactly those
whose excess distribution converges to the GPD as the threshold is raised, with the
*same* shape parameter ξ for tail and maxima.

The GPD is self-stabilizing under thresholding: if F = G_{ξ,β} then
F_u(x) = G_{ξ,β+ξu}(x), so the excess of an excess is again GPD with the same ξ and
a scale that grows linearly in u. This makes the **mean-excess function linear**,
e(u) = (β + ξu)/(1−ξ) for 0 ≤ ξ < 1 — the linearity is a *characterizing* property
and the basis of the mean-excess-plot threshold diagnostic (upward slope ⇒ ξ>0,
flat ⇒ exponential tail, downward ⇒ ξ<0).

Given Assumption 5.21 (F_u is exactly GPD above u), tail probabilities invert to a
high quantile read as VaR, and ES follows from integrating the quantile; far into
the tail ES_α/VaR_α → (1−ξ)^{−1} for 0 ≤ ξ < 1. The **POT model** embeds this in a
point-process framework: exceedances arrive as a Poisson process and excess amounts
are iid GPD. The **Hill estimator** α̂^{(H)}_{k,n} is built from the log-spacings of
the top k order statistics and, under standard model assumptions, is consistent and
asymptotically normal as n → ∞, k → ∞, k/n → 0.

```
  e(u)  mean-excess
   ^            .-' slope ξ/(1-ξ)  (linear ⇒ GPD tail)
   |        .-'
   |    .-'
   |.-'
   +----------------------> u (threshold)
```

**Source:** McNeil et al. (2015) Ch.5 §5.2.1–5.2.3 printed pp.149–157 (PDF pp.169–179).

## See Also
- [rm-cramer-lundberg-heavy-tail-ruin](./rm-cramer-lundberg-heavy-tail-ruin.md) — the heavy-tail / ruin mechanism for fat tails (contrast).
- [rm-fisher-tippett-ev-types](./rm-fisher-tippett-ev-types.md) — the EKM extreme-value-types limit law underlying the GEV here.
- [rm-levy-stable-paretian-tails](./rm-levy-stable-paretian-tails.md) — the Levy / stable-Paretian mechanism for fat tails (contrast).
- [rm-crash-as-critical-point](./rm-crash-as-critical-point.md) — the Sornette criticality mechanism for fat tails (contrast).
- [rm-expected-shortfall-mechanics](./rm-expected-shortfall-mechanics.md) — the ES this card feeds deep in the tail.

## Escalate to Raw When
You need the worked GPD/ML fits (the Danish fire ξ̂, β̂ estimates and standard
errors), the fitted 99% VaR and ES point estimates, the Hill-plot numerics, or the
explicit estimation recipe using F̄(u) ≈ N_u/n — those calibrated numbers and worked
tail tables live in the raw text (Rule 1).

**Source:** McNeil et al. (2015) Ch.5 §5.2 printed pp.135–167 (PDF pp.156–179).
