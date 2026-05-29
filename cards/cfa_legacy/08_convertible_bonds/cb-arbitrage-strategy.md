---
schema_version: "cacg.v0"
id: "cb-arbitrage-strategy"
title: "Convertible Arbitrage Strategy"
reading_id: "08_convertible_bonds"
summary: "A convertible-arbitrage position pairs a long CB with a short of delta-equivalent underlying shares, harvesting positive gamma via dynamic rebalancing; the P&L decomposes into delta, gamma, vega, theta, coupon, dividend, and borrow components with secondary credit/vega hedges optionally overlaid for double-signed-gamma stress regimes."
tags: ["convertible-bonds", "arbitrage-strategy"]
citations:
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p021:0019"
    chunk_hash: "b929674cbb30422015f513c673dbb0e571e39c6b721962595dcff309e5b0ae49"
    page_range: [21, 22]
    quote: "Convertible securities are hybrid issues that have fixed-income and equity characteristics."
    edge_type: "defines"
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p274:0302"
    chunk_hash: "4c2aa81b7e409af37cee030d47636432e579616db2dcc6ffe5450b0a56c4a996"
    page_range: [274, 275]
    quote: "Each position vega in points is multiplied by the position size to come up with the dollar exposure to vega."
    edge_type: "supports"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p128:0165"
    chunk_hash: "e2c05a0651e787c7e62277bd197a690414ecdb8b1e93413e9a02f04da32299f4"
    page_range: [128, 129]
    quote: "The probability of such a conversion will increase when the share price reaches depressed levels, even if the trigger is linked to, for example, a capital ratio."
    edge_type: "supports"
  - source_id: "cb_thorp_kassouf_1967_beat_the_market"
    chunk_id: "cb_thorp_kassouf_1967_beat_the_market:p058:0054"
    chunk_hash: "b9d57e332466c90e4068a509540b591c4a7926c5c83fa31a21d67c31801e4d98"
    page_range: [58, 59]
    quote: "The Molybdenum Story I first purchased Molybdenum common shares and sold short the warrants in October 1961."
    edge_type: "supports"
card_hash: "62f19619b5b94729e9bc7672f76b20055b58147353f2942739e5adf015335079"
---
# Convertible Arbitrage Strategy

## Intuition

