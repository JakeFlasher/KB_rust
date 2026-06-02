---
schema_version: "cacg.v0"
id: "be-extrapolation-from-recent-data"
title: "Extrapolation From Recent Data (Survey Evidence)"
reading_id: "10_behavioral_finance"
summary: "Survey expectations of stock returns, earnings growth, and credit spreads are highly correlated across populations, predict actual behavior, and are predominantly extrapolative -- high after good past performance, then predictably disappointed -- the empirical case that expectations over-react to recent data."
tags: ["behavioral-finance", "extrapolation", "survey-expectations", "overreaction"]
citations:
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p124:0117"
    chunk_hash: "d59b5888d5a19ee425449d3e1b7a4dfe6496811bfedab7fe1a8dafe783981702"
    page_range: [125, 125]
    quote: "expectations of stock returns are predominantly extrapolative: Expectations of future returns are strongly positively correlated with past returns."
    edge_type: "supports"
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p130:0123"
    chunk_hash: "e0bf24cf6f235c1b2fceeae52f141bbebed2b6672329012f774443a95476ba86"
    page_range: [131, 131]
    quote: "high past returns predict expectation errors: When past returns are high, expected future returns are on average higher than realizations."
    edge_type: "supports"
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p134:0127"
    chunk_hash: "06b642b98f539f9c55412442f5349755de106e895e564943d4515fd7c49106a8"
    page_range: [135, 135]
    quote: "Analysts use information about past performance but overreact by"
    edge_type: "supports"
card_hash: "db20c6dfe44aeca8ff28eeafc3f28ad000914829870609f734dcba06cf092080"
---
# Extrapolation From Recent Data (Survey Evidence)

## Intuition

The empirical engine of the diagnostic program is survey expectations taken at face value. Rational-expectations economics long dismissed survey data as noise, but Gennaioli and Shleifer show measured expectations are highly consistent across surveys conducted with different methods and populations, and -- decisively -- they predict actual behavior: when investors expect high stock returns they pour money into equity mutual funds. People put their money where their mouth is, so expectations data are not noise and can be used to test belief rationality.
**Source:** Gennaioli & Shleifer (2018) Ch.4 pp.110, 115-116.

Read this way, the data reject rational expectations: forecast errors are systematically predictable, and the reason is extrapolation. Across six survey sources, expected future stock returns are strongly positively correlated with *past* returns -- Gallup expectations and the past 12-month S&P 500 return practically lie on top of each other -- yet high market valuations following high returns predict *low* realized returns. So high past returns predict positive expectation errors: investors expect high returns to continue, but realizations are on average low. The same pattern recurs in analyst earnings-growth forecasts, CFO expectations, and credit-spread forecasts.
**Source:** Gennaioli & Shleifer (2018) Ch.4 pp.111-112, 117-118.

## Definition

**Extrapolative expectations** are beliefs in which expected future outcomes are positively correlated with recent past outcomes -- high price growth is projected into the future, becoming highly inflated after several years of rapid growth.
**Source:** Gennaioli & Shleifer (2018) Ch.4 pp.111-112.

**Predictable forecast error** is a forecast error (realization minus expectation) that can be anticipated from information available when the forecast was made -- here, from past returns or past performance -- contradicting the Rational Expectations Hypothesis.
**Source:** Gennaioli & Shleifer (2018) Ch.4 pp.110, 118.

**HLTG / LLTG portfolios** are the 10 percent of stocks with the most optimistic / most pessimistic analyst long-term earnings-growth forecasts; HLTG stocks subsequently underperform, evidence of extrapolative over-optimism.
**Source:** Gennaioli & Shleifer (2018) Ch.4 pp.119-120.

## Mathematical Reasoning

The aggregate evidence is correlational, not a closed-form derivation: survey expected returns and trailing returns co-move tightly (Figure 4.2), while the correlation of the *same* expected return with the *next* 12-month S&P return is negative (though statistically insignificant), and rational-expectations model-implied returns are significantly *negatively* correlated with survey expectations. The interpretation: under extrapolation, prices are high because investors expect prices to rise further; under RE, prices are high because investors accept low forward returns -- the opposite sign on expectations.
**Source:** Gennaioli & Shleifer (2018) Ch.4 pp.116-118.

The cross-section sharpens this into a kernel-of-truth statement. A portfolio of HLTG (top-decile optimism) stocks returns about 3 percent in the year after formation over 1981-2015, while LLTG (bottom-decile) stocks return about 15 percent -- the source's reported magnitudes, not an exam computation. Analyst expected long-term growth peaks exactly at HLTG portfolio formation and declines afterward; analysts learn they were too optimistic and revise down, and returns follow revisions. Critically, extrapolation is *not mechanical*: HLTG firms genuinely have a fatter right tail (more future "Googles"), so analysts react in the right direction but predict too many high performers -- overreaction to news, not blind trend-following.
**Source:** Gennaioli & Shleifer (2018) Ch.4 pp.119-122.

```
   Analyst LT-growth expectations around HLTG portfolio formation:

   exp.growth
     high  |        ___
           |      _/   \_      <- peak optimism at formation (year 0)
           |    _/       \__
           |___/            \____  <- predictable disappointment, returns low after
           +---------------------- years rel. to formation
              -3   -1   0   +1  +3
```

**Source:** Gennaioli & Shleifer (2018) Ch.4 pp.120-121 (Figures 4.3-4.4).

## See Also

- [be-diagnostic-expectations](./be-diagnostic-expectations.md#intuition) -- the operator that rationalizes extrapolation as overreaction to news.
- [be-rational-vs-diagnostic-expectations](./be-rational-vs-diagnostic-expectations.md#mathematical-reasoning) -- the negative error-revision covariance this evidence motivates.
- [be-kernel-of-truth](./be-kernel-of-truth.md#mathematical-reasoning) -- the "too many future Googles" exaggeration of a real fat tail.
- [be-belief-driven-credit-cycle](./be-belief-driven-credit-cycle.md#intuition) -- the credit-spread extrapolation facts modeled dynamically.

## Escalate to Raw When

- You need the six survey sources, the pairwise correlation matrix (Figure 4.1), or the fund-flow correlation row (pp.114-116).
- You need the Case-Shiller-Thompson home-price expectation figures or the CFO earnings-growth-vs-investment evidence (pp.50-51, 113).
- You need the BGLS (2017) future-Google distribution figure and the non-mechanical learning argument (pp.122).
