---
schema_version: "cacg.v0"
id: "pm-anomalies-and-cross-sectional-pricing"
title: "Anomalies and Cross-Sectional Pricing"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Anomalies and Cross-Sectional Pricing: cataloging the empirical cross-sectional anomalies (value, momentum, quality, low-volatility, size) at intuition level — what each anomaly captures, why it persists in equilibrium under the efficiently-inefficient framing, and how the anomalies fit into the multifactor pricing structure"
tags: ["portfolio-management", "anomalies", "cross-sectional"]
citations:
  - source_id: "pm_pedersen_2015_efficiently_inefficient"
    chunk_id: "pm_pedersen_2015_efficiently_inefficient:p156:0205"
    chunk_hash: "2c89310c15d929ee677b0ab90fdbb9902f9f782218630f35ad46219d11a93dd1"
    page_range: [156, 157]
    quote: "Fundamental quants trade on factors such as value, momentum, quality, size, and low risk."
    edge_type: "defines"
  - source_id: "pm_cochrane_2005_asset_pricing_revised"
    chunk_id: "pm_cochrane_2005_asset_pricing_revised:p167:0197"
    chunk_hash: "00811a8ca8754959fe9d90828803106cc237aa9c222f78eece5d95eb19df89b4"
    page_range: [167, 168]
    quote: "Linear factor pricing models are the most popular models of this sort in finance."
    edge_type: "supports"
card_hash: "476833b219ace90ef4fde6851507b396d664c26a4a724cad6464e13ce4ae61d3"
---
# Anomalies and Cross-Sectional Pricing

## Intuition

The cross-sectional anomaly literature documents systematic
average-return differences across stocks sorted on observable
characteristics. Five anomalies receive the most attention: value
(high book-to-market beats low), momentum (past winners beat past
losers over a medium-term horizon), quality (high-profitability,
low-leverage firms beat their opposites), low-volatility (low-vol
stocks beat high-vol stocks per unit risk), and size (small-cap
beats large-cap, with significant variation across periods). The
multifactor pricing extension in the prior sibling card frames
each as a priced risk factor; the Pedersen efficiently-inefficient
framing in another sibling explains why each persists in
equilibrium. This card surveys the five at intuition level.
**Source:** Pedersen (2015) pp.87-164.

```
        major cross-sectional anomalies (intuition level)
        ==================================================

        anomaly    | sort variable                | pattern
        -----------+-------------------------------+----------------
        value      | book-to-market (B/M)          | high B/M wins
        momentum   | medium-term past return        | past winners win
        quality    | profitability + leverage      | high quality wins
        low-vol    | trailing volatility            | low-vol wins
                   |                                | per unit risk
        size       | market capitalization          | small-cap wins
                   |                                | (period-dependent)

        each pattern survives standard CAPM-only risk adjustment;
        each is captured in the multifactor pricing structure as
        a separate factor with its own price of risk lambda_j.
```

The anomalies are persistent — documented over decades and across
markets — but not infinitely exploitable. Pedersen's framework
explains the persistence: arbitrage capital is bounded; trades
to exploit each anomaly carry their own risk premium for the
liquidity provider; equilibrium spreads sit where capital and
demand balance. The anomalies are simultaneously evidence that
the single-factor CAPM is misspecified and evidence that priced
factor structure exists at the cross-sectional level. **Source:**
Pedersen (2015) pp.87-164.

## Definition

A cross-sectional anomaly is a documented relationship between a
stock characteristic and the stock's average excess return that
is not predicted by the CAPM. **Source:** Pedersen (2015)
pp.87-164.

```
anomaly_j: average return on portfolio sorted high on characteristic_j
          minus average return on portfolio sorted low on characteristic_j
          ≠ 0 in long-run data,
          and not explained by the CAPM-implied expected return
          based on each portfolio's market beta
```

The five major anomalies have distinct sort variables and
empirical patterns. **Source:** Pedersen (2015) pp.87-164.

The value anomaly sorts stocks by book-to-market ratio (or
related fundamentals-to-price metrics: P/E, P/CF, dividend yield)
and finds that high-B/M ("value") stocks earn higher average
returns than low-B/M ("growth") stocks beyond what the market
beta implies. **Source:** Pedersen (2015) pp.133-158.

The momentum anomaly sorts stocks by trailing return over a
medium-term formation window and finds that past winners
outperform past losers over a comparable subsequent window. The
window choice matters — a short-term reversal pattern and a
long-term reversal pattern coexist with the medium-term momentum
pattern, and the formation-and-holding-window choice picks which
of the three the strategy captures. **Source:** Pedersen (2015)
pp.133-158.

The quality anomaly sorts stocks by combined profitability and
financial-strength signals and finds that high-quality firms
outperform low-quality firms beyond what factor exposure to size,
value, or market would predict. The "quality" composite varies
across implementations: gross profitability, return on equity,
asset growth, leverage. **Source:** Pedersen (2015) pp.133-158.

The low-volatility anomaly sorts stocks by trailing return
volatility and finds that low-vol stocks earn higher Sharpe
ratios than high-vol stocks — a violation of the CAPM prediction
that high-beta (and therefore high-vol) stocks earn higher
returns proportional to their risk. **Source:** Pedersen (2015)
pp.133-158.

