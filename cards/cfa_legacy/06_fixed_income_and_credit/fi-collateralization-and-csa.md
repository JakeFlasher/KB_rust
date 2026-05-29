---
schema_version: "cacg.v0"
id: "fi-collateralization-and-csa"
title: "Collateralization and CSA Mechanics"
reading_id: "06_fixed_income_and_credit"
summary: "Collateralization as the mechanism that reduces counterparty exposure: CSA agreements, threshold and minimum-transfer-amount, eligible-collateral menus, haircuts, and the feedback into multi-currency-basis multi-curve discounting."
tags: ["fixed-income", "collateralization-csa"]
citations:
  - source_id: "fi_crepey_bielecki_brigo_2014_counterparty_risk_funding"
    chunk_id: "fi_crepey_bielecki_brigo_2014_counterparty_risk_funding:p088:0105"
    chunk_hash: "47d85c3e3c34e4bbbd08fa852074c164ec5f8b169260b72f9b1d6fc42cfe1eb4"
    page_range: [88, 89]
    quote: "The horizon T represents the maturity in a Credit Support Annex (CSA) regarding a “contract”, in the sense of a generic netted portfolio of OTC derivatives between two parties."
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p055:0076"
    chunk_hash: "d4164f569ae15b90fa1477b1f2529827a9d139c8c0ff3f5846c9e4f61027327a"
    page_range: [55, 56]
    quote: "The collateral is similar to the margin required by exchange clearing houses or CCPs from their members."
    edge_type: "supports"
card_hash: "eb25f41c72b37a5d8fe4cd7d9246b87ed8418c0058eb698cc731de9265ade484"
---
# Collateralization and CSA Mechanics

## Intuition

A Credit Support Annex (CSA) is the legal addendum to
an ISDA Master Agreement that governs collateral
exchange between OTC-derivative counterparties. Daily
mark-to-market plus collateral posting reduces
unsecured exposure to a small operational residual; the
counterparty-risk machinery from
[`fi-counterparty-risk-cva.md`](./fi-counterparty-risk-cva.md#mathematical-reasoning)
attenuates accordingly. CSAs also dictate which
currency or instrument can be posted as collateral,
which feeds back into multi-curve discounting choices
in
[`fi-yield-curve-construction.md`](./fi-yield-curve-construction.md#mathematical-reasoning).
**Source:** Crepey+Bielecki+Brigo (2014) §4
pp.200-300.

```
position MTM(t)
   ^
   |          *
   |        *  *  *      collateral covers most exposure
   |      *      *  *    (residual is threshold + MTA)
   |    *           *
   |   ----+----------+-----  threshold T
   |    *      *  *
   |              *      collateral repaid as MTM unwinds
   +-----------------------------------> t
   posted collateral tracks MTM minus threshold;
   netted against MTA (minimum-transfer-amount).
```

## Definition

A CSA specifies: collateral threshold `T` (uncollateralized
exposure tolerance); minimum-transfer-amount `MTA` (small
amounts not exchanged to reduce operational overhead);
eligible collateral types (cash in specified currencies,
sovereign bonds, etc.); haircut schedule for non-cash
collateral; remuneration rate for posted collateral
(typically OIS for cash, repo rate for bonds).
**Source:** Crepey+Bielecki+Brigo (2014) §4
pp.200-300.

Daily mark-to-market plus collateral exchange means the
unsecured portion of the position is the running excess
of MTM above the threshold. When threshold and MTA are
both zero, unsecured exposure is operational only
(settlement delays, model error); when threshold is
positive, unsecured exposure equals `min(MTM,
threshold)`. **Source:** Crepey+Bielecki+Brigo (2014) §4
pp.200-300.

The collateral-currency choice affects discounting:
posting USD cash on a EUR-denominated swap means the
EUR cashflows discount at the USD-funding rate plus a
cross-currency basis. CSAs with multi-currency
optionality let the cheapest-to-deliver collateral
choice drive the effective discount curve. **Source:**
Crepey+Bielecki+Brigo (2014) §4 pp.200-300.

## Mathematical Reasoning

The CVA reduction from collateralization is the
expected positive exposure (EPE) integrated only over
the residual unsecured portion, weighted by the
counterparty's default-time density:
`CVA_collateralized = (1 - R_C) · ∫_0^T D(t) · h^Q_C(t)
· S_C(t) · residual_EPE(t) · dt`,
where `h^Q_C(t) · S_C(t)` is the default-time density
`f^Q_C(t)` shared with the uncollateralized form in
[`fi-counterparty-risk-cva.md`](./fi-counterparty-risk-cva.md#mathematical-reasoning).
With perfect daily collateral exchange and zero
threshold, residual EPE collapses to the operational
delay window (typically 5-10 business days), and CVA
shrinks accordingly. **Source:** Crepey+Bielecki+Brigo
(2014) §4 pp.200-300.

The CSA's specified collateral remuneration rate
defines the appropriate discount curve for the
position's MTM. Posting cash that earns OIS aligns the
discount curve with the OIS curve from
[`fi-yield-curve-construction.md`](./fi-yield-curve-construction.md#mathematical-reasoning);
posting bonds whose repo rate differs from OIS
introduces a basis. The single-currency single-curve
view of pre-2008 pricing breaks under non-OIS
collateral. **Source:** Crepey+Bielecki+Brigo (2014)
§4 pp.200-300.

Wrong-way collateral risk arises when collateral value
correlates negatively with the counterparty's
solvency (e.g. accepting a counterparty's own equity
as collateral). The collateral framework breaks down
in these scenarios because the collateral-call falls
exactly when collateral value drops. ISDA standard
CSAs typically exclude such collateral choices
explicitly. **Source:** Crepey+Bielecki+Brigo (2014)
§4 pp.200-300.

The XVA stack from
[`fi-counterparty-risk-cva.md`](./fi-counterparty-risk-cva.md#definition)
becomes more complete when CSA terms are explicit:
CVA / DVA depend on residual-EPE; FVA depends on the
funding cost of posted vs received collateral; KVA /
MVA fold in regulatory capital and initial margin.
**Source:** Crepey+Bielecki+Brigo (2014) §4
pp.200-300.

## See Also

- [`fi-counterparty-risk-cva.md`](fi-counterparty-risk-cva.md) — CVA / DVA / FVA stack that collateralization reshapes
- [`fi-yield-curve-construction.md`](fi-yield-curve-construction.md) — multi-curve discounting that CSA-eligible collateral defines

## Escalate to Raw When

Open Crepey+Bielecki+Brigo Chapter 4 directly when any
of the criteria below applies. **Source:** Crepey+
Bielecki+Brigo (2014) §4 pp.200-300.

- Initial-margin / SIMM model-driven margin requirements
  are in scope; the CSA framework here covers variation
  margin only. **Source:** Crepey+Bielecki+Brigo
  (2014) §4 pp.200-300.
- Central-counterparty (CCP) clearing replaces bilateral
  CSAs with novated contracts; the collateral structure
  shifts to default-fund + initial-margin pool model.
  **Source:** Crepey+Bielecki+Brigo (2014) §4
  pp.200-300.
- Cheapest-to-deliver collateral optimization across
  multi-currency CSAs requires explicit cross-currency
  basis modeling beyond this card's scope. **Source:**
  Hull §11 pp.165-180.
