---
schema_version: "cacg.v0"
id: "pa-transaction-based-attribution-and-trading-cost"
title: "Transaction-Based Attribution and the Trading-Cost Bridge"
reading_id: "15_performance_and_attribution"
summary: "Transaction-based attribution reads value-add directly from trade-level data, reconciling exactly to the published return and capturing intraperiod timing, bid/offer, and transaction costs that holdings-based attribution leaves as a residual."
tags: ["transaction-based-attribution", "trading-cost", "holdings-based"]
citations:
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p268:0319"
    chunk_hash: "d2f2fcd673ce582f33f53413e21f36aa1f82a2981c459cfd5f8b80732c15e7a5"
    page_range: [268, 268]
    quote: "Transaction-based attribution is the most complete form of attribution identifying all sources of return and reconciling exactly to the published return."
    edge_type: "defines"
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p192:0188"
    chunk_hash: "22d99d1e048fb580d74fb0ba60b78fc4f89cdaf5ac188a031a3def2562ddfdd5"
    page_range: [193, 193]
    quote: "Cash Flows The framework by itself does not address cash flows between segments."
    edge_type: "supports"
card_hash: "bfd3eb49ad438cf1f5db84189dca1035f1f9b73cd5bb6c996baf0142903924ae"
---
# Transaction-Based Attribution and the Trading-Cost Bridge

## Intuition

Attribution can be fed from three depths of data. **Returns-based** attribution regresses the return stream onto factors and never looks inside the portfolio. **Holdings-based** attribution takes the beginning weights and applies externally sourced returns, ignoring every trade made during the period. **Transaction-based** attribution feeds on the actual trade tape — every buy and sell at its actual transaction price — so it sees the things a holdings snapshot cannot: a position opened and closed mid-period, the bid/offer spread paid, the commission. The payoff is reconciliation: a transaction-based decomposition sums *exactly* to the return the client was reported, leaving no residual to explain away. The price is operational overhead and a demand for clean, granular data.

**Source:** Bacon (2023) §6 (Return Attribution) printed pp.245-246 (PDF pp.267-268)

## Definition

Bacon defines transaction-based attribution as *"Return attribution calculated directly from transaction and holdings-based data,"* and frames it as the most complete of the three types because it identifies all sources of return and reconciles exactly to the published return. Because the transaction price is sourced directly, the decomposition can measure **timing effects within the measurement period**, the **bid/offer spread**, and **transaction costs** — return components that the holdings-based approach simply drops.

The contrast is sharp. Holdings-based attribution ignores the impact of transactions during the period, so its explained return *will not reconcile* with the actual portfolio return; the gap is a **residual** that tends to grow with more active managers, large cash flows, illiquid assets, and longer measurement periods. Because the holdings analyst never expects reconciliation, operational errors and the value added by the trading desk both go unseen. The trading-cost bridge therefore lives only in the transaction-based form: Bacon notes that almost all transaction-based methodologies absorb transaction costs into the **stock-selection effect** by default, while allocation effects are measured against the category and overall benchmark with no allowance for cost — even though allocation decisions, once implemented, clearly generate cost that arguably belongs to the allocator.

**Source:** Bacon (2023) §6 (Return Attribution) printed pp.246, 271 (PDF pp.268, 293)

## Mathematical Reasoning

Let the published whole-portfolio return be `R`. Any attribution model decomposes `R` into a sum of named effects plus a residual `epsilon`:

```
   R  =  sum (allocation_i + selection_i + interaction_i)  +  epsilon
        +------------ explained by the model ------------+   + unexplained +

   holdings-based:      epsilon != 0   (grows with activity, cash flows, illiquidity, period length)
   transaction-based:   epsilon = 0    (reconciles exactly to the published R)
```

**Source:** Bacon (2023) §6 (Return Attribution) printed p.246 (PDF p.268)

