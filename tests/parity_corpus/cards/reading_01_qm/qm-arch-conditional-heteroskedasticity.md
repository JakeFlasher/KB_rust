---
schema_version: "cacg.v0"
id: "qm-arch-conditional-heteroskedasticity"
title: "ARCH(1) Conditional Heteroskedasticity"
reading_id: "reading_01_qm"
summary: "framing the ARCH(1) conditional-heteroskedasticity recursion for a time-series residual process — the regression of squared residuals on their own lag that defines ARCH, the significance test on the lag coefficient, and the consequence for risk forecasting and OLS-versus-GLS estimator choice when ARCH is present"
tags: ["definition", "garch"]
card_edges:
  - target: "qm-regression-assumption-violations"
    edge_type: "extends"
  - target: "qm-time-series-foundations"
    edge_type: "extends"
citations:
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p012:0010"
    chunk_hash: "d93b5d3b209eb7b86bc8afd86aec0f445c58e0b9cfecc1be584d88dee4cddb6a"
    page_range: [12, 13]
    quote: "Model seasonal lag e.g.xt+1=bo+b,xt+b2xt-314-G21 ? Unit Root 3i2+& yt=b1yt-1+et 3b1=1,B131ztle,non-stationary * a"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p012:0011"
    chunk_hash: "9c81607bf05735e43b9ca4e6ae6ecf08cc3815aa18293cc5b112f9b2ea3e548a"
    page_range: [12, 13]
    quote: "Firstdifferencing -PTTE3?=1uNt vo0t = 73 5373= 2unitVoot aconditionalARCH Root? Unit Linear or Exponential Serial"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p013:0012"
    chunk_hash: "611d81c11337c94f68e768154d35f4fd0bd7ce048d91d747cbb4b5521edd04ef"
    page_range: [13, 14]
    quote: "Step 2: test whether the error term from the regression has a unit root using a Dickey?Fuller test"
    edge_type: "supports"
card_hash: "1b278c517fd3117c6092d0746f211934edf11dbddc2cd8721150de92e69015fb"
---
framing the ARCH(1) conditional-heteroskedasticity recursion for a time-series residual process — the regression of squared residuals on their own lag that defines ARCH, the significance test on the lag coefficient, and the consequence for risk forecasting and OLS-versus-GLS estimator choice when ARCH is present

## Original Card (preserved verbatim)

## Intuition

The aggregate-residual heteroskedasticity test in the sibling
[`qm-regression-assumption-violations`](qm-regression-assumption-violations.md)
card detects whether residual variance depends on the original
PREDICTORS. The time-series counterpart asks a different question:
whether the residual variance at time `t` depends on what the
residual variance was at time `t − 1`. When it does, the residual
process has "volatility clustering" — periods of large squared
residuals tend to cluster together, and periods of small squared
residuals tend to cluster together. **Source:**
notes/CFA_note_2.ocr.pdf pp.13.

ARCH (Autoregressive Conditional Heteroskedasticity) is the simplest
parametric model of that clustering: it regresses the squared
residual at time `t` on the squared residual at time `t − 1`. The
slope coefficient on the lag, if significantly different from zero,
is the diagnostic that conditional heteroskedasticity is present.
When it is, the unconditional variance is no longer the correct
input to a risk forecast at time `t`; the conditional variance
`σ_t²` is, and the OLS estimator that assumes constant residual
variance is inefficient — generalised least squares (GLS) becomes
the appropriate estimator. **Source:** notes/CFA_note_2.ocr.pdf
pp.13.

```
   ε_t² (squared residual at time t)
       ^
       |                                       .
       |                                  .
       |                              .
       |                          .
       |                      .              σ_t² conditional-variance
       |                  .                  line under ARCH(1):
       |              .                      σ_t² = a_0 + a_1 · ε_{t-1}²
       |          .
       |       .                              slope a_1  >  0
       |    .                                 ⇒ large ε_{t-1}² predicts
       | a_0                                    large ε_t² ("volatility
       | .                                      clustering")
       |
       +-------------------------------------> ε_{t-1}² (squared residual
                                               at the previous period)

   under conditional homoskedasticity (a_1 = 0):  σ_t² ≡ a_0  (flat)
   under ARCH (a_1 > 0):                          σ_t² rises with ε_{t-1}²
```

## Definition

For a time-series regression residual process `{ε_t}` from a fitted
linear regression (the time-series foundations card's AR or trend
model providing the residual stream), the **ARCH(1) model** is the
auxiliary regression
`ε_t² = a_0 + a_1 · ε_{t-1}² + u_t`,
where `a_0` is the intercept, `a_1` is the slope on the lagged
squared residual, and `u_t` is the auxiliary-regression error. The
ARCH(1) coefficient `a_1` is fitted by OLS on the squared residual
series. **Source:** notes/CFA_note_2.ocr.pdf pp.13.

The **ARCH detection rule** is to test `H_0: a_1 = 0` against
`H_A: a_1 ≠ 0` using the standard t-statistic on the OLS estimate
`â_1`. Rejecting the null is the verdict that conditional
heteroskedasticity is present in the residual process. **Source:**
notes/CFA_note_2.ocr.pdf pp.13.

