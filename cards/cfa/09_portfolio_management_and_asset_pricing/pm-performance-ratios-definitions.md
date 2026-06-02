---
schema_version: "cacg.v0"
id: "pm-performance-ratios-definitions"
title: "Performance Ratios — Sharpe, Treynor, Information"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Performance Ratios — Sharpe, Treynor, Information: defining the symbolic forms of the Sharpe ratio, Treynor ratio, and Information Ratio at L1 depth — what each measures, which denominator-risk concept each uses, and how the choice of risk measure determines which ratio is the right tool"
tags: ["portfolio-management", "sharpe-ratio", "risk-adjusted-return"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3188:4756"
    chunk_hash: "a0b96d424a4abc9183c7e61dd279455f9a4fdcf824e613d01998ff24602fbb92"
    page_range: [3188, 3189]
    quote: "A commonly used measure of performance is the Sharpe ratio, which is defined as the portfolio’s risk premium divided by its risk."
    edge_type: "defines"
card_hash: "7e213390e5f1e2176d57504bf3c48e37e4064a43c87c237c80ba5a50f25640a2"
---
# Performance Ratios — Sharpe, Treynor, Information

## Intuition

A performance ratio scales realized excess return by a risk measure
in the denominator. The choice of risk measure tells you which
question the ratio answers. The Sharpe ratio uses total volatility
and answers "per unit of total risk taken, how much excess return
did this portfolio earn?" The Treynor ratio uses market beta and
answers "per unit of systematic risk, how much excess return?" The
Information Ratio uses tracking error against a benchmark and
answers "per unit of active risk relative to the benchmark, how much
active return?" **Source:** CFA L1 Curriculum (2022) Vol.6/pp.555-572.

```
        Sharpe                Treynor              Information
        ------                -------              -----------
        excess return         excess return        active return
        --------------        --------------       --------------
        total volatility      market beta          tracking error
        sigma_p               beta_p               sigma_(p - b)

        scope: stand-alone    scope: well-          scope: portfolio
        portfolio                  diversified           vs benchmark
                                   portfolio in
                                   a CAPM world
```

The choice of denominator carries an implicit assumption about
which risk the investor cares about. A retail investor putting all
their wealth in one fund cares about total volatility — the Sharpe
ratio is the right tool. An institutional investor blending many
diversified building blocks cares about each building block's
systematic exposure within a CAPM-style framework — Treynor reframes
the same portfolio with beta in the denominator. An investor
comparing an active manager to a passive benchmark cares about
deviation from that benchmark — the Information Ratio captures
exactly that. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.555-572.

## Definition

The Sharpe ratio is excess return over the risk-free rate divided
by the standard deviation of returns. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.555-572.

```
Sharpe_p = (E[r_p] - R_f) / sigma_p
```

The Treynor ratio is excess return over the risk-free rate divided
by the portfolio's beta to the market. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.555-572.

```
Treynor_p = (E[r_p] - R_f) / beta_p
```

The Information Ratio is active return (portfolio minus benchmark)
divided by tracking error (the standard deviation of the active
return). **Source:** CFA L1 Curriculum (2022) Vol.6/pp.555-572.

```
IR_p = (E[r_p] - E[r_b]) / sigma_(r_p - r_b)
```

In each formula, the numerator is the realized excess that the
investor cares about, and the denominator is the realized risk that
the investor took to earn it. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.555-572.

## Mathematical Reasoning

The Sharpe and Treynor ratios are proportional — but not equal —
when the portfolio is well-diversified and lies on the Capital
Market Line. On the CML, total volatility is proportional to
beta: every CML portfolio is a leveraged or de-leveraged
combination of the market, so `sigma_p = beta_p · sigma_M`.
Substituting in either ratio gives `Treynor_p = sigma_M ·
Sharpe_p`; the two ratios share a common rank ordering of CML
portfolios (since the market volatility `sigma_M` is a positive
constant for all of them) but their numerical values differ by
the `sigma_M` scale factor. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.555-572.

```
on the CML:  sigma_p = beta_p · sigma_M
Sharpe_p     = (E[r_p] - R_f) / (beta_p · sigma_M)
Treynor_p    = (E[r_p] - R_f) / beta_p
             = sigma_M · Sharpe_p
```

The two ratios diverge when the portfolio carries idiosyncratic
risk. A poorly diversified portfolio has `sigma_p > beta_p · sigma_M`
— the standalone variance exceeds the systematic component — and the
Sharpe ratio falls relative to the Treynor ratio. The gap quantifies
the cost of failing to diversify within the CAPM frame. **Source:**
CFA L1 Curriculum (2022) Vol.6/pp.555-572.

The Information Ratio is the corresponding measure when the
investor's reference is not the risk-free asset but a tracked
benchmark. The active-return numerator and tracking-error
denominator decompose the portfolio's behavior relative to the
benchmark. The Information Ratio is the slope of the manager's
realized active return per unit of active risk; under standard
assumptions, the maximum achievable IR governs the manager's value
add net of tracking-error budget. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.555-572.

A specific implication for ratio comparison: the three ratios are
not interchangeable rankings. A portfolio with low total volatility
but high market correlation may rank well on Sharpe and Treynor
together; a portfolio with low tracking error against a niche
benchmark but high standalone risk may rank well on IR and poorly
on Sharpe. The investor's evaluation question — stand-alone risk vs
systematic-only risk vs benchmark-relative risk — picks the
appropriate ratio. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.555-572.

The numerators above use expected (or realized) returns; both ex-
ante and ex-post variants exist and the curriculum uses both
depending on context. The ex-ante form treats the ratio as a
forecast quality measure; the ex-post form treats it as a realized
performance metric. The L1 framing presents the symbolic form
without engaging the estimation issue, which is sample-statistics
territory and belongs in future-01. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.555-572.

## See Also

- [`pm-capm-and-sml.md`](pm-capm-and-sml.md) — the CAPM single-factor frame in which Treynor's beta denominator is grounded
- [`pm-return-and-risk-fundamentals.md`](pm-return-and-risk-fundamentals.md) — the variance and standard deviation definitions used in the Sharpe denominator
- [`pm-tracking-error-and-active-risk.md`](pm-tracking-error-and-active-risk.md) — tracking error as the active-risk denominator in the Information Ratio

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R50 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.555-572.

- M-squared (Modigliani risk-adjusted performance) and Jensen's
  alpha as alternative ratio constructions — Vol.6 R50 introduces
  them and the deeper attribution treatment belongs in future-15.
  **Source:** CFA L1 Curriculum (2022) Vol.6/pp.555-572.
- Sortino ratio (downside-deviation denominator) and other
  asymmetric-risk variants — these are mentioned in passing at L1
  and developed elsewhere. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.555-572.
- Estimation properties of these ratios (sampling distribution,
  small-sample bias, t-statistic significance) — these belong in
  future-01 quantitative methods. **Source:** CFA L1 Curriculum
  (2022) Vol.6/pp.555-572.
