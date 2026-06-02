---
schema_version: "cacg.v0"
id: "eq-comparable-company-analysis"
title: "Comparable-Company Analysis"
reading_id: "05_equity"
summary: "Comparable-company analysis values a target by reading off peer firms' multiples. The two central judgements are peer-set selection and multiple-family choice; cross-multiple disagreement is a diagnostic signal. Damodaran develops the relative-valuation discipline; CFA L1 R38 supplies the curriculum framing."
tags: ["equity", "comparable-company"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p616:0805"
    chunk_hash: "ce37654c5b718de41e7582e19836e7d5ab2089a8426973a4c3f9c217ae8a8cc4"
    page_range: [616, 616]
    quote: "In relative valuation, the objective is to value assets based on how similar assets are currently priced in the market."
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p620:0812"
    chunk_hash: "52dac5de6f7811e3784c4240ff98acb771839a62e2cf96087416fb0754f90d9e"
    page_range: [620, 621]
    quote: "If the numerator for a multiple is an equity value, then the denominator should be an equity value as well."
    edge_type: "supports"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p634:0832"
    chunk_hash: "8456d0d42f3fd31ecebe018fce372aaa5b3fa53f541ffed8eec409fba6bf71cf"
    page_range: [635, 635]
    quote: "Earnings multiples remain the most used measure of relative value."
    edge_type: "supports"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p690:0897"
    chunk_hash: "ee01be57ae3e0fdd5d49d093d5e57004313f7e3910a1a2dc4781183b44a86f23"
    page_range: [690, 691]
    quote: "The relationship between price and book value has always attracted the attention of investors."
    edge_type: "supports"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p726:0940"
    chunk_hash: "4ab3d346163d91b99ea320b3801f20556731be16abfca6bee34eac73d358a288"
    page_range: [726, 726]
    quote: "For young firms that have negative earnings, multiples of revenues have replaced multiples of earnings."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2338:3425"
    chunk_hash: "ade9885e07402e070f0c7984223bec1beae08d5b8707630d4e08dd4fa5d25f34"
    page_range: [2338, 2338]
    quote: "The choice of model and the derivation of inputs require skill and judgment."
    edge_type: "supports"
card_hash: "154db099c7d9f4aae094f02610cad7624a4a8fe355dc4fdd3c91aafbc6d1e141"
---
# Comparable-Company Analysis

## Intuition

Comparable-company analysis values a target firm by reading off the
prices that the market currently assigns to similar firms. A target
that trades at a lower multiple than its peers is, in relative terms,
cheap; a target that trades at a higher multiple is, in relative
terms, rich. The valuation gives an answer relative to the sector's
current pricing — it does not tell the analyst whether the entire
sector is mispriced. **Source:** Damodaran (2012) Ch.17 pp.616-634.

The two practical questions in any comparable-company analysis are:
which firms count as peers, and which multiples to use. Peer-set
selection determines the cross-sectional benchmark; multiple-family
selection determines the lens through which the comparison is made.
Different multiples capture different aspects of the firm — earnings-
based multiples capture profitability, book-value multiples capture
asset intensity, sales-based multiples capture revenue scale.
Disagreement across multiples is itself a diagnostic signal. **Source:**
Damodaran (2012) Ch.17 pp.616-634.

```
<!-- primitive: multiples-map source: _diagram_primitives.md -->
multiples comparison       metric ->
peer set                   P/E   P/B   EV/EBITDA   EV/Sales
   |
   |  peer 1                 +     +        .          .
   |
   |  peer 2                 .     .        .          -
   |
   |  peer 3                 -     -        -          -
   |
   |  ...                    .     .        .          .
   |
   |  sector median          .     .        .          .
   |
   |  sector mean            .     +        .          .
   |
   |  TARGET company         ?     ?        ?          ?
   v
                  legend: + rich   . in line   - cheap
                  ? = the multiple value being assessed
                  cells are conceptual ranks, not numeric levels
```

## Definition

Comparable-company analysis is the family of relative-valuation
methods that reads off the prices implied by peer firms' multiples
and applies them to the target. The peer set is a collection of
firms judged similar to the target along the dimensions that drive
the chosen multiples. **Source:** Damodaran (2012) Ch.17 pp.616-634.

Peer-set construction is the central judgement. Damodaran identifies
several criteria: industry / sector classification, business-line
mix, scale, growth profile, profitability, capital structure, and
risk profile. A peer set selected only by sector code (SIC, GICS) is
weaker than one selected by underlying business comparability;
mismatched leverage in particular biases equity multiples (P/E, P/B)
even when the underlying operations are similar. **Source:** Damodaran
(2012) Ch.17 pp.616-634.

The multiple families used in comparable-company analysis fall into
the buckets developed in
[`eq-pe-and-relative-valuation`](./eq-pe-and-relative-valuation.md)
and
[`eq-pb-and-multiples-taxonomy`](./eq-pb-and-multiples-taxonomy.md):
earnings (P/E, EV/EBIT, EV/EBITDA), book-value (P/B, EV/Capital),
revenue (P/Sales, EV/Sales), and sector-specific (EV/Subscriber,
P/Reserves, EV/Gross-Merchandise-Volume). The earnings family is
covered in Damodaran Ch.18; the book-value family in Ch.19; the
revenue and sector-specific families in Ch.20. **Source:** Damodaran
(2012) Ch.17 pp.616-634.

The sector-multiple summary (median or mean) is the central tendency;
the sector-multiple spread (interquartile range, standard deviation)
is the dispersion. A target valued against a wide-spread sector
inherits more uncertainty than one valued against a tight-spread
sector. The dispersion question — what causes peer multiples to
differ — is the bridge to fundamental relative valuation, where the
analyst regresses multiples on fundamentals (growth, payout, risk).
The depth of that regression-based dispersion analysis is in
[`eq-multiples-dispersion`](./eq-multiples-dispersion.md) and
[`eq-cross-sectional-multiples-distribution`](./eq-cross-sectional-multiples-distribution.md).
**Source:** Damodaran (2012) Ch.18 pp.635-689.

## Mathematical Reasoning

The simplest comparable-company valuation is `Value_target =
multiple_peer · X_target`, where `multiple_peer` is the peer-set
central tendency (median or mean) of the chosen multiple and
`X_target` is the target's matching scale variable (earnings, book
value, sales). The output is a point estimate of the target's value
under the assumption that the target should trade at the peer-set
average. **Source:** Damodaran (2012) Ch.17 pp.616-634.

