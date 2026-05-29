---
schema_version: "cacg.v0"
id: "cb-brooks-trend-range-regime-map"
title: "Brooks Trend / Range Regime Map for Chinese-CB Tapes"
reading_id: "08_convertible_bonds"
summary: "Before applying any per-CB strategy, the practitioner classifies the tape's regime — 单边市 (one-sided trend) vs 震荡市 (oscillating range) — using 安道全's 安全—弹性 prospectus + credit + parity read; Brooks's trend-vs-trading-range vocabulary supplies the chart-grammar overlay without predicting the next bar's expected return."
tags: ["convertible-bonds", "brooks-trend"]
citations:
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p088:0051"
    chunk_hash: "efae2eaa7a7657197b05e07210ead6a8fb04db0bd4ca60e29722d10e0e6082e6"
    page_range: [88, 89]
    quote: "动量因子：正股的近 10 日收益率，股票的收益率有延续 原杢的运动斱向的趋势，即过去一段时间里收益率较高 的股票在未杢获得的收益率会高于过去收益率较低的股 票。"
    edge_type: "defines"
  - source_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar"
    chunk_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar:p059:0073"
    chunk_hash: "fa4fa61f90d645839d32462b8fe87606dce391989f11f3fd11c2085ebd018ec9"
    page_range: [59, 60]
    quote: "Bar 4 was a doji bar after two other dojis, and a bar with a tiny body is not a good setup bar when the lows, highs, and closes are trending up."
    edge_type: "supports"
  - source_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar"
    chunk_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar:p147:0184"
    chunk_hash: "e6cf83e3ef9ee4fceac27d4fad24450bf633da1c237014953b7f88c17240dfee"
    page_range: [147, 148]
    quote: "You always want a show of strength before your buy setup."
    edge_type: "supports"
  - source_id: "tpa_brooks_2012_trends"
    chunk_id: "tpa_brooks_2012_trends:p229:0321"
    chunk_hash: "45a0a3aec6cbade8573ef6dd2628ce17d4cd396400628066d317c2bfd887bbc7"
    page_range: [229, 230]
    quote: "As a trend progresses, countertrend moves break the trend lines and usually the breakouts fail, setting up with trend entries."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3403:5134"
    chunk_hash: "65f13607d2f88c321b7d8e5a51d079a1f658be146cc012512d1a5aec7aa89072"
    page_range: [3403, 3404]
    quote: "Technical analysis is a form of security analysis that uses price and volume data, often graphically displayed, in decision making."
    edge_type: "supports"
card_hash: "6e03826949e52dffba337f47e5beadca99cf8a338678d6b3995fc73b0978e503"
---
# Brooks Trend / Range Regime Map for Chinese-CB Tapes

## Intuition

Before applying any per-CB strategy (三线—复式, 双低, 强赎博弈), the
Chinese-CB practitioner needs to classify the current tape: is the CB
in a one-sided 单边市 regime (a directional move dominates) or an
oscillating 震荡市 regime (price moves around a level with no clear
direction)? 安道全's 安全—弹性 framework derives the regime label
from the prospectus + credit + parity read; Brooks's trend-vs-
trading-range vocabulary supplies the chart-grammar overlay. The
label is descriptive — it does not predict the next bar's expected
return. **Source:** 安道全 (2023) §2.6 pp.78-92.

```
   regime map (one CB, holding-period horizon):

       price ↑
             │   单边市 (one-sided / trend regime)
             │        Brooks: trend bar, with-trend pullback,
             │                breakout, channel slope
             │        ╲╲
             │           ╲╲___ price trends in one direction
             │
             │   震荡市 (oscillating / range regime)
             │        Brooks: trading range, range bar,
             │                range mean reversion
             │        ───────  high
             │         ~~~~~~  (price oscillates inside the band)
             │        ───────  low
             │
             └──────────────────────────────────────→ time
```

