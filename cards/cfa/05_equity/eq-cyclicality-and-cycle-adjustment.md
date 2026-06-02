---
schema_version: "cacg.v0"
id: "eq-cyclicality-and-cycle-adjustment"
title: "Cyclicality and Cycle Adjustment"
reading_id: "05_equity"
summary: "Cyclical firms' reported earnings swing over a multi-year cycle; current-year multiples are misleading. Cycle adjustment normalizes earnings to mid-cycle profitability via averaged historical earnings, normalized margins, or mid-cycle return on capital. CAPE is the canonical instance. Damodaran's Ch.22 covers cyclical-firm normalization; CFA R37 covers cyclicality classification."
tags: ["equity", "cyclicality-cycle"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p386:0473"
    chunk_hash: "70915e3d162fa5a553f0ff16844eba5a41f7474e3019476d5864996300452277"
    page_range: [386, 387]
    quote: "The arithmetic mean weights percentage changes in earnings in each period equally and ignores compounding effects in earnings."
    edge_type: "supports"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p808:1061"
    chunk_hash: "ca79fc6bf21f7ee522f827f4ac5129c03db79210a316b1b108768ae71708ed89"
    page_range: [808, 808]
    quote: "for commodity firms, where cyclical movements in commodity prices can affect earnings, we argue for the use of normalized earnings in valuation."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2248:3300"
    chunk_hash: "e5ab0a7384a4a9b97a9e011e41a26a2b72a24728372013f43c408fd7bd05f127"
    page_range: [2248, 2249]
    quote: "One limitation of the cyclical/non-cyclical classification is that business-cycle sensitivity is a continuous spectrum rather than an “either/or” issue"
    edge_type: "supports"
card_hash: "31686f52fa08ce8dea1ea3fb4255b4c085e2ec391aa47edc6c15c9d4890b2bb1"
---
# Cyclicality and Cycle Adjustment

## Intuition

A cyclical firm's reported earnings swing dramatically over a multi-
year cycle: high during the upswing, low or negative during the
trough. Reading a current-year multiple off these swinging earnings
gives a misleading picture — a low P/E during a peak earnings year
is not cheap, and a high P/E during a trough year is not expensive.
The market knows this; the multiple compresses at peaks and expands
at troughs to compensate. The analyst's job is to value the firm
through the cycle, not at a moment in the cycle. **Source:**
Damodaran (4ed) Ch.22 pp.808-844.

Cycle adjustment normalizes the cyclical input — earnings, margins,
return on capital — to a mid-cycle level that reflects the firm's
average performance over the full cycle. The same DCF and multiples
machinery developed in earlier cards then operates on the normalized
inputs. The normalized valuation is more stable through the cycle
than the unadjusted valuation; that is the whole point. **Source:**
Damodaran (4ed) Ch.22 pp.808-844.

```
reported earnings over time
       *
      * *           peak (cycle high)
     *   *
    *     *  *                        *
   *       **          *  *          * *
  *                 *      *        *
                  *          *  *  *
                                          trough (cycle low)
   |   |   |   |   |   |   |   |   |   |
   t1  t2  t3  t4  t5  t6  t7  t8  t9  t10
                                          --> cycle period
                                              spans many years

  cycle-adjusted earnings (normalization)
  ------------------------------------ <-- mid-cycle line
                                          (average through cycle;
                                          discount or value off
                                          THIS line, not the
                                          observed peaks / troughs)
```

## Definition

Cyclicality is the tendency of a firm's earnings or cash flows to
move with a multi-year economic or commodity cycle. Cyclical firms
include commodity producers (oil, mining, steel), heavy industrials
(autos, capital equipment), construction-and-housing, and certain
financials. A cyclical firm's earnings stream over the cycle is the
true measure of its earning power; any single year is a sample from
that distribution. **Source:** Damodaran (4ed) Ch.22 pp.808-844.

Cycle adjustment is the family of normalization techniques that
replace cyclical inputs with through-the-cycle equivalents. The
three main approaches in Damodaran's frame: (a) average historical
earnings over a full cycle (the Shiller-style smoothing); (b)
average historical margin applied to current revenues (preserves
the firm's current scale while normalizing profitability); (c)
mid-cycle return on capital combined with current invested capital
(preserves the firm's current capital base while normalizing
profitability). **Source:** Damodaran (4ed) Ch.22 pp.808-844.

The DCF-cycle dispatch carries through to the cyclical case. The
explicit-forecast period should span at least one full cycle so the
model captures both peak and trough years; a forecast that spans
only the upswing or only the downswing inherits the bias of the
selected window. The terminal-value form should use mid-cycle
profitability — applying peak margins to a perpetuity terminal
value embeds peak-cycle conditions forever, and applying trough
margins does the opposite. **Source:** Damodaran (4ed) Ch.22 pp.808-844.

The cycle-adjusted multiple — applying a peer-multiple central
tendency to a cycle-adjusted scale variable — bridges relative
valuation to through-the-cycle thinking. The Shiller cyclically-
adjusted P/E (CAPE) is the canonical instance: P/E computed on a
ten-year average of inflation-adjusted earnings rather than current
earnings. The same logic generalizes to EV/EBITDA on smoothed
EBITDA, P/B on cycle-adjusted book value, and so on. **Source:**
Damodaran (4ed) Ch.22 pp.808-844.

The money-losing-firm extension in Damodaran Ch.22 covers the
related case of firms with currently negative earnings — whether
because of cyclical trough, life-cycle stage, or distress.
Normalization in this case is more aggressive: forecast revenues
through a recovery, apply mid-cycle (or industry) margins, and
recover an implied earning power. The cyclical-trough sub-case
overlaps with this card; the life-cycle-stage and distress sub-
cases are deferred to future work. **Source:** Damodaran (4ed) Ch.22 pp.808-844.

## Mathematical Reasoning

The cycle-adjusted earnings approach takes the average of a firm's
historical earnings over a window of length `K` years that spans at
least one full cycle: `Earnings_normal = (1/K) · sum_{i=1..K}
Earnings_i`. The averaging window must be chosen carefully — too
short and the average inherits cycle bias, too long and the firm's
underlying business may have changed. **Source:** Damodaran (4ed) Ch.22 pp.808-844.

The margin-normalization approach preserves the firm's current scale
while normalizing profitability: `Earnings_normal = Revenue_current
· margin_normal`, where `margin_normal` is the firm's average
operating margin (or peer-set average margin) over the full cycle.
The form is symbolic: it expresses what the firm would earn if
current revenues persisted at mid-cycle profitability. **Source:**
Damodaran (4ed) Ch.22 pp.808-844.

The return-on-capital-normalization approach preserves the firm's
current capital base while normalizing profitability:
`Earnings_normal = Invested_Capital_current · ROC_normal`, where
`ROC_normal` is the firm's average return on capital (or peer-set
average ROC) over the full cycle. This form is preferred when the
firm has grown its capital base substantially over the cycle, so
the average-historical-earnings approach would understate the
normalized earning power. **Source:** Damodaran (4ed) Ch.22 pp.808-844.

The DCF-cycle interaction works through the explicit-forecast
schedule. The period-by-period cash flows should reflect actual
cycle position (high in upswing years, low in downswing years), not
mid-cycle smoothing applied uniformly — that distorts the timing
of cash flows. The terminal value beyond the explicit horizon, in
contrast, must use mid-cycle profitability because the perpetuity
form embeds the chosen condition forever. The cycle-adjustment
discipline: explicit period reflects the cycle, terminal value
reflects mid-cycle. **Source:** Damodaran (4ed) Ch.22 pp.808-844.

The reinvestment-rate identity from
[`eq-payout-policy-and-growth`](./eq-payout-policy-and-growth.md)
applies cycle-adjusted as well: stable-state growth in the terminal
year requires reinvestment consistent with mid-cycle ROC, not peak
or trough ROC. Mismatching cycle-position assumptions between the
growth rate and the reinvestment rate produces an internal
inconsistency that no amount of explicit-forecast detail can repair.
**Source:** Damodaran (4ed) Ch.22 pp.808-844.

The Shiller-CAPE generalization writes the cycle-adjusted multiple
as `CAPE = Price / Earnings_normal`, with the central observation
that CAPE is more stable through the cycle than current-year P/E and
better-suited as a sector-mean-reversion diagnostic. The peer-set
analogue replaces the time-average with a cross-sectional median or
trimmed mean of normalized earnings across cycle peers. **Source:**
Damodaran (4ed) Ch.22 pp.808-844.

The CFA L1 frame addresses cyclicality in the industry-and-company
analysis reading: cyclical industries are characterized by demand
sensitivity to the business cycle, the analyst should distinguish
secular from cyclical drivers, and forecasting through the cycle
requires explicit treatment of cycle position. **Source:** CFA L1
Curriculum (2022) Vol.4/pp.307-360.

## See Also

- [`eq-fcfe-fcff-decomposition`](./eq-fcfe-fcff-decomposition.md) — the cash-flow definitions that get cycle-adjusted
- [`eq-payout-policy-and-growth`](./eq-payout-policy-and-growth.md) — the reinvestment-rate identity that ties cycle-adjusted growth to cycle-adjusted ROC
- [`eq-pe-and-relative-valuation`](./eq-pe-and-relative-valuation.md) — the relative-valuation context where the cycle-adjusted multiple (CAPE) sits

## Escalate to Raw When

Open Damodaran Ch.22 directly when any of the criteria below
applies. **Source:** Damodaran (4ed) Ch.22 pp.808-844.

- the cycle period is unusually long (decade-plus commodity supercycle, structural-rate-regime cycle, demographic / housing cycle) and the analyst needs guidance on the smoothing-window choice — Damodaran Ch.22 develops the window-selection criteria. **Source:** Damodaran (4ed) Ch.22 pp.808-844.
- the firm is currently money-losing for cyclical reasons and the analyst needs the negative-earnings normalization machinery — Damodaran Ch.22 develops the recovery-forecast and margin-normalization techniques in detail. **Source:** Damodaran (4ed) Ch.22 pp.808-844.
- the firm's capital base has grown materially over the cycle, making historical-average-earnings normalization understate current earning power — Damodaran Ch.22 develops the ROC-normalization alternative. **Source:** Damodaran (4ed) Ch.22 pp.808-844.
