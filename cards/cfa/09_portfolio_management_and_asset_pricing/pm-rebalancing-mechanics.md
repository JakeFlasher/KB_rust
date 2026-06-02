---
schema_version: "cacg.v0"
id: "pm-rebalancing-mechanics"
title: "Rebalancing Mechanics"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Rebalancing Mechanics: framing rebalancing as the discipline that returns drifted weights to strategic targets, contrasting calendar and threshold rules, and analyzing the transaction-cost / tracking-risk tradeoff"
tags: ["portfolio-management", "rebalancing", "asset-allocation"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3302:4972"
    chunk_hash: "5569c66ae1e4da0cf78252198a430b4d5d00edaf415cb98d146e2619e62eddbb"
    page_range: [3302, 3303]
    quote: "Periodically, or when a certain threshold deviation from the policy weight (the bandwidth) has been breached, the portfolio should be rebalanced back to the policy weights."
    edge_type: "defines"
card_hash: "0a6256831c43d56c8d2504bc93c0f5baf967a710baa71bac51f77d5c16e5e909"
---
# Rebalancing Mechanics

## Intuition

Market drift erodes the strategic asset allocation: if equity
outperforms fixed income over a quarter, the equity weight rises and
the fixed-income weight falls without any decision being made. Left
unchecked, drift converts the carefully chosen `w_SAA` into a
risk-loaded portfolio that no longer matches the IPS risk objective.
Rebalancing is the corrective action — selling the drifted-up class
and buying the drifted-down class — that returns the realized weight
vector to the policy target. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.35-48.

```
        w_actual after market drift
        ============================>
   +--------+         +--------+         +--------+
   | equity |   -->   | equity |  <--    | equity |
   +--------+         | EXCESS |         +--------+
   |  bond  |         +--------+         |  bond  |
   +--------+         |  bond  |         +--------+
                      | UNDER- |
                      |  WT    |
                      +--------+
   policy target      drifted weights     post-rebalance:
   w_SAA              after market move   sell excess /
                                          buy under-weight
                                          to return to w_SAA
```

The investor faces a tradeoff. Tighter rebalancing (smaller drift
tolerance, or more frequent calendar review) keeps the realized
portfolio closer to the policy target, reducing tracking risk
against the IPS-implied benchmark. Looser rebalancing reduces
trading volume and therefore transaction costs, taxes, and market-
impact slippage. The optimal rule trades the tracking benefit
against the trading cost. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.35-48.

## Definition

A rebalancing rule has two ingredients: a trigger that determines
when to act, and a target that determines where to rebalance to.
The L1 framing names two trigger families and one default target
choice. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.35-48.

The calendar rebalancing trigger acts on a fixed schedule —
quarterly, semiannually, or annually — independent of how far
weights have drifted. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.35-48.

```
trigger_calendar(time):  rebalance if (time mod tau_cal) == 0
```

The threshold (or "percentage-of-portfolio") rebalancing trigger
acts whenever any class's realized weight deviates from its target
by more than a tolerance band. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.35-48.

```
trigger_threshold:  rebalance class i if |w_i_actual - w_i_SAA| > band_i
```

The target choice is typically the strategic weight `w_SAA` (full
rebalance to policy) or a "rebalance to band edge" (rebalance only
the drifted class back to the band boundary, not all the way to
the target). The full-rebalance target is simpler; the band-edge
target reduces trading volume per event. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.35-48.

## Mathematical Reasoning

The cost-benefit calculus for a rebalancing decision compares the
expected reduction in tracking variance against the deterministic
trading cost. Symbolically, the per-event tradeoff at decision
horizon `h` reads as a difference. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.35-48.

```
benefit(rebalance)
   = E[ tracking_var(no_action, h) - tracking_var(rebalanced, h) ]
cost(rebalance)
   = sum_i |trade_i| · (commission_i + spread_i + impact_i)
act_if : benefit > cost
```

Tracking variance grows over the holding horizon `h` whenever no
rebalancing occurs, because correlated drift accumulates: the
covariance structure of the asset classes does not preserve the
target weights. Rebalancing collapses the cumulative drift back to
zero. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.35-48.

The threshold rule is more efficient than the calendar rule under
two conditions, both intuitive. First, when transaction costs
dominate, the threshold rule avoids forced trades on dates when
weights happen to be near target — the calendar rule pays the
trading cost regardless. Second, when volatility is heterogeneous
across classes, the threshold rule lets stable classes go untraded
while triggering only on volatile classes, whereas the calendar
rule trades the whole portfolio on every fixed date. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.35-48.

The threshold rule has a downside the calendar rule lacks: the
trigger condition depends on observed drift, so realized trading
frequency rises in volatile markets when transaction costs are
typically higher (wider spreads, larger market impact). The
calendar rule has a known cost schedule by construction; the
threshold rule's expected cost is a function of realized
volatility. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.35-48.

A hybrid rule — threshold trigger with an outer calendar safety net
— captures most of each rule's strength. The threshold trigger
handles routine drift; the calendar safety net prevents a long
quiet period from leaving any allocation unreviewed. The
curriculum treats the hybrid as standard practice. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.35-48.

The IPS records the chosen rule (calendar / threshold / hybrid),
the trigger parameters (`tau_cal` and per-class `band_i`), and the
target policy (full rebalance vs band edge). Without these records,
rebalancing decisions become discretionary; the IPS discipline
removes that discretion. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.35-48.

## See Also

- [`pm-allocation-process.md`](pm-allocation-process.md) — the strategic / tactical layers whose target weights rebalancing returns to
- [`pm-investment-policy-statement.md`](pm-investment-policy-statement.md) — the document that records the rebalancing rule, trigger parameters, and target

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R51 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.35-48.

- Worked examples of band-width selection under specific volatility
  and transaction-cost assumptions — the curriculum walks through
  illustrative parameter choices that the present card abstracts.
  **Source:** CFA L1 Curriculum (2022) Vol.6/pp.35-48.
- Tax-aware rebalancing for taxable accounts (loss-harvesting
  interaction; wash-sale rules; tax-lot selection) — these belong
  in future-13. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.35-48.
- Rebalancing with derivatives overlays (futures-based equity-bond
  rebalancing without liquidating spot) — the curriculum mentions
  the technique; deeper construction belongs at the intersection of
  09 and 07. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.35-48.
