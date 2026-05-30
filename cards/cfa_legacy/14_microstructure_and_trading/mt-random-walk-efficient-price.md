---
schema_version: "cacg.v0"
id: "mt-random-walk-efficient-price"
title: "Random-Walk / Martingale Model of the Fundamental (Efficient) Price"
reading_id: "14_microstructure_and_trading"
summary: "The unobservable efficient price is the public-information conditional expectation of terminal value; it evolves as a driftless random walk (martingale), and observed trade prices equal it plus a transient bid-ask component."
tags: ["microstructure", "random-walk", "martingale", "efficient-price", "roll-model"]
citations:
  - source_id: "mt_hasbrouck_2007_empirical_market_microstructure"
    chunk_id: "mt_hasbrouck_2007_empirical_market_microstructure:p039:0046"
    chunk_hash: "c355d81dafee437e5af817e6b4cadb75d356cb55129a64ceebd1b2560093a576"
    page_range: [40, 40]
    quote: "we keep the random-walk assumption but now apply it to the (martingale) efficient price instead of the actual transaction price"
    edge_type: "defines"
card_hash: "8062a92eaee95e4e4bcd5eb4ad3c6fb04dc58c211ad9dd0e0dc93c3984ba3d0f"
---
# Random-Walk / Martingale Model of the Fundamental (Efficient) Price

## Intuition
Before microstructure theory turned attention to the trading process itself, the standard statistical model for a security price was the random walk: tomorrow's price equals today's price plus an unforecastable, information-driven increment. That model is no longer accepted as a complete description of *short-term* price dynamics — bid-ask bounce and other frictions intervene — but it survives as the right model for the security's underlying *fundamental value*. The trick of modern microstructure is to keep the random-walk assumption but relocate it: instead of imposing it on the observed transaction price, we impose it on a latent, unobservable price that Hasbrouck calls the **efficient price** `m_t`.

Conceptually, `m_t` is what a frictionless, fully-informed market would post. It is the conditional expectation of the security's eventual payoff given all public information. As information accrues and beliefs update, `m_t` wanders without systematic direction — a martingale. The price we actually see on the tape is this clean efficient price contaminated by a transient overlay (the dealer's spread, trade-direction bounce), which is exactly why short-run observed price changes are *not* themselves a martingale.

```
   information arrives (i.i.d. shocks u_t)
        |        |        |        |
        v        v        v        v
   m_0 ----> m_1 ----> m_2 ----> m_3 ...   efficient price = driftless random walk
        +        +        +        +
   transient bid-ask / bounce component (mean-reverting, NOT permanent)
        =        =        =        =
   p_0      p_1      p_2      p_3 ...       observed transaction prices
```

**Source:** Hasbrouck (2007) §3.2, §3.4 pp.25-29

## Definition
Let `p_t` be the transaction price at calendar time `t`. The random-walk model (with drift) is `p_t = p_{t-1} + mu + u_t`, where the `u_t` are i.i.d. random variables arising from new information bearing on security value and `mu` is the expected price change (drift). When `mu = 0`, the price cannot be forecast beyond its most recent value, `E[p_{t+1} | p_t, p_{t-1}, ...] = p_t` — the **martingale** property. More generally, `{x_t}` is a martingale with respect to an information sequence `{Omega_k}` if `E|x_t| < infinity` and `E(x_{t+1} | Omega_t, Omega_{t-1}, ...) = x_t`.

When the conditioning information is "all public information," the conditional expectation `x_t = E[v | Omega_t]` of the terminal payoff `v` is called the **fundamental value** or **efficient price** of the security. A random walk — a cumulated sum of i.i.d. zero-mean variables — is a special case of a martingale.

**Source:** Hasbrouck (2007) §3.2 pp.25-26

## Mathematical Reasoning
The efficient price is defined directly as `m_t = E[v | Omega_t]`. Because the information sequence does not contract (anything known at time `t` is known at any later `tau > t`), the law of iterated expectations gives `E[m_{t+1} | Omega_t] = E[E[v | Omega_{t+1}] | Omega_t] = E[v | Omega_t] = m_t`. Hence `m_t` is a martingale *by construction*, with no auxiliary equilibrium assumption needed beyond a non-contracting information filtration.

Dropping the drift (`mu = 0`) is justified because at microstructure sampling frequencies the expected per-interval return is negligible relative to the variance of the information shocks, so `m_t = m_{t-1} + u_t` with `Cov(u_s, u_t) = 0` for `s != t`. The increments of a random walk are therefore uncorrelated, so the autocorrelations of efficient-price changes satisfy `rho_k ~ 0` for `k != 0`.

Observed prices, however, layer a transient component on top: in the Roll setup `p_t = m_t + q_t c`, where `q_t = +1` (customer buys, lifts the ask) or `-1` (customer sells, hits the bid) and `c` is the half-spread. The transaction-price *changes* then carry the bounce term `(q_t - q_{t-1})c`, which induces *negative* first-order autocorrelation in `Delta p_t` even though `Delta m_t` is serially uncorrelated. This is precisely why empirically estimated `rho_1` of short-run price changes is typically negative — the gap between the martingale fundamental and the non-martingale observed price.

**Source:** Hasbrouck (2007) §3.2, §3.4 pp.25-29

## Boundary Notes
- The martingale result for asset prices classically follows from individual optimization, absence of arbitrage, or market equilibrium — but those derivations assume *frictionless* trading, which is inappropriate in microstructure. Here the efficient price is martingale-by-definition (iterated expectations), not by an equilibrium argument; the friction (the spread) is then added back explicitly.
- Time-homogeneity (same behavior whenever sampled) is what makes the i.i.d.-increment random walk sensible. It holds well for equities as claims on ongoing economic activity, but **fails** for finite-maturity instruments (bonds, swaps, options) whose maturity boundary conditions pull values toward known terminal values; a random walk may fit them over short samples but mis-describes long-run behavior.
- Observed transaction prices are usually NOT martingales, and public information need not even include the history of trades (in dealer markets trades are often unreported). One can still *recover* a martingale component by imposing economic or statistical structure (the Roll model is the simplest such structure).

**Source:** Hasbrouck (2007) §3.2 pp.25-26

## See Also
- [`mt-roll-implicit-spread-estimator`](./mt-roll-implicit-spread-estimator.md) -- adds `p_t = m_t + q_t c` and backs out the spread from `Delta p_t` autocovariance
- [`mt-permanent-vs-transitory-price-components`](./mt-permanent-vs-transitory-price-components.md) -- decomposes observed prices into the random-walk (permanent) part and the mean-reverting (transitory) part
- [`mt-prices-martingale-information-process`](./mt-prices-martingale-information-process.md) -- the information-set martingale framing `m_t = E[v | Omega_t]`

## Escalate to Raw When
Hasbrouck establishes the martingale property of `E[v | Omega_t]` and the general-information-set martingale definition (def. 1.2 in Karlin and Taylor) more carefully than this card sketches; re-read pp.25-26 for the precise filtration conditions. For the algebra connecting the random-walk efficient price to the observed-price autocovariance structure and the resulting spread estimator, re-read the Roll-model derivation on pp.29-30 (Eq. 3.3-3.4).
