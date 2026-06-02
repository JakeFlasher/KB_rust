---
schema_version: "cacg.v0"
id: "fra-cash-flow-statement-mechanics"
title: "Cash Flow Statement Mechanics"
reading_id: "03_financial_reporting_analysis"
summary: "Lays out the cash flow statement's structure under the IFRS / US GAAP framework — operating, investing, and financing classification; direct vs indirect method for the operating section; how IFRS/US GAAP differ on classification choices for interest, dividends, and tax; and how the indirect-method bridge from net income to operating cash flow is constructed."
tags: ["financial-reporting", "cash-flow"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1305:1905"
    chunk_hash: "20a48e7d2e7310e23de480ee65720530202b65a7d55b338646f2fbc92ba1354f"
    page_range: [1305, 1306]
    quote: "The cash flow statement provides information about a company’s cash receipts and cash payments during an accounting period"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1306:1907"
    chunk_hash: "0dd282dccbc72bbadf7f22789dc2dc87723da0dce220711e6bf647f6749ec985"
    page_range: [1306, 1307]
    quote: "The cash flow statement has subsections relating specific items to the operating, investing, and financing activities of the company"
    edge_type: "defines"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p137:0228"
    chunk_hash: "e34c12870566a37189a2dc95b562a7db37aab0af2ce1d6b46fe6a02cad18d9f4"
    page_range: [137, 138]
    quote: "The cash flow statement tracks operating and investment activities with cash accounting"
    edge_type: "supports"
card_hash: "dc8f8e94c481ad1946bf8da1aeb3127c8ac6dd6d80ee55524fbaa8405e0d0181"
---
# Cash Flow Statement Mechanics

## Intuition

The cash flow statement explains why the cash line on the balance
sheet changed between two reporting dates. Three buckets — operating,
investing, financing — partition the period's cash movements by the
business activity that generated them. Operating cash flow (CFO)
reports cash from the core business activity that produces revenue;
investing cash flow (CFI) reports cash spent on or received from
long-lived assets and certain investments; financing cash flow (CFF)
reports cash from or to the firm's capital providers (debt and
equity holders). Their sum equals the period's net change in cash.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.119-174.

The operating section can be presented two ways. The direct method
lists actual operating cash receipts (cash from customers) and
disbursements (cash to suppliers, cash to employees, cash for
interest, cash for taxes); it is the cleaner presentation but
requires firms to track cash flows directly, which most do not.
The indirect method starts from net income and adjusts for non-cash
items and working-capital changes to recover operating cash flow;
it is the dominant presentation in practice because the inputs
(net income, balance-sheet changes) are already on the firm's books.
The two methods produce the same operating cash flow total; they
differ only in how the period's CFO is decomposed for the reader.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.119-174.

```
+--------------------------------------------+
| Cash Flow Statement                        |
+--------------------------------------------+
|  Cash Flow from Operating (CFO)            |
|    Indirect method:                        |
|       NI                                   |
|       + non-cash items (D&A, SBC, impair)  |
|       +- working-capital changes (ΔAR/Inv/AP)|
|       = CFO                                |
|    Direct method (alternative):            |
|       Cash from customers                  |
|       - Cash to suppliers                  |
|       - Cash to employees                  |
|       - Cash for interest, taxes           |
|       = CFO                                |
+--------------------------------------------+
|  Cash Flow from Investing (CFI)            |
|       - CapEx                              |
|       - Acquisitions                       |
|       + Asset sale proceeds                |
|       +- Investments in / divestments      |
+--------------------------------------------+
|  Cash Flow from Financing (CFF)            |
|       + Debt issuance                      |
|       - Debt repayment                     |
|       + Share issuance                     |
|       - Share repurchase                   |
|       - Dividends paid                     |
+--------------------------------------------+
|  Net Δ Cash = CFO + CFI + CFF              |
+--------------------------------------------+
```

The schematic above shows the canonical three-bucket structure with
both operating-section presentation methods. The articulation
identity ties the bottom line to the balance sheet: ending cash =
opening cash + net change in cash. **Source:** CFA L1 Curriculum
(2022) Vol.3/pp.119-174.

## Definition

The cash flow statement is the financial statement that reports the
firm's cash inflows and outflows over a reporting period,
partitioned into operating, investing, and financing activities.
Each section has specific classification criteria. **Source:** CFA
L1 Curriculum (2022) Vol.3/pp.119-174.

Operating activities are the cash effects of the firm's primary
revenue-producing activities and other activities not classified as
investing or financing. Operating cash flow includes cash collected
from customers, cash paid to suppliers and employees, and (under
US GAAP) cash for interest received, interest paid, dividends
received, and income taxes. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.119-174.

Investing activities are the cash effects of acquiring and
disposing of long-lived assets and investments not held for trading.
Investing cash flow includes cash spent on capital expenditure
(CapEx), cash spent on acquisitions, cash from selling property
and equipment, cash for purchases or sales of investments in other
firms (where not classified as cash equivalents). **Source:** CFA
L1 Curriculum (2022) Vol.3/pp.119-174.

