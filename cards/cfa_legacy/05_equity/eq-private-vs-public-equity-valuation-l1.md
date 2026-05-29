---
schema_version: "cacg.v0"
id: "eq-private-vs-public-equity-valuation-l1"
title: "Private vs Public Equity Valuation — L1"
reading_id: "05_equity"
summary: "Framing private-firm valuation at L1 depth: the same DCF/multiples machinery applies but four practical differences (no observed price, no trading history, looser accounting, undiversified owner) require bottom-up beta construction and an illiquidity discount on the public-firm-equivalent intrinsic value. The valuation motive (private sale, public sale, IPO) drives the discount size."
tags: ["equity", "private-public"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p870:1158"
    chunk_hash: "38577c560b24d21d2aea13dc330c1f6c0f84a9c8048bcf426d607fa9ca3294b5"
    page_range: [870, 870]
    quote: "WHAT MAKES PRIVATE FIRMS DIFFERENT? There are a number of common characteristics shared by private firms with publicly traded firms, but there are four significant differences"
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p297:0332"
    chunk_hash: "617251267b4eee8e1b1db12a346d3b70011f62618e8e8c5cb7b5c84f94bd74c4"
    page_range: [297, 298]
    quote: "bottom-up betas can be estimated for private firms, divisions of businesses, and stocks that have just started trading in financial markets."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2219:3255"
    chunk_hash: "786cd6f4e2902dccb80ade7a4817adbea51331e65ee3089b3aed7cfd92598c64"
    page_range: [2219, 2220]
    quote: "Private equity securities are issued primarily to institutional investors via non-public offerings, such as private placements."
    edge_type: "supports"
card_hash: "e1e3d94353bd94de5192880432ede91503d4226c22f64872f8db6f1f343326a6"
---
# Private vs Public Equity Valuation — L1

## Intuition

A private firm's intrinsic value follows the same DCF / multiples
machinery as a public firm — present-value expected cash flows,
benchmark against peer multiples — but four practical differences
make private valuation harder. The firm has no observed market
price, no continuously updated trading history, looser accounting
standards, and an owner whose wealth is concentrated in the firm
rather than diversified. Each of these affects either the cash-flow
forecast or the discount rate. **Source:** Damodaran (2012) Ch.24
pp.870-906.

The single most distinctive feature is illiquidity. A buyer of a
private equity stake cannot sell quickly into a continuous
secondary market; the only exits are negotiated sales, IPO, or
holding to maturity. Damodaran's intuition is that this absence of
liquid exit imposes a cost on the buyer that should be reflected
either as a higher discount rate or as a discount applied directly
to the intrinsic value computed under public-firm assumptions. The
illiquidity discount is the bridge between the public-firm
valuation methodology and the private-firm answer. **Source:**
Damodaran (2012) Ch.24 pp.870-906.

```
public-firm valuation     private-firm valuation
   |                          |
   |  observed price          |  no observed price
   |  liquid market exit      |  illiquid: only negotiated sale,
   |  continuous data         |  IPO, or hold-to-maturity exit
   |  diversified shareholders|  concentrated owner; no diversification
   |  GAAP / SEC standards    |  looser accounting standards
   |
   v                          v
  intrinsic value V_public  intrinsic value V_public
                              minus illiquidity discount
                              = V_private
                              ^
                              |
                              size of discount depends on
                              valuation motive:
                              - sale to private buyer
                              - sale to public firm
                              - IPO
```

## Definition

Private-firm valuation is the family of methodologies for
estimating the intrinsic value of equity in a firm whose stock is
not continuously traded on a public exchange. The same DCF and
multiples engines apply, but several inputs are adjusted for
private-firm characteristics: beta is constructed bottom-up (no
historical regression beta available), the cost of equity may be
adjusted for owner-concentration risk, and the resulting intrinsic
value is reduced by an illiquidity discount that reflects the cost
of holding an equity claim that cannot be readily sold. **Source:**
Damodaran (2012) Ch.24 pp.870-906.

The illiquidity discount is a markdown applied to the public-firm-
equivalent intrinsic value of a private firm. Damodaran's intuition-
level estimate is in the 20-30% range for typical private firms,
varying with firm size, profitability, business stability, and the
specific motive for the valuation. The discount captures the
investor's loss from holding an asset that cannot be quickly
liquidated; in market-based estimates, it is calibrated against
the observed discounts on restricted-stock issues by public firms
(restricted shares are publicly issued but cannot be sold for a
specified holding period, making them a partial proxy for the
private-equity illiquidity). **Source:** Damodaran (2012) Ch.24
pp.870-906.

The valuation motive matters because each motive identifies a
different buyer with a different ability to mitigate illiquidity.
A sale to another private buyer carries the full illiquidity
discount because the new owner inherits the same illiquidity
problem. A sale to a publicly traded firm via acquisition reduces
or eliminates the illiquidity discount because the acquirer can
absorb the asset into its own publicly traded equity (the seller
receives liquid public shares or cash). An initial public offering
also reduces the discount, with the residual reflecting the IPO-
period restriction and the post-IPO trading float. **Source:**
Damodaran (2012) Ch.24 pp.870-906.

The owner-concentration adjustment is a second L1 distinction. The
owner of a private firm typically holds a large fraction of personal
wealth in the firm, so the relevant risk measure is total risk (not
just systematic-market-risk beta). For a sale to a private buyer
who will also be undiversified, the cost of equity may be adjusted
upward to reflect total-risk pricing. For a sale to a publicly
traded firm or to a diversified institutional buyer, the standard
CAPM beta-based cost of equity applies because the buyer can
diversify away the firm-specific component. **Source:** Damodaran
(2012) Ch.24 pp.870-906.

## Mathematical Reasoning

The private-firm intrinsic value at L1 depth in symbolic form
applies the standard DCF mechanics to private-firm cash flows and
discounts at the appropriate cost of equity, then subtracts an
illiquidity discount. **Source:** Damodaran (2012) Ch.24 pp.870-906.

```
V_public_equivalent = sum_{i=1..N} CF_i / (1 + r_e)^i
                    + TV_N / (1 + r_e)^N

V_private = V_public_equivalent · (1 - illiquidity_discount)
```

The cost of equity `r_e` for a private firm uses the same CAPM form
as a public firm but with bottom-up beta because no regression beta
is available. The bottom-up construction inherits from
[`eq-equity-cost-of-capital-estimation`](./eq-equity-cost-of-capital-estimation.md):
take the average pure-play beta of public firms in the same
industry, unlever to remove the public peers' financial-leverage
component, relever to the private firm's target capital structure.
**Source:** Damodaran (2012) Ch.8 pp.279-332.

The total-risk adjustment for an undiversified-buyer scenario
modifies the cost of equity by replacing the systematic-risk beta
with a total-risk measure. Damodaran's intuition: the adjustment
factor is `sqrt(1 / R-squared)` where R-squared is the proportion
of return variance explained by market exposure. A firm with low
R-squared (high firm-specific risk) gets a large adjustment because
the undiversified buyer bears the unsystematic risk that a
diversified buyer would diversify away. The adjusted cost of equity
is `r_e_total = Rf + (beta / sqrt(R-squared)) · ERP`. **Source:**
Damodaran (2012) Ch.24 pp.870-906.

The illiquidity discount in the L1 frame is calibrated against
restricted-stock studies. The intuition: restricted-stock issues
trade at observed discounts to the unrestricted-share price during
the restriction period; those observed discounts (typically 20-30%)
are a market-derived estimate of the cost of holding a public
equity that cannot be quickly sold. The private-firm equivalent
applies because the holding-period restriction is structurally
similar, with adjustments for firm size and profitability (smaller,
less-profitable firms get larger discounts). The empirical
calibration depth is in Damodaran Ch.24; the L1 takeaway is the
20-30% range and the structural rationale. **Source:** Damodaran
(2012) Ch.24 pp.870-906.

The private-vs-public bridge in the matched-assumption identity:
under a hypothetical exit-to-public-market scenario at horizon `T`,
the private-firm value at `T` is the public-firm-equivalent value
without the illiquidity discount. Today's private value is then
the discounted expected future public-firm-equivalent value minus
the present-value cost of bearing illiquidity from now to `T`.
This formulation makes the illiquidity discount horizon-dependent:
shorter expected exit horizons reduce the discount (less time
holding the illiquid asset). **Source:** Damodaran (2012) Ch.24
pp.870-906.

The CFA L1 frame presents private-firm valuation as a special case
of equity valuation distinguished by the absence of an observed
market price, the need for bottom-up beta, and the illiquidity
discount. The depth of restricted-stock empirical calibration and
the multi-stage exit-horizon adjustments are at higher CFA levels.
**Source:** CFA L1 Curriculum (2022) Vol.4/pp.271-306.

## See Also

- [`eq-dcf-mechanics`](./eq-dcf-mechanics.md) — the DCF engine that supplies V_public_equivalent
- [`eq-comparable-company-analysis`](./eq-comparable-company-analysis.md) — the public-peer benchmark used for cost-of-equity inputs and multiples comparisons
- [`eq-equity-cost-of-capital-estimation`](./eq-equity-cost-of-capital-estimation.md) — the bottom-up beta construction the private firm relies on

## Escalate to Raw When

Open Damodaran Ch.24 directly when any of the criteria below applies.
**Source:** Damodaran (2012) Ch.24 pp.870-906.

- the illiquidity discount estimate is contested and the analyst needs the full restricted-stock empirical calibration framework — Damodaran Ch.24 surveys the studies in detail. **Source:** Damodaran (2012) Ch.24 pp.870-906.
- the private firm has unusual characteristics (single-customer concentration, regulated regime, key-person risk concentrated in the owner) that interact with the standard discount and adjustment framework — Damodaran Ch.24 develops the per-feature adjustments. **Source:** Damodaran (2012) Ch.24 pp.870-906.
- the buyer is undiversified and the total-risk adjustment to the cost of equity is the analytical pivot — Damodaran Ch.24 develops the R-squared-scaling adjustment in detail. **Source:** Damodaran (2012) Ch.24 pp.870-906.
