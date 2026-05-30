---
schema_version: "cacg.v0"
id: "mt-information-shares-price-discovery"
title: "Information Shares: Where Price Discovery Happens Across Venues"
reading_id: "14_microstructure_and_trading"
summary: "When one security trades on several venues whose prices are cointegrated, a VECM implies a single common efficient price; each venue's information share is its share of that common-factor innovation variance."
tags: ["microstructure", "price-discovery", "information-share", "cointegration", "vecm"]
citations:
  - source_id: "mt_hasbrouck_2007_empirical_market_microstructure"
    chunk_id: "mt_hasbrouck_2007_empirical_market_microstructure:p111:0142"
    chunk_hash: "1d3b2766fc8557866a343f1672c9a5d3e9cf785341d55886e428efe07a213970"
    page_range: [112, 112]
    quote: "One property developed earlier for the structural model, however, is general: The rows of θ(1) are identical."
    edge_type: "defines"
---
# Information Shares: Where Price Discovery Happens Across Venues

## Intuition
A single security often trades simultaneously in many places — a primary exchange, ECNs, dark pools, a futures or ETF market on the same underlying. Each venue posts its own quotes and prints its own trades, so at any instant the observed prices differ slightly. But they cannot wander apart without bound: arbitrage and cross-venue order routing tether them together. They share one underlying fundamental value. The natural question is operational, not philosophical: when new information arrives, *which venue's price moves first and pulls the others along?* That venue is the locus of price discovery.

Hasbrouck's answer formalizes the tether as *cointegration*. Although each venue's price is individually a (near-) random walk, the differences between venues are stationary — they oscillate around a long-run constant rather than drifting. Cointegration of the price vector implies there is exactly one common stochastic trend driving all of them: a single scalar random-walk "efficient price." Innovations to that common trend are permanent (they revise fundamental value); everything else is transitory microstructure noise. The information share of a venue is the fraction of the variance of those permanent innovations that can be attributed to that venue's own price shocks.

```
   venue 1 quotes  -+
   venue 2 quotes  -+        single scalar
   venue 3 trades  -+-->  efficient price  m_t  (random walk, permanent)
        ...        -+            |
                                 v
   each venue price = m_t·ι  +  transitory noise s_t   (stationary, mean-reverting)

   information share of venue i = share of Var(common-factor innovation w_t)
                                  contributed by venue i's own innovation
```

The venue with the largest information share is the price leader: it is "the venue where most value-relevant information is first revealed," and the other venues' prices follow its moves.

**Source:** Hasbrouck (2007) §10.3.1 pp.111-113

## Definition
For a security with `n` prices (one per venue), stack them in `p_t` (n×1) and posit the random-walk decomposition

```
p_t = m_t · ι + s_t ,   m_t = m_{t-1} + w_t ,
```

where `ι` is a vector of ones, `s_t` is stationary, and crucially `m_t` is a **scalar** — the common efficient price shared by all venues. The dynamics are written as a Vector Error-Correction Model (VECM):

```
Δp_t = φ_1 Δp_{t-1} + φ_2 Δp_{t-2} + ... + β(z_{t-1} − b) + ε_t ,
```

where `z_{t-1} = A' p_{t-1}` collects the stationary cointegrating combinations (e.g. pairwise price differences `p_1 − p_i`), `b` is the vector of long-run mean "errors" (discrepancies, not disequilibria), and `β` is the matrix of speed-of-adjustment coefficients. From the VECM one recovers the vector moving-average (VMA) form `Δp_t = θ(L)ε_t`.

The **information share** of price `i` is the relative contribution of venue `i`'s innovations to the variance of the common efficient-price innovation `w_t`.

**Source:** Hasbrouck (2007) §10.3.1 pp.110-112

