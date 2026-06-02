# _research/30 — CFA-legacy CACG v0 Release & Rebuild Recipe

> **Superseded for the current milestone by `_research/31_v1_release_and_rebuild_recipe.md`.**
> This document is the historical v0 record (268 active + 6 quarantined = 274 across 11 readings, 70
> sources). The **v1** milestone (402 active + 6 quarantined = 408 across 14 readings, 87 sources — the
> migration of readings 14/15/22) is recorded in `_research/31`, which is the active release recipe the
> release-cleanliness gate consults. The counts below describe v0 and are kept as the v0 baseline.

This document is the release record for the CFA-legacy CACG **v0 release
candidate**. It states the v0 definition-of-done, enumerates the tracked release
artifacts, gives the intentionally-untracked **disposition note**, documents the exact
**rebuild recipe** for the excluded `chunks_manifest.json`, records the **fresh-root git
lineage**, and points at the reproducibility + scope evidence. It is the successor to
`_research/29` (the decision ledger).

A re-runnable, fail-closed checker —
`sources/cfa/_registry/check_release_cleanliness.py` — re-derives the
tracked/ignored/disposition facts from git + disk and fails on drift (so this document
cannot silently rot).

## v0 Definition of Done

**v0 complete := 268 active emitted cards + 6 notes-taint quarantined cards (274 legacy
total).** All ten acceptance criteria pass, each backed by a re-runnable gate:

| AC | What | Gate |
|----|------|------|
| AC-1 | Truthful CLI + help-snapshot + missing-card diagnostic | `cargo test -p cacg-cli` (`kb_help_snapshot`, `kb_verify`) |
| AC-2 | Reconciled counts + disjoint scope ledger (268/6/0 = 274) | `build_scope_ledger.py` |
| AC-3 | Clean full-corpus Layer-1 lint | `run_corpus_gate.sh` step [2/4] |
| AC-4 | Clean full-corpus Layer-2 verify (268/268) | `run_corpus_gate.sh` step [3/4] |
| AC-5 | Anchor-truth review of the 6 never-reviewed slices, all M/E/H closed | `build_anchor_truth_review.py` |
| AC-6 | Active scope = 268 (Pedersen/Cochrane maps re-derived; 5 PM cards) | `emit_09_pm_slice.py --plan-only` + `test_emit_09_validation.py` |
| AC-7 | Notes-taint quarantine frozen + invariant-checked | `check_quarantine_invariant.py` (gate step [4/4]) |
| AC-8 | `_research/29` decision ledger (Q1–Q12 + rulings) | `check_decision_ledger.py` |
| AC-9 | Byte-reproducible frozen index | `check_index_reproducible.py` (+ `release_baseline/index_repro.json`) |
| AC-10 | Clean, tracked, tagged release | this doc + `check_release_cleanliness.py` |

Constraints honored (per the plan): no re-ingest for v0; no citation-frontmatter schema
change (`volume_page` stays registry-side, DEC-2); Critical Rule 9 unrelaxed (no
`notes_provenance` on active cards); no `source_id` renames.

## Tracked release artifacts

The release is self-describing from version control:

- **268 active card bodies** — `cards/cfa/<reading>/<card>.md` (11 readings:
  01, 02, 03, 05, 06, 07, 08, 09, 10, 11, 17).
- **268 trust-chain history sidecars** — `cards/cfa/**/*.history.jsonl`
  (checksum-chained audit trail; one per active card).
- **The small published index** — `out/cfa/cards_manifest.json` (~194 KB),
  `out/cfa/summaries.json` (~200 KB), `out/cfa/INDEX.md` (~28 KB).
  Force-tracked via the `.gitignore` re-include chain (the `/out/` tree is otherwise
  ignored). Byte-reproducible under `KB_FROZEN_CLOCK=1` (AC-9).
- **Pdfium provenance** — `out/cfa/pdfium_provenance.json` (~1.5 KB): records
  BOTH the documented canonical pin (`pdfium 149.0.7825.0`) AND the operational ingest
  runtime actually used (`libpdfium-nojs 7778.r8.72ea487e43-1` at `/usr/lib/libpdfium.so`,
  via `pdfium-render`), plus a `pin_gap_warning`: the 70-source corpus was ingested
  against the libpdfium-nojs build, NOT the canonical pin, so the chunk-text bytes +
  `chunk_hash` values are tied to that runtime. This is the Q4/AC-8 reproducibility
  evidence; recording it is sufficient for v0 (no re-ingest). The rebuild recipe below
  therefore requires the SAME libpdfium-nojs 7778 build to reproduce existing hashes.
- **Registry + gates** — `sources/cfa/_registry/` scripts (scope ledger, anchor
  review, quarantine, decision-ledger, index-repro, release-cleanliness checkers; the
  emitters; curated-citation registries; page-coordinate maps) and the tracked
  `release_baseline/` artifacts (`scope_ledger.json`, `index_repro.json`,
  `lint_journal_baseline.jsonl`, `verify_baseline_tally.json`,
  `anchor_truth_review.json` + resolutions/verdicts).
- **Decision records** — `_research/23`–`_research/29` (bootstrap → decision ledger) and
  this `_research/30`.

## Intentionally-untracked artifacts (disposition note)

Every path below is excluded from version control on purpose; each is either large,
regenerable, transient, or loop-local. `git status --porcelain --untracked-files=all` is
release-clean precisely because these are all `.gitignore`-covered.

