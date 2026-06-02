---
schema_version: "cacg.v0"
id: "deriv-discretization-and-bias"
title: "Path Discretization and Bias"
reading_id: "07_derivatives_and_volatility"
summary: "When the risk-neutral SDE has no closed-form terminal sampler, Monte Carlo pricing discretizes the path in time. The Euler scheme has weak order 1 and strong order 1/2; the Milstein refinement adds an Itô correction lifting strong order to 1 (weak order stays at 1). MSE decomposes as bias² + variance/N; the budget-optimal time step is dt ≈ N^{-1/(2p)}."
tags: ["derivatives", "discretization-bias"]
citations:
  - source_id: "cb_glasserman_2003_monte_carlo_methods"
    chunk_id: "cb_glasserman_2003_monte_carlo_methods:p359:0441"
    chunk_hash: "eaa2eac886320ffee609928d50d36814e6e6792d63737c480b4bd35bb5a153ca"
    page_range: [359, 360]
    quote: "In more detail, the Euler scheme has strong order 1/2 under conditions only slightly stronger than those in Theorem B.2.1 of Appendix B.2 for existence"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p352:0519"
    chunk_hash: "5322ab6542f2e9c7cd927a4eaa9c1c327720fac96944c2fcd4572e9be2b8c28b"
    page_range: [352, 352]
    quote: "The most famous solutions to the differential equation (15.16) are the Black–Scholes– Merton formulas for the prices of European call and put options."
    edge_type: "supports"
card_hash: "2e674a1ebe93873a5e80908022ee55ff768c58bc2fe39d7aa1ef46b821019a86"
---
# Path Discretization and Bias

## Intuition

When the underlying's risk-neutral SDE has no closed-form
terminal-distribution sampler, Monte Carlo simulation must
discretize the SDE in time. Each discretization scheme
introduces a path-bias separate from the MC sampling variance:
the price estimator is biased by an amount that depends on the
time-step `dt` and the scheme order. Euler is the simplest
scheme (first-order weak convergence, half-order strong
convergence); the basic Milstein refinement adds an Itô
correction term that raises strong order to one while leaving
weak order at one. Higher weak order requires the
second-order Taylor schemes that lie past this card. **Source:**
Glasserman §6 pp.339-370.

```
discretization schemes

    Euler:    S_(t+dt) = S_t · (1 + r · dt + sigma · sqrt(dt) · Z)
    Milstein: adds (1/2) · sigma^2 · S_t · ((sqrt(dt) · Z)^2 - dt)
                                      ^
                                      |
                              second-order Ito correction

    weak convergence: E[ f(S_T_discrete) ] -> E[ f(S_T) ] as dt -> 0
        rate: O(dt) for both Euler and the basic Milstein
              refinement; weak order two requires the
              second-order Taylor schemes (Kloeden-Platen).

    strong convergence: E[|S_T_discrete - S_T|] -> 0 as dt -> 0
        rate: O(sqrt(dt)) for Euler, O(dt) for Milstein.
```

## Definition

The **Euler scheme** for the risk-neutral SDE
`dS / S = r · dt + σ · dW^Q` is
`S_(n+1) = S_n + r · S_n · dt + σ · S_n · sqrt(dt) · Z_n`,
where `Z_n ~ N(0, 1)` are i.i.d. The scheme is the simplest
first-order discretization of the SDE; for geometric Brownian
motion the Euler scheme can also be applied to `log(S)` to
avoid negative-spot issues. The discretization error in the
terminal-distribution approximation is `O(dt)` for weak
convergence (the error in pricing functionals
`E[f(S_T_discrete)] - E[f(S_T)]`). **Source:** Glasserman §6
pp.339-360.

The **Milstein scheme** adds the next-order Itô correction
term:
`S_(n+1) = S_n + r · S_n · dt + σ · S_n · sqrt(dt) · Z_n
        + (1/2) · σ^2 · S_n · ((sqrt(dt) · Z_n)^2 - dt)`.