## Mathematical Reasoning
Because all `n` prices share one scalar common trend, a structural property carries over from the single-security model: **the rows of `θ(1)` are identical** (each row is the long-run cumulative impulse response, and a permanent shock must move every venue's price equally in the long run). Hence the common random-walk innovation variance can be computed from the first row:

```
σ²_w = [θ(1)]_1 · Ω · [θ(1)]_1' ,
```

where `Ω` is the covariance matrix of the structural disturbances `ε_t`. To split this variance across venues, factor `Ω = F'F` with `F` its Cholesky factor and define the row vector `d = [θ(1)]_1 · F'`. Then

```
σ²_w = Σ_i d_i² .
```

The absolute contribution of venue `i` is `d_i²`; its **information share** is the relative contribution

```
IS_i = d_i² / σ²_w ,    with   Σ_i IS_i = 1 .
```

Comparative-statics intuition: a venue whose price shocks load heavily onto the permanent common factor (large `d_i²`) leads price discovery; a venue whose shocks are mostly transitory contributes little. The speed-of-adjustment coefficients `β` carry the dual reading — like a bargaining game, the venue that "concedes" the least (adjusts its price least toward the others) is the stronger, leading market; the venue that moves the most is the follower.

A caveat the math forces: the Cholesky factor `F` imposes a causal ordering, so `IS_i` is order-dependent unless `Ω` is diagonal. Hasbrouck's remedy is to report, for each venue, the **minimum and maximum information share over all causal permutations**, yielding bounds rather than a point estimate. No worked arithmetic is given or needed; the result is a variance-decomposition identity.

**Source:** Hasbrouck (2007) §10.3.1 pp.112-113

## Boundary Notes
- **Requires cointegration of one security's prices.** The construction assumes the venue prices share a single common trend; their differences must be stationary. In microstructure this is usually a structural certainty (bids, asks, and trade prices for one security "cannot reasonably diverge without bound"), so cointegration testing is less of a concern than in macroeconomics — except in **pairs trading**, where the cointegrating relation is estimated and is vulnerable to data-snooping bias and structural breaks.
- **Bounds widen with time aggregation.** When the off-diagonal elements of `Ω` are large, the Cholesky ordering matters more and the min/max bounds across permutations grow wide. Coarser sampling (e.g. one minute vs. one second) makes truly-sequential events appear contemporaneous, inflating off-diagonal covariances; a **shorter time interval gives tighter bounds**. With `Ω` diagonal the information shares are exactly determined.
- **Information share vs. Gonzalo–Granger component.** A related but distinct measure normalizes the long-run VMA coefficients to sum to one, `β = θ(1)_1 / (ι' θ(1)_1)`, giving the permanent/transitory (GG) common-factor weights. Information shares attribute *innovation variance*; GG components attribute *long-run weight*. They generally differ and are debated in the literature (special issue of the *Journal of Financial Markets*, 2002).
- **Scope of `m_t`.** `m_t` is a single scalar for the security; the framework can be extended (eq. 10.16) to add order-flow variables or prices of *non-cointegrated* securities, but those must not be cointegrated with the original price set.

**Source:** Hasbrouck (2007) §10.3.1-10.4 pp.112-115

## See Also
- [`mt-permanent-vs-transitory-price-components`](./mt-permanent-vs-transitory-price-components.md) -- the random-walk/efficient-price vs. stationary-noise split that the information share decomposes
- [`mt-hasbrouck-var-trades-quotes`](./mt-hasbrouck-var-trades-quotes.md) -- the VAR/VMA and variance-decomposition machinery (section 8.6) reused here for multiple prices
- [`mt-market-fragmentation`](./mt-market-fragmentation.md) -- why one security has multiple competing venues whose price-discovery roles differ
- [`mt-index-portfolio-markets-design`](./mt-index-portfolio-markets-design.md) -- index-arbitrage application where component weights define the cointegrating vector (ETF/index price discovery)

## Escalate to Raw When
Re-read Hasbrouck §10.3.1 (pp.110-113) for the exact derivation that the rows of `θ(1)` are identical and for the `d = [θ(1)]_1 F'` Cholesky decomposition; §8.6 for the general random-walk variance-decomposition that this section reuses; §9.4 for the trades-first causal ordering rationale behind the Cholesky factorization; and §10.4 (pp.114-115) for the wall-clock-time sampling and bound-tightening arguments. For the comparison of information shares against Gonzalo–Granger common-factor weights, follow the cited 2002 *Journal of Financial Markets* special issue references.
