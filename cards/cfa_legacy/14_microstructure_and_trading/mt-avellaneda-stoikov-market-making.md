---
schema_version: "cacg.v0"
id: "mt-avellaneda-stoikov-market-making"
title: "Dynamic Market Making under Inventory Risk: Optimal Bid/Ask Depth Postings"
reading_id: "14_microstructure_and_trading"
summary: "A market maker chooses bid/ask limit-order depths by solving an HJB problem that trades margin-per-fill against inventory risk, so quotes skew and mean-revert as inventory accumulates."
tags: ["microstructure", "market-making", "inventory-risk", "hjb", "limit-orders", "optimal-quotes"]
citations:
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p265:0344"
    chunk_hash: "3418b8e66550672179b36de047b02901e1fcf90955b775a2b42ce2c23e4b1997"
    page_range: [265, 266]
    quote: "also induces mean reversion to an optimal inventory"
    edge_type: "defines"
---
# Dynamic Market Making under Inventory Risk: Optimal Bid/Ask Depth Postings

## Intuition

A market maker (MM) earns the spread by posting a limit sell order above the midprice and a limit buy order below it. Each posting faces a tension: post deep (far from the midprice) and you capture a large margin per fill but rarely get hit; post shallow (close to the midprice) and you fill often but earn little per trade. The MM controls the *depths* `delta+` (sell side) and `delta-` (buy side) measured from the midprice `S`, and the chance a posting gets filled — the *fill probability* — falls off with depth. Optimal quoting balances margin-per-fill against fill rate.

Layered on top is inventory risk. Every fill moves inventory `q` one unit (a sell fill takes `q` down, a buy fill takes it up), exposing the MM to adverse price moves on the held position. So the MM does not quote symmetrically: when she is long, she wants to lean against the position — quoting her sell side more aggressively (shallower) and her buy side more passively (deeper) — to shed inventory. This *skew* pushes inventory back toward a target (near zero).

```
        depth from midprice
  buy LO  <--- delta- ---|S|--- delta+ --->  sell LO
                          ^
                       midprice
  long inventory (q>0):  shrink delta+ (sell eagerly), widen delta- (buy reluctantly)
  short inventory (q<0): mirror image
  fill prob  P(delta) = e^{-kappa*delta}  (decreasing in depth)
  margin per fill = delta ;  expected edge per side ~ delta * e^{-kappa*delta}
```

This is the Avellaneda-Stoikov / Cartea-Jaimungal-Penalva market-making problem: a stochastic-control formulation where the quoting policy emerges from maximizing terminal cash net of an inventory penalty.

**Source:** Cartea, Jaimungal & Penalva (2015) §10.2 pp.247-254

## Definition

Let the midprice follow `dS = sigma dW`. The MM posts limit orders at depths `delta+`, `delta-` from `S`. Market buy (sell) orders arrive as Poisson processes with rates `lambda+` (`lambda-`); a posted limit order at depth `delta` is filled with probability `e^{-kappa delta}` (fill probability decays exponentially in depth). Each fill changes cash `X` by `S +/- delta` and inventory `q` by one unit. Inventory is capped in `[q_min, q_max]`.

The MM maximizes the performance criterion (book Eq. 10.2-10.3): terminal cash plus mark-to-market inventory liquidated at `T` with a quadratic terminal-liquidation penalty `alpha q^2`, minus a *running inventory penalty* `phi * integral of q_u^2 du`, with `alpha >= 0`, `phi >= 0`. The value function is `H(t,x,S,q) = sup over admissible quoting strategies of that expectation`. The book takes the ansatz `H(t,x,q,S) = x + q S + h(t,q)`: accumulated cash, marked-to-market inventory, plus `h(t,q)`, the extra value of trading optimally to `T`.

**Source:** Cartea, Jaimungal & Penalva (2015) §10.2 pp.247-249

## Mathematical Reasoning

The Dynamic Programming Principle yields the HJB equation (Eq. 10.4): the time derivative plus the diffusion term `(1/2) sigma^2 H_SS`, minus the running penalty `phi q^2`, plus arrival-rate terms `lambda+ sup_{delta+} { e^{-kappa+ delta+}(H(t, x + S + delta+, q-1, S) - H) }` and the mirror `lambda-` term, with terminal condition `H(T,x,S,q) = x + q(S - alpha q)`. Substituting the ansatz collapses the PDE to one for `h(t,q)`.

