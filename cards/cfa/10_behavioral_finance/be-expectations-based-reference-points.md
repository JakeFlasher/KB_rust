---
schema_version: "cacg.v0"
id: "be-expectations-based-reference-points"
title: "Expectations-Based Reference Points"
reading_id: "10_behavioral_finance"
summary: "Koszegi-Rabin make the reference point the rational expectation of outcomes: gains and losses are felt relative to a reference lottery R, each outcome compared to every outcome that might have occurred; imposing that expectations equal actual behavior (personal equilibrium) sharply disciplines the otherwise free reference point."
tags: ["behavioral-finance", "reference-dependence", "expectations-based", "personal-equilibrium", "koszegi-rabin"]
citations:
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p039:0054"
    chunk_hash: "26c08c554dbaba624ba4074c3837c7394fbd3895723090a69578176ee047e236"
    page_range: [40, 40]
    quote: "gains and losses are defined relative to an expectations-based referent"
    edge_type: "defines"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p040:0055"
    chunk_hash: "68b581162727181936671ffef3ec0ac2b72c626f94c8a1ddf30868c0c3e2f961"
    page_range: [40, 40]
    quote: "if one uses an expectations-based referent combined with an assumption that expectations must be rational, the"
    edge_type: "supports"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p041:0056"
    chunk_hash: "f5eefeb898dadc23f452c24d5e6ea536727df972bc12bbf90c72f0cfa328be5d"
    page_range: [41, 41]
    quote: "is compared to every outcome that might have occurred in the reference lottery R."
    edge_type: "supports"
  - source_id: "bf_hbe_vol1_2018"
    chunk_id: "bf_hbe_vol1_2018:p043:0059"
    chunk_hash: "23732ff5fd7b2bd6ceca59c2e26fc687a33ae4ed6010e72eb9416be5d62b298b"
    page_range: [43, 43]
    quote: "much of the literature has assumed that expectations must be"
    edge_type: "supports"
card_hash: "9d8ee27118ece344726755f841a35688e364ccff8c1798a2b92f7608690423b0"
---
# Expectations-Based Reference Points

## Intuition

A persistent weakness of reference dependence is that the reference point `r` is a free parameter -- choose it cleverly and the model can rationalize almost anything. Koszegi and Rabin (KR) propose a discipline: make the reference point the person's *rational expectation* of outcomes. Gains and losses are then felt relative to what the person expected to get, not relative to the status quo or an arbitrary anchor. Crucially, if the expectations-based referent is combined with the requirement that expectations be rational (consistent with actual behavior), the model's flexibility is dramatically reduced -- the reference point is no longer a free dial.
**Source:** O'Donoghue & Sprenger (2018) §5.1 pp.39.

Because expectations involve uncertainty, the referent is a *lottery*, not a single outcome. The first modeling question is how to compare a realized outcome to a reference *distribution*. KR's answer differs from the older disappointment-aversion (DA) tradition. DA compares each outcome to a single summary statistic (the mean of the reference lottery). KR instead compare each outcome to *every* outcome that might have occurred, weighting by the reference probabilities. So an outcome larger than some reference outcomes but smaller than others is felt as a gain relative to the former and a loss relative to the latter, and these sensations are aggregated.
**Source:** O'Donoghue & Sprenger (2018) §5.2 pp.40, 41.

The second question is what determines expectations. Two routes: exogenous expectations (treated like a standard fixed reference point, suitable for "surprise choice"), or endogenous expectations induced by the choice itself. KR's solution concepts are "personal equilibrium" -- where the reference point equals the rational forecast of one's own behavior -- and, when commitment precedes resolution of uncertainty, "choice-acclimating personal equilibrium" (CPE), where the chosen lottery `L` becomes its own referent so utility is `U(L|L)`. The rational-expectations consistency condition is exactly the discipline that pins down the reference point.
**Source:** O'Donoghue & Sprenger (2018) §5.3-5.3.2 pp.42, 43.

## Definition

**Expectations-based referent** is a reference point determined by the person's expectations over outcomes, formalized as a reference *lottery* `R = (r_1, q_1; ...; r_M, q_M)` rather than a single outcome.
**Source:** O'Donoghue & Sprenger (2018) §5.1-5.2 pp.39, 40.

