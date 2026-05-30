---
schema_version: "cacg.v0"
id: "mt-limit-order-book-equilibrium"
title: "Limit Order Book Markets: The Limit-vs-Market Order Choice as an Option"
reading_id: "14_microstructure_and_trading"
summary: "In a continuous LOB a limit order is a free option offering price improvement but bearing non-execution and pick-off (adverse-selection) risk, while a market order pays the spread for immediacy; equilibrium order choice trades these off."
tags: ["microstructure", "limit-order-book", "make-or-take", "adverse-selection", "free-option"]
citations:
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p204:0312"
    chunk_hash: "2aee46340987dfea841003a560c15d2c5ac9189ec4aca407603b2d0891bb1157"
    page_range: [205, 205]
    quote: "submitting a sell limit order is similar to writing an American call option"
    edge_type: "defines"
card_hash: "3fd23be9e00c8689ec8d4c90f49665134cfb78ae4542fba5aabb56be671f74d8"
---
# Limit Order Book Markets: The Limit-vs-Market Order Choice as an Option

## Intuition
In a continuous limit order book (LOB) there is no privileged class of designated market makers: any trader can supply liquidity by posting a limit order or demand it by sending a market order. A market order pays for immediacy — it walks up (or down) the book and executes against resting limit orders at once, paying the bid-ask spread. A limit order does the opposite: it offers price improvement (you buy below or sell above the mid) but you wait, and execution is neither certain nor innocent.

The sharp insight is that a posted limit order behaves like a *free written option*. A resting sell limit order at ask `A` is like a written American call struck at `A`: whoever the market lets in can "buy from you at `A`," and they will choose to do so precisely when that is good for them and bad for you. The option is "free" because you receive no premium for writing it; it can still be worthwhile because some incoming market orders come from liquidity traders who must trade regardless of news, so the order sometimes fills profitably even when it is "out of the money."

```
   limit-vs-market order choice
   ----------------------------------------
   MARKET ORDER  --> immediate fill, pay spread,  no pick-off / no waiting
   LIMIT  ORDER  --> price improvement, BUT:
                        (1) risk of NON-EXECUTION  (queue never reached)
                        (2) risk of being PICKED OFF (fills when news moves against you)

   fair value v_t            sell limit @ A  (written call, strike A)
        |                          |
        |<---- price improvement ->|  (you earn A - v if a liquidity buyer arrives)
        |                          |
        +-- if good news pushes v above A --> informed buyer exercises -> you lose
```

So liquidity in a LOB is not posted by fiat; it *emerges* from how patient traders trade off these costs and benefits. **Source:** Foucault, Pagano & Roell (2013) ch.6 §6.4.4 Box 6.3 pp.205-206

## Definition
Setup (FPR §6.2 / §6.4.1): a risky security has fair value `v_t` that follows `v_t = v_{t-1} + ε_t`, where each per-period innovation `ε_t = +σ` (good news) or `−σ` (bad news) with equal probability, so `σ` is per-period volatility. Each period the asset pays off with probability `1 − τ` (so `τ` is an inverse measure of trader impatience). Sequentially arriving risk-neutral traders observe `v_t` and the LOB and have a private value `y_i ∈ {+L, −L}`; trader utility from `q ∈ {+1, 0, −1}` shares at price `p` is `U = q·(v_T + y_i − p)`.

A **limit order** posts a price and waits; it executes only if a later market order reaches it (execution probability `P_t < 1`). A **market order** executes immediately at the best available quote. A **competitive (no-entry/no-exit) equilibrium** of the static book fills each price up to the cumulative depth where the marginal limit order earns zero expected profit: there is no price at which adding a limit order is profitable and none at which cancelling one is.

The **execution probability** of the marginal share at ask `A_k` with cumulative depth `Y_k` is `P(Y_k) = Pr(q ≥ Y_k) = 1 − F(Y_k)`, declining in queue depth. **Source:** Foucault, Pagano & Roell (2013) ch.6 §6.2-§6.4.1 pp.196-215

## Mathematical Reasoning
**Static zero-profit depth.** For the marginal sell limit unit at `A_k`, profit on execution is `A_k − v − C` (display/monitoring cost `C`), and `−C` on non-execution, so with uninformed flow `E(v | q ≥ Y_k) = μ`:

```
   Π_k(Y_k) = P(Y_k)·(A_k − μ) − C
```

The no-entry/no-exit condition sets `Π_k = 0`, giving the equilibrium execution probability `P(Y_k) = C / (A_k − μ)`. Since a buy market order arrives with probability ½, `P ≤ ½`, so no sell limit posts below `A* = 2C + μ`: closer to the mid, the option's payoff cannot cover its cost.

**Make-or-take with informed flow.** A one-period buy limit at bid `B` for a type-`y_i` trader has expected utility