The **conditional-variance forecast** under ARCH(1) is
`σ_t² = a_0 + a_1 · ε_{t-1}²`, which is time-varying (it depends
on the most recent squared residual rather than being a constant).
The notes assert that under conditional heteroskedasticity the
analyst's risk forecast at time `t` should use the conditional
variance `σ_t²` rather than the unconditional variance. **Source:**
notes/CFA_note_2.ocr.pdf pp.13.

The **estimator switch** when ARCH is present is from OLS to
generalised least squares (GLS); the notes assert this substitution
as the appropriate estimator response under conditional
heteroskedasticity without specifying the GLS weighting scheme or
the standard-error correction details. **Source:**
notes/CFA_note_2.ocr.pdf pp.13.

## Mathematical Reasoning

The ARCH(1) recursion (source ASSERTS) `ε_t² = a_0 + a_1 · ε_{t-1}² + u_t`
is the notes' auxiliary regression for the second moment of the
residual process. The notes assert the recursion form; the
interpretation of `a_1 ≠ 0` as significant lag-dependence is the
content of the detection test below. The reduction `a_1 = 0 →
constant variance` and the deeper estimator-foundations question
(maximum-likelihood vs OLS-on-squared-residuals) are outside the
notes' span and belong to a raw time-series-volatility reference.
**Source:** notes/CFA_note_2.ocr.pdf pp.13.

The detection test (source ASSERTS) is the notes' significance test
on `â_1`: a significant coefficient on the lagged squared residual
is the conditional-heteroskedasticity verdict. The notes pair this
test with Breusch–Pagan as the predictor-conditioned analogue in
the sibling assumption-violations card; ARCH is the time-conditional
version. The specific reference distribution / asymptotic argument
for `â_1` in the squared-residual auxiliary regression is outside
the notes' span. **Source:** notes/CFA_note_2.ocr.pdf pp.13.

The conditional-variance forecasting input (source ASSERTS) is the
notes' guidance that under conditional heteroskedasticity the
time-`t` risk projection uses the conditional variance
`σ_t² = a_0 + a_1 · ε_{t-1}²` — a time-varying quantity that depends
on the most recent squared residual — rather than the unconditional
variance. The notes state this as a forecasting-input choice; the
algebraic relationship between conditional and unconditional
variance is outside the notes' span. **Source:**
notes/CFA_note_2.ocr.pdf pp.13.

The OLS-to-GLS switch (source ASSERTS) is the notes' prescription
that generalised least squares becomes the appropriate estimator
when ARCH conditional heteroskedasticity is present. The notes
assert the substitution at the estimator-name level; the detailed
weighting scheme and standard-error correction are outside the
notes' span and belong to a raw econometric reference. **Source:**
notes/CFA_note_2.ocr.pdf pp.13.

## See Also

- [`qm-time-series-foundations`](qm-time-series-foundations.md) —
  provides the AR(p) and trend models whose fitted residuals
  `{ε_t}` are the inputs to this card's ARCH(1) auxiliary
  regression
- [`qm-regression-assumption-violations`](qm-regression-assumption-violations.md)
  — covers the Breusch–Pagan predictor-conditioned heteroskedasticity
  test, of which this card's ARCH(1) detection is the
  time-conditional analogue

## Escalate to Raw When

Open the underlying source or a more rigorous econometric reference
when any of the criteria below applies. **Source:**
notes/CFA_note_2.ocr.pdf pp.13.

- The residual process exhibits more complex volatility memory than
  ARCH(1) captures — the notes on `notes/CFA_note_2.ocr.pdf pp.13`
  cover the single-lag ARCH(1) form only; multi-lag and lagged-
  variance extensions are outside the notes' scope and belong to a
  raw time-series-volatility reference. **Source:**
  notes/CFA_note_2.ocr.pdf pp.13.
- The volatility response to positive and negative residual shocks
  is asymmetric — the notes' ARCH(1) on `notes/CFA_note_2.ocr.pdf
  pp.13` is symmetric in the sign of `ε_{t-1}`; asymmetric-volatility
  extensions are outside the notes' scope and belong to a raw
  time-series-volatility reference. **Source:**
  notes/CFA_note_2.ocr.pdf pp.13.
- The analyst needs full likelihood-based estimation of the ARCH
  parameters — the notes on `notes/CFA_note_2.ocr.pdf pp.13` describe
  the OLS regression on squared residuals as the detection device;
  full maximum-likelihood ARCH-family estimation is outside the
  notes' scope and belongs to a raw time-series-volatility reference.
  **Source:** notes/CFA_note_2.ocr.pdf pp.13.
- The analysis requires multivariate ARCH (cross-asset volatility
  spillover) — the notes' ARCH on `notes/CFA_note_2.ocr.pdf pp.13`
  is single-series; multivariate-ARCH extensions are outside the
  notes' scope and belong to a raw time-series-volatility reference.
  **Source:** notes/CFA_note_2.ocr.pdf pp.13.
