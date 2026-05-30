---
schema_version: "cacg.v0"
id: "mt-garman-order-flow-temporal-microstructure"
title: "Garman Order-Flow Foundation: Stochastic Arrival and Temporal Microstructure"
reading_id: "14_microstructure_and_trading"
summary: "Garman (1976) recast the market as a stochastic flow of buy/sell orders whose temporal imbalances expose a monopolist dealer to ruin, inaugurating explicit microstructure modeling of how prices and spreads arise."
tags: ["microstructure", "order-flow", "inventory-models", "market-maker", "poisson-arrival"]
citations:
  - source_id: "mt_ohara_1995_market_microstructure_theory_en"
    chunk_id: "mt_ohara_1995_market_microstructure_theory_en:p023:0027"
    chunk_hash: "35419133e310ac14676e7f3c83b001466d2e22e5ada39898d90cdbfa228eff00"
    page_range: [23, 24]
    quote: "Garman argued that an exchange market could be characterized by a flow of orders to buy and sell."
    edge_type: "defines"
card_hash: "296e07f1ada69944010dc37300c4b47340c0ba102dee562a4f1a1790f2bd3df5"
---
# Garman Order-Flow Foundation: Stochastic Arrival and Temporal Microstructure

## Intuition
Classical price theory tells us the equilibrium price is the one at which quantity
demanded equals quantity supplied. But that picture quietly assumes buyers and
sellers show up at the same instant. Garman's (1976) starting observation is that
they do not: orders to buy and to sell trickle in at random, asynchronous moments.
If the buy stream and the sell stream are not perfectly synchronized, then at any
given moment there is generally an imbalance, and someone — a price-setting agent —
must absorb it. The study of *how* that imbalance is absorbed, instant by instant,
is what O'Hara calls the "temporal microstructure," and it is what Garman put at the
center of analysis.

The conceptual move is to stop treating supply and demand as static schedules and
instead treat them as *stochastic processes* — a flow of orders. This reframing makes
the dealer's situation look like a classic inventory or insurance "ruin" problem: a
monopolist market maker stands ready to buy at a bid and sell at an ask, and random
order arrivals push his stock and cash holdings up and down like a random walk. The
dealer's central worry is not pricing per se but *staying alive* — never running out
of either stock or cash.

```
   sell orders (rate lam_b)         buy orders (rate lam_a)
        |  |   |     | |                 |    | |  |   |
        v  v   v     v v                 v    v v  v   v
   ----------------------------[ DEALER ]----------------------------
        pays bid p_b, takes stock         charges ask p_a, gives stock
                 |                                  |
        stock inventory I_s(t)  <----  cash inventory I_c(t)
        (random walk; hits 0 => FAILURE)  (random walk; hits 0 => FAILURE)
```

Garman's framing inaugurated the explicit theoretical study of market microstructure:
prices and the spread are no longer assumed away but *derived* from the interaction
between the order-arrival processes and the dealer's survival constraint.
**Source:** O'Hara (1995) ch.2 §2.1 (Order Arrival and Market Making) pp.13-15.

## Definition
A single, **monopolistic market maker** sets, once and for all at time 0, an ask price
`p_a` (the price at which he fills orders to buy stock from him) and a bid price `p_b`
(the price at which he fills orders to sell stock to him). Orders are for one unit each.
The dealer maximizes expected profit per unit time *subject to avoiding failure*, where
failure means running out of either cash or inventory.

Order arrivals are modeled as two **independent stationary Poisson processes** with
price-dependent arrival-rate functions `lam_a(p)` for buy orders (stock flowing out of
the dealer) and `lam_b(p)` for sell orders (stock flowing into the dealer). A Poisson
process means inter-arrival times are exponentially distributed; over a short interval
of length `dt`, a buy order arrives with probability approximately `lam_a * dt`.

Let `I_c(t)` and `I_s(t)` denote the dealer's cash and stock holdings at time `t`, with
initial endowments `I_c(0)` and `I_s(0)`. Let `N_a(t)` be the cumulative shares *sold to*
traders (executed buy orders) and `N_b(t)` the cumulative shares *bought from* traders
(executed sell orders). The dealer may not borrow stock or cash, so his position is
fully determined by the realized order arrivals.
**Source:** O'Hara (1995) ch.2 §2.1 pp.15-17.

## Mathematical Reasoning
**Inventory dynamics.** Holdings evolve deterministically given the order counts:

```
   I_c(t) = I_c(0) + p_a * N_a(t)  -  p_b * N_b(t)      (cash)
   I_s(t) = I_s(0) +       N_b(t)  -        N_a(t)      (stock)
```

Each executed buy adds `p_a` of cash and removes one share; each executed sell removes
`p_b` of cash and adds one share. Because `N_a` and `N_b` are driven by *separate*
Poisson processes, both inventories behave like random walks.

