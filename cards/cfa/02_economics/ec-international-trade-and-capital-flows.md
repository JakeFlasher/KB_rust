---
schema_version: "cacg.v0"
id: "ec-international-trade-and-capital-flows"
title: "International Trade and Capital Flows (CFA L1 R13)"
reading_id: "02_economics"
summary: "CFA L1 R13 international trade and BOP: comparative-advantage (Ricardian unit-labor) trade-gains argument; trade-policy taxonomy (tariffs/quotas/VER/subsidies/embargoes) with deadweight-loss diagrams; BOP accounting identity CA + KA = 0."
tags: ["economics", "international-trade"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p930:1309"
    chunk_hash: "9f2df0cda772471d88b87d12bf870c575334c029314056d3774baf7cfe7d67e2"
    page_range: [930, 931]
    quote: "In this section, we define comparative advantage, distinguish it from the notion of absolute advantage, and demonstrate the gains from trading in accordance with comparative advantage."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p952:1345"
    chunk_hash: "adba971f49f7f432f09a96241bd010aa04c680d22ec6cd0f00b4cdeb85870add"
    page_range: [952, 953]
    quote: "9.1 Balance of Payments Accounts The BOP is a double-entry system in which every transaction involves both a debit and credit."
    edge_type: "defines"
card_hash: "3510a42b9a4f26b5d1ca9dd53db84f59356b9d5c89b583e409b7a74bcf064214"
---
# International Trade and Capital Flows (CFA L1 R13)

## Intuition

The CFA L1 R13 reading re-teaches the canonical international-trade arguments and the balance-of-payments accounting identity at exam depth. Three core ideas: (a) **comparative advantage** — even a country that is absolutely worse at producing all goods can gain from trade by specializing in the good for which its opportunity cost is lowest, a result Ricardo originally demonstrated and which underlies the standard pro-free-trade welfare argument; (b) **trade-policy welfare costs** — tariffs, quotas, and other trade barriers create deadweight loss (the area on the supply-demand diagram between the with-trade and without-trade equilibria that no one captures), and the welfare gain to protected producers is smaller than the welfare loss to consumers; (c) **balance-of-payments accounting** — the current account (trade balance + net factor income + net transfers) and the capital account (net financial flows) must sum to zero (modulo statistical discrepancy), because every cross-border transaction has two sides. **Source:** CFA Institute (2022) Vol.2 pp.343-441.

```
   comparative advantage + BOP identity (CFA L1 R13)

   comparative-advantage condition (symbolic):       BOP identity:
                                                      CA + KA  =  zero
   country A produces goods X and Y                   (modulo statistical
   with unit-labor requirements a_LX, a_LY            discrepancy)
   country B produces with b_LX, b_LY
                                                      CA = current account
   A has comparative advantage in X iff              (trade balance + factor
       (opp_cost of X in A) < (opp_cost in B)         income + net transfers)
       a_LX / a_LY  <  b_LX / b_LY
                                                      KA = capital account
   This holds independent of absolute advantage:      (net financial flows +
   A can have a_LX > b_LX (worse in X absolutely)     reserve changes; under
   yet still have a comparative advantage in X        new IMF taxonomy KA
   if A's RELATIVE cost in X is lower.                splits into financial-
                                                      account and small-K)
   A specializes in X; B specializes in Y;
   both gain from trade vs autarky.

   trade-barrier welfare cost (tariff example):
   producer-surplus gain + tariff revenue
   < consumer-surplus loss
   → deadweight loss = net welfare cost of the barrier
```

The **CFA L1 trade-policy taxonomy** the exam tests: **tariffs** (per-unit or ad-valorem taxes on imports), **quotas** (quantity caps on imports), **voluntary export restraints** (quota-like agreements imposed by exporting country), **export subsidies** (payments to domestic producers), **embargoes** (total bans), each with associated welfare-cost diagrams. The L1 question format typically asks which barrier creates the largest deadweight loss for a given protection level (answer: quotas and VERs are equivalent to tariffs in price/quantity outcomes but transfer the quota rent to importers/exporters instead of government revenue). **Source:** CFA Institute (2022) Vol.2 pp.343-441.

## Definition

The **Ricardian comparative-advantage condition** in symbolic form. **Source:** CFA Institute (2022) Vol.2 pp.343-441.

```
country A specializes in good X if  (opp_cost_X in A) < (opp_cost_X in B)
                                    a_LX / a_LY  <  b_LX / b_LY

where:  a_LX = labor required per unit of good X in country A
        a_LY = labor required per unit of good Y in country A
        b_LX, b_LY = same for country B
```

This is a strictly weaker condition than absolute advantage (which would require `a_LX < b_LX`). The comparative-advantage condition can hold even when one country is absolutely more productive at everything — what matters is the RELATIVE opportunity cost between goods within each country. **Source:** CFA Institute (2022) Vol.2 pp.343-441.

The **trade-policy welfare-cost taxonomy** in symbolic form. **Source:** CFA Institute (2022) Vol.2 pp.343-441.

```
tariff:               domestic price ↑ by tariff amount
                      producer surplus ↑, consumer surplus ↓, gov revenue ↑
                      deadweight loss = production + consumption distortion
quota:                same as tariff in price/quantity, BUT quota rent goes
                      to license holders (importers) instead of government
                      (deadweight loss is the same; the rectangle of revenue
                      is captured privately instead of publicly)
VER:                  voluntary export restraint = quota imposed by exporter
                      same as quota but rent goes to foreign exporters
export subsidy:       lowers domestic-producer price relative to foreign;
                      creates terms-of-trade cost (export at less than the
                      consumer would pay)
embargo:              total ban; extreme tariff with infinite welfare cost
                      for affected goods; typically driven by non-economic
                      considerations (sanctions / national security)
```

**Source:** CFA Institute (2022) Vol.2 pp.343-441.

The **balance-of-payments accounting identity**. **Source:** CFA Institute (2022) Vol.2 pp.343-441.

```
CA  +  KA  =  0          [modulo statistical discrepancy]

CA (Current Account)  =  trade balance (exports − imports)
                       + net factor income from abroad
                       + net transfers

KA (Capital + Financial Account)  =  net financial inflows
                                   + net capital transfers
                                   + change in reserve assets
```

The accounting identity is exact: every cross-border transaction has two sides (e.g., when a US firm sells equipment to a German firm, the US records an export in the CA and a financial-account inflow in the KA when the payment is received). The empirical "statistical discrepancy" line reconciles measurement errors. **Source:** CFA Institute (2022) Vol.2 pp.343-441.

## Mathematical Reasoning

The **gains-from-trade derivation**: in the Ricardian two-country, two-good, one-input model, the production-possibility frontier (PPF) for each country is linear, and the world PPF (the set of feasible combinations of two goods produced by both countries combined) is piecewise-linear with the "kink" at the specialization point. Free trade allows the world to operate on the world PPF; autarky restricts each country to its own (smaller) PPF. The gains-from-trade theorem: under free trade, each country can consume more of at least one good than under autarky, and never less of the other — a strict Pareto improvement (assuming lump-sum redistribution to compensate any within-country losers). The L1 framing emphasizes the directional outcome; the formal mathematical proof requires a few lines of linear-programming algebra. **Source:** CFA Institute (2022) Vol.2 pp.343-441.

The **deadweight-loss-from-tariff derivation**: on a supply-demand diagram with downward-sloping domestic demand and upward-sloping domestic supply, the autarky equilibrium has domestic-only `(P, Q)` clearing. Free trade at world price `P_w < P_autarky` gives quantity supplied domestically `Q_S^free`, quantity demanded `Q_D^free`, and imports `Q_D^free − Q_S^free`. A tariff `t` raises the domestic price to `P_w + t`, reducing imports to `Q_D^tariff − Q_S^tariff`. The producer-surplus gain is the trapezoid between the autarky-price and new-domestic-price horizontal lines; the consumer-surplus loss is the larger trapezoid; the government-revenue rectangle is the tariff times the new import quantity. The deadweight-loss triangles are the production-distortion triangle (cost of producing units that should have been imported) plus the consumption-distortion triangle (consumers giving up units they would have valued above world price). **Source:** CFA Institute (2022) Vol.2 pp.343-441.

The **BOP-identity consequence**: because CA + KA = 0, a country's persistent CA deficit must be financed by a matching KA surplus (foreign investment flowing in to fund domestic consumption / investment in excess of saving). The L1 framing emphasizes the accounting identity rather than the structural determinants. The structural-macro literature explains CA deficits via national-saving-vs-investment imbalances (CA = S − I); a country with low saving relative to investment runs a CA deficit and a KA surplus. The policy implications (sustainable CA deficits, sudden-stop risk, real-exchange-rate adjustment) are taught at L2+ depth. **Source:** CFA Institute (2022) Vol.2 pp.343-441.

## See Also

- [`ec-currency-exchange-rates-and-parity`](./ec-currency-exchange-rates-and-parity.md) — capital-flow framework + Mundell-Fleming that connects this card's BOP-identity to FX policy regimes
- [`ec-monetary-fiscal-policy-mechanics-l1`](./ec-monetary-fiscal-policy-mechanics-l1.md) — fiscal-policy framework that interacts with trade-policy welfare analysis (e.g., budget deficits and CA deficits via the "twin deficits" hypothesis)

## Escalate to Raw When

The full CFA L1 R13 reading (chapter-detail treatment of regional trade agreements, the WTO institutional structure, dispute-settlement mechanisms, and the empirical literature on trade-agreement welfare effects) is in CFA Institute (2022) Vol.2 pp.343-441. The graduate-trade-theory extensions (Heckscher-Ohlin factor-endowment model, Stolper-Samuelson distributional effects of trade, factor-price equalization theorem, New Trade Theory with monopolistic competition and intra-industry trade, gravity-equation empirics) are out of v10 scope. The macroeconomic-imbalances literature on CA deficits (Obstfeld-Rogoff intertemporal-trade model, sudden-stop crises, original-sin in foreign debt denomination) is also out of v10 scope. **Source:** CFA Institute (2022) Vol.2 pp.343-441.
