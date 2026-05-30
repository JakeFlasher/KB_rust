---
schema_version: "cacg.v0"
id: "pa-return-gap-kacperczyk-sialm-zheng"
title: "The Return Gap"
reading_id: "15_performance_and_attribution"
summary: "The return gap is the realised net return minus the holdings-implied gross return; it isolates the value (positive) or destruction (negative) of a manager's unobserved interim actions and hidden trading costs not captured by disclosed end-of-period holdings."
tags: ["return-gap", "holdings-based", "unobserved-actions"]
citations:
  - source_id: "pa_fischer_wermers_2013"
    chunk_id: "pa_fischer_wermers_2013:p138:0170"
    chunk_hash: "fdeaee410f2c4b5b931d7c88807045833c8a7a99c5ddd03e2ee416a6809a598e"
    page_range: [139, 139]
    quote: "It also allows an analysis of the missing performance due"
    edge_type: "defines"
---
# The Return Gap

## Intuition

A fund discloses its holdings only periodically (e.g., quarter-end), but it trades
continuously in between. Two windows therefore exist onto the same fund: one
built from the **disclosed holdings** (a hypothetical buy-and-hold of the last
reported portfolio at gross security returns) and one from the **realised net
return** actually delivered to shareholders. If managers added value through
interim trades the disclosure missed — or destroyed it through sloppy execution,
window dressing, or hidden bets — these two windows disagree. The return gap is
exactly that disagreement, and it earns its name because it measures the
"missing performance due to unobserved actions" that neither a pure
holdings-based nor a pure returns-based measure can see alone.

**Source:** Fischer & Wermers (2013) §5 (Abstract) p.139

## Definition

Let the **holdings-implied gross return** be the portfolio-weighted gross
buy-and-hold return on the fund's most recently disclosed stockholdings, and let
the **realised net return** be the return actually earned by shareholders (after
trading costs and fees). The return gap is their difference:

> Return gap = (realised net return) - (holdings-implied gross return)

A *positive* gap means the fund delivered more (or lost less) than its stale
disclosed holdings would predict — evidence of skilled, unobserved interim
trading that added value net of costs. A *negative* gap signals hidden value
leakage: high trading costs, window dressing, or unprofitable interim bets that
the disclosed holdings could not reveal. Kacperczyk, Sialm and Zheng (KSZ;
2008) introduced the gap as a forward-looking skill signal: the difference
between the portfolio-holdings and net-returns approaches "can help to capture
manager skills and predict future performance."

**Source:** Fischer & Wermers (2013) §5.1 Introduction pp.139-140

## Mathematical Reasoning

The gap is a **decomposition residual**, not an independent measure. Place the
two evaluation lenses side by side. Holdings-based evaluation decomposes the
gross return on disclosed stocks into characteristic selectivity, style timing,
and style components; returns-based evaluation benchmark-adjusts the net return
shareholders receive. The return gap is what remains when the gross-holdings lens
is subtracted from the net-return lens — by construction it captures everything
the holdings snapshot omits: the value of interim trades net of transaction costs
and fees.

```
   Holdings lens (disclosed, gross)        Returns lens (realised, net)
   +----------------------------+          +--------------------------+
   | Characteristic selectivity |          |                          |
   | + Style timing             |          |   Net return to          |
   | + Style                    |          |   shareholders           |
   +----------------------------+          +--------------------------+
            R_gross^holdings                       R_net^realised
                     \                              /
                      \                            /
                       v                          v
                  RETURN GAP = R_net^realised - R_gross^holdings
                  (unobserved interim actions - trading costs - fees)
```

Because the two lenses are measured over the same fund and period, the identity
is symbolic and exact given its inputs; the book defines the gap as a difference
and asserts its predictive content from KSZ's empirical study without re-deriving
that result here. The accompanying decomposition (gross-return components, plus a
separate estimate of trading costs and the expense ratio) is developed elsewhere
in the chapter; this card states the residual identity and labels the empirical
predictive claim as asserted, not proved.

**Source:** Fischer & Wermers (2013) §5.2 Performance-Decomposition Methodology pp.140-141

## See Also

- [`pa-dgtw-cs-ct-as-decomposition.md`](pa-dgtw-cs-ct-as-decomposition.md) — supplies the holdings-implied gross-return side (CS/CT) of the gap.
- [`pa-transaction-based-attribution-and-trading-cost.md`](pa-transaction-based-attribution-and-trading-cost.md) — the trading-cost term that drives much of a negative gap.
- [`pa-luck-vs-skill-fdr-and-bootstrap.md`](pa-luck-vs-skill-fdr-and-bootstrap.md) — whether a measured gap reflects true skill or sampling noise.
- [`pa-multifactor-alpha-timing-conditional.md`](pa-multifactor-alpha-timing-conditional.md) — the returns-based net-alpha lens that the gap differences against the holdings lens.

The gap is the holdings-vs-realised analogue of the implementation shortfall
studied under microstructure (see `mt-implementation-shortfall`): both measure
value lost between a paper portfolio and what was actually achieved.

## Escalate to Raw When

- You need the KSZ point estimates (e.g., the basis-points-per-month under- and
  out-performance of large negative vs. positive return-gap funds) — read the
  source's empirical results rather than reproducing numbers here.
- You must specify the exact buy-and-hold gross-return construction or the
  trading-cost estimation procedure feeding each side of the gap.
- You are extending the gap to non-equity sectors (e.g., the fixed-income
  adaptation by Cici and Gibson) and need the sector-specific benchmark
  definitions.
