# Seed Idea — A Successor Framework for Agentic Knowledge-Base Authoring & Verification

## Context (read-only references)

- `/home/jakeshea/CFA_reading/` — a working, 219-card CFA-curriculum knowledge base built from quotable PDFs (10 of 17 topic verticals complete; 41-code linter + atomic FTS5 manifest publish; 80 pytest fixtures). See `_research/01_cfa_reading_framework_summary.md`.
- `/home/jakeshea/humanize/` — a Claude Code plugin (RLCR = Ralph-Loop with Codex Review) that orchestrates gen-idea → gen-plan → iterative implementation with independent Codex review, BitLessons capture, Knowledge-Consulted provenance contracts. See `_research/02_humanize_harness_summary.md`.
- `/home/jakeshea/knowledge_base_framework_discovery/_research/03_sota_web_research.md` — 2025-2026 SOTA web research (atomic notes, context engineering, citation linting, PDF verification, hybrid retrieval, agentic frameworks, content-addressable cards, golden tests, streaming, Pydantic, hash-pinning, RAG failure modes).

## Problem

The CFA_reading framework is mature but has friction that scales with the corpus:

1. **Context for agentic workflow** is not first-class. The 41-code linter, the 31-row source matrix, the 11 SKILL.md routers, the SQLite FTS5 manifest, and the 7 round-summary files in each `.humanize/rlcr/<ts>/` together represent enormous machine-readable state — but loading the *right slice* into an agent's context for the *right task* is ad-hoc, with no formal contract for what to load when.
2. **Linting checks citation _bounds_ but not citation _content_**. CITE-006 verifies page-range bounds via `pdfinfo`. There is no automated check that the quoted phrase in a card actually appears in the cited PDF page span. The mechanism `_corpus_planning/05_source_matrix.md` SHA256-locks the *file*, but cards can silently drift from the *text*.
3. **No content-addressable card identity.** Card edits don't bump a hash; staleness detection relies on git diff plus matrix SHA256 plus retraction log. A card that re-cites a stale source after the source was replaced would not auto-flag at lint time without a retraction trigger.
4. **The harness (humanize) does not verify the knowledge it consults.** "Knowledge Consulted" is a post-hoc text-pattern Codex audit. Codex checks the *list* of paths, not the *content* of the cards. There is no active "card-content-verified" handshake during the RLCR loop.
5. **Authoring is not streaming-aware.** A card is written, then linted; a malformed citation surfaces after the full card is generated, requiring full regeneration cost.

## Goals (must-have)

1. **Save agentic context.** Card discovery, retrieval, and verification must minimize tokens loaded into the orchestrating agent. The new framework should make "the right card at the right time" the default, with progressive disclosure as a first-class primitive.
2. **Fast linting / verification of knowledge cards against the knowledge source.** Verification must be (a) mechanical where possible, (b) hash-pinned so silent drift is impossible, (c) staged so cheap checks fail fast before expensive ones, (d) < 100ms per card for the common case.

## Desirable features

- **Content-hash-pinned source chunks** so cards reference `chunk_id + chunk_hash`; any source change auto-stales all dependents.
- **Append-only history per card** so refactors are auditable without git archaeology.
- **Sharding/namespacing per reading** so the 200+-reading vision scales.
- **Golden + adversarial card test suites** as CI gates for the linter itself.
- **Streaming per-line lint** during authoring so malformed citations fail fast.
- **Pydantic-validated frontmatter** for machine-readable card schema.
- **Skills + Subagents architecture** (not monolithic): `kb:authoring`, `kb:lint`, `kb:verify`.
- **Manifest-driven discovery** (small eagerly-loaded `INDEX.md`-of-cards) + lazy full-card loading.
- **Compatibility shim** so existing CFA_reading cards can be ingested (read-only reference) and verified without modification.
- **Plugin packaging** so the framework drops in next to humanize as a sibling Claude Code plugin (or extends humanize with a `kb:` namespace).
- **Provider-portable**: works with Claude, optional Codex/Gemini sidecars for two-pass authoring (LLMChallenge pattern).
- **Privacy mode** consistent with humanize's `--privacy` flag.
- **Hard loop caps + observability**: structured JSON lint logs, append-only `lint_journal.jsonl`, max-retry budgets.
- **CLI ergonomics**: `kb lint <card>`, `kb verify <card>`, `kb ingest <pdf>`, `kb index <reading>`, `kb new <reading> <slug>`.

## Constraints

