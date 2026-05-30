---
schema_version: "cacg.v0"
id: "fa-international-price-discovery-and-enav"
title: "International ETFs: Price Discovery When the Underlying Market Is Closed"
reading_id: "22_fund_level_arbitrage"
summary: "When the foreign basket is closed during US hours, no real-time arbitrage links ETF to NAV; the ETF becomes a price-discovery vehicle estimating the next open, its IIV is stale, and traders build an estimated NAV (eNAV) by shocking the stale basket by an expected change x plus estimated cash."
tags: ["international-etf", "enav", "stale-iiv"]
citations:
  - source_id: "fa_abner_2016_etf_handbook"
    chunk_id: "fa_abner_2016_etf_handbook:p262:0348"
    chunk_hash: "a7e9f60aa3ca8b5d943fdf74018105858ec626075f842b78cdb8d8a62642cf8f"
    page_range: [263, 263]
    quote: "ETFs will usually trade at a premium or discount to their IIV because there is no arbitrage mechanism available if the underlying markets are closed."
    edge_type: "supports"
  - source_id: "fa_abner_2016_etf_handbook"
    chunk_id: "fa_abner_2016_etf_handbook:p263:0350"
    chunk_hash: "72066448526bdf351c07a247abe66a39f5856d44bae06403c09b4631dcd82da4"
    page_range: [264, 264]
    quote: "net asset value (eNAV). Estimated NAVs have been used to value closed-end funds (CEFs) for many years prior to the invention of the ETF."
    edge_type: "defines"
  - source_id: "fa_hill_2015_cfa_rf_etfs_1e"
    chunk_id: "fa_hill_2015_cfa_rf_etfs_1e:p043:0049"
    chunk_hash: "6d65aca2949e8503bbc93dccbb2809166320bb1ca8e53674c2ec977199226e5e"
    page_range: [44, 44]
    quote: "A Comprehensive Guide to Exchange-Traded Funds (ETFs) 28 ©2015 The CFA Institute Research Foundation “stuck,” even as the ETF’s share price engages in price discovery."
    edge_type: "supports"
---
# International ETFs: Price Discovery When the Underlying Market Is Closed

## Intuition
A US-listed ETF holding Japanese (or any foreign) stocks trades while its underlying market sleeps. Nobody can buy the closed basket to collapse a mispricing, so the usual creation/redemption arbitrage tether is severed for those hours. The ETF stops being a derivative that tracks a basket and becomes a *forecast*: its market price is the crowd's best real-time guess of where the basket will reopen. That is why these funds routinely trade at a premium or discount to the published intraday value — the premium is information about the next open, not an exploitable gap.

The official IIV is the trap. It is computed from the basket's *last* (stale) local closing prices, converted at a moving spot FX rate. The equity component is frozen; only the currency leg flickers. So during US hours the IIV is a backward-looking snapshot while the ETF price is forward-looking. Comparing the live ETF price to a stale IIV manufactures a *phantom* premium that has no arbitrage cure.

```
    US trading day (foreign basket CLOSED)
    ----------------------------------------------------------
    last foreign close --> IIV  (stale equities, x = 0)
                            |  only FX spot flickers
                            v
    news / sentiment --> eNAV = stale-basket*(1+x) + est. cash   [forecast]
                            |
                            v
    ETF market price -------+   price-discovery vehicle
                            |
                            v
        no creation/redemption to close gap --> true vs phantom premium
                            |
    foreign market OPEN ----+ basket reprices; arbitrage tether restored
```
**Source:** Abner (2016) *The ETF Handbook* 2e §10 pp.263-264.

## Definition
- **Price-discovery vehicle:** an international ETF trading while its constituents are closed acts as a mechanism that estimates where the basket will trade at the local open; the ETF and basket move independently because no real-time arbitrage links them.
- **Stale IIV:** the published intraday indicative value uses the most recent local *close* of the equities; intraday it changes only through the spot FX conversion (for un-hedged funds) and barely at all for currency-hedged funds.
- **Reported NAV lag:** the official NAV of a foreign fund is published on a one-day lag relative to local trading and at the US close.
- **Estimated NAV (eNAV):** a forward-looking, subjective estimate that shocks the stale basket by an expected proportional change `x` (derived from correlating proxy assets) and uses *estimated* cash, to approximate real-time fund value. Abner introduces and names this construct.
- **True vs phantom premium:** measured against stale IIV, the gap is largely a forecast (phantom); measured against the correctly shocked eNAV, the residual is the genuine, tradable premium/discount.

