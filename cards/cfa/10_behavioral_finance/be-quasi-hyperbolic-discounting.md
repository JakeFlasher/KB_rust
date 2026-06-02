---
schema_version: "cacg.v0"
id: "be-quasi-hyperbolic-discounting"
title: "Quasi-Hyperbolic (Beta-Delta) Discounting"
reading_id: "10_behavioral_finance"
summary: "The beta-delta quasi-hyperbolic model U_t = u_t + beta*sum delta^k u_{t+k}: a single present-bias parameter beta<1 multiplies the exponentially delta-discounted continuation stream, generating dynamic inconsistency."
tags: ["behavioral-finance", "present-bias", "discounting", "time-inconsistency"]
citations:
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p016:0019"
    chunk_hash: "691ca6ae9d99fc0793a4786244669c4953f706833b74b303985d1c493b5fec29"
    page_range: [17, 17]
    quote: "(1) Here Ut is total utility, ut is flow utility in period t, β is the present bias parameter, and δ is the long-run discount factor."
    edge_type: "defines"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p017:0021"
    chunk_hash: "19feb17bc57429780e8916b9aaba7e6a78606ebf577ee61f102edc4da46f45ef"
    page_range: [18, 18]
    quote: "= 1 these preferences revert to exponential discounting."
    edge_type: "supports"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p018:0022"
    chunk_hash: "6b9a8d2f4f1be3b4a5cd030b643565bc9cb6ef972ed575b6ee88ba69e421eeff"
    page_range: [18, 18]
    quote: "Present-biased preferences are dynamically inconsistent"
    edge_type: "supports"
card_hash: "9800d530550c5f1715f0132345e39585f139c1c6b8f99d1e6494744cdf9bc632"
---
# Quasi-Hyperbolic (Beta-Delta) Discounting

## Intuition
Quasi-hyperbolic discounting is the workhorse model of present bias and the most commonly used intertemporal-choice model in behavioral economics. Its appeal is parsimony: a single extra parameter `β` captures the universal pull toward the immediate, while the familiar exponential factor `δ` governs all longer-run tradeoffs. The "quasi-hyperbolic" name signals its intellectual debt to the older hyperbolic-discounting literature, but the functional form is far simpler — there is one discontinuous drop between now and the next period, then constant exponential decay thereafter.
**Source:** Ericson & Laibson (2019) Ch.1 §2.1 pp.15-16.

The object being discounted is a stream of *utility flows* (pleasures and pains experienced at points in time), not financial flows. The model says nothing about how an agent treats a fully anticipated paycheck that arrives Friday vs Monday when liquidity is ample; it concerns when pleasurable consumption is *experienced*. This distinction, first articulated by Ramsey, separates the discount rate for utility from the market interest rate for money.
**Source:** Ericson & Laibson (2019) Ch.1 §2.1 p.17.

## Definition
**Present bias (quasi-hyperbolic discounting)** is the preference structure in which the entire continuation utility stream from the next period onward is multiplied by a single factor `β < 1` before exponential discounting is applied, so the present period gets weight 1 and all future periods are jointly downweighted.
**Source:** Ericson & Laibson (2019) Ch.1 §2.1 pp.15-16.

**β (present-bias parameter)** is the weighting parameter on all future utility flows; **δ (long-run discount factor)** is the per-period exponential factor. Note `β` is the "discount factor" jargon for the weight, while the horizon-dependent **discount function** `D(t)` is the function of weights.
**Source:** Ericson & Laibson (2019) Ch.1 §2.1 pp.17-18.

## Mathematical Reasoning
Present-biased preferences are written `U_t = u_t + β·δ·u_{t+1} + β·δ^2·u_{t+2} + β·δ^3·u_{t+3} + ...`, where `u_t` is flow utility in period `t`. Note `β` is *not* exponentiated whereas `δ` is. Factoring out `β` from the continuation makes the structure transparent: `U_t = u_t + β·[ δ·u_{t+1} + δ^2·u_{t+2} + ... ]`, i.e. the continuation payoff stream is weighted by `β` and then exponentially discounted thereafter.
**Source:** Ericson & Laibson (2019) Ch.1 §2.1 pp.17-18.

The discount function is `D(t) = 1` if `t = 0` and `D(t) = β·δ^t` if `t ≥ 1`, with bounds `0 < β ≤ 1` and `0 < δ ≤ 1`. When `β = 1` the preferences revert to exponential discounting, so behavioral models nest the classical model as a special case. Dynamic inconsistency follows directly: with `δ = 1` and a cost `c` now vs benefit `b` later, if `b > c > β·b` then `−c + β·b < 0` but `β·(−c + b) > 0`, so the agent at `t` prefers to exercise at `t' > t` yet at `t'` prefers to nap. The local rate of decline of the discount function, `−[D(t+1) − D(t)] / D(t)`, is the short-run rate `1 − β·δ` and the long-run rate `1 − δ`, with `1 − δ < 1 − β·δ` — a monotonically falling discount rate.
**Source:** Ericson & Laibson (2019) Ch.1 §2.1 pp.17-19.

## See Also
- [be-present-focused-preferences-taxonomy](./be-present-focused-preferences-taxonomy.md#intuition) — the meta-category in which present bias is one cell.
- [be-commitment-and-naivete](./be-commitment-and-naivete.md#intuition) — sophistication/naivete about β and the resulting demand for commitment.
- [be-household-liquidity-illiquidity-puzzle](./be-household-liquidity-illiquidity-puzzle.md#intuition) — calibrated β≈0.5, δ≈0.99 explaining household balance sheets.

## Escalate to Raw When
- You need the continuous-time generalization of present bias (Harris-Laibson) and its smooth-policy properties. **Source:** Ericson & Laibson (2019) Ch.1 §2.1 p.19.
- You need the original Phelps-Pollak intergenerational interpretation versus the intra-personal reinterpretation. **Source:** Ericson & Laibson (2019) Ch.1 §2.1 p.17.
