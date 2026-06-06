# hkex deck — registry helpers & reproducibility

Deck-local tooling for the standalone `cards/hkex/` knowledge deck (distilled from the
狗不叫 Xueqiu corpus). These helpers are **adapted from, and never mutate**, the `cfa`
registry. They read but never write `cards/cfa/**`, `out/cfa/**`, or `sources/cfa/**`.

## Helpers (Round 0 / AC-1 + AC-2)

| File | Purpose | Self-check |
|------|---------|-----------|
| `deck_paths.py` | Scaffold guard: a card may only be written under `cards/hkex/`; writes into any `cfa` path are rejected. | `python3 deck_paths.py` |
| `cfa_isolation_guard.py` | Capture + re-check a fingerprint (count, per-card `card_hash`, git tree ids, untracked-file scan) proving the `cfa` deck is byte-untouched. | `--capture` / `--check` |
| `cacg_normalize.py` | Faithful port of the kernel `normalize_text`; used to propose/parity-check matches (the authority is `kb verify`). | `python3 cacg_normalize.py` (21 kernel vectors) |
| `corpus_model.py` | Parse `corpus_complete.md` into utterances; separate each `★AUTHOR` citable text from its `(↪in reply to …)` parent context. | `python3 corpus_model.py` |
| `render_corpus_pdf.py` | Deterministic Noto-CJK renderer → `goubujiao_corpus.pdf`. | `--write` / `--check-determinism` / `--self-test` |
| `check_corpus_parity.py` | Corpus→PDF parity proof (ingest + per-page containment + id mapping over every author utterance). | `--check` |
| `check_cjk_ingest_spike.py` | CJK ingest parity SPIKE (AC-3, hard gate before authoring): ingest with the CJK config; prove single-chunk **author-origin** verbatim binding of all 192 candidate seed quotes (lengthen/pid-correct; fail closed on un-lengthenable multi-match via `seed_overrides.json`; reject `//@non-author` repost spans) + edge/exclusion fixtures; control-char gate; synthetic `kb verify`. | `--check` |
| `run_ingest_per_source.py` | Per-source ingest runner (AC-4): ingests each `ingest_plan.json` source into `out/hkex/ingest_per_source/<source_id>/` with a per-source chunk `--config`; clobber/partial fail closed; resumable; resolves `kb` fail-closed. | (run) / `--force` |
| `merge_hkex_manifests.py` | Deterministic merge (AC-4): composes `out/hkex/{chunks,sources}_manifest.json` (validate one-source/manifest, recompute `source_sha256`+`chunk_hash`, dedup, canonical sort, atomic, byte-identical re-run); writes the committed `ingest_merge_report.json`. | (run) / `--self-test` |
| `html_to_pdf.py` | Deterministic HTML→text-layer PDF recipe (AC-4) for the IRD/IFEC HTML grounding sources; records a `.provenance.json`. | `--html … --out …` / `--self-test` |
| `validate_source_matrix.py` | Validates `out/hkex/source_matrix.json` (AC-5): reading_ids ∈ CFA taxonomy; matrix sources ∈ merged `sources_manifest`; Xueqiu authorized for exactly the inventory-derived authorable readings (no blanket); card-aware no-unauthorized/no-unused once cards exist. READING-level only — §5.2 exclusions + ★AUTHOR attribution are AC-8. | (run) / `--self-test` |
| `seed_overrides.json` / `ingest_plan.json` | Reviewer-authorized seed disambiguations (AC-3); the per-source ingest plan (AC-4). | (data) |
| `out/hkex/source_matrix.json` | Hand-authored authorization matrix (AC-5), committed via the `.gitignore` re-include chain. | (data) |

Committed evidence: `renderer_provenance.json`, `parity_report.json`, `cfa_isolation_baseline.json`,
`cjk_ingest_spike_report.json`, `ingest_merge_report.json`. The reusable per-source ingest output
+ the merged manifests live (gitignored) under `out/hkex/` and are regenerated idempotently.

## Rebuilding the local-only artifacts

Per **DEC-1 / AC-11** the Xueqiu corpus is excerpts-only: these are gitignored and
**rebuilt locally**, never committed (same model as `cfa`'s gitignored
`out/cfa/chunks_manifest.json`):

