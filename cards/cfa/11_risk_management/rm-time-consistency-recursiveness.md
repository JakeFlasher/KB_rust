---
schema_version: "cacg.v0"
id: "rm-time-consistency-recursiveness"
title: "Time Consistency, Recursiveness and the Supermartingale Criterion"
reading_id: "11_risk_management"
summary: "Föllmer-Schied's dynamic-risk consistency: (strong) time consistency of a sequence of conditional convex risk measures equals recursiveness rho_t = rho_t(-rho_{t+1}), and (for sensitive sequences) equals a Q-supermartingale / Doob property of the penalty process. Entropic risk is consistent; AV@R is not."
tags: ["risk-management", "dynamic-risk-measures", "time-consistency", "supermartingale"]
citations:
  - source_id: "rm_follmer_schied_2025_stochastic_finance"
    chunk_id: "rm_follmer_schied_2025_stochastic_finance:p532:0628"
    chunk_hash: "1cc83d8174682f976300f3f265564e7a6e6e21c13e8f3cc3016329454efa77fc"
    page_range: [532, 532]
    quote: "Time consistency is equivalent to each of the following two properties:"
    edge_type: "defines"
  - source_id: "rm_follmer_schied_2025_stochastic_finance"
    chunk_id: "rm_follmer_schied_2025_stochastic_finance:p532:0628"
    chunk_hash: "1cc83d8174682f976300f3f265564e7a6e6e21c13e8f3cc3016329454efa77fc"
    page_range: [533, 533]
    quote: "is time-consistent. Let us check recursiveness:"
    edge_type: "supports"
  - source_id: "rm_follmer_schied_2025_stochastic_finance"
    chunk_id: "rm_follmer_schied_2025_stochastic_finance:p532:0628"
    chunk_hash: "1cc83d8174682f976300f3f265564e7a6e6e21c13e8f3cc3016329454efa77fc"
    page_range: [533, 533]
    quote: "is time-consistent. Let us check recursiveness"
    edge_type: "supports"
  - source_id: "rm_follmer_schied_2025_stochastic_finance"
    chunk_id: "rm_follmer_schied_2025_stochastic_finance:p539:0636"
    chunk_hash: "9f41e5b761fdf8070faee5eb0121f34bd3b989574807b82549f8e6166d6a26d5"
    page_range: [540, 540]
    quote: "the process (αmin t (Q))t=0,1,...,T is a Q-supermartingale for any such Q"
    edge_type: "supports"
card_hash: "da6c8df263428a6002260165f50a98c2850f7055fc2d8295b3e5a2822d8396d7"
---
# Time Consistency, Recursiveness and the Supermartingale Criterion

## Intuition

A *dynamic* risk measure assigns, at every date `t`, an `F_t`-measurable risk
assessment `rho_t(X)` to a terminal position `X`. The single most important
structural question is whether these date-by-date assessments *cohere* through
time. The intuitive demand is: if at every state of date `t+1` position `X` looks
no riskier than `Y`, then already at date `t` it should look no riskier than `Y`.
A family that obeys this is **(strongly) time-consistent**; one that violates it
can rank `X` below `Y` tomorrow in *every* scenario yet rank `X` above `Y` today —
a genuine dynamic inconsistency, not a feature of risk aversion.

Föllmer and Schied show this single demand is equivalent to a purely algebraic
**recursion**: today's risk of `X` equals today's risk of *minus tomorrow's risk*,
`rho_t(X) = rho_t(-rho_{t+1}(X))`. So a consistent measure can be built by backward
induction one step at a time — the same recursive logic that underlies dynamic
programming and the Snell-envelope / superhedging schemes of Part II of the book.
Two canonical examples sharpen the picture: the **conditional entropic** measure
satisfies the recursion exactly (the tower property of conditional expectation does
the work), whereas conditional **Average Value at Risk** — and conditional V@R, and
mean-standard-deviation rules — *fail* it. The deeper layer is dynamic: the same
penalty-function machinery that gives the static robust representation acquires a
*supermartingale* law of motion in the consistent case.

