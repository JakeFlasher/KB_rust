---
schema_version: "cacg.v0"
id: "qm-regression-assumption-violations"
title: "Regression Assumption Violations"
reading_id: "reading_01_qm"
summary: "framing the three classical-assumption violations on a fitted regression — conditional heteroskedasticity, serial correlation of residuals, and multicollinearity among predictors — including the diagnostic statistic for each and the consequence for inferential standard errors and test statistics"
tags: ["definition", "regression", "regression-diagnostics"]
card_edges:
  - target: "qm-regression-hypothesis-tests"
    edge_type: "extends"
citations:
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p006:0004"
    chunk_hash: "4ed9fdb76ece800b7858c72ac5a20e57c71b4ec14093b7f3938aece82f48fda6"
    page_range: [6, 7]
    quote: "Compared to AIC, BIC assesses a greater penalty for adding independent variables"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p007:0005"
    chunk_hash: "24bb790fc6330d355391282cf0b01ffb8a0889eaa4fe8b42720fb37a9ccbd961"
    page_range: [7, 8]
    quote: "Heteroskedastiuty F1p ?Serial correlation 3 Mutticollinearity 32 x Homoskedastiaty a"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p008:0006"
    chunk_hash: "0365d99304e51f23e88d44f17e34fc068e8f23ecbc42c586e670f38e808a56af"
    page_range: [8, 9]
    quote: "Covrect H: p1 =0; H,: p1 ? 0.(- ? VIFj > 5, further investigation ?F-distributed with n-p-k-1 and p degrees of freedom,"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p009:0007"
    chunk_hash: "1a085a0ed614503ab4705f52128bf79000c5e957519c0fd68d822234b3ac758e"
    page_range: [9, 10]
    quote: "T-tests indicate no significance highly correlatedt-statistic?, when F-test indicates overall Multicollinearity Type II"
    edge_type: "supports"
card_hash: "00249145eb91076fa7eebd811f75c4ce00b32afc0f2b3e8dcb248747779f0bc0"
---
framing the three classical-assumption violations on a fitted regression — conditional heteroskedasticity, serial correlation of residuals, and multicollinearity among predictors — including the diagnostic statistic for each and the consequence for inferential standard errors and test statistics

## Original Card (preserved verbatim)

## Intuition

The classical OLS inference machinery — the t-tests on individual
slopes and the F-tests on joint significance — rests on three working
assumptions about the residual / predictor structure: the residuals
have constant variance across observations (homoskedasticity), they
are uncorrelated across observations (independence / no serial
correlation), and the predictors carry distinct identifying variation
(no perfect or near-perfect multicollinearity). When any of these
fails, the OLS point estimates may remain unbiased but the inferential
standard errors are distorted, which corrupts t-statistics, F-tests,
and the p-values they generate. **Source:**
notes/CFA_note_2.ocr.pdf pp.7-9.

Each violation has a named diagnostic statistic in the notes and a
characteristic effect on inference. Heteroskedasticity inflates
t-statistics (raises Type I error). Serial correlation similarly
distorts standard errors, typically deflating them when the
correlation is positive. Multicollinearity inflates the standard
errors of the affected coefficients, deflating their t-statistics
even though the joint fit remains adequate. The diagnostic-statistic
identity for each violation and its inferential consequence are the
two pieces the notes pair on `pp.7-9`. **Source:**
notes/CFA_note_2.ocr.pdf pp.7-9.

```
<!-- primitive: residual-vs-fitted-plot source: _diagram_primitives.md -->
    residual e_i
       ^
       |        homoskedastic band            heteroskedastic fan
       |       . .  .   .   . .   .              .       .
       |     .   . .  . .  .   . .              .    .   .
   0  -+----+-+-+-+-+-+-+-+-+-+-+-+----    -+--+-+-+-+-+--+-+--
       |    .  .  .   . . . .  . . .         .    .   . .
       |       .  .  .   .   . .              .       .
       |        (constant variance)              (fan widens)
       +--------------------------------> fitted ŷ_i
```

## Definition

For the fitted regression
`Y_i = b_0 + b_1·x_{1i} + ... + b_k·x_{ki} + ε_i`, the three
assumption violations and their diagnostic statistics are listed
below. **Source:** notes/CFA_note_2.ocr.pdf pp.7-9.

- **Conditional heteroskedasticity** occurs when the residual
  variance `Var(ε_i | X)` depends on the predictor values rather
  than being constant. The notes' Breusch–Pagan test regresses
  squared residuals on the original predictors and produces the test
  statistic `n · R²_resid`; a significant statistic is the notes'
  reject-homoskedasticity verdict. The specific reference distribution
  is outside the notes' span. **Source:** notes/CFA_note_2.ocr.pdf
  pp.7-9.

- **Serial correlation** occurs when residuals are correlated across
  observations (typical in time-series data). The notes' Durbin–Watson
  statistic tests for first-order serial correlation; the Breusch–
  Godfrey test is the notes' alternative for higher-order serial
  correlation. A significant test statistic is the notes' reject-
  independence verdict. **Source:** notes/CFA_note_2.ocr.pdf pp.7-9.

