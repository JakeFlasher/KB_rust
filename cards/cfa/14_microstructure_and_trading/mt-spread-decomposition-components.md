---
schema_version: "cacg.v0"
id: "mt-spread-decomposition-components"
title: "Decomposing the Spread: Adverse-Selection vs Order-Processing (and Inventory) Components"
reading_id: "14_microstructure_and_trading"
summary: "The quoted half-spread sums an order-processing cost, an adverse-selection cost, and an inventory cost; each leaves a distinct price-dynamics signature (permanent vs transient impact, fast vs slow reversal) that lets the components be separated and matched to different policy remedies."
tags: ["microstructure", "bid-ask-spread", "adverse-selection", "order-processing-cost", "inventory-risk"]
citations:
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p095:0144"
    chunk_hash: "8841f4529e13c357250e4f64af97c4ae8e66672cdd8266cd1e226d7c87321f33"
    page_range: [95, 95]
    quote: "now has two different components: order-processing cost (2γ) and adverse-selection cost"
    edge_type: "defines"
card_hash: "c27ea3353dda5c3513da84c6270100b0c99733d1f2434a04cce4e7f3e1cb9870"
---
# Decomposing the Spread: Adverse-Selection vs Order-Processing (and Inventory) Components

## Intuition

The bid-ask spread is not a single thing. Foucault, Pagano, and Röell decompose the compensation a liquidity supplier demands into three economically distinct costs: (i) the expected loss from trading against better-informed counterparties (adverse-selection cost), (ii) the real operational cost of executing and clearing a transaction (order-processing cost), and (iii) the cost of carrying risky inventory because buy and sell orders do not arrive simultaneously (inventory holding cost). Imperfect competition can add a fourth wedge: a dealer rent (markup) on top of true costs. Each cost pushes the spread wider, but for different reasons, so the right policy response differs by source.

Why bother separating them? Because the components carry *different implications for short-term price movements after a trade*, and those signatures are observable. A buy order always pushes the price up on impact, but what happens next reveals the cause. If the spread is purely informational (adverse selection), the impact is **permanent** — dealers revise their value estimate and the price stays revised. If the spread compensates for processing or inventory cost, the impact is **transient** — it reverts. And the *speed* of reversion separates processing from inventory: processing reversal is immediate, inventory reversal is gradual.

```
   price
    |            permanent (adverse selection)
    |        o---------------------------------  long-run level shifts up
    |       /
    |      o    fast revert (order processing)
    |     /|\
    |    / | o....                                price blips up then
 P0 |---o  |     `....o.....o.....                snaps back / drifts back
    |      |  slow revert (inventory)
    +------+--------------------------------->  time
        buy order at t
