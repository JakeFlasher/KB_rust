#!/usr/bin/env python3
r"""Acquire the free Tier-1 grounding sources (AC-6 ingest half).

Downloads each source named in `grounding_sources_spec.json` to a dated, SHA-pinned local
PDF (HTML pages are snapshotted via the deterministic `html_to_pdf.py` recipe), records the
URL + raw-download SHA + snapshot-PDF SHA + acquisition date + page count in the committed
`grounding_sources.json` manifest, and appends each acquired source to `ingest_plan.json`
(default chunk config — English grounding PDFs). A source that cannot be resolved/downloaded
goes to the manifest's omission ledger and is deferred, NEVER fabricated. Idempotent: a source
whose local PDF already matches the recorded snapshot SHA is reused (no re-download) so the
committed manifest is stable; `--refresh` re-downloads.

The `*.pdf` snapshots + raw `*.source.html` are gitignored (local-only, like the corpus); the
manifest + the per-HTML `.pdf.provenance.json` are committed. A clean rebuild re-downloads from
the recorded URL (volatile HKEX URLs may need re-resolution) and a changed SHA is surfaced.

  --acquire [--refresh] [--date YYYY-MM-DD]   acquire per the spec; write the manifest + plan
"""
from __future__ import annotations

import argparse
import datetime
import hashlib
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "sources/hkex/_registry"))
import html_to_pdf  # noqa: E402  # pyright: ignore[reportMissingImports]
from cacg_normalize import normalize_text  # noqa: E402  # pyright: ignore[reportMissingImports]

SPEC = ROOT / "sources/hkex/_registry/grounding_sources_spec.json"
MANIFEST = ROOT / "sources/hkex/_registry/grounding_sources.json"
INGEST_PLAN = ROOT / "sources/hkex/_registry/ingest_plan.json"
PDF_DIR = ROOT / "sources/hkex/pdfs"
UA = "Mozilla/5.0 (X11; Linux x86_64) hkex-deck-grounding-acquire"


def sha256_bytes(b: bytes) -> str:
    return hashlib.sha256(b).hexdigest()


def sha256_file(p: Path) -> str:
    return sha256_bytes(p.read_bytes())


def curl(url: str, dest: Path) -> str:
    """Download url -> dest; return HTTP code. Follows redirects, 60s cap."""
    r = subprocess.run(["curl", "-sSL", "-m", "60", "-A", UA, "-o", str(dest), "-w", "%{http_code}", url],
                       capture_output=True, text=True)
    return r.stdout.strip() or "000"


def page_count(pdf: Path) -> int:
    import pypdfium2 as pdfium
    doc = pdfium.PdfDocument(str(pdf))
    n = len(doc)
    doc.close()
    return n


def extract_pdf_text(pdf: Path) -> str:
    import pypdfium2 as pdfium
    doc = pdfium.PdfDocument(str(pdf))
    try:
        return " ".join(doc[i].get_textpage().get_text_range() for i in range(len(doc)))
    finally:
        doc.close()


def missing_phrases(pdf: Path, phrases: list[str]) -> list[str]:
    """Fail-closed content sentinel: every expected phrase must appear (case-insensitive)
    in the snapshot's CACG-normalized extracted text — so a 200 'Page not found' / nav-chrome
    body that still snapshots into a PDF is rejected rather than silently accepted."""
    norm = normalize_text(extract_pdf_text(pdf)).lower()
    return [p for p in phrases if p.lower() not in norm]


