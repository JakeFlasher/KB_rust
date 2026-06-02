---
schema_version: "cacg.v0"
id: "be-prospect-theory-anomaly-verdict"
title: "Prospect-Theory Anomaly Scorecard"
reading_id: "10_behavioral_finance"
summary: "The BJW model helps explain 14 of 23 anomalies (momentum, volatility, distress, profitability, skewness, issuance, PEAD, ...), makes no strong prediction on 2, and predicts the wrong sign on 7 (notably size and value), with the failures concentrated in belief-driven anomalies."
tags: ["behavioral-finance", "anomalies", "asset-pricing", "model-evaluation"]
citations:
  - source_id: "bf_barberis_jin_wang_2021_pt_anomalies"
    chunk_id: "bf_barberis_jin_wang_2021_pt_anomalies:p037:0049"
    chunk_hash: "341b1adbfa15db9067807995b30bce8f7e39ac61355399d5beb8a65e39806ac2"
    page_range: [38, 38]
    quote: "the model is helpful for thinking about 14 of the 23 anomalies"
    edge_type: "defines"
  - source_id: "bf_barberis_jin_wang_2021_pt_anomalies"
    chunk_id: "bf_barberis_jin_wang_2021_pt_anomalies:p041:0053"
    chunk_hash: "c4c233496a5a820df5f6ef5c06fc1d25284edbc511a54b7651da2117f0a9638a"
    page_range: [41, 41]
    quote: "Table III shows that the model incorrectly predicts a negative alpha for decile 1 of the size anomaly, which contains the stocks with the lowest market capitalization."
    edge_type: "supports"
  - source_id: "bf_barberis_jin_wang_2021_pt_anomalies"
    chunk_id: "bf_barberis_jin_wang_2021_pt_anomalies:p041:0053"
    chunk_hash: "c4c233496a5a820df5f6ef5c06fc1d25284edbc511a54b7651da2117f0a9638a"
    page_range: [41, 41]
    quote: "a large fraction of their return comes around earnings announcement dates."
    edge_type: "supports"
  - source_id: "bf_barberis_jin_wang_2021_pt_anomalies"
    chunk_id: "bf_barberis_jin_wang_2021_pt_anomalies:p043:0056"
    chunk_hash: "376a457f9773591e17fa31bef839f59c73597796e46dd310566f9e79da71702a"
    page_range: [43, 43]
    quote: "our model performs better than the CAPM and three-factor model, similarly to the four-factor model, and less well than the five- and six-factor models"
    edge_type: "supports"
card_hash: "fdf8670842fa3c5c4582887454aff299c2d81a6a1dd381441df6531231d2b49f"
---
# Prospect-Theory Anomaly Scorecard

## Intuition

The model's verdict is graded by a simple rule on the predicted alpha spread between the extreme deciles. The model *helps explain* an anomaly when it predicts the correct sign of the empirical alpha difference and the predicted spread is substantial (above 1.5% in absolute value); it *fails* when it predicts a substantial spread of the wrong sign; and it *makes no strong prediction* when the predicted spread is under 1.5%. By this rule the model helps explain 14 of the 23 anomalies, makes no strong prediction on 2, and performs poorly on 7.
**Source:** Barberis, Jin & Wang (2021) §IV, §IV.A pp.35-37.

The 14 successes share a common mechanism: in each, the extreme decile with the lower empirical alpha contains stocks with more volatile returns, more positively skewed returns, and a more negative gain overhang. Higher volatility pushes the required return up, but the greater skewness and more negative overhang push it down, and quantitatively the latter two dominate -- so the model correctly predicts the low average return on these stocks. The successes include momentum, idiosyncratic volatility, failure probability, return-on-assets, gross profitability, expected idiosyncratic skewness, maximum daily return, Z-Score, gain overhang, external finance, composite equity issuance, net stock issuance, post-earnings announcement drift, and difference of opinion.
**Source:** Barberis, Jin & Wang (2021) §IV.A pp.37-38.