```
   E_t[U] = [(v_t + y_i − B) + E(ε_{t+1} | q_{t+1}=1)]·P_t(B)
```

The bracket's first term `(v_t + y_i − B)` is the gain under *innocent* execution; the second term `E(ε_{t+1} | q_{t+1}=1)` captures **pick-off / adverse selection**. Let `φ` be the probability that value *rose* conditional on execution. A buy limit is more likely to fill on bad news, so `φ < ½ ⇒ E(ε_{t+1} | q=1) < 0`, eroding the option's value. The limit order is chosen over the market order iff `E_t[U_limit] ≥ E_t[U_market]`; this holds when `P_t` is high (low non-execution risk) and `φ ≈ ½` (low pick-off risk), and fails when both `P_t` and `φ` are small.

**Comparative statics.**
- *Execution risk / eagerness (§6.4.2).* Recall `τ` is an *inverse* measure of impatience: a *higher* `τ` means more eager/patient successors and a higher continuation-arrival probability, so the marginal limit order's fill probability is `P_t = τ/2` (the non-execution probability is `1 − τ/2`). Thus a higher `τ` *raises* `P_t` and *lowers* non-execution risk. Because waiting is now cheaper, limit orders can post *more aggressively* to attract counterparties — ask falls and bid rises (`∂A*/∂τ ≤ 0`, `∂B*/∂τ ≥ 0`) — so the spread `A* − B* = 2L·(2 − τ)/(2 + τ)` *narrows* (it is decreasing in `τ`, hence increasing in execution risk). This is why spreads widen near the close, when fill likelihood drops.
- *Volatility (§6.4.3, NON-MONOTONIC).* A higher `σ` does two opposing things: it raises the pick-off cost (larger "in-the-money" loss on the written option) but it also *erodes the limit-order trader's market power* — to keep a high execution probability the buyer must bid ever higher (and the seller ask ever lower) as `σ` rises. Netting these, the spread is non-monotonic about a threshold `σ̂ = 4L/(4 + τ)`: for `σ ≤ σ̂` the high-fill quotes dominate and the spread *decreases* in `σ` (the market-power erosion wins); only for `σ > σ̂` do traders switch to shaded, low-fill quotes whose execution-only-on-bad-news shading makes the spread *increase* in `σ`.

The option analogy is explicit: a sell limit at `A` is a written American call exercised when "in the money," i.e. when `A` is below fair value. **Source:** Foucault, Pagano & Roell (2013) ch.6 §6.2.2-§6.4.3 pp.197-223

## Boundary Notes
- **Competitive limit.** The zero-profit equilibrium says nothing about *how* the book is reached; it is the limiting benchmark as the number of competing limit-order traders → ∞. With finite, imperfect competition, depth and spreads differ. (FPR §6.2.1)
- **Static dichotomy vs. endogenous choice.** §6.2 fixes traders as either makers or takers; the make-or-take *choice* (§6.4) is what actually closes the model — a LOB is viable only if some traders choose each side.
- **"Free" but not always a loss.** Unlike exchange-traded options, no premium is received, yet limit orders profit because liquidity (uninformed) traders sometimes execute them out of the money. Pick-off risk further depends on *monitoring/routing speed*: faster cancellation (algorithmic/co-located traders) shrinks the written option's value; competition to pick off forces near-immediate exercise as soon as the option is even slightly in the money, capping its value. (Box 6.3)
- **Contrast with auctions.** A continuous LOB is a *discriminatory* auction (each limit order fills at its own posted price); a call market is a *uniform* auction (single clearing price) — so optimal order placement differs across the two designs. **Source:** Foucault, Pagano & Roell (2013) ch.6 §6.2.1, §6.4.4 Box 6.3 pp.192-225

## See Also
- [`mt-prospective-execution-cost-tradeoff`](./mt-prospective-execution-cost-tradeoff.md) -- the price-improvement vs. delayed/uncertain execution trade-off this card formalizes
- [`mt-spread-equilibrium-timing-option`](./mt-spread-equilibrium-timing-option.md) -- how pick-off risk and volatility set the equilibrium bid-ask spread
- [`mt-limit-order-book-mechanics`](./mt-limit-order-book-mechanics.md) -- the discriminatory-auction book mechanics underlying execution probability
- [`mt-market-fragmentation`](./mt-market-fragmentation.md) -- competition between LOB platforms applies this same order-choice model

## Escalate to Raw When
FPR proves the full equilibrium depth schedule with *informed* market orders (§6.2.3) and derives closed-form spread/volatility relations (§6.4.2-§6.4.3, building on Foucault 1999) that this card only sketches. Re-read pp.197-215 for the exact zero-profit depth solution and the proof method behind the spread-volatility comparative statics; re-read Box 6.3 (pp.225) for the Copeland-Galai option analogy and the monitoring/co-location refinements.
