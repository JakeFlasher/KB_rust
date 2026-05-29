---
schema_version: "cacg.v0"
id: "deriv-implied-volatility"
title: "Implied Volatility"
reading_id: "07_derivatives_and_volatility"
summary: "Implied volatility σ_imp(K, T) is the unique σ that recovers an observed market option price under the BSM closed form. The map is well-defined because BSM is monotone-increasing in σ; practitioners invert by Newton-Raphson on vega. Implied vol is the market's risk-adjusted expectation under the risk-neutral measure, NOT a forecast of realized vol — the wedge is the variance risk premium."
tags: ["derivatives", "implied-volatility"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p460:0680"
    chunk_hash: "dbb1a951b4681c9a970c0f4aa926883892d32533cea1166dd5552bc0a4c7a566"
    page_range: [460, 460]
    quote: "The formulas for delta and other Greek letters in Chapter 19 assume that the implied volatility remains the same when the asset price changes."
    edge_type: "defines"
card_hash: "82631e5556ddd510241f9eee909c127b02cf352735be5268dc079339aa2d6c12"
---
# Implied Volatility

## Intuition

Implied volatility is the volatility input that makes the BSM
formula return an observed market option price. It is a market
quote in volatility units rather than dollar units, and it
sidesteps the question of "what is the right vol?" by letting
the market's option prices answer. A constant implied vol
across strikes and tenors would mean BSM fits the market; in
practice implied vol varies systematically across the surface,
the source of the smile / skew / term-structure features
covered in the next card. **Source:** Hull §20 pp.460-475.

```
implied vol as the BSM inverse

   market price                    implied vol
   C_market(K, T)         ->       sigma_implied(K, T)
                       (BSM inverse)
                   such that
   BSM(S, K, T, r, sigma_implied) = C_market

   the BSM closed form is monotone in sigma, so the inverse
   exists and is unique whenever the market price is in the
   no-arbitrage range derived in deriv-no-arbitrage-bounds.
```

## Definition

**Implied volatility** `σ_imp(K, T)` for a European option with
strike `K` and expiry `T` is the unique `σ` such that the BSM
closed form
`C_BSM(S, K, T, r, σ) = C_market(K, T)`,
where `C_market` is the observed market call price (or
analogously for the put). The implied vol exists when the market
price falls strictly inside the no-arbitrage call bounds
`max(S - K · exp(-r · T), 0) ≤ C_market ≤ S`; outside those
bounds no `σ ≥ 0` recovers the price and the market quote is
mis-priced or stale. **Source:** Hull §20 pp.460-475; CFA L1
Curriculum (2022) Vol.5/pp.440-450.

The **at-the-money (ATM)** implied vol is the implied vol at
the strike closest to the spot (or sometimes at the
forward-strike `K = F_0 = S · exp(r · T)`). ATM vol is the
practitioner benchmark: it is the implied vol most directly
comparable to historical realized volatility and the most
liquid quote on most option markets. The full
`σ_imp(K, T)` surface across all (`K`, `T`) cells is treated
in [`deriv-vol-surface-anatomy.md`](deriv-vol-surface-anatomy.md#definition).
**Source:** Hull §20 pp.460-475.

## Mathematical Reasoning

The BSM call price is monotone strictly increasing in `σ` for
any fixed `(S, K, T, r)`, so the inverse map from market price
to `σ_imp` is well-defined. Practitioners compute the inverse
by Newton-Raphson on the BSM-vega derivative
`vega(σ) = S · sqrt(T) · N'(d_1(σ))`,
which converges quadratically because vega is positive and
bounded away from zero in the relevant region. The Brent /
bisection fallback is used near the boundary where vega
approaches zero (deep in-the-money or out-of-the-money quotes).
**Source:** Hull §20 pp.460-475.

Implied vol's units depend on the time convention. The standard
practitioner quote is annualized volatility in percentage
terms (e.g. "20% vol" means `σ = 0.20` per year), and the BSM
closed form's `T` is measured in years. Converting between
daily realized volatility and annualized implied volatility
uses the square-root-of-time scaling
`σ_annual = σ_daily · sqrt(252)` (252 trading days per year);
the convention is fragile to weekend / holiday adjustments and
to whether realized vol uses calendar-day or business-day
counts. **Source:** Hull §20 pp.460-475.

The implied vol is NOT a forecast of future realized vol; it is
the market's risk-adjusted vol expectation under the risk-
neutral measure. The wedge between implied and realized vol
(the variance risk premium) is positive on average for equity
indices because option writers demand compensation for tail
exposure. Implied vol is also model-dependent: it is the BSM
inverse, not the inverse of any other model, so any reference
to "the option's vol" should specify "BSM-implied vol" when the
smile is non-flat (because no single scalar describes a non-flat
surface). **Source:** Hull §20 pp.460-475.

## See Also

- [`deriv-bsm-formula.md`](deriv-bsm-formula.md) — BSM closed form whose vol input is the inverse of the market price
- [`deriv-vol-surface-anatomy.md`](deriv-vol-surface-anatomy.md) — full strike-by-tenor implied-vol surface and the smile / skew / term-structure features

## Escalate to Raw When

Open Hull chapter 20 directly when any of the criteria below
applies. **Source:** Hull §20 pp.460-485.

- Multiple smile-quoting conventions are in play (delta-strike
  vs strike-strike, sticky-strike vs sticky-delta dynamics);
  these matter for re-pricing a delta-hedged book between
  quote dates. **Source:** Hull §20 pp.475-485.
- Inverse-pricing instability near zero-vega strikes (deep
  ITM or OTM) requires arbitrage-free interpolation of the
  surface; that calibration uses local-vol or stochastic-vol
  models. **Source:** Hull §27 pp.626-660.
- A non-BSM model (Heston / SABR / jump-diffusion) is being
  calibrated; the model's "implied vol" is an algebraic
  artifact, not a direct observable. **Source:** Hull §27
  pp.626-660.
