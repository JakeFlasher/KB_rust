# PDF Fixtures Rebuild Ceremony

The PDF fixtures in `tests/parity_corpus/pdfs/` and `tests/fixtures/` are
committed as frozen artifacts. Their generator scripts have been moved to
`tools/python_legacy/pdf/` as part of the Python-tree retirement (DEC-6 Path B).

## When to rebuild

Only if the committed PDF fixtures need regeneration (e.g., after a
deliberate format change). This is an operator-initiated ceremony, not a
CI-gated step.

## How to rebuild

```bash
cd tools/python_legacy/pdf
python -m venv .venv
.venv/bin/pip install -e .
.venv/bin/python build_sample_pdf.py
.venv/bin/python build_pdf_negative_fixtures.py
.venv/bin/python build_qm_trim_fixtures.py
```

The generators write their output to the committed fixture paths under
`tests/parity_corpus/pdfs/` and `tests/fixtures/`. After regeneration,
verify the committed fixtures are byte-stable by diffing SHA-256 sums.

## Dependencies

The generators require `pikepdf`, `fpdf2`, and `pypdfium2` as declared
in `tools/python_legacy/pdf/pyproject.toml`. CI does not install these
packages; the rebuild is a manual ceremony only.
