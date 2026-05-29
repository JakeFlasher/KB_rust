---
schema_version: "cacg.v0"
id: "cb-china-three-line-duplex-strategy"
title: "China Convertible Bond — 三线—复式 (Three-Line / Duplex) Strategy"
reading_id: "08_convertible_bonds"
summary: "安道全's 三线—复式 strategy collapses Chinese-CB position management to three decreasing buy-side PRICE lines (建仓线 / 加仓线 / 重仓线) derived from per-CB 安全—弹性 (safety-elasticity) analysis, paired with a sell-side strong-call-trigger line (typically 130 RMB). Price action reduces to three states: 买买买 / 等等等 / 卖卖卖. The 极简参数分散 (minimal-parameter diversified) variant automates the line-drawing using mechanical-..."
tags: ["convertible-bonds", "china-three"]
citations:
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p096:0056"
    chunk_hash: "f22408dcda44fa71eb9d36f81a519d6133f6eaef0ae499de5aae139642bed57a"
    page_range: [96, 97]
    quote: "‚三线—复式‛答略是《可转债投资魔法书》前两版中‚面 值—高价折扣法‛的升级完善版。"
    edge_type: "defines"
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p148:0088"
    chunk_hash: "c81535ae5edc6cff161b04a711e711924cd313a626eea6c99296d32aa08d6d02"
    page_range: [148, 149]
    quote: "条件单，就是可以提前设定买卖的触収价栺或触収觃则，满足 后自动万单亣易。"
    edge_type: "supports"
  - source_id: "cb_an_daoquan_2014_magic_book_2ed"
    chunk_id: "cb_an_daoquan_2014_magic_book_2ed:p107:0103"
    chunk_hash: "75ce537a06e11edb76094b2cc7e94f82204e0cdc06484f57f6f0d221ccbfb855"
    page_range: [107, 108]
    quote: "第4章 可转债投资三大流派和八项注意 本章介绍可转债投资方法中常见的三大流派和相关的投资基本 知识。"
    edge_type: "supports"
card_hash: "00369b462e9a133115aa60776e43083394acccf77c5f6e7805788becf24124c1"
---
# China Convertible Bond — 三线—复式 (Three-Line / Duplex) Strategy

## Intuition

