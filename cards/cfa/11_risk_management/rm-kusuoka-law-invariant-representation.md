---
schema_version: "cacg.v0"
id: "rm-kusuoka-law-invariant-representation"
title: "Kusuoka Representation: Every Law-Invariant Coherent Measure Is a Mixture of AV@R"
reading_id: "11_risk_management"
summary: "Follmer-Schied's Kusuoka representation: on an atomless space, law-invariant convex risk measures are exactly a sup over probability mixtures of AV@R minus a penalty, the coherent case drops the penalty, and law-invariance auto-implies the Fatou property — the bridge from convex/coherent axioms to the AV@R/ES spectral family."
tags: ["risk-management", "coherent-risk", "law-invariance", "kusuoka", "expected-shortfall"]
citations:
  - source_id: "rm_follmer_schied_2025_stochastic_finance"
    chunk_id: "rm_follmer_schied_2025_stochastic_finance:p257:0306"
    chunk_hash: "73d2ee7d03695e981d07f0d4f08a10399ec08293c0e96bccfddc745c4c31a704"
    page_range: [258, 258]
    quote: "can be viewed as the building blocks for law-invariant convex risk measures"
    edge_type: "defines"
  - source_id: "rm_follmer_schied_2025_stochastic_finance"
    chunk_id: "rm_follmer_schied_2025_stochastic_finance:p258:0307"
    chunk_hash: "a789e1e5cc999fbfd5a8544fd8af9a4180751e730e1ef9708805c78f23ef5980"
    page_range: [259, 259]
    quote: "Conversely, for any probability measure μ on (0,1], the function q defined by"
    edge_type: "defines"
  - source_id: "rm_follmer_schied_2025_stochastic_finance"
    chunk_id: "rm_follmer_schied_2025_stochastic_finance:p248:0295"
    chunk_hash: "432b0bdefaeb0234fe834afed9f56ec93d81a018890d433506bf348ce0591732"
    page_range: [249, 249]
    quote: "Every law-invariant convex risk measure on L∞ has the Fatou property."
    edge_type: "supports"
card_hash: "02b1ea42b3056e96df7be00b551e9cad9b401a7c9eb6a2398e1660e01a37ed36"
---
# Kusuoka Representation: Every Law-Invariant Coherent Measure Is a Mixture of AV@R

## Intuition
The convex/coherent axioms tell you a risk measure has *some* robust representation —
a worst case over a set of probabilistic scenarios Q penalized by α(Q). That set can
be enormous and abstract. The Kusuoka representation collapses it: as soon as you add
the very natural requirement that the risk only depends on the *distribution* of the
position (law-invariance — two positions with the same P-law get the same number),
the abstract scenario set is forced into a concrete shape. Every law-invariant convex
risk measure is a worst case over probability **mixtures of Average Value at Risk**,
penalized by a functional on those mixtures; and in the coherent (positively
homogeneous) case the penalty vanishes and you are left with a pure supremum of
AV@R-mixtures over some family M of mixing measures. So AV@R_λ at the single levels λ
are the *atoms*, and every reasonable law-invariant coherent measure is built by
averaging them over λ and then taking a worst case across a set of averaging recipes.
This is the structural reason the AV@R / expected-shortfall spectral family is not one
choice among many but *the* generating set for this whole class.

```
   abstract robust rep            law-invariance forces            Kusuoka form
   sup_Q ( E_Q[-X] - α(Q) )  ───────────────────────►   sup_μ ( ∫ AV@R_λ(X) μ(dλ) - β(μ) )
   scenarios Q (huge set)        depends only on law of dQ/dP      mix over levels λ ∈ (0,1]
                                                                   coherent ⇒ drop β, sup over M
```

**Source:** Follmer & Schied (Stochastic Finance, 5e) Ch.4 §4.7 (Robust representation of law-invariant risk measures) printed pp.238-241 (PDF pp.256-259).

