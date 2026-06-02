---
schema_version: "cacg.v0"
id: "pa-irr-money-weighted-return"
title: "The Internal Rate of Return as the Money-Weighted Method"
reading_id: "15_performance_and_attribution"
summary: "IRR is the discount rate setting net present value to zero; as the money-weighted return it assigns every invested dollar one constant effective rate, which aids cash-flow-sensitive measurement but blocks attribution disaggregation."
tags: ["money-weighted-return", "irr", "cash-flow"]
citations:
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p050:0055"
    chunk_hash: "b2d06ce744a329f4cf6787e3bf53e877712014d8f74265a6a00edd383bb5b7d2"
    page_range: [50, 50]
    quote: "IRR is an example of a money-weighted return methodology; each amount or dollar invested"
    edge_type: "defines"
card_hash: "e995e6d097119c510237e3e850f653c06a58bda6a21f037c5395a6941c17ba83"
---
# The Internal Rate of Return as the Money-Weighted Method

## Intuition

When an investor adds or withdraws capital mid-period, a naive ratio of end value to start value no longer measures the manager's return: the external cash flow itself moves the valuation, so we cannot simply chain market-value ratios. The internal rate of return (IRR) answers a different question — given when and how much money actually entered and left, what single constant rate of growth would reconcile the starting value, the dated cash flows, and the ending value? Because larger sums and longer-held sums carry more arithmetic weight in that reconciliation, the IRR is a *money-weighted* return: the performance during the periods when the most money was invested dominates the result. This is why the timing and size of external cash flow can flatter or penalise the IRR even when the underlying per-period market performance is unchanged.

**Source:** Bacon (2023) §3 Money-Weighted Returns pp.47-50

## Definition

The IRR is the discount rate that makes the net present value of a series of dated cash flows equal to zero in a discounted cash flow analysis. Used to evaluate a prospective project this is an *ex-ante* IRR; applied to realised portfolio data over a measurement period it is an *ex-post* IRR. In its single-period money-weighted form for a portfolio the rate `r` is the solution to a relation linking the start value, the dated external cash flows, and the end value. The defining property is that "IRR is an example of a money-weighted return methodology; each amount or dollar invested" is assumed to achieve the same effective rate of return irrespective of when it was invested.

**Source:** Bacon (2023) §3 Money-Weighted Returns pp.48-50

## Mathematical Reasoning

The book states the single-period money-weighted (simple) IRR as the rate `r` satisfying

```
End value VE = VS*(1 + r) + C*(1 + r)^0.5
```

where `VS` is the start market value and `C` the total external cash flow assumed received at the mid-point of the period. Generalising, the ex-post form weights each dated flow by the fraction of the period it was available for investment:

```
VE = VS*(1 + r) + sum_{t=1..T}  C_t * (1 + r)^{W_t}
```

with `W_t = (TD - D_t)/TD`, where `TD` is the total number of days in the measurement period and `D_t` is the number of days elapsed when flow `C_t` occurs. Equivalently, IRR is the rate making net present value zero: discounting all dated flows and the terminal value at `r` returns the initial outlay.

The book lists, without formal proof, the principal criticisms of the method and we report them as asserted:

```
                 IRR / money-weighted method
                 ---------------------------------
  iterative      no closed-form root in general; needs Newton-Raphson-style
  solution       iteration (a closed quadratic exists ONLY for the simple
                 single-period mid-point case via x = (1 + r)^0.5)
  multiple       a sign change in cash-flow series admits multiple roots
  roots          (technically possible per sign change; rare in practice)
  constant       a single constant "force of return" is assumed for the whole
  force          period and for every asset --> CANNOT split r across asset
                 categories --> blocks attribution disaggregation
  reinvestment   ex-ante IRR assumes flows reinvest at the same r (an ex-ante,
                 not ex-post, concern)
  manipulable    controlling the timing of capital calls/returns shifts r
```

The constant-force-of-return assumption is the decisive limitation for attribution work: because one rate is held fixed across the whole portfolio and period, the IRR cannot be decomposed into per-asset-category contributions, so it does not support the additive disaggregation that attribution analysis requires. The book asserts this consequence directly rather than deriving it; we mirror that and label the derivation as not supplied by the source.

**Source:** Bacon (2023) §3 Money-Weighted Returns (Eqs. 3.13-3.15) pp.49-50

## See Also

- [`pa-twr-vs-mwr-when-each-applies.md`](pa-twr-vs-mwr-when-each-applies.md) — when the money-weighted IRR is the right choice versus a time-weighted return.
- [`pa-dietz-methods-mwr-approximations.md`](pa-dietz-methods-mwr-approximations.md) — simple and modified Dietz as closed-form approximations to the iterative IRR.
- [`pa-true-twr-and-chain-linking.md`](pa-true-twr-and-chain-linking.md) — the cash-flow-neutral alternative that restores the disaggregation IRR forfeits.

## Escalate to Raw When

- You need a worked single-period or ex-post IRR (e.g. Exhibits 3.4-3.6 solving for `r` given start `$74.2m`, end `$104.4m`, and cash flow `$37.1m`), including the quadratic-formula solution for the simple case — Bacon (2023) pp.50-51 carries the arithmetic this card omits under Critical Rule 1.
- You need the precise day-count weighting `W_t = (TD - D_t)/TD` applied to a real dated cash flow, or the distinction between Excel's `IRR` and `XIRR` functions for irregular periods.
- You need the formal references behind the multiple-roots and manipulation criticisms (Fischer & Wermers on multiple solutions; Phalippou on IRR hazards) cited in Bacon (2023) pp.49-50.

**Source:** Bacon (2023) §3 Money-Weighted Returns pp.50-51
