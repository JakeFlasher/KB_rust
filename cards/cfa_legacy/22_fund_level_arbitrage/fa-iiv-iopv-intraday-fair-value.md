---
schema_version: "cacg.v0"
id: "fa-iiv-iopv-intraday-fair-value"
title: "IIV / iNAV / IOPV: The Intraday Fair-Value Signal"
reading_id: "22_fund_level_arbitrage"
summary: "NAV is struck once post-close and is static; IIV (also iNAV/IOPV) re-estimates the creation-unit basket value plus estimated cash roughly every 15 seconds intraday. Because it is last-trade-based it can go stale, so price, NAV, and intrinsic value form three distinct numbers."
tags: ["iiv", "iopv", "intraday-value"]
citations:
  - source_id: "fa_abner_2016_etf_handbook"
    chunk_id: "fa_abner_2016_etf_handbook:p249:0331"
    chunk_hash: "cd85621264823eb4d42b76c20825579430d6bec67b477b5bc2842a468c6aa8a2"
    page_range: [250, 250]
    quote: "The IOPV is also sometimes known as the intraday indicative value (IIV) or the indicative net asset value (iNAV)."
    edge_type: "defines"
  - source_id: "fa_madhavan_2016_etfs_new_dynamics"
    chunk_id: "fa_madhavan_2016_etfs_new_dynamics:p052:0055"
    chunk_hash: "788a8edc6d61404d6b8dac8f62bcd541d45fcdb42e144a1815ef6130acc86fe8"
    page_range: [53, 53]
    quote: "values: (1) the ETF secondary market price, (2) NAV, and (3) the"
    edge_type: "supports"
---
# IIV / iNAV / IOPV: The Intraday Fair-Value Signal

## Intuition
A mutual fund gives you exactly one number a day: the NAV, struck after the close from the day's last prices, and then frozen until the next close. An ETF is traded continuously, so a once-a-day number is useless for an investor or arbitrageur acting at 11:42 a.m. The fix is the IIV (Intraday Indicative Value), a second, faster-refreshing estimate of what the underlying basket is worth right now. It re-prices the published creation-unit basket against the most recent trade in each constituent, adds the estimated per-share cash, and re-broadcasts the result on a fixed cadence (typically every ~15 seconds) throughout the trading day. NAV tells you what the fund was worth at last night's close; IIV tries to tell you what it is worth this moment. The IIV is what lets the arbitrage band (see the creation/redemption card) be measured against a live anchor rather than a stale one.

```
   POST-CLOSE                          INTRADAY (every ~15s)
  +-----------+                       +-----------------------+
  |   NAV     |  static, struck once  |  IIV / iNAV / IOPV    |
  | last-close| ------ frozen ------> |  basket@last-trade    |
  |  prices   |   until next close    |  + estimated cash     |
  +-----------+                       +-----------+-----------+
                                                  | refresh tick
                                                  v
   three distinct numbers an arbitrageur watches:
        PRICE (tape)  ~vs~  IIV  ~vs~  intrinsic value (unobserved)
              \________ deviation drives the arb band ________/
```
**Source:** Abner (2016) *The ETF Handbook* 2e pp.239-240; Madhavan (2016) pp.53.

## Definition
- **NAV (Net Asset Value):** the most recent *official* value of the ETF, calculated post-market-close from the day's closing constituent prices and an actual accounting of total cash; it is a static, standardized, backward-looking number used for performance and accounting comparison.
- **IIV (Intraday Indicative Value):** the calculation of the most recent value of the fund based on market prices of the underlying securities, disseminated at regular intervals (typically every ~15 seconds) during the trading day. It is built from the published creation-unit share quantities priced at each constituent's last trade, plus the *estimated* (not total) cash per share. It is also called **iNAV** (indicative NAV) and **IOPV** (Indicative Optimized Portfolio Value); IIV is the mainstream quoting name.
- **Estimated cash vs total cash:** total cash is the post-close, backward-looking cash figure used for official NAV (so creations/redemptions occur at NAV); *estimated cash* anticipates intraday dividends, fees, and portfolio changes and is the cash term used inside IIV/eNAV.
- **Madhavan's three values:** the secondary-market **price**, the **NAV**, and the **(unobserved) intrinsic value** are three distinct numbers; both observed numbers may differ from intrinsic value, which need not even lie between them.