```

This taxonomy is the conceptual scaffold under the empirical decomposition estimators (Roll, Glosten-Harris, Huang-Stoll): each isolates the transient "cost" piece from the permanent "information" piece using the autocorrelation that transient costs imprint on trade-to-trade returns.

**Source:** Foucault, Pagano & Röell (2013) ch.3 §3.4 pp.95-96.

## Definition

Let μ_{t-1} be the dealers' value estimate given all public information just before trade t, and let d_t ∈ {−1, +1} signal a seller- or buyer-initiated order. Write s^a_t and s^b_t for the adverse-selection components on the ask and bid sides, and γ for the order-processing cost per share. The quote-setting break-even conditions are

  a_t = μ_{t-1} + γ + s^a_t   (ask)
  b_t = μ_{t-1} − γ − s^b_t   (bid)

so the quoted spread is

  S_t ≡ a_t − b_t = 2γ + s^a_t + s^b_t.

Here 2γ is the **order-processing component** (passed straight through to liquidity demanders) and s^a_t + s^b_t is the **adverse-selection component**. The text notes the spread "now has two different components: order-processing cost (2γ) and adverse-selection cost (s^a_t + s^b_t)." The inventory component enters in §3.5 as a separate additive risk premium tied to the dealer's accumulated position and risk aversion (ρσ_ε type term), and Box 3.1 shows a dealer rent γ_r can hide inside the measured γ = γ_c + γ_r.

**Source:** Foucault, Pagano & Röell (2013) ch.3 §3.4.1 pp.95-96; Box 3.1 p.96.

## Mathematical Reasoning

Transaction prices satisfy p_t = μ_{t-1} + (s(d_t) + γ) d_t, and because the efficient price updates as μ_t = μ_{t-1} + s(d_t) d_t, this collapses to

  p_t = μ_t + γ d_t,

so the transaction price deviates from fundamental value μ_t by exactly the processing markup, |p_t − μ_t| = γ. This deviation is *transient*: it reflects no revision of beliefs, so it is later corrected by an opposite move (a reversal).

Define the short-run impact of a buy as the move from the prior fundamental: ST impact ≡ p_t − μ_{t-1} = a_t − μ_{t-1} = s^a_t + γ > 0. For the long-run impact, take expectations at a horizon t+T far enough that the day's value uncertainty has resolved, so the current trade direction no longer predicts future direction, E(d_{t+T} | Ω_{t-1}, d_t = 1) = 0. Then

  E(p_{t+T} | Ω_{t-1}, d_t = 1) = μ_{t-1} + s^a_t  ⟹  LT impact = s^a_t.

Subtracting gives the clean identity

  ST impact − LT impact = γ.

So the permanent piece of a trade's price impact equals the adverse-selection component, and the gap between immediate and eventual impact equals the processing cost. When γ = 0 (pure information), ST = LT and there is no reversal; when γ > 0, the price overshoots on impact and reverts, imprinting **negative serial correlation** on trade-to-trade returns. Roll's spread estimator is exactly the special case where the whole spread is processing cost (so the entire impact reverts).

Inventory cost generates reversals too — aggregate dealer inventory mean-reverts to zero, dragging price back — but the distinguishing comparative static is **speed**: processing reversal is immediate (one tick), inventory reversal is gradual, decaying as inventory unwinds and depending on dealer risk aversion. This three-way speed/permanence ordering (permanent | fast-revert | slow-revert) is what makes the components separately identifiable from order-flow and price data.

**Source:** Foucault, Pagano & Röell (2013) ch.3 §3.4.2 pp.96-97; §3.5 p.97.

## Boundary Notes

- The framework normalizes order size to one share (q_t = d_t ∈ {−1,+1}); size-dependent and nonlinear impact require richer models, and the *speed* of inventory reversion here is fixed by assumption rather than endogenous.
- **Identification caveat (Box 3.1):** price-reversal dynamics cannot separate true processing cost γ_c from dealer rent γ_r; the measured "order-processing" component is γ = γ_c + γ_r, so it may overstate operational cost wherever dealer competition is imperfect (e.g., the 1990s Nasdaq collusion episode).
- The permanent-vs-transient dichotomy assumes the long horizon t+T is past value resolution so that E(d_{t+T} | d_t) = 0; with persistent order-flow autocorrelation (order splitting), the clean ST − LT = γ identity blurs and the informational component itself can show extended price drift.
- Policy mapping is the payoff of the decomposition: order-processing-dominated illiquidity calls for trading-system / competition reforms, whereas adverse-selection-dominated illiquidity calls for disclosure and trader-parity measures — "technological upgrades and rules encouraging competition among trading platforms can reduce processing costs, while action against insider trading may mitigate adverse selection."

**Source:** Foucault, Pagano & Röell (2013) ch.3 §3.4 pp.95-97; Box 3.1 p.96.

## See Also

- [`mt-glosten-milgrom-adverse-selection`](./mt-glosten-milgrom-adverse-selection.md) -- supplies the s^a_t + s^b_t adverse-selection term as the permanent, belief-revising component.
- [`mt-roll-implicit-spread-estimator`](./mt-roll-implicit-spread-estimator.md) -- the pure-order-processing special case whose return autocovariance backs out the spread.
- [`mt-generalized-roll-spread-decomposition`](./mt-generalized-roll-spread-decomposition.md) -- generalizes Roll to split permanent (information) from transient (processing) impact.
- [`mt-spread-equilibrium-timing-option`](./mt-spread-equilibrium-timing-option.md) -- relates spread to liquidity-supply timing and rents.

## Escalate to Raw When

The card sketches the ST − LT = γ derivation and the equilibrium quote conditions but states the inventory premium only qualitatively. Re-read FPR (2013) §3.4.2 for the full long-run-impact expectation algebra (eqs. 3.32–3.37), §3.5 for the explicit inventory/risk-aversion quote derivation (the ρσ_ε z_t term and its gradual reversion), Box 3.1 for the rent-vs-cost identification proof, and Chapter 5 for the empirical estimators (Glosten-Harris, Huang-Stoll) that operationalize the permanent/transient split on real trade data.
