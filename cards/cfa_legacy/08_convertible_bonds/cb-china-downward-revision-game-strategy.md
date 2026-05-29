---
schema_version: "cacg.v0"
id: "cb-china-downward-revision-game-strategy"
title: "China Convertible Bond — 下修博弈 (Downward-Revision Game) Strategy"
reading_id: "08_convertible_bonds"
summary: "The 攻守 handbook treats 下修博弈 as a CB strategy distinct from the mechanical 下修 clause. It taxonomizes the playbook into three event categories: 被动下修 (forced by approaching put-eligible date), 主动下修 (issuer proactively chooses to reset for capital-efficient conversion), and 特殊事件博弈 (other corporate-action catalysts). Selection criteria focus on candidates near a put window with high non-CB sharehold..."
tags: ["convertible-bonds", "china-downward"]
citations:
  - source_id: "cb_gongshou_practical_handbook_1ed"
    chunk_id: "cb_gongshou_practical_handbook_1ed:p016:0013"
    chunk_hash: "0c90836e80f05cbf3fb35bd9baaf8875ff0e4989b6e6a20ba7e0a2b17c9e98ca"
    page_range: [16, 17]
    quote: "为了确保下修议案能 顺利通过，控股股东往往会在拟下修前清仓可转债，这一点可作为判断可转债是否 会下修的线索。"
    edge_type: "defines"
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p096:0056"
    chunk_hash: "f22408dcda44fa71eb9d36f81a519d6133f6eaef0ae499de5aae139642bed57a"
    page_range: [96, 97]
    quote: "‚三线—复式‛答略是《可转债投资魔法书》前两版中‚面 值—高价折扣法‛的升级完善版。"
    edge_type: "supports"
  - source_id: "cb_an_daoquan_2014_magic_book_2ed"
    chunk_id: "cb_an_daoquan_2014_magic_book_2ed:p107:0103"
    chunk_hash: "75ce537a06e11edb76094b2cc7e94f82204e0cdc06484f57f6f0d221ccbfb855"
    page_range: [107, 108]
    quote: "第4章 可转债投资三大流派和八项注意 本章介绍可转债投资方法中常见的三大流派和相关的投资基本 知识。"
    edge_type: "supports"
card_hash: "c00f76346400d18af1344452ef12faff9a23eff3904fadb60b3e217d06c38a83"
---
# China Convertible Bond — 下修博弈 (Downward-Revision Game) Strategy

## Intuition

