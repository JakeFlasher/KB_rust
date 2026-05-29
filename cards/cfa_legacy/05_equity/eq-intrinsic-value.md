---
schema_version: "cacg.v0"
id: "eq-intrinsic-value"
title: "Intrinsic Value"
reading_id: "05_equity"
summary: "Framing intrinsic value as the value an asset would command from a fully informed investor analyzing its expected cash flows and risk — the benchmark against which market price is judged rich, in line, or cheap. Damodaran's DCF foundation and the CFA L1 over/under/fairly-valued classification both estimate intrinsic value and compare to observed price."
tags: ["equity", "intrinsic-value"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p059:0047"
    chunk_hash: "0120f2cbc2fe755de3586196ecdd0e32d0306f76706999683b093465610df8bd"
    page_range: [59, 60]
    quote: "In discounted cash flow valuation, we try to estimate the intrinsic value of an asset based on its fundamentals."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2298:3368"
    chunk_hash: "4fe667b7c5b2fd551c40670359b1297fbc795f1f8d1cf66e261b3a7c59306628"
    page_range: [2298, 2299]
    quote: "This reading introduces equity valuation models used to estimate the intrinsic value (synonym: fundamental value) of a security"
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2234:3280"
    chunk_hash: "9d2bb931d13e219d18b5ac19401e72160c1482cd19cf1255e1becbed27f9f7d9"
    page_range: [2234, 2234]
    quote: "A company’s intrinsic value can only be estimated because it is impossible to predict the amount and timing of its future cash flows."
    edge_type: "supports"
card_hash: "f9145ddccb9664056e4b49a4d110cd3a41baa7e93433a0e2d3d20b2d17fa7e9d"
---
# Intrinsic Value

## Intuition

Intrinsic value is the value an asset would command from a fully
informed investor analyzing the asset's expected cash flows and the
risk attached to those cash flows. It is the value the asset SHOULD
trade at if every market participant agreed on the same fundamentals
and discount rate. **Source:** Damodaran (2012) Ch.1 pp.41-57.

The market price is what an asset DOES trade at — the outcome of
supply, demand, mood, narrative, and the shifting beliefs of every
participant. The gap between price and intrinsic value is the surface
that valuation work targets: a disciplined estimate of intrinsic value
gives the investor a benchmark against which the price can be judged
rich, in line, or cheap. **Source:** Damodaran (2012) Ch.1 pp.41-57.

```
<!-- primitive: valuation-waterfall source: _diagram_primitives.md -->
value per share
   ^
   |   observed market price
   |        |
   |        v
   |   compare after fundamental build
   |
   |   +----------+
   |   | + ops    |
   |   |  value   |
   |   +----------+
   |        |
   |        v   add non-operating assets (cash, stakes)
   |   +----------+
   |   | + non-op |
   |   |  assets  |
   |   +----------+
   |        |
   |        v   subtract debt + other claims
   |   +----------+
   |   | - debt + |
   |   |  claims  |
   |   +----------+
   |        |
   |        v   divide by diluted share count
   |   +----------+
   |   |intrinsic |
   |   |  value   |     <-- target
   |   +----------+
   |        |
   |        v
   |   +----------+
   |   | price -  |
   |   | value gap|
   |   +----------+
   |
   +-> bridge sign per step is conceptual, not numeric
```

## Definition

Three valuation approaches recover intrinsic value from different
inputs. Intrinsic (discounted-cash-flow) valuation derives value from
the asset's expected cash flows discounted at a rate reflecting the
risk of those cash flows. Relative (multiples) valuation infers value
by comparing standardized prices of comparable assets. Contingent-
claim valuation uses option-pricing models for assets whose cash flows
are conditional on uncertain future states. **Source:** Damodaran
(2012) Ch.2 pp.58-79.

For a firm whose cash flows accrue to equity holders, intrinsic equity
value per share is built from operating-asset value, augmented by
non-operating assets (cash, minority stakes), reduced by debt and
other claims senior to common equity, and divided by the diluted
share count. The waterfall above shows the sign of each step
conceptually; the magnitudes are what the valuation work supplies.
**Source:** Damodaran (2012) Ch.2 pp.58-79.

The CFA L1 framing parallels this structure: the analyst estimates an
intrinsic value, compares it to the observed market price, and
classifies the asset as overvalued (price > intrinsic), fairly valued
(price ~ intrinsic), or undervalued (price < intrinsic). **Source:**
CFA L1 Curriculum (2022) Vol.4/pp.361-416.

## Mathematical Reasoning

For an asset producing expected cash flows `CF_i` over a horizon `T`
indexed by `i` and discounted at a required return `r` reflecting
cash-flow risk, intrinsic value is the present value of the cash-flow
stream `V_0 = sum_{i in 1..T} CF_i / (1 + r)^i`. **Source:**
Damodaran (2012) Ch.2 pp.58-79.

The cash-flow stream may extend to perpetuity for a going concern; in
that case the sum is decomposed into an explicit-forecast horizon
plus a terminal value capturing all cash flows beyond the explicit
horizon. **Source:** Damodaran (2012) Ch.2 pp.58-79.

The required return `r` is decomposed into a riskless component and a
risk premium (see [`eq-discount-rate-and-required-return-foundations`
](./eq-discount-rate-and-required-return-foundations.md)). The cash-
flow stream `CF_i` itself is built from earnings, reinvestment, and
growth assumptions specific to the asset class — for equity claims,
the stream is dividends (DDM), free cash flow to equity (FCFE), or
free cash flow to the firm (FCFF) depending on the level of the
claim being priced. **Source:** Damodaran (2012) Ch.2 pp.58-79.

The intrinsic-value estimate is conditional on its inputs. Two
analysts working from the same observed price may produce different
intrinsic-value estimates because they differ on `CF_t` (growth /
reinvestment / margin assumptions) or on `r` (risk perception).
Valuation does not deliver a unique number; it delivers a defended
position whose drivers are auditable. **Source:** Damodaran (2012)
Ch.1 pp.41-57.

## See Also

- [`eq-discount-rate-and-required-return-foundations`](./eq-discount-rate-and-required-return-foundations.md) — how `r` decomposes into riskless rate plus risk premium and the CAPM frame
- [`eq-dividend-discount-models`](./eq-dividend-discount-models.md) — DDM as the canonical equity intrinsic-value formula
- [`eq-pe-and-relative-valuation`](./eq-pe-and-relative-valuation.md) — relative-valuation approach as a sibling of intrinsic valuation

## Escalate to Raw When

Open Damodaran's Investment Valuation 4ed Ch.1-2 directly when any
of the criteria below applies. **Source:** Damodaran (2012) Ch.1-2
pp.41-79.

- the asset's cash flows are contingent on a future state (e.g., a development-stage drug, a real option to delay or expand) — Damodaran Ch.5 and Ch.28-29 are the canonical references for option-pricing-style valuation. **Source:** Damodaran (2012) Ch.2 pp.58-79.
- the firm is in distress and survival is uncertain — Damodaran Ch.30 (Valuing Equity in Distressed Firms) reframes the discounting horizon to allow non-going-concern outcomes. **Source:** Damodaran (2012) Ch.2 pp.58-79.
- a sector-specific valuation framework is required (financial-service firms, money-losing firms, start-ups, private firms) — Damodaran Ch.21-24 supply the sector-specific adjustments. **Source:** Damodaran (2012) Ch.2 pp.58-79.
