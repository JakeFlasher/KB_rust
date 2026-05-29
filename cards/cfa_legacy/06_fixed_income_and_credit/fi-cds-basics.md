---
schema_version: "cacg.v0"
id: "fi-cds-basics"
title: "CDS Basics"
reading_id: "06_fixed_income_and_credit"
summary: "Credit default swap (CDS): premium leg, protection leg, par-spread quotation, CDS-bond basis. Isolates credit risk from interest-rate risk via the premium-vs-protection PV balance at contract initiation."
tags: ["fixed-income", "cds-basics"]
citations:
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p227:0283"
    chunk_hash: "f6deb695118f4bac8edf40a1e8fa53e91030bd7f2a48ba9434f5d830502c7729"
    page_range: [227, 228]
    quote: "The same is true of the delivery option, which means that the protection seller does not know which bond is delivered and is likely to price the contract using an unfavorable choice."
    edge_type: "supports"
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p149:0179"
    chunk_hash: "a49f0a5d1477cb7258a7e55a32a240eb166080aa9a93a40d976ae9528aec14ed"
    page_range: [149, 150]
    quote: "In both approaches we will assume that the default boundary is known, and this requires a few comments first."
    edge_type: "supports"
card_hash: "63ff86e5e85285e14b289a2ba4a10d1d854661da3961c7ee5049d129610180be"
---
# CDS Basics

## Intuition

A credit default swap (CDS) is a bilateral contract: the
protection buyer pays a periodic premium; the protection
seller pays the loss-given-default if the reference entity
defaults. The CDS isolates credit risk from interest-rate
risk — pricing the premium leg vs the protection leg
yields the par CDS spread, the rate-of-return for credit
exposure as a stand-alone instrument. **Source:**
Crepey+Bielecki+Brigo (2014) §2 pp.20-60.

```
protection buyer                       protection seller
       |                                       |
       | -- periodic premium s * notional ---->|
       |       (premium leg)                   |
       |                                       |
       |<--- LGD * notional on default ----|---|
       |        (protection leg)               |
       |                                       |
       reference entity defaults at tau:
       no further premiums; payout (1 - R) * F
```

## Definition

A CDS contract has tenor `T`, notional `F`, par spread
`s`, and payment frequency `freq` (typically quarterly).
The premium leg is the buyer's stream of payments
`s · F · day_count / basis` until default or maturity.
The protection leg is the seller's payment of
`(1 - R) · F` if the reference entity defaults at any
time `τ ≤ T`. **Source:** Crepey+Bielecki+Brigo
(2014) §2 pp.20-60.

The par CDS spread is the value of `s` that makes the
present value of the premium leg equal the present value
of the protection leg at contract initiation. Standard
quoted spreads are par spreads at multiple tenors,
forming the CDS curve. **Source:** Lando (2004) §4
pp.130-170.

The CDS-bond basis is the difference between the par CDS
spread and the bond's Z-spread or asset-swap spread.
Positive basis (CDS > bond) historically arises when CDS
demand exceeds supply or when bond holders bear repo /
funding costs that bond spreads do not absorb. **Source:**
Crepey+Bielecki+Brigo (2014) §2 pp.20-60.

## Mathematical Reasoning

Pricing the premium leg uses the survival probability
`S(t) = exp(-∫_0^t h^Q(s) · ds)` from
[`fi-credit-spread-machinery.md`](./fi-credit-spread-machinery.md#definition).
Each premium payment is conditioned on survival to that
date; the premium-leg PV is
`s · F · ∑_i Δt_i · D(t_i) · S(t_i)` where `D` is the
risk-free discount factor. **Source:** Crepey+Bielecki+
Brigo (2014) §2 pp.20-60.

The protection-leg PV integrates the loss payment over
the default-time density:
`(1 - R) · F · ∫_0^T D(t) · h^Q(t) · S(t) · dt`.
Equating premium-leg and protection-leg PVs yields the
par CDS spread `s`. For flat hazard and recovery, this
recovers the credit-triangle approximation
`s ≈ h^Q · (1 - R)` from
[`fi-credit-spread-machinery.md`](./fi-credit-spread-machinery.md#mathematical-reasoning).
**Source:** Crepey+Bielecki+Brigo (2014) §2 pp.20-60;
Lando (2004) §4 pp.130-170.

The CDS market provides a direct read on `h^Q(t)` because
spreads are quoted across tenors and bootstrapping the
hazard curve from CDS quotes is a well-defined inverse
problem. CDS-implied default probabilities are typically
higher than rating-agency implied probabilities for the
same issuer; the wedge is the credit risk premium per
[`fi-credit-spread-machinery.md`](./fi-credit-spread-machinery.md#mathematical-reasoning).
**Source:** Lando (2004) §4 pp.130-170.

The default-recovery branching from
[`fi-default-models-and-recovery.md`](./fi-default-models-and-recovery.md#mathematical-reasoning)
governs the protection-leg payoff; CDS contracts
typically use a fixed `R = 40%` for unsecured corporate
references in standard ISDA-defined contracts. **Source:**
Crepey+Bielecki+Brigo (2014) §2 pp.20-60.

## See Also

- [`fi-credit-spread-machinery.md`](fi-credit-spread-machinery.md) — hazard-rate decomposition feeding CDS pricing
- [`fi-default-models-and-recovery.md`](fi-default-models-and-recovery.md) — recovery conventions
- [`fi-counterparty-risk-cva.md`](fi-counterparty-risk-cva.md) — counterparty-credit valuation that CDS pricing extends to

## Escalate to Raw When

Open Crepey+Bielecki+Brigo Chapter 2 directly when any
of the criteria below applies. **Source:** Crepey+
Bielecki+Brigo (2014) §2 pp.20-60.

- A specific bootstrapping algorithm for `h^Q(t)` from
  observed CDS quotes is required; this card frames the
  pricing equation but not the inverse calibration.
  **Source:** Crepey+Bielecki+Brigo (2014) §2 pp.20-60.
- CDS index products (CDX, iTraxx) or first-to-default
  baskets are in scope; the single-name model breaks
  under default correlation. **Source:** Lando (2004)
  §4 pp.130-170.
- Restructuring credit events have non-trivial
  cashflow impact (cheapest-to-deliver convention,
  modified restructuring); deal-specific terms apply.
  **Source:** Crepey+Bielecki+Brigo (2014) §2
  pp.20-60.
