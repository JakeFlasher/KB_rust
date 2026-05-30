---
schema_version: "cacg.v0"
id: "pa-fi-shift-twist-butterfly-and-krd"
title: "Shift, Twist, Butterfly, and Key-Rate Duration"
reading_id: "15_performance_and_attribution"
summary: "Nested four-curve repricing splits the sovereign-curve return into parallel shift, twist, and curvature (butterfly); the key-rate-duration partition is an alternative split that coincides exactly only via first-principles repricing."
tags: ["fixed-income-attribution", "yield-curve", "key-rate-duration"]
citations:
  - source_id: "pa_colin_2016"
    chunk_id: "pa_colin_2016:p153:0161"
    chunk_hash: "72ea8cc751b3bdd9c4705b21f4053114bb8f5ea8e13f6ae1f116ef355a976877"
    page_range: [153, 154]
    quote: "Curvature movement is sometimes referred to as butterfly, and attribution analysis that includes these three effects may be labelled shift-twist-butterfly"
    edge_type: "defines"
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p353:0423"
    chunk_hash: "056b90509a3a64a5edc5db2f9420094ec9f3212b99bccbf6f16c38edefa90118"
    page_range: [353, 354]
    quote: "The changing shape of the yield curve can be decomposed into three different movements: shift, twist (or slope) and curvature (or butterfly)."
    edge_type: "supports"
card_hash: "a5d0a955ef8a315aa4ea1c7180d5b33f14f3a5b78793492d7429ea7b27701a34"
---
# Shift, Twist, Butterfly, and Key-Rate Duration

## Intuition

A sovereign yield curve almost never moves rigidly. The dominant move is a parallel slide of the whole curve (shift), but the curve also pivots — short and long ends move by different amounts (twist), and the middle can bow relative to the wings (curvature, or "butterfly"). Shift-twist-butterfly (STB) attribution decomposes a bond's risk-free return by repricing the security on a *nested* sequence of curves, each adding one more movement type, so the sub-returns sum exactly to the total curve return. Key-rate duration (KRD) is an alternative partition of that same curve return: instead of asking "how parallel/sloped/bowed was the move," it asks "how sensitive is this bond to a wiggle at each maturity point," organising the return by maturity location. The two partitions coincide *exactly* only when KRD is built from first principles (successively repricing the bond on curves modified maturity-by-maturity until the end curve is rebuilt); the perturbational KRD form is a first-order approximation, not an identity.

**Source:** Colin (2016) §10.5.4–10.5.8 pp.153-156

## Definition

**Shift, twist, and curvature.** Bacon defines the family directly: "The changing shape of the yield curve can be decomposed into three different movements: shift, twist (or slope) and curvature (or butterfly)." Shift is a parallel move at all maturities; twist (slope) is a steepening or flattening pivot; curvature (butterfly) is a relative move of the middle versus the wings. Colin notes the labelling convention: "Curvature movement is sometimes referred to as butterfly, and attribution analysis that includes these three effects may be labelled shift-twist-butterfly attribution. The origin of the term is obscure."

**Nested four-curve construction.** An STB model builds four curves: (1) the start curve; (2) start plus parallel shift; (3) start plus shift plus twist; (4) start plus shift plus twist plus higher-order (= the actual end curve). The security is priced on each, giving prices p0, p1, p2, p3, and the three sub-returns are the successive price differences scaled by p0.

**Key-rate duration (KRD).** KRD "ignores global curve movements, and instead calculates the effect of changes in the yield curve at predefined maturity points." It is the analogue of modified duration but localised: modified duration measures sensitivity to a parallel shift, KRD_i measures sensitivity to a move at key maturity i. KRD is the natural representation for amortising securities (e.g. mortgage-backed bonds) whose cash flows span many maturities.

**Source:** Colin (2016) §10.5.4, §10.5.6 pp.153-155; Bacon (2023) §6 pp.354-355

## Mathematical Reasoning

STB return attribution is a telescoping decomposition. With four nested curves and prices p0..p3,

```
rparallel  = (p1 - p0) / p0      # repricing on start curve + parallel shift
rtwist     = (p2 - p1) / p0      # add twist (slope) move
rcurvature = (p3 - p2) / p0      # add residual higher-order (butterfly)
-----------------------------------------------------------------
rtotal     = (p3 - p0) / p0  =  rparallel + rtwist + rcurvature
```

The three sub-returns telescope to the total curve return because the numerators sum to (p3 - p0) and every denominator is the common base p0; no cross terms or residual arise by construction. The yield-change identity is parallel: at any maturity the actual yield change equals shift + twist + curvature, where curvature is *defined residually* as (actual change) - (shift) - (twist), exactly as twist is defined as (shift+twist change) - (shift). This is a definitional partition, not an approximation.

