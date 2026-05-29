---
schema_version: "cacg.v0"
id: "fi-cmo-tranching-mechanics"
title: "CMO Tranching Mechanics"
reading_id: "06_fixed_income_and_credit"
summary: "Collateralized mortgage obligation (CMO) tranching: sequential-pay, PAC/TAC support, IO/PO strip mechanics, and the cash-flow waterfall logic by which an MBS pool's prepayment-uncertain cash flows are redistributed across tranches with distinct prepayment-and-extension-risk profiles."
tags: ["fixed-income", "cmo-tranching"]
citations:
  - source_id: "fi_davidson_levin_2014_mortgage_valuation_models"
    chunk_id: "fi_davidson_levin_2014_mortgage_valuation_models:p042:0041"
    chunk_hash: "0e875862e59763beb3d0d16e1e6e8d4d712d853d9641e40bb176c506db0f21af"
    page_range: [42, 42]
    quote: "There are two main forms of structuring in the mortgage market: collateralized mortgage obligations (CMOs) and senior/subordinated."
    edge_type: "defines"
  - source_id: "fi_davidson_levin_2014_mortgage_valuation_models"
    chunk_id: "fi_davidson_levin_2014_mortgage_valuation_models:p157:0173"
    chunk_hash: "8c3d780e8366a4b2b235c8cb1445f13b924845a2ea2d9b8695f86cf99c5087db"
    page_range: [157, 157]
    quote: "The OAS term in (8.1) and (8.2) stands in recognition that, practically speaking, not all risk factors can be reflected in the dynamics of xt (), be it a scalar or a vector of many factors."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2526:3715"
    chunk_hash: "15df92724e2c12b32a6c8a49b7574c90b87ccb110d58b1ddd9e1ecd1a06f246a"
    page_range: [2526, 2527]
    quote: "The securitization process transfers ownership of assets such as loans or receivables from the original owners into a special legal entity."
    edge_type: "supports"
card_hash: "116f96f3d05312dfe55b68998a81d186c68d5d350ceaea8789b2687ccc415594"
---
# CMO Tranching Mechanics

## Intuition

A pass-through MBS distributes every dollar of principal-and-interest from the underlying pool to its holders pro rata — every holder faces the same prepayment uncertainty. A **collateralized mortgage obligation (CMO)** restructures the same underlying pool's cash flows into multiple tranches with distinct claim priorities. The simplest sequential-pay CMO directs all principal first to one tranche until it is fully retired, then to the next, and so on; the result is that the early-retired tranche has shorter expected life and less extension risk, while the later-retired tranches have longer expected life and more uncertainty. More sophisticated structures (PAC, TAC, IO/PO strips) refine the tranching to absorb specific prepayment-and-extension-risk dimensions. **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.

```
sequential-pay CMO cash-flow waterfall (3-tranche example)
   underlying pool cash flow at month t:
       interest_t + scheduled_principal_t + prepayment_t

   tranche allocation:
       Tranche A: receives all principal until A is retired
                  receives coupon-rate interest on its remaining balance
       Tranche B: receives no principal until A retires
                  receives coupon-rate interest meanwhile
                  takes all principal once A is gone, until B retires
       Tranche C: receives no principal until B retires
                  receives coupon-rate interest meanwhile
                  takes all principal once B is gone

   PAC structure (planned-amortization-class) with support:
       PAC tranche P: scheduled to receive principal on a deterministic
                      schedule between a low PSA and high PSA band
                      (e.g. 100-PSA to 250-PSA collar)
       Support tranche S: absorbs all prepayment fluctuation;
                      gets paid only after P's scheduled amount
                      is fulfilled; faces amplified prepayment-and-
                      extension risk
   PAC tranche P has more stable cash flows than the underlying
   pool; the support tranche S has more uncertainty. The total
   pool cash flows are conserved; tranching redistributes risk.

   IO / PO strip:
       IO (interest-only) strip: receives all interest cash flows,
                      no principal; value rises when prepays slow
       PO (principal-only) strip: receives all principal cash flows,
                      no interest; value rises when prepays accelerate
       IO + PO replicate the underlying pool's cash flows; they
       trade as opposite-direction bets on prepayment speed.
```

## Definition

A **CMO (collateralized mortgage obligation)** is a structured fixed-income security backed by a pool of mortgages (or by another MBS) whose cash flows are distributed across a set of tranches according to a pre-specified waterfall. The CMO's tranches carry distinct priorities on principal, interest, or combined cash flows. **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.

A **sequential-pay CMO** is the simplest structure: tranches are ordered, and principal flows entirely to the first tranche until it is retired, then to the next, etc. Interest is paid pro rata to all active tranches each month. The early-retired tranche has short expected average life; later tranches have longer expected lives. **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.

A **planned-amortization-class (PAC) tranche** is a tranche with a deterministic principal-payment schedule designed to be honored across a range of prepayment scenarios (the "PAC collar", typically expressed as a low-PSA to high-PSA band). The PAC tranche's cash flows are stable as long as actual prepayment falls within the collar. Outside the collar, the PAC's protection breaks down and its cash flows become uncertain. **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.

