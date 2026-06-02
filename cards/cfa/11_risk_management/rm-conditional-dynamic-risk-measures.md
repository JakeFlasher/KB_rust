---
schema_version: "cacg.v0"
id: "rm-conditional-dynamic-risk-measures"
title: "Conditional (Dynamic) Risk Measures and Their Robust Representation"
reading_id: "11_risk_management"
summary: "Follmer-Schied lift the static monetary-risk-measure axioms to a filtration: conditional cash invariance, monotonicity, and convexity define an Ft-measurable risk map, with a conditional robust (dual) representation via an essential-supremum over models and an Ft-measurable penalty (conditional relative entropy in the entropic case), per Ch.11 §11.1."
tags: ["risk-management", "convex-risk-measures", "dynamic-risk", "robust-representation"]
citations:
  - source_id: "rm_follmer_schied_2025_stochastic_finance"
    chunk_id: "rm_follmer_schied_2025_stochastic_finance:p522:0618"
    chunk_hash: "060d81f9863fdbba60678240539d7b85be20009e38c6aaad206ede7ccd0c095f"
    page_range: [523, 523]
    quote: "we prove a conditional version of the robust representation theorem"
    edge_type: "defines"
  - source_id: "rm_follmer_schied_2025_stochastic_finance"
    chunk_id: "rm_follmer_schied_2025_stochastic_finance:p525:0621"
    chunk_hash: "fc3f64448df451426bc06696b51c509c1b41dbaf42fcdcbf237f44a47911464c"
    page_range: [525, 525]
    quote: "we are going to take the minimal penalty function, defined as the worst conditional loss"
    edge_type: "supports"
  - source_id: "rm_follmer_schied_2025_stochastic_finance"
    chunk_id: "rm_follmer_schied_2025_stochastic_finance:p529:0625"
    chunk_hash: "368bc33ff71d32cb2d35f9fc3aba6aefcf307209afe3e46eb0e15298e9f48d29"
    page_range: [529, 529]
    quote: "the induced convex conditional risk measure ρt is given by"
    edge_type: "supports"
card_hash: "cc6015300eb0ee9ed070453b82aed083891919c5151ada77bac296e93ffc27f3"
---
# Conditional (Dynamic) Risk Measures and Their Robust Representation

## Intuition
A static monetary risk measure ρ answers one question once: "how much capital must I
add to position X today to make it acceptable?" The answer is a single number. But risk
is reassessed as information arrives. At an intermediate time t we already know the
history up to t — modelled as the σ-algebra F_t in a filtration (F_t) — and the capital
requirement now *depends on that history*, so it is no longer a number but an
F_t-measurable random variable ρ_t(X). This card lifts the static axioms of
rm-risk-measure-axioms to a filtration: it is the *information-indexed* generalization
the static convex-risk cards do not carry.

The conceptual moves are: (i) the cash buffer X_t you can add is itself any *known* (i.e.
F_t-measurable) amount, not just a constant — so cash invariance becomes conditional;
(ii) the mixing weight λ in convexity may be a *known* random fraction, not just a scalar;
and (iii) the dual representation's expectations become *conditional* expectations and
its scalar penalty α(Q) becomes an F_t-measurable penalty α_t(Q). Setting t = 0 (where
F_0 = {∅, Ω} so every F_0-measurable variable is constant) collapses every conditional
statement back to the static one, recovering Chapter 4 exactly.

```
  static (t = 0)                       conditional (general t)
  --------------                       -----------------------
  ρ(X)  ∈ ℝ            ── lift to ──►   ρ_t(X)  ∈ L∞_t   (F_t-measurable)
  ρ(X+m), m ∈ ℝ                        ρ_t(X+X_t), X_t ∈ L∞_t
  λ ∈ [0,1] scalar                     λ ∈ L∞_t, 0 ≤ λ ≤ 1
  E_Q[−X] − α(Q)                       E_Q[−X | F_t] − α_t(Q)
  sup over Q                           ess sup over Q
```

**Source:** Follmer & Schied (Stochastic Finance, 5e) Ch.11 §11.1 printed pp.505–507 (PDF pp.523–525).

