---
schema_version: "cacg.v0"
id: "cb-china-distressed-workouts"
title: "China Convertible-Bond Distressed Workouts (post-2024 default cohort)"
reading_id: "08_convertible_bonds"
summary: "Distressed Chinese-CB workouts follow a six-step gradient — clause-bargaining (条款博弈: 下修/不强赎/回售) at the contractual layer, then 债务重组 (debt restructuring) and 破产清算 (bankruptcy liquidation), with delisting-linkage (退市联动) entangling 2024+ cases. The Lianhe 2025 reconstruction report frames this taxonomy on the heels of the five-default 2024-2025 cohort; the SPC 2020 bond-disputes summary supplies t..."
tags: ["convertible-bonds", "china-distressed"]
citations:
  - source_id: "china_cb_lianhe_risk_reconstruction_2025"
    chunk_id: "china_cb_lianhe_risk_reconstruction_2025:p001:0000"
    chunk_hash: "5cdfc043bc31f662c359fa75245feae1cbab0ed39ea9f200921714640444d435"
    page_range: [1, 2]
    quote: "可转债风险处置呈现“常规化解-极端处置”的梯度路径体系"
    edge_type: "defines"
  - source_id: "china_cb_lianhe_credit_risk_resolution"
    chunk_id: "china_cb_lianhe_credit_risk_resolution:p001:0000"
    chunk_hash: "21819e5ea341f3b1f10883300ddb9dff46047d614f47860a1e16c4852f037e70"
    page_range: [1, 2]
    quote: "从传统的回收处置方式来看，破产重整和债务重组是 违约可转债常见的处置方式。"
    edge_type: "supports"
  - source_id: "china_cb_spc_bond_disputes_2020"
    chunk_id: "china_cb_spc_bond_disputes_2020:p003:0002"
    chunk_hash: "e75d1e39529b473a129eab4e58b149c2bc50b7936b2e3d71fd08c44196eb16a6"
    page_range: [3, 4]
    quote: "保障受托管理人和 其他债券代表人能够履行参与诉讼、债务重组、破产重整、和解、清算等债券持有人会 议赋予的职责"
    edge_type: "supports"
  - source_id: "china_cb_dajia_credit_risk_immune_2024"
    chunk_id: "china_cb_dajia_credit_risk_immune_2024:p007:0003"
    chunk_hash: "904e8bc789e46ae335c16e80a4b0359aab08e1f34ac8ab54d5854fca77548cef"
    page_range: [7, 8]
    quote: "如果正股因公司破产清 算而退市，同步退市可转债的未转股部分， 将被作为破产债权加速到期。"
    edge_type: "supports"
card_hash: "efc33ec41b991925796973115547c4c7f7a9fd8d8312443d8eba79dd5a206063"
---
# China Convertible-Bond Distressed Workouts (post-2024 default cohort)

## Intuition

Through 2023, China's onshore convertible-bond market had a **zero
formal-default record**: every CB had either converted, been called,
been put back, or matured-with-cash-redemption. **2024 broke this
norm**: in 2024-2025, five convertible bonds reached substantive default
status (multiple-bond rating downgrades, five issues delisted alongside
their underlying equity), and the regulatory delisting-rule revision
(`退市新规`) tightened the gradient from distress to delisting. Lianhe
Ratings frames the workout space as a **"常规化解 — 极端处置" gradient**:
a six-step ladder from contractual-clause manoeuvres at the gentle end
to bankruptcy + delisting at the punitive end. **Source:** Lianhe (2025)
§1 pp.1-5.

```
distressed-CB workout ladder
(常规化解 — 极端处置 gradient)
                                                   severity
            (1) 条款博弈 (clause game-play: 下修 / call / put) │ gentle
                       │                                       │
                       ▼                                       │
            (2) 提前赎回 (early issuer call to wash out CB)     │
                       │                                       │
                       ▼                                       │
            (3) 引入战投 (introduce strategic investor)          │
                       │                                       │
                       ▼                                       │
            (4) 债务重组 (debt restructuring)                    │
                       │                                       │
                       ▼                                       │
            (5) 破产清算 (bankruptcy liquidation)                │
                       │                                       │
                       ▼                                       ▼
            (6) 退市联动 (delisting linkage post-退市新规)         punitive
```

The post-2024 case-study cohort (蓝盾, 鸿达, 搜特, 全筑, 岭南, etc.)
demonstrates that the upper rungs (clause game-play, early call) succeed
when issuer fundamentals are still serviceable and the market retains
trust; the lower rungs (restructuring, bankruptcy, delisting) become
inevitable when underlying credit collapses faster than the conversion
option can capture residual equity value. **Source:** Lianhe (2025) §1
pp.1-5; Dajia AM (2024) §1 pp.1-3 (NAFMII Vol.142).

## Definition

A **distressed Chinese convertible bond** is a CB whose issuer has
encountered material credit deterioration AND for which the conversion
option has lost value (typically S << K_c) such that the equity-route
exit is no longer credible. The workout space becomes the contractual
+ restructuring + judicial space rather than the conversion market.
The **six workout rungs** in increasing severity span: (1) **条款博弈
(clause game-play)** — the issuer pulls contractual levers (下修 to
drop `K_c` close to depressed share price; 强赎 after brief recovery
to force conversion); (2) **提前赎回 (early call)** — when clause
game-play is insufficient, the issuer calls the CB outright using
residual cash, requiring liquidity above what put-eligible holders
demand; (3) **引入战投 (strategic investor)** — a third party injects
equity or subordinated debt to underwrite conversion or settle put-
back, often combined with a 下修 vote; (4) **债务重组 (debt
restructuring)** — out-of-court or court-supervised restructuring
with principal haircut + extended maturity + occasional equity-for-
debt conversion in unusual ratios (the 全筑转债 2023 case delivered a
non-standard cash + equity + trust-receipt-in-kind mix departing
from prospectus vanilla); (5) **破产清算 (bankruptcy liquidation)** —
the bond enters bankruptcy under the SPC 2020 bond-disputes minutes
where bondholders are ordinary unsecured creditors unless prospectus
or secured-creditor classification governs; (6) **退市联动 (delisting
linkage post-退市新规)** — the 2024 delisting-rule revision tightly
couples equity-side delisting triggers to CB exit paths; when the
underlying equity is delisted, the CB is typically delisted alongside
to OTC / interbank settlement with materially worse liquidity.
**Source:** Lianhe (2025) §1 pp.1-15; Supreme People's Court (2020)
§I-§V pp.1-6.

