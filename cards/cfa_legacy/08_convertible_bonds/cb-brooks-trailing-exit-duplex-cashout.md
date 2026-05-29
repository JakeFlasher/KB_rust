---
schema_version: "cacg.v0"
id: "cb-brooks-trailing-exit-duplex-cashout"
title: "Brooks Trailing-Exit Vocabulary for the 复式兑现 + 回撤价 Cashout Discipline"
reading_id: "08_convertible_bonds"
summary: "安道全's 复式 cashout discipline is a three-state sell-side playbook around the strong-call corridor: sell at corridor to lock the gain, re-enter if the CB later trades below corridor by more than the holder's 回撤价 tolerance; Brooks's trailing-exit vocabulary names the chart moves while clause-anchored discipline carries the exit decision."
tags: ["convertible-bonds", "brooks-trailing"]
citations:
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p271:0160"
    chunk_hash: "568720e0171bc5e04dcf85e2cfd0e0a5c35e86b7bee8f54f5f8a7850bc9bc02b"
    page_range: [271, 272]
    quote: "复式的触収标准，本杢就不是‚是否公告强赎‛，而是‚丆了 130 元以后回撤一定程度‛。"
    edge_type: "defines"
  - source_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar"
    chunk_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar:p381:0470"
    chunk_hash: "89245aba784eb87d93ecb54bdb673f54f513c75808ebabd9c6262319eb3faa24"
    page_range: [381, 381]
    quote: "The most difficult part of trading is deciding whether a setup is good enough to warrant placing a trade."
    edge_type: "supports"
  - source_id: "tpa_brooks_2012_trading_ranges"
    chunk_id: "tpa_brooks_2012_trading_ranges:p623:0593"
    chunk_hash: "0a6c19dc0cadcac699928376d42f3ee48d2f42b9c8cccfea83b1086e7cba5db8"
    page_range: [623, 624]
    quote: "It is far more accurate to think of them as continuation patterns that rarely fail but, when they do, the failure can lead to a reversal."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3403:5134"
    chunk_hash: "65f13607d2f88c321b7d8e5a51d079a1f658be146cc012512d1a5aec7aa89072"
    page_range: [3403, 3404]
    quote: "Technical analysis is a form of security analysis that uses price and volume data, often graphically displayed, in decision making."
    edge_type: "supports"
card_hash: "ec384d765b8f4cd746549804d953db9157292a69d876a037afd4852b6798843a"
---
# Brooks Trailing-Exit Vocabulary for the 复式兑现 + 回撤价 Cashout Discipline

## Intuition

安道全's 复式 (duplex) cashout discipline is a three-state sell-side
playbook around the strong-call corridor on Chinese CBs. When the
issuer announces strong-call (the trigger condition activates the
mandatory-conversion countdown), the holder is contractually pushed
toward a near-term decision: convert at the prevailing parity, sell
at the corridor price, or risk holding through the redemption date.
安道全's discipline says: sell at the corridor to lock in the gain;
if the same CB later trades below the corridor by more than the
holder's pre-set 回撤价 tolerance, re-enter; if a more attractive
candidate emerges after the sale, rebuild at the new candidate
instead. Brooks's trailing-exit vocabulary (trend-reversal-as-exit-
trigger, Trader's-Equation cadence) supplies the chart-grammar
overlay for naming these moves. **Source:** 安道全 (2023) Ch.5
pp.260-298.

```
   复式兑现 + 回撤价 cashout (one CB, around strong-call corridor):

       price ↑
             │   ╲╲                          (price falls back from corridor)
             │     ╲╲    rebuild candidate?
             │       ╲╲╱╲        ← 回撤价: re-enter if drop is deep enough
             │
             │   ────── strong-call corridor (sell exit) ──────
             │                ╱─→ sell here (复式 first leg: cash out)
             │              ╱
             │             ╱  ← Brooks: trailing exit at trend reversal
             │           ╱
             │         ╱       (CB has approached corridor in trend regime)
             │
             └───────────────────────────────────────────────→ time
```

