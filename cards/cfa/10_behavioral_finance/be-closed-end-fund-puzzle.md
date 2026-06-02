---
schema_version: "cacg.v0"
id: "be-closed-end-fund-puzzle"
title: "Closed-End Fund Discount Puzzle"
reading_id: "10_behavioral_finance"
summary: "The four-part closed-end fund puzzle (start at a premium, drift to a discount, discounts comove and mean-revert, discounts shrink on open-ending) is evidence for systematic individual-investor sentiment rather than agency, tax, or illiquidity costs."
tags: ["behavioral-finance", "closed-end-funds", "investor-sentiment", "noise-trader-risk"]
citations:
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p062:0070"
    chunk_hash: "252e929b15a8a031ffacdcd3e001e1943b286aec213b87281e4068943c82bb58"
    page_range: [63, 63]
    quote: "There are four important pieces to the puzzle which together characterize the life cycle of a closed end fund"
    edge_type: "defines"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p067:0076"
    chunk_hash: "cad7d93f9395220d1afbbdcf22238d4f20b133191d023160a3695ed404b28905"
    page_range: [68, 68]
    quote: "Suppose further that noise traders' beliefs about the return on u relative to the return on s are subject to"
    edge_type: "supports"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p072:0083"
    chunk_hash: "3f90ff2091a70b9d732a583d3045c23c5e13e20dffb070afdeb2375c521a160a"
    page_range: [73, 73]
    quote: "levels of and changes in discounts should be highly correlated across funds. Since the same"
    edge_type: "supports"
  - source_id: "bf_shleifer_2000_inefficient_markets"
    chunk_id: "bf_shleifer_2000_inefficient_markets:p068:0077"
    chunk_hash: "cc7967319b97986d18cb25f8b65ebef261862c2d80fcc4c524ffebba92f297b4"
    page_range: [68, 68]
    quote: "In this model, the risk from holding a closed end fund (and any other security subject to the same stochastic sentiment) consists of two parts: the risk of holding the fund's portfolio and the risk that noise trader sentiment about the funds changes."
    edge_type: "supports"
card_hash: "441f14ab3dbe9a3d6b2420fa9a1dd732120dee5b22397d64e780bca96c1c5fa1"
---
# Closed-End Fund Discount Puzzle

## Intuition

A closed-end fund issues a fixed number of shares that trade on an exchange; unlike an open-end fund, an investor cannot redeem a share for its net asset value (NAV) but must sell it to another investor at the prevailing market price. The puzzle is that closed-end fund shares persistently sell at prices that diverge from the per-share market value of the assets the fund holds — typically at discounts of 10 to 20 percent in recent decades. If markets were efficient, the fund and its portfolio (which trade in the same market) should carry the same price, because an arbitrageur could buy the discounted fund and short its underlying portfolio. **Source:** Shleifer (2000) Ch.3 pp.53-53.

