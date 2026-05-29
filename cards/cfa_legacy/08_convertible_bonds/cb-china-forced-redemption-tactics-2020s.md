---
schema_version: "cacg.v0"
id: "cb-china-forced-redemption-tactics-2020s"
title: "China Convertible Bond — 复式兌现 Tactics Around the 强赎 Trigger (post-2014 case studies)"
reading_id: "08_convertible_bonds"
summary: "安道全 develops a holder-side tactical discipline around the strong-call (强赎) trigger — '复式兌现' — that treats the trigger price as a reliable cash-out point and treats below-trigger dips as low-cost re-entries. The 2018-2022 weekly retrospectives in Ch.5 are organized as case studies of issuers who pulled the call vs. those who declined (publicly committing not to call), illustrating that under pos..."
tags: ["convertible-bonds", "china-forced"]
citations:
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p040:0022"
    chunk_hash: "6c641188f21c64a9af394b056a2dab99c68937e4b7262594407ed2ac2caeb9ea"
    page_range: [40, 41]
    quote: "2021 年，据浙商证券《宏观和会计视角：为何今年超半数转债 没有强赎》统计，57%的触収可转债公告不强赎"
    edge_type: "defines"
  - source_id: "cb_an_daoquan_2014_magic_book_2ed"
    chunk_id: "cb_an_daoquan_2014_magic_book_2ed:p071:0068"
    chunk_hash: "c0cd048f2f0122d728aff1c012021176e80927b5a894690840c2af944694cf1f"
    page_range: [71, 72]
    quote: "上市公司怎么能够100%保证：无论股市涨跌，都 能让股价超越转股价的130%呢？"
    edge_type: "supports"
card_hash: "6a00003b4ac92fd246777c46eaf2116b7e80b69187a83c1cca3319fdbc9c2b94"
---
# China Convertible Bond — 复式兌现 Tactics Around the 强赎 Trigger (post-2014 case studies)

## Intuition

