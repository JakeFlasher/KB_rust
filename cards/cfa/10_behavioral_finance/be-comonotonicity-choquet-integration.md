---
schema_version: "cacg.v0"
id: "be-comonotonicity-choquet-integration"
title: "Comonotonicity and Choquet Integration"
reading_id: "10_behavioral_finance"
summary: "Comonotonic prospects share one complete ranking of events so each event keeps a fixed rank; on a comonotonic set the rank-dependent (Choquet) integral over a nonadditive capacity W coincides with an EU functional, and comonotonic tradeoff consistency is the behavioral axiom behind rank dependence."
tags: ["behavioral-finance", "comonotonicity", "choquet-integral", "tradeoff-consistency"]
citations:
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p326:0446"
    chunk_hash: "61f4d303a75210e260b4b3707eb1c2bacf1da9906d10c1370c8db3e8eaf5e58f"
    page_range: [326, 326]
    quote: "A set of prospects is comonotonic if the same complete ranking of events (further discussed below) can be used for all prospects."
    edge_type: "defines"
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p325:0444"
    chunk_hash: "9a0f516ec58af7bc107bc26e2630b48b8f96c405797a781d24522d7fcc450968"
    page_range: [325, 325]
    quote: "Similarly to the integral definition of RDU in Eq. (6.9.1) for risk, RDU for uncertainty can be calculated as"
    edge_type: "supports"
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p329:0452"
    chunk_hash: "6ad59d852523544bb96c7dc5165c618361276f74a04420b8766cef5c1b4bc239"
    page_range: [330, 330]
    quote: "rank-tradeoff consistency can be weakened to comonotonic tradeoff consistency."
    edge_type: "supports"
card_hash: "7d6c6e9246ad1bda18f4b8904f55c6fadccdf419a8ede08f0b809615baeef422"
---
# Comonotonicity and Choquet Integration

## Intuition

Under uncertainty (unknown probabilities), rank-dependent utility weights events through a nonadditive set function `W` — a capacity assigning weight to "good-news" events rather than additive probabilities. The mathematical object that organizes this is the *Choquet integral*: the integral of utility with respect to a nonadditive measure, which Wakker presents in ranked form to keep it elementary. The subtlety is that the decision weight of an event depends on the event's *rank* — which other outcomes are better — so the same event can carry different weights in different prospects.
**Source:** Wakker (2010) App.10.10 pp.309, App.10.12 pp.310.

Comonotonicity is the condition that tames this rank-dependence. Two prospects are comonotonic if a single complete ranking of the events works for both, so every event has the same rank across the prospects in the set. On a comonotonic set, ranks are fixed, the nonadditive `W` behaves like an additive probability, and the Choquet/RDU integral reduces to an ordinary expected-utility functional. This is why behavioral conditions are imposed only *within* comonotonic sets (comoncones): outside them, mixing prospects could change ranks and break the additivity.
**Source:** Wakker (2010) App.10.12 pp.310-312.

## Definition

**Comonotonic set of prospects** is one for which the same complete ranking of events can be used for all prospects; equivalently, no two prospects `x`, `y` and states `s`, `t` exist with `x(s) > x(t)` but `y(s) < y(t)`. Each event then has the same rank for all prospects in the set.
**Source:** Wakker (2010) App.10.12 pp.310-311.

**Capacity (weighting function) `W`** is a nonadditive set function on events with `W(empty)=0`, `W(S)=1`, monotone in set inclusion, giving the decision weight of the good-news event.
**Source:** Wakker (2010) §10.2 pp.282, App.10.10 pp.309; normalization `W(empty)=0`, `W(S)=1` stated at §10.3 Example 10.3.1 pp.286.

**Comonotonic tradeoff consistency** is the behavioral axiom restricting tradeoff/indifference consistency to the case where all four prospects compared are comonotonic; it is weaker than (unrestricted) rank-tradeoff consistency.
**Source:** Wakker (2010) App.10.12 pp.314.

## Mathematical Reasoning

The Choquet/RDU integral for uncertainty is the integral of utility with respect to the capacity `W`, written over the survival sets of the utility:

```
RDU(x) = integral_{R+}  W{ s in S : U(x(s)) > t } dt
       - integral_{R-} [ 1 - W{ s in S : U(x(s)) > t } ] dt.
```

For a completely ranked event-contingent prospect `E_1 x_1 ... E_n x_n` with `x_1 >= ... >= x_n`, this reduces to the ranked sum with decision weights `pi(E_j^{rank}) = W(E_1 ∪ ... ∪ E_j) - W(E_1 ∪ ... ∪ E_{j-1})`, the marginal `W`-contribution of `E_j` to its cumulative rank.
**Source:** Wakker (2010) App.10.10 pp.309, §10.2 pp.282.

On a comonotonic set one can construct a probability measure `P` from `W` via `P{t : t rho-better than s} = W{t : t rho-better than s}`, and then EU under `P` agrees with RDU on that comonotonic set: each outcome event keeps its rank and so its decision weight equals `P(E_j)`. Hence within comoncones the sure-thing principle holds and the additive aggregation of classical EU is recovered.
**Source:** Wakker (2010) App.10.12 pp.312-313.

```
comonotonic set: one fixed ranking of events
   E_(1) > E_(2) > ... > E_(n)   (same for every x in the set)
        |        |              |
   rank fixed -> W behaves additively -> Choquet integral = EU functional
   (sure-thing principle holds within the comoncone)
```

Wakker notes comonotonic preference conditions yield stronger theorems on behavioral foundations, but he deliberately uses the more tractable rank conditions throughout the book; the comonotonic generalization (comonotonic tradeoff consistency) is the weaker condition that still suffices, since the proof of the RDU foundation (Theorem 10.5.6) never used more than the comonotonic condition.
**Source:** Wakker (2010) App.10.12 pp.313-314.

## See Also

- [be-rank-dependent-utility-via-ranks](./be-rank-dependent-utility-via-ranks.md#intuition) — the rank-based presentation that comonotonicity underlies.
- [be-cumulative-prospect-theory-risk](./be-cumulative-prospect-theory-risk.md#intuition) — sign- and rank-dependent extension built on the same machinery.
- [be-multiple-priors-maxmin-eu](./be-multiple-priors-maxmin-eu.md#intuition) — Choquet EU connects to the CORE and multiple-priors benchmark.
- [be-ambiguity-sources-ellsberg](./be-ambiguity-sources-ellsberg.md#intuition) — nonadditive `W` accommodates the Ellsberg paradox.

## Escalate to Raw When

- You need the formal comonotonicity proofs (comoncones, Szpilrajn extension, maximal comonotonic sets) and Exercise 10.12.2-3.
**Source:** Wakker (2010) App.10.12 pp.311-312.
- You need the full behavioral foundation of RDU for uncertainty via rank-tradeoff consistency (Theorem 10.5.6).
**Source:** Wakker (2010) §10.5 pp.297-298, App.10.12 pp.314.
