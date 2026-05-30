---
schema_version: "cacg.v0"
id: "pa-multiperiod-linking-smoothing-vs-linking"
title: "Multi-Period Linking: Smoothing vs Linking Algorithms"
reading_id: "15_performance_and_attribution"
summary: "Arithmetic attribution effects do not sum across periods (a residual remains); smoothing methods (Carino, Menchero) distribute that residual, while linking methods (GRAP, Frongello) compound it order-dependently."
tags: ["multiperiod-attribution", "smoothing", "linking"]
citations:
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p388:0462"
    chunk_hash: "d06490574f24a23ed8ced7b96d7ec7ca1220286965934f57d3d85628a5f7a6c5"
    page_range: [389, 389]
    quote: "Carino and Menchero are examples of smoothing algorithms in which the natural residual of the multi-period arithmetic attribution is structurally distributed across all the contributions to performance."
    edge_type: "defines"
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p204:0199"
    chunk_hash: "7677222169796785900832c5ba8a2f2fb3c768c404f464df0fc3405eb1184590"
    page_range: [205, 205]
    quote: "Attempting to add returns over time leaves a residual."
    edge_type: "supports"
---
# Multi-Period Linking: Smoothing vs Linking Algorithms

## Intuition

Single-period arithmetic attribution decomposes the excess return into allocation,
selection, and interaction effects that add up exactly within that period. The trouble
starts when you try to summarize a year from its four quarters: returns compound
(multiply) across periods, but the per-period effects were built to add. So if you
naively sum the quarterly effects, they will not reconcile to the annual arithmetic
excess return — a leftover "residual" appears purely as an artifact of mixing additive
within-period bookkeeping with multiplicative across-period growth.

**Source:** Christopherson, Cariño & Ferson (2009) §19 "Linking Attribution Effects" printed pp.191-192 (PDF pp.204-205)

Two families of fixes exist, and they answer one question differently: *where does the
residual go?* **Smoothing algorithms** (Carino, Menchero) spread the residual back
across every contribution so the adjusted effects sum cleanly. **Linking algorithms**
(GRAP, Frongello) instead compound each period's effect through the actual portfolio
returns before it and the benchmark returns after it, absorbing the cross-period growth
directly — at the cost of being order-dependent. Bacon notes these are presentational
conveniences: their only justification is making numbers that "should not add up" add
up, and the choice is usually dictated by the software vendor.

**Source:** Bacon (2023) §6 "Return Attribution" printed pp.362-367 (PDF pp.384-389)

## Definition

**Smoothing algorithm** — a multi-period arithmetic-attribution method in which the
natural residual is *structurally distributed across all the contributions* so the
adjusted per-period effects sum to the total-period arithmetic excess return.

> "Carino and Menchero are examples of smoothing algorithms in which the natural residual of the multi-period arithmetic attribution is structurally distributed across all the contributions to performance."

- **Carino**: introduces a per-period factor $k_t$ and a whole-period factor $k$ built
  from logarithmic returns; scaling each effect by $k_t/k$ makes effects additive.
- **Menchero**: uses a constant scaling factor $M$ plus a Lagrange-optimized per-period
  correction $\alpha_t$ chosen so the linking coefficients $(M + \alpha_t)$ are as
  uniform as possible.

**Linking algorithm** — a method that compounds each single-period effect through the
realized return path rather than redistributing a residual.

- **GRAP**: compounds each period's effect by the actual portfolio return *up to* that
  period and reinvests it at the benchmark return *thereafter*.
- **Frongello**: same compounding concept as GRAP, with the prior-period adjusted
  effects also grown by the current benchmark return.

Both GRAP and Frongello produce the **same total-period effects** and are
**order-dependent**: the linked results change if the order of periods is rearranged.

**Source:** Bacon (2023) §6 "Return Attribution" printed pp.366-373 (PDF pp.388-395)

A key practical drawback of smoothing: the adjusted per-period effects are unique to the
chosen overall window — extending the analysis period forces a recalculation of every
period's revised effect.

**Source:** Bacon (2023) §6 "Return Attribution" printed pp.364,366 (PDF pp.386,388)

## Mathematical Reasoning

The residual exists because the multi-period excess return is a difference of
*compounded* returns, while a naive sum is a sum of *single-period* differences:

$$R - \bar{R} \neq (R_1 - \bar{R}_1) + (R_2 - \bar{R}_2) + \cdots + (R_T - \bar{R}_T)$$

**Source:** Christopherson, Cariño & Ferson (2009) §19 "Linking Attribution Effects" printed pp.191-192 (PDF pp.204-205)

