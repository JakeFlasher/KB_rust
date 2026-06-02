---
schema_version: "cacg.v0"
id: "rm-chinese-cb-default-cohort-attribution"
title: "Chinese CB Default-Cohort Attribution Under McNeil Portfolio Credit-Risk Framework"
reading_id: "11_risk_management"
summary: "Reframes the post-2014 Chinese convertible-bond default cohort under McNeil's Ch.11 threshold + mixture-model portfolio-credit-risk framework: defaults attributed to issuer credit-quality drift, sector concentration, and macro-cycle dependence rather than purely idiosyncratic events."
tags: ["risk-management", "chinese-cb"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p446:0645"
    chunk_hash: "b84ba0083c486660fe9e53c82783f633726c63632b9f91aaace10bd081bb16a0"
    page_range: [446, 447]
    quote: "Since different firms are affected by common macroeconomic factors, this creates dependence between their defaults."
    edge_type: "defines"
  - source_id: "china_cb_dajia_credit_risk_immune_2024"
    chunk_id: "china_cb_dajia_credit_risk_immune_2024:p003:0001"
    chunk_hash: "9b5d9a29fc02e78a31d0c38abe45083141ab04494b31047b5e03f44e4119429d"
    page_range: [3, 4]
    quote: "目前，公募可转债尚未出现 公开违约，但考虑到存量 6042 只可转债中 尚有 493 只可转债的未转股比例大于 90%， 整体未转股比例较高，叠加宏观经济下行、 信用风险加剧的背景，发行主体信用资质 较弱的可转债未来存在出现违约的可能性。"
    edge_type: "supports"
  - source_id: "china_cb_lianhe_risk_reconstruction_2025"
    chunk_id: "china_cb_lianhe_risk_reconstruction_2025:p001:0000"
    chunk_hash: "5cdfc043bc31f662c359fa75245feae1cbab0ed39ea9f200921714640444d435"
    page_range: [1, 2]
    quote: "2024 年成为可转债市场的重要转折点，长期以来的零违约格局被打破，2024－ 2025 年，5 只转债相继发生实质性违约，多行业评级下调事件频发，5 只转债随正股 同步退市"
    edge_type: "supports"
  - source_id: "china_cb_lianhe_h1_default_review_2025"
    chunk_id: "china_cb_lianhe_h1_default_review_2025:p001:0000"
    chunk_hash: "8f45e9d9a7c3448996f95f07f67a8894671e3ae0fad088a837de4b0dec58ef49"
    page_range: [1, 2]
    quote: "2025 年上半年，我国债券市场新增 7 家违约发行人，共涉及到期违约债券 15 期，到期违约金额合计约 140.20 亿元"
    edge_type: "supports"
  - source_id: "china_cb_lianhe_annual_bond_market_2025"
    chunk_id: "china_cb_lianhe_annual_bond_market_2025:p001:0000"
    chunk_hash: "9508b0fe8ea0123e3e4c9620113ee313ebac27c619bdda0d0a7c0780e43d66d5"
    page_range: [1, 2]
    quote: "信用风险呈现收敛态势。展望 2026 年，债市收益率有望维持低位波动，信 用利差或结构性分化；债券市场发行规模有望稳步增长；债券市场信用风险将持续收 敛，违约率或处于历史低位。"
    edge_type: "supports"
  - source_id: "china_cb_spc_bond_disputes_2020"
    chunk_id: "china_cb_spc_bond_disputes_2020:p001:0000"
    chunk_hash: "235c5b5c6a6940077f81fa446cd0d12c8179e922ec899801f9edca81174ed70b"
    page_range: [1, 2]
    quote: "为正确审理因公司债券、企业债券、非金融企业债务融资工具的发行和交易所引发 的合同、侵权和破产民商事案件，统一法律适用，保护债券投资人的合法权益"
    edge_type: "supports"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p393:0563"
    chunk_hash: "4c7786ff6cd0d657a0de2144cea63016980fabe51e035d9f5beecb0a2502f08a"
    page_range: [393, 394]
    quote: "In practice, the concept that is used to describe exposure is exposure at default or EAD, which recognizes that the exposure for many instruments will depend on the exact default time."
    edge_type: "supports"
card_hash: "e4bb4e3fb7f6dc8f2339ed3b1ea1612a5f3801984fcedb3c3731c4f9b1103f0a"
---
# Chinese CB Default-Cohort Attribution Under McNeil Portfolio Credit-Risk Framework

## Intuition

The 08 vertical's [cb-china-default-cohort-attribution](../08_convertible_bonds/cb-china-default-cohort-attribution.md)
card carries the practitioner case-study taxonomy for the post-2014
Chinese convertible-bond default cohort: 4-stage life-cycle staging,
the 蓝盾 / 鸿达 / 搜特 case sequence, the 2024 default ramp-up, and the
2027 maturity-cliff projection. The 11 vertical's risk-management
companion card re-frames this same cohort through the **portfolio-
aggregation lens** that McNeil owns: the cohort is not a sequence of
independent idiosyncratic single-name defaults but a **portfolio-
credit-risk realization** whose losses cluster across issuer credit-
quality drift, sector concentration, and macro-cycle dependence.
The boundary against 08 is precise: 08 owns the per-issuer / per-case
taxonomy; 11 owns the portfolio-level aggregation, the default-
correlation modeling, and the cohort risk-attribution at the
portfolio level. **Source:** McNeil et al. (2015) Ch.10 §10.4-§10.6
pp.425-470; NAFMII Dajia (2024) pp.1-9 (Chinese-market 4-stage
life-cycle taxonomy at the practitioner level).

```
   Chinese-CB default-cohort attribution — 08 vs 11 framing
   --------------------------------------------------------

   08 vertical (case-study taxonomy):
     +-----------------------------------+
     | per-issuer post-2014 cohort:      |   <- 08 owns
     |   2014 (鸿达, 神武 EB), 2019 (辉丰), |
     |   2023 (蓝盾, 搜特), 2024+ cohort   |
     | 4-stage life-cycle attribution    |
     | per-case workout taxonomy          |
     +-----------------------------------+
                       |
                       v cross-link
   11 vertical (portfolio-aggregation):
     +-----------------------------------+
     | cohort as portfolio realization:  |   <- 11 owns
     |   joint default distribution      |
     |   issuer credit-quality drift     |
     |   sector concentration            |
     |   macro-cycle dependence          |
     | McNeil threshold + mixture models |
     +-----------------------------------+
```

**Source:** McNeil et al. (2015) Ch.10 pp.425-470; Lianhe (2025) pp.1-15
(2024-2025 cohort + 2027 maturity-cliff projection).

## Definition

The **portfolio-credit-risk attribution** of the post-2014 Chinese-CB
default cohort decomposes the cohort's aggregate loss into three
risk-driver components per the McNeil threshold + mixture-model
framework: (a) **per-issuer marginal default risk** (single-name
PD_i drift across the cohort window), (b) **default dependence**
(joint-default tail behaviour under copula / mixture models), and
(c) **exposure concentration** (sector / industry / issuer-size
weighting that determines which cohort outcomes drive aggregate
losses). The cohort's empirical loss realization, as compiled
qualitatively by NAFMII Dajia (2024) and Lianhe (2025), maps onto
this decomposition: the 2024-2025 event cluster — the substantive
public CB default window — exhibits both component (a) per-issuer
PD drift in the single-name distress events and component (b)
candidate clustering across sector-aligned issuers within a narrow
calendar window, with component (b) read through this card's
McNeil-side inference rather than as a calibrated source claim;
the 2027 maturity-cliff projection introduces component (c)
(exposure-concentration in the early-2024 issuance vintage with
shared maturity-2027). **Source:** McNeil et al. (2015) Ch.10
§10.4 pp.425-456 (threshold + mixture models for portfolio credit
risk); NAFMII Dajia (2024) pp.1-9 (qualitative life-cycle taxonomy);
Lianhe (2025) pp.1-15 (2024-2025 case ledger + 2027 maturity-cliff
projection).

## Mathematical Reasoning

The portfolio-level loss decomposition uses McNeil's standard
threshold-model framing. **Source:** McNeil et al. (2015) Ch.10 §10.4
pp.425-456.

```
   per-issuer single-period loss:   L_i = EAD_i * LGD_i * 1_{default_i}

   portfolio loss across cohort:    L_cohort = Sum_{i in cohort} L_i

   threshold model:                  1_{default_i} = 1{ X_i <= d_i }
                                     X_i: latent ability-to-pay variable
                                     d_i: issuer-specific default threshold

   dependence layer:                 (X_1, ..., X_N) ~ Gaussian / t-copula
                                     calibrated to sector + macro factors

   cohort-level VaR:                 VaR_alpha(L_cohort) tracks α-quantile
                                     of portfolio loss distribution
```

The portfolio-level VaR is the natural risk measure for the
post-2014 Chinese-CB cohort because it captures both the marginal
defaults (component a) and the tail dependence (component b) in a
single number; the [rm-credit-var-portfolio](./rm-credit-var-portfolio.md)
card carries the formal derivation of the threshold-model VaR
estimator for a generic portfolio. **Source:** McNeil et al. (2015)
Ch.10 §10.4 pp.425-456; McNeil et al. (2015) Ch.10 §10.1 pp.367-374
(PD/LGD/EAD restatement for the portfolio context).

The **empirical Chinese-CB cohort** as documented by NAFMII Dajia
(2024) and Lianhe (2025) places substantive public CB defaults
predominantly in the calendar-year window spanning 2024 through
2025, with isolated pre-2024 case-study events. Lianhe's case
ledger names 搜特转债 (event in early 2024), 鸿达转债 and 蓝盾转债
(both in mid-2024), 岭南转债 (also mid-2024), and 中装2 (in
2025). Mapped to the portfolio framework, the cohort exhibits
three risk-attribution regularities. First, the per-issuer
PD_i drift across the cohort window is concentrated in the 2024-2025
event cluster, with each named issuer carrying a single-name distress
trajectory that the NAFMII Dajia 4-stage life-cycle taxonomy describes
at the issuer level. Second, the proximity of these 2024 defaults
within roughly a 5-month window (March-August 2024) and the shared
sector exposures (cyclically-exposed manufacturing and consumer-
discretionary names) suggest cross-name dependence rather than purely
independent events; this card interprets that empirical clustering
through the McNeil component (b) lens. Third, the post-2025 cohort
and the 2027 maturity-cliff projection introduce component (c)
exposure-concentration risk: the early-2024 issuance vintage
(a particular bond-vintage cohort with shared maturity-2027) creates
a concentrated EAD_i pool whose aggregate LGD_i * 1_{default}
realization, even under modest per-issuer PD_i, drives a substantial
tail-loss for the cohort.
**Source:** NAFMII Dajia (2024) pp.1-9 (4-stage life-cycle taxonomy
+ issuer-level empirical evidence); Lianhe (2025) pp.1-15 (2024-2025
event-date ledger + 2027 maturity-cliff projection + sector
concentration analysis).

The **default-dependence layer** (component b) is the most actively
researched portion of the McNeil portfolio-credit framework as
applied to the Chinese-CB cohort. THIS CARD'S MCNEIL-BASED INFERENCE
on the cited Lianhe (2025) empirical evidence is that the 2024-2025
clustering admits a t-copula-with-positive-tail-dependence
interpretation (joint-default distribution with fatter tails than
the Gaussian-copula benchmark), under which the observed proximity
of defaults within sector-aligned issuers is more probable than
under the independent-Bernoulli null. Lianhe itself reports
empirical event dates and qualitative sector exposures, not a
calibrated copula model; the t-copula reading is the McNeil-side
inference, not Lianhe's modeling claim.
The [rm-credit-risk-metrics-restatement](./rm-credit-risk-metrics-restatement.md)
card carries the PD/LGD/EAD per-issuer restatement layer that this
card aggregates over. **Source:** McNeil et al. (2015) Ch.10 §10.5
pp.456-470 (copula calibration for default dependence); Lianhe
(2025) pp.1-15.

The **legal-recovery layer** (a refinement of LGD_i) is sourced from
the Supreme People's Court 2020 symposium ruling on bond-dispute
case treatment, which governs the recovery cash flows realized by
the cohort's defaulted issuers. The legal-recovery framework
provides a Chinese-market-specific anchor for LGD_i calibration:
post-2020, the SPC framework formalizes acceleration of CB principal
on event-of-default, treatment of CB holders as senior unsecured
creditors in restructuring, and the timing of recovery cash flows.
This is a Chinese-market refinement of the generic LGD_i framework
in McNeil and feeds directly into the cohort's aggregate L_cohort
realization. **Source:** SPC (2020) pp.1-10 (legal-recovery framework
for CB defaults); McNeil et al. (2015) Ch.10 §10.1 pp.367-374
(LGD_i in the portfolio context).

Asymptotic behaviour of the portfolio-level VaR under the three
component regimes follows three patterns. Independent-default regime:
with no cross-name dependence (component b is zero), the portfolio
loss approaches a sum of independent Bernoulli * exposure variables;
the VaR scales sub-linearly in the cohort size by CLT-like arguments.
Clustered-default regime: under a t-copula with positive tail
dependence, the VaR grows super-linearly in the cohort size for the
alpha-tail (alpha >= 95%); this card's McNeil-side reading is that
the 2024-2025 Chinese-CB cohort lives in this regime (the t-copula
language is the McNeil-side interpretation, not a Lianhe source
fact). Concentration-dominated regime: when a small
subset of issuers carries disproportionate EAD_i weight (as in the
2024 issuance vintage), the cohort's tail loss converges to that
subset's joint-default scenario, and diversification benefits vanish;
this is the regime that the 2027 maturity-cliff projection occupies.
**Source:** McNeil et al. (2015) Ch.10 §10.4-§10.6 pp.425-470.

## See Also

- [[cb-china-default-cohort-attribution]] — [`../08_convertible_bonds/cb-china-default-cohort-attribution.md`](../08_convertible_bonds/cb-china-default-cohort-attribution.md) — 08-vertical paired card carrying the per-case taxonomy (per-issuer 4-stage life-cycle staging, 蓝盾 / 鸿达 / 搜特 case sequence, 2024 ramp-up, 2027 maturity-cliff)
- [[cb-china-distressed-workouts]] — [`../08_convertible_bonds/cb-china-distressed-workouts.md`](../08_convertible_bonds/cb-china-distressed-workouts.md) — 08-vertical sibling for the distressed-CB workout taxonomy (6-rung escalation: 条款博弈 -> 提前赎回 -> 引入战投 -> 债务重组 -> 破产清算 -> 退市联动)
- [`rm-credit-risk-metrics-restatement.md`](rm-credit-risk-metrics-restatement.md) — PD/LGD/EAD restatement at the portfolio level (the single-counterparty derivation is owned by 06)
- [`rm-credit-var-portfolio.md`](rm-credit-var-portfolio.md) — portfolio-level Credit-VaR estimator that the cohort-attribution framework uses
- [`rm-loss-distribution-anatomy.md`](rm-loss-distribution-anatomy.md) — generic loss-distribution decomposition that the cohort-attribution applies to the Chinese-CB-specific case

## Escalate to Raw When

Open McNeil et al. (2015) Ch.10 §10.4-§10.6 pp.425-470 directly for
the formal threshold + mixture model for portfolio credit risk: the
latent-variable framework, the Gaussian / t-copula dependence layer,
the cohort-level VaR derivation, and the asymptotic regimes (sub-
linear / linear / super-linear scaling). Open McNeil §10.1 pp.367-374
for the PD/LGD/EAD restatement that the portfolio aggregation
aggregates over. Open NAFMII Dajia (2024) pp.1-9 for the Chinese-CB
practitioner cohort taxonomy + 4-stage life-cycle empirical evidence
through 2024. Open Lianhe (2025) pp.1-15 for the 2027 maturity-cliff
projection + sector-concentration analysis of the upcoming cohort.
Open SPC (2020) pp.1-10 for the legal-recovery framework that
governs the cohort's LGD_i realization in Chinese bankruptcy
procedure. **Source:** McNeil et al. (2015) Ch.10 pp.367-470; NAFMII
Dajia (2024) pp.1-9; Lianhe (2025) pp.1-15; SPC (2020) pp.1-10.
