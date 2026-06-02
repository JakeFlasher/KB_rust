---
schema_version: "cacg.v0"
id: "cb-china-exchange-price-limit-suitability-rules"
title: "China Convertible-Bond Exchange-Rulebook Price-Limit, Suspension, and Suitability Rules"
reading_id: "08_convertible_bonds"
summary: "On top of generic CB trading mechanics, the SSE/SZSE rulebook layer adds a tighter operational envelope: T+0 全价 申报 with 0.001-yuan tick; 1000-yuan-face lot; 20%-up/-down daily price-limit (57.3%/-43.3% on first listing day); suitability gates requiring investor risk-disclosure acknowledgement; algorithmic-trading reporting; temporary suspension on extreme intraday moves. The 2022 SSE-SZSE 'Impl..."
tags: ["convertible-bonds", "china-exchange"]
citations:
  - source_id: "china_cb_sse_rules_compilation"
    chunk_id: "china_cb_sse_rules_compilation:p454:0421"
    chunk_hash: "e8478e9a93f11afd76b0de11acefd1c83aa132305c5e7d8e8849de2da7cf7170"
    page_range: [454, 455]
    quote: "向不特定对象发行的可转债上市后的首个交易日涨幅比例 为 57.3%、跌幅比例为 43.3%。上市首个交易日后，涨跌幅比例 为 20%。"
    edge_type: "defines"
  - source_id: "china_cb_szse_trading_rules_2022"
    chunk_id: "china_cb_szse_trading_rules_2022:p004:0003"
    chunk_hash: "799539c56654a6d0f2df7b77f47b4ae61afb4960e11c015cbc6cf437257316a3"
    page_range: [4, 5]
    quote: "除上市首日外，向不特定对象发行的可转债的价 格涨跌幅限制比例为 20%。"
    edge_type: "supports"
  - source_id: "cb_an_daoquan_2023_three_line_duplex_3ed"
    chunk_id: "cb_an_daoquan_2023_three_line_duplex_3ed:p032:0018"
    chunk_hash: "186acff34a5dd1161006a058ea42934a3eac982b9c7d06804c85eec252be2c51"
    page_range: [32, 33]
    quote: "①在转股期内，如果公司股票在仸意违续三十个交易日中至少 十五个交易日的收盘价栺不低于当期转股价栺的 130%（含 130%）；"
    edge_type: "supports"
  - source_id: "cb_gongshou_practical_handbook_1ed"
    chunk_id: "cb_gongshou_practical_handbook_1ed:p018:0014"
    chunk_hash: "95ff1ecb153d3a50520c3bcda12fe3d2e157ae46a7be808eff64935f23ac089c"
    page_range: [18, 19]
    quote: "在可转债最后两个计息年度，股票任何连续三十个交易日收盘价格低于当期 转股价格的70%，可转债持有人有权将其持有的可转债全部或者部分按债券面值加 上当期应计利息回售给公司。"
    edge_type: "supports"
card_hash: "dfbdb67088d012227c3345820a9569a8d227230faf6ceb891520d726bec8dd0b"
---
# China Convertible-Bond Exchange-Rulebook Price-Limit, Suspension, and Suitability Rules

## Intuition

China onshore convertible-bond trading on SSE and SZSE is governed
by an exchange-rulebook layer that sits on top of the generic
trading mechanics (see [cb-china-trading-mechanics](./cb-china-trading-mechanics.md)).
This rulebook layer adds (a) price-limit and temporary-suspension
triggers that constrain intraday CB price movement, (b) an
**investor-suitability gate** that restricts CB trading to
qualified participants under the prescribed risk-disclosure
framework, and (c) an algorithmic-trading reporting requirement
for participants whose CB strategies meet defined operational
thresholds. The SSE rulebook is consolidated in the SSE official
compilation 《可转换公司债券相关规定汇编》 with the trading-
rule section starting at page 450 and the suitability /
risk-disclosure / algorithmic-trading clauses through page 498.
The parallel SZSE rule 《深圳证券交易所可转换公司债券交易实施
细则》 (深证上〔2022〕719号 2022 final, effective 2022-08-01)
adopts the same exchange-rulebook structure with chapter
分别 covering 总则 / public-CB trading / private-CB transfer /
附则. **Source:** SSE compilation 2022- §三 pp.450-498 (trading
rule + suitability + risk-disclosure + algorithmic-trading
sections); SZSE rule (2022 final) pp.1-14 (4-chapter parallel
SZSE rulebook).

