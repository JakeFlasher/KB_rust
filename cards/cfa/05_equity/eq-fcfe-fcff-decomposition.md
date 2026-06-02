---
schema_version: "cacg.v0"
id: "eq-fcfe-fcff-decomposition"
title: "FCFE / FCFF Decomposition"
reading_id: "05_equity"
summary: "FCFE is cash available to equity holders after debt service; FCFF is cash available to all capital providers before debt service. FCFE feeds the equity DCF discounted at cost of equity; FCFF feeds the firm DCF discounted at WACC. Damodaran Ch.10 develops the cash-flow construction; Ch.15 develops the matched-assumption reconciliation."
tags: ["equity", "fcfe-fcff"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p359:0431"
    chunk_hash: "1888e77b755ed4f4986f09def1cd7290e3ff8cb592b1e731690c5bc14378a04a"
    page_range: [359, 359]
    quote: "To examine how much a firm is reinvesting, we will break it down into reinvestment in tangible and long-lived assets (net capital expenditures) and short-term assets (working capital)."
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p487:0608"
    chunk_hash: "ad3fc38b8fe9f5f90e39981a7925047b19101287b4bbef4dfd441cd7d9d7dc42"
    page_range: [487, 488]
    quote: "The third and most general measure of cash flow to equity is to compute cash flows left over after reinvestment and financing needs have been met (i.e., potential dividends)."
    edge_type: "supports"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p538:0678"
    chunk_hash: "0da7564857ee02ab58fccb33e1d9d4347e09b4df1d5761358cd83a49e4040f98"
    page_range: [538, 539]
    quote: "FCFF = EBIT(1 − Tax rate) + Depreciation − Capital expenditure − ΔWorking capital Since this cash flow is prior to debt payments, it is often referred to as an unlevered cash flow."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2309:3384"
    chunk_hash: "3fd5b07bea2c5c6f31a71b7ecd95eb69f208c2ed854103f33a2b9bc965921ff6"
    page_range: [2309, 2310]
    quote: "Dividend discount models focus on expected dividends. How is the required rate of return for use in present value models estimated?"
    edge_type: "supports"
card_hash: "16586993ea3d92d300f073dd4662b1ef3535dd72f524fa482144995b6c98c2ab"
---
# FCFE / FCFF Decomposition

## Intuition

Free cash flow is the cash a firm generates after paying for the
investments needed to sustain and grow its operations. The free
cash flow available to equity holders (FCFE) is what is left after
the firm has serviced debt; the free cash flow available to the
firm (FCFF) is what is generated before any payment to capital
providers — equity or debt. The two definitions answer different
questions and feed different DCF engines. **Source:** Damodaran
(2012) Ch.10 pp.359-383.

FCFE answers: how much cash COULD the firm pay to equity holders
this period if it chose to distribute everything available?
FCFF answers: how much cash COULD the firm pay to ALL capital
providers this period? FCFE is the bridge between the dividend
discount model's actual-distribution view and the potential-
distribution view; FCFF is the foundation for firm-level (enterprise)
valuation. **Source:** Damodaran (2012) Ch.10 pp.359-383.

```
operating cash flow
   |
   |  +-- subtract reinvestment (CapEx - depreciation;
   |  |   change in non-cash working capital)
   |  v
firm-level free cash flow before debt service
   |  ==> FCFF                  --> discount at WACC for firm value
   |
   |  +-- subtract after-tax interest + net debt repayment
   |  |   (or + net debt issuance)
   |  v
equity-level free cash flow after debt service
   |  ==> FCFE                  --> discount at cost of equity
   |
   v       reconciliation: equity value (firm-DCF route)
           = enterprise value - net debt
           equals equity value (FCFE route) under matched
           assumptions
```

## Definition

FCFE is the cash flow available to equity holders after the firm has
made all necessary reinvestments and serviced its debt. The Damodaran
construction starts from net income, subtracts net capital
expenditures (CapEx less depreciation — the reinvestment beyond
maintaining existing assets), subtracts the change in non-cash
working capital, and adjusts for net debt issuance (new borrowing
minus principal repayment). The net-CapEx form already absorbs the
non-cash depreciation reversal, so no separate D&A add-back is
required in this notation. **Source:** Damodaran (2012) Ch.10
pp.359-383.

FCFF is the cash flow available to all capital providers — equity
and debt — before any debt-service payments. The Damodaran
construction starts from after-tax operating income (EBIT times one
minus the tax rate), subtracts net capital expenditures, and
subtracts the change in non-cash working capital. FCFF excludes all
financing-side effects (interest, debt repayment, debt issuance);
those effects appear in the discount-rate side via WACC. **Source:**
Damodaran (2012) Ch.10 pp.359-383.

The dispatch rule pairs the cash-flow definition with the matching
discount rate: FCFE with the cost of equity to recover equity value
directly; FCFF with WACC to recover enterprise value, from which
equity value is derived by subtracting net debt. The DDM is the FCFE
model with potential dividends replaced by actual dividends; the two
agree when actual payout equals FCFE and diverge when the firm
distributes less or more than its FCFE. **Source:** Damodaran (2012)
Ch.14 pp.487-537.

The reinvestment quantity that links cash flow to growth — net
capital expenditures plus change in non-cash working capital — is the
denominator of the reinvestment-rate / return-on-capital identity that
governs sustainable growth (see
[`eq-payout-policy-and-growth`](./eq-payout-policy-and-growth.md)).
A firm whose reinvestment exceeds its operating cash flow has
negative FCFF, signalling that growth is being financed externally.
**Source:** Damodaran (2012) Ch.10 pp.359-383.

## Mathematical Reasoning

The FCFE construction in symbolic form starts from net income and
applies the cash-flow-statement adjustments below. **Source:**
Damodaran (2012) Ch.10 pp.359-383.

```
FCFE = Net Income
     - Net CapEx                       (CapEx - Depreciation)
     - Change in Non-cash Working Capital
     + Net Debt Issued                 (new borrowing - repayment)
```

Each term is a paraphrase of the cash-flow-statement decomposition:
net CapEx captures investment beyond what depreciation already
recovers (so the non-cash depreciation reversal is already absorbed
into this single line); working-capital changes capture cash tied up
in operations; the financing-side adjustment captures debt-service
mechanics. The equivalent gross-CapEx form `FCFE = Net Income + D&A -
CapEx - ΔNWC + Net Debt Issued` is algebraically identical because
`Net CapEx = CapEx - D&A`; the two notations must not be mixed
because doing so double-counts depreciation. **Source:** Damodaran
(2012) Ch.10 pp.359-383.

The FCFF construction in symbolic form starts from after-tax
operating income and applies the same operating reinvestment
adjustments without the financing-side terms. **Source:** Damodaran
(2012) Ch.10 pp.359-383.

```
FCFF = EBIT · (1 - t)
     - Net CapEx
     - Change in Non-cash Working Capital
```

The starting point is after-tax operating income — EBIT times one
minus the marginal tax rate `t` — which represents the cash earned
by operations under a hypothetical all-equity capital structure. The
financing-side adjustments present in FCFE (interest, debt issuance)
are absent because FCFF is pre-debt-service. **Source:** Damodaran
(2012) Ch.10 pp.359-383.

The reconciliation identity links FCFE and FCFF through the
financing-side bridge: `FCFE = FCFF - Interest · (1 - t) + Net Debt
Issued`. Interpreted: starting from FCFF (cash available to all
capital providers), subtract the after-tax interest paid to debt
holders (the cash that goes to debt service), then add back any net
new debt the firm issued (cash inflow from financing). The remainder
is FCFE — what is left for equity holders. **Source:** Damodaran
(2012) Ch.10 pp.359-383.

The matched-assumption identity links the two valuation routes:
under coherent growth, reinvestment, leverage, and tax assumptions,
the equity value computed by discounting FCFE at the cost of equity
equals the equity value computed by discounting FCFF at WACC and
subtracting net debt. Damodaran develops the proof by showing that
the financing-side terms cancel when the discount-rate side absorbs
them through WACC's debt-weight component. **Source:** Damodaran
(2012) Ch.15 pp.538-582.

In the DDM-vs-FCFE comparison, the gap between FCFE and actual
dividends measures the firm's distribution discipline. A firm that
distributes substantially less than its FCFE accumulates cash on its
balance sheet (or invests in non-operating assets); a firm that
distributes more than its FCFE is borrowing or issuing equity to
sustain the payout. The DDM uses actual dividends as the cash flow;
the FCFE model uses potential dividends. The two converge when
distribution equals capacity. **Source:** Damodaran (2012) Ch.14
pp.487-537.

The CFA L1 frame presents FCFE and FCFF as the two free-cash-flow
families, distinguishes the cost-of-equity vs WACC discount-rate
dispatch, and emphasizes that FCFE is the cash flow `that could be
paid out as dividends` while FCFF is the cash flow `available to all
capital providers`. **Source:** CFA L1 Curriculum (2022) Vol.4/pp.361-416.

## See Also

- [`eq-dcf-mechanics`](./eq-dcf-mechanics.md) — the DCF engine FCFE / FCFF feed
- [`eq-dividend-discount-models`](./eq-dividend-discount-models.md) — the DDM as the actual-distribution sibling of the FCFE potential-distribution view
- [`eq-payout-policy-and-growth`](./eq-payout-policy-and-growth.md) — reinvestment, payout, and the growth identity that links FCFE / FCFF to growth

## Escalate to Raw When

Open Damodaran Ch.10 / Ch.14 / Ch.15 directly when any of the criteria
below applies. **Source:** Damodaran (2012) Ch.10 pp.359-383.

- the firm's working-capital or CapEx schedule is irregular and the textbook constant-reinvestment shortcut fails — Damodaran Ch.10 develops the full balance-sheet-driven construction. **Source:** Damodaran (2012) Ch.10 pp.359-383.
- the FCFE-vs-FCFF identity fails to reconcile and the source of inconsistency must be located — Damodaran Ch.15 walks the matched-assumption proof in detail. **Source:** Damodaran (2012) Ch.15 pp.538-582.
- the firm is using preferred stock, hybrid securities, or convertibles in its capital structure — Damodaran Ch.10 discusses the adjustments needed beyond plain-vanilla debt and equity. **Source:** Damodaran (2012) Ch.10 pp.359-383.
