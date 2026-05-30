---
schema_version: "cacg.v0"
id: "pa-fi-perturbational-attribution-equation"
title: "The Fixed-Income Perturbational Attribution Equation"
reading_id: "15_performance_and_attribution"
summary: "Taylor-expanding a bond's price in time and yield yields r ~= y*dt - MD*dy + (1/2)C*dy^2, the master identity that maps a security's risk numbers into return components with no pricing model; residual dr = rM - rC."
tags: ["fixed-income-attribution", "perturbational-equation", "duration-convexity"]
citations:
  - source_id: "pa_colin_2016"
    chunk_id: "pa_colin_2016:p126:0132"
    chunk_hash: "5f3c978442883ac44dd78a9f5937343bf296a3e17ad57179c74aa163ec368a83"
    page_range: [127, 127]
    quote: "Instead of requiring the details of how an arbitrary security is priced, it replicates the results of a returns calculation by using the security’s risk numbers: yield to maturity, modified duration and convexity."
    edge_type: "defines"
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p353:0422"
    chunk_hash: "c0580b90b4fd2e3641992a1c63558a8fce2b0c27d0ec0e611c0e2903e0be4634"
    page_range: [353, 353]
    quote: "There is an alternative vocabulary of yield changes rather than returns."
    edge_type: "supports"
card_hash: "9be4e5edac8b9077932b029929170666a886598e115d38a72f48bb385dbf8ec3"
---
# The Fixed-Income Perturbational Attribution Equation

## Intuition

Equity attribution starts from holdings and prices; fixed-income attribution starts from a different dialect entirely — bond managers think in yield changes, not directly in returns. The perturbational equation is the bridge between those two vocabularies. It says: do not bother re-pricing each bond from its cash flows. If you already know a security's yield, modified duration, and convexity — its *risk numbers* — those alone are enough to reconstruct its return, because price is a smooth function of yield and time. The risk numbers are the local slope and curvature of that function, so a first/second-order expansion replays the return without any pricing machinery.

**Source:** Colin (2016) §8.1, §8.3 pp.126-127 (printed 103-104)

The payoff is enormous in practice: one formula serves every bond in the portfolio and benchmark, no instrument-specific pricer is required, and new security types can usually be slotted into the same framework. The cost is that the expansion is local and single-cash-flow in spirit, so it never accounts *exactly* for the realised return — the gap it leaves is the residual.

**Source:** Colin (2016) §8.3 pp.128-129 (printed 105-106)

## Definition

Assume the price `p` of an arbitrary security is a function of time `t` and yield `y`. The **perturbational equation** (also called the fundamental attribution equation) approximates the security's return `r` as

```
r ~= y*dt - MD*dy + (1/2)*C*dy^2
```

where the defined quantities are `r = dp/p` (return), `y = (1/p)(dp/dt)` (the carry / yield-to-maturity term), `MD = -(1/p)(dp/dy)` (modified duration), and `C = (1/p)(d^2 p/dy^2)` (convexity). The expansion is carried to first order in time but second order in yield, because securities have small second-order dependence on time yet appreciable second-order yield dependence.

**Source:** Colin (2016) §8.3 eqs.(8.1)-(8.6) pp.126-127 (printed 103-104)

The equation is "essentially a proxy for any pricing function": instead of the details of how a security is priced, it replicates a returns calculation from the security's risk numbers (yield to maturity, modified duration, convexity). The **residual** `dr = rM - rC` is the difference between the realised *market* return `rM` (from market price plus coupons) and the *calculated* return `rC` (from the equation), and it captures whatever the local expansion misses.

**Source:** Colin (2016) §8.3, §8.4 pp.127, 130 (printed 104, 107)

Bacon frames the same object from the bond manager's side: fixed-income performance is driven by changes in the shape of the yield curve, expressed in "an alternative vocabulary of yield changes rather than returns" — exactly the change-of-variable the perturbational equation formalises.

**Source:** Bacon (2023) §6 "Fixed Income Attribution" pp.353-354 (printed 331-332)

