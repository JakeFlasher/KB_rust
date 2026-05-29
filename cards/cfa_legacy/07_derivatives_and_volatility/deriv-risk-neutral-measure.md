---
schema_version: "cacg.v0"
id: "deriv-risk-neutral-measure"
title: "Risk-Neutral Measure and Numeraire Change"
reading_id: "07_derivatives_and_volatility"
summary: "The risk-neutral measure Q is the equivalent probability measure under which discounted tradable-asset prices are martingales; under Q every asset drifts at the risk-free rate, so the no-arbitrage price equals the discounted expected payoff. Choosing a numeraire other than the bank account (share, T-bond) gives an equivalent measure with the same martingale-pricing logic."
tags: ["derivatives", "risk-neutral"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p670:0999"
    chunk_hash: "f854336599d8da916c4d21e88ef206d1083197703281b837eb33809741f9ec3a"
    page_range: [670, 670]
    quote: "A martingale is a zero-drift stochastic process. A measure is the unit in which we value security prices"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p673:1003"
    chunk_hash: "909d1e430bda4c035a3590e2d381fa9c91ae998b162be1d7a3264e6b14b6bcdd"
    page_range: [673, 673]
    quote: "The market price of interest rate risk is, using the expected return and volatility for the first security"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p675:1007"
    chunk_hash: "b58475a5f1787e6bc35612acd50b12feea3c2c7334a20c0e4fcb3c0e9ce06e3d"
    page_range: [675, 676]
    quote: "The equivalent martingale measure result shows that, when there are no arbitrage opportunities, f is a martingale for some choice of the market price of risk"
    edge_type: "defines"
  - source_id: "cb_glasserman_2003_monte_carlo_methods"
    chunk_id: "cb_glasserman_2003_monte_carlo_methods:p042:0049"
    chunk_hash: "454cac8c39103d443856cfb87c302809a9f60063474c8811b15367df7a7a4d78"
    page_range: [42, 42]
    quote: "This simple transformation is the cornerstone of derivative pricing by Monte Carlo simulation"
    edge_type: "supports"
card_hash: "a5cf5d9bc04c355cb8692d5d4e19a945136ad6fa14252065a2f558f2929b4141"
---
# Risk-Neutral Measure and Numeraire Change

## Intuition

The risk-neutral measure is the artificial probability
distribution under which all tradable assets earn the risk-free
rate in expectation. Pricing under this measure recovers the
no-arbitrage price as the discounted expected payoff. The
mechanism is a probability tilt: tilt the physical-measure
probabilities so that the drift of the underlying becomes the
risk-free rate. Girsanov's theorem makes this rigorous by
constructing the Radon-Nikodym derivative; from a practical
standpoint the risk-neutral measure is just the algebra that
makes all prices into expected values. **Source:** Hull §28
pp.660-680.

```
physical measure P                  risk-neutral measure Q
       |                                       |
       |  dS/S = μ · dt + σ · dW^P             |  dS/S = r · dt + σ · dW^Q
       |  (drift = expected return)            |  (drift = risk-free rate)
       v                                       v
   E^P[ ... ]                              E^Q[ exp(-rT) · payoff ]
   not directly used                       = no-arbitrage price
   for derivative pricing.                 (Hull §28; replication argument).
```

## Definition

A **risk-neutral measure** `Q` is a probability measure
equivalent to the physical measure `P` (assigning the same
events probability zero) under which the discounted price of
every traded asset is a martingale. For a non-dividend-paying
underlying with risk-free rate `r`, the discounted spot
`exp(-r · t) · S(t)` is a `Q`-martingale, equivalent to saying
that under `Q` the underlying drifts at the risk-free rate:
`dS / S = r · dt + σ · dW^Q`. **Source:** Hull §28 pp.660-680.

A **numeraire** is a strictly positive traded asset used as the
unit of account. The risk-neutral measure `Q` corresponds to
choosing the bank account `B(t) = exp(r · t)` as the numeraire.
Other choices — the underlying itself (the share-numeraire
measure that gives BSM's `N(d_1)` term), a zero-coupon bond
maturing at `T` (the `T`-forward measure used for interest-rate
derivatives) — produce equivalent measures with different
martingale-property assets. The fundamental theorem of asset
pricing states that no-arbitrage is equivalent to the existence
of an equivalent martingale measure for some numeraire choice.
**Source:** Hull §28 pp.680-690; Glasserman §1 pp.1-30.

## Mathematical Reasoning

The risk-neutral pricing formula is
`V(t) = numeraire(t) · E^Q[V(T) / numeraire(T) | F_t]`.
For the bank-account numeraire this collapses to
`V(t) = E^Q[exp(-r · (T - t)) · V(T) | F_t]`, the discounted
expected payoff under `Q`. The formula's content is that the
information needed to price any derivative is the joint
distribution of the underlying and the numeraire under `Q`,
not the physical-measure dynamics. The physical drift `μ`
disappears entirely from the price formula; this is why option
pricing depends on volatility but not on expected return.
**Source:** Hull §28 pp.660-690.

Girsanov's theorem is the mathematical tool that constructs
`Q` from `P`. It states that if `dW^Q = dW^P + θ · dt` for an
adapted process `θ`, and if a regularity condition (Novikov's
condition) holds, then the Radon-Nikodym derivative
`dQ / dP = exp(- ∫_0^T θ · dW^P - (1/2) · ∫_0^T θ^2 · dt)`
defines an equivalent measure under which `W^Q` is a Brownian
motion. The shift `θ = (μ - r) / σ` is the market price of
risk; under `Q` the underlying's drift becomes
`μ - σ · θ = r`. The intuition: the physical-measure return
premium `μ - r` is absorbed into the measure change, so the
new measure rewards no risk. **Source:** Hull §28 pp.680-690.

A change of numeraire from `B` (bank account) to `N` (some
other traded asset) tilts `Q` to a new measure `Q^N` under
which discounted prices using `N` are martingales. The
share-numeraire measure makes `S(t) / N(t) = 1` martingale (a
trivial statement) and reproduces the `N(d_1)` term in BSM as
a probability under `Q^S`. The `T`-forward measure (numeraire
= zero-coupon bond `P(t, T)`) is convenient for interest-rate
derivative pricing because the forward rate is a `Q^T`-
martingale (the boundary cross-link into the 06 short-rate /
HJM machinery). **Source:** Hull §28 pp.680-690.

## See Also

- [`deriv-binomial-tree-valuation.md`](deriv-binomial-tree-valuation.md) — discrete-time analogue: `q = (exp(r · dt) - d) / (u - d)` is the binomial risk-neutral probability
- [`deriv-bsm-formula.md`](deriv-bsm-formula.md) — continuous-time application: BSM PDE is the Feynman-Kac counterpart of the risk-neutral expectation
- [`deriv-monte-carlo-pricing.md`](deriv-monte-carlo-pricing.md) — practical estimation: simulate paths under `Q` and average the discounted payoff

## Escalate to Raw When

Open Hull chapter 28 or Glasserman chapter 1 directly when any
of the criteria below applies. **Source:** Hull §28 pp.660-690;
Glasserman §1 pp.1-30.

- The card needs Girsanov's theorem at full rigor (Novikov
  condition, exponential local-martingale property, semi-
  martingale framework). **Source:** Hull §28 pp.680-690.
- Multi-currency / quanto pricing or convexity-adjustment
  applications need the multi-numeraire framework.
  **Source:** Hull §28 pp.680-690.
- Interest-rate derivative pricing under the `T`-forward
  measure is needed; that uses the 06 short-rate /
  HJM machinery. **Source:** Hull §31 pp.720-750.
