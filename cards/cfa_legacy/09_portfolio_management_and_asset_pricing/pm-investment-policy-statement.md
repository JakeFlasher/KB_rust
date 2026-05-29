---
schema_version: "cacg.v0"
id: "pm-investment-policy-statement"
title: "Investment Policy Statement"
reading_id: "09_portfolio_management_and_asset_pricing"
summary: "Investment Policy Statement: framing the Investment Policy Statement (IPS) as the governing document for a portfolio — its purpose, structural sections, and the role it plays in disciplining downstream allocation and rebalancing decisions"
tags: ["portfolio-management", "ips", "objectives"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3275:4932"
    chunk_hash: "1b3a1d947a74b2cf4932dbe1671930ad5d68061ba1ecca9bb177f271d9f37644"
    page_range: [3275, 3276]
    quote: "The IPS is the starting point of the portfolio management process. Without a full understanding of the client’s situation and requirements, it is unlikely that successful results will be achieved."
    edge_type: "defines"
card_hash: "fb7fd868d2977e2d5604acd747a84618868f45106418d6ade566a8040f9f28f1"
---
# Investment Policy Statement

## Intuition

The Investment Policy Statement is the contract between the investor
and the manager that fixes the rules of engagement before any holding
is bought. It records who the investor is, what they want from the
portfolio, what risk they can absorb, and what constraints bind the
allocation choice. Once signed, the IPS is the reference document
that downstream decisions — strategic allocation, security selection,
rebalancing, performance review — must answer to. **Source:** CFA L1
Curriculum (2022) Vol.6/pp.5-25.

```
        +-----------------------------------+
        |   Investment Policy Statement     |
        |                                   |
        |   purpose         (why a policy)  |
        |   objectives      (return + risk) |
        |   constraints     (5 families)    |
        |   governance      (roles, review) |
        +-----------------------------------+
                |             ^
                v             |
        strategic        feedback &
        allocation       periodic review
                |             ^
                v             |
        execution &      performance
        rebalancing      measurement
```

The IPS protects against two failure modes. First, it disciplines the
investor against ad-hoc reaction to short-run market noise — a stated
risk objective and rebalancing rule make panic-selling visibly
out-of-policy. Second, it protects the manager against client drift —
a documented constraint set means a request to deviate is checkable
against the original mandate. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.5-25.

## Definition

An IPS contains four standard sections at L1 depth: a statement of
purpose, the investor's objectives, the binding constraints, and
governance / review provisions. Some sections appear under different
headings in practice (e.g. "duties and responsibilities" alongside
governance), but the four-pillar content map is canonical for the
CFA L1 framing. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.

The two objective categories are the return objective and the risk
objective. The return objective records what level of return the
portfolio is intended to deliver — stated as required, desired, or
both — given the funding need. The risk objective records the level
of variability the investor can absorb without compromising the goal,
decomposed into ability and willingness components covered in a
sibling card. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.

The five constraint families are liquidity, time horizon, taxes,
legal / regulatory, and unique circumstances (the LLTTU mnemonic).
Each family bounds the feasible portfolio in a different dimension;
the strategic asset allocation operates only within the intersection
of all five. The constraint families are detailed in a sibling card.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.

## Mathematical Reasoning

The IPS does not by itself encode an optimization, but it specifies
the constraint set inside which the strategic allocation operates.
Symbolically, the strategic asset allocation problem reads as a
constrained mean-variance optimization on the feasible set. **Source:**
CFA L1 Curriculum (2022) Vol.6/pp.5-25.

```
choose w in W_IPS  to  maximize  U(E[r_p(w)], var(r_p(w)))
                       subject to constraints from LLTTU
```

Here `W_IPS` is the feasible weight set defined by the IPS
constraints; `U` is the investor utility expressed as a function of
expected portfolio return and variance; the LLTTU constraints
restrict `W_IPS` from the unconstrained simplex. The IPS does not pin
down `w` directly — that work belongs to the strategic asset
allocation step — but it determines `W_IPS` and the form of `U`.
**Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.

The implication is procedural rather than computational: changing the
IPS changes the feasible set `W_IPS`, which can move the optimal `w*`
discontinuously even when no market parameter has changed. A
liquidity-driven shrinkage of `W_IPS`, a tighter legal restriction
on a sector, or a lengthening of the time horizon all alter `W_IPS`
and therefore the strategic allocation. The IPS is the upstream
authority for any allocation drift, and IPS revision must precede
material allocation revision. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.5-25.

The governance / review cadence sets a discrete schedule on which the
IPS itself is re-examined. The cadence is typically annual or upon
material life events for individuals, and quarterly or upon mandate
changes for institutions. Between reviews, the IPS is treated as
fixed; deviations are recorded as exceptions rather than silent
policy drift. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.

## See Also

- [`pm-portfolio-perspective.md`](pm-portfolio-perspective.md) — the portfolio-process loop that the IPS sits at the head of
- [`pm-risk-tolerance-and-objectives.md`](pm-risk-tolerance-and-objectives.md) — ability vs willingness decomposition of the risk objective
- [`pm-portfolio-constraints.md`](pm-portfolio-constraints.md) — the five constraint families (LLTTU) that compose `W_IPS`
- [`pm-allocation-process.md`](pm-allocation-process.md) — the strategic / tactical allocation step downstream of the IPS

## Escalate to Raw When

Open the CFA L1 Curriculum Vol.6 R51 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.6/pp.5-25.

- Detailed institutional IPS templates (defined-benefit pension fund,
  endowment / foundation, insurance general account) — these expand
  the L1 framing into mandate-specific sections that belong in
  future-13. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.
- Worked examples of return-objective formulation under inflation,
  spending, and tax constraints simultaneously — Vol.6 R51 walks
  through scenarios that the present card abstracts symbolically.
  **Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.
- Behavioral overlays on objective elicitation (loss-aversion-driven
  risk-objective drift, mental accounting) — these sit at the
  intersection of IPS construction and behavioral finance, deferred
  to future-10. **Source:** CFA L1 Curriculum (2022) Vol.6/pp.5-25.
