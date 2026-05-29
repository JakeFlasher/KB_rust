---
schema_version: "cacg.v0"
id: "cc-standard-v-investment-analysis-diligence"
title: "Standard V(A) Diligence and Reasonable Basis + V(B) Communication + V(C) Record Retention"
reading_id: "17_cross_cutting"
summary: "Standard V.A REQUIRES diligence/independence/thoroughness with a reasonable and adequate basis (calibrated to investment complexity) for any recommendation; V.B requires disclosure of investment process and clear opinion/fact distinction; V.C requires record retention sufficient to support ex-post audit of the V.A reasonable-basis analysis."
tags: ["cfa-ethics", "standard-v"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p3688:5550"
    chunk_hash: "edfcb8c731db341214862bf9421b13b1d66fd002d97300781d635d83f91b2c80"
    page_range: [3688, 3689]
    quote: "A reasonable basis is formed through a balance of these resources appropriate for the security or decision being analyzed."
    edge_type: "defines"
card_hash: "3e7575b4b0c329f2c93c2ed7a82aa02776c73866ff539eaefafd6a110312f81b"
---
# Standard V(A) Diligence and Reasonable Basis + V(B) Communication + V(C) Record Retention

## Intuition

Standard V.A REQUIRES Members and Candidates to: (1) exercise
diligence, independence, and thoroughness in analyzing investments,
making investment recommendations, and taking investment actions;
(2) have a reasonable and adequate basis, supported by appropriate
research and investigation, for any analysis, recommendation, or
action. The reasonable-basis test is the diligence anchor: a
recommendation absent supporting analysis is a V.A violation
regardless of whether the recommendation happens to be correct.
**Source:** CFA Institute (2022) L1 Vol.6/pp.419-428.

Standard V.B REQUIRES Members and Candidates to: (1) disclose to
clients and prospective clients the basic format and general
principles of the investment processes used; (2) promptly disclose
any changes that might materially affect those processes; (3)
distinguish between fact and opinion when presenting investment
analysis and recommendations. The opinion-vs-fact distinction is
the most exam-tested V.B predicate: "we believe earnings will grow
10%" is opinion; "earnings grew 10% last year" is fact; presenting
opinion as fact violates V.B (and may violate I.C). **Source:**
CFA Institute (2022) L1 Vol.6/pp.429-440.

Standard V.C REQUIRES Members and Candidates to develop and
maintain appropriate records to support their investment analyses,
recommendations, actions, and other investment-related
communications with clients and prospective clients. Records must
be sufficient to permit ex-post audit of the reasonable-basis
analysis under V.A. **Source:** CFA Institute (2022) L1
Vol.6/pp.440-442.

```
<!-- primitive: ethics-standards-hierarchy source: _diagram_primitives.md -->
+--------------------------------------------------------------+
|  Code of Ethics (six principles; preamble obligation)        |
|  +--------------------------------------------------------+  |
|  |  Standards I-VII (binding conduct requirements)        |  |
|  |  +--------------------------------------------------+  |  |
|  |  |  Guidance (CFA Institute interpretive commentary)|  |  |
|  |  |  +--------------------------------------------+  |  |  |
|  |  |  | Recommended Procedures (diligence evidence)|  |  |  |
|  |  |  +--------------------------------------------+  |  |  |
|  |  +--------------------------------------------------+  |  |
|  +--------------------------------------------------------+  |
+--------------------------------------------------------------+
   Binding ^                                          Best-     v
   force   |                                          practice
           |    REQUIRES / PROHIBITS / PERMITS          marker  |
           |               ^                                    |
           |               | (binding)                          |
           |               | RECOMMENDS marker                  |
                          (best-practice)
```

## Definition

Standard V.A REQUIRES Members and Candidates to: (1) exercise
diligence, independence, and thoroughness in analyzing investments,
making investment recommendations, and taking investment actions;
(2) have a reasonable and adequate basis, supported by appropriate
research and investigation, for any analysis, recommendation, or
action. The reasonable-basis test scales with the complexity of
the investment: a publicly traded large-cap equity may require less
investigation than a private illiquid investment; the Standard's
"reasonable" is calibrated to the specific recommendation, not a
fixed threshold. **Source:** CFA Institute (2022) L1
Vol.6/pp.419-428.

Standard V.A's third-party-research clause REQUIRES Members who
rely on third-party research (sell-side analyst reports, vendor
data, external models) to make a reasonable effort to verify the
research's quality before relying on it. The Standard's Guidance
identifies due-diligence factors: the research provider's
reputation, the research's methodology disclosure, the consistency
of conclusions with the underlying data, and any conflict
disclosures by the research provider. A Member who relies on
flawed third-party research without diligence violates V.A even
absent direct fabrication. **Source:** CFA Institute (2022) L1
Vol.6/pp.422-428.

Standard V.B REQUIRES disclosure of the basic investment-process
format, prompt disclosure of material process changes, and
distinction between fact and opinion. The disclosure obligation is
forward-looking: clients should understand HOW the manager will
make investment decisions before the decisions are made, not just
ex-post in performance reports. The Recommended Procedure
(RECOMMENDS) includes a written investment-process description,
periodic updates, and explicit signals (phrases like "we
believe", "in our opinion", "data show") to distinguish
opinion from fact. **Source:** CFA Institute (2022) L1
Vol.6/pp.429-440.

Standard V.C REQUIRES record retention sufficient to support the
V.A reasonable-basis analysis and the V.B communications. The
Recommended Procedure (RECOMMENDS) a 7-year retention default,
though specific regulatory regimes (SEC 17a-3, MiFID II) may
require longer retention. The records include research notes,
model inputs, communication records, and trade-execution records.
**Source:** CFA Institute (2022) L1 Vol.6/pp.440-442.

## Mathematical Reasoning

The reasonable-basis test under V.A (source REQUIRES) operationalizes
the diligence principle by requiring the recommendation's supporting
analysis to be both adequate (covers the relevant factors) and
reasonable (the inferences are defensible from the data). A
recommendation supported by inadequate analysis violates V.A even
when the recommendation is correct; conversely, a recommendation
supported by adequate analysis does not violate V.A even when the
recommendation later proves wrong. The Standard governs PROCESS,
not OUTCOME. The Repo-touchpoints link to
`eq-equity-cost-of-capital-estimation.md` is the cross-vertical
anchor: the equity-valuation diligence in subcorpus 05 operationalizes
V.A for equity recommendations. **Source:** CFA Institute (2022) L1
Vol.6/pp.419-428.

The opinion-vs-fact distinction under V.B (source REQUIRES) maps
to two distinct violation predicates: (1) presenting opinion as
fact (e.g., "the stock will outperform" — implicit prediction
presented as established outcome); (2) presenting fact as opinion
(e.g., "in our view, earnings grew 10%" — measurable historical
result presented as opinion). The Recommended Procedures
RECOMMENDS using explicit hedge phrases for opinion ("we believe",
"in our opinion", "our model suggests") and unmarked declarative
phrases for fact ("earnings grew", "the index returned"). The
hedge convention is the diligence-evidence that V.B inspections
look for. **Source:** CFA Institute (2022) L1 Vol.6/pp.430-440.

The record-retention discipline under V.C (source REQUIRES) ties
back to V.A and V.B: the records must be sufficient to support
ex-post audit of the reasonable-basis analysis and the disclosed
investment process. The Recommended Procedure RECOMMENDS retention
of research notes, model inputs, communication records, and
trade-execution records — collectively the diligence trail that a
PCP audit would follow. Inadequate records do NOT prove a V.A or
V.B violation directly, but they make it impossible for the Member
to defend against a V.A or V.B charge ex post. **Source:** CFA
Institute (2022) L1 Vol.6/pp.440-442.

## See Also

- [`cc-code-of-ethics`](cc-code-of-ethics.md) — the Code's
  third principle (use reasonable care and exercise independent
  professional judgment) is the foundation for V.A's diligence
  obligation
- [`eq-equity-cost-of-capital-estimation`](../05_equity/eq-equity-cost-of-capital-estimation.md)
  — the equity-valuation diligence in subcorpus 05 operationalizes
  V.A's reasonable-basis test for equity recommendations

## Escalate to Raw When

Open CFA L1 Vol.6 Reading 58 Standard V.A (pp.419-428), V.B (pp.429-440),
and V.C (pp.440-442) sections directly for the full Application
examples, particularly the third-party-research diligence fact patterns
and the opinion-vs-fact violation cases that the card omits. **Source:**
CFA Institute (2022) L1 Vol.6/pp.419-442.

- The reader needs the third-party-research due-diligence
  Application detail (when does a Member's reliance on sell-side
  research cross from reasonable to V.A violation? what verification
  is enough?). **Source:** CFA Institute (2022) L1 Vol.6/pp.422-428.
- The reader needs the V.B disclosure-of-material-change Application
  detail (when is a process change material? what disclosure timing
  satisfies the "prompt" requirement?). **Source:** CFA Institute
  (2022) L1 Vol.6/pp.430-440.
- The reader needs the V.C record-retention Application detail
  (what records are minimally sufficient? how do regulatory rules
  interact with the V.C Standard?). **Source:** CFA Institute
  (2022) L1 Vol.6/pp.440-442.
