---
schema_version: "cacg.v0"
id: "fi-on-the-run-off-the-run-mechanics"
title: "On-the-Run vs Off-the-Run Treasury Mechanics"
reading_id: "06_fixed_income_and_credit"
summary: "On-the-Run vs Off-the-Run Treasury Mechanics — auto-generated placeholder summary; revise in fix-pass if needed; full audit notes available in audit_notes."
tags: ["fixed-income", "run-off"]
citations:
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p362:0451"
    chunk_hash: "ae7b1b57f274193c3aafaefcba0f0e6ede3cd57ef13a2d091d05dd8aa9b20de4"
    page_range: [362, 363]
    quote: "In other words, by the time the then-current 10-year has been around for a month, the specialness of the 31 2 s is projected to have dissipated."
    edge_type: "defines"
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p065:0079"
    chunk_hash: "2433e328c9b0829c6bd229ebd6eb43d842aecb07dcfbe36d17893e50f3afbb13"
    page_range: [65, 66]
    quote: "Treasury market, strategists assess relative value using spreads of individual Treasury issues against the USD swap curve."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2689:4021"
    chunk_hash: "72f02cfa469156bce1b7e5d81fcd6ea9cf83bbef4670c35576266e9bf4a60727"
    page_range: [2689, 2690]
    quote: "Importantly, analysts must consider the correlation between benchmark yields and credit spreads when deciding whether to use empirical or analytical duration estimates."
    edge_type: "supports"
card_hash: "669573ee89bbe4c67a64e1878755fbb0804cb1d4691ed78522862c0dfd9db6de"
---
# On-the-Run vs Off-the-Run Treasury Mechanics

## Intuition

The Treasury benchmark issue at each maturity tenor (2y, 3y, 5y, 7y, 10y, 30y) is the most recently auctioned bond at that point — the "on-the-run" issue. As the auction cycle rotates each month or quarter, the previously on-the-run issue becomes "old" (first off-the-run) and then "older" (further off-the-run) while still trading actively in the secondary market. At any given moment, multiple issues with similar remaining maturities coexist: the current on-the-run, the first-off-the-run from the prior auction, and the second-off-the-run with comparable cash-flow profile. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

```
issuance calendar (10-year tenor example)
    auction t-2        auction t-1        auction t (current)
       |                  |                   |
       v                  v                   v
   *--+--------------+--+--------------+--+--------------+--*
   issue: 10y_t-2   issue: 10y_t-1   issue: 10y_t (on-the-run)
   tenor remaining at observation time t+:
       9.83y               9.92y              10.00y
   status:
   second-off-the-run   first-off-the-run    on-the-run
   bid-ask: wider           moderate          tightest
   yield:    higher          slightly higher    lowest
```

## Definition

The **on-the-run issue** is the most recent Treasury auction outcome at a given benchmark maturity. Practitioner usage names the current 10y on-the-run as "the 10s" or "the current 10s"; the prior auction's bond becomes "old 10s" once superseded. The **off-the-run issues** are all earlier auctions whose original tenor is approximately the current benchmark tenor but whose remaining maturity has decayed by the time since auction. **Source:** Tuckman & Serrat 3e (2011) Overview pp.1-46.

The **when-issued (WI) market** trades the next on-the-run before the auction settlement date — the WI yield embeds the market's expectation of the auction result and reveals the supply-and-demand-balance the auction has yet to clear. Once the auction settles, the WI tag retires and the bond becomes the new on-the-run. The previous on-the-run rolls to first-off-the-run status simultaneously. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

The **benchmark rotation** is the cyclical replacement of on-the-runs by newly auctioned bonds. For the 2y Treasury this happens monthly; for the 10y and 30y, quarterly. Each rotation creates a discrete shift in which bond dealers use to quote the yield curve. **Source:** Tuckman & Serrat 3e (2011) Overview pp.1-46.

The **on-the-run / off-the-run spread** is the yield differential between two adjacent-issue Treasuries with near-identical remaining maturities. The on-the-run typically trades several basis points below (lower yield = higher price) the first-off-the-run at the same tenor. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

