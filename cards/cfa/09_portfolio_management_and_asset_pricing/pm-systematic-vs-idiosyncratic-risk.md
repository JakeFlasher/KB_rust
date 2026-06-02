---
schema_version: "cacg.v0"
id: "pm-systematic-vs-idiosyncratic-risk"
title: "Systematic vs Idiosyncratic Risk"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Systematic vs Idiosyncratic Risk: decomposing total return variance into a systematic component (priced by a factor model) and an idiosyncratic component (diversifiable in a broad portfolio), and showing the asymptotic limit of diversification benefit"
tags: ["portfolio-management", "systematic-risk", "diversification"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3169:4725"
    chunk_hash: "042cfd4583cfbdbe3d534430fd5db43dfb8936dd215c704aef496744efb844be"
    page_range: [3169, 3169]
    quote: "Systematic risk, also known as non-diversifiable or market risk, is the risk that affects the entire market or economy."
    edge_type: "defines"
card_hash: "94063485e804fa83f1788cd98c5fd8ac0426732fe47152df38fcd5dbb104aac9"
---
# Systematic vs Idiosyncratic Risk

## Intuition

Total return variance has two components — a systematic part driven
by exposure to common factors (the market and any priced extension
factors), and an idiosyncratic part specific to the asset itself.
The two components have very different fates in a portfolio. The
systematic part adds up across holdings and cannot be removed by
diversification. The idiosyncratic part averages toward zero as the
portfolio is broadened across uncorrelated holdings, because the
specific shocks are independent in expectation. The investor earns a
risk premium for bearing systematic risk; idiosyncratic risk is
unpaid because it can be eliminated for free by holding a broad
portfolio. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.500-540.

```
        var(r_i) = systematic + idiosyncratic
        ============================================
        var(r_p) for N-asset equally weighted portfolio:

        |                                           total (any single
        |       *                                    asset)
        |        \
        |         \  *
        |          \
        |           \   *
        |            \      *
        |             \           *
        |              \                 *
        |               +-------------------------- systematic floor
        |                                           (priced; can't be
        |                                            diversified away)
        +------------------------------------> N (number of holdings)
                  -- idiosyncratic component decays as 1/N --
```

The asymptotic floor is the systematic risk that all broad
portfolios share by construction. Adding more uncorrelated holdings
reduces idiosyncratic risk at the rate `1/N`, but the systematic
risk does not decline because every holding shares some exposure to
the same priced factors. The floor is not zero — it is the
unavoidable cost of being invested in the market, and the investor
is paid for bearing it. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.500-540.

## Definition

For a single-factor decomposition under CAPM, the asset return
splits into a market-driven systematic term plus a residual specific
term that is uncorrelated with the market by construction.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.500-540.

```
r_i - R_f = beta_i · (r_M - R_f) + epsilon_i
```

Variance decomposition follows from the uncorrelated residual
property. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.500-540.

```
var(r_i) = beta_i^2 · var(r_M) + var(epsilon_i)
                     ^                ^
                     |                +-- idiosyncratic
                     +-- systematic
```

In a multifactor extension, the systematic component generalizes to
a sum of factor-driven terms, and the idiosyncratic component
remains the residual uncorrelated with all the factors. **Source:**
CFA L1 Curriculum (2022) Vol.6/pp.500-540.

```
var(r_i) = sum_j sum_k  beta_(i,j) · beta_(i,k) · cov(F_j, F_k)
        + var(epsilon_i)
```

The idiosyncratic variance `var(epsilon_i)` is sometimes called
specific risk or unsystematic risk; the systematic variance is
sometimes called factor risk, market risk, or non-diversifiable
risk. The terminology varies; the partition does not. **Source:**
CFA L1 Curriculum (2022) Vol.6/pp.500-540.

## Mathematical Reasoning

The diversification limit is most cleanly seen for an equally-
weighted portfolio of `N` assets with identical idiosyncratic
variance and pairwise residual independence. The portfolio variance
splits into a systematic term that does not depend on `N` and an
idiosyncratic term that scales as `1/N`. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.460-500.

```
var(r_p) = beta_p^2 · var(r_M) + (1 / N) · var(epsilon_average)
                ^                       ^
                +-- systematic floor    +-- decays to zero as N grows
```

As `N` grows, the second term shrinks to zero and the portfolio's
total variance converges to the systematic floor `beta_p^2 ·
var(r_M)`. This is the asymptotic argument for why broad
diversification eliminates idiosyncratic risk and leaves only the
unavoidable systematic exposure. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.460-500.

The decomposition has a direct implication for risk-premium pricing
under CAPM. Only systematic risk earns a premium because only
systematic risk cannot be diversified away. An asset with high
idiosyncratic variance and low beta has high standalone variance
but a small expected-return premium; an asset with low standalone
variance but high beta has a large expected-return premium relative
to its variance. The CAPM and the diversification logic together
produce the principle: in a CAPM world, idiosyncratic risk is
unpriced. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.500-540.

The empirical violations of strict CAPM (size, value, momentum
anomalies) are reframed in the multifactor extension as additional
priced factors rather than as deviations from the systematic /
idiosyncratic partition itself. The partition is preserved; what
changes is the dimension of the systematic component. The unpaid
status of the residual `epsilon_i` after netting all priced factors
remains. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.500-540.

A specific implication for portfolio construction: a "well-
diversified" portfolio is one whose idiosyncratic variance is
small relative to its systematic variance — the convergence has
substantially completed. The first holdings deliver the bulk of
the asymptotic floor approach because the `1/N` decay is steepest
at small `N`; further additions yield rapidly diminishing variance
reduction once the systematic floor is approached. **Source:**
CFA L1 Curriculum (2022) Vol.6/pp.500-540.

## See Also

- [`pm-beta-and-factor-exposure.md`](pm-beta-and-factor-exposure.md) — single-factor beta and the systematic / idiosyncratic split it generates
- [`pm-diversification-and-correlation.md`](pm-diversification-and-correlation.md) — covariance and correlation as the inputs that determine portfolio variance and the diversification benefit
- [`pm-factor-models-intuition.md`](pm-factor-models-intuition.md) — multifactor extension that generalizes the systematic component while preserving the partition

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R49 / R50 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.500-540.

- Variance-covariance matrix estimation issues (sample size needs,
  shrinkage estimators, factor-model collapse for high-dimensional
  estimation) — these belong in future-01 quantitative methods.
  **Source:** CFA L1 Curriculum (2022) Vol.6/pp.500-540.
- Expected-shortfall and tail-risk measures that go beyond
  variance — Vol.6 mentions these in passing; deeper VaR / ES
  framework lives in subcorpus 11 (closed via v11; see
  `rm-var-and-es-taxonomy.md` and `rm-expected-shortfall-mechanics.md`
  under `.claude/knowledge/11_risk_management/`). **Source:** CFA L1
  Curriculum (2022) Vol.6/pp.500-540.
- Higher-moment risk decomposition (co-skewness, co-kurtosis as
  priced factors) — Vol.6 mentions but L1 framing covers only
  variance. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.500-540.
