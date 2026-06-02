---
schema_version: "cacg.v0"
id: "be-cumulative-prospect-theory-risk"
title: "Cumulative Prospect Theory Under Risk"
reading_id: "10_behavioral_finance"
summary: "Cumulative prospect theory under risk combines a reference-dependent value function with rank- AND sign-dependent decision weights, using separate weighting functions w+ on gain-ranks and w- on loss-ranks summed against U(x)."
tags: ["behavioral-finance", "prospect-theory", "rank-dependence", "sign-dependence"]
citations:
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p268:0363"
    chunk_hash: "ca98082d4e24e8d914037f64f3f9f8251740dc6dbb33bbd9c67c3d55db5615fe"
    page_range: [269, 269]
    quote: "the decision weight of an outcome depends not only on its rank, but also on its sign. Hence, Luce & Fishburn (1991) used the term rank- and sign-dependent utility"
    edge_type: "defines"
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p268:0362"
    chunk_hash: "a93a636dd4ff8617a99c7e0fc5e472f882cbd9a444491ac9f823d76f087feef0"
    page_range: [268, 268]
    quote: "Prospect theory (PT), in its updated version as introduced by Tversky & Kahneman (1992), generalizes rank-dependent utility by incorporating reference dependence and loss aversion."
    edge_type: "supports"
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p270:0365"
    chunk_hash: "36217413760d5bf8bfc83dcd4d44ab089c6ba1dfae53492d42e979a3e8c94096"
    page_range: [270, 270]
    quote: "It is often useful to take the PT functional as the sum of an RDU functional applied to gains, and another RDU functional applied to losses"
    edge_type: "supports"
card_hash: "83ab4ed1e744f79f1015e00765145ceded2fae12ca3b3f2fd9c887d716540b84"
---
# Cumulative Prospect Theory Under Risk

## Intuition

Cumulative prospect theory (CPT, Tversky & Kahneman 1992) is the integration of three components of risk attitude: utility curvature, probabilistic sensitivity (rank dependence), and loss aversion. It generalizes rank-dependent utility by incorporating a reference point and allowing the probability weighting to differ for gains and losses. Outcomes are completely *sign-ranked*: ordered best-to-worst and split at the reference point 0 into a positive arm (gains) and a negative arm (losses).
**Source:** Wakker (2010) §9.2 pp.252, §9.1 pp.251.

The key innovation over RDU is that a decision weight now depends not only on an outcome's rank but also on its sign. Gains are weighted by transforming gain-ranks (probability of a better gain) through `w+`; losses are weighted by transforming loss-ranks (probability of a worse loss) through `w-`. This is "rank- and sign-dependent utility." Because the gain and loss arms are each separately RDU functionals, CPT cleanly satisfies monotonicity and stochastic dominance — repairing the central flaw of the original 1979 prospect theory.
**Source:** Wakker (2010) §9.2 pp.253, §9.4 pp.261.

## Definition

**Sign-ranking** is a complete ranking of outcomes `x_1 >= ... >= x_k >= 0 >= x_{k+1} >= ... >= x_n` relative to each other and to the reference point 0.
**Source:** Wakker (2010) §9.2 pp.252.

**Gain-rank** `g` of a positive outcome is the probability of a strictly better gain; weighted via `w+`. **Loss-rank** `l = 1-p-g` of a negative outcome is the probability of a strictly worse loss; weighted via `w-`.
**Source:** Wakker (2010) §9.2 pp.253.

**Decision weight**: for `a > 0`, `pi = w+(p+g) - w+(g)`; for `a < 0`, `pi = w-(p+l) - w-(l)`.
**Source:** Wakker (2010) §9.2 pp.253.

## Mathematical Reasoning

The full CPT functional writes the ranks and the rank dependence of decision weights in full:

```
PT(x) = sum_{i=1}^{k} [ w+(p_i + ... + p_1) - w+(p_{i-1} + ... + p_1) ] U(x_i)     (gains)
      + sum_{j=k+1}^{n} [ w-(p_j + ... + p_n) - w-(p_{j+1} + ... + p_n) ] U(x_j)   (losses)
```

where `U(0)=0`, `U` is strictly increasing, and `w+`, `w-` are two probability weighting functions. The gain arm cumulates ranks downward from the best gain; the loss arm cumulates loss-ranks upward from the worst loss — a reflection that mirrors gain-ranks by loss-ranks about the reference point.
**Source:** Wakker (2010) §9.2 pp.253.

A clean decomposition splits the prospect into its positive part `x+` (negative outcomes replaced by 0) and negative part `x-` (positive outcomes replaced by 0):

```
PT(x) = PT(x+) + PT(x-),
```

each summand an RDU functional — `PT(x+)` with weighting `w+`, and `PT(x-)` with the dual of `w-`. Because both arms are RDU functionals, improving any outcome improves `PT(x)`, so monotonicity and strict stochastic dominance hold (Observation 9.4.1). A common fitted specification uses `w+(p)=p^c/(p^c+(1-p)^c)^{1/c}`, an analogous `w-` with exponent `d`, and `U(a)=a^theta` for gains, `U(a)=-lambda(-a)^{theta'}` for losses, with `c=0.61, d=0.69, theta=theta'=0.88, lambda=2.25`.
**Source:** Wakker (2010) §9.3 pp.255, §9.4 pp.261, §9.3 pp.256.

## See Also

- [be-rank-dependent-utility-via-ranks](./be-rank-dependent-utility-via-ranks.md#intuition) — each sign-arm of CPT is an RDU functional.
- [be-probability-weighting-inverse-s](./be-probability-weighting-inverse-s.md#intuition) — supplies the `w+`, `w-` shapes.
- [be-loss-aversion-reference-dependence](./be-loss-aversion-reference-dependence.md#intuition) — supplies the value function `U` with the `lambda` kink.
- [be-prospect-theory-three-characteristic-pricing](./be-prospect-theory-three-characteristic-pricing.md#intuition) — downstream three-characteristic asset-pricing model.
- [be-prospect-theory-asset-pricing](./be-prospect-theory-asset-pricing.md#intuition) — downstream CPT asset-pricing application.

## Escalate to Raw When

- You need the proof that CPT satisfies monotonicity and stochastic dominance (Observation 9.4.1) and the eleven-step calculation procedure.
**Source:** Wakker (2010) §9.3 pp.255-256, §9.4 pp.261.
- You need the behavioral foundation (sign-tradeoff consistency) that uniquely characterizes the PT functional.
**Source:** Wakker (2010) §9.4 pp.262-263.
