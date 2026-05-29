---
schema_version: "cacg.v0"
id: "cb-china-basket-diversification-strategy"
title: "China Convertible Bond — 摊大饼 (Basket-Diversification) Strategy"
reading_id: "08_convertible_bonds"
summary: "摊大饼 (basket-diversification) is 攻守's multi-strategy composite for Chinese retail-CB investors anchored on '三个尽可能' — max strategy types, max name count, single-name ≤ 5% — operated under 筛选分散、不做预测、低频轮动、小步调仓 principles; the defensive intuition rests on long-term strong-call concentration plus short-term unforecastability."
tags: ["convertible-bonds", "china-basket"]
citations:
  - source_id: "cb_gongshou_practical_handbook_1ed"
    chunk_id: "cb_gongshou_practical_handbook_1ed:p083:0077"
    chunk_hash: "e7b589d7f6ee7a9f3bd082f4a70b0a62cddc566b09de4620813dc7307a6f9f04"
    page_range: [83, 84]
    quote: "该策略的精要是“三个尽可能”——包含策略类型尽可能多，可转债数量尽可 能多，单只可转债占比尽可能小。"
    edge_type: "defines"
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p114:0067"
    chunk_hash: "e9bdf54c5fc125ef36a47492bf3a208906d991b636130bb7672efb13c84a4b5d"
    page_range: [114, 115]
    quote: "3.6 三线何来？如何设置三线 确定了分析可转债的标准 ‚安全—弹性双原则‛，如何设置 丅线，也就显而易见了。"
    edge_type: "supports"
card_hash: "4b3aff3e04f13894e33aa3a68551c0e069da33793f5ea169a62f7175ad0e7720"
---
# China Convertible Bond — 摊大饼 (Basket-Diversification) Strategy

## Intuition

摊大饼 ("spreading the pancake") is 攻守's signature
multi-strategy composite for Chinese retail-CB investors. Rather
than betting on a single screen — low price, low premium, low
double-low score, high rating, high YTM — the author folds many
screens into one combined basket. The defensive intuition rests on
two structural cognitions the author makes explicit: long-term,
the vast majority of Chinese CBs exit via strong-call (so the
"final-state" distribution is concentrated, not idiosyncratic);
short-term, which specific CB will rally next is essentially
unforecastable. A wide, diversified basket exploits the first
cognition while accepting the second. **Source:** 攻守 (2020) Ch.4
§9 pp.83-84.

```
   conceptual basket sourcing (qualitative):

      ┌───────────────────────────┐
      │ 低价 (low price)          │ ── pick names ─→ ┐
      │ 低溢价 (low premium)      │ ── pick names ─→ │
      │ 双低 (double-low)         │ ── pick names ─→ │ pooled basket
      │ 高评级 (high rating)      │ ── pick names ─→ │ (excludes 双高:
      │ 高债底 / 高YTM           │ ── pick names ─→ │  high-price +
      └───────────────────────────┘                    └  high-premium)
```

## Definition

The strategy's core is summarized as **三个尽可能** ("three
as-many-as-possibles"). **Source:** 攻守 (2020) Ch.4 §9 pp.83-84.

- **策略类型尽可能多 (as many strategy types as possible).** Mix
  low-price (象限 2), low-premium (象限 4), 双低 (象限 3), high-
  rating, high-bond-floor / high-YTM screens — the author EXCLUDES
  象限 1 (双高: high-price ∩ high-premium) candidates. **Source:**
  攻守 (2020) Ch.4 §9 pp.83-84.
- **可转债数量尽可能多 (as many CBs as possible).** Count guidance
  scales with capital: roughly 10-20 names for ~10万 RMB, 30-50
  for ~100万 RMB, 60+ for ~1000万 RMB. **Source:** 攻守 (2020)
  Ch.4 §9 pp.83-84.
- **单只可转债占比尽可能小 (single-name weight as small as possible).**
  Single-name weight generally ≤ 5% of the basket. **Source:**
  攻守 (2020) Ch.4 §9 pp.83-84.