## Definition
Fix an **atomless** probability space (Ω, F, P) and X = L∞(Ω, F, P). A monetary risk
measure ρ is **law-invariant** (Def. 4.57) if ρ(X) = ρ(Y) whenever X and Y have the
same distribution under P. Write q_{-X}(t) for a quantile function of -X and
φ_Q := dQ/dP. The structural results are:

- **Spectral atoms (AV@R).** AV@R_λ(X) is the law-invariant coherent risk measure that
  averages the tail of -X beyond level λ; it is the building block, owned by the
  expected-shortfall card. (Exercise 4.7.1: AV@R_λ(X) = (1/λ)∫₀^λ V@R_γ(X) dγ.)
- **Structure theorem (Thm 4.65).** A convex risk measure ρ is law-invariant **iff**
  it is continuous from above and its minimal penalty α_min(Q) depends only on the law
  of φ_Q under P. Then
  ρ(X) = sup_{Q∈M₁(P)} ( ∫₀¹ q_{-X}(t) q_{φ_Q}(t) dt − α_min(Q) )   (4.62).
- **Kusuoka representation, convex case (Thm 4.68).** A convex risk measure ρ is
  law-invariant **iff** there is a penalty β_min on mixing measures with
  ρ(X) = sup_{μ∈M₁((0,1])} ( ∫_{(0,1]} AV@R_λ(X) μ(dλ) − β_min(μ) )   (4.64),
  where β_min(μ) = sup_{X∈A_ρ} ∫_{(0,1]} AV@R_λ(X) μ(dλ) and A_ρ is the acceptance set.
- **Kusuoka representation, coherent case (Cor. 4.69).** A coherent risk measure ρ is
  law-invariant **iff** ρ(X) = sup_{μ∈M} ∫_{(0,1]} AV@R_λ(X) μ(dλ) for some set
  M ⊆ M₁((0,1]) of mixing measures (the penalty drops out by positive homogeneity).
- **Automatic Fatou property (Thm 4.58).** On an atomless space, every law-invariant
  convex risk measure on L∞ has the Fatou property (equivalently, is continuous from
  above and σ(L∞,L¹)-lower semicontinuous) — so the robust representation above is
  available *for free*, no extra continuity axiom needed.

**Source:** Follmer & Schied (Stochastic Finance, 5e) Ch.4 §4.6-4.7 (Def. 4.57, Thm 4.58, Thm 4.65, Thm 4.68, Cor. 4.69) printed pp.231-241 (PDF pp.249-259).

## Mathematical Reasoning
The argument runs in three moves. **(1) Free continuity.** Theorem 4.58: on an
atomless space every law-invariant convex risk measure on L∞ has the Fatou property.
The proof leans on Proposition 4.59 — a uniformly bounded sequence converging in
probability can be approximated in L∞-norm by averages of random variables each having
the *same law* as the original terms — after which law-invariance plus convexity push
the inequality ρ(X) ≤ liminf ρ(Xₙ) through. By Theorem 4.33 the Fatou property is
equivalent to continuity from above, so Theorem 4.33's robust representation
ρ(X) = sup_Q ( E_Q[-X] − α_min(Q) ) is automatic.

**(2) Penalty depends only on the law of the density (Thm 4.65).** Because X ∈ A_ρ
implies every X̃ ∼ X is in A_ρ, one rewrites
α_min(Q) = sup_{X∈A_ρ} E[-X φ_Q] = sup_{X∈A_ρ} ∫₀¹ q_{-X}(t) q_{φ_Q}(t) dt, using
Proposition 4.66 (a Hardy–Littlewood / rearrangement identity
∫₀¹ q_X(t)q_Y(t) dt = max_{X̃∼X} E[X̃Y], where the max is attained because the space is
atomless). So α_min(Q) sees Q only through the law of φ_Q, giving (4.62).

