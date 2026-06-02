---
schema_version: "cacg.v0"
id: "deriv-greeks-overview"
title: "Greeks Overview"
reading_id: "07_derivatives_and_volatility"
summary: "The Greeks are the option price's partial derivatives with respect to its inputs: delta = ∂V/∂S, gamma = ∂²V/∂S², vega = ∂V/∂σ, theta = ∂V/∂t, rho = ∂V/∂r. Under BSM, a European call has Δ = N(d₁), Γ = N'(d₁)/(S·σ·√T), and vega = S·√T·N'(d₁); signs flip for puts on delta/rho. Greeks aggregate linearly across positions."
tags: ["derivatives", "greeks-overview"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p444:0658"
    chunk_hash: "ef2c7ab10008fff92ae366bb5b74e0bf8c8e998beaf8d50eb59a5014062869d4"
    page_range: [445, 445]
    quote: "Once an option position has been made delta neutral, the next stage is often to look at its gamma"
    edge_type: "defines"
card_hash: "96fc76b61bf38ae195537d3b253d47250f2ed41ec53b46ad5c1217925166a733"
---
# Greeks Overview

## Intuition

The Greeks are the option price's partial derivatives with
respect to its inputs. Delta measures sensitivity to the
underlying, gamma measures the rate of change of delta, vega
measures sensitivity to volatility, theta measures sensitivity
to time, and rho measures sensitivity to the risk-free rate.
For a European call under BSM the signs are positive delta,
positive gamma, positive vega, negative theta, positive rho;
for a European put the signs flip on delta and rho. Greeks
aggregate linearly across positions in a portfolio, which is
why they are the practitioner's risk-management lingua franca.
**Source:** Hull §19 pp.430-455.

```
<!-- primitive: greek-decomposition-tree source: _diagram_primitives.md -->
                  option price V
                         |
        +----------+-----+-----+----------+
        |          |           |          |
      delta       vega       theta       rho
   = dV/dS    = dV/d(sig)   = dV/dt    = dV/dr
        |          |
        |          +---------+
        |                    |
      gamma                vanna
    = d2V/dS2          = d2V/dS d(sig)
                              |
                            volga
                       = d2V/d(sig)2
   first-order: linear sensitivities
   second-order: curvature / cross-curvature
```

## Definition

**Delta** `Δ = ∂V / ∂S` measures the option price's first-order
sensitivity to the underlying spot. For a European call under
BSM, `Δ_call = N(d_1)`, ranging in `(0, 1)` from out-of-the-
money to deep in-the-money; for a European put,
`Δ_put = N(d_1) - 1`, ranging in `(-1, 0)`. Delta is also the
share-numeraire risk-neutral probability that the option
finishes in-the-money, the hedge ratio that makes a one-step
delta-neutral portfolio, and the slope of the option-price-vs-
spot curve at the current spot. **Source:** Hull §19 pp.430-440.

**Gamma** `Γ = ∂²V / ∂S² = ∂Δ / ∂S` measures the second-order
sensitivity. For European calls and puts under BSM,
`Γ = N'(d_1) / (S · σ · sqrt(T))` is the same and is positive,
peaking around at-the-money. Gamma quantifies how often a delta
hedge must be rebalanced and the convexity profit from a long-
option position when the underlying moves. **Source:** Hull §19
pp.440-445.

**Vega** `ν = ∂V / ∂σ` (also written `vega`) measures sensitivity
to the volatility input. For European options under BSM,
`vega = S · sqrt(T) · N'(d_1)`, positive for both call and put
and peaking at-the-money. **Theta** `Θ = ∂V / ∂t` measures the
option's calendar decay: for a European call `Θ < 0` (the option
loses value as expiry approaches), peaking in magnitude near
expiry. **Rho** `ρ = ∂V / ∂r` measures sensitivity to the
risk-free rate; positive for calls and negative for puts.
**Source:** Hull §19 pp.445-455.

## Mathematical Reasoning

Greeks are partial derivatives of the option-price function;
they aggregate linearly across positions in a portfolio. For a
portfolio of `n_i` units of option `i`, the portfolio Greeks
are `Δ_port = Σ n_i · Δ_i`, `Γ_port = Σ n_i · Γ_i`, etc. This
linearity is the practitioner's convenience: a desk can target
delta-neutral, gamma-neutral, or vega-neutral via offsetting
trades whose Greeks sum to zero. **Source:** Hull §19
pp.430-455.

The BSM PDE
`∂V/∂t + (1/2) · σ^2 · S^2 · ∂²V/∂S^2 + r · S · ∂V/∂S = r · V`
relates theta, gamma, and delta algebraically: every BSM-
priced derivative satisfies this constraint at every point.
The relationship `Θ + (1/2) · σ^2 · S^2 · Γ + r · S · Δ = r · V`
implies that a delta-hedged BSM portfolio earns the risk-free
rate at instantaneous frequency: gamma profit pays for theta
decay. This is the algebraic identity behind dynamic
delta-hedging. **Source:** Hull §19 pp.450-455.

For exotic and non-BSM models, the Greeks lose their closed-
form expressions but retain the partial-derivative
interpretation. Practitioners estimate them numerically via
finite-difference bumps, pathwise differentiation, or
likelihood-ratio methods (covered in
[`deriv-mc-greek-estimation.md`](./deriv-mc-greek-estimation.md#definition)).
The first-order Greeks (delta, vega, theta, rho) and the key
second-order Greek (gamma) remain the practitioner's daily
risk metrics regardless of the pricing model. **Source:** Hull
§19 pp.450-455.

## See Also

- [`deriv-bsm-formula.md`](deriv-bsm-formula.md) — BSM closed-form whose partial derivatives are these Greeks
- [`deriv-delta-and-hedging.md`](deriv-delta-and-hedging.md) — practical use of delta and gamma in dynamic hedging
- [`deriv-vega-and-theta.md`](deriv-vega-and-theta.md) — vega and theta in detail and the BSM-PDE relationship

## Escalate to Raw When

Open Hull chapter 19 directly when any of the criteria below
applies. **Source:** Hull §19 pp.430-460.

- Higher-order Greeks (vanna, volga, color, charm, speed) are
  needed; those are practitioner extensions covered in Hull §19
  pp.450-460. **Source:** Hull §19 pp.450-460.
- Greeks under stochastic-vol or jump-diffusion models are
  needed; the closed-form BSM expressions no longer apply.
  **Source:** Hull §27 pp.626-660.
- Vol-surface-aware Greeks (smile-adjusted delta, sticky-strike
  vs sticky-delta vega) are needed; those depend on the smile
  parameterization in
  [`deriv-vol-surface-anatomy.md`](deriv-vol-surface-anatomy.md#definition).
  **Source:** Hull §20 pp.460-485.
