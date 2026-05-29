---
schema_version: "cacg.v0"
id: "fra-long-lived-asset-capitalization"
title: "Long-Lived Asset Capitalization"
reading_id: "03_financial_reporting_analysis"
summary: "Framing the capitalize-vs-expense decision for long-lived assets: capitalizable costs benefit multiple periods (PP&E acquisition, qualifying intangibles, dismantling estimates) vs immediate expense (training, opening costs, maintenance). IFRS permits revaluation and impairment reversal; US GAAP uses historical cost and irreversible impairment."
tags: ["financial-reporting", "long-lived"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1507:2180"
    chunk_hash: "598d1f32d2bace59454352e1ab54a2725a322526c0dee30800fa5a343f3fe67f"
    page_range: [1507, 1508]
    quote: "The scope of this reading is limited to long-lived tangible and intangible assets (hereafter, referred to for simplicity as long-lived assets)."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1577:2280"
    chunk_hash: "da2442a66a381e2d57edff9ab7e2cb373066bf67780dbfba2114a28a94eaac67"
    page_range: [1577, 1578]
    quote: "Only costs necessary for the machine to be ready to use can be capitalized."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1108:1585"
    chunk_hash: "a0d7c48d7912fd7856b93dd0144fa48dab97552df213064fa5cf5c6d8e6b6d65"
    page_range: [1108, 1109]
    quote: "Arguably, the most critical are the differences that exist between IFRS and US GAAP."
    edge_type: "supports"
card_hash: "a2001ef0f689e215526b2ec27c4a4eaf67789fe6b2b19119c62edbb162be1beb"
---
# Long-Lived Asset Capitalization

## Intuition

Some costs the firm incurs benefit a single period; others benefit
multiple periods. The accounting treatment differs by intent: costs
that benefit a single period are expensed immediately (matching the
expense to the period that received the benefit); costs that
benefit multiple periods are capitalized as an asset and expensed
over the periods they benefit (depreciation for tangible assets,
amortization for intangible assets). The capitalize-vs-expense
choice is therefore the period-allocation choice for the firm's
spending. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.321-396.

The capitalize-vs-expense decision has direct effects on reported
profitability and on the balance sheet. Capitalizing a cost shifts
it off the period's income statement and onto the balance sheet,
boosting reported income in the current period and increasing
reported assets; subsequent periods absorb the cost via depreciation
or amortization. Expensing recognizes the cost immediately and
leaves no asset on the balance sheet. The choice is constrained by
recognition criteria (probable future economic benefit; reliable
measurement of cost) and by the cost's nature (asset acquisition
vs maintenance; development vs research). **Source:** CFA L1
Curriculum (2022) Vol.3/pp.321-396.

```
+--------------------------------------------+
| Capitalize vs Expense Decision             |
+--------------------------------------------+
|  Cost incurred                             |
|       |                                    |
|       v                                    |
|  Future economic benefits probable?        |
|       |                                    |
|     +-+----------+                         |
|     |            |                         |
|    YES          NO                         |
|     |            |                         |
|     v            v                         |
|  Single        Expense                     |
|  period?       immediately                 |
|     |          (current-period             |
|   +-+-+         income statement)          |
|   |   |                                    |
|  YES  NO                                   |
|   |   |                                    |
|   |   +---> Capitalize as long-lived asset |
|   |         (balance sheet) +              |
|   |         depreciate / amortize over     |
|   |         useful life                    |
|   |                                        |
|   v                                        |
| Expense in the single-period it benefits   |
+--------------------------------------------+
```

The decision tree above maps the capitalize-vs-expense logic. The
intermediate node ("future economic benefits probable") is the
recognition criterion shared by IFRS and US GAAP; the period-
allocation node ("single period?") is the matching-principle
implementation. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.321-396.

## Definition

A long-lived asset is an asset expected to provide economic benefits
to the firm over more than one reporting period. Long-lived assets
include tangible assets (property, plant, and equipment — PP&E),
intangible assets (patents, copyrights, trademarks, customer lists,
in-process R&D, computer software, goodwill from acquisitions),
and right-of-use assets from leases. Each category has specific
recognition, measurement, and amortization / depreciation rules.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.321-396.

The capitalization criteria for tangible assets require that the
cost be directly attributable to bringing the asset to the location
and condition necessary for its intended use. Capitalizable costs
include the purchase price, import duties, non-refundable taxes,
direct labor for installation, professional fees for construction
oversight, and the estimated cost of dismantling and removing the
asset at the end of its useful life. Costs that are NOT capitalizable
include staff training, opening expenses for a new facility,
operating losses incurred before reaching planned utilization, and
ongoing maintenance. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.321-396.

