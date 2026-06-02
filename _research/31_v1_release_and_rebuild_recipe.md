# _research/31 — CFA-legacy CACG v1 Release and Rebuild Recipe

Successor to `_research/30` (the v0 recipe). This records the **v1 milestone**: the migration of
three pre-authored `cacg.v0` card-skeleton sets from the read-only sibling repo
`~/CFA_reading/deferred_books/` into the released `cfa` corpus as three new readings.

## v1 Definition of Done

**v1 complete := 402 active emitted cards + 6 notes-taint quarantined cards (408 legacy total) across
14 readings.** The original 11 readings (01,02,03,05,06,07,08,09,10,11,17) plus the migrated
`14_microstructure_and_trading` (73 `mt-`), `15_performance_and_attribution` (35 `pa-`), and
`22_fund_level_arbitrage` (26 `fa-`) = 134 new cards. Schema stays `cacg.v0` (corpus milestone bump
only; no kernel change). The `v0-candidate` tag is left immutable; a new annotated `v1-candidate` tag
marks this milestone.

> **Superseded by the 2026-06 quarantine absorption → 408 active + 0 quarantined (408 total).**
> The 6 notes-taint cards deferred under FUT-2 (`_research/29`) were re-authored from non-notes
> primary sources (5 RM from McNeil QRM 2015; `rm-risk-objectives-and-tolerance` +
> `pm-tracking-error-and-active-risk` from CFA L1) and admitted, so the active corpus is now **408**
> with **0 quarantined** (`QUARANTINE_IDS` empty). The same rename consolidated `cfa_legacy → cfa`
> across `sources/`, `cards/`, and `out/`. Read every `402`/`6 quarantined` count below as
> **408 / 0** post-absorption; the gates (`check_v1_constants.py`, `check_v1_card_hash_invariance.py`,
> `run_corpus_gate.sh`) are green at **408/0/408**.

| v0 | v1 | what |
|----|----|------|
| 268 | **402** | active emitted cards |
| 274 | **408** | legacy total (active + 6 quarantined) |
| 70 | **87** | authorized/ingested sources (70 original + 17 migrated, incl. the O'Hara sandwich) |
| 11 | **14** | readings on disk |

## Sources & ingest (87 sources)

The 16 text-layer migration PDFs were copied to `sources/cfa/pdfs/<reading>/<source_id>.pdf`
(SHA-256-verified against each set's `_sources.json`) and the 17th source, O'Hara 1995 (image-only), was
resolved via a reproducibly-generated text-layer **sandwich** PDF (`build_ohara_sandwich.py`; pinned
fpdf2 + DejaVuSans; determinism inputs recorded in `v1_ohara_resolution.json`). `register_migration_sources.py`
appended the `ingest_plan.json` rows and extended `source_matrix.json` (keys 14 + 22 added; 15 extended,
preserving its existing authorizations). Ingest runs under the chromium/7778 Pdfium pin (host build
proven byte-equivalent — `v1_libpdfium_equivalence_proof.json`):

```
# per-source ingest under the pin, then merge at the final source count:
python3 sources/cfa/_registry/run_ingest_per_source.py
python3 sources/cfa/_registry/merge_ingest_manifests.py --force --require-count 87 --out out/cfa
```

This regenerates `out/cfa/chunks_manifest.json` (gitignored, never committed) and
`sources_manifest.json`; every original-70 `chunk_id`/`ordinal`/`chunk_hash` is byte-identical pre/post
merge. `chunks_manifest.json` and the regenerated `out/cfa/source_matrix.json` are intentionally
NOT committed (large/derived); they are rebuilt from the recipe.

## Resolve → emit → cross-link

1. `resolve_migration_citations.py` binds each incoming skeleton citation to a real chunk (fail-closed,
   `kb verify`-confirmed) into the 3 committed registries (`mt_14`/`pa_15`/`fa_22_curated_citations.json`)
   + `migration_bind_report.json` + `migration_quote_audit.json`. 54 quotes were repaired under
   machine-proven / reviewed structured causes (`migration_reviewed_quote_overrides.json` for the
   faithfulness-reviewed re-anchors; see `migration_faithfulness_review.json`).
2. `emit_migration_readings.py` renders the 134 cards (body verbatim from the read-only skeleton,
   frontmatter from the registry, no `card_hash`; cross-reading See-Also links + prose rewrites injected
   from `migration_cross_links.json`); `apply_released_card_backlinks.py` adds the reciprocal back-link to
   the one allowlisted released card (`be-limits-of-arbitrage`).
3. `KB_FROZEN_CLOCK=1 kb index cards/cfa --out out/cfa` stamps `card_hash` on all 402 cards,
   regenerates `cards_manifest.json` / `summaries.json` / `INDEX.md` / `pdfium_provenance.json`, and
   appends the 134 new `.history.jsonl` sidecars (402 total).

## Release gates (all green at 402/6/408)

- `check_v1_card_hash_invariance.py` — the 268 pre-existing `card_hash`es change ONLY for the recorded
  cross-link released-card allowlist (`be-limits-of-arbitrage`); the 6 quarantined ids are unchanged; total 402.
- `KB_FROZEN_CLOCK=1 run_corpus_gate.sh` — scope ledger reconciles 402 active + 6 quarantined = 408;
  full `kb lint --all-readings` exit 0 (402 journal entries); per-card `kb verify` 402/402; quarantine
  invariant intact (exactly the 6 frozen ids; Critical Rule 9 pin unchanged).
- `check_index_reproducible.py` — re-baselined at 402: two frozen `kb index` runs are byte-identical for
  the 3 published artifacts; the published index is clock-invariant; the frozen clock is load-bearing on
  the audit sidecar.
- `check_v1_constants.py` — structured stale-constant sweep: zero unreviewed v0 count literals across the
  gate scripts + research docs; the v1 literals agree with counts derived from data (manifest / queue /
  ingest_plan / reading dirs).
- `check_release_cleanliness.py [--require-tag]` — the small published-index artifacts + research docs +
  recipe scripts tracked; `chunks_manifest.json` excluded; one sidecar per active card; worktree
  release-clean; the annotated `v1-candidate` tag references the evidence and points at HEAD.

## What is committed vs rebuilt

Committed: the 402 card bodies + `.history.jsonl` sidecars; the 4 small published-index artifacts
(`cards_manifest.json`, `summaries.json`, `INDEX.md`, `pdfium_provenance.json`); the registry scripts +
curated registries + decision ledgers (`_research/29`, this doc). Rebuilt from the recipe (NOT committed):
`chunks_manifest.json`, `out/cfa/source_matrix.json`, `summaries.sqlite`. The `v0-candidate` tag and
all v0 card hashes are untouched.
