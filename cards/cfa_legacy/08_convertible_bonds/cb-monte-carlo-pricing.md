---
schema_version: "cacg.v0"
id: "cb-monte-carlo-pricing"
title: "Monte Carlo Pricing for Convertibles"
reading_id: "08_convertible_bonds"
summary: "Monte Carlo Pricing for Convertibles — placeholder summary                      "
tags: ["convertible-bonds", "monte-carlo"]
citations:
  - source_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities"
    chunk_id: "cb_despiegeleer_schoutens_vanhulle_2014_hybrid_securities:p164:0217"
    chunk_hash: "6ae864fbf03e08ba391f5180490627a4d3f971f74a7ff444967de2a7464bdf8a"
    page_range: [164, 165]
    quote: "In this book we will focus on closed-form solutions, trinomial trees, and the American Monte Carlo method."
    edge_type: "supports"
  - source_id: "cb_hull_2022_options_futures_derivatives_11ed"
    chunk_id: "cb_hull_2022_options_futures_derivatives_11ed:p651:0968"
    chunk_hash: "0c1ac86a333b1329e33bc49bdf424202104f13df481a1858657a6c47ba2dcea5"
    page_range: [651, 652]
    quote: "The life of the tree is set equal to the life of the convertible bond."
    edge_type: "supports"
card_hash: "23f8e980b1bd58174f9b4174e5b5512313fb9c3c28c650d5f1c86fa7f9dac9fe"
---
# Monte Carlo Pricing for Convertibles

## Intuition

