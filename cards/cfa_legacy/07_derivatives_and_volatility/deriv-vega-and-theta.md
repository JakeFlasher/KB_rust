---
schema_version: "cacg.v0"
id: "deriv-vega-and-theta"
title: "Vega and Theta"
reading_id: "07_derivatives_and_volatility"
summary: "Vega is the option price's partial derivative with respect to volatility (positive for long calls and puts, peaks ATM); theta is the partial derivative with respect to calendar time (negative for long options as time decay erodes optionality). Under BSM the gamma-theta-vega identity from the PDE links them algebraically."
tags: ["derivatives", "vega-theta"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p428:0636"
    chunk_hash: "16102d3000dedd4a9aec8c72d1b50216038857b752f9b7b21fc46e9f832af8d4"
    page_range: [428, 428]
    quote: "For an at-the-money call option, theta is large and negative"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p434:0641"
    chunk_hash: "7291afbc7680d82ad5ba3f703a533e2ae772901351ba3e60f9a846e66598509c"
    page_range: [434, 434]
    quote: "The vega of an option, V, is the rate of change in its value with respect to the volatility of the underlying asset"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p434:0642"
    chunk_hash: "8bef184d4eff286b8dc6b0fff4d87a7e9d97c99d8d4187a72c2e6b6954900882"
    page_range: [434, 434]
    quote: "Unfortunately, a portfolio that is gamma neutral will not in general be vega neutral, and vice versa"
    edge_type: "defines"
card_hash: "8e530fc18679caa4f814739522b3501382017c0fb6d14ea307ab9f3556736902"
---
# Vega and Theta

## Intuition

Vega measures how much an option's price responds to a change
in the volatility input. A long-option position is long vega:
higher implied vol means a wider terminal-price distribution,
which makes the convex option payoff more valuable. Theta
measures how much an option loses per unit of calendar time
(per day in practitioner units). Long-option positions are
short theta: time-decay erodes the optionality value as expiry
approaches. Under BSM these two Greeks are linked by the
gamma-theta-vega relationship that follows from the option-
pricing PDE. **Source:** Hull §19 pp.445-460.

```
vega and theta profiles for an at-the-money European call

vega                                  theta
   ^                                     ^
   |    *                                |
   |  *   *                              |
   |*       *                            |  *  *  *  *
   *           *                         *           *
   |              *                       |             *
   |                 *                    |               *
   +-->                                   +-->
   peaks ATM, decays           negative; magnitude grows
   at K << S or K >> S         as expiry approaches
```

## Definition

**Vega** `ν = ∂V / ∂σ` measures the option's first-order
sensitivity to the volatility input. For a European call or
put under BSM, `vega = S · sqrt(T) · N'(d_1)`, where `N'(·)` is
the standard-normal density. Vega is positive for both long
calls and long puts (a long-option position benefits from
higher vol), peaks at-the-money, and decays toward zero deep
in- or out-of-the-money. Vega's units are price per unit of
volatility (in practice quoted as dollars per 1% vol move).
**Source:** Hull §19 pp.445-450.

**Theta** `Θ = ∂V / ∂t` measures the option's first-order
sensitivity to calendar-time progress (note: `t` here denotes
the current calendar time, not zero). For a European call under
BSM,
`Θ_call = -(S · σ · N'(d_1)) / (2 · sqrt(T)) - r · K · exp(-r ·
T) · N(d_2)`,
which is negative (the option loses value as expiry approaches)
and peaks in magnitude at-the-money near expiry. The European-
put theta has an analogous formula. Theta's units are price per
unit of time (per day or per year depending on convention).
**Source:** Hull §19 pp.450-455.

## Mathematical Reasoning

The BSM partial differential equation
`∂V/∂t + (1/2) · σ^2 · S^2 · ∂²V/∂S^2 + r · S · ∂V/∂S = r · V`
constrains theta, gamma, and delta algebraically. Substituting
the Greek symbols, the PDE rearranges to
`Θ + (1/2) · σ^2 · S^2 · Γ + r · S · Δ - r · V = 0`,
which expresses theta as a deterministic function of the other
Greeks. This is why a delta-hedged BSM portfolio (with `Δ` of
the underlying offsetting the option's delta) earns the risk-
free rate instantaneously: the residual `(1/2) · σ^2 · S^2 · Γ
+ Θ` is identically zero modulo the `r · (V - Δ · S)` term.
**Source:** Hull §19 pp.450-455.

Vega and gamma are linked but not identically. Vega is the
sensitivity to the implied-vol input; gamma is the sensitivity
to the spot's second derivative. Under BSM with constant `σ`,
the relationship `vega = σ · S^2 · T · Γ` holds for European
calls and puts (a calculation that follows from
differentiating the BSM closed form with respect to `σ` and
comparing to the second derivative with respect to `S`).
Practitioners use this to convert between vega and gamma
exposures, especially when comparing positions with different
maturities. **Source:** Hull §19 pp.450-455.

The practitioner's view of vega and theta as profile shapes
matters more than the algebraic formulas. An at-the-money
straddle (long call + long put at the same strike) is delta-
neutral by construction, has its largest vega and gamma, and
its theta is the most negative; this is the canonical "long
vol" position whose P&L tracks realized-vs-implied-vol over the
option's life. A risk-reversal (long call, short put at
different strikes) has less vega and more delta. The vol-
surface-aware extensions in
[`deriv-vol-surface-anatomy.md`](deriv-vol-surface-anatomy.md#definition)
modify these profiles by introducing strike-dependent implied
vol; the BSM-flat-vol formulas above are the baseline.
**Source:** Hull §19 pp.450-455.

## See Also

- [`deriv-greeks-overview.md`](deriv-greeks-overview.md) — full Greeks taxonomy that places vega and theta against delta, gamma, rho
- [`deriv-delta-and-hedging.md`](deriv-delta-and-hedging.md) — gamma-theta-vega relationship as the algebraic basis for delta-hedged BSM portfolios earning the risk-free rate
- [`deriv-bsm-formula.md`](deriv-bsm-formula.md) — closed-form vega and theta as partial derivatives of the BSM call / put price

## Escalate to Raw When

Open Hull chapter 19 directly when any of the criteria below
applies. **Source:** Hull §19 pp.445-460.

- Higher-order vega Greeks (vanna `∂²V / ∂S ∂σ`, volga `∂²V /
  ∂σ²`) are needed for second-order vol exposure. **Source:**
  Hull §19 pp.450-460.
- Vol-surface-aware theta (smile-consistent decay, forward-vol
  carry) is needed; the BSM-flat-vol formula understates real-
  market theta because the smile changes shape with calendar
  time. **Source:** Hull §20 pp.460-485.
- Stochastic-vol-aware vega is needed; under Heston / SABR the
  vega becomes a vector across the model's parameters and the
  BSM closed form no longer applies. **Source:** Hull §27
  pp.626-660.
