---
schema_version: "cacg.v0"
id: "be-kernel-of-truth"
title: "Kernel of Truth"
reading_id: "10_behavioral_finance"
summary: "The kernel-of-truth principle: representativeness distorts beliefs by exaggerating real differences across types/states, amplifying outcomes objectively made more likely by recent news -- so biases move in the right direction but too far, and depend on measurable features of the world."
tags: ["behavioral-finance", "representativeness", "kernel-of-truth", "stereotypes"]
citations:
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p153:0145"
    chunk_hash: "e0b3314314bd85364fe87548c05c7062d084b4eba942b174255ebfd1ffcf06a7"
    page_range: [154, 154]
    quote: "Representativeness causes biases by exaggerating real patterns in the data."
    edge_type: "defines"
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p163:0155"
    chunk_hash: "9953702e673baba93fe26c69e331db9c7db0d64ad1c08c111f4314867f15b417"
    page_range: [164, 164]
    quote: "This idea that stereotypes exaggerate real differences between groups is referred to as"
    edge_type: "supports"
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p160:0151"
    chunk_hash: "0e2ab8e9b21c131373fa5486814d012edb14f6c9b6aac3628b504609eb5ad9dd"
    page_range: [160, 160]
    quote: "As a result, red hair is representative of Irish people."
    edge_type: "supports"
card_hash: "e89943524ce90ad5756194544df00fc31fd0b8afe8be016314266b4d01b7f63f"
---
# Kernel of Truth

## Intuition

The kernel-of-truth property is the disciplining feature that makes representativeness an empirically testable model rather than a free-form license to assume any bias. Because representativeness scores a type by its *relative* frequency in the target class versus a comparison class, the types that get overweighted are precisely those that are objectively more common in the target than elsewhere. Distortions therefore exaggerate *real* differences in the data: stereotypes are mental representations of genuine group differences, localized around the features that are most distinctive of the group. Biases move in the right direction but go too far.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.141, 151.

The canonical illustration is red hair and the Irish. Red hair is rare in absolute terms even among the Irish (only 10 percent), yet it is roughly ten times more prevalent among the Irish than in the world at large. That high *relative* prevalence makes red hair representative of the Irish, so memory oversamples it and inflates its judged frequency. The exaggeration is anchored to a true fact -- the Irish really are disproportionately red-haired -- which is the kernel of truth. The same logic transfers to finance: good news that raises the objective probability of high cash flows makes high cash flows representative, so beliefs over-amplify exactly the outcomes the news genuinely favored.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.146-147, 151.

## Definition

**Kernel of truth** is the property that representativeness-driven biases exaggerate real patterns in the data: the types or states that beliefs overweight are those whose true relative frequency in the target class genuinely exceeds their frequency in the comparison class.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.141, 151.

**Distinctive type** (representative type) is a type `tau` whose true frequency in the assessed group `G` is high relative to its frequency in a comparison group `-G`; such types are quickly recalled and overweighted, while non-distinctive types are underweighted or neglected.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.147-148.

## Mathematical Reasoning

Representativeness of type `tau` for group `G` is the likelihood ratio against a comparison group `-G`,

```
  R(tau, G) = h(T = tau | G) / h(T = tau | -G).
```

A type is representative -- and hence overweighted -- if and only if `R(tau, G) > 1`, i.e. it is *relatively* more frequent in `G` than in `-G`. The hair-color example makes the kernel of truth concrete via the ordering of likelihood ratios:

```
  Pr(red | Irish)/Pr(red | World)   = 10%/1%  = 10
        >  Pr(light/brown | Irish)/Pr(light/brown | World) = 40/14
        >  Pr(dark | Irish)/Pr(dark | World)               = 50/85.
```

Red hair has the highest likelihood ratio even though dark hair is the absolute majority among the Irish; representativeness orders types by `R(.)`, not by absolute frequency. (The source states the red ratio's value as 10 -- i.e. 10%/1% = 10 -- as an illustration of the ratio, not an exam computation.)
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.145-146.

The kernel of truth survives in the financial dynamic version because the comparison group is the *previous* period. The diagnostic mean is inflated above the truth, `mu_0(theta) > mu_0`, if and only if there is genuine good news about average cash flow, `mu_0 > mu_{-1}`; absent real news there is no distortion. Thus the model predicts *when* tail risk is neglected versus exaggerated as a function of fundamentals -- bad news with rising volatility makes left-tail states representative and over-weighted instead. The bias is real-difference-driven, hence testable.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.155-156, 160.

## See Also

- [be-representativeness-conjunction-base-rate](./be-representativeness-conjunction-base-rate.md#intuition) -- the heuristic and likelihood-ratio definition the kernel of truth refines.
- [be-diagnostic-expectations](./be-diagnostic-expectations.md#mathematical-reasoning) -- the operator in which the kernel of truth becomes `mu_0(theta) > mu_0 iff mu_0 > mu_{-1}`.
- [be-extrapolation-from-recent-data](./be-extrapolation-from-recent-data.md#intuition) -- analyst over-prediction of "future Googles" as kernel-of-truth exaggeration of a real fat tail.

## Escalate to Raw When

- You need the full hair-color frequency table (Irish vs World by red / light-brown / dark) as printed (pp.146).
- You need the social-stereotype evidence (political polarization, gender-domain trivia) used to validate the kernel of truth across contexts (pp.151-152).
- You need the precise memory-interference (fan-effect) account linking similarity to selective recall (pp.147-148).
