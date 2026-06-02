---
schema_version: "cacg.v0"
id: "cb-brooks-support-resistance-clause-lines"
title: "Brooks Support / Resistance Vocabulary Mapped onto Chinese-CB Clause Levels"
reading_id: "08_convertible_bonds"
summary: "Brooks's support/resistance vocabulary describes Chinese-CB clause-anchored levels — 三线 (建仓线/加仓线/重仓线), the strong-call corridor near 130%, the holder-put boundary near face, and to-maturity / 回售 reference values; Brooks supplies the chart-grammar overlay while the load-bearing level identification remains 安道全's clause + prospectus read."
tags: ["convertible-bonds", "brooks-support"]
citations:
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p114:0067"
    chunk_hash: "e9bdf54c5fc125ef36a47492bf3a208906d991b636130bb7672efb13c84a4b5d"
    page_range: [114, 115]
    quote: "3.6 三线何来？如何设置三线 确定了分析可转债的标准 ‚安全—弹性双原则‛，如何设置 丅线，也就显而易见了。"
    edge_type: "defines"
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p114:0067"
    chunk_hash: "e9bdf54c5fc125ef36a47492bf3a208906d991b636130bb7672efb13c84a4b5d"
    page_range: [114, 115]
    quote: "理解了‚安全—弹性双原则‛，好转债和坏转债的区分，以及后 面的丅线划定、丅线配比等问题的解决，就一气呵成、行亐洿水了。"
    edge_type: "supports"
  - source_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar"
    chunk_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar:p147:0184"
    chunk_hash: "e6cf83e3ef9ee4fceac27d4fad24450bf633da1c237014953b7f88c17240dfee"
    page_range: [147, 148]
    quote: "You always want a show of strength before your buy setup."
    edge_type: "supports"
  - source_id: "tpa_brooks_2012_trading_ranges"
    chunk_id: "tpa_brooks_2012_trading_ranges:p519:0489"
    chunk_hash: "497c27bc10572a5d47a054a56189f04373d8cc594d26dd8f69aef680ee680d1c"
    page_range: [519, 520]
    quote: "There needs to be at least a tiny trend line break between the high 1 and the high 2 to indicate that the trend traders are still active."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3403:5134"
    chunk_hash: "65f13607d2f88c321b7d8e5a51d079a1f658be146cc012512d1a5aec7aa89072"
    page_range: [3403, 3404]
    quote: "Technical analysis is a form of security analysis that uses price and volume data, often graphically displayed, in decision making."
    edge_type: "supports"
card_hash: "c3e6c73177c416f12d6592a24c8aba10f626c50c3dff5724c530d91a8479e661"
---
# Brooks Support / Resistance Vocabulary Mapped onto Chinese-CB Clause Levels

## Intuition

Brooks's chart vocabulary distinguishes support (a level below current
price where buyers are expected to step in) from resistance (a level
above current price where sellers are expected to step in). Chinese-CB
practice already has a four-level clause-anchored support/resistance
taxonomy that 安道全 calls the 三线 set (建仓线 / 加仓线 / 重仓线 on
the buy side) plus the strong-call corridor on the sell side, with
the 回售 boundary and the 到期 / face handle as deeper-support
references. Brooks supplies the chart grammar; the levels themselves
are clause-derived, not chart-derived. **Source:** 安道全 (2023) Ch.3
pp.97-149.

```
   Chinese-CB clause-anchored level stack (price axis):

       price ↑
             │   above strong-call corridor:
             │     sell-side resistance (clause-mandated exit zone)
             │   ────── strong-call upper line (sell ─ 卖卖卖) ──────
             │
             │   middle band (between buy-side 三线 and strong-call):
             │     no clear support / resistance for the CB itself;
             │     wait state (等等等)
             │
             │   ────── 建仓线 (begin filling)               ──────
             │   ────── 加仓线 (add next slice)              ──────
             │   ────── 重仓线 (top up; cannot sleep otherwise)──────
             │
             │   below 三线: clause-anchored deep support:
             │   ────── 回售价 (holder-put strike)             ──────
             │   ────── 到期价值 / face handle                  ──────
             │
             └────────────────────────────────────────────→ time
```

