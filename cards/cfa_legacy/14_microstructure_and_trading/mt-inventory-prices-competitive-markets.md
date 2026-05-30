---
schema_version: "cacg.v0"
id: "mt-inventory-prices-competitive-markets"
title: "Prices and Inventories in Competitive Dealer Markets"
reading_id: "14_microstructure_and_trading"
summary: "When dealers compete, inventory still shifts a dealer's quotes intertemporally but is driven toward second-best pricing; the spread compensates for the cost of holding undesired inventory rather than monopoly rent, and a gravitational pull keeps it positive."
tags: ["microstructure", "inventory-model", "dealer-competition", "bid-ask-spread", "ho-stoll"]
citations:
  - source_id: "mt_ohara_1995_market_microstructure_theory_en"
    chunk_id: "mt_ohara_1995_market_microstructure_theory_en:p057:0074"
    chunk_hash: "117a6077c5ea794909033a83417f41d6728b9d2ccef5a1b9d670945d8f65bdce"
    page_range: [57, 57]
    quote: "the dealer does not have an exclusive franchise on clearing the order flow"
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p027:0034"
    chunk_hash: "053bbf912c7f45f6406294a554b5486cc7fc251078dd96a415be97ae051eef96"
    page_range: [28, 28]
    quote: "the management of inventory risk is a major determinant of bid and ask prices"
    edge_type: "supports"
---
# Prices and Inventories in Competitive Dealer Markets

## Intuition

In the single-dealer inventory models, a monopolist specialist provides immediacy and
extracts a spread that partly reflects market power. Real markets, however, usually have
*multiple* liquidity suppliers — competing dealers, an interdealer market, or a limit-order
book — so no one dealer has a franchise on clearing the order flow. The natural question is:
once competition strips out the monopoly rent, does inventory still matter, and does the
spread survive?

O'Hara's answer (via Ho and Stoll [1983]) is yes on both counts, but with a changed economic
content. Inventory still shifts each dealer's *reservation* prices: a dealer who is already
long the asset is reluctant to buy more, so he lowers both his bid and ask, and conversely
when short. But because traders route to whoever quotes best, the *posted* market quote is no
longer a single dealer's reservation price — it is bid down to the *second-best* dealer's
reservation price (a Dutch/second-price-auction logic). Competition thus erodes the monopoly
markup, yet a strictly positive spread persists because of a "gravitational pull": as the
spread narrows, filling an order pushes a dealer's inventory away from its preferred level,
which immediately worsens (widens) his subsequent quotes.

```
   Inventory shifts the QUOTE LEVEL, not the spread WIDTH
   ------------------------------------------------------
   long dealer    |--bid--ask--|         (both prices shifted DOWN)
   balanced       |    --bid--ask--|
   short dealer   |        --bid--ask--|  (both prices shifted UP)
                  +------------------------------> price
   Competition: posted quote = SECOND-best reservation price
   #dealers = 2  -> spread CAN EXCEED monopoly spread
   #dealers = 3  -> spread = monopoly-spread benchmark
   #dealers > 3  -> spread CAN FALL BELOW it
```

**Source:** O'Hara (1995) Ch.2 §2.4 "Prices and Inventories in Competitive Markets" pp.44-50.

## Definition