```
   China-onshore CB exchange-rulebook layer (SSE / SZSE)

   +-------------------------------------------+
   |  cb-china-trading-mechanics               |   <- mechanics
   |   (T+0, lot, conversion-ratio mechanics)  |      (other card)
   +-------------------------------------------+
                       |
                       |  on top of mechanics:
                       v
   +-------------------------------------------+
   |  Exchange-rulebook layer (THIS CARD):     |
   |    1. Price-limit + temporary-suspension  |
   |    2. Investor-suitability gate           |
   |    3. Risk-disclosure prerequisite        |
   |    4. Algorithmic-trading reporting       |
   |  Governed by:                             |
   |    SSE compilation pp.450-498             |
   |    SZSE rule (2022 final) pp.1-14         |
   +-------------------------------------------+
```

## Definition

The **exchange-rulebook layer** for Chinese onshore CB trading
carries four structurally-distinct components on top of the
generic trading mechanics. **Source:** SSE compilation 2022- §三
pp.450-466 (trading rule); SZSE rule (2022 final) Ch.2-3 pp.4-14.

- **Price-limit + temporary-suspension framework**: the SSE / SZSE
  rules specify the intraday price-movement bounds for CB trading
  and the conditions under which the exchange may temporarily
  suspend trading (high-volatility events; issuer-disclosure events
  pending material announcement). The bounds and triggers are
  exchange-specific and reside at the rulebook level rather than
  in the per-CB prospectus. **Source:** SSE compilation §三(一)
  pp.450-466; SZSE rule Ch.2 pp.4-9.
- **Investor-suitability gate**: trading in CBs requires a
  participant to meet the prescribed investor-suitability standard.
  The SSE 《关于可转换公司债券适当性管理相关事项的通知》 at
  SSE compilation pp.466-468 + the parallel SZSE 适当性 provisions
  in the SZSE rule Ch.1 §四 set the suitability gate, including
  experience-and-asset prerequisites for retail participants and
  documented risk-acknowledgment requirements. **Source:** SSE
  compilation §三(二)-(三) pp.466-470; SZSE rule Ch.1 §四 pp.2-3.
- **Risk-disclosure prerequisite**: SSE member firms must obtain a
  signed risk-disclosure document with mandatory disclosure
  clauses before a customer's first CB trade. The SSE compilation
  pp.470-484 contains the 风险揭示书必备条款 (risk-disclosure
  mandatory-clause set) updated July 2022. SZSE adopts an
  equivalent disclosure structure within its member firm management
  rules. **Source:** SSE compilation §三(四) pp.470-484; SZSE rule
  Ch.1 §四 pp.2-3 (cross-reference to member firm management).
- **Algorithmic-trading reporting**: SSE compilation pp.484-493
  carries the 程序化交易报告 requirement specifying that
  participants whose CB-trading volume / order-rate meets defined
  thresholds must file pre-trade reports with the SSE. The
  reporting framework parallels the broader equity-market algo-
  trading reporting requirement adapted to the CB market.
  **Source:** SSE compilation §三(五) pp.484-493.

## Mathematical Reasoning

The exchange-rulebook layer does not introduce new pricing or
valuation math beyond the generic trading mechanics. Its
mathematical content is structural — boolean gates and threshold
checks that admit or reject orders before they reach the matching
engine. **Source:** SSE compilation §三 pp.450-498; SZSE rule
(2022 final) Ch.2-3 pp.4-14.

```
   Order-admission gate (schematic):

   client_order(CB_i, qty, price) admitted iff:
     suitability_gate(client_i)          == TRUE    (Component 2)
     AND risk_disclosure_signed(client_i) == TRUE   (Component 3)
     AND price in [P_low(t), P_high(t)]             (Comp.1: intraday band)
     AND temporary_suspension_active(CB_i, t) == FALSE
     AND algo_trading_reported(client_i, qty,...) == TRUE if threshold (Comp.4)
```

