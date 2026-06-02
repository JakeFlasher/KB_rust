---
schema_version: "cacg.v0"
id: "rm-fisher-tippett-ev-types"
title: "Fisher-Tippett-Gnedenko Theorem and the Three Extreme-Value Types"
reading_id: "11_risk_management"
summary: "EKM's heavy-tail-probability mechanism for fat tails via maxima: the only non-degenerate limit laws for affinely-normalized sample maxima c_n⁻¹(M_n − d_n) are the three max-stable extreme-value types — Fréchet Φ_α, Weibull Ψ_α, Gumbel Λ — unified by the GEV shape parameter ξ, per Embrechts, Klüppelberg & Mikosch (1997) Ch.3."
tags: ["risk-management", "extreme-value-theory", "fat-tails"]
citations:
  - source_id: "rm_embrechts_kluppelberg_mikosch_1997_modelling_extremal_events"
    chunk_id: "rm_embrechts_kluppelberg_mikosch_1997_modelling_extremal_events:p132:0125"
    chunk_hash: "a480bdbf690ea76e66036fa3af44711ba7b1af11ef791c18df54400bfafbf917"
    page_range: [132, 132]
    quote: "The following result is the basis of classical extreme value theory"
    edge_type: "defines"
card_hash: "ab94617faf14220075a91ff7e6f68b9097f677079893da3f3024f5c58e990d1a"
---
# Fisher-Tippett-Gnedenko Theorem and the Three Extreme-Value Types

## Intuition
This card is the **heavy-tail probability / extreme-value-types** mechanism for fat
tails — the EKM route that asks what the *largest* observation in a long iid sample
can possibly look like, the maxima analogue of the other three routes by which fat
tails enter risk: applied / statistical EVT fit to the tail (`rm-evt-gpd-pot-hill`),
Lévy / stable-Paretian scaling (`rm-levy-stable-paretian-tails`), and
crash-as-criticality (`rm-crash-as-critical-point`). Just as the central limit
theorem says that *sums*, suitably normalized, can converge only to the stable
laws (and in the finite-variance case the normal), Fisher-Tippett says that the
running *maximum* M_n = max(X_1,…,X_n), suitably normalized, can converge only to a
tiny menagerie: exactly three shapes. Which one you land in is dictated by the tail
of the underlying loss law — a power-law tail goes to Fréchet, a bounded-above tail
to Weibull, an exponentially-thin tail to Gumbel. This is *why* a single shape
parameter is enough to summarize "how fat is the tail of the worst case."

```
   CLT for sums:    normalized Σ X_i  ──►  stable laws (normal if Var finite)
   FT for maxima:   normalized max X_i ──►  one of THREE types only:

        Fréchet Φ_α          Weibull Ψ_α          Gumbel Λ
        (power tail,         (bounded above,      (thin tail,
         ξ > 0)               ξ < 0)               ξ = 0)
            ╲                    │                    ╱
             ╲___________  GEV H_ξ  ___________╱   (unified by shape ξ)
```

**Source:** Embrechts, Klüppelberg & Mikosch (1997) Ch.3 §3.2 printed pp.120–121 (PDF pp.131–132).

## Definition
Let (X_n) be iid with df F and M_n = max(X_1,…,X_n). A non-degenerate df is
**max-stable** if for iid copies the affine identity max(X_1,…,X_n) =_d c_n X + d_n
holds for every n ≥ 2 with norming constants c_n > 0, d_n ∈ ℝ; equivalently
c_n⁻¹(M_n − d_n) =_d X. Theorem 3.2.2 shows the max-stable laws coincide with the
class of all possible non-degenerate limit laws for normalized maxima.

The **Fisher-Tippett (Gnedenko) theorem** (Theorem 3.2.3) then names them: if there
exist norming constants c_n > 0, d_n ∈ ℝ and a non-degenerate H with
c_n⁻¹(M_n − d_n) →_d H, then H is of the type of one of three dfs:

| Type | Df | Range / parameter |
|------|----|----|
| Fréchet | Φ_α(x) = exp(−x^{−α}) | x > 0, α > 0 (else 0) |
| Weibull | Ψ_α(x) = exp(−(−x)^α) | x ≤ 0, α > 0 (else 1) |
| Gumbel | Λ(x) = exp(−e^{−x}) | x ∈ ℝ |

These collapse into one **generalized extreme value (GEV)** family H_ξ via a shape
parameter ξ: ξ > 0 ↔ Fréchet (with α = 1/ξ), ξ < 0 ↔ Weibull, ξ = 0 ↔ Gumbel.

**Source:** Embrechts, Klüppelberg & Mikosch (1997) Ch.3 §3.2 Def 3.2.1, Thm 3.2.2–3.2.3 (the three separate types) printed pp.120–122 (PDF pp.131–133); the GEV/shape-ξ unification (α = 1/ξ) is §3.4 Def 3.4.1 (Jenkinson–von Mises) printed p.152 (PDF p.163).

## Mathematical Reasoning
The result is a "convergence to types" argument. First (Theorem 3.2.2): if
F^n(c_n x + d_n) → H(x) for a non-degenerate H, then along the subsequence nk one
has both F^{nk}(c_n x + d_n) → H^k(x) and F^{nk}(c_{nk} x + d_{nk}) → H(x). The
convergence-to-types theorem (Khinchin) forces the two norming sequences to be
asymptotically affinely related, c_{nk}/c_n → c̃_k and (d_{nk} − d_n)/c_n → d̃_k,
giving max(Y_1,…,Y_k) =_d c̃_k Y + d̃_k for iid Y with df H — i.e. **the limit law
must itself be max-stable**. So the set of attainable limits is exactly the
max-stable class.

Second (Theorem 3.2.3): solving the functional max-stability equation
H^k(c̃_k x + d̃_k) = H(x) for all k pins H down to the three closed forms above.
The argument parallels the stable-law classification for sums: max-stability is the
multiplicative-on-the-df analogue of stability, and the type is selected by the
limiting behavior of the norming constants. The three types are mutually exclusive
representatives of distinct "types" (equivalence classes under affine transformation),
and the GEV reparametrization H_ξ makes the family a single smooth curve in ξ, so
that estimating the worst-case tail reduces to estimating one shape parameter. The
companion question — *which* F lands in *which* type — is answered by the
maximum-domain-of-attraction characterizations (e.g. F ∈ MDA(Φ_α) ⟺ F̄ regularly
varying; see `rm-mda-regular-variation-frechet`).

**Source:** Embrechts, Klüppelberg & Mikosch (1997) Ch.3 §3.2 Thm 3.2.2–3.2.3 printed pp.121–124 (PDF pp.132–135); GEV reparametrization H_ξ §3.4 Def 3.4.1 printed p.152 (PDF p.163).

## See Also
- [rm-evt-gpd-pot-hill](./rm-evt-gpd-pot-hill.md) — the applied / statistical EVT mechanism whose GEV/POT models rest on these types.
- [rm-cramer-lundberg-heavy-tail-ruin](./rm-cramer-lundberg-heavy-tail-ruin.md) — the companion EKM heavy-tail / ruin mechanism (sums vs maxima).
- [rm-levy-stable-paretian-tails](./rm-levy-stable-paretian-tails.md) — the Lévy / stable-Paretian mechanism for fat tails (contrast).
- [rm-crash-as-critical-point](./rm-crash-as-critical-point.md) — the Sornette criticality mechanism for fat tails (contrast).
- [rm-var-and-es-taxonomy](./rm-var-and-es-taxonomy.md) — the VaR/ES taxonomy that EVT tail estimates refine.

## Escalate to Raw When
You need worked norming-constant computations c_n, d_n for a specific F, the fitted
GEV densities plotted with α = 1 in Figure 3.2.4, or any numeric block-maxima fit —
those worked examples and figures live in the raw text (Rule 1).

**Source:** Embrechts, Klüppelberg & Mikosch (1997) Ch.3 §3.2–3.3 printed pp.120–128 (PDF pp.131–139).