The author articulates four governing principles that operate on
top of "三个尽可能": **筛选分散、不做预测、低频轮动、小步调仓**
("screen-and-diversify; do not forecast; low-frequency rotation;
small-step rebalancing"). The empirical motivation: the author
cites the prior-decade statistic that ~90 % of Chinese CBs
successfully completed strong-call exits and ~95 %+ touched 130
RMB at some point, making the long-term distribution highly
concentrated even when short-term price action is not. **Source:**
攻守 (2020) Ch.4 §9 pp.83-84.

```
   per-name fill schedule (qualitative):

      capital range   →  basket count guidance
      ────────────────────────────────────────
      ~10万 RMB       →  10-20 names
      ~100万 RMB      →  30-50 names
      ~1000万 RMB     →  60+ names
      single-name weight cap: ~5%
```

## Mathematical Reasoning

The strategy is **rules-based, not optimization-based**. The author
does not derive a Markowitz-style covariance optimum; he picks
weights small enough that single-name idiosyncratic risk is
attenuated by the multi-strategy + multi-name composite. The
diversification arithmetic rests on two structural properties of
the Chinese CB universe rather than on independence assumptions:
(i) the long-term distribution concentrates near strong-call exit
(≥ 130 RMB), so the basket's terminal-state risk is dominated by
how many names reach that exit and how quickly; (ii) the short-
term distribution is wide and unforecastable, so a wide basket
captures whatever names rally without needing to predict which.
**Source:** 攻守 (2020) Ch.4 §9 pp.83-84.

The author presents a worked **simulated portfolio** as concrete
evidence: ~30万 RMB initial capital, 20-30 names, single-name weight
≤ 5%, built on 2019-04-30, tracked through 2020-03-06. Over that
window the simulated portfolio rebalanced through strong-call
exits (e.g., 冰轮 / 平银 / 绝味 / 通威), cash-substitute take-
profit on G三峡EB1, price-spike sells on 尚荣 / 模塑, several
rotation swaps, and additions of new names (e.g., 华通 / 深南 /
希望 / 孚日 / 鸿达 / 新莱). The author reports the backtested
return materially exceeded 沪深 300 over the same window — but the
empirical evidence in the cited pages ends at 2020-03-06 and does
not cover the 2020-2022 forced-redemption wave. **Source:** 攻守
(2020) Ch.4 §9 pp.84-86.

Author's stated takeaway: the strategy combines OBJECTIVE rules
(the three "as-many-as-possible" filters, the per-name cap, the
低频轮动 cadence) with SUBJECTIVE human judgement (the author
prefers human-supervised selection over fully-automated data-only
rules). The card preserves the rules surface but omits all numeric
calibrations beyond the explicitly-stated capital-scale guidance
and the 5 % weight cap per Critical Rule 1. **Source:** 攻守 (2020)
Ch.4 §9 pp.85-86.

## See Also

- [`cb-china-double-low-strategy.md`](./cb-china-double-low-strategy.md#definition)
  — the 双低 screen that is one input to a 摊大饼 basket.
- [`cb-investor-clientele.md`](./cb-investor-clientele.md#intuition)
  — the retail-clientele identification that places 摊大饼 in
  context.
- [`cb-arbitrage-strategy.md`](./cb-arbitrage-strategy.md#intuition)
  — the contrasting US-style delta-hedged trade (摊大饼 is not an
  arbitrage in the Calamos sense).
- [`cb-relative-value-screens.md`](./cb-relative-value-screens.md#intuition)
  — the desk-side Calamos-tradition cross-issuer fair-value / vol /
  credit-equity multifactor screens that 摊大饼 explicitly does NOT
  replicate (retail basket discipline vs professional arbitrage).

## Escalate to Raw When

Open 攻守 (2020) Ch.4 §9 pp.83-86 directly when the reader needs the
specific name-by-name rebalancing decisions in the worked simulated
portfolio (the card summarizes but does not enumerate the trade
log). **Source:** 攻守 (2020) Ch.4 §9 pp.83-86.

Open 攻守 (2020) Ch.4 §9 pp.83-84 when the reader needs the
author's stated empirical motivation for the strategy (the
prior-decade strong-call exit statistic, the ≥ 130 RMB touch
statistic). **Source:** 攻守 (2020) Ch.4 §9 pp.83-84.
