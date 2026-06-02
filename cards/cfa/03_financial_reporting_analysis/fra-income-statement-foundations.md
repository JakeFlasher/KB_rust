---
schema_version: "cacg.v0"
id: "fra-income-statement-foundations"
title: "Income Statement Foundations"
reading_id: "03_financial_reporting_analysis"
summary: "Laying out the income statement under IFRS / US GAAP — revenue, expenses, gains, losses flowing through gross profit, operating income, EBT, to net income, then bridging into comprehensive income via OCI. The recurring vs non-recurring distinction grounds sustainable-earnings estimation."
tags: ["financial-reporting", "income-statement"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1191:1740"
    chunk_hash: "bcd97bc02d7241161437812a8c15c5979629bb394cebc40c5e6125969560985e"
    page_range: [1191, 1192]
    quote: "The income statement communicates how much revenue the company generated during a period and what costs it incurred in connection with generating that revenue."
    edge_type: "defines"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p065:0101"
    chunk_hash: "6bb0dedfc1a0ecfc8e578a0bf2d644f40a979d568fecec5f3664cfbeeea25d12"
    page_range: [65, 65]
    quote: "The income statement displays the sources of net income, broadly classified as revenue (value coming in from selling products) and expenses (value going out in earning revenue)."
    edge_type: "supports"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p137:0228"
    chunk_hash: "e34c12870566a37189a2dc95b562a7db37aab0af2ce1d6b46fe6a02cad18d9f4"
    page_range: [137, 138]
    quote: "Both techniques prove to be unsatisfactory, for the simple reason that cash flows do not capture value added in a business."
    edge_type: "supports"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p287:0474"
    chunk_hash: "12698db5a18ed921ab00f863bb62cf5263e16ef27814dde3fa98eac13a0ac427"
    page_range: [287, 287]
    quote: "Preferred stock is included in shareholders’ equity in the GAAP statement, but it is a liability for the common shareholders."
    edge_type: "supports"
card_hash: "6fbaeab48410efd5f3387bcd85880e8456fb8c6fd7e54bf6af4fa39128d08fcb"
---
# Income Statement Foundations

## Intuition

The income statement reports the firm's revenues, expenses, gains,
and losses over a period and explains in flow form why the equity
stock changed via operating activity. Where the balance sheet is a
snapshot, the income statement is a movie. The bottom line — net
income — flows into retained earnings on the balance sheet and is
the headline number that the firm presents to capital markets.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.5-62.

The income statement organizes itself by revenue source and expense
function. Operating activity (revenue from the firm's main
business, the costs of producing it, and operating overhead) sits
above the line; non-operating items (financing income and expense,
investment gains and losses, tax) sit below. The structure invites
the analyst to ask three questions in order: how much did the firm
sell, how efficiently did it produce what it sold, and how much of
the operating profit survived financing and tax leakage to reach
shareholders. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.5-62.

```
+----------------------------------------+
| INCOME STATEMENT (Period Flow)         |
+----------------------------------------+
|  Revenue (net sales)                   |
|  - Cost of goods sold                  |
|  = Gross Profit                        |
|  - Selling, general, administrative    |
|  - R&D                                 |
|  - Depreciation & amortization         |
|  = Operating Income (EBIT)             |
|  + Other income / - Other expense      |
|  - Interest expense                    |
|  + Interest income                     |
|  = Earnings before tax (EBT)           |
|  - Income tax expense                  |
|  = Net Income                          |
+----------------------------------------+
|  Net Income                            |
|  + OCI items                           |
|  = Comprehensive Income                |
+----------------------------------------+
```

The schematic above orders the income statement from top-line
revenue down through gross profit, operating income, EBT, and net
income, then extends into comprehensive income via the OCI bridge.
The intermediate subtotals (gross profit, operating income, EBT)
are the analyst's primary diagnostic anchors — each is a margin
that the analyst tracks across periods and against peers. **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.5-62.

## Definition

The income statement is the financial statement that reports the
firm's revenues, expenses, gains, and losses over a specified
reporting period and yields net income (or loss) as the bottom-line
flow that closes into retained earnings. Each component has a
specific recognition criterion. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.5-62.

Revenue is the inflow of economic benefits from the firm's ordinary
activities, recognized when the firm has substantially performed
under a contract and collection is reasonably assured. Under the
current revenue-recognition standard (ASC 606 / IFRS 15), revenue
is recognized over the period in which the firm transfers control
of the promised good or service to the customer, with the amount
allocated to performance obligations under the contract. **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.5-62.

Expenses are outflows of economic benefits incurred to generate
revenue, recognized in the period whose revenue they helped
produce (the matching principle). Cost of goods sold matches against
sales of inventory; depreciation matches against use of long-lived
assets; selling, general, and administrative expenses match against
the period's overall operating activity. **Source:** CFA L1
Curriculum (2022) Vol.3/pp.5-62.

Gains are increases in equity from peripheral transactions outside
the firm's main business (sale of a non-operating asset at a price
above book value); losses are the symmetric decrease. Gains and
losses appear separately from operating revenue and expenses so the
analyst can identify them as non-recurring and exclude them when
estimating sustainable earnings. **Source:** CFA L1 Curriculum
(2022) Vol.3/pp.5-62.

