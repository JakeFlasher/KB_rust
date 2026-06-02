---
schema_version: "cacg.v0"
id: "mt-volume-critique-time-clock"
title: "The Volume Critique and the Role of Time in Price Adjustment"
reading_id: "14_microstructure_and_trading"
summary: "Under event uncertainty, trade volume and the timing of trades carry information beyond price; busier order flow signals that news exists, so high-volume markets impound information more slowly per trade."
tags: ["microstructure", "volume", "trade-time", "event-uncertainty", "price-adjustment"]
citations:
  - source_id: "mt_ohara_1995_market_microstructure_theory_en"
    chunk_id: "mt_ohara_1995_market_microstructure_theory_en:p179:0232"
    chunk_hash: "fde332bf70f61ccbcc39c1ae961215b07669df32365cf11a843dec5e4d84c8e3"
    page_range: [179, 179]
    quote: "while trades provide signals of the direction of any new information, the lack of trade provides a signal of the existence of any new information"
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p107:0170"
    chunk_hash: "065f1da2018f31d795d3105f078dd78af964ffe0fc19aab9ec34b3bc8265a164"
    page_range: [108, 108]
    quote: "dealers learn not only from the order flow but also from trading volume"
    edge_type: "supports"
card_hash: "280174db5caebdf884c124058801bf05185afd8367e5782cb29f7692a3adeaae"
---
# The Volume Critique and the Role of Time in Price Adjustment

## Intuition

Folklore says "it takes volume to move prices," and empirically the absolute size of price changes is strongly linked to trading volume. Yet the canonical microstructure models leave no room for this link. In Kyle's batch auction the informed trader rescales her order to keep the same fraction of order flow, so the equilibrium price path is invariant to the *scale* of volume. In the Glosten-Milgrom sequential-trade model the probability of facing an informed counterparty is constant across time, so the *cumulative* number of trades moves the price but volume *per se* carries no extra signal — all information is already in each individual trade. This is O'Hara's "volume critique": the strong empirical price-volume relation is not explained by the workhorse theories.

The resolution is to let *time itself* become informative. The standard models implicitly treat calendar time as exogenous to the value process: a no-trade interval can have no content because, by assumption, an information event has already occurred. Easley and O'Hara break this by introducing *event uncertainty* — with probability `α` an information event happens, and with probability `1-α` nothing new exists at all. Now uninformed traders do not even know whether there is news to trade on, and the rate at which trades arrive becomes a signal of whether news exists.

```
   trade arrives  -->  signals DIRECTION of news (buy=high, sell=low)
   NO trade       -->  signals news LESS LIKELY to exist (event uncertainty)

   busy clock (high volume)  -->  P(information event) UP   -->  trade-by-trade
                                                                 content per trade DOWN
   quiet clock (low volume)  -->  P(information event) DOWN  -->  spread narrows toward V*
```

The punchline is a refined adage — *unexpected* volume moves prices — and a striking cross-sectional prediction: markets with higher normal volume adjust to information *more slowly* per trade, because in a busy market any single trade is less surprising and so less informative.

**Source:** O'Hara (1995) §6.2 The Volume Critique and §6.3 The Role of Time in Price Adjustment pp.160-177.

## Definition

Take a Glosten-Milgrom-style sequential-trade market over discrete intervals `t = 1, 2, …, T`, each long enough for at most one trade, with a competitive risk-neutral market maker who posts bid and ask equal to conditional expectations given trade type and public history.

- **Event uncertainty.** An information event occurs with probability `α > 0`. Conditional on an event, the signal is low `L` with probability `δ` or high `H` with probability `1-δ`. Prior expected value is `V*`.
- **Trader composition.** Informed traders (present only if an event occurred) are risk-neutral and trade to maximize expected profit, taking a fraction `μ` of trades conditional on an event; the limiting case `μ = 1` (all trades informed given an event) is admissible. Uninformed traders split into noise traders (exogenous liquidity) and price-sensitive traders who may decline the quote.
- **No-trade outcome.** Because an informed trader always transacts when the price is off the full-information level, a no-trade interval can only come from an uninformed trader who checks the quote and walks away. The probability of no-trade differs between the event state and the no-event state, and *that divergence is what makes the absence of trade informative.*
- **Time as endogenous.** Once the inter-trade interval is correlated with the underlying information structure, time is no longer exogenous to the price process; the sequence of trades *and* no-trades — i.e., realized volume — enters the market maker's belief updating.

**Source:** O'Hara (1995) §6.3 pp.168-176; Foucault, Pagano, and Röell (2013) §3.7 p.108.

## Mathematical Reasoning

Let `H_t` be the public history through interval `t`. The market maker Bayes-updates on the *type* of the interval, where the type is now richer than {buy, sell}: it is {buy, sell, no-trade}.

Conditional no-trade probabilities (in clean reconstructed notation; let `γ` be the uninformed potential-seller fraction and `ε_B, ε_S` the probabilities an uninformed buyer/seller actually trades):

