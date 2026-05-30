---
schema_version: "cacg.v0"
id: "pa-true-twr-and-chain-linking"
title: "True Time-Weighted Return and Geometric Chain-Linking"
reading_id: "15_performance_and_attribution"
summary: "True (classical) TWR values the portfolio at every external cash flow, forms simple wealth-ratio sub-period returns, and geometrically chain-links them; the unit-price (NAV) method is its variant, at the cost of accurate valuation at each flow."
tags: ["time-weighted-return", "chain-linking", "unit-price-nav"]
citations:
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p055:0061"
    chunk_hash: "38a791dc91e8cac82a3a56a7b3a7219072bbb80bee8f1d6b371cac9b5b7b1a17"
    page_range: [55, 56]
    quote: "sub-period between cash flows using simple wealth ratios. The sub-period returns are then"
    edge_type: "defines"
---
# True Time-Weighted Return and Geometric Chain-Linking

## Intuition

A time-weighted return tries to isolate the manager's skill from the client's
cash-flow timing: every elapsed time period gets equal weight regardless of how
much money happened to be invested in it. To do that purely, you must "cut" the
measurement interval at every external cash flow, because a deposit or withdrawal
changes the capital base without being a manager decision. Within each slice the
portfolio is undisturbed by flows, so its growth is a clean wealth ratio. Compound
(chain-link) those slice returns and the cash-flow effect cancels out: you recover
the growth path the assets actually delivered, "the return that would have been
achieved had there been no cash flows."

**Source:** Bacon (2023) §3 (Time-Weighted Returns) pp.55-61

## Definition

In the **true (or classical) time-weighted** methodology, performance is calculated
for each sub-period between cash flows using **simple wealth ratios**, and the
sub-period returns are then geometrically chain-linked into the full-period return.
A valuation `Vt` is struck immediately after each external cash flow `Ct`; the wealth
ratio over a slice is the end value (net of the flow that closes the slice) divided
by the slice's start value.

The **unit price** (or **unitised** / **net asset value**, NAV) method is a variant:
instead of raw market-value ratios, a standardised unit price is computed immediately
before each cash flow by dividing market value by units outstanding, and units are
bought/sold at that price. The ratio of end-period to start-period unit price then
gives the return directly, irrespective of intervening flows. It is typically used
for pooled funds (unit trusts, mutual funds, hedge funds, ETFs) and always agrees
with the true TWR. A major drawback of true TWR is that **accurate valuations are
required at the date of each cash flow** — an onerous, expensive "daily valuation"
requirement.

**Source:** Bacon (2023) §3 (True time-weighted; Unit price method) pp.56-62

## Mathematical Reasoning

Let `Vt` be the valuation immediately after cash flow `Ct` at the end of sub-period
`t`, with start value `VS` and end value `VE`. The end-of-day-flow chain-link
(Bacon Eq. 3.23) is the product of slice wealth ratios:

```
(V1 - C1)   (V2 - C2)         (V_{n-1} - C_{n-1})   (VE - Cn)
--------- x --------- x ... x ------------------- x --------- = 1 + r
   VS           V1                  V_{n-2}            V_{n-1}
```

Defining each slice return by the wealth ratio `(Vt - Ct)/V_{t-1} = 1 + rt` (the
ratio struck immediately prior to receiving the external flow), the identity collapses
to the familiar geometric compounding form:

```
(1 + r1)(1 + r2)(1 + r3) ... (1 + r_{n-1})(1 + rn) = 1 + r
```

The timing convention only changes the denominator into which each slice's gain or
loss is allocated: a start-of-day assumption divides by `V_{t-1} + C` (Eq. 3.24),
and a midday half-weight assumption (Eq. 3.25) puts `C/2` into both numerator and
denominator — but the half-weight day is a per-day money-weighted return, so that
hybrid "ceases to be a true time-weighted rate of return." For the unit-price variant
(Eq. 3.26), the intermediate `NAV_i` terms telescope:

```
NAV_1   NAV_2         NAV_{n-1}   NAV_E     NAV_E
----- x ----- x ... x --------- x ------- = ------- = 1 + r
NAV_S   NAV_1         NAV_{n-2}   NAV_{n-1}  NAV_S
```

so only the endpoint unit prices are needed. With distributions, an adjusted unit
price `NAV'_n = NAV_n * prod_i (1 + D_i/NAV_i)` reinvests income before the ratio is
taken (Eq. 3.27). Bacon asserts the equivalence of the unit-price and classical
methods by construction and exhibit, not by separate proof; this card asserts it at
the same level of rigor.

**Source:** Bacon (2023) §3 Eqs. 3.23-3.27 pp.56-59

## Boundary Notes

Chain-linking here is the *intra-period* compounding across cash-flow slices that
defines a single TWR figure. It is distinct from *inter-period* linking of already-
computed periodic returns (and from the geometric-vs-arithmetic linking choice in
multi-period attribution), which is governed by separate identities. Mixing a
per-day money-weighted (midday half-weight) component into the slice return breaks
the "pure" TWR property.

**Source:** Bacon (2023) §3 (Note on Eq. 3.25 hybrid) pp.56-57

## See Also

- [`pa-twr-vs-mwr-when-each-applies.md`](pa-twr-vs-mwr-when-each-applies.md) — when control/liquidity of flows dictates TWR vs MWR.
- [`pa-dietz-methods-mwr-approximations.md`](pa-dietz-methods-mwr-approximations.md) — Dietz as a money-weighted approximation, the foil to true TWR.
- [`pa-multiperiod-linking-smoothing-vs-linking.md`](pa-multiperiod-linking-smoothing-vs-linking.md) — inter-period linking of periodic returns, contrasted with intra-period chain-linking.
- [`pa-gips-2020-composites-and-mechanics.md`](pa-gips-2020-composites-and-mechanics.md) — GIPS valuation/large-flow rules driving the daily-valuation mindset.

## Escalate to Raw When

- You need the worked timing-assumption exhibits (Exhibits 3.11-3.13: start-/end-/
  midday-flow returns of roughly -9.44%, -9.93%, -9.63% on the same loss) to see how
  the denominator allocation shifts the slice returns.
- You need the unit-price worked example reconciling NAV ratio to classical TWR
  (Exhibit 3.14) or the distribution-adjusted pooled-fund example (Exhibit 3.15).
- You need to quantify the valuation-error sensitivity (Exhibit 3.17 shows a wrong
  intra-period valuation produces a permanent return distortion).
- You need exact day-count weighting policy for a cash flow received mid-month
  (start-/end-of-day day-count conventions).

**Source:** Bacon (2023) §3 Exhibits 3.11-3.17 pp.57-62
