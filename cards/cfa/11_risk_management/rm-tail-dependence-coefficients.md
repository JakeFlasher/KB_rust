---
schema_version: "cacg.v0"
id: "rm-tail-dependence-coefficients"
title: "Coefficients of Tail Dependence"
reading_id: "11_risk_management"
summary: "Upper/lower tail-dependence coefficients lambda_u, lambda_l are limiting conditional quantile-exceedance probabilities depending only on the copula; the Gauss copula is asymptotically independent while the t and Gumbel copulas show positive tail dependence, per McNeil et al. (2015) Ch.7 §7.2.4."
tags: ["risk-management", "tail-dependence", "copulas"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p269:0386"
    chunk_hash: "8215c325c6896b38d6af35576d63f4587c745c0edf030031cfc9b4ededab158e"
    page_range: [269, 269]
    quote: "the Gauss copula is asymptotically independent in both tails, while the t copula has both upper and lower tail dependence of the same magnitude"
    edge_type: "defines"
card_hash: "aae24644ca50bf10ecb5de8a669754f74e6287f0bbcef091d646776deb94b071"
---
# Coefficients of Tail Dependence

## Intuition
Ordinary correlation tells you how two risk factors co-move *on average*; it is
silent about whether they crash *together*. Tail dependence answers the question a
risk manager actually cares about: given that one position blows through its extreme
quantile, what is the chance the other does too? This is a pure-dependence
diagnostic — it depends only on the copula, not on the margins — and it is what
separates a benign-looking correlation from a portfolio that experiences synchronized
extremes. The headline contrast is that a Gaussian dependence structure has *zero*
tail dependence (joint extremes decouple asymptotically), whereas t and Gumbel
copulas keep co-crashing all the way into the tail.

```
   X2
   ^
   |              ::          large simultaneous
   |            ::::   <----- exceedances persist
   |     . . : :::::          (positive tail dependence)
   | . . . . :::::
   |. . . . :::
   +-----------------------> X1
        both > their q-quantile, q -> 1
```

**Source:** McNeil et al. (2015) Ch.7 §7.2.4 printed pp.247–248 (PDF pp.268–269).

## Definition
For rvs X_1, X_2 with continuous dfs F_1, F_2, the **coefficient of upper tail
dependence** is
λ_u = lim_{q→1−} P( X_2 > F_2^←(q) | X_1 > F_1^←(q) ),
when the limit exists in [0,1]. If λ_u ∈ (0,1] the pair shows upper tail dependence
(extremal dependence in the upper tail); if λ_u = 0 they are **asymptotically
independent** in the upper tail. The **lower** coefficient is the analogous limit as
q → 0+ over joint *non-exceedances*. Both are functions of the copula only.

**Source:** McNeil et al. (2015) Ch.7 §7.2.4 printed p.247 (PDF p.268).

## Mathematical Reasoning
For continuous margins the unique copula C gives closed forms:
λ_l = lim_{q→0+} C(q,q)/q and λ_u = lim_{q→0+} Ĉ(q,q)/q, where Ĉ is the survival
copula. For a **radially symmetric** copula C = Ĉ, forcing λ_l = λ_u.

The contrasts follow directly:
- **Gaussian copula** — asymptotically independent in both tails (λ_u = λ_l = 0) for
  any correlation ρ < 1; extreme co-movement vanishes as you push into the tail.
- **t copula** — radially symmetric with strictly positive and equal upper and lower
  tail dependence; even with the *same* correlation as the Gauss copula it keeps
  co-crashing.
- **Gumbel copula** — for dependence parameter θ > 1, positive *upper* tail
  dependence whose strength → 1 as θ → ∞.
- **Clayton copula** — positive *lower* tail dependence.

This is why two models can be fitted to identical pairwise correlations yet imply
radically different joint-loss tails: the difference is entirely in λ_u, λ_l.

**Source:** McNeil et al. (2015) Ch.7 §7.2.4 printed pp.247–248 (PDF pp.268–269).

## See Also
- [rm-copulas-sklar-dependence](./rm-copulas-sklar-dependence.md) — the copula machinery these coefficients are read from.
- [rm-elliptical-spherical-distributions](./rm-elliptical-spherical-distributions.md) — the Gauss vs t copulas live in the elliptical world.
- [rm-evt-gpd-pot-hill](./rm-evt-gpd-pot-hill.md) — univariate tail estimation that this multivariate diagnostic complements.
- [rm-integrated-firm-wide-risk-aggregation](./rm-integrated-firm-wide-risk-aggregation.md) — where joint-extreme co-movement breaks naive diversification.

## Escalate to Raw When
You need the L'Hopital derivations of λ_u for the Gumbel/Clayton copulas, the
closed-form tail-dependence coefficient of the t copula as a function of (ρ, ν), or
the worked figures comparing Gauss/t/Gumbel/Clayton scatter clouds — those
calculations live in the raw text (Rule 1).

**Source:** McNeil et al. (2015) Ch.7 §7.2.4 printed pp.247–249 (PDF pp.268–270).
