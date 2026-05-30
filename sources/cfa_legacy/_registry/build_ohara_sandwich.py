#!/usr/bin/env python3
"""Build + register the O'Hara 1995 text-layer "sandwich" source.

O'Hara 1995 (`mt_ohara_1995_market_microstructure_theory_en`) is an image-only
scan, so it cannot enter the Pdfium trust path directly. Its pages were OCR'd
once (PaddleOCR PP-OCRv5 mobile English) to a per-page text file. This module
renders that pinned OCR text into a DETERMINISTIC text-layer PDF -- one PDF page
per OCR page -- which `kb ingest` then parses like any other source. Keeping the
Pdfium kernel unchanged (no custom OCR ingest path) is the whole point.

Reproducibility is pinned by committed, in-repo inputs: the vendored OCR text
(`ohara_ocr_en.txt`) and a recorded font + generator version. The generated PDF
is byte-stable across runs (fixed PDF dates, fixed producer, deterministic font
subset), and the resolution artifact (`v1_ohara_resolution.json`) records every
determinism input plus the build's own SHA so a rebuild can be checked.

  --write   generate the sandwich PDF, register the source (ingest_plan row +
            source-matrix authorization), and write the build half of the
            resolution artifact.
  --check   bind every O'Hara skeleton citation to a chunk of a supplied chunks
            manifest and run the REAL `kb verify` on each (the kernel is the
            normalization authority), then record the machine-check half of the
            resolution artifact (pass count + any +/-1 page-window corrections).
  --self-test  drive the pure helpers hermetically.
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
import unicodedata
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[3]
REGISTRY = ROOT / "sources/cfa_legacy/_registry"
sys.path.insert(0, str(REGISTRY))
from register_migration_sources import (  # noqa: E402  # pyright: ignore[reportMissingImports]
    ingest_row, merge_allowed, serialize_compact, serialize_pretty, serialize_plan,
    INGEST_PLAN, MATRIX_COMPACT, MATRIX_PRETTY, MATRIX_OUT,
)

SOURCE_ID = "mt_ohara_1995_market_microstructure_theory_en"
READING_ID = "14_microstructure_and_trading"
OCR_VENDORED = REGISTRY / "ohara_ocr_en.txt"
SANDWICH_PDF = ROOT / f"sources/cfa_legacy/pdfs/{READING_ID}/{SOURCE_ID}.pdf"
DECISION_PATH = REGISTRY / "v1_ohara_resolution.json"
KB_BINARY = ROOT / "target/debug/kb"
FONT_PATH = Path(os.environ.get("KB_OHARA_FONT", "/usr/share/fonts/TTF/DejaVuSans.ttf"))
DEFERRED = Path(os.environ.get("KB_DEFERRED_ORIGIN", "/home/jakeshea/CFA_reading/deferred_books"))
SKELETON_DIR = DEFERRED / "14_Microstructure_and_Trading/_card_skeletons" / READING_ID

# The book is a single continuously-paginated scan; one rendered PDF page per OCR
# page keeps the skeletons' OCR-page citations aligned. The page format is huge
# so no OCR page can overflow (page geometry does not affect extracted text).
PAGE_FORMAT_MM = (300, 2000)
FONT_SIZE_PT = 9
FROZEN_TIMESTAMP = "1970-01-01T00:00:00Z"


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for block in iter(lambda: f.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def atomic_write_bytes(path: Path, data: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, tmp = tempfile.mkstemp(prefix=f".{path.name}.", suffix=".tmp", dir=path.parent)
    try:
        with os.fdopen(fd, "wb") as f:
            f.write(data); f.flush(); os.fsync(f.fileno())
        os.replace(tmp, path)
    finally:
        if os.path.exists(tmp):
            os.unlink(tmp)


# --------------------------------------------------------------------------- #
# Pure helpers (driven hermetically by --self-test).
# --------------------------------------------------------------------------- #

def split_ocr_pages(text: str) -> dict[int, str]:
    """Split the assembled OCR text into ``{page_number: page_text}`` on the
    ``===== PDF page N =====`` markers."""
    pages: dict[int, str] = {}
    cur: int | None = None
    buf: list[str] = []
    for line in text.splitlines():
        m = re.match(r"===== PDF page (\d+) =====", line)
        if m:
            if cur is not None:
                pages[cur] = "\n".join(buf)
            cur = int(m.group(1)); buf = []
        else:
            buf.append(line)
    if cur is not None:
        pages[cur] = "\n".join(buf)
    return pages


def normalize_quote(s: str) -> str:
    """A pragmatic normalization used ONLY to PROPOSE a candidate chunk; the real
    authority for a match is `kb verify` (run in --check)."""
    s = unicodedata.normalize("NFC", s)
    for a, b in [("’", "'"), ("‘", "'"), ("“", '"'), ("”", '"'),
                 ("–", "-"), ("—", "-"), ("‐", "-"), ("­", "")]:
        s = s.replace(a, b)
    return re.sub(r"\s+", " ", s).strip()


# --------------------------------------------------------------------------- #
# Sandwich generation.
# --------------------------------------------------------------------------- #

def generate_sandwich(pages: dict[int, str], out_path: Path) -> int:
    from fpdf import FPDF

    n = max(pages)
    pdf = FPDF(unit="mm", format=PAGE_FORMAT_MM)
    pdf.set_creation_date(datetime(1970, 1, 1, tzinfo=timezone.utc))
    pdf.set_auto_page_break(False)
    pdf.add_font("dv", "", str(FONT_PATH))
    pdf.set_font("dv", size=FONT_SIZE_PT)
    for i in range(1, n + 1):
        pdf.add_page()
        pdf.set_xy(5, 5)
        pdf.multi_cell(w=PAGE_FORMAT_MM[0] - 10, h=4, text=pages.get(i, ""))
    pdf.set_producer("cfa-legacy-ohara-sandwich")
    out_path.parent.mkdir(parents=True, exist_ok=True)
    pdf.output(str(out_path))
    if pdf.page != n:
        raise SystemExit(f"page overflow: wrote {pdf.page} pages, expected {n}")
    return n


def load_ohara_citations() -> list[dict[str, Any]]:
    cites: list[dict[str, Any]] = []
    for f in sorted(SKELETON_DIR.glob("*.md")):
        txt = f.read_text(encoding="utf-8")
        m = re.search(r"\ncitations:\n(.*?)\n---", txt, re.S)
        if not m:
            continue
        for item in re.split(r"\n  - source_id:", m.group(1)):
            if "mt_ohara" not in item:
                continue
            pr = re.search(r"page_range:\s*\[([0-9]+)\s*,\s*([0-9]+)\]", item) or re.search(r"page_range:\s*\[([0-9]+)\]", item)
            q = re.search(r'quote:\s*"(.*?)"\s*\n', item, re.S)
            et = re.search(r'edge_type:\s*"(\w+)"', item)
            cites.append({
                "card": f.name,
                "page_range": [int(x) for x in pr.groups()] if pr else None,
                "quote": q.group(1) if q else None,
                "edge_type": et.group(1) if et else None,
            })
    return cites


# --------------------------------------------------------------------------- #
# Modes.
# --------------------------------------------------------------------------- #

def do_write() -> dict[str, Any]:
    if not OCR_VENDORED.is_file():
        raise SystemExit(f"vendored OCR text missing: {OCR_VENDORED}")
    if not FONT_PATH.is_file():
        raise SystemExit(f"font missing: {FONT_PATH} (set KB_OHARA_FONT)")
    pages = split_ocr_pages(OCR_VENDORED.read_text(encoding="utf-8"))
    page_count = generate_sandwich(pages, SANDWICH_PDF)
    sandwich_sha = sha256_file(SANDWICH_PDF)

    # register: ingest_plan row + source-matrix authorization (idempotent).
    plan = json.loads(INGEST_PLAN.read_text(encoding="utf-8"))
    if SOURCE_ID not in {str(r["source_id"]) for r in plan}:
        plan.append(ingest_row(READING_ID, SOURCE_ID))
        atomic_write_bytes(INGEST_PLAN, serialize_plan(plan))
    matrix = json.loads(MATRIX_COMPACT.read_text(encoding="utf-8"))
    matrix["allowed"] = merge_allowed(matrix.get("allowed", {}), {READING_ID: [SOURCE_ID]})
    atomic_write_bytes(MATRIX_COMPACT, serialize_compact(matrix))
    atomic_write_bytes(MATRIX_PRETTY, serialize_pretty(matrix))
    atomic_write_bytes(MATRIX_OUT, serialize_pretty(matrix))

    decision = {
        "schema_version": "cfa_legacy.v1_ohara_resolution.v1",
        "generated_at_utc": FROZEN_TIMESTAMP,
        "chosen_path": "sandwich_textlayer",
        "source_id": SOURCE_ID,
        "reading_id": READING_ID,
        "rationale": (
            "Sandbox-first (recorded decision): the image-only O'Hara scan is rendered into a deterministic "
            "text-layer PDF from the pinned OCR text and ingested as a normal source; the Pdfium trust "
            "path is unchanged. Source registered with the SANDWICH artifact's own SHA (its _sources.json "
            "value is the original image PDF and intentionally does not match)."
        ),
        "determinism_inputs": {
            "ocr_text_vendored": str(OCR_VENDORED.relative_to(ROOT)),
            "ocr_text_sha256": sha256_file(OCR_VENDORED),
            "ocr_toolchain": "paddleocr PP-OCRv5 mobile english (upstream, in CFA_reading origin)",
            "font_path": str(FONT_PATH),
            "font_sha256": sha256_file(FONT_PATH),
            "generator": "fpdf2",
            "generator_version": __import__("fpdf").__version__,
            "page_format_mm": list(PAGE_FORMAT_MM),
            "font_size_pt": FONT_SIZE_PT,
        },
        "sandwich_pdf": str(SANDWICH_PDF.relative_to(ROOT)),
        "sandwich_sha256": sandwich_sha,
        "page_count": page_count,
        "citation_verification": None,  # filled by --check
    }
    # preserve a prior --check result if re-writing the build half
    if DECISION_PATH.is_file():
        prior = json.loads(DECISION_PATH.read_text(encoding="utf-8"))
        if prior.get("citation_verification") and prior.get("sandwich_sha256") == sandwich_sha:
            decision["citation_verification"] = prior["citation_verification"]
    atomic_write_bytes(DECISION_PATH, json.dumps(decision, ensure_ascii=False, indent=2, sort_keys=True).encode() + b"\n")
    return {"page_count": page_count, "sandwich_sha256": sandwich_sha,
            "registered_source": SOURCE_ID, "decision": str(DECISION_PATH.relative_to(ROOT))}


def do_check(chunks_manifest: Path) -> dict[str, Any]:
    if not DECISION_PATH.is_file():
        raise SystemExit("run --write first (no resolution artifact)")
    decision = json.loads(DECISION_PATH.read_text(encoding="utf-8"))
    chunks = json.loads(chunks_manifest.read_text(encoding="utf-8"))["chunks"]
    nchunks = [(c, normalize_quote(c["text"])) for c in chunks if c["source_id"] == SOURCE_ID]
    if not nchunks:
        raise SystemExit(f"no {SOURCE_ID} chunks in {chunks_manifest}")
    cites = load_ohara_citations()
    matrix = REGISTRY / "_ohara_check_matrix.json"
    atomic_write_bytes(matrix, json.dumps(
        {"allowed": {READING_ID: [SOURCE_ID]}, "schema_version": "cacg.v0"}).encode())
    env = dict(os.environ, KB_FROZEN_CLOCK="1",
               LD_LIBRARY_PATH=":".join(["/usr/lib", os.environ.get("LD_LIBRARY_PATH", "")]).strip(":"))

    results = []
    passes = corrections = 0
    work = Path(tempfile.mkdtemp(prefix="ohara_check_"))
    try:
        for i, c in enumerate(cites):
            lo, hi = c["page_range"]; q = normalize_quote(c["quote"])
            cand = [ck for ck, t in nchunks if not (ck["end_page"] < lo or ck["start_page"] > hi) and q in t]
            if not cand:
                results.append({"card": c["card"], "status": "no_bind", "page_range": c["page_range"]}); continue
            ck = cand[0]
            bound_pr = [max(lo, ck["start_page"]), min(hi, ck["end_page"])]
            corrected = bound_pr != c["page_range"]
            card = {"schema_version": "cacg.v0", "id": f"mt-ohara-check-{i}", "title": "O'Hara check",
                    "reading_id": READING_ID, "summary": "x" * 90, "tags": ["microstructure"],
                    "citations": [{"source_id": SOURCE_ID, "chunk_id": ck["chunk_id"], "chunk_hash": ck["chunk_hash"],
                                   "page_range": bound_pr, "quote": c["quote"], "edge_type": c["edge_type"]}]}
            cpath = work / f"card_{i}.md"
            cpath.write_text("---\n" + json.dumps(card) + "\n---\n\nbody\n", encoding="utf-8")
            # frontmatter must be YAML; json is valid YAML for these scalar/list values
            r = subprocess.run([str(KB_BINARY), "verify", str(cpath),
                                "--chunks-manifest", str(chunks_manifest), "--source-matrix", str(matrix),
                                "--journal", str(work / f"j{i}.jsonl")], capture_output=True, text=True, env=env)
            ok = r.returncode == 0
            passes += ok; corrections += (ok and corrected)
            results.append({"card": c["card"], "status": "verify_pass" if ok else "verify_fail",
                            "chunk_id": ck["chunk_id"], "authored_page_range": c["page_range"],
                            "bound_page_range": bound_pr, "page_window_corrected": corrected,
                            "error": None if ok else (r.stdout.strip() or r.stderr.strip())[:160]})
    finally:
        import shutil
        shutil.rmtree(work, ignore_errors=True)
        matrix.unlink(missing_ok=True)

    decision["citation_verification"] = {
        "chunks_manifest_sha256": sha256_file(chunks_manifest),
        "total_citations": len(cites),
        "verify_pass": passes,
        "page_window_corrections": corrections,
        "all_verify": passes == len(cites) and len(cites) > 0,
        "results": results,
    }
    atomic_write_bytes(DECISION_PATH, json.dumps(decision, ensure_ascii=False, indent=2, sort_keys=True).encode() + b"\n")
    return {"total": len(cites), "verify_pass": passes, "page_window_corrections": corrections,
            "all_verify": decision["citation_verification"]["all_verify"]}


def self_test() -> int:
    failures: list[str] = []
    pages = split_ocr_pages("pre\n===== PDF page 1 =====\nalpha\nbeta\n===== PDF page 2 =====\ngamma")
    if pages != {1: "alpha\nbeta", 2: "gamma"}:
        failures.append(f"split_ocr_pages wrong: {pages}")
    if normalize_quote("the  “quote”—here") != 'the "quote"-here':
        failures.append(f"normalize_quote wrong: {normalize_quote('the  “quote”—here')!r}")
    if failures:
        print("SELF-TEST FAILED:"); [print("  -", f) for f in failures]; return 1
    print("SELF-TEST PASSED (split_ocr_pages + normalize_quote)")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--write", action="store_true", help="generate + register + write build provenance")
    parser.add_argument("--check", metavar="CHUNKS_MANIFEST", help="machine-check the 14 citations verify")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    if args.write:
        print(json.dumps(do_write(), ensure_ascii=False, indent=2, sort_keys=True))
        return 0
    if args.check:
        result = do_check(Path(args.check))
        print(json.dumps(result, ensure_ascii=False, sort_keys=True))
        return 0 if result["all_verify"] else 1
    parser.error("one of --self-test / --write / --check is required")


if __name__ == "__main__":
    raise SystemExit(main())
