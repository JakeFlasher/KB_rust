---
schema_version: "cacg.v0"
id: "mt-kyle-lambda-market-depth-price-impact"
title: "Kyle's Lambda: Linear Price Impact and Market Depth (1/lambda)"
reading_id: "14_microstructure_and_trading"
summary: "In Kyle's batch-auction equilibrium the market maker sets price linearly in net order flow, p = p0 + lambda*(x+u); lambda is the price-impact slope and its reciprocal 1/lambda measures market depth."
tags: ["microstructure", "kyle-lambda", "price-impact", "market-depth", "informed-trading", "liquidity"]
citations:
  - source_id: "mt_ohara_1995_market_microstructure_theory_en"
    chunk_id: "mt_ohara_1995_market_microstructure_theory_en:p103:0134"
    chunk_hash: "4649c2355c650cb165ba9d6ffc0e49fa8b0072eb51c46a5879cbfc582191f74c"
    page_range: [104, 104]
    quote: "Hence the market maker's price will, in fact, be linear in the order flow"
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p058:0081"
    chunk_hash: "7561c50ca31aacd1ae7cccfdef1c7892afe7d104b3177276b4ce70eb98ce6a68"
    page_range: [58, 58]
    quote: "reciprocal of λ can be seen as a measure of market depth"
    edge_type: "supports"
card_hash: "9eab948ab3e4b7fbe11451377ccbb445669bb419eef57eab7e5e6e58d1484310"
---
# Kyle's Lambda: Linear Price Impact and Market Depth (1/lambda)

## Intuition

In Kyle's (1985) single-auction model a single risk-neutral informed trader who knows the asset's true liquidation value submits a market order alongside a stream of random "noise" orders. A competitive market maker never sees the orders individually; he observes only the *aggregate net order flow* and must set one market-clearing price. Because some of that flow is informed, the market maker rationally treats heavier net buying as a signal that value is higher, and heavier net selling as a signal it is lower. The remarkable equilibrium result is that the price he sets rises in a *straight line* with net order flow.

The slope of that line is Kyle's lambda. It is the single number that says how many cents the price moves per unit of net buying pressure — the market maker's defensive markup against the risk that the order flow is information-driven. A large lambda means even modest imbalances jerk the price around; a small lambda means the market can absorb large orders with little price disturbance. Its reciprocal, 1/lambda, is therefore the natural definition of **market depth**: the volume of order flow that can be accommodated before the price moves by one unit.

```
        price p
          |                          p = p0 + lambda*(x+u)
          |                       . /
          |                    .  /   slope = lambda
       p0 +----------------.----/        (steeper => more price impact,
          |             .  |   /           thinner market)
          |          .     |  /
          |       .        | /
          +----.-----------+/------------------ net order flow (x+u)
                            0
          depth  =  1/lambda  =  order flow that moves price one unit
```

**Source:** O'Hara (1995) ch.4 §4.1.1 pp.99-105

## Definition

Setup. The asset's liquidation value v is normal with prior mean p0 and variance Sigma0. Noise (uninformed) traders submit an aggregate quantity u that is normal with mean 0 and variance sigma_u^2, independent of v. The informed trader observes v and chooses his order x; he knows the *distribution* of u but not its realization, so he cannot condition his order on actual noise flow. The market maker observes only the aggregate net flow x + u and sets a single price p to clear the batch.

Equilibrium objects.
- **Pricing rule:** p = P(x + u), a function the market maker commits to.
- **Market efficiency (zero expected profit):** P(x + u) = E[v | x + u], so the cleared price equals the conditional expectation of value given total order flow.
- **Informed order strategy:** X(v), chosen to maximize expected profit pi = (v - p)x given the conjectured pricing rule.
- **Lambda:** the price-impact coefficient in the linear pricing rule.
- **Market depth:** 1/lambda, "how much order flow affects price adjustment" inverted into how much flow can be absorbed.

**Source:** O'Hara (1995) ch.4 §4.1.1 pp.91-93

## Mathematical Reasoning

O'Hara shows the model has a linear equilibrium of the form

    X(v) = beta * (v - p0)        (informed trade scales with mispricing)
    P(x + u) = p0 + lambda * (x + u)   (price linear in net flow)