Financing activities are the cash effects of obtaining and repaying
capital from debt holders and shareholders. Financing cash flow
includes cash from debt issuance, cash for debt repayment, cash
from share issuance, cash for share repurchase, and cash for
dividends paid. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.119-174.

The classification of interest, dividends, and taxes differs between
IFRS and US GAAP. Under US GAAP, interest paid and interest received
both go in CFO; dividends received go in CFO; dividends paid go in
CFF; income taxes paid go in CFO. Under IFRS, the firm has
classification choice for interest paid (CFO or CFF), interest
received (CFO or CFI), dividends received (CFO or CFI), and dividends
paid (CFO or CFF); the firm must apply its choice consistently
period to period. The flexibility under IFRS is a routine
reclassification adjustment in cross-framework peer comparison.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.119-174.

## Mathematical Reasoning

The aggregate cash-flow identity ties the three sections to the
period change in cash: `Net Δ Cash = CFO + CFI + CFF`. This is the
articulation identity that connects the cash flow statement to the
balance sheet's cash line: `Cash_t = Cash_{t-1} + Net Δ Cash`.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.119-174.

The indirect method's bridge from net income to CFO is the algebraic
expansion `CFO = NI + non-cash items - ΔWC` where `non-cash items`
collects depreciation, amortization, stock-based compensation,
impairment charges, and gain or loss on asset sales (subtracted out
when the asset-sale gain is reported in NI but the cash effect is
in CFI), and `ΔWC` is the period change in net non-cash working
capital. The detailed working-capital component breakdown is `ΔWC =
Δ AR + Δ Inv − Δ AP − Δ Accrued Liabilities` (using the convention
that an increase in AR or Inv is a use of cash and an increase in AP
or Accrued Liab is a source of cash). **Source:** CFA L1 Curriculum
(2022) Vol.3/pp.119-174.

The direct method's reconciliation to CFO is alternative algebra
that produces the same total. Cash from customers equals revenue
minus the period change in accounts receivable. Cash to suppliers
equals cost of goods sold plus the period change in inventory minus
the period change in accounts payable. Cash to employees equals
salary and wage expense minus the period change in accrued
compensation. The direct-method line items are derived from the
income-statement line items adjusted for the corresponding
balance-sheet working-capital changes. **Source:** CFA L1 Curriculum
(2022) Vol.3/pp.119-174.

The classification choices interact with the analyst's interpretation
of CFO. When IFRS-reporting firms classify interest paid in CFF
rather than CFO, their reported CFO is higher than the equivalent
US-GAAP-reporting firm; analysts comparing the two should reverse
the classification difference (subtract interest paid from CFO to
get a comparable measure). The same logic applies to dividends
received and to tax payments. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.119-174.

A diagnostic the analyst routinely uses: the relationship between
operating cash flow and net income across periods. When CFO grows
faster than NI, the firm's accruals are reversing into cash, which
typically reflects working-capital efficiency. When NI grows faster
than CFO over multiple periods, the firm's earnings are
increasingly accrual-driven, and the analyst should examine the
working-capital build for either growth-investment causes (large
inventory build for upcoming sales) or accrual-quality concerns
(ageing receivables, inventory obsolescence). **Source:** Penman
(2013) Ch.4 pp.110-139.

The classification of CapEx in CFI and the treatment of acquisitions
in CFI together drive a related diagnostic: free cash flow to firm
(FCFF) is computed as CFO + after-tax interest paid - CapEx, which
the analyst uses as a valuation input. The detailed FCFF / FCFE
construction is treated in existing 05 Equity cards; this card
provides the CFI/CFO classification foundation those cards consume.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.119-174.

## See Also

- [`fra-articulation-of-financial-statements`](./fra-articulation-of-financial-statements.md) — the cash roll-forward identity that ties CFS to the balance sheet
- [`fra-cash-vs-accrual-accounting`](./fra-cash-vs-accrual-accounting.md) — the conceptual basis for the indirect-method bridge
- [`fra-ifrs-vs-us-gaap-framework`](./fra-ifrs-vs-us-gaap-framework.md) — interest / dividends / tax classification choices differ between IFRS and US GAAP

## Escalate to Raw When

Open the CFA L1 curriculum Vol.3 R19 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.119-174.

- the firm presents the operating section using the direct method
  and the analyst needs the curriculum's per-line cash-derivation
  formula. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.119-174.
- the analyst is reconciling a cross-framework peer comparison where
  one firm classifies interest paid in CFO and the other in CFF —
  the curriculum gives the explicit treatment. **Source:** CFA L1
  Curriculum (2022) Vol.3/pp.119-174.
- the analyst is constructing a FCFF or FCFE estimate from the cash
  flow statement and needs the curriculum's CFO-to-FCFF bridge.
  **Source:** CFA L1 Curriculum (2022) Vol.3/pp.119-174.
