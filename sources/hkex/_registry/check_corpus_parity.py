#!/usr/bin/env python3
r"""Corpus -> PDF parity proof (AC-2 positive test #2).

Ingests the rendered `goubujiao_corpus.pdf` with the kernel `kb ingest` (the same
libpdfium every chunk_hash depends on), reconstructs each page's extracted text by
splitting on its `@@HKEX p=N … au=…@@` marker, normalizes it with the fixture-verified
port of the kernel `normalize_text`, and proves that EVERY ★AUTHOR utterance carrying a
citable span is:

  - CONTAINED in the normalized extracted text of its origin page, and
  - MAPPED to the expected `post_id`/`comment_id` via that page's `au=1` marker.

This is a FULL gate over all author utterances (not a sample); a stratified coverage
breakdown is reported for transparency. Hardening (per the Codex AC-2 review):
  * the rendered PDF's SHA-256 must equal the committed renderer provenance (no stale PDF);
  * every page must carry exactly one marker (no extraction loss);
  * quote spans are drawn only from the citable class AND only from codepoints the
    embedded font can render (glyph-coverage fail-closed: emoji / glyph-absent chars are a
    documented non-citable class, recorded, never gated);
  * a length-matched fabricated control must NOT be found (fail-closed);
  * comment pages must carry `au=0` (so the author/comment `au` discrimination is real).

  --check [--keep-ingest]   ingest + run the parity proof; writes parity_report.json
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
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "sources/hkex/_registry"))
from corpus_model import parse, CORPUS  # noqa: E402  # pyright: ignore[reportMissingImports]
from cacg_normalize import normalize_text  # noqa: E402  # pyright: ignore[reportMissingImports]

PDF = ROOT / "sources/hkex/pdfs/goubujiao_corpus.pdf"
PROVENANCE = ROOT / "sources/hkex/_registry/renderer_provenance.json"
REPORT = ROOT / "sources/hkex/_registry/parity_report.json"

_MARKER = re.compile(r"@@HKEX p=(\d+) k=(\w+) pid=(\d+) cid=(\S+) au=([01])@@")

# Disjoint traditional-only vs simplified-only sentinel characters (no overlap).
_TRAD = set("個務實調價淨會證據濟產廣經當銀環縣輸華買賣與點對營應夢憶")
_SIMP = set("个务实调价净会证据济产广经当银环县输华买卖与点对营应梦忆")
assert not (_TRAD & _SIMP), "trad/simp sentinel sets must be disjoint"

_EXTRA_PUNCT = "，。！？；：、「」『』（）《》【】…—·“”‘’％¥"


def resolve_kb() -> Path:
    """Resolve the `kb` binary from committed-build conventions, fail-closed.

    `KB_BIN` (if set) wins after an executable-file check; otherwise prefer
    `target/debug/kb` because AC-1's `cargo build --workspace` guarantees it, and
    fall back to `target/release/kb`. With neither present the gate refuses rather
    than depend on an undeclared local build artifact."""
    env = os.environ.get("KB_BIN")
    if env:
        p = Path(env)
        if not (p.is_file() and os.access(p, os.X_OK)):
            raise SystemExit(f"FAIL: KB_BIN={env} is not an executable file")
        return p
    for cand in (ROOT / "target/debug/kb", ROOT / "target/release/kb"):
        if cand.is_file() and os.access(cand, os.X_OK):
            return cand
    raise SystemExit("FAIL: no kb binary found; run `cargo build --workspace` "
                     "(produces target/debug/kb) or set KB_BIN=/path/to/kb")


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def font_codepoints(otf_path: Path) -> set[int]:
    from fontTools.ttLib import TTFont
    f = TTFont(str(otf_path))
    cps: set[int] = set()
    for table in f["cmap"].tables:
        cps.update(table.cmap.keys())
    return cps


def is_citable_char(ch: str, renderable: set[int]) -> bool:
    """A char the deck may cite: in the embedded font's cmap AND in the citable
    class (CJK ideographs, CJK/ASCII punctuation, ASCII letters/digits/symbols)."""
    o = ord(ch)
    if o not in renderable:
        return False  # font cannot render it -> dropped on render -> never citable
    if ch.isascii():
        return ch.isprintable()
    if 0x4E00 <= o <= 0x9FFF or 0x3400 <= o <= 0x4DBF or 0xF900 <= o <= 0xFAFF:
        return True
    if 0x3000 <= o <= 0x303F or 0xFF00 <= o <= 0xFFEF:
        return True
    return ch in _EXTRA_PUNCT


def clean_span(text: str, renderable: set[int], want: int = 12) -> str | None:
    """Longest run of citable+renderable chars; return a >=`want`-char interior slice."""
    runs: list[str] = []
    cur: list[str] = []
    for ch in text:
        if is_citable_char(ch, renderable):
            cur.append(ch)
        elif cur:
            runs.append("".join(cur)); cur = []
    if cur:
        runs.append("".join(cur))
    best = max(runs, key=len) if runs else ""
    if len(best) < want:
        return None
    start = (len(best) - want) // 2
    return best[start:start + want]


def reconstruct_pages(chunks: list[dict]) -> tuple[dict[int, str], list[str]]:
    """page_no -> text by splitting each chunk on its markers. Also returns
    structural errors: any page whose marker count != 1."""
    pages: dict[int, str] = {}
    marker_count: dict[int, int] = {}
    for c in chunks:
        parts = _MARKER.split(c["text"])
        i = 1
        while i < len(parts):
            page = int(parts[i])
            marker = f"@@HKEX p={parts[i]} k={parts[i+1]} pid={parts[i+2]} cid={parts[i+3]} au={parts[i+4]}@@"
            seg = parts[i + 5] if i + 5 < len(parts) else ""
            marker_count[page] = marker_count.get(page, 0) + 1
            if page not in pages:
                pages[page] = marker + seg
            i += 6
    errors = [f"page {p} has {n} markers (expected 1)" for p, n in sorted(marker_count.items()) if n != 1]
    return pages, errors


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--check", action="store_true", required=True)
    ap.add_argument("--keep-ingest", action="store_true")
    args = ap.parse_args()

    if not PDF.is_file():
        print(f"FAIL: rendered PDF missing ({PDF}); run render_corpus_pdf.py --write", file=sys.stderr)
        return 1
    if not PROVENANCE.is_file():
        print("FAIL: renderer_provenance.json missing; run render_corpus_pdf.py --write", file=sys.stderr)
        return 1
    prov = json.loads(PROVENANCE.read_text())
    kb = resolve_kb()

    # Hardening: never run parity against a stale PDF.
    live_sha = sha256_file(PDF)
    if live_sha != prov["output_pdf_sha256"]:
        print(f"FAIL: PDF sha {live_sha[:16]} != provenance {prov['output_pdf_sha256'][:16]} "
              "(re-run render_corpus_pdf.py --write)", file=sys.stderr)
        return 1

    renderable = font_codepoints(ROOT / prov["font_otf_cache"])

    env = dict(os.environ, KB_FROZEN_CLOCK="1", KB_SKIP_PDFIUM_HASH_CHECK="1",
               LD_LIBRARY_PATH=":".join(["/usr/lib", os.environ.get("LD_LIBRARY_PATH", "")]).strip(":"))
    work = Path(tempfile.mkdtemp(prefix="hkex_parity_"))
    try:
        # Ingest with the CJK chunk config (single-page, non-overlapping chunks) — the
        # same config the AC-3 spike uses — so each page maps to exactly one chunk and
        # carries exactly one marker. (`kb ingest --config` is implemented; the stale
        # CLI --help claiming CACG-CLI-004 is wrong, verified empirically.)
        cfg = work / "cjk_chunk.yaml"
        cfg.write_text("chunking:\n  target_tokens: 100000\n  overlap_tokens: 0\n  max_pages_per_chunk: 1\n")
        t0 = time.time()
        r = subprocess.run([str(kb), "ingest", str(PDF), "--config", str(cfg), "--out", str(work),
                            "--source-id", "goubujiao_xueqiu_corpus"],
                           capture_output=True, text=True, env=env)
        ingest_secs = round(time.time() - t0, 1)
        if r.returncode != 0:
            print(f"FAIL: kb ingest rc={r.returncode}\n{(r.stderr or r.stdout)[:500]}", file=sys.stderr)
            return 1
        chunks = json.loads((work / "chunks_manifest.json").read_text())["chunks"]
        pages, struct_errors = reconstruct_pages(chunks)

        utts = parse(CORPUS)
        if len(pages) != len(utts):
            struct_errors.append(f"reconstructed {len(pages)} pages != {len(utts)} utterances")

        # FULL author-utterance gate.
        gated = failed = skipped_short = flagged_glyph = 0
        strata = {k: 0 for k in ("trad", "simp", "ticker", "percent", "numeral", "long", "short", "edge")}
        edge_tokens = ("行駛", "行使", "石鼓", "實物", "除淨", "半裸")
        misses: list[dict] = []
        for page, u in enumerate(utts, 1):
            if not u.is_author:
                continue
            span = clean_span(u.text, renderable)
            if span is None:
                if any(ord(ch) not in renderable for ch in u.text):
                    flagged_glyph += 1
                else:
                    skipped_short += 1
                continue
            gated += 1
            ptext = pages.get(page, "")
            npage = normalize_text(ptext)
            contained = normalize_text(span) in npage
            m = _MARKER.search(npage)
            id_ok = bool(m) and m.group(3) == u.post_id and m.group(5) == "1" and \
                m.group(4) == (u.comment_id or "NA")
            if not (contained and id_ok):
                failed += 1
                if len(misses) < 20:
                    misses.append({"page": page, "post_id": u.post_id, "comment_id": u.comment_id,
                                   "quote": span, "contained": contained, "id_mapped": id_ok})
            ts = set(u.text)
            if ts & _TRAD: strata["trad"] += 1
            if ts & _SIMP: strata["simp"] += 1
            if re.search(r"\$[^$]+\$", u.text): strata["ticker"] += 1
            if "%" in u.text or "％" in u.text: strata["percent"] += 1
            if any(ch.isdigit() for ch in u.text): strata["numeral"] += 1
            if any(t in u.text for t in edge_tokens): strata["edge"] += 1
            strata["long" if len(u.text) >= 120 else "short"] += 1

        # au discrimination: comment pages must be au=0 (proves the author flag is real).
        au_bad = 0
        sampled_comment_pages = 0
        for page, u in enumerate(utts, 1):
            if u.is_author or sampled_comment_pages >= 200:
                continue
            sampled_comment_pages += 1
            m = _MARKER.search(pages.get(page, ""))
            if not m or m.group(5) != "0":
                au_bad += 1

        # control chars across all chunks
        BAD = {"\x02", "￾", "�"}
        ctrl = sorted({hex(ord(ch)) for c in chunks for ch in c["text"]
                       if (ord(ch) < 0x20 and ch not in "\n\t\r") or ch in BAD})

        # length-matched fail-closed negative control (18 fabricated chars)
        full_norm = normalize_text(" ".join(pages.values()))
        neg = normalize_text("此處乃完全虛構之不存在語句不可能命中XYZ") in full_norm

        ok = (failed == 0 and gated >= 40 and au_bad == 0 and not neg
              and not ctrl and not struct_errors)
        try:
            kb_recorded = str(kb.relative_to(ROOT))
        except ValueError:
            kb_recorded = str(kb)
        report = {
            "schema_version": "hkex.parity_report.v2",
            "pdf_sha256": live_sha,
            "provenance_sha_match": True,
            "kb_binary": kb_recorded,
            "ingest_runtime_secs": ingest_secs,
            "pdf_page_count": len(pages),
            "chunk_count": len(chunks),
            "author_utterances_total": sum(1 for u in utts if u.is_author),
            "gated_author_quotes": gated,
            "failed": failed,
            "skipped_too_short": skipped_short,
            "flagged_nonrenderable_glyph": flagged_glyph,
            "comment_pages_checked": sampled_comment_pages,
            "comment_au_violations": au_bad,
            "structural_errors": struct_errors,
            "control_chars": ctrl,
            "negative_control_found": neg,
            "strata_coverage_over_gated": strata,
            "verdict": "PASS" if ok else "FAIL",
            "misses": misses,
        }
        REPORT.write_text(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        print(json.dumps({k: v for k, v in report.items() if k != "misses"}, ensure_ascii=False, indent=2, sort_keys=True))
        for x in misses:
            print("  MISS:", x, file=sys.stderr)
        return 0 if ok else 1
    finally:
        if not args.keep_ingest:
            import shutil
            shutil.rmtree(work, ignore_errors=True)
        else:
            print(f"(kept ingest output at {work})", file=sys.stderr)


if __name__ == "__main__":
    raise SystemExit(main())
