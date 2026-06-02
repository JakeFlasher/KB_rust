---
schema_version: "cacg.v0"
id: "eq-implied-cost-of-capital-foundations"
title: "Implied Cost of Capital Foundations"
reading_id: "05_equity"
summary: "The implied-cost-of-capital approach inverts the intrinsic-value formula: given observed price and a forecast cash-flow stream, solve for the discount rate that equates the model price to the observed price. The Gordon-growth special case admits the closed-form r* = D1/P0 + g. Damodaran develops the foundational machinery; multi-stage variants require numerical iteration."
tags: ["equity", "implied-cost"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p279:0310"
    chunk_hash: "1340259d891857cee0ab25f9560b9ced2fef8d8dab890cc7c80c8608544bbb99"
    page_range: [279, 279]
    quote: "We label this expected return the cost of equity. Similarly, the expected return that lenders hope to make on their investments includes a premium for default risk"
    edge_type: "defines"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p487:0607"
    chunk_hash: "ee9e2c083b2ce3e4e8399a7bf6da7c2e29a587e4f416e432890d00bef8d8eb7d"
    page_range: [487, 487]
    quote: "While many analysts have turned away from the dividend discount model and view it as outmoded, much of the intuition that drives discounted cash flow valuation stems from the dividend discount model."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2336:3423"
    chunk_hash: "34790b0cd36b29cca4f5234c0c8e2c225304fe82386327d5002e15363ab10c8e"
    page_range: [2336, 2337]
    quote: "Company data for dividend per share (DPS), earnings per share (EPS), share price, and price-to-earnings ratio (P/E) for the most recent five years are presented in Exhibit"
    edge_type: "supports"
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p275:0304"
    chunk_hash: "6822b7daf4f3a25d1027258300f7707d897e8414403649c3d9fffedd36c22d28"
    page_range: [275, 276]
    quote: "This approach does require that we start with a valuation model for equities, and estimate the expected growth and cash flows, collectively, on equity investments."
    edge_type: "supports"
card_hash: "afdd3ddd92ac43137a9f89aa7300d8f8b2bdb3dac898db4fb56b8f7c9e4183de"
---
# Implied Cost of Capital Foundations

## Intuition

Every intrinsic-value model has the form `V_0 = sum(CF_i) / (1 + r)^i`
— given expected cash flows and a discount rate, the model produces a
value. The implied-cost-of-capital approach inverts the model:
given an observed price `P_0` and a forecast of expected cash flows,
it solves for the discount rate `r*` that makes the model's value
equal the observed price. The market's price is treated as the
output of an unobserved investor-required-return computation; the
analyst recovers that required return by inverting the pricing
machinery. **Source:** Damodaran (2012) Ch.8 pp.279-332.

The implied cost of capital is the alternative to the CAPM-input
approach to estimating the cost of equity. Where CAPM constructs `r`
from a riskless rate and a beta-times-equity-risk-premium term, the
implied approach reads `r*` off market prices directly. The two
methods are complementary: CAPM gives a model-based required return
the analyst supplies as input; the implied approach gives a market-
based required return the analyst recovers as output. Comparing the
two is itself a diagnostic — a large gap suggests either CAPM
input mis-specification or market mispricing of the asset.
**Source:** Damodaran (2012) Ch.8 pp.279-332.

```
<!-- primitive: implied-cost-of-capital-loop source: _diagram_primitives.md -->
                       observed price P_0
                       forecast CF_1..N, TV_N
                              |
                              v
                       guess discount rate r
                              |
                              v
                       compute model price
                       PV(r) = sum CF_t / (1 + r)^t
                              |
                              v
                       PV(r)  =  P_0  ?
                              |
                  +-----------+-----------+
                  | no                    | yes
                  v                       v
              adjust r                r* := r
              (Newton / bisection)    implied cost
              loop back               of capital
                  |                       |
                  +-----------+           v
                              |       report r*
                              |       compare to claim-
                              |       level benchmark
                              v
                          (re-enter
                           guess step)
```

## Definition

The implied cost of capital `r*` is the discount rate that equates a
forecast cash-flow stream's present value to the observed market
price. For an equity claim valued by the DDM, `r*` solves `P_0 =
sum(D_i) / (1 + r*)^i + TV_T / (1 + r*)^T`. For a firm valued by
DCF, `r*` solves `EV_0 = sum(FCFF_i) / (1 + r*)^i + TV_N / (1 +
r*)^N`. The cash-flow / discount-rate dispatch from
[`eq-dcf-mechanics`](./eq-dcf-mechanics.md) carries through to the
implied case: equity-claim prices yield implied cost of equity;
enterprise-value prices yield implied WACC. **Source:** Damodaran
(2012) Ch.8 pp.279-332.

The implied cost of capital is also called the market-implied
required return, the implied discount rate, or — in the special case
of the constant-growth Gordon-growth DDM — the implied cost of
equity from the dividend-yield + growth identity. The Gordon-growth
inversion is `r* = D_1 / P_0 + g`, where `D_1` is the next-period
expected dividend, `P_0` is the observed price, and `g` is the
assumed perpetual growth rate. This closed-form inversion is
direct (no iteration) but assumes the firm is in stable Gordon-
growth state; the multi-stage inversion requires iteration.
**Source:** Damodaran (2012) Ch.14 pp.487-537.

The cash-flow forecast is an EXOGENOUS analyst input. The implied
cost of capital is the unique unknown the inversion solves for; the
expected cash flows are the analyst's separate forecast, drawn from
analyst consensus, the firm's guidance, or the analyst's own
modeling. The quality of the implied `r*` is bounded by the quality
of the cash-flow forecast — biased forecasts produce biased `r*`.
**Source:** Damodaran (2012) Ch.8 pp.279-332.

The implied equity risk premium is the cross-firm or aggregate-
market analogue of the implied cost of equity: it is the difference
between the implied cost of equity and the riskless rate. Inverting
an aggregate-market index price (S&P 500 level) and aggregate-
expected dividends/buybacks yields the implied market ERP, which
Damodaran maintains as a monthly time series on his website. The
data-pipeline mechanics of this estimation are in
[`eq-implied-cost-of-capital-from-market-prices`](./eq-implied-cost-of-capital-from-market-prices.md).
**Source:** Damodaran (2012) Ch.8 pp.279-332.

## Mathematical Reasoning

The general implied-cost-of-capital equation, in the DDM form,
expresses observed price as the present value of expected dividends
plus terminal value, all discounted at the unknown `r*`. **Source:**
Damodaran (2012) Ch.14 pp.487-537.

```
P_0 = sum_{i=1..T} D_i / (1 + r*)^i  +  TV_T / (1 + r*)^T
```

where `r*` is the unknown, `P_0` is the observed price, `D_i` are
expected dividends through the explicit-forecast horizon `T`, and
`TV_T` is the terminal value (typically Gordon-growth form `TV_T =
D_{T+1} / (r_stable - g_stable)`). The equation is a polynomial in
`(1 + r*)` and is solved numerically by bisection or Newton-
Raphson. **Source:** Damodaran (2012) Ch.14 pp.487-537.

The Gordon-growth special case admits a closed-form inversion. The
DDM Gordon-growth equation `P_0 = D_1 / (r* - g)` rearranges to the
dividend-yield-plus-growth identity. **Source:** Damodaran (2012)
Ch.14 pp.487-537.

```
r* = D_1 / P_0 + g
```

Interpreted: the implied cost of equity equals the forward dividend
yield plus the perpetual growth rate. This identity is the
foundation of the implied-cost-of-equity intuition — for a stable-
growth mature firm, the cost of equity is the dividend yield the
investor receives plus the growth rate the dividend is expected to
deliver. The closed form requires `g` strictly less than `r*`
(otherwise the underlying perpetuity diverges). **Source:**
Damodaran (2012) Ch.14 pp.487-537.

The firm-DCF inversion replaces dividends with FCFF and the cost of
equity with WACC: `EV_0 = sum FCFF_i / (1 + WACC*)^i + TV_N / (1 +
WACC*)^N`, where `EV_0` is the observed enterprise value (market
cap + gross debt + minority interests, less non-operating assets).
The implied WACC is then decomposed into implied cost of equity
and after-tax cost of debt by holding the capital-structure weights
and the after-tax cost of debt fixed. **Source:** Damodaran (2012)
Ch.8 pp.279-332.

```
WACC* = (E/(D+E)) · r_e* + (D/(D+E)) · r_d · (1 - t)
=>  r_e* = (WACC* - (D/(D+E)) · r_d · (1 - t)) / (E/(D+E))
```

This decomposition delivers the same `r_e*` as the equity-DCF
inversion under matched assumptions (the matched-assumption
identity from [`eq-dcf-mechanics`](./eq-dcf-mechanics.md) carries
through to the inversion). **Source:** Damodaran (2012) Ch.8
pp.279-332.

The implied-vs-CAPM gap is the diagnostic comparison: `r_e_CAPM = Rf
+ beta · ERP_CAPM` versus `r_e_implied = D_1/P_0 + g` (Gordon-growth
case) or the multi-stage iterative solution (general case). A
material gap between the two suggests either an input mis-
specification in CAPM (wrong beta, wrong ERP estimate) or a
mispricing in the observed `P_0`. The gap sign distinguishes the
two interpretations: if `r_e_implied > r_e_CAPM`, the market either
demands a higher risk premium than CAPM specifies or has discounted
the price below intrinsic value; if `r_e_implied < r_e_CAPM`, the
opposite. **Source:** Damodaran (2012) Ch.8 pp.279-332.

The CFA L1 frame presents the dividend-yield-plus-growth identity
as the canonical implied-cost-of-equity intuition; the multi-stage
iterative inversion and the implied-WACC decomposition are
discussed at higher CFA levels. **Source:** CFA L1 Curriculum
(2022) Vol.4/pp.361-416.

## See Also

- [`eq-dividend-discount-models`](./eq-dividend-discount-models.md) — the DDM that implied cost of capital inverts
- [`eq-dcf-mechanics`](./eq-dcf-mechanics.md) — the firm-DCF that implied WACC inverts
- [`eq-discount-rate-and-required-return-foundations`](./eq-discount-rate-and-required-return-foundations.md) — the CAPM-input baseline that implied-vs-CAPM compares against
- [`eq-equity-risk-premium-intuition`](./eq-equity-risk-premium-intuition.md) — the implied ERP as the cross-sectional analogue

## Escalate to Raw When

Open Damodaran Ch.8 / Ch.14 directly when any of the criteria below
applies. **Source:** Damodaran (2012) Ch.8 pp.279-332.

- the implied-vs-CAPM gap is large and persistent and the analyst needs the diagnostic framework for distinguishing input mis-specification from mispricing — Damodaran Ch.8 develops the comparison in detail. **Source:** Damodaran (2012) Ch.8 pp.279-332.
- the explicit-forecast schedule for cash flows is non-standard (firm in transition, payout policy changing) and the multi-stage iterative inversion is required — Damodaran Ch.14 derives the multi-stage DDM inversion. **Source:** Damodaran (2012) Ch.14 pp.487-537.
- the analyst needs the data-pipeline mechanics of estimating an aggregate-market implied ERP from an index price + aggregate dividends/buybacks — see [`eq-implied-cost-of-capital-from-market-prices`](./eq-implied-cost-of-capital-from-market-prices.md). **Source:** Damodaran (2012) Ch.8 pp.279-332.