The KRD representation re-organises the curve return as a sum over maturities, and it comes in two flavours that must not be conflated. **First-principles KRD** is the exact analogue of the STB telescope: the bond is repriced on the start curve, then on curves successively modified so each reference maturity in turn takes its end-curve level, until "the intermediate curve is identical to the end curve." The sub-returns from those repricings telescope to "the overall return for the security over the interval" — exact, by the same construction logic that makes STB exact. **Perturbational KRD** instead uses Colin's first-order sensitivity identity

```
r_riskfree-curve ~= - sum_{i=1..n} KRD_i * dy_i
```

where n is the number of key rates, KRD_i is the key-rate duration at maturity i, and dy_i is the change in the zero curve at maturity i. The minus sign carries the usual price-yield inversion (yields up, price down). This form is an *approximation* (linear in the yield changes, ignoring convexity and cross-maturity terms), not an identity. So STB and KRD are alternative *partitions* of the same curve return — one organised by movement *type* (shift/twist/butterfly), one by movement *location* (each key maturity) — but they coincide *exactly* only under the first-principles repricing construction; the perturbational KRD partition only approximately matches.

The two views are reconcilable but not identical in their building blocks: Colin labels the source's own caveat that PCA-derived shift/twist/curvature "are typically slightly different from more conventional interpretations of these terms" — the shift eigenfunction is close to but not exactly a parallel shift, and the twist is not uniform across maturities. The card asserts this divergence; the source asserts it without a closed-form mapping, so no such mapping is claimed here.

**Source:** Colin (2016) §10.5.3–10.5.5, §10.5.7–10.5.8 Eq.(10.5) pp.152-156

```
 STB: organise by MOVEMENT TYPE        KRD: organise by MOVEMENT LOCATION
  (nested curves, telescoping)           (per-maturity sensitivities)

  yield                                   yield
   |   _ _ _  shift (parallel)             |        x   <- bump at key rate i
   |  /                                    |       / \
   |_/_______ + twist (pivot/slope)        |  ____/   \____
   |/    \                                 |  | | | | | | |
   |      \__ + curvature (butterfly)      |  m1 m2 ... mn  key maturities
   |________________ maturity              |________________ maturity
   p0 -> p1 -> p2 -> p3                     r ~= -sum_i KRD_i * dy_i
   (sub-returns sum to rtotal,              (perturbational: approx; only
    exact telescope)                         first-principles KRD is exact)
```

**Source:** Colin (2016) §10.5.4, §10.5.8 pp.153-156

## Boundary Notes

STB and KRD both decompose only the *risk-free / sovereign-curve* component of bond return. Carry (coupon, roll-down, pull-to-par) and credit-spread return are separate effects layered on top; see the carry and perturbational-equation siblings. Bacon places shift/twist/curvature inside the broader "yield curve" branch alongside carry, credit, selection, and currency. Colin also flags that there is "no standard market approach" to measuring twist and "no standard, agreed way to calculate the parallel shift" — the decomposition is exact *given* a chosen shift/twist definition, but the choice itself is a modelling decision.

**Source:** Colin (2016) §10.3.1, §10.4 pp.147-151; Bacon (2023) §6 pp.356-357

## See Also

- [`pa-fi-perturbational-attribution-equation.md`](pa-fi-perturbational-attribution-equation.md) — the perturbational analytics framing where KRD * dy sensitivities live alongside spread duration.
- [`pa-fi-carry-rolldown-pulltopar-time-decomposition.md`](pa-fi-carry-rolldown-pulltopar-time-decomposition.md) — the carry/time effects that sit beside the curve-movement effects in the full bond decomposition.
- [`pa-fi-parametric-vs-nonparametric-curve-models.md`](pa-fi-parametric-vs-nonparametric-curve-models.md) — PCA and Nelson-Siegel alternatives to the nested STB construction.

Cross-vertical: the KRD sensitivity vector is the same key-rate-duration object used for interest-rate risk in fixed-income risk management (rm-* / fixed-income readings), repurposed here as an attribution partition rather than a hedging measure.

## Escalate to Raw When

- You need the worked numeric STB example (USD curve over Q4 2008, Option 1/2/3 parallel-shift figures, the 5-year shift = -1.59% / twist = +0.17% / curvature = -0.20% table) — Colin (2016) §10.3.1–10.5.4 pp.149-153.
- You need the trapezoidal area-under-curve formula for parallel shift, Eq.(10.3)–(10.4) — Colin (2016) §10.3.1 pp.148-149; or the exact first-principles KRD curve-modification algorithm step by step — Colin (2016) §10.5.7 pp.155-156.
- You need the full fixed-income attribution effects tree (carry / yield curve / selection / other branches) or the Wagner-Tito Fama-type duration decomposition — Bacon (2023) §6 Figures 6.14–6.15 pp.356-357.
- You need to reconcile commercial-system STB output against a PCA decomposition when twist is small but curvature is large — Colin (2016) §10.5.5 pp.154-155.
