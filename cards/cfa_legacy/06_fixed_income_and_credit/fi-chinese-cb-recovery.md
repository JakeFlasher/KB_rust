---
schema_version: "cacg.v0"
id: "fi-chinese-cb-recovery"
title: "Senior-Unsecured Convertible-Bond Recovery in the Chinese Post-2014 Default Cohort"
reading_id: "06_fixed_income_and_credit"
summary: "Applies Lando's reduced-form recovery framework to senior-unsecured CB recovery in the post-2014 Chinese-CB default cohort: structural-seniority position of CB claims, post-default cash-flow timing, and the legal-recovery framework governing LGD realization in Chinese bankruptcy."
tags: ["fixed-income", "chinese-cb"]
citations:
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p130:0155"
    chunk_hash: "c2c4d2d53d7fe9ea84b5a2edc24903ae3f46e0eadba923753b922e916ede102a"
    page_range: [130, 130]
    quote: "To better understand what the intensity model tries to achieve, let us briefly recall the notion of a hazard rate and its link to conditional default probabilities."
    edge_type: "supports"
  - source_id: "china_cb_lianhe_risk_reconstruction_2025"
    chunk_id: "china_cb_lianhe_risk_reconstruction_2025:p001:0000"
    chunk_hash: "5cdfc043bc31f662c359fa75245feae1cbab0ed39ea9f200921714640444d435"
    page_range: [1, 2]
    quote: "www.lhratings.com 研究报告 1 可转债风险重构与应对—基于风险案例 与退市新规视角 联合资信 工商评级二部 杨学慧 王阳 2024 年成为可转债市场的重要转折点，长期以来的零违约格局被打破"
    edge_type: "supports"
  - source_id: "china_cb_dajia_credit_risk_immune_2024"
    chunk_id: "china_cb_dajia_credit_risk_immune_2024:p003:0001"
    chunk_hash: "9b5d9a29fc02e78a31d0c38abe45083141ab04494b31047b5e03f44e4119429d"
    page_range: [3, 4]
    quote: "114 《金融市场研究》2024.03 VOL.142 风险 管理 Risk Management 在已摘牌的 4731 只可转债中，有 15 只为部 分转股，有 7 只触发回售条款且以回售为 主要退出方式，有 61 只以持有至到期为主 要退出方式（其中 58 只以接近 100% 的比 1 数据来源于Wind数据库“债券-可转债分析-可转债专题-退出方式统计”，统计区间为1990—2023年"
    edge_type: "supports"
  - source_id: "china_cb_spc_bond_disputes_2020"
    chunk_id: "china_cb_spc_bond_disputes_2020:p001:0000"
    chunk_hash: "235c5b5c6a6940077f81fa446cd0d12c8179e922ec899801f9edca81174ed70b"
    page_range: [1, 2]
    quote: "最高人民法院 2020 年 7 月 15 日 全国法院审理债券纠纷案件座谈会纪要 近年来，我国债券市场发展迅速，为服务实体经济发展和国家重点项目建设提供了 有力的支持和保障"
    edge_type: "supports"
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p078:0090"
    chunk_hash: "5aef6a0cf0795b51f38105ad235a3e309266944a2793099970ff375b023af13b"
    page_range: [78, 79]
    quote: "Endogenous Default Boundaries and Optimal Capital Structure 3.1 Leland’s Model The following model is presented in Leland (1994) and Leland and Toft (1996)."
    edge_type: "supports"
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p129:0144"
    chunk_hash: "f64396a05a7a4c942689002ae0afa6291abf759a9c9bc751337c99daa28360eb"
    page_range: [129, 130]
    quote: "The arbitrageur must constantly measure the position hedge ratio and compare it to the convertible’s current theoretical delta."
    edge_type: "supports"
card_hash: "259ba46717ecaeb4efa262d8dae9a48ac48b4be5a94a1cbf4cc0d006b16f49e8"
---
# Senior-Unsecured Convertible-Bond Recovery in the Chinese Post-2014 Default Cohort

## Intuition

The 08 vertical's [cb-china-distressed-workouts](../08_convertible_bonds/cb-china-distressed-workouts.md)
card carries the per-case practitioner taxonomy for the 6-rung CB-
workout escalation (条款博弈 -> 提前赎回 -> 引入战投 -> 债务重组 ->
破产清算 -> 退市联动). The 06 vertical's recovery-specific companion
card re-frames the same cohort through the **fixed-income recovery-
modeling lens**: a convertible bond's recovery in a defaulted issuer
is governed by (a) the structural seniority of the CB claim against
other creditor classes, (b) the post-default recovery cash-flow
timing as determined by the bankruptcy / restructuring procedure,
and (c) the legal-recovery framework that maps the issuer's residual
asset value into a concrete LGD realization for CB holders. Lando's
reduced-form credit-risk framework provides the formal model for the
recovery process; the Chinese-CB case studies provide the empirical
anchor for calibration. The boundary against 08 is precise: 08 owns
the practitioner workout taxonomy and per-case sequence; 06 owns the
formal recovery-modeling layer and the credit-spread implications.
**Source:** Lando (2004) Ch.6 pp.130-200; Lianhe (2025) pp.1-15
(Chinese-market post-2014 recovery empirics).

