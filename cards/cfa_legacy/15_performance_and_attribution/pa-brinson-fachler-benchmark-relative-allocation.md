---
schema_version: "cacg.v0"
id: "pa-brinson-fachler-benchmark-relative-allocation"
title: "Brinson-Fachler: Benchmark-Relative Allocation"
reading_id: "15_performance_and_attribution"
summary: "Brinson-Fachler refines BHB allocation to (wi-Wi)(bi-b), so an overweight adds value only when the sector beats the OVERALL benchmark, not merely when its return is positive; the redefinition sums to zero across sectors."
tags: ["brinson-fachler", "asset-allocation", "attribution"]
citations:
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p277:0332"
    chunk_hash: "b9c45c0a49edf6d5ef7fddc802282ae99737e39d225011e8c41e40bd744be87e"
    page_range: [277, 278]
    quote: "The Brinson and Fachler model solves this problem by modifying the asset allocation factor"
    edge_type: "defines"
---
# Brinson-Fachler: Benchmark-Relative Allocation

## Intuition

The Brinson-Hood-Beebower (BHB) allocation factor `(wi - Wi) * bi` rewards any
overweight in a sector with a positive return, irrespective of how that sector
fared against the rest of the portfolio's opportunity set. But that is not how an
asset allocator actually thinks: the allocator is not trying to be overweight in
markets that merely rise, but overweight in markets that *outperform the overall
benchmark*. An overweight in a positive-return sector that nevertheless lagged the
total benchmark has destroyed relative value, yet BHB books it as a positive
allocation contribution. Brinson-Fachler (BF) repairs this by measuring each
sector's return against the overall benchmark return `b` rather than against zero.

**Source:** Bacon (2023) §6 (Brinson and Fachler) pp.277-278

## Definition

Brinson-Fachler redefines the allocation contribution of sector `i` so that the
relevant comparison is the sector benchmark return `bi` against the *overall*
benchmark return `b`:

> Ai = (wi - Wi) * (bi - b)

where `wi` is the portfolio weight, `Wi` the benchmark weight, `bi` the sector
benchmark return, and `b` the total benchmark return. Selection `Wi*(ri - bi)` and
interaction `(wi - Wi)*(ri - bi)` are unchanged from BHB — only the allocation
factor is modified, so the total excess return decomposition is preserved. Bacon
states the BF model "solves this problem by modifying the asset allocation factor
to compare returns against the overall benchmark".

**Source:** Bacon (2023) §6 eq.6.18-6.19 pp.278

## Mathematical Reasoning

The BF and BHB allocation factors agree in aggregate because the cross term that
distinguishes them vanishes on summation. Expanding the BF factor:

> (wi - Wi)*(bi - b) = (wi - Wi)*bi - (wi - Wi)*b

Summing over all `i` and using the weight identities `sum wi = sum Wi = 1`:

> sum (wi - Wi)*b = b * (sum wi - sum Wi) = b * (1 - 1) = 0

so the second term sums to zero and the total allocation effect is identical to
BHB's `sum (wi - Wi)*bi`. The redefinition is therefore a pure *reattribution
across sectors*, not a change in the total: each sector's allocation reward is now
sign-correct relative to the overall benchmark, while the sum-to-zero property
guarantees the top-level excess return is preserved.

Sign logic follows directly. An overweight (`wi - Wi > 0`) earns positive
allocation iff the sector beats the benchmark (`bi - b > 0`); an underweight
(`wi - Wi < 0`) earns positive allocation iff the sector lags the benchmark
(`bi - b < 0`). A positive-but-lagging sector held overweight now correctly books
a *negative* allocation effect — the case BHB mishandles.

**Source:** Bacon (2023) §6 eq.6.18-6.21 pp.278

```
            BF allocation grid: reward only for benchmark-relative bets
            (overweight rewarded ONLY when sector beats overall benchmark b)

                    bi - b > 0                 bi - b < 0
                 (sector beats b)          (sector lags b)
   wi - Wi > 0 |   +  allocation       |    -  allocation       |
   (overweight)|   (correct bet)       |   (overweight a loser) |
   ------------+-----------------------+------------------------+
   wi - Wi < 0 |   -  allocation       |    +  allocation       |
   (underweight)|  (missed a winner)   |   (avoided a loser)    |

   BHB instead keys off bi vs 0 (positive return), rewarding ANY overweight
   in a positive sector even when it trails the overall benchmark b.
```

**Source:** Bacon (2023) §6 Figure 6.6 pp.278

## See Also

- [`pa-brinson-bhb-allocation-selection-interaction.md`](pa-brinson-bhb-allocation-selection-interaction.md) — the BHB base model whose allocation factor BF corrects.
- [`pa-arithmetic-vs-geometric-excess-return.md`](pa-arithmetic-vs-geometric-excess-return.md) — the arithmetic excess return that this decomposition partitions.
- [`pa-geometric-attribution-brinson-extended.md`](pa-geometric-attribution-brinson-extended.md) — geometric variant that extends the same allocation/selection split.
- [`pa-multilevel-attribution-successive-notional-funds.md`](pa-multilevel-attribution-successive-notional-funds.md) — nesting BF allocation across hierarchy levels.

## Escalate to Raw When

- A worked numerical attribution table is needed (e.g. Bacon's Table 6.4 example
  with per-sector weights, returns, and the resulting allocation/selection/
  interaction figures) — this card states only the symbolic identities.
- You need the graphical area-decomposition (Figure 6.2 vs Figure 6.6) showing how
  the allocation rectangle reshapes from `(wi - Wi)*bi` to `(wi - Wi)*(bi - b)`.
- The treatment of interaction (whether to report it separately or fold it into
  selection) matters for the mandate's reporting convention.
