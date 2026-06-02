---
schema_version: "cacg.v0"
id: "pm-active-vs-passive-decision"
title: "Active vs Passive Decision"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Active vs Passive Decision: walking through the L1-framed decision between active and passive management — when each is rational, what costs and benefits enter the comparison, and how market efficiency interacts with the choice"
tags: ["portfolio-management", "active-vs-passive", "fees"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3065:4578"
    chunk_hash: "3eeb41d6e312a7e8b8be14a0cca0f4377e367bb122697da557c2ffadc3c50871"
    page_range: [3065, 3065]
    quote: "management fees for index (or other passive) funds are often a fraction of those for active strategies. Another catalyst is the challenge that many active asset managers face in generating ex ante alpha"
    edge_type: "supports"
card_hash: "1136cd1dfd994bac267dfdc85f518dbd8cec11dc3fdfefdb0f7155a40bc7f769"
---
# Active vs Passive Decision

## Intuition

Active management bets that a manager can earn return above a
benchmark net of costs; passive management buys the benchmark and
accepts its return. The active-vs-passive choice is rational only
when the expected after-cost active return exceeds zero, which
requires both that mispricings exist (so alpha is available) and
that the chosen manager has the skill to capture it net of fees.
If either condition fails, passive dominates. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.420-441.

```
<!-- primitive: active-passive-decision-tree source: _diagram_primitives.md -->
                       market segment efficient?
                              |
                  +-----------+-----------+
                  | yes (semi-strong /     | no (anomalies persist;
                  |  strong form holds)    |  alpha may be available)
                  v                        v
              choose passive          mgr expected alpha
              (index / ETF)               > active costs?
                                              |
                                   +----------+----------+
                                   | yes (skill +         | no (costs
                                   |  net of fees beats   |  exceed alpha;
                                   |  passive)            |  net hurt)
                                   v                      v
                               choose active         choose passive
                               (selected mgr)        (default fallback)
```

The L1 framing is asymmetric. Passive is the default fallback
because its expected return matches the benchmark and its costs are
low and observable. Active is the deviation that must justify
itself: the manager must clear a hurdle equal to the cost
difference plus the tracking-risk penalty. The asymmetry reflects
the realization that net-of-cost average active performance is
zero by construction — winners and losers split the alpha pie
evenly, but the active investor pays fees that passive does not.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

## Definition

A passive investment strategy holds a portfolio designed to track
a benchmark — an index fund or an ETF replicating an index. The
expected return is the benchmark return less a small replication
cost. Tracking error against the benchmark is small by construction.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

```
E[r_passive] = E[r_b] - cost_passive
```

An active investment strategy deviates from the benchmark in pursuit
of alpha. The expected return is the benchmark return plus
expected alpha less active costs. Tracking error against the
benchmark is meaningfully nonzero by design. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.420-441.

```
E[r_active] = E[r_b] + E[alpha] - cost_active
```

The active-vs-passive choice rule reduces to a comparison of
expected after-cost returns. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

```
choose active  iff  E[alpha] > cost_active - cost_passive
```

The cost-difference threshold is sometimes called the "active
hurdle." Active wins only if expected alpha clears the hurdle. The
investor's belief about market efficiency determines `E[alpha]`:
under semi-strong-or-stronger efficiency, `E[alpha]` for a
representative manager is zero and the hurdle is impossible to
clear; under weaker forms or in segments with documented
inefficiencies, `E[alpha]` may be positive for a skilled manager
and the hurdle becomes a real test. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.420-441.

## Mathematical Reasoning

The cost of active management has multiple components: management
fees (higher than passive in expectation), transaction costs from
active trading (commissions plus market impact plus spread), tax
drag (higher turnover triggers more realized capital gains), and
the implicit cost of tracking-risk concentration (the active
investor is less diversified than the benchmark in the trade-
weighted dimension). **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

```
cost_active - cost_passive
   = delta_fees + delta_trading + delta_tax + delta_tracking_risk
```

Each component is positive in expectation: fees are higher; trading
volume is higher; turnover is higher; concentration is greater. The
active hurdle is therefore strictly positive in practical
applications, and tax-exempt institutional investors face a lower
hurdle than retail investors with after-tax accounts because tax
drag drops out for the former. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.420-441.

The market-efficiency premise interacts directly with `E[alpha]`.
The CAPM-equilibrium argument from the SML card shows that under
strict CAPM assumptions every asset prices to its beta-implied
expected return, so `E[alpha] = 0` for any manager. Empirical
violations — size, value, momentum, and other documented anomalies
— suggest `E[alpha]` may be nonzero in some segments, but the
average manager cannot capture more than zero alpha after costs
because the alpha sum across managers is bounded by the deviation
sum of all market participants. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.540-565.

The implication for the investor is two-step. First, identify
whether the segment is efficient enough that no representative
manager has positive `E[alpha]`. If yes, the choice is settled —
passive dominates and saves the cost difference deterministically.
Second, if some managers may have positive `E[alpha]`, the investor
must select among them with confidence high enough to overcome the
hurdle. The selection problem is well-documented to be hard — past
returns are a noisy signal of future skill — and the L1 framing
treats it as a meaningful but not automatic step. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.420-441.

The conventional retail-investor conclusion follows: in
broadly-followed segments (large-cap U.S. equity, developed-market
sovereign debt) the efficiency hurdle is high enough that passive
wins on average; in less-covered segments (small-cap international
equity, emerging-market high yield, micro-cap distressed) the
inefficiency hurdle is lower and active is more often defensible.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.

## See Also

- [`pm-portfolio-perspective.md`](pm-portfolio-perspective.md) — the portfolio-process loop in which the active / passive choice is made
- [`pm-capm-and-sml.md`](pm-capm-and-sml.md) — CAPM equilibrium pricing under which alpha is zero by construction
- [`pm-market-efficiency-core.md`](pm-market-efficiency-core.md) — the three CFA-framing forms that determine whether `E[alpha]` is plausibly positive

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R48 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.420-441.

- Worked manager-evaluation procedure (peer-group analysis, t-stat
  on excess return, persistence testing) — Vol.6 R48 outlines the
  method; deeper development belongs in future-13 / future-15.
  **Source:** CFA L1 Curriculum (2022) Vol.6/pp.420-441.
- Smart-beta and factor-tilt strategies as a hybrid third option —
  the curriculum mentions the category; deeper construction belongs
  in future-05 equity vertical. **Source:** CFA L1 Curriculum (2022)
  Vol.6/pp.420-441.
- Pedersen-style efficiently-inefficient framing where alpha is the
  payment for risk-bearing capacity — this lives in the AC-42
  extension card (`pm-active-management-and-alpha.md`). **Source:**
  CFA L1 Curriculum (2022) Vol.6/pp.420-441.
