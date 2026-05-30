---
schema_version: "cacg.v0"
id: "mt-trade-direction-signing"
title: "Signing Trades and the Trade-Direction Indicator (Lee-Ready)"
reading_id: "14_microstructure_and_trading"
summary: "Empirical microstructure classifies each trade as buyer- (q=+1) or seller-initiated (q=-1) from quote/tick rules; mis-signing biases spread and price-impact estimates, and real signs are positively autocorrelated from order splitting."
tags: ["microstructure", "trade-signing", "lee-ready", "order-flow", "price-impact"]
citations:
  - source_id: "mt_hasbrouck_2007_empirical_market_microstructure"
    chunk_id: "mt_hasbrouck_2007_empirical_market_microstructure:p103:0131"
    chunk_hash: "551facb233cdedd1f973c0fc6352987611e1187377811fd018b1d176639595cf"
    page_range: [103, 104]
    quote: "flow that has been signed as to buyer- or seller-initiated. The sign is usually"
    edge_type: "defines"
---
# Signing Trades and the Trade-Direction Indicator (Lee-Ready)

## Intuition
Most public trade-and-quote (TAQ) tapes report *what* traded — a price and a size —
but not *who* demanded liquidity. Yet nearly every empirical microstructure
quantity (effective spread, price impact, order-flow autocorrelation, PIN) is
defined relative to the *initiator* of the trade: the impatient side that crossed
the spread to demand immediacy. So before any estimation can begin, the analyst
must attach a sign `q_t = +1` (buyer-initiated) or `q_t = -1` (seller-initiated)
to each transaction. This is *trade signing*.

The sign is not observed; it is *inferred* by comparing the trade price to the
contemporaneous quotes. The intuition is simple: a buyer demanding immediacy lifts
the offer, so a print at or near the ask is tagged a buy; a seller hits the bid,
so a print at or near the bid is tagged a sell. When the print lands exactly at
the midpoint the quote gives no information and a fallback is used.

```
        bid          midpoint (BAM)          ask
         |--------------|--------------|       quote rule
         v              v              v
   sell q=-1     (ambiguous)      buy q=+1
                       |
                       +--> tie-break with TICK RULE:
                            price > last  -> q=+1 (uptick)
                            price < last  -> q=-1 (downtick)
```

This quote-rule-plus-tick-rule fallback is the Lee-Ready algorithm. It is the
workhorse that turns raw prints into the signed order-flow series `x_t = q_t V_t`
on which spread and impact regressions run.

**Source:** Hasbrouck (2007) §9.9 / §14.3.3 pp.103, 162

## Definition
In the Roll model and its descendants the transaction price decomposes as
`p_t = m_t + q_t c`, where `m_t` is the unobserved efficient price, `c` is the
half-spread, and `q_t` is "a trade direction indicator set to +1 if the customer
is buying and -1 if the customer is selling."

The **quote rule** signs a trade by comparing its price `p_t` to the bid-ask
midpoint BAM: in trade-based cost work "it is assumed that an execution price
above the BAM originates from a market buy order, and below, from a sell order,"
which mathematically makes the effective half-cost `|p_t - BAM|`.

The **tick rule** is the fallback for midpoint (or unquoted) prints: classify
relative to the most recent price change — an uptick is a buy, a downtick a sell.
The combined quote-rule-with-tick-rule-tie-break procedure is due to Lee and
Ready (1991), "Inferring trade direction from intraday data."

**Source:** Hasbrouck (2007) §3.2 pp.40; §14.3.3 pp.162

## Mathematical Reasoning
Why signing matters for *every downstream estimator*: in the Roll model
`Δp_t = c(q_t - q_{t-1}) + Δm_t`, so the half-spread `c` is identified from the
first-order autocovariance `γ_1 = -c²` of price changes. That identification
presumes the `q_t` are correctly classified and serially uncorrelated; both
assumptions are stressed by real data. The decomposition `p_t = m_t + q_t c`
makes the dependence explicit — flip the sign on a fraction of trades and the
estimated covariance structure (hence `c` and any `λ` price-impact slope) is
biased toward zero, because misclassification injects spurious sign reversals.