The basic Milstein refinement raises **strong** order to
`O(dt)` (one full order higher than Euler's `O(sqrt(dt))`) while
**weak** order remains `O(dt)`, the same as Euler — Glasserman
§6.2 is explicit on this point. The advantage is therefore on
the pathwise side, not the expectation side. For GBM the
Milstein scheme is essentially exact (the GBM SDE has a closed-
form solution, so the discretization error vanishes when
applied to `log(S)`); for stochastic-vol or jump-diffusion SDEs
the Milstein correction term shrinks the dominant
discretization-bias coefficient even at unchanged weak order.
**Source:** Glasserman §6 pp.360-370.

## Mathematical Reasoning

**Weak convergence** measures the error in expected
functionals: a scheme has weak order `p` if
`|E[f(S_T_discrete)] - E[f(S_T)]| ≤ C · dt^p`
for smooth `f`. Weak convergence is the relevant criterion for
European-option pricing because the price is the expected
discounted payoff. **Strong convergence** measures the error in
pathwise distance: a scheme has strong order `p_s` if
`E[|S_T_discrete - S_T|] ≤ C · dt^(p_s)`,
relevant for path-dependent payoffs (barrier, lookback, Asian)
where the entire path matters. The two orders are independent.
Euler is weak order one and strong order one-half. The basic
Milstein refinement keeps weak order at one but raises strong
order to one; weak order two requires the second-order Taylor
schemes (Kloeden-Platen) covered in the Escalate section.
**Source:** Glasserman §6 pp.339-370.

The bias-variance trade-off in MC pricing depends on the choice
of `dt` and `N`. The total mean-squared error is approximately
`MSE = bias^2 + variance / N ≈ C_1 · dt^(2p) + C_2 / N`,
where `p` is the weak order. To minimize MSE at fixed
computational budget (which is roughly `N / dt`), the optimal
trade-off is `dt^(2p) ≈ 1 / N`, equivalent to choosing
`dt ≈ N^(-1 / (2p))`. Both Euler and the basic Milstein
refinement have `p = 1`, so the asymptotic scaling
`dt ≈ N^(-1/2)` is identical for the two schemes; the
Milstein advantage is in the `C_1` constant (the Itô-correction
term shrinks the leading discretization-bias coefficient) and in
the strong-order improvement that matters for path-dependent
payoffs, not in the weak-order exponent. Genuine `p = 2`
behaviour requires the second-order Taylor schemes, where
`dt ≈ N^(-1/4)` is the asymptotic optimum. **Source:**
Glasserman §6 pp.339-370.

The discretization bias is separate from the MC sampling
variance and must be controlled separately. Variance-reduction
techniques (antithetic, control, importance sampling — covered
in [`deriv-variance-reduction.md`](./deriv-variance-reduction.md#definition))
reduce the variance term; smaller `dt` and higher-order schemes
reduce the bias term. The Glasserman recommendation is to
profile the MSE breakdown empirically: if bias dominates, halve
`dt`; if variance dominates, double `N`. The pathwise variance
reduction (antithetic, control) has zero interaction with the
bias; importance sampling can interact with bias when the
importance density distorts the discretization regime.
**Source:** Glasserman §6 pp.339-370.

## See Also

- [`deriv-monte-carlo-pricing.md`](deriv-monte-carlo-pricing.md) — plain MC framework whose convergence rate this card refines
- [`deriv-variance-reduction.md`](deriv-variance-reduction.md) — orthogonal source of MSE: variance reduction at fixed time-step

## Escalate to Raw When

Open Glasserman chapter 6 directly when any of the criteria
below applies. **Source:** Glasserman §6 pp.339-370.

- Genuine **weak-order-two** schemes (the second-order Taylor
  / Kloeden-Platen family) are needed when the basic Milstein
  refinement's `O(dt)` weak bias is still material — for
  example, in deep-tail stochastic-vol or jump-diffusion
  pricing where the dominant-bias coefficient shrunk by the
  Itô correction is not enough. **Source:** Glasserman §6
  pp.339-370.
- Implicit / semi-implicit schemes are needed for stiff SDEs
  (mean-reverting CIR variance with high `κ`); the explicit
  Euler / Milstein schemes can be unstable. **Source:**
  Glasserman §6 pp.339-370.
- Barrier-crossing bias correction is needed; the Brownian-
  bridge construction recovers the conditional barrier-hit
  probability that Euler / Milstein schemes miss between time-
  steps. **Source:** Glasserman §6 pp.339-370.
