---
schema_version: "cacg.v0"
id: "deriv-mc-greek-estimation"
title: "Monte Carlo Greek Estimation"
reading_id: "07_derivatives_and_volatility"
summary: "Three methods compete for MC Greek estimation: finite-difference bumping (bias O(h²), variance O(1/h²)); pathwise differentiation (unbiased, low variance, requires smooth payoff); likelihood-ratio method (unbiased, supports any payoff, higher variance). Pathwise is the default; LR is the universal fallback for discontinuous payoffs (digital, barrier)."
tags: ["derivatives", "mc-greek"]
citations:
  - source_id: "cb_glasserman_2003_monte_carlo_methods"
    chunk_id: "cb_glasserman_2003_monte_carlo_methods:p391:0480"
    chunk_hash: "3d3a3a1073c1de63dd330d37d6b6a078ca84d0957797fd513a554ff619c365b4"
    page_range: [391, 392]
    quote: "The pathwise method differentiates each simulated outcome with respect to the parameter of interest; the likelihood ratio method differentiates a probabilit"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p444:0658"
    chunk_hash: "ef2c7ab10008fff92ae366bb5b74e0bf8c8e998beaf8d50eb59a5014062869d4"
    page_range: [445, 445]
    quote: "Once an option position has been made delta neutral, the next stage is often to look at its gamma"
    edge_type: "supports"
card_hash: "959c53a61dc7bc2eda6dceda8d28bd3b079bd96e0080c467c01d387ffd0d0a04"
---
# Monte Carlo Greek Estimation

## Intuition

Estimating Greeks by Monte Carlo introduces a bias-variance
trade-off that is distinct from the price estimator's. Three
methods compete: finite-difference bumping (re-price at a
shifted parameter and divide by the bump size), pathwise
differentiation (differentiate the simulated payoff with
respect to the parameter inside the expectation), and
likelihood-ratio scoring (differentiate the simulation density
with respect to the parameter). Bumping is the most general
but suffers from finite-difference bias; pathwise is bias-free
but requires a smooth payoff; LR is bias-free but typically
high-variance. **Source:** Glasserman §7 pp.377-420.

```
Greek estimation taxonomy

    finite-difference bump                  pathwise method
    Delta_hat = (V(S+h) - V(S-h)) / (2h)    Delta_hat = E[ d/dS payoff(S_T(S)) ]
    bias O(h^2), variance O(1/h^2)          unbiased, low variance,
                                            requires payoff smooth in S

    likelihood-ratio (LR) method
    Delta_hat = E[ payoff(S_T) · score(S; S_T) ]
    unbiased, no payoff-smoothness requirement,
    typically higher variance than pathwise.
```

## Definition

**Finite-difference bumping** estimates a Greek
`∂V / ∂θ` by re-pricing the derivative at a shifted parameter
value and computing the difference quotient. The central
difference
`∂V / ∂θ ≈ (V(θ + h) - V(θ - h)) / (2 · h)`
has bias `O(h^2)` from the second-order Taylor truncation and
variance `O(1 / h^2)` from the difference of two noisy
estimators. The optimal `h` minimizes the total MSE; the
practitioner default uses common random numbers across the two
re-prices, which dramatically reduces the variance term and
shifts the optimal `h` to a smaller value than the
independent-RNG case. **Source:** Glasserman §7 pp.377-400.

**Pathwise differentiation** computes the Greek as
`∂V / ∂θ = E^Q[exp(-r · T) · ∂_θ payoff(S_T(θ))]`,
where the parameter dependence is propagated through the path
inside the expectation. For the spot delta of a European call
under GBM, `∂S_T / ∂S = S_T / S`, and the pathwise estimator
becomes
`Δ_hat = E^Q[exp(-r · T) · 1{S_T > K} · S_T / S]`,
which is unbiased and typically lower variance than bumping.
The method requires the payoff to be differentiable
almost-everywhere with respect to the parameter; payoffs with
discontinuities (digital, barrier) need the smoothing techniques
or the LR method as a fallback. **Source:** Glasserman §7
pp.400-415.

**Likelihood-ratio method** computes the Greek as
`∂V / ∂θ = E^Q[exp(-r · T) · payoff(S_T) · ∂_θ ln p(S_T; θ)]`,
where `p(S_T; θ)` is the risk-neutral density of `S_T` as a
function of `θ`. The method differentiates the density rather
than the payoff, which works for any payoff (including
discontinuous ones) but typically gives higher variance than
pathwise because the score function `∂_θ ln p` is multiplied
by the raw payoff. The LR method is the universal fallback
when pathwise is unavailable. **Source:** Glasserman §7
pp.415-420.

## Mathematical Reasoning

The bias-variance trade-off across the three methods governs
the practitioner choice. Finite-difference bumping with
common-random-numbers achieves variance `O(1)` (independent of
`h`) at the cost of a residual `O(h^2)` bias; halving `h`
quarters the bias without affecting variance, so the trade-off
is favorable until floating-point precision dominates.
Pathwise is unbiased and typically the lowest-variance choice
when applicable; LR is unbiased and a guaranteed fallback at
the cost of higher variance. The practitioner default is
pathwise where the payoff allows, LR where it does not, and
bumping with common RNG when implementation simplicity
dominates. **Source:** Glasserman §7 pp.377-420.

For BSM vanilla-option Greeks, all three methods agree
asymptotically with the BSM closed-form Greeks. The deviations
appear when the underlying is non-BSM (stochastic-vol, jump-
diffusion) or the payoff is exotic. The practitioner test is
to compute a BSM-vanilla Greek by all three methods plus the
analytic formula; the agreement validates the implementation.
**Source:** Glasserman §7 pp.377-420; Hull §19 pp.430-460.

The variance-reduction techniques from
[`deriv-variance-reduction.md`](./deriv-variance-reduction.md#definition)
apply to Greek estimators as well as to price estimators.
Antithetic variates and common random numbers reduce the
variance of the bump, pathwise, and LR estimators alike;
control variates with a related-instrument analytic Greek can
dramatically lower variance for stochastic-vol Greeks where a
BSM analytic counterpart provides high correlation with the
target Greek. **Source:** Glasserman §7 pp.377-420.

## See Also

- [`deriv-monte-carlo-pricing.md`](deriv-monte-carlo-pricing.md) — MC framework whose Greek estimation this card extends
- [`deriv-greeks-overview.md`](deriv-greeks-overview.md) — Greek taxonomy whose closed-form BSM benchmarks validate the MC estimators
- [`deriv-variance-reduction.md`](deriv-variance-reduction.md) — variance-reduction techniques that apply to both price and Greek estimators

## Escalate to Raw When

Open Glasserman chapter 7 directly when any of the criteria
below applies. **Source:** Glasserman §7 pp.377-420.

- Adjoint Algorithmic Differentiation (AAD) is needed for
  high-dimensional Greek vectors; the practitioner default at
  modern dealing-desk scale. **Source:** Glasserman §7
  pp.377-420.
- Malliavin-calculus-based Greek estimation is needed for
  payoffs with severe discontinuities or for second-order
  Greeks (gamma, vanna). **Source:** Glasserman §7
  pp.377-420.
- Second-order Greek interactions (gamma, vanna, volga) need
  to be estimated jointly with first-order Greeks; the
  pathwise method requires twice-differentiable payoff and
  the LR method requires the second-derivative score function.
  **Source:** Glasserman §7 pp.377-420.
