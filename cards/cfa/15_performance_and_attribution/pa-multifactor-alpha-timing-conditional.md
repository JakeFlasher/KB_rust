---
schema_version: "cacg.v0"
id: "pa-multifactor-alpha-timing-conditional"
title: "Multifactor Alpha, Market Timing, and Conditional Models"
reading_id: "15_performance_and_attribution"
summary: "Time-series regression of fund excess return on factor excess returns yields the intercept alpha (Jensen 1-factor, Carhart 4-factor, Fung-Hsieh 7-factor); quadratic/option timing terms split timing from selectivity, conditional betas vary with public information, and Roll (1978) shows benchmark choice makes alpha ambiguous."
tags: ["jensen-alpha", "market-timing", "conditional-models"]
citations:
  - source_id: "pa_fischer_wermers_2013"
    chunk_id: "pa_fischer_wermers_2013:p068:0088"
    chunk_hash: "35af0af8a4a2793bd9388fa547738bd53e53410262ed0ac17a634b7206f29e4d"
    page_range: [68, 69]
    quote: "alpha, usually called the Jensen alpha (since Jensen introduced this measure in his 1968 paper)."
    edge_type: "defines"
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p135:0135"
    chunk_hash: "2b13c6f56fc58e60764ca6eb37a733e14283a3d8d167b74f1d2a09af7ce99c7e"
    page_range: [135, 136]
    quote: "Using a single coefficient a p in Equation (12.3) presumes that expected abnormal performance is constant over time."
    edge_type: "supports"
card_hash: "1aa9970645aef2787734ab48e0999213e3dbe9eeda105e25cbd9c0eea038f0fa"
---
# Multifactor Alpha, Market Timing, and Conditional Models

## Intuition

Returns-based performance evaluation asks one question: after we strip out
everything the manager earned merely by bearing known, priced risks, is there
anything left over? That leftover is alpha. The mechanics are a time-series
regression of the fund's excess return (net of the riskfree rate) on the excess
return of one or more benchmark/factor portfolios. Whatever the factors cannot
explain piles up in the regression intercept. With a single market factor the
intercept is the Jensen alpha; add size, value, and momentum and it becomes the
Carhart four-factor alpha; layer on bond, currency, and trend-following factors
and it becomes the Fung-Hsieh seven-factor alpha for hedge funds. More factors
soak up more "explainable" return, so the surviving alpha is a cleaner estimate
of genuine skill — the four-factor model, for instance, pushes a passive index
fund's spurious one-factor alpha back down toward its expense ratio.

**Source:** Fischer & Wermers (2013) §3.5.1-3.5.2 pp.68-76 (print 63-71)

Two complications break the simple intercept story. First, a manager who *times*
the market — raising beta before up-markets and cutting it before down-markets —
generates a return pattern that is curved, not linear; a straight-line regression
mis-fits the curve and biases both alpha and beta. Second, a manager who merely
reads the newspaper and adjusts beta to widely-known economic conditions has no
real stock-picking skill, yet a static-beta regression credits this naive timing
as alpha. Timing models (Treynor-Mazuy, Henriksson-Merton) and conditional models
(Ferson-Schadt) exist to peel these effects apart from true selectivity.

**Source:** Fischer & Wermers (2013) §3.5.3-3.5.4 pp.77-79 (print 72-74)

## Definition

**Single-factor (Jensen) alpha.** Regress fund excess return on benchmark excess
return; the intercept is the alpha "usually called the Jensen alpha." The slope
is beta and the Treynor ratio rescales the expected excess return by that beta.

**Carhart four-factor alpha.** The equity workhorse adds size (SMB), value (HML),
and one-year momentum (UMD) to the market excess return (RMRF); the intercept is
the four-factor or "Carhart alpha." Fixed-income and mixed funds extend the factor
set with TERM, DEFAULT, FIRF, and OPTION factors. Hedge-fund multi-strategy funds
use the Fung-Hsieh seven-factor model, whose factors include three dynamic
trend-following benchmarks rather than passive indexes.

