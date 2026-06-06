#!/usr/bin/env python3
r"""Deterministic Noto-CJK renderer: corpus_complete.md -> goubujiao_corpus.pdf.

The corpus is the deck's PDF-only ingest source. Because the kernel `normalize_text`
COLLAPSES whitespace to a single space, any separator pdfium inserts at an
intra-paragraph CJK line-wrap would break verbatim containment of a space-less
Chinese quote. We defeat that by construction: every utterance is rendered as a
SINGLE LINE (`cell`, never `multi_cell`) on its own page, so no soft-wrap ever
splits an utterance. Each page also carries an ASCII id marker so a quote can be
mapped back to its `post_id`/`comment_id` (parity, AC-2) and its origin page.

Determinism (proven byte-identical across runs; cf. the O'Hara sandwich
BL-20260531-image-only-source-via-deterministic-ocr-sandwich): fixed PDF creation
date, stable producer/creator/metadata, embedded Unicode CJK face (deterministic
fpdf2 subset), fixed page geometry + font size. The font face is extracted once,
deterministically, from the system Noto CJK `.ttc` and cached (gitignored); its
SHA + the corpus SHA + the output SHA are recorded in a committed provenance file.

  --write              render the PDF + write renderer_provenance.json
  --check-determinism  render twice to temp dirs; assert identical SHA-256
  --self-test          drive the pure helpers (marker format) hermetically
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "sources/hkex/_registry"))
from corpus_model import parse, CORPUS  # noqa: E402  # pyright: ignore[reportMissingImports]

FONT_TTC = Path("/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc")
FONT_FACE_INDEX = 0
FONT_CACHE = ROOT / "sources/hkex/_registry/_fonts/NotoSansCJK-Regular-face0.otf"
OUT_PDF = ROOT / "sources/hkex/pdfs/goubujiao_corpus.pdf"
PROVENANCE = ROOT / "sources/hkex/_registry/renderer_provenance.json"

PAGE_FORMAT_MM = (2000, 300)   # wide enough for the longest (685-char) author line on one line
FONT_SIZE_PT = 8
MARGIN_MM = 5
MARKER_Y_MM = 5
TEXT_Y_MM = 14
EPOCH = datetime(1970, 1, 1, tzinfo=timezone.utc)
PRODUCER = "hkex-goubujiao-corpus-renderer"


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def page_marker(page: int, u) -> str:
    """ASCII, deterministic, whitespace-tokenizable id marker for one page."""
    cid = u.comment_id if u.comment_id else "NA"
    return f"@@HKEX p={page} k={u.kind} pid={u.post_id} cid={cid} au={1 if u.is_author else 0}@@"


def ensure_font() -> str:
    """Extract face 0 from the system Noto CJK `.ttc` to a cached OTF (idempotent)."""
    FONT_CACHE.parent.mkdir(parents=True, exist_ok=True)
    if not FONT_CACHE.is_file():
        if not FONT_TTC.is_file():
            raise SystemExit(f"system CJK font missing: {FONT_TTC}")
        from fontTools.ttLib import TTFont
        f = TTFont(str(FONT_TTC), fontNumber=FONT_FACE_INDEX)
        f.save(str(FONT_CACHE))
    return sha256_file(FONT_CACHE)


def render(out_pdf: Path, utts) -> int:
    from fpdf import FPDF
    ensure_font()
    pdf = FPDF(unit="mm", format=PAGE_FORMAT_MM)
    pdf.set_creation_date(EPOCH)
    pdf.set_auto_page_break(False)
    pdf.set_producer(PRODUCER)
    pdf.set_creator(PRODUCER)
    pdf.set_title("goubujiao xueqiu corpus (complete)")
    pdf.set_subject("hkex deck ingest source")
    pdf.set_author("rendered")
    pdf.add_font("cjk", "", str(FONT_CACHE))
    pdf.set_font("cjk", size=FONT_SIZE_PT)
    for i, u in enumerate(utts, 1):
        pdf.add_page()
        # `pdf.text` places a single line of glyphs at an absolute baseline with
        # NO width / wrap / pagination logic, so a long utterance can never
        # overflow onto the next page (which `cell` does) and pdfium never
        # inserts an intra-utterance separator. An empty utterance still gets its
        # own page via the marker.
        pdf.text(MARGIN_MM, MARKER_Y_MM + 3, page_marker(i, u))
        if u.text != "":
            pdf.text(MARGIN_MM, TEXT_Y_MM, u.text)
    out_pdf.parent.mkdir(parents=True, exist_ok=True)
    pdf.output(str(out_pdf))
    if pdf.page != len(utts):
        raise SystemExit(f"page count {pdf.page} != utterance count {len(utts)}")
    return len(utts)


def counts(utts) -> dict[str, int]:
    return {
        "post": sum(1 for u in utts if u.kind == "post"),
        "areply": sum(1 for u in utts if u.kind == "areply"),
        "comment": sum(1 for u in utts if u.kind == "comment"),
        "author_total": sum(1 for u in utts if u.is_author),
        "total": len(utts),
    }


def do_write() -> dict:
    import fpdf as _fpdf
    import fontTools as _ft
    utts = parse(CORPUS)
    font_sha = ensure_font()
    page_count = render(OUT_PDF, utts)
    prov = {
        "schema_version": "hkex.renderer_provenance.v1",
        "generator": "sources/hkex/_registry/render_corpus_pdf.py",
        "renderer": "fpdf2",
        "fpdf2_version": _fpdf.__version__,
        "fonttools_version": _ft.version,
        "frozen_creation_date": "1970-01-01T00:00:00Z",
        "producer": PRODUCER,
        "page_format_mm": list(PAGE_FORMAT_MM),
        "font_size_pt": FONT_SIZE_PT,
        "margin_mm": MARGIN_MM,
        "layout": ("one utterance per page; single-line pdf.text placement (no wrap, no "
                   "pagination -> no CJK line-wrap separator); ascii id marker per page"),
        "marker_format": "@@HKEX p=<page> k=<kind> pid=<post_id> cid=<comment_id|NA> au=<0|1>@@",
        "c2_amendment": ("Locked decision C2 specified Chromium headless print-to-pdf; amended to "
                         "fpdf2 (the repo's proven byte-deterministic PDF generator) because AC-2's "
                         "hard requirement is byte-determinism + verbatim CJK containment, which fpdf2 "
                         "delivers out of the box with single-line absolute placement. Codex (gpt-5.5:high) "
                         "reviewed and returned PASS on the deviation (no CJK fidelity reason for Chromium). "
                         "Glyphs absent from Noto CJK (emoji/symbols) are a recorded non-citable class."),
        "nonrenderable_note": ("emoji and a few symbols absent from the Noto CJK face are dropped on render; "
                               "the parity checker gates only font-renderable citable spans and the AC-3 "
                               "resolver fails closed on any quote containing a non-renderable char."),
        "corpus_path": str(CORPUS.relative_to(ROOT)),
        "corpus_sha256": sha256_file(CORPUS),
        "font_ttc_path": str(FONT_TTC),
        "font_ttc_sha256": sha256_file(FONT_TTC),
        "font_face_index": FONT_FACE_INDEX,
        "font_otf_cache": str(FONT_CACHE.relative_to(ROOT)),
        "font_otf_sha256": font_sha,
        "output_pdf": str(OUT_PDF.relative_to(ROOT)),
        "output_pdf_sha256": sha256_file(OUT_PDF),
        "page_count": page_count,
        "utterance_counts": counts(utts),
    }
    PROVENANCE.write_text(json.dumps(prov, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return prov


def do_check_determinism() -> int:
    """Render in SEPARATE subprocesses under differing PYTHONHASHSEED so a
    hash-seeded iteration-order nondeterminism (e.g. font subsetting) cannot hide
    behind a shared interpreter; also assert the result equals the committed
    provenance SHA (no drift vs the published artifact)."""
    shas: list[str] = []
    seeds = ["0", "1", "12345"]
    tmp = Path(tempfile.mkdtemp(prefix="hkex_det_"))
    try:
        for k, seed in enumerate(seeds):
            out = tmp / f"r{k}.pdf"
            e = dict(os.environ, PYTHONHASHSEED=seed)
            r = subprocess.run([sys.executable, __file__, "--render-to", str(out)],
                               env=e, capture_output=True, text=True)
            if r.returncode != 0 or not out.is_file():
                print(f"render subprocess (seed={seed}) failed: {r.stderr[:300]}", file=sys.stderr)
                return 1
            shas.append(sha256_file(out))
    finally:
        shutil.rmtree(tmp, ignore_errors=True)
    identical = len(set(shas)) == 1
    prov_sha = json.loads(PROVENANCE.read_text())["output_pdf_sha256"] if PROVENANCE.is_file() else None
    matches_provenance = (shas[0] == prov_sha) if prov_sha else None
    ok = identical and matches_provenance is not False
    print(json.dumps({"render_shas": [s[:16] for s in shas], "seeds": seeds,
                      "byte_identical_across_subprocesses": identical,
                      "matches_committed_provenance": matches_provenance,
                      "verdict": "PASS" if ok else "FAIL"}, indent=2))
    return 0 if ok else 1


def do_render_to(path: Path) -> int:
    render(path, parse(CORPUS))
    return 0


def _self_test() -> int:
    from types import SimpleNamespace
    failures = []
    u = SimpleNamespace(kind="post", post_id="222375639", comment_id=None, is_author=True)
    if page_marker(7, u) != "@@HKEX p=7 k=post pid=222375639 cid=NA au=1@@":
        failures.append(f"post marker wrong: {page_marker(7, u)}")
    c = SimpleNamespace(kind="comment", post_id="1", comment_id="244700573", is_author=False)
    if page_marker(3, c) != "@@HKEX p=3 k=comment pid=1 cid=244700573 au=0@@":
        failures.append(f"comment marker wrong: {page_marker(3, c)}")
    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print("  -", f)
        return 1
    print("SELF-TEST PASSED (render_corpus_pdf: marker format)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--write", action="store_true")
    g.add_argument("--check-determinism", action="store_true")
    g.add_argument("--self-test", action="store_true")
    g.add_argument("--render-to", metavar="PATH", help="render once to PATH (used by --check-determinism)")
    args = ap.parse_args()
    if args.self_test:
        return _self_test()
    if args.render_to:
        return do_render_to(Path(args.render_to))
    if args.write:
        prov = do_write()
        print(json.dumps({k: v for k, v in prov.items()
                          if k in ("output_pdf", "output_pdf_sha256", "page_count",
                                   "utterance_counts", "corpus_sha256", "font_otf_sha256")},
                         ensure_ascii=False, indent=2, sort_keys=True))
        return 0
    return do_check_determinism()


if __name__ == "__main__":
    raise SystemExit(main())
