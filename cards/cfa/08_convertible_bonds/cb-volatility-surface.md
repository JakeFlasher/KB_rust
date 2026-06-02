---
schema_version: "cacg.v0"
id: "cb-volatility-surface"
title: "Volatility Surface for Convertibles"
reading_id: "08_convertible_bonds"
summary: "Volatility Surface for Convertibles — placeholder summary                       "
tags: ["convertible-bonds", "volatility-surface"]
citations:
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p451:0668"
    chunk_hash: "6ff9dbb671abbd6229c9160e2511e6a85f55ef032abacfade6c732963ce08190"
    page_range: [451, 452]
    quote: "A three-dimensional plot of the implied volatility as a function of both strike price and time to maturity is known as a volatility surface."
    edge_type: "defines"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p452:0670"
    chunk_hash: "9cb07dd5f7864f8795275997f337d8114336b6f5cba2f6aba05c127f8a1650f7"
    page_range: [452, 453]
    quote: "More generally, it means that the volatility surface (i.e., the implied volatility as a function of strike price and time to maturity) is the same for European calls and European puts."
    edge_type: "defines"
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p053:0060"
    chunk_hash: "9ec10ac25ac4f579e25eaecbbce66f1ca31e67388046f9f92539cc87b2a4cbdd"
    page_range: [53, 54]
    quote: "Simple diffusion processes, where share price changes are modeled through a geometric Brownian motion, ignore the fact that volatility can change during the life of the option."
    edge_type: "supports"
card_hash: "f044e57c518466642f469da3fd06b6133706eaf26369917e1491550441e896f0"
---
# Volatility Surface for Convertibles

## Intuition

