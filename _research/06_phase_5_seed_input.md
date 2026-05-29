# CACG Phase 5 Seed: Phase-4.x Closure + Operator Read Surface + CFA-Parity Graph Governance

## Context (post-Phase-4 state)

- `/home/jakeshea/CFA_reading/` — read-only design oracle: 248-card production KB with 3 HTML builders (per-card + summary cheatsheet + volume bundles), `_dependency_order.md` per subcorpus with topological validation, `_diagram_primitives.md` registry with PRIM-001..006 enforcement, 49-code linter (vs CACG's ~30 today), PDF quality audit, per-reading source role-maps with stance + relevance + audit ratings.
- `/home/jakeshea/humanize/` — Claude Code RLCR plugin (v1.17.0). CACG integrates via `kb verify --round-summary`.
- Current CACG state — **Phase 4 SHIPPED MERGE_READY end-to-end** (613 tests passing under `KB_FROZEN_CLOCK=1`; final Codex closing audit `MERGE_READY_THROUGH_PHASE_4` after closing 3 P1 findings in one rework round).

## Three research streams seeded this phase

1. **Codex strategic review** (2026-05-20, gpt-5.5:high, 137s): top-3 ROI ranked by trust/operator/perf value. Recommendation: 3 milestones. **M0 closure** (2 rounds) — Phase-D2 content-hashed skip + 100k AC-P4 fixture + conftest auto-injection removal. **M1 operator** (3-4 rounds) — `kb show` then static no-JS HTML per-card + INDEX. **M2 CFA-parity governance** (3-4 rounds) — port dependency-order validation, primitive registry, banned-fence checks, drift detection, BUT do NOT duplicate CACG's `card_edges` as source of truth.

2. **CFA-parity audit** (Explore subagent, 105s): file-by-file evidence inventory of what CFA has that CACG lacks. 49 CFA lint codes vs ~30 CACG; 23 CFA-only codes (EPUB, NUM, SEC, DENS, EDGE, PRIM, STYLE, OUTLINE, TRANS, SCOPE in full). HTML builders at `build_html_cards.py:854-920`, `build_html_summary.py:1-78`, `build_html_volumes.py:1-54` (~925 LOC for per-card alone). `_dependency_order.md` validated by `_lint_doc_overview.py:180-229` (`check_outline_topological`). `_diagram_primitives.md` validated by PRIM-001..006 in `lint_cards.py:1502-1510` + `_lint_misc_gates.py`. PDF audit at `tools/audit_pdf_quality.py`. Total Phase 5 scope estimate: ~3,000-4,500 LOC of new validation + output logic.

3. **Post-Phase-4 residual audit** (Explore subagent, 89s): NO blocking debt. 92 test files / 16.7K lines, no FIXMEs/XXXes (clean). 2 real pyright errors (`chunker.py:212` page_spans variance; `cli.py:1561` `SidecarStaleError` possibly unbound in except). Dead code is one stub (`_reject_anchors_and_tags`, internal-only). `conftest.py:_patched_cli_main` still auto-injects `--source-matrix` with per-injection deprecation warning; ~50 test sites depend on it. Duplicated `_source_ids_sorted_unique` validator across `CardManifestEntry` + `SummaryEntry` (DRY violation, low impact). Reserved codes `CACG-IDX-CACHE-001`, `CACG-CARDS-DEP-001` documented but never fire (defined-but-unused).

The three streams CONVERGE: the operator read surface (`kb show` + HTML) is the highest user-facing ROI; Phase-4.x perf closure is the highest trust ROI; full CFA-parity is a deliberate stretch that should be split from operator work.

## Problem (combined scope: closure + operator + governance)

The Phase-4-shipped framework has three distinct classes of debt the next phase should close:

1. **Phase-4.x perf + trust closure** (Codex closing-audit residuals):
   - AC-P9 missed at 10k scale: warm-cache `kb index` at 725ms vs 100ms STRICT / 200ms LOOSE. Root cause: Phase-D2 republish (`summaries.json` + FTS5 sidecar + `cards_manifest.json` + `INDEX.md`) runs UNCONDITIONALLY on every `kb index`. Proposed fix: **content-hashed Phase-D2 skip** — compute SHA-256 of would-be `summaries.json` bytes, compare against the prior commit's hash; if unchanged, skip both JSON replace AND FTS5 rebuild AND cards_manifest + INDEX republish. Same pattern for unchanged cards_manifest payload.
   - AC-P4 untested at 100k: FTS5 cold-open <50ms claim is functional-only. Need to build a 100k-card stress fixture (4x larger source PDF; `N_CARDS=100000 N_PAGES=10000`) and add a perf-test gate.
   - AC-T7 partial: `tests/conftest.py:_patched_cli_main` still monkey-patches `cacg.cli.main` at import time. Per-injection deprecation warning is visible but ~50 test sites still depend on it. Full removal requires updating each call site to use the explicit `permissive_source_matrix` fixture.
   - Two pyright type errors: `chunker.py:212` page_spans variance, `cli.py:1561` SidecarStaleError possibly unbound. Cheap fixes.

2. **Operator read surface** (Codex M1 + CFA-parity audit):
   - `kb show <card_id>` verb missing. Operators currently `cat cards/.../foo.md`. CFA also lacks this (shared gap; CACG can lead). Should pretty-print: title + summary + frontmatter table + citations table + linked-cards section + verification status. Resolve by `card_id` through `cards_manifest.json`; refuse directly-retracted AND dependency-retracted cards by default; `--allow-retracted` opt-out. `--json` for machine consumption + human/markdown for terminal.
   - Static no-JS HTML output layer missing. CFA has 3 builders totaling ~1,100+ LOC. Phase 5 ships the FIRST one — per-card pages + `INDEX.html` navigator. Deterministic output (byte-identical under `KB_FROZEN_CLOCK=1`); sanitized Markdown→HTML; links resolve through declared `card_edges`; source/citation metadata visible; respects retraction state. **Defer summary cheatsheet + volume bundlers** to a later phase.

3. **CFA-parity graph governance** (Codex M2 + CFA-parity audit):
   - Dependency topological validation missing. CACG has `CardEdge {target, edge_type ∈ {depends_on, extends}}` and validates cycle-freedom (CACG-DEP-001..004) but cannot generate a reading order OR validate that an outline respects the DAG. CFA's `check_outline_topological` (`_lint_doc_overview.py:180-229`) is the reference. Codex strongly recommends: `card_edges` REMAINS the source of truth; the generated dependency-order is a VIEW, not a parallel artifact. Do NOT add `_dependency_order.md` per-subcorpus.
   - ASCII diagram primitives missing. CFA has `_diagram_primitives.md` per subcorpus with HTML-comment headers (`<!-- primitive: <name> -->`), max-80-col enforcement, drift-by-canonical-hash, banned-fence-language detection (mermaid/plantuml/graphviz/dot/d2/kroki/vega/vegalite all rejected). CACG would add `cards/reading_NN/_diagram_primitives.md` per-reading + a corpus-wide collision check + 6 PRIM-* lint codes.
   - 23 missing CFA lint codes. Highest-ROI batch: PRIM-* (6), OUTLINE-* (3), TRANS-* (2), SCOPE-* (3) — these are the trust/governance gates. EPUB/NUM/SEC/DENS/STYLE codes are more stylistic and can be deferred to a follow-on.
   - PDF quality audit missing. CFA has `tools/audit_pdf_quality.py` returning OK/GOOD/SCAN/POOR. CACG's `kb ingest` trusts the PDF blindly. Adding `kb audit-pdf <path>` with the OCR-text-density heuristic closes this gap. **Could be deferred** to Phase 5.1 if scope pressure emerges.

## Three combined milestones (M0 + M1 + M2)

### M0: Phase-4.x Closure + Trust Harness Cleanup (2 rounds)

- **Round 0**: Content-hashed Phase-D2 skip. New helper `_should_skip_phase_d_republish(out_dir, candidate_summaries_bytes)` compares SHA-256 of the candidate `summaries.json` payload against the current on-disk file's hash. If match: skip `summaries.json` write, skip FTS5 sidecar rebuild (sidecar's `meta.summaries_hash` will already be correct), AND skip `cards_manifest.json` + `INDEX.md` republish when those payloads are also unchanged. Tests: unchanged warm-cache `kb index` produces ZERO file writes (verified via mtime comparison). Re-measure AC-P9 on the 10k fixture; gate at the strict 100ms target. Also fix the 2 pyright errors (`chunker.py:212`, `cli.py:1561`).
- **Round 1**: 100k stress fixture builder (`N_CARDS=100000 N_PAGES=10000`) + perf test `tests/perf/test_phase4_fts5_100k_cold_open.py` asserting AC-P4 <50ms with 2x margin loose. Remove `tests/conftest.py:_patched_cli_main` monkey-patch entirely; sweep all ~50 test sites to use `permissive_source_matrix` fixture explicitly. Add a meta-test that asserts no test invokes `cli.main(["lint" | "verify", ...])` without `--source-matrix` (static grep).

