---
schema_version: "cacg.v0"
id: "ec-rental-rate-of-capital-microeconomic-foundation"
title: "Rental Rate of Capital — Microeconomic Foundation"
reading_id: "02_economics"
summary: "The rental rate of capital from production theory: the firm's static FOC equates value of marginal product to gross rental price (p*MPK = R), giving the net rental rate r = MPK - delta after depreciation; Romer's user-cost extension r_K = (r + delta - p_K-dot/p_K) * p_K folds in interest, depreciation, and capital-price changes."
tags: ["economics", "rental-rate"]
citations:
  - source_id: "econ_mwg_1995_microeconomic_theory"
    chunk_id: "econ_mwg_1995_microeconomic_theory:p159:0258"
    chunk_hash: "38191583c017e58b19268da77fa69cc0d2d4db8b983d2c456039021318a0494a"
    page_range: [159, 160]
    quote: "Replace f( ·) by u( · ), q by u, and z by x (i.e., interpret the production function as a utility function), and the CMP becomes the expenditure minimization problem (EMP)"
    edge_type: "defines"
  - source_id: "econ_romer_2019_advanced_macro_5ed"
    chunk_id: "econ_romer_2019_advanced_macro_5ed:p440:0628"
    chunk_hash: "8cbbea173bda941a0981076ff86d735dd2101adb221260d635b3389f3603335e"
    page_range: [440, 441]
    quote: "the firm rents capital up to the point where its marginal revenue product equals its rental price."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p644:0888"
    chunk_hash: "4cf439f1039fa77e88c345737b3b07a02b9418d91977e0c4c0358df1f9c0a7fc"
    page_range: [644, 645]
    quote: "Perfect competition Many Homogeneous/ Standardized Very Low None None Monopolistic competition Many Differentiated Low Some Advertising and Product Differentiation"
    edge_type: "supports"
card_hash: "49a7f96dcd0936ac9c62d8c34b5a28fbe6025a66bd24a6e89b825aa7899ca4bc"
---
# Rental Rate of Capital — Microeconomic Foundation

## Intuition

When a competitive firm hires capital in a one-period model, the price it pays per unit of capital is the **rental rate** — call the gross-of-depreciation rental rate `R = r + δ`. MWG Ch.5 gives the firm's static profit-max FOC `p · MPK = R` (value of marginal product equals the gross rental price of capital). The depreciation-adjusted **net rental rate** `r = MPK − δ` (assuming `p = 1` for the macro normalization) comes from rearranging the FOC: at the optimum `MPK = R = r + δ`, hence `r = MPK − δ`. The firm hires capital up to the point where the gross rental rate equals the marginal product, equivalently the net rental rate equals the marginal product minus depreciation. **Source:** Mas-Colell et al. (1995) Ch.5 pp.135-152.

The depreciation-net framing `r = MPK − δ` is the canonical macro convention used by the Solow / Ramsey / Romer growth models in Batch 1; MWG's static one-period framework gives the gross FOC, and Romer Ch.9 (Investment) extends the framework to dynamic adjustment-cost investment where depreciation appears explicitly in the user-cost expression. **Source:** Romer (2019) Ch.9 pp.420-440.

This is a bridge card per resolved DEC-4. The rental rate from production theory is microeconomic; it is the foundation that: **Source:** Mas-Colell et al. (1995) pp.135-165.
- the Solow growth model uses to characterize the steady-state capital-output ratio,. **Source:** Mas-Colell et al. (1995) pp.135-165.
- the firm's investment decision (sibling `ec-investment-cost-of-capital`) extends to a dynamic adjustment-cost setting,. **Source:** Mas-Colell et al. (1995) pp.135-165.
- the corporate-finance cost-of-capital frameworks (in 05) build on by adding tax adjustments, debt-financing structures, and the equity-vs-debt WACC weighting. **Source:** Mas-Colell et al. (1995) pp.135-165.

