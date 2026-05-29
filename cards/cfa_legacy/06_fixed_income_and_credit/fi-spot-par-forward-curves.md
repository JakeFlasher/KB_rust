---
schema_version: "cacg.v0"
id: "fi-spot-par-forward-curves"
title: "Spot, Par, and Forward Curves"
reading_id: "06_fixed_income_and_credit"
summary: "Three rate curves describe the same term-structure surface: the spot (zero) rate z(t) discounts a single cashflow at horizon t; the par rate c(T) is the coupon rate that makes a fixed-coupon bond of maturity T price at par; the forward rate f(t1,t2) is the no-arbitrage rate implied between two future horizons. All three are equivalent representations of the discount-factor function D(t)."
tags: ["fixed-income", "spot-par"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2491:3668"
    chunk_hash: "207efbf69f2844f0c3bbc1ed71b2e2b9d50c7eb5cc693e0dfff7a323b8686535"
    page_range: [2491, 2492]
    quote: "A forward rate is the interest rate on a bond or money market instrument traded in a forward market"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p104:0153"
    chunk_hash: "0685db0df6f9513e50c2de4210f23e8b5cc11e1e0970498aa147a5049753462f"
    page_range: [104, 105]
    quote: "The n-year zero-coupon interest rate is the rate of interest earned on an investment that starts today and lasts for n years"
    edge_type: "supports"
  - source_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed"
    chunk_id: "fi_brigo_mercurio_2006_interest_rate_models_2ed:p062:0097"
    chunk_hash: "4fa23726dc19daac0b1316cf7af329bd708771702838f4d29a5fd93e745bb82b"
    page_range: [62, 63]
    quote: "Forward rates are interest rates that can be locked in today for an investment in a future time period, and are set consistently with the current term structure of discount factors"
    edge_type: "supports"
card_hash: "d1b339b3b2321caba71748cbcc9099678a17a66de165f400ddfacd1feb697b7b"
---
# Spot, Par, and Forward Curves

## Intuition

Three rate curves describe the same term-structure surface:
the spot (zero) rate `z(t)` discounts a single cashflow at
horizon `t`; the par rate `c(T)` is the coupon rate that
makes a fixed-coupon bond of maturity `T` price at par; the
forward rate `f(t_1, t_2)` is the rate implied between two
future horizons. They are three views of the same discount-
factor function `D(t)`. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.130-160.

```
<!-- primitive: yield-curve source: _diagram_primitives.md -->
yield (%)
   ^                                       forward
   |                                . - - .
   |                              .         . - - .
   |                            *                  par
   |                       *                   ====
   |                  *                  zero
   |             o
   |        o
   |   o
   +-----------------------------------------------> T (years)
   0    1     2     3     5     7    10    20    30
   legend:  o zero rate    * par rate    . forward
```

## Definition

The spot (zero) rate `z(t)` is defined by `D(t) = exp(-z(t)
· t)` (continuous compounding) or `D(t) = (1 + z(t))^(-t)`
(annual compounding). It discounts a single risk-free
cashflow at horizon `t`. **Source:** CFA L1 Curriculum
(2022) Vol.5/pp.130-150;
Hull §4 pp.84-110.

The par rate `c(T)` is the coupon rate at which a fixed-
coupon bond of tenor `T` prices at par (price = face). For
annual compounding it solves
`1 = sum_{i=1}^{T} c(T) · D(i) + D(T)`. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.140-160; Hull §4 pp.84-110.

The forward rate `f(t_1, t_2)` between horizons `t_1 < t_2`
is the rate implied today by no-arbitrage: it makes the
investment "buy `D(t_2)` directly" equivalent to "buy
`D(t_1)` and roll forward at `f(t_1, t_2)`". For continuous
compounding,
`f(t_1, t_2) = (z(t_2) · t_2 - z(t_1) · t_1) / (t_2 - t_1)`.
**Source:** Hull §4 pp.95-110;
Brigo+Mercurio (2006) §1.4 pp.10-35.

## Mathematical Reasoning

The three curves are equivalent representations of `D(t)`:
given any one, the others are recoverable by no-arbitrage
algebra. The mapping is informationally lossless on a
no-arbitrage market. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.140-160; Hull §4 pp.95-110.

Par rates are derived quotes, not additive pricing primitives:
combining bonds requires summing discounted cashflows, not
averaging coupon rates or par rates. The spot / discount-factor
curve is the linear pricing primitive. **Source:** CFA L1
Curriculum (2022) Vol.5/pp.140-160; Brigo+Mercurio (2006)
§1.4 pp.10-35.

The forward curve is the marginal rate: when the spot curve
is upward-sloping, the forward curve lies above it; when
flat, all three coincide; when inverted, forwards lie below
spots. The relationship is not assumption-laden — it follows
from the discount-factor algebra alone. **Source:** Hull §4
pp.95-110.

The yield-to-maturity from
[`fi-yield-and-price-mechanics.md`](./fi-yield-and-price-mechanics.md#mathematical-reasoning)
is a single internal rate that flattens the term structure
to a constant level; the spot curve preserves the term
structure's shape and is the preferred input to bond
portfolio pricing. **Source:** CFA L1 Curriculum (2022)
Vol.5/pp.130-160.

## See Also

- [`fi-yield-and-price-mechanics.md`](fi-yield-and-price-mechanics.md) — pricing under a single yield vs the full term structure
- [`fi-term-structure-theories.md`](fi-term-structure-theories.md) — economic theories of the curve's shape

## Escalate to Raw When

Open Hull Chapter 4 or Brigo+Mercurio Chapter 1 directly when
any of the criteria below applies. **Source:** Hull §4
pp.84-110; Brigo+Mercurio (2006) §1.4 pp.10-35.

- Bootstrapping a zero curve from a vector of par bond prices
  requires a specific interpolation choice (linear in zero,
  cubic spline, etc.) whose effect this card does not
  develop. **Source:** Hull §4 pp.95-110.
- Forward-rate agreement (FRA) and swap pricing apply the
  same forward-rate machinery in a derivative context;
  follow [`fi-term-structure-theories.md`](./fi-term-structure-theories.md#mathematical-reasoning)
  and Hull's swap chapter for that extension. **Source:**
  Hull §4 pp.95-110.
- Multi-curve / collateralized discounting (post-2008
  practice) departs from the single-curve assumption used
  here and requires a basis-curve framework. **Source:**
  Brigo+Mercurio (2006) §1.4 pp.10-35.
