---
schema_version: "cacg.v0"
id: "cb-brooks-range-mean-reversion-double-low"
title: "Brooks Range / Mean-Reversion Vocabulary for the 双低 Rotation Workflow"
reading_id: "08_convertible_bonds"
summary: "The Chinese-CB 双低 strategy (low price ∩ low conversion premium) is a defensive screening rule that picks the smallest-N names by composite score and rebalances on swap discipline; it operates as a range / mean-reversion workflow that Brooks's range-mean-reversion and first-pullback vocabulary describes without changing the load-bearing 安道全 composite screen."
tags: ["convertible-bonds", "brooks-range"]
citations:
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p072:0040"
    chunk_hash: "8b4e745d7454d36988e621507f411b490101ff84f166f4f15d42391e960d2391"
    page_range: [72, 73]
    quote: "2.5 “双低”策略：泥潭行军 ‚双低‛答略是网丆最常见的可转债答略，基本原则就是买入可 转债价和转股溢价率都低的可转债。"
    edge_type: "defines"
  - source_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar"
    chunk_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar:p101:0128"
    chunk_hash: "90b7aca378ab49a5dc56533bb7a6479985260eb8eaa672cc97c736fbd25cfc3a"
    page_range: [101, 102]
    quote: "If there is a trend, even a small one, any pullback that breaks any trendline defines that trend as a leg, and the pullback as another leg."
    edge_type: "supports"
  - source_id: "tpa_brooks_2012_trading_ranges"
    chunk_id: "tpa_brooks_2012_trading_ranges:p192:0174"
    chunk_hash: "fd393472d5763d227f5479859730e1577a67b100e1dfef6c91b9084a2ac60334"
    page_range: [192, 193]
    quote: "Uncertainty is the hallmark of a trading range, so a breakout is a search for a trading range, for uncertainty, and a 50 percent directional probability of an equidistant move."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3403:5134"
    chunk_hash: "65f13607d2f88c321b7d8e5a51d079a1f658be146cc012512d1a5aec7aa89072"
    page_range: [3403, 3404]
    quote: "Technical analysis is a form of security analysis that uses price and volume data, often graphically displayed, in decision making."
    edge_type: "supports"
card_hash: "a5fc1299fca1c7341dcd8d25de0b686b3b178354796e696089839c6f3a1a75d2"
---
# Brooks Range / Mean-Reversion Vocabulary for the 双低 Rotation Workflow

## Intuition

The Chinese-CB 双低 strategy (双低 = 低价 ∩ 低溢价率, low price intersected
with low conversion premium) is a defensive screening rule that picks
the smallest 5-20 names by composite score and rebalances on swap
discipline (replace a held name when a candidate's score is at least a
few units lower). The screen is operationally a range / mean-reversion
workflow: it buys CBs that have fallen into the lower band of the
universe-wide distribution of double-low scores and rotates as the
distribution evolves. It does NOT rely on Brooks-style trend-following
or breakout setups. **Source:** 安道全 (2023) §2.3-§2.4 pp.59-78.

```
   双低 rotation as range / mean-reversion (universe-wide):

       double-low ↑
       score      │
                  │ █  █                              high-band names:
                  │ █  █  █                            not in the basket
                  │ █  █  █  █
                  │ ─────────────────────────────  swap-discipline upper
                  │ ░  ░  ░  ░  ░       (mid band)
                  │ ─────────────────────────────  basket entry threshold
                  │ ▒  ▒  ▒  ▒  ▒  ▒  ▒  (low band)   held basket: smallest
                  │ ▒  ▒  ▒  ▒  ▒  ▒  ▒                 by composite score
                  │
                  └────────────────────────────→ CBs (sorted by composite)
```

The mean-reversion mechanism is across NAMES, not across BARS within
one name's chart. The screen assumes that the cheapest combination of
price and premium across the listed universe is incidentally a
defensive basket (cheap CBs have higher to-maturity yield cushion;
low-premium CBs have closer-to-parity equity sensitivity). The
practitioner does not need a chart-pattern trigger to enter or exit;
the trigger is the swap-discipline arithmetic on the composite score.
**Source:** 安道全 (2023) §2.3-§2.4 pp.59-78.