The card stops at the microeconomic foundation: rental rate from MPK and depreciation. The WACC mechanics (weights, tax shields, leverage adjustments) stay in 05 per AC-11 BOUNDARY-DISCIPLINE; the CAPM-derived cost of equity stays in 09. **Source:** Mas-Colell et al. (1995) Ch.5 pp.149-160.

```
   marginal value of capital (= p · MPK)
   ^
   |
   |   .
   |     .                       horizontal line: rental rate r
   |       .   .   .   .   .   .   .   .   .   . r
   |          .
   |             .
   |               .         (decreasing MPK as K rises — diminishing
   |                  .       returns to capital under DRS or interior CRS)
   +-------------+------+-------------+----> K (capital)
                       K*

   firm hires capital up to where p · MPK = r
   user-cost identity: r = MPK − δ        (one-period frictionless)
```

## Definition

In the static one-period model with output price `p` (normalized to 1 in the macro framing) and frictionless capital markets, the firm's profit-maximization first-order condition for capital is `p · MPK = R` where `R = r + δ` is the **gross rental rate of capital** (the per-period price the firm pays to rent one unit of physical capital). Equivalently, the **net rental rate** is `r = R − δ`, the rate of return after the per-period depreciation `δ ∈ [0, 1]` is subtracted off: **Source:** Mas-Colell et al. (1995) pp.135-165.

```
gross rental rate:  R = MPK         (firm FOC; p normalized to 1)
net rental rate:    r = MPK − δ     (gross minus depreciation)
```

where `MPK = ∂f / ∂K` is the marginal product of capital and `δ` is the share of capital that physically wears out each period. **Source:** Mas-Colell et al. (1995) Ch.5 pp.135-152.

The depreciation-net identity `r = MPK − δ` is the convention adopted by Solow / Ramsey / Romer growth models in Batch 1. **Source:** Romer (2019) Ch.9 pp.420-440.

For a Cobb-Douglas production function `Y = K^α · L^(1-α)`,. **Source:** Mas-Colell et al. (1995) pp.135-165.

```
MPK = α · Y / K        (capital's marginal product)
R = α · Y / K          (gross rental rate)
r = α · Y / K  −  δ    (net rental rate)
```

The **gross capital share** `R · K / Y = α` is constant for Cobb-Douglas; the **net capital share** `r · K / Y = α − δ · K / Y` adjusts for depreciation. The gross capital share equals the Cobb-Douglas exponent `α` and is the empirical anchor for measuring `α` in growth accounting. **Source:** Mas-Colell et al. (1995) Ch.5 pp.135-152.

The Solow steady-state framework uses the gross rental rate `R = r + δ` to characterize the balanced growth path: at the steady state, the marginal product of capital equals the sum of the net real interest rate and the depreciation rate, `α · y/k = r* + δ`, where in Solow's exogenous-savings framework `r*` is pinned down by the saving rate via the steady-state capital-output ratio `k/y = s / (n + g + δ)` (substituting yields `r* + δ = α · (n + g + δ) / s`); Solow's `r*` is therefore determined by `s` rather than by any time-discount condition. The Ramsey modified-golden-rule condition `r* = ρ + θ · g` (which fixes the interest rate from intertemporal optimization rather than from `s`) is an extension covered in sibling [`ec-ramsey-cass-koopmans-savings`](./ec-ramsey-cass-koopmans-savings.md), not a Solow result. **Source:** Romer (2019) Ch.1 pp.6-49.

## Mathematical Reasoning

The rental rate identity derives directly from the firm's break-even condition for hiring capital. With output price `p = 1` and the firm renting one unit of capital this period at gross price `R`, using it to produce `MPK` units of output, and ending with `(1 − δ)` units of physical capital that can be resold at unit price (frictionless capital market), the break-even condition is `R = MPK`. Equivalently, defining the net rental rate `r = R − δ`, we have `MPK = R = r + δ`, hence `r = MPK − δ`. The `δ` adjustment is the production-theory analog of the maintenance / opportunity cost that corporate-finance frameworks bury inside WACC; the microeconomic foundation is explicit about depreciation as the physical wear of capital. **Source:** Mas-Colell et al. (1995) Ch.5 pp.135-152.

