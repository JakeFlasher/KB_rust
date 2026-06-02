---
schema_version: "cacg.v0"
id: "eq-fama-french-construction-at-security-level"
title: "Fama-French Construction at Security Level"
reading_id: "05_equity"
summary: "Fama-French extends CAPM with two portfolio-constructed factors at security level: HML (high-minus-low book-to-market, value) and SMB (small-minus-big market cap, size). Each factor is a long-short portfolio return series; a security's exposure is its beta in a multivariate regression. Damodaran's market-efficiency evidence motivates the extension."
tags: ["equity", "fama-french"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p120:0121"
    chunk_hash: "4a1e2a039febae6c1aa9cd1f7f0c933d507b365a7d4c3600d8867a6366aa4b83"
    page_range: [120, 120]
    quote: "But how do we measure default and equity risk? More importantly, how do we come up with the default and equity risk premiums?"
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p217:0235"
    chunk_hash: "41020512f003abdadd7af185ab485cadb734a28022a85cc9a0e64ded89627178"
    page_range: [217, 218]
    quote: "Studies such as Banz (1981) and Keim (1983) have consistently found that smaller firms (in terms of market value of equity) earn higher returns than larger firms of equivalent risk"
    edge_type: "supports"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p690:0897"
    chunk_hash: "ee01be57ae3e0fdd5d49d093d5e57004313f7e3910a1a2dc4781183b44a86f23"
    page_range: [690, 691]
    quote: "Stocks selling for well below the book value of equity have generally been considered undervalued, while those selling for more than book value have been targeted as overvalued."
    edge_type: "supports"
card_hash: "33282a5e297c2ab23bd38d6db4392f053536840621a3246ab0ad2eb73f5d2278"
---
# Fama-French Construction at Security Level

## Intuition

CAPM predicts that a single market-beta factor explains the cross-
section of equity expected returns. Damodaran's Ch.6 market-
efficiency evidence shows this prediction is empirically incomplete:
firms with high book-to-market ratios (low P/B, "value") have
historically earned excess returns relative to firms with low book-
to-market ratios ("growth"); firms with small market capitalization
have historically earned excess returns relative to firms with
large market capitalization. These two cross-sectional patterns are
the empirical foundation of the Fama-French extension. **Source:**
Damodaran (2012) Ch.4 pp.120-155.

The Fama-French construction takes these patterns and operationalizes
them as PORTFOLIO FACTORS at the security level: long a portfolio of
high-B/M securities and short a portfolio of low-B/M securities
constructs the value factor; long small-cap and short large-cap
constructs the size factor. Each factor is itself a portfolio return
series that the analyst can use to estimate a security's exposure
(its beta on that factor) by regression. The single-beta CAPM
generalizes to a multi-beta model with one beta per factor.
**Source:** Damodaran (2012) Ch.6 pp.183-245.

```
single-factor CAPM         multi-beta Fama-French
                                                                
r_i = Rf + beta_i_M · ERP   r_i = Rf
                                + beta_i_M · ERP
                                + beta_i_HML · HML_premium
                                + beta_i_SMB · SMB_premium

where:                      where:
  beta_i_M = exposure to     beta_i_M = exposure to market
   market portfolio          beta_i_HML = exposure to value (high-B/M
                                          minus low-B/M) portfolio
                             beta_i_SMB = exposure to size (small-cap
                                          minus large-cap) portfolio

  one cross-sectional        three cross-sectional
  return source              return sources
```

## Definition

The Fama-French three-factor model extends CAPM by adding two
cross-sectional return factors constructed at the security level:
HML (high-minus-low book-to-market, the value factor) and SMB
(small-minus-big market capitalization, the size factor). Each
factor is a long-short portfolio return series; a security's
exposure to each factor is its beta-coefficient in a multivariate
regression of the security's excess return on the three factor
return series. **Source:** Damodaran (2012) Ch.4 pp.120-155.

The HML factor is constructed by sorting the equity universe on
book-to-market ratio (B/M = book value of equity / market value of
equity) at a portfolio-formation date, taking the top decile or
quintile (high B/M = value stocks) long and the bottom decile or
quintile (low B/M = growth stocks) short, and computing the
monthly return spread. Damodaran's Ch.19 P/B framing (see
[`eq-pb-and-multiples-taxonomy`](./eq-pb-and-multiples-taxonomy.md))
supplies the B/M denominator: B/M is the inverse of P/B. **Source:**
Damodaran (2012) Ch.19 pp.690-725.