**Source:** Abner (2016) *The ETF Handbook* 2e pp.239-243, 250; Madhavan (2016) pp.53.

## Mathematical Reasoning
Let the published creation unit hold quantity `q_i` of constituent `i`, let `P_i^last` be that constituent's last traded price, let `C` be the creation-unit (CU) share count, and let `EstCash` be estimated per-CU cash. Then

```
IIV = ( SUM_i q_i * P_i^last ) / C  +  EstCash / C
```

The structurally parallel NAV uses *closing* prices and *total* cash:

```
NAV = ( SUM_i q_i * P_i^close ) / C  +  TotalCash / C
```

So IIV and NAV differ in exactly two inputs: the price vector (`P^last` vs `P^close`) and the cash term (estimated vs total). Refresh cadence sets a quantization floor: if the basket moves continuously but IIV ticks only on a `Δt ≈ 15 s` grid, the published IIV lags true basket value by up to one tick. **Staleness** is a separate, additive error: a constituent that has not traded since 2:00 p.m. contributes `P_i^last` even though the live midpoint may already be `(P_i^bid + P_i^ask)/2 > P_i^last`. Hence the observed gap decomposes as

```
ETF_price - IIV_published  =  (true basket move within dt)  +  (stale-print error)  +  (genuine premium/discount)
```

For a domestic, simultaneously-trading basket, no-arbitrage forces `ETF_price ≈ IIV` in parity; the residual is mostly latency, not real premium. For an international/illiquid basket the staleness term dominates and `IIV` ceases to anchor anything, which is why an estimated IIV (eNAV) is substituted. In Madhavan's framing the live tape price `p`, the vendor `NAV`, and the unobserved intrinsic value `v` satisfy no fixed ordering: `v` may exceed or fall below both.

**Source:** Abner (2016) *The ETF Handbook* 2e pp.243, 250-252; Madhavan (2016) pp.53.

## See Also
- [`fa-etf-creation-redemption-arbitrage-band`](./fa-etf-creation-redemption-arbitrage-band.md) — the arb band is measured as price-vs-IIV; this card supplies the live anchor the band is drawn around.
- [`fa-dual-rail-pricing-nav-vs-market`](./fa-dual-rail-pricing-nav-vs-market.md) — the primary-market-NAV vs secondary-market-price split that IIV bridges intraday.
- [`fa-pcf-cash-and-fund-seeding`](./fa-pcf-cash-and-fund-seeding.md) — the published creation-unit / portfolio-composition file and estimated-cash figures that feed the IIV formula.
- [`fa-nav-staleness-and-arbitrage-speed`](./fa-nav-staleness-and-arbitrage-speed.md) — develops the staleness term and how price-discovery speed interacts with stale anchors.
- [`fa-international-price-discovery-and-enav`](./fa-international-price-discovery-and-enav.md) — when the home-market basket is closed, IIV must be replaced by an estimated NAV (eNAV).

## Escalate to Raw When
Go to the raw source when you need the worked numeric demonstration. Abner's chapter walks a concrete five-step IIV computation on a sample creation unit (multiplying each name's required shares by its last price, summing, dividing by CU shares, then adding per-share estimated cash) and a simulated order-book example showing how a limit placed at a discount to a quoted IIV does or does not fill against a liquidity provider hedging the basket. Madhavan's model assigns explicit weights to NAV and price to back out an estimate of the unobserved intrinsic value and runs the bond-fund "snake eating its own tail" circularity case where IIV is itself derived from ETF prices. Pull these for the arithmetic and the empirical decompositions; they are out of scope for this skeleton per the no-worked-arithmetic rule.

**Source:** Abner (2016) *The ETF Handbook* 2e pp.250-252; Madhavan (2016) pp.40, 53.