Optimizing pointwise over each side gives the feedback controls (Eq. 10.8):
`delta+,*(t,q) = 1/kappa+ - h(t,q-1) + h(t,q)` and `delta-,*(t,q) = 1/kappa- - h(t,q+1) + h(t,q)`.

Each optimal depth decomposes into two pieces. The first, `1/kappa`, is the unconstrained-inventory solution: setting `alpha = phi = 0` and unbounded `q`, the problem reduces to maximizing expected edge `delta e^{-kappa delta}` per side; the first-order condition gives `delta* = 1/kappa` (Eq. 10.13, 10.16) — a constant, symmetric depth that maximizes the probability-weighted margin regardless of inventory or time. The second piece, `-h(t,q-1) + h(t,q)`, is the inventory adjustment: it shrinks the sell-side depth when long (raising the chance of selling) and conversely on the buy side, and through `h` it induces mean reversion of inventory toward a target level driven by the penalties `phi`, `alpha` and the approach of `T`.

When fill intensities are symmetric (`kappa+ = kappa- = kappa`), substituting `h(t,q) = (1/kappa) log w(t,q)` linearizes the system into a matrix ODE `w'(t) + A w(t) = 0` whose solution `w(t) = e^{A(T-t)} z` is closed-form (Eq. 10.10-10.11). Comparative statics: optimal depth is decreasing in inventory and in time-to-go (as `T` nears, the MM quotes more aggressively to flatten), and a larger penalty (`alpha` or `phi`) pushes terminal/running inventory harder toward zero.

**Source:** Cartea, Jaimungal & Penalva (2015) §10.2-10.2.1 pp.248-254

## Boundary Notes

The model assumes a driftless diffusive midprice and exogenous Poisson market-order arrivals with a fixed exponential fill-probability law `e^{-kappa delta}` — there is no adverse selection in the base section: order flow does not predict future price moves, so fills are "fair." Adding informed flow (short-term alpha) changes the calculus, treated separately in the book's adverse-selection section. The base case also lets depths grow without bound: when `alpha` or `q` is large, the formula can push `delta` negative or to extreme values, where the feedback solution stops being economically sensible and inventory bounds bind (the indicator terms drop one side of the quoting).

Setting `phi = alpha = 0` recovers the inventory-neutral MM (§10.2.1) who quotes the symmetric constant `1/kappa` and ignores inventory entirely — the right benchmark for isolating how much of real spread-setting is inventory management versus pure fill-rate optimization. Contrast with the at-the-touch variant (§10.2.2), where postings are restricted to best bid/offer with a fixed spread and fill-with-probability-one when matched, a discrete on/off control rather than a continuous depth.

**Source:** Cartea, Jaimungal & Penalva (2015) §10.2.1-10.2.2 pp.254-256

## See Also

- [`mt-dealer-inventory-problem-spread`](./mt-dealer-inventory-problem-spread.md) -- the inventory-cost rationale for the bid/ask spread that this dynamic model operationalizes
- [`mt-stochastic-control-hjb-liquidation`](./mt-stochastic-control-hjb-liquidation.md) -- the HJB / dynamic-programming machinery reused here for quoting
- [`mt-grossman-miller-inventory-liquidity-premium`](./mt-grossman-miller-inventory-liquidity-premium.md) -- the inventory liquidity premium underlying why holding `q` is costly
- [`mt-adverse-selection-short-term-alpha`](./mt-adverse-selection-short-term-alpha.md) -- the informed-flow extension that the base model abstracts from

## Escalate to Raw When

This card states the feedback controls (Eq. 10.7-10.8), the unconstrained reduction (Eq. 10.13/10.16), and the linearizing log-transform (Eq. 10.10-10.11) but only sketches the DPP-to-HJB derivation and does not reproduce the matrix-ODE solution `w(t) = e^{A(T-t)} z` or the figures showing depth-vs-time-and-inventory. Re-read Cartea, Jaimungal & Penalva (2015) §10.2 pp.247-254 for the full HJB construction, the ansatz substitution, the boundary/indicator handling at inventory caps, and the numerical behavior in Figure 10.1; §10.2.1 pp.254-255 for the no-inventory-restriction first-order condition; the adverse-selection treatment in §10.4 for the informed-flow correction.
