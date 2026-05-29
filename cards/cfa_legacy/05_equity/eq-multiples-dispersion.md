---
schema_version: "cacg.v0"
id: "eq-multiples-dispersion"
title: "Multiples Dispersion"
reading_id: "05_equity"
summary: "Framing the cross-firm spread of price multiples within a peer set — how dispersion decomposes into fundamentals-explained heterogeneity vs idiosyncratic noise, why the central tendency loses information when dispersion is wide, and how the book-value family inherits an additional accounting-policy spread on top of economic dispersion."
tags: ["equity", "multiples-dispersion"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p636:0834"
    chunk_hash: "4f3048b6e7ae999c682b3510be9b962482c371bbb408386b70a343742e0b4ae2"
    page_range: [636, 637]
    quote: "A critical step in using PE ratios is to understand how the cross-sectional multiple is distributed across firms in the sector and the market."
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p692:0901"
    chunk_hash: "1137cea4bb22fc8e687570fe04e604121031c77e2f4081b585f669fa9718642c"
    page_range: [692, 693]
    quote: "Note that this distribution is heavily skewed, as is evidenced by the fact that the average price-to-book ratio for U.S. (global) firms is 9.94 (6.15)"
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2321:3403"
    chunk_hash: "a5005d6c544845659e17152e468a5d04728454d50d6cef4d2ef5158b43c12264"
    page_range: [2321, 2322]
    quote: "Many practitioners use ratios when examining a group or sector of stocks and consider the shares for which the ratio is relatively low to be attractively valued securities."
    edge_type: "supports"
card_hash: "e6b1d75dc331519b57a81e40501f5ee9667041528d19ca7c1f7d161d6b5453a8"
---
# Multiples Dispersion

## Intuition

Even within a tight peer set — same industry, similar scale,
comparable business model — multiples scatter substantially across
firms. P/E ratios commonly span a wide multiplicative range between
the cheapest and richest peer in the same sector; P/B ratios span
wider still. Dispersion is the cross-firm spread of multiples around
the sector central tendency, and its width is a diagnostic of how
much information the central tendency loses. **Source:** Damodaran
(2012) Ch.18 pp.635-689.

The first question dispersion answers is structural: how much of
the spread reflects differences in fundamentals (growth, payout,
risk, leverage, profitability) versus how much is idiosyncratic
noise that should mean-revert. A high-dispersion sector with
fundamentals-explained spread is not a signal that any single peer
is mispriced — it is a signal that the peer set is heterogeneous on
fundamentals and the comparable-company exercise should price each
peer on its own fundamentals, not on the sector mean. A high-
dispersion sector with idiosyncratic-noise spread is the opposite
signal: the central tendency carries information and outliers
likely revert. **Source:** Damodaran (2012) Ch.18 pp.635-689.

```
multiple distribution within sector

count
   ^                  *
   |               * * *               -- peer mean
   |             * * * * *             -- peer median
   |          * * *   * * *
   |       * * *   |   * * *
   |    * * *      |      * * *
   |   *           |           *
   |   *           |           *  <-- outliers may be:
   |               |              (a) explained by fundamentals
   |               |                  (different growth/risk/payout)
   |               |              (b) noise (idiosyncratic, mean-reverts)
   |               |              (c) mispricing (revert as info diffuses)
   +---|-----------|-----------|------> multiple level
       low         center      high
```

## Definition

Dispersion is the cross-firm distribution of a price multiple
within a defined peer set, measured by spread statistics —
interquartile range, standard deviation, or 90th-vs-10th percentile
gap. Two sectors with the same central tendency can have very
different dispersion: one with a wide IQR-equivalent spread carries
more uncertainty when the analyst values a target against the
median, while one with a narrow spread carries less. The central
tendency alone is not a sufficient summary. **Source:** Damodaran
(2012) Ch.18 pp.635-689.

The fundamentals-explained component of dispersion comes from cross-
firm differences in the drivers of justified multiples. The justified
P/E identity (see
[`eq-pe-and-relative-valuation`](./eq-pe-and-relative-valuation.md))
expresses P/E as a function of payout, growth, and cost of equity;
firms with higher growth, higher payout, or lower risk should trade
at higher P/E. Cross-firm spread that aligns with cross-firm spread
in these drivers is fundamentals-explained — it is rational
heterogeneity, not mispricing. **Source:** Damodaran (2012) Ch.18
pp.635-689.

The idiosyncratic / residual component is what remains after
fundamentals-driven spread is removed. This component captures
short-horizon noise, transient sentiment, asymmetric information,
and other firm-specific deviations. Outliers in the residual
component are candidates for mispricing analysis — a firm whose
multiple is an extreme positive residual relative to the residual
distribution has either a fundamental driver the analyst has not
yet identified or a mispricing that may revert. **Source:**
Damodaran (2012) Ch.18 pp.635-689.

The book-value family (P/B, EV/Capital) tends to show wider
dispersion than the earnings family (P/E, EV/EBIT) because book
values reflect accumulated investment under varying accounting
treatments (depreciation policies, impairment timing, acquisition
goodwill), and the cross-firm comparability of book values is
weaker. The revenue family (P/Sales, EV/Sales) shows different
dispersion patterns because cross-firm margin variation drives the
spread. **Source:** Damodaran (2012) Ch.19 pp.690-725.

The multiple's denominator volatility also drives dispersion: P/E
ratios are more volatile near earnings troughs (where small
denominators amplify ratios) and become misleading for cyclical or
loss-making firms. Damodaran's recommendation is to use the most
stable normalized denominator available — trailing-12-month
earnings smoothed across a cycle for P/E, average-of-recent-quarters
EBITDA for EV/EBITDA — to reduce denominator-driven dispersion that
is not informative. **Source:** Damodaran (2012) Ch.18 pp.635-689.

## Mathematical Reasoning

Sector-multiple dispersion is the cross-firm distribution of
`multiple_j` for `j = 1..K` peer firms within the sector. The most
common spread statistics are interquartile range `IQR =
multiple_75 - multiple_25` and the 90/10 spread `multiple_90 -
multiple_10`. Standard deviation is also used but is sensitive to
extreme outliers, which are common in multiples data because
distributions are right-skewed (the `multiple = price / fundamental`
ratio is unbounded above when the fundamental approaches zero).
**Source:** Damodaran (2012) Ch.18 pp.635-689.

The fundamentals-explained component is captured by the justified-
multiple identity from
[`eq-pe-and-relative-valuation`](./eq-pe-and-relative-valuation.md).
The justified P/E in symbolic form expresses the multiple as a
function of payout, growth, and cost of equity. **Source:** Damodaran
(2012) Ch.18 pp.635-689.

```
(P/E_1)_justified = payout / (r - g)
```

Cross-firm dispersion in the justified multiple equals the dispersion
that fundamental-driver dispersion would imply if all peers' actual
multiples sat at their justified level. The fundamentals-explained
share of total dispersion is then the ratio of fundamental-driver-
implied spread to total spread; the idiosyncratic share is the
complement. **Source:** Damodaran (2012) Ch.18 pp.635-689.

Damodaran's intuition-level decomposition (the regression-based
machinery is deferred to future-01 per DEC-1) writes each peer's
multiple as a justified-from-fundamentals component plus a residual.
**Source:** Damodaran (2012) Ch.18 pp.635-689.

