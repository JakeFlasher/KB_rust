---
schema_version: "cacg.v0"
id: "pa-variability-ratios-sharpe-information"
title: "Variability Ratios: Sharpe and the Information Ratio"
reading_id: "15_performance_and_attribution"
summary: "Family card for the Sharpe-form reward/variability template: ratio = excess return over a dispersion measure, read as the gradient on a return-risk plane. The Information Ratio is its benchmark-relative twin (excess over tracking error), with MAD, Gini, and skew/kurtosis-adjusted denominator variants."
tags: ["sharpe-ratio", "information-ratio", "risk-adjusted-return"]
citations:
  - source_id: "pa_bacon_2022_rapm"
    chunk_id: "pa_bacon_2022_rapm:p070:0062"
    chunk_hash: "9a129eb89b37b02953790fac0964c9918cf2eda23ee7e2f33bca7c552837460e"
    page_range: [71, 71]
    quote: "The Sharpe ratio can be described as the return (or reward) per unit of variability (or risk)."
    edge_type: "defines"
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p113:0112"
    chunk_hash: "d9d5c235736326428a93b3287db7ea173427e1c0dfd5e8ffd77f5407ef5aff9c"
    page_range: [114, 114]
    quote: "The appraisal ratio, more commonly called the information ratio, is the ratio of alpha to residual risk."
    edge_type: "supports"
card_hash: "9b7d5285be003f51d364d4ba041eb7d524fb257d912d285d28784c1a81e52410"
---
# Variability Ratios: Sharpe and the Information Ratio

## Intuition

Return alone never settles which portfolio is better: a given return earned at low
risk dominates the same return earned at high risk. The Sharpe family resolves this by
dividing a reward (return net of a baseline) by a measure of how much the return
varied to earn it. Plot reward on the vertical axis and variability on the
horizontal axis, anchor a ray at the risk-free rate, and the slope of the ray to a
portfolio's point *is* the Sharpe ratio. The steeper the gradient, the further into
the prized top-left (high reward, low risk) corner the portfolio sits. Everything
else in this family is the same gradient idea with a different choice of "reward
baseline" or a different "variability" yardstick in the denominator.

**Source:** Bacon (2022) §"Sharpe Ratio" pp.70-71

## Definition

The **Sharpe ratio** is reward per unit of variability: annualised portfolio
return minus the annualised risk-free rate, divided by annualised return
variability (standard deviation). The greater the ratio, the steeper the gradient
and the better the risk/return combination.

The **Information Ratio** is the benchmark-relative Sharpe ratio: it swaps absolute
return for *excess* return over the benchmark, and swaps absolute risk for
*tracking error* (the standard deviation of excess return, also called relative or
active risk). Because excess returns are already benchmark-relative, no risk-free
rate appears and the ray is anchored at the origin — the benchmark itself is the
natural starting point. Bacon notes the Information Ratio is directly related to
the revised Sharpe ratio, simply replacing the risk-free rate with the benchmark.

In a market-model framing, the same statistic is the **appraisal ratio**: alpha
divided by residual risk, taken as a measure of security-selection skill.

**Variability-substitution variants** keep the reward numerator and replace the
denominator:
- **MAD ratio** — mean absolute deviation instead of standard deviation; less
  sensitive to extreme returns.
- **Gini ratio** — mean difference (Gini) instead of standard deviation; better
  suited to non-normal distributions.
- **Adjusted / skew-adjusted Sharpe (and Information) ratios** — apply a penalty
  factor for negative skewness and excess kurtosis, on the view that any analytic
  ignoring higher moments implicitly assumes normality.

**Sources:** Bacon (2022) §"Sharpe Ratio"/"Information Ratio"/"MAD Ratio"/"Gini
Ratio" pp.71-86; Christopherson, Cariño & Ferson (2009) §"Appraisal Ratio and
Information Ratio" pp.106-116

## Mathematical Reasoning

Let `r` be the annualised portfolio return, `r_F` the annualised risk-free rate,
and `sigma` the annualised return standard deviation. The defining template is a
slope:

```
                  reward
   ratio  =  ----------------
              variability
```

