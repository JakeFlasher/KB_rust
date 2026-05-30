---
schema_version: "cacg.v0"
id: "mt-dealer-inventory-problem-spread"
title: "The Dealer's Inventory Problem: Bid-Ask Spread as Inventory-Risk Compensation"
reading_id: "14_microstructure_and_trading"
summary: "Inventory models (Garman, Stoll, Ho-Stoll, Amihud-Mendelson) derive the bid-ask spread as compensation a risk-averse dealer demands for bearing the position risk of holding a suboptimal portfolio, independent of asymmetric information."
tags: ["microstructure", "inventory-model", "bid-ask-spread", "dealer", "risk-aversion"]
citations:
  - source_id: "mt_ohara_1995_market_microstructure_theory_en"
    chunk_id: "mt_ohara_1995_market_microstructure_theory_en:p037:0046"
    chunk_hash: "cec1ec3600216925933a2651911308d69f604e01341732d8158068d836d42cc9"
    page_range: [37, 37]
    quote: "A large (positive) inventory causes the dealer to face a higher cost for absorbing more inventory, and this increased cost lowers both bid and ask prices by the same amount."
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p081:0114"
    chunk_hash: "045ebd9bccccfabff584bdfc2aecca54d71b050ec93f057f9b487c295a947bbb"
    page_range: [81, 81]
    quote: "If the dealer is risk averse, his quotes should depend directly on his exposure to inventory"
    edge_type: "supports"
---
# The Dealer's Inventory Problem: Bid-Ask Spread as Inventory-Risk Compensation

## Intuition
Strip away private information for a moment. Even when no trader knows more than the
dealer, a positive bid-ask spread still appears — because the dealer is an ordinary
*risk-averse* portfolio holder who is repeatedly asked to take the *other side* of
whatever the crowd wants to trade. To supply immediacy, the dealer must accept a
position that pushes the dealer away from the desired (efficient) portfolio, and bearing
that exposure is costly. The spread is the price of that service: the dealer quotes a low
bid and a high ask so that the round-trip compensates the dealer for accepting a
suboptimal, risky inventory.

A useful mental picture separates two distinct things the dealer controls: the *width* of
the spread and its *placement* relative to the dealer's view of true value.

```
        Stoll / Ho-Stoll inventory dealer

   true value P*
        |
   bid  |  ask              <- spread WIDTH set by risk aversion z,
   <----+---->                 stock variance sigma^2, trade size |Q|, wealth W0
        |
  -------------------------------------------------
  long inventory  =>  shift BOTH quotes DOWN  (entice sells, discourage buys)
  short inventory =>  shift BOTH quotes UP    (entice buys,  discourage sells)
        ^
        |
   PLACEMENT moves with current inventory; WIDTH (ideally) does not.
```

So inventory affects *where* the dealer hangs the spread, not necessarily *how wide* it
is. A dealer who is long shades both quotes down to attract sellers' counterparties and
unload; a dealer who is short shades both quotes up. The width itself is the risk premium
demanded for the act of intermediation.

**Source:** O'Hara (1995) ch.2 §2.2 (The Dealer's Problem), pp.25-28.

## Definition
**Inventory paradigm.** A class of microstructure models in which the bid-ask spread and
quote dynamics arise from a dealer managing the *position risk* of holding inventory and
absorbing imbalances in order flow — as opposed to the information paradigm, where the
spread arises from trading against better-informed counterparties.

**Three strands (O'Hara's taxonomy).**
1. **Order-flow / ruin (Garman 1976).** A monopolistic, risk-neutral market maker faces
   stochastic (Poisson) buy and sell arrivals and must avoid "failure" — running out of
   either cash or stock. The spread protects against the bankruptcy/ruin probability.
2. **Dealer's optimization (Stoll 1978; Ho-Stoll 1981).** The market maker is a
   risk-averse portfolio holder ("supplier of immediacy") who sets quotes to maximize
   expected utility of wealth; the spread is compensation for the *costs* of providing
   immediacy.
3. **Inventory-control / market power (Amihud-Mendelson 1980).** A risk-neutral
   monopolist sets prices that depend on inventory, with the spread reflecting market
   power rather than risk-bearing.

**Stoll's three cost sources of immediacy.** (i) *Holding (inventory) costs* — the
exposure risk of carrying a suboptimal portfolio, which matters precisely because the
dealer is risk-averse; (ii) *order-processing costs* — exchange fees, transfer taxes, a
fixed fee per transaction; (iii) *adverse-information costs* — the cost of trading with
someone who knows more (treated only in a limited way here; the focus of later
information models).

**Source:** O'Hara (1995) ch.2 §§2.1-2.2, pp.13-29.

## Mathematical Reasoning
Reconstructing Stoll's (1978) two-date setup in clean notation (the OCR garbles the
algebra). Let the dealer have initial wealth `W0`, a coefficient of relative risk
aversion `z`, and beliefs about a *fixed* true price `P*`. A trade of true dollar value
`Q` (positive = dealer buys, negative = dealer sells) moves the dealer off the efficient
portfolio. The dealer borrows/lends at the risk-free rate `Rf` and is willing to transact
only if utility is no worse than not trading:

```
        E[ U(W0 (1 + R*)) ]  =  E[ U(W) ]
```