## Definition
Fix a filtration (F_t)_{t=0,...,T} on (Ω, F, P) with F_0 = {∅, Ω} and F_T = F. Positions
live in L∞ := L∞(Ω, F, P); the subspace L∞_t := L∞(Ω, F_t, P) holds positions depending
only on history up to t. All (in)equalities on random variables hold P-a.s.

A map ρ_t : L∞ → L∞_t is a **monetary conditional risk measure** if, for all X, Y ∈ L∞:

- **Conditional cash invariance:** ρ_t(X + X_t) = ρ_t(X) − X_t for any X_t ∈ L∞_t. (A
  known position added now reduces the required capital one-for-one.)
- **Monotonicity:** X ≤ Y ⇒ ρ_t(X) ≥ ρ_t(Y).
- **Normalization:** ρ_t(0) = 0.

It is **convex** if in addition it has

- **Conditional convexity:** ρ_t(λX + (1−λ)Y) ≤ λρ_t(X) + (1−λ)ρ_t(Y) for λ ∈ L∞_t with
  0 ≤ λ ≤ 1 (the mixing weight is F_t-measurable, not merely a scalar).

It is **coherent** if it is convex and also

- **Conditional positive homogeneity:** ρ_t(λX) = λρ_t(X) for λ ∈ L∞_t with λ ≥ 0.

**Acceptance set.** To ρ_t associate A_t := {X ∈ L∞ | ρ_t(X) ≤ 0}, satisfying:
monotone (X ∈ A_t, Y ≥ X ⇒ Y ∈ A_t); ess inf{X ∈ L∞_t | X ∈ A_t} = 0 with 0 ∈ A_t. The
risk measure is recovered as the conditional capital requirement
ρ_t(X) = ess inf{Y ∈ L∞_t | X + Y ∈ A_t}, and ρ_t is convex iff A_t is conditionally
convex (X, Y ∈ A_t, λ ∈ L∞_t, 0 ≤ λ ≤ 1 ⇒ λX + (1−λ)Y ∈ A_t).

**Source:** Follmer & Schied (Stochastic Finance, 5e) Ch.11 §11.1 (Def. 11.1) printed pp.505–506 (PDF pp.523–524).

## Mathematical Reasoning
**The conditional robust (dual) representation — Theorem 11.2.** Recall the static result
(Theorem 4.33): a Fatou convex risk measure satisfies ρ(X) = sup_{Q ∈ M_1(P)} (E_Q[−X] −
α(Q)) with minimal penalty α_min(Q) = sup_{X: ρ(X) ≤ 0} E_Q[−X]. The conditional version
indexes the penalty by the history up to t. Define the **minimal conditional penalty
function** as the worst *conditional* expected loss over all currently-acceptable
positions,

```
  α_min_t(Q) = ess sup_{X ∈ A_t} E_Q[ −X | F_t ].
```

For this to be well-defined under both Q and the reference P, restrict to
Q_t := {Q ∈ M_1(P) | Q ≈ P on F_t}.

Theorem 11.2 states that for a convex conditional risk measure ρ_t the following are
equivalent: (a) the **representation** ρ_t(X) = ess sup_{Q ∈ Q_t} (E_Q[−X | F_t] −
α_min_t(Q)) holds (and continues to hold over the smaller P_t := {Q ∈ Q_t | Q = P on
F_t}); (b) ρ_t has the **Fatou property** ρ_t(X) ≤ lim inf_n ρ_t(X_n) for bounded
sequences converging P-a.s.; (c) ρ_t is **continuous from above** (X_n ↘ X ⇒ ρ_t(X_n) ↗
ρ_t(X)). The static dual sup becomes an essential supremum over models, the unconditional
expectation becomes a conditional expectation, and the scalar penalty becomes an
F_t-measurable random variable.

```
   STATIC                              CONDITIONAL (Thm 11.2)
   ρ(X) = sup_Q ( E_Q[−X]    − α(Q) )
                  └ scalar ──────────► ess sup_Q ( E_Q[−X|F_t] − α_min_t(Q) )
                                                      └ F_t-measurable penalty
```

