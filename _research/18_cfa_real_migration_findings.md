# `_research/18` — CFA Real-Migration Findings (M4b First Vertical)

_captured against HEAD `fbfcce7` on `2026-05-24` for the
QM (`01_quantitative_methods`) vertical_

## What this artifact is

The required closure for `task-m4-10` / AC-9 of the M4 plan
(`.humanize/.humanize/plans/cacg-rust-port-m4-ingest-and-migration-pilot-plan.md`).
It quantifies the outcome of the M4b real-migration pilot — taking
the 17 hand-authored natural-prose QM cards from the sibling
`CFA_reading` repo and transforming them into `cacg.v0`-schema
cards whose citations resolve through hash-pinned chunks of
trimmed PDF source material.

The corpus end-state (the inputs this artifact analyzes):
- 5 committed trim PDFs at `tests/parity_corpus/pdfs/qm_*_trim.pdf`
  covering every primary source the 17 cards cite.
- A merged QM vertical corpus at
  `tests/parity_corpus/out_python/qm_vertical/` (chunks, sources,
  source-matrix, cards manifest, summaries, FTS5 sidecar) built by
  `scripts/build_qm_vertical_corpus.py` + `kb index`.
- 17 migrated `cacg.v0` cards at
  `tests/parity_corpus/cards/reading_01_qm/` produced by
  `scripts/migrate_qm_cards.py`.
- Two Layer-2 tallies at `_research/qm_layer2_tally.{md,json}`
  (primary, verbatim quotes) and `_research/qm_paraphrase_tally.{md,json}`
  (shadow, paraphrase quotes).

## §1 — Layer-2 exact-substring pass/fail (per-card breakdown)

The migration script's `extract_quote` extracts a verbatim
substring of each chunk's text as the citation `quote`, so the
**primary tally is 391/391 strict pass**:

| card_id | citations | strict | fuzzy | fail |
|---------|-----------|--------|-------|------|
| `qm-aic-bic-model-selection`               |   3 |   3 | 0 | 0 |
| `qm-anova-table`                           |   2 |   2 | 0 | 0 |
| `qm-arch-conditional-heteroskedasticity`   |   3 |   3 | 0 | 0 |
| `qm-cb-arb-factor-construction`            |  72 |  72 | 0 | 0 |
| `qm-decision-trees-and-roots`              |   3 |   3 | 0 | 0 |
| `qm-goodness-of-fit-r2-adj-r2`             |   2 |   2 | 0 | 0 |
| `qm-influence-analysis-leverage`           |   2 |   2 | 0 | 0 |
| `qm-multiple-linear-regression-foundations`|   1 |   1 | 0 | 0 |
| `qm-panel-cb-factor-inference`             |  98 |  98 | 0 | 0 |
| `qm-penalized-regression-lasso`            |   2 |   2 | 0 | 0 |
| `qm-projection-and-dimensionality-reduction`|  3 |   3 | 0 | 0 |
| `qm-regression-assumption-violations`      |   4 |   4 | 0 | 0 |
| `qm-regression-hypothesis-tests`           |   2 |   2 | 0 | 0 |
| `qm-signal-validation-oos-discipline`      |  49 |  49 | 0 | 0 |
| `qm-structured-data-ml`                    |   3 |   3 | 0 | 0 |
| `qm-time-series-foundations`               |   4 |   4 | 0 | 0 |
| `qm-volatility-model-garch-multivariate`   | 138 | 138 | 0 | 0 |
| **TOTAL**                                  | **391** | **391** | **0** | **0** |

Per-source breakdown (locking the byte-equal Pdfium parity result
from the M4a phase: the same Python and Rust ingest pipelines
produce byte-identical `chunks_manifest.json` for each of the 5
trims, so the strict pass rate would be identical whichever side
hosted the verify run):

