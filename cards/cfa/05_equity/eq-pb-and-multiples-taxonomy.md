---
schema_version: "cacg.v0"
id: "eq-pb-and-multiples-taxonomy"
title: "P/B and Multiples Taxonomy"
reading_id: "05_equity"
summary: "Framing the equity-multiples taxonomy beyond P/E — P/B (book value), EV/EBITDA (enterprise value), P/Sales (revenue) — and the equity-vs-enterprise-value distinction that determines which multiple is appropriate. P/B is most informative for asset-heavy firms (banks, insurers); EV/EBITDA suits cross-capital-structure comparisons."
tags: ["equity", "pb-multiples"]
citations:
  - source_id: "eq_damodaran_2025_investment_valuation_4ed"
    chunk_id: "eq_damodaran_2025_investment_valuation_4ed:p690:0897"
    chunk_hash: "ee01be57ae3e0fdd5d49d093d5e57004313f7e3910a1a2dc4781183b44a86f23"
    page_range: [690, 691]
    quote: "Stocks selling for well below the book value of equity have generally been considered undervalued, while those selling for more than book value have been targeted as overvalued."
    edge_type: "defines"
  - source_id: "cfa_2022_l1_combined"
    chunk_id: "cfa_2022_l1_combined:p2301:3372"
    chunk_hash: "c05526a2cfb42b95a4be11d73b69a3c1b4b702b832ad71528035c6be66267b53"
    page_range: [2301, 2302]
    quote: "These models are based chiefly on share price multiples or enterprise value multiples."
    edge_type: "supports"
card_hash: "ff6847b67477e31d198a7456e44da4a8b2327ec11b580db2d082bb1d0e2624de"
---
# P/B and Multiples Taxonomy

## Intuition