```
   Chinese-CB recovery framework — 08 vs 06 framing
   ------------------------------------------------

   08 vertical (workout taxonomy):
     +-----------------------------------+
     | 6-rung distressed CB escalation:  |   <- 08 owns
     |   condition-clause gaming         |
     |   early redemption                 |
     |   strategic investor               |
     |   debt restructuring               |
     |   bankruptcy liquidation           |
     |   delisting linkage                |
     | per-case sequence                  |
     +-----------------------------------+
                       |
                       v cross-link
   06 vertical (recovery modeling):
     +-----------------------------------+
     | CB recovery process:              |   <- 06 owns
     |   structural seniority             |
     |   recovery-cash-flow timing        |
     |   LGD calibration from cohort      |
     |   credit-spread implications       |
     | Lando reduced-form recovery model  |
     +-----------------------------------+
```

**Source:** Lando (2004) Ch.6 pp.130-200; Lianhe (2025) pp.1-15.

## Definition

The **convertible-bond recovery process** in a defaulted Chinese
issuer is governed by three jointly-determined quantities at the
claim level: (a) the **structural seniority** of the CB claim (in
Chinese onshore practice, CBs rank as senior unsecured corporate
debt, behind secured / preferred claims but ahead of equity), (b)
the **recovery-cash-flow timing** (the time elapsed between default
event and recovery realization, which feeds the discount-PV factor
in the LGD formula), and (c) the **legal-recovery framework** under
the Supreme People's Court (2020) symposium ruling that formalizes
acceleration of CB principal on event-of-default, treatment of CB
holders as senior unsecured creditors in restructuring, and the
sequencing of recovery distributions. Lando's reduced-form framework
maps these three components into a recovery-rate process R(t)
calibrated to empirical cohort data. **Source:** Lando (2004) Ch.6
§6.1-§6.3 pp.130-180 (reduced-form recovery framework); SPC (2020)
pp.1-10 (Chinese-market legal-recovery framework).

## Mathematical Reasoning

The reduced-form recovery framework decomposes the realized recovery
into a process discounted to the default event. **Source:** Lando
(2004) Ch.6 pp.130-200.

```
   recovery at default time tau:
     R(tau) = realized recovery rate (fraction of face)

   discounted recovery PV (recovery of market value convention):
     RecoveryPV(tau) = R(tau) * V_pre_default(tau-) * D_rf(0, tau + delta)
     where delta is the recovery delay (time from default to realization)

   LGD definition:
     LGD = 1 - R(tau)
     E[LGD] = 1 - E[R(tau)]                              (cohort mean)
     Var[LGD] = Var[R(tau)]                              (cohort dispersion)

   structural seniority constraint:
     R_CB(tau) >= R_subordinated(tau)
     R_CB(tau) <= R_secured(tau)
     subject to: bankruptcy waterfall ranking
```

The **Chinese-market structural-seniority position** of CBs is
between secured / preferred claims (higher recovery) and equity
(zero recovery). Lando's framework predicts the cohort's R_CB(tau)
distribution as a two-branch mixture: a high-recovery branch when
the issuer's asset value at default exceeds the senior-unsecured
claim aggregate (in which case the CB recovery approaches face
value minus haircut), and a low-recovery branch when the asset
value falls below that threshold (in which case the CB recovery
scales linearly with the asset shortfall). The post-2014 Chinese-CB
cohort, as documented qualitatively by Lianhe (2025) and NAFMII
Dajia (2024), maps onto this two-branch mixture in the workout-
taxonomy sense: defaults resolved via debt-restructuring workouts
fall in the higher-recovery branch (continued-issuer scenario);
defaults associated with delisting and bankruptcy liquidation fall
in the lower-recovery branch. Lianhe and NAFMII Dajia report
qualitative case-by-case workout outcomes rather than a calibrated
recovery-rate distribution; the two-branch mixture is Lando's
theoretical framework that this card uses to interpret the
qualitative cohort empirics.
**Source:** Lando (2004) Ch.6 §6.2 pp.140-170 (recovery-process
mixture decomposition); Lianhe (2025) pp.1-15 (qualitative cohort
workout-outcome cases).

