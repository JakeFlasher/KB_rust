---
schema_version: "cacg.v0"
id: "cb-china-default-cohort-attribution"
title: "China CB Default Cohort: Life-cycle Credit-Risk Attribution"
reading_id: "08_convertible_bonds"
summary: "The Dajia (大家资产) 2024 white paper rebuts the long-held \"CB immunity\" claim by showing that mature-market data sets CB default rates on par with high-yield bonds and by attributing post-2014 onshore CB credit risk across the bond life-cycle (上市初期 debt-side risk; 转股期 strike-revision game; 强赎促进 forced-conversion pressure; final two coupon years redemption-pressure). The Lianhe 2025 reconstruction ..."
tags: ["convertible-bonds", "china-default"]
citations:
  - source_id: "china_cb_dajia_credit_risk_immune_2024"
    chunk_id: "china_cb_dajia_credit_risk_immune_2024:p001:0000"
    chunk_hash: "2e0dbc24f972c53421e0c8584deb7f31f8726d628b28bbe6e72fd4de00a94ebf"
    page_range: [1, 2]
    quote: "根据成熟市场的统计数据，可转债不仅存在违约，且违约率与高收益债相当。"
    edge_type: "defines"
  - source_id: "china_cb_lianhe_risk_reconstruction_2025"
    chunk_id: "china_cb_lianhe_risk_reconstruction_2025:p001:0000"
    chunk_hash: "5cdfc043bc31f662c359fa75245feae1cbab0ed39ea9f200921714640444d435"
    page_range: [1, 2]
    quote: "2024 年成为可转债市场的重要转折点，长期以来的零违约格局被打破"
    edge_type: "supports"
  - source_id: "china_cb_lianhe_h1_default_review_2025"
    chunk_id: "china_cb_lianhe_h1_default_review_2025:p001:0000"
    chunk_hash: "8f45e9d9a7c3448996f95f07f67a8894671e3ae0fad088a837de4b0dec58ef49"
    page_range: [1, 2]
    quote: "2025 年上半年，评级负面调整 有所减少，仍主要集中在可转债发行人。"
    edge_type: "supports"
  - source_id: "china_cb_spc_bond_disputes_2020"
    chunk_id: "china_cb_spc_bond_disputes_2020:p003:0002"
    chunk_hash: "e75d1e39529b473a129eab4e58b149c2bc50b7936b2e3d71fd08c44196eb16a6"
    page_range: [3, 4]
    quote: "要充分发挥债券持有人会议的议事平台作用"
    edge_type: "supports"
card_hash: "a5472290b4550c76b7db7f4d2ff87c7f081a6af3080f53784f65485df41946f3"
---
# China CB Default Cohort: Life-cycle Credit-Risk Attribution

## Intuition

Through 2023, China's onshore CB market displayed a **zero formal-default
record**, sustained by the conversion-route exit pattern: most issuers
either successfully promoted equity-route conversion (via 下修 + 强赎)
or used residual cash to honour 回售 / 到期赎回 obligations on the
non-converted residual. The post-2014 distressed cohort (蓝盾 / 鸿达 /
搜特 / 全筑 / 蓝盾退债 / 搜特退债, plus the 2024-2025 wave) shows that
this norm broke when **issuer credit deteriorated faster than the
conversion option could capture residual equity value**. The Dajia AM
credit desk frames the systematic risk via a four-stage life-cycle
taxonomy. **Source:** Dajia AM (2024) §I pp.1-3 (NAFMII Vol.142).

```
CB life-cycle credit-risk taxonomy (Dajia AM 2024)

  Stage 1: 上市初期 (initial listing window)
     |        - debt-like behaviour; investors face credit risk
     |        - similar to ordinary corporate bonds
     |        - default not yet structurally feasible (conversion option
     |          dormant until +6 months)
     ▼
  Stage 2: 转股期 (conversion window, post +6 months)
     |        - equity-like behaviour as S → K_c
     |        - issuer pulls 下修 / 强赎 (Lianhe's Rung 1) to promote
     |          conversion
     |        - debt-route credit risk receding if conversion succeeds
     ▼
  Stage 3: 强赎促进 (strong-call-promoted conversion)
     |        - if promotion succeeds, CB exits via equity route;
     |          credit risk effectively retired
     |        - if promotion fails, CB enters Stage 4
     ▼
  Stage 4: 最后两个计息年度 (final 2 interest years)
              - 回售 / 到期赎回 obligations bind
              - weakened credit profile → high credit risk
              - this is where the post-2014 cohort defaults concentrate
```

