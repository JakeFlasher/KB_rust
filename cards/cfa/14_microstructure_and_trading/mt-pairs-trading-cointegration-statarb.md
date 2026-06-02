---
schema_version: "cacg.v0"
id: "mt-pairs-trading-cointegration-statarb"
title: "Pairs Trading and Statistical Arbitrage via Cointegration with Optimal Bands"
reading_id: "14_microstructure_and_trading"
summary: "StatArb trades a mean-reverting cointegration factor of co-moving assets, buying when the spread is cheap and selling when dear; optimal entry/exit bands solve an optimal-stopping problem, and the strategy bets on typical price behaviour rather than being a true riskless arbitrage."
tags: ["microstructure", "statistical-arbitrage", "pairs-trading", "cointegration", "mean-reversion", "optimal-stopping"]
citations:
  - source_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading"
    chunk_id: "mt_cartea_jaimungal_penalva_2015_algorithmic_hf_trading:p290:0375"
    chunk_hash: "a4270e576d91d01603793f20d8994054060c7e50e47f8102bd411125c7acd89d"
    page_range: [290, 291]
    quote: "pairs trading fall under the class of strategies sometimes labeled as"
    edge_type: "defines"
card_hash: "218a37543f68e388aa4a6ffb52eeda343d9073741919ce8925f4244787f1595e"
---
# Pairs Trading and Statistical Arbitrage via Cointegration with Optimal Bands

## Intuition
Predicting where one stock goes next is hard; predicting the *joint* behaviour of two
co-moving stocks is easier. Pairs trading exploits this by trading not a single name but
a *portfolio* — a fixed long/short linear combination of two (or more) assets whose
relative price is stable even when each leg wanders. When two names share a common
permanent driver, a well-chosen weighting cancels that common trend and leaves a residual
"spread" that wiggles around a fixed level and keeps coming back. You buy the spread when
it is cheap and sell it when it is dear, harvesting the reversion.

```
 INTC, SMH midprices            cointegration factor  C_t = A·S_INTC + B·S_SMH
 (move together)                (mean-reverts to ~0)

  price                          C_t
   ^   INTC  __                   ^ - - - - - - - - upper band  (SELL the spread here)
   |      \ /  \                  |      /\      /\
   |  SMH  X    \__               |  ___/  \____/  \___  <- mean-reverting level (0)
   |      / \                     | \/                \
   |__________________> t         | - - - - - - - - lower band  (BUY the spread here)
                                   |__________________________> t
```

This is what practitioners call *statistical arbitrage* (StatArb). The word "arbitrage" is
aspirational, not literal: the trade bets on the typical, historically observed behaviour
of prices. If the spread keeps widening instead of reverting, the position loses money.

**Source:** Cartea, Jaimungal & Penalva (2015) §11.1 pp.273-274

## Definition
Let the (mid)price vector `S_t = (S_t^1, S_t^2)'` of two co-moving assets follow a
mean-reverting (Ornstein-Uhlenbeck-type) vector process with a transitory component plus a
permanent Brownian component, `dS_t = K(theta - S_t) dt + a dW_t`, with `Sigma = a a'`. A
**cointegration factor** is the scalar portfolio `C_t = A·S_t^1 + B·S_t^2` whose weights
`(A, B)` are chosen (estimated from data) so that `C_t` is dominated by the mean-reverting
component and hovers around a fixed long-run level (normalised to zero). A **pairs-trading
strategy** goes long the portfolio when `C_t` is below its mean-reverting level ("cheap")
and short when above it ("dear"), closing the position as `C_t` returns toward the level.

A **true arbitrage** is a strategy that earns returns above the risk-free rate with zero
risk; StatArb is explicitly *not* one — it bets off the typical behaviour of asset prices
and is therefore not risk-free.

**Source:** Cartea, Jaimungal & Penalva (2015) §3.7, §11.1 pp.273-274 (with model setup at p.59)

## Mathematical Reasoning
**Why a linear combination beats a single name.** Writing the vector dynamics
`dS_t = K(theta - S_t) dt + a dW_t` with a generic mean-reversion matrix `K`, one
diagonalises `K` so the transformed components decouple. The component whose eigenvalue has
the largest absolute mean-reversion rate `max{|kappa_1|, |kappa_2|}` carries the strongest
mean-reverting (hence most predictable) signal — that transformed component is the
cointegration factor. Isolating it concentrates the predictable part of the joint dynamics
into one tradable scalar.

