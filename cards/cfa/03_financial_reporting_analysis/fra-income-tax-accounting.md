---
schema_version: "cacg.v0"
id: "fra-income-tax-accounting"
title: "Income Tax Accounting"
reading_id: "03_financial_reporting_analysis"
summary: "Framing book income (IFRS/US GAAP) vs taxable income (tax law) and the temporary vs permanent differences that drive a wedge between them. Deferred tax assets and liabilities record the future tax consequences of temporary differences; the effective-tax-rate reconciliation footnote surfaces persistent drivers."
tags: ["financial-reporting", "income-tax"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1584:2291"
    chunk_hash: "5a43cca07c48aedb6d70dac52dbb547ba4fb65de2a5e2cb71195d4a745257b21"
    page_range: [1584, 1585]
    quote: "Because of different guidelines for how income is reported on a company’s financial statements and how it is measured for income tax purposes, accounting profit and taxable income may differ."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1583:2290"
    chunk_hash: "1c268424d1f45acc2058d913392b6d0706e1486109eeee5cc65497c1971471cf"
    page_range: [1583, 1584]
    quote: "Deferred tax assets or liabilities usually arise when accounting standards and tax authorities recognize the timing of revenues and expenses at different times."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1108:1585"
    chunk_hash: "a0d7c48d7912fd7856b93dd0144fa48dab97552df213064fa5cf5c6d8e6b6d65"
    page_range: [1108, 1109]
    quote: "Arguably, the most critical are the differences that exist between IFRS and US GAAP."
    edge_type: "supports"
card_hash: "5cbb2c640ea0f5e7d8d4b1687ba05f4e925a32b504ae955dd46cf54cc88ec402"
---
# Income Tax Accounting

## Intuition

Two different rule systems compute the firm's profit from the same
underlying economic activity. The accounting framework (IFRS or US
GAAP) computes accounting profit using the matching principle and
specified recognition criteria. The tax-law framework computes
taxable profit using the rules of the relevant tax jurisdiction.
The two systems agree on most items (revenue, most operating
expenses) but disagree on others (depreciation method choice,
provisions and reserves, certain revenue-recognition items,
research-and-development capitalization). The disagreements create
differences between book and tax results. **Source:** CFA L1
Curriculum (2022) Vol.3/pp.397-434.

These differences split into two families. Temporary differences
reverse over time — accounting and tax recognize the same total
amount over the asset or liability's life, but they recognize it
in different periods. A firm that uses accelerated depreciation
for tax (faster) and straight-line for books (slower) records
higher tax depreciation in early years and lower in late years; the
total depreciation is the same. Permanent differences do not
reverse — items that are recognized in books but never in tax (or
vice versa), such as municipal-bond interest income (tax-free in
many jurisdictions) or certain non-deductible fines. Permanent
differences create a permanent gap between book and tax results.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.397-434.

```
+--------------------------------------------+
| Book vs Tax Profit Reconciliation          |
+--------------------------------------------+
|  Accounting Profit (Pre-tax NI)            |
|       |                                    |
|       +- Temporary differences (reversing) |
|       |    e.g., accelerated tax dep.      |
|       |          deferred-revenue timing   |
|       |          warranty accruals         |
|       |                                    |
|       +- Permanent differences (no reverse)|
|       |    e.g., tax-free interest         |
|       |          non-deductible fines      |
|       v                                    |
|  Taxable Profit                            |
|       × Statutory Tax Rate                 |
|       = Current Tax Expense                |
+--------------------------------------------+
|  Income Tax Expense (book) =               |
|     Current Tax Expense                    |
|     + Δ Deferred Tax Liabilities           |
|     − Δ Deferred Tax Assets                |
+--------------------------------------------+
```

The schematic shows the two-track reconciliation: the upper block
adjusts accounting profit to taxable profit using the temporary
and permanent differences; the lower block reconciles cash tax
(current tax expense, paid this year) to book tax (income tax
expense, recognized in the income statement). The deferred-tax
movement is the bridge between the two — it captures the future
tax consequences of the temporary differences. **Source:** CFA L1
Curriculum (2022) Vol.3/pp.397-434.

## Definition

Accounting profit (also called pre-tax income, book income) is the
profit before income tax expense as measured under the firm's
financial-reporting framework (IFRS or US GAAP). Taxable profit
(also called taxable income) is the profit on which income tax is
computed under the relevant tax-law framework. The two are
generally not equal. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.397-434.

A temporary difference arises when an item of income or expense is
recognized in different periods under accounting and tax rules but
the cumulative amount over time is the same. **Source:** CFA L1
Curriculum (2022) Vol.3/pp.397-434.

- Accelerated depreciation for tax (faster method, often
  double-declining-balance with tax-favored short useful life) vs
  straight-line for books. Early years: book expense < tax expense;
  later years: book expense > tax expense; cumulative equal.
  **Source:** CFA L1 Curriculum (2022) Vol.3/pp.397-434.
- Deferred revenue: cash collected before earning is taxable in
  the cash period under tax law but recognized as accounting revenue
  later. Period of cash collection: tax recognizes; book delays;
  period of earning: book recognizes; tax already did; cumulative
  equal. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.397-434.
- Warranty provisions: book accrues warranty cost when sale occurs
  (matching); tax recognizes only when warranty cost is actually
  paid out. Book recognizes earlier; tax later; cumulative equal.
  **Source:** CFA L1 Curriculum (2022) Vol.3/pp.397-434.

A permanent difference arises when an item is recognized in one
system and never in the other. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.397-434.

- Tax-free municipal-bond interest income: included in book income;
  excluded from taxable income. Lowers taxable profit relative to
  accounting profit permanently. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.397-434.
- Non-deductible fines and penalties: included in book expense;
  excluded from tax-deductible expense. Raises taxable profit
  relative to accounting profit permanently. **Source:** CFA L1
  Curriculum (2022) Vol.3/pp.397-434.
- Dividends-received deduction (US): a portion of dividends from
  certain investee firms is excluded from taxable income; included
  in book income. Lowers taxable profit relative to accounting
  profit permanently. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.397-434.

A deferred tax liability (DTL) is recorded when temporary
differences cause taxable profit to be lower than accounting
profit in the current period (the firm "owes" tax in a future
period when the difference reverses). A deferred tax asset (DTA)
is recorded when temporary differences cause taxable profit to be
higher than accounting profit in the current period (the firm
"prepaid" tax that will reverse to a future deduction). The DTL
and DTA balances are measured at the tax rate expected to apply
when the difference reverses. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.397-434.

## Mathematical Reasoning

The income-statement income tax expense is the sum of two
components: current tax expense (the tax actually owed for the
current period under tax law) plus the period change in deferred
tax balances (the net change in DTL minus the net change in DTA).
Algebraically: `Income Tax Expense = Current Tax Expense + ΔDTL −
ΔDTA`. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.397-434.

The current tax expense is computed by adjusting accounting profit
for temporary and permanent differences to derive taxable profit,
then applying the statutory tax rate: `Current Tax Expense =
Taxable Profit × Statutory Tax Rate`. For the permanent
differences, the adjustment is one-way and does not affect
deferred-tax balances. For the temporary differences, the
adjustment changes the period's current tax but creates an
offsetting change in DTL or DTA so that the income-statement
income tax expense reflects the accounting-profit-based tax rather
than the taxable-profit-based tax. **Source:** CFA L1 Curriculum
(2022) Vol.3/pp.397-434.

The effective tax rate (ETR) is the ratio of income-statement
income tax expense to accounting profit: `ETR = Income Tax Expense
/ Accounting Profit`. The ETR differs from the statutory tax rate
by the contribution of permanent differences and certain other
items (foreign-tax-rate differentials, one-time tax credits, tax
contingencies). The firm reconciles ETR to statutory rate in a
required tax-rate reconciliation footnote, which the analyst reads
to identify the drivers of any persistent gap. **Source:** CFA L1
Curriculum (2022) Vol.3/pp.397-434.

The valuation-allowance mechanism limits DTA recognition. Under
both IFRS and US GAAP, a DTA is recognized only when it is
probable (IFRS) or more-likely-than-not (US GAAP) that the firm
will have sufficient future taxable income to use the deduction.
When the firm cannot meet this test, it records a valuation
allowance that reduces the DTA. The allowance is reversed in
later periods if the firm's prospects improve. The analyst reads
a large or growing valuation allowance as a signal of doubt about
future profitability. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.397-434.

A change in the statutory tax rate triggers a re-measurement of
existing DTL and DTA balances at the new rate. The re-measurement
hits the income-statement income tax expense in the period of the
rate-change enactment. A large enacted statutory-rate reduction
produces a one-period spike (or negative spike, for net-DTA firms)
in reported income tax expense that the analyst should treat as
non-recurring when projecting sustainable earnings. **Source:** CFA
L1 Curriculum (2022) Vol.3/pp.397-434.

## See Also

- [`fra-income-statement-foundations`](./fra-income-statement-foundations.md) — income tax expense is the line item directly governed by this card's mechanics
- [`fra-balance-sheet-foundations`](./fra-balance-sheet-foundations.md) — DTA / DTL balances appear as non-current asset / liability lines
- [`fra-depreciation-and-amortization`](./fra-depreciation-and-amortization.md) — book-vs-tax depreciation method choice is the canonical temporary-difference example

## Escalate to Raw When

Open the CFA L1 curriculum Vol.3 R23 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.397-434.

- the firm has reported a large valuation allowance change and the
  analyst needs the curriculum's framework for interpreting the
  allowance's signal about future profitability. **Source:** CFA L1
  Curriculum (2022) Vol.3/pp.397-434.
- the analyst is reconciling effective tax rate to statutory rate
  for a multi-jurisdiction firm and needs the curriculum's per-line
  treatment of the rate-reconciliation footnote categories.
  **Source:** CFA L1 Curriculum (2022) Vol.3/pp.397-434.
- a tax-law change is enacted (statutory rate change, deduction
  rule change) and the analyst needs the curriculum's treatment of
  DTA / DTL re-measurement and its income-statement effect.
  **Source:** CFA L1 Curriculum (2022) Vol.3/pp.397-434.
