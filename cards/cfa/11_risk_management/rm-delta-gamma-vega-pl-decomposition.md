---
schema_version: "cacg.v0"
id: "rm-delta-gamma-vega-pl-decomposition"
title: "Delta-Gamma-Vega P&L Decomposition — L1 Notes Portfolio-Risk-Reporting Framing"
reading_id: "11_risk_management"
summary: "Decomposes portfolio P&L into delta + gamma + vega Greek contributions plus a residual via the second-order Taylor expansion of the loss operator l_{[t]}, the same Taylor calculus that drives delta-gamma sensitivity-based VaR and risk reporting."
tags: ["risk-management", "delta-gamma"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p348:0496"
    chunk_hash: "9bb049e3a48ec5bce84097dcbe277bf3e2d0b3d184710401a82df71dc35c9f7e"
    page_range: [348, 349]
    quote: "We can also develop a second-order Taylor series, or so-called delta–gamma, approximation."
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p349:0497"
    chunk_hash: "c9fb854409f037fc7c1adec65529adcfe24bc0e880c7974caf73fd60a03e25a8"
    page_range: [349, 350]
    quote: "In Example 9.1 below we give an application of the delta–gamma approximation (9.9)."
    edge_type: "supports"
card_hash: "52fc3d8fde0dc8b381124ec8ab10ad98e0540ab675a5cdaccc710e3b674a5481"
---
# Delta-Gamma-Vega P&L Decomposition — L1 Notes Portfolio-Risk-Reporting Framing

## Intuition

For a portfolio of derivative or derivative-sensitive positions, the L1 source frame the **risk-reporting P&L decomposition** as: split the realised one-period P&L `ΔV` into a **delta contribution** (linear in underlying price moves), a **gamma contribution** (quadratic in underlying moves — convexity), a **vega contribution** (linear in implied-volatility moves), and a **residual** absorbing theta, rho, and higher-order cross-Greeks. This is the same Taylor expansion that drives sensitivity-based VaR (see `[[rm-sensitivity-versus-simulation]]`), but reframed as **attribution** of an observed P&L rather than prediction of a tail P&L. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.327-329.

The source-side framing emphasises three uses. (1) **Daily P&L attribution**: each desk reports its realised P&L decomposed into delta / gamma / vega / residual so the risk function can verify the P&L "explained" by the Greeks matches the books-and-records P&L. A large residual is a **diagnostic signal** — typically pointing to a model-mismatch, a missing risk factor, or an operational booking error. (2) **Risk-limit consumption**: per-desk Greek limits (delta budget, gamma budget, vega budget) are consumed by the decomposition; the risk function aggregates desk-level Greeks to firm-wide Greek exposures. (3) **Hedge-effectiveness check**: a delta-hedged book should show near-zero delta contribution day-over-day; persistent non-zero delta P&L flags broken-hedge slippage. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.327-329.

The boundary against 07 Derivatives is sharp: 07 owns the **instrument-level** Greek derivations (`deriv-greeks-overview.md`, `deriv-delta-and-hedging.md`) — closed-form Black-Scholes Greeks, binomial-tree Greeks, exotic-option Greek computation. 11 owns the **portfolio-aggregation** framing — how desk-level Greeks aggregate to firm-level risk-reporting metrics. The same partial-derivatives appear; the framing differs by aggregation level and use-case. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.327-329.

```
<!-- primitive: pl-distribution-decomposition source: _diagram_primitives.md -->
   density f_ΔV(v)
   ^
   |            * *  total P&L = ΔV
   |          *      *
   |        *           *
   |      *                *
   |    *                    *
   |  *                        *
   |*                            *
   +------------------------------------> ΔV

   ΔV   ≈   Δ · ΔS    +    1/2 · Γ · (ΔS)^2    +    ν · Δσ    +    ε
          (delta)         (gamma curvature)       (vega)        (residual)

   * delta contribution: linear in underlying move ΔS
   * gamma contribution: convexity term (non-negative for long-option books)
   * vega contribution:  sensitivity to volatility shift Δσ
   * residual ε:         theta, rho, higher-order cross-Greeks
```

## Definition

Let `V(S, σ, t, …)` be the portfolio value as a function of underlying price `S`, implied volatility `σ`, calendar time `t`, and any other state variables. The L1 source' **delta / gamma / vega P&L decomposition** over one period (with state-variable changes `ΔS`, `Δσ`, `Δt`) is the second-order Taylor expansion: **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.327-329 + McNeil et al. (2015) Ch.9 pp.327-329.

```
ΔV  ≈  Δ · ΔS  +  1/2 · Γ · (ΔS)²  +  ν · Δσ  +  Θ · Δt  +  ε

where:
  Δ  =  ∂V/∂S       (delta:  linear underlying sensitivity)
  Γ  =  ∂²V/∂S²     (gamma:  convexity in underlying)
  ν  =  ∂V/∂σ       (vega:   linear volatility sensitivity)
  Θ  =  ∂V/∂t       (theta:  time-decay sensitivity)
  ε                  (residual: rho, vanna, volga, higher-order)
```

For risk-reporting at L1 depth, the source typically aggregates `Δ · ΔS + 1/2 · Γ · (ΔS)² + ν · Δσ` as the **explained P&L** and treat `Θ · Δt + ε` as the **residual carry-and-noise** term. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.327-329.

The **portfolio aggregation** rule for Greeks is straightforward additivity: for a portfolio of positions `i = 1, …, N` with quantities `q_i`, the portfolio-level Greeks are: **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.327-329.

```
Δ_port  =  Σ_i q_i·Δ_i      Γ_port  =  Σ_i q_i·Γ_i      ν_port  =  Σ_i q_i·ν_i
```

This linearity in `q_i` is the structural reason Greek-based risk reporting scales to large books — each Greek is additive across positions in a single underlying. Cross-underlying Greek aggregation requires the joint factor model (which factor moves drive which positions) and is the bridge to factor-based VaR (`[[rm-parametric-var]]`). **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.327-329.

## Mathematical Reasoning

The second-order Taylor expansion is **exact** up to a remainder of order `O(‖(ΔS, Δσ, Δt)‖³)` provided the third derivatives are bounded in the neighbourhood. For most plain-vanilla option books, this remainder is small over a single trading day's typical factor moves, so the explained-P&L term captures the bulk of `ΔV` and the residual is small. For books with **non-linear payoffs near the boundary** (digital options near the strike, barrier knock-outs near the barrier, deep-out-of-the-money options at expiry), the third-order remainder is non-trivial and the decomposition's explanatory power degrades. Practice handles this by either (a) computing the explained P&L with cross-Greeks (vanna, volga, charm, color) added, or (b) acknowledging the unexplained residual and triggering a books-and-records review. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.327-329 + McNeil et al. (2015) Ch.9 pp.327-329.

The **gamma term is structurally non-negative for long-option books**: `Γ ≥ 0` for any portfolio long convexity. This means a long-gamma book makes money from any underlying move — up or down — proportional to `(ΔS)²`. Conversely, a short-gamma book loses money from any underlying move. This asymmetry between long-vs-short gamma is the foundation of **gamma scalping** (long-gamma + delta-hedge = systematic positive carry from underlying volatility, paid for by negative theta). The source treats this as background context; full mechanics live in 07. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.327-329.

The **vega term scales by absolute volatility, not relative**: `ν · Δσ` is the dollar P&L from a chosen absolute increment `Δσ_unit` in implied volatility. Risk-reporting conventions vary in the choice of `Δσ_unit` (per vol point, per percentage point, per basis point), and the source flags the need for unit-consistency across the desk's vega ladder so that aggregated vega exposures share a single `Δσ_unit` convention. The portfolio-level vega `ν_port` aggregates across positions in the same underlying; cross-underlying vega exposures require an implied-vol factor model. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.327-329.

The **theta term is the deterministic time-decay carry**, the only term in the decomposition that is approximately known in advance: for an at-the-money long option, `Θ < 0` (premium decays). The risk-reporting framing treats theta as the "expected carry" of the book — what the book makes or loses per unit of calendar time absent any factor move. Persistent residual P&L beyond theta + Greeks-explained P&L flags missing risk factors. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.327-329.

The cross-Greeks **vanna `∂²V/∂S∂σ`** and **volga `∂²V/∂σ²`** typically live in the residual for L1-depth risk reporting. They become first-order important for books with structurally correlated `S` and `σ` — e.g., equity index books where spot drops trigger vol spikes (the "volatility leverage" effect). Whether to surface vanna/volga as named line items in the daily attribution depends on the desk's structural exposure; the source flags the trade-off without prescribing. Full cross-Greek depth lives in future-07 extensions. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.327-329 + McNeil et al. (2015) Ch.9 pp.327-329.

## See Also

- [rm-sensitivity-versus-simulation](./rm-sensitivity-versus-simulation.md) — Batch-0 card framing sensitivity-based vs simulation-based risk measurement; same Taylor expansion is reframed as risk prediction rather than P&L attribution.
- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — Batch-0 card defining the loss convention `L = −ΔV` that this decomposition reads.
- [rm-parametric-var](./rm-parametric-var.md) — Batch-2 card developing the second-order quadratic-form VaR from the Greek aggregation.
- [../07_derivatives_and_volatility/deriv-greeks-overview](../07_derivatives_and_volatility/deriv-greeks-overview.md) — instrument-level Greek derivations (07 territory).
- [../07_derivatives_and_volatility/deriv-delta-and-hedging](../07_derivatives_and_volatility/deriv-delta-and-hedging.md) — delta-hedging mechanics at instrument level (07 territory).

## Escalate to Raw When

The L1-source risk-reporting framing stops at the second-order decomposition + simple additive Greek aggregation. When the operator needs full cross-Greek depth (vanna, volga, charm, color, speed, vomma — including their P&L attribution under joint S/σ scenarios), full second-order quadratic-form VaR construction with the Hessian matrix, or instrument-level Greek derivations (Black-Scholes, binomial-tree, finite-difference, automatic-differentiation), open the 07 Derivatives cards above OR McNeil Ch.9 §9.1.2-§9.2.2 pp.327-345 directly. **Source:** McNeil et al. (2015) Ch.9 pp.327-345.
