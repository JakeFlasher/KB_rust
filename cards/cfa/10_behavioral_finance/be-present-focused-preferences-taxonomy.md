---
schema_version: "cacg.v0"
id: "be-present-focused-preferences-taxonomy"
title: "Present-Focused Preferences Taxonomy"
reading_id: "10_behavioral_finance"
summary: "Ericson-Laibson's present-focused preferences as a meta-category for intertemporal-choice models, classified along two axes (dynamic consistency, taste for commitment), with present bias as one special case."
tags: ["behavioral-finance", "intertemporal-choice", "present-bias", "time-inconsistency"]
citations:
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p013:0015"
    chunk_hash: "d78d6971636709870dc8e2a9815f272e908ee8a293f47fa7514400a58000a31f"
    page_range: [14, 14]
    quote: "Present-focused preferences exist if agents are more likely in the present to choose an action that generates immediate experienced utility"
    edge_type: "defines"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p015:0017"
    chunk_hash: "57a75170b00e5324e522cd8d8314f64f332303d2d414e59b2dfa85abf5e4f7a7"
    page_range: [15, 15]
    quote: "Preferences can be dynamically consistent even if choices are dynamically inconsistent."
    edge_type: "supports"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p027:0038"
    chunk_hash: "1654695c32bd577a541bbd3ba0d4cdd9b25f733d80b577a1036918710f97009a"
    page_range: [27, 27]
    quote: "These models share the property that agents choose more impatiently for the present than when they choose for the future."
    edge_type: "supports"
card_hash: "5ea51ac27178baff6528dbf24d7c685c83b9695db5e7c01e1dfbcd89273c7ede"
---
# Present-Focused Preferences Taxonomy

## Intuition
Almost all decisions weigh utility flows that arrive at different points in time, and people consistently struggle to make those tradeoffs in a way that gives "some special priority to the present." Rather than committing to one micro-foundation, Ericson and Laibson introduce a deliberately broad meta-category: a model has *present-focused preferences* if, at the moment of choice, the agent leans more heavily toward immediate experienced utility than she would if the entire choice set were pushed back by a uniform delay. The label "present-focus" is chosen over "present-bias" precisely because "bias" prejudges the behavior to be a mistake, and several present-focused models are fully welfare-consistent.
**Source:** Ericson & Laibson (2019) Ch.1 §1-2 pp.13-14.

The value of the meta-category is that it groups a large, diverse family of models — hyperbolic and quasi-hyperbolic discounting, temptation models, dual-self/planner-doer models, objective counter-party risks, psychometric distortions, and myopia — under one testable property, while still letting two crosscutting dimensions distinguish them: whether the model induces *dynamically inconsistent* choices, and whether it generates a *taste for commitment*. The chapter organizes the entire literature into a 2x2 table along exactly these two axes.
**Source:** Ericson & Laibson (2019) Ch.1 §2 pp.13-14, 27-28.

## Definition
**Present-focused preferences** are present when agents are more likely in the present to choose an action that generates immediate experienced utility than they would be if all consequences of the actions in their choice set were delayed by the same amount of time.
**Source:** Ericson & Laibson (2019) Ch.1 §2 pp.13-14.

**Dynamic consistency of preferences** holds between dates `t` and `t' > t` when a person's state-contingent preferences for actions at `t'`, expressed at date `t`, coincide with the state-contingent preferences for the same actions expressed at date `t'`. Crucially, *preferences* can be dynamically consistent even when *choices* are dynamically inconsistent.
**Source:** Ericson & Laibson (2019) Ch.1 §2 p.15.

**Taste for commitment** is a strictly preferred restriction of one's own future choice set, arising from pure intra-personal mechanisms (e.g. a smoker who flushes her cigarettes), excluding inter-personal strategic motives.
**Source:** Ericson & Laibson (2019) Ch.1 §2 p.15.

## Mathematical Reasoning
The taxonomy is organized as a 2x2 classification over two properties: (i) the *dynamic consistency of preferences*, and (ii) whether the model generates a taste for commitment. Present-biased preferences occupy the cell {dynamically inconsistent preferences, taste for commitment if sophisticated}; temptation models occupy {dynamically consistent preferences, taste for commitment}; present-bias with perfect naivete, psychometric distortions, and myopia models occupy {dynamically inconsistent preferences, no taste for commitment}.
**Source:** Ericson & Laibson (2019) Ch.1 §2 pp.13-14.

Formally, define state-contingent preferences held at date `t` over actions implemented at `t'`, and compare them to those held at `t'`. Dynamic inconsistency in self-control "arises if there is any pair of values `t` and `t' > t`, which is not characterized by dynamic consistency in preferences." The key conceptual separation is that a model can produce inconsistent *choices* (e.g. choosing `x` over `y` at `t` for date `t'` but `y` over `x` at `t'`) while the underlying *preferences* remain consistent — for instance noisy-perception myopia, where the agent has consistent underlying preferences she cannot act on cleanly.
**Source:** Ericson & Laibson (2019) Ch.1 §2 p.15.

```
   dynamic consistency       taste for commitment?
   of preferences             YES            NO
                        +--------------+--------------+
   dynamically          | present-bias | present-bias |
   inconsistent         | (if sophist.)| w/ perfect   |
                        |              | naivete;     |
                        |              | psychometric |
                        |              | distortions; |
                        |              | myopia        |
                        +--------------+--------------+
   dynamically          | temptation   | (rational    |
   consistent           | / unitary-   |  benchmark)  |
                        | self models  |              |
                        +--------------+--------------+
```
**Source:** Ericson & Laibson (2019) Ch.1 §2 pp.13-14.

## See Also
- [be-quasi-hyperbolic-discounting](./be-quasi-hyperbolic-discounting.md#intuition) — the canonical present-biased special case (β-δ).
- [be-commitment-and-naivete](./be-commitment-and-naivete.md#intuition) — when present-focus induces a taste for commitment and how naivete blunts it.
- [be-household-liquidity-illiquidity-puzzle](./be-household-liquidity-illiquidity-puzzle.md#intuition) — a leading empirical regularity attributed to present bias.
- [be-sparsity-attention-framework](./be-sparsity-attention-framework.md#intuition) — Gabaix recasts global inattention to the future as a present-focused phenomenon.

## Escalate to Raw When
- You need the full enumeration of which models occupy each of the four boxes (temptation, dual-self, objective counter-party risk, psychometric distortion, myopia). **Source:** Ericson & Laibson (2019) Ch.1 §2.7 pp.27-28.
- You need the precise state-contingency machinery (twisted-ankle / absent-tempting-good examples) used to define dynamic consistency. **Source:** Ericson & Laibson (2019) Ch.1 §2 p.15.
