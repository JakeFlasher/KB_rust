---
schema_version: "cacg.v0"
id: "rm-risk-measure-axioms"
title: "Coherent and Convex Risk-Measure Axioms"
reading_id: "11_risk_management"
summary: "The four Artzner-Delbaen-Eber-Heath coherence axioms (monotonicity, translation-invariance, positive-homogeneity, subadditivity) with VaR's documented failure of subadditivity and ES's coherence; convex / law-invariant extensions per McNeil Ch.2 §2.3.5 + Ch.8 §8.1."
tags: ["risk-management", "risk-measure"]
citations:
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p094:0135"
    chunk_hash: "9794eb566c6ca5dbe47b9ce590a247d4faf4aca6d6645e733bdd6d850baf8c9e"
    page_range: [94, 95]
    quote: "subadditivity reflects the idea that risk can be reduced by diversification, a time-honoured principle in finance and economics."
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p095:0137"
    chunk_hash: "56db9085e37a17a1ede4db0b0679a2f218253651cc69be7a0b8743f61f37f1e0"
    page_range: [95, 96]
    quote: "However, the following example shows that VaR is in general not subadditive, and hence, in general, neither is it a convex nor a coherent measure of risk."
    edge_type: "defines"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p296:0424"
    chunk_hash: "763c816c0294ae7f1b894657552fb8969e0f5112e1c079ff5de5eca34bdc662b"
    page_range: [296, 297]
    quote: "Definition 8.1. A risk measure that satisfies the monotonicity, translation invariance and convexity axioms is called a convex measure of risk"
    edge_type: "supports"
  - source_id: "rm_mcneil_frey_embrechts_2015_qrm"
    chunk_id: "rm_mcneil_frey_embrechts_2015_qrm:p300:0429"
    chunk_hash: "63a412467d72b5d0b939a2e9d59a8c0e2eb2de24675514a3547f324a9fd5fd66"
    page_range: [300, 300]
    quote: "risk measures of the form (8.6) are frequently"
    edge_type: "supports"
  - source_id: "rm_hull_2023_rmfi"
    chunk_id: "rm_hull_2023_rmfi:p268:0359"
    chunk_hash: "1f6cd2558f83c7d6760b26c6da5315d10f534ebb22e0914c67da70b5e48e4d5a"
    page_range: [268, 268]
    quote: "Risk measures satisfying all four conditions given above are referred to as coherent."
    edge_type: "supports"
  - source_id: "rm_bouchaud_potters_2003_theory_financial_risk"
    chunk_id: "rm_bouchaud_potters_2003_theory_financial_risk:p193:0253"
    chunk_hash: "dcc635d71c7be0b13e36be67fe7fde4e9d6d75b0359d46970d65e8f71dc9eb53"
    page_range: [193, 193]
    quote: "a Gaussian model for the price fluctuations is never justified for the extreme events"
    edge_type: "supports"
  - source_id: "rm_follmer_schied_2025_stochastic_finance"
    chunk_id: "rm_follmer_schied_2025_stochastic_finance:p206:0244"
    chunk_hash: "bc99d4bd78ad883b3f402ea197e89bc1fe5795675319ef9fde3998e4368dbd06"
    page_range: [206, 206]
    quote: "Together with convexity and monotonicity, it singles out the class of convex risk measures"
    edge_type: "supports"
  - source_id: "rm_follmer_schied_2025_stochastic_finance"
    chunk_id: "rm_follmer_schied_2025_stochastic_finance:p210:0249"
    chunk_hash: "ac7e27fd0724a39ab9a6e5372e76417a294a1dffe83635f9474071c90c38fb12"
    page_range: [210, 210]
    quote: "we can then define the capital requirement"
    edge_type: "supports"
  - source_id: "rm_follmer_schied_2025_stochastic_finance"
    chunk_id: "rm_follmer_schied_2025_stochastic_finance:p218:0260"
    chunk_hash: "a93807d9eea35753eed5acf7877864f30c7ae117e94f1d7f90657e0c08bd6bcb"
    page_range: [219, 219]
    quote: "Any convex risk measure ρ on X is of the form"
    edge_type: "supports"
card_hash: "0089afa465fe7c78bccff70fae3e55da890a73758540913bc68cb6dadd50e9c3"
---
# Coherent and Convex Risk-Measure Axioms

## Intuition

A **risk measure** `ρ` maps a loss random variable `L` to a real number — the "amount of capital" the firm must hold against the position to be acceptable. The axiomatic approach asks: which structural properties should every reasonable risk measure satisfy? Coherence (Artzner-Delbaen-Eber-Heath 1999, formalised in McNeil-Frey-Embrechts Ch.2 §2.3.5) is the canonical 4-axiom answer. **Source:** McNeil et al. (2015) Ch.2 pp.72-78.

