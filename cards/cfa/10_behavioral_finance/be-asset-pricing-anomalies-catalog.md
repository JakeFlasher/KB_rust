---
schema_version: "cacg.v0"
id: "be-asset-pricing-anomalies-catalog"
title: "Asset-Pricing Anomalies Catalog"
reading_id: "10_behavioral_finance"
summary: "The set of empirical facts behavioral asset-pricing models target: a high equity premium, excess volatility (= time-series predictability via the P/D ratio), the cross-section of average returns (momentum, reversal, value, beta and volatility anomalies), bubbles, and high trading volume."
tags: ["behavioral-finance", "asset-pricing", "anomalies", "trading-volume", "bubbles"]
citations:
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p090:0133"
    chunk_hash: "5df91b7e281e20159f3d97178624dd5662ac21c55c44ff5533a5282e6f857a03"
    page_range: [91, 91]
    quote: "There are three central facts about the returns on the overall U.S. stock market: these"
    edge_type: "defines"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p091:0135"
    chunk_hash: "5d85d3a454050cc2c1756c304cf2d5545b54627dbbe8ac3a3e8dc4e98d388927"
    page_range: [92, 92]
    quote: "Time-series predictability and excess volatility are now seen as the same"
    edge_type: "supports"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p094:0138"
    chunk_hash: "04ce6861fe556d9f8ea77b20caf267bc1e6076e6be47aee95ed68741929c209e"
    page_range: [94, 94]
    quote: "Notice the contrast with long-term reversal."
    edge_type: "supports"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p121:0183"
    chunk_hash: "fd023d214263db1e9ad5375a023538fc7ce195cdf897afc371bc5c32366fffac"
    page_range: [121, 121]
    quote: "the concept of overconfidence can help explain a number of puzzling facts in finance, most notably the very high trading volume in financial markets"
    edge_type: "supports"
card_hash: "f508bf7fe1f3630a7c1c8dab2632d65b737e6cf6c0cdd7a9a83efd5a3ef5fb39"
---
# Asset-Pricing Anomalies Catalog

## Intuition

Behavioral asset pricing is a target-driven field: it exists to explain a specific, widely-agreed set of empirical facts that the traditional model struggles to account for. Most were first documented in the U.S. stock market because of high-quality data, but a recurring theme is that the same patterns -- momentum, long-term reversal, the beta anomaly, excess volatility -- also appear in real estate, government bonds, currencies, and commodities. This pervasiveness across asset classes is itself an argument for psychology-based explanations, since the leading behavioral assumptions apply naturally everywhere, not just to stocks.
**Source:** Barberis (2018) §2.1-2.2 pp.91, 92.

For the *aggregate* market there are three central facts: returns are predictable in the time series, returns display excess volatility, and the average level of returns (the equity premium) is high. A key unification is that excess volatility and time-series predictability are the *same* phenomenon: since the P/D ratio is stationary, a high P/D ratio cannot forecast higher dividend growth (that is ruled out by excess volatility), so it must forecast lower future returns -- which is exactly time-series predictability.
**Source:** Barberis (2018) §2.1 pp.91, 92.

For the *cross-section*, the CAPM is roundly rejected: high-beta stocks do not earn higher average returns, yet many other characteristics predict returns. The headline patterns are momentum (past 6-12 month returns predict positively), long-term reversal (past 3-5 year returns predict negatively), short-term reversal, post-earnings-announcement drift, the value premium (low price-to-fundamentals predicts positively), the size, issuance, beta, and idiosyncratic-volatility anomalies. Two further facts complete the catalog: bubbles (episodes of substantial, temporary overvaluation with characteristic features) and high trading volume (U.S. turnover has exceeded 100% per year since 1998), the latter being something the traditional model essentially cannot produce.
**Source:** Barberis (2018) §2.2-2.3, §5 pp.94, 88, 121.

## Definition

**Equity premium** is the excess of the average U.S. stock-market return over Treasury Bills, which has exceeded 5% per year over the past century -- "a puzzle known as the 'equity premium puzzle.'"
**Source:** Barberis (2018) §2.1 pp.92.

**Excess volatility** means aggregate stock prices fluctuate more than can be justified by rationally-varying forecasts of the future cash flows paid to investors.
**Source:** Barberis (2018) §2.1 pp.91.

