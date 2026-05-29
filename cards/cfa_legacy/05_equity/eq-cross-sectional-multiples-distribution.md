---
schema_version: "cacg.v0"
id: "eq-cross-sectional-multiples-distribution"
title: "Cross-Sectional Multiples Distribution"
reading_id: "05_equity"
summary: "Cross-sectional multiples distributions are typically long-tailed right-skewed with negative-P/E firms truncated. Damodaran develops the regression of multiples on fundamentals (growth, payout, cost of equity) as the empirical generalization of the justified-P/E identity, providing a universe-wide benchmark complementary to within-peer-set comparisons."
tags: ["equity", "cross-sectional"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p621:0814"
    chunk_hash: "ab6c3446c339315dc1e872e4f2f901da47ce8e20dd2d024644dd248c5872e044"
    page_range: [621, 622]
    quote: "What is often lacking, however, is a sense of how the multiple is distributed across the entire market."
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p622:0815"
    chunk_hash: "9072e9eb8169ee385289f4fd631c4b8458709f027b53a2fd5b41476d8e7650a0"
    page_range: [622, 623]
    quote: "In fact, the sensitivity of the estimated average to outliers is another reason for looking at the median values for multiples."
    edge_type: "supports"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p636:0834"
    chunk_hash: "4f3048b6e7ae999c682b3510be9b962482c371bbb408386b70a343742e0b4ae2"
    page_range: [636, 637]
    quote: "A critical step in using PE ratios is to understand how the cross-sectional multiple is distributed across firms in the sector and the market."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2331:3416"
    chunk_hash: "bd4eb7d1f28f3748abeaa7cbf7aef010bb5b10bef5fed3c30e6a8d7110672081"
    page_range: [2331, 2331]
    quote: "Furthermore, differences in reporting rules among different markets and in chosen accounting methods can result in revenues, earnings, book values, and cash flows that are not easily comparable."
    edge_type: "supports"
card_hash: "0c623338c6df12dcbd17ae2e5724b4d687289451c4fe9e19605548f2f5960f2a"
---
# Cross-Sectional Multiples Distribution

## Intuition

When the analyst widens the lens from a tight peer set to a full
sector or full-market universe of many firms, the distribution of
multiples becomes a population-level object. The cross-sectional
distribution of P/E across all listed equities at any point in
time is not a tight cluster around
a sector mean — it is a long-tailed right-skewed distribution with
a substantial mass of negative-P/E firms (loss-makers), a wide bulk
near the sector medians, and a thin tail of very high P/E firms.
**Source:** Damodaran (2012) Ch.17 pp.616-634.

The cross-sectional perspective answers a different question than
the within-peer-set dispersion of `eq-multiples-dispersion`. The
within-peer question is: among similar firms, how much do multiples
spread? The cross-sectional question is: across the universe, what
explains why some firms trade at much higher multiples than others?
The two are linked — the within-peer spread is a slice of the
cross-sectional spread — but the cross-sectional view emphasizes
the relationship between multiples and fundamentals at scale.
Damodaran's empirical observation is that, at L1 intuition depth,
the relationship is approximately linear in the right transformations
of fundamental drivers. **Source:** Damodaran (2012) Ch.17
pp.616-634.

```
cross-sectional plot of P/E vs growth (g)

P/E
   ^
   |                                    *
   |                            *  *
   |                                                <-- visible
   |                       *  *  *                      positive slope
   |                  *  *  *                           (P/E increases
   |              *  *  *  *                            with g)
   |          *  *  *  *
   |       *  *  *  *
   |     *  *  *  *
   |    *  *
   |   *
   |
   +-----------------------------------------------> growth rate g

   each * = one firm in the cross-section; cloud of points has
   visible upward slope plus residual scatter that is the
   firm-specific deviation from the cross-sectional mean line
```

## Definition

The cross-sectional multiples distribution is the empirical
distribution of a price multiple across the universe of firms (or
sector universe) at a given point in time. Damodaran's three
defining properties: right-skew (long tail toward high multiples),
truncation at zero or below for ratio multiples (P/E becomes
negative or undefined for loss-makers), and approximately log-
normal shape for ratio multiples computed on positive denominators.
**Source:** Damodaran (2012) Ch.17 pp.616-634.

The cross-sectional fundamentals-to-multiples link is Damodaran's
empirical generalization of the justified-multiple identity. Where
the within-peer justified P/E says `(P/E)_justified = payout / (r -
g)` for a single firm, the cross-sectional version regresses
observed multiples across all firms on the firms' fundamental
drivers (growth, payout, cost of equity, leverage, profitability)
and recovers a linear-in-fundamentals approximation that holds on
average. The intercept and slopes of the cross-sectional regression
characterize the market-wide fundamentals-to-multiples mapping at
that point in time. **Source:** Damodaran (2012) Ch.18 pp.635-689.

The depth of the regression-based machinery — Fama-MacBeth two-pass
regressions, errors-in-variables corrections, panel-vs-cross-
section choice, return-prediction econometrics — is deferred to
future-01 per DEC-1. The L1-intuition-depth statement is: a
cross-sectional regression of P/E on growth and payout, run across
the universe of firms at a single date, recovers a positive
coefficient on growth and a positive coefficient on payout, with
residuals that capture the firm-specific deviation from the
universe's average fundamentals-to-multiples mapping. **Source:**
Damodaran (2012) Ch.18 pp.635-689.

The cross-sectional benchmark is an alternative to the within-peer
benchmark for relative valuation. The within-peer approach values a
target against the median of a tight set of similar firms (see
[`eq-comparable-company-analysis`](./eq-comparable-company-analysis.md)).
The cross-sectional approach values the target against the
universe-wide regression line: `predicted multiple_target =
intercept + slopes · fundamentals_target`. The two benchmarks
disagree when the target's sector trades systematically rich or
cheap relative to the universe; the within-peer view says "the
target is in line with peers" while the cross-sectional view says
"the entire peer set is rich relative to the universe." Damodaran
recommends running both and triangulating. **Source:** Damodaran
(2012) Ch.17 pp.616-634.

## Mathematical Reasoning

The cross-sectional distribution of a multiple `M_j` for `j = 1..K`
firms in the universe has, in Damodaran's L1-intuition-depth
characterization, the following moment summary at a point in time:
median, mean, standard deviation, skewness, and kurtosis. The
median is robust to the right-tail outliers that drive the mean;
the mean is sensitive to extreme values; the standard deviation
captures total spread; skewness captures the right-tail asymmetry;
kurtosis captures the heaviness of the tails. Truncation at the
loss-maker boundary (negative-denominator firms drop out of the
P/E distribution) means the visible distribution is a conditional
distribution on positive-earnings firms. **Source:** Damodaran
(2012) Ch.17 pp.616-634.

The fundamentals-to-multiples cross-sectional regression, at
L1-intuition-depth, expresses each firm's multiple as a linear
combination of fundamental drivers plus a firm-specific residual
(regression estimation depth deferred to future-01 per DEC-1).
**Source:** Damodaran (2012) Ch.18 pp.635-689.

```
P/E_j  =  alpha  +  beta_g · g_j  +  beta_payout · payout_j
       +  beta_r · r_j  +  beta_leverage · leverage_j  +  e_j

for j = 1..K firms in the universe; e_j is the firm-specific
residual (the cross-sectional analogue of dispersion-residual)
```

The expected signs of the coefficients are: `beta_g > 0` (higher
growth supports higher P/E), `beta_payout > 0` (higher payout
supports higher P/E because more of the value-driver is already
distributed), `beta_r < 0` (higher cost of equity reduces P/E),
`beta_leverage` can be either sign depending on whether leverage
is contributing return-on-equity or financial risk to the cross-
section. **Source:** Damodaran (2012) Ch.18 pp.635-689.

The cross-sectional residual `e_j` is the deviation of firm `j`'s
multiple from what the universe-wide regression line would predict
given the firm's fundamentals. Outliers in `|e_j|` are candidates
for further investigation: firms with `e_j > 0` (multiple above
cross-sectional prediction) are rich relative to the universe-wide
fundamentals-to-multiples mapping; firms with `e_j < 0` are cheap.
This residual is the cross-sectional analogue of the within-peer
residual from [`eq-multiples-dispersion`](./eq-multiples-dispersion.md);
the difference is that the within-peer residual benchmarks against
sector peers, while the cross-sectional residual benchmarks against
the universe. **Source:** Damodaran (2012) Ch.18 pp.635-689.

The dispersion-vs-cross-sectional split has a practical implication
for relative-valuation reporting: the within-peer view is more
sensitive to peer-set selection but more stable under macro regime
shifts (the sector central tendency moves with the sector); the
cross-sectional view is less sensitive to peer-set construction
but more sensitive to macro regime shifts (the universe-wide
intercept moves with the market). Reporting both gives the analyst
a triangulation that is more robust than either alone. **Source:**
Damodaran (2012) Ch.17 pp.616-634.

The CFA L1 frame presents the cross-sectional fundamentals-to-
multiples link at intuition depth, identifies regression as the
empirical tool that recovers it, and emphasizes that the regression
is a one-snapshot benchmark, not a time-series prediction. **Source:**
CFA L1 Curriculum (2022) Vol.4/pp.361-416.

## See Also

- [`eq-multiples-dispersion`](./eq-multiples-dispersion.md) — the within-peer-set spread that the cross-sectional view generalizes to the universe
- [`eq-pe-and-relative-valuation`](./eq-pe-and-relative-valuation.md) — the justified P/E identity that the cross-sectional regression empirically generalizes
- [`eq-comparable-company-analysis`](./eq-comparable-company-analysis.md) — the within-peer-benchmark alternative to the cross-sectional benchmark

## Escalate to Raw When

Open Damodaran Ch.17 / Ch.18 directly when any of the criteria below
applies. **Source:** Damodaran (2012) Ch.17 pp.616-634.

- the cross-sectional distribution shape is non-standard (extreme right-tail mass, multimodal, regime-shifting between time periods) and the analyst needs guidance on interpretation — Damodaran Ch.17 documents the historical shapes of P/E, P/B, P/Sales distributions. **Source:** Damodaran (2012) Ch.17 pp.616-634.
- the cross-sectional regression's coefficient estimates need to be interpreted causally rather than as fitting tools, and the analyst needs the econometric caveats — the regression-estimation econometric depth is deferred to future-01 per DEC-1, but Damodaran Ch.18 surveys the L1 caveats. **Source:** Damodaran (2012) Ch.18 pp.635-689.
- the within-peer and cross-sectional benchmarks disagree materially on the target's relative-valuation position — Damodaran Ch.17 develops the triangulation framework. **Source:** Damodaran (2012) Ch.17 pp.616-634.
