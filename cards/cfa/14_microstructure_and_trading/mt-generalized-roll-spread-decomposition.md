---
schema_version: "cacg.v0"
id: "mt-generalized-roll-spread-decomposition"
title: "Generalized Roll Model: Decomposing the Spread into Adverse-Selection and Cost Components"
reading_id: "14_microstructure_and_trading"
summary: "Letting the efficient-price increment carry a trade-direction term w_t = lambda*q_t + u_t makes the half-spread c+lambda, splitting the spread into a permanent adverse-selection part lambda and a transient noninformational cost c."
tags: ["microstructure", "bid-ask-spread", "adverse-selection", "roll-model", "price-impact"]
citations:
  - source_id: "mt_hasbrouck_2007_empirical_market_microstructure"
    chunk_id: "mt_hasbrouck_2007_empirical_market_microstructure:p078:0097"
    chunk_hash: "4ab8c67142ce4eb6c9066017c51fd68880f7392f459c375be78b61150aa0ef6c"
    page_range: [79, 79]
    quote: "Here, λqt reflects the information content of the time-t trade"
    edge_type: "defines"
card_hash: "ea0ffd9e90e1ff2ef7a4bc5b3edf470dfea71ea5d42aa7c8af04a3076460f03e"
---
# Generalized Roll Model: Decomposing the Spread into Adverse-Selection and Cost Components

## Intuition
The original Roll model treats the bid-ask spread as a pure transaction cost: the dealer earns `c` per round trip, the efficient price wanders as an exogenous random walk, and trade direction tells you nothing about where value is headed. That is too clean. In real markets a buy order is mild evidence that *someone* knows the asset is cheap, so a dealer who fills it should rationally mark the efficient price up a little, not just collect a fee. The generalized Roll model adds exactly this channel.

The move is to let the efficient-price innovation absorb a piece of the trade-direction indicator. Write the efficient price `m_t` as a random walk whose increment is `w_t = λq_t + u_t`, where `q_t = +1` for a customer buy and `q_t = −1` for a customer sell. The `u_t` term is ordinary (nontrade) public news; the `λq_t` term is the permanent revision the market makes *because a trade happened in a given direction*. So now the observed transaction price `p_t = m_t + cq_t` reflects two distinct frictions stacked on top of news: a recoverable cost `c` and an information-driven impact `λ`.

```
   trade arrives (q_t = +1, a BUY)
            |
            v
  efficient price jumps PERMANENTLY by  +λ   <- adverse selection (sticks)
            +
  price quoted TRANSIENTLY higher by   +c    <- dealer cost (reverses next trade)
            =
  buyer pays  m_{t-1} + u_t + (c + λ)        <- the ASK
  seller gets m_{t-1} + u_t − (c + λ)        <- the BID
                          \________  ________/
                                   \/
                       half-spread = c + λ ;  full spread = 2(c + λ)
```

The half-spread is no longer just `c`; it is `c + λ`. Half of it (`c`) is a temporary, mean-reverting wedge between price and value; the other half (`λ`) is a permanent shift in value itself. That is the decomposition.

**Source:** Hasbrouck (2007) §8.1–8.2 pp.79-80

## Definition
The structural model keeps a random-walk efficient price but gives its increment two components (Hasbrouck eq. 8.1):

- Efficient price: `m_t = m_{t-1} + w_t`, with `w_t = λq_t + u_t`.
- `q_t ∈ {+1, −1}` is the trade-direction indicator (`+1` buy / at the ask, `−1` sell / at the bid).
- `λ > 0` is the adverse-selection (price-impact) coefficient: the permanent revision in the efficient price attributable to the information content of a directional trade.
- `u_t` is the nontrade public-information innovation, with `Corr(q_t, u_t) = 0`.
- `c` is the noninformational per-trade cost (clearing, clerical, etc.), entering only the transaction price: `p_t = m_t + cq_t`.

Quoted prices are set symmetrically about `m_{t-1} + u_t`: the ask is `m_{t-1} + c + λ + u_t` and the bid is `m_{t-1} − c − λ + u_t`, so the spread equals `2(c + λ)`. Setting `λ = 0` collapses the model back to the basic Roll spread (pure cost); setting `c = 0` leaves a pure information spread.

**Source:** Hasbrouck (2007) §8.2 pp.79-80

## Mathematical Reasoning
Substitute the efficient-price recursion into the transaction-price definition to get the observed first difference (Hasbrouck eq. 8.2):

