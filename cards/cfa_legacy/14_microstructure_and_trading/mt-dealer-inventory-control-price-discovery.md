---
schema_version: "cacg.v0"
id: "mt-dealer-inventory-control-price-discovery"
title: "Dealer Inventory Control and the Price-Discovery Process"
reading_id: "14_microstructure_and_trading"
summary: "Dealers steer inventory toward a target (zero absent speculation) by skewing quotes; the hunt for prices that yield a balanced two-sided order flow is the price-discovery process."
tags: ["microstructure", "dealer", "inventory-control", "price-discovery", "quote-skewing", "liquidity"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p296:0489"
    chunk_hash: "f724d232edadcb24527836d317fac120f9c731b4706a23ffc1597b678907171c"
    page_range: [297, 297]
    quote: "Dealers also may increase their bid sizes, and lower their ask sizes, to decrease their inventories."
    edge_type: "defines"
card_hash: "8b4056a3513a61f0b459604f99cdfb43d50ca9b48e944d902768c46a1f566148"
---
# Dealer Inventory Control and the Price-Discovery Process

## Intuition
A dealer is not a directional bettor; absent any view, the dealer wants to end each round trip flat, holding no position to finance and none exposed to adverse price moves. The dealer's only lever over which side fills is the quote. Because customers choose the side they trade, the dealer cannot order the flow — but the dealer *can* tilt the odds. To bleed off an oversized long, the dealer drops *both* quotes: the lower ask lures buyers (selling reduces the position) while the lower bid repels sellers (so the position does not grow further). To rebuild a depleted or short position, the dealer lifts both quotes. The whole quote is dragged, not just one side; the skew, not the level alone, does the inventory work.

```
   INVENTORY TOO HIGH (long)            INVENTORY TOO LOW (short/below target)
   want to SELL it down                 want to BUY it back
        |                                    |
   lower BOTH quotes                    raise BOTH quotes
        |                                    |
   bid v  -> sellers stay away          bid ^  -> sellers rush in
   ask v  -> buyers come in             ask ^  -> buyers stay away
        |                                    |
   net flow tilts toward dealer-SELL    net flow tilts toward dealer-BUY
                       \                /
                        v              v
              price that yields BALANCED two-sided flow
                     = the discovered market value
```

When the book is already at target, the dealer wants buy and sell volume to arrive in equal measure so inventory stays put. The search for the quote that produces that balanced, two-sided flow is *price discovery*: at those prices supply meets demand, and that balance point is the market value the dealer is trying to find.
**Source:** Harris (2003) ch.13 §13.6 pp.283-285.

## Definition
- **Inventory:** the dealer's position in an instrument; rises on net buying, falls on net selling, since customers pick the side.
- **Target inventory:** the position the dealer *wants* to hold. For a pure dealer (no speculation, hedging, or investing) with symmetric long/short costs, the target is zero. Speculative, hedging, or investing motives shift the target away from zero (e.g., long when the dealer thinks the instrument is undervalued).
- **Inventory imbalance:** actual position minus target position.
- **Two-sided order flow:** a mix of buyers and sellers wanting to trade equal quantities.
- **Price-discovery process:** the search for the prices that produce a two-sided order flow — i.e., the prices at which buying and selling quantities are just in balance, supply equals demand, and market value is revealed.
**Source:** Harris (2003) ch.13 §13.6 pp.283-285.

## Mathematical Reasoning
Let the dealer's inventory be `I` and target `I*`, with imbalance `x = I - I*`. Customer order flow at quotes `(bid, ask)` decomposes into a sell rate `S(bid)` (volume hitting the dealer's bid) and a buy rate `B(ask)` (volume lifting the dealer's ask). Standard monotonicity: `S` increases in `bid` (a higher bid attracts sellers) and `B` decreases in `ask` (a higher ask repels buyers). The dealer's net inventory drift is `dI/dt ∝ S(bid) - B(ask)`.

Comparative statics of quote-skewing follow directly:
- To shed a long (`x > 0`, want `dI/dt < 0`): lower `ask` raises `B`, lower `bid` lowers `S`, so `S - B` falls. Both quotes move *down* together.
- To accumulate (`x < 0`, want `dI/dt > 0`): raise `bid` raises `S`, raise `ask` lowers `B`, so `S - B` rises. Both quotes move *up* together.

Sizes reinforce the same direction: increasing bid size / shrinking ask size pushes flow toward dealer-buying, and vice versa.

Balance — and hence price discovery — is the quote pair `(bid*, ask*)` at which expected flow is two-sided given the target: `S(bid*) = B(ask*)` when `x = 0`. That equality is the supply-equals-demand condition; the implied mid is the discovered market value. If the dealer cannot afford to wait for flow, an alternative is to *demand* liquidity from another dealer — buying at the other's ask and selling at the other's bid — which closes the imbalance instantly but realizes a **negative** spread, the cost of urgency. No worked figures are needed: the signs of the partials fix the directions.
**Source:** Harris (2003) ch.13 §13.6, §13.6.1 pp.283-285.

## Boundary Notes
- **Zero target assumes symmetry.** The zero-target result holds only when long and short positions are equally costly to create and hold. In markets where shorting or holding a short is dearer, dealers carry a positive target inventory to dodge those costs.
- **Speculation/hedging breaks the flat default.** Dealers who also speculate, hedge, or invest set nonzero targets; their quote-skewing then chases that nonzero target, not a flat book.
- **Quote-skewing is influence, not command.** Because customers choose the side, the dealer only tilts probabilities of flow; quotes coax order flow rather than guarantee it.
- **Inventory risk is the binding constraint.** Large positions are costly to finance and exposed to adverse moves; if imbalance grows too large the clearer forces liquidation, so control is a survival constraint, not just optimization.
- **Price discovery here is order-flow balancing, distinct from learning from informed trades.** This card is the inventory-balancing channel; the adverse-selection / information channel of price formation is a separate mechanism.
**Source:** Harris (2003) ch.13 §13.6, §13.6.1 pp.283-285.

## See Also
- [`mt-dealer-inventory-problem-spread`](./mt-dealer-inventory-problem-spread.md) -- how holding inventory and inventory risk feed the bid-ask spread the dealer must charge.
- [`mt-grossman-miller-inventory-liquidity-premium`](./mt-grossman-miller-inventory-liquidity-premium.md) -- formal model where finite market-maker risk capacity prices the liquidity premium for absorbing imbalance.
- [`mt-informed-traders-price-efficiency`](./mt-informed-traders-price-efficiency.md) -- the complementary information channel of price formation that this inventory channel sits alongside.

## Escalate to Raw When
Harris ch.13 lays out the full Table 13-2 tactic grid (bid/ask price and size moves for each inventory condition) plus the urgency tactic of demanding liquidity at a negative realized spread; this card sketches the direction of skew but not the complete tactic taxonomy. Re-read pp.283-285 for the exact condition-tactic-purpose mapping and chapter 14 for how these inventory pressures combine with adverse selection to set the spread itself.
