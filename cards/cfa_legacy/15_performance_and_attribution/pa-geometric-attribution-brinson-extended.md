---
schema_version: "cacg.v0"
id: "pa-geometric-attribution-brinson-extended"
title: "Geometric Attribution: The Compounding Brinson Extension"
reading_id: "15_performance_and_attribution"
summary: "Geometric Brinson extension: allocation (1+bi)/(1+b) and selection effects with the 1/(1+bS) term COMPOUND, not sum, via (1+SG)(1+AG)-1 = (1+r)/(1+b)-1 to the geometric excess return."
tags: ["geometric-attribution", "brinson", "excess-return"]
citations:
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p283:0339"
    chunk_hash: "797aa8eeb1928aad27d727f403289ef87185a5dc6d933f1aed81dbb69f25635a"
    page_range: [283, 284]
    quote: "The total stock selection and asset allocation effects compound together to produce the"
    edge_type: "defines"
---
# Geometric Attribution: The Compounding Brinson Extension

## Intuition

Arithmetic Brinson attribution asks "how many percentage points did each decision
add?" and the answers SUM. Geometric attribution asks a multiplicative question:
"by what factor did each decision scale the wealth ratio?" — and the answers
COMPOUND. The reason is that geometric excess return itself is a ratio,
(1+r)/(1+b)-1, not a difference r-b. To decompose a ratio you split it into a
product of sub-ratios, so the natural arithmetic of geometric attribution is
multiplication, not addition.

Concretely, the same intermediate semi-notional fund used in the standard Brinson
method (active weights applied to benchmark category returns) splits the total
wealth ratio into two stages: benchmark -> semi-notional captures the asset-allocation
decision, and semi-notional -> portfolio captures the stock-selection decision.
Multiplying those two stage-ratios telescopes back to the total geometric excess
return.

**Source:** Bacon (2023) §6 (Geometric Excess Return Attribution) pp.282-284

## Definition

For categories i with portfolio weights w_i, benchmark weights W_i, portfolio
category returns r_i, and benchmark category returns b_i, define the aggregate
quantities r (portfolio return), b (benchmark return), and the semi-notional return
bS = sum_i w_i * b_i.

- Geometric excess return: g = (1+r)/(1+b) - 1.
- Total geometric asset allocation: A_G = (1+bS)/(1+b) - 1.
- Total geometric stock selection: S_G = (1+r)/(1+bS) - 1.
- Per-category allocation: A_G,i = (w_i - W_i) * ( (1+b_i)/(1+b) - 1 ).
- Per-category selection: S_G,i = w_i * ( (1+r_i)/(1+b_i) - 1 ) * ( (1+b_i)/(1+bS) ).

The defining structural fact is that the two total effects do not add to g; they
compound to it.

**Source:** Bacon (2023) §6 (Asset allocation / Stock selection) pp.282-284

The per-category selection formula carries an "unexpected" factor (1+b_i)/(1+bS)
that has no arithmetic analogue. Bacon explains it as a weighting correction:
outperformance in a category whose benchmark is already performing well adds more
value geometrically than the equivalent outperformance in a weak-benchmark
category. Bacon labels the full per-category derivations as proved in his
Appendix A rather than re-deriving them in the section, so this card asserts the
identities and points to that appendix for the formal proof.

**Source:** Bacon (2023) §6 (Stock selection) pp.283

## Mathematical Reasoning

The compounding identity is a telescoping product. Insert the semi-notional return
bS as a pivot in the total wealth ratio:

```
                  semi-notional pivot bS
                          |
        benchmark   b --------->  bS  --------->  r   portfolio
                    \____________/    \__________/
                     allocation stage   selection stage
                     (1+bS)/(1+b)        (1+r)/(1+bS)

   (1+bS)     (1+r)        (1+r)
   ------  x  ------   =   ------          (semi-notional cancels)
   (1+b)      (1+bS)       (1+b)
```

**Source:** Bacon (2023) §6 (Equation 6.31) pp.283

Restating the same product in effect-notation gives the headline identity:

