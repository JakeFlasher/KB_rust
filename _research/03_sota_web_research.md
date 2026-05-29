# SOTA Web Research (2025-2026): KB Frameworks for Agentic Workflows

Synthesis of web research across 10 topic areas. Full source links at end.

## 1. Knowledge-card / atomic-note systems for LLM agents

- **A-Mem** (arXiv 2502.12110): agentic memory with memories generating their own contextual descriptions, forming links, evolving. Distinguishes agency at the *storage* level, not just retrieval.
- **Karpathy's "LLM Wiki" pattern**: reframes RAG as knowledge management — "maintain an accurate, growing representation of what you know." MCP primitives: `wiki_search`, `wiki_ingest`, `wiki_lint`, `wiki_graph`.
- **Atomic** (atomicapp.ai): atoms = markdown notes chunked, embedded, tagged, linked by semantic similarity; synthesizable into wiki articles.
- **Andy Matuschak Evergreen notes**: atomic, densely linked, titles "are like APIs" — interface contract for retrievability.

**Patterns to borrow:** atomic cards with stable API-like titles, contextual metadata + embedding-ready body, pre-synthesized cards beat raw retrieval chunks.

**Watch-outs:** evergreens are "not optimal for skill acquisition" when most notes are unoriginal — CFA material falls here; cards should be pragmatic, not strictly Zettelkasten.

## 2. Context engineering for agentic workflows

**Anthropic canonical**: "Effective Context Engineering for AI Agents":

- Context is finite; **context rot** degrades quality well before hard limit.
- Just-in-time strategy: lightweight identifiers, dynamic resolution at runtime via tools.
- Compaction: summarize conversation, reinitialize with summary. Preserve architectural decisions, unresolved bugs, implementation details; discard redundant tool outputs.
- Structured note-taking: agents maintain NOTES.md-style persistent memory outside context.
- Sub-agent architectures: tens of thousands of tokens internally, returns 1-2k condensed summary.

**LangChain Deep Agents**: tool response >20k tokens → offload to filesystem, substitute with path reference + 10-line preview.

**Hermes Agent**: structured summarization with `_summarize_tool_result` collapsing large outputs to 1-line summaries; structured summary contains Resolved Questions, Pending Questions, Progress, Decisions, Remaining Work.

**ACE — Agentic Context Engineering** (arXiv 2510.04618, Stanford + SambaNova): addresses **context collapse**. Three roles (Generator → Reflector → Curator), incremental delta updates instead of full rewrites, grow-and-refine bullet-merging. +17.1% AppWorld accuracy, 86.9% less adaptation latency.

**Manifest + lazy-load**: AGENTS.md present empirically gives 29% runtime reduction + 17% token reduction.

**Anthropic Skills progressive disclosure**: metadata at startup, on-demand load of full content.

**Patterns to borrow:** card-of-cards (eagerly loaded), full cards (lazily loaded via tool), subagent-per-task isolation.

## 3. Citation / claim-source linting

- **Two failure classes**: citation doesn't resolve to anything real, vs citation resolves but doesn't support claim. **Both must be linted separately.**
- **Span-level verification**: each assertion matched against retrieved passages; confirm, flag unverified, identify contradiction.
- **GSAR** (arXiv 2604.23366): claim typology `grounded | ungrounded | contradicted | complementary`; asymmetric contradiction-penalized score; three-tier decision proceed | regenerate | replan.
- **Citation-Grounded Code Comprehension** (arXiv 2512.12117): *architectural* citation enforcement — "LLMs cannot cite code they haven't seen." 92% citation accuracy, zero hallucinations via mechanical verification. **Adopt as design principle.**
- **PaperTrail** (CHI 2026): claim-evidence interface highlights "unsupported answer claims and omitted paper claims."
- **CiteGuard** (arXiv 2510.17853): given an excerpt, search for paper matching missing citation.
- **FACTS Grounding** (Google DeepMind): grounds long-form responses to document context up to 32k tokens.

**Patterns to borrow:** two-stage citation lint (exists + supported); mechanical verifier (regex / fuzzy match) for most cards; LLM-judge only for "semantically supported" questions.

## 4. PDF + LLM verification pipelines

