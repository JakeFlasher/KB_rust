---
schema_version: "cacg.v0"
id: "mt-glosten-milgrom-adverse-selection"
title: "The Glosten-Milgrom Sequential-Trade Model: Spread from Adverse Selection"
reading_id: "14_microstructure_and_trading"
summary: "Risk-neutral competitive dealers post regret-free bid/ask quotes equal to the asset's conditional expected value given a sell or buy, so the spread is pure adverse-selection compensation and transaction prices form a martingale."
tags: ["microstructure", "adverse-selection", "bid-ask-spread", "sequential-trade", "bayesian-learning", "martingale"]
citations:
  - source_id: "mt_ohara_1995_market_microstructure_theory_en"
    chunk_id: "mt_ohara_1995_market_microstructure_theory_en:p069:0092"
    chunk_hash: "656991e69e278a72477ca30277d4f803d991c259d7f0a6851ac112f239638aa0"
    page_range: [69, 70]
    quote: "prices are set equal to the specialists' conditional expectation of the asset's value given the type of trade that occurs"
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p084:0122"
    chunk_hash: "3b16e8a65cfbdc686906cdbe8cc37e75647f307add042e7fa0fea3792f5832ff"
    page_range: [84, 84]
    quote: "In this model the ask price exceeds the bid because the former is set in anticipation of receiving a buy order, the latter a sell order"
    edge_type: "supports"
---
# The Glosten-Milgrom Sequential-Trade Model: Spread from Adverse Selection

## Intuition
Glosten and Milgrom (1985) ask a deceptively simple question: why does a bid-ask spread
exist even when the dealer bears no inventory cost, no order-processing cost, and faces no
taxes or commissions? Their answer is that the spread is the price the dealer charges to
protect against *informed* counterparties. Some traders know the asset's true value V; the
rest (liquidity traders) trade for exogenous reasons. The dealer cannot tell who is who on
any single trade, but every incoming order is a *signal*: a buy nudges the dealer toward
believing the news is good, a sell toward believing it is bad. The dealer therefore quotes an
ask above his current estimate (to brace for the chance the buyer knows good news) and a bid
below it (to brace for the chance the seller knows bad news).

Because dealers are risk-neutral and compete, expected profit on every trade is bid down to
zero. The surviving quotes are *conditional expectations*: the ask equals the expected value
of V given that a buy arrives, the bid equals the expected value given that a sell arrives.
O'Hara stresses that these are "regret-free" prices — given the trade that actually occurred,
the dealer already considers the executed price fair. The spread is thus not a markup over a
known value; it is the gap between two conditional expectations that already embed the
information a trade reveals.

```
              ask = E[V | Buy]   <-- braces for an informed buyer (good news)
   spread  {  ----------------
              bid = E[V | Sell]  <-- braces for an informed seller (bad news)

   wider spread  <==  more informed traders / more uncertain V
   narrower      <==  mostly liquidity traders / well-pinned V
```

As trades accumulate, the preponderance of orders on one side teaches the dealer the true
value; quotes converge to full-information levels and the spread shrinks. The mechanism is
Bayesian learning embedded directly in price-setting.

**Source:** O'Hara (1995) §3.3 "The Glosten-Milgrom Model" pp.58-66 (PDF pp.67-75).

## Definition
Setup. A single asset has terminal value given by a random variable V. In the binary
illustration, V takes a low value V_L or a high value V_H. Trade is *sequential*: at each
date one trader, drawn probabilistically from a fixed pool, transacts exactly one unit at the
dealer's posted bid or ask, then returns to the pool. A trader is **informed** (knows V) with
probability mu and **uninformed / liquidity** with probability 1 − mu. Informed traders buy
only on good news and sell only on bad news; uninformed traders buy or sell for reasons
exogenous to the model.

Dealer. The market maker (and all participants) are risk-neutral and competitive, with
unlimited capital, no bankruptcy, and a short horizon — assumptions that make inventory
*irrelevant by construction*, isolating the information effect. Competition forces the
**zero-expected-profit condition** on every trade: any rent would be bid away by a rival
dealer quoting the same prices (two competing dealers suffice).

Regret-free quotes. Let B_t denote the event "a buy arrives at date t" and S_t "a sell
arrives at date t", with information set I_{t-1} carried in from prior trades. The equilibrium
quotes are conditional expectations:

- Ask: a_t = E[V | I_{t-1}, B_t]
- Bid: b_t = E[V | I_{t-1}, S_t]

The bid is the dealer's expected value of the asset given that a trader wants to *sell* to
him; the ask is the expected value given that a trader wants to *buy* from him.

**Source:** O'Hara (1995) §3.3 pp.58-62 (PDF pp.67-71); Foucault, Pagano & Röell (2013)
§3.3 pp.84-85.

## Mathematical Reasoning
Posterior beliefs (binary case). The dealer updates the probability that V = V_H by Bayes'
rule. Writing theta for the prior P(V = V_H) and treating a sell S as the conditioning event:

```
                       P(V_H) * P(S | V_H)
   P(V_H | S)  =  ------------------------------------------------
                  P(V_H) * P(S | V_H)  +  P(V_L) * P(S | V_L)
```

with the buy posterior P(V_H | B) defined symmetrically. The quotes follow as the
expectations a_t = E[V | B_t] and b_t = E[V | S_t], computed from these posteriors. After the
realized trade, the posterior replaces the prior and the next period's quotes are recomputed —
beliefs (and hence prices) evolve as a Bayesian filter on the order flow.

Why buys and sells are not equally likely. Using the probability tree (nature picks
good/bad news with probabilities theta / 1 − theta; a fraction mu of traders are informed;
uninformed buy/sell with probabilities gamma_B, gamma_S), the unconditional trade
probabilities are, in O'Hara's notation:

- P(Sell) = (1 − mu) * gamma_S  +  (1 − theta) * mu
- P(Buy)  = (1 − mu) * gamma_B  +  theta * mu

In each line the first term is the contribution of uninformed flow and the second the
contribution of informed flow. Whenever prices are away from full-information levels, the two
sides are *unequal* — good news tilts flow toward buys, bad news toward sells — so the naive
"buys = sells" assumption fails.

Spread and comparative statics. The spread a_t − b_t = E[V | B_t] − E[V | S_t] is strictly
positive whenever the order flow carries information. Qualitatively (O'Hara establishes the
direction; the algebra is in the source), the spread *widens* with (i) greater dispersion in
V (more to learn), (ii) a higher informed fraction mu (more adverse selection), and shrinks
toward zero as flow becomes dominated by liquidity traders. In the limit where everyone is
informed, buys and sells become equally likely.

Martingale property. Transaction prices p_t satisfy E[p_{t+1} | I_t] = p_t — they form a
martingale with respect to the dealer's information set. Two corollaries O'Hara draws: (a)
price changes are *serially uncorrelated*, unlike the negative serial correlation induced by
inventory or order-processing costs (the basis of Roll's spread estimator, which therefore
fails under asymmetric information); and (b) prices are semi-strong-form efficient along the
path and converge to strong-form efficiency (the true V) as learning completes.

**Source:** O'Hara (1995) §3.3 pp.62-65 (PDF pp.71-74), eqs.(3.3)-(3.5) and Fig.3.1;
Foucault, Pagano & Röell (2013) §3.3 pp.85-86.

## Boundary Notes
Assumptions that make the result clean: risk-neutral, competitive, deep-pocketed dealers; one
unit per trade; probabilistic trader selection so the informed cannot trade unboundedly and
the dealer always knows the *population* mix; liquidity traders present so a Milgrom-Stokey
"no-trade equilibrium" is avoided (with purely speculative uninformed traders, they would
rationally refuse to trade against the informed).

Where it breaks. If the informed fraction mu is too large, adverse selection can force the
spread so wide that trade — and hence the price-revealing mechanism — *collapses*, the
microstructure analogue of Akerlof's lemons market. The no-inventory simplification is also a
weakness: real spreads blend adverse selection with inventory and order-processing costs, which
a richer model must combine.

Contrast with siblings. Versus **Copeland-Galai**: both yield an information spread, but in
Copeland-Galai the spread merely balances static expected gains and losses on a one-shot
option-writing problem, whereas Glosten-Milgrom makes quotes *dynamic* conditional
expectations that move with each trade. Versus **Kyle (1985)**: Glosten-Milgrom uses
sequential, unit-size, price-taking trades with quotes set as conditional expectations; Kyle
has a single strategic informed trader who chooses order *size* to disguise himself within
batched noise trading, producing a linear price-impact coefficient lambda rather than a
posted bid-ask spread.

**Source:** O'Hara (1995) §3.3 pp.59-66 (PDF pp.68-75).

## See Also
- [`mt-order-flow-information-content`](./mt-order-flow-information-content.md) -- why each trade is a signal that moves the dealer's beliefs (the premise this model formalizes).
- [`mt-bayesian-learning-price-discovery`](./mt-bayesian-learning-price-discovery.md) -- the Bayes-rule updating engine that drives convergence of quotes to full-information value.
- [`mt-kyle-strategic-informed-trader-lambda`](./mt-kyle-strategic-informed-trader-lambda.md) -- the strategic-order-size sibling model (lambda price impact) contrasted above.
- [`mt-spread-decomposition-components`](./mt-spread-decomposition-components.md) -- how the adverse-selection component sits alongside inventory and order-processing costs in observed spreads.

## Escalate to Raw When
The OCR garbles O'Hara's equations (3.3)-(3.5) and the Figure 3.1 probability tree, and only
sketches the comparative statics and the martingale/efficiency proofs. For the exact Bayesian
posterior algebra, the worked numerical example, and the formal characterization of how the
spread depends on information dispersion, informed-trader count, and demand elasticities,
re-read O'Hara (1995) §3.3 PDF pp.71-75 plus the chapter Appendix on Bayesian-learning
dynamics; for the convergence/no-trade-collapse arguments see PDF pp.73-75.
