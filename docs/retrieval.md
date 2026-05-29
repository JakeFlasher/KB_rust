# CACG Retrieval Surface (Phase 3)

The Phase-3 retrieval surface lets sub-agents discover and consult cards without bulk-loading the corpus. Three artifacts + two CLI verbs cover the lifecycle: eager-load metadata (`summaries.json`), per-reading routing hints (optional `SKILL.md`), and deterministic BM25 search (`kb search`).

## Artifacts

### `out/summaries.json` — eager-load card-of-cards

Published atomically by `kb index` via the sequenced Phase-D publisher. Strict Pydantic (`schema_version="cacg.v0"`, `extra="forbid"`). Sorted by `(reading_id, id)`. Retracted cards are excluded entirely (the canonical retraction state is `cards_manifest.retracted_cards`; tombstone-in-summaries would bloat the eager-load payload).

```json
{
  "schema_version": "cacg.v0",
  "summaries": [
    {
      "schema_version": "cacg.v0",
      "id": "content-addressable-identity",
      "title": "Content-Addressable Identity",
      "reading_id": "reading_01",
      "summary": "Required bounded-length summary used by kb search and the eager-load surface.",
      "tags": ["hashing", "verification"],
      "path": "cards/reading_01/g.md",
      "card_hash": "<64-hex>"
    }
  ]
}
```

The publisher-side invariant `_validate_summaries_match_cards_manifest` asserts every summary entry's `(id, card_hash, reading_id, title, summary, path)` matches the corresponding `cards_manifest.cards[*]` entry. `tags` is summaries-only and not cross-checked.

### `cards/reading_<NN>/SKILL.md` — optional per-reading routers

Anthropic-Skills-style task router. Frontmatter is strict Pydantic; missing SKILL.md is silently allowed. Per DEC-5 default, `routes_to` references must point at cards in the SAME reading.

```yaml
---
name: "understanding-fixed-income"
description: "Routes the agent to the right card for fixed-income questions about duration, OAS, or callable bonds."
routes_to:
  - "duration-and-convexity"
  - "oas-calculation"
trigger_keywords:
  - "duration"
  - "OAS"
---
```

Validation codes: `CACG-SKILL-001` (cross-router name collision), `CACG-SKILL-002` (`routes_to` references unknown / retracted / cross-reading card), `CACG-SKILL-003` (frontmatter schema violation).

`kb index` filters `SKILL.md` out of the card walk; `kb migrate-summaries` does the same. Routers are NOT cards.

## `kb search <query> --source-matrix <matrix>`

Deterministic BM25 retrieval over `(title + summary + tags)` (per DEC-13; body inclusion deferred). Mandatory `--source-matrix` per DEC-8 — cards under unauthorized readings are excluded from results BEFORE the top-K cap so an unauthorized hit cannot displace an authorized one.

**Ranking contract:**
- Raw BM25 scores are sorted descending; deterministic tiebreak by `card_id` ascending.
- Sort happens on raw scores; rounding to 6 decimals is presentation-only (closes the "round-before-sort" regression from Round 4).
- Match contract is lexical-overlap (query token set ∩ doc token set), NOT BM25 score > 0 — small / degenerate corpora can produce negative IDF scores that are still legitimate matches.

**Output formats:**
- Default (human): `score  card_id  [reading_id]  title  -> path` per hit.
- `--json`: canonical-JSON list of `{card_id, reading_id, title, summary, tags, path, card_hash, score}` per hit. Byte-identical under `KB_FROZEN_CLOCK=1`.

**Determinism:** two consecutive runs over identical inputs produce identical stdout (no time / uuid / locale-sensitive formatting).

## `kb scaffold-role-map --reading <id>`

Synthesizes a permissive starter `out/role_maps/<reading_id>.json` from `cards_manifest.cards`. Default stance = `supporting`, default relevance = `core`. Atomic write via `os.link` (no-clobber) or `os.replace` (`--force`). Path-escape protection: cards whose manifest `path` resolves outside `--cards-dir` are skipped.

See `docs/schema.md` for the role-map artifact format and `docs/lint-codes.md` for `CACG-ROLE-001/002/003`.

## Sub-agent integration

A typical sub-agent workflow preloads `summaries.json` once, then loads only the card files it needs:

1. `kb search "duration convexity" --source-matrix m.json --top-k 5 --json` → list of card ids + paths.
2. For each hit, `cat <path>` or `kb show <card_id>` (future verb) for the full body.
3. Cite consulted paths in the round summary's `## Knowledge Consulted` section.
4. `kb verify --round-summary <summary>` validates the section against the cards manifest.

`kb verify --round-summary` can be combined with `--semantic <cache>` (B1) or `--semantic-judge` (B2) to invoke Layer-3 verification on any cited card whose Layer-2 exact-match fails. See `docs/semantic-verifier.md` for the Layer-3 contract.

## Phase-4 SQLite FTS5 sidecar

`kb index` builds an additional artifact `out/summaries.sqlite` atomically
as a Phase-D2 step right after `summaries.json` lands on disk. The sidecar
is a FTS5 virtual table (`cards_fts(card_id UNINDEXED, reading_id UNINDEXED,
path UNINDEXED, card_hash UNINDEXED, title, summary, tags, source_ids)` with
`tokenize = "unicode61 remove_diacritics 1"`) plus a `meta` table sealing
`(schema_version, builder_version, summaries_hash, summaries_count)`.

`kb search` opens the sidecar read-only and verifies the seal:

- If `meta.summaries_hash` matches `sha256(summaries.json)`, the FTS5 backend
  serves the query. Cold-open + query latency is targeted at sub-50ms on a
  100k-card corpus (the in-memory BM25 backend rebuilds the corpus per
  invocation; FTS5 avoids that).
- If the seal mismatches (stale sidecar because `summaries.json` was mutated
  without re-running `kb index`), `kb search` emits `CACG-FTS-001`
  informational and falls back to the in-memory BM25 backend.
- If `summaries.sqlite` is absent (either pre-Phase-4 corpus or the runtime
  sqlite3 build lacks FTS5 → `CACG-FTS-002` was logged at `kb index` time),
  `kb search` falls back silently.

**Cross-backend rank-order parity is NOT a contract.** Each backend has
its own deterministic ordering — FTS5 sorts by `bm25(cards_fts), card_id`;
the in-memory backend sorts by `rank-bm25` score with the same `card_id`
tie-break. Tests assert each backend's determinism on its own fixtures,
not score-or-order equivalence across backends.

**Authorization + retraction filtering happens AFTER FTS5 returns matches**
but BEFORE the `top_k` cap, so an authorized hit is never displaced by
a higher-rank unauthorized one (closes the Phase-4 closing-audit P1 #2 finding).

**Raw `summaries.sqlite` bytes are EXCLUDED from the `KB_FROZEN_CLOCK=1`
byte-determinism contract** (per DEC-8 user adjudication). The contract on
the sidecar is on QUERY RESULTS (deterministic via `ORDER BY rank, card_id`)
and the sealed `meta.summaries_hash`; raw SQLite bytes vary by page
allocation and B-tree splits.

## Phase-4 incremental `kb index` cache

`out/.kb_index_cache.json` caches `(path, mtime_ns, size, prior card_hash)`
per card so a re-index on an unchanged corpus can skip the (dominant)
`load_card` parse step. The cache is a **performance heuristic, not a
trust authority**: `kb verify` always rehashes from card text bytes, so a
pathological mtime-preserving content edit is detected at the next
verify, not at index time (DEC-13 resolved).

The cache header (`cache_format_version`, `schema_version`, `sentinel`)
forces a full rebuild on schema bumps and after `kb migrate-summaries`
(which writes a non-empty sentinel as a precaution).

## Performance budgets (Phase-4 closure)

- AC-P1: `kb search` 1000-card p95 < 500ms (Phase 3 budget; unchanged).
- AC-P2 (R5 single-pass YAML loader): 1000-card `kb verify --round-summary`
  drops from ~2900ms baseline to under 1000ms strict / 2000ms loose. See
  [`perf-reports/phase-4-closure.md`](../perf-reports/phase-4-closure.md);
  measured 555ms.
- AC-P9 (R7 incremental cache): warm-cache 1000-card `kb index` under
  200ms (measured 99ms). The 10k-card scale exceeds the strict 100ms
  target at 725ms; documented in the closure report with a proposed
  Phase-4.x optimization (content-hashed Phase-D2 skip).

All gates pinned by `legacy_python_oracle/tests/perf/test_phase3_retrieval_budgets.py` and
`legacy_python_oracle/tests/perf/test_phase4_*_budget.py`. The 10k-card local-only stress
fixture (see `docs/stress-10k.md`) is not in the CI gate but is documented
for operators.