- **Marker-PDF** (34.4k GitHub stars): `--use_llm` flag for accuracy-critical docs; multiple LLM backends.
- **Docling** (IBM Research): structured DoclingDocument preserving semantic hierarchy; skips OCR when PDF has selectable text.
- **olmOCR**: fine-tuned Qwen2-VL-7B-Instruct; **olmOCR-Bench uses binary unit-test rules**, deliberately avoids LLM-as-judge. **Worth following.**
- Production pipeline pattern (Microsoft Azure, Unstract):
  - OCR/text with layout preservation
  - Per-page classification with confidence
  - Layout analysis (LayoutLMv3, DiT, Table Transformer)
  - **Parallel/dual extraction** (Azure Content Understanding + DSPy)
  - **LLMChallenge**: second LLM challenges primary; disagreement → null. "A null value is preferable to a wrong value."
  - Schema-constrained output (Pydantic / JSON grammars)
  - HITL for disputed fields.

## 5. Hybrid retrieval for verification

- **Hybrid (BM25 + dense) is the new baseline.** Dense retrieval silently fails on exact lexical identity (error codes, product SKUs, exact quoted phrases).
- For **exact-text verification**, BM25 is essential — cards quote exact phrases.
- Benchmark (arXiv 2604.01733): Hybrid + Cohere Rerank Recall@5 = 0.816 vs BM25 alone 0.644, dense alone 0.587.
- **RRF (Reciprocal Rank Fusion)** prevents suppression of documents highly ranked by only one pipeline.
- **HyDE can hurt fact verification** — pseudo-documents introduce noise. **Avoid in verifier.**
- **Anthropic Contextual Retrieval**: prepend 50-100 tokens of chunk-specific context before embedding/BM25 indexing. -49% top-20 retrieval failure rate; with reranking -67%. ~$1.02 per million doc tokens with prompt caching.

**Recommended hybrid for verifier:**

- BM25 over exact quoted snippet (fast, deterministic).
- Embedding similarity over surrounding claim (semantic fallback).
- RRF fuse; top-3; check claimed page matches.
- Page-window constraint: if card cites page 47, score chunks within ±3 pages only.

## 6. Agentic workflow frameworks 2025-2026

Stabilization into specialization:

| Framework | Best for | 2026 |
|-----------|----------|------|
| LangGraph (v0.4) | Production, complex state, HITL, audit | Surpassed CrewAI |
| CrewAI | Fast prototypes, role-based | Added Flows |
| AutoGen / AG2 | Multi-agent debate | Maintenance mode |
| Claude Agent SDK | MCP-native, plugin/skill | Matured |
| DSPy | Declarative compiled programs | Pydantic-first; ReAct + MIPROv2 |

**Anthropic Building Effective Agents** five patterns canonical: prompt chaining, routing, parallelization, orchestrator-worker, evaluator-optimizer.

**Antipatterns:** monolithic agents, over-engineered planning, missing observability.

**Claude Agent SDK Skills**:

- Filesystem-based, **progressive disclosure** (metadata → instructions → resources).
- "No practical limit on bundled content" — files don't consume context until accessed.
- Two types: capability uplift vs encoded preference.
- Open standard, portable across Claude/Cursor/Gemini CLI/Codex CLI/Copilot.

**Substrate for us:** Claude Agent SDK + Skills + Subagents.

## 7. Memory & long-running agent state

- **Letta (formerly MemGPT)**: three-tier (core memory always in context, recall searchable history, archival cold storage). V1 (Oct 2025) rearchitected for GPT-5 + Claude 4.5 native memory tools. Lock-in risk: agents run *inside* Letta runtime.
- **Mem0 vs Letta**: Mem0 wins consumer "remember the user"; Letta wins long-horizon coherence (30-day continuous, 500+ interactions).
- **GraphRAG ~35% precision lift** over vector-only for multi-hop.
- **Anthropic's surprising pivot**: Claude Code abandoned RAG for **agentic search** (grep/glob/file reads). "Agentic search outperformed everything. By a lot." Avoids security/privacy/staleness/reliability issues of vector DBs.
- **The RAG Bifurcation thesis**: under 1M tokens → context window directly; over 1M → hypergraphs with iterative reasoning. Middle ground (chunk-embed-top-5) "legacy technology."
- **Anthropic Knowledge Graph Cookbook**: Haiku for high-volume schema-constrained extraction; Sonnet for entity resolution and summarization.