## Mathematical Reasoning

Start from the Taylor expansion of price in its two arguments, keeping one time term and two yield terms:

```
dp = (dp/dt) dt + (dp/dy) dy + (1/2)(d^2 p/dy^2) dy^2 + O(dt^2, dy^3)
```

Divide throughout by `p`. The leading time coefficient becomes the carry/yield term `y = (1/p)(dp/dt)`; the first yield coefficient is `-MD` because modified duration is *defined* with a minus sign, `MD = -(1/p)(dp/dy)`, encoding the inverse price-yield relationship; the quadratic coefficient is `(1/2)C` with `C = (1/p)(d^2 p/dy^2)`. Substituting the definitions returns the decomposition

```
r ~= y*dt  -  MD*dy  +  (1/2)*C*dy^2
     carry   duration   convexity
```

so each named risk number is literally a normalised derivative of the price function. The signs are structural: positive `dy` (rising yields) feeds a negative duration contribution `-MD*dy` because modified duration is defined positive, while the convexity term enters at the (non-negative) factor `dy^2` with sign carried by `C`.

**Source:** Colin (2016) §8.3 eqs.(8.1)-(8.6) pp.126-127 (printed 103-104)

The identity is local (an expansion truncated after second order in yield, first order in time) and treats the security as having a single cash flow. Colin therefore does not claim it is exact: it is asserted as a proxy whose error appears as the residual `dr = rM - rC`. The book labels — rather than proves away — its limits: securities with extra return sources (inflation-linked bonds), multiple sensitivities (an FRN has separate risk-free-curve and credit-spread durations), large bullet payments handled via key-rate durations, or instruments with no carry (some bond futures) all strain the single-risk-measure assumption.

**Source:** Colin (2016) §8.3 pp.129-130 (printed 106-107)

```
 PRICE FUNCTION p(t, y)                RISK-NUMBER PROXY (no pricer)
 ---------------------                 ----------------------------
   true rM from                          y*dt        (carry / time)
   re-pricing  ........ approximated by  - MD*dy     (1st-order yield)
   with & without                        + (1/2)C dy^2 (2nd-order yield)
   each risk                                  |
        |                                     v
        +----------- residual dr = rM - rC ---+
```

**Source:** Colin (2016) §8.3, §8.4 pp.127, 130 (printed 104, 107)

## See Also

- [`pa-fi-shift-twist-butterfly-and-krd.md`](pa-fi-shift-twist-butterfly-and-krd.md) — decomposes the `dy` term into yield-curve shift/twist/butterfly and key-rate durations.
- [`pa-fi-carry-rolldown-pulltopar-time-decomposition.md`](pa-fi-carry-rolldown-pulltopar-time-decomposition.md) — separates the deterministic time/carry return from roll-down and pull-to-par.
- [`pa-fi-parametric-vs-nonparametric-curve-models.md`](pa-fi-parametric-vs-nonparametric-curve-models.md) — supplies/interpolates the yield-curve inputs consumed by the duration-convexity attribution equation.
- [`pa-currency-attribution-karnosky-singer.md`](pa-currency-attribution-karnosky-singer.md) — adds the currency/local-rate layer (Van Breukelen builds on this on top of the duration approximation).

## Escalate to Raw When

- You need a fully worked numerical example: Colin (2016) Table 8.1 walks the perturbational equation across a 4.25% Treasury Gilt 2049, day by day, showing `rM`, `ydt`, `-MD*dy`, `rC`, and the residual `dr` — Colin (2016) §8.3 p.128 (printed 105).
- You must decide between risk-number attribution and first-principles re-pricing for a specific instrument (OTC swaps, inflation linkers, sinking bonds, bond futures) — Colin (2016) §8.3.1-§8.3.2 p.130 (printed 107).
- You need the arithmetic Van Breukelen weighted-duration form `rLi = xi + Di*(-delta_yi)` that grafts the duration approximation onto a Brinson-style top-down model — Bacon (2023) eq.(6.121) p.358 (printed 336).
