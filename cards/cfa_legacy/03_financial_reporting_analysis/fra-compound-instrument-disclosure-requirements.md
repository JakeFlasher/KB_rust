---
schema_version: "cacg.v0"
id: "fra-compound-instrument-disclosure-requirements"
title: "IAS 32 / IFRS 7 Disclosure Baseline for Issuer Compound Instruments"
reading_id: "03_financial_reporting_analysis"
summary: "Frames the IFRS 7 disclosure baseline for issuer-side compound financial instruments — what must appear in financial-statement footnotes when an entity issues a convertible bond or other instrument carrying both a liability and an equity component; what each disclosure cell tells the analyst."
tags: ["financial-reporting", "compound-instrument"]
citations:
  - source_id: "fra_hkicpa_hkfrs_7_2018"
    chunk_id: "fra_hkicpa_hkfrs_7_2018:p013:0015"
    chunk_hash: "3f4a6b5a6c11f4c2b358eaad0af464181d52de3ed5af4ff5cde421565431ecf1"
    page_range: [13, 14]
    quote: "If an entity has issued an instrument that contains both a liability and an equity component (see paragraph 28 of HKAS 32) and the instrument has multiple embedded derivatives"
    edge_type: "defines"
  - source_id: "fra_hkicpa_hkas_32_2022"
    chunk_id: "fra_hkicpa_hkas_32_2022:p017:0026"
    chunk_hash: "ed474f1f085154e890b9da79d0dc2912b34da425ccd0daf26691b4dd989930c1"
    page_range: [17, 18]
    quote: "The issuer of a non-derivative financial instrument shall evaluate the terms of the financial instrument to determine whether it contains both a liability and an equity component"
    edge_type: "supports"
card_hash: "a8d78778d541656fe29a4da06455c3bd84e9babc26f3a5b2a1124c0480b83da9"
---
# IAS 32 / IFRS 7 Disclosure Baseline for Issuer Compound Instruments

## Intuition

A convertible bond on an issuer's balance sheet must surface
through a specific set of disclosure cells in the financial-
statement footnotes — not as a one-line "convertible bonds
outstanding" entry, but as a structured pattern spanning balance-
sheet categories, fair-value disclosures, income-recognition
analysis, and a dedicated multi-embedded-derivative existence
disclosure. The IFRS 7 baseline (HKFRS 7) defines the minimum
cell set; what an issuer's footnote includes, omits, or buries
becomes the analyst's reporting-quality diagnostic. **Source:**
HKICPA HKFRS 7 (2018) pp.14-15.

The convertible-specific anchor is HKFRS 7 paragraph 17:
"If an entity has issued an instrument that contains both a
liability and an equity component (see paragraph 28 of HKAS 32)
and the instrument has multiple embedded derivatives whose
values are interdependent (such as a callable convertible debt
instrument), it shall disclose the existence of those features."
This paragraph attaches a specific existence-disclosure
obligation to the case where the convertible has more than one
embedded option (e.g., a conversion option PLUS an issuer call
PLUS an investor put). The analyst's first read of any
convertible-bond footnote should be whether paragraph 17 has
been triggered and, if so, whether the issuer has explicitly
named the interdependent features. **Source:** HKICPA HKFRS 7
(2018) pp.14.

```
+----------------------------------------------------------+
|  Convertible-bond disclosure surface (HKFRS 7 baseline)  |
+----------------------------------------------------------+
|                                                          |
|  Cell 1: Categories carrying amounts (§8)                |
|     - liability component in financial liabilities at    |
|       amortized cost line                                |
|     - equity component in equity section line            |
|                                                          |
|  Cell 2: Multiple-embedded-derivative existence (§17)    |
|     - explicit naming of interdependent features         |
|       (conversion, call, put) when ≥2 present            |
|                                                          |
|  Cell 3: Income / expense / gains / losses (§20)         |
|     - interest expense on the liability component (using |
|       effective-interest method)                         |
|     - any gain or loss on early extinguishment           |
|                                                          |
|  Cell 4: Fair-value disclosures (§25 + §26 + §29)        |
|     - fair value of the liability component (§25)        |
|     - class-grouping discipline (§26)                    |
|     - exemptions when carrying ≈ fair value (§29)        |
+----------------------------------------------------------+
```

