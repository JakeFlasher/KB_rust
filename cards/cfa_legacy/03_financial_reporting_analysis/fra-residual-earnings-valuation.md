---
schema_version: "cacg.v0"
id: "fra-residual-earnings-valuation"
title: "Residual Earnings Valuation"
reading_id: "03_financial_reporting_analysis"
summary: "Framing residual-earnings valuation as the accounting-anchored equity-value framework: intrinsic value = book value + PV of future residual earnings, where residual earnings = NI − r × BV_prev. The justified P/B drops out as a corollary; the accounting-policy invariance property makes the framework robust to choice."
tags: ["financial-reporting", "residual-earnings"]
citations:
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p169:0282"
    chunk_hash: "7eba7fe6369e5f48e3c1e65ae9479e08375216521ad70fc2c792f7d6e4bd5466"
    page_range: [169, 170]
    quote: "A measure that captures the value added to book value is residual earnings or residual income."
    edge_type: "defines"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p169:0282"
    chunk_hash: "7eba7fe6369e5f48e3c1e65ae9479e08375216521ad70fc2c792f7d6e4bd5466"
    page_range: [169, 170]
    quote: "The residual earnings value for a terminal project is always the same as that calculated with discounted cash flow methods."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2305:3379"
    chunk_hash: "26f59888d6000d86e025c5553ad48f88468a861e6168c672055ec4b21289465a"
    page_range: [2305, 2306]
    quote: "If the issuing company is assumed to be a going concern, the intrinsic value of a share is the present value of expected future dividends."
    edge_type: "supports"
card_hash: "59c81776c83fbf6727010bdd26f781acdb1d601a4a71b0dd0bf8ebc1ca7931d5"
---
# Residual Earnings Valuation

## Intuition

Residual-earnings valuation answers the same question as
discounted-cash-flow valuation — what is the firm worth? — but it
starts from the firm's accounting books rather than from its
forecasted free cash flows. The starting point is book value
(equity per share); the analyst then asks whether the firm is
expected to earn more than the cost of equity on its accounting
investment. If yes, the firm should be worth more than its book
value, and the premium is the present value of those future
above-cost-of-equity earnings. If no, the firm should trade at or
below book. **Source:** Penman (2013) Ch.5 pp.140-177.

The framework is anchored on accounting numbers because the analyst
has them directly from the financial statements; cash-flow
valuation requires constructing an FCF forecast that often involves
significant judgment. Residual-earnings valuation lets the analyst
do most of the work in earnings-per-share space, where accruals
have already smoothed the volatility of cash flows. The framework
therefore complements rather than replaces DCF: both approaches
should converge on the same intrinsic value when applied
consistently, but they emphasize different inputs and forecast
horizons. **Source:** Penman (2013) Ch.5 pp.140-177.

```
+--------------------------------------------+
| Residual Earnings Valuation Bridge         |
+--------------------------------------------+
|  Book Value (BV)                           |
|       |                                    |
|       +- Forecast period earnings: NI_t    |
|       +- Cost of equity charge: r × BV_{t-1}|
|       +- Residual Earnings: RE_t = NI_t -  |
|          r × BV_{t-1}                      |
|       v                                    |
|  Sum: BV + Σ_t RE_t / (1+r)^t              |
|       |                                    |
|       v                                    |
|  Intrinsic Equity Value (V)                |
|                                            |
|  Justified P/B = 1 + Σ_t RE_t/BV / (1+r)^t |
+--------------------------------------------+
```

The bridge above shows the residual-earnings valuation chain:
beginning book value plus the present value of forecast residual
earnings yields intrinsic equity value. The justified
price-to-book multiple drops out as a corollary: a firm earning
exactly the cost of equity has zero residual earnings and trades
at book; a firm earning more trades at a premium; a firm earning
less trades at a discount. **Source:** Penman (2013) Ch.5
pp.140-177.

## Definition

Residual earnings (sometimes called residual income, abnormal
earnings, or excess earnings) for a period is the period's net
income less a charge for the cost of equity capital applied to
the beginning-period book value of equity: `RE_t = NI_t − r ×
BV_{t-1}` where `r` is the firm's cost of equity. Residual
earnings represent the value the firm creates ABOVE the
opportunity cost of the equity capital it employs. **Source:**
Penman (2013) Ch.5 pp.140-177.

The residual-earnings valuation model writes intrinsic equity
value as the sum of beginning book value plus the present value
of all future residual earnings, summed across the forecast
horizon and discounted at the cost of equity. In the perpetual
form, intrinsic value equals book value plus the discounted
infinite stream of residual earnings; in practice the analyst
truncates the sum to an explicit forecast horizon and adds a
continuing-value tail that captures residual earnings beyond the
explicit horizon. **Source:** Penman (2013) Ch.5 pp.140-177.

The framework rests on two structural identities. The clean-
surplus relation requires that all changes to equity flow either
through net income or through transactions with shareholders
(dividends, share issuance, share repurchase): `BV_t = BV_{t-1} +
NI_t − Div_t + Iss_t`. The cost-of-equity charge isolates the
shareholder-required return: a firm that earns exactly the cost of
equity returns the opportunity cost and creates no value above it.
**Source:** Penman (2013) Ch.5 pp.140-177.

The framework's equivalence with DCF is structural: both
approaches discount future shareholder claims (dividends in DDM,
free cash flow to equity in DCF, residual earnings in RE) at the
cost of equity. The three approaches yield the same intrinsic
value when applied to consistent forecasts; they differ in which
financial-statement quantity drives the forecast. RE drives the
forecast from the income-statement and balance-sheet structure
that the analyst can read directly from the firm's accounting.
**Source:** Penman (2013) Ch.5 pp.140-177.

