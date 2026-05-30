---
schema_version: "cacg.v0"
id: "pa-gips-2020-composites-and-mechanics"
title: "GIPS 2020: Composites and Calculation Mechanics"
reading_id: "15_performance_and_attribution"
summary: "GIPS 2020 are voluntary fair-representation standards split into Firms, Asset-Owners, and Verifiers chapters; firms claim compliance on a firm-wide basis over a five-year minimum window, grouping similar mandates into composites reported on a TWR or MWR basis."
tags: ["gips", "composites", "performance-presentation"]
citations:
  - source_id: "pa_bacon_2023_attribution"
    chunk_id: "pa_bacon_2023_attribution:p427:0503"
    chunk_hash: "9ef6dacafa15dff0b7d6272ca64dbe63b9d7cc4f0aa4766dc146354045e12a79"
    page_range: [428, 428]
    quote: "Clearly, it’s very inefficient for individual portfolio managers to be wasting their time ensuring returns are calculated correctly."
    edge_type: "defines"
  - source_id: "pa_christopherson_carino_ferson_2009"
    chunk_id: "pa_christopherson_carino_ferson_2009:p463:0477"
    chunk_hash: "bf9df0311da2fd3d0f57be4472cce9715fabcbcaa868f72f38b6b2cf72b87510"
    page_range: [463, 464]
    quote: "These two principles provide a foundation for virtually all aspects"
    edge_type: "supports"
card_hash: "d1db2310311cf61cc3475fc2eb64f3ef75e0a493b69e15e0ef3f0d89acb06819"
---
# GIPS 2020: Composites and Calculation Mechanics

## Intuition

The investment-management business is fiercely competitive, and a firm can flatter its track record many ways short of outright fraud: cherry-pick the winning accounts, splice simulated history onto realised returns, or quietly drop closed losing portfolios. GIPS exists to remove that discretion. It is a *voluntary* set of ethical standards — not law — that a firm adopts so a prospect can compare two managers on a level playing field. The price of admission is mechanical discipline: you must measure every comparable mandate the same way, group them, and present the group, not just your trophies.

**Source:** Bacon (2023) §THE GIPS STANDARDS pp.428-430

The enforcement bite comes not from statute but from market and legal pressure: a manager who ignores a widely accepted standard, consistent with most national anti-fraud law, sits in a weak position. The two organising ideas are *full disclosure* and *fair representation* — every other rule is a way of operationalising those two.

**Source:** Christopherson, Cariño & Ferson (2009) Ch.33 pp.462-465

## Definition

GIPS (Global Investment Performance Standards) are ethical standards for investment-performance presentation whose purpose is to ensure fair representation and full disclosure of a firm's performance track record. The 2020 edition is organised into three chapters: (1) GIPS Standards for **Firms**, (2) GIPS Standards for **Asset Owners**, and (3) GIPS Standards for **Verifiers**. Asset owners report to an oversight body rather than compete for business; verifiers perform the independent third-party review.

**Source:** Bacon (2023) §THE GIPS STANDARDS pp.428-430

Key mechanics for firms:

- **Firm-wide basis.** Compliance must be met firm-wide and cannot be claimed for only certain asset classes, strategies, products, or pooled funds. The *definition of the firm* — a distinct business entity held out to clients — fixes the boundary over which total firm assets are determined.
- **Five-year minimum window.** To initially claim compliance a firm must attain it for a minimum of five years, or for the period since inception if the firm is younger than five years.
- **Composites.** Compliant firms combine *similar* accounts into composites so aggregate skill in a particular asset class or market segment is communicated without selectivity; firms must provide lists of composites on request.
- **Return basis.** Performance is reported on a time-weighted-return (TWR) basis or, where the firm controls cash flows (e.g. closed-end / committed-capital vehicles), a money-weighted-return (MWR) basis — the firms chapter carries separate composite TWR, composite MWR, pooled-fund TWR, and pooled-fund MWR report sections.

**Sources:** Bacon (2023) §FUNDAMENTALS OF COMPLIANCE pp.429-430 (firm-wide basis, five-year window, eight report sections) and p.444 (TWR default; MWR only if the firm controls external cash flows); Christopherson, Cariño & Ferson (2009) Ch.33 p.464 (combining similar accounts into composites, lists on request)

A standard distinction the standards draw: **requirements** (the `MUST` provisions, mandatory to claim compliance) versus **recommendations** (optional good practice). Claiming compliance without independent verification is flagged as highly risky, though verification is itself not strictly required.

**Source:** Bacon (2023) §THE GIPS STANDARDS pp.428-429

## Mathematical Reasoning

