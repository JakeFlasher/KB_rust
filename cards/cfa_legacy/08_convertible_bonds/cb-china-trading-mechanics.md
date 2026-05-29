---
schema_version: "cacg.v0"
id: "cb-china-trading-mechanics"
title: "China Convertible-Bond Trading Mechanics"
reading_id: "08_convertible_bonds"
summary: "Chinese onshore CB trading: 100 RMB par; lot of 1000 RMB face for matched orders; T+0 round-trip (same-day buy and sell); 20% daily price-limit after listing day; 0.001-yuan minimum tick on a quoted-as-full-price basis. The conversion-share source defaults to newly issued shares but can use repurchased shares under the 2025 SZSE update. Trading differs structurally from US-style listed converti..."
tags: ["convertible-bonds", "china-trading"]
citations:
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p032:0018"
    chunk_hash: "186acff34a5dd1161006a058ea42934a3eac982b9c7d06804c85eec252be2c51"
    page_range: [32, 33]
    quote: "①在转股期内，如果公司股票在仸意违续三十个交易日中至少 十五个交易日的收盘价栺不低于当期转股价栺的 130%（含 130%）；"
    edge_type: "defines"
  - source_id: "cb_an_daoquan_2014_magic_book_2ed"
    chunk_id: "cb_an_daoquan_2014_magic_book_2ed:p014:0013"
    chunk_hash: "b8d22a0115f690fe31f95410fdee5e3a03f52a1374e926b5fbba64366d3790e3"
    page_range: [14, 15]
    quote: "可转债，全名叫做“可转换公司债券”（Convertible bond；CB），"
    edge_type: "supports"
  - source_id: "cb_gongshou_practical_handbook_1ed"
    chunk_id: "cb_gongshou_practical_handbook_1ed:p064:0058"
    chunk_hash: "e5e3ecbf7f39ceb564b729f086337f4deed090ce2c037eec9d54ed2bd186b777"
    page_range: [64, 65]
    quote: "可转 债网上申购具备“两低两高”的特征——风险低，中签高；投入低，收益高。"
    edge_type: "supports"
  - source_id: "china_cb_szse_trading_rules_2022"
    chunk_id: "china_cb_szse_trading_rules_2022:p004:0003"
    chunk_hash: "799539c56654a6d0f2df7b77f47b4ae61afb4960e11c015cbc6cf437257316a3"
    page_range: [4, 5]
    quote: "除上市首日外，向不特定对象发行的可转债的价 格涨跌幅限制比例为 20%。"
    edge_type: "supports"
card_hash: "ae7af48e7d93dc48ec3f97735a42b8494c4f2d3a509d9ed13c0503c6eeb2319c"
---
# China Convertible-Bond Trading Mechanics

## Intuition

China onshore convertibles (可转债) trade on the Shanghai and Shenzhen
exchanges with a set of mechanics that materially differ from US-style
listed CBs. Three differences matter most for the arbitrageur and the
buy-and-hold investor: **T+0 settlement** (a CB bought today can be sold
today, unlike stocks which are T+1), **daily price-limit rules** that
differ between the bond and its underlying share, and **lot conventions**
that fix the unit of trading at one bond per lot rather than per share.
**Source:** 安道全 (2023 3ed) §1-§3 pp.4-72 (current mechanics; post-2022 SSE/SZSE rule regime); 安道全 (2014) §1-§3 pp.20-60 (legacy mechanics, retained as Supporting reference).

```
US listed CB                       China onshore CB (可转债)
   |                                  |
   |- next-day settlement             |- same-day round-trip allowed
   |- one bond is the face unit       |- one bond is the face unit (面值)
   |- round-lot varies by issue       |- one lot fixes the bond count (一手)
   |- no daily price cap by default   |- intraday move triggers exchange
   |  (circuit breakers ad hoc)       |  suspension at practitioner caps
   |- equity short usually borrowable |- equity short typically gated
```

## Definition

The mechanics below summarize the practitioner-quoted conventions for
exchange-listed China onshore convertibles after the 2022 SSE/SZSE
CB-trading-rule revision. **Source:** 安道全 (2023 3ed) §1-§3 pp.4-72; 攻守 §1-§2 pp.26-41; Zubulake §3 pp.50-90.

- **Face per unit** `F`: every issued bond has a fixed RMB face (typically `F = 100元`); prices are
  quoted "per bond" against the face anchor, so a bond trading at "120"
  means 120% of face per bond. **Source:** 安道全 (2023 3ed) §1 pp.4-22; 攻守 §1 pp.26-35.
- **Lot size**: one **lot** is a fixed number of bonds (`1 手 = 10 张`),
  i.e. ten bonds. Most retail trading is in single-lot increments.
  **Source:** 安道全 (2023 3ed) §1 pp.56-72; 攻守 §1 pp.40-41.
- **Settlement** (`T+0`): a bond purchased on a trade date `τ` can be sold
  the same day; cash settles on the next business day. This contrasts
  with the underlying share, which is next-business-day settlement (sale
  proceeds available the day after). The asymmetry has practical
  implications for arbitrage hedging — a hedged short-stock leg cannot be
  unwound the same day as a CB purchase. **Source:** 安道全 (2023 3ed) §2
  pp.23-92; 攻守 §2 pp.26-71.
- **Daily price limits + intraday suspension**: after the 2022 SSE/SZSE
  rule revision, exchange-listed convertibles trigger temporary trading
  suspensions when intraday moves cross specified thresholds (the
  practitioner-quoted thresholds replaced the earlier soft-cap regime
  per the post-2022 rulebook). **Source:** 安道全 (2023 3ed) §5 pp.166
  (suspension mechanics); 攻守 §2 pp.27-96 (intraday-price-limit and
  竞价/集合-竞价 mechanics).
