---
schema_version: "cacg.v0"
id: "cb-china-t-plus-zero-arbitrage"
title: "Chinese CB Conversion-Arbitrage Strategy Regimes"
reading_id: "08_convertible_bonds"
summary: "Zhang+Feng (SJTU Antai, 2014) decompose Chinese onshore convertible-bond conversion-arbitrage into two regimes by whether the underlying stock is shortable via the 融资融券 framework: (1) shortable underlying → delta-neutral CB-long / stock-short hedge captures the mean-reverting pricing-deviation (定价偏差); (2) non-shortable → T-day CB-buy + same-day conversion + T+1 stock-open sell. Both rely on thr..."
tags: ["convertible-bonds", "china-t"]
citations:
  - source_id: "china_cb_zhang_feng_strategy_2014"
    chunk_id: "china_cb_zhang_feng_strategy_2014:p001:0000"
    chunk_hash: "22f2191ec135c4b19e64ee8d50529b74d535cd5b2e4417da5ea6ce4a75593db8"
    page_range: [1, 2]
    quote: "根据模型的 假设前提和每个关键时点的最优策略，建立理论模型，并构建了定价偏差指标，采用均值回 复模型以及可转债股性价值指标，深入分析不同条款下的可转债投资空间。"
    edge_type: "defines"
  - source_id: "china_cb_zhang_feng_strategy_2014"
    chunk_id: "china_cb_zhang_feng_strategy_2014:p012:0006"
    chunk_hash: "b07f0eed60c00924c6b2f939489a5d765ce986962e063d256162c10bd31ad7b1"
    page_range: [12, 13]
    quote: "未融券套利收益率=(次日股票开盘价—转股平价)/转股平价—交易成本费用率"
    edge_type: "supports"
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p032:0018"
    chunk_hash: "186acff34a5dd1161006a058ea42934a3eac982b9c7d06804c85eec252be2c51"
    page_range: [32, 33]
    quote: "①在转股期内，如果公司股票在仸意违续三十个交易日中至少 十五个交易日的收盘价栺不低于当期转股价栺的 130%（含 130%）；"
    edge_type: "supports"
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p275:0303"
    chunk_hash: "83ac2b3b1924fe0b0b692bd41225a92adf896bf13cd8d3fc19240e3e76f07e5b"
    page_range: [275, 276]
    quote: "Each contract unit pays $250 for every 1 point in NDX volatility."
    edge_type: "supports"
card_hash: "8806442b0c9cc00228893db899594e9a9e48d1a7e831ac4ca7fdd5adaa54de09"
---
# Chinese CB Conversion-Arbitrage Strategy Regimes

## Intuition

Zhang+Feng (2014) decompose Chinese onshore convertible-bond
investment strategies into two regime cases by whether the
underlying stock is shortable via the 融资融券 (margin financing
and short-borrow) framework, anchored on a mean-reverting-pricing-
deviation entry premise. The shortable case is a T-day-buy +
same-day-convert + 融券-short-stock structure with the short leg
repaid by the converted shares on T+1; the non-shortable case is
a T-day-buy + same-day-convert + T+1-stock-sale directional
structure with a threshold-`λ` gating signal. The China-onshore
SSE/SZSE rule-layer T+0 turnover convention (codified in
[cb-china-csdc-settlement-mechanics](./cb-china-csdc-settlement-mechanics.md)
per SZSE §五) permits the CB-side same-day entry; the stock-side
action is structurally T+1 in both cases because A-share equity
settlement is T+1 even when the CB side trades T+0. **Source:**
Zhang+Feng (2014) §三 pp.6-12 (two strategy cases + 融资融券 +
T+1 timing).

```
   Chinese CB conversion-arbitrage strategy regimes (Zhang+Feng 2014)

   Mean-reversion entry premise: large spread between observed CB
   price P_B and a fundamental-value proxy (股性价值 indicator)
   triggers a candidate arbitrage entry, gated by clause-structure
   filter (no-put-back AND no-mandatory-下修 AND no-dividend-
   conversion-adjustment) which is the empirical filter Zhang+Feng
   apply for reliable mean reversion.
                                |
                                v
        +-----------------------+-----------------------+
        |                                               |
        v                                               v
   Strategy A (可以卖空 / shortable)        Strategy B (不可以卖空 / non-shortable)
   - T-day close: buy CB at P_B                  - T-day (spread > λ): buy CB
   - same day: convert at strike P               - same day: convert at strike P
   - same day: 融券 short stock at P_s           - T+1 open: sell stock at P_s
     (LOCKS stock sale at T-day P_s;           - directional P&L: q*P_s_new-P_B
      no overnight risk)                       - risk: T+1 overnight gap on P_s
   - T+1 open: repay 融券 with                     (no 融券 leg available;
     converted shares                              underlying outside 融资融券 list)
   - P&L: (q * P_s - P_B) - 融券 cost
   - 融券 daily rate (paper example): 0.024%
   - residual risks: 融券 cost +
     transaction cost + liquidity/impact +
     CB availability + trading-rule limits
```

**Source:** Zhang+Feng (2014) §三 pp.6-12 (Strategy A 可以卖空 +
Strategy B 不可以卖空 + clause-structure filter).