```
multiple_j = (justified multiple from fundamentals)_j  +  residual_j

dispersion(multiple) = dispersion(justified-from-fundamentals)
                     + dispersion(residual)
                     + 2 · cov(...)
```

The two-way decomposition is approximate (the variance equality
ignores the covariance term and assumes orthogonality of fundamentals
and residuals), but it is the L1-depth intuition that captures the
diagnostic role: most of the cross-section sits inside the
fundamentals-explained band, and the residual outliers are the
candidates for further investigation. **Source:** Damodaran (2012)
Ch.18 pp.635-689.

The book-value family inherits an additional dispersion source: the
accounting-policy spread. Firms using accelerated depreciation will
report lower book values and higher P/B than firms using straight-
line depreciation, holding economic value constant. The dispersion in
P/B that reflects accounting-policy heterogeneity is not informative
about valuation — it is informative about accounting. Adjusting book
values to a consistent accounting basis (e.g., adjusting for goodwill
write-offs, lease capitalization conventions) reduces this
component. **Source:** Damodaran (2012) Ch.19 pp.690-725.

The CFA L1 frame presents multiples dispersion as the cross-firm
spread within a sector, identifies fundamentals-driven heterogeneity
as the primary source, and emphasizes that the sector central
tendency loses information when dispersion is wide. **Source:** CFA
L1 Curriculum (2022) Vol.4/pp.361-416.

## See Also

- [`eq-pb-and-multiples-taxonomy`](./eq-pb-and-multiples-taxonomy.md) — book-value, revenue, and sector-specific multiple families whose dispersion is being analyzed
- [`eq-comparable-company-analysis`](./eq-comparable-company-analysis.md) — the peer-set construction that defines the dispersion universe
- [`eq-pe-and-relative-valuation`](./eq-pe-and-relative-valuation.md) — the justified-P/E identity that supplies the fundamentals-explained baseline

## Escalate to Raw When

Open Damodaran Ch.18 / Ch.19 directly when any of the criteria below
applies. **Source:** Damodaran (2012) Ch.18 pp.635-689.

- the dispersion in the peer set is unusually wide and the analyst needs the full fundamentals-explained-vs-idiosyncratic decomposition framework — Damodaran Ch.18 develops this in detail. **Source:** Damodaran (2012) Ch.18 pp.635-689.
- the dispersion source is suspected to be accounting-policy heterogeneity rather than economic heterogeneity — Damodaran Ch.19 covers the book-value adjustments needed for cross-firm comparability. **Source:** Damodaran (2012) Ch.19 pp.690-725.
- the analyst is moving beyond intuition-level dispersion analysis and needs Damodaran's regression-based fundamentals-to-multiples link — Damodaran Ch.18 sketches the regression form; the depth of regression-based econometric machinery is deferred to future-01 per DEC-1. **Source:** Damodaran (2012) Ch.18 pp.635-689.
