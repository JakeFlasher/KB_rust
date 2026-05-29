---
schema_version: "cacg.v0"
id: "eq-pe-and-relative-valuation"
title: "P/E and Relative Valuation"
reading_id: "05_equity"
summary: "Framing relative valuation as a sibling of intrinsic valuation: standardize the target's price by an accounting metric (earnings, book value, sales), choose comparable firms, and compare the target's multiple to the peer distribution. P/E is the canonical equity multiple; the four-step relative-valuation process organizes the work."
tags: ["equity", "pe-relative"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p616:0805"
    chunk_hash: "ce37654c5b718de41e7582e19836e7d5ab2089a8426973a4c3f9c217ae8a8cc4"
    page_range: [616, 616]
    quote: "In relative valuation, the objective is to value assets based on how similar assets are currently priced in the market."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2301:3372"
    chunk_hash: "c05526a2cfb42b95a4be11d73b69a3c1b4b702b832ad71528035c6be66267b53"
    page_range: [2301, 2302]
    quote: "Multiplier models (synonym: market multiple models). These models are based chiefly on share price multiples or enterprise value multiples."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2233:3279"
    chunk_hash: "aaea54a47ddec32968a743d463549fafaa5c177fb38f28d63c6e8db5e3eeeca6"
    page_range: [2233, 2234]
    quote: "For Pfizer the price-to-book ratio is: Price-to-book ratio = Market price per share/Book value of equity per share"
    edge_type: "supports"
card_hash: "c359dcc0d35f532c5da9b0e008f1d024908fe1c8e5477ef92e68ebf7775e0c90"
---
# P/E and Relative Valuation

## Intuition

Relative valuation infers an asset's value by comparing standardized
prices of similar assets. Where intrinsic valuation builds value bottom
up from cash flows and a discount rate, relative valuation asks
"what do other comparable equity claims trade at, in price-per-unit-of-
fundamentals terms?" and applies that benchmark multiple to the
target's fundamentals. The two approaches are siblings, not
substitutes — each disciplines the other. **Source:** Damodaran (2012)
Ch.17 pp.616-634.

The price-earnings ratio (P/E) is the canonical equity multiple: the
share price divided by earnings per share. It says how many dollars
of price the market currently pays for one dollar of the firm's
earnings. Comparable-firm P/E ratios reveal whether the target's
current P/E is rich, in line, or cheap against the peer set, holding
fundamentals roughly constant. **Source:** Damodaran (2012) Ch.18
pp.635-689.

```
relative-valuation process

  step 1                step 2                  step 3
  +--------+            +-----------+           +------------+
  | choose | -- pick -> | choose    | -- pick ->| standardize|
  | metric |   relevant | comparables          |  metric    |
  +--------+   metric   +-----------+           +------------+
                                                       |
                                                       v
                                               +-------------+
                                               |  step 4     |
                                               |  compare    |
                                               |  target to  |
                                               |  comp peers |
                                               +-------------+
```

## Definition

The four-step relative-valuation process organizes the work: (1)
identify comparable assets that share the target's fundamentals
(growth, risk, payout); (2) standardize price by an accounting or
operating metric (earnings, book value, sales, EBITDA); (3) collect
the standardized multiples for the comparable set; (4) compare the
target's multiple to the comparable distribution and adjust for any
fundamental differences. **Source:** Damodaran (2012) Ch.17
pp.616-634.

The trailing P/E uses the most recent twelve-month earnings per share
in the denominator; the forward P/E uses next-period expected
earnings. Variants include the diluted P/E (denominator uses fully-
diluted share count) and normalized P/E (denominator uses cycle-
adjusted earnings to remove transitory swings). The choice of
variant depends on the use case: forward P/E for valuation
decisions, trailing P/E for historical-snapshot comparisons.
**Source:** Damodaran (2012) Ch.18 pp.635-689.

The justified P/E is the multiple consistent with a Gordon-growth
DDM. It collapses the equity-pricing problem into a relationship
between the target's fundamentals (payout ratio, growth rate, cost
of equity) and the multiple it should trade at — a bridge between
intrinsic and relative valuation. **Source:** Damodaran (2012) Ch.18
pp.635-689.

