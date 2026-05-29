---
schema_version: "cacg.v0"
id: "fra-earnings-based-valuation"
title: "Earnings-Based Valuation"
reading_id: "03_financial_reporting_analysis"
summary: "Frames earnings-based valuation via the price-earnings ratio and abnormal earnings growth (AEG) — how an analyst values equity by capitalizing forward earnings and adding present value of expected abnormal earnings growth, in contrast with book-value-anchored residual-earnings models."
tags: ["financial-reporting", "earnings-based"]
citations:
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p205:0342"
    chunk_hash: "1d1629204992f480b67eca022a92d28731c7d27b78a8835cedfa1cb8189f4b62"
    page_range: [205, 206]
    quote: "By anchoring on earnings, the analyst develops the price-earnings ratio (P/E)"
    edge_type: "defines"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p212:0354"
    chunk_hash: "03ddc471f389067bdf7076faefb08da74be215d65b5fee0c17afd2d7eace2180"
    page_range: [212, 213]
    quote: "Abnormal earnings growth is the metric that captures the extra value, so the value of the equity for a going concern is Value of equity = Capitalized forward earnings"
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2298:3368"
    chunk_hash: "4fe667b7c5b2fd551c40670359b1297fbc795f1f8d1cf66e261b3a7c59306628"
    page_range: [2298, 2299]
    quote: "Analysts gather and process information to make investment decisions, including buy and sell recommendations"
    edge_type: "supports"
card_hash: "fbcbde6ecf94bbae0e9f365633c8b2c5bd9b8db55190f31cd5bd0d173d61f5b6"
---
# Earnings-Based Valuation

## Intuition

Earnings-based valuation values the firm directly from its
forecasted accounting earnings stream. The simplest form is the
capitalized-earnings model: a perpetual stream of constant earnings
discounted at the cost of equity gives a value equal to earnings
divided by the cost of equity. The firm's value-to-earnings
multiple in this base case is the inverse of the cost of equity —
the "earnings yield equals required return" relationship. **Source:**
Penman (2013) Ch.6 pp.178-209.

The capitalized-earnings base case is often inadequate because real
firms' earnings grow over time. The abnormal-earnings-growth (AEG)
framework extends the base case by adding the present value of
forecast earnings GROWTH that exceeds what the firm would generate
by simply reinvesting at the cost of equity. AEG is the
earnings-growth analogue of residual earnings: residual earnings
measure the period-by-period return above cost of equity on book
value; AEG measures the period-by-period earnings growth above the
cost-of-equity-driven earnings growth. **Source:** Penman (2013)
Ch.6 pp.178-209.

```
+--------------------------------------------+
| Earnings-Based Valuation Family            |
+--------------------------------------------+
|  Capitalized Earnings (no growth)          |
|     V = NI / r                             |
|     P/E = 1 / r                            |
|       |                                    |
|       v                                    |
|  Add Growth (AEG framework)                |
|     V = NI/r + PV(AEG)                     |
|     where AEG_t = (NI_t - retention)       |
|       grown above the no-growth path       |
|       |                                    |
|       v                                    |
|  Justified P/E = 1/r + (PV growth) / NI    |
|                                            |
|  Higher growth => higher justified P/E     |
|  Higher cost of equity => lower P/E        |
+--------------------------------------------+
```

The diagram orders the family from the no-growth capitalized-
earnings base case to the AEG-augmented form. The justified P/E
is therefore the sum of the no-growth multiple plus a growth
premium; cross-firm differences in P/E reflect differences in
expected earnings growth and in cost of equity. **Source:** Penman
(2013) Ch.6 pp.178-209.

## Definition

Capitalized-earnings valuation models the firm's intrinsic value
as the present value of a perpetual constant earnings stream
discounted at the cost of equity. For a firm whose forward
earnings is `NI_1`, the capitalized-earnings value is `V_0 =
NI_1 / r` where `r` is the cost of equity. The corresponding
justified earnings multiple is `V/NI = 1/r`. The model assumes the
firm earns the same amount each future period, retains nothing,
and pays everything as dividends. **Source:** Penman (2013) Ch.6
pp.178-209.

The abnormal-earnings-growth (AEG) framework extends the
capitalized-earnings model to allow earnings growth. Define
abnormal earnings growth at period `t` as the excess of period-`t`
earnings (plus reinvested-dividend earnings on prior-period
dividends) over the cost-of-equity-driven path: `AEG_t = NI_t +
r × Div_{t-1} − (1 + r) × NI_{t-1}`. The AEG-extended valuation
adds the present value of the AEG stream to the capitalized
earnings: `V_0 = NI_1 / r + PV(AEG)`. **Source:** Penman (2013)
Ch.6 pp.178-209.

The justified P/E multiple in the AEG framework is `P/E_justified
= 1/r + PV(AEG) / NI_1`. The first term is the no-growth multiple;
the second is a growth premium that reflects the firm's ability to
generate earnings growth above the cost of equity. A firm with no
expected AEG has `P/E_justified = 1/r` (the no-growth case); a
firm with positive expected AEG has `P/E_justified > 1/r`. **Source:**
Penman (2013) Ch.6 pp.178-209.

The framework's relationship to residual-earnings valuation is
exact: AEG-based valuation and residual-earnings valuation produce
the same intrinsic-value estimate when applied to the same forecast
inputs and the same cost of equity. The two frameworks are
algebraic restatements of each other; they emphasize different
forecast inputs (book value plus forecast residual earnings vs
forward earnings plus forecast earnings growth) but converge on
the same value. **Source:** Penman (2013) Ch.6 pp.178-209.

