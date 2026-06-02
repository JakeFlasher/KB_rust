---
schema_version: "cacg.v0"
id: "be-rationally-heterogeneous-expectations"
title: "Rationally Heterogeneous Expectations in Learning-to-Forecast"
reading_id: "10_behavioral_finance"
summary: "Learning-to-forecast experiments: subjects coordinate within groups on simple trend-following/anchoring rules rather than the rational-expectations equilibrium, producing oscillations and bubbles; a Brock-Hommes heuristic-switching model with a few rules reproduces this rationally heterogeneous expectation formation."
tags: ["behavioral-finance", "learning-to-forecast", "heterogeneous-expectations", "heuristic-switching", "positive-feedback"]
citations:
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p523:0780"
    chunk_hash: "a624c3d4da3771b0e7a44e2866115bcab2f5bd248b8d140bdc1fcca80a4361e1"
    page_range: [523, 523]
    quote: "there is tremendous coordination on price forecasts within each group, a kind of group specific expectation norm."
    edge_type: "supports"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p523:0780"
    chunk_hash: "a624c3d4da3771b0e7a44e2866115bcab2f5bd248b8d140bdc1fcca80a4361e1"
    page_range: [523, 523]
    quote: "is found to be positive, indicating that if subjects see a positive (negative) trend in the past two prices, they expect prices to continue to increase (decrease)."
    edge_type: "supports"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p525:0782"
    chunk_hash: "ab3d3bf90e7c04ac654e81de696fc0010cae559e0eaa4556bff6d8872afbda6b"
    page_range: [525, 525]
    quote: "Anufriev and Hommes (2012) therefore propose the use of a heuristic switching model, based on Brock and Hommes (1997) to explain the"
    edge_type: "defines"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p523:0780"
    chunk_hash: "a624c3d4da3771b0e7a44e2866115bcab2f5bd248b8d140bdc1fcca80a4361e1"
    page_range: [524, 524]
    quote: "Hommes et al. conclude that 75 percent of their subjects can be classified using linear adaptive rules that depart from the rational expectations equilibrium prediction."
    edge_type: "supports"
card_hash: "b13338017269d641d5e9326e4a8f8ab2f214e93bb297b1a53287c9dc1e0445d7"
---
# Rationally Heterogeneous Expectations in Learning-to-Forecast

## Intuition

In a learning-to-forecast experiment, subjects are paid only to PREDICT next period's price; their forecasts are aggregated and fed into the market equation that actually determines the price, closing a belief-outcome loop. The striking finding (Hommes et al. 2005) is that subjects do not coordinate on the rational-expectations equilibrium. Instead, each group of six forecasters coordinates internally on a simple rule — a kind of group-specific "expectation norm" — and these rules are typically trend-extrapolating, so prices oscillate, sometimes converging, sometimes producing persistent bubbles. About three-quarters of subjects are classifiable by linear adaptive rules that DEPART from the rational prediction. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.504-507.

The phenomenon is "rationally heterogeneous expectations" in the sense that the heterogeneity is not random error but a structured, persistent mix of boundedly rational forecasting rules selected by their recent success. When the system has positive feedback (higher expectations raise realized prices, as in the asset market), trend-extrapolation is locally self-validating, which is exactly why coordination on trend rules generates the observed oscillations and bubbles. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.505-505.

A single representative learning rule cannot capture this. Anufriev and Hommes (2012) instead use a Brock-Hommes heuristic-switching model: a small menu of forecasting heuristics competes, each period weighted by its recent forecasting accuracy through a logit rule, so the population mix evolves endogenously. This few-rule model reproduces both the converging and the oscillating/bubbling groups, providing a micro-foundation for the experimental data. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.507-507.

## Definition

**Learning-to-forecast experiment** is a design in which subjects are paid for the accuracy of their price forecasts, and the aggregate of those forecasts feeds the data-generating price equation, so expectations and outcomes interact endogenously. **Source:** Arifovic and Duffy (2018) §2.3 pp.504-505.