The deeper empirical fact is that real trade directions are *not* serially
independent: "In reality trades in most markets are positively autocorrelated,
for example, buys tend to follow buys." Hasbrouck models this as an MA(1),
`q_t = v_t + β v_{t-1}` with `β > 0`, or equivalently (Madhavan-Richardson-Roomans)
as an AR(1) `q_t = φ q_{t-1} + v_t` driven by a *continuation probability* α:

```
   Pr(q_{t+1}=+1 | q_t=+1) = Pr(q_{t+1}=-1 | q_t=-1) = α
   α = 1/2  ->  signs uncorrelated   (basic Roll assumption)
   1/2<α<1  ->  signs persistent  =>  φ = 2α - 1 > 0
```

The economic driver of `α > 1/2` is **order splitting**: a large parent order is
sliced into many child trades on the same side, so consecutive prints share a
sign. This persistence is exactly what biases naive spread estimators (the Roll
covariance is contaminated when `q_t` is autocorrelated) and what later
trade-and-quote VAR models exploit to separate transient (inventory/spread) from
permanent (information) price impact.

(No worked arithmetic is given here; the relations above are structural.)

**Source:** Hasbrouck (2007) §3.2 pp.40; §4 (Exercise 4.1) pp.48; §9.2 pp.90

## Boundary Notes
- **When the quote rule is reliable:** automated markets with accurate,
  low-latency time stamps. Lee-Ready originally lagged the quote by five seconds
  on NYSE data; in cleaner modern feeds signing on reported times "with no lag is
  likely to be more accurate."
- **When signing breaks:** stale or laggy trade reports can push a buyer-initiated
  print below a midpoint that moved after the trade, "lead[ing] to error in the
  buy-sell inference." Cross-venue consolidation creates non-uniform delays that
  mis-order trades and quotes. Hidden/iceberg liquidity also breaks it — a market
  buy can execute *at or below* the BAM against an aggressive hidden sell, so the
  quote rule mis-signs it.
- **Imputed ≠ true:** Odders-White (2000) compares signs imputed from quote data
  against the directions of the *actual* underlying orders, documenting the
  classification-error rate that the methods carry.
- **Contrast with order data:** when raw order records exist, the initiator is
  observed directly and no inference is needed; signing is the second-best tool
  for the common case where only trades and quotes are public.

**Source:** Hasbrouck (2007) §14.3.3 pp.162; note to ch.9 pp.191

## See Also
- [`mt-roll-implicit-spread-estimator`](./mt-roll-implicit-spread-estimator.md) -- consumes the signed `q_t` to back out the half-spread `c`
- [`mt-generalized-roll-spread-decomposition`](./mt-generalized-roll-spread-decomposition.md) -- relaxes the serial-independence-of-signs assumption flagged here
- [`mt-hasbrouck-var-trades-quotes`](./mt-hasbrouck-var-trades-quotes.md) -- VAR on price changes and signed trades, the multivariate successor
- [`mt-pin-probability-informed-trading`](./mt-pin-probability-informed-trading.md) -- uses signed buy/sell counts to estimate informed-trade probability

## Escalate to Raw When
The card sketches the Lee-Ready logic and the persistence-of-signs result but does
not derive (a) the exact MA(1)/AR(1) algebra linking α, φ, β, and the autocovariance
of `q_t` (Hasbrouck Exercise 4.1, p.48 — solve the eight-path table to show
φ = 2α − 1), or (b) the bias formula for the Roll estimator under autocorrelated
signs. For the half-normal `E[|p_t| | |x_t|]` construction used to proxy price
impact from unsigned daily volume, and for the exact contamination of γ₁ under
signed-order persistence, re-read Hasbrouck (2007) §3.2 (Roll variance/covariance,
pp.30–40), §9.2 and §9.9 (structural signed-trade model and price impact,
pp.79–93). For classification-error magnitudes, go to Odders-White (2000).
