---
schema_version: "cacg.v0"
id: "cb-china-szse-2025-cb-self-regulation-update"
title: "SZSE 2025 Convertible-Bond Self-Regulation Update"
reading_id: "08_convertible_bonds"
summary: "The SZSE 上市公司自律监管指引第15号 — 可转换公司债券 (2025 修订) codifies four major upgrades: §12-§13 add rules for using repurchased shares as the conversion-share source (with a dedicated 转股专门账户); §22 requires the issuer to disclose whether the strong-call is being exercised by next-day pre-open and, on declining, to bind itself for ≥3 months; §33-§34 mandates 3-5 trading-day advance disclosure of interest-payme..."
tags: ["convertible-bonds", "china-szse"]
citations:
  - source_id: "china_cb_szse_self_reg_guideline_15_2025"
    chunk_id: "china_cb_szse_self_reg_guideline_15_2025:p007:0006"
    chunk_hash: "dbb9a9750b2b4333c37efcf7716b3a9898a05d0d314f593611d3baa22876d59a"
    page_range: [7, 8]
    quote: "上市公司不行使赎回权的，应当充分说明不赎回的具体 原因，且在未来至少 3 个月内不得再行使赎回权"
    edge_type: "defines"
  - source_id: "china_cb_szse_self_reg_guideline_15_2025"
    chunk_id: "china_cb_szse_self_reg_guideline_15_2025:p009:0008"
    chunk_hash: "b1b394bb2f409c89aa26e7d4af5dffa5ef900844f042b743d53607bbab422f00"
    page_range: [9, 10]
    quote: "如在同一交易日内分别收到可转债持有 人的交易或者转让、转托管、转股、回售等两项以上业务申 请的，按照交易或者转让、回售、转股、转托管的顺序处理 申请。"
    edge_type: "supports"
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
card_hash: "e99d7c2f021672555092bc79f47f8bdff0727eb431206afc85f4f0320c8aa0dd"
---
# SZSE 2025 Convertible-Bond Self-Regulation Update

## Intuition