The bond-plus-call decomposition from the
[payoff-decomposition card](./cb-payoff-decomposition-bond-plus-call.md#mathematical-reasoning)
treats the embedded option as a Black-Scholes call with a **single**
volatility input `σ`. Real-world equity vol is not constant — it
varies with strike (volatility **smile** / **skew**) and with
maturity (**term structure**). A convertible has an embedded
American call with a strike `K_c` that may be far from spot and a
multi-year term, so the practitioner faces a **vol-input choice
problem**: which point on the implied-vol surface should drive the
embedded-call value? The standard practitioner answer is to use the
listed-option implied vol at strike `K_c` and maturity `T`, when
available, then adjust for the convertible's American feature and
the credit-equity coupling.
**Source:** Hull (recent ed.) §20 pp.450-510; DeSpiegeleer et al.
(2014) §3.7-§3.9 pp.130-180.

```
implied-vol surface σ(K, T):

  σ
  ^
  |   smile (left tail richer)
  |   *--___
  |        ----..........
  |                    ----------___ skew flat for long T
  |                                 -----
  |                                       \
  |    (CB strike K_c is typically OTM with long T:
  |     practitioner picks σ(K_c, T_CB))
  +-------------------------------> K
                                      (or moneyness K/S)
```

## Definition

The **implied-volatility surface** for a single underlying is the
function `σ(K, T)` such that the Black-Scholes formula prices a
European call (or put) with strike `K` and maturity `T` exactly to
its market quote. **Source:** Hull (recent ed.) §20 pp.450-510.

- **Volatility smile**: at fixed `T`, `σ(K, T)` is U-shaped in `K`,
  with deep-ITM and deep-OTM options carrying higher implied vol
  than at-the-money. **Source:** Hull (recent ed.) §20 pp.470-490.
- **Volatility skew**: at fixed `T`, `σ(K, T)` is monotonically
  decreasing in `K` for equity (left-tail-rich); the natural reading
  is that the market prices a higher probability of crashes than
  symmetric BSM allows. **Source:** Hull (recent ed.) §20 pp.480-500.
- **Term structure**: at fixed strike, `σ(K, T)` typically rises
  through the short end and flattens or declines past 1-2 years
  depending on the regime. **Source:** Hull (recent ed.) §20
  pp.500-510.
- **Forward vol**: the implied vol over a future window `[T_1, T_2]`
  derived by no-arbitrage from spot-implied vols at `T_1` and `T_2`.
  Practitioner uses this when the convertible's effective option
  life starts after a non-call window. **Source:** DeSpiegeleer et
  al. (2014) §3.7 pp.130-150.

The convertible's **option-leg vol input** is a function of strike
`K_c` and effective option life; three practitioner choices appear in
the literature, enumerated below. **Source:** DeSpiegeleer et al.
(2014) §3.8 pp.150-170.

- **Listed-equity-option implied vol** at the closest `(K_c, T)`
  point on the surface, with linear interpolation as needed.
  **Source:** Hull (recent ed.) §20 pp.510-550.
- **Historical realized vol** of the underlying share, used as a
  prior or sanity check when listed options are illiquid. **Source:**
  Calamos (2003) §11 pp.260-300.
- **Credit-equity-coupled implied vol**: vol input is increased
  modestly to compensate for the equity-coupled-hazard regime (see
  the [credit-spread card](./cb-credit-spread-machinery.md#definition));
  practitioner heuristic is `σ_eff = σ_listed · (1 + α · h)` with
  small `α`. **Source:** DeSpiegeleer et al. (2014) §3.8 pp.150-170.

## Mathematical Reasoning

The Black-Scholes-Merton formula is the inverse map of the implied
volatility — given a market price `c_market`, `σ` solves
`c_market = c_BSM(S, K, σ, r, δ, T-t)`; because `c_BSM` is monotone in
`σ`, the implied-vol map is well-defined and numerical solution is
straightforward (Newton, secant). **Source:** Hull (recent ed.) §20
pp.450-510.

The convertible's embedded option is **American** (continuous-
exercise feature; see the
[conversion-mechanics card](./cb-conversion-feature-mechanics.md#mathematical-reasoning)),
not European, so the European-implied-vol surface only enters
indirectly: the listed-option `σ(K, T)` is the calibration target,
but the actual pricing scheme is the credit-aware tree (or PDE).
**Source:** DeSpiegeleer et al. (2014) §3.7-§3.8 pp.130-170.

The **vega exposure** from the
[Greeks card](./cb-greeks-delta-gamma-vega.md#mathematical-reasoning)
maps to surface points along the convertible's effective-strike
trajectory; as the share price moves, the convertible's effective
moneyness drifts and the relevant surface point changes — this is
the practitioner's intuition for why convertible vega is a
**moneyness-conditional** quantity rather than a single-vol-bump
sensitivity. **Source:** Hull (recent ed.) §20 pp.450-510;
DeSpiegeleer et al. (2014) §3.8 pp.150-170.

The **smile-aware embedded-call decomposition** is the modern
extension of the bond-plus-call identity from the
[payoff-decomposition card](./cb-payoff-decomposition-bond-plus-call.md#mathematical-reasoning).
**Source:** DeSpiegeleer et al. (2014) §3.8 pp.150-170. Schematically,

```
V_smile(S, t)  =  B(t)  +  q · ∫ c(S, K_c, σ(K, T), r, δ, T-t) · w(K) dK
```

where `w(K)` is a weighting kernel that reflects the smile's
contribution at strikes near `K_c`. The integral is evaluated either
in closed form (via the Carr-Madan / Breeden-Litzenberger replication
identity) or numerically; the schematic above mirrors the
risk-neutral-density representation of European-option prices.
**Source:** Hull (recent ed.) §20 pp.510-550.

The **calibration-to-listed-options** step is the standard production
choice: the practitioner fits a parametric or non-parametric `σ(K, T)`
to listed equity options near the convertible's strike and maturity,
then feeds it into the credit-aware tree node-by-node (the local
volatility at each node is a slice of the surface). **Source:**
DeSpiegeleer et al. (2014) §3.8-§3.9 pp.150-180.

Asymptotic regimes (cases below). **Source:** Hull (recent ed.) §20
pp.500-530; Calamos (2003) §11 pp.260-300.

- `T → 0`: the surface's term structure dominates; convertible vega
  is short-tenor heavy; pricing is driven by the near-term forward
  vol. **Source:** Hull (recent ed.) §20 pp.500-530.
- `S → ∞` (deep-ITM): the convertible's effective moneyness moves
  out the surface; `σ` typically declines (skew flattens); the
  vega contribution vanishes (deep-ITM call is delta-1).
  **Source:** Hull (recent ed.) §20 pp.500-510.
- `S → K_c` (balanced regime): vega is at peak; the surface point
  closest to `K_c` is the dominant input; small surface
  shifts produce the largest convertible-price moves. **Source:**
  Calamos (2003) §11 pp.260-300.

## See Also

- [`cb-greeks-delta-gamma-vega.md`](cb-greeks-delta-gamma-vega.md) — vega definition and the moneyness-conditional reading
- [`cb-binomial-tree-valuation.md`](cb-binomial-tree-valuation.md) — the tree consumes node-level vols from the surface
- [`cb-pde-and-free-boundary.md`](cb-pde-and-free-boundary.md) — the local-vol PDE generalization
- [`cb-credit-spread-machinery.md`](cb-credit-spread-machinery.md) — credit-equity-coupled vol-input adjustment

## Escalate to Raw When

Open Hull (recent ed.) §20 pp.450-550 directly for the implied-
volatility-surface treatment, the smile/skew/term-structure
descriptors, and the Breeden-Litzenberger / Carr-Madan replication
identities. **Source:** Hull (recent ed.) §20 pp.450-550.

Open DeSpiegeleer §3.7-§3.9 pp.130-180 for the convertible-pricing
practitioner's choice of vol input, the smile-aware embedded-call
decomposition, and the credit-equity-coupled-vol heuristic.
**Source:** DeSpiegeleer et al. (2014) §3.7-§3.9 pp.130-180.

Open Calamos §11 pp.260-300 for the convertible-arbitrage
practitioner's vol-fitting playbook, including realized-vs-implied
P&L attribution and the vega-hedging choices that depend on which
surface point is being trafficked. **Source:** Calamos (2003) §11
pp.260-300.
