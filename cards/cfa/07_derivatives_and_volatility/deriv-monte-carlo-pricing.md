---
schema_version: "cacg.v0"
id: "deriv-monte-carlo-pricing"
title: "Monte Carlo Pricing"
reading_id: "07_derivatives_and_volatility"
summary: "Monte Carlo derivative pricing represents the no-arbitrage price as a risk-neutral discounted expected payoff and estimates it by sample-averaging i.i.d. simulated paths. The LLN gives consistency; the CLT and standard error sigma/sqrt(N) give a dimension-independent O(N^-1/2) convergence rate that is slow but immune to the curse of dimensionality."
tags: ["derivatives", "monte-carlo"]
citations:
  - source_id: "cb_glasserman_2003_monte_carlo_methods"
    chunk_id: "cb_glasserman_2003_monte_carlo_methods:p015:0010"
    chunk_hash: "4dee2295f2110fc2d4d4d76c98e83b8629f35328534a0309581993708798d7a9"
    page_range: [15, 16]
    quote: "The law of large numbers ensures that this estimate converges to the correct value as the number of draws increases"
    edge_type: "defines"
  - source_id: "cb_glasserman_2003_monte_carlo_methods"
    chunk_id: "cb_glasserman_2003_monte_carlo_methods:p016:0011"
    chunk_hash: "f98211a4183e27944c9638036cd8e04fb3a9144cad0e43d6f0cbd78ce14ae428"
    page_range: [16, 17]
    quote: "The form of the standard error σf / √n is a central feature of the Monte Carlo method"
    edge_type: "defines"
  - source_id: "cb_glasserman_2003_monte_carlo_methods"
    chunk_id: "cb_glasserman_2003_monte_carlo_methods:p017:0012"
    chunk_hash: "9bff7bc191fa829e263f7ea903a69513e46d7054048ae79147cce3b24d199fb7"
    page_range: [17, 17]
    quote: "the price of a derivative security can be usefully represented as an expected value"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p352:0519"
    chunk_hash: "5322ab6542f2e9c7cd927a4eaa9c1c327720fac96944c2fcd4572e9be2b8c28b"
    page_range: [352, 352]
    quote: "The expected payoff from the derivative changes and the discount rate that must be used for this payoff changes"
    edge_type: "supports"
card_hash: "000cb0b3fc61cea965bf7269a26a0c2f601d408cf862097f631a6eaabb154f40"
---
# Monte Carlo Pricing

## Intuition

Monte Carlo derivative pricing replaces the analytic risk-
neutral expectation with a sample average over simulated paths
of the underlying. The price is approximated as
`V_hat = (1 / N) · Σ exp(-r · T) · payoff(path_i)`,
and the law of large numbers guarantees `V_hat → V` as the
sample size `N → ∞`. The convergence rate is `O(1 / sqrt(N))`,
which is slow but dimension-independent: this is why MC is the
practitioner default for high-dimensional payoffs (basket
options, path-dependent payoffs, multi-asset exotics) where
PDE solvers struggle. **Source:** Glasserman §1 pp.1-30.

```
monte carlo workflow

    risk-neutral SDE             N independent paths
    dS/S = r dt + sigma dW^Q -->   S^(1)_T, S^(2)_T, ...
                                          |
                                          v
                              payoff(S^(i)_T) values
                                          |
                                          v
                  V_hat = (1/N) sum exp(-rT) payoff(S^(i)_T)
                                          |
                                          v
            standard error: sigma_payoff / sqrt(N)
            (reporter typically gives V_hat plus or minus 1.96 SE)
```

## Definition

The **Monte Carlo price estimator** for a European derivative
with terminal payoff `g(S_T)` is
`V_hat = (1 / N) · Σ_i exp(-r · T) · g(S^(i)_T)`,
where `S^(i)_T` for `i = 1, ..., N` are independent draws of
the terminal underlying price under the risk-neutral measure.
The estimator is unbiased: `E[V_hat] = V` (the true price), and
the standard error of the estimator is
`SE = sigma_g / sqrt(N)`, where `sigma_g^2 = Var(exp(-r · T) ·
g(S_T))` under the risk-neutral measure. Practitioners report
the price as `V_hat ± 1.96 · SE` for an approximate
95-percent confidence interval. **Source:** Glasserman §1
pp.1-15.

