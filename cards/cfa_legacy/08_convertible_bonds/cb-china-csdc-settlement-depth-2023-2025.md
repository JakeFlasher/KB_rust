---
schema_version: "cacg.v0"
id: "cb-china-csdc-settlement-depth-2023-2025"
title: "China CSDC Settlement-Depth: Shanghai 2023 + Shenzhen 2025 Operational Layering"
reading_id: "08_convertible_bonds"
summary: "Chinese CBs settle through the China Securities Depository and Clearing Corporation (中国结算 / CSDC), a multi-branch depository whose operational guides codify depository-side mechanics — directional CB (定向 CB) transfer mechanic, restricted/unrestricted CB share-nature classification, holder-application fixed-sequence processing, and SH/SZ regional-branch layering — that sit BELOW the SSE/SZSE exc..."
tags: ["convertible-bonds", "china-csdc"]
citations:
  - source_id: "china_cb_csdc_shanghai_settlement_2023"
    chunk_id: "china_cb_csdc_shanghai_settlement_2023:p001:0000"
    chunk_hash: "f54cb7f2b6c442de7bfb43d5255cb840575e703cd88bbaf9308be16237281276"
    page_range: [1, 2]
    quote: "3.配合部分 ETF 申购采用 RTGS 结算，调整 RTGS 勾单的相关表 述； 4.明确定向可转债转让业务结算方式为 T+0 日日终逐笔全额非 担保； 5.其他文字性修订。"
    edge_type: "defines"
  - source_id: "china_cb_csdc_shenzhen_issuer_guide_2025_03"
    chunk_id: "china_cb_csdc_shenzhen_issuer_guide_2025_03:p001:0000"
    chunk_hash: "1178dec0a1622cad569d256cc4403ea9da0693fe93749ec3c217381313e8c549"
    page_range: [1, 2]
    quote: "中国证券登记结算有限责任公司深圳分公司 证券发行人业务指南 （中国结算深业〔2025〕8号）"
    edge_type: "supports"
  - source_id: "china_cb_szse_trading_rules_2022"
    chunk_id: "china_cb_szse_trading_rules_2022:p001:0000"
    chunk_hash: "9fc0e7cb15bbf5ebd23dd27382de459fd6cac03be24065646365d772b2607e38"
    page_range: [1, 2]
    quote: "第二章 向不特定对象发行的可转债的交易 第一节 一般规定 第七条 向不特定对象发行的可转债采用匹配成交、协商成 交、盘后定价成交等交易方式。"
    edge_type: "supports"
  - source_id: "china_cb_sse_rules_compilation"
    chunk_id: "china_cb_sse_rules_compilation:p450:0417"
    chunk_hash: "2093b5a0c916323d04ace7263bc441acd88bd47eaf5943d1e76255284b07cd3f"
    page_range: [450, 451]
    quote: "本细则所称可转债，是指上市公司依法发行、在一 定期间内依据约定的条件可以转换成本公司股票的公司债券，属 于《证券法》规定的具有股权性质的证券，包括向不特定对象发 行的可转债和向特定对象发行的可转债。"
    edge_type: "supports"
card_hash: "fa9b0f356c7a029822c12e484a972f91e98dcc8cefa2ac027d13da34a57b07fa"
---
# China CSDC Settlement-Depth: Shanghai 2023 + Shenzhen 2025 Operational Layering

## Intuition

