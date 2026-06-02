---
schema_version: "cacg.v0"
id: "mt-market-quality-volatility-origins"
title: "The Origins of Volatility and Market Quality"
reading_id: "14_microstructure_and_trading"
summary: "Observed price volatility decomposes into non-reverting fundamental volatility (information) and reverting transitory volatility (bid-ask bounce, liquidity shocks); a high-quality market keeps the transitory component small."
tags: ["microstructure", "volatility", "market-quality", "transitory-volatility", "bid-ask-bounce", "liquidity"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p426:0719"
    chunk_hash: "b14ded32b61ef625de6db944ee9767479b15167ae8fa9b9347c80ddcd94054d7"
    page_range: [426, 426]
    quote: "Transitory volatility results when the demands of impatient uninformed traders cause prices to diverge from fundamental values"
    edge_type: "defines"
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p092:0122"
    chunk_hash: "bae4997381968b3c931f2bee9abdfaf8ef288418ab7853f1b09bf843be733aa5"
    page_range: [93, 93]
    quote: "the literature differentiates between fundamental volatility and microstructure noise"
    edge_type: "supports"
card_hash: "1632fe7876e7b1139fd849d383e1bf99693173204f6566fbd585064deb2f2cb6"
---
# The Origins of Volatility and Market Quality

## Intuition

When you watch a quote bounce around through the day, two completely different
forces are mixed into that wiggle. Part of the motion is the price genuinely
relocating to a new fundamental value because someone learned something — a frost
hits the Florida orange crop, a central bank surprises, an issuer's credit
deteriorates. That part of the motion *sticks*; the price has no reason to come
back. The other part of the motion is just the mechanics of trading: an impatient
buyer pays the ask, the next impatient seller hits the bid, and the print
ricochets between the two even though nothing about value changed. That part of the
motion *reverts* — value traders and arbitrageurs eventually notice the price is
off and trade it back. Harris labels these two ingredients **fundamental
volatility** and **transitory volatility**.

```
   observed price path
        |
        |        ^         <- transitory bounce around the
   P    |   ^   / \   ^        random-walk "true value" line
        |  / \ /   \ / \
        | /   v     v   \____  <- a step here = fundamental shock
        |/                  (no reversion)
        +----------------------------> time
        \__ bid/ask bounce + order-imbalance reversals __/
```

The reason this split is the heart of *market quality* is that the two components
respond to completely different levers. Regulators and exchanges cannot do much
about fundamental volatility — that is the world changing, and a market that
*hides* it just produces worse prices. But they can shrink transitory volatility,
because transitory volatility is essentially the price-footprint of liquidity
demand, and that footprint is the transaction cost uninformed traders pay. A
high-quality market is therefore one with tight spreads, low transitory
volatility, and prices that still stay informative.

**Source:** Harris (2003) Trading and Exchanges, ch.20 §20.1–20.2 pp.410–415.

## Definition

Let total (observed) volatility be the dispersion of measured price changes,
conventionally summarized by a variance, standard deviation, or mean absolute
deviation of returns.

- **Fundamental volatility** is the volatility "due to unanticipated changes in
  instrument values." Only *unexpected* changes in valuation factors move a fully
  informative price; expected changes are already incorporated, so fundamental
  price innovations are unpredictable — a random walk.
- **Transitory volatility** is the volatility "due to trading activity by
  uninformed traders." It arises when impatient, uninformed liquidity demand pushes
  the price away from fundamental value; the displacement is *temporary* and
  reverses when value traders, arbitrageurs, and dealers restore the price.
- **Bid/ask bounce** is the simplest transitory component: market orders buy at the
  ask and sell at the bid, so prints alternate across the spread even with a static
  value. The transaction-cost (transitory) component of the spread is its source.

The companion algorithmic-trading literature frames the same split as
**fundamental volatility** versus **microstructure noise** — "extraneous
fluctuations due to the way the market operates" — and treats raw/unconditional
volatility as the (undifferentiated) sum.

**Source:** Harris (2003) ch.20 §20.1–20.3 pp.410–415; Cartea, Jaimungal & Penalva
(2015) §4.3 p.93.

## Mathematical Reasoning

Decompose total volatility additively into its two components:

```
Var(ΔP) = Var(fundamental innovations) + Var(transitory component)
        = "non-reverting" part          + "reverting" part
```

The two parts are distinguished by their **serial correlation**, not by their
size:

- Fundamental innovations follow a random walk, so successive fundamental price
  changes are (to first order) **serially uncorrelated** — knowing the last change
  tells you nothing about the next.
- Transitory displacements reverse, so they inject **negative serial correlation**:
  an up-tick caused by a buyer paying the ask is disproportionately followed by a
  down-tick when a seller hits the bid (and order-imbalance pushes by uninformed
  traders mean-revert over longer horizons). Reversals are therefore more common
  than continuations.

This gives the identifying comparative statics. Holding fundamental news fixed,
anything that *widens* the transaction-cost (transitory) spread component or that
forces larger uninformed order imbalances *increases* transitory volatility and
makes price-change serial correlation *more negative*. Conversely, deepening
liquidity shrinks the price footprint of a given uninformed order, so transitory
volatility falls toward zero in a perfectly liquid market — which is exactly the
sense in which "transitory volatility and the transaction costs of uninformed
traders are very closely correlated." Roll's serial-covariance spread estimator
operationalizes this: under a random-walk value plus an independent buy/sell
indicator times half-spread, the *expected serial covariance* of price changes is
strictly negative and is a function only of the spread, so the transitory
component (and hence an implied effective spread) can be backed out from the
sample serial covariance — Harris states this result and sketches its assumptions
rather than re-deriving the estimator at length.

**Source:** Harris (2003) ch.20 §20.2–20.3 pp.413–415 (incl. Roll serial-covariance
sidebar).

## Boundary Notes

- **The split is about reversion, not magnitude.** A large move can be pure
  fundamental volatility (a frost shock) and a small persistent drift can be
  *non*-volatility (a zero-coupon bond's predictable pull-to-par compensates for
  carry and does **not** count as fundamental volatility because it is fully
  expected). Only *unexpected* changes feed fundamental volatility.
- **Negative serial correlation is suggestive, not conclusive.** Harris's
  perishable-commodity caveat: a spot-price series is a sequence of prices for
  *different-dated* items, so high negative autocorrelation in spot prices can
  reflect genuinely changing fundamentals across delivery dates rather than
  transitory bounce — futures on the same commodity isolate the transitory part
  better. Misreading fundamental reversion as transitory noise can lead regulators
  to "fix" volatility that should not be suppressed.
- **Adverse selection straddles the boundary.** Informed-trader order flow that
  dealers learn from drives the *adverse-selection* spread component, and the
  resulting permanent price moves are counted as **fundamental** volatility — so
  not all spread-related volatility is transitory.
- **Regulatory leverage is one-sided.** Policy cannot durably reduce fundamental
  volatility (suppressing it only degrades price informativeness); it can move
  transitory volatility either way. Hence high transitory volatility is the signal
  that markets are illiquid — the legitimate target of intervention.

**Source:** Harris (2003) ch.20 §20.1.2, §20.2 pp.411–415.

## See Also

- [`mt-permanent-vs-transitory-price-components`](./mt-permanent-vs-transitory-price-components.md) -- the econometric decomposition underlying the fundamental-vs-transitory split.
- [`mt-temporary-permanent-price-impact`](./mt-temporary-permanent-price-impact.md) -- how a single order's price footprint divides into a reverting (transitory) and a permanent (fundamental) piece.
- [`mt-bubbles-crashes-circuit-breakers`](./mt-bubbles-crashes-circuit-breakers.md) -- the regulatory response when volatility appears excessive (Harris ch.28).

## Escalate to Raw When

You need the formal Roll serial-covariance spread-estimator derivation (Harris's
boxed sidebar, p.415: random-walk value + independent buy/sell indicator,
expected serial covariance = −S²/4, inversion to an effective-spread estimate) or
the precise variance algebra of the decomposition — Harris states the result and
the estimator inversion but compresses the steps. Re-read Trading and Exchanges
ch.20 §20.3 pp.414–416. For multiple competing raw-vs-microstructure volatility
*estimators* (realized volatility, return range, quote-change counts) and how
microstructure noise contaminates high-frequency realized variance, re-read Cartea,
Jaimungal & Penalva (2015) §4.3 "Volatility" pp.93–95.
