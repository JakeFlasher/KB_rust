---
schema_version: "cacg.v0"
id: "rm-cramer-lundberg-heavy-tail-ruin"
title: "Cramér-Lundberg Ruin Theory for Heavy-Tailed (Subexponential) Claims"
reading_id: "11_risk_management"
summary: "EKM's heavy-tail-probability / ruin-asymptotics mechanism for fat tails: when the integrated-tail F_I is subexponential the ruin probability decays polynomially as ψ(u) ~ ρ⁻¹ F̄_I(u), an estimate that holds if and only if F_I ∈ S, contrasting the exponential small-claim Lundberg bound, per Embrechts, Klüppelberg & Mikosch (1997) Ch.1."
tags: ["risk-management", "heavy-tails", "ruin-theory"]
citations:
  - source_id: "rm_embrechts_kluppelberg_mikosch_1997_modelling_extremal_events"
    chunk_id: "rm_embrechts_kluppelberg_mikosch_1997_modelling_extremal_events:p054:0050"
    chunk_hash: "efc6be3f3ef7b47bd58c9a1d6dc81850ff1d32a189a553e41c6797ce8fc1a2f2"
    page_range: [54, 54]
    quote: "natural class when it comes to ruin estimates whenever the"
    edge_type: "defines"
card_hash: "19dde75c27f6d572bf912903f87a1a5a9e4adfb586b441667d9a31cc59ead003"
---
# Cramér-Lundberg Ruin Theory for Heavy-Tailed (Subexponential) Claims

## Intuition
This card is the **heavy-tail probability / ruin-asymptotics** mechanism for fat
tails — the EKM route in which fatness enters through how a *sum* of insurance
claims breaches a reserve, as opposed to the other three routes by which fat tails
arrive in risk: applied / statistical EVT fit directly to the tail
(`rm-evt-gpd-pot-hill`), Lévy / stable-Paretian scaling of returns
(`rm-levy-stable-paretian-tails`), and crash-as-criticality
(`rm-crash-as-critical-point`). Here the question is operational: a surplus process
U(t) = u + ct − S(t) starts at capital u, earns premium at rate c, and pays out the
aggregate claim S(t); ruin is the event that U(t) ever drops below 0, with
probability ψ(u). For *small* claims (exponential moment exists) the classical
Cramér-Lundberg bound makes ψ(u) decay **exponentially** in u. For *large* /
heavy-tailed claims that exponential moment is gone, the adjustment coefficient does
not exist, and ruin instead decays only as fast as the claim tail itself — much
slower, dominated by a single catastrophic claim rather than an unlucky accumulation
of ordinary ones.

```
   ruin prob ψ(u)
   ^
   | \                         small claims (Cramér-Lundberg):
   |  \  e^{-Ru}               exponential decay, light reserve suffices
   |   \
   |    '-.____                heavy / subexponential claims:
   |          '''----____      ψ(u) ~ ρ⁻¹ F̄_I(u)  (polynomial / slow decay)
   +----------------------------> u  (initial reserve)
```

**Source:** Embrechts, Klüppelberg & Mikosch (1997) Ch.1 §1.3.2 printed pp.42–43 (PDF pp.53–54).

## Definition
Work in the Cramér-Lundberg model with iid positive claims X_k of df F, finite mean
μ, Poisson claim arrivals, and **net profit condition** ρ > 0 (the safety loading; EKM
denotes this loading ρ). Two objects govern the heavy-tailed regime:

- **Integrated tail distribution.** F_I(x) = μ⁻¹ ∫₀ˣ F̄(y) dy, the df whose tail
  F̄_I weights how much tail mass the claim size carries beyond a level.
- **Subexponential class S.** A df with support (0,∞) is subexponential if for all
  n ≥ 2, F̄^{n*}(x)/F̄(x) → n as x → ∞ — equivalently P(S_n > x) ~ P(M_n > x), i.e.
  the tail of the *sum* of n iid claims is governed by the tail of the *largest*
  one (see `rm-subexponential-single-big-jump`).