- `../CFA_reading` and `../humanize` are **read-only**. We may import, reference, or shim — never modify.
- No emoji, no CJK in code/docs (humanize convention).
- Plain Markdown + YAML + Python stdlib + a small set of vetted libraries (`rank-bm25` or `tantivy-py`, `pydantic>=2`, `pypdfium2`, `pytest`). No pandoc, no graphviz, no JS frameworks, no remote network calls in the verifier path.
- Atomic operations where reasonable (manifest publish, index rebuild).
- Deterministic gates: same inputs → same exit code + same diagnostic output.
- The MVP needs to be a **minimal working example** demonstrable on a small slice of CFA cards (perhaps re-using one of the 10 closed verticals as fixtures, with cards copied/linked into the local repo).

## Out of scope (for v0/MVP)

- Re-writing humanize. We integrate, we don't fork.
- A new RAG retriever for *generation*. The verifier is what we need; generation is humanize's lane.
- A web UI / dashboard. CLI + structured logs is enough.
- A full migration of CFA_reading's 41-code linter. We pick the high-value subset for v0 and document the gap.

## Solution surface to explore (non-prescriptive — directed swarm should diverge!)

Rough mental model of the new framework as **three concentric loops**:

```
┌─────────────────────────────────────────────────────────────┐
│  L3: AUTHORING LOOP (humanize RLCR orchestrated)            │
│   - gen-idea → gen-plan → start-rlcr-loop                   │
│   - Per-round summary includes Knowledge Consulted          │
├─────────────────────────────────────────────────────────────┤
│  L2: CARD-LIFECYCLE LOOP (kb plugin's domain)               │
│   - kb new → kb stream-lint → kb verify → kb commit         │
│   - Streaming per-line lint as Claude authors               │
│   - Hash-pinned chunks for verifier                         │
├─────────────────────────────────────────────────────────────┤
│  L1: VERIFICATION ENGINE (deterministic, mechanical)        │
│   - Stage 1: regex/format (μs)                              │
│   - Stage 2: BM25 exact-match against hash-pinned chunks (ms)│
│   - Stage 3: embedding cosine / LLM-judge (s, optional)     │
└─────────────────────────────────────────────────────────────┘
```

Open questions for the directed swarm to address from orthogonal angles:

- **Granularity of hash-pinning**: page-level? section-level? span-level (sentences)?
- **Index format**: SQLite FTS5 (CFA precedent) vs Tantivy vs duckdb-fts vs flat JSONL?
- **Card schema evolution**: schema_version on every frontmatter? Compatibility shim?
- **Plugin vs library**: ship as Claude Code plugin? Pure Python CLI? Both?
- **Streaming hook surface**: PreToolUse Write hook? Watch-mode file daemon? Inline LLM-side guardrail?
- **Two-pass LLMChallenge integration**: optional Codex sidecar at author time? Just at verify time?
- **Golden cards**: hand-curate from 219 existing CFA_reading cards? Or synthesize new minimal fixtures?
- **MVP demo target**: pick one CFA reading and produce 3-5 cards end-to-end (ingest PDF → produce cards → lint → verify → archive)?

## Success criteria for MVP

- A short demo (`make demo` or equivalent) showing:
  1. Ingest a small PDF → produce a chunk manifest with hashes.
  2. Author 2-3 cards (manually or with `kb new` template).
  3. `kb lint` passes; introduce a citation drift; `kb lint` fails clearly.
  4. `kb verify` confirms quoted text appears in chunk; introduce a fake quote; `kb verify` fails clearly.
  5. `kb index` produces a small `INDEX.md` of cards plus a Pydantic-validated manifest.
  6. The whole demo runs in < 30s on a laptop, with a `lint_journal.jsonl` audit trail.
- Tests: pytest suite with at least one golden card + one adversarial card per failure mode.
- Documentation: a `README.md` explaining the three loops, the schema, and how to extend.
- Integration: a one-line example of invoking `kb verify` from within a humanize RLCR round summary (the Knowledge Consulted contract gets a content-verified handshake).

## What success would unlock

- The 219-card CFA_reading corpus could be re-verified content-wise in minutes, not days.
- New readings could be onboarded with a single `kb ingest` followed by streaming-lint-guided authoring.
- Humanize's RLCR loop gains a hard verification gate ("Codex reviewed *the summary*; the verifier reviewed *the knowledge it cited*").
- The framework generalizes beyond CFA: any PDF-grounded knowledge domain (medicine, law, engineering specs) can plug in.
