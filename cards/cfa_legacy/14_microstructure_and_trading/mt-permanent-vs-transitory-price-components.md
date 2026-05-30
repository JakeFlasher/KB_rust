---
schema_version: "cacg.v0"
id: "mt-permanent-vs-transitory-price-components"
title: "Permanent vs Transitory Price Components (Random-Walk + Pricing Error)"
reading_id: "14_microstructure_and_trading"
summary: "An observed transaction price decomposes into a permanent random-walk efficient price driven by information and a stationary transitory pricing error reflecting microstructure friction."
tags: ["microstructure", "random-walk", "pricing-error", "price-discovery", "efficient-price"]
citations:
  - source_id: "mt_hasbrouck_2007_empirical_market_microstructure"
    chunk_id: "mt_hasbrouck_2007_empirical_market_microstructure:p083:0104"
    chunk_hash: "9e34465cf79be6c759ac8aa1ef7b8ffd87ec5accfed40fab2fde63a2af0bdd92"
    page_range: [83, 83]
    quote: "random-walk decompositions are usually called permanent/transitory"
    edge_type: "defines"
---
# Permanent vs Transitory Price Components (Random-Walk + Pricing Error)

## Intuition

When you watch a stream of trade prints, two very different things are moving them. One is news: each genuine piece of information shifts the security's fundamental value, and that shift never reverses — it is *permanent*. The other is friction: a buyer who lifts the offer "overpays" relative to fundamental value, and the next seller who hits the bid "underpays", so the print bounces around the true value without going anywhere — this is *transitory*. Hasbrouck formalises this by writing the observed price `pt` as a permanent random-walk efficient price `mt` plus a stationary tracking error `st`.

```
   observed price pt
        |
        |   pt = mt + st
        v
   +---------------------+      +-----------------------+
   | mt  (PERMANENT)     |      | st  (TRANSITORY)      |
   | random walk;        |      | covariance-stationary |
   | absorbs information |      | pricing error;        |
   | never reverts       |      | mean-reverts to 0     |
   +---------------------+      +-----------------------+
        |                              |
   Var(increment) = sigma_w^2     Var(level) = sigma_s^2
   measures INFORMATION           measures EXECUTION FRICTION
```

The vocabulary is borrowed from the macroeconomics trend/cycle literature: there the random-walk piece is the long-term trend and the stationary piece is the transient business-cycle wobble. In microstructure the same algebra splits the price into the information-bearing efficient price and the noise injected by the trading mechanism. **Source:** Hasbrouck (2007) §8.5–8.6 pp.71–72 (PDF pp.83–84).

## Definition

Impose the minimal economic structure `pt = mt + st`, where the efficient price follows a random walk `mt = mt-1 + wt` with serially uncorrelated increments `wt`, and `st` is a zero-mean, covariance-stationary tracking error that may itself be serially correlated and partially or fully correlated with `wt`. Then:

- **Permanent component** `mt`: the (martingale) efficient price; its innovation variance per unit time is `sigma_w^2 = Var(wt)`.
- **Transitory component / pricing error** `st = pt - mt`: the gap between the transaction price and the efficient price; its (level) variance is `sigma_s^2 = Var(st)`.

The observable price-change series `Δpt = θ(L)εt` has a moving-average representation of arbitrary order; the decomposition asks what that representation reveals about the unobserved `mt` and `st`. **Source:** Hasbrouck (2007) §8.6 p.72 (PDF p.84).

## Mathematical Reasoning

Build the long-horizon price forecast `ft ≡ lim_{k→∞} E*[pt+k | pt, pt-1, ...]`. Because the increments of `ft` are a constant multiple `θ(1)` of the uncorrelated `εt`, they are themselves uncorrelated — a necessary (though not sufficient) condition for `ft` to be a martingale. Identifying `ft = mt` gives the identification-invariant result for the permanent component:

```
   wt = θ(1) εt           =>      sigma_w^2 = θ(1)^2 * sigma_eps^2
```

So the random-walk (permanent) variance is pinned down by the moving-average representation alone, independent of which structural story generated the data.

The transitory part is only bounded, not point-identified. Decompose `st = (pt - ft) - (mt - ft)`. Since `ft` is the linear projection of `mt` on the price history, the filtering error `(mt - ft)` is orthogonal to `(pt - ft)`, so variances add:

```
   sigma_s^2 = Var(pt - ft) + Var(mt - ft)
                 \_____known_____/   \____>= 0____/
```

The first term is computable from the reduced form; the second is non-negative, giving a **lower bound** `sigma_s^2 >= Var(pt - ft)`, attained when all information is trade-related. There is **no upper bound**: observationally equivalent models with a more "stale" price (e.g. `pt = mt-2 + cqt`) inflate `sigma_s^2` arbitrarily, so any ceiling must come from economics, not statistics.

Comparative reading: a higher `sigma_w^2` means more information is being impounded per unit time (active price discovery); a higher `sigma_s^2` means trades are landing further from fundamental value (costlier execution / noisier mechanism). The two variances answer orthogonal questions. **Source:** Hasbrouck (2007) §8.5–8.6 pp.71–72 (PDF pp.83–84).

## Boundary Notes

- **Time dimension differs.** `sigma_w^2` is a variance *per unit time* (rescale across intervals to get hourly/daily figures); `sigma_s^2` is the variance of a difference of two level variables *at a point in time* and carries no time dimension. Do not annualise the pricing-error variance.
- **Identification asymmetry.** The permanent variance is invariant to structural identification; the transitory variance is unidentified and only lower-bounded. Reported `sigma_s^2` figures are floor estimates unless an economic staleness assumption is added.
- **When it breaks.** The random-walk description is a model of *fundamental value*, not a complete description of all securities: instruments with finite maturity (bonds, swaps, options) converge to known terminal values, so the random walk holds only over short samples, not the long run.
- **Contrast with structural Roll model.** The generalized Roll model is a specific order-one case; this univariate decomposition generalises it to arbitrary MA order, keeping the same two substantive results (`sigma_w^2` identified, `sigma_s^2` lower-bounded). **Source:** Hasbrouck (2007) §8.5–8.6 pp.71–72 (PDF pp.83–84).

## See Also

- [`mt-random-walk-efficient-price`](./mt-random-walk-efficient-price.md) -- defines the martingale efficient price `mt` that is the permanent component here.
- [`mt-generalized-roll-spread-decomposition`](./mt-generalized-roll-spread-decomposition.md) -- the order-one structural special case from which this decomposition generalises.
- [`mt-information-shares-price-discovery`](./mt-information-shares-price-discovery.md) -- multivariate extension attributing the permanent-component variance across venues.
- [`mt-hasbrouck-var-trades-quotes`](./mt-hasbrouck-var-trades-quotes.md) -- VAR machinery used to estimate the moving-average representation behind `sigma_w^2`.

## Escalate to Raw When

This card sketches the orthogonality argument and the variance-addition step but does not reproduce the full derivation of the lower bound (eq. 8.7) or the moving-average coefficient algebra (eqs. 8.8–8.13, `Ci` and `Ak` in terms of the `θj`). Re-read Hasbrouck §8.5–8.6 (PDF pp.83–86) and the chapter appendix for the formal projection-theory development, and exercise 8.3 for the rescaling of the random-walk variance across sampling intervals.
