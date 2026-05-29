# CACG Phase 4 Seed: Schema/Trust Tightening + Performance Push

## Context (read-only siblings + current repo state)

- `/home/jakeshea/CFA_reading/` — read-only design oracle (mature 248-card production KB; FTS5 SQLite manifest; per-vertical role maps; HTML deliverables; ASCII diagram primitives; 41-code linter). CACG mirrors the discipline but is missing the FTS5 persistent index and the HTML output layer.
- `/home/jakeshea/humanize/` — Claude Code plugin v1.17.0 implementing RLCR. CACG integrates via `kb verify --round-summary` + the `## Knowledge Consulted` round-summary contract.
- Current CACG state — Phase 3 SHIPPED end-to-end (Milestones 1+2+3+4+5 closed; 502 tests passing; full Codex audit chain confirmed MERGE_READY_THROUGH_PHASE_3 after Round 9b). Rounds 0+1 of Phase 4 ALREADY SHIPPED (schema tightening + fail-closed manifest loading; 521 tests passing; Codex confirmed MERGE_READY for Rounds 0-1).

## Two Codex consultations seeded this phase

1. **Comprehensive framework review** (2026-05-20, gpt-5.5:high, 317s): whole-framework audit identifying P1 + P2 rooms-of-improvement across architecture, performance, schema/API ergonomics, test coverage, missing-features-vs-CFA, operational, documentation, determinism, security. Top 5 next-steps recommendation:
   1. Tighten schema/trust (unique card_ids, Literal schema_versions, 64-hex hashes, page-span structural validation, fail-closed manifests).
   2. Fix search/retraction consistency (carry source_ids into summaries; update summaries on retract).
   3. Replace ephemeral BM25 with SQLite FTS5 persistent index; cache BM25 hint corpora per source.
   4. CFA-parity operational features (`kb show`, HTML, dep-order, diagram primitives).
   5. Docs + CLI stabilization (README update, hide test-helper flags, CI).

2. **Rust refactor analysis** (2026-05-20, gpt-5.5:high, 341s): independent research with actual micro-benchmarks on the 1000-card stress fixture. Measured hot-path distribution: PyYAML `parse_card` = 91% of verify time (2.64s of 2.90s). Hashing/JSON/Pydantic together = ~2% of time. Recommendation: **(c) Keep Python pure; focus on algorithmic improvements**. Full Rust rewrite would cost 3-6 months, lose Python plugin ecosystem advantages, and replace already-Rust-backed components (Pydantic v2 uses `pydantic-core`; pypdfium2 already binds C PDFium). Best ROI investments: remove double-YAML parse, add incremental `kb index`, persist search as SQLite FTS5.

The two consultations CONVERGE: prioritize algorithmic + correctness improvements over a Rust port.

## Problem (combined scope: trust + perf)

The Phase-3-shipped framework has two distinct classes of debt the user wants closed in one phase:

1. **Schema/trust gaps** (Codex comprehensive review P1):
   - `CardsManifest` accepted duplicate `cards[*].id` — `card_dag` trusted it for uniqueness.
   - `CardsManifest.schema_version` + `CardManifestEntry.schema_version` were plain `str`, not `SchemaVersionLiteral`. Wrong-version manifests loaded silently.
   - `card_hash` accepted any string in `CardManifestEntry` + `SummaryEntry`. No 64-hex validation.
   - `ChunkRecord.page_spans` admitted non-monotonic offsets, out-of-range pages, duplicate pages.
   - `RetractionSpec.from_cards_manifest_lenient` silently treated a present-but-malformed `cards_manifest.json` as empty retraction set (FAIL-OPEN on retract enforcement).
   - `kb search --source-matrix` filter only checks `reading_id` membership — not whether a card's cited `source_id`s are authorized. `summaries.json` doesn't carry source_ids.
   - `kb retract` mutates `cards_manifest.retracted_cards` but doesn't update `summaries.json` — `kb search` keeps returning retracted cards until next `kb index`.
   - `scaffold-matrix` lacks the `--cards-dir` containment check `scaffold-role-map` has.
   - `tests/conftest.py` auto-injects `--source-matrix` so README-style invocations pass tests while failing in production.
   - No CI workflow file; `--skip-lint` exposed as public verify option despite test-helper labeling.
   - Concurrent journal writers can corrupt the checksum chain (no fcntl lock).
   - Docs drift: README quickstart missing mandatory `--source-matrix`; `docs/schema.md` says chunk_hash is over text only but actually binds page metadata.

