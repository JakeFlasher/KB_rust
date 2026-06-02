---
schema_version: "cacg.v0"
id: "be-rank-dependent-utility-via-ranks"
title: "Rank-Dependent Utility via Ranks"
reading_id: "10_behavioral_finance"
summary: "Rank-dependent utility evaluates a prospect by ranking outcomes best-to-worst and weighting each outcome's utility by the marginal w-contribution of its probability to the cumulative rank, w(p+r)-w(r)."
tags: ["behavioral-finance", "rank-dependent-utility", "decision-weights", "probability-weighting"]
citations:
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p181:0240"
    chunk_hash: "3c12c6c81b58bba61de2a3249f7a2f4877c044120eb33db3faab99b81ea40ff7"
    page_range: [182, 182]
    quote: "Note that w(p þ r) is the rank of the outcome in the prospect next-worse to a."
    edge_type: "defines"
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p176:0234"
    chunk_hash: "4edee00fa87a06e95ddbae5a9dd89c1e0abeca95b3624d1295eff796b9b3e3d9"
    page_range: [176, 176]
    quote: "Under RDU, the weight of a utility is the difference between two transformed ranks"
    edge_type: "supports"
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p172:0229"
    chunk_hash: "659e2c8873fc0196470b33b1bacf29eb56874015bb5fa42e4a1545bb9af90e2f"
    page_range: [172, 172]
    quote: "describes the probability of y yielding an outcome ranked strictly better than"
    edge_type: "supports"
card_hash: "a0d124a5552d5867cd4eb4d5139a53b396dc1247ac6722d839385f515f937f93"
---
# Rank-Dependent Utility via Ranks

## Intuition

Expected utility weights each outcome by its raw probability. Rank-dependent utility (RDU) instead transforms not the separate-outcome probabilities but the *ranks* — the cumulative probability of receiving a better outcome. The pedagogical move that organizes Wakker's whole presentation is to order the outcomes of a prospect from best to worst and ask, for each outcome, "what is the probability of doing strictly better than this?" That good-news probability is the rank, and the decision weight is the marginal contribution of the outcome's own probability to that rank, measured through a weighting function `w`.
**Source:** Wakker (2010) §5.4 pp.156, §6.1 pp.170.

This is why RDU is psychologically natural and avoids the stochastic-dominance violations that plagued the original 1979 prospect theory: increasing a rank is unambiguously favorable, whereas increasing a separate-outcome probability could help or hurt depending on which outcome it shifts mass toward. Transforming ranks (good-news probabilities) rather than fixed-outcome probabilities is the correct generalization of expected utility into the probability dimension.
**Source:** Wakker (2010) §5.4 pp.156.

## Definition

**Rank** (good-news probability) of an outcome `x_i` is `r_i = p_{i-1} + ... + p_1`, the probability of receiving a strictly better-ranked outcome, where outcomes are completely ranked `x_1 >= ... >= x_n`. Rank 0 is the *best rank* (`p^b`); rank `1-p` is the *worst rank* (`p^w`).
**Source:** Wakker (2010) §5.4 pp.156, §6.1 pp.171.

**Ranked probability** `p^r` is the pair `(p, r)` of an outcome probability `p` and its rank `r`, with `p >= 0`, `r >= 0`, `p + r <= 1`.
**Source:** Wakker (2010) §6.1 pp.170.

**Decision weight** of a ranked probability is `pi(p^r) = w(p+r) - w(r)`, the marginal w-contribution of the outcome probability to its rank.
**Source:** Wakker (2010) §6.1 pp.170.

## Mathematical Reasoning

For a prospect `p_1 x_1 ... p_n x_n` with complete ranking `x_1 >= ... >= x_n`, RDU is

```
RDU = sum_{j=1}^{n} pi_j U(x_j)
    = sum_{j=1}^{n} [ w(p_j + ... + p_1) - w(p_{j-1} + ... + p_1) ] U(x_j).
```

Each utility `U(x_j)` is weighted by the difference of `w` applied to two cumulative ranked probabilities: the rank including `x_j` (probability of `x_j` or anything better) minus the rank excluding it (probability of anything strictly better). The first transformed rank is the probability of receiving the outcome or any better outcome; the second is the probability of any better outcome.
**Source:** Wakker (2010) §6.1 pp.171, §5.4 pp.160.

Expected utility is the special case where `w` is the identity, so the differences telescope back to the raw `p_j` and `RDU = sum p_j U(x_j)`. The terms in the cumulative-rank format make this transparent:

```
EU:  sum_{j=1}^{n} [ (p_j + ... + p_1) - (p_{j-1} + ... + p_1) ] U(x_j).
```

Replacing the linear cumulative argument by `w(.)` yields RDU. The slope of `w` around an outcome's rank — not the absolute level of `w` — drives the decision weight, so a convex `w` enlarges weights on bad ranks (pessimism) and a concave `w` enlarges weights on good ranks (optimism).
**Source:** Wakker (2010) §5.4 pp.160, §6.3 pp.173-175.

## See Also

- [be-comonotonicity-choquet-integration](./be-comonotonicity-choquet-integration.md#intuition) — comonotonic tradeoff consistency is the behavioral axiom underlying rank dependence.
- [be-probability-weighting-inverse-s](./be-probability-weighting-inverse-s.md#intuition) — the shape of `w` that the ranks are fed into.
- [be-cumulative-prospect-theory-risk](./be-cumulative-prospect-theory-risk.md#intuition) — adds sign-dependence (separate `w+`, `w-`) on top of rank dependence.
- [be-prospect-theory-three-characteristic-pricing](./be-prospect-theory-three-characteristic-pricing.md#intuition) — downstream asset-pricing application of probability weighting.

## Escalate to Raw When

- You need the formal behavioral foundation (rank-tradeoff consistency) that uniquely characterizes RDU and its uniqueness results — see §6.5.2 pp.184 and Theorem 6.5.6.
**Source:** Wakker (2010) §6.5 pp.184.
- You need the continuous-distribution integral representation of RDU rather than the finite ranked sum.
**Source:** Wakker (2010) §6.9 pp.199.