```
   risk seen tomorrow (state by state)      risk seen today
   rho_{t+1}(X) <= rho_{t+1}(Y)   ==(time-consistency)==>   rho_t(X) <= rho_t(Y)
                         |
                         | equivalent (Lemma 11.11)
                         v
   recursion:   rho_t(X) = rho_t( -rho_{t+1}(X) )      (build backward, one step at a time)
```

**Source:** Föllmer & Schied (Stochastic Finance, 5e) Ch.11 §11.2 (Time consistency), Def. 11.10 / Lemma 11.11, printed pp.514-515 (PDF pp.532-533).

## Definition

Throughout, `(rho_t)_{t=0,...,T}` is a sequence of **convex conditional risk
measures** `rho_t : L^inf -> L^inf_t` (each conditionally cash-invariant, monotone,
conditionally convex, normalized), as developed in the sibling card on conditional
dynamic risk measures.

- **(Strong) time consistency (Definition 11.10).** The sequence is *(strongly)
  time-consistent* if for all `X, Y in L^inf` and all `t >= 0`,
  `rho_{t+1}(X) <= rho_{t+1}(Y)  ==>  rho_t(X) <= rho_t(Y)`.
  (The implication is the *monotone-extension* property: tomorrow's ordering, holding
  in every state, propagates to today.)

- **Recursiveness.** The sequence is *recursive* if
  `rho_t = rho_t(-rho_{t+1})` for `t = 0,...,T-1`. Equivalently (Exercise 11.2.1),
  for `0 <= s < t <= T`, `rho_s(X) = rho_s(-rho_t(X))`: any number of one-step
  rollbacks may be collapsed into a single multi-step rollback. The sign convention
  (`-rho_{t+1}`) reflects that `rho_{t+1}(X)` is a capital requirement, fed back in as
  a position before being re-assessed.

- **Equivalence (Lemma 11.11).** Time consistency is equivalent to *each* of:
  (a) `rho_{t+1}(X) = rho_{t+1}(Y) ==> rho_t(X) = rho_t(Y)` (order-preservation of
  equalities); and (b) **Recursiveness** above.

- **Sensitivity / relevance (Definition 11.16).** The sequence is *sensitive* if
  every `rho_t` admits the robust representation
  `rho_t(X) = ess sup_{Q in Q} ( E_Q[-X | F_t] - alpha^min_t(Q) )` with respect to
  one *fixed* set `Q = {Q ~ P}` of equivalent measures and the *minimal penalty
  function* `alpha^min_t`. Sensitivity is the standing hypothesis of the
  supermartingale criterion (Theorem 11.17).

- **One-step penalty.** Restricting `rho_t` to `L^inf_{t+1}` gives the one-step
  acceptance set `A_{t,t+1} = {X in L^inf_{t+1} : rho_t(X) <= 0}` and the one-step
  penalty `alpha^min_{t,t+1}(Q) = ess sup_{X in A_{t,t+1}} E_Q[-X | F_t]`. These are
  the building blocks of the penalty dynamics.

**Source:** Föllmer & Schied (Stochastic Finance, 5e) Ch.11 §11.2, Def. 11.10, Lemma 11.11, Def. 11.16, printed pp.514-518 (PDF pp.532-536).

## Mathematical Reasoning

**Recursiveness <=> time consistency (Lemma 11.11).** Time consistency trivially
gives (a) (apply the implication both ways). For **(a) => recursiveness**: by
conditional cash invariance and normalization, `rho_{t+1}(-rho_{t+1}(X)) =
rho_{t+1}(X)`; applying (a) with `Y := -rho_{t+1}(X)` yields `rho_t(X) =
rho_t(-rho_{t+1}(X))`. For **recursiveness => time consistency**: if
`rho_{t+1}(X) <= rho_{t+1}(Y)`, then monotonicity of `rho_t` gives
`rho_t(-rho_{t+1}(X)) <= rho_t(-rho_{t+1}(Y))`, i.e. `rho_t(X) <= rho_t(Y)` by the
recursion. The argument is purely algebraic — only the monetary axioms are used.

**Example: the entropic recursion (Example 11.12).** Fix `beta > 0` and let
`rho_t(X) = (1/beta) log E[ e^{-beta X} | F_t ]` (the conditional entropic risk
measure). Recursiveness is checked symbolically:

```
rho_t( -rho_{t+1}(X) )
   = (1/beta) log E[ exp( -beta * ( -(1/beta) log E[ exp(-beta X) | F_{t+1} ] ) ) | F_t ]
   = (1/beta) log E[ E[ e^{-beta X} | F_{t+1} ] | F_t ]      (the exp/log cancel)
   = (1/beta) log E[ e^{-beta X} | F_t ]                     (tower property)
   = rho_t(X).
```

The cancellation of `exp` against `log` collapses the nested measure, and the
**tower property** of conditional expectation then flattens `F_{t+1}` into `F_t`.
So entropic risk is time-consistent. Two caveats the text flags: consistency is
**lost** if the constant `beta` is replaced by an adapted process `(beta_t)`; and,
under a dynamic law-invariance condition, the entropic family is essentially the
*only* time-consistent dynamic convex risk measure (the limiting case `beta = 0` is
ordinary conditional expectation `-E[X | F_t]`).

**Counterexample: AV@R is not time-consistent (Example 11.13).** Conditional
`AV@R_lambda(.|F_t)` (and conditional V@R, and Sharpe-ratio / mean-standard-
deviation rules) fail recursiveness. On conditionally Gaussian positions these
measures take the form `rho_t(X) = -E[-X | F_t] + gamma * sqrt(Var(X | F_t))`. Take
`X = X_1 + X_2` with independent `X_i ~ N(0, sigma_i^2)` and `F_1 = sigma(X_1)`.
Then `rho_1(X) = -X_1 + gamma*sigma_2`, so the rolled-back value is
`rho_0(-rho_1(X)) = gamma*(sigma_1 + sigma_2)`, whereas the directly computed value
is `rho_0(X) = gamma*(sigma_1^2 + sigma_2^2)^{1/2}`. Since `(sigma_1+sigma_2) >
(sigma_1^2+sigma_2^2)^{1/2}` unless one `sigma_i = 0`, we get
`rho_0(-rho_1(X)) > rho_0(X)`: the recursion is *strictly violated*, so AV@R cannot
be made time-consistent by backward iteration of itself. (The same obstruction
extends to any Kusuoka-type mixture `ess sup_mu integral AV@R_lambda mu(dlambda)`.)

**The supermartingale criterion (Theorem 11.17).** For a **sensitive** sequence of
convex conditional risk measures, the following are equivalent:
(a) time consistency;
(b) the penalty function obeys the *one-step decomposition*
`alpha^min_t(Q) = alpha^min_{t,t+1}(Q) + E_Q[ alpha^min_{t+1}(Q) | F_t ]` for all
`Q in Q` and all `t`;
(c) for every `Q in Q` and `X in L^inf`, the process
`U^{Q,X}_t := rho_t(X) + alpha^min_t(Q)` satisfies
`E_Q[ U^{Q,X}_{t+1} | F_t ] <= U^{Q,X}_t` and is a **Q-supermartingale** whenever
`alpha^min_0(Q) < inf`.

```
   penalised risk process    U^{Q,X}_t = rho_t(X) + alpha^min_t(Q)
   consistency  <=>  U^{Q,X} is a Q-supermartingale (drifts down "on average" under Q)
                       (Theorem 11.17, (a)<=>(c) is the main result of §11.2)
```

