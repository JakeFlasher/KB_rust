---
schema_version: "cacg.v0"
id: "mt-lit-dark-pool-optimal-execution"
title: "Optimal Execution Across Lit and Dark Venues with Order-Flow Trends"
reading_id: "14_microstructure_and_trading"
summary: "An execution agent splits a liquidation between a lit market (paying temporary impact while walking the LOB) and a midprice-pegged dark pool (impact-free but with random execution risk), slowing lit trading to leave room for dark fills."
tags: ["microstructure", "optimal-execution", "dark-pool", "crossing-network", "market-impact", "execution-risk"]
citations:
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p192:0252"
    chunk_hash: "1b053bb4aade60f775ff617d7a7164722e359bc82de3cf838dc60a500da67d4f"
    page_range: [193, 193]
    quote: "exposed to execution risk, but on the other hand does not receive the additional"
    edge_type: "defines"
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p147:0232"
    chunk_hash: "c39720fb92468315ac1e105fac8dbea10b5bdd3956b72f8180c83255e25e98e9"
    page_range: [147, 147]
    quote: "Less than 10 percent of their order volume ever crosses."
    edge_type: "supports"
card_hash: "96cc1e955573a21ab8334500f98986f52e2bad8625aa5481faa8767f3990e3b0"
---
# Optimal Execution Across Lit and Dark Venues with Order-Flow Trends

## Intuition
A trader liquidating a large block faces two venues with opposite cost profiles. On the **lit market** every market order walks the limit order book, paying a temporary impact that eats into the realized price; the more she rushes, the worse the price. A **dark pool** (here a crossing network) shows no quotes and crosses incoming orders at the midprice pegged from the lit market, so a dark fill carries *no* price impact — but there is no guarantee anyone is on the other side, so fills arrive at random Poisson times. The dark venue trades certainty for free execution: you may get the midprice with no impact, or you may get nothing.

The optimal strategy posts the entire remaining inventory to the dark pool (it is impact-free, so there is never a reason to withhold) while simultaneously feeding the lit market more slowly than she would absent a dark pool. The slowdown reserves inventory in case a dark cross arrives; if the clock runs out with no cross, she accelerates lit trading to finish before the deadline.

```
          posted (full inventory, impact-free)
   AGENT  ------------------------------------->  DARK POOL  (mid-pegged,
     |                                              random Poisson fills)
     |  v_t dt  (market orders, walk the LOB)
     +---------------------------------------->  LIT MARKET (temporary impact k)
   inventory leaks two ways: continuous lit drain + discrete dark cross
```

**Source:** Cartea, Jaimungal & Penalva (2015) §7.4 pp.176-182.

## Definition
The agent must liquidate `Q_0` shares by terminal time `T`. The midprice `S_t` is a Brownian motion. Trading `v_t dt` in the lit market yields `S_t - k v_t` per share (`k > 0`, temporary impact). She also posts `y_t <= q_t` shares in the dark pool; matching buy orders arrive as a compound Poisson process — Poisson arrivals `N_t` with intensity `lambda` and i.i.d. fill volumes `xi_j` — and crossed shares execute at the midprice `S_t` with no impact. Inventory evolves as

```
dQ_t = - v_t dt - min(y_t, xi_{N_t-}) dN_t
```

(first term: lit market orders; second: discrete dark crosses). The performance criterion adds a terminal-liquidation penalty `-a q^2` and a running inventory penalty `-phi q^2`. Because the dark cross is impact-free and pegged to mid, "the trader who sends orders to the dark pool is exposed to execution risk, but on the other hand does not receive the additional temporary price impact of walking the LOB."

**Source:** Cartea, Jaimungal & Penalva (2015) §7.4 pp.176-178.

## Mathematical Reasoning
The value function `H(t,x,S,q)` satisfies a dynamic-programming equation with a `sup_v` term (lit speed) and a `sup_{y<=q} E[...]` term (dark posting, expectation over random fill volume `xi`). The ansatz `H = x + qS + h(t,q)` reduces it to an equation for `h`, with first-order condition giving the optimal lit speed `v* = -(1/2k) partial_q h`.

