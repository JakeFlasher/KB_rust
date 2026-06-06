This is exactly the corpus that grounds the four methods. The very first post confirms all the load-bearing specifics: CNOOC (00883) special-dividend contract adjustment to strike 11.31 / 1105 shares, sell-put/sell-call rolling on Tencent/Alibaba, the H-share 10% withholding vs red-chip exemption (941 vs 728/762/883/941), the physical-certificate withdrawal tax workaround (提取實物 → 石鼓/registered shares), Stock Connect cannot trade options (港股通沒法做期權), American-style early exercise, and contract-multiplier confusion (一張合約 ≠ 100股).

I have everything I need. The investor's four methods, confirmed from the corpus and the recommendation JSON's `investor_methods_grounded` indices, are:
1. Options income overlay (sell covered calls + cash-secured puts on owned HK blue chips; rolling/assignment; theta-vs-delta strike selection; 100% collateral / never naked)
2. Yield + patience + contrarian accumulation of HK blue-chip dividend payers (value temperament)
3. HK-specific operational mechanics (HKEX option contract adjustment on special dividends, Stock Connect options exclusion, H-share vs red-chip dividend withholding, stamp duty, physical-certificate withdrawal)
4. Discipline / behavioral temperament (持股心态, balanced portfolio, rules-based exits, never lose the base position)

I have enough to write the report. No further tool calls needed.

---

# HKEX Practitioner Deck — Grounding-Source Coverage & Ingest Plan

