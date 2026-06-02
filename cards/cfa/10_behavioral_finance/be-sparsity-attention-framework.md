---
schema_version: "cacg.v0"
id: "be-sparsity-attention-framework"
title: "Sparsity-Based Attention Framework"
reading_id: "10_behavioral_finance"
summary: "Gabaix sparse-max operator: an attention vector m in [0,1] dampens perceived variables toward a default, chosen by trading attention gains against a sparsity-inducing cost kappa*sum m_i^alpha; alpha=1 yields both sparsity and a continuous reaction."
tags: ["behavioral-finance", "inattention", "bounded-rationality", "sparse-max"]
citations:
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p276:0455"
    chunk_hash: "98778302129b653016e46216696a295d33be691f84f9ace7fb319d397f6287b5"
    page_range: [276, 276]
    quote: "people anchor on a simple perception of the world, and partially adjusts toward it"
    edge_type: "supports"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p299:0491"
    chunk_hash: "d9f6aa733fdc327c558ad86b32855b24a4c334cfed9209593571985cb74305cd"
    page_range: [300, 300]
    quote: "attention creates a psychic cost"
    edge_type: "defines"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p302:0495"
    chunk_hash: "13fa2982af7eb841670837ff9ff513d2aea82447b723454994e105990e43f57d"
    page_range: [302, 302]
    quote: "Eliminate each feature of the world that would change the action by only a small amount"
    edge_type: "supports"
card_hash: "986a87cdf8726f25d1f607412d1c73b2d83f3aa18f6af7b2936b5373c51c64f9"
---
# Sparsity-Based Attention Framework

## Intuition
Our attention is limited: choosing a bottle of wine, we weigh price and quality but not the myriad minor considerations (future income, interest rates, learning value) that classical rationality says we should process. Gabaix's sparsity framework formalizes this by giving the agent an attention vector `m`, where `m_i = 1` is full attention to dimension `i` and `m_i = 0` is complete inattention — the agent then perceives a convex combination of the true value and a simple "default" perception of the world.
**Source:** Gabaix (2019) Ch.4 §2.3 p.276.

The power of the framework is that one parameter `m` per dimension unifies a wide range of behavioral biases (inattention to taxes and prices, base-rate neglect, over/underreaction, projection bias, even hyperbolic discounting as global inattention to the future) at the formal level. The "sparse max" operator makes `m` endogenous: the agent pays attention only to the few features important enough to move her decision, shrinking the rest toward the default. This sidesteps the "infinite regress" problem of fully optimizing over how much to think.
**Source:** Gabaix (2019) Ch.4 §2.3, §4.1 pp.276, 299-300.

## Definition
**Attention parameter `m`** parameterizes the convex combination of the default and true models: the subjectively perceived value is `m·(true) + (1−m)·(default)`, so `m = 0` means the agent relies entirely on the crude default and `m = 1` recovers the rational model.
**Source:** Gabaix (2019) Ch.4 §2.3 p.276.

**Sparse max operator** `smax_a u(a,x)` subject to `b(a,x) ≥ 0` is a less-than-fully-attentive version of the `max` operator: the agent optimizes in two steps — choosing an attention vector `m*` under a simplified problem, then choosing the action under the exact utility modulated by `m*`.
**Source:** Gabaix (2019) Ch.4 §4.1 p.300.

**Sparsity-inducing cost** `C(m) = κ·sum_i m_i^α` with `α ≥ 0`: `κ ≥ 0` is the penalty for lack of sparsity, and `κ = 0` recovers the costless-cognition rational agent.
**Source:** Gabaix (2019) Ch.4 §4.1 p.300.

## Mathematical Reasoning
The behavioral agent, after choosing `m`, optimizes the simplified perception `a^s(x,m) = sum_i b_i·m_i·x_i`, so `m_i = 0` means dimension `i` is ignored. Attention costs `C(m) = κ·sum_i m_i^α`. In Step 1 the agent solves a linear-quadratic approximation, treating `x` as drawn from a mean-zero distribution with the accurate variances; in Step 2 she acts under the exact utility modulated by `m*`. For a single variable with `σ^2 = Λ_11`, the problem reduces to `min_m (1/2)(1 − m)^2·σ^2 + κ·m^α`, solved by an attention function `m = A_α(σ^2/κ)`.
**Source:** Gabaix (2019) Ch.4 §4.1 pp.300-302.

The general solution (Proposition 4.1) sets `m_i* = A_α( σ_i^2·|a_{x_i}·u_{aa}·a_{x_i}| / κ )`, so more attention is paid to a variable when it is more variable (high `σ_i^2`), when it matters more for the action (high `|a_{x_i}|`), when mistakes are costlier (high `|u_{aa}|`), and when cognition is cheap (low `κ`). Lemma 4.1 establishes that `α ≤ 1` (and only then) makes the attention function *sparse* — assigning exactly `m = 0` to unimportant variables — while `α ≥ 1` makes it *continuous*; only `α = 1` delivers both sparsity and continuity, so `α = 1` is recommended. The procedure amounts to the rule: "Eliminate each feature of the world that would change the action by only a small amount."
**Source:** Gabaix (2019) Ch.4 §4.1 p.302.

```
   A_0 (fixed cost):    step -> sparsity, NOT continuous
   A_1 (linear cost):   max(1 - 1/sigma^2, 0) -> sparse AND continuous
   A_2 (quadratic cost):sigma^2/(2+sigma^2)   -> continuous, NOT sparse
```
**Source:** Gabaix (2019) Ch.4 §4.1 pp.301-302.

## See Also
- [be-rational-inattention-entropy](./be-rational-inattention-entropy.md#intuition) — sibling framework using Shannon-entropy capacity cost (uniform dampening) rather than a sparsity penalty.
- [be-asset-pricing-anomalies-catalog](./be-asset-pricing-anomalies-catalog.md#intuition) — PEAD / inattention payoffs in finance (Barberis, Vol.1).
- [be-present-focused-preferences-taxonomy](./be-present-focused-preferences-taxonomy.md#intuition) — hyperbolic discounting recast as global inattention to the future.

## Escalate to Raw When
- You need the constrained sparse-max (budget constraint, perceived prices) or the behavioral consumer-theory / Slutsky-asymmetry results. **Source:** Gabaix (2019) Ch.4 §4.1.2, §5 pp.303-304.
- You need the source-specific dampening contrast with the Sims uniform-dampening solution and when framing of attributes matters. **Source:** Gabaix (2019) Ch.4 §6.2.2 p.324.