2. **Performance debt** (Codex Rust analysis micro-benchmarks):
   - **PyYAML double-parse**: `_reject_anchors_and_tags()` walks the YAML event stream, then `yaml.load()` re-parses the doc. ~91% of verify time on 1000-card stress fixture; collapsing into a single custom Loader pass plausibly gives 3-10x speedup.
   - **BM25 rebuild per invocation**: `kb search` instantiates `BM25Okapi(corpus)` each run; at 100k cards the build is 569ms before the first query. CFA reference ships persistent SQLite FTS5 + smoke tests.
   - **BM25 hint corpus rebuild per failure**: `verify_citation` calls `bm25_hints.top_k(quote, chunks, k=3)` per failed citation; each call rebuilds the BM25 corpus from scratch. Should be cached per source per verify run.
   - **`kb index` is whole-world**: every card loaded + hashed + diffed against the manifest on every invocation. Acceptable at 1k; painful at 10k+. Incremental cache keyed by `(path, mtime, size, content_hash)` would make unchanged-corpus index near-instant.

## Three combined milestones (M1 + M2 + closure)

### M1: Schema/Trust Tightening (8 rounds; Rounds 0+1 SHIPPED; 6 rounds remain)

- **Round 0 (SHIPPED, +13 tests)**: `CardsManifest._reject_duplicate_card_ids`; `schema_version` Literal on `CardsManifest` + `CardManifestEntry`; 64-hex validator on `CardManifestEntry.card_hash` + `SummaryEntry.card_hash`; `ChunkRecord._validate_page_spans_structurally` (monotonic offsets, bounded by len(text), pages in [start_page, end_page], no duplicates).
- **Round 1 (SHIPPED, +6 tests)**: `RetractionSpec.from_cards_manifest_lenient` re-raises `RetractionLoadError` on present-but-malformed; absent stays empty (pre-Phase-3 workflow); both `_cmd_verify` and `_cmd_verify_round_summary` map to `CACG-MAN-001` EXIT_FAIL.
- **Round 2**: Search source_id propagation + retraction/summaries consistency. Add `source_ids: list[str]` (sorted, unique) field to `SummaryEntry` capturing every cited `source_id`. `kb search --source-matrix` filter now checks `reading_id` membership AND that every cited `source_id` is on the matrix allow-list for that reading. `kb retract` (and `retract-source` / `retract-chunk`) update `summaries.json` atomically alongside `cards_manifest.json` so search excludes retracted cards immediately.
- **Round 3**: `scaffold-matrix` path-escape containment check; `tests/conftest.py` autouse fixture that injects `--source-matrix` is removed (tests must explicitly pass it); `--skip-lint` either removed or renamed to `--unsafe-skip-lint` so production callers can't accidentally weaken Layer-1 trust.
- **Round 4**: README quickstart updated with mandatory `--source-matrix`; `docs/schema.md` hash-envelope documentation corrected (chunk_hash binds text + start_page + end_page + page_spans, NOT just text); CI workflow file added (`.github/workflows/ci.yml` runs pytest + pyright + ruff/black + the demo); concurrent journal writers use `fcntl.flock` advisory lock on `lint_journal.jsonl` + per-card `history.jsonl` so parallel CLI runs can't corrupt the prev_checksum chain.

### M2: Performance Push (3 rounds)

- **Round 5**: Remove double-YAML parse. Fold `_reject_anchors_and_tags()` into a single `yaml.load(...)` pass via a `_NoDuplicateKeysSafeLoader` subclass that catches anchor/tag events during construction. Benchmark on the 1000-card stress fixture (target 3-10x speedup on the parse_card slice; expected total verify drop from ~2.7s to ~0.5-1s).
- **Round 6**: SQLite FTS5 persistent search index. New `out/summaries.sqlite` artifact built atomically at `kb index` time (sequenced Phase D alongside `summaries.json`). `kb search` opens read-only against the sidecar. Backward-compat fallback to in-memory `rank-bm25` if sidecar absent. Per-source BM25 hint cache during verify (closes the per-failure rebuild issue).
- **Round 7**: Incremental `kb index`. Cache keyed by `(path, mtime, size, prior card_hash)` so unchanged-corpus re-index is sub-100ms. Schema-bump or hash-drift forces full rebuild. Deterministic invalidation under `KB_FROZEN_CLOCK=1`.

### Closure (Round 8)

- **Round 8**: Documentation refresh (`docs/retrieval.md`, `docs/semantic-verifier.md`, `docs/integration-with-humanize.md` cross-reference + drift cleanup); final Codex pass; perf rerun on 1000-card + 10k-card stress fixtures; final demo refresh.

## Non-goals (explicitly deferred)

- Full Rust rewrite (Codex recommends against; deferred indefinitely).
- HTML deliverables (CFA-parity feature deferred to Phase 5).
- `kb show <card_id>` verb (deferred to Phase 5).
- Multi-LLM concurrent judging for semantic verifier (B1+B2 ensemble; deferred).
- ASCII diagram primitives + drift checks (CFA-parity, deferred to Phase 5).
- Cross-platform Windows support for the perf-test fcntl lock (POSIX-only; Windows runs degrade to unsynchronized).
- PyO3 / mypyc / Cython hot-path bindings (deferred until profiling proves remaining hot path warrants native code).

## Constraints (non-negotiable)

