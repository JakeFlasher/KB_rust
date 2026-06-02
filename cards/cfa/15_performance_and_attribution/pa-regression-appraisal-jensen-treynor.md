---
schema_version: "cacg.v0"
id: "pa-regression-appraisal-jensen-treynor"
title: "Regression Appraisal: Jensen's Alpha and the Treynor Ratio"
reading_id: "15_performance_and_attribution"
summary: "CAPM-regression appraisal family: Jensen's alpha (intercept), Treynor and modified-Treynor reward ratios, the appraisal (Treynor-Black) and modified-Jensen ratios, bull/bear betas, and Fama selectivity/diversification/net-selectivity decomposition."
tags: ["jensen-alpha", "treynor-ratio", "fama-decomposition"]
citations:
  - source_id: "pa_bacon_2022_rapm"
    chunk_id: "pa_bacon_2022_rapm:p100:0090"
    chunk_hash: "de17318fef996c6b08cdd880ed4ad17b314d17687b29d157e409d5bc6ccb1c28"
    page_range: [101, 101]
    quote: "Jensen’s alpha is the intercept of the regression equation in the CAPM and is in effect the excess return adjusted for systematic risk."
    edge_type: "defines"
card_hash: "ccb4adf9f14697f21a78a17226ff62673a666ce0cbd88d61da43b158d73c1d63"
---
# Regression Appraisal: Jensen's Alpha and the Treynor Ratio

## Intuition

If you regress a portfolio's excess return on its benchmark's excess return, the
fitted line tells two stories at once. Its **slope** is beta: how much systematic
(market-driven) risk the manager carries. Its **intercept** is the return the
manager earned that the market did not hand them for free — Jensen's alpha, the
abnormal return left unexplained by systematic (beta) risk and, hopefully,
attributable to skill. (Alpha is *not* itself a measure of specific risk; it is
the appraisal/Treynor-Black ratio that scales alpha *by* residual specific risk.)
The same regression
output then feeds a whole family of single-number "appraisal" ratios that pair a
reward (excess return, or alpha) with a risk denominator (beta, systematic risk,
or specific risk). Each ratio is a gradient in a reward/risk plane; steeper is
better, and which risk you put on the horizontal axis is the only thing that
changes between them.

**Source:** Bacon (2022) Ch.4 (Regression Analysis) pp.79-102 (PDF pp.101-124)

## Definition

Working in excess-return space (subtract the risk-free rate `rF`), the CAPM
regression is

```
r - rF = a + b x (b_mkt - rF) + e
```

where the slope `b` (beta) is systematic risk and the intercept `a` is
**Jensen's alpha** — "the intercept of the regression equation in the CAPM and
is in effect the excess return adjusted for systematic risk," also called
Jensen's measure, Jensen's differential return, or ex-post alpha.

The appraisal family built from this regression:

- **Treynor ratio (reward to volatility):** excess return over the risk-free
  rate divided by beta — like the Sharpe ratio but with *systematic* risk, not
  total risk, in the denominator. It ignores specific risk; for a fully
  diversified portfolio it ranks identically to Sharpe.
- **Modified Treynor ratio:** excess return divided by systematic risk `sigma_S`
  (= beta x benchmark variability), a form more consistent with Sharpe's units.
- **Appraisal ratio (Treynor-Black):** Jensen's alpha divided by **specific**
  risk (standard deviation of the residual). Measures systematic-risk-adjusted
  reward per unit of specific risk taken — conceptually the information ratio but
  with alpha over residual risk rather than active return over tracking error.
- **Modified Jensen:** Jensen's alpha divided by **systematic** risk (beta), the
  systematic-risk-adjusted return per unit of systematic risk.
- **Bull beta / bear beta:** regression slopes fitted to only positive
  (`b+`) or only negative (`b-`) benchmark returns; their ratio is the **beta
  timing ratio**, capturing convexity (higher beta in up markets).
- **Fama decomposition:** splits excess return into **selectivity** (= Jensen's
  alpha) plus **systematic-risk** return, then splits selectivity into
  **net selectivity** and **diversification** (the return that justifies giving
  up diversification).

**Source:** Bacon (2022) Ch.4 (Regression Analysis) pp.79-102 (PDF pp.101-124); Jensen's alpha p.79 (PDF p.101)

## Mathematical Reasoning

Jensen's alpha is the regression intercept, so dropping the error term for the
ex-post estimate gives the identity

```
alpha = r - rF - b x (b_mkt - rF).
```

Subtracting the benchmark return from both sides of the CAPM line rewrites
arithmetic excess return as a beta-tilt identity,

```
r - b_mkt = alpha + (b - 1) x (b_mkt - rF),
```

so excess-over-benchmark equals alpha plus a term that vanishes only when
beta = 1. Total risk splits orthogonally — specific and systematic risk are by
construction independent, so by a Pythagorean relation

```
sigma^2 = sigma_S^2 + sigma_e^2,   with   sigma_S = beta x sigma_b,
```

