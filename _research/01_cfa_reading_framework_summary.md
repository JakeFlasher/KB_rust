# CFA_reading Framework — Exploration Summary

Read-only reference: `/home/jakeshea/CFA_reading/`. Captured here for use as research input only; not a working artifact of the new framework.

## Mission

A personal study corpus distilling the 17-area CFA Level 1 curriculum (plus a convertible-securities specialization) into vivid, rigorous quick-read books with ASCII diagrams and fully traceable PDF citations.

Core principles:

- **No worked numerical examples** in deliverable books; intuition + mathematical reasoning required.
- **Every fact carries PDF + page citation** in frontmatter (`Primary raw source`, `Supporting sources`) and inline (`**Source:** <author> (<year>) §<section> pp.<N-M>`).
- **Diagrams are ASCII only** (no images, no mermaid/plantuml/graphviz).
- **Only OK/GOOD-rated PDFs are quotable**; EPUBs entirely blacklisted (page anchors missing).
- **Topic-first directory layout** with 17 CFA topic slots, 10 currently complete.

## Status

- 10 Codex-verified verticals complete: 01, 02, 03, 05, 06, 07, 08, 09, 11, 17.
- 219 cards aggregate, indexed in FTS5 SQLite manifest.
- 7 verticals empty placeholders (04, 10, 12-16).

## Knowledge card schema

Mandatory YAML frontmatter:

```yaml
---
Use when: <one-line trigger>
Primary raw source: <relative-pdf-path> pp.<N-M>
Supporting sources:
  - <relative-pdf-path> pp.<N-M>
Repo touchpoints:
  - <other-card-path>
Out of scope: <explicit exclusions>
Version family: <CFA cycle / book edition>
CFA Relevance: core | adjacent | extension
Source Stance: primary-<stance> | supporting-only
deliverable-ready: true | false
---
```

Body sections (mandatory order):

1. `## Intuition` — paragraph + optional ASCII diagram
2. `## Definition` — rigorous definition at source's level
3. `## Mathematical Reasoning` — derivations, key results
4. `## See Also` — relative-path links to sibling cards
5. `## Escalate to Raw When` — re-open the source PDF

Citation format examples:

- Inline: `**Source:** DeSpiegeleer et al. (2014) §2.1-§2.2 pp.21-30`
- Combined CFA PDF: `Vol.5/pp.42-58` (linter code CITE-005)

## Supporting infrastructure per closed vertical

- `_source_role_map.md` — every card's primary anchor + supporting anchors + page spans + CFA Relevance + Source Stance.
- `_dependency_order.md` — DAG of prerequisite relationships.
- `_diagram_primitives.md` — canonical ASCII diagram library, with whitespace-normalized SHA256 enforcement (PRIM-002/PRIM-005).
- `_style_guide.md` — topic-specific authoring rules; must `Extends: ../_style_guide_common.md`.
- `_chapter_overviews.md` — auxiliary chapter→card JSON map (e.g., `CHAPTER_OVERVIEW_MAP_V1` with `allow_duplicate_card_coverage: true`).

## Top-level `.claude/` artifacts

- `kb_manifest.sqlite` — FTS5 full-text index (gitignored, rebuilt).
- `kb_manifest.json` — parallel JSON dump for non-SQLite consumers.
- `retracted_cards.json` — SHA256-mismatch / missing-source retraction log.
- `dist/` `dist_summary/` `dist_volumes/` — generated HTML outputs (gitignored).
- `skills/` — 11 task-shaped SKILL.md routers (per-topic + meta-router `navigating-cfa-kb`).
- `memory/snapshot/` — portable user-level memory for new-machine setup.

## Source eligibility matrix

`_corpus_planning/05_source_matrix.md` — 31 data rows, columns:

- PDF path (relative to repo root)
- Format (pdf/epub), Edition
- `audit_rating` (OK | GOOD | SCAN | POOR)
- `quotable: yes/no` (defense-in-depth)
- `primary_or_supporting`
- SHA256 hash
- `subcorpora` (two-digit topic numbers authorized to cite)

## Verification & linting pipeline

Defense-in-depth dual validation:

**Stage 1 — `scripts/kb/lint_cards.py` (1187 LOC, 41 diagnostic codes)**

