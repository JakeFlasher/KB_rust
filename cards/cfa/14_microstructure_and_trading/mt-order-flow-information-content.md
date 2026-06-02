---
schema_version: "cacg.v0"
id: "mt-order-flow-information-content"
title: "Adverse Selection and the Information Content of Trades"
reading_id: "14_microstructure_and_trading"
summary: "A trade may signal private information or mere liquidity need; unable to distinguish, the risk-neutral market maker self-protects by revising beliefs conditional on trade direction — the core adverse-selection mechanism."
tags: ["microstructure", "adverse-selection", "order-flow", "bayesian-learning", "market-maker", "information-asymmetry"]
citations:
  - source_id: "mt_ohara_1995_market_microstructure_theory_en"
    chunk_id: "mt_ohara_1995_market_microstructure_theory_en:p066:0088"
    chunk_hash: "b11c206cc5d707395d54e6e1bfcdfe3a1f2022b7e02c3e2385387687b62290ec"
    page_range: [67, 67]
    quote: "market maker receives trades, therefore, his expectation of the asset's value"
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p013:0013"
    chunk_hash: "363bc1bc87d5e6e8820c06b60c9e435ed4cf31fdb704d414ecd8fbc101fb9442"
    page_range: [13, 13]
    quote: "avoid making a market in a situation where he might very easily lose money to informed"
    edge_type: "supports"
card_hash: "06cf4d5b99b90048e4679dc4130662ffd34ed8a40c3d51e1de0f9cd0532ab8cc"
---
# Adverse Selection and the Information Content of Trades

## Intuition

The defining move of information-based microstructure is to treat each incoming
order not as a neutral request for liquidity but as a *signal*. In a competitive
market, an agent with superior information sells when he privately knows bad
news and buys when he privately knows good news. So when someone hits the
market maker's bid, that sell could mean "I know this stock is overvalued" — or
it could just mean "I need cash and would have sold regardless." The market
maker cannot tell the two apart. Because he is exposed to the first
possibility, every trade carries an *adverse-selection* cost: on average he
transacts with informed counterparties precisely when the trade is against him.

His only defense is to learn. He cannot refuse to quote, but he can *condition*:
he treats the direction of the order as evidence about value and revises his
expectation accordingly. A buy nudges his estimate of value up (it is more
likely under good news); a sell nudges it down. The quote he posts after the
trade reflects what the trade itself just taught him.

```
   incoming order
        |
   buy? ----> P(good news | buy)  up   ----> raise expected value ----> quotes rise
   sell? ---> P(bad news  | sell) up   ----> lower expected value ----> quotes fall
        |
   (market maker cannot see WHO traded, only WHICH SIDE)
```

Over a sequence of trades, if informed agents persistently lean to one side,
their information leaks into the order flow, the market maker's beliefs migrate
toward the truth, and prices converge to full-information value. This makes the
spread and the price path *endogenous* to private information — the central
break from earlier inventory-only models, where order flow was exogenous.

**Source:** O'Hara (1995) §3.2 "The Information Content of Trades" pp.57-58.

## Definition

Setup (sequential-trade, asymmetric-information world). A single asset has an
eventual value given by a random variable `V`. A risk-neutral, competitive
market maker posts a bid and an ask before each trade. The trading population is
a mix of:

- **Informed traders**, who know (something about) `V` and trade in the
  profitable direction;
- **Liquidity (uninformed) traders**, who trade for reasons exogenous to the
  model and are equally likely to buy or sell.

Let `B` denote the event "an arriving trader buys from the market maker (lifts
the ask)" and `S` the event "an arriving trader sells to the market maker (hits
the bid)." The **information content of a trade** is the change it induces in
the market maker's conditional expectation of `V`. Adverse selection is the
expected loss the market maker bears from the chance that the counterparty is
informed; he compensates by quoting a bid below, and an ask above, his prior
mean — and by *moving* those quotes after each observed trade.

**Source:** O'Hara (1995) §3.2-§3.3 pp.57-59.

## Mathematical Reasoning

Competition plus risk neutrality forces a **zero-expected-profit** condition on
every trade: any rent would be bid away by a rival market maker. The consequence
is that quotes are set equal to conditional expectations of value given the side
of the trade — "regret-free" prices:

- Ask: `a = E[V | B]` — the value expected *given* that the counterparty chose
  to buy.
- Bid: `b = E[V | S]` — the value expected *given* that the counterparty chose
  to sell.

Because a buy is more likely when value is high and a sell more likely when
value is low, conditioning makes `E[V | B] >= E[V | S]`, so a strictly positive
spread `a - b >= 0` emerges *purely from information asymmetry* — no inventory
cost, no order-processing cost required.

The belief update is **Bayesian**. With two states (a low value `V_L` and a high
value `V_H`) and prior `Pr{V = V_H}`, the posterior after observing a sell is

```
                       Pr{V_H} * Pr{S | V_H}
  Pr{V_H | S}  =  -----------------------------------------------
                  Pr{V_H} * Pr{S | V_H} + Pr{V_L} * Pr{S | V_L}
```

with the buy-side posterior `Pr{V_H | B}` formed symmetrically. Since informed
sellers cluster under the low state, `Pr{S | V_L} > Pr{S | V_H}`, so observing
`S` lowers the posterior on `V_H` and hence lowers both quotes. The new prior
for the next round is this posterior; quotes therefore follow a martingale-like
path driven by realized order flow.

Comparative statics implied by the mechanism:

- More informed participation (a larger fraction of informed traders) widens the
  spread and accelerates belief revision per trade.
- Greater prior uncertainty about `V` widens the spread.
- The expected order flow is *unbalanced* whenever quotes are not yet at
  full-information levels: good news produces a buy-heavy flow, bad news a
  sell-heavy flow — buys and sells are not equally likely.

No worked numerals are computed here; the structure above is the model
mechanism, not an exam calculation.

**Source:** O'Hara (1995) §3.3 pp.59-62.

## Boundary Notes

- **Assumptions that make this clean:** market-maker risk neutrality, unlimited
  capital, no bankruptcy, one-unit trades, no explicit transaction or
  inventory-carrying costs, and order flow observable to all competing market
  makers. These deliberately *switch off* inventory effects so information per
  se can be isolated.
- **Where it breaks:** if inventory matters (risk aversion or capital
  constraints) or if order flow is not common knowledge, quotes can carry
  idiosyncratic, dealer-specific components and the regret-free conditional-
  expectation characterization fails. The "no-trade theorem" (Milgrom-Stokey)
  also bites: uninformed agents must have a non-speculative (liquidity) motive,
  or trade collapses entirely.
- **Contrast with siblings:** the *inventory* tradition (see the dealer-inventory
  card) treats order flow as exogenous and spreads as compensation for bearing
  position risk; the *information* tradition here makes order flow endogenous and
  spreads as compensation for adverse selection. The Glosten-Milgrom card
  formalizes the discrete sequential-trade version; the Kyle-style strand instead
  lets an informed trader optimize order size against a batch-clearing market
  maker.

**Source:** O'Hara (1995) §3.3 pp.59-62; Foucault, Pagano & Röell (2013) Introduction p.13.

## See Also

- [`mt-glosten-milgrom-adverse-selection`](./mt-glosten-milgrom-adverse-selection.md) — the discrete sequential-trade model that formalizes this signal-extraction mechanism.
- [`mt-informed-traders-price-efficiency`](./mt-informed-traders-price-efficiency.md) — how informed order flow drives convergence of prices to fundamentals.
- [`mt-dealer-inventory-problem-spread`](./mt-dealer-inventory-problem-spread.md) — the rival (inventory-cost) explanation of the spread that this card contrasts against.

## Escalate to Raw When

You need the full Glosten-Milgrom derivation rather than the sketch above. The
OCR garbles the equations (the conditional-expectation pricing relations and the
Bayes-rule posteriors render with corrupted symbols such as `a1 = E[V| B1]` and
`Pr{V = V|S1}`), so for the exact algebra re-read O'Hara (1995) §3.3 pp.59-62
(PDF pages 67-71), and the chapter Appendix on Bayesian learning dynamics
(pp.77 onward) for the formal proof that beliefs converge to full-information
value. Use the source for any claim about convergence *rates* or the precise
spread formula, which this card only describes qualitatively.
