---
schema_version: "cacg.v0"
id: "rm-portfolio-xva-aggregation"
title: "Portfolio XVA Aggregation — McNeil Ch.17 §17.2-§17.3 + Crepey-Bielecki-Brigo Ch.1-2"
reading_id: "11_risk_management"
summary: "Portfolio-level aggregation of CVA / DVA / FVA / KVA via netting-set exposures and Euler-principle capital allocation; legacy section anchors are McNeil Ch.17 §17.2-§17.3 (counterparty risk management + dynamic portfolio credit models) supported by Crepey-Bielecki-Brigo's TVA-and-funding framing and McNeil Ch.12 portfolio credit derivatives context."
tags: ["risk-management", "portfolio-xva"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p624:0908"
    chunk_hash: "d1d14a9330aa18c54366db55120f6fd4d2c4ae238f54fb017aa642e767f613f3"
    page_range: [624, 625]
    quote: "A substantial proportion of all derivative transactions are carried out OTC, so that counterparty credit risk is a key issue for financial institutions."
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p627:0912"
    chunk_hash: "3a799a60098230e04b4fcf77ba2525298fd1813cf56e710f3f3fcc0c2a8f248d"
    page_range: [627, 627]
    quote: "The CVA in (17.7) reflects the potential loss incurred by B due to a premature default of S; the debt value adjustment, or DVA, in (17.8) reflects the potential loss"
    edge_type: "defines"
  - source_id: "fi_crepey_bielecki_brigo_2014_counterparty_risk_funding"
    chunk_id: "fi_crepey_bielecki_brigo_2014_counterparty_risk_funding:p018:0015"
    chunk_hash: "647cf5d8726e26fb36f353288f363651e6dc6346c6c29118a6bf8b33440e57aa"
    page_range: [18, 18]
    quote: "We will call total valuation adjustment (TVA) the aggregate value of all the adjustments which are required in order to account for bilateral counterparty risk under funding constraints."
    edge_type: "supports"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p498:0723"
    chunk_hash: "4021c5404eec12d378781039b87159cfd7f46ecf22b96063b0f74ddf924f44d4"
    page_range: [498, 499]
    quote: "A CDO is a financial instrument for the securitization of a portfolio of credit products such as bonds, loans or mortgages."
    edge_type: "supports"
card_hash: "0b3449e6f7d2a153b704448d89eabb6952da4e8df8ba5a70d0d88d7f8a3985bb"
---
# Portfolio XVA Aggregation — McNeil Ch.17 §17.2-§17.3 + Crepey-Bielecki-Brigo Ch.1-2

## Intuition

**XVA** is the family of valuation adjustments applied to derivative positions to reflect non-default-free counterparty and funding effects. The principal members: **CVA** (credit valuation adjustment, for counterparty default risk on the firm's positive exposure), **DVA** (debt valuation adjustment, for the firm's own default risk on its negative exposure), **FVA** (funding valuation adjustment, for the cost of funding uncollateralised exposure), and **KVA** (capital valuation adjustment, for the cost of regulatory capital required to support the position). The single-counterparty single-trade derivation of each XVA belongs to 06 (`fi-counterparty-risk-cva.md`); the 11 vertical owns the **portfolio-aggregation layer** — how XVAs combine across many trades with many counterparties under one funding pool and one capital framework. **Source:** McNeil et al. (2015) Ch.17 pp.603-624.

The structural difficulty in portfolio XVA is **netting and collateralisation**: under ISDA Master Agreement / CSA (Credit Support Annex), trades with the same counterparty net into a single exposure, and posted collateral reduces the exposure further. So portfolio XVA is not the sum of single-trade XVAs; it is computed on **netting sets** (sets of trades that legally net within a counterparty's exposure), with collateral cash flows reducing the effective exposure profile. The **Euler-principle capital-allocation framework** (introduced in McNeil Ch.8 §8.5 and reused for XVA allocation in Ch.17) distributes portfolio-level XVA back to per-trade contributions in a way that preserves additivity. **Source:** McNeil et al. (2015) Ch.17 pp.603-624 + Ch.8 pp.299-322.

Crepey-Bielecki-Brigo (the Supporting source authorised for `06, 11` in the matrix) develops XVA at a more dynamic-modelling depth: stochastic exposure profiles, wrong-way risk in CVA, multi-curve funding under FVA, and the formal interaction with regulatory capital under KVA. The 11 vertical uses Crepey as supporting for the portfolio-aggregation layer — specifically, the framing that CVA / DVA / FVA / KVA all sit inside a single funding-cost-and-default framework rather than as independent adjustments. **Source:** Crepey-Bielecki-Brigo (2014) Ch.1-2 pp.1-50.

```
   Portfolio XVA aggregation pipeline
   ──────────────────────────────────

   per-trade pricing:
     mark-to-market values {V_k}_k for trades in the firm's book

                  | netting + collateralisation per ISDA / CSA
                  v
   netting-set level:
     +-----------------+
     | per-counterparty|       collapse trades into netting sets;
     | netting set j   |       compute net exposure profile E_j(t)
     | for cpty j      |       (collateral-adjusted)
     +--------+--------+
              |
              v
   per-counterparty XVA contributions:
     CVA_j  =  E^Q[ LGD_j · ∫ E_j(t)^+ · dP_default_j(t) ]
     DVA_j  =  symmetric to CVA_j, on firm's own default
     FVA_j  =  E^Q[ ∫ funding-spread(t) · F_j(t) · dt ]
     KVA_j  =  E^Q[ ∫ cost-of-capital(t) · K_j(t) · dt ]

                  | sum across counterparties + Euler attribution
                  v
   portfolio XVA + Euler allocation back to trades:
     +------------------------+
     | XVA_portfolio          |  = Σ_j XVA_j (additive across netting sets)
     +----------+-------------+
                |
                v
     Euler-principle allocation: per-trade contribution
     such that Σ_k allocation_k  =  XVA_portfolio  (additivity preserved)
```

## Definition

Let `{k}` index trades in the firm's book and `{j}` index counterparties. A **netting set** is the set of trades with one counterparty that legally net within a single exposure profile (typically defined by an ISDA Master Agreement scope). The **net exposure profile** for netting set `j` is: **Source:** McNeil et al. (2015) Ch.17 pp.603-610.

```
E_j(t)  =  ( Σ_{k ∈ netset_j}  V_k(t) )  −  C_j(t)

where:
  V_k(t)        =  mark-to-market value of trade k at time t
  C_j(t)        =  posted collateral by counterparty j at time t
  E_j(t)^+      =  max(E_j(t), 0)  (firm's positive exposure to counterparty j)
  E_j(t)^-      =  max(-E_j(t), 0)  (cpty's positive exposure to firm
                                       = firm's negative exposure under DVA)
```

The principal XVA contributions per netting set are: **Source:** McNeil et al. (2015) Ch.17 pp.610-624 + Crepey-Bielecki-Brigo (2014) Ch.1-2 pp.1-50.

```
CVA_j  =  E^Q [ LGD_j  ·  ∫ E_j(t)^+ · dP_{default_j}(t) ]      (cpty default)
DVA_j  =  E^Q [ LGD_firm · ∫ E_j(t)^- · dP_{default_firm}(t) ]  (own default)
FVA_j  =  E^Q [ ∫ funding_spread(t) · F_j(t) · dt ]             (funding cost)
KVA_j  =  E^Q [ ∫ cost_of_capital(t) · K_j(t) · dt ]            (capital cost)
```

where `E^Q[·]` denotes risk-neutral expectation, `dP_{default_j}` is the counterparty default-time measure, `LGD_j` is loss given default of counterparty `j`, `F_j(t)` is the funding exposure / cash requirement for netting set `j` under the desk's sign convention, and `K_j(t)` is the capital allocated to netting set `j`. The **portfolio XVA** aggregates additively across netting sets: **Source:** McNeil et al. (2015) Ch.17 pp.620-624.

```
XVA_portfolio  =  Σ_j  XVA_j                  (additive across counterparties)
```

The **Euler-principle allocation** distributes portfolio XVA back to per-trade contributions: for a risk measure `ρ` that is positively-homogeneous (as XVA is, in `EAD`-scaling), the Euler allocation per trade is: **Source:** McNeil et al. (2015) Ch.8 pp.299-322 + Ch.17 pp.620-624.

```
allocation_k  =  ∂ XVA_portfolio / ∂ q_k    ·  q_k

where q_k = trade k's notional / scaling parameter
        Σ_k allocation_k  =  XVA_portfolio    (Euler's identity, hom-1 funcs)
```

The Euler allocation is the unique allocation that simultaneously preserves additivity (`Σ_k allocation_k = XVA_portfolio`) and is consistent with marginal capital contribution. **Source:** McNeil et al. (2015) Ch.8 pp.299-322 + Ch.17 pp.620-624.

## Mathematical Reasoning

The structural reason single-trade XVAs cannot simply be summed to portfolio XVA is the **netting and collateral aggregation**: a trade with positive value to the firm and another with negative value, both with the same counterparty under one ISDA Master Agreement, partially offset within the netting set. The net exposure `E_j(t)` is a difference, not a sum. CVA and DVA depend on the **positive part** `E_j(t)^+` and the **negative part** `E_j(t)^-`, while FVA depends on the netting-set funding exposure `F_j(t)` under the desk's sign convention. Single-trade CVAs computed on per-trade exposures over-count because they don't see the netting offset; portfolio CVA must use the netting-set exposure profile. **Source:** McNeil et al. (2015) Ch.17 pp.603-610.

The **CVA / DVA asymmetry** is intentional. CVA reflects the firm's loss when the counterparty defaults while owing the firm money (positive exposure to counterparty). DVA reflects the firm's gain when the firm itself defaults while owing the counterparty money (negative exposure, the firm's debt). DVA is controversial accounting-wise: a firm reports a higher own-default DVA when its credit deteriorates, which inflates accounting profits during stress. The economic interpretation is that DVA is a hedge accounting item against the firm's own default — it nets against CVA from the counterparty's perspective so the bilateral CVA-DVA pair sums to zero across the two firms. **Source:** McNeil et al. (2015) Ch.17 pp.610-615 + Crepey-Bielecki-Brigo (2014) Ch.1-2 pp.1-50.

**FVA** addresses an asymmetry the original CVA-DVA framework missed: uncollateralised or imperfectly collateralised derivative exposures create funding cash requirements that are priced at the firm's funding spread rather than at a default-free rate. The funding cost is real even if no default occurs; FVA quantifies it as the discounted funding-spread payments over the exposure profile's lifetime. The sign convention and exposure base vary by desk and accounting framework, so this card keeps the FVA input as `F_j(t)` rather than identifying it mechanically with either `E_j(t)^+` or `E_j(t)^-`. The interaction between FVA and DVA is subtle, and the precise treatment is a topic of ongoing literature; Crepey-Bielecki-Brigo develops the funding-and-default-interaction framework. **Source:** McNeil et al. (2015) Ch.17 pp.615-620 + Crepey-Bielecki-Brigo (2014) Ch.1-2 pp.1-50.

**KVA** is the most recently-formalised XVA: the cost of holding regulatory capital `K_j(t)` against the netting set over its lifetime. KVA depends on the regulatory capital framework (Basel III risk-weighted assets, FRTB capital charges, CVA-capital charges) and on the firm's required return on capital. KVA is the link between trade-level pricing and the firm's overall capital deployment — including KVA in trade pricing internalises the regulatory-capital cost that uncharged trades would otherwise impose on the firm's balance sheet. McNeil treats KVA at the conceptual level; full IRB / FRTB implementation depth belongs to authorized regulatory text. **Source:** McNeil et al. (2015) Ch.17 pp.620-624 + Crepey-Bielecki-Brigo (2014) Ch.1-2 pp.1-50.

The **Euler allocation** is the work-horse mathematical technique for splitting portfolio XVA back to per-trade contributions. For a positively-homogeneous function `f(q) = f(q_1, …, q_K)` (XVA scales linearly with notionals `q_k`), Euler's identity states `f(q) = Σ_k q_k · ∂f/∂q_k`. So the per-trade allocation `q_k · ∂f/∂q_k` sums to portfolio `f(q)` exactly, and the partial derivative is interpretable as the **marginal XVA** contribution of trade `k` — the XVA cost the portfolio would save if trade `k` were closed out. This is the firm's natural per-trade attribution and the basis for trade-level XVA pricing. **Source:** McNeil et al. (2015) Ch.8 pp.299-322 + Ch.17 pp.620-624.

A structural subtlety: **CVA / DVA / FVA / KVA are not independent quantities** — they interact through shared netting-set exposures (`E_j(t)^+` for CVA, `E_j(t)^-` for DVA, `F_j(t)` for FVA, and `K_j(t)` for KVA which itself depends on CVA via the CVA-capital charge). A consistent portfolio framework prices them jointly under a single funding-and-default model rather than as four independent adjustments. Crepey-Bielecki-Brigo's contribution is the joint-pricing framework; McNeil's contribution is the portfolio-aggregation + Euler-allocation layer. The 11 vertical uses both. **Source:** McNeil et al. (2015) Ch.17 pp.603-624 + Crepey-Bielecki-Brigo (2014) Ch.1-2 pp.1-50.

## See Also

Cross-vertical (Fixed Income — single-counterparty CVA + CSA collateralisation):

- [fi-counterparty-risk-cva](../06_fixed_income_and_credit/fi-counterparty-risk-cva.md) — single-counterparty CVA derivation.

For the collateral-and-CSA mechanics that drive netting and collateral-adjusted exposure, see also [fi-collateralization-and-csa](../06_fixed_income_and_credit/fi-collateralization-and-csa.md).

Within v11 Risk Management:

- [rm-credit-risk-metrics-restatement](./rm-credit-risk-metrics-restatement.md) — Batch-3 sibling card on PD / LGD / EAD inputs.
- [rm-credit-var-portfolio](./rm-credit-var-portfolio.md) — Batch-3 sibling card on portfolio credit-VaR (the loss-distribution layer).
- [rm-integrated-firm-wide-risk-aggregation](./rm-integrated-firm-wide-risk-aggregation.md) — Batch-3 sibling card on firm-wide aggregation that XVA capital sits inside.

## Escalate to Raw When

The conceptual depth in this card stops at the netting-set aggregation + four-XVA framing + Euler allocation. When the operator needs the full single-counterparty CVA derivation (stochastic exposure profiles, Monte Carlo CVA computation, wrong-way risk modelling, multi-curve funding under FVA, KVA under FRTB capital), open the 06 cards above OR McNeil Ch.17 pp.603-640 directly. Crepey-Bielecki-Brigo provides the funding-and-default-interaction framework at deeper modelling depth. Specific ISDA / regulatory CVA add-on depth (BCBS standardised CVA, SA-CVA) belongs to authorized regulatory text. **Source:** McNeil et al. (2015) Ch.17 pp.603-640 + Crepey-Bielecki-Brigo (2014) Ch.1-2 pp.1-50.
