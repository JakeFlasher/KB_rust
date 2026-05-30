---
schema_version: "cacg.v0"
id: "fa-etf-creation-redemption-arbitrage-band"
title: "The ETF Creation-Redemption Arbitrage Band: Why Price Tracks NAV"
reading_id: "22_fund_level_arbitrage"
summary: "The creation/redemption channel pins an ETF's market price inside a no-arbitrage band NAV ± c. APs step in only when the gap exceeds round-trip cost c; c scales with underlying illiquidity, fees, and hedge cost — wider band, looser tracking."
tags: ["arbitrage-band", "creation-redemption", "authorized-participant"]
citations:
  - source_id: "fa_hill_2025_cfa_rf_etfs_2e"
    chunk_id: "fa_hill_2025_cfa_rf_etfs_2e:p023:0028"
    chunk_hash: "289ba8af488fe600e100b83d837280e889fdf7bc5f6a8d0be2a301c26a50a940"
    page_range: [24, 24]
    quote: "the “arbitrage gap,” and it varies with the liquidity of the underlying securities and a variety of related costs."
    edge_type: "defines"
  - source_id: "fa_madhavan_2016_etfs_new_dynamics"
    chunk_id: "fa_madhavan_2016_etfs_new_dynamics:p090:0109"
    chunk_hash: "f6d492a55db310d4b98f815aae638fb3d43fc76d67f53c8b11f0699466490f88"
    page_range: [91, 91]
    quote: "ETFs typically trade within a no arbitrage zone between the underlying bid and ask prices plus the explicit costs of creation and exchange fees."
    edge_type: "supports"
---
# The ETF Creation-Redemption Arbitrage Band: Why Price Tracks NAV

## Intuition
An ETF's secondary-market price is set by supply and demand like any stock, so it can drift away from the fair value of the basket it holds. What stops the drift is a profit incentive: an authorized participant (AP) can create new shares at end-of-day fair value (selling rich ETF shares while buying the cheap basket) or redeem shares for the basket (buying cheap ETF shares while selling the rich basket). But the AP only acts once the mispricing covers a round-trip cost `c` — the bid-ask spread, market impact, exchange/creation fees, and any hedge cost. The result is a no-arbitrage *band* of width roughly `2c` centered on fair value: inside the band nobody profits from intervening, so the price floats freely; the moment price pierces an edge, an AP arbitrages it back. The book names this width the "arbitrage gap," and it widens with the illiquidity of the underlying.

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.23-24.

```
            premium leg (AP creates: sell ETF, buy basket)
                 ^   ETF price                            
                 |   above upper edge  ->  ARBITRAGE      
   NAV + c  -----+------ upper edge -----------------------
                 |                                         
                 |   Region: free float (no AP profit)    
   NAV     ......|...... fair value (IIV / iNAV) .........
                 |                                         
                 |   Region: free float (no AP profit)    
   NAV - c  -----+------ lower edge -----------------------
                 |   ETF price                             
                 v   below lower edge  ->  ARBITRAGE       
            discount leg (AP redeems: buy ETF, sell basket)

   band half-width  c = f( underlying illiquidity, fees, hedge cost )
```

## Definition
**Arbitrage gap (band):** the valuation differential at which it becomes profitable for an AP/market maker to step in; per the source it "creates a band around fair value inside which the ETF will fluctuate without inviting arbitrage." Let `P` = ETF market price, `NAV` = fair value of the underlying basket, and `c` = the AP's round-trip cost per share. The no-arbitrage band is `|P − NAV| <= c`.
- **Premium leg** (`P − NAV > c`): AP *creates* — sells the rich ETF shares and buys the cheap basket, exchanging the basket for a creation unit at day's end. This pushes `P` down and basket prices up.
- **Discount leg** (`NAV − P > c`): AP *redeems* — buys the cheap ETF shares and presents them for the basket, then sells the basket. This pushes `P` up and basket prices down.

Madhavan formalizes the same picture as three zones around the intrinsic value: **Region A** (inside the ETF's own bid-ask, normal trading), **Region B** (outside the ETF quote but still cheaper than buying the basket — no arbitrage), and **Region C** (beyond the upper/lower arbitrage bound — arbitrage profit exists, dealer intervenes).

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.23-24; Madhavan (2016) §6 pp.91.

## Mathematical Reasoning
Let the AP's round-trip arbitrage cost decompose as

```
c = (1/2)*spread_basket + impact_basket + fee_create/redeem + hedge_cost
```

The AP enters only when expected gross gain exceeds `c`, so the band edges are at `NAV + c` (creation/premium edge) and `NAV − c` (redemption/discount edge). Comparative statics fall straight out:
- `∂c / ∂(illiquidity) > 0` — wider underlying spreads and larger market impact raise `c`, widening the band; for mega-cap or Treasury baskets `c` is tiny (the gap can be as small as a cent), for thin or hard-to-replicate baskets it is large.
- `∂c / ∂(fee) > 0` — higher creation/redemption fees raise `c`.
- `∂c / ∂(hedge_cost) > 0` — sub-creation-unit demand forces the AP to hedge the residual, raising `c`.

Madhavan's ordering of bounds is `intrinsic ± [½·spread_ETF] ⊂ intrinsic ± [½·spread_basket + impact_basket] ⊂ intrinsic ± [+ fees + creation costs]`, i.e. Region A ⊂ A∪B ⊂ A∪B∪C; only crossing the outermost bound (into Region C) makes `(P − NAV) − c > 0` and triggers arbitrage. Tracking tightness is therefore inversely related to band width: realized `|P − NAV|` is bounded above by `c`, so liquid-underlying ETFs track NAV far more tightly than illiquid ones.

**Source:** Madhavan (2016) §6 pp.91; CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.24.

## See Also
- [`fa-etf-creation-redemption-mechanism`](./fa-etf-creation-redemption-mechanism.md) — the primary-market create/redeem plumbing that makes the band enforceable; this card is the price consequence of that mechanism.
- [`fa-iiv-iopv-intraday-fair-value`](./fa-iiv-iopv-intraday-fair-value.md) — the intraday fair-value estimate (IIV/iNAV/IOPV) that defines the `NAV` center of the band.
- [`fa-dual-rail-pricing-nav-vs-market`](./fa-dual-rail-pricing-nav-vs-market.md) — the two-price (NAV vs secondary market) structure the band reconciles.
- [`fa-tracking-error-attribution-and-tco`](./fa-tracking-error-attribution-and-tco.md) — band width `c` is a driver of realized tracking error.

Legacy cross-refs (other tree, prose only): the convertible-bond arbitrage strategy card cb-arbitrage-strategy is the closest analogue — a long-basket / short-mispriced-security trade that closes when a no-arbitrage relation is restored; and the China T+0 convertible arbitrage card cb-china-t-plus-zero-arbitrage shows the same intraday band-enforcement logic in a different settlement regime.

## Escalate to Raw When
Go to the raw sources when you need the worked numerics this card deliberately abstracts: the 2e text runs a concrete Exhibit 9 scenario with a stated ETF bid, fair value, per-side AP cost, and the resulting per-share profit captured on both the premium and discount legs — read those pages if you must trace the exact cent-by-cent arithmetic of a creation vs redemption round trip. For a fully calibrated band, Madhavan works a Russian-ETF example computing the dealer arbitrage bound from stated creation/redemption costs plus the underlying GDR bid-ask, and gives the cost-function form `E[C] = min(secondary cost, primary cost)` underlying the bounds.

**Source:** CFA RF *A Comprehensive Guide to ETFs* 2e (Module 1, 2025) pp.23-24; Madhavan (2016) §6 pp.91-92.
