---
schema_version: "cacg.v0"
id: "fi-yield-curve-construction"
title: "Yield Curve Construction"
reading_id: "06_fixed_income_and_credit"
summary: "Curve construction inverts the pricing operation: given a vector of bond prices (deposits at the short end, futures/FRAs in the middle, par swaps at the long end), bootstrap each tenor's zero rate iteratively, then choose an interpolation rule (linear-in-zero, cubic spline, monotone cubic in log-discount factor). Small zero-rate differences amplify into large forward-rate differences."
tags: ["fixed-income", "yield-curve"]
citations:
  - source_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed"
    chunk_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed:p062:0097"
    chunk_hash: "4fa23726dc19daac0b1316cf7af329bd708771702838f4d29a5fd93e745bb82b"
    page_range: [62, 63]
    quote: "Forward rates are interest rates that can be locked in today for an investment in a future time period, and are set consistently with the current term structure of discount factors"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p106:0156"
    chunk_hash: "749b45e653901225273ed35eb70ef52cee0e5f7c4d69e72781b4d3bdc5620ae7"
    page_range: [106, 107]
    quote: "4.7 DETERMINING ZERO RATES In this section we describe a procedure known as the bootstrap method which can be used to determine zero rates"
    edge_type: "supports"
  - source_id: "fi_veronesi_2010_fixed_income_securities"
    chunk_id: "fi_veronesi_2010_fixed_income_securities:p096:0130"
    chunk_hash: "7ae7ef1a7b08f2d73a9c3ce07b7c15db3e20cc866c838a15a995afff192580a6"
    page_range: [96, 97]
    quote: "Compute the semiannual yield curve, spanning over 9 years, from the data using the bootstrap procedure"
    edge_type: "supports"
card_hash: "8658639c1a99fb014f32d7e5dbecb85f8731ef2a6f55cd2356e01781ed5b34f6"
---
# Yield Curve Construction

## Intuition

The market doesn't quote a continuous zero curve — it
quotes a vector of bond prices (or par-rate / swap-rate
quotes). Curve construction is the inverse problem:
given the price vector, find a function `D(t)` such
that the implied bond prices match the market. The
inverse is under-determined (finitely many quotes,
infinitely many possible curves), so the modeler chooses
an interpolation rule. **Source:** Brigo+Mercurio
(2006) §1.4 pp.1-30.

```
quoted bond prices (or swap rates):
   tenor: 3M  6M  1y  2y  3y  5y  7y  10y 20y 30y
   price: o   o   o   o   o   o   o   o   o   o
                |
                | bootstrap each tenor's
                v zero rate using prior tenors
   inferred zero curve:
   z(t)
       *  *
       o    *  *
            o    *
                 o    *
                      o   *
                          o
```

## Definition

Bootstrapping recovers zero rates iteratively: start at
the shortest tenor where the bond is essentially a
zero-coupon (or treat the first par bond's coupon as
known by assumption); use the implied `D(t_1)` to price
the next bond, solving for the new tenor's `D(t_2)`;
continue. The procedure is well-defined when bond prices
and tenors are consistent with no-arbitrage. **Source:**
Brigo+Mercurio (2006) §1.4 pp.1-30; Hull §4
pp.84-110.

Interpolation choices include: linear-in-zero (simple
but produces non-smooth forwards); cubic spline on zero
rates (smooth zero, jumpy forwards); monotone cubic
splines on log-discount factors (smooth forwards,
practitioner standard). **Source:** Brigo+Mercurio
(2006) §1.4 pp.1-30.

The market data vector typically combines deposits at
the short end, futures or FRAs in the middle, and par
swaps at the long end. The transition between
instrument types introduces calibration nuances
(convexity adjustment from FRAs to forwards, swap-vs-
deposit basis). **Source:** Hull §4 pp.84-110.

## Mathematical Reasoning

The bootstrap is exact at quoted tenors and depends on
the interpolation rule between them. Different
interpolation choices produce different forward-rate
shapes even when the zero curve looks similar — small
differences in `z(t)` translate into large differences
in `f(t_1, t_2)` because the forward rate is a
difference of zero-rate cumulants. **Source:** Brigo+
Mercurio (2006) §1.4 pp.1-30.

The forward-rate algebra of
[`fi-spot-par-forward-curves.md`](./fi-spot-par-forward-curves.md#mathematical-reasoning)
makes this sensitivity precise: a non-smooth zero curve
produces a step-function forward curve, while a smooth
log-discount factor produces a smooth forward curve.
Practitioner curves favor smooth forwards because
forward rates are the inputs to derivative pricing.
**Source:** Brigo+Mercurio (2006) §1.4 pp.1-30.

Curve calibration is the inverse of the pricing
operation in
[`fi-yield-and-price-mechanics.md`](./fi-yield-and-price-mechanics.md#mathematical-reasoning):
pricing maps `(D(t_1), D(t_2), ..., D(t_N))` to
`(P_1, P_2, ..., P_N)`; bootstrapping inverts this map.
The Jacobian's conditioning depends on the bond set's
tenor coverage and coupon homogeneity. **Source:**
Brigo+Mercurio (2006) §1.4 pp.1-30.

Multi-currency / collateralized discounting introduces
basis spreads between curves and breaks the single-
curve discount-factor view. Post-2008 practitioner
calibration uses an OIS / collateralized curve for
discounting and a separate forwarding curve per
reference rate. **Source:** Brigo+Mercurio (2006)
§1.4 pp.1-30.

## See Also

- [`fi-spot-par-forward-curves.md`](fi-spot-par-forward-curves.md) — algebraic structure that bootstrapping fills in
- [`fi-yield-and-price-mechanics.md`](fi-yield-and-price-mechanics.md) — pricing forward direction that bootstrapping inverts

## Escalate to Raw When

Open Brigo+Mercurio Chapter 1 directly when any of the
criteria below applies. **Source:** Brigo+Mercurio
(2006) §1.4 pp.1-30.

- Multi-curve / OIS-discount calibration is needed;
  this card frames the single-curve case. **Source:**
  Brigo+Mercurio (2006) §1.4 pp.1-30.
- A specific interpolation algorithm (Hagan-West
  monotone cubic, tension splines) needs evaluation
  for stability and arbitrage-freeness. **Source:**
  Brigo+Mercurio (2006) §1.4 pp.1-30.
- Convexity-adjustment from FRA / futures to forward
  rates is needed for short-end calibration accuracy.
  **Source:** Hull §4 pp.84-110.