The size anomaly sorts stocks by market capitalization and finds
small-caps outperform large-caps, with significant time variation
in the spread. The size effect was strongest in early decades of
documented data and has weakened more recently, prompting active
debate about whether the original size factor reflects a stable
risk premium. **Source:** Pedersen (2015) pp.133-158.

## Mathematical Reasoning

Each anomaly admits the same algebraic treatment under the
multifactor pricing structure. The factor-mimicking portfolio for
anomaly `j` is a long-short construction that goes long the high-
characteristic side of the cross-section and short the low-
characteristic side. The specific quantile breakpoints, weighting
scheme, and rebalancing cadence are equity-implementation choices
deferred to future-05. **Source:** Pedersen (2015) pp.133-158.

```
F_j(time)  =  R_(high-characteristic_j)(time)
           - R_(low-characteristic_j)(time)

E[F_j]  =  realized factor risk premium  =  lambda_j (under model fit)
```

The factor risk premium `lambda_j` is the time-series average of
the long-short portfolio's return. Empirically, each of the five
anomalies has produced a positive `lambda_j` over multi-decade
samples in U.S. equity markets, with international samples
showing similar (often weaker) patterns. **Source:** Pedersen
(2015) pp.133-158.

The Fama-French 3-factor model uses market, size, and value as
its three factors. The Carhart 4-factor model extends FF3 by
adding momentum. Each is a specific specification of the
multifactor pricing structure with the corresponding factors.
**Source:** Cochrane (2005) pp.149-183.

The post-Cochrane Fama-French 5-factor model adds profitability
and investment factors to the FF3 market / size / value triple,
omitting momentum (the Carhart extension is the alternative that
retains momentum). The five factors capture the value, size,
quality (via profitability and investment), and market premia
without a separate momentum factor. **Source:** Pedersen (2015)
pp.133-158.

The interpretation of `lambda_j` divides the asset-pricing
literature. Two camps exist. **Source:** Pedersen (2015)
pp.87-164.

```
camp A (rational risk-based):
   each anomaly factor captures a priced macro / state-variable
   risk; the factor return is compensation for bearing that risk;
   lambda_j > 0 reflects the fundamental risk premium

   examples: value as recession risk; quality as bankruptcy risk;
            momentum as macro-state-aligned risk

camp B (behavioral / limits-to-arbitrage):
   each anomaly reflects systematic mispricing driven by investor
   biases, persisting because arbitrage capital is bounded;
   lambda_j > 0 reflects the cost of arbitrage rather than risk

   examples: value as overreaction-correction; momentum as under-
            reaction; low-vol as lottery-preference distortion
```

The Pedersen efficiently-inefficient framing is sympathetic to
both camps and points out that the empirical implications are
similar: factor-based portfolios capture the spread between
mispriced and fairly-priced (or risky and less-risky) securities.
The investor who loads on the factors earns the spread either as
risk premium or as arbitrage compensation; the framework does
not require resolving which interpretation is correct.
**Source:** Pedersen (2015) pp.87-164.

A specific implication for portfolio construction: explicit
factor exposures replace the binary active-vs-passive choice
with a continuum of factor-weighted portfolios. A "smart-beta"
portfolio targets known factor premia at low cost via systematic
construction, sitting between pure passive (market-cap-weighted)
and pure active (manager-discretionary). The L1-core
`pm-active-vs-passive-decision.md` framing treats these as a
hybrid case; the multifactor structure here makes the hybrid
explicit. **Source:** Pedersen (2015) pp.87-164.

The boundary with the L1-core
`pm-factor-models-intuition.md` sibling and with future-05 is
clear. The core sibling presents the multifactor pricing
structure abstractly; this extension card populates it with the
documented empirical anomalies. The future-05 equity-vertical
work covers security-level construction (which book values to
use, how to handle accruals, how to define momentum windows,
backtest construction discipline). **Source:** Pedersen (2015)
pp.87-164.

## See Also

- [`pm-efficient-markets-and-anomalies.md`](pm-efficient-markets-and-anomalies.md) — efficiently-inefficient framing that explains why these anomalies persist in equilibrium
- [`pm-factor-models-intuition.md`](pm-factor-models-intuition.md) — L1-core multifactor pricing structure that this card populates with empirical anomalies
- [`pm-multifactor-asset-pricing-intuition.md`](pm-multifactor-asset-pricing-intuition.md) — Cochrane-anchored derivation of the linear multifactor pricing equation that admits each anomaly as a priced factor

## Escalate to Raw When

Open Pedersen (2015) Pt.II directly when any of the criteria
below applies. **Source:** Pedersen (2015) pp.87-164.

- Detailed signal-construction methodology for each anomaly (B/M
  computation including off-balance-sheet adjustments, momentum
  window choice, quality-score weighting) — Pedersen Ch.9
  develops; deeper coverage belongs in future-05. **Source:**
  Pedersen (2015) pp.133-158.
- Equity strategy specifics: discretionary equity (Ch.7),
  dedicated short bias (Ch.8), quantitative equity (Ch.9). The
  card here surveys the cross-sectional anomalies that all three
  strategy types act on; per-strategy implementation is downstream.
  **Source:** Pedersen (2015) pp.95-158.
- The combined-factor portfolio construction problem (how much
  weight to put on value vs momentum vs quality, how to manage
  the resulting correlation structure) — Ch.9 introduces; deeper
  treatment belongs in future-05. **Source:** Pedersen (2015)
  pp.133-158.
