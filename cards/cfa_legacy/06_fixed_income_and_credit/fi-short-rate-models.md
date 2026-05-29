---
schema_version: "cacg.v0"
id: "fi-short-rate-models"
title: "Short-Rate Models"
reading_id: "06_fixed_income_and_credit"
summary: "Short-rate models parameterise the term structure with one stochastic state variable r(t). Vasicek (mean-reverting Gaussian) and CIR (square-root diffusion, non-negative) admit affine bond-price closed forms. Hull-White extends Vasicek with a time-dependent drift theta(t) calibrated to today's curve exactly, making it the practitioner default for path-dependent rate derivatives."
tags: ["fixed-income", "short-rate"]
citations:
  - source_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed"
    chunk_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed:p106:0151"
    chunk_hash: "296714b43f84e4c0c57b731c1fcd21fa3f7f9d6648fc698b53d88180d1f3623a"
    page_range: [106, 107]
    quote: "The success of models like that of Vasicek (1977) and that of Cox, Ingersoll and Ross (1985) was mainly due to their possibility of pricing analytically bonds"
    edge_type: "defines"
  - source_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed"
    chunk_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed:p108:0152"
    chunk_hash: "1fb9068c26b190b5528d092eec2972f9500a59712f570b2ceec01209ad030d0b"
    page_range: [108, 108]
    quote: "Vasicek (1977) assumed that the instantaneous spot rate under the real-world measure evolves as an Ornstein-Uhlenbeck process"
    edge_type: "supports"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p721:1075"
    chunk_hash: "4b3839a1aaa6fc4d69c1c771a0998c95bbb662d094f108ea7032dacfc8f9005a"
    page_range: [721, 721]
    quote: "When r is high, mean reversion tends to cause it to have a negative drift; when r is low, mean reversion tends to cause it to have a positive drift"
    edge_type: "supports"
card_hash: "9c75feb4e053634b8ae5bf0a85f0f7b2a6b6ee502e16ee901763bf46437f8d4e"
---
# Short-Rate Models

## Intuition

A short-rate model parameterizes the entire term structure
with one stochastic state variable: the instantaneous risk-
free rate `r(t)`. Once `r(t)` is specified as a stochastic
process under the risk-neutral measure, every bond's price
follows by no-arbitrage. The model's appeal is parsimony —
one Markov state captures the curve. Its limitation is that
the curve dynamics are then mechanically tied to a single
factor. **Source:** Brigo+Mercurio (2006) Ch.3
pp.55-130.

```
r(t) (short rate)
   ^
   |               *  *
   |             *  *  *  *  *  random walk
   |          *  *  *     *  *  with drift toward
   |       *           mean   *  long-run mean
   |    *
   |    o-------------------------> long-run mean theta
   |
   +---------------------------------------> t
   instantaneous rate fluctuates around theta with
   speed-of-mean-reversion kappa.
```

## Definition

The Vasicek model specifies
`dr(t) = κ · (θ - r(t)) · dt + σ · dW(t)` under the risk-
neutral measure, where `κ > 0` is mean-reversion speed,
`θ` is the long-run mean, and `σ` is volatility. Bond
prices admit closed-form expressions in terms of these
three parameters; rates can go negative. **Source:** Brigo+
Mercurio (2006) §3.2 pp.55-90.

The Cox-Ingersoll-Ross (CIR) model specifies
`dr(t) = κ · (θ - r(t)) · dt + σ · sqrt(r(t)) · dW(t)`
under the risk-neutral measure; the state-dependent
diffusion bounds rates below by zero provided the Feller
condition `2 · κ · θ ≥ σ^2` holds. Bond prices remain
closed-form but the distribution of `r(t)` is non-central
chi-square rather than Gaussian. **Source:** Brigo+Mercurio
(2006) §3.4 pp.90-130;
Hull §28 pp.690-720.