**Time-series predictability** is the finding that ratios of price to fundamentals (P/E, P/D) predict the market's subsequent excess return with a negative sign; it is the same phenomenon as excess volatility.
**Source:** Barberis (2018) §2.1 pp.91, 92.

**Momentum / long-term reversal** are, respectively, the positive predictive power of past 6-12 month returns and the negative predictive power of past 3-5 year returns for the cross-section of subsequent returns.
**Source:** Barberis (2018) §2.2 pp.94.

**Bubble** is, by the empirical definition, an episode in which an asset's price rises sharply and then reverses, accompanied during the rise by reports of overvaluation, abnormally high trading volume, highly extrapolative investor expectations, and some sophisticated investors increasing exposure.
**Source:** Barberis (2018) §2.3 pp.96, 97.

**High trading volume** is the empirical fact of very high turnover (annual U.S. turnover above 100% since 1998), hard to reconcile with non-speculative trading motives alone.
**Source:** Barberis (2018) §5 pp.121.

## Mathematical Reasoning

The aggregate facts are linked through the stationarity of the price-to-dividend ratio. If the P/D ratio is stationary, a high P/D today must be "undone" either by future dividend growth `D` rising or by future price `P` falling. Excess volatility evidence shows the P/D ratio does *not* forecast dividend growth with a positive sign (Campbell-Shiller), so the only remaining channel is that high P/D forecasts low future returns -- time-series predictability. Hence the two facts are one.
**Source:** Barberis (2018) §2.1 pp.92.

The cross-sectional facts are summarized as signed predictive relationships: a characteristic `F` has "negative predictive power" if, controlling for beta, high-`F` stocks earn lower average returns than low-`F` stocks. The catalog (the source's Table 1) gives signs:

```
  Characteristic              Sign of predictive power   Anomaly name
  -------------------------   ------------------------   --------------------
  past 3-5 year return        -                          long-term reversal
  past 6-12 month return      +                          momentum
  past week/month return      -                          short-term reversal
  earnings surprise           +                          PEAD
  market capitalization       -                          size
  price-to-fundamentals       -                          value premium
  equity issuance             -                          issuance
  systematic (beta)           -  (no positive premium)    beta anomaly
  idiosyncratic volatility    -                          idio-vol anomaly
  profitability               +                          profitability
```

A caution the source flags: with over a hundred published predictors, data mining is a real concern, but the listed characteristics are credited with genuine power because they forecast out of sample (other countries, pre-1960 U.S. data).
**Source:** Barberis (2018) §2.2 pp.93, 94, 95.

Trading volume is the fact that pins down which framework is needed: non-speculative motives (liquidity, rebalancing, taxes) cannot account for turnover above 100% per year, so most volume must be driven by *disagreement* about future prices. The source notes disagreement requires that investors have different priors, observe different information, or are not fully rational -- and rational difference-of-information alone tends to predict *low* volume, pointing to overconfidence.
**Source:** Barberis (2018) §5 pp.121.

## See Also

- [be-three-frameworks-behavioral-asset-pricing](./be-three-frameworks-behavioral-asset-pricing.md#intuition) -- the three model classes built to explain this catalog.
- [be-sentiment-vs-fundamentals](./be-sentiment-vs-fundamentals.md#intuition) -- the sentiment lens on these mispricing facts.
- [be-extrapolative-beliefs-asset-prices](./be-extrapolative-beliefs-asset-prices.md#intuition) -- accounts for excess volatility, predictability, and bubbles.
- [be-overconfidence-disagreement-short-sale](./be-overconfidence-disagreement-short-sale.md#intuition) -- accounts for overvaluation and high volume.
- [be-prospect-theory-asset-pricing](./be-prospect-theory-asset-pricing.md#intuition) -- accounts for the equity premium and average returns.
- [be-value-anomaly](./be-value-anomaly.md#intuition) and [be-momentum-anomaly](./be-momentum-anomaly.md#intuition) -- two catalog entries in depth.

## Escalate to Raw When

- You need the full empirical definition of a bubble with all six characteristics (i)-(vi) and the technology-bubble illustration (pp.96-97).
- You need the complete Table 1 of cross-sectional predictors with citations and the data-mining discussion (pp.93-95).
- You need the rational benchmark models (rare disasters, long-run risk, habit, rational learning) the behavioral facts are contrasted against (pp.91, footnote 3).
