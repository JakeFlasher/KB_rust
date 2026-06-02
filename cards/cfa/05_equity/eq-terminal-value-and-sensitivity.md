---
schema_version: "cacg.v0"
id: "eq-terminal-value-and-sensitivity"
title: "Terminal Value and Sensitivity"
reading_id: "05_equity"
summary: "Closing a DCF with a terminal value `TV_N` that summarizes all cash flows beyond the explicit forecast horizon. Gordon-growth-perpetuity and exit-multiple are the two canonical forms; the stable-growth constraint `g < r_f` and reinvestment-consistency `g = RR · ROC` bound the inputs. TV typically dominates the headline DCF, so sensitivity to terminal growth and discount rate must be reported."
tags: ["equity", "terminal-value"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p429:0536"
    chunk_hash: "368f053cd2b9ba4195fa1167401f8b306c0091bb9a42f7380ace457f38813453"
    page_range: [429, 430]
    quote: "You can find the terminal value in one of two ways. One is to assume a liquidation of the firm’s assets in the terminal year and estimate what others would pay"
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p488:0609"
    chunk_hash: "08cdb1fed30b5eee52fc2ac717c0fb8291bce2ee83f5b14957206db2e45b17b2"
    page_range: [488, 489]
    quote: "While the Gordon growth model provides a simple approach to valuing equity, its use is limited to firms that are growing at a stable growth rate."
    edge_type: "supports"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p537:0677"
    chunk_hash: "c39352348ca9ca6f8af3c6960cb645b796a75bd3827acadeb201ef02ce4ef48a"
    page_range: [538, 538]
    quote: "discounting the cumulated cash flows to all claim holders in the firm by the weighted average cost of capital (the cost of capital approach)"
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2317:3397"
    chunk_hash: "fa54d00deeaeac6f3c563a7e37f66414ec37cc8b592497ecdc02ef4a16ac9ea5"
    page_range: [2317, 2317]
    quote: "The Gordon growth model is used to estimate a terminal value at time n that reflects the present value at time n of the dividends received during the sustainable growth period."
    edge_type: "supports"
card_hash: "af7004538ed7b3ab15d2962cc64b4c1227b6e33975d25695b141bc97fd69012b"
---
# Terminal Value and Sensitivity

## Intuition

A discounted-cash-flow valuation cannot forecast cash flows forever.
Beyond a finite explicit-forecast horizon `N`, the analyst summarizes
all subsequent cash flows in a single terminal value `TV_N`. The
terminal value is itself a present-value computation done as of date
`N`, then discounted back to today at the same rate used through the
explicit period. **Source:** Damodaran (2012) Ch.12 pp.429-457.

Terminal value almost always dominates the headline valuation,
especially for growth firms or firms with short explicit-forecast
horizons. A small change in the assumed terminal growth rate or the
assumed exit multiple can produce a large change in present value
because the terminal value is the longest-duration component of the
valuation. The two main forms are Gordon-growth perpetuity (the
firm reaches a stable-growth state and grows forever at `g_stable`)
and exit multiple (the firm is valued at `N` using a relative-
valuation multiple applied to a `N`-period earnings or cash-flow
measure). **Source:** Damodaran (2012) Ch.12 pp.429-457.

```
explicit-forecast horizon          terminal value
[ FCF_1, FCF_2, ..., FCF_N ]   ==>   TV_N
   |                                   |
   v                                   v
discount back at r              discount back at r
each period                     once, from period N
   |                                   |
   +------------+----------------------+
                |
                v
           V_0  =  PV(explicit) + PV(terminal)
           ^^^^                       ^^^^^^^^^^^
           often the smaller       often the larger
           contribution            contribution
```

## Definition

The terminal value at horizon `N` summarizes the present value, as of
date `N`, of all cash flows from `N+1` onward. Two canonical forms
dominate practice: the Gordon-growth perpetuity and the exit-multiple
valuation. **Source:** Damodaran (2012) Ch.12 pp.429-457.

The Gordon-growth perpetuity form is `TV_N = CF_{N+1} / (r -
g_stable)` where `CF_{N+1}` is the next-period cash flow after the
explicit horizon, `r` is the discount rate appropriate to the cash-
flow definition, and `g_stable` is the perpetual stable growth rate
strictly less than `r` to ensure convergence. The form is the same
geometric-series collapse that underlies the DDM Gordon-growth
special case (see
[`eq-dividend-discount-models`](./eq-dividend-discount-models.md)).
**Source:** Damodaran (2012) Ch.12 pp.429-457.

The exit-multiple form values the firm at horizon `N` using a
relative-valuation multiple applied to a `N`-period earnings, cash-
flow, sales, or book-value measure: `TV_N = multiple_N · X_N`. The
multiple is selected from comparable companies' multiples or from the
firm's own historical multiples; the choice of `X_N` (EBITDA, EBIT,
revenue, book value) determines which multiple family applies. The
exit-multiple form imports a relative-valuation judgement into a DCF
framework, mixing the two valuation paradigms. **Source:** Damodaran
(2012) Ch.12 pp.429-457.

