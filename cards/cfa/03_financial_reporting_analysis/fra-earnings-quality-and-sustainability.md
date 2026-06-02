---
schema_version: "cacg.v0"
id: "fra-earnings-quality-and-sustainability"
title: "Earnings Quality and Sustainability"
reading_id: "03_financial_reporting_analysis"
summary: "Frames earnings quality as a separate dimension from earnings level — what makes reported earnings persistent vs transitory, how the accrual share of earnings signals quality, why high-accrual firms tend to underperform on subsequent-period earnings, and how to translate quality diagnostics into sustainable-earnings estimates for valuation."
tags: ["financial-reporting", "earnings-quality"]
citations:
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p423:0684"
    chunk_hash: "8756935d808485917cb413a4930b115dfe5ecf820a4298caa01f51627b4aec2a"
    page_range: [423, 423]
    quote: "Earnings that can repeat in the future, and grow, are called sustainable earnings, persistent earnings, core earnings"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1362:1979"
    chunk_hash: "13c90f6c0f47061eafc10f1d37b23a40ee2a3b9db6163f7e300edb4c96ab1bb9"
    page_range: [1362, 1362]
    quote: "In essence, an analyst converts data into financial metrics that assist in decision making"
    edge_type: "supports"
card_hash: "60d42c032ede7584582cdb1d48f8dcd9d72dc581d645bd1d699e4757970d093e"
---
# Earnings Quality and Sustainability

## Intuition

Two firms can report the same net income for a period and yet have
very different prospects for sustaining it. One firm's earnings come
from operating activity that has been growing steadily and that the
firm expects to repeat next period; the other's earnings come from
a one-time gain on the sale of an unused factory. Both report the
same headline number, but the first earnings are sustainable and
the second are not. Earnings quality is the dimension that separates
the two. The analyst's question is not just "how much did the firm
earn?" but "how persistent is the earning power that produced this
number?" **Source:** Penman (2013) Ch.13 pp.392-433.

The accrual share of earnings is the primary diagnostic. Earnings
that are mostly cash (operating cash flow close to net income)
tend to persist; earnings that are mostly accrual (net income far
above operating cash flow because of working-capital build-up,
deferred-revenue timing, depreciation choice, etc.) tend to revert.
The accrual-anomaly literature documents that high-accrual firms
underperform low-accrual firms in subsequent periods — the market
appears to underweight the persistence signal that accruals carry.
**Source:** Penman (2013) Ch.13 pp.392-433.

```
+--------------------------------------------+
| Earnings Quality Decomposition             |
+--------------------------------------------+
|  Net Income (the headline number)          |
|       |                                    |
|       +---- Cash component (CFO)           |
|       |     - high persistence             |
|       |     - low reversion                |
|       |                                    |
|       +---- Accrual component (NI - CFO)   |
|             - lower persistence            |
|             - higher reversion             |
|                                            |
|  Diagnostic ratios:                        |
|     CFO / NI ratio                         |
|     Accrual ratio = (NI - CFO) / Avg NOA   |
|     Operating accruals / Total accruals    |
+--------------------------------------------+
```

The diagram decomposes net income into cash and accrual components
and lists the standard diagnostic ratios. A firm with a high
CFO/NI ratio and a low accrual ratio has high-quality earnings; a
firm with the opposite has lower-quality earnings the analyst
should discount when forecasting. **Source:** Penman (2013) Ch.13
pp.392-433.

## Definition

Earnings quality refers to the degree to which reported earnings
reflect sustainable economic performance rather than transitory or
manipulation-driven items. High-quality earnings persist into
future periods and translate predictably into cash flow; low-quality
earnings reverse, are non-recurring, or have been managed (within
or outside GAAP) to obscure underlying performance. **Source:**
Penman (2013) Ch.13 pp.392-433.

The persistence of earnings is the empirical regularity that
high-quality earnings exhibit: a firm whose earnings are sustainable
will report similar earnings (adjusted for systematic growth) in
subsequent periods. The persistence of cash earnings is high; the
persistence of accrual earnings is lower because accruals reverse
when the underlying economic event matures (a deferred-revenue
balance becomes recognized revenue when the firm performs; a
working-capital build reverses when inventory ships and receivables
collect). **Source:** Penman (2013) Ch.13 pp.392-433.

The standard earnings-quality diagnostics partition into three
families. **Source:** Penman (2013) Ch.13 pp.392-433.

- Cash-vs-accrual ratios: `CFO / NI` (high values signal
  high-quality earnings); `(NI − CFO) / Avg NOA` (the accrual
  ratio, where high values signal accrual-heavy earnings); the
  trend in CFO relative to NI across periods. **Source:** Penman
  (2013) Ch.13 pp.392-433.