The SMB factor is constructed by sorting the equity universe on
market capitalization at the portfolio-formation date, taking the
small-cap subset long and the large-cap subset short, and computing
the monthly return spread. The factor's value reflects the
historically-documented small-cap premium that Damodaran Ch.6
surveys. **Source:** Damodaran (2012) Ch.6 pp.183-245.

The market-beta factor is the same single-factor exposure CAPM
uses (see [`eq-discount-rate-and-required-return-foundations`](./eq-discount-rate-and-required-return-foundations.md)):
the security's regression slope on the market portfolio's excess
return. In the Fama-French construction, the market-beta is one of
three factor exposures rather than the only one. **Source:**
Damodaran (2012) Ch.4 pp.120-155.

The depth of REGRESSION ESTIMATION machinery — the OLS estimator
itself (`qm-multiple-linear-regression-foundations.md`), the
classical-assumption-violation diagnostics
(`qm-regression-assumption-violations.md`), and the per-observation
leverage and influence diagnostics
(`qm-influence-analysis-leverage.md`) — anchors in the 01
quantitative-methods vertical; deeper Fama-MacBeth two-pass
regressions, errors-in-variables corrections, panel-vs-cross-section
choice, and distributional assumptions on factor returns sit beyond
the L1 quant-methods scope and belong to a raw econometric
reference. The
intuition-depth statement here is: a security's required return
under the Fama-French model is the riskless rate plus the sum of
factor-beta-times-factor-premium contributions; the factor
construction is portfolio-based; the betas are estimated from
historical data. **Source:** Damodaran (2012) Ch.4 pp.120-155.

The cross-vertical bridge to 09 (Portfolio Management) is direct:
09's `pm-multifactor-asset-pricing-intuition.md` covers the
asset-pricing-theory framing of multifactor models at the
portfolio level; this card supplies the security-level
construction that 09 deferred. The Fama-French model is one
specific multifactor instantiation; the broader multifactor
framework includes APT (Arbitrage Pricing Theory), which Damodaran
discusses in Ch.4 alongside CAPM as the alternative single-vs-
multifactor decomposition of the risk premium. **Source:**
Damodaran (2012) Ch.4 pp.120-155.

## Mathematical Reasoning

The Fama-French three-factor cost-of-equity equation in symbolic
form expresses required return as a sum of factor-beta-times-
factor-premium contributions. **Source:** Damodaran (2012) Ch.4
pp.120-155.

```
r_i  =  Rf
     +  beta_i_M    · (E[Rm] - Rf)        <-- market factor
     +  beta_i_HML  · E[HML_premium]      <-- value factor
     +  beta_i_SMB  · E[SMB_premium]      <-- size factor

where:
  beta_i_M    = regression slope of (R_i - Rf) on (Rm - Rf)
  beta_i_HML  = regression slope of (R_i - Rf) on HML_t
  beta_i_SMB  = regression slope of (R_i - Rf) on SMB_t

  HML_t = R_high_BM_portfolio_t  -  R_low_BM_portfolio_t
  SMB_t = R_small_cap_portfolio_t -  R_large_cap_portfolio_t

  E[HML_premium] = expected long-run mean of HML_t
  E[SMB_premium] = expected long-run mean of SMB_t
```

The portfolio-formation step in symbolic form sorts the universe
of N securities on B/M (ascending) and on market cap (ascending),
takes top and bottom subsets, and computes the spread. **Source:**
Damodaran (2012) Ch.6 pp.183-245.

