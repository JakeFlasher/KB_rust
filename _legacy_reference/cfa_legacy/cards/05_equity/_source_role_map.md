# Subcorpus 05 — Source-Role Map

This file maps every planned 05 card to its primary anchor source and an
explicit chapter span, plus the per-card `CFA Relevance` and `Source
Stance` declarations required by AC-51. The map is authored before any
05 card is drafted; each card's frontmatter MUST agree with the row
below at draft time. Every `Primary anchor` cell uses the exact
repo-relative path recorded in `_corpus_planning/05_source_matrix.md`
(no basenames). All 05 cards anchor on Damodaran with a `pp.<N-M>` span;
CFA Vol.4 appears only in `Supporting sources:` with a `Vol.4/pp.<N-M>`
span. Where a card cannot yet be narrowed below a chapter, the full
chapter scope is used (per Codex Round-0 review guidance carried from
v3 AC-35).

> **Provenance (current state, after Round 2 finalization)**: this
> file is rebuilt against the actual Damodaran 4ed publisher TOC, with
> the Batch-4 extension topic list set by the immutable AC-59 plan
> text. Authoring history: Round 0 fabricated chapter labels for
> Ch.3-8 and Ch.26-32; Codex Round-0 review caught the fabrication.
> Round 1 corrected the chapter index by re-extracting the publisher
> TOC via `pdftotext` (front-matter pages 1-25 as ground truth) AND
> overreached by substituting the AC-59 Batch-4 topic list with
> Damodaran-Ch.26-32 topics (real options, distressed equity, EVA,
> probabilistic valuation); Codex Round-1 review correctly rejected
> the substitution as plan drift. Round 2 restored the AC-59 Batch-4
> topic list (security-level Fama-French construction; value /
> momentum / quality / low-vol scoring; implied cost of capital from
> market prices; industry / sector factor models) anchored on real
> Damodaran chapters at intuition depth (Ch.4 + Ch.6 + Ch.8 +
> Ch.14-15 + Ch.17-21) per DEC-1's pre-existing depth qualifier
> (signals + ranking + scoring intuition only; Fama-MacBeth regression
> machinery deferred to future-01). The active card list, anchors,
> and depth boundaries below reflect the Round-2 restored state. See
> the goal-tracker Plan Evolution Log entries for Rounds 1, 1 review,
> and 2 for the full audit trail.

## Source-stance vocabulary (v4)

The two allowed values for `Source Stance:` on every 05 card are:

- `primary-damodaran` — card's `Primary raw source:` is
  `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` with a
  `pp.<N-M>` page span. Used by every Damodaran-anchored 05 card
  (CFA-core AND extension).

- `primary-cfa` — card's `Primary raw source:` is the OCR'd
  CFA personal-notes PDF ([notes-citation removed per Critical Rule 9]). Admissible on
  05 cards because the source matrix authorizes that PDF for the 05
  subcorpus.
  [Prior revision opened an additional notes-PDF-derived primary
  stance here; the admission was removed per Critical Rule 9 because
  the notes source is user-volatile and non-quotable under the
  cacg.v0 source-matrix. Pre-existing Damodaran-anchored cards
  remain unchanged.]

The per-05-subcorpus FM006 branch in both validators rejects any
other value: legacy `theoretical` / `practitioner` / `mixed`
(admissible only on 06/07/08 cards), v3 `primary-cfa` /
`primary-cochrane` / `primary-pedersen` (admissible only on 09
cards), AND the meta-tag `supporting-only` (admissible on 09
CFA-core cards). The 05 source-role map narrows the global enum
to two per-subcorpus admissible values, mirroring the v3
Round-12 per-09-subcorpus stance vocabulary discipline (see
`BL-20260505-stance-enum-vs-plan-vocabulary` Round-12
generalization in `.humanize/bitlesson.md`).

CFA Vol.4 Equity readings appear in the role map's "supporting" role —
they are cited in `Supporting sources:` of CFA-core cards. No 05 card
declares `Source Stance: supporting-only`; that meta-tag is reserved
for 09 (where the source-role map defines it as a stance whose
`Primary raw source:` is the CFA L1 combined PDF). On 05 cards, CFA
Vol.4 supporting-cite use does NOT alter the card's `primary-damodaran`
stance.

