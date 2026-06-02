---
schema_version: "cacg.v0"
id: "fi-credit-spread-machinery"
title: "Credit-Spread Machinery"
reading_id: "06_fixed_income_and_credit"
summary: "Credit-Spread Machinery — CFA Vol.5/pp.350-380 (PDF 2987-3017) falls in alternative investments/real estate; FI lives in Vol.5 pp.1-145 (PDF 2638-2782)."
tags: ["fixed-income", "credit-spread"]
citations:
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p075:0087"
    chunk_hash: "78f1ed2e0da3e69dba615b54b207f4c50b2089b8c25eea7a6de77460dc62d985"
    page_range: [75, 75]
    quote: "They find, however, that correcting for endogeneous defaults along the lines of Geske (1977) produces more realistic credit spreads."
    edge_type: "defines"
card_hash: "f1765d7d74df60724a15ab2997dac38b71a93520e316ec3c086a889fcd9cb125"
---
# Credit-Spread Machinery

## Intuition

The credit-risk fundamentals
[`fi-credit-risk-fundamentals.md`](./fi-credit-risk-fundamentals.md#mathematical-reasoning)
state the spread as expected loss plus risk premium. The
quantitative extension parameterizes default through a
hazard rate `h(t)` — the conditional default rate per
unit time — and recovers the credit-triangle relation
`s ≈ h · (1 - R)` as a continuous-time first-order
approximation under specific assumptions. **Source:**
Lando (2004) §2 pp.20-60.

```
hazard rate h(t)
   ^
   |     *
   |   *   *      *
   |  *     *    * *      h(t) flat -> exponential
   |          * *   *     survival; h(t) increasing
   |               *      -> hazard term structure
   +----------------------> t
   survival probability:
   S(t) = exp(- integral_0^t h(s) ds)
```

## Definition

The hazard rate `h(t)` is the conditional default rate at
time `t` given survival to `t`:
`h(t) · dt ≈ Pr(default in [t, t+dt] | survives to t)`.
Survival probability is
`S(t) = exp(-∫_0^t h(s) · ds)`. For a flat hazard rate
`h`, survival is exponential `S(t) = exp(-h · t)`.
**Source:** Lando (2004) §2 pp.20-60.

The risk-neutral hazard rate `h^Q(t)` differs from the
physical hazard rate by the credit risk premium; market
spreads are driven by `h^Q`, not the historical default
rate. The wedge between physical and risk-neutral hazard
absorbs default-rate variability and recovery uncertainty
premia. **Source:** Lando (2004) §2 pp.20-60.

The recovery rate `R` parameterizes the loss-given-default
under one of two conventions: recovery of face value (RFV)
pays `R · F` at default, while recovery of market value
(RMV) loses fraction `(1 - R)` of the pre-default market
value. Under RMV with constant `h^Q` and `R`, the
risk-neutral zero-coupon price admits the closed form
`P_risky = exp(-(r + h^Q · (1 - R)) · T)` for tenor `T`.
Under RFV the exact price is the survival-discounted face
plus integrated discounted recovery on default,
`P_risky = S(T) · exp(-r · T) + R · ∫_0^T h^Q · S(t) · exp(-r · t) · dt`
with `S(t) = exp(-h^Q · t)`; the closed form is then a
short-tenor approximation (the credit-triangle limit
recovered in the next paragraph). **Source:** Lando (2004)
§2 pp.20-60.

## Mathematical Reasoning

Equating risk-neutral risky price to a Treasury-discounted
formulation yields the credit-triangle approximation
`s ≈ h^Q · (1 - R)` for a flat hazard, flat recovery, and
short tenor. The approximation under-estimates observed
spreads when default-rate or recovery variability is
priced because those premia are absorbed into the wedge
between `h^Q` and `h^P` (the physical hazard). **Source:**
Lando (2004) §2 pp.20-60;
CFA L1 Curriculum (2022) Vol.5/pp.350-380.

Term-structure of credit spreads emerges when `h^Q(t)` is
not flat: an upward-sloping hazard implies an
upward-sloping credit spread. Empirically the credit
spread term structure for investment-grade issuers
upward-slopes; for high-yield issuers it can be hump-
shaped or downward-sloping near distress (the "credit-
spread inversion" phenomenon). **Source:** Lando
(2004) §2 pp.20-60.

The credit-triangle is the quantitative bridge between
the rating-agency overview of
[`fi-credit-risk-fundamentals.md`](./fi-credit-risk-fundamentals.md#definition)
and the convertible-bond credit-spread machinery in
[`../08_convertible_bonds/cb-credit-spread-machinery.md`](../08_convertible_bonds/cb-credit-spread-machinery.md#mathematical-reasoning),
where equity-coupled hazard adds dependence between
default and the issuer's equity price. **Source:** Lando
(2004) §2 pp.20-60.

## See Also

- [`fi-credit-risk-fundamentals.md`](fi-credit-risk-fundamentals.md) — qualitative credit-risk primitives
- [`../08_convertible_bonds/cb-credit-spread-machinery.md`](../08_convertible_bonds/cb-credit-spread-machinery.md) — equity-coupled hazard for convertible-bond credit modeling

## Escalate to Raw When

Open Lando Chapter 2 directly when any of the criteria
below applies. **Source:** Lando (2004) §2 pp.20-60.

- Calibration of `h^Q(t)` from observed CDS quotes or
  bond prices is required; this card frames the model
  but does not develop the calibration loop. **Source:**
  Lando (2004) §2 pp.20-60.
- Doubly-stochastic intensity (Cox-process) extensions
  are in scope; the deterministic-`h(t)` view of this
  card breaks. **Source:** Lando (2004) §2 pp.20-60.
- Equity-coupled hazard for convertible-bond pricing
  applies; that lives in the 08 specialty. **Source:**
  Lando (2004) §2 pp.20-60.
