---
schema_version: "cacg.v0"
id: "deriv-swap-cashflow-mechanics"
title: "Swap Cashflow Mechanics"
reading_id: "07_derivatives_and_volatility"
summary: "A vanilla interest-rate swap exchanges fixed-rate coupons for floating-rate coupons on a common notional; each reset nets into one cashflow. The swap is structurally a strip of forward-rate agreements, and the par swap rate is the fixed coupon making the no-arbitrage replication PV zero at inception."
tags: ["derivatives", "swap-cashflow"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p173:0261"
    chunk_hash: "4aa080434c444c2c0d90aa1cab87b7325f2480792ca6c4c917119984b9897ed2"
    page_range: [173, 173]
    quote: "An OIS is an agreement to exchange a fixed rate of interest for a reference rate of interest that is calculated from realized overnight rates"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p174:0262"
    chunk_hash: "62e93854cdf13c9c399d3edf3f96c9decb7436ed04463757f134653f33d907c6"
    page_range: [174, 174]
    quote: "Apple is the fixed-rate payer; Citigroup is the floating-rate payer"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p175:0264"
    chunk_hash: "56e196cbccd70451afa9e942f0cea23d6daed4499f61baa4fd9fd61484172f32"
    page_range: [175, 175]
    quote: "The table shows that the swap is equivalent to the exchange of a floating rate bond (cash flows in the third column) for a fixed rate bond"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2883:4319"
    chunk_hash: "352627c75370e3a48e70af01dcad42fd4a9586e82131efb5a2ee2aef28837a80"
    page_range: [2883, 2883]
    quote: "each forward contract will be created at the fixed price that corresponds to the fixed price of a swap of the same"
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2918:4375"
    chunk_hash: "526e7875ae07062af8d0a2880b3a3e3f4234d7cc1581f7915a424118007b5494"
    page_range: [2918, 2918]
    quote: "When two parties engage in a series of forward contracts and initially agree on a price of FS0(T), some of the forward contracts have positive values and some have negative values"
    edge_type: "supports"
card_hash: "e6a9c9bfcdeb82b8e94f697db42f9e6143015e78aa716209cc9379ac59ccbd0d"
---
# Swap Cashflow Mechanics

## Intuition

A vanilla interest-rate swap exchanges a stream of fixed-rate
coupons for a stream of floating-rate coupons on a common
notional principal `N`. Party A pays the fixed rate `F` and
receives floating; Party B pays floating and receives fixed.
Each reset date the two legs net into a single cashflow whose
sign depends on whether the prevailing floating rate is above
or below the fixed rate. The swap is OTC, multi-period, and
worth zero at inception when the par swap rate is chosen
correctly. **Source:** Hull §7 pp.156-180.

```
<!-- primitive: swap-cashflow-ladder source: _diagram_primitives.md -->
inception                                            maturity
   |                                                     |
   |   F       F       F       F       F       F        |
   v   v       v       v       v       v       v        v
A--*---|---|---|---|---|---|---|---|---|---|---|---|----*    fixed leg
   t=0 t1  t2  t3  t4  t5  t6  t7  t8  t9 t10 t11 t12  T
                       |
                       |    L1      L2      L3      LN
                       v    v       v       v       v
B--*---|---|---|---|---|---|---|---|---|---|---|---|----*    float leg
       net = (F - Li) per period; A pays fixed, B pays float
```

## Definition

A **vanilla interest-rate swap** is specified by `(N, F, T, n,
ref)` where `N` is the notional principal (used for cashflow
scaling, not exchanged at inception or maturity), `F` is the
fixed coupon rate, `T` is the swap tenor, `n` is the number of
reset periods (typically semiannual or quarterly), and `ref`
is the floating-rate benchmark (historically LIBOR, now
SOFR / SONIA / OIS). At each reset date `t_i` the net cashflow
is `N · (F - L_i) · τ_i` to the fixed receiver, where `L_i` is
the prevailing floating rate at the start of the period and
`τ_i` is the day-count fraction. **Source:** Hull §7
pp.156-180; CFA L1 Curriculum (2022) Vol.5/pp.405-410.

The **par swap rate** is the fixed rate that makes the swap's
net present value zero at inception. It is determined by the
zero-coupon discount curve: the PV of the fixed leg
`N · F · Σ τ_i · D(0, t_i)` must equal the PV of the floating
leg, which under the no-arbitrage construction
`floating PV = N · (1 - D(0, T))` (the floating leg's PV equals
notional times one minus the terminal discount factor when the
floating leg is reset to the prevailing rate at each period
start). Setting the two equal gives
`F = (1 - D(0, T)) / (Σ τ_i · D(0, t_i))`. **Source:** Hull §7
pp.156-180.

## Mathematical Reasoning

A vanilla swap decomposes into a strip of forward contracts on
the floating-rate benchmark: each reset-period cashflow is
identical in shape to the cashflow from a forward-rate-
agreement (FRA) on the period's reference rate. The swap's
PV at inception is the sum of the FRA-strip PVs; setting that
sum to zero gives the par-swap-rate equation. This
decomposition is the bridge between the swap (multi-period
exchange) and the forward (single-period exchange) covered in
the prior card. **Source:** Hull §7 pp.156-180.

Post-inception the swap's mark-to-market value is non-zero in
general: when the floating-rate curve moves, the
fixed-receiver's position changes by approximately
`-DV01_swap × Δ curve`, where `DV01_swap` is the present-value
sensitivity to a one-basis-point parallel shift in the curve.
The fixed-leg's DV01 dominates for short remaining tenors and
flat curves; the floating leg's DV01 is concentrated in the
next reset period. The decomposition into duration components
is the bridge into 06's `fi-duration-and-convexity.md`
machinery. **Source:** Hull §7 pp.180-205.

Modern post-LIBOR swap markets use OIS discounting and overnight-
rate benchmarks such as SOFR / SONIA for the floating reference
rate; the dual-curve or multi-curve construction (per
[`fi-yield-curve-construction.md`](../06_fixed_income_and_credit/fi-yield-curve-construction.md#mathematical-reasoning))
adapts the par-swap-rate equation to a discount curve that
differs from the projection curve. The L1 treatment uses a
single-curve construction; the boundary into the multi-curve
practitioner machinery lies in 06. **Source:** Hull §7
pp.156-180.

## See Also

- [`deriv-anatomy-and-instrument-types.md`](deriv-anatomy-and-instrument-types.md) — taxonomy that places the swap against forward / future / option
- [`deriv-forward-and-futures-payoff.md`](deriv-forward-and-futures-payoff.md) — the FRA-strip decomposition links each swap reset to a forward

## Escalate to Raw When

Open Hull chapter 7 directly when any of the criteria below
applies. **Source:** Hull §7 pp.156-205.

- The swap is non-vanilla (basis swap, currency swap,
  amortizing notional, off-market fixed rate); the
  par-swap-rate equation generalizes per Hull §7.
  **Source:** Hull §7 pp.156-205.
- Multi-curve / OIS-discount construction matters; see 06's
  [`fi-yield-curve-construction.md`](../06_fixed_income_and_credit/fi-yield-curve-construction.md#mathematical-reasoning)
  for the modern dual-curve treatment. **Source:** Hull §7
  pp.156-180.
- Counterparty / CSA / collateral mechanics matter; see 06's
  [`fi-collateralization-and-csa.md`](../06_fixed_income_and_credit/fi-collateralization-and-csa.md#definition)
  for CSA terms and collateral-currency choice (DEC-12
  RESOLVED placement of XVA in 06). **Source:** Hull §7
  pp.180-205.
