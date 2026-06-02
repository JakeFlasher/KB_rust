---
schema_version: "cacg.v0"
id: "rm-stress-testing"
title: "Stress Testing — Coherent-Risk-Measure Duals and Reverse Stress"
reading_id: "11_risk_management"
summary: "Stress testing as the dual of a coherent risk measure (rho(L) = sup_{Q in Q} E_Q[L]); the McNeil-Smith linear-portfolio result that every coherent measure on linear portfolios IS a stress test, with EVT-tail (generalized Pareto / Pickands-Balkema-de Haan) consistency framing per McNeil Ch.5 + Ch.8 §8.3."
tags: ["risk-management", "stress-testing"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p314:0448"
    chunk_hash: "68d289dfb73fcf91e893bc989486c7ec1c2ebf9d44e9afc32aa918da2a5e16cd"
    page_range: [314, 315]
    quote: "every coherent risk measure on the set M in (8.34) can be viewed as a stress test"
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p156:0219"
    chunk_hash: "826bd34c917a098b21142cb40d19e6f67ebe8523ebd9b6dee057b5d261fe9aaf"
    page_range: [156, 157]
    quote: "We focus on two main kinds of model for extreme values."
    edge_type: "supports"
  - source_id: "rm_christoffersen_2012_elements"
    chunk_id: "rm_christoffersen_2012_elements:p314:0435"
    chunk_hash: "7b0f31afeb9e10614749d90e702b258f97df171f638e55639d97d46745d8fa55"
    page_range: [314, 314]
    quote: "does not tell the portfolio manager anything about the probability of the scenario"
    edge_type: "supports"
card_hash: "36ee00ba8629f1f3a3e20a7789d4807ba41a9e09ada683303d0a6a97edd91ba0"
---
# Stress Testing — Coherent-Risk-Measure Duals and Reverse Stress

## Intuition

**Stress testing** generalises scenario analysis (see `[[rm-scenario-analysis]]`) by formalising the connection between designed scenarios and coherent risk measures. The McNeil-Frey-Embrechts treatment introduces a stress test as the dual of a coherent risk measure: for any coherent `ρ`, there exists a set `Q` of probability measures (the "generalised scenarios") such that `ρ(L) = sup_{Q ∈ Q} E_Q[L]`. A stress test that evaluates `E_Q[L]` for a specific `Q ∈ Q` is then a **lower bound** on `ρ(L)`; stress-testing across the full set `Q` recovers `ρ(L)` exactly. This connects scenario-design intuition to the axiomatic risk-measure framework. **Source:** McNeil et al. (2015) Ch.8 pp.293-298 + Ch.2 pp.72-78.

The stress-testing landscape has three structural pieces. (1) **Sensitivity-style stress** shocks one factor at a time (move equities by `δ_eq`, hold others constant) — used for risk-budget enforcement against single-factor limits. (2) **Multi-factor scenario stress** shocks a coordinated bundle of factors representing a coherent narrative (a sovereign default, a flash crash, a pandemic) — used for board-level risk reporting. (3) **Reverse stress** inverts the design: fix a target loss (typically the solvency limit) and search for the most-plausible scenario producing that loss — used to identify the "story" of firm failure. **Source:** McNeil et al. (2015) Ch.8 pp.293-298.

The **EVT (extreme-value theory) connection** is intuition-only at this depth: the tail of the loss distribution past the high-confidence quantile is governed by the **generalised Pareto distribution** under the Pickands-Balkema-de Haan theorem. EVT provides a model for the tail past the threshold where empirical observations are scarce — the same domain stress testing probes. The connection is that EVT-based tail-quantile estimators and well-designed stress scenarios should produce concordant loss magnitudes; persistent disagreement flags tail-mis-specification. Full EVT depth (POT estimators, GPD parameter fitting, threshold selection) defers to future-01 quantitative econometrics — the present treatment stays at extension intuition only. **Source:** McNeil et al. (2015) Ch.5 pp.135-172.

```
   Stress-testing landscape
   ────────────────────────

   Severity ↓        Stress type                       Use case
   ─────────────────────────────────────────────────────────────
                     sensitivity stress                single-factor risk-limit
                     (shock 1 factor at a time)         enforcement
                     ────────────────────────
                     scenario stress (multi-factor)   board-level risk reporting
                     - historical replay               + regulator submissions
                     - hypothetical construction
                     ────────────────────────
                     reverse stress                    identify the "story" of
                     (find scenario explaining          firm failure
                      a target loss)
                     ────────────────────────
                     EVT-tail (generalised Pareto)     deep-tail extrapolation;
                     intuition-only at this depth      consistency check against
                                                        stress losses
                     ────────────────────────

   Each row is a dual of a coherent risk measure:
     ρ(L)  =  sup_{Q ∈ Q} E_Q[L]
     a stress test = compute E_Q[L] for one Q (or a finite set of Q's)
     reverse stress = invert the question: find Q such that E_Q[L] = target
```

## Definition

A **stress test** is the evaluation of `E_Q[L]` for a designed probability measure `Q` (the "stress measure") on the factor space, where `L` is the portfolio loss. Equivalently, when `Q` concentrates mass at a single factor outcome `X_s`, the stress test is the deterministic scenario loss `L_s = −(V(X_s) − V(X_0))` from `[[rm-scenario-analysis]]`. **Source:** McNeil et al. (2015) Ch.8 pp.293-295.

The **dual representation of coherent risk measures** (Artzner-Delbaen-Eber-Heath, formalised in McNeil Ch.8 §8.1.2) states that for any coherent `ρ`, there exists a set `Q` of probability measures such that: **Source:** McNeil et al. (2015) Ch.8 pp.280-285 + 293-295.

```
ρ(L)  =  sup_{Q ∈ Q}  E_Q[L]
```

The set `Q` is the **generalised scenarios** that the risk measure considers. For `ρ = ES_α`, `Q` is parameterised by the bound `dQ/dP ≤ 1/(1−α)` (see `[[rm-expected-shortfall-mechanics]]`). A stress test for one `Q ∈ Q` evaluates `E_Q[L]` and provides a lower bound on `ρ(L)`; stress-testing across all `Q ∈ Q` recovers `ρ(L)` exactly. **Source:** McNeil et al. (2015) Ch.8 pp.293-298.

A **reverse-stress** procedure inverts the design: given a target loss `L*` (typically the solvency capital limit), find a stress measure `Q*` and corresponding scenario such that: **Source:** McNeil et al. (2015) Ch.8 pp.295-298.

```
E_{Q*}[L]  =  L*       AND     Q* is "plausible" under the factor distribution
```

The plausibility constraint is the load-bearing one — without it, reverse stress reduces to "any sufficiently extreme `Q`", which is uninformative. Plausibility is typically operationalised as a lower bound on `Q*` likelihood under the calibrated factor distribution, or as a constraint that the factor moves under `Q*` lie in a credibility region of the joint factor model. **Source:** McNeil et al. (2015) Ch.8 pp.295-298.

The **EVT extension** introduces the **generalised Pareto distribution (GPD)** as the asymptotic family for losses past a high threshold. The Pickands-Balkema-de Haan theorem states that for a broad class of underlying loss distributions, the tail past a sufficiently high threshold `u` is approximately GPD with parameters depending on the underlying distribution's tail behavior. EVT-based tail-quantile estimators fit the GPD to historical exceedances past `u` and extrapolate to even higher quantiles — addressing the deep-tail data-scarcity problem of historical-simulation VaR. The intuition is that stress losses should be checked against EVT-implied tail quantiles for consistency; the present treatment stays at extension intuition only. **Source:** McNeil et al. (2015) Ch.5 pp.135-172.

## Mathematical Reasoning

The **dual-representation framing** is the unifying structural fact for stress testing. Coherent risk measures are characterised by their dual scenario sets `Q`; stress testing is the operational practice of evaluating expectations under specific scenarios in `Q`. The theoretical content: a stress test that finds `Q` outside `Q` (i.e., a scenario the risk measure does not consider) provides no lower bound on `ρ(L)` — it tells you about a possible loss but does not constrain capital under `ρ`. Stress test design should ensure scenarios live inside (or are dominated by elements of) `Q`. **Source:** McNeil et al. (2015) Ch.8 pp.293-298.

The **reverse-stress mathematical structure** is an inverse problem: given a target loss `L*`, search the scenario space `Q` for the `Q*` that produces `E_{Q*}[L] = L*` while maximising plausibility (or, equivalently, minimising the Kullback-Leibler divergence from the unstressed measure `P`). The minimum-KL solution has a closed form in special cases (e.g., exponential tilting under Gaussian factor models) and a numerical solution in general. The output `Q*` decomposes into factor-specific shocks that, together, produce the target loss — providing the "scenario story" of firm failure. **Source:** McNeil et al. (2015) Ch.8 pp.295-298.

The **complementarity between stress testing and law-based risk measures** is structural. VaR/ES summarise the loss distribution into a single number by averaging probability mass; stress tests provide loss numbers conditional on specific (designed) scenarios. The two are not competitors — they answer different questions and should be reported jointly. Persistent gaps between a stress loss `E_Q[L]` and the law-based measure `ρ(L)` flag either (a) `Q` is outside the risk-measure's dual scenario set (and so requires extra capital beyond `ρ(L)`) or (b) the factor model under `ρ` is mis-calibrated (and underestimates the tail). Both are diagnostic; both warrant escalation. **Source:** McNeil et al. (2015) Ch.8 pp.293-298.

The **EVT-tail consistency check** is operationally useful even at intuition depth: stress losses at the high-confidence end should roughly track EVT-implied tail quantiles. If stress losses are systematically higher than EVT-quantiles at matched confidence levels, the stress scenarios may be over-conservative (or the EVT calibration may be missing tail thickness). If stress losses are systematically lower, the scenarios may be under-conservative (or the EVT model is over-fitting recent extreme observations). The cross-check disciplines both sides of the analysis. **Source:** McNeil et al. (2015) Ch.5 pp.135-172 + Ch.8 pp.293-298.

A subtle point on **plausibility**: the reverse-stress plausibility constraint is what separates "useful scenario identification" from "any extreme scenario will do". Without plausibility, reverse stress would identify scenarios so extreme they exceed any conceivable adverse joint move; with plausibility, the scenarios stay within the credibility region of the factor model. The plausibility constraint can be parameterised: tighter plausibility yields scenarios closer to the body of the factor distribution (less severe, more frequent); looser plausibility yields scenarios deeper in the tail (more severe, less frequent). Industry practice tunes the plausibility constraint to match the firm's risk appetite. **Source:** McNeil et al. (2015) Ch.8 pp.295-298.

## See Also

- [rm-scenario-analysis](./rm-scenario-analysis.md) — Batch-2 sibling card on deterministic scenario construction (the operational practice that stress testing generalises).
- [rm-expected-shortfall-mechanics](./rm-expected-shortfall-mechanics.md) — Batch-2 sibling card on ES + its dual representation (the canonical example of a coherent measure with stress-test interpretation).
- [rm-risk-measure-axioms](./rm-risk-measure-axioms.md) — Batch-0 card on the coherence axioms that underpin the dual-representation framework.
- `rm-coherent-probability-weighted-stress-testing` (Christoffersen (2012) Elements of FRM, pp.314) — deepening that extends this card.

## Escalate to Raw When

The conceptual depth in this card stops at the dual-representation framing + reverse-stress inversion + EVT-tail intuition. When the operator needs the full EVT machinery (peaks-over-threshold estimation, generalised Pareto parameter fitting, threshold selection via mean-excess / Hill plots, asymptotic distribution theory for EVT estimators), the regulator-prescribed stress packages (Federal Reserve CCAR / DFAST, EBA stress test scenarios, IMF FSAP frameworks), or the formal optimization theory for reverse stress (KL-divergence-constrained inversion, exponential-tilting closed forms), open McNeil Ch.5 pp.135-172 + Ch.8 §8.3 pp.293-305 directly. Regulator-prescribed stress packages live in their respective authorized regulatory texts. **Source:** McNeil et al. (2015) Ch.5 + Ch.8 pp.135-305.