- If an event occurred: `P(no-trade | event) = (1-μ)·[ γ(1-ε_S) + (1-γ)(1-ε_B) ]`.
- If no event occurred (all traders uninformed): `P(no-trade | no-event) = γ(1-ε_S) + (1-γ)(1-ε_B)`.

Since `(1-μ) < 1`, no-trade is strictly *more* likely when there is no news. Observing no-trade therefore raises the posterior on "no event" and lowers the posterior on both `L` and `H` proportionally — the *relative* odds of high vs. low are unchanged, but their *absolute* mass shrinks. Belief mass migrates toward the prior `V*`.

Comparative statics that follow:

- **Quote response to no-trade.** With less perceived adverse-selection risk, the maker pulls quotes toward `V*`: if bid/ask were above `V*` they fall, if below they rise, and the *spread narrows* after a no-trade interval. (Contrast Diamond-Verrecchia below, where no-trade is unambiguously bad news.)
- **Volume → existence-of-news inference.** Higher realized volume ⇒ fewer no-trade intervals ⇒ higher posterior that an event exists. Hence cumulative volume to time `t` shifts where the price goes at `t+1` — volume is informative *because* it is correlated with the information structure, unlike the standard sequential model where volume adds nothing beyond individual trades.
- **Speed-of-adjustment / cross section.** In a high-normal-volume market each trade is less surprising, so the per-trade information content is lower and full-information convergence is *slower*: high-liquidity markets look "less efficient" in trade time even as they converge in the limit. (O'Hara contrasts this with the Chapter-3 Appendix entropy rate and Diamond-Verrecchia's first-passage-time measure of adjustment speed.)
- **Econometric bias.** Transaction prices here are martingales but **not Markov**: the transaction series is an optional sampling of the latent value process where sampling intensity rises with information (informed traders trade more), so price moves at `t+1` are not independent of preceding trades, and transaction-data variances are biased upward and inversely correlated with volume.

No worked arithmetic is given; the content is the sign and direction of these updates.

**Source:** O'Hara (1995) §6.3 pp.174-177.

## Boundary Notes

- **Where it holds.** Requires (i) a trade-by-trade venue where no-trade intervals are *observable* — batch / rational-expectations auctions obliterate inter-trade timing and so cannot host this mechanism; and (ii) genuine event uncertainty (`α < 1`). If an event is assumed certain (`α = 1`, the plain Glosten-Milgrom case), no-trade carries no content and time collapses back to exogenous.
- **Where it breaks vs. siblings.** In **Kyle** the linear price-volume relation is built in but volume does not affect the price *path's* scale-invariance, so Kyle cannot generate the empirical volume effect. In **plain Glosten-Milgrom** the constant informed-trade probability removes any role for volume beyond the running trade count.
- **Contrast — Diamond-Verrecchia (1987).** They make time informative via *short-sale constraints*: an absence of trade correlates with constrained selling on bad news, so no-trade is read as bad news, prices fall, and the spread *widens*. Easley-O'Hara's event-uncertainty channel instead reads no-trade as "news less likely," pulling prices toward `V*` and *narrowing* the spread. Same headline ("time matters") but opposite spread response — the difference is whether the friction is a directional trading constraint or symmetric event uncertainty.
- **Scope.** This card is the theory foundation only; the empirical operationalization (PIN estimation from buy/sell/no-trade counts) lives in the PIN card.

**Source:** O'Hara (1995) §6.3 pp.168-177; Foucault, Pagano, and Röell (2013) §3.7 p.108.

## See Also

- [`mt-pin-probability-informed-trading`](./mt-pin-probability-informed-trading.md) — turns this event-uncertainty / no-trade structure into the estimable Probability of Informed Trading.
- [`mt-glosten-milgrom-adverse-selection`](./mt-glosten-milgrom-adverse-selection.md) — the base sequential-trade model this extends; supplies the constant-informed-probability benchmark the volume critique attacks.
- [`mt-kyle-lambda-market-depth-price-impact`](./mt-kyle-lambda-market-depth-price-impact.md) — the batch-auction model whose scale-invariance is the other half of the volume critique.
- [`mt-random-walk-efficient-price`](./mt-random-walk-efficient-price.md) — why transaction prices remain martingales here yet fail the Markov property.

## Escalate to Raw When

O'Hara works the Bayesian quote-revision algebra, the explicit no-trade probabilities, and the Diamond-Verrecchia first-passage-time and Chapter-3-Appendix entropy speed-of-adjustment measures with full formal detail; the OCR garbles those equations (Greek letters and subscripts are mangled). Re-read **O'Hara (1995) §6.3 pp.174-177** (Easley-O'Hara event-uncertainty derivation, no-trade probabilities, martingale-not-Markov sampling argument) and **pp.168-173** (Diamond-Verrecchia short-sale-constraint variant and first-passage-time metric) for the exact derivations before relying on any equation reconstructed above.
