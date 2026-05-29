---
schema_version: "cacg.v0"
id: "cb-brooks-pullback-depth-position-sizing"
title: "Brooks Pullback-Depth Vocabulary for CB-Arbitrage Gamma-Rebalance Position Sizing"
reading_id: "08_convertible_bonds"
summary: "Brooks's High/Low 1-2-3-4 pullback-counting vocabulary supplies execution-timing grammar around an already-approved CB-arbitrage position; the load-bearing position-sizing logic remains Calamos's gamma-rebalance / hedge-ratio cadence, with Brooks naming which successive pullback leg the rebalance traverses without changing the answer."
tags: ["convertible-bonds", "brooks-pullback"]
citations:
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p055:0057"
    chunk_hash: "0ca1a724558b37d6e66b90e371edb2cce9be116fa2e81654e650a7a5535643af"
    page_range: [55, 56]
    quote: "The binomial model allows for stock-price correlated credit spreads as well as stochastic credit spreads in the convertible valuation process."
    edge_type: "defines"
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p274:0302"
    chunk_hash: "4c2aa81b7e409af37cee030d47636432e579616db2dcc6ffe5450b0a56c4a996"
    page_range: [274, 275]
    quote: "Buying put options can also reduce this vega risk to the extent that volatility and equity prices move inversely to each other."
    edge_type: "supports"
  - source_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar"
    chunk_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar:p101:0128"
    chunk_hash: "90b7aca378ab49a5dc56533bb7a6479985260eb8eaa672cc97c736fbd25cfc3a"
    page_range: [101, 102]
    quote: "If there is a trend, even a small one, any pullback that breaks any trendline defines that trend as a leg, and the pullback as another leg."
    edge_type: "supports"
  - source_id: "tpa_brooks_2012_trends"
    chunk_id: "tpa_brooks_2012_trends:p329:0467"
    chunk_hash: "a48a8a5b70d4d94bd6f8b5162ca82c2100f7ed6b1c99a4d21ec6e4d8ff2e397e"
    page_range: [329, 330]
    quote: "They don’t buy the pullback when it finally comes, because the market might be reversing into a bear trend, and the buy setup does not look strong enough."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3403:5134"
    chunk_hash: "65f13607d2f88c321b7d8e5a51d079a1f658be146cc012512d1a5aec7aa89072"
    page_range: [3403, 3404]
    quote: "Technical analysis is a form of security analysis that uses price and volume data, often graphically displayed, in decision making."
    edge_type: "supports"
card_hash: "e5716f616e25131407057e9afe8f80d0a70d92a20c6be1e479d51d04bce3190e"
---
# Brooks Pullback-Depth Vocabulary for CB-Arbitrage Gamma-Rebalance Position Sizing

## Intuition

Once a convertible-arbitrage position is on (long CB + short delta-
hedged equity), the gamma-rebalance cadence is governed by the
position's Greeks: as the underlying share price moves, delta drifts,
and the practitioner periodically rebalances the equity short to
re-center on zero delta. Each rebalance is a forced sell-as-stock-
rises or buy-as-stock-falls trade, which mechanically harvests
realized volatility (the gamma scalp). The CADENCE of these
rebalances — how deep a pullback in the underlying triggers the next
rebalance, how aggressively to scale in versus wait — is a position-
sizing question that Calamos answers using the gamma profile.
Brooks's vocabulary describes WHICH pullback the rebalance is sitting
inside (a first pullback after a trend bar, a deeper second pullback,
etc.), without changing the load-bearing answer. **Source:** Calamos
(2003) §3 pp.40-65.

```
   gamma-rebalance cadence inside a directional move:

       share price ↑
                  │     trend leg up
                  │        ╱╲          ← first pullback
                  │       ╱  ╲╱╲       ← second pullback
                  │      ╱      ╲╱╲    ← third pullback
                  │     ╱          ╲╲   ← deeper pullback
                  │   ╱─────────────────╲╲
                  │
                  │   each pullback marks a rebalance candidate:
                  │     delta drifts as price moves
                  │     gamma profile sets the rebalance bandwidth
                  │
                  └────────────────────────────→ time
```

