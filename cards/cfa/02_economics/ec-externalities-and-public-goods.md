---
schema_version: "cacg.v0"
id: "ec-externalities-and-public-goods"
title: "Externalities and Public Goods"
reading_id: "02_economics"
summary: "MWG Ch.11 market-failure classes where FFWT assumptions break: externalities (uncompensated spillover; Pigovian-tax remedy) and public goods (non-rival, non-excludable; Samuelson provision condition Σ MRS = MRT). Coase-theorem private-bargaining alternative under low transaction costs."
tags: ["economics", "externalities-public"]
citations:
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p369:0621"
    chunk_hash: "5aefd962f1afbf61b9213298a058d9a2481f3e3c78ef189b2ac606934186e3ea"
    page_range: [369, 370]
    quote: "The private provision of public goods generates a special type of externality: if one individual provides a unit of a public good, all individuals benefit."
    edge_type: "defines"
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p369:0621"
    chunk_hash: "5aefd962f1afbf61b9213298a058d9a2481f3e3c78ef189b2ac606934186e3ea"
    page_range: [369, 370]
    quote: "In this chapter, we study two types of market failure, known as externalities and public goods."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p640:0881"
    chunk_hash: "9b433183505c6884a02ef3e4325ccf1b38e400f5a5f9b80c7939434c97e02fd7"
    page_range: [640, 641]
    quote: "As different market structures result in different sets of choices facing a firm’s decision makers, an understanding of market structure is a powerful tool in 1 READING 9 © 2019 CFA Institute."
    edge_type: "supports"
card_hash: "cd89a6ff8b8928b7e63692b5c845d3c9de933cfc8f5a7d8719e57253921c0e96"
---
# Externalities and Public Goods

## Intuition

The First Welfare Theorem assumes that all actions affecting any agent's payoff are mediated through market prices. **Externalities** violate this assumption: when one agent's production or consumption directly affects another agent's payoff without a market transaction (the canonical example: a polluting factory imposes health costs on downstream residents), the competitive equilibrium misallocates resources. **Public goods** violate the assumption through non-rivalry (one agent's consumption doesn't diminish another's) and non-excludability (consumers cannot be prevented from using the good); the market underprovides public goods because no individual can capture the full social benefit. **Source:** Mas-Colell et al. (1995) Ch.11 pp.350-382.

```
   private MC vs social MC, with negative externality (e.g., pollution)

   P
   ^
   |                                       social MC = private MC + ext cost
   |                                      /
   |                                     /
   |                                    /
   |                              *    /  ← social optimum: lower Q, higher P
   |                              |   /
   |                              |  /   private MC (without externality)
   |                              | /
   |                  competitive *  ← market equilibrium (higher Q, lower P)
   |                  equilibrium  \
   |                                \
   |                                 D
   |                                  \
   +-----------------+-----------------+--------> Q
                    Q_social         Q_market

   gap = external cost per unit
   Pigovian tax = external marginal cost
   shifts private MC up to social MC; market reaches Q_social
```

The **Pigovian tax** remedy adds a per-unit tax equal to the external marginal cost, shifting the private cost curve up to coincide with the social cost curve; the competitive equilibrium then sits at the social optimum. The **Samuelson condition** for public-goods provision sets aggregate willingness-to-pay equal to marginal cost: `Σ_i MRS_i = MRT`. The **Coase theorem** offers a private-bargaining alternative: with well-defined property rights and zero transaction costs, the affected parties can negotiate to the efficient allocation without government intervention; the initial allocation of rights affects distribution but not efficiency. **Source:** Mas-Colell et al. (1995) Ch.11 pp.355-382.

## Definition

An **externality** exists when one agent's action enters another agent's utility or production function without a market price. **Negative externality**: agent's action reduces another's payoff (pollution, congestion). **Positive externality**: agent's action increases another's payoff (vaccination, education, R&D). **Source:** Mas-Colell et al. (1995) Ch.11 pp.351-360.

For a producer-on-consumer negative externality (e.g., factory pollutes residents), the **social marginal cost** is the sum of private marginal cost and external marginal cost: **Source:** Mas-Colell et al. (1995) pp.350-382.

```
SMC(Q) = PMC(Q) + EMC(Q)
   SMC = social MC; PMC = private MC; EMC = external MC
```

The **competitive equilibrium** solves `P = PMC`; the **social optimum** solves `P = SMC`. With `EMC > 0` (negative externality), social optimum produces less than competitive equilibrium. The **Pigovian tax** `τ = EMC(Q_social)` per unit of output shifts the private cost curve up to coincide with the social cost curve, restoring market efficiency. **Source:** Mas-Colell et al. (1995) Ch.11 pp.360-370.

A **public good** is non-rival (`x_i = x_j` for all agents: everyone consumes the same quantity) and non-excludable (no agent can be prevented from consuming). The **Samuelson condition** for optimal provision of a public good `G` sets aggregate marginal benefit equal to marginal cost: **Source:** Mas-Colell et al. (1995) pp.350-382.

```
Σ_i MRS_{i, G→x} = MRT_{G→x}        (Samuelson optimality)
```

where `MRS_{i, G→x}` is consumer `i`'s marginal rate of substitution between the public good `G` and the numeraire `x`, and `MRT_{G→x}` is the marginal rate of transformation in production. This contrasts with the private-good condition `MRS_i = p_G/p_x` for each individual: the public good's optimality aggregates marginal benefits, while the private good's optimality equates each individual's marginal benefit to the price. **Source:** Mas-Colell et al. (1995) Ch.11 pp.370-380.

