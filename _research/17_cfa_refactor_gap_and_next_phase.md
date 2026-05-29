# CFA_reading Refactor — Gap Analysis and Next-Phase Options

**Date:** 2026-05-23
**Status:** Strategic deep research / scoping. No code changes implied.
**Author role:** senior software architect — next-phase decision for the CACG Rust port.
**Scope:** Decide what the NEXT PHASE of the CACG Rust port should be, measured against
the user's ultimate goal: *refactor the legacy `../CFA_reading` corpus into a
fine-grained, ultra-fast, high-performance, high-accuracy/high-recall knowledge base,
using the CACG framework.*

**Method:** Read the live `../CFA_reading` corpus (README, CLAUDE.md, STATUS.md,
REMAINING_BOOKS.md, `_corpus_planning/`, `.humanize/plans/`, and sample card files in the vertical
directories); read the CACG repo's `_research/01,03,07,09,12,13,15`, all of `.humanize/plans/` and
`.humanize/drafts/`, `docs/{schema,retrieval,semantic-verifier,diagnostic-parity}.md`, `README.md`,
the Rust workspace `Cargo.toml`, the `crates/` source layout, and the committed M3
first-bite artifacts under `tests/parity_corpus/cfa_first_bite/`. Every crate version pin
in §6 was cross-verified against the live `crates.io` JSON API on 2026-05-23. No external
LLM consultation was used; all analysis is first-hand from code and docs.

This report is a companion to the in-flight fine-grained M5 audit (a parallel agent owns
that); it carries only enough port-state awareness to weigh next-phase options.

---

## 1. Ultimate-Goal Decomposition

### 1.1 The goal, stated precisely

> Take the ~268-card legacy `../CFA_reading` knowledge base — built on a pre-CACG
> line-based pseudo-YAML card format with source-only SHA hashing — and **re-author it as
> a CACG-v0 corpus**: chunk-hash-pinned citations, atomic tamper-evident publish,
> deterministic gates, and a retrieval surface (`kb search` / `kb show` / FTS5 sidecar)
> that is *fast*, *high-accuracy* (citations provably grounded in the cited PDF span),
> and *high-recall* (a query reliably surfaces every relevant card).

"Refactor … using the CACG framework" is unambiguous: the CACG Rust port is the *engine*,
and the migrated CFA corpus is the *first real production payload*. The goal is not "build
a framework"; it is "run the legacy corpus through the framework and have it come out
fine-grained, fast, and trustworthy."

### 1.2 Decomposition into required framework capabilities

For the goal to be reachable, the CACG framework (Rust port) must be able to do ALL of:

| # | Capability | Why the goal needs it |
|---|------------|------------------------|
| C1 | **Ingest CFA PDFs into hash-pinned chunks** (`kb ingest` → `sources_manifest.json` + `chunks_manifest.json`) | The legacy corpus hashes only PDF *bytes*; CACG's whole value proposition is chunk-level integrity. Without a Rust chunker, every migrated card cites chunks that were never extracted by CACG — they are synthetic placeholders. |
| C2 | **Author / scaffold cards** (`kb new`, `kb scaffold-matrix`, `kb scaffold-role-map`) | A real migration authors ~268 cards plus per-reading source-matrices and role-maps. Hand-writing those artifacts defeats the framework. |
| C3 | **Lint + verify cards** (Layer-1 mechanical lint, Layer-2 exact-substring containment) | The trust kernel. **DONE in Rust** (M0–M3). |
| C4 | **Retrieve cards** (`kb search`, `kb show`, FTS5 sidecar, BM25 ranking) | The "fast / high-recall" half of the goal. **Near-done in Rust** (M5, the currently-active loop). |
| C5 | **Verify semantic accuracy** (Layer-3: claim is *semantically* supported by the cited chunk, not merely a substring of it) | The "high-accuracy" half of the goal. The legacy linter checks page *bounds* but never checks that quoted text *appears in* — let alone is *supported by* — the cited span (`_research/01` "Friction observed", final bullet). |
| C6 | **Migrate the corpus** (a repeatable per-vertical adapter + the actual 11-vertical run) | The goal *is* the migration. Everything else is means. |
| C7 | **Index incrementally + retract** (`kb index` cache, `kb retract*`, `kb history`) | A 268-card corpus needs incremental re-index and source-/chunk-/card-level retraction (the CFA corpus already carries legacy retraction records and an EPUB blacklist). |

**Critical observation.** "High recall" decomposes into TWO sub-requirements that the
project has been conflating:

- **Retrieval recall** — `kb search` returns all relevant cards. M5 delivers this
  (BM25 + FTS5). Largely DONE.
- **Citation recall / grounding** — every card claim is *actually supported* by the
  cited PDF chunk, and no claim silently drifts from its source. This is **C1 + C5**, and
  it is the part of "high-accuracy/high-recall" that is *structurally impossible today*
  because there is no Rust PDF chunker (C1) and no Layer-3 semantic verifier (C5).

The user's phrase "high-accuracy/high-recall knowledge base" is therefore mostly about
**C1 + C5**, not C4. M5 (the active loop) has been serving C4. The next phase must serve
C1/C5/C6, or the goal cannot be reached.

---

## 2. Target-Corpus Characterization (`../CFA_reading`)

### 2.1 What actually exists — measured, not quoted from stale docs

`../CFA_reading` is internally inconsistent about its own card count. The README/CLAUDE.md
say **258**, STATUS.md's header says **268** (post-v18 Round 2), and CACG's own
`_research/01` says **219**. A direct `find` over `.claude/knowledge/<NN>_*/` (excluding
`_*` infra files, `INDEX.md`, `README.md`) on 2026-05-23 returns the ground truth:

```
01_quantitative_methods             17
02_economics                        29
03_financial_reporting_analysis      29
05_equity                            24
06_fixed_income_and_credit           36
07_derivatives_and_volatility        20
08_convertible_bonds                 44
09_portfolio_management_…            24
10_behavioral_finance                 5   ← v12 Batch-0 mid-state, NOT a closed vertical
11_risk_management                   24
17_cross_cutting                     16
-----------------------------------------
TOTAL                               268   across 11 directories
```

**Finding A — the corpus is bigger and messier than the project believes.** It is **268
cards**, not 258 and not 219. Ten verticals are "Codex-verified-complete" (01, 02, 03, 05,
06, 07, 08, 09, 11, 17 = 263 cards); `10_behavioral_finance` holds **5** cards from an
unfinished v12 Batch-0 and is *not* a closed vertical. Seven CFA topic areas (04, 12, 13,
14, 15, 16, and the not-yet-finished 10) are empty or partial placeholders. **Any
migration plan must pin "268 cards across 11 directories, 10 closed verticals + 1 partial"
as its canonical scope number** and not trust the in-repo prose. This count is also a
moving target — v18 was authoring cards as recently as 2026-05-21 — so the migration must
snapshot a git SHA.

### 2.2 Source PDFs — where they live, how large

