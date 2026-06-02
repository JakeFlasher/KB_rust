# CACG Diagnostic Codes

Every code is a string constant in `cacg.lint.codes` (Layer-1 codes) and `cacg.verify.layer2` / `cacg.integrate.round_summary` (Layer-2 and integration codes). Codes follow `CACG-<CATEGORY>-NNN`.

## Frontmatter / schema (CACG-FM-*)

| Code | When it fires | Example |
|------|---------------|---------|
| `CACG-FM-001` | Required frontmatter field missing | Drop `title` line from a card; the loader reports `missing required field: title`. |
| `CACG-FM-002` | Unknown `schema_version` | Change `cacg.v0` to `cacg.v9`. |
| `CACG-FM-003` | Extra/unknown top-level field | Add `unknown_field: oops` next to `summary`. |
| `CACG-FM-004` | Missing YAML frontmatter delimiters | Card body without leading `---\n...\n---\n` block. |
| `CACG-FM-005` | Forbidden YAML construct (anchors, custom tags) | `reading_id: &anchor "..."`. |
| `CACG-FM-006` | YAML parse error | Unterminated quoted string. |
| `CACG-FM-007` | Frontmatter root is not a mapping | `---\n- not\n- a\n- mapping\n---\n`. |
| `CACG-FM-008` | Other Pydantic validation error | Anything not mapped to a more specific code. |

## Citation structure (CACG-CITE-*)

| Code | When it fires |
|------|---------------|
| `CACG-CITE-001` | Malformed `chunk_id` (does not match `<slug>:p<NNN>:<NNNN>`) |
| `CACG-CITE-002` | Malformed or all-zero placeholder `chunk_hash` |
| `CACG-CITE-003` | Reversed `page_range` (`[end, start]`) |
| `CACG-CITE-004` | `chunk_id` not present in chunks manifest |
| `CACG-CITE-005` | `page_range` disjoint from cited chunk's pages |
| `CACG-CITE-006` | `source_id` on the citation disagrees with the chunk's `source_id` |

## Hash mismatch and staleness (CACG-HASH-*)

| Code | When it fires |
|------|---------------|
| `CACG-HASH-001` | Citation `chunk_hash` does not match the manifest's `chunk_hash` (source drift). |
| `CACG-HASH-002` | Card `card_hash` stored in frontmatter does not match the recomputed value (manual edit without `kb index`). |
| `CACG-HASH-003` | A chunk's stored `chunk_hash` disagrees with `chunk_hash(chunk.text)` (manifest hand-edited; text mutated, hash not bumped). |

## Index / publish (CACG-IDX-*)

| Code | When it fires |
|------|---------------|
| `CACG-IDX-004` | Manifest publish failed (used by `kb index` rollback). |
| `CACG-IDX-005` | `kb index --out <path>` was given an output path inside (or equal to) the cards directory; the framework would otherwise walk its own generated `INDEX.md` as a card on the next run. |
| `CACG-IDX-006` | `kb index` detected a pre-existing `<card>.md.tmp` or `<card>.md.bak` next to a card that needs rehashing; refusing to clobber sidecar files we did not create (editor backups, leftovers from other tools, remnants of a prior failed run). Remove the listed sidecar(s) and re-run. |
| `CACG-IDX-007` | `kb index` detected a pre-existing `cards_manifest.json.tmp`, `cards_manifest.json.bak`, `INDEX.md.tmp`, or `INDEX.md.bak` in `--out`; refusing to clobber recovery evidence from a prior crashed run. Remove the listed sidecar(s) and re-run. |
| `CACG-IDX-008` | `kb index`: `cards_manifest.json` or `INDEX.md` in `--out` exists as a non-file (typically a directory created in error). The publisher's `replace(bak)` flow would otherwise move the directory and crash on success-cleanup `unlink()`, leaving partial state. Remove or replace the non-file target before re-running. |

## CLI surface (CACG-CLI-*)

| Code | When it fires |
|------|---------------|
| `CACG-CLI-001` | File or directory not found. |
| `CACG-CLI-002` | Subcommand not yet implemented (no remaining stubs in MVP). |
| `CACG-CLI-003` | Bad CLI argument (e.g. slug that does not match the safe-characters regex). |
| `CACG-CLI-004` | Config error (`--config` file malformed or has unknown keys). |
| `CACG-CLI-005` | Output already exists (e.g. `kb new` refused to clobber without `--force`). |

