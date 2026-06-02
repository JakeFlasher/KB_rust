---
schema_version: "cacg.v0"
id: "rm-credit-risk-metrics-restatement"
title: "Credit Risk Metrics Restatement — McNeil Ch.10 §10.1 + §10.1.5"
reading_id: "11_risk_management"
summary: "Restates the EAD / PD / LGD credit-risk metrics at the portfolio-aggregation level: each metric is one of the three dependent inputs to the IRB-formula and to the portfolio loss distribution L_portfolio = Σ_i EAD_i · LGD_i · 1_{default_i}."
tags: ["risk-management", "credit-risk"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p393:0562"
    chunk_hash: "0368720885427c89b930c2a22ba181c85eb507a9978860e7bf38358cf5c285cd"
    page_range: [393, 393]
    quote: "They are key inputs to the Basel formula in the internal-ratings-based (IRB) approach to determining capital requirements for credit-risky portfolios, so it is important to consider them."
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p388:0554"
    chunk_hash: "096068a32cafde7758253a71bcca61aa7531726028093b8ef9abe148d1f71e6f"
    page_range: [388, 388]
    quote: "10.1 Credit-Risky Instruments In this section we give an overview of the universe of credit-risky instruments, starting with the simplest examples of loans and bonds."
    edge_type: "supports"
card_hash: "d9dd2bbd2571fc3f5afdf0c55f80d182a63c449c9f118516a114a408f87cadab"
---
# Credit Risk Metrics Restatement — McNeil Ch.10 §10.1 + §10.1.5

## Intuition

The three foundational credit-risk metrics — **PD** (probability of default), **LGD** (loss given default), and **EAD** (exposure at default) — get a different framing in the risk-management vertical than in the fixed-income vertical. In **06 Fixed Income**, PD / LGD / EAD are derived from single-counterparty structural models (Merton, Lando hazard-rate); each metric is computed for one obligor and one period. In **11 Risk Management**, the three metrics are **inputs to a portfolio loss-distribution aggregation**: the firm holds many counterparties, each with its own (PD_i, LGD_i, EAD_i), and the question is what the aggregate loss distribution looks like across the portfolio. **Source:** McNeil et al. (2015) Ch.10 pp.367-374.

The canonical single-counterparty single-period loss decomposition is `L_i = EAD_i · LGD_i · 1_{default_i}`. The first-moment summary (expected loss) is `E[L_i] = EAD_i · E[LGD_i] · PD_i`, which is the **expected credit loss** that drives loan-loss provisions and pricing. The risk-management interest is in the **distribution** of `L_i` and especially in the portfolio aggregation `L_portfolio = Σ_i L_i`, whose distribution captures default correlation, exposure concentration, and recovery uncertainty in a way that the per-counterparty expectations cannot. **Source:** McNeil et al. (2015) Ch.10 pp.372-374.

The boundary against 06 is precise. **06 owns** the single-counterparty derivations: Merton-Lando structural-model PD from asset volatility + default threshold, hazard-rate intensity calibration, rating-migration matrices, single-name CDS pricing, and the term-structure of default for a single obligor. **11 owns** the portfolio-aggregation layer: how `{L_i}` aggregates to `L_portfolio`, how default correlation enters (threshold models, mixture models), and how the portfolio's tail distribution differs from the sum of marginals. The same PD / LGD / EAD metrics appear in both, but the framing differs by aggregation level. **Source:** McNeil et al. (2015) Ch.10 pp.367-374.

```
   Credit-risk metrics — single-counterparty to portfolio
   ──────────────────────────────────────────────────────

   single counterparty:
     +-----+    PD_i = P(default in horizon Δt)         <- 06 derivation
     | i   |    LGD_i = 1 - recovery rate                  (Merton-Lando,
     +-----+    EAD_i = exposure if default occurs         hazard-rate)
        |
        v
     L_i  =  EAD_i · LGD_i · 1_{default_i}          (single-period, single-name)
     E[L_i]  =  EAD_i · E[LGD_i] · PD_i             (expected credit loss)

   portfolio aggregation:                              <- 11 layer
     +-------------------+
     | L_portfolio       |
     | = Σ_i L_i         |        (sum across all obligors in book)
     +---------+---------+
               |
               v
     full distribution:  F_{L_portfolio}(l) = P(L_portfolio ≤ l)
       depends on:
         - marginal distributions of L_i (from 06)
         - joint default dependence (threshold model / mixture model)
         - exposure concentration (large EAD_i for few counterparties)
         - recovery uncertainty (LGD_i is itself random)

     [[rm-credit-var-portfolio]] develops the joint-distribution + portfolio-VaR
```

## Definition

For counterparty `i` in the firm's portfolio, the **single-counterparty single-period credit loss** is: **Source:** McNeil et al. (2015) Ch.10 pp.372-374.

```
L_i  =  EAD_i  ·  LGD_i  ·  1_{default_i}

where:
  EAD_i        =  exposure at default (notional or marked-to-market exposure)
  LGD_i        =  loss given default = 1 - recovery rate (typically random)
  1_{default_i} =  default indicator over the horizon (1 if default, else 0)
  PD_i         =  P( default_i  occurs in horizon Δt )
```

The **expected credit loss** per counterparty is the product of the first moments: **Source:** McNeil et al. (2015) Ch.10 pp.372-374.

```
E[L_i]  =  EAD_i  ·  E[LGD_i]  ·  PD_i        (under LGD-default independence)
```

The **portfolio credit loss** aggregates across all counterparties: **Source:** McNeil et al. (2015) Ch.10 pp.367-372.

```
L_portfolio  =  Σ_i  L_i  =  Σ_i  EAD_i · LGD_i · 1_{default_i}
```

The portfolio loss distribution `F_{L_portfolio}` is the central object the risk-management vertical analyses. It depends on (a) the marginal distributions of each `L_i` (from per-counterparty 06 modelling), (b) the joint dependence structure across `{1_{default_i}}` (threshold / mixture models), (c) the exposure concentration profile across counterparties, and (d) the joint distribution of `{LGD_i}` (recovery-rate uncertainty often correlated with default). **Source:** McNeil et al. (2015) Ch.10 pp.367-374 + Ch.11.

## Mathematical Reasoning

The **product-form decomposition** `L_i = EAD_i · LGD_i · 1_{default_i}` is the standard simplification but rests on the assumption that the three components are **independent or sufficiently approximated as independent** for the first-moment summary. In practice, all three exhibit correlations: defaults cluster in stressed periods, LGD rises during stressed defaults ("wrong-way recovery"), and EAD can rise during stressed periods if mark-to-market derivative exposures move adversely. The expected-loss formula `E[L_i] = EAD_i · E[LGD_i] · PD_i` ignores these correlations; the higher-moment / tail analysis must restore them. McNeil treats the wrong-way correlations in Ch.10 §10.1.5 and Ch.17. **Source:** McNeil et al. (2015) Ch.10 pp.372-374 + Ch.17.

The **portfolio aggregation `L_portfolio = Σ_i L_i`** has very different distribution properties from the sum of marginal expectations. Under **independent** defaults, the law of large numbers smooths the aggregate and reduces relative dispersion as portfolio size grows — diversification benefit accrues. Under **correlated** defaults (driven by common macroeconomic factors), the aggregate retains tail thickness even as portfolio size grows — the asymptotic limit is non-degenerate. This is the fundamental insight that justifies the dedicated portfolio-credit-VaR machinery (see `[[rm-credit-var-portfolio]]`): independent defaults give diversifiable risk, correlated defaults give systematic risk that survives aggregation. **Source:** McNeil et al. (2015) Ch.10 pp.367-372 + Ch.11.

The **exposure concentration** dimension is operationally critical. A portfolio with `N` counterparties of equal `EAD` has very different tail behavior from a portfolio with one large counterparty and many small ones — even if all share the same PD. The **largest-EAD counterparty** dominates the portfolio tail because its single default produces a loss that no diversification can absorb. Regulatory and internal-control regimes typically impose **single-name exposure limits** (no counterparty should exceed a fixed fraction of total firm exposure) to bound the concentration risk that EAD aggregation otherwise produces. **Source:** McNeil et al. (2015) Ch.10 pp.367-374.

The **recovery-rate uncertainty** is the most under-appreciated of the three. The standard expected-loss formula treats `LGD_i` as a fixed fraction, but realised recoveries vary substantially across defaults and across recovery-process regimes. Distribution-level analysis must treat `LGD_i` as random (typically modelled as a beta distribution on `[0, 1]`) and aggregate across counterparties accordingly. Under **wrong-way recovery** (recoveries fall in the same regimes when defaults cluster — typically driven by collateral asset values declining during stress), the portfolio tail is fatter than under independent-recovery assumptions. **Source:** McNeil et al. (2015) Ch.10 pp.372-374.

A subtle structural point: **PD is forward-looking, LGD is backward-looking**. PD is typically inferred from market prices (CDS spreads, bond yields) or historical default rates conditioned on rating; LGD is typically estimated from historical recovery data on resolved defaults. The asymmetry means PD reflects current market expectations (which can swing sharply in stressed periods) while LGD reflects long-run historical averages (which are slow-moving). Portfolio risk under stress can be under-estimated if both metrics are taken at face value without conditioning on the current state. **Source:** McNeil et al. (2015) Ch.10 pp.367-374.

The boundary with 06 is summarised: 06 owns the per-obligor derivation pipeline (Merton equity-as-call PD inference, Lando intensity calibration, rating-migration matrix construction, single-name CDS spread decomposition); 11 owns the cross-obligor aggregation pipeline (default correlation modelling, portfolio-VaR construction, large-portfolio asymptotics, capital allocation by Euler principle). The two cards `[[rm-credit-var-portfolio]]` and `[[rm-portfolio-xva-aggregation]]` develop the aggregation layer in detail. **Source:** McNeil et al. (2015) Ch.10 pp.367-374.

## See Also

Cross-vertical (Fixed Income — single-counterparty derivation territory):

- [fi-credit-risk-fundamentals](../06_fixed_income_and_credit/fi-credit-risk-fundamentals.md) — single-counterparty credit-risk fundamentals.

For the structural and intensity-model derivations of PD, see also [fi-default-models-and-recovery](../06_fixed_income_and_credit/fi-default-models-and-recovery.md) for the Merton-Lando structural and hazard-rate intensity models. For single-name CDS pricing, see [fi-cds-basics](../06_fixed_income_and_credit/fi-cds-basics.md).

Within v11 Risk Management:

- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — Batch-0 card on the loss-distribution machinery that the credit metrics feed.
- [rm-risk-type-taxonomy](./rm-risk-type-taxonomy.md) — Batch-0 card on the credit-risk type.
- [rm-credit-var-portfolio](./rm-credit-var-portfolio.md) — Batch-3 sibling card developing portfolio credit-VaR with threshold and mixture models.

## Escalate to Raw When

The conceptual depth in this card stops at the PD / LGD / EAD restatement + portfolio-aggregation framing + the wrong-way-recovery / concentration / forward-vs-backward asymmetry caveats. When the operator needs the full single-counterparty derivation (Merton equity-as-call PD inference, Lando hazard-rate intensity calibration, rating-migration matrix construction, term-structure of default for a single obligor), open the 06 Fixed Income cards above OR McNeil Ch.10 §10.2-§10.4 pp.379-424 directly. **Source:** McNeil et al. (2015) Ch.10 pp.367-424.