- All 521 existing tests stay green at every milestone gate (no regressions).
- Byte-deterministic output under `KB_FROZEN_CLOCK=1` preserved on every default path.
- `cacg.v0` schema purely additive (only `SummaryEntry.source_ids` added; no breaking change to existing manifests; pre-Round-2 summaries.json loads with empty source_ids per backward-compat fallback).
- Three-phase atomic publish contract unchanged (per-card + manifest pair + sequenced Phase D extended to include the new SQLite sidecar).
- `--semantic` / `--semantic-judge` flags unchanged from Phase 3.
- No `--no-verify`, no hook-skipping flags, no destructive operations on card .md files beyond the existing retraction contract.
- Permitted deps: stdlib `sqlite3` (no new package install for FTS5; CPython ships FTS5 enabled by default on most distributions); existing `rank-bm25`, `pypdfium2`, `pydantic>=2`, `pyyaml`, `pytest`. No new mandatory dep.

## Open questions for the directed-swarm / gen-plan to address

- **`SummaryEntry.source_ids` semantics**: sorted unique list of every cited `source_id` from the card's citations? Or only the "primary" source_id (per role_map.json's `primary.source_id`)? Tradeoff: full list catches more authorization failures but bloats summaries.json; primary-only is leaner but matches role-map semantics.
- **`scaffold-matrix` containment failure-mode**: skip the offending card (parity with `scaffold-role-map`) or exit 1 with `CACG-CLI-003` for any path-escape? The role-map verb chose skip-and-continue.
- **SQLite FTS5 schema version**: prefix the sidecar filename (`summaries_v1.sqlite`) so a schema bump can land without breaking older operators? Or hardcode + require operator-driven rebuild?
- **Incremental `kb index` cache file format**: stored as `.kb_index_cache.json` alongside `cards_manifest.json` (Pydantic-validated, canonical-JSON)? Or as a SQLite table inside the FTS5 sidecar? Tradeoff: separate file is simpler; embedded is one fewer artifact.
- **fcntl lock release on crash**: rely on OS-level cleanup (lock file is OS-managed) or write an explicit `try/finally` recovery path?
- **`--skip-lint` removal vs rename**: removing it changes the CLI surface (potentially breaks downstream scripts). Renaming to `--unsafe-skip-lint` preserves the capability with louder semantics. The plan must close this DEC before Round 3.
- **`tests/conftest.py` auto-injection removal**: cleanly possible? Or do some tests legitimately need it (e.g., the `kb verify --round-summary` golden-card test)? Audit needed.

## Suggested orthogonal directions

The directed swarm should diverge across at least these axes:

1. **"Trust spine first"** — M1 entirely first, M2 deferred to a follow-on phase. Lowest concurrent surface; matches Codex's top-5 prioritization (schema/trust + search-source-id at #1-#2).
2. **"Perf push first"** — M2 entirely first, M1 deferred. Riskier (perf changes touch hot paths in lint/verify) but answers user's stated intent of combining both.
3. **"Interleave M1 + M2 by Codex P1 rank"** — schedule rounds by Codex's priority across both milestones, not by milestone. E.g., R0 schema tightening (P1, M1), R5 double-YAML (P1 perf, M2), R2 search-source-id (P1, M1), R6 FTS5 (P1, M2), R3+R4+R7 P2 items.
4. **"M1 then M2 in series" (current)** — finish M1 (Rounds 0-4) then start M2 (Rounds 5-7). Each milestone closes before the next begins.
5. **"Minimum-viable cut"** — M1 only; M2 (perf push) deferred to a named Phase 4.5 follow-on. Honest "no" voice if the combined scope exceeds the user's appetite.

The synthesis picks the strongest as PRIMARY, renders the rest as Alt-1..Alt-K with explicit tradeoffs.

## Success criteria for the seed exploration + gen-plan output

The `/humanize:gen-plan` output should produce a plan document at `.humanize/.humanize/plans/cacg-phase-4-trust-perf-plan.md` that:

- Identifies the PRIMARY direction across the 5 candidate axes above.
- Renders Alt-1..Alt-K alternatives with explicit tradeoffs.
- Surfaces 1-3 open DEC-* decisions the user must close before the loop continues (the 7 open questions above need consolidation).
- Specifies 18-25 ACs in TDD format (positive + negative tests), covering both Schema/Trust + Perf push.
- Names the load-bearing files in `src/cacg/` that each round touches.
- Documents the upper-bound vs lower-bound scope (e.g., upper = all 8 rounds; lower = M1 only + FTS5 deferred).
- Adopts the same convergence + RLCR contract used by the Phase 3 plan (Goal Description, Acceptance Criteria, Path Boundaries, Feasibility Hints, Dependencies + Sequence, Task Breakdown with `coding`/`analyze` tags, Claude-Codex Deliberation, Pending User Decisions, Implementation Notes).
- Explicitly references Rounds 0+1 as ALREADY SHIPPED with the 19 new tests + Codex MERGE_READY verdict already on the board.
- Documents the Codex-recommended rejection of a Rust rewrite (DEC-X resolved: "no Rust port; algorithmic improvements first") so the next phase doesn't relitigate it.