## Ingest (CACG-INGEST-*)

| Code | When it fires |
|------|---------------|
| `CACG-INGEST-001` | PDF cannot be opened or text-extracted (corrupt, encrypted, etc.). |
| `CACG-INGEST-002` | No chunks produced (empty PDF text after normalization). |
| `CACG-INGEST-003` | Manifest publish failed during `kb ingest` (read-only `--out`, `os.replace` mid-publish failure, ENOSPC, etc.). The shape preflight under `CACG-CLI-001` catches typos; this catches operational filesystem failures after validation. |

## Manifest parse / validation (CACG-MAN-*)

| Code | When it fires |
|------|---------------|
| `CACG-MAN-001` | Any persisted manifest (`chunks_manifest.json`, `cards_manifest.json`, or `summaries.json`) is missing, unreadable, parses as invalid JSON, or fails Pydantic validation. The CLI emits the diagnostic + a journal entry (where applicable) instead of a Python traceback. |
| `CACG-MAN-002` | `kb ingest` detected a pre-existing `sources_manifest.json.tmp` / `chunks_manifest.json.tmp` / `.bak` sidecar (from a prior crashed run or manual edit); refusing to clobber. Remove the listed sidecar(s) and re-run. |
| `CACG-MAN-003` | `kb ingest`: `sources_manifest.json` or `chunks_manifest.json` in `--out` exists as a non-file (typically a directory). Same root cause as `CACG-IDX-008`; remove or replace the non-file target before re-running. |

## Lint journal integrity (CACG-JNL-*)

| Code | When it fires |
|------|---------------|
| `CACG-JNL-001` | The `--journal` file already exists but `validate_jsonl` flags one or more lines as tampered (mutated `event_checksum`, broken chain, or unparseable line). The CLI maps the failure to a deterministic exit 1 instead of a Python traceback. |

## Verify (CACG-VERIFY-*)

| Code | When it fires |
|------|---------------|
| `CACG-VERIFY-001` | Normalized exact-substring containment failed (fail-closed by default; `--fuzzy` may rescue OCR-grade drift). The diagnostic carries up to 3 BM25 `hint_only=true` candidates. |

## History (CACG-HIST-*)

| Code | When it fires |
|------|---------------|
| `CACG-HIST-001` | History JSONL malformed or same-`seq` rewrite detected. |

## Round-summary integration (CACG-RS-*)

| Code | When it fires |
|------|---------------|
| `CACG-RS-001` | `## Knowledge Consulted` section missing on a round summary that mentions KB-relevant paths. |
| `CACG-RS-002` | Section has both the `N/A` sentinel AND cited paths (mixed signal). Paths are verified, sentinel collision is reported as STALE. |
| `CACG-RS-003` | `N/A -- task not KB-relevant this round` sentinel claimed on a round whose body references `cards/` or `.claude/knowledge/`. |
| `CACG-RS-004` | `## Knowledge Consulted` section is present but lists no paths AND the body is KB-relevant. |

## Performance (CACG-PERF-*)

| Code | When it fires |
|------|---------------|
| `CACG-PERF-001` | PDF parsing detected on the common lint/verify path (sentinel-based assertion). |

## Source authorization (CACG-AUTH-*) — mandatory via `--source-matrix`

Authorization is **mandatory** as of the Round 6 trust-depth contract (Phase-3 DEC-8 reaffirmed). The `--source-matrix <path>` flag is required on `kb lint`, `kb verify`, `kb verify --round-summary`, and `kb search`; missing-flag exits 2 with argparse usage error. The matrix enforces a per-reading allow-list of source_ids. Layer-1 fires the diagnostic during normal lint; layer-2 fires it independently when `--unsafe-skip-lint` is used so the trust boundary holds either way.

| Code | When it fires |
|------|---------------|
| `CACG-AUTH-000` | `--source-matrix` was supplied but the path is not a regular file, parses as invalid JSON, or fails Pydantic validation (e.g., unknown top-level key, malformed `allowed` mapping). One diagnostic + one journal event per card visit so batch cardinality is preserved. |
| `CACG-AUTH-001` | The card's `reading_id` is not a key in `source_matrix.allowed`. |
| `CACG-AUTH-002` | The card's `reading_id` IS in the matrix but the citation's `source_id` is not on the allow-list for that reading. |

