---
schema_version: "cacg.v0"
id: "ec-consumer-utility-and-demand"
title: "Consumer Utility and Demand"
reading_id: "02_economics"
summary: "MWG Ch.2-3 consumer's Walrasian demand from utility maximization s.t. budget; indirect utility v(p,w) and expenditure function e(p,u) as duals; Slutsky decomposition separating income from substitution effects."
tags: ["economics", "consumer-utility"]
citations:
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p036:0048"
    chunk_hash: "776999c6fcdd7f74b13fc8ae2b5859f31c9d731fed310f758e0db3b821699a77"
    page_range: [36, 37]
    quote: "In terms of the choice-based approach to individual decision making introduced in Section 1.C, the Walrasian demand function is the consumer's choice rule."
    edge_type: "defines"
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p089:0133"
    chunk_hash: "416e5e080bf818dc07523aa8fff70e4a1071913235464d397edab8d28f86006d"
    page_range: [89, 90]
    quote: "This important result, known as the Slutsky equation, means that the properties listed in Proposition 3.G.2 translate into restrictions on the observable Walrasian demand function x(p, w)."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p585:0802"
    chunk_hash: "052e3107129d9ca8afe9f367f1ef0972e6a5e6fdef8d20be48c42a225f94a5aa"
    page_range: [585, 585]
    quote: "The slope of the demand curve is measured as the change in price, P, divided by the change in quantity, Q (∆P/∆Q, where ∆ stands for “the change in”)."
    edge_type: "supports"
card_hash: "8752efb7814ee0e4cb77f07e8cf75ead498885ee1bee47df744ff35788746ad4"
---
# Consumer Utility and Demand

## Intuition

A consumer maximizes utility subject to a budget constraint; the solution to that program is the Walrasian demand function `x(p, w)`. The geometry is the tangency between the highest reachable indifference curve and the budget line: at the optimum, the slope of the indifference curve (the marginal rate of substitution, MRS) equals the slope of the budget line (the price ratio). The indirect utility function `v(p, w)` is the maximum utility attainable at prices `p` and wealth `w`; the expenditure function `e(p, u)` is the minimum wealth required to achieve utility level `u` at prices `p`. These two functions are duals — one is the value of the primal max problem, the other is the value of the dual min problem. **Source:** Mas-Colell et al. (1995) Ch.3 pp.50-65.

```
<!-- primitive: indifference-and-budget source: _diagram_primitives.md -->
   y
   ^
   |  \
   |   \
   |    \                         budget line: p_x x + p_y y = m
   |     \                        slope = -p_x / p_y
   |      \  IC_2
   |       \  .
   |        \  *  <- (x*, y*) optimal bundle
   |         \. (tangency: MRS = p_x / p_y)
   |          \ IC_1
   |           \.
   |            \  .
   |             \   .
   |              \    . IC_0
   +---------------\---------*--------------> x
                                m / p_x

   IC_0 < IC_1 < IC_2: indifference curves (higher = more utility)
   *: tangency between budget line and IC_1 (highest reachable)
```

The Slutsky equation decomposes the total response of demand to a price change into a substitution effect (the consumer's reallocation along the original indifference curve) and an income effect (the consumer's wealth-equivalent gain or loss from the price change). Both effects are signed; for normal goods the substitution effect is unambiguously negative while the income effect varies in sign across normal vs inferior goods. **Source:** Mas-Colell et al. (1995) Ch.3 pp.70-82.

## Definition

The Walrasian (Marshallian) demand is the solution to the utility-maximization problem. **Source:** Mas-Colell et al. (1995) pp.17-104.

```
x(p, w) = argmax_{x ≥ 0}  u(x)
            s.t.  p · x ≤ w
```

The indirect utility function evaluates the maximized utility at the demand bundle. **Source:** Mas-Colell et al. (1995) pp.17-104.

```
v(p, w) = u(x(p, w))
```

The Hicksian (compensated) demand is the solution to the dual expenditure-minimization problem. **Source:** Mas-Colell et al. (1995) pp.17-104.

```
h(p, u) = argmin_{x ≥ 0}  p · x
            s.t.  u(x) ≥ u
```