安道全's `三线—复式` framework collapses China-CB position management
to three buy-side **price** lines plus one sell-side trigger line.
For each candidate CB, the author first does a 安全—弹性 (safety-
elasticity) read of the prospectus and credit profile, then draws
three decreasing buying prices: 建仓线 (where you'd at least start
buying), 加仓线 (where you'd add to the position), and 重仓线
(where, in the author's words, you cannot sleep without buying).
**Source:** 安道全 (2023 3ed) Ch.3 pp.97-115.

```
   conceptual three-line layout (price axis):

      price ↑
            │  CB price too high to start; no action.
            │  ────── 建仓线 (entry) ──────  begin filling cap
            │  ────── 加仓线 (add)  ──────  add the next slice
            │  ────── 重仓线 (heavy)──────  top up the cap, lock
            │  too cheap relative to fundamentals.
```

The 复式 (duplex) overlay pairs this buy-side machinery with a
sell-side line set at the strong-call trigger price (practitioner-
quoted typically near 130 RMB). The resulting decision map has only
three states: below 三线 → 买买买 (keep buying as it falls); above
the strong-call line → 卖卖卖 (take profit / accept conversion);
between → 等等等 (wait; ignore the noise). **Source:** 安道全
(2023 3ed) Ch.5 §10 pp.200-220.

## Definition

For one candidate CB, the framework specifies three components.
**Source:** 安道全 (2023 3ed) Ch.3 pp.97-149.

- **安全—弹性 read.** The author begins with a qualitative score of
  the CB's downside safety (credit / put protection / coupon
  cushion) and upside elasticity (parity, conversion premium, equity
  sensitivity). 安全—弹性双佳 (both good) → use all three lines and
  consider a higher per-CB cap; 弹性差 (poor elasticity) → set the
  lines low or omit; 完全不安全 → use only one or two lines (i.e.,
  buy little or none). **Source:** 安道全 (2023 3ed) Ch.3 pp.97-115.
- **三线 price thresholds.** Three decreasing prices on the CB's own
  trading scale: 建仓线 ≥ 加仓线 ≥ 重仓线. The author derives the
  base from candidate yardsticks (to-maturity value, put-back price,
  to-maturity YTM-implied price), then offsets to the practitioner's
  conviction. Each line is psychological as well as quantitative.
  **Source:** 安道全 (2023 3ed) Ch.3 pp.105-115.
- **Position-cap split.** Each CB has an upper portfolio weight cap
  (the author uses 3% as an example). The cap is split across the
  three lines, typically 1/3 : 1/3 : 1/3 — 安全—弹性双佳 CBs may
  tilt toward 建仓线 (load earlier); 惰性 (sluggish) CBs may tilt
  toward 重仓线 (load only deep). When the price crosses 建仓线 the
  position fills to that line's share; when 加仓线 is crossed, the
  next slice is added; at 重仓线 the position is topped to the cap
  and locked. **Source:** 安道全 (2023 3ed) Ch.3 pp.115-125.

```
   per-CB position fill schedule (cap split 1/3 : 1/3 : 1/3):

        price falls ↓
              │  建仓线 reached → fill the first slice
              │  加仓线 reached → add the second slice
              │  重仓线 reached → top up to cap; lock
              ▼
```

For the portfolio as a whole, the three regions aggregate into
total-portfolio commitment bands: many names in the 建仓 region →
0~1/3 of total capital deployed; many in the 加仓 region → 1/3~2/3;
many in the 重仓 region → 2/3~100%. The total-portfolio bands are
qualitative rather than mechanical because per-CB caps and tilts
differ. **Source:** 安道全 (2023 3ed) Ch.3 pp.118-125.

The 复式 overlay (Ch.5 §10) adds a sell-side line at the strong-
call trigger (typically near 130 RMB), so the decision space
collapses to three states. **Source:** 安道全 (2023 3ed) Ch.5
§10 pp.200-213.

```
       price action map (per CB):

   above strong-call trigger    →  卖卖卖 (sell / take parity)
                                    
   between strong-call and 建仓  →  等等等 (wait; ignore noise)

   below 建仓线                   →  买买买 (fill per cap schedule)
```

**Source:** 安道全 (2023 3ed) Ch.5 §10 pp.200-213.

## Mathematical Reasoning

The framework is **threshold-based**, not optimization-based. The
author's claim is that 三线—复式 acts as a noise filter: most
price action lives between 建仓线 and the strong-call line, in the
等等等 region, so the policy explicitly suppresses 90 %+ of intraday
/ intraweek motion as actionable signal. The trade-offs are
qualitative: a 安全—弹性双佳 CB invites a wider gap between 建仓线
and 重仓线 (loading earlier with smaller increments); a 惰性 CB
invites a narrower gap with smaller total cap. **Source:** 安道全
(2023 3ed) Ch.5 §10 pp.203-204.

The lines are not formula-derived from a closed-form valuation
model. They are practitioner judgments anchored on candidate
benchmarks (the CB's to-maturity value, its put-back price, a YTM-
implied fair price). Once set, the lines remain stable unless the
underlying 安全—弹性 read shifts (e.g., a 下修 announcement changes
the CB's elasticity, or a credit downgrade changes its safety). In
that case the lines are redrawn and the position is rebalanced to
the new schedule. **Source:** 安道全 (2023 3ed) Ch.3 pp.115-149.

## See Also

- [`cb-china-trading-mechanics.md`](./cb-china-trading-mechanics.md#definition)
  — face / lot / T+0 conventions the framework presupposes.
- [`cb-china-call-redemption-rules.md`](./cb-china-call-redemption-rules.md#definition)
  — the strong-call (强赎) mechanics that anchor the sell-side line.
- [`cb-china-forced-redemption-tactics-2020s.md`](./cb-china-forced-redemption-tactics-2020s.md#mathematical-reasoning)
  — the tactical-decision card for the 卖卖卖 state.

## Escalate to Raw When

Open 安道全 (2023 3ed) Ch.3 pp.97-149 directly when the reader needs
the per-CB worked examples that calibrate 建仓线 / 加仓线 / 重仓线
against the candidate yardsticks; the card intentionally omits
specific numeric values per Critical Rule 1. **Source:** 安道全
(2023 3ed) Ch.3 pp.97-149.

Open 安道全 (2023 3ed) Ch.5 §10 pp.200-213 when the reader needs the
duplex (复式) sell-side framing or the 买买买 / 等等等 / 卖卖卖
state map applied to a real CB's K-line chart. **Source:** 安道全
(2023 3ed) Ch.5 §10 pp.200-213.