```
Δp_t = p_t − p_{t-1} = c(q_t − q_{t-1}) + λ q_t + u_t.
```

The `c(q_t − q_{t-1})` block is the reversing Roll term (it changes sign as the trade flips sides); the `λq_t` block is the non-reversing impact carried into the efficient price; `u_t` is news. From this the price-change autocovariances follow (eqs. 8.3):

```
γ_0 = Var(Δp_t)        = c² + (c + λ)² + σ_u²
γ_1 = Cov(Δp_t, Δp_{t-1}) = − c(c + λ)
γ_k = 0   for all k ≥ 2.
```

Comparative statics read directly off these expressions. First, the negative first-order autocovariance — the bid-ask "bounce" — is now `−c(c + λ)` rather than the basic-Roll `−c²`; the information term `λ` widens the magnitude of the observed reversal even though `λ` itself is not a reversing cost. Second, because only `γ_0` and `γ_1` (equivalently the MA(1) parameters `θ, σ_ε²`) are observable, the three structural parameters `{λ, c, σ_u²}` are **not** separately identified from prices alone: one must be pinned by restriction or by adding trade data.

One combination *is* identified without restriction. From eq. 8.1 the efficient-price-change variance is

```
σ_w² = Var(w_t) = λ² + σ_u² = γ_0 + 2γ_1,
```

which is exactly the long-horizon, time-scaled variance `Var(m_t − m_{t-k}) = k σ_w²`. Intuitively, over long windows microstructure (the `c`-driven bounce) washes out and the random walk dominates, so `σ_w²` is what you recover from low-frequency return variance. Separating `λ` from `c` is precisely what requires the richer multivariate (trade-and-quote) machinery.

**Source:** Hasbrouck (2007) §8.2–8.3 pp.80-81

## Boundary Notes
- **Identification.** Univariate price data identify `σ_w² = λ² + σ_u²` but not `{λ, c, σ_u²}` individually. To split adverse selection from cost you need either a restriction (`λ=0`, `c=0`, or `σ_u²=0`) or trade-direction data in a bivariate model — the natural sequel.
- **Orthogonality assumption.** The decomposition relies on `Corr(q_t, u_t) = 0` and (in the simplest form) serially uncorrelated `q_t`. Real order flow is positively autocorrelated (buys follow buys); when that holds, the increment to the efficient price is driven by the *order innovation* `q_t − E[q_t | past]`, and the naive estimates are biased. The model holds cleanly only as an approximation to that richer dynamic.
- **Permanent vs. transient.** `λ` is permanent (it enters `m_t`); `c` is transient (it enters only `p_t − m_t = cq_t` and reverses on the next trade). This is the load-bearing economic contrast: the pricing error `s_t = p_t − m_t = cq_t` has variance `σ_s² = c²`, which (like `c`) is unidentified but bounded below.
- **Dealer-market framing.** The `liquidity supplier vs. demander` cost story is cleanest in a quote-driven dealer market; in a limit-order book the supplier/demander roles blur, but the question "how closely does `p_t` track `m_t`?" still applies.

**Source:** Hasbrouck (2007) §8.3–8.5 pp.81-82

## See Also
- [`mt-roll-implicit-spread-estimator`](./mt-roll-implicit-spread-estimator.md) -- the `λ=0` special case this model generalizes
- [`mt-kyle-lambda-market-depth-price-impact`](./mt-kyle-lambda-market-depth-price-impact.md) -- the same `λ` as the structural price-impact / depth parameter
- [`mt-spread-decomposition-components`](./mt-spread-decomposition-components.md) -- broader taxonomy of spread components (order-processing, inventory, adverse selection)
- [`mt-hasbrouck-var-trades-quotes`](./mt-hasbrouck-var-trades-quotes.md) -- the bivariate VAR that actually separates `λ` from `c`

## Escalate to Raw When
Read Hasbrouck §8 directly when you need: the full derivation of the filtered efficient-price estimate `f_t = E*[m_t | p_t, p_{t-1}, …] = p_t + θε_t` and the proof that it equals `m_t` exactly under `σ_u²=0` (§8.4–8.5); the formal lower bound on the pricing-error variance `σ_s²` (eq. 8.6); or the rationale for why `σ_w² = γ_0 + 2γ_1` extends to multiple lags and multivariate/multi-price models. The card asserts these results but only sketches the MA(1) and projection algebra behind them.
