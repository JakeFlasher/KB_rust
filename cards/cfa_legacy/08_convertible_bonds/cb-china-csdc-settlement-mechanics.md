---
schema_version: "cacg.v0"
id: "cb-china-csdc-settlement-mechanics"
title: "China Convertible-Bond Exchange-Rule-Side Trading and Settlement Conventions"
reading_id: "08_convertible_bonds"
summary: "China onshore CB trading on SSE/SZSE is codified with structurally-distinct conventions: full-price quotation with 0.001-yuan minimum increment, same-day round-trip (T+0) trading and transfer, public-CB multilateral net settlement vs private-CB transaction-by-transaction gross settlement, and ex-interest treatment — all rule-level specifications from the SZSE 2022 trading rules and SSE rules co..."
tags: ["convertible-bonds", "china-csdc"]
citations:
  - source_id: "china_cb_szse_trading_rules_2022"
    chunk_id: "china_cb_szse_trading_rules_2022:p001:0000"
    chunk_hash: "9fc0e7cb15bbf5ebd23dd27382de459fd6cac03be24065646365d772b2607e38"
    page_range: [1, 2]
    quote: "第二章 向不特定对象发行的可转债的交易 第一节 一般规定 第七条 向不特定对象发行的可转债采用匹配成交、协商成 交、盘后定价成交等交易方式。"
    edge_type: "defines"
  - source_id: "china_cb_sse_rules_compilation"
    chunk_id: "china_cb_sse_rules_compilation:p450:0417"
    chunk_hash: "2093b5a0c916323d04ace7263bc441acd88bd47eaf5943d1e76255284b07cd3f"
    page_range: [450, 451]
    quote: "为落实中国证监会《可转换公司债券管理办法》的要求，进 一步规范上市公司可转换公司债券交易行为，维护市场交易秩序， 上海证券交易所制定了《上海证券交易所可转换公司债券交易实 施细则》，已经中国证监会批准，现予以发布，并自 2022 年 8 月 1 日起施行。"
    edge_type: "supports"
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p012:0006"
    chunk_hash: "7b58bd1ed8ca47b54d3552c1ac9cf0db649e6c2671d542ab17cae0e6f572ebae"
    page_range: [12, 13]
    quote: "1.2 从哪里入手了解可转债——収行公告 或募集说明书 了解一只可转债，最权威、最快捷的办法是阅读这只可转债的 収行公告或者募集说明书。"
    edge_type: "supports"
  - source_id: "cb_an_daoquan_2014_magic_book_2ed"
    chunk_id: "cb_an_daoquan_2014_magic_book_2ed:p111:0107"
    chunk_hash: "d52dc30103c21a11c546426ea592940c24958bbd454ce19e1ec7a5ea8061d891"
    page_range: [111, 112]
    quote: "投资者只要坚守纪律，甚 至无须太多关心股市和股票本身，耐心等待强制赎回的到来就行了， 这特别适合信息和技术都处于劣势的小散户们。"
    edge_type: "supports"
card_hash: "c83403f254cb84be60701a2d4cf97330e83472272c1283df77a57b2217e402b2"
---
# China Convertible-Bond Exchange-Rule-Side Trading and Settlement Conventions

## Intuition

China onshore CB trading on SSE and SZSE is codified at the
exchange-rule layer with a small set of structurally-distinct
conventions that determine how trades are quoted, matched, and
settled within the exchange's own clearing structure. The SZSE
rule 《深圳证券交易所可转换公司债券交易实施细则》 (深证上〔2022〕
719号, 2022 final, effective 2022-08-01) and the parallel SSE
compilation 《可转换公司债券相关规定汇编》 §三(一) pp.450-466
together carry the rule-level specification of: (a) full-price
quotation with a 0.001-yuan minimum-increment convention,
(b) same-day round-trip trading and transfer (the practitioner-
called T+0 trading convention), (c) public-CB multilateral net
settlement versus private-CB transaction-by-transaction gross
settlement, (d) ex-interest treatment on interest-payment dates
that affects the price-limit reference, and (e) three trading
methods (匹配成交 / 协商成交 / 盘后定价成交) for public-CB
trades. The cited rules are bounded to these exchange-side
conventions; CSDC-side operational specifics (depository
retirement, conversion / redemption / put cash-flow processing)
are NOT specified by the cited PDFs and stay out of this card's
scope. **Source:** SZSE rule (2022 final) §1-§9 + §39-§40 pp.1-14;
SSE compilation §三(一) pp.450-466.