The weak-form-efficiency canon constrains how this card may frame the
screen's expected return. The 双低 screen is a cross-sectional
strategy whose past empirical performance is recorded in 攻守 (cited
on [`cb-china-double-low-strategy.md`](./cb-china-double-low-strategy.md#mathematical-reasoning));
this card neither replicates nor extends those claims. The Brooks
overlay is a vocabulary mapping for the chart-side description of
each held name's individual tape, not an alpha-generating layer over
the screen. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

## Definition

The double-low rotation workflow has three components — the universe-
wide composite-score distribution, the swap-discipline rule, and the
chart-grammar description of each held name's individual tape using
Brooks's range / mean-reversion / first-pullback vocabulary.
**Source:** 安道全 (2023) §2.3-§2.4 pp.59-78.

| Claim area | Primary anchor (08 PDF + page span) | Brooks supporting span (chapter + page span) | Allowed use | Forbidden inference |
|------------|--------------------------------------|----------------------------------------------|-------------|---------------------|
| Universe-wide composite-score band identification | 安道全 (2023) §2.3-§2.4 pp.59-78 (低价 ∩ 低溢价率 ranking and basket selection) | Brooks (2012) Trading Ranges Ch.11-12 pp.190-290 (range-anchored bottoming / mean-reversion grammar) | describe the held basket as occupying the low band of the score distribution | claim that a Brooks-style chart pattern at the basket level predicts the universe-wide score distribution shift |
| Held-name chart description (range / mean-reversion) | 安道全 (2023) §2.3-§2.4 pp.59-78 | Brooks (2009) Ch.4 pp.100-145 (pullbacks inside a range) | use Brooks's pullback and first-leg-down vocabulary to describe each held name's intraday tape | claim that the Brooks pullback count predicts the timing of the swap-discipline trigger |
| Swap-discipline trigger (composite-score arithmetic) | 安道全 (2023) §2.3-§2.4 pp.59-78 (rotation rule when candidate score is sufficiently lower) | Brooks (2012) Trading Ranges Ch.11-12 pp.190-290 (descriptive only) | describe the rotation cadence as a periodic universe-wide rescan | derive the rotation cadence from Brooks chart pattern recognition |

The chart-grammar mapping then translates Brooks's range / mean-
reversion vocabulary onto each held name's tape. **Source:** Brooks
(2009) Ch.4 pp.100-145; 安道全 (2023) §2.3-§2.4 pp.59-78.

| Brooks term | Held-name tape equivalent | Source anchor (Brooks) | Source anchor (Chinese-CB) |
|-------------|----------------------------|------------------------|----------------------------|
| range mean-reversion | held CB oscillating in its individual 震荡市 band | Brooks (2009) Ch.5 pp.145-200 | 安道全 (2023) §2.3-§2.4 pp.59-78 |
| first pullback (descriptive) | initial dip after a held CB's early appreciation, before a possible rotation candidate emerges | Brooks (2012) Trading Ranges Ch.11-12 pp.190-290 | 安道全 (2023) §2.3-§2.4 pp.59-78 |
| double bottom / double top (descriptive grammar only) | tape feature recorded in passing; the swap decision is composite-score-driven, not pattern-driven | Brooks (2012) Trading Ranges Ch.11-12 pp.190-290 | 安道全 (2023) §2.3-§2.4 pp.59-78 |
| range expansion | universe-wide widening of the composite-score distribution (more names enter the low band) | Brooks (2009) Ch.5 pp.145-200 | 安道全 (2023) §2.3-§2.4 pp.59-78 |

The composite-score arithmetic and the basket-selection rule belong
to [`cb-china-double-low-strategy.md`](./cb-china-double-low-strategy.md#definition).
This card's role is to make explicit that the workflow is range /
mean-reversion in character — not trend-following or breakout — and
that Brooks's vocabulary describes that character. **Source:** 安道全
(2023) §2.3-§2.4 pp.59-78.

## Mathematical Reasoning

The mean-reversion claim operates on the CROSS-SECTIONAL distribution
of composite scores rather than the TIME-SERIES of any one CB's
price. Across the listed universe, the composite score has an
empirical distribution; the held basket occupies the lowest tail of
that distribution. As individual CBs in the held basket appreciate
(their scores rise), they exit the tail; as other CBs fall (their
scores enter the tail), they enter the basket. The rotation is the
arithmetic consequence of the composite score being an order
statistic. **Source:** 安道全 (2023) §2.3-§2.4 pp.59-78.

**Do not infer:** The Brooks range / mean-reversion vocabulary is
execution-context only; describing a held CB's tape with Brooks's
range terminology implies no predictive edge above the unconditional
benchmark. The weak-form-efficiency canon
([[pm-market-efficiency-core]]) holds for the chart-side description.
Whatever empirical edge the 双低 rotation has historically
demonstrated — as recorded in [[cb-china-double-low-strategy]] from
attic-cited backtest windows — is anchored on the screen's cross-
sectional logic, not on Brooks vocabulary. This card does not
relitigate or extend the empirical record; it only marks the chart-
grammar overlay as descriptive. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

The asymmetric attractiveness of range / mean-reversion vs trend /
breakout vocabulary for the 双低 setting is a structural property of
the screen, not a Brooks distinction. A cross-sectional low-band
screen rebalances as the composite-score distribution evolves; this
evolution looks like mean reversion in the tail (cheap names rotate
out, new cheap names rotate in) and looks unlike trend continuation
(the basket is not chosen for its momentum). Brooks's range
vocabulary therefore ports naturally, while his trend / breakout
vocabulary does not. **Source:** 安道全 (2023) §2.3-§2.4 pp.59-78.

The two failure modes most relevant here are MA/EMA pullback signals
and formal chart pattern names. The screen does not consume moving-
average crosses; the composite-score arithmetic is sufficient.
Formal chart pattern names describe multi-bar shapes that the screen
ignores. **Source:** 安道全 (2023) §2.3-§2.4 pp.59-78.

## See Also

- [`cb-brooks-price-action-guardrails.md`](./cb-brooks-price-action-guardrails.md#intuition) — the supplement-only framing this card inherits.
- [`cb-brooks-trend-range-regime-map.md`](./cb-brooks-trend-range-regime-map.md#intuition) — the regime classification that dispatches to the 双低 workflow when the universe-wide pattern is in a range / mean-reversion phase.
- [`cb-china-double-low-strategy.md`](./cb-china-double-low-strategy.md#mathematical-reasoning) — the composite-score screen this card supplies vocabulary for; the load-bearing rotation rule lives there.
- [`pm-market-efficiency-core.md`](../09_portfolio_management_and_asset_pricing/pm-market-efficiency-core.md#definition) — weak-form-efficiency canon; the chart-grammar overlay is descriptive.

## Escalate to Raw When

Open Brooks (2009) Ch.4 pp.100-145 directly when the reader needs the
full pullback / first-leg-down vocabulary beyond the range / mean-
reversion mapping recorded here. **Source:** Brooks (2009) Ch.4
pp.100-145.

Open Brooks (2012) Trading Ranges Ch.11-12 pp.190-290 for the deeper
first-pullback / double-bottom / double-top vocabulary; the
operational use in 双低 rotation is bounded by the composite-score
arithmetic documented in [`cb-china-double-low-strategy.md`](./cb-china-double-low-strategy.md#definition).
**Source:** Brooks (2012) Trading Ranges Ch.11-12 pp.190-290.

Open 安道全 (2023) §2.3-§2.4 pp.59-78 directly for the screen's per-
CB construction; the card here records the vocabulary overlay but
not the worked calibration. **Source:** 安道全 (2023) §2.3-§2.4
pp.59-78.
