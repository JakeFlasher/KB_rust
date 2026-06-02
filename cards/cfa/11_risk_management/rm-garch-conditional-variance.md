---
schema_version: "cacg.v0"
id: "rm-garch-conditional-variance"
title: "The GARCH(1,1) Conditional-Variance Engine"
reading_id: "11_risk_management"
summary: "GARCH(1,1) models tomorrow's variance as a mean-reverting mix of long-run variance, today's squared return, and today's variance; it nests RiskMetrics and yields a multi-day variance term structure, per Christoffersen Ch.4."
tags: ["risk-management", "garch", "volatility-modeling"]
citations:
  - source_id: "rm_christoffersen_2012_elements"
    chunk_id: "rm_christoffersen_2012_elements:p080:0101"
    chunk_hash: "d567b5ec44c41e14e484082c4f11e1b7e14100d87efb4c36a1340cd725cd123b"
    page_range: [81, 81]
    quote: "the RiskMetrics model can be viewed as a special case of the simple GARCH model"
    edge_type: "defines"
  - source_id: "rm_christoffersen_2012_elements"
    chunk_id: "rm_christoffersen_2012_elements:p081:0103"
    chunk_hash: "e8a4e5120d81391badbd595f50ecfa56b952411c1072eef0a081c68d4b35abd4"
    page_range: [81, 81]
    quote: "We will refer to α + β as the persistence of the model"
    edge_type: "supports"
card_hash: "e57b9f51cdf6aeb86488e3c17812b33c98ad242612a14a6a57316a8d842f1931"
---
# The GARCH(1,1) Conditional-Variance Engine

## Intuition
Volatility clusters: calm days follow calm days, turbulent days follow turbulent days.
A naive rolling-window variance reacts mechanically — an extreme return inflates the
estimate for exactly `m` days then drops it off a cliff. GARCH instead lets variance
decay smoothly and, crucially, pulls it back toward a stable long-run level. The
RiskMetrics exponential smoother is the degenerate special case with no mean reversion;
GARCH adds the missing anchor.

```
   variance forecast σ²_{t+k}
   ^
   |  shock
   |   \
   |    \__               GARCH: reverts toward σ̄²
   |       \___
   |           \_____________________  σ̄²  (long-run variance)
   |  ........................................ RiskMetrics: stays at σ²_{t+1}
   +-------------------------------------------> horizon k
```

**Source:** Christoffersen (2012) Ch.4 §2–3 printed pp.69–71 (PDF pp.79–81).

## Definition
The simple GARCH(1,1) recursion for next-day variance is

```
σ²_{t+1} = ω + α R²_t + β σ²_t ,   with α + β < 1, and ω, α, β > 0.
```

`R²_t` is today's squared return, `σ²_t` today's variance. RiskMetrics is recovered by
forcing `α = 1−λ`, `β = λ` (so `α + β = 1`) and `ω = 0`. The sum `α + β` is the
**persistence**. Asymmetric extensions (NGARCH, GJR-GARCH, EGARCH) replace the symmetric
news-impact `R²_t` with a leverage-sensitive function — e.g. NGARCH uses
`α σ²_t (z_t − θ)²` so a negative shock (`z_t < 0`) raises variance more than a positive
one of equal size; the general news-impact function `NIF(z_t)` nests these, with the
symmetric GARCH `NIF(z_t) = z²_t` as the baseline.

**Source:** Christoffersen (2012) Ch.4 §3, §5 printed pp.70–77 (PDF pp.80–87).

## Mathematical Reasoning
Taking unconditional expectations of the recursion and using stationarity
(`E[σ²_{t+1}] = E[σ²_t] ≡ σ̄²`, `E[R²_t] = σ̄²`):

```
σ̄² = ω + α σ̄² + β σ̄²   ⇒   σ̄² = ω / (1 − α − β).
```

So the long-run variance is well defined only when `α + β < 1`; in RiskMetrics
(`α + β = 1`) it is undefined, which is why that model ignores mean reversion.
Substituting `ω = (1 − α − β) σ̄²` back gives the mean-reverting form

```
σ²_{t+1} = σ̄² + α (R²_t − σ̄²) + β (σ²_t − σ̄²),
```

i.e. tomorrow's variance is the long-run average plus deviations of today's squared
return and today's variance from that average. The k-step-ahead forecast follows by
recursion. Writing `E_t[σ²_{t+k}] − σ̄²`, each application of the dynamics multiplies the
gap by `(α + β)`:

```
E_t[σ²_{t+k} − σ̄²] = (α + β)^{k−1} (σ²_{t+1} − σ̄²),
```

so forecasts decay geometrically toward `σ̄²` at rate equal to the persistence. For the
variance of K-day cumulative returns (zero-autocorrelation returns) the two models
diverge sharply:

```
RiskMetrics:  σ²_{t+1:t+K} = K σ²_{t+1}
GARCH:        σ²_{t+1:t+K} = K σ̄² + Σ_{k=1}^{K} (α+β)^{k−1} (σ²_{t+1} − σ̄²) ≠ K σ²_{t+1}.
```

When today is calm (`σ²_{t+1} < σ̄²`), the GARCH cumulative forecast exceeds the
RiskMetrics one — so a RiskMetrics user gets a false sense of long-horizon calm.

**Source:** Christoffersen (2012) Ch.4 §3 printed pp.71–73 (PDF pp.81–83).

## See Also
- [rm-filtered-historical-simulation](./rm-filtered-historical-simulation.md) — uses GARCH variance to inflate model-free shocks.
- [rm-dynamic-conditional-correlation-dcc](./rm-dynamic-conditional-correlation-dcc.md) — extends GARCH from variance to correlation dynamics.
- [rm-value-at-risk-notes](./rm-value-at-risk-notes.md) — VaR built on a conditional-variance forecast.
- [rm-evt-gpd-pot-hill](./rm-evt-gpd-pot-hill.md) — EVT applied to the GARCH-standardized shock tail.

## Escalate to Raw When
You need the QMLE-estimated parameter set (the worked S&P 500 dynamics with the fitted
ω, α, β values and the resulting near-unit persistence) or the multi-day
cumulative-variance term-structure figure — those numeric recipes and the worked example
live in the raw text (Rule 1).

**Source:** Christoffersen (2012) Ch.4 §4–5 printed pp.73–82 (PDF pp.83–92).