Multiples standardize price by an accounting or operating metric so
that prices of differently-sized firms become comparable. P/E
standardizes by earnings (the equity claim's bottom-line cash flow);
P/B standardizes by book value (an accounting measure of accumulated
equity); EV/EBITDA standardizes the firm's total enterprise value by
operating cash flow before non-cash charges; P/Sales standardizes by
revenue. Each multiple emphasizes a different dimension of the firm's
fundamentals. **Source:** Damodaran (2012) Ch.18 pp.635-689.

The choice of multiple matters because different multiples carry
different sensitivities and apply to different firm types. A loss-
making firm has no meaningful trailing P/E (the denominator is zero
or negative); EV/EBITDA or P/Sales remains positive and continues to
discriminate. A bank's value derives from book equity in a way that
makes P/B more interpretable than P/E. Damodaran's taxonomy is a
choice ladder for picking the multiple that the firm's fundamentals
support. **Source:** Damodaran (2012) Ch.19 pp.690-725.

```
multiples taxonomy

  equity-claim       enterprise-value         revenue-based
  multiples          multiples                multiples
  +--------+         +-----------+            +---------+
  |  P/E   |         | EV/EBITDA |            | P/Sales |
  |  P/B   |         | EV/Sales  |            | EV/Sales|
  +--------+         +-----------+            +---------+
       |                  |                        |
       v                  v                        v
   prices the        prices the firm's         prices the
   equity claim      operating assets          firm's revenue
   (after debt)      (before debt-equity      generation power
                      capital structure)
```

## Definition

The price-to-book ratio is the share price divided by book value per
share. Book value per share is the accounting equity claim per share
— assets minus liabilities, divided by share count. P/B is most
informative for firms whose value is closely tied to balance-sheet
assets (banks, insurers, asset-heavy industrials) and least
informative for service or intangible-heavy firms whose book value
under-measures economic equity. **Source:** Damodaran (2012) Ch.19
pp.690-725.

The enterprise-value-to-EBITDA multiple takes the firm's enterprise
value (market value of equity plus market value of debt minus cash)
in the numerator and earnings before interest, taxes, depreciation,
and amortization (EBITDA) in the denominator. EV/EBITDA prices the
firm's operating-cash generation independent of capital structure —
useful for cross-firm comparisons when leverage differs widely or
when the analyst needs a capital-structure-neutral operating-asset
metric. **Source:** Damodaran (2012) Ch.18 pp.635-689.

The price-to-sales ratio uses the share price divided by revenue
per share. P/Sales survives losses and accounting variability (the
revenue line is harder to manipulate than earnings); it is widely
applied to early-stage firms, firms in transition, and revenue-
intensive industries. EV/Sales is the enterprise-value-numerator
analog. **Source:** Damodaran (2012) Ch.20 pp.726-770.

The equity-vs-enterprise-value distinction is structural: equity
multiples (P/E, P/B) price the residual claim on the firm AFTER debt
is serviced; enterprise-value multiples (EV/EBITDA, EV/Sales) price
the operating assets BEFORE the debt-equity split. The two families
agree on stable comparison only when capital structures are similar
across the peer set. **Source:** Damodaran (2012) Ch.18 pp.635-689.

The CFA L1 frame presents the multiples-taxonomy as the L1-canonical
relative-valuation toolkit and emphasizes P/B's strength for asset-
intensive firms and EV/EBITDA's strength for cross-capital-structure
comparisons. **Source:** CFA L1 Curriculum (2022) Vol.4/pp.361-416.

## Mathematical Reasoning

The justified P/B follows from a residual-income perspective on
equity: the multiple a firm should trade at relative to book is the
expected return on equity divided by the cost of equity, modulated
by sustainable growth. The closed form (analogous to justified P/E)
is `(P_0 / B_0)_justified = (ROE - g) / (r - g)`. Higher ROE relative
to cost of equity pushes the justified multiple above one; ROE equal
to cost of equity gives a justified multiple of one (firm's equity
trades at book). **Source:** Damodaran (2012) Ch.19 pp.690-725.

The justified EV/EBITDA, derived from a free-cash-flow-to-firm
perspective, depends on the firm's reinvestment rate, tax rate,
WACC, and stable growth. The qualitative shape is parallel to
justified P/E: higher growth, lower WACC, and lower reinvestment
all push the multiple up. The exact formula is more involved than
P/E because EBITDA includes non-cash charges and ignores capital
structure. **Source:** Damodaran (2012) Ch.18 pp.635-689.

The justified forward P/Sales for a stable-growth firm is `(P_0 /
Sales_1)_justified = (margin · payout) / (r - g)`, where `Sales_1`
is next-period expected sales, `margin` is net margin, and `payout`
is the dividend payout ratio. The forward-sales form keeps the
algebra clean and parallels the justified-P/E formulation that uses
`E_1`. (For a trailing-sales statement the right-hand side picks up
a `(1 + g)` growth factor: `(P_0 / Sales_0)_justified = margin ·
payout · (1 + g) / (r - g)`.) The expression makes operating margin
a first-order driver of justified P/Sales — a high-margin firm with
a moderate payout commands a higher P/Sales multiple than a low-
margin firm at the same growth and risk. **Source:** Damodaran
(2012) Ch.20 pp.726-770.

Tobin's Q ratio compares market value of assets to replacement cost
of assets. When Q is greater than one, the market values the firm's
assets above the cost of replicating them — a signal that the firm
generates economic rents above its cost of capital. Damodaran treats
Q as a P/B variant when book value approximates replacement cost.
**Source:** Damodaran (2012) Ch.19 pp.690-725.

## See Also

- [`eq-pe-and-relative-valuation`](./eq-pe-and-relative-valuation.md) — P/E as the canonical equity multiple and the justified-P/E foundation
- [`eq-intrinsic-value`](./eq-intrinsic-value.md) — the intrinsic-valuation frame that grounds the multiples derivations
- [`eq-dividend-discount-models`](./eq-dividend-discount-models.md) — Gordon-DDM payout-and-growth structure that drives justified-P/B and justified-P/Sales

## Escalate to Raw When

Open Damodaran Ch.18-20 directly when any of the criteria below
applies. **Source:** Damodaran (2012) Ch.18 pp.635-689.

- the target is a financial-service firm where standard P/B or P/E formulas need bank-specific adjustments — Damodaran Ch.21 (Valuing Financial Service Firms) supplies the sector-specific framework. **Source:** Damodaran (2012) Ch.19 pp.690-725.
- the target is a money-losing or early-stage firm where revenue-based multiples are the only viable choice — Damodaran Ch.20 (Revenue Multiples) and Ch.22 (Money-Losing Firms) develop the application. **Source:** Damodaran (2012) Ch.20 pp.726-770.
- a sector-specific multiple is required (e.g., EV/Subscribers for telecoms, EV/Reserves for resource firms) — Damodaran Ch.20 develops the sector-specific multiples chapter. **Source:** Damodaran (2012) Ch.20 pp.726-770.
