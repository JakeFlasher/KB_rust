---
schema_version: "cacg.v0"
id: "cb-brooks-price-action-guardrails"
title: "Brooks Price-Action Guardrails for Chinese-CB Arbitrage Timing"
reading_id: "08_convertible_bonds"
summary: "Brooks's price-action vocabulary supplies execution grammar (trend bar, trading range, pullback, signal bar) layered on top of a CB thesis already formed from clause structure, valuation, liquidity, and risk controls; it is explicitly NOT an alpha model, not a TA signal authority, and not a predictor of excess return — Brooks names the chart, the CB thesis carries the load."
tags: ["convertible-bonds", "brooks-price"]
citations:
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p274:0302"
    chunk_hash: "4c2aa81b7e409af37cee030d47636432e579616db2dcc6ffe5450b0a56c4a996"
    page_range: [274, 275]
    quote: "The volatility swap allows arbitrageurs a direct means to gain long or short exposure to market volatility."
    edge_type: "defines"
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p055:0057"
    chunk_hash: "0ca1a724558b37d6e66b90e371edb2cce9be116fa2e81654e650a7a5535643af"
    page_range: [55, 56]
    quote: "The flexibility of the binomial model allows for the range of credit assumptions linked to the company’s stock price."
    edge_type: "supports"
  - source_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar"
    chunk_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar:p014:0007"
    chunk_hash: "4c65aeee93b0a08711e3d3da4d62af63b8be0439a4dbdca076ad5c4ac8ee948e"
    page_range: [14, 14]
    quote: "I read charts bar by bar and look for any information that each bar is telling me."
    edge_type: "supports"
  - source_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar"
    chunk_id: "tpa_brooks_2009_reading_price_charts_bar_by_bar:p381:0470"
    chunk_hash: "89245aba784eb87d93ecb54bdb673f54f513c75808ebabd9c6262319eb3faa24"
    page_range: [381, 381]
    quote: "The most difficult part of trading is deciding whether a setup is good enough to warrant placing a trade."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3403:5134"
    chunk_hash: "65f13607d2f88c321b7d8e5a51d079a1f658be146cc012512d1a5aec7aa89072"
    page_range: [3403, 3404]
    quote: "Technical analysis is a form of security analysis that uses price and volume data, often graphically displayed, in decision making."
    edge_type: "supports"
card_hash: "fe0d34b1c1834dbf604cbc2ff6e77dd6ef53a76da5d8b96ca88a955936d851b9"
---
# Brooks Price-Action Guardrails for Chinese-CB Arbitrage Timing

## Intuition

Convertible-arbitrage execution sits one layer below the position
decision. The position decision — long this CB at this size, short
this many shares against it — comes from clause structure, valuation,
liquidity, and risk controls. Once that decision is locked, the
practitioner still has to choose entry timing, hedge-ratio rebalance
cadence, and exit timing on a chart populated by intraday and
intraweek price moves. Brooks's price-action vocabulary supplies
labels for that chart — trend bar, trading range, pullback, signal
bar, regime — without supplying the thesis itself. **Source:** Calamos
(2003) §9 pp.240-260.

```
   what Brooks supplies vs what the CB thesis supplies:

       +----------------------------------------------+
       |  CB thesis (Calamos / 安道全 layer)           |
       |    clause structure, valuation, liquidity,    |
       |    risk controls, position size, stance       |
       |  ------- already decided BEFORE Brooks -------|
       |  Brooks vocabulary overlay                    |
       |    regime label (trend vs trading range)      |
       |    level identification (support / resistance)|
       |    pullback-depth counting                    |
       |    exit-cadence vocabulary (trailing stops)   |
       |  ------- chart-grammar only, no thesis -------|
       +----------------------------------------------+
```

The framing here is descriptive, not predictive. Recording that a CB's
intraday tape shows a Brooks-style pullback after a trend bar adds a
label to a chart; it does not assert that the next bar's expected
return exceeds the unconditional benchmark. The CFA L1 weak-form-
efficiency canon holds — information already in past prices is already
in current prices, and chart-pattern-based timing rules earn zero
expected excess return absent a separate edge — and the cross-linked
efficiently-inefficient framing slots the same point into the active-
management equilibrium, where managers are compensated for liquidity
provision and risk bearing, not for past-price pattern recognition.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

