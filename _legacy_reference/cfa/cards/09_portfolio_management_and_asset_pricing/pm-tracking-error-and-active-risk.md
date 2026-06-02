---
Use when: defining active return as the portfolio-minus-benchmark difference and active risk (tracking error) as the volatility of that difference, and locating these quantities in the active-management evaluation framework
Primary raw source: CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.6/pp.560-572
Supporting sources:
  - CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.6/pp.420-441
Repo touchpoints:
  - .claude/knowledge/09_portfolio_management_and_asset_pricing/pm-active-vs-passive-decision.md
  - .claude/knowledge/09_portfolio_management_and_asset_pricing/pm-performance-ratios-definitions.md
  - .claude/knowledge/09_portfolio_management_and_asset_pricing/pm-systematic-vs-idiosyncratic-risk.md
Out of scope: VaR / parametric / historical / Monte Carlo risk-metric machinery (see [`../11_risk_management/rm-value-at-risk-notes.md`](../11_risk_management/rm-value-at-risk-notes.md) for VaR at L1-notes depth, [`../11_risk_management/rm-historical-simulation-var.md`](../11_risk_management/rm-historical-simulation-var.md) for the historical-simulation route, and [`../11_risk_management/rm-monte-carlo-var.md`](../11_risk_management/rm-monte-carlo-var.md) for the Monte Carlo route); tracking-error attribution to allocation vs selection vs interaction (deferred to future-15); deeper Pedersen / Grinold-Kahn fundamental-law-of-active-management treatment (deferred to AC-42 extension card)
CFA Relevance: core
Source Stance: primary-cfa
deliverable-ready: true
---

# Tracking Error and Active Risk

## Intuition

When a portfolio is evaluated against a benchmark rather than
against the risk-free rate, two new quantities matter: the active
return — how much the portfolio earned over the benchmark — and the
active risk — how variable the portfolio-minus-benchmark difference
has been. Tracking error is the standard deviation of the active-
return time series. A portfolio that perfectly replicates its
benchmark has zero active return and zero tracking error; a
portfolio that aggressively deviates has nonzero active return (in
either direction) and nonzero tracking error. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.560-572.

```
        portfolio return:   r_p(time)
        benchmark return:   r_b(time)

        active return at time t:
        a(t) = r_p(t) - r_b(t)

        active risk (tracking error) over a window of T periods:
        TE = sqrt( var(a) ) = sigma_(r_p - r_b)

        +---------+         +---------+        +---------+
        |  r_p    |    -    |  r_b    |   =    |   a     |
        |  series |         |  series |        |  series |
        +---------+         +---------+        +---------+

        average of a series        --> active return
        std deviation of a series  --> tracking error
```

The two measures differ from total return and total volatility in
their reference point. Total volatility measures variability around
the asset's own mean; tracking error measures variability around
the benchmark. A high-volatility portfolio that closely matches its
benchmark may have low tracking error; a low-volatility portfolio
that drifts from its benchmark may have meaningfully positive
tracking error. The benchmark choice is therefore decisive: the
same portfolio has different tracking errors against different
benchmarks. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.560-572.

## Definition

Active return is the time-series difference between portfolio
return and benchmark return at the same evaluation horizon.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.560-572.

```
a(t) = r_p(t) - r_b(t)
```

The expected active return — sometimes labeled "alpha" in the
context of an active manager — is the long-run average. **Source:**
CFA L1 Curriculum (2022) Vol.6/pp.560-572.

```
E[a] = E[r_p] - E[r_b]
```

Tracking error is the standard deviation of the active-return
series, equivalently written as the standard deviation of the
portfolio-minus-benchmark difference. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.560-572.

```
TE = sigma_a = sqrt( var( r_p - r_b ) )
```

Active risk and tracking error are typically used as synonyms at
L1 framing. The Information Ratio defined in the performance-ratios
sibling is the active-return / tracking-error quotient — the
performance-per-unit-of-active-risk metric. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.560-572.

