---
schema_version: "cacg.v0"
id: "mt-liquidity-nature-provision-return"
title: "The Nature of Liquidity and the Return to Liquidity Provision"
reading_id: "14_microstructure_and_trading"
summary: "In the Grossman-Miller model liquidity is endogenous: with no private information the price move between periods is driven solely by liquidity shocks, so it equals the return risk-averse speculators earn for absorbing temporary order imbalances and supplying immediacy."
tags: ["microstructure", "liquidity", "immediacy", "inventory", "grossman-miller", "order-imbalance"]
citations:
  - source_id: "mt_ohara_1995_market_microstructure_theory_en"
    chunk_id: "mt_ohara_1995_market_microstructure_theory_en:p225:0297"
    chunk_hash: "e6ce8c2335b051d25b86fe8d793c3e315844fe04540004e2f82a04ce11719434"
    page_range: [225, 226]
    quote: "the change in price between periods can be thought of as the return to a speculator for providing liquidity"
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p097:0148"
    chunk_hash: "c7fec6d6db600550318f7291842b0cd7fb4b38155fbe03ad2bf5dba3448da0b9"
    page_range: [97, 97]
    quote: "one important function of liquidity suppliers is to serve as counterparty when the order flow is unbalanced."
    edge_type: "supports"
---
# The Nature of Liquidity and the Return to Liquidity Provision

## Intuition
Liquidity is "like pornography, easily recognized but not so easily defined."
O'Hara's resolution is to stop treating liquidity as a fixed attribute of a market
and to treat it instead as the *price of immediacy*: the concession an impatient
trader pays for the privilege of transacting now rather than waiting for the other
side of the market to arrive. Crucially, that price is not handed down by a
specialist — it is set endogenously by how many risk-averse speculators are willing
to step in and hold an unwanted inventory position until the imbalance clears.

The Grossman-Miller (1988) three-period model isolates this idea by stripping out
private information entirely. Nobody knows anything others do not, so prices carry
no informational content and cannot move because of adverse selection. The *only*
thing that can push price around is a liquidity shock: an exogenous order imbalance
that someone must absorb. A customer arrives at time 1 wanting to trade; speculators
buy from him at a price below the asset's expected value; at time 2 the offsetting
customer arrives and the imbalance reverses. The speculators unwind into that
arriving flow and pocket the price reversal. That reversal *is* the return to
liquidity provision.

```
   time 1                 time 2                 time 3
   liquidity shock i      offsetting shock -i    asset pays P3
        |                       |                     |
   customer demands        imbalance reverses     liquidation
   immediacy NOW           customer 2 arrives
        |                       |
        v                       v
   speculators BUY at P1 -> hold inventory -> SELL into flow at P2
        \________ price reversal P2 - P1 = return for bearing risk ________/
                  larger when:  fewer speculators (M small)
                                more risk aversion (a large)
                                more price variance
```

Read this way, an illiquid market is simply one where few speculators stand ready
to absorb imbalances, so each demanded trade moves price a lot; a liquid market is
thick with risk-bearing capacity, so the same shock barely dents price. Liquidity
is therefore a *return earned*, not a free service.

**Source:** O'Hara (1995) §8.1 "The Nature of Liquidity" pp.215-221.

## Definition
The setting is a three-period, single-risky-asset economy with a risk-free asset
(cash). The risky asset pays a terminal value P3 (a random variable) at time 3 and
trades at endogenous prices P1, P2 in periods 1 and 2. Two trader classes act:

- **Outside customers (liquidity demanders).** One receives an endowment, or
  *liquidity shock*, of size i at time 1; the other receives an exactly offsetting
  shock −i at time 2. The shocks are perfectly negatively serially correlated, a
  device capturing the fact that order imbalances even out over time but impose a
  cost when they occur.
- **Speculators / market makers (liquidity suppliers).** There are M of them. They
  take positions in the risky asset but, unlike specialists, do *not* quote bid and
  ask prices; they function only as risk-bearing counterparties willing, for a
  price, to hold an unbalanced inventory.

Every agent maximizes the *same* negative-exponential (CARA) utility with common
risk-aversion coefficient a, and all random variables are jointly normal. There is
**no private information**, so prices aggregate no signal and there is no rational-
expectations fixed point to solve — the model's only randomness is the exogenous
liquidity demand.

Define the speculator's gross return from period 1 to period 2 as
r = P2/P1 − 1. The card's organizing claim is that, under these assumptions,
E(r) is exactly the compensation a speculator requires to provide immediacy.

**Source:** O'Hara (1995) §8.1 pp.217-220.

## Mathematical Reasoning
Under CARA utility and normality, each agent's *gross* demand for the risky asset
takes the standard mean-variance form and, notably, depends on neither wealth nor
endowment — only on the conditional expected excess return per unit of conditional
variance. Writing the time-2 demand schematically,

    x2 = ( E2[P3] − P2 ) / ( a · var2[P3] ).

