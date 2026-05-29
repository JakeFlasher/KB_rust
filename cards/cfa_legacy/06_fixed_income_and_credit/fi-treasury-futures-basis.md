---
schema_version: "cacg.v0"
id: "fi-treasury-futures-basis"
title: "Treasury Bond Futures Basis and Cheapest-to-Deliver"
reading_id: "06_fixed_income_and_credit"
summary: "A Treasury futures contract obligates the short to deliver from a basket of eligible bonds; the short chooses the cheapest-to-deliver (CTD) by minimising the clean-price basis P_i - F·c_i where c_i is the conversion factor. The implied repo rate (IRR) ranks delivery candidates; CTD-switching optionality belongs to the short and depresses the futures price."
tags: ["fixed-income", "treasury-futures"]
citations:
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p392:0489"
    chunk_hash: "9d74b58edff980aa761dd9e8b1d27ce9293408c70a90f14e69c79c5cc7a97dff"
    page_range: [392, 393]
    quote: "The bond that minimizes the cost of delivery is called the cheapest-to-deliver or the CTD"
    edge_type: "defines"
  - source_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed"
    chunk_id: "fi_tuckman_serrat_2011_fixed_income_securities_3ed:p362:0451"
    chunk_hash: "ae7b1b57f274193c3aafaefcba0f0e6ede3cd57ef13a2d091d05dd8aa9b20de4"
    page_range: [362, 363]
    quote: "In other words, by the time the then-current 10-year has been around for a month, the specialness of the 31 2 s is projected to have dissipated"
    edge_type: "supports"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p123:0180"
    chunk_hash: "49b6c3fef1f0dfab708063680b684dbbd9f4a7b358c8cc0091c0fcc82e985c3c"
    page_range: [123, 124]
    quote: "Forward contracts are easier to analyze than futures contracts because there is no daily settlement—only a single payment at maturity"
    edge_type: "supports"
card_hash: "5549f88f7ef3f6801bd8d6dcd804bbcc16241ff74517ee02177ad5080972f1af"
---
# Treasury Bond Futures Basis and Cheapest-to-Deliver

## Intuition

A Treasury bond futures contract obligates the short side to deliver an eligible Treasury bond from a pre-specified basket at expiry. Multiple bonds in the deliverable basket satisfy the contract's eligibility (typically Treasuries with at least a minimum remaining maturity), so the short chooses which to deliver. The futures invoice paid to the short at delivery is `F · c_i + accrued_i` for the chosen delivery `i`, where `F` is the futures price and `c_i` is the bond's **conversion factor**. The short's economic optimization is to deliver the **cheapest-to-deliver (CTD)** bond — the bond whose net delivery cost is minimized. Because the bond purchased in the cash market is paid for at its dirty price `cleanPrice_i + accrued_i` and the futures invoice is `F · c_i + accrued_i`, the accrued terms cancel and the basis reduces to the clean-price comparison `cleanPrice_i − F · c_i`. The full practitioner convention used throughout this card is the clean-price basis. **Source:** Tuckman & Serrat 3e (2011) Ch.14 pp.373-399.

```
deliverable basket on a Treasury bond futures expiry
   basket (eligible Treasuries):
      Bond A: clean price P_A, conv factor c_A
      Bond B: clean price P_B, conv factor c_B
      Bond C: clean price P_C, conv factor c_C
      ...
   short's choice: deliver bond i minimizing the
   clean-price basis:
       basis_i = P_i − F · c_i
       (basis_i = the clean cash-bond price minus the
        conversion-adjusted futures price)
       the bond with smallest basis_i is the CTD.
   futures price F implied by the CTD's zero basis
   (ignoring repo carry to delivery):
       F = P_CTD / c_CTD
   over time as rates change, the CTD identity can switch:
       CTD optionality on the short side = the right to
       switch delivery to a newly cheapest bond.
```

## Definition

The **conversion factor** `c_i` for deliverable bond `i` is a multiplier that normalizes the bond's price to the equivalent of a hypothetical 6-percent-coupon bond delivered against the futures contract. The conversion factor depends only on the bond's coupon and remaining maturity at delivery; it does not depend on yields. The conversion factor is published by the futures exchange ahead of each delivery month and is fixed for the contract. **Source:** Tuckman & Serrat 3e (2011) Ch.14 pp.373-399.

The **basis** of bond `i` versus the futures contract is `basis_i = P_i − F · c_i` under the clean-price convention, where `P_i` is the bond's clean (quoted) price and `F` is the futures price. The accrued terms on both sides of the cash-vs-futures comparison cancel; the basis is a clean-price quantity. The bond's **net basis** further adjusts for the bond's repo cost between observation and delivery — a positive net basis means the bond costs more to hold-to-delivery than the futures-implied price; a negative net basis means the inverse. **Source:** Tuckman & Serrat 3e (2011) Ch.14 pp.373-399.

The **cheapest-to-deliver (CTD)** is the bond in the deliverable basket whose net basis is the smallest. The short delivers the CTD because that minimizes the cost of fulfillment. The futures price is determined by the CTD's clean price and conversion factor via the no-arbitrage condition `F = P_CTD / c_CTD` (less repo carry to delivery). **Source:** Tuckman & Serrat 3e (2011) Ch.14 pp.373-399.

