---
schema_version: "cacg.v0"
id: "fa-pcf-cash-and-fund-seeding"
title: "The Portfolio Composition File, the Cash DOF & Fund Seeding"
reading_id: "22_fund_level_arbitrage"
summary: "The PCF is the sponsor's published recipe of basket shares plus cash for one creation unit. Because share counts are fixed, cash is the single degree of freedom that reconciles the basket's fluctuating market value to the CU's NAV; at launch the same recipe is bootstrapped via fixed, floating, or zero-cash seeding."
tags: ["pcf", "cash-component", "fund-seeding"]
citations:
  - source_id: "fa_weiner_2021_etf_portfolio"
    chunk_id: "fa_weiner_2021_etf_portfolio:p081:0077"
    chunk_hash: "698b2ce2abf476ee05f0888fbe48fa904ca6664aa3c54c8090e1b69430f6f288"
    page_range: [82, 82]
    quote: "Fixed shares and fluctuating market values only leave one “degree of freedom” in this equation: cash."
    edge_type: "defines"
  - source_id: "fa_weiner_2021_etf_portfolio"
    chunk_id: "fa_weiner_2021_etf_portfolio:p080:0076"
    chunk_hash: "87f6d6ea2dd2f6b2b388b059a3a6e15a624356adfd3f9b630e65eec69edec8b8"
    page_range: [81, 81]
    quote: "The PCF relays the actual amount that would have satisfied a creation or redemption from the prior business day (actual cash) and a number of fields that are used to calculate the amount of cash expected to be part of a transaction on the current business day (estimated cash)."
    edge_type: "supports"
  - source_id: "fa_weiner_2021_etf_portfolio"
    chunk_id: "fa_weiner_2021_etf_portfolio:p091:0089"
    chunk_hash: "070c453a9b3036bce3d4eaabdc593dba9dd5afbfb699a382ab06b51fd5952c06"
    page_range: [92, 92]
    quote: "There is a catch-22, however: A fund that has not launched does not have a NAV, but to launch, the fund needs a PCF that requires the NAV as one of its inputs."
    edge_type: "supports"
  - source_id: "fa_abner_2016_etf_handbook"
    chunk_id: "fa_abner_2016_etf_handbook:p087:0115"
    chunk_hash: "27a357e8be797b30fae58c87022e54513bef7bee821ed2adef08605559c89e77"
    page_range: [87, 87]
    quote: "Paying financing charges on the money used to buy the ETF"
    edge_type: "supports"
card_hash: "edb5edd768eb4b257b5023e0b491c024691e0633679b95a6f1d19eb45340909c"
---
# The Portfolio Composition File, the Cash DOF & Fund Seeding

## Intuition
The Portfolio Composition File (PCF) is the sponsor's published "list of ingredients" for one creation unit (CU): a vector of named securities with share counts, plus an amount of cash. When an authorized participant (AP) creates, it delivers exactly the PCF; when it redeems, it receives exactly the PCF. The deep point is that the PCF must be published before closing prices exist, so the share counts are frozen while the value of those shares (and the CU's NAV) keeps moving. Rounding to whole lots means the basket value almost never lands exactly on the CU NAV. Something has to absorb the residual — and because everything else is fixed, that something is cash. Cash is the single degree of freedom that makes the swap value-neutral.

```
        PCF for 1 CU (published pre-close)
   +-----------------------------------------+
   |  fixed share vector  s_1 ... s_n        |   <-- frozen, cannot flex
   |  prices  p_i         (settle at close)  |   <-- fluctuate intraday
   |  -------------------------------------- |
   |  cash component C    <-- the ONLY knob  |---> tune so basket == CU NAV
   +-----------------------------------------+
            C = NAV_CU - sum_i s_i * p_i        (the plug that evens the trade)
```

**Source:** Weiner (2021) ch.5 pp.76-82.