Setup (Ho and Stoll [1983], as presented by O'Hara). Two risk-averse competing market makers
each quote a *buying fee* and a *selling fee* on each of two stocks (M and N) to maximize
expected utility of terminal wealth; they may trade either with the public or with one another
in an *interdealer market*. The model is solved in a one-period form (the intertemporal
inventory dimension and explicit game-theoretic strategy are suppressed), with perfect
information about each dealer's inventory and wealth.

Key objects:
- A dealer's **reservation buying/selling fee**: the price at which his trading utility is no
  lower than not trading at all.
- A dealer's **overall inventory position** in a stock, which depends not only on that stock's
  holdings but, through return covariance, on his holdings of the other stock.
- The **market (posted) quote**: the best bid/ask actually shown to traders, determined by
  competition among dealers, distinct from any single dealer's reservation price.

**Source:** O'Hara (1995) Ch.2 §2.4 pp.48-49.

## Mathematical Reasoning

Write a dealer's relevant inventory in stock M, in own-and-cross-variance-adjusted form, as
an *effective inventory* I_M that blends the direct holding M with the holding N in the
correlated stock:

```
   I_M  =  M  +  (sigma_NM / sigma_M^2) * N
```

so a position in N that is positively correlated with M effectively augments the dealer's M
exposure. A dealer's reservation buying and selling fees are linear in this effective
inventory and in a risk-aversion-scaled term; their *difference* (the dealer's own spread) is

```
   s  =  sigma_M^2 * R * Q
```

where R is the (discounted) coefficient of absolute risk aversion and Q a fixed per-trade
transaction value. Two comparative-statics facts follow directly:

1. **Inventory relocates, it does not widen.** I_M enters the *placement* (the midpoint /
   level) of the bid and ask, but cancels out of the *difference* s. So a dealer's own spread
   is governed by risk aversion R and return variance sigma_M^2, while inventory only slides
   both quotes up or down together. (Hence diversification across stocks does not change s —
   the spread is independent of the number of stocks traded.)

2. **Competition sets the posted spread by second-best pricing.** Each dealer quotes just
   inside the *next* dealer's reservation price (an epsilon improvement), so the *market*
   spread tracks the second-best reservation price, not the winner's own s. This makes the
   posted spread a function of how many competitors there are. O'Hara reports Ho-Stoll's
   count-comparative-static:

```
   N_dealers = 2 :  posted spread  >  sigma_M^2 R Q   (each side acts as a local monopolist)
   N_dealers = 3 :  posted spread  =  sigma_M^2 R Q
   N_dealers > 3 :  posted spread  <  sigma_M^2 R Q
```

The reason the spread does not collapse to zero even with many dealers is the **gravitational
pull**: a sale to a dealer raises his inventory, which lowers his bid and thereby re-widens
the spread; as the market spread narrows, order flow gravitates toward the dealers, worsening
the best bid and ask. This is the same mechanism O'Hara identifies in the Cohen-Maier-
Schwartz-Whitcomb limit-vs-market-order model, and it ties the surviving spread to the *supply
of liquidity* rather than to monopoly power.

(No worked arithmetic is given here, consistent with the qualitative model; the equation
labels are reconstructed in clean notation — the OCR garbles the original (2.49)-(2.50).)

**Source:** O'Hara (1995) Ch.2 §2.4 pp.48-50; Foucault, Pagano, and Röell (2013) Ch.3 §3.5
restate the risk-averse competitive-dealer inventory pricing of Stoll (1978).

## Boundary Notes

- **One period, no genuine dynamics or game.** O'Hara stresses that Ho-Stoll *do not actually
  show that the conjectured equilibrium occurs*: a one-period model cannot carry the dynamic
  gravitational-pull effect, and a true demonstration would need a formal game-theoretic
  structure absent here. The model also omits expectations over rivals' actions and rivals'
  inventories from each dealer's pricing function — interactions that a multi-period or
  game-theoretic treatment would reintroduce.
- **Inventory paradigm, not information paradigm.** Here the spread is a *risk-aversion /
  inventory-holding-cost* phenomenon, and order flow is by assumption uncorrelated with the
  asset's future value. This is the opposite assumption from the adverse-selection models
  (Glosten-Milgrom, Kyle) in later chapters, where the spread survives even with risk-neutral,
  competitive dealers because order flow carries private information. Do not conflate the two
  spread sources.
- **Long-run irrelevance of inventory.** Because the inflows/outflows are value-unrelated, the
  dealer's price impact is transitory: prices revert to "true" levels once order flow
  balances. The inventory effect therefore characterizes only the *short run*.
- **Sibling contrast.** The Grossman-Miller framing reaches a similar "competition prices out
  monopoly rent, but an inventory/immediacy premium remains" conclusion; the dealer-inventory-
  problem card supplies the single-dealer benchmark spread this competitive model erodes.

**Source:** O'Hara (1995) Ch.2 §2.4 pp.49-50 and §2.5 pp.51 (long-run reversion); Ch.3
(information-based contrast).

## See Also

- [`mt-dealer-inventory-problem-spread`](./mt-dealer-inventory-problem-spread.md) — the
  single-dealer inventory/spread benchmark that competition here erodes via second-best pricing.
- [`mt-grossman-miller-inventory-liquidity-premium`](./mt-grossman-miller-inventory-liquidity-premium.md)
  — sibling competitive-market result where inventory survives as a liquidity premium.

## Escalate to Raw When

O'Hara states the Ho-Stoll reservation-fee equations ((2.49)-(2.50)), the effective-inventory
expression I_M, the spread s = sigma_M^2 R Q, and the dealer-count comparative statics; the OCR
garbles these formulae. Re-read O'Hara (1995) pp.48-49 (PDF pages 57-58) for the exact algebraic
forms, and pp.49-50 (PDF pages 58-59) for the second-best-pricing / interdealer gravitational-
pull argument and O'Hara's caveat that no formal equilibrium is proven.