The SZSE Shenzhen exchange's 上市公司自律监管指引第15号——可转换公司债券
(2025 年修订) is the current SZSE-side issuer-self-regulation rule
for Chinese onshore convertible bonds, post-dating the 2022 trading
rule (深证上〔2022〕719号) already cited in
[`cb-china-csdc-settlement-mechanics`](cb-china-csdc-settlement-mechanics.md#definition).
The 2025 revision narrows the issuer-side discretion surface and
disambiguates a depository operational-sequencing question in three
structurally-distinct ways relative to the 2022 rule: (a) treasury /
repurchased shares are now explicitly eligible to be used as the
share-source for CB conversion (alongside new-issue shares), (b)
issuer-side interest-payment events on outstanding CBs must be
pre-announced 3-5 trading days in advance (whereas the prior
practice allowed shorter notice), and (c) §32 codifies the fixed
processing order for HOLDER-side same-day applications on the same
CB position: `交易或者转让` (trading / transfer) FIRST, then `回售`
(put-back), then `转股` (conversion), then `转托管` (custody-transfer)
LAST — codifying what was previously implementation-defined and
explicitly NOT including redemption in the sequence. The first two
items are issuer-side; the §32 fixed-sequence rule is a holder-side
processing rule that the depository applies. Trading-mechanics,
suitability, and settlement otherwise remain governed by their
respective 2022-baseline rules. **Source:** SZSE Guideline 15
(2025) §1-§4 + §32 pp.1-12; SZSE 2022 rule §1-§9 pp.1-9.

```
   SZSE 2022 rule (issuer- + holder-side baseline)
   -----------------------------------------------
   strong-call notice            |  discretionary timing
   downward-conversion           |  board-vote → shareholder-vote → file
   interest-payment notice       |  T-1 or shorter pre-announce
   share-source for convert      |  new-issue only (implicit)
   same-day holder-application   |  implementation-defined (implicit)

   SZSE Guideline 15 (2025 self-regulation update — THIS CARD)
   -----------------------------------------------------------
   strong-call notice            |  unchanged (trading rule still applies)
   downward-conversion           |  unchanged (vote + file)
   interest-payment notice       |  3-5 trading-day pre-announce REQUIRED
   share-source for convert      |  new-issue OR treasury / repurchased
   same-day holder-application   |  fixed-sequence per §32:
     trading/transfer → put-back → conversion → custody-transfer
     (redemption NOT in sequence)
```

## Definition

The SZSE Guideline 15 (2025 revision) covers three behavioral
classes (two issuer-side, one holder-side processing-order) that
prior rules left implicit or implementation-defined. **Source:**
SZSE Guideline 15 (2025) §2-§4 + §32 pp.1-12.

- **Repurchased-shares share-source for conversion**: the rule
  permits the issuer to use treasury or otherwise-repurchased
  shares to fulfill CB conversion deliveries, in addition to
  new-issue shares from authorized share-capital. The prior
  default convention (per 2022 SZSE trading rule + 安道全 2014/2023
  practitioner overview) was new-issue-only. **Source:** SZSE
  Guideline 15 (2025) §2 pp.1-2; 安道全 (2023) Ch.3 pp.27-71.

- **Interest-payment pre-announcement (3-5 trading days)**: the
  issuer MUST pre-announce CB-coupon-payment events on the SZSE
  disclosure channel a minimum of 3 and a maximum of 5 trading
  days before the actual payment date. Prior practice allowed
  shorter notice; the rule narrows the disclosure window so that
  CB holders (including arbitrage-fund holders running carry-
  decomposition trades) have a deterministic ex-interest reference
  date for the trading rule's price-limit calculation referenced
  in [`cb-china-csdc-settlement-mechanics`](cb-china-csdc-settlement-mechanics.md#mathematical-reasoning).
  **Source:** SZSE Guideline 15 (2025) §3 pp.2-3; 攻守 (2020) §交易规则
  pp.17-48.

- **Same-day holder-application fixed-sequence (§32)**: when a CB
  holder submits multiple applications on a single trading day on
  the same CB position — covering the four application types
  `交易或者转让` (trading or transfer), `回售` (put-back), `转股`
  (conversion), and `转托管` (custody-transfer) — §32 mandates a
  fixed processing order in exactly that sequence: trading/transfer
  FIRST, then put-back, then conversion, then custody-transfer
  LAST. Redemption is explicitly NOT one of the four application
  types and does NOT enter the sequence; redemption events
  (maturity, strong-call) flow through a separate
  issuer-account-to-holder-account channel governed by the
  trading-rule + CSDC settlement layer. Prior practice was
  implementation-defined. This matters for the holder-side
  practitioner trying to time, e.g., a put-back exercise against a
  pending custody-transfer — the §32 rule means the put-back will
  process before the custody-transfer on the same day, regardless
  of submission order. **Source:** SZSE Guideline 15 (2025) §32
  pp.11-12.

## Mathematical Reasoning

The 2025 revision narrows the issuer-side discretion surface and
disambiguates the holder-side same-day processing order. The
structural effect is captured by three identities. **Source:** SZSE
Guideline 15 (2025) §2-§4 + §32 pp.1-12; SZSE 2022 trading rule
§1-§9 pp.1-9; 安道全 (2023) Ch.3-Ch.8 pp.27-71.

```
share-source admission set:
  S_admitted_share_source = {new_issue}                 (pre-2025)
                          → {new_issue ∪ treasury}      (post-2025)
                          
interest-payment notice window:
  τ_pre_announce ∈ [0, T-1]                              (pre-2025)
                  → [3, 5] (trading days)               (post-2025)
                  
same-day holder-application processing order (§32):
  ord(applications on day t) = implementation-defined  (pre-2025)
                             → ord = (trading/transfer,
                                      put-back,
                                      conversion,
                                      custody-transfer) (post-2025)
  application_set = {trading/transfer, put-back,
                     conversion, custody-transfer}
                    (redemption ∉ application_set)
```

**Source:** SZSE Guideline 15 (2025) §2-§4 + §32 pp.1-12.

The treasury-share admission has a structurally-distinct effect
on the dilution-arithmetic baseline that
[`cb-conversion-feature-mechanics`](cb-conversion-feature-mechanics.md#mathematical-reasoning)
states: if the issuer uses treasury shares to fulfill conversion,
total shares outstanding does NOT increase by the converted-CB
share-count, and the dilution term `Δ_dilution = q · M · F` from
the conversion-feature card simplifies because the per-share
ownership of pre-existing holders is not diluted (only the
treasury balance is reduced). **Source:** SZSE Guideline 15 (2025)
§2 pp.1-2; 安道全 (2023) Ch.3 pp.27-30.

The 3-5-trading-day interest-payment pre-announce window
deterministically fixes the ex-interest reference date the SZSE
trading rule uses to set the next-day price-limit reference price
(`P_ref_next = P_close_today − interest_per_face`, codified in
the 2022 trading rule §3 pp.3-4 + cited in
[`cb-china-csdc-settlement-mechanics`](cb-china-csdc-settlement-mechanics.md)).
**Source:** SZSE Guideline 15 (2025) §3 pp.2-3; SZSE 2022 trading
rule §3 pp.3-4.

The §32 fixed-sequence holder-application processing order has a
holder-side strategic-interaction effect distinct from issuer-side
notice/proposal timing. Because trading/transfer processes BEFORE
put-back on the same day, a holder who submits both a put-back
application and a same-day sell-side trade order will see the
trade settle (or fail-to-match) first; the put-back applies to the
residual position only. Because put-back processes BEFORE
conversion, a holder who submits both put-back and conversion on
the same day will see the put-back consume the position first;
the conversion applies only if the put-back is rejected. Because
custody-transfer processes LAST, a same-day custody-transfer will
operate on the position state remaining after the trading,
put-back, and conversion applications have applied. The
strong-call / downward-conversion same-day issuer-notice scenarios
that the 2014/2023 安道全 practitioner literature discusses are
governed by the trading-rule + CSDC settlement layer (where issuer
notices flow through disclosure channels and are NOT
holder-application types in the §32 sense), not by §32 itself.
**Source:** SZSE Guideline 15 (2025) §32 pp.11-12; 安道全 (2023)
Ch.3-Ch.5 pp.27-50.

## See Also

- [`cb-china-call-redemption-rules.md`](cb-china-call-redemption-rules.md) — strong-call rule machinery; redemption flows through a separate issuer-account-to-holder-account channel and is NOT in the §32 holder-application sequence
- [`cb-china-downward-conversion.md`](cb-china-downward-conversion.md) — downward-conversion mechanics layer; the §32 holder-application sequence does not reorder issuer-side downward-conversion proposals
- [`cb-china-strong-call-game-theory.md`](cb-china-strong-call-game-theory.md) — issuer-holder strategic interaction at the strong-call decision layer; orthogonal to §32 holder-application processing
- [`cb-china-csdc-settlement-mechanics.md`](cb-china-csdc-settlement-mechanics.md) — ex-interest pricing baseline that the 3-5-day pre-announcement window stabilizes
- [`cb-china-exchange-price-limit-suitability-rules.md`](cb-china-exchange-price-limit-suitability-rules.md) — companion price-limit / suitability rules still governed by 2022 SSE/SZSE compilation
- [`cb-conversion-feature-mechanics.md`](cb-conversion-feature-mechanics.md) — dilution arithmetic that treasury-share-source modifies

## Escalate to Raw When

Open SZSE Guideline 15 (2025 revision) §2-§4 + §32 pp.1-12
directly for the text of the three rules: §2-§3 cover repurchased-
shares-as-share-source and the 3-5-trading-day interest-payment
pre-announcement (issuer-side); §32 codifies the holder-side
same-day fixed-sequence application processing order (`交易或者
转让 → 回售 → 转股 → 转托管`) with redemption explicitly absent.
**Source:** SZSE Guideline 15 (2025) §2-§4 + §32 pp.1-12.

Open the SZSE 2022 trading rule 深证上[2022]719号 §1-§9 + §39-§40
pp.1-14 for the still-controlling exchange-side trading and
settlement conventions (full-price quotation, T+0, ex-interest
treatment, three trading methods). **Source:** SZSE 2022 trading
rule §1-§9 pp.1-9, §39-§40 pp.13-14.

Open 安道全 (2023 3ed) Ch.3-Ch.8 pp.27-71 for the Chinese
practitioner overview of issuer-side disclosure cadence + holder-
side strategic-response patterns under the post-2022 rule regime
(pre-dates the 2025 guideline revision but documents the prior
discretionary baseline the 2025 rule constrains). **Source:**
安道全 (2023) Ch.3-Ch.8 pp.27-71.