The CFA L1 weak-form-efficiency canon constrains the interpretive
weight of the regime label. A regime classification based on past
prices is itself a past-price-based label, so the canon asserts that
the label adds no expected-return content beyond what the price
already reflects. The regime label's load-bearing use in Chinese-CB
practice is to select WHICH 安道全 tactic to deploy (三线—复式 inside
震荡市; forced-redemption tactics around the 单边市 strong-call exit;
双低 across the listed universe), not to predict price direction.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

## Definition

The regime-map structure has three components — the regime label
itself, the Brooks vocabulary overlay that maps onto each regime, and
the Chinese-CB-tactic anchor that the label dispatches to. **Source:**
安道全 (2023) §2.6 pp.78-92.

| Claim area | Primary anchor (08 PDF + page span) | Brooks supporting span (chapter + page span) | Allowed use | Forbidden inference |
|------------|--------------------------------------|----------------------------------------------|-------------|---------------------|
| 单边市 (trend) regime identification | 安道全 (2023) §2.6 pp.78-92 (regime distinction inside Chinese-CB practice) | Brooks (2009) Ch.3 pp.60-100 (trends vocabulary) + Brooks (2012) Trends Ch.13-17 pp.230-330 (trend-line and channel grammar) | use Brooks's trend-bar / channel-slope vocabulary to describe a 单边市 tape | claim Brooks's trend setups generate excess return on the next bar |
| 震荡市 (range) regime identification | 安道全 (2023) §2.6 pp.78-92 | Brooks (2009) Ch.5 pp.145-200 (trading-range vocabulary) | use Brooks's trading-range and range-bar vocabulary to describe a 震荡市 tape | claim Brooks's range-trade setups produce alpha for CB-arb books |
| Regime → tactic dispatch | 安道全 (2023) §2.6 pp.78-92 (regime gates which per-CB tactic applies) | Brooks (2009) Ch.3 pp.60-100 (regime-distinction language only) | use the regime label to select the 安道全 tactic family (三线 vs 双低 vs forced-redemption) | infer tactic selection from Brooks alone, independent of 安道全's 安全—弹性 read |

The regime-label vocabulary maps onto the Chinese-CB tactic layer.
**Source:** Brooks (2009) Ch.3 pp.60-100; 安道全 (2023) Ch.3 pp.97-149.

| Brooks term | Chinese-CB regime equivalent | Allowed dispatch target | Source anchor |
|-------------|-------------------------------|--------------------------|---------------|
| with-trend setup | 单边市 inside a directional move (toward strong-call corridor or toward put-back boundary) | 复式兑现 / forced-redemption-tactics on the upside; distressed-workout on the downside | Brooks (2009) Ch.3 pp.60-100; 安道全 (2023) Ch.3 pp.97-149 |
| pullback inside a trend | 单边市 intra-leg consolidation | 三线—复式 entry-cadence discipline at 加仓线 / 重仓线 | Brooks (2009) Ch.4 pp.100-145; 安道全 (2023) Ch.3 pp.115-125 |
| trading range high / low | 震荡市 upper / lower band | 双低 axis selection across the listed universe | Brooks (2009) Ch.5 pp.145-200; 安道全 (2023) §2.6 pp.78-92 |
| range breakout (descriptive) | shift from 震荡市 to 单边市 | re-read 安全—弹性 before re-entering | Brooks (2012) Trends Ch.13-17 pp.230-330; 安道全 (2023) §2.6 pp.78-92 |

The regime label is not a binary predictor; it is a context tag that
the practitioner uses to choose tactics. 安道全 explicitly distrusts
the regime-prediction frame because the 单边市 / 震荡市 boundary is
typically only visible in hindsight on per-CB tapes; the operational
use is to choose a default tactic conditional on the recent label,
then re-evaluate when the label shifts. **Source:** 安道全 (2023)
§2.6 pp.78-92.

## Mathematical Reasoning

