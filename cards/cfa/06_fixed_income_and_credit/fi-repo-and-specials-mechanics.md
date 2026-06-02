---
schema_version: "cacg.v0"
id: "fi-repo-and-specials-mechanics"
title: "Repurchase Agreements and Specials"
reading_id: "06_fixed_income_and_credit"
summary: "A repurchase agreement is a short-term collateralised loan: the borrower sells the bond and agrees to repurchase it at a slightly higher price, paying the repo rate. The market clears at a general-collateral (GC) rate for any acceptable Treasury and a lower special rate for bonds in unusually high demand; specialness benefits the long holder via funding subsidy and is bounded by the fails-charge."
tags: ["fixed-income", "repo-specials"]
citations:
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p342:0423"
    chunk_hash: "17ecd9575cb85004b927a77007f5d563a3a28d616becd3c8a2659fe6efa475e8"
    page_range: [342, 343]
    quote: "Repos are short-term contracts that are used to lend money on the security of usually high-grade collateral, to finance the purchase of bonds, and to borrow bonds to be sold short"
    edge_type: "defines"
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p348:0431"
    chunk_hash: "21a5ca651efee2f963016c13f5fc0a27af8aa85468b529a4c6bd6021c4316072"
    page_range: [348, 349]
    quote: "So while repo investors are willing to accept general collateral, reverses require the delivery of a particular bond"
    edge_type: "supports"
  - source_id: "fi_crepey_bielecki_brigo_2014_counterparty_risk_funding"
    chunk_id: "fi_crepey_bielecki_brigo_2014_counterparty_risk_funding:p209:0273"
    chunk_hash: "f57432bb1e257c713d40971f9e7383df1026920b64cb8d289cb09bb170c8164f"
    page_range: [209, 210]
    quote: "We still assume r = 0, except in the numerics, and we work with the clean CSA closeout valuation scheme Q = P"
    edge_type: "supports"
card_hash: "507af03ed329f13a8ffa907089ddb1ff1d336e2ccea8b7dd5c17ee1cf759f8f4"
---
# Repurchase Agreements and Specials

## Intuition

A repurchase agreement (repo) is a short-term collateralized loan: the borrower sells a security today and agrees to repurchase it at a fixed price on a near-term date (often the next trading day for an overnight repo). The difference between the sale and repurchase prices is the implicit interest at the repo rate. From the cash side, it is borrowing secured by the bond; from the security side, it is lending the bond out for cash. The market clears at two distinct repo rates: a **general-collateral (GC) rate** that applies to any Treasury that can substitute as collateral, and a lower **special rate** that applies to a specific bond that is in unusually high demand. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

```
repo cash flows for an overnight repo
   t = today                              t = next day
       |                                       |
       |  borrower delivers bond + cash to     |
       |  lender; face F, market price P       |
       v                                       v
   *---+-----------------------------------+---*
         <- 1 period ->   repo rate r
                                          borrower repurchases bond:
                                          cash repaid =
                                              P·(1 + r·Δt) − accrual
                                          lender's gain = r·Δt · P
   gc rate vs special rate (same maturity, same date)
      gc rate          r_gc
      special rate     r_sp
      specialness      Δ = r_gc − r_sp   (Δ > 0)
   the specialness benefits the bond's holder, who can lend
   out the bond in repo at the lower r_sp and effectively
   reduce funding cost on the long cash-bond position by Δ
   per unit notional per period.
```

## Definition

A **repurchase agreement (repo)** is a contract for the simultaneous sale and forward repurchase of a security, with the cash-equivalent loan secured by the security. The cash leg pays a repo rate `r` over the term. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

**General collateral (GC) repo** is a repo in which the lender accepts any security from a defined pool (typically the on-the-run Treasury universe). GC repo rates closely track the OIS / overnight risk-free rate plus a small funding spread; in normal markets GC trades at or slightly above OIS. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

**Specials repo** is repo where the lender requires a specific bond as collateral (the "special"). Because the bond holder has bargaining power — the bond is in demand for short-covering or for direct purchase by another party — the bond holder pays a low repo rate, often substantially below GC. The bond is said to "trade special". **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

A **fail** occurs when the borrower of a security cannot deliver it on the settlement date. Treasury market fails carry a penalty rate (post-2009 the Treasury Market Practices Group (TMPG) fails-charge fixed at 3% minus the fed funds target rate, floored at zero), which sets a backstop on how negative repo specialness can become. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

**Specialness** is the GC − special spread for a given bond on a given date. A bond with specialness of 200 basis points trades 200 bps below GC in repo. Practitioners refer to the broad family of these bond-specific funding-rate departures as repo specials, a market segment distinct from the general-collateral market. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

