---
schema_version: "cacg.v0"
id: "pa-brinson-bhb-allocation-selection-interaction"
title: "The Brinson Decomposition: Allocation, Selection, and Interaction"
reading_id: "15_performance_and_attribution"
summary: "The Brinson-Hood-Beebower model splits arithmetic excess (r-b) into allocation Ai=(wi-Wi)bi, selection Si=Wi(ri-bi), and interaction Ii=(wi-Wi)(ri-bi), built from a semi-notional and a selection-notional fund. Interaction is a defined cross-term, not a residual."
tags: ["brinson-bhb", "asset-allocation", "attribution"]
citations:
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p271:0324"
    chunk_hash: "4039bbcb6a77e9f649f77201bdc900d4a21c057414d75658e00d360187fb773b"
    page_range: [272, 272]
    quote: "Interaction is not a residual or a balancing item. It is a mathematically defined term – the combination of allocation and selection decisions."
    edge_type: "defines"
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p191:0187"
    chunk_hash: "1136ad5fadf45b9a17be8df8014b5ce84dbefb436d521184229b9beaf8e1eb9e"
    page_range: [192, 192]
    quote: "the allocation and selection effects as defined do not combine to explain the active management effect completely."
    edge_type: "supports"
  - source_id: "pa_colin_2016"
    chunk_id: "pa_colin_2016:p051:0049"
    chunk_hash: "4af54a837ba89f0f4a6e3b158e1fdca096f57fcff412efa74af54049bae16110"
    page_range: [52, 52]
    quote: "3.2 Brinson attribution 3.2.1 Asset allocation The first type of return measured by Brinson attribution is value added by overweighting some sectors and underweighting others."
    edge_type: "supports"
card_hash: "bf0e83181fbda4a9825fe6efd5e74e1918ccc786e89d392997b560659e9ec48b"
---
# The Brinson Decomposition: Allocation, Selection, and Interaction

## Intuition

A portfolio can only differ from its benchmark in two ways: it can hold *different
amounts* of each sector, and within each sector it can hold *different securities*.
Brinson, Hood and Beebower (BHB) make this concrete by asking the manager to act
out a standard investment process — first decide sector weights (allocation), then
pick stocks inside each sector (selection) — and then measuring how much excess
return each decision produced. The trick is to build two halfway "notional" funds
that each freeze one decision while letting the other vary, so the manager's two
bets can be isolated cleanly rather than tangled together.

**Source:** Bacon (2023) §6 (Brinson, Hood and Beebower) printed pp.248-249 (PDF pp.270-271)

The allocation bet is "overweight the sectors you expect to win, underweight the
ones you expect to lose"; the selection bet is "inside each sector, hold the stocks
you expect to beat that sector's index." Christopherson, Cariño and Ferson frame
the same idea as the only logically possible source of active return: at the most
basic level a portfolio's return can differ from the benchmark's only if its
security weights differ. Colin labels the family "Brinson attribution" and stresses
the sector classification must mirror how decisions were actually made.

**Source:** Christopherson, Cariño & Ferson (2009) ch.18 "Performance Attribution" printed pp.177-178 (PDF pp.190-191)

## Definition

Index sectors by `i`. Let `wi`, `ri` be the portfolio sector weight and return, and
`Wi`, `bi` the benchmark sector weight and return, with portfolio return
`r = sum wi*ri` and benchmark return `b = sum Wi*bi`. BHB decompose the *arithmetic*
excess return `(r - b)` using two intermediate funds:

- **Semi-notional (allocation) fund** `bS = sum wi*bi` — actual weights applied to
  *index* returns. It carries the allocation bets but no selection. The allocation
  contribution of sector `i` is the difference from the benchmark:

      Ai = (wi - Wi)*bi          with   sum Ai = bS - b

- **Selection-notional fund** `rS = sum Wi*ri` — benchmark weights applied to *actual*
  returns. It carries selection but no allocation. The selection contribution of
  sector `i` is:

      Si = Wi*(ri - bi)          with   sum Si = rS - b

- **Interaction** is the third term that closes the identity. Its sector
  contribution is the product of the active weight and the active return:

      Ii = (wi - Wi)*(ri - bi)   with   sum Ii = r - rS - bS + b

Bacon is emphatic on what interaction *is*: "Interaction is not a residual or a
balancing item. It is a mathematically defined term – the combination of allocation
and selection decisions."

**Source:** Bacon (2023) §6 eq.6.5-6.17 printed pp.248-250 (PDF pp.270-272)

## Mathematical Reasoning

