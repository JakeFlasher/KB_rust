---
schema_version: "cacg.v0"
id: "ec-welfare-theorems"
title: "Welfare Theorems"
reading_id: "02_economics"
summary: "The First Fundamental Welfare Theorem: every Walrasian (price-taking) equilibrium under local nonsatiation is Pareto optimal — the formal expression of Adam Smith's invisible hand. The Second FWT under convexity says any Pareto-optimal allocation can be supported as a price equilibrium with appropriate lump-sum transfers."
tags: ["economics", "welfare-theorems"]
citations:
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p567:0990"
    chunk_hash: "6ede0593d63608b795d789be360c4fef1adbb95af4a2cd878e0bc0e68a541a77"
    page_range: [567, 568]
    quote: "The first fundamental theorem of welfare economics states conditions under which any price equilibrium with transfers, and in particular any Walrasian equilibrium, is a Pareto optimum."
    edge_type: "defines"
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p564:0984"
    chunk_hash: "637ec76daef3c8e3125557cc48943c872b4e254f0da3ae0c9f47c5c58f2cf018"
    page_range: [564, 565]
    quote: "This result is known as the second fundamental theorem of welfare economics."
    edge_type: "defines"
  - source_id: "econ_mascolell_general_equilibrium_game_theory"
    chunk_id: "econ_mascolell_general_equilibrium_game_theory:p087:0097"
    chunk_hash: "c740ed9904a5db1c827042490948cdf7526afd8f21291d7d809fe120d66f3d76"
    page_range: [87, 88]
    quote: "A state (x,m) is Pareto optimal (P.o.) if it is feasible and if there is no feasible state"
    edge_type: "supports"
card_hash: "60fe78eae451a2a0af658ec90790a2c747091c776cd5ecebc31dfaf2518b5db0"
---
# Welfare Theorems

## Intuition

The two Fundamental Welfare Theorems are the formal statement of Adam Smith's invisible hand. The **First Theorem (FFWT)** says any competitive equilibrium is Pareto efficient — there is no feasible reallocation that makes everyone weakly better off and at least one consumer strictly better off. The **Second Theorem (SFWT)** is the converse — any Pareto-efficient allocation can be supported as a competitive equilibrium given an appropriate redistribution of initial endowments. Together they say: with the right initial wealth distribution, perfectly competitive markets reach the welfare frontier; the planner can pick any point on the frontier she likes by redistributing endowments, and markets do the rest. **Source:** Mas-Colell et al. (1995) Ch.16 pp.545-577.

```
   Edgeworth box (2 consumers, 2 goods): contract curve = Pareto-efficient set

       good 2
   B  ↗
   |
   |  consumer A's IC
   |       .
   |        .
   |         .             contract curve
   |          .              (Pareto-efficient
   |   *       .             allocations)
   |     \      .
   |      \     .              consumer B's IC
   |       *     .              .
   |       |      .             .
   |       |       .            .
   |       |        *           .
   |       |        |    .      .
   |       |        |     .     .
   |       |        |      .    *  ← endowment ω
   |       |        |       .  .
   A───────+────────+────────.───────────→ good 1
                                   B'

   FFWT: competitive equilibrium at any endowment lies on the contract curve
   SFWT: any point on the contract curve can be reached by redistributing ω
```

The theorems pin down what assumptions perfect competition needs to deliver efficiency. FFWT needs **local nonsatiation** of preferences (consumers always weakly prefer slightly more); SFWT needs **convexity** of preferences and production sets (convex upper-contour sets and convex `Y`). When the assumptions fail — increasing returns, externalities, public goods, asymmetric information — the welfare theorems fail and market intervention may be needed. **Source:** Mas-Colell et al. (1995) Ch.16 pp.553-575.

## Definition

A **competitive (Walrasian) equilibrium** for a private-ownership economy with `I` consumers, `J` firms, and `L` goods is a price vector `p* ∈ R^L_+` and an allocation `(x_1^*, ..., x_I^*, y_1^*, ..., y_J^*)` such that: **Source:** Mas-Colell et al. (1995) pp.545-577.

```
1. Profit maximization:   y_j^* ∈ argmax  p* · y_j   for each firm j
                                  s.t.  y_j ∈ Y_j

2. Utility maximization:  x_i^* ∈ argmax  u_i(x_i)
                                  s.t.  p* · x_i ≤ p* · ω_i + Σ_j θ_{ij} π_j(p*)
                          (budget = endowment value + firm-profit shares)

3. Market clearing:       Σ_i x_i^* = Σ_i ω_i + Σ_j y_j^*
                          (consumption = endowment + production, summed)
```

An **allocation** `(x_1, ..., x_I, y_1, ..., y_J)` is **Pareto efficient** (synonyms: Pareto optimal) if it is feasible and there is no feasible alternative `(x_1', ..., y_J')` such that `u_i(x_i') ≥ u_i(x_i)` for all `i` and `u_i(x_i') > u_i(x_i)` for some `i`. The set of all Pareto-efficient feasible allocations is the **contract curve** (in the Edgeworth-box visualization) or the **utility-possibility frontier** (in utility space). **Source:** Mas-Colell et al. (1995) Ch.16 pp.546-555.

**First Fundamental Welfare Theorem (FFWT)** (Proposition 16.C.1 in MWG): if `(p*, x^*, y^*)` is a competitive equilibrium and all consumers have locally nonsatiated preferences, then the equilibrium allocation `(x^*, y^*)` is Pareto efficient. **Source:** Mas-Colell et al. (1995) pp.545-577.