**Personal equilibrium (rational-expectations discipline)** is the requirement that expectations be consistent with the person's own behavior -- the reference point is the rational forecast of what she will actually do, removing the free-parameter flexibility.
**Source:** O'Donoghue & Sprenger (2018) §5.1, §5.3 pp.39, 42.

**KR gain-loss comparison** compares each realized outcome `x_n` to every outcome that might have occurred in the reference lottery `R`, aggregating the gain/loss sensations with weights equal to the reference probabilities `q_m`.
**Source:** O'Donoghue & Sprenger (2018) §5.2 pp.40, 41.

**Choice-acclimating personal equilibrium (CPE)** is the case where commitment precedes the resolution of uncertainty, so the chosen lottery `L` becomes its own referent and the person maximizes `U(L|L)`.
**Source:** O'Donoghue & Sprenger (2018) §5.3.2 pp.43.

## Mathematical Reasoning

Given a reference lottery `R`, a person evaluates lottery `L = (x_1, p_1; ...; x_N, p_N)` by

```
  U(L | R) = sum_{n=1}^{N} p_n [ u(x_n) + v(x_n | R) ],
```

where `u(x_n)` is intrinsic utility and `v(x_n | R)` is the gain-loss utility relative to `R`. The two traditions differ in `v(x_n|R)`:

```
  DA approach:  v(x_n|R) = mu( u(x_n) - sum_m q_m u(x_m) )          (compare to the mean)
  KR approach:  v(x_n|R) = sum_m q_m * mu( u(x_n) - u(x_m) )        (compare to every outcome)
```

with `mu(.)` the gain-loss function (loss aversion: losses weighted more).
**Source:** O'Donoghue & Sprenger (2018) §5.2 pp.40.

Under endogenous expectations (`R = L`), the two solution concepts evaluate the chosen lottery as

```
  DA:   U(L|L) = sum_n p_n [ u(x_n) + mu( u(x_n) - sum_m p_m u(x_m) ) ]
  CPE:  U(L|L) = sum_n p_n [ u(x_n) + sum_m p_m mu( u(x_n) - u(x_m) ) ].
```

For CPE with a two-part-linear `mu` and ordered outcomes `x_1 <= ... <= x_N`, defining `Lambda = eta(lambda - 1)`, the source derives `U(L|L) = sum_n p_n u(x_n) - sum_{n} sum_{m>n} p_n p_m Lambda ( u(x_m) - u(x_n) )`, so only the product `Lambda` (not `eta` and `lambda` separately) matters.
**Source:** O'Donoghue & Sprenger (2018) §5.3.2 pp.45.

A key behavioral difference: under DA, increased risk in the reference lottery that leaves its expected intrinsic utility unchanged has *no* effect on behavior; under KR, such increased reference risk makes the person *more willing to bear risk* -- an endowment effect for risk. Sprenger's experiments find support for KR over DA. Both DA and CPE can generate violations of first-order stochastic dominance and betweenness, because probabilities enter nonlinearly through both the expectation and the referent.
**Source:** O'Donoghue & Sprenger (2018) §5.2, §5.3.2 pp.41, 44.

## See Also

- [be-reference-dependent-preferences-foundations](./be-reference-dependent-preferences-foundations.md#intuition) -- the gain-loss / reference-point foundation this card builds on.
- [be-loss-aversion-reference-dependence](./be-loss-aversion-reference-dependence.md#intuition) -- the loss-aversion asymmetry inside `mu(.)`.
- [be-prospect-theory-asset-pricing](./be-prospect-theory-asset-pricing.md#intuition) -- where "forward-looking" expectations-based reference points are discussed for finance.
- [be-regret-aversion-status-quo-endowment](./be-regret-aversion-status-quo-endowment.md#intuition) -- the endowment-effect intuition that KR's reference risk extends to risk.
- [be-household-liquidity-illiquidity-puzzle](./be-household-liquidity-illiquidity-puzzle.md#intuition) -- an application setting for expectations-based reference dependence.

## Escalate to Raw When

- You need the full worked Example 1 contrasting DA and CPE choices `L_1` vs `L_2` with the explicit thresholds (p.41).
- You need the DA/CPE indifference-curve geometry (Figure 4) and the first-order-stochastic-dominance violation example with `L_1, L_2` and `Lambda > 1.1` (pp.44-45).
- You need the precise distinction between personal equilibrium (PE) and CPE, and the timing assumptions about commitment vs. resolution of uncertainty (Sections 5.3.1-5.3.2, pp.42-43).