The four coherence axioms are: **monotonicity** (a bigger loss state-by-state requires more capital), **translation-invariance** (adding a sure loss `c` to the portfolio raises the capital by exactly `c`), **positive-homogeneity** (scaling the portfolio by `λ ≥ 0` scales capital by `λ`), and **subadditivity** (the capital for a merged portfolio cannot exceed the sum of standalone capitals — a "diversification benefit"). A measure satisfying all four is **coherent**. **Source:** McNeil et al. (2015) Ch.2 pp.72-76.

The axioms are not just aesthetics — they have direct portfolio consequences. Subadditivity is the property VaR famously **lacks**: there are explicit counter-examples in which combining two portfolios increases VaR. Expected shortfall (ES), in contrast, is coherent under mild regularity, which is the formal reason ES is structurally preferred when capital must be aggregated across portfolios or desks. **Source:** McNeil et al. (2015) Ch.2 pp.76-78.

Diagrammatically, subadditivity is the axiom that supports the firm-wide aggregation tree: if `ρ` is coherent, then merging silos into a firm-wide loss cannot exceed the sum of silo-level capitals. The aggregation-tree primitive (see `[[rm-integrated-firm-wide-risk-aggregation]]` for the firm-wide depth treatment) illustrates the directional bound. **Source:** McNeil et al. (2015) Ch.2 pp.75-76 + Ch.8 pp.299-322.

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

The four coherence axioms (each must hold for `ρ` to be coherent) are restated symbolically. **Source:** McNeil et al. (2015) Ch.2 pp.72-76.

```
    M : L₁ ≤ L₂ pointwise        ⇒ ρ(L₁) ≤ ρ(L₂)       (monotonicity)
    T : ρ(L + c) = ρ(L) + c                              (translation)
    H : ρ(λ L) = λ ρ(L)           for λ ≥ 0              (homogeneity)
    S : ρ(L₁ + L₂) ≤ ρ(L₁) + ρ(L₂)                       (subadditivity)
```

A weaker variant, **convexity**, replaces positive-homogeneity + subadditivity with the single requirement `ρ(λ L₁ + (1−λ) L₂) ≤ λ ρ(L₁) + (1−λ) ρ(L₂)` for `λ ∈ [0,1]`. Convex measures admit liquidity-cost effects (scaling the portfolio by a large `λ` can cost MORE than `λ ρ(L)` because liquidating a bigger position into a thin market hurts the price). Coherent measures are a special case of convex measures with positive-homogeneity assumed. **Source:** McNeil et al. (2015) Ch.8 pp.275-280.

## Definition

A **risk measure** is a functional `ρ : L → R` mapping a loss random variable to a real number; conventionally `ρ(L)` is the capital required to make position `L` "acceptable". **Source:** McNeil et al. (2015) Ch.2 pp.61-64.

A risk measure `ρ` is **coherent** if it satisfies the following four axioms for all losses `L`, `L₁`, `L₂` in the relevant space and all constants `c ∈ R`, `λ ≥ 0`: **Source:** McNeil et al. (2015) Ch.2 pp.72-76.

```
(M)  Monotonicity:           L₁ ≤ L₂  (almost surely)  ⇒  ρ(L₁) ≤ ρ(L₂)
(T)  Translation-invariance: ρ(L + c) = ρ(L) + c
(H)  Positive-homogeneity:   ρ(λ L) = λ · ρ(L)            for λ ≥ 0
(S)  Subadditivity:          ρ(L₁ + L₂) ≤ ρ(L₁) + ρ(L₂)
```

A risk measure is **convex** if it satisfies (M), (T), and the convexity axiom: **Source:** McNeil et al. (2015) Ch.8 pp.275-280.

```
(C)  Convexity: ρ(λ L₁ + (1 − λ) L₂) ≤ λ ρ(L₁) + (1 − λ) ρ(L₂)   for λ ∈ [0, 1]
```

Under positive-homogeneity, convexity is equivalent to subadditivity: `(H) + (C) ⇔ (H) + (S)`. Convex-but-not-coherent measures relax positive-homogeneity to allow scale-dependent capital (large positions cost more per unit). **Source:** McNeil et al. (2015) Ch.8 pp.280-285.

A risk measure is **law-invariant** if `ρ(L₁) = ρ(L₂)` whenever `L₁` and `L₂` have the same distribution. Both VaR and ES are law-invariant. **Source:** McNeil et al. (2015) Ch.8 pp.286-292.

## Mathematical Reasoning

Each axiom encodes a structural property. **Monotonicity** is the most basic: if one position dominates another in every state of nature, it cannot be less risky. Without monotonicity, the risk measure could prefer a strictly worse outcome — clearly unacceptable. **Source:** McNeil et al. (2015) Ch.2 pp.72-74.