## Mathematical Reasoning

The capitalized-earnings model derives directly from the
discounted-cash-flow framework under a no-growth assumption. If
the firm distributes all earnings as dividends each period (`Div_t
= NI_t = NI_1` constant), the dividend stream is a perpetuity, and
the present value is `V_0 = NI_1 / r`. The implied P/E multiple is
the reciprocal: `P/E = 1/r`. This is the "earnings yield equals
required return" identity at zero growth. **Source:** Penman (2013)
Ch.6 pp.178-209.

The AEG framework's algebra extends the no-growth case by tracking
earnings growth that exceeds the cost-of-equity-driven baseline.
The baseline assumes any retained earnings are reinvested at the
cost of equity, so the period's baseline earnings equal the prior
period's earnings plus the cost-of-equity return on the prior
period's retained portion. Earnings that exceed the baseline are
abnormal earnings growth `AEG_t`, defined as the period's actual
earnings adjusted for the cost-of-equity return on prior-period
dividends, less the prior-period earnings grown at one plus the
cost of equity. The definition isolates the per-period earnings
growth that exceeds what mechanical reinvestment-at-cost-of-equity
would produce. **Source:** Penman (2013) Ch.6 pp.178-209.

Adding the discounted AEG stream to the capitalized earnings yields
the AEG valuation: `V_0 = NI_1 / r + PV(AEG_2, AEG_3, ...)`. The
first term captures the value of the no-growth perpetuity; the
second captures the value of all periods' earnings growth above
the reinvestment-at-cost-of-equity baseline. The model collapses
to the capitalized-earnings model when AEG is identically zero.
**Source:** Penman (2013) Ch.6 pp.178-209.

The AEG = 0 condition is informative: a firm earning exactly the
cost of equity on its retained earnings has zero AEG and trades
at the no-growth P/E. A firm earning more than the cost of equity
on retained earnings has positive AEG and trades at a premium; a
firm earning less has negative AEG and trades at a discount. The
condition aligns with the residual-earnings framework's identity:
positive expected AEG corresponds to positive expected residual
earnings, and the two valuations agree on the value premium.
**Source:** Penman (2013) Ch.6 pp.178-209.

The accounting-policy invariance property carries over from
residual-earnings valuation. Both AEG and residual-earnings
valuation are robust to accounting-policy choice because the same
choice that increases reported NI also increases the cost-of-
equity charge or the baseline growth path; the two effects offset.
The intrinsic-value estimate is approximately invariant.
**Source:** Penman (2013) Ch.6 pp.178-209.

The AEG framework relates to multiples-based valuation: the
justified P/E identity above grounds the multiple in forecast
earnings growth, providing a structural alternative to the
market-implied P/E that 05 Equity's `eq-pe-and-relative-valuation`
treats. The Penman-anchored AEG framework derives the multiple from
forecast inputs; the Damodaran-anchored multiples framework reads
the multiple off market data. The two presentations should
converge when the analyst's forecast inputs match the market's
implied forecast. **Source:** Penman (2013) Ch.6 pp.178-209.

The CFA L1 framing covers the no-growth capitalized-earnings model
and the Gordon-growth model (an alternative growth-model variant
with a single perpetual growth rate). The AEG extension generalizes
the Gordon-growth case to allow period-by-period growth that
varies. The curriculum's treatment of justified P/E aligns with
Penman's no-growth-plus-growth-premium decomposition. **Source:**
CFA L1 Curriculum (2022) Vol.4/pp.361-416.

## See Also

- [`fra-residual-earnings-valuation`](./fra-residual-earnings-valuation.md) — AEG and residual-earnings produce the same intrinsic value; the two frameworks are algebraic restatements
- [`fra-ratio-decomposition-dupont`](./fra-ratio-decomposition-dupont.md) — DuPont's ROE feeds the residual-earnings forecast that AEG also relies on
- [`fra-earnings-quality-and-sustainability`](./fra-earnings-quality-and-sustainability.md) — sustainable earnings is the input to both AEG and residual-earnings valuation
- [`eq-pe-and-relative-valuation`](../05_equity/eq-pe-and-relative-valuation.md) — accounting-first vs market-data-first framing differential: this card derives the justified P/E from Penman's forecast-earnings AEG framework, whereas the 05 P/E card anchors on Damodaran's market-implied multiples treatment
- [`eq-dividend-discount-models`](../05_equity/eq-dividend-discount-models.md) — the DDM framework is the cash-distribution analogue of the AEG framework

## Escalate to Raw When

Open Penman Ch.6 directly when any of the criteria below applies.
**Source:** Penman (2013) Ch.6 pp.178-209.

- the firm has unusual retention / payout dynamics that make the
  AEG decomposition non-trivial (high-growth retention, dividend-
  paying mature firm with capex catch-up) and the analyst needs
  Penman's discussion of growth-vs-distribution tradeoffs.
  **Source:** Penman (2013) Ch.6 pp.178-209.
- the analyst is reconciling AEG and residual-earnings valuations
  for the same firm and finding they diverge — Penman's treatment
  identifies the typical forecast-input inconsistency. **Source:**
  Penman (2013) Ch.6 pp.178-209.
- the firm has a non-constant cost of equity over the forecast
  horizon (cyclically variable beta, changing leverage) and the
  analyst needs the curriculum-level treatment for time-varying-r
  AEG valuation. **Source:** Penman (2013) Ch.6 pp.178-209.
