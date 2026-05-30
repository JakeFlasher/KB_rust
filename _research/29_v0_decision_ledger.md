# _research/29 — CFA-legacy CACG v0 Decision Ledger

This ledger closes the carried-forward open questions from `_research/28` §7 (Q1–Q12) and records the must-fix rulings for the CFA-legacy CACG **v0 release candidate**. It is the plan-record successor to `_research/28`. A deterministic checker (`sources/cfa_legacy/_registry/check_decision_ledger.py`) enforces that every Q1–Q12 carries a status and that no ruling contradicts DEC-2.

Generated from the Round-8 analyze lane (privacy-mode Claude analysis workflow: doc-28 §7 enumeration + rulings, Tsay/Fabozzi edition audit, 03_FRA freshness, completeness critic).

## v0 Definition of Done

v0 Definition of Done: 268 active emitted cards + 6 notes-taint quarantined cards (274 legacy total). Active cards verified clean on production gate (kb lint --all-readings exit 0; kb verify per-card exit 0). Quarantine frozen (Critical Rule 9 unrelaxed; no notes_provenance on active cards). All doc-28 §7 open questions closed with recorded rulings (DECIDED/IMPLICITLY_SETTLED/DEFERRED). Decision ledger written (_research/29). Byte-stable index proven. Clean tagged v0 release with sidecars/small manifests tracked and large chunks_manifest.json recipe documented. No re-ingest required (Pdfium provenance already recorded).

## §7 Open-Question Dispositions (Q1–Q12)

Each question carries exactly one status: **DECIDED** (resolved this phase), **IMPLICITLY SETTLED** (already true on disk; evidence path given), or **DEFERRED** (linked to a `FUT-*`).