The Hull-White (one-factor extended Vasicek) model
specifies `dr(t) = (θ(t) - α · r(t)) · dt + σ · dW(t)`
with `α > 0` the mean-reversion speed and `θ(t)` a
deterministic function calibrated so the model fits
today's observed term structure exactly; it preserves
Gaussian tractability at the cost of negative-rate
possibility. **Source:** Hull §28 pp.690-720;
Brigo+Mercurio (2006) §3.3 pp.65-90.

## Mathematical Reasoning

Under any one-factor Markovian short-rate dynamics
`dr(t) = μ(r, t) · dt + σ(r, t) · dW(t)` (risk-neutral),
the zero-coupon bond price `P(t, T)` satisfies the
risk-neutral bond-pricing PDE
`∂P/∂t + μ · ∂P/∂r + (1/2) · σ^2 · ∂²P/∂r² = r · P`
with terminal condition `P(T, T) = 1`. The PDE is the
Feynman-Kac counterpart of the discounted-expectation
pricing rule
`P(t, T) = E^Q[exp(- ∫_t^T r(s) · ds) | F_t]`, and every
short-rate model in this card slots into it by substituting
its `(μ, σ)` pair. **Source:** Brigo+Mercurio (2006) §3.1
pp.50-55; Hull §28 pp.690-695.

The Markovian short-rate state implies bond prices admit
the affine representation
`P(t, T) = exp(A(t, T) - B(t, T) · r(t))` where `A` and
`B` are deterministic functions of model parameters and
solve a system of Riccati ODEs derived by substituting the
ansatz into the bond-pricing PDE. Vasicek and CIR both
fall in the affine class; the calibration task reduces to
fitting `(κ, θ, σ)` to market prices. **Source:** Brigo+
Mercurio (2006) Ch.3 pp.55-130.

Term-premium effects from
[`fi-term-structure-theories.md`](./fi-term-structure-theories.md#mathematical-reasoning)
are absorbed into the model's risk-neutral drift; the
physical-measure drift differs by the market price of
risk. The classical theories survive as parameter
restrictions: pure-expectations corresponds to zero risk-
premium, liquidity-preference adds a positive premium
term. **Source:** Brigo+Mercurio (2006) Ch.3
pp.55-130.

The Hull-White extension fixes today's curve exactly by
absorbing the curve-shape mismatch into the deterministic
`θ(t)` function. This is essential for pricing path-
dependent rate derivatives (caps, floors, swaptions) where
the calibration target is not just a single curve but
an entire option surface. **Source:** Hull §28
pp.690-720.

The forward-rate framework
[`fi-hjm-forward-rate-framework.md`](./fi-hjm-forward-rate-framework.md#mathematical-reasoning)
generalizes the short-rate view by modeling the entire
forward curve as a stochastic process; short-rate models
are a special-case projection of HJM onto a one-factor
Markovian state. **Source:** Brigo+Mercurio (2006) Ch.5
pp.155-190.

## See Also

- [`fi-spot-par-forward-curves.md`](fi-spot-par-forward-curves.md) — the curve representation short-rate models project onto
- [`fi-term-structure-theories.md`](fi-term-structure-theories.md) — classical theories as parameter restrictions on short-rate models
- [`fi-hjm-forward-rate-framework.md`](fi-hjm-forward-rate-framework.md) — the forward-rate generalization of short-rate models

## Escalate to Raw When

Open Brigo+Mercurio Chapter 3 or Hull Chapter 28 directly
when any of the criteria below applies. **Source:** Brigo+
Mercurio (2006) Ch.3 pp.55-130; Hull §28 pp.690-720.

- A specific calibration to swaption / cap / floor implied
  vols is required; this card states the model family but
  does not develop the calibration loop. **Source:**
  Brigo+Mercurio (2006) Ch.3 pp.55-130.
- Two-factor or stochastic-volatility extensions are in
  scope (Hull-White two-factor, G2++); the one-factor
  Markov assumption breaks. **Source:** Hull §28
  pp.690-720.
- Real-world (physical-measure) calibration vs risk-
  neutral pricing requires explicit market-price-of-risk
  parameterization. **Source:** Brigo+Mercurio (2006)
  Ch.3 pp.55-130.
