---
schema_version: "cacg.v0"
id: "fa-liquidity-adjusted-var"
title: "Liquidity-Adjusted VaR: Splitting Out the Liquidation Cost"
reading_id: "22_fund_level_arbitrage"
summary: "Traditional VaR prices only return risk and assumes you exit at the midprice. LVaR adds a liquidation-cost term built from the bid-ask spread, then exposes a feedback loop: rising volatility lifts measured VaR, forcing risk-limit selling that depresses prices and raises VaR again."
tags: ["liquidity-adjusted-var", "liquidation-cost", "var-multiplier"]
citations:
  - source_id: "fa_vandermerwe_2015_market_liquidity_risk"
    chunk_id: "fa_vandermerwe_2015_market_liquidity_risk:p163:0177"
    chunk_hash: "77c9bb4e9723f8debb3c0d629e4f6db5222053cb3c3da51d287a76aa5012887c"
    page_range: [163, 163]
    quote: "The concept behind the liquidity-adjusted VaR is simple."
    edge_type: "defines"
---
# Liquidity-Adjusted VaR: Splitting Out the Liquidation Cost

## Intuition
Plain VaR quietly assumes you can unwind your book at the quoted midprice. That is fine for a tiny position in a deep name, but an arbitrageur forced to liquidate pays the spread (and more, if the order exceeds quoted depth) on the way out. Liquidity-adjusted VaR (LVaR) repairs this by splitting market risk into two pieces: a *return* component (the pure price-move risk plain VaR already captures) and a *liquidity* component (the cost of actually getting out). The crude field workaround — inflate the volatility or stretch the holding period to ten days — buys an orderly-liquidation cushion, but LVaR makes the liquidation cost explicit rather than smuggling it into a fudged sigma.

The deeper warning is a feedback loop. Because the liquidation term grows when spreads widen, and because spreads widen exactly when volatility spikes, LVaR rises sharply in stress. If every desk is held to the same LVaR limit, a vol shock lifts everyone's measured risk at once, forcing simultaneous selling that pushes prices down and spreads out — which raises LVaR again. Risk measurement becomes its own destabilizer.

```
 vol up --> measured (L)VaR up --> breaches risk limit
   ^                                      |
   |                                      v
 spreads widen <-- prices fall <-- forced liquidation
   (the VaR multiplier / fire-sale feedback)
```

**Source:** van der Merwe (2015) pp.163-166.

## Definition
- **Return (market) risk component** — pure price-move risk, e.g. interest-rate risk in a fixed-income book; this is what traditional VaR already measures, assuming exit at the midprice. The text: "Market risk is split into two components: the return risk, which can be thought of as a pure market risk component ... and a liquidity risk."
- **Liquidity-adjusted VaR (LVaR)** — "regular VaR plus the cost of liquidating positions"; it "extends the traditional value at risk calculation to incorporate the cost of liquidity."
- **Exogenous vs endogenous liquidity risk** — exogenous liquidity risk is set by the collective behavior of all participants and is outside any one trader's control; endogenous liquidity risk is specific to the size of a participant's own position. When order size is below quoted depth the immediate-execution cost is half the bid-ask spread; the excess over the half-spread when size exceeds depth is the endogenous cost. The implemented framework here incorporates only the (data-friendly) exogenous component.
- **VaR multiplier effect** — Pedersen's result that "subjecting traders to liquidity-adjusted value at risk gives rise to a multiplier effect," a feedback between market liquidity and risk management.

**Source:** van der Merwe (2015) pp.163-166.

## Mathematical Reasoning
Let `P_t` be the price, `sigma_t` the one-day return volatility (zero mean assumed), and use the 99% normal quantile. Traditional VaR is

```
VaR_t = P_t * (1 - e^{-2.33 * sigma_t}).
```

The liquidation cost is built from the relative spread `s = (Offer - Bid)/Mid`, its volatility `sigma_s`, and a scaling factor `k`:

```
Liquidation Cost = P_t * (s + k*sigma_s) / 2.
```

Assuming (conservatively) that extreme return moves and extreme spread moves occur concurrently, the two terms simply add:

```
Liquidity VaR_t = P_t * (1 - e^{-2.33*sigma_t}) + P_t * (s + k*sigma_s)/2.
```

Comparative statics: `Liquidity VaR_t >= VaR_t` always, since the liquidation term is non-negative; the gap widens with `s`, with `sigma_s`, and with the stress factor `k`. The factor `k` distinguishes normal from stressed regimes — `k = 1` corresponds to a normal distribution and `k > 1` to a fat-tailed, stressed market. Because `sigma_t` and the spread terms co-move and feed risk limits, an exogenous rise in `sigma_t` raises `Liquidity VaR_t`, which (under a binding limit) triggers liquidation that raises `s` and `sigma_s`, raising `Liquidity VaR_t` further — the self-reinforcing multiplier. Portfolio extension swaps the single `s` for a weighted-average portfolio spread series rather than estimating a full spread covariance matrix, since spread distributions are not well-behaved.

**Source:** van der Merwe (2015) pp.164-166.

## Boundary Notes
The LVaR construction adopts explicit simplifying assumptions: zero-mean normal returns at the 99% (2.33-sigma) quantile; the conservative assumption that extreme return moves and extreme spread moves occur concurrently (so the two terms simply add); and modeling only the exogenous (spread) liquidity component, dropping the endogenous size-greater-than-depth cost. Flag these scope limits when transplanting the formula to fat-tailed, non-Gaussian, or depth-constrained books.

**Source:** van der Merwe (2015) pp.163-166.

## See Also
- [`fa-funding-spirals-and-fire-sales`](./fa-funding-spirals-and-fire-sales.md) — the forced-liquidation downward spiral that the VaR multiplier feeds into when limits bind across many desks at once.
- [`fa-market-impact-transaction-costs-and-turbulence-breakdown`](./fa-market-impact-transaction-costs-and-turbulence-breakdown.md) — the trade-size-vs-depth and turbulence mechanics underlying the endogenous liquidation cost.
- [`fa-liquidity-measurement-and-price-impact`](./fa-liquidity-measurement-and-price-impact.md) — spread and Roll-measure inputs that supply `s` and `sigma_s`.
- `mt-funding-liquidity-fire-sales` (reading 14) carries the fire-sale feedback this card's VaR-multiplier loop shares; reading 14 has no LVaR card, so this card is the sole owner of the liquidity-adjusted-VaR construction.

Legacy cross-references (other tree, prose only): the value-at-risk notes card rm-value-at-risk-notes gives the baseline VaR mechanics and confidence-quantile convention this card extends, and the loss-distribution-anatomy card rm-loss-distribution-anatomy frames why the return distribution and the spread distribution must be modeled jointly under stress.

## Escalate to Raw When
Go to the raw text when you need the worked single-security and portfolio LVaR computations, the choice of historical-simulation vs parametric vs Monte Carlo for the spread distribution (including the multimodal, regime-shifting "worst-case" spread scenario), the Roll-measure substitution when daily spreads are unavailable, or the Basel Committee and Brigo-Nordio limitation discussion on stochastic holding periods and the credit/market-liquidity dependence. The Pedersen 1998-Russian-default anecdote that motivates the multiplier feedback also lives there as a concrete narrative.

**Source:** van der Merwe (2015) pp.165-166.