**Timing-vs-selectivity.** The Treynor-Mazuy model adds a squared market term
`[RMRF]^2`; the Henriksson-Merton model adds an option-payoff term `max(0, RMRF)`.
In each, the intercept measures selectivity and the gamma coefficient measures
timing. Henriksson-Merton's timing term is equivalent to the payoff of an
at-the-money call (a bundle of "free puts") on the excess market return.

**Conditional models.** Ferson-Schadt let beta be a linear function of lagged,
public market-information variables `z`, so the regression gains interaction terms
`z*RMRF`. Christopherson-Ferson-Glassman further let alpha be a function of `z`.
Conditional performance evaluation (CPE) thus separates skill at exploiting public
macro information from genuine private selectivity.

**Benchmark ambiguity (Roll 1978).** Because beta — and therefore alpha — depends
on the correlation between the fund and the chosen benchmark, reweighting the
benchmark (e.g., value-weighted vs. equal-weighted) can dramatically change, even
reverse, the estimated alpha. An improper benchmark credits or penalizes a manager
for benchmark-error returns rather than skill.

**Source:** Fischer & Wermers (2013) §3.5 pp.68-79 (print 63-74)

Ferson and Schadt "find that this covariance" — between the conditional beta and
the conditional expected market return — "is a major source of measurement error
in unconditional alphas of mutual funds"; controlling for it via the interaction
terms produces more reliable alpha estimates.

**Source:** Christopherson, Cariño & Ferson (2009) Ch.12 pp.135-137 (print 122-124)

## Mathematical Reasoning

The single-factor model is the identity

```
R_p,t - R_F,t  =  alpha_p  +  beta_p (R_B,t - R_F,t)  +  epsilon_p,t
```

so `alpha_p` is, by construction, the part of the fund's mean excess return not
proportional to the benchmark excess return. The Carhart generalization replaces
the single regressor with a factor vector:

```
r_t = alpha + beta*RMRF_t + s*SMB_t + h*HML_t + u*UMD_t + epsilon_t
```

