# CACG Phase 3 Seed: Retrieval Layer + Semantic Verifier + Dependency DAG

## Context (read-only siblings + current repo state)

- `/home/jakeshea/CFA_reading/` — mature 248-card production knowledge base (10 closed verticals + Chinese-CB extension + QM extension + FI extension), 41-code linter across 10 diagnostic categories (FM, CITE, EPUB, NUM, SEC, DENS, SKILL, DEP, PRIM, STYLE, TRANS), 31-row source matrix with per-vertical FM006 stance vocabularies, SQLite FTS5 manifest, atomic publish + retraction log, 11 task-shaped SKILL.md routers, per-vertical `_source_role_map.md` + `_dependency_order.md` + `_diagram_primitives.md` + `_style_guide.md` + `_chapter_overviews.md`. Read-only; treat as design oracle, never modify.
- `/home/jakeshea/humanize/` — Claude Code plugin v1.17.0 implementing RLCR (Ralph Loop + Codex Review). Provides the round-summary contract (`## Knowledge Consulted`) and BitLessons capture that CACG integrates with.
- `/home/jakeshea/knowledge_base_framework_discovery/_research/` — three SOTA snapshots: CFA framework summary, humanize harness summary, 2025-2026 web research (atomic-note systems, context engineering, citation linting, PDF+LLM pipelines, hybrid retrieval, agentic frameworks, content-addressable cards, golden tests, streaming verification, Pydantic, hash-pinning, RAG failure modes).
- Current CACG state — MVP plan (Round 45 + finalize) and Trust-Depth plan (Round 9 + finalize) both shipped. Current capabilities: `cacg.v0` schema, three-phase atomic publish, content-addressable card + chunk hashing, Layer-1 mechanical lint (FM/CITE/HASH/IDX/CLI/INGEST/MAN/JNL/HIST/RS/PERF), Layer-2 normalized exact-substring verification + opt-in fuzzy + BM25 hint-only, `ChunksIndex` process-local cache, mandatory `--source-matrix` authorization (CACG-AUTH-*), three-grain retraction at card/source/chunk (CACG-RETR-*), `kb verify --round-summary` humanize integration, deterministic byte-identical output under `KB_FROZEN_CLOCK=1`, 1000-card stress fixture in CI + 10000-card local-only stress documented.

## Problem (stated user goals for this phase)

Two goals frame this phase. They are correlated: progressive disclosure of cards reduces both context spend AND retrieval miss rate.

1. **Save context for sub-agents.** Sub-agents that consult the KB today have only `cards_manifest.json` (flat metadata: id/title/reading_id/card_hash/citation_count) + `INDEX.md` (flat list of paths with truncated card_hash) + the full card files. They either bulk-load too much (full bodies, blowing context) or too little (manifest only, no semantic surface to route on). The framework has no `summary` semantics enforced (the schema allows `summary: ""`), no `tags`, no `kb search`, no preloadable card-of-cards artifact, no task-shaped SKILL.md routers.

2. **Improve accuracy / recall.** Layer-2 is mechanical exact-substring containment with `--fuzzy` Levenshtein escape and `hint_only=true` BM25 diagnostic candidates. Paraphrased misquotes that pass exact-match (semantically equivalent but textually different — "framework rejects duplicates" vs "rejects collisions") slip through. There is no Layer-3 semantic check. Additionally, cards have no structural cross-references — no `depends_on` / `extends` edges, no per-reading role map declaring primary-anchor / supporting-anchor / source-stance — so orphan citations, dangling backing-links, and missing role-map coverage are undetectable.

## Three combined sub-goals (A + B + C)

### A. Progressive-disclosure retrieval contract

What this delivers:
- Promote `summary` to required (Pydantic, bounded char count, schema-enforced).
- Add `tags: list[str]` (bounded count, lowercase-slug regex, schema-enforced).
- New `kb search <query>` verb: deterministic BM25 (using existing `rank-bm25` dep from `cacg.verify.bm25_hints`) over title + summary + tags; bounded top-K; sorted ranking with deterministic tiebreak by card_id.
- New `summaries.json` artifact at `kb index` time: sorted `{id, title, summary, tags, reading_id, path, card_hash}`. Preloadable card-of-cards for eager sub-agent ingestion; lazy `kb show <card_id>` on full body.
- Task-shaped SKILL.md router schema (Anthropic Skills progressive-disclosure pattern): per-reading optional `cards/reading_<NN>/SKILL.md` with `name:` + `description:` (used as routing trigger) + optional `routes_to: [card_id]`.
- New diagnostic families: `CACG-SUM-*` (missing/oversized summary, summary-body divergence heuristic, tags non-conforming) and `CACG-SKILL-*` (router schema violations, `routes_to` references unknown card_id, name collisions across readings).