**Coherent case (Corollary 11.6).** If ρ_t is coherent with the Fatou property, the
penalty can only take values 0 or ∞ (a positive-homogeneity scaling argument: on
{α_min_t(Q) > 0}, scaling acceptable X by λ ↑ ∞ forces the penalty to +∞). Hence the
penalty drops out and ρ_t(X) = ess sup_{Q ∈ P^ρ_t} E_Q[−X | F_t] over the conditional
model set P^ρ_t = {Q ∈ P_t | α_min_t(Q) = 0 P-a.s.}.

**Conditional entropic risk measure (Example 11.5).** With exponential utility
u(x) = 1 − exp(−βx), β > 0, the acceptance set A_t = {X | E[e^{−βX} | F_t] ≤ 1} induces

```
  ρ_t(X) = (1/β) · log E[ e^{−βX} | F_t ].
```

Its minimal penalty is α_min_t(Q) = (1/β) Ĥ_t(Q | P) for Q ∈ P_t, where the **conditional
relative entropy** given F_t is

```
  Ĥ_t(Q|P) := E_Q[ log (dQ/dP) | F_t ] = E[ (dQ/dP) log (dQ/dP) | F_t ].
```

This is the conditional analogue of the static entropic penalty (1/β)·H(Q|P); Ĥ_t is the
Radon-Nikodym density's conditional entropy, and ρ_t is therefore called the *conditional
entropic risk measure*. (Exercise 11.1.3 supplies the variational identity Ĥ_t(Q|P) =
sup_Z (E_Q[Z | F_t] − log E[e^Z | F_t]), the conditional Gibbs/Donsker-Varadhan form, with
the supremum attained at Z = log(dQ/dP).)

**Proof-gap label (per Critical Rule 6).** FS prove the (c)⇒(a)⇒(b)⇒(c) cycle of
Theorem 11.2 in full (using the upward-directedness Lemma 11.3 to compute the essential
supremum as a limit of an increasing sequence) and prove Corollary 11.6 directly. The
convexity, the robust representation, and the explicit minimal-penalty formula (11.19)
for the conditional *entropic* measure are stated as a result but **deferred to
Exercise 11.1.3** (argued "as in Theorem C.5" and "as in Example 4.34"); FS do not write
that derivation out in §11.1. We reproduce the claims at FS's stated rigor and flag the
entropic-penalty derivation as an exercise gap rather than an in-text proof.

**Source:** Follmer & Schied (Stochastic Finance, 5e) Ch.11 §11.1 (Thm. 11.2, Cor. 11.6, Ex. 11.5) printed pp.507–512 (PDF pp.525–530).

## See Also
- [rm-time-consistency-recursiveness](./rm-time-consistency-recursiveness.md) — the dynamic sibling: how a *sequence* (ρ_t)_{t=0,...,T} of these conditional measures links across time (recursiveness, supermartingale penalty).
- [rm-risk-measure-axioms](./rm-risk-measure-axioms.md) — the static (t=0) monetary/convex/coherent axioms this card conditionalizes.
- [rm-expected-shortfall-mechanics](./rm-expected-shortfall-mechanics.md) — the coherent static ES whose conditional AV@R counterpart is the Corollary-11.6 coherent representation.

## Escalate to Raw When
The static-to-conditional dictionary here is definitional; the raw text carries the
pieces this card deliberately omits under Rule 1. Escalate to FS §11.1 for: the explicit
β-parameter algebra of the conditional entropic measure ρ_t(X) = (1/β) log E[e^{−βX} |
F_t] and the value 1/β of its penalty scale; the worked acceptance-set chain for
Example 11.5; the conditional Value-at-Risk and conditional AV@R constructions
(Example 11.7 / Definition 11.8, with the density bound 1/λ); the full directed-upward
argument and equations (11.8)–(11.17) behind Lemma 11.3; and any numeric instantiation of
the dual penalty. Those derivation steps and any concrete numbers live in the raw text.

**Source:** Follmer & Schied (Stochastic Finance, 5e) Ch.11 §11.1 (Ex. 11.5–11.7, Def. 11.8, Lem. 11.3) printed pp.510–513 (PDF pp.528–531).