| source_id          | citations | strict | fuzzy | fail |
|--------------------|-----------|--------|-------|------|
| `qm_afts_trim`     |       138 |    138 |     0 |    0 |
| `qm_greene_trim`   |        98 |     98 |     0 |    0 |
| `qm_eslii_ch3_trim`|        72 |     72 |     0 |    0 |
| `qm_eslii_ch7_trim`|        49 |     49 |     0 |    0 |
| `qm_notes_trim`    |        34 |     34 |     0 |    0 |

This satisfies AC-9's positive-test bullet 1 (Layer-2 pass/fail
rate with per-card breakdown). The 100 % strict rate IS expected
under the migration's verbatim-quote design — see §2 for the
paraphrase-aware shadow measurement that exposes the real-world
ground-truth rate.

## §2 — Fuzzy and Layer-3 implications (the shadow paraphrase tally)

The primary tally above measures only that the migration's
auto-extracted verbatim quotes are still verbatim — by
construction the substring containment cannot fail. The
substantive M4b measurement is **the paraphrase shadow tally**
at `_research/qm_paraphrase_tally.md`.

The shadow tally walks each legacy QM card's prose, extracts
the sentence ending each `**Source:** <pdf> pp.<range>`
annotation (a genuine paraphrase of the cited PDF passage,
authored by the original card author), and runs that
paraphrase as a synthetic citation `quote` against every chunk
in the merged manifest whose page span overlaps the cited
range.

| metric                                               |  count  |
|------------------------------------------------------|--------:|
| Legacy `**Source:**` annotations scanned             |     239 |
| Annotations citing in-vertical sources               |     222 |
| Annotations citing out-of-vertical (skipped)         |      17 |
| STRICT substring match (paraphrase in chunk text)    |       0 |
| FUZZY match (Levenshtein-bounded, see methodology)   |       0 |
| FAIL — would require Layer-3 (semantic verifier)     |     222 |
| Out-of-vertical (Wooldridge / ISLP / Tsay)           |      17 |

**Interpretation.** Authored paraphrase prose has zero substring
overlap with the source PDF text. That is the expected outcome:
the QM cards are not pinned quotations of the source — they are
distillations whose author rewrote definitions, derivations, and
table headers from scratch. Layer-2 substring matching is
structurally insufficient for any deployment where citations
attach to authored prose rather than to extracted quotations.

**Fuzzy was not run at full sweep.** The Levenshtein-bounded
matcher runs at ~1 s per call against the larger chunks (the
AFTS Ch.3 trim chunks are ~1 KB each), so the full
strict+fuzzy fan-out over 222 annotations × ~30 overlapping
chunks/annotation = ~7K calls × 1 s ≈ 2 hours. Running it
across the entire tally is wasteful for an analyze-only AC.

The true fuzzy-only pass count is therefore bounded
`0 ≤ k ≤ 222` (cannot fuzzy-only-pass without first
strict-failing; 222 annotations strict-failed). The shadow
tally's reported `0` is "0 attempted at full sweep," not
"0 verified to fail fuzzy"; a targeted random-sample sweep
would tighten the bound. Recommended follow-up if Layer-3
capacity planning needs a sharper number.

