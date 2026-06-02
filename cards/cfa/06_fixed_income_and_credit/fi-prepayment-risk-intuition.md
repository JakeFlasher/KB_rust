---
schema_version: "cacg.v0"
id: "fi-prepayment-risk-intuition"
title: "Prepayment Risk (Mortgage Securitization)"
reading_id: "06_fixed_income_and_credit"
summary: "MBS holders are effectively short a prepayment option: when rates fall, borrowers refinance and principal arrives early to be reinvested at lower rates (contraction risk); when rates rise, prepayments slow and effective maturity extends (extension risk). The PSA convention parameterises the CPR ramp; pass-through and CMO tranches reshape the prepayment exposure asymmetrically."
tags: ["fixed-income", "prepayment-risk"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2543:3744"
    chunk_hash: "19259a48edc3300e0697213b245ca7856b8af9206062797596ca2d96b4908be0"
    page_range: [2543, 2544]
    quote: "extension risk is the risk that when interest rates rise, prepayments will be lower than forecasted because homeowners are reluctant to give up the ben"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2544:3746"
    chunk_hash: "a7218b917fcc305cdd0012c1e46c9c547951337cad64ac68986584bb9354e025"
    page_range: [2544, 2545]
    quote: "Public Securities Association (PSA) prepayment benchmark, which is produced by the Securities Industry"
    edge_type: "supports"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p202:0305"
    chunk_hash: "adf0f1300780a0ebd49fd939d0f911629838ab842ac66026514d60a304dd077e"
    page_range: [202, 203]
    quote: "MBS investors did face uncertainty about mortgage prepayments. Prepayments tend to be greatest when interest rates are low"
    edge_type: "supports"
card_hash: "b33b5683b557197289a7003e7ad8b4be67c147053ee6ab91d557929632f0dbb3"
---
# Prepayment Risk (Mortgage Securitization)

## Intuition

Mortgage borrowers can prepay their loan early —
refinancing into a lower-rate mortgage when rates fall,
selling the home, or paying down faster than required.
For an MBS holder, prepayments arrive as unexpected
principal payments that must be reinvested at the
prevailing (lower) rate. The MBS holder is effectively
short a prepayment option: prepayments accelerate when
they are most costly to the holder. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.190-220.

```
expected MBS cashflows
   ^   |
   |   |  scheduled principal + interest
   |   |  +  prepayment principal (rate-driven)
   |   |
   |   v   |     |     |     |     |     |
   |  +---+----+----+----+----+----+----+
   |  |   |    |    |    |    |    |    |
   +--+---+----+----+----+----+----+----+--> t
      t0
      rate falls -> prepayments rise (contraction)
      rate rises -> prepayments fall (extension)
```

## Definition

The conditional prepayment rate (CPR) is the annualized
fraction of the remaining mortgage pool that prepays in
a given period. The single monthly mortality (SMM) is
the monthly equivalent: `SMM = 1 - (1 - CPR)^(1/12)`.
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.190-220.

The PSA (Public Securities Association) convention
defines a benchmark prepayment ramp: 100 PSA means CPR
ramps from 0% at month 0 to 6% by month 30 and stays at
6% thereafter. Empirical pools are quoted as multiples
of PSA (e.g. "150 PSA" means 1.5× the standard ramp).
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.190-220.

Extension risk is the risk that prepayments are slower
than expected, so the MBS effective maturity extends
when rates have risen. Contraction risk is the symmetric
risk that prepayments accelerate when rates fall,
shortening the MBS effective maturity. **Source:** CFA
L1 Curriculum (2022) Vol.5/pp.190-220.

## Mathematical Reasoning

The MBS holder's effective duration combines the bond's
own price-yield response with the prepayment-driven
cashflow re-weighting. When rates fall, prepayments
accelerate, the holder receives principal early at the
old high coupon, and reinvests at the new low rate; the
combined effect can produce a duration shorter than the
maturity-aligned vanilla bond. The price-yield curve
exhibits negative convexity near at-the-money par
because the cashflow acceleration absorbs part of the
upside. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.190-220.

The structural similarity to a callable bond's
[`fi-callable-and-putable-bonds.md`](./fi-callable-and-putable-bonds.md#mathematical-reasoning)
truncated upside is intentional: an MBS is functionally
a portfolio of callable instruments where each
borrower's individual prepayment is a call exercise.
The aggregation across thousands of borrowers smooths
the prepayment distribution but does not eliminate the
embedded-option asymmetry. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.190-220.

Tranches in the securitization waterfall of
[`fi-securitization-fundamentals.md`](./fi-securitization-fundamentals.md#mathematical-reasoning)
absorb prepayment risk asymmetrically. PAC (planned-
amortization-class) tranches receive a smoothed
principal stream within a target prepayment-rate band;
support tranches absorb the prepayment variability
above and below that band. So PAC tranches have lower
effective convexity than support tranches. **Source:**
CFA L1 Curriculum (2022) Vol.5/pp.190-220.

## See Also

- [`fi-securitization-fundamentals.md`](fi-securitization-fundamentals.md) — pool / tranche structure that prepayment risk reshapes
- [`fi-callable-and-putable-bonds.md`](fi-callable-and-putable-bonds.md) — single-issuer call analog of pool prepayment

## Escalate to Raw When

Open CFA L1 Curriculum Vol.5 Reading 44 directly when
any of the criteria below applies. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.190-220.

- A specific prepayment model (Andrew Davidson's PSA
  variants, Markov-state models) needs calibration.
  **Source:** CFA L1 Curriculum (2022) Vol.5/pp.190-220.
- OAS / effective-duration computation across rate
  scenarios is required; this card frames the risk
  qualitatively, not OAS calibration. **Source:** Hull
  §8 pp.183-200.
- Burnout / lock-in / refinance-incentive nonlinearities
  matter for the specific deal; aggregate-pool
  approximations break down. **Source:** CFA L1
  Curriculum (2022) Vol.5/pp.190-220.