The expenditure function evaluates the minimized wealth at the Hicksian demand. **Source:** Mas-Colell et al. (1995) pp.17-104.

```
e(p, u) = p · h(p, u)
```

Duality between the two problems implies `e(p, v(p, w)) = w` and `v(p, e(p, u)) = u` whenever utility is locally non-satiated. **Source:** Mas-Colell et al. (1995) Ch.3 pp.50-67.

The Slutsky equation links Walrasian and Hicksian demand at the same `(p, w)`. **Source:** Mas-Colell et al. (1995) pp.17-104.

```
∂x_l(p, w) / ∂p_k  =  ∂h_l(p, u*) / ∂p_k  −  x_k(p, w) · ∂x_l(p, w) / ∂w
                       └── substitution ──┘   └────── income effect ──────┘
                       (sign: ≤ 0 for k=l)
```

where `u* = v(p, w)`. **Source:** Mas-Colell et al. (1995) Ch.3 pp.71-74.

## Mathematical Reasoning

The first-order necessary conditions of the Walrasian problem (interior solution) equate the marginal rate of substitution to the price ratio for every pair of goods. With Lagrangian `L = u(x) − λ (p · x − w)`, the FOCs `∂u/∂x_l = λ p_l` for each good `l` imply `(∂u/∂x_l) / (∂u/∂x_k) = p_l / p_k`. The Lagrange multiplier `λ` is the marginal utility of wealth — equal to `∂v(p, w) / ∂w` by the envelope theorem. **Source:** Mas-Colell et al. (1995) Ch.3 pp.50-58.

Three classical properties of the demand function follow from local non-satiation and continuity: homogeneity of degree zero in `(p, w)` (proportional scaling of prices and wealth leaves the demand bundle unchanged), Walras' law (the consumer spends the full budget), and the WARP-equivalent Slutsky matrix property (the substitution matrix `S(p, w)` of `∂h_l/∂p_k` evaluated at `u = v(p, w)` is symmetric and negative semidefinite). Symmetry follows from `e(p, u)`'s concavity in `p` and Young's theorem; negative semi-definiteness follows from `e`'s concavity in `p` for fixed `u`. **Source:** Mas-Colell et al. (1995) Ch.3 pp.65-82.

Roy's identity recovers Walrasian demand from the indirect utility function without solving the FOCs directly: `x_l(p, w) = − (∂v/∂p_l) / (∂v/∂w)`. Shephard's lemma is the dual companion: the Hicksian demand for good `l` equals the partial of the expenditure function with respect to `p_l` — `h_l(p, u) = ∂e(p, u) / ∂p_l`. The two identities convert algebraic forms for `v` or `e` directly into demand functions, bypassing the constrained-optimization problem. **Source:** Mas-Colell et al. (1995) Ch.3 pp.59-70.

## See Also

- [`ec-consumer-preference-and-choice`](./ec-consumer-preference-and-choice.md) — rationality axioms + WARP that justify the utility-maximization framing
- [`ec-aggregate-demand-representative-consumer`](./ec-aggregate-demand-representative-consumer.md) — when does individual demand aggregate to a representative agent
- [`ec-elasticity-and-demand-curves`](./ec-elasticity-and-demand-curves.md) — CFA L1 exam-depth elasticity (price / income / cross-price)

## Escalate to Raw When

The Slutsky-matrix integrability conditions (when WARP-satisfying demand can be derived from a single utility function in `R^L`) sit in MWG Ch.3 pp.75-92 — re-open if a question requires the full integrability proof or the role of homogeneity and Walras' law as integrability conditions. The cardinal-utility variants (von Neumann-Morgenstern, Bernoulli, Allais paradoxes) are NOT in scope here; see `ec-utility-and-choice-under-uncertainty` and MWG Ch.6. For multi-period intertemporal consumer choice (an Euler equation appears in the Ramsey-Cass-Koopmans card under primary-romer), see `ec-ramsey-cass-koopmans-savings` and Romer Ch.2. **Source:** Mas-Colell et al. (1995) pp.17-104.
