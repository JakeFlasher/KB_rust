---
schema_version: "cacg.v0"
id: "cb-china-double-low-strategy"
title: "China Convertible Bond — 双低 (Double-Low) Strategy"
reading_id: "08_convertible_bonds"
summary: "The 攻守 handbook organizes Chinese CBs on a price-vs-premium plane. The 双低 (double-low) strategy targets the (low price ∩ low premium) quadrant using a 双低数值 = X·price + Y·premium composite score, with a defensive screen exploiting 'bond floor + equity option' to deliver capped downside with re-rating optionality; backtest cited 2017-12-29 → 2020-03-06 with adjustable (X, Y) weights for personal ..."
tags: ["convertible-bonds", "china-double"]
citations:
  - source_id: "cb_gongshou_practical_handbook_1ed"
    chunk_id: "cb_gongshou_practical_handbook_1ed:p064:0058"
    chunk_hash: "e5e3ecbf7f39ceb564b729f086337f4deed090ce2c037eec9d54ed2bd186b777"
    page_range: [64, 65]
    quote: "通过对2017年9月可转债信用申购以来的数据进行回测和分析，可以认为，可转 债网上申购具备“两低两高”的特征——风险低，中签高；投入低，收益高。"
    edge_type: "defines"
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p096:0056"
    chunk_hash: "f22408dcda44fa71eb9d36f81a519d6133f6eaef0ae499de5aae139642bed57a"
    page_range: [96, 97]
    quote: "‚三线—复式‛答略是《可转债投资魔法书》前两版中‚面 值—高价折扣法‛的升级完善版。"
    edge_type: "supports"
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p064:0068"
    chunk_hash: "1a37de4363e5d295e0938ee0afbe696d4490f50342b050bcbc60721473582712"
    page_range: [64, 65]
    quote: "Delta A Delta B Delta C Conversion value Stock Price FIGURE 3.1 Visualizing Delta."
    edge_type: "supports"
card_hash: "a03bc3f0f558579a5ad8ff974f6d69805cef9f67af0228ac7a96c6f42c64251e"
---
# China Convertible Bond — 双低 (Double-Low) Strategy

## Intuition

攻守 organizes Chinese CBs on a price × premium plane, partitioning
candidates into four quadrants: 象限 1 (high price, high premium,
"双高"), 象限 2 (low price, high premium), 象限 3 (low price, low
premium, "双低"), 象限 4 (high price, low premium). 双低 sits in
象限 3: bond-like protection from the low price, equity-like
participation from the low premium. The author classifies the
quadrant as 防守反击的典范 — "the model of defensive
counter-attack" — because either leg alone is too permissive (low
price can mean credit distress; low premium can mean an exhausted
equity story), while the conjunction binds both. **Source:** 攻守
(2020) Ch.4 §7 pp.74-75.

```
   conceptual price × premium quadrants:

           π (premium %)
                ↑
                │ 象限 1: 双高           象限 2: 低价+高溢价
                │ (high P, high π)        (low P, high π)
                │ excluded                 — 丑小鸭 candidates
                │
       ─────────┼─────────────────────────→  P (price)
                │ 象限 4: 高价+低溢价      象限 3: 双低
                │ (high P, low π)          (low P, low π)
                │ — strong call,           — INCLUDE
                │   pre-strong-call         (defensive)
```

## Definition

The **standard 双低** screen ranks the ~200 actively-trading
Chinese CBs by a composite score. **Source:** 攻守 (2020) Ch.4
§7 pp.74-75.

```
   双低数值 = 价格 + 溢价率 × 100
```

The author picks the smallest 5–20 names by 双低数值 as the basket.
The construction excludes high-price names (the price term) and
high-premium names (the premium term) simultaneously. **Source:**
攻守 (2020) Ch.4 §7 pp.74-75.

Rotation rules are stated explicitly. The strategy is **dynamic**:
rebalance either by a time cadence (e.g., weekly) OR by a
threshold on 双低数值 differences. The author's backtest uses a
threshold rotation rule: replace a held name with a candidate when
`(held 双低数值) − (candidate 双低数值) > 5`. The chosen backtest
window is **2017-12-29 → 2020-03-06**; the author reports the 双低
前十指数 (top-10-双低 index) outperformed both the 国证转债 index
and 沪深 300 over that window, while noting that the strategy's
drawdown extreme can exceed the 国证转债 index — the screen
captures upside concentration AND downside volatility. **Source:**
攻守 (2020) Ch.4 §7 pp.75-76.

The author offers a **personalization knob** on the score.
**Source:** 攻守 (2020) Ch.4 §7 pp.75-76.

```
   personalized 双低 score:
   转债价格 × X + 100 × 转债溢价率 × Y
```

with `(X = 1.2, Y = 0.8)` weighting price more heavily (more
defensive: lower-price, higher-premium candidates); `(X = 0.8,
Y = 1.2)` weighting premium more heavily (more aggressive: higher-
price, lower-premium candidates). **Source:** 攻守 (2020) Ch.4
§7 pp.75-76.

## Mathematical Reasoning

The composite score additively combines the two screening
dimensions, so the conjunction is implicit: a candidate with high
`P` OR high `π` (alone) is pushed out of the top of the ranking;
only candidates low on BOTH ranks-low end up in the basket. The
`×100` scaling on premium is a unit-balancing choice that makes
one price-unit comparable to one premium-percentage-point at the
default `(X, Y) = (1, 1)`; the personalized `(X, Y)` lets a user
tilt the conjunction's defensive / offensive bias without rewriting
the screen. **Source:** 攻守 (2020) Ch.4 §7 pp.74-76.

The author closes the section with two practitioner cautions: the
screen uses only price + premium, so it is "two-factor" and
deliberately blind to underlying-stock fundamentals, industry, or
rating. The author recommends layering qualitative filters (正股
质地、所处行业、信用评级、转债条款) on top of the screened
basket. The card preserves the formula surface and the backtest
window but does NOT reproduce the per-name return tables or the
specific weighting calibrations per Critical Rule 1. **Source:**
攻守 (2020) Ch.4 §7 pp.75-76.

## See Also

- [`cb-conversion-premium.md`](./cb-conversion-premium.md#definition)
  — the `π = (P − V)/V` premium definition used as the second leg
  of the screen.
- [`cb-china-trading-mechanics.md`](./cb-china-trading-mechanics.md#definition)
  — face / lot / T+0 conventions the screen presupposes.
- [`cb-china-basket-diversification-strategy.md`](./cb-china-basket-diversification-strategy.md#intuition)
  — the 摊大饼 multi-strategy composite that uses 双低 as one of
  its inputs.

## Escalate to Raw When

Open 攻守 (2020) Ch.4 §7 pp.74-76 directly when the reader needs the
specific per-name return tables and the figure 4-2 + table 4-1
comparing the 双低 前十指数 against the 国证转债 index and 沪深
300 over the cited backtest window (the card intentionally omits
the numeric tables per Critical Rule 1). **Source:** 攻守 (2020)
Ch.4 §7 pp.74-76.

Open 攻守 (2020) Ch.4 §7 pp.75-76 when the reader is tuning the
`(X, Y)` weighting for a specific risk tilt; the author's
defensive vs offensive examples sit at the bottom of p.76.
**Source:** 攻守 (2020) Ch.4 §7 pp.75-76.
