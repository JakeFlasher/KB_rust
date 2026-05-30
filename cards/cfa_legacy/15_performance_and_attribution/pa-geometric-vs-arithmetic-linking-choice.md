---
schema_version: "cacg.v0"
id: "pa-geometric-vs-arithmetic-linking-choice"
title: "The Geometric-vs-Arithmetic Linking Choice"
reading_id: "15_performance_and_attribution"
summary: "Return contributions compound additively across markets but geometrically over time; the cross-term forces an arithmetic-vs-geometric rescaling to path-independence, with Carino's k=ln(1+r)/r as the canonical arithmetic bridge."
tags: ["multiperiod-linking", "smoothing-algorithms", "path-independence"]
citations:
  - source_id: "pa_colin_2016"
    chunk_id: "pa_colin_2016:p094:0098"
    chunk_hash: "30db2a9ea4eef3c8d2c00cf24039e30eab19a854aefd825d038038f359a5f06f"
    page_range: [95, 95]
    quote: "Arithmetic: numbers add up, but compounding over time is lost"
    edge_type: "defines"
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p383:0456"
    chunk_hash: "69bf14e0a799ddeb9ee6629b5565771fe5af3e56deb70eed4e6e1a7a8bacbd77"
    page_range: [384, 384]
    quote: "or linking algorithm must be employed; there is no economic justification other than presentational convenience."
    edge_type: "supports"
---
# The Geometric-vs-Arithmetic Linking Choice

## Intuition

Single-period attribution is clean: across sectors or risk sources, contributions
simply add up to the period's active return. Trouble starts the moment you string
periods together. Returns combine *additively across a portfolio* but
*multiplicatively over time*, and those two aggregation rules do not commute — so
the nicely-adding single-period effects refuse to sum to the multi-period
total. You are then forced to choose how to rescale (smooth) the contributions so
they once again add up: keep additivity and sacrifice compounding (arithmetic), or
keep compounding and sacrifice additivity (geometric). There is no third option
that preserves both.

**Source:** Colin (2016) §5.1, §5.3 pp.92-95

## Definition

**Cross term.** A security earning r1 then r2 over two intervals has total return
(1 + r1)(1 + r2) - 1 = r1 + r2 + r1*r2. The product r1*r2 is the *cross term*; it is
exactly what prevents a compounded return from decomposing into an arithmetic sum
of its period returns.

**The linking choice.** When the adjusted contributions r1, r2 must recombine to the
total R, you elect one of two algebras:

- *Arithmetic linking:* R = r1 + r2. "numbers add up, but compounding over time is
  lost."
- *Geometric linking:* R = (1 + r1)(1 + r2) - 1. Compounding is retained, but
  additivity over individual samples is lost.

**Path-independence.** Either way, all contributions are rescaled so they aggregate
to the correct totals *regardless of the order* in which they are combined — by
sector, by risk source, or by time. That order-invariance is what "path-independent"
means, and it is the property an attribution report needs to be self-consistent.
Bacon stresses that the smoothing/linking step is "a presentational convenience to
ensure that numbers that should not add up, in fact do add up": if arithmetic excess
returns are preferred, then a smoothing "or linking algorithm must be employed;
there is no economic justification other than presentational convenience."

**Source:** Colin (2016) §5.1.1, §5.3 pp.92-95; Bacon (2023) Ch.6 pp.384

## Mathematical Reasoning

The non-commutativity is the whole story. Aggregating returns over time is a
*product* of (1 + r) gross factors, while aggregating across a portfolio at a single
date is a *weighted sum* of contributions c = w*r. Because product-then-sum need not
equal sum-then-product, the security- or risk-level effects compounded over T
periods generally do not reconcile to the portfolio's compounded outperformance —
this is asserted via the two-period cross-term identity, not proved in general, and
the card carries that assertion no further than the source does.