Comprehensive income extends net income to include other
comprehensive income (OCI) items that bypass net income but still
affect equity. OCI items include foreign currency translation
adjustments, unrealized gains and losses on available-for-sale
securities, certain derivative cash-flow-hedge adjustments, and
specified pension actuarial gains and losses. The two-step bridge
`Net Income + OCI = Comprehensive Income` is the formal
presentation. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.5-62.

## Mathematical Reasoning

The income-statement equation reads top to bottom as a sequence of
subtractions and additions. Letting `Rev` denote revenue, `COGS`
cost of goods sold, `OpEx` operating expenses (SG&A + R&D + D&A),
`OtherInc` other operating income / expense, `IntExp` interest
expense net of interest income, and `Tax` income tax expense, the
equation is `NI = Rev - COGS - OpEx + OtherInc - IntExp - Tax`. The
intermediate subtotals are `Gross Profit = Rev - COGS`,
`Operating Income (EBIT) = Gross Profit - OpEx + OtherInc`, and
`EBT = EBIT - IntExp`. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.5-62.

Each subtotal divided by revenue yields a margin: gross margin
(Gross Profit / Rev), operating margin (EBIT / Rev), pretax margin
(EBT / Rev), and net margin (NI / Rev). The margins are the
income-statement's primary diagnostic surface — the analyst reads
them across periods and against peers to identify operating
efficiency trends and financing-driven profitability shifts.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.5-62.

The income statement articulates with the balance sheet via the
retained-earnings roll: `RE_t = RE_{t-1} + NI_t - Div_t`. Net income
flows into retained earnings; dividends declared during the period
flow out. The articulation is what makes the income statement
matter for valuation — the period's accrual earnings ultimately
update the equity stock that valuation models discount. **Source:**
Penman (2013) Ch.2 pp.32-71.

The income statement also articulates with the cash flow statement
via the indirect-method bridge: operating cash flow equals net
income adjusted for non-cash items (D&A, SBC, impairments) and
working-capital changes (Δ AR, Δ Inventory, Δ AP, Δ Accrued Liab).
The bridge formalizes the accrual-vs-cash divergence — net income
and operating cash flow differ precisely by the period's accrual
adjustments and reverse to convergence over the firm's life.
**Source:** Penman (2013) Ch.4 pp.110-139.

Stock-based compensation expense is the income-statement entry
that records the cost of equity instruments granted to employees
as compensation. The expense equals the grant-date fair value of
the instrument, allocated to the income statement over the
employee's service (vesting) period. The expense reduces net
income while increasing additional paid-in capital on the balance
sheet — no cash leaves the firm at recognition, so the entry is a
non-cash compensation cost. The non-cash add-back reverses the
income-statement expense in the indirect-method CFO bridge, which
is why stock-based compensation appears as a CFO non-cash
adjustment alongside depreciation and amortization. Issuer cash
inflow arises only when employees exercise stock options at the
strike price; that exercise inflow is a financing-activity item
rather than an operating-activity item. Restricted-stock vesting
is itself non-cash for the issuer; specialist tax-withholding and
settlement mechanics that may produce ancillary cash flows are
outside the L1 recognition frame. **Source:** Penman (2013) Ch.9
pp.258-291.

The analyst's most common transformation of the income statement is
common-size analysis: divide every line item by revenue and report
the result as a percentage of sales. Common-size statements make
cross-period and cross-firm comparisons direct because they remove
scale differences. They also surface unusual cost-structure shifts
— a sudden widening of cost of goods sold as a share of revenue is
a clear diagnostic signal even when the absolute dollar values look
smooth. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.5-62.

The recurring-vs-non-recurring distinction separates persistent
operating activity from one-off items. Recurring items (revenue
from ordinary business, regular operating expenses) feed the
analyst's forecast of future earnings; non-recurring items
(restructuring charges, impairments, gains from asset sales,
discontinued operations) describe the period but should not be
extrapolated. Penman frames this as the persistent-vs-transitory
decomposition that grounds residual-earnings valuation. **Source:**
Penman (2013) Ch.2 pp.32-71.

## See Also

- [`fra-cash-vs-accrual-accounting`](./fra-cash-vs-accrual-accounting.md) — net income (income statement) and operating cash flow disagree by construction; the bridge is informative
- [`fra-double-entry-mechanics`](./fra-double-entry-mechanics.md) — revenue and expense accounts close into retained earnings via the double-entry rule

## Escalate to Raw When

Open the CFA L1 curriculum Vol.3 R17 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.5-62.

- the firm has unusual revenue arrangements (long-term contracts
  with milestone payments, multi-element bundled offerings,
  consignment sales) where the recognition timing materially affects
  reported revenue — the curriculum's recognition framework is the
  authoritative reference. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.5-62.
- the firm reports significant OCI components and the analyst needs
  to distinguish reclassifications (items that move from OCI to net
  income on realization, like AFS-security gains) from permanent
  OCI items (currency translation differences) — the curriculum
  presents the per-component treatment. **Source:** CFA L1
  Curriculum (2022) Vol.3/pp.5-62.
- the analyst is constructing a sustainable-earnings estimate and
  needs the textbook's framework for separating persistent from
  transitory components — Penman's Ch.2 supplements the LOS-level
  curriculum content. **Source:** Penman (2013) Ch.2 pp.32-71.
