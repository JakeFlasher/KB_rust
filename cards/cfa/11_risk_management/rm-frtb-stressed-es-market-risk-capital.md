---
schema_version: "cacg.v0"
id: "rm-frtb-stressed-es-market-risk-capital"
title: "FRTB: Stressed Expected Shortfall as the Market-Risk Capital Engine"
reading_id: "11_risk_management"
summary: "Basel's Fundamental Review of the Trading Book replaces 99% VaR with stressed expected shortfall at 97.5%, buckets risk factors into five liquidity horizons, and runs parallel standardized (delta/vega/curvature + default + residual) and internal-models approaches, per Hull Ch.27."
tags: ["risk-management", "frtb", "market-risk-capital"]
citations:
  - source_id: "rm_hull_2023_rmfi"
    chunk_id: "rm_hull_2023_rmfi:p614:0848"
    chunk_hash: "c7278460f4c7da857a11a91dbf4e1e24d284d44416282c5857e0bb0e8c0768d8"
    page_range: [614, 614]
    quote: "Instead of VaR with a 99% confidence level, it uses expected shortfall (ES) with a 97.5% confidence level."
    edge_type: "defines"
card_hash: "c397d8eea6b4a9692df56f3471efee24ba1f0e4c1df9d20a39b0dc636c4eb5f5"
---
# FRTB: Stressed Expected Shortfall as the Market-Risk Capital Engine

## Intuition
Basel I and II.5 set market-risk capital off a 99% VaR over a fixed 10-day horizon.
The 2008 crisis exposed two weaknesses: VaR ignores how bad losses are *beyond* the
cutoff, and a single horizon pretends every position can be unwound in ten days. The
Fundamental Review of the Trading Book (FRTB) re-engineers the capital measure to fix
both — it switches the risk measure to **expected shortfall** (averaging the tail, not
reading one quantile), stresses it (calibrated to a historical period of market
turmoil for the current book), and lets the unwind horizon scale with each risk
factor's liquidity. It also runs a standardized approach in parallel with internal
models so the standardized number can act as a capital floor.

```
   loss density
   ^
   |        ___
   |      /     \   97.5% ES = average loss in the worst 2.5% tail
   |____/        \____......> L
   +----------|---------
              VaR_97.5   (ES integrates everything to the right)
```

**Source:** Hull (2023) Ch.27 §27.1 printed pp.585–586 (PDF pp.613–614).

## Definition
- **Risk measure.** Market-risk capital is driven by *stressed expected shortfall at
  a 97.5% confidence level* — calculations are based on how risk factors moved during
  a stressed period (as in stressed VaR), not the immediately preceding window.
- **Liquidity horizons.** Risk factors ("risk factors" = market variables; their
  moves are "shocks") are assigned to one of **five liquidity horizons**: 10, 20, 40,
  60, and 120 days, reflecting how quickly each factor's position can be exited.
- **Two approaches, both required.** FRTB specifies a *standardized approach* and an
  *internal models approach*; even banks approved for internal models must also
  compute the standardized number, which provides a floor.
- **Standardized capital = three components.** (1) a risk-sensitivity charge =
  delta + vega + curvature charges across seven risk classes; (2) a *default risk
  charge* (jump-to-default, via LGD × default risk weight); (3) a *residual risk
  add-on* (for exotics not captured by delta/vega/curvature).

**Source:** Hull (2023) Ch.27 §27.1–27.3 printed pp.586–591 (PDF pp.614–619).

## Mathematical Reasoning
**Why ES@97.5 ≈ VaR@99 for normals.** For a normal loss with mean μ and standard
deviation σ, the 99% VaR is μ + z₀.₉₉ σ and the 97.5% ES is μ + (φ(z)/(1−X)) σ; with
the standardized critical values these multipliers are nearly equal, so

    VaR_0.99(normal) ≈ ES_0.975(normal).

The two are *not* equivalent off the normal: when the loss law has a heavier-than-normal
tail, ES_0.975 ≥ VaR_0.99 (strict), because ES averages the deeper tail the quantile
ignores. That inequality is the whole motivation for the switch — it makes capital
sensitive to tail thickness.

**Delta risk charge form.** Within a risk class the delta charge aggregates weighted
sensitivities with regulator-set correlations,

    DeltaRiskCharge = sqrt( Σ_i Σ_j ρ_ij δ_i δ_j W_i W_j ),

where δ_i are bank-supplied sensitivities and the risk weights W_i are set as multiples
of stressed daily volatility scaled by the liquidity horizon and the ES confidence
level. Vega risk is handled identically with volatilities as the risk factors;
curvature captures the gamma effect (the net-of-delta loss under up/down shocks of
size W_i, floored at zero). There are assumed to be no diversification benefits
across the seven risk classes.

**Source:** Hull (2023) Ch.27 §27.1–27.2.3 printed pp.586–591 (PDF pp.614–619).

## See Also
- [rm-expected-shortfall-mechanics](./rm-expected-shortfall-mechanics.md) — the ES tail-average this capital engine uses.
- [rm-var-and-es-taxonomy](./rm-var-and-es-taxonomy.md) — where VaR vs ES sits in the measure taxonomy.
- [rm-basel-capital-accord-evolution](./rm-basel-capital-accord-evolution.md) — the accord lineage FRTB caps off.
- [rm-stress-testing](./rm-stress-testing.md) — the stressed-period calibration logic FRTB inherits.
- [rm-delta-gamma-vega-pl-decomposition](./rm-delta-gamma-vega-pl-decomposition.md) — the delta/vega/curvature sensitivities the standardized charge weights.

## Escalate to Raw When
You need the worked risk-weight arithmetic (the stressed-volatility × horizon-scaling ×
percentile-multiplier example), the specified LGD and default-risk-weight numbers, the
curvature scenario geometry, or the liquidity-adjusted-ES aggregation formula combining
the per-bucket ES terms — those numeric recipes live in the raw text (Rule 1).

**Source:** Hull (2023) Ch.27 printed pp.585–595 (PDF pp.613–623).
