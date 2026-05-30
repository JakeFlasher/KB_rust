---
schema_version: "cacg.v0"
id: "mt-pin-probability-informed-trading"
title: "PIN: The Probability of Informed Trading from Buy/Sell Order Counts"
reading_id: "14_microstructure_and_trading"
summary: "PIN is a maximum-likelihood estimate of the fraction of order flow that is informed, inferred purely from the daily counts of buys and sells modeled as a Poisson mixture, with no price or quote data required."
tags: ["microstructure", "informed-trading", "pin", "order-flow", "adverse-selection"]
citations:
  - source_id: "mt_hasbrouck_2007_empirical_market_microstructure"
    chunk_id: "mt_hasbrouck_2007_empirical_market_microstructure:p069:0086"
    chunk_hash: "89ae0ea2ac99b4f49a3157f54f4c3dfdefeff3342338bd0eee54641ec4bbd011"
    page_range: [69, 69]
    quote: "PIN is the unconditional probability that a randomly chosen trader on a randomly chosen day is informed"
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p164:0257"
    chunk_hash: "41e6f3a03d164e5ce8082b68cc2c3aa8c47c651f9c767dac527ff3c847e10fc3"
    page_range: [165, 165]
    quote: "PIN is the ratio between the rate of arrival of informed traders and the total rate of order arrival"
    edge_type: "supports"
---
# PIN: The Probability of Informed Trading from Buy/Sell Order Counts

## Intuition
Adverse-selection theory says a dealer loses to informed counterparties and recoups those losses via the spread, but the dealer never observes who is informed. PIN (Probability of INformed trading) is the empirical answer to the question "what fraction of the trades I face are informed?" Its striking feature is that it is recovered from order-flow imbalance alone — the daily tally of buyer-initiated versus seller-initiated trades — without ever using prices, quotes, or returns.

The intuition rests on a signature in the data. On an ordinary (no-news) day, informed traders are absent, so buys and sells arrive from balanced uninformed demand and the day looks roughly symmetric. On a day with an information event, informed traders pile in on one side — all buying after good news, all selling after bad news — so the day's order flow becomes lopsided. Over many days the joint distribution of (buys, sells) therefore develops two "lobes" of one-sidedness flanking a central balanced cloud. The more pronounced the lobes, the more informed trading the market is absorbing.

```
   sells
     ^
     |   . (bad-news lobe: many sells, few buys)
     |  .
     | . . . (no-news cloud: balanced, centered)
     |    . .
     |       . (good-news lobe: many buys, few sells)
     +------------------> buys
```

Maximum likelihood fits a mixture model to that cloud-plus-lobes shape; PIN is the single summary number distilled from the fitted parameters.
**Source:** Hasbrouck (2007) §6.2 pp.57-59

## Definition
Order arrivals follow a Poisson process. Uninformed buyers and sellers each arrive with intensity ε in every state. With probability α an information event occurs; conditional on an event, with probability δ the news is bad (informed sellers arrive, adding intensity µ to sells) and with probability (1−δ) the news is good (informed buyers add µ to buys). The day's buy count b and sell count s are jointly distributed as a Poisson mixture (eq. 6.3):

  Pr(b,s) = (1−α)·Pr(b;ε)·Pr(s;ε) + α[δ·Pr(b;ε)·Pr(s;µ+ε) + (1−δ)·Pr(b;µ+ε)·Pr(s;ε)]

where Pr(n;λ) is the Poisson probability of n arrivals at intensity λ. PIN is defined (eq. 6.4) as the expected share of arriving traders who are informed:

  PIN = αµ / (αµ + 2ε)

In words, "PIN is the unconditional probability that a randomly chosen trader on a randomly chosen day is informed." The supporting treatment states the same idea as the ratio of the informed arrival rate to the total order-arrival rate.
**Source:** Hasbrouck (2007) §6.2 pp.58-59; Foucault, Pagano & Röell (2013) §5.4 p.165

## Mathematical Reasoning
The numerator αµ is the expected informed arrival intensity (informed traders appear only with event probability α, at rate µ). The denominator αµ + 2ε adds the two uninformed streams (buyers and sellers each at rate ε), giving the total expected arrival intensity. PIN is thus a pure ratio of intensities, bounded in [0,1).

A crucial identification fact: the parameters α and µ enter PIN only through their product αµ. The likelihood is therefore relatively informative about the composite αµ (and hence PIN) while being weak about α and µ separately — frequent-but-thin information (high α, low µ) and rare-but-heavy information (low α, high µ) generate nearly indistinguishable buy/sell distributions. Estimation errors in α and µ are strongly negatively correlated, but for the object of interest, PIN, that imprecision largely cancels, so PIN can be estimated reliably and can stay stable even as α and µ drift offsettingly.

Comparative statics: ∂PIN/∂(αµ) > 0 and ∂PIN/∂ε < 0 — more informed flow or less uninformed liquidity both raise PIN, sharpening the two lobes. The likelihood uses only the totals b and s each day (order identities and sequencing are irrelevant), confirming no price data enter. The supporting source closes the loop to pricing: under symmetry it derives the opening spread a₁ − b₁ = 2·PIN·(v_H − v_L), so PIN maps monotonically into the adverse-selection component of the bid-ask spread.
**Source:** Hasbrouck (2007) §6.2 pp.58-59; Foucault, Pagano & Röell (2013) §5.4 p.165

## Boundary Notes
The model is highly stylized: at most one information event per day, the event arriving at the day's start, and only two value realizations. The author warns it is "probably a mistake to take the model's estimates too literally"; PIN is best read as a meaningful measure of order-flow one-sidedness rather than a literal information count. One-sidedness can also arise from non-information mechanics — slowly diffusing public news that leaves stale quotes to be picked off, or order fragmentation in an illiquid stock — so PIN may capture cross-sectional liquidity variation that is not pure private information. PIN inherits the sequential-trade adverse-selection setup (Glosten-Milgrom lineage) but, unlike the trade-and-price approach, deliberately discards price dynamics, trading information content for a price-free estimator.
**Source:** Hasbrouck (2007) §6.3 pp.59-60

## See Also
- [`mt-glosten-milgrom-adverse-selection`](./mt-glosten-milgrom-adverse-selection.md) -- the sequential-trade adverse-selection model PIN is built upon
- [`mt-empirical-determinants-illiquidity`](./mt-empirical-determinants-illiquidity.md) -- PIN as an empirical driver of the adverse-selection component of illiquidity
- [`mt-trade-direction-signing`](./mt-trade-direction-signing.md) -- classifying trades as buys vs sells supplies the b,s counts PIN consumes

## Escalate to Raw When
Hasbrouck derives the binomial-mixture special case (§6.1) and the full Poisson-mixture likelihood (eq. 6.3) that this card only states; re-read pp.56-59 for the exact mixture algebra and the figures showing the unimodal-to-bimodal transition as µ rises. For the explicit maximum-likelihood factorization over good/bad/no-news days and the spread-mapping derivation, re-read Foucault, Pagano & Röell §5.4 pp.165-167 (eqs. 5.27-5.29).