which is exactly why the appraisal ratio (alpha / sigma_e) and the modified
Jensen (alpha / sigma_S, or alpha / beta) use the two orthogonal legs as
distinct denominators.

The **Fama decomposition** is a chain of identities. Excess return is
selectivity plus systematic-risk return:

```
r - rF = [ r - rF - b x (b_mkt - rF) ]  +  [ b x (b_mkt - rF) ]
            \---------- selectivity ----------/   \---- systematic risk ----/
```

Selectivity equals Jensen's alpha. To value the diversification surrendered,
define the **Fama beta** `beta_F = sigma / sigma_b` — the beta whose systematic
risk equals the portfolio's *total* risk. Because total risk >= systematic risk,
`beta_F >= beta` always. The diversification charge is

```
d = (beta_F - beta) x (b_mkt - rF),
```

and **net selectivity** is what remains: `S_net = alpha - d`. A negative net
selectivity means the manager did not earn enough alpha to justify abandoning
full diversification.

The book asserts the CAPM linearity assumption as its "biggest limitation"
without deriving a correction, and flags that when R-squared falls much below
about 0.7 the resulting alphas, betas, and every appraisal measure derived from
them should be ignored — these are stated cautions, not proven bounds, and the
card asserts them at the source's level of rigor.

**Source:** Bacon (2022) Ch.4 (Regression Analysis) pp.78-102 (PDF pp.100-124); Jensen identity p.79 (PDF p.101), CAPM-linearity limit p.78 (PDF p.100), R-squared caution p.91 (PDF p.113), Fama decomposition pp.100-102 (PDF pp.122-124)

```
 reward/risk planes - same vertical reward, different horizontal risk
 (gradient from origin = the ratio; steeper is better)

  excess        |        .B        alpha       |     .B
  return        |     .A'           (Jensen)    |  .A'
  r - rF        |  .'                           | .'
                |.'________________             |.'________________
                  systematic risk (beta)          specific risk (sigma_e)
                  => TREYNOR ratio                 => APPRAISAL ratio
                                                    (Treynor-Black)

  bull/bear split:   steeper slope b+ for          beta timing ratio = b+ / b-
                     positive benchmark returns,    (> 1 desirable; b+ need only
                     flatter slope b- for negative   exceed b-, not exceed 1)
```

**Source:** Bacon (2022) Ch.4 (Regression Analysis) Figures 4.2 (p.84, PDF p.106), 4.8 (p.98, PDF p.120), 4.9 (p.99, PDF p.121)

## Boundary Notes

This card covers the CAPM single-factor regression appraisal toolkit. Bacon
extends it with multi-factor market-timing regressions (Henriksson-Merton's
up-market dummy, Treynor-Mazuy's quadratic term) and the Fama-French
three-factor model (adding SMB and HML); those multifactor and conditional-alpha
treatments live in the multifactor-alpha sibling. The Sharpe and information
ratios — total-risk and active-risk analogues sharing the same reward/risk
design — live in the variability-ratios sibling.

**Source:** Bacon (2022) Ch.4 (Regression Analysis) pp.85-102 (PDF pp.107-124)

## See Also

- [`pa-variability-ratios-sharpe-information.md`](pa-variability-ratios-sharpe-information.md) — Sharpe (total risk) and information (active risk) ratios; same reward/risk design, different denominators.
- [`pa-multifactor-alpha-timing-conditional.md`](pa-multifactor-alpha-timing-conditional.md) — extends single-beta alpha to Fama-French and market-timing regressions.
- [`pa-active-risk-tracking-error-ex-ante-vs-ex-post.md`](pa-active-risk-tracking-error-ex-ante-vs-ex-post.md) — tracking error, the active-risk denominator the appraisal ratio deliberately replaces with specific risk.
- [`pa-arithmetic-vs-geometric-excess-return.md`](pa-arithmetic-vs-geometric-excess-return.md) — defines the excess-return numerator these ratios consume.

Related cross-vertical: pm-* portfolio-management cards develop CAPM and the
security market line as the asset-pricing foundation for beta and alpha.

## Escalate to Raw When

- You need a worked numerical example (Bacon computes beta = 0.982, Jensen's
  alpha = -0.082, bull beta = 1.035, bear beta = 0.948, beta timing ratio =
  1.092 from the Table 4.1-4.7 monthly data — all deferred here per the
  no-worked-arithmetic rule).
- You need the full regression formulas for CAPM beta with the risk-free-rate
  adjustment, annualised alpha variants, or the residual/specific-risk
  computation (Equations 4.7-4.25).
- You need the market-timing multiple-regression specifications
  (Henriksson-Merton Eq. 4.15, Treynor-Mazuy Eq. 4.16) or the Fama-French
  three-factor equation (Eq. 4.43) with their coefficient interpretations.
- You need the geometry-of-risk / risk-compass visual framework (Figures
  4.4-4.7) relating correlation, tracking error, and the law of cosines.

**Source:** Bacon (2022) Ch.4 (Regression Analysis) pp.75-102 (PDF pp.97-124)