## Mathematical Reasoning

Let `V(t)` denote the CB market value at time `t`. In the equity-route
regime, `V(t)` tracks `q · S(t)` plus an embedded-option premium. In the
**distressed regime**, the credit floor collapses and the CB price moves
toward `max( H · F, q · S(t))` where `H` is the recovery fraction the
issuer's residual assets can support. **Source:** Lando (2004) Ch.2-3
pp.100-200 (reduced-form credit machinery transferred to CB context);
Calamos (2003) §6 pp.130-170 (distressed-CB practitioner framing).

```
distressed-regime CB value:
    V(t) ≈ max( H · F · D_rf(t, T) ,  q · S(t) )
           └──────credit floor──────┘  └───equity───┘
                       │
                       ▼
   typical post-default: H << 1, S(t) << K_c
   ⇒ both legs collapse; V(t) collapses similarly.
```

The **workout-rung selection** maps to which of the six contractual
or judicial mechanisms is feasible given the residual cash, regulator
acceptance, and bondholder coordination. The mapping is empirical
(Lianhe's case taxonomy) rather than analytical; no closed-form
selection rule exists across the rungs. The **recovery fraction** `H`
for Chinese-CB defaults observed in 2024-2025 has been small in
restructuring outcomes (mixed cash + equity-in-kind + trust-receipt-
in-kind for the 全筑转债 bankruptcy-restructuring case; broadly
comparable across the 蓝盾退债 / 搜特退债 cohort with explicit interest
default events). Quantitative recovery-rate estimation across the
default cohort is the natural domain of the paired sibling card
[`fi-chinese-cb-recovery`](../06_fixed_income_and_credit/fi-chinese-cb-recovery.md)
under subcorpus 06. The **rung-1 → rung-6 ladder** is monotonic in
severity (lower rungs strictly dominate higher rungs in terms of value
preservation), so a rational issuer pulls the lowest-rung lever first.
The empirical observation is that **the gentle-rung tools** (下修 +
提前赎回) succeed only when the underlying credit is **transient-
distressed** rather than **structurally-distressed**; the post-2024
cohort consists chiefly of the latter. **Source:** Lianhe (2025) §1
pp.1-15; Dajia AM (2024) §2 pp.3-9.

## See Also

- [`cb-default-and-recovery.md`](cb-default-and-recovery.md) — general Lando-anchored default + recovery machinery that distressed-CB workouts instantiate in the Chinese market
- [`cb-china-call-redemption-rules.md`](cb-china-call-redemption-rules.md) — strong-call mechanics that drive Rung 1 (clause game-play)
- [`cb-china-downward-conversion.md`](cb-china-downward-conversion.md) — 下修 mechanics that drive Rung 1 (clause game-play)
- [`cb-china-default-cohort-attribution.md`](cb-china-default-cohort-attribution.md) — sibling card (authored Round 5) carrying cross-cohort attribution of which Rung-1-through-Rung-6 paths each post-2014 default issuer took
- [`fi-chinese-cb-recovery.md`](../06_fixed_income_and_credit/fi-chinese-cb-recovery.md) — sibling card (authored Round 10) under subcorpus 06 carrying the Lando reduced-form recovery analysis for the same default cohort

## Escalate to Raw When

Open Lianhe (2025) pp.5-15 directly for the case-by-case workout-path
taxonomy + 2024-2025 default cohort + 退市新规 impact analysis +
forward-looking 2027 maturity-cliff projection. Open Lianhe 2025 H1
default review pp.1-15 + Lianhe CB credit-risk-analysis-and-resolution-
research pp.1-18 for the latest cohort updates and resolution-
mechanism specifics applied during the most recent default events.
**Source:** Lianhe 2025 H1 default review §1 pp.1-15; Lianhe CB
credit-risk-analysis-and-resolution research §1 pp.1-18. Open Supreme People's
Court (2020) pp.1-10 for the judicial-framework backbone of bond-
dispute litigation, bondholder-rights protection, and issuer-
bankruptcy administrator duties; this is the operative legal text
referenced when a Chinese CB enters Rung 5 (bankruptcy liquidation).
Open Dajia AM (2024) NAFMII journal Vol.142 pp.1-9 for the
practitioner-credit-analyst framing of why "CBs are not immune to
credit risk": life-cycle credit-risk taxonomy by stage. Open Lando
(2004) Ch.2-3 pp.100-200 for the reduced-form credit modelling
machinery (hazard rate, default intensity, defaultable term structure)
that the Chinese-CB distressed regime can be evaluated against. Open
Calamos (2003) §6 pp.130-170 for the general practitioner framing of
distressed-CB trading dynamics that transfers to the Chinese-market
case. **Source:** Lianhe (2025) §1 pp.1-15; Supreme People's Court
(2020) §I-§V pp.1-10; Dajia AM (2024) §1-§2 pp.1-9; Lando (2004) Ch.2-3
pp.100-200; Calamos (2003) §6 pp.130-170.
