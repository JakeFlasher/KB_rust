# 21_SOTA_Acquisitions_2026_Non_China — Deferred Items (Non-Chinese-CB)

**Deferred**: 2026-05-21. **Total**: 36 PDFs + 1 HTML, ~68 MB.

These items were downloaded by the 2026-05-21 SOTA acquisition pass into `/home/jakeshea/CFA_reading/21_SOTA_Acquisitions_2026/`, but subsequently **deferred to this folder** per the Chinese-CB-focused filter. They lack **strong relations to Chinese convertible bond market and investment**, even though many are SOTA references in their own domains.

**Hard-block status**: All items here are **non-citable** per CLAUDE.md
Critical Rule 8 (added 2026-05-21). Enforcement is automated via
`scripts/kb/lint_cards.py` DEFER-001 (frontmatter) + DEFER-002 (body) and
mirrored in `scripts/kb/build_manifest.py` `_validate_rows`. To re-activate
any item, see the re-activation section below.

**Original-acquisition folder note (post-merge 2026-05-21)**: The Chinese-CB-direct items that survived the filter were subsequently merged into `/home/jakeshea/CFA_reading/20_Chinese_Convertible_Bonds_Research/` (with kebab-case path renames + 2 byte-identical duplicates removed); the standalone `21_SOTA_Acquisitions_2026/` folder no longer exists. The items deferred here remain in this folder unchanged and continue to be hard-blocked.

## Why each subfolder is deferred

| Subfolder | Files | Defer rationale |
|-----------|------:|-----------------|
| `01_BIS_Basel/` | 10 | Global Basel III / AT1 / CoCo regulatory framework. Chinese AT1 CoCo issuance is small; corporate CB market dominates Chinese-CB universe. |
| `02_FSB_Switzerland_Fed/` | 6 | US (SVB / Signature) + Swiss (Credit Suisse) bank-failure post-mortems. Not Chinese-CB. |
| `03_EBA_ECB/` | 0 (README only) | EU AT1/CoCo regulatory framework. Not Chinese. |
| `04_FASB_IASB_FRC/` | 3 | US GAAP (FASB ASUs) + US tax (IRS Pub 1212). Chinese issuers use Chinese GAAP (CAS) closer to IFRS than to US GAAP. |
| `06_NAFMII_Quarterly_and_Sample_Issuer/` | 4 | NAFMII Q2-Q4 reports cover all bond types (corporate credit + financial); not CB-focused. CCXI sample report is generic CITIC Securities issuer rating, not CB-specific. |
| `07_Big4_Practitioner_US_focused/` | 2 | Mayer Brown CB Issuer's Guide is US securities-law focused. RSM Debt Modifications guide is US GAAP. |
| `08_Academic_Papers_global/` | 6 | López de Prado causal-factor (generic methodology), Sîrbu-Pikovsky-Shreve (generic CB pricing theory), FEDS CoCo (US), Kang-Kim-Park-Stulz (Japan 1990s), Mitchell-Pedersen-Pulvino (global CB-arb capacity), Asquith (US CB call policy). |
| `09_OpenAccess_Books_global/` | 2 | Acharya SVB and Beyond (US crisis post-mortem). Signature Methods in Finance (generic ML for derivatives). |
| `10_Damodaran_Global_Dataset/` | 1 (HTML) | Damodaran Country Risk Premium global dataset (includes China but not China-focused). |
| `11_India_Korea/` | 2 | RBI India FCCB Master Direction + ADB Korea Bond Market Guide. Other Asian markets, not Chinese. |
| `12_Industry_Reports_global/` | 1 | SSGA "A Look Under the Convertible Bond Bonnet" — US/global CB market commentary, not Chinese-focused. |

## Re-activation

To restore any item:

1. Move the PDF back from `deferred_books/21_SOTA_Acquisitions_2026_Non_China/<subfolder>/<file>.pdf` to its original location in `21_SOTA_Acquisitions_2026/<subfolder>/` (recreate subfolder if needed).
2. Update `21_SOTA_Acquisitions_2026/README.md` to re-include the file.
3. Update the relevant subfolder `README.md`.
4. If activating for matrix-registered citation: run `python tools/audit_pdf_quality.py` + add row to `_corpus_planning/05_source_matrix.md`.

## Recovery-path note

These items remain on disk so the user can reactivate at zero re-acquisition cost. They are not deleted; only logically deferred from the active Chinese-CB acquisition surface.

## Per-subfolder READMEs preserved

The original subfolder README files (which describe each PDF with source + size + content) are preserved inside each subfolder here. They retain the same content from the original acquisition pass.

## Provenance trail

- Original acquisition: 2026-05-21 via parallel `curl` downloads from BIS, FSB, FINMA, Fed, FASB, IRS, CFA Institute, CMU, NBER, CEPR, Springer OAPEN, Mayer Brown, RSM, SSGA, ADB, LexComply, NAFMII, CSI, Lianhe, etc.
- Defer pass: 2026-05-21 under the Chinese-CB filter requested by the user
- Original folder: `/home/jakeshea/CFA_reading/21_SOTA_Acquisitions_2026/`
- Tracking: `/home/jakeshea/CFA_reading/REMAINING_BOOKS.md` §-deferred