## Retraction (CACG-RETR-*) — card-level, source-level, chunk-level

CACG supports three retraction granularities:

- `kb retract <card>` retracts a single card; appends a tombstone history event AND atomically adds the card's `id` to `cards_manifest.retracted_cards`. The physical `.md` file is preserved on disk as a historical artifact.
- `kb retract-source <source_id>` retracts an entire source; atomically removes every chunk with that `source_id` from `chunks_manifest.chunks` AND adds the `source_id` to `chunks_manifest.retracted_source_ids`. Cards citing the retracted source fail verify with `CACG-RETR-002`.
- `kb retract-chunk <chunk_id>` retracts a single chunk; atomically removes the chunk from `chunks_manifest.chunks` AND adds its `chunk_id` to `chunks_manifest.retracted_chunk_ids`. Cards citing the retracted chunk fail verify with `CACG-RETR-003`.

| Code | When it fires |
|------|---------------|
| `CACG-RETR-001` | A `kb verify` (single-card, batch, or `--round-summary`) targets a card whose `id` appears in `cards_manifest.retracted_cards`. Severity is `"error"` by default; `--allow-retracted` downgrades to `"warning"` while still emitting the diagnostic and journal event. |
| `CACG-RETR-002` | A citation's `source_id` appears in `chunks_manifest.retracted_source_ids` (the source was retracted via `kb retract-source`). Emitted at layer-1 BEFORE the chunk-presence check (CITE-004) so retracted-source citations get the more specific diagnostic. Layer-2 enforces the same rule under `--unsafe-skip-lint`. Severity is always `"error"`; `--allow-retracted` only applies to RETR-001. |
| `CACG-RETR-003` | A citation's `chunk_id` appears in `chunks_manifest.retracted_chunk_ids` (the chunk was retracted via `kb retract-chunk`). Same enforcement pattern as RETR-002. Severity is always `"error"`. |

## Summary / tags (CACG-SUM-*) — Phase 3 retrieval surface

These codes are reserved for Layer-1 lint diagnostics surrounding the `summary` field promotion + the new `tags: list[str]` additive field. They fire once `kb migrate-summaries` lands and Layer-1 begins enforcing the bounded-summary contract.

| Code | When it fires |
|------|---------------|
| `CACG-SUM-001` | `summary` is empty on a Phase 3 card (post-migration window). The bound is `SUMMARY_MIN_LENGTH=80` characters; raw empty strings fail this gate. |
| `CACG-SUM-002` | `summary` exceeds `SUMMARY_MAX_LENGTH=400` characters. |
| `CACG-SUM-003` | A `tags` entry fails the slug regex (`^[a-z0-9][a-z0-9_-]*$`) or violates the per-tag length bounds `[TAG_MIN_LENGTH=2, TAG_MAX_LENGTH=40]`. Surfaced at Pydantic parse time (mapped to CACG-FM-008 wrapper) AND at Layer-1 lint when present in a card that already loaded successfully under a prior schema. |
| `CACG-SUM-004` | `tags` exceeds the count bound `TAGS_MAX_COUNT=10` entries. |
| `CACG-SUM-005` | Phase 4: cross-manifest source_ids consistency violated — a `SummaryEntry.source_ids` value differs from the corresponding `CardManifestEntry.source_ids` value at `_validate_summaries_match_cards_manifest` time. Indicates a tampered `summaries.json` or `cards_manifest.json` (the two manifests are derived from the same in-memory card load at index time). |
| `CACG-SUM-006` | Phase 4: `retract_card` consistency invariant violated — `cards_manifest.retracted_cards` contains a card_id that is NOT excluded from `summaries.json` (or vice versa). The two artifacts must be updated atomically. |
| `CACG-SUM-007` | Phase 4: `SummariesIndex.from_path` rejects a legacy `summaries.json` whose entries lack the additive `source_ids` key. Operator must run `kb migrate-summaries --out <out>` to populate source_ids from on-disk cards. |
| `CACG-SUM-008` | Phase 4: `kb migrate-summaries` failed to populate source_ids for one or more summaries entries — source cards missing on disk or have invalid citations. The migration exits 1 so the operator must fix the source side before proceeding. |

## Phase-4 retract / dependency-cascade (CACG-RET-* + CACG-CARDS-DEP-*)