## 8. Repo-grounded LLM tools

- **Cursor**: chunk-hash-keyed embedding cache; unchanged code reuses embeddings. **Gold pattern for PDFs:** content-hash-keyed cache means unchanged chunks never re-embed.
- **CocoIndex**: Tree-sitter syntax-aware chunking + incremental processing.
- **Git-based incremental** (wiki ingestion): records last commit; future ingests look at git diff only. **For PDFs: pin to source hash; reprocess only when source changes.**
- **AgentForge**: "dominance of localization errors suggests improvements in repository understanding may yield larger gains than further scaling the base model."
- **Recursive Language Models** (MIT CSAIL): model gets searchable environment, queries for what it needs, drills deeper.

**Claude Code itself** uses agentic search (no vector DB) for code — strong signal for direct grep/read over PDF index.

## 9. Specific features evaluated

### Content-addressable card identity
- **Hashcards**: spaced-repetition system; cards identified by hash of text; progress reset on edit.
- **Pharma "context units"**: hash extracted snippets; Merkle root over selected units for audit.
- Pattern: CID = hash(content); any change → new CID → mechanical staleness.

### Append-only event-log vs edit-in-place
- **Event Sourcing**: full history append-only; current state materialized. Strategies: upcasting (transform older schemas), in-place migration (last resort).
- For cards: hybrid — card file is current state; `cards/<id>.history.jsonl` is append-only diff log.

### Card sharding / namespacing
- **Hash function assigns docs to shards; parallel search via ThreadPoolExecutor; batch ingestion.**
- Claude Agent SDK plugin namespacing: `plugin:skill`. Map: `reading_47:card_credit_risk_concepts`.
- For CFA 200+ readings: each reading is a namespace. **Paths: `cards/reading_<num>/<slug>.md`.**

### Test-card / golden-card patterns
- **DeepEval / Confident AI**: **Goldens** = precursor edit-heavy fixtures; **test cases** = immutable, post-evaluation, with pass/fail.
- **Trajectory + Response eval** (Google ADK): "golden dataset is snapshot of agent performing correctly... captures Trajectory and Final Response."
- CI/CD integration: evaluations in pytest, GitHub Actions blocks deploy on regression.
- For us: canonical **golden cards** (hand-verified) + **adversarial cards** (known errors). Linter must catch all bad, pass all good — test suite for the linter.

### Streaming verification while authoring
- **LangChain v1.3 event streaming**: typed-projection API.
- **vLLM Realtime API** (Jan 2026): streaming input + output.
- Production pattern: stream card output, run cheap deterministic lint per-line as arrives (citation format regex, quote delimiters), expensive checks (BM25 + embedding) once full card complete.

### Pydantic schemas
- **Pydantic v2**: 3.5× faster than JSON Schema; type system integration.
- **Instructor**: patches LLM client to enforce Pydantic; auto-retries on validation failure.
- **Pydantic AI**: declarative agents with typed output, retry-with-error-feedback loop.
- For us: YAML frontmatter on disk + Pydantic at lint time.

### Hash-pinned source chunks
- Chunk PDF, hash each chunk, store `(chunk_id, hash, page, text)` in manifest.
- Cards reference `chunk_id + chunk_hash` assertion.
- If PDF changes, every dependent card flags itself stale.
- **Strongest "no silent drift" guarantee available.**

## 10. What's NOT working well

**RAG failure modes 2026:**

- 72% of enterprise RAG deployments fail in first year.
- **Context collapse** (temporal, organizational, conversational).
- **Retrieval thrash**: agent oscillates broaden → narrow → broaden. LangGraph's own tutorial had this bug; required `rewrite_count` cap.
- **Tool storms**: redundant lookups burn tokens.
- **Context bloat**: overwhelms with evidence then acts on wrong snippet.
- **Hidden metadata-embedding misalignment**: "no error raised, no flag set; corrupted signal looks correct at every layer."

**Claude Code skills/plugins pain points:**

