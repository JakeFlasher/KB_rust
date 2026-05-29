---
schema_version: "cacg.v0"
id: "deriv-option-payoff-anatomy"
title: "Option Payoff Anatomy"
reading_id: "07_derivatives_and_volatility"
summary: "European calls pay max(ST - K, 0) and puts max(K - ST, 0) at expiry; pre-expiry option price decomposes into intrinsic value plus time value. American options add the right to exercise before T, generating an early-exercise premium for dividend-paying calls and any put when in-the-money."
tags: ["derivatives", "option-payoff"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p227:0347"
    chunk_hash: "10c2925163595acdc2c3274b2ee50021c85ee18e90fde3418f64b58f9ffbbf6f"
    page_range: [227, 227]
    quote: "American options can be exercised at any time up to the expiration date, whereas European options can be exercised only on the expiration date itself"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p230:0351"
    chunk_hash: "a3bb3b251ad9391851cacc94100b16d7e0b1dc3e01dfbe6654c45b4c75501502"
    page_range: [230, 231]
    quote: "payoff from a long position in a European call option is max1ST - K, 02 This reflects the fact that the option will be exercised if ST 7 K and will not be exercised if ST … K"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p234:0356"
    chunk_hash: "d0885b6162d01270a9ccb76b562f422e905c1be59360f28b10bf41ae574f2633"
    page_range: [234, 234]
    quote: "For a call option, the intrinsic value is therefore max1S - K, 02. For a put option, it is max1K - S, 02"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2886:4324"
    chunk_hash: "6bd369b86c9e90b15f75a37649d5c5005a377ff55dc81585944bc8c4012b94a4"
    page_range: [2886, 2886]
    quote: "The value of a European call at expiration is the exercise value, which is the greater of zero or the value of the underlying minus the exercise price"
    edge_type: "supports"
card_hash: "767bc0a04db2692fc18ba0d86bf3055954af28b111fac51fabf9bdda5b12e03d"
---
# Option Payoff Anatomy

## Intuition

A European call grants the holder the right (but not the
obligation) to buy the underlying at strike `K` at expiry `T`;
a European put grants the symmetric right to sell. The payoff
diagram is the canonical hockey-stick: zero below the kink for
the call, linear above; mirrored for the put. Pre-expiry, the
option's market price exceeds intrinsic value by a positive
time premium that decays as `t → T`. **Source:** Hull §10
pp.215-230.

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

A **European call** with strike `K` and expiry `T` pays the
holder `max(S_T - K, 0)` at `T`. A **European put** pays
`max(K - S_T, 0)`. The kink at `S_T = K` is the hallmark of
optionality: payoff is one-sided and bounded below by zero.
The holder pays an upfront premium `c` (call) or `p` (put) to
the writer at inception; the writer's payoff is the negative
of the holder's. **Source:** Hull §10 pp.215-230.

An **American option** is the same contract except the holder
may exercise on any date `t ≤ T`. For a non-dividend-paying
underlying the American call's optimal exercise policy is
"never exercise early" (Hull §11), so American and European
calls coincide; American puts may be optimally exercised early
when the underlying is deep in-the-money (the early-exercise
premium becomes positive). **Source:** Hull §11 pp.260-274.

The pre-expiry option price decomposes into **intrinsic value**
(the immediate-exercise payoff `max(S_t - K, 0)` for a call,
`max(K - S_t, 0)` for a put) plus **time value** (the residual
positive premium reflecting the remaining option to wait). At
expiry time value collapses to zero and the option price equals
intrinsic value. **Source:** Hull §10 pp.230-240; CFA L1
Curriculum (2022) Vol.5/pp.395-410.

## Mathematical Reasoning

The call payoff `max(S_T - K, 0)` is convex in `S_T`. Convexity
means a mean-preserving spread of the terminal-price
distribution increases the expected payoff: the option holder
benefits from variance per Jensen's inequality. This is the
intuition behind vega (option-price sensitivity to implied
vol): a higher vol broadens the terminal distribution and
raises the expected payoff. **Source:** Hull §10 pp.230-240.

For a non-dividend-paying underlying with positive risk-free
rate `r`, the early-exercise decision for an American call
trades off (a) the payoff `S_t - K` collected immediately
against (b) the strike-deferral value `K - K · exp(-r · (T -
t))` plus the remaining time-value of the option. Both terms
favor "wait", so the optimal stopping policy is to hold to
expiry; the American call price equals the European call price.
For a put with `r > 0` the strike-deferral term works in the
opposite direction (early exercise locks in the strike sooner),
so an American put can have a positive early-exercise premium
when deep in-the-money. **Source:** Hull §11 pp.260-274.

The time-value component is highest when the remaining exercise
choice is most valuable and collapses to zero at expiry. In the
BSM / Greeks layer, the same intuition is quantified through
theta, gamma, and vega; that machinery belongs to later 07
cards rather than to the payoff-definition layer here.
**Source:** Hull §10 pp.230-240; Hull §19 pp.430-455.

## See Also

- [`deriv-anatomy-and-instrument-types.md`](deriv-anatomy-and-instrument-types.md) — taxonomy placing the option against forward / future / swap
- [`deriv-put-call-parity.md`](deriv-put-call-parity.md) — algebraic relationship between European call, European put, forward, and bond
- [`deriv-no-arbitrage-bounds.md`](deriv-no-arbitrage-bounds.md) — lower / upper bounds the option price must satisfy

## Escalate to Raw When

Open Hull chapters 10-11 or CFA L1 Curriculum Vol.5 Reading 45
directly when any of the criteria below applies. **Source:**
Hull §10 pp.215-260; CFA L1 Curriculum (2022) Vol.5/pp.395-410.

- The option is exotic (Asian, barrier, lookback, digital);
  payoff anatomy departs from the European-American hockey-
  stick. **Source:** Hull §26 pp.602-626.
- Dividends, early-exercise, or American-style features
  require the binomial / PDE / MC machinery developed in
  later 07 batches. **Source:** Hull §13 pp.302-340.
- The card needs option Greeks (delta, gamma, vega, theta,
  rho); those follow a separate decomposition layer.
  **Source:** Hull §19 pp.430-455.