```
   China-onshore CB exchange-rule layer (THIS CARD'S SCOPE)
   --------------------------------------------------------

   +------------------------------------------------------+
   |  SSE compilation pp.450-466                          |
   |  SZSE rule (2022 final) pp.1-14                      |
   |                                                      |
   |   (a) full-price quotation + 0.001-yuan increment    |
   |   (b) same-day round-trip trading + transfer (T+0)   |
   |   (c) public-CB multilateral net settlement          |
   |       (SZSE §九 / SSE compilation §三(一))            |
   |       private-CB transaction-by-transaction gross    |
   |       settlement (SZSE §四十)                         |
   |   (d) ex-interest treatment on interest-payment day  |
   |       (SZSE §十)                                      |
   |   (e) three public-CB trading methods                |
   |       (matching / negotiated / after-hours-fixed)    |
   |                                                      |
   |  OUT OF SCOPE for this card:                         |
   |    CSDC-side operational depository details          |
   |    Conversion / redemption / put cash-flow specifics |
   |    Cross-border (Bond Connect) mechanics             |
   +------------------------------------------------------+
```

## Definition

The **exchange-rule-side trading and settlement conventions** for
Chinese onshore CBs decompose into five rule-level components
that the cited SZSE 2022 final rule and SSE compilation actually
specify. **Source:** SZSE rule (2022 final) §五-§十 + §三十九-§四十
pp.1-14; SSE compilation §三(一) pp.450-466.

**Full-price quotation + minimum-increment convention**: SZSE rule
§五 specifies CB trading or transfer adopts full-price (`全价价格`)
quotation. SZSE §六 specifies the price-quotation unit is "每百元面
额债券的价格" (price per 100-yuan face value) with a minimum
increment of 0.001 yuan. The SSE compilation carries the parallel
SSE conventions. **Same-day round-trip trading and transfer (T+0
trading convention)**: SZSE §五 specifies trades and transfers are
executed with same-day round-trip turnover (`实行当日回转交易或者
转让`). This is the rule-layer codification of what practitioners
call the T+0 trading convention. The cited rules bound the
codification to the trading / transfer execution side; they do NOT
codify CSDC-side cash-settlement-completion timing or depository
retirement. **Public-CB multilateral net settlement (`多边净额方式
结算`)**: SZSE §九 specifies that public-CB trades settle through
the multilateral net method; the SSE compilation adopts a parallel
public-CB rule. **Private-CB transaction-by-transaction gross
settlement (`逐笔全额方式`)**: SZSE §四十 specifies that private-CB
transfers settle in transaction-by-transaction gross-settlement mode
(the counterpart to public-CB net settlement); SZSE §三十九
specifies that once the exchange's trading system confirms a
private-CB transfer, the parties must accept the result and fulfill
the settlement obligation (`履行交收义务`). **Ex-interest treatment
on interest-payment days**: SZSE §十 specifies that on the trading
day following the record date for a CB interest payment, the
exchange processes the bond as ex-interest (`除息处理`). The
ex-interest reference price for the next-day price-limit calculation
is the prior close minus the paid interest (`除息参考价 = 前收盘价
− 本次支付的利息`).
**Source:** SZSE rule §五-§十 + §三十九-§四十 pp.2-13; SSE compilation §三(一) pp.450-466.

The **three public-CB trading methods** specified by SZSE §七 are
matching (`匹配成交`), negotiated (`协商成交`), and after-hours-
fixed-price (`盘后定价成交`). Matching is the standard
price-time-priority auction the matching engine runs throughout
the trading day. Negotiated trades are between two parties who
designate each other as counterparties and negotiate price and
quantity. After-hours-fixed-price trades execute after the close
at either the day's close or the day's matching-mode VWAP.
**Source:** SZSE rule §七 pp.4-5.

## Mathematical Reasoning

The exchange-rule-side trading and settlement conventions are
structurally a set of rule-level specifications that determine
how trade and transfer events are quoted, matched, and settled
within the exchange's own clearing structure. **Source:** SZSE rule
(2022 final) §五-§十 + §三十九-§四十 pp.1-14.

