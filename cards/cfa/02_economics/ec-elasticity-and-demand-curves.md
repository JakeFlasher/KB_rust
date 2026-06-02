---
schema_version: "cacg.v0"
id: "ec-elasticity-and-demand-curves"
title: "Elasticity and Demand Curves (CFA L1 R8)"
reading_id: "02_economics"
summary: "CFA L1 R8 elasticity vocabulary at exam depth: price (ε_p), income (ε_y), cross-price (ε_xy) elasticities; slope vs elasticity distinction; elastic/unit-elastic/inelastic classification by |ε_p|; total-revenue test."
tags: ["economics", "elasticity-demand"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p586:0804"
    chunk_hash: "709147d326da0db860aa9ef84650ac2a778d60ce122f5ac61eb7a2deacb9311c"
    page_range: [586, 586]
    quote: "In the case of own-price elasticity of demand, that measure is illustrated in Equation 5: E Q P p d x d x x % % This equation expresses the sensitivity of the quantity demanded to a change in price."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p582:0798"
    chunk_hash: "67e88d478828c19e234974d611fd05ca06841647974b8417b4713ad362be2be8"
    page_range: [582, 583]
    quote: "In this section, we examine three important topics concerning the demand side of the model: (1) elasticities, (2) substitution and income effects, and (3) normal and inferior goods."
    edge_type: "defines"
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p059:0083"
    chunk_hash: "a40528fe29698293b9b2654206eb06d2a58a67e3b0d560b51d513a2842d29213"
    page_range: [59, 60]
    quote: "This material offers important insights into the structure of 40 SECTION 3.B: PRE FERENCE REL ATIONS: BASIC PR OPER TIES 41 preference-based demand theory."
    edge_type: "supports"
card_hash: "d0aa5af4154c3aa9ce6879cfabe0b9b9a4b1cf23f58759e109e9e870e24754f9"
---
# Elasticity and Demand Curves (CFA L1 R8)

## Intuition

Elasticity measures the **responsiveness** of one quantity to another in unit-free percentage terms. The **price elasticity of demand** `ε_p = (% ΔQ) / (% ΔP)` answers: when price rises by one percent, by how many percent does quantity demanded fall? Because both numerator and denominator are percent changes, the elasticity is unit-free — it compares quantities and prices in different units (apples vs USD/apple, barrels vs USD/barrel) on a common scale. The slope of the demand curve `dQ/dP`, in contrast, is unit-dependent and changes with the choice of units. **Source:** CFA Institute (2022) Vol.2 pp.3-61.

```
   slope vs elasticity along a linear demand curve

   P
   ^
   |* (high P, low Q)   ← |ε_p| > 1  (elastic region)
   |   *
   |     *
   |        *
   |  midpoint *   ← |ε_p| = 1  (unit-elastic; TR maximum)
   |             *
   |                *
   |                   *   ← |ε_p| < 1  (inelastic region)
   |                      * (low P, high Q)
   +----------------------------+--> Q

   linear demand: constant slope dQ/dP everywhere, but
   elasticity ε_p = (P/Q) · (dQ/dP) VARIES along the curve
   because P/Q changes from high (top-left) to low (bottom-right)
```

The classic **total-revenue test** is the standard L1 exam check: if demand is elastic (`|ε_p| > 1`), a price increase reduces total revenue (`TR = P · Q`); if demand is inelastic (`|ε_p| < 1`), a price increase raises total revenue; at unit elasticity (`|ε_p| = 1`), total revenue is at its maximum. The intuition: when `|ε_p| > 1`, the percentage drop in quantity exceeds the percentage rise in price, so revenue falls; when `|ε_p| < 1`, quantity barely changes and the higher price wins. The total-revenue lens is the practical reason the elasticity vocabulary matters for firms setting prices and for governments setting taxes on demand-inelastic goods. **Source:** CFA Institute (2022) Vol.2 pp.3-61.

## Definition

The **own-price elasticity of demand**, in symbolic form. **Source:** CFA Institute (2022) Vol.2 pp.3-61.

```
ε_p  =  (% ΔQ) / (% ΔP)  =  (dQ/Q) / (dP/P)  =  (P/Q) · (dQ/dP)   (point)
```

For linear demand `Q = a − b · P`, the slope is constant `dQ/dP = −b`, and the point elasticity at `(P, Q)` is `ε_p = −b · P / Q`. Because `P/Q` varies along the demand curve, the elasticity varies even when the slope is constant — high at high prices (top of the curve), low at low prices (bottom of the curve), and exactly `−1` at the midpoint where total revenue is maximized. **Source:** CFA Institute (2022) Vol.2 pp.3-61.

The **arc elasticity** (the formula CFA L1 most often tests) uses the midpoint to avoid the asymmetry of point elasticity computed at one endpoint vs the other. **Source:** CFA Institute (2022) Vol.2 pp.3-61.

```
ε_p^arc  =  (Q2 − Q1) / [ (Q1 + Q2) / 2 ]   ÷   (P2 − P1) / [ (P1 + P2) / 2 ]
```

The CFA L1 exam treatment stays symbolic (no plug-and-chug arithmetic), but the arc formula's structure is the canonical one taught. **Source:** CFA Institute (2022) Vol.2 pp.3-61.

The **income elasticity of demand** `ε_y = (% ΔQ) / (% ΔY)` classifies goods by sign and magnitude. **Source:** CFA Institute (2022) Vol.2 pp.3-61.

```
ε_y > 0          : normal good
ε_y > 1          : luxury (income-elastic normal good)
0 < ε_y < 1      : necessity (income-inelastic normal good)
ε_y < 0          : inferior good
```

The **cross-price elasticity** `ε_{x,y} = (% ΔQ_x) / (% ΔP_y)` classifies goods as substitutes (`ε_{x,y} > 0`: when good y's price rises, demand for x rises) or complements (`ε_{x,y} < 0`: when good y's price rises, demand for x falls). **Source:** CFA Institute (2022) Vol.2 pp.3-61.

## Mathematical Reasoning

The **total-revenue derivation** ties elasticity to revenue's response to price. Total revenue is `TR = P · Q`; differentiating with respect to `P` gives `dTR/dP = Q + P · (dQ/dP) = Q · (1 + ε_p)`. So `dTR/dP > 0` iff `1 + ε_p > 0` iff `ε_p > −1` (since `ε_p` is negative for normal demand). Equivalently, a price increase raises revenue iff `|ε_p| < 1` (inelastic). At `|ε_p| = 1`, `dTR/dP = 0` and revenue is locally maximized along the demand curve. This is the symbolic statement underlying the L1 exam's total-revenue test. **Source:** CFA Institute (2022) Vol.2 pp.3-61.

The **determinants of price elasticity** the CFA curriculum emphasizes: (i) availability of close substitutes (more substitutes → more elastic; salt has few substitutes and is inelastic, brand-specific cola has many substitutes and is elastic); (ii) share of income spent on the good (larger budget share → more elastic; rent is inelastic if you must live somewhere, even though it's a large budget share, because few short-run substitutes exist); (iii) time horizon (longer horizon → more elastic; gasoline demand is inelastic in the short run but elastic over years as households substitute toward fuel-efficient vehicles). The L1 exam tests these determinants qualitatively — the right answer is structural reasoning, not numerical computation. **Source:** CFA Institute (2022) Vol.2 pp.3-61.

The bridge to the MWG microeconomic-foundation: the Walrasian demand `x(p, w)` derived in sibling [`ec-consumer-utility-and-demand`](./ec-consumer-utility-and-demand.md) yields the elasticities as derivatives of the demand function evaluated at the equilibrium price/income. The Slutsky decomposition splits the Marshallian own-price effect into a substitution effect (always negative for normal goods) and an income effect (negative for normal goods, positive for inferior goods), giving the theoretical foundation for why elastic / inelastic classifications hold. **Source:** Mas-Colell et al. (1995) Ch.3 pp.40-104.

## See Also

- [`ec-consumer-utility-and-demand`](./ec-consumer-utility-and-demand.md) — MWG Walrasian/Hicksian demand foundations; this CFA L1 elasticity card is the exam-depth re-teaching
- [`ec-aggregate-supply-demand-mechanics`](./ec-aggregate-supply-demand-mechanics.md) — macro-level AS-AD that aggregates individual demand elasticities
- [`ec-consumer-preference-and-choice`](./ec-consumer-preference-and-choice.md) — the rationality axioms that underlie the demand function whose elasticity is computed here

## Escalate to Raw When

The MWG microeconomic-foundation derivation of elasticities from the consumer's utility-maximization problem (Slutsky decomposition, gross substitutes / complements, Giffen-good edge case) is in Mas-Colell et al. (1995) Ch.3 pp.40-104. The empirical estimation of demand elasticities (instrumental-variable identification, panel-data techniques, willingness-to-pay surveys) is graduate-econometrics material out of v10 scope but cross-references the discussion of regression methodology in subcorpus 01 quantitative methods. The applied-pricing literature on demand-curve estimation in marketing and industrial-organization contexts (conjoint analysis, choice experiments, structural-discrete-choice models) is also out of v10 scope. **Source:** Mas-Colell et al. (1995) Ch.3 pp.40-104; CFA Institute (2022) Vol.2 pp.3-61.
