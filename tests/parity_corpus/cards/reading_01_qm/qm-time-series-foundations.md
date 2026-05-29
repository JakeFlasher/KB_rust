---
schema_version: "cacg.v0"
id: "qm-time-series-foundations"
title: "Time-Series Foundations: Trend, AR(p), Stationarity, Unit Root"
reading_id: "reading_01_qm"
summary: "framing the time-series foundations the CFA notes cover — deterministic trend models (linear and log-linear), the autoregressive AR(p) recursion, the covariance-stationarity definition that gates OLS estimation of AR models, and the Dickey–Fuller test that flags a unit-root non-stationarity requiring first-differencing"
tags: ["definition", "time-series"]
citations:
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p010:0008"
    chunk_hash: "d5032da0221c72c91dd5dbc858a2d4d875fd2f6c170f91c4f4d83143d35631e1"
    page_range: [10, 11]
    quote: "B - Probabilioy 3i3l vs Probabilioy chreshol Dummy variables can also be used as dependent variables in Fit of logit"
    edge_type: "supports"
  - source_id: "qm_notes_trim"
    chunk_id: "qm_notes_trim:p011:0009"
    chunk_hash: "d481200f411572ed8e983862e53f79d8e945e5878d9c51558d2aaa45a7e06b72"
    page_range: [11, 12]
    quote: "Exhibit 2: Autocorrelations of the Residual from AR(1) Model Constant and finite covariance with itself for a fixed"
    edge_type: "supports"
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
card_hash: "0a47d737ac3d4a498103f181ad70c9d1ae50a93883561b56734589091a6500db"
---
framing the time-series foundations the CFA notes cover — deterministic trend models (linear and log-linear), the autoregressive AR(p) recursion, the covariance-stationarity definition that gates OLS estimation of AR models, and the Dickey–Fuller test that flags a unit-root non-stationarity requiring first-differencing

## Original Card (preserved verbatim)

## Intuition

The cross-sectional OLS apparatus assumes observations are
exchangeable rows. Time-series observations are not — observation `t`
is naturally close to observation `t − 1` in clock-order, and that
serial dependence has to enter the model directly through a lag
structure rather than being modelled as a noise nuisance. The two
basic time-series models the notes cover are (a) a deterministic
trend in time, where `t` itself is the predictor, and (b) the
autoregressive AR(p) model, where the predictors are the variable's
own past values. **Source:** notes/CFA_note_2.ocr.pdf pp.11-12.

The covariance-stationarity assumption is the gating condition that
makes the AR(p) OLS estimator valid: the series' mean, variance,
and autocovariance-at-each-lag must be constant in time. When the
AR slope on the most recent lag equals `1`, the series has a unit
root, has no finite mean-reverting level, and OLS estimation is
invalid. The Dickey–Fuller test rewrites the AR(1) regression to
test the unit-root null directly; failing to reject it means the
series must be first-differenced before any AR estimation. **Source:**
notes/CFA_note_2.ocr.pdf pp.11-12.

```
<!-- primitive: autocorrelation-decay source: _diagram_primitives.md -->
   ρ_k (autocorrelation at lag k)
       ^
   1.0 +■
       |■
       | ■                      stable AR(1):  ρ_k = b_1^k,
       |  ■                                    |b_1| < 1
       |   ■
       |    ■                   unit root:     ρ_k ≈ 1 for all k
       |  ●  ■                  (no decay)
   0.5 +  ●   ■■■
       |   ●     ■■■■
       |    ●        ■■■■■■
       |  ●  ●           ■■■■■■■■■■■■■  ←  AR(1) decay envelope
       |  ●  ●  ●  ●  ●  ●  ●  ●  ●  ●  ←  unit-root persistence
   0.0 +-------------------------------------> lag k
       0   1   2   3   4   5   6   7   8
```

## Definition

The notes' time-series models on `pp.11-12` are listed below.
**Source:** notes/CFA_note_2.ocr.pdf pp.11-12.

- **Linear trend model**: `y_t = b_0 + b_1 · t + ε_t`, where `t` is
  the time index. Fitted by OLS treating `t` as the single
  predictor; the slope `b_1` is the per-period change in the
  expected value of `y_t`. **Source:** notes/CFA_note_2.ocr.pdf
  pp.11-12.

- **Log-linear trend model**: `ln(y_t) = b_0 + b_1 · t + ε_t`. Fitted
  by OLS on the log-transformed series; the slope `b_1` is the
  per-period continuous-compounded growth rate of `y_t`. **Source:**
  notes/CFA_note_2.ocr.pdf pp.11-12.

- **Autoregressive model AR(p)**:
  `x_t = b_0 + b_1 · x_{t-1} + b_2 · x_{t-2} + ... + b_p · x_{t-p} + ε_t`.
  The predictors are the variable's own `p` most recent lags;
  fitted by OLS treating the lagged values as predictors. **Source:**
  notes/CFA_note_2.ocr.pdf pp.11-12.

- **Covariance stationarity** holds for a series `{x_t}` when (a)
  `E[x_t]` is the same constant for every `t`, (b) `Var(x_t)` is
  the same finite constant for every `t`, and (c) the autocovariance
  `Cov(x_t, x_{t-k})` depends only on the lag `k`, not on the
  reference time `t`. Covariance stationarity is the assumption that
  validates OLS estimation of the AR(p) coefficients. **Source:**
  notes/CFA_note_2.ocr.pdf pp.11-12.