## Scope note
"Existing ingested corpus" = the 73-source registry actually in `sources/cfa/_registry/source_inventory.json` (e.g. Hull 11e, Damodaran 2025, McNeil/Frey/Embrechts QRM, Shleifer, Gennaioli-Shleifer, Pedersen *Efficiently Inefficient*, Cochrane *Asset Pricing*, plus the CFA L1/L2/L3 curricula and the microstructure card set built from Harris/O'Hara/Hasbrouck-style sources). The investor's four methods, confirmed against the 狗不叫 corpus at `_research/xueqiu_goubujiao/corpus_full.md`:
- **M1** options-income overlay (sell covered calls + cash-secured puts on owned HK blue chips; rolling, assignment, theta-vs-delta strike/tenor, strict 100%-collateral / never-naked)
- **M2** yield + patience + contrarian accumulation of HK blue-chip dividend payers (value temperament)
- **M3** HK-specific operational mechanics (HKEX option contract adjustment on special dividends, Stock Connect options exclusion, H-share-vs-red-chip 10% withholding, stamp duty, physical-certificate withdrawal)
- **M4** discipline / behavioral temperament (持股心态, balanced book, never lose the base position, rules-based exits)

---

## Coverage verdict

The existing corpus is **strong on theory and behavioral economics, but does not ground any of the four methods on its own.** It is sufficient as a *cross-link layer* in three places and *insufficient — NEW sources required* everywhere the methods become operational.

**Sufficient (cross-link existing CFA cards, do NOT re-ingest):**
- **M1 option theory layer** — `cards/cfa/07_derivatives_and_volatility/` is a complete Hull-grounded theory set: `deriv-greeks-overview`, `deriv-vega-and-theta`, `deriv-delta-and-hedging`, `deriv-bsm-formula`, `deriv-put-call-parity`, `deriv-implied-volatility`, `deriv-vol-surface-anatomy`, `deriv-option-payoff-anatomy`. These ground *what theta/delta/IV are*. They are pure no-arbitrage PDE theory (verified: `deriv-vega-and-theta` cites Hull p.428/434 on theta/vega definitions, nothing on writing premium). New practitioner sources should *cite up* to these, not duplicate them.
- **M2 valuation layer** — `cards/cfa/05_equity/` (`eq-dividend-discount-models`, `eq-payout-policy-and-growth`, `eq-intrinsic-value`, `eq-dcf-mechanics`, `eq-quality-and-low-vol-factor-scoring`) plus Damodaran 2025 fully ground intrinsic value / DDM / payout mechanics. The *mechanics* of dividend valuation are covered; the *doctrine and temperament* are not.
- **M4 behavioral layer** — `cards/cfa/10_behavioral_finance/` is unusually deep (Shleifer limits-of-arbitrage, prospect theory, loss aversion, overconfidence, mental accounting, extrapolative beliefs). This grounds the *academic* "why biases exist." The *practitioner operating temperament* is not present.
- **M3 generic microstructure** — `cards/cfa/14_microstructure_and_trading/` covers spreads, order types, price-time precedence, market impact, liquidity. This grounds generic venue mechanics but says **nothing HK-specific**.

**Insufficient — NEW sources required (the real gaps):**
- **M1 practitioner mechanics** — no card or source covers covered-call writing, cash-secured put selling, rolling (down/out/up), mid-contract unwind, assignment management, or moneyness-based strike/tenor selection as a lived workflow. The `grep` for "covered call / cash-secured / put writing / rolling / assignment" across all cards returns only incidental hits (a CFA ethics card, an externalities card). **Hard gap.**
- **M1 vol-as-edge layer** — no source frames the variance/vol-risk premium as a tradeable edge with sizing and hedging-error P&L. Pedersen is strategy-survey level; Hull is theory. **Gap.**
- **M3 — the entire HK operational regime** — zero coverage of the HKEX special-dividend contract-adjustment regime (the 2% threshold, adjustment-ratio formula), Stock Connect's exclusion of options, H-share-vs-red-chip 10% withholding, HK stamp duty, or the physical-certificate-withdrawal-for-tax maneuver. This is the most acute gap and exactly the material the corpus cannot derive. **Hard gap — official HK primary sources mandatory.**
- **M2 doctrine + China-SOE context** — no value-investing doctrine source (Graham), no dividend-growth-as-discipline source, and no China-SOE institutional grounding ("why HK-listed Chinese blue chips behave as policy-market dividend instruments"). **Gap.**

---

## Tier-1 must-ingest (minimal set that closes the real gaps)

| Title | Author / issuer | Edition / year | Framework family | Investor methods | Ingestibility |
|---|---|---|---|---|---|
| Options as a Strategic Investment | Lawrence G. McMillan | 5th, 2012 (Prentice Hall/Penguin) | derivatives | M1, M3 | EN; commercial, no free PDF; ISBN 9780735204652; ~1,072pp. Ingest Ch.2 (covered calls) + Ch.19 (put sale) + Ch.15–20 only — the load-bearing slice. |
| Alan Ellman's Complete Encyclopedia for Covered Call Writing (Classic Ed.) | Alan Ellman | 2nd, 2019 (Digital Publishing of FL) | derivatives | M1, M4 | EN; commercial; ISBN 9781937183066; ~522pp. The 1:1 covered-call income-overlay workflow + rules-based exit/roll flowcharts. |
| Option Volatility and Pricing | Sheldon Natenberg | 2nd, 2015 (McGraw-Hill) | derivatives | M1 | EN; commercial; ISBN 9780071818773; ~592pp. Desk-level theta-vs-delta / short-gamma-long-theta intuition Hull lacks. |
| Volatility Trading (+ Website) | Euan Sinclair | 2nd, 2013 (Wiley) | derivatives | M1 | EN; commercial Wiley PDF; ISBN 9781118347133; ~320pp. The variance-premium-as-edge + sizing layer. |
| Trading Volatility: Correlation, Term Structure and Skew | Colin Bennett | 1st, 2014 (self-pub) | derivatives | M1, M3 | EN; **FREE author-sanctioned PDF** `trading-volatility.com/Trading-Volatility.pdf` (~316pp). Skew/term-structure + dividend↔option-pricing bridge. Best ingestibility of the books. |
| OTP for Options Trading EPs — **Ch.8 Special Events (Capital Adjustments)** | SEHK / HKEX | rolling (ref. 48-15) | derivatives | M1, M3 | EN; **FREE HKEX PDF** ~15–20pp. THE 2%-special-dividend threshold + adjustment-ratio formula. *Re-resolve the chap08 URL at ingest — a previously cited path now resolves to an unrelated file; pin from the Stock Options "Contract Adjustments" page.* |
| Stock Connect Information Book for Investors (+ FAQ) | HKEX | Info Book 23 Sep 2024; FAQ rolling | market-microstructure | M3 | EN; **FREE HKEX PDF** ~54pp. The "options NOT tradable via 港股通" fact + eligibility/quotas/settlement. |
| IRD — Notes to Tax Rates (DTA) + dividend treatment (s.26 IRO) | Inland Revenue Dept (HKSAR) | rolling 2024–26 | economics | M2, M3 | EN; **FREE printable HTML** `ird.gov.hk/eng/tax/dta_notes.htm`. Zero HK dividend WHT vs the 10% PRC withholding on H-shares (Guoshuihan [2008] No.897). Pair with PwC HK tax summary for cross-check. |
| Rates of Stamp Duty — Transfer of HK Stock | Inland Revenue Dept (HKSAR) | rates eff. 17 Nov 2023 | market-microstructure | M3 | EN; **FREE IRD PDF** `ird.gov.hk/eng/pdf/sd_stock_rates.pdf` 1–2pp. 0.1%/side, HK$5 fixed, stamping deadlines. |
| Shares Holding (CCASS vs paper) + Physical Share Certificates | IFEC ("The Chin Family") | current (2024 update) | market-microstructure | M2, M3 | EN; **FREE IFEC HTML**, few pp. The withdraw-cert → re-register-at-registrar → control-dividend workflow. Pair with HKEX Operating Guide for Investor Participants (stock deposit/withdrawal PDF). |
| The Intelligent Investor (Zweig commentary, Buffett preface) | Benjamin Graham | Rev. 4th 2003 **or** 3rd/75th-Anniv. 2024 | equity-valuation | M2, M4 | EN; trade book; ISBN 9780060555665 (2003) / 9780063423527 (2024). No free official PDF. Prefer 2003 for stable pagination unless 2024 currency wanted; pick ONE edition. |
| Security Analysis, 7th Ed. (Klarman commentary) | Graham & Dodd; ed. Klarman | 7th, 2023 (McGraw-Hill) | equity-valuation | M2, M4 | EN; ISBN 9781264932405; 864pp. Heaviest item — preserves Buffett-endorsed 1940 text + modern value-manager commentary. |
| The Single Best Investment: Dividend Growth | Lowell Miller | 2nd rev., 2006 | equity-valuation | M2, M4 | EN; ISBN 9780965175081; ~260pp. Author-authorized PDF on mhinvest.com (**verify it is the complete 2nd ed.** before ingest); also Internet Archive. Dividend-growth-as-discipline. |
| The Most Important Thing Illuminated | Howard Marks (+ Klarman et al. annotations) | Illuminated ed., 2013 | behavioral-finance | M2, M4 | EN; Columbia UP e-book EPUB/PDF; ISBN 9780231162845; 248pp. Practitioner cycle/contrarian temperament; embeds Klarman counterpoints (covers the un-ingestible *Margin of Safety*). |
| Red Capitalism (Revised) | Walter & Howie | Rev., 2012 (Wiley) | equity-valuation | M2, M3, M4 | EN; ISBN 9781118255100; 260pp. The "why HK-listed Chinese SOEs are policy-market dividend instruments" institutional grounding. |

> **Dedup note:** The hk-official "Ch.8 Special Events" and hk-china-equity "Stock Options OTP Ch.8" entries, and the two "Stock Connect Information Book" entries, are the **same primary documents** appearing under two categories — ingest **once each**. The 2024 *Intelligent Investor* appears under both dividend-value and temperament-behavioral — **one edition only**.

---

## Tier-2 valuable

| Title | Author / issuer | Edition / year | Framework family | Methods | Ingestibility |
|---|---|---|---|---|---|
| Alan Ellman's Selling Cash-Secured Puts | Alan Ellman | 1st, 2014 | derivatives | M1, M2 | EN; ISBN 9781942634003; ~282pp. CSP-for-accumulation as a distinct 100%-collateral discipline — matches the investor exactly. Borderline Tier-1 for M1. |
| Positional Option Trading | Euan Sinclair | 1st, 2020 (Wiley) | derivatives | M1 | EN; ISBN 9781119583516; ~240pp. Deepens variance-premium + sizing; overlaps Sinclair 2e — complement, not essential. |
| Get Rich with Dividends | Marc Lichtenfeld | 3rd, 2023 (Wiley) | equity-valuation | M2 | EN; ISBN 9781119985556; ~250pp. Current (2023) rules-based dividend-safety screening; complements Miller's philosophy with executable criteria. |
| The Dhandho Investor | Mohnish Pabrai | 1st, 2007 (Wiley) | behavioral-finance | M2, M4 | EN; ISBN 9780470043899; ~208pp. Asymmetric-payoff + Kelly sizing; short, easy ingest. |
| The Psychology of Money | Morgan Housel | 1st, 2020 (Harriman) | behavioral-finance | M4, M2 | EN; ISBN 9780857197689; ~256pp. Patience/behavior-over-intelligence in plain distillable form. |
| Privatizing China: Inside China's Stock Markets | Walter & Howie | 2nd, 2006 (Wiley) | equity-valuation | M2, M3 | EN; ISBN 9780470822142; ~400pp. Share-class taxonomy (H-share vs red-chip) → the WHT difference. Overlaps Red Capitalism; more dated. |
| HKEX Stock Options Information Sheet + FAQ | HKEX | 2016 (still current) | derivatives | M1, M3 | EN; **FREE HKEX PDF** ~4pp. The actual HK instrument (SEOCH clearing, American-style, margin/MM). |
| SEHK Options Trading Rules (Whole OTR) | SEHK / HKEX | rolling | derivatives | M1, M3 | EN; **FREE HKEX PDF** ~80–150pp. Binding rule text for exercise/assignment/levies/adjustment. |
| SEOCH Operational Clearing Procedures — Ch.14 Capital Adjustments | SEOCH / HKEX | rolling | derivatives | M1, M3 | EN; **FREE HKEX PDF** ~10–15pp. Clearing-side: auto **de-covering** of shares backing short calls on adjustment evening — precise covered-call collateral detail. |
| HKEX Stock Options Product Page — Contract Specs / multiplier / board lot | HKEX | live 2026 | derivatives | M1, M3 | EN; **FREE HKEX web** (dynamic). Per-class lot/multiplier for the investor's exact tickers. Capture as a **dated snapshot table** — specs change. |

---

## Tier-3 optional

| Title | Author / issuer | Edition / year | Framework family | Methods | Ingestibility |
|---|---|---|---|---|---|
| Get Rich with Options | Lee Lowell | 2nd, 2009 (Wiley) | derivatives | M1, M4 | EN; ISBN 9780470445891; ~256pp. Floor-trader premium-selling voice; frames puts as "naked" — softer than the investor's strict-collateral rule. Supplementary. |
| Poor Charlie's Almanack | Munger; ed. Kaufman | Stripe Press abridged, 2023 | behavioral-finance | M4 | EN; ISBN 9781953953230; 384pp. Mental-model latticework; broad, premium edition, least convenient to ingest. |
| Guide on Distribution of Dividends and Other Entitlements | HKEX Listing Dept | rolling | equity-valuation | M2, M3 | EN; **FREE HKEX PDF** ~10–20pp. Issuer-side book-close/ex-date calendar; ties dividend timing to adjustment triggers. |
| HKEX Introductory Guide to the Stock Options Corner | HKEX | current | derivatives | M1 | EN; **FREE HKEX PDF** ~10–15pp. Greeks/IV tooling — already covered by Hull; only HK-tool framing is marginally new. |

---

## Mapping to the two-phase plan

Two phases: **Phase A** distills grounding (framework/theory) cards into the existing `cards/cfa/<framework>/` sub-folders; **Phase B** distills 狗不叫 practitioner cards into a new `cards/hkex/` deck that cites the Xueqiu corpus and cross-links the Phase-A grounding cards. Below, each Tier-1/2 source's grounding-card home + the Phase-B Xueqiu card-themes that will cite it.

**Derivatives → `07_derivatives_and_volatility/` (cross-link the existing Hull theory cards)**
- **McMillan** → grounding cards `deriv-covered-call-writing`, `deriv-put-selling-income`, `deriv-rolling-and-assignment` → cited by Xueqiu themes: *sell-call on owned 00883/00700*, *平倉 + 追加 sell-put rolling on 00700/09988*, *assignment / 接货 decisions*.
- **Ellman (Covered Call; CSP T2)** → `deriv-covered-call-workflow`, `deriv-csp-for-accumulation`, `deriv-exit-roll-decision-rules` → Xueqiu: *moneyness-based strike pick*, *roll-down/out/up*, *"不弄丟底倉" rules-based exits* (also cross-links M4 discipline cards).
- **Natenberg** → `deriv-theta-vs-delta-desk-intuition`, `deriv-short-premium-pnl` → Xueqiu: *theta-collection rationale*, *why sell premium on blue chips*.
- **Sinclair (Vol Trading; Positional T2)** → `deriv-variance-risk-premium`, `deriv-premium-seller-sizing` → Xueqiu: *systematic premium-selling edge*, *position sizing under 100%-collateral*.
- **Bennett** → `deriv-skew-term-structure`, `deriv-dividend-option-pricing-interaction` → Xueqiu: *special-dividend → strike/tenor choice* (bridges into M3 adjustment cards).
- **HKEX Info Sheet / Whole OTR / SEOCH Ch.14 / Contract Specs (T2)** → `deriv-hkex-stock-option-product`, `deriv-hkex-exercise-assignment-levies`, `deriv-hkex-contract-multiplier-board-lot`, `deriv-hkex-decovering-on-adjustment` → Xueqiu: *"一張合約≠100股" multiplier confusion*, *American-style early-exercise*, *covered-call collateral through an adjustment*.

**HK contract adjustment → `07_derivatives_and_volatility/` (the highest-value M3 card)**
- **OTP Ch.8 Special Events** → grounding card `deriv-hkex-special-dividend-adjustment` (2% threshold; adjustment-ratio formula; ≤10-trading-day window) → cited by the **flagship Xueqiu card**: *00883 sell-call strike 12.5 → adjusted 11.31 / 1105 shares on the special dividend* (Post 222375639). Cross-links to Bennett (dividend↔pricing) and SEOCH Ch.14 (clearing-side).

**Microstructure / Stock Connect → `14_microstructure_and_trading/`**
- **Stock Connect Information Book + FAQ** → `mt-hk-stock-connect-eligibility`, `mt-hk-connect-no-options` → Xueqiu: *"港股通沒法做期權"* (a recurring teaching point to Mainland followers).

**HK tax & costs → `02_economics/` (tax) and `14_microstructure_and_trading/` (transaction cost)**
- **IRD DTA Notes + s.26** → `ec-hk-dividend-withholding-regime` (zero HK WHT; 10% PRC on H-shares; red-chip exemption) → Xueqiu: *941 vs 728/762/883 withholding*, *移動 vs 電信 tax differential*, *red-chip (母公司香港注冊) exemption*.
- **IRD Stamp Duty** → `mt-hk-stamp-duty-transaction-cost` (0.1%/side, eff. 17 Nov 2023) → Xueqiu: *cost-aware accumulation / rolling economics*.

**Holding mechanics → `14_microstructure_and_trading/`**
- **IFEC Shares Holding / Physical Certificates (+ HKEX Operating Guide)** → `mt-hk-ccass-nominee-vs-register`, `mt-hk-physical-cert-withdrawal-workflow` → Xueqiu: *the 提取實物 → re-register → 石鼓 tax-exemption maneuver* and the "must re-register to receive dividends directly" step.

**Value doctrine & temperament → `05_equity/` (doctrine) and `10_behavioral_finance/` (temperament)**
- **Graham *Intelligent Investor*** → `eq-margin-of-safety-doctrine`, `be-mr-market-temperament` → Xueqiu: *contrarian accumulation at bottoms*, *"持股心态最重要"*, *margin-of-safety on beaten-down blue chips*.
- **Graham & Dodd *Security Analysis 7e*** → `eq-graham-dodd-safety-of-income`, `eq-dividend-record-earning-power` → Xueqiu: *insurer/SOE valuation reasoning (太保 財產險/壽險 sum-of-parts)*.
- **Miller *Single Best Investment*** → `eq-dividend-growth-discipline` → Xueqiu: *yield + compounding on HK blue chips*.
- **Lichtenfeld (T2)** → `eq-dividend-safety-screening` → Xueqiu: *payout/coverage screening of the dividend universe*.
- **Marks *Most Important Thing*** → `be-second-level-thinking-cycles`, `be-contrarian-patience-temperament` → Xueqiu: *cycle-aware contrarian accumulation*, *patience underpinning the overlay*.
- **Pabrai / Housel (T2)** → `be-asymmetric-payoff-bet-sizing`, `be-patience-behavior-over-intelligence` → Xueqiu: *concentrated balanced book*, *"该加仓的时候跑了去减仓" discipline failures as cautionary*.

**China-SOE institutional context → `05_equity/` (or `02_economics/`)**
- **Red Capitalism** → `eq-china-soe-policy-market-dynamics` → Xueqiu: *why CNOOC / China Mobile / banks / insurers behave as state-controlled dividend instruments* (grounds the M2 stock universe).
- **Privatizing China (T2)** → `eq-china-share-class-taxonomy` (H-share vs red-chip incorporation) → Xueqiu: *the incorporation→withholding link* (reinforces the IRD tax card).

---

## Acquisition & ingestibility notes

**Freely / officially downloadable (no purchase) — prioritize, all English:**
- **HKEX PDFs:** OTP Ch.8 Special Events; Stock Connect Information Book + FAQ; Stock Options Info Sheet; Whole OTR; SEOCH OCP Ch.14; Dividend Distribution guide; Stock Options Corner guide. All free, no auth. **Caveat: HKEX URL paths are volatile** — a previously cited `chap08` path now resolves to an unrelated file. Re-resolve every HKEX link from its parent landing page at ingest time and **pin the dated file**. Contract Specs is a **dynamic web page** — capture a dated snapshot table, not a live URL.
- **IRD / IFEC / GovHK:** DTA Notes, stamp-duty rates PDF + stamping notes, IFEC holding pages. Free; mostly short printable **HTML** (a few pp). HTML ingest is clean text (no OCR risk).
- **Bennett, *Trading Volatility*:** the one **free, author-sanctioned book PDF** (`trading-volatility.com/Trading-Volatility.pdf`, ~316pp). Best ingestibility of the books.
- **Miller, *Single Best Investment*:** author-authorized PDF on mhinvest.com — **verify it is the complete 2nd edition** before ingesting (also on Internet Archive).

**Commercial books the user must obtain (English, no free official PDF):** McMillan, both Ellman titles, Natenberg, both Sinclair titles, Lowell, Graham (*Intelligent Investor*), Graham & Dodd (*Security Analysis 7e*), Lichtenfeld, Pabrai, Housel, Marks (Columbia UP e-book), Munger (Stripe), Red Capitalism, Privatizing China. Obtain as purchased e-book/DRM-removed PDF/eText (this matches the existing corpus's commercial-PDF ingest pattern, e.g. Hull/Damodaran/QRM).

**Image-only / OCR risk:** Born-digital e-book PDF/EPUB (all the Wiley/McGraw-Hill/Harriman/Columbia titles, the HKEX/IRD PDFs) are text-layer clean — low risk and good for the project's byte-stable extraction requirement. **Risks to flag:** (a) any *scanned hardcover* fallback for McMillan/Munger would be image-only and need OCR — avoid, prefer the publisher e-text; (b) **EPUB has no stable pagination** — the registry already excludes EPUBs (`epub_blacklist`), so convert to PDF or treat as page-unstable per the project's existing policy; (c) the dynamic HKEX Contract-Specs page must be snapshotted (not a stable PDF).

**CJK vs English:** All recommended grounding sources are **English** (official HKEX/IRD docs publish authoritative English versions; bilingual variants exist but pin the English file). The **only CJK content is the Xueqiu corpus itself** (Traditional/Simplified mixed, already captured at `_research/xueqiu_goubujiao/`), which is the Phase-B *practitioner-card* source, not a grounding source. So the grounding layer is monolingual-English; the CJK handling burden lives entirely in Phase B.

---

## Honest gaps (flag for raw-Xueqiu-only grounding — no good ingestible source)

These are discussed by the investor but have **no authoritative ingestible source**; the Phase-B cards must cite the Xueqiu corpus directly with a "practitioner-claim, no external grounding" flag:

1. **The cross-border execution logistics of the physical-certificate maneuver** — IFEC/HKEX cover the CCASS-withdrawal + re-registration *workflow*, but the investor's lived detail (which Mainland brokers will authorize a proxy to do the 提倉/過戶, doing it in person in HK, the practical "find a friend to handle it" route) is folk-procedural. He himself says "真不知道" which brokers offer it. **No source covers this; raw-Xueqiu only.**
2. **The economic break-even of cert-withdrawal vs. eating the 10% WHT, per name** — his rule that converting 728 (電信) to physical "變得沒有意義 because it still pays 10%" while 941 (移動, red-chip) becomes exempt is a name-specific arbitrage judgement. The IRD/HKEX sources give the *rates and the registry mechanism*; the *per-ticker decision* is his synthesis. **Ground the rule from IRD + IFEC; flag the per-name verdicts as Xueqiu-only.**
3. **Specific HK-name fundamental theses** — his concrete views (太保 sum-of-parts: 財產險 ≥600億 + 壽險 ≥2000億; 平安 property-debt/trust-rollover risk; 聯邦制藥 debt burden; 中移動 vs 中電信 cloud-business optionality; 銀保 "沒將來") are idiosyncratic stock calls. Graham/Damodaran ground the *method*; **the specific theses are Xueqiu-only.**
4. **The exact contract-multiplier values and odd-lot/碎股 outcomes per HK name through an adjustment** — the *formula* is grounded (OTP Ch.8) and *specs* are snapshot-able (HKEX product page), but the investor's worked examples (00883: 12.5×1000 → 11.31×1105 producing 碎股, 神華/49-strike multiplier surprises) are lived arithmetic. **Ground the rule; cite his worked numbers from Xueqiu.**
5. **His personal exit/roll heuristics as a complete system** — Ellman/McMillan ground *named* rolling strategies, but his specific overlay-on-a-permanent-base philosophy ("不弄丟底倉 below my fair value," collecting cumulative premium across serial rolls e.g. 15→27元 on 00700) is his own discipline. **Ground the building blocks; flag the personal ruleset as practitioner-original.**

**Net:** 15 Tier-1 ingests (≈4 free HK-official PDFs + IRD/IFEC HTML being the decisive M3 gap-closers; the rest commercial books) plus the existing CFA derivatives/equity/behavioral/microstructure cards as the cross-link layer fully ground M1, M2, and M4, and ground M3 to the extent any source can — with the five items above remaining honestly Xueqiu-only.

Relevant absolute paths: ingested-source registry `/home/jakeshea/knowledge_base_framework_discovery/sources/cfa/_registry/source_inventory.json`; existing cross-link card folders under `/home/jakeshea/knowledge_base_framework_discovery/cards/cfa/{07_derivatives_and_volatility,05_equity,10_behavioral_finance,14_microstructure_and_trading,02_economics}/`; Xueqiu practitioner corpus `/home/jakeshea/knowledge_base_framework_discovery/_research/xueqiu_goubujiao/corpus_full.md` (+ `batch_manifest.json`, `batches/`).