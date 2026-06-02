---
schema_version: "cacg.v0"
id: "fa-in-kind-basket-design-and-fees"
title: "In-Kind Basket Design, Custom Baskets & Creation Fees"
reading_id: "22_fund_level_arbitrage"
summary: "The issuer's published creation/redemption basket, its unit size, custom/partial deviations, cash-in-lieu, and fee tiers are levers that set the AP's cost of keeping price near NAV — and in-kind delivery of low-basis lots is what makes the wrapper tax-efficient."
tags: ["custom-basket", "cash-in-lieu", "creation-fee"]
citations:
  - source_id: "fa_abner_2016_etf_handbook"
    chunk_id: "fa_abner_2016_etf_handbook:p074:0095"
    chunk_hash: "a4c908e790dccfef58791b8cc7f644e1b48713fc3d68bf80e363ba2d4c7e2496"
    page_range: [74, 75]
    quote: "amount of shares that can be submitted by an AP for either a creation or a redemption order."
    edge_type: "defines"
  - source_id: "fa_abner_2016_etf_handbook"
    chunk_id: "fa_abner_2016_etf_handbook:p325:0429"
    chunk_hash: "5040b07021b0d3ee5d9a0ef1140c173267a8de0b830cc609f38b5f9d1db429dd"
    page_range: [325, 325]
    quote: "the ETF manager can choose to deliver low-cost stocks/bonds from an accounting perspective, clearing the balance sheet of potential gains."
    edge_type: "supports"
  - source_id: "fa_hill_2025_cfa_rf_etfs_2e"
    chunk_id: "fa_hill_2025_cfa_rf_etfs_2e:p024:0030"
    chunk_hash: "c48c8000c82ed4a9d49b5b62fa19002457a6dc2e0d7301ea8beb4461e6edcf35"
    page_range: [25, 25]
    quote: "In general, the more work the issuer will need to do to true-up the portfolio because of cash in lieu of securities, the larger the creation and redemption fee will be"
    edge_type: "supports"
card_hash: "87a13075842d7bb4d35477ec76d61cd4e3b2baaea257252e8a2b28f2ecd69e38"
---
# In-Kind Basket Design, Custom Baskets & Creation Fees

## Intuition
The creation/redemption mechanism is only an arbitrage engine if the basket the issuer publishes is something an authorized participant (AP) can actually source and deliver cheaply. So the issuer holds a small set of dials that govern how expensive it is for the AP to true the price to NAV: which securities go in the basket, whether the AP may deliver a custom or partial basket, how much cash (cash-in-lieu) substitutes for awkward names, the size of the creation unit, and the fee charged per create/redeem. Tightening or loosening these dials directly changes the AP's economics — and therefore how aggressively price is pulled toward NAV. The same in-kind delivery that lets the AP avoid the fund's trading desk also lets the portfolio manager hand out (on redemptions) the lowest-cost-basis lots, sweeping embedded gains out of the fund without a taxable sale.

```
  ISSUER DIALS                       AP ECONOMICS              PRICE vs NAV
  ------------                       ------------              -----------
  basket constituents  ----+
  custom / partial     ----+----> sourcing + true-up ----> cheaper create/redeem
  cash-in-lieu portion ----+        cost per unit          => tighter band |P-NAV| small
  creation-unit size   ----+
  creation/redeem fee  ----+----> bottom-line per arb ----> richer fee => band widens
```

**Source:** Abner (2016) *The ETF Handbook* 2e pp.74-75.

## Definition
- **Creation unit / basket**: the issuer-published list of constituent shares (plus a cash adjustment) that the AP exchanges in-kind for a fixed block of ETF shares. The creation unit size is the minimum share block an AP may submit for a create or redeem; smaller orders are aggregated by the AP off-book.
- **Custom (partial) basket**: a basket that deviates from full pro-rata replication of the holdings, permitted so the issuer can omit illiquid or hard-to-source names and lower the cost of creation.
- **Cash-in-lieu (CIL)**: cash substituted for a security the AP cannot or should not deliver in-kind (e.g., a restricted, corporate-action, or illiquid name); the fund must later "true-up" by trading that name itself.
- **Creation/redemption fee**: a flat (and/or variable) charge the issuer levies on the AP per transaction to recover the work of trading-up cash portions and balance-sheet usage; richer for funds requiring more true-up.
- **In-kind tax efficiency**: because redemptions deliver securities out rather than selling them, the manager can ship the lowest-cost-basis lots, cleansing embedded gains without realizing a taxable event at the fund level.