## Mathematical Reasoning

The spot-rate curve constructed in [`fi-spot-par-forward-curves.md`](./fi-spot-par-forward-curves.md#mathematical-reasoning) assumes all Treasuries at a given maturity discount the same future cash flow at the same rate. The on-the-run / off-the-run wedge violates this assumption: at the same maturity, the on-the-run prices at a slightly lower yield than the first-off-the-run despite the cash flows being identical (or nearly so) and the credit being identical. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

The wedge is a **liquidity premium**: the on-the-run holder benefits from tighter bid-ask spreads on resale and higher repo demand for the bond as collateral (the on-the-run is the most frequently shorted Treasury, which makes it "special" in repo per [`fi-repo-and-specials-mechanics.md`](./fi-repo-and-specials-mechanics.md#definition)). Investors are willing to accept a lower yield on the on-the-run because the implicit option to exit at low transaction cost is itself valuable. The wedge is therefore positive (on-the-run yield < off-the-run yield) at the same maturity. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

The on-the-run / off-the-run spread is empirically small (often 1-5 basis points in normal markets) but cyclically wide: during the 2008 crisis the spread blew out beyond 50 basis points as dealer balance sheets shrank and the option to exit the off-the-run cohort became materially harder. The wedge therefore informs cyclical liquidity-risk premia priced into the cash-bond curve. **Source:** Tuckman & Serrat 3e (2011) Overview pp.1-46.

For yield-curve construction (developed in [`fi-yield-curve-construction.md`](./fi-yield-curve-construction.md#mathematical-reasoning)) the practitioner must choose between fitting (a) on-the-run yields only — gives a "cleaner" benchmark curve but loses information at off-tenor maturities, or (b) a smoothed curve through all Treasuries — captures the full data but absorbs the liquidity-premium noise into the resulting spot rates. Tuckman's framework develops (b) as the default for relative-value work and treats (a) as a special benchmark-only convention. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

The bond-anatomy contract from [`fi-bond-anatomy-and-cashflows.md`](./fi-bond-anatomy-and-cashflows.md#mathematical-reasoning) is unchanged by on-the-run status: the contractual stream is identical regardless of which auction issued the bond. The on-the-run / off-the-run distinction is a market-microstructure property of the holder's secondary-market access, not a property of the issuer's promise. **Source:** CFA L1 Curriculum (2022) Vol.5/pp.50-100.

## See Also

- [`fi-spot-par-forward-curves.md`](fi-spot-par-forward-curves.md) — spot-rate curve baseline the on-the-run wedge perturbs
- [`fi-cash-bond-liquidity.md`](fi-cash-bond-liquidity.md) — v2 L1 framing of the on-the-run liquidity premium at intuition depth
- [`fi-yield-curve-construction.md`](fi-yield-curve-construction.md) — practitioner curve-construction choice between on-the-run-only fits and smoothed all-Treasury fits
- [`fi-repo-and-specials-mechanics.md`](fi-repo-and-specials-mechanics.md) — repo specialness mechanism that makes the on-the-run cheaper to fund

## Escalate to Raw When

Open Tuckman & Serrat 3e Ch.2 (Spot, Forward, and Par Rates) and the Overview (Global Fixed Income Markets) directly when any of the criteria below applies. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349; Overview pp.1-46.

- The auction calendar mechanics, when-issued market
  settlement, or specific rotation rules for a particular
  tenor are required at desk-level detail.
  **Source:** Tuckman & Serrat 3e (2011)
  Overview pp.1-46.
- A specific historical on-the-run / off-the-run spread
  time series (e.g. the crisis blow-out, the pandemic
  dislocation) is required for empirical analysis.
  **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.
- The card user needs the closed-form premium-
  decomposition into liquidity vs repo-specialness vs
  flight-to-quality components for a particular dated
  trade — Tuckman provides the framework; the empirical
  decomposition requires the dated repo-rate series
  outside this card's scope.
  **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.