**Source:** HKICPA HKFRS 7 (2018) pp.8-9, pp.14-15, pp.21-22;
HKICPA HKAS 32 (2022) pp.17-18 (paragraphs 28-31 anchor the
equity-component separate-presentation requirement reflected in
Cell 1's "equity component in equity section line" diagram entry).

## Definition

The HKFRS 7 cell stack for compound instruments builds from
the foundation paragraphs into the convertible-specific overlay.
The foundation is HKFRS 7 paragraph 8: the carrying amounts of
each category of financial assets and financial liabilities must
be disclosed either in the statement of financial position or in
the notes, with the categories defined by reference to HKFRS 9
(amortized cost, fair value through other comprehensive income,
fair value through profit or loss). For a convertible bond
issuer following the IAS 32 path, the liability component appears
in the amortized-cost category line — its carrying amount `L_t`
at the reporting date — and the equity component appears
separately in the equity section at its constant value `E_0` per
the HKAS 32 paragraph 28-31 separate-presentation requirement
(HKFRS 7 paragraph 8 anchors only the financial-asset / financial-
liability category carrying amounts; the equity-component separate
presentation is a HKAS 32 obligation). **Source:** HKICPA HKFRS 7
(2018) pp.8-9; HKICPA HKAS 32 (2022) pp.17-18 (paragraphs 28-31
anchor the equity-component separate-presentation requirement
invoked in this paragraph).

Paragraph 17 then layers on the multiple-embedded-derivatives
existence disclosure when the instrument has more than one
interdependent embedded derivative. The disclosure is
qualitative — "shall disclose the existence of those features" —
but binding: a footnote that omits to name the call and the put
features alongside the conversion option for a callable-puttable
convertible is non-compliant. The analyst should treat the
presence of an explicit paragraph-17 disclosure as a
reporting-quality positive and its absence (when the instrument
clearly has multiple embedded options per its terms) as a
red flag. **Source:** HKICPA HKFRS 7 (2018) pp.14.

Paragraph 20 requires disclosure of items of income, expense,
gains, or losses, with sub-items including: total interest
income and total interest expense calculated using the
effective-interest method for financial assets and financial
liabilities measured at amortized cost; fee income and fee
expense for these items; and gains or losses recognized on the
derecognition of financial assets and financial liabilities. For
the convertible-bond issuer, this surfaces the period
`Interest_t = r_market · L_{t-1}` on the liability leg, plus
any gain or loss on early repurchase (covered in
[`fra-conversion-extinguishment-accounting`](./fra-conversion-extinguishment-accounting.md)).
**Source:** HKICPA HKFRS 7 (2018) pp.14-15.

Paragraph 25 requires fair-value disclosure for each class of
financial assets and financial liabilities, in a way that permits
comparison with the carrying amount. For a convertible issuer
that reports the liability component at amortized cost, the
analyst expects to find both the carrying amount `L_t` AND a
fair-value estimate of the liability component. The gap between
amortized-cost carrying amount and fair value is a credit-spread
and interest-rate sensitivity signal. Paragraph 26 prescribes
the class-grouping discipline (carrying amounts of financial
instruments shall be grouped into classes appropriate to the
nature of the information disclosed and that take into account
the characteristics of those financial instruments). Paragraph 29
sets out the exemptions where fair-value disclosure is not
required (e.g., when carrying amount approximates fair value for
short-term trade receivables and payables). **Source:** HKICPA
HKFRS 7 (2018) pp.21-22.

## Mathematical Reasoning

The analyst's reconstruction protocol exploits the disclosure
cells to recover the original split mechanics from the footnote
disclosures alone. The starting point is the equity-component
constant `E_0` reported in the equity section. The liability
component carrying amount `L_t` at reporting date is in the
amortized-cost-category disclosure under paragraph 8. The
effective-interest-method interest expense for the period is in
the paragraph-20 disclosure. From these three observable cells
the analyst can recover: the original issue proceeds
`P = L_0 + E_0` (when `L_0` is also disclosed or can be back-
calculated from amortization tables), the market rate
`r_market` (implied from `Interest_t / L_{t-1}`), and the
remaining unamortized discount or premium. **Source:** HKICPA
HKFRS 7 (2018) pp.8-9, pp.14-15; HKICPA HKAS 32 (2022) pp.17-18
(paragraphs 28-31 anchor the separate-presentation requirement
that locates the equity component `E_0` in the equity section).

```
+----------------------------------------------------------+
|  Analyst reconstruction protocol from HKFRS 7 footnotes  |
+----------------------------------------------------------+
|                                                          |
|  Observable from §8 disclosure:                          |
|     L_t       = liability carrying amount at reporting   |
|                  date (amortized-cost category line)     |
|                                                          |
|  Observable from equity section (separate presentation   |
|  per HKAS 32 paragraphs 28-31):                          |
|     E_0       = equity component (constant through life) |
|                                                          |
|  Observable from §20 disclosure:                         |
|     Interest_t = effective-interest expense on the       |
|                  liability for the reporting period      |
|                                                          |
|  Observable from §25 fair-value disclosure:              |
|     FV_t,L    = fair-value estimate of the liability     |
|                                                          |
|  Recoverable by computation:                             |
|     r_market_eff = Interest_t / L_{t-1}                  |
|                  (implied effective rate; relies on      |
|                   prior-period disclosure of L_{t-1})    |
|     P_0          = L_0 + E_0                             |
|                  (when L_0 is disclosed or recoverable)  |
|     credit-and-rate-shift = (FV_t,L − L_t) / L_t         |
|                  (relative fair-value-to-carrying gap;   |
|                   captures credit-spread and rate moves  |
|                   since issuance)                        |
+----------------------------------------------------------+
```

**Source:** HKICPA HKFRS 7 (2018) pp.8-9, pp.14-15, pp.21-22;
HKICPA HKAS 32 (2022) pp.17-18 (paragraphs 28-31 anchor the
separate-presentation requirement for the equity component used
by the equity-section reconstruction observable above).

The reporting-quality assessment uses the HKFRS 7 disclosure
cell set as a checklist. A high-quality footnote on a
callable-puttable convertible carries: (i) explicit paragraph-17
multiple-embedded-derivative naming when the instrument has more
than one interdependent embedded derivative; (ii) paragraph-8
carrying amount of the liability category (amortized cost)
together with the equity component separately presented in the
equity section per the HKAS 32 paragraph 28-31
separate-presentation requirement; (iii) paragraph-20
interest expense computed using the effective-interest method
for the amortized-cost-classified liability; (iv) paragraph-25
fair-value disclosure of the liability component in a way that
permits comparison with the amortized-cost carrying amount,
respecting the paragraph-26 class-grouping discipline and the
paragraph-29 exemptions. The absence of any of cells (i)-(iv)
reduces the analyst's ability to apply the reconstruction
protocol above and constitutes a reporting-quality weakness on
the [`fra-reporting-quality-framework`](./fra-reporting-quality-framework.md)
ladder. **Source:** HKICPA HKFRS 7 (2018) pp.14-15, pp.21-22;
HKICPA HKAS 32 (2022) pp.17-18 (paragraphs 28-31 anchor the
separate-presentation requirement invoked by cell (ii)).

The classification regime determines which disclosure cells
apply. Under the IAS 32 path (fixed-for-fixed passes per the
boundary analysis in
[`fra-ifrs9-vs-ias32-compound-evolution`](./fra-ifrs9-vs-ias32-compound-evolution.md)),
the convertible's disclosure pattern is dominated by §8
amortized-cost-category carrying amount + §17 multiple-embedded
existence + §20 effective-interest expense + §25 fair-value
comparison. Under the IFRS 9 path (fixed-for-fixed fails, host +
bifurcated embedded derivative), the same §8 / §17 / §20 / §25
cells still apply — the §8 categories disclosure surfaces the
host debt at amortized cost and the bifurcated derivative under
its own measurement category, and the §20(a)(i) income-statement
disclosure surfaces the period net gains or losses on the
fair-value-through-profit-or-loss derivative alongside the host's
effective-interest expense. The HKFRS 7 standard alone does not specify the
fair-value-hierarchy presentation depth (Level 1 / 2 / 3) nor
the model-input sensitivity table sometimes shown alongside;
those depths are governed by other standards not anchored in
this card's source set. **Source:** HKICPA HKFRS 7 (2018)
pp.8-9, pp.14-15.

## See Also

- [`fra-ifrs9-vs-ias32-compound-evolution`](./fra-ifrs9-vs-ias32-compound-evolution.md) — the classification regime that determines which disclosure cells apply
- [`fra-issuer-side-compound-instrument-split`](./fra-issuer-side-compound-instrument-split.md) — the underlying split mechanism that the disclosure cells document
- [`fra-effective-interest-amortization-bond-side`](./fra-effective-interest-amortization-bond-side.md) — the amortized-cost mechanics that produce the interest-expense disclosure under §20
- [`fra-conversion-extinguishment-accounting`](./fra-conversion-extinguishment-accounting.md) — the settlement-path mechanics that generate gain/loss disclosures under §20
- [`fra-reporting-quality-framework`](./fra-reporting-quality-framework.md) — the seven-tier reporting-quality ladder against which the disclosure footnote is graded
- [`fra-financial-statement-applications`](./fra-financial-statement-applications.md) — broader HKFRS 7 risk-disclosure requirements (credit-risk, liquidity-risk, market-risk) that surround the convertible-specific cells

## Escalate to Raw When

Open HKICPA HKFRS 7 (pp.14-15 + pp.8-9 + pp.21-22) directly
when any of the criteria below applies. **Source:** HKICPA HKFRS 7
(2018) pp.8-15, pp.21-22.

- the convertible has multiple interdependent embedded
  derivatives and the analyst must determine whether the
  paragraph-17 existence disclosure has been correctly
  triggered, or whether the issuer has incorrectly disaggregated
  the derivatives into separate financial-instrument categories.
  **Source:** HKICPA HKFRS 7 (2018) pp.14.
- the issuer reports a fair-value disclosure on the bifurcated
  embedded derivative whose presentation depth (e.g., level-of-input
  hierarchy or model-input sensitivity) is governed by other
  standards not anchored in this card's source set, and the
  analyst needs to inspect the cross-referenced HKFRS 13 / HKAS 1
  policy notes (out of this card's scope; admit those standards to
  the source matrix to extend coverage).
- the issuer has reclassified the convertible mid-life (e.g., a
  contingent-settlement provision activated, triggering an
  IAS 32 → IFRS 9 boundary crossing) and HKFRS 7 paragraph 12B
  reclassification disclosures must be checked for completeness.
  **Source:** HKICPA HKFRS 7 (2018) pp.11-12.
