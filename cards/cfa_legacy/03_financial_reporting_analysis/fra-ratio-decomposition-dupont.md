---
schema_version: "cacg.v0"
id: "fra-ratio-decomposition-dupont"
title: "Ratio Decomposition (DuPont)"
reading_id: "03_financial_reporting_analysis"
summary: "Framing the 3-step DuPont identity (ROE = Net Margin × Asset Turnover × Financial Leverage) and the 5-step extension that splits net margin into tax burden × interest burden × EBIT margin. Sector profiles diverge sharply on which decomposition component carries the ROE; sustainability differs by lever."
tags: ["financial-reporting", "ratio-decomposition"]
citations:
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p401:0650"
    chunk_hash: "f517df4d617b571337fba8fc1d8d367fcd786e4feaf3dc6eb5b8203476036691"
    page_range: [401, 401]
    quote: "This decomposition of operating profitability is known as the DuPont model."
    edge_type: "defines"
  - source_id: "fra_penman_fsa_security_valuation_5ed"
    chunk_id: "fra_penman_fsa_security_valuation_5ed:p394:0639"
    chunk_hash: "bc342034f3f03a8956522e15d786ebc37da053a44ea08b2cd453932dc2a404c2"
    page_range: [394, 395]
    quote: "Just as financial liabilities can lever up the ROCE, so can operating liabilities lever up the return on net operating assets."
    edge_type: "supports"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p1407:2045"
    chunk_hash: "766934ee8c8e58dbba369eb55e7f9d931e721865e7d0fefe6245bc086e7db0ea"
    page_range: [1407, 1408]
    quote: "DuPont analysis shows the relationship between the various categories of ratios discussed in this reading and how they all influence the return to the investment of the owners."
    edge_type: "supports"
card_hash: "33fcee439ec07ad3ab3dbe41d3ddfe738dde702e765187775fcbed392599c2a6"
---
# Ratio Decomposition (DuPont)

## Intuition

Return on equity (ROE) is the headline measure of how well the
firm's equity capital is producing earnings. But ROE is a
composite — three different operating and financial-structure
levers each contribute to it, and two firms with the same ROE can
get there through completely different combinations of margin,
turnover, and leverage. The DuPont decomposition makes those
contributions explicit, so the analyst can see whether a firm's
ROE is driven by operating efficiency, by asset productivity, or
by financial leverage. **Source:** Penman (2013) Ch.12 pp.364-391.

The three-step DuPont decomposition reads `ROE = NetMargin ×
AssetTurnover × FinancialLeverage`. Each factor isolates one
business lever: net margin captures profitability per dollar of
sales; asset turnover captures sales generated per dollar of
assets; financial leverage captures assets supported per dollar of
equity. The five-step extension goes further, splitting net margin
into tax burden × interest burden × EBIT margin, which separates
operating margin from the financing and tax structure that
determines what survives to net income. **Source:** Penman (2013)
Ch.12 pp.364-391.

```
<!-- primitive: ratio-decomposition-tree source: _diagram_primitives.md -->
ROE = Net Income / Equity
 |
 +-- 3-step DuPont:
 |
 |   ROE = (NI / Sales) x (Sales / Assets) x (Assets / Equity)
 |        ----------    ----------------    ----------------
 |        Net Margin     Asset Turnover     Leverage
 |
 +-- 5-step extension:
 |
 |   Net Margin = (NI / EBT) x (EBT / EBIT) x (EBIT / Sales)
 |               ----------    -----------    -------------
 |               Tax Burden    Int. Burden    EBIT Margin
 |
 |   ROE = TaxBur x IntBur x EBITMargin x Turnover x Leverage
 |
 +-- Cross-section interpretation:
        - High Margin sectors: tech, pharma
        - High Turnover sectors: retail, distribution
        - High Leverage sectors: banks, REITs
```

The decomposition tree shows the 3-step and 5-step forms with
each factor's algebraic identity. The cross-section interpretation
at the bottom names sectors that typically score high on each
component — different industries reach respectable ROE through
different mixes. **Source:** Penman (2013) Ch.12 pp.364-391.

## Definition

The DuPont decomposition is the multiplicative breakdown of return
on equity (ROE) into a chain of intermediate ratios that each
isolate one driver of profitability or capital structure. The
decomposition is identity-level (it must hold by algebraic
construction); it is not an empirical model but a way of reading
the firm's reported ROE through interpretable components.
**Source:** Penman (2013) Ch.12 pp.364-391.

The 3-step form starts from `ROE = NI / Equity`. Multiply and
divide by Sales: `ROE = (NI / Sales) × (Sales / Equity)`. Then
multiply and divide by Assets: `ROE = (NI / Sales) × (Sales /
Assets) × (Assets / Equity)`. The three resulting factors are net
margin, asset turnover, and financial leverage. The decomposition
is identity-preserving: the right side equals ROE for any firm
with non-zero sales, assets, and equity. **Source:** Penman (2013)
Ch.12 pp.364-391.

