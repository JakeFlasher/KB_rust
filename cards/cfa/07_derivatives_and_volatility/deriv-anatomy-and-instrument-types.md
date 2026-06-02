---
schema_version: "cacg.v0"
id: "deriv-anatomy-and-instrument-types"
title: "Derivative Anatomy and Instrument Types"
reading_id: "07_derivatives_and_volatility"
summary: "A derivative is a financial instrument whose value derives from an underlying asset. The four canonical types — forward, future, option, swap — divide on payoff shape (linear vs convex) and contractual venue (OTC vs exchange-traded), and supply the entry-point taxonomy refined by later 07 cards."
tags: ["derivatives", "anatomy-instrument"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2786:4151"
    chunk_hash: "e6df3509482638ceb9c6bb05995dbc43ff9e872ed1eefdc4ade600c739bb5132"
    page_range: [2786, 2787]
    quote: "A derivative is a financial instrument that derives its performance from the performance of an underlying asset."
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p022:0024"
    chunk_hash: "6cf6fa27636807a6c8abaf23c26082bde0b79cb00bfbcbcb440a26a5a414f9ac"
    page_range: [22, 23]
    quote: "A derivative involves two parties agreeing to a future tranasaction. Its value depends on (or derives from) the values of other underlying variables."
    edge_type: "supports"
card_hash: "082501b91715e9c8df5737bfc5f830b04eb343fa45a6cf382c5ddcef4373f2d0"
---
# Derivative Anatomy and Instrument Types

## Intuition

A derivative is a contract whose value depends on the price of
some underlying asset. Four canonical instrument types
parameterize the L1 surface: forward, future, option, swap.
Each has a distinctive payoff shape — linear (forward / future
/ swap-leg) or hockey-stick (option) — and a distinctive
contractual venue (over-the-counter vs exchange-traded). The
entry-point picture below shows the option payoff to introduce
the convex/non-convex axis along which the four instruments
divide. **Source:** Hull §1 pp.1-15.

```
<!-- primitive: option-payoff-diagram source: _diagram_primitives.md -->
payoff                                payoff
   ^   long call                         ^   long put
   |             /                       | \
   |            /                        |  \
   |           /                         |   \
   |          /                          |    \
   |---------+----------> S              |-----+--------> S
             K                                 K
   max(S - K, 0)                         max(K - S, 0)
```

## Definition

A **forward** is a private, customizable contract to buy or
sell an underlying asset at a specified price `K` on a future
date `T`; settlement is bilateral and there are no interim
cashflows. A **future** is the exchange-traded analogue with
daily mark-to-market through a margin account, standardized
contract size, and a clearing-house counterparty. **Source:**
CFA L1 Curriculum (2022) Vol.5/pp.380-395.

An **option** is a contract that grants the holder the right
(but not the obligation) to buy (call) or sell (put) the
underlying at strike `K` on or before expiry `T`. The holder
pays the writer an upfront premium for this asymmetric right;
the writer receives the premium and assumes a potentially
unbounded obligation (uncovered short call) or a bounded one
(short put bounded by `K`). **Source:** CFA L1 Curriculum
(2022) Vol.5/pp.395-410; Hull §10 pp.215-240.

A **swap** is a contract to exchange a stream of cashflows.
The vanilla interest-rate swap exchanges fixed-rate payments
for floating-rate payments on a notional principal; equity
swaps exchange total-return on an equity index for a financing
rate; FX swaps exchange currency-denominated cashflows. The
contract is OTC, multi-period, and unwound by netting at each
reset date. **Source:** CFA L1 Curriculum (2022) Vol.5/pp.395-
410; Hull §7 pp.156-180.

## Mathematical Reasoning

Forward / futures payoff at maturity is linear in the
underlying terminal price `S_T`: long-forward gets `S_T - K`
per contract, short-forward gets `K - S_T`. The two payoffs
sum to zero (a forward is a zero-sum bilateral contract).
Cashflow timing is a single payment at `T` for the forward;
for the future the daily mark-to-market discounts the same
terminal payoff into a sequence of small daily cashflows.
**Source:** Hull §1 pp.6-15;
Hull §2 pp.43-58.

Option payoff at expiry is convex in `S_T`: long-call gets
`max(S_T - K, 0)`; long-put gets `max(K - S_T, 0)`. The
convexity is the source of the option's non-linear sensitivity
to vol; under no arbitrage the option's pre-expiry price `V`
exceeds the intrinsic-value floor and reflects the time-value
premium that decays toward zero as `t → T`. **Source:** Hull
§10 pp.230-240; CFA L1 Curriculum (2022) Vol.5/pp.395-410.

Swap payoff at each reset date is the difference between the
two legs scaled by the notional and the day-count fraction; net
present value at inception is zero (the par swap rate is set so
the fixed leg PV equals the floating leg PV). The swap unwinds
into a strip of forward contracts on the floating-rate
benchmark, each delivering one period's net cashflow.
**Source:** Hull §7 pp.156-180.

## See Also

- [`deriv-forward-and-futures-payoff.md`](deriv-forward-and-futures-payoff.md) — linear payoff and futures-vs-forward mark-to-market difference
- [`deriv-option-payoff-anatomy.md`](deriv-option-payoff-anatomy.md) — call / put payoff, intrinsic vs time value, American vs European exercise
- [`deriv-swap-cashflow-mechanics.md`](deriv-swap-cashflow-mechanics.md) — vanilla swap as a strip of forwards on the floating leg

## Escalate to Raw When

Open CFA L1 Curriculum Vol.5 Reading 45 or Hull's introduction
chapter directly when any of the criteria below applies.
**Source:** CFA L1 Curriculum (2022) Vol.5/pp.380-410.

- The contract is non-vanilla (basket option, range accrual,
  variance swap, total-return swap on a non-equity benchmark);
  the payoff anatomy departs from the four canonical types in
  this card. **Source:** Hull §26 pp.602-626.
- Settlement / clearing mechanics matter (CCP margining,
  initial vs variation margin, ISDA / CSA boundary into 06's
  CDS / CVA / CSA cards). **Source:** Hull §2 pp.43-58.
- The card needs the L2 / L3 derivative valuation machinery
  (BSM, binomial, Monte Carlo); those live in subsequent
  batches of subcorpus 07. **Source:** Hull §13 pp.302-340.
