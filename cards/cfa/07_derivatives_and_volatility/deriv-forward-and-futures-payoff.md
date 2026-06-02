---
schema_version: "cacg.v0"
id: "deriv-forward-and-futures-payoff"
title: "Forward and Futures Payoff"
reading_id: "07_derivatives_and_volatility"
summary: "A long-forward payoff at maturity is linear in S_T: payoff = S_T - K per contract; short pays K - S_T. Under risk-neutral pricing the fair forward price on a non-dividend-paying asset is F_0 = S_0 · e^{rT}. A futures contract has the same terminal payoff but trades on an exchange with daily mark-to-market — the single payoff becomes a path of daily margin cashflows."
tags: ["derivatives", "forward-futures"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p064:0091"
    chunk_hash: "019209ce43460baeb268938b31647a58c8caa99877c5d5a36d684e8c664f9fb7"
    page_range: [64, 64]
    quote: "A forward contract is traded in the over-the-counter market and there is no standard contract size or standard delivery arrangements."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2795:4168"
    chunk_hash: "2898546a3c460439358e00da1edd32507d52ff1a97aa0a2f18cff328c1bc8a62"
    page_range: [2795, 2796]
    quote: "A forward contract is an over-the-counter derivative contract in which two parties agree that one party"
    edge_type: "supports"
card_hash: "c53d31ff58bfddbaa35b1ed60bad8fa8c4c305a9a7ab0f48ed3cca4fcb0505fb"
---
# Forward and Futures Payoff

## Intuition

A long-forward position is a bet that the underlying will
appreciate above the contracted price `K`; a short-forward
position is the symmetric bet on depreciation. The payoff at
maturity is linear in the terminal price `S_T`: every dollar
move of `S_T` past `K` translates one-for-one into the long's
gain. A futures contract is the exchange-traded analogue with
the same terminal payoff but daily mark-to-market through a
margin account. **Source:** Hull §1 pp.6-15.

```
<!-- primitive: forward-futures-payoff source: _diagram_primitives.md -->
payoff
   ^   long forward                short forward
   |       /                              \
   |      /                                \
   |     /                                  \
   |----+------------> S      ---------------\---> S
        K                                     K
   |   /                                      \
   |  /                                        \
   | /                                          \
   payoff = (S_T - K)             payoff = (K - S_T)
   per long contract              per short contract
```

## Definition

A **forward contract** with strike `K` and maturity `T`
specifies that the long pays `K` and receives the underlying
at `T`; the short delivers the underlying and receives `K`.
There are no interim cashflows; the contract is privately
negotiated and bilaterally settled. The fair forward price at
inception under cost-of-carry on a non-dividend-paying asset
is `F_0 = S_0 · exp(r · T)`, derived by the no-arbitrage
replication argument: a long-forward is equivalent to
borrowing `K · exp(-r · T)` at the risk-free rate and buying
the spot at `S_0`. **Source:** Hull §5 pp.110-130.

A **futures contract** has the same terminal payoff structure
but trades on an organized exchange with standardized contract
size, clearing-house counterparty, and daily mark-to-market.
Each day the contract value is reset to zero by margin-account
debits and credits; the holder's economic position aggregates
to the same `S_T - K` payoff at maturity, but cashflows are
distributed across the contract's life rather than concentrated
at `T`. **Source:** Hull §2 pp.43-58.

## Mathematical Reasoning

The long-forward payoff `S_T - K` is linear in `S_T`. Under
risk-neutral pricing the forward price `F_0` solves
`E^Q[S_T - F_0] · exp(-r · T) = 0`, so `F_0 = E^Q[S_T]`. For a
non-dividend-paying asset whose price follows a martingale
under the risk-neutral measure scaled by the bank account,
`E^Q[S_T] = S_0 · exp(r · T)`. With a continuous dividend
yield `q`, the formula generalizes to `F_0 = S_0 · exp((r - q)
· T)`. **Source:** Hull §5 pp.110-130.

The forward-vs-futures price differ in general because daily
mark-to-market introduces interest-rate-sensitive cashflows
along the way. When interest rates are deterministic the two
prices coincide; when interest rates are stochastic and
correlated with the underlying, the futures price embeds a
convexity adjustment that depends on the covariance of the
underlying and the bank-account numeraire. The L1 / Hull
treatment treats the difference as second-order and uses
`F_0` interchangeably for forward and futures price.
**Source:** Hull §5 pp.130-138.

The futures margining mechanic transforms the single terminal
payoff into a path-dependent sequence of daily cashflows.
Initial margin is the buffer; variation margin is the daily
mark-to-market. A position is liquidated if the margin balance
falls below the maintenance level; the daily-settlement design
limits counterparty credit exposure and makes the central
counterparty the ultimate guarantor (the boundary into 06's
counterparty-risk material). **Source:** Hull §2 pp.43-58.

## See Also

- [`deriv-anatomy-and-instrument-types.md`](deriv-anatomy-and-instrument-types.md) — taxonomy that places forward / future against option / swap
- [`deriv-put-call-parity.md`](deriv-put-call-parity.md) — parity uses the synthetic long-forward `C - P` decomposition

## Escalate to Raw When

Open Hull chapters 2 and 5 directly when any of the criteria
below applies. **Source:** Hull §2 pp.43-58; §5 pp.110-138.

- The contract is a futures spread, calendar spread, or
  basis-trade; multi-leg payoff aggregation departs from the
  single-leg picture in this card. **Source:** Hull §2
  pp.43-58.
- Cost-of-carry includes storage costs, convenience yield, or
  income (commodity futures, equity-index dividends); the
  formula generalizes per Hull §5. **Source:** Hull §5
  pp.110-138.
- Stochastic-rate convexity adjustment matters (long-dated
  futures vs forward, eurodollar / overnight-index swap
  basis). **Source:** Hull §5 pp.130-138.
