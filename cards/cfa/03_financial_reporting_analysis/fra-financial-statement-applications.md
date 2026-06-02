---
schema_version: "cacg.v0"
id: "fra-financial-statement-applications"
title: "Financial Statement Applications"
reading_id: "03_financial_reporting_analysis"
summary: "Framing the FRA toolkit application by analyst question: equity-valuation input construction (sustainable earnings, FCF, ROE), credit assessment (solvency + cash-flow coverage), M&A screening (full ratio battery + quality flags), and segment analysis. Discipline is auditable selection of tools per question with explicit assumptions."
tags: ["financial-reporting", "financial-statement"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1762:2545"
    chunk_hash: "790e4d8557bce1740f346a31cb955a21d66ec69f5f4c9b7ad5feafce4d4abff4"
    page_range: [1762, 1762]
    quote: "Whatever the techniques adopted, the analytical focus of credit analysis is on debt-paying ability."
    edge_type: "defines"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p369:0600"
    chunk_hash: "25a804faca9c7e68dc156b10338ab97f59dccc1235bfc6538ccb100320b75e35"
    page_range: [369, 369]
    quote: "The cash flow statement describes the cash generation in a business, and reformulation highlights the cash flows that are important to analysis."
    edge_type: "supports"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p319:0525"
    chunk_hash: "1441f55a02c880aa1b989d5b0f3c106edba02cbaec72cdbd80004cede903e4d9"
    page_range: [319, 319]
    quote: "Thus the analysis begins with a reformulation of the statements, following the templates of Chapter 8, to distinguish operating activities from financing activities."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1421:2066"
    chunk_hash: "c960e167f4ffc555ec5fae75fa80c6c5d8fe742e71565e2d98eb393ac9d46b98"
    page_range: [1421, 1422]
    quote: "Companies must disclose the factors used to identify reportable segments and the types of products and services sold by each reportable segment."
    edge_type: "supports"
card_hash: "73b014b03813a2cf0a41ac3e7dd96af096dab420d48cd15c5503fffa6b2b4e42"
---
# Financial Statement Applications

## Intuition

The earlier FRA cards build the analyst's toolkit: the four
statements, articulation, accrual mechanics, ratio families, DuPont
decomposition, earnings quality, reporting quality. The applications
card asks: when the analyst sits down to answer a specific
investment question, which subset of the toolkit applies, and in
what sequence? Different questions pull different tools. An equity
valuation question pulls profitability ratios and earnings-quality
diagnostics to estimate sustainable earnings. A credit question
pulls solvency ratios and cash-flow-based coverage measures. An
M&A target screen pulls the full ratio battery against industry
benchmarks. A segment analysis pulls disaggregated revenue and
operating data. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.561-598.

The applications discipline is choosing the right tools and
applying them in a defensible sequence. The analyst's report is
auditable: each conclusion should trace back to the specific
statements and ratios that supported it, with explicit assumptions
called out. The applications framework is therefore as much about
analytical organization as about specific computations. **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.561-598.

```
+--------------------------------------------+
| FRA Application by Question                |
+--------------------------------------------+
|  Question: Equity valuation                |
|     Tools:                                 |
|       - Profitability ratios               |
|       - DuPont decomposition               |
|       - Earnings quality                   |
|       - Sustainable-earnings estimate      |
|     Output: Valuation input (NI, FCF, ROE) |
+--------------------------------------------+
|  Question: Credit assessment               |
|     Tools:                                 |
|       - Solvency ratios                    |
|       - Interest coverage, debt/EBITDA     |
|       - Cash-flow-based coverage           |
|       - Liquidity ratios                   |
|     Output: Default-risk indicator         |
+--------------------------------------------+
|  Question: M&A target screen               |
|     Tools:                                 |
|       - Common-size statements             |
|       - Full ratio battery                 |
|       - Peer comparison                    |
|       - Reporting-quality flag             |
|     Output: Long / short list of targets   |
+--------------------------------------------+
|  Question: Segment / business analysis     |
|     Tools:                                 |
|       - Segment disclosures                |
|       - Operating-margin by segment        |
|       - Capex allocation by segment        |
|     Output: Segment-by-segment view        |
+--------------------------------------------+
```

The diagram maps four common analyst questions to the FRA-toolkit
subsets each pulls. The mapping is not exclusive — most actual
analyses combine multiple questions, and the toolkit subsets
overlap. But the diagram captures the typical first cut: pick the
question, identify the tool subset, sequence the application.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.561-598.

## Definition

The financial-statement-applications framework is the structured
process by which the analyst translates raw published statements
plus the analytical toolkit into specific investment, credit, or
deal-screening conclusions. The framework has three structural
elements. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.561-598.

- The analytical question — what specific decision the analysis
  supports (equity buy/sell/hold, credit rating, M&A target
  inclusion, segment investment thesis). The question determines
  which subset of FRA techniques applies. **Source:** CFA L1
  Curriculum (2022) Vol.3/pp.561-598.
- The data-and-tool selection — which statements, footnotes, and
  ratio families produce the most relevant signal for the chosen
  question. The analyst reads the question through the toolkit,
  selecting tools that bear on the question and skipping those
  that do not. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.561-598.
- The conclusion-and-recommendation step — synthesizing the
  selected analyses into a defensible conclusion with explicit
  assumptions called out. The conclusion is auditable: every claim
  should trace to specific statements and ratios. **Source:** CFA
  L1 Curriculum (2022) Vol.3/pp.561-598.

The four standard application categories the CFA L1 curriculum
recognizes are equity valuation, credit assessment, M&A screening,
and segment / peer-group analysis. Each has its own toolkit subset
and its own typical output. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.561-598.

The applications framework also incorporates the analyst-treatment
discipline for data adjustments. Two types of adjustments are
common: comparability adjustments (LIFO-to-FIFO conversion for
US-LIFO firms compared to IFRS peers; lease-on-balance-sheet
adjustment for cross-period comparison spanning the IFRS 16 / ASC
842 adoption boundary; depreciation-method normalization between
peers using different methods), and quality adjustments (removing
non-recurring items from earnings; discounting accrual-heavy
earnings; flagging firms with reporting-quality concerns). The
adjustments produce comparable, sustainable-basis numbers from
disparate published surfaces. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.561-598.

## Mathematical Reasoning

For equity valuation, the FRA toolkit produces three primary
inputs: a sustainable earnings estimate (`NI` adjusted for
non-recurring items and discounted for accrual-quality concerns),
a free-cash-flow estimate (`CFO − CapEx` for FCFF; `CFO −
CapEx − Net Debt Issuance` for FCFE), and a normalized return
metric (sustainable ROE). These feed downstream valuation models —
intrinsic-value DCF, residual-earnings, justified-multiple
frameworks — which the existing 05 Equity cards cover in detail.
The FRA application is the input-construction step. **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.561-598.

For credit assessment, the FRA toolkit produces solvency and
coverage indicators that map to default-probability proxies. The
cash-flow-based ratios receive emphasis because cash flow services
debt: `CFO / Total Debt` is more diagnostic than `NI / Total
Debt`; `CFO / Interest Paid` is more diagnostic than `EBIT /
Interest Expense` for marginal cases. The credit-application output
is a default-risk indicator that feeds into spread-pricing or
into rating-grid placement. The detailed credit-spread machinery
is treated in the existing 06 fixed-income cards. **Source:** CFA
L1 Curriculum (2022) Vol.3/pp.561-598.

For M&A screening, the FRA toolkit produces a comparable-firm
ranking. The analyst computes the full ratio battery for a target
universe, then filters: firms with reporting-quality red flags
out; firms with extreme leverage or liquidity-ratio outliers out;
firms with stable margin and turnover patterns in. The remaining
short list goes to deeper diligence. The screen's value is in
narrowing a large universe into a tractable candidate set;
synthesis of the candidates' strategic fit lives outside the FRA
toolkit. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.561-598.

For segment / peer-group analysis, the FRA toolkit pulls
disaggregated reporting (segment revenue, operating income, capex
by segment) plus footnote disclosures (geographic exposure,
product-line concentration). The output is a per-segment view of
profitability and capital deployment that supports thesis
construction at the business-line level rather than the firm level.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.561-598.

The standards-level framing for segment disclosure follows IFRS 8
and FASB ASC Topic 280: a firm reports the operating segments its
senior management actually reviews, with each reportable segment
disclosing segment profit or loss, segment assets, segment revenue
(distinguishing external customers from inter-segment revenue),
and reconciliation of segment totals to the consolidated income
statement and balance sheet. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.234-252.

The applications-level analyst uses these disclosures to construct
industry-mix concentration, geographic exposure, product-line
profitability, and segment-level DuPont decomposition. The
disclosure-quality assessment (whether the segment categorization
matches the firm's actual operating reality, whether the firm
aggregates segments in ways that obscure cross-segment differences)
layers the reporting-quality framework on top of the disaggregated
footnote information. **Source:** Penman (2013) Ch.10 pp.292-341.

The contingent-liability adjustment that the non-current-liabilities
card flagged for liquidity assessment surfaces in the credit
application: large pending litigation contingencies, contingent
acquisition consideration, third-party-obligation guarantees all
sit outside reported total liabilities but represent potential
future cash outflows. The credit analyst's standard practice is to
incorporate material contingencies into stress-tested liquidity
analysis even though they do not change reported leverage ratios.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.561-598.

Penman frames the applications surface around accounting
informativeness for security selection. The FRA toolkit's outputs
are inputs to security-level investment decisions; the analyst's
discipline is to maintain a documented trail from published data
to adjustments applied to the conclusion reached. Conclusions
should reference the specific accounting items and adjustments
they rest on rather than abstract categories, so a reader can
re-trace the analyst's reasoning back to the source statements.
**Source:** Penman (2013) Ch.11 pp.342-363.

## See Also

- [`fra-financial-analysis-techniques`](./fra-financial-analysis-techniques.md) — the toolkit whose subsets each application pulls
- [`fra-reporting-quality-framework`](./fra-reporting-quality-framework.md) — applications use the reporting-quality flag as a target screen
- [`fra-ratio-decomposition-dupont`](./fra-ratio-decomposition-dupont.md) — DuPont's component view drives sustainable-ROE estimates for valuation applications

## Escalate to Raw When

Open the CFA L1 curriculum Vol.3 R26 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.561-598.

- the analyst is constructing a comprehensive equity-valuation
  workup and needs the curriculum's framework for sequencing the
  FRA-input-construction stages. **Source:** CFA L1 Curriculum
  (2022) Vol.3/pp.561-598.
- the analyst is screening an M&A target universe and needs the
  curriculum's per-screening-step ratio battery and reporting-
  quality red-flag list. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.561-598.
- the analyst is constructing a credit report incorporating
  material contingent liabilities and needs the curriculum's
  framework for stress-tested liquidity assessment with off-balance-
  sheet contingencies. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.561-598.