Brooks's support/resistance vocabulary describes WHERE these levels
sit on the chart, not why the levels are load-bearing. The why is
clause-anchored: the strong-call corridor reflects the issuer's
mandate to call when parity persists above the trigger; the 回售
boundary reflects the holder's contractual put-back right; the 三线
levels reflect the practitioner's 安全—弹性 read of how cheap the CB
must be to invite incremental capital deployment. The weak-form-
efficiency canon constrains the predictive use of these labels: a
level identified by past-price action does not imply that buyers will
step in next time the price approaches it. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.420-441.

## Definition

The level taxonomy has six clause-anchored elements, plus the Brooks
vocabulary overlay that supplies the chart-grammar description.
**Source:** 安道全 (2023) Ch.3 pp.97-149; Ch.5 pp.171-220.

| Claim area | Primary anchor (08 PDF + page span) | Brooks supporting span (chapter + page span) | Allowed use | Forbidden inference |
|------------|--------------------------------------|----------------------------------------------|-------------|---------------------|
| 三线 buy-side levels (建仓线 / 加仓线 / 重仓线) | 安道全 (2023) Ch.3 pp.97-149 (per-CB 安全—弹性 derivation of buy-side prices) | Brooks (2009) Ch.5 pp.145-200 (trading-range support vocabulary) | describe each 三线 as a clause-and-fundamentals-derived buy-side support level | claim the chart-pattern formation at the 三线 predicts the next bar's direction |
| Strong-call corridor (sell-side resistance) | 安道全 (2023) Ch.5 pp.171-220 (forced-redemption tactics anchored on the trigger zone) | Brooks (2009) Ch.5 pp.145-200 (range-high / resistance vocabulary) | describe the strong-call corridor as the clause-anchored sell-side resistance | claim the corridor is a Brooks-style resistance line in the price-action sense |
| 回售 holder-put boundary | 安道全 (2023) Ch.3 pp.97-149 (holder-put as a deep-support reference) | Brooks (2009) Ch.5 pp.145-200 (range-low / support vocabulary) | describe the 回售价 as a clause-anchored deep-support level | claim chart-pattern recognition near the 回售 boundary generates an edge |
| 到期 / face handle | 安道全 (2023) Ch.3 pp.97-149 (face / to-maturity as the deepest reference) | Brooks (2012) Trading Ranges Ch.21-23 pp.500-600 (range-anchored level grammar) | describe the face handle as a credit-bounded floor in the absence of default | infer recovery-side or default-side payoffs from chart action near face |

The level mapping then translates Brooks support/resistance grammar
into the Chinese-CB clause vocabulary. **Source:** Brooks (2009) Ch.5
pp.145-200; 安道全 (2023) Ch.3 pp.97-149.

| Brooks term | Chinese-CB clause-anchored equivalent | Source anchor (Brooks) | Source anchor (Chinese-CB) |
|-------------|-----------------------------------------|------------------------|----------------------------|
| support level | 建仓线 / 加仓线 / 重仓线 (buy-side 三线) | Brooks (2009) Ch.5 pp.145-200 | 安道全 (2023) Ch.3 pp.97-149 |
| deep support | 回售价 / 到期 face handle | Brooks (2009) Ch.5 pp.145-200 | 安道全 (2023) Ch.3 pp.97-149 |
| resistance level | strong-call corridor near the conversion-mandate handle | Brooks (2009) Ch.5 pp.145-200 | 安道全 (2023) Ch.5 pp.171-220 |
| range-bound (between support and resistance) | 中间带 inside the 等等等 zone | Brooks (2012) Trading Ranges Ch.21-23 pp.500-600 | 安道全 (2023) Ch.5 pp.171-220 |
| breakout (descriptive) | regime shift from 震荡市 to 单边市 across one of the boundaries | Brooks (2009) Ch.5 pp.145-200 | 安道全 (2023) §2.6 pp.78-92 |

The four lines on the chart map cleanly onto Brooks's range-anchored
vocabulary because each is a horizontal price level, not a sloping
trend channel. This makes the Brooks vocabulary overlay clean for
description and useless for prediction: knowing that the price has
approached the 建仓线 is observationally trivial; the load-bearing
decision (deploy capital here, or wait) is a separate 安全—弹性 read
that this card does not relitigate. **Source:** 安道全 (2023) Ch.3
pp.105-115; Brooks (2009) Ch.5 pp.145-200.