Diminishing returns to capital (MPK decreasing in K, the standard concavity assumption) implies that the firm's capital demand is **downward sloping** in `R`: higher gross rental rate means fewer profitable capital units; lower gross rental rate means more. Under CRS Cobb-Douglas, `R = α · y/k`, so the capital-output ratio `k/y = α / R = α / (r + δ)` is pinned down by the rental rate and the capital share. This identity is the macro engine: a permanent increase in the saving rate raises `k/y` toward `α / (r + δ)` until the steady state re-establishes. **Source:** Romer (2019) Ch.1 pp.6-49.

**Boundary discipline (AC-11)**: This card stops at the microeconomic rental rate. The downstream cost-of-capital frameworks in the equity vertical build on `r` by: **Source:** Mas-Colell et al. (1995) pp.135-165.
- adding a tax wedge: the after-tax rental rate equals the pre-tax rate scaled by `(one − τ)`, with `τ` the corporate tax rate (treated in the equity vertical's WACC mechanics). **Source:** Mas-Colell et al. (1995) pp.135-165.
- weighting equity vs debt: WACC combines the equity required return and the after-tax debt cost via the capital-structure weights (treated in [`eq-equity-cost-of-capital-estimation`](../05_equity/eq-equity-cost-of-capital-estimation.md)). **Source:** Mas-Colell et al. (1995) pp.135-165.
- adding equity risk premium machinery from CAPM (sourced via the portfolio-management vertical's [`pm-capm-and-sml`](../09_portfolio_management_and_asset_pricing/pm-capm-and-sml.md), not re-derived here per AC-11 BOUNDARY-DISCIPLINE). **Source:** Mas-Colell et al. (1995) pp.135-165.

The 02 card owns the foundational identity `r = MPK − δ`; the 05 and 09 cards own the value-of-the-firm machinery that builds on top. Repo touchpoints link this card to its downstream consumers but no derivation is duplicated. **Source:** Mas-Colell et al. (1995) Ch.5 pp.149-165.

## See Also

- [`ec-production-functions-and-firm`](./ec-production-functions-and-firm.md) — the technology side `f(K, L)` and the marginal-product framework
- [`ec-firm-profit-maximization`](./ec-firm-profit-maximization.md) — the FOC `p · MPK = r + δ` from the firm's profit-max problem
- [`ec-solow-growth-model`](./ec-solow-growth-model.md) — uses the gross rental rate `(r + δ)` to characterize the steady-state capital-output ratio
- [`ec-investment-cost-of-capital`](./ec-investment-cost-of-capital.md) — extends the static rental-rate framing to dynamic adjustment-cost investment (Romer Ch.9)
- [`eq-equity-cost-of-capital-estimation`](../05_equity/eq-equity-cost-of-capital-estimation.md) — downstream WACC mechanics; do not duplicate
- [`eq-discount-rate-and-required-return-foundations`](../05_equity/eq-discount-rate-and-required-return-foundations.md) — downstream discount-rate framing; do not duplicate

## Escalate to Raw When

The full derivation of the firm's profit-max FOC `p · MPK = R` from the static cost-minimization + scale-choice problem is in MWG Ch.5 pp.135-152. The extension to dynamic investment with adjustment costs and the Jorgenson user-cost framework (where `r` includes tax adjustments and adjustment-cost terms) lives in Romer Ch.9 pp.420-440 and is treated more fully in the sibling `ec-investment-cost-of-capital` card. The corporate-finance discount-rate frameworks (WACC, APV) are out of scope for this card per AC-11 BOUNDARY-DISCIPLINE and live in 05 cards. **Source:** Mas-Colell et al. (1995) pp.135-165.
