---
schema_version: "cacg.v0"
id: "fi-default-models-and-recovery"
title: "Default Models and Recovery"
reading_id: "06_fixed_income_and_credit"
summary: "Default Models and Recovery — CFA Vol.5/pp.350-380 (PDF 2987-3017) falls in alternative investments/real estate; outside FI range Vol.5 pp.1-145."
tags: ["fixed-income", "default-models"]
citations:
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p129:0154"
    chunk_hash: "f258aaf50ca4973d22cd39843efbbc867bc1793982ee090aabab73d49deafea8"
    page_range: [129, 130]
    quote: "To better understand what the intensity model tries to achieve, let us briefly recall the notion of a hazard rate and its link to conditional default probabilities."
    edge_type: "defines"
card_hash: "5d26672a4cf46b30ea102a20365b166dbd53447b60afe60b8924c29536517e44"
---
# Default Models and Recovery

## Intuition

Two model families dominate credit pricing. Structural
models (Merton 1974 and successors) treat default as the
event when issuer asset value falls below a debt
threshold; default probability is a function of the
issuer's leverage, asset volatility, and tenor. Reduced-
form (intensity) models treat default as an unpredictable
jump governed by a hazard rate; calibration targets
observed spreads directly. Both produce expected-loss
implied spreads consistent with the credit-triangle
machinery from
[`fi-credit-spread-machinery.md`](./fi-credit-spread-machinery.md#definition).
**Source:** Lando (2004) §2-§3 pp.60-130.

```
Merton structural view
   asset value V(t)
   ^
   |   *
   |    *  V(t) ~ GBM
   |     *           *
   |      * *        *
   |         *    *
   |           * *
   |     ----------------- D (debt threshold)
   |      *
   |   default at first hitting (T) when V(T) < D
   +-----------------------------------> t
                                         T
```

## Definition

The Merton structural model assumes asset value `V(t)`
follows a geometric Brownian motion under the risk-
neutral measure. Equity is a call option on `V` struck at
the debt face `D`; debt is the no-arbitrage residual.
Default at maturity occurs iff `V(T) < D`; default
probability follows from the standard log-normal
distribution. **Source:** Lando (2004) §2 pp.60-90.

Reduced-form (intensity) models treat default as a Cox
process: the default time `τ` has hazard rate `h(t)` (or
stochastic `h(t, ω)`), so survival probability is
`S(t) = E[exp(-∫_0^t h(s) · ds)]`. Pricing follows from
the credit-triangle algebra of
[`fi-credit-spread-machinery.md`](./fi-credit-spread-machinery.md#mathematical-reasoning).
**Source:** Lando (2004) §3 pp.90-130.

Recovery conventions: Recovery of Face Value (RFV) pays
`R · F` at default; Recovery of Market Value (RMV) pays
`R · P_pre-default(τ)`; Recovery of Treasury Value (RTV)
pays `R · P_treasury(τ)`. Practitioner default
parameterization typically uses RFV with `R ≈ 0.4` for
unsecured corporate bonds, with senior secured higher and
subordinated lower. **Source:** Lando (2004) §2-§3
pp.60-130; CFA L1 Curriculum (2022) Vol.5/pp.350-380.

## Mathematical Reasoning

Structural models predict default probabilities as a
function of leverage and asset volatility; calibration
targets historical default rates and observed equity
volatility. The implicit asset volatility is challenging
to estimate (asset value isn't directly observed) and
predictions of short-tenor spreads can be too low because
the model assigns near-zero default probability when the
issuer is far from the debt threshold. **Source:** Lando
(2004) §2 pp.60-90.

Reduced-form models calibrate directly to observed CDS
spreads or bond yields, side-stepping the asset-value
identification problem. The cost is that they are silent
on the economic mechanism of default — `h(t)` is a
fitted nuisance, not a structural quantity. **Source:**
Lando (2004) §3 pp.90-130.

The credit-spread machinery from
[`fi-credit-spread-machinery.md`](./fi-credit-spread-machinery.md#mathematical-reasoning)
is the natural home of reduced-form pricing; structural
models map to specific hazard-rate dynamics by deriving
`h^Q(t)` from the asset-value / leverage process. The
two views can be reconciled but rarely calibrate to the
same observed spread surface without additional
adjustments. **Source:** Lando (2004) §2-§3 pp.60-130.

The recovery convention matters: with RFV, the recovery
payment doesn't depend on the bond's market price; with
RMV, recovery and pre-default price interact, producing a
non-linear pricing equation. RTV is rare in practice but
appears in some sovereign distressed exchanges.
**Source:** Lando (2004) §3 pp.90-130.

## See Also

- [`fi-credit-risk-fundamentals.md`](fi-credit-risk-fundamentals.md) — qualitative default / recovery / spread primitives
- [`fi-credit-spread-machinery.md`](fi-credit-spread-machinery.md) — quantitative hazard-rate decomposition
- [`../08_convertible_bonds/cb-default-and-recovery.md`](../08_convertible_bonds/cb-default-and-recovery.md) — convertible-bond default-recovery treatment

## Escalate to Raw When

Open Lando Chapters 2-3 directly when any of the
criteria below applies. **Source:** Lando (2004) §2-§3
pp.60-130.

- KMV / commercial structural-model calibration is
  required; this card frames the family but does not
  develop proprietary commercial implementations.
  **Source:** Lando (2004) §2 pp.60-90.
- Stochastic recovery (recovery correlated with default
  intensity) is in scope; this card uses constant
  recovery for the credit-triangle approximation.
  **Source:** Lando (2004) §3 pp.90-130.
- Sovereign distressed-exchange / restructuring credit
  events are in scope; corporate default mechanics
  differ from sovereign cases. **Source:** Lando (2004)
  §3 pp.90-130.
