---
schema_version: "cacg.v0"
id: "ec-aggregate-supply-demand-mechanics"
title: "Aggregate Supply / Aggregate Demand Mechanics (CFA L1 R10)"
reading_id: "02_economics"
summary: "CFA L1 R10 AS-AD framework at exam depth: AD downward-sloping via wealth/interest-rate/exchange-rate effects, SRAS upward-sloping under sticky wages, LRAS vertical at potential output. Demand-pull (AD shift) vs cost-push (SRAS shift) inflation taxonomy."
tags: ["economics", "aggregate-supply"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p720:1000"
    chunk_hash: "7c1d8f74df1b8b3f9b3bcbbcc22cdbbb1462b46e9abb06d1757925826ab05b3d"
    page_range: [720, 721]
    quote: "The wealth effect is one reason that the aggregate demand curve is downward sloping."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p731:1017"
    chunk_hash: "d611c4ac13f095ca97b2118f2ebbfe1c1b5b8305c7782bf91a0218c5318a56ca"
    page_range: [731, 732]
    quote: "An increase in nominal wages raises production costs, resulting in a decrease in AS and a leftward shift in the SRAS curve."
    edge_type: "defines"
card_hash: "27f51c77cab278676d887a4f8ed9407f9afaa4d7f92e1aa794a25d44af0ba1c1"
---
# Aggregate Supply / Aggregate Demand Mechanics (CFA L1 R10)

## Intuition

The **AS-AD framework** is the standard CFA L1 lens for short-run macroeconomic equilibrium: the **aggregate-demand (AD) curve** is downward-sloping in `(P, Y)` space (higher price level reduces real wealth, real money balances, and net exports, each lowering demand); the **short-run aggregate-supply (SRAS) curve** is upward-sloping (firms expand output as prices rise faster than nominal wages, raising real margins); the **long-run aggregate-supply (LRAS) curve** is vertical at potential output `Y*` (in the long run all nominal variables are flexible, so the price level adjusts to whatever level clears the labor market at full employment). Short-run equilibrium is the AD-SRAS intersection; long-run equilibrium adds the constraint that the equilibrium output equals `Y*`. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

```
   price level P
   ^                      LRAS
   |                       |
   |                       | (vertical at Y*)
   |                       |
   |            SRAS      |
   |             /        |
   |            /         |
   |           /          |
   |          /  *        |  ← SR equilibrium (AD ∩ SRAS)
   |         /  / \       |
   |        /  /   AD     |
   |       /  /     \     |
   |          /       \   |
   |                    \ |
   |                     \|  ← LR equilibrium (AD ∩ LRAS)
   |                      *
   |                      |\
   |                      | \
   +----------------------+--+---> Y (real output)
                          Y*
```

The two inflation taxonomies the L1 exam tests are: **demand-pull inflation** (AD shifts right; both `P` and `Y` rise in the short run; in the long run wages adjust and only `P` rises while `Y` returns to `Y*`); and **cost-push inflation** (SRAS shifts left, e.g., from an oil-price shock; both `P` rises and `Y` falls in the short run — stagflation; in the long run real wages adjust and the economy returns to `Y*` at a higher price level). The framework is mechanical and the L1 exam tests the directional reasoning rather than numerical computation. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

## Definition

The **aggregate-demand (AD) curve** in symbolic CFA L1 form. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

```
Y^d  =  C(Y − T, ...) + I(r, ...) + G + NX(P, Y_foreign, ...)   (open AD)
```

where `C` is consumption (function of disposable income), `I` is investment (function of real interest rate `r`), `G` is government spending (exogenous), and `NX` is net exports (function of relative prices and foreign output). Holding monetary policy fixed, AD is downward-sloping in `P` because: (a) **wealth effect** — higher `P` reduces real money balances and real wealth, lowering consumption; (b) **interest-rate effect** — higher `P` raises money demand at given nominal money supply, raising `r` and lowering investment; (c) **exchange-rate effect** — higher domestic `P` makes domestic goods relatively expensive, lowering net exports. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

The **aggregate-supply curves** in symbolic form. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

```
SRAS:   Y^s = Y* + α · (P − P^e)        (upward-sloping in P)
LRAS:   Y^s = Y*                        (vertical at Y*; price-level neutral)
```

where `Y*` is potential output, `α > 0` is the SRAS slope (responsiveness of output to price-expectation surprise), and `P^e` is the expected price level. The crucial L1 distinction: SRAS is upward-sloping because nominal wages and other costs are sticky in the short run (firms see higher prices but pay the same wages, expanding output for higher margins). LRAS is vertical because in the long run all nominal variables adjust — only real factors (technology, capital, labor force, productivity) determine `Y*`. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

The **equilibrium conditions** are stated as. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

```
short-run:  Y^d(P, ·) = Y^s_SRAS(P, P^e, ·)        (AD ∩ SRAS)
long-run:   Y^d(P, ·) = Y^s_LRAS                   (AD ∩ LRAS = Y*)
```

The long-run equilibrium requires `P` to adjust so that the AD curve intersects LRAS at `Y*`; any deviation from `Y*` triggers wage-price adjustments that shift SRAS until short-run equilibrium coincides with long-run equilibrium. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

## Mathematical Reasoning

The **shift-vs-movement distinction** is the standard L1 exam check. A movement ALONG the AD curve is a price-level change holding everything else constant (e.g., a higher price level reducing real wealth and lowering quantity demanded). A SHIFT of the AD curve is a change in any of its determinants OTHER than `P` — examples include an exogenous increase in `G` (fiscal stimulus), an exogenous decrease in `T` (tax cut), an autonomous rise in consumer confidence raising `C`, or a foreign-income increase raising `NX`. Similarly, an SRAS shift comes from a wage-price expectation revision (`P^e` change), an input-cost shock (oil), or a productivity shock affecting the implicit `α` parameter. The L1 exam tests whether students can correctly identify each shock as a movement vs shift and trace the directional consequences in `(P, Y)` space. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

The **demand-pull adjustment dynamics**: starting from long-run equilibrium at `(P_0, Y*)`, an AD-rightward shock raises short-run equilibrium to `(P_1, Y_1)` with `P_1 > P_0` and `Y_1 > Y*`. Over time, workers and firms revise `P^e` upward to match the new actual `P`; SRAS shifts leftward (`P^e` rise reduces output supplied at any given `P`); the economy moves up along the new AD curve until reaching long-run equilibrium at `(P_2, Y*)` with `P_2 > P_1 > P_0`. The output gain is purely transitional; the long-run effect of an AD shock is entirely on the price level. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

The **cost-push adjustment dynamics**: starting from `(P_0, Y*)`, an SRAS-leftward shock (negative supply shock, e.g., oil-price spike) moves short-run equilibrium to `(P_1, Y_1)` with `P_1 > P_0` and `Y_1 < Y*` — the stagflation outcome (rising prices with falling output). Over time, wages adjust downward as unemployment rises (or productivity recovers from the shock), SRAS shifts back rightward, and the economy returns to long-run equilibrium at `(P_0, Y*)` — the original price level, since the shock was temporary. If the supply shock is permanent (e.g., a permanent productivity loss), `Y*` itself falls, and the new long-run equilibrium has both lower output and a permanent price-level adjustment. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

## See Also

- [`ec-aggregate-demand-representative-consumer`](./ec-aggregate-demand-representative-consumer.md) — MWG aggregation foundations behind the AD curve's representative-consumer assumption
- [`ec-business-cycles-and-output-gaps`](./ec-business-cycles-and-output-gaps.md) — CFA L1 R11 business-cycle taxonomy that uses the output gap `Y − Y*` as the primary diagnostic
- [`ec-monetary-fiscal-policy-mechanics-l1`](./ec-monetary-fiscal-policy-mechanics-l1.md) — CFA L1 R12 policy framework that operates on AD via monetary / fiscal levers

## Escalate to Raw When

The DSGE micro-foundation of the AS-AD framework (Romer-style stochastic Euler equation + Calvo Phillips curve as the modern derivation) is in the Romer-anchored Batch 1 cards (`ec-ramsey-cass-koopmans-savings`, `ec-real-business-cycle-theory`, `ec-nominal-rigidity-and-NK-monetary`). The empirical macroeconometric estimation of AS-AD slopes (VAR identification of supply vs demand shocks, structural VAR with sign restrictions) is graduate research literature out of v10 scope. The historical macroeconomic episodes (1970s stagflation, 2008 financial crisis, 2020-21 supply-driven inflation) provide rich case studies but are out-of-scope as primary content for an intuition-plus-rigor card. **Source:** CFA Institute (2022) Vol.2 pp.143-197.
