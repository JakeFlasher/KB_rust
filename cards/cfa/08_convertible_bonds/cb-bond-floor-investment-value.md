---
schema_version: "cacg.v0"
id: "cb-bond-floor-investment-value"
title: "Bond Floor (Investment Value)"
reading_id: "08_convertible_bonds"
summary: "The credit-risky straight-bond floor B(t) is the lower bound of a CB price: discounting the coupon-and-face stream at the risk-free rate plus credit spread gives the value of the convertible stripped of its conversion right; the floor is itself risky and collapses to recovery on default, so 'downside protection' is conditional on issuer solvency."
tags: ["convertible-bonds", "bond-floor"]
citations:
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p045:0050"
    chunk_hash: "36d0e915b9efae7a3e8aa407ef024af6169687bca4eb32a53e0778e7e98c95bf"
    page_range: [45, 46]
    quote: "The bond floor or the investment value is the value of the convertible if it were to be stripped of the possibility to convert into the underlying shares."
    edge_type: "defines"
  - source_id: "cb_calamos_2003_convertible_arbitrage"
    chunk_id: "cb_calamos_2003_convertible_arbitrage:p055:0057"
    chunk_hash: "0ca1a724558b37d6e66b90e371edb2cce9be116fa2e81654e650a7a5535643af"
    page_range: [55, 56]
    quote: "Credit-risk assessment is extremely critical in the valuation process for convertibles."
    edge_type: "supports"
  - source_id: "cb_lando_2004_credit_risk_modeling"
    chunk_id: "cb_lando_2004_credit_risk_modeling:p081:0094"
    chunk_hash: "1d65fc593814bbb3ebd8362b8696d3ad70649bb6f5953ba4f8f67480a35e0700"
    page_range: [81, 82]
    quote: "After debt and equity have been issued, it is the equity owners who decide when to default."
    edge_type: "supports"
card_hash: "bf8fd07718b2901ddacd4eb59a6c5bf5d9a37ef77da699be0e7768ab10c385c0"
---
# Bond Floor (Investment Value)

## Intuition

Strip the conversion feature out of a convertible bond and you are left with
a credit-risky corporate bond — coupons up to maturity, then face. Its
present value is the **bond floor** `B(t)`, sometimes called the
**investment value**. It is a lower bound for the convertible price for a
holder who never intends to convert.
**Source:** DeSpiegeleer et al. (2014) §2.4 pp.40-58; Calamos (2003) §3
pp.30-50.

```
value
  ^
  |          straight-bond price
  |            with given y, c, T
  |          .__.__.__.__.__.__. <-- B(t)  ("investment value")
  |
  |
  +--------------------------------> share price S
```

The bond floor is itself **risky**: when the issuer's credit spread widens
`B(t)` falls; on default `B(t)` collapses to a recovery payment, not face.
"Downside protection" is therefore conditional on the issuer remaining
solvent. **Source:** DeSpiegeleer et al. (2014) §2.4 pp.45-58.

## Definition

For a CB with face `F`, coupon `c`, payment dates `t_1 < ... < t_n = T`, and
issuer credit spread `s` over the riskless curve `r`, decompose the floor as
the sum of **survival cashflow PV** and **default recovery PV**, each evaluated
under a chosen recovery convention. **Source:** Lando (2004) §3-§4 pp.60-90.

    B(t) := B_surv(t) + B_rec(t)

    B_surv(t) := E^Q[ Σ_{k:t_k>t} c · F · e^(-∫_t^{t_k} (r+s) du) · 1_{τ>t_k}
                      + F · e^(-∫_t^T (r+s) du) · 1_{τ>T} | F_t ]

    B_rec(t)  := E^Q[ R · M(τ) · e^(-∫_t^τ r du) · 1_{t<τ≤T} | F_t ]

where `τ` is the default time, `R` the recovery rate, and `M(τ)` the
recovery base — face `F` (recovery-of-face), the prevailing market price
(recovery-of-market-value), or the riskless-discounted nominal (recovery-of-
treasury). The choice is a modelling primitive, not a derivable result.
**Source:** Lando (2004) §3-§4 pp.60-90.