Shleifer (drawing on Lee, Shleifer, and Thaler 1991) ARGUES that the closed-end fund is the cleanest available laboratory for noise-trader theory: the supply of fund shares is fixed exactly like the unsafe asset in the noise-trader equilibrium of [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#intuition), and the fundamental value (NAV) is observable, so any sentiment-driven gap is directly measurable. Because U.S. closed-end funds are held and traded predominantly by individual investors, fluctuating individual-investor sentiment about future fund returns drives the discount up and down, while NAV moves only with the underlying portfolio. **Source:** Shleifer (2000) Ch.3 pp.59-60.

The decisive move is that this same sentiment is **systematic**: it touches all funds at once, plus other securities (notably small stocks) held by the same individual clientele. That makes the discount-widening risk undiversifiable, hence priced — which is precisely why the bounded arbitrage of [`be-limits-of-arbitrage.md`](./be-limits-of-arbitrage.md#intuition) cannot close the gap. **Source:** Shleifer (2000) Ch.3 pp.74-74.

## Definition

**Closed-end fund** is a managed fund with a fixed share count traded on a stock exchange; shares are sold to other investors rather than redeemed for NAV, so price and NAV can diverge. **Source:** Shleifer (2000) Ch.3 pp.53-53.

**The four-part puzzle** is the closed-end fund life cycle that any complete theory must explain. Shleifer DOCUMENTS the four pieces: (1) funds start at a premium of almost 10 percent when organizers raise new money; (2) within ~120 days they move to an average discount of over 10 percent, with discounts thereafter the norm; (3) discounts fluctuate widely over time, appear mean-reverting, and large discounts predict positive abnormal returns; (4) when funds are terminated by liquidation or open-ending, share prices rise and discounts shrink. **Source:** Shleifer (2000) Ch.3 pp.54-55.

**Standard (rational) explanations** are agency costs (high management fees / poor expected management), capital-gains tax liabilities on unrealized appreciation embedded in NAV, and illiquidity of restricted/block holdings overstating NAV. Shleifer ARGUES these explain at most part two (the existence of discounts) for some funds and, even collectively, fail to explain why funds start at a premium, why discounts fluctuate and comove, and why abnormal returns are realized on open-ending. **Source:** Shleifer (2000) Ch.3 pp.56-59.

**Discount comovement** is the cross-fund correlation of discount levels and changes, and the correlation of discount changes with returns on other individual-investor-held securities (e.g., small stocks). This is the most important and most testable implication of the sentiment view. **Source:** Shleifer (2000) Ch.3 pp.74-74.

## Mathematical Reasoning

Reinterpret the noise-trader model of [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#mathematical-reasoning) with the safe asset `s` equal to the fund's underlying portfolio and the unsafe asset `u` equal to the fund itself. Noise-trader sentiment about the return on `u` relative to `s` fluctuates stochastically; optimism drives the fund toward a premium or smaller discount, pessimism toward a wider discount. The discount is the directly observable image of the latent sentiment variable. **Source:** Shleifer (2000) Ch.3 pp.54,59.

Because holding the fund carries two risks — the risk of the underlying portfolio AND the risk that future sentiment turns more pessimistic (the discount widens) — the required return on assets held as fund shares must, on average, exceed the required return on the same assets held directly. Symbolically, the fund's equilibrium price reflects a sentiment risk premium, so the fund must on average sell at a discount: `E[r_fund] > E[r_portfolio]` implies `Price_fund < NAV`. This is the "create space" / mispricing-persistence consequence applied to closed-end funds. **Source:** Shleifer (2000) Ch.3 pp.72-72.

```
       closed-end fund life cycle (the four-part puzzle)
   premium
   +10% |  *(1) IPO premium
        |   \
    0%  +----\-------------------------------*----  (4) open-end:
        |     \                             /        discount -> 0
        |      \___ (2) drift to discount _/
  -10%  |          \      /\      /\      /
        |           \    /  \    /  \    /  (3) wide, mean-
  -20%  |            \__/    \__/    \__/      reverting swings
        +------------------------------------------> time
   NAV is the moving baseline; the gap to price = the discount.
```

Arbitrage cannot close the gap unless the arbitrageur has an infinite horizon and is never forced to liquidate: buying the discounted fund and shorting its portfolio is NOT a pure arbitrage opportunity because, at any finite liquidation date, the discount may have widened and the trade closed at a loss. Since arbitrageurs do not receive the full short-sale proceeds and may face redemptions, bearing noise-trader risk is unavoidable, positions are limited, and mispricing persists — the four-condition logic of [`be-limits-of-arbitrage.md`](./be-limits-of-arbitrage.md#mathematical-reasoning). **Source:** Shleifer (2000) Ch.3 pp.61-61.

## See Also

- [`be-noise-trader-equilibrium.md`](./be-noise-trader-equilibrium.md#intuition) — the fixed-supply unsafe-asset model reinterpreted with the fund as `u` and its portfolio as `s`.
- [`be-limits-of-arbitrage.md`](./be-limits-of-arbitrage.md#mathematical-reasoning) — why the buy-fund/short-portfolio hedge is not riskless when discounts can widen before a forced liquidation.
- [`be-investor-clientele-segmentation.md`](./be-investor-clientele-segmentation.md#intuition) — why the individual-investor clientele is what makes fund sentiment systematic and priced.
- [`be-sentiment-vs-fundamentals.md`](./be-sentiment-vs-fundamentals.md#intuition) — the general divergence-path framing the discount fluctuation instantiates.

## Escalate to Raw When

- The empirical tests of discount comovement with small-stock returns require the original Lee-Shleifer-Thaler (1991) regression specifications and the data-and-variable construction beyond the value-weighted discount index summary. **Source:** Shleifer (2000) Ch.3 pp.65-66.
- A specific standard explanation (agency, tax, restricted-stock/block-discount) must be quantitatively bounded (e.g., Malkiel's 2–6 percent tax bound) rather than dismissed qualitatively. **Source:** Shleifer (2000) Ch.3 pp.57-58.
- The full four-part-puzzle implication chain (why funds start, why discounts must vary stochastically, why open-ending narrows them) requires the chapter's own derivation rather than the card summary. **Source:** Shleifer (2000) Ch.3 pp.72-74.
