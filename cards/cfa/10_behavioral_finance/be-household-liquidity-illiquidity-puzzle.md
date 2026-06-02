---
schema_version: "cacg.v0"
id: "be-household-liquidity-illiquidity-puzzle"
title: "Household Liquidity / Co-Holding Puzzle"
reading_id: "10_behavioral_finance"
summary: "Households simultaneously hold low liquid wealth, high illiquid wealth, and revolving high-interest debt with high MPC out of liquid changes; Laibson et al. explain this with present bias (beta~0.51, delta~0.99) where illiquidity acts as commitment."
tags: ["behavioral-finance", "present-bias", "household-finance", "commitment"]
citations:
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p045:0067"
    chunk_hash: "a7303be02874c7cb3b80dbade689908a4d26d55ec06f73de164e9f8425bf806b"
    page_range: [45, 45]
    quote: "Households tend to hold very low levels of liquid assets."
    edge_type: "supports"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p046:0069"
    chunk_hash: "f0349cbabd925b3e96bf600804880021e27b3450f34eda743aa9cf1233a21daa"
    page_range: [46, 46]
    quote: "households that borrow on their credit cards tend to also accumulate large stocks of illiquid wealth"
    edge_type: "supports"
  - source_id: "bf_hbe_vol2_2019"
    chunk_id: "bf_hbe_vol2_2019:p046:0070"
    chunk_hash: "e98f18a4caa5ea9a628b43509ba9f58ab2a7ada1886e2941ef81a814c4ba3aa0"
    page_range: [47, 47]
    quote: "= 0.99, and a coefficient of relative risk aversion of"
    edge_type: "supports"
card_hash: "bac3322b00ea00a25601b2ab099db9cc9986c344e94d071ea7f1b346ea1300a9"
---
# Household Liquidity / Co-Holding Puzzle

## Intuition
The typical U.S. household lives without a meaningful buffer of liquid assets, yet accumulates almost all of its voluntary wealth in *illiquid* forms (home equity, retirement accounts) — and does so while simultaneously revolving expensive credit-card debt. In any given month roughly half of households are not paying their card balance in full. These facts are puzzling for a frictionless model: why pay 6%+ on cards while locking savings into low-return illiquid assets? The co-occurrence within the *same* households rules out simple heterogeneity (patient savers vs impatient borrowers) as a full explanation.
**Source:** Ericson & Laibson (2019) Ch.1 §3.7 pp.45-46.

The behavioral resolution treats illiquidity as a feature, not a bug. A present-biased household cannot hold onto liquid assets — they get spent on immediate consumption — so it rationally (given its β) channels saving into illiquid vehicles that its future impatient selves cannot easily raid. Illiquidity functions as an implicit commitment device. The same present bias that makes liquid assets evaporate makes the household willing to borrow at high rates and to invest in illiquid assets with only modest returns, because its *long-run* discount rate is low.
**Source:** Ericson & Laibson (2019) Ch.1 §3.7 pp.46-47.

## Definition
**The co-holding / illiquidity puzzle** is the joint empirical regularity that households hold low liquid net wealth, relatively high illiquid net wealth, and a high marginal propensity to consume out of changes in liquid wealth — with borrowing and illiquid accumulation tending to occur in the same households.
**Source:** Ericson & Laibson (2019) Ch.1 §3.7 pp.45-46.

**High marginal propensity to consume (MPC)** out of liquid-wealth changes means households spend a large fraction of liquidity windfalls (anticipated or not), inconsistent with permanent-income smoothing.
**Source:** Ericson & Laibson (2019) Ch.1 §3.7 p.46.

## Mathematical Reasoning
The chapter contrasts a present-biased household's two discount rates. The short-run rate governs the willingness to spend liquid assets immediately (high impatience), while the long-run discount rate is only `ln δ ≈ 1 − δ`, typically estimated below the long-run real after-tax risk-adjusted return on housing or matched 401(k) contributions. Hence the household borrows on cards (short-run impatience) yet still invests in illiquid assets with modest returns (low long-run rate) — exactly the co-holding pattern.
**Source:** Ericson & Laibson (2019) Ch.1 §3.7 pp.46-47.

Laibson, Maxted, Repetto, and Tobacman use the method of simulated moments in a lifecycle consumption model, fitting voluntary wealth formation and credit-card borrowing jointly, and estimate `β = 0.51`, `δ = 0.99`, and a coefficient of relative risk aversion of `1.3`. The small `β` delivers the high MPC and the credit-card borrowing; the high `δ` delivers the illiquid accumulation. A non-behavioral alternative (Kaplan-Violante "wealthy hand-to-mouth") generates similar balance-sheet facts by assuming a negative real after-tax return on liquid wealth alongside a 6.29% illiquid return — the source notes both literatures continue to explore the mechanisms.
**Source:** Ericson & Laibson (2019) Ch.1 §3.7 pp.46-47.

```
   present-biased household balance sheet
   liquid assets   : ~0   (high short-run impatience -> spent fast)
   illiquid assets : HIGH  (acts as commitment; low long-run rate)
   high-rate debt  : HIGH  (short-run impatience -> borrow now)
   => co-holding of illiquid wealth AND expensive debt
```
**Source:** Ericson & Laibson (2019) Ch.1 §3.7 pp.46-47.

## See Also
- [be-quasi-hyperbolic-discounting](./be-quasi-hyperbolic-discounting.md#mathematical-reasoning) — the β-δ structure with the short-run vs long-run discount-rate wedge.
- [be-commitment-and-naivete](./be-commitment-and-naivete.md#intuition) — illiquidity as a commitment device.
- [be-present-focused-preferences-taxonomy](./be-present-focused-preferences-taxonomy.md#intuition) — present-focus as the umbrella explanation.

## Escalate to Raw When
- You need the SCF percentile detail (median net liquid assets by age cohort) and the payday-loan / bankruptcy statistics. **Source:** Ericson & Laibson (2019) Ch.1 §3.7 pp.45-46.
- You need the Aguiar-Hurst caloric-smoothing counterargument and the Stephens-Toohey rebuttal on retirement consumption. **Source:** Ericson & Laibson (2019) Ch.1 §3.7 p.46.