- **Multicollinearity** occurs when one predictor is close to a
  linear combination of the others; near-collinearity inflates the
  variance of the affected coefficients. The notes' variance-
  inflation factor `VIF_j = 1 / (1 − R²_j)`, where `R²_j` is the
  R² from regressing predictor `j` on the remaining predictors,
  quantifies the inflation; common rule-of-thumb thresholds are
  `VIF > 5` (worth investigating) and `VIF > 10` (severe). The
  perfect-collinearity limiting case and its OLS-singularity
  consequence are outside the notes' span and belong to a raw
  econometric reference. **Source:** notes/CFA_note_2.ocr.pdf pp.7-9.

## Mathematical Reasoning

The Breusch–Pagan statistic (source ASSERTS) is the notes'
heteroskedasticity diagnostic computed as `n · R²` of squared
residuals on the original predictors. The notes assert the test
statistic; the specific reference distribution and its
degrees-of-freedom are outside the notes' span and belong to a raw
econometric reference. **Source:** notes/CFA_note_2.ocr.pdf pp.7-9.

The Durbin–Watson statistic (source ASSERTS) is the notes'
first-order serial-correlation diagnostic; the notes assert it as
the named test alongside Breusch–Godfrey for higher-order serial
correlation. The specific algebraic form of the DW statistic and
the numerical interpretation of its values are outside the notes'
span and belong to a raw econometric reference. **Source:**
notes/CFA_note_2.ocr.pdf pp.7-9.

The variance-inflation-factor identity `VIF_j = 1 / (1 − R²_j)`
(source ASSERTS) is the notes' multicollinearity diagnostic, where
`R²_j` is the coefficient of determination from regressing predictor
`j` on the remaining predictors. The notes pair the formula with the
rule-of-thumb thresholds `VIF > 5` (investigate) and `VIF > 10`
(severe). The full limiting-case algebra (no multicollinearity ⇒
VIF = 1; perfect collinearity ⇒ VIF → ∞) is outside the notes' span
and belongs to a raw econometric reference. **Source:**
notes/CFA_note_2.ocr.pdf pp.7-9.

The inferential consequence (source ASSERTS) is that
heteroskedasticity inflates the apparent t-statistics by
under-estimating standard errors, biasing inference toward false
rejection of zero-coefficient nulls (Type I error). Serial
correlation has the same direction of effect when the correlation is
positive. Multicollinearity inflates standard errors of the affected
coefficients, deflating their t-statistics and biasing inference
toward false acceptance of zero-coefficient nulls (Type II error)
even when the joint fit remains adequate. The notes assert these
sign-of-effect rules without deriving them from the OLS sandwich-
variance algebra. **Source:** notes/CFA_note_2.ocr.pdf pp.7-9.

## See Also

- [`qm-multiple-linear-regression-foundations`](qm-multiple-linear-regression-foundations.md)
  — establishes the classical Gauss-Markov assumption set (linearity,
  homoskedasticity, independence, normality, no-perfect-multicollinearity)
  whose violations this card diagnoses
- [`qm-regression-hypothesis-tests`](qm-regression-hypothesis-tests.md)
  — describes the t-test and F-test machinery whose reference
  distributions assume the assumption set held; this card identifies
  the specific assumption-violation patterns that distort those
  reference distributions

## Escalate to Raw When

Open the underlying source or a more rigorous econometric reference
when any of the criteria below applies. **Source:**
notes/CFA_note_2.ocr.pdf pp.7-9.

- The analyst needs an estimator that corrects for the diagnosed
  violation — the notes on `notes/CFA_note_2.ocr.pdf pp.7-9` cover
  the diagnostic statistics and the inferential consequence; any
  variance-correction estimator beyond the diagnostic level is
  outside the notes' scope and belongs to a raw econometric
  reference. **Source:** notes/CFA_note_2.ocr.pdf pp.7-9.
- The serial correlation under inspection has known structure —
  the notes' tests on `notes/CFA_note_2.ocr.pdf pp.7-9` are
  diagnostic; specialised time-series treatment of autoregressive
  errors lives in
  [`qm-time-series-foundations`](qm-time-series-foundations.md), and
  any model-class extensions are outside the notes' scope. **Source:**
  notes/CFA_note_2.ocr.pdf pp.7-9.
- The diagnosed heteroskedasticity is time-conditional — the
  notes' Breusch–Pagan test on `notes/CFA_note_2.ocr.pdf pp.7-9`
  tests against the original predictors; time-conditional variance
  extension is the
  [`qm-arch-conditional-heteroskedasticity`](qm-arch-conditional-heteroskedasticity.md)
  sibling card's domain, and any further extensions are outside the
  notes' scope. **Source:** notes/CFA_note_2.ocr.pdf pp.7-9.
- The analyst observes per-observation outliers or high-leverage
  points distorting the fit — the notes' assumption-violation tests
  on `notes/CFA_note_2.ocr.pdf pp.7-9` are aggregate-residual
  diagnostics; per-observation influence analysis is the
  [`qm-influence-analysis-leverage`](qm-influence-analysis-leverage.md)
  sibling card's domain. **Source:** notes/CFA_note_2.ocr.pdf pp.7-9.