The chart-grammar overlay records each successive pullback by its
Brooks pullback-count label. The Greeks-side decision (scale into the
hedge at the second pullback versus wait for the third) is a gamma-
profile and risk-tolerance read; Brooks's pullback labels do not
predict the next leg's direction. The CFA L1 weak-form-efficiency canon constrains the
chart-grammar overlay: a count of pullbacks is a function of past
prices and carries no expected-return content beyond the unconditional
benchmark. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

## Definition

The position-sizing cadence has three components — the gamma profile
that bounds the rebalance bandwidth, the entry-cadence vocabulary
(when to scale in versus wait), and the chart-grammar overlay that
names each pullback the rebalance traverses. **Source:** Calamos
(2003) §3 pp.40-65; §9 pp.240-260.

| Claim area | Primary anchor (08 PDF + page span) | Brooks supporting span (chapter + page span) | Allowed use | Forbidden inference |
|------------|--------------------------------------|----------------------------------------------|-------------|---------------------|
| Gamma-rebalance bandwidth | Calamos (2003) §3 pp.40-65 (delta-drift and gamma-profile derivation of rebalance triggers) | Brooks (2009) Ch.4 pp.100-145 (pullback-depth taxonomy) | name each pullback the rebalance traverses with Brooks's High and Low pullback-counting vocabulary | claim the pullback label predicts the rebalance fill |
| Entry cadence (scale-in versus wait) | Calamos (2003) §9 pp.240-260 (trade-execution and entry-discipline framing) | Brooks (2012) Trends Ch.18-20 pp.330-430 (how-to-trade-a-trend pullback-depth cadence) | describe entry cadence as scale-in-at-with-trend-pullback in Brooks vocabulary | infer position-size increments from chart bars instead of from gamma profile |
| Risk-cap on cumulative rebalance | Calamos (2003) §3 pp.40-65 (book-level position-sizing constraint) | Brooks (2009) Ch.4 pp.100-145 (deepest-pullback counting) | describe the deepest-pullback bound with Brooks's deeper-pullback vocabulary | derive the risk cap from chart-pattern recognition alone |

The pullback-depth vocabulary maps onto the gamma-rebalance cadence.
**Source:** Brooks (2009) Ch.4 pp.100-145; Calamos (2003) §3
pp.40-65.

| Brooks term | Gamma-rebalance equivalent | Source anchor (Brooks) | Source anchor (CB-arb) |
|-------------|-----------------------------|------------------------|-------------------------|
| first with-trend pullback (Brooks notation High one / Low one) | first delta-drift increment after the position is on | Brooks (2009) Ch.4 pp.100-145 | Calamos (2003) §3 pp.40-65 |
| second pullback (Brooks notation High two / Low two) | second delta-drift increment; gamma-profile-driven rebalance candidate | Brooks (2009) Ch.4 pp.100-145 | Calamos (2003) §3 pp.40-65 |
| third pullback (Brooks notation High three / Low three) | third delta-drift increment | Brooks (2012) Trends Ch.18-20 pp.330-430 | Calamos (2003) §3 pp.40-65 |
| deeper pullback (Brooks notation High four / Low four) | deepest tolerable delta-drift before risk-cap; trigger for cumulative rebalance | Brooks (2009) Ch.4 pp.100-145 | Calamos (2003) §3 pp.40-65 |
| scale-in cadence | gamma-profile-driven hedge-ratio increment schedule | Brooks (2012) Trends Ch.18-20 pp.330-430 | Calamos (2003) §9 pp.240-260 |

Brooks's pullback-depth count is a chart-grammar description of where
the rebalance is sitting; it is not a sizing rule. Calamos's gamma
profile is the sizing rule. The pullback labels supply names for the
sequence of rebalances, not the cadence itself. **Source:** Calamos
(2003) §3 pp.40-65.

## Mathematical Reasoning

The hedge-ratio drift between rebalances is a function of the
position's gamma, the underlying's realized move, and the
rebalance-bandwidth tolerance. Inside the regime where the gamma
profile is roughly constant (the balanced regime where the embedded
option has not gone deeply in-the-money or out-of-the-money), the
rebalance cadence is approximately periodic in price moves: each
delta-drift increment of fixed size triggers one rebalance. Brooks's
H1/L1/H2/L2 sequence is the chart-grammar overlay onto this
periodicity. **Source:** Calamos (2003) §3 pp.40-65.

