---
schema_version: "cacg.v0"
id: "eq-industry-and-sector-factor-models"
title: "Industry and Sector Factor Models"
reading_id: "05_equity"
summary: "Framing industry- and sector-level factor models as a distinct class from academic Fama-French factors: sector membership is a categorical cross-sectional exposure, sector-specific multiples (subscribers, reserves, ARR, FFO) reveal industry-specific cash-flow drivers, and Damodaran's financial-service-firms exemplar shows how regulatory structure makes generic valuation machinery break down."
tags: ["equity", "industry-sector"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p758:0980"
    chunk_hash: "73cf641d6e56d0de17be9a60657608f674530d341a0e7139a5a7ef49ead38ec0"
    page_range: [758, 759]
    quote: "analysts following new technology firms have become particularly inventive with multiples that range from value per subscriber for online service providers"
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p770:0998"
    chunk_hash: "3f011f079f9f1fbdd352a09ed9159bbdf8f1b31445ba760820b4831d74390933"
    page_range: [771, 771]
    quote: "Banks, insurance companies, and other financial service firms pose unique challenges for an analyst attempting to value them for two reasons."
    edge_type: "supports"
card_hash: "fe468064163a730011f81900502f40c07f7df7d67203c0e04413e694cb7ba6d2"
---
# Industry and Sector Factor Models

## Intuition

The Fama-French canonical three factors (market, value, size) plus
the quality and low-vol extensions describe cross-sectional return
spread that is invariant to which sector or industry a security
belongs to. But Damodaran Ch.20's sector-specific multiples and
Ch.21's industry-exemplar (financial service firms) discussions
show that significant cross-sectional return spread comes from
SECTOR-SPECIFIC structures: a firm's industry classification
itself carries a return signal that the academic factors do not
capture. **Source:** Damodaran (2012) Ch.20 pp.726-770.

Sector- and industry-factor models treat a security's sector
membership as a categorical exposure — the security has a non-
zero exposure to its sector's factor and zero (or near-zero)
exposure to other sectors. The factor return is the sector-
portfolio return spread vs the broad market: long the sector
portfolio, short the market. Aggregating across sectors produces
a sector-factor model that is parallel to the FF factor model but
indexed by sector rather than by value/size/momentum. **Source:**
Damodaran (2012) Ch.20 pp.726-770.

```
sector-factor exposure structure

security exposure matrix       sector ->
                          tech  fin  energy  cons  health  ...
   |
   |  sec 1 (tech)         1     0     0       0      0
   |  sec 2 (fin)          0     1     0       0      0
   |  sec 3 (energy)       0     0     1       0      0
   |  ...
   v
                  each security's sector factor exposure is 1 for
                  its own sector and 0 for other sectors (in the
                  simplest pure-play case); diversified firms get
                  fractional exposures based on revenue mix

multifactor cost of equity:
r_i  =  Rf
     +  beta_i_M    · ERP
     +  beta_i_HML  · HML_premium
     +  beta_i_SMB  · SMB_premium
     +  sum over sectors of beta_i_sector · sector_premium

  the academic factors capture sector-invariant spread
  the sector factors capture sector-specific spread
```

## Definition

A sector or industry factor is a cross-sectional return source
defined by membership in a specific economic sector. Damodaran
Ch.20's sector-specific-multiples discussion identifies the
cash-flow drivers that distinguish each sector: subscriber
counts for telecom, reserves and production volumes for energy
and mining, gross merchandise volume for marketplace platforms,
annual recurring revenue for software-as-a-service. Each driver
underlies a sector-specific multiple (EV/Subscriber, P/Reserves,
EV/GMV, EV/ARR) that is meaningful only within the sector and
that links cleanly to the sector's economic structure. **Source:**
Damodaran (2012) Ch.20 pp.726-770.

The sector-factor return is constructed as the value-weighted (or
equal-weighted) return on a sector portfolio minus the broad-
market return: `sector_factor_t = R_sector_portfolio_t - R_market_t`.
The portfolio formation uses sector classification (GICS, ICB,
analyst-defined sector codes) to assign each security to one or
more sectors. **Source:** Damodaran (2012) Ch.20 pp.726-770.

A security's sector exposure is the security's sensitivity to its
sector factor, estimated as the regression slope of the security's
excess return on the sector factor's return. For a pure-play firm
the sector exposure is close to one for the firm's own sector and
near zero for other sectors; for a diversified firm the sector
exposures distribute across the segments according to the firm's
revenue mix (the same revenue-share weighting used in bottom-up
beta construction in
[`eq-equity-cost-of-capital-estimation`](./eq-equity-cost-of-capital-estimation.md)).
**Source:** Damodaran (2012) Ch.21 pp.771-807.

The industry-exemplar treatment in Damodaran Ch.21 (Valuing
Financial Service Firms) shows what makes an industry a distinct
factor: financial service firms' debt is a raw material rather
than a financing choice, regulatory capital constraints replace
market-driven capital structure, mark-to-market accounting
distorts trailing earnings, and reinvestment-rate measurement is
not standard. These structural differences are not captured by
the academic factors; they are captured only by an industry-
specific factor or by a separate industry-specific valuation
treatment. **Source:** Damodaran (2012) Ch.21 pp.771-807.

The cross-vertical bridge: the sector-factor framing extends the
multiples-dispersion analysis (see
[`eq-multiples-dispersion`](./eq-multiples-dispersion.md)) by
treating the systematic component of dispersion as a sector signal
rather than as cross-sectional noise. The within-sector dispersion
remains the analyst's diagnostic for security-specific mispricing;
the across-sector dispersion is the sector-factor return source.
The 09 cross-vertical link is `pm-beta-and-factor-exposure.md`,
which covers exposure-based portfolio analytics that consume
sector-factor exposures upstream. **Source:** Damodaran (2012)
Ch.20 pp.726-770.

## Mathematical Reasoning

The sector-factor multifactor cost of equity equation in symbolic
form extends the FF specification by adding sector-factor
contributions. **Source:** Damodaran (2012) Ch.20 pp.726-770.

```
r_i  =  Rf
     +  beta_i_M    · (E[Rm] - Rf)
     +  beta_i_HML  · E[HML_premium]
     +  beta_i_SMB  · E[SMB_premium]
     +  sum over sectors s of (beta_i_s · E[sector_s_premium])

where:
  beta_i_s = regression slope of (R_i - Rf) on sector_s_factor
  E[sector_s_premium] = expected long-run mean of sector_s_factor

  for a pure-play firm in sector s0:
    beta_i_s0 ≈ 1
    beta_i_s ≈ 0 for s ≠ s0

  for a diversified firm:
    beta_i_s = revenue_share_i_s · beta_pure_play_s
    where revenue_share_i_s = firm i's revenue share in sector s
```

The sector-factor return construction in symbolic form computes
the value-weighted sector-portfolio return spread over the market.
**Source:** Damodaran (2012) Ch.20 pp.726-770.

```
sector_s_factor_t = R_sector_s_portfolio_t  -  R_market_t

where:
  R_sector_s_portfolio_t = value-weighted return of all sec j in
                            sector s at time t
                         = sum over j in sector_s of
                             (market_cap_j_t · R_j_t)
                            / sum over j in sector_s of
                              (market_cap_j_t)
  R_market_t = value-weighted return of all securities at time t
```

The sector-factor estimation in symbolic form runs a sector-factor
regression analogous to the FF factor regression. **Source:**
Damodaran (2012) Ch.20 pp.726-770.

```
(R_i_t - Rf_t) = alpha_i
              +  beta_i_M    · (Rm_t - Rf_t)
              +  beta_i_HML  · HML_t
              +  beta_i_SMB  · SMB_t
              +  beta_i_s    · sector_s_factor_t
              +  e_i_t

slope estimates beta_i_s capture the security's exposure to its
sector beyond the market / value / size factor exposures
```

The sector-specific-multiples link from Damodaran Ch.20 in
symbolic form expresses each sector's per-firm valuation in terms
of its sector-specific cash-flow driver. **Source:** Damodaran
(2012) Ch.20 pp.726-770.

```
EV_telecom    = multiple · subscriber_count
EV_energy     = multiple · reserves
EV_software   = multiple · annual_recurring_revenue
P_REIT        = multiple · book_value_or_FFO
EV_marketplace = multiple · gross_merchandise_volume

each sector's multiple is the per-firm cross-sectional median
or mean within that sector, with the dispersion structure inheriting
from eq-multiples-dispersion's sector-conditional form
```

The depth of regression-estimation econometrics for the sector-
factor specification — clustered standard errors, panel-vs-cross-
section choice, time-varying sector weights — is deferred to
future-01 per DEC-1. The L1-extension-depth statement is: a
multifactor cost-of-equity model extends with as many additional
factors as the analyst's investment process can support; sector
factors are one canonical extension class. **Source:** Damodaran
(2012) Ch.20 pp.726-770.

## See Also

- [`eq-multiples-dispersion`](./eq-multiples-dispersion.md) — the within-sector dispersion that the sector-factor model treats as the diagnostic baseline
- [`eq-fama-french-construction-at-security-level`](./eq-fama-french-construction-at-security-level.md) — the FF three-factor framework the sector-factor model extends
- [`eq-pb-and-multiples-taxonomy`](./eq-pb-and-multiples-taxonomy.md) — the multiples taxonomy that includes the sector-specific multiples (EV/Subscriber, P/Reserves, EV/ARR) anchoring this card
- [`pm-beta-and-factor-exposure`](../09_portfolio_management_and_asset_pricing/pm-beta-and-factor-exposure.md) — exposure-based portfolio analytics upstream in 09

## Escalate to Raw When

Open Damodaran Ch.20 / Ch.21 directly when any of the criteria
below applies. **Source:** Damodaran (2012) Ch.20 pp.726-770.

- the target firm operates in a sector requiring a specialized cash-flow driver (banking, insurance, telecom, energy / mining, software ARR, REIT FFO) and the analyst needs the per-sector valuation framework — Damodaran Ch.20 covers the sector-specific multiples; Damodaran Ch.21 develops the financial-service-firms exemplar in detail. **Source:** Damodaran (2012) Ch.20 pp.726-770.
- the sector-factor specification is contested (sector classification choice, value-weighted vs equal-weighted construction, time-varying sector weights) — the deeper estimation machinery is in future-01 Quantitative Methods per DEC-1, with cross-references back to Damodaran Ch.20. **Source:** Damodaran (2012) Ch.20 pp.726-770.
- the firm is a regulated financial-service firm where the standard valuation machinery breaks down — Damodaran Ch.21 develops the per-firm-class adjustments (banks, insurance companies, asset managers, brokerage firms) at intuition depth. **Source:** Damodaran (2012) Ch.21 pp.771-807.