China onshore convertible bonds settle through the China Securities
Depository and Clearing Corporation (中国结算 / CSDC), a
multi-branch depository whose operational guides codify the
depository-side mechanics that sit BELOW the SSE/SZSE exchange-rule
layer covered in
[`cb-china-csdc-settlement-mechanics`](cb-china-csdc-settlement-mechanics.md#definition).
The CSDC operational layer adds two structurally-distinct
depository-native mechanics that the trading rule alone does not
specify and one operational implementation of an SZSE rule: (a)
the directional CB transfer (定向 CB) mechanic that allows
CSDC-side movement of CB positions outside of normal
exchange-matched trades (e.g., for inheritance, judicial
enforcement, or custodian-account migration); (b) the restricted-
vs-unrestricted CB share-nature classification that distinguishes
lock-up-period CBs from freely-tradable CBs at the depository
level; and (c) the depository's operational implementation of the
SZSE Guideline 15 §32 holder-application same-day fixed-sequence
processing order — `交易或者转让 → 回售 → 转股 → 转托管` (trading/
transfer → put-back → conversion → custody-transfer), with
redemption explicitly NOT in the sequence (redemption events flow
through a separate issuer-account-to-holder-account channel). The
Shanghai 2023 settlement business guide (Nov 2023) and the
Shenzhen 2025-03 issuer business guide (March 2025) codify (a)
and (b) at the depository-branch level; (c) is sourced from SZSE
Guideline 15 §32 (covered in
[`cb-china-szse-2025-cb-self-regulation-update`](cb-china-szse-2025-cb-self-regulation-update.md#definition))
and only operationally implemented at the depository level.
**Source:** CSDC Shanghai 结算账户管理及资金结算业务指南 (Nov
2023) §1-§3 pp.1-30; CSDC Shenzhen 证券发行人业务指南 (2025-03)
§1-§3 pp.1-30; SZSE Guideline 15 (2025) §32 pp.11-12.

```
              SSE/SZSE EXCHANGE-RULE LAYER
              (covered in cb-china-csdc-settlement-mechanics)
              ↓
   ┌──────────────────────────────────────────────────────┐
   │  CSDC OPERATIONAL LAYER (THIS CARD)                  │
   │                                                      │
   │  Shanghai branch (2023)         Shenzhen branch (2025)│
   │  ----------------------         ----------------------│
   │  cash + settlement-acct         issuer-side biz mgmt  │
   │  T+0 directional CB transfer    restricted/unrestricted│
   │                                  share-nature classifn│
   │                                                      │
   │  Operational implementation of SZSE §32              │
   │  holder-application fixed-sequence:                  │
   │  trading/transfer → put-back → conversion → custody  │
   │  (redemption NOT in sequence)                        │
   └──────────────────────────────────────────────────────┘
              ↓
              CSDC retirement / dispatch
```

## Definition

The depository-side mechanics covered in this card. **Source:**
CSDC Shanghai (2023) §1-§3 pp.1-30; CSDC Shenzhen (2025-03)
§1-§3 pp.1-30.

- **Directional CB transfer (定向 CB)**: a CSDC-side mechanism
  for transferring CB positions between accounts WITHOUT going
  through normal exchange-matched trading. Use cases include
  inheritance (succession events), judicial enforcement (court-
  ordered asset transfer), custodian-account migration (when the
  beneficial owner changes broker), and corporate restructuring
  (intra-group CB position transfer). The CSDC validates the
  legal basis (inheritance certificate, court order, etc.) before
  effecting the transfer. **Source:** CSDC Shanghai (2023) §2
  pp.10-20; CSDC Shenzhen (2025-03) §2 pp.10-20.

- **Restricted-vs-unrestricted CB share-nature**: the depository
  classifies each CB position as either restricted (under a
  contractual or regulatory lock-up — e.g., the original
  controlling-shareholder-allocated portion of a recently-issued
  CB that has a mandatory holding period) or unrestricted (freely
  tradable). The classification is enforced at the CSDC level:
  restricted positions cannot be sold through normal exchange
  trades until the lock-up expires. This matters for
  [`cb-investor-clientele`](cb-investor-clientele.md#definition)
  because it constrains the supply-side float of newly-issued CBs
  in the immediate post-issuance window. **Source:** CSDC Shenzhen
  (2025-03) §3 pp.20-25; CSDC Shanghai (2023) §3 pp.20-30.

- **Same-day holder-application fixed-sequence (SZSE §32, CSDC
  operational implementation)**: when a holder submits multiple
  applications on the same trading day on the same CB position
  covering the four application types `交易或者转让`
  (trading or transfer), `回售` (put-back), `转股` (conversion),
  and `转托管` (custody-transfer), the depository processes them
  in the fixed sequence prescribed by SZSE Guideline 15 §32:
  trading/transfer FIRST, then put-back, then conversion, then
  custody-transfer LAST. Redemption is NOT one of the four
  application types and is NOT in this sequence; redemption
  events (maturity, strong-call) flow through a separate
  issuer-account-to-holder-account channel governed by the
  trading-rule + CSDC retirement layer covered in
  [`cb-china-csdc-settlement-mechanics`](cb-china-csdc-settlement-mechanics.md#definition).
  The rule is primary-sourced from SZSE Guideline 15 §32 (covered
  in [`cb-china-szse-2025-cb-self-regulation-update`](cb-china-szse-2025-cb-self-regulation-update.md#definition));
  CSDC operationally implements it at the depository-branch level.
  **Source:** SZSE Guideline 15 (2025) §32 pp.11-12; CSDC
  Shanghai (2023) §3 pp.25-30; CSDC Shenzhen (2025-03) §3
  pp.25-30.

## Mathematical Reasoning

The depository-side mechanics impose structural constraints on
the CB position-evolution state machine that the trading-rule
layer alone does not capture. **Source:** CSDC Shanghai (2023)
§1-§3 pp.1-30; CSDC Shenzhen (2025-03) §1-§3 pp.1-30.

```
CB position state (per beneficial owner, per CB):
  Q(t) = q_unrestricted(t) + q_restricted(t)
  
  q_unrestricted: freely-tradable; can flow through SSE/SZSE
                  matching engines
  q_restricted:   lock-up-period CBs; CSDC-level-blocked from
                  exchange-trade settlement until t ≥ t_unlock

position-flow operations:

  (A) Depository-native operations (per CSDC business guide):
    Δ_directional(t)     : CSDC-side transfer; bypasses exchange
                           matching; requires legal-basis validation
    Δ_lock_unlock(t)     : restricted ↔ unrestricted classification
                           transition at lock-up boundary

  (B) Holder-application operations (per SZSE §32 fixed-sequence):
    Δ_trade_or_transfer(t): exchange-matched trade or holder-
                            requested transfer; processed FIRST
    Δ_putback(t)          : holder-exercised put-back; processed
                            SECOND
    Δ_conversion(t)       : holder-exercised conversion; processed
                            THIRD
    Δ_custody_transfer(t) : beneficial owner unchanged; only the
                            custodian (broker) changes; processed
                            LAST

  (C) Issuer-channel operations (separate from §32 sequence):
    Δ_redemption(t)       : maturity / strong-call redemption flows
                            from issuer account to CB-holder
                            settlement account; NOT part of the
                            holder-application sequence
```

**Source:** CSDC Shanghai (2023) §2-§3 pp.10-30; CSDC Shenzhen
(2025-03) §2-§3 pp.10-30; SZSE Guideline 15 (2025) §32 pp.11-12.

The §32 fixed-sequence rule implies a strict processing-order
identity over the four holder-application types: for any single
trading day `t` with multiple holder applications on the same
position, the depository updates the position state in exactly
the prescribed sequence. **Source:** SZSE Guideline 15 (2025) §32
pp.11-12; CSDC Shanghai (2023) §3 pp.25-30.

```
holder-application processing order on day t (§32):
  Q(t, after-applications) = Q(t, start-of-day)
                           + Δ_trade_or_transfer(t)
                           + Δ_putback(t)
                           + Δ_conversion(t)
                           + Δ_custody_transfer(t)
                           
issuer-channel flows (separate accounting):
  Δ_redemption(t) and Δ_directional(t) update Q(t) outside the
  §32 sequence per their respective rule layers
```

The implication for the holder: if multiple applications are
filed on the same day, the deterministic sequence means later-
sequence applications see the already-updated position from
earlier-sequence applications. A practical example surfaces in
the put-back-vs-conversion choice: because put-back processes
BEFORE conversion in the §32 sequence, a holder who files both
on the same day will see the put-back exhaust the position first;
the conversion applies only if the put-back is partially or fully
rejected. To get a different ordering, the holder must split the
filings across consecutive trading days. **Source:** SZSE
Guideline 15 (2025) §32 pp.11-12.

The restricted-vs-unrestricted classification implies a flow
constraint on the trading-engine: at any time `t` the CSDC blocks
any sell-side exchange order whose order-quantity exceeds
`q_unrestricted(t)`. **Source:** CSDC Shenzhen (2025-03) §3
pp.20-25.

## See Also

- [`cb-china-csdc-settlement-mechanics.md`](cb-china-csdc-settlement-mechanics.md) — the exchange-rule-side trading and settlement conventions (SSE/SZSE rule layer); this card extends to depository-side operations
- [`cb-china-trading-mechanics.md`](cb-china-trading-mechanics.md) — practitioner trading-mechanic baseline
- [`cb-china-szse-2025-cb-self-regulation-update.md`](cb-china-szse-2025-cb-self-regulation-update.md) — primary source for the SZSE Guideline 15 §32 holder-application same-day fixed-sequence rule that the depository operationally implements
- [`cb-china-distressed-workouts.md`](cb-china-distressed-workouts.md) — distressed-CB scenarios where the holder-application sequencing affects put-back-vs-conversion choice
- [`cb-investor-clientele.md`](cb-investor-clientele.md) — investor-clientele taxonomy where the restricted-vs-unrestricted classification constrains newly-issued CB float

## Escalate to Raw When

Open CSDC Shanghai 结算账户管理及资金结算业务指南 (Nov 2023) §1-§3
pp.1-30 directly for the depository-side cash + settlement-account
management rules and the directional CB transfer mechanic at the
Shanghai branch. The same-day holder-application processing
sequence is primary-sourced from SZSE Guideline 15 §32; the CSDC
guides only operationally implement it. **Source:** CSDC Shanghai
(2023) §1-§3 pp.1-30; SZSE Guideline 15 (2025) §32 pp.11-12.

Open CSDC Shenzhen 证券发行人业务指南 (2025-03) §1-§3 pp.1-30 for
the issuer-side depository business management rules, the
restricted-vs-unrestricted share-nature classification, and the
issuer-account operational mechanics specific to the Shenzhen
branch. **Source:** CSDC Shenzhen (2025-03) §1-§3 pp.1-30.

Open the SZSE 2022 trading rule 深证上[2022]719号 §1-§14 + SSE
compilation §三(一) pp.450-466 for the still-controlling
exchange-rule-side conventions that this depository-layer card
sits below. **Source:** SZSE 2022 rule §1-§9 pp.1-9; SSE
compilation pp.450-466.
