---
schema_version: "cacg.v0"
id: "fi-mbs-oas-calculation-depth"
title: "MBS OAS Calculation Depth (Monte Carlo)"
reading_id: "06_fixed_income_and_credit"
summary: "MBS OAS Calculation Depth (Monte Carlo) — auto-generated placeholder summary; revise in fix-pass if needed; full audit notes available in audit_notes."
tags: ["fixed-income", "mbs-oas"]
citations:
  - source_id: "fi_davidson_levin_2014_mortgage_valuation_models"
    chunk_id: "fi_davidson_levin_2014_mortgage_valuation_models:p187:0213"
    chunk_hash: "a77987a7432309198f5cf8849396d4426ae3ce48b1f823fdc69e38e06c6e47f2"
    page_range: [187, 188]
    quote: "MONTE CARLO BASICS FOR THE OPTION-ADJUSTED SPREAD FRAMEWORK Simulating Random Factors Let us consider how one might run simulations of interest rates."
    edge_type: "defines"
  - source_id: "fi_davidson_levin_2014_mortgage_valuation_models"
    chunk_id: "fi_davidson_levin_2014_mortgage_valuation_models:p106:0120"
    chunk_hash: "e190e73aac1c2899717d18d15516f01f4ad9a3379e1dad2c44281f4b8e2b4fb6"
    page_range: [106, 107]
    quote: "Adding a Second Factor to Short-Rate Models Let us consider a fixed-income instrument that pays floating coupon indexed to some short rate (such as 3-month LIBOR)."
    edge_type: "supports"
  - source_id: "fi_davidson_levin_2014_mortgage_valuation_models"
    chunk_id: "fi_davidson_levin_2014_mortgage_valuation_models:p138:0154"
    chunk_hash: "d8050a6e9a846027254b007e334e50377b3b3c91b4c51d3de420e64af5cb552c"
    page_range: [138, 139]
    quote: "Discount mortgages generally have prepayment speeds under 10% CPR, while premium mortgages can exhibit prepayments speeds well above 50% CPR."
    edge_type: "supports"
card_hash: "ed51157df8bfc570fcbcb6c7c0b1061573e276b8eff7152041f55b48bef71ec5"
---
# MBS OAS Calculation Depth (Monte Carlo)

## Intuition

A mortgage-backed security's cash flows are path-dependent: the prepayment behavior at each future month depends on the rate path that led there (burnout) and on the rate level at that month (refi incentive). Pricing the MBS therefore requires simulating many distinct rate paths under the risk-neutral measure, generating the prepayment-driven cash flow on each path, discounting along the path's stochastic short-rate process, and averaging across paths. The **option-adjusted spread (OAS)** is the constant additive spread on the short-rate discount curve that equates the simulated-average MBS price to the observed market price. **Source:** Davidson & Levin (2014) Ch.9 pp.174-200.

```
OAS Monte Carlo workflow
   inputs:
       - calibrated short-rate model (one-factor or HJM-style)
         producing risk-neutral rate paths r_t^(p) for p = 1, ..., N_paths
       - prepayment model from the prior cluster card
         producing path-specific CPR_t^(p) at each t and each p
       - candidate OAS spread s (to be solved)
       - market price P_market

   procedure (sketch):
       for each path p in 1..N_paths:
           initial remaining principal RP_0 := par
           initial burnout state B_0 := baseline
           for each month t in 1..T:
               incentive on path: i_t^(p) := WAC − m_t^(p)
               burnout state update: B_t^(p) := f(B_{t-1}^(p), i_t^(p))
               prepayment rate from model: CPR_t^(p) given B_t^(p)
               monthly cash flow: CF_t^(p) := scheduled + prepayment
               remaining principal update: RP_t^(p)
               path-specific discount factor:
                   D_t^(p) := exp(− Σ_{u≤t} (r_u^(p) + s) · Δt )
           path-present-value: PV_p := Σ_t D_t^(p) · CF_t^(p)
       model-implied price: P_model(s) := (1 / N_paths) · Σ_p PV_p

   OAS calibration: find s such that P_model(s) = P_market.
   solution: numerical root-finding (typically bisection or
   Newton on the price-spread relationship; convergence is
   monotonic because P_model is decreasing in s).

   interpretation:
       OAS = path-averaged net premium for credit, liquidity,
             and model risk above the calibrated rate curve,
             after stripping the embedded prepayment option's
             value via the simulation.
```

## Definition

The **Monte Carlo simulation** under the risk-neutral measure generates `N_paths` distinct rate paths consistent with the calibrated short-rate model. Each path is a sequence of monthly short rates from origination to the MBS's terminal date. **Source:** Davidson & Levin (2014) Ch.9 pp.174-200.

The **path-by-path cash-flow generation** runs the prepayment model along each rate path, propagating the burnout state from the prior month's cumulative in-the-money exposure. Each path produces a distinct cash-flow sequence and a distinct discount-factor sequence. **Source:** Davidson & Levin (2014) Ch.9 pp.174-200.

The **option-adjusted spread (OAS)** is the constant additive spread `s` such that the average across paths of the discounted-cash-flow-sum equals the observed market price. The procedure is a 1-D root-finding problem on a monotone decreasing function (raising `s` lowers the model-implied price). **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

