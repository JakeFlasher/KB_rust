#!/usr/bin/env python3
"""Build all four committable trimmed QM-source PDFs (task-m4-7
+ M4-Round-15).

Each of the 17 cards under `01_quantitative_methods/` has a
`Primary raw source: <pdf> pp.<range>` line in its frontmatter.
The cards split across four source PDFs:

  - `notes/CFA_note_2.ocr.pdf` — 13 cards, pages 1-20
  - `01_Quantitative_Methods/William H. Greene ... .pdf` — 1 card,
    pages 413-470
  - `01_Quantitative_Methods/Analysis of Financial Time Series.pdf`
    — 1 card, pages 97-200
  - `01_Quantitative_Methods/ESLII_print12_toc.pdf` — 2 cards,
    pages 43-99 + 219-260 (union)

The raw source PDFs are 4-115 MB each and ship as sibling-repo
assets, not committed to this repo. This script builds four
small, committable trims (one per source PDF) into
`tests/parity_corpus/pdfs/`. Each trim is produced by:

  1. Importing the cited pages into a fresh pikepdf doc.
  2. Stripping every `/Subtype /Image` from each page's
     `/Resources/XObject` so the result is text-only.
  3. `pikepdf.Pdf.remove_unreferenced_resources()` to GC the
     orphaned image streams.
  4. `save(deterministic_id=True)` so re-running produces a
     byte-identical output (the default `/ID` is random).

Determinism is load-bearing for the byte-equal parity contract
(`kb_ingest_parity_qm_*_trim` rows in the xtask matrix). The
script prints each output SHA-256 so the developer can pin it
in `tests/parity_corpus/pdfs/README.md`.

Usage:
    .venv/bin/python scripts/build_qm_trim_fixtures.py            # all four
    .venv/bin/python scripts/build_qm_trim_fixtures.py qm_notes   # one only
"""

import dataclasses
import hashlib
import pathlib
import sys

try:
    import pikepdf  # type: ignore
except ImportError as exc:
    print(f"error: pikepdf import failed: {exc}", file=sys.stderr)
    print(
        "Hint: this script must run inside the project's .venv "
        "(`.venv/bin/python scripts/build_qm_trim_fixtures.py`). "
        "pikepdf is declared as a dev-dep in pyproject.toml; "
        "if your venv is stale, run `.venv/bin/pip install -e .[dev]`.",
        file=sys.stderr,
    )
    sys.exit(1)


WORKSPACE_ROOT = pathlib.Path(__file__).resolve().parents[2]
CFA_READING_ROOT = pathlib.Path("/home/jakeshea/CFA_reading")
OUTPUT_DIR = WORKSPACE_ROOT / "tests/parity_corpus/pdfs"


@dataclasses.dataclass(frozen=True)
class TrimConfig:
    """One PDF-trim recipe. `name` is the stable identifier used by
    the output filename (`<name>.pdf`), the `source_id` of the
    ingest run, and the xtask parity row label
    (`kb_ingest_parity_<name>`). `pages_one_indexed` is the union
    of every page the in-scope cards cite from `source_pdf`."""

    name: str
    source_pdf: pathlib.Path
    pages_one_indexed: list[int]
    cards_covered: int

    @property
    def output_pdf(self) -> pathlib.Path:
        return OUTPUT_DIR / f"{self.name}.pdf"


TRIMS: list[TrimConfig] = [
    TrimConfig(
        name="qm_notes_trim",
        source_pdf=CFA_READING_ROOT / "notes/CFA_note_2.ocr.pdf",
        # OCR'd hand-written notes: book pages == PDF pages.
        pages_one_indexed=list(range(1, 21)),
        cards_covered=13,
    ),
    TrimConfig(
        name="qm_greene_trim",
        source_pdf=CFA_READING_ROOT
        / "01_Quantitative_Methods"
        / "William H. Greene - Econometric Analysis Global Edition (2019, Pearson-prentice Hall) - libgen.li.pdf",
        # qm-panel-cb-factor-inference.md cites BOOK pp.413-470
        # (Ch.11 "Models for Panel Data"). Greene's PDF has one
        # front-matter page before book numbering starts, so
        # PDF page N = book page (N - 1). To cover book pp.413-470
        # we extract PDF pp.414-471. Verified: PDF p414 begins
        # "413  11  MODELS FOR PANEL DATA"; PDF p471 ends book
        # page 470. The earlier off-by-23 in the round-15 build
        # used PDF pp.413-470 (= book pp.412-469) — close enough
        # that the previous Layer-2 verify did pass on substring
        # matches, but the cited content was off by one book
        # page.
        pages_one_indexed=list(range(414, 472)),
        cards_covered=1,
    ),
    TrimConfig(
        name="qm_afts_trim",
        source_pdf=CFA_READING_ROOT
        / "01_Quantitative_Methods"
        / "Analysis of Financial Time Series.pdf",
        # qm-volatility-model-garch-multivariate.md cites BOOK
        # pp.97-200 (Ch.3 "Conditional Heteroscedastic Models" —
        # GARCH, ARCH, EGARCH content). AFTS PDF has 25 pages of
        # front-matter (title, preface, TOC) before book p.1, so
        # PDF page N = book page (N - 25). To cover book pp.97-200
        # we extract PDF pp.122-225. Verified: PDF p122 begins
        # "CHAPTER 3  Conditional Heteroscedastic Models". The
        # earlier round-15 build used PDF pp.97-200 (= book pp.72-175)
        # which was the wrong content (Ch.2 linear time series, not
        # Ch.3 GARCH); migrated card would Layer-2 fail.
        pages_one_indexed=list(range(122, 226)),
        cards_covered=1,
    ),
    # ESLII has 19 pages of front-matter (title, TOC, preface
    # leaves) before book p.1, so PDF page N = book page (N - 19).
    # The two ESLII-cited cards live in disjoint chapters
    # (Ch.3 ridge/LASSO and Ch.7 cross-validation) so we publish
    # two separate trims rather than one trim with an artificial
    # seam. A seamed trim would produce one chunker chunk that
    # concatenates the end of Ch.3 with the start of Ch.7 — text
    # whose citation would be nonsense.
    TrimConfig(
        name="qm_eslii_ch3_trim",
        source_pdf=CFA_READING_ROOT
        / "01_Quantitative_Methods"
        / "ESLII_print12_toc.pdf",
        # qm-cb-arb-factor-construction.md cites BOOK pp.43-99
        # (Ch.3 "Linear Methods for Regression" — ridge / LASSO).
        # Verified: PDF p62 begins "This is page 43 ... 3 Linear
        # Methods for Regression".
        pages_one_indexed=list(range(62, 119)),
        cards_covered=1,
    ),
    TrimConfig(
        name="qm_eslii_ch7_trim",
        source_pdf=CFA_READING_ROOT
        / "01_Quantitative_Methods"
        / "ESLII_print12_toc.pdf",
        # qm-signal-validation-oos-discipline.md cites BOOK pp.219-260
        # (Ch.7 "Model Assessment and Selection" — cross-validation).
        # Verified: PDF p238 begins "This is page 219 ... 7 Model
        # Assessment and Selection".
        pages_one_indexed=list(range(238, 280)),
        cards_covered=1,
    ),
]