The capitalization criteria for intangible assets differ by source.
Intangibles purchased separately (e.g., a patent bought from another
firm) are capitalized at cost. Intangibles acquired in a business
combination are recognized at fair value as part of the acquisition
accounting. Internally generated intangibles face stricter rules:
under IFRS, research costs are expensed and development costs are
capitalized only when the firm can demonstrate technical
feasibility, intent and ability to complete, future economic
benefits, and reliable cost measurement. Under US GAAP, internally
generated intangibles (other than certain software costs) are
generally expensed. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.321-396.

After initial recognition, long-lived assets are reported under the
cost model (historical cost less accumulated depreciation /
amortization less impairment) under both IFRS and US GAAP. IFRS
additionally permits a revaluation model, in which PP&E is
periodically revalued to fair value with the surplus going to OCI
(or to net income if reversing a prior impairment). US GAAP
prohibits upward revaluation. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.321-396.

## Mathematical Reasoning

The balance-sheet effect of capitalization is straightforward: a
capitalized cost appears as an asset on the balance sheet at its
acquisition cost, then declines via depreciation or amortization
over the useful life until either the asset is fully depreciated /
amortized, sold, or written down for impairment. The income-
statement effect is symmetric: rather than the full cost hitting
the period of acquisition, only the period's depreciation /
amortization charge hits each period. **Source:** CFA L1 Curriculum
(2022) Vol.3/pp.321-396.

For an asset with cost `C`, useful life `N` years, and residual /
salvage value `R`, the straight-line depreciation expense per year
is `(C - R) / N`. Each year's accumulated depreciation grows by
this amount; the asset's net book value declines from `C` toward
`R`. Alternative methods (declining balance, units of production,
sum-of-the-years' digits) produce different period-by-period
expense patterns but the same total depreciation `C - R` over the
full useful life. The detailed depreciation-method comparison is
treated in the next FRA card. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.321-396.

For intangible assets with finite useful lives, amortization
follows the same logic as depreciation. Intangibles with indefinite
useful lives (e.g., trademarks expected to remain marketable
indefinitely) are not amortized; they are tested for impairment
annually. Goodwill is the canonical indefinite-life intangible:
goodwill is not amortized under either IFRS or US GAAP and is
tested for impairment annually. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.321-396.

The impairment framework triggers a re-examination of the
capitalized asset value when indicators suggest the asset may have
lost economic value. Under IFRS (IAS 36), an impairment loss is
recognized when the asset's carrying amount exceeds its recoverable
amount (the higher of fair value less costs to sell and value in
use, where value in use is the present value of expected future
cash flows from the asset). Under US GAAP (ASC 360), the impairment
test is two-step: first, undiscounted future cash flows are compared
to carrying amount (no impairment if undiscounted CF ≥ carrying);
second, if the first test fails, the impairment loss equals the
shortfall to fair value. The two-step US GAAP test is intentionally
more permissive than the IFRS one-step test, so under similar
conditions IFRS firms recognize impairment more readily. **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.321-396.

The reversibility of impairment losses also differs. Under IFRS,
impairment losses (other than for goodwill) can be reversed up to
the original carrying amount when the conditions causing the loss
have changed. Under US GAAP, impairment losses are generally
irreversible. The asymmetry contributes to cross-framework
comparability differences when asset values recover. **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.321-396.

## See Also

- [`fra-balance-sheet-foundations`](./fra-balance-sheet-foundations.md) — long-lived assets are a major non-current-asset category with measurement-basis heterogeneity
- [`fra-cash-vs-accrual-accounting`](./fra-cash-vs-accrual-accounting.md) — depreciation and amortization are canonical non-cash adjustments in the indirect-method bridge
- [`fra-ifrs-vs-us-gaap-framework`](./fra-ifrs-vs-us-gaap-framework.md) — capitalization, revaluation, and impairment-reversal rules differ between the frameworks

## Escalate to Raw When

Open the CFA L1 curriculum Vol.3 R22 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.321-396.

- the firm has recently acquired a major long-lived asset and the
  analyst needs the curriculum's treatment of which costs are
  capitalizable and which are expensed. **Source:** CFA L1
  Curriculum (2022) Vol.3/pp.321-396.
- the firm has reported an impairment loss and the analyst needs
  the framework-specific (IFRS one-step / US GAAP two-step) test
  detail to verify the loss magnitude or to anticipate potential
  reversal under IFRS. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.321-396.
- the firm is IFRS-reporting and applies the revaluation model to
  PP&E or to intangibles; the analyst needs the curriculum's
  treatment of revaluation surplus presentation in equity vs
  reversals through net income. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.321-396.
