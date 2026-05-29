---
schema_version: "cacg.v0"
id: "fra-credit-risk-from-accounting"
title: "Credit Risk from Accounting"
reading_id: "03_financial_reporting_analysis"
summary: "Frames credit risk from accounting indicators — leverage, interest coverage, cash-flow generation relative to debt service, and pro-forma default-scenario discipline built from balance-sheet and income-statement data; complements market-data-driven credit-spread estimates."
tags: ["financial-reporting", "credit-risk"]
citations:
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p706:1137"
    chunk_hash: "6788428bf806000758ff43231703e93724877d67b9998ff20e36e6eb4e462ac3"
    page_range: [706, 707]
    quote: "Most of the analysis in the book to this point has been concerned with the valuation of the firm and the valuation of the equity claim on the firm"
    edge_type: "defines"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p709:1141"
    chunk_hash: "feaace03576428cccd79d1d4b0a39e1652d244beec542f64309a214639436358"
    page_range: [709, 710]
    quote: "Credit analysis calls for a different analysis, and many of the ratios involved are different from those for equity analysis"
    edge_type: "defines"
card_hash: "25f7fe95055107b4e7cf4677604efd7337439baec64aa7d3059a40e0f0ce84da"
---
# Credit Risk from Accounting

## Intuition

Credit risk is the risk that the firm fails to make a contractually
required debt payment. The probability that this happens — the
default probability — is what credit analysts try to estimate.
Financial-statement data carries strong signal about default
probability because default fundamentally happens when cash flow
runs short of debt service for long enough. Accounting indicators
that bear on credit risk are leverage (how much debt the firm has
to service), coverage (how much earnings or cash flow is available
to service it), liquidity (how much short-term cushion exists),
and earnings stability (how predictable the available cash flow
is). The credit analyst combines these indicators into an overall
default-probability estimate. **Source:** Penman (2013) Ch.20
pp.680-708.

The accounting-based credit indicators complement the market-data-
based credit-spread signals that the existing 06 fixed-income cards
treat. Market spreads incorporate the market's default-probability
estimate plus a risk premium for default-risk-bearing capacity;
accounting indicators provide an independent estimate that can
either confirm or contradict the market signal. When the market
spread implies higher default probability than the accounting
indicators suggest, the analyst examines whether the market is
pricing tail risk that the accounting numbers miss (legal
contingencies, off-balance-sheet exposures, fraud concerns).
**Source:** Penman (2013) Ch.20 pp.680-708.

```
+--------------------------------------------+
| Credit Risk Indicator Families             |
+--------------------------------------------+
|  Leverage:                                 |
|     - Debt / Equity                        |
|     - Debt / Total Assets                  |
|     - Debt / EBITDA                        |
+--------------------------------------------+
|  Coverage:                                 |
|     - EBIT / Interest Expense              |
|     - CFO / Interest Paid                  |
|     - CFO / Total Debt Service             |
+--------------------------------------------+
|  Liquidity:                                |
|     - Current Ratio                        |
|     - Quick Ratio                          |
|     - Cash / Current Liabilities           |
+--------------------------------------------+
|  Earnings stability:                       |
|     - Volatility of EBIT                   |
|     - Margin compression history           |
|     - Earnings-quality flags               |
+--------------------------------------------+
|       |                                    |
|       v                                    |
|  Combine into default-probability proxy    |
|  Compare to market-implied spread          |
+--------------------------------------------+
```

The diagram organizes accounting-based credit indicators into four
families: leverage (debt magnitude relative to equity, assets, or
earnings), coverage (cash-flow availability relative to required
debt service), liquidity (short-term cushion), and earnings
stability (predictability of the available cash flow). The four
families together produce the analyst's accounting-based credit
view. **Source:** Penman (2013) Ch.20 pp.680-708.

## Definition

Credit risk for a firm is the probability that the firm fails to
meet a contractually required debt payment in full and on time.
The probability is a function of the firm's debt burden, its
ability to service that debt from operating cash flow, its
short-term liquidity cushion, and the predictability of its cash
flow generation. **Source:** Penman (2013) Ch.20 pp.680-708.

The four credit-indicator families partition the analyst's
accounting toolkit for credit assessment. **Source:** Penman (2013)
Ch.20 pp.680-708.

- Leverage indicators measure the firm's debt magnitude relative
  to its equity or earnings base. The standard ratios are
  debt-to-equity (`Total Debt / Total Equity`), debt-to-assets
  (`Total Debt / Total Assets`), and debt-to-EBITDA (`Total Debt /
  EBITDA`). High leverage means the firm has more debt to service
  per unit of operating capacity. **Source:** Penman (2013) Ch.20
  pp.680-708.
- Coverage indicators measure the cash-flow or earnings
  availability relative to required debt service. The standard
  ratios are interest coverage (`EBIT / Interest Expense`),
  fixed-charge coverage (incorporating lease payments and other
  fixed charges), and cash-flow coverage (`CFO / Interest Paid`
  or `CFO / Total Debt Service`). Cash-flow-based coverage is
  more diagnostic for credit purposes than earnings-based coverage
  because cash actually services debt. **Source:** Penman (2013)
  Ch.20 pp.680-708.
- Liquidity indicators measure the firm's short-term cushion to
  meet immediate obligations: current ratio, quick ratio, cash
  ratio. A firm with adequate liquidity can survive a temporary
  cash-flow shortfall; an illiquid firm cannot. **Source:** Penman
  (2013) Ch.20 pp.680-708.