The credit-aware CRR tree from the
[binomial-tree card](./cb-binomial-tree-valuation.md#mathematical-reasoning)
prices the canonical convertible cleanly, but **path-dependent
provisions** force tree augmentation that is expensive: a soft-call
trigger over a trailing window of `N` daily closes inflates state
size by a factor of `N`, and a stochastic-volatility model adds
another factor for the vol state. Monte Carlo simulation handles
path-dependence naturally — each simulated path carries its own
trigger history — at the cost of harder American-exercise pricing.
The practitioner solution is the **Longstaff-Schwartz** algorithm:
simulate forward, regress continuation values backward against
basis functions of the current state, and exercise when the
regression-implied continuation value falls below the immediate-
exercise value. **Source:** Glasserman (2003) §8 pp.420-450;
DeSpiegeleer et al. (2014) §3.6 pp.95-110.

```
Longstaff-Schwartz pricing flow:

  1. Simulate N share-price + hazard-rate paths forward to T
  2. Initialize V at maturity = max(F, q · S(T))
  3. Backward sweep over time steps:
       - At each step, on paths in-the-money for conversion:
         - Regress V_continue on basis functions of (S, vol-state, ...)
         - If regression-implied continuation < q · S, exercise
       - Update V on paths to chosen exercise / continuation
  4. Discount back; average across paths
```

## Definition

The Monte Carlo pricer for convertibles consists of four building
blocks. **Source:** Glasserman (2003) §1, §8 pp.10-30, 420-450;
DeSpiegeleer et al. (2014) §3.6 pp.95-110.

- **Path simulation**: under a chosen risk-neutral model
  (typically GBM share + intensity-based default; possibly with
  stochastic vol or stochastic intensity), generate `M` paths of
  `(S(t), σ(t), h(t))` from `0` to `T` with step `Δt`. **Source:**
  Glasserman (2003) §1 pp.10-30.
- **Default-event handling**: at each step, draw an independent
  Bernoulli with success probability `h(t_k) · Δt`; if a default
  fires on the path, terminate at the recovery PV `R · F` from
  that step. **Source:** DeSpiegeleer et al. (2014) §3.6 pp.95-110.
- **Backward regression** (Longstaff-Schwartz): at each
  exercise-eligible step, regress the next-step discounted value
  on a basis (e.g., low-order polynomials in `S`, `S²`, `S · σ`)
  using only in-the-money paths; the fitted continuation value is
  compared to immediate exercise. **Source:** Glasserman (2003)
  §8 pp.420-450.
- **Discounting and averaging**: each path's terminal payoff is
  discounted back to time `0`; the average across paths is the
  Monte Carlo estimate of `V(0)`. The standard error scales as
  `O(M^{-1/2})`; variance reduction (control variates,
  importance sampling) recovers an additional constant factor.
  **Source:** Glasserman (2003) §4 pp.150-200.

The tree-vs-MC tradeoff is well-defined. **Source:** DeSpiegeleer
et al. (2014) §3.6 pp.95-110; Glasserman (2003) §8 pp.420-450.

- The CRR tree converges as `O(1/N)` for European payoffs,
  `O(1/√N)` for early-exercise boundaries. **Source:** Hull
  (recent ed.) §27.4 pp.650-653.
- Monte Carlo converges as `O(1/√M)` regardless of payoff
  structure, but supports arbitrary path-dependence at no extra
  state-size cost. **Source:** Glasserman (2003) §1 pp.10-30.
- For canonical optional CBs without soft calls or stochastic
  vol, the tree is faster. For soft-call-trailing-window or
  stochastic-vol convertibles, MC dominates. **Source:**
  DeSpiegeleer et al. (2014) §3.6 pp.95-110.

## Mathematical Reasoning

The Longstaff-Schwartz regression at exercise step `t_k` proceeds
as follows. **Source:** Glasserman (2003) §8 pp.420-450.

```
Backward step at t_k:

  In-the-money paths: I = { ω : q · S^ω(t_k) > exercise threshold }

  Regress on I:
     Y(ω) = e^(-r Δt) · V^ω(t_{k+1})        (next-step discounted PV)
     X(ω) = (1, S^ω(t_k), S^ω(t_k)^2, σ^ω(t_k), ...)  (basis features)

     β̂ = OLS estimator of E[Y | X]

  Decision per in-the-money path ω:
     V_continue^ω(t_k) := β̂ · X(ω)
     if q · S^ω(t_k) > V_continue^ω(t_k):
         V^ω(t_k) := q · S^ω(t_k)            (exercise)
     else:
         V^ω(t_k) := e^(-r Δt) · V^ω(t_{k+1})  (continuation)

  On out-of-the-money paths: V^ω(t_k) := e^(-r Δt) · V^ω(t_{k+1})
```

The **basis function choice** matters: practitioner-quoted choices
for convertibles include monomials `(S, S², S³)` plus interaction
terms with the volatility state and the hazard rate, and the
regression is **low-rank** because the state space is
finite-dimensional and each step uses only the in-the-money paths
to build the basis. **Source:** Glasserman (2003) §8 pp.430-445.

The **soft-call-trigger handling** is the canonical reason to
prefer MC over the tree for convertibles. **Source:** DeSpiegeleer
et al. (2014) §2.5 pp.60-78. At each exercise-eligible step on
each path, evaluate the trigger indicator `K(t)` from the
[call-and-put card](./cb-call-and-put-protection.md#mathematical-reasoning):

```
Soft-call-trigger update on path ω at step t_k:

  K^ω(t_k) := sum_{i=1}^{N} 1{ S^ω(t_k - i+1) ≥ α · K_c }

  if K^ω(t_k) ≥ M:
      issuer-call cap: V^ω(t_k) := min(V^ω(t_k), K(t_k))
```

The path-conditional trigger evaluation is **free** in MC (each
path stores its own history) but expensive in the tree (each
node must be augmented with the trailing count). This is the
canonical motivation for using MC on issues with active soft-call
provisions. **Source:** DeSpiegeleer et al. (2014) §2.5 pp.60-78;
Glasserman (2003) §8 pp.420-450.

The **default-and-recovery handling** in MC mirrors the
default-branch-of-the-tree machinery: a Bernoulli draw at each step
`Δt` with success probability `h(t_k) · Δt` either fires a default
(path terminates at recovery PV) or continues, and the hazard rate
may itself depend on path state (equity-coupled `h(S, t)` or
doubly-stochastic `h(t, X)`). **Source:** Glasserman (2003) §8
pp.420-450; see the
[default-and-recovery card](./cb-default-and-recovery.md#mathematical-reasoning).

Asymptotic behavior (cases below). **Source:** Glasserman (2003)
§1, §8 pp.10-30, 420-450.

- `M → ∞`: MC estimate converges to the PDE-and-free-boundary
  value (cf. the
  [PDE card](./cb-pde-and-free-boundary.md#mathematical-reasoning))
  at rate `O(1/√M)`. **Source:** Glasserman (2003) §1
  pp.10-30.
- `Δt → 0` with `M` fixed: each path becomes a finer
  approximation of the underlying continuous-time process; the
  default-event probability per step `h Δt` shrinks proportionally.
  **Source:** Glasserman (2003) §1 pp.10-30.
- Number of basis functions `K → ∞`: the regression-implied
  continuation value converges to the true conditional
  expectation; standard-error bias from low-rank regression
  shrinks. **Source:** Glasserman (2003) §8 pp.420-450.

## See Also

- [`cb-binomial-tree-valuation.md`](cb-binomial-tree-valuation.md) — discrete-state alternative
- [`cb-pde-and-free-boundary.md`](cb-pde-and-free-boundary.md) — continuum-limit PDE
- [`cb-call-and-put-protection.md`](cb-call-and-put-protection.md) — soft-call trigger machinery
- [`cb-default-and-recovery.md`](cb-default-and-recovery.md) — default-event Bernoulli draw

## Escalate to Raw When

Open Glasserman §1 + §8 pp.10-30 + 420-450 directly for the formal
treatment of risk-neutral path simulation, the Longstaff-Schwartz
regression algorithm, and the convergence and bias properties.
**Source:** Glasserman (2003) §1, §8 pp.10-30, 420-450.

Open DeSpiegeleer §3.6 pp.95-110 for the practitioner's hybrid-
securities Monte Carlo implementation, including the soft-call
path-conditional trigger evaluation. **Source:** DeSpiegeleer et al.
(2014) §3.6 pp.95-110.

Open Hull §27.4 pp.650-653 for the credit-aware tree's
backward-induction recurrence that the Monte Carlo sweep
generalizes via regression-on-paths. **Source:** Hull (recent ed.)
§27.4 pp.650-653.
