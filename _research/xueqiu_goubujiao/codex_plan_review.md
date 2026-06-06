**LOCKED Decisions**
1. **Q1:** Correct: build `cards/hkex/` as a standalone deck with native `kb ingest/new/lint/verify/index`, `out/hkex/*`, its own `source_matrix`; do not touch CFA migration gates; do not reuse CFA migration machinery except as reference code, author/resolve citations directly against `chunks_manifest.json`.

2. **Q2:** Generate one Xueqiu citation PDF from escaped `corpus_full.md` via **Chromium + embedded Noto CJK**, not xelatex; render atomic utterances page-contained; spike gate before authoring: regenerate PDF → ingest → assert normalized PDF text covers all seed quotes, no STX/U+FFFE/control junk, 100% resolver containment on all 73 candidates plus edge-case quotes, then verify synthetic cards.

3. **Q3:** Yes: author Chinese quote from `corpus_full.md`, resolver finds the containing chunk and fills `source_id/chunk_id/chunk_hash/page_range`, fail-closed, then `kb verify`; for Xueqiu use high target tokens, zero overlap, and page-contained utterances, preferably `max_pages_per_chunk: 1` so short CJK quotes do not straddle chunks.

4. **Q4:** Ship **Phase-A v1 from free sources first**: Bennett + HKEX/IRD/IFEC PDFs/snapshots; defer commercial-book grounding cards until acquired; do not block HK-operational cards on McMillan/Ellman/Natenberg/Graham.

5. **Q5:** Acceptance bar: only `faithful` or rewritten exactly to `corrected_summary`; every options card must carry the no-leverage/full-underlying/full-cash/ordinary-investor-warning spine; reject `//@` reposts and `(to @...)` third-party text; recurrence = distinct author utterances; exclude weak/misattributed; hkex v1 target **~45 cards**: ~15 free grounding + ~30 practitioner.

6. **Q6:** Encode snapshot status with tags plus body: every practitioner card gets `xueqiu-2022h1`; tactical-price cards also get `dated-levels`; first body section `## Dated State` says the levels are from the June/July 2022 corpus snapshot and are not durable recommendations.

7. **Q7:** Reuse CFA-style folder/reading_ids inside `cards/hkex/` and key `source_matrix.allowed` by those ids; options mechanics in `07_derivatives_and_volatility`, Stock Connect/CCASS/cert/stamp duty in `14_microstructure_and_trading`, dividend-WHT/tax incidence in `02_economics`, synthesis/risk-spine cards only in `17_cross_cutting`.

8. **Q8:** Cross-deck See Also prose to CFA ids and hkex grounding ids is fine; lint/verify ignore body prose; keep `card_edges: []` unless you build one combined manifest.

**Top Risks**
1. **CJK extraction parity:** Chromium/PDFium line wrapping or hidden controls can break verbatim quote containment. Mitigation: parity spike before authoring, page-contained utterances, reject control characters, resolver must fail closed.

2. **Faithfulness regression:** The known failure mode is sanitized option cards that drop fatal-risk caveats. Mitigation: hard risk-spine checklist on every options card.

3. **Attribution contamination:** Reposts and third-party replies inflate recurrence and misattribute claims. Mitigation: parser/lint gate rejecting `//@` and `(to @...)` quote sources.

4. **Commercial-source dependency:** Full theory layer cannot be completed until books are legally acquired and paginated PDFs are stable. Mitigation: free Phase-A v1 now; commercial Phase-A later.

5. **HK facts are time-sensitive:** Stock Connect eligibility, stamp duty, contract specs, and HKEX PDFs move. Mitigation: print/snapshot to PDF with as-of date; cite official sources; mark Xueqiu-only operational claims honestly.

**Disagreement**
Do not call the options layer “covered-call strategy” as the default label. The accurate deck language is **base-position-protected, fully collateralized option selling around fair value**, with “half-naked” preserved where the author says it.