**Source:** Abner (2016) *The ETF Handbook* 2e pp.74-75, p.325; CFA Institute Research Foundation, *An Introduction to Exchange-Traded Funds* 2e pp.24-25.

## Mathematical Reasoning
Let the creation unit hold weights w_i in constituents i, let the issuer's per-transaction fee be f, and let the AP's frictional cost of sourcing/delivering the basket be c (financing, spread, true-up of any cash-in-lieu portion). The AP only acts when the price-NAV gap exceeds total cost:

    |P - NAV| > c + f   ==>   AP creates/redeems

so the no-arbitrage band half-width is approximately c + f. Every issuer dial moves c or f:
- raising the creation-unit size Q raises the residual-inventory and aggregation cost the AP must carry when client demand < Q, increasing effective c per economic unit transacted;
- substituting cash-in-lieu for an illiquid name raises the issuer's true-up burden, monotonically increasing f (true-up work up, fee up);
- a custom/partial basket that drops illiquid names lowers the AP's sourcing cost c at the price of some tracking deviation.

Tax efficiency is a sort selection, not arithmetic: on redemption the manager delivers lots with basis b minimizing realized gain (price minus b), pushing realized gain toward its floor of zero. Comparative statics: dband/dc > 0 and dband/df > 0, so the issuer chooses the dials trading tracking fidelity against a tighter price band.

**Source:** Abner (2016) *The ETF Handbook* 2e pp.74-75, p.325; CFA Institute Research Foundation, *An Introduction to Exchange-Traded Funds* 2e pp.24-25.

## See Also
- [`fa-etf-creation-redemption-mechanism`](./fa-etf-creation-redemption-mechanism.md) — this card extends the base create/redeem engine by detailing the basket-design and fee dials that set its frictional cost.
- [`fa-etf-creation-redemption-arbitrage-band`](./fa-etf-creation-redemption-arbitrage-band.md) — the c + f terms here are the explicit ingredients of that no-arbitrage band half-width.
- [`fa-pcf-cash-and-fund-seeding`](./fa-pcf-cash-and-fund-seeding.md) — the portfolio composition file is where the daily basket and its cash component (including cash-in-lieu lines) are published.
- [`fa-tracking-error-attribution-and-tco`](./fa-tracking-error-attribution-and-tco.md) — custom/partial baskets buy lower creation cost at the cost of replication fidelity, which surfaces as tracking error.

Legacy cross-refs (other tree, prose only): the in-kind low-basis-lot cleansing here is the structural cousin of issuer-side compound-instrument cost-basis discipline discussed under financial-reporting analysis; the band-widening logic mirrors limits-of-arbitrage reasoning in the behavioral cards.

## Escalate to Raw When
Go to the raw sources when you need the actual worked basket economics rather than the symbolic levers: the Abner basket-liquidity exhibit (the fictional 20-ticker grid showing shares-per-creation-unit as a percent of ADV, implied daily tradable shares, and how the least-liquid name caps daily ETF tradable notional) when you must size a creation unit against real ADV; the CFA Research Foundation fee schedule (the concrete range from a de-minimis charge on mega-cap/Treasury baskets up to the documented percentage cap for heavy cash-in-lieu true-up) when quoting fee tiers; and the Madhavan optimal-creation-unit-size case study (the emerging-market worked example trading dealer financing cost against round-lot frictions) when you need the cost-minimizing unit-size calculation. Heartbeat-style same-day in-and-out trades that supercharge low-basis-lot cleansing are likewise described, not computed, in the tax-efficiency discussion.

**Source:** Abner (2016) *The ETF Handbook* 2e pp.68-70, p.325; CFA Institute Research Foundation, *An Introduction to Exchange-Traded Funds* 2e pp.24-25.