- **Layer confusion**: behavioral constraint that should be a hook gets written into system prompt; reusable workflow that should be a skill gets copy-pasted.
- **Plugin bloat**: "12+ plugins in first week, three weeks unwinding conflicts."
- **CLAUDE.md grows into procedures document.**
- **Hook JSON parsing errors** from shell profile `echo` polluting stdout.
- **Skill activation**: `description` is fuzzy-matched trigger, not docs. Vague → never fires; overlapping → wrong one fires.
- **Hooks have no Claude awareness** — they run in background; don't expect them to influence reasoning.

**Lessons for new framework:**

- Hard caps on retry/loop iterations.
- Manifest-driven discovery, not bulk-loading.
- Separate hooks (deterministic, no LLM) from skills (LLM workflows).
- Don't grow CLAUDE.md/MANIFEST.md with prose — keep it a card index.

## Actionable design ideas (12+ ranked by impact)

1. **Content-hash-pinned source chunks** — non-silent-drift guarantee. Parse PDFs → chunks → SHA-256 per chunk; cards reference `chunk_id + chunk_hash`. (Hashcards + ISPE context-units.)
2. **Two-tier card storage**: `index.md` eagerly loaded + `<slug>.md` lazily loaded. (AGENTS.md + Claude Skills progressive disclosure.)
3. **Mechanical multi-layer citation linter**:
    - Layer 1 (regex/format, μs): citation present, page in valid range, quote in fenced block, source_id known.
    - Layer 2 (exact-match BM25, ms): quoted phrase appears in cited chunk or ±N pages. **Workhorse.** Uses `tantivy` / `rank-bm25` over manifest.
    - Layer 3 (semantic verifier, s, optional): embedding cosine or LLM-judge for "semantically supported" — only for claims that fail Layer 2.
4. **Golden-card test suite**: `tests/golden/` (known-good) + `tests/adversarial/` (planted errors). Linter passes all goldens, fails all adversarials. CI gate.
5. **Subagent-per-reading for context isolation**: ingest with subagent that sees only one reading's PDF + schema + style; returns validated cards.
6. **Append-only card history + in-place current state**: `cards/<reading>/<slug>.md` current; `cards/<reading>/<slug>.history.jsonl` append-only diff log.
7. **Pydantic schema for card frontmatter**, Instructor for authoring retry-with-error-feedback.
8. **Two-pass authoring with LLMChallenge** (different models for author vs challenge).
9. **Streaming per-line lint during authoring** — interrupt on malformed citation before full regeneration.
10. **Card sharding by reading namespace** — each reading has own manifest, BM25 index, embedding shard.
11. **Embedding + BM25 caches keyed by chunk hash** — re-indexing 200 readings near-instant after first.
12. **Hard loop caps + observability**: `max_card_revisions: 3`; `max_lint_retries: 2`; structured JSON lint logs; `lint_journal.jsonl` append-only.
13. **Skills + Subagents architecture, not monolithic** — `kb:authoring` (style), `kb:lint` (linter), `kb:verify` (BM25+embed in isolation).

## Selected sources

- Anthropic — Effective Context Engineering for AI Agents
- Anthropic — Building Effective AI Agents
- Anthropic — Contextual Retrieval
- Claude Agent SDK Skills Docs
- A-Mem (arXiv 2502.12110)
- Karpathy LLM Wiki Pattern
- Atomic (atomicapp.ai)
- Andy Matuschak Evergreen Notes
- ACE (arXiv 2510.04618)
- GSAR (arXiv 2604.23366)
- Citation-Grounded Code Comprehension (arXiv 2512.12117)
- olmOCR Paper
- Hybrid Search Production (TianPan blog)
- BM25 to Corrective RAG benchmarks (arXiv 2604.01733)
- Marker-PDF, Docling
- LangChain Deep Agents — Context Management
- Hermes Agent — Context Compression
- Cursor codebase indexing (Towards Data Science)
- CocoIndex incremental indexing
- Hashcards (eudoxia0/hashcards)
- ISPE Content Addressability in Knowledge Management
- DeepEval / Confident AI Goldens
- LangGraph vs CrewAI vs AutoGen 2026 comparisons
- DSPy Pydantic-first
- RAG Bifurcation thesis (mmntm.net)
- Agentic RAG Failure Modes (Towards Data Science)
- Anthropic Knowledge Graph Cookbook
- Letta V1 rearchitecting
