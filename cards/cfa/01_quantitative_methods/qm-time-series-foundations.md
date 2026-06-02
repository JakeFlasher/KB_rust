---
schema_version: "cacg.v0"
id: "qm-time-series-foundations"
title: "Time-Series Foundations: Trend, AR(p), Stationarity, Unit Root"
reading_id: "01_quantitative_methods"
summary: "AR(p) recursion, covariance stationarity, the Dickey–Fuller unit-root test, and log-linear trend models are NOT covered in CFA L1 2022. Reading 7 only distinguishes cross-sectional vs time-series regression data; AR(p) / unit-root / Dickey–Fuller machinery is CFA L2 / raw time-series-textbook content."
tags: ["quantitative-methods", "time-series"]
citations:
  - source_id: "qm_tsay_2005_afts_2e"
    chunk_id: "qm_tsay_2005_afts_2e:p049:0044"
    chunk_hash: "eec1cca92206e6fee29cc29ba8562f8616110331120205e41aaa1967b2654374"
    page_range: [49, 50]
    quote: "From the definitions, if rt is strictly stationary and its first two moments are finite, then rt is also weakly stationary."
    edge_type: "supports"
  - source_id: "qm_tsay_2005_afts_2e"
    chunk_id: "qm_tsay_2005_afts_2e:p093:0100"
    chunk_hash: "2d8577e77959793a7dd6bc18844d65ff92b1eb6c21a645e6811d8f22d9a8f2d7"
    page_range: [93, 93]
    quote: "This is the well-known unit-root testing problem; see Dickey and Fuller (1979)."
    edge_type: "supports"
card_hash: "b1d16bdc37d4d25f59a688cb29ae74b9b1aaf0c90841937eebf6805c8e42e6c0"
---
# Time-Series Foundations: Trend, AR(p), Stationarity, Unit Root

## Intuition

The cross-sectional OLS apparatus assumes observations are
exchangeable rows. Time-series observations are not — observation `t`
is naturally close to observation `t − 1` in clock-order, and that
serial dependence has to enter the model directly through a lag
structure rather than being modelled as a noise nuisance. The two
basic time-series models the source covers are (a) a deterministic
trend in time, where `t` itself is the predictor, and (b) the
autoregressive AR(p) model, where the predictors are the variable's
own past values. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The covariance-stationarity assumption is the gating condition that
makes the AR(p) OLS estimator valid: the series' mean, variance,
and autocovariance-at-each-lag must be constant in time. When the
AR slope on the most recent lag equals `1`, the series has a unit
root, has no finite mean-reverting level, and OLS estimation is
invalid. The Dickey–Fuller test rewrites the AR(1) regression to
test the unit-root null directly; failing to reject it means the
series must be first-differenced before any AR estimation. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

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

The source' time-series models on `pp.11-12` are listed below.
**Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Linear trend model**: `y_t = b_0 + b_1 · t + ε_t`, where `t` is
  the time index. Fitted by OLS treating `t` as the single
  predictor; the slope `b_1` is the per-period change in the
  expected value of `y_t`. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Log-linear trend model**: `ln(y_t) = b_0 + b_1 · t + ε_t`. Fitted
  by OLS on the log-transformed series; the slope `b_1` is the
  per-period continuous-compounded growth rate of `y_t`. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Autoregressive model AR(p)**:
  `x_t = b_0 + b_1 · x_{t-1} + b_2 · x_{t-2} + ... + b_p · x_{t-p} + ε_t`.
  The predictors are the variable's own `p` most recent lags;
  fitted by OLS treating the lagged values as predictors. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Covariance stationarity** holds for a series `{x_t}` when (a)
  `E[x_t]` is the same constant for every `t`, (b) `Var(x_t)` is
  the same finite constant for every `t`, and (c) the autocovariance
  `Cov(x_t, x_{t-k})` depends only on the lag `k`, not on the
  reference time `t`. Covariance stationarity is the assumption that
  validates OLS estimation of the AR(p) coefficients. **Source:**
  CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- **Unit root and Dickey–Fuller test**: when the AR(1) slope `b_1`
  equals `1`, the series is a random walk and has a unit root.
  The Dickey–Fuller test rearranges `x_t = b_0 + b_1 · x_{t-1} + ε_t`
  as `x_t − x_{t-1} = b_0 + g_1 · x_{t-1} + ε_t` (where
  `g_1 = b_1 − 1`) and tests `H_0: g_1 = 0` against `H_A: g_1 < 0`.
  Failing to reject the null is the unit-root verdict; the series
  must be first-differenced before AR estimation can proceed.
  **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

## Mathematical Reasoning

The trend-model OLS estimators (source ASSERTS) come directly from
applying the cross-sectional OLS apparatus with `t` as the single
predictor; the slope coefficient `b̂_1` in the linear trend is the
per-period change in `E[y_t]`, and the slope coefficient `b̂_1` in
the log-linear trend is the per-period continuous-compounded growth
rate of `y_t` because `d(ln y_t)/dt = b_1` ⇔ `y_t = y_0 · e^{b_1·t}`
on the deterministic part. The source asserts both interpretations
without proving the OLS sampling properties in the time-series
setting. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The AR(p) recursion (source ASSERTS) treats the lagged values
`x_{t-1}, ..., x_{t-p}` as predictors and fits the coefficients
`b_0, b_1, ..., b_p` by OLS on the `(n − p)` observation rows that
have a complete lag set. The source asserts that covariance
stationarity is the gating condition under which OLS estimation of
the AR(p) coefficients is valid; the underlying large-sample
justification is outside the source' scope. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The autocorrelation-decay property (source ASSERTS) is that under a
stable AR(1) with `|b_1| < 1`, the lag-`k` autocorrelation is
`ρ_k = b_1^k`, which decays geometrically to zero as `k` grows.
Under a unit root (`b_1 = 1`), the lag-`k` autocorrelation does not
decay and the series is non-stationary. The source asserts the
geometric-decay rule and the unit-root contrast. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

The Dickey–Fuller rearrangement (source DECOMPOSES) re-expresses
the AR(1) regression as
`Δx_t = b_0 + g_1 · x_{t-1} + ε_t` with `g_1 = b_1 − 1`. The null
`H_0: g_1 = 0` is algebraically equivalent to `H_0: b_1 = 1` (the
unit-root null), and the source asserts that the test uses revised
critical values appropriate to the unit-root setting rather than the
standard t-critical values. Failing to reject the null implies the
unit-root verdict; the first-differenced series `Δx_t` is then the
candidate for AR estimation. The source asserts the rearrangement and
the revised-critical-value rule. **Source:**
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

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
CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.

- The series exhibits time-varying conditional variance — the source'
  AR(p) and trend models on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360`
  treat the residual variance as constant in time; the conditional-
  variance extension is the
  [`qm-arch-conditional-heteroskedasticity`](qm-arch-conditional-heteroskedasticity.md)
  sibling card's domain. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The series shows seasonal or higher-frequency periodic patterns —
  the source' trend and AR(p) models on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` do not encode seasonal terms; any seasonal-decomposition
  machinery is outside the source' scope and belongs to a raw
  time-series reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analyst needs an integrated moving-average structure — the
  notes on `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` cover the AR (and
  differencing) components only; the MA (and full ARIMA) layer is
  outside the source' scope and belongs to a raw time-series
  reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
- The analysis is multivariate — the source' AR(p) on
  `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360` is single-series; vector-
  autoregression, Granger-causality, and cointegration analysis are
  outside the source' scope and belong to a raw time-series
  reference. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.1/pp.300-360.