The mechanics card
[`cb-china-downward-conversion.md`](./cb-china-downward-conversion.md#definition)
covers the rules: who can trigger 下修, what shareholder vote is
required, how `K_c` is recomputed. This card answers the
orthogonal question: given the rules, when does the issuer
actually invoke 下修, and how do investors position into the
event? 攻守 classifies 下修 events by who is driving and why,
yielding a three-category taxonomy with specific candidate-
selection criteria per category. **Source:** 攻守 (2020) Ch.4
§6 pp.71-73.

```
   conceptual event taxonomy:

      ┌────────────────────────────────────┐
      │  被动下修 (passive / forced):       │
      │    avoid put-back trigger          │
      │    avoid additional put-back       │
      ├────────────────────────────────────┤
      │  主动下修 (proactive):              │
      │    push-to-conversion              │
      │    allottee self-rescue            │
      ├────────────────────────────────────┤
      │  特殊事件博弈 (special-event game): │
      │    e.g., 减资 (capital reduction)   │
      └────────────────────────────────────┘
```

## Definition

The author enumerates three event categories with explicit
candidate-selection criteria for each. **Source:** 攻守 (2020)
Ch.4 §6 pp.71-73.

- **被动下修 — avoid put-back (避免回售).** Issuer reduces `K_c`
  to keep conversion value above the put-back trigger. Candidate
  filter: ≥ 4 years since listing AND conversion value < 70 RMB.
  **Source:** 攻守 (2020) Ch.4 §6 pp.71-72.
- **被动下修 — avoid additional put-back (触发附加回售条款).**
  Triggered when the issuer changes the use of fundraised
  capital; investors gain a put-back right tied to that change.
  Issuer often pre-empts by revising `K_c` aggressively.
  **Source:** 攻守 (2020) Ch.4 §6 pp.72.
- **主动下修 — push-to-conversion (促使转股).** Some issuers
  revise even without put-back pressure, signaling strong
  intent. Bank CBs with PB > 1 are the prototypical case (bank
  must convert to top up regulatory capital). Joint candidate
  filter: ①strong stated revision intent; ②stock price below the
  下修 threshold; ③正股 PB > 1, ideally > 1.3; ④controlling
  shareholder selling its CB position to clear voting-rights
  conflicts. **Source:** 攻守 (2020) Ch.4 §6 pp.72.
- **主动下修 — allottee self-rescue (配售方自救).** Underwriters
  or controlling shareholders end up over-allocated at issuance
  when the CB trades long-term below face; the issuer revises to
  lift price above face so the over-allocation can exit at a
  profit. Joint candidate filter: ①CB long-term below face;
  ②controlling shareholder OR underwriter holds ≥ 20% of the
  outstanding CB. **Source:** 攻守 (2020) Ch.4 §6 pp.72.
- **特殊事件博弈 — special-event game.** Examples include
  corporate **减资 (capital reduction)** invoking the put-back
  right under PRC Company Law Article 177 (within 45 days of
  the reduction announcement). The author cites the 2019 洪涛
  + 永鼎 cases where an outside CB holder triggered the
  put-back-right negotiation that forced the issuer to revise
  `K_c` downward to settle. **Source:** 攻守 (2020) Ch.4
  §6 pp.73.

```
   per-category candidate filter (qualitative):

      被动 — avoid put-back:        ≥4 yr listed & V < 70 RMB
      被动 — additional put-back:   issuer changed use-of-funds
      主动 — push-to-conversion:    bank w/ PB>1; intent signals
      主动 — allottee self-rescue:  ≥20% holdings + below face
      特殊事件:                     减资 (Co. Law §177) etc.
```

## Mathematical Reasoning

The strategy is a **rule-based event-classification scheme**: the
author enumerates which issuer behaviors precede a 下修 announcement
and which candidate features identify a CB whose issuer is on the
edge of one of those behaviors. The "math" lives in the
classification rules, not in any closed-form probability or payoff
formula — the cited pages do not state a `p_revise` or an
expected-return model. **Source:** 攻守 (2020) Ch.4 §6 pp.71-73.

The 被动 category groups events the issuer cannot avoid: the
put-back arithmetic of the prospectus forces a `K_c` reduction
when the conversion-value falls below the put-back trigger, and
the use-of-funds change adds a second put-back right that the
issuer pre-empts by revising. The 主动 category groups events
where the issuer initiates: a bank with PB > 1 needs the conversion
to top up regulatory capital; an over-allocated underwriter or
controlling shareholder needs the CB above face to exit. The
特殊事件 category groups corporate actions (e.g., 减资 capital
reductions) where outside CB holders assert PRC Company Law
Article 177 put-back rights and the issuer settles via 下修.
**Source:** 攻守 (2020) Ch.4 §6 pp.71-73.

The card preserves this taxonomy and the per-category candidate
filters but omits the per-CB numerical case-study returns per
Critical Rule 1. **Source:** 攻守 (2020) Ch.4 §6 pp.71-73.

## See Also

- [`cb-china-downward-conversion.md`](./cb-china-downward-conversion.md#definition)
  — the mechanics card describing rules and shareholder vote.
- [`cb-china-downward-reset-signaling.md`](./cb-china-downward-reset-signaling.md#intuition)
  — the partial-reset signaling-game payoff structure separating
  passive (被动) from active (主动) issuer behavior.
- [`cb-conversion-premium.md`](./cb-conversion-premium.md#definition)
  — the `π = (P − V)/V` premium that this event re-anchors.

## Escalate to Raw When

Open 攻守 (2020) Ch.4 §6 pp.71-73 directly when the reader needs the
specific worked case studies the author cites (江银 + 无锡 +
常熟 + 洪涛 + 永鼎 + 蓝思) or the per-category filter conditions
applied to a real prospectus. **Source:** 攻守 (2020) Ch.4
§6 pp.71-73.

Open 安道全 (2014) §4-§5 pp.100-200 when the reader needs the
mechanics of `K_c` recomputation under the prospectus's stated
revision rule (this card focuses on event-classification, not
arithmetic). **Source:** 安道全 (2014) §4-§5 pp.100-200.