**Translation-invariance** specifies how cash interacts with risk. Adding a guaranteed loss `c` (e.g., a known future expense) raises required capital by `c`; adding a guaranteed gain `−c` reduces capital by `c`. Equivalently, `ρ(L − ρ(L)) = 0`: subtracting the required capital makes the position exactly "acceptable" (zero residual risk to capitalize). This is the formal sense in which `ρ(L)` is the capital amount. **Source:** McNeil et al. (2015) Ch.2 pp.74.

**Positive-homogeneity** asserts capital scales linearly with position size. Doubling the portfolio doubles required capital. The axiom is appropriate when the firm faces no liquidation friction — selling double the position incurs no incremental price impact. Real markets violate this for large positions; convex measures preserve the rest of coherence while dropping (H). **Source:** McNeil et al. (2015) Ch.2 pp.74-75.

**Subadditivity** is the diversification axiom. Merging two positions `A` and `B` into one portfolio cannot require more capital than holding them as separate businesses: `ρ(L_A + L_B) ≤ ρ(L_A) + ρ(L_B)`. The intuition: extreme losses in `A` and `B` rarely coincide, so the merged tail is smaller than the sum of tails. Without subadditivity, a regulator's "diversification penalty" would create perverse incentives — a bank could halve its capital by spinning off a business unit. **Source:** McNeil et al. (2015) Ch.2 pp.75-76.

VaR fails subadditivity in general. The canonical counter-example: two defaultable bonds with independent low-probability defaults can have a combined VaR(α) exceeding `VaR(α) + VaR(α)` for some `α` — merging them moves more mass past the VaR quantile. ES does NOT have this problem; it averages losses past the quantile, so by Jensen-type arguments the merged ES is bounded by the sum. **Source:** McNeil et al. (2015) Ch.2 pp.76-78 + Ch.8 pp.286-292.

The Artzner-Delbaen-Eber-Heath theorem establishes a **dual representation**: every coherent risk measure on a finite probability space can be written as a supremum of expected losses over a set of "generalised scenarios" (probability measures). Convex measures admit a similar representation with a penalty function on scenarios. Dual-representation depth (Daniell-type results on general spaces, penalty functions, Fenchel-Legendre duality) is McNeil Ch.8 §8.1.2-§8.1.3 territory and is out of scope for the v11 vertical; the result we use is just that coherent ⇒ "worst expected loss over a plausible scenario set". **Source:** McNeil et al. (2015) Ch.8 pp.275-285.

## See Also

- [rm-var-and-es-taxonomy](./rm-var-and-es-taxonomy.md) — applies the coherence axioms side-by-side to VaR and ES; subadditivity contrast.
- [rm-expected-shortfall-mechanics](./rm-expected-shortfall-mechanics.md) — derives ES as the canonical coherent measure with the dual integral representation.
- [rm-integrated-firm-wide-risk-aggregation](./rm-integrated-firm-wide-risk-aggregation.md) — uses subadditivity for the aggregation bound (`ρ(L_total) ≤ Σ ρ(L_silo)`) and the Euler-principle capital allocation.
- `rm-convex-risk-measures-dual-representation` (McNeil-Frey-Embrechts (2015) QRM, pp.300) — deepening that extends this card.
- `rm-coherent-risk-measure-axioms-practitioner` (Hull (2023) RMFI, pp.268) — deepening that adds a supporting source to this card.
- `rm-volatility-inadequate-fat-tail-risk` (Bouchaud-Potters (2003) Theory of Financial Risk, pp.193) — deepening that extends this card.
- `rm-convex-risk-measure-axioms` (Foellmer-Schied (2025) Stochastic Finance, pp.206) — deepening that extends this card.
- `rm-acceptance-set-duality` (Foellmer-Schied (2025) Stochastic Finance, pp.210) — deepening that extends this card.
- `rm-robust-dual-representation` (Foellmer-Schied (2025) Stochastic Finance, pp.219) — deepening that extends this card.

## Escalate to Raw When

The conceptual depth in this card stops at the 4-axiom statement plus the convex/law-invariant extensions. When the operator needs the full dual representation (Artzner-Delbaen-Eber-Heath theorem proof, penalty-function characterisation of convex measures, Fenchel-Legendre duality on general probability spaces, or distortion / spectral / expectile risk measures as canonical coherent families), open McNeil Ch.8 §8.1.2-§8.2 pp.280-292 directly. The proof techniques there require functional-analysis machinery beyond the L1 risk-management entry point and intentionally defer to a quantitative-methods extension. **Source:** McNeil et al. (2015) Ch.8 pp.280-292.
