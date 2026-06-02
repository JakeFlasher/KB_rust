---
schema_version: "cacg.v0"
id: "pm-active-management-and-alpha"
title: "Active Management and Alpha"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Active Management and Alpha: framing the evaluation of an active manager and the structure of alpha capacity — what makes a performance measure informative, why expected alpha decreases with assets under management, and how active risk-budgeting respects capacity limits"
tags: ["portfolio-management", "active-management", "alpha"]
citations:
  - source_id: "pm_pedersen_2015_efficiently_inefficient"
    chunk_id: "pm_pedersen_2015_efficiently_inefficient:p048:0052"
    chunk_hash: "f859afe4729bad71cf42dbe5f63c1745be745da084879b6e2ec033f848a97a30"
    page_range: [48, 49]
    quote: "Beta is the strategy’s market exposure, while alpha is the excess return after accounting for performance due to market movements."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3838:5848"
    chunk_hash: "3e27d3610cdb4797e0e87430597c332b3ca7d6e3bdfc7dafa9fcbaf80de1a02f"
    page_range: [3838, 3838]
    quote: "Tracking error The standard deviation of the differences between a portfolio’s returns and its benchmark’s returns; a synonym of active risk."
    edge_type: "supports"
card_hash: "afcfb4cd10351ac1a1318fee288131a8e7d570b793a9d6411b2fbaf45171a048"
---
# Active Management and Alpha

## Intuition

Evaluating an active manager requires answering two coupled
questions: did past performance reflect genuine skill, and does
expected future performance justify the cost of capital, fees,
and tracking risk? Pedersen frames the evaluation around three
properties: the performance measure must be sized to a relevant
benchmark (so the result is interpretable), it must be adjusted
for risk (so high-volatility strategies are not flattered), and
it must respect capacity (so a strategy that outperforms at small
size is not assumed to outperform at large size). Skill exists,
but skill is bounded — the manager who runs a $100M strategy
profitably may dilute the same edge to break-even at $10B.
**Source:** Pedersen (2015) pp.27-38.

```
        active management evaluation framework
        =======================================

        +---------------------------------------+
        |  performance measure (Sharpe / IR /    |
        |  alpha vs benchmark / drawdown)        |
        +---------------------------------------+
                       |
                       v
        +---------------------------------------+
        |  risk adjustment (factor exposures,    |
        |  beta-net-out, tracking error)         |
        +---------------------------------------+
                       |
                       v
        +---------------------------------------+
        |  capacity check (alpha decay with      |
        |  AUM; market-impact, position-limit    |
        |  binding constraints)                  |
        +---------------------------------------+
                       |
                       v
        +---------------------------------------+
        |  net-of-fees comparison vs passive     |
        |  benchmark; entry / exit decision      |
        +---------------------------------------+
```

The capacity dimension is the part Pedersen emphasizes most
relative to the L1-core framing. A skilled manager generates alpha
by transacting against demand-driven price distortions; the
distortions are bounded in size by the underlying flow. Doubling
the manager's capital does not double the available alpha because
the distortions and the trades that capture them are finite. The
result is a downward-sloping alpha-capacity curve: small managers
have high marginal alpha; large managers approach zero marginal
alpha. **Source:** Pedersen (2015) pp.54-62.

## Definition

A performance measure scales realized return by an appropriate
risk denominator. Pedersen catalogs four with distinct uses.
**Source:** Pedersen (2015) pp.27-38.

```
total Sharpe ratio:        S_p = (E[r_p] - Rf) / sigma_p
                            evaluates standalone; benchmark = riskless
information ratio:         IR_p = (E[r_p] - E[r_b]) / sigma_(p - b)
                            evaluates relative to benchmark; benchmark
                            = passive index
factor-adjusted alpha:     a_p = E[r_p] - Rf - sum_j  beta_(p,j) · lambda_j
                            evaluates skill net of all priced factor
                            exposures
drawdown / hitrate:        max drawdown, % winning trades
                            evaluate downside risk and consistency
                            beyond moments 1 and 2
```

The choice of measure picks the question being asked. A manager
loaded on size and value factors looks good on Sharpe and IR but
poor on factor-adjusted alpha (the factor returns explain the
performance). The factor-adjusted alpha is the conservative
measure of genuine skill above what passive factor exposure would
have delivered. **Source:** Pedersen (2015) pp.27-38.

The capacity-bounded alpha relation summarizes the empirical
regularity: expected after-cost alpha decreases monotonically with
fund size up to a saturation point where it equals zero.
**Source:** Pedersen (2015) pp.54-62.

```
expected after-cost alpha:
   alpha_net(AUM)  =  alpha_gross(AUM) - cost_per_trade · turnover - fees
   alpha_gross is decreasing in AUM
   transaction-cost component is increasing in AUM (market impact)
   fees component is approximately constant in % terms

   ==>  alpha_net(AUM)  is decreasing in AUM,
        crossing zero at some "critical capacity" AUM*
```