The decomposition is an algebraic identity, not an approximation. Allocation and
selection alone do not reconstruct the excess return — each is measured against the
benchmark, so summing them double-subtracts `b`:

    Si + Ai  =  (rS - b) + (bS - b)  =  rS + bS - 2b

To recover `r - b` we must add back the missing piece. Writing it out:

    (rS - b) + (bS - b) + (r - rS - bS + b)  =  r - b

so the third bracket — interaction — is forced by the algebra. Christopherson,
Cariño and Ferson make the same point in words: "the allocation and selection
effects as defined do not combine to explain the active management effect
completely," which is exactly why the interaction term appears.

**Source:** Bacon (2023) §6 eq.6.12-6.13 printed p.249 (PDF p.271)

Interaction is genuinely a *product* of the two active bets, which is why it is a
defined cross-term. Substituting the fund definitions and collecting sums:

    r - rS - bS + b = sum wi*ri - sum Wi*ri - sum wi*bi + sum Wi*bi
                    = sum (wi - Wi)*(ri - bi)

so each sector's interaction `Ii = (wi - Wi)*(ri - bi)` is literally the active
weight times the active return. Its sign is therefore determined: an overweight
(`wi > Wi`) in a sector where the manager *also* out-selected (`ri > bi`) earns
positive interaction; out-selecting inside an *underweight* sector earns negative
interaction, because the good selection was applied to too little capital. The
total decomposition

    r - b  =  sum Ai  +  sum Si  +  sum Ii

holds exactly by construction.

**Source:** Bacon (2023) §6 eq.6.14-6.17 printed pp.249-250 (PDF pp.271-272)

```
   Brinson 2x2 quadrant grid (Bacon Figure 6.1)
   axes: WEIGHTS (actual vs passive) x RETURNS (actual vs passive)

                    Returns: actual            Returns: passive
                 +--------------------------+--------------------------+
   Weights:      |  IV  Portfolio           |  II  Semi-notional       |
   actual        |      r  = sum wi*ri         |      bS = sum wi*bi         |
                 +--------------------------+--------------------------+
   Weights:      |  III Selection-notional  |  I   Benchmark           |
   passive       |      rS = sum Wi*ri         |      b  = sum Wi*bi         |
                 +--------------------------+--------------------------+

   Allocation  = II  - I          (vary weights, freeze returns)
   Selection   = III - I          (vary returns, freeze weights)
   Interaction = IV - III - II + I   (the closing cross-term)
   Total       = IV - I  =  r - b
```

**Source:** Bacon (2023) §6 Figure 6.1 printed p.250 (PDF p.272)

## Boundary Notes

This is the *classical* arithmetic, single-period BHB form. The allocation factor
`(wi - Wi)*bi` rewards any overweight in a positive-return sector even when that
sector trails the overall benchmark — the defect the Brinson-Fachler variant
corrects. Whether interaction is reported as its own line or folded into selection
(bottom-up) or allocation (top-down) is a reporting convention, not a change to the
total: Colin notes "stock selection return is just a particular form of aggregated
asset allocation return," and CCF tabulate the top-down/bottom-up merges.

**Sources:** Colin (2016) §3.2.3 (Stock selection) printed p.31 (PDF p.54); Christopherson, Cariño & Ferson (2009) ch.18 (Top-Down vs Bottom-Up; Table 18.2) printed pp.181-182 (PDF pp.194-195)

## See Also

- [`pa-brinson-fachler-benchmark-relative-allocation.md`](pa-brinson-fachler-benchmark-relative-allocation.md) — corrects the BHB allocation factor to (wi-Wi)(bi-b) so reward keys off the overall benchmark.
- [`pa-arithmetic-vs-geometric-excess-return.md`](pa-arithmetic-vs-geometric-excess-return.md) — defines the arithmetic excess (r-b) that this model partitions.
- [`pa-geometric-attribution-brinson-extended.md`](pa-geometric-attribution-brinson-extended.md) — the geometric variant that dissolves the interaction term into the compoundable effects.
- [`pa-multilevel-attribution-successive-notional-funds.md`](pa-multilevel-attribution-successive-notional-funds.md) — generalizes the semi-/selection-notional funds to a nested hierarchy.

## Escalate to Raw When

- A worked numerical attribution table is needed (Bacon's per-sector weights,
  returns, and the resulting allocation/selection/interaction figures) — this card
  states only the symbolic identities.
- You need the full set of intermediate-fund nomenclature (semi-notional vs
  partially restrained fund) and Bacon's preference argument among the names.
- The mandate requires choosing a reporting convention for interaction (separate
  line vs top-down/bottom-up merge) — see CCF Table 18.2 for the exact merges.
