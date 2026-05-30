---
schema_version: "cacg.v0"
id: "fa-illiquidity-discount-and-crisis-amplification"
title: "The Illiquidity Discount & Crisis Amplification"
reading_id: "22_fund_level_arbitrage"
summary: "Pessimistic forced sellers valuing an asset at V-delta plus capital-rationed arbitrageurs push price below fundamental value; when a funding crisis exhausts arbitrage capital the discount jumps discontinuously, explaining why discounts persist and deepen."
tags: ["illiquidity-discount", "funding-crisis", "forced-liquidation"]
citations:
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p111:0117"
    chunk_hash: "63f8751dca8189b7f59395a9e6bef706005a119bf31e9e2ba79edcc1c03a02d9"
    page_range: [112, 112]
    quote: "We assume that the nonspecialists are overly pessimistic as they value the bond at V − δ even though it will pay V for sure at date 2."
    edge_type: "defines"
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p115:0121"
    chunk_hash: "cd63749b6256769b084dffa5aee999673ba0cc23dcd868566f054bc9f8944a1a"
    page_range: [115, 115]
    quote: "This implies that the illiquidity discount is higher and the price of the asset is lower during the funding crisis at date 1."
    edge_type: "supports"
---
# The Illiquidity Discount & Crisis Amplification

## Intuition
When the natural buyers of a cheap asset are sidelined, price stays below fundamental value — not because anyone disputes the value, but because the buyers who would correct it have no capital to deploy. Van der Merwe builds this from two ingredients. First, a class of *nonspecialists* who are overly pessimistic: they value an asset worth V (paid for sure later) at only V - delta, and they sell when shocked. Second, arbitrageurs who *know* V but allocate scarce capital across dates and strategies, so they cannot always absorb the supply. The asset then trades at a *negative liquidity premium* — an illiquidity discount — and because capital constraints persist, so does the discount. The dangerous regime is a funding crisis: the arbitrageurs who held back capital to buy at date 1 are themselves hit by a financing cut, so there are *no buyers at all*. Supply must clear against the pessimists alone, who only buy at deeply discounted prices, and the discount jumps down discontinuously. That jump is the most fund-relevant reason discounts both persist and suddenly deepen.

**Source:** van der Merwe (2015) pp.111-115.

```
NORMAL STATE                          CRISIS STATE (funding cut binds)
                                      
nonspecialists sell (val V-delta)     nonspecialists sell (val V-delta)
arbitrageurs (know V) BUY ---+        arbitrageurs: capital exhausted, NO buyers
   partial absorption        |            |
        v                    |            v  forced liquidators ADD supply
price = V - small premium    |        pessimists must become NET buyers
  (illiquidity discount)     |        price = V - LARGE premium
                             |            |
                    -- discontinuous DOWNWARD jump --+
                       gap widens with delta  (M1_crisis - M1* = delta(1 - phi*))
```

## Definition
- **Nonspecialists**: less-expert agents who are overly pessimistic, valuing the bond at V - delta (delta > 0) even though it pays V for sure; larger delta = larger expected price discount. Their shock-driven selling is the supply that creates the mispricing.
- **Illiquidity discount (negative liquidity premium)**: when assets trade well below fundamental value but there are no buyers, the price embeds a negative liquidity premium — the market price equals V adjusted *downward* by a liquidity premium term.
- **Funding/"crisis" state**: a low-probability, unexpected cut in arbitrageur financing (credit crunch or unrelated capital loss) at the date their reserved capital was meant to buy. Demand collapses to zero and the clearing condition changes.
- **Crisis amplification**: relative to normal times, the crisis-state mispricing is strictly larger; the price falls further and more dramatically the larger delta is.

**Source:** van der Merwe (2015) pp.111-115.

## Mathematical Reasoning
Nonspecialist aggregate supply at date 1 falls with mispricing: Supply = 1 + (P_A1 - V)/delta = 1 - M1/delta, so a larger mispricing M1 induces more pessimist selling. The equilibrium price is the fundamental value adjusted by a liquidity-premium term, of the general form p_A1 = V - delta(phi^2/2 + phi); the liquidity premium increases in the pessimists' discount delta and in the sensitivity phi of arbitrage capital under management. The forced-liquidation threshold phi-hat = (M0 - kappa*M1)/(kappa*M1) (eq. 4.14) governs how much arbitrage capital is committed early versus held in reserve.

In the crisis state the buyers vanish, so the clearing condition sets demand to zero and the liquidity premium becomes M1_crisis = delta(phi^2/2 + 1). Comparing states:

  M1_crisis - M1*  =  delta * (1 - phi*)        (eq. 4.19)

Since delta > 0 and phi* < 1, this difference is strictly positive — the discount is *unambiguously larger* in crisis, and the gap scales linearly in delta. Comparative statics: dM1_crisis/d(delta) > 0 (deeper discount when pessimists value the asset less, which is precisely when distress is worst), and the disappearance of arbitrage capital produces a sharper, discontinuous price fall as delta rises. Because the same capital pool backs positions across markets, a capital drop can force liquidation elsewhere — the channel for contagion.

**Source:** van der Merwe (2015) pp.112-116.

## Boundary Notes
The crisis-amplification result (a discontinuous downward price jump) depends on stated assumptions: nonspecialist sellers overly pessimistic at V - delta, a staleness/recovery parameter phi* < 1, a low-probability unexpected funding cut, and the three-date agency structure. The discontinuous jump does NOT hold absent the funding-crisis state. Flag these.

**Source:** van der Merwe (2015) pp.112-116.

## See Also
- [`fa-funding-spirals-and-fire-sales`](./fa-funding-spirals-and-fire-sales.md) — the funding-side mechanism (margin/financing cuts force liquidation) that triggers the discontinuous jump modeled here.
- [`fa-shleifer-vishny-limits-to-arbitrage`](./fa-shleifer-vishny-limits-to-arbitrage.md) — the performance-sensitive-capital and forced-redemption logic underpinning why arbitrageurs cannot always correct mispricing.
- [`fa-amihud-mendelson-and-priced-liquidity-risk`](./fa-amihud-mendelson-and-priced-liquidity-risk.md) — the priced-liquidity counterpart: why illiquidity commands a return premium in equilibrium.
- `mt-funding-liquidity-fire-sales` (reading 14) derives the same nonspecialist-supply / equilibrium-price model from primary sources; this card keeps the discontinuous crisis-jump result.

Legacy cross-references (other tree, prose only): the behavioral card on sentiment-versus-fundamentals (be-sentiment-vs-fundamentals) supplies the noise-trader pessimism analogue to the nonspecialists' V - delta valuation, and be-limits-of-arbitrage carries the parallel argument that bounded arbitrage capital lets mispricing persist.

## Escalate to Raw When
Reach for the raw text when you need the full date-0/date-1/date-2 timeline of the three-scenario agency model, the explicit market-clearing equations (4.15)-(4.18), the indifference-curve geometry of Figure 4.3 (intersection determining equilibrium phi* and M1*), or the worked comparison in Table 4.2 showing the crisis premium and the normal-vs-crisis gap for high and low delta — including the much larger gap when delta is small. The book reports concrete premium figures there; consult them directly rather than reproducing the arithmetic, and follow the cross-market contagion treatment forward to chapter 6.

**Source:** van der Merwe (2015) pp.110-116.
