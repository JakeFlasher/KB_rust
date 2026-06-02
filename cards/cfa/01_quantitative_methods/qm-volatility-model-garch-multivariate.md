---
schema_version: "cacg.v0"
id: "qm-volatility-model-garch-multivariate"
title: "Volatility-Model Estimation (ARCH / GARCH + Multivariate)"
reading_id: "01_quantitative_methods"
summary: "Tsay AFTS 2e Ch.3 defines ARCH(m) and GARCH(m, s) recursive conditional-variance models on a financial-return innovation series; GARCH(1, 1) with persistence α_1 + β_1 is the practitioner default. Tsay MTSA 2014 Ch.7 extends σ_t² to a multivariate volatility matrix Σ_t with BEKK and DCC parameterizations."
tags: ["quantitative-methods", "volatility-model"]
citations:
  - source_id: "qm_tsay_2005_afts_2e"
    chunk_id: "qm_tsay_2005_afts_2e:p127:0142"
    chunk_hash: "f7fc09f89e666cdc0981e3c823e732be2de414e9ade08c54a7b30c3da4b665d6"
    page_range: [127, 128]
    quote: "The basic idea of ARCH models is that (a) the shock at of an asset return is serially uncorrelated, but dependent"
    edge_type: "defines"
  - source_id: "qm_tsay_2005_afts_2e"
    chunk_id: "qm_tsay_2005_afts_2e:p138:0154"
    chunk_hash: "86c11c801431672d4d14c7b0be317a4790f67a2f4af6426d5d09b19045de68bc"
    page_range: [138, 139]
    quote: "Bollerslev (1986) proposes a useful extension known as the generalized ARCH (GARCH) model."
    edge_type: "defines"
  - source_id: "qm_tsay_2014_multivariate_time_series"
    chunk_id: "qm_tsay_2014_multivariate_time_series:p419:0479"
    chunk_hash: "4937f83e616ba7cfc60e9c9cee47cbe3d28f6494d872c9faed4119770b171ac8"
    page_range: [419, 420]
    quote: "Many multivariate volatility models have been proposed in the literature, includ"
    edge_type: "supports"
  - source_id: "qm_tsay_2014_multivariate_time_series"
    chunk_id: "qm_tsay_2014_multivariate_time_series:p421:0481"
    chunk_hash: "a9c648ba5ea6d0033864e7f1330408043b469eccec02c4c2e00ad371e24d7daf"
    page_range: [421, 421]
    quote: "We study in Section 7.5 the simple Baba–Engle–Kraft– Kroner (BEKK) model of Engle and Kroner (1995) and discuss its pros and cons."
    edge_type: "supports"
card_hash: "debe3c88ad20153b79128ace88e5b1a1ac450b136e88ee0de4528b8909a31503"
---
# Volatility-Model Estimation (ARCH / GARCH + Multivariate)

## Intuition

The CB-arb practitioner who wants to size vega exposure on a
delta-hedged convertible needs a forward-looking estimate of the
underlying-equity return volatility (see
[`cb-volatility-surface`](../08_convertible_bonds/cb-volatility-surface.md)
for the CB implied-vol-surface context and
[`cb-greeks-delta-gamma-vega`](../08_convertible_bonds/cb-greeks-delta-gamma-vega.md)
for the vega-attribution context). The univariate ARCH / GARCH
family is the canonical time-series toolkit: ARCH models the
conditional variance `σ_t²` as a linear combination of past squared
returns, and GARCH extends ARCH by adding an autoregressive lag in
the conditional variance itself, producing a parsimonious recursion
that captures the observed volatility clustering in financial-return
time-series. **Source:**
01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

Tsay 3e Ch.3 motivates the ARCH model on financial-return time-
series that exhibit volatility clustering: large absolute returns
tend to be followed by large absolute returns and small by small. The
canonical illustrative example in §3.4.4 is the Deutsche mark / U.S.
dollar 10-minute exchange-rate returns (Figure 3.2), where the
empirical autocorrelation of squared returns is significantly non-
zero at the first several lags while the autocorrelation of raw
returns is near-zero — the diagnostic signature for ARCH effects in
the residual series. The CB-arb adaptation treats the underlying-
equity return series as the input; the resulting `σ_t²` forecast
feeds the vega-input cross-link at
[`cb-volatility-surface`](../08_convertible_bonds/cb-volatility-surface.md).
**Source:**
01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

