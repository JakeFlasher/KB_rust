---
schema_version: "cacg.v0"
id: "eq-dcf-mechanics"
title: "DCF Mechanics"
reading_id: "05_equity"
summary: "A discounted-cash-flow valuation discounts a forecast cash-flow stream at a matching discount rate, summing the explicit-period present values plus a terminal value. The cash-flow / discount-rate dispatch is mandatory: FCFE with cost of equity, FCFF with WACC. Damodaran Ch.14 develops the equity intrinsic-value engine; Ch.15 develops WACC and APV alternatives."
tags: ["equity", "dcf-mechanics"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p487:0607"
    chunk_hash: "ee9e2c083b2ce3e4e8399a7bf6da7c2e29a587e4f416e432890d00bef8d8eb7d"
    page_range: [487, 487]
    quote: "The simplest model for valuing equity is the dividend discount model (DDM)—the value of a stock is the present value of expected dividends on it."
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p488:0609"
    chunk_hash: "08cdb1fed30b5eee52fc2ac717c0fb8291bce2ee83f5b14957206db2e45b17b2"
    page_range: [488, 489]
    quote: "The model is flexible enough to allow for time-varying discount rates, where the time variation is because of expected changes in interest rates or risk across time."
    edge_type: "supports"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p537:0677"
    chunk_hash: "c39352348ca9ca6f8af3c6960cb645b796a75bd3827acadeb201ef02ce4ef48a"
    page_range: [538, 538]
    quote: "approaches to valuation in which the entire firm is valued, by either discounting the cumulated cash flows to all claim holders in the firm by the weighted average cost of capital"
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2336:3423"
    chunk_hash: "34790b0cd36b29cca4f5234c0c8e2c225304fe82386327d5002e15363ab10c8e"
    page_range: [2336, 2337]
    quote: "The choice of models will depend on the availability of information to put into the models."
    edge_type: "supports"
card_hash: "188b6de362fb76ded69d641677e07c8bf8d4c04af60e29609a65a464c7849e63"
---
# DCF Mechanics

## Intuition

A discounted-cash-flow valuation says: the value of a claim today is
the present value of the cash flows the claimant expects to receive
over the holding horizon, each discounted at a rate that reflects the
risk of that flow. The DDM is a special case in which the cash flow
is the dividend; firm-level DCF generalizes the cash flow to the free
cash flow available to ALL capital providers. **Source:** Damodaran
(2012) Ch.14 pp.487-537.

The DCF engine has three working parts: (a) a forecast of cash flows
out to a finite horizon, (b) a discount rate that matches the risk of
those cash flows and the identity of the claimant, and (c) a terminal
value that summarizes everything beyond the explicit-forecast horizon.
The analyst chooses the cash-flow definition first; the discount-rate
choice and the terminal-value form follow from that choice.
**Source:** Damodaran (2012) Ch.14 pp.487-537.

```
<!-- primitive: dcf-bridge source: _diagram_primitives.md -->
present value
   ^
   |  +--------+
   |  | FCF_1  |  / (1 + r)^1
   |  +--------+
   |  | FCF_2  |  / (1 + r)^2
   |  +--------+
   |  | FCF_3  |  / (1 + r)^3
   |  +--------+
   |  |  ...   |  ...
   |  +--------+
   |  | FCF_N  |  / (1 + r)^N
   |  +--------+
   |  |        |
   |  |  TV_N  |  / (1 + r)^N    <-- terminal value (perpetuity
   |  |        |                       or exit multiple)
   |  +--------+
   |
   +-> EV = sum of explicit-period PVs + PV of TV
       equity value = EV - net debt; per-share = / share count
```

## Definition

A DCF valuation is the present-value computation that maps a forecast
cash-flow stream and a matching discount rate to a single value. The
general form sums the discounted explicit-period flows and adds the
discounted terminal value. **Source:** Damodaran (2012) Ch.14
pp.487-537.

The cash-flow / discount-rate pairing is mandatory: equity-claim cash
flows (dividends or FCFE) are discounted at the cost of equity to
recover equity value directly; firm-level cash flows (FCFF) are
discounted at the weighted-average cost of capital (WACC) to recover
enterprise value, from which equity value is derived by subtracting
net debt and other non-equity claims. Mixing levels (e.g., FCFF
discounted at the cost of equity) double-counts or omits the financing
effect and is forbidden. **Source:** Damodaran (2012) Ch.14 pp.487-537.

The explicit-forecast horizon `N` is the analyst-chosen period over
which year-by-year cash-flow forecasting is feasible and informative;
beyond `N`, the firm is assumed to have transitioned to a stable-
growth state captured by a terminal-value formula. The horizon is set
by when the firm reaches stable growth, not by an arbitrary calendar.
**Source:** Damodaran (2012) Ch.15 pp.538-582.

The adjusted-present-value (APV) decomposition is an alternative firm-
level DCF that separates the unlevered firm value from the value of
the financing-side tax shield: discount unlevered free cash flows at
the unlevered cost of equity, then add the present value of the
expected interest tax shield. APV and WACC-DCF agree under matching
assumptions; APV is preferred when the capital structure is changing
materially over the forecast horizon. **Source:** Damodaran (2012)
Ch.15 pp.538-582.

## Mathematical Reasoning

The general DCF formula sums the discounted explicit-period cash
flows plus the discounted terminal value: `V_0 = sum_{i=1..N} CF_i /
(1 + r)^i + TV_N / (1 + r)^N`. Setting `CF_i = D_i` recovers the DDM
(see [`eq-dividend-discount-models`](./eq-dividend-discount-models.md));
setting `CF_i = FCFE_i` and using the cost of equity recovers the
equity-FCFE model; setting `CF_i = FCFF_i` and using WACC recovers
the firm-DCF model. **Source:** Damodaran (2012) Ch.14 pp.487-537.

Equity-FCFE and firm-FCFF DCF agree on equity value when assumptions
are matched: both must use consistent growth, reinvestment, leverage,
and tax assumptions, and the firm-DCF result must subtract the same
net debt that the FCFE model implicitly nets out via interest expense.
Damodaran's identity proof shows the two models recover the same
equity per share when assumptions are coherent; divergence in practice
reveals an inconsistency in the underlying assumptions, not a
preference between the models. **Source:** Damodaran (2012) Ch.15
pp.538-582.

WACC is the weighted average of the after-tax cost of debt and the
cost of equity, weighted by the market-value mix of debt and equity:
`WACC = (E / (D + E)) · r_e + (D / (D + E)) · r_d · (1 - t)`. The tax
factor `(1 - t)` captures the deductibility of interest at the
corporate marginal rate `t`; the equity weight uses market values to
match the discounting of market-value-equivalent cash flows.
**Source:** Damodaran (2012) Ch.15 pp.538-582.

The terminal value `TV_N` is itself a DCF computation: in the
Gordon-growth form `TV_N = CF_{N+1} / (r - g_stable)` where `g_stable
< r` ensures convergence (the same convergence condition that governs
the DDM Gordon-growth special case). The terminal value is then
discounted back from `N` to today at the same `r` used through the
explicit-forecast period. The depth of terminal-value mechanics —
exit-multiple alternatives, sensitivity to `g_stable`, fade
schedules — is in
[`eq-terminal-value-and-sensitivity`](./eq-terminal-value-and-sensitivity.md).
**Source:** Damodaran (2012) Ch.15 pp.538-582.

The APV decomposition writes firm value as the unlevered firm value
plus the present value of expected tax shields: `V_firm = V_unlevered
+ PV(tax shields) - PV(financial distress costs)`, where unlevered
free cash flows are discounted at the unlevered cost of equity and
tax shields are discounted at the cost of debt (or another rate
matching the riskiness of the shield). The APV form makes the
financing-side contribution explicit, useful when leverage shifts
materially. **Source:** Damodaran (2012) Ch.15 pp.538-582.

The CFA L1 frame presents DCF as the foundational equity-valuation
engine, distinguishes equity-claim from firm-claim discounting, and
emphasizes that the choice of cash-flow definition determines the
discount-rate match and the terminal-value form. **Source:** CFA L1
Curriculum (2022) Vol.4/pp.361-416.

## See Also

- [`eq-intrinsic-value`](./eq-intrinsic-value.md) — the general intrinsic-value frame DCF instantiates
- [`eq-dividend-discount-models`](./eq-dividend-discount-models.md) — the dividend-stream special case of DCF
- [`eq-discount-rate-and-required-return-foundations`](./eq-discount-rate-and-required-return-foundations.md) — the cost-of-equity / WACC discount-rate inputs

## Escalate to Raw When

Open Damodaran Ch.14 / Ch.15 directly when any of the criteria below
applies. **Source:** Damodaran (2012) Ch.14 pp.487-537.

- the capital structure changes materially over the forecast horizon and APV vs WACC-DCF give materially different equity values — Damodaran Ch.15 derives the APV decomposition and the conditions under which the two models agree. **Source:** Damodaran (2012) Ch.15 pp.538-582.
- the FCFE-vs-FCFF identity fails to reconcile and the source of inconsistency must be located — Damodaran Ch.15 walks the matched-assumption proof in detail. **Source:** Damodaran (2012) Ch.15 pp.538-582.
- the terminal-value form is unclear or the explicit-forecast horizon is in question — see [`eq-terminal-value-and-sensitivity`](./eq-terminal-value-and-sensitivity.md) and Damodaran Ch.15. **Source:** Damodaran (2012) Ch.15 pp.538-582.
