---
schema_version: "cacg.v0"
id: "rm-operational-risk-basics"
title: "Operational Risk Basics — McNeil Ch.13 §13.1"
reading_id: "11_risk_management"
summary: "Operational risk is the loss class from inadequate / failed internal processes, people, systems, or external events; McNeil Ch.13 §13.1 introduces the Basel BIA / SA / AM tiered measurement approaches with the seven event-type × eight business-line cell taxonomy."
tags: ["risk-management", "operational-risk"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p525:0766"
    chunk_hash: "3b278bbf3e5ed8178596b26b7e5e5041fed1ed8ca251c3bf990b01ff392bccba"
    page_range: [525, 526]
    quote: "An essential difference between operational risk, on the one hand, and market and credit risk, on the other, is that operational risk has no upside for a bank."
    edge_type: "defines"
card_hash: "d672956c9ee5fd5a65d8137040fb16a889864ce2ee7d3082e2b2ce2c7789962a"
---
# Operational Risk Basics — McNeil Ch.13 §13.1

## Intuition

**Operational risk** is the loss from inadequate / failed internal processes, people, systems, or external events. It is structurally different from market and credit risk because the losses are driven by **event types** rather than factor moves: internal fraud, external fraud, employment practices, clients-products-business-practices, damage to physical assets, business disruption, and execution-delivery-process management. Each event type has its own loss-data generating process, and the aggregate operational loss is the sum over all event types and business lines. **Source:** McNeil et al. (2015) Ch.13 pp.503-507.

The measurement framework spans three regulatory tiers of sophistication. **Basic-indicator approach (BIA)** assigns operational-risk capital as a fixed fraction of gross income — simple, audit-friendly, blind to actual loss experience. **Standardised approach (SA)** disaggregates the BIA across business lines with different multipliers — capturing business-mix differences but still loss-data-blind. **Advanced measurement approach (AMA)** is loss-data-driven: the firm estimates a frequency × severity model per event-type × business-line cell and aggregates into a portfolio operational-loss distribution; capital is the high-quantile of that distribution. AMA dominates the other two for capital efficiency at the cost of substantially more data and modelling investment. **Source:** McNeil et al. (2015) Ch.13 pp.507-509.

The **loss-data taxonomy** is the canonical Basel-II 7-event-type × business-line matrix. Each cell holds an internal loss history `{L_t}` plus (typically) an external loss-data overlay for events the firm has not experienced but peer firms have. AMA aggregates the per-cell distributions into a firm-wide operational loss distribution using either a copula or a single-factor common shock. The aggregation is conceptually identical to portfolio credit-VaR (see `[[rm-credit-var-portfolio]]`) but with event types in place of obligors and frequency × severity in place of PD × LGD × EAD. **Source:** McNeil et al. (2015) Ch.13 pp.503-509.

```
   Operational risk measurement landscape
   ──────────────────────────────────────

   Tier   Method                       Loss-data driven?    Capital sensitivity
   ────────────────────────────────────────────────────────────────────────────
   BIA    capital = fixed-fraction      no                  blind to actual loss
          of gross income                                    experience
   ────────────────────────────────────────────────────────────────────────────
   SA     per-business-line variant   no                business-mix sensitive
          of BIA                                          but loss-blind
   ────────────────────────────────────────────────────────────────────────────
   AMA    frequency × severity model    yes                 fully loss-sensitive
          per (event-type, business-                         (data-intensive)
          line) cell; aggregate via
          copula or common factor

   Basel-II 7 event types:
     1. internal fraud
     2. external fraud
     3. employment practices
     4. clients / products / business practices
     5. damage to physical assets
     6. business disruption + system failures
     7. execution, delivery + process management

   AMA capital  =  q_α ( aggregate operational loss distribution )
                   (α set by regulator; loss-distribution depth
                    in [[rm-operational-risk-quantification]])
```

## Definition

**Operational risk** is the risk of loss arising from inadequate or failed internal processes, people, systems, or external events. The canonical Basel-II event-type taxonomy partitions operational losses into seven classes: internal fraud, external fraud, employment practices and workplace safety, clients-products-business-practices, damage to physical assets, business disruption and system failures, and execution-delivery-process management. **Source:** McNeil et al. (2015) Ch.13 pp.503-507.

The three regulatory measurement approaches are: **Source:** McNeil et al. (2015) Ch.13 pp.507-509.

```
Basic-Indicator Approach (BIA):
    K_BIA  =  α_BIA · max(GI, 0)      averaged over a multi-year window
    GI = gross income
    α_BIA = fixed regulatory factor

Standardised Approach (SA):
    K_SA  =  Σ_b  β_b · max(GI_b, 0)   summed across business lines b
    GI_b = gross income per business line
    β_b  = per-business-line regulatory factor

Advanced Measurement Approach (AMA):
    K_AMA  =  q_α ( L_op )            high quantile of internal loss dist
    L_op   =  Σ_(c)  Σ_(t)  L_{c,t}   summed across cells c and time t
    cell c = (event type, business line)
    L_{c,t} = single loss event in cell c at time t
```

The **AMA frequency × severity decomposition** within each cell `c` is: **Source:** McNeil et al. (2015) Ch.13 pp.508-509.

```
N_c  ~  F_freq(λ_c)            (event count in cell c; e.g., Poisson(λ_c))
X_{c,n}  ~  F_sev(θ_c)  i.i.d.         (severity of n-th event in cell c)
L_c  =  Σ_n  X_{c,n}                 (aggregate loss in cell c, a compound sum)
```

The cell-level aggregate loss `L_c` is a **compound random variable** (random sum of random severities); its distribution is treated in `[[rm-operational-risk-quantification]]` via Panjer recursion + EVT-tail intuition. **Source:** McNeil et al. (2015) Ch.13 pp.508-509.

## Mathematical Reasoning

The structural difference between operational risk and market / credit risk is the **lack of a continuous factor mapping**. Market risk losses come from factor moves with well-defined sensitivities (deltas, gammas); credit risk losses come from default events with model-derived PDs. Operational risk losses come from discrete events whose drivers are organisation-specific, often not modellable from external data, and frequently linked to one-time governance / control failures rather than recurring market processes. This forces a **loss-data-driven** rather than a **factor-model-driven** quantification approach. **Source:** McNeil et al. (2015) Ch.13 pp.503-507.

The **AMA frequency × severity decomposition** is the workhorse construction. Frequency is typically modelled as Poisson (single-rate parameter `λ_c` per cell) or negative binomial (allowing for over-dispersion); severity is typically modelled as a heavy-tailed distribution (log-normal, generalised Pareto, Weibull) fit to historical loss magnitudes per cell. The compound sum `L_c = Σ_n X_{c,n}` has a known distribution under specific frequency-severity choices (compound Poisson + exponential severity gives a Tweedie distribution; compound Poisson + GPD severity gives an EVT-flavoured aggregate) and admits the **Panjer recursion** for computing the aggregate distribution when frequency is in the (a, b)-class. **Source:** McNeil et al. (2015) Ch.13 pp.507-509.

The **per-cell loss distributions must be aggregated** into a firm-wide operational loss distribution. The aggregation step is where dependence assumptions enter. Independent cells give a diversification benefit (firm-wide tail is thinner than the sum of cell tails); positively-dependent cells give a less diversifying aggregate. The dependence model is typically a **copula** linking cell-level cumulative distributions, with a Gaussian or t copula as the default choice. The copula tail dependence determines whether multiple operational-loss events tend to cluster (e.g., a single governance failure triggers events across multiple cells). **Source:** McNeil et al. (2015) Ch.13 pp.508-509.

A subtle structural point: **operational risk is more episodic than continuous**. Internal-fraud events are rare but large; execution-delivery events are frequent but small. The frequency × severity decomposition captures this differently in each cell — high-`λ_c` low-severity cells dominate the body of the loss distribution while low-`λ_c` high-severity cells dominate the tail. AMA capital is determined almost entirely by the tail-driving cells (rare-large events), which are also the least data-rich (few observations to calibrate). This is the structural reason external-loss-data overlays and expert-judgment scenarios are essential in serious AMA implementations. **Source:** McNeil et al. (2015) Ch.13 pp.508-509.

The **regulatory-implementation boundary** matters. McNeil's conceptual landscape is the BIA / SA / AMA progression: simple gross-income proxies at one end, loss-data-driven internal models at the other. Current operational-risk capital rules, including standardised measurement variants and their calibration schedules, belong to authorised regulatory text rather than to this card. **Source:** McNeil et al. (2015) Ch.13 pp.507-509.

## See Also

Within v11 Risk Management:

- [rm-risk-type-taxonomy](./rm-risk-type-taxonomy.md) — Batch-0 card on the broader risk-type partition; operational risk is one of the six types.
- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — Batch-0 card on the loss-distribution machinery that AMA's compound sum feeds.
- [rm-integrated-firm-wide-risk-aggregation](./rm-integrated-firm-wide-risk-aggregation.md) — Batch-3 sibling card on firm-wide aggregation; operational risk is one silo in the firm-wide tree.
- [rm-operational-risk-quantification](./rm-operational-risk-quantification.md) — Batch-3 sibling card on the compound-sum + Panjer recursion + EVT-tail machinery for AMA.

## Escalate to Raw When

The conceptual depth in this card stops at the BIA / SA / AMA taxonomy + frequency × severity decomposition + dependence-aggregation overview. When the operator needs the full insurance-actuarial methodology (compound-Poisson conjugacy, Panjer recursion implementation, EVT-tail GPD fitting for operational severities), Basel operational-risk regulatory implementation depth, or the depth on external-loss-data consortium pooling (ORX-style data sharing), open McNeil Ch.13 §13.2-§13.5 pp.509-536 directly. Specific regulatory schedules belong to authorized regulatory text. **Source:** McNeil et al. (2015) Ch.13 pp.503-536.
