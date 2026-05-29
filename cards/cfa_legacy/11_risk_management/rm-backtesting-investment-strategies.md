---
schema_version: "cacg.v0"
id: "rm-backtesting-investment-strategies"
title: "Backtesting Investment Strategies — L1 Notes Exceedance-Counting Framework"
reading_id: "11_risk_management"
summary: "Backtesting in the McNeil Ch.9 §9.3 sense compares out-of-sample VaR estimates against realized losses via exceedance counting: under correct calibration the violation indicators form an iid Bernoulli(1−α) process whose count is binomial(T, 1−α)."
tags: ["risk-management", "backtesting-investment"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p372:0532"
    chunk_hash: "1a65a6be3b11053e68bffca5543ed3d49005c0b545271cca0dbfda08baaa3ac5"
    page_range: [372, 373]
    quote: "Backtesting is the practice of evaluating risk measurement procedures by comparing out-of-sample estimates of risk measures with actual realized losses and gains."
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p373:0533"
    chunk_hash: "dd0ec426b41e36714cd8bb616af0792935d4eabac5afae5ee48736c10aba7e14"
    page_range: [373, 374]
    quote: "Moreover, the following lemma shows that the sequence of VaR violation indicators (It) forms a Bernoulli trials process, i.e."
    edge_type: "supports"
card_hash: "c696f4d4d3286628a4e17869c7ddc3440e2eb835c8a314b02d37d4b0196a8903"
---
# Backtesting Investment Strategies — L1 Notes Exceedance-Counting Framework

## Intuition

**Backtesting** in a risk-management frame asks: did our forecast loss measure (typically `VaR_α`) match realised losses out-of-sample? The L1 source' core test is **exceedance counting**: over a backtest window of `T` periods, count the number of periods `N` in which the realised loss exceeded the period's forecast VaR. Under a correctly-calibrated `VaR_α`, exceedances should occur with probability `1 − α` per period, so the expected exceedance count is `E[N] = (1 − α) · T` with binomial variance `(1 − α) · α · T`. A backtest "passes" if the observed `N` sits inside a plausibility corridor around `E[N]`; outside the corridor, the VaR estimator is either under-conservative (too many exceedances) or over-conservative (too few). **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.351-365.

The source-side framing treats backtesting as **strategy-evaluation discipline**, not statistical hypothesis testing. The L1 reader's takeaways are: (1) declare the forecast model and its parameters before the backtest window opens; (2) record exceedances honestly without re-fitting in-sample; (3) compare against a plausibility envelope built from `(1 − α)` per period; (4) flag a VaR estimator that produces a count outside the envelope for re-calibration. The McNeil-side formal violation-based tests (unconditional coverage, independence, conditional coverage, dynamic-quantile, ES-specific tests) defer to future-01 quantitative econometrics. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.351-365 + McNeil et al. (2015) Ch.9 pp.351-365.

The structural caveat: exceedance counting tells you about **frequency** of tail breaches but says nothing about **severity** when they occur. A VaR estimator can pass exceedance counting while systematically under-estimating the average tail loss past `VaR_α`. ES-specific backtesting addresses severity but is more involved (joint elicitability with VaR; see `[[rm-var-and-es-taxonomy]]`). The source flags this limitation and forward-link without developing it. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.351-365 + McNeil et al. (2015) Ch.9 pp.354-356.

```
<!-- primitive: backtest-failure-cones source: _diagram_primitives.md -->
   cumulative exceedances over T periods
   ^
   |                                                 .
   |                                               .   <- upper band (reject)
   |                                          . .
   |                                       . .
   |                                    . .
   |                                 . .
   |                              . . __________
   |                           . .   /            \
   |                        . .     / observed     \   <- pass corridor
   |                     . .       (cumulative)     \
   |                  . . ___________________________\
   |               . .
   |            . .                          <- lower band (anti-conservative)
   |         . .
   |       .
   +----------------------------------------------------> period t (1..T)

   expected exceedance count: E[N] = (1 − α) · T
   alpha-band corridor: Bernoulli (1 − α) confidence interval around E[N]
   pass corridor: observed N stays inside bands across T periods
   above upper band: VaR underestimates tail (too few exceedances expected)
   below lower band: VaR overestimates tail (too many exceedances allowed)
```

## Definition

Let `{(L_t, V̂_t)}` for `t ∈ {1, …, T}` be a backtest series consisting of realised one-period losses `L_t` and forecast `VaR_α` numbers `V̂_t` (each `V̂_t` produced before observing `L_t`). Define the **exceedance indicator** for period `t` as: **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.351-365 + McNeil et al. (2015) Ch.9 pp.351-353.

```
I_t  =  1{ L_t  >  V̂_t }       (1 if the period was an exceedance, else 0)
```

The **total exceedance count** is `N = Σ_t I_t` (summed across the backtest window). Under correctly-calibrated VaR with i.i.d. exceedance events, `I_t ~ Bernoulli(1 − α)` and `N ~ Binomial(T, 1 − α)`, giving: **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.351-365 + McNeil et al. (2015) Ch.9 pp.352.

```
E[N]   =  (1 − α) · T
Var[N] =  (1 − α) · α · T
```

The L1 source' **unconditional-coverage check** declares the VaR forecast acceptable if `N` lies inside the central Bernoulli interval around `E[N]` at a stated outer confidence level. Outside the interval, the estimator is rejected for re-calibration; the source does not prescribe the outer-confidence number, leaving it to the risk-function policy. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.351-365.

Two failure modes the source names explicitly: **clustering** (exceedances bunch together, violating the i.i.d. assumption — a count `N` may pass unconditional coverage while the temporal dependence flags a stale volatility model) and **drift** (exceedances increase over time, suggesting the VaR estimator is anchored to a stale regime and is no longer calibrated to current conditions). The McNeil-side formal independence + dynamic-quantile tests defer to future-01. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.351-365 + McNeil et al. (2015) Ch.9 pp.353-356.

## Mathematical Reasoning

The Bernoulli model for exceedances is the **null hypothesis of correct VaR calibration**: under the null, each period independently breaches VaR with probability `1 − α`, so the count statistic is binomial. The plausibility corridor around `E[N]` is the central region of the binomial distribution at the outer confidence level, and the test rejects when `N` falls outside that region. The corridor widens with `√T` (binomial SD) and tightens with larger `α` (smaller `1 − α` shrinks the expected count and its variance jointly). This is the foundation of the **failure-cones** visualisation: cumulative `N_t = Σ_{s ≤ t} I_s` traces a path that, under the null, stays inside a slowly-widening band; a path that exits the band signals model failure. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.351-365 + McNeil et al. (2015) Ch.9 pp.352-353.

The **independence assumption** is the load-bearing one. Real loss series exhibit volatility clustering: a single bad day raises the probability of the next day being bad too. Under clustering, the exceedance series `{I_t}` is no longer i.i.d. Bernoulli; the count `N` retains the same `E[N]` but exhibits inflated variance (a binomial-overdispersion phenomenon). An unconditional-coverage test that uses the i.i.d. binomial corridor will accept the model more often than nominal level under clustering — a false-negative bias. The source flags the clustering caveat and forward to McNeil's independence + Christoffersen conditional-coverage test for the formal correction (out of scope at L1 depth). **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.351-365 + McNeil et al. (2015) Ch.9 pp.353-356.

Backtesting is also subject to **window-length trade-offs**. A short window (small `T`) gives wide corridors and weak rejection power: a genuinely-broken VaR estimator can produce an `N` that still fits inside the wide corridor and passes. A long window (large `T`) tightens the corridor and increases rejection power but pulls in observations from earlier regimes where the VaR estimator may have been calibrated to different conditions. The L1 source recommend declaring the backtest window in advance and not adjusting it post-hoc — a window chosen after seeing `N` is invalid (in-sample over-fitting of the test). **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.351-365.

The **strategy-evaluation framing** generalises beyond VaR backtesting: any forecast statistic produced by an investment strategy (return forecast, alpha, signal-rank, drawdown bound) admits an analogous out-of-sample-versus-realised test. The source' contribution is to frame backtesting as a **discipline of pre-declared forecasts + honest accounting** rather than as a specific statistical procedure. The risk-management value is not the test statistic but the institutional commitment to disclose model failures rather than refit silently. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.351-365.

The **ES-specific gap** matters going forward: under the elicitability framework, VaR is elicitable (the pinball/quantile-loss scoring function works — comparing two competing VaR forecasters by mean pinball loss is theoretically clean) but ES is not elicitable on its own (it requires joint scoring with VaR). The source acknowledges this asymmetry without resolving it; the operator who needs to backtest ES forecasts must use either a joint scoring rule with the VaR forecast OR a violation-distribution test that compares the conditional exceedance distribution rather than a point score. Full treatment lives in future-01 quantitative methods. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.351-365 + McNeil et al. (2015) Ch.9 pp.354-356.

## See Also

- [rm-value-at-risk-notes](./rm-value-at-risk-notes.md) — Batch-1 sibling card defining the VaR statistic that backtesting evaluates.
- [rm-var-and-es-taxonomy](./rm-var-and-es-taxonomy.md) — Batch-0 card with the VaR-vs-ES coherence contrast that motivates the ES-backtesting gap.

## Escalate to Raw When

The L1-source treatment stops at unconditional exceedance counting + the cluster / drift caveats. When the operator needs the full formal apparatus (Christoffersen unconditional + independence + conditional coverage tests, dynamic-quantile tests, ES-specific scoring under joint elicitability with VaR, regression-based backtests for spectral / distortion risk measures), open McNeil Ch.9 §9.3 pp.351-365 directly. Full statistical-power analysis and Monte Carlo size-and-power simulation belong to future-01 quantitative econometrics. **Source:** McNeil et al. (2015) Ch.9 pp.351-365.