The two slopes are pinned down jointly by two requirements. (1) *Optimality:* taking the linear pricing rule as given, the informed trader's profit (v - p)x is quadratic in x, so the optimum is linear in v, fixing beta. (2) *Market efficiency:* applying the conditional-expectation formula for jointly normal variables to v and the flow y = x + u, the posterior mean E[v | y] is itself linear in y, which forces the pricing rule to be linear and fixes lambda. Self-consistency requires that the beta the market maker infers equals the beta the informed trader actually uses (rational expectations) — and the linear conjecture is the unique linear fixed point.

The comparative statics are the load-bearing content:

- **Depth rises with noise.** Roughly, lambda scales like sqrt(Sigma0)/sigma_u, so 1/lambda (depth) increases with noise-trading variance and decreases with the prior value uncertainty Sigma0. More noise lets the informed trader hide, lowering the per-unit signal content of flow and flattening the price-impact line.
- **Order-flow neutrality of price volatility.** Doubling sigma_u doubles the informed trader's optimal order (he keeps his relative share constant) and *also* halves lambda; the two effects offset, so the ex-ante price distribution is independent of trading volume. This neutrality is special to the single-informed-trader case.
- **Deterministic information revelation.** The posterior variance after one auction is exactly Sigma0/2, regardless of the realized trade — half the private information is impounded into price, the rest withheld by the informed trader's restraint. Prices follow a martingale: the expected posterior mean equals p0.

Interpretation of the slope: "large volume results in a worse price, but not an increasingly worse price." A linear rule means impact does not snap instantly to full-information value the way a competitive (lambda -> 0 limit fully revealing) market would. With multiple informed traders (Holden-Subrahmanyam extension) lambda is no longer held constant — it is larger early and falls rapidly as competition reveals information faster, and depth 1/lambda diverges in the continuous limit.

**Source:** O'Hara (1995) ch.4 §4.1.1 pp.93-99; §4.1.x pp.109-112

## Boundary Notes

The linear equilibrium leans hard on **joint normality** of v and u; relaxing it (Foster-Viswanathan, elliptical distributions) destroys the clean deterministic-variance and constant-lambda properties. Two further caveats O'Hara flags: (i) a linear rule mechanically implies negative prices for a large enough net-sell imbalance, so it is at best a local approximation to real price formation; and (ii) uniqueness holds only among *linear* equilibria — nonlinear equilibria may exist but are analytically intractable.

Contrast with sibling models. Unlike the **Glosten-Milgrom** sequential-trade model, where each trade is observed individually and the spread is set order-by-order, Kyle batches all orders so it is the *aggregate net quantity* that moves price; lambda is a continuous depth/impact slope rather than a discrete bid-ask spread. Unlike **inventory** models, Kyle's market maker is competitive and earns zero expected profit — price impact is pure adverse selection, not inventory-holding cost. The single-trader neutrality result (volume does not affect price volatility) breaks once the number of informed traders is endogenous or greater than one.

**Source:** O'Hara (1995) ch.4 §4.1.1 pp.97-100, pp.108-112

## See Also

- [`mt-kyle-strategic-informed-trader-lambda`](./mt-kyle-strategic-informed-trader-lambda.md) -- the informed trader's strategic optimization that pins down beta and feeds lambda.
- [`mt-temporary-permanent-price-impact`](./mt-temporary-permanent-price-impact.md) -- decomposes the price move lambda generates into permanent (information) vs transitory components.
- [`mt-price-impact-measures-amihud`](./mt-price-impact-measures-amihud.md) -- empirical estimation of lambda (Amihud illiquidity, regression of price change on order imbalance).
- [`mt-market-impact-price-concession`](./mt-market-impact-price-concession.md) -- practitioner-side execution view of the same depth/impact trade-off.
- [`fa-liquidity-measurement-and-price-impact`](../22_fund_level_arbitrage/fa-liquidity-measurement-and-price-impact.md) — cross-set: Roll / Kyle / Amihud price-impact and implied-spread estimators (reading-14 primary derivations; reading-22 ETF liquidity-measurement application).
## Escalate to Raw When

O'Hara derives lambda, beta, and the Sigma1 = Sigma0/2 variance result from the conditional-normal projection formula (eqs. 4.1-4.20); the OCR garbles every equation line and the Bayesian-update algebra. Re-read O'Hara (1995) pp.99-107 for the exact closed-form solution and the step-by-step projection derivation, and pp.108-112 for the Holden-Subrahmanyam multiple-informed-trader limit results (lambda and depth dynamics) that this card only sketches.
