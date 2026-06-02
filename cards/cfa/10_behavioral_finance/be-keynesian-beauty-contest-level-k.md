---
schema_version: "cacg.v0"
id: "be-keynesian-beauty-contest-level-k"
title: "Keynesian Beauty Contest and Level-k Reasoning"
reading_id: "10_behavioral_finance"
summary: "The p-guessing (beauty contest) game and the level-k model: starting from a random level-0 anchor, a level-k player best-responds to level k-1, so observed guesses cluster at finitely many iterations rather than at the rational-expectations equilibrium, capturing bounded depth of strategic reasoning."
tags: ["behavioral-finance", "beauty-contest", "level-k", "higher-order-beliefs", "bounded-rationality"]
citations:
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p601:0894"
    chunk_hash: "3b522963e0b133e9776152ec4a0196b0a8fad4e3c5ab06f31cacc9e73155d68e"
    page_range: [601, 601]
    quote: "and tournament payoffs) to the experimental literature."
    edge_type: "defines"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p601:0895"
    chunk_hash: "134179f9e8a94f1a3809fcb5fdf37ee516fccafca6e92353688903aa1352f6cf"
    page_range: [602, 602]
    quote: "2/3 = 33.33, if b = 2/3. A level-2 (L2) player anticipates a level-1"
    edge_type: "supports"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p601:0895"
    chunk_hash: "134179f9e8a94f1a3809fcb5fdf37ee516fccafca6e92353688903aa1352f6cf"
    page_range: [602, 602]
    quote: "It is assumed that all (naive) players in a Beauty Contest game choose randomly, with an average of 50 in the interval [0, 100], for insufficient reasoning."
    edge_type: "supports"
  - source_id: "bf_compecon_v4_2018_ham"
    chunk_id: "bf_compecon_v4_2018_ham:p601:0894"
    chunk_hash: "3b522963e0b133e9776152ec4a0196b0a8fad4e3c5ab06f31cacc9e73155d68e"
    page_range: [601, 601]
    quote: "first-period behavior is far away from the equilibrium, all choosing zero, but behavior slowly converges over time towards zero."
    edge_type: "supports"
card_hash: "c9d7dd9ba18d099387e6e2f2811327a5644e6180cd8b3491fe3a0e3aceb0a3f1"
---
# Keynesian Beauty Contest and Level-k Reasoning

## Intuition

Keynes likened investing to a newspaper beauty contest in which one wins not by picking the prettiest face but by picking the face others will pick — and others are doing the same. The experimental incarnation is the **p-guessing game**: each of `N` players names a number in `[0,100]`, and the winner is whoever is closest to `b` times the group average. With `b = 2/3`, iterated rationality drives every guess to the unique Nash equilibrium of zero. Yet in the lab, first-period choices are far from zero, clustering at intermediate values, and only converge toward zero slowly with repetition. The gap between the equilibrium prediction and observed play is the empirical hook. **Source:** Mauersberger and Nagel (2018) §4.1 pp.583-583.

The **level-k model** explains the gap by positing bounded depth of strategic reasoning. A level-0 player is non-strategic and effectively guesses randomly (mean 50). A level-1 player best-responds to level-0, guessing `50 b`. A level-2 best-responds to level-1, guessing `50 b^2`, and so on. Real subjects do only a few steps of this iteration, so the population is a mixture of low-level types, and aggregate behavior settles at finitely many "spikes" rather than at the infinite-iteration equilibrium. The model fills the modeling gap between purely random behavior and full equilibrium. **Source:** Mauersberger and Nagel (2018) §4.1 pp.584-584.

The framework is foundational for behavioral finance because it formalizes higher-order beliefs and disagreement about others' sophistication — exactly the "what does the crowd think the crowd thinks" reasoning behind speculative pricing. Empirically, depth of reasoning is stable and low (typically 0-3 levels), it varies with the sophistication of the subject pool, and it underlies how quickly markets do or do not converge to equilibrium. **Source:** Mauersberger and Nagel (2018) §4.1 pp.587-588.

## Definition

