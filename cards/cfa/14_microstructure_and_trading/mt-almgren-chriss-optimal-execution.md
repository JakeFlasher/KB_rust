---
schema_version: "cacg.v0"
id: "mt-almgren-chriss-optimal-execution"
title: "Almgren-Chriss Optimal Execution: Temporary vs Permanent Impact and the Urgency Trade-off"
reading_id: "14_microstructure_and_trading"
summary: "Liquidating a large position optimally is a control problem balancing impact cost against price risk over a fixed window; a running inventory penalty sets urgency, bending the schedule from a TWAP straight line toward convex front-loading."
tags: ["microstructure", "optimal-execution", "price-impact", "stochastic-control", "almgren-chriss"]
citations:
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p158:0208"
    chunk_hash: "e52880bc1d00f1aa401f27d7fb71514ce138fba75e8bef38c504fbbde7df8009"
    page_range: [158, 159]
    quote: "the shares must be liquidated at a constant rate and this strategy is the same as that of the time weighted average price"
    edge_type: "defines"
card_hash: "7ed137222607d5754effbf8269e39d42dda57e1ab7cd5d98ac7f4181e0f3b00c"
---
# Almgren-Chriss Optimal Execution: Temporary vs Permanent Impact and the Urgency Trade-off

## Intuition
An agent who must sell a large block by a deadline `T` cannot dump it at once: the market lacks the depth to absorb a large order at the best price, so an aggressive order walks down the book and realises poor prices. The remedy is to slice the parent order and spread the child orders through time. But slicing has a competing cost — while the agent waits, the midprice drifts on a random (Brownian) path, so a slow schedule exposes the unsold inventory to price risk. Optimal execution is therefore a trade-off between **impact cost** (pay more by trading fast) and **timing/price risk** (carry more uncertainty by trading slow).

The agent's distaste for carrying open inventory is encoded by an **urgency parameter**: a running penalty on holding a non-zero position. Turn urgency off and the agent only minimises expected impact cost; turn it up and the agent front-loads, selling more early to flatten the book sooner. A separate terminal penalty enforces the deadline by punishing leftover shares at `T`.

```
inventory Q_t
 Q0 |*.
    | '*.            phi = 0  (no urgency)  -> straight line (TWAP)
    |   '*._
    |      `*-._        phi > 0  -> convex, sell more early
    |          `-.__
    |              `--.____
  0 +---------------------*----> time
    0                     T