The 5-step extension takes the net margin factor and decomposes it
further: `NI / Sales = (NI / EBT) × (EBT / EBIT) × (EBIT / Sales)`.
The three sub-factors are tax burden (the share of pretax income
that survives tax), interest burden (the share of operating income
that survives interest), and EBIT margin (the operating margin).
Combining: `ROE = TaxBurden × InterestBurden × EBITMargin ×
AssetTurnover × FinancialLeverage`. **Source:** Penman (2013) Ch.12
pp.364-391.

The five components map cleanly to operational levers. Tax burden
reflects the firm's effective tax rate (lower tax burden means
more pretax income survives — typically lower jurisdictional rates
or larger tax credits). Interest burden reflects the firm's
financing cost relative to operating profit (lower interest burden
means more EBIT survives interest expense — typically less debt
or lower-cost debt). EBIT margin is operating efficiency. Asset
turnover is asset productivity. Financial leverage is capital
structure. Each component answers a different management or
analyst question. **Source:** Penman (2013) Ch.12 pp.364-391.

## Mathematical Reasoning

The DuPont identities are algebraic. The 3-step form is verified by
multiplying out the right side: `(NI / Sales) × (Sales / Assets)
× (Assets / Equity) = NI / Equity = ROE` (the Sales and Assets
terms cancel). The 5-step form is verified the same way: `(NI /
EBT) × (EBT / EBIT) × (EBIT / Sales) = NI / Sales`. Both are
identity-level decompositions; they hold for any firm-period for
which the denominators are non-zero. **Source:** Penman (2013)
Ch.12 pp.364-391.

The decomposition's value lies in cross-sectional and time-series
interpretation. Two firms with identical ROE may have completely
different decomposition profiles. A margin-driven firm pairs a high
net margin with low asset turnover and substantial financial
leverage — typical of high-margin industries with relatively
asset-heavy operations and moderate-to-high debt funding. A
turnover-driven firm pairs a thin net margin with very high asset
turnover and modest leverage — typical of low-margin /
high-velocity industries such as retail and distribution. The
identity allows both profiles to yield the same ROE, but the
sustainability of each ROE is different: the margin-driven firm's
ROE depends on pricing power and operating efficiency; the
turnover-driven firm's ROE depends on volume velocity. The analyst
reads which lever a firm's ROE leans on to assess that
sustainability. **Source:** Penman (2013) Ch.12 pp.364-391.

The 5-step extension separates operating profit from
financial-structure choices. A firm whose ROE has been climbing
solely because tax burden has been falling (lower effective tax
rate) is in a different position from one whose EBIT margin has
been climbing — the first depends on tax-policy stability; the
second reflects operating improvement. Similarly, ROE growth
driven by financial leverage is debt-financed and reverses if the
firm deleverages or if rates rise; ROE growth driven by operating
margin is sustainable. The decomposition surfaces these distinctions
without requiring an explicit forecast model. **Source:** Penman
(2013) Ch.12 pp.364-391.

The CFA L1 framing of DuPont is consistent with Penman's analytical
treatment: the curriculum covers the 3-step and 5-step forms with
the same identity proofs and the same cross-component
interpretation, framed as a standard ratio-analysis tool within
the broader analytical-techniques toolkit. **Source:** CFA L1
Curriculum (2022) Vol.3/pp.175-252.

The directional check the analyst should run on every DuPont
reading is consistency between the cross-component story and the
firm's operating narrative. If the firm reports rising ROE and the
decomposition attributes it to leverage growth, but management
narrative claims the firm is deleveraging, one of the readings is
wrong. The DuPont identity exposes the inconsistency by
construction. **Source:** Penman (2013) Ch.12 pp.364-391.

## See Also

- [`fra-financial-analysis-techniques`](./fra-financial-analysis-techniques.md) — DuPont is a specific decomposition within the broader profitability-ratio family
- [`fra-income-statement-foundations`](./fra-income-statement-foundations.md) — the income-statement subtotals (revenue, EBIT, EBT, NI) feed every DuPont factor

## Escalate to Raw When

Open Penman Ch.12 directly when any of the criteria below applies.
**Source:** Penman (2013) Ch.12 pp.364-391.

- the analyst is comparing peer firms whose 3-step DuPont profiles
  are similar but whose 5-step extensions diverge sharply on tax
  or interest burden — Penman's discussion of the operating-vs-
  financing decomposition supports the diagnostic. **Source:**
  Penman (2013) Ch.12 pp.364-391.
- the firm's ROE has shifted markedly across periods and the
  analyst needs Penman's framework for attributing the change to
  specific factor movements rather than aggregate causes.
  **Source:** Penman (2013) Ch.12 pp.364-391.
- the analyst is constructing a sustainable-ROE forecast and
  needs Penman's decomposition framework to identify which
  components are policy-stable vs cyclically variable.
  **Source:** Penman (2013) Ch.12 pp.364-391.