| Code | When it fires |
|------|---------------|
| `CACG-RET-003` | Phase 4: `retract_card` cards-manifest pair + history append succeeded but the atomic `summaries.json` re-publish failed; all retract steps rolled back. |
| `CACG-CARDS-DEP-001` | Phase 4 R8 reserved: chunks_manifest retracted-sets and `cards_manifest.dependency_retracted_cards` are out of sync (cascade drift). Currently checked via the model_validator disjointness invariant; reserved for an explicit lint pass in a future phase. |
| `CACG-CARDS-DEP-002` | Phase 4: `cards_manifest` invariant violated — `retracted_cards ∩ dependency_retracted_cards ≠ ∅` (direct retraction subsumes cascade) OR `dependency_retracted_cards ⊄ cards.id` (a cascade-retracted card_id must still be in the active `cards` list; a fully-removed card belongs in `retracted_cards`). |

## Phase-4 CLI containment + lock fallback

| Code | When it fires |
|------|---------------|
| `CACG-CLI-003` | Phase 4: `kb scaffold-matrix --cards-dir <dir>` detected a card path that escapes the supplied cards root (DEC-2 resolved fail-loud rather than silently omit — the matrix is a security-authorization artifact). |
| `CACG-LCK-001` | Phase 4: `fcntl.flock` is unavailable in the runtime (Windows / unsupported FS); `lint_journal.jsonl` + per-card `history.jsonl` appenders degrade to unsynchronized (POSIX is the supported platform). Emitted once per process. |

## Phase-4 FTS5 sidecar (CACG-FTS-*)

The `out/summaries.sqlite` sidecar is built atomically as a Phase-D2 step
after `summaries.json` lands; `kb search` opens it read-only and verifies
the seal before consuming results. See [`docs/retrieval.md`](retrieval.md)
for the search backend contract.

| Code | When it fires |
|------|---------------|
| `CACG-FTS-001` | Sidecar staleness: `meta.schema_version` / `meta.builder_version` / `meta.summaries_hash` differs from the runtime's expected values, OR a query-time `sqlite3.OperationalError` indicates corruption (e.g., the `cards_fts` table was dropped). `kb search` falls back to the in-memory BM25 backend. |
| `CACG-FTS-002` | The runtime sqlite3 build lacks FTS5 support; `kb index` skips the sidecar build (informational), and `kb search` runs against the in-memory backend. No CI failure; DEC-9 graceful degradation. |
| `CACG-FTS-003` | Reserved for a future "FTS5 absent AND in-memory rank-bm25 absent" failure mode (both backends unavailable → exit 1). Currently unused since `rank-bm25` is a hard dep. |

## Phase-4 incremental index cache (CACG-IDX-CACHE-*)

`out/.kb_index_cache.json` is the heuristic warm-cache keyed by
`(path, mtime_ns, size, prior card_hash)`. DEC-13 resolved: the cache is a
PERFORMANCE HEURISTIC, not a trust authority — `kb verify` always rehashes
from card text bytes.

| Code | When it fires |
|------|---------------|
| `CACG-IDX-CACHE-001` | Reserved for a verify-time cache drift diagnostic (cache says `card_hash=X` but recomputed hash is `Y`). The runtime invalidation happens automatically; this code is for explicit operator surface when added in a future phase. |

## SKILL.md routers (CACG-SKILL-*) — Phase 3, optional per-reading

Optional `cards/reading_<NN>/SKILL.md` artifacts validate via Layer-1 when present. Missing SKILL.md is silently allowed (no diagnostic).

| Code | When it fires |
|------|---------------|
| `CACG-SKILL-001` | Two SKILL.md routers across different `reading_id`s declare the same `name`. Subject to DEC-5 (cross-reading collision contract; default per-reading-only). |
| `CACG-SKILL-002` | A SKILL.md `routes_to: [card_id]` entry references a card that is not in the active `cards_manifest` (unknown card_id, or card_id in `cards_manifest.retracted_cards`). |
| `CACG-SKILL-003` | A SKILL.md frontmatter violates the router schema (missing required `name` or `description`, extra unknown YAML keys, malformed `routes_to`). |

## Card-to-card dependency edges (CACG-DEP-*) — Phase 3

