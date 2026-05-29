---
schema_version: "cacg.v0"
id: "fi-mbs-key-rate-and-effective-duration"
title: "MBS Effective and Key-Rate Duration"
reading_id: "06_fixed_income_and_credit"
summary: "MBS Effective and Key-Rate Duration — CFA Vol.5/pp.275-310 (PDF 2912-2947) is in derivatives R47-R49; MBS duration content not present here."
tags: ["fixed-income", "mbs-key"]
citations:
  - source_id: "fi_davidson_levin_2014_mortgage_valuation_models"
    chunk_id: "fi_davidson_levin_2014_mortgage_valuation_models:p226:0261"
    chunk_hash: "7baebd927a238242e5dcf06cf6b6baaf8cad9106cbf82c8e8303af107d06d31e"
    page_range: [226, 227]
    quote: "In contrast, Effective Duration assumes hypothetical changes in interest rates only, that is, it intends to measure a conditional dependence."
    edge_type: "defines"
  - source_id: "fi_davidson_levin_2014_mortgage_valuation_models"
    chunk_id: "fi_davidson_levin_2014_mortgage_valuation_models:p138:0154"
    chunk_hash: "d8050a6e9a846027254b007e334e50377b3b3c91b4c51d3de420e64af5cb552c"
    page_range: [138, 139]
    quote: "Discount mortgages generally have prepayment speeds under 10% CPR, while premium mortgages can exhibit prepayments speeds well above 50% CPR."
    edge_type: "supports"
card_hash: "33aed2d443f4ce062a87b0b473e71fe84bb2fb5d40593e8c94986cd73ee6c13d"
---
# MBS Effective and Key-Rate Duration

## Intuition

A vanilla fixed-coupon bond's duration is a closed-form quantity derived from its cash-flow schedule; the bond's price moves linearly in yield to first order. An MBS's cash flows depend on the rate path via the prepayment model, so its price is a non-trivial function of the rate curve. The **effective duration** measures the MBS's price sensitivity to a parallel shift in the entire short-rate curve by re-running the OAS Monte Carlo at shifted curves and finite-differencing. The **key-rate duration** generalizes this to per-tenor shifts: shift only the 2y point, observe the price response, repeat at every tenor. The result is a vector of per-tenor sensitivities that exposes how the MBS's risk is distributed across the curve. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

```
finite-difference computation of MBS effective duration
   procedure:
       step a: OAS at base curve r_base via Monte Carlo
       step b: shift entire curve up by Δr: r_up := r_base + Δr
               re-run Monte Carlo at r_up holding OAS fixed
               obtain model price P_up at r_up + OAS_base
       step c: shift entire curve down by Δr: r_down := r_base − Δr
               re-run Monte Carlo at r_down holding OAS fixed
               obtain model price P_down at r_down + OAS_base
       step d: effective duration := − (P_up − P_down) / (2 · P_base · Δr)
       step e: convexity := (P_up + P_down − 2 · P_base) / (P_base · Δr^2)

   convexity profile (typical agency MBS):
       price
        ^
        |      vanilla bond
        |       (convex curve, positive convexity)
        |     ____
        |    /    \___
        |   /         \___
        |  /              \___          ___
        | /                   \___     /
        |/                        \___/  MBS price
        +----------------------------> rate
                rate falls          rate rises
       interpretation:
       - vanilla bond: price rises convexly as rate falls
       - MBS: prepays accelerate as rate falls, capping the
         price upside; this creates the price-plateau on
         the left side and the negative-convexity signature

   key-rate duration vector for an MBS:
       KRD_2y   small positive
       KRD_5y   medium positive
       KRD_10y  large positive    (the dominant exposure on
                                   most MBS)
       KRD_30y  small positive
       sum equals the effective duration
```

## Definition

The **effective duration** of an MBS is `−(P_up − P_down) / (2 · P_base · Δr)` where `P_base`, `P_up`, `P_down` are model prices computed via the OAS Monte Carlo at the base curve and at the curve shifted up and down by `Δr`. The OAS is held fixed at the base-curve calibration to isolate the rate-sensitivity of the prepayment-and-cash-flow response. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

The **effective convexity** is `(P_up + P_down − 2 · P_base) / (P_base · Δr^2)`, the second-order PnL coefficient. For agency MBS the effective convexity is typically negative — the MBS price has a concave-down profile as rates fall because accelerated prepayments cap the upside. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