**Carino's smoothing identity.** Continuously compounded returns sum, so with
$k_t = \dfrac{\ln(1+r_t) - \ln(1+b_t)}{r_t - b_t}$ and the whole-period analogue
$k = \dfrac{\ln(1+r) - \ln(1+b)}{r - b}$, the arithmetic excess return decomposes
additively as

$$r - b = \sum_{t=1}^{n} \frac{k_t}{k}\,(A_t + S_t + I_t),$$

where $A_t, S_t, I_t$ are the period-$t$ allocation, selection, and interaction effects.
The residual is absorbed into the $k_t/k$ weights — it is *distributed*, not compounded.

**Source:** Bacon (2023) §6 "Return Attribution" printed pp.362-365 (PDF pp.384-387)

**GRAP linking identity.** Writing $a_t = r_t - b_t$, the two-period excess return
expands to reveal the cross-product structure:

$$ (r - b) \;=\; a_1 (1 + b_2) + (1 + r_1)\, a_2, $$

generalizing over $n$ periods so that each effect is compounded by the actual portfolio
return before it and the benchmark return after it:

$$r - b = \sum_{T=1}^{n} (A_T + S_T + I_T)\!\left(\prod_{t=1}^{T-1}(1+r_t)\right)\!\left(\prod_{t=T+1}^{n}(1+b_t)\right).$$

Because $r_t$ precedes and $b_t$ follows in fixed position, reordering periods changes
the products — hence order-dependence.

**Source:** Bacon (2023) §6 "Return Attribution" printed pp.367-369 (PDF pp.389-391)

**Geometric attribution compounds naturally.** Geometric excess return links through
time without any residual or fix-up, and geometric attribution effects inherit this:

$$\prod_{t=1}^{n}(1 + S_t^{G}) \times \prod_{t=1}^{n}(1 + A_t^{G}) - 1 = g,$$

so multi-period geometric attribution "does not suffer the same linking challenges" as
its arithmetic counterpart. Bacon asserts this from the geometric-compounding property
established earlier rather than re-deriving it here.

**Source:** Bacon (2023) §6 "Multi-period geometric attribution" printed p.376 (PDF p.398)

```
                 Multi-period arithmetic excess return
                  (effects DON'T sum -> residual left)
                              |
              +---------------+----------------+
              |                                |
        SMOOTHING                          LINKING
   (distribute residual)            (compound through path)
        /         \                      /         \
     Carino     Menchero             GRAP        Frongello
   k_t/k       M + alpha_t       prod(1+r) /     same totals,
   weights     (Lagrange)        prod(1+b)       order-dependent
        \         /                      \         /
   window-specific;                 path-dependent;
   recompute if period               same total as
   extended                          each other

   Geometric effects:  prod(1+S^G)*prod(1+A^G) - 1 = g
                        (compound naturally, no residual)
```

**Source:** Bacon (2023) §6 "Return Attribution" printed pp.366-376 (PDF pp.388-398)

## Boundary Notes

These algorithms are described by Bacon as "black box" presentational conveniences with
"no economic justification other than presentational convenience." The card therefore
asserts their mechanics and their order-dependence / window-dependence as the source
states them, without claiming any one method recovers a uniquely correct economic
decomposition. The Davies-Laker compounded-notional-funds method (an evolution stage
between arithmetic and geometric) is covered under multilevel/notional-fund attribution.

**Source:** Bacon (2023) §6 "Return Attribution" printed pp.362,373 (PDF pp.384,395)

## See Also

- [`pa-arithmetic-vs-geometric-excess-return.md`](pa-arithmetic-vs-geometric-excess-return.md) — why geometric excess return links naturally while arithmetic leaves a residual.
- [`pa-geometric-vs-arithmetic-linking-choice.md`](pa-geometric-vs-arithmetic-linking-choice.md) — the practitioner choice that smoothing-vs-linking only arises for under arithmetic effects.
- [`pa-multilevel-attribution-successive-notional-funds.md`](pa-multilevel-attribution-successive-notional-funds.md) — Davies-Laker compounded notional funds as a partly-geometric linking route.
- [`pa-true-twr-and-chain-linking.md`](pa-true-twr-and-chain-linking.md) — chain-linking of returns, the multiplicative property that creates the linking problem.

## Escalate to Raw When

- You need the worked four-quarter example reconciling the Carino, Menchero, GRAP, and
  Frongello revised effects to the 13.27% annual arithmetic excess return (Bacon Tables
  6.60-6.64) — including the numerical $k_t$, $M$, $\alpha_t$, and compounded factors.
- You need Menchero's exact Lagrange-multiplier derivation of $\alpha_t$ or the
  closed-form correction term.
- You need the Davies-Laker exact-attribution numbers or the logarithmic-linking-
  coefficient equivalence proof in Christopherson, Cariño & Ferson §19.
