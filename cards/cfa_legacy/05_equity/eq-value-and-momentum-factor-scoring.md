---
schema_version: "cacg.v0"
id: "eq-value-and-momentum-factor-scoring"
title: "Value and Momentum Factor Scoring"
reading_id: "05_equity"
summary: "Scoring securities on value (low P/E, low P/B) and momentum (trailing-return continuation, post-earnings-announcement drift) factors at signals-plus-ranking depth. Damodaran's Ch.6 market-anomaly evidence documents the value and momentum patterns; the cross-sectional ranking maps signals to per-security scores that feed a long-top/short-bottom portfolio construction recipe."
tags: ["equity", "value-momentum"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p222:0240"
    chunk_hash: "f87712011ccef5d126b7bd649dbd6da7e4859d2457c1e5f6b512ada72e3ec7bf"
    page_range: [222, 223]
    quote: "there is a negative relationship between returns and price–book value ratios— low price–book value ratio stocks earn higher returns than high price–book value ratio stocks."
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p634:0832"
    chunk_hash: "8456d0d42f3fd31ecebe018fce372aaa5b3fa53f541ffed8eec409fba6bf71cf"
    page_range: [635, 635]
    quote: "Even market novices have heard of price earnings ratios, and many market strategists make judgments on whether the market is under- or overpriced"
    edge_type: "supports"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p690:0897"
    chunk_hash: "ee01be57ae3e0fdd5d49d093d5e57004313f7e3910a1a2dc4781183b44a86f23"
    page_range: [690, 691]
    quote: "The relationship between price and book value has always attracted the attention of investors."
    edge_type: "supports"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p205:0224"
    chunk_hash: "ea6930dd175ad16df7b31bfd294200b16402c83186f87762a9ae205804e7f0cc"
    page_range: [205, 206]
    quote: "The momentum effect is just as strong in the European markets, though it seems to be weaker in emerging markets."
    edge_type: "supports"
card_hash: "68b7edf5e5884df49ad4b73d8b6ce9e8a77e7918dc1667e584093bf0568f1193"
---
# Value and Momentum Factor Scoring

## Intuition

Damodaran's Ch.6 market-efficiency evidence documents two persistent
cross-sectional return patterns: value securities (low P/E, low P/B,
high B/M) have outperformed growth securities over long horizons,
and momentum securities (those with positive recent price returns
or positive post-earnings-announcement drift) have outperformed
those with negative recent returns. The factor-scoring exercise
turns these patterns into security-level signals an analyst can
use to rank the cross-section. **Source:** Damodaran (2012) Ch.6
pp.183-245.

A factor score is a security's standardized position on a signal
relative to the cross-section: a high value score means the
security is among the cheapest in the universe by P/E or P/B; a
high momentum score means the security has had the strongest recent
price performance or earnings-revision momentum. Scoring is the
intermediate step between raw signals (P/E, trailing-12-month
return) and portfolio construction (long high-score, short low-
score). **Source:** Damodaran (2012) Ch.18 pp.635-689.

```
<!-- primitive: factor-score-grid source: _diagram_primitives.md -->
factor scoring          factor ->
security                value  mom  quality  low-vol  size  | comp
   |
   |  sec 1               +     +      .        .       -   |  +2
   |
   |  sec 2               .     +      +        .       .   |  +2
   |
   |  sec 3               -     .      +        +       .   |  +1
   |
   |  sec 4               +     -      .        .       .   |   0
   |
   |  sec 5               .     .      .        +       +   |  +2
   |
   |  ...                 .     .      .        .       .   |  ..
   |
   |  sec N               -     -      -        -       +   |  -3
   v
                  legend: + high  . neutral  - low
                  composite is conceptual aggregation, not a price
                  ranking method (equal-weight, IC-weighted, etc.)
                  is the card's choice
```

## Definition

A value signal is any cross-sectional measure that orders
securities from "cheap" to "expensive" relative to a fundamental
denominator. Damodaran Ch.6's anomaly evidence centers on two
canonical value signals: low P/E (price-to-earnings, earnings as
the denominator) and low P/B (price-to-book, book value as the
denominator). The value-scoring step assigns each security a
standardized position in the cross-section on each signal: top
quintile of low-P/E gets a high value score; bottom quintile of
low-P/E (i.e., highest P/E firms) gets a low value score.
**Source:** Damodaran (2012) Ch.6 pp.183-245.

A momentum signal is any cross-sectional measure that orders
securities by their recent price-return or earnings-revision
performance. Damodaran Ch.6 documents two main momentum patterns:
short-horizon price-reversal (over horizons of weeks to a few
months, prices revert), and longer-horizon trend continuation
(over horizons of six to twelve months, recent winners continue
to outperform). The momentum-scoring step ranks securities by the
chosen horizon's return: top quintile of trailing-12-month-minus-
last-month return gets a high momentum score; bottom quintile gets
a low momentum score. The "minus-last-month" exclusion captures
the short-horizon-reversal effect by removing the most recent
month's return from the trailing window. **Source:** Damodaran
(2012) Ch.6 pp.183-245.

The post-earnings-announcement drift (PEAD) signal is a momentum-
adjacent signal that measures the cross-sectional spread of returns
in the weeks following an earnings announcement: securities that
beat consensus tend to drift positive for several weeks, and those
that miss tend to drift negative. PEAD is a momentum signal in the
sense that it captures continuation of positive earnings news.
**Source:** Damodaran (2012) Ch.6 pp.183-245.

The cross-sectional ranking step takes a raw signal `s_j` for `j =
1..N` securities and converts it to a score `score_j` that
expresses the security's relative position. Two canonical
conversions: percentile ranking (`score_j = rank(s_j) / N`) and
z-score standardization (`score_j = (s_j - mean(s)) / std(s)`).
The choice between them depends on the analyst's robustness
preference (percentile is robust to outliers; z-score uses the
distribution shape). **Source:** Damodaran (2012) Ch.6 pp.183-245.

The composite-score step aggregates per-factor scores into a
single ranking: equal-weighted (`composite_j = (score_j_value +
score_j_momentum) / 2`); IC-weighted (each factor weighted by its
historical predictive power); risk-adjusted (each factor weighted
inversely to its volatility). The composite-aggregation choice is
the analyst's; Damodaran's Ch.6 evidence does not adjudicate
between specific aggregation rules at the L1 / extension depth.
**Source:** Damodaran (2012) Ch.6 pp.183-245.

The cross-vertical bridge: the value signals (low P/E, low P/B)
inherit from `eq-pe-and-relative-valuation` and
`eq-pb-and-multiples-taxonomy` (the multiples-taxonomy cards from
Batches 1/2). The factor framing inherits from
[`eq-fama-french-construction-at-security-level`](./eq-fama-french-construction-at-security-level.md)
(the value factor is one of the three FF canonical factors). The
upstream asset-pricing-theory framing for cross-sectional anomalies
is in 09's `pm-anomalies-and-cross-sectional-pricing.md`. **Source:**
Damodaran (2012) Ch.6 pp.183-245.

## Mathematical Reasoning

The value scoring on the low-P/E signal in symbolic form ranks the
universe by P/E ascending (low P/E ranked highest as "most value"),
then standardizes the rank into a score. **Source:** Damodaran
(2012) Ch.18 pp.635-689.

```
sort securities by P/E_j ascending  (j = 1..N securities)

  rank_value_j = rank position of security j in the ascending sort
                 (1 = lowest P/E, N = highest P/E)

  score_value_j = either:
                  (a) percentile form:  (N - rank_value_j + 1) / N
                                       (inverse percentile: low P/E
                                        gets the highest score; sign
                                        flipped to match the
                                        cheap-is-high convention)
                  (b) z-score form:    (mean(P/E) - P/E_j) / std(P/E)
                                       (sign flipped so low P/E
                                        = high value score)
```

The same construction applies to low-P/B with B/M (book-to-market)
or its inverse 1/(P/B) as the underlying signal. The choice of
denominator (earnings vs book value) determines which value
"flavor" the scoring captures: P/E-based value tilts toward
earnings-yield momentum; P/B-based value aligns with the canonical
Fama-French HML factor. **Source:** Damodaran (2012) Ch.19
pp.690-725.

The momentum scoring in symbolic form computes the security's
trailing return over a chosen horizon, excludes the most recent
month to remove short-horizon-reversal contamination, ranks
securities by this windowed return, and standardizes. **Source:**
Damodaran (2012) Ch.6 pp.183-245.

```
trailing_return_j_T = product of (1 + monthly_return_j_t) for
                       t in [T-12, T-1]   (12 months ending at T-1,
                                            so excludes month T)

rank_momentum_j = rank position of security j in the descending
                  sort of trailing_return_j_T
                  (1 = highest return = strongest momentum)

score_momentum_j = standardized form of rank_momentum_j
```

The excluded last month captures the short-horizon-reversal
component documented in Damodaran Ch.6: at horizons under one
month, prices revert; the exclusion removes the reversal noise
from the longer-horizon momentum signal. **Source:** Damodaran
(2012) Ch.6 pp.183-245.

The composite scoring in symbolic form aggregates per-factor
scores into a single ranking. The simplest equal-weight composite
takes the per-factor average. **Source:** Damodaran (2012) Ch.6
pp.183-245.

```
composite_j = (1/F) · sum over factors of (score_factor_j)

where F = number of factors aggregated (here: 2 for value+momentum)

ranking by composite_j produces the analyst's ordered universe;
top-quantile securities are long-candidates, bottom-quantile are
short-candidates
```

Damodaran's Ch.6 evidence supports this construction at the
intuition / signals + ranking depth; the deeper question of
WHEN to weight each factor differently (factor-timing) and HOW
to estimate factor IC (information coefficient) is in the
econometric machinery deferred to future-01.
**Source:** Damodaran (2012) Ch.6 pp.183-245.

The portfolio-implementation bridge uses the composite ranking to
construct a long-short portfolio: long the top quantile, short the
bottom quantile (or long-only the top quantile if shorting is not
permitted). The per-period return of the long-short portfolio is
the realized factor premium for the analyst's specific
construction; the long-run mean of these returns is the analyst's
estimate of the expected factor premium. **Source:** Damodaran
(2012) Ch.6 pp.183-245.

## See Also

- [`eq-pe-and-relative-valuation`](./eq-pe-and-relative-valuation.md) — the P/E denominator that supplies the low-P/E value signal
- [`eq-pb-and-multiples-taxonomy`](./eq-pb-and-multiples-taxonomy.md) — the P/B denominator that supplies the low-P/B value signal (and its inverse B/M for the FF HML factor)
- [`eq-fama-french-construction-at-security-level`](./eq-fama-french-construction-at-security-level.md) — the FF construction that the value factor instantiates
- [`pm-anomalies-and-cross-sectional-pricing`](../09_portfolio_management_and_asset_pricing/pm-anomalies-and-cross-sectional-pricing.md) — the cross-sectional anomaly evidence framing upstream in 09

## Escalate to Raw When

Open Damodaran Ch.6 directly when any of the criteria below applies.
**Source:** Damodaran (2012) Ch.6 pp.183-245.

- the cross-sectional anomaly evidence supporting the value or momentum signal is contested for the target market (emerging markets, sector ETFs, niche universes) — Damodaran Ch.6 surveys the historical evidence in detail. **Source:** Damodaran (2012) Ch.6 pp.183-245.
- the analyst needs the historical-vs-implied factor-premium comparison for portfolio construction (analogous to historical-vs-implied ERP) — Damodaran Ch.6 covers the historical evidence; the implied analogue requires future-01 econometric machinery. **Source:** Damodaran (2012) Ch.6 pp.183-245.
- the composite-score weighting choice is contested (equal-weight vs IC-weighted vs risk-adjusted) — the deeper estimation machinery is deferred to future-01 Quantitative Methods per DEC-1. **Source:** Damodaran (2012) Ch.6 pp.183-245.