Beyond `AUM*`, the manager's marginal contribution to investor
return is negative; the manager should either return capital or
accept that incremental dollars are loss-making. The Berk-Green
equilibrium argument extends this: in equilibrium, fund flows
push every active manager to the capacity edge where net-of-
fees alpha is zero, leaving the manager-investor surplus
captured entirely by fees. **Source:** Pedersen (2015) pp.54-62.

## Mathematical Reasoning

The Sharpe-IR distinction is sharper than the L1 framing implies.
Sharpe rewards low standalone volatility; IR rewards low tracking
error against the benchmark. A market-neutral strategy that earns
modest returns at low volatility may have high Sharpe but low IR
against a benchmark whose factor exposure differs from the
strategy's (because tracking error is large when the strategy is
uncorrelated with the benchmark). A benchmark-tilted strategy
with the same modest returns and low tracking error may have
lower Sharpe (because volatility is higher) but higher IR.
**Source:** Pedersen (2015) pp.27-38.

The factor-adjusted alpha decomposition turns the manager
evaluation into a regression problem. **Source:** Pedersen (2015)
pp.27-38.

```
r_p(time) - Rf  =  alpha_p + sum_j  beta_(p,j) · F_j(time) + epsilon_p(time)
```

Genuine skill is measured by `alpha_p`. Factor loadings
`beta_(p,j)` are passive — any investor can replicate them at low
cost via factor ETFs. A manager whose `alpha_p` is statistically
indistinguishable from zero after adjusting for factor exposures
is not delivering skill; the manager is delivering factor exposure
that costs more than the passive equivalent. **Source:** Pedersen
(2015) pp.27-38.

The portfolio-construction discipline Pedersen presents in Ch.4
embeds capacity discipline in the optimization itself. Each
position is sized so that the marginal alpha contribution exceeds
the marginal market-impact cost. **Source:** Pedersen (2015)
pp.54-62.

```
optimal_position(stock_i):
   weight_i  =  c · alpha_(i)_per_unit_size  /  (transaction_cost_(i)
                +  marginal_factor_risk_(i))

   where:  alpha_per_unit_size declines with weight
           (large positions move the market against the trader)
           transaction_cost rises with size and turnover
           factor_risk grows with position concentration
```

The result is a position-by-position capacity check: each
candidate trade is sized to its own capacity. The aggregate
portfolio's alpha is bounded by the sum of per-position
capacities. **Source:** Pedersen (2015) pp.54-62.

A specific implication for the active-vs-passive decision: the
active hurdle from the L1-core sibling card is sharper under
the Pedersen framing. The hurdle is not just "expected alpha
must beat the cost gap" — it is "expected after-cost-and-after-
capacity alpha must beat the cost gap, and the manager must
have unused capacity to deploy the investor's incremental dollar."
A manager who is skilled but capacity-saturated cannot benefit
the investor's marginal allocation. **Source:** Pedersen (2015)
pp.54-62.

The boundary with the sibling cards is functional. The L1-core
`pm-performance-ratios-definitions.md` defines Sharpe / Treynor /
IR symbolically; this extension card connects them to the
capacity-aware evaluation of skill. The L1-core
`pm-tracking-error-and-active-risk.md` defines active risk in
benchmark-relative terms; this card adds the constraint that
active risk respects per-position capacity. **Source:** Pedersen
(2015) pp.27-62.

## See Also

- [`pm-active-vs-passive-decision.md`](pm-active-vs-passive-decision.md) — L1-core choice rule that this card refines with capacity discipline
- [`pm-performance-ratios-definitions.md`](pm-performance-ratios-definitions.md) — symbolic-form definitions whose interpretation this card sharpens
- [`pm-tracking-error-and-active-risk.md`](pm-tracking-error-and-active-risk.md) — benchmark-relative risk that interacts with the capacity constraint
- [`pm-efficient-markets-and-anomalies.md`](pm-efficient-markets-and-anomalies.md) — efficiently-inefficient framing that produces the alpha-as-liquidity-compensation equilibrium

## Escalate to Raw When

Open Pedersen (2015) Ch.2 / Ch.4 directly when any of the criteria
below applies. **Source:** Pedersen (2015) pp.27-62.

- Specific risk-management mechanics under leverage and funding
  constraints — Pedersen Ch.4 develops position sizing under
  capital-at-risk and Value-at-Risk constraints. **Source:**
  Pedersen (2015) pp.54-62.
- Backtest construction discipline (in-sample / out-of-sample
  splits, look-ahead bias, survivorship bias) — Ch.3 develops the
  backtest pitfalls catalog. **Source:** Pedersen (2015) pp.39-53.
- Manager interview and qualitative-evaluation procedures — the
  book's interview chapters illustrate; the L1 / extension framing
  here covers only quantitative evaluation. **Source:** Pedersen
  (2015) pp.27-38.