## Damodaran (2012, 4ed) chapter index

Verified against `05_Equity/Damodaran_Investment_Valuation_4ed.pdf`
(1356 pages; audited GOOD at 1639.6 chars/page; SHA256
`0d68f0e02293970c2e104bb7662becbd8f04d322382b572b358cb1c5944b8869`).
Chapter labels below are extracted from the publisher Table of Contents
(front-matter pages 1-25 of the on-disk PDF) using `pdftotext -layout`.
The extracted TOC is the ground truth; any prior draft of this file
that disagrees with the TOC is incorrect.

- Ch.1  Introduction to Valuation
- Ch.2  Approaches to Valuation
- Ch.3  Understanding Financial Statements
- Ch.4  The Basics of Risk
- Ch.5  Option Pricing Theory and Models
- Ch.6  Market Efficiency — Definition, Tests, and Evidence
- Ch.7  Riskless Rates and Risk Premiums
- Ch.8  Estimating Risk Parameters and Costs of Financing
- Ch.9  Measuring Earnings
- Ch.10 From Earnings to Cash Flows
- Ch.11 Estimating Growth
- Ch.12 Closure in Valuation: Estimating Terminal Value
- Ch.13 Narrative and Numbers — Story to Value
- Ch.14 Equity Intrinsic Value Models (DDM, FCFE, augmented DDM)
- Ch.15 Firm Valuation: Cost of Capital and Adjusted Present Value Approaches (FCFF, APV, sum-of-the-parts)
- Ch.16 Estimating Equity Value per Share (nonoperating assets, stock-based comp, voting-rights premium)
- Ch.17 Fundamental Principles of Relative Valuation
- Ch.18 Earnings Multiples (P/E, PEG, variants, EV/EBITDA)
- Ch.19 Book Value Multiples (P/B, value-to-book, Tobin's Q)
- Ch.20 Revenue Multiples and Sector-Specific Multiples
- Ch.21 Valuing Financial Service Firms
- Ch.22 Valuing Money-Losing Firms
- Ch.23 Valuing Young or Start-Up Firms
- Ch.24 Valuing Private Firms
- Ch.25 Acquisitions and Takeovers (boundary-discipline ONLY: 05 does not anchor any card here; M&A depth defers to future-04)
- Ch.26 Valuing Real Estate
- Ch.27 Valuing Other Assets
- Ch.28 The Option to Delay and Valuation Implications (real options)
- Ch.29 The Options to Expand and to Abandon: Valuation Implications (real options)
- Ch.30 Valuing Equity in Distressed Firms
- Ch.31 Value Enhancement: A Discounted Cash Flow Valuation Framework
- Ch.32 Value Enhancement: Economic Value Added, Cash Flow Return, and Other Tools
- Ch.33+ Probabilistic Approaches in Valuation: Scenario Analysis, Decision Trees, Simulations (and additional special-application chapters)

> **Damodaran 4ed extension-depth note (Round 2 finalization;
> Round 18 source-fidelity correction)**:
> Damodaran 4ed is a deep DCF-and-multiples valuation reference that
> touches the Fama-French / value / momentum / quality / low-vol /
> sector-factor topics at INTUITION depth in Ch.4 (Alternative Models
> for Equity Risk — APT and multifactor framing; distress-cost framing
> for quality scoring inputs), Ch.6 (Market Efficiency — documented
> anomaly catalogue: size, low-P/E, low-P/B, momentum, reversal, and
> post-earnings-announcement drift; standalone low-volatility anomaly
> and quality-outperformance / QMJ / BAB premia are post-Damodaran-2012
> research outside Ch.6 coverage), Ch.8 (Estimating Risk Parameters —
> beta, bottom-up risk decomposition, and realized-volatility risk
> inputs that anchor the low-vol scoring signal), Ch.14-15 (reverse-DCF
> for market-implied cost of capital), Ch.17-20 (relative valuation /
> multiples / sector-specific multiples), and Ch.21 (industry-specific
> valuation for financial service firms). It does NOT contain a
> dedicated Fama-French construction chapter at academic-factor-
> construction depth, nor Fama-MacBeth cross-sectional regression
> machinery. Per DEC-1 (Pending User Decision in plan v4), the 05
> extension cards stay at **signals + ranking + scoring intuition**
> depth; the deeper Fama-MacBeth regression / continuous-time
> econometrics machinery defers to future-01 (Quantitative Methods).
> The cards themselves remain in scope per AC-59; the depth limit is
> the legitimate scope constraint, not a topic exclusion.
>
> Ch.26-32 cover unrelated topics (real estate Ch.26-27; real options
> Ch.28-29; distressed equity Ch.30; EVA / value enhancement Ch.31-32;
> probabilistic methods Ch.33+) and are NOT used as Batch 4 anchors.
> The Round-0 draft of this map fabricated factor-model content for
> Ch.26-32; that error was caught in Codex Round-0 review and Round-1
> rebuilt the chapter index from the real publisher TOC. Round-1
> additionally over-reached by replacing the AC-59 topic list with
> Ch.26-32 topics; Codex Round-1 review correctly rejected that as
> plan drift, and Round 2 restores the immutable AC-59 topic list
> anchored on Ch.4 + Ch.6 + Ch.8 + Ch.14-15 + Ch.17-21.

## CFA Vol.4 Equity reading scopes (publisher pagination per the on-disk PDF, supporting only)

Verified against `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf`
(combined PDF, 4353 pages per `pdfinfo`; Vol.4 segment carries the
Equity readings R36-R38 inside a publisher-paginated sequence
accessed via the `Vol.4/pp.<P-Q>` form required by AC-2.1).

> **Round 5 [v4] re-extraction provenance**: an earlier draft of this
> table listed R36 "Market Organization", R37 "Security Market Indexes",
> R38 "Market Efficiency", R39 "Overview of Equity Securities", R40
> "Introduction to Industry and Company Analysis", R41 "Equity
> Valuation: Concepts and Basic Tools" — that mapping reflects an older
> CFA L1 syllabus number-to-title scheme. The on-disk 2022 combined PDF
> was re-extracted via `pdftotext -layout` in Round 5; the actual Vol.4
> Equity readings are the three below (R36-R38 only). R39, R40, R41 are
> Fixed-Income readings ("Fixed-Income Securities: Defining Elements",
> "Fixed-Income Markets: Issuance, Trading, and Funding", "Introduction
> to Fixed-Income Valuation") and are out of scope for 05 — they belong
> to existing-06 or a future-FI subcorpus. Publisher pagination was
> derived by reading the page-header strings on PDF pages 2210-2230;
> publisher_page = pdf_page - 1937 (offset confirmed by PDF page 2211
> showing printed page "274" against R36 starting at PDF page 2208 =
> publisher page 271).

| Reading | Title | Vol.4 publisher page span | Subcorpus 05 topical scope |
|---|---|---|---|
| R36 | Overview of Equity Securities | Vol.4/pp.271-306 | Touched by `eq-intrinsic-value` (equity-securities perspective), `eq-pe-and-relative-valuation` (multiples on equity claims), `eq-private-vs-public-equity-valuation-l1`, `eq-share-count-and-per-share-effects`. |
| R37 | Introduction to Industry and Company Analysis | Vol.4/pp.307-360 | Touched by `eq-cyclicality-and-cycle-adjustment` (industry-cycle vocabulary), `eq-comparable-company-analysis` (peer-group construction), `eq-industry-and-sector-factor-models` (sector classification). |
| R38 | Equity Valuation: Concepts and Basic Tools | Vol.4/pp.361-416 | Primary supporting reading for the entire Batch 1 (`eq-intrinsic-value`, `eq-discount-rate-and-required-return-foundations`, `eq-dividend-discount-models`, `eq-pe-and-relative-valuation`, `eq-pb-and-multiples-taxonomy`) and Batch 2 (`eq-dcf-mechanics`, `eq-comparable-company-analysis`, `eq-sum-of-parts-valuation`). The L1 valuation primer at intuition depth. |

## Card-by-card map

24 planned cards: 19 CFA-core + 5 extension. The split satisfies AC-58
(17-19 core) and AC-59 (4-5 extension). Total card count satisfies
DEC-15 (24 target; 22-24 band). Lower-band fallback at 22 cards drops
two cards (`eq-equity-risk-premium-intuition` from Batch 1 and either
`eq-share-count-and-per-share-effects` or `eq-private-vs-public-equity-valuation-l1`
from Batch 3) if first-batch Codex calibration recommends consolidation;
the optional 5th extension card (`eq-industry-and-sector-factor-models`)
becomes the first deferral if the lower-band path is chosen.

### First batch (7 cards) — CFA-core, valuation foundations

These seven cards carry `CFA Relevance: core` and `Source Stance:
primary-damodaran`. Each declares Damodaran as Primary raw source with
a chapter span; CFA Vol.4 R38 "Equity Valuation: Concepts and Basic
Tools" (Vol.4/pp.361-416) is the primary supporting reading, and where
noted CFA Vol.4 R36 "Overview of Equity Securities" (Vol.4/pp.271-306)
appears in `Supporting sources:` for the equity-securities-perspective
context. Citations use the `Vol.4/pp.<P-Q>` form required by AC-2.1.

| Card filename | CFA Relevance | Source Stance | Primary anchor (full matrix path + chapter span) | First-batch role |
|---|---|---|---|---|
| `eq-intrinsic-value.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.1 (Introduction to Valuation) + Ch.2 (Approaches to Valuation) | What intrinsic value is; the firm-level rationale for valuation; the value-vs-price gap; the philosophical basis for valuation |
| `eq-discount-rate-and-required-return-foundations.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.4 (The Basics of Risk) + Ch.7 (Riskless Rates and Risk Premiums) + Ch.8 (Estimating Risk Parameters and Costs of Financing) | Required-return decomposition (riskless rate + risk premium); cost-of-equity frame; CAPM as the foundational pricing model in Damodaran's frame |
| `eq-dividend-discount-models.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.14 (Equity Intrinsic Value Models — DDM, augmented DDM, FCFE comparison) | DDM as the canonical equity-cash-flow-to-price translation; Gordon growth as a special case; augmented DDM (multi-stage) intuition |
| `eq-payout-policy-and-growth.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.11 (Estimating Growth — fundamental determinants) + Ch.14 (DDM payout-and-growth integration) | Sustainable-growth identity (g = retention × ROE) as a valuation INPUT only — dividend-policy depth deferred to future-04 (Corporate Finance) |
| `eq-pe-and-relative-valuation.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.17 (Fundamental Principles of Relative Valuation) + Ch.18 (Earnings Multiples — P/E section) | P/E as the canonical multiple; the price-vs-earnings ratio at L1 depth; the four-step relative-valuation process |
| `eq-pb-and-multiples-taxonomy.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.18 (Earnings Multiples — EV/EBITDA) + Ch.19 (Book Value Multiples — P/B, Tobin's Q) + Ch.20 (Revenue and Sector-Specific Multiples) | P/B taxonomy; EV/EBITDA and P/Sales as enterprise-value extensions; multiples grouping and standardization |
| `eq-equity-risk-premium-intuition.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.7 (Riskless Rates and Risk Premiums — Equity Risk Premium section) | Equity risk premium as the price-of-equity-risk; historical vs implied ERP intuition, with country-risk-premium extensions deferred unless needed |

Supporting sources for the first batch are typically CFA Vol.4 R38
"Equity Valuation: Concepts and Basic Tools" (Vol.4/pp.361-416) at L1
depth; some cards also cite R36 "Overview of Equity Securities"
(Vol.4/pp.271-306) for the equity-securities-perspective context. CFA
Vol.4 citations use the `Vol.4/pp.<P-Q>` form required by AC-2.1.

### Second batch (6 cards) — CFA-core, valuation methodology

Process / methodology / DCF-and-multiples-mechanics cards. All carry
`CFA Relevance: core` and `Source Stance: primary-damodaran`.

| Card filename | CFA Relevance | Source Stance | Primary anchor (full matrix path + chapter span) | Notes |
|---|---|---|---|---|
| `eq-dcf-mechanics.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.14 (Equity Intrinsic Value Models) + Ch.15 (Firm Valuation: Cost of Capital and APV) | DCF mechanics: discounting horizon-cash-flows + terminal value; FCFE vs FCFF dispatch; APV as alternative to WACC-based DCF |
| `eq-fcfe-fcff-decomposition.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.10 (From Earnings to Cash Flows) + Ch.14 (FCFE) + Ch.15 (FCFF) | FCFE = NI + D&A − ΔWC − CapEx + ΔDebt; FCFF = EBIT(1−τ) + D&A − ΔWC − CapEx; bridge between |
| `eq-terminal-value-and-sensitivity.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.12 (Closure in Valuation: Estimating Terminal Value) | Terminal value as the dominant DCF component; Gordon-growth vs exit-multiple methods; survival assumption; sensitivity intuition |
| `eq-comparable-company-analysis.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.17 (Fundamental Principles) + Ch.18-20 (Multiples chapters — peer group construction sections) | Comparable-company method; peer-group construction; multiple selection at L1 depth |
| `eq-sum-of-parts-valuation.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.15 (Firm Valuation — Sum of the Parts subsection) + Ch.16 (Equity Value per Share — nonoperating assets) | Sum-of-parts as a conglomerate-discount remedy; segment-level DCF aggregation; nonoperating asset value addition |
| `eq-cyclicality-and-cycle-adjustment.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.11 (Estimating Growth — historical and qualitative growth) + Ch.22 (Valuing Money-Losing Firms — cyclical-firm framing) | Normalized-earnings frame; mid-cycle adjustment; cycle-exposure intuition (NOT macro-cycle modeling — defer to 02 Economics; cyclicality is distributed across Damodaran chapters rather than localized to a single chapter) |

Supporting sources for batch 2 are CFA Vol.4 R37 "Introduction to
Industry and Company Analysis" (Vol.4/pp.307-360) and R38 "Equity
Valuation: Concepts and Basic Tools" (Vol.4/pp.361-416).

### Third batch (6 cards) — CFA-core, measurement and L1-process

These six cards extend the measurement and exposure vocabulary while
staying within Damodaran anchoring. Each carries `CFA Relevance: core`
and `Source Stance: primary-damodaran`.

| Card filename | CFA Relevance | Source Stance | Primary anchor (full matrix path + chapter span) | Notes |
|---|---|---|---|---|
| `eq-implied-cost-of-capital-foundations.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.8 (Cost of Equity / Cost of Capital) + Ch.14 (DDM-implied k by inversion) | Implied-cost-of-capital foundations: invert the DDM/DCF to back out the discount rate; depends on DDM (Batch 1) AND DCF mechanics (Batch 2) — placed in Batch 3 per AC-56 dependency-clean revision |
| `eq-multiples-dispersion.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.18-19 (Earnings + Book multiples cross-firm distribution sections) | Multiples-dispersion intuition: cross-firm spread in P/E, P/B, EV/EBITDA; sector-driven dispersion vs idiosyncratic |
| `eq-cross-sectional-multiples-distribution.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.17 (Standardized Values and Multiples) + Ch.18 (regression-based fundamentals-to-multiples links at intuition depth) | Cross-sectional distribution of multiples; Damodaran's regression-based fundamentals-to-multiples link at intuition depth (Fama-MacBeth machinery deferred to future-01 per BOUNDARY-DISCIPLINE) |
| `eq-equity-cost-of-capital-estimation.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.7 (Riskless Rate / Risk Premium) + Ch.8 (Estimating Risk Parameters and Costs of Financing) | Cost-of-equity estimation at L1 depth; CAPM-input frame; bottom-up beta intuition (security-level) |
| `eq-private-vs-public-equity-valuation-l1.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.24 (Valuing Private Firms) | Private-vs-public valuation at L1 depth; illiquidity discount intuition; private-equity valuation context |
| `eq-share-count-and-per-share-effects.md` | core | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.16 (Estimating Equity Value per Share) | Share-count and per-share-effects as VALUATION INPUTS only — buyback / dividend policy mechanics deferred to future-04 (Corporate Finance) per BOUNDARY-DISCIPLINE |

Supporting sources for batch 3 are CFA Vol.4 R36 "Overview of Equity
Securities" (Vol.4/pp.271-306) and R38 "Equity Valuation: Concepts and
Basic Tools" (Vol.4/pp.361-416).

### Fourth batch (4-5 cards) — extension, primary-damodaran (security-level factor implementation per AC-59)

These 4-5 cards carry `CFA Relevance: extension` and `Source Stance:
primary-damodaran`. They specialize 09's Cochrane-anchored multifactor
pricing extension into the security-level factor implementation that
09 deferred 33 times across 10 cards. Per AC-59 immutable text the
topics are: security-level Fama-French construction; value / momentum
/ quality / low-vol / size scoring at security level; implied
cost-of-capital from market prices; industry / sector factor models
distinct from FF academic factors. Per DEC-1 (Pending User Decision
in plan v4): the default scope for these extension cards is **signals
+ ranking + scoring intuition only**; Fama-MacBeth cross-sectional
regression econometrics and continuous-time pricing machinery defer
to future-01 (Quantitative Methods). The cards remain in scope; the
depth limit is the legitimate scope constraint.

Damodaran 4ed touches the AC-59 topics at intuition depth (without
the academic factor-construction depth a quant-equity textbook would
provide):

- Ch.4 (Alternative Models for Equity Risk — APT, multifactor framing, comparative analysis of equity-risk models including FF-style multi-beta; distress-cost framing as the quality-scoring anchor).
- Ch.6 (Market Efficiency — documented anomaly catalogue: size, low-P/E, low-P/B, momentum, reversal, post-earnings-announcement drift; standalone low-volatility anomaly and quality-outperformance / QMJ / BAB premium claims are post-Damodaran-2012 research outside Ch.6 coverage).
- Ch.8 (Estimating Risk Parameters and Costs of Financing — bottom-up beta as security-level factor exposure; comparative-firm beta proxies; realized-volatility risk inputs that anchor the low-vol scoring signal).
- Ch.14-15 (Equity / Firm DCF — reverse-DDM and reverse-DCF for market-implied cost of capital).
- Ch.17-20 (Relative Valuation: Standardized Values; Earnings Multiples P/E + PEG; Book Value Multiples P/B; Revenue and Sector-Specific Multiples — value scoring lattice).
- Ch.21 (Valuing Financial Service Firms — industry-specific factor structure as an exemplar of sector-level factor models).

09 cards (`pm-multifactor-asset-pricing-intuition.md`,
`pm-anomalies-and-cross-sectional-pricing.md`,
`pm-factor-models-intuition.md`, `pm-beta-and-factor-exposure.md`,
`pm-capm-and-sml.md`) appear in `Repo touchpoints:` (cross-vertical
citation for theory context) — NOT in `Supporting sources:` (which is
reserved for raw-PDF citations per the matrix's path-based row
scope).

| Card filename | CFA Relevance | Source Stance | Primary anchor (full matrix path + chapter span) | Notes |
|---|---|---|---|---|
| `eq-fama-french-construction-at-security-level.md` | extension | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.4 (Alternative Models for Equity Risk — APT and multifactor framing) + Ch.6 (Market Anomalies — value and size premium evidence) + Ch.19 (Book Value Multiples — P/B as size/value proxy) | FF construction at security level: long high-B/M minus low-B/M; small-minus-big; market-beta plus value plus size as the canonical 3-factor multi-beta. Stays at intuition depth per DEC-1 (Fama-MacBeth regression deferred to future-01). Closes 09 `pm-multifactor-asset-pricing-intuition.md` future-05 deferral. Repo touchpoints to 09's `pm-multifactor-asset-pricing-intuition.md` and `pm-anomalies-and-cross-sectional-pricing.md` for the asset-pricing-theory framing. |
| `eq-value-and-momentum-factor-scoring.md` | extension | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.6 (Market Anomalies — value via low P/E and low P/B; momentum via post-earnings-announcement drift and price reversals) + Ch.18 (Earnings Multiples — P/E ranking) + Ch.19 (Book Value Multiples — P/B ranking) | Security-level value scoring (low P/E, low P/B); security-level momentum scoring (price-reversal and post-earnings-drift signals). Stays at signals + ranking + scoring intuition depth per DEC-1; Damodaran's Ch.6 anomaly evidence is the depth ceiling. Closes 09 `pm-anomalies-and-cross-sectional-pricing.md` future-05 deferrals. |
| `eq-quality-and-low-vol-factor-scoring.md` | extension | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.4 (Distress Costs and Alternative Risk Models — financial-strength / earnings-stability / distress proxies as quality scoring inputs) + Ch.8 (Estimating Risk Parameters — bottom-up beta and realized-volatility risk inputs that anchor the low-vol scoring signal) | Security-level quality scoring (financial-strength / earnings-stability / low-distress proxies as cost-of-equity inputs); security-level low-vol scoring (low-beta, low-realized-vol as risk inputs to ranking). Stays at intuition depth per DEC-1. Closes 09 `pm-factor-models-intuition.md` future-05 deferral. Standalone low-vol anomaly and QMJ / BAB quality-outperformance premium claims are explicitly OUTSIDE Damodaran 4ed coverage; econometric estimation of those premia deferred to future-01. |
| `eq-implied-cost-of-capital-from-market-prices.md` | extension | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.14 (Equity Intrinsic Value Models — reverse-DDM) + Ch.15 (Firm Valuation: Cost of Capital — reverse-DCF) | Reverse-DCF: market price → implied k. Cross-references `eq-implied-cost-of-capital-foundations.md` (Batch 3) for the foundation; Damodaran's data-driven implied-ERP narrative (his website's monthly implied-ERP series is anchored on this method). |
| `eq-industry-and-sector-factor-models.md` (optional 5th) | extension | primary-damodaran | `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` Ch.20 (Sector-Specific Multiples — sector-level multiples dispersion as a sector-factor signal) + Ch.21 (Valuing Financial Service Firms — exemplar of industry-specific factor structure) | Industry / sector factor models distinct from FF academic factors; equity-style classification context (sector-driven multiples dispersion). Closes 09 `pm-beta-and-factor-exposure.md` future-05 deferral. Stays at intuition depth per DEC-1. |

## Boundary discipline summary

Boundary deferrals enforced by the 05 source-role map (and by the 05
specialty SKILL routing at promotion time):

- Multifactor-pricing intuition + CAPM/SML → 09 (already complete; 05
  cites 09 cards via `Repo touchpoints:` rather than re-deriving).
- Behavioral overlays → future-10 (already deferred via 09).
- Portfolio risk decomposition / sector exposure budgeting / stress /
  VaR / risk reporting → 11_risk_management (closed v11) (05 covers ONLY security-level
  industry/sector classification context within Ch.20 sector multiples
  and Ch.21 financial-firm-specific valuation; portfolio aggregation
  depth lives in 11).
- M&A / spinoffs / LBOs / dividend-policy depth / buyback mechanics →
  future-04 (Corporate Finance; Damodaran Ch.25 acquisition coverage
  is referenced for boundary discipline only, not anchored in any 05
  card).
- Tax-arbitrage / equity-execution spreads → future-14
  (Microstructure & Trading).
- Fama-MacBeth cross-sectional regression econometrics / continuous-time
  pricing / academic-depth factor construction → future-01
  (Quantitative Methods). Per DEC-1, the 05 extension cards stay at
  signals + ranking + scoring intuition depth; the regression-estimation
  and pricing-econometrics machinery is the legitimate boundary line,
  not the factor topic itself. Security-level Fama-French construction,
  value/momentum/quality/low-vol scoring, and industry/sector factor
  models REMAIN IN SCOPE for 05 at intuition depth and are anchored on
  Damodaran Ch.4 + Ch.6 + Ch.8 + Ch.17-21 per the Batch 4 card-by-card
  map above.
- Performance attribution / GIPS → future-15.

## Summary counts

- 19 CFA-core cards (`Source Stance: primary-damodaran`) — first batch
  (7) + second batch (6) + third batch (6).
- 5 extension cards (target; lower-band fallback drops the optional
  5th `eq-industry-and-sector-factor-models.md` to land at 4) —
  all `Source Stance: primary-damodaran`.
- Total: 24 (DEC-15 target). Lower-band fallback at 22 cards drops
  `eq-equity-risk-premium-intuition.md` (Batch 1) and one of
  `eq-share-count-and-per-share-effects.md` /
  `eq-private-vs-public-equity-valuation-l1.md` (Batch 3), and the
  optional 5th extension card, if first-batch Codex calibration
  recommends consolidation.
