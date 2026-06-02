---
schema_version: "cacg.v0"
id: "fra-depreciation-and-amortization"
title: "Depreciation and Amortization"
reading_id: "03_financial_reporting_analysis"
summary: "Lays out the depreciation and amortization accounting machinery under IFRS / US GAAP — straight-line vs accelerated vs units-of-production methods; how depreciation method, useful life, and residual value estimates feed into the income statement and balance sheet; how the choice propagates into ratios and analyst-side adjustments."
tags: ["financial-reporting", "depreciation-amortization"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1529:2214"
    chunk_hash: "8b377397e55b30ebad62ddb43ce60a626f6387ea24cb905c412a81c3c6205324"
    page_range: [1529, 1530]
    quote: "The choice of depreciation method affects the amounts reported on the financial statements, including the amounts for reported assets and operating and net income"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1530:2215"
    chunk_hash: "5e63e834a553a65f7ff02434039f609d008de70a209f93b144f69e549d4b4c42"
    page_range: [1530, 1531]
    quote: "Regardless of the depreciation method used, the carrying amount of the asset is not reduced below the estimated residual value"
    edge_type: "defines"
card_hash: "8cd1a512432702d3faa7d979c47540f21282160673324c8f753986cad59ee489"
---
# Depreciation and Amortization

## Intuition

A long-lived asset's cost benefits multiple periods. Depreciation
is the systematic period-by-period allocation of that cost to the
periods that benefit from the asset's use. Amortization is the
parallel concept for finite-life intangible assets. Both are
non-cash expenses — the cash outflow happened at acquisition
(capitalized at that time); the period charge is just the
accounting allocation of the prior outflow against current period
revenue. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.321-396.

The period-allocation choice is the depreciation method. All
methods conserve the total expense over the asset's life
(`Total Depreciation = Cost − Residual Value`), but they distribute
that total differently across periods. Straight-line distributes
equally; accelerated methods (declining balance, sum-of-the-years'
digits) front-load the expense; units-of-production ties the
expense to actual usage. The choice affects the period-by-period
profile of reported earnings but not the cumulative outcome.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.321-396.

```
+-----------------------------------------------+
| Depreciation Expense Profile by Method        |
+-----------------------------------------------+
| Year:    1    2    3    4    5    Total       |
|                                                |
| SL:      X    X    X    X    X    5X           |
| (equal each year)                              |
|                                                |
| DB:      Y1 > Y2 > Y3 > Y4 > Y5    5X          |
| (front-loaded, decreasing)                     |
|                                                |
| SYD:     Z1 > Z2 > Z3 > Z4 > Z5    5X          |
| (front-loaded, decreasing arithmetic)          |
|                                                |
| UoP:     U1   U2   U3   U4   U5   5X           |
| (varies with usage; total = sum)               |
|                                                |
| Conservation: Σ all-method-expense = Cost − R  |
+-----------------------------------------------+
```

The diagram shows the conservation property — total depreciation
expense over the asset's life equals (Cost − Residual Value)
regardless of method — and the front-loading property of accelerated
methods. The choice of method matters for how a single year's
reported earnings compare to peers; over the asset's full life the
methods are equivalent on cumulative expense. **Source:** CFA L1
Curriculum (2022) Vol.3/pp.321-396.

## Definition

Depreciation is the systematic allocation of the depreciable amount
of a tangible long-lived asset over its useful life. The depreciable
amount is the asset's cost less its expected residual value at the
end of its useful life. The useful life is the period over which
the asset is expected to be available for use. **Source:** CFA L1
Curriculum (2022) Vol.3/pp.321-396.

Amortization is the parallel allocation for intangible assets with
finite useful lives. The mechanics are the same as depreciation;
the convention uses different vocabulary by tradition (depreciate
PP&E, amortize intangibles). Intangible assets with indefinite
useful lives (including goodwill) are NOT amortized; they are
tested for impairment annually and written down when impaired.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.321-396.

The four canonical depreciation methods are described below.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.321-396.

- Straight-line method (SL) allocates equal expense each period:
  `Depreciation_year = (Cost − Residual) / Useful Life`. The most
  commonly used method for financial reporting because of its
  simplicity and the smooth period-by-period profile. **Source:**
  CFA L1 Curriculum (2022) Vol.3/pp.321-396.
- Declining-balance method (DB) applies a constant rate (often a
  multiple of the straight-line rate, hence "double-declining-
  balance" at twice the straight-line rate) to the asset's
  declining net book value each period. Front-loads the expense.
  No residual value is subtracted at each step; instead, the method
  switches to straight-line when straight-line on the remaining
  book value would be larger, ensuring the asset depreciates to
  residual value by the end of useful life. **Source:** CFA L1
  Curriculum (2022) Vol.3/pp.321-396.
- Sum-of-the-years' digits (SYD) allocates expense in arithmetic
  decreasing fractions of the depreciable amount: with useful life
  N, the year-1 fraction is N/(N(N+1)/2), the year-2 fraction is
  (N-1)/(N(N+1)/2), and so on down to 1/(N(N+1)/2) for year N.
  Front-loads the expense in a smooth arithmetic decline. **Source:**
  CFA L1 Curriculum (2022) Vol.3/pp.321-396.
- Units-of-production method (UoP) allocates expense in proportion
  to actual usage: `Depreciation_period = (Cost − Residual) ×
  (Units_used_period / Total_estimated_units)`. Useful when an
  asset's economic productivity varies meaningfully with usage
  (mining equipment, manufacturing tooling). **Source:** CFA L1
  Curriculum (2022) Vol.3/pp.321-396.

## Mathematical Reasoning

Each method conserves the total depreciation expense over the
asset's useful life: `Σ (Depreciation_year_i for i in 1..N) =
Cost − Residual`. The conservation is a definitional property of
the depreciable amount; methods differ in how the period-by-period
allocation distributes the total. **Source:** CFA L1 Curriculum
(2022) Vol.3/pp.321-396.

The straight-line method's period expense is constant: `D_SL =
(Cost − Residual) / N` for each of the N years. The straight-line
expense profile is flat. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.321-396.

Declining-balance applies a fixed depreciation rate `r` to the
beginning-period net book value `BV_t` so that the period expense
is `D_DB(t) = r × BV_t`. Because each period's beginning book
value is the prior period's ending book value (which already reflects
the prior period's expense), the resulting expense profile is
geometrically decreasing across periods. The cumulative depreciation
across the full useful life equals `Cost − Residual` when `r` is
set so the asset reaches its residual value by the end of useful
life; in practice the firm switches to straight-line on the
remaining book value at the period when straight-line on the
remaining balance would yield a larger expense than declining-
balance. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.321-396.

Sum-of-the-years' digits' year-t expense is `D_SYD(t) = (Cost −
Residual) × (N − t + 1) / (N(N+1)/2)`, a linear arithmetic decline.
The denominator `N(N+1)/2` is the sum of integers from 1 to N. The
total expense over N years equals `(Cost − Residual)` by
construction — the numerators sum to the denominator. **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.321-396.

Units-of-production method allocates expense in proportion to
period usage relative to total estimated lifetime units: `D_UoP(t) =
(Cost − Residual) × (Units_t / Total_Units)`. The total over the
asset's life equals `(Cost − Residual)` when actual cumulative usage
equals the estimated total. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.321-396.

The relative-profile comparison: under accelerated methods (DB,
SYD), early-life depreciation expense exceeds the straight-line
expense, making early-life reported earnings lower; late-life
depreciation expense falls below straight-line, making late-life
reported earnings higher. The cumulative effect is zero (both
methods total to Cost − Residual), but the per-period comparison
matters for ratios computed in any single year. **Source:** CFA L1
Curriculum (2022) Vol.3/pp.321-396.

The choice of useful life and residual value involves estimation
and is subject to revision under both frameworks when subsequent
information indicates the original estimates are incorrect. A
revision to useful life is a change in accounting estimate (not a
change in accounting policy), applied prospectively from the date
of revision; the unrecorded depreciable amount is allocated over
the revised remaining useful life. **Source:** Penman (2013) Ch.2
pp.32-71.

The depreciation choice interacts with valuation through reported
earnings persistence. A firm that uses accelerated depreciation
front-loads expense and back-loads earnings; the analyst comparing
its reported earnings to a straight-line-using peer must adjust
to compare apples-to-apples, particularly in early years of the
asset's life. The depreciation choice does NOT affect operating
cash flow (depreciation is non-cash), so cash-flow-based ratios
are insulated from this comparability issue. **Source:** CFA L1
Curriculum (2022) Vol.3/pp.321-396.

## See Also

- [`fra-long-lived-asset-capitalization`](./fra-long-lived-asset-capitalization.md) — capitalization is the prior step that creates the asset whose cost depreciation allocates over time
- [`fra-cash-vs-accrual-accounting`](./fra-cash-vs-accrual-accounting.md) — depreciation is a canonical non-cash item in the indirect-method CFO bridge
- [`fra-balance-sheet-foundations`](./fra-balance-sheet-foundations.md) — accumulated depreciation is the contra-asset that reduces gross PP&E to net book value

## Escalate to Raw When

Open the CFA L1 curriculum Vol.3 R22 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.321-396.

- the firm uses an accelerated method and the analyst needs to
  reconstruct the period-by-period expense profile (with a switch to
  straight-line when applicable) for cross-period comparison.
  **Source:** CFA L1 Curriculum (2022) Vol.3/pp.321-396.
- the firm has recently revised an asset's useful life or residual
  value and the analyst needs the prospective-application treatment
  for the recomputed depreciation. **Source:** CFA L1 Curriculum
  (2022) Vol.3/pp.321-396.
- the firm has long-lived intangibles with indefinite useful lives
  (goodwill, certain trademarks) and the analyst needs the no-
  amortization-but-impair treatment vs the finite-life amortization
  treatment. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.321-396.
