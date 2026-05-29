# CFA Knowledge Library

Canonical PDF library for the CFA legacy KB migration. **70 active +
36 deferred + 6 excluded = 112 items**, organized by status × topic ×
role for efficient navigation when revising migrated knowledge cards.

The PDFs themselves are **not tracked in git** (large binaries; ~1.5 GB
total). What IS tracked: the directory structure, this README, the
machine-readable `_catalog.json`, and the per-subdir README files
inherited from the legacy KB's deferred-books inventory.

## Why this library exists

The legacy KB at `/home/jakeshea/CFA_reading/CFA_reading/` is the
immutable read-only source-of-record. Its on-disk layout has acquisition
artifacts (z-library / libgen filename rot, doubled spaces, non-ASCII
Chinese names with `饕饕→饕餮` typos, etc.) and mixes user-volatile
content (`notes/`) with knowledge sources. The migration target
(`cards/cfa_legacy/<reading>/*.md`) cites legacy PDFs by `source_id`,
not by filesystem path — but humans revising cards still need to **find
the PDF**, **find the relevant section**, and **verify a quote**.

This library is the navigation layer for that workflow:

- ASCII canonical filenames (no libgen marks; no doubled spaces; no CJK
  smart quotes in filenames).
- Stable directory structure by subcorpus.
- Single canonical home per source — Hull lives in `shared_anchors/`,
  not duplicated across `06_/`, `07_/`, `08_/`.
- Co-located metadata (`_catalog.json`) with SHA-256, page count, role,
  audit rating, and review flags so you don't have to bounce between
  manifests to find what you need.
- Deferred + excluded items are PRESERVED in clearly-labeled subdirs
  rather than scattered or omitted, so context is preserved for future
  re-evaluation.

## Tree