The CFA L1 frame presents relative valuation in parallel with
intrinsic valuation as the two operational valuation approaches at L1
depth, and introduces P/E, P/B, and P/Sales as the canonical
standardized multiples. **Source:** CFA L1 Curriculum (2022)
Vol.4/pp.361-416.

## Mathematical Reasoning

The trailing P/E ratio is `P_0 / E_TTM`, where `P_0` is the current
share price and `E_TTM` is trailing-twelve-month earnings per share.
The forward P/E is `P_0 / E_forward`, with the next-period expected
earnings in the denominator. **Source:** Damodaran (2012) Ch.18
pp.635-689.

The justified P/E follows from the Gordon DDM `V_0 = D_1 / (r - g)`
divided through by earnings. With payout ratio `p = D_1 / E_1`, the
justified forward P/E is `(P_0 / E_1)_justified = p / (r - g)`;
using current earnings instead gives the justified trailing P/E
`(P_0 / E_0)_justified = p · (1 + g) / (r - g)`. The trailing form
picks up the growth factor because `E_1 = E_0 · (1 + g)`. The
expression makes explicit that a higher payout ratio, lower cost of
equity, or higher sustainable growth rate all push the justified
multiple up. **Source:** Damodaran (2012) Ch.18 pp.635-689.

The justified-P/E formula breaks down for high-growth firms where
`g >= r` over a transitional horizon — the same convergence
constraint that bounds Gordon DDM (see
[`eq-dividend-discount-models`](./eq-dividend-discount-models.md)).
For such firms, a two-stage P/E framework discounts a high-growth-
period multiple plus a stable-state terminal multiple back to today.
**Source:** Damodaran (2012) Ch.18 pp.635-689.

The cross-sectional comparison standardizes the target's P/E against
the peer distribution. Three reference points are common: the peer
median, the peer mean, and a fundamentals-controlled benchmark from a
regression of P/E on growth / risk / payout (the regression-based
benchmark is covered at intuition depth in
[`eq-cross-sectional-multiples-distribution`](./eq-cross-sectional-multiples-distribution.md)).
The
target's classification (rich / in line / cheap) reflects the
distance between the target's P/E and the chosen reference, NOT a
worked numeric multiplication. **Source:** Damodaran (2012) Ch.18
pp.635-689.

The CFA L1 supporting reading reinforces P/E as the L1-canonical
equity multiple and presents the value-vs-price diagnostic flowing
from comparison: when target P/E exceeds peer-median P/E by a wide
margin without compensating growth, the target is candidate-
overvalued; the inverse points to candidate-undervaluation. **Source:**
CFA L1 Curriculum (2022) Vol.4/pp.361-416.

## See Also

- [`eq-intrinsic-value`](./eq-intrinsic-value.md) — the sibling intrinsic-valuation approach
- [`eq-dividend-discount-models`](./eq-dividend-discount-models.md) — Gordon DDM that yields the justified-P/E formula
- [`eq-pb-and-multiples-taxonomy`](./eq-pb-and-multiples-taxonomy.md) — P/B and other multiples in the taxonomy

## Escalate to Raw When

Open Damodaran Ch.17-18 directly when any of the criteria below
applies. **Source:** Damodaran (2012) Ch.17 pp.616-634.

- the target is a high-growth firm where Gordon-style justified-P/E breaks down — Damodaran Ch.18 develops the two-stage P/E framework. **Source:** Damodaran (2012) Ch.18 pp.635-689.
- a sector-specific multiple is more relevant than P/E (e.g., EV/EBITDA for capital-intensive firms, P/Sales for unprofitable growth firms) — see `eq-pb-and-multiples-taxonomy.md` and Damodaran Ch.20 (sector-specific multiples). **Source:** Damodaran (2012) Ch.18 pp.635-689.
- a regression-based fundamentals-to-multiples benchmark is required (cross-sectional inference at intuition depth) — see [`eq-cross-sectional-multiples-distribution`](./eq-cross-sectional-multiples-distribution.md) and Damodaran Ch.17 standardization framework. **Source:** Damodaran (2012) Ch.17 pp.616-634.
