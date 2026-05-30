---
schema_version: "cacg.v0"
id: "mt-liquidity-supplier-taxonomy"
title: "Liquidity-Supplier Taxonomy: Five Types of Liquidity Providers"
reading_id: "14_microstructure_and_trading"
summary: "Harris classifies liquidity suppliers into five niches -- market makers, block dealers, value traders, precommitted traders, and arbitrageurs -- each best at a different liquidity dimension."
tags: ["microstructure", "liquidity", "market-makers", "dealers", "arbitrage", "trader-taxonomy"]
citations:
  - source_id: "mt_harris_2003_trading_and_exchanges"
    chunk_id: "mt_harris_2003_trading_and_exchanges:p417:0706"
    chunk_hash: "5fdace2d8dfcdf1ef6e080d0f5e42444b280d663f60537e465cb5d7c295d974d"
    page_range: [417, 418]
    quote: "therefore are not suppliers of liquidity but porters of"
    edge_type: "defines"
---
# Liquidity-Supplier Taxonomy: Five Types of Liquidity Providers

## Intuition
Liquidity is not supplied by one undifferentiated crowd. Harris organizes the supply
side into five recognizable trader types, each occupying a distinct niche and each best
at delivering a *different* dimension of liquidity (immediacy, depth, or resiliency).
A trader "supplies" liquidity whenever it trades in response to an order that someone
else initiated -- so order type (limit vs. market) is *not* the reliable tell. The tell
is motive: passive profit-seekers wait to be hit, while precommitted traders post offers
only to cheapen a trade they already intend to make.

The five niches, and the dimension each one is built for:

```
                       INFORMATION HELD             OFFERS
  Market makers  ->  little (about value/clients)  immediacy, small size, narrow
  Block dealers  ->  much (about WHO trades)        depth, to large uninformed clients
  Value traders  ->  most (about fundamental VALUE) depth + resiliency; supplier of last resort
  Precommitted   ->  none special; already want to  immediacy via aggressive limit orders
  Arbitrageurs   ->  relative value across markets   cross-market depth (transport, not creation)
```

The picture is a division of labor: a small retail buyer is filled by a market maker; an
institution dumping a block goes to a block dealer; when a price overshoots fundamentals
the value trader steps in as the buyer of last resort; aggressive public limit orders
(precommitted traders) narrow the spread; and arbitrageurs ferry depth from a deep market
into a thin one. **Source:** Harris (2003) ch.19 §19.3 pp.401-405.

## Definition
Harris (§19.3.1) first splits suppliers by motive. *Passive* liquidity suppliers --
**dealers** and **value traders** -- trade primarily to profit and generally will not
trade unless impatient traders demand liquidity. *Precommitted* liquidity suppliers post
offers only to lower the cost of a trade they already intend to make; "they would demand
liquidity if they did not offer it," and may revert to market orders if their limit
orders go unfilled. The five characteristic types (§19.3.2-19.3.6):

- **Market makers** -- dealers who quote bid/ask for impatient customers; supply
  *immediacy*, narrow but only in small size; passive; capital-constrained; avoid informed flow.
- **Block dealers** -- take large client positions (facilitations); supply *depth* to
  large *uninformed* clients by knowing *who* they trade with; slow, capital-intensive.
- **Value traders** -- informed about fundamental value; the *ultimate* suppliers of
  depth; trade when price diverges from value regardless of counterparty; make markets *resilient*.
- **Precommitted traders** -- post aggressive limit orders to obtain better prices on
  intended trades; supply *immediacy*, can offer very narrow spreads, little displayed depth.
- **Arbitrageurs** -- trade price discrepancies across markets; *porters* (transporters),
  not creators, of liquidity -- they demand it where abundant and supply it where scarce.

**Source:** Harris (2003) ch.19 §19.3.1-19.3.7 pp.401-405.

## Mathematical Reasoning
The taxonomy is driven by an adverse-selection ordering, not arithmetic. Each supplier's
willingness to quote depends on whether it can recover from uninformed counterparties what
it expects to lose to informed ones; the type that can best identify or out-know the informed
trader can profitably offer the most size.

- Market makers hold little value information, so they quote narrow only for small size and
  widen quotes as size rises to protect against informed flow -- a monotone size-spread tradeoff.
- Block dealers reduce adverse selection along a *different* axis: by screening *who* they
  trade with rather than knowing value, which lets them offer larger size (depth) than market
  makers to clients they judge uninformed.
- Value traders solve adverse selection by knowing value best, so their reservation prices
  define the *outside spread* -- wider than market-maker spreads because they trade larger size
  and must fund research, yet they will trade "when nobody else will." This is the resiliency
  mechanism: prices that diverge from value attract value-trader depth, pulling prices back.
- Precommitted traders bear no dealer cost of doing business, so they can price limit orders
  *inside* dealer quotes and, in competitive public auctions, drive dealers out -- a competitive
  pressure that tightens spreads.
- Arbitrageurs equalize prices across venues: by demanding liquidity in the deep market and
  supplying it in the thin one, they raise effective depth in the thin market without creating
  new net liquidity. They thereby *compete* with dealers, who connect buyers and sellers in the
  *same* market across *time* rather than across *venues*.

**Source:** Harris (2003) ch.19 §19.3.2-19.3.6 pp.402-405.

## Boundary Notes
- The five types are *characteristic strategies*, not disjoint people: Harris explicitly treats
  each trader as using only its single characteristic strategy "as though," while in practice
  most traders mix strategies. The taxonomy is an analytic decomposition, not a population census.
- Order type does not classify a supplier: an institution using a market order to answer a
  liquidity request still *supplies* liquidity; an undisplayed or ask-the-broker order still
  supplies it. Motive and direction-of-initiation are what matter.
- "Supplier of last resort" applies to *value traders*, not market makers; market makers may
  themselves *demand* liquidity (lay off inventory onto value traders) when uncomfortable.
- Arbitrageurs are the boundary case: labeled *porters* rather than suppliers because they move
  pre-existing depth between venues rather than originating it. The card's quote anchors exactly
  this distinction.
- Holds in markets with the relevant venue/agent populations; in a single venue with no
  competing market, the arbitrageur role is empty and depth must come from the other four.

**Source:** Harris (2003) ch.19 §19.3.1, §19.3.6 pp.401-405.

## See Also
- [`mt-liquidity-depth-immediacy-width`](./mt-liquidity-depth-immediacy-width.md) -- the
  immediacy/depth/resiliency dimensions each supplier type specializes in.
- [`mt-dealer-inventory-control-price-discovery`](./mt-dealer-inventory-control-price-discovery.md)
  -- mechanics of the market-maker / dealer node in this taxonomy.
- [`mt-block-trader-upstairs-depth`](./mt-block-trader-upstairs-depth.md) -- block-dealer depth
  provision and the upstairs facilitation market.
- [`mt-value-traders-arbitrageurs`](./mt-value-traders-arbitrageurs.md) -- value traders as
  ultimate depth suppliers and arbitrageurs as porters of liquidity.

## Escalate to Raw When
Harris develops each supplier type across chapters 13-18 (dealers, block trading, value/news
traders, arbitrageurs); §19.3 is the integrating summary. Re-read §19.3.4 for the two divergence
scenarios (value changed vs. price changed) that motivate value-trader liquidity supply and the
resiliency argument, and §19.3.6 plus Table 19-3 (pp.405-409) for the full porter-vs-supplier
contrast and the side-by-side characteristic table. The card sketches the adverse-selection
ordering qualitatively; the source argues the capital, incentive, and risk-management constraints
on each type in detail.
