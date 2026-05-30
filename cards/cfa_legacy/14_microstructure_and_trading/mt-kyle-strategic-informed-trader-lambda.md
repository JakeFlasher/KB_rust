---
schema_version: "cacg.v0"
id: "mt-kyle-strategic-informed-trader-lambda"
title: "The Kyle (1985) Model: Strategic Informed Trader, Noise Traders, and Camouflage"
reading_id: "14_microstructure_and_trading"
summary: "In Kyle (1985) a single risk-neutral informed trader hides linear-in-signal orders inside noise-trader flow while a risk-neutral market maker sets one batch-clearing price from aggregate net order flow alone."
tags: ["microstructure", "kyle-1985", "informed-trading", "batch-auction", "price-impact", "adverse-selection"]
citations:
  - source_id: "mt_ohara_1995_market_microstructure_theory_en"
    chunk_id: "mt_ohara_1995_market_microstructure_theory_en:p098:0128"
    chunk_hash: "35d138df27d4572db37e8c65e3b6946080558078a97568baeefe489c6d206995"
    page_range: [98, 99]
    quote: "that affects price behavior and not the size of any individual trade"
    edge_type: "defines"
  - source_id: "mt_foucault_pagano_roell_2013"
    chunk_id: "mt_foucault_pagano_roell_2013:p058:0081"
    chunk_hash: "7561c50ca31aacd1ae7cccfdef1c7892afe7d104b3177276b4ce70eb98ce6a68"
    page_range: [58, 58]
    quote: "can be seen as a measure of market depth (see Chapter 1) in that a lower"
    edge_type: "supports"
card_hash: "d61eaaaad49bd8623c4cb8e18be85dfc24a15e1c17fa72a9e16e78c432f00fc5"
---
# The Kyle (1985) Model: Strategic Informed Trader, Noise Traders, and Camouflage

## Intuition

Most early information models treat the informed trader as a price-taker who keeps submitting
orders until the price catches up to the truth. Kyle (1985) asks the sharper question: if there
is only *one* informed trader, he is an information *monopolist* and should not behave
competitively. He should choose his order size strategically, recognizing that a big order moves
the price against himself and burns his own informational rent. The whole model is built around
that single tension — trade more to capture more of the gap between the true value and the
price, but trade so much that the price reveals you and the gap collapses.

The market is organized as a single batch auction rather than a sequence of one-share trades.
Everyone submits quantities; the market maker never sees who is who. He observes only the
*aggregate net order flow* and must set one price that clears it. The informed trader is
camouflaged by "noise" (liquidity) traders whose demand is a random quantity with zero mean.
Because the informed trader cannot be distinguished from a lucky string of noise orders, he can
trade aggressively without fully giving himself away.

```
            informed order  x = beta*(v - p0)   \
                                                  >--- aggregate net flow y = x + mu
            noise order     mu ~ N(0, sigma_u^2) /
                                                       |
                                                       v
                              market maker sets ONE price
                                p = p0 + lambda*y = E[v | y]
                                                       |
                                                       v
                              informed profit  pi = (v - p)*x
```

The key emergent object is lambda, the slope of price in net order flow. A larger lambda means
each unit of imbalance moves the price more, i.e., a *less deep, more adverse-selection-prone*
market. More noise-trader variance lowers lambda and lets the informed trader hide better and
earn more. O'Hara stresses that learning here is fundamentally different from the
trade-by-trade sequential models: it is the *aggregate* quantity, not any individual trade size,
that moves the price.

**Source:** O'Hara (1995) §4.1, "The Strategic Behavior of an Informed Trader" pp.89-93.

## Definition

Setup (single-auction version):

- One risk-neutral informed trader who privately observes the ex-post liquidation value `v` of
  the asset, where `v ~ N(p0, Sigma_0)`. `p0` is the prior mean and `Sigma_0` the prior variance.
- Noise (liquidity) traders submit a net quantity `mu ~ N(0, sigma_u^2)`, independent of `v`, and
  do **not** behave strategically.
- One risk-neutral, competitive market maker who observes only the aggregate net order flow
  `y = x + mu`, not the components, and sets a single market-clearing price `p`.

Two equilibrium objects are conjectured and then verified:

- An informed order strategy `X(.)`: a map from the observed signal to a quantity.
- A market-maker pricing rule `P(.)`: a map from observed net flow to a clearing price.