The **support tranche** (also "support bond" or "companion") absorbs the prepayment fluctuation that PAC tranches are insulated from. When actual prepayment exceeds the PAC's upper-collar speed, the excess principal goes to support tranches (accelerating their payoff); when actual prepayment falls below the lower-collar speed, the deficit comes out of support tranches (extending them). Support tranches therefore have amplified prepayment-and-extension risk relative to the underlying pool. **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.

An **IO (interest-only) strip** is a tranche that receives all of the pool's interest cash flows and none of the principal. An IO has a notional principal balance for interest-computation purposes but never receives principal; its value rises when prepayments slow (interest accrual continues longer). An **PO (principal-only) strip** is the complementary tranche that receives only the principal cash flows; its value rises when prepayments accelerate (principal arrives faster). IO + PO together replicate the underlying pool. **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.

A **TAC (targeted-amortization-class) tranche** is a one-sided PAC: it is protected against prepayment acceleration but not against deceleration (or vice versa). The TAC structure offers asymmetric protection at a different cost than a two-sided PAC. **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.

## Mathematical Reasoning

The waterfall logic conserves cash flow: at every month, the total principal-and-interest paid to all tranches equals the pool's total principal-and-interest. Tranching redistributes the uncertainty across tranches without creating or destroying cash. **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.

For a sequential-pay CMO with N tranches, the expected average life of tranche `i` depends on the pool's prepayment behavior and on the cumulative size of all earlier tranches. At any prepayment-speed scenario, the senior tranche retires first and has the shortest expected life; the junior tranches retire later and have progressively longer expected lives. The expected-life-vs-prepayment-speed sensitivity differs across tranches; the senior tranche is least sensitive (its retirement happens early regardless), while the junior tranches are most sensitive. **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.

For a PAC structure, the PAC tranche's principal schedule is the minimum of the principal flows that would be paid under the upper-PSA and lower-PSA boundary scenarios. As long as actual prepayment stays within the collar (between the two boundary scenarios), the PAC receives at least the scheduled amount; any excess principal goes to support tranches. The PAC's cash flows are therefore deterministic-up-to-the-collar; this is the structural insurance the PAC provides. **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.

The IO and PO strips have complementary prepayment-sensitivity profiles. The IO's value depends on the present value of future interest, which depends on how long the principal balance stays outstanding. When prepays accelerate, the principal balance drops faster and the future interest stream is curtailed — IO value falls. The PO's value depends on the timing of the principal cash flows; when prepays accelerate, the principal is received earlier and at a higher present value — PO value rises. **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.

The CMO tranching extends the L1 securitization scaffold from [`fi-securitization-fundamentals.md`](./fi-securitization-fundamentals.md#mathematical-reasoning) by adding structured-tranching mechanics on top of the pool-level cash flow. The L1 framework introduces the senior / mezzanine / junior tranching for credit-risk distribution; the CMO mechanics specialize this to prepayment-and-extension-risk distribution under the agency credit guarantee. **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.

The valuation of a CMO tranche under the OAS framework of [`fi-mbs-oas-calculation-depth.md`](./fi-mbs-oas-calculation-depth.md#mathematical-reasoning) requires running the Monte Carlo simulation at the pool level, then propagating the per-path cash flows through the waterfall to each tranche, then discounting under the path-specific short-rate process. Different tranches have different OAS values reflecting their different cash-flow profiles. **Source:** Davidson & Levin (2014) Ch.8 pp.145-173.

## See Also

- [`fi-securitization-fundamentals.md`](fi-securitization-fundamentals.md) — L1 securitization scaffold the CMO tranching specializes
- [`fi-prepayment-risk-intuition.md`](fi-prepayment-risk-intuition.md) — L1 prepayment / extension intuition the CMO structure redistributes
- [`fi-mbs-prepayment-models.md`](fi-mbs-prepayment-models.md) — prepayment-model cash flows feeding the CMO waterfall
- [`fi-mbs-oas-calculation-depth.md`](fi-mbs-oas-calculation-depth.md) — OAS Monte Carlo at the pool level extended to per-tranche valuation

## Escalate to Raw When

Open Davidson & Levin (2014) Ch.2 (Fundamentals of
Securitization) and Ch.8 (Engineering of Valuation Models
without Simulations) directly when any of the criteria below
applies. **Source:** Davidson & Levin (2014)
Ch.2 pp.22-38; Ch.8 pp.145-173.

- The card user needs the specific waterfall logic for a
  particular CMO deal (e.g. a Ginnie Mae REMIC issuance
  with named PAC and support tranches) at the offering-
  document level of detail.
  **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.
- A specific historical PAC-collar break (when actual
  prepayment fell outside the upper-PSA boundary and the
  PAC's protection eroded) requires the dated deal data
  Davidson+Levin discusses.
  **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.
- IO / PO valuation at desk-level numerical precision
  (specific path-by-path cash-flow allocation for an
  IO strip on a particular Fannie pool) requires the
  Monte Carlo infrastructure and dated pool data.
  **Source:** Davidson & Levin (2014) Ch.8 pp.145-173.
- Non-agency RMBS / private-label CMO structural mechanics
  with credit waterfalls (subordination, over-collateralization,
  excess spread accounts) are required — out of this card's
  agency framing; escalate to Davidson+Levin Ch.12-15 for the
  non-agency credit machinery.
  **Source:** Davidson & Levin (2014) Ch.2 pp.22-38.