## Definition
- PCF: the sponsor-specified basket of securities (in shares) and the amount of cash exchanged per creation unit; the AP delivers/receives exactly this in the primary market.
- Cash component (the DOF): the scalar amount of cash added to (or subtracted from) the basket so that basket-plus-cash equals the CU NAV. With fixed shares and fluctuating market values, cash is the one free variable that "evens the trade."
- Expected cash: the cash level the PCF reports for the current trade date, computed from the prior business day's closing prices and NAV (forward-looking estimate).
- Actual cash: the cash that actually made the prior business day's transaction value-neutral; published in the following day's PCF. Expected and actual cash generally differ and are not same-day comparable.
- Cash in lieu (CIL): a cash equivalent substituted for a security the AP cannot or should not transact (e.g., halted name, restriction); CIL feeds into the overall cash calculation.
- Seed capital: the initial creation order required to list the fund, funded by the sponsor or an AP/lead market maker.

**Source:** Weiner (2021) ch.5-6 pp.76-92.

## Mathematical Reasoning
Let a creation unit consist of fixed share counts s_i in securities i = 1..n at (yet-to-settle) prices p_i, with NAV_CU the unit's net asset value. The value-neutrality ("rule number one") constraint is:

  sum_i s_i * p_i + C = NAV_CU

Solving for the only unconstrained variable gives the cash component as a plug:

  C = NAV_CU - sum_i s_i * p_i

Sign convention: if the basket value exceeds the CU NAV (sum_i s_i * p_i > NAV_CU) then C < 0 ("negative cash" — the fund pays the AP); if the basket falls short, C > 0 (the AP adds cash). Lots are rounded down so C tends positive. Expected vs actual cash differ only because the s_i are frozen across the gap between the prior close (used to publish C_expected) and today's close (which fixes C_actual): C_actual - C_expected = (NAV_today - NAV_prior) - sum_i s_i (p_i,today - p_i,prior). At launch the same identity has no holdings to anchor it — a catch-22 since the PCF needs a NAV but an unlaunched fund has none. The three seeding regimes resolve it by fixing a different variable: fixed-NAV pins NAV_T0 = NAV_T(-1) and lets C absorb the difference; floating-NAV treats cash as a $1 security so the basket exactly matches the unit (NAV then floats out via marked basket value / shares outstanding); zero-cash sets C = 0 and recomputes NAV from basket value alone. Symbolically the seeder's carry is long-ETF / short-basket: financing cost on the ETF leg minus financing earned on the short, net of borrow fees and the dividend differential — a financing trade, not a directional view.

**Source:** Weiner (2021) ch.5-6 pp.81-96; Abner (2016) pp.87.

## See Also
- [`fa-etf-creation-redemption-mechanism`](./fa-etf-creation-redemption-mechanism.md) — the primary-market create/redeem flow the PCF instructs; this card supplies the recipe that flow consumes.
- [`fa-in-kind-basket-design-and-fees`](./fa-in-kind-basket-design-and-fees.md) — how the security side of the basket (and CIL, fees) is chosen; the cash DOF here is the residual after that design.
- [`fa-iiv-iopv-intraday-fair-value`](./fa-iiv-iopv-intraday-fair-value.md) — intraday fair value that the published PCF basket feeds.
- [`fa-tracking-error-attribution-and-tco`](./fa-tracking-error-attribution-and-tco.md) — cash drag and CIL-driven misallocation flow into tracking error.

## Escalate to Raw When
Go to the raw text when you need the worked SWA-ETF numerical walk-throughs: the holdings-based PCF tables (5.1-5.4) that show how a concrete closing NAV and basket market value back out to a specific expected-vs-actual cash figure; the index-open-file procedure (compute CU NAV, build a share vector from index weights / price, round to lots, derive positive or negative cash); and the launch worked examples (fixed, floating, zero-cash methods) that carry a stylized $25 T(-1) NAV through an 8-unit create to a reported T(0) NAV. Also escalate for the full seed-position economics line items (financing, management fees, borrow costs, dividend legs) when you must quantify a seeder's carry rather than reason about it symbolically.

**Source:** Weiner (2021) ch.5-6 pp.77-97; Abner (2016) pp.87.
