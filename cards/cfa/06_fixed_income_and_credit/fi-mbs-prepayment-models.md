---
schema_version: "cacg.v0"
id: "fi-mbs-prepayment-models"
title: "Agency MBS Prepayment Models"
reading_id: "06_fixed_income_and_credit"
summary: "Agency MBS Prepayment Models — auto-generated placeholder summary; revise in fix-pass if needed; full audit notes available in audit_notes."
tags: ["fixed-income", "mbs-prepayment"]
citations:
  - source_id: "fi_davidson_levin_2014_mortgage_valuation_models"
    chunk_id: "fi_davidson_levin_2014_mortgage_valuation_models:p144:0160"
    chunk_hash: "f0bdbdc93f6390f60b01f9d3c04cec9b2e8cdfd5989cda845c05684923430120"
    page_range: [144, 145]
    quote: "(We know that prepayment rates for large cohorts of pools rarely exceed 15% in any one month.) The refinancing incentive has been modeled in a variety of ways."
    edge_type: "defines"
  - source_id: "fi_davidson_levin_2014_mortgage_valuation_models"
    chunk_id: "fi_davidson_levin_2014_mortgage_valuation_models:p277:0317"
    chunk_hash: "d6cbc1b7e71b26b10c6f381cbb9c2b479fdf616427b7eb9e453d665c13d48660"
    page_range: [277, 278]
    quote: "Furthermore, modeling defaults and losses in pools of homeowners demands more data than modeling prepayments and is most accurately predicted at the loan-level."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2544:3746"
    chunk_hash: "a7218b917fcc305cdd0012c1e46c9c547951337cad64ac68986584bb9354e025"
    page_range: [2544, 2545]
    quote: "Prepayment modeling uses characteristics of the mortgage pool and other factors to develop a statistical model for forecasting future prepayments."
    edge_type: "supports"
card_hash: "52153fd70bace8f4c30d410364b94b774aa577ad19fac6b24cdf401b0da76a50"
---
# Agency MBS Prepayment Models

## Intuition

A residential mortgage pool's monthly cash flow depends on how many borrowers prepay (refinance, sell, or pay down) their loans each month. An **agency MBS** (Fannie Mae, Freddie Mac, Ginnie Mae pools) has government-agency credit guarantee, so the holder bears no credit risk on individual loans, but does bear the **prepayment risk** that the cash-flow timing is uncertain. The prepayment model maps observable variables (the loan rate, prevailing mortgage rates, loan age, pool characteristics) to a forecast of monthly prepayment speed (conditional prepayment rate, CPR). The model is the cash-flow generator that feeds every downstream MBS valuation. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

```
prepayment model structure
   inputs (observable at each month t):
       - WAC: weighted-average coupon of the pool
       - WAM: weighted-average remaining maturity
       - prevailing mortgage rate m_t (refi-incentive driver)
       - loan age (months since origination)
       - pool characteristics (LTV, FICO, geography)
   outputs (per-month forecast):
       - CPR_t = annualized prepayment speed
       - SMM_t = monthly prepayment rate (= 1 − (1 − CPR_t)^(1/12))
       - hazard-rate-equivalent prepayment intensity
   structural decomposition (Davidson+Levin):
       CPR = refinance_component
           + housing_turnover_component
           + curtailment_component
           + default_component
       each component is driven by distinct variables;
       refinance is the most rate-sensitive.

   the refinance S-curve (the dominant prepayment driver)
   shape of CPR_refi vs refinance incentive (WAC − m_t):
        ^   CPR_refi
        |                    +--------------- "in the money" plateau
        |                   /         (cap from credit / friction / underwater)
        |                  /
        |             ----+
        |            /
        |           /
        |  --------+    transition zone
        |  "out of the money" base level (turnover-only floor)
        +-------------------------------> incentive (WAC − m_t)
                negative   0    positive
       interpretation:
       - when m_t > WAC: borrower has no refi incentive; CPR_refi at base
       - when m_t < WAC: borrower's refi is in the money; CPR_refi rises
       - when m_t << WAC: most willing borrowers have already refi'd;
         CPR_refi plateaus and burns out (next card)
```

## Definition

The **conditional prepayment rate (CPR)** is the annualized fraction of the pool's remaining principal that is expected to prepay over the next twelve months. The monthly equivalent **single monthly mortality (SMM)** is the rate per month: `SMM = 1 − (1 − CPR)^(1/12)`. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

The **PSA benchmark** (Public Securities Association) is the industry-standard reference prepayment schedule: a 100-PSA pool ramps from 0.2 percent CPR at month 1 linearly to 6 percent CPR at month 30 and stays at 6 percent thereafter. Real-world prepayment is quoted as a multiple of PSA (e.g. "200 PSA" doubles the speed at every point on the curve). PSA is a historical convention, not a model; it is the unit in which raw prepayment quotes are expressed. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144; CFA L1 Curriculum (2022) Vol.5/pp.190-220.