- `Convertible_Bonds/` — 191 MB. English CB classics (`classic_books_CB_english/`:
  Calamos 2003, Philips 1997, Thorp+Kassouf 1967, Zubulake 1991,
  `DeSpiegeleer_Schoutens_VanHulle_2014_Handbook_of_Hybrid_Securities.pdf`) +
  `classic_pricing_books_english/` (Hull, Lando, Glasserman, Koziol) +
  `targeted_books_Chinese/`.
- `CFA_Program_Curriculum/` — 124 MB. `CFA_2022_Level_I_Volumes_1-6.pdf` (the combined
  6-volume, ~4353-page edition; CFA citations must qualify `Vol.<N>/pp.<P-Q>`).
- `20_Chinese_Convertible_Bonds_Research/` — 22 MB, 19 PDFs (regulatory + practitioner).
- `notes/` — **513 MB** — includes `CFA_note_2.ocr.pdf` (a GOOD-audited OCR-derivative
  PDF; the primary anchor for `01_quantitative_methods` and parts of `11`). The raw
  `notes/CFA_note_2.pdf` is matrix-registered `quotable: no` (SCAN).
- `deferred_books/` — hard-blocked from citation by CLAUDE.md Critical Rule 8 (DEFER-001/
  002 + manifest mirror). 37 SOTA-pass PDFs + the 2026-05-18 CB-pivot defers. **The
  migration must NOT ingest anything under `deferred_books/`.**

Total **70 non-deferred PDFs + 2 EPUBs** on disk. The source-eligibility matrix
(`_corpus_planning/05_source_matrix.md`) carries **~31 authoritative data rows** (the
file's ~80 pipe-table lines include narrative and per-subcorpus header tables; only ~31
are true source rows). EPUBs (Maitland 2022, 攻守) are entirely blacklisted — no page
anchors — and must be excluded from any CACG `source_matrix.json`.

### 2.3 The legacy card file format (and how it differs from `cacg.v0`)

A representative legacy card
(`.claude/knowledge/08_convertible_bonds/cb-parity-and-conversion-value.md`) has a
**line-based pseudo-YAML** frontmatter:

```
---
Use when: explaining `P(t) = q · S(t)` (parity, conversion value) …
Primary raw source: Convertible_Bonds/classic_books_CB_english/DeSpiegeleer_…_2014_…pdf pp.32-50
Supporting sources:
  - …/Calamos_2003_Convertible_Arbitrage.pdf pp.40-65
  - …/Hull_Options_Futures_and_Other_Derivatives.pdf pp.241-242
Repo touchpoints:
  - .claude/knowledge/08_convertible_bonds/cb-conversion-feature-mechanics.md
Out of scope: pricing models, free-boundary PDEs, vol surfaces
Version family: DeSpiegeleer 2014 1ed; Calamos 2003 1ed; Hull (recent ed.)
CFA Relevance: adjacent
Source Stance: primary-despiegeleer
deliverable-ready: true
---
```

The body is structured prose: mandatory `## Intuition` / `## Definition` /
`## Mathematical Reasoning` / `## See Also` / `## Escalate to Raw When`, with ASCII
diagrams and an inline `**Source:** <author> (<year>) §<section> pp.<N-M>` marker on every
paragraph.

The structural deltas vs `cacg.v0` (`docs/schema.md`) — this is the migration's actual
work surface:

| Dimension | Legacy `CFA_reading` | CACG `cacg.v0` |
|-----------|----------------------|-----------------|
| Frontmatter | Line-based pseudo-YAML (`Key: value`, indented list items) | Strict real YAML, `extra="forbid"`, `schema_version: "cacg.v0"` required |
| Card identity | None (rely on git for change detection) | `card_hash` = SHA-256 over canonical frontmatter + normalized body; stale → `CACG-HASH-002` |
| Citation | **String** `<pdf-path> pp.5-10` — page bounds only | **Structured** `{source_id, chunk_id, chunk_hash, page_range, quote, edge_type}` |
| Source integrity | SHA-256 of **raw PDF bytes only** | SHA-256 of **extracted+normalized chunk text + page metadata** (`chunk_hash`) |
| Verification depth | Linter checks page bounds, banned spans, audit rating — never checks the quote appears in the page | Layer-2 normalized exact-substring containment of `quote` inside the pinned chunk |
| Summary | No bounded summary field | `summary` REQUIRED, `[80, 400]` chars, drives `kb search` BM25 |
| Tags | No tags | `tags` lowercase-slug list, ≤10, participate in BM25 |
| Card-to-card edges | `Repo touchpoints:` (free-form path list) + `## See Also` prose | `card_edges: [{target, edge_type}]` with DAG validation `CACG-DEP-001..004` |
| Stance/relevance | `Source Stance:` / `CFA Relevance:` on the card; per-subcorpus `_source_role_map.md` (Markdown) | Optional `out/role_maps/<reading>.json` (strict Pydantic); closed vocab `{primary,supporting,deferred}` × `{core,adjacent,extension}` |
| Reading id | Topic directory name `08_convertible_bonds` | `reading_id` field, e.g. `reading_08` |
| Authorization | `subcorpora` column on each matrix row | `source_matrix.json` `allowed: {reading_id → [source_id]}` |
| Per-subcorpus rules | Thousands of `if subcorpus == "06"` branches in `lint_cards.py` / `_lint_fm006_dispatch.py` | All rules generic; matrix is data, not code |

### 2.4 How the legacy framework chunks / verifies

It does **not chunk** in any CACG sense. `tools/audit_pdf_quality.py` rates PDFs via
`pdfinfo`/`pdftotext` text density; `scripts/kb/build_manifest.py` builds an FTS5 + JSON
manifest keyed on whole cards and per-PDF SHA-256; `scripts/kb/lint_cards.py` (~1187 LOC,
41 diagnostic codes across 10 categories: FM/CITE/EPUB/NUM/SEC/DENS/SKILL/DEP/PRIM/STYLE/
TRANS) is the gate. Its retraction trigger fires on PDF-SHA mismatch / missing source /
audit-rating downgrade — i.e. it detects a *replaced PDF file* but is structurally blind
to *quote-vs-page-content drift*. The gate stack is Python-stdlib-only (no PyYAML, no
pandoc). This is exactly the trust gap CACG-v0 was designed to close.

### 2.5 What "refactor this corpus into CACG" requires — step by step

Concretely, a full migration of `../CFA_reading` into CACG is this pipeline:

1. **Snapshot** the read-only corpus at a fixed git SHA (268-card count is volatile).
2. **Ingest** each of the ~70 non-deferred quotable PDFs with `kb ingest` →
   per-source `sources_manifest.json` rows + a real `chunks_manifest.json` of
   hash-pinned chunks. *Requires C1 — a Rust Pdfium chunker. Does not exist.*
3. **Build `source_matrix.json`** from the legacy `_corpus_planning/05_source_matrix.md`:
   one `reading_NN` key per closed vertical, allow-listing its authorized `source_id`s.
   *Requires C2 — `kb scaffold-matrix`. Stub in Rust.*