- **Conversion timing**: holders may submit conversion orders on any
  trading day after the conversion-eligible start date `T_conv_start`
  (typically six months after issuance). Conversion settles on the next
  business day in the underlying share. **Source:** 安道全 (2023 3ed) §3
  pp.60-92.
- **No fractional shares**: the conversion ratio `q = F / K_c` is rounded
  down per holder; residual face value is paid in cash. This is consistent
  with the rounding behavior described in the
  [conversion-mechanics card](./cb-conversion-feature-mechanics.md#definition).
  **Source:** 安道全 (2023 3ed) §3 pp.62-92.

The trading-mechanics rules **do not** alter the issuer's contractual
embedded options (call, put, downward-conversion). Those are governed by
prospectus and are covered by the related cards on
[issuer-call mechanics](./cb-call-and-put-protection.md#definition) and
the China-specific call/down-conversion cards (later batch).
**Source:** 安道全 (2023 3ed) §3 pp.60-92.

## Mathematical Reasoning

The settlement asymmetry between bond and stock changes the arbitrageur's
cash-flow timing but not the steady-state hedge math. **Source:** 安道全
(2023 3ed) §2 pp.23-92. Specifically, if the trader purchases `M` face of CB
on trade date `τ` (settles next day) and shorts `Δ_S · M` shares on the
same trade date (also next-day settlement), the cash-balance impact
matches the settlement-conventional case discussed in the
[arbitrage-strategy card](./cb-arbitrage-strategy.md#mathematical-reasoning).
However, an intraday close-out of the CB leg on the same day as the
opening trade (allowed under same-day round-trip rules) cannot be matched
by a same-day close of the stock leg (next-day rule). This asymmetry
matters for **scalping** strategies but not for **buy-and-hold-plus-
delta-hedge** strategies. **Source:** 安道全 (2023 3ed) §2 pp.23-92; 攻守 §2 pp.71-96.

The post-2022 price-limit rule is **threshold-triggered suspension**: when
an intraday move crosses the SSE/SZSE-specified threshold, trading is
suspended for a fixed window rather than capped, after which the next
auction-period quote sets the resumption price. The next day's open is
unconstrained relative to the suspension trigger. This differs from US
listed convertibles, which have no intraday cap absent ad-hoc circuit-
breaker triggers. **Source:** 安道全 (2023 3ed) §5 pp.166; 攻守 §2
pp.27-96; Zubulake §3 pp.50-90.

The lot-size convention `10 bonds = 1 lot` interacts with the conversion
ratio `q = F / K_c`. A holder converting one lot of face-times-ten RMB at
strike `K_c` receives `floor((10 · F) / K_c)` shares plus residual cash for
the rounding remainder. This is the practitioner intuition that
"conversion is share-discrete; the residual settles in cash". **Source:**
安道全 (2023 3ed) §3 pp.62-92.

Asymptotic regimes for China-onshore CBs (cases below). **Source:**
安道全 (2023 3ed) §1-§3 pp.4-92; Calamos (2003) §6 pp.95-130.

- **Low-vol bull regime**: many CBs trade well above face (e.g. >120) for
  long stretches, sustained by retail demand and the absence of effective
  short-selling on the underlying. **Source:** 安道全 (2023 3ed) §1-§3
  pp.4-92.
- **Forced-conversion regime**: when the issuer triggers a strong-call
  redemption (covered in the China call-redemption card, later batch),
  the bond price collapses toward parity within a single trading window.
  **Source:** 安道全 (2023 3ed) §3 pp.62-92; 攻守 §3 pp.27-90.
- **Distressed regime**: low-credit issuers see secondary-market liquidity
  shrink dramatically; bid-ask spreads widen and the bond floor is poorly
  enforced because the implied stock-borrow market is illiquid. **Source:**
  Calamos (2003) §6 pp.95-130 (general distressed-CB framing transferred
  to the China context per 安道全 (2023 3ed) §3 pp.62-92).

## See Also

- [`cb-bond-anatomy-and-cashflows.md`](cb-bond-anatomy-and-cashflows.md) — face, coupon, maturity contracted in prospectus
- [`cb-conversion-feature-mechanics.md`](cb-conversion-feature-mechanics.md) — base conversion exercise rules
- [`cb-call-and-put-protection.md`](cb-call-and-put-protection.md) — issuer call / holder put structure (trigger forms)
- [`cb-arbitrage-strategy.md`](cb-arbitrage-strategy.md) — hedge-ratio mechanics that the T+0 asymmetry interacts with

## Escalate to Raw When

Open 安道全 (2023 3ed) §1-§3 pp.4-92 directly for the post-2022 China-onshore
practitioner playbook: lot conventions, settlement timeline, intraday
suspensions, and conversion-day mechanics. **Source:** 安道全 (2023 3ed)
§1-§3 pp.4-92.

Open 攻守 §1-§2 pp.26-96 for the complementary practitioner-handbook
treatment of trading rules, intraday-price-limit, and 竞价 mechanics.
**Source:** 攻守 §1-§2 pp.26-96.

Open 安道全 (2014) §1-§3 pp.20-100 (Supporting) for the pre-2022 baseline
language; useful when comparing pre/post-2022 rule changes. **Source:**
安道全 (2014) §1-§3 pp.20-100.

Open Zubulake §3 pp.50-90 for the cross-jurisdictional comparison
(US, Europe, Japan, China) of trading and settlement mechanics.
**Source:** Zubulake §3 pp.50-90.

Open Calamos (2003) §6 pp.95-130 for the practitioner's framing of
illiquid / distressed CB trading dynamics that transfer to the China
distressed regime. **Source:** Calamos (2003) §6 pp.95-130.
