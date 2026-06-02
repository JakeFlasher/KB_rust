---
schema_version: "cacg.v0"
id: "fi-term-structure-theories"
title: "Term-Structure Theories"
reading_id: "06_fixed_income_and_credit"
summary: "Three classical theories interpret the yield curve: pure expectations says forwards equal expected future spots; liquidity preference adds a positive term premium that grows with horizon; segmented-markets / preferred-habitat says tenor-specific supply / demand anchor the curve locally. Modern arbitrage-free models recast the premium as a risk-neutral / physical drift difference."
tags: ["fixed-income", "term-structure"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p116:0170"
    chunk_hash: "e8dd841e7e600a4da03aba94c5bed828ecf362359130238030c01444853e40b8"
    page_range: [116, 117]
    quote: "The simplest is expectations theory, which conjectures that long-term interest rates should reflect expected future short-term interest rates"
    edge_type: "defines"
  - source_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed"
    chunk_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed:p064:0100"
    chunk_hash: "0a6c8ca1c3630596246e2ff4cba71c2259ee1829f498b2b50be5fad3b4bdb6b3"
    page_range: [64, 65]
    quote: "Instantaneous forward rates are fundamental quantities in the theory of interest rates"
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2488:3664"
    chunk_hash: "96363d2837dd8d0e1f62a6906ea4b63771ad920f1f38182475aff8e9af7949ff"
    page_range: [2488, 2489]
    quote: "Sometimes, a spot curve is downward sloping in that shorter-term yields are higher than longer-term yields"
    edge_type: "supports"
card_hash: "5b2d544d51d953c9bdd6c0559402cb926db1ec9d31dbb4b5bad99d3d106583ae"
---
# Term-Structure Theories

## Intuition

The yield curve's shape — upward-sloping in normal regimes,
flat at cycle peaks, occasionally inverted before recessions
— is interpreted by three classical theories. The
expectations theory says forwards equal expected future
spots; liquidity preference adds a positive term premium;
segmented markets / preferred-habitat says supply and demand
at each tenor anchor the curve locally. The three theories
make different predictions about the relationship between
forward rates and realized future short rates. **Source:**
Hull §4 pp.95-110.

```
yield (%)
   ^
   |              .- - - .- - - .                expectations:
   |          .  .                .              forwards
   |       .                                     match expected
   |    .                                        spots
   |  o ----- spot curve today
   |
   | (under liquidity preference, forwards lie
   |  ABOVE expected future spots by a term
   |  premium that grows with horizon)
   |
   +---------------------------------------> T
        1y    5y    10y    20y    30y
```

## Definition

Pure-expectations theory: the forward rate `f(t_1, t_2)`
equals the market's expectation of the future spot rate
`E[z_{t_1}(t_2 - t_1)]`. The theory treats the forward rate as
an unbiased forecast by assuming away a term premium.
**Source:** Hull §4
pp.95-110;
CFA L1 Curriculum (2022) Vol.5/pp.150-180.

Liquidity-preference theory: holders of long-term bonds
demand a positive term premium `λ(t)` over the expected
short-rate path because long bonds carry more
mark-to-market and reinvestment risk. Forwards lie above
expected future spots:
`f(t_1, t_2) = E[z_{t_1}(t_2 - t_1)] + λ(t_2 - t_1)` with
`λ ≥ 0` and typically increasing in horizon. **Source:**
Hull §4 pp.95-110.

Segmented-markets / preferred-habitat: investors have tenor-
specific preferences (insurance liabilities, bank funding
horizons, pension-immunization needs) and the curve at each
tenor reflects local supply and demand. Forward rates are
determined locally and need not satisfy a global expectations
or premium structure. **Source:** Hull §4 pp.95-110;
CFA L1 Curriculum (2022) Vol.5/pp.160-180.

## Mathematical Reasoning

Under pure-expectations, the forward-curve algebra of
[`fi-spot-par-forward-curves.md`](./fi-spot-par-forward-curves.md#mathematical-reasoning)
identifies forward rates as the no-arbitrage rate between
two horizons; expectations theory adds the equilibrium claim
`f(t_1, t_2) = E[z_{t_1}(t_2 - t_1)]`. The empirical content
is testable: realized future short rates should match
forward predictions on average. Empirical tests reject this
prediction in many regimes, suggesting term premia exist.
**Source:** Hull §4 pp.95-110;
Brigo+Mercurio (2006) §1.5 pp.10-35.

Liquidity-preference adds a non-negative `λ(t)` term:
`f - E[z]` should be measurably positive on average, growing
with horizon. In arbitrage-free modeling, the same premium
question is handled through the difference between physical
and risk-neutral dynamics. **Source:** Hull §4 pp.95-110;
Brigo+Mercurio (2006) Ch.3-Ch.5 pp.58-190.

Segmented-markets is the weakest equilibrium claim: each
tenor's supply / demand is local and the curve fits between
the global no-arbitrage envelope and the agents' tenor-
specific preferences. It is consistent with curve-shape
shifts that the other two theories struggle to explain
(e.g. "twist" moves where short and long ends move opposite
directions). **Source:** Hull §4 pp.95-110;
CFA L1 Curriculum (2022) Vol.5/pp.160-180.

Modern arbitrage-free short-rate and forward-rate models
(Vasicek, CIR, HJM) do not rely on the classical theories as
pricing rules. They re-express the term-premium question as a
risk-neutral / physical-measure drift difference and calibrate
curve dynamics separately. The classical theories remain
conceptual scaffolding, not a substitute for model
calibration. **Source:** Brigo+Mercurio (2006) Ch.3-Ch.5
pp.58-190.

## See Also

- [`fi-spot-par-forward-curves.md`](fi-spot-par-forward-curves.md) — the no-arbitrage forward / spot algebra these theories interpret
- [`fi-yield-and-price-mechanics.md`](fi-yield-and-price-mechanics.md) — yield-to-maturity as a single-rate flattening of the curve

## Escalate to Raw When

Open Hull Chapter 4 or Brigo+Mercurio Chapter 1 directly
when any of the criteria below applies. **Source:** Hull
§4 pp.95-110; Brigo+Mercurio (2006) §1.5 pp.10-35.

- A specific short-rate model (Vasicek, CIR, Hull-White,
  HJM, BGM) needs to be calibrated and the term-structure
  theory's qualitative predictions must be cast as numerical
  parameter restrictions. **Source:** Brigo+Mercurio (2006)
  Ch.3-Ch.5 pp.58-190.
- Empirical measurement of term premia across cycles is
  required (e.g. surveys vs. options-implied forecasts).
  **Source:** Hull §4 pp.95-110.
- Multi-currency basis or collateralized-discounting
  decompositions are in scope; the single-currency
  single-curve view of this card does not apply.
  **Source:** Brigo+Mercurio (2006) §1.5 pp.10-35.
