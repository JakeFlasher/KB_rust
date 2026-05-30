---
schema_version: "cacg.v0"
id: "mt-order-imbalance-signal"
title: "Order Imbalance as a Short-Horizon Trading Signal"
reading_id: "14_microstructure_and_trading"
summary: "Quoted-volume imbalance (relative bid vs ask depth) measures buy/sell pressure and predicts short-horizon market-order arrivals and midprice direction, yielding an alpha signal for execution and market-making."
tags: ["microstructure", "order-imbalance", "limit-order-book", "short-horizon-alpha", "markov-chain"]
citations:
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p314:0404"
    chunk_hash: "ad8feb1c821c3d26905532b111a223371bffd9e4bdc06ff47490f0830a8a27fc"
    page_range: [314, 315]
    quote: "order imbalance is positively correlated with price changes"
    edge_type: "defines"
card_hash: "8296886babc65162f2f7d533d91d2173410022d9c4608a661c8faea581389b8d"
---
# Order Imbalance as a Short-Horizon Trading Signal

## Intuition
A limit order book (LOB) is not symmetric. At any instant there is typically more
resting volume on one side than the other. When the bid side is much deeper than
the ask side, there is more standing buying interest than selling interest at the
touch, and the next few market orders and the next midprice move are more likely
to push the price up. Order imbalance turns this lopsidedness into a single number
that summarizes instantaneous buy-versus-sell pressure, and that number carries
predictive power: it forecasts both how fast buy versus sell market orders (MOs)
will arrive and the direction and size of the next small price change.

```
        bid depth          ask depth
        V_bid              V_ask
        ##########  | ###                  imbalance p -> +1  (bid-heavy)
        more buying     thin offer         => up-tick more likely

        ###  |  ##########                 imbalance p -> -1  (ask-heavy)
        thin bid     more selling          => down-tick more likely

        #####  |  #####                    imbalance p -> 0   (neutral)
                                           => no directional edge
```

Because the signal lives in resting depth rather than in executed trades, it is a
*pre-trade* state variable: an algorithm reads it before sending an order. Empirically
the authors find order imbalance is strongly autocorrelated (it persists over
seconds) and is positively correlated with the next price change, so it is exploitable
at high frequency by execution and market-making algorithms.

**Source:** Cartea, Jaimungal & Penalva (2015) Ch.12 §12.1-12.2 pp.312-314

## Definition
Let `V_t^b` be the LO volume posted on the bid side of the LOB and `V_t^a` the LO
volume posted on the ask side. The (limit) **order imbalance** at time `t` is the
ratio of the volume imbalance to the total quoted volume:

```
        V_t^b - V_t^a
  p_t = -------------         p_t in [-1, +1]
        V_t^b + V_t^a
```

so `p_t = +1` is a purely bid-heavy book, `p_t = -1` is purely ask-heavy, and
`p_t = 0` is balanced. The volumes may be measured at-the-touch (best bid/ask
only), over the best n levels, or within n ticks of the midprice; the authors use
at-the-touch because empirically it gives the best predictive-power-versus-complexity
trade-off. For modelling, `p_t` is binned into `K` discrete regimes `Z_t in {1,...,K}`
(e.g. 5 equal bins from sell-heavy to buy-heavy).

**Source:** Cartea, Jaimungal & Penalva (2015) Ch.12 §12.2 pp.313-314

## Mathematical Reasoning
The construction `p_t = (V^b - V^a)/(V^b + V^a)` normalizes the raw depth
difference by total quoted volume, bounding the signal to `[-1, +1]` and making it
comparable across times and assets regardless of the absolute size of the book.

The signal is modelled as a finite-state Markov chain: `Z_t in {1,...,K}` with
transition matrix `A`, where `A_{ij}` is the probability of moving from regime `i`
to regime `j`. The maximum-likelihood estimator of the transition probabilities is

```
  _{ij} = n_{ij} / sum_k n_{ik}
```

where `n_{ij}` counts observed transitions from regime `i` to regime `j` — i.e. each
row is just the empirical transition frequency out of state `i`. This follows from
maximizing the multinomial likelihood of the observed regime sequence, row by row.

Two predictive relationships then make `p_t` a signal rather than noise:

- **Order-flow prediction.** MO arrival intensities depend on the regime. Total MO
  arrival is U-shaped in `p_t` (heaviest when the book is strongly bid- or
  ask-heavy), and conditional on a bid-heavy regime the *buy*-MO intensity exceeds
  the *sell*-MO intensity even on a net sell-heavy day — so imbalance predicts the
  composition of order flow, not just its level.
- **Price prediction.** Conditioning the next short-horizon midprice change on the
  prior regime, the distribution of price changes shifts with imbalance: bid-heavy
  regimes load probability on up-ticks. Comparative statics: as the autocorrelation
  lag grows, the imbalance-vs-price correlation decays toward zero, so the edge is
  short-horizon only.

**Source:** Cartea, Jaimungal & Penalva (2015) Ch.12 §12.2.1-12.2.3 pp.313-320

## Boundary Notes
The signal assumes the at-the-touch depth is observable and meaningful — it weakens
if quoted depth is dominated by fleeting/spoofed orders or if most liquidity hides
beyond the touch, since then resting volume misrepresents true intent. The Markov
specification assumes the regime process is (approximately) memoryless across the
chosen bins; richer history dependence would require a higher-order chain. The
predictive correlation is genuinely short-horizon: it is strong at small lags and
decays as the lag increases, so imbalance is an execution/market-making input, not a
multi-day forecast. It is computed from *limit-order* depth and so complements, but
is distinct from, signed-trade order-flow measures and the microprice; the microprice
is one way to fold imbalance directly into a fair-value estimate.

**Source:** Cartea, Jaimungal & Penalva (2015) Ch.12 §12.2 pp.313-314

## See Also
- [`mt-microprice-midprice-spread`](./mt-microprice-midprice-spread.md) -- microprice folds imbalance into a depth-weighted fair-value estimate
- [`mt-adverse-selection-short-term-alpha`](./mt-adverse-selection-short-term-alpha.md) -- imbalance is a source of the short-term-alpha that drives adverse selection
- [`mt-order-anticipators-front-running`](./mt-order-anticipators-front-running.md) -- imbalance-based signals are how anticipators detect and trade ahead of pressure

## Escalate to Raw When
The card sketches the MLE for the transition matrix and asserts the U-shaped arrival
intensity and the conditional price-change shifts; it does not reproduce the full
likelihood derivation, the joint MO-arrival/price-jump Markov-chain calibration, or
the optimal-liquidation-with-LOs problem of §12.4. Re-read Cartea, Jaimungal &
Penalva (2015) Ch.12 (esp. §12.2.1 for the likelihood, §12.2.2-12.2.3 for the joint
arrival/price-jump estimation, and §12.4 for the execution application) for the
worked model and proofs.
