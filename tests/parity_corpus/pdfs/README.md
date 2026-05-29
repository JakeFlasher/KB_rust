# Parity corpus PDFs

## Committed PDFs (small + portable)

- `sample.pdf` — 3239 bytes, fpdf2 deterministic fixture used by the
  canonical valid + adversarial cards. Built via
  `tests/fixtures/build_sample_pdf.py` and copied verbatim.
- `cfa_vol1_trim.pdf` — pages 1-30 extracted from the CFA_reading Vol.1 PDF.
  Built ONCE by `scripts/build_parity_corpus.py` using `pypdfium2.PdfDocument.import_pages`,
  committed verbatim at `tests/fixtures/cfa_vol1_trim.pdf`, and copied into the corpus
  on every build. pypdfium2's `save()` is not byte-deterministic across runs
  (trailer IDs and creation timestamps drift), so the commit-once-reuse-forever
  flow is the only way to keep the resulting `sources_manifest.json` byte-stable.
- `qm_notes_trim.pdf` / `qm_greene_trim.pdf` / `qm_afts_trim.pdf` /
  `qm_eslii_trim.pdf` — the four PDF trims that together carry the
  primary-source citations of all 17 cards under
  `01_quantitative_methods/`:
    - `qm_notes_trim.pdf` (44 KB, 20 pages, 20 images stripped):
      pages 1-20 of `notes/CFA_note_2.ocr.pdf` — primary source for
      13 cards. The OCR'd notes are image-heavy; stripping the
      `/Subtype /Image` XObjects leaves the OCR text layer the
      chunker consumes intact. SHA-256:
      `1ba174a43cda30b28ab475793e8eeaee6572f55ee7dac714a20c579e9bce5fca`.
    - `qm_greene_trim.pdf` (1.15 MB, 58 pages, 0 images stripped):
      pages 413-470 of Greene's `Econometric Analysis Global Edition`
      — primary source for `qm-panel-cb-factor-inference.md`.
      SHA-256:
      `b24f8679d7ad1b94bbfab2c2a5763cb6b9bbb720e128a1ccedd1012325a516a2`.
    - `qm_afts_trim.pdf` (803 KB, 104 pages, 47 images stripped):
      pages 97-200 of Tsay's `Analysis of Financial Time Series` —
      primary source for `qm-volatility-model-garch-multivariate.md`.
      SHA-256:
      `42f663c9abda46fbd38c4d27c0f6764635c88a57bc548f78e576a19730d30517`.
    - `qm_eslii_trim.pdf` (1.18 MB, 99 pages, 1 image stripped):
      pages 43-99 + 219-260 of Hastie/Tibshirani/Friedman's
      `Elements of Statistical Learning II` — primary source for
      `qm-cb-arb-factor-construction.md` (pp.43-99) and
      `qm-signal-validation-oos-discipline.md` (pp.219-260).
      SHA-256:
      `fed2847242426d6dee5818417a2495d621ef9657ab9af6ac3e76c0338c4c527e`.

  All four are built by `scripts/build_qm_trim_fixtures.py` (pikepdf,
  deterministic via `save(deterministic_id=True)`). The Python
  oracles are authored by `KB_FROZEN_CLOCK=1 .venv/bin/python -m
  cacg.cli ingest` under PY-IS-ORACLE (`_research/09` DEC-1); Rust
  verifies byte-equal via the four `kb_ingest_parity_qm_*_trim` rows
  in the xtask matrix.

  ### Round-15 Python-extractor fix

  Round 15 surfaced a Pdfium API divergence: the per-string
  accessor `FPDFText_GetText` lossy-maps supplementary-plane chars
  (`U+10000+` including the entire mathematical-symbols block) to a
  BMP fallback at the C++ level — e.g. `𝚫` (MATHEMATICAL BOLD
  CAPITAL DELTA, `U+1D6AB`) is returned as plain `z`. The
  per-character accessor `FPDFText_GetUnicode(handle, char_index)`
  preserves the codepoint correctly. The Rust port has always used
  the per-char API (AC-5 BYTE-EQUAL, Round 9). Python's
  `cacg.pdf.extract_pages_normalized` historically used the
  per-string API and silently destroyed math content; Round 15
  switches it to the per-char loop too (see `src/cacg/pdf.py`
  module docstring). The cfa_vol1_trim / qm_notes_trim oracles are
  unchanged (those PDFs contain no supplementary-plane chars); the
  three math-heavy trims (Greene / AFTS / ESLII) carry the
  corrected extraction.

## Operator-local-only PDFs (NOT committed; size constraints)

Per the existing `tests/fixtures/cfa_smoke/.gitignore` policy ('Never commit
the 110 MB PDF'), large CFA_reading PDFs are NOT mirrored into the parity
corpus directory. The build script reads them from sibling-repo paths only
when those paths are present locally; on a fresh CI runner without the sibling
repo, the build skips CFA ingestion entirely and the committed trim PDF stays
the byte-equal contract baseline:

- The 21 MB full Vol.1 PDF — the trim extraction above replaces it as the
  committed byte-equal parity input; the full PDF is operator-local-only.
- The 114 MB OCR `CFA_note_2.ocr.pdf` exceeds GitHub's 100 MB hard limit and
  is permanently outside the merge-blocking byte-equal parity gate. Operators
  with sibling-repo access can ingest it locally for triage but the result is
  not gated.

## DEC-5 carry-forward

The PROPOSED-DEFAULT in `_research/09_dec_proposed_defaults.md` adopted both
Vol.1 + OCR PDFs as parity inputs. Round 3 narrows the byte-equal-gated subset
to (sample.pdf + cfa_vol1_trim.pdf) for portability; the OCR PDF moves to the
operator-local non-gated tier. User overrides may broaden the gated subset, but
doing so requires Git LFS, a separate PDF-only repo, or a per-PDF page-trim
policy (whatever stays under 50 MB total).
