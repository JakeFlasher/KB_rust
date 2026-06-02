---
schema_version: "cacg.v0"
id: "mt-hasbrouck-var-trades-quotes"
title: "Hasbrouck VAR: Vector Autoregression of Trades and Quotes"
reading_id: "14_microstructure_and_trading"
summary: "Stacking price changes and signed trades into a bivariate VAR/VMA attributes random-walk price variance to public versus trade-related (private) information and recovers the permanent impact of a trade."
tags: ["microstructure", "vector-autoregression", "price-impact", "information-asymmetry", "random-walk-decomposition"]
citations:
  - source_id: "mt_hasbrouck_2007_empirical_market_microstructure"
    chunk_id: "mt_hasbrouck_2007_empirical_market_microstructure:p096:0121"
    chunk_hash: "b279bf2a83c2da4c46b2aa3bb8fe2558e8c937572c67e39d09971253fc4d0907"
    page_range: [97, 97]
    quote: "decomposition of the random-walk variance provides a basis for measuring the importance of different sources of market"
    edge_type: "defines"
card_hash: "fc779873a39f1aec1e7ab46464487a9676306585dad37c002afa0b5bcd48007d"
---
# Hasbrouck VAR: Vector Autoregression of Trades and Quotes

## Intuition
A single trade price tells you only so much; its dynamics become legible once
you watch it alongside the order flow that moves it. Hasbrouck's multivariate
program collects the price change and one or more supplementary variables —
most simply the signed trade indicator — into one vector and lets the data
describe how each piece predicts the others. Because trades are positively
autocorrelated (buys follow buys) and because a buy reveals something an
uninformed liquidity supplier did not know, the cross-dynamics between order
flow and price carry the fingerprints of information.

The payoff is an attribution. Some of a stock's permanent price movement is
the market reacting to public news that no trade was needed to reveal; the
rest is the market learning from the order flow itself — the private-information
channel of the sequential and strategic trade models. Fitting the joint time
series lets you split the permanent (random-walk) variance of price into these
two buckets and read off how much of price discovery runs through trades.

```
   public news u_t ---------------+
                                   v
 order innovation v_t --> λ --> efficient-price shock w_t = u_t + λ v_t
                                   |
                                   v   (+ transient spread/inventory term c q_t)
                              observed Δp_t
   stack:  y_t = [ Δp_t , q_t ]'  -- VMA θ(L) / VAR φ(L) --> variance attribution
```

**Source:** Hasbrouck (2007) ch.9 §9.2, §9.5 pp.89-98

## Definition
Let `y_t = [Δp_t, x_t']'` be an `n×1` covariance-stationary vector series whose
first element is the price change and `x_t` collects supplementary variables;
the illustrative case is the bivariate `y_t = [Δp_t, q_t]'` of price changes and
signed trade directions. By the multivariate Wold theorem the de-meaned process
has a vector moving-average (VMA) representation `y_t = θ(L) ε_t` with
`ε_t` vector white noise, `Var(ε_t) = Ω`. The corresponding vector
autoregression (VAR) is `y_t = φ_1 y_{t-1} + φ_2 y_{t-2} + ... + ε_t`, which
exists when the VMA is invertible; in practice, writing the fitted VAR
autoregressive polynomial as `φ(L)`, the VMA is recovered by inverting it —
`θ(L) = φ(L)^{-1}`, obtained by series expansion. The random-walk (permanent)
component `m_t` evolves as `m_t = m_{t-1} + w_t` with innovation
`w_t = u_t + λ v_t`, where `u_t` is the public-information shock and `v_t` is
the order-flow innovation scaled by the price-impact coefficient `λ`.

**Source:** Hasbrouck (2007) ch.9 §9.1-§9.2 eqs.(9.2)-(9.5) pp.89-91

## Mathematical Reasoning
With signed trades following `q_t = v_t + β v_{t-1}` (β > 0) and the priced
relation `p_t = m_t + c q_t`, price changes stack with trades into a finite VMA
in the structural shocks `[u_t, v_t]'`, which a normalizing matrix `B` puts into
the canonical `θ(L)` form with `θ_k = 0` for `k > 2`. The permanent variance is

```
σ²_w = [θ(1)]₁ Ω [θ(1)]₁'
```

where `[θ(1)]₁` is the row of the summed MA coefficients corresponding to price.
When `Ω` is diagonal each variable contributes one additive term to `σ²_w`; when
`Ω` is non-diagonal a Cholesky factor `F` of `Ω` yields `d = [θ(1)]₁ F'` and the
squared entries `d_i²` are the per-source contributions. Ordering trades "first"
in the factorization identifies the structural ordering: a one-`σ_v` shock to
`q_t` causes an immediate `(c+λ)σ_v` shock to `p_t`. Carrying through the
algebra gives `σ²_w = σ²_u + λ² σ²_v`, cleanly separating the public-information
piece `σ²_u` from the trade-related ("private information") piece `λ² σ²_v`. The
relative measure `λ² σ²_v / σ²_w` behaves like the R² of projecting price changes
on trades, and `λ σ_v` (in log prices) approximates the standard deviation of the
trade-driven return. Because the model embeds trades, the pricing-error lower
bound `σ²_s` is attained exactly here — stronger than the Roll-model case, which
was exact only when all information was trade-related.

**Source:** Hasbrouck (2007) ch.9 §9.5 eqs.(9.6)-(9.25) pp.91-98

## Boundary Notes
The construction assumes covariance stationarity and that the supplementary
variables `x_t` are NOT cointegrated with the price under study — closely related
or cross-market prices belong to the cointegrated-VAR / common-trends treatment
of the following chapter, not this single-price model. Placing `Δp_t` first is
purely expositional and implies no causal priority; the informational attribution
likewise depends on the imposed Cholesky ordering when `Ω` is non-diagonal, so
the split into public versus trade-related variance is identified only under that
ordering assumption. The structural illustration is deliberately stylized (finite
MA(2), serially correlated trades) and does not exhibit the full dynamic
interactions a general VAR can accommodate. The decomposition equates the
trade-related channel with asymmetric information, a mapping borrowed from
sequential/strategic trade models rather than proved within the linear model.

**Source:** Hasbrouck (2007) ch.9 §9, §9.6 pp.89, 97-98

## See Also
- [`mt-generalized-roll-spread-decomposition`](./mt-generalized-roll-spread-decomposition.md) -- the univariate structural antecedent this bivariate model generalizes
- [`mt-permanent-vs-transitory-price-components`](./mt-permanent-vs-transitory-price-components.md) -- the random-walk/pricing-error split that the VAR variance decomposition operationalizes
- [`mt-information-shares-price-discovery`](./mt-information-shares-price-discovery.md) -- multi-market extension attributing price discovery across venues
- [`mt-price-impact-measures-amihud`](./mt-price-impact-measures-amihud.md) -- reduced-form price-impact proxies contrasted with the VAR-estimated λ

## Escalate to Raw When
The source derives the explicit VMA coefficient matrices `θ_1, θ_2` and the
covariance `Ω` (eqs. 9.7-9.9), proves the pricing-error lower bound is exact for
this structural model (eqs. 9.24-9.25), and details the Cholesky-ordering
identification argument — all only sketched here. Re-read Hasbrouck (2007) ch.9
§9.2-§9.6 pp.89-98 (and ch.8 for the random-walk/pricing-error definitions) for
the full matrix algebra and the invertibility/ordering caveats before applying
the estimator.