def strip_image_xobjects(page) -> int:
    """Remove every `/XObject` of `/Subtype /Image` from `page`.
    Returns the count removed. The text layer is preserved."""
    if "/Resources" not in page:
        return 0
    resources = page["/Resources"]
    if "/XObject" not in resources:
        return 0
    xobj = resources["/XObject"]
    to_remove = []
    for name, obj in xobj.items():
        try:
            if obj.get("/Subtype") == "/Image":
                to_remove.append(name)
        except (KeyError, AttributeError, pikepdf.PdfError):
            # A non-stream / non-dict XObject (rare) — leave it
            # alone. Narrower than `except Exception` so a typo'd
            # attribute lookup in a future refactor surfaces.
            pass
    for name in to_remove:
        del xobj[name]
    return len(to_remove)


def build_one(trim: TrimConfig) -> bool:
    """Build one trim. Returns True on success."""
    if not trim.source_pdf.is_file():
        print(
            f"error [{trim.name}]: source PDF not found at {trim.source_pdf}",
            file=sys.stderr,
        )
        print(
            "This script requires the sibling CFA_reading repo to be "
            "checked out at /home/jakeshea/CFA_reading/. The source "
            "PDFs are not committed to this repo.",
            file=sys.stderr,
        )
        return False

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    src = pikepdf.open(str(trim.source_pdf))
    try:
        src_count = len(src.pages)
        for p in trim.pages_one_indexed:
            if p < 1 or p > src_count:
                print(
                    f"error [{trim.name}]: page {p} out of range "
                    f"(source has {src_count} pages)",
                    file=sys.stderr,
                )
                return False

        out = pikepdf.Pdf.new()
        for p in trim.pages_one_indexed:
            out.pages.append(src.pages[p - 1])

        total_removed = 0
        for page in out.pages:
            total_removed += strip_image_xobjects(page)

        out.remove_unreferenced_resources()
        # `deterministic_id=True` is load-bearing: the default
        # injects a random trailer `/ID` and the SHA-256 drifts
        # every run. See M4-Round-13 review P1.
        out.save(str(trim.output_pdf), deterministic_id=True)
        out.close()
    finally:
        src.close()

    sha = hashlib.sha256(trim.output_pdf.read_bytes()).hexdigest()
    size = trim.output_pdf.stat().st_size
    print(f"=== {trim.name} ===")
    print(f"  source:          {trim.source_pdf.name}")
    print(f"  output:          {trim.output_pdf}")
    print(f"  size:            {size:,} bytes")
    print(f"  sha256:          {sha}")
    print(f"  pages:           {len(trim.pages_one_indexed)}")
    print(f"  images stripped: {total_removed}")
    print(f"  cards covered:   {trim.cards_covered}")
    return True


def main() -> int:
    selected = sys.argv[1:] if len(sys.argv) > 1 else None
    if selected:
        unknown = [s for s in selected if s not in {t.name for t in TRIMS}]
        if unknown:
            print(
                f"error: unknown trim name(s): {unknown}. "
                f"Known: {[t.name for t in TRIMS]}",
                file=sys.stderr,
            )
            return 1
        trims = [t for t in TRIMS if t.name in selected]
    else:
        trims = TRIMS

    all_ok = True
    for trim in trims:
        if not build_one(trim):
            all_ok = False
    return 0 if all_ok else 1


if __name__ == "__main__":
    sys.exit(main())
