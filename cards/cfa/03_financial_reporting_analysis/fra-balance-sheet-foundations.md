---
schema_version: "cacg.v0"
id: "fra-balance-sheet-foundations"
title: "Balance Sheet Foundations"
reading_id: "03_financial_reporting_analysis"
summary: "Lays out the balance sheet's structure under the IFRS / US GAAP framework — how assets and liabilities classify, how the equity section organises itself, and what the analyst reads from the snapshot at a reporting date versus what is deliberately left to footnotes."
tags: ["financial-reporting", "balance-sheet"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1249:1825"
    chunk_hash: "5a34a6dd8d226b1b108c0b52c8a95281d90c77c5c9d1ad8306e4ebd62fccd280"
    page_range: [1249, 1250]
    quote: "The balance sheet provides information on a company’s resources (assets) and its sources of capital (equity and liabilities/debt)"
    edge_type: "defines"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p069:0109"
    chunk_hash: "cc08875592a254e5c1d76ec6fcea8792607c9446d13f636d752c6221d92bd827"
    page_range: [69, 70]
    quote: "The stock of equity value in the balance sheet increases from net income that is detailed in the income statement and from other comprehensive income"
    edge_type: "supports"
card_hash: "9785d10cd97bd2e813fe63c5bc54fe2e75a45d53640fa2aed26e12d351e19a47"
---
# Balance Sheet Foundations

## Intuition

The balance sheet is a snapshot. At the reporting date, it claims to
list everything the firm owns (assets), everything the firm owes
(liabilities), and the residual claim that belongs to shareholders
(equity). Two things follow immediately. First, the snapshot misses
the in-period flows entirely — for those the analyst reads the
income and cash flow statements. Second, the values on the balance
sheet are accounting measurements, not market values, and the
analyst must read carefully which line items are at historical cost,
which are at fair value, and which are at amortized cost.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.63-118.

The balance sheet's organization is itself a claim. By convention,
assets and liabilities are presented in order of liquidity — current
items (those expected to convert to cash or be settled within one
year or one operating cycle, whichever is longer) appear before
non-current items. The convention reflects the user's likely order
of interest: short-term solvency questions read off the top,
long-term capital-structure questions read off the bottom. **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.63-118.

```
+------------------------------+
| BALANCE SHEET (Snapshot)     |
+------------------------------+
| ASSETS                       |
|   Current assets             |
|     Cash and equivalents     |
|     Marketable securities    |
|     Accounts receivable      |
|     Inventory                |
|     Prepaid expenses         |
|   Non-current assets         |
|     Property, plant, equip.  |
|     Intangible assets        |
|     Goodwill                 |
|     Long-term investments    |
|   Total Assets               |
+------------------------------+
| LIABILITIES                  |
|   Current liabilities        |
|     Accounts payable         |
|     Short-term debt          |
|     Accrued liabilities      |
|     Deferred revenue (curr.) |
|   Non-current liabilities    |
|     Long-term debt           |
|     Deferred tax liabilities |
|     Pension obligations      |
|   Total Liabilities          |
+------------------------------+
| EQUITY                       |
|   Common stock + APIC        |
|   Retained earnings          |
|   AOCI                       |
|   Treasury stock (contra)    |
|   Total Equity               |
+------------------------------+
| Total Liab. + Equity         |
| = Total Assets (identity)    |
+------------------------------+
```

The schematic above is the canonical balance-sheet shape: liquidity-
ordered on both sides, with the accounting identity at the bottom
showing that total assets must equal total liabilities plus equity
at every reporting date. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.63-118.

## Definition

The balance sheet (sometimes called the statement of financial
position) is the financial statement that reports the firm's assets,
liabilities, and equity at a single point in time. Each section has
specific recognition and measurement criteria. **Source:** CFA L1
Curriculum (2022) Vol.3/pp.63-118.

An asset is a resource controlled by the firm as a result of past
events from which future economic benefits are expected to flow to
the firm. Recognition requires that the future benefits are probable
and that the cost or value can be reliably measured. **Source:** CFA
L1 Curriculum (2022) Vol.3/pp.63-118.

A liability is a present obligation of the firm arising from past
events whose settlement is expected to result in an outflow of
economic resources. Recognition requires the obligation to be
present (legally or constructively) and the outflow to be reliably
measurable. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.63-118.

Equity is the residual interest in the firm's assets after deducting
its liabilities. The equity section organizes itself around how the
residual claim was created — share issuance contributes to common
stock plus additional paid-in capital, retained period earnings
accumulate in retained earnings, and unrealized gains and losses on
specified items (currency translation, certain pension adjustments,
available-for-sale securities) accumulate in accumulated other
comprehensive income. Treasury stock — repurchased own shares —
sits as a contra-equity account that reduces total equity. **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.63-118.

## Mathematical Reasoning

The balance-sheet identity holds at every reporting date by
construction: `Total Assets = Total Liabilities + Total Equity`.
Sub-categorization within assets and within liabilities (current vs
non-current; operating vs financial) is a presentation choice, not
an identity-altering one — total assets remains the sum of all
assets regardless of internal breakdown. **Source:** CFA L1
Curriculum (2022) Vol.3/pp.63-118.

Measurement bases differ by line item and standard. Cash and
equivalents are at face value. Marketable securities classified as
trading or available-for-sale are at fair value (with the
unrealized gains and losses landing in net income or AOCI
respectively). Marketable securities classified as held-to-maturity
are at amortized cost. Accounts receivable are at face value less
an allowance for doubtful accounts. Inventory is at the lower of
cost (FIFO, LIFO under US GAAP, or weighted-average) and net
realizable value. Property, plant, and equipment is at historical
cost less accumulated depreciation under US GAAP, or optionally at
fair value under IFRS revaluation model. Intangibles are at cost
less amortization, with goodwill not amortized but tested for
impairment. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.63-118.

This measurement heterogeneity has a direct analyst implication:
total assets is a sum of numbers measured on different bases. A
balance sheet that prints `Total Assets = $1B` is not claiming that
$1B of fair-market-value resources is sitting on the firm; it is
claiming that the sum of historical-cost-less-depreciation,
fair-value, amortized-cost, and similar measurements totals $1B
under the firm's reporting framework. The analyst reads the
footnotes to learn which accounts are on which basis. **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.63-118.

The current-vs-non-current classification is a liquidity heuristic.
Current assets are those expected to be realized in cash, sold, or
consumed within one year or one operating cycle, whichever is
longer; everything else is non-current. Current liabilities are
those expected to be settled within the same horizon; everything
else is non-current. The classification supports several common
ratios — the current ratio (current assets / current liabilities)
and the quick ratio (current assets less inventory / current
liabilities) — that diagnose short-term solvency. **Source:** CFA
L1 Curriculum (2022) Vol.3/pp.63-118.

The analyst's read of the balance sheet integrates with articulation
identities. The ending-cash line on the balance sheet must equal
opening cash plus the period's net change in cash from the cash
flow statement. The ending equity line must reconcile to opening
equity plus net income from the income statement plus other
comprehensive income less dividends declared plus net share
issuance. Any deviation between published values and the
articulation identity signals either a reporting reclassification
(legitimate) or a reporting error (rare but real) that the
analyst should investigate via the notes. **Source:** Penman (2013)
Ch.2 pp.32-71.

## See Also

- [`fra-articulation-of-financial-statements`](./fra-articulation-of-financial-statements.md) — balance-sheet stocks must articulate with the period's flow statements
- [`fra-double-entry-mechanics`](./fra-double-entry-mechanics.md) — the double-entry rule that preserves the balance-sheet identity at every transaction

## Escalate to Raw When

Open the CFA L1 curriculum Vol.3 R18 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.63-118.

- the firm has unusual asset categories (biological assets under
  IFRS, regulatory assets in utilities, intangible assets from a
  recent acquisition) and the analyst needs the curriculum's
  recognition and measurement guidance per category. **Source:** CFA
  L1 Curriculum (2022) Vol.3/pp.63-118.
- the firm uses fair-value measurement extensively and the analyst
  needs the curriculum's level-1 / level-2 / level-3 fair-value
  hierarchy framing. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.63-118.
- the equity section is non-trivial (multiple share classes,
  treasury-stock movements, AOCI reclassifications) and the
  analyst needs the curriculum's component-level treatment.
  **Source:** CFA L1 Curriculum (2022) Vol.3/pp.63-118.