`CardEdge` carries card-to-card dependency edges as `{target, edge_type}` pairs in `CardFrontmatter.card_edges`. Validation runs at Layer-1 against the active `cards_manifest`. `CardEdge.edge_type` admits only `depends_on | extends` (narrower than the `CitationEdge` enum which covers card-to-chunk relations).

| Code | When it fires |
|------|---------------|
| `CACG-DEP-001` | A `card_edges[*].target` is not present in the active `cards_manifest.cards`. |
| `CACG-DEP-002` | A cycle exists in the card-edge DAG. The diagnostic fires on every card on the cycle so any one of them can be fixed by the operator. |
| `CACG-DEP-003` | An orphan card detected under `--check-orphans` opt-in (no inbound `card_edges` AND no inbound role-map references). Default lint does NOT emit this. |
| `CACG-DEP-004` | A `card_edges[*].target` references a card in `cards_manifest.retracted_cards` (dangling depends_on to retracted card, per DEC-7). |

## Per-reading role maps (CACG-ROLE-*) — Phase 3

Per-reading `source_role_map.json` artifacts (one per `reading_id`) declare the primary anchor + supporting sources + stance + relevance for every active card under that reading. The role map's existence is what triggers symmetric validation; readings with no role-map file are silently skipped.

| Code | When it fires |
|------|---------------|
| `CACG-ROLE-001` | A role-map entry references a `card_id` that is in `cards_manifest.retracted_cards` (asymmetry, retracted side). |
| `CACG-ROLE-002` | An active card has no entry in the per-reading role map (asymmetry, active side). |
| `CACG-ROLE-003` | A role-map entry's `stance` or `relevance` value is outside the closed-set vocabulary. Per DEC-2 v1 hardcoded enum: `stance` admits `{primary, supporting, deferred}`; `relevance` admits `{core, adjacent, extension}`. |

## Layer-3 semantic verifier (CACG-VERIFY-002) — Phase 3, opt-in

Layer-3 fires ONLY when Layer-2 exact-match fails AND `--fuzzy` rejects (or is absent) AND one of `--semantic` (B1 frozen-cache lookup) or `--semantic-judge` (B2 LLM-judge via Claude Haiku) is explicit.

| Code | When it fires |
|------|---------------|
| `CACG-VERIFY-002` | Semantic verification returned a verdict for a Layer-2-failed citation. Payload: `{verdict: pass | fail | abstain, score: float, reasoning?: str}` riding inside the same per-card journal event as Layer-2's CACG-VERIFY-001 (preserves per-card journal cardinality, AC-V1). `verdict: abstain` is the default outcome under B1 when the `(chunk_hash, claim_window_hash)` key is absent from `out/semantic_cache.json` (cache-as-oracle posture, DEC-9). |

## Adversarial coverage map

Every adversarial fixture under `tests/adversarial/` corresponds to exactly one expected diagnostic code. Rust integration tests and committed parity fixtures enforce the one-code coverage contract.

| Fixture | Expected code |
|---------|---------------|
| `01-malformed-hash.md` | `CACG-CITE-002` |
| `02-reversed-page-range.md` | `CACG-CITE-003` |
| `03-chunk-not-in-manifest.md` | `CACG-CITE-004` |
| `04-stale-card-hash.md` | `CACG-HASH-002` |
| `05-chunk-hash-drift.md` | `CACG-HASH-001` |
| `06-page-disjoint.md` | `CACG-CITE-005` |
| `07-fake-quote.md` | `CACG-VERIFY-001` |
| `08-auth-unknown-reading.md` | `CACG-AUTH-001` (fires only with `--source-matrix`) |
| `09-auth-unauthorized-source.md` | `CACG-AUTH-002` (fires only with `--source-matrix`) |
| `10-retracted-card.md` | `CACG-RETR-001` (fires when the card's `id` is in `cards_manifest.retracted_cards`) |
| `11-retracted-source-cited.md` | `CACG-RETR-002` (fires when the cited `source_id` is in `chunks_manifest.retracted_source_ids`) |
| `12-retracted-chunk-cited.md` | `CACG-RETR-003` (fires when the cited `chunk_id` is in `chunks_manifest.retracted_chunk_ids`) |

The retraction fixtures are structurally valid cards. Rust tests cover RETR-001 severity modes (`error` and `warning` under `--allow-retracted`), RETR-002/003 layer-1 and layer-2 enforcement, `retract-chunk` manifest mutation, and round-summary batch integration.