**(3) From densities to AV@R-mixtures (Thm 4.68).** The key is to convert the inner
quantile integral into a mixture over AV@R levels. Writing q_{-X}(t) = V@R_{1-t}(X) and
representing the nonnegative increasing right-continuous quantile q_{φ⁺}(t) = ν([1−t,1])
by a positive Radon measure ν, the rescaled measure μ(dt) = t·ν(dt) is a *probability*
measure on (0,1] (the normalization ∫t ν(dt) = E[φ] = 1 falls out by Fubini). A second
Fubini then yields the identity
∫₀¹ q_{-X}(t) q_φ(t) dt = ∫_{(0,1]} AV@R_s(X) μ(ds)  (4.65)-(4.66),
and the construction is reversible, giving a one-to-one correspondence between laws of
densities φ and probability measures μ on (0,1]. Substituting into (4.62) produces the
Kusuoka form (4.64). **(4) Coherent collapse (Cor. 4.69).** For a coherent ρ positive
homogeneity forces the penalty α_min (hence β_min) to be the indicator of a set: it is
0 on an acceptable family M of mixing measures and +∞ elsewhere, so the sup-minus-
penalty reduces to a plain sup over M ⊆ M₁((0,1]). The converse direction in each
theorem is the easy one: any such sup of AV@R-mixtures (minus a penalty) *defines* a
law-invariant convex (resp. coherent) risk measure continuous from above.

**Proof-gap note (Rule 6):** FS prove Thm 4.68 modulo Theorem 4.65, Proposition 4.66,
and the density-to-μ correspondence; the rearrangement identity (Prop. 4.66) and the
atomless existence lemmas (D.7, D.13, D.16, D.17) are cited rather than reproduced
here, and the coherent collapse to an indicator penalty is stated at FS's level of
rigor. Kusuoka's original coherent result is attributed in the Notes to Kusuoka [230].

```
  Thm 4.58 (free Fatou) ─► Thm 4.33 robust rep ─► Thm 4.65 (α depends on law of dQ/dP)
        │                                                     │
        │                                  Prop 4.66 (q-q = max E[X̃Y], atomless)
        ▼                                                     ▼
  density φ  ◄──── one-to-one ────►  μ ∈ M₁((0,1])  ─► Thm 4.68 (sup over μ of ∫AV@R_λ dμ − β)
                                                            │ coherent: β = indicator of M
                                                            ▼
                                                  Cor 4.69  ρ = sup_{μ∈M} ∫ AV@R_λ dμ
```

**Source:** Follmer & Schied (Stochastic Finance, 5e) Ch.4 §4.6-4.7 (Thm 4.58, Prop. 4.59, Thm 4.65, Prop. 4.66, Thm 4.68, Cor. 4.69; Notes attribute the coherent case to Kusuoka [230]) printed pp.231-241 (PDF pp.249-259).

## See Also
- [rm-expected-shortfall-mechanics](./rm-expected-shortfall-mechanics.md) — the AV@R / expected-shortfall atoms this theorem mixes; the building blocks of the representation.
- [rm-risk-measure-axioms](./rm-risk-measure-axioms.md) — the convex/coherent monetary-risk axioms and robust representation this theorem specializes under law-invariance.
- [rm-var-and-es-taxonomy](./rm-var-and-es-taxonomy.md) — where V@R/AV@R sit in the VaR/ES landscape that the spectral family generalizes.

## Escalate to Raw When
You need the actual quantile-integral and Fubini steps (4.62)-(4.66), the explicit
penalty formulas α_min(Q) = sup_{X∈A_ρ} ∫₀¹ q_{-X}(t)q_{φ_Q}(t) dt (4.63) and
β_min(μ) = sup_{X∈A_ρ} ∫ AV@R_λ(X) μ(dλ), the Exercise 4.7.1 derivation
AV@R_λ(X) = (1/λ)∫₀^λ V@R_γ(X) dγ, the Example 4.67 utility-based α_min with the
Fenchel–Legendre transform ℓ*, the rearrangement-inequality proof of Proposition 4.66,
or the downstream concave-distortion / comonotonicity refinements (Cor. 4.84, Thm 4.99).
Those derivations and any worked levels/constants live in the raw text (Rule 1).

**Source:** Follmer & Schied (Stochastic Finance, 5e) Ch.4 §4.7 (eqs. 4.62-4.66, Exercise 4.7.1, Example 4.67) printed pp.239-241 (PDF pp.257-259).
