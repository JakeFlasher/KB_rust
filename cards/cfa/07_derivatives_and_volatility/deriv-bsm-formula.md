---
schema_version: "cacg.v0"
id: "deriv-bsm-formula"
title: "Black-Scholes-Merton Formula"
reading_id: "07_derivatives_and_volatility"
summary: "The Black-Scholes-Merton formula gives the closed-form European-call price C = S·N(d₁) - K·e^{-rT}·N(d₂) under continuous-time geometric Brownian motion, with d₁ = (ln(S/K) + (r + σ²/2)·T) / (σ·√T). The BSM PDE follows from the delta-hedging no-arbitrage argument; the closed form solves the PDE under the call-payoff boundary condition and is the limit of the binomial tree as the time step shrinks."
tags: ["derivatives", "bsm-formula"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p352:0519"
    chunk_hash: "5322ab6542f2e9c7cd927a4eaa9c1c327720fac96944c2fcd4572e9be2b8c28b"
    page_range: [352, 352]
    quote: "The most famous solutions to the differential equation (15.16) are the Black–Scholes– Merton formulas for the prices of European call and put options."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2885:4321"
    chunk_hash: "c42941bb3619147fd8d2aa9eeb769f4a8ee968bf57148be2d0e13da1df8012a1"
    page_range: [2885, 2885]
    quote: "Because the right to exercise can be a complex feature of an option, European options are easier to understand, and we will focus on them first."
    edge_type: "supports"
card_hash: "5886f065c2814c41903309282b4c7b8daebabe055c87181a3eee3edf0dbb084c"
---
# Black-Scholes-Merton Formula

## Intuition

The Black-Scholes-Merton (BSM) formula is the closed-form
European-option price under continuous-time geometric Brownian
motion. Two pieces work in tandem: the BSM partial differential
equation, derived from the no-arbitrage delta-hedging argument,
and the explicit closed form `C = S · N(d_1) - K · exp(-r · T)
· N(d_2)`, derived by solving the PDE under the call-payoff
boundary condition. The closed form is the limit of the
binomial tree as the time step shrinks, and the natural
benchmark every Greek and every vol-surface card refines.
**Source:** Hull §15 pp.346-360.

```
BSM call price decomposes as

   C   =     S · N(d_1)   -    K · exp(-r · T) · N(d_2)
   ^         ^                 ^
   call      delta term        discounted strike term
   price     (probability      (probability the call
             of S_T > K        finishes in-the-money under
             times spot)       risk-neutral measure)
```

## Definition

For a European call on a non-dividend-paying underlying with
spot `S`, strike `K`, expiry `T`, volatility `σ`, and risk-free
rate `r`, the Black-Scholes-Merton call price is
`C = S · N(d_1) - K · exp(-r · T) · N(d_2)`,
where `d_1 = (ln(S / K) + (r + σ^2 / 2) · T) / (σ · sqrt(T))`,
`d_2 = d_1 - σ · sqrt(T)`, and `N(·)` is the standard-normal
CDF. The European put price follows from put-call parity:
`P = K · exp(-r · T) · N(-d_2) - S · N(-d_1)`. **Source:** Hull
§15 pp.346-360; CFA L1 Curriculum (2022) Vol.5/pp.430-450.

The BSM PDE governs the option price `V(S, t)` under the
risk-neutral measure:
`∂V/∂t + (1/2) · σ^2 · S^2 · ∂²V/∂S^2 + r · S · ∂V/∂S = r · V`,
with terminal condition `V(S, T) = max(S - K, 0)` for a call or
`max(K - S, 0)` for a put. The PDE is the Feynman-Kac
counterpart of the discounted-expectation pricing rule and is
analytically solvable under the lognormal underlying-price
distribution. **Source:** Hull §15 pp.346-360.

## Mathematical Reasoning

The BSM derivation rests on three steps. First, assume the
underlying follows geometric Brownian motion
`dS / S = μ · dt + σ · dW` under the physical measure. Second,
construct a self-financing portfolio long the option and short
`Δ = ∂V / ∂S` units of the underlying; Itô's lemma applied to
the option price plus the cost-of-carry on the short position
eliminates the `dW` term, leaving a deterministic instantaneous
return that must equal the risk-free rate by no-arbitrage.
Third, the resulting equation rearranges into the BSM PDE.
**Source:** Hull §15 pp.346-360.

The closed-form `C = S · N(d_1) - K · exp(-r · T) · N(d_2)`
follows from solving the BSM PDE under the call-payoff
boundary condition, equivalent to evaluating the discounted
risk-neutral expectation
`C = exp(-r · T) · E^Q[max(S_T - K, 0)]` where `S_T` is
lognormal under the risk-neutral measure. The two `N(·)` terms
have probabilistic interpretations: `N(d_2)` is the
risk-neutral probability that the call finishes in-the-money,
and `N(d_1)` is the corresponding probability under the
share-numeraire measure (also the call's BSM delta). **Source:**
Hull §15 pp.346-360.

The model's assumptions and known limitations bound its scope.
Constant volatility, constant interest rate, no dividends,
continuous trading, no transaction costs, and lognormal
distribution of `S_T` are stylized; real markets exhibit a
volatility smile / skew / term-structure that the constant-σ
BSM cannot reproduce. The BSM formula remains the practitioner
benchmark — implied vol is defined as the σ that recovers a
market option's price under BSM (per
[`deriv-implied-volatility.md`](./deriv-implied-volatility.md#definition))
— but pricing exotic / non-vanilla payoffs requires the
extensions covered in later 07 cards. **Source:** Hull §15
pp.360-380.

## See Also

- [`deriv-binomial-tree-valuation.md`](deriv-binomial-tree-valuation.md) — discrete-time precursor that converges to BSM as the time step shrinks
- [`deriv-risk-neutral-measure.md`](deriv-risk-neutral-measure.md) — equivalent martingale measure that underpins the BSM PDE
- [`deriv-greeks-overview.md`](deriv-greeks-overview.md) — partial derivatives of the BSM closed form

## Escalate to Raw When

Open Hull chapter 15 directly when any of the criteria below
applies. **Source:** Hull §15 pp.346-380.

- The underlying pays continuous or discrete dividends; the
  `S` term in the closed form is replaced by `S · exp(-q · T)`
  for continuous yield `q` per Hull §15. **Source:** Hull §15
  pp.360-380.
- Constant-vol BSM is being calibrated to market prices and
  produces a vol surface that is not flat; that signals the
  smile / skew handled by
  [`deriv-vol-surface-anatomy.md`](deriv-vol-surface-anatomy.md#definition).
  **Source:** Hull §20 pp.460-485.
- Stochastic vol or jump-diffusion is needed; those are
  separate model classes covered in later 07 batches.
  **Source:** Hull §27 pp.626-660.