### B. Layer-3 semantic verifier (opt-in, deterministic-by-default)

Two implementations to compare in the directed-swarm:
- **B1: Local embedding-cosine.** Pinned small sentence-transformer (e.g., `all-MiniLM-L6-v2`) under a `--semantic` flag. Embedding cache committed to disk keyed by `chunk_hash + claim_window_hash`; same input → same float bytes → same verdict. Deterministic under `KB_FROZEN_CLOCK=1` if model + cache are pinned. Trades off network independence vs introduces a ML/embedding dependency.
- **B2: LLM-as-judge via Claude API.** `--semantic-judge` flag invokes `claude-haiku-4-5` with structured `{verdict: pass|fail|abstain, score: float, reasoning: str}`. Non-deterministic by construction; CI-only mode, never on default verify path.

Both implementations:
- Trigger ONLY when Layer-2 exact-match fails AND the semantic flag is explicit.
- Emit `CACG-VERIFY-002` with score + verdict; preserve per-card journal cardinality.
- Default remains: Layer-2 mechanical is the gate. Semantic is escape hatch + accuracy lift.

### C. Dependency DAG + per-reading role map

What this delivers:
- Additive frontmatter: `depends_on: list[card_id]`, `extends: list[card_id]` (additive to `cacg.v0` schema; default `[]` for backward compatibility).
- Validation: unknown `card_id` → `CACG-DEP-001`; cycle detection → `CACG-DEP-002`; orphan card detection (no inbound dep edges, no inbound role-map references, no inbound citations from any active card → `CACG-DEP-003`).
- Per-reading `source_role_map.json` artifact (deterministic canonical JSON, separate file per reading_id): for every card_id, primary `(source_id, page_span)` + supporting list + relevance-tier vocabulary (configurable per reading) + source-stance vocabulary (configurable per reading, mirrors CFA's per-vertical FM006 stance branch).
- Validation: symmetric with `cards_manifest` (every active card has exactly one role-map entry; every role-map entry maps to an active card); stance vocabulary closed-set per reading; relevance vocabulary closed-set per reading.
- New diagnostic family `CACG-ROLE-*` (missing role-map entry, unauthorized stance value, unauthorized relevance value, role-map ↔ card-set asymmetry).

## Non-goals (explicitly deferred this phase)

- Full SQLite FTS5 migration. `kb search` uses `rank-bm25` in-memory over a `SummariesIndex` extending `ChunksIndex`. SQLite remains behind the boundary.
- Multi-LLM concurrent judging (LLMChallenge two-judge cross-check).
- Card chunk re-embedding pipeline for retrieval-side (separate from Layer-3 verification embedding).
- Streaming per-line lint during authoring (research-noted, not built).
- HTML deliverables (`build_html_cards.py` / `build_html_summary.py` / `build_html_volumes.py` from CFA stay deferred).
- Diagram primitives + ASCII whitespace-SHA256 enforcement (CFA's PRIM-002/PRIM-005).
- Full 41-code CFA lint parity. Add only what A+B+C contracts demand.

## Constraints (non-negotiable)

- All current tests stay green (255+ from MVP + trust-depth additions).
- Byte-deterministic output under `KB_FROZEN_CLOCK=1` preserved on the default path.
- `cacg.v0` schema purely additive; no breaking change to existing manifests.
- Three-phase atomic publish contract unchanged; `summaries.json` and `source_role_map.json` either join the existing pair-atomic publisher (becoming triplet/quadruplet) or land as a separate Phase D after cards are committed.
- `--semantic` opt-in only; never default; never on the default CI gate (until B1 reproducibility is demonstrated cross-machine).
- No `--no-verify`, no hook-skipping flags, no destructive operations on card .md files.
- Permitted deps: `rank-bm25` (already used in `cacg.verify.bm25_hints`), `pydantic>=2`, `pypdfium2` (ingest only), `pytest`. Optional new dep for B1: `sentence-transformers` (or equivalent small embedding library); must be optional install.
- No emoji, no CJK in code/docs.
- No remote network on default `kb verify` path. B2 semantic-judge is opt-in CI-only.
- Path boundaries: extension only of `src/cacg/` and `cards/`; new fixtures under `tests/golden/`, `tests/adversarial/`, `tests/perf/`; documentation under `docs/`.

## Open questions for the directed swarm to address

- **`kb search` index granularity.** BM25 over (title + summary + tags) only, vs (title + summary + tags + body)? Body inclusion lifts recall but bloats index and breaks the eager-load size budget.
- **`summaries.json` publish ordering.** Integrate as a third file in the pair-atomic publisher (becomes triplet-atomic), OR publish as a separate Phase D after `cards_manifest.json` + `INDEX.md` land?
- **Layer-3 implementation choice.** B1 deterministic embedding-cosine vs B2 LLM-judge — pick one for MVP, or build both behind one `--semantic` umbrella with sub-modes?
- **Embedding model pinning under frozen clock (B1).** Pinned model SHA256 + frozen embedding cache committed to disk keyed by `chunk_hash + claim_window_hash`? What's the cache eviction / rebuild policy?
- **DAG edge semantics.** `depends_on` + `extends` only, or also `contradicts` / `applies_to` (citation `edge_type` enum already supports the latter two — should card-to-card edges mirror citation edge types)?
- **Role-map vocabulary.** Hardcoded enum vs per-reading configurable closed-set (CFA does the latter; e.g., subcorpus 02 economics admits `{primary-cfa, primary-cfa-notes, primary-mwg, primary-romer}`)?
- **SKILL.md router contract.** Anthropic Skills minimal (`name:` + `description:`) only, or extend with `routes_to: [card_id]` + `trigger_keywords: [str]` (the CFA pattern)?
- **Existing-corpus migration story.** What does `kb migrate-summaries` look like? Auto-populate empty summaries from first-paragraph heuristic, or require manual fill with a `--strict` gate?
- **Stress validation envelope.** New 1k / 10k benchmarks: `kb search <q>` < ?ms; `kb verify --semantic` < ?ms; `summaries.json` build < ?ms? Should the 10k stress fixture stay local-only or join CI?
- **Backward-compat sequencing.** Land A first (additive schema fields default-empty; tests adjust), then C (additive frontmatter + new artifact), then B (additive flag + new diagnostic + optional dep)? Or parallelize where state is independent?

## Suggested orthogonal directions for the directed swarm

The directed swarm should diverge across at least these axes:

1. **Direction "retrieval-first"** — A is the spine; B and C are secondary. Minimum schema commitment. Optimizes for the explicit "save sub-agent context" goal. New deps: `rank-bm25` only (already present). SKILL.md routers as the headline feature.
2. **Direction "verification-first"** — B is the spine; A and C are secondary. Closes the paraphrase-misquote accuracy hole. Pins embedding model + frozen cache discipline. New deps: `sentence-transformers` (optional).
3. **Direction "parity-first"** — C is the spine; A and B are secondary. Achieves structural parity with CFA's role-map + dependency-order + SKILL.md infrastructure. Maximum schema commitment per reading.
4. **Direction "safe sequencing"** — A first, then C, then B. Each milestone lands before the next depends on it. Minimal concurrent surface. Slow but low-risk.
5. **Direction "max velocity"** — A and C parallelized (independent state: A operates on card-level summary; C operates on cross-card edges + per-reading role map). B as Phase 2. Highest throughput; needs careful merge discipline.
6. **Direction "minimum-viable cut"** — Argues A+B+C is too ambitious in one phase; recommends only A (the explicit-goal direct hit) and explicitly defers B and C. Honest "no" direction that lets the synthesis push back on the user's combined scope.

The synthesis picks the strongest as PRIMARY, renders the rest as Alt-1..Alt-K with explicit tradeoffs.

## Success criteria for the seed exploration

The output of `/humanize:gen-idea` should produce a `.humanize/ideas/<slug>-<timestamp>.md` draft that:

- Identifies the PRIMARY direction with a 3-5 paragraph approach summary, objective evidence (specific files / line numbers / existing schema), known risks (deterministic-cache discipline, optional-dep packaging, schema-bump avoidance), and confidence rating.
- Renders Alt-1..Alt-K alternatives with the same shape but at lower depth.
- Surfaces 1-3 open decisions (DEC-*) that need user input before `/humanize:gen-plan` can converge.
- Names the load-bearing files in `src/cacg/` that each direction would touch (cli.py, schema.py, frontmatter.py, index.py, manifest.py, lint/codes.py, lint/layer1.py, verify/layer2.py, verify/runner.py, integrate/round_summary.py, chunks_index.py).
- Identifies the new test fixtures + adversarial cases each direction requires.
- Lays out the schema additions explicitly (e.g., `summary: str`, `tags: list[str]`, `depends_on: list[str]`, `extends: list[str]`) and notes the additive nature.
