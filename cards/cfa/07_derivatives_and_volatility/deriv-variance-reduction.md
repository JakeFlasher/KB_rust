---
schema_version: "cacg.v0"
id: "deriv-variance-reduction"
title: "Monte Carlo Variance Reduction"
reading_id: "07_derivatives_and_volatility"
summary: "Variance reduction techniques cut Monte Carlo standard error at fixed sample size. Antithetic variates pair each draw with its sign-flipped twin; control variates subtract a correlated estimator with known mean to leave residual variance (1 - rho^2) times the original; importance sampling re-weights paths under a tilted measure that concentrates on high-payoff regions."
tags: ["derivatives", "variance-reduction"]
citations:
  - source_id: "cb_glasserman_2003_monte_carlo_methods"
    chunk_id: "cb_glasserman_2003_monte_carlo_methods:p198:0240"
    chunk_hash: "54ace23b16689a1d3a0dd78af0a0c6b976480a56219a838e3c679ea438ef0449"
    page_range: [198, 199]
    quote: "This chapter develops methods for increasing the efficiency of Monte Carlo simulation by reducing the variance of simulation estimates"
    edge_type: "defines"
  - source_id: "cb_glasserman_2003_monte_carlo_methods"
    chunk_id: "cb_glasserman_2003_monte_carlo_methods:p219:0266"
    chunk_hash: "3b10f81540c256e12ff3cfdd858441902a79d973700e46c4ace2b091d79ebea8"
    page_range: [219, 219]
    quote: "The method of antithetic variates attempts to reduce variance by introduc"
    edge_type: "defines"
  - source_id: "cb_glasserman_2003_monte_carlo_methods"
    chunk_id: "cb_glasserman_2003_monte_carlo_methods:p270:0330"
    chunk_hash: "6ad8d4f2e65026402404f4f06937608d290ce00070448868dee26baabafc5d9c"
    page_range: [270, 270]
    quote: "in designing an effective importance sampling strategy, we should try to sample in proportion to the product of h and f"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p352:0519"
    chunk_hash: "5322ab6542f2e9c7cd927a4eaa9c1c327720fac96944c2fcd4572e9be2b8c28b"
    page_range: [352, 352]
    quote: "The expected payoff from the derivative changes and the discount rate that must be used for this payoff changes"
    edge_type: "supports"
card_hash: "688a29b4896613b3a0ac93da97ef5d5079410835b892169019c075c36b95d531"
---
# Monte Carlo Variance Reduction

## Intuition

Variance reduction techniques cut the Monte Carlo standard
error at a fixed sample size, equivalent to running far more
samples for free. Antithetic variates pair each random draw
with its sign-flipped twin; control variates subtract a
correlated estimator with known mean; importance sampling
re-weights the simulation toward the payoff's high-payoff
region. Each technique buys variance-reduction at the cost of
implementation effort, and the practitioner choice depends on
the payoff's structure and the available analytic results for
related instruments. **Source:** Glasserman §4 pp.182-260.

```
variance reduction taxonomy

    plain MC               antithetic                control
    Y_i = g(Z_i)           pair (Z, -Z) per draw     subtract a correlated
                                                    estimator h with known
                                                    mean: Y_hat = g(Z) -
                                                    beta · (h(Z) - E[h])

    importance sampling
    sample under measure Q_tilde, re-weight by likelihood ratio:
    V = E_Q_tilde[ (dQ/dQ_tilde) · g(S_T) ]
    chosen so the importance density concentrates near the
    payoff's high-payoff region.
```

## Definition

**Antithetic variates**: for each MC draw `Z_i`, also use
`-Z_i` as a paired draw. The estimator is
`V_hat = (1 / N) · Σ_i ((g(Z_i) + g(-Z_i)) / 2)`,
which has variance lower than the plain estimator exactly when
`g(Z)` and `g(-Z)` have negative covariance — typical for
monotone payoffs like a call, where pairing each draw with its
sign-flipped twin smooths the estimator. The technique is
cheapest to implement: the same random-number generator
produces the paired draws, and the pricing function evaluates
twice. There is no upper bound on the achievable variance
reduction in general; for outputs that are linear in the
antithetic input the variance can collapse to zero, while for
outputs that are even-symmetric in `Z` antithetic variates give
no improvement over plain MC. **Source:** Glasserman §4
pp.190-210.

