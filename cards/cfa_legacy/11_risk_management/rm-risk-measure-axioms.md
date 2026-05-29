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
card_hash: "dc5bf63c0f68996bd0be38d9ffc722ed81dba825c10c371d2f364f9ebafabc6a"
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

## Escalate to Raw When

The conceptual depth in this card stops at the 4-axiom statement plus the convex/law-invariant extensions. When the operator needs the full dual representation (Artzner-Delbaen-Eber-Heath theorem proof, penalty-function characterisation of convex measures, Fenchel-Legendre duality on general probability spaces, or distortion / spectral / expectile risk measures as canonical coherent families), open McNeil Ch.8 §8.1.2-§8.2 pp.280-292 directly. The proof techniques there require functional-analysis machinery beyond the L1 risk-management entry point and intentionally defer to a quantitative-methods extension. **Source:** McNeil et al. (2015) Ch.8 pp.280-292.
