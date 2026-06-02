---
schema_version: "cacg.v0"
id: "be-overconfidence-bias"
title: "Overconfidence Bias"
reading_id: "10_behavioral_finance"
summary: "Overconfidence as unwarranted faith in one's reasoning, split into prediction overconfidence (too-narrow confidence intervals) and certainty overconfidence (miscalibrated certainty); Barber-Odean links it to excessive trading and underperformance."
tags: ["behavioral-finance", "overconfidence", "cognitive-bias", "miscalibration", "excessive-trading"]
citations:
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p070:0071"
    chunk_hash: "06a6d8653bad335a3e19befe13f6615f113b621aa1c4ec15ef34f60aec534352"
    page_range: [71, 71]
    quote: "intuitive reasoning, judgments, and cognitive abilities."
    edge_type: "defines"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p071:0072"
    chunk_hash: "a67953f7b06d64b92842702a1a5408c3073c7b618d56c8f3a2848fab3a610693"
    page_range: [72, 72]
    quote: "intervals that investors assign to their investment predictions are too"
    edge_type: "supports"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p074:0075"
    chunk_hash: "76cc6742ec987048d0ebb52ee1fe5d6fb8a9975752892934f053bb7a7856208b"
    page_range: [75, 75]
    quote: "study would have done better, on average, if they had maintained their start-of-the-year portfolios for the entire year."
    edge_type: "supports"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p072:0073"
    chunk_hash: "a6ef9250a1716e6c2100e8ec7d85e7e546c4b06d1ca8dbcff51e573f9974d2dc"
    page_range: [73, 73]
    quote: "Subjects reported being 100 percent sure when they were actually only 70 percent to 80 percent"
    edge_type: "supports"
card_hash: "f56baa8076d032119666810240da4ca2c8e3d744bfb3470baa7a70a37ed29228"
---
# Overconfidence Bias

## Intuition

Overconfidence is, in Pompian's words, unwarranted faith in one's own intuitive reasoning, judgments, and cognitive abilities — in short, people think they are smarter and have better information than they actually do. It is tagged a cognitive bias because it stems from a calibration failure that better information can in principle correct, yet Pompian calls it one of the most detrimental biases an investor can exhibit: it underwrites underestimated downside risk, excessive trading, the chase for the "next hot stock," and chronically underdiversified portfolios.
**Source:** Pompian (2006) Ch.4 pp.51.

Pompian splits the bias into two distinct flavors. *Prediction overconfidence* is too-narrow confidence intervals: forecasting a stock's value, investors leave far too little room in their range of payoffs, so they underestimate downside risk. *Certainty overconfidence* is being too sure of one's judgments: as people learn more about a situation, their accuracy does not rise much but their confidence does, because they fallaciously equate quantity of information with quality. The two are diagnosed and discussed separately but the practitioner advice is delivered across-the-board.
**Source:** Pompian (2006) Ch.4 pp.52.

## Definition

**Overconfidence** is unwarranted faith in one's intuitive reasoning, judgments, and cognitive abilities; people overestimate both their predictive ability and the precision of the information they are given.
**Source:** Pompian (2006) Ch.4 pp.51.

**Prediction overconfidence** is the variety in which the confidence intervals assigned to investment predictions are too narrow, leaving too little leeway in the range of expected payoffs and thereby underestimating downside risk.
**Source:** Pompian (2006) Ch.4 pp.52.

**Certainty overconfidence** is the variety in which people are too certain of their judgments; as they learn more, confidence increases though accuracy does not, leading to excessive trading and underdiversified portfolios.
**Source:** Pompian (2006) Ch.4 pp.52.

**Miscalibration** is the empirical gap between stated confidence and realized accuracy: in Fischhoff-Slovic-Lichtenstein general-knowledge tests, subjects reported being 100 percent sure when they were actually only 70 to 80 percent correct.
**Source:** Pompian (2006) Ch.4 pp.53.

## Mathematical Reasoning

Miscalibration is a probability-calibration inequality. Let `c` be a subject's stated confidence and `a(c)` the realized relative frequency of correct answers conditional on stating `c`. Perfect calibration is `a(c) = c` for all `c`. Overconfidence is the systematic shortfall `a(c) < c`: the source reports `a(1.00) ≈ 0.80`, `a(0.90) ≈ 0.75`, and `a(0.80) ≈ 0.65`. Prediction overconfidence is the same defect in the dispersion dimension: a stated 90% confidence interval `[L, H]` actually contains the truth far less than 90% of the time because `H - L` is set too small (the classic "90% interval for the DJIA value" survey in which essentially no respondent's interval covered the answer).
**Source:** Pompian (2006) Ch.4 pp.52-53, pp.58-59.

The Barber-Odean ("Boys Will Be Boys," 1991-1997, 35,000 households) result formalizes the trading consequence. Overconfident investors overestimate the precision of their private information and thus the expected gains of trading; a rational investor trades only when trading raises expected utility, but the overconfident investor trades when true expected net gains are negative. The empirical signature: sorted by turnover, net return falls as turnover rises — the most active quintile (monthly turnover over 9%) realized ~10% annual pretax returns versus the least active quintile's ~17.5%, and the stocks investors *sold* outperformed the stocks they *bought* (men's purchases lagged their sales by 20 bp/month, women's by 17 bp/month). Trading is, in their phrase, "hazardous to your wealth."
**Source:** Pompian (2006) Ch.4 pp.54-55, pp.60.

```
 Net return |  *                      schematic of Figure 4.1:
   (annual)  |     *                   net return falls and
            |        *                turnover rises across
            |           *   ___       quintiles 1 (low) -> 5 (high)
            |  ___ ___      |  |       Turnover bars climb left->right.
            | |  | |  | ___ |  |
            +-+--+-+--+-+-+-+--+----> turnover quintile (1..5)
```

## See Also

- [be-cognitive-vs-emotional-bias-taxonomy](./be-cognitive-vs-emotional-bias-taxonomy.md#intuition) — parent: overconfidence is a cognitive bias, a candidate for moderation.
- [be-information-processing-biases](./be-information-processing-biases.md#intuition) — self-attribution feeds dynamic overconfidence (Gervais-Odean "learning to be overconfident").
- [be-overconfidence-self-attribution-prices](./be-overconfidence-self-attribution-prices.md#intuition) — overconfidence as an asset-pricing driver.
- [be-overconfidence-disagreement-short-sale](./be-overconfidence-disagreement-short-sale.md#intuition) — overconfidence generating disagreement and volume under short-sale constraints.

## Escalate to Raw When

- You need the full Barber-Odean turnover-quintile figures, the 80% annual turnover statistic, or the 12.3% vs 6.3% fund-versus-investor return gap.
**Source:** Pompian (2006) Ch.4 pp.55, pp.59-60.
- You need the prediction/certainty diagnostic test questions and their results-analysis for client administration.
**Source:** Pompian (2006) Ch.4 pp.56-59.
- You need Box 4.1's four overconfidence "wealth hazards" verbatim for an advice memo.
**Source:** Pompian (2006) Ch.4 pp.54.
