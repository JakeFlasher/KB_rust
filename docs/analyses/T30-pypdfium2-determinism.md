# T30 -- pypdfium2 Determinism Across Versions

> Codex analyze (gpt-5.5:high) memo prepared for the CACG MVP plan. Date: 2026-05-18.
> Source script: see Codex skill artifact in the local loop state directory.

CACG should treat PDF text extraction as versioned input, not a stable semantic operation. The recommendation below is conservative first-principles engineering, with support from pypdfium2's own changelog.

## Sources of Nondeterminism

1. **PDFium ABI / binary drift.**
   pypdfium2 wheels vendor PDFium via `pdfium-binaries`, and pypdfium2 releases regularly update that binary. The 5.8.0 changelog says it updated `pdfium-binaries` from `7802` to `7825`; 5.7.1 fixed a PDFium text/char-box regression; 4.30.1 was yanked for a PDFium text extraction regression. These are direct signals that PDFium changes can affect extracted text or text geometry. See https://pypdfium2-team.github.io/pypdfium2/changelog.html.

2. **Python-side pypdfium2 behavior changes.**
   pypdfium2 helper defaults and text APIs can change across minor releases. Examples: 5.8.0 changed font-name decoding default behavior to `errors="replace"`; 5.0.0 changed `PdfTextPage.get_text_range()` behavior and recommends `get_text_bounded()` for full Unicode support. Even if CACG only calls one helper today, minor releases can change decoding, fallback, warnings, or object/text helpers.

3. **OS locale and font fallback.**
   Text extraction is mostly driven by embedded PDF data, but missing fonts, broken encodings, or system-font fallback can interact with PDFium behavior. pypdfium2 5.7.0 added `PdfSysfontBase` as a first step toward inspecting or altering system font use, which implies font resolution is a real moving part.

4. **PDF-specific instability cases.**
   The same visual PDF can produce different normalized text across installs when extraction depends on heuristics: untagged PDFs, PDFs without reliable `/ToUnicode` maps, embedded subset fonts, ligatures, vertical writing, malformed encodings, XFA/forms, annotations, generated table layouts, and print-oriented PDF/X-1a files. PDF/A with embedded fonts and good Unicode maps should be more stable; PDF/X-1a often optimizes for print fidelity, not text semantics.

## Mitigations

- Replace the lower bound with an exact version for MVP determinism.
- A weaker but acceptable maintenance compromise is `pypdfium2==5.8.*`, but patch releases may still change PDFium or helper behavior.
- `pypdfium2>=5.8,<5.9` is too loose for chunk-hash stability; it allows patch drift.
- Hash-pinned wheels via `pip --require-hashes` would give stronger supply-chain reproducibility, but is probably over-cautious for the MVP unless CACG ships a lockfile.
- Document an upgrade procedure: bump the pin, run the AC-1 double-ingest test, run a fixture corpus diff, record old/new `PYPDFIUM_INFO` and `PDFIUM_INFO`, then intentionally regenerate chunk hashes/manifests.

## Recommended MVP Pin

Use this exact dependency string in `pyproject.toml`:

```toml
"pypdfium2==5.8.0"
```

This matches the current Python 3.14 venv: `PYPDFIUM_INFO=5.8.0`, `PDFIUM_INFO=149.0.7825.0`.

## Warn or Refuse to Chunk

CACG should warn or refuse for:

- PDFs with no extractable text after normalization.
- Scanned-only/image-only PDFs unless OCR is explicitly added.
- Encrypted/password-protected PDFs.
- PDFs with XFA/forms where visible values may not be captured consistently.
- PDFs with extraction errors, malformed encodings, or excessive replacement characters.
- PDFs whose page count opens but text extraction returns empty for most pages.

## 2-Line Docs Summary

CACG chunk hashes are deterministic only for the same source bytes, CACG version, normalization code, and pinned pypdfium2/PDFium version. When the pypdfium2 pin changes, rerun the fixture diff suite and regenerate chunk manifests intentionally if extracted text changes.
