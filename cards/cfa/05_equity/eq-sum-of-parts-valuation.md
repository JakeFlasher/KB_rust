---
schema_version: "cacg.v0"
id: "eq-sum-of-parts-valuation"
title: "Sum-of-Parts Valuation"
reading_id: "05_equity"
summary: "Valuing a multi-business firm by separately valuing each segment with the methodology appropriate to its economics (DCF at segment WACC, peer multiples, or asset-based) and summing the parts. Parent-level adjustments — corporate overhead PV, cross-segment synergies, holding-company discount, parent debt — bridge from operating-segment values to firm value and then to equity value."
tags: ["equity", "sum-parts"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p594:0767"
    chunk_hash: "36351c02e8d196ccf88626a862484b25a4ad00cc6d6afc96e39c88366a241284"
    page_range: [594, 594]
    quote: "You would value the equity in each holding separately and estimate the value of the proportional holding."
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p562:0715"
    chunk_hash: "283fdda9e851c4a6f57724949befe50cc04d082398083a0d30e45f20d642ee1e"
    page_range: [562, 562]
    quote: "The beta for a multibusiness company is a weighted average of the betas of the different businesses it operates in."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2302:3373"
    chunk_hash: "8b856ecd11930b6c085249fe7c054bed9f4aac81bc041dd54136aab03c1e7f82"
    page_range: [2302, 2302]
    quote: "The theory underlying the asset-based approach is that the value of a business is equal to the sum of the value of the business’s assets."
    edge_type: "supports"
card_hash: "e0262bc0ab65bdc9c94e813de2e7b8cf592136699e0ff59b055589c1202ab5b7"
---
# Sum-of-Parts Valuation

## Intuition

A multi-business firm — a conglomerate, a holding company, a
diversified industrial — has segments with different economics:
different growth rates, different margins, different risk profiles,
different capital intensities. An aggregate DCF that uses a single
discount rate and a single growth assumption blurs those
differences. Sum-of-parts (SOTP) valuation values each segment
separately and adds the pieces, preserving the per-segment economics.
**Source:** Damodaran (2012) Ch.16 pp.583-615.

The SOTP question is when to bother. For a focused single-business
firm, an aggregate DCF gives the same answer with less work. SOTP
adds information when segment economics diverge meaningfully — for
example, when one segment is mature-and-cash-generating while
another is high-growth-and-cash-burning, or when one segment
operates in a high-risk geography while another does not. The
parent-level adjustments (corporate overhead, holding-company
discount, cross-segment debt allocation) are the SOTP-specific
machinery that an aggregate DCF avoids by construction. **Source:**
Damodaran (2012) Ch.16 pp.583-615.

```
multi-business firm
   |
   |  +-- segment A: stable, cash-cow                 --> value A
   |  |     (DCF at WACC_A; or peer multiples on EBITDA_A)
   |  |
   |  +-- segment B: high-growth, cash-burning         --> value B
   |  |     (DCF at WACC_B; OR exit multiple at horizon)
   |  |
   |  +-- segment C: capital-intensive, regulated      --> value C
   |        (DCF at WACC_C; or P/B-style book value)
   v
sum: V_segments = value A + value B + value C
                  |
                  +-- subtract parent-level corporate overhead PV
                  +-- subtract / add cross-segment synergy or
                  |   diseconomy adjustments
                  +-- subtract holding-company discount (if any)
                  +-- subtract net debt at parent
                  v
              equity value of the parent firm
```

## Definition

Sum-of-parts valuation is the technique of valuing a multi-business
firm by independently valuing each segment using the methodology
appropriate to that segment, then aggregating the segment values
with parent-level adjustments to recover the firm-level value.
**Source:** Damodaran (2012) Ch.16 pp.583-615.

The per-segment methodology can be DCF (discounting segment-level
cash flows at a segment-specific cost of capital), comparable-
company multiples (applying peer multiples for the segment's
industry to the segment's matching scale variable), or asset-based
(book value or net asset value for asset-heavy segments). Damodaran's
recommendation is to choose the methodology that best matches the
segment's economics, not to force a single methodology across all
segments. **Source:** Damodaran (2012) Ch.16 pp.583-615.

The segment-cost-of-capital adjustment is the SOTP-specific
discipline that mirrors the bottom-up beta construction in
[`eq-discount-rate-and-required-return-foundations`](./eq-discount-rate-and-required-return-foundations.md).
Each segment carries the cost of capital appropriate to a pure-play
firm in that industry, weighted by the segment's contribution to
firm value. The aggregate firm cost of capital is then the value-
weighted average of the segment costs of capital — but the SOTP
analysis uses the per-segment costs directly, not the aggregate.
**Source:** Damodaran (2012) Ch.16 pp.583-615.

The parent-level adjustments include: (a) corporate overhead — the
present value of unallocated headquarters expenses that no individual
segment bears; (b) holding-company discount — a markdown applied
when the market historically values diversified holding companies
below the sum of their segment values, often attributed to
governance frictions or capital-allocation inefficiency; (c) cross-
segment synergies — present value of revenue or cost benefits that
arise from combined ownership; (d) consolidated debt allocation —
attributing the firm's debt across segments using a value-weighted
or business-judgement allocation rule. **Source:** Damodaran (2012)
Ch.16 pp.583-615.

## Mathematical Reasoning

The SOTP bridge has three stages, in the Damodaran Ch.16 form: first
aggregate operating segment values into operating enterprise value
with operating-side parent-level adjustments; then bridge to firm
value by adding non-operating assets (cash, marketable securities,
non-operating stakes); then bridge to equity value by subtracting
gross nonequity claims (gross debt, preferred, minority interests).
**Source:** Damodaran (2012) Ch.16 pp.583-615.

```
V_operating = sum over segments of (operating Value_segment_i)
            + PV(cross-segment synergies)
            - PV(corporate overhead)
            - holding-company discount

Firm Value = V_operating
           + cash and marketable securities at parent
           + other non-operating assets at parent

Equity Value = Firm Value
             - gross debt at parent
             - preferred stock at parent
             - minority interests at parent
             - other nonequity claims at parent
```

Each segment value `Value_segment_i` is the segment's operating
value, computed by the per-segment methodology (segment DCF using
`WACC_i` and segment-specific forecast, or peer multiple
`multiple_i · X_i` for the segment's scale variable `X_i`). The
operating-side aggregation collapses the segment-specific discount
rates and growth rates into a single operating enterprise value
without ever computing an aggregate firm-level discount rate. The
firm-value stage adds non-operating assets that are valued separately
from the operating businesses (cash and marketable securities at
their carrying value; non-operating stakes at their market value or
analyst-derived fair value); the equity-value stage subtracts the
gross financial claims senior to common equity, in the same form as
the equity bridge in [`eq-dcf-mechanics`](./eq-dcf-mechanics.md).
The three-stage form avoids the cash-double-count error of mixing
"non-operating assets" (which include cash) with "net debt" (which
already nets cash); if a single-stage form using net debt is
preferred, the non-operating-assets line must explicitly exclude
the cash already netted in net debt. **Source:** Damodaran (2012)
Ch.16 pp.583-615.

The segment DCF inherits the structure of
[`eq-dcf-mechanics`](./eq-dcf-mechanics.md): explicit-forecast period
of segment cash flows discounted at the segment WACC, plus a segment
terminal value computed at the segment stable-growth rate. The
segment WACC uses the pure-play industry beta (or bottom-up beta)
from the segment's industry, the same target capital structure as
peers in that industry, and the corporate marginal tax rate. The
segment terminal-value form follows the same conventions as the
aggregate DCF (Gordon-growth perpetuity or exit multiple). **Source:**
Damodaran (2012) Ch.15 pp.538-582.

The segment-multiple alternative writes `Value_segment_i =
multiple_i_peer · X_segment_i`, where `multiple_i_peer` is drawn
from a peer set of pure-play firms in that segment's industry and
`X_segment_i` is the segment's matching scale variable (segment
EBITDA, segment book value, segment revenue). The peer-multiple
choice follows the principles in
[`eq-comparable-company-analysis`](./eq-comparable-company-analysis.md):
peer set selected by underlying business comparability, multiple
family chosen for the segment's economics. **Source:** Damodaran
(2012) Ch.16 pp.583-615.

The corporate-overhead adjustment is the present value of
unallocated headquarters expenses growing at a stable rate and
discounted at the firm's blended cost of capital. The negative sign
reflects that overhead reduces firm value — these expenses are
incurred but not borne by any segment's standalone cash flow.
Damodaran's recommendation is to compute the overhead PV explicitly
rather than allocate overhead pro-rata across segments, because
allocation distorts the segment-level economics. **Source:**
Damodaran (2012) Ch.16 pp.583-615.

The holding-company discount has both an empirical and a
theoretical motivation. Empirically, diversified holding companies
have historically traded at discounts to their NAV; theoretically,
the discount captures governance frictions, capital-allocation
inefficiency, and information asymmetry between the holding company
and its segments. The discount is firm-specific and judgement-driven;
applying a generic 10-20% range is a starting point, not a derived
quantity. **Source:** Damodaran (2012) Ch.16 pp.583-615.

The CFA L1 frame presents SOTP as a complement to aggregate DCF for
multi-business firms, identifies segment-specific cost of capital as
the methodological core, and emphasizes that parent-level adjustments
(overhead, holding-company discount) are the SOTP-specific machinery.
**Source:** CFA L1 Curriculum (2022) Vol.4/pp.361-416.

## See Also

- [`eq-dcf-mechanics`](./eq-dcf-mechanics.md) — the per-segment DCF engine
- [`eq-comparable-company-analysis`](./eq-comparable-company-analysis.md) — the per-segment peer-multiple alternative
- [`eq-discount-rate-and-required-return-foundations`](./eq-discount-rate-and-required-return-foundations.md) — the bottom-up beta construction that supplies segment-specific cost of capital

## Escalate to Raw When

Open Damodaran Ch.16 directly when any of the criteria below applies.
**Source:** Damodaran (2012) Ch.16 pp.583-615.

- the firm has many segments with widely divergent economics, and aggregating them under a single discount rate visibly distorts the valuation — Damodaran Ch.16 develops the per-segment methodology dispatch in detail. **Source:** Damodaran (2012) Ch.16 pp.583-615.
- the corporate-overhead allocation is contested and the analyst needs the present-value computation rather than a pro-rata allocation — Damodaran Ch.16 derives the explicit overhead-PV form. **Source:** Damodaran (2012) Ch.16 pp.583-615.
- the holding-company discount selection is non-obvious (governance-distinctive holding company, family-controlled conglomerate, sovereign-linked entity) — Damodaran Ch.16 discusses the empirical and theoretical determinants of the discount. **Source:** Damodaran (2012) Ch.16 pp.583-615.