The choice of central tendency affects the result. The median is
robust to outliers but ignores the mass of the distribution; the
mean uses the entire distribution but is sensitive to extreme peers
(very high-growth or very levered firms). Damodaran's general
recommendation is the median for narrow peer sets and a trimmed mean
for broad peer sets. **Source:** Damodaran (2012) Ch.17 pp.616-634.

Earnings-based multiples (P/E, EV/EBIT, EV/EBITDA) work best when
peers have similar profitability and capital intensity. Book-value
multiples (P/B, EV/Capital) work best when assets are similar in age
and accounting treatment, which is rare across firms. Revenue
multiples (P/Sales, EV/Sales) work when earnings are negative,
volatile, or distorted but require the analyst to assume comparable
margins across peers — the form `EV/Sales = (operating margin) ·
(EV/EBIT)` shows that revenue multiples bake in a margin assumption.
**Source:** Damodaran (2012) Ch.20 pp.726-770.

Cross-multiple comparison turns disagreement into a diagnostic. If
the target appears cheap on EV/EBITDA but rich on P/E, the gap is
explained by the firm's capital structure or tax position (the two
multiples differ in how they treat interest and depreciation). If
cheap on P/Sales but rich on P/E, the firm's margin profile differs
from peers. Damodaran develops the algebraic bridges between
multiples in Ch.18 and Ch.19 to support this kind of cross-
diagnostic. **Source:** Damodaran (2012) Ch.18 pp.635-689.

The fundamental version of comparable analysis writes the multiple as
a function of fundamentals: for example, the Gordon-growth-derived
justified P/E `P/E = payout · (1 + g) / (r - g)` (see
[`eq-pe-and-relative-valuation`](./eq-pe-and-relative-valuation.md))
expresses what the multiple should be given the firm's payout,
growth, and cost of equity. Comparing the actual multiple to the
justified multiple separates relative-valuation findings from
fundamental-valuation findings — a peer set that all trades above
fundamentals shows that the entire sector is rich on fundamentals,
not that the target is cheap relative to peers. **Source:** Damodaran
(2012) Ch.18 pp.635-689.

The CFA L1 frame presents comparable-company analysis as the
relative-valuation methodology used alongside DCF, identifies peer-
set selection and multiple-family choice as the two central
judgements, and emphasizes that relative valuation answers a
relative question — not an absolute mispricing question. **Source:**
CFA L1 Curriculum (2022) Vol.4/pp.361-416.

## See Also

- [`eq-pe-and-relative-valuation`](./eq-pe-and-relative-valuation.md) — earnings-based multiples and the justified-P/E derivation
- [`eq-pb-and-multiples-taxonomy`](./eq-pb-and-multiples-taxonomy.md) — book-value, revenue, and sector-specific multiple families
- [`eq-intrinsic-value`](./eq-intrinsic-value.md) — the intrinsic-value benchmark relative valuation complements

## Escalate to Raw When

Open Damodaran Ch.17 / Ch.18 / Ch.19 / Ch.20 directly when any of the
criteria below applies. **Source:** Damodaran (2012) Ch.17 pp.616-634.

- the peer set is small or non-obvious (niche firm, conglomerate, cross-border listing) and the analyst needs the full peer-construction criteria — Damodaran Ch.17 develops the selection framework. **Source:** Damodaran (2012) Ch.17 pp.616-634.
- the cross-multiple disagreement is large and the analyst needs the algebraic bridges between multiples to diagnose the source — Damodaran Ch.18 (earnings) and Ch.19 (book value) develop the per-family decompositions. **Source:** Damodaran (2012) Ch.18 pp.635-689.
- the target operates in a sector requiring specialized multiples (banking, insurance, energy reserves, telecom subscribers, software ARR) — Damodaran Ch.20 covers the sector-specific multiple families and their adjustments. **Source:** Damodaran (2012) Ch.20 pp.726-770.
