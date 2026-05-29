---
schema_version: "cacg.v0"
id: "pm-return-and-risk-fundamentals"
title: "Return and Risk Fundamentals"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Return and Risk Fundamentals: framing return measures (holding period return, arithmetic mean, geometric mean) and risk measures (variance, semi-variance, downside) at the holding and portfolio level"
tags: ["portfolio-management", "return-measures", "holding-period-return"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3079:4599"
    chunk_hash: "b88d02e1073088a7d13a01bd0917a6c48f073044376c6f677296d030c0a7a7f9"
    page_range: [3079, 3080]
    quote: "A holding period return is the return earned from holding an asset for a single specified period of time."
    edge_type: "defines"
card_hash: "9ce19eef05cb2277350bc97111517f1485562c9f2cc2386d418a11574bfc6d55"
---
# Return and Risk Fundamentals

## Intuition

Return is a random variable that summarizes one period's investment
outcome relative to the capital deployed. Risk quantifies the
dispersion or downside of the return distribution. The two are joint
descriptors of an asset or portfolio: a higher return is desirable; a
narrower or less-asymmetric distribution is desirable. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.442-475.

```
   density f(r)
       ^
       |          .  <-- mode
       |        .   .
       |      .       .
       |    .           .
       |  .               .
       | .                  .  <-- right tail
       |.       E[r]          .
       +---|---*-|-|---|---|----> r
        loss      gain
       |<-- downside | upside -->|
       |  semi-       (uncon-
       | variance     strained
       |  domain      variance)
```

The investor decomposes the distribution into central tendency
(mean / median / mode), spread (variance / standard deviation), and
asymmetry (skewness / downside-only measures). Variance treats upside
and downside symmetrically; downside measures isolate losses below a
threshold the investor cares about. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.442-475.

## Definition

The single-period holding period return is the price change plus
income relative to the entry price. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.442-475.

```
HPR_i = (P_i - P_{i-1} + D_i) / P_{i-1}
```

Over multiple periods the arithmetic mean is the simple average of
period HPRs while the geometric mean is the constant compounded rate
equivalent that produces the observed terminal wealth. **Source:**
CFA L1 Curriculum (2022) Vol.6/pp.442-475.

```
arithmetic mean: r_bar  is the simple average of HPR_i across periods
geometric mean:  r_geo  satisfies  (1 + r_geo)^horizon  = product of (1 + HPR_i)
```

Variance measures squared deviation around the mean; standard
deviation is its square root. Semi-variance restricts the deviation
sum to returns below a threshold (often the mean or zero); downside
deviation is its square root. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.442-475.

```
variance:        var(r) = E[(r - E[r])^2]
semi-variance:   sv(r, c) = E[max(c - r, 0)^2]
```

## Mathematical Reasoning

The arithmetic mean overstates the geometric mean whenever returns
have non-zero variance; equality holds only for constant returns.
This is a direct consequence of Jensen's inequality applied to the
log function. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.442-475.

```
r_bar  >=  r_geo
gap proportional to var(r) (second-order term)
```

The geometric mean is the relevant compounding rate for buy-and-hold
horizon planning; the arithmetic mean is the relevant input for
expected one-period return calculations. The two answer different
questions, and the gap between them grows with return volatility.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.442-475.

Variance is symmetric — a return one unit above the mean and one unit
below contribute equally to the dispersion. Investors who care
asymmetrically about losses (loss aversion) under-account for tail
risk if they use variance alone. Semi-variance preserves the
quadratic penalty structure but sums only the below-threshold
deviations, restoring sensitivity to downside concentration without
penalizing upside spread. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.475-500.

The threshold `c` in semi-variance is typically chosen as the mean
(then `sv = downside variance around mean`), the risk-free rate
(then `sv = below-Rf variance`), or a wealth-preservation threshold
(then `sv = shortfall risk`). The choice anchors the metric to the
investor's loss-aversion reference point. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.475-500.

## See Also

- [`pm-portfolio-perspective.md`](pm-portfolio-perspective.md) — portfolio as joint distribution; portfolio expected return and variance formulas
- [`pm-diversification-and-correlation.md`](pm-diversification-and-correlation.md) — covariance and correlation as the inputs that determine portfolio variance

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R49 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.442-475.

- Return-distribution shape detail (skewness, kurtosis, tail
  parametrizations) beyond the mean / variance framing here. The
  reading discusses higher moments where they affect portfolio-
  selection conclusions. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.442-475.
- Time-weighted vs money-weighted return distinction for managed
  accounts; the distinction is necessary when external cash flows
  enter or leave the portfolio. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.442-475.
- Sample-statistics treatment (estimator bias, sampling distribution,
  hypothesis testing) of mean and variance — sample-stat theory
  belongs in future-01 quantitative methods. **Source:** CFA L1
  Curriculum (2022) Vol.6/pp.442-475.
