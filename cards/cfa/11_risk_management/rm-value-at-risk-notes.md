---
schema_version: "cacg.v0"
id: "rm-value-at-risk-notes"
title: "Value-at-Risk — L1 Notes Definition and Estimation Framework"
reading_id: "11_risk_management"
summary: "Quantile-of-loss definition of VaR (alpha-quantile of L) with the three canonical estimator routes (parametric / historical-simulation / Monte Carlo), the (alpha, horizon, estimator) triplet, and McNeil's surfaced limitations (subadditivity failure + tail-shape blindness); McNeil Ch.2 §2.3.2 entry point."
tags: ["risk-management", "value-at"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p085:0120"
    chunk_hash: "1b1b814b9d97e9ea983ab9f4115c4ada656f01b6a77947b8c053674d902d5ff0"
    page_range: [85, 85]
    quote: "the VaR of a portfolio with loss L at the confidence level α is given by the smallest number l such that the probability that the loss L exceeds l is no larger than 1 − α."
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p085:0121"
    chunk_hash: "a27636dee6a4ad4ecb60856e1a572fec433faa4f7faa28578ee5841b241a0d08"
    page_range: [85, 86]
    quote: "The 95% VaR value is approximately 2.2, indicating that there is a 5% chance that we lose at least this amount."
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p086:0122"
    chunk_hash: "357988f0d10dcba21a64fe3b3f051481674fd16a0d80d7e50513ab94b163e1b5"
    page_range: [86, 87]
    quote: "This result is routinely used in the variance–covariance approach (also known as the delta-normal approach) to computing risk measures."
    edge_type: "supports"
  - source_id: "rm_hull_2023_rmfi"
    chunk_id: "rm_hull_2023_rmfi:p298:0402"
    chunk_hash: "64f2a9a89badb31306ddf416c4384dd7a58c2b137a3bc273c6abb1c836b05b4f"
    page_range: [299, 299]
    quote: "Extreme value theory (EVT) is the term used to describe the science of estimating the tails of a distribution."
    edge_type: "supports"
  - source_id: "rm_christoffersen_2012_elements"
    chunk_id: "rm_christoffersen_2012_elements:p036:0039"
    chunk_hash: "815a6c806dc2349e25db0756447cfe7ff1edda8832c1749502fa49729baae25f"
    page_range: [36, 36]
    quote: "ignore well-established stylized facts on return dependence, most importantly variance"
    edge_type: "supports"
  - source_id: "rm_bouchaud_potters_2003_theory_financial_risk"
    chunk_id: "rm_bouchaud_potters_2003_theory_financial_risk:p195:0257"
    chunk_hash: "6f950a971fd171914de47083d9019411c773a75ef7be6f338887f04d5e248e4f"
    page_range: [196, 196]
    quote: "the measure of risk as a loss probability keeps its meaning even if the variance is infinite"
    edge_type: "supports"
card_hash: "0d6cfd62adc74cacb5f7321790a3ddc699e861e43804dd3ff8da0379ad362045"
---
# Value-at-Risk — L1 Notes Definition and Estimation Framework

## Intuition

**Value-at-Risk (VaR)** at L1 depth is the simplest one-number tail risk summary: given a portfolio with loss random variable `L` over a fixed horizon, `VaR_α` is the smallest loss `l*` such that the probability of exceeding `l*` is at most `1 − α`. The source frames VaR as "the maximum loss we will not exceed with `α` confidence over a stated horizon" — a frequency statement about how often the firm will see a loss worse than the threshold. The two ingredients VaR consumes are the confidence level `α` (close to 1) and the horizon `Δt`. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.64-67.

The source-side framing emphasises three orthogonal estimation routes. **Parametric VaR** assumes a distributional family (typically normal or Student-t for returns) and reads `VaR_α` from a closed form parameterised by mean and volatility. **Historical-simulation VaR** uses the empirical CDF of past loss observations and reads the empirical α-quantile directly with no distributional assumption. **Monte Carlo VaR** draws scenarios from a calibrated joint factor model, full-revalues the portfolio per scenario, and reads the empirical α-quantile from the simulated loss sample. The L1 source summarise the trade-off but defer the mechanics to Batch 2 cards `[[rm-parametric-var]]`, `[[rm-historical-simulation-var]]`, `[[rm-monte-carlo-var]]`. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.64-67 + McNeil et al. (2015) Ch.2 pp.64-67.

VaR's two principal limitations — surfaced at the L1 entry point — are that it (1) says nothing about how bad losses get **past** the threshold (the tail shape is invisible), and (2) **fails subadditivity** in general (merging two portfolios can raise the VaR-implied capital). The source flags both as caveats and forward-link to expected shortfall (ES) and the coherence apparatus in `[[rm-var-and-es-taxonomy]]` and `[[rm-risk-measure-axioms]]` for the full treatment. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.64-67 + McNeil et al. (2015) Ch.2 pp.67.

```
   L1-source VaR pipeline
   ─────────────────────

   +---------------+      +-----------------+      +------------------+
   | Confidence α  |      | Horizon Δt      |      | Loss data /      |
   | (close to 1)  |      | (fixed)         |      | factor model     |
   +-------+-------+      +--------+--------+      +---------+--------+
           |                       |                         |
           +-----------+-----------+-------------------------+
                       |
                       v
            +----------------------+
            | Estimator route:     |
            |   • parametric       |
            |   • historical sim   |
            |   • Monte Carlo      |
            +----------+-----------+
                       |
                       v
            +----------------------+
            | VaR_α  =  q_α(L)     |     read off the loss-distribution
            +----------+-----------+     α-quantile
                       |
                       v
            interpret: probability of loss > VaR_α  ≤  1 − α
            caveat:   silent on tail past VaR_α; fails subadditivity
```

## Definition

Let `L` be the loss random variable over horizon `Δt` (under the `L = −ΔV` convention from `[[rm-loss-distribution-anatomy]]`), and let `α ∈ (0, 1)` be a confidence level (close to 1). The L1 source define VaR as: **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.64-67 + McNeil et al. (2015) Ch.2 pp.64-66.

```
VaR_α(L)  =  inf { l ∈ R : P(L ≤ l) ≥ α }  =  q_α(L)
```

This is the α-quantile of the loss distribution. Equivalently, `P(L > VaR_α) ≤ 1 − α`: the probability of a loss strictly exceeding `VaR_α` is at most the tail mass `1 − α`. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.64-67.

The three L1-source estimation routes for `VaR_α` are: **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.64-67 + McNeil et al. (2015) Ch.2 pp.64-67.

```
parametric:    assume L ~ F(θ);   read VaR_α = F^{-1}(α; θ)        (closed form)
historical:    empirical F̂_L from past losses {L_t};
               read VaR_α = empirical α-quantile of {L_t}      (no distribution)
Monte Carlo:   simulate {L^{(k)}} from a joint factor model;
               read VaR_α = empirical α-quantile of {L^{(k)}}  (model-implied)
```

Each route trades modelling assumption against statistical efficiency: parametric is variance-efficient if the assumed family is correct and biased if not; historical is assumption-free but data-hungry in the tail; Monte Carlo lets the modeller choose any factor distribution but transfers all error to the calibration step. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.64-67.

## Mathematical Reasoning

The source treats VaR as a **risk-reporting summary**, not an axiom-derived measure. The quantile definition is operational: pick `α`, pick `Δt`, pick an estimator, and read off one number. The number's interpretation is conditional on the chosen ingredients — there is no canonical VaR for a portfolio; there is `VaR_{α, Δt, estimator}`. This explains why VaR-shopping across estimators is a common practitioner game and why the L1 source insist on disclosing the full triplet `(α, Δt, estimator)` with every VaR number. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.64-67.

The **quantile interpretation** is the one mathematical fact that travels across estimators. For continuous loss distributions, `VaR_α` is the unique `l*` such that `P(L ≤ l*) = α`. For discrete or mixed distributions (any historical-simulation case), `inf` makes the quantile well-defined as the smallest threshold containing at least `α` of the mass. This `inf` convention is **required** for the empirical-VaR estimator to be consistent: ranking historical losses `L_(1) ≤ L_(2) ≤ … ≤ L_(n)` and selecting `L_(⌈α · n⌉)` is the empirical inf-quantile. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.64-67 + McNeil et al. (2015) Ch.2 pp.64-66.

VaR's **subadditivity failure** receives surface treatment at the L1 source: a textbook counter-example with two independent low-probability default bonds shows that `VaR_α(L_A + L_B) > VaR_α(L_A) + VaR_α(L_B)` for some `α`. The source flags the failure as a caveat and defer the formal coherence apparatus to the foundation cards. The risk-management response is not to abandon VaR but to **complement** it with ES (which IS coherent) and to use ES when aggregating across silos — see `[[rm-var-and-es-taxonomy]]` for the side-by-side treatment. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.64-67 + McNeil et al. (2015) Ch.2 pp.67.

The **horizon scaling** under the parametric route follows the square-root-of-time rule for Gaussian factor models with i.i.d. log-returns: `VaR_α(Δt) ≈ √Δt · VaR_α(1)`. The source flags this as the standard scaling assumption and note the failure modes (fat tails, autocorrelation, liquidity overlays). The historical-simulation route bypasses the scaling question entirely by using horizon-matched loss observations; the Monte Carlo route bakes the horizon into the path-simulation step. Detailed horizon-scaling treatment lives in `[[rm-loss-distribution-anatomy]]`. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.64-67.

The L1 source draw a boundary against **backtesting**: VaR estimation gives a forward-looking summary, but verifying that the chosen estimator-and-parameters produced a reliable VaR over time is a separate activity. Backtesting at L1 depth counts exceedances `N = |{t : L_t > VaR_α(t)}|` and compares against the expected count `(1 − α) · T`. The detailed treatment is in `[[rm-backtesting-investment-strategies]]`. **Source:** 11_Risk_Management/McNeil_Frey_Embrechts_Quantitative_Risk_Management.pdf pp.64-67.

## See Also

- [rm-var-and-es-taxonomy](./rm-var-and-es-taxonomy.md) — Batch-0 side-by-side treatment of VaR and ES with the coherence subadditivity contrast.
- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — Batch-0 definition of the loss variable `L = −ΔV` that VaR reads.
- [rm-backtesting-investment-strategies](./rm-backtesting-investment-strategies.md) — Batch-1 sibling card on exceedance-count backtesting at L1-source depth.
- [rm-parametric-var](./rm-parametric-var.md) — Batch-2 depth treatment of the parametric estimator route.
- [rm-historical-simulation-var](./rm-historical-simulation-var.md) — Batch-2 depth treatment of the historical-simulation estimator route.
- [rm-monte-carlo-var](./rm-monte-carlo-var.md) — Batch-2 depth treatment of the Monte Carlo estimator route.
- `rm-practitioner-evt-pot-historical-simulation` (Hull (2023) RMFI, pp.299) — deepening that adds a supporting source to this card.
- `rm-dynamic-vs-static-var-hs-critique` (Christoffersen (2012) Elements of FRM, pp.36) — deepening that extends this card.
- `rm-nongaussian-var-first-principles` (Bouchaud-Potters (2003) Theory of Financial Risk, pp.196) — deepening that extends this card.

## Escalate to Raw When

The source-anchored treatment stops at the quantile definition + three estimator routes + subadditivity caveat. When the operator needs the full estimator-specific mechanics (variance-covariance closed forms under elliptical assumptions, dynamic historical simulation with volatility scaling, full Monte Carlo factor-model construction, EVT-tail-fit POT estimators, joint-elicitability scoring for forecast comparison), open the Batch 2 cards above OR McNeil Ch.9 pp.325-365 directly. **Source:** McNeil et al. (2015) Ch.9 pp.325-365.