- Persistence-of-earnings indicators: separation of recurring
  vs non-recurring items (one-time gains and losses, restructuring
  charges, asset impairments); the sustainable component of
  earnings excludes the non-recurring items. **Source:** Penman
  (2013) Ch.13 pp.392-433.
- Accrual-anomaly indicators: working-capital accrual changes
  (Δ AR, Δ Inventory, Δ AP); long-term-accrual changes (deferred
  taxes, pension accruals); ratio of accruals to total assets as a
  signal of accrual-driven earnings inflation. **Source:** Penman
  (2013) Ch.13 pp.392-433.

Earnings management is the broader phenomenon: management's use of
discretion in accounting choices and estimates to influence
reported earnings toward a target. Examples include accelerating
revenue recognition near period-end, deferring expense recognition,
selecting accounting methods that smooth earnings, or building
cookie-jar reserves in good periods to release in bad periods.
Earnings management within GAAP is legal but reduces earnings
quality; earnings management outside GAAP becomes misrepresentation
or fraud. The full taxonomy is treated in the next FRA card on
reporting quality. **Source:** Penman (2013) Ch.13 pp.392-433.

## Mathematical Reasoning

The decomposition `NI = Cash Component + Accrual Component` where
`Cash Component = CFO` and `Accrual Component = NI − CFO` is the
algebraic restatement of the indirect-method bridge. Persistence
analysis treats the two components as having different long-run
dynamics: cash earnings tend to follow a near-random-walk; accrual
earnings tend to mean-revert. **Source:** Penman (2013) Ch.13
pp.392-433.

The accrual ratio is normalized by net operating assets so that
firms of different size are comparable: `Accrual Ratio = (NI −
CFO) / Avg NOA` where `NOA = Operating Assets − Operating
Liabilities`. A high accrual ratio (sustained period after period)
signals that net income is running ahead of cash extraction; this
divergence often reverses. The diagnostic does not directly tell
the analyst whether the firm is committing earnings management —
it could be growing inventory in advance of legitimate sales — but
it raises a flag that the analyst should examine the working-capital
build's drivers. **Source:** Penman (2013) Ch.13 pp.392-433.

The accrual-anomaly empirical regularity is that high-accrual firms
(top decile by accrual ratio) underperform low-accrual firms
(bottom decile) on a risk-adjusted basis in subsequent periods. The
literature interprets this as evidence that market participants
underweight the lower persistence of accrual earnings. The
implication for the equity analyst: discount the accrual portion of
reported earnings when forecasting sustainable earnings; the cash
portion is a more reliable input. The implication for the credit
analyst: cash-flow-based credit ratios are less manipulable than
accrual-based earnings ratios. **Source:** Penman (2013) Ch.13
pp.392-433.

The sustainable-earnings estimate is the analyst's working number
for valuation. It strips out non-recurring items (gains on asset
sales, restructuring charges, large impairments, tax-rate
adjustments from rate changes) and adjusts for accrual-quality
concerns (where the analyst suspects the accrual is unusually
inflated). The result is an earnings number that the analyst would
project forward at a sustainable growth rate; in residual-earnings
valuation and in justified-multiple frameworks, the sustainable-
earnings number is the input that drives the implied equity value.
**Source:** Penman (2013) Ch.13 pp.392-433.

The CFA L1 framing extends this to the analytical-techniques
toolkit: the curriculum covers the same cash-vs-accrual diagnostic
plus the recurring-vs-non-recurring distinction. The vocabulary is
consistent with Penman's. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.175-252.

## See Also

- [`fra-cash-vs-accrual-accounting`](./fra-cash-vs-accrual-accounting.md) — the cash-vs-accrual conceptual basis underlying earnings quality
- [`fra-income-statement-foundations`](./fra-income-statement-foundations.md) — the income statement's recurring-vs-non-recurring distinction is one earnings-quality lens
- [`fra-ratio-decomposition-dupont`](./fra-ratio-decomposition-dupont.md) — DuPont's components also reveal which lever is driving the firm's earnings

## Escalate to Raw When

Open Penman Ch.13 directly when any of the criteria below applies.
**Source:** Penman (2013) Ch.13 pp.392-433.

- the firm reports earnings whose cash backing is unusually weak
  (CFO well below NI for multiple periods) and the analyst needs
  Penman's framework for separating growth-investment causes from
  accrual-quality concerns. **Source:** Penman (2013) Ch.13
  pp.392-433.
- the analyst is constructing a sustainable-earnings estimate for
  valuation and needs Penman's discussion of which adjustments to
  reported earnings produce the most stable forecast input.
  **Source:** Penman (2013) Ch.13 pp.392-433.
- the firm's recent period has unusual non-recurring items
  (restructuring, asset impairments, tax-law-change effects) and
  the analyst needs the textbook treatment for separating
  sustainable from transitory components. **Source:** Penman (2013)
  Ch.13 pp.392-433.