The canonical *arithmetic* bridge is the Carino factor. Define, over the whole
interval and over each interval t (for rP != rB),

    k  = [ ln(1 + rP) - ln(1 + rB) ] / (rP - rB)
    kt = [ ln(1 + rPt) - ln(1 + rBt) ] / (rPt - rBt)

with k = kt = 1 in the degenerate equal-return case. Scaling every period-t
contribution by kt / k makes the smoothed contributions sum across all periods to
the true compounded active return. The k = ln(1 + r)/r form is what linearises the
log-return identity: continuously compounded returns *do* sum over time, so

    ln(1 + r) = sum_t ln(1 + rt),   ln(1 + b) = sum_t ln(1 + bt)

and subtracting gives ln(1 + r) - ln(1 + b) = sum_t kt*(rt - bt), which the entire-period
factor k then transforms back into the arithmetic excess r - b = sum_t (kt/k)*(rt - bt).
The *geometric* route instead rescales each contribution so the gross factors
(1 + c~i) multiply to (1 + r), trading additivity for exact multiplicative closure.

**Source:** Colin (2016) §5.1.1, §5.4 pp.92-98; Bacon (2023) Eq.6.153-6.159 pp.384-385

```
              FORCED LINKING CHOICE (multi-period contributions)
              ----------------------------------------------------
  single-period truth:   contributions ADD across markets  (sum w*r = R_t)
  multi-period truth:    periods MULTIPLY over time         (prod(1+R_t)-1 = R)
                                  |
                  cross term r1*r2 breaks reconciliation
                                  |
            +---------------------+---------------------+
            |                                           |
      ARITHMETIC link                            GEOMETRIC link
      R = r1 + r2                                R = (1+r1)(1+r2)-1
      keep: additivity                          keep: compounding
      lose: compounding                         lose: additivity
      bridge: Carino k = ln(1+r)/r              bridge: rescale prod(1+c~)=1+r
            \___________________ both rescale to ___________________/
                              PATH-INDEPENDENCE
                 (same total by sector, by risk, or by time)
```

**Source:** Colin (2016) §5.3-§5.5 pp.95-100

## Boundary Notes

The arithmetic/geometric *linking* choice (this card) is the multi-period
reconciliation mechanism; it is distinct from, though motivated by, the
arithmetic-vs-geometric *excess-return* definition itself. The smoothing factors are
interval-dependent, so an ad-hoc report over an arbitrary window must recompute them
from stored unsmoothed data — the link is not free to re-slice.

**Source:** Colin (2016) §5.4 pp.98-99

## See Also

- [`pa-multiperiod-linking-smoothing-vs-linking.md`](pa-multiperiod-linking-smoothing-vs-linking.md) — the parent problem this choice solves: why single-period effects refuse to chain.
- [`pa-arithmetic-vs-geometric-excess-return.md`](pa-arithmetic-vs-geometric-excess-return.md) — the upstream excess-return definition that motivates which algebra you must link in.
- [`pa-geometric-attribution-brinson-extended.md`](pa-geometric-attribution-brinson-extended.md) — geometric attribution, where the multiplicative link is native and no smoothing is needed.
- [`pa-true-twr-and-chain-linking.md`](pa-true-twr-and-chain-linking.md) — chain-linking of sub-period TWRs, the return-level analogue of contribution linking.

## Escalate to Raw When

- You need the worked four-quarter Carino example (the period kt values, the
  kt/k multipliers, and the smoothed per-quarter contributions reconciling to the
  4.21% active return) — Colin Tables 5.4-5.11 and Bacon Table 6.60.
- You must implement geometric smoothing numerically, including the
  |ci|-weighted exponent and the worked Q1 contributions — Colin §5.5 Eq.5.7-5.8,
  Table 5.12.
- You need the named arithmetic-smoothing alternatives (Menchero, GRAP, Davies &
  Laker, Frongello) and their trade-offs — Colin §5.4 and Bacon Ch.6.
