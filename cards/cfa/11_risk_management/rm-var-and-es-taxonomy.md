---
schema_version: "cacg.v0"
id: "rm-var-and-es-taxonomy"
title: "VaR and Expected Shortfall — Side-by-Side Taxonomy"
reading_id: "11_risk_management"
summary: "Side-by-side comparison of VaR (alpha-quantile of L) and ES (tail-conditional expectation past quantile) with the coherence contrast (ES coherent, VaR fails subadditivity), Gaussian/Student-t closed-form contrasts, and elicitability framing per McNeil Ch.2 §2.3.4 + Ch.8 §8.2."
tags: ["risk-management", "var-and"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p090:0129"
    chunk_hash: "bfaf33fc2e40351464f557cbaa0106b1b65b5313d8e128dafd376817dc2f7a23"
    page_range: [90, 91]
    quote: "the ES at confidence level α ∈ (0, 1) is defined as ESα = 1 1 − α 1 α qu(FL) du, (2.22) where qu(FL) = F ← L (u) is the quantile function of FL."
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p092:0132"
    chunk_hash: "3485b06e4b641b428bc867fdc6b47243baa76f654b339ef0afe2d26e93de7880"
    page_range: [92, 93]
    quote: "if we use ES, the risk in the tails of the t model is reflected in our risk measurement for lower values of α."
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p307:0438"
    chunk_hash: "8d4b0a823fcb3b05f4a397323b14bde7dffcb2d143b8fa3e3ee11f47f66b75e4"
    page_range: [307, 308]
    quote: "examples of law-invariant risk measures areVaR and expected shortfall. On the other hand, the stress-test risk measures of Example 8.9 are typically not law invariant."
    edge_type: "supports"
card_hash: "eb59028ad01da7b6ced5bf3823d512e967600d44586e407d3457e60c93d992b1"
---
# VaR and Expected Shortfall — Side-by-Side Taxonomy

## Intuition

Value-at-Risk (`VaR_α`) and Expected Shortfall (`ES_α`) are the two canonical tail risk measures applied to a loss random variable `L`. Both are indexed by a confidence level `α ∈ (0, 1)` (with `α` close to 1) — but they answer **different questions** about the tail. **VaR** answers: "what is the smallest loss we are at least `α`-confident we will not exceed?" — a **quantile** of the loss distribution. **ES** answers: "given that we are in the worst `(1 − α)` fraction of outcomes, what loss should we expect?" — a **tail-conditional expectation**. **Source:** McNeil et al. (2015) Ch.2 pp.64-68.

The two measures coincide only in the deepest tail behavior they capture. VaR uses a single threshold and ignores how bad things can get past that threshold; ES averages over the tail and is therefore sensitive to the **shape** of the tail, not just its location. A portfolio with a thin tail past `VaR_α` and one with a fat tail past `VaR_α` can have **identical VaR but very different ES**. This tail-shape sensitivity, together with coherence (see below), is the formal reason ES is structurally preferred over VaR for capital aggregation. **Source:** McNeil et al. (2015) Ch.2 pp.68-72.

The deepest contrast is structural: ES is **coherent** under mild regularity (continuous loss distribution); VaR is **not coherent** because it fails subadditivity. Concretely, two portfolios with low-probability independent defaults can have a combined VaR that exceeds the sum of standalones — a regulator who used VaR for capital aggregation would create a perverse incentive to fragment positions across legal entities. ES has no such defect: by averaging past the quantile it inherits a Jensen-type inequality that keeps the merged tail expectation below the sum. See `[[rm-risk-measure-axioms]]` for the axiom statements. **Source:** McNeil et al. (2015) Ch.2 pp.76-78 + Ch.8 pp.286-292.

```
<!-- primitive: var-tail-and-es source: _diagram_primitives.md -->
   density f_L(l)
   ^
   |   * * *
   |  *       *
   | *          *
   |*             *
   |*               *
   |*                 *
   |*                   *
   |*                      *           VaR_α = q_α(L)
   |*                          *       (α-quantile of loss L)
   |*                              *
   |*                                  ES_α = E[L | L >= q_α]
   |*                                     *
   |*           body of f_L                * tail (mass = 1 − α)
   |*                                          *
   |*                                                 *  *
   +*------------------------------------*------------------> L
                                       q_α (VaR)
       <-------- α probability mass -------->
       <------- 1 − α tail mass: ES averages L here ------>
```

## Definition

Let `L` be the loss random variable over a fixed horizon (loss = `−ΔV` in the P&L convention; see `[[rm-loss-distribution-anatomy]]`). For confidence level `α ∈ (0, 1)`, **Value-at-Risk at level α** is the lower α-quantile of `L`: **Source:** McNeil et al. (2015) Ch.2 pp.64-66.

```
VaR_α(L) = inf { l ∈ R : P(L ≤ l) ≥ α } = q_α(L)
```

Equivalently, `P(L > VaR_α) ≤ 1 − α`: the probability of a loss strictly exceeding `VaR_α` is at most the tail mass `1 − α`. **Source:** McNeil et al. (2015) Ch.2 pp.65.

**Expected Shortfall at level α** is the expected loss conditional on the loss falling in the worst `(1 − α)` tail. For continuous loss distributions the two equivalent representations are: **Source:** McNeil et al. (2015) Ch.2 pp.66-68.

```
ES_α(L) = E[L | L ≥ VaR_α(L)]                              (tail-CTE form)

ES_α(L) = (1 / (1 − α)) ∫_{α}^{1} VaR_u(L) du            (quantile-average form)
```

The quantile-average form generalises cleanly to distributions with point mass at the quantile and is the regulatory-text definition. Both forms satisfy `ES_α ≥ VaR_α` with equality iff the tail has no further dispersion past `q_α`. **Source:** McNeil et al. (2015) Ch.2 pp.66-68 + Ch.8 pp.286-289.

## Mathematical Reasoning

The two definitions encode complementary tail summaries. **VaR is a frequency statement**: under VaR-based capital, the firm holds enough capital to survive `α` of all loss realisations; in `(1 − α)` of them, it does not. **ES is a severity statement**: it tells you, conditional on the bad-state event occurring, how bad the average outcome is. Frequency-only summaries hide tail thickness; severity summaries reward holding capital against fat-tailed exposures. **Source:** McNeil et al. (2015) Ch.2 pp.66-68.

The **coherence contrast** is the load-bearing structural difference. Of the four coherence axioms (monotonicity, translation-invariance, positive-homogeneity, subadditivity — see `[[rm-risk-measure-axioms]]`), VaR satisfies the first three under mild regularity but **fails subadditivity** in general. The canonical counter-example is two independent defaultable bonds each with default probability `p < 1 − α`: standalone `VaR_α = 0` for each (no default in the upper α-quantile), but jointly the combined `VaR_α` can be strictly positive because the combined default probability `2p(1−p) + p² > 1 − α`. So `VaR_α(L_A + L_B) > 0 = VaR_α(L_A) + VaR_α(L_B)`. ES does NOT exhibit this pathology: averaging past the quantile smooths the tail and the merged tail expectation is bounded above by the sum of standalone tail expectations. **Source:** McNeil et al. (2015) Ch.2 pp.76-78 + Ch.8 pp.289-292.

A second derived contrast is **elicitability** (the existence of a strictly consistent scoring function that the true measure uniquely minimises in expectation). VaR is elicitable (the pinball/quantile-loss scoring function works); ES is NOT elicitable on its own (it is conditionally elicitable jointly with VaR). The implication: comparing competing VaR-forecasters is theoretically clean (rank by mean pinball loss); comparing competing ES-forecasters needs joint scoring with the VaR-forecaster or a backtest framework that compares exceedance distributions rather than point scores. Elicitability depth defers to the future-01 quantitative-methods extension. **Source:** McNeil et al. (2015) Ch.8 pp.289-292.

Structurally, switching from a VaR to an ES capital measure brings two upgrades simultaneously: (a) **coherent aggregation** — `ES` satisfies subadditivity so silo-level capitals are an upper bound on firm-wide capital; and (b) **fat-tail sensitivity** — `ES` averages over the tail past the quantile rather than reading a single threshold, so books with thick tails past `VaR_α` register strictly higher under `ES_{α'}` than under `VaR_α` for matched `α'`. For Gaussian-like books the two measures can be calibrated to similar magnitudes; for fat-tailed books `ES_{α'}` is strictly more conservative. **Source:** McNeil et al. (2015) Ch.2 pp.69-72.

For the standalone case where `L ∼ N(μ, σ²)`, both measures admit closed forms: `VaR_α = μ + σ Φ^{-1}(α)` and `ES_α = μ + σ · φ(Φ^{-1}(α)) / (1 − α)`, where `φ` and `Φ` are the standard normal density and CDF. The ratio `ES_α / VaR_α` exceeds 1 by an `α`-dependent factor that quantifies "how much more average tail loss" ES measures past the VaR threshold; for fat-tailed distributions (Student-t, generalised Pareto in the tail), the ratio is strictly larger than the Gaussian case. The Batch 2 expected-shortfall-mechanics card derives the ratio for the Student-t case. **Source:** McNeil et al. (2015) Ch.2 pp.68-69 + Ch.8 pp.286-289.

## See Also

- [rm-risk-measure-axioms](./rm-risk-measure-axioms.md) — the four coherence axioms (M/T/H/S) with the subadditivity failure of VaR.
- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — the loss-variable convention `L = −ΔV` that both VaR and ES read.
- [rm-expected-shortfall-mechanics](./rm-expected-shortfall-mechanics.md) — Batch-2 derivation of the closed-form ES expressions and the ES/VaR ratio for Student-t tails.

## Escalate to Raw When

The conceptual depth in this card stops at the two definitions, the coherence contrast, and the Gaussian closed forms. When the operator needs the full estimator comparison (historical / parametric / Monte Carlo VaR-and-ES variance properties, EVT-based POT estimators for the deep tail, or joint elicitability scoring for ES-vs-VaR forecast comparison), open McNeil Ch.8 §8.2-§8.3 pp.286-310 directly. Specific regulatory rules (Basel III FRTB internal-model approach, α-rescaling schedules, liquidity-horizon mappings) belong to the authorized regulatory text and are not derivable from McNeil. The Batch 2 cards `[[rm-parametric-var]]`, `[[rm-historical-simulation-var]]`, `[[rm-monte-carlo-var]]`, and `[[rm-expected-shortfall-mechanics]]` cover the estimator depth as it lands. **Source:** McNeil et al. (2015) Ch.8 pp.286-310.
