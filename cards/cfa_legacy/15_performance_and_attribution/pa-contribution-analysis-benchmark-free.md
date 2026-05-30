---
schema_version: "cacg.v0"
id: "pa-contribution-analysis-benchmark-free"
title: "Contribution Analysis: Benchmark-Free Absolute-Return Attribution"
reading_id: "15_performance_and_attribution"
summary: "Benchmark-free contribution analysis decomposes a portfolio's total absolute return by individual instrument or instrument type; market-neutral strategies can still be attributed against a customised (zero/cash) benchmark."
tags: ["contribution-analysis", "absolute-return", "attribution"]
citations:
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p401:0474"
    chunk_hash: "a40a355c4725ea786f819df0f0d53959ebaad54fd4f6082ac5159f0e99c1dd07"
    page_range: [402, 402]
    quote: "simply breaks down the total return of the portfolio by individual instrument or instrument"
    edge_type: "defines"
---
# Contribution Analysis: Benchmark-Free Absolute-Return Attribution

## Intuition

Standard performance attribution presupposes a benchmark: it explains *excess* return, the gap between what the manager did and what the policy index did. But absolute-return and market-neutral mandates often have no meaningful index to attribute against — the objective is a positive number, not a number relative to the S&P. For these portfolios the question shifts from "why did we beat the benchmark?" to the more primitive "where did our return come from?" Contribution analysis answers that by attributing the whole total return down to the building blocks — each instrument or instrument type — rather than to allocation/selection effects measured against benchmark weights.

**Source:** Bacon (2023) §6 Contribution Analysis pp.402-402

## Definition

Contribution analysis simply breaks down the total return of the portfolio by individual instrument or instrument type to expose the sources of return inside the portfolio. It is the natural decomposition for portfolios "without benchmarks or absolute return strategies" that "do not readily fit into the definition" of performance attribution, because there is no benchmark against which to attribute.

Two cases sit on a spectrum:

- **No benchmark / pure absolute return:** decompose total return by instrument or instrument-type contribution only. There is no allocation-vs-selection split because there are no benchmark weights to compare against.
- **Implicit zero/cash benchmark:** even when the benchmark return is zero (or cash), standard attribution analysis may still be applied. If the manager runs a deliberate value-adding strategy, that strategy can be converted to a *customised benchmark*, and full attribution computed against it. Market-neutral-type strategies are the canonical example.

**Source:** Bacon (2023) §6 Contribution Analysis (Absolute Return Attribution) pp.402-402

## Mathematical Reasoning

Let the portfolio hold instruments (or instrument types) indexed by $i$, with portfolio weight $w_i$ and instrument return $r_i$. The total portfolio return is the weight-return inner product

$$ R_P \;=\; \sum_i w_i\, r_i, \qquad \sum_i w_i = 1 . $$

Contribution analysis names each addend $c_i = w_i r_i$ as instrument $i$'s **contribution to return**, so the decomposition is the trivially exact identity

$$ R_P \;=\; \sum_i c_i, \qquad c_i = w_i\, r_i . $$

This is an *additive, benchmark-free* identity: it requires no benchmark weights $W_i$ or benchmark returns $b_i$, which is precisely why it survives when no benchmark exists. Contributions can be regrouped by instrument *type* (asset class, sector, long-book vs short-book) by summing the $c_i$ within each type — the partition is exact because the addends are disjoint.

Contrast with benchmark-relative (Brinson-style) attribution, which needs both portfolio and benchmark weights to form an allocation term $(w_i - W_i)\,b_i$ and a selection term $w_i(r_i - b_i)$. When a customised benchmark is *introduced* (e.g. a zero/cash benchmark with $b_i \equiv 0$, or a manager-defined strategy proxy), those benchmark-relative effects re-enter and the absolute-return portfolio can be attributed by the standard machinery — this is the "market-neutral can still be attributed" claim.

The text asserts the decomposition and the customised-benchmark bridge without a formal proof of optimality; this card states the identity and labels the bridge as an asserted modelling choice, not a derived result. The contribution-to-return identity $c_i = w_i r_i$ is Bacon's Equation 3.51 (the contributions sum to the single-period total return); the Brinson allocation term $(w_i - W_i)\,b_i$ is Bacon's Equation 6.7, and the selection term $w_i(r_i - b_i)$ is Bacon's Brinson-and-Fachler "selection including interaction" form.

**Sources:** Bacon (2023) §3 Contribution to Return pp.120-120 (Eq 3.51); §6 Brinson model pp.271-281 (Eqs 6.7, 6.14-6.15); §6 Contribution Analysis (Absolute Return Attribution) pp.402-402

```
            Does a meaningful benchmark exist?
                         |
            +------------+------------+
            | NO                      | YES (incl. zero/cash
            v                         v   or customised strategy)
   Contribution analysis      Standard attribution
   R_P = sum w_i r_i          R_P - R_B split into
   (break total return        allocation (w_i-W_i)b_i
    down by instrument /       + selection w_i(r_i-b_i)
    instrument type)           e.g. market-neutral vs
                               customised benchmark
```
**Sources:** Bacon (2023) §6 Contribution Analysis (Absolute Return Attribution) pp.402-402; §3 Contribution to Return pp.120-120; §6 Brinson model pp.271-281

## See Also

- [`pa-twr-vs-mwr-when-each-applies.md`](pa-twr-vs-mwr-when-each-applies.md) — the total return that contribution analysis decomposes is itself a TWR/MWR measurement choice.
- [`pa-brinson-bhb-allocation-selection-interaction.md`](pa-brinson-bhb-allocation-selection-interaction.md) — the benchmark-relative allocation/selection split that contribution analysis dispenses with when no benchmark exists.
- [`pa-brinson-fachler-benchmark-relative-allocation.md`](pa-brinson-fachler-benchmark-relative-allocation.md) — the customised-benchmark route by which market-neutral strategies re-enter standard attribution.
- [`pa-arithmetic-vs-geometric-excess-return.md`](pa-arithmetic-vs-geometric-excess-return.md) — once a customised benchmark is chosen, excess return must be defined arithmetically or geometrically before attributing.

## Escalate to Raw When

- You need a *worked* contribution table (instrument-by-instrument $w_i r_i$ figures, multi-period contribution-to-excess-return totals) — see the Exhibit 6.111 numeric example on Bacon (2023) p.402, deliberately omitted here per the no-worked-arithmetic rule.
- You need the explicit construction of a *customised benchmark* for a market-neutral or absolute-return mandate (how to convert a stated value-add strategy into proxy weights/returns), which Bacon describes only by example.
- You need contribution geometrically compounded and chain-linked across multiple periods rather than the single-period additive identity shown here.
