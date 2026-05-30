---
schema_version: "cacg.v0"
id: "fa-funding-spirals-and-fire-sales"
title: "Funding & Market-Liquidity Spirals and Fire Sales"
reading_id: "22_fund_level_arbitrage"
summary: "Funding liquidity (traders' capital/margin) and market liquidity (price impact of trading) are two sides of one coin: a shock to either raises margins, forces deleveraging, and triggers self-reinforcing fire-sale spirals with cross-asset commonality."
tags: ["funding-liquidity", "margin-spiral", "fire-sale"]
citations:
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p166:0181"
    chunk_hash: "92fc9c0b1d3973b6dd7b3c8aa9953bd62b1bc33e8ebebc1bb8f2e3d12e9c4cb5"
    page_range: [167, 167]
    quote: "This self-reinforcing process can lead to a downward spiral in asset prices and the net worth of market participants."
    edge_type: "defines"
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p086:0090"
    chunk_hash: "aa21ae6e9aa9630a34b47d5886f36fa552955b05d7eb8533cb59d0f91cf9edae"
    page_range: [87, 87]
    quote: "Under certain conditions, margins are destabilizing, and market liquidity and funding liquidity are mutually reinforcing, leading to liquidity spirals."
    edge_type: "supports"
  - source_id: "fa_madhavan_2016_etfs_new_dynamics"
    chunk_id: "fa_madhavan_2016_etfs_new_dynamics:p229:0282"
    chunk_hash: "874305ca0b5bb91e7e184878a3ba1705a1f221fc92d2291460589f06c26df2cf"
    page_range: [229, 229]
    quote: "Such a “fire sale” could create a chain reaction that spreads to related asset classes, possibly even creating the perception of risk at the asset manager level."
    edge_type: "supports"
---
# Funding & Market-Liquidity Spirals and Fire Sales

## Intuition
Providing market liquidity costs capital. Even a "free" short sale or repo-financed
position requires the trader to post margin (the haircut), and that margin must be
funded out of the trader's own capital. So a trader's ability to absorb others'
supply/demand shocks — to provide *market* liquidity — is hostage to their
*funding* liquidity. The two are coupled in both directions: when funding tightens,
liquidity providers retreat and prices move more on a given trade; and when prices
move adversely, the assets' market liquidity worsens, which raises the haircut the
financier demands, which tightens funding again. Under stress this two-way coupling
stops being a damping loop and becomes an amplifying one — a *liquidity spiral*.
A fire sale is its terminal stage: a firm forced to dump assets to raise cash or pay
creditors drives prices below fundamentals, marking down the same assets held by
others, pushing *them* toward distress and forced sales. The damage spreads as a
negative externality, not just a private loss.
**Source:** van der Merwe (2015) pp.66-72, 152-153.

```
  shock (price drop / redemption)
            |
            v
  +--> asset value falls -> mark-to-market loss -> equity capital erodes
  |         |                                            |
  |         v                                            v
  |   market liquidity worsens                  funding tightens
  |         |                                            |
  |         v                                            v
  |   haircut / margin RISES <----- "margins destabilizing" -----+
  |         |                                            |
  |         v                                            |
  |   forced deleveraging / FIRE SALE ------------------>+
  |         |
  +---------+   (spillover: similar assets held by others marked down)
       SELF-REINFORCING DOWNWARD SPIRAL
```

## Definition
- **Funding liquidity**: the ease with which a trader obtains capital — cash on hand
  or credit on acceptable terms — to finance positions and meet margin.
- **Market liquidity**: the ease (low price impact) of trading an asset.
- **Margin / haircut**: the difference between a security's price and its collateral
  value in a repo; it must be financed out of the borrower's own capital, and total
  margin cannot exceed the participant's capital.
- **Slow-moving capital** (Duffie): short-term impediments to capital mobility that
  delay market makers' response to supply/demand shocks, affecting observed prices.
- **Fire sale**: a sale of securities mandated because, without it, the firm cannot
  obtain funding or pay creditors; it pushes prices away from fundamentals.
