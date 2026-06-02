---
schema_version: "cacg.v0"
id: "rm-raroc-risk-adjusted-return-on-capital"
title: "RAROC / RORAC: Risk-Adjusted Performance on Economic Capital"
reading_id: "11_risk_management"
summary: "RAROC measures a business unit's return per unit of economic capital — (revenues − costs − expected losses) / economic capital — with the RORAC naming nuance (it is the denominator that is risk-adjusted) and an ex-ante-vs-ex-post use split, per Hull Ch.28.8."
tags: ["risk-management", "raroc", "economic-capital"]
citations:
  - source_id: "rm_hull_2023_rmfi"
    chunk_id: "rm_hull_2023_rmfi:p640:0886"
    chunk_hash: "2dc18f8185e6e58f8d90234746602f61968e5f1d9703658887e69f8e5de7a334"
    page_range: [641, 641]
    quote: "RAROC can be calculated ex-ante (before the start of the year) or ex-post (after the end of the year)."
    edge_type: "defines"
card_hash: "c0df236bff5edfd482750f2174181202d74c6244b108c675e0582984b5d81b98"
---
# RAROC / RORAC: Risk-Adjusted Performance on Economic Capital

## Intuition
Once a bank allocates economic capital to its business units, it needs a way to judge
which units actually *earn their keep* on the risk capital they consume. A raw return
number is misleading: a unit can post a high profit by quietly taking on much more
risk. **RAROC** (risk-adjusted return on capital) fixes this by dividing return by the
economic capital the unit was allocated, so two units are compared on a common,
risk-normalized footing. It is the central tool of risk-adjusted performance
measurement (RAPM): expand units with high RAROC, contract those with low RAROC.

```
   unit P&L (risk-adjusted numerator)
   ───────────────────────────────────  = RAROC
        economic capital (denominator)

   high RAROC ─► expand        low RAROC ─► contract / discontinue
```

**Source:** Hull (2023) Ch.28 §28.8 printed pp.611–612 (PDF pp.639–640).

## Definition
- **RAROC.** The most common risk-adjusted performance measure:

      RAROC = (Revenues − Costs − Expected losses) / Economic capital.

  Expected losses are subtracted in the numerator (they are not capital-absorbed),
  and economic capital — the unit's unexpected-loss buffer — is the denominator.
- **RORAC naming nuance.** Matten (2000) notes it is more accurate to call this
  *RORAC* (return on risk-adjusted capital), because it is the *capital* (the
  denominator) that is risk-adjusted, not the return. True RAROC would adjust the
  numerator for risk; in practice "RAROC" labels a wide range of return-on-capital
  calculations.
- **Ex-ante vs ex-post.** Ex-ante RAROC (before the year) uses estimated expected
  profit and drives expand/contract decisions; ex-post RAROC (after the year) uses
  actual profit and drives performance evaluation and bonuses.

**Source:** Hull (2023) Ch.28 §28.8 printed pp.611–613 (PDF pp.639–641).

## Mathematical Reasoning
The measure is a ratio of a (risk-adjusted) flow to a (risk-adjusted) stock:

    RAROC = [ Revenues − Costs − E(loss) ] / EconCap.

Subtracting E(loss) in the numerator and using EconCap = q_X − E(loss) in the
denominator is what makes the ratio comparable across units of differing tail risk:
two units with identical accounting profit but different loss-distribution shapes get
different EconCap and hence different RAROC. The semantic point behind the RORAC label
is that this construction injects risk through the denominator only; an exact
risk-adjusted-return measure would instead deflate the numerator by a risk charge.

The ex-ante / ex-post distinction is about which inputs feed the same ratio: ex-ante
uses E[profit] (forward-looking, for capital allocation), ex-post uses realized profit
(backward-looking, for evaluation). A single bad year's ex-post RAROC should not by
itself drive expand/contract decisions, since a one-time event can dominate realized
profit — strategic decisions belong to expected long-term (ex-ante) results.

**Source:** Hull (2023) Ch.28 §28.8 printed pp.611–613 (PDF pp.639–641).

## See Also
- [rm-economic-capital-vs-regulatory-capital](./rm-economic-capital-vs-regulatory-capital.md) — the economic-capital denominator RAROC divides by.
- [rm-value-added-active-return](./rm-value-added-active-return.md) — a related risk-adjusted-performance view on the return side.
- [rm-loss-distribution-anatomy](./rm-loss-distribution-anatomy.md) — the expected-loss term subtracted in the numerator.

## Escalate to Raw When
You need the worked Example 28.5 lending-unit RAROC numerics (the base case and the
variant that adds interest earned on economic capital) — those plug-and-chug figures
live in the raw text (Rule 1).

**Source:** Hull (2023) Ch.28 §28.8 printed pp.611–613 (PDF pp.639–641).
