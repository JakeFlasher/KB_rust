---
schema_version: "cacg.v0"
id: "fi-duration-and-convexity"
title: "Duration and Convexity"
reading_id: "06_fixed_income_and_credit"
summary: "Duration and Convexity — CFA Vol.5/pp.250-300 (PDF 2887-2937) is in derivatives readings (R47+); FI duration content lives in Vol.5/R43 (~pp.5-50)."
tags: ["fixed-income", "duration-convexity"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2670:3993"
    chunk_hash: "8c420ac6454efbc11f1b708cfb3b3f2ccce698f5ccabda75d07d457621d22078"
    page_range: [2670, 2671]
    quote: "An advantage of the second approach is that callable bonds, putable bonds, and floating-rate notes can be included in the weighted average using the effective durations for these securities."
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p097:0143"
    chunk_hash: "b93fa00bd175ccfa216e18798de1b2baa62758d42883c5496a0fd8627b1be36b"
    page_range: [97, 98]
    quote: "Finally, it explains the use of duration and convexity measures to determine the sensitivity of bond prices to interest rate changes."
    edge_type: "supports"
card_hash: "b0e79a4cb2de556871307afc68eb0fb1297ae022f9408d266ff0ab6570029d62"
---
# Duration and Convexity

## Intuition

Duration is the first-order sensitivity of a bond's price to
its yield: a 1-basis-point yield rise produces approximately
a `D · 0.0001 · price` price drop. Convexity is the
second-order correction: the actual price-yield curve is
convex and bends upward, so a linear duration estimate
under-prices large yield moves in either direction.
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.250-300.

```
<!-- primitive: duration-convexity-tangent source: _diagram_primitives.md -->
price
   ^   *
   |    *
   |     *               actual price (convex)
   |      *
   |       *
   |        *
   |         o ----- linear duration tangent at y0
   |          *  --
   |           *    --
   |            *     --
   |             *      --
   |              o-------+--> tangent under-prices when y >> y0
   |               *
   |                *
   +-----------------------------> yield
                  y0
```

## Definition

Macaulay duration `D_Mac` is the weighted-average time to
receive each cashflow, with weights equal to each cashflow's
present-value share of total price:
`D_Mac = (sum_i t_i · PV(c_i)) / P`. It has units of time.
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.250-275.

Modified duration `D_mod = D_Mac / (1 + y / m)` measures the
percentage change in price for a unit change in yield (in
the same compounding convention). For continuous compounding
`D_mod = D_Mac` exactly. **Source:** CFA L1 Curriculum
(2022) Vol.5/pp.260-280; Hull §4 pp.92-105.

Effective duration estimates `D_mod` from a small symmetric
yield shock (`+Δy` and `-Δy`):
`D_eff = (P_- - P_+) / (2 · P_0 · Δy)`. It accommodates
embedded options for which an analytical Macaulay formula
does not apply. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.275-300.

Convexity `C` is the second derivative of price with respect
to yield, normalized by price:
`C = (sum_i t_i · (t_i + 1/m) · PV(c_i)) · (1 / P) · (1 + y / m)^(-2)`
for fixed coupon vanilla bonds.
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.280-300.

## Mathematical Reasoning

The first-order Taylor expansion of price in yield gives
`ΔP / P ≈ -D_mod · Δy + (1/2) · C · (Δy)^2`. The first term
is the linear duration approximation; the second term is the
convexity correction, always non-negative for vanilla bonds
because `C > 0`. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.280-300; Hull §4 pp.92-105.

The price-yield curve from
[`fi-yield-and-price-mechanics.md`](./fi-yield-and-price-mechanics.md#mathematical-reasoning)
is decreasing (`-D_mod < 0`) and convex (`C > 0`), so the
linear duration tangent at the current yield always
under-prices the actual curve for large moves in either
direction. The convexity term recovers the asymmetry.
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.280-300.

For a bullet bond with a positive coupon, longer maturity,
lower coupon, and lower yield each push duration upward;
duration is a maturity-weighted average and a coupon-bearing
bond's `D_Mac < T`, with equality only for a zero-coupon
bond. **Source:** CFA L1 Curriculum (2022) Vol.5/pp.250-280;
Hull §4 pp.92-105.

Embedded options break the symmetric Taylor expansion: a
callable bond's duration is bounded above by the call-
truncated horizon, and the price-yield curve can show
negative convexity when the call option is at-the-money. The
analytical formula above does NOT apply to callable / putable
bonds; effective duration via shock is the only reliable
measure. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.290-300.

## See Also

- [`fi-yield-and-price-mechanics.md`](fi-yield-and-price-mechanics.md) — the price-yield surface this card differentiates
- [`fi-bond-anatomy-and-cashflows.md`](fi-bond-anatomy-and-cashflows.md) — the cashflow stream feeding the PV weights

## Escalate to Raw When

Open CFA L1 Curriculum Vol.5 Reading 45 or Hull Chapter 4
directly when any of the criteria below applies. **Source:**
CFA L1 Curriculum (2022) Vol.5/pp.250-300.

- The bond has embedded callability or putability and a
  closed-form Macaulay or modified duration is needed
  (effective duration via shock is the only correct measure).
  **Source:** CFA L1 Curriculum (2022) Vol.5/pp.290-300.
- A non-parallel yield shift (curve steepening, twist,
  butterfly) is in scope; modified duration assumes parallel
  shifts only and key-rate / partial-duration measures are
  required. **Source:** CFA L1 Curriculum (2022)
  Vol.5/pp.295-300.
- Portfolio immunization across a curve requires duration
  matching plus convexity matching; this card does not
  develop the immunization framework. **Source:** CFA L1
  Curriculum (2022) Vol.5/pp.290-300.