```
sort securities by B/M, low to high
  H_BM = top quantile (high B/M = value stocks)
  L_BM = bottom quantile (low B/M = growth stocks)
  HML_t = (1/|H_BM|) · sum_{i in H_BM} R_i_t
        - (1/|L_BM|) · sum_{i in L_BM} R_i_t

sort securities by market cap, low to high
  S_cap = bottom quantile (small cap)
  L_cap = top quantile (large cap)
  SMB_t = (1/|S_cap|) · sum_{i in S_cap} R_i_t
        - (1/|L_cap|) · sum_{i in L_cap} R_i_t
```

The portfolio quantile choice (decile, quintile, terciles) is an
analyst-implementation parameter; Damodaran's Ch.6 evidence uses
quintile sorts as the canonical exposition. **Source:** Damodaran
(2012) Ch.6 pp.183-245.

The factor-beta estimation in symbolic form runs a multivariate
regression of the security's excess return on the three factor
return series. **Source:** Damodaran (2012) Ch.4 pp.120-155.

```
(R_i_t - Rf_t) = alpha_i
              + beta_i_M    · (Rm_t - Rf_t)
              + beta_i_HML  · HML_t
              + beta_i_SMB  · SMB_t
              + e_i_t

slope estimates beta_i_M / beta_i_HML / beta_i_SMB are the
security's exposures; alpha_i is the residual return not
explained by the three factors; e_i_t is the firm-specific
residual at time t
```

The deeper estimation theory — sample-period choice, weekly-vs-
monthly-vs-daily frequency, robustness to outliers (anchored in
`qm-influence-analysis-leverage.md` for L1-depth leverage and
studentised-residual diagnostics), errors-in-variables corrections
— sits beyond the L1 quant-methods scope covered in 01 and belongs
to a raw econometric reference. **Source:** Damodaran (2012) Ch.4
pp.120-155.

The Damodaran framing distinguishes the FF construction from the
APT (Arbitrage Pricing Theory) generalization: APT does not
specify which factors to use; FF specifies market + value + size
as a parsimonious empirical choice. The choice of which factors
to include is an empirical question that Damodaran treats as
unresolved in Ch.4; the FF three-factor specification is one
canonical answer. **Source:** Damodaran (2012) Ch.4 pp.120-155.

## See Also

- [`eq-discount-rate-and-required-return-foundations`](./eq-discount-rate-and-required-return-foundations.md) — the CAPM single-factor baseline that FF extends
- [`eq-pb-and-multiples-taxonomy`](./eq-pb-and-multiples-taxonomy.md) — the P/B taxonomy that supplies the B/M denominator for HML
- [`pm-multifactor-asset-pricing-intuition`](../09_portfolio_management_and_asset_pricing/pm-multifactor-asset-pricing-intuition.md) — the asset-pricing-theory framing of multifactor models upstream in 09
- [`pm-anomalies-and-cross-sectional-pricing`](../09_portfolio_management_and_asset_pricing/pm-anomalies-and-cross-sectional-pricing.md) — the cross-sectional anomaly evidence upstream in 09

## Escalate to Raw When

Open Damodaran Ch.4 / Ch.6 directly when any of the criteria below
applies. **Source:** Damodaran (2012) Ch.4 pp.120-155.

- the analyst needs the full APT-vs-FF comparison framework for choosing between specifications — Damodaran Ch.4 develops the comparison in detail. **Source:** Damodaran (2012) Ch.4 pp.120-155.
- the cross-sectional anomaly evidence supporting the FF factor choice is contested or non-obvious for the target market (emerging markets, small-cap-tilted indices, sector ETFs) — Damodaran Ch.6 surveys the evidence. **Source:** Damodaran (2012) Ch.6 pp.183-245.
- the regression-estimation depth needed exceeds the intuition-depth ceiling (e.g., panel data with time-varying exposures, robust regression for outliers, errors-in-variables corrections) — the L1 quant-methods foundations are in `qm-multiple-linear-regression-foundations.md` + `qm-regression-assumption-violations.md` + `qm-influence-analysis-leverage.md`, and the deeper machinery sits beyond L1 in a raw econometric reference. **Source:** Damodaran (2012) Ch.4 pp.120-155.
