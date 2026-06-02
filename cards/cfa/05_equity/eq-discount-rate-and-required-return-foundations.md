---
schema_version: "cacg.v0"
id: "eq-discount-rate-and-required-return-foundations"
title: "Discount Rate and Required Return Foundations"
reading_id: "05_equity"
summary: "The discount rate is the return investors require to bear an asset's risk: a riskless rate plus a risk premium. CAPM operationalizes the premium as beta times the equity risk premium. Damodaran develops the foundation in Ch.4 and extends it through Ch.7 (riskless rate, ERP) and Ch.8 (cost-of-equity estimation)."
tags: ["equity", "discount-rate"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p120:0121"
    chunk_hash: "4a1e2a039febae6c1aa9cd1f7f0c933d507b365a7d4c3600d8867a6366aa4b83"
    page_range: [120, 120]
    quote: "When valuing assets and firms, we need to use discount rates that reflect the riskiness of the cash flows."
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p246:0264"
    chunk_hash: "78f7dcca490700d47e73469a531c7b525e26c305f65878423ba0deb8e9a186d2"
    page_range: [246, 247]
    quote: "The expected returns on risky investments are then measured relative to the risk-free rate, with the risk creating an expected risk premium that is added to the risk-free rate."
    edge_type: "supports"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p279:0310"
    chunk_hash: "1340259d891857cee0ab25f9560b9ced2fef8d8dab890cc7c80c8608544bbb99"
    page_range: [279, 279]
    quote: "If we consider all the financing that the firm takes on, the composite cost of financing will be a weighted average of the costs of equity and debt, and this weighted cost is the cost of capital."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2309:3384"
    chunk_hash: "3fd5b07bea2c5c6f31a71b7ecd95eb69f208c2ed854103f33a2b9bc965921ff6"
    page_range: [2309, 2310]
    quote: "To estimate the required rate of return on a share, analysts frequently use the capital asset pricing model (CAPM)"
    edge_type: "supports"
card_hash: "922e430780321ca72b08161290a7ff98b30385374587941835d984c7168dbc13"
---
# Discount Rate and Required Return Foundations

## Intuition

The discount rate `r` in an intrinsic-value model is not an external
market quote — it is the return an investor REQUIRES to bear the risk
of the asset's cash flows. A riskier cash-flow stream demands a higher
required return; a less risky stream demands less. The required
return is what the analyst supplies as the price of bearing the
asset's specific risk profile. **Source:** Damodaran (2012) Ch.4
pp.120-155.

The required return decomposes into two components: a riskless rate
that compensates for the time value of money under no risk, and a
risk premium that compensates for bearing the asset's risk. Different
asset-pricing models (CAPM, APT, multifactor) differ in HOW they
decompose the risk premium, but they share the additive structure.
**Source:** Damodaran (2012) Ch.4 pp.120-155.

```
required return r
   ^
   |  +-----------------+
   |  |  risk premium   |    <-- compensation for asset risk
   |  |  (model-specific |        (CAPM: beta * (Rm - Rf);
   |  |   decomposition) |         APT/multifactor: sum
   |  |                  |         of factor-beta * factor-
   |  |                  |         premium contributions)
   |  +-----------------+
   |  |  riskless rate  |    <-- compensation for time
   |  |  Rf             |        value of money under no risk
   |  +-----------------+
   |
   +-> r = Rf + risk premium
```

## Definition

The riskless rate `Rf` is the return on an asset that delivers its
promised cash flow with certainty. In practice, default-risk-free
government bonds (matched to the cash-flow horizon) supply the
riskless-rate proxy; the riskless rate carries the time-horizon
convention of the cash-flow being discounted. **Source:** Damodaran
(2012) Ch.7 pp.246-278.

The equity risk premium `ERP = E[Rm] - Rf` is the excess return the
market portfolio is expected to earn over the riskless rate; it is
the per-unit price of equity-market risk. ERP is estimated from
historical excess returns, from market-implied inversions of current
prices (see
[`eq-implied-cost-of-capital-from-market-prices`](./eq-implied-cost-of-capital-from-market-prices.md)),
and from country-risk-premium adjustments when the relevant equity
market is not a mature default-risk-free benchmark. Historical and
implied estimates can differ substantially, so the analyst must align
the ERP choice with the risk-free-rate convention and valuation
mission. **Source:** Damodaran (2012) Ch.7 pp.246-278.

The Capital Asset Pricing Model (CAPM) is Damodaran's foundational
single-factor decomposition of the risk premium: an asset's required
return equals the riskless rate plus the asset's beta times the equity
risk premium. Beta measures the asset's exposure to market risk —
the only risk that is not diversifiable in CAPM's framing. **Source:**
Damodaran (2012) Ch.4 pp.120-155.

## Mathematical Reasoning

The CAPM equation expresses the required return on an asset as a
linear function of its market-beta exposure: `r_i = Rf + beta_i ·
(E[Rm] - Rf)`. The slope `(E[Rm] - Rf)` is the equity risk premium.
**Source:** Damodaran (2012) Ch.4 pp.120-155.

Beta itself is the covariance of the asset's return with the market
portfolio's return divided by the market portfolio's variance:
`beta_i = Cov(R_i, Rm) / Var(Rm)`. By construction the market
portfolio has beta one and the riskless asset has beta zero.
**Source:** Damodaran (2012) Ch.8 pp.279-332.

For estimation, beta is recoverable from a regression of the asset's
historical excess returns on the market's historical excess returns;
the regression slope is beta. Alternatively, bottom-up beta builds
the asset's beta from the betas of its business segments weighted by
revenue or value contribution — useful when the firm's history does
not represent its forward-looking business mix. **Source:** Damodaran
(2012) Ch.8 pp.279-332.

CAPM is one of several equity-risk models. Multifactor models such as
Arbitrage Pricing Theory (APT) and the Fama-French three-factor model
generalize the single-beta decomposition to a sum of factor-beta-
times-factor-premium contributions. The multifactor framing is
covered upstream in 09 (see
[`pm-multifactor-asset-pricing-intuition.md`
](../09_portfolio_management_and_asset_pricing/pm-multifactor-asset-pricing-intuition.md));
05's security-level construction of value / momentum / quality /
low-vol factor scores is covered in
[`eq-fama-french-construction-at-security-level`](./eq-fama-french-construction-at-security-level.md),
[`eq-value-and-momentum-factor-scoring`](./eq-value-and-momentum-factor-scoring.md),
and
[`eq-quality-and-low-vol-factor-scoring`](./eq-quality-and-low-vol-factor-scoring.md).
**Source:** Damodaran (2012) Ch.4 pp.120-155.

The CFA L1 frame at intuition depth presents required return as a
sum of nominal-riskless rate plus an equity-risk premium; the
elaborated CAPM derivation lives in CFA Vol.6 (Portfolio Management),
which 05 cites only via 09 cross-references. **Source:** CFA L1
Curriculum (2022) Vol.4/pp.361-416.

## See Also

- [`eq-intrinsic-value`](./eq-intrinsic-value.md) — the consumer of `r` in `V_0 = sum CF_i / (1 + r)^i`
- [`eq-dividend-discount-models`](./eq-dividend-discount-models.md) — DDM uses cost of equity as the discount rate
- [`pm-capm-and-sml`](../09_portfolio_management_and_asset_pricing/pm-capm-and-sml.md) — CAPM/SML derivation upstream in 09
- [`pm-beta-and-factor-exposure`](../09_portfolio_management_and_asset_pricing/pm-beta-and-factor-exposure.md) — beta and factor exposure upstream in 09

## Escalate to Raw When

Open Damodaran Ch.4 / Ch.7 / Ch.8 directly when any of the criteria
below applies. **Source:** Damodaran (2012) Ch.4 pp.120-155.

- the riskless-rate proxy is non-obvious (emerging market, distressed sovereign, hard-currency mismatch with cash-flow currency) — Damodaran Ch.7 develops the country-risk-premium adjustment. **Source:** Damodaran (2012) Ch.7 pp.246-278.
- bottom-up beta is needed (the firm has changed its business mix or has insufficient trading history) — Damodaran Ch.8 develops the segment-weighted construction in detail. **Source:** Damodaran (2012) Ch.8 pp.279-332.
- a multifactor model (Fama-French, APT) is required and the security-level factor construction is the focus — see [`eq-fama-french-construction-at-security-level`](./eq-fama-french-construction-at-security-level.md) and the upstream theory cards in 09. **Source:** Damodaran (2012) Ch.4 pp.120-155.