- **Commonality in liquidity**: bid-ask spreads widen simultaneously across many
  markets when funding to market makers is restricted.
- Two fire-sale channels: (a) **leverage-constrained** — debt capacity is equity
  times a lender-set leverage multiple, and rising haircuts cut that multiple; and
  (b) **equity-constrained** — firms cannot raise equity, become risk-averse, and
  shed risky assets even when debt is available.
**Source:** van der Merwe (2015) pp.66-71, 152-154.

## Mathematical Reasoning
Let a participant hold capital K, finance a position of value P at haircut (margin
rate) m. The binding funding constraint is m * P <= K, so maximum leverage is
P/K <= 1/m. Available debt financing D scales with equity E and a lender-set
multiple L: D = L * E, and asset demand is a function of total funds D + E.

The destabilizing feedback: the haircut m is itself an increasing function of the
asset's illiquidity, m = m(illiquidity), and illiquidity rises when forced sellers
push price below fundamental value. A negative price shock => illiquidity up =>
m up => max leverage 1/m down => forced position reduction => more selling =>
price further below fundamental. When the gain from this loop exceeds the damping
from arbitrage capital entering, the fixed point is unstable and the spiral runs.
A margin call is "destabilizing" precisely when dm/d(price) < 0 (margins rise as
prices fall), so that delevering accelerates rather than self-corrects.

Commonality: if many market makers share the same funding shock, their spreads
s_i widen together, so cross-asset liquidity correlation corr(s_i, s_j) > 0 even
absent any fundamental correlation — a pure funding channel.
**Source:** van der Merwe (2015) pp.66-72, 152-154.

## See Also
- [`fa-shleifer-vishny-limits-to-arbitrage`](./fa-shleifer-vishny-limits-to-arbitrage.md) — why capital-constrained arbitrageurs cannot close mispricing exactly when it is widest; the funding spiral is the dynamic mechanism behind those limits.
- [`fa-illiquidity-discount-and-crisis-amplification`](./fa-illiquidity-discount-and-crisis-amplification.md) — how the resulting price dislocations get priced as a crisis-state illiquidity discount.
- [`fa-amihud-mendelson-and-priced-liquidity-risk`](./fa-amihud-mendelson-and-priced-liquidity-risk.md) — the steady-state liquidity premium that the spiral perturbs in stress.
- [`fa-limits-to-arbitrage-when-creation-channel-breaks`](./fa-limits-to-arbitrage-when-creation-channel-breaks.md) — the ETF-specific analogue: redemption/creation gating as a funding-channel break.
- Legacy (other tree, prose only): the risk-management loss-distribution material (`rm-loss-distribution-anatomy`) frames the fat-tailed second-round spillover losses that fire sales generate as a negative externality on top of direct losses.
- `mt-funding-liquidity-fire-sales` and `mt-three-dimensions-liquidity` (reading 14) give the primary-source Brunnermeier-Pedersen derivation of the margin/liquidity spiral; this card adds the repo-haircut, CDS-basis, and ETF-redemption channels.

## Escalate to Raw When
Go to the raw source when you need the concrete crisis episodes and magnitudes the
prose abstracts away: van der Merwe walks the CDS-corporate-bond basis trade and
shows how the initial corporate-bond margin climbed through 2005-2008 until
financing for many hedge funds was simply unavailable, driving the basis sharply
negative across investment-grade and high-yield portfolios; he also gives the
LTCM (1998) and Amaranth (2006) deleveraging cases and Federal-Reserve-Bank-of-
New-York estimates of fire-sale spillover losses as a fraction of system capital
for broker-dealers and banks. For the Brunnermeier-Pedersen formal margin-spiral
model and the exact leverage-multiple algebra, follow his pointer to chapter 6.
The Madhavan supporting source supplies the fund-flow / first-mover-advantage and
mutual-fund-vs-ETF redemption-mechanics version of the same chain reaction.
**Source:** van der Merwe (2015) pp.68-71, 152-154; Madhavan (2016) pp.211-214, 226-234.
