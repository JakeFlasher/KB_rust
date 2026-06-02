---
schema_version: "cacg.v0"
id: "ec-perfect-competition-equilibrium"
title: "Perfect Competition Equilibrium"
reading_id: "02_economics"
summary: "Walrasian competitive equilibrium is a price vector p* and allocation (x*, y*) with profit maximization, utility maximization, and market clearing; under local nonsatiation the First Fundamental Welfare Theorem makes every equilibrium Pareto efficient (Adam Smith's invisible hand), and convexity assumptions give the Second Fundamental Welfare Theorem."
tags: ["economics", "perfect-competition"]
citations:
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p333:0554"
    chunk_hash: "57e398b795e3c71c72320bbf456a760fad88498df34940cde6eb42e7ab86cd5b"
    page_range: [333, 333]
    quote: "In a competitive economy, a market exists for each of the L goods, and all consumers and producers act as price takers."
    edge_type: "defines"
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p346:0577"
    chunk_hash: "4527c0be8e6118ba3bb833f31321067a711ec98fd959cdb28e13c45570fb6fbc"
    page_range: [346, 346]
    quote: "The first fundamental welfare theorem est.iblishes conditions under which market equilibria are necessarily Pareto optimal."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p642:0884"
    chunk_hash: "73d9011abe75902090bd9688c017853375fabd1440c039dbc5d3005a43637070"
    page_range: [642, 642]
    quote: "We start with the most competitive environment, perfect competition. Unlike some economic concepts, perfect competition is not merely an ideal based on assump"
    edge_type: "supports"
card_hash: "cf9fa16ab93b47b7c52a8e684f3cabbf4bc75dc84ecdca8f28d19dc8e8d958ad"
---
# Perfect Competition Equilibrium

## Intuition

Under perfect competition many small price-taking firms each maximize profit independently. Industry supply is the horizontal sum of firm supplies; market demand aggregates over consumers. The competitive equilibrium is the price-quantity pair `(p*, Q*)` where industry supply equals market demand. In the **long run**, free entry and exit drive economic profit to zero — entry pushes the price down until incumbents earn only the opportunity cost of capital; exit pushes the price up until survivors break even. The long-run equilibrium price equals the minimum of the firm's long-run average cost. **Source:** Mas-Colell et al. (1995) Ch.10 pp.311-330.

```
   P
   ^
   |                                  S_short-run (steep, capacity-bound)
   |                                /
   |                               /
   |                              /
   |                             *  <- short-run equilibrium (P_SR, Q_SR)
   |                            /
   |                  ─────────/────────────  long-run equilibrium price
   |                         */    P_LR = min(LRAC)
   |                        / \
   |                       /   \   S_long-run (flat at P_LR)
   |                      /     \
   |                     /       \
   |                    /         D
   +-------------------+-----------+--------> Q
                      Q_SR       Q_LR
   S_SR > S_LR slope: short-run capacity locks the supply curve;
   long-run free entry/exit flattens it at min(LRAC).
```

The first Fundamental Welfare Theorem says this competitive equilibrium is Pareto efficient: there is no way to make any participant better off without making another worse off. The second Fundamental Welfare Theorem reverses the direction: any Pareto-efficient allocation can be supported as a competitive equilibrium given appropriate lump-sum redistribution. The two theorems are the formal expression of the "invisible hand" — perfect competition reaches the welfare frontier without central planning. **Source:** Mas-Colell et al. (1995) Ch.10 pp.328-345.

## Definition

A **competitive equilibrium** in a partial-equilibrium market consists of a price `p*` and a quantity `Q*` such that: **Source:** Mas-Colell et al. (1995) pp.311-349.

```
Market demand:     Q^D(p*) = Q*
Industry supply:   Q^S(p*) = Σ_j y_j(p*, w, r) = Q*
                   (sum of firm supplies at market price p*)
```

where each firm `j` solves its profit-maximization problem given input prices `(w, r)` and the market price `p*`. **Source:** Mas-Colell et al. (1995) Ch.10 pp.311-318.

**Short-run vs long-run**: in the short run, the number of firms `J` is fixed and capacity is bounded; the industry supply is the sum `Σ_j y_j(p)` over the fixed set. In the long run, free entry / exit makes `J` variable; firms enter until the marginal entrant earns zero economic profit. The long-run industry supply is flat at the price equal to `min(LRAC)` (long-run average cost minimum) if firms are identical. **Source:** Mas-Colell et al. (1995) Ch.10 pp.318-325.

**Pareto efficiency**: an allocation `(x_1, ..., x_I, y_1, ..., y_J)` is Pareto efficient if there is no feasible alternative `(x_1', ..., y_J')` such that `u_i(x_i') ≥ u_i(x_i)` for all consumers `i` and `u_i(x_i') > u_i(x_i)` for at least one consumer. **Source:** Mas-Colell et al. (1995) pp.311-349.

**First Fundamental Welfare Theorem (FFWT)**: under local nonsatiation of consumer preferences, every competitive equilibrium allocation is Pareto efficient. **Source:** Mas-Colell et al. (1995) Ch.10 pp.328-340.

