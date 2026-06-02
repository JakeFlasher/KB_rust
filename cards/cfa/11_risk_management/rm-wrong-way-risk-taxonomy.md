---
schema_version: "cacg.v0"
id: "rm-wrong-way-risk-taxonomy"
title: "Wrong-Way and Right-Way Risk in CVA"
reading_id: "11_risk_management"
summary: "Wrong-way risk is unfavourable dependence between exposure (EPE) and counterparty credit quality — exposure peaks just as the counterparty is most likely to default; Gregory Ch.17.6 splits it into general (macro) vs specific (structural) WWR with right-way risk as the favourable mirror."
tags: ["risk-management", "wrong-way-risk", "cva"]
citations:
  - source_id: "rm_gregory_2020_xva_challenge"
    chunk_id: "rm_gregory_2020_xva_challenge:p523:0778"
    chunk_hash: "600c5b99ae2965cdd4aad5588f6fee6c642ca4f6774c6973a88cf364a21b675c"
    page_range: [523, 523]
    quote: "WWR is the phrase generally used to indicate an unfavourable dependence between exposure (EPE) and counterparty credit quality: the exposure is high when the counterparty is more likely to default, and vice versa."
    edge_type: "defines"
  - source_id: "rm_gregory_2020_xva_challenge"
    chunk_id: "rm_gregory_2020_xva_challenge:p523:0778"
    chunk_hash: "600c5b99ae2965cdd4aad5588f6fee6c642ca4f6774c6973a88cf364a21b675c"
    page_range: [523, 523]
    quote: "In contrast, ‘right-way’ risk can also exist in cases where the dependence between exposure and credit quality is a favourable one."
    edge_type: "supports"
card_hash: "0ef63842241abc4493ef41ad981b2fc8f8ac82c2a4082ff3e66e8f4475ea82a2"
---
# Wrong-Way and Right-Way Risk in CVA

## Intuition
The standard CVA computation quietly assumes the three ingredients — exposure (EPE),
default probability (PD), and loss-given-default (LGD) — are independent. Wrong-way risk
(WWR) is what breaks that assumption in the dangerous direction: the trade is worth most
to you *precisely when* your counterparty is most likely to fail to pay. The exposure and
the default are positively coupled, so the loss you actually suffer is worse than the
naive product EPE × PD × LGD suggests. Right-way risk is the benign mirror — exposure
shrinks just as default looms, so realised losses are smaller. WWR is hard to spot,
model, and hedge because the coupling often hides in subtle macro-economic or structural
linkages that historical data may never have shown.

```
   value to you
   ^
   |   exposure rising  ___
   |                __--   \
   |            __--        \   <-- counterparty default likelihood
   |        __--             \      rising at the SAME time = WWR
   +-----------------------------> stress / time
        (right-way risk = the two move in opposite directions)
```

**Source:** Gregory (2020) Ch.17.6.1 printed p.514 (PDF p.523).

## Definition
- **Wrong-way risk (WWR).** Unfavourable dependence between exposure (EPE) and
  counterparty credit quality — exposure is high exactly when the counterparty is more
  likely to default. It inflates CVA (and can affect DVA and other xVA via collateral
  and funding linkages).
- **Right-way risk.** Favourable dependence — exposure falls as credit quality
  deteriorates; this *reduces* CVA.
- **General WWR.** Driven by macro-economic relationships; potentially detectable in
  historical data and capable of being incorporated into pricing models.
- **Specific WWR.** Driven by structural / causal linkages between the exposure (or
  margin) and the counterparty's own default; often not present in historical or
  market-implied data, and best addressed qualitatively (e.g. via stress testing) and
  generally avoided.
- **Canonical examples.** Buying a put on a bank's stock from another bank; an FX
  forward or cross-currency swap where you *pay* a sovereign's local currency (and
  receive the foreign currency), so your exposure grows as that local currency weakens
  alongside the sovereign's deterioration; buying CDS protection on an entity strongly
  related to the protection seller (e.g. a bank selling protection on its own
  sovereign). Equity calls, hedging commodity producers, and selling CDS on a related
  name are the corresponding right-way cases.

**Source:** Gregory (2020) Ch.17.6.1 printed pp.514–516 (PDF pp.523–525).

## Mathematical Reasoning
The unconditional CVA writes loss as an integral over time of EPE weighted by marginal
default probability and LGD, *assuming independence*. WWR is introduced conceptually by
replacing the unconditional EPE with the exposure **conditional on the counterparty
defaulting** at that time:

    EPE(t)  ⟶  EPE( t | t = τ_C ),

where τ_C is the counterparty default time. Under WWR this conditional EPE exceeds the
unconditional one, so CVA rises; under right-way risk it is smaller, so CVA falls.
Qualitatively, a practitioner can bound WWR by reasoning about *how much larger* the
conditional exposure is than the unconditional one, without committing to a specific
dependence parameter. Regulation captures *general* WWR only crudely — through the
requirement to use stressed exposure data and a conservative alpha factor — because
*specific* WWR is, by construction, not visible in the macro relationships those tools
rely on.

**Source:** Gregory (2020) Ch.17.6.2 printed p.516 (PDF p.525).

## See Also
- [rm-portfolio-xva-aggregation](./rm-portfolio-xva-aggregation.md) — the CVA/xVA framework whose independence assumption WWR breaks.
- [rm-exposure-profile-shapes](./rm-exposure-profile-shapes.md) — the EPE profiles that WWR conditions on default.
- [rm-sa-ccr-counterparty-capital](./rm-sa-ccr-counterparty-capital.md) — where the stressed-data and alpha-factor capitalisation of general WWR lives.
- [rm-stress-testing](./rm-stress-testing.md) — the qualitative tool of choice for specific WWR.

## Escalate to Raw When
You need the worked WWR multiplier example (the figure showing how positive vs negative
exposure-default correlation scales EPE up or down), the conditional-EPE
single-correlation formula in Appendix 17F, or the sovereign-FX residual-value table by
rating — those numeric recipes live in the raw text (Rule 1).

**Source:** Gregory (2020) Ch.17.6 printed pp.514–526 (PDF pp.522–535).