**Do not infer:** Brooks's pullback-depth count is execution-context
only; counting the first, second, or third pullback implies no
predictive edge above the unconditional benchmark. The weak-form-efficiency canon
([[pm-market-efficiency-core]]) holds for the count. The load-bearing
rebalance decision is the gamma-profile and risk-tolerance read,
which is documented in [[cb-arbitrage-strategy]] and
[[cb-greeks-delta-gamma-vega]]. This card does not relitigate or
extend the gamma-scalp identity; it only marks the pullback-depth
overlay as descriptive. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

The asymmetric usefulness of pullback-depth vocabulary across the
three CB regimes (balanced, equity-like, distressed) is a structural
property of the gamma profile, not a Brooks distinction. In the
balanced regime, the gamma profile is roughly constant and pullback-
depth counting maps cleanly onto a periodic rebalance cadence. In the
equity-like regime, gamma collapses and the rebalance cadence
degenerates into a near-static long-stock residual; Brooks's
vocabulary still describes the chart, but the rebalance count drops
to one or zero. In the distressed regime, double-signed gamma stress
makes the standard Brooks pullback count operationally noisy because
the gamma sign itself can flip on credit-spread movement.
**Source:** Calamos (2003) §3 pp.40-65; §9 pp.240-260.

Three failure modes are most relevant here. 5-minute scalp setups
conflict with the gamma-rebalance cadence horizon (rebalances are
intraday to multi-day events, not 5-minute events; the Chinese-CB
same-day-CB / next-day-stock settlement asymmetry documented in
[[cb-china-t-plus-zero-arbitrage]] further constrains the
operational cadence). MA/EMA pullback signals confuse the chart-side
pullback count with a moving-average indicator; the gamma profile
uses neither. Measured-move price targets impose a chart-derived
exit projection on a position whose exits are clause-anchored or
unwind-on-trade-thesis-degradation. **Source:** Calamos (2003) §9
pp.240-260.

## See Also

- [`cb-brooks-price-action-guardrails.md`](./cb-brooks-price-action-guardrails.md#intuition) — the supplement-only framing this card inherits.
- [`cb-arbitrage-strategy.md`](./cb-arbitrage-strategy.md#mathematical-reasoning) — the load-bearing gamma-scalp identity that the pullback-depth vocabulary supplies chart grammar for.
- [`cb-greeks-delta-gamma-vega.md`](./cb-greeks-delta-gamma-vega.md#definition) — the Greek primitives that determine the rebalance bandwidth Brooks's pullback labels describe.
- [`cb-china-t-plus-zero-arbitrage.md`](./cb-china-t-plus-zero-arbitrage.md#mathematical-reasoning) — Chinese-CB same-day-CB / next-day-stock settlement asymmetry that further constrains which pullback-depth cadences are operationally executable on Chinese-CB books.
- [`pm-market-efficiency-core.md`](../09_portfolio_management_and_asset_pricing/pm-market-efficiency-core.md#definition) — weak-form-efficiency canon; the pullback count is a function of past prices and carries no expected-return content.

## Escalate to Raw When

Open Brooks (2009) Ch.4 pp.100-145 directly when the reader needs the
full High / Low 1-2-3-4 pullback-counting taxonomy beyond the H1 / L1
shorthand recorded here. **Source:** Brooks (2009) Ch.4 pp.100-145.

Open Brooks (2012) Trends Ch.18-20 pp.330-430 for the how-to-trade-a-
trend pullback-depth cadence; the operational use here is bounded by
the gamma-profile-driven rebalance bandwidth documented in
[`cb-arbitrage-strategy.md`](./cb-arbitrage-strategy.md#definition).
**Source:** Brooks (2012) Trends Ch.18-20 pp.330-430.

Open Calamos (2003) §3 pp.40-65 directly when the reader needs the
canonical gamma-rebalance / hedge-ratio framing this card supplies
chart grammar for. **Source:** Calamos (2003) §3 pp.40-65.

Open Calamos (2003) §9 pp.240-260 for the trade-execution discipline
that anchors the entry-cadence layer. **Source:** Calamos (2003) §9
pp.240-260.