| Path | Why untracked | How to recover |
|------|---------------|----------------|
| `out/cfa/chunks_manifest.json` (~138 MB) | Large ingest product (DEC-3: never commit). 57,603 chunks. | Rebuild recipe below. |
| `out/cfa/source_matrix.json` (~5 KB) | Regenerated by the SAME ingest+merge as chunks_manifest; kept with it for consistency (a half-tracked ingest output set is worse than a fully-regenerated one). | Rebuild recipe below (emitted by `merge_ingest_manifests.py`). |
| `out/cfa/lint_journal.jsonl` (~9.4 MB) | Transient — rewritten on every `run_corpus_gate.sh` run (`--journal`); not a release artifact. | Re-run the corpus gate. |
| `out/cfa/sources_manifest.json`, `out/cfa/summaries.sqlite`, `out/cfa/ingest_per_source/` | Ingest/search side-products (sources manifest, FTS5 sidecar, per-source ingest dirs); regenerated by ingest + `kb index`. | Rebuild recipe below + `kb index`. |
| `out/cfa/_repro/` | AC-9 reproducibility staging (throwaway `kb index` copies); auto-cleaned by `check_index_reproducible.py`. | Re-run the AC-9 harness. |
| `framework.tar.gz` / `*.tar.gz` | Multi-GB local working-tree self-snapshot. | n/a (local backup only). |
| `target/` | Cargo build output (the `kb` binary). | `cargo build --workspace`. |
| `.humanize*` | RLCR loop state (round contracts/summaries, goal-tracker, bitlesson). | n/a (process artifacts, not release content). |
| `out/semantic_cache.json` (+ provenance) | Already force-tracked separately (B1 cache, `_research/20`). | n/a. |

## Rebuild recipe — `out/cfa/chunks_manifest.json` (+ `source_matrix.json`)

Prerequisites:
- Pdfium pinned to **build 7778** (`chromium/7778`) at `/usr/lib/libpdfium.so` (matches
  `out/cfa/pdfium_provenance.json`); the source PDFs under
  `sources/cfa/pdfs/<reading>/`.
- `cargo build --workspace` (the `kb` binary at `target/debug/kb`).

Run from the repo root, under the frozen clock for deterministic chunk hashes:

```bash
# Phase A — per-source ingest (chunk + hash every authorized source). This wrapper
# reads the per-source plan from page_offset_worklist.json and drives `kb ingest`; it
# takes no path flags (paths are fixed by the plan). Optional: --only <source_id> /
# --limit N / --start-after <id> for partial runs; --dry-run to preview.
KB_FROZEN_CLOCK=1 python3 sources/cfa/_registry/run_ingest_per_source.py

# Phase B — merge the per-source manifests into the corpus chunks_manifest +
# source_matrix (70 = the authorized source count; --out names the output dir):
python3 sources/cfa/_registry/merge_ingest_manifests.py \
    --force --require-count 70 --out out/cfa
```

This regenerates `out/cfa/chunks_manifest.json` and
`out/cfa/source_matrix.json` with chunk hashes identical to those the 268 cards
cite (the citation `chunk_hash` envelope is invariant — no card edits needed). Verify
with `KB_FROZEN_CLOCK=1 sources/cfa/_registry/run_corpus_gate.sh` (expects
268/268). DO NOT commit `chunks_manifest.json` (DEC-3).

## Fresh-root git lineage (disposition)

The working copy this loop inherited had **no `.git` and no `target/`** (only a 2 GB
`framework.tar.gz` self-backup); the original migration history — including the plan's
stated `base_commit c2a3a1c` — is unrecoverable (absent from every on-disk repo; the
tarball carries no `.git`). Git was therefore **re-initialized on `master`** with a
fresh root baseline commit, and all stabilization rounds (0 → 13) build on it.

Implication for this tag: the `v0-candidate` tag diffs against a **re-initialized tree**,
not the original lineage. The legacy provenance is preserved as DATA, not git history:
`sources/cfa/_registry/snapshot.json` records `legacy_git_head =
856c4f3cfa9228ac6c4fd4a23e60ee90556b4225`, `legacy_git_unpushed_count = 30`, and
`artifact_set_id = cfa-legacy-v0`; the card `.history.jsonl` sidecars carry the per-card
trust chain. This is the agreed disposition: the trust chain lives in the
content-addressed cards + manifests, not in git ancestry.

## Reproducibility & scope evidence

- **Byte-reproducible index (AC-9):** `sources/cfa/_registry/release_baseline/index_repro.json`
  — two `KB_FROZEN_CLOCK=1 kb index` runs over the same input are byte-identical for
  `cards_manifest.json` / `summaries.json` / `INDEX.md`; the published artifacts are
  clock-invariant; the frozen clock is load-bearing on the audit sidecar. Re-derive with
  `check_index_reproducible.py`.
- **Scope reconciliation (AC-2):** `sources/cfa/_registry/release_baseline/scope_ledger.json`
  — active_emitted 263 + active_deferred_then_emitted_this_loop 5 + quarantined 6 +
  excluded 0 = 274; disk == manifest == sidecars == 268 active. Re-derive with
  `build_scope_ledger.py`.
- **Corpus gate (AC-2/3/4/7):** `run_corpus_gate.sh` — scope ledger PASS, lint exit 0
  (268 journal entries), verify 268/268, quarantine invariant PASS.

## The v0-candidate tag

An annotated **`v0-candidate`** tag is cut on the release commit (local; not pushed),
after AC-1 through AC-9 pass and `git status` is release-clean. Its message references
this document, the scope ledger, and the AC-9 reproducibility result.