## Mathematical Reasoning

Tracking error decomposes into the portfolio and benchmark
volatilities and their correlation. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.560-572.

```
var(r_p - r_b) = var(r_p) + var(r_b) - 2 · cov(r_p, r_b)
              = sigma_p^2 + sigma_b^2 - 2 · rho_(p,b) · sigma_p · sigma_b
```

When the portfolio is identical to the benchmark, `rho_(p,b) =
1` and `sigma_p = sigma_b`, and the tracking error is exactly
zero. Tracking error is monotonically decreasing in `rho_(p,b)`:
at `rho = 0` it equals the quadrature sum
`sqrt(sigma_p^2 + sigma_b^2)`; at `rho = -1` it reaches the
maximum simple sum `sigma_p + sigma_b`, applicable to inverse /
short benchmark-relative positions where the portfolio is
constructed to move against the benchmark. Real long-only active
portfolios sit near the `rho ≈ +1` corner — high positive
correlation with the benchmark but a small leftover deviation
that drives tracking error nonzero. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.560-572.

The relationship between tracking error and the portfolio's beta
to the benchmark is informative. If the portfolio has beta `beta_p`
to the benchmark and residual variance `var(epsilon_p)` relative
to that benchmark, the tracking error has two components.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.560-572.

```
TE^2 = (beta_p - 1)^2 · var(r_b) + var(epsilon_p)
       ^                                 ^
       |                                 +-- residual / selection risk
       +-- systematic-tilt risk
            (beta away from 1.0)
```

The first term is the systematic-tilt component: a portfolio with
beta `beta_p` different from `1.0` against the benchmark adds
tracking risk proportional to the squared deviation. The second
term is the residual / selection component: stock-picking choices
within asset-class allocations contribute idiosyncratic deviation
from the benchmark. The two components are typically called
allocation risk and selection risk in attribution language; the
deeper attribution treatment belongs to future-15. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.560-572.

The IPS records a tracking-error budget `TE_max` that the manager
must respect. The tracking-error budget is the policy constraint
on how aggressively the manager may deviate from the benchmark in
pursuit of alpha. A narrow budget constrains the manager to a
near-index strategy with modest tilts; a wide budget permits
high-conviction concentration. The budget choice reflects the
investor's belief about manager skill: confidence in positive
`E[alpha]` justifies a wider budget. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.420-441.

A specific implication for evaluation: tracking error and total
volatility are not interchangeable. The tracking error depends on
the correlation between portfolio and benchmark returns, so it
cannot be obtained by subtracting the benchmark's volatility from
the portfolio's volatility. The two-volatility-difference heuristic
is wrong whenever the correlation is below `1`, which is always the
case in practice. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.560-572.

## See Also

- [`pm-active-vs-passive-decision.md`](pm-active-vs-passive-decision.md) — the upstream choice that determines whether tracking error is policy-relevant at all
- [`pm-performance-ratios-definitions.md`](pm-performance-ratios-definitions.md) — the Information Ratio that uses tracking error as its denominator
- [`pm-systematic-vs-idiosyncratic-risk.md`](pm-systematic-vs-idiosyncratic-risk.md) — variance decomposition that mirrors the allocation / selection split inside tracking-error attribution

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R50 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.560-572.

- Brinson allocation / selection / interaction attribution that
  decomposes active return by source — Vol.6 R50 introduces the
  framework; deeper attribution belongs in future-15. **Source:**
  CFA L1 Curriculum (2022) Vol.6/pp.560-572.
- Pedersen / Grinold-Kahn fundamental-law-of-active-management
  framework (`IR = IC · sqrt(BR)`) — this links tracking error to
  manager-skill and breadth at theoretical depth that lives in the
  AC-42 extension card. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.560-572.
- Estimation issues for tracking error (small-sample bias, time-
  varying volatility, regime change) — these belong in future-01
  quantitative methods. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.560-572.