- `_research/xueqiu_goubujiao/corpus_complete.md` (ingest source), `corpus_full.md`
  (author index), `batches/` — one public author's scraped posts; kept local.
- `sources/hkex/pdfs/goubujiao_corpus.pdf` — the rendered ingest source (`*.pdf` is
  globally gitignored).
- `sources/hkex/_registry/_fonts/NotoSansCJK-Regular-face0.otf` — face 0 extracted
  deterministically from the system `/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc`.

Rebuild + verify from a checkout that has the local corpus + the system Noto CJK font:

```bash
cargo build --workspace                                                  # produces target/debug/kb
python3 sources/hkex/_registry/render_corpus_pdf.py --write              # PDF + provenance
python3 sources/hkex/_registry/render_corpus_pdf.py --check-determinism  # 3 subprocs -> identical SHA == provenance
python3 sources/hkex/_registry/check_corpus_parity.py --check            # corpus->PDF parity PASS
python3 sources/hkex/_registry/check_cjk_ingest_spike.py --check         # AC-3 ingest spike PASS (single-chunk author-origin binding)
python3 sources/hkex/_registry/run_ingest_per_source.py --force          # AC-4 per-source ingest -> out/hkex/ingest_per_source/<id>/
python3 sources/hkex/_registry/merge_hkex_manifests.py --require-count 1  # AC-4 deterministic merge -> out/hkex/{chunks,sources}_manifest.json
python3 sources/hkex/_registry/merge_hkex_manifests.py --self-test        # AC-4 dedup/clobber/sort/sha/atomic/struct
python3 sources/hkex/_registry/html_to_pdf.py --self-test                 # AC-4 HTML->PDF recipe
python3 sources/hkex/_registry/validate_source_matrix.py                  # AC-5 source_matrix authorization PASS
python3 sources/hkex/_registry/validate_source_matrix.py --self-test      # AC-5 negatives
python3 sources/hkex/_registry/cfa_isolation_guard.py --check            # cfa byte-untouched
```

`run_ingest_per_source.py` + `merge_hkex_manifests.py` are adapted from (and never mutate) the
`cfa` registry's `run_ingest_per_source.py` / `merge_ingest_manifests.py`. AC-6 appends the free
grounding sources (HKEX/IRD/IFEC/Bennett) to `ingest_plan.json` — HTML sources are first
snapshotted to PDF via `html_to_pdf.py` — then re-runs the runner + merge with the new count.

`check_corpus_parity.py` resolves the `kb` binary from `KB_BIN` (if set), else
`target/debug/kb` (guaranteed by `cargo build --workspace`), else `target/release/kb`, and
fails closed if none exists — so the gate is reproducible from the committed sources, not
an undeclared local build. The resolved binary path is recorded in `parity_report.json`.

The committed `renderer_provenance.json` pins the exact inputs (corpus SHA, font `.ttc`
+ extracted-OTF SHA, fpdf2/fontTools versions, page geometry) and the output PDF SHA, so
the build is auditable even though the heavy/private artifacts are not committed.

## Pins & environment

- Renderer: **fpdf2** (amends locked decision C2's "Chromium"; Codex-reviewed PASS — see
  `c2_amendment` in `renderer_provenance.json`). Byte-deterministic via fixed PDF creation
  date, stable producer/metadata, embedded font, single-line `pdf.text` placement.
- Ingest: `kb ingest --config` with `max_pages_per_chunk: 1, overlap_tokens: 0` →
  single-page, non-overlapping chunks (one utterance per chunk). `kb ingest --config` is
  implemented; the CLI `--help` claiming `CACG-CLI-004` is stale (verified empirically).
- libpdfium: system `/usr/lib/libpdfium.so` with `KB_SKIP_PDFIUM_HASH_CHECK=1`
  (same pin-gap disposition as the `cfa` corpus; see `out/cfa/pdfium_provenance.json`).
- Non-citable glyphs: emoji and a few symbols absent from Noto CJK are dropped on render;
  the parity gate excludes them (glyph-coverage check) and the AC-3 resolver fails closed
  on any quote containing one.