```
   (1 + S_G) x (1 + A_G) - 1  =  (1+r)/(1+b) - 1  =  g
```

so the geometric selection and allocation effects are multiplicative factors on
the wealth ratio. This contrasts with arithmetic attribution, where the
corresponding statement is additive: allocation + selection (+ interaction) = r - b.

**Source:** Bacon (2023) §6 (Equation 6.32) pp.284

Two structural consequences follow without any worked arithmetic:

- Per-category vs total aggregation differs between the two layers. Bacon notes the
  individual category allocations SUM to the total geometric allocation A_G, so the
  allocation layer is formally a "mixed geometric-arithmetic" method; a pure
  geometric model would instead have the per-category allocations COMPOUND to A_G.
  The compounding claim of this card therefore applies strictly to the two TOTAL
  effects (S_G and A_G), not necessarily to the per-category pieces.
- The per-category selection identity simplifies, when the (1+b_i)/(1+bS) weighting
  is folded in, to S_G,i = w_i * (r_i - b_i)/(1+bS) — the arithmetic category
  difference divided by the semi-notional fund — which is the form preferred by
  Burnie, Knowles and Teder.

**Source:** Bacon (2023) §6 (Equations 6.25-6.29) pp.283

## Boundary Notes

The compounding identity is exact only for a single measurement period in which the
disaggregation condition holds (category weights and returns reconcile to the total).
It says nothing about how single-period geometric effects link across multiple
periods; that multiperiod question, and the smoothing-vs-linking choice it forces,
is a separate concern handled by the linking cards below. The relationship to the
arithmetic decomposition is a per-effect rescaling by a benchmark-type denominator,
with the two layers carrying different denominators. Geometric selection is the
arithmetic selection divided by (1+bS): since (1+b_i)/(1+b_i) cancels in Equation
6.28, the per-category effect reduces to w_i*(r_i - b_i)/(1+bS), so a positive
semi-notional return bS makes that denominator exceed 1 and damps selection toward
zero. Per-category allocation is correspondingly the arithmetic Brinson-Fachler
allocation (w_i - W_i)*(b_i - b) divided by (1+b), since (1+b_i)/(1+b) - 1 equals
(b_i - b)/(1+b). Bacon's reading of these denominators is uniform: when the total
benchmark return is positive the geometric excess return is less than the arithmetic
excess return, and the geometric allocation and selection contributions are of the
same order but slightly less than their arithmetic counterparts, while "The sign will
always be the same."

**Source:** Bacon (2023) §6 (Geometric Excess Return Attribution) pp.282-284

## See Also

- [`pa-brinson-bhb-allocation-selection-interaction.md`](pa-brinson-bhb-allocation-selection-interaction.md) — the arithmetic parent model whose additive allocation/selection/interaction terms this card converts to compounding factors.
- [`pa-arithmetic-vs-geometric-excess-return.md`](pa-arithmetic-vs-geometric-excess-return.md) — defines the (1+r)/(1+b)-1 ratio that this decomposition splits.
- [`pa-geometric-vs-arithmetic-linking-choice.md`](pa-geometric-vs-arithmetic-linking-choice.md) — extends single-period geometric effects across periods, where compounding stays exact.
- [`pa-brinson-fachler-benchmark-relative-allocation.md`](pa-brinson-fachler-benchmark-relative-allocation.md) — the benchmark-relative allocation refinement that the geometric (1+b_i)/(1+b) form mirrors.

## Escalate to Raw When

- You need the numeric worked example (Bacon's Exhibits 6.7-6.9 and Table 6.3 data:
  UK/Japan/US equity allocation and selection effects reconciling to a 1.79%
  geometric excess return) — those plug-and-chug figures are deliberately omitted
  here per Critical Rule 1.
- You need the formal per-category derivations, including why the (1+b_i)/(1+bS)
  selection factor arises — Bacon defers these to his Appendix A.
- You must reconcile attribution under intra-period cash flows, where Dietz-type
  returns can mis-split effects between allocation and selection; see Bacon's Sector
  Weights discussion and Tables 6.8-6.9 (pp.285+).
