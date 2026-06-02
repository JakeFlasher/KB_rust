---
schema_version: "cacg.v0"
id: "be-commitment-and-naivete"
title: "Commitment and Naivete"
reading_id: "10_behavioral_finance"
summary: "Sophisticated agents foresee their own future present bias and demand commitment devices; naive agents (beta-hat ~ 1) do not; partial naivete and welfare consequences in credit and gym contracts."
tags: ["behavioral-finance", "present-bias", "commitment", "sophistication-naivete"]
citations:
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p018:0022"
    chunk_hash: "6b9a8d2f4f1be3b4a5cd030b643565bc9cb6ef972ed575b6ee88ba69e421eeff"
    page_range: [18, 18]
    quote: "Sophisticated beliefs imply that the agent has a correct theory of her own future time preferences."
    edge_type: "defines"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p041:0062"
    chunk_hash: "a9818907e09fb24a105b2a8c2e424fbd9c6c515cd16895e255c1d07d0e5abe02"
    page_range: [42, 42]
    quote: "People partially but do not fully anticipate their extent of future present-focus."
    edge_type: "supports"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p043:0064"
    chunk_hash: "78c0529c516b6cbdfb8347126c7ee01082ecd69a6a7f6e9af015162dd63408c4"
    page_range: [43, 43]
    quote: "firms design credit contracts with large penalties for deferring repayment and"
    edge_type: "supports"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p047:0071"
    chunk_hash: "f115122fe44adbefd78b47af50f5d5d99791df92ffb2960ed77f4261c2c489e8"
    page_range: [47, 47]
    quote: "People sometimes demand commitment devices."
    edge_type: "supports"
card_hash: "08d5105e1be5497a26b5008821776e216331650e4ccd8a8146d669ef55cdbd5a"
---
# Commitment and Naivete

## Intuition
Whether a present-biased agent *knows* she is present-biased changes everything. A sophisticated agent has a correct theory of her future selves and so foresees that, left unconstrained, she will succumb to immediate gratification; this foresight creates a demand to tie her own hands in advance. A naive agent believes her future selves will behave like time-consistent exponential discounters, so she sees no need for commitment and is repeatedly surprised. Between these poles lies partial naivete — recognizing some future present-focus but underestimating its extent — which the empirical evidence broadly supports.
**Source:** Ericson & Laibson (2019) Ch.1 §2.1, §3.5 pp.17-18, 42.

The sophistication/naivete distinction matters for the contracts agents select in equilibrium and for welfare. Firms design products that exploit naivete: backloaded gym fees, credit contracts with steep deferral penalties. Even a small amount of naivete can leave an agent discontinuously worse off than a sophisticate, because she predictably walks into penalties she failed to anticipate.
**Source:** Ericson & Laibson (2019) Ch.1 §3.5, §3.8 pp.43, 47.

## Definition
**Sophisticated beliefs** imply the agent has a correct theory of her own future time preferences (correct perception of her future `β`). **Fully naive beliefs** imply she believes her future selves are not present-biased at all, i.e. `β̂ = 1`, so she expects no future preference for commitment.
**Source:** Ericson & Laibson (2019) Ch.1 §2.1 pp.17-18.

**Partial naivete** is the belief that future selves have some present bias but less than they actually do: `β(t') < E_t[β(t')] < 1`, where `β̂` is shorthand for the *perceived* future `β`.
**Source:** Ericson & Laibson (2019) Ch.1 §2.1 pp.17-18.

**Pure commitment** is a preference to restrict one's own choice set holding all else equal — a strictly preferred restriction, distinct from "impure" commitments tied to financial inducements like 401(k) matching.
**Source:** Ericson & Laibson (2019) Ch.1 §3.8 p.47.

## Mathematical Reasoning
The perceived present-bias parameter is denoted `β̂`, a shorthand for `E_t[β(t')]`. Sophistication is `β̂ = β`; full naivete is `β̂ = 1`; partial naivete is `β < β̂ < 1`. The structural estimates make the gap concrete: Augenblick and Rabin estimate a real-effort-task `β ≈ 0.8` but `β̂` near 1 — they cannot reject no perceived present bias (consistent with full naivete) but can reject accurate perception of present bias, pinning down substantial naivete.
**Source:** Ericson & Laibson (2019) Ch.1 §2.1, §3.5 pp.17-18, 43.

Empirical regularity 8 states that *people sometimes demand commitment devices*. For a present-biased agent, an irreversible restriction prevents a suboptimal future choice; for a temptation agent, commitment removes temptation costs even when the tempting good is not chosen. Yet *pure* commitment (uncorrupted by tied financial inducements) is rarely observed in organic markets and willingness to pay for it is typically near zero in lab experiments — Schilbach's rickshaw-driver study being a notable exception, where about a third of subjects forgo ~10% of daily income for a sobriety commitment. The welfare wedge from naivete arises because, as Heidhues and Koszegi show, firms design credit contracts with large deferral penalties that naive borrowers select precisely because they underestimate the probability of paying them.
**Source:** Ericson & Laibson (2019) Ch.1 §3.5, §3.8 pp.43, 47-49.

## See Also
- [be-quasi-hyperbolic-discounting](./be-quasi-hyperbolic-discounting.md#mathematical-reasoning) — the β-δ model whose β the agent must perceive (β̂).
- [be-present-focused-preferences-taxonomy](./be-present-focused-preferences-taxonomy.md#mathematical-reasoning) — taste for commitment as one classification axis.
- [be-household-liquidity-illiquidity-puzzle](./be-household-liquidity-illiquidity-puzzle.md#intuition) — illiquid wealth as an implicit commitment strategy.

## Escalate to Raw When
- You need the full list of lab/field commitment-demand studies (Ashraf, Bryan, Gine, Kaur, Beshears) and the open question on why willingness to pay is near zero. **Source:** Ericson & Laibson (2019) Ch.1 §3.8 p.49.
- You need the detailed evidence distinguishing naivete from overconfidence-about-memory or projection bias. **Source:** Ericson & Laibson (2019) Ch.1 §3.5 pp.42-43.
