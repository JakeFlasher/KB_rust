---
schema_version: "cacg.v0"
id: "fa-true-vs-reported-premium-price-discovery-share"
title: "True vs Reported Premium & the Price-Discovery Share"
reading_id: "22_fund_level_arbitrage"
summary: "The reported premium pi = p - NAV conflates a tradable shock to price (true premium u = p - v) with stale NAV. Madhavan's price-discovery share D = 1 - (sigma_u/sigma_pi)^2 is the variance fraction driven by the price leading a stale NAV, NOT by u: high D means the screen premium is mostly non-tradable staleness; low D means it is mostly the tradable true premium."
tags: ["price-discovery-share", "true-premium", "nav-staleness"]
citations:
  - source_id: "fa_madhavan_2016_etfs_new_dynamics"
    chunk_id: "fa_madhavan_2016_etfs_new_dynamics:p074:0087"
    chunk_hash: "c6fc4d87db03951531021a5cd659e017f751aee24963dfc7dd595a43a3acc2f4"
    page_range: [74, 74]
    quote: "Alternatively, if there is no NAV staleness and/or noise, then n v tt = and the premium entirely reflects the shock ut to the ETF price through the secondary market."
    edge_type: "defines"
  - source_id: "fa_madhavan_2016_etfs_new_dynamics"
    chunk_id: "fa_madhavan_2016_etfs_new_dynamics:p053:0056"
    chunk_hash: "20de591afe370cf391fe1098730ffcd1f2f2c266babcb855f659c45b85cdc5c2"
    page_range: [54, 54]
    quote: "Here, ut is the “true premium” that arises from transitory liquidity pressure."
    edge_type: "supports"
---
# True vs Reported Premium & the Price-Discovery Share

## Intuition
The premium an investor sees on a screen, pi = p - NAV, is not the premium an arbitrageur trades against. It mixes two very different things: a genuine shock to the fund's market price relative to its true intrinsic value (the *true* premium u = p - v), and a purely mechanical gap caused by NAV being stale (v - NAV). Buying into a "discount" feels like getting a bargain, but if that discount is just stale NAV catching up, there is nothing tradable there — the price was right and the NAV was wrong. The natural question becomes: of all the variation we observe in pi, how much is the tradable true-premium shock u versus the mechanically-correcting staleness gap? Madhavan answers it with the *price-discovery share* D — defined as the part of premium variance that is NOT the transitory shock u, i.e. the part where an actively-priced ETF is merely leading a stale NAV. The naming is counterintuitive: a HIGH D means the screen premium is mostly that un-tradable stale-NAV gap (the price is right, the NAV lags), while a LOW D means most of the premium is the genuine, tradable deviation u.

```
   reported premium pi = p - NAV
            |
   +--------+----------------+
   |                         |
 true premium            staleness gap
   u = p - v               v - NAV
 (tradable shock        (NAV lags true value;
  to ETF price)          NOT tradable -- corrects
                          mechanically)
            |
            v
   D = 1 - (sigma_u/sigma_pi)^2  = var(pi) share NOT from the shock u
       (Madhavan's "price-discovery" part: ETF price leading a stale NAV)
   D -> 1 : pi is almost all staleness gap  -- NOT tradable (NAV catches up)
   D -> 0 : pi is almost all the shock u    -- TRADABLE (genuine deviation)
```
**Source:** Madhavan (2016) §4.5.3 pp.74-75.

## Definition
Let p be the ETF market price, v its (unobserved) intrinsic value, and NAV (n) the reported net asset value. The **reported (observed) premium** is pi = p - n. The **true premium** is u = p - v, the residual of price over intrinsic value. Madhavan writes "Here, ut is the 'true premium' that arises from transitory liquidity pressure." The reported premium decomposes as pi = (p - v) + (v - n) = u + (v - n): a true-premium term plus a NAV-staleness term. The **price-discovery component** D is then defined as the portion of total premium variance not attributable to transitory noise shocks.
**Source:** Madhavan (2016) §4.5.2-4.5.3 pp.54, 73-74.

## Mathematical Reasoning
With sigma_u the standard deviation of the true-premium residual and sigma_pi the standard deviation of the observed premium, the price-discovery share is

    D = 1 - (sigma_u / sigma_pi)^2.

Read the two extremes symbolically. If the ETF price always reflects true value, u -> 0 so sigma_u -> 0 and D -> 1: the entire observed premium reflects staleness/pricing errors in NAV — i.e. it is phantom. Conversely, if NAV is never stale (n = v), then the staleness term vanishes and pi = u, so sigma_u -> sigma_pi and D -> 0: every wiggle in the observed premium is a tradable shock to ETF price.

The residual scale itself is sigma_u = sigma_eps / sqrt(1 - psi^2), where psi is the arbitrage-speed parameter and sigma_eps the transitory-liquidity-shock volatility. So sigma_u rises with sigma_eps (more noise) and rises with psi; more efficient arbitrage (smaller psi) shrinks residuals. Comparative statics on D follow from which term dominates var(pi). The staleness term dominates — so D is high — when an actively-priced ETF sits on a very stale NAV, as with international funds whose underlying markets are closed during local trading hours; Madhavan finds the price-discovery share largest for large international funds. The transitory-shock term dominates — so D is low — for small, thinly traded funds (e.g. small fixed-income funds), where liquidity noise u is a larger fraction of the premium. The driver is NAV staleness (raises D) versus transitory-liquidity noise (lowers D), not fund size per se.
**Source:** Madhavan (2016) §4.5.3 pp.74-75.

## See Also
- [`fa-premium-decomposition-and-estimation`](./fa-premium-decomposition-and-estimation.md) — the state-space estimation machinery that recovers u, v, psi, and phi from observable p and NAV; this card consumes those residuals.
- [`fa-iiv-iopv-intraday-fair-value`](./fa-iiv-iopv-intraday-fair-value.md) — intrinsic value v is the moving target that distinguishes true from reported premium.
- [`fa-nav-staleness-and-arbitrage-speed`](./fa-nav-staleness-and-arbitrage-speed.md) — the staleness coefficient phi and arbitrage speed psi that drive whether a reported premium is phantom or tradable.
- [`fa-international-price-discovery-and-enav`](./fa-international-price-discovery-and-enav.md) — closed underlying markets are where D is largest, since the ETF price leads stale foreign NAV.

In the legacy convertible-bonds tree, the analogous distinction is between an apparent CB price gap and the tradable mispricing once stale or model-marked components are stripped out, as discussed under cb-arbitrage-strategy. The behavioral-tree framing of noise-driven versus fundamental price gaps appears under be-sentiment-vs-fundamentals and be-noise-trader-equilibrium.

## Escalate to Raw When
Go to Madhavan (2016) Chapter 4 when you need the worked HYG and MUB case applications — e.g. the June 2013 Municipal Bond episode where the Bloomberg-observed premium and the state-space true-premium estimate diverge, or the HYG regression of true on observed premium whose slope reports the empirical fraction of the observed premium attributable to price discovery. The raw text also reports cross-sectional average values of D by asset class and fund size, and the financial-crisis subperiod estimates showing how arbitrage-speed parameters change under stress. The eight-parameter state-space estimation, the Kalman-filter recovery of the unobserved true premium, and the full derivation of sigma_u = sigma_eps / sqrt(1 - psi^2) live in the model appendix.
**Source:** Madhavan (2016) §4.5.1-4.5.4 pp.55-77.