- Earnings-stability indicators measure the volatility and
  predictability of the firm's cash-flow generation: historical
  EBIT volatility, margin-compression episodes, earnings-quality
  flags from the accrual-anomaly framework. Volatile earnings
  signal that even adequate average coverage may not translate to
  reliable debt-service capacity. **Source:** Penman (2013) Ch.20
  pp.680-708.

The standard credit-rating-agency methodologies use ratio grids
that map specific levels of these accounting indicators to
indicative credit ratings. While the credit analyst rarely
replicates the full agency methodology, the underlying logic is
the same: a firm whose accounting indicators sit deep in the
investment-grade range is unlikely to default near-term; a firm
sitting deep in the speculative-grade range has a meaningfully
elevated default probability. **Source:** Penman (2013) Ch.20
pp.680-708.

## Mathematical Reasoning

Penman frames default-probability estimation as a pro-forma
exercise: the analyst constructs a pro-forma scenario of the
firm's near-term cash availability and compares it to the firm's
contractually required debt service. The default event is the
shortfall: `Pr{default} = Pr{Cash available for debt service <
Debt service requirement}` over the relevant horizon. The four
accounting-indicator families are inputs to the pro-forma
construction: leverage sets the size of the debt-service
requirement; coverage measures the average cushion of cash relative
to that requirement; liquidity sets the short-term reserve that
can absorb a temporary shortfall; earnings stability shapes the
distribution of cash available. **Source:** Penman (2013) Ch.20
pp.680-708.

The coverage ratios feed the pro-forma directly. Interest coverage
`EBIT / Interest` measures the ratio of current earnings to the
interest obligation; cash-flow coverage `CFO / Interest Paid` does
the same for cash. The analyst's pro-forma applies stress
adjustments to the numerator (earnings shock, margin compression,
revenue disruption) and re-evaluates the coverage under stress; a
firm whose stressed coverage remains adequate is a stronger credit
than a firm whose stressed coverage falls below the threshold of
ability-to-pay, even if the unstressed coverage is similar.
**Source:** Penman (2013) Ch.20 pp.680-708.

The combination of leverage and coverage is more diagnostic than
either alone. A firm with moderate leverage and ample coverage is
typically investment-grade; a firm with high leverage and tight
coverage is typically speculative-grade; the in-between cases
require the analyst's judgment, often informed by industry
benchmarks and the firm's earnings-stability profile. The credit-
analyst's discipline is to read multiple indicators together rather
than mechanically applying any single threshold. **Source:** Penman
(2013) Ch.20 pp.680-708.

The earnings-stability indicator is especially important because
volatile earnings can produce coverage ratios that look adequate
on average but inadequate in stress periods. Average coverage can
hide stress-period weakness: a firm whose adequate average masks
historical stress periods of inadequate coverage is a riskier
credit than a firm whose coverage has been steady at the same
average. The earnings-quality framework from the previous analysis
card provides the persistence diagnostic that distinguishes these
cases. **Source:** Penman (2013) Ch.13 pp.392-433.

The contingent-liability adjustment from the non-current-liabilities
card surfaces explicitly in credit assessment. Recognized
provisions on the balance sheet contribute directly to total
liabilities and to leverage ratios. Disclosed but non-recognized
contingencies (probable but not reliably measurable, or possible
but not probable) sit outside the leverage ratios but represent
potential future cash outflows that the credit analyst should
incorporate into stress-tested liquidity analysis. The accounting-
based credit framework therefore extends beyond the headline
balance-sheet leverage to include disclosed contingencies. **Source:**
Penman (2013) Ch.20 pp.680-708.

The accounting-vs-market-data complementarity is similar to the
equity-risk case: market-data credit spreads provide an
independent estimate of default probability that should align with
the accounting-based view. When the two diverge, the analyst
examines whether the market is pricing information the accounting
numbers miss (legal exposure, fraud concerns, business-model
disruption) or whether the market is mispricing the credit (in
which case there is a potential trade in the firm's debt). **Source:**
Penman (2013) Ch.20 pp.680-708.

## See Also

- [`fra-non-current-liabilities`](./fra-non-current-liabilities.md) — non-current debt and contingent liabilities are the primary inputs to leverage and stress-tested liquidity analysis
- [`fra-cash-flow-statement-mechanics`](./fra-cash-flow-statement-mechanics.md) — CFO is the input to cash-flow-based coverage ratios that drive credit assessment
- [`fra-earnings-quality-and-sustainability`](./fra-earnings-quality-and-sustainability.md) — earnings-quality flags inform whether accounting coverage is reliable
- [`fra-financial-analysis-techniques`](./fra-financial-analysis-techniques.md) — the broader ratio-analysis toolkit that the credit-indicator families subset

## Escalate to Raw When

Open Penman Ch.20 directly when any of the criteria below applies.
**Source:** Penman (2013) Ch.20 pp.680-708.

- the firm has unusual debt structure (covenants, secured / unsecured
  layers, callable / convertible features) and the analyst needs
  the curriculum-level treatment for incorporating these features
  into the accounting-based credit view. **Source:** Penman (2013)
  Ch.20 pp.680-708.
- the firm is in distress or near-distress and the standard ratios
  no longer produce stable signals — Penman's treatment of distressed-
  firm credit indicators is the canonical reference. **Source:**
  Penman (2013) Ch.20 pp.680-708.
- the firm operates in an industry with significant off-balance-
  sheet credit exposure (financial services, insurance, utilities)
  and the analyst needs Penman's discussion of contingent-claim
  incorporation into the credit framework. **Source:** Penman (2013)
  Ch.20 pp.680-708.