**Source:** Abner (2016) *The ETF Handbook* 2e §10 pp.263-264; CFA Institute Research Foundation (2015) *A Comprehensive Guide to ETFs* p.44.

## Mathematical Reasoning
Let the basket's stale local equity value be `B_stale = Σ (shares_i × LastLocalPrice_i)`, `FX` the spot rate, `x` the expected proportional move of the constituents before the next open, `C_est` estimated cash, and `N` creation-unit shares. Then (symbolically, per Abner's formula):

  eNAV = [ (B_stale / FX) × (1 + x) ] / N + C_est / N

For currency-hedged funds add the daily forward-position P&L term. Two limiting cases:

- **x → 0:** eNAV collapses toward the stale IIV (equities frozen). Any ETF-price gap to IIV under x = 0 is *phantom* — pure forecast embedded in price.
- **Genuine premium:** define `P` = ETF market price. Reported premium `prem_IIV = P/IIV − 1` mixes forecast and mispricing; the economically meaningful premium is `prem_eNAV = P/eNAV − 1`, which strips out the expected reopen move `x`.

Limits-to-arbitrage condition: with the basket closed there is no riskless trade `ETF ↔ basket`, so no force drives `P → eNAV` intraday. A liquidity provider who shorts the ETF must hold a *correlating* hedge (not the creation basket); because the hedge is non-fungible with the creation unit, the position cannot be flattened by creation/redemption until the local market reopens — exactly the wide-arbitrage-band regime. The realized cost/benefit of paying the US-hours premium is `(reopen move) − x`: if the basket opens by less than `x` the buyer overpaid relative to waiting; if more, they saved — but `x` itself is unobservable, so the spread compensates the LP for hedge-breakdown risk.

**Source:** Abner (2016) *The ETF Handbook* 2e §10 pp.263-264, 267-268.

## See Also
- [`fa-true-vs-reported-premium-price-discovery-share`](./fa-true-vs-reported-premium-price-discovery-share.md) — extends this: how much of the reported premium is genuine vs price-discovery noise.
- [`fa-iiv-iopv-intraday-fair-value`](./fa-iiv-iopv-intraday-fair-value.md) — the domestic IIV/IOPV machinery whose staleness is the root cause here.
- [`fa-etf-creation-redemption-arbitrage-band`](./fa-etf-creation-redemption-arbitrage-band.md) — the band widens unboundedly while the underlying market is closed.
- [`fa-limits-to-arbitrage-when-creation-channel-breaks`](./fa-limits-to-arbitrage-when-creation-channel-breaks.md) — a closed basket is one concrete way the creation channel goes dark.
- [`fa-etf-vs-cef-premium-discount`](./fa-etf-vs-cef-premium-discount.md) — eNAV was first used for CEFs with infrequent NAVs; the analogy is direct.

(Legacy cross-refs, prose only: the behavioral-finance treatment of limits-of-arbitrage and noise-trader-driven premia parallels this — see the be-limits-of-arbitrage and be-noise-trader-equilibrium discussions in the other tree.)

## Escalate to Raw When
Go to the raw source when you need the worked international-ETF execution example (the step-by-step AP facilitation of an NAV order on a US-listed Japanese ETF, including the currency conversion and forward-hedge mechanics), the concrete premium-versus-reopen scenarios Abner walks through with specific percentage moves, the full Exhibit 10.2–10.4 liquidity-provider hedging diagrams showing why a correlating hedge cannot be collapsed via creation/redemption, or the cash-creation carve-out for restricted-access countries. Also escalate for the exact NAV/IIV/eNAV formulas including the currency-hedged forward-P&L term and the proxy-asset choices that generate `x`.

**Source:** Abner (2016) *The ETF Handbook* 2e §10 pp.255-257, 264-268.