The regime classification is a coarsening of the price path into two
categories; coarsening cannot create information that is not already
in the price path. If the past-price information set `I_past` already
reflects all relevant information (weak-form efficiency), then any
function `f(I_past)` such as the regime label is also already
reflected, and `E[ r_next | f(I_past) ] = E[ r_next ]`. The label is
descriptively useful for selecting which tactic to deploy, but it
carries no expected-return content. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.420-441.

**Do not infer:** The regime label is execution-context only; the
single act of labelling a tape as 单边市 or 震荡市 implies no
predictive edge above the unconditional benchmark. The weak-form-
efficiency canon ([[pm-market-efficiency-core]]) holds. The regime
label routes the practitioner to a 安道全 tactic that itself derives
its load-bearing claim from prospectus + credit + parity analysis,
not from chart pattern recognition. See [[cb-china-three-line-duplex-strategy]]
for the tactic the regime dispatch typically lands on inside 震荡市.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

Asymptotic stability of the regime label is itself a property of the
underlying CB's clause structure rather than of Brooks's vocabulary.
A CB whose parity is far below 100 in a distressed regime tends to
stay in a directional 单边市 tape until either a clause-driven event
(下修, 强赎 announcement) or a recovery in the underlying equity
shifts the regime. A CB whose parity orbits near the strong-call
corridor tends to oscillate inside a 震荡市 band defined by the
strong-call upper bound and the holder-put lower bound. The regime
label inherits its stability from those clause-defined boundaries,
not from chart-pattern persistence. **Source:** 安道全 (2023) §2.6
pp.78-92.

The seven failure modes (enumerated on the guardrails card) are
out-of-scope for regime labelling. 5-minute scalp setups conflict
with the holding-period horizon of the label (the regime is meant to
persist over days-to-months, not minutes); always-in positioning
presumes a symmetric long-short tactic library that Chinese CBs do
not support; pre-market gaps do not exist under the call-auction
opening protocol. **Source:** 安道全 (2023) §2.6 pp.78-92.

## See Also

- [`cb-brooks-price-action-guardrails.md`](./cb-brooks-price-action-guardrails.md#intuition) — the supplement-only framing this card inherits; Brooks-as-vocabulary disclaimer.
- [`cb-china-three-line-duplex-strategy.md`](./cb-china-three-line-duplex-strategy.md#intuition) — the canonical tactic that dispatches inside 震荡市 once the regime label is set; Brooks vocabulary supplies the chart-grammar overlay for the 三线 entry cadence.
- [`cb-china-trading-mechanics.md`](./cb-china-trading-mechanics.md#definition) — the call-auction / price-limit rule layer that constrains which Brooks regime vocabulary ports onto the Chinese-CB tape.
- [`pm-market-efficiency-core.md`](../09_portfolio_management_and_asset_pricing/pm-market-efficiency-core.md#definition) — weak-form-efficiency canon; the regime label is a function of past prices and therefore carries no expected-return content.

## Escalate to Raw When

Open Brooks (2009) Ch.3 pp.60-100 directly when the reader needs the
full trend-vocabulary catalog (trend bar, with-trend pullback, channel,
breakout) beyond the regime-distinction vocabulary captured here.
**Source:** Brooks (2009) Ch.3 pp.60-100.

Open Brooks (2009) Ch.5 pp.145-200 when the reader needs the trading-
range vocabulary catalog (range high, range low, range mean-reversion,
range bar) beyond what the term-mapping table here records.
**Source:** Brooks (2009) Ch.5 pp.145-200.

Open Brooks (2012) Trends Ch.13-17 pp.230-330 for the trend-line /
channel framing that augments Brooks (2009) Ch.3; the operational use
here is bounded by the per-CB regime label that dispatches to the
relevant Chinese-CB tactic. **Source:** Brooks (2012) Trends Ch.13-17
pp.230-330.

Open 安道全 (2023) §2.6 pp.78-92 directly for the regime-distinction
framework as the author defines it; the card here records the
vocabulary mapping but not the per-CB calibration the author works
through. **Source:** 安道全 (2023) §2.6 pp.78-92.