## Definition

The execution-grammar role for Brooks is defined by an explicit
claim-ownership table. Every claim area carries a primary CB anchor
(Calamos for trade-execution / gamma-rebalance language; the Chinese
practitioner anchor in the sibling cards for the per-CB strategy
language); Brooks supplies only the vocabulary overlay. **Source:**
Calamos (2003) §3 pp.40-65; §9 pp.240-260.

| Claim area | Primary anchor (08 PDF + page span) | Brooks supporting span (chapter + page span) | Allowed use | Forbidden inference |
|------------|--------------------------------------|----------------------------------------------|-------------|---------------------|
| Execution-vs-thesis layering | Calamos (2003) §9 pp.240-260 (trade execution sits below position decision) | Brooks (2009) Ch.1 pp.1-30 (price-action vocabulary catalog) | label intraday chart with regime / level / pullback / exit-cadence vocabulary | claim Brooks's chart-pattern recognition generates excess return |
| Trade-management cadence vocabulary | Calamos (2003) §3 pp.40-65 (gamma rebalance) | Brooks (2009) Ch.13 pp.380-428 (trade management framing) | borrow terminology like "scale in at pullback", "trail the stop" to describe an already-decided rebalance schedule | infer position-sizing rules from chart bars instead of from Greek profile |
| Weak-form-efficiency guardrail | CFA L1 (2022) Vol.6/pp.420-441 (no excess return from past-price information) | Brooks (2009) Ch.1 pp.1-30 (vocabulary defined, no profit claim here) | use Brooks's labels as descriptive language for the chart | claim a Brooks-derived timing rule has predictive value, alpha, or excess return above the unconditional benchmark |
| Active-management equilibrium | Pedersen efficiently-inefficient framing (cross-linked, not cited here) | Brooks (2009) Ch.1 pp.1-30 | frame Brooks as a chart-vocabulary layer that does NOT unlock the active-management compensation | claim Brooks's framework constitutes a published CB-arb practitioner edge |

The vocabulary's content side is then enumerated as a term-mapping
table — Brooks term on the left, Chinese-CB execution equivalent on
the right, source anchors on both ends. **Source:** Brooks (2009) Ch.1
pp.1-30; 安道全 (2023) Ch.3 pp.97-149.

| Brooks term | Chinese-CB execution equivalent | Source anchor (Brooks) | Source anchor (Chinese-CB) |
|-------------|---------------------------------|------------------------|----------------------------|
| trend bar | 单边市内的强势日 / 弱势日 | Brooks (2009) Ch.1 pp.1-30 | 安道全 (2023) Ch.3 pp.97-149 |
| trading range | 震荡市 | Brooks (2009) Ch.1 pp.1-30 | 安道全 (2023) Ch.3 pp.97-149 |
| pullback | 回撤 inside the in-progress trend | Brooks (2009) Ch.1 pp.1-30 | 安道全 (2023) Ch.3 pp.97-149 |
| signal bar | the bar Brooks labels as a setup precursor (descriptive only) | Brooks (2009) Ch.1 pp.1-30 | — (no Chinese-CB equivalent; signal-bar taxonomy not in 安道全) |
| trailing stop | 回撤价 / 复式兑现 sell discipline | Brooks (2009) Ch.13 pp.380-428 | 安道全 (2023) Ch.5 pp.171-298 |

The seven failure modes enumerated in `Out of scope:` complete the
vocabulary scope. Each is a Brooks pattern category whose porting into
Chinese-CB execution would either contradict the microstructure
(same-day CB settlement, daily price-limit corridor, no native short-
borrow, clause-anchored sells) or require holding-period assumptions
Chinese CBs do not support. **Source:** Calamos (2003) §9 pp.240-260;
安道全 (2023) Ch.5 pp.171-298.

## Mathematical Reasoning

The reasoning core of the guardrail is the non-arithmetic identity
between vocabulary and expected return. Brooks's framework is a
labeling layer: every bar receives a regime label, every level
receives a support/resistance label, every retracement receives a
pullback label. Labels carry no expected-return content unless paired
with a separately motivated thesis that explains why the label
predicts future cash flows or risk premia. The weak-form-efficiency
canon asserts that for any past-price-based label `L`,
`E[ r_next | past_prices_and_label_L ] = E[ r_next ]`. The label adds
no information beyond what is already in the price. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.420-441.

