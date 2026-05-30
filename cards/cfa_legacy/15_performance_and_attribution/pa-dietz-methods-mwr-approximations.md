---
schema_version: "cacg.v0"
id: "pa-dietz-methods-mwr-approximations"
title: "Dietz Methods: Simple and Modified as IRR Approximations"
reading_id: "15_performance_and_attribution"
summary: "Simple Dietz assumes mid-period flows over average capital invested; modified Dietz day-weights each flow. Both are money-weighted returns, and modified Dietz is a first-order approximation to the IRR — not to the time-weighted return."
tags: ["dietz-method", "money-weighted-return", "irr-approximation"]
citations:
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p054:0060"
    chunk_hash: "a91eed7fafb7c3a4d6c4a653c90284f7844c1e4970b422ed081b4a2ff2d761c1"
    page_range: [54, 55]
    quote: "The modified Dietz method is a money-weighted rate of return and can be accurately described as an approximation of the internal rate of return."
    edge_type: "defines"
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p048:0043"
    chunk_hash: "3a6662e6ed87b059df591979b188428e4647d14b32dc6d1f665ab3b32ee8438c"
    page_range: [49, 49]
    quote: "Dietz proposed a simpler calculation than the IRR, one that also assumes a constant rate of growth over a period between valuations."
    edge_type: "supports"
---
# Dietz Methods: Simple and Modified as IRR Approximations

## Intuition

The internal rate of return is the exact money-weighted return, but it has no
closed form: solving it requires iteration. Peter Dietz proposed cheaper
algebraic stand-ins. The core trick is to stop solving for the rate that
discounts every flow exactly, and instead *adjust the capital base* so the
gain divides cleanly. Simple Dietz makes the crudest possible timing
assumption — every external flow lands at the midpoint of the period — so half
the flow is treated as invested capital. Modified Dietz keeps the same shape
but replaces that one-size midpoint with a per-flow day-weight, so a flow
arriving late in the period counts as little capital and a flow arriving early
counts as nearly a full period. Both inherit the IRR's money-weighted
character: a manager who cannot control the timing or size of client flows is
still scored on the cash actually at work.
**Source:** Bacon (2023) §3 (Simple Dietz / Modified Dietz) pp.52-55

## Definition

**Simple (original) Dietz.** Return equals the investment gain over the
average capital invested, where average capital is the start value plus half
the net external cash flow:

> r = (V_E - V_S - C) / (V_S + C/2)

The denominator is *not* the average of start and end values — that would leak
portfolio return into the capital base. The mid-period assumption is the only
timing input, so simple Dietz needs only start value, end value, and total
external flow.
**Source:** Bacon (2023) §3 (Simple Dietz) pp.52-53

**Modified (day-weighted) Dietz.** Each flow C_t is weighted by the fraction
of the period it was invested, W_t = (TD - D_t)/TD, giving a more accurate
average capital employed:

> r = (V_E - V_S - C) / (V_S + sum C_t * W_t)

The analyst must fix a company policy for whether a same-day flow is treated as
beginning-of-day (manager can act, include the day) or end-of-day (manager
cannot act, exclude the day) and apply it consistently.
**Source:** Bacon (2023) §3 (Modified Dietz) pp.54-55

**Classification.** Both Dietz formulas are money-weighted methodologies, and
Christopherson, Cariño & Ferson present the Dietz idea as a constant-growth
approximation in the same family as the IRR.
**Source:** Christopherson, Cariño & Ferson (2009) §5 (The Dietz Method) pp.49-51

## Mathematical Reasoning

Simple Dietz is the limiting case of modified Dietz where every flow is
assigned the midpoint weight W_t = 1/2: then sum C_t * W_t collapses to C/2 and
the day-weighted denominator becomes V_S + C/2. So the two formulas share one
algebraic skeleton, differing only in how the capital base is weighted.

Bacon reports the result, due to Fischer and Wermers, that modified Dietz is a
**first-order linear approximation to the IRR**, not to the time-weighted
return. The source asserts this without reproducing the expansion, so this card
asserts it too and labels the gap: the second-order term exists but is more
complex and less practical, and modified Dietz is already a good approximation.
The crucial corrective is that the approximation target is the IRR (a
money-weighted quantity), so calling modified Dietz an approximation of the TWR
is a category error.

```
            money-weighted family            time-weighted family
        ---------------------------------    --------------------
        IRR  (exact, iterative)              true TWR (exact, needs
          |                                   valuation at each flow)
          | 1st-order linear approx
          v
        modified Dietz  (day-weighted base)
          |
          | all flows -> midpoint weight 1/2
          v
        simple Dietz    (mid-period base)

        Dietz approximates the IRR, NOT the TWR.
```

**Source:** Bacon (2023) §3 (Modified Dietz / Caution) pp.54-55

## Boundary Notes

Income handling is a separate axis from timing. The ICAA extension and the
"income unavailable" variant adjust the numerator and capital base for income
that is or is not available for reinvestment; treating retained income as a
negative cash flow shrinks the denominator and gears the rate. Those numerator
adjustments are out of scope here — this card covers only the timing
assumption that separates simple from modified Dietz.
**Source:** Bacon (2023) §3 (ICAA method / Income unavailable) pp.53-54

## See Also

- [`pa-irr-money-weighted-return.md`](pa-irr-money-weighted-return.md) — the exact money-weighted return that Dietz approximates.
- [`pa-true-twr-and-chain-linking.md`](pa-true-twr-and-chain-linking.md) — the time-weighted return Dietz is sometimes wrongly said to approximate.
- [`pa-twr-vs-mwr-when-each-applies.md`](pa-twr-vs-mwr-when-each-applies.md) — when a money-weighted Dietz figure is the right scoring choice.

## Escalate to Raw When

- You need the worked Dietz numbers (Bacon's standard example: start 74.2m, end
  104.4m, flow 37.1m, giving simple Dietz -7.44% and modified Dietz -7.30% or
  -7.21% depending on the day-14 timing convention) — read Bacon (2023) pp.52-55.
- You need the ICAA / income-unavailable numerator variants and their worked
  results — read Bacon (2023) pp.53-54.
- You need the order-of-approximation expansion proving modified Dietz is
  first-order in the IRR, including the second-order term — read the underlying
  Fischer and Wermers treatment.
- You need the cash-flow-magnitude threshold (e.g. the 10%-of-value revaluation
  trigger) governing when Dietz/IRR degrade as approximations — read
  Christopherson, Cariño & Ferson (2009) pp.48-51.
