---
schema_version: "cacg.v0"
id: "be-three-frameworks-behavioral-asset-pricing"
title: "Three Frameworks of Behavioral Asset Pricing"
reading_id: "10_behavioral_finance"
summary: "Barberis argues behavioral finance is not scattered: its center of gravity is three psychology-based frameworks -- extrapolative beliefs, overconfidence/disagreement, and gain-loss (prospect-theory) preferences -- each with a distinct natural application across prices and volume."
tags: ["behavioral-finance", "asset-pricing", "extrapolation", "overconfidence", "prospect-theory"]
citations:
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p090:0132"
    chunk_hash: "6cd43b7111f73fed715ae6ee9186bf2aec28c80f5c6c893f8d14cd90d6539791"
    page_range: [90, 90]
    quote: "finance lies in just three frameworks: the extrapolation framework (Section 4), the overconfidence framework (Section 5), and a gain-loss utility framework inspired by prospect theory (Section 7)."
    edge_type: "defines"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p090:0132"
    chunk_hash: "6cd43b7111f73fed715ae6ee9186bf2aec28c80f5c6c893f8d14cd90d6539791"
    page_range: [90, 90]
    quote: "extrapolation is most helpful for explaining fluctuations in financial markets, overconfidence for"
    edge_type: "supports"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p088:0129"
    chunk_hash: "c3475a37518d5c2e668f5afc3b889993d25e868fdb7a89821c4410de147230c5"
    page_range: [88, 88]
    quote: "Research in behavioral finance has tried to improve the psychological realism of the traditional model along three dimensions."
    edge_type: "supports"
card_hash: "83c7ee19be200fc6fe9251b40e69ccbb39f4d529615fbbc03d7e9f83757bff58"
---
# Three Frameworks of Behavioral Asset Pricing

## Intuition

For four decades after Markowitz and Modigliani-Miller, asset-pricing research ran on a single "traditional" framework resting on two assumptions about individual psychology: investors hold rational (Bayesian) beliefs, and they choose by maximizing Expected Utility over consumption. Behavioral finance, emerging in the 1980s, sets out to make sense of the same data using assumptions that are psychologically more realistic. It improves the traditional model along three dimensions -- beliefs (people deviate from Bayes' rule), preferences (people may not be Expected-Utility maximizers), and cognitive limits (people cannot instantly process all relevant information).
**Source:** Barberis (2018) §1 pp.88.

A natural worry is that the field is a grab-bag of disconnected ideas. Barberis argues the opposite: the center of gravity lies in just three frameworks. (1) *Extrapolative beliefs* -- investors expect future returns or fundamentals to continue recent trends. (2) *Overconfidence / differences of opinion* -- investors overweight their own information signals and so disagree. (3) *Gain-loss (prospect-theory) preferences* -- investors derive utility from gains and losses relative to a reference point, not only from consumption. These are complements, not competitors, because each has a different natural domain of application.
**Source:** Barberis (2018) §2 (intro), §3 pp.90.

The division of labor is the key organizing claim. Extrapolation is most helpful for explaining market *fluctuations* (excess volatility, bubbles, predictability). Overconfidence-driven disagreement is most helpful for understanding *trading volume*, which the traditional model cannot generate. Gain-loss utility is most helpful for thinking about *average returns* (the equity premium, the cross-section). No unified model yet combines realistic beliefs and realistic preferences in one parsimonious specification, but the three frameworks together cover the lion's share of the anomalies.
**Source:** Barberis (2018) §2 pp.90.

## Definition

**Traditional (rational) framework** rests on two psychological assumptions: investors update beliefs correctly via Bayes' rule, and they maximize Expected Utility for a utility function increasing and concave over consumption.
**Source:** Barberis (2018) §1 pp.88.

**Extrapolation framework** posits that an investor's estimate of a quantity's future value is a positive function of that quantity's recent past values; applied to returns or fundamentals it is the workhorse for market fluctuations.
**Source:** Barberis (2018) §2 pp.90.

**Overconfidence framework** posits that investors overestimate the precision of their own information signals (and underestimate others'), generating disagreement; it is the workhorse for trading volume.
**Source:** Barberis (2018) §2, §5 pp.90.

**Gain-loss utility framework** posits that investors derive utility from gains and losses in financial wealth relative to a reference point, with loss aversion; it is the workhorse for average returns.
**Source:** Barberis (2018) §2 pp.90.

## Mathematical Reasoning

Barberis treats the frameworks as positive (predictive) models, each disciplined enough to be written down and confronted with data. The shared methodological stance is that the modern debate is not "efficient" versus "inefficient" markets in the abstract, but between specific, precisely-defined models -- "long-run risk vs. extrapolation, say, or habit formation vs. gain-loss utility." Each behavioral model must specify the exact irrationality or friction and show it explains a range of facts and makes testable predictions.
**Source:** Barberis (2018) §3 pp.101.

The three frameworks attach to the three improvement dimensions. Two dimensions concern *beliefs* (extrapolation; overconfidence), so they share the assumption that the perceived distribution of future outcomes departs from the rational one while preferences stay Expected-Utility. The third concerns *preferences* (gain-loss utility) so it retains rational beliefs but replaces the consumption-only Expected-Utility objective. A fourth dimension, cognitive limits / bounded rationality, is treated as a cross-cutting third approach rather than as part of the core trio.
**Source:** Barberis (2018) §1, §2 pp.88, 90.

The applications map cleanly:

```
  Framework            Realism dimension   Natural domain
  ------------------   -----------------   ----------------------------------
  extrapolation        beliefs             fluctuations: excess volatility,
                                           bubbles, time-series predictability
  overconfidence       beliefs             trading volume; over-/under-reaction
  gain-loss utility    preferences         average returns: equity premium,
                                           the cross-section of returns
```

A unifying claim, but not a unifying *model*: the source states there is "as yet no 'unified' model in behavioral finance" combining realistic beliefs and preferences, though the research points toward the form one might take.
**Source:** Barberis (2018) §2 pp.90.

## See Also

- [be-asset-pricing-anomalies-catalog](./be-asset-pricing-anomalies-catalog.md#intuition) -- the empirical facts these three frameworks target.
- [be-extrapolative-beliefs-asset-prices](./be-extrapolative-beliefs-asset-prices.md#intuition) -- framework (1) worked out in an equilibrium model.
- [be-overconfidence-disagreement-short-sale](./be-overconfidence-disagreement-short-sale.md#intuition) -- framework (2) coupled with a short-sale constraint.
- [be-prospect-theory-asset-pricing](./be-prospect-theory-asset-pricing.md#intuition) -- framework (3) in equilibrium.
- [be-limits-of-arbitrage](./be-limits-of-arbitrage.md#intuition) -- why irrational investors can move prices, the precondition for all three.
- [be-two-model-mispricing](./be-two-model-mispricing.md#intuition) -- the methodological discipline of writing specific behavioral models.
- [be-sentiment-vs-fundamentals](./be-sentiment-vs-fundamentals.md#intuition) -- the sentiment channel these frameworks formalize.

## Escalate to Raw When

- You need the full taxonomy of "other belief-based approaches" (sticky beliefs, rare-events beliefs, feelings, herding) that sit alongside the three core frameworks (Section 6, pp.119-130).
- You need Barberis' concluding sketch of what a future unified model might look like (Section 10, pp.159-162).
- You need the cognitive-limits / bounded-rationality framework (inattention, categorization) treated as the cross-cutting fourth approach (Section 9, pp.154-159).