**Beauty contest (p-guessing) game** is the game where player `i` chooses `y^i` to be close to `b` times the (expected) group average, `y^i = b * E-hat^i[(1/N) sum_j y^j]`, with tournament payoff to the closest guess. **Source:** Mauersberger and Nagel (2018) §4.1 pp.583-583.

**Level-0** is the non-strategic reference type: a naive player who chooses randomly with mean 50 in `[0,100]`, providing the anchor from which iterated best replies start. **Source:** Mauersberger and Nagel (2018) §4.1 pp.584-584.

**Level-k player** is one who best-responds to level k-1, choosing `50 * b^k`; a level-k reasoner believes all others are exactly one level below, so the model requires no consistency of beliefs and is a non-equilibrium model. **Source:** Mauersberger and Nagel (2018) §4.1 pp.584-585.

## Mathematical Reasoning

For a contest multiplier `b` and a uniform level-0 anchor of 50, each best reply multiplies the prior level's guess by `b`, so the level-`k` guess is the geometric term `L_k = 50 * b^k` (with `b = 2/3` this is the case the source states explicitly). The sequence is `L0 = 50`, `L1 = 50 b`, `L2 = 50 b^2`, ..., a geometric progression. When `b < 1` it decays monotonically, and a player who believes everyone iterates infinitely takes the limit `L_k -> 0`, the unique equilibrium. The level-k model thus predicts a geometric sequence of choice spikes; Nagel allows noise by building intervals around these theoretical levels and uses the geometric mean to capture the decreasing level-`k` values. **Source:** Mauersberger and Nagel (2018) §4.1 pp.584-585.

```
   level-k choice spikes, b < 1 (anchor 50)
   density
     |              L2       L1
     |   L_inf      |         |       (Nash rarely chosen)
     |     |        |         |
     |     |        |         |
     +-----0------50b^2-----50b-------50------>  guess
        equilib                     anchor (L0)
     L_k = 50 * b^k  ->  0  as k -> infinity
```

A key conceptual point: level-k is a NON-equilibrium model because a level-k type's beliefs about opponents (concentrated on level k-1) are inconsistent with opponents' actual choices. Generalizations relax the rigid scheme — the cognitive-hierarchy model has types best-respond to a Poisson distribution of all lower levels (estimated parameter ~1.5), and quantal-response variants let higher levels carry higher rationality (skill) parameters `lambda`. Over repeated play, a directional-learning / law-of-effect adjustment combined with level-k explains the slow convergence toward zero, since a player who iterated too few levels relative to the realized target tends to iterate more next period, producing self-fulfilling slow convergence. **Source:** Mauersberger and Nagel (2018) §4.1 pp.585-590.

The parameter `b` governs the strategic environment: `b < 1` is strategic complements with a stable equilibrium converged to from above; `b = 4/3 > 1` reverses the dynamic so behavior converges to the upper bound 100 (the stable equilibrium), illustrating that the same game maps to many core experimental games by varying parameters. **Source:** Mauersberger and Nagel (2018) §4.1 pp.589-589.

## See Also

- [be-experimental-asset-bubbles](./be-experimental-asset-bubbles.md#intuition) — laboratory asset markets where coordination on others' beliefs produces bubbles, a beauty-contest dynamic in price form.
- [be-rationally-heterogeneous-expectations](./be-rationally-heterogeneous-expectations.md#intuition) — learning-to-forecast experiments where heterogeneous depth of reasoning drives coordination on trend rules.
- [be-rational-inattention-entropy](./be-rational-inattention-entropy.md#intuition) — the cognitive-cost view under which a level-k player is (ir)rationally inattentive to higher levels.

## Escalate to Raw When

- The full taxonomy mapping the BC game to other core games (strategic complements/substitutes, matching, global games) by parameter changes is needed. **Source:** Mauersberger and Nagel (2018) §3 pp.573-581.
- Specific estimated level-k distributions, cognitive-hierarchy `tau`, or quantal-response `lambda` values must be quoted. **Source:** Mauersberger and Nagel (2018) §4.1 pp.585-586.
- The level-0 specification debate, elicitation methods (response time, eye-tracking), and macroeconomic applications require the source's detailed treatment. **Source:** Mauersberger and Nagel (2018) §4.1, §5 pp.587-618.