Each gate is a discrete pre-trade check at the broker / member-
firm side; the exchange enforces them at the matching engine.
The price-limit band `[P_low(t), P_high(t)]` is exchange-specified
and updates intraday under the SSE / SZSE rule conventions; the
temporary-suspension flag `temporary_suspension_active(CB_i, t)`
is exchange-controlled and triggers on disclosure events or
extreme price moves. The investor-suitability gate is
boolean-on-client and persists across the client's entire CB
trading lifecycle once admitted. **Source:** SSE compilation §三(一)
pp.450-466 (price-limit + suspension rules); SSE compilation
§三(二) pp.466-468 (suitability gate); SSE compilation §三(四)
pp.470-484 (risk-disclosure prerequisite).

The **asymmetry** between SSE and SZSE rulebooks is small at the
structural level — both exchanges adopt the same four-component
framework, with minor differences in trigger-threshold numerical
values and reporting timing. The parallel SZSE rule
《深圳证券交易所可转换公司债券交易实施细则》 (深证上〔2022〕719号)
covers Chapter 1 (总则) + Chapter 2 (向不特定对象发行的可转债的
交易, public-CB trading) + Chapter 3 (向特定对象发行的可转债的
转让, private-CB transfer) + Chapter 4 (附则), all of which
contain the price-limit / suitability / risk-disclosure clauses
within their respective scope. **Source:** SZSE rule (2022 final)
Ch.1-4 pp.1-14.

Asymptotic / regime behaviour of the exchange-rulebook layer
follows three patterns. Normal-trading regime: all four components
are passive gates that admit orders for qualified clients trading
within the price band. High-volatility regime: the price-limit +
temporary-suspension framework dominates; orders outside the band
are rejected and the exchange may halt the CB pending issuer
disclosure. Disclosure-event regime: a pending material
announcement triggers a temporary suspension; the suitability +
risk-disclosure gates remain active but cannot trade through the
suspension. The algorithmic-trading reporting component activates
only when participant volume / order-rate meets the SSE threshold;
for retail participants this is dormant.
**Source:** SSE compilation §三 pp.450-498.

## See Also

- [`cb-china-trading-mechanics.md`](cb-china-trading-mechanics.md) — base China-onshore CB trading mechanics (T+0, lot-size, conversion-ratio) that the exchange-rulebook layer sits on top of
- [`cb-china-call-redemption-rules.md`](cb-china-call-redemption-rules.md) — strong-call mechanics that the exchange-rulebook disclosure events can trigger
- [`cb-china-csrc-disclosure-timing.md`](cb-china-csrc-disclosure-timing.md) — CSRC-level disclosure-content standards that the exchange-rulebook layer enforces operationally

## Escalate to Raw When

Open the SSE compilation 《可转换公司债券相关规定汇编》 §三
pp.450-498 directly for the SSE-side comprehensive treatment:
the trading rule itself (pp.450-466), the suitability-mgmt
notice (pp.466-468), the Sci-Tech innovation board suitability
parallel (pp.468-470), the risk-disclosure mandatory clauses
(pp.470-484), the algorithmic-trading reporting (pp.484-493),
and the trading-fee adjustment (pp.493+). Open the SZSE rule
《深圳证券交易所可转换公司债券交易实施细则》 (2022 final,
深证上〔2022〕719号) pp.1-14 for the parallel SZSE rulebook
in its 4-chapter structure (general + public-CB trading +
private-CB transfer + 附则). Open 安道全 (2023 3ed) §1.6-§1.10
pp.27-71 for the Chinese-market practitioner cross-check on
how these exchange-rulebook gates interact with the strong-call
+ no-call-commitment + 下修 game plays at the participant level.
Open 攻守 §2 pp.17-48 for an additional practitioner-handbook
cross-check on the same rulebook layer. **Source:** SSE
compilation §三 pp.450-498; SZSE rule (2022 final) pp.1-14;
安道全 (2023 3ed) §1.6-§1.10 pp.27-71; 攻守 §2 pp.17-48.
