---
schema_version: "cacg.v0"
id: "fra-ifrs-vs-us-gaap-framework"
title: "IFRS vs US GAAP Framework"
reading_id: "03_financial_reporting_analysis"
summary: "Framing the two main reporting frameworks (IASB-issued IFRS; FASB-issued US GAAP) and the principles-based vs rules-based design differential. Specific measurement-rule divergences (inventory LIFO, development costs, PP&E revaluation, impairment-loss reversal) require analyst-side adjustments for peer comparability."
tags: ["financial-reporting", "ifrs-us"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1108:1585"
    chunk_hash: "a0d7c48d7912fd7856b93dd0144fa48dab97552df213064fa5cf5c6d8e6b6d65"
    page_range: [1108, 1109]
    quote: "Arguably, the most critical are the differences that exist between IFRS and US GAAP."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1109:1586"
    chunk_hash: "b9faff538e72c4619a273335ca87c82112809e1b40c966b476bcaa4e445434ff"
    page_range: [1109, 1110]
    quote: "Differences between IFRS and US GAAP remain and affect the framework as well as numerous financial reporting standards."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1421:2066"
    chunk_hash: "c960e167f4ffc555ec5fae75fa80c6c5d8fe742e71565e2d98eb393ac9d46b98"
    page_range: [1421, 1422]
    quote: "Companies must disclose the factors used to identify reportable segments and the types of products and services sold by each reportable segment."
    edge_type: "supports"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p059:0093"
    chunk_hash: "615f90c007748ebf05db1b9d93415b35e83af2155030f6942154a65d88015a1a"
    page_range: [59, 60]
    quote: "This chapter gives you a basic understanding of the financial statements with a view to using them as an analysis tool."
    edge_type: "supports"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p319:0525"
    chunk_hash: "1441f55a02c880aa1b989d5b0f3c106edba02cbaec72cdbd80004cede903e4d9"
    page_range: [319, 319]
    quote: "Thus the analysis begins with a reformulation of the statements, following the templates of Chapter 8, to distinguish operating activities from financing activities."
    edge_type: "supports"
card_hash: "334884f19172b424c768f5870a2f4ddadc1221f0e3af284f4dd56b82ce12cf87"
---
# IFRS vs US GAAP Framework

## Intuition

The world's listed firms report under one of two main standards
systems. International Financial Reporting Standards (IFRS), issued
by the International Accounting Standards Board (IASB), is required
or permitted in over 100 jurisdictions and is principles-based at
its core. US Generally Accepted Accounting Principles (US GAAP),
issued by the Financial Accounting Standards Board (FASB), is
required for SEC-registered firms in the United States and is
historically rules-based with extensive industry-specific guidance.
Both systems share the same valuation-relevant ambition — to produce
faithful, comparable financial information about the firm — but
they reach the goal through different mixes of principles and
detailed rules. **Source:** CFA L1 Curriculum (2022)
Vol.2/pp.515-540.

The practical consequence for the analyst is double-tracked.
Cross-listed firms that report under both frameworks publish
reconciliation footnotes; same-jurisdiction peer comparisons are
within-framework and therefore directly comparable; cross-framework
comparisons require the analyst to identify and adjust for the
specific measurement differences (inventory cost methods,
development-cost capitalization, asset revaluation, lease
accounting, financial-instrument classification). The analyst's
working knowledge needs to be where the systems agree and
specifically where they diverge. **Source:** CFA L1 Curriculum (2022)
Vol.2/pp.515-540.

```
+--------------------------------------------+
| Standards System Comparison                |
+--------------------------------------------+
|              | IFRS         | US GAAP      |
+--------------+--------------+--------------+
| Issuer       | IASB         | FASB         |
| Approach     | Principles   | Rules + Prin.|
| Inventory    | FIFO/WAC     | FIFO/WAC/LIFO|
| LIFO allowed | NO           | YES          |
| Dev. costs   | Capitalize   | Expense      |
|              | (criteria)   |              |
| PP&E reval.  | Allowed      | NOT allowed  |
| Goodwill     | No amort.,   | No amort.,   |
|              | impair test  | impair test  |
| Intangibles  | Reval allowed| Reval not    |
|              | (criteria)   | allowed      |
| Loss reverse | Allowed      | NOT allowed  |
+--------------+--------------+--------------+
```

The schematic above lists the most-cited measurement differences.
The list is not exhaustive — lease accounting, financial-instrument
classification, employee-benefit accounting, and certain
revenue-recognition edge cases also differ — but the listed items
are the canonical L1 LOS-level set the analyst should know.
**Source:** CFA L1 Curriculum (2022) Vol.2/pp.515-540.

## Definition

A financial-reporting framework is the codified set of recognition,
measurement, presentation, and disclosure rules that govern how
firms prepare and present financial statements. The two main
frameworks are IFRS and US GAAP. **Source:** CFA L1 Curriculum (2022)
Vol.2/pp.515-540.

IFRS is published by the IASB, which operates under the IFRS
Foundation. IFRS standards consist of the IFRS conceptual framework
(qualitative characteristics, elements of financial statements,
recognition / measurement bases) plus individual IFRS and IAS
standards on specific topics. The IFRS approach is generally
principles-based: standards state the underlying economic principle
and require management judgment to apply it to specific
transactions. **Source:** CFA L1 Curriculum (2022) Vol.2/pp.515-540.

US GAAP is published by the FASB, an independent private-sector
body whose standards have official authority for SEC-registered
firms via SEC delegation under the Sarbanes-Oxley Act of 2002. US
GAAP standards historically include extensive industry-specific
guidance and detailed bright-line rules that reduce the need for
judgment but increase the volume of rules to track. The FASB has
been engaged in a long-running convergence project with the IASB
that has narrowed but not eliminated differences. **Source:** CFA
L1 Curriculum (2022) Vol.2/pp.515-540.

The principles-vs-rules distinction has three practical
consequences. First, IFRS reporting often requires more management
judgment in application; rules-based reporting often requires
detailed rule-following. Second, IFRS allows certain accounting
choices (revaluation of PP&E, intangible-asset revaluation, certain
loss reversals) that US GAAP does not permit. Third, US GAAP
permits LIFO inventory cost flow that IFRS prohibits. **Source:**
CFA L1 Curriculum (2022) Vol.2/pp.515-540.

## Mathematical Reasoning

The two frameworks share the accounting identity `Assets =
Liabilities + Equity` and the articulation identities for the four
primary statements. They differ in measurement bases for specific
line items, which can lead to different reported asset values,
different reported earnings, and different reported equity for the
same underlying firm activity. **Source:** CFA L1 Curriculum (2022)
Vol.2/pp.515-540.

The most cited measurement-rule differences and their valuation
consequences are listed below. Each pairing identifies one IFRS
choice, the US GAAP counterpart, and the direction in which reported
balance-sheet or income-statement values differ. **Source:** CFA L1
Curriculum (2022) Vol.2/pp.515-540.

- Inventory cost flow: IFRS permits FIFO and weighted-average cost
  but prohibits LIFO. US GAAP permits FIFO, weighted-average, and
  LIFO. Under rising prices, LIFO produces higher COGS and lower
  reported inventory than FIFO; the choice affects gross margin,
  inventory turnover ratios, and tax (LIFO is selected partly for
  tax-deferral effect under US tax law). The LIFO-vs-FIFO contrast
  is a routine peer-comparability adjustment for US-GAAP-reporting
  firms. **Source:** CFA L1 Curriculum (2022) Vol.2/pp.515-540.
- Development costs: IFRS permits capitalization of development
  costs once specified criteria are met (technical feasibility,
  intent and ability to complete, future economic benefits
  identifiable). US GAAP generally requires expensing of all
  research and development costs as incurred (with narrow exceptions
  for software development under ASC 985-20). IFRS-reporting firms
  with capitalized development costs show higher intangible-asset
  balances and lower current-period expense than US-GAAP-reporting
  peers with similar R&D activity. **Source:** CFA L1 Curriculum
  (2022) Vol.2/pp.515-540.
- Property, plant, and equipment: IFRS permits revaluation to fair
  value (with the gain recorded in OCI as a revaluation surplus, or
  in net income if reversing a prior impairment loss). US GAAP
  prohibits upward revaluation; PP&E is at historical cost less
  accumulated depreciation. IFRS-reporting firms that revalue PP&E
  show higher asset balances and higher reported equity than peers
  on the cost model. **Source:** CFA L1 Curriculum (2022)
  Vol.2/pp.515-540.
- Impairment loss reversals: IFRS permits reversal of previously
  recognized impairment losses (other than for goodwill) when the
  conditions causing the loss have changed. US GAAP prohibits
  reversal of any impairment loss. IFRS-reporting firms whose
  asset values recover from a prior impairment show recovery in
  net income; US-GAAP peers show no such recovery. **Source:** CFA
  L1 Curriculum (2022) Vol.2/pp.515-540.
- Intangible-asset revaluation: IFRS permits revaluation of
  intangible assets (other than goodwill) when an active market
  exists for the asset. US GAAP prohibits intangible-asset
  revaluation. IFRS-reporting firms with revalued intangibles show
  higher intangible balances. **Source:** CFA L1 Curriculum (2022)
  Vol.2/pp.515-540.

For both frameworks, goodwill is not amortized; instead, goodwill
is tested for impairment annually (or more frequently if indicators
exist), and the impairment is recognized immediately in net income
and is non-reversible under both frameworks. The convergence
project has aligned several other treatments (revenue recognition
under IFRS 15 / ASC 606; lease accounting under IFRS 16 / ASC 842).
**Source:** CFA L1 Curriculum (2022) Vol.2/pp.515-540.

Segment reporting under IFRS 8 (Operating Segments) and FASB ASC
Topic 280 (Segment Reporting) follows the management-approach
principle: an operating segment is a component of a company that
engages in revenue-generating activities, whose results are
regularly reviewed by the company's senior management (including
the chief operating decision maker), and for which discrete
financial information is available. A segment is reportable if it
constitutes 10 percent or more of the combined operating segments'
revenue, assets, or profit; if the combined revenue from external
customers across reportable segments falls below 75 percent of
total company revenue, additional segments are added until the
75 percent floor is reached. For each reportable segment the
firm discloses a measure of segment profit or loss, segment
assets, segment revenue (distinguishing external customers from
inter-segment revenue), and reconciliation of segment totals to
the consolidated statements. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.234-252.

