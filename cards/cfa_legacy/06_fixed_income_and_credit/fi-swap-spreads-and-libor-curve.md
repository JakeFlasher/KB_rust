---
schema_version: "cacg.v0"
id: "fi-swap-spreads-and-libor-curve"
title: "Swap Spreads and the LIBOR / OIS Swap Curve"
reading_id: "06_fixed_income_and_credit"
summary: "An interest-rate swap exchanges a fixed coupon for a floating coupon indexed to LIBOR / SOFR; the swap-fixed rate at each tenor defines the swap curve. The swap spread = swap-fixed minus Treasury par yield embeds bank-credit, dealer-funding, and Treasury-convenience components. Post-2008 two-curve practice splits projection (LIBOR/SOFR) from collateralised discounting (OIS / GC repo)."
tags: ["fixed-income", "swap-spreads"]
citations:
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p458:0584"
    chunk_hash: "7bbb8f045b23213190eafeca4946e5ccb8f922a718741a43cf414f44223765bc"
    page_range: [458, 458]
    quote: "Furthermore, since an interest rate swap exchanges a fixed rate for future LIBOR rates, expectations and risk premia with respect to future LIBOR rates will have an impact on observed swap rates"
    edge_type: "defines"
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p474:0607"
    chunk_hash: "2478cf4b1f6c9517ce524764809e06494939d49c16886a01c92794b70454d389"
    page_range: [474, 474]
    quote: "In any case, attitudes changed completely through the 2007–2009 crisis as the LIBOR-OIS spread rose to hundreds of basis points (see Chapter 15)"
    edge_type: "supports"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p174:0263"
    chunk_hash: "8b6a84ab31562c3e512d791826241e2e71df5602b5a558884a0b486042e2923e"
    page_range: [174, 175]
    quote: "The table shows that the swap is equivalent to the exchange of a floating rate bond (cash flows in the third column) for a fixed rate bond (cash flows in the fourth column)"
    edge_type: "supports"
card_hash: "220001c3d5b88589ccfbd4cd2c7e59673a47c24515abf9f43e700b17a1bf37ec"
---
# Swap Spreads and the LIBOR / OIS Swap Curve

## Intuition

A plain-vanilla interest-rate swap exchanges a fixed coupon for a floating coupon indexed to a short-term rate (historically LIBOR; increasingly SOFR / OIS post-LIBOR transition). The swap-fixed rate at each tenor defines the **swap curve**. Empirically the swap-fixed rate lies above the Treasury par-yield at the same tenor — the difference is the **swap spread**. The spread reflects a bundle of factors: bank credit risk priced into the floating leg, dealer balance-sheet capacity, scarcity of high-quality collateral, and the structural funding premium of unsecured bank borrowing relative to government collateral. **Source:** Tuckman & Serrat 3e (2011) Ch.16 pp.435-455.

```
yield (%)
   ^
   |                                       swap curve (LIBOR-fixed)
   |                                 .                      .
   |                            .                      .
   |                       .                      .
   |                  .              swap spread (bps)
   |             .
   |        .  .  .  .  .  .  .  .  .  .  .  .  .  .  .
   |   o                                      Treasury par curve
   |  o
   | o
   +-------------------------------------------------------> tenor
   2y      5y      10y      30y
   spread:  ~25     ~45      ~60     ~30
   legend:  o Treasury par   . LIBOR swap-fixed
```

## Definition

The **swap spread** at tenor `T` is `SwapFixedRate(T) − TreasuryParYield(T)`. By construction it is the differential at the par convention, not at the spot or forward level. Practitioners read the spread directly off market quotes; the spread is positive in normal markets and typically widens cyclically with credit stress. **Source:** Tuckman & Serrat 3e (2011) Ch.16 pp.435-455.

The **Z-spread** (zero-volatility spread) of a corporate or risky bond is the parallel shift to the Treasury (or swap) spot curve that equates the discounted cash flows to the observed market price — assuming no embedded options. The Z-spread differs from the swap spread because Z-spread compares a risky bond to a reference curve (chosen to be the swap curve in practitioner workflows), whereas the swap spread compares the two riskless reference curves (swap vs Treasury). **Source:** Tuckman & Serrat 3e (2011) Ch.20 pp.585.

The **asset-swap spread** is the spread that the swap dealer pays to the bond holder when the holder swaps the bond's fixed coupons for floating-rate receipts. It is the most operational of the spread definitions — it is a transacted price, not a model output. Z-spread and asset-swap spread differ by terms-of-trade effects (Tuckman shows the wedge is small for investment-grade bonds and large for distressed bonds). **Source:** Tuckman & Serrat 3e (2011) Ch.19 pp.536-572.

The **two-curve discounting framework** post-2008 separates the cash-flow-projection curve (LIBOR / SOFR-projected forwards) from the discount curve (OIS / collateralized rate) — the wedge between them is the funding-and-collateral premium. **Source:** Tuckman & Serrat 3e (2011) Ch.17 pp.457-481.

## Mathematical Reasoning

For a fixed-for-floating swap with notional `N`, tenor `T`, and fixed rate `K`, the par swap rate at inception (zero present value) satisfies the no-arbitrage condition `PV(fixed leg) = PV(floating leg)`. The fixed-leg PV uses the discount curve `D(t)`; the floating-leg PV simplifies because each floating coupon at reset is the period's forward rate. Substituting the par condition and solving for `K` gives the swap-fixed rate as a discount-curve-weighted average of forward rates. **Source:** Tuckman & Serrat 3e (2011) Ch.16 pp.435-455.