## Mathematical Reasoning

For a long-bond holder, the carry over a repo period `Δt` is the bond's coupon accrual plus the spread between the bond's yield and the cost of financing the position in repo. If the bond can be lent out at the special rate `s` while GC rate is `g`, the holder earns `(g − s) · Δt` of carry-on-top per unit notional. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

Equivalently, the bond's effective yield to the holder is the cash-bond yield plus the specialness discount the holder captures via repo lending. This makes a bond that trades special more valuable than a comparable bond at GC. The market prices the specialness into the cash bond — the bond's yield is lower (price higher) than a non-special comparable to embed the expected specialness over the bond's life. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

The on-the-run / off-the-run wedge analyzed in [`fi-on-the-run-off-the-run-mechanics.md`](./fi-on-the-run-off-the-run-mechanics.md#mathematical-reasoning) is partly a repo-specialness phenomenon: the on-the-run is the most frequently shorted Treasury (used for hedging duration in dealer inventory, for short-positioning by macro investors), so it commands the largest expected specialness premium and the lowest cash yield. Tuckman quantifies this attribution and shows that the on-the-run wedge fraction attributable to specialness varies from quiet markets (most of the wedge is liquidity) to crisis markets (most is specialness as dealers desperately short for hedge purposes). **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

Failures are bounded by the fails-charge. The repo rate `s` cannot trade more negative than `g − failsCharge` because a borrower would rather pay the fails-charge than the negative rate. This caps specialness at the fails-charge level (3% minus the fed funds target rate, floored at zero, in the TMPG convention). **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

For collateralized derivatives (the CSA framework of [`fi-collateralization-and-csa.md`](./fi-collateralization-and-csa.md#mathematical-reasoning)), the cash collateral posted earns the OIS rate, which is closely tied to the GC repo rate. The two-curve discounting framework of [`fi-swap-spreads-and-libor-curve.md`](./fi-swap-spreads-and-libor-curve.md#definition) uses the OIS / GC-repo curve as the discount curve for collateralized derivatives. The bond-specific specialness is invisible at the swap-derivative level but matters for cash-bond practitioners optimizing carry. **Source:** Crepey+Bielecki+Brigo (2014) §4 pp.150-200.

The cash-bond liquidity intuition from [`fi-cash-bond-liquidity.md`](./fi-cash-bond-liquidity.md#mathematical-reasoning) names repo specialness as one component of the bond's funding subsidy; this card develops the mechanism at L2 depth: specialness has a market price (the GC − special spread), a contractual ceiling (the fails-charge), and a cyclical empirical pattern (widens with short-covering demand and dealer balance-sheet stress). **Source:** CFA L1 Curriculum (2022) Vol.5/pp.50-100.

## See Also

- [`fi-cash-bond-liquidity.md`](fi-cash-bond-liquidity.md) — L1 intuition framing of repo specialness within the liquidity-premium decomposition
- [`fi-on-the-run-off-the-run-mechanics.md`](fi-on-the-run-off-the-run-mechanics.md) — on-the-run / off-the-run wedge partially driven by specialness expectations
- [`fi-swap-spreads-and-libor-curve.md`](fi-swap-spreads-and-libor-curve.md) — OIS / GC-repo discount curve in the two-curve framework
- [`fi-collateralization-and-csa.md`](fi-collateralization-and-csa.md) — CSA cash-collateral earning OIS / GC-repo

## Escalate to Raw When

Open Tuckman & Serrat 3e Ch.12 (Repurchase Agreements and Financing) directly when any of the criteria below applies. **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.

- The card user needs the haircut and margin-call
  mechanics on a particular repo trade (e.g. a higher
  haircut on a long-dated Treasury versus a short-dated
  one) for a dated counterparty exposure analysis.
  **Source:** Tuckman & Serrat 3e (2011)
  Ch.12 pp.327-349.
- A specific historical repo-special episode (e.g. a
  crisis-period dislocation or a September-quarter-end
  repo spike) requires the empirical record Tuckman
  summarizes.
  **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.
- Tri-party repo settlement mechanics, FICC clearing
  details, or specific GCF Repo Index methodology are
  required — these are operational details out of
  CFA L1 and L2 scope.
  **Source:** Tuckman & Serrat 3e (2011) Ch.12 pp.327-349.
- Cross-border repo (USD repo against EUR collateral,
  cross-currency repo basis) is in scope — out of this
  card's single-currency framing; route to a future
  specialty plan.
  **Source:** Crepey+Bielecki+Brigo (2014) §4 pp.150-200.