def acquire(refresh: bool, date: str) -> dict:
    spec = json.loads(SPEC.read_text(encoding="utf-8"))["sources"]
    prior = {}
    if MANIFEST.is_file():
        prior = {s["source_id"]: s for s in json.loads(MANIFEST.read_text(encoding="utf-8")).get("sources", [])}
    PDF_DIR.mkdir(parents=True, exist_ok=True)
    acquired, omissions = [], []
    for s in spec:
        sid, rid, typ, url = s["source_id"], s["reading_id"], s["type"], s["url"]
        pdf = PDF_DIR / f"{sid}.pdf"
        # idempotent reuse: local PDF present + matches the recorded snapshot SHA
        if not refresh and pdf.is_file() and sid in prior and sha256_file(pdf) == prior[sid].get("snapshot_pdf_sha256"):
            acquired.append(prior[sid])
            continue
        with tempfile.TemporaryDirectory() as d:
            tmp = Path(d) / "dl"
            code = curl(url, tmp)
            if code != "200" or not tmp.is_file() or tmp.stat().st_size == 0:
                omissions.append({"source_id": sid, "url": url, "http_code": code,
                                  "content_sha256": None, "missing_phrases": None,
                                  "reason": f"HTTP {code} / empty body"})
                continue
            raw = tmp.read_bytes()
            raw_sha = sha256_bytes(raw)
            html_prov = None
            if typ == "pdf":
                if not raw.startswith(b"%PDF"):
                    omissions.append({"source_id": sid, "url": url, "http_code": code,
                                      "content_sha256": raw_sha, "missing_phrases": None,
                                      "reason": "not a PDF (magic bytes)"})
                    continue
                pdf.write_bytes(raw)
            elif typ == "html":
                html_src = PDF_DIR / f"{sid}.source.html"   # gitignored
                html_src.write_bytes(raw)
                html_prov = html_to_pdf.snapshot(html_src, pdf)   # writes pdf + .provenance.json
            else:
                omissions.append({"source_id": sid, "url": url, "http_code": code,
                                  "content_sha256": raw_sha, "missing_phrases": None,
                                  "reason": f"unknown type {typ}"})
                continue
            # fail-closed content sentinel: a 404 / nav-chrome body that still snapshots is rejected.
            miss = missing_phrases(pdf, s.get("expected_phrases", []))
            if miss:
                omissions.append({"source_id": sid, "url": url, "http_code": code,
                                  "content_sha256": raw_sha, "missing_phrases": miss,
                                  "reason": "snapshot text missing expected sentinel(s)"})
                pdf.unlink(missing_ok=True)
                (pdf.with_suffix(pdf.suffix + ".provenance.json")).unlink(missing_ok=True)
                continue
            acquired.append({
                "source_id": sid, "reading_id": rid, "type": typ, "url": url,
                "acquisition_date": date,
                "raw_download_sha256": raw_sha,
                "snapshot_pdf_sha256": sha256_file(pdf),
                "page_count": page_count(pdf),
                "grounds": s.get("grounds"),
                "html_snapshot_provenance": (str((pdf.with_suffix(pdf.suffix + ".provenance.json")).relative_to(ROOT))
                                             if html_prov else None),
            })
    manifest = {
        "schema_version": "hkex.grounding_sources.v1",
        "note": ("Dated, SHA-pinned snapshots of the free Tier-1 grounding sources. The *.pdf are "
                 "gitignored; this manifest + the per-HTML .pdf.provenance.json are committed. A clean "
                 "rebuild re-downloads from `url`; a changed SHA means the upstream source moved/changed "
                 "(volatile HKEX URLs may need re-resolution). Unresolved sources are in omission_ledger."),
        "source_count": len(acquired),
        "sources": sorted(acquired, key=lambda x: x["source_id"]),
        "omission_ledger": sorted(omissions, key=lambda x: x["source_id"]),
    }
    MANIFEST.write_text(json.dumps(manifest, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    _reconcile_ingest_plan(acquired)
    return manifest


def _reconcile_ingest_plan(acquired: list[dict]) -> None:
    """Reconcile ingest_plan to exactly: the non-grounding rows (the Xueqiu corpus, kept
    verbatim with its CJK config) PLUS one grounding row per ACQUIRED source. A grounding row
    for a source no longer acquired (URL went stale / dropped from the spec) is removed so the
    plan never references a source the merge can't validate."""
    plan = json.loads(INGEST_PLAN.read_text(encoding="utf-8"))
    kept = [r for r in plan if r.get("source_type") != "grounding_pdf"]
    for s in acquired:
        kept.append({
            "source_id": s["source_id"],
            "source_type": "grounding_pdf",
            "canonical_path": f"sources/hkex/pdfs/{s['source_id']}.pdf",
            "out_dir": f"out/hkex/ingest_per_source/{s['source_id']}",
            "note": f"Free Tier-1 grounding source ({s['reading_id']}); acquired {s['acquisition_date']} from {s['url']}.",
        })
    kept.sort(key=lambda r: str(r["source_id"]))
    INGEST_PLAN.write_text(json.dumps(kept, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def self_test() -> int:
    """Hermetic fail-closed probe: prove the content sentinel rejects a nav-chrome / 'page not
    found' body that still snapshots into a valid PDF, and accepts a genuine-content body. This
    is the audit Codex asked for (a 404/nav body must be omission-ledgered, never silently used)."""
    failures: list[str] = []
    good_html = ("<html><body><h1>Rates of Stamp Duty</h1>"
                 "<p>Stamp duty on Hong Kong stock is charged at 0.1% on every sold note "
                 "and every bought note.</p></body></html>")
    nav_html = ("<html><body><nav>Skip to main content Home About Us Search Contact Us "
                "Site Map</nav><h1>Page not found</h1><p>The page you requested is unavailable."
                "</p></body></html>")
    sentinels = ["stamp duty", "0.1%", "every bought note"]
    with tempfile.TemporaryDirectory() as d:
        gp, gpdf = Path(d) / "g.html", Path(d) / "g.pdf"
        np_, npdf = Path(d) / "n.html", Path(d) / "n.pdf"
        gp.write_text(good_html, encoding="utf-8"); html_to_pdf.snapshot(gp, gpdf)
        np_.write_text(nav_html, encoding="utf-8"); html_to_pdf.snapshot(np_, npdf)
        miss_good = missing_phrases(gpdf, sentinels)
        miss_nav = missing_phrases(npdf, sentinels)
        if miss_good:
            failures.append(f"genuine-content body wrongly rejected: missing {miss_good}")
        if not miss_nav:
            failures.append("nav/404 body NOT rejected (fail-closed sentinel did not fire)")
    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print("  -", f)
        return 1
    print("SELF-TEST PASSED (acquire_grounding_sources: genuine-content body accepted; "
          "nav/404 body rejected by the expected_phrases content sentinel -> omission ledger)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--acquire", action="store_true")
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("--refresh", action="store_true")
    ap.add_argument("--date", default=datetime.date.today().isoformat())
    args = ap.parse_args()
    if args.self_test:
        return self_test()
    if not args.acquire:
        ap.error("provide --acquire or --self-test")
    m = acquire(args.refresh, args.date)
    print(json.dumps({"source_count": m["source_count"],
                      "acquired": [s["source_id"] for s in m["sources"]],
                      "omitted": [{"source_id": o["source_id"], "reason": o["reason"],
                                   "missing_phrases": o.get("missing_phrases")} for o in m["omission_ledger"]]},
                     ensure_ascii=False, indent=2, sort_keys=True))
    # Fail closed: every spec source is current-scope, so any omission is a failure to be
    # addressed (re-resolve the URL) or explicitly accepted in the round summary + tracker.
    return 0 if not m["omission_ledger"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
