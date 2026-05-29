---
schema_version: "cacg.v0"
id: "deriv-stochastic-vol-models"
title: "Stochastic Volatility Models"
reading_id: "07_derivatives_and_volatility"
summary: "Stochastic-volatility models add a second random process for the underlying's volatility, correlated with the spot's Brownian motion. Heston uses mean-reverting square-root variance with a Fourier-inversion semi-closed form; SABR uses joint forward-rate and vol diffusions plus Hagan's asymptotic implied-vol formula. Both generate non-flat smiles where BSM cannot."
tags: ["derivatives", "stochastic-vol"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p646:0959"
    chunk_hash: "05132440708291b8dc05e985e5ca1684526205c9d2b797201c93351945f0a03b"
    page_range: [646, 646]
    quote: "An alternative to the variance-gamma model is a model where the process followed by the volatility variable is specified explicitly"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p647:0960"
    chunk_hash: "388609bb3a1b936fbced813904f3f8ee9207eaa572442f801f41242c7cb1de46"
    page_range: [647, 647]
    quote: "when volatility is stochastic but uncorrelated with the asset price, the price of a European option is the Black–Scholes–Merton price integrated over the probability distribution"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p648:0962"
    chunk_hash: "c38cc6d6fb4a200162e12694f169cd6bca39fd6b479f32053616b50c19eebd1e"
    page_range: [648, 648]
    quote: "The parameter s0 defines the level of the volatility. (It is roughly equivalent to the volatility estimate in the usual lognormal model multiplied by F1-b 0 .)"
    edge_type: "defines"
card_hash: "595ac042f020b40e9e16546bce11a54131a21fc20dc8894aad1b3618fb54d3c3"
---
# Stochastic Volatility Models

## Intuition

A stochastic-volatility model treats the underlying's
volatility as itself a random process driven by a second
Brownian motion, possibly correlated with the spot's Brownian
motion. The two-factor structure produces a non-flat
implied-vol smile / skew without invoking deterministic
state-and-time-dependent local vol; the smile arises from the
joint distribution of terminal spot and terminal vol. Heston
and SABR are the two canonical stochastic-vol families:
Heston for equity indices, SABR for interest-rate
caplets/floorlets and FX. **Source:** Hull §27 pp.640-660.

```
stochastic-vol family

   spot:           dS/S    = r · dt + sigma · dW1
   vol process:    d sigma = kappa·(theta - sigma) · dt + xi · dW2

   parameters:
     kappa = mean-reversion speed of vol back to long-run mean
     theta = long-run mean vol
     xi    = vol-of-vol (how much vol itself fluctuates)
     rho   = correlation(dW1, dW2), drives skew direction

   smile mechanism:
     non-zero rho     -> skew direction
     positive xi      -> wings rise (smile width grows with tenor)
```

## Definition

The **Heston model** specifies the spot under risk-neutral
measure as
`dS / S = r · dt + sqrt(v) · dW^S`,
`dv = κ · (θ - v) · dt + ξ · sqrt(v) · dW^v`,
where `v` is the instantaneous variance, `κ` is the mean-
reversion speed, `θ` is the long-run variance level, `ξ` is the
vol-of-vol parameter, and `Corr(dW^S, dW^v) = ρ`. The
square-root vol process is mean-reverting and bounded below by
zero; bond-pricing-style affine techniques give a semi-closed-
form for European call prices via Fourier inversion. **Source:**
Hull §27 pp.640-650.

The **SABR model** (Stochastic Alpha-Beta-Rho) is a forward-
rate stochastic-vol model commonly used for caplets/floorlets
and swaptions:
`dF = α · F^β · dW^F`,
`dα = ν · α · dW^α`,
with `Corr(dW^F, dW^α) = ρ`. The parameters are `α` (initial
vol), `β ∈ [0, 1]` (CEV-skew exponent: `β = 0` gives normal,
`β = 1` lognormal, `β = 0.5` square-root), `ρ` (correlation),
and `ν` (vol-of-vol). The Hagan asymptotic implied-vol formula
gives an explicit smile parameterization that is the
practitioner default for caplet pricing. **Source:** Hull §27
pp.650-660.

## Mathematical Reasoning

Stochastic-vol models generate the implied-vol smile because
the joint distribution of `S_T` and `v_T` (or `α_T`) under the
risk-neutral measure is no longer lognormal. A non-zero
correlation `ρ` introduces directional asymmetry: negative `ρ`
(typical for equity indices, where higher vol coincides with
falling spot) produces a downward-sloping skew, and positive
`ρ` produces an upward-sloping skew. The vol-of-vol parameter
`ξ` (or `ν`) widens the skew tails: more vol-of-vol broadens
the terminal-vol distribution, raising both wings of the
implied-vol smile. **Source:** Hull §27 pp.640-660.

The Heston semi-closed-form follows from the affine structure
of the two-factor process. The characteristic function of `(S_T,
v_T)` under risk-neutral measure is exponential-affine in `v_t`,
and the European-call Fourier-inversion price uses
`C(K, T) = S · P_1 - K · exp(-r · T) · P_2`,
where `P_1` and `P_2` are the integrated characteristic-
function expressions playing the role of `N(d_1)` and `N(d_2)`
in BSM. The integral is one-dimensional and numerically tractable
in milliseconds; this efficiency is why Heston is the
practitioner default for equity-index vol-surface fitting.
**Source:** Hull §27 pp.640-650.

The SABR Hagan asymptotic implied-vol formula provides the
practitioner shortcut. Rather than numerically integrating the
SABR SDE for each strike, the formula gives a closed-form
expression for `σ_imp(K, F, T)` in terms of `(α, β, ρ, ν)` plus
the moneyness `K / F`, treating the formula as a smile
parameterization rather than an exact result. **Source:** Hull
§27 pp.650-660.

## See Also

- [`deriv-vol-surface-anatomy.md`](deriv-vol-surface-anatomy.md) — implied-vol surface that stochastic-vol models target
- [`deriv-local-volatility.md`](deriv-local-volatility.md) — local-vol complement; LSV combines both for static fit + dynamic smile
- [`deriv-bsm-formula.md`](deriv-bsm-formula.md) — BSM closed form that constant-vol stochastic-vol limits to as `ξ → 0`

## Escalate to Raw When

Open Hull chapter 27 directly when any of the criteria below
applies. **Source:** Hull §27 pp.640-660.

- Heston / SABR calibration to a specific market surface is
  needed; the practitioner workflow involves smile-fitting,
  parameter identification, and stability under daily updates.
  **Source:** Hull §27 pp.640-660.
- Local-stochastic-volatility blending is needed for static
  fit + dynamic smile; the LSV framework superposes a
  local-vol leverage function on the stochastic-vol diffusion.
  **Source:** Hull §27 pp.640-660.
- Jump-diffusion extensions are needed (e.g. Bates model =
  Heston + Merton jumps); those add a third randomness source
  (jump times) on top of the two-factor stochastic vol.
  **Source:** Hull §27 pp.640-660.