The **Coase theorem** states: if property rights are well-defined and transaction costs are zero, voluntary bargaining among affected parties achieves the efficient allocation regardless of the initial assignment of rights. The initial assignment affects who pays whom (distribution) but not the level of the externality-causing activity (efficiency). **Source:** Mas-Colell et al. (1995) Ch.11 pp.360-365.

## Mathematical Reasoning

The Pigovian tax remedy works algebraically by aligning private incentives with social welfare. Without the tax, the firm chooses output where `P = PMC`; with the tax, the firm chooses output where `P − τ = PMC`, equivalent to `P = PMC + τ`. Setting `τ = EMC(Q_social)` evaluated at the social optimum makes the post-tax private FOC coincide with the social FOC `P = SMC`. The tax internalizes the externality by making the polluter pay the social cost of pollution. The tax revenue can be redistributed lump-sum without distorting the equilibrium. **Source:** Mas-Colell et al. (1995) Ch.11 pp.365-375.

The Samuelson condition derives from the social planner's first-order condition for public-goods provision. With a single public good `G` and a private numeraire `x`, the planner maximizes `Σ_i u_i(x_i, G)` subject to `Σ_i x_i + c(G) = ω` (resource constraint, `c(G)` is the cost of providing `G`). The FOC for `G` gives `Σ_i (∂u_i / ∂G) = c'(G)`; dividing each consumer's marginal utility by her marginal utility of private consumption (`λ_i = ∂u_i / ∂x_i`) and noting that the planner uses the same shadow price for the private good across all consumers, gives `Σ_i MRS_{i, G→x} = MRT_{G→x} = c'(G)` (where MRT equals the per-unit cost of providing one more unit of `G`). The intuition: each consumer's MRS measures her willingness-to-pay for an extra unit of `G`; the aggregate is the social willingness-to-pay; the planner provides `G` up to where social willingness-to-pay equals the marginal cost. **Source:** Mas-Colell et al. (1995) Ch.11 pp.370-382.

**Free-rider problem**: with `N` consumers, each individual's marginal incentive to contribute to public-goods provision is her own MRS, which is below the social `Σ MRS`. Each consumer therefore underprovides relative to the social optimum; the aggregate provision is `1/N`-th of the social optimum in the symmetric case (Lindahl-Wicksell). The free-rider problem motivates government provision financed by general taxation. The Coase theorem applies primarily to bilateral or few-party externalities where bargaining is feasible; for large-scale externalities (climate change, public-goods provision with millions of beneficiaries) transaction costs are prohibitive and Pigovian / government solutions dominate. **Source:** Mas-Colell et al. (1995) Ch.11 pp.370-382.

## Boundary Notes

Externalities and public goods are **two distinct market-failure classes** that the v10 BOUNDARY-DISCIPLINE keeps separate from monopoly (a third failure class covered in the sibling `ec-monopoly-pricing` card) and from asymmetric information (the fourth class in MWG Ch.13-14, out of v10 scope). Each class fails a different FFWT assumption: externality fails the no-uncompensated-spillover assumption; public good fails the rivalry / excludability assumption; monopoly fails the price-taking / no-market-power assumption; asymmetric information fails the complete-information assumption. The appropriate policy instrument varies by class — Pigovian tax for externality; provision-plus-financing for public good; antitrust / regulation for monopoly; mechanism design for information. **Source:** Mas-Colell et al. (1995) Ch.11 pp.351-356.

The Pigovian tax `τ = EMC(Q_social)` and the Samuelson condition `Σ MRS = MRT` are **first-best instruments** — they restore the social optimum exactly given perfect information about the externality / public-good benefits. In practice, the planner does not observe `EMC` or each consumer's `MRS` directly; the **second-best literature** addresses how to design instruments under incomplete information (e.g., tradable pollution permits as a quantity instrument vs Pigovian tax as a price instrument; Clarke-Groves mechanisms for incentive-compatible public-good provision). This card stops at the first-best instruments per scope; second-best treatment lives in MWG Ch.11 §11.D pp.378-382 and is the subject of an extensive applied literature out of v10 scope. **Source:** Mas-Colell et al. (1995) Ch.11 pp.378-382.

## See Also

- [`ec-perfect-competition-equilibrium`](./ec-perfect-competition-equilibrium.md) — the welfare-theorem baseline that externalities break
- [`ec-welfare-theorems`](./ec-welfare-theorems.md) — formal FFWT and SFWT; externalities violate the FFWT's assumption that all spillovers are market-mediated
- [`ec-monopoly-pricing`](./ec-monopoly-pricing.md) — a different source of market failure (market power vs externality)

## Escalate to Raw When

The full quasi-linear framework for externality analysis (where consumer utility is `u_i(x_i, h) = x_i + φ_i(h)` with `h` the externality-generating activity) is in MWG Ch.11 pp.356-365 and supports the cleanest Coase-theorem derivation. The taxonomy of public-goods provision mechanisms (Lindahl equilibrium, Clarke-Groves mechanisms, voluntary contribution games) is in MWG Ch.11 pp.378-382. For asymmetric-information sources of market failure (adverse selection, moral hazard) see MWG Ch.13-14; these are out of v10 scope. **Source:** Mas-Colell et al. (1995) pp.350-382.