- **Unit root and Dickey–Fuller test**: when the AR(1) slope `b_1`
  equals `1`, the series is a random walk and has a unit root.
  The Dickey–Fuller test rearranges `x_t = b_0 + b_1 · x_{t-1} + ε_t`
  as `x_t − x_{t-1} = b_0 + g_1 · x_{t-1} + ε_t` (where
  `g_1 = b_1 − 1`) and tests `H_0: g_1 = 0` against `H_A: g_1 < 0`.
  Failing to reject the null is the unit-root verdict; the series
  must be first-differenced before AR estimation can proceed.
  **Source:** notes/CFA_note_2.ocr.pdf pp.11-12.

## Mathematical Reasoning

The trend-model OLS estimators (source ASSERTS) come directly from
applying the cross-sectional OLS apparatus with `t` as the single
predictor; the slope coefficient `b̂_1` in the linear trend is the
per-period change in `E[y_t]`, and the slope coefficient `b̂_1` in
the log-linear trend is the per-period continuous-compounded growth
rate of `y_t` because `d(ln y_t)/dt = b_1` ⇔ `y_t = y_0 · e^{b_1·t}`
on the deterministic part. The notes assert both interpretations
without proving the OLS sampling properties in the time-series
setting. **Source:** notes/CFA_note_2.ocr.pdf pp.11-12.

The AR(p) recursion (source ASSERTS) treats the lagged values
`x_{t-1}, ..., x_{t-p}` as predictors and fits the coefficients
`b_0, b_1, ..., b_p` by OLS on the `(n − p)` observation rows that
have a complete lag set. The notes assert that covariance
stationarity is the gating condition under which OLS estimation of
the AR(p) coefficients is valid; the underlying large-sample
justification is outside the notes' scope. **Source:**
notes/CFA_note_2.ocr.pdf pp.11-12.

The autocorrelation-decay property (source ASSERTS) is that under a
stable AR(1) with `|b_1| < 1`, the lag-`k` autocorrelation is
`ρ_k = b_1^k`, which decays geometrically to zero as `k` grows.
Under a unit root (`b_1 = 1`), the lag-`k` autocorrelation does not
decay and the series is non-stationary. The notes assert the
geometric-decay rule and the unit-root contrast. **Source:**
notes/CFA_note_2.ocr.pdf pp.11-12.

The Dickey–Fuller rearrangement (source DECOMPOSES) re-expresses
the AR(1) regression as
`Δx_t = b_0 + g_1 · x_{t-1} + ε_t` with `g_1 = b_1 − 1`. The null
`H_0: g_1 = 0` is algebraically equivalent to `H_0: b_1 = 1` (the
unit-root null), and the notes assert that the test uses revised
critical values appropriate to the unit-root setting rather than the
standard t-critical values. Failing to reject the null implies the
unit-root verdict; the first-differenced series `Δx_t` is then the
candidate for AR estimation. The notes assert the rearrangement and
the revised-critical-value rule. **Source:**
notes/CFA_note_2.ocr.pdf pp.11-12.

## See Also

- [`qm-multiple-linear-regression-foundations`](qm-multiple-linear-regression-foundations.md)
  — provides the cross-sectional OLS estimator and Gauss-Markov
  assumption set that the AR(p) and trend models specialise to the
  time-series setting; covariance stationarity in this card is the
  time-series analogue of the cross-sectional i.i.d. assumption
- [`qm-regression-assumption-violations`](qm-regression-assumption-violations.md)
  — covers serial-correlation diagnostics (Durbin–Watson and
  Breusch–Godfrey) that detect serial correlation in residuals of
  any regression, including the AR-model residuals after the
  recursive lag structure has been fitted

## Escalate to Raw When

Open the underlying source or a more rigorous econometric reference
when any of the criteria below applies. **Source:**
notes/CFA_note_2.ocr.pdf pp.11-12.

- The series exhibits time-varying conditional variance — the notes'
  AR(p) and trend models on `notes/CFA_note_2.ocr.pdf pp.11-12`
  treat the residual variance as constant in time; the conditional-
  variance extension is the
  [`qm-arch-conditional-heteroskedasticity`](qm-arch-conditional-heteroskedasticity.md)
  sibling card's domain. **Source:** notes/CFA_note_2.ocr.pdf
  pp.11-12.
- The series shows seasonal or higher-frequency periodic patterns —
  the notes' trend and AR(p) models on `notes/CFA_note_2.ocr.pdf
  pp.11-12` do not encode seasonal terms; any seasonal-decomposition
  machinery is outside the notes' scope and belongs to a raw
  time-series reference. **Source:** notes/CFA_note_2.ocr.pdf
  pp.11-12.
- The analyst needs an integrated moving-average structure — the
  notes on `notes/CFA_note_2.ocr.pdf pp.11-12` cover the AR (and
  differencing) components only; the MA (and full ARIMA) layer is
  outside the notes' scope and belongs to a raw time-series
  reference. **Source:** notes/CFA_note_2.ocr.pdf pp.11-12.
- The analysis is multivariate — the notes' AR(p) on
  `notes/CFA_note_2.ocr.pdf pp.11-12` is single-series; vector-
  autoregression, Granger-causality, and cointegration analysis are
  outside the notes' scope and belong to a raw time-series
  reference. **Source:** notes/CFA_note_2.ocr.pdf pp.11-12.