The **key-rate duration (KRD)** at tenor `T_k` is the price sensitivity to a localized shift of the short-rate curve at tenor `T_k` only (with other tenors held fixed). The KRD vector sums to the effective duration. The KRD profile reveals where on the curve the MBS's risk is concentrated; for typical agency MBS the dominant KRD is at intermediate tenors (5y-10y), reflecting the typical mortgage's expected average life. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

A **shift method** for KRD computation is the per-tenor parallel shift: the short-rate curve is shifted by `Δr` only at tenor `T_k` and by zero elsewhere (with interpolation between shifted and unshifted tenors). The price response is finite-differenced to get the KRD at `T_k`. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

## Mathematical Reasoning

The finite-difference computation introduces small numerical error of order `O(Δr^2)` in the duration estimate; the shift size `Δr` is typically chosen at the scale of a few basis points to balance numerical truncation against Monte Carlo simulation noise. Davidson+Levin discuss the practical choice. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

The effective duration's interpretation as the (negative) price-elasticity to a parallel curve shift is identical in form to the vanilla-bond Macaulay-and-modified duration of [`fi-duration-and-convexity.md`](./fi-duration-and-convexity.md#mathematical-reasoning); the difference is that MBS effective duration is shorter than the underlying mortgages' average life because prepayment cushions the rate sensitivity. An MBS with 30-year underlying mortgages typically has an effective duration of 5-10 years depending on prepayment-speed expectations. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

The negative convexity arises from the embedded prepayment option: as rates fall, the prepayment-driven cash flows accelerate, returning par to the holder earlier. The early-arriving principal must be reinvested at the (now-lower) prevailing rates; the holder's PnL is therefore less than the convex-upside of a vanilla bond at the same maturity. The asymmetric response creates the price-plateau signature seen in MBS price-vs-rate curves. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

The key-rate duration generalizes the partial-DV01 framework from [`fi-key-rate-and-partial-duration.md`](./fi-key-rate-and-partial-duration.md#mathematical-reasoning) to MBS: the L1 partial-DV01 framework computes per-tenor sensitivities for vanilla bonds with no prepayment optionality; the Davidson+Levin extension applies the same per-tenor shift methodology to the OAS Monte Carlo. The mathematical structure is identical; the input changes (vanilla bond price vs MBS Monte Carlo price). **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

The KRD profile of a typical agency MBS concentrates at intermediate tenors (5y-10y) because the typical mortgage's expected average life (after accounting for prepayments and amortization) falls in that range. The 30y mortgage's nominal maturity does not imply 30y KRD concentration; the effective economic horizon is much shorter. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

For hedging applications, the KRD vector enables an MBS to be hedged with a curated set of Treasury bonds or interest-rate swaps at the dominant-KRD tenors, rather than with a single duration-matched bond. The hedge ratios are the KRD vector entries; the resulting hedged-portfolio's residual PnL captures the negative convexity (which the linear hedge cannot eliminate) and the OAS basis (which the hedge instruments do not carry). **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

## See Also

- [`fi-oas-and-effective-duration.md`](fi-oas-and-effective-duration.md) — L1 OAS / effective-duration intuition the Davidson+Levin depth quantifies
- [`fi-key-rate-and-partial-duration.md`](fi-key-rate-and-partial-duration.md) — L1 partial-DV01 framework generalized to MBS
- [`fi-mbs-oas-calculation-depth.md`](fi-mbs-oas-calculation-depth.md) — OAS Monte Carlo machinery underlying the duration finite-differences
- [`fi-mbs-prepayment-models.md`](fi-mbs-prepayment-models.md) — prepayment-model rate-sensitivity that drives the negative-convexity signature
- [`fi-duration-and-convexity.md`](fi-duration-and-convexity.md) — vanilla-bond baseline for the parallel-shift duration concept

## Escalate to Raw When

Open Davidson & Levin (2014) Ch.10 (Applications of OAS
Valuation) directly when any of the criteria below applies.
**Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

- The card user needs the per-MBS effective-duration time
  series during a specific historical rate-shock episode at
  dated precision.
  **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.
- A specific MBS hedge construction (e.g. duration-matched
  Treasury short or swap-spread overlay) requires the dated
  KRD vector and the hedge-instrument KRD profile.
  **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.
- The card user needs the finite-difference shift size and
  variance-reduction technique selection for production-
  precision MBS duration quotes.
  **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.
- Negative-convexity hedging with options-on-rates (caps,
  floors, swaptions) is required — out of this card's
  linear-hedge framing; escalate to a future specialty plan.
  **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.