4. **Re-author or transform 268 cards** into `cacg.v0`: parse the legacy pseudo-YAML,
   map `Primary raw source: <pdf> pp.N-M` → a structured `Citation` against a real
   ingested `chunk_id + chunk_hash`, synthesize the required `summary` (80–400 chars),
   derive `tags`, convert `Repo touchpoints` + `See Also` into `card_edges`. *Requires
   C2 + C6.* The hardest sub-step: **a legacy `pp.N-M` page range must be resolved to a
   concrete CACG chunk** — and CACG chunks span ≤ 2 pages, so one legacy multi-page
   citation fans out into several chunk citations, each needing an exact `quote`
   substring that genuinely appears in that chunk.
5. **`kb index`** → `cards_manifest.json` + `INDEX.md` + `summaries.json` +
   `summaries.sqlite`. *Rust `kb index` exists.*
6. **`kb lint` + `kb verify`** every card. Layer-2 will *fail* on every citation whose
   legacy `quote` text does not normalized-exact-substring-match the freshly extracted
   chunk — which will be common, because the legacy cards paraphrase ("No paraphrase of
   math beyond the source's own rigor" is a *prose* rule, not a verbatim-quote rule).
   This is where **C5 (Layer-3 semantic)** becomes load-bearing: without it, a faithful
   paraphrase that is genuinely supported by the source is indistinguishable from a
   fabrication, and the migration cannot pass its own gates.
7. **Retraction handling** — preserve the legacy EPUB blacklist + `deferred_books`
   hard-block as `source_matrix` exclusions and/or `retracted_source_ids`.
8. **Retrieval-eval** the migrated corpus for recall regressions.

Steps 2 and 6 are the binding ones. Step 2 needs C1 (no Rust chunker). Step 6 needs C5
(no Layer-3) to avoid a wall of false-positive verify failures on legitimate paraphrase.

---

## 3. Gap Analysis — CAN vs CANNOT, and the Binding Constraint

### 3.1 Capability-by-capability status of the CACG Rust port

Port state (high-level; the parallel agent owns the fine-grained M5 audit):

- **DONE:** M0/M1 trust kernel (schema, hashing, canonical JSON, normalize, determinism,
  frontmatter, atomic publish, journal); M2 parity infrastructure; M3 full `kb` CLI
  surface + verify hot path (Layer-1 lint, Layer-2 exact-substring + bounded Levenshtein
  + Ratcliff-Obershelp, `verify_one_card`, `kb verify --round-summary`, `kb index`);
  M3 CFA *first-bite* (the `05_Equity` vertical, 4 cards, migrated by a one-shot Python
  tool into `tests/parity_corpus/cfa_first_bite/`). M5 retrieval foundation — in-house
  BM25 in `cacg-core/src/bm25.rs`, BM25 verify hints in `cacg-core/src/verify/bm25_hints.rs`,
  `cacg-search` in-memory search, SQLite FTS5 sidecar, `kb search`, `kb show`,
  retrieval-eval gate — is the currently-active RLCR loop, near completion.
- **NOT DONE:** M4 Pdfium ingest (`crates/cacg-ingest/src/lib.rs` is a 1-function
  placeholder; `pdfium-render` is optional behind a never-enabled `ingest` feature);
  the M3 authoring-tail verbs (`kb new`, `history`, `retract`, `retract-source`,
  `retract-chunk`, `scaffold-matrix`, `scaffold-role-map`, `migrate-summaries` all
  dispatch to an unimplemented-subcommand stub in Rust); Layer-3 semantic verification
  (`crates/cacg-semantic/src/lib.rs` is a placeholder; `docs/semantic-verifier.md`
  designs B1 cache-as-oracle + B2 LLM-judge); and the actual full migration of
  `../CFA_reading`.

| Cap | What the port CAN do today | What it CANNOT do | Verdict |
|-----|----------------------------|--------------------|---------|
| **C1 Ingest** | Nothing in Rust. `cacg-ingest` is a placeholder. Python `kb ingest` exists. | Extract text from a CFA PDF into hash-pinned `cacg.v0` chunks via the Rust binary. | **BLOCKED** |
| **C2 Author/scaffold** | Nothing in Rust — `kb new` / `kb scaffold-matrix` / `kb scaffold-role-map` are stubs. Python exists. | Scaffold a `source_matrix.json` or role-map for a migrated vertical via Rust. | **BLOCKED (Rust); workaroundable via Python** |
| **C3 Lint+verify** | Full Layer-1 + Layer-2 in Rust, byte-equal-gated, perf-gated (p50 ≤ 50 µs/card). | — | **DONE** |
| **C4 Retrieve** | `kb search` + `kb show` + FTS5 sidecar + BM25 ranking land with M5. | — | **DONE (M5 closing)** |
| **C5 Semantic accuracy** | Nothing. `cacg-semantic` is a placeholder. *Python has no Layer-3 either* — `docs/semantic-verifier.md` is a design, not an implementation. | Decide whether a paraphrased card claim is genuinely supported by its cited chunk. | **BLOCKED (both langs)** |
| **C6 Migration** | M3 first-bite proved the *recipe* on 4 `05_Equity` cards — but via a one-shot Python script (`migrate_cfa_vertical.py` was specified; `build_cfa_first_bite_corpus.py` is the committed builder), with **synthetic placeholder chunks** (legacy citations whose chunk text was unavailable were synthesized from the card's `quote` field under a placeholder `source_sha256`, annotated `cacg.v0/scope:synthetic-chunk`). | Migrate the full 268-card corpus against *real* PDF-extracted chunks. The first-bite chunks are synthetic — they do not test C1 and do not give real grounding. | **PARTIAL (4/268, synthetic chunks)** |
| **C7 Index/retract** | `kb index` (+ incremental cache) DONE. `kb retract*` / `kb history` are Rust stubs; Python exists. | Retract a source/chunk/card via the Rust binary. | **PARTIAL** |

### 3.2 The binding constraint

> **The binding constraint is C1: there is no Rust PDF-to-hash-pinned-chunk ingest.**

Reasoning. Walk the dependency chain of the ultimate goal backward:

- The goal is a *migrated, trustworthy* CFA corpus (C6).
- C6's trust value is **chunk-level integrity** — that is the entire reason CACG-v0
  exists instead of the legacy framework (`_research/07` §2: "No chunk-level integrity:
  CFA_reading hashes ONLY the source PDF bytes … CACG-v0 closes this by hashing extracted
  normalized chunk text").
- Chunk-level integrity requires **real chunks extracted from the real PDFs** — i.e. C1.
- The M3 first-bite explicitly side-stepped C1 by *synthesizing* chunks from the card's
  own `quote` field. A synthetic chunk is a tautology: it verifies because the quote was
  copied into the chunk. It proves the migration *plumbing*, not the *grounding*. A full
  migration built on synthetic chunks would be a 268-card corpus whose `chunk_hash`
  citations certify nothing — exactly the legacy framework's weakness, re-skinned in
  `cacg.v0` JSON.

Therefore: **until `kb ingest` extracts real chunks from the CFA PDFs, the ultimate goal
is structurally unreachable** — not "slower," but *impossible to do correctly*. Every
other missing capability is either already-workaroundable (C2/C7 via the existing Python
binary) or a quality *enhancement* on top of a corpus that must first exist with real
chunks (C5 sharpens accuracy, but you cannot semantically-verify a claim against a chunk
you never extracted).

C5 (Layer-3 semantic) is the *second* binding constraint and the direct server of the
"high-accuracy" adjective — but it is strictly *downstream* of C1: Layer-3 verifies a
claim against a chunk, and there are no real chunks without C1.

**Secondary (non-binding) gaps.** The M3 authoring-tail verbs (C2/C7) are missing from
the *Rust* binary but the *Python* binary implements them all — a migration can call
Python `kb scaffold-matrix` / `kb retract*` while the Rust binary owns the hot path. They
are real work but they do not *block* the goal; they are a Rust/Python parity debt.

---

## 4. Next-Phase Candidate Options

Each option below gives: scope, dependencies, what it unblocks, risk, rough effort
(RLCR-round estimate, calibrated against the project's history — M3 verify-hot-path was a
~28-task plan; M5 is ~20–30 rounds; the legacy 08 vertical took 23 rounds), and how
directly it serves the ultimate goal — especially "high accuracy / high recall."

### Option A — M4: Pdfium Ingest + Rust Chunker

**Scope.** Implement `cacg-ingest` end-to-end: `pdfium-render` per-page text extraction,
the page-window chunker (≤ `max_pages_per_chunk` = 2 distinct pages per chunk), the
`chunk_hash` envelope (SHA-256 over normalized text + `start_page` + `end_page` +
`page_spans`), `sources_manifest.json` + `chunks_manifest.json` pair-atomic publish, and
the `kb ingest` CLI verb. The hard sub-deliverable is the **byte-equal Pdfium parity
gate**: `pdfium-render`'s `PdfPage::text()` must produce extracted text byte-identical to
Python `pypdfium2 5.8.0`'s `get_text_range()` so that an existing Python-built
`chunks_manifest.json` re-verifies under the Rust binary (`_research/07` §3.1 / §4.1
invariant 1; `_research/09` DEC-1 BYTE-EQUAL-vs-HASH-STABLE is still open).

**Dependencies.** `pdfium-render = "0.9.1"` (already pinned) + a pinned Pdfium binary
(chromium-builds release, exact build SHA logged). Isolated to `cacg-ingest`;
`#![forbid(unsafe_code)]` is intentionally omitted there (the only unsafe FFI surface).

**What it unblocks.** **C1 — the binding constraint.** This is the single deliverable
whose absence makes the goal impossible. Once `kb ingest` produces real chunks, a CFA
migration can pin citations to genuine PDF-extracted content.

**Risk.** HIGH. (a) Byte-identical FFI text extraction across `pypdfium2` vs
`pdfium-render` is *not* automatic — terminator nulls, internal whitespace, hyphenation
marks, Unicode replacement chars can all differ; if they diverge, the project must invoke
the `_research/09` DEC-1 HASH-STABLE fallback (a documented chunk-hash regeneration
ceremony) — survivable but it must be decided. (b) Pdfium is C++ FFI: untrusted-PDF DoS /
OOM / panic-across-FFI; per-page extraction must run inside `std::panic::catch_unwind`.
(c) Pdfium binary versioning is a permanent determinism liability (every bump = a
chunk-hash regeneration event). The CFA PDFs are large (Vol.1-6 combined ~124 MB / 4353
pages; the OCR PDF tests the OCR-extraction path) — DEC-5 already trimmed the parity
fixture to `cfa_vol1_trim.pdf` (pages 1-30, 426 KB) to keep the repo lean.

**Rough effort.** ~22–30 RLCR rounds. The chunker + envelope + publish is mechanical and
reuses `cacg-core` primitives; the Pdfium parity gate is the long pole and may itself
consume 8–12 rounds of golden-byte iteration.

**Serves the goal.** DIRECTLY and uniquely — it is the prerequisite for a *real* (not
synthetic-chunk) migration. It does not by itself improve retrieval recall or semantic
accuracy; it makes both *possible* by producing the chunks they operate on.

### Option B — M3 Authoring-Tail Verbs

**Scope.** Port the seven stubbed verbs to Rust: `kb new` (card scaffold from template),
`kb history` (per-card `history.jsonl` print/validate), `kb retract` / `retract-source` /
`retract-chunk` (tombstone events + manifest disjointness maintenance), `kb
scaffold-matrix`, `kb scaffold-role-map`, `kb migrate-summaries`. Plus the auxiliary lint
families the M3 hot-path carve-out deferred (`CACG-SUM`, `CACG-SKILL`, `CACG-DEP`,
`CACG-ROLE` — `validate_skill_routers`, `validate_role_maps`, `validate_card_dag`), which
closes the §3a `docs/diagnostic-parity.md` carve-out.

**Dependencies.** None new; all consume existing `cacg-core` types.

**What it unblocks.** C2 + C7 on the *Rust* side — closes the Rust/Python parity debt so
the Rust binary is a complete `kb` and Python can be deprecated (the M7 ceremony).

**Risk.** LOW. These are deterministic, well-specified, no native deps, and Python
reference implementations exist for byte-equal parity testing. The only subtlety is
`migrate-summaries`' heuristic mode.

**Rough effort.** ~18–24 RLCR rounds (7 verbs + 4 lint families, each with parity
fixtures).

**Serves the goal.** INDIRECTLY. Every one of these verbs is *already usable today via
the Python binary*. Porting them to Rust improves nothing the user can observe about the
migrated corpus — it is a parity/cleanup phase. It does NOT serve "high accuracy / high
recall" at all. Doing it before C1 spends ~20 rounds without moving the binding
constraint.

### Option C — Layer-3 Semantic Verification (B1 cache-oracle + B2 LLM-judge)

**Scope.** Implement `cacg-semantic`: B1 — a strict O(1) dict lookup against a frozen
`out/semantic_cache.json` keyed by `(chunk_hash, claim_window_hash)`, built offline by a
pinned `sentence-transformers/all-MiniLM-L6-v2` embedding model; B2 — an opt-in,
CI-only, async LLM-judge via Claude Haiku (`reqwest` + `tokio`, feature-gated). Layer-3
fires only when Layer-2 exact-substring + fuzzy both fail and the operator passes
`--semantic <cache>` or `--semantic-judge`; the verdict (`CACG-VERIFY-002`) rides inside
the existing per-card journal event.

**Dependencies.** B2: `reqwest` + `tokio` (workspace-pinned). B1 *runtime* needs nothing
(strict dict lookup). B1's **cache-builder** needs an embedding stack — and this is a real
open problem: there is no mature pure-Rust `sentence-transformers` equivalent (see §6).
`docs/semantic-verifier.md` parks the builder as "Phase 3.1, out of scope" and assumes
operators check out a pre-built cache; a Rust-native builder would pull in
`candle`/`ort`/`fastembed`.

**What it unblocks.** C5 — the "high-accuracy" adjective. With Layer-3, a faithful CFA
paraphrase that is genuinely supported by its source can pass verification with a
documented semantic verdict, instead of being a hard Layer-2 failure.

**Risk.** MEDIUM–HIGH. (a) B2 is non-deterministic by design — it can never gate merges
and is CI-review-only. (b) B1's determinism depends on a frozen cache; building that
cache reproducibly requires pinning a model + platform, and the Rust embedding-crate
landscape is immature. (c) Layer-3 is *useless without real chunks* — semantically
verifying a claim against a *synthetic* chunk (which contains a copy of the claim) is
circular. So C is strictly downstream of A.

**Rough effort.** ~16–22 rounds for B1 runtime + B2 dispatcher; **+8–14 more** if a
Rust-native cache-builder is in scope rather than deferred.

**Serves the goal.** DIRECTLY serves "high accuracy" — but only *after* C1 exists. Run
before A, it has nothing real to verify.

### Option D — A Real `../CFA_reading` Migration Pilot (one vertical, end-to-end)

**Scope.** Take ONE closed vertical end-to-end through the *real* pipeline: `kb ingest`
its PDFs into real chunks → `kb scaffold-matrix` → transform its cards to `cacg.v0` with
citations pinned to real `chunk_id + chunk_hash` → `kb index` → `kb lint` + `kb verify`
→ retrieval-eval. Unlike the M3 first-bite (4 `05_Equity` cards, synthetic chunks), this
pilot uses *genuine* extracted chunks and reports the real Layer-2 pass rate. Natural
candidate: `01_quantitative_methods` (17 cards, single dominant source
`notes/CFA_note_2.ocr.pdf`) — small blast radius, one PDF, and it stress-tests the
OCR-extraction path.

**Dependencies.** **Hard-depends on Option A (C1).** Cannot produce real chunks without
`kb ingest`. Benefits from C2 (scaffold) and is *informed* by C5 (the pilot will measure
how badly Layer-2 fails on paraphrase and thus quantify how much C5 is needed).

**What it unblocks.** Converts the migration from "recipe proven on synthetic data" to
"recipe proven on real data," and produces the empirical Layer-2-failure-rate number that
tells the project how urgently it needs Layer-3.

**Risk.** MEDIUM. The dominant risk is discovering that the legacy cards' paraphrased
prose fails Layer-2 exact-substring at a high rate — but that is the *point* of the
pilot: surface the number early on 17 cards rather than late on 268.

**Rough effort.** ~10–16 rounds *given A is done* (otherwise A's ~25 rounds are a
precondition). Largely a transform-tool + fixtures effort, not new engine code.

**Serves the goal.** DIRECTLY — it is a literal slice of the goal. But it cannot be a
standalone next phase: without A it degenerates back into the synthetic-chunk first-bite.

### Option E — Sequenced Hybrid: M4 Ingest, then a Real One-Vertical Pilot

**Scope.** A single milestone with two sequenced sub-phases and a mandatory checkpoint
between them (the project's established cadence — M5 itself shipped as M5a/M5b with a
checkpoint, and the user explicitly prefers tight review splits):

- **Sub-phase α (M4-core):** `cacg-ingest` — `pdfium-render` extraction, the page-window
  chunker, the `chunk_hash` envelope, `sources_manifest` + `chunks_manifest` pair-atomic
  publish, the `kb ingest` verb, and the **Pdfium byte-equal parity gate** against
  `pypdfium2 5.8.0` (with the DEC-1 HASH-STABLE fallback decision made explicit). The
  checkpoint gate: `kb ingest cfa_vol1_trim.pdf` produces a `chunks_manifest.json` whose
  hashes the Rust *and* Python binaries both verify.
- **Sub-phase β (real first-vertical migration):** a deterministic, read-only
  transform tool that ingests `01_quantitative_methods`'s PDF(s) for real, builds a real
  `source_matrix.json`, transforms its 17 cards to `cacg.v0` with citations pinned to
  *genuine* extracted chunks, runs `kb index` + `kb lint` + `kb verify`, and reports the
  **real Layer-2 pass/fail rate** as a committed findings artifact
  (`_research/18_cfa_real_migration_findings.md`).

**Dependencies.** α: `pdfium-render` (pinned). β: depends on α; read-only against
`../CFA_reading`.

**What it unblocks.** C1 (binding constraint) AND the first *real* slice of C6 — and it
produces the empirical signal (β's Layer-2 failure rate) that lets the project decide,
with data, whether Layer-3 (Option C) is the immediately-following phase or can wait.

**Risk.** Inherits Option A's HIGH Pdfium-parity risk, contained behind the α/β
checkpoint: if α's parity gate cannot reach byte-equal, the milestone *stops at the
checkpoint* with the HASH-STABLE decision documented, rather than dragging an unproven
ingest into a 268-card migration. β is bounded (17 cards, one PDF).

**Rough effort.** ~30–40 RLCR rounds total (α ≈ 22–28, β ≈ 10–14), split across the two
sub-phases — consistent with the project's milestone-sized loops.

**Serves the goal.** Most directly of all the options: it removes the binding constraint
*and* validates removal against a real corpus slice in one milestone, and it deliberately
produces the data needed to scope the *next* milestone (C5). It is the rigorous,
defensively-sequenced option.

### 4.1 Option scorecard

| Option | Unblocks | Serves "accuracy/recall" | Risk | Effort (rounds) | Standalone-viable? |
|--------|----------|---------------------------|------|-----------------|--------------------|
| A — M4 ingest | C1 (binding) | enables it | HIGH | 22–30 | Yes |
| B — authoring tail | C2/C7 (Rust parity) | no | LOW | 18–24 | Yes |
| C — Layer-3 semantic | C5 (accuracy) | yes, but needs A first | MED–HIGH | 16–22 (+8–14) | No (needs real chunks) |
| D — real 1-vertical pilot | C6 slice | yes | MED | 10–16 (+A) | No (needs A) |
| E — M4 + real pilot (hybrid) | C1 **and** C6 slice | yes | HIGH (checkpointed) | 30–40 | Yes |

---

## 5. Recommendation

### 5.1 Recommended next phase — **M4: Pdfium Ingest + Real First-Vertical Migration** (Option E)

Adopt **Option E** as the next phase, labelled **milestone M4**, scoped as two sequenced
sub-phases (**M4a** ingest core, **M4b** real first-vertical migration) with a mandatory
checkpoint between them.

**Justification against the ultimate goal.** The goal is a *migrated, trustworthy* CFA
corpus. §3.2 establishes that the **binding constraint is C1 (Rust PDF ingest)** — without
real extracted chunks, a migration certifies nothing and the goal is not merely delayed
but *unreachable correctly*. Option E removes C1 (sub-phase M4a) and immediately proves
the removal against a real corpus slice (sub-phase M4b), which is the most direct possible
service of the goal. Crucially, M4b's empirical output — the real Layer-2 exact-substring
pass/fail rate on 17 genuinely-extracted cards — is the *data the project currently
lacks* to scope Layer-3 (Option C): if paraphrase makes Layer-2 fail on, say, 60% of
citations, Layer-3 is the urgent next milestone; if it fails on 10%, fuzzy + targeted
re-authoring may suffice. Choosing E now buys both the binding-constraint removal and the
information to sequence what follows.

This also matches the user's documented preference profile (auto-memory: "prefers the
rigorous / defensive / review-tight option," "likes tight review cadence with explicit
sub-phase splits"): the α/β checkpoint is a hard gate that refuses to drag an unproven
Pdfium parity result into a 268-card migration, and the milestone is split exactly the way
M5 (M5a/M5b) was.

**Why not the alternatives as the next phase.** Option B (authoring tail) is low-risk but
serves the goal *not at all* — every verb is already usable via the Python binary; doing
it now spends ~20 rounds without touching the binding constraint. Option C (Layer-3) is
the right *eventual* accuracy phase but is circular before C1 (you cannot semantically
verify against synthetic chunks). Option D is a literal goal-slice but degenerates into
the synthetic-chunk first-bite without A. Only A and E remove the binding constraint, and
E dominates A by adding the real-corpus validation and the Layer-3-scoping data for a
bounded extra ~10–14 rounds.

### 5.2 Runner-up — **M4-core only** (Option A)

If the reviewer judges Option E too large for one milestone, fall back to **Option A
alone** as M4: ship `cacg-ingest` + the Pdfium parity gate, and defer the real
first-vertical migration to a separate M5-successor milestone. This still removes the
binding constraint; it merely splits E's β sub-phase into its own loop. The cost is that
the project does not get the real-corpus Layer-2-failure signal until a milestone later,
which weakens the ability to scope Layer-3. Recommended only if round-budget pressure is
acute. **Do not** make the runner-up "Option B" or "Option C" — both leave the binding
constraint in place.

### 5.3 Scoped, ordered deliverable list for M4 (draft-document seed)

This is concrete enough to become a `gen-idea`/`gen-plan` draft. Sub-phase tags follow the
M5a/M5b precedent; AC-7 is cross-cutting.

**Sub-phase M4a — Pdfium ingest core**

- **D1. `cacg-ingest` PDF text extractor.** `pdfium-render` per-page text extraction
  behind the existing `ingest` feature; each per-page extraction wrapped in
  `std::panic::catch_unwind`; a malformed-PDF failure surfaces as `CACG-INGEST-001`, never
  a panic across FFI. *AC: positive — a fixture PDF extracts to expected per-page text;
  negative — a truncated/garbage PDF yields a diagnostic, not a crash.*
- **D2. Page-window chunker.** Port the chunker: monotonic `ordinal`, `chunk_id =
  <source_id>:p<NNN>:<ordinal>`, `start_page`/`end_page`/`page_spans`, ≤
  `max_pages_per_chunk` (2) distinct pages per chunk, `token_count`, `text_preview`.
  *AC: chunk boundaries + ids byte-equal with the Python chunker on the sample PDF.*
- **D3. `chunk_hash` envelope.** SHA-256 over the canonical hash envelope binding
  `(normalize_text(text), start_page, end_page, page_spans)`, reusing `cacg-core`'s
  `normalize` + `canonical_json` + `hash`. *AC: `chunk_hash` byte-equal with Python for
  identical envelope inputs.*
- **D4. `sources_manifest.json` + `chunks_manifest.json` pair-atomic publish.** Reuse
  `cacg-core::atomic_publish`; `source_sha256` over raw PDF bytes; honor the
  `retracted_source_ids` / `retracted_chunk_ids` disjointness invariants. *AC: pair-atomic
  publish + rollback under failure injection; no `.tmp`/`.bak` debris.*
- **D5. `kb ingest` CLI verb.** Wire `kb ingest <pdf>` through `cacg-cli` → `cacg-ingest`;
  remove `ingest` from the unimplemented-subcommand stub; PDF-isolation contract holds
  (`cargo xtask audit-cacg-core-deps` green; no Pdfium edge into `cacg-core` or any
  common-path verb). *AC: `cargo tree` shows zero Pdfium edges outside `cacg-ingest`.*
- **D6 (CHECKPOINT). Pdfium byte-equal parity gate.** `kb ingest cfa_vol1_trim.pdf`
  (the DEC-5 fixture) produces a `chunks_manifest.json` whose `chunk_hash`es BOTH the Rust
  and Python binaries verify. **Resolve `_research/09` DEC-1 here:** if byte-equal holds,
  declare BYTE-EQUAL; if `pdfium-render` text output diverges from `pypdfium2 5.8.0`,
  invoke HASH-STABLE — author the chunk-hash regeneration ceremony in a new
  `_research/19_pdfium_parity_report.md` and pin the exact Pdfium build SHA. *This
  checkpoint gates entry into M4b.*

**Sub-phase M4b — real first-vertical migration (`01_quantitative_methods`)**

- **D7. Read-only real-ingest of the QM vertical's source PDF(s).** Run `kb ingest` for
  real on `notes/CFA_note_2.ocr.pdf` (the QM primary anchor); the tool is strictly
  read-only against `../CFA_reading/` (assert via before/after recursive SHA-256 +
  `(size, mtime_ns, dev, inode)` tuples, per the M3 read-only meta-test discipline).
- **D8. Real `source_matrix.json` for `reading_01`.** Derive from
  `_corpus_planning/05_source_matrix.md`; exclude EPUBs and any `deferred_books/*` path.
- **D9. Legacy→`cacg.v0` card transform for the 17 QM cards.** Parse the line-based
  pseudo-YAML; map each legacy `Primary raw source: <pdf> pp.N-M` to one or more
  structured `Citation`s against *real* `chunk_id + chunk_hash` (a multi-page legacy span
  fans out across ≤2-page chunks); synthesize the required `summary` (80–400 chars) and
  `tags`; convert `Repo touchpoints` + `See Also` to `card_edges`; preserve the card body
  verbatim. Deterministic under `KB_FROZEN_CLOCK=1`.
- **D10. `kb index` + `kb lint` + `kb verify` the migrated QM vertical** with the real
  chunks; capture the real Layer-2 exact-substring pass/fail rate.
- **D11. Findings artifact `_research/18_cfa_real_migration_findings.md`.** Document: the
  real Layer-2 pass rate; how many citations need fuzzy; how many would need Layer-3
  semantic; chunk-fan-out statistics for multi-page legacy spans; the chunk-id mapping
  recipe; OCR-extraction-path observations. **This artifact is the explicit scoping input
  for the post-M4 Layer-3 decision.**
- **D12. Retrieval-eval over the migrated QM vertical** — a recall-at-k smoke gate
  reusing the M5 `xtask retrieval_eval` harness, asserting expected hits survive top-k.

**Cross-cutting**

- **D-X. AC-7 — crate-boundary + determinism invariants**, checked at the close of *both*
  sub-phases: `#![forbid(unsafe_code)]` everywhere except `cacg-ingest`; PDF-isolation
  audit green; `KB_FROZEN_CLOCK=1` byte-determinism on all emitted artifacts; the
  Pdfium binary build SHA logged and pinned.

**Explicit non-goals for M4.** Migrating all 11 verticals (M4b does exactly one); Layer-3
semantic verification (post-M4, scoped by D11's findings); the M3 authoring-tail Rust
verbs (separate parity-cleanup milestone — M4b may shell out to the *Python* `kb
scaffold-matrix` if a Rust scaffold is not yet available, documented as a carry-forward);
Python deprecation (M7).

---

## 6. Crate / Dependency Implications

Every version below was cross-verified against the live `crates.io` JSON API on
2026-05-23 (`curl -s https://crates.io/api/v1/crates/<name>` — `max_stable_version` +
latest-release date + `recent_downloads`).

### 6.1 Crates the recommended phase (M4) needs

| Crate | Pin to use | crates.io `max_stable` (2026-05-23) | Latest release | Status | Notes |
|-------|-----------|--------------------------------------|----------------|--------|-------|
| `pdfium-render` | **`0.9.1`** (already in `[workspace.dependencies]`) | `0.9.1` | 2026-05-02 | **Current, healthy** | 331,522 recent downloads; updated 3 weeks ago. The pin is exactly right; no bump needed. Isolated to `cacg-ingest` behind the `ingest` feature. |
| `sha2` | `0.10.9` (already pinned) | `0.11.0` exists | — | Healthy | Keep `0.10.9` for stability per `_research/07` §3.5; bump to 0.11 only after benchmarking. No M4-specific change. |
| `serde` / `serde_json` | `1.0.228` / `1.0.149` (already pinned) | current | — | Healthy | Reused for the `chunk_hash` canonical-JSON envelope. |
| `tempfile` + `rustix` | `3.27.0` / `1.1.4` (already pinned) | current | — | Healthy | Reused for `sources`/`chunks` pair-atomic publish. |
| `insta` | `1.47.2` (already pinned, dev) | `1.47.2` | 2026-03-30 | Healthy | Snapshot tests for the Pdfium parity gate. |
| `iai-callgrind` + `criterion` | `0.16.1` / `=0.5.1` (already pinned, dev) | `0.16.1` / `0.8.2` | 2025-07-30 / 2026-02-04 | See note | `iai-callgrind` is healthy. **`criterion` is pinned `=0.5.1` while crates.io `max_stable` is `0.8.2`** — a deliberate exact-pin (the workspace also pins it for M6). Not stale-by-neglect; flagged only so the M6 closure consciously decides whether to advance it. M4 needs no change. |

**The non-trivial M4 dependency is not a Rust crate — it is the Pdfium *binary*.**
`pdfium-render` links a Pdfium shared library; the project must pin an exact Pdfium build
(chromium-builds release, SHA logged in `_research/19`). This is a versioning liability,
not a crate: every Pdfium bump is a chunk-hash regeneration event. M4's D6 checkpoint must
fix the build SHA.

**Verdict for M4: no new crate needs to be *added*.** Every crate M4 requires
(`pdfium-render`, `sha2`, `serde`, `tempfile`, `rustix`, `insta`, `iai-callgrind`) is
already in `[workspace.dependencies]` with a current, healthy pin. M4 is a
*feature-enable* (`ingest`) + *implementation* phase, not a dependency-expansion phase.

### 6.2 Crates the runner-up (Option A alone) needs

Identical to §6.1 — Option A is M4a, a strict subset of M4. No new crates.

### 6.3 Forward-looking flags for the *post-M4* phases (Layer-3, Option C)

These are NOT needed for M4 but are flagged now because Option C (the likely post-M4
phase) hits a real dependency problem the plan-generator should see early:

- `reqwest` — workspace pins `"0.12"`; crates.io `max_stable` is now **`0.13.3`**
  (2026-04-27). The `"0.12"` pin is **one minor behind** and will resolve to the latest
  0.12.x, not 0.13. Not broken, but Layer-3 B2 should consciously decide whether to
  advance to `0.13`. *Flag for the C-phase plan, not M4.*
- `tokio` — workspace pins `"1"`; resolves to `1.52.3` (2026-05-08). Healthy; the
  open-ended `"1"` is fine.
- **B1 semantic-cache builder — no good pure-Rust option.** `docs/semantic-verifier.md`
  needs `sentence-transformers/all-MiniLM-L6-v2` embeddings to *build* the B1 cache.
  crates.io findings: there is **no `sentence-transformers-rs` crate** (does not exist);
  `rust-bert = "0.23.0"` is **STALE — last release 2024-09-29, ~20 months old, only
  16,226 recent downloads — flag as unmaintained, do not adopt**; `candle-core = "0.10.2"`
  (2026-04-01, healthy) and `fastembed = "5.13.4"` (2026-04-27, healthy, 725k downloads)
  and `ort` (`2.0.0-rc.12`, ONNX Runtime, no stable release) are the live options. The
  B1 design *already* sidesteps this by keeping the builder offline ("Phase 3.1, out of
  scope") and shipping a pre-built cache. **Recommendation for the post-M4 Layer-3 plan:
  keep the B1 cache-builder offline / out-of-band (a Python `sentence-transformers`
  script is the path of least resistance), so the Rust workspace never takes an embedding
  dependency.** Surfacing this now prevents the C-phase from discovering it late.
- `pdf-extract` (`0.10.0`) / `lopdf` (`0.40.0`) — both current; relevant only as the
  `_research/07` §3.1 documented *alternative* to `pdfium-render` (pure-Rust extraction).
  M4 does not use them; noted for completeness.

---

## 7. Risk Register — for the recommended M4 phase

| ID | Risk | Likelihood | Impact | Mitigation |
|----|------|-----------|--------|------------|
| R1 | **`pdfium-render` text output is not byte-identical to `pypdfium2 5.8.0`** — terminator nulls, internal whitespace, hyphenation, replacement chars differ; existing Python `chunks_manifest.json` fails to re-verify. | MED–HIGH | HIGH | This is *the* M4a risk. D6 is a hard CHECKPOINT before M4b. If divergence is found, invoke `_research/09` DEC-1 HASH-STABLE: author `_research/19_pdfium_parity_report.md`, pin the Pdfium build SHA, and define the chunk-hash regeneration ceremony. The α/β split exists specifically to contain this. |
| R2 | **Pdfium native crash / OOM / panic-across-FFI on a malformed CFA PDF** takes down `kb ingest`. | MED | MED | Per-page extraction inside `std::panic::catch_unwind`; report `CACG-INGEST-001`; consider `-C panic=abort` only in the ingest binary. `cacg-ingest` is the sole crate without `#![forbid(unsafe_code)]` — keep the unsafe surface minimal and reviewed. |
| R3 | **Pdfium binary version drift** — a future Pdfium bump silently changes extracted text and therefore every `chunk_hash`. | MED (over time) | HIGH | D6 pins an exact Pdfium build SHA in `_research/19`; CI logs it; treat every bump as a deliberate chunk-hash regeneration event (same discipline as the current `pypdfium2==5.8.0` pin). |
| R4 | **M4b reveals legacy CFA paraphrase fails Layer-2 exact-substring at a high rate** — the migrated QM cards do not verify. | HIGH | MED | This is an *expected finding*, not a failure — it is the data D11 is designed to capture. M4b's success criterion is "the pipeline runs and the rate is measured," NOT "100% Layer-2 pass." The rate scopes the post-M4 Layer-3 phase. Document it; do not let it block M4 closure. |
| R5 | **PDF-isolation contract regression** — a Pdfium edge leaks into `cacg-core` or a common-path verb. | LOW | MED | `cargo xtask audit-cacg-core-deps` + `cargo tree -e features` snapshot in AC-7, checked at both sub-phase boundaries; `ingest` feature default-off. |
| R6 | **Repo bloat from committed CFA PDF fixtures** — large PDFs inflate every CI clone over a multi-month horizon. | MED | LOW | Reuse the DEC-5 `cfa_vol1_trim.pdf` (426 KB, 30 pages) for the parity gate; for M4b use `notes/CFA_note_2.ocr.pdf` *read in place* from the sibling repo (read-only, never copied into the CACG repo). Commit only manifests/artifacts, not source PDFs. |
| R7 | **Chunk fan-out explosion** — a legacy multi-page `pp.N-M` citation spanning many pages fans out into many ≤2-page CACG chunk citations, bloating cards and complicating the transform. | MED | LOW–MED | D9 must handle 1→N citation fan-out explicitly; D11 reports fan-out statistics; if pathological, revisit `max_pages_per_chunk` as a documented DEC. |
| R8 | **`../CFA_reading` mutates under the migration** — its card count moved as recently as 2026-05-21 (v18). | MED | MED | M4b pins a specific `../CFA_reading` git SHA as its input snapshot; the read-only meta-test (before/after recursive SHA-256 + inode tuples) guarantees the migration never writes to the sibling. |
| R9 | **Scope creep — M4 drifts toward migrating all 11 verticals.** | MED | MED | The plan must make "M4b migrates exactly `01_quantitative_methods`" an IMMUTABLE acceptance criterion; the other 10 verticals are an explicit post-M4 non-goal. |

---

## 8. Open Decisions — surface to the user / plan-generator before implementation

- **DEC-A — Pdfium parity policy (BYTE-EQUAL vs HASH-STABLE).** `_research/09` DEC-1 is
  still PROPOSED-DEFAULT, not resolved. M4a's D6 checkpoint *forces* the decision: if
  `pdfium-render` matches `pypdfium2 5.8.0` byte-for-byte, declare BYTE-EQUAL; otherwise
  HASH-STABLE with a documented regeneration ceremony. **The plan should pre-commit to
  "D6 empirically decides DEC-1" so the milestone has a defined branch at the
  checkpoint.** Recommendation: BYTE-EQUAL if achievable; HASH-STABLE is an acceptable,
  pre-blessed fallback — do not let an unattainable byte-equal target stall the milestone.

- **DEC-B — which vertical does M4b migrate?** This report recommends
  `01_quantitative_methods` (17 cards, single dominant source `CFA_note_2.ocr.pdf`,
  exercises the OCR path). The M3 first-bite used `05_Equity` (24 cards). Alternatives:
  `17_cross_cutting` (16 cards, single CFA-Vol.6 anchor — but the combined-PDF needs
  `Vol.N/pp` disambiguation, more complex). **User/plan-generator should confirm
  `01` vs re-using `05`.** Re-using `05` would let M4b directly compare real vs the M3
  synthetic-chunk first-bite — a real point in `05`'s favor.

- **DEC-C — does M4b ship a Rust `kb scaffold-matrix`, or shell out to Python?** M4b needs
  a `source_matrix.json`. The Rust verb is a stub (Option B territory). Cheapest path:
  M4b's transform tool shells out to the existing *Python* `kb scaffold-matrix`, or emits
  the matrix directly. **Decide whether a minimal Rust `scaffold-matrix` is pulled into
  M4b** (small scope creep, but closes one authoring-tail verb) **or deferred.**
  Recommendation: defer — emit the matrix from M4b's own transform tool; keep M4 focused
  on ingest.

- **DEC-D — is the M4b legacy→`cacg.v0` transform a Python one-shot tool or a Rust verb?**
  The M3 first-bite used a Python script (`build_cfa_first_bite_corpus.py`). A Rust
  `kb migrate-cfa` verb would be cleaner long-term but is new surface. **Recommendation:
  keep it a Python one-shot tool for M4b** (matches the M3 precedent, the migration tool
  is not a trust-bearing hot-path component, and Python already parses the legacy
  pseudo-YAML); a Rust migration verb can be a later milestone if the recipe generalizes.

- **DEC-E — Layer-3 sequencing.** Option C (Layer-3 semantic) is the natural post-M4
  phase, but *only M4b's D11 findings* tell the project how urgent it is. **Pre-agree
  that the post-M4 milestone choice (Layer-3 vs broader migration vs authoring-tail) is
  explicitly gated on the D11 Layer-2-failure-rate number** — do not pre-commit the
  milestone after M4 until that data exists.

- **DEC-F — B1 semantic-cache builder location (post-M4, but flag now).** §6.3 shows
  there is no healthy pure-Rust embedding crate (`rust-bert` is stale; `candle`/`fastembed`
  are heavy). **Pre-agree that the B1 cache-builder stays offline / out-of-band (a Python
  `sentence-transformers` script), so the Rust workspace never takes an embedding
  dependency** — consistent with `docs/semantic-verifier.md`'s existing "Phase 3.1, out
  of scope" stance. Surfacing this now prevents a late C-phase scramble.

- **DEC-G — `chunks_manifest.json` granularity for a 70-PDF corpus.** A full migration
  ingests ~70 PDFs; CACG's schema today implies one `chunks_manifest.json`. Decide whether
  the full corpus is one combined chunks manifest or per-source manifests. M4b (one
  vertical, one PDF) does not force this, but the plan-generator should note it as a
  *full-migration* design fork to resolve before scaling past M4b.

---

## 9. Summary

The CACG Rust port has, through M5, built a complete, fast, parity-gated trust-and-
retrieval engine — but it cannot yet reach the user's ultimate goal (a real, trustworthy
refactor of the 268-card `../CFA_reading` corpus) because the **binding constraint is
C1: there is no Rust PDF-to-hash-pinned-chunk ingest.** Without real extracted chunks,
every migrated citation is a synthetic placeholder that certifies nothing — the M3
first-bite proved the migration *plumbing* on 4 cards with synthetic chunks, not the
*grounding*. The recommended next phase is **milestone M4 — Pdfium Ingest + Real
First-Vertical Migration** (Option E): sub-phase **M4a** ships `cacg-ingest` (Pdfium
extraction, page-window chunker, `chunk_hash` envelope, pair-atomic publish, `kb ingest`)
behind a hard byte-equal Pdfium parity **checkpoint**; sub-phase **M4b** then runs the
first *real* migration of one vertical (`01_quantitative_methods`, 17 cards) against
genuine chunks and reports the empirical Layer-2 exact-substring pass/fail rate as the
scoping input for Layer-3. The runner-up is **M4-core alone** (Option A — ship ingest,
defer the real migration) if the round budget cannot absorb E. M4 needs **no new crates** —
`pdfium-render 0.9.1` and every other dependency are already pinned and healthy on
crates.io as of 2026-05-23; the real M4 liability is pinning an exact Pdfium *binary*
build. This recommendation ties directly to the goal: it removes the one constraint that
makes the goal structurally unreachable, validates the removal on real corpus data within
the same milestone, and is sequenced as the rigorous, checkpointed, sub-phase-split
milestone the user consistently prefers.
