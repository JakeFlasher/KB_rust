---
schema_version: "cacg.v0"
id: "fi-mbs-burnout-and-seasoning"
title: "MBS Burnout and Seasoning"
reading_id: "06_fixed_income_and_credit"
summary: "MBS Burnout and Seasoning — auto-generated placeholder summary; revise in fix-pass if needed; full audit notes available in audit_notes."
tags: ["fixed-income", "mbs-burnout"]
citations:
  - source_id: "fi_davidson_levin_2014_mortgage_valuation_models"
    chunk_id: "fi_davidson_levin_2014_mortgage_valuation_models:p144:0160"
    chunk_hash: "f0bdbdc93f6390f60b01f9d3c04cec9b2e8cdfd5989cda845c05684923430120"
    page_range: [144, 145]
    quote: "(We know that prepayment rates for large cohorts of pools rarely exceed 15% in any one month.) The refinancing incentive has been modeled in a variety of ways."
    edge_type: "defines"
  - source_id: "fi_davidson_levin_2014_mortgage_valuation_models"
    chunk_id: "fi_davidson_levin_2014_mortgage_valuation_models:p187:0213"
    chunk_hash: "a77987a7432309198f5cf8849396d4426ae3ce48b1f823fdc69e38e06c6e47f2"
    page_range: [187, 188]
    quote: "MONTE CARLO BASICS FOR THE OPTION-ADJUSTED SPREAD FRAMEWORK Simulating Random Factors Let us consider how one might run simulations of interest rates."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2544:3746"
    chunk_hash: "a7218b917fcc305cdd0012c1e46c9c547951337cad64ac68986584bb9354e025"
    page_range: [2544, 2545]
    quote: "Prepayment modeling uses characteristics of the mortgage pool and other factors to develop a statistical model for forecasting future prepayments."
    edge_type: "supports"
card_hash: "a5313c2934d86818eae310b42cf84dacd81d77fe75aef23a4af969f160e72a92"
---
# MBS Burnout and Seasoning

## Intuition