*Proof skeleton (FS's own structure).* (a)=>(b): time consistency gives the
acceptance-set splitting `A_t = A_{t,t+1} + A_{t+1}` (Proposition 11.15), and taking
`ess sup` of `E_Q[-X | F_t]` over the two summands separates `alpha^min_t` into the
one-step term plus the conditional expectation of the next penalty (Lemma 11.3 in
the last step). (b)=>(c): using an upward-directed approximating sequence `Q_n`
(Lemma 11.19, via the pasting Lemma 11.18) and monotone convergence, one shows
`E_Q[U_{t+1}|F_t] <= U_t`; integrability (hence the genuine supermartingale property)
follows because `(b)` forces `E_Q[alpha^min_t(Q)] <= alpha^min_0(Q) < inf` and
`rho_t(X)` is bounded by `||X||_inf`. (c)=>(a): the supermartingale inequality plus
the dual bound `rho_{t+1}(X)+alpha^min_{t+1}(Q) >= E_Q[-X | F_{t+1}]` yields
`rho_t(Y) >= E_Q[-X | F_t] - alpha^min_t(Q)` for all `Q`, and the representation
(11.28) closes `rho_t(Y) >= rho_t(X)`.

**Doob decomposition of the penalty process (Remark 11.20).** Property (b) is
strictly stronger than the supermartingale property alone: it pins down the
*predictable increasing part*. Writing the Doob decomposition of the
Q-supermartingale `(alpha^min_t(Q))`,

```
   alpha^min_t(Q) = M^Q_t - sum_{k=0}^{t-1} alpha^min_{k,k+1}(Q),     M^Q a Q-martingale.
```

The predictable, increasing compensator is *exactly* the running sum of the one-step
penalties. FS read this as a **built-in learning effect**: if the world is in fact
driven by `Q`, the penalization of `Q` decreases "on average," i.e. the penalty is a
Q-supermartingale. (The weaker inequality `E_Q[alpha^min_{t+1}(Q)|F_t] <=
alpha^min_t(Q)` of (11.35) alone characterizes only the weaker *weak* time
consistency `rho_{t+1}(X) <= 0 => rho_t(X) <= 0`; see Remark 11.21.) In the
coherent special case (`alpha^min` is 0/+inf valued) Theorem 11.22 specializes this
to the stability-under-pasting characterization of §6.4 / Theorem 6.52.

*Proof-gap note (per Critical Rule 6):* this card states Theorem 11.17 with FS's full
proof skeleton; the supporting Lemmas 11.3, 11.18, 11.19, Proposition 11.15 and the
Fatou/sensitivity hypotheses are quoted by role, not reproved here — they live in the
conditional-risk-measure sibling card and §11.1.

**Source:** Föllmer & Schied (Stochastic Finance, 5e) Ch.11 §11.2, Lemma 11.11, Examples 11.12-11.13, Theorem 11.17, Remark 11.20, printed pp.514-522 (PDF pp.532-540).

## See Also
- [rm-conditional-dynamic-risk-measures](./rm-conditional-dynamic-risk-measures.md) — the conditional convex/coherent risk measures, robust representation and minimal penalty `alpha^min_t` this card's recursion and supermartingale criterion build on.
- [rm-expected-shortfall-mechanics](./rm-expected-shortfall-mechanics.md) — AV@R / Expected Shortfall mechanics; AV@R is FS's canonical *non*-time-consistent example here.
- [rm-risk-measure-axioms](./rm-risk-measure-axioms.md) — the static monetary/convex/coherent axioms (cash invariance, monotonicity, convexity) whose conditional versions drive the recursion proof.

## Escalate to Raw When
The raw text carries the explicit numbers and algebra deliberately kept out of this
skeleton (Critical Rule 1): the **entropic chain of identities** in Example 11.12 (the
`exp(-beta(...))` cancellation, PDF p.533); the **AV@R Gaussian counterexample**
arithmetic `rho_0(-rho_1(X)) = gamma(sigma_1+sigma_2)` vs `rho_0(X) =
gamma(sigma_1^2+sigma_2^2)^{1/2}` with the strict inequality (PDF p.533); the **closed
form of the one-step penalty** `alpha^min_{t,t+1}(Q) = ess sup_{X in A_{t,t+1}}
E_Q[-X|F_t]` and the displayed equations (11.28)-(11.35) (PDF pp.535-540); the
**pasting/directedness constructions** of Lemmas 11.18-11.19 and the line-by-line
`(a)=>(b)=>(c)=>(a)` proof of Theorem 11.17 (PDF pp.537-540); and the **explicit Doob
decomposition** `alpha^min_t(Q) = M^Q_t - sum_{k<t} alpha^min_{k,k+1}(Q)` with `M^Q` a
Q-martingale (Remark 11.20, PDF p.540). Consult the raw for Exercises 11.2.1-11.2.5
(multi-step recursion, stopping-time pasting, weak time consistency) and Theorem 11.22
(coherent / stability-under-pasting) when a full proof or worked instance is required.

**Source:** Föllmer & Schied (Stochastic Finance, 5e) Ch.11 §11.2, Examples 11.12-11.13, Theorem 11.17, Remarks 11.20-11.21, printed pp.515-523 (PDF pp.533-541).