## Mathematical Reasoning

The level taxonomy partitions the CB's price axis into a sequence of
bands separated by clause-anchored lines. Within each band the
practitioner's action is determined by the band's identity (above the
strong-call corridor: sell; between corridor and 建仓线: wait; below
建仓线 in fill order: scale in). The partition is a function of the
clause structure and the 安全—弹性 read, not of the chart's past
behavior. **Source:** 安道全 (2023) Ch.3 pp.115-149.

**Do not infer:** The Brooks support/resistance labels are
execution-context only; identifying a level on the chart implies no
predictive edge above the unconditional benchmark. The weak-form-
efficiency canon ([[pm-market-efficiency-core]]) holds: if the level
is visible in past prices, it is already incorporated into the
current price, and a fresh approach to the level carries no
expected-return content beyond the unconditional benchmark. See
[[cb-china-call-redemption-rules]] for the clause mechanics that make
the strong-call corridor a clause-anchored sell-side level rather
than a chart-pattern resistance. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

The asymmetry between buy-side and sell-side levels is a structural
property of the CB instrument, not a Brooks distinction. The buy-side
三线 are practitioner-derived from a fundamentals read; they shift
when the underlying credit or parity inputs shift. The sell-side
strong-call corridor is contract-derived from the prospectus trigger
clause; it is essentially fixed once the prospectus is issued (apart
from issuer-side 不赎回 announcements that can suspend the trigger).
Brooks's symmetric trading-range vocabulary does not capture this
asymmetry; the Chinese-CB equivalent is a hybrid where the lower
bound is judgemental and the upper bound is contractual. **Source:**
安道全 (2023) Ch.5 pp.171-220.

The seven failure modes (enumerated on the guardrails card) constrain
which Brooks levels port. Measured-move price targets, which Brooks
recovers from past-range height projected past the breakout point,
do not apply to Chinese-CB exits — exits are clause-anchored to the
strong-call corridor and the holder-put boundary. Formal chart
pattern names (wedges, flags, head-and-shoulders, double-tops) name
specific multi-bar shapes that the Chinese-CB practitioner does not
operationalize: the four lines are sufficient. **Source:** 安道全
(2023) Ch.3 pp.97-149.

## See Also

- [`cb-brooks-price-action-guardrails.md`](./cb-brooks-price-action-guardrails.md#intuition) — the supplement-only framing this card inherits.
- [`cb-china-three-line-duplex-strategy.md`](./cb-china-three-line-duplex-strategy.md#definition) — the canonical Chinese-CB use of the 三线 buy-side support stack plus the strong-call sell-side resistance.
- [`cb-china-call-redemption-rules.md`](./cb-china-call-redemption-rules.md#definition) — the clause mechanics that make the strong-call corridor a contract-anchored sell-side level.
- [`pm-market-efficiency-core.md`](../09_portfolio_management_and_asset_pricing/pm-market-efficiency-core.md#definition) — weak-form-efficiency canon; chart-identified support/resistance levels carry no expected-return content.

## Escalate to Raw When

Open Brooks (2009) Ch.5 pp.145-200 directly when the reader needs the
full trading-range vocabulary beyond the support/resistance + breakout
grammar mapped here. **Source:** Brooks (2009) Ch.5 pp.145-200.

Open Brooks (2012) Trading Ranges Ch.21-23 pp.500-600 for the deeper
range-trading framing; the operational use in Chinese-CB execution is
bounded by the per-CB clause-anchored level stack documented in
[`cb-china-three-line-duplex-strategy.md`](./cb-china-three-line-duplex-strategy.md#intuition).
**Source:** Brooks (2012) Trading Ranges Ch.21-23 pp.500-600.

Open 安道全 (2023) Ch.3 pp.97-149 directly for the per-CB 三线
derivation procedure; the card here records the level taxonomy but
not the worked calibration the author performs. **Source:** 安道全
(2023) Ch.3 pp.97-149.

Open 安道全 (2023) Ch.5 pp.171-220 directly for the strong-call
corridor framing the sell-side resistance line emerges from.
**Source:** 安道全 (2023) Ch.5 pp.171-220.