The **implied repo rate (IRR)** for bond `i` is the rate that makes the bond's net basis zero against the futures contract — i.e. the repo rate at which an arbitrageur could fund the bond-versus-short-futures position at exactly break-even. The CTD has the highest IRR in the deliverable basket; other bonds have lower IRRs (would require costlier funding to break even). **Source:** Tuckman & Serrat 3e (2011) Ch.14 pp.373-399.

The **CTD-switching option** (or "delivery option") is the short side's right to substitute the CTD if a different bond's net basis falls below the current CTD's near delivery. As the yield curve shifts, the CTD identity can change. This optionality belongs to the short and depresses the futures price relative to its CTD-only theoretical level. **Source:** Tuckman & Serrat 3e (2011) Ch.14 pp.373-399.

## Mathematical Reasoning

The conversion-factor scheme equates economically-similar bonds in the basket. Without conversion factors, a low-coupon long-duration bond would always be cheaper to deliver in absolute price terms; the conversion factor adjusts for this so the differential reduces to (a) the curve-shape sensitivity of the bond's market price relative to the standardized 6-percent reference. **Source:** Tuckman & Serrat 3e (2011) Ch.14 pp.373-399.

The IRR ranking ties to [`fi-repo-and-specials-mechanics.md`](./fi-repo-and-specials-mechanics.md#mathematical-reasoning): a bond that trades special in repo (low repo rate, see the prior card's `r_sp < r_gc`) has the funding-cost advantage that translates into a lower net basis. The CTD is therefore often a bond that trades special — the futures contract embeds repo-specialness as part of its delivery price. **Source:** Tuckman & Serrat 3e (2011) Ch.14 pp.373-399.

The CTD identity is sensitive to the yield-curve shape: bonds whose duration is below the basket's average duration are cheaper to deliver when rates rise; bonds with above-average duration are cheaper when rates fall. The switching point — the curve level at which the CTD changes — is the **delivery-option strike**. **Source:** Tuckman & Serrat 3e (2011) Ch.14 pp.373-399.

The **basis trade** is the practitioner relative-value trade: long the cash bond + short the futures, expecting the net basis to converge to zero at delivery (or for the bond to retain its CTD status and the IRR to clear a financing target). The basis trade's PnL is driven by (a) carry from the bond's coupon less repo cost, (b) convergence of basis to zero, and (c) any CTD-switching event that hurts the long-bond / short-futures position. **Source:** Tuckman & Serrat 3e (2011) Ch.14 pp.373-399.

The link to forward pricing developed in [`fi-spot-par-forward-curves.md`](./fi-spot-par-forward-curves.md#mathematical-reasoning) is direct: the futures price is approximately a forward-price equivalent on the CTD, less the value of the CTD-switching option. As Hull develops in the futures-pricing chapter, the daily settlement of futures (mark-to-market with margin calls) introduces a small wedge between futures and forwards via the covariance between margin-call cash flows and the short rate; for Treasury futures the wedge is empirically small. **Source:** Hull §5 pp.105-130.

The bond-pricing baseline from [`fi-yield-and-price-mechanics.md`](./fi-yield-and-price-mechanics.md#mathematical-reasoning) gives the price `P_i` for each deliverable bond from its cash flows; the futures-basis machinery layers on the conversion-factor adjustment and the short-side delivery optionality. Cash-bond pricing is unchanged; the futures contract's economic value uses the CTD's price as input. **Source:** Tuckman & Serrat 3e (2011) Ch.14 pp.373-399.

## See Also

- [`fi-repo-and-specials-mechanics.md`](fi-repo-and-specials-mechanics.md) — repo carry component of the basis-trade PnL
- [`fi-yield-and-price-mechanics.md`](fi-yield-and-price-mechanics.md) — bond-pricing baseline that supplies P_i for each deliverable
- [`fi-spot-par-forward-curves.md`](fi-spot-par-forward-curves.md) — forward-vs-futures pricing link
- [`fi-on-the-run-off-the-run-mechanics.md`](fi-on-the-run-off-the-run-mechanics.md) — on-the-run bonds are often in the deliverable basket and command repo specialness that affects CTD selection

## Escalate to Raw When

Open Tuckman & Serrat 3e Ch.13 (Forwards and Futures: Preliminaries)
and Ch.14 (Note and Bond Futures) directly when any of the criteria
below applies. **Source:** Tuckman & Serrat 3e (2011)
Ch.13-14 pp.351-399.

- The card user needs the exact conversion-factor formula for a
  particular contract month and bond at desk-level numerical
  precision.
  **Source:** Tuckman & Serrat 3e (2011) Ch.14 pp.373-399.
- A historical CTD switching event (rate move that moved CTD
  identity from one bond to another) requires the dated bond
  prices, conversion factors, and repo rates Tuckman analyzes.
  **Source:** Tuckman & Serrat 3e (2011) Ch.14 pp.373-399.
- The contract specification (delivery dates, last-trade date,
  notification mechanics, wildcard option, end-of-month option)
  is required at exchange-rulebook detail — out of CFA L1 and L2
  scope.
  **Source:** Tuckman & Serrat 3e (2011) Ch.14 pp.373-399.
- Cross-contract spreads (the calendar-spread between adjacent
  contract months, the inter-tenor spread between 5y / 10y / 30y
  contracts) are required — Tuckman provides the framework but
  desk-level execution practice is out of scope.
  **Source:** Hull §5 pp.105-130.
