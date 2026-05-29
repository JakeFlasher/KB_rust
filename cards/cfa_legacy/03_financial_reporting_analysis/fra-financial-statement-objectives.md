---
schema_version: "cacg.v0"
id: "fra-financial-statement-objectives"
title: "Financial Statement Objectives"
reading_id: "03_financial_reporting_analysis"
summary: "Framing what financial statements assert about the firm and the multi-audience compromise the framework strikes (creditor, equity analyst, regulator, manager). The CFA L1 six-step FSA framework structures the analyst's read; Penman frames statements as inputs to forecasting future cash flows and the risk to those cash flows."
tags: ["financial-reporting", "financial-statement"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1054:1503"
    chunk_hash: "e79396d3e45017d0d0c9b843ab7866d13966a5c47704fef58dcdbdb33a32c5e9"
    page_range: [1054, 1054]
    quote: "In general, analysts seek to examine the past and current performance and financial position of a company in order to form expectations about its future performance and financial position."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1052:1501"
    chunk_hash: "3d399f684d8ce2638879c11d3ded5466dc63dcc1d9a49a2bad08e490bf450658"
    page_range: [1052, 1053]
    quote: "This reading is organized as follows: Section 2 discusses the scope of financial statement analysis."
    edge_type: "supports"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p059:0093"
    chunk_hash: "615f90c007748ebf05db1b9d93415b35e83af2155030f6942154a65d88015a1a"
    page_range: [59, 60]
    quote: "This chapter gives you a basic understanding of the financial statements with a view to using them as an analysis tool."
    edge_type: "supports"
card_hash: "43fa971dee01f066cbf5fb2008e572f17c171197d7fd15627cf7608d6d46bddd"
---
# Financial Statement Objectives

## Intuition

Financial statements are the firm's structured public account of its
own economic activity over a period. They claim that within an
agreed-upon framework (IFRS or US GAAP), the firm's recorded numbers
faithfully represent its financial position, performance, and cash
generation, well enough that a thoughtful outside analyst can form
an informed judgment about valuation and risk. The analyst's first
task is therefore to understand what the statements assert and what
they deliberately leave out. **Source:** CFA L1 Curriculum (2022)
Vol.2/pp.475-514.

The same numbers serve very different audiences with very different
questions. A creditor asks whether the firm can service debt; an
equity analyst asks whether the firm is creating shareholder value;
a regulator asks whether reporting is compliant; a manager asks
whether the operating plan is working. The financial-statement
framework is a compromise across these uses — it is general-purpose,
not investor-specific, and the analyst must therefore extract
investor-relevant signal from a report whose structure was not
built solely for them. **Source:** CFA L1 Curriculum (2022)
Vol.2/pp.475-514.

Penman frames the purpose of financial statements directly in
valuation terms: the statements are inputs to the analyst's claim on
the firm's value, and the analysis discipline asks "what do these
numbers tell about future cash flows and the risk to those cash
flows?" The statements describe the past, but the analyst's interest
is the future — the statements are useful insofar as they discipline
forecasts of payoffs that valuation requires. **Source:** Penman
(2013) Ch.2 pp.32-71.

```
+----------------------------------------+
| Conceptual Framework Hierarchy         |
+----------------------------------------+
|  Fundamental qualitative chars:        |
|     - Relevance                        |
|     - Faithful representation          |
+----------------------------------------+
|  Enhancing qualitative chars:          |
|     - Comparability                    |
|     - Verifiability                    |
|     - Timeliness                       |
|     - Understandability                |
+----------------------------------------+
|  Constraints:                          |
|     - Cost vs benefit                  |
|     - Materiality                      |
+----------------------------------------+
```

The diagram orders the qualitative characteristics by the framework's
own ranking: fundamental characteristics (relevance and faithful
representation) are necessary for usefulness; enhancing
characteristics (comparability, verifiability, timeliness,
understandability) make useful information more useful but do not
substitute for the fundamentals. The framework is principles-level
and is not itself enforceable rule; specific standards (IFRS, US
GAAP) operationalize the qualitative characteristics. **Source:**
CFA L1 Curriculum (2022) Vol.2/pp.475-514.

## Definition

The firm's financial position and performance are reported through
four primary statements that articulate (link) to each other through
shared accounts. Each statement answers a distinct question while
remaining algebraically tied to the others. **Source:** CFA L1
Curriculum (2022) Vol.2/pp.475-514.

- The balance sheet (statement of financial position) is a snapshot
  at a point in time of the firm's resources (Assets), claims against
  those resources (Liabilities), and the residual claim (Equity),
  satisfying the accounting identity. The balance sheet is a stock
  measure. **Source:** CFA L1 Curriculum (2022) Vol.2/pp.475-514.
- The income statement (statement of comprehensive income) is a flow
  measure of revenue earned and expenses incurred over a period,
  yielding net income. The income statement explains a part of why
  the equity stock changed between two balance-sheet dates.
  **Source:** CFA L1 Curriculum (2022) Vol.2/pp.475-514.
- The statement of cash flows is a flow measure of cash generated
  or used over a period, partitioned into operating, investing, and
  financing activities. It explains why the cash line on the
  balance sheet changed between two reporting dates. **Source:** CFA
  L1 Curriculum (2022) Vol.2/pp.475-514.
- The statement of changes in equity is a flow measure of how each
  equity account (retained earnings, share capital, accumulated
  other comprehensive income) changed over the period. It explains
  every part of the equity-stock change that the income statement
  alone does not. **Source:** CFA L1 Curriculum (2022)
  Vol.2/pp.475-514.

Notes to the statements and the management discussion and analysis
(MD&A) accompany the four statements and are essential to a
faithful read. Critical accounting policies, segment disclosures,
related-party transactions, and contingent liabilities live in the
notes and routinely contain information that materially changes
interpretation of the headline numbers. **Source:** CFA L1
Curriculum (2022) Vol.2/pp.475-514.

## Mathematical Reasoning

The accounting identity links the three balance-sheet aggregates by
construction: at every reporting date, `Assets = Liabilities +
Equity`. This is not an empirical regularity but a definitional
constraint of the double-entry system — every recorded transaction
preserves the identity. **Source:** CFA L1 Curriculum (2022)
Vol.2/pp.475-514.

Articulation across the four statements is captured by the equity
roll-forward. Letting `E_t` denote ending equity, `E_{t-1}` opening
equity, `NI_t` net income, `Div_t` dividends declared, and `OCI_t`
other comprehensive income / share-issuance / repurchase effects,
articulation requires `E_t = E_{t-1} + NI_t - Div_t + OCI_t` (with
sign conventions for issuance positive, repurchase negative). The
income statement contributes the `NI_t` term; the statement of
changes in equity contributes the `Div_t` and `OCI_t` terms.
**Source:** CFA L1 Curriculum (2022) Vol.2/pp.475-514.

Cash-line articulation is symmetric: ending cash on the balance sheet
equals opening cash plus the net change in cash from the statement
of cash flows, with the cash-flow statement decomposing the change
into operating, investing, and financing activities. The two
articulation identities together mean that no balance-sheet account
moves without a flow statement explaining the move; the four
statements are not independent reports but a single articulated
description of the firm's period activity. **Source:** Penman (2013)
Ch.2 pp.32-71.

The CFA L1 curriculum sets out a six-step financial-statement-
analysis framework applied to any analytical exercise. The steps
discipline what the analyst should ask before consulting the
statements themselves, with each step constraining the next.
**Source:** CFA L1 Curriculum (2022) Vol.2/pp.475-514.

1. Articulate the purpose and context — what question is the analysis
   trying to answer (equity valuation, credit assessment, M&A
   screen, fraud detection); the question determines which line
   items receive primary attention. **Source:** CFA L1 Curriculum
   (2022) Vol.2/pp.475-514.
2. Collect input data — financial statements (current period plus
   comparable historical periods), notes, MD&A, regulatory
   filings, industry data, peer-firm reports. **Source:** CFA L1
   Curriculum (2022) Vol.2/pp.475-514.
3. Process the data — compute ratios, common-size statements,
   growth rates, segment-level decompositions; reconcile data sets
   where inconsistencies appear. **Source:** CFA L1 Curriculum
   (2022) Vol.2/pp.475-514.
4. Analyze and interpret the processed data — what patterns,
   trends, anomalies stand out, and how do they compare to peers,
   to history, to a hypothesized economic narrative. **Source:**
   CFA L1 Curriculum (2022) Vol.2/pp.475-514.
5. Develop and communicate conclusions and recommendations with
   evidence, with explicit assumptions stated, in a form the
   intended decision-maker can act on. **Source:** CFA L1
   Curriculum (2022) Vol.2/pp.475-514.
6. Follow up — update the analysis as new periods report and as
   conditions change. **Source:** CFA L1 Curriculum (2022)
   Vol.2/pp.475-514.

## See Also

- [`eq-intrinsic-value`](../05_equity/eq-intrinsic-value.md) — financial statements feed the cash-flow and discount-rate inputs to intrinsic-valuation work
- [`eq-discount-rate-and-required-return-foundations`](../05_equity/eq-discount-rate-and-required-return-foundations.md) — how the analyst translates accounting risk indicators into a required return

## Escalate to Raw When

Open the CFA L1 curriculum Vol.2 R15 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.2/pp.475-514.

- a specific LOS-level qualitative characteristic interpretation is
  needed (e.g., distinguishing relevance from faithful representation
  for a borderline disclosure choice) — the curriculum's own
  paragraph-level discussion is the canonical authority. **Source:**
  CFA L1 Curriculum (2022) Vol.2/pp.475-514.
- the analyst is constructing a non-public comparative — peer firms
  use different reporting frameworks (IFRS vs US GAAP) and the
  analyst needs the curriculum's framework-comparison guidance.
  **Source:** CFA L1 Curriculum (2022) Vol.2/pp.475-514.
- the analytical question is firm-comparability across periods or
  across reporting regimes — Penman Ch.2's discussion of
  comparability constraints supplements the curriculum's framework
  account. **Source:** Penman (2013) Ch.2 pp.32-71.
