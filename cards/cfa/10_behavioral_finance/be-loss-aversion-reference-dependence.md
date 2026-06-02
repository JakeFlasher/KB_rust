---
schema_version: "cacg.v0"
id: "be-loss-aversion-reference-dependence"
title: "Loss Aversion and Reference Dependence"
reading_id: "10_behavioral_finance"
summary: "Outcomes are coded as gains and losses relative to a reference point; the value function has a kink at the reference point with loss aversion coefficient lambda>1, modeled as U(a)=u(a) for gains and U(a)=lambda*u(a) for losses."
tags: ["behavioral-finance", "loss-aversion", "reference-dependence", "value-function"]
citations:
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p254:0342"
    chunk_hash: "1ef8671f20cfeb0af665349327999cb96318b0a7ba1bea295e7e017967eb2a87"
    page_range: [255, 255]
    quote: "> 1, so that losses are overweighted relative to gains."
    edge_type: "defines"
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p254:0342"
    chunk_hash: "1ef8671f20cfeb0af665349327999cb96318b0a7ba1bea295e7e017967eb2a87"
    page_range: [255, 255]
    quote: "“Losses loom larger than gains” (Kahneman & Tversky, 1979)."
    edge_type: "supports"
  - source_id: "bf_wakker_2010_pt_risk_ambiguity"
    chunk_id: "bf_wakker_2010_pt_risk_ambiguity:p256:0345"
    chunk_hash: "3ea20ccaf834cba64d9a01b66d7ed07057835619bfa320d7ae3ece840fd00968"
    page_range: [257, 257]
    quote: "It is only if reference points vary that the modeling of outcomes as changes with respect to a reference point really deviates"
    edge_type: "supports"
card_hash: "9038cfdf89e926b70a6b440a3dc302529b49dee217f7dbc93979cc3b0e6a2b46"
---
# Loss Aversion and Reference Dependence

## Intuition

Classical expected utility evaluates final wealth positions. The descriptively prevailing alternative is that people process outcomes as *changes* relative to a reference point — usually the status quo or initial wealth — coding outcomes above the reference as gains and below it as losses. As long as the reference point is fixed (initial wealth), this is "nothing but an alternative way of modeling final wealth." The real breakaway from final-wealth models comes only when the reference point *varies* across choice situations.
**Source:** Wakker (2010) §8.3 pp.238, §8.5 pp.240-241.

The central empirical regularity at the reference point is loss aversion: "losses loom larger than gains." The pain of a loss exceeds the pleasure of an equal-sized gain. This produces a kink in the utility function at the reference point — a downward bend where the loss branch is steeper than the gain branch by the loss aversion factor `lambda`. Reference dependence plus loss aversion together drive Rabin's paradox, the endowment effect, the disposition effect, and first-order risk aversion for small stakes.
**Source:** Wakker (2010) §8.4 pp.239, §8.6 pp.243-244.

## Definition

**Reference point** is the point of comparison from which outcomes are coded as gains (above) or losses (below); when fixed it is the *initial wealth* `I`, but it may vary across choice situations.
**Source:** Wakker (2010) §8.3 pp.238, §8.5 pp.240.

**Loss aversion** holds if `lambda > 1`, so that losses are overweighted relative to gains. `lambda < 1` is the opposite, *gain seeking*.
**Source:** Wakker (2010) §8.4 pp.239.

**Basic utility** `u` captures the intrinsic value of outcomes (smooth and differentiable at 0); the **overall utility** `U` applies the loss aversion factor to the loss branch.
**Source:** Wakker (2010) §8.4 pp.239.

## Mathematical Reasoning

The overall utility is built from a regular basic utility `u` and a loss aversion parameter `lambda > 0`:

```
U(a) = u(a)         for a >= 0   (gains)
U(a) = lambda*u(a)  for a < 0    (losses)
```

with scaling convention `u(1)=1`, `u(-1)=-1`, `U(1)=1`, so that `lambda = -U(-1)`. With `u` differentiable at 0, loss aversion (`lambda > 1`) generates nondifferentiability of `U` at zero — the kink is the measure of loss aversion. Mathematically, loss aversion can be read as extreme, "infinite," concavity of `U` exactly at `a = 0`.
**Source:** Wakker (2010) §8.4 pp.239.

Diminishing sensitivity makes `U` concave over gains and convex over losses (the basic utility curves away from the reference point in both directions), so the value function is S-shaped overall while the kink at 0 sits between the two arms:

```
                value U
                  |          gains: concave (diminishing sensitivity)
                  |       .--''''
                  |   .-''
 losses ----------+--*---------------- outcome
        .-'       |  ^ reference point 0, kink: loss branch
     .-'          |    steeper by factor lambda > 1
   .'             |
 (convex)         |
```

A typical fitted specification uses power basic utility `u(a) = a^theta` for gains and `u(a) = -(-a)^theta'` for losses, with Tversky & Kahneman's (1992) estimate `lambda = 2.25`. The source cautions that the power family is not differentiable at 0 and creates problems for estimating `lambda`; loss aversion concerns only *mixed* prospects and does not affect preferences among pure gains or pure losses.
**Source:** Wakker (2010) §9.3 pp.256, §9.3 pp.259.

## See Also

- [be-cumulative-prospect-theory-risk](./be-cumulative-prospect-theory-risk.md#intuition) — combines this value function with rank- and sign-dependent weighting.
- [be-reference-dependent-preferences-foundations](./be-reference-dependent-preferences-foundations.md#intuition) — preference-foundation treatment of reference dependence.
- [be-expectations-based-reference-points](./be-expectations-based-reference-points.md#intuition) — endogenous (expectations-based) theory of the reference point.
- [be-prospect-theory-asset-pricing](./be-prospect-theory-asset-pricing.md#intuition) — downstream asset-pricing use of loss aversion.

## Escalate to Raw When

- You need the analytical problems of power utility for measuring `lambda` (unit-dependence and the `U(a) > -U(-a)` issue) and the remedies.
**Source:** Wakker (2010) §9.6 pp.267-271.
- You need Rabin's paradox in full, showing why ignoring reference dependence forces classical EU into absurd utility curvature.
**Source:** Wakker (2010) §8.6 pp.242-244.
