---
schema_version: "cacg.v0"
id: "mt-limit-order-book-mechanics"
title: "Limit Order Book Mechanics: Price-Time Priority, Walking the Book, and Maker-Taker Fees"
reading_id: "14_microstructure_and_trading"
summary: "An electronic exchange matches incoming market orders against resting limit orders by price-time priority; a large market order walks the book to deeper price levels, and maker-taker fees rebate liquidity providers while charging takers, distorting net trade prices."
tags: ["microstructure", "limit-order-book", "price-time-priority", "market-orders", "maker-taker", "liquidity"]
citations:
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p026:0024"
    chunk_hash: "df5a89d09107aa1346d34329ccd7ccb652eaa5a208e7bce619edd9fbfad41e09"
    page_range: [27, 27]
    quote: "the process whereby an entering market order executes against standing LOs deeper in the LOB is called 'walking the book'."
    edge_type: "defines"
---
# Limit Order Book Mechanics: Price-Time Priority, Walking the Book, and Maker-Taker Fees

## Intuition

Picture a continuous double auction frozen in a single data structure. On the sell
(ask) side, traders have posted limit orders (LOs) at various prices; on the buy
(bid) side, others have posted LOs they are willing to buy at. The limit order book
(LOB) is just the sorted ledger of all this resting, unexecuted liquidity, maintained
by the exchange's matching engine. Nothing trades until an aggressive order crosses
the spread.

When a market order (MO) arrives, the matching engine consumes resting LOs by
**price-time priority**: best price first, and among equally priced LOs, the one that
arrived earliest. A small MO is fully absorbed at the best quote. A *large* MO
exhausts the best level, then reaches into the next-best price, then the level after
that — it "walks the book," paying progressively worse prices the deeper it eats.

```
   ASK side (sell LOs)                          a buy MO of size Q walks up:
     p_3  | [q_3]                               - takes oldest queue first at p_1,
     p_2  | [q_2]                                 then p_2, then p_3, ...
     p_1  | [.][.]  <- best ask, FIFO queue     - stops at level k once filled
   ------------------ spread -----------------   => avg fill price >= best ask p_1
     b_1  | [.][.]  <- best bid, FIFO queue
     b_2  | [.]
   BID side (buy LOs)        (asks: p_1 < p_2 < p_3 ; bids: b_1 > b_2)
```

The depth and shape of the book therefore *is* liquidity: a thick book absorbs size
with little price movement; a thin book forces large MOs deep, generating slippage.

**Source:** Cartea, Jaimungal & Penalva (2015) §1.3 pp.10-12.

## Definition

- **Limit order (LO):** an order to buy (sell) up to a stated quantity at a price no
  worse than a stated limit; it rests in the book providing liquidity until matched or
  cancelled.
- **Market order (MO):** an order to transact a stated quantity immediately at the
  best available prices, consuming (taking) liquidity.
- **Matching engine + LOB:** orders are managed by a matching engine and a limit
  order book; the engine applies a well-defined algorithm that decides when a trade
  can occur and which standing orders are selected for execution.
- **Price-time priority:** the engine prioritises MOs over LOs, then matches an
  incoming buy MO first against the sell LOs at the lowest price; among LOs at that
  best price it executes the oldest first; if size remains it proceeds to the second-
  best price, then the third, and so on. An incoming LO joins the book at its price
  and is placed *last* in the execution queue at that price.
- **Walking the book:** the process whereby an entering MO executes against standing
  LOs that are increasingly deeper (worse-priced) in the LOB.
- **Maker-taker fees:** the MO sender (liquidity taker) pays a trading fee, while the
  filled LO poster (liquidity maker) pays a much lower fee or receives a rebate; an
  inverted *taker-maker* schedule reverses these roles.

**Source:** Cartea, Jaimungal & Penalva (2015) §1.3.1, §1.3.5, §1.4 pp.10-15.

## Mathematical Reasoning

Let the ask side hold cumulative depth at ascending price levels
$p_1 < p_2 < \cdots$ with available quantities $q_1, q_2, \ldots$. A buy MO of size
$Q$ executes by consuming levels in order until filled. Let $k$ be the smallest
index with $\sum_{j=1}^{k} q_j \ge Q$. The order fills entirely at levels
$1,\ldots,k$, taking all of levels $1,\ldots,k-1$ and a residual
$Q - \sum_{j=1}^{k-1} q_j$ at level $k$. The volume-weighted execution price is

