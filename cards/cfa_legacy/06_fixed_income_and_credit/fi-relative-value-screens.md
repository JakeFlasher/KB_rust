---
schema_version: "cacg.v0"
id: "fi-relative-value-screens"
title: "Relative-Value Screens"
reading_id: "06_fixed_income_and_credit"
summary: "Relative-value screens compare a bond's spread vector (Z-spread, OAS, asset-swap spread) against a tightly-defined peer cohort and flag cheap (z < -1) and rich (z > +1) outliers. The OAS-vs-Z-spread comparison decomposes credit richness from embedded-option richness; the bond-CDS basis surfaces persistent cash-vs-derivative mispricing."
tags: ["fixed-income", "relative-value"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2744:4100"
    chunk_hash: "50486c3114e2d11d2c83029b737aed44950bcbc642062d7a1eaa79c45b9c73c0"
    page_range: [2744, 2745]
    quote: "OAS is option-adjusted spread, which incorporates the value of the embedded call option in certain corporate bonds that issuers have the right to exercise before maturity"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2744:4099"
    chunk_hash: "ac88ae32486256359bc6594f560190c434e79acc4373dc1be28c6e76cd2a36ec"
    page_range: [2744, 2744]
    quote: "investors anticipate credit measures will improve due to rising corporate cash flow, thus reducing the risk of default"
    edge_type: "supports"
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p191:0233"
    chunk_hash: "f0a9da9f2ddd7e28ef270b945b9f3a10a73cf8d5c2e4f79d29a0657ae8f45629"
    page_range: [191, 192]
    quote: "quotes on the floating spread are sometimes available in the market, for example, through the asset swap market"
    edge_type: "supports"
card_hash: "d37e037c2d649cc2d365db75af311faf3b0413a69e67f19330d9c5676553fbfb"
---
# Relative-Value Screens

## Intuition

Every cash bond can be summarized by a vector of yield
components: rate (zero curve), credit (Z-spread), option
(Z minus OAS), and liquidity (residual). A relative-
value screen compares one bond's component vector to a
peer group of similar bonds (same issuer / same tenor
band / same rating) and flags the outliers — bonds whose
components diverge from the peer median. The cheap
bonds are buy candidates; the rich bonds are sell
candidates. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.350-380.

```
peer-group OAS distribution
   ^
   |        median           rich
   |        |                |
   |    *   |   *   *   *   *
   |  *   * | *   *   *   * |  *
   |   *   *|*  *  *  *  *  | *
   |        |                |
   +--+-----+----------------+----> OAS
   cheap   median            rich
   z-score below 0           z-score above 0
```

## Definition

Z-score relative to peer median is the standard cheapness
metric:
`z = (s_{bond} - s_{peer median}) / s_{peer std}`. Bonds
with `z < -1` are flagged cheap; `z > 1` flagged rich.
The peer set must be defined sharply (issuer cohort,
sector, rating band, tenor band) for the metric to be
meaningful. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.350-380.

Asset-swap spread isolates pure credit / liquidity by
swapping the bond's fixed coupon for a floating reference
+ spread. A bond's asset-swap spread should equal its
CDS-implied spread plus a small basis; persistent
deviation indicates relative-value opportunity in the
cash-or-CDS direction. **Source:** Lando (2004) §4
pp.150-180.

Curve-relative basis: a bond's yield can be compared to
its issuer's own yield curve (built from the issuer's
other outstanding bonds). A bond whose yield is
materially above the issuer's curve at the same tenor
is cheap relative to the issuer's other paper. **Source:**
CFA L1 Curriculum (2022) Vol.5/pp.350-380.

## Mathematical Reasoning

The z-score normalizes for peer-group variance, so a
two-sigma cheapness is comparable across sectors with
different absolute spread levels. The metric assumes
peer-group spreads are roughly Gaussian, which is
approximately true for investment-grade bonds and
breaks down for high-yield where the right tail
fattens during credit cycles. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.350-380.

The OAS-vs-Z-spread comparison from
[`fi-oas-and-effective-duration.md`](./fi-oas-and-effective-duration.md#mathematical-reasoning)
is one input to the screen: a bond whose OAS is in
line with peers but whose Z-spread is rich means the
embedded-option value is rich, not the credit. The two
metrics decompose differently and the relative-value
screen surfaces the decomposition. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.350-380.

The credit-spread machinery from
[`fi-credit-spread-machinery.md`](./fi-credit-spread-machinery.md#mathematical-reasoning)
provides the spread-versus-CDS basis check: cash bonds
trade rich to CDS in stressed markets when CDS demand
exceeds bond demand; bonds trade cheap to CDS when bond
holders bear funding costs CDS sellers do not. The
basis is observable and the screen surfaces persistent
deviations. **Source:** Lando (2004) §4 pp.150-180.

The convertible-bond relative-value screens at
[`../08_convertible_bonds/cb-relative-value-screens.md`](../08_convertible_bonds/cb-relative-value-screens.md#definition)
extend the same z-score / peer-group mechanics into
the convertible-specific dimensions (adjusted
conversion premium, IV-RV gap, parity-spread).
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.350-380.

## See Also

- [`fi-oas-and-effective-duration.md`](fi-oas-and-effective-duration.md) — OAS as one of the screen's inputs
- [`fi-credit-spread-machinery.md`](fi-credit-spread-machinery.md) — spread vs CDS basis check
- [`../08_convertible_bonds/cb-relative-value-screens.md`](../08_convertible_bonds/cb-relative-value-screens.md) — convertible-specific relative-value extension

## Escalate to Raw When

Open CFA L1 Curriculum Vol.5 Reading 46 or Lando
Chapter 4 directly when any of the criteria below
applies. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.350-380; Lando (2004) §4 pp.150-180.

- A factor-model attribution (carry, slide, curvature
  factors) is required to decompose the screen's
  signal. **Source:** CFA L1 Curriculum (2022)
  Vol.5/pp.350-380.
- Machine-learning peer-group construction is in
  scope; this card uses sector / rating / tenor as
  the peer-defining variables. **Source:** Lando
  (2004) §4 pp.150-180.
- The screen's signal is for curve-trade construction
  (steepeners, flatteners) rather than single-name
  buy / sell decisions; curve trades require additional
  hedging logic. **Source:** CFA L1 Curriculum
  (2022) Vol.5/pp.350-380.