A naive prepayment model that depends only on the current refinance incentive `WAC − m_t` overpredicts prepayments in two distinct ways: first, a pool that has already experienced prior episodes of high in-the-money refi behavior contains residual borrowers who did not refinance despite the opportunity — they are **burned-out** and have lower refi-sensitivity than the fresh pool. Second, a freshly originated loan has very low prepayment regardless of rate environment because borrowers take some months to organize a refinance — the loan is not yet **seasoned**. Burnout and seasoning are the two path-dependent modulators on top of the structural S-curve developed in [`fi-mbs-prepayment-models.md`](./fi-mbs-prepayment-models.md#mathematical-reasoning). **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

```
seasoning ramp and burnout decay
   seasoning: prepayment speed rises from origination
   to a steady-state level over the first ~30 months
       CPR
        ^
        |                  steady-state CPR (after seasoning ramp)
        |    ____________________________________________
        |   /
        |  /
        | /
        |/
        +------+------+-----------------> loan age (months)
                 ramp     post-seasoning
       interpretation: a freshly originated loan needs time to
       develop the borrower's awareness of refi opportunities;
       PSA hardcodes a 30-month ramp as the industry default.

   burnout: a pool with prior in-the-money exposure has lower
   refi response than a fresh pool at the same current incentive
       CPR_refi
        ^
        |                       fresh pool S-curve
        |                        ___________________
        |                       /
        |                      /
        |   burned-out pool   /
        |   S-curve           /
        |    ________________/
        |   /                /
        |  /                /
        | / ___________ ___/
        +-------+-----+----+------------> refinance incentive (WAC − m_t)
                       transition zone
       interpretation: a pool that has been "burned" by prior
       in-the-money episodes contains residual non-refi-willing
       borrowers (high friction, low credit, behavioral inertia);
       the burned-out pool's S-curve sits below the fresh pool's.
```

## Definition

**Seasoning** is the empirical regularity that prepayment speed rises from a near-zero base at month-zero to a steady-state level over the first ~30 months of the loan's life. The seasoning ramp reflects the borrower's gradual transition from origination friction (closing costs, paperwork, awareness gaps) to mature refi-receptivity. The PSA benchmark encodes this as the linear ramp from 0.2 percent CPR at month-1 to 6 percent CPR at month-30. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

**Burnout** is the empirical regularity that a pool whose loans have been previously in the money for refi (i.e. `m_t < WAC` for an extended past episode) has lower refi response to a current rate decline than a fresh pool at the same incentive. Burnout is a path-dependent quantity that distinguishes between pools with the same current observable state but different histories. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

The **burnout factor** is the multiplicative scaling on the fresh-pool refi component that accounts for the cumulative prior in-the-money exposure. Davidson+Levin parametrize the burnout factor as a decaying function of the cumulative exposure (or alternatively as a state variable that decays toward a fully-burned-out asymptote). **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

The **path dependence** in the prepayment model is the consequence of burnout: the future cash-flow forecast depends on the path of historical rates, not just the current rate. This makes MBS valuation under Monte Carlo necessarily path-by-path rather than expectation-based on terminal state. **Source:** Davidson & Levin (2014) Ch.9 pp.174-200.

## Mathematical Reasoning

The seasoning ramp is empirically calibrated to historical data: the per-month CPR is multiplied by a seasoning factor that rises from near-zero at month-0 to one by ~30 months. A common parametric form is a piecewise-linear ramp matching the PSA convention, or alternatively a smooth logistic ramp with a calibrated mid-point and slope. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

The burnout factor depends on the cumulative in-the-money exposure: as the pool accumulates more months where `m_t < WAC`, the burnout factor decays toward a floor (the fully-burned-out asymptote at which only the housing-turnover floor remains). The decay rate is calibrated to historical pool behavior — Davidson+Levin report empirical decay constants reflecting how quickly the willing-to-refi borrowers exhaust themselves. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

The combined CPR forecast multiplies the structural S-curve from [`fi-mbs-prepayment-models.md`](./fi-mbs-prepayment-models.md#mathematical-reasoning) by both modulators: `CPR_t = SeasoningFactor(age_t) · BurnoutFactor(history_t) · CPR_structural(incentive_t) + nonrefi_components`. The first three months of a fresh loan see very low CPR (seasoning damps); a pool with prior refi waves sees reduced CPR even at the same current incentive (burnout damps). **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

The path dependence introduced by burnout is the key reason MBS valuation in [`fi-mbs-oas-calculation-depth.md`](./fi-mbs-oas-calculation-depth.md#mathematical-reasoning) must use Monte Carlo simulation under the risk-neutral measure rather than backward-induction on a recombining tree: burnout depends on the full historical path, not just the current state, so a recombining lattice cannot capture it. Monte Carlo simulates many distinct rate paths and accumulates the path-specific burnout state along each. **Source:** Davidson & Levin (2014) Ch.9 pp.174-200.

The empirical S-curve documented for a fresh-pool baseline shifts and steepens during the seasoning ramp; after seasoning completes, subsequent burnout shifts the S-curve down and flattens it. The CFA L1 framing in [`fi-prepayment-risk-intuition.md`](./fi-prepayment-risk-intuition.md#mathematical-reasoning) is built around the post-seasoning, no-burnout PSA baseline; the practitioner depth Davidson+Levin adds is the recognition that this baseline does not hold for fresh-originate or burnt-out pools — a fact that materially changes the valuation and the negative-convexity profile of the MBS. **Source:** CFA L1 Curriculum (2022) Vol.5/pp.190-220.

A critical caveat: the burnout-and-seasoning calibration is most stable for agency pools with long histories; for newly originated programs (e.g. a new pool type after a policy change) the model relies on cross-pool extrapolation, and the practitioner must apply judgment regarding the extrapolation's validity. **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

## See Also

- [`fi-mbs-prepayment-models.md`](fi-mbs-prepayment-models.md) — structural prepayment-model decomposition that the burnout and seasoning modulators sit on top of
- [`fi-mbs-oas-calculation-depth.md`](fi-mbs-oas-calculation-depth.md) — OAS Monte Carlo simulation that propagates the path-dependent burnout state along each rate path
- [`fi-prepayment-risk-intuition.md`](fi-prepayment-risk-intuition.md) — L1 PSA framing that pre-supposes a fully-seasoned no-burnout baseline

## Escalate to Raw When

Open Davidson & Levin (2014) Ch.7 (Agency Pool Prepayment
Models) directly when any of the criteria below applies.
**Source:** Davidson & Levin (2014) Ch.7 pp.121-144.

- The card user needs the parametric form of the seasoning
  ramp and burnout-factor decay for a specific calibrated
  agency-pool model.
  **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.
- A specific historical episode (e.g. a refi wave that
  burned out a pool over a sustained low-rate period)
  requires the dated CPR-vs-time series and the residual
  attribution Davidson+Levin analyzes.
  **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.
- The card user needs the Monte Carlo simulation infrastructure
  for propagating burnout state along risk-neutral rate paths.
  **Source:** Davidson & Levin (2014) Ch.9 pp.174-200.
- Newly originated pool-type extrapolation (calibrating
  burnout-and-seasoning for a pool with no historical
  precedent) is required at desk-level judgment — Davidson+Levin
  provides framework; the dated extrapolation belongs to
  practitioner workflows.
  **Source:** Davidson & Levin (2014) Ch.7 pp.121-144.