**Do not infer:** This vocabulary mapping is execution-context only;
the regime / level / pullback / exit-cadence labels imply no
predictive edge above the unconditional benchmark. The weak-form-
efficiency canon ([[pm-market-efficiency-core]]) holds, and the
efficiently-inefficient framing ([[pm-efficient-markets-and-anomalies]])
explains why active-management compensation accrues to liquidity
provision and risk bearing, not to chart-vocabulary recognition.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

Any claim that adding a Brooks-labeled regime filter improves a
Chinese-CB book's risk-adjusted return, hit-rate, peak-to-trough
loss, or arbitrage performance is not supported by this card; such a
claim would require a registered empirical source, and no such
source exists in the project matrix as of the current authoring
round. The supplement-only framing is therefore the only stance the
card admits. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

The same reasoning applies in the opposite direction: a tape that
fails to display Brooks-recognizable structure does NOT contraindicate
a CB position whose thesis derives from clause / valuation / liquidity
analysis. Brooks supplies vocabulary, not a veto. **Source:** Calamos
(2003) §3 pp.40-65.

The asymmetric microstructure between Brooks's reference markets and
Chinese-CB execution rules out the seven failure modes. The Chinese-CB
tape lives under daily price-limit halts and same-day-CB / next-day-
stock settlement asymmetry that Brooks's bar-by-bar setups do not
anticipate; see [[cb-china-trading-mechanics]] for the rule layer.
Always-in positioning presumes symmetric short-borrow availability,
which Chinese CBs do not offer. Measured-move price targets presume a
continuous price-target regime; Chinese-CB exits instead anchor on
clause-defined trigger zones (the strong-call corridor and the holder-
put boundary near the face handle) rather than chart-derived targets.
**Source:** 安道全 (2023) Ch.3 pp.97-149.

## See Also

- [`cb-arbitrage-strategy.md`](./cb-arbitrage-strategy.md#mathematical-reasoning) — the position-decision layer Brooks's vocabulary sits below; Calamos gamma-scalp identity is the load-bearing arbitrage thesis.
- [`cb-china-three-line-duplex-strategy.md`](./cb-china-three-line-duplex-strategy.md#intuition) — 安道全 三线—复式 framework that the Brooks vocabulary supplements (regime / level / exit vocabulary used inside the 三线 + 强赎 line discipline).
- [`cb-china-t-plus-zero-arbitrage.md`](./cb-china-t-plus-zero-arbitrage.md#mathematical-reasoning) — Chinese-CB arbitrage timing under the CB-side same-day settlement asymmetry; the microstructure that constrains which Brooks vocabulary ports.
- [`pm-market-efficiency-core.md`](../09_portfolio_management_and_asset_pricing/pm-market-efficiency-core.md#definition) — weak-form-efficiency canon; the cite-through-showstopper guardrail this card respects.
- [`pm-efficient-markets-and-anomalies.md`](../09_portfolio_management_and_asset_pricing/pm-efficient-markets-and-anomalies.md#intuition) — Pedersen efficiently-inefficient framing for the active-management equilibrium.

## Escalate to Raw When

Open Brooks (2009) Ch.1 pp.1-30 directly when the reader needs the
full bar-by-bar setup taxonomy beyond the regime / pullback / exit-
cadence vocabulary captured here. The card intentionally omits the
fine-grained signal-bar catalogue. **Source:** Brooks (2009) Ch.1
pp.1-30.

Open Brooks (2009) Ch.13 pp.380-428 when the reader needs Brooks's
own trade-management framing (entry-stops, breakeven stops, trail-by-
swing-low) as background; the operational use in Chinese-CB execution
is bounded by the per-CB clause-anchored exit discipline documented in
[`cb-brooks-trailing-exit-duplex-cashout.md`](./cb-brooks-trailing-exit-duplex-cashout.md#mathematical-reasoning).
**Source:** Brooks (2009) Ch.13 pp.380-428.

Open Calamos (2003) §9 pp.240-260 directly when the reader needs the
full trade-execution discussion that anchors the gamma-rebalance /
hedge-cadence layer this card describes via Brooks-flavored language;
Calamos supplies the load-bearing claim, Brooks supplies the
descriptive grammar. **Source:** Calamos (2003) §9 pp.240-260.
