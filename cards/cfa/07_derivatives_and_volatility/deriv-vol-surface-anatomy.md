---
schema_version: "cacg.v0"
id: "deriv-vol-surface-anatomy"
title: "Vol-Surface Anatomy"
reading_id: "07_derivatives_and_volatility"
summary: "The implied-vol surface is a strike-by-tenor grid showing BSM-implied vol for each cell; non-flat structure (smile, skew, term structure) encodes market beliefs that the underlying is not lognormal. Equity indices exhibit downward-sloping skew, FX shows symmetric smile, term structure varies with vol regime."
tags: ["derivatives", "vol-surface"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p451:0668"
    chunk_hash: "6ff9dbb671abbd6229c9160e2511e6a85f55ef032abacfade6c732963ce08190"
    page_range: [451, 451]
    quote: "A three-dimensional plot of the implied volatility as a function of both strike price and time to maturity is known as a volatility surface"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p456:0675"
    chunk_hash: "173f0b757b7a6e4f59af844b7f40947947a9b7bb9e06e04788a1787d17774737"
    page_range: [456, 456]
    quote: "Since 1987, the volatility smile used by traders to price equity options (both on individual stocks and on stock indices) has tended to look like that in Figure 20.3"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p463:0683"
    chunk_hash: "6ecd58cc3fb0592cba6179c823b79ae79c4ae0f94ba6b6eee7102b1f427c54ee"
    page_range: [463, 463]
    quote: "For equity options, the volatility smile tends to be downward sloping"
    edge_type: "defines"
card_hash: "5bbdd2cf38970e5b11b1b70cdf44f5cdae683d714b44f6b6ef42e402c3a5077f"
---
# Vol-Surface Anatomy

## Intuition

The implied-vol surface is a two-dimensional grid: strike on
one axis, tenor on the other, and BSM-implied vol in each cell.
A flat surface would mean BSM fits perfectly; in practice the
surface has structure. Equity indices show a downward-sloping
skew (puts are richer than calls); FX shows a smile (both wings
are richer than ATM); rates show term-structure features driven
by the future-rate distribution. Reading the surface is the
practitioner's first step in pricing any non-vanilla derivative.
**Source:** Hull §20 pp.460-475.

```
<!-- primitive: vol-surface-heatmap source: _diagram_primitives.md -->
implied vol      tenor (months) ->
strike            1m    3m    6m   12m   24m   36m
   |
   |  90% K       *     +     +     .     .     .
   |
   |  95% K       +     +     .     .     .     .
   |
   | 100% K       .     .     .     .     .     .
   |
   | 105% K       +     +     .     .     .     .
   |
   | 110% K       *     +     +     .     .     .
   v
                  legend: . low  + mid  * high
                  smile rises in K-wings; term flattens with t
```

## Definition

The **implied-vol surface** `σ_imp(K, T)` is the function
mapping each (`K`, `T`) cell to the BSM-implied vol that
recovers the corresponding market option price. The surface is
parameterized by either absolute strike `K` or relative strike
(forward `K / F_0`, log-moneyness `ln(K / F_0)`, or BSM-delta
`Δ`); the latter conventions normalize across spot levels and
are the practitioner default. **Source:** Hull §20 pp.460-475.

The surface's three canonical features are: the **smile**, where
implied vol increases as strike moves away from ATM in either
direction (predominant for FX); the **skew**, where implied vol
slopes monotonically across strike (downward for equity
indices, where put strikes carry higher vol); and the **term
structure**, where ATM implied vol varies with tenor (typically
upward-sloping in low-vol regimes and inverted in stress
regimes). The full surface combines all three features into a
strike-by-tenor map. **Source:** Hull §20 pp.475-485; CFA L1
Curriculum (2022) Vol.5/pp.445-455.

## Mathematical Reasoning

The smile and skew encode market beliefs that BSM's lognormal
underlying is an oversimplification. A put strike well below
spot earns a high implied vol because the market prices in fat
left tails (crash risk) that lognormality understates; a call
strike well above spot likewise earns a vol premium for fat
right tails (less common in equity indices, more common in
single-name commodities). The BSM-flat-vol benchmark is the
limit where the underlying truly is lognormal; the gap between
flat-BSM and observed surface is the markup-for-tail-risk that
local-vol and stochastic-vol models attempt to reproduce.
**Source:** Hull §20 pp.475-485.

The term structure of ATM vol reflects the time-aggregation of
short-horizon and long-horizon vol regimes. Under a stationary
stochastic-vol model the term structure flattens at long
horizons (the conditional vol asymptotes to its long-run mean);
under an unstable regime the long-horizon vol is dominated by
expected-future-state premia. The 06 short-rate-models
analogue is the term-structure of yields: a similar trade-off
between expectations of future short rates and term-premium
compensation. The cross-link is structural; the boundary stays
in 07 because the underlying is the equity / FX / commodity
spot, not a rate. **Source:** Hull §20 pp.475-485.

The cross-asset patterns differ in shape. Equity indices show
the strong downward-sloping skew because index puts are
dominantly demanded by long-only managers hedging downside
exposure. FX implied vol surfaces are typically smiles centered
near ATM-forward because FX has symmetric tail risk under the
typical risk-neutral measure (no dominant directional fear).
**Source:** Hull §20 pp.475-485. Interest-rate-derivative
implied-vol surfaces (caplets / floorlets / swaptions) sit
under separate Hull chapters covering rate-derivative pricing
machinery, including the SABR family for rates-vol; this card
stops at the equity / FX surface and treats rates-vol as an
escalation topic. **Source:** Hull §27 pp.640-660.

## See Also

- [`deriv-implied-volatility.md`](deriv-implied-volatility.md) — implied-vol definition that builds each cell of the surface
- [`deriv-local-volatility.md`](deriv-local-volatility.md) — local-vol model whose calibration to the surface reproduces the smile
- [`deriv-stochastic-vol-models.md`](deriv-stochastic-vol-models.md) — Heston / SABR generation of smile from underlying-and-vol joint dynamics

## Escalate to Raw When

Open Hull chapter 20 directly when any of the criteria below
applies. **Source:** Hull §20 pp.460-485.

- The card needs sticky-strike vs sticky-delta dynamics (how
  the smile shifts when spot moves), or vega-bucketed
  sensitivities for portfolio risk management. **Source:**
  Hull §20 pp.475-485.
- A parametric smile model (e.g. SABR-static) needs to be
  fitted to the surface; that uses the SABR machinery covered
  in Hull §27 pp.626-660. Other practitioner parameterizations
  outside the Hull / Glasserman / CFA scope (e.g. SVI) are
  deferred per the 07 style-guide tier-3 boundary and are not
  addressed here. **Source:** Hull §27 pp.626-660.
- The surface is the input to an exotic-option pricing model
  (barrier, Asian, lookback); local-vol or stochastic-vol
  calibration to the surface is required. **Source:** Hull
  §27 pp.626-660.
