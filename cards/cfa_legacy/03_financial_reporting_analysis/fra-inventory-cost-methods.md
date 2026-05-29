---
schema_version: "cacg.v0"
id: "fra-inventory-cost-methods"
title: "Inventory Cost Methods"
reading_id: "03_financial_reporting_analysis"
summary: "Laying out FIFO / LIFO / weighted-average-cost methods under IFRS / US GAAP. LIFO is US-GAAP only; the rising-price ranking inverts COGS vs ending inventory under FIFO and LIFO. The LIFO-reserve disclosure enables FIFO-equivalent conversion for peer comparison."
tags: ["financial-reporting", "inventory-cost"]
citations:
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1440:2089"
    chunk_hash: "7d32b6021ea1ebb6110791d35084aa3524e31b6968f762a1f2f8e1e9b1d587b3"
    page_range: [1440, 1440]
    quote: "If there was no inflation or deflation with respect to inventory costs and thus unit costs were unchanged, the choice of inventory valuation method would be irrelevant."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1446:2098"
    chunk_hash: "2e0efc0333ed16e93792e8b5d065dafac3f145d74ba8ace47aaed2ce02872086"
    page_range: [1446, 1446]
    quote: "Because the cost of a kg of soap declined over the period, LIFO had the highest ending inventory amount, the lowest cost of sales, and the highest gross profit."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1109:1586"
    chunk_hash: "b9faff538e72c4619a273335ca87c82112809e1b40c966b476bcaa4e445434ff"
    page_range: [1109, 1110]
    quote: "Based on Rules Principles Inventory valuation FIFO, LIFO and Weighted Average Method."
    edge_type: "supports"
card_hash: "0502bd997e978b904f6c502f47ad74756b3f2b0a7712a048e8b2ff4e08eec005"
---
# Inventory Cost Methods

## Intuition

Inventory is the cost of goods the firm holds for sale. When the
firm sells some of its inventory, the related cost moves from the
balance sheet's inventory line to the income statement's cost of
goods sold (COGS). Because the firm typically buys inventory
multiple times at different prices, an accounting choice is needed
to specify which units' costs flow into COGS and which units' costs
remain in ending inventory. The inventory cost method is that
choice. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.253-320.

The three primary methods route purchase costs into COGS and ending
inventory differently. First-in, first-out (FIFO) treats the oldest
inventory as sold first, leaving the most-recently-purchased units
in ending inventory. Last-in, first-out (LIFO, US GAAP only) treats
the newest inventory as sold first, leaving the oldest units in
ending inventory. Weighted-average cost (WAC) computes a single
period-average cost per unit and applies that average to both COGS
and ending inventory. Specific identification tracks each unit's
actual cost individually and is the natural choice for high-value
unique items (jewelry, vehicles). **Source:** CFA L1 Curriculum
(2022) Vol.3/pp.253-320.

```
<!-- primitive: inventory-cost-flow source: _diagram_primitives.md -->
                  Goods Available for Sale
                  (Beginning Inv + Purchases)
                              |
        +---------------------+---------------------+
        |                     |                     |
        v                     v                     v
   +---------+           +---------+           +---------+
   |  FIFO   |           |  LIFO   |           |  WAC    |
   +---------+           +---------+           +---------+
   First-in =            Last-in =             All units
   first-out             first-out             at avg cost
        |                     |                     |
        v                     v                     v
   COGS = oldest         COGS = newest         COGS = avg
   Inv_end = newest      Inv_end = oldest      Inv_end = avg

  Identity (all 3): COGS_period + Inv_end = COGS_available
  Rising-price ranking: LIFO COGS > WAC COGS > FIFO COGS
                        FIFO Inv_end > WAC Inv_end > LIFO Inv_end
```

