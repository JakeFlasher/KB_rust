---
schema_version: "cacg.v0"
id: "rm-filtered-historical-simulation"
title: "Filtered Historical Simulation (FHS)"
reading_id: "11_risk_management"
summary: "FHS standardizes returns by a GARCH variance forecast to extract i.i.d. shocks, resamples those shocks model-free, and re-inflates by tomorrow's forecast variance to compute VaR/ES at any horizon, per Christoffersen Ch.6."
tags: ["risk-management", "filtered-historical-simulation", "garch"]
citations:
  - source_id: "rm_christoffersen_2012_elements"
    chunk_id: "rm_christoffersen_2012_elements:p134:0178"
    chunk_hash: "57be0b41eb6f327d2b7f21a16c6830fd749537af3d82ac560be497bd509a04d3"
    page_range: [134, 134]
    quote: "FHS combines model-based methods of dynamic variance, such as GARCH, with model-free methods of distribution"
    edge_type: "defines"
card_hash: "e961b4bb49acfc3987f81cef710e4fb7747ce4b7fb7f52988366f30f2ed08a58"
---
# Filtered Historical Simulation (FHS)

## Intuition
Plain Historical Simulation assumes tomorrow's returns are drawn from the same
distribution as the past raw returns — so it inherits whatever volatility regime
happened to prevail in the window and reacts slowly when the market shifts. FHS splits
the problem in two: it trusts a dynamic variance model (GARCH) to say *how big*
tomorrow's move will be, but stays agnostic about the *shape* of the shock distribution,
letting the data's own standardized residuals speak. The payoff: it can generate a large
loss tomorrow even if no large *return* was ever observed — by pairing a big past shock
with a high forecast variance.

```
  raw returns R_t ──► GARCH ──► σ_t ──► standardize  z_t = R_t/σ_t  (i.i.d. shocks)
                                                          │
                                                resample {ẑ} model-free
                                                          │
                                  re-inflate:  R*_{t+1} = σ_{t+1} · ẑ  ──► VaR / ES
```

**Source:** Christoffersen (2012) Ch.6 §4 printed pp.124–126 (PDF pp.134–136).

## Definition
FHS is the procedure that combines a conditional variance model with a model-free shock
distribution. Given a fitted GARCH-type model for portfolio variance,

```
R_{PF,t+1} = σ_{PF,t+1} z_{t+1},   σ²_{PF,t+1} = ω + α R²_{PF,t} + β σ²_{PF,t},
```

extract the standardized residuals over the window τ = 1,…,m:

```
ẑ_{t+1−τ} = R_{PF,t+1−τ} / σ_{PF,t+1−τ}.
```

These `{ẑ}` are treated as i.i.d. draws from the unknown shock distribution. The 1-day
VaR and ES at coverage `p` are read off the empirical percentile of the shock database,
scaled by the one-step variance forecast:

```
VaR^p_{t+1} = −σ_{PF,t+1} · Percentile({ẑ}, 100p),
ES^p_{t+1}  = −σ_{PF,t+1} · (average of ẑ below that percentile).
```

**Source:** Christoffersen (2012) Ch.6 §4 printed pp.125–126 (PDF pp.135–136).

## Mathematical Reasoning
FHS retains conditionality through `σ_{t+1}` while discarding the parametric
distribution assumption. The only assumption is that the historical sample of
standardized shocks `{ẑ}` describes the distribution of future shocks `z`. This is
strictly weaker than the HS assumption that the sample of raw returns `{R}` describes
the distribution of future raw returns, because standardizing removes the
variance-clustering nonstationarity before resampling. For a K-day horizon, one draws K
i.i.d. shocks from `{ẑ}`, feeds each through the GARCH recursion to simulate the path of
σ, and aggregates the K simulated returns — so the model-based variance dynamics
propagate the horizon while the shock distribution stays model-free. Because a large
negative `ẑ` recorded on a low-variance day can be re-applied to a high-variance day,
the simulated loss can exceed any loss in the recorded past.

**Source:** Christoffersen (2012) Ch.6 §4 printed pp.125–126 (PDF pp.135–136); the K-day multi-step FHS path simulation (drawing/propagating shocks through the GARCH recursion and aggregating) is developed in Ch.8 (Multi-Period Risk) §8.2 printed pp.180–183 (PDF pp.186–189).

## See Also
- [rm-garch-conditional-variance](./rm-garch-conditional-variance.md) — supplies the σ_{t+1} forecast that FHS re-inflates by.
- [rm-historical-simulation-var](./rm-historical-simulation-var.md) — the model-free baseline FHS improves on.
- [rm-monte-carlo-var](./rm-monte-carlo-var.md) — alternative simulation-based VaR that draws from a parametric model instead.
- [rm-expected-shortfall-mechanics](./rm-expected-shortfall-mechanics.md) — the ES measure FHS also computes from the shock database.

## Escalate to Raw When
You need the worked Cornish-Fisher comparison figures or any numeric percentile/shortfall
calculation on a specific S&P 500 shock database — those plug-and-chug examples live in
the raw text (Rule 1).

**Source:** Christoffersen (2012) Ch.6 §4–5 printed pp.124–127 (PDF pp.134–137).
