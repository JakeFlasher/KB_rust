# Rename and Metadata Decisions

- `03_Financial_Reporting_Analysis/HKICPA_HKAS_32_Financial_Instruments_Presentation_2022.pdf` -> `sources/cfa/pdfs/03_financial_reporting_analysis/fra_hkicpa_hkas_32_2022.pdf`
  - current_version_sensitive: local HKAS 32 revision may be stale
- `03_Financial_Reporting_Analysis/HKICPA_HKFRS_9_Financial_Instruments_2024.pdf` -> `sources/cfa/pdfs/03_financial_reporting_analysis/fra_hkicpa_hkfrs_9_2024.pdf`
  - current_version_sensitive: later HKFRS 9/HKFRS 7 amendments exist
- `03_Financial_Reporting_Analysis/HKICPA_HKFRS_7_Financial_Instruments_Disclosures_2018.pdf` -> `sources/cfa/pdfs/03_financial_reporting_analysis/fra_hkicpa_hkfrs_7_2018.pdf`
  - current_version_sensitive: local HKFRS 7 revision is likely stale
- `03_Financial_Reporting_Analysis/Stickney_Brown_Wahlen_2013_FRA_and_Valuation_8ed.pdf` -> `sources/cfa/pdfs/03_financial_reporting_analysis/fra_wahlen_baginski_bradshaw_2023_fsa_valuation_10ed.pdf`
  - metadata_corrected: legacy filename/matrix said Stickney+Brown+Wahlen 2013 8ed; PDF identifies Wahlen/Baginski/Bradshaw 2023 10e
- `05_Equity/Damodaran_Investment_Valuation_4ed.pdf` -> `sources/cfa/pdfs/05_equity/eq_damodaran_2025_investment_valuation_4ed.pdf`
  - metadata_corrected: matrix said Damodaran 2012; PDF metadata/content indicate 2025 4e
- `06_Fixed_Income_and_Credit/The handbook of fixed income securities..pdf` -> `sources/cfa/pdfs/06_fixed_income_and_credit/fi_fabozzi_handbook_fixed_income_securities.pdf`
  - metadata_corrected: matrix said 2012 8e; PDF metadata/title indicate 2021 9e
- `Convertible_Bonds/classic_pricing_books_english/Hull_Options_Futures_and_Other_Derivatives.pdf` -> `sources/cfa/pdfs/convertible_bonds/cb_hull_2022_options_futures_derivatives_11ed.pdf`
  - metadata_corrected: PDF identifies Hull 2022 11e Global Edition; legacy matrix had no edition tag
- `20_Chinese_Convertible_Bonds_Research/sell_side_research/China_Galaxy_Securities/china-galaxy-securities-fund-suitability-rating-2025-10-29.pdf` -> `sources/cfa/pdfs/china_convertible_bonds/china_cb_china_galaxy_fund_suitability_2025_10_29.pdf`
  - version_date_preserved: source_id and filename retain the 2025-10-29 version date
- `20_Chinese_Convertible_Bonds_Research/offshore_HKEX/hkex-mb-listing-rules-ch16-convertible-equity.pdf` -> `sources/cfa/pdfs/china_convertible_bonds/china_cb_hkex_ch16_convertible_equity.pdf`
  - current_version_sensitive: HKEX current PDF captured from 2023-12-13 metadata; verify latest official chapter before final citation
- `20_Chinese_Convertible_Bonds_Research/offshore_HKEX/hkex-mb-listing-rules-ch28-convertible-debt.pdf` -> `sources/cfa/pdfs/china_convertible_bonds/china_cb_hkex_ch28_convertible_debt.pdf`
  - current_version_sensitive: HKEX current PDF captured from 2023-12-13 metadata; verify latest official chapter before final citation
- `20_Chinese_Convertible_Bonds_Research/exchange_rules/SSE/sse-cb-rules-compilation.pdf` -> `sources/cfa/pdfs/china_convertible_bonds/china_cb_sse_rules_compilation.pdf`
  - current_version_sensitive: SSE rules compilation can update irregularly; official releases control
  - non_authoritative_compilation: use as convenience compilation only after checking controlling exchange releases
- `Trading_Price_Action/Brooks_2012_Trading_Price_Action_Reversals.pdf`
  - excluded: unregistered PDF; metadata/text appear mismatched and language quality requires manual audit before registration

Generated source IDs use snake_case because Rust `kb ingest --source-id` rejects hyphens.
Chinese CB regulatory and research PDFs use concise ASCII `china_cb_*` source IDs; original titles and paths remain in `legacy_path_map.json`.
`excluded_sources.json` is the hash authority for matrix-excluded and non-matrix source-like files.
Legacy cards, auxiliary markdown, and volume drafts are not copied as canonical cards yet; their SHA256s are preserved in `legacy_content_manifest.json`.

## Source-id renames (bucket 4.5, dated 2026-05-28)

After the initial bootstrap, the independent re-research surfaced three
source-id naming inconsistencies. The following renames were applied coordinated
across `sources_manifest.json`, `chunks_manifest.json` (all matching chunks +
chunk_id prefix), `source_matrix.json`, `legacy_path_map.json`, registry
JSONs, per-source ingest directories, page coordinate maps, and the 3
affected behavioral-finance cards (`be-noise-trader-equilibrium`,
`be-regret-matching-foundations`, `be-sentiment-vs-fundamentals`):

| Old source_id | New source_id | Reason |
|---|---|---|
| `econ_hart_mascolell_regret_matching` | `econ_hart_mascolell_2013_simple_adaptive_strategies` | PDF identity confirmed as the 2013 World Scientific book *Simple Adaptive Strategies* (ISSN 2251-2071), not the 2000 Econometrica paper. Year+title now in source_id per doc 27's convention. |
| `fi_fabozzi_handbook_fixed_income_securities` | `fi_fabozzi_2021_handbook_fixed_income_9e` | PDF metadata Title field confirms 9th Edition (Adobe InDesign 15.1, CreationDate 2021); matrix said 8e/2012. Edition now encoded in source_id. |
| `qm_tsay_2010_afts` | `qm_tsay_2005_afts_2e` | PDF first page reads "Second Edition" (CreationDate 2005-08-19; the 2010 timestamp is ModDate metadata, not edition year). Matrix said 3e/2010. Edition now corrected and encoded. |

`chunk_hash` is invariant under source_id rename (envelope is
`{end_page, page_spans, start_page, text}`); only `chunk_id` prefix changes.
3,679 of 57,603 chunks were re-keyed (3 sources × their chunk counts).
