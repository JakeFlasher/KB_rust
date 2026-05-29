---
schema_version: "cacg.v0"
id: "fi-securitization-fundamentals"
title: "Securitization Fundamentals"
reading_id: "06_fixed_income_and_credit"
summary: "Securitisation pools cashflow-generating assets (mortgages, auto loans, credit-card receivables) into a special-purpose entity that issues tranched claims. Senior tranches absorb losses last and earn a low coupon; equity tranches absorb losses first. RMBS specialise in residential mortgages, CMBS in commercial real estate, ABS in non-mortgage receivables."
tags: ["fixed-income", "securitization-fundamentals"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2527:3717"
    chunk_hash: "8c4443133027b4c2ffe2993419fbdc7d9dab7dba46825a72bb5c0bb99efd5d4b"
    page_range: [2527, 2528]
    quote: "Securitization allows for the creation of tradable securities with better liquidity than that of the original loans on the bank"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2529:3720"
    chunk_hash: "62d18565447f783c984d80b486a7904b747f7bab551879ba5765152b1c034e41"
    page_range: [2529, 2529]
    quote: "Such a legal entity is referred to as a special purpose entity (SPE) and sometimes also called a special purpose vehicle (SPV) or a special purpose company"
    edge_type: "supports"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p202:0305"
    chunk_hash: "adf0f1300780a0ebd49fd939d0f911629838ab842ac66026514d60a304dd077e"
    page_range: [202, 203]
    quote: "A portfolio of income-producing assets such as loans is sold by the originating banks to a special purpose vehicle (SPV) and the cash flows from the assets are then allocated to tranches"
    edge_type: "supports"
card_hash: "4e60fdde4a8972d0f2c027dfa4e4155362e136e7d77f07a6859b53a155f7f041"
---
# Securitization Fundamentals

## Intuition

Securitization pools cashflow-generating assets
(mortgages, auto loans, credit-card receivables, lease
payments) into a special-purpose entity (SPE) and
issues tranched claims against the pool. Senior tranches
take losses last and earn a low coupon; junior /
equity tranches take losses first and earn a higher
coupon. The economic effect is that diversified
underlying pools support investment-grade-rated senior
debt even when individual underlying loans are
sub-investment-grade. **Source:** CFA L1 Curriculum
(2022) Vol.5/pp.181-220.

```
underlying pool                       SPE issues
   |                                  ----------------
   | mortgages, auto loans,            | senior   |
   | credit-card balances,             | tranche  |  AAA, low coupon
   | lease payments                    ----------------
   |                                   | mezzanine|
   |--> SPE collects cashflows -->     | tranche  |  BBB, mid coupon
   |    waterfalls payments by         ----------------
   |    seniority                      | equity   |
   |                                   | tranche  |  unrated, residual
   |                                   ----------------
```

## Definition

A residential mortgage-backed security (RMBS) is a
securitization whose underlying pool is residential
mortgages. The pool's monthly cashflows (scheduled
principal + interest + prepayments) are passed through
or restructured into tranches. Pass-through structures
distribute pool cashflows pro-rata; CMOs (collateralized
mortgage obligations) restructure into PAC / TAC /
support tranches with different prepayment-risk
profiles. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.181-220.

A commercial mortgage-backed security (CMBS) pools
commercial real-estate loans. Compared to RMBS,
prepayment risk is reduced (commercial loans typically
have prepayment penalties or yield-maintenance clauses)
and credit risk dominates. **Source:** CFA L1 Curriculum
(2022) Vol.5/pp.200-220.

An asset-backed security (ABS) generalizes the
securitization to non-mortgage pools: auto loans, credit-
card receivables, student loans, lease payments. The
tranching mechanics mirror RMBS but the credit /
prepayment risk profile depends on the underlying asset
class. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.181-220.

## Mathematical Reasoning

The pool's loss distribution drives tranche-rating
mathematics. With diversified underlyings, pool default
correlation determines the loss-distribution tail: low
correlation produces a thin tail and supports
investment-grade senior tranches; high correlation
("everyone defaults together") fattens the tail and
forces ratings down. The 2007-2008 mortgage crisis
demonstrated the rating-agency under-estimation of
correlation in stressed scenarios. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.181-220.

Tranching is loss-allocation rather than asset
allocation: the senior tranche has first claim on
cashflows but bears losses last (the equity tranche
absorbs first). For a pool of expected loss `EL` and
tranche thresholds `(0, A_e, A_m, F)` (equity / mezzanine
/ senior boundaries), the senior loss is positive only
when realized loss exceeds `A_m`. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.181-220.

The vanilla bond cashflow stream of
[`fi-bond-anatomy-and-cashflows.md`](./fi-bond-anatomy-and-cashflows.md#mathematical-reasoning)
generalizes to securitization tranches by replacing the
single coupon-bond stream with the tranche-specific
waterfall. Prepayment risk introduces an additional
cashflow uncertainty that
[`fi-prepayment-risk-intuition.md`](./fi-prepayment-risk-intuition.md#definition)
addresses. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.181-220.

## See Also

- [`fi-bond-anatomy-and-cashflows.md`](fi-bond-anatomy-and-cashflows.md) — vanilla cashflow stream as the pre-securitization baseline
- [`fi-prepayment-risk-intuition.md`](fi-prepayment-risk-intuition.md) — prepayment-driven cashflow uncertainty in mortgage securitization

## Escalate to Raw When

Open CFA L1 Curriculum Vol.5 Reading 44 directly when
any of the criteria below applies. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.181-220.

- A specific PAC / TAC / support tranche cashflow
  decomposition is needed and the prepayment scenarios
  must be modeled. **Source:** CFA L1 Curriculum (2022)
  Vol.5/pp.181-220.
- Synthetic CDO mechanics or single-tranche CDOs are
  in scope; this card covers cash securitizations only.
  **Source:** Hull §8 pp.183-200.
- A regulatory-capital treatment (Basel III risk-weights,
  retention rules) is required; legal / regulatory
  framing falls outside this card's scope. **Source:**
  CFA L1 Curriculum (2022) Vol.5/pp.181-220.