The diagram captures the conservation identity (COGS available =
COGS for the period + ending inventory; this holds for all three
methods because it is just an accounting restatement of "what we had
went somewhere") and the rising-price ranking. The ranking inverts
under falling prices. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.253-320.

## Definition

Inventory cost methods are the recognized accounting conventions
for assigning historical cost between the period's COGS expense and
the ending balance-sheet inventory asset, given that the firm
purchases inventory at different prices over time. **Source:** CFA
L1 Curriculum (2022) Vol.3/pp.253-320.

- FIFO assumes the oldest units are sold first. Under rising
  prices, FIFO produces the lowest COGS, the highest reported gross
  margin, the highest ending inventory balance, and the highest
  reported income tax (because taxable income is higher). Under
  falling prices, FIFO produces the highest COGS and lowest ending
  inventory. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.253-320.
- LIFO assumes the newest units are sold first. Under rising
  prices, LIFO produces the highest COGS, the lowest reported gross
  margin, the lowest ending inventory balance, and the lowest
  reported income tax. The accumulated difference between FIFO and
  LIFO ending inventory is called the LIFO reserve, which firms
  using LIFO disclose so analysts can convert to FIFO-equivalent
  inventory. LIFO is permitted under US GAAP but prohibited under
  IFRS. **Source:** CFA L1 Curriculum (2022) Vol.3/pp.253-320.
- Weighted-average cost (WAC) assigns each unit the period-average
  cost and routes both COGS and ending inventory at that average.
  WAC produces results between FIFO and LIFO under both rising and
  falling prices. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.253-320.

The lower-of-cost-and-net-realizable-value rule (IFRS) or
lower-of-cost-or-market rule (US GAAP) requires inventory to be
written down when its market value falls below its carrying cost.
Under IFRS, write-downs can be reversed (with the reversal limited
to the original write-down amount); under US GAAP, write-downs are
generally not reversible. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.253-320.

## Mathematical Reasoning

The conservation identity for any inventory cost method is
`COGS_available = COGS_period + Inv_end` where `COGS_available =
Inv_beg + Purchases`. The identity holds because every unit of
inventory the firm started with or purchased during the period
either (a) was sold and its cost flowed to COGS, or (b) remains on
hand and its cost remains in ending inventory. The conservation is
a definitional restatement, not an empirical regularity. **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.253-320.

Under the assumption of rising prices for the period (purchase price
strictly increasing over the period), the methods produce the
inequality chain shown below. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.253-320.

```
Rising prices:
   COGS:        FIFO < WAC < LIFO
   Inv_end:     LIFO < WAC < FIFO
   Gross profit: LIFO < WAC < FIFO
   Income tax:  LIFO < WAC < FIFO
```

The inequalities are direct consequences of the cost-flow
assumption: FIFO routes the cheapest (oldest) costs into COGS,
leaving the most expensive (newest) costs in ending inventory; LIFO
does the opposite. WAC sits in the middle by construction.
**Source:** CFA L1 Curriculum (2022) Vol.3/pp.253-320.

Under falling prices, the inequalities reverse. Under flat prices
(constant purchase price), all three methods produce the same COGS
and the same ending inventory because there is no price-vintage
distinction to exploit. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.253-320.

The LIFO reserve disclosure (`LIFO Reserve = FIFO Inv − LIFO Inv`)
lets analysts convert a LIFO-reporting firm to a FIFO-equivalent
basis for peer comparison. The conversion adjustments are: (a)
add the period change in LIFO reserve to LIFO COGS to get FIFO
COGS; (b) add the LIFO reserve to LIFO ending inventory to get
FIFO ending inventory; (c) adjust pretax income upward by the
period change in LIFO reserve; (d) adjust the tax liability for
the higher pretax income at the firm's effective tax rate; (e)
adjust retained earnings for the cumulative after-tax difference.
The conversion is a routine cross-firm comparability exercise when
a US-LIFO firm is compared to IFRS or US-FIFO peers. **Source:**
CFA L1 Curriculum (2022) Vol.3/pp.253-320.

LIFO liquidations occur when a LIFO-reporting firm sells more units
than it purchases in a period, which forces the firm to release old
LIFO-layer costs into COGS. Under rising prices, the released old
costs are lower than current replacement costs, so the firm's
reported gross margin temporarily inflates and reported tax expense
correspondingly rises. The analyst reads LIFO liquidations as
non-recurring profit; sustainable margin should be estimated on a
no-liquidation basis. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.253-320.

## See Also

- [`fra-balance-sheet-foundations`](./fra-balance-sheet-foundations.md) — inventory is one of the measurement-base-heterogeneous current-asset categories
- [`fra-income-statement-foundations`](./fra-income-statement-foundations.md) — COGS reduces gross profit; the inventory cost method drives COGS
- [`fra-ifrs-vs-us-gaap-framework`](./fra-ifrs-vs-us-gaap-framework.md) — LIFO is the canonical US-GAAP-vs-IFRS measurement-rule difference

## Escalate to Raw When

Open the CFA L1 curriculum Vol.3 R21 directly when any of the
criteria below applies. **Source:** CFA L1 Curriculum (2022)
Vol.3/pp.253-320.

- the analyst is converting a US-LIFO firm to a FIFO-equivalent
  basis for peer comparison and needs the curriculum's per-step
  conversion formula. **Source:** CFA L1 Curriculum (2022)
  Vol.3/pp.253-320.
- the firm has reported a LIFO liquidation in the period and the
  analyst needs the curriculum's framework for separating
  liquidation-driven margin inflation from sustainable margin.
  **Source:** CFA L1 Curriculum (2022) Vol.3/pp.253-320.
- the firm has applied an inventory write-down (lower of cost and
  NRV / lower of cost or market) and the analyst needs the
  curriculum's treatment of the write-down's income-statement and
  balance-sheet effects under IFRS vs US GAAP. **Source:** CFA L1
  Curriculum (2022) Vol.3/pp.253-320.
