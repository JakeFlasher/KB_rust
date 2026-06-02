---
schema_version: "cacg.v0"
id: "pm-efficient-markets-and-anomalies"
title: "Efficiently Inefficient Markets"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Efficiently Inefficient Markets: framing the Pedersen \"efficiently inefficient\" view of markets — why prices are nearly but not exactly efficient, why active managers can earn returns above the market in equilibrium, and how anomalies fit as compensation for risk-bearing capacity rather than as mispricings to be arbitraged away"
tags: ["portfolio-management", "efficient-markets", "anomalies"]
citations:
  - source_id: "pm_pedersen_2015_efficiently_inefficient"
    chunk_id: "pm_pedersen_2015_efficiently_inefficient:p025:0021"
    chunk_hash: "3eda5ef5742536df7a6aed802b0a6027adce67a31bd04464133fd25f27b8cebf"
    page_range: [25, 25]
    quote: "The truth is equally well-defined: the truth is that markets are efficiently inefficient."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3702:5573"
    chunk_hash: "fb658aff770f546e1aded65a2942252bf51ee1595d8c599639cd84cf38f96a17"
    page_range: [3702, 3702]
    quote: "Capacity is the investment amount beyond which returns will be negatively affected by new investments."
    edge_type: "supports"
card_hash: "1f492eb0c5085d11723f3a7644017779597259d279f5097c53246a20f239d2d6"
---
# Efficiently Inefficient Markets

## Intuition

The Pedersen view is that markets are neither perfectly efficient
(in the strong Fama sense — prices reflect all information) nor
chaotically inefficient (in the strong behavioral sense — prices
disconnect from fundamentals). The truth sits between the two
extremes at a specific equilibrium: prices are inefficient just
enough that money managers can be compensated for the cost and
risk of providing liquidity to the market, and efficient enough
that no manager earns excessive returns. The framing is named
"efficiently inefficient" — a deliberately paradoxical label that
captures the equilibrium nature of the inefficiency. **Source:**
Pedersen (2015) pp.3-7.

```
        spectrum of market efficiency views
        ===================================

        +--------------------------------------------------+
        |  fully efficient (strong Fama)                    |
        |    prices reflect all info; no alpha possible     |
        |    --> passive investing dominates                |
        +--------------------------------------------------+
                            |
                            | reality
                            v
        +--------------------------------------------------+
        |  EFFICIENTLY INEFFICIENT (Pedersen)               |
        |    prices nearly efficient via competition        |
        |    money managers earn alpha as compensation      |
        |    for liquidity provision and risk-bearing       |
        |    no manager earns excess returns net of costs   |
        +--------------------------------------------------+
                            ^
                            |
        +--------------------------------------------------+
        |  fully inefficient (strong behavioral)            |
        |    prices disconnect from fundamentals;            |
        |    beating the market should be easy               |
        +--------------------------------------------------+
```

The mechanism that produces the equilibrium is the Grossman-
Stiglitz paradox in a richer form. If prices fully reflected all
information, no investor would have incentive to gather costly
information, so prices could not actually reflect that
information. Pedersen extends the argument with the observation
that money managers are paid for the service of providing liquidity
and bearing risk, not just for collecting information. Their
profits reflect the equilibrium price of liquidity — high enough to
sustain manager activity, low enough to deter additional entry.
**Source:** Pedersen (2015) pp.3-7.

## Definition

Pedersen's efficiently-inefficient market is one where prices and
returns satisfy two conditions simultaneously. **Source:** Pedersen
(2015) pp.3-7.

```
condition 1 (near-efficiency):
   net of all market frictions, securities' returns are very close
   to fully efficient levels --- consistently beating the market
   is extremely difficult.

condition 2 (sustained inefficiency):
   gross-of-costs returns earned by skilled money managers exceed
   passive returns by enough to cover their costs, fees, and risk-
   bearing premium --- managers can stay in business but not
   enrich themselves arbitrarily.
```

Demand-pressure and institutional friction are the immediate
mechanisms that push prices away from fundamentals. Examples
Pedersen names include hedging trades by commodity producers,
liquidation by capital-constrained funds, regulatory rebalancing
by pensions, capital-requirement-driven preferences by banks, and
clientele-driven aversion to illiquid securities. Each demand
shock creates a temporary price deviation that liquidity providers
trade against. **Source:** Pedersen (2015) pp.4-5.