The card's claim is the **large-claim Cramér-Lundberg estimate**: when F_I ∈ S, the
ruin probability satisfies ψ(u) ~ ρ⁻¹ F̄_I(u) as u → ∞ (Theorem 1.3.6).

**Source:** Embrechts, Klüppelberg & Mikosch (1997) Ch.1 §1.3.2 printed pp.42–43 (PDF pp.53–54).

## Mathematical Reasoning
Ruin in the Cramér-Lundberg model is a compound-geometric tail: with the
compound-geometric (Pollaczek-Khinchine) representation,
ψ(u) = (1 − (1+ρ)⁻¹) Σ_{n≥0} (1+ρ)⁻ⁿ F̄_I^{n*}(u), u ≥ 0,
so ψ is a geometric mixture of convolution tails of F_I. Divide through by F̄_I(u)
and let u → ∞. Subexponentiality of F_I supplies exactly the two ingredients needed
to pass the limit inside the sum: term-by-term, F̄_I^{n*}(u)/F̄_I(u) → n, while the
uniform domination bound F̄_I^{n*}(u)/F̄_I(u) ≤ K(1+ε)ⁿ (with (1+ρ)⁻¹(1+ε) < 1)
licenses dominated convergence. The series then collapses to a geometric sum,
yielding the limit
ψ(u)/F̄_I(u) → ρ⁻¹, i.e. ψ(u) ~ ρ⁻¹ F̄_I(u).

The estimate is not merely sufficient — it is **characterizing**. Theorem 1.3.8
states the equivalence of: (a) F_I ∈ S; (b) 1 − ψ ∈ S; and (c)
lim_{u→∞} ψ(u)/F̄_I(u) = ρ⁻¹. Hence the asymptotic *holds if and only if* the
integrated tail is subexponential; S is the natural class for ruin estimates exactly
when the small-claim Lundberg condition fails. Contrast the two decay regimes: the
small-claim bound ψ(u) ≤ e^{−Ru} relies on an adjustment coefficient R > 0 (a
positive root of the moment-generating equation, which requires F to have an
exponential moment); subexponential F_I has no such moment — its Laplace-Stieltjes
transform has an essential singularity at 0 — so no exponential bound exists and
decay is only polynomial / tail-rate slow.

```
  small claims:  light tail ──► R > 0 exists ──► ψ(u) ≲ e^{-Ru}   (geometric)
  heavy claims:  F_I ∈ S    ──► no R         ──► ψ(u) ~ ρ⁻¹ F̄_I(u) (single big jump)
                              ⇕ (iff, Thm 1.3.8)
                 the estimate is *equivalent* to subexponentiality of F_I
```

**Source:** Embrechts, Klüppelberg & Mikosch (1997) Ch.1 §1.3.2–1.3.3 Thm 1.3.6/1.3.8 printed pp.42–45 (PDF pp.53–56).

## See Also
- [rm-evt-gpd-pot-hill](./rm-evt-gpd-pot-hill.md) — the applied / statistical EVT mechanism for fat tails (contrast).
- [rm-fisher-tippett-ev-types](./rm-fisher-tippett-ev-types.md) — the companion EKM limit-law for maxima.
- [rm-levy-stable-paretian-tails](./rm-levy-stable-paretian-tails.md) — the Lévy / stable-Paretian mechanism for fat tails (contrast).
- [rm-crash-as-critical-point](./rm-crash-as-critical-point.md) — the Sornette criticality mechanism for fat tails (contrast).
- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — the loss-tail anatomy this asymptotic refines.

## Escalate to Raw When
You need a worked ruin-probability number, a fitted adjustment coefficient R, a
plug-and-chug Lundberg-bound evaluation, or the simulated lognormal/Pareto risk-path
realisations of U(t) — those numeric examples and the worked figures live in the raw
text (Rule 1).

**Source:** Embrechts, Klüppelberg & Mikosch (1997) Ch.1 §1.3–1.4 printed pp.42–55 (PDF pp.53–66).