The stable-growth assumption is more demanding than it sounds. A
firm in stable growth must have growth that does not exceed the
overall economy's growth rate (otherwise the firm eventually exceeds
the size of the economy), reinvestment consistent with that growth
(via the `g = retention · ROE` identity — see
[`eq-payout-policy-and-growth`](./eq-payout-policy-and-growth.md)),
and a risk profile consistent with mature-firm cost of capital. A
common defect is assigning a stable-growth rate higher than the long-
run riskless rate, which violates the consistency between growth and
the riskless rate's implied long-run nominal economic growth.
**Source:** Damodaran (2012) Ch.12 pp.429-457.

The cash-flow / discount-rate consistency rule from
[`eq-dcf-mechanics`](./eq-dcf-mechanics.md) carries through to the
terminal value: equity-claim TV uses the cost of equity; firm-claim
TV uses WACC. The reinvestment-rate assumption in the stable-growth
phase typically differs from the explicit-forecast phase, reflecting
the firm's transition to maturity. **Source:** Damodaran (2012) Ch.12
pp.429-457.

## Mathematical Reasoning

The Gordon-growth perpetuity formula `TV_N = CF_{N+1} / (r -
g_stable)` derives from the geometric-series collapse of an infinite
stable-growth cash-flow stream evaluated at date `N`. The discounted
sum from `N+1` to infinity, with first term `CF_{N+1} / (1 + r)` and
common ratio `(1 + g_stable) / (1 + r)`, converges to the closed
form provided `g_stable < r`. The convergence condition is identical
to the DDM Gordon-growth special case. **Source:** Damodaran (2012)
Ch.12 pp.429-457.

The terminal value is then discounted from `N` back to today at the
same `r`: `PV(TV_N) = TV_N / (1 + r)^N`. The total DCF value is
`V_0 = sum_{i=1..N} CF_i / (1 + r)^i + TV_N / (1 + r)^N`. This is
the same structure as the general DCF formula in
[`eq-dcf-mechanics`](./eq-dcf-mechanics.md), with the terminal value
substituted in place of the implicit infinite tail. **Source:**
Damodaran (2012) Ch.12 pp.429-457.

The sensitivity of TV to `g_stable` is governed by the partial
derivative `d(TV_N) / d(g_stable) = CF_{N+1} / (r - g_stable)^2`,
which is positive and grows quadratically as `g_stable` approaches
`r`. The denominator `(r - g_stable)^2` shrinks rapidly as `g_stable`
nears the convergence boundary, so a small upward shift in `g_stable`
near that boundary produces a disproportionate jump in TV. The
sensitivity to `r` has the opposite sign and the same near-boundary
amplification. **Source:** Damodaran (2012) Ch.12 pp.429-457.

The reinvestment-consistency check writes the stable-growth cash
flow as `CF_{N+1} = CF_N · (1 + g_stable)` with the implied
reinvestment rate `RR_stable = g_stable / ROC_stable` (where `ROC` is
the stable-growth return on capital). If reinvestment in the stable
phase is too low to sustain the assumed growth, the model has an
internal inconsistency between the cash flow and the growth rate.
**Source:** Damodaran (2012) Ch.12 pp.429-457.

The exit-multiple form `TV_N = multiple_N · X_N` does not require a
geometric-series convergence condition because it is not a perpetuity.
However, it does import the assumption that the multiple at `N` is
representative of the firm's mature-state economics. A common defect
is using current-period industry multiples for `multiple_N` without
adjusting for the firm's expected mature-state risk and growth
profile, which embeds today's multiples regime into the terminal
year. **Source:** Damodaran (2012) Ch.12 pp.429-457.

The CFA L1 frame presents terminal value as the closing component of
DCF, distinguishes Gordon-growth and exit-multiple forms, and
emphasizes the dominant contribution of TV to most DCF valuations.
**Source:** CFA L1 Curriculum (2022) Vol.4/pp.361-416.

## See Also

- [`eq-dcf-mechanics`](./eq-dcf-mechanics.md) — the DCF engine TV closes
- [`eq-payout-policy-and-growth`](./eq-payout-policy-and-growth.md) — the `g = retention · ROE` reinvestment-consistency identity
- [`eq-dividend-discount-models`](./eq-dividend-discount-models.md) — the Gordon-growth special case shares the convergence condition

## Escalate to Raw When

Open Damodaran Ch.12 directly when any of the criteria below applies.
**Source:** Damodaran (2012) Ch.12 pp.429-457.

- the firm is unlikely to reach a true stable-growth state within a tractable horizon (very long-cycle commodity firm, secular-decline industry, regulated utility with prolonged transition) — Damodaran Ch.12 develops the multi-stage transition forms. **Source:** Damodaran (2012) Ch.12 pp.429-457.
- the exit-multiple selection is contested and the analyst needs the framework for adjusting today's multiples to mature-state economics — Damodaran Ch.12 derives the multiple-decomposition adjustments. **Source:** Damodaran (2012) Ch.12 pp.429-457.
- the headline valuation is dominated by TV (e.g., 80%+ of total PV) and the analyst must report the implied stable-growth and discount-rate sensitivity bands — Damodaran Ch.12 develops the partial-derivative sensitivity machinery. **Source:** Damodaran (2012) Ch.12 pp.429-457.
