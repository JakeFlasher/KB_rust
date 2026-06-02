---
schema_version: "cacg.v0"
id: "deriv-no-arbitrage-bounds"
title: "No-Arbitrage Option Bounds"
reading_id: "07_derivatives_and_volatility"
summary: "Model-free upper and lower bounds on European call and put prices follow from dominance arguments: max(S0 - K·exp(-rT), 0) ≤ C ≤ S0 and max(K·exp(-rT) - S0, 0) ≤ P ≤ K·exp(-rT). These bounds sanity-check any pricing model and tighten via the put-call parity bridge."
tags: ["derivatives", "no-arbitrage"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p252:0385"
    chunk_hash: "491f92a0f376786d3a980e9352d5b9b93b0eee86ce8a1a3099bab88243c5c93b"
    page_range: [252, 252]
    quote: "A lower bound for the price of a European call option on a non-dividend-paying stock is S0 - Ke-rT"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p253:0387"
    chunk_hash: "beb5417c93b65b6c7b14ae7159a6eb1c8486bfda0c3031287e01c5851a07369c"
    page_range: [253, 253]
    quote: "For a European put option on a non-dividend-paying stock, a lower bound for the price is Ke-rT - S0"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p264:0401"
    chunk_hash: "cbf119e1e77dcb1fff8514ba7cf31f335128bfd7473530d71ea7bc1ae95f7e04"
    page_range: [264, 264]
    quote: "A European call option on a non-dividend-paying stock must be worth more than max1S0 - Ke-rT, 02"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2892:4336"
    chunk_hash: "74f2855c312f4d9b270290b05d0a3fbd65b5f25e5e6c445275861c51f14722b5"
    page_range: [2892, 2892]
    quote: "the value of the call strategy, c0, has to be worth at least the value of the leveraged transaction, S0"
    edge_type: "supports"
card_hash: "80d5ef4df162ba0e504467628aab0030ced5a2c3a5e2ee7d1b5b8ad13f11541f"
---
# No-Arbitrage Option Bounds

## Intuition

Option prices satisfy model-free upper and lower bounds derived
purely from the no-arbitrage principle: a portfolio whose
payoff dominates another at every state must cost at least as
much. These bounds give a sanity-check before any pricing
model is invoked. The European call's lower bound combines the
underlying's value with the strike's present-value cost; the
upper bound is the underlying itself (a call cannot be worth
more than what it gives the holder the right to buy). The
same logic, mirrored, gives the put's bounds. **Source:** Hull
§11 pp.250-275.

```
European call bounds (non-dividend-paying)

C(K, T)
  ^
  |     /  upper bound: C <= S_0 (cannot exceed underlying)
  |    /
  |   /
  |  /
  | /  lower bound: C >= max(S_0 - K · exp(-r · T), 0)
  |/
  +----------------------+--> S_0
                          K · exp(-r · T)
                          (PV of strike)
```

## Definition

For a European call on a non-dividend-paying underlying with
strike `K`, expiry `T`, spot `S_0`, and risk-free rate `r`, the
no-arbitrage bounds are:
`max(S_0 - K · exp(-r · T), 0) ≤ C(K, T) ≤ S_0`.
The lower bound comes from comparing the call to the synthetic
long-forward `S_0 - K · exp(-r · T)` (a long-call dominates a
long-forward since the call has limited downside while the
forward has unlimited downside). The upper bound comes from
the observation that the call's payoff `max(S_T - K, 0)` is
bounded above by `S_T`, and at inception no asset whose payoff
is bounded by `S_T` can be worth more than `S_0`. **Source:**
Hull
§11 pp.250-263; CFA L1 Curriculum (2022) Vol.5/pp.420-440.

For a European put on a non-dividend-paying underlying, the
bounds are:
`max(K · exp(-r · T) - S_0, 0) ≤ P(K, T) ≤ K · exp(-r · T)`.
The lower bound is the present-value exercise-price floor; the
upper bound is the present value of the strike (a European put
pays at most `K` at expiry). **Source:** Hull §11 pp.263-272.

## Mathematical Reasoning

The call's lower bound `max(S_0 - K · exp(-r · T), 0)` follows
from comparing two portfolios: (I) one European call plus a
zero-coupon bond paying `K` at `T`; (II) one share of
underlying. At expiry, Portfolio I pays `max(S_T, K)` (the
call exercises if `S_T > K`, else the bond pays `K`); Portfolio
II pays `S_T`. Since `max(S_T, K) ≥ S_T`, Portfolio I dominates
weakly, so `C + K · exp(-r · T) ≥ S_0`, giving
`C ≥ S_0 - K · exp(-r · T)`. The non-negativity floor `C ≥ 0`
comes from the option's limited-liability structure (the
holder cannot owe money). **Source:** Hull §11 pp.250-263.

The call's upper bound `C ≤ S_0` follows from the dominance
`max(S_T - K, 0) ≤ S_T` at expiry. If `C > S_0`, an
arbitrageur sells the call and buys the underlying, locking
in a positive cashflow `C - S_0 > 0` at inception and a
non-negative terminal position (at expiry the holder is short
the call payoff and long the share, which always pays at
least the call's maximum payoff `S_T - K` against the strike
`K`). **Source:** Hull §11 pp.250-263.

The American-call lower bound coincides with the European
bound for a non-dividend-paying underlying (early exercise is
suboptimal so the American and European call prices coincide).
When expected discrete dividends are present, early exercise
immediately before an ex-dividend date can become optimal; the
card stops at that boundary and leaves American-option valuation
to later 07 machinery. **Source:** Hull §11 pp.260-274.

The put-call parity from
[`deriv-put-call-parity.md`](deriv-put-call-parity.md#mathematical-reasoning)
links the two sets of bounds: bounds on `C` translate
mechanically to bounds on `P` via
`P = C - S_0 + K · exp(-r · T)`. In this card sequence, parity
is the bridge that keeps the call and put bounds mutually
consistent: any tightening of one side propagates to the other.
**Source:** Hull §11 pp.260-275.

## See Also

- [`deriv-option-payoff-anatomy.md`](deriv-option-payoff-anatomy.md) — the call / put terminal payoffs the bounds rest on
- [`deriv-put-call-parity.md`](deriv-put-call-parity.md) — algebraic identity linking call, put, underlying, bond

## Escalate to Raw When

Open Hull chapter 11 or CFA L1 Curriculum Vol.5 Reading 46
directly when any of the criteria below applies. **Source:**
Hull §11 pp.250-275; CFA L1 Curriculum (2022)
Vol.5/pp.420-440.

- Dividends, borrow cost, or stochastic interest rates
  perturb the bounds; the formulas generalize per Hull §11.
  **Source:** Hull §11 pp.260-275.
- The card needs tighter bounds (e.g. via convexity / Jensen
  inequality applied to the BSM closed form); those are
  model-specific and live in later 07 batches. **Source:**
  Hull §15 pp.346-380.
- The option is American-style on a dividend-paying
  underlying, making the early-exercise bound nontrivial.
  **Source:** Hull §11 pp.260-274.
