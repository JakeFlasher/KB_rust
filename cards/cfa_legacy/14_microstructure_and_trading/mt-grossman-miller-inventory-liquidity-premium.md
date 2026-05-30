---
schema_version: "cacg.v0"
id: "mt-grossman-miller-inventory-liquidity-premium"
title: "Grossman-Miller Market Making: Inventory Risk and the Liquidity Premium"
reading_id: "14_microstructure_and_trading"
summary: "A risk-averse market maker with no intrinsic demand for inventory earns a liquidity premium that exactly compensates her for the price risk of warehousing the asset until an offsetting liquidity trader arrives."
tags: ["microstructure", "market-making", "inventory-risk", "liquidity-premium", "grossman-miller"]
citations:
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p037:0041"
    chunk_hash: "b0dca7ef2a9b9de5a29ac6a7a6cd3cf912be8f1849b2333666a0057bc856a720"
    page_range: [38, 38]
    quote: "MMs obtain a liquidity premium from liquidity traders that exactly compensates"
    edge_type: "defines"
---
# Grossman-Miller Market Making: Inventory Risk and the Liquidity Premium

## Intuition

A market maker (MM) does not want to own the asset for its own sake. She has "no
intrinsic need or desire to hold any inventory," so she only takes one side of a
trade in anticipation of unwinding it later against an offsetting order. The
problem is timing: when a liquidity trader (LT1) arrives wanting to sell, the
balancing buyer (LT2) has not yet shown up. Between those two moments the MM must
warehouse the asset and is exposed to the risk that its price moves against her.

If everyone were risk-neutral, this delay would be costless — mean-zero price
shocks impose no expected utility loss, so the MM would absorb LT1's shares at the
efficient price. The model becomes interesting precisely because the MMs (and the
liquidity traders) are risk-averse. Bearing price risk now carries a utility
penalty, and no one will hold risky inventory without compensation. That
compensation is the liquidity premium / discount: LT1 must sell below the
asset's expected value to induce the MMs to take the shares.

```
   t=1                         t=2                         t=3
 LT1 wants to                LT2 arrives,               asset value
 sell i units               clears imbalance            S3 realised
     |                           |                           |
     v                           v                           v
 MMs absorb part            inventories                  no inventory
 at S1 < E[S2]=mu           return to 0 at S2=mu         left to bear
     |                           ^
     |   MMs warehouse risk      |
     +---- over [t=1, t=2] -------+
        (compensated by |S1 - mu|)
```

The wedge between the trade price S1 and the efficient price mu is the cost of
immediacy LT1 pays for not having to wait for the natural counterparty.

**Source:** Cartea, Jaimungal & Penalva (2015) §2.1.1 pp.38-39

## Definition

Setup (simplified Grossman & Miller 1988): n identical MMs and three dates
t in {1,2,3}. At t=1 liquidity trader LT1 arrives holding i units it wishes to
trade; its offsetting counterparty LT2 (holding -i) does not arrive until t=2.
All agents start with cash W0; MMs initially hold no assets.

- **Terminal value:** S3 = mu + e2 + e3, with mu constant and e2, e3 independent
  N(0, sigma^2) shocks publicly announced just before t=2 and t=3 respectively.
- **Preferences:** all agents (MMs and LTs) are risk-averse with CARA utility
  U(X) = -exp(-gamma X), gamma > 0 the risk-aversion parameter.
- **Frictions:** no trading costs and no direct inventory-holding costs in the
  baseline; the only friction is the price risk of carrying inventory across the
  arrival gap.

The MM is a risk-averse intermediary with zero intrinsic asset demand who supplies
immediacy by holding inventory between LT1's and LT2's arrivals. The **liquidity
premium** is the compensation |S1 - mu| per share she earns for bearing that
interim price risk. The textbook states it directly: the model "describes how
MMs obtain a liquidity premium from liquidity traders that exactly compensates
MMs for the price risk of holding an inventory of the asset until they can unload
it later to another liquidity trader."

**Source:** Cartea, Jaimungal & Penalva (2015) §2.1.1 pp.38-39

## Mathematical Reasoning

Solve backwards. Under CARA + normality, the expected utility of an agent exiting
date t=2 with cash X2 and inventory q2 is monotone in the certainty-equivalent
X2 + q2 E[S3 | e2] - (1/2) gamma sigma^2 q2^2. Maximising over q2 gives every
agent the same optimal holding

    q2* = ( E[S3 | e2] - S2 ) / ( gamma sigma^2 ).

