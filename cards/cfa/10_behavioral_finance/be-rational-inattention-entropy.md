---
schema_version: "cacg.v0"
id: "be-rational-inattention-entropy"
title: "Rational Inattention (Entropy-Based)"
reading_id: "10_behavioral_finance"
summary: "Sims rational inattention: agents freely choose a signal structure subject to a Shannon mutual-information capacity constraint I(A,X) <= K; the entropy penalty yields a tractable Gaussian solution m = 1 - e^{-2K} with uniform dampening across dimensions."
tags: ["behavioral-finance", "inattention", "information-theory", "entropy"]
citations:
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p320:0522"
    chunk_hash: "05279ffb8457428e225bf40ac62ad8e18a032a6a65bcfd04e8041d6418b5db30"
    page_range: [320, 320]
    quote: "Sims (1998, 2003) extends the ideas to allow for larger choice sets in which agents freely choose the properties of their signals. He uses the entropy penalty"
    edge_type: "defines"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p320:0522"
    chunk_hash: "05279ffb8457428e225bf40ac62ad8e18a032a6a65bcfd04e8041d6418b5db30"
    page_range: [320, 320]
    quote: "the entropy of X for the discrete case is defined as"
    edge_type: "supports"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p323:0526"
    chunk_hash: "84448ce2ab2a953264f5f327cc057d1e375340f2bb8b195afd1fed99b26b3c63"
    page_range: [323, 323]
    quote: "the decision problem (70) boils down to"
    edge_type: "supports"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p324:0527"
    chunk_hash: "905a9627bba3fdd62b64b440031068c89dd5be05081a81774155fe5bb886653f"
    page_range: [324, 324]
    quote: "with the global entropy constraint of Sims we obtain uniform dampening across all variables"
    edge_type: "supports"
card_hash: "a5698920bb97eaae2824ca8be867dce417e7656c71b969ace524b96e73240ba4"
---
# Rational Inattention (Entropy-Based)

## Intuition
Rational inattention takes a different route to bounded attention: rather than positing a default and a dampening parameter, Sims lets the agent *freely choose the properties of her signals* subject to a single information-processing budget. The cost of attention is measured in the currency of information theory — Shannon entropy and mutual information — so the agent behaves like a capacity-constrained communication channel deciding which features of the world to transmit to her own decision.
**Source:** Gabaix (2019) Ch.4 §6.2 p.320.

The appeal is a *universally applicable* measure of the cost of information that keeps the agent on the optimal-processing frontier. In simple linear-quadratic-Gaussian settings the entropy model and the sparsity model look similar; the crucial difference is that Sims generates *uniform* dampening across all variables, whereas sparsity allows source-specific attention. The cost is tractability: away from the Gaussian case, entropy problems become computationally hard (solutions can be non-smooth, with atoms).
**Source:** Gabaix (2019) Ch.4 §6.2.2 p.324.

## Definition
**Entropy** of a discrete random variable `X` with mass `p_i` is `H(X) = −E[log f(X)] = −sum_i p_i·log p_i`, a measure of uncertainty (maximized by the uniform distribution, giving `H(X) = log n`).
**Source:** Gabaix (2019) Ch.4 §6.2.1 pp.320-321.

**Mutual information** `I(X,Y) = H(X) − H(X|Y)` is the reduction in entropy of `X` from learning `Y`; it is symmetric and equals the Kullback-Leibler divergence between the joint density and the product of marginals.
**Source:** Gabaix (2019) Ch.4 §6.2.1 pp.321-322.

**Rational inattention** is the model in which the agent chooses a stochastic action density `q(a|x)` to maximize expected utility subject to a capacity constraint on the mutual information between her action and the state: `I(A,X) ≤ K`.
**Source:** Gabaix (2019) Ch.4 §6.2.2 p.323.

## Mathematical Reasoning
Sims' problem: `max over q(a|x) of integral u(a,x)·q(a|x)·f(x) da dx` subject to `I(A,X) ≤ K`. The agent instructs a "black box" to return a noisy action prescription whose informativeness is capped at `K` bits. In the targeting example with `x ~ N(0, σ^2)` and quadratic loss `u = −(1/2)(a − x)^2`, the optimal action is `a(s) = E[x|s] = m·x + m·ε` with `m = σ^2/(σ^2 + σ_ε^2)`. For jointly Gaussian variables with correlation `ρ`, mutual information is `I = (1/2)·log[1/(1 − ρ^2)]`, and since `ρ^2 = m`, the constraint becomes `(1/2)·log[1/(1 − m)] ≤ K`.
**Source:** Gabaix (2019) Ch.4 §6.2.1-6.2.2 pp.322-323.

The decision problem `max_m −(1/2)(1 − m)·σ^2` subject to `(1/2)·log[1/(1 − m)] ≤ K` then "boils down to" the closed-form attention `m = 1 − e^{−2K}`, with action `a^Sims = m·x + η`. Equivalently this is the basic sparsity problem of Section 4.1 with a cost function `C(m) = (1/2)·log[1/(1 − m)]`. To calibrate even `m ≤ 0.9` requires capacity `K ≤ 1.15` natural units (about 1.7 bits) — a very small capacity. In the multidimensional uncorrelated-Gaussian case the solution is `a^Sims = m·sum_i b_i·x_i + η`, i.e. with the *global* entropy constraint one obtains **uniform dampening across all variables** (single `m`), in contrast to sparsity's source-specific `a^s = sum_i m_i·b_i·x_i` where `m_1 > m_10` is possible.
**Source:** Gabaix (2019) Ch.4 §6.2.2 pp.323-324.

## See Also
- [be-sparsity-attention-framework](./be-sparsity-attention-framework.md#intuition) — sibling framework with a sparsity penalty and source-specific (non-uniform) dampening.
- [be-asset-pricing-anomalies-catalog](./be-asset-pricing-anomalies-catalog.md#intuition) — inattention payoffs in finance (Barberis, Vol.1).
- [be-present-focused-preferences-taxonomy](./be-present-focused-preferences-taxonomy.md#intuition) — myopia/inattention to the future as a related theme.

## Escalate to Raw When
- You need the information-theory crash course in full (additivity of independent entropies, KL divergence, Gaussian entropy formulas). **Source:** Gabaix (2019) Ch.4 §6.2.1 pp.321-322.
- You need the random-choice-via-entropy-penalty (logit) connection or the dynamic slow-information-accumulation results. **Source:** Gabaix (2019) Ch.4 §6.3, §7.3 pp.325-326.