| Question | Status | Evidence / FUT | Ruling |
|----------|--------|----------------|--------|
| Q1 | DECIDED | `sources/cfa_legacy/_registry/rename_decisions.md` | DECIDED: source editions canonicalized per `rename_decisions.md` — Wahlen -> fra_wahlen_baginski_bradshaw_2023_fsa_valuation_10ed, Damodaran -> eq_damodaran_2025_investment_valuation_4ed, Fabozzi -> fi_fabozzi_2021_handbook_fixed_income_9e (edition encoded in the token), Tsay -> qm_tsay_2005_afts_2e. The three renames (Hart/Mas-Colell, Fabozzi, Tsay) were coordinated across manifests/chunks/maps/cards with chunk_hash envelope-invariant. source_id tokens are now FIXED (no further renames, per the no-rename constraint). NOTE: the edition-metadata audit (below) flags a Tsay discrepancy — the token reads 2005/2e while a PDF-metadata audit suggests 3e/2010 — recorded as a metadata correction to verify post-v0; it changes no source_id and no citation. |
| Q2 | IMPLICITLY SETTLED | `sources/cfa_legacy/_registry/excluded_sources.json` | IMPLICITLY SETTLED: the 4 Chinese targeted-books are already excluded on disk (`excluded_sources.json`, quotable:no equivalent) and no active v0 card cites them, so the v0 disposition is fixed = excluded. Replacing them with publisher editions to enable 20_China_CB is a post-v0 acquisition enhancement, not a v0 decision. |
| Q3 | IMPLICITLY SETTLED | `sources/cfa_legacy/_registry/excluded_sources.json` | IMPLICITLY SETTLED: the Portuguese Brooks PDF is already excluded on disk (`excluded_sources.json`, `unregistered_pdf` + `language_quality_risk`) and no active v0 card cites it, so the v0 disposition is fixed = excluded. Acquiring an English edition / bilingual policy is a post-v0 acquisition enhancement. |
| Q4 | DECIDED | `sources/cfa_legacy/_registry/page_coordinate_maps/cfa_2022_l1_combined.json` | DECIDED per DEC-2: For v0, `volume_page` persists as registry-side evidence in page_coordinate_maps (v2.1 schema with per-volume offset tables); card frontmatter citations retain only `pdf_page`. Evidence: all 268 active cards use pdf_page in page_range; no volume_page field exists in card frontmatter. CFA L1 combined.json uses v2.1 per-volume table (6 volumes with verified_evidence triples); 11 single-source v1 maps (plus the v2.1 combined CFA L1 map). Promoting volume_page into frontmatter deferred to FUT-4 (v0.x schema loop). |
| Q5 | IMPLICITLY SETTLED | `sources/cfa_legacy/_registry/snapshot.json` | IMPLICITLY SETTLED: Legacy volume drafts preserved as snapshot only. snapshot.json records legacy_volume_markdown: 20 (draft volumes under legacy_content_manifest.json, not migrated to new-framework). Volume regeneration deferred pending card-content maturity; legacy linkage preserved in legacy_content_manifest for auditability. |
| Q6 | DECIDED | `cards/cfa_legacy/08_convertible_bonds/` | RESOLVED: 08_CB contains exactly 50 cb-*.md active cards (verified: find cards/cfa_legacy/08_convertible_bonds -maxdepth 1 -name '*.md' \| wc -l = 50). Unprefixed README.md is NOT a card and is NOT migrated to cards_manifest.json. The README is preserved as documentation-only in legacy_content_manifest per doc 28 §4.3. Doc 28 figure was stale; actual count 50 cards confirmed on disk and in manifest. |
| Q7 | IMPLICITLY SETTLED | `sources/cfa_legacy/_registry/card_migration_queue.json` | IMPLICITLY SETTLED: v0 preserves the inventory-exact per-subcorpus stance vocabulary — each card keeps its recorded source-stance in `card_migration_queue.json` and the per-slice curated-citation registries keep their N/W/M/E/H verdicts; no collapse to a global enum is performed for v0. |
| Q8 | IMPLICITLY SETTLED | `sources/cfa_legacy/_registry/card_migration_queue.json` | IMPLICITLY SETTLED: only active (emitted) stances are migrated. No active card FRONTMATTER contains a `primary-cfa` field in v0 (the token survives only inside a quoted body annotation on one card, outside the frontmatter fence). The admit-only-defensive stance pattern is preserved in the legacy reference registry (`legacy_source_ref_map.json`, legacy-only, not in the active card schema) plus the per-card `source_stance` in `card_migration_queue.json`. Critical Rule 9 remains unrelaxed; no `notes_provenance` on active cards. |
| Q9 | DEFERRED | FUT-1 | DEFERRED to v0.x per DEC-4. Current state: all 268 active cards have empty card_edges: [] (verified in history.jsonl sidecars). Legacy-edge richer vocabulary (untyped Repo touchpoints + typed edges:) is NOT migrated to cacg `card_edges` which allows only `depends_on`/`extends`. Legacy-edge loss documented in v0 decision ledger; edge unification tracked as FUT-1 (DEC-4) for v1 schema. |
| Q10 | IMPLICITLY SETTLED | `sources/cfa_legacy/_registry/snapshot.json` | IMPLICITLY SETTLED: Migration baseline is local HEAD. snapshot.json records legacy_git_head: 856c4f3cfa9228ac6c4fd4a23e60ee90556b4225 and legacy_git_unpushed_count: 30. The 30 unpushed commits are captured as part of the v0 provenance baseline (line 27 of snapshot.json). All 268 active cards emitted against this baseline. |
| Q11 | DEFERRED | FUT-5 | DEFERRED to v1 schema work. Current state: v0 active scope covers 11 readings (01-11, 17) with 268 cards. Subcorpora 14 (Microstructure), 18, 19 remain unscoped for v0. No active cards in these buckets. Future work: requires schema-grid cleanup decision before enabling these subcorpora. |
| Q12 | DEFERRED | FUT-6 | DEFERRED to v0.x scope-architecture work. Current state: Chinese-CB scope filter applied orthogonally (08_CB and 20_China_CB migrations constrained by Chinese-language parser availability and licensing decisions). No first-class scope-filter schema in v0. Future: requires decision on making Chinese-language scope a core schema concept vs config-only. |

### Per-question detail

**Q1 — DECIDED.**

> Edition canonicalization: adopt on-disk Wahlen 10e (2023), Damodaran 2025 4e, Fabozzi 9e (2021), Tsay 2e (2005), ISLP (Python, 2023) as authoritative? Bootstrap has done 2 (Wahlen, Damodaran), partially done 1 (Fabozzi: matrix flagged but source_id token missing), not done 1 (Tsay).

DECIDED: source editions canonicalized per `rename_decisions.md` — Wahlen -> fra_wahlen_baginski_bradshaw_2023_fsa_valuation_10ed, Damodaran -> eq_damodaran_2025_investment_valuation_4ed, Fabozzi -> fi_fabozzi_2021_handbook_fixed_income_9e (edition encoded in the token), Tsay -> qm_tsay_2005_afts_2e. The three renames (Hart/Mas-Colell, Fabozzi, Tsay) were coordinated across manifests/chunks/maps/cards with chunk_hash envelope-invariant. source_id tokens are now FIXED (no further renames, per the no-rename constraint). NOTE: the edition-metadata audit (below) flags a Tsay discrepancy — the token reads 2005/2e while a PDF-metadata audit suggests 3e/2010 — recorded as a metadata correction to verify post-v0; it changes no source_id and no citation.