```

The classic Almgren-Chriss insight is that with linear impact and a quadratic inventory penalty the whole problem collapses to a deterministic trade schedule, indexed by a single urgency knob.
**Source:** Cartea, Jaimungal & Penalva (2015) §5.2.2, §6.3-6.5 pp.101-103, 140-147.

## Definition
Let `Q_t` be remaining inventory, `v_t >= 0` the (controlled) liquidation rate, `S_t` the fundamental midprice and `X_t` accumulated cash. The state dynamics are
`dQ_t = -v_t dt`, `dS_t = -g(v_t) dt + sigma dW_t`, execution price `S_hat_t = S_t - h(v_t)`, `dX_t = v_t S_hat_t dt`.
Here `g(.)` is the **permanent** impact on the fundamental price and `h(.)` is the **temporary** impact on the price actually executed. With a terminal liquidation penalty `alpha` and a running inventory penalty `phi >= 0`, the performance criterion maximised over admissible (F-predictable, non-negative, bounded) strategies is

`H = E[ X_T + Q_T(S_T - alpha Q_T) - phi ∫_0^T (Q_s)^2 ds ]`.

The terminal term `Q_T(S_T - alpha Q_T)` marks any leftover inventory to market and taxes it quadratically; the integral `phi ∫ Q^2` is the running inventory penalty (urgency). The standard tractable case takes linear impact `f(v) = k v` (temporary) and `g(v) = b v` (permanent), `k, b >= 0`.
**Source:** Cartea, Jaimungal & Penalva (2015) eq.(5.3)-(5.4), eq.(6.20), §6.5 pp.101-103, 145-146.

## Mathematical Reasoning
By the Dynamic Programming Principle the value function solves the HJB equation
`0 = (∂_t + ½ sigma^2 ∂_SS)H - phi q^2 + sup_v { (v(S - f(v))∂_x - g(v)∂_S - v∂_q) H }`,
with terminal condition `H(T,x,S,q) = x + S q - alpha q^2`. With linear impact the first-order condition gives the optimal speed in feedback form, `v* = (1/2k)(S ∂_x - b ∂_S - ∂_q)H / ∂_x H`. The ansatz `H = x + S q + h(t,q)` strips out the cash and book-value terms; because nothing depends explicitly on `S` and the terminal data is `S`-free, `∂_S h = 0`, and a separation `h(t,q) = h_2(t) q^2` reduces everything to a scalar Riccati ODE for `h_2(t)` with `h_2(T) = -alpha`.

Two regimes follow:

- **No urgency (`phi = 0`).** The midprice volatility drops out entirely — the Brownian part is a martingale and contributes zero in expectation — so risk plays no role. The ODE `∂_t h_2 + h_2^2 / k = 0` integrates to an inventory profile that decays linearly, and the optimal speed is constant: the agent liquidates at a uniform rate. This is exactly TWAP. (Per the source, "the shares must be liquidated at a constant rate and this strategy is the same as that of the time weighted average price.")

- **With urgency (`phi > 0`).** The Riccati ODE becomes `∂_t x = (1/k)(k phi - x^2)` after the substitution `h_2 = -½b + x`, integrating to a hyperbolic solution governed by the rate `gamma = sqrt(phi/k)`. The optimal inventory and speed become deterministic functions of time. In the strong-deadline limit `alpha -> ∞` (force `Q_T = 0`) they reduce to the clean Almgren-Chriss forms
`Q_t* = sinh(gamma(T-t)) / sinh(gamma T) · Q0`, `v_t* = gamma cosh(gamma(T-t)) / sinh(gamma T) · Q0`,
both independent of the permanent-impact `b`.

Comparative statics on the urgency knob: as `phi` rises, `gamma` rises, the trading curve becomes more convex, and the agent sells more early — front-loading to cut inventory risk. As `phi -> 0`, `gamma -> 0` and the convex `sinh` profile degenerates to the linear TWAP line. The ratio `phi/k` (urgency relative to temporary-impact cost) is the single dimensionless number that sets how aggressively the schedule front-loads.
**Source:** Cartea, Jaimungal & Penalva (2015) eq.(6.6)-(6.12), (6.20)-(6.30) pp.140-147.

## Boundary Notes
The clean closed form rests on strong assumptions: **linear** temporary and permanent impact (`h(v)=kv`, `g(v)=bv`), a **quadratic** inventory penalty, arithmetic (additive Brownian) midprice with no drift, deterministic constant coefficients, and a continuous trading rate posted via market orders. Under these, the schedule is purely deterministic and the asset volatility `sigma` does not affect the *path* (only the risk it penalises through `phi`). Relax any of these — non-linear/concave impact, transient (decaying) impact, stochastic liquidity, drift in `S`, or discrete order placement — and the deterministic-trajectory result breaks; the problem stays a genuine stochastic control problem with feedback that depends on realised price.

Note the modelling stance: the running inventory penalty `phi` is **not** a realised financial cost; it is a risk-aversion / urgency device (the source notes it is equivalent to ambiguity aversion over a stochastic midprice drift). Permanent impact `b` shifts the fundamental price for all participants and, in the `alpha -> ∞` limit, drops out of the optimal trajectory entirely — only temporary impact and urgency shape the schedule there. Contrast with execution that also posts limit orders, which can save the spread but adds fill-probability risk.
**Source:** Cartea, Jaimungal & Penalva (2015) §5.2.3, §6.5 pp.103, 145-147.

## See Also
- [`mt-temporary-permanent-price-impact`](./mt-temporary-permanent-price-impact.md) -- the two impact channels (`h`, `g`) that this control problem trades off against time risk.
- [`mt-stochastic-control-hjb-liquidation`](./mt-stochastic-control-hjb-liquidation.md) -- the HJB/DPP machinery that solves the liquidation problem.
- [`mt-implementation-shortfall`](./mt-implementation-shortfall.md) -- the realised-cost benchmark the optimal schedule is designed to minimise.
- [`mt-vwap-pov-volume-targeting`](./mt-vwap-pov-volume-targeting.md) -- schedule-tracking execution; TWAP is the `phi=0` special case here.

## Escalate to Raw When
The source actually *integrates* the Riccati ODE in closed form and derives the inventory path `Q_t*` and speed `v_t*` step by step (eq. 6.25-6.30); this card only sketches the substitution `h_2 = -½b + x` and quotes the result. Re-read §6.3 (pp.140-141) for the full `phi=0` TWAP derivation, §6.4 for the acquisition/terminal-penalty variant, and §6.5 (pp.145-147) for the permanent-impact + running-penalty solution and the `alpha -> ∞` `sinh`/`cosh` limit, including the numerical convexity figures (Fig. 6.2) that this card describes only qualitatively.
