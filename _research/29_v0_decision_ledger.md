# _research/29 — CFA-legacy CACG v0 Decision Ledger

This ledger closes the carried-forward open questions from `_research/28` §7 (Q1–Q12) and records the must-fix rulings for the CFA-legacy CACG **v0 release candidate**. It is the plan-record successor to `_research/28`. A deterministic checker (`sources/cfa_legacy/_registry/check_decision_ledger.py`) enforces that every Q1–Q12 carries a status and that no ruling contradicts DEC-2.

Generated from the Round-8 analyze lane (privacy-mode Claude analysis workflow: doc-28 §7 enumeration + rulings, Tsay/Fabozzi edition audit, 03_FRA freshness, completeness critic).

## v0 Definition of Done

v0 Definition of Done: 268 active emitted cards + 6 notes-taint quarantined cards (274 legacy total). Active cards verified clean on production gate (kb lint --all-readings exit 0; kb verify per-card exit 0). Quarantine frozen (Critical Rule 9 unrelaxed; no notes_provenance on active cards). All doc-28 §7 open questions closed with recorded rulings (DECIDED/IMPLICITLY_SETTLED/DEFERRED). Decision ledger written (_research/29). Byte-stable index proven. Clean tagged v0 release with sidecars/small manifests tracked and large chunks_manifest.json recipe documented. No re-ingest required (Pdfium provenance already recorded).