```
sources/cfa_knowledge_library/
├── README.md                  # this file
├── _catalog.json              # machine-readable index (sha256, sizes, metadata)
├── .gitignore                 # rules to keep PDFs untracked
│
├── active/                    # 70 quotable PDFs admitted by the source matrix
│   ├── 01_quantitative_methods/        # 6 PDFs (Tsay 2e, Tsay multivariate,
│   │                                      ESL 2e, ISLP 2023, Wooldridge intro 8e,
│   │                                      Greene 8e Global)
│   ├── 02_economics/                   # 5 PDFs (MWG 1995, Romer 5e, Hart-MasColell
│   │                                      2013, MasColell GE+GT, Cochrane FTPL)
│   ├── 03_financial_reporting_analysis/ # 9 PDFs (3 HKICPA + Wahlen 10e 2023,
│   │                                      Penman 5e, Kieso 4e IFRS, Robinson 4e,
│   │                                      Schilit 4e, White 3e)
│   ├── 05_equity/                      # 1 PDF (Damodaran 4e 2025)
│   ├── 06_fixed_income_and_credit/     # 7 PDFs (Brigo-Mercurio, Crepey, Davidson-
│   │                                      Levin, Duffie-Singleton, Fabozzi 9e
│   │                                      2021, Tuckman-Serrat 3e, Veronesi)
│   ├── 07_derivatives_and_volatility/  # EMPTY (cards anchor on shared_anchors/
│   │                                      Hull/Glasserman + CFA L1)
│   ├── 08_convertible_bonds/           # 8 PDFs (Calamos, DeSpiegeleer, Philips,
│   │                                      Thorp-Kassouf, Zubulake, AnDaoquan 2e+3e,
│   │                                      Gongshou practical handbook)
│   ├── 09_portfolio_management_and_asset_pricing/  # 2 PDFs (Cochrane 2005,
│   │                                                  Pedersen 2015)
│   ├── 10_behavioral_finance/          # 2 PDFs (Shleifer 2000, Gennaioli-
│   │                                      Shleifer 2018)
│   ├── 11_risk_management/             # 1 PDF (McNeil-Frey-Embrechts QRM 2015 rev)
│   ├── 17_cross_cutting/               # EMPTY (cards anchor on CFA L1 Ethics Vol.6)
│   ├── cfa_curriculum/                 # 3 PDFs (CFA L1 2022, L2 2023, L3 2022 —
│   │                                      combined-volume PDFs; see N1 in
│   │                                      _research/29 for V6 junk-tail caveat)
│   ├── china_convertible_bonds/        # 19 PDFs (CSDC/CSRC/PBOC/SSE/SZSE/HKEX
│   │                                      exchange rules, SPC court judgments,
│   │                                      sell-side research from Lianhe/NAFMII/
│   │                                      Dajia/China Galaxy/Clifford Chance)
│   ├── shared_anchors/                 # 4 PDFs (Hull 11e 2022, Glasserman MC
│   │                                      methods, Lando credit risk, Koziol CB
│   │                                      valuation) — cited across 06/07/08
│   └── trading_price_action/           # 3 PDFs (Brooks 2009 Reading Charts;
│                                          2012 Trends; 2012 Trading Ranges)
│
├── deferred/                  # 36 PDFs hard-blocked per Critical Rule 8 of the
│   │                            legacy KB. Acquired in the 2026-05-21 SOTA pass
│   │                            but failed the Chinese-CB relevance filter. Kept
│   │                            here for future re-evaluation when scope expands.
│   └── 21_sota_acquisitions_2026_non_china/
│       ├── README.md          # rationale per-subfolder
│       ├── 01_bis_basel/      # 10 PDFs (BIS / Basel Committee on Banking
│       │                        Supervision: AT1, CoCo, CET1, capital framework)
│       ├── 02_fsb_switzerland_fed/   # 6 PDFs (FSB + Swiss FINMA + Fed post-
│       │                                mortems: SVB, Signature, Credit Suisse)
│       ├── 03_eba_ecb/                # EMPTY (acquisition failed HTTP 502)
│       ├── 04_fasb_iasb_frc/          # 3 PDFs (US-GAAP / IRS regs)
│       ├── 06_nafmii_quarterly/       # 4 PDFs (NAFMII bond reports — not CB-
│       │                                specific)
│       ├── 07_big4_practitioner/      # 2 PDFs (US securities law focus)
│       ├── 08_academic_papers/        # 6 PDFs (CB/CoCo pricing theory, US/JP/
│       │                                global empirics)
│       ├── 09_openaccess_books/       # 2 PDFs (crisis post-mortem; ML-for-deriv)
│       ├── 10_damodaran_global_dataset/  # 1 HTML (Country Risk Premium snapshot)
│       ├── 11_india_korea/            # 2 PDFs (Asian markets ex China)
│       └── 12_industry_reports/       # 1 PDF (US/global CB commentary)
│
└── excluded/                  # 6 items: non-quotable per various rules
    ├── epub_blacklist/        # 2 EPUBs (Critical Rule 4 / DEC-2 — no stable
    │                              page anchors): Maitland 2022 CB Securities,
    │                              taotaohai/dingfengbo/youmei 2020 攻守 manual
    ├── scan_nonquotable/      # 1 PDF (Wooldridge Cross/Panel 2e — image-only
    │                              scan, 0.0 cpp, no text layer)
    ├── portuguese_translation/  # 1 PDF (Brooks 2012 Reversals — Portuguese
    │                                translation; quarantined pending English
    │                                Wiley edition)
    └── notes_user_volatile/   # 2 PDFs (CFA_note_2.pdf + .ocr.pdf — hand-written
                                  study notes; Critical Rule 9 forbids citation
                                  as Primary/Supporting source)
```

## Workflow: revising a migrated knowledge card

1. **Locate the card**: `cards/cfa_legacy/<reading_id>/<card_id>.md`
2. **Identify cited source**: card's frontmatter `citations[].source_id`
   gives the canonical id (e.g., `cfa_2022_l1_combined`,
   `pm_pedersen_2015_efficiently_inefficient`).
3. **Find the PDF**: look up `source_id` in `_catalog.json` → get
   `library_path` (e.g., `active/cfa_curriculum/cfa_2022_l1_combined.pdf`).