**Q2 — IMPLICITLY SETTLED.** *(evidence: `sources/cfa_legacy/_registry/excluded_sources.json`)*

> Chinese licensing: 4 Chinese books in `targeted_books_Chinese/` carry libgen/z-lib marks. Quarantine `quotable: no`, replace with publisher editions, or drop entirely? Affects 08_CB and 20_China_CB migrations.

IMPLICITLY SETTLED: the 4 Chinese targeted-books are already excluded on disk (`excluded_sources.json`, quotable:no equivalent) and no active v0 card cites them, so the v0 disposition is fixed = excluded. Replacing them with publisher editions to enable 20_China_CB is a post-v0 acquisition enhancement, not a v0 decision.

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

DEFERRED to v1 schema work. Current state: v0 active scope covers 11 readings (01-11, 17) with 268 cards. Subcorpora 14 (Microstructure), 18, 19 remain unscoped for v0. No active cards in these buckets. Future work: requires schema-grid cleanup decision before enabling these subcorpora.

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

Audit of Tsay and Fabozzi source-edition metadata: TWO findings reported. (1) qm_tsay_2005_afts_2e: CRITICAL MISMATCH — source_id token falsely claims '2005 2e' when actual edition is Tsay 3e 2010. The inventory metadata is correct (shows 3e 2010 at line 1281), but source_id token at line 1290 is wrong. Recommend updating documentation/metadata to note the discrepancy, but source_id token CANNOT be renamed (would break all citations). (2) qm_tsay_2014_multivariate_time_series: CORRECT — source_id and metadata are aligned (1e 2014). (3) fi_fabozzi_2021_handbook_fixed_income_9e: CORRECT — source_id and metadata are aligned (9e 2021). No source_id renames required. All three sources correctly cited in source_matrix.json; only the first requires a metadata correction note/documentation update to clarify the edition discrepancy.

Constraint honored: **`source_id` tokens are NOT renamed** (renaming would break every citation that binds to them). `any_source_id_rename_needed = False`.

| source_id | Action | Observed edition metadata | Correct edition | Detail |
|-----------|--------|---------------------------|-----------------|--------|
| `qm_tsay_2005_afts_2e` | metadata-correction-recommended | Tsay 3e 2010 Analysis of Financial Time Series (Wiley) [canonical file: qm_tsay_2010_afts.pdf] | FLAGGED: token says 2e/2005; audit suggests 3e/2010 (verify post-v0) | Edition-metadata discrepancy: the source_id token encodes 2005/2e, but an audit of the underlying PDF metadata suggests 3rd edition / 2010. This is a metadata-only discrepancy flagged for post-v0 verification against the PDF; the source_id token is NOT renamed (renaming would break every citation; chunk_hash envelope-invariant), and no v0 citation is affected. doc-28 §7 Q1 lists 'Tsay 2e (2005)'; rename_decisions.md recorded 2005/2e. Reconcile the true edition before any v1 re-ingest. |
| `qm_tsay_2014_multivariate_time_series` | ok | Tsay 1e 2014 Multivariate Time Series Analysis: With R and Financial Applications (Wiley) | Tsay 1e 2014 | Source-id token 'qm_tsay_2014_multivariate_time_series' correctly reflects year (2014) and edition (1e). Metadata is consistent: source_id matches edition date/count. No correction needed. |
| `fi_fabozzi_2021_handbook_fixed_income_9e` | ok | Fabozzi (ed.) 2021 9ed The Handbook of Fixed Income Securities | Fabozzi 9e 2021 | Source-id token 'fi_fabozzi_2021_handbook_fixed_income_9e' correctly embeds year (2021) and edition (9e). Inventory metadata matches (line 1477). Prior discrepancy noted in review_flags (legacy matrix said 2012 8e; PDF metadata confirmed 2021 9e) has been resolved. Metadata is correct and consistent. |

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

Task 32 in progress: AC-8 analyze lane covers doc-28 §7 Q1-Q12 ruling + edition audit + FRA freshness. All 12 questions mapped above with status, evidence paths, and rulings grounded in on-disk artifacts. Edition audit completed per rename_decisions.md (3 renames: Hart/Mas-Colell, Fabozzi, Tsay — all coordinated across manifests/chunks/maps/cards). FRA freshness documented in doc-28 §5.2 lines 515-518: 03_FRA 29 cards need regulatory freshness decisions for HKFRS 7 (stale per 2018 snapshot, later amendments exist), HKAS 32 (possibly stale per 2022 snapshot), HKFRS 9 (current per 2024 snapshot). Wahlen 10e already renamed. Remaining work: task 33 (write _research/29 ledger), task 34 (deterministic checker), gates, release.

