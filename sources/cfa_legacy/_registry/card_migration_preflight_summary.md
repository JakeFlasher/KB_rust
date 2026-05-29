# CFA Legacy Card Migration Preflight

Generated: `1970-01-01T00:00:00Z`
Artifact set: `c8993eb5bdf29063552938fc32ea9e83ca137044b24a44bc29075e94ac9e5613`

## Summary

- Active legacy cards scanned: 274
- Ready for offset/quote mapping: 266
- Notes-tainted quarantine: 8
- Source resolution blocked: 0
- Source authorization blocked: 0
- Unique legacy source paths referenced: 63
- Total source-reference occurrences: 693
- Legacy active-card set hash: `a451248efebec90dee0d1071586b018b995b2c063599483b471e16b04752a777`

## Buckets

- `quarantine_notes_taint`: 8
- `ready_for_offset_and_quote_mapping`: 266

## Problems

- `notes_taint`: 8

## Blocked Samples

- `.claude/knowledge/02_economics/ec-currency-exchange-rates-and-parity.md`: notes_taint
- `.claude/knowledge/09_portfolio_management_and_asset_pricing/pm-tracking-error-and-active-risk.md`: notes_taint
- `.claude/knowledge/11_risk_management/rm-historical-simulation-var.md`: notes_taint
- `.claude/knowledge/11_risk_management/rm-monte-carlo-var.md`: notes_taint
- `.claude/knowledge/11_risk_management/rm-parametric-var.md`: notes_taint
- `.claude/knowledge/11_risk_management/rm-risk-objectives-and-tolerance.md`: notes_taint
- `.claude/knowledge/11_risk_management/rm-sensitivity-versus-simulation.md`: notes_taint
- `.claude/knowledge/17_cross_cutting/cc-material-info-and-dissemination-delay.md`: notes_taint

## Migration Caveat

A card in `ready_for_offset_and_quote_mapping` is not ready to emit as CACG yet. It only means its legacy source paths resolve to active `source_id`s and are authorized for the target `reading_id`. The next pass must verify book-page to PDF-page offsets and choose verbatim chunk quotes before writing CACG cards.

`deliverable_ready` is preserved only as legacy frontmatter metadata. Downstream migration tooling must gate on `migration_bucket`, `notes_taint`, and `eligible_for_offset_quote_mapping`; every card remains `eligible_for_cacg_emission: false` until quote and chunk-hash binding finish.

A source-level worklist for that next pass is written to `page_offset_worklist.json`.

## 08_CB README drift resolution (added 2026-05-28)

The legacy `/home/jakeshea/CFA_reading/CFA_reading/.claude/knowledge/08_convertible_bonds/`
directory contains 50 `.md` files: 49 `cb-*.md` cards plus 1 unprefixed
`README.md`. The `README.md` is a subcorpus overview document, NOT a card.

`kb_manifest.json` correctly excludes it (active count = 274 across all subcorpora,
of which 50 belong to 08), but a naive `*.md` directory listing returns 51 under
08_convertible_bonds, which surfaced as a 50-vs-51 drift in the independent
audit (`_research/27_*`).

Resolution: `README.md` is preserved in the legacy reference bundle and is NOT
emitted to `cards/cfa_legacy/08_convertible_bonds/` during migration. It is
documentation, not knowledge.

Other subcorpora confirmed clean (no unprefixed Markdown other than `INDEX.md`):
01, 02, 03, 05, 06, 07, 09, 10, 11, 17.
