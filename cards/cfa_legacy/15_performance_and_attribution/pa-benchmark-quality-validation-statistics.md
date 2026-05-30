---
schema_version: "cacg.v0"
id: "pa-benchmark-quality-validation-statistics"
title: "Benchmark-Quality Validation Statistics"
reading_id: "15_performance_and_attribution"
summary: "Statistical tests confirming a candidate benchmark fits a manager or style universe: regress manager returns on the benchmark and check beta toward 1, standard error toward 0, R-squared toward 1, plus a quartile/box-chart median-proximity test."
tags: ["benchmark-validation", "characteristic-line", "style-analysis"]
citations:
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p321:0323"
    chunk_hash: "a9758a6105b67da05d15e2eb078eaa242a6b278b07ba0d0cd5da238e59ea7b30"
    page_range: [321, 321]
    quote: "If an index consistently appears in either the first or fourth quartile, it is likely to indicate a problem with the benchmark, not with the managers."
    edge_type: "defines"
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p319:0321"
    chunk_hash: "ec01d6d9d037728d9ebfb9f0111f1de3cec22b6c8354b2937ec89a1c9e6a6077"
    page_range: [320, 320]
    quote: "we would expect the beta of the average manager characteristic line regression relative to the style index to be closer to 1.0, the standard error closer to zero, and the R-squared closer to 1.00."
    edge_type: "supports"
card_hash: "7c149dd64755fe7e6b7bb0b093550467aaf4c0b2821f81558a922a827ff1de34"
---
# Benchmark-Quality Validation Statistics

## Intuition

A benchmark is only useful if it actually looks like what the manager
does. Asserting that a style index is a "good proxy" is not enough; the
claim must be tested against the manager's realized return stream. Two
complementary checks do this. First, a regression-fit test: a better
benchmark should explain more of a manager's return variance than a broad
market index, so its characteristic-line statistics should be sharper.
Second, a relative-position test: over many periods a good benchmark
should sit near the *middle* of the peer universe it represents, not
persistently at an extreme. If the benchmark keeps landing in the top or
bottom quartile of the manager universe, the benchmark — not the
managers — is miscalibrated.

**Source:** Christopherson, Cariño & Ferson (2009) ch.25 pp.306-308 (PDF pp.319-321)

## Definition

**Regression-fit (characteristic-line) test.** Regress the manager (or the
style-universe average) return series on the candidate benchmark return
series. A superior benchmark is signaled by three movements relative to a
broad-market baseline: the slope **beta approaches 1.0**, the **standard
error of the regression approaches zero**, and the **R-squared approaches
1.00**. Christopherson, Cariño & Ferson state: "If the style indexes are
better proxies than the broad market for the style universes, then we
would expect the beta of the average manager characteristic line
regression relative to the style index to be closer to 1.0, the standard
error closer to zero, and the R-squared closer to 1.00."

**Quartile / box-chart median test.** An additional appropriateness check
is whether the benchmark is, on average over time, closer to the *median*
manager than a broad market index is. The diagnostic rule: "If an index
consistently appears in either the first or fourth quartile, it is likely
to indicate a problem with the benchmark, not with the managers."

**Source:** Christopherson, Cariño & Ferson (2009) ch.25 pp.307-308 (PDF pp.320-321)

## Mathematical Reasoning

Let `r_p` be the manager (or universe-average) return and `r_b` the
candidate benchmark return. The characteristic-line regression is the
identity

```
r_p = alpha + beta * r_b + epsilon,    Var(r_p) = beta^2 * Var(r_b) + Var(epsilon)
R^2 = beta^2 * Var(r_b) / Var(r_p) = 1 - Var(epsilon)/Var(r_p)
```

A benchmark that is the manager's true normal portfolio absorbs almost all
systematic movement, so `Var(epsilon)` shrinks. Hence `R^2 -> 1` and the
standard error of the regression, `SE = sqrt(Var(epsilon))` adjusted for
degrees of freedom, falls toward zero. Beta near 1.0 says the manager's
risk magnitude matches the benchmark's: the universe average "varies more
like" the style index than like the broad market.

The book frames the gain in terms of *residual* variance, not raw `R^2`.
When the broad market already explains a large fraction of variance, a
small absolute `R^2` rise represents a large share of the *previously
unexplained* variance:

```
residual variance explained by style index = (R^2_style - R^2_market) / (1 - R^2_market)
```

The authors report this comparative-statics relationship qualitatively
and assert — without a closed-form proof — that the style benchmark is
therefore the better proxy: because so little variance is left unexplained
by the market, even a slight `R^2` improvement captures a large fraction of
the market-unexplained variance. The card asserts this conclusion at the
source's level of rigor and labels the gap: the text demonstrates the
pattern empirically rather than deriving an optimality theorem.

```
         BENCHMARK-FIT TEST                 MEDIAN-PROXIMITY TEST
  ------------------------------       ----------------------------------
  regress r_p on r_b:                  peer-universe box per period:
                                         max  ----
     beta  --> 1.0   (risk match)       Q1   ----  <- benchmark here often?
     SE    --> 0     (tight fit)        med  ----  <- GOOD benchmark sits here
     R^2   --> 1.0   (more explained)   Q3   ----
                                         min  ----  <- benchmark here often?
  better proxy than broad market        persistent Q1/Q4 => bad benchmark
```

**Source:** Christopherson, Cariño & Ferson (2009) ch.25 pp.307-308 (PDF pp.320-321)

## See Also

- [`pa-valid-benchmark-properties.md`](pa-valid-benchmark-properties.md) — the qualitative property checklist these statistics quantitatively test.
- [`pa-normal-portfolio-construction.md`](pa-normal-portfolio-construction.md) — the normal portfolio is the limiting "perfect" benchmark these fit statistics approach.
- [`pa-returns-based-style-analysis.md`](pa-returns-based-style-analysis.md) — companion regression-based method for inferring a manager's style benchmark from returns.
- [`pa-regression-appraisal-jensen-treynor.md`](pa-regression-appraisal-jensen-treynor.md) — same characteristic-line regression machinery used for risk-adjusted appraisal.
- [`pa-active-risk-tracking-error-ex-ante-vs-ex-post.md`](pa-active-risk-tracking-error-ex-ante-vs-ex-post.md) — tracking error as a benchmark-fit diagnostic: a low active risk against a candidate benchmark is the residual-variance counterpart of these fit statistics.

## Escalate to Raw When

- You need the actual worked regression statistics (the book's Table 25.5
  gives specific beta, standard error, and R-squared values for the
  Price-Driven and Earnings Growth universes versus each candidate index),
  the worked R-squared rise (from roughly 0.93 to 0.96 for the Price-Driven
  universe versus its style index), or the numeric residual-variance-explained
  figures (~42%, ~56%).
- You need the period-by-period quartile distributions (Figures 25.5-25.8)
  to judge whether a specific style index persistently lands at an extreme.
- You are deciding how many periods of return history are required before
  the median-proximity test is reliable; the chapter relies on a ten-year
  history (1979-1988) but does not state a formal minimum.
- You need the GIPS-compliant benchmark-disclosure rules (cross-ref the
  17 ethics GIPS material), which govern how validated benchmarks must be
  presented to clients.
