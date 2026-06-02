---
schema_version: "cacg.v0"
id: "rm-sa-ccr-counterparty-capital"
title: "Regulatory Counterparty Capital: SA-CCR, EAD = α×EEPE, and the CVA Capital Ladder"
reading_id: "11_risk_management"
summary: "How Basel turns derivatives counterparty risk into capital: EAD via CEM→SA-CCR→IMM (IMM sets EAD=α×EEPE with α=1.4; SA-CCR sets EAD=α×(RC+PFE) with an over-collateralisation multiplier), feeding the CVA capital charge (basic BA-CVA and standardised SA-CVA; the older internal-model/VaR-style charge is removed), per Gregory Ch.13."
tags: ["risk-management", "counterparty-capital", "sa-ccr"]
citations:
  - source_id: "rm_gregory_2020_xva_challenge"
    chunk_id: "rm_gregory_2020_xva_challenge:p374:0558"
    chunk_hash: "f71df92d64638f848428d78bb297a1ca1c00146a1963ec1d1d5fc2e20e3c683e"
    page_range: [375, 375]
    quote: "The SA-CCR methodology – like the CEM – treats the EAD as a combination of the CE or replacement cost (RC) and PFE"
    edge_type: "defines"
  - source_id: "rm_gregory_2020_xva_challenge"
    chunk_id: "rm_gregory_2020_xva_challenge:p382:0570"
    chunk_hash: "55a85940bda3b2e50b365a8a3f19d225f72c1e12666c80e9a0d20a358afb046c"
    page_range: [382, 382]
    quote: "is the previously described alpha parameter with a default value of 1.4."
    edge_type: "supports"
card_hash: "fdf3f6880d50641ea5d033737bc0a9f4348a6eb96649e049f6b9c2d9aa997f0a"
---
# Regulatory Counterparty Capital: SA-CCR, EAD = α×EEPE, and the CVA Capital Ladder

## Intuition
A bank holding a derivatives portfolio faces two distinct counterparty-risk capital
demands. First, **default risk** — the chance the counterparty fails and the bank loses
its positive exposure; this needs an *exposure-at-default* (EAD) number per
counterparty. Second, **CVA risk** — the chance the *market value of CVA* swings as
credit spreads move, even with no actual default; this needs its own market-risk-style
charge. Both reduce a probabilistic exposure profile down to a single number, then
multiply by weights. The regulatory toolkit is a *ladder* of methods of increasing
sophistication: for EAD it climbs CEM → SA-CCR → IMM; for CVA the revised framework has
two methods, basic BA-CVA and standardised SA-CVA (SA-CVA the more sophisticated). The
older VaR-style internal-model (IMA-CVA) charge is the *current* method SA-CVA replaces,
not a higher rung, and was removed. The higher rungs are more risk-sensitive (and usually
save capital) but demand model approval.

```
   EAD ladder (per counterparty)    CVA-capital charge (portfolio)
   ┌──────────┐                     ┌──────────┐
   │   CEM    │  crude add-ons      │ BA-CVA   │  basic (any bank)
   ├──────────┤                     ├──────────┤
   │  SA-CCR  │  EAD=α·(RC+PFE)     │ SA-CVA   │  standardised, approval-
   ├──────────┤                     └──────────┘  gated (most sophisticated)
   │   IMM    │  EAD=α·EEPE
   └──────────┘                     (the older VaR-style internal-model
        │                             CVA charge was removed, not a rung)
        └──────────►  EAD term feeds the CVA charge
```

**Source:** Gregory (2020) Ch.13.3.1, §13.4 printed pp.343–344, 360–369 (PDF pp.356–357, 374–383).

## Definition
- **EAD, internal model method (IMM).** The risk-sensitive EAD is the *effective
  expected positive exposure* scaled by a multiplier:

      EAD = α × EEPE,        α default value = 1.4.

  EEPE is the time-average of the non-decreasing ("effective") EE profile over a
  one-year horizon; α corrects average-EPE for finite portfolio size, concentration,
  and a "bad state" of the economy (floored at 1.2 if a bank computes its own).
- **EAD, SA-CCR (standardised).** A formula replacement for IMM/CEM:

      EAD = α × (RC + PFE),       α = 1.4,

  where RC is the (netted) replacement cost and PFE is an add-on built from supervisory
  factors and duration across five asset-class hedging sets, scaled by an
  over-collateralisation **multiplier** that lowers PFE when value-minus-collateral
  (V − C) is negative (floored at 5%).
- **CVA capital charge.** The revised framework has two methods: **BA-CVA** (basic,
  required of all banks; reduced and full versions) and **SA-CVA** (standardised,
  sensitivity-based, approval-gated, with a model-risk multiplier and explicit WWR hook) —
  SA-CVA being the more sophisticated of the two. The older VaR-style internal-model
  (advanced/IMA-CVA) charge is the *current* method SA-CVA replaces, not a higher rung,
  and the IMA-CVA option was removed. All CVA-capital methods are *portfolio-level*
  single-number calculations, unlike the per-counterparty default-risk charge.

**Source:** Gregory (2020) §13.3.3, §13.3.5, §13.4.3, §13.4.5 printed pp.345–346, 351–353, 362, 369 (PDF pp.358–359, 364–366, 375, 382).

## Mathematical Reasoning
**Why EAD = α × (average) EPE.** Wilde's result shows that in the limit of an
infinitely diversified portfolio of small, uncorrelated exposures with no wrong-way
risk, the economic-capital-relevant exposure collapses to the *average EPE*. Real
portfolios violate all three idealisations, so a correction multiplier α is applied:

      EAD = α × (average EPE),     α ≥ 1,

with α grossing up for finite size and concentration. Regulators then harden "average
EPE" into **EEPE** — the average of the non-decreasing envelope EEE(t) = max over u ≤ t
of EE(u) — to stop a short-lived exposure spike from being averaged away and to capture
roll-over risk:

      EEE(t) = sup_{u ≤ t} EE(u),     EEPE = (1/T)∫₀^T EEE(t) dt ≥ EPE.

Because EEE never decreases, EEPE ≥ EPE by construction. SA-CCR reuses the same α = 1.4
on (RC + PFE) so the standardised number is calibrated *consistently* with IMM — which
has the side effect of inflating the replacement-cost component by 40%.

**Source:** Gregory (2020) §13.4.3–13.4.5 printed pp.362–369 (PDF pp.375–382).

## See Also
- [rm-exposure-profile-shapes](./rm-exposure-profile-shapes.md) — the EE/EPE profiles that EEPE and add-ons summarise.
- [rm-frtb-stressed-es-market-risk-capital](./rm-frtb-stressed-es-market-risk-capital.md) — SA-CVA is an adaptation of the FRTB standardised market-risk approach.
- [rm-basel-capital-accord-evolution](./rm-basel-capital-accord-evolution.md) — the accord lineage these CEM→SA-CCR→IMM methods sit within.
- [rm-wrong-way-risk-taxonomy](./rm-wrong-way-risk-taxonomy.md) — the general WWR that α and stressed EEPE partially capitalise.

## Escalate to Raw When
You need the worked EAD comparison examples (the six-year-swap SA-CCR add-on, the
SA-CCR-vs-IMM comparison figure, the CEM add-on Table 13.8), the SA-CCR supervisory-factor
and correlation tables, the over-collateralisation-multiplier formula, or the BA-CVA /
SA-CVA risk-weight and aggregation formulas — those numeric recipes live in the raw
text (Rule 1).

**Source:** Gregory (2020) Ch.13.3–13.4 printed pp.343–377 (PDF pp.356–390).
