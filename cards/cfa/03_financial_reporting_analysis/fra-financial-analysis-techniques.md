---
schema_version: "cacg.v0"
id: "fra-financial-analysis-techniques"
title: "Financial Analysis Techniques"
reading_id: "03_financial_reporting_analysis"
summary: "Lays out the CFA L1 toolkit for financial analysis — activity, liquidity, solvency, profitability, and valuation ratios; ratio relationships and limitations; DuPont decomposition of ROE; and equity / credit analysis applications under IFRS / US GAAP."
tags: ["financial-reporting", "financial-analysis"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1362:1979"
    chunk_hash: "13c90f6c0f47061eafc10f1d37b23a40ee2a3b9db6163f7e300edb4c96ab1bb9"
    page_range: [1362, 1362]
    quote: "In essence, an analyst converts data into financial metrics that assist in decision making"
    edge_type: "defines"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p319:0526"
    chunk_hash: "8a28bca47536dfca2143ab777a12de6daa7c7ad49c2473c3e53108f6152aaca6"
    page_range: [319, 320]
    quote: "The main aim of reformulating the balance sheet and income statements, however, is to discover the drivers of ROCE (return on common equity) and growth"
    edge_type: "supports"
card_hash: "12cece2468c47f85cad752a432ad2c086863cecf6f0bdade769da447da269499"
---
# Financial Analysis Techniques

## Intuition

The analyst's toolkit for reading financial statements has a few
core operations. First, scale: divide every line item by a common
base (revenue for the income statement, total assets for the
balance sheet) so cross-period and cross-firm comparisons become
direct. Second, ratios: combine line items from across statements
to reveal relationships invisible from any single statement
(profitability margins from the income statement; liquidity from
balance-sheet current accounts; solvency from balance-sheet leverage;
activity from how fast inventory and receivables cycle). Third,
trends: look at how each ratio moves across periods to identify
whether the firm is improving, deteriorating, or stable. **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.175-252.

The discipline of the toolkit is sequencing. The analyst rarely
starts by computing every possible ratio; instead, the question
drives the ratio choice. A liquidity question pulls quick ratio,
current ratio, cash-conversion cycle. A solvency question pulls
debt-to-equity, interest coverage, debt-to-EBITDA. A profitability
question pulls margin and return ratios decomposed via DuPont. The
toolkit's purpose is to translate raw statements into specific
diagnostic signals; it is not a sufficient substitute for thinking
about the underlying business. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.175-252.

```
+--------------------------------------------+
| Analyst Toolkit Layers                     |
+--------------------------------------------+
|  RAW STATEMENTS (3 statements + notes)     |
|       |                                    |
|       v                                    |
|  COMMON-SIZE                               |
|     Vertical (% of base)                   |
|     Horizontal (% growth)                  |
|       |                                    |
|       v                                    |
|  RATIOS by family:                         |
|     Activity   (turnover, days)            |
|     Liquidity  (current, quick, cash)      |
|     Solvency   (debt-equity, interest cov) |
|     Profitability (margin, return)         |
|       |                                    |
|       v                                    |
|  TRENDS: cross-period direction            |
|       |                                    |
|       v                                    |
|  COMPARISON: peer / industry / time        |
|       |                                    |
|       v                                    |
|  INVESTMENT / CREDIT CONCLUSIONS           |
+--------------------------------------------+
```

The diagram orders the toolkit from raw statements through
common-size to ratios to trends to comparison to conclusions. Each
layer adds analytical structure to the prior; conclusions rest on
all the layers together. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.175-252.

## Definition

Common-size analysis is the technique of restating financial
statement line items as percentages of a common base. Vertical
common-size analysis divides every income-statement line by
revenue (yielding margins at every level) and every balance-sheet
line by total assets (yielding the asset and liability composition
as percentages). Horizontal common-size analysis divides every
line by its prior-period value, yielding period-over-period growth
rates. Both presentations remove scale differences across firms
and across periods. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.175-252.

The four ratio families partition the analyst's standard ratios by
the question they answer. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.175-252.

- Activity ratios measure how efficiently the firm uses its assets.
  Inventory turnover (`COGS / Avg Inventory`), receivables turnover
  (`Revenue / Avg AR`), days inventory on hand (`365 / Inventory
  turnover`), days sales outstanding (`365 / Receivables turnover`),
  payables turnover (`Purchases / Avg AP`), days payable outstanding,
  and the cash conversion cycle (`DIO + DSO − DPO`) capture the
  firm's working-capital efficiency. **Source:** CFA L1 Curriculum
  (2022) Vol.3/pp.175-252.
- Liquidity ratios measure the firm's ability to meet short-term
  obligations. Current ratio (`Current Assets / Current
  Liabilities`), quick ratio (`(Current Assets − Inventory) /
  Current Liabilities`), and cash ratio (`(Cash + Marketable
  Securities) / Current Liabilities`) increase in conservatism
  from current down to cash. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.175-252.
- Solvency ratios measure the firm's longer-term financial
  obligations. Debt-to-equity (`Total Debt / Total Equity`),
  debt-to-assets (`Total Debt / Total Assets`), debt-to-EBITDA
  (`Total Debt / EBITDA`), interest coverage (`EBIT / Interest
  Expense`), and fixed-charge coverage (`(EBIT + Lease Payments) /
  (Interest Expense + Lease Payments)`) capture leverage and
  ability to service it. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.175-252.
- Profitability ratios measure the firm's ability to generate
  earnings from its activities. Gross margin (`Gross Profit /
  Revenue`), operating margin (`EBIT / Revenue`), net margin (`NI
  / Revenue`), return on assets (`NI / Avg Assets`), return on
  equity (`NI / Avg Equity`), return on invested capital, and the
  EBITDA margin (`EBITDA / Revenue`) sit at the top of the analyst's
  diagnostic surface. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.175-252.

Per-share metrics combine ratios with share count: earnings per
share (basic and diluted), book value per share, dividends per
share. The diluted-EPS calculation accounts for potential dilution
from convertible securities, options, and warrants using the
treasury-stock and if-converted methods. **Source:** CFA L1
Curriculum (2022) Vol.3/pp.175-252.

## Mathematical Reasoning

The analyst's standard sequence is to compute common-size
statements, then the activity / liquidity / solvency / profitability
ratio families, then period-over-period growth rates, then compare
across periods, peers, and industry benchmarks. The sequence yields
a set of diagnostic signals; no single signal suffices for an
investment conclusion, but the joint pattern across ratios is
informative. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.175-252.

The cash conversion cycle (CCC) is a synthetic activity ratio that
captures how long the firm waits between paying for inventory and
collecting cash from customers: `CCC = DIO + DSO − DPO`. A short
CCC reflects efficient working-capital management; a long CCC ties
up cash. The CCC's components decompose into operations levers the
manager can adjust: tighten credit policies to shorten DSO, improve
inventory turnover to shorten DIO, negotiate longer payment terms
to lengthen DPO. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.175-252.

The leverage ratios interact with profitability through the
DuPont decomposition. ROE can be decomposed as `ROE = NetMargin ×
AssetTurnover × FinancialLeverage`; the three-factor form makes
explicit that ROE is driven by operating efficiency (margin and
turnover) plus capital structure (leverage). The detailed DuPont
decomposition mechanics (3-step + 5-step extension) are treated in
the next FRA card. **Source:** Penman (2013) Ch.10 pp.292-341.

The credit-analysis subset of ratios receives outsized attention
when the analyst's question is debt-service capacity rather than
equity returns. Interest coverage and debt-to-EBITDA are the most
cited credit ratios; rating agencies use ratio-based grids that
map specific levels of these ratios to indicative credit ratings.
The credit-analysis question also pulls cash-flow-based ratios
(`CFO / Total Debt`, `CFO / Interest Paid`) because cash flow is
what services debt, not accrual earnings. **Source:** CFA L1
Curriculum (2022) Vol.3/pp.175-252.

The analyst's interpretation of any single ratio depends heavily on
context: industry-typical levels (retailers run higher inventory
turnover than capital-equipment manufacturers); cycle position
(margin compression in a downturn is normal, not a red flag in
isolation); accounting-policy choices (LIFO vs FIFO inventory cost
flow affects inventory turnover significantly under rising prices).
The analyst's discipline is to read each ratio in its peer-firm /
peer-period context, not against absolute thresholds. **Source:**
Penman (2013) Ch.10 pp.292-341.

## See Also

- [`fra-cash-flow-statement-mechanics`](./fra-cash-flow-statement-mechanics.md) — CFO is the input to cash-flow-based credit ratios
- [`fra-income-statement-foundations`](./fra-income-statement-foundations.md) — margin ratios at every income-statement level
- [`fra-balance-sheet-foundations`](./fra-balance-sheet-foundations.md) — current-vs-non-current classification drives liquidity and solvency ratios

## Escalate to Raw When

Open the CFA L1 curriculum Vol.3 R20 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.175-252.

- the analyst is constructing a credit-analysis report and needs the
  curriculum's full set of credit-ratio definitions plus rating-
  agency-style grid mappings. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.175-252.
- the firm operates in an unusual industry where standard ratios
  need adaptation (banking, insurance, real estate) and the analyst
  needs the curriculum's industry-specific guidance. **Source:** CFA
  L1 Curriculum (2022) Vol.3/pp.175-252.
- the analyst is computing per-share metrics with significant
  dilutive securities outstanding and needs the curriculum's
  treasury-stock-method and if-converted-method detail. **Source:**
  CFA L1 Curriculum (2022) Vol.3/pp.175-252.