A **convertible-arbitrage** position is a long CB hedged with a short of
`Δ_S` shares of the underlying stock. The short neutralizes the first-order
equity exposure; the residual exposure is dominated by the convertible's
**positive gamma**. Because gamma is convex in `S`, share-price oscillation
generates positive P&L when the position is rebalanced ("buy-low, sell-
high" enforced by the hedge ratio adjusting). The strategy was first
formalized by Thorp and Kassouf for warrants in 1967 and adapted to
convertible bonds throughout the 1980s-2000s practitioner literature.
**Source:** Thorp+Kassouf (1967) §3-§5 pp.50-130; Calamos (2003) §1
pp.3-30.

```
share-price oscillation  →  delta-hedged P&L (gamma scalp)

       S(t)                          P&L(t)
        ^                              ^
        |       /\          /\         |     /\        /\
        |      /  \    /\  /  \        |    /  \      /  \
        |     /    \  /  \/    \       |   /    \    /    \
        |    /      \/         |       |  /      \  /      \
        +-----------------> t        +------------\/--------> t
```

## Definition

A canonical convertible-arbitrage trade at time `t` is built around the
**hedge ratio** `Δ_S = Δ × N / 100` (number of underlying shares to short
per face-percentage CB; `N` is shares per face). **Source:** Calamos (2003)
§3 pp.50-75.

- **Long leg**: `M` units (face) of the convertible, financed at the
  arbitrage book's marginal funding rate `r_fund`. **Source:** Calamos
  (2003) §3 pp.50-65.
- **Short leg**: short `M · Δ_S` underlying shares; short proceeds earn
  `r_short` (the rebate); the short pays `b` (borrow fee) per dollar of
  short notional. **Source:** Calamos (2003) §3 pp.65-75.
- **Coupon collection**: long CB receives `c · F` per period. **Source:**
  Calamos (2003) §3 pp.50-65.
- **Dividend payment**: short stock pays away `q_div · S` per period
  (negative carry on the hedge). **Source:** Calamos (2003) §3 pp.65-75.
- **Credit hedge** (optional secondary): single-name CDS or short straight
  bonds of the same issuer, sized to neutralize `B(t)` exposure to spread
  widening. **Source:** DeSpiegeleer et al. (2014) §3.6 pp.95-110.
- **Vega hedge** (optional secondary): listed equity options or variance
  swaps, sized to neutralize `ν` exposure. **Source:** Calamos (2003) §11
  pp.260-285.

The position is **dynamically rebalanced** as `Δ_S` evolves; the
rebalancing trades (forced "sell shares as `S` rises, buy as `S` falls")
mechanically harvest gamma. **Source:** Thorp+Kassouf (1967) §4 pp.80-110.

## Mathematical Reasoning

Decompose the position's instantaneous P&L using a Taylor expansion of
`V(S, σ, t)` and the Itô correction. **Source:** Calamos (2003) §11
pp.260-285; Thorp+Kassouf (1967) §4 pp.80-110.

```
dΠ = dV - Δ · dS - b · M · Δ_S · S · dt
       + c · F · dt - q_div · M · Δ_S · S · dt
```

Substituting the convertible's Itô expansion `dV = (∂V/∂t) dt + Δ · dS +
½ · Γ · (dS)^2 + ν · dσ + ε · dq_div + ψ · db + ρ · dr` and using `dS = μ
S dt + σ S dW` plus the delta-cancellation identity, the first-order
equity term vanishes; the net P&L per unit time satisfies the identity
displayed below. **Source:** Calamos (2003) §11 pp.260-285.

```
dΠ/dt  =  ½ · Γ · σ^2 · S^2          (gamma scalp -- positive)
        + ν · (∂σ/∂t)                (vega P&L from vol changes)
        + Θ                           (theta -- bond accrual minus call decay)
        + c · F                       (coupon)
        - q_div · M · Δ_S · S         (dividend leg)
        - b · M · Δ_S · S             (borrow cost)
        + ρ · (∂r/∂t)                 (rho from rate moves)
```

The gamma term `½ · Γ · σ^2 · S^2` is the realized-vol harvesting term.
**Source:** Calamos (2003) §11 pp.270-285. When realized vol exceeds
implied vol the position earns positive P&L net of theta — the classical
"long realized, short implied" volatility trade. **Source:** Thorp+Kassouf
(1967) §5 pp.110-130.

The **break-even realized volatility** `σ_BE` solves
`½ · Γ · σ_BE^2 · S^2 + Θ + c · F − (q_div + b) · M · Δ_S · S = 0`. If
realized vol over the holding period exceeds `σ_BE` (and the model implied
vol used to compute Greeks does not move adversely against the trader),
the trade earns positive expected P&L. **Source:** Calamos (2003) §11
pp.275-300.

The decomposition fails under **double-signed gamma stress** (see the
[bond-floor card](./cb-bond-floor-investment-value.md#mathematical-reasoning)):
when issuer credit deteriorates as `S` falls, the realized gamma differs
from the constant-credit Greek used to set the hedge. The arbitrageur's
P&L picks up an unhedged credit term unless secondary credit-default-swap
protection is purchased. **Source:** DeSpiegeleer et al. (2014) §3.6
pp.95-110.

Asymptotic strategy regimes (cases below). **Source:** Calamos
(2003) §11 pp.260-300.

- **Balanced regime** (`S ≈ K_c`): peak gamma + vega; the canonical "sweet
  spot" for convertible arbitrage. **Source:** Calamos (2003) §11
  pp.260-300.
- **Equity-like regime** (`S ≫ K_c`): gamma collapses; the position
  degrades into a small residual long-stock position plus residual
  carry — practitioners typically unwind. **Source:** Calamos (2003) §11
  pp.260-300.
- **Distressed regime** (`S ≪ K_c`): bond-floor exposure dominates; the
  short-stock leg shrinks toward zero; the trade becomes a credit play
  unless explicit credit hedges have been booked. **Source:** Calamos
  (2003) §11 pp.290-300.

## See Also

- [`cb-greeks-delta-gamma-vega.md`](cb-greeks-delta-gamma-vega.md) — the Greek primitives the trade hedges
- [`cb-credit-vs-equity-decomposition.md`](cb-credit-vs-equity-decomposition.md) — the regime classifier the trader uses to size hedges
- [`cb-bond-floor-investment-value.md`](cb-bond-floor-investment-value.md) — the credit-stress side of the P&L attribution
- [`cb-payoff-decomposition-bond-plus-call.md`](cb-payoff-decomposition-bond-plus-call.md) — the underlying identity
- [`cb-china-t-plus-zero-arbitrage.md`](cb-china-t-plus-zero-arbitrage.md) — Chinese-market Zhang+Feng (2014) T/T+1 conversion-arbitrage strategy regimes under 融资融券 short-borrow availability, anchored on mean-reverting pricing deviations rather than continuous delta hedging (the SSE/SZSE T+0 rule-layer turnover permits the CB-side same-day entry; the stock-side action is structurally T+1)

## Escalate to Raw When

Open Calamos §1-§5 pp.3-95 directly for the full practitioner playbook:
hedge-ratio choice, financing setup, credit hedging, distressed-CB
unwinds. **Source:** Calamos (2003) §1-§5 pp.3-95.

Open Calamos §11 pp.260-300 for the realized-vs-implied-vol P&L
attribution, including the dollar-gamma scaling that practitioners use
to compare positions across issuers. **Source:** Calamos (2003) §11
pp.260-300.

Open Thorp+Kassouf (1967) §3-§5 pp.50-130 for the original warrant-arb
formulation that the convertible-arb playbook generalizes. **Source:**
Thorp+Kassouf (1967) §3-§5 pp.50-130.
