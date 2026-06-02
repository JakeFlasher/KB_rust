---
schema_version: "cacg.v0"
id: "be-representativeness-conjunction-base-rate"
title: "Representativeness, Conjunction Fallacy, Base-Rate Neglect"
reading_id: "10_behavioral_finance"
summary: "Kahneman-Tversky's representativeness heuristic judges likelihood by similarity, and accounts for two robust errors -- the conjunction fallacy (Linda) and base-rate neglect (overreaction to a positive medical test) -- the psychology Gennaioli-Shleifer formalize."
tags: ["behavioral-finance", "representativeness", "heuristics", "judgment-biases"]
citations:
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p155:0147"
    chunk_hash: "de8da0f3dc3687cba0e567d3110593c17bce46989a1e005a551ddfa4ff9af5b1"
    page_range: [155, 155]
    quote: "Kahneman and Tversky describe the representativeness heuristic as our tendency to judge likelihood by similarity."
    edge_type: "defines"
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p155:0147"
    chunk_hash: "de8da0f3dc3687cba0e567d3110593c17bce46989a1e005a551ddfa4ff9af5b1"
    page_range: [156, 156]
    quote: "The conjunction fallacy refers to the mistake of judging an event A ∩ B to be more likely than either A or B alone."
    edge_type: "supports"
  - source_id: "bf_gennaioli_shleifer_2018_crisis_beliefs"
    chunk_id: "bf_gennaioli_shleifer_2018_crisis_beliefs:p156:0148"
    chunk_hash: "306be575553e36a208ad0f142c121a845c166ab304a1cc6d493f4b2bd3548dfc"
    page_range: [157, 157]
    quote: "Base rate neglect refers to the tendency for individuals to react too strongly to information"
    edge_type: "supports"
card_hash: "26ffdc793c4a750cf776049ee15e2935dadb19ae5faed26bc813ce7bf140710c"
---
# Representativeness, Conjunction Fallacy, Base-Rate Neglect

## Intuition

Starting in the early 1970s, Kahneman and Tversky collected laboratory findings of systematic departures of human judgment from Bayesian inference, and explained them by positing a few judgment heuristics -- representativeness, availability, and anchoring. Heuristics are rules of thumb that speed up cognition and often yield good approximate answers, but produce incorrect answers in some situations. The representativeness heuristic is the tendency to judge likelihood by similarity: a shy, orderly, detail-loving man is judged more likely to be a librarian than a farmer because his attributes resemble those of a typical librarian, even though male farmers vastly outnumber male librarians.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.142.

The power -- and danger -- of representativeness is that likelihood and similarity do not always go together. Conflating the two makes people neglect how common one type is relative to another in the population. This single heuristic accounts for disparate documented phenomena, including the conjunction fallacy and base-rate neglect, and it is the psychological raw material Gennaioli and Shleifer formalize into a tractable operator on probability distributions.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.142-143.

## Definition

**Representativeness heuristic** is the tendency to judge the likelihood of an event by its similarity to a stereotype or typical case, rather than by its actual relative frequency.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.142.

**Conjunction fallacy** is the mistake of judging a conjunction `A and B` to be more likely than one of its constituent events `A` or `B` alone -- a logical impossibility, since the conjunction is contained in each constituent.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.143.

**Base-rate neglect** is the tendency to react too strongly to case-specific information and to underweight the prior (base-rate) frequency of the hypothesis, as when doctors attach too high a probability to a patient being sick after a positive test for a rare disease.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.144.

## Mathematical Reasoning

The **Linda problem** exposes the conjunction fallacy. Respondents rank the likelihood that Linda -- described as outspoken, concerned with discrimination, a former antinuclear demonstrator -- is `(1)` a bank teller versus `(2)` a bank teller active in the feminist movement. Most rank `(2)` above `(1)`, yet probability theory requires `Pr(bank teller and feminist) <= Pr(bank teller)` because option 1 contains option 2. Similarity drives the error: Linda resembles the *stereotypical* feminist far more than the stereotypical (apolitical) bank teller.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.143.

**Base-rate neglect** is exposed by the medical-test problem. By Bayes' rule,

```
  Pr(sick | positive) = Pr(positive | sick) * Pr(sick) / Pr(positive)
```

If the base rate `Pr(sick)` is small, a positive test is very likely a false positive and the patient is probably still healthy. Doctors nonetheless overreact to the positive test, because a sick patient is *similar* to someone who tested positive -- representativeness inflates `Pr(sick | positive)` and suppresses the base rate.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.143-144.

The unifying lesson Gennaioli and Shleifer draw is that an attribute is judged representative of a class when it is *diagnostic* -- its relative frequency is much higher in that class than in a relevant reference class -- so representativeness is a statement about relative, not absolute, likelihood. This relative-frequency reading is exactly what licenses their formal operator built on a likelihood ratio.
**Source:** Gennaioli & Shleifer (2018) Ch.5 pp.144-145.

## See Also

- [be-kernel-of-truth](./be-kernel-of-truth.md#intuition) -- why representativeness distorts beliefs in the direction of true differences between classes.
- [be-diagnostic-expectations](./be-diagnostic-expectations.md#mathematical-reasoning) -- the formal operator that turns this psychology into a distorted density.
- [be-rational-vs-diagnostic-expectations](./be-rational-vs-diagnostic-expectations.md#intuition) -- how the heuristic generates overreaction relative to Bayesian norms.

## Escalate to Raw When

- You need the full Linda vignette text or the complete ranked option list as administered (pp.143).
- You need the precise statement of the bank-teller likelihood-ratio argument (feminist vs nonfeminist interference) used to derive the conjunction fallacy (pp.149-150).
- You need the references to the original Kahneman-Tversky (1974, 1983) and Casscells et al. (1978) experiments (pp.142, 144).
