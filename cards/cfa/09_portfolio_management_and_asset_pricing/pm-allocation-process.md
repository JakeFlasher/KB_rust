---
schema_version: "cacg.v0"
id: "pm-allocation-process"
title: "Asset Allocation Process"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Asset Allocation Process: distinguishing strategic asset allocation (long-run target weights derived from the IPS and the efficient frontier) from tactical asset allocation (short-run deviations responding to market views), and locating allocation in the portfolio-management process"
tags: ["portfolio-management", "asset-allocation", "ips"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3292:4957"
    chunk_hash: "3f4ebac1951945cbbb1ec57db74b7f78efdb84f8c08db973be265487777b9403"
    page_range: [3292, 3292]
    quote: "The strategic asset allocation (SAA) is the set of exposures to IPS-permissible asset classes that is expected to achieve the client’s long-term objectives given the client’s risk profile and investment constraints."
    edge_type: "defines"
card_hash: "d41a0cb4954334ffedb0de18821986f5a1486f390920758562f1a1c5077179d7"
---
# Asset Allocation Process

## Intuition

Asset allocation is the choice of how much capital sits in each broad
asset class. The strategic layer is the long-run answer: target
weights chosen so the resulting portfolio sits on the efficient
frontier given the IPS-stated objectives and constraints. The
tactical layer is short-run deviation around those targets in
response to market views or relative-value signals. The tactical
deviation must remain within bands set in the IPS; outside those
bands, a deviation is a policy revision rather than a tactical bet.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.20-40.

```
<!-- primitive: allocation-waterfall source: _diagram_primitives.md -->
total portfolio
   ^
   |  +---------------+
   |  |     cash      |    <-- liquidity buffer
   |  +---------------+
   |  |               |
   |  |  fixed income |    <-- bonds (govt, corporate, securitized)
   |  |               |
   |  +---------------+
   |  |               |
   |  |               |
   |  |    equity     |    <-- public equity (domestic + intl)
   |  |               |
   |  |               |
   |  +---------------+
   |  | alternatives  |    <-- PE, RE, commodities, hedge funds
   |  +---------------+
   |
   +-> strategic-allocation breakdown (rung heights are conceptual,
       not specific weights)
```

The dominant empirical finding cited by the curriculum is that
strategic allocation explains the bulk of cross-portfolio return
variation; security selection and tactical timing are secondary
contributors at the L1 framing. The implication is that the
allocation choice is the highest-leverage decision in the
portfolio-management process, which is why it sits immediately
downstream of the IPS. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.20-40.

## Definition

The strategic asset allocation is the long-run target weight vector
`w_SAA = (w_1, ..., w_K)` over `K` asset classes, chosen to maximize
the IPS-implied utility on the efficient frontier subject to the
LLTTU constraint set. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.20-40.

```
w_SAA = argmax_{w in W_IPS} U(E[r_p(w)], var(r_p(w)))
```

The tactical asset allocation is the short-run actual weight vector
`w_TAA(time)` that may deviate from `w_SAA` within IPS-permitted
tolerance bands. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.20-40.

```
w_TAA(time) = w_SAA + delta(time)     with  ||delta||_inf <= band_IPS
```

Here `delta(time)` is the tactical-deviation vector and `band_IPS`
is the per-asset-class deviation tolerance recorded in the IPS.
Rebalancing is the discipline that drives `w_actual` back toward
`w_SAA + delta(time)` when market drift pushes the realized weights
outside band tolerances; the rebalancing mechanics are covered in a
sibling card. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.20-40.

The allocation step distinguishes asset classes from securities. An
asset class is a group of holdings with internally homogeneous
risk-return characteristics that differ meaningfully from other
classes — public equity, fixed income, real estate, commodities,
cash. Selection within an asset class (which equity, which bond)
is a downstream step deferred to equity / fixed-income verticals.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.20-40.

## Mathematical Reasoning

Strategic allocation inherits the mean-variance optimization
framework from the efficient-frontier card. Given expected returns
`mu = (E[r_1], ..., E[r_K])`, covariance matrix `Sigma`, and risk-
aversion parameter `A`, the unconstrained mean-variance optimum is
the well-known solution. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.490-519.

```
w*_unconstrained = (1 / A) · Sigma^(-1) · (mu - R_f · 1)
```

The constrained problem replaces the simplex with `W_IPS`, which
adds linear inequality constraints (no shorts in retail accounts;
maximum sector exposure; minimum liquidity) and possibly equality
constraints (cash floor; tax-lot harvest). The constrained solution
is the projection of `w*_unconstrained` onto `W_IPS` under the
mean-variance objective. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.20-40.

Tactical allocation is a perturbation around `w_SAA`. The expected
incremental contribution from a tactical deviation `delta(time)` is
linear in the manager's expected-return forecast, while the
incremental risk is quadratic in `delta(time)`. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.20-40.

```
E[r_TAA] - E[r_SAA] = delta' · (mu_forecast - mu_SAA)
var(r_TAA) - var(r_SAA) ≈ 2 · w_SAA' · Sigma · delta + delta' · Sigma · delta
```

For small `delta`, the first-order risk term dominates and the
tactical deviation acts like an active bet against the strategic
benchmark. The Information Ratio framework — covered in a
performance-ratios sibling — measures whether the tactical bet
delivers expected return per unit of incremental tracking risk.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.20-40.

A specific implication: a tactical band `band_IPS` that is too wide
turns the strategic allocation into an advisory anchor rather than
a binding policy; a band that is too narrow eliminates the manager's
ability to express any view at all. The L1 framing treats
band-setting as part of IPS construction, with the bands chosen so
that fully-utilized tactical deviations are still within the IPS-
risk-objective tolerance. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.20-40.

## See Also

- [`pm-investment-policy-statement.md`](pm-investment-policy-statement.md) — the upstream document that fixes `W_IPS` and `band_IPS`
- [`pm-efficient-frontier.md`](pm-efficient-frontier.md) — the geometric object on which `w_SAA` lands by construction
- [`pm-rebalancing-mechanics.md`](pm-rebalancing-mechanics.md) — the discipline that returns `w_actual` to `w_SAA + delta` when market drift exceeds band tolerance

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R51 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.20-40.

- Detailed asset-class definition criteria (the curriculum lists
  five) — when the boundary between two asset classes is unclear
  for a specific holding (e.g. high-yield convertible vs equity),
  the curriculum provides definition tests. **Source:** CFA L1
  Curriculum (2022) Vol.6/pp.20-40.
- Tactical-allocation signal construction beyond the framework above
  — momentum, valuation, macro-regime signals are active-management
  territory and the curriculum covers only the L1-process framing.
  **Source:** CFA L1 Curriculum (2022) Vol.6/pp.20-40.
- Glide-path construction for life-cycle / target-date portfolios —
  Vol.6 R51 introduces the concept; deeper construction belongs in
  future-13 wealth and institutional. **Source:** CFA L1 Curriculum
  (2022) Vol.6/pp.20-40.
