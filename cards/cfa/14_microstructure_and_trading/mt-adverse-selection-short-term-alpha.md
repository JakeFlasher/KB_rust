---
schema_version: "cacg.v0"
id: "mt-adverse-selection-short-term-alpha"
title: "Market Making with Adverse Selection and Short-Term Alpha"
reading_id: "14_microstructure_and_trading"
summary: "A market maker's passive limit orders are picked off when the midprice jumps with incoming order flow; modelling that flow as midprice jumps or an order-driven short-term-alpha drift lets the optimal quoting strategy price in adverse selection."
tags: ["microstructure", "adverse-selection", "market-making", "short-term-alpha", "order-flow", "limit-order"]
citations:
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p283:0368"
    chunk_hash: "7f53e6971f84c58efa0faede27681919bd253f64852f4bf522f1360857e718ed"
    page_range: [284, 284]
    quote: "traders who are better informed will pick-off the LOs posted by the less informed MM"
    edge_type: "defines"
card_hash: "9a0a70aea83236a1900d2732bc670594642c89900ba67f6b4244f3c6a6282d5a"
---
# Market Making with Adverse Selection and Short-Term Alpha

## Intuition
A market maker (MM) supplies liquidity by posting passive limit orders (LOs) on both sides of the book. Her standing quotes are options that other traders may exercise against her. The danger is that the counterparties who choose to hit her are, on average, better informed about where the price is going next: a market buy order that lifts her sell LO tends to arrive precisely when the asset is about to tick up, and a market sell that hits her buy LO arrives just before a tick down. This is *adverse selection* — she is filled right before the midprice moves against her position. The trades that look most attractive ex ante are systematically the ones that lose money ex post.

```
   informed buy MO arrives
   ----------------------->  [ MM sell LO @ S+δ ]  ====FILLED====
                                                          |
   midprice S  ------o------------o'  (jumps UP by ε after the fill)
                     ^ fill here   ^ MM now short at a stale price
```

Cartea, Jaimungal, and Penalva formalize this in two complementary ways. Either the midprice itself *jumps* in the direction of each incoming market order (MO) — a permanent price-impact view of informed trading — or the midprice carries a mean-reverting *short-term-alpha* drift component that is bumped up by buy MOs and down by sell MOs. In both views, order flow is the signal: trading pressure forecasts the next move, so an MM who cannot "see" that flow is the natural prey of those who can.

**Source:** Cartea, Jaimungal & Penalva (2015) §10.4–§10.4.2 pp.261–267.

## Definition
Two midprice specifications drive the MM problem (extending the inventory-control market-making setup of §10.2):

1. **Market-order impact (§10.4.1).** The midprice is the sum of a diffusion plus a jump process, `dS_t = σ dW_t + ε⁺ dM_t⁺ − ε⁻ dM_t⁻`, where `M_t±` are Poisson processes (intensities `λ±`) counting buy (+) and sell (−) MOs, and the i.i.d. jump sizes `ε±` have means `c± = E[ε±]`. The diffusion captures broadcast information; the jumps capture the permanent price impact of MOs.

2. **Short-term-alpha (§10.4.2).** The midprice drift splits into a long-term constant `ν` and a predictable zero-mean-reverting short-term component `α_t`: `dS_t = (ν + α_t) dt + σ dW_t`. The alpha is an Ornstein–Uhlenbeck-type process that jumps with order flow, `dα_t = −ζ α_t dt + η dW_t^α + ε⁺ dM_t⁺ − ε⁻ dM_t⁻`, mean-reverting at rate `ζ` and jumping up on buy MOs, down on sell MOs.

The MM maximizes terminal mark-to-market wealth net of an inventory penalty over admissible (predictable, inventory-constrained) posting strategies, with value function `H` solving the corresponding dynamic-programming equation (DPE) under terminal condition `H(T,x,S,q)= x + q(S − αq)` (and the analogous `α`-augmented form in §10.4.2).

**Source:** Cartea, Jaimungal & Penalva (2015) §10.4.1 eq.(10.22)–(10.24), §10.4.2 eq.(10.30)–(10.31) pp.262–268.

## Mathematical Reasoning
Make the ansatz `H(t,x,S,q) = x + qS + h(t,q)` (book value of cash and inventory, plus a value-of-market-making term `h`). Substituting into the DPE for the MO-impact model and solving the `sup` over the half-spreads yields optimal postings in feedback form whose key feature is the explicit appearance of the conditional jump mean:

```
δ⁺,*(t,q) = 1/κ + c⁺ − h(t,q−1) + h(t,q)        (sell-side offset)
δ⁻,*(t,q) = 1/κ + c⁻ − h(t,q+1) + h(t,q)        (buy-side offset)
```

The `c± = E[ε±]` term widens each quote *away from the midprice by the expected adverse jump conditional on a fill*. Intuitively, the MM posts a sell LO at a half-spread that already includes the expected upward jump she will suffer when an informed buyer lifts it; symmetrically on the buy side. In this way, on average, she recovers the loss she would otherwise hand to informed flow. Crucially the authors stress this is **not** simply the no-adverse-selection control plus a `c±` add-on: the future arrival of price-moving MOs also feeds back through the solution of `h(t,q)`, so the whole policy reoptimizes.

In the short-term-alpha model, the ansatz becomes `H = x + qS + h(t,α,q)`, and the reduced equation for `h` contains a source term `α q`. This single term is what couples the policy to `α`: were it absent, the terminal condition's `α`-independence would make `h` and the optimal posts independent of `α`. Its presence lets the strategy *adapt* to the adverse selection induced by order flow — when `α` is positive (buy pressure forecasting an up-move) the MM skews her quotes and is willing to hold inventory long, and conversely when `α` is negative. Comparative statics from the solved surface: far from maturity the policy is time-stationary and symmetric near `α ≈ 0`; as `|α|` grows the MM tilts to one side and eventually posts on only the favorable side; as maturity approaches the policy becomes `α`-insensitive and instead liquidates toward zero inventory.

**Source:** Cartea, Jaimungal & Penalva (2015) §10.4.1 eq.(10.25)–(10.27), §10.4.2 eq.(10.30)–(10.31) and Fig.10.10 pp.263–270.

## Boundary Notes
- **Assumptions.** Poisson (constant-rate `λ±`) MO arrivals; i.i.d. jump sizes independent of all other processes; at-the-touch posting with fill-probability one in the §10.4.2 variant (Exercise E.10.2 generalizes to fill probability < 1); hard inventory bounds `q̄`, `q̲`. The short-term-alpha is *predictable* and zero-mean-reverting, so it forecasts only transient moves, not the long-term drift `ν`.
- **When it holds vs breaks.** The whole framework is load-bearing only when the MM operates fast enough to *observe* the short-term component. An MM who trades at a time scale where `α_t` is invisible is not merely suboptimal — she systematically loses money to better-informed traders who pick off her stale quotes.
- **Contrast.** §10.4.1 books adverse selection as *permanent midprice jumps* (price-impact view); §10.4.2 books it as a *mean-reverting drift signal* (predictive view). The jump model widens spreads statically by `c±`; the alpha model produces a dynamic, inventory- and signal-dependent skew. Both differ from the no-adverse-selection baseline of §10.2, where quotes depend on inventory and horizon alone.

**Source:** Cartea, Jaimungal & Penalva (2015) §10.4.1–§10.4.2 and Exercise E.10.2 pp.262–271.

## See Also
- [`mt-avellaneda-stoikov-market-making`](./mt-avellaneda-stoikov-market-making.md) -- the inventory-control market-making baseline this card augments with adverse selection.
- [`mt-order-imbalance-signal`](./mt-order-imbalance-signal.md) -- order flow / imbalance is the observable that drives the short-term-alpha jumps.
- [`mt-kyle-strategic-informed-trader-lambda`](./mt-kyle-strategic-informed-trader-lambda.md) -- the strategic-informed-trader origin of order-flow price impact.
- [`mt-order-anticipators-front-running`](./mt-order-anticipators-front-running.md) -- the better-informed traders who pick off the MM's stale quotes.

## Escalate to Raw When
You need the explicit closed-form solution of the DPE (the matrix `A`, the log-transform `h = (1/κ) log w`, and the resulting linear ODE system in §10.4.1's "Solving the DPE"), the full `h`-equation and optimal at-the-touch indicators for the short-term-alpha model (§10.4.2), the parameter set and sample-path behavior behind Figures 10.10–10.11, or the fill-probability-<1 generalization (Exercise E.10.2). Re-read pp.263–271 for the derivations this card only sketches.