- **Sharpe:**            `SR  = (r - r_F) / sigma`
- **Information (IR):**  `IR  = a / sigma_A`   where `a` = annualised excess return
  over benchmark and `sigma_A` = annualised tracking error (std. dev. of excess
  return).

The two are structurally identical: IR is obtained from the Sharpe form by the
substitutions `r_F -> benchmark` and `sigma -> sigma_A`. Hence the Sharpe ray originates at
`r_F` on the vertical axis, while the IR ray originates at the origin (the
benchmark baseline). Sign reasoning carries over directly: a positive IR signals
outperformance, a negative IR underperformance.

Variability substitution preserves the numerator and swaps the denominator:

```
   denominator choice  ->  ratio name
   ---------------------------------
   sigma (std. deviation) ->  Sharpe
   MAD (mean abs. dev.)   ->  MAD ratio
   MD  (Gini mean diff.)  ->  Gini ratio
```

The book *asserts* these properties (less sensitivity to extremes for MAD, better
behaviour under non-normality for Gini) without supplying a formal proof; the card
asserts them at the same level and labels the gap. Likewise the adjusted-Sharpe
penalty is presented as a suggested adjustment, not a derived optimum.

A sign caveat the book flags explicitly: when the numerator is negative, the ratio
"rewards" higher variability (a more variable losing portfolio scores a less
negative ratio). Bacon treats negative ratios as still meaningful for ranking
losses per unit of risk; modified variants exist that re-impose a tracking-error
penalty under negative excess return.

**Source:** Bacon (2022) §3 Eq.(3.1), Eq.(3.13)-(3.14), Eq.(3.21) pp.71-86

## Boundary Notes

This is the **variability-ratio (second-moment) family**. Ratios whose denominator
is a *regression* risk (systematic beta or residual risk) — Treynor and Jensen —
live in the appraisal-regression card; ratios that use a *downside* dispersion
(target semideviation) — Sortino, Omega, Kappa — live in the partial-moment card;
ratios built on *drawdowns* (Calmar, Sterling, Ulcer) and the `M^2` return-space
transform of the Sharpe ratio are their own cards. The choice between arithmetic
and geometric excess return in the IR numerator is a definitional fork covered in
its own card. The Sharpe ratio is not generally used to rank individual securities,
since it ignores cross-correlation.

**Source:** Bacon (2022) §3 pp.70-86; Christopherson, Cariño & Ferson (2009) §10
pp.106-116

## See Also

- [`pa-regression-appraisal-jensen-treynor.md`](pa-regression-appraisal-jensen-treynor.md) — sibling family using regression (beta / residual) risk rather than total variability; the appraisal ratio bridges the two.
- [`pa-partial-moment-ratios-sortino-omega-kappa.md`](pa-partial-moment-ratios-sortino-omega-kappa.md) — same template with downside (semideviation) dispersion in the denominator.
- [`pa-m2-risk-adjusted-return-transform.md`](pa-m2-risk-adjusted-return-transform.md) — the Sharpe ratio re-expressed in return units, preserving Sharpe rankings.
- [`pa-active-risk-tracking-error-ex-ante-vs-ex-post.md`](pa-active-risk-tracking-error-ex-ante-vs-ex-post.md) — the tracking-error denominator that turns Sharpe into the Information Ratio. The Sharpe/Information Ratio pair also anchors the active-management ratios in pm-* portfolio-management cards and the GIPS risk-disclosure expectations under 17 ethics.

## Escalate to Raw When

- You need the worked Sharpe / alternative / revised / skew-adjusted / adjusted
  numeric examples (Bacon Exhibits 3.1-3.5, pp.78-95) — these are explicitly
  out of scope here per the no-worked-arithmetic rule.
- You need the exact algebra of the adjusted-Sharpe and adjusted-Information-ratio
  skew/kurtosis penalty factors (Eq. 3.5-3.6, 3.27-3.30, pp.75-92) or the
  alternative/smoothing/skewness-kurtosis variants.
- You must reconcile the appraisal-ratio (alpha / residual risk) definition with
  the excess-return / tracking-error definition for a beta != 1.0 portfolio — see
  Christopherson, Cariño & Ferson (2009) pp.103-115.
- You need benchmark-quality, data-frequency, or ex-ante-vs-ex-post calculation
  conventions before reporting a live Information Ratio.