```
<!-- primitive: garch-volatility-clustering source: _diagram_primitives.md -->
   |ε_t|  (absolute residual magnitude)
       ^
       |         IID / constant variance (no clustering)
       |    . . . . . . . . . . . . . . . . . . . . . .
       |    . . . . . . . . . . . . . . . . . . . . . .
       +────────────────────────────────────────────────> t
       |
       |   ARCH/GARCH conditional-variance clustering
       |
       |              ■■■                ■■■■
       |              ■■■■              ■■■■■
       |   .  .  .   ■■■■■   .  .  .   ■■■■■■   .  .
       |   . .  . .  ■■■■■■   . . .   ■■■■■■■  . . .
       +────────────────────────────────────────────────> t
                    └─cluster─┘        └──cluster──┘

   E[ε_t² | F_{t−1}] = a_0 + a_1·ε_{t−1}² + ...  (ARCH/GARCH)
   Large shock at t−1 inflates conditional variance at t.
```

## Definition

An ARCH(m) model specifies the conditional variance of an innovation
series `ε_t` (typically the residuals from a conditional-mean model
such as an AR or ARMA fit) as `σ_t² = α_0 + α_1·ε_{t-1}² + α_2·
ε_{t-2}² + ... + α_m·ε_{t-m}²` with `α_0 > 0` and `α_i ≥ 0` for
`i = 1, ..., m` for non-negativity of `σ_t²`, plus the
weak-stationarity condition `Σ_{i=1}^{m} α_i < 1` for finite
unconditional variance. The CB-arb consumer of the ARCH output is
the vega-input cross-link at
[`cb-volatility-surface`](../08_convertible_bonds/cb-volatility-surface.md).
**Source:**
01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

A GARCH(m, s) model extends ARCH by adding autoregressive lags in
the conditional variance itself. The conditional-variance recursion
is given by the expression `σ_t² = α_0 + Σ α_i·ε_{t-i}² + Σ β_j·σ_{t-j}²`
where the first summation runs over the `m` ARCH lags and the second
runs over the `s` GARCH lags, subject to the usual non-negativity
constraints on `α_0`, the `α_i`, and the `β_j`, plus the sum-less-
than-unity stationarity condition. The canonical practitioner default
for univariate equity-vol modelling is the GARCH(1, 1) special case
because it fits most empirical clustering patterns with two slope
parameters and a constant, and its persistence statistic is the
half-life proxy that gates the CB-arb's vega-deployment horizon (the
practitioner choice of horizon is documented at
[`cb-greeks-delta-gamma-vega`](../08_convertible_bonds/cb-greeks-delta-gamma-vega.md)).
**Source:**
01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

The multivariate generalisation in Tsay's Multivariate Time Series
Analysis Ch.10 extends the univariate `σ_t²` recursion to a
conditional covariance matrix `Σ_t` of a vector return series. The
BEKK and DCC (Dynamic Conditional Correlation) parameterisations are
the practitioner-standard parameterisations: BEKK writes `Σ_t = C·Cᵀ
+ A·ε_{t-1}·ε_{t-1}ᵀ·Aᵀ + B·Σ_{t-1}·Bᵀ` with positive-definiteness
guaranteed by construction; DCC separates volatility (univariate
GARCH per asset) from correlation (a separate scalar DCC recursion
on standardized residuals). The CB-arb application is cross-issuer
gamma attribution when the CB book holds positions in multiple
convertibles whose underlying-equity correlations matter (see
[`cb-greeks-delta-gamma-vega`](../08_convertible_bonds/cb-greeks-delta-gamma-vega.md)).
**Source:**
01_Quantitative_Methods/Multivariate Time Series Analysis (Ruey S. Tsay) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.359-410.

## Mathematical Reasoning

The ARCH(1) conditional-variance recursion (source ASSERTS) is `σ_t² = α_0 + α_1·ε_{t-1}²` where `α_0 > 0` and `α_1 ≥ 0`. The unconditional variance is `Var(ε_t) = α_0 / (1 − α_1)` (finite iff `α_1 < 1`); the kurtosis of `ε_t` exceeds 3 when `3·α_1² < 1`, which is the algebraic source of the fat-tailed marginal distribution that ARCH models reproduce. Tsay 3e Ch.3 §3.4 establishes both results without proof beyond the algebraic manipulation of conditional moments. **Source:** 01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

The GARCH(1, 1) recursion (source ASSERTS) is `σ_t² = α_0 + α_1·ε_{t-1}² + β_1·σ_{t-1}²` with `α_0 > 0`, `α_1, β_1 ≥ 0`, and `α_1 + β_1 < 1` for weak stationarity. Tsay §3.5 establishes the recursion form and the stationarity condition without further proof beyond the algebraic manipulation of conditional moments. **Source:** 01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

