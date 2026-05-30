---
schema_version: "cacg.v0"
id: "pa-multilevel-attribution-successive-notional-funds"
title: "Multi-Level Attribution and Successive Notional Funds"
reading_id: "15_performance_and_attribution"
summary: "Multi-level attribution as a chain of successive notional funds (policy -> strategy -> selection), each isolating one decision step; Holbrook's three-level pension framework links the chain to Fama-style risk decomposition."
tags: ["multilevel-attribution", "notional-funds", "decision-tree"]
citations:
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p266:0316"
    chunk_hash: "c9c366db3b41bcc8d68fdd2b98124b091d01c71777e7898da294eb94572240b7"
    page_range: [266, 266]
    quote: "first articulation of successive notional portfolios, each isolating one step in the investment decision process."
    edge_type: "defines"
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p405:0478"
    chunk_hash: "9ff7382d0364887b6a63f9a5eb3e5e18bed9ac24bdf5a98e88dc6172e22a0b49"
    page_range: [406, 406]
    quote: "importantly, the order of the decision process, we can still employ the Brinson model."
    edge_type: "defines"
card_hash: "0735e143364e892483997b920f8a3cc892d893e8bdff67c439044c44c444016c"
---
# Multi-Level Attribution and Successive Notional Funds

## Intuition

A real manager does not make one decision; they make a *chain* of nested decisions: first the strategic split between broad investment markets (equity vs. fixed-interest), then the tilt across asset classes and regions, then the choice of individual securities inside each bucket. Single-level Brinson attribution collapses all of this into two effects. Multi-level attribution instead walks the decision tree one step at a time, building a sequence of **notional funds** that differ from each other by exactly one decision. The performance gap between adjacent notional funds isolates the value added (or destroyed) by that single decision, so the analyst can hand each result to the person who actually owned that decision.

**Source:** Bacon (2023) §6 (Return Attribution) printed pp.241-244 (PDF pp.263-266)

## Definition

Holbrook directed his framework at pension fund trustees and split the management process into three levels: **Policy** (the long-term proportions across markets, fundamentally the equity-vs-fixed-interest split), **Strategy** (deliberate departures from policy in light of current conditions), and **Selection** (the choice of particular investments within each market). To measure the contribution of each, he constructs successive notional funds with returns `rm` (the industry model), `rp` (trustees' policy proportions), `rA` (actual allocations), and `r` (the actual fund). Each adjacent pair shares all decisions except one, so the step between them is "clean."

This generalizes the **fully restrained / partly restrained / actual** language of single-level attribution. The fully restrained fund holds benchmark weights and benchmark returns; the **partly restrained (semi-notional) fund** holds the manager's *actual* weights applied to *index* returns; the actual fund holds actual weights and actual returns. Asset allocation is measured as (partly restrained minus fully restrained); security selection as (actual minus partly restrained). The multi-level extension simply inserts more semi-notional rungs between the model and the actual fund — strategic -> asset-class -> region -> security — so the chain decomposes the total decision process rather than a single layer. Bacon works this for a five-step balanced portfolio, each intermediate step represented by a semi-notional fund, noting that this requires identifying each step and, "importantly, the order of the decision process, we can still employ the Brinson model."

**Sources:** Bacon (2023) §6 (Return Attribution) printed pp.241-244 (PDF pp.263-266) (Holbrook three-level framework, fully/partly restrained funds); printed pp.384-386 (PDF pp.406-408) (multi-level semi-notional rungs)

## Mathematical Reasoning

Bacon presents Holbrook's whole-fund **geometric** excess return `g` as a product of single-decision wealth ratios:

```
                policy step   strategy step   selection step
            +-----------+  +-----------+  +-----------+
(1 + g) =   (1 + rp)        (1 + rA)        (1 + r)
            ---------   x   ---------   x   ---------
            (1 + rm)        (1 + rp)        (1 + rA)

         =  (1 + r) / (1 + rm)        (telescoping product)
```

**Source:** Bacon (2023) §6 (Return Attribution) printed p.244 (PDF p.266) (Eq. 6.4)

The construction is a telescoping product: each rung's denominator cancels the previous rung's numerator, so the chain collapses to the total whole-fund excess `(1 + r)/(1 + rm)`. This identity is what makes the decomposition **complete** — every basis point of total excess return is assigned to exactly one decision with no residual, provided each notional fund's return is itself the sum of its parts (the additivity requirement that fails for the IRR). The first factor is the contribution from choosing a policy that differs from the model; the second, from strategic departures from policy; the third, from selection within sectors. Holbrook then maps these factors onto Fama's decomposition — equating the first to the investor's return from systematic risk, the second to the manager's return from systematic risk, and the third to selectivity — but Bacon presents this linkage as Holbrook's assertion rather than a derived result, so the card asserts it likewise and labels the gap.

**Source:** Bacon (2023) §6 (Return Attribution) printed p.244 (PDF p.266)

## Boundary Notes

The clean telescoping identity above is the **geometric** form; an arithmetic multi-level decomposition does not chain so cleanly, so Bacon notes that if arithmetic excess returns are preferred a smoothing or linking algorithm must be employed to make the effects sum to the total. Multi-level attribution also presupposes a decision tree that mirrors the *actual* investment process — Bacon stresses identifying each step and "importantly, the order of the decision process," so imposing an asset-class -> region -> security order on a manager who decides region-first will mislabel where value was added.

**Sources:** Bacon (2023) §6 (Return Attribution) printed p.244 (PDF p.266) (geometric telescoping); printed pp.360-362 (PDF pp.382-384) (arithmetic smoothing/linking); printed p.384 (PDF p.406) (decision-process ordering)

## See Also

- [`pa-brinson-bhb-allocation-selection-interaction.md`](pa-brinson-bhb-allocation-selection-interaction.md) — the single-level allocation/selection decomposition that the notional-fund chain generalizes.
- [`pa-geometric-attribution-brinson-extended.md`](pa-geometric-attribution-brinson-extended.md) — the geometric semi-notional-fund machinery that makes the telescoping product exact.
- [`pa-geometric-vs-arithmetic-linking-choice.md`](pa-geometric-vs-arithmetic-linking-choice.md) — why geometric excess return chains across steps without an interaction residual.
- [`pa-dgtw-cs-ct-as-decomposition.md`](pa-dgtw-cs-ct-as-decomposition.md) — a returns/characteristic-based multi-component decomposition contrasting with this holdings-based decision-tree view.

## Escalate to Raw When

- You need the worked numeric walk-through showing how successive semi-notional funds produce specific per-step contributions that telescope to the whole-fund excess (Bacon's worked five-step balanced-portfolio multi-level example, Table 6.68 and Figure 6.18).
- You must reconcile the multi-level (returns-based / holdings-based) result to the published portfolio return and quantify the residual, which Bacon discusses grows with manager activity, large cash flows, illiquid assets, and longer measurement periods.
- The investment process has more than three levels (e.g., benchmark -> equity/bond allocation -> region/sector -> industrial sector -> security) and you need the exact ordering convention and intermediate semi-notional-fund definitions for each rung.

**Sources:** Bacon (2023) §6 (Return Attribution) printed pp.244-246 (PDF pp.266-268) (residual reconciliation, holdings-based residual drivers); printed pp.384-386 (PDF pp.406-408) (worked multi-level example, semi-notional-fund per-rung definitions)