GIPS prescribes *which* aggregation and return identities a firm must use, not a new estimator. The composite is an asset-weighted aggregate of its member portfolios. Symbolically, for a composite of member portfolios $i$ over a period, the composite return is the asset-weighted combination of member returns

$$ R_{\text{composite}} = \sum_i w_i\, R_i, \qquad w_i = \frac{V_i}{\sum_j V_j}, $$

where $w_i$ is the beginning-of-period (or cash-flow-adjusted) asset weight of portfolio $i$. The asset-weighting — rather than a simple average — is what makes the composite *representative* of the strategy as actually run, eliminating the selectivity bias that motivates the standard; larger portfolios have a more significant influence on the composite return.

**Source:** Bacon (2023) §INPUT DATA AND CALCULATION METHODOLOGY — Composite returns pp.457 (provision 2.A.36)

The two return identities the firm may use are the standard ones owned by sibling cards: TWR neutralises external cash flows by chain-linking sub-period returns, whereas MWR (the IRR) solves for the rate that equates discounted flows. GIPS does not redefine either; the default is TWR, and a firm may present MWR only if it has control over the portfolios' external cash flows. The sources assert these provisions without re-deriving the estimators, so this card asserts likewise and defers the algebra to the dedicated return cards.

**Source:** Bacon (2023) §FUNDAMENTALS OF COMPLIANCE — Use of time-weighted or money-weighted returns p.444 (provisions 1.A.35–1.A.36)

```
GIPS 2020 - three chapters, and the Firms calculation spine
============================================================

                +---------------------------------------+
                |            GIPS 2020 edition           |
                +---------------------------------------+
                   |             |              |
              (1) FIRMS    (2) ASSET OWNERS  (3) VERIFIERS
            compete for    report to        independent
            business       oversight body   third-party review
                   |
                   v   firm-wide basis (1.A.1) + >=5yr window (1.A.3)
        +----------------------------------------------+
        |  group SIMILAR mandates  -->  COMPOSITE      |
        |  (asset-weighted, no selectivity)            |
        +----------------------------------------------+
                   |                         |
          controlled cash flows?    uncontrolled cash flows?
                   |                         |
                  MWR                       TWR
            (composite/pooled         (composite/pooled
             MWR report)               TWR report)
```

**Source:** Bacon (2023) §FUNDAMENTALS OF COMPLIANCE pp.429-430 (firm-wide basis 1.A.1, five-year window 1.A.3, eight report sections), p.444 (TWR default vs. MWR-only-if-firm-controls-cash-flows, 1.A.35–1.A.36)

## Boundary Notes

This card covers GIPS *calculation and composite mechanics* only — firm definition, the five-year window, composite construction, and the TWR/MWR reporting split. The ethics-side **claim of compliance** (how a member or firm may state adherence under the CFA Code & Standards, Standard III(D) Performance Presentation) lives in topic-17 (ethics). Worked composite or dispersion numbers are out of scope per Critical Rule 1.

**Source:** Bacon (2023) §THE GIPS STANDARDS pp.428-429

## See Also

- [`pa-twr-vs-mwr-when-each-applies.md`](pa-twr-vs-mwr-when-each-applies.md) — the controlled-vs-uncontrolled-cash-flow test that GIPS uses to decide composite TWR vs. MWR reporting.
- [`pa-true-twr-and-chain-linking.md`](pa-true-twr-and-chain-linking.md) — the time-weighted estimator GIPS requires for the composite TWR report.
- [`pa-irr-money-weighted-return.md`](pa-irr-money-weighted-return.md) — the money-weighted (IRR) estimator GIPS permits where the firm controls cash flows.
- [`pa-valid-benchmark-properties.md`](pa-valid-benchmark-properties.md) — composite presentation depends on a disclosed, valid benchmark choice for the strategy each composite represents.

The firm-wide claim-of-compliance and Standard III(D) Performance Presentation obligations are an ethics matter handled in the 17 cross-cutting (ethics) GIPS material, not here.

## Escalate to Raw When

- You need the *exact* required provisions (e.g. the numbered `1.A.x` MUST clauses on the firm definition, recordkeeping, portability, error correction) or the full eight-section structure of the Firms chapter — read Bacon (2023) pp.428-430 and the CFA Institute GIPS standards themselves.
- You must actually *compute* a composite return, asset-weighted average, or composite dispersion statistic on real account data — the worked aggregation is deferred per Critical Rule 1.
- You need the verifier-side procedures (purpose, scope, engagement terms, representation letter, verification report) — see the ten-item verifier list in Bacon (2023) p.428.
- You need the ethics-side rules for *stating* a claim of GIPS compliance under the Code & Standards — escalate to the topic-17 ethics material.
