---
schema_version: "cacg.v0"
id: "ec-monopoly-pricing"
title: "Monopoly Pricing"
reading_id: "02_economics"
summary: "A monopolist facing market demand sets quantity where MR = MC and price above MC; the markup (Lerner index L = (P-MC)/P = 1/|epsilon|) reflects the inframarginal effect of selling additional units on a downward-sloping curve, and the welfare cost is the deadweight-loss triangle between monopoly output Q* and the competitive output Q_c where P = MC."
tags: ["economics", "monopoly-pricing"]
citations:
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p404:0689"
    chunk_hash: "1a84b37a22223d3e104881bd15ec2c99a23598992a07e5730182b1d2932610e4"
    page_range: [404, 404]
    quote: "marginal revenue must equal marginal cost at the monopolist's optimal output level"
    edge_type: "defines"
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p404:0689"
    chunk_hash: "1a84b37a22223d3e104881bd15ec2c99a23598992a07e5730182b1d2932610e4"
    page_range: [404, 404]
    quote: "the price under monopoly exceeds marginal cost. Correspondingly, the monopolist's optimal output q m must be below the socially optimal (competitive) output level"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p643:0886"
    chunk_hash: "5ba394d114d2eee6e7a51ad7f0c97b96f349c4f8f3e8c25ffb0bc95bb83aaf6b"
    page_range: [643, 644]
    quote: "There is a single seller, which, if allowed to operate without constraint, exercises considerable power over pricing and output decisions."
    edge_type: "supports"
card_hash: "18d590c6b6e5a164b0358ddc9f6be5de0b7497bc7288fae5abadef145bc4aae8"
---
# Monopoly Pricing

## Intuition

A monopolist faces the entire market demand curve `P(Q)`, not a fixed price. Selling one more unit requires lowering the price on **all** units sold (the inframarginal effect), so marginal revenue is below price: `MR(Q) = P(Q) + Q · P'(Q) < P(Q)`. Profit-maximizing output equates marginal revenue to marginal cost: `MR(Q*) = MC(Q*)`. The monopoly price `P(Q*)` sits above marginal cost; the gap between them is the **markup**. The Lerner index `L = (P − MC)/P` is the standard markup measure and equals `1/|ε|` (the reciprocal of the absolute price elasticity of demand at the equilibrium). High elasticity → small markup; low elasticity → large markup. **Source:** Mas-Colell et al. (1995) Ch.12 pp.383-410.

```
   P, MR, MC
   ^
   |                                      D = P(Q)
   |     .                                /  (demand)
   |       .       deadweight            /
   |         .     loss triangle       /
   |   P*  ───*    [P_c − MC at Q* ──*  ←─── P* (monopoly price)
   |          |\                    /|
   |          | \                  / |
   |          |  *  ←──── (Q*,MC*) /  |
   |   MC ────│───*                /   │──────── MC (marginal cost)
   |   P_c    │                  /    │
   |   ───────│─────*  ←─── (Q_c,MC) ─│──── P_c = MC: perfect competition
   |          │                       │
   |          │            MR(Q) below D, twice as steep for linear D
   |          │              \        │
   |          │               \       │
   +----------+----------------\------+--------> Q
                              Q*    Q_c
   P* > MC at Q*; DWL = (P* − MC) · (Q_c − Q*) / 2 (linear case)
```

The deadweight loss vs perfect competition (where price would equal marginal cost) is the welfare cost of monopoly: consumers who would buy at `P = MC` no longer buy at `P = P*`, and the surplus on those unsold units evaporates. The monopolist captures part of consumer surplus (the rectangle of inframarginal markup) and creates the deadweight-loss triangle. **Source:** Mas-Colell et al. (1995) Ch.12 pp.395-420.

## Definition

A **monopolist** is the sole supplier of a market good with no close substitutes; she faces the inverse demand `P(Q)` and maximizes profit: **Source:** Mas-Colell et al. (1995) pp.383-410.

```
max_{Q ≥ 0}  π(Q) = P(Q) · Q  −  c(Q)
```

where `c(Q)` is total cost (production cost as a function of output). **Source:** Mas-Colell et al. (1995) pp.383-410.

**Marginal revenue** is the derivative of revenue with respect to output: **Source:** Mas-Colell et al. (1995) pp.383-410.

```
MR(Q) = d[P(Q) · Q] / dQ = P(Q) + Q · P'(Q)
```

The **first-order condition** for profit maximization (interior) equates marginal revenue and marginal cost: **Source:** Mas-Colell et al. (1995) pp.383-410.

```
MR(Q*) = MC(Q*)        (FOC of monopoly profit maximization)
```

The **Lerner index** (markup as a fraction of price) equals the reciprocal of the absolute elasticity of demand at the equilibrium: **Source:** Mas-Colell et al. (1995) pp.383-410.

```
L = (P − MC) / P = 1 / ε          where ε = −(P/Q) · dQ/dP > 0  (positive)
```

For linear demand `P(Q) = a − bQ`, marginal revenue is `MR(Q) = a − 2bQ` — the same intercept `a` and twice the slope. For constant-elasticity demand `Q(P) = A · P^(-ε)` with elasticity `ε > 1`, the Lerner index is constant and equal to `1/ε`. **Source:** Mas-Colell et al. (1995) Ch.12 pp.383-415.