- FM-* (5 codes): frontmatter parse + required field checks + enum validation.
- CITE-* (13 codes): PDF path parse, matrix existence, quotable=no, audit rating, undisambiguated CFA volume, page bounds, reversed range, banned spans (e.g., Hull pp.602-615), subcorpus authorization, placeholder detection.
- EPUB-* (3 codes): frontmatter/body cite .epub, blacklisted author-year reference.
- NUM-*: numerical example detection (banned).
- SEC-*: section structure validation.
- DENS-*: every paragraph must end with `**Source:**` or relative backing-card link.
- SKILL-*: SKILL.md schema.
- DEP-*: dependency-order acyclicity.
- PRIM-*: diagram primitive instantiation drift.
- STYLE-*: per-topic style guide extends + overrides table.
- TRANS-*: transitive backing-card-link resolution.

**Stage 2 — `scripts/kb/build_manifest.py` (736 LOC)**

- 19 unit tests mirror every CITE-* invariant.
- Atomic publish: writes `.tmp`, validates FTS5 round-trip + JSON parse, atomically renames.
- Retraction check (default on): SHA256 mismatch, missing quotable source, or audit-rating downgrade triggers retraction into `.claude/retracted_cards.json` and exits 1.

**Stage 3 — `scripts/kb/check_volume_citations.py` (295 LOC)**

- Per-volume DENS-001 analog: every paragraph cites a Source or backed-card link.
- Outline-to-card mapping (OUTLINE-001/002/003).

**Stage 4 — pytest suite (6 entry points, 80 fixtures)**

- `test_lint_cards.py`, `test_build_manifest.py`, `test_fm006_per_vertical_stance.py`, `test_check_volume_citations.py`, `test_check_source_matrix.py`, `test_build_html_volumes.py`.

## Build tooling

- `build_html_cards.py` — per-card standalone HTML with HTMLParser allow-list sanitizer.
- `build_html_summary.py` — graduate-student compressed-notes summary HTML.
- `build_html_volumes.py` — interactive single-page HTML volume book; CSS Grid sticky TOC; `<details>/<summary>` collapsibles; `:target` highlighting; `prefers-color-scheme`; zero JavaScript.
- `tools/audit_pdf_quality.py` — PDF text-density rating via pdfinfo/pdftotext.
- `scripts/kb/smoke_manifest_scaling.py` — latency smoke: 12 legacy + 7 AC-49 + 3 AC-66 + 8 AC-83 FRA + 8 QM + 13 ethics + 10 econ + 10 risk FTS5 predicates; asserts MIN_CARDS >= 214 + no query exceeds 200 ms median.

**Forbidden dependencies:** pandoc, PyYAML, networkx, graphviz, pydot, JavaScript frameworks, remote network, CDN assets.

## Context-saving patterns observed

- Task-shaped SKILL.md routers as primary discovery layer.
- INDEX.md as thin fallback inventory.
- SQLite FTS5 manifest for sub-millisecond retrieval (well below 50ms target).
- Retraction log as auditable PDF-replacement trail.
- ASCII-only diagrams keep cards small + plain-text searchable.

## Pain points & gaps (from STATUS.md and BitLessons)

- 7 uncarded topic areas remain.
- EPUB blacklist friction: high-quality EPUBs (Maitland 2022, 攻守) currently non-quotable until PDF surfaces.
- PDF corpus partial organization: 17-topic directory tree at root sparsely populated.
- Page-number stability: combined CFA L1 PDF (6 vols, 4353 pages) requires `Vol.<N>/pp.<P-Q>` qualification.
- BitLessons recorded:
  - `BL-20260504-manifest-source-sha256-defect` — SHA256 mismatch detection during retraction.
  - `BL-20260504-marker-resolution-check` — transitive backing-card-link resolution can fail when target section drifts.

## Strengths to preserve

- Atomic manifest publishing prevents partial/corrupted indexes.
- SHA256 hashing detects silent file corruption.
- Defense-in-depth dual validation catches citation drift early.
- Subcorpus-agnostic gates: pointing linter/manifest at a new topic works without code changes, given matrix rows + dependency graphs.
- ASCII-only diagrams + plain-markdown cards eliminate vendor lock-in.
- 12-round iterative refinement methodology recorded in BitLessons.

## Friction observed

- 41 diagnostic codes is a lot of cognitive load to keep in working memory.
- The matrix is a single 31-row file — fine for now, fragile at 200+ readings.
- Smoke test of FTS5 latencies is bespoke; would benefit from a generic harness.
- No streaming / partial-card lint — full card must be written before verifying.
- No content-addressable card identity — edits don't bump a hash, must rely on git.
- No automated check that quoted text **appears in the cited PDF page span** (linter checks bounds but not content).
