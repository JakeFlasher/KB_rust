---
schema_version: "cacg.v0"
id: "fi-floaters-and-reset-mechanics"
title: "Floaters and Reset Mechanics"
reading_id: "06_fixed_income_and_credit"
summary: "Floaters and Reset Mechanics — CFA Vol.5/pp.130-150 (PDF 2767-2787) is R44 Fundamentals of Credit Analysis content (still in FI volume); no FRN/floater material — floater content actually lives in R43 (~PDF 2659+, Vol.5 pp.~22-55).; Hull pp.140-160 (PDF 140-160) is Ch.5 Foreign Currency Futures / Ch.6 Interest"
tags: ["fixed-income", "floaters-reset"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2435:3579"
    chunk_hash: "5196001aeb447d6b9be0b18f59841d4ecaa68c83641314b8f376993d438466b5"
    page_range: [2435, 2436]
    quote: "investors who hold these floaters participate partially in movements of the 10-year constant maturity swap rate."
    edge_type: "defines"
card_hash: "e836ad87f698ec04d016470ac6ddf1271f752c910d42dc9d2e7ed9b02db2d933"
---
# Floaters and Reset Mechanics

## Intuition

A floating-rate note (FRN) pays a coupon that resets
periodically against a reference rate (e.g. SOFR, EURIBOR)
plus a fixed quoted margin. Between reset dates the FRN
behaves like a short-tenor instrument; at reset the
coupon adjusts so the FRN's price reverts toward par.
This makes FRNs less sensitive to yield-curve shifts than
fixed-rate bonds of the same final maturity. **Source:**
CFA L1 Curriculum (2022) Vol.5/pp.130-150.

```
coupon
   ^
   |       reset at t1                reset at t3
   |         |                          |
   |    *    +    *      *      *      +    *
   |   * *   |   * *    * *    * *     |   * *
   |  *  *   |  *  *    *  *    *  *   |  *  *
   |---+--+--+--+--+--+--+--+--+--+--+-+--+--+--> t
   t0  c1   t1  c2 t2  c3 t3  c4 t4  c5    c6
   between resets the next coupon is fixed;
   at reset the coupon updates to (reference + margin).
```

## Definition

A FRN is the bundle `(F, ref, margin, freq, T,
day_count, reset_schedule)` where the coupon at period
`i` is `c_i = (ref(t_{i-1}) + margin) · F · day_count /
basis`. The reset rule fixes `c_i` at the start of the
period using the observed reference rate; settlement
happens at the end of the period. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.130-150.

The quoted margin compensates for issuer credit risk plus
liquidity premium relative to the reference benchmark.
The discount margin (effective margin) is the observed
margin that, applied to the reference curve, prices the
FRN at par; for a floater trading above par, the
discount margin is below the quoted margin and vice
versa. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.135-150.

## Mathematical Reasoning

Between reset dates the FRN's price is a function of the
fixed next-coupon plus the present value of the
remaining unfixed periods. At each reset date the unfixed
periods reprice to the new reference rate, so the FRN's
price reverts toward `F + (remaining discount margin
adjustment)`. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.130-150; Hull §7 pp.140-160.

The interest-rate sensitivity (modified duration) is
small between resets and resets to a low value at each
reset date; the empirical duration of an FRN over a
multi-period horizon is bounded by the reset frequency,
not the final-maturity tenor. The duration analysis from
[`fi-duration-and-convexity.md`](./fi-duration-and-convexity.md#mathematical-reasoning)
applies only to the period before the next reset.
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.135-150.

Credit-sensitivity (spread duration) of an FRN is roughly
the final-maturity tenor: the discount margin captures
issuer-credit risk and reprices over the full life of
the bond, not just the reset period. So an FRN can be
duration-light and spread-duration-heavy simultaneously,
which has portfolio-construction implications: a fund
that wants credit exposure without rate exposure can
hold FRNs from a credit-diverse issuer set. **Source:**
CFA L1 Curriculum (2022) Vol.5/pp.135-150.

## See Also

- [`fi-bond-anatomy-and-cashflows.md`](fi-bond-anatomy-and-cashflows.md) — the fixed-coupon counterpart
- [`fi-yield-and-price-mechanics.md`](fi-yield-and-price-mechanics.md) — fixed-rate yield measures vs FRN discount margin

## Escalate to Raw When

Open CFA L1 Curriculum Vol.5 Reading 43 or Hull Chapter 7
directly when any of the criteria below applies.
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.130-150;
Hull §7 pp.140-160.

- The FRN has caps or floors on the floating coupon and
  the option value is non-trivial (this card treats the
  uncapped uncollared case). **Source:** Hull §7
  pp.140-160.
- The reference rate's transition from LIBOR to SOFR
  introduces a fallback / spread-adjustment that
  materially alters the discount-margin calculation.
  **Source:** CFA L1 Curriculum (2022) Vol.5/pp.130-150.
- Inverse floaters or leveraged floaters are in scope;
  this card covers vanilla FRNs only. **Source:** Hull
  §7 pp.140-160.
