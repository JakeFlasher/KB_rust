---
schema_version: "cacg.v0"
id: "fi-key-rate-and-partial-duration"
title: "Key-Rate and Partial Duration"
reading_id: "06_fixed_income_and_credit"
summary: "Key-Rate and Partial Duration — CFA Vol.5/pp.275-310 (PDF 2912-2947) is in derivatives readings (R47-R49); key-rate/partial-duration content not found there. CFA L1 FI duration treatment lives in Vol.5/R43 (~pp.5-50)."
tags: ["fixed-income", "key-rate"]
citations:
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p169:0208"
    chunk_hash: "14356156a0954d1d49c01f9fd6e1f7080bfdfdfc9d8dbe0eccfc46df43276b6d"
    page_range: [169, 170]
    quote: "Key-rate exposures are used for measuring and hedging the risk of bond portfolios in terms of a relatively small number of the most liquid bonds available, usually the most recently issued, near-par,"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p097:0143"
    chunk_hash: "b93fa00bd175ccfa216e18798de1b2baa62758d42883c5496a0fd8627b1be36b"
    page_range: [97, 98]
    quote: "Finally, it explains the use of duration and convexity measures to determine the sensitivity of bond prices to interest rate changes."
    edge_type: "supports"
card_hash: "21498a1a56f6248b03cff4849c9e79b4c800a162ee1a35eb02fa02ee7c7bdfdc"
---
# Key-Rate and Partial Duration

## Intuition

Modified duration assumes a parallel yield-curve shift —
all rates change by the same `Δy`. Real curves rarely
move that way: the front end and the back end can move
differently (twist), or the middle can move opposite to
the wings (butterfly). Key-rate duration measures
sensitivity to a shift at one specific tenor while
holding others fixed; partial duration is the
generalization to a portfolio of bond positions.
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.275-310.

```
yield (%)
   ^
   |        twist: front rises, back falls
   |  before        ____
   |  -------*  *  /    \  *  *
   |       *      /  *  *
   |     *      /         <-- 5y key-rate shock
   |   *      /
   |  *
   |
   +-----------------------------------> tenor
   1y  2y  3y  5y  7y  10y  20y  30y
   key-rate duration measures sensitivity to one
   tenor's shock, holding others constant.
```

## Definition

Key-rate duration `KRD_i` is the bond's price sensitivity
to a 1-basis-point shift at the `i`-th key-rate node,
holding all other key rates fixed:
`KRD_i = -(∂P / ∂y_i) / P`. The standard set of key-rate
nodes is `{1y, 2y, 3y, 5y, 7y, 10y, 20y, 30y}` though
practitioners pick benchmarks aligned with their hedging
instruments. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.275-310.

The sum of key-rate durations equals the modified
duration under a parallel shift: `D_mod = ∑_i KRD_i`.
This recovers the parallel-shift case as a special-case
restriction. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.275-310.

Partial duration generalizes the concept to a portfolio:
each position contributes its own key-rate durations
weighted by position size; the portfolio's net key-rate
duration vector is the aggregate exposure to each tenor.
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.295-310.

## Mathematical Reasoning

For a non-parallel shift with Δy_i at each tenor, the
price change is approximately
`ΔP / P ≈ -∑_i KRD_i · Δy_i`. This recovers the parallel-
shift duration approximation when all `Δy_i = Δy` are
equal. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.275-310.

A duration-matched portfolio (matched to a benchmark's
modified duration) is NOT immunized against twist or
butterfly shifts. Immunization across non-parallel
shifts requires matching the entire key-rate-duration
vector, not just the scalar sum. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.295-310.

The forward-curve algebra of
[`fi-spot-par-forward-curves.md`](./fi-spot-par-forward-curves.md#mathematical-reasoning)
provides the natural change-of-basis: a single key-rate
shift maps to a band of forward-rate adjustments. The
mapping clarifies why a 5y key-rate shock produces an
asymmetric forward-rate response — it raises forward
rates only between the 4y and 6y horizons, leaving
adjacent forwards roughly unchanged. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.275-310.

The basic duration view of
[`fi-duration-and-convexity.md`](./fi-duration-and-convexity.md#definition)
is the special case where all key-rate durations move
together; key-rate duration is the strict
generalization that survives non-parallel shifts.
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.275-310;
Hull §4 pp.92-110.

## See Also

- [`fi-duration-and-convexity.md`](fi-duration-and-convexity.md) — parallel-shift duration as the special case
- [`fi-spot-par-forward-curves.md`](fi-spot-par-forward-curves.md) — forward-curve algebra mapping key-rate shocks to forward-rate bands

## Escalate to Raw When

Open CFA L1 Curriculum Vol.5 Reading 45 directly when
any of the criteria below applies. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.275-310.

- A factor-model decomposition (level / slope /
  curvature factors) of the curve is needed; this card
  treats key rates as the primitive. **Source:** CFA
  L1 Curriculum (2022) Vol.5/pp.275-310.
- Multi-currency hedging across cross-currency basis is
  required; this card stays in single-currency space.
  **Source:** CFA L1 Curriculum (2022) Vol.5/pp.275-310.
- Stochastic-volatility curve dynamics drive the
  hedging horizon; this card uses point-in-time partial
  derivatives. **Source:** Hull §4 pp.92-110.