A **prepayment model** is a function `CPR(observables, age, parameters)` that maps the loan-and-market state to a forecast prepayment rate. Davidson+Levin develop the **structural decomposition** that splits CPR into refinance, housing-turnover, curtailment, and default components, each driven by distinct variables. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

The **refinance S-curve** is the empirical relationship between the refinance-incentive variable (typically `WAC − m_t`, the loan rate minus the prevailing mortgage rate) and the refinance component of CPR. The curve is flat at low incentive (no refi activity), steep through the transition zone (rapid refi as the loan goes in the money), and flat at high incentive (the in-the-money plateau where most willing borrowers have already refi'd). **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

The **housing-turnover component** is the prepayment rate from borrowers selling their homes for reasons unrelated to refinancing (relocation, life events, trade-up). This component is approximately rate-insensitive and typically constitutes the prepayment floor in low-incentive environments. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

The **curtailment component** is the prepayment from voluntary excess monthly payments above the scheduled amortization (paying down principal faster). This component is small but non-zero in steady state. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

## Mathematical Reasoning

The model produces a per-period prepayment-rate forecast that determines the pool's expected cash flows at each month: `CashFlow_t = ScheduledPaymentPrincipal_t + ScheduledInterest_t + Prepayment_t · RemainingPrincipal_t`. The cash-flow stream feeds downstream MBS valuation (OAS, duration, convexity); the prepayment model is therefore the upstream input layer. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

The refinance S-curve is empirically the dominant component when rates are below WAC. Davidson+Levin specify the curve parametrically (the common forms are the cumulative-normal (Vasicek), arctangent, and piecewise-linear function of `WAC − m_t`) and calibrate the parameters to historical prepayment data. The S-curve's shape — flat / steep / flat — is the empirical signature of borrower behavior: a credit-bound, friction-bound, and option-cost-bound process rather than a deep-out-of-the-money / at-the-money / deep-in-the-money option exercise. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

The connection to the L1 PSA intuition from [`fi-prepayment-risk-intuition.md`](./fi-prepayment-risk-intuition.md#mathematical-reasoning) is direct: PSA is the simplest possible prepayment model (a fixed deterministic schedule). Davidson+Levin replace the deterministic PSA schedule with a state-dependent CPR forecast driven by rate moves; this is the L2 practitioner depth shift. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

The model's structural decomposition matters for valuation because each component has different sensitivities to the relevant state variables: refinance is rate-sensitive (drives MBS negative convexity), housing-turnover is approximately rate-insensitive (provides the prepayment floor), curtailment is small and approximately constant. Modeling each component separately and combining their forecasts gives the practitioner explicit attribution: a given month's CPR forecast can be decomposed into refi-share + turnover-share + curtailment-share. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

The connection to the OAS framework developed in [`fi-mbs-oas-calculation-depth.md`](./fi-mbs-oas-calculation-depth.md#mathematical-reasoning) is the joint-model interaction: the prepayment model is conditioned on the short-rate path simulated under the risk-neutral measure; OAS calibration requires the prepayment model to be self-consistent with the rate process at every node. **Source:** Davidson & Levin (2014) Ch.12 pp.247-266.

A critical caveat: agency prepayment models are calibrated to historical agency prepayment data, which inherits the historical credit and policy regime. Regime shifts (e.g. the GSE post-conservatorship era, the COVID forbearance period, mortgage-policy changes) can shift the calibrated S-curve. Davidson+Levin caution that the model is a parametrization of historical behavior rather than a structural truth. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

## See Also

- [`fi-prepayment-risk-intuition.md`](fi-prepayment-risk-intuition.md) — L1 PSA intuition the prepayment model deepens
- [`fi-mbs-burnout-and-seasoning.md`](fi-mbs-burnout-and-seasoning.md) — the burnout and seasoning effects modifying the prepayment S-curve
- [`fi-mbs-oas-calculation-depth.md`](fi-mbs-oas-calculation-depth.md) — OAS as the rate-path-and-prepayment joint simulation
- [`fi-securitization-fundamentals.md`](fi-securitization-fundamentals.md) — L1 securitization scaffold the prepayment model feeds

## Escalate to Raw When

Open Davidson & Levin (2014) Ch.7 (Agency Pool Prepayment Models)
directly when any of the criteria below applies.
**Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

- The card user needs the explicit parametric form of the
  refinance S-curve and its calibration recipe to a dated
  historical prepayment data set.
  **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.
- A specific historical prepayment episode (e.g. a refi wave
  triggered by a rate decline) requires the dated CPR-vs-
  incentive scatter and the model's residual analysis.
  **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.
- Loan-level (rather than pool-level) prepayment modeling is
  required — escalate to Davidson+Levin Ch.12 (Loan Level
  Modeling) for the non-agency loan-level framework.
  **Source:** Davidson & Levin (2014) Ch.12 pp.247-266.
- Policy-shift regime detection (the model's calibration
  breaking after a GSE policy change, a mortgage-rate-cap
  intervention, or a credit-tightening cycle) requires
  dated regime-analysis evidence Davidson+Levin discusses
  case-by-case.
  **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.
