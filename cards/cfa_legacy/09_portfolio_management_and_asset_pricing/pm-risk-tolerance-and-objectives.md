---
schema_version: "cacg.v0"
id: "pm-risk-tolerance-and-objectives"
title: "Risk Tolerance and Objectives"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Risk Tolerance and Objectives: framing the IPS return-and-risk objectives at L1 depth — distinguishing return objective formulation, risk objective formulation, and the ability-vs-willingness decomposition of risk tolerance"
tags: ["portfolio-management", "risk-tolerance", "ability-vs-willingness"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3278:4938"
    chunk_hash: "44499ae35c821428f8f7bebfb9bb6c80c9199c7191ebf80e0dd2586a0ca13e79"
    page_range: [3278, 3279]
    quote: "A client’s overall risk tolerance is a function of the client’s ability to bear (accept) risk and her “risk attitude,” which might be considered as the client’s willingness to take risk."
    edge_type: "defines"
card_hash: "0770de30da7c54df09f47991d797e70e50e44a915ea2a68e904d9c5a9c859b76"
---
# Risk Tolerance and Objectives

## Intuition

The investor's objectives are stated in two pieces: what the
portfolio must earn (return objective) and what variability the
investor can absorb in earning it (risk objective). The two
objectives are coupled — a high return objective is achievable only
by accepting high risk under any sensible risk-return frontier — so
they cannot be set independently. The IPS records both, with the
risk objective decomposed further into the investor's ability and
willingness to bear risk. Ability is a financial fact (how much
loss can be absorbed without compromising the goal); willingness is
a psychological fact (how much loss can be absorbed without
behavioral failure). The binding constraint is whichever is
smaller. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.

```
       risk tolerance
       ==============
       +-----------------------+
       |   ability to bear     |  financial: time horizon, wealth
       |   risk                |   buffer, income stability
       +-----------------------+
                |
                +---- min ---->  binding risk tolerance
                |                used in IPS objective
       +-----------------------+
       |   willingness to bear |  behavioral: loss aversion, prior
       |   risk                |   experience, expressed preference
       +-----------------------+
```

When ability and willingness disagree, the curriculum's prescription
is to take the minimum and educate toward convergence. An investor
with high ability but low willingness should not be sold a high-
risk portfolio; doing so risks behavioral failure (panic selling at
the trough) that destroys ability. An investor with high willingness
but low ability should not be sold a high-risk portfolio either;
the financial buffer is insufficient even if the investor's
preference suggests otherwise. The minimum protects against both
failure modes. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.

## Definition

The return objective is a stated target return level, in the
nominal-vs-real / pre-tax-vs-after-tax / required-vs-desired
language the IPS adopts. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.5-25.

```
return_objective:  R_target  (nominal or real, pre-tax or after-tax,
                              required or desired)
```

The "required" return is the minimum the portfolio must earn to
fund the stated goal (retirement income, scholarship endowment
spend rate, charitable distribution); the "desired" return is the
ambition above the minimum if risk capacity allows. **Source:** CFA
L1 Curriculum (2022) Vol.6/pp.5-25.

The risk objective is a stated tolerance for variability in the
portfolio's return realizations. The L1 framing typically uses a
volatility cap (`sigma_max`), a drawdown cap (peak-to-trough loss
ceiling), or a probability-of-shortfall cap (probability that the
realized return falls below the required-return threshold).
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.

```
risk_objective:  sigma(r_p) <= sigma_max
              or  Pr( r_p < R_required ) <= alpha_shortfall
              or  max_drawdown(r_p) <= drawdown_max
```

The risk tolerance — the input to the risk objective — is the
minimum of ability and willingness. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.5-25.

```
risk_tolerance = min( ability_to_bear_risk, willingness_to_bear_risk )
```

Ability is determined by the financial situation: longer time
horizon increases ability (more periods to recover from drawdown);
larger wealth-to-spending ratio increases ability (more buffer);
stable non-portfolio income increases ability (less reliance on
portfolio realization). Willingness is determined by the investor's
expressed preference and behavioral profile, typically elicited
through risk-tolerance questionnaires and discussion. **Source:**
CFA L1 Curriculum (2022) Vol.6/pp.5-25.

## Mathematical Reasoning

The objective specification feeds directly into the strategic
asset allocation. In a mean-variance formulation, the investor's
utility takes the canonical form. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.5-25.

```
U = E[r_p] - 0.5 · A · var(r_p)
```

The risk-aversion parameter `A` summarizes the investor's risk
tolerance — higher `A` corresponds to lower tolerance. The IPS-
specified return objective `R_target` and risk objective
`sigma_max` together pin down the required risk-return point on
the efficient frontier; the implied `A` is the value that makes
that point the utility maximum. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.5-25.

A specific implication for objective consistency: a return
objective that requires earning above the maximum-Sharpe-portfolio
return forces a leveraged-or-concentrated allocation that exceeds
the IPS risk objective. The two objectives are then jointly
infeasible. The IPS revision in this case must either lower the
return objective (accept less spending power), raise the risk
objective (accept higher volatility), or extend the time horizon
(longer compounding can lower the per-period return target).
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.

The probability-of-shortfall framing connects the return and risk
objectives more tightly. Approximating the portfolio return as
normally distributed with mean `E[r_p]` and standard deviation
`sigma_p`, the shortfall probability is determined by the z-score
distance from the required return. **Source:** CFA L1 Curriculum
(2022) Vol.6/pp.5-25.

```
Pr( r_p < R_required ) = Phi( (R_required - E[r_p]) / sigma_p )
```

The IPS shortfall constraint `Pr <= alpha_shortfall` translates to
a minimum z-score requirement. Setting the constraint tighter (a
smaller `alpha_shortfall`) forces a larger gap between expected
return and required return relative to volatility — the portfolio
must earn more in expectation, hold less risk, or lower the
required threshold. The connection between return and risk
objectives is therefore not a soft preference but a hard inequality
implied by the probability-of-shortfall constraint. **Source:**
CFA L1 Curriculum (2022) Vol.6/pp.5-25.

The behavioral significance of the willingness component is that
an investor who panic-sells at a drawdown crystallizes a loss the
portfolio could otherwise have recovered from over a longer horizon.
The minimum-of-ability-and-willingness rule prevents this failure
by ensuring the investor stays committed to the policy through
realized drawdowns within the stated risk objective. The L1 framing
treats willingness elicitation seriously even when ability would
permit more risk. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.5-25.

## See Also

- [`pm-investment-policy-statement.md`](pm-investment-policy-statement.md) — the IPS document in which the return and risk objectives are recorded
- [`pm-portfolio-constraints.md`](pm-portfolio-constraints.md) — the LLTTU constraint families that further restrict the feasible portfolio
- [`pm-investor-types.md`](pm-investor-types.md) — retail / institutional categories that drive characteristic ability profiles

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R51 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.5-25.

- Worked return-objective derivation under inflation, spending, and
  tax constraints — Vol.6 R51 walks through scenarios with concrete
  parameters that the present card abstracts symbolically.
  **Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.
- Risk-tolerance questionnaire design and standardized scoring
  rubrics — Vol.6 R51 mentions; the deeper development belongs in
  future-13. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.
- Behavioral-finance overlays (loss aversion, mental accounting,
  herding) on willingness elicitation — these route to future-10.
  **Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.
