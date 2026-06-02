---
schema_version: "cacg.v0"
id: "rm-integrated-firm-wide-risk-aggregation"
title: "Integrated Firm-Wide Risk Aggregation — McNeil Ch.8 §8.4-§8.5"
reading_id: "11_risk_management"
summary: "Aggregates firm-wide risk across market / credit / operational silos via modular summation versus fully-integrated copula-based loss aggregation, with the Euler-principle capital allocation distributing the aggregate risk-measure back to silos as marginal capital contributions."
tags: ["risk-management", "integrated-firm"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p320:0457"
    chunk_hash: "a2473e4edae59d3d09a48a7d3c8967b35a6705e78ad4d9ddeefa01fd4ef477b1"
    page_range: [320, 321]
    quote: "8.4 Risk Aggregation The need to aggregate risk can arise in a number of situations."
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p525:0766"
    chunk_hash: "3b278bbf3e5ed8178596b26b7e5e5041fed1ed8ca251c3bf990b01ff392bccba"
    page_range: [525, 526]
    quote: "An essential difference between operational risk, on the one hand, and market and credit risk, on the other, is that operational risk has no upside for a bank."
    edge_type: "supports"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p337:0483"
    chunk_hash: "ad23c10d1547caae252db45c2ee1e9e89aa31ce2281e181853c8746b0d14d3b5"
    page_range: [338, 338]
    quote: "We now look at a number of specific examples of Euler allocations"
    edge_type: "supports"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p324:0463"
    chunk_hash: "4567799654940c7860d51566b611f7580fea4ed56bb9d24f44bed2f7e6a4527f"
    page_range: [324, 324]
    quote: "there can be complex interactions between risk factors that would require even higher capital than indicated by the sum of losses due to single-risk-factor stresses"
    edge_type: "supports"
  - source_id: "rm_hull_2023_rmfi"
    chunk_id: "rm_hull_2023_rmfi:p276:0370"
    chunk_hash: "fe563221dda08fc17f51c47f72f554e0e5e280ee93c5e2ea60069e02a2740a5f"
    page_range: [276, 276]
    quote: "the total VaR for a portfolio is the sum of the component VaRs for the subportfolios."
    edge_type: "supports"
  - source_id: "rm_bouchaud_potters_2003_theory_financial_risk"
    chunk_id: "rm_bouchaud_potters_2003_theory_financial_risk:p213:0280"
    chunk_hash: "e2bacebff7785139dd107c05052c721482e9f7c9a602f910695387a9a77e9ff6"
    page_range: [213, 213]
    quote: "are correlated Gaussian variables with a correlation coefficient ρ"
    edge_type: "supports"
  - source_id: "rm_potters_bouchaud_2020_random_matrix_theory"
    chunk_id: "rm_potters_bouchaud_2020_random_matrix_theory:p289:0346"
    chunk_hash: "a3a1db38fe30a52eaee04441c5a1b6261f3f792f70fcf36ad155fdeba5918915"
    page_range: [290, 290]
    quote: "the spectrum of the sample covariance matrix E is always wider (for"
    edge_type: "supports"
  - source_id: "rm_potters_bouchaud_2020_random_matrix_theory"
    chunk_id: "rm_potters_bouchaud_2020_random_matrix_theory:p349:0421"
    chunk_hash: "d7d4fe743e3c9abacd498c48ffd5aebd222ca1683205cf2e390f19f696a78730"
    page_range: [349, 349]
    quote: "the optimal rie (19.26) actually minimizes the out-of-sample risk within the class of rotationally invariant estimators"
    edge_type: "supports"
card_hash: "1da6b23eaf3bae89dcade5eb17144ee2dc15547d7d545d7f21865ec2e0ccd1f2"
---
# Integrated Firm-Wide Risk Aggregation — McNeil Ch.8 §8.4-§8.5

## Intuition

**Firm-wide risk aggregation** is the step that converts silo-level risk numbers (market `ρ(L_market)`, credit `ρ(L_credit)`, operational `ρ(L_op)`) into a single firm-wide capital number `ρ(L_total)`. Two structurally different approaches dominate practice: **modular aggregation** sums silo-level capitals directly (`ρ(L_total) = ρ(L_market) + ρ(L_credit) + ρ(L_op)`), and **fully-integrated aggregation** constructs the joint loss distribution across silos via a copula (or other dependence model) and applies `ρ` to the joint loss directly. Modular is simpler and audit-friendly but ignores diversification; integrated is more capital-efficient but requires a calibrated dependence model. **Source:** McNeil et al. (2015) Ch.8 pp.299-310.

Under a coherent risk measure `ρ` (e.g., ES — see `[[rm-risk-measure-axioms]]` for the coherence axioms), **subadditivity** delivers the structural inequality `ρ(L_total) ≤ Σ_silo ρ(L_silo)`: the firm-wide capital under integrated aggregation cannot exceed the sum of modular silo capitals. The inequality is **strict** whenever silo losses are not comonotone — i.e., whenever a bad day in market does not coincide with a bad day in credit with probability 1. The strictness measures the **diversification benefit** the firm earns from holding multiple imperfectly-correlated risk silos. **Source:** McNeil et al. (2015) Ch.8 pp.299-310.

Once the firm-wide capital `ρ(L_total)` is computed, the next operational question is **capital allocation**: how should `ρ(L_total)` be distributed back to silos (or to desks within silos) for the purpose of performance measurement, risk-adjusted return computation, and limit-setting? The **Euler-principle allocation** is the canonical answer: under positively-homogeneous `ρ`, Euler's identity gives a unique additive allocation where each silo's contribution equals its marginal capital impact. The Euler allocation is the same machinery used for trade-level XVA attribution in `[[rm-portfolio-xva-aggregation]]`. **Source:** McNeil et al. (2015) Ch.8 pp.310-322.

```
<!-- primitive: risk-aggregation-tree source: _diagram_primitives.md -->
                           +-----------------------+
                           |   Firm-wide risk      |
                           |   ρ(L_total)          |
                           +-----+-----------+-----+
                                 |           |
                  modular        |           |    fully-integrated
                  aggregation    |           |    aggregation
                  (sum or        |           |    (joint loss dist
                  copula-link)   |           |    via copula)
                                 |           |
              +------------------+           +-------------------+
              |                                                  |
       +------+-------+                                  +-------+------+
       | Market risk  |                                  | Credit risk  |
       | ρ(L_market)  |                                  | ρ(L_credit)  |
       +------+-------+                                  +-------+------+
              |                                                  |
              |                +---------------------+           |
              +--------------> |  Operational risk   | <---------+
                               |  ρ(L_op)            |
                               +---------------------+

   modular sum:   ρ(L_total) = ρ(L_market) + ρ(L_credit) + ρ(L_op)
   integrated:    ρ(L_total) <= sum of silos     (sub-additive bound)
   Euler allocation distributes ρ(L_total) back to silos
```

## Definition

Let `L_market`, `L_credit`, `L_op` (and possibly other silo-level losses) be the per-silo aggregate loss random variables and `L_total = L_market + L_credit + L_op` the firm-wide loss. **Modular aggregation** sums silo-level capitals: **Source:** McNeil et al. (2015) Ch.8 pp.299-310.

```
ρ_modular(L_total)  =  ρ(L_market)  +  ρ(L_credit)  +  ρ(L_op)
                     (silo capitals computed independently and summed)
```

**Fully-integrated aggregation** constructs the joint loss distribution via a copula `C` linking the marginal silo CDFs `(F_market, F_credit, F_op)` and applies `ρ` to the joint loss: **Source:** McNeil et al. (2015) Ch.8 pp.299-310.

```
F_total(l)  =  P( L_market + L_credit + L_op ≤ l )
            obtained from the joint distribution
            F_joint  =  C(F_market, F_credit, F_op)

ρ_integrated(L_total)  =  ρ( L_total )  read off  F_total

under subadditivity:
    ρ_integrated(L_total)  ≤  ρ_modular(L_total)        (diversification bound)
```

The **Euler-principle allocation** distributes the firm-wide capital back to silos: for a positively-homogeneous `ρ` parameterised by scaling factors `(λ_market, λ_credit, λ_op)` on each silo's exposure, Euler's identity gives: **Source:** McNeil et al. (2015) Ch.8 pp.310-322.

```
allocation_silo  =  λ_silo  ·  ∂ρ(L_total) / ∂λ_silo

Σ_silo  allocation_silo  =  ρ(L_total)                  (Euler additivity)
```

The partial derivative is the **marginal capital contribution** of an infinitesimal scaling of the silo's exposure, and the Euler allocation is the unique additive allocation where each silo's share equals its marginal contribution. **Source:** McNeil et al. (2015) Ch.8 pp.310-322.

## Mathematical Reasoning

The structural value of **subadditivity** is exactly the modular-vs-integrated gap. Under any coherent `ρ` (see `[[rm-risk-measure-axioms]]` for the axioms), `ρ(Σ L_i) ≤ Σ ρ(L_i)` by axiom; the integrated computation realises the inequality and the modular sum sits at the conservative upper bound. The gap is the **diversification benefit** — capital that can be released because not all silos lose money on the same day. The gap's size depends on the joint-distribution dependence: comonotone losses (perfectly correlated tails) give zero gap (modular = integrated), independent losses give a large gap, and tail-dependent losses (joint losses cluster in stress) give an intermediate gap. **Source:** McNeil et al. (2015) Ch.8 pp.299-310.

The **copula dependence model** is the load-bearing modelling choice for integrated aggregation. A Gaussian copula understates tail dependence (independent extreme losses across silos); a Student-t copula captures tail dependence (extreme losses across silos cluster); an Archimedean copula (Clayton, Gumbel) gives asymmetric tail dependence. The copula choice determines whether the diversification benefit is realistic or optimistic; calibration is typically done from joint loss data at silo level (sparse, hence model-risk-heavy). McNeil treats copula depth in Ch.7 and recommends conservative copula choices (heavy-tail dependence) for integrated aggregation. **Source:** McNeil et al. (2015) Ch.8 pp.299-310 + Ch.7 pp.220-274.

The **operational-risk silo overlay** has special structure: operational losses are typically **less correlated** with market and credit losses than market and credit are with each other. Internal-fraud losses, system-failure losses, and execution-delivery losses are driven by firm-internal events that rarely coincide with market crashes or credit-event clusters. The operational silo therefore contributes a large diversification benefit in integrated aggregation — the firm-wide capital is much smaller than the sum of market + credit + operational silo capitals because the operational tail is approximately independent. **Source:** McNeil et al. (2015) Ch.13 pp.503-512 + Ch.8 pp.299-310.

The **Euler allocation's structural property** is that it is the unique additive allocation consistent with the marginal-cost interpretation. Alternative allocations (proportional to standalone capital, proportional to expected loss, equal-share) preserve additivity but lose the marginal interpretation; non-additive allocations (covariance-based, beta-weighted) preserve a marginal interpretation but break additivity. Euler is the unique satisfier of both, which is why it is the canonical choice for performance measurement and risk-adjusted-return computation. **Source:** McNeil et al. (2015) Ch.8 pp.310-322.

A subtle structural point: the **Euler allocation can produce negative contributions for silos that hedge other silos**. If silo A is short a position that silo B is long, then increasing silo A's scale reduces the firm's net exposure and reduces firm-wide capital. The Euler partial derivative `∂ρ(L_total) / ∂λ_A < 0`, so silo A's allocation is negative. The economic interpretation is that silo A is a **hedging silo** earning a negative capital cost (it is paid for hedging the firm). The negative Euler allocation is the mathematical formalisation of this hedging-benefit framing. **Source:** McNeil et al. (2015) Ch.8 pp.310-322.

The **integrated-aggregation operational reality** is more complex than the clean math suggests. Real firms have silo-level risk measures computed by different teams with different models, different calibration windows, and different reporting conventions. Constructing a fully-integrated `L_total` requires either (a) re-running all silo models under a common framework (expensive and rare) or (b) aggregating silo-level capital numbers with an estimated diversification benefit (typical practice). Practice (b) is the modular-with-diversification-adjustment approach, which is the closest operationally-feasible approximation to integrated aggregation. The integrated approach is the theoretical reference; the firm's operational reality is typically a calibrated version of the modular approach. **Source:** McNeil et al. (2015) Ch.8 pp.299-322.

## See Also

Within v11 Risk Management:

- [rm-risk-measure-axioms](./rm-risk-measure-axioms.md) — Batch-0 card on the coherence axioms (especially subadditivity) that this aggregation framework rests on.
- [rm-operational-risk-basics](./rm-operational-risk-basics.md) — Batch-3 sibling card on the operational-risk silo whose loss distribution feeds this aggregation.
- [rm-credit-var-portfolio](./rm-credit-var-portfolio.md) — Batch-3 sibling card on the credit-risk silo.
- [rm-portfolio-xva-aggregation](./rm-portfolio-xva-aggregation.md) — Batch-3 sibling card; the Euler allocation introduced here is reused for per-trade XVA attribution.
- `rm-euler-capital-allocation` (McNeil-Frey-Embrechts (2015) QRM, pp.338) — deepening that extends this card.
- `rm-risk-aggregation-frechet-bounds` (McNeil-Frey-Embrechts (2015) QRM, pp.324) — deepening that extends this card.
- `rm-component-var-euler-risk-budgeting` (Hull (2023) RMFI, pp.276) — deepening that adds a supporting source to this card.
- `rm-tail-dependence-extreme-correlations` (Bouchaud-Potters (2003) Theory of Financial Risk, pp.213) — deepening that adds a supporting source to this card.
- `rm-sample-covariance-distortion-highdim` (Potters-Bouchaud (2020) A First Course in RMT, pp.290) — deepening that extends this card.
- `rm-rie-minimizes-out-of-sample-risk` (Potters-Bouchaud (2020) A First Course in RMT, pp.349) — deepening that adds a supporting source to this card.

## Escalate to Raw When

The conceptual depth in this card stops at the modular / integrated dichotomy + the subadditivity diversification framing + Euler allocation overview. When the operator needs the full Fréchet-problem treatment of risk aggregation under unspecified dependence, the formal copula-construction depth (Sklar's theorem, Archimedean families, tail-dependence coefficients, copula calibration from joint loss data), or the operational-implementation depth (silo-level model harmonisation, common-scenario aggregation, regulatory-capital reconciliation), open McNeil Ch.7 pp.220-274 + Ch.8 §8.4-§8.5 pp.299-322 directly. **Source:** McNeil et al. (2015) Ch.7 + Ch.8 pp.220-322.
