---
schema_version: "cacg.v0"
id: "fi-credit-risk-fundamentals"
title: "Credit Risk Fundamentals"
reading_id: "06_fixed_income_and_credit"
summary: "Building blocks of credit risk: rating, default probability, recovery, credit spread - without yet entering reduced-form pricing. Decomposes the credit-spread risk premium into expected loss (PD x LGD) plus a risk premium for spread variation."
tags: ["fixed-income", "credit-risk"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2705:4043"
    chunk_hash: "19e46db7f56e3858f915a5e849bbefa9823403a5767430515f12a7d9a596f6e4"
    page_range: [2705, 2706]
    quote: "The ratings upgrade most likely reflects a lower expected probability of default and/or a greater level of recovery of assets if default occurs."
    edge_type: "defines"
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p023:0021"
    chunk_hash: "a2594bc265885b1ea58f27fcb0c47e0ae0a952accf1842d9b0a4d2bb2351a501"
    page_range: [23, 24]
    quote: "We also present a model with stochastically varying spreads for different rating classes, which will become useful later in the chapter on interest-rate swaps."
    edge_type: "supports"
  - source_id: "fi_duffie_singleton_2003_credit_risk"
    chunk_id: "fi_duffie_singleton_2003_credit_risk:p019:0017"
    chunk_hash: "a8280768f39e99293742991c9a37992972f1868a78065528db844d25dd2edd28"
    page_range: [19, 20]
    quote: "In particular, the aggregate credit risk of a diverse portfolio of instruments is often not measured effectively."
    edge_type: "supports"
card_hash: "aad64f8659dbc72caa20195ecc525665b5a24448806e18ac2c579e4ff6695214"
---
# Credit Risk Fundamentals

## Intuition

Credit risk on a bond decomposes into three primitives: the
probability that the issuer defaults over a given horizon
(`p`), the fraction of face value the holder recovers in
default (recovery `R`, with loss `1 - R`), and the spread
(`s`) over the riskless yield that compensates the holder for
expected loss plus a risk premium. Rating agencies map issuer
financials and structure to ordinal grades that summarize
expected default frequency. **Source:** CFA L1 Curriculum
(2022) Vol.5/pp.330-370.

```
<!-- primitive: credit-spread-decomposition source: _diagram_primitives.md -->
yield (%)
   ^
   |
   |  +----------------------+   <- risky-bond YTM
   |  |    risk premium      |
   |  | (compensation for    |
   |  |   spread variation)  |
   |  +----------------------+
   |  |   expected loss      |
   |  | (default probability |
   |  |   x loss given def.) |
   |  +----------------------+   <- Treasury (riskless) YTM
   |  |     time value       |
   |  |  + inflation premium |
   |  +----------------------+
   |
   +--------------------------------> tenor
```

## Definition

Default is the event that the issuer fails to meet a
contractual obligation (coupon or principal) per the
indenture's covenants. Bankruptcy is one terminal default
form; restructuring (covenant breach, distressed exchange)
is another. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.330-345.

The recovery rate `R ∈ [0, 1]` is the fraction of face value
the holder receives at default; loss given default is
`LGD = 1 - R`. Average historical recoveries depend on
seniority and instrument type (senior secured bonds recover
materially more than subordinated bonds). **Source:** CFA L1
Curriculum (2022) Vol.5/pp.345-360.

The credit spread `s = y_risky - y_riskless` is the yield
premium of a risky bond over a riskless reference of the
same tenor and similar features. It can be measured as a
G-spread (over Treasuries), I-spread (over swaps), or option-
adjusted spread (after call options are stripped out).
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.350-370;
Lando (2004) §2 pp.5-30.

A credit rating is an ordinal label assigned by an agency
(Moody's, S&P, Fitch) summarizing the issuer's expected
default frequency over a multi-year horizon. Investment-grade
labels (e.g. AAA through BBB-) and high-yield labels
(BB+ and below) anchor regulatory and indexing frameworks.
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.360-370.

## Mathematical Reasoning

For a single-period zero-coupon bond with risk-neutral default
probability `p`, recovery `R`, and face value `F`, the expected
payoff is `(1 - p) · F + p · R · F`. Comparing the risky
discount rate with the Treasury discount rate yields the
first-order short-horizon spread approximation
`s ≈ p · (1 - R)`. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.350-370; Lando (2004) §2
pp.5-30.

The first-order approximation usually sits below observed
corporate-bond spreads because risk, liquidity, tax, and
recovery-uncertainty premia are omitted. The decomposition
`s = expected loss + premium components` is qualitative at the
level of this card; the
reduced-form intensity machinery that quantifies it lives in
[`fi-credit-spread-machinery.md`](./fi-credit-spread-machinery.md#definition).
**Source:** Lando (2004) §2 pp.5-30.

Recovery is correlated with the bond's seniority and
instrument type, and historically with the macroeconomic
state (recoveries fall in distressed cycles). Average levels
range roughly from `R ≈ 0.6-0.8` for senior secured bonds
down to `R ≈ 0.2-0.3` for unsecured subordinated bonds; the
asymmetry survives at long horizons even after accounting
for issuer-specific features. **Source:** CFA L1 Curriculum
(2022) Vol.5/pp.345-360.

The contractual stream from
[`fi-bond-anatomy-and-cashflows.md`](./fi-bond-anatomy-and-cashflows.md#mathematical-reasoning)
is the no-default baseline; the credit-risk machinery adds
the default-recovery branching that produces the credit
spread. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.330-345.

## See Also

- [`fi-bond-anatomy-and-cashflows.md`](fi-bond-anatomy-and-cashflows.md) — the riskless contractual stream this card adds default to
- [`fi-credit-spread-machinery.md`](fi-credit-spread-machinery.md) — reduced-form spread machinery as the next layer
- [`fi-default-models-and-recovery.md`](fi-default-models-and-recovery.md) — calibration of `p` and `R` in structural and reduced-form pricing applications

## Escalate to Raw When

Open CFA L1 Curriculum Vol.5 Reading 46 or Lando's chapters
2-3 directly when any of the criteria below applies.
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.330-370.

- The card needs reduced-form intensity calibration (hazard
  rate vs default rate, term-structure of default
  probabilities); that lives in
  [`fi-credit-spread-machinery.md`](./fi-credit-spread-machinery.md#mathematical-reasoning).
  **Source:** Lando (2004) §2 pp.5-30.
- A structural (Merton-style) decomposition of default
  probability into asset value and leverage is in scope;
  this card stays at the reduced-form / rating-agency level.
  **Source:** Lando (2004) §2 pp.5-30.
- CDS pricing, basis-trading, or counterparty risk
  mechanics are needed; those follow a separate
  decomposition layer (per DEC-12 placement of the
  Crepey-anchored CDS / XVA cards in 06).
  **Source:** Lando (2004) §2 pp.5-30.