**Second Fundamental Welfare Theorem (SFWT)**: under additional convexity assumptions (convex preferences, convex production sets), every Pareto-efficient allocation can be supported as a competitive equilibrium given appropriate lump-sum redistribution of initial wealth. **Source:** Mas-Colell et al. (1995) Ch.10 pp.340-349.

## Mathematical Reasoning

The competitive-equilibrium price `p*` jointly determines firm output, input demands, and consumer demand. Each firm chooses `(K, L)` to maximize `π = p* · f(K, L) − wL − rK`, giving the supply `y_j(p*)`. Each consumer chooses `x_i` to maximize `u_i(x_i)` subject to `p* · x_i ≤ w_i` (with `w_i` the consumer's wealth), giving the demand `x_i^D(p*)`. Market clearing requires `Σ_i x_i^D(p*) = Σ_j y_j(p*)`. The existence of `p*` solving this system requires standard regularity conditions (continuity of supply and demand, monotonicity, etc.); MWG Ch.10 §10.C develops the partial-equilibrium existence result. **Source:** Mas-Colell et al. (1995) Ch.10 pp.316-328.

**FFWT proof sketch**: suppose the competitive equilibrium allocation is Pareto-dominated by some feasible allocation `(x_1', ..., y_J')`. Then by local nonsatiation, for any consumer `i` whose utility strictly improves, we have `p* · x_i' > p* · x_i = w_i` (the new bundle is unaffordable at equilibrium prices for that consumer). Summing the inequality across all consumers gives `Σ p* · x_i' > Σ w_i = Σ p* · y_j`. But by feasibility `Σ x_i' = Σ y_j'`, and each `y_j'` cannot increase any firm's profit (firms already maximized at `p*`), giving `Σ p* · y_j' ≤ Σ p* · y_j`. Combining contradicts the strict inequality. **Source:** Mas-Colell et al. (1995) Ch.10 pp.328-340.

**SFWT proof sketch**: pick any Pareto-efficient allocation `(x_i^*, y_j^*)`. Under convex preferences and convex production sets, the upper-contour sets `{ x : u_i(x) ≥ u_i(x_i^*) }` and the production sets `Y_j` admit a separating hyperplane through the equilibrium-aggregates `Σ x_i^* = Σ y_j^*`. The normal to the hyperplane is the equilibrium price vector `p*`. The lump-sum transfers `T_i = p* · (x_i^* − ω_i)` (with `ω_i` the consumer's initial endowment) shift wealth so that each consumer can afford exactly her Pareto-efficient bundle at the equilibrium prices. **Source:** Mas-Colell et al. (1995) Ch.10 pp.340-349.

The two theorems make assumptions explicit: **Source:** Mas-Colell et al. (1995) pp.311-349.
- **Local nonsatiation** is necessary for FFWT (otherwise consumers might have unspent budget and the welfare result fails). **Source:** Mas-Colell et al. (1995) pp.311-349.
- **Convexity** (of preferences and production sets) is necessary for SFWT (otherwise the separating hyperplane may not exist). **Source:** Mas-Colell et al. (1995) pp.311-349.
- **No externalities / public goods** is implicit in the partial-equilibrium framing (the sibling `ec-externalities-and-public-goods` card covers when these assumptions fail). **Source:** Mas-Colell et al. (1995) pp.311-349.

When the assumptions fail, the welfare theorems fail: externalities create market failures requiring Pigovian intervention; public goods underprovision requires alternative provision mechanisms; non-convexities (IRS, fixed costs, lumpiness) allow natural-monopoly tendencies that perfect-competition framing cannot reach. These failure modes are the bridge to the sibling cards on monopoly, externalities, and welfare theorems. **Source:** Mas-Colell et al. (1995) Ch.10 pp.328-349.

## See Also

- [`ec-firm-profit-maximization`](./ec-firm-profit-maximization.md) — the firm-side decision that this card aggregates across firms
- [`ec-consumer-utility-and-demand`](./ec-consumer-utility-and-demand.md) — the consumer-side decision that this card aggregates across consumers
- [`ec-monopoly-pricing`](./ec-monopoly-pricing.md) — what changes when there is only one firm (downward-sloping demand curve)
- [`ec-externalities-and-public-goods`](./ec-externalities-and-public-goods.md) — when FFWT fails because of unpriced spillovers
- [`ec-welfare-theorems`](./ec-welfare-theorems.md) — general-equilibrium statement of FFWT / SFWT beyond partial equilibrium

## Escalate to Raw When

The full existence proof for partial competitive equilibrium uses the Brouwer fixed-point theorem on the excess-demand function — re-open MWG Ch.10 pp.316-328 if a question requires the existence argument rather than the equilibrium characterization. The general-equilibrium proofs of FFWT and SFWT (without the partial-equilibrium restriction) are in MWG Ch.16 pp.545-577 and are summarized in the sibling `ec-welfare-theorems` card. The breakdown of welfare theorems under externalities, public goods, asymmetric information, and increasing returns is treated in MWG Ch.11 (externalities and public goods) and Ch.12 (market power); the corresponding 02 cards cover these failure modes. **Source:** Mas-Colell et al. (1995) pp.311-349.