The Brooks vocabulary overlay describes the chart-side action of each
move (a trend reversal in Brooks's language matches the strong-call-
corridor sell; a deeper-than-tolerance pullback matches the rebuild
trigger). The load-bearing decision is the clause-anchored corridor
and the holder's pre-set 回撤价. The weak-form-efficiency canon
constrains the chart-grammar overlay's predictive content: a trend-
reversal label at the corridor describes the chart but does not
predict the next bar's direction. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.420-441.

## Definition

The cashout discipline has three components — the clause-anchored
corridor that triggers the sell, the 回撤价 tolerance that triggers
the rebuild, and the chart-grammar overlay (Brooks's trend-reversal
and trade-management vocabulary). **Source:** 安道全 (2023) Ch.5
pp.260-298.

| Claim area | Primary anchor (08 PDF + page span) | Brooks supporting span (chapter + page span) | Allowed use | Forbidden inference |
|------------|--------------------------------------|----------------------------------------------|-------------|---------------------|
| Strong-call-corridor sell trigger | 安道全 (2023) Ch.5 pp.260-298 (clause-anchored sell at the corridor) | Brooks (2009) Ch.8 pp.270-330 (trend-reversal vocabulary at the exit point) | describe the sell as a Brooks-style trend-reversal exit overlaid on the clause trigger | claim the Brooks reversal predicts the corridor approach |
| 回撤价 rebuild trigger | 安道全 (2023) Ch.5 pp.260-298 (pre-set holder tolerance for re-entry on dips) | Brooks (2012) Trading Ranges Ch.25-32 pp.620-900 (Trader's Equation cadence; orders / trade management) | describe the rebuild trigger as a trailing pull-back tolerance in Brooks's order-management vocabulary | claim the Trader's Equation derives the optimal 回撤价 numerical level |
| Candidate-rotation rebuild | 安道全 (2023) Ch.5 pp.260-298 (rebuild at a more attractive candidate after sale) | Brooks (2012) Trading Ranges Ch.25-32 pp.620-900 (trade-management framing) | describe the rotation as a routine reset of the next position's entry-cadence layer | derive the candidate selection from Brooks chart pattern recognition alone |

The trailing-exit vocabulary maps onto the 复式兑现 cashout layer.
**Source:** Brooks (2009) Ch.8 pp.270-330; 安道全 (2023) Ch.5
pp.260-298.

| Brooks term | Chinese-CB cashout equivalent | Source anchor (Brooks) | Source anchor (Chinese-CB) |
|-------------|---------------------------------|------------------------|----------------------------|
| trend reversal (as exit cue) | strong-call corridor approach in a 单边市 leg up | Brooks (2009) Ch.8 pp.270-330 | 安道全 (2023) Ch.5 pp.260-298 |
| trailing stop (descriptive) | holder's 回撤价 tolerance for re-entry | Brooks (2009) Ch.13 pp.380-428 | 安道全 (2023) Ch.5 pp.260-298 |
| Trader's Equation cadence | exit-decision framing: locking in the corridor gain versus holding through redemption | Brooks (2012) Trading Ranges Ch.25-32 pp.620-900 | 安道全 (2023) Ch.5 pp.260-298 |
| order management (descriptive) | sell-and-rebuild routine around the corridor | Brooks (2012) Trading Ranges Ch.25-32 pp.620-900 | 安道全 (2023) Ch.5 pp.260-298 |

安道全's 复式 framing is a duplex — sell, then optionally rebuild —
rather than Brooks's one-leg trailing exit. The chart-grammar overlay
describes the two legs (the exit and the rebuild) using Brooks
vocabulary; the load-bearing rule (when to sell, when to rebuild) is
the clause-corridor and 回撤价 read, not Brooks's trend-reversal
identification. **Source:** 安道全 (2023) Ch.5 pp.260-298.

## Mathematical Reasoning

The cashout discipline is a state-machine, not a continuous-trading
rule. The machine has three states (holding, sold, rebuilt) and the
transitions are clause-anchored or tolerance-anchored: a strong-call
announcement triggers the holding-to-sold transition; a price drop
exceeding the pre-set 回撤价 tolerance triggers the sold-to-rebuilt
transition; a more attractive candidate appearing during the sold
state can redirect the rebuilt transition to a different name. The
state-machine's transitions are not chart-derived; Brooks's vocabulary
only describes the chart-side appearance of each transition.
**Source:** 安道全 (2023) Ch.5 pp.260-298.

**Do not infer:** Brooks's trend-reversal label and trailing-stop
language are execution-context only; identifying a Brooks-style
reversal at the corridor does not assert that the reversal label
predicts the next bar's direction. The weak-form-efficiency canon
([[pm-market-efficiency-core]]) holds for the chart-grammar overlay.
The load-bearing decisions (when to sell, when to rebuild, when to
rotate) live in [[cb-china-forced-redemption-tactics-2020s]] and
anchor on the clause-corridor read. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.420-441.

The asymmetric chart-side appearance of the exit and the rebuild
makes Brooks's vocabulary unevenly useful across the duplex legs.
The exit leg looks like a trend-reversal in Brooks's vocabulary
because the corridor sell is typically against an in-progress
directional move toward the trigger; this is a clean mapping. The
rebuild leg, when it occurs, looks like a first-pullback in a fresh
range; Brooks's range vocabulary supplies the description. The
candidate-rotation case has no Brooks analogue at all — it is a
universe-wide rescreen, not a chart pattern. **Source:** 安道全
(2023) Ch.5 pp.260-298.

Three failure modes are most relevant here. Bar-by-bar candlestick
reversals presume that a single-bar pattern triggers the exit; the
load-bearing trigger is the clause announcement, not a chart bar.
5-minute scalp setups conflict with the multi-day cashout cadence
typical for the corridor approach. Always-in long/short positioning
presumes a symmetric next-state from the sold state (re-enter short);
Chinese CBs do not support this because the corridor is a long-only
exit point and the holder cycles between long and flat.
**Source:** 安道全 (2023) Ch.5 pp.260-298.

## See Also

- [`cb-brooks-price-action-guardrails.md`](./cb-brooks-price-action-guardrails.md#intuition) — the supplement-only framing this card inherits.
- [`cb-china-forced-redemption-tactics-2020s.md`](./cb-china-forced-redemption-tactics-2020s.md#definition) — the canonical Chinese-CB 复式兑现 playbook this card supplies vocabulary for; the load-bearing cashout rule lives there.
- [`cb-china-three-line-duplex-strategy.md`](./cb-china-three-line-duplex-strategy.md#definition) — the entry-side counterpart that pairs with the cashout discipline (the buy-side 三线 mirrors the sell-side corridor).
- [`pm-market-efficiency-core.md`](../09_portfolio_management_and_asset_pricing/pm-market-efficiency-core.md#definition) — weak-form-efficiency canon; the chart-grammar overlay carries no expected-return content beyond the unconditional benchmark.

## Escalate to Raw When

Open Brooks (2009) Ch.8 pp.270-330 directly when the reader needs
Brooks's full trend-reversal-as-exit vocabulary beyond the corridor-
sell mapping recorded here. The card uses the generic word "reversal"
in the Brooks vocabulary sense (no Reversals-PDF citation, per project
policy). **Source:** Brooks (2009) Ch.8 pp.270-330.

Open Brooks (2012) Trading Ranges Ch.25-32 pp.620-900 directly when
the reader needs the Trader's Equation framing or the orders / trade-
management cadence beyond the trailing-stop and order-management
vocabulary mapped here. **Source:** Brooks (2012) Trading Ranges
Ch.25-32 pp.620-900.

Open 安道全 (2023) Ch.5 pp.260-298 directly for the per-CB 复式兑现
playbook and the 回撤价 tolerance derivation the author works
through. **Source:** 安道全 (2023) Ch.5 pp.260-298.