The analyst uses these disclosures to assess business-mix
concentration, segment-level profitability divergence, and
capital-allocation priorities. Disaggregated segment detail from
footnote disclosures often reveals more about a firm's operations
than the consolidated statements convey at first reading.
**Source:** Penman (2013) Ch.10 pp.292-341.

The cross-framework comparability problem has a structural
solution: cross-listed firms publish a reconciliation that adjusts
their reported numbers from one framework to the other. The
reconciliation makes the underlying differences explicit, and the
analyst can use it to construct comparable peer benchmarks. For
firms reporting under only one framework, the analyst must
estimate the adjustments using disclosed footnote information.
**Source:** Penman (2013) Ch.2 pp.32-71.

## See Also

- [`fra-financial-statement-objectives`](./fra-financial-statement-objectives.md) — the qualitative-characteristics framework that both IFRS and US GAAP share
- [`fra-balance-sheet-foundations`](./fra-balance-sheet-foundations.md) — measurement-basis heterogeneity within total assets is governed by the framework choice
- [`fra-income-statement-foundations`](./fra-income-statement-foundations.md) — comprehensive-income vs net-income classification interacts with framework-specific OCI treatments

## Escalate to Raw When

Open the CFA L1 curriculum Vol.2 R16 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.2/pp.515-540.

- the analyst is comparing peer firms across the IFRS / US GAAP
  boundary and needs the specific measurement-rule differences for
  inventory, PP&E, intangibles, or impairment — the curriculum's
  per-area treatment is the authoritative reference. **Source:** CFA
  L1 Curriculum (2022) Vol.2/pp.515-540.
- the firm has a recently published cross-framework reconciliation
  footnote and the analyst needs to interpret the magnitude and
  direction of each line. **Source:** CFA L1 Curriculum (2022)
  Vol.2/pp.515-540.
- the firm is in an industry with significant standard-specific
  treatments (financial services, insurance, real estate, oil and
  gas) where US GAAP has detailed industry guidance that IFRS does
  not. **Source:** CFA L1 Curriculum (2022) Vol.2/pp.515-540.
