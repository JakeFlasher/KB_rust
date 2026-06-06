#!/usr/bin/env python3
r"""Deterministically merge per-source ingest manifests for the ``hkex`` deck.

Adapted from (never mutating) ``sources/cfa/_registry/merge_ingest_manifests.py``. Composes
``out/hkex/{chunks_manifest,sources_manifest}.json`` from the per-source dirs named in
``ingest_plan.json``. For every source it RE-DERIVES the evidence rather than trusting it
(cf. BL-20260530-re-derive-stored-evidence-not-count): exactly one source per per-source
manifest, ``source_id``/``source_path`` match, recompute ``source_sha256`` from the file,
recompute every ``chunk_hash`` from its canonical envelope. Duplicate ``source_id`` /
``chunk_id`` are rejected at the trust boundary; sources/chunks are canonically sorted; the
merged manifests are written atomically and a re-run is byte-identical ("unchanged").

  (default)     merge the real plan into out/hkex
  --self-test   drive the merge/validate/dedup/clobber logic hermetically
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import tempfile
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[3]
PLAN_PATH = ROOT / "sources/hkex/_registry/ingest_plan.json"
OUT_DIR = ROOT / "out/hkex"
REPORT = ROOT / "sources/hkex/_registry/ingest_merge_report.json"
LIBPDFIUM = Path("/usr/lib/libpdfium.so")


def canonical_json_bytes(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode("utf-8")


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def recompute_chunk_hash(chunk: dict[str, Any]) -> str:
    spans = [{"byte_offset_in_chunk": int(s["byte_offset_in_chunk"]), "page": int(s["page"])}
             for s in chunk.get("page_spans", [])]
    envelope = {"end_page": int(chunk["end_page"]), "page_spans": spans,
                "start_page": int(chunk["start_page"]), "text": str(chunk["text"])}
    return hashlib.sha256(canonical_json_bytes(envelope)).hexdigest()


def atomic_write(path: Path, data: bytes, *, force: bool) -> str:
    if path.exists():
        if path.read_bytes() == data:
            return "unchanged"
        if not force:
            raise SystemExit(f"{path}: exists and differs; rerun with --force to replace")
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(f"{path.name}.tmp")
    if tmp.exists():
        raise SystemExit(f"{path}: sidecar {tmp.name} exists; refusing")
    tmp.write_bytes(data)
    os.replace(tmp, path)
    return "written"


def validate_source_manifest(path: Path, expected_id: str, expected_path: str, root: Path) -> dict[str, Any]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    if payload.get("schema_version") != "cacg.v0":
        raise SystemExit(f"{path}: unexpected schema_version")
    sources = payload.get("sources")
    if not isinstance(sources, list) or len(sources) != 1:
        raise SystemExit(f"{path}: expected exactly one source")
    source = sources[0]
    if source.get("source_id") != expected_id:
        raise SystemExit(f"{path}: source_id mismatch for {expected_id}")
    if source.get("source_path") != expected_path:
        raise SystemExit(f"{path}: source_path mismatch for {expected_id}: {source.get('source_path')!r}")
    src_file = root / expected_path
    if not src_file.is_file():
        raise SystemExit(f"{path}: source file missing for {expected_id}: {src_file}")
    actual = sha256_file(src_file)
    if source.get("source_sha256") != actual:
        raise SystemExit(f"{path}: source_sha256 mismatch for {expected_id}: "
                         f"manifest={source.get('source_sha256')} actual={actual}")
    return source


def validate_chunks_manifest(path: Path, expected_id: str) -> list[dict[str, Any]]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    if payload.get("schema_version") != "cacg.v0":
        raise SystemExit(f"{path}: unexpected schema_version")
    if payload.get("retracted_source_ids", []) or payload.get("retracted_chunk_ids", []):
        raise SystemExit(f"{path}: expected no retracted ids at bootstrap")
    chunks = payload.get("chunks")
    if not isinstance(chunks, list) or not chunks:
        raise SystemExit(f"{path}: expected non-empty chunks")
    for chunk in chunks:
        if chunk.get("source_id") != expected_id:
            raise SystemExit(f"{path}: chunk source_id mismatch for {expected_id}")
        recomputed = recompute_chunk_hash(chunk)
        if chunk.get("chunk_hash") != recomputed:
            raise SystemExit(f"{path}: chunk_hash mismatch for {chunk.get('chunk_id')}: "
                             f"manifest={chunk.get('chunk_hash')} actual={recomputed}")
    return chunks


def merge(plan: list[dict], root: Path) -> tuple[dict, dict]:
    sources: list[dict] = []
    chunks: list[dict] = []
    seen_sources: set[str] = set()
    seen_chunks: set[str] = set()
    for row in plan:
        sid = str(row["source_id"])
        if sid in seen_sources:
            raise SystemExit(f"duplicate source_id in plan: {sid}")
        seen_sources.add(sid)
        out_dir = root / str(row["out_dir"])
        sp, cp = out_dir / "sources_manifest.json", out_dir / "chunks_manifest.json"
        if not sp.is_file() or not cp.is_file():
            raise SystemExit(f"missing per-source manifests for {sid}: {out_dir}")
        sources.append(validate_source_manifest(sp, sid, str(row["canonical_path"]), root))
        for chunk in validate_chunks_manifest(cp, sid):
            cid = str(chunk.get("chunk_id", ""))
            if cid in seen_chunks:
                raise SystemExit(f"duplicate chunk_id: {cid}")
            seen_chunks.add(cid)
            chunks.append(chunk)
    sources.sort(key=lambda s: str(s["source_id"]))
    chunks.sort(key=lambda c: (str(c["source_id"]), int(c["ordinal"]), int(c["start_page"]), str(c["chunk_id"])))
    merged_sources = {"schema_version": "cacg.v0", "sources": sources}
    merged_chunks = {"schema_version": "cacg.v0", "chunks": chunks,
                     "retracted_source_ids": [], "retracted_chunk_ids": []}
    return merged_sources, merged_chunks


def write_report(merged_sources: dict, merged_chunks: dict) -> None:
    """Committed, deterministic evidence: source/chunk counts, per-source SHAs, the
    libpdfium identity, and the merged-manifest SHAs (re-merge reproduces them => the
    byte-identical proof). The large chunks_manifest itself stays gitignored."""
    chunks_by_src: dict[str, int] = {}
    for c in merged_chunks["chunks"]:
        chunks_by_src[c["source_id"]] = chunks_by_src.get(c["source_id"], 0) + 1
    srcs = [{"source_id": s["source_id"], "source_path": s["source_path"],
             "source_sha256": s["source_sha256"], "page_count": s.get("page_count"),
             "parser_name": s.get("parser_name"), "parser_version": s.get("parser_version"),
             "chunk_count": chunks_by_src.get(s["source_id"], 0)} for s in merged_sources["sources"]]
    report = {
        "schema_version": "hkex.ingest_merge_report.v1",
        "source_count": len(merged_sources["sources"]),
        "chunk_count": len(merged_chunks["chunks"]),
        "sources": srcs,
        "merged_sources_manifest_sha256": hashlib.sha256(canonical_json_bytes(merged_sources)).hexdigest(),
        "merged_chunks_manifest_sha256": hashlib.sha256(canonical_json_bytes(merged_chunks)).hexdigest(),
        "libpdfium": {
            "path": str(LIBPDFIUM),
            "sha256": sha256_file(LIBPDFIUM) if LIBPDFIUM.is_file() else None,
            "parser_name": srcs[0]["parser_name"] if srcs else None,
            "parser_version": srcs[0]["parser_version"] if srcs else None,
            "pin_disposition": "KB_SKIP_PDFIUM_HASH_CHECK=1 (documented deviation; same as the cfa corpus)",
        },
        "byte_identical_proof": ("re-running merge_hkex_manifests.py reproduces the merged-manifest SHAs "
                                 "above (atomic_write reports 'unchanged'); out/hkex/{chunks,sources}_manifest.json "
                                 "are gitignored and regenerated from the committed per-source plan + the rendered PDF."),
    }
    REPORT.write_text(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def do_merge(out_dir: Path, require_count: int | None, force: bool, report: bool) -> dict:
    plan = json.loads(PLAN_PATH.read_text(encoding="utf-8"))
    if require_count is not None and len(plan) != require_count:
        raise SystemExit(f"{PLAN_PATH}: expected {require_count} rows, got {len(plan)}")
    merged_sources, merged_chunks = merge(plan, ROOT)
    s_status = atomic_write(out_dir / "sources_manifest.json", canonical_json_bytes(merged_sources), force=force)
    c_status = atomic_write(out_dir / "chunks_manifest.json", canonical_json_bytes(merged_chunks), force=force)
    if report:
        write_report(merged_sources, merged_chunks)
    return {"sources_manifest": s_status, "chunks_manifest": c_status,
            "source_count": len(merged_sources["sources"]), "chunk_count": len(merged_chunks["chunks"]),
            "out_dir": str(out_dir)}


# --------------------------------------------------------------------------- #
# Hermetic self-test of the merge/validate/dedup/clobber logic.
# --------------------------------------------------------------------------- #
def _emit_source(root: Path, sid: str, rel_pdf: str, text: str, ordinal: int = 0) -> dict:
    (root / rel_pdf).parent.mkdir(parents=True, exist_ok=True)
    (root / rel_pdf).write_bytes(f"fake-pdf-{sid}".encode())
    out_dir = f"out/x/{sid}"
    od = root / out_dir
    od.mkdir(parents=True, exist_ok=True)
    chunk = {"schema_version": "cacg.v0", "source_id": sid, "chunk_id": f"{sid}:p001:{ordinal:04d}",
             "ordinal": ordinal, "start_page": 1, "end_page": 1,
             "page_spans": [{"page": 1, "byte_offset_in_chunk": 0}], "token_count": 3, "text": text,
             "text_preview": text[:40]}
    chunk["chunk_hash"] = recompute_chunk_hash(chunk)
    (od / "chunks_manifest.json").write_text(json.dumps(
        {"schema_version": "cacg.v0", "chunks": [chunk], "retracted_source_ids": [], "retracted_chunk_ids": []}))
    (od / "sources_manifest.json").write_text(json.dumps({"schema_version": "cacg.v0", "sources": [
        {"schema_version": "cacg.v0", "source_id": sid, "source_path": rel_pdf,
         "source_sha256": sha256_file(root / rel_pdf), "parser_name": "fake", "parser_version": "0",
         "page_count": 1, "extracted_at": "1970-01-01T00:00:00Z"}]}))
    return {"source_id": sid, "canonical_path": rel_pdf, "out_dir": out_dir}


def self_test() -> int:
    failures: list[str] = []
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        r1 = _emit_source(root, "src_b", "pdfs/b.pdf", "beta text")
        r2 = _emit_source(root, "src_a", "pdfs/a.pdf", "alpha text")
        # happy path: 2 sources merge; canonical sort puts src_a before src_b
        ms, mc = merge([r1, r2], root)
        if [s["source_id"] for s in ms["sources"]] != ["src_a", "src_b"]:
            failures.append("sources not canonically sorted")
        if len(mc["chunks"]) != 2:
            failures.append("expected 2 merged chunks")
        # byte-identical re-run via atomic_write "unchanged"
        out = root / "out/x/merged"
        b = canonical_json_bytes(mc)
        if atomic_write(out / "c.json", b, force=False) != "written" or \
           atomic_write(out / "c.json", b, force=False) != "unchanged":
            failures.append("atomic re-run not byte-identical")
        # clobber: differing content without force fails closed
        try:
            atomic_write(out / "c.json", b + b"x", force=False)
            failures.append("clobber not refused")
        except SystemExit:
            pass
        # duplicate source_id rejected
        try:
            merge([r1, dict(r2, source_id="src_b")], root)
            failures.append("dup source_id not rejected")
        except SystemExit:
            pass
        # duplicate chunk_id across sources rejected: force src_c's chunk_id to collide
        # with src_a's (its own source_id stays src_c, so it passes the per-source source_id
        # check; the cross-source chunk_id dedup must still reject the merge).
        r3 = _emit_source(root, "src_c", "pdfs/c.pdf", "gamma")
        cm = root / r3["out_dir"] / "chunks_manifest.json"
        p = json.loads(cm.read_text())
        p["chunks"][0]["chunk_id"] = "src_a:p001:0000"
        cm.write_text(json.dumps(p))
        try:
            merge([r2, r3], root)
            failures.append("dup chunk_id not rejected")
        except SystemExit:
            pass
        # source_sha256 mismatch rejected (tamper the pdf after manifest)
        (root / r1["canonical_path"]).write_bytes(b"tampered")
        try:
            merge([r1], root)
            failures.append("source_sha256 mismatch not rejected")
        except SystemExit:
            pass
    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print("  -", f)
        return 1
    print("SELF-TEST PASSED (merge_hkex_manifests: sort/dedup/clobber/sha/atomic byte-identical)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("--force", action="store_true")
    ap.add_argument("--out", default=str(OUT_DIR))
    ap.add_argument("--require-count", type=int, default=None,
                    help="assert the plan has exactly N rows (the rebuild recipe passes it)")
    ap.add_argument("--no-report", action="store_true", help="skip writing the committed merge report")
    args = ap.parse_args()
    if args.self_test:
        return self_test()
    out_dir = Path(args.out)
    if not out_dir.is_absolute():
        out_dir = ROOT / out_dir
    print(json.dumps(do_merge(out_dir, args.require_count, args.force, not args.no_report),
                     ensure_ascii=False, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