The unconditional variance under the GARCH(1, 1) stationarity assumption is the practitioner-canonical `Var(ε_t) = α_0 / (1 − α_1 − β_1)`. The persistence parameter is the sum `α_1 + β_1`, which controls the half-life of a volatility shock; a value close to unity indicates that volatility shocks decay slowly. The boundary case where `α_1 + β_1` equals unity is the so-called IGARCH limit, which has infinite unconditional variance and a unit-root in conditional variance — the practitioner-folkloric interpretation that is consistent with the algebraic forms. **Source:** 01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

The maximum-likelihood estimator (source ASSERTS) for the GARCH parameter vector `θ = (α_0, α_1, β_1)` maximises the conditional Gaussian log-likelihood `ℓ(θ) = − (1/2) · Σ_t [ln(σ_t²(θ)) + ε_t² / σ_t²(θ)]` where `σ_t²(θ)` is computed by the GARCH(1, 1) recursion. Quasi-maximum-likelihood estimation (QMLE) substitutes the Gaussian density for the true error distribution and remains consistent and asymptotically normal under mild regularity conditions; the sandwich-form standard errors are the robust-to-misspecification covariance estimator. **Source:** 01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

The multivariate DCC parameterisation (source ASSERTS) decomposes the time-varying conditional covariance as `Σ_t = D_t · R_t · D_t` where `D_t = diag(σ_{1,t}, ..., σ_{n,t})` is the diagonal matrix of univariate GARCH-fitted volatilities and `R_t` is the time-varying correlation matrix evolved by a scalar DCC recursion on the standardised residuals `z_{i,t} = ε_{i,t} / σ_{i,t}`. The estimation proceeds in two stages: first fit univariate GARCH per asset; then estimate the scalar DCC parameters from the standardised residuals. Tsay Multivariate Ch.10 establishes the two-stage estimator and its quasi-likelihood form without proof beyond the conditional-Gaussian algebra. **Source:** 01_Quantitative_Methods/Multivariate Time Series Analysis (Ruey S. Tsay) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.359-410.

## See Also

- [`qm-cb-arb-factor-construction`](qm-cb-arb-factor-construction.md) — the sibling factor-construction card whose ridge / lasso / boosting machinery sits upstream of the vol-forecasting step here; the vol forecast feeds into the factor-model regression's residual variance assumption
- [`cb-volatility-surface`](../08_convertible_bonds/cb-volatility-surface.md) — the CB implied-vol-surface card that consumes the GARCH univariate `σ_t²` output as the historical-vol benchmark for the CB underlying-equity vol input to vega
- [`cb-greeks-delta-gamma-vega`](../08_convertible_bonds/cb-greeks-delta-gamma-vega.md) — the CB Greeks card that consumes the multivariate `Σ_t` cross-asset covariance estimate for cross-issuer gamma attribution when the CB book holds multiple convertibles whose underlying-equity correlations matter

## Escalate to Raw When

Open Tsay 3e directly or the more rigorous multivariate-vol
references when any of the criteria below applies. **Source:**
01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.

- The vol-model needs stochastic-volatility depth beyond GARCH
  (latent-volatility-process modelling, particle-filter estimation)
  — Tsay 3e Ch.3 §3.12 introduces the stochastic-volatility model
  at intuition depth only; deeper state-space machinery (Tsay
  Ch.11+) is out of scope per the v7+ CB-arb extension boundary
  discipline (see frontmatter `Out of scope:`). **Source:**
  01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.
- The vol-model needs jump-diffusion or realized-vol high-frequency
  estimation — out of scope; consult the relevant raw references
  if needed. **Source:**
  01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.
- The multivariate vol-model needs BEKK identifiability conditions
  or copula-vol depth beyond Ch.10 DCC — out of scope under the v7+
  CB-arb extension policy. **Source:**
  01_Quantitative_Methods/Multivariate Time Series Analysis (Ruey S. Tsay) (z-library.sk, 1lib.sk, z-lib.sk).pdf pp.359-410.
- The CB-arb vol input needs implied-vol surface construction
  rather than historical-vol estimation — route to the
  [`cb-volatility-surface`](../08_convertible_bonds/cb-volatility-surface.md)
  card. **Source:**
  01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.
- The CB-arb vega computation needs the option-pricing surface
  (Black-Scholes / Black implied-vol mapping) — route to the
  [`cb-greeks-delta-gamma-vega`](../08_convertible_bonds/cb-greeks-delta-gamma-vega.md)
  card. **Source:**
  01_Quantitative_Methods/Analysis of Financial Time Series.pdf pp.97-200.
