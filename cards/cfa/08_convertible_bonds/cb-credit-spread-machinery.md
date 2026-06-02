---
schema_version: "cacg.v0"
id: "cb-credit-spread-machinery"
title: "Credit-Spread Machinery for Convertibles"
reading_id: "08_convertible_bonds"
summary: "Credit-Spread Machinery for Convertibles — placeholder summary                  "
tags: ["convertible-bonds", "credit-spread"]
citations:
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p102:0122"
    chunk_hash: "60ff35e0a1b878067742233d3b9a780d017c970cd6f19d549f62a1df576d872a"
    page_range: [102, 103]
    quote: "The hazard function is of particular interest in default modeling because of its link to conditional default probabilities, which is similar to the link we saw in the discrete-time case."
    edge_type: "defines"
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p102:0122"
    chunk_hash: "60ff35e0a1b878067742233d3b9a780d017c970cd6f19d549f62a1df576d872a"
    page_range: [102, 103]
    quote: "so h(t) t is approximately the conditional probability of a default in a small interval after t given survival up to and including t."
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p651:0967"
    chunk_hash: "c04abce6411df0c614d161917476d374cfabf4393d43ac4e6e20224be5a3e12f"
    page_range: [651, 651]
    quote: "Credit risk plays an important role in the valuation of convertibles. If credit risk is ignored, poor prices are obtained because the coupons and principal payments on the bond are overvalued."
    edge_type: "supports"
card_hash: "7e586a51f05d3d73bbf0901d57a079745b5122c519aede90a6b5cd36544e6390"
---
# Credit-Spread Machinery for Convertibles

## Intuition

A convertible's bond leg is **credit-risky**: the issuer can default
at any time, defaulting cashflows include only a recovery fraction
`R · F` of face. The hazard rate `h(t)` is the instantaneous
risk-neutral probability of default per unit time, and it maps
approximately to the bond-yield spread `s` over riskless rates via
the practitioner-quoted **credit-triangle approximation**
`s ≈ h · (1 − R)`. The relationship is a small-spread / small-
default-interval approximation, not an identity for defaultable
bond yields; the exact spread depends on the recovery convention,
the term structure of `h`, and the discount factor. The
approximation is accurate enough for routine
spread-↔-hazard translation in production pricing systems.
**Source:** Lando (2004) §3-§5 pp.60-130.

```
mapping spread ↔ hazard ↔ risk-neutral default probability:

     bond spread  s        ←——  market-observed CDS / bond yield
                  ↓
     hazard rate  h ≈ s / (1 − R)        (credit-triangle approx.)
                  ↓
     survival     P(τ > t) = exp( -∫_0^t h(u) du )
                  ↓
     PV recovery  R · F · D_rf · (1 − P(τ > T))
```

## Definition

Let `τ` denote the (random) default time of the issuer. The **hazard
rate** `h(t)` is the instantaneous default rate conditional on no
prior default. **Source:** Lando (2004) §3 pp.60-90.

- **Reduced-form intensity model**: `τ` is the first-jump time of an
  inhomogeneous Poisson process with intensity `h(t)`. The risk-
  neutral survival function satisfies `P^Q(τ > t) = exp(− ∫_0^t h(u)
  du)`. **Source:** Lando (2004) §3 pp.60-90.
- **Risk-neutral default probability** over `[0, T]`: `P^Q(τ ≤ T) = 1
  − exp(− ∫_0^T h(u) du)`. **Source:** Lando (2004) §3 pp.65-95.
- **Defaultable zero-coupon bond price** (no recovery): `B^d(0, T) =
  D_rf(0, T) · P^Q(τ > T)`. **Source:** Lando (2004) §3 pp.70-100.
- **Recovery convention**: practitioners use either **face-value
  recovery** `R · F` paid at default, **market-value recovery** `R ·
  V_pre-default(τ)`, or **Treasury-value recovery** `R · D_rf(τ, T) ·
  F`. The face-value convention is the simplest and the most common
  default in convertible pricing trees. **Source:** Lando (2004) §5
  pp.100-130.

The **credit-spread mapping** is the practitioner-quoted relationship
between the observed yield spread `s` (over riskless) and the
risk-neutral hazard rate. **Source:** Lando (2004) §4 pp.85-115;
DeSpiegeleer et al. (2014) §3.6 pp.95-110.

```
credit-triangle approximation (small h Δt and constant h):

  s ≈ h · (1 − R)
```