The **effective duration** for an MBS is the negative-of-price-derivative with respect to a parallel rate shift, computed by re-running the OAS Monte Carlo with the entire short-rate curve shifted up and down by a small amount and finite-differencing. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

The **convexity** of an MBS — typically negative because of the embedded prepayment option — is the second-derivative of price with respect to rates, computed analogously via the OAS Monte Carlo at up, base, and down rate shifts. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

## Mathematical Reasoning

The OAS Monte Carlo unifies the rate model from [`fi-arbitrage-free-valuation-l2.md`](./fi-arbitrage-free-valuation-l2.md#mathematical-reasoning) with the prepayment model from [`fi-mbs-prepayment-models.md`](./fi-mbs-prepayment-models.md#mathematical-reasoning) and the burnout-seasoning modulators from [`fi-mbs-burnout-and-seasoning.md`](./fi-mbs-burnout-and-seasoning.md#mathematical-reasoning). The simulation is the practitioner's workhorse for any path-dependent MBS valuation. **Source:** Davidson & Levin (2014) Ch.9 pp.174-200.

The path-averaged model price `P_model(s)` is monotone-decreasing in `s` because raising the spread increases the discount factor's magnitude at every path-time pair, reducing the present-value contribution of every cash flow. Root-finding for OAS therefore converges deterministically; Davidson+Levin recommend the bisection method for robustness or Newton's method for speed when the price-spread relationship's slope is well-conditioned. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

The number of Monte Carlo paths required to achieve a target OAS precision depends on the variance of `PV_p` across paths. Davidson+Levin discuss variance-reduction techniques (antithetic variates, control variates, importance sampling) borrowed from Glasserman's framework. A typical MBS valuation uses several hundred to several thousand paths for production OAS quotes. **Source:** Davidson & Levin (2014) Ch.9 pp.174-200.

The OAS's interpretation as the "constant additive premium" relies on the assumption that the short-rate model and prepayment model are correctly specified. When they are mis-specified, OAS absorbs the misspecification as well as the credit-and-liquidity premium. Davidson+Levin caution that OAS is therefore a model-dependent quantity; comparing OAS across MBS valued by different models or different prepayment specifications is ill-defined. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

The connection to the L1 OAS intuition from [`fi-oas-and-effective-duration.md`](./fi-oas-and-effective-duration.md#mathematical-reasoning) is the depth shift: the L1 framework asserts OAS as the model-stripped credit-and-liquidity spread; the Davidson+Levin practitioner depth specifies what "model-stripped" actually means computationally — the OAS Monte Carlo simulation under the risk-neutral measure with the calibrated prepayment model. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

The negative convexity of agency MBS arises from the path-dependent prepayment response: when rates fall, refi-driven prepayments accelerate and the MBS holder receives par back earlier than expected — the upside is capped. When rates rise, refi-driven prepayments slow and the MBS holder is stuck holding a low-coupon bond longer than expected — the downside is amplified. The OAS Monte Carlo captures this asymmetry by simulating both up and down rate scenarios and observing the price's nonlinear response. **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.

## See Also

- [`fi-oas-and-effective-duration.md`](fi-oas-and-effective-duration.md) — L1 OAS intuition the Monte Carlo machinery quantifies
- [`fi-mbs-prepayment-models.md`](fi-mbs-prepayment-models.md) — structural prepayment-model decomposition feeding the simulation
- [`fi-mbs-burnout-and-seasoning.md`](fi-mbs-burnout-and-seasoning.md) — path-dependent modulators making Monte Carlo necessary (vs backward-induction trees)
- [`fi-mbs-key-rate-and-effective-duration.md`](fi-mbs-key-rate-and-effective-duration.md) — finite-difference duration measures computed via the OAS Monte Carlo at rate shifts
- [`fi-arbitrage-free-valuation-l2.md`](fi-arbitrage-free-valuation-l2.md) — risk-neutral measure foundation for the path simulation

## Escalate to Raw When

Open Davidson & Levin (2014) Ch.9 (Monte Carlo Methods) and Ch.10
(Applications of OAS Valuation) directly when any of the
criteria below applies.
**Source:** Davidson & Levin (2014) Ch.9-10 pp.174-220.

- The card user needs the variance-reduction technique
  selection (antithetic variates, control variates, importance
  sampling, Latin-hypercube stratification) for a specific
  MBS valuation task.
  **Source:** Davidson & Levin (2014) Ch.9 pp.174-200.
- A specific calibrated short-rate model and its risk-neutral
  path-generation algorithm is required at numerical precision
  — escalate to Davidson+Levin Ch.5 (Short-Rate Term-Structure
  Modeling).
  **Source:** Davidson & Levin (2014) Ch.5 pp.76-105.
- Credit-OAS (prOAS, the prepayment-risk-neutral variant)
  for non-agency MBS is required — out of this card's agency
  framing; escalate to Davidson+Levin Ch.13.
  **Source:** Davidson & Levin (2014) Ch.9 pp.174-200.
- The card user needs the bid-ask OAS convention or the
  cross-dealer OAS comparison protocol for trading — out of
  CFA L1 and L2 scope.
  **Source:** Davidson & Levin (2014) Ch.10 pp.200-220.
