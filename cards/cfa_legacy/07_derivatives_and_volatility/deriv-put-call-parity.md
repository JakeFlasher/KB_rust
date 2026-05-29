---
schema_version: "cacg.v0"
id: "deriv-put-call-parity"
title: "Put-Call Parity"
reading_id: "07_derivatives_and_volatility"
summary: "European put-call parity is the model-free no-arbitrage identity C - P = S0 - K·exp(-rT) linking call, put, spot, and risk-free bond. The proof is a portfolio dominance argument: a long call plus a bond paying K replicates a long put plus the underlying at expiry (both pay max(ST,K)), forcing equal initial cost."
tags: ["derivatives", "put-call"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p256:0391"
    chunk_hash: "9bab8ea09e52eb4a86f5ff56d3a37d9a9b3db8885927c250ec8984a90ec6ffef"
    page_range: [256, 256]
    quote: "Portfolio A is overpriced relative to portfolio C. An arbitrageur can short the securities in portfolio A and buy the securities in portfolio C"
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p264:0401"
    chunk_hash: "cbf119e1e77dcb1fff8514ba7cf31f335128bfd7473530d71ea7bc1ae95f7e04"
    page_range: [264, 264]
    quote: "Put–call parity is a relationship between the price, c, of a European call option on a stock and the price, p, of a European put option on a stock"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2896:4342"
    chunk_hash: "8c99182748ecf0ae39ed24e11dedb7bbd595a6a70db9efd2d82cb99f47431ad7"
    page_range: [2896, 2896]
    quote: "Comparing Exhibit 13 with Exhibit 14 shows that a protective put and a fiduciary call produce the same result"
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2897:4343"
    chunk_hash: "165526a9e7563bf985570d3497016bcc5169d3a38c709a64719ad9ba15bbd5c7"
    page_range: [2897, 2897]
    quote: "Put–call parity does not tell us which price is correct, and it requires knowledge of one price to get the other"
    edge_type: "supports"
card_hash: "ffab0d674eb4243346c9ac68a58528a69146e35a5868ba0ec2bcfb3486e51437"
---
# Put-Call Parity

## Intuition

European put-call parity is the algebraic identity that links
the price of a European call, a European put, the underlying,
and a risk-free zero-coupon bond at the same strike. It says:
holding a long call and a short put with strike `K` and expiry
`T` is equivalent to holding the underlying minus the bond
that pays `K` at `T`. This identity holds without any pricing
model; it is a pure no-arbitrage relationship that constrains
how the two option prices can move relative to each other.
**Source:** Hull §11 pp.260-275.

```
synthetic long-forward decomposition

  long call           short put          long stock          bond paying K
  C(K, T)        -    P(K, T)       =    S_0           -     K · exp(-r · T)

  +-------+    -    +-------+      =    +-----+    -    +-----+
  |       |         |       |           |     |         |     |
  | C     |         | P     |           |  S  |         |  B  |
  |       |         |       |           |     |         |     |
  +-------+         +-------+           +-----+         +-----+
  hockey            inverse-            linear          flat (PV
   stick             hockey              up              of K at T)

  payoff of (C - P) at T = (S_T - K), the long-forward payoff.
```

## Definition

European put-call parity for a non-dividend-paying underlying
states that, for any strike `K` and expiry `T`,
`C(K, T) - P(K, T) = S_0 - K · exp(-r · T)`,
where `C(K, T)` is the European call price, `P(K, T)` is the
European put price, `S_0` is the spot price of the underlying,
`r` is the risk-free continuously-compounded rate, and `T` is
the time to expiry. The right-hand side is the present value
of a long-forward position struck at `K`. **Source:** Hull §11
pp.260-275; CFA L1 Curriculum (2022) Vol.5/pp.410-425.

For a dividend-paying underlying with continuous dividend yield
`q`, the spot is replaced by the present-value-adjusted spot
`S_0 · exp(-q · T)`:
`C(K, T) - P(K, T) = S_0 · exp(-q · T) - K · exp(-r · T)`.
For a discrete dividend `D` paid at `t_D`, the adjustment is
`-D · exp(-r · t_D)` subtracted from `S_0`. **Source:** Hull
§11 pp.260-275.

## Mathematical Reasoning

The proof is a one-period replication argument. Define
Portfolio I as a long European call `C(K, T)` plus a
zero-coupon bond paying `K` at `T`, and Portfolio II as a
long European put `P(K, T)` plus one share of underlying.
At expiry `T`, Portfolio I pays
`max(S_T - K, 0) + K = max(S_T, K)` (the call exercises if
`S_T > K`, otherwise the bond delivers `K`); Portfolio II
pays `max(K - S_T, 0) + S_T = max(K, S_T)`. Since the two
portfolios have identical terminal payoffs, no-arbitrage
forces their initial values to be equal:
`C + K · exp(-r · T) = P + S_0`. Rearranging gives the
parity formula. **Source:** Hull §11 pp.260-275.

The synthetic-position interpretation: parity says
`(long call) - (long put) = (long underlying) - (long bond
paying K)`. The right-hand side is the synthetic long-forward
struck at `K`, with present value `S_0 - K · exp(-r · T)`. The
left-hand side is a long call financed by a short put, not a
straddle; it is the option-pair representation of the same
directional forward exposure. The two views are algebraically
equivalent: parity is the identity that makes this equivalence
hold. **Source:** Hull §11 pp.260-275.

Parity must hold for any arbitrage-free pricing of the two
options. Empirical violations are absorbed into transaction
costs, dividends, borrow costs, or counterparty / collateral
spreads. The boundary into the practitioner basis-trade
machinery and 06's CDS-bond basis (a cousin form of
"two assets, same payoff, different prices") lies past this
card. **Source:** Hull §11 pp.260-275.

## See Also

- [`deriv-option-payoff-anatomy.md`](deriv-option-payoff-anatomy.md) — call / put payoff at expiry that the parity argument matches
- [`deriv-forward-and-futures-payoff.md`](deriv-forward-and-futures-payoff.md) — the synthetic long-forward `C - P` decomposition
- [`deriv-no-arbitrage-bounds.md`](deriv-no-arbitrage-bounds.md) — parity-derived call / put price bounds

## Escalate to Raw When

Open Hull chapter 11 or CFA L1 Curriculum Vol.5 Reading 46
directly when any of the criteria below applies. **Source:**
Hull §11 pp.260-275; CFA L1 Curriculum (2022)
Vol.5/pp.410-425.

- The underlying pays discrete dividends, has stochastic
  borrow cost, or is hard-to-locate; the parity formula's
  PV-adjustment terms generalize. **Source:** Hull §11
  pp.260-275.
- The options are American-style; parity becomes an
  inequality because the American put can carry an early-exercise
  premium. For a non-dividend-paying underlying, Hull gives the
  boundary `S_0 - K ≤ C - P ≤ S_0 - K · exp(-r · T)`.
  **Source:** Hull §11 pp.270-275.
- The card needs the BSM closed-form expressions for the
  individual option prices (parity is model-free; BSM is
  model-specific); BSM lives in a later 07 batch.
  **Source:** Hull §15 pp.346-380.
