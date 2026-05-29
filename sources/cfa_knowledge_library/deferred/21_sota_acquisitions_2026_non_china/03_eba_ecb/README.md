# 03_EBA_ECB — EU AT1 Regulatory (Empty — acquisition failed)

**Status**: Empty as of 2026-05-21. EBA's website returned HTTP 502 (maintenance mode) on all probed URLs during acquisition.

## Items to manually retry

1. **EBA "Report on the monitoring of Additional Tier 1 (AT1), Tier 2 and TLAC/MREL instruments of EU institutions"** (June 2024)
   - Primary URL: https://www.eba.europa.eu/sites/default/files/2024-06/4c63729b-bb98-4edc-91ec-3001cd06050d/Report%20on%20monitoring%20AT1%20and%20MREL.pdf
   - Landing page: https://www.eba.europa.eu/publications-and-media/press-releases/eba-updates-monitoring-additional-tier-1-instruments
   - Net-new vs. on-disk McNeil 2015: prudential valuation reflecting actual loss-absorbency, FX-on-equity-classified AT1, multi-trigger conditions, contractual bail-in clauses for English-law AT1 issuances

2. **EBA Risk Assessment Report December 2025**
   - URL: https://www.eba.europa.eu/publications-and-media/publications/risk-assessment-report-december-2025
   - Content: EU AT1 stands at ~1.4% of RWA, Tier 2 at 2.6% as of late 2025

3. **EBA Annual Report 2024** (June 2025)
   - URL: https://www.eba.europa.eu/sites/default/files/2025-06/bee4e97f-91a9-43bd-abdb-bd774e0259bf/2024%20Annual%20Report.pdf
   - Content: AT1 monitoring framework annual roll-up

4. **EBA Opinion on MDA (Dec 2015)** — the canonical Maximum Distributable Amount mechanics document
   - Landing: https://www.eba.europa.eu/publications-and-media/press-releases/eba-calls-more-certainty-and-consistency-application

5. **European Parliament IPOL_BRI(2016)574399 "Maximum Distributable Amount, CoCo Bonds and Volatile Markets"**
   - URL: https://www.europarl.europa.eu/RegData/etudes/BRIE/2016/574399/IPOL_BRI(2016)574399_EN.pdf

6. **ECB Dec 2025 simplification framework / HLTF AT1 recommendations**
   - URL: https://www.ecb.europa.eu/press/pubbydate/2025/html/ecb.simplification_supervisory_reporting_framework202512.en.html

**Retry strategy**: EBA site maintenance was transient. Re-run `curl -sSL -A "Chrome/120" -e "https://www.eba.europa.eu/" -o <local> <url>` after EBA portal restores.