A common practitioner shortcut is the **spread-shift** form, which folds
recovery and survival into a single discount; this is exact only when the
recovery contribution is absorbed into a constant adjustment to `s`, which
is rarely the case in stress. **Source:** DeSpiegeleer et al. (2014) §3.6
pp.95-110.

    B_practitioner(t) ≈ Σ_{k:t_k>t} c · F · e^(-∫_t^{t_k} (r+s) du)
                       + F · e^(-∫_t^T (r+s) du)

## Mathematical Reasoning

The bond-floor inequality `V(t) ≥ B(t)` is a no-arbitrage consequence of the
embedded conversion right being **a holder right, not an obligation**: a
holder who never converts collects exactly the credit-risky straight-bond
stream, so `V(t) - B(t) ≥ 0`. **Source:** DeSpiegeleer et al. (2014) §2.4
pp.40-50.

Stress asymptotics depend on the recovery convention (cases below).
**Source:** Lando (2004) §4-§5 pp.75-130; DeSpiegeleer et al.
(2014) §2.4 pp.40-50.

- As `S(t) → 0` and the issuer's credit deteriorates, `s(t)` widens, so
  `B_surv(t)` shrinks; `B_rec(t)` grows because default becomes more likely
  but is bounded above by `R · F`. The floor does not in general converge
  to a single closed-form limit; it depends on `R`, on the timing of `τ`,
  and on the recovery-of-X convention. **Source:** Lando (2004) §4
  pp.75-90.
- As `S(t) → ∞`, the issuer's credit improves and `B(t)` is bounded above
  by the riskless-discounted nominal stream `Σ c·F·D_rf + F·D_rf`, which is
  insensitive to `S(t)`. In this regime the equity component dominates the
  convertible price (see the
  [payoff-decomposition card](./cb-payoff-decomposition-bond-plus-call.md#mathematical-reasoning)).
  **Source:** Calamos (2003) §3 pp.30-45; DeSpiegeleer et al. (2014) §3.4
  pp.65-78.

Under credit-equity coupling — when issuer credit spread widens as `S(t)`
falls — the convertible's effective gamma can change sign relative to the
constant-spread case: the credit-driven drop in `B(t)` overlays the
equity-driven payoff curvature, and the convertible can lose value at a
rate steeper than a static-floor model predicts. DeSpiegeleer (2014)
calls this regime **double-signed gamma** and illustrates it with the
Renewable Energy CB case study (simultaneous equity and bond-floor
collapse). **Source:** DeSpiegeleer et al. (2014) §2.4 pp.45-58.

## See Also

- [`cb-bond-anatomy-and-cashflows.md`](cb-bond-anatomy-and-cashflows.md) — the cash-flow stream `B(t)` discounts
- [`cb-payoff-decomposition-bond-plus-call.md`](cb-payoff-decomposition-bond-plus-call.md) — `V(t) ≈ B(t) + N · c_call` decomposition

## Escalate to Raw When

Open Lando §3-§4 pp.60-90 directly when any criterion below applies.
**Source:** Lando (2004) §3-§4 pp.60-90.

- A reduced-form hazard-rate model is needed for `s(t)` calibration to CDS
  spreads or bond yields. **Source:** Lando (2004) §3 pp.60-75.
- The recovery model matters: `R` may be deterministic, stochastic, or a
  function of `S(t)` (recovery-of-treasury vs recovery-of-face vs
  recovery-of-market-value) — pick one consistently. **Source:** Lando
  (2004) §4 pp.75-90.

Open DeSpiegeleer §2.4 pp.45-58 when the **double-signed gamma** in distress
matters — i.e. the convertible is already trading near `B(t)` and the share
price is falling. **Source:** DeSpiegeleer et al. (2014) §2.4 pp.45-58.

For deeper interest-rate-curve dynamics (term-structure models, HJM, LMM)
the canonical reference will live in `06_fixed_income_and_credit/` (not yet
authored). **Source:** Lando (2004) §3 pp.60-75 (placeholder until
`06_fixed_income_and_credit/` cards land).