**Group expectation norm** is the within-group coordination of forecasts on a common (often trend-following) prediction rule, even though different groups coordinate on different rules and few reach the rational-expectations equilibrium. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.505-505.

**Heuristic switching model (HSM)** is the Brock-Hommes-based model in which several fixed forecasting heuristics are aggregated with logit weights `n_{h,t}` updated by each rule's recent performance, generating an endogenous, evolving mix of expectations. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.507-507.

## Mathematical Reasoning

The aggregate forecast entering the price equation is `p-bar^e_{t+1} = sum_{h=1}^{H} n_{h,t} p^e_{h,t+1}`. Each heuristic's fitness is updated by past squared forecast error, `U_{h,t-1} = mu U_{h,t-2} - (p_{t-1} - p^e_{h,t-1})^2` with memory `mu in (0,1)`, and the weights follow the logit `n_{h,t} = lambda n_{h,t-1} + (1-lambda) [e^{beta U_{h,t-1}} / sum_h e^{beta U_{h,t-1}}]`, where `lambda` is inertia and `beta` the intensity of choice. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.507-507.

The four heuristics estimated from subjects' individual forecasts are:

```
   ADA  Adaptive expectations  : p^e_{t+1} = 0.65 p_{t-1} + 0.35 p^e_t
   WTR  Weak trend following   : p^e_{t+1} = p_{t-1} + 0.4 (p_{t-1} - p_{t-2})
   STR  Strong trend following : p^e_{t+1} = p_{t-1} + 1.3 (p_{t-1} - p_{t-2})
   LAA  Anchoring & adjustment : p^e_{t+1} = 0.5[(t-1)^{-1} sum_{j=0}^{t-1} p_j + p_{t-1}]
                                              + (p_{t-1} - p_{t-2})
```

The trend rules extrapolate the last two price moves; the LAA rule anchors on a sample average and adds a trend term. In groups that follow oscillatory paths, the flexible LAA rule becomes dominant because it tracks turning points, while ADA and the trend rules lose weight; in converging groups all four retain similar weight. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.507-508.

Individual subjects' forecasts in the Hommes et al. design are well described by an AR(2) form `p^e_{t+1} = alpha + beta p_{t-1} + gamma(p_{t-1} - p_{t-2})` with estimated `gamma > 0` (positive trend extrapolation), which is what produces the oscillatory price pattern under positive feedback. (The source reports these as estimated rules, not derived ones.) **Source:** Arifovic and Duffy (2018) §2.3.1 pp.505-505.

## See Also

- [be-brock-hommes-switching](./be-brock-hommes-switching.md#intuition) — the adaptive belief / logit fitness-switching that the heuristic-switching model adapts to lab data.
- [be-experimental-asset-bubbles](./be-experimental-asset-bubbles.md#intuition) — the bubble outcomes these trend-following expectations generate in the asset-market experiment.
- [be-keynesian-beauty-contest-level-k](./be-keynesian-beauty-contest-level-k.md#intuition) — the bounded-depth-of-reasoning view of coordination on simple rules.
- [be-fundamentalist-chartist-ham](./be-fundamentalist-chartist-ham.md#intuition) — the fundamentalist/chartist rule split mirrored in the experimental heuristics.

## Escalate to Raw When

- The exact experimental design (robot fundamentalists, forecast intervals, scoring rules) of Hommes et al. (2005, 2008) is needed. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.504-506.
- The full HSM parameterization (`beta`, `lambda`, `mu`, `delta`) and the in/out-of-sample fit results must be quoted. **Source:** Arifovic and Duffy (2018) §2.3.1 pp.507-508.
- The cobweb (negative-feedback) experiments and the genetic-algorithm / individual-evolutionary-learning alternatives require the source's later sections. **Source:** Arifovic and Duffy (2018) §2.3.2-2.3.3 pp.508-513.