Market efficiency (the market maker's zero-expected-profit / "regret-free" condition) requires
`P(x + mu) = E[v | x + mu]`. The informed trader's strategy must maximize his expected profit
`pi = (v - p) x` given that pricing rule. Crucially, the informed trader knows the *distribution*
of `mu` but not its *realization*, so he cannot condition his order on the realized noise flow —
a deliberate departure from rational-expectations models where agents condition on the
equilibrium price.

**Source:** O'Hara (1995) §4.1.1, "The Single-Auction Setting" pp.91-92.

## Mathematical Reasoning

O'Hara reconstructs Kyle's linear equilibrium. Conjecture that both sides are linear:

- Informed order:  `x = beta * (v - p0)`.
- Pricing rule:    `p = p0 + lambda * (x + mu)`.

Step 1 — Informed trader's problem. Taking lambda as given, the informed trader maximizes
expected profit conditional on his signal:

```
    max_x  E[ (v - p) x | v ]  =  (v - p0 - lambda*x) * x .
```

(The market maker's response to flow is `p = p0 + lambda*(x + mu)`, and `E[mu] = 0`.) This
objective is quadratic and concave in `x` (for `lambda > 0`), so the first-order condition
`v - p0 - 2*lambda*x = 0` gives the linear best response

```
    x = (1 / (2*lambda)) * (v - p0)   =>   beta = 1 / (2*lambda).
```

Step 2 — Market maker's problem. With the informed order linear in `v`, net flow
`y = beta*(v - p0) + mu` is jointly normal with `v`. Applying the conditional-expectation formula
for jointly normal variables, `E[v | y]` is linear in `y`, with slope equal to the covariance of
`v` and `y` divided by the variance of `y`:

```
    lambda = Cov(v, y) / Var(y)
           = (beta * Sigma_0) / (beta^2 * Sigma_0 + sigma_u^2).
```

Step 3 — Fixed point. Imposing consistency between Step 1 (`beta = 1/(2*lambda)`) and Step 2
yields the unique *linear* equilibrium

```
    beta   = sqrt( sigma_u^2 / Sigma_0 )            (trade intensity)
    lambda = (1/2) * sqrt( Sigma_0 / sigma_u^2 ).   (price impact)
```

Comparative statics (all stated by O'Hara, not numerically worked):

- Larger noise variance `sigma_u^2` raises `beta` (trade more) and *lowers* `lambda` (price
  moves less per unit of flow). More noise = better camouflage = larger informed profit.
- Larger signal variance `Sigma_0` *raises* `lambda`: more potential private information makes
  the market maker react more strongly to flow.
- Doubling noise volume leads the informed trader to double his order while leaving the
  *ex ante* price unchanged, because the offsetting moves in optimal `x` and `lambda` cancel.
- Posterior variance after the auction is exactly `Sigma_0 / 2`: independent of the realized
  flow, the informed trader's strategy reveals exactly half his information into the price.
- The reciprocal `1/lambda` is the natural measure of market *depth*: the order flow needed to
  move the price by one unit.

**Source:** O'Hara (1995) §4.1.1, equations (4.1)-(4.20) pp.92-99; Foucault, Pagano, and
Röell (2013) §2.3.2 p.58 (interpreting `1/lambda` as market depth).

## Boundary Notes

- **One informed trader is load-bearing.** The price-independence-from-volume result and the
  exactly-one-half information-revelation result both hinge on a *fixed* number of informed
  traders (here, one). With competing informed traders the strategic restraint erodes and more
  information leaks faster; that is the multiple-informed-trader extension, not this card.
- **Linearity is an equilibrium selection, not a theorem of uniqueness.** O'Hara notes there are
  no other *linear* equilibria, but nonlinear equilibria cannot be ruled out; linearity is
  retained for tractability and rests on joint normality of `v` and `mu`.
- **No bid-ask spread and no per-trade price.** Unlike the sequential Glosten-Milgrom model, the
  batch design clears everyone at one price, so it characterizes aggregate price impact rather
  than a quoted spread or the transaction price of an individual trade.
- **Linear pricing is only an approximation.** Because `p = p0 + lambda*y` is linear, a large
  enough negative net imbalance would imply a negative price — an artifact flagged by O'Hara as
  a limit of the linear rule.
- **Contrast with Glosten-Milgrom.** Glosten-Milgrom is sequential, quote-driven, and produces a
  spread from adverse selection trade-by-trade; Kyle is batch, quantity-driven, and produces a
  continuous price-impact slope lambda from aggregate adverse selection.

**Source:** O'Hara (1995) §4.1.1 pp.97-99 and §4.2 (multiple informed traders) p.106.

## See Also

- [`mt-kyle-lambda-market-depth-price-impact`](./mt-kyle-lambda-market-depth-price-impact.md) — unpacks lambda as price impact / inverse market depth and its empirical estimation.
- [`mt-glosten-milgrom-adverse-selection`](./mt-glosten-milgrom-adverse-selection.md) — the sequential, spread-generating sibling model this batch model is contrasted against.
- [`mt-multiple-informed-traders-competition`](./mt-multiple-informed-traders-competition.md) — relaxes the single-informed-trader assumption and shows faster information impounding.

## Escalate to Raw When

O'Hara derives the linear equilibrium via the conditional-distribution formula for jointly normal
variables and a Bayesian-updating argument (eqs. 4.6-4.20), but the OCR garbles every equation
line. To re-check the exact algebra for `beta`, `lambda`, the posterior mean/variance update, and
the half-information-revelation result, re-read O'Hara (1995) pp.92-99 (PDF pages 100-107) in the
clean publisher PDF. For the multiple-informed-trader and sequential/continuous-auction
extensions (Kyle's later sections, Back 1992, Holden-Subrahmanyam), see O'Hara pp.106-118.