$$\bar p(Q) = \frac{1}{Q}\Big[\sum_{j=1}^{k-1} q_j\,p_j + \Big(Q-\sum_{j=1}^{k-1}q_j\Big)p_k\Big].$$

Two structural consequences follow without any arithmetic. First, $\bar p(Q)$ is
**non-decreasing in $Q$** (for buys): adding size can only pull in weakly worse
levels, so $\bar p(Q') \ge \bar p(Q)$ for $Q' > Q$. This is the formal content of
walking the book — average cost rises with order size, and the gap
$\bar p(Q) - p_1$ is the slippage relative to the touch. Second, $\bar p(Q) = p_1$
**iff** $Q \le q_1$: an MO no larger than the best-level depth never walks.

Time priority adds a tie-break that does not enter $\bar p(Q)$ but governs *whose*
LO fills: at a shared price the queue is first-in-first-out, so a later-arriving LO at
the same price is strictly worse-positioned than an earlier one. This makes
queue position a state variable for limit-order traders even when the quoted price is
identical.

Maker-taker fees shift the economically relevant price from the quoted price to the
**net** price. If a taker pays fee $f_t$ per share and a maker receives rebate $r_m$,
the buyer's effective cost is $\bar p(Q) + f_t$ while the filled seller's effective
proceeds are $p + r_m$ (rebate raising proceeds). The quoted price thus need not
equal the price either side actually realises, so observed quotes are a biased proxy
for transaction economics by the size of the fee/rebate wedge.

**Source:** Cartea, Jaimungal & Penalva (2015) §1.3.1, §1.3.5, §1.4 pp.10-15.

## Boundary Notes

- **Price-time is not universal.** Pro-rata matching (some money markets) fills LOs
  at the best price *in proportion* to posted size with no time priority; futures
  markets sometimes mix pro-rata and time priority. Some venues also grant extra
  priority to designated market makers. Harris (2003) frames the general taxonomy:
  price priority is the primary, self-enforcing rule across order-matching markets,
  while time precedence is a secondary rule that traders must actively defend.
- **Rerouting overrides naive walking.** Under US order-protection rules an MO may
  be rerouted to another venue still displaying the best price rather than walking
  the local book; the simple single-venue VWAP $\bar p(Q)$ assumes no rerouting.
- **Lit vs dark.** The mechanics above assume a lit (open) book whose state is
  observable. In dark pools the resting liquidity is hidden, so an agent cannot
  pre-compute $\bar p(Q)$ from the visible book.
- **Auctions, not continuous matching.** Opening, closing, and post-halt auctions
  use single-price batch clearing, not the continuous price-time consumption modelled
  here.

**Source:** Cartea, Jaimungal & Penalva (2015) §1.3.2, §1.4 pp.11-16; Harris (2003) §6.1-6.2.1 pp.126-130.

## See Also

- [`mt-order-precedence-price-time`](./mt-order-precedence-price-time.md) -- the
  precedence-rule taxonomy (price priority, time precedence, pro-rata) this card relies on.
- [`mt-order-types-market-limit-stop`](./mt-order-types-market-limit-stop.md) -- the
  order-type primitives (MO, LO, stop) being matched in the book.
- [`mt-microprice-midprice-spread`](./mt-microprice-midprice-spread.md) -- how the
  best bid/ask of this book aggregate into mid- and microprice liquidity measures.
- [`mt-limit-order-book-equilibrium`](./mt-limit-order-book-equilibrium.md) -- the
  equilibrium theory of why traders post the LOs that populate this book.

## Escalate to Raw When

The card sketches walking the book as a deterministic consumption of a static book
and the VWAP monotonicity result, but the source develops these only narratively
around Figures 1.2-1.3 — re-read §1.4 for the worked artificial-LOB illustration of
queue placement and the rerouting variant. For the formal treatment of how fees move
quoted vs underlying prices, the source cites Colliard & Foucault (2012) rather than
proving it; for the full precedence-rule taxonomy and the leapfrog dynamics of
defending time precedence, escalate to Harris (2003) §6.1-6.2.1 pp.126-130.