The post-2014 cohort attribution maps each defaulted issuer onto one of
the Rung-1-through-Rung-6 paths catalogued in `cb-china-distressed-workouts`.
The mapping is empirical (NAFMII Dajia + Lianhe case taxonomies) rather
than analytical; no closed-form selection rule exists. **Source:** Dajia
AM (2024) §I pp.1-3; Lianhe (2025) §1 pp.5-15.

## Definition

The **post-2014 Chinese-CB default cohort** consists of the convertible
bonds whose issuers reached substantive credit-deterioration outcomes —
explicit interest default, rating-downgrade-to-default, restructuring,
or delisting — between 2014 and the current frontier. **Source:** Dajia
AM (2024) §I pp.1-3.

### Credit-quality downshift relative to ordinary bonds

Per Dajia AM's 2023 issuance statistics, ordinary corporate-bond
issuers concentrate at the top three rating grades (AAA / AA+ / AA),
while CB issuers downshift by 2-3 sub-grades to AA / AA- / A+ as the
modal-rating cluster. This systematic downshift means CB issuers carry
materially higher unconditional credit risk than ordinary-bond peers,
even while the conversion-route exit empirically masks the realised
default rate. **Source:** Dajia AM (2024) §II pp.3-5.

### Four life-cycle stages of CB credit-risk exposure

The four stages are inventoried below in order of credit-risk-binding
intensity. **Source:** Dajia AM (2024) §I-§III pp.1-7.

**Stage 1 — 上市初期**: from issuance through the +6-month no-conversion lockout, the CB behaves as ordinary debt; investors face standard corporate-bond credit risk; default is structurally feasible but rare in this window. **Stage 2 — 转股期**: post-lockout, investors may convert on any trading day, and the issuer's optimal lever set includes the contractual 下修 + 强赎 mechanisms (covered in [cb-china-downward-conversion](./cb-china-downward-conversion.md) and [cb-china-call-redemption-rules](./cb-china-call-redemption-rules.md)); conversion promotes if S/K_c improves, otherwise the CB drifts toward Stage 4. **Stage 3 — 强赎促进**: the issuer pulls clause-game-play to force conversion (Lianhe Rung 1 in [cb-china-distressed-workouts](./cb-china-distressed-workouts.md)); success exits the CB via the equity route, failure escalates to Stages 4-6 of the workout ladder. **Stage 4 — 最后两个计息年度**: in the final two interest-payment years, 回售 and 到期赎回 obligations bind — investors holding the non-converted residual can put the bond back, and the issuer must honour the put or face explicit interest / principal default. The cohort defaults concentrate here. **Source:** Dajia AM (2024) §I-§III pp.1-9.

### Cohort attribution rule

For each defaulted issuer in the post-2014 cohort, attribution maps to a (stage, rung) pair: which life-cycle stage the issuer reached at the default event, and which workout-ladder rung was the operative exit path. The flagship examples documented by Lianhe + Dajia + SPC all sit at the final-interest-year stage along distinct workout rungs: **蓝盾退债** exited via bankruptcy with synchronized equity delisting (bankruptcy-plus-delisting rung); **搜特退债** experienced explicit interest default in early 2024 followed by the bankruptcy-plus-delisting rung with timed default-event linkage; **鸿达转债** exited via debt restructuring along the restructuring rung; **全筑转债** exited via bankruptcy-restructuring delivering a mixed cash-plus-equity-in-kind-plus-trust-receipt claim along the restructuring rung with bankruptcy-rung fall-through. **Source:** Lianhe (2025) §1 pp.5-15; Dajia AM (2024) §III pp.5-9.

## Mathematical Reasoning

Let `τ_default` denote the random default time and `τ_conv` the random
voluntary-conversion time for a holder. The cohort default rate is then
`P(τ_default < τ_conv ∧ τ_default < T_maturity)`. The Dajia AM thesis
is that **conversion-route success masks but does not eliminate the
underlying default intensity**: even when realised defaults are rare,
the issuer's unconditional default intensity λ(t) remains material
(consistent with the 2-3 sub-grade rating downshift). **Source:** Dajia
AM (2024) §II pp.3-5; Lando (2004) Ch.2-3 pp.100-200 (reduced-form
default-intensity framework).

