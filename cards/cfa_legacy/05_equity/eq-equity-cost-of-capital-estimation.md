---
schema_version: "cacg.v0"
id: "eq-equity-cost-of-capital-estimation"
title: "Equity Cost of Capital Estimation"
reading_id: "05_equity"
summary: "Cost-of-equity estimation under CAPM is a three-input procedure: riskless-rate proxy, beta estimate, and equity risk premium. Beta is sourced top-down (regression) or bottom-up (segment-weighted unlevered betas relevered to target capital structure). Damodaran Ch.8 develops the estimation discipline; Ch.7 supplies the Rf and ERP foundations."
tags: ["equity", "equity-cost"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p279:0311"
    chunk_hash: "ff8cc7e111392b70e23b8ac38f664b7ac4b216594b6b7f38c0ff5ee6cac532ac"
    page_range: [279, 280]
    quote: "There are three approaches available for estimating these parameters: One is to use historical data on market prices for individual investments"
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p281:0313"
    chunk_hash: "25859a1dc78dd2340a244ae60a20ab310754ff203120166a717be83903d967ed"
    page_range: [281, 282]
    quote: "The slope of the regression, like any statistical estimate, may be different from the true value, and the standard error reveals just how much error there could be in the estimate."
    edge_type: "supports"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p247:0266"
    chunk_hash: "0ecf33a02fab3a2f7a7c5cbcabc491a7ba571e0c34430c08b59d6a34b0befa2e"
    page_range: [247, 247]
    quote: "When doing investment analysis on longer-term projects or valuations, the risk-free rate should be the long-term government bond rate."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2309:3384"
    chunk_hash: "3fd5b07bea2c5c6f31a71b7ecd95eb69f208c2ed854103f33a2b9bc965921ff6"
    page_range: [2309, 2310]
    quote: "Equation 5 states that the required rate of return on a share is the sum of the current expected risk-free rate plus a risk premium that equals the product of the stock’s beta"
    edge_type: "supports"
card_hash: "d4da5b93288f860dfbe81ce7d77a3c86535318a3e9e18e43569676c1ebb97f54"
---
# Equity Cost of Capital Estimation

## Intuition

Estimating a firm's cost of equity is not a single calculation —
it is a sequence of input choices, each of which the analyst must
defend. CAPM says `r_e = Rf + beta · ERP`; that equation hides three
input questions: which riskless rate proxy to use, which beta
estimate to use, and which equity risk premium estimate to use. The
answers depend on the firm being valued, the cash-flow horizon
being discounted, and the analyst's mission. **Source:** Damodaran
(2012) Ch.8 pp.279-332.

The L1-process discipline is to make each input choice explicit and
internally consistent: match the riskless rate to the cash-flow
currency and tenor; use a beta that reflects the firm's forward-
looking business mix (top-down regression beta only when the firm
is stable, bottom-up beta otherwise); use an ERP that is consistent
with the riskless rate's currency and the analyst's historical-vs-
implied estimation choice. Failure to align these inputs produces a
cost of equity that is internally inconsistent — for example,
discounting USD cash flows with a USD riskless rate but pairing
with a global-historical-USD ERP estimated against a different
riskless-rate proxy. **Source:** Damodaran (2012) Ch.8 pp.279-332.

```
r_e = Rf + beta · ERP

  +-- Rf source -----+----- equity-claim cash-flow horizon
  |                  |      and currency match
  |                  |
  |  10-year govt   |  matched-tenor govt yield in CF currency
  |  (mature mkt)   |  (default-risk-free; CRP add-on if not)
  |
  +-- beta source ---+----- firm-specific risk exposure
  |                  |
  |  regression     |  history >= 5y; stable business
  |  bottom-up      |  segment-weighted; new firm or
  |                  |  changing-mix firm
  |  factor-implied |  multifactor (deferred to extension cards)
  |
  +-- ERP source ---+----- per-unit market-risk price
                    |
                    historical excess return,
                    implied-from-prices,
                    country-risk-premium add-on
```

## Definition

The cost of equity for a single firm at L1 depth is the CAPM-form
required return: `r_e = Rf + beta · ERP`, where `Rf` is a riskless
rate proxy matched to the cash-flow horizon and currency, `beta` is
the firm's exposure to systematic market risk, and `ERP` is the
expected excess return of the equity market over the riskless rate.
The estimation is a three-input procedure where each input has its
own sourcing methodology. **Source:** Damodaran (2012) Ch.8
pp.279-332.

The riskless rate `Rf` proxy is conventionally a default-risk-free
government bond yield matched to the cash-flow currency and tenor.
For mature-market USD cash flows, the 10-year U.S. Treasury yield
is the standard proxy. For non-mature-market currencies, the
analyst either uses the local default-risk-free yield (rare —
emerging-market sovereigns are not default-risk-free) or the U.S.
yield with a country-risk-premium adjustment that captures the
incremental sovereign risk. The matching tenor convention applies
because long-horizon equity cash flows reflect long-horizon
inflation expectations. **Source:** Damodaran (2012) Ch.7
pp.246-278.

Beta estimation has two main routes: top-down (regression) beta and
bottom-up beta. Top-down beta regresses the firm's historical
excess returns on the market's historical excess returns; the
slope is the regression beta. The estimate is statistically clean
when the firm has a long history (5+ years) of stable business
mix. Bottom-up beta computes the firm's beta as the value-weighted
average of the betas of pure-play firms in each of the firm's
business segments, then unlevers and relevers to the firm's target
capital structure. Bottom-up is preferred when the firm has changed
its business mix, has insufficient trading history, or operates in
multiple distinct industries. **Source:** Damodaran (2012) Ch.8
pp.279-332.

The ERP estimate has three main approaches: historical (average
excess return of the equity market over the riskless rate over a
long sample); implied (back out from current prices and forecast
cash flows — see
[`eq-implied-cost-of-capital-foundations`](./eq-implied-cost-of-capital-foundations.md));
country-risk-premium (CRP) adjustment for non-mature markets where
neither historical nor implied estimates from a default-risk-free
benchmark are available. The choice of approach depends on whether
the analyst is estimating a steady-state ERP (historical) or a
current-conditions ERP (implied), and the choice must be consistent
with the riskless-rate sourcing. **Source:** Damodaran (2012) Ch.7
pp.246-278.

The internal consistency rule pairs the inputs: USD cash flows
discounted at a U.S. cost of equity should pair a U.S. Treasury
riskless rate with a U.S.-equity-market-derived ERP and a beta
referenced to a U.S. market-portfolio proxy. Mixing across currencies
or markets requires explicit translation (currency forward parity
on cash flows; CRP add-ons for emerging-market exposure). **Source:**
Damodaran (2012) Ch.8 pp.279-332.

## Mathematical Reasoning

The CAPM cost-of-equity equation in symbolic form expresses the
required return as a linear combination of the riskless rate and a
beta-weighted equity risk premium. **Source:** Damodaran (2012)
Ch.8 pp.279-332.

```
r_e  =  Rf  +  beta · ERP

  where ERP  =  E[Rm]  -  Rf
        beta  =  Cov(R_firm, Rm)  /  Var(Rm)
```

Top-down beta regresses historical excess returns of the firm on
historical excess returns of the market: `(R_firm - Rf) = alpha +
beta · (Rm - Rf) + e`. The regression slope is the beta estimate;
the standard error of the slope quantifies estimation noise; the
R-squared captures the share of firm return variance explained by
market exposure. Damodaran's recommendation is at least 5 years of
weekly data for the regression sample to balance estimation
precision against business-mix-change risk. **Source:** Damodaran
(2012) Ch.8 pp.279-332.

Bottom-up beta construction in symbolic form unlevers per-segment
pure-play betas, value-weights them, and relevers to the firm's
target capital structure. **Source:** Damodaran (2012) Ch.8
pp.279-332.

```
unlever each peer's regression beta by removing the peer's
financial-leverage effect (Hamada-style):

  beta_unlevered_peer_k = beta_levered_peer_k
                       / (1 + (1 - t) · (D/E)_peer_k)

aggregate to industry / segment unlevered beta (median or
revenue-weighted across peers in the industry):

  beta_unlevered_industry_i = aggregate_over_k(beta_unlevered_peer_k)

aggregate firm unlevered beta across the firm's business segments
(value- or revenue-weighted):

  beta_unlevered_firm = sum over segments of
                        (revenue_share_i · beta_unlevered_industry_i)

relever to the firm's target capital structure:

  beta_relevered_firm = beta_unlevered_firm
                      · (1 + (1 - t) · (D/E)_target_firm)
```

The unlevering step strips the financial-leverage component from
the pure-play firms' regression betas; the value-weighted (or
revenue-weighted) average aggregates the firm's segment betas; the
relevering step reapplies the firm's target capital structure.
Bottom-up betas are typically more stable than regression betas
because the segment weights are observable today rather than
estimated from a 5-year window. **Source:** Damodaran (2012) Ch.8
pp.279-332.