and the Fung-Hsieh hedge-fund model extends to seven factors (market, size, two
bond factors, and three trend-following factors). In every case alpha is the
intercept; adding regressors mechanically reduces the in-sample residual
variance, but whether the resulting alpha is a *cleaner* skill estimate depends
on the benchmark's and factors' validity (Roll's critique) — a fund whose
"alpha" shrinks toward zero (or toward its fee) as economically valid factors
are added had no skill the cheaper factors did not already supply.

**Source:** Fischer & Wermers (2013) §3.5.1-3.5.2 pp.68-76 (print 63-71)

Timing introduces curvature. Treynor-Mazuy:

```
r_i,t = alpha^TM + b^TM*RMRF_t + gamma^TM*[RMRF_t]^2 + epsilon_i,t
```

Henriksson-Merton, with the up-market indicator `[RMRF_t]^+ = max(0, RMRF_t)`:

```
r_i,t = alpha^MH + b^MH*RMRF_t + gamma^MH*[RMRF_t]^+ + epsilon_i,t
```

In Treynor-Mazuy the timing contribution is `gamma^TM*Var[RMRF]`; in Henriksson-Merton
it is `gamma^MH*C[RMRF,T]`, where `C[RMRF,T]` is the value of an at-the-money call on
the excess market return over the evaluation horizon. A static-beta model that
omits the timing term biases the alpha — usually downward — when the manager is a
genuine timer, because the curvature is forced into a straight line.

```
       Manager excess return
            ^
            |              .  A  (high-beta choice, up-market)
            |          . /
   alpha >--|---------/----  (timing curvature: convex up)
            |     . /
            |   B (low-beta choice, down-market)
            |  /
            +------------------------>  benchmark excess return
           C
   Straight-line fit through A,B,C distorts intercept (alpha) and slope (beta).
```

**Source:** Fischer & Wermers (2013) §3.5.3 pp.77-78 (print 72-73)

Conditional models replace constant coefficients with functions of lagged public
information `z_t`. Ferson-Schadt time-varying beta and the resulting regression:

```
beta_p(z_t) = b_0p + B_p'*z_t
r_p,t+1  = alpha_p + b_0p*r_b,t+1 + B_p'*[z_t*r_b,t+1] + mu_p,t+1
```

The interaction products `z_t*r_b,t+1` capture the covariance between the
conditional beta and the conditional expected market return — the quantity the
supporting source identifies as the dominant bias in unconditional alpha.
Christopherson-Ferson-Glassman add a conditional alpha `alpha_p(z_t) = a_0p + A_p'*z_t`,
so total measured skill is `a_0p + A_p'*z_t`: a baseline `a_0p` plus a
business-cycle-varying increment. Roll's (1978) ambiguity result is the comparative
static behind all of this — because `beta_p` tracks the fund-benchmark correlation,
switching benchmarks shifts `beta_p` and hence `alpha_p`; the source derives the
difference `alpha^equalwt - alpha^valuewt` directly in terms of the two betas and the
benchmarks' average excess returns. The book asserts the timing-bias and
CPE-improvement results from the cited empirical literature rather than proving
them from first principles, and this card asserts them at the same level.

**Source:** Fischer & Wermers (2013) §3.5.1, §3.5.4 pp.69, 78-79 (print 64, 73-74); Christopherson, Cariño & Ferson (2009) Ch.12-13 pp.135-145 (print 122-132)

## Boundary Notes

Non-timing nonlinearities (derivatives in the portfolio, stale prices, dynamic
trading at finer-than-measured frequency) can masquerade as timing skill and
mis-specify both alpha and beta; the supporting source flags Scholes-Williams
lagged-beta corrections and CPE as partial remedies. Conditional alpha being
*higher* does not guarantee superior realized returns — it is a better
*predictor* out of sample, not a certainty.

**Source:** Christopherson, Cariño & Ferson (2009) Ch.13 pp.139-148 (print 126-135)

## See Also

- [`pa-regression-appraisal-jensen-treynor.md`](pa-regression-appraisal-jensen-treynor.md) — the single-factor Jensen/Treynor appraisal that this card generalizes to multifactor and timing settings.
- [`pa-factor-model-types-and-covariance-decomposition.md`](pa-factor-model-types-and-covariance-decomposition.md) — taxonomy of the factor models (fundamental/statistical/macro) whose excess returns are the regressors here.
- [`pa-luck-vs-skill-fdr-and-bootstrap.md`](pa-luck-vs-skill-fdr-and-bootstrap.md) — how to test whether an estimated multifactor alpha reflects genuine skill rather than sampling luck.
- [`pa-valid-benchmark-properties.md`](pa-valid-benchmark-properties.md) — the benchmark-quality criteria that Roll's (1978) ambiguity result makes load-bearing for any alpha estimate.

Cross-vertically, this connects to the active-management decomposition in the
pm-* portfolio-management cards (alpha as the active-return signal), to factor
risk in the rm-* risk cards (the same factor exposures drive ex-ante tracking
error), and to GIPS performance-presentation discipline in the 17 ethics
material (how net-of-fee alpha must be reported).

## Escalate to Raw When

- You need the actual estimated alphas, betas, gammas, and R-squared values from
  the worked Treynor-Mazuy / Henriksson-Merton example tables (the source's
  Tables 13.1-13.2 give the numeric fits and "true vs. estimated" comparisons).
- You need the precise factor-construction recipes (SMB/HML breakpoints, the
  Fung-Hsieh trend-following primitive-strategy definitions, FIRF/OPTION factor
  details) to replicate a regression rather than reason about it.
- You need the full conditional-model worked example with specific information
  variables (dividend yield, detrended T-bill) and the explicit interaction-term
  regression coefficients, or the worked bull/bear bias numbers (e.g., the spurious
  7.5% alpha, 0.625 beta) illustrating the unconditional-model error.
- You need Roll's full ambiguity-paper derivation or the exact `alpha^equalwt -
  alpha^valuewt` bias-table sign analysis with realized benchmark return inputs.
