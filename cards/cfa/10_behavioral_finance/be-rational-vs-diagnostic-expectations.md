---
schema_version: "cacg.v0"
id: "be-rational-vs-diagnostic-expectations"
title: "Diagnostic vs Rational (and Adaptive) Expectations"
reading_id: "10_behavioral_finance"
summary: "Diagnostic expectations nest rational expectations at theta=0, are forward-looking unlike mechanical adaptive rules, and generate overreaction to news: forecast revisions and forecast errors are negatively correlated, so good news breeds excess optimism that predictably reverses."
tags: ["behavioral-finance", "diagnostic-expectations", "rational-expectations", "overreaction"]
citations:
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p184:0175"
    chunk_hash: "c2bbfbb77a7da813cef23d4cf8bbebefc70612a3c53235649354bbac893e23cc"
    page_range: [185, 185]
    quote: "expectations distort the rational expectation"
    edge_type: "defines"
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p185:0176"
    chunk_hash: "f1deb465324203baecf94e0423f2e55820881dcaf5153e8f8503c81f93df5f8a"
    page_range: [186, 186]
    quote: "when forecasters overreact to news, the correlation between their forecast revisions and forecast errors should be negative."
    edge_type: "supports"
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p188:0179"
    chunk_hash: "c8780ca6687461b0645862f5ce48e1331fe15cbe7252c5c504930aa5441c99b4"
    page_range: [189, 189]
    quote: "for given current news, updating about future outcomes is more aggressive in macroeconomic series characterized by higher"
    edge_type: "supports"
card_hash: "94b1ae05409a761f01d15ff2be8d8996f1dfb20edfea9b0ede075beca4351278"
---
# Diagnostic vs Rational (and Adaptive) Expectations

## Intuition

Three families of beliefs sit on a spectrum. Rational expectations (RE) use the true structure of the economy to form statistically optimal forecasts, so forecast errors are unpredictable. Adaptive/extrapolative expectations mechanically project past changes forward via a fixed coefficient, ignoring the data-generating structure -- powerful but Lucas-critique-vulnerable. Diagnostic expectations sit between: they are forward-looking like RE (they respond to regime changes and news), but they over-react to news because representativeness oversamples the states the news favors. Crucially, diagnostic expectations *nest* RE as the `theta = 0` special case, so the model is a strict generalization, not a rejection.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.170, 172.

The signature empirical fingerprint that distinguishes diagnostic from rational beliefs is overreaction: after good news the forecaster is too optimistic, so the subsequent forecast *error* (realization minus forecast) is negative on average, while the forecast *revision* was positive. Revisions and errors are therefore negatively correlated -- the opposite of RE (zero correlation) and of rational inattention (also zero for an individual forecaster). On average there is no future news, so optimism cools off and expectations revert toward rationality, generating predictable reversals.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.172-173, 175.

## Definition

**Rational expectation** of an AR(1) variable is `E_t(X_{t+1}) = rho*X_t`; forecast errors are mean-zero and unpredictable from time-`t` information.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.170.

**Diagnostic expectation** distorts the rational expectation in the direction of current news: `E^theta_t(X_{t+1}) = rho*X_t + rho*theta*(X_t - rho*X_{t-1})`, where `X_t - rho*X_{t-1}` is the news between `t-1` and `t`.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.172.

**Adaptive expectation** is the backward-looking rule `E^a_t(X_{t+1}) = X_t + mu*[X_t - E^a_{t-1}(X_t)]`, governed by a fixed coefficient `mu` independent of the true persistence `rho`.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.175-176.

## Mathematical Reasoning

For an AR(1) process `X_{t+1} = rho*X_t + eps_{t+1}` with i.i.d. shock variance `sigma^2`, Proposition 6.1 gives the believed distribution as normal with the same variance `sigma^2` but distorted mean

```
  E^theta_t(X_{t+1}) = rho*X_t + rho*theta*(X_t - rho*X_{t-1}).
```

If `rho > 0`, diagnostic beliefs look extrapolative: good news (`X_t > rho*X_{t-1}`) makes right-tail states representative and the forecast too optimistic. But extrapolation is *not* universal -- if `rho < 0`, diagnostic expectations exaggerate reversals of current conditions, a prediction that mechanical extrapolation cannot make.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.172.

The distinguishing property, holding for any `rho`, is overreaction, captured by the Coibion-Gorodnichenko covariance of forecast error with forecast revision:

```
  cov( X_{t+1} - E^theta_t(X_{t+1}),  E^theta_t(X_{t+1}) - E^theta_{t-1}(X_{t+1}) )
       = -theta*(1+theta)*rho^2*sigma^2  < 0   for theta > 0.
```

Under RE the same covariance is zero. The negative sign says positive revisions (good news) are followed by negative errors (disappointment) -- excess optimism after good news.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.173.

Two further contrasts with adaptive expectations: (i) the law of iterated expectations *fails* against the true process, `E_t[E^theta_{t+1}(X_{t+2})] = E_t(X_{t+2}) = rho^2*X_t`, so expectations systematically revert to rationality and mispricing self-corrects; (ii) diagnostic updating depends on the true persistence `rho` -- updating is more aggressive in more persistent series -- whereas mechanical adaptive beliefs are shaped only by the fixed `mu` and ignore `rho`. The data show negative error-revision correlation even for highly persistent variables like interest rates, favoring diagnostic over adaptive.
**Source:** Gennaioli & Shleifer (2018) Ch.6 pp.175-176.

## See Also

- [be-diagnostic-expectations](./be-diagnostic-expectations.md#mathematical-reasoning) -- the underlying operator and its lognormal closed form.
- [be-sentiment-vs-fundamentals](./be-sentiment-vs-fundamentals.md#intuition) -- a related framing of belief-driven mispricing relative to fundamentals.
- [be-extrapolation-from-recent-data](./be-extrapolation-from-recent-data.md#intuition) -- the survey evidence of predictable, extrapolative forecast errors.
- [be-belief-driven-credit-cycle](./be-belief-driven-credit-cycle.md#intuition) -- predictable reversals applied to debt issuance and spreads.

## Escalate to Raw When

- You need the exact stock-market (`rho=1`) and earnings (`cov = -theta*rho*sigma^2`) special cases (pp.174).
- You need the formal statement of the failure of the law of iterated expectations and its footnote (pp.175).
- You need the Coibion-Gorodnichenko test setup and the rational-inattention comparison footnotes (pp.173).
