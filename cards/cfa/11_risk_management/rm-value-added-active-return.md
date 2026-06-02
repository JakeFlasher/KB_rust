---
schema_version: "cacg.v0"
id: "rm-value-added-active-return"
title: "Value-Added and Active Return — L1 Notes Risk-Budget-Consumption Framing"
reading_id: "11_risk_management"
summary: "Sharpe / Treynor / M2 / Jensen's-alpha performance-appraisal triple plus the information ratio framed as risk-budget consumption per CFA L1 2022 Reading 50 'Portfolio Risk and Return: Part II' (CORRECTED to Vol.5, not Vol.6)."
tags: ["risk-management", "value-added"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3188:4756"
    chunk_hash: "a0b96d424a4abc9183c7e61dd279455f9a4fdcf824e613d01998ff24602fbb92"
    page_range: [3188, 3188]
    quote: "Four ratios are commonly used in performance appraisal."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3199:4770"
    chunk_hash: "94a609bfccce9993e0241aaf66b04679cf4e0d618854a6dfe078d1f0b20f234a"
    page_range: [3199, 3200]
    quote: "The larger the information ratio is, the more valuable the security."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3190:4759"
    chunk_hash: "f182e877d0d709b3b188a1724bb06e76923b962fccb2811eef3ed71182ae9bbb"
    page_range: [3190, 3191]
    quote: "Because the Sharpe ratio is defined as ERpf R p , the equation shows that M2 can be thought of as a rescaling of the Sharpe ratio that allows for easier comparisons among different portfolios."
    edge_type: "supports"
card_hash: "98170abf7995f406c1c9893d3b856a5c383ef7cad04bcccf963f29468217ea24"
---
# Value-Added and Active Return — L1 Notes Risk-Budget-Consumption Framing

## Intuition

The L1 source frame **active return** (`R_active = R_portfolio − R_benchmark`) and its decomposition as a **risk-budget consumption** problem from the risk-management vertical's perspective. Every basis point of expected active return purchased by an active manager consumes some basis points of **tracking-error budget** (active risk); the risk function's job is to verify that the budget allocation across positions is coherent with the overall tracking-error tolerance specified in the IPS (see `[[rm-risk-objectives-and-tolerance]]`). The realized active return ex-post is then compared against the ex-ante budget to assess whether the budget was consumed efficiently. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.5/pp.551-572.

The boundary against 09 Portfolio Management is sharp. **09 owns** the manager-skill / capacity / alpha-decay framing under the Pedersen extension: how to assess whether the manager's alpha is a fluke, a capacity-constrained edge, or a decaying signal; how active management fits into the fundamental law of active management (`IR = IC · √breadth`); how alpha persists or doesn't across periods. **11 owns** the risk-management framing: how the tracking-error budget is allocated, how realized active returns consume the budget, and whether the position-level active risks aggregate coherently to the portfolio-level constraint. The two cards complement each other; the L1 source anchor the 11-side framing without re-deriving the 09 machinery. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.5/pp.551-572 + CFA L1 2022 Vol.5 pp.551-572.

The source-side framing emphasises three operational points. (1) **Active return = portfolio return − benchmark return**: a simple subtraction, but the choice of benchmark is the load-bearing decision — an inappropriate benchmark inflates or deflates apparent active return without reflecting genuine manager skill or risk consumption. (2) **Active risk = tracking error = σ(R_active)**: a single statistic summarising the deviation of portfolio returns from benchmark returns; the IPS specifies an upper bound `s%` and the risk function monitors against it. (3) **Information ratio `IR = E[R_active] / σ(R_active)`** quantifies how much active return is earned per unit of active risk consumed, the canonical risk-adjusted-active-return measure; the L1 source flag it as a forward-looking expectation, while ex-post realized IR is the comparison statistic. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.5/pp.551-572.

```
   L1-source active-return risk-budget pipeline
   ───────────────────────────────────────────

   +-------------------+      +-------------------+
   | Portfolio return  |      | Benchmark return  |
   | R_portfolio       |      | R_benchmark       |
   +---------+---------+      +---------+---------+
             |                          |
             +-----------+--------------+
                         |
                         v
              +----------------------+
              | Active return        |
              | R_active = R_p − R_B |
              +----------+-----------+
                         |
                         v
              +----------------------+
              | Active risk          |
              | TE = σ(R_active)     |
              +----------+-----------+
                         |
                         v
              +----------------------+
              | Risk-budget consumed |
              | (compared against    |
              |  IPS tracking-error  |
              |  bound s)            |
              +----------+-----------+
                         |
                         v
              ex-post check:  IR = E[R_active] / TE
                              (info ratio = active alpha per unit active risk)
              (See pm-active-management-and-alpha for manager-skill
               / capacity / decay treatment — 09 territory)
```

## Definition

Let `R_p` be the portfolio's realized return and `R_B` the benchmark's realized return over the same period. The **active return** is: **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.5/pp.551-572 + CFA L1 2022 Vol.5 pp.551-572.

```
R_active  =  R_p  −  R_B            (single-period realised active return)
```

For a multi-period series of active returns, the **tracking error** (active risk) is the sample standard deviation of the active-return series: **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.5/pp.551-572 + CFA L1 2022 Vol.5 pp.551-572.

```
TE  =  σ(R_active)  =  √( E[ (R_active − E[R_active])² ] )
```

The **information ratio** combines expected active return and active risk into a single risk-adjusted statistic: **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.5/pp.551-572 + CFA L1 2022 Vol.5 pp.551-572.

```
IR  =  E[R_active] / σ(R_active)  =  expected active return / active risk
```

The L1 source' **risk-budget framing** treats `TE` as a **scarce resource** consumed by active positions. For a portfolio of `N` active positions with individual active weights `w_i^{active} = w_i − w_i^{benchmark}` and individual active-return contributions, the portfolio-level tracking error is: **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.5/pp.551-572 + CFA L1 2022 Vol.5 pp.551-572.

```
TE_portfolio  =  √( Σ_i Σ_j  w_i^{active} · w_j^{active} · Cov(R_i, R_j) )
```

Subject to the IPS tracking-error bound `TE_portfolio ≤ s`, the active-management problem is to allocate the active-weight budget across positions to maximise expected `R_active` per unit of `TE_portfolio`. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.5/pp.551-572.

## Mathematical Reasoning

The active-return / tracking-error pair is the **risk-management mirror image** of the return / volatility pair on the absolute scale. Where absolute risk management uses VaR / ES on the loss distribution of `R_p`, relative risk management uses tracking error on the loss distribution of `R_active`. The same statistical machinery (sample variance, distributional assumptions, sub-additivity of risk) applies, just with `R_active` in place of `R_p`. The 11 vertical treats this as a re-application of the loss-distribution apparatus from `[[rm-loss-distribution-anatomy]]` to the relative-return loss `L_active = −R_active = R_B − R_p`. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.5/pp.551-572.

The **risk-budget framing** rests on the **convexity of variance**: portfolio-level `TE²` is a weighted sum of position-pair active covariances, so reducing `TE` requires either lowering individual active weights `w_i^{active}` or holding positions whose active returns are negatively correlated (diversification across active bets). A naive risk-budget allocation gives each position an equal share of `TE`; an efficient allocation gives more `TE` to positions with higher expected active return per unit of active risk (Sharpe-optimal allocation). The source flags the efficiency argument without deriving the full mean-variance optimisation (that's 09 territory). **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.5/pp.551-572 + CFA L1 2022 Vol.5 pp.551-572.

The **information ratio** has a structural interpretation as the **t-statistic for active-return persistence**: under i.i.d. active returns with mean `μ_a` and standard deviation `σ_a`, the sample IR after `T` periods is `IR_sample ≈ N(μ_a / σ_a, 1 / √T)`. The hypothesis test "is the manager's IR genuinely positive?" reduces to "is `IR_sample · √T > critical value?" — which means distinguishing skill from luck requires a sufficiently long track record. The L1 source flag this **noise-to-signal** problem without deriving the full statistical test, which is 09 / future-15 territory. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.5/pp.551-572.

The **boundary between active return as risk-budget consumption (11) and as manager-skill signal (09)** has a clean separation in practice. The risk function's job is to **monitor that the budget is consumed within the IPS bound** — it does not assess whether the budget produced commensurate alpha. The portfolio-management function's job is to **decide whether the manager's active return is skill or luck** — it does not police the budget bound. The two functions communicate via a shared tracking-error report; the risk function flags budget overruns, and the portfolio management function flags persistent under-performance. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.5/pp.551-572.

The **realized vs expected active return** distinction is the standard ex-post / ex-ante decomposition: `E[R_active]` is the **expected** active return baked into the position sizing decision; realized `R_active` is the random outcome; the difference is the period's **active-return surprise**. The risk-management vertical reports both — expected (consumed budget) and realized (period P&L) — and the persistent gap between them flags either a stale expected-return model or a regime shift in the benchmark-portfolio relationship. **Source:** CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf Vol.5/pp.551-572.

## See Also

- [../09_portfolio_management_and_asset_pricing/pm-active-management-and-alpha](../09_portfolio_management_and_asset_pricing/pm-active-management-and-alpha.md) — 09 vertical's manager-skill / alpha-capacity / alpha-decay framing under the Pedersen extension.
- [../09_portfolio_management_and_asset_pricing/pm-tracking-error-and-active-risk](../09_portfolio_management_and_asset_pricing/pm-tracking-error-and-active-risk.md) — 09 vertical's portfolio-construction framing of tracking error as an active-management constraint.
- [rm-risk-objectives-and-tolerance](./rm-risk-objectives-and-tolerance.md) — Batch-0 card defining the IPS tracking-error bound that the active-return decomposition consumes.

## Escalate to Raw When

The L1-source treatment stops at the active-return / tracking-error / IR triple framed as risk-budget consumption. When the operator needs the full manager-skill assessment (fundamental law of active management, IC × √breadth decomposition, alpha-decay modelling, capacity constraints, persistence statistical tests), open the 09 cards above OR CFA L1 2022 Vol.5 pp.551-572 directly. Full performance-attribution worked drills (Brinson-Hood-Beebower, factor-based attribution, allocation / selection / interaction decomposition) defer to future-15 performance-and-attribution. **Source:** CFA L1 2022 Vol.5 pp.551-572.