### M1: Operator Read Surface (3-4 rounds)

- **Round 2**: `kb show <card_id>` verb. Resolver loads `cards_manifest.json` + `summaries.json`; finds entry by id. Default rejection: card in `retracted_cards` OR `dependency_retracted_cards` exits 1 with CACG-SHOW-001 (new) "card is retracted; use --allow-retracted to display historical artifact." Optional `--allow-retracted` downgrades to a warning (analogous to verify's `--allow-retracted`). Mandatory `--source-matrix` (per Codex DEC recommendation; resolve-by-id needs auth context). Outputs: default human/markdown (title + summary + frontmatter table + citations + linked-cards), `--json` for machine consumption. Optional `--path <p>` overrides id-resolution for path-based callers.
- **Round 3**: HTML renderer foundation. New module `src/cacg/render_html.py` builds `out/html/<card_id>.html` per card + `out/html/INDEX.html` navigator. Stdlib `markdown` library OR a constrained inline renderer (DEC; see open questions). Inline CSS (no external assets), no JavaScript, deterministic output under `KB_FROZEN_CLOCK=1`. Sanitized Markdown→HTML (no script tags, no on-* event handlers). Links to other cards through `card_edges` resolve to `<card_id>.html` relative URLs.
- **Round 4**: `kb render-html` CLI verb. Mandatory `--source-matrix`. Respects retraction: directly-retracted cards are EXCLUDED from output by default (cannot be reached via INDEX.html); dependency-retracted cards ARE rendered but carry a visible "⚠ Dependency Retracted" badge. INDEX.html groups by `reading_id`, sorts by `(reading_id, id)`. Footer carries the manifest seal (sha256 of cards_manifest.json) + render timestamp (frozen under KB_FROZEN_CLOCK).
- **Round 5**: HTML refinement. Citation table inside each card page renders `chunk_hash`, `page_range`, `quote`, `source_id`. Cross-card link safety: any `card_edges.target` not in the active cards_manifest renders as `<a class="dangling">` with `aria-label="dangling reference"`; cycles surface as a banner. Determinism gate: `tests/test_phase5_html_determinism.py` builds the corpus twice under `KB_FROZEN_CLOCK=1` and asserts byte-identical HTML output.

### M2: CFA-Parity Graph Governance (3-4 rounds)

- **Round 6**: Dependency-order generator + outline validator. New verb `kb dep-order [--reading <id>]` topologically sorts the active cards by `card_edges.depends_on` and writes a generated view `out/dep_order/<reading_id>.md` (or stdout). Cycles emit CACG-DEP-005 (new). New lint `kb lint --check-outline <outline.md>` parses an authoritative reading outline and asserts every card appears AFTER all its declared prerequisites (CACG-OUTLINE-001..003 new). The outline is a SECOND artifact (similar to CFA), but `card_edges` remains the source of truth — the outline is checked against the DAG, not vice versa.
- **Round 7**: Diagram primitive registry. New optional file `cards/reading_NN/_diagram_primitives.md` per reading with `<!-- primitive: <name> -->` HTML-comment headers + canonical body inside fenced code blocks. New lint pass:
  - CACG-PRIM-001: primitive width exceeds 80 columns
  - CACG-PRIM-002: declared primitive not found in registry
  - CACG-PRIM-003: instantiation drift (body hash differs from registry canonical)
  - CACG-PRIM-004: name collision across readings (corpus-wide check)
  - CACG-PRIM-005: empty primitive body
  - CACG-PRIM-006: banned fence language (`mermaid|plantuml|graphviz|dot|d2|kroki|vega|vegalite`)
- **Round 8**: SCOPE-* role-map governance + OUTLINE-001..003 + TRANS-001..002. Per CFA: SCOPE-001 retracted card in role-map; SCOPE-002 active card missing from role-map; SCOPE-003 stance/relevance enum violation. OUTLINE codes for outline-vs-DAG drift (R6). TRANS codes for outline-card transitive coverage.
- **Round 9**: PDF audit (`kb audit-pdf <path>`) + closure. OCR-text-density heuristic: extract text via pypdfium2 (already a dep); compute `chars_per_page` and `chars_with_alpha_ratio`; classify as `OK | GOOD | SCAN | POOR`. New `cards_manifest.sources_audit.json` artifact pinning the rating per source_id. Lint code CACG-INGEST-002 (PDF quality below threshold). **Closure work**: final Codex audit pass + 10k AC-P9 re-measure (should now pass post-M0 R0 with content-hashed skip) + final demo refresh + `perf-reports/phase-5-closure.md`.

## Non-goals (explicitly deferred)

- **MCP server** — Codex explicitly recommends deferring until the operator read surface (kb show + HTML) is in operator hands and usage data is available.
- **Multi-LLM concurrent judging for semantic verifier (B1+B2 ensemble)** — current single-judge contract is intentionally non-authoritative; ensemble adds expensive nondeterminism without changing the trust contract.
- **PyO3 / mypyc / Cython hot-path bindings** — Phase 4 proved bottlenecks are artifact-republish-shaped, not Python-bytecode-shaped. DEC-Rust-Port (Phase 4) remains locked.
- **Windows fcntl parity** — POSIX-only is acceptable while journal semantics are still evolving. Defer to a named Windows operator request.
- **HTML summary cheatsheet (`build_html_summary.py` analog)** — CFA-specific exam-prep artifact; defer.
- **HTML volume bundler (`build_html_volumes.py` analog)** — product-specific; would overfit CACG to a single use case.
- **23-code-full CFA-parity lint port** — only PRIM-* + SCOPE-* + OUTLINE-* + TRANS-* ship in Phase 5. EPUB/NUM/SEC/DENS/STYLE codes are mostly stylistic and defer to a follow-on phase or a per-corpus opt-in extension.
- **`_dependency_order.md` as a parallel canonical artifact** — `card_edges` remains the source of truth; only the generated VIEW is added.

## Constraints (non-negotiable per Codex hard-constraints + Phase 4 carryover)

- `KB_FROZEN_CLOCK=1` byte-determinism preserved for ALL user-facing JSON/Markdown/HTML artifacts. Raw SQLite bytes remain excluded (DEC-8 carry-forward).
- `cacg.v0` schema purely additive. NO breaking schema bump in Phase 5 unless a user-adjudicated governance event surfaces. If new fields are needed (e.g., `cards_manifest.sources_audit` reference, primitive registry index), they're additive with empty defaults.
- NO new mandatory runtime dependency. HTML rendering can use stdlib `markdown` OR a constrained inline renderer (open DEC); MUST default to lean common paths (lint/verify/search/index unchanged).
- `kb verify` remains the trust authority. New artifacts (HTML, dep-order, primitive registry, PDF audit) are PERFORMANCE/PRESENTATION artifacts, never proof artifacts.
- **No path exposes unauthorized OR directly-retracted OR dependency-retracted cards by default.** This is explicit for `kb show`, `kb render-html`, AND `kb dep-order`. `--allow-retracted` opt-out is the only escape.
- All new artifacts use atomic publish (tmp + Pydantic round-trip + os.replace + .bak rollback).
- Common-path PDF isolation preserved: display/search/lint/verify/show/render-html MUST NOT import `pypdfium2`. Only `kb ingest` and `kb audit-pdf` may.
- Three-phase atomic publish contract extended: HTML output directory follows the same tmp + sidecar pattern as Phase-3 summaries.json + Phase-4 summaries.sqlite.

## Open questions for the directed-swarm / gen-plan to address

- **`kb show` input contract**: `card_id` primary + `--path` override (Codex recommended), OR accept either positional argument and auto-detect? Tradeoff: card_id-primary forces operators to look up the id (good for trust); path-or-id is friendlier but blurs the boundary.
- **`kb show` retraction default**: refuse-by-default with `--allow-retracted` opt-out (Codex recommended) OR display-with-banner-by-default? Tradeoff: refuse-by-default matches `kb search`'s posture; display-with-banner is friendlier for historical lookup.
- **HTML markdown rendering**: stdlib `markdown` library (well-tested, requires new optional dep) OR a constrained inline renderer in `src/cacg/render_html.py` (no new dep, more code to maintain, smaller attack surface)? Codex flagged this needs explicit decision.
- **HTML output directory**: `out/html/` or `out/dist/` (matches CFA's `.claude/dist/`)? Filename convention: `<card_id>.html` (collision-prone if id has slashes) or `<slug>.html`? Tradeoff: id-named is unambiguous but ugly URLs; slug-named is friendlier but needs collision detection.
- **Dependency-order artifact location**: `out/dep_order/<reading_id>.md` (generated view, matches Phase-3 role_maps shape) OR stdout-only (no persisted artifact)? Persisted-view is grep-friendly; stdout-only avoids stale-artifact risk.
- **Outline-checker input format**: parse a single `outline.md` (CFA-style sequence list) OR walk a directory of per-reading outlines? Match CFA's per-subcorpus shape OR consolidate?
- **Diagram primitive registry scope**: per-reading files only (each `_diagram_primitives.md` lives under its reading) + a generated global collision-check (Codex recommended), OR a single corpus-wide registry at `cards/_diagram_primitives.md`? Tradeoff: per-reading matches CFA + isolates blast radius; corpus-wide is simpler but couples readings.
- **PDF audit thresholds**: hardcoded chars/page bounds (`OK >= 1500`, `GOOD >= 500`, `SCAN < 500`, `POOR < 100`) OR configurable via CLI flag? Hardcoded matches DEC-2 (Phase 4) hardcoded vocabulary pattern; configurable lets operators tune per-corpus.
- **23 missing lint codes — which subset?** PRIM-* (6) + SCOPE-* (3) + OUTLINE-* (3) + TRANS-* (2) = 14 codes. Plus optionally EDGE-005 + DEP-005 = 16. Defer EPUB/NUM/SEC/DENS/STYLE (9 codes)?
- **Conftest cleanup ordering**: do full removal (M0 R1) BEFORE M1 starts (Codex strong rec; matches "no new UX on synthetic test surface"), OR allow M1 R2 to start in parallel with the cleanup? Parallel is faster but risks new tests being written against the auto-injection.

## Suggested orthogonal directions

The directed swarm should diverge across at least these axes:

1. **"Closure-first three-milestone series"** (Codex recommended; current default) — M0 (closure) → M1 (operator) → M2 (governance). Tightest dependency chain; lowest concurrent surface; honest "we shipped what was promised before extending."
2. **"Operator-first"** — M1 first (kb show + HTML), M0 closure work in parallel inside M1's rounds, M2 governance deferred to Phase 6. Highest user-facing value per round but defers the perf gate.
3. **"Governance-first"** — M2 governance first (PRIM + SCOPE + OUTLINE codes), THEN operator surface. Honors the trust-spine hierarchy (governance < operator features) but ships less user-facing value early.
4. **"Minimum-viable cut"** — M0 only + `kb show` only (drop static HTML + all governance). Shortest phase; honest "perf closure + one operator verb." Becomes Phase 5.0; Phase 5.1+ ships HTML + governance.
5. **"Full CFA-parity push"** — ALL 23 missing lint codes + ALL 3 HTML builders (per-card + summary + volumes) + PDF audit + dep-order. Largest scope; ~6 months of work; Codex flagged as too much.

The synthesis picks the strongest as PRIMARY, renders the rest as Alt-1..Alt-K with explicit tradeoffs.

## Success criteria for the seed exploration + gen-plan output

The `/humanize:gen-plan` output should produce a plan document at `.humanize/.humanize/plans/cacg-phase-5-closure-operator-governance-plan.md` that:

- Identifies the PRIMARY direction across the 5 candidate axes above (Codex-recommended is "Closure-first three-milestone series").
- Renders Alt-1..Alt-K alternatives with explicit tradeoffs.
- Surfaces 3-5 open DEC-* decisions the user must close before the loop continues (consolidate the 10 open questions above).
- Specifies 22-30 ACs in TDD format (positive + negative tests), grouped by milestone (C group for M0 closure; O group for M1 operator; G group for M2 governance).
- Names the load-bearing files in `src/cacg/` that each round touches AND identifies new modules to create (`src/cacg/render_html.py`, `src/cacg/dep_order.py`, `src/cacg/diagram_primitives.py`, `src/cacg/pdf_audit.py`).
- Documents the upper-bound vs lower-bound scope (upper = all 10 rounds across 3 milestones; lower = M0 only + `kb show` only per Alt-4).
- Adopts the same convergence + RLCR contract used by the Phase 4 plan (Goal, ACs, Path Boundaries, Feasibility Hints, Dependencies + Sequence, Task Breakdown with `coding`/`analyze` tags, Claude-Codex Deliberation, Pending User Decisions, Implementation Notes, Resolved Disagreements).
- Explicitly references the Phase 4 RESIDUALS the closure milestone is closing (AC-P9 at 10k, AC-P4 100k, AC-T7 conftest cleanup, 2 pyright errors) AND the DECs already locked from Phase 4 (DEC-Rust-Port, DEC-12c third-state model).
- Carries forward the hard constraints: byte-determinism, additive-only cacg.v0, no new mandatory runtime dep, kb verify trust authority, retraction-respect by default on every new operator surface, atomic publish, common-path PDF isolation.
- Notes the Codex-recommended deferrals (MCP, multi-LLM ensemble, PyO3, Windows fcntl, HTML volumes/summary) so the next-next phase doesn't re-litigate them.
