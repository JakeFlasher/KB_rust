---
schema_version: "cacg.v0"
id: "deriv-binomial-tree-valuation"
title: "Binomial-Tree Option Valuation"
reading_id: "07_derivatives_and_volatility"
summary: "The binomial tree replaces continuous geometric Brownian motion with a discrete two-state lattice. Backward induction discounts risk-neutral expected child values with q = (e^{r·dt} - d) / (u - d). The CRR parameterization u = exp(σ·√dt) makes the lattice price converge to BSM; American options use max(intrinsic, continuation) at each node."
tags: ["derivatives", "binomial-tree"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p302:0453"
    chunk_hash: "ffea02555852d25547873f4529c5d3e38ecf98323ff0b7abe69053bdd5d5c008"
    page_range: [302, 303]
    quote: "When the binomial tree is used to price a European option, the price converges to the Black–Scholes–Merton price, as expected, as the number of time steps is increased."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2900:4348"
    chunk_hash: "ad09121dafdab5d739acc15a298478fdd64c9d98bcff3888dac20e20898c3974"
    page_range: [2900, 2901]
    quote: "This model with two possible outcomes is called the binomial model."
    edge_type: "supports"
card_hash: "10320800cd76c44e260d2e88a4d562905fc2af081d19e47ef11a4687ebab26ee"
---
# Binomial-Tree Option Valuation

## Intuition

The binomial tree replaces the continuous price process with
a discrete two-state lattice: at each step the underlying
either jumps up by factor `u` or down by factor `d`. The
option price at each node is the discounted expected payoff
under the risk-neutral measure `q = (exp(r · dt) - d) / (u -
d)`. Backward induction starts from the terminal payoff and
folds back to the root. The tree is the discrete-time
analogue of the Black-Scholes PDE: it converges to the BSM
closed-form as the time-step shrinks. **Source:** Hull §13
pp.302-320.

```
multi-period binomial lattice (3-period example)

                                 u^3 · S
                            *
                       *
                  *           u^2 d · S
            *           *
       *           *
  S         *           *      u d^2 · S
       *           *
            *           *
                  *           d^3 · S
                       *
                            *
   each up-step multiplies by u; each down-step multiplies by d.
```

## Definition

A one-period binomial tree on a non-dividend-paying underlying
specifies the spot `S`, the up-factor `u > 1`, the down-factor
`d < 1`, the per-period risk-free rate `r`, and the period
length `dt`. After one step the underlying takes value `u · S`
(up-state) or `d · S` (down-state). The risk-neutral probability
`q = (exp(r · dt) - d) / (u - d)` makes the discounted spot a
martingale: `S = exp(-r · dt) · (q · u · S + (1 - q) · d · S)`.
**Source:** Hull §13 pp.302-320; CFA L1 Curriculum (2022)
Vol.5/pp.420-440.

The European-option price at the root is
`V_0 = exp(-r · dt) · (q · V_u + (1 - q) · V_d)`, where `V_u`
and `V_d` are the option's terminal payoffs at the up- and
down-states. For an American option the root price is
`V_0 = max(intrinsic_payoff, exp(-r · dt) · (q · V_u + (1 - q)
· V_d))`, where the early-exercise check compares immediate
payoff to the discounted continuation value. **Source:** Hull
§13 pp.302-320; Hull §17 pp.398-420.

## Mathematical Reasoning

The risk-neutral probability `q` is derived from a no-arbitrage
replication argument. A portfolio of `Δ` shares of underlying
plus `B` cash, chosen so that the portfolio's terminal value
equals the option's terminal payoff in both states, has present
value `Δ · S + B`. Setting that equal to the option price `V_0`
and solving the two-equation system for `Δ` and `B` yields
`V_0 = exp(-r · dt) · (q · V_u + (1 - q) · V_d)` with
`q = (exp(r · dt) - d) / (u - d)`. The probability `q` does NOT
depend on the physical-measure probability of the up-move; it is
purely an algebraic artifact of the no-arbitrage condition.
**Source:** Hull §13 pp.302-320.

The multi-period tree is constructed by iterating the one-period
step. The Cox-Ross-Rubinstein parameterization sets
`u = exp(σ · sqrt(dt))` and `d = 1 / u` so that the lattice
matches the variance of a continuous-time geometric Brownian
motion with volatility `σ` as `dt → 0`. The European-option
price at the root is the discounted expected terminal payoff
under `q^N`, where `N` is the number of steps; the price
converges to the BSM closed-form as `N → ∞`. **Source:**
Hull §13 pp.310-330.

The American-option valuation requires comparing immediate
exercise to continuation at every node, not just at the
terminal date. Backward induction starts at the terminal nodes
(where the value is the intrinsic payoff), folds back one step
at a time, and at each interior node sets
`V_node = max(intrinsic_node, exp(-r · dt) · (q · V_up_child + (1 - q) · V_down_child))`.
The full continuation value is the discounted expected child
value, so both probability-weighted children sit inside the
discount factor's parentheses. The American premium over the
European price is positive only when the early-exercise region
is non-empty (e.g. American puts on positive-rate underlyings;
American calls on dividend-paying underlyings). **Source:**
Hull §17 pp.398-420.

## See Also

- [`deriv-option-payoff-anatomy.md`](deriv-option-payoff-anatomy.md) — terminal payoff that seeds the backward induction
- [`deriv-no-arbitrage-bounds.md`](deriv-no-arbitrage-bounds.md) — replication-dominance arguments that justify the risk-neutral price
- [`deriv-risk-neutral-measure.md`](deriv-risk-neutral-measure.md) — equivalent martingale measure that underpins the binomial probability `q`

## Escalate to Raw When

Open Hull chapters 13 and 17 directly when any of the criteria
below applies. **Source:** Hull §13 pp.302-340; §17 pp.398-420.

- Multi-asset / basket trees, recombining vs non-recombining
  trees, or implicit-finite-difference equivalents are needed.
  **Source:** Hull §17 pp.398-420.
- The card needs convergence-rate analysis (`O(1 / N)` error
  for European options, `O(1 / sqrt(N))` for American options
  under standard parameterizations). **Source:** Hull §17
  pp.398-420.
- Continuous-time BSM closed-form derivation is needed; that
  card is `deriv-bsm-formula.md`. **Source:** Hull §15
  pp.346-380.