The swap spread `S(T) = SwapFixed(T) − TreasPar(T)` decomposes into (i) the **credit component** = bank counterparty credit risk priced into the LIBOR floating leg (post-LIBOR transition this shrinks since SOFR is overnight-collateralized), (ii) the **funding component** = dealer balance-sheet cost of warehousing the swap position, (iii) the **convenience-yield component** = Treasury premium for cash-like government collateral. The decomposition is not closed-form; Tuckman empirically attributes the 30y inverted swap-spread to a regulatory-driven funding squeeze on dealer Treasury balance sheets. **Source:** Tuckman & Serrat 3e (2011) Ch.16 pp.435-455.

The Z-spread satisfies the equation `Price = Σ CF_i · exp(−(z_i + S) · t_i)` where `z_i` is the spot rate at tenor `t_i` and `S` is the constant Z-spread. Z-spread is the single-number summary of the term-structure-aware credit-and-liquidity premium. The relationship with the par-curve swap-spread is non-trivial: a bond with a flat spot-curve risk profile has Z-spread ≈ swap-spread; bonds with steeply rising spot curves have Z-spread > swap-spread, and conversely. **Source:** Tuckman & Serrat 3e (2011) Ch.20 pp.585.

The two-curve framework (Tuckman Ch.17) refines the discount-curve construction developed in [`fi-yield-curve-construction.md`](./fi-yield-curve-construction.md#mathematical-reasoning): post-2008, the OIS rate (collateralized) is the right discount rate under a CSA-collateralized derivative, while LIBOR / SOFR forwards remain the right projection rate for the floating coupons. The wedge OIS − LIBOR is the FVA (funding valuation adjustment) territory and connects to [`fi-counterparty-risk-cva.md`](./fi-counterparty-risk-cva.md#mathematical-reasoning) for the dealer's funding-side accounting. **Source:** Tuckman & Serrat 3e (2011) Ch.17 pp.457-481.

The riskless-rate references developed in [`fi-spot-par-forward-curves.md`](./fi-spot-par-forward-curves.md#mathematical-reasoning) assume a single discount curve; the two-curve refinement shows that pre-2008 single-curve practice was an approximation that worked while the swap-Treasury spread was small but broke down under stress. The L2 practitioner discipline is therefore: use the swap curve as the relative-value reference for non-government bonds, but use a two-curve framework where collateral semantics differ between projection and discounting. **Source:** Tuckman & Serrat 3e (2011) Ch.17 pp.457-481.

For corporate-bond relative value (see [`fi-relative-value-screens.md`](./fi-relative-value-screens.md#mathematical-reasoning)), the bond's Z-spread vs the issuer's CDS spread is informative: the wedge between bond Z-spread and CDS spread is the **basis** = bond-implied credit risk minus CDS-implied credit risk; persistent positive bases suggest funding or liquidity friction in the cash bond market; negative bases suggest CDS over-trading. **Source:** Hull §7 pp.155-170; Brigo+Mercurio Ch.1 pp.10-35.

## See Also

- [`fi-spot-par-forward-curves.md`](fi-spot-par-forward-curves.md) — riskless single-curve baseline that two-curve discounting refines
- [`fi-yield-curve-construction.md`](fi-yield-curve-construction.md) — discount-curve bootstrap that the two-curve framework splits into projection + discount
- [`fi-credit-spread-machinery.md`](fi-credit-spread-machinery.md) — Z-spread decomposition into expected loss + risk premium under reduced-form credit
- [`fi-relative-value-screens.md`](fi-relative-value-screens.md) — bond-CDS basis as a practitioner relative-value trigger
- [`fi-counterparty-risk-cva.md`](fi-counterparty-risk-cva.md) — FVA / collateral-adjusted-discounting connection

## Escalate to Raw When

Open Tuckman & Serrat 3e Ch.16 (Swaps) and Ch.17 (Arbitrage with Financing and Two-Curve Discounting) directly when any of the criteria below applies. **Source:** Tuckman & Serrat 3e (2011) Ch.16 pp.435-455; Ch.17 pp.457-481.

- A specific swap-spread time series, decomposition
  attribution, or historical episode (e.g. an inverted
  30y swap-spread regime) requires the empirical evidence
  Tuckman provides at chapter detail.
  **Source:** Tuckman & Serrat 3e (2011)
  Ch.16 pp.435-455.
- The Z-spread vs OAS comparison is required for a bond
  with embedded options — escalate to Tuckman Ch.18
  (Fixed Income Options) and the OAS card
  [`fi-oas-and-effective-duration.md`](./fi-oas-and-effective-duration.md)
  together.
  **Source:** Tuckman & Serrat 3e (2011)
  Ch.17 pp.457-481.
- The L2-aligned multi-curve bootstrap algorithm with
  explicit OIS / SOFR / LIBOR pillars is required —
  escalate to Brigo+Mercurio and Hull for the bootstrap
  math; Tuckman provides the conceptual frame but not the
  full numerical recipe.
  **Source:** Brigo+Mercurio (2006) Ch.1 pp.10-35;
  Hull §7 pp.155-170.
- Post-LIBOR-transition jurisdiction-specific reference-
  rate mechanics (SOFR vs SONIA vs ESTR vs TONA) are
  required at desk-level detail — out of CFA L1 and L2
  scope.
  **Source:** Tuckman & Serrat 3e (2011) Ch.16 pp.435-455.