```
cohort default-time decomposition:

  P(τ_default < T) ≈ ∫_0^T  λ(t) · S(t)_survival · dt
                       │
                       └── λ(t) reflects issuer credit profile
                            (CB issuer credit ≈ AAA - 2~3 sub-grades)

  But realised default rate:
  P(τ_default < min(τ_conv, T))
                       │
                       └── conversion competes with default
                            → (S/K_c) high → τ_conv ≪ τ_default → default ≈ 0
                            → (S/K_c) low → τ_default ≪ τ_conv → default high
```

The **stage-binding asymmetry** is structural: the conversion option's
hazard rate `λ_conv(t)` is bounded below by 0 in Stage 1 (lockout), peaks
in Stages 2-3 (post-lockout with active issuer promotion), and decays
toward 0 in Stage 4 as the maturity-truncation effect dominates. The
default hazard `λ_default(t)` is roughly stage-monotone — issuers
defaulting in Stage 1 are uncommon, and Stage 4 captures the bulk of
the cohort. The **cross-cohort homogeneity** of Stage-4 default
concentration suggests that the conversion-route exit pattern is
structurally biased toward Stage 2-3 success cases; the failures that
drive the realised default cohort are systematically those where the
issuer reaches Stage 4 without successful prior promotion. This is the
empirical observation that the Dajia AM credit desk frames as "CBs are
not immune to credit risk" (the paper's title question). **Source:**
Dajia AM (2024) §I, §III, §IV pp.1-9; Lando (2004) Ch.3 pp.150-200
(hazard-rate piecewise structure).

## See Also

- [`cb-china-distressed-workouts.md`](cb-china-distressed-workouts.md) — the 6-rung workout-ladder taxonomy that each defaulted issuer in this cohort traversed (Rung selection complements the life-cycle stage attribution here)
- [`cb-default-and-recovery.md`](cb-default-and-recovery.md) — general Lando-anchored default + recovery machinery that the Chinese-CB cohort instantiates
- [`cb-credit-spread-machinery.md`](cb-credit-spread-machinery.md) — hazard-rate framework for the unconditional default intensity used in the cohort decomposition
- [`rm-chinese-cb-default-cohort-attribution.md`](../11_risk_management/rm-chinese-cb-default-cohort-attribution.md) — sibling card (authored Round 10) under subcorpus 11 carrying the cross-vertical operational-risk + portfolio-threshold framing via McNeil portfolio-credit-risk machinery
- [`fi-chinese-cb-recovery.md`](../06_fixed_income_and_credit/fi-chinese-cb-recovery.md) — sibling card (authored Round 10) under subcorpus 06 carrying Lando reduced-form recovery analysis for the same default cohort

## Escalate to Raw When

Open Dajia AM (2024) NAFMII Vol.142 pp.1-9 directly for the
practitioner-credit-analyst framing of why CBs are not "immune" to
credit risk: 4-stage life-cycle taxonomy + 2-3 sub-grade rating
downshift + comparison with mature-market default rates. Open
Lianhe (2025) pp.1-15 for the case-by-case Rung-selection mapping
(which post-2014 default issuer ended on which workout-ladder rung +
2024-2025 default cohort + 退市新规 impact + 2027 maturity-cliff
projection). Open Supreme People's Court (2020) pp.1-10 for the
judicial-framework backbone of bond-dispute litigation when a Chinese-
CB issuer enters Rung 5 (bankruptcy liquidation). Open Lando (2004)
Ch.2-3 pp.100-200 for the reduced-form default-intensity machinery
(hazard rate, default intensity, defaultable term structure) that the
cohort decomposition uses; and Calamos (2003) §6 pp.130-170 for the
general practitioner framing of distressed-CB credit dynamics that
transfers to the Chinese-market cohort. Open Lianhe 2025 H1 default
review pp.1-15 for empirical confirmation of the maturity-cliff
projection through the latest cohort + outlook period, and the NAFMII
2025 China bond market reform report pp.30-50 for the macro-regulatory
context that has reshaped Chinese CB issuance + secondary-market
liquidity since the prior cohort attribution was authored.
**Source:** Dajia AM (2024) §I-§IV pp.1-9; Lianhe (2025) §1 pp.1-15;
Lianhe 2025 H1 default review §1 pp.1-15; NAFMII 2025 reform report
pp.30-50; Supreme People's Court (2020) §I-§V pp.1-10; Lando (2004)
Ch.2-3 pp.100-200; Calamos (2003) §6 pp.130-170.