**Control variates**: for an estimator `Y` of the target with
mean `V`, use a correlated estimator `X` whose mean
`E[X] = m` is known analytically. The control-variate estimator
is
`V_hat = Y - β · (X - m)`,
with `β` chosen to minimize variance: `β* = Cov(Y, X) / Var(X)`.
The variance reduction is by a factor `1 / (1 - ρ^2)` where
`ρ = Corr(Y, X)`, so a correlation of 0.95 buys ~10x variance
reduction. The practitioner application is to use a
related-instrument analytic price (e.g. BSM call price as
control for a stochastic-vol Heston call price). **Source:**
Glasserman §4 pp.210-235.

**Importance sampling**: reweight the simulation under a
different measure `Q_tilde` chosen so that the importance
density concentrates near the high-payoff region. The estimator
is
`V_hat = (1 / N) · Σ_i exp(-r · T) · g(S^(i)_T) · L(S^(i)_T)`,
where the likelihood ratio `L = dQ / dQ_tilde` corrects for the
measure change. The variance reduction can be dramatic for
deep-out-of-the-money options (where most plain-MC paths
contribute zero), but the technique requires choosing the
importance density carefully — a poorly chosen `Q_tilde`
inflates variance instead. **Source:** Glasserman §4
pp.235-260.

## Mathematical Reasoning

The variance-reduction effectiveness is bounded by the
correlation structure between the original estimator and the
auxiliary estimator. Antithetic variates achieve full variance
reduction only when `g(Z) + g(-Z)` is constant (deterministic);
no payoff achieves this in finite samples, so the reduction is
partial. Control variates achieve variance reduction
`Var(Y) · (1 - ρ^2)`, asymptotically optimal in `β`; selecting
the optimal `β` from sample data introduces a small bias that
vanishes as `N → ∞`. Importance sampling's variance-reduction
factor depends on the choice of `Q_tilde`, and the worst case
is unbounded (a poorly chosen `Q_tilde` can make the variance
infinite when the likelihood ratio has fat tails). **Source:**
Glasserman §4 pp.182-260.

The practitioner choice trades variance-reduction power against
implementation cost. Antithetic variates are nearly free
(double the pricing-function calls, same RNG); the variance
reduction is modest but reliable. Control variates require
identifying a correlated analytic instrument and computing the
optimal `β` from the sample covariance, but the variance
reduction can exceed 10x for well-correlated controls.
Importance sampling requires bespoke `Q_tilde` design (often
an exponential change of measure for diffusions, or a tilted
Gaussian for terminal payoffs); the reward is dramatic for
rare-event payoffs but the design effort is significant.
**Source:** Glasserman §4 pp.182-260; Hull §15 pp.376-380.

The combination of techniques is permitted: antithetic +
control + importance sampling can be stacked for compounding
variance reduction. The practical limit is that gains
attenuate (each technique exploits a different correlation
dimension), and the overall implementation overhead grows
non-linearly. The practitioner default for vanilla
European-payoff MC is antithetic + control (cheap and
reliable); importance sampling is reserved for deep-OTM
exposures and rare-event payoffs (default-risk, barrier
breaches) where plain-MC is infeasible. **Source:** Glasserman
§4 pp.182-260.

## See Also

- [`deriv-monte-carlo-pricing.md`](deriv-monte-carlo-pricing.md) — plain MC framework that variance reduction modifies
- [`deriv-discretization-and-bias.md`](deriv-discretization-and-bias.md) — orthogonal source of error: discretization bias separate from sampling variance
- [`deriv-mc-greek-estimation.md`](deriv-mc-greek-estimation.md) — Greek estimation that benefits from variance reduction applied to the Greek estimator

## Escalate to Raw When

Open Glasserman chapter 4 directly when any of the criteria
below applies. **Source:** Glasserman §4 pp.182-260.

- Stratified sampling, Latin-hypercube sampling, or moment-
  matching techniques are needed. **Source:** Glasserman §4
  pp.235-260.
- Conditional MC for barrier-option pricing is needed: the
  conditional-expectation trick computes the analytic
  contribution along the unbroken path. **Source:** Glasserman
  §4 pp.182-260.
- Importance-sampling design for path-dependent payoffs
  (Asian, lookback, basket-default) is needed; the optimal
  `Q_tilde` is non-trivial and depends on the payoff structure.
  **Source:** Glasserman §4 pp.235-260.