## Definition

Zhang+Feng (2014) define the strategy framework through four
analytical components. **Pricing-deviation indicator**: the spread
between the observed CB price `P_B` and the paper's theoretical
Monte-Carlo-based CB value is the entry-trigger statistic. Large
positive deviations identify under-priced CBs relative to the
theoretical model. **Mean-reverting framework**: Zhang+Feng adapt
the mean-reverting framework to the CB-spread context, with the
股性价值 (equity-value indicator) as the fundamental-value proxy.
The arbitrage entry premise is that CB price tends to mean-revert
toward the 股性价值 indicator for cohorts that pass the clause-
structure filter. **融资融券 / shortable-vs-non-shortable
dimension**: the strategy decomposes by whether the underlying
stock is in the 融资融券 standard list. Zhang+Feng's empirical
regression on the cohort observes a positive association between
融资融券 membership and mean-reversion appearance, with the
regression coefficient `B_1 = 0.2471` at 90% significance per the
paper's text — this is the paper's empirical claim, NOT a model
implication. **Clause-structure filter**: Zhang+Feng filter the
cohort to CBs that simultaneously satisfy (a) no-put-back-clause
AND (b) no-mandatory-special-downward-revision-clause AND (c) no-
dividend-triggered-conversion-price-adjustment. Cohorts outside
this filter exhibit weaker mean reversion.
**Source:** Zhang+Feng (2014) §二-§三 pp.4-12.

## Mathematical Reasoning

The Strategy A (`可以卖空`) construction follows the paper's
T-day-buy + same-day-convert + 融券-short-stock + T+1-repay
mechanic. **Source:** Zhang+Feng (2014) §三 pp.7-9.

```
   Strategy A (可以卖空 / shortable case) — Zhang+Feng (2014) §三:

   T-day close:   buy 1 unit CB at P_B
                  convert into q = F/P shares
                  same day: 融券 short q shares of underlying at P_s
                  net cash flow: q * P_s - P_B   (long the spread)
   T+1 open:      repay 融券 with the q converted shares
                  no further cash flow (融券 cost already accrued)

   Arbitrage P&L per unit face:
     P&L_A = (q * P_s - P_B) - 融券_cost
     where 融券_cost = q * P_s * r_securities * dt
     r_securities (paper example): 0.024% per day

   Key risk-locking property (per C-14 §三):
     Same-day 融券 shorting at T-day P_s LOCKS the effective
     stock sale price at the T-day level. The paper states:
     "由于卖空交易机制，可以将股票卖出价格锁定在当天的正股价格，
      而不必承担股价隔夜的风险" — the short mechanism locks the
     stock sale price at the same-day price and thereby relieves
     the strategy from bearing overnight stock-price risk.
     For shortable-CB arbitrage with available short-borrow, the
     paper observes 套利空间 (arbitrage spread) AS the return,
     net of 融券 cost and transaction cost.

   Residual risks per C-14 §四 practical-risk warning:
     - 融券 cost (the daily 融券 rate must be covered by the
       arbitrage spread)
     - transaction cost (brokerage, exchange fees)
     - liquidity / market impact on the underlying stock leg
     - CB availability (Zhang+Feng note difficulty buying CBs
       in sufficient size on the secondary market)
     - trading-rule constraint risk (regulatory / exchange-rule
       gating that may restrict the strategy)
```

The Strategy B (`不可以卖空`) construction follows the paper's
T-day-buy + same-day-convert + T+1-stock-sale mechanic with a
threshold-gating signal. **Source:** Zhang+Feng (2014) §三
pp.9-12.

```
   Strategy B (不可以卖空 / non-shortable case) — Zhang+Feng (2014) §三:

   Entry signal:  T-day close: arbitrage spread (q * P_s - P_B) > threshold λ
                  AND CB clause-structure filter passes
   T-day close:   if signal: buy 1 unit CB at P_B
                  same day: convert into q = F/P shares
   T+1 open:      sell q converted shares at P_s_new
                  P&L_B = q * P_s_new - P_B

   Risk:  T+1 overnight gap on P_s (no 融券 leg to lock the spread);
          if P_s_new < P_s by more than the entry-spread cushion, the
          strategy loses; the threshold λ is set to make this
          expected-positive across cohort.
```

The key analytical content of the C-14 paper is the
**clause-structure filter** (no-put-back AND no-mandatory-下修
AND no-dividend-conversion-adjustment) — CBs outside this filter
exhibit weaker mean-reversion and the arbitrage entry premise
breaks down. Cohorts inside the filter, the paper argues, are
the candidates where the pricing-deviation indicator reliably
mean-reverts. **Source:** Zhang+Feng (2014) §三 pp.9-12.

The **enabling rule-layer context** is the SSE/SZSE T+0 turnover
convention (codified per SZSE §五 / SSE compilation §三(一); see
[cb-china-csdc-settlement-mechanics](./cb-china-csdc-settlement-mechanics.md)).
T+0 permits the CB-side same-day buy + convert flow; it does NOT
extend to the stock-side which is T+1 under standard A-share
settlement. Both Strategy A and Strategy B inherit this T+1
stock-side timing structurally. **Source:** SZSE rule (2022 final)
§五 pp.2-3 (T+0 turnover on CB side); Zhang+Feng (2014) §三
pp.6-12 (T+1 stock-side timing in both strategy cases).