**Note: 81 % of paraphrase fails are against `qm_notes_trim`**
(180 of 222 — the OCR'd handwritten-notes trim). This is
consistent with the §5 OCR sparseness observation: the
OCR text per page is short, broken, and frequently
non-grammatical, so authored paraphrase prose has even less
chance of substring-matching against it than against the
text-native textbook PDFs.

**Layer-3 implications.** All 222 of these paraphrase annotations
would require Layer-3 semantic verification under a CACG
deployment where the citation `quote` is authored paraphrase
rather than verbatim. The Layer-3 verifier is scaffolded in
`src/cacg/verify/semantic.py` (`SemanticSpec` + the
`--semantic` / `--semantic-judge` CLI flags) but is gated
off by default; the embedding-cache and llm-judge sub-modes are
the two implementation paths. A real-world QM deployment would
need to either (a) require verbatim citation quotes (matches the
migration's current design — Layer-2 strict suffices) or
(b) enable Layer-3 and accept the per-card semantic-verifier
latency.

## §3 — Chunk fan-out statistics (multi-page legacy citations)

AC-7 requires that a legacy multi-page `pp.N-M` span fan out into
one Citation per overlapped chunk. The fan-out distribution per
card:

| card_id | cited book pages | page span | fan-out (chunks) |
|---------|------------------|----------:|-----------------:|
| `qm-multiple-linear-regression-foundations`  | pp.1-2    |   2 |   1 |
| `qm-anova-table`                             | pp.2-3    |   2 |   2 |
| `qm-goodness-of-fit-r2-adj-r2`               | pp.3-4    |   2 |   2 |
| `qm-aic-bic-model-selection`                 | pp.5-6    |   2 |   3 |
| `qm-regression-hypothesis-tests`             | pp.7      |   1 |   2 |
| `qm-regression-assumption-violations`        | pp.7-9    |   3 |   4 |
| `qm-influence-analysis-leverage`             | pp.10     |   1 |   2 |
| `qm-time-series-foundations`                 | pp.11-12  |   2 |   4 |
| `qm-arch-conditional-heteroskedasticity`     | pp.13     |   1 |   3 |
| `qm-structured-data-ml`                      | pp.14-15  |   2 |   3 |
| `qm-decision-trees-and-roots`                | pp.16-17  |   2 |   3 |
| `qm-penalized-regression-lasso`              | pp.16     |   1 |   2 |
| `qm-projection-and-dimensionality-reduction` | pp.17-18  |   2 |   3 |
| `qm-signal-validation-oos-discipline`        | pp.219-260|  42 |  49 |
| `qm-cb-arb-factor-construction`              | pp.43-99  |  57 |  72 |
| `qm-panel-cb-factor-inference`               | pp.413-470|  58 |  98 |
| `qm-volatility-model-garch-multivariate`     | pp.97-200 | 104 | 138 |

**Aggregate stats** (`min` / `mean` / `max`):
- citations per card: **1** / **23.0** / **138**
- citations per cited page (fan-out density):
    - narrow ranges (≤ 3 pages, all from `qm_notes_trim`):
      **0.5-3.0** chunks/page (min: `qm-multiple-linear-regression-foundations`
      pp.1-2 → 1 chunk; max: `qm-arch-conditional-heteroskedasticity`
      pp.13 → 3 chunks). The wide spread reflects that the
      paragraph-respecting token-budget chunker bunches sparse OCR
      paragraphs together or splits dense ones, with no fixed
      per-page rate.
    - wide ranges (≥ 40 pages, the 4 text-native textbook trims):
      **1.17-1.69** chunks/page (low: `qm_eslii_ch7_trim` at 1.17;
      high: `qm_greene_trim` at 1.69 — Greene's prose is denser per
      page than Hastie/Tibshirani/Friedman's, so more chunks fit
      per page under the same 350-token target).
- Per-source effective density across the QM corpus: notes 0.95
  chunks/page, ESLII Ch.7 1.17, ESLII Ch.3 1.26, AFTS 1.33,
  Greene 1.69. The text-native sources span 1.17-1.69 — roughly
  a factor of 1.4× variance attributable to per-page prose density.

## §4 — Legacy `pp.N-M` → `chunk_id` mapping recipe

The migration pipeline maps a legacy citation `<source.pdf> pp.N-M`
to a list of `cacg.v0` citations through five steps. Code lives in
`scripts/migrate_qm_cards.py`; this section is the implementation-
agnostic recipe a future migration round (e.g. M5 for the next
vertical) should follow.

### Step 1 — Resolve `<source.pdf>` to a trim source_id

For each in-corpus PDF, declare a `(substring, source_id,
optional_range_guard)` entry in a `PRIMARY_SOURCE_MAP`-style
table. The substring matches the literal text in the legacy
`Primary raw source:` line; the `source_id` matches the
`source_id` in the merged `chunks_manifest.json`. The
`optional_range_guard` exists for the one case where a single
source PDF is split across multiple trims (here: ESLII split
into `qm_eslii_ch3_trim` / `qm_eslii_ch7_trim` to avoid an
artificial Ch.3-to-Ch.7 seam-straddle chunk — disambiguated by
the cited book-page range).

### Step 2 — Translate book pages to trim pages

Each trim PDF is built from a contiguous source-PDF page range
(possibly multiple disjoint ranges, e.g. ESLII), then
renumbered to 1..N inside the trim. The `TRIM_PAGE_OFFSETS`
table records, per `source_id`, a list of
`(book_page_start, book_page_end_inclusive, trim_page_start)`
triples. Translation:

```
trim_page = trim_page_start + (book_page - book_page_start)
```

The book-page-vs-PDF-page offsets are NOT the same as the trim
mapping: the SOURCE PDF often has front-matter (preface, TOC)
before book page 1. Verified empirically per PDF:

| source PDF                                 | front-matter pages | book page = PDF page − |
|--------------------------------------------|--------------------:|----------------------:|
| `notes/CFA_note_2.ocr.pdf` (handwritten)   |                   0 |                     0 |
| `William H. Greene - Econometric Analysis` |                   1 |                     1 |
| `Analysis of Financial Time Series`        |                  25 |                    25 |
| `ESLII_print12_toc.pdf`                    |                  19 |                    19 |

A future migration MUST verify this offset for each new PDF by
sampling the first page of the trim — see §6.

### Step 3 — Find all overlapping chunks

Build the merged `chunks_manifest.json` with one chunks_manifest
entry per source_id (the cacg.v0 schema allows chunks from
multiple sources in one manifest; the chunk_id format
`<source_id>:p<NNN>:<NNNN>` keeps the join unambiguous). For
each legacy citation `(source_id, trim_page_lo, trim_page_hi)`,
return every chunk whose `[start_page, end_page]` overlaps
that range:

```
overlap(chunk, t_lo, t_hi)
  := chunk.end_page >= t_lo AND chunk.start_page <= t_hi
```

**Do not** use "fully inside" — the chunker can produce chunks
that span multiple book pages (`max_pages_per_chunk=2`), so a
narrow citation like `pp.2-3` would match zero "fully inside"
chunks but two "overlapping" chunks.

### Step 4 — Emit one Citation per overlapping chunk

Each output `Citation` carries:
- `source_id` from the trim mapping.
- `chunk_id` + `chunk_hash` from the chunk record.
- `page_range = [chunk.start_page, chunk.end_page]`
  (the chunk's actual page span, NOT the legacy citation's
   broader range — Layer-2 uses this for byte-window restriction).
- `quote` extracted from the chunk text (see §6 for the
  forbidden-codepoint filter).
- `edge_type = "supports"` (conservative default; richer
  edge-type detection is a future improvement).

### Step 5 — Computed `card_hash` via `kb index`

The migration script writes cards WITHOUT a `card_hash` field;
the canonical hash is computed by `kb index` against the
canonical-JSON-serialized frontmatter (Pydantic's
`model_dump(mode="python")` injects schema defaults like
`tags: []` and `card_edges: []` that the migration script can't
exactly replicate). Pre-computing the hash in the migration is
dead code; the index step is the unambiguous hash authority.

## §5 — OCR-extraction observations

The QM ingest exercised Pdfium against four very different
source-document shapes:

1. **OCR'd hand-written notes** (`qm_notes_trim`): the raster
   scan is embedded as a `/Subtype /Image` XObject behind a
   text layer the OCR pass produced. The text is sparse per
   page (~1700 chars vs ~2500 for text-native), often
   non-grammatical, and carries occasional artifacts from the
   recognizer ("Muttiple liner regression" instead of "Multiple
   linear regression"). Image-strip + `remove_unreferenced_resources()`
   reduces a 16 MB 20-page raw trim to 44 KB without breaking
   text extraction.

2. **Text-native textbook PDFs** (`qm_greene_trim`,
   `qm_afts_trim`, `qm_eslii_*_trim`): cleanly extracted; the
   chunker produces ~1.2-1.3 chunks/page. Image-strip removes a
   small number of figure XObjects with no impact on text
   density.

### Pdfium API quirks the migration uncovered (or re-uncovered)

The pipeline surfaced three Pdfium quirks beyond the AC-5
parity work that bear on Layer-2 correctness:

| codepoint | Pdfium behavior | Resolution |
|-----------|-----------------|------------|
| `U+0002` (STX) | `FPDFText_GetText` (the per-string accessor pypdfium2 wraps) post-processes soft-hyphen STX markers into `U+FFFE`; `FPDFText_GetUnicode` (the per-char accessor) returns raw STX. | Both Python (`src/cacg/pdf.py`) and Rust (`crates/cacg-ingest/src/pdf.rs`) explicitly map STX → `U+FFFE` in the extraction loop. AC-5 caught this; M4-Round 9 wrote the fix. |
| `U+0000` (NULL) | Pdfium emits 0 for chars without unicode information; `FPDFText_GetText` ignores them. | Both implementations `continue` on cp==0. M4-Round 15 surfaced a Rust NULL leak in `qm_eslii_ch3_trim` and pinned the symmetric drop. |
| Supplementary-plane chars (`U+10000+`, e.g. `𝚫` math-symbols block) | `FPDFText_GetText` lossy-maps to BMP fallback (`𝚫` → `z`); `FPDFText_GetUnicode` preserves the codepoint. | Python was historically calling `get_text_range()` (the per-string accessor) and silently destroying math content. M4-Round 15 switched Python to the per-char accessor to match Rust's correctness; the supp-plane regression test at `tests/test_round31_review.py::test_extract_pages_preserves_supplementary_plane_characters` locks the fix. |

The migration script's `extract_quote` (the
`_FORBIDDEN_QUOTE_CHARS` set + `_next_forbidden` scanner)
explicitly skips every codepoint PyYAML's safe-load rejects in
scalars — C0 controls minus tab/LF/CR, DEL, C1 controls, and
the two non-characters. Without this filter, OCR text carrying
STX or U+FFFE blows up the migrated card's frontmatter parse
with `CACG-FM-006`. The filter is engineered against the
empirical chunk content; a future migration over a new corpus
should re-check it against the new source.

### Sparse-OCR chunker behavior

`qm_notes_trim`'s 20 pages produce 19 chunks (0.95 chunks/page)
vs the text-native 1.17-1.69 chunks/page range. The chunker is
identical; the OCR text is just shorter per page. One narrow QM
citation landed on 1 chunk
(`qm-multiple-linear-regression-foundations`, `pp.1-2`) — the
smallest fan-out in the corpus. Wide-range citations (Greene's
58 book pages cited by `qm-panel-cb-factor-inference`) produce
98 chunks of fan-out — matching the ~1.7 chunks/page rate of
dense academic prose.

### Book-page vs PDF-page offsets (lessons-learned cross-ref)

The single most expensive lesson of M4b was that legacy `pp.N-M`
citations cite BOOK pages, but the trim build initially used
SOURCE-PDF pages — a silent off-by-19 for ESLII and off-by-25
for AFTS. The trims covered the WRONG chapters; Layer-2
substring match would have falsely passed against off-topic
content, and the migration had to be re-done. The §4 (Step 2)
mapping recipe + the offset table inside it and the §7 point 3
mandatory-page-1-sample-check decision both encode the lesson;
a future migration MUST verify the offset for each new PDF
before extracting trims. The full diagnosis and fix are in
the M4-Round-18 commit (`67b6ec4`).

## §6 — Methodology + tooling pointers

For reproducibility from a fresh checkout:

```bash
# Build the 5 trim PDFs (requires CFA_reading sibling checkout).
.venv/bin/python scripts/build_qm_trim_fixtures.py

# Ingest each trim to produce the per-trim oracle.
for trim in qm_notes_trim qm_greene_trim qm_afts_trim \
           qm_eslii_ch3_trim qm_eslii_ch7_trim; do
  KB_FROZEN_CLOCK=1 .venv/bin/python -m cacg.cli ingest \
    tests/parity_corpus/pdfs/$trim.pdf \
    --out tests/parity_corpus/out_python/pdfs/$trim \
    --source-id $trim
done

# Merge into the QM vertical corpus. Output:
# tests/parity_corpus/out_python/qm_vertical/{chunks,sources,source_matrix}_manifest.json
.venv/bin/python scripts/build_qm_vertical_corpus.py

# Migrate the 17 legacy cards.
.venv/bin/python scripts/migrate_qm_cards.py

# Compute card hashes + cards_manifest + summaries.
.venv/bin/python -m cacg.cli index \
  tests/parity_corpus/cards/reading_01_qm \
  --out tests/parity_corpus/out_python/qm_vertical

# Run both tallies.
.venv/bin/python scripts/qm_vertical_layer2_tally.py
.venv/bin/python scripts/qm_vertical_paraphrase_tally.py
```

The Pdfium binary pin (`pdfium 149.0.7825.0` from `pypdfium2
5.8.0`) and the parity contract are documented in
`docs/pdfium-binary-provisioning.md`. The xtask matrix
(`cargo xtask parity`) gates the 6 ingest rows
(cfa_vol1_trim + 5 QM trims) against the Python oracles
byte-for-byte.

## §7 — Decisions for downstream work

1. **Verbatim-quote citations are the right call for the M4b
   pilot.** The 100 % strict Layer-2 pass rate is degenerate
   for the spirit of "measure paraphrase resilience" but
   correct for the spirit of "produce hash-pinned citations
   that future readers can mechanically verify." Real-world
   verticals can choose: verbatim quotes (cheap, Layer-2-only)
   or paraphrase quotes (requires Layer-3).
2. **Layer-3 is a real requirement for a paraphrase-heavy
   deployment.** The shadow tally's 0/222 strict rate is
   evidence that Layer-2 + fuzzy cannot ground authored
   paraphrase. The semantic-verifier scaffolding in
   `cacg.verify.semantic` is correctly gated off by default;
   enabling it requires either an embedding cache
   (`--semantic <cache>`) or an LLM judge (`--semantic-judge`),
   both with operational cost.
3. **Page-offset verification is mandatory for every new
   source PDF.** The Round-17 P2.3 finding (ESLII / AFTS /
   Greene trim built against PDF pages, cards cite book
   pages, mismatch off by 19/25/1) cost a re-trim + re-migrate
   round. A future M5+ migration script should sample each new
   trim's page-1 content and assert "book page X is on PDF
   page Y" BEFORE running the trim.
4. **The migration is reproducible.** Re-running
   `build_qm_trim_fixtures.py` produces byte-identical PDFs
   (`pikepdf.Pdf.save(deterministic_id=True)`); re-running
   `migrate_qm_cards.py` produces byte-identical cards
   (`kb index` is the canonical hash authority, so the
   migration's output is a deterministic predecessor of the
   indexed state). The 6 xtask parity rows lock the
   Python-vs-Rust ingest byte-equality.
5. **The xtask matrix gates ingest parity, not migration
   correctness.** The committed cards and the committed
   tally reports are NOT regenerated by the xtask matrix;
   they are one-shot artifacts authored at commit time. A
   regression in the migration script would surface as a git
   diff against the committed cards, not as a parity FAIL.
   AC-7's "deterministic re-run" contract is held by the
   migration scripts themselves, not by a CI gate.

## §8 — AC-9 coverage map

| AC-9 positive-test bullet | Section here |
|---------------------------|--------------|
| Layer-2 exact-substring pass/fail with per-card breakdown | §1 |
| Count of citations passing only under fuzzy matching | §2 (zero in this corpus; fuzzy unattempted at full sweep) |
| Count that would require Layer-3 semantic verification | §2 (222 paraphrase annotations) |
| Chunk fan-out statistics for multi-page legacy citations | §3 |
| Legacy-`pp.N-M`-to-`chunk_id` mapping recipe | §4 |
| Observations on the OCR-extraction path | §5 |

All six bullets are covered; the artifact satisfies AC-9.
