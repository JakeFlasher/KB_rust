---
schema_version: "cacg.v0"
id: "fi-yield-and-price-mechanics"
title: "Bond Yield and Price Mechanics"
reading_id: "06_fixed_income_and_credit"
summary: "A bond's price is the discounted sum of its contractual cashflows; yield-to-maturity (YTM) is the single internal rate that equates the discounted-cashflow sum to the observed price. Price and YTM are inverse representations of the same information. Flat (clean) price excludes accrued; full (dirty) price includes it. Portfolio YTM is NOT the cashflow-weighted average of constituent YTMs."
tags: ["fixed-income", "yield-price"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2458:3615"
    chunk_hash: "eed3812135cf22e26f295daab0a92c836ed051f440ec86d01db40b43a44502ab"
    page_range: [2458, 2459]
    quote: "The yield-to-maturity is the rate of return on the bond to an investor given three critical assumptions: 1 The investor holds the bond to maturity"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p105:0154"
    chunk_hash: "0a62819daa964b4fe53ae85c0d28939959dbc0c6ba26c70d46415a146926c0bb"
    page_range: [105, 105]
    quote: "(DerivaGem can be used to calculate bond prices.) Bond Yield A bond’s yield is the single discount rate that, when applied to all cash flows, gives a bond price equal to its market price"
    edge_type: "supports"
card_hash: "af873214f22a7a9364711c96612ae4d8d85b4e6dc3c6587b94dd6ce142b9f2d6"
---
# Bond Yield and Price Mechanics

## Intuition

A bond's price is the discounted sum of its contractual
cashflows; its yield-to-maturity (YTM) is the single
discount rate that makes that sum equal the observed price.
Price and YTM are inverse representations of the same
information: holding cashflows fixed, a higher price implies
a lower YTM and vice versa. **Source:** CFA L1 Curriculum
(2022) Vol.5/pp.110-150.

```
price
   ^   *
   |    *  (price is convex and decreasing in yield)
   |     *
   |      *
   |       *           current quote (price, YTM)
   |        +----o
   |             *
   |              *
   |               *
   +--------------------------> yield
                   YTM
   higher YTM <-> lower price; the curve is monotone
   decreasing and convex.
```

## Definition

For a vanilla bond with coupon cashflows `(c_1, c_2, ..., c_N)`
paid at times `(t_1, t_2, ..., t_N)` and final principal
redemption `F` at `t_N = T`, the price is
`P = sum_{i=1}^{N} c_i / (1 + y / m)^(m · t_i)
 + F / (1 + y / m)^(m · T)`, where `m` is the compounding
frequency (e.g. `m = 2` for semi-annual quotes common in US
Treasury markets). **Source:** CFA L1 Curriculum
(2022) Vol.5/pp.110-130.

The yield-to-maturity is the value `y = YTM` that makes the
right-hand side equal the observed price; for vanilla bullet
bonds with strictly positive cashflows, the price equation is
monotone in `y` and has a unique admissible solution. The
solution is positive when the observed price is below the
undiscounted sum of promised cashflows. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.115-135.

The flat (clean) price excludes accrued interest since the
last coupon date; the full (dirty) price includes it. Markets
typically quote flat prices and exchange the dirty price at
settlement. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.115-130.

## Mathematical Reasoning

Discount-factor invariance: any pricing formula expressed as
a sum of cashflows times discount factors `D(t)` admits an
equivalent zero-rate representation
`D(t) = exp(-z(t) · t)` and a yield representation
`D(t) = (1 + y / m)^(-m · t)`. The two parameterizations
coincide for a flat term structure and diverge otherwise; the
zero-rate view is closed under linear combinations of bonds
while the YTM view is not. **Source:** CFA L1 Curriculum
(2022) Vol.5/pp.130-150;
Hull §4 pp.84-95.

Consequently a portfolio's YTM is NOT the cashflow-weighted
average of its constituents' YTMs in general; the
zero-rate-equivalent flat price plus a recomputed IRR is the
correct portfolio yield. The decomposition into spot, par,
and forward rates lives in
[`fi-spot-par-forward-curves.md`](./fi-spot-par-forward-curves.md#definition).
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.130-150.

The price-yield curve is decreasing, convex, and asymptotes
to zero as yield grows. The convexity of the curve gives the
second-order correction relative to the linear duration
approximation developed in
[`fi-duration-and-convexity.md`](./fi-duration-and-convexity.md#mathematical-reasoning).
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.140-150;
Hull §4 pp.84-92.

## See Also

- [`fi-bond-anatomy-and-cashflows.md`](fi-bond-anatomy-and-cashflows.md) — the cashflow stream this card prices
- [`fi-spot-par-forward-curves.md`](fi-spot-par-forward-curves.md) — three views of the discount-factor surface
- [`fi-duration-and-convexity.md`](fi-duration-and-convexity.md) — first- and second-order yield sensitivity

## Escalate to Raw When

Open CFA L1 Curriculum Vol.5 Reading 43 or Hull's Chapter 4
directly when any of the criteria below applies. **Source:**
CFA L1 Curriculum (2022) Vol.5/pp.110-150.

- The bond has non-bullet cashflows (sinking-fund schedule,
  amortizing principal, step-up coupons) and the standard
  YTM equation does not apply directly. **Source:** CFA L1
  Curriculum (2022) Vol.5/pp.110-150.
- The market quotes use a non-standard compounding convention
  (continuous, money-market, simple) that this card does not
  cover. **Source:** Hull §4 pp.84-95.
- A floating-rate note or inflation-linked instrument is
  involved; FRN yield measures (effective, discount, simple)
  fall outside this card's vanilla scope. **Source:** CFA L1
  Curriculum (2022) Vol.5/pp.130-150.
