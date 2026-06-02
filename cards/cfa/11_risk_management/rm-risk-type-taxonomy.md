---
schema_version: "cacg.v0"
id: "rm-risk-type-taxonomy"
title: "Risk-Type Taxonomy — Market / Credit / Liquidity / Operational / Business / Regulatory"
reading_id: "11_risk_management"
summary: "Six-class taxonomy of financial risk (market / credit / liquidity / operational / business / regulatory) with the loss-distribution mapping framework per McNeil Ch.1 §1.1.2 + Ch.2 §2.1.2; quantifiable types feed coherent risk measures while judgmental types feed scenario analysis."
tags: ["risk-management", "risk-type"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p065:0087"
    chunk_hash: "1f32243e191e47b1520d192ba70648cb1768478f3490e56a1f5a923d184c4b5a"
    page_range: [65, 66]
    quote: "An obvious source of risk for a bank is a decrease in the value of its investments on the asset side of the balance sheet."
    edge_type: "supports"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p025:0021"
    chunk_hash: "0330f7f634eb6339c2af8c123daf297cc465ab5e288a50ae9cb57f7f0bf8f33e"
    page_range: [26, 26]
    quote: "The best-known type of risk is probably market risk: the risk of a change in the value of a financial position or portfolio due to changes in the value of the underlying components"
    edge_type: "defines"
  - source_id: "rm_gregory_2020_xva_challenge"
    chunk_id: "rm_gregory_2020_xva_challenge:p060:0080"
    chunk_hash: "0081e33b86dbe88d343f118f8cd82aaf55b8349397714d275a942a953c305af0"
    page_range: [61, 61]
    quote: "Only one party takes lending risk. A bondholder takes considerable credit risk, but an issuer of a bond does not face a loss if the buyer of the bond defaults."
    edge_type: "supports"
card_hash: "73b9b7157ce9fe319f0c06bea0576c26cd2a338acd2c245d701e1899eb898fa3"
---
# Risk-Type Taxonomy — Market / Credit / Liquidity / Operational / Business / Regulatory

## Intuition

Quantitative risk management organises financial risk into a small set of **types** so that measurement, capital, and governance can be modular. The McNeil-Frey-Embrechts taxonomy distinguishes **six** principal types: **market risk** (loss from movements in market prices / rates / vols), **credit risk** (loss from a counterparty failing to meet contractual obligations), **liquidity risk** (loss from inability to transact at quoted prices or to fund positions), **operational risk** (loss from failed processes, people, systems, or external events), **business risk** (loss from strategic / competitive / macroeconomic environment), and **regulatory risk** (loss from changes in legal / regulatory framework). **Source:** McNeil et al. (2015) Ch.1 pp.5-7 + Ch.2 pp.44-46.

The taxonomy is not academic — it determines who measures the risk (front-office desks for market, credit-risk function for counterparty, treasury for liquidity, ops-risk function for events), how it is modelled (loss-distribution math for market/credit/operational; mostly judgmental for business/regulatory), and how regulatory capital is sized (Basel pillars allocate capital by type). The vertical's Batch 0 axiomatic and loss-distribution cards apply to **all** quantifiable types in identical form — the difference is in the **risk-factor mapping** that feeds the loss distribution. **Source:** McNeil et al. (2015) Ch.1 pp.5-7.

A critical distinction is **quantifiable** vs **judgmental** types. Market, credit, and operational risk admit explicit loss distributions and feed regulatory capital formulas (`ρ(L_market) + ρ(L_credit) + ρ(L_op)`). Liquidity risk straddles the boundary — funding liquidity admits scenario-based limits while market liquidity is harder to summarise as a loss CDF. Business and regulatory risk are intrinsically judgmental and feed scenario / stress tests rather than capital formulas. Firm-wide aggregation across the quantifiable silos is the subject of `[[rm-integrated-firm-wide-risk-aggregation]]`. **Source:** McNeil et al. (2015) Ch.2 pp.44-46.

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

A **risk type** is a partition class of the firm's loss random variable `L_total` based on the **causal channel** of the loss. McNeil-Frey-Embrechts define six principal classes; the loss decomposition is `L_total = L_market + L_credit + L_liquidity + L_op + L_business + L_regulatory` where each summand is the loss attributable to the corresponding channel. **Source:** McNeil et al. (2015) Ch.1 pp.5-7 + Ch.2 pp.44-46.

**Market risk** is the loss from changes in the value of positions arising from movements in market risk factors `X` (prices, rates, vols, spreads): **Source:** McNeil et al. (2015) Ch.2 pp.45.

```
L_market = −(V(X_1) − V(X_0))     (factor-mapping convention)
```

where `V` is the portfolio value function and `X_t` is the factor vector at time `t`. Subtypes: **equity / interest-rate / FX / commodity / volatility / basis risk**, by which factor in `X` drives the loss. **Source:** McNeil et al. (2015) Ch.2 pp.45.

**Credit risk** is the loss from a counterparty's failure to perform on a contractual obligation. The canonical decomposition is: **Source:** McNeil et al. (2015) Ch.2 pp.45-46.

```
L_credit  =  EAD · LGD · 1_{default}     (single-name, single-period)
```

where `EAD` is exposure at default, `LGD` is loss given default (`1 − recovery`), and `1_{default}` is the default indicator. Subtypes: **default risk** (binary event), **spread risk** (mark-to-market on rating migration), **counterparty risk** (default by derivatives counterparty), **wrong-way risk** (correlation between exposure and default). **Source:** McNeil et al. (2015) Ch.2 pp.45-46.

**Liquidity risk** splits into **market liquidity risk** (cost of trading: bid-ask spread, market-impact, price-decay during unwind) and **funding liquidity risk** (inability to roll funding or post collateral). **Source:** McNeil et al. (2015) Ch.2 pp.46.

**Operational risk** is the loss from inadequate / failed internal processes, people, systems, or external events. Basel II classifies operational events into seven event types (internal fraud, external fraud, employment practices, clients/products/business practices, damage to physical assets, business disruption, execution/delivery/process management); see `[[rm-operational-risk-basics]]` for the LDA frequency-severity machinery. **Source:** McNeil et al. (2015) Ch.1 pp.6-7.

**Business risk** is the loss from strategic / competitive / macroeconomic environment changes — not directly attributable to a market move, credit event, or operational failure. **Regulatory risk** is the loss from changes in legal / regulatory framework. Both are typically managed via scenario analysis rather than quantitative loss-distribution math. **Source:** McNeil et al. (2015) Ch.1 pp.6.

## Mathematical Reasoning

Every risk type that admits a loss random variable feeds the same axiomatic apparatus from `[[rm-risk-measure-axioms]]`: choose a coherent risk measure `ρ` (typically `ES_α`), apply it to the type-specific loss `L_type`, and obtain a capital number `ρ(L_type)`. The mathematical machinery is identical across types; what differs is the **risk-factor mapping** that defines `L_type`. Market risk uses factor-mapping calculus (`L = −ΔV(X)`); credit risk uses default-LGD decomposition; operational risk uses the loss-distribution approach (LDA: frequency `N` × severity `X_i`). **Source:** McNeil et al. (2015) Ch.2 pp.44-46.

The taxonomy provides a **clean partition** of `L_total` only under a **causal-attribution convention**: each individual loss event is attributed to the dominant channel. In practice the partition is fuzzy at the edges — a counterparty default that drops a hedge position is partly credit (the default) and partly market (the consequent unhedged exposure); a flash-crash unwind that costs more than expected is partly market (the price move) and partly liquidity (the unwind friction); a rogue-trader loss is partly operational (the failed control) and partly market (the position taken). Industry practice is to attribute the **first-order channel** and capture the residual via correlation adjustments in aggregation. **Source:** McNeil et al. (2015) Ch.2 pp.45-46.

The aggregation step `L_total = Σ L_type` raises a coherence-relevant question: should the firm-wide capital be the **sum** of type-level capitals `Σ ρ(L_type)`, or the **measure applied to the sum** `ρ(L_total)`? Under subadditivity (which `ρ = ES_α` satisfies — see `[[rm-risk-measure-axioms]]`), `ρ(Σ L_type) ≤ Σ ρ(L_type)`. The inequality is strict whenever the type-level losses are not comonotone — i.e., whenever a bad market event does not coincide with a bad credit event with probability 1. The **diversification gap** between the modular sum and the integrated measure is the firm-wide diversification benefit, which `[[rm-integrated-firm-wide-risk-aggregation]]` quantifies via a copula on the type-level losses. **Source:** McNeil et al. (2015) Ch.2 pp.45-46.

The **quantifiable vs judgmental** distinction has a structural reason: market, credit, and operational risk have abundant **loss-data** (P&L histories, default histories, operational-event databases) to fit loss-distribution parameters. Business and regulatory risk have **rare events with idiosyncratic causes** — a competitor's product launch, a sovereign regime change, a new capital rule — for which a frequentist loss distribution either does not exist or is so heavily Bayesian-prior-dominated that the loss-distribution approach degenerates into scenario analysis. The vertical's Batch 2 scenario / stress cards (`[[rm-scenario-analysis]]`, `[[rm-stress-testing]]`) are the canonical tools for the judgmental types. **Source:** McNeil et al. (2015) Ch.1 pp.5-7.

Boundary discipline matters: liquidity-risk depth past the bid-ask / funding-cost framing defers to a future microstructure plan; the **conduct-risk and ethics-flavored** subtype of operational risk (rogue trading, insider dealing, market abuse) belongs to the 17 ethics vertical, not here. The v11 vertical scope covers the **quantifiable** types at McNeil's depth plus the L1 risk-objectives bridge — see the `_source_role_map.md` boundary table for the full out-of-scope list. **Source:** McNeil et al. (2015) Ch.2 pp.44-46.

## See Also

- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — the loss-variable convention `L = −ΔV` applied uniformly across types.
- [rm-integrated-firm-wide-risk-aggregation](./rm-integrated-firm-wide-risk-aggregation.md) — Batch-3 card developing the modular-vs-integrated aggregation contrast and the copula-based diversification quantification.
- [rm-operational-risk-basics](./rm-operational-risk-basics.md) — Batch-3 card developing the operational-risk loss-distribution approach (LDA frequency × severity model).
- `rm-counterparty-vs-lending-risk` (Gregory (2020) The xVA Challenge, pp.61) — deepening that adds a supporting source to this card.

## Escalate to Raw When

The conceptual depth in this card stops at the six-type partition + the quantifiable-vs-judgmental distinction. When the operator needs the regulatory-framework depth (Basel III pillar 1 / 2 / 3 mechanics, FRTB market-risk internal-model rules, IRB credit-risk parameter floors, AMA / SMA operational-risk capital methodologies, or the conduct-risk / culture-of-compliance taxonomy that belongs to subcorpus 17), open McNeil Ch.1 §1.1.2 + Ch.2 §2.1 directly (pp.5-15 + pp.44-50) and consult the relevant Basel committee text. **Source:** McNeil et al. (2015) Ch.1 pp.5-15 + Ch.2 pp.44-50.