The completeness claim is the statement `epsilon = 0`: every basis point of `R`, including the basis points spent crossing the spread and paying commission, is assigned to a named effect. This holds only because the inputs are the *actual* transactions at their *actual* prices, so the model's reconstructed return is the realized return by construction rather than an estimate built from external valuations. Christopherson, Cariño and Ferson make the same point from the framework side: the sector-based decomposition "by itself does not address cash flows" between segments, which is why one must either use time-weighted returns or shorten the period under a buy-and-hold assumption — and even then one must "acknowledge and understand the possible discrepancy between" the actual return and the weighted return. That discrepancy is precisely the holdings-based residual `epsilon`; capturing the intraperiod transactions is what drives it to zero. Bacon and CCF both assert the completeness/reconciliation property descriptively rather than proving it from a stated axiom set, so this card asserts it likewise and labels the gap.

**Source:** Bacon (2023) §6 printed p.246 (PDF p.268); Christopherson, Cariño & Ferson (2009) §18 printed p.180 (PDF p.193)

## Boundary Notes

This card owns the *attribution-side* treatment of trading cost — how cost enters and reconciles inside a value-add decomposition. It does **not** own the execution-quality measurement of trading cost itself: the implementation-shortfall decomposition (decision price vs. arrival vs. execution vs. opportunity cost) lives in topic-14 microstructure (see `mt-implementation-shortfall`). The trading-cost *bridge* here is the seam between the two: the transaction cost that implementation shortfall measures is the same quantity that transaction-based attribution absorbs (by default into stock selection). The choice of which depth — returns/holdings/transaction-based — to use is itself driven by purpose, decision process, data quality and availability, cost, asset type, and reporting objective, not by a single dominance rule.

**Source:** Bacon (2023) §6 (Return Attribution) printed p.246 (PDF p.268)

## See Also

- [`pa-brinson-bhb-allocation-selection-interaction.md`](pa-brinson-bhb-allocation-selection-interaction.md) — the allocation/selection/interaction decomposition that transaction-based data feeds and reconciles.
- [`pa-multilevel-attribution-successive-notional-funds.md`](pa-multilevel-attribution-successive-notional-funds.md) — the holdings-based decision-tree decomposition whose residual transaction-based data eliminates.
- [`pa-true-twr-and-chain-linking.md`](pa-true-twr-and-chain-linking.md) — why intraperiod cash flows and revaluation force time-weighting, the same data problem that separates holdings- from transaction-based attribution.
- [`pa-fi-carry-rolldown-pulltopar-time-decomposition.md`](pa-fi-carry-rolldown-pulltopar-time-decomposition.md) — a fixed-income time-decomposition that, like transaction-based attribution, aims to reconcile every component of realized return.

Cross-vertical: the trading-cost component absorbed here is measured as execution quality by topic-14 microstructure's [`mt-implementation-shortfall`](../14_microstructure_and_trading/mt-implementation-shortfall.md); GIPS composite-return reconciliation (topic-17 ethics) relies on the same exact-reconciliation discipline.
- [`mt-effective-cost-trade-benchmark`](../14_microstructure_and_trading/mt-effective-cost-trade-benchmark.md) — cross-set: implementation-shortfall / effective-cost trade benchmark (reading-14 execution-cost measures; reading-15 attribution absorption).
- [`fa-tracking-error-attribution-and-tco`](../22_fund_level_arbitrage/fa-tracking-error-attribution-and-tco.md) — cross-set: trading-cost / tracking-error attribution (reading-22 ETF total-cost-of-ownership; reading-15 transaction-based attribution).
## Escalate to Raw When

- You need the worked numeric example showing a transaction-based attribution reconciling exactly to the published return while a parallel holdings-based run leaves a quantified residual.
- You must decide where to book a specific transaction cost — Bacon's default is stock selection, but allocation-driven costs (e.g., emerging-market entry) may need reassigning to the allocator, and the worked allocation is in the raw.
- You need the off-benchmark (zero-weight sector) attribution mechanics or the full advantages/disadvantages table (Bacon Table 6.2) comparing returns-, holdings-, and transaction-based approaches.
- You need CCF's full set of input-consistency rules (Dietz-adjusted weights, buy-and-hold period choice, internally consistent weights) that govern when the holdings-based residual is tolerable.