The compensation paid to liquidity providers equals the per-trade
spread between the price they pay (when buying from a forced
seller) and the fundamental value (where the price will revert).
This spread is bounded above by the cost of additional liquidity-
provision capital and bounded below by the cost of competition
among existing providers. Equilibrium spread sits where these
bounds meet. **Source:** Pedersen (2015) pp.5-7.

## Mathematical Reasoning

The equilibrium argument decomposes a money manager's expected
gross-of-fees return on capital into two pieces: a passive
benchmark component and an alpha component compensating for
liquidity provision and risk-bearing. **Source:** Pedersen (2015)
pp.4-7.

```
E[r_manager_gross]  =  E[r_benchmark]  +  alpha_LP

alpha_LP   =  expected compensation for liquidity provision and
              risk-bearing capacity supplied to the market
costs(mgr) =  fees + transaction costs + risk-bearing capital cost
```

The equilibrium condition is that net-of-cost active returns
approximately equal passive returns: any persistent gap above
zero would attract entry; any persistent gap below zero would
trigger exit. **Source:** Pedersen (2015) pp.4-7.

```
equilibrium:  alpha_LP  ≈  costs(mgr)
              E[r_manager_net]  ≈  E[r_benchmark]

  ==>  the average active manager underperforms the benchmark
       by approximately the fee, but the cross-sectional spread
       around that average reflects skill differences.
```

The empirical implication Pedersen highlights is that average-
manager underperformance and the existence of skilled managers
are not contradictory — they are simultaneously implied by the
equilibrium. The average manager is capacity-bound and expense-
laden; the skilled manager outperforms net of costs by enough to
justify their fees. **Source:** Pedersen (2015) pp.4-7.

A specific implication for the L1 framing: documented anomalies
(size, value, momentum, low-volatility, quality) are reframed as
risk premia paid to investors who bear the corresponding factor
risks under capital and capacity constraints. The anomalies
persist because removing them entirely would require unlimited
liquidity-provision capital, which the equilibrium does not
provide. The Fama "joint hypothesis" issue — that anomaly tests
are simultaneous tests of efficiency and the asset-pricing model —
becomes a feature rather than a bug under this view: an anomaly
indicates either a missing risk factor in the pricing model or a
limit on arbitrage capital, and both interpretations point at the
same efficiency-inefficiency boundary. **Source:** Pedersen (2015)
pp.3-7.

The boundary with the L1-core `pm-market-efficiency-core.md`
sibling is the framing of efficiency. The core card uses Fama's
three-form taxonomy (weak / semi-strong / strong) and asks
"which form is empirically supported?" This extension card uses
Pedersen's equilibrium framing and asks "what equilibrium level of
inefficiency is sustained by the cost of providing liquidity?"
The two framings are complementary: Fama's taxonomy describes the
information set; Pedersen's framing describes the equilibrium that
absorbs it. **Source:** Pedersen (2015) pp.3-7.

## See Also

- [`pm-market-efficiency-core.md`](pm-market-efficiency-core.md) — L1-core three-form taxonomy that this extension card recasts as an equilibrium between active and passive
- [`pm-active-vs-passive-decision.md`](pm-active-vs-passive-decision.md) — the L1-core choice rule that takes efficiency as input; the efficiently-inefficient view sharpens the rule
- [`pm-active-management-and-alpha.md`](pm-active-management-and-alpha.md) — sibling extension card on active-manager evaluation under the efficiently-inefficient framework

## Escalate to Raw When

Open Pedersen (2015) Introduction section i and Ch.1 directly when
any of the criteria below applies. **Source:** Pedersen (2015)
pp.3-26.

- Detailed liquidity-provision economics with funding-liquidity-
  driven price-distortion mechanics — Pedersen Ch.5 develops the
  market-liquidity / funding-liquidity duality that this card
  abstracts. **Source:** Pedersen (2015) pp.63-83.
- Specific hedge-fund strategy taxonomies (long-short equity,
  global macro, fixed-income arbitrage, merger arbitrage) — Ch.1
  surveys; subsequent chapters develop each strategy. **Source:**
  Pedersen (2015) pp.19-26.
- The Berk-Green model of fund flows and decreasing returns to
  scale — Pedersen mentions in passing; the formal derivation
  belongs to the active-management sibling card. **Source:**
  Pedersen (2015) pp.4-5.
