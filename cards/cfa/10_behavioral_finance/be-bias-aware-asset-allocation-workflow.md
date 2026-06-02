---
schema_version: "cacg.v0"
id: "be-bias-aware-asset-allocation-workflow"
title: "Bias-Aware Asset-Allocation Workflow"
reading_id: "10_behavioral_finance"
summary: "The best-practical-allocation workflow: diagnose client biases, decide moderate-or-adapt, then adjust the rational mean-variance allocation within a bounded deviation (~20%) to a behaviorally adjusted, livable portfolio the client can adhere to."
tags: ["behavioral-finance", "asset-allocation", "best-practical-allocation", "mean-variance", "workflow"]
citations:
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p062:0063"
    chunk_hash: "981529c06ac2cbc8adf144a43bca0a68b440e11347ce18adcb75bcfdb78e73c4"
    page_range: [62, 62]
    quote: "best practical allocation may be a slightly underperforming long-term investment program to which the client can comfortably"
    edge_type: "defines"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p061:0062"
    chunk_hash: "74e021f5f6f5d7a11cc2fe168a623925713c3f38a2d152b78d41208fb20645e3"
    page_range: [61, 61]
    quote: "risk tolerance questionnaires provide, at best, broad guidelines for asset allocation and should only be used in concert with other"
    edge_type: "supports"
  - source_id: "bf_pompian_2006_bfwm"
    chunk_id: "bf_pompian_2006_bfwm:p064:0066"
    chunk_hash: "a85a48a6e56a0ce97f187aa94caad504bee7c5b80a4baa6ea51978d5fa263529"
    page_range: [65, 65]
    quote: "a behaviorally adjusted allocation should not stray more than 20 percent from the mean-variance-optimized allocation."
    edge_type: "supports"
card_hash: "01b6b37d0140e1883a9a291130740b36a4c63c9d68af1eebb12ca69272324773"
---
# Bias-Aware Asset-Allocation Workflow

## Intuition

The end product Pompian targets is the *best practical allocation*: the right allocation is the one that helps the client reach financial goals while providing enough psychological security for the client to sleep at night and stick to the plan. It may be a slightly underperforming long-term program the client can comfortably adhere to — warding off the impulse to "change horses" mid-race — because moving repeatedly in and out of an allocation causes serious long-term damage. A mean-variance-optimal portfolio the client abandons in a downturn is worse than a marginally suboptimal one the client keeps.
**Source:** Pompian (2006) Ch.3 pp.42.

The workflow is: (1) recognize that risk-tolerance questionnaires alone are insufficient — they are imprecise, administered once, interpreted too literally, and ignore behavioral issues, so they provide at best broad guidelines to be used in concert with behavioral assessment; (2) diagnose the client's biases via diagnostic questions or case-study assessment; (3) apply the moderate-or-adapt rule (cognitive/emotional x wealth) to decide whether and how to deviate from the rational allocation; (4) compute the rational mean-variance allocation and adjust it within a bounded deviation; (5) embed the result in a behaviorally aware investment plan. Biases must be identified *before* the allocation is executed.
**Source:** Pompian (2006) Ch.3 pp.41-42, pp.43.

## Definition

**Best practical allocation** is a behaviorally adjusted allocation — possibly slightly underperforming the mean-variance optimum — that the client can comfortably adhere to long-term, balancing goal attainment against psychological security.
**Source:** Pompian (2006) Ch.3 pp.40, pp.42.

**Risk-tolerance questionnaire limitation** is the recognition that such questionnaires give imprecise, format-sensitive, literally-misread, once-administered, behavior-blind results and must be used only alongside behavioral assessment tools.
**Source:** Pompian (2006) Ch.3 pp.41.

**Bias adjustment factor** is the bounded magnitude of discretionary deviation from the mean-variance output; Pompian recommends a behaviorally adjusted allocation stray no more than ~20% from the mean-variance-optimized allocation.
**Source:** Pompian (2006) Ch.3 pp.45-46.

## Mathematical Reasoning

The deviation is bounded by an explicit, named algorithm. Let `M_i` be the mean-variance output weight for asset class `i` and `B_i` the bias-adjusted weight. Pompian's "Method for Determining Appropriate Deviations from the Rational Portfolio": (1) form the difference `D_i = M_i - B_i`; (2) take the absolute percentage change `|D_i| / M_i`; (3) weight each percentage change by the mean-variance base `M_i` and sum to get the *bias adjustment factor*. The constraint is `bias adjustment factor <= 20%`. The rationale: most investment policy statements permit discretionary asset-class ranges of +/-10% in either direction (a 20% band), so a 60/40 equity/fixed-income prototype admits routine 50-70% equity and 30-50% fixed-income adjustments.
**Source:** Pompian (2006) Ch.3 pp.45-46.

```
 DIAGNOSE biases  ->  CLASSIFY type    ->  MODERATE-or-ADAPT
 (questions /         (cognitive vs        (Principles I & II,
  case study)          emotional;           per bias)
                       wealth level)              |
                                                  v
 RATIONAL alloc  -> ADJUST within   -> BEST PRACTICAL
 (mean-variance)    +/-20% band         allocation -> IPS
                    (bias adj. factor)  (client can adhere to)
```

The workflow is intentionally not prescriptive-absolute: the principles "should be consulted along with other data on risk tolerance, financial goals, asset class preferences." The math caps *how far* an adaptation may go; the moderate-vs-adapt grid decides *whether and which direction*. (The source gives the three-step method and the 20% cap as a recommended guideline, illustrated on two hypothetical clients whose adjustment factors do not exceed 20%; per Critical Rule 1 the per-client arithmetic is not reproduced here.)
**Source:** Pompian (2006) Ch.3 pp.43, pp.46-47.

## See Also

- [be-moderate-vs-adapt-bias-allocation](./be-moderate-vs-adapt-bias-allocation.md#intuition) — the decision rule this workflow operationalizes.
- [be-cognitive-vs-emotional-bias-taxonomy](./be-cognitive-vs-emotional-bias-taxonomy.md#intuition) — the bias classification fed into the diagnose step.
- [be-self-control-mental-accounting](./be-self-control-mental-accounting.md#intuition) — the pyramid-portfolio failure this correlation-aware workflow guards against.

## Escalate to Raw When

- You need the verbatim three-step "Method for Determining Appropriate Deviations" and Tables 3.1/3.2 (Mr. Jones, Adams Family) to see the bias-adjustment-factor computation.
**Source:** Pompian (2006) Ch.3 pp.46-47.
- You need Pompian's full critique of risk-tolerance questionnaires (Sharpe's discounting, format sensitivity, the 20%-loss literalism example).
**Source:** Pompian (2006) Ch.3 pp.41.
- You need the per-bias diagnostic question sets that drive the "diagnose" step for a specific bias chapter.
**Source:** Pompian (2006) Ch.3 pp.43.