**Ad hoc bands.** A naive rule places entry bands one standard deviation above/below the
mean-reverting level and an exit band within a small interval (e.g. 1/10 standard
deviation) of that level: buy one unit when the lower band is hit, sell when the upper band
is hit, unwind near the mean. Profits accrue as long as `C_t` keeps oscillating around and
reverting to the level.

**Optimal bands as optimal stopping.** The bands need not be ad hoc. Model the
cointegration factor as a single mean-reverting OU process
`dC_t = kappa(theta - C_t) dt + sigma dW_t`,
where `kappa` is the reversion speed, `theta` the reversion level, and `sigma` the
volatility. The trader makes one round trip with no terminal horizon. The exit value of a
long position is the optimal-stopping value
`H^+(C) = sup_tau E[ e^{-rho(tau-t)} (C_tau - c) ]`,
with transaction cost `c` and discount/urgency parameter `rho > 0` (larger `rho` pushes the
exit boundary in toward the long-run level). The entry problem nests this: entering long
pays `C + c` to acquire the exit option worth `H^+`. By the dynamic programming principle
the value functions solve coupled variational inequalities
`max{ (L - rho) H^+(C), (C - c) - H^+(C) } = 0`,
`max{ (L - rho) G^+(C), (H^+(C) - C - c) - G^+(C) } = 0`,
where `L = kappa(theta - C) d/dC + (1/2) sigma^2 d^2/dC^2` is the OU generator. The exit VI
is structurally a perpetual-call problem: solve `(L - rho) F(C) = 0` for fundamental
solutions `F_{±}`, write `H^+ = A F_+ + B F_-` on the continuation region and `H^+ = C - c`
on the exercise region, and pin the free boundary `C^*` with value-matching and
smooth-pasting conditions.

**Why P&L clusters at band multiples.** Because the trader enters at a band and exits near
the mean, realised P&L per cycle is roughly the band size; over a horizon the P&L
distribution concentrates near integer multiples of the band size, the weight on each
multiple being the probability of that many round trips. The smear off the multiples comes
from horizon-end positions that must be closed before reversion completes — possibly at a
loss.

**Source:** Cartea, Jaimungal & Penalva (2015) §3.7, §11.2-§11.3 pp.273-278

## Boundary Notes
- **Assumptions.** Cointegration must be *persistent*: the weights `(A, B)` and the
  mean-reverting level are estimated from data (e.g. day-by-day) and assumed stable over the
  trading horizon. If the common factor structure breaks (regime shift, structural break),
  the residual no longer reverts and the trade has no edge.
- **Not riskless.** The strategy is explicitly *not* a true arbitrage — it can lose if the
  spread diverges. The optimal-stopping formulation has no terminal horizon, but in practice
  a finite horizon forces possibly-loss-making liquidations when the factor has not yet
  reverted.
- **Ad hoc vs optimal bands.** §11.2 bands are heuristic (fixed standard-deviation
  multiples); §11.3 bands are derived from the OU generator and an urgency parameter `rho`,
  trading off reversion speed `kappa`, volatility `sigma`, and transaction cost `c`.
- **Scope.** This card covers the price-level cointegration factor and its single-round-trip
  optimal bands. The drift-cointegration variant across a collection of assets (§11.4) and
  the short-term-alpha extension are out of scope here.

**Source:** Cartea, Jaimungal & Penalva (2015) §11.1-§11.3 pp.273-278

## See Also
- [`mt-value-traders-arbitrageurs`](./mt-value-traders-arbitrageurs.md) -- StatArb is the quantitative arbitrageur's strategy; this card supplies the cointegration mechanics behind that trader type.
- [`mt-information-shares-price-discovery`](./mt-information-shares-price-discovery.md) -- the cointegrating relationship and common permanent component connect directly to price-discovery / common-factor decompositions.
- [`mt-order-imbalance-signal`](./mt-order-imbalance-signal.md) -- order-imbalance signals are complementary short-horizon predictors that can refine StatArb entry timing.

## Escalate to Raw When
Re-read Cartea, Jaimungal & Penalva (2015) Ch.11 (and §3.7 for the estimation) when you
need: the explicit closed forms for the OU fundamental solutions `F_{±}` and the
smooth-pasting equations that locate the optimal boundaries `C^*`/`E^*` (pp.277-278+); the
derivation of the entry value function `G^+` and its coupling to `H^+`; the §11.4
drift-cointegration multi-asset algorithm and short-term-alpha model; and the empirical
estimation of the cointegration weights `(A, B)` from the INTC/SMH discrete fit. This card
sketches the variational-inequality structure but does not reproduce the boundary-solving
algebra the source carries out.
