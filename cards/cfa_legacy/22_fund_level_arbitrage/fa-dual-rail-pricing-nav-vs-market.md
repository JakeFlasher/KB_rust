---
schema_version: "cacg.v0"
id: "fa-dual-rail-pricing-nav-vs-market"
title: "Dual-Rail Pricing: NAV vs Market Price"
reading_id: "22_fund_level_arbitrage"
summary: "A fund carries two value rails: NAV struck on its holdings (the asset rail) and the secondary-market price set by supply and demand (the trading rail). Open-end funds force them equal at end-of-day NAV; closed-end funds let them drift; ETFs hybridize via creation/redemption so the master signal premium/discount = (P-NAV)/NAV stays small."
tags: ["premium-discount", "nav", "open-end-closed-end"]
citations:
  - source_id: "fa_hill_2025_cfa_rf_etfs_2e"
    chunk_id: "fa_hill_2025_cfa_rf_etfs_2e:p014:0013"
    chunk_hash: "fd6a54cc812f6c0c7eb61789577ce9bdf1054e19e99efd473efefdfa1500f0a4"
    page_range: [14, 14]
    quote: "they have a creation and redemption mechanism to ensure that they trade close to their true net asset value (NAV) throughout the day."
    edge_type: "defines"
  - source_id: "fa_hill_2025_cfa_rf_etfs_2e"
    chunk_id: "fa_hill_2025_cfa_rf_etfs_2e:p022:0026"
    chunk_hash: "a54939fa94e9db1366bc8ee731d3e70c6f10bdc0d81f18dd4e6ad086bcb7a8dd"
    page_range: [23, 23]
    quote: "The price those investors pay is based entirely on supply and demand for that ETF, as with a stock. When buyers outnumber sellers, the price of the ETF goes up."
    edge_type: "supports"
card_hash: "0f15b116fab158d139e75c9db6c4677f1c916e899e86e3987eb9c489f9cd44f1"
---
# Dual-Rail Pricing: NAV vs Market Price

## Intuition
A pooled fund is valued on two separate rails that need not agree at any instant. The **asset rail** is net asset value (NAV): add up what the fund actually holds, divide by shares outstanding. The **trading rail** is the secondary-market price at which a share changes hands on an exchange, set by who wants to buy and who wants to sell right now. NAV answers "what is it worth?"; market price answers "what will someone pay?" The gap between them is the entire reason fund-level arbitrage exists.

The structure of the fund decides whether the rails are welded together or allowed to drift. An open-end fund transacts subscriptions and redemptions only at end-of-day NAV, so the rails meet once per day by construction. A closed-end fund has a fixed share count and no redemption channel, so its price floats freely and can trade at a persistent premium or discount. An ETF is the hybrid: it lists like a closed-end fund but bolts on a creation/redemption mechanism that lets large traders convert shares into the underlying basket (and back) at fair value, so the trading rail is continuously tugged toward the asset rail.

```
            ASSET RAIL                         TRADING RAIL
        NAV = holdings / shares            P = exchange supply/demand
              |                                    |
   open-end:  +======= forced equal at EoD ========+   premium/discount -> 0 each day
   closed-end:+ - - - - free float - - - - - - - - +   premium/discount can persist
   ETF:       +<== creation/redemption arbitrage ==>+   |P-NAV| held inside a band
                         signal: prem/disc = (P - NAV)/NAV
```

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.14-14.

## Definition
- **NAV (net asset value):** the per-share value of the fund's holdings, the asset rail.
- **Market price (P):** the exchange-traded price of a fund share, the trading rail, "based entirely on supply and demand for that ETF, as with a stock."
- **Premium / discount:** the signed relative gap between the rails, the master fund-arbitrage signal.
- **Open-end fund (OEF):** redeemable at end-of-day NAV; one of the three 1940-Act investment-company types alongside CEFs and UITs.
- **Closed-end fund (CEF):** fixed share count, no creation/redemption channel, price floats relative to NAV.
- **ETF:** an OEF (in the base case) granted exemptive relief, distinguished from a CEF because it "ha[s] a creation and redemption mechanism to ensure that they trade close to their true net asset value (NAV) throughout the day."

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.14-23.

## Mathematical Reasoning
Let `NAV` be the asset-rail value per share and `P` the trading-rail price. Define the master signal

```
premium/discount  =  (P - NAV) / NAV
```

so `P > NAV` is a premium and `P < NAV` a discount. The structure governs the admissible range of this ratio:

- **Open-end:** transactions clear at NAV, so the realized subscription/redemption price satisfies `P = NAV` at the strike, forcing `(P - NAV)/NAV -> 0` once per day.
- **Closed-end:** with no redemption rail the supply/demand price is unconstrained; `(P - NAV)/NAV` can be a persistent nonzero number of either sign.
- **ETF:** the creation/redemption channel lets arbitrageurs act whenever the gap exceeds round-trip cost `c` (basket trading + creation fee), so the no-arbitrage condition is `|P - NAV| <= c`, i.e. the signal is bounded inside a band `|(P - NAV)/NAV| <= c/NAV`.

Comparative statics: the ETF band half-width scales with transaction cost `c`, which rises with underlying illiquidity; as `c -> 0` the trading rail collapses onto the asset rail and `(P - NAV)/NAV -> 0` continuously, recovering open-end-like behavior intraday. A CEF is the `c -> infinity` limit where no arbitrage force acts.

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.14-23.

## See Also
- [`fa-etf-creation-redemption-arbitrage-band`](./fa-etf-creation-redemption-arbitrage-band.md) — how the creation/redemption round-trip sets the band half-width that bounds the premium/discount signal.
- [`fa-iiv-iopv-intraday-fair-value`](./fa-iiv-iopv-intraday-fair-value.md) — the intraday fair-value proxy that lets traders read the gap before end-of-day NAV is struck.
- [`fa-etf-vs-cef-premium-discount`](./fa-etf-vs-cef-premium-discount.md) — why the CEF rails drift while ETF rails stay welded.
- [`fa-premium-decomposition-and-estimation`](./fa-premium-decomposition-and-estimation.md) — decomposing the (P-NAV)/NAV signal into its drivers.

Legacy: this dual-rail framing parallels the convertible-arbitrage logic in the cb-arbitrage-strategy material, where a hedged position monetizes the gap between a security's traded price and its model/intrinsic value rather than its fund NAV.

## Escalate to Raw When
Go to the raw source when you need the worked arbitrage example with concrete bid, fair-value, and cost figures (the book walks a hypothetical where an AP buys the basket, creates a unit, and sells shares at the inflated exchange bid to pocket the per-share differential net of round-trip cost), the exhibit that draws the explicit dollar band around fair value, or the full 1940-Act/1933-Act and Rule 6c11 legal-structure detail distinguishing OEF-based ETFs from CEFs and UITs. Those numeric and regulatory specifics are abstracted out here and live in Module 1's "ETF Arbitrage" and "US ETF Legal Structures" sections.

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.23-29.