## Mathematical Reasoning

The residual-earnings identity follows directly from the clean-
surplus relation and the present-value equation. Starting from the
dividend-discount-model expression of intrinsic value (the
discounted infinite stream of expected dividends), substitute the
clean-surplus relation `Div_t = NI_t − (BV_t − BV_{t-1})`
(ignoring share transactions for simplicity), and after algebraic
manipulation the present-value sum reduces to beginning book value
plus the discounted infinite stream of residual earnings, with
each period's residual earnings defined as `NI_t − r × BV_{t-1}`.
The reduction is exact under the clean-surplus assumption.
**Source:** Penman (2013) Ch.5 pp.140-177.

The justified price-to-book multiple drops out by dividing both
sides by beginning book value: the justified P/B equals one plus
the discounted infinite stream of period residual earnings each
divided by beginning book value. A firm earning exactly the cost
of equity in every future period has zero residual earnings each
period, and the multiple collapses to P/B equal to one. A firm
earning more than the cost of equity has positive residual
earnings and a multiple above one; a firm earning less has the
opposite. The justified P/B is therefore a direct read of
expected future excess earnings, not an arbitrary market multiple.
**Source:** Penman (2013) Ch.5 pp.140-177.

The accounting-policy invariance property is the framework's
distinctive feature. Because residual earnings deduct the cost-of-
equity charge from net income computed on the same accounting
policy used to record the equity book value, accounting-policy
choices that increase reported net income (capitalization rather
than expensing, slower depreciation, less conservative provisions)
also increase the equity book value on which the cost-of-equity
charge is computed. The two effects offset; the resulting residual
earnings stream is approximately invariant to accounting-policy
choice. The intrinsic value the framework produces is therefore
robust to the accounting-policy choice in a way that pure earnings-
based or book-value-based metrics are not. **Source:** Penman
(2013) Ch.5 pp.140-177.

The framework's continuing-value treatment is structural. Beyond
an explicit forecast horizon, the analyst typically assumes
residual earnings grow at a sustainable rate `g` (often the
long-run economy growth rate) or fade to zero (mean-reversion to
the cost of equity). The continuing-value formula `CV = RE_T+1 /
(r − g)` (Gordon-growth form) or its perpetual-zero variant adds
to the explicit-horizon present value. The continuing-value choice
is the analyst's biggest forecast-horizon judgment. **Source:**
Penman (2013) Ch.5 pp.140-177.

The framework complements 05 Equity's DCF and multiples cards.
DCF (existing 05 card `eq-dcf-mechanics`) discounts forecast cash
flows at WACC; residual-earnings discounts forecast accounting
earnings above cost-of-equity. Multiples-based valuation (existing
05 cards `eq-pe-and-relative-valuation`, `eq-pb-and-multiples-
taxonomy`) reads off market-implied multiples and applies them.
The Penman-anchored treatment grounds the multiples in residual-
earnings logic: the justified P/B identity above is derivable from
RE valuation. **Source:** Penman (2013) Ch.5 pp.140-177.

The CFA L1 framing supports residual-earnings valuation as one of
several DDM-equivalent approaches. The curriculum's broader
intrinsic-valuation framework treats DCF, DDM, and residual-
earnings as three equivalent presentations of the same
shareholder-claim-discounting principle. **Source:** CFA L1
Curriculum (2022) Vol.4/pp.361-416.

## See Also

- [`fra-cash-vs-accrual-accounting`](./fra-cash-vs-accrual-accounting.md) — accrual accounting is the foundation that makes residual-earnings valuation tractable from accounting numbers
- [`fra-income-statement-foundations`](./fra-income-statement-foundations.md) — net income enters residual-earnings via the period earnings figure
- [`fra-ratio-decomposition-dupont`](./fra-ratio-decomposition-dupont.md) — DuPont's ROE decomposition feeds the residual-earnings forecast (RE = NI − r×BV depends on ROE relative to r)
- [`eq-intrinsic-value`](../05_equity/eq-intrinsic-value.md) — accounting-first vs market-data-first framing differential: this card anchors on Penman's accrual-accounting frame, whereas the 05 intrinsic-value card anchors on Damodaran's DCF-first treatment; both produce equivalent intrinsic-value estimates under consistent forecasts
- [`eq-pb-and-multiples-taxonomy`](../05_equity/eq-pb-and-multiples-taxonomy.md) — the justified P/B identity here grounds the market-multiple presentation in 05

## Escalate to Raw When

Open Penman Ch.5 directly when any of the criteria below applies.
**Source:** Penman (2013) Ch.5 pp.140-177.

- the analyst is constructing a residual-earnings forecast with
  significant accounting-policy choices that may or may not be
  invariant under the framework — Penman's discussion of when the
  invariance property holds and when it breaks is the canonical
  reference. **Source:** Penman (2013) Ch.5 pp.140-177.
- the analyst is comparing residual-earnings and DCF valuations
  for the same firm and finding they diverge — Penman's reconciliation
  of the two frameworks identifies the common forecast-input
  inconsistencies. **Source:** Penman (2013) Ch.5 pp.140-177.
- the firm has unusual continuing-value characteristics (cyclically
  variable ROE, unsustainable competitive position, expected fade
  to cost of equity) that complicate the standard Gordon-growth
  continuing-value formula. **Source:** Penman (2013) Ch.5
  pp.140-177.
