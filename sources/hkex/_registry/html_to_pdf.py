#!/usr/bin/env python3
r"""Deterministic HTML -> text-layer PDF recipe for the ``hkex`` HTML grounding sources.

The free IRD / IFEC grounding sources are HTML pages; ``kb ingest`` is PDF-only, so each is
snapshotted to a DETERMINISTIC text-layer PDF (same byte-reproducible fpdf2 path as the
corpus renderer: fixed creation date, stable producer, embedded Noto CJK face). HTML is
reduced to text deterministically (``html.parser`` with block-structure separators and
entity decoding; ``<script>``/``<style>`` dropped), then rendered as wrapped paragraphs.
English official text wraps safely (the kernel normalize collapses a wrap newline to the
same single space the quote carries), so a verbatim quote binds across line/page breaks.

A committed provenance record pins the inputs (html sha, font sha, fpdf2 version) and the
output PDF sha so a re-snapshot is checkable. AC-6 applies this to the real snapshots and
records each source's PDF sha.

  --html PATH --out PDF   snapshot one HTML file (writes PDF + a .provenance.json sidecar)
  --self-test             render a synthetic page twice (identical SHA) + prove a quote ingests
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
from html.parser import HTMLParser
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "sources/hkex/_registry"))
from render_corpus_pdf import ensure_font, FONT_CACHE, FONT_TTC, EPOCH  # noqa: E402  # pyright: ignore[reportMissingImports]
from check_corpus_parity import resolve_kb  # noqa: E402  # pyright: ignore[reportMissingImports]

PRODUCER = "hkex-html-snapshot"
PAGE_FORMAT = "A4"
FONT_SIZE_PT = 10
MARGIN_MM = 15

_BLOCK = {"p", "div", "br", "li", "tr", "h1", "h2", "h3", "h4", "h5", "h6",
          "table", "ul", "ol", "section", "article", "header", "footer", "blockquote"}
_SKIP = {"script", "style", "head", "title"}


class _TextExtractor(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.parts: list[str] = []
        self.skip = 0

    def handle_starttag(self, tag, attrs):
        if tag in _SKIP:
            self.skip += 1
        elif tag in _BLOCK and self.parts and not self.parts[-1].endswith("\n"):
            self.parts.append("\n")
        elif tag in ("td", "th"):
            self.parts.append("\t")

    def handle_endtag(self, tag):
        if tag in _SKIP and self.skip:
            self.skip -= 1
        elif tag in _BLOCK and self.parts and not self.parts[-1].endswith("\n"):
            self.parts.append("\n")

    def handle_data(self, data):
        if not self.skip:
            self.parts.append(data)


def html_to_text(html_str: str) -> str:
    p = _TextExtractor()
    p.feed(html_str)
    lines = [re.sub(r"[ \t\f\v]+", " ", ln).strip() for ln in "".join(p.parts).split("\n")]
    return "\n".join(ln for ln in lines if ln)


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def render_text_pdf(text: str, out_pdf: Path) -> None:
    from fpdf import FPDF
    from fpdf.enums import WrapMode, XPos, YPos
    ensure_font()
    pdf = FPDF(unit="mm", format=PAGE_FORMAT)
    pdf.set_creation_date(EPOCH)
    pdf.set_producer(PRODUCER)
    pdf.set_creator(PRODUCER)
    pdf.set_title("hkex html snapshot")
    pdf.set_author("rendered")
    pdf.set_margins(MARGIN_MM, MARGIN_MM, MARGIN_MM)
    pdf.set_auto_page_break(True, margin=MARGIN_MM)
    pdf.add_font("cjk", "", str(FONT_CACHE))
    pdf.set_font("cjk", size=FONT_SIZE_PT)
    pdf.add_page()
    for line in text.split("\n"):
        # new_x=LMARGIN returns the cursor to the left margin each block (a bare
        # multi_cell(w=0) otherwise leaves x at the right edge -> 0 width next call);
        # wrapmode=CHAR breaks an over-long token instead of raising.
        pdf.multi_cell(w=0, h=5, text=line, new_x=XPos.LMARGIN, new_y=YPos.NEXT, wrapmode=WrapMode.CHAR)
    out_pdf.parent.mkdir(parents=True, exist_ok=True)
    pdf.output(str(out_pdf))


def snapshot(html_path: Path, out_pdf: Path) -> dict:
    import fpdf as _fpdf
    text = html_to_text(html_path.read_text(encoding="utf-8"))
    render_text_pdf(text, out_pdf)
    prov = {
        "schema_version": "hkex.html_snapshot_provenance.v1",
        "generator": "sources/hkex/_registry/html_to_pdf.py",
        "renderer": "fpdf2",
        "fpdf2_version": _fpdf.__version__,
        "extraction": "html.parser convert_charrefs; block-separated; script/style dropped; intra-line ws collapsed",
        "frozen_creation_date": "1970-01-01T00:00:00Z",
        "producer": PRODUCER,
        "page_format": PAGE_FORMAT,
        "font_size_pt": FONT_SIZE_PT,
        "html_path": str(html_path),
        "html_sha256": sha256_file(html_path),
        "font_ttc_path": str(FONT_TTC),
        "font_otf_cache": str(FONT_CACHE.relative_to(ROOT)),
        "font_otf_sha256": sha256_file(FONT_CACHE),
        "output_pdf": str(out_pdf),
        "output_pdf_sha256": sha256_file(out_pdf),
    }
    (out_pdf.with_suffix(out_pdf.suffix + ".provenance.json")).write_text(
        json.dumps(prov, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return prov


def self_test() -> int:
    failures: list[str] = []
    sample_html = (
        "<html><head><style>.x{}</style><title>ignored</title></head><body>"
        "<h1>IRD &amp; Stamp Duty</h1>"
        "<p>The rate is 0.1% per side plus HK$5 fixed.</p>"
        "<table><tr><td>H-share</td><td>10% PRC withholding</td></tr>"
        "<tr><td>red-chip</td><td>exempt</td></tr></table>"
        "<script>var z='dropme';</script>"
        "</body></html>"
    )
    extracted = html_to_text(sample_html)
    if "0.1% per side" not in extracted or "dropme" in extracted or "ignored" in extracted:
        failures.append(f"html_to_text wrong: {extracted!r}")

    with tempfile.TemporaryDirectory() as d:
        hp = Path(d) / "s.html"
        hp.write_text(sample_html, encoding="utf-8")
        a, b = Path(d) / "a.pdf", Path(d) / "b.pdf"
        render_text_pdf(extracted, a)
        render_text_pdf(extracted, b)
        if sha256_file(a) != sha256_file(b):
            failures.append("two renders not byte-identical")
        # the rendered PDF must ingest and contain a verbatim quote
        kb = resolve_kb()
        env = dict(os.environ, KB_FROZEN_CLOCK="1", KB_SKIP_PDFIUM_HASH_CHECK="1",
                   LD_LIBRARY_PATH=":".join(["/usr/lib", os.environ.get("LD_LIBRARY_PATH", "")]).strip(":"))
        work = Path(d) / "ing"
        r = subprocess.run([str(kb), "ingest", str(a), "--out", str(work), "--source-id", "html_probe"],
                           capture_output=True, text=True, env=env)
        if r.returncode != 0:
            failures.append(f"ingest failed: {(r.stderr or r.stdout)[:200]}")
        else:
            full = " ".join(c["text"] for c in json.loads((work / "chunks_manifest.json").read_text())["chunks"])
            full = re.sub(r"\s+", " ", full)
            for q in ("The rate is 0.1% per side", "10% PRC withholding"):
                if q not in full:
                    failures.append(f"quote not contained after ingest: {q!r}")

    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print("  -", f)
        return 1
    print("SELF-TEST PASSED (html_to_pdf: deterministic render + entity/block extraction + ingest containment)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("--html")
    ap.add_argument("--out")
    args = ap.parse_args()
    if args.self_test:
        return self_test()
    if not (args.html and args.out):
        ap.error("provide --html and --out, or --self-test")
    prov = snapshot(Path(args.html), Path(args.out))
    print(json.dumps({k: prov[k] for k in ("output_pdf", "output_pdf_sha256", "html_sha256")},
                     ensure_ascii=False, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