Market clearing at t=2 requires total demand to equal total supply. The net asset
stock entering t=2 is the i units LT1 brought minus the i units LT2 wants
(n q2^MM + q2^LT1 + q2^LT2 = i - i = 0), so the aggregate inventory is zero. With
all q2* equal and summing to zero, each q2* = 0 and the price equals fundamental
value: S2 = E[S3 | e2] = mu + e2. At t=2 the imbalance is resolved, no one holds
risky inventory, and price is efficient.

At t=1 the participating agents (the n MMs and LT1) anticipate exiting t=2 with
zero inventory (so X3 = X2), and the analogous first-order condition yields a
common per-agent holding. Imposing t=1 clearing — where LT1 brings i units and
each MM brings 0 — gives the equilibrium

    S1 = mu - gamma sigma^2 * i / (n + 1).

In equilibrium each of the n MMs and LT1 holds q1* = i / (n+1) units: LT1 sells
only n/(n+1) of i immediately and retains 1/(n+1) to unload later. The liquidity
discount is

    | S1 - mu | = | gamma sigma^2 * i / (n + 1) |.

Comparative statics (all intuitive):
- increasing in |i| (size of the liquidity shock),
- increasing in gamma (risk aversion),
- increasing in sigma^2 (asset volatility),
- decreasing in n (competition among MMs).

As n -> infinity the liquidity premium -> 0, S1 -> mu (the efficient price), and
LT1's executed quantity converges to its full liquidity need i. If LT1 instead
wants to buy (i < 0), the sign flips: MMs earn a premium |S1 - mu| when selling.

**Source:** Cartea, Jaimungal & Penalva (2015) §2.1.1 pp.38-41

## Boundary Notes

- **Risk aversion is essential.** With risk-neutral MMs (gamma = 0), mean-zero
  shocks impose no utility cost, the premium vanishes, and trade clears at mu.
  The premium is purely a risk-bearing rent, not an adverse-selection or
  order-processing cost — there is no asymmetric information in this model.
- **Deterministic counterparty arrival.** The baseline assumes LT2 arrives for
  sure at t=2; immediacy demand is modelled, but search/arrival uncertainty is
  abstracted away. The model "avoids looking into the details" of how orders
  actually meet.
- **No explicit frictions in the core result.** Trading fees and inventory carry
  costs are zero here; the discount is driven solely by price-risk warehousing.
  The follow-on §2.1.2 reintroduces participation cost c (which lowers n and thus
  raises the premium) and per-trade trading costs.
- **Premium vs discount sign.** |S1 - mu| is a discount when LT1 sells (i>0) and a
  premium when LT1 buys (i<0); the absolute compensation per share is symmetric.
- **Contrast with information-based and dynamic inventory models.** This static,
  symmetric-information story isolates inventory risk; it contrasts with
  adverse-selection spreads and with fully dynamic limit-order inventory models
  (e.g. Avellaneda-Stoikov) that track a continuously controlled inventory path.

**Source:** Cartea, Jaimungal & Penalva (2015) §2.1.1-2.1.2 pp.38-42

## See Also

- [`mt-dealer-inventory-problem-spread`](./mt-dealer-inventory-problem-spread.md) -- inventory risk as a driver of the bid-ask spread, the dealer-side analogue
- [`mt-avellaneda-stoikov-market-making`](./mt-avellaneda-stoikov-market-making.md) -- dynamic continuous-time inventory control extending this static intuition
- [`mt-liquidity-premium-asset-pricing`](./mt-liquidity-premium-asset-pricing.md) -- how liquidity premia feed into expected asset returns
- [`mt-inventory-prices-competitive-markets`](./mt-inventory-prices-competitive-markets.md) -- competitive-market inventory and price formation

## Escalate to Raw When

This card sketches the backward-induction solution but compresses the algebra. Go
to Cartea, Jaimungal & Penalva (2015) §2.1.1 (pp.38-41, including Figure 2.1) for:
the full CARA + normality certainty-equivalent derivation, the explicit t=2 and
t=1 first-order conditions and clearing equations, and the exact statement of the
equilibrium holdings q* = i/(n+1). For the endogenous-competition extension (how
participation cost c determines n and thus scales the premium) and the addition of
activity-dependent trading costs, re-read §2.1.2 (pp.41 onward).
