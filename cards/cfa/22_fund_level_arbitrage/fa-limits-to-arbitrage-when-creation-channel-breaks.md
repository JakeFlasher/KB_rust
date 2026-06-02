---
schema_version: "cacg.v0"
id: "fa-limits-to-arbitrage-when-creation-channel-breaks"
title: "When the Creation Channel Breaks: an Open-End Fund Trades Like a CEF"
reading_id: "22_fund_level_arbitrage"
summary: "The arbitrage band that pins an ETF to NAV depends on a working create/redeem channel. When issuance is halted (regulatory limit, market closure, dealer position limit), the band bound vanishes and the fund trades like a closed-end fund at a persistent premium until issuance resumes."
tags: ["creation-halt", "limits-to-arbitrage", "closed-end-fund"]
citations:
  - source_id: "fa_abner_2016_etf_handbook"
    chunk_id: "fa_abner_2016_etf_handbook:p328:0434"
    chunk_hash: "0356a5952a42fb9330a0f7e772d6483b17ea33654e7cf5efc65900237408d4e3"
    page_range: [329, 329]
    quote: "When an open-ended product loses its ability to issue new shares, it begins to act like a closed-end fund."
    edge_type: "defines"
  - source_id: "fa_madhavan_2016_etfs_new_dynamics"
    chunk_id: "fa_madhavan_2016_etfs_new_dynamics:p227:0279"
    chunk_hash: "7b86002db85827e09d44faf44dc46096bda38163225f1950708114addda070d9"
    page_range: [227, 227]
    quote: "the ETF will trade like a closed-end fund with possibly wider premiums or discounts."
    edge_type: "supports"
card_hash: "757bbea6df2661084fd57f9eae28ddbf9f217c153d1fb2620add43ef39abbe28"
---
# When the Creation Channel Breaks: an Open-End Fund Trades Like a CEF

## Intuition
The whole reason an ETF tracks its net asset value is that an arbitrageur can collapse any gap by creating or redeeming shares: too rich, short the fund and create new ones; too cheap, buy the fund and redeem. That discipline is conditional. The moment the create/redeem channel is interrupted — a regulator restricts the underlying derivatives, the home market closes, or the only desk willing to trade hits its position limit — the share count is frozen, the arbitrageur can no longer manufacture or extinguish supply, and the price floats free of the basket. An open-end fund that cannot issue shares stops being open-end in effect: it behaves like a closed-end fund, where the only way to express demand is to bid up the existing fixed pool of shares. So the gap to NAV is no longer bounded by creation cost — it can widen to a large, persistent premium and stay there until issuance resumes.

```
   NORMAL: create/redeem OPEN          BROKEN: channel HALTED
   ----------------------------        ----------------------------
   |P - NAV| <= creation_cost          band bound REMOVED
        ^                                   |
        | arbitrageur creates/redeems       | share count FROZEN
        | -> supply elastic                 | -> supply inelastic
        v                                   v
   price snaps back to NAV            price drifts to PREMIUM (CEF-like)
                                           |
                                      [persists] --until issuance resumes-->
                                           |
                                      premium collapses back into band
```

**Source:** Abner (2016) *The ETF Handbook* 2e pp.329-331.

## Definition
Creation/redemption interruption: a state in which the primary-market mechanism that lets authorized participants exchange a basket of underlying assets for fund shares (and vice versa) is suspended. Triggers documented include regulatory restriction of the underlying access vehicle (e.g., a securities regulator restricting OTC derivatives linked to a foreign equity index), closure of the home market of the underlying basket, and a dealer reaching its own internal collateral or position limits. Once issuance is frozen, the fund loses the connection between the price of the underlying assets and the price of the fund share, and trades like a closed-end fund — a fixed pool of shares whose secondary-market price can sit at a premium or discount to NAV indefinitely. When the underlying market reopens or issuance resumes, the fund "snaps back" to trading at standard small premiums and discounts.

**Source:** Abner (2016) *The ETF Handbook* 2e pp.329-331.

## Mathematical Reasoning
Let `P` be the fund's secondary-market price and `NAV` the per-share basket value. Under a working channel the no-arbitrage band is `|P - NAV| <= c`, where `c` aggregates creation/redemption costs (basket trading, fees, hedge slippage). This band is the *enforcement* mechanism, not an identity: it holds only because an arbitrageur facing `P - NAV > c` can short the fund, buy the basket, and create shares to deliver, driving `P` down. Interruption removes that action set. Symbolically, if the feasible creation quantity is constrained to `q <= q_max` and the halt sets `q_max = 0`, the upper enforcement disappears and the constraint becomes one-sided or absent. With supply frozen, price is set purely by secondary-market clearing of demand `D(P)` against a fixed float, so `P` can satisfy `P - NAV >> c` with no force pushing it back. The expected residual gap behaves like `E[P - NAV] > 0` and is *persistent* (positive serial correlation) rather than mean-reverting within the day, the defining comparative-static contrast with the open-end case. Shorting cannot substitute for creation: short interest is capped by shares available in the loan program, `short <= L`, and no new shares can be issued to satisfy `L`, so the short side cannot expand to discipline the premium. The premium therefore widens until the regime switch (`q_max > 0` restored), at which point arbitrage re-engages and `|P - NAV|` collapses back into the band.

**Source:** Abner (2016) *The ETF Handbook* 2e pp.329-331.

## See Also
- [`fa-etf-creation-redemption-arbitrage-band`](./fa-etf-creation-redemption-arbitrage-band.md) — defines the `|P - NAV| <= c` band whose enforcement this card shows is conditional on an open channel.
- [`fa-etf-vs-cef-premium-discount`](./fa-etf-vs-cef-premium-discount.md) — the closed-end-fund premium/discount regime that a halted open-end fund collapses into.
- [`fa-etf-creation-redemption-mechanism`](./fa-etf-creation-redemption-mechanism.md) — the primary-market plumbing whose suspension is the trigger here.
- [`fa-international-price-discovery-and-enav`](./fa-international-price-discovery-and-enav.md) — when the home market closes, the still-trading fund becomes the price-discovery tool, the flip side of the broken band.
- [`fa-settlement-clearing-and-cascading-shorts`](./fa-settlement-clearing-and-cascading-shorts.md) — short-side capacity limits (`short <= L`) that prevent shorting from substituting for blocked creation.

In the legacy tree, this is the fund-arbitrage instance of be-limits-of-arbitrage: a textbook limits-to-arbitrage case where the convergence trade exists but cannot be executed, so mispricing persists.

## Escalate to Raw When
Escalate to the raw source when you need the concrete worked episodes rather than the mechanism: Abner walks through the iPath MSCI India ETN (INP), whose creation halted in late 2007 after the Indian regulator restricted the OTC derivatives it used to access the index, and the resulting premium spike (Exhibit 13.3); the 2011 Egypt ETF, which kept trading for an eight-week home-market closure and moved to a double-digit premium (Exhibit 13.4); and the June 2013 "Taper Tantrum," when a large dealer hit internal collateral limits and suspended redemptions for an afternoon. Go to the raw chart annotations and the specific percentage premium figures, and to Madhavan's discussion of the joint-AP-cessation scenario and worst-case simultaneous redemption halts, when you need calibrated magnitudes or the named-case timeline.

**Source:** Abner (2016) *The ETF Handbook* 2e pp.328-332; Madhavan (2016) pp.224-234.
