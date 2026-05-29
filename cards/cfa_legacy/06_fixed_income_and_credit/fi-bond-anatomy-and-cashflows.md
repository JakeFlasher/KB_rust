---
schema_version: "cacg.v0"
id: "fi-bond-anatomy-and-cashflows"
title: "Fixed-Income Bond Anatomy and Cashflows"
reading_id: "06_fixed_income_and_credit"
summary: "Defines what a fixed-coupon bond IS at issuance: the cash-flow profile, indenture fields a holder reads first, and the legal/economic primitive every later FI card refines."
tags: ["fixed-income", "bond-anatomy"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2355:3444"
    chunk_hash: "7b663aea9f9a5f9dfa83ac3eeaae82e377e53166c1e5e8695d75f9c199d2cdef"
    page_range: [2355, 2356]
    quote: "1.1 Overview of a Fixed-Income Security A bond is a contractual agreement between the issuer and the bondholders."
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p097:0143"
    chunk_hash: "b93fa00bd175ccfa216e18798de1b2baa62758d42883c5496a0fd8627b1be36b"
    page_range: [97, 98]
    quote: "It covers zero rates, par yields, and yield curves, discusses bond pricing, and outlines a “bootstrap” procedure commonly used to calculate zero-coupon interest rates."
    edge_type: "supports"
  - source_id: "fi_fabozzi_2021_handbook_fixed_income_9e"
    chunk_id: "fi_fabozzi_2021_handbook_fixed_income_9e:p033:0026"
    chunk_hash: "6f6eb22516e8b6e8307e6da975d36c330ab96fbd36a5909bc6515a9111a4d9fd"
    page_range: [33, 34]
    quote: "BONDS Bonds are instruments of debt; the issuer of a bond borrows money from the bond investor."
    edge_type: "supports"
card_hash: "bb4bf929d11e142acc3e5f136614d6b085d943e168464b2ecec01952c03f48eb"
---
# Fixed-Income Bond Anatomy and Cashflows

## Intuition

A vanilla fixed-coupon bond is a contractual stream of periodic
coupon payments plus a single principal redemption at legal
maturity. The issuer raises issue proceeds at issuance and
promises the holder periodic coupons plus the principal `F` at
maturity `T`. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.5-30.

```
<!-- primitive: bond-cashflow-ladder source: _diagram_primitives.md -->
issuance                                                  maturity
   |                                                          |
   |   C       C       C       C       C       C       C+F    |
   v   v       v       v       v       v       v       v      v
   *---|---|---|---|---|---|---|---|---|---|---|---|---|---|--*
   t=0 t1  t2  t3  t4  t5  t6  t7  t8  t9 t10 t11 t12 t13  T
   issuer pays C at each coupon date; F repaid at T
```

## Definition

A fixed-coupon bond is the bundle `(F, c, freq, T, day_count,
seniority, callability, putability)` where `F` is the face
(par) value, `c` is the annual coupon rate, `freq` is coupon
frequency (typically annual or semi-annual), `T` is legal
maturity, `day_count` records the accrued-interest convention
(30/360, actual/actual, actual/360), `seniority` is the bond's
priority in the capital structure, and the optional
`callability` and `putability` fields enumerate issuer-call
and holder-put rights respectively. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.5-30.

The indenture is the legal contract that binds these fields;
covenants (affirmative, negative) constrain the issuer's
behavior over the bond's life and are part of the same
prospectus surface. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.10-25.

## Mathematical Reasoning

The pre-default cash-flow stream is `{(c / freq) · F at each
coupon date, F + (c / freq) · F at T}` for a vanilla bullet
bond paying an annual coupon rate `c` at frequency `freq` over
horizon `T`. **Source:**
CFA L1 Curriculum (2022) Vol.5/pp.15-25.

The stream is fully specified by the indenture's `(F, c, T,
freq, day_count)` fields plus the seniority and any issuer-
call / holder-put schedule. Default risk is parameterized
separately by the issuer's credit quality (see
[`fi-credit-risk-fundamentals.md`](./fi-credit-risk-fundamentals.md#definition));
this card models only the contractual stream conditional on
no default. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.25-30; Hull §4 pp.78-92.

For a holder who buys at issuance and holds to maturity with
no default and no call/put exercise, total return is the
internal rate of return that equates the price to the
discounted stream; the IRR is the bond's yield-to-maturity
under standard pricing conventions developed in
[`fi-yield-and-price-mechanics.md`](./fi-yield-and-price-mechanics.md#definition).
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.20-30.

## See Also

- [`fi-yield-and-price-mechanics.md`](fi-yield-and-price-mechanics.md) — pricing the stream and computing yield-to-maturity
- [`fi-credit-risk-fundamentals.md`](fi-credit-risk-fundamentals.md) — modeling default conditional on the contractual stream

## Escalate to Raw When

Open the prospectus or CFA L1 Curriculum Vol.5 Reading 41
directly when any of the criteria below applies. **Source:**
CFA L1 Curriculum (2022) Vol.5/pp.5-30.

- The bond has non-standard features (PIK coupons, step-up
  coupons, sinking funds, change-of-control puts, tax events).
  **Source:** CFA L1 Curriculum (2022) Vol.5/pp.10-25.
- Day-count or coupon-frequency conventions matter for
  accrued-interest calculation beyond this card's scope.
  **Source:** CFA L1 Curriculum (2022) Vol.5/pp.15-30.
- The bond is a securitized cashflow (RMBS, CMBS, ABS) whose
  underlying mortgage / receivable behavior alters the simple
  contractual stream — the securitization layer follows a
  different decomposition than the holder-issuer case.
  **Source:** CFA L1 Curriculum (2022) Vol.5/pp.181-220.