The historical ERP is the time-series average of `(Rm_t - Rf_t)`
over a long sample. Damodaran's preferences: long sample (since
1928 for U.S. data) for noise reduction; geometric mean for long-
horizon valuation (matches the compounding of cash flows over the
horizon); arithmetic mean only when the analyst is estimating a
one-period unbiased mean under uncorrelated returns. The implied
ERP is the difference between the implied cost of equity (recovered
by inversion of the DDM/DCF, see
[`eq-implied-cost-of-capital-foundations`](./eq-implied-cost-of-capital-foundations.md))
and the riskless rate. The CRP add-on supplements the mature-market
ERP for non-mature-market exposures. **Source:** Damodaran (2012)
Ch.7 pp.246-278.

The CFA L1 frame presents CAPM as the canonical cost-of-equity
formula, identifies the three inputs (Rf, beta, ERP) as the
required estimates, and emphasizes internal consistency across the
inputs. The bottom-up beta construction is covered as a "when
to use" alternative. **Source:** CFA L1 Curriculum (2022)
Vol.4/pp.361-416.

## See Also

- [`eq-discount-rate-and-required-return-foundations`](./eq-discount-rate-and-required-return-foundations.md) — the foundational CAPM frame the estimation operationalizes
- [`eq-equity-risk-premium-intuition`](./eq-equity-risk-premium-intuition.md) — the ERP estimation intuition (historical vs implied vs country-risk-premium)
- [`eq-implied-cost-of-capital-foundations`](./eq-implied-cost-of-capital-foundations.md) — the implied-cost-of-equity inversion that supplies the implied ERP
- [`pm-beta-and-factor-exposure`](../09_portfolio_management_and_asset_pricing/pm-beta-and-factor-exposure.md) — beta and factor exposure upstream in 09

## Escalate to Raw When

Open Damodaran Ch.7 / Ch.8 directly when any of the criteria below
applies. **Source:** Damodaran (2012) Ch.8 pp.279-332.

- the riskless-rate proxy is non-obvious (emerging market, distressed sovereign, hard-currency mismatch with cash-flow currency) — Damodaran Ch.7 develops the country-risk-premium adjustment in detail. **Source:** Damodaran (2012) Ch.7 pp.246-278.
- the firm has changed its business mix or has insufficient trading history, and the bottom-up beta construction needs the segment-weighted methodology in detail — Damodaran Ch.8 develops the bottom-up procedure. **Source:** Damodaran (2012) Ch.8 pp.279-332.
- the ERP source choice (historical vs implied vs CRP) materially affects the cost-of-equity estimate and the analyst needs the comparison framework — Damodaran Ch.7 surveys the trade-offs. **Source:** Damodaran (2012) Ch.7 pp.246-278.