```
   Exchange-rule-side convention mapping (schematic):

   Convention                |  Rule-level specification
   ---------                 |  -------------------------
   Quotation                 |  full-price (P_quote);
                             |  P per 100-yuan face;
                             |  min increment 0.001 yuan
   Turnover                  |  same-day round-trip
                             |  (T+0 trading convention)
   Public-CB settlement      |  multilateral net
   Private-CB settlement     |  transaction-by-transaction gross
   Ex-interest reference     |  P_ex = P_prev_close − interest_paid
   Trading methods           |  matching / negotiated / after-hours-fixed
```

The **price-limit reference** on the ex-interest day uses the
`P_ex` reference per SZSE §十, so the next day's price-limit
calculation `[P_low_next, P_high_next]` anchors on `P_ex` rather
than `P_prev_close`. This is a small structural feature that
practitioners note when computing the day-2 price-limit band on
interest-payment cycles. **Source:** SZSE rule §十 pp.5-6.

The **public-vs-private settlement asymmetry** (SZSE §九 vs §四十)
is the rule-level distinction between two settlement methods:
public-CB trades settle via multilateral net (`多边净额`); private-CB
transfers settle via transaction-by-transaction gross (`逐笔全额`).
The cited rules specify the settlement-method distinction only;
operational settlement-failure processing, depository-balance
update timing, and counterparty-risk allocation between the two
methods are NOT specified in the cited PDFs and would require a
separate quotable CSDC operational source which is not on disk.
**Source:** SZSE rule §九 + §四十 pp.5-6 + pp.12-13.

Asymptotic / regime behaviour of the exchange-rule-side
conventions follows three patterns. Normal-trading regime: all
five conventions apply uniformly; matched public-CB trades net
into the multilateral pool and private-CB transfers gross-settle
per the rule. Ex-interest-day regime: the day-2 price-limit band
anchors on `P_ex` rather than the prior close; the multilateral
net pool and gross-settlement modes are unchanged. Trading-halt
regime: the same-day round-trip convention is paused for the
duration of any temporary suspension (the suspension framework
itself resides in the price-limit + suitability-rules layer at
[cb-china-exchange-price-limit-suitability-rules](./cb-china-exchange-price-limit-suitability-rules.md)).
**Source:** SZSE rule (2022 final) §五-§十 pp.1-9.

## See Also

- [`cb-china-trading-mechanics.md`](cb-china-trading-mechanics.md) — base China-onshore CB trading mechanics (T+0, lot, conversion-ratio) at the practitioner level that this card's rule-level codification implements
- [`cb-china-exchange-price-limit-suitability-rules.md`](cb-china-exchange-price-limit-suitability-rules.md) — sibling exchange-rule-layer card covering price-limit + suspension + investor-suitability gates
- [`cb-china-csdc-settlement-depth-2023-2025.md`](cb-china-csdc-settlement-depth-2023-2025.md) — depository-side operational layer that extends this card's exchange-rule-side coverage with directional CB transfer + restricted-vs-unrestricted classification + same-day depository request-priority ordering

## Escalate to Raw When

Open the SZSE rule 《深圳证券交易所可转换公司债券交易实施细则》
(2022 final, 深证上〔2022〕719号) pp.1-14 directly for the
SZSE-side 4-chapter exchange-rule structure: §1 总则 covers
quotation conventions + same-day round-trip + price-quotation
units; §2 covers public-CB trading + multilateral net settlement
+ ex-interest treatment + three trading methods; §3 covers
private-CB transfer + transaction-by-transaction gross
settlement; §4 附则. Open the SSE compilation 《可转换公司债券
相关规定汇编》 §三(一) pp.450-466 for the parallel SSE
trading-rule structure with the same conventions embedded.
Open 安道全 (2023 3ed) §1.2-§1.10 pp.4-92 for the Chinese-market
practitioner cross-check on T+0 trading, ex-interest day
behaviour, and price-quotation unit conventions at the
participant level. Open 安道全 (2014) §3-§5 pp.20-100 for the
pre-2018 baseline practitioner language on CB-specific trading
conventions. NOTE: this card does NOT cover CSDC-side
operational specifics (depository retirement, conversion /
redemption / put cash-flow processing); those would require a
quotable CSDC source which is not currently on disk and are
out of scope per the cited PDFs. **Source:** SZSE rule (2022
final) pp.1-14; SSE compilation §三(一) pp.450-466; 安道全 (2023
3ed) §1.2-§1.10 pp.4-92; 安道全 (2014) §3-§5 pp.20-100.
