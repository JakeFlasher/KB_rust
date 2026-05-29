---
schema_version: "cacg.v0"
id: "rm-scenario-analysis"
title: "Scenario Analysis — Forward-Looking Loss Aggregation under Designed Scenarios"
reading_id: "11_risk_management"
summary: "Scenario-based risk measures as deterministic complement to law-determined VaR/ES: designed scenarios L(x), generalized-scenario representation, and aggregation of stress losses; per McNeil Ch.2 §2.3.1 + Ch.8 §8.4.2 (stressing risk factors)."
tags: ["risk-management", "scenario-analysis"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p084:0119"
    chunk_hash: "ac9bbb217978a42fc448d1856dedf06d4120e4f40eccc23b918161ae461be894"
    page_range: [84, 85]
    quote: "A risk measure of the form (2.16), where P[X,w] is replaced by some arbitrary subset P of the set of all probability measures on the space of risk-factor changes, is termed a generalized scenario."
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p323:0461"
    chunk_hash: "b2b4b17ec49ab76b42c964462ccb09055cd9b4bf21b5de54d6d81a0099d6886a"
    page_range: [323, 323]
    quote: "Aggregation Based on Stressing Risk Factors"
    edge_type: "supports"
card_hash: "8ab02a799ea93e9d90bba649ed6f7e103338e6b9f38118f96d6714c3b751e742"
---
# Scenario Analysis — Forward-Looking Loss Aggregation under Designed Scenarios

## Intuition

**Scenario analysis** is the deterministic counterpart to probabilistic VaR / ES: rather than reading a tail quantile off a loss distribution, the risk function **designs** a small set of factor-shock scenarios and full-revalues the portfolio under each. The output is a vector of scenario-specific losses `{L_s}` rather than a single quantile, and the risk function evaluates the portfolio against each individually. Scenarios are designed to probe specific concerns — concentration in a sector, exposure to a particular macro regime, dependence on a specific liquidity source — that the law-determined risk measures (VaR / ES) may average out. **Source:** McNeil et al. (2015) Ch.2 pp.61-64.

The scenario landscape spans a **severity spectrum** from central case (expected outcomes) through moderate stress (plausible adverse), historical extreme (prior crisis replay), hypothetical extreme (beyond-historical constructions), and finally reverse stress (find the scenario that explains a specified loss). The spectrum's structure is the scenario-pyramid primitive: as severity rises, frequency drops; designed scenarios live below the historical record and into hypothetical territory. The two construction modes — **historical-extreme replay** (revalue under the factor moves observed during a prior crisis episode) and **hypothetical construction** (design plausible adverse joint moves that have not occurred but could) — trade backward-looking realism against forward-looking relevance. **Source:** McNeil et al. (2015) Ch.2 pp.61-64 + Ch.8 pp.302-305.

Scenario analysis sits **alongside** the law-determined risk measures, not as a substitute. A coherent risk-management programme reports VaR/ES (the law-based summaries) AND scenario losses (the designed-stress probes). VaR/ES tells the firm how bad an "average tail period" is; scenarios tell the firm how bad specific adverse joint moves are. Disagreement between the two — a scenario loss far exceeding the implied tail risk — is a diagnostic signal pointing to either factor-model mis-specification or unmodelled correlation between factors in the scenario. **Source:** McNeil et al. (2015) Ch.2 pp.61-64.

```
<!-- primitive: scenario-pyramid source: _diagram_primitives.md -->
              ___________________
             /                   \
            /     central case    \           expected outcomes
           /_______________________\
          /                         \
         /     moderate stress       \        plausible adverse
        /_____________________________\
       /                               \
      /      historical extreme         \     prior crisis replay
     /___________________________________\
    /                                     \
   /       hypothetical extreme            \  beyond historical
  /_________________________________________\
 /                                           \
/    reverse stress (loss → solvency limit)   \  find the scenario
\_____________________________________________/  explaining the loss

   severity ↑ as you descend; frequency ↓ as you descend
   reverse stress inverts the question: given a target loss, what scenario?
```

## Definition

Let `X ∈ R^d` be the risk-factor vector and `V(X)` the portfolio value function. A **scenario** `s` is a designed factor shock `ΔX_s ∈ R^d` representing a specific adverse (or non-adverse) joint factor move. The corresponding **scenario loss** is: **Source:** McNeil et al. (2015) Ch.2 pp.61-63.

```
L_s  =  −( V(X_0 + ΔX_s)  −  V(X_0) )

(full-revaluation under the scenario; symbolic form, no worked plug-in)
```

A **scenario set** `S = {s_1, s_2, …, s_K}` is a finite collection of designed scenarios, producing a vector of scenario losses `{L_{s_1}, L_{s_2}, …, L_{s_K}}`. The risk function evaluates each `L_{s_k}` individually against limits and reports the full vector to oversight. **Source:** McNeil et al. (2015) Ch.2 pp.61-64.

The two principal construction modes are: **Source:** McNeil et al. (2015) Ch.2 pp.61-64 + Ch.8 pp.302-305.

```
historical replay:   ΔX_s  =  ΔX_{historical event}    (e.g., prior crisis)
hypothetical:        ΔX_s  =  designed (analyst-chosen) (forward-looking, beyond
                                                          historical record)
```

A **reverse-stress scenario** inverts the construction: given a target loss `L*` (typically the solvency limit or a board-level tolerance), find a scenario `s*` such that `L_{s*} ≈ L*` and is **plausible** (low but non-negligible probability under the factor model). The reverse-stress output is the **scenario explanation** of the loss — what joint factor move could plausibly produce such a loss. **Source:** McNeil et al. (2015) Ch.8 pp.302-305.

A **scenario-coherent aggregation** combines a base loss measure `ρ(L)` (VaR or ES under the unstressed distribution) with the scenario set: the firm-wide stressed measure is `max_s { L_s }` (worst-scenario loss) or, more conservatively, `ρ(L | scenario set S)` where the loss distribution is conditioned on at least one scenario being realised. **Source:** McNeil et al. (2015) Ch.8 pp.302-305.

## Mathematical Reasoning

The structural difference between scenario analysis and law-determined risk measures is **determinism vs probability**. VaR/ES reduce the loss distribution to a single number by integrating over probability mass; scenarios reduce the loss to a single number per scenario by **conditioning** on a specified factor realisation. Both reduce dimensionality, but along orthogonal axes: VaR/ES averages over states, weighting by probability; scenarios pick out specific states, ignoring probability. The two are complementary because they answer different questions — "how bad is a typical adverse period?" vs "how bad is THIS adverse joint move?". **Source:** McNeil et al. (2015) Ch.2 pp.61-64.

The **historical-extreme replay** route is the most empirically grounded scenario construction: pick a historical date, lift the factor moves from that date, and revalue the current portfolio. The advantage is that the factor moves are internally consistent — equity-down comoves with credit-spread-widening, dollar-up comoves with EM-equity-down — exactly as observed in the historical event. The disadvantage is that the firm's current portfolio composition differs from the composition at the historical event; the historical factor moves may translate to a portfolio-loss pattern unrepresentative of the current book. **Source:** McNeil et al. (2015) Ch.2 pp.62-63.

The **hypothetical-construction** route addresses forward-looking concerns the historical record cannot anchor: a new regulatory regime, a sovereign default of a country that has not defaulted, a cyber-attack on payment infrastructure. The advantage is forward-looking relevance; the disadvantage is the **scenario design risk** — the analyst's chosen joint factor moves may be internally inconsistent (e.g., assuming a vol spike without the accompanying credit-spread widening). Practice addresses this by specifying the scenario as a "trigger event" and using a calibrated factor-model to fill in the joint factor moves conditional on the trigger. **Source:** McNeil et al. (2015) Ch.2 pp.62-64.

The **reverse-stress** technique inverts the usual flow: instead of designing a scenario and computing its loss, fix a target loss (typically the solvency limit) and search the scenario space for the **most-plausible** scenario producing that loss. The output is the "story" of the firm's worst plausible outcome. Reverse stress is the answer to a board-level question: "what would have to happen for us to fail?". The plausibility constraint matters — without it, reverse stress reduces to "any sufficiently extreme scenario", which is uninformative. McNeil Ch.8 §8.3 treats the formal connection between reverse stress and coherent-risk-measure dual representations. **Source:** McNeil et al. (2015) Ch.8 pp.302-305.

The **scenario set coverage** is a fundamental design question: how many scenarios, of what severities, covering what factor combinations? A small set is easy to communicate but probes few hypotheses; a large set is comprehensive but loses focus and burdens reporting. Industry practice typically maintains a **standing scenario library** covering the firm's principal exposures plus an **ad-hoc scenario capability** (new scenarios designed in response to emerging concerns). The library should span the severity spectrum from moderate stress through hypothetical extreme — reverse-stress scenarios sit at the bottom. **Source:** McNeil et al. (2015) Ch.2 pp.61-64.

A subtle structural point: scenario analysis **does not require a calibrated factor distribution** — the scenarios are designed, not sampled. This makes it robust to factor-model mis-specification (the dominant error source in parametric VaR; see `[[rm-parametric-var]]`). The trade-off is that without a probability distribution, no quantile / expectation summary is possible; the risk function reports a vector of scenario losses and lets oversight read the vector individually. Industry practice combines scenario losses with their (analyst-assigned, subjective) plausibility weights to produce a probability-weighted scenario summary — a hybrid that returns to the law-determined framing without losing the scenario specificity. **Source:** McNeil et al. (2015) Ch.2 pp.61-64.

## See Also

- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — Batch-0 loss-distribution machinery that scenario analysis complements.
- [rm-risk-type-taxonomy](./rm-risk-type-taxonomy.md) — Batch-0 card framing the risk types that scenarios probe.
- [rm-stress-testing](./rm-stress-testing.md) — Batch-2 sibling card on stress-testing as a generalisation of scenario analysis (coherent-risk-measure dual + EVT tail framing).

## Escalate to Raw When

The conceptual depth in this card stops at the scenario taxonomy + historical-vs-hypothetical-vs-reverse construction + scenario-coherent aggregation overview. When the operator needs the full formal connection to coherent-risk-measure dual representations, regulator-prescribed scenario libraries (CCAR, EBA stress tests, IMF FSAP), or scenario-generation machine-learning techniques (generative adversarial networks for synthetic scenarios, conditional copula sampling), open McNeil Ch.2 §2.3.1 + Ch.8 §8.4.2 pp.61-64 + 302-305 directly. Machine-learning scenario generation defers to future-01 quantitative methods. **Source:** McNeil et al. (2015) Ch.2 pp.61-64 + Ch.8 pp.302-305.