Market clearing at time 2 sums the demands of both arriving customers and the M
speculators against zero net supply (the shocks i and −i cancel: i + (−i) = 0). In
equilibrium this forces E2[P3] = P2, which makes each speculator's time-2 position
zero: by time 2 the imbalance is gone and there is nothing left to absorb.

At time 1 only the first customer and the M speculators are present, and net supply
is the shock i. Clearing customer demand plus the M speculators' demand against i
yields the compact equilibrium condition

    ( E1[P3] − P1 ) / ( a · var1[ E2(P3) ] )  =  i / (1 + M).

Because there is no information, P1 deviates from expected value *solely* to induce
1 + M risk-averse agents to hold the imbalance i; the (1 + M) in the denominator is
the total risk-bearing capacity sharing that load. Re-expressing the expected price
change as a return r = P2/P1 − 1 gives the model's punchline,

    E(r) = [ i·P1 / (1 + M) ] · a · var(r),

i.e. expected speculator return = (per-speculator inventory) × (risk aversion) ×
(return variance). Each speculator absorbs the fraction i/(1 + M) of the shock, so
the first bracket is literally the inventory each one carries.

Comparative statics fall straight out:

- **More speculators (M ↑):** denominator grows, E(r) falls, each absorbs less, and
  more of the shock clears in period 1 — the market is *more liquid*. As entry cost
  c → 0, M → ∞ and the market becomes infinitely liquid (E(r) → 0).
- **Higher risk aversion (a ↑) or higher variance (var(r) ↑):** E(r) rises — supplying
  immediacy is costlier, so liquidity is *dearer*.
- **Risk-neutral speculators (a = 0):** the whole liquidity-induced price move
  vanishes — price is constant across periods and immediacy is free. Risk aversion
  is what makes liquidity a priced, scarce service.

Endogenizing entry via a per-speculator cost c pins down equilibrium M as a function
of c, a, and the price/endowment variances, closing the loop: liquidity supply is a
return-seeking decision, and a market is liquid precisely when its return to
liquidity provision draws in enough risk-bearing capacity.

**Source:** O'Hara (1995) §8.1 pp.218-221 (eqs. 8.6-8.15).

## Boundary Notes
- **No-private-information is load-bearing.** The clean "price change = return to
  liquidity provision" reading holds *only* because all price movement is liquidity-
  driven. Once order flow is informative, the price move blends a transitory
  immediacy premium with a *permanent* adverse-selection component, and the simple
  identity breaks — this is exactly the Glosten-Milgrom / Kyle territory handled by
  the information-based sibling cards.
- **Inventory model, not a quote-setting model.** Speculators bear risk but post no
  bid/ask; price is determined by inventory imbalance, making this a multiperiod
  inventory model (in the spirit of Stoll 1980) rather than a spread-decomposition
  model. The compensation here is the inventory-holding cost component of
  illiquidity, not the order-processing or adverse-selection components.
- **Entry cost is exogenous.** Without an assumed cost c there is no interior M; the
  source flags that what truly determines the equilibrium provision of liquidity —
  especially the missing cost of trading against the better-informed — is left
  outside the model.
- **Perfectly offsetting shocks are a modeling device.** The assumption that the
  time-1 and time-2 liquidity shocks are i and −i (perfectly negatively serially
  correlated) is admittedly not obviously realistic; it is the cleanest way to make
  imbalances "even out across time" while still imposing a cost when they occur.

**Source:** O'Hara (1995) §8.1 pp.220-222; Foucault, Pagano & Röell (2013) §3.5 p.97.

## See Also
- [`mt-liquidity-depth-immediacy-width`](./mt-liquidity-depth-immediacy-width.md) -- decomposes liquidity into width/depth/immediacy/resiliency; this card derives the *price* of the immediacy dimension.
- [`mt-market-viability-no-trade-breakdown`](./mt-market-viability-no-trade-breakdown.md) -- the prior-chapter result that trade requires the mechanism to match buyers and sellers; here liquidity provision is what performs that matching.
- [`mt-liquidity-premium-asset-pricing`](./mt-liquidity-premium-asset-pricing.md) -- extends the per-trade return to liquidity provision into a cross-sectional liquidity premium in expected asset returns.

## Escalate to Raw When
O'Hara §8.1 (pp.218-221) carries the full dynamic-programming solution: the explicit
time-2 and time-1 demand functions (eqs. 8.6-8.11), the two market-clearing
conditions, and the derivation of the equilibrium return identity (eqs. 8.13-8.15)
plus the endogenous-M entry argument. The OCR garbles every one of these equation
lines, so re-read those exact pages in the clean PDF for the algebra. For the full
risk-neutral-limit and infinite-liquidity (c → 0, M → ∞) discussion and its
caveats, continue into pp.221-222.