The 7 failures -- size, value, long-term reversal, short-term reversal, accrual, asset growth, and investment -- are anomalies where the same dominance of skewness and overhang pushes the prediction to the wrong sign. For size, the model wrongly predicts a *negative* alpha for the smallest-cap decile; for value, it predicts a *lower* return on value stocks, opposite to the value premium. Strikingly, for five of these seven, much of the anomaly return arrives around earnings-announcement dates, suggesting they are belief-driven (incorrect forecasts later corrected) rather than preference-driven -- exactly the kind of anomaly prospect-theory risk attitudes are not designed to capture.
**Source:** Barberis, Jin & Wang (2021) §IV.B pp.40-41.

## Definition

**Helps explain (eq. 28)** holds when `sign(alpha_d(10) - alpha_d(1)) = sign(alpha_m(10) - alpha_m(1))` and `|alpha_m(10) - alpha_m(1)| > 0.015`: the model gets the sign right and the predicted spread is at least 1.5% per year.
**Source:** Barberis, Jin & Wang (2021) §IV pp.35.

**Performs poorly (eq. 29)** holds when the model predicts a substantial spread (`> 0.015`) but of the *opposite* sign to the data; **no strong prediction (eq. 30)** holds when the predicted spread is below 1.5% in absolute value.
**Source:** Barberis, Jin & Wang (2021) §IV pp.36.

**Belief-driven vs. preference-based anomalies** -- anomalies whose returns concentrate around earnings announcements are interpreted as driven by incorrect beliefs corrected at announcement, distinct from preference-based anomalies driven by the risk attitudes prospect theory captures.
**Source:** Barberis, Jin & Wang (2021) §IV.B pp.40-41.

## Mathematical Reasoning

Model alpha for decile `l` is the CAPM intercept on the model-predicted return,

```
  alpha_m(l) = E(R_{100l}) - ( R_f + beta_{100l}(E(R_M) - R_f) ),
  with  E(R_M) = sum_i theta_{M,i} E(R_i).
```

The verdict compares the model spread `alpha_m(10) - alpha_m(1)` with the empirical spread `alpha_d(10) - alpha_d(1)`. For the 14 successes the model also reproduces the *concavity* of the empirical alpha pattern across deciles -- alphas are similar across most deciles but fall sharply as one approaches the most-skewed extreme decile -- a feature visible for volatility and failure-probability anomalies.
**Source:** Barberis, Jin & Wang (2021) §IV, §IV.A pp.35, 38.

Formal performance is graded by average absolute pricing error on the 23 long-short (decile-1-minus-decile-10) portfolios, compared with the CAPM, Fama-French three-factor, Carhart four-factor, and the five- and six-factor models. The prospect-theory model beats the CAPM and three-factor model, matches the four-factor model, and trails the five- and six-factor models -- notable because the factor models were built with knowledge of the anomalies while the prospect-theory model was not.
**Source:** Barberis, Jin & Wang (2021) §IV.D pp.42.

```
  Verdict group       count  examples
  -----------------   -----  --------------------------------------------
  helps explain         14   momentum, idio. vol, failure prob, ROA,
                             gross profitability, idio. skew, MAX, Z-Score,
                             gain overhang, external finance, CEI, NSI,
                             PEAD, difference of opinion
  no strong prediction   2   net operating assets, organizational capital
  performs poorly        7   size, value, long-term reversal, short-term
                             reversal, accrual, asset growth, investment
```

## See Also

- [be-bjw-anomaly-pricing-model](./be-bjw-anomaly-pricing-model.md#intuition) -- the model whose decile-by-decile predictions are scored here.
- [be-prospect-theory-ingredient-decomposition](./be-prospect-theory-ingredient-decomposition.md#intuition) -- which prospect-theory ingredient drives the successful predictions.
- [be-value-anomaly](./be-value-anomaly.md#intuition) -- the value premium the model predicts with the wrong sign.
- [be-momentum-anomaly](./be-momentum-anomaly.md#intuition) -- a flagship success of the model.
- [be-asset-pricing-anomalies-catalog](./be-asset-pricing-anomalies-catalog.md#intuition) -- the broader anomaly inventory this scorecard draws from.

## Escalate to Raw When

- You need Table III's exact decile-1 and decile-10 model alphas, model spreads, and empirical spreads for each of the 23 anomalies (referenced pp.36-37).
- You need Table IV's actual average absolute pricing errors for the prospect-theory model versus each factor model (referenced p.42).
- You need the precise earnings-announcement evidence for the five belief-driven failures, or the alternative-anomaly-set robustness check (Internet Appendix Sections IX, XIII; pp.40-41, 47).
