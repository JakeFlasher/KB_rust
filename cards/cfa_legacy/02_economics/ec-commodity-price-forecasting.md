---
schema_version: "cacg.v0"
id: "ec-commodity-price-forecasting"
title: "Commodity Price Forecasting Framework (CFA L1, per DEC-6)"
reading_id: "02_economics"
summary: "CFA L1 R10/R11 commodity-price forecasting framework: integrate business-cycle context, AS-AD shifts, and commodity-specific factors (capacity/inventory/geopolitical/weather). Bridges to 05 equity cyclicality adjustment."
tags: ["economics", "commodity-price"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p724:1007"
    chunk_hash: "6810ee48e8444ead1f662c155f13db5486945505818eefe70a19dee0de8b952e"
    page_range: [724, 725]
    quote: "Shifts in the AD and AS curves determine the short-run changes in the economy associated with the business cycle."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p739:1030"
    chunk_hash: "3379255f9221c4cf7ecd1a6a5e31ed6216a7eb1c21127344be6f70ffbd302a8f"
    page_range: [739, 740]
    quote: "■ Reduce investments in commodities and/or commodity-oriented companies because the decline in commodity prices will slow revenue growth and reduce profit margins."
    edge_type: "defines"
card_hash: "9a015cd7af9ed22ebab5d5d1b561d5b35eb3f5608b434a0ec84a4b0f8309c78d"
---
# Commodity Price Forecasting Framework (CFA L1, per DEC-6)

## Intuition

Commodity prices are driven by the interplay of cyclical demand (global growth, industrial-production trends, business-cycle phase) and supply (production capacity, marginal-cost curves, geopolitical disruption, weather shocks for ags). The CFA L1 framework treats commodity-price forecasting as a structured combination of three lenses: (a) **macroeconomic cycle context** — where the global economy sits in the business cycle (sibling [`ec-business-cycles-and-output-gaps`](./ec-business-cycles-and-output-gaps.md)); (b) **AS-AD aggregate framework** — how shifts in either curve translate to commodity-demand or commodity-supply movements (sibling [`ec-aggregate-supply-demand-mechanics`](./ec-aggregate-supply-demand-mechanics.md)); and (c) **commodity-specific factors** — industry capacity, inventory, geopolitical risk, weather. The L1 framing emphasizes structural reasoning over forecasting accuracy; commodity-price forecasts have high uncertainty even from well-specified frameworks. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

```
   commodity-price forecasting framework (CFA L1, DEC-6)

   global macro cycle phase  +  AS-AD shifts           +  commodity-specific
   (expansion/peak/             (demand-pull vs           (capacity/inventory/
    contraction/trough)          cost-push inflation)      geopolitical/weather)

         |                              |                          |
         v                              v                          v
   directional bias on              bias on commodity         microstructural
   commodity demand                  prices via energy /        adjustments
   (growth-sensitive vs              input-cost channel
    growth-insensitive)
                                     |
                                     v
                       commodity-price forecast
                       (intuition-first; high uncertainty)
                                     |
                                     v
                       feeds into 05 equity valuation
                       (cycle-adjusted earnings + cyclicality
                        adjustment — see Damodaran framework
                        in [eq-cyclicality-and-cycle-adjustment])
```

The L1 curriculum places commodity-price forecasting within R10 Aggregate Output (the supply-demand framework determining commodity equilibrium) and connects it to R11 Business Cycles (the cyclical positioning of commodities as growth-sensitive vs growth-insensitive assets). The 05 equity vertical's [`eq-cyclicality-and-cycle-adjustment`](../05_equity/eq-cyclicality-and-cycle-adjustment.md) card uses a mid-cycle-earnings methodology to value cyclical equities (steel, copper, oil-and-gas, mining); per the v10 plan's resolved DEC-6, this 02 card closes the prior cross-vertical `future-02 Economics` deferral in that 05 card by supplying the macroeconomic forecasting context (the 05 card now cross-links to this card rather than carrying a deferral placeholder). **Source:** CFA Institute (2022) Vol.2 pp.143-197.

## Definition

The **commodity-price equilibrium** in symbolic AS-AD form. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

```
P_commodity*  =  point at which commodity supply = commodity demand
                 (both functions of global activity, capacity, FX,
                  storage cost, geopolitical risk)
```

Shifts in commodity demand (e.g., from a China growth surge raising industrial-metals demand) or commodity supply (e.g., from OPEC+ production cuts) translate to price changes via the standard AS-AD shift mechanics taught in sibling [`ec-aggregate-supply-demand-mechanics`](./ec-aggregate-supply-demand-mechanics.md). The L1 exam tests directional reasoning rather than equilibrium-price computation. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

The **business-cycle-to-commodity mapping** (CFA L1 R11 + R10 integration). **Source:** CFA Institute (2022) Vol.2 pp.143-197.

```
expansion / peak:   ↑ growth-sensitive commodities (industrial metals, energy)
                    relative outperformance vs growth-insensitive (gold)

contraction:        ↓ growth-sensitive commodities (recession demand drop)
                    ↑ defensive commodities (gold as safe-haven)

trough / early
  expansion:        rebound in growth-sensitive commodities ahead of
                    confirmed economic recovery (leading-indicator
                    behavior of copper / oil-equity prices)
```

**Source:** CFA Institute (2022) Vol.2 pp.143-197.

The **inflation-commodity relationship** (R10 + R12 integration). **Source:** CFA Institute (2022) Vol.2 pp.143-197.

```
demand-pull inflation:  rising commodity prices contribute to and
                        confirm the inflationary AD shift
cost-push inflation:    commodity-price shock IS the cost-push trigger
                        (oil-price spike → SRAS leftward shift → stagflation)
```

This bidirectional relationship — commodities as both inflation symptom and inflation cause — is one of the L1 R10 + R12 emphases. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

## Mathematical Reasoning

The **cycle-positioning forecasting heuristic** the L1 framework teaches: identify (a) where the global economy is in the business cycle (via the leading/coincident/lagging indicator framework from sibling [`ec-business-cycles-and-output-gaps`](./ec-business-cycles-and-output-gaps.md)), (b) which commodities are most sensitive to that cycle phase (industrial metals and energy at peaks; defensive commodities at troughs), and (c) which AS-AD shifts are active (demand-pull from China growth, supply-side from OPEC cuts, cost-push from energy shocks). The directional forecast then combines these three inputs into a qualitative bias rather than a precise number. The L1 exam tests the framework's directional consistency, not the numerical prediction. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

The **bridge to the equity-valuation vertical**: the cycle-adjustment methodology in [`eq-cyclicality-and-cycle-adjustment`](../05_equity/eq-cyclicality-and-cycle-adjustment.md) uses long-run (peak-to-peak or trough-to-trough) average earnings as the input to a DCF model for cyclical equities, to smooth out cyclical noise and yield a "normalized" valuation. The 02 commodity-forecasting framework provides the macroeconomic context (where in the cycle, which commodities, which shifts) that informs the cycle-adjustment estimate. Per the resolved DEC-6 of the v10 plan, this 02 card closes the prior `future-02 Economics` deferral in the equity card; the equity card's `Out of scope:` line now cross-references this card via a markdown link rather than carrying a deferral placeholder. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

The **forecasting-uncertainty disclaimer**: even with a well-specified L1 framework, commodity-price forecasts have high uncertainty because (a) macroeconomic forecasts themselves are uncertain (output-gap estimates revise heavily); (b) commodity-specific shocks (geopolitical, weather, OPEC+ politics) are unpredictable; (c) financial-market dynamics (speculative positioning, futures-curve dynamics — though futures-curve mechanics is out of v10 scope, treated in 07 derivatives) introduce additional noise. The L1 framing emphasizes structured reasoning over forecasting accuracy; practitioners use these frameworks as input to scenario analysis rather than point forecasts. **Source:** CFA Institute (2022) Vol.2 pp.143-197.

## See Also

- [`ec-aggregate-supply-demand-mechanics`](./ec-aggregate-supply-demand-mechanics.md) — AS-AD framework that the commodity-price equilibrium uses
- [`ec-business-cycles-and-output-gaps`](./ec-business-cycles-and-output-gaps.md) — business-cycle taxonomy that positions commodities as growth-sensitive vs defensive
- [`eq-cyclicality-and-cycle-adjustment`](../05_equity/eq-cyclicality-and-cycle-adjustment.md) — Damodaran cycle-adjustment for valuing cyclical equities (uses this card's macro forecasting context per DEC-6)

## Escalate to Raw When

The full futures-curve dynamics of commodities (contango vs backwardation, convenience yield, storage cost, term-structure of forward prices, calendar-spread arbitrage) are in 07 derivatives vertical and out of v10 02-Economics scope. The commodity-specific-fundamental forecasting at trader depth (oil-supply OPEC modeling, copper-mine production curves, grain stocks-to-use ratios, weather forecasting integration) is out of L1 scope. The Damodaran cycle-adjustment methodology in full (DCF with normalized earnings, peak-to-trough margin analysis, sector-specific cyclicality factors) is in 05 equity (`eq-cyclicality-and-cycle-adjustment`); per DEC-6, this 02 card supplies the macro forecasting context, and the 05 card retains the valuation methodology. **Source:** CFA Institute (2022) Vol.2 pp.143-197.