The mechanics card
[`cb-china-call-redemption-rules.md`](./cb-china-call-redemption-rules.md#definition)
covers the strong-call (强赎) rule. This card covers the
**holder-side tactical** discipline 安道全 develops around that
rule in Ch.5 (周报撷英, weekly-note retrospectives 2018-2022):
how to sell at the strong-call trigger, how to re-enter on
below-trigger dips, and how to keep the discipline through false-
trigger near-misses. The author's framing is summarized as
"在到期价值内（尽量低价）买入，并坚持持有到强赎触収线以上"
("buy within maturity value, hold until above the strong-call
trigger"). **Source:** 安道全 (2023 3ed) Ch.5 pp.171-189.

```
   conceptual playbook timeline:

      price
        │
        │  ─── 强赎触収线 (~130 RMB) ───  → 复式兌现 (SELL)
        │           ↑ ↓
        │     post-sell dip
        │           ↓
        │     etalonge-quantity buy-back below 130 (LOWER COST)
        │           ↓
        │  ─── 建仓 / 三线 region ────  → buy buy buy
        ▼ time
```

## Definition

The author defines **复式兌现** as a holder-side action paired
with the 三线—复式 framework's sell-side line (typically near 130
RMB; see
[`cb-china-three-line-duplex-strategy.md`](./cb-china-three-line-duplex-strategy.md#definition)).
The discipline has three elements. **Source:** 安道全 (2023 3ed)
Ch.5 §10 pp.198-213.

- **Sell on trigger.** When the CB price touches the strong-call
  trigger, the holder treats this as the strategy's pre-committed
  exit. No effort is spent predicting whether the call will be
  invoked; the trigger itself is the action signal. **Source:**
  安道全 (2023 3ed) Ch.5 §10 pp.200-213.
- **Equal-quantity buy-back below trigger.** If the price retraces
  below the trigger after the sell, the holder buys back the same
  quantity at the lower price; the round-trip lowers the cost
  basis for the rest of the strategy. The author labels the
  ordering rule "**先复式兌现, 再等量 (一次或分批) 买回**" — sell
  first, then re-enter equal-quantity. **Source:** 安道全 (2023
  3ed) Ch.5 §10 pp.198-200.
- **Hold through near-misses.** If the price comes close to but
  does not cross the trigger (e.g., touches 129 then retraces),
  the position stays put. The author writes: "持有, 就是对它们
  能够达成强赎触収线价栺, 在概率丆是认可的" — "to hold IS the
  endorsement that this CB will eventually touch the trigger over
  its remaining life." **Source:** 安道全 (2023 3ed) Ch.5
  §10 pp.198-199.

```
   per-CB action map (qualitative):

      price ≥ 强赎触収线          →  复式兌现 (sell)
      price falls back below     →  等量 buy-back (lower basis)
      price between 三线 and trigger →  hold; ignore noise
      price ≤ 三线 (建仓线)        →  fill per cap schedule
```

The author also articulates the strategy's worst-case stance with
"两个不介意" (two non-cares): "不介意满仓以后出现更低价栺，
不介意满仓后出现更好的可转债" — don't mind that a lower price may
appear after you've filled position, don't mind that a better CB
candidate appears after you've committed. **Source:** 安道全 (2023
3ed) Ch.5 pp.162.

## Mathematical Reasoning

The discipline is **rule-based, not probability-weighted**. The
author explicitly rejects forecasting whether the call will be
invoked on this trigger touch; the rule fires the same way
regardless. The author's stated buy-back rule is plain: after the
trigger-touch sell, if the price retraces back below the trigger,
buy back the **same quantity** (one shot or in slices) at the
lower price; the round-trip lowers the cost basis on the
re-acquired CB. The order matters because the buy-back price is
unknown ex ante — the discipline says NEVER to skip the sell on
the chance of a more advantageous re-entry. **Source:** 安道全
(2023 3ed) Ch.5 §10 pp.198-200.

The author's claim about the strategy's robustness is empirical
not theoretical. The Ch.5 retrospective entries (dated 2018-2022)
illustrate the rule applied through multiple CB life-cycles:
straight-shot trigger hits (e.g., 丅一转债, 东财转债, 常熟转债)
where the sell completes cleanly; oscillating cases (e.g., 蓝标
转债, 曙光转债, 隆基转债, 景旺转债) where the round-trip equal-
quantity buy-back lowers the basis multiple times; and
near-misses where the patience to wait pays off later. The author
treats the rule's robustness as a function of the asymmetry
inherent to the Chinese CB market (most CBs eventually touch the
trigger because the structural distribution concentrates on
strong-call exit), not as a function of issuer-specific
forecasts. **Source:** 安道全 (2023 3ed) Ch.5 pp.171-220.

## See Also

- [`cb-china-call-redemption-rules.md`](./cb-china-call-redemption-rules.md#definition)
  — the rule mechanics this card complements with holder-side
  tactics.
- [`cb-china-strong-call-game-theory.md`](./cb-china-strong-call-game-theory.md#intuition)
  — the issuer-side strategic-call equilibrium that explains why
  the trigger is a reliable cash-out anchor.
- [`cb-china-three-line-duplex-strategy.md`](./cb-china-three-line-duplex-strategy.md#mathematical-reasoning)
  — the 三线—复式 framework whose sell-side line this card
  operationalizes.
- [`cb-conversion-feature-mechanics.md`](./cb-conversion-feature-mechanics.md#definition)
  — the conversion mechanics that underpin the sell-at-trigger
  decision.

## Escalate to Raw When

Open 安道全 (2023 3ed) Ch.5 pp.171-298 directly when the reader
needs the specific 2018-2022 周报撷英 case studies (e.g., the
丅一 / 东财 / 常熟 / 蓝标 / 曙光 / 隆基 examples) or the
author's reflections on discipline through individual CB
life-cycles. **Source:** 安道全 (2023 3ed) Ch.5 pp.171-298.

Open 安道全 (2023 3ed) Ch.5 §10 pp.198-213 specifically when
applying the **复式兌现** + **equal-quantity buy-back** mechanics
to a live CB; that section also covers the 三-state action map
this card collapses to summary form. **Source:** 安道全 (2023
3ed) Ch.5 §10 pp.198-213.