The **international CB-arbitrage comparison context** (NOT a
Zhang+Feng claim, separately cited for educational completeness):
international CB arbitrage as treated by DeSpiegeleer et al.
(2014) §3.1-§3.3 + Calamos (2003) §11 emphasizes intraday
delta-neutral hedging + gamma-scalping P&L capture under
positive embedded-option gamma. The Chinese-onshore Zhang+Feng
strategies do NOT use this framework — they are static T/T+1
conversion-arbitrage cases anchored on the pricing-deviation +
mean-reversion premise rather than continuous delta-rehedging.
The two frameworks share the abstract long-CB + short-stock
structure (Strategy A) but differ in timing horizon (international
intraday-continuous vs Chinese T/T+1) and in entry premise
(international model-implied delta vs Chinese pricing-deviation
mean-reversion). **Source:** DeSpiegeleer et al. (2014) §3.1-§3.3
pp.95-130 (international delta-neutral CB arbitrage); Calamos
(2003) §11 pp.260-300 (gamma-scalping practitioner description).

Asymptotic / regime behaviour of the strategy surface follows
three patterns per Zhang+Feng (2014) §三-§四 pp.6-15.
Mean-reversion-success regime: the pricing-deviation
mean-reverts within the holding horizon; Strategy A captures
the locked spread `(q*P_s - P_B)` net of 融券 cost (the paper
observes that the arbitrage spread IS the return when 融券 is
available because shorting locks the stock sale price);
Strategy B captures the directional spread net of any T+1
overnight-gap realization on `P_s`. 融券-cost-dominated regime
(Strategy A): if the arbitrage spread is small relative to the
0.024%/day 融券 financing cost across the holding period, the
strategy turns negative — this is the regime where 融券 cost
exceeds the spread Strategy A is locking. T+1-overnight-gap
regime (Strategy B): `P_s` moves adversely overnight between
T-day close and T+1 open; Strategy B has no 融券 leg to
neutralize the move and is fully exposed; if the gap exceeds
the entry-spread cushion (the threshold-λ headroom), Strategy B
turns negative — this is the regime Strategy B's λ threshold is
designed to keep expected-positive across the cohort. Clause-
filter-violating regime: CBs outside the clause-structure filter
exhibit weaker mean-reversion empirically and the strategy's
entry premise breaks down; this is the empirical cohort
Zhang+Feng exclude from their candidate-investment list.
**Source:** Zhang+Feng (2014) §三-§四 pp.6-15.

## See Also

- [`cb-arbitrage-strategy.md`](cb-arbitrage-strategy.md) — generic international CB arbitrage framework (delta-hedge construction + gamma-scalping mechanics) which is conceptually adjacent to but mechanically distinct from the Chinese-onshore T/T+1 Zhang+Feng strategies
- [`cb-china-trading-mechanics.md`](cb-china-trading-mechanics.md) — base China-onshore CB trading mechanics that these strategy regimes operate within
- [`cb-china-csdc-settlement-mechanics.md`](cb-china-csdc-settlement-mechanics.md) — rule-layer codification of the T+0 same-day round-trip CB turnover that permits the CB-side same-day entry in both Zhang+Feng strategies (the stock-side action remains structurally T+1 in both cases)
- [`cb-greeks-delta-gamma-vega.md`](cb-greeks-delta-gamma-vega.md) — delta + gamma + vega definitions for the international-context comparison paragraph

## Escalate to Raw When

Open Zhang+Feng (2014) directly for the China-specific academic
treatment of CB investment strategy under 融资融券 short-borrow
constraints + mean-reversion + clause-structure filter: §一
introduction + clause framework (pp.1-3); §二 Monte Carlo
simulation modeling + theoretical pricing + pricing-deviation
indicator (pp.4-6); §三 strategy-design under 可以卖空 vs
不可以卖空 cases + 融资融券 regression + clause-structure filter
+ T/T+1 mechanics (pp.6-12); §四 sample-bond strategy-return
back-testing (pp.12-15; this card does NOT import the worked
numeric back-testing tables per Critical Rule 1). Open 安道全
(2023 3ed) §1.2-§1.10 pp.4-92 for the Chinese-market practitioner
treatment of CB turnover conventions and arbitrage-strategy
context. Open 攻守 §2-§3 pp.17-48 for an additional
practitioner-handbook cross-check. Open DeSpiegeleer §3.1-§3.3
pp.95-130 + Calamos §11 pp.260-300 ONLY for the international
delta-neutral / gamma-scalping framework that is a comparison
context — NOT Zhang+Feng's framework. **Source:** Zhang+Feng
(2014) §一-§四 pp.1-15; 安道全 (2023 3ed) §1.2-§1.10 pp.4-92;
攻守 §2-§3 pp.17-48; DeSpiegeleer et al. (2014) §3.1-§3.3
pp.95-130; Calamos (2003) §11 pp.260-300.