The **deadweight loss** (DWL) is the welfare loss vs the competitive benchmark `P = MC`: **Source:** Mas-Colell et al. (1995) pp.383-410.

```
DWL = ∫_{Q*}^{Q_c} [P(Q) − MC(Q)] dQ
```

where `Q_c` is the competitive output (where `P(Q_c) = MC(Q_c)`). For linear demand and constant `MC`, DWL is the area of the triangle with base `(Q_c − Q*)` and height `(P* − MC)`. **Source:** Mas-Colell et al. (1995) Ch.12 pp.415-435.

## Mathematical Reasoning

The MR-below-P relationship is the structural consequence of selling on a downward-sloping demand curve. With `Q · P'(Q) < 0` (because `P'(Q) < 0`), we have `MR = P + Q · P' < P`. This contrasts with perfect competition where the firm faces a horizontal demand at the market price, so `P'(Q) = 0` and `MR = P`. Equating `MR = MC` therefore gives a quantity below the competitive level (`Q* < Q_c` for any well-behaved demand), and the equilibrium price above marginal cost. **Source:** Mas-Colell et al. (1995) Ch.12 pp.385-395.

The Lerner-index identity `(P − MC)/P = 1/ε` (positive-elasticity convention) follows from rewriting the FOC. Define `ε ≡ −(P/Q) · dQ/dP > 0` (the standard CFA / MWG positive-elasticity convention; demand slopes down so `dQ/dP < 0`, hence `ε > 0`). From `MR = P + Q · P'(Q) = MC` rearrange to `P − MC = −Q · P'(Q)`. Dividing by `P` gives `(P − MC)/P = −(Q/P) · P'(Q)`. Substituting `P'(Q) = dP/dQ = 1 / (dQ/dP)` and rewriting `(Q/P) · (1 / (dQ/dP)) = −1/ε` (because `ε = −(P/Q)(dQ/dP)`, so `(Q/P)(dQ/dP) = −1/ε` after sign-flipping), we get `(P − MC)/P = −(−1/ε) = 1/ε`. The markup is therefore `1/ε > 0` under the positive-elasticity convention. **Source:** Mas-Colell et al. (1995) Ch.12 pp.388-395.

**Welfare comparison**: at the competitive output `Q_c` where `P_c = MC`, total surplus (consumer + producer) is maximized — every unit with `WTP ≥ MC` is produced. At the monopoly output `Q* < Q_c`, units `Q ∈ (Q*, Q_c)` have `P(Q) > MC` but are not produced. The DWL captures this foregone surplus. The monopolist's profit is the rectangle `(P* − MC) · Q*`; consumer surplus shrinks vs the competitive benchmark; total surplus loses the DWL triangle. The aggregate welfare cost of monopoly is the DWL — the rectangle is a transfer from consumers to the monopolist (not a social loss). **Source:** Mas-Colell et al. (1995) Ch.12 pp.395-420.

**Price discrimination** (first / second / third degree) allows the monopolist to extract more surplus from heterogeneous consumers. First-degree (perfect) price discrimination charges each consumer her willingness-to-pay and achieves the competitive output without DWL (but with all surplus captured by the seller); third-degree price discrimination (segment-by-segment markups) does not eliminate DWL but does increase aggregate output relative to uniform pricing. This card stops at the uniform-pricing monopoly model (MWG pp.383-410); the detailed price-discrimination taxonomy lives in MWG pp.420-435 and is referenced in Escalate-to-Raw. **Source:** Mas-Colell et al. (1995) Ch.12 pp.383-410.

## See Also

- [`ec-firm-profit-maximization`](./ec-firm-profit-maximization.md) — what changes when the firm is a price-taker
- [`ec-perfect-competition-equilibrium`](./ec-perfect-competition-equilibrium.md) — the competitive benchmark against which monopoly DWL is measured
- [`ec-externalities-and-public-goods`](./ec-externalities-and-public-goods.md) — different source of market failure (externality vs market power)
- [`ec-game-theory-and-strategic-equilibrium`](./ec-game-theory-and-strategic-equilibrium.md) — oligopoly intermediate cases between monopoly and competition
- [`cc-standard-ii-b-market-manipulation`](../17_cross_cutting/cc-standard-ii-b-market-manipulation.md) — ethical / regulatory treatment of market power (referenced for regulatory framings; this card stops at economic-welfare analysis)

## Escalate to Raw When

The full taxonomy of price discrimination (first / second / third degree, two-part tariffs, bundling, intertemporal discrimination) is in MWG Ch.12 pp.420-435. The natural-monopoly regulation framework (Ramsey pricing, average-cost pricing, two-part tariff regulation) is in MWG Ch.12 pp.405-420 and out of scope here. For oligopoly equilibrium concepts (Cournot, Bertrand, Stackelberg) see MWG Ch.12 pp.387-415; covered at high level in the Batch 3 game-theory card. **Source:** Mas-Colell et al. (1995) pp.383-410.