The **recovery-cash-flow timing** is a Chinese-market-specific
refinement of the standard Lando framework: the SPC (2020) ruling
formalizes a multi-stage recovery sequence in which CB principal
is accelerated on event-of-default, then a structured negotiation /
restructuring window precedes any distribution to bondholders. This
recovery delay delta appears in the recovery-PV discount factor and
reduces the present value of recovery relative to face value. The
SPC framework provides the legal-recovery anchor for Chinese-market
LGD calibration; the cohort case-by-case timing varies, with longer
delays observed qualitatively in the bankruptcy-liquidation rung of
the 08-vertical workout taxonomy and shorter delays in the
debt-restructuring rung. **Source:** SPC (2020) pp.1-10 (multi-
stage recovery sequence + acceleration on event-of-default);
NAFMII Dajia (2024) pp.1-9 (qualitative cohort workout-stage
description).

The **credit-spread implications** of the cohort recovery
distribution feed back into the [fi-credit-spread-machinery](./fi-credit-spread-machinery.md)
framework: the bond's pre-default credit spread is the expected-loss
PV per unit time scaled by the discount factor; with cohort R_CB(tau)
distribution as the empirical anchor, the implied PD term-structure
for the surviving CB cohort can be backed out from the observed
yield-to-default-event under the standard reduced-form pricing
formula. The [fi-default-models-and-recovery](./fi-default-models-and-recovery.md)
card carries the generic single-name PD / R(tau) calibration; this
card specializes the calibration to the Chinese-CB cohort empirics.
**Source:** Lando (2004) Ch.3 + Ch.6 pp.60-130 + pp.130-200 (single-
name calibration + recovery model); Calamos (2003) §5 pp.130-170
(CB credit-equity decomposition with recovery treatment).

Asymptotic behaviour of the recovery process under the three
structural-seniority regimes follows three patterns from Lando's
theoretical framework. Senior-unsecured-dominated regime: when the
issuer's asset value at default substantially exceeds the
secured-claim aggregate, the CB recovery approaches face value
minus a haircut; this is the higher-recovery theoretical branch
of Lando's two-branch mixture. Asset-deficiency regime: when asset
value falls below the secured-claim aggregate but above the
senior-unsecured aggregate, the CB recovery scales linearly with
the asset surplus over secured claims; this is the middle-branch
theoretical case typical of debt-restructuring workouts in the
qualitative Chinese-CB workout taxonomy. Equity-dominated regime:
when asset value falls below the senior-unsecured aggregate, the
CB recovery approaches zero and the loss distribution behaves like
an equity position; this is the lower-recovery theoretical branch
typical of delisting + bankruptcy liquidation cases qualitatively
described in Lianhe / NAFMII Dajia's case-by-case workout outcomes.
**Source:** Lando (2004) Ch.6 pp.130-200; SPC (2020) pp.1-10.

## See Also

- [[cb-china-distressed-workouts]] — [`../08_convertible_bonds/cb-china-distressed-workouts.md`](../08_convertible_bonds/cb-china-distressed-workouts.md) — 08-vertical paired card carrying the 6-rung distressed-CB workout taxonomy (the practitioner-side process whose recovery implications this card models)
- [[cb-china-default-cohort-attribution]] — [`../08_convertible_bonds/cb-china-default-cohort-attribution.md`](../08_convertible_bonds/cb-china-default-cohort-attribution.md) — 08-vertical sibling for the post-2014 default-cohort case-study taxonomy
- [`fi-default-models-and-recovery.md`](fi-default-models-and-recovery.md) — generic single-name PD / R(tau) calibration that this card specializes to the Chinese-CB cohort
- [`fi-credit-spread-machinery.md`](fi-credit-spread-machinery.md) — credit-spread framework that the cohort recovery distribution feeds into
- [`fi-credit-risk-fundamentals.md`](fi-credit-risk-fundamentals.md) — CFA L1 baseline for credit-risk metrics

## Escalate to Raw When

Open Lando (2004) Ch.6 §6.1-§6.3 pp.130-200 directly for the
formal reduced-form recovery framework: recovery-process mixture
decomposition, recovery-of-market-value convention, recovery-cash-
flow PV mechanics. Open Lando Ch.3 pp.60-130 for the single-name
PD + R(tau) calibration that this card specializes. Open Lianhe
(2025) pp.1-15 for the Chinese-market post-2014 cohort case-by-case
workout outcomes (qualitative case ledger of recovery pathways
across debt-restructuring vs delisting + bankruptcy rungs) and the
2027 maturity-cliff projection. Open NAFMII Dajia (2024) pp.1-9 for
the 4-stage life-cycle taxonomy and qualitative workout-stage
description across the Chinese-CB cohort. Open SPC (2020) pp.1-10
for the Chinese-market legal-recovery framework (acceleration on
default, structured negotiation window, distribution sequencing).
Open Calamos (2003) §5 pp.130-170 for the cross-jurisdictional
CB credit-equity decomposition with recovery treatment. **Source:**
Lando (2004) Ch.3 + Ch.6 pp.60-200; Lianhe (2025) pp.1-15; NAFMII
Dajia (2024) pp.1-9; SPC (2020) pp.1-10; Calamos (2003) §5 pp.130-170.