When the agent's order is small relative to dark inflow (`xi_i >> Q_0`), every posted order fills in full, so `min(y,xi)=y` and the dark supremum is solved in closed form. With the quadratic ansatz `h = h_0 + h_1 q + h_2 q^2`, the optimal dark volume is `y* = q + h_1/(2 h_2)`, and since `h_1` solves a linear ODE vanishing at `T`, `h_1 = 0` — hence `y* = q`: **always post the entire remaining inventory** (no impact means no reason to hold back). The lit speed becomes `v* = -(1/k) h_2(t) Q_t`.

The remaining ODE for `h_2` is Riccati type:
```
partial_t h_2 - phi - lambda h_2 + (1/k) h_2^2 = 0,   h_2(T) = -a
```
This differs from the no-dark-pool Almgren–Chriss case (set `lambda = 0` and it reduces exactly to that benchmark) only by the `-lambda h_2` term, which represents a **"leakage" of inventory** from the possibility of a full dark cross. Comparative statics: as `lambda` grows the agent front-loads less in the lit market; in the limit `lambda -> infinity` a dark cross is essentially guaranteed and she does **not trade in the lit market at all** until forced to clear at `T`. As `lambda -> 0` the optimal inventory path recovers the convex Almgren–Chriss trajectory; for `lambda > 0` the lit trading rate may be increasing or decreasing and the inventory curve loses its fixed convexity, since slow early trading must be compensated by faster late trading if no cross arrives.

**Source:** Cartea, Jaimungal & Penalva (2015) §7.4.1 pp.178-182.

## Boundary Notes
The clean closed form relies on **full execution**: dark inflow dwarfs the agent's order (`xi_i >> Q_t`), so posts always fill entirely. Drop this and `min(y,xi)` is genuinely binding and one must resort to numerics or impose more structure. The model also assumes the agent sits at the **front of the dark sell queue** (first to cross); accounting for orders ahead adds another random variable but does not change the approach. The midprice is driftless here — with a midprice drift the `h_0, h_1` terms no longer vanish, coupling the strategy to a trend (this is the order-flow / trend-following channel studied in §7.3, where going with or against net order flow adjusts the trading rate). Crucially, dark execution risk is real: empirically crossing networks fill only a small fraction of submitted volume ("Less than 10 percent of their order volume ever crosses"), so relying on `lambda -> infinity` is a modeling limit, not a practical guarantee.

**Source:** Cartea, Jaimungal & Penalva (2015) §7.4.1 pp.179-180; Harris (2003) §6.5 p.147.

## See Also
- [`mt-market-transparency-dark-pools`](./mt-market-transparency-dark-pools.md) -- institutional mechanics of dark pools / crossing networks that this execution model abstracts.
- [`mt-almgren-chriss-optimal-execution`](./mt-almgren-chriss-optimal-execution.md) -- the no-dark-pool benchmark recovered when `lambda = 0`.
- [`mt-limit-order-book-mechanics`](./mt-limit-order-book-mechanics.md) -- the lit-venue LOB whose walking generates the temporary impact `k`.
- [`mt-block-trader-upstairs-depth`](./mt-block-trader-upstairs-depth.md) -- alternative impact-avoiding venue for large blocks.

## Escalate to Raw When
The card sketches the Riccati derivation, the `h_1 = h_0 = 0` vanishing argument, and the explicit closed-form solution for `h_2` (roots `zeta_pm`, the `sinh` limit forms). For the full integration of the Riccati equation, the exact constants, the `a -> infinity` and `lambda -> 0` limiting trajectories, and the order-flow/drift extension (Exercise E.7.3, §7.3), re-read Cartea, Jaimungal & Penalva (2015) §7.4-7.4.1 pp.176-182 and the chapter exercises.
