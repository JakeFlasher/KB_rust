---
schema_version: "cacg.v0"
id: "fa-shleifer-vishny-limits-to-arbitrage"
title: "Shleifer-Vishny Limits to Arbitrage: Performance-Sensitive Capital"
reading_id: "22_fund_level_arbitrage"
summary: "Arbitrageurs run on performance-sensitive outside capital. When mispricing widens, interim losses trigger investor withdrawals, forcing liquidation exactly when the opportunity is richest, so the wedge persists rather than closes."
tags: ["shleifer-vishny", "performance-sensitive-capital", "limits-to-arbitrage"]
citations:
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p108:0113"
    chunk_hash: "0a8a95b6b7720b137dda40bfd224ba007f22d23af19d2573c811cd77d5e77b05"
    page_range: [108, 108]
    quote: "This dynamic could significantly limit arbitrageurs’ effectiveness in achieving market efficiency because they do not have the capital resources to eliminate mispricing."
    edge_type: "defines"
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p109:0115"
    chunk_hash: "f280434f7f032ff5e64f6d5d130ec01401348a10e0cd5df117fd82105fc6f712"
    page_range: [109, 109]
    quote: "In this version of the model, we assume that the arbitrageur is a fund"
    edge_type: "supports"
---
# Shleifer-Vishny Limits to Arbitrage: Performance-Sensitive Capital

## Intuition
Textbook arbitrage assumes the arbitrageur trades with deep, patient, own-money capital, so any law-of-one-price violation is instantly competed away. Real fund-level arbitrage separates the people who *supply* capital (pensions, endowments, wealthy individuals) from the people who *implement* the trade (hedge funds, prop desks). Outside capital chases past returns. When a mispricing widens, the arbitrageur posts an interim mark-to-market loss; outside investors who cannot distinguish "transient divergence" from "the manager was wrong" pull their money. The arbitrageur is then forced to unwind the position at the very moment the expected return is highest. Stabilizing capital evaporates precisely when the market needs it most, so the wedge persists instead of converging.

The perverse feedback is that *widening* mispricing — which mechanically *raises* the forward expected return — is exactly what triggers the *withdrawal* that removes the capital needed to capture it.

```
mispricing widens  ->  interim loss on the book  ->  outside investors withdraw
      ^                                                        |
      |                                                        v
      +----------  wedge persists  <--  forced liquidation (sell low / cover high)
                   (no patient capital left to close it)
```

**Source:** van der Merwe (2015) pp.107-108.

## Definition
- **Limits to arbitrage**: the empirical fact that the arbitrage process *sometimes fails* to push prices to fundamental value, because executing the trade requires capital subject to frictions (margins, haircuts, funding) and to withdrawal by outside investors.
- **Separation of capital and implementation**: outside investors supply funds to specialized arbitrageurs; this creates a "responsiveness of funds under management to past returns."
- **Shleifer-Vishny agency model**: an asset-pricing model fusing classic risk/return with behavioral capital-allocation; it has two agent types — *third-party investors* (uninformed capital suppliers) and *arbitrageurs* (specialized fund managers). The arbitrageur's allocated capital depends on past performance, so adverse interim performance can trigger forced redemption.

**Source:** van der Merwe (2015) pp.106-109.

## Mathematical Reasoning
Let two equivalent zero-coupon bonds A and B have fundamental value V at the terminal date. A date-0 shock opens a mispricing M0 > 0 (P0A = P0B - M0). With probability kappa the mispricing *grows* to M1 > M0; with probability (1 - kappa) it vanishes. Let phi be the probability the arbitrageur is *forced to liquidate early* (investors withdraw) if the mispricing widens.

Expected profit if the arbitrageur **waits** to date 1:
- Pi_wait = kappa * M1   (captures M1 only in the state where the wedge survives).

Expected profit if the arbitrageur **acts at date 0** under withdrawal risk:
- Pi_act(phi) = M0 - kappa * phi * M1   (pockets M0 now, but loses on the forced unwind when the wedge widens *and* capital is pulled).

Waiting dominates iff Pi_wait > Pi_act, i.e. kappa * M1 > M0 - kappa * phi * M1. Solving for the liquidation-risk threshold:

  phi* = (M0 - kappa * M1) / (kappa * M1).

Comparative statics: the arbitrageur defers (does *not* stabilize early) when phi > phi* — that is, when forced-liquidation risk is high enough. Higher phi (more performance-sensitive capital) and larger M1 (richer delayed payoff) both push toward *waiting*, leaving the date-0 wedge uncorrected. The binding constraint is structural: the arbitrageur becomes capital-constrained in the high-kappa state — exactly the state of widest mispricing and highest expected return.

**Source:** van der Merwe (2015) pp.109-111.

## See Also
- [`fa-market-liquidity-dimensions-and-no-arbitrage`](./fa-market-liquidity-dimensions-and-no-arbitrage.md) — the no-arbitrage benchmark this card shows breaking when capital is constrained.
- [`fa-funding-spirals-and-fire-sales`](./fa-funding-spirals-and-fire-sales.md) — the macro amplification: margin/haircut funding shocks that turn redemptions into self-reinforcing fire sales.
- [`fa-limits-to-arbitrage-when-creation-channel-breaks`](./fa-limits-to-arbitrage-when-creation-channel-breaks.md) — the ETF-specific analogue, where the creation/redemption arbitrage rail fails.
- [`fa-illiquidity-discount-and-crisis-amplification`](./fa-illiquidity-discount-and-crisis-amplification.md) — how persistent unclosed wedges show up as a negative liquidity premium.
- [`mt-funding-liquidity-fire-sales`](../14_microstructure_and_trading/mt-funding-liquidity-fire-sales.md) and `mt-value-traders-arbitrageurs` (reading 14) derive the same performance-sensitive-capital limits-to-arbitrage mechanism from primary sources; this card applies it to the fund creation/redemption break.
- [`be-limits-of-arbitrage`](../10_behavioral_finance/be-limits-of-arbitrage.md) (reading 10, already migrated) is the canonical Shleifer-2000 owner of the "arbitrageurs most constrained when opportunities are best" result; defer the core derivation there at migration.

Legacy cross-refs (other tree, prose only): the behavioral-finance card be-limits-of-arbitrage develops the same Shleifer-Vishny separation of capital and implementation from the sentiment side, and be-noise-trader-equilibrium frames the demand shock (here the nonspecialists' overly pessimistic value V - delta) as noise-trader risk that rational arbitrageurs cannot fully offset.

## Escalate to Raw When
Go to the source when you need the worked three-period numerical walk-through of the agency model — the explicit date-0/date-1/date-2 cash flows under the three scenarios (intervene-and-hold, intervene-and-be-liquidated, wait), the LTCM 25-to-1 leverage case study, and the S&P 500 index-deletion example. Also escalate for the derivation of the equilibrium date-1 price from the intersection of nonspecialist supply (1 - M1/delta) and constrained arbitrageur demand, and the resulting closed-form illiquidity discount.

**Source:** van der Merwe (2015) pp.107-112.