where `W` is terminal wealth after carrying the position. Expanding both sides in a
second-order Taylor series, setting `Rf = 0`, and solving for the *percentage cost of
immediacy* `c(Q) = C/Q` gives a function of the form (schematically):

```
        c(Q)  ~  (z / W0) * [ sigma_iP * Qp  +  (1/2) sigma^2 * Q ]
```

where `Qp` is the dealer's existing total inventory (dollar value), `sigma_iP` is the
covariance/correlation of stock i's return with the efficient portfolio, and `sigma^2` is
the variance of stock i's return. Under competitive pricing the bid and ask just offset
this cost relative to `P*`, and the spread reduces (for `|Q_bid| = |Q_ask| = |Q|`) to a
linear form:

```
        (Pa - Pb)/P*  ~  (z / W0) * sigma^2 * |Q|
```

**Comparative statics (the load-bearing predictions):**
- Spread **rises** with risk aversion `z` and with stock variance `sigma^2`.
- Spread **falls** with dealer wealth `W0` (more capacity to absorb risk).
- Spread **increases linearly** in trade size `|Q|`.
- Crucially, **inventory `Qp` does not enter the spread equation** — it enters the
  *placement*: a large positive `Qp` raises the cost of absorbing more, lowering *both*
  bid and ask by the same amount; a negative inventory shifts both up. Hence inventory
  affects placement, not width — "an important and potentially testable hypothesis."

Adding a *fixed* per-transaction order-processing fee makes per-unit processing cost
*decrease* in trade size while inventory cost *increases* in trade size, so total cost is
**U-shaped** — implying an optimal (cost-minimizing) preferred trade size.

**Ho-Stoll (1981) dynamic extension.** With a finite horizon `T`, quadratic utility, and
a certain liquidation date, the optimal spread decomposes as a *risk-neutral
(monopolistic) spread* — set by the elasticities/slopes of supply and demand, greater
elasticity narrowing it — *plus a risk adjustment* that grows with time-to-horizon, risk
aversion, and instantaneous variance. As `T -> 0` the risk adjustment vanishes and only
the risk-neutral spread remains. Two further results: transaction-arrival uncertainty per
se does *not* enter the spread (it works only indirectly through the portfolio), and the
spread remains *independent of the inventory level* — inventory again moves placement,
not width.

**No worked arithmetic is performed here** (per Critical Rule 1); only the structure,
signs, and monotonicities are stated.

**Source:** O'Hara (1995) ch.2 §2.2, pp.35-44 (Stoll 1978 cost/spread eqns; Ho-Stoll
1981 dynamic properties).

## Boundary Notes
- **What makes the spread positive here:** risk aversion plus inability to hedge/diversify
  inventory. If the dealer were risk-neutral or could fully diversify, the immediacy cost
  (and the inventory-driven spread) would collapse toward zero or to order-processing
  costs alone. This is the "risk-aversion" role of the spread.
- **Contrast with siblings.** In **Garman**, the spread is partly a defense against the
  *ruin/bankruptcy* probability; in **Amihud-Mendelson**, the spread reflects *market
  power* of a risk-neutral monopolist and falls to zero under competition; in
  **Glosten-Milgrom / Kyle** (information paradigm), the spread is *adverse-selection*
  compensation that survives even with risk-neutral, competitive, zero-inventory-cost
  dealers. Inventory and information are complementary, not rival, explanations.
- **Fragile assumptions in Ho-Stoll.** A *fixed* true price `P*` is fundamental — if
  intrinsic value were itself stochastic, even approximating the solution is "formidable"
  and the fixed-spread-around-true-price result may fail. The finite horizon with certain
  liquidation injects the time effect but implies implausible deterministic intraday
  patterns (e.g., spreads widest in the morning, narrowing through the day) and that
  traders are always worse off facing a long-horizon dealer.
- **Empirical hook.** The "inventory moves placement, not width" prediction is the
  testable signature examined by Hasbrouck (1988) and others; it distinguishes inventory
  effects from information effects in quote-revision data.

**Source:** O'Hara (1995) ch.2 §2.2, pp.28-29, 36-37.

## See Also
- [`mt-garman-order-flow-temporal-microstructure`](./mt-garman-order-flow-temporal-microstructure.md) -- the ruin/order-flow strand this card builds on.
- [`mt-inventory-prices-competitive-markets`](./mt-inventory-prices-competitive-markets.md) -- the market-power (Amihud-Mendelson) strand contrasted above.
- [`mt-glosten-milgrom-adverse-selection`](./mt-glosten-milgrom-adverse-selection.md) -- the information-paradigm sibling where the spread survives without inventory costs.
- [`mt-dealer-inventory-control-price-discovery`](./mt-dealer-inventory-control-price-discovery.md) -- how inventory-driven quote placement feeds price discovery.

## Escalate to Raw When
O'Hara derives the Stoll cost function via an explicit second-order Taylor expansion of
the expected-utility indifference condition and solves the Ho-Stoll dynamic program by a
first-order approximation under quadratic utility — the OCR garbles every equation
(eqns 2.12-2.16 around pp.35-37, and the Ho-Stoll value-function recursion around eqn
2.23, pp.42-44). Re-read O'Hara (1995) pp.35-44 directly for the full algebra, the exact
coefficients on `z/W0`, `sigma_iP`, and `sigma^2`, and the formal proof that the spread
is inventory-independent while the placement is not.
