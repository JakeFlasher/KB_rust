---
schema_version: "cacg.v0"
id: "fi-counterparty-risk-cva"
title: "Counterparty Risk and CVA / DVA / FVA"
reading_id: "06_fixed_income_and_credit"
summary: "Counterparty credit risk and the X-value adjustments (CVA/DVA/FVA) that price it: CVA as expected loss integrated over EPE under counterparty default-time density; DVA as the symmetric own-default benefit; FVA as the funding-cost asymmetry."
tags: ["fixed-income", "counterparty-risk"]
citations:
  - source_id: "fi_crepey_bielecki_brigo_2014_counterparty_risk_funding"
    chunk_id: "fi_crepey_bielecki_brigo_2014_counterparty_risk_funding:p086:0104"
    chunk_hash: "f2d8028fe8cd4514022e19f8b3cff8e14fb783f410648b135ffb9d297431fb6e"
    page_range: [86, 88]
    quote: "This issue (in the case of bilateral counterparty risk in particular) will be added to the discussion in the next chapter."
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p561:0822"
    chunk_hash: "a826b5db2a7dff1a40ec14cf1d543804cb087db3a486b86b22182a885396c943"
    page_range: [561, 562]
    quote: "In this chapter we consider another important risk for financial institutions: credit risk."
    edge_type: "supports"
card_hash: "b3a03d55b48685a474eee768f0d2be3aba405bc506e0b53945edeb5a436ff53d"
---
# Counterparty Risk and CVA / DVA / FVA

## Intuition

When two parties trade an OTC derivative, each carries
the credit risk that the other defaults before settling.
The Credit Valuation Adjustment (CVA) is the market
price of that risk: the discount applied to a derivative
position to reflect potential loss from counterparty
default. DVA mirrors CVA from the counterparty's
perspective. FVA captures funding-cost asymmetries
between assets and liabilities. **Source:** Crepey+
Bielecki+Brigo (2014) §3 pp.60-150.

```
default-free derivative value V*
    | <- CVA (subtract counterparty default risk)
    v
    risky-counterparty value V = V* - CVA + DVA - FVA + ...
    ^
    | <- DVA (add own default benefit)
    | <- FVA (subtract funding asymmetry premium)
    |
    full XVA stack reconciles risk-neutral pricing with
    real-world bilateral risk premia.
```

## Definition

CVA = expected loss to the bank from counterparty
default, integrated over the future expected positive
exposure (EPE) under the counterparty's default-time
density:
`CVA = (1 - R_C) · ∫_0^T D(t) · h^Q_C(t) · S_C(t) ·
EPE(t) · dt` where `R_C` is counterparty recovery,
`h^Q_C(t)` is the counterparty's risk-neutral hazard
rate, and `S_C(t) = exp(-∫_0^t h^Q_C(s) · ds)` is the
counterparty's risk-neutral survival probability. The
product `h^Q_C(t) · S_C(t)` is the default-time density
`f^Q_C(t)`, integrated against the discounted exposure
profile. **Source:** Crepey+Bielecki+Brigo (2014) §3
pp.60-150.

DVA = symmetric adjustment from the counterparty's
perspective: the bank's own default reduces the
counterparty's expected losses, accruing a benefit to
the bank's position. DVA's accounting status is
contested (it represents a "win-on-own-default" that
many regulators dislike). **Source:** Crepey+Bielecki+
Brigo (2014) §3 pp.60-150.

FVA = funding valuation adjustment for the wedge
between an OTC position's funding cost and the
risk-neutral discount rate. Posting collateral on a
swap-asset position requires unsecured funding;
receiving collateral on a swap-liability position
provides cheap funding. The asymmetry produces a non-
zero FVA. **Source:** Crepey+Bielecki+Brigo (2014)
§3 pp.60-150.

## Mathematical Reasoning

The exposure profile EPE(t) is the unconditional expected
positive mark-to-market of the derivative position at
time `t`: `EPE(t) = E^Q[max(MTM(t), 0)]` under the
risk-neutral measure. The standard CVA formula assumes
exposure and counterparty default are independent;
wrong-way risk is the violation case where they are
positively correlated. For a vanilla interest-rate swap,
EPE(t) is hump-shaped: zero at initiation, rising as the
swap accrues in-the-money potential, then declining as
remaining notional shrinks. **Source:** Hull §20
pp.555-575;
Crepey+Bielecki+Brigo (2014) §3 pp.60-150.

CVA scales the loss-given-default `(1 - R_C)` by the
default-time density
`f^Q_C(t) = h^Q_C(t) · S_C(t)` — sourced from CDS-curve
calibration via
[`fi-cds-basics.md`](./fi-cds-basics.md#mathematical-reasoning) —
and the exposure profile EPE(t). The integral aggregates
expected-loss contributions over the position's life.
The same `h^Q · S` weighting appears in the protection
leg of a CDS contract; CVA reuses this credit-pricing
primitive against an exposure that varies with time.
**Source:** Crepey+Bielecki+Brigo (2014) §3 pp.60-150.

Collateralization (CSA agreements) reduces EPE: when
collateral is posted to a margin threshold, the
unsecured exposure shrinks. Perfect daily collateral
exchange with zero threshold reduces CVA toward zero.
The collateral framework lives in
[`fi-collateralization-and-csa.md`](./fi-collateralization-and-csa.md#definition).
**Source:** Crepey+Bielecki+Brigo (2014) §3
pp.60-150.

Wrong-way risk arises when EPE and counterparty default
probability are positively correlated (e.g. a CDS sold
on a counterparty whose hazard is correlated with the
reference entity's hazard). Generic CVA assumes
independence; wrong-way risk requires a bivariate
default model that materially elevates CVA. **Source:**
Crepey+Bielecki+Brigo (2014) §3 pp.60-150.

## See Also

- [`fi-cds-basics.md`](fi-cds-basics.md) — CDS-implied hazard rates feeding CVA
- [`fi-credit-spread-machinery.md`](fi-credit-spread-machinery.md) — hazard-rate decomposition shared with CVA
- [`fi-collateralization-and-csa.md`](fi-collateralization-and-csa.md) — collateral mechanics that reduce CVA

## Escalate to Raw When

Open Crepey+Bielecki+Brigo Chapter 3 directly when any
of the criteria below applies. **Source:** Crepey+
Bielecki+Brigo (2014) §3 pp.60-150.

- Wrong-way risk is material and a bivariate default
  model is needed; this card uses independence.
  **Source:** Crepey+Bielecki+Brigo (2014) §3
  pp.60-150.
- Regulatory CVA capital (Basel III SA-CVA / BA-CVA)
  is in scope; market vs accounting CVA differ
  materially. **Source:** Crepey+Bielecki+Brigo
  (2014) §3 pp.60-150.
- Margin Valuation Adjustment (MVA) for initial-margin
  funding cost or Capital Valuation Adjustment (KVA)
  is in scope; the XVA stack extends beyond CVA / DVA
  / FVA. **Source:** Crepey+Bielecki+Brigo (2014) §3
  pp.60-150.