**Birth-death decomposition.** Direct computation of the time-to-ruin is intractable,
so Garman tracks the *distribution* of holdings. Let `Q_k(t) = Pr[I_c(t) = k]` and
`R_k(t) = Pr[I_s(t) = k]`. Because Poisson jumps larger than one unit have probability
of order `o(dt)`, the dealer can hold exactly `k` units of stock at `t` only via three
mutually exclusive routes from `t - dt`: held `k-1` and a sell order arrived; held `k+1`
and a buy order arrived; or held `k` and nothing happened. This yields a standard
birth-death difference equation whose limiting (steady-state) behavior gives the failure
probabilities.

**Ruin probabilities.** Treating cash inflow rate as `~ lam_a(p_a) * p_a` and outflow as
`~ lam_b(p_b) * p_b`, the limiting probability of *cash* ruin has the gambler's-ruin form

```
   lim_{t->inf} Q_0(t)  ~  ( lam_b(p_b) p_b / lam_a(p_a) p_a )^{ I_c(0) / p_bar }   if  lam_a(p_a) p_a > lam_b(p_b) p_b
                        =  1                                                        otherwise,
```

with `p_bar` an average price between bid and ask used to normalize the cash-flow units.
Symmetrically, the *stock* ruin probability is

```
   lim_{t->inf} R_0(t)  ~  ( lam_a(p_a) / lam_b(p_b) )^{ I_s(0) }   if  lam_a(p_a) < lam_b(p_b)
                        =  1                                        otherwise.
```

**Why a spread must exist.** To avoid *certain* failure on both fronts simultaneously,
the dealer must choose prices satisfying both

```
   p_a * lam_a(p_a) > p_b * lam_b(p_b)        (cash drifts upward)
   lam_b(p_b)       > lam_a(p_a)              (stock drifts upward)
```

The second condition forces `p_b < p_a`: the dealer must buy low and sell high. Hence a
**bid-ask spread is an inherent structural property** of this exchange, not an add-on —
it is what keeps positive drift in *both* inventories. Yet even with a spread, both ruin
probabilities remain strictly positive, so survival is never guaranteed. If instead the
dealer collapses bid and ask to a single market-clearing price `p*` (zero spread, or a
pure zero-drift policy), at least one inventory has zero net drift, the random walk
eventually hits an absorbing barrier, and the dealer fails with probability one. No
worked numbers are needed: the comparative-statics inequalities above carry the result.
**Source:** O'Hara (1995) ch.2 §2.1.1-2.1.2 pp.16-22.

## Boundary Notes
The model's power comes bundled with strong assumptions. (1) Order flow is *uninformed*:
Poisson arrivals require many small, independent traders, none dominant, so order flow is
stochastic but carries no information about future value — there is no adverse selection
here. (2) The dealer **cannot revise prices midstream** and cannot borrow stock or cash,
so prices are set once at time 0 and inventory plays no decision-theoretic role even though
it determines survival — a noted paradox. (3) Arrival rates are the only endogenous-facing
parameters; all else is exogenous.

These restrictions place Garman squarely in the *inventory paradigm* and distinguish it from
its successors. Amihud and Mendelson (1980) keep Garman's Poisson scaffolding but let the
dealer continuously reset bid/ask as a function of an inventory state variable (a semi-Markov
process) bounded above and below, removing the ruin problem and yielding the result that
optimal bid and ask are *monotone decreasing* in inventory. Information-based models
(Glosten-Milgrom, Kyle, treated in later chapters) abandon the uninformed-flow assumption
entirely, deriving the spread from adverse selection rather than inventory survival. Garman's
contribution is thus foundational and "simplistic but provocative": its influence lies in
inaugurating the field, not in its realism.
**Source:** O'Hara (1995) ch.2 §2.1.2 pp.22-24.

## See Also
- [`mt-microstructure-scope-price-formation`](./mt-microstructure-scope-price-formation.md) -- sets the scope (price formation as the behavior of a specific price-setting agent) that Garman's order-flow model first operationalizes.
- [`mt-dealer-inventory-problem-spread`](./mt-dealer-inventory-problem-spread.md) -- the dealer-optimization branch (Stoll, Ho-Stoll) that extends Garman's inventory-and-spread mechanism.

## Escalate to Raw When
You need the full algebra rather than this card's reconstruction: the OCR garbles every
equation (the birth-death difference equations, the ruin-probability solutions, and the
`p_bar` units-normalization footnote). Re-read O'Hara (1995) ch.2 pp.16-20 for the exact
derivation of equations (2.1)-(2.9), pp.20-22 for the spread conditions (2.10)-(2.11) and
Figure 2.1, and pp.22-24 for the Amihud-Mendelson reformulation, before quoting any formula
beyond what the prose states verbatim.
