---
schema_version: "cacg.v0"
id: "rm-real-rate-policy-rate-notes"
title: "Real Rate vs Policy Rate — L1 Notes Risk-Input Decomposition"
reading_id: "11_risk_management"
summary: "Fisher-effect decomposition of nominal rates into real-rate + inflation-expectation components plus the neutral-policy-rate framing, treated as a risk-input for the loss-distribution discount-rate channel; the topic is anchored in CFA L1 2022 Reading 12 'Monetary and Fiscal Policy' (CORRECTED to Vol.2, not Vol.6)."
tags: ["risk-management", "real-rate"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p859:1198"
    chunk_hash: "d9e56c0c54e6826545e31b79057bf49c53863062e1a4c4a0e9cf19f7f0120b24"
    page_range: [859, 859]
    quote: "the Fisher effect states that the real rate of interest in an economy is stable over time so that changes in nominal interest rates are the result of changes in expected inflation."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p882:1237"
    chunk_hash: "a5f1659c23aa21922992d976049771f8d508321ca695632591fe98f4484b33a4"
    page_range: [882, 883]
    quote: "the neutral policy rate for any economy comprises two components"
    edge_type: "supports"
card_hash: "863c3e9ab26a8d0663035e89a70a9b969a73b13ff68d821c7ef44e622b94bca1"
---
# Real Rate vs Policy Rate — L1 Notes Risk-Input Decomposition

## Intuition

The L1 source frame the real-rate / policy-rate decomposition as a **risk-input** for the loss-distribution machinery, not as a macroeconomic derivation. The discount rate `r` that flows into bond / equity / derivative valuation can be split into a **real-rate component `r_real`** (the rate after stripping inflation expectations) and an **inflation-expectation component `π_e`**, with `r ≈ r_real + π_e` (Fisher identity at first order). The **policy rate** `i_policy` is the central-bank-set short rate that anchors the front end of the yield curve and propagates into longer-dated `r` through term-structure relationships. For risk-management, what matters is which component drives the change in `r`: a policy-rate move shifts the entire curve roughly in parallel, while an inflation-expectation shock can flatten or steepen the curve. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.2/pp.278-310.

The boundary against 02 Economics is firm. **02 owns** the macroeconomic derivation: the Taylor rule (`i_policy = r* + π_target + φ_π · (π − π_target) + φ_y · ỹ`), the Fisher identity (`(1 + i) = (1 + r_real)(1 + π_e)`), the Mundell-Fleming open-economy extension, and the central-bank reaction-function depth. **11 owns** the risk-input framing: how a symbolic shock to `r_real` vs `π_e` vs `i_policy` translates to a portfolio loss via the discount-rate sensitivity channel (duration, key-rate duration, OAS shift). The 11 framing assumes the macroeconomic derivation as given; the operator looking for the derivation should open the 02 cards `[[../02_economics/ec-monetary-policy-and-inflation]]` and `[[../02_economics/ec-monetary-fiscal-policy-mechanics-l1]]`. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.2/pp.278-310.

For the risk-management vertical, the decomposition has three operational uses. (1) **Curve risk attribution**: a one-period move in the yield curve is decomposed into a parallel shift (policy-rate-driven), a level shift (inflation-expectations-driven), and a residual (term-premium-driven), each consuming a different duration / convexity exposure. (2) **Scenario design**: stress scenarios for fixed-income books are typically parameterised as shocks to `r_real` and `π_e` separately because the joint correlation in stress is non-zero. (3) **Cross-asset spillover**: equity discount rates and derivative funding rates inherit `r_real` and `π_e` shocks via correlation matrices; the risk function tracks how a fixed-income shock propagates to other asset classes. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.2/pp.278-310.

```
   L1-source real-rate / policy-rate risk-input pipeline
   ────────────────────────────────────────────────────

   +------------------+       +-------------------+
   | Policy rate      |       | Inflation         |
   | i_policy         |       | expectations π_e  |
   | (central bank)   |       | (market-implied)  |
   +--------+---------+       +---------+---------+
            |                           |
            +-------------+-------------+
                          |
                          v
              +----------------------+
              | Real rate decomp:    |
              |   r_real ≈ i - π_e   |   ← Fisher (first-order)
              |   (02 derivation)    |
              +----------+-----------+
                         |
                         v
              +----------------------+
              | Discount-rate r      |
              | for valuation        |
              +----------+-----------+
                         |
                         v
                +---------------+
                | Portfolio     |
                | value V(r,…)  |     ← duration / convexity sensitivity
                +-------+-------+
                        |
                        v
              feeds → loss distribution
              L = -ΔV(r, π_e, i_policy, …)
              (see [[rm-loss-distribution-anatomy]])
```

## Definition

Let `i` denote a nominal interest rate (typically the relevant zero-coupon yield for the position's duration), `r_real` the real rate, `π_e` the period's expected inflation, and `i_policy` the central-bank-set policy rate. The **first-order Fisher decomposition** is: **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.2/pp.278-310 + CFA L1 2022 Vol.2 pp.278-310.

```
i  ≈  r_real  +  π_e         (first-order; exact: (1+i) = (1+r_real)(1+π_e))
r_real  =  i  −  π_e          (rearranged at first order)
```

For risk-management purposes, the **portfolio loss-from-rate-move** channel is summarised via **modified duration** `D_mod`: **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.2/pp.278-310.

```
ΔV / V  ≈  −D_mod · Δi  +  1/2 · C · (Δi)²

where:
  D_mod  =  −(1/V) · ∂V/∂i        (modified duration; nominal-yield sens.)
  C      =  (1/V) · ∂²V/∂i²       (convexity)
```

The **risk-input decomposition** for `Δi` splits the rate shock by source: **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.2/pp.278-310.

```
Δi  =  Δr_real  +  Δπ_e
ΔV / V  ≈  −D_mod · (Δr_real + Δπ_e)  +  higher order

with risk-attribution: real-rate-driven loss = −D_mod · Δr_real
                       inflation-driven loss   = −D_mod · Δπ_e
```

The **policy-rate channel** maps `Δi_policy` to `Δi` (curve point at duration `T`) via the term-structure / expectations-hypothesis machinery (out of scope at L1; see 02 for derivation). **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.2/pp.278-310.

## Mathematical Reasoning

The Fisher decomposition is **first-order in the small variables** `(r_real · π_e)`: the exact identity `(1 + i) = (1 + r_real)(1 + π_e)` expands to `i = r_real + π_e + r_real · π_e`, and the cross-term `r_real · π_e` is dropped at small magnitudes. For risk-management purposes the first-order approximation is essentially always used; the second-order correction matters only for high-inflation regimes where both `r_real` and `π_e` are individually large. The 02 cards retain the full identity; the 11 notes use the linearised form. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.2/pp.278-310 + CFA L1 2022 Vol.2 pp.278-310.

The **duration channel** is the bridge from rate movement to portfolio P&L. For a single bond, `D_mod` is the negative log-derivative of price with respect to yield; for a portfolio of bonds, `D_mod` aggregates linearly weighted by market value. Crucially, duration is a **first-order Greek** — analogous to delta for equity options — and its loss-attribution role is identical: linear in `Δi`, with convexity `C` capturing the second-order term. The L1 source treat duration as given (06 Fixed Income territory for the full derivation) and use it as the lever between rate-shock decomposition and loss-distribution input. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.2/pp.278-310.

The **decomposition's risk-attribution value** comes from the fact that `Δr_real` and `Δπ_e` typically arise from different drivers: real-rate moves are driven by growth / productivity expectations and policy real-rate shifts; inflation-expectation moves are driven by supply shocks, demand shocks, and central-bank credibility. The two components have **different correlation structure with other risk factors**: a real-rate shock often correlates with equity-market shocks (real-rate-up → equity-down for high-multiple stocks); an inflation-expectation shock often correlates with commodity-market shocks (inflation-up → commodity-up). Risk-management uses this differential correlation when designing stress scenarios — a stress for `Δr_real` shocks different cross-asset positions than a stress for `Δπ_e`. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.2/pp.278-310.

The **policy-rate channel** is structurally upstream of the market real rate and inflation expectations. A central-bank policy-rate move propagates through (a) the front end of the curve directly (short rates move roughly in parallel with `Δi_policy`), (b) the long end through the expectations hypothesis (long rates move by `(1/T) · Σ E[i_policy(t)]` plus a term premium), and (c) cross-asset correlations through funding-cost and discount-rate channels. The L1 source acknowledge the channel without deriving it; the 02 monetary-policy cards develop the transmission mechanism. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.2/pp.278-310.

A subtle risk-management point: **`r_real` is not directly observable** — it's inferred from `i − π_e`, where `π_e` is itself estimated (from breakeven inflation, survey expectations, or model-implied). This means the real-rate component of any risk decomposition inherits the estimation error of the inflation-expectations proxy. Practice uses multiple `π_e` proxies (5y5y breakeven, TIPS-implied, survey-of-professional-forecasters) and reports the decomposition under each to bound the estimation uncertainty. The source flags this as a measurement-error caveat without prescribing a specific proxy. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.2/pp.278-310.

## See Also

- [../02_economics/ec-monetary-policy-and-inflation](../02_economics/ec-monetary-policy-and-inflation.md) — 02 vertical's macroeconomic derivation of the Fisher identity and inflation-expectations machinery.
- [../02_economics/ec-monetary-fiscal-policy-mechanics-l1](../02_economics/ec-monetary-fiscal-policy-mechanics-l1.md) — 02 vertical's L1 treatment of policy-rate transmission and Taylor-rule mechanics.
- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — Batch-0 card defining the loss-distribution channel that this rate-input decomposition feeds.

## Escalate to Raw When

The L1-source treatment stops at the Fisher-decomposed real-rate / policy-rate inputs + duration channel. When the operator needs the full macroeconomic derivation (Taylor-rule central-bank reaction function, Mundell-Fleming open-economy extension, expectations-hypothesis term-structure derivation, multiple-proxy inflation-expectations construction, or the differential cross-asset correlation matrix for stress design), open the 02 Economics cards above OR CFA L1 2022 Vol.2 pp.278-310 directly. **Source:** CFA L1 2022 Vol.2 pp.278-310 + CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.2/pp.278-310.