The **convergence rate** of MC is `O(1 / sqrt(N))`: doubling
the sample size halves the standard error only by a factor of
`sqrt(2)`. To halve the confidence-interval width, the sample
size must quadruple. This rate is the "MC tax": it is slower
than the `O(1 / N)` rate of binomial trees for European options
or the spectral convergence of analytic-formula evaluation, but
it does not depend on the dimensionality of the underlying — a
basket of `d` assets has the same `1 / sqrt(N)` rate as a
single asset. **Source:** Glasserman §1 pp.15-30.

## Mathematical Reasoning

The MC estimator is justified by the strong law of large
numbers: if `Y_i = exp(-r · T) · g(S^(i)_T)` are i.i.d. with
mean `V` and finite variance `sigma_g^2`, then
`(1 / N) · Σ Y_i → V` almost surely as `N → ∞`. The central
limit theorem gives the asymptotic distribution
`sqrt(N) · (V_hat - V) → N(0, sigma_g^2)`,
which is the basis for the practitioner standard-error
reporting and confidence-interval construction. **Source:**
Glasserman §1 pp.1-15.

The MC convergence rate is dimension-independent because the
sample mean's variance depends only on the variance of the
single-path discounted payoff, not on the dimension of the
state. By contrast, deterministic methods (PDE finite-
difference grids, lattice trees) suffer from the curse of
dimensionality: the grid size grows exponentially in the
number of risk factors. For payoffs with more than three or
four risk factors the practitioner crossover point favors MC
over PDE solvers; this is why exotic basket options, multi-
currency payoffs, and path-dependent features default to MC.
**Source:** Glasserman §1 pp.15-30.

The MC framework relies on the practitioner being able to
sample from the underlying's risk-neutral distribution at
terminal time `T`, OR to simulate the underlying along a path
when path-dependence is needed. For BSM-class lognormal
underlyings the terminal sample is direct (one draw of a normal
random variable per path); for non-trivial dynamics
(stochastic-vol, jump-diffusion, multi-asset with correlated
Brownians) the path simulation requires discretization of the
SDE — the topic of
[`deriv-discretization-and-bias.md`](./deriv-discretization-and-bias.md#definition).
The bias from discretization is separate from the MC sampling
variance and must be controlled separately. **Source:**
Glasserman §3 pp.79-100.

## See Also

- [`deriv-risk-neutral-measure.md`](deriv-risk-neutral-measure.md) — the measure under which the simulated paths are drawn
- [`deriv-variance-reduction.md`](deriv-variance-reduction.md) — techniques to reduce the standard error at fixed sample size
- [`deriv-discretization-and-bias.md`](deriv-discretization-and-bias.md) — the path-simulation bias separate from sampling variance

## Escalate to Raw When

Open Glasserman chapter 1 directly when any of the criteria
below applies. **Source:** Glasserman §1 pp.1-50.

- Variance reduction (antithetic, control, importance sampling)
  is needed; this is the next layer of practitioner technique
  for cutting the standard error at fixed sample size.
  **Source:** Glasserman §4 pp.182-260.
- Quasi-Monte Carlo with low-discrepancy sequences (Sobol,
  Halton) is needed; convergence improves to roughly
  `O(log(N)^d / N)` for moderate dimension.
  **Source:** Glasserman §5 pp.281-340.
- Greek estimation by pathwise / likelihood-ratio / finite-
  difference methods is needed; that is a separate set of
  bias / variance trade-offs handled in
  [`deriv-mc-greek-estimation.md`](deriv-mc-greek-estimation.md#definition).
  **Source:** Glasserman §7 pp.377-420.