4. **Locate the cited page**:
   - For combined-volume CFA PDFs, use the per-volume offset table at
     `sources/cfa_legacy/_registry/page_coordinate_maps/cfa_2022_l1_combined.json`.
     The helper module `volume_page_map.py` provides
     `vol_page_to_pdf_page(volume, volume_page) -> pdf_page`.
   - For single-volume PDFs, the card's `citations[].page_range` is the
     PDF page index directly (may need source-specific offset for some).
5. **Verify the quote**: card's `citations[].quote` should appear verbatim
   on the cited pages. The chunks_manifest at
   `out/cfa_legacy/chunks_manifest.json` resolves chunk_id →
   chunk text byte-by-byte.
6. **Update the card**: edit `cards/cfa_legacy/.../<card>.md`, then run
   `kb index cards --out out/cfa_legacy` followed by `kb verify <card>`
   to regenerate card_hash and confirm the citation chain.

## How this library relates to other parts of the workspace

| Workspace location | What it holds | Relationship to library |
|---|---|---|
| `/home/jakeshea/CFA_reading/CFA_reading/` | Legacy KB (immutable) | Source of all PDFs; not modified by this workspace |
| `sources/cfa_legacy/pdfs/` | Bootstrap-staged PDFs (70 quotable + 4 shared_anchors copies) | Subset of library's `active/`; populated by Bucket 4. Will be deprecated once card pipelines migrate to library paths. |
| **`sources/cfa_knowledge_library/`** | **THIS library** (canonical 112 PDFs)| **Authoritative navigation layer going forward** |
| `sources/cfa_legacy/_registry/` | Manifests + scripts (legacy_path_map, source_matrix, etc.) | Reference metadata; library's `_catalog.json` cross-references these |
| `out/cfa_legacy/sources_manifest.json` | Ingested source metadata (page_count, parser provenance) | Library catalog mirrors page_count + other ingest-time facts |
| `out/cfa_legacy/chunks_manifest.json` | Chunk-level text (57,603 chunks) | Cards cite chunks; library is the PDF-level upstream |
| `cards/cfa_legacy/<reading>/*.md` | Migrated cacg.v0 cards | Cite library PDFs via source_id |

## Critical caveats for revising cards

1. **CFA L1 combined PDF has 512 pages of non-CFA junk after the Wiley EULA**
   (PDF pages 3842–4353 contain unrelated content from a flawed iLovePDF
   concatenation). The offset table's V6 `last_pdf_page` is correctly capped
   at 3841. If a card cites Vol.6/p.573 or higher, the citation is invalid
   regardless of how it converts.

2. **Wrong-volume legacy citations are common**: per Doc-29 review,
   13 of 18 09_PM cards had legacy Vol.6 citations that should have been
   Vol.5 (or Vol.4 for `pm-market-efficiency-core`). Always cross-check
   PDF content against the cited topic; the offset table mechanically
   converts whatever you give it.

3. **Soft-hyphen PDF artifacts**: pdftotext extraction sometimes inserts
   a U+FFFE character mid-word (e.g., `rela￾tion`). When picking quotes,
   ensure the verbatim match handles or avoids these.

4. **Notes-taint citations**: legacy KB has 8 cards quarantined for
   using the bare-alias `CFA_note_2 (2026 OCR) pp.X-Y` form. Scrubbed
   versions live at `_legacy_reference/cfa_legacy/cards/`. Never re-introduce
   these citations during card revision.

5. **Deferred + excluded items**: do NOT cite from these subdirs in any
   cacg.v0 card. The catalog records them for future re-evaluation only.

## Maintenance

To regenerate this library from the legacy KB (e.g., after a legacy KB
refresh):

1. Verify legacy KB git state matches `snapshot.json:legacy_git_head`.
2. Run the copy/canonicalize logic (currently inline in the bootstrap
   registry; a future `scripts/build_cfa_knowledge_library.py` should
   formalize it).
3. Re-run `_catalog.json` generation against the new tree.
4. Diff the new catalog against the committed one (`git diff
   _catalog.json`) to surface any source additions/removals/SHA changes.

If the library catalog reports SHA-256 differences from the
`sources_manifest.json` it would indicate a legacy KB drift that must be
investigated before any new card emission.