**Second Fundamental Welfare Theorem (SFWT)** (Proposition 16.D.1 in MWG): assume preferences are convex and continuous, production sets are convex and closed, and there is a Pareto-efficient allocation `(x^*, y^*)`. Then there is a price vector `p* ≠ 0` such that `(p*, x^*, y^*)` is a quasi-equilibrium with appropriate lump-sum wealth transfers `T_i = p* · (x_i^* − ω_i) − Σ_j θ_{ij} π_j(p*)`. Under additional conditions (cheaper-point condition for each consumer; positive equilibrium wealth), the quasi-equilibrium is a full competitive equilibrium. **Source:** Mas-Colell et al. (1995) Ch.16 pp.558-577.

## Mathematical Reasoning

**FFWT proof (sketch)**: suppose for contradiction that some feasible allocation `(x', y')` Pareto-dominates the competitive equilibrium `(x^*, y^*)`. For any consumer `i` with `u_i(x_i') > u_i(x_i^*)`, the bundle `x_i'` must cost strictly more than the equilibrium budget `w_i^*` — otherwise the consumer would have chosen `x_i'` over `x_i^*`. For any consumer `i` with `u_i(x_i') = u_i(x_i^*)`, local nonsatiation implies a small perturbation `x_i' + ε ξ` is strictly preferred for some direction `ξ` and `ε > 0`; this perturbation must also cost more than `w_i^*`. Summing across all consumers: `Σ p* · x_i' > Σ w_i^* = Σ p* · ω_i + Σ p* · y_j^*`. By firms' profit-maximization, `Σ p* · y_j' ≤ Σ p* · y_j^*` (firms wouldn't switch to `y_j'` if it offered higher profit). By feasibility, `Σ x_i' = Σ ω_i + Σ y_j'`. Combining gives `Σ p* · x_i' = Σ p* · ω_i + Σ p* · y_j' ≤ Σ p* · ω_i + Σ p* · y_j^*`, contradicting the strict inequality. **Source:** Mas-Colell et al. (1995) Ch.16 pp.555-565.

**SFWT proof (sketch)**: given a Pareto-efficient allocation `(x^*, y^*)`, define for each consumer the upper-contour set `V_i = { x : u_i(x) ≥ u_i(x_i^*) }`. Under convex preferences, each `V_i` is convex. Define the aggregate consumption upper-contour set `V = Σ V_i = { Σ x_i : x_i ∈ V_i }`. Define the aggregate production set `Y = Σ Y_j`. Both `V` and `Y + Σ ω_i` are convex sets. Pareto efficiency of `(x^*, y^*)` implies that `V` and `Y + Σ ω_i` intersect only at the equilibrium aggregate `Σ x_i^* = Σ ω_i + Σ y_j^*`. By the **separating hyperplane theorem**, there exists a price vector `p* ≠ 0` such that `p* · v ≥ p* · (Σ ω_i + y)` for all `v ∈ V` and `y ∈ Y`. This `p*` supports the Pareto-efficient allocation as a quasi-equilibrium; under cheaper-point conditions it becomes a full competitive equilibrium. **Source:** Mas-Colell et al. (1995) Ch.16 pp.565-577.

**Assumption failure → welfare-theorem failure**: **Source:** Mas-Colell et al. (1995) pp.545-577.
- **FFWT fails** under externalities (consumers' utility is not separable across market-mediated goods; covered in `ec-externalities-and-public-goods`); public goods (non-rivalrous goods are not allocated by price; same card); asymmetric information (adverse selection / moral hazard; out of v10 scope). **Source:** Mas-Colell et al. (1995) pp.545-577.
- **SFWT fails** under non-convex preferences (e.g., addictive goods with increasing marginal utility); non-convex production sets (increasing returns to scale, fixed costs); informational frictions that prevent the planner from observing consumer types for the lump-sum transfer design. **Source:** Mas-Colell et al. (1995) pp.545-577.

The welfare theorems' role in microeconomics is not as descriptive claims about real markets — they are **conditional theorems** that specify when markets reach efficiency and when intervention is justified. Identifying the failure mode (which assumption breaks) tells the policymaker which instrument is appropriate (Pigovian tax for externalities; public provision for public goods; regulation for monopoly; mechanism design for information; redistribution for distributional concerns separate from efficiency). **Source:** Mas-Colell et al. (1995) Ch.16 pp.565-577.

## See Also

- [`ec-perfect-competition-equilibrium`](./ec-perfect-competition-equilibrium.md) — partial-equilibrium statement of FFWT and SFWT
- [`ec-externalities-and-public-goods`](./ec-externalities-and-public-goods.md) — primary v10 card for FFWT failure modes
- [`ec-monopoly-pricing`](./ec-monopoly-pricing.md) — failure of perfect competition assumption
- [`ec-aggregate-demand-representative-consumer`](./ec-aggregate-demand-representative-consumer.md) — when does individual demand aggregate cleanly

## Escalate to Raw When

The general-equilibrium existence proof (Brouwer fixed-point on excess demand, Kakutani for non-smooth preferences) is in MWG Ch.17 pp.578-651. The core convergence theorems (Debreu-Scarf; competitive equilibrium as the only outcome of pure exchange when bargaining is unrestricted in large economies) are in MWG Ch.18. The cheaper-point condition and the regularity conditions that promote SFWT's quasi-equilibrium to a full competitive equilibrium are in MWG Ch.16 pp.570-577. For intertemporal welfare theorems (general equilibrium with dated commodities), see MWG Ch.20. **Source:** Mas-Colell et al. (1995) pp.545-577.