v1 Definition of Done (migration of readings 14/15/22): 402 active emitted cards + 6 notes-taint quarantined cards (408 legacy total) across 14 readings — the original 11 (01,02,03,05,06,07,08,09,10,11,17) plus the three migrated readings 14_microstructure_and_trading (73 mt-), 15_performance_and_attribution (35 pa-), and 22_fund_level_arbitrage (26 fa-). The 134 new cards pass the same trust gates as v0 (kb lint --all-readings exit 0; kb verify per-card 402/402; byte-reproducible frozen index re-baselined at 402); the 87-source corpus (70 original + 17 migrated, incl. the O'Hara sandwich) is ingested under the chromium/7778 Pdfium pin; bidirectional See-Also cross-links added (one released card edited: be-limits-of-arbitrage, on the recorded allowlist); the v0-candidate tag is left immutable and a new annotated v1-candidate tag marks this milestone. Schema stays cacg.v0 (corpus milestone bump only; no kernel change). The migration recipe is recorded in _research/31. Q11 (reading scope) is hereby extended: v1 adds 14/15/22, so reading-22 is registered as a deliberate non-canonical reading id (DEC-9).

## §7 Open-Question Dispositions (Q1–Q12)

Each question carries exactly one status: **DECIDED** (resolved this phase), **IMPLICITLY SETTLED** (already true on disk; evidence path given), or **DEFERRED** (linked to a `FUT-*`).

| Question | Status | Evidence / FUT | Ruling |
|----------|--------|----------------|--------|
| Q1 | DECIDED | `sources/cfa_legacy/_registry/rename_decisions.md` | DECIDED: source editions canonicalized per rename_decisions.md — Wahlen -> fra_wahlen_baginski_bradshaw_2023_fsa_valuation_10ed, Damodaran -> eq_damodaran_2025_investment_valuation_4ed, Fabozzi -> fi_fabozzi_2021_handbook_fixed_income_9e (edition encoded in the token), Tsay -> qm_tsay_2005_afts_2e. The three renames (Hart/Mas-Colell, Fabozzi, Tsay) were coordinated across manifests/chunks/maps/cards with chunk_hash envelope-invariant. source_id tokens are now FIXED (no further renames, per the no-rename constraint). Tsay AFTS is the 2nd edition / 2005 (PDF title page reads "Second Edition"; pdfinfo CreationDate 2005); the stale edition metadata in source_inventory.json + legacy_path_map.json was corrected to "Tsay 2e 2005" this round (source_id unchanged). See the edition-metadata audit below. |
| Q2 | DECIDED | `sources/cfa_legacy/_registry/source_inventory.json` | DECIDED: split disposition by registration. The 4 `targeted_books_Chinese` sources are NOT all excluded — v0 ACCEPTS the 3 registered PDFs (`source_inventory.json`: status:active, quotable:yes), actively cited in 08_CB: cb_an_daoquan_2014_magic_book_2ed (11 cards), cb_an_daoquan_2023_three_line_duplex_3ed (19), cb_gongshou_practical_handbook_1ed (10) = 40 citing cards. Only the EPUB `convertible_bonds_targeted_books_chinese_v` is excluded (status:excluded, exclude_reason:epub_blacklist). The legacy targeted_books_Chinese/ folder mark is NOT itself a v0 disqualifier — the registered PDFs carry sha256 + page provenance and pass kb verify; publisher-edition replacement and the 20_China_CB scope are post-v0 (see Q12 -> FUT-6). |
| Q3 | IMPLICITLY SETTLED | `sources/cfa_legacy/_registry/excluded_sources.json` | IMPLICITLY SETTLED: the Portuguese Brooks PDF is already excluded on disk (`excluded_sources.json`, `unregistered_pdf` + `language_quality_risk`) and no active v0 card cites it, so the v0 disposition is fixed = excluded. Acquiring an English edition / bilingual policy is a post-v0 acquisition enhancement. |
| Q4 | DECIDED | `sources/cfa_legacy/_registry/page_coordinate_maps/cfa_2022_l1_combined.json` | DECIDED per DEC-2: For v0, `volume_page` persists as registry-side evidence in page_coordinate_maps (v2.1 schema with per-volume offset tables); card frontmatter citations retain only `pdf_page`. Evidence: all 268 active cards use pdf_page in page_range; no volume_page field exists in card frontmatter. CFA L1 combined.json uses v2.1 per-volume table (6 volumes with verified_evidence triples); 11 single-source v1 maps (plus the v2.1 combined CFA L1 map). Promoting volume_page into frontmatter deferred to FUT-4 (v0.x schema loop). |
| Q5 | IMPLICITLY SETTLED | `sources/cfa_legacy/_registry/snapshot.json` | IMPLICITLY SETTLED: Legacy volume drafts preserved as snapshot only. snapshot.json records legacy_volume_markdown: 20 (draft volumes under legacy_content_manifest.json, not migrated to new-framework). Volume regeneration deferred pending card-content maturity; legacy linkage preserved in legacy_content_manifest for auditability. |
| Q6 | DECIDED | `cards/cfa_legacy/08_convertible_bonds/` | RESOLVED: 08_CB contains exactly 50 cb-*.md active cards (verified: find cards/cfa_legacy/08_convertible_bonds -maxdepth 1 -name '*.md' \| wc -l = 50). Unprefixed README.md is NOT a card and is NOT migrated to cards_manifest.json. The README is preserved as documentation-only in legacy_content_manifest per doc 28 §4.3. Doc 28 figure was stale; actual count 50 cards confirmed on disk and in manifest. |
| Q7 | IMPLICITLY SETTLED | `sources/cfa_legacy/_registry/card_migration_queue.json` | IMPLICITLY SETTLED: v0 preserves the inventory-exact per-subcorpus stance vocabulary — each card keeps its recorded source-stance in `card_migration_queue.json` and the per-slice curated-citation registries keep their N/W/M/E/H verdicts; no collapse to a global enum is performed for v0. |
| Q8 | IMPLICITLY SETTLED | `sources/cfa_legacy/_registry/card_migration_queue.json` | IMPLICITLY SETTLED: only active (emitted) stances are migrated. No active card FRONTMATTER contains a `primary-cfa` field in v0 (the token survives only inside a quoted body annotation on one card, outside the frontmatter fence). The admit-only-defensive stance pattern is preserved in the legacy reference registry (`legacy_source_ref_map.json`, legacy-only, not in the active card schema) plus the per-card `source_stance` in `card_migration_queue.json`. Critical Rule 9 remains unrelaxed; no `notes_provenance` on active cards. |
| Q9 | DEFERRED | FUT-1 | DEFERRED to v0.x per DEC-4. Current state: all 268 active cards have empty card_edges: [] (verified in history.jsonl sidecars). Legacy-edge richer vocabulary (untyped Repo touchpoints + typed edges:) is NOT migrated to cacg `card_edges` which allows only `depends_on`/`extends`. Legacy-edge loss documented in v0 decision ledger; edge unification tracked as FUT-1 (DEC-4) for v1 schema. |
| Q10 | IMPLICITLY SETTLED | `sources/cfa_legacy/_registry/snapshot.json` | IMPLICITLY SETTLED: Migration baseline is local HEAD. snapshot.json records legacy_git_head: 856c4f3cfa9228ac6c4fd4a23e60ee90556b4225 and legacy_git_unpushed_count: 30. The 30 unpushed commits are captured as part of the v0 provenance baseline (line 27 of snapshot.json). All 268 active cards emitted against this baseline. |
| Q11 | DEFERRED | FUT-5 | DEFERRED to v1 schema work. Current state: v0 active scope covers exactly 11 readings — 01, 02, 03, 05, 06, 07, 08, 09, 10, 11, 17 (note: NOT 04) — with 268 cards. Subcorpora 14 (Microstructure), 18, 19 remain unscoped for v0. No active cards in these buckets. Future work: requires schema-grid cleanup decision before enabling these subcorpora. |
| Q12 | DEFERRED | FUT-6 | DEFERRED to v0.x scope-architecture work. Current state: Chinese-CB scope filter applied orthogonally (08_CB and 20_China_CB migrations constrained by Chinese-language parser availability and licensing decisions). No first-class scope-filter schema in v0. Future: requires decision on making Chinese-language scope a core schema concept vs config-only. |

### Per-question detail

**Q1 — DECIDED.**

> Edition canonicalization: adopt on-disk Wahlen 10e (2023), Damodaran 2025 4e, Fabozzi 9e (2021), Tsay 2e (2005), ISLP (Python, 2023) as authoritative? Bootstrap has done 2 (Wahlen, Damodaran), partially done 1 (Fabozzi: matrix flagged but source_id token missing), not done 1 (Tsay).

DECIDED: source editions canonicalized per rename_decisions.md — Wahlen -> fra_wahlen_baginski_bradshaw_2023_fsa_valuation_10ed, Damodaran -> eq_damodaran_2025_investment_valuation_4ed, Fabozzi -> fi_fabozzi_2021_handbook_fixed_income_9e (edition encoded in the token), Tsay -> qm_tsay_2005_afts_2e. The three renames (Hart/Mas-Colell, Fabozzi, Tsay) were coordinated across manifests/chunks/maps/cards with chunk_hash envelope-invariant. source_id tokens are now FIXED (no further renames, per the no-rename constraint). Tsay AFTS is the 2nd edition / 2005 (PDF title page reads "Second Edition"; pdfinfo CreationDate 2005); the stale edition metadata in source_inventory.json + legacy_path_map.json was corrected to "Tsay 2e 2005" this round (source_id unchanged). See the edition-metadata audit below.

**Q2 — DECIDED.**

> Chinese licensing: 4 Chinese books in `targeted_books_Chinese/` carry libgen/z-lib marks. Quarantine `quotable: no`, replace with publisher editions, or drop entirely? Affects 08_CB and 20_China_CB migrations.

DECIDED: split disposition by registration. The 4 `targeted_books_Chinese` sources are NOT all excluded — v0 ACCEPTS the 3 registered PDFs (`source_inventory.json`: status:active, quotable:yes), actively cited in 08_CB: cb_an_daoquan_2014_magic_book_2ed (11 cards), cb_an_daoquan_2023_three_line_duplex_3ed (19), cb_gongshou_practical_handbook_1ed (10) = 40 citing cards. Only the EPUB `convertible_bonds_targeted_books_chinese_v` is excluded (status:excluded, exclude_reason:epub_blacklist). The legacy targeted_books_Chinese/ folder mark is NOT itself a v0 disqualifier — the registered PDFs carry sha256 + page provenance and pass kb verify; publisher-edition replacement and the 20_China_CB scope are post-v0 (see Q12 -> FUT-6).

**Q3 — IMPLICITLY SETTLED.** *(evidence: `sources/cfa_legacy/_registry/excluded_sources.json`)*

> Portuguese Brooks Reversals: quarantine pending English edition, or accept bilingual citation with `language: pt` flag? Currently excluded as `unregistered_pdf` with `language_quality_risk` flag.

IMPLICITLY SETTLED: the Portuguese Brooks PDF is already excluded on disk (`excluded_sources.json`, `unregistered_pdf` + `language_quality_risk`) and no active v0 card cites it, so the v0 disposition is fixed = excluded. Acquiring an English edition / bilingual policy is a post-v0 acquisition enhancement.

**Q4 — DECIDED.**

> Combined-volume citation form: persist both `volume_page` and `pdf_page` in cacg citations, or just one?

DECIDED per DEC-2: For v0, `volume_page` persists as registry-side evidence in page_coordinate_maps (v2.1 schema with per-volume offset tables); card frontmatter citations retain only `pdf_page`. Evidence: all 268 active cards use pdf_page in page_range; no volume_page field exists in card frontmatter. CFA L1 combined.json uses v2.1 per-volume table (6 volumes with verified_evidence triples); 11 single-source v1 maps (plus the v2.1 combined CFA L1 map). Promoting volume_page into frontmatter deferred to FUT-4 (v0.x schema loop).

**Q5 — IMPLICITLY SETTLED.** *(evidence: `sources/cfa_legacy/_registry/snapshot.json`)*

> Volume drafts disposition: regenerate from new-framework cards (drop legacy drafts), preserve as snapshot, or port + re-link?

IMPLICITLY SETTLED: Legacy volume drafts preserved as snapshot only. snapshot.json records legacy_volume_markdown: 20 (draft volumes under legacy_content_manifest.json, not migrated to new-framework). Volume regeneration deferred pending card-content maturity; legacy linkage preserved in legacy_content_manifest for auditability.

**Q6 — DECIDED.**

> 08_CB 50-vs-51 drift: confirmed as unprefixed README.md. Migrate as docs-only?

RESOLVED: 08_CB contains exactly 50 cb-*.md active cards (verified: find cards/cfa_legacy/08_convertible_bonds -maxdepth 1 -name '*.md' | wc -l = 50). Unprefixed README.md is NOT a card and is NOT migrated to cards_manifest.json. The README is preserved as documentation-only in legacy_content_manifest per doc 28 §4.3. Doc 28 figure was stale; actual count 50 cards confirmed on disk and in manifest.

**Q7 — IMPLICITLY SETTLED.** *(evidence: `sources/cfa_legacy/_registry/card_migration_queue.json`)*

> Per-subcorpus FM006 stance vocabulary: preserve inventory-exact admit-sets, or collapse to global enum?

IMPLICITLY SETTLED: v0 preserves the inventory-exact per-subcorpus stance vocabulary — each card keeps its recorded source-stance in `card_migration_queue.json` and the per-slice curated-citation registries keep their N/W/M/E/H verdicts; no collapse to a global enum is performed for v0.

**Q8 — IMPLICITLY SETTLED.** *(evidence: `sources/cfa_legacy/_registry/card_migration_queue.json`)*

> `primary-cfa` admit-only-defensive pattern: preserve, or migrate only active stances?

IMPLICITLY SETTLED: only active (emitted) stances are migrated. No active card FRONTMATTER contains a `primary-cfa` field in v0 (the token survives only inside a quoted body annotation on one card, outside the frontmatter fence). The admit-only-defensive stance pattern is preserved in the legacy reference registry (`legacy_source_ref_map.json`, legacy-only, not in the active card schema) plus the per-card `source_stance` in `card_migration_queue.json`. Critical Rule 9 remains unrelaxed; no `notes_provenance` on active cards.

**Q9 — DEFERRED.** *(deferred → FUT-1)*

> Edge unification: cacg `card_edges` is narrower than legacy. Drop, map, or omit?

DEFERRED to v0.x per DEC-4. Current state: all 268 active cards have empty card_edges: [] (verified in history.jsonl sidecars). Legacy-edge richer vocabulary (untyped Repo touchpoints + typed edges:) is NOT migrated to cacg `card_edges` which allows only `depends_on`/`extends`. Legacy-edge loss documented in v0 decision ledger; edge unification tracked as FUT-1 (DEC-4) for v1 schema.

**Q10 — IMPLICITLY SETTLED.** *(evidence: `sources/cfa_legacy/_registry/snapshot.json`)*

> 30 unpushed local commits: migration baseline is local HEAD or remote tip?

IMPLICITLY SETTLED: Migration baseline is local HEAD. snapshot.json records legacy_git_head: 856c4f3cfa9228ac6c4fd4a23e60ee90556b4225 and legacy_git_unpushed_count: 30. The 30 unpushed commits are captured as part of the v0 provenance baseline (line 27 of snapshot.json). All 268 active cards emitted against this baseline.

**Q11 — DEFERRED.** *(deferred → FUT-5)*

> Subcorpora 14/18/19: retire from `DEFERRED_TOPICS.md`?

DEFERRED to v1 schema work. Current state: v0 active scope covers exactly 11 readings — 01, 02, 03, 05, 06, 07, 08, 09, 10, 11, 17 (note: NOT 04) — with 268 cards. Subcorpora 14 (Microstructure), 18, 19 remain unscoped for v0. No active cards in these buckets. Future work: requires schema-grid cleanup decision before enabling these subcorpora.

**Q12 — DEFERRED.** *(deferred → FUT-6)*

> Scope-filter inheritance: encode Chinese-CB filter as first-class scope concept, or orthogonal config?

DEFERRED to v0.x scope-architecture work. Current state: Chinese-CB scope filter applied orthogonally (08_CB and 20_China_CB migrations constrained by Chinese-language parser availability and licensing decisions). No first-class scope-filter schema in v0. Future: requires decision on making Chinese-language scope a core schema concept vs config-only.

## DEC-2 — Combined-volume citation form / `volume_page` ruling

DEC-2: Combined-volume `volume_page` placement for v0. RESOLVED (user decision): For v0, `volume_page` persists as registry-side evidence in page_coordinate_maps (v2.1 schema for CFA L1 with per-volume offset tables; v1 schema for single-source PDFs); the card citation coordinate remains `pdf_page` only. All 268 active cards follow this pattern. Promoting `volume_page` into the card citation frontmatter schema (requiring coordinated re-emit of hash/emitter/verify) is deferred to FUT-4 (v0.x schema loop dedicated to citation-form enrichment). This keeps v0 with a small, clean surface while improving auditability as a forward-looking v1 concern.

**v0 stance (load-bearing):** for v0, `volume_page` lives as registry-side evidence inside `sources/cfa_legacy/_registry/page_coordinate_maps/`; the card citation coordinate remains `pdf_page` (the chunk-pinned coordinate every `kb verify` checks). Promoting `volume_page` into card frontmatter is **deferred to FUT-4**. No v0 card frontmatter carries a `volume_page` field.

<!-- DEC-2-MACHINE-CHECK: authoritative key/value block parsed by check_decision_ledger.py; flipping a value flips the gate. -->
```
v0_card_citation_coordinate: pdf_page
v0_volume_page_placement: registry-side
volume_page_in_card_frontmatter: false
```

## Source-edition metadata audit (Tsay / Fabozzi)

Tsay/Fabozzi source-edition metadata audit. (1) qm_tsay_2005_afts_2e: the source_id is CORRECT — Tsay 2nd edition / 2005 (PDF title page "Second Edition", pdfinfo CreationDate 2005). The stale edition metadata in source_inventory.json + legacy_path_map.json was corrected this round to "Tsay 2e 2005" so both registries agree with the PDF. (2) qm_tsay_2014_multivariate_time_series: correct (1e/2014). (3) fi_fabozzi_2021_handbook_fixed_income_9e: correct (9e/2021). No source_id renames (renaming would break every citation); only metadata values were corrected.

Constraint honored: **`source_id` tokens are NOT renamed** (renaming would break every citation that binds to them). `any_source_id_rename_needed = False`.

| source_id | Action | Observed edition metadata | Correct edition | Detail |
|-----------|--------|---------------------------|-----------------|--------|
| `qm_tsay_2005_afts_2e` | metadata-corrected | stale edition string in source_inventory.json + legacy_path_map.json (since corrected) | Tsay 2e / 2005 | CORRECTED this round. Direct PDF evidence: the title page reads "Second Edition"; pdfinfo reports CreationDate 2005 (the later 2010 timestamp is only a re-save ModDate). rename_decisions.md already recorded this. The stale edition strings in source_inventory.json (edition field) and legacy_path_map.json (edition field) were corrected to "Tsay 2e 2005". source_id qm_tsay_2005_afts_2e is unchanged (no rename; no citation/chunk change). Both registries now read 2e/2005 and agree with the PDF. |
| `qm_tsay_2014_multivariate_time_series` | ok | Tsay 1e 2014 Multivariate Time Series Analysis: With R and Financial Applications (Wiley) | Tsay 1e 2014 | Source-id token 'qm_tsay_2014_multivariate_time_series' correctly reflects year (2014) and edition (1e). Metadata is consistent: source_id matches edition date/count. No correction needed. |
| `fi_fabozzi_2021_handbook_fixed_income_9e` | ok | Fabozzi (ed.) 2021 9ed The Handbook of Fixed Income Securities | Fabozzi 9e 2021 | Source-id token 'fi_fabozzi_2021_handbook_fixed_income_9e' correctly embeds year (2021) and edition (9e). Inventory metadata matches (line 1477). Prior discrepancy noted in review_flags (legacy matrix said 2012 8e; PDF metadata confirmed 2021 9e) has been resolved. Metadata is correct and consistent. |

<!-- LEDGER-MACHINE-CHECK: authoritative key/value facts parsed by check_decision_ledger.py and cross-checked against on-disk registries + active card citations; flipping a value (or disk disagreeing) flips the gate. -->
```
tsay_afts_edition: 2e
tsay_afts_year: 2005
q2_targeted_chinese_all_excluded: false
q2_targeted_chinese_active_cited: cb_an_daoquan_2014_magic_book_2ed, cb_an_daoquan_2023_three_line_duplex_3ed, cb_gongshou_practical_handbook_1ed
q2_excluded_chinese_source_id: convertible_bonds_targeted_books_chinese_v
q2_excluded_chinese_reason: epub_blacklist
```

## 03_FRA regulatory-freshness note (HKFRS / HKAS)

The 03_FRA slice cites three Hong Kong Financial Reporting Standards / Accounting Standards (HKFRS/HKAS) across compound-instrument accounting cards. HKFRS 7 (2018) on disclosure is current but has seen post-2018 amendments to the IASB equivalent that may not be fully reflected in the HK versions cited; HKAS 32 (2022) is the current version and its core compound-instrument mechanics remain normative; HKFRS 9 (2024) is the latest version and its bifurcation rules are current. No supersessions detected on-disk. Recommendation: v0 should note that amendment timelines for IASB equivalents (which HK standards track) are not fully documented in the knowledge base; caveat compound-instrument disclosures and classification boundaries as 'current per 2024 HKFRS 9 + 2022 HKAS 32 + 2018 HKFRS 7 framework' with a note that Hong Kong standards are aligned with IASB standards but amendment effective dates should be independently verified against current IFRS guidance."

Standards cited by 03_FRA: `HKFRS 7 (2018)`, `HKAS 32 (2022)`, `HKFRS 9 (2024)`.

| Standard | Freshness | Note |
|----------|-----------|------|
| HKFRS 7 (2018) — Financial Instruments: Disclosures | current (as adopted in HK with IASB equivalents) | 2018 version is current; Hong Kong Financial Reporting Standards track IASB IFRS standards. IFRS 7 has had amendments post-2018 (e.g., IFRS 7 2020 on interest rate benchmark reforms) but the core structure remains in force. No on-disk evidence of effective dates; recommend documenting amendment timeline for full v0 caveat. |
| HKAS 32 (2022) — Financial Instruments: Presentation | current (amended-since) | 2022 version is the current issued version for HK standards. This version incorporates amendments to the IASB equivalent (IAS 32) through 2022. The compound-instrument split mechanics (paras 28-32) cited in cards are stable and not superseded. However, IAS 32 has been amended several times since initial issue; full amendment status unclear from on-disk evidence. Recommend verifying amendment history for comprehensive caveat. |
| HKFRS 9 (2024) — Financial Instruments | current | 2024 version is the latest published iteration. HKFRS 9 embedded-derivative rules (section 4.3 cited in cards) are current and track IASB IFRS 9. Effective date for the standard is 1 January 2018 globally; the 2024 version reflects any post-2018 amendments. No on-disk evidence of supersession. Cards cite embedding mechanics that remain normative. |

## Notes-taint quarantine reconciliation

8 legacy cards carry `notes_taint=true` in `card_migration_queue.json`; 2 of them (`cc-material-info-and-dissemination-delay`, `ec-currency-exchange-rates-and-parity`) were re-anchored to non-notes primary sources and emitted (flagged in the scope ledger as `notes_taint_flag_emitted_for_review`), and the remaining **6** stay quarantined (AC-7). So FUT-2's 6 quarantined cards and the queue's 8 notes_taint flags are consistent.

## Future-Work links (with Source DEC)

Each `FUT-*` carries its `Source DEC` inline. FUT-1/2/4 originate in the plan; FUT-5/FUT-6 are introduced by this ledger (doc-29) for the two genuinely-deferred §7 questions (Q11, Q12). `DEC-*` references point to the `.humanize/plans/cfa-v0-stabilization-plan.md` Decisions registry (DEC-2 = volume_page; DEC-4 → FUT-1 edge-unification; DEC-5 → FUT-2 notes-taint re-authoring) — NOT the older, differently-numbered `_research/09` proposals.

- **FUT-1** — Edge unification — represent the richer legacy typed/untyped edges (cacg `card_edges` currently allows only `depends_on`/`extends`). Source DEC: DEC-4. Current-loop handoff: AC-8 (the ledger documents the deferral and the legacy-edge loss). Promotion trigger: a v1 schema loop that extends `CardEdgeType` to include legacy predicates (typed Repo touchpoints + edges vocabulary) and implements the mapping logic.
- **FUT-2** — Re-author the 6 notes-tainted quarantined cards from non-notes sources. Source DEC: DEC-5. Current-loop handoff: AC-7 (the ledger records each card's re-authoring criterion). Promotion trigger: replacement primary sources identified, or a sanctioned notes-provenance policy. Quarantined cards: rm-historical-simulation-var, rm-monte-carlo-var, rm-parametric-var, rm-risk-objectives-and-tolerance, rm-sensitivity-versus-simulation, pm-tracking-error-and-active-risk.
- **FUT-4** — Promote `volume_page` into the card citation frontmatter schema (and the dependent hash/emitter/verify/tests). Source DEC: DEC-2. Current-loop handoff: AC-8 (the ledger records the v0 registry-side evidence policy and this promotion path). Promotion trigger: a v0.x schema loop dedicated to citation-form enrichment. This will require: (1) adding volume_page field to Citation frontmatter, (2) updating citation_hash computation envelope, (3) extending emitter to bind volume_page from page_coordinate_maps, (4) updating verify layer to cross-check volume_page against map evidence, (5) full citation re-binding for all affected cards.
- **FUT-5** — Decide retirement of subcorpora 14 (Microstructure) / 18 / 19 from `DEFERRED_TOPICS.md`. Source DEC: doc-29 (this ledger). Current-loop handoff: AC-8 records the deferral; no active v0 cards exist in these buckets. Promotion trigger: a v1 scope expansion beyond the current 11 active readings (01–11, 17).
- **FUT-6** — Decide whether the Chinese-CB scope filter becomes a first-class scope concept or stays orthogonal config. Source DEC: doc-29 (this ledger). Current-loop handoff: AC-8 records the deferral; v0 applies the filter via `excluded_sources.json` + source-matrix authorization. Promotion trigger: a v1 schema loop that formalizes scope-filter inheritance.

## Notes

This is the final v0 decision ledger. All 12 doc-28 §7 questions are statused with rulings re-derived from on-disk artifacts (source_inventory.json, active card citations, PDF title-page/pdfinfo, the registries named per row), not from narrative summaries. The deterministic checker sources/cfa_legacy/_registry/check_decision_ledger.py (re-runnable, --self-test) gates this file.