In the convertible-pricing tree (see the
[binomial-tree card](./cb-binomial-tree-valuation.md#definition)), the
hazard rate `h(t)` is converted into a per-step default probability
`h(t_k) · Δt` and used to weight the default branch. **Source:** Hull
(recent ed.) §27.4 pp.650-653.

## Mathematical Reasoning

The defaultable bond price under face-value recovery is the sum of
the survival cashflow PV and the recovery PV. **Source:** Lando
(2004) §4 pp.85-115.

```
B^d(0, T) = Σ_k c · F · D_rf(0, t_k) · P^Q(τ > t_k)
            + F · D_rf(0, T) · P^Q(τ > T)
            + R · F · ∫_0^T D_rf(0, u) · h(u) · P^Q(τ > u) du
```

Differentiating the bond yield with respect to maturity at fixed `R`
yields the credit-triangle approximation `s ≈ h · (1 − R)` for small
`h Δt` and small default intervals; the exact spread for a
defaultable bond also depends on the term structure of `h`, the
discount factor, and the chosen recovery convention. A narrow
exact equality holds only in the protection-leg framing of the
continuous-time par-CDS premium under constant `h`, constant `R`,
and recovery paid at default — for ordinary defaultable-bond
yield spreads the relation is a working approximation, not an
identity. **Source:** Lando (2004) §4 pp.85-115.

The **credit-equity coupling** for convertibles is the structural
feature that the share-price diffusion `S(t)` and the hazard rate
`h(t)` may share common drivers, producing the **double-signed
gamma** dynamic discussed in the
[bond-floor card](./cb-bond-floor-investment-value.md#mathematical-reasoning)
and the
[Greeks card](./cb-greeks-delta-gamma-vega.md#mathematical-reasoning).
**Source:** DeSpiegeleer et al. (2014) §3.6 pp.95-130.

Two practitioner extensions of the constant-hazard baseline matter
for convertibles. **Source:** Lando (2004) §4-§5 pp.85-130.

- **Equity-coupled hazard**: `h(t) = h_0 · (S_0 / S(t))^η` for some
  positive elasticity `η`; produces the qualitative double-signed
  gamma behavior because hazard rises as `S` falls. **Source:** Lando
  (2004) §5 pp.110-130.
- **Doubly-stochastic intensity**: `h(t)` is itself a stochastic
  process driven by macro factors (rates, credit indices); requires
  Monte Carlo or a multi-factor tree. **Source:** Lando (2004) §3
  pp.65-95.

The **default-event PV** under a constant `R` and constant `h` is
recoverable in closed form. **Source:** Lando (2004) §4 pp.85-115.

```
Recovery PV (constant h, constant R, T-maturity):

  PV_rec(0)  =  R · F · h / (r + h) · ( 1 − e^(-(r+h) T) )
```

Asymptotic regimes (cases below). **Source:** Lando (2004) §3-§4
pp.60-115; DeSpiegeleer et al. (2014) §3.6 pp.95-130.

- `h → 0`: defaultable bond → riskless bond; `s → 0`; the
  credit-aware tree's default branch carries zero weight; the
  bond-plus-call identity from the
  [payoff-decomposition card](./cb-payoff-decomposition-bond-plus-call.md#mathematical-reasoning)
  becomes exact. **Source:** Lando (2004) §3 pp.60-90.
- `h → ∞`: `B^d(0, T) → R · F · D_rf(0, t_default_avg)`;
  practitioner shorthand: "the bond trades flat at recovery". The
  convertible's bond floor collapses to the recovery floor.
  **Source:** Lando (2004) §4 pp.85-115.
- Equity-coupled `h(S, t)` with rising `h` as `S → 0`: the
  bond-floor stress regime, where the convertible's gamma can flip
  sign. **Source:** DeSpiegeleer et al. (2014) §3.6 pp.95-130.

## See Also

- [`cb-credit-vs-equity-decomposition.md`](cb-credit-vs-equity-decomposition.md) — the practitioner-quoted credit/equity ratios fed by `h`
- [`cb-binomial-tree-valuation.md`](cb-binomial-tree-valuation.md) — the tree consumes `h` in its default branch
- [`cb-bond-floor-investment-value.md`](cb-bond-floor-investment-value.md) — the survival/recovery split underlying `B(t)`
- [`cb-default-and-recovery.md`](cb-default-and-recovery.md) — recovery models and `R` conventions
- [`cb-china-default-cohort-attribution.md`](cb-china-default-cohort-attribution.md) — Chinese-CB life-cycle credit-risk taxonomy applying the hazard-rate framework to post-2014 default cohort attribution

## Escalate to Raw When

Open Lando §3-§5 pp.60-130 directly for the rigorous treatment of
intensity-based default models, doubly-stochastic intensities, and
recovery conventions. **Source:** Lando (2004) §3-§5 pp.60-130.

Open DeSpiegeleer §3.6 pp.95-130 for the practitioner's hybrid-
securities credit-spread machinery, including the equity-coupled
hazard parametrization. **Source:** DeSpiegeleer et al. (2014) §3.6
pp.95-130.

Open Hull §27.4 pp.650-653 for the conversion of `h(t)` into the
credit-aware tree's default-branch probability. **Source:** Hull
(recent ed.) §27.4 pp.650-653.
