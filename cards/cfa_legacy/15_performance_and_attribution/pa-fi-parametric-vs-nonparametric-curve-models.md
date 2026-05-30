---
schema_version: "cacg.v0"
id: "pa-fi-parametric-vs-nonparametric-curve-models"
title: "Parametric vs Non-Parametric Yield-Curve Models"
reading_id: "15_performance_and_attribution"
summary: "Two ways to read a yield curve for attribution: the parametric approach (Nelson-Siegel/polynomial, a few fitted parameters) versus the non-parametric spline approach (piecewise-polynomial interpolation on raw data); each trades smoothness against fidelity to observed points."
tags: ["fixed-income-attribution", "yield-curve-models", "nelson-siegel"]
citations:
  - source_id: "pa_colin_2016"
    chunk_id: "pa_colin_2016:p296:0307"
    chunk_hash: "bd90123728a79f7787c7656174aa0c0cabeefb26aa97758feb7d9c4af039a088"
    page_range: [297, 297]
    quote: "■ Virtually any curve observed in the marketplace can be fitted to a Nelson-Siegel function, using least-squares or similar techniques."
    edge_type: "defines"
---
# Parametric vs Non-Parametric Yield-Curve Models

## Intuition

Fixed-income attribution needs a yield at *every* maturity, but the market only quotes a handful of points. Something has to fill the gaps between them, and there are two philosophies. The **parametric** philosophy says the curve has a recognisable global shape — sloped, humped, inverted — so impose a single smooth mathematical form with a few knobs and let least-squares find the best fit; the whole curve then compresses to four numbers. The **non-parametric / spline** philosophy refuses to commit to a global shape: it stitches local low-order polynomials between adjacent observed points, so the curve passes through (or very near) every quote and adapts locally. The trade-off is the classic smoothness-versus-fidelity tension: the parametric form is parsimonious and globally smooth but a few fixed parameters cannot reproduce every wiggle; the spline interpolates the raw data closely with local polynomials, and Colin notes that refinements are needed to keep such a curve globally smooth.

**Source:** Colin (2016) §D.1, §D.2 pp.297

In practice Colin notes that both approaches are encountered, yet the most widely used method for a yield at an arbitrary maturity is simpler still — plain linear interpolation between existing yield points. Parametric and spline modelling are the two formal alternatives when linear interpolation is not good enough.

**Source:** Colin (2016) §10.2 pp.146-147

## Definition

**Parametric curve modelling.** The form of the yield curve is *assumed* to follow a particular pattern described by a mathematical function with a small number of parameters; those parameters are then supplied in place of the raw curve data. The best-known example is **Nelson-Siegel (1987)**, which writes the yield at maturity `t` in terms of four parameters `b0, b1, b2, lambda`. It matches a wide range of curve behaviours (sloped, flat, curved, inverted, humped), and virtually any observed curve can be fitted to it by least-squares or similar techniques. The parameters carry direct economic meaning: `b0` is the asymptotic (long) yield, `b0 + b1` is the short (spot) rate, and `b1` (the short-minus-long spread) is read as the slope of the curve.

**Source:** Colin (2016) §D.1 eq.(D.1) pp.297

**Non-parametric / spline modelling.** A spline model typically fits a first- or second-order polynomial between successive curve data points, with many refinements (such as enforcing global smoothness) layered on top. No global functional form is imposed; the model is non-parametric in the sense that its degrees of freedom grow with the number of data points rather than being fixed at a small count.

**Source:** Colin (2016) §D.2 pp.297

## Mathematical Reasoning

The two model families differ in how their degrees of freedom scale and therefore in what they can and cannot reproduce. Let the curve be observed at `n` maturity points.

```
                  PARAMETRIC                      NON-PARAMETRIC / SPLINE
                  (Nelson-Siegel)                 (piecewise polynomial)
  form            one global function y(t)        local polynomial per interval
  free params     fixed (4: b0,b1,b2,lambda)      grows with n  (~ O(n))
  fit to quotes   approximate (least-squares)     near-exact (passes through pts)
  shape control   global, smooth by construction  local; refinements added to
                                                   keep the curve globally smooth
  econ. meaning   b0=long yield, b0+b1=short,      none intrinsic; purely
                  b1=slope                         interpolative
  refinements     a single fitted function         "many refinements ... such as
                                                   ensuring the curve is globally
                                                   smooth"
```

**Source:** Colin (2016) §D.1, §D.2 pp.297

In the parametric case the curve is a single function defined by four fixed parameters, fitted with "least-squares or similar techniques"; because the parameter count does not grow with `n`, the fitted function need not pass exactly through every quote. In the spline case Colin describes a first- or second-order polynomial fitted between successive data points, so the degrees of freedom rise with the number of points and the construction is local rather than global; Colin notes only that "many refinements" are layered on "such as ensuring the curve is globally smooth," without specifying which constraints achieve this. Colin states these structural properties descriptively — Nelson-Siegel's four-parameter form and the spline's piecewise-polynomial construction — and does not derive fitting-error bounds, an exact-interpolation result, or a formal noise-sensitivity analysis; this card asserts the smoothness-versus-fidelity trade-off at the same descriptive level and labels the absence of any formal error analysis as a gap.

**Source:** Colin (2016) §D.1 eq.(D.1), §D.2 pp.297

## See Also

- [`pa-fi-perturbational-attribution-equation.md`](pa-fi-perturbational-attribution-equation.md) — consumes the modelled `y(t)` and its change `dy` as the raw input to the duration/convexity return decomposition.
- [`pa-fi-shift-twist-butterfly-and-krd.md`](pa-fi-shift-twist-butterfly-and-krd.md) — the Nelson-Siegel slope/curvature parameters are an alternative way to measure the twist that this card's shift/twist/butterfly split also captures.
- [`pa-fi-carry-rolldown-pulltopar-time-decomposition.md`](pa-fi-carry-rolldown-pulltopar-time-decomposition.md) — roll-down needs a yield at a shifted maturity, which is exactly what the interpolated/fitted curve supplies.

## Escalate to Raw When

- You need the explicit Nelson-Siegel functional form (the full eq.(D.1) with the `(1 - e^(-t/lambda))/(t/lambda)` loading terms) to implement a fit, rather than the parameter interpretation summarised here — Colin (2016) §D.1 eq.(D.1) pp.297.
- You must choose spline order or the global-smoothness refinements for a specific curve, or compare against the default linear interpolation — Colin (2016) §D.2 pp.297 and §10.2 pp.146-147.
- You need a worked least-squares fit of an observed curve to Nelson-Siegel, or a worked spline interpolation showing the residual-at-quotes versus exact-interpolation contrast quantitatively.
