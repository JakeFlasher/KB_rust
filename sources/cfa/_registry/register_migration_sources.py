#!/usr/bin/env python3
"""Register the text-layer migration sources (readings 14 / 15 / 22) into the
cfa ingest + authorization registries.

For every TEXT-LAYER source declared in each set's ``_sources.json`` this copies
the incoming PDF to ``sources/cfa/pdfs/<reading_id>/<source_id>.pdf``
(verified byte-identical by SHA-256 against the manifest value), appends one
well-formed ``ingest_plan.json`` row, and extends the source-authorization matrix
-- the canonical ``source_matrix.json``, its ``.pretty`` mirror, and the
gate-consumed ``out/cfa/source_matrix.json`` -- with the new reading keys
while PRESERVING every existing authorization.

O'Hara 1995 (``mt_ohara_*``, image-only/OCR, ``text_layer:false``) is intentionally
EXCLUDED here; its source set is resolved separately before the final merge.

The run is idempotent (a PDF already copied with a matching SHA is left alone; an
already-registered ``source_id`` is not duplicated) and fail-closed (a SHA
mismatch, a missing PDF, or an unknown set aborts before any registry write).
``--self-test`` drives the pure helpers from synthetic inputs.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import tempfile
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[3]
REGISTRY = ROOT / "sources/cfa/_registry"
PDFS_DIR = ROOT / "sources/cfa/pdfs"
INGEST_PLAN = REGISTRY / "ingest_plan.json"
MATRIX_COMPACT = REGISTRY / "source_matrix.json"
MATRIX_PRETTY = REGISTRY / "source_matrix.pretty.json"
MATRIX_OUT = ROOT / "out/cfa/source_matrix.json"
DEFERRED = Path(os.environ.get("KB_DEFERRED_ORIGIN", "/home/jakeshea/CFA_reading/deferred_books"))

# (origin subdirectory, target reading_id). The reading_id is the on-disk PDF
# folder and the source-matrix authorization key.
INCOMING_SETS = (
    ("14_Microstructure_and_Trading", "14_microstructure_and_trading"),
    ("15_Performance_and_Attribution", "15_performance_and_attribution"),
    ("22_Fund_Level_Arbitrage", "22_fund_level_arbitrage"),
)


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for block in iter(lambda: f.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def atomic_write_bytes(path: Path, data: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, tmp_name = tempfile.mkstemp(prefix=f".{path.name}.", suffix=".tmp", dir=path.parent)
    try:
        with os.fdopen(fd, "wb") as f:
            f.write(data)
            f.flush()
            os.fsync(f.fileno())
        os.replace(tmp_name, path)
    finally:
        if os.path.exists(tmp_name):
            os.unlink(tmp_name)


# --------------------------------------------------------------------------- #
# Pure helpers (driven hermetically by --self-test).
# --------------------------------------------------------------------------- #

def serialize_compact(obj: Any) -> bytes:
    return json.dumps(obj, ensure_ascii=False, separators=(",", ":"), sort_keys=True).encode("utf-8") + b"\n"


def serialize_pretty(obj: Any) -> bytes:
    return json.dumps(obj, ensure_ascii=False, indent=2, sort_keys=True).encode("utf-8") + b"\n"


def serialize_plan(plan: list[dict[str, Any]]) -> bytes:
    return json.dumps(plan, ensure_ascii=False, indent=2).encode("utf-8") + b"\n"


def merge_allowed(existing: dict[str, list[str]], additions: dict[str, list[str]]) -> dict[str, list[str]]:
    """Union each reading's authorizations with the additions, sorted + deduped.

    Existing entries are PRESERVED (the union never drops one); the result is
    sorted so the matrix is byte-stable regardless of input order.
    """
    merged = {key: sorted(set(values)) for key, values in existing.items()}
    for reading_id, source_ids in additions.items():
        merged[reading_id] = sorted(set(merged.get(reading_id, [])) | set(source_ids))
    return merged


def ingest_row(reading_id: str, source_id: str) -> dict[str, Any]:
    """One ingest_plan row matching the existing field shape + key order."""
    canonical = f"sources/cfa/pdfs/{reading_id}/{source_id}.pdf"
    out_dir = f"out/cfa/ingest_per_source/{source_id}"
    return {
        "canonical_path": canonical,
        "command": [
            "cargo", "run", "-p", "cacg-cli", "--bin", "kb", "--",
            "ingest", canonical, "--source-id", source_id, "--out", out_dir,
        ],
        "out_dir": out_dir,
        "source_id": source_id,
    }


# --------------------------------------------------------------------------- #
# Registration.
# --------------------------------------------------------------------------- #

def load_text_layer_sources() -> list[dict[str, Any]]:
    """Read every set's ``_sources.json`` and return the text-layer sources as
    ``{reading_id, source_id, src_pdf, sha256}`` records (O'Hara/OCR excluded)."""
    records: list[dict[str, Any]] = []
    for origin_subdir, reading_id in INCOMING_SETS:
        sources_json = DEFERRED / origin_subdir / "_sources.json"
        data = json.loads(sources_json.read_text(encoding="utf-8"))
        for source in data["sources"]:
            if not source.get("text_layer", True):
                continue
            records.append({
                "reading_id": reading_id,
                "source_id": str(source["source_id"]),
                "src_pdf": DEFERRED / origin_subdir / str(source["file"]),
                "sha256": str(source["sha256"]),
            })
    return records


def copy_and_verify(record: dict[str, Any]) -> str:
    """Copy one source PDF into the framework and verify its SHA-256.

    Idempotent: an already-present copy whose SHA matches is left untouched. A
    SHA mismatch (incoming or copied) aborts.
    """
    src: Path = record["src_pdf"]
    if not src.is_file():
        raise SystemExit(f"incoming PDF missing: {src}")
    incoming_sha = sha256_file(src)
    if incoming_sha != record["sha256"]:
        raise SystemExit(
            f"incoming PDF SHA mismatch for {record['source_id']}: "
            f"manifest={record['sha256']} actual={incoming_sha}"
        )
    dst = PDFS_DIR / record["reading_id"] / f"{record['source_id']}.pdf"
    if dst.is_file() and sha256_file(dst) == record["sha256"]:
        return "present"
    dst.parent.mkdir(parents=True, exist_ok=True)
    fd, tmp_name = tempfile.mkstemp(prefix=f".{dst.name}.", suffix=".tmp", dir=dst.parent)
    os.close(fd)
    try:
        shutil.copyfile(src, tmp_name)
        copied_sha = sha256_file(Path(tmp_name))
        if copied_sha != record["sha256"]:
            raise SystemExit(f"copied PDF SHA mismatch for {record['source_id']}: {copied_sha}")
        os.replace(tmp_name, dst)
    finally:
        if os.path.exists(tmp_name):
            os.unlink(tmp_name)
    return "copied"


def run(write: bool) -> dict[str, Any]:
    records = load_text_layer_sources()

    pdf_status: dict[str, str] = {}
    for record in records:
        pdf_status[record["source_id"]] = copy_and_verify(record) if write else "dry-run"

    # ingest_plan: append rows for any not-yet-registered source_id.
    plan = json.loads(INGEST_PLAN.read_text(encoding="utf-8"))
    existing_ids = {str(row["source_id"]) for row in plan}
    added_rows = 0
    for record in sorted(records, key=lambda r: r["source_id"]):
        if record["source_id"] not in existing_ids:
            plan.append(ingest_row(record["reading_id"], record["source_id"]))
            existing_ids.add(record["source_id"])
            added_rows += 1

    # source matrix: extend with the new reading authorizations, preserving all.
    matrix = json.loads(MATRIX_COMPACT.read_text(encoding="utf-8"))
    additions: dict[str, list[str]] = {}
    for record in records:
        additions.setdefault(record["reading_id"], []).append(record["source_id"])
    matrix["allowed"] = merge_allowed(matrix.get("allowed", {}), additions)

    if write:
        atomic_write_bytes(INGEST_PLAN, serialize_plan(plan))
        atomic_write_bytes(MATRIX_COMPACT, serialize_compact(matrix))
        atomic_write_bytes(MATRIX_PRETTY, serialize_pretty(matrix))
        atomic_write_bytes(MATRIX_OUT, serialize_pretty(matrix))

    return {
        "text_layer_sources": len(records),
        "pdf_status": pdf_status,
        "ingest_plan_rows_added": added_rows,
        "ingest_plan_total": len(plan),
        "matrix_keys": {rid: matrix["allowed"][rid] for _, rid in INCOMING_SETS},
    }


# --------------------------------------------------------------------------- #
# Self-test.
# --------------------------------------------------------------------------- #

def self_test() -> int:
    failures: list[str] = []

    existing = {"15_x": ["cfa_b", "cfa_a"], "01_x": ["z"]}
    merged = merge_allowed(existing, {"15_x": ["pa_2", "pa_1"], "14_x": ["mt_b", "mt_a"]})
    if merged["15_x"] != ["cfa_a", "cfa_b", "pa_1", "pa_2"]:
        failures.append(f"15_x not preserved+extended+sorted: {merged['15_x']}")
    if merged["14_x"] != ["mt_a", "mt_b"]:
        failures.append(f"14_x not added+sorted: {merged['14_x']}")
    if merged["01_x"] != ["z"]:
        failures.append("untouched key 01_x changed")
    # idempotent: re-merging the same additions is a no-op
    if merge_allowed(merged, {"15_x": ["pa_1"], "14_x": ["mt_a"]}) != merged:
        failures.append("merge_allowed not idempotent")
    # a duplicate across existing+addition does not double
    if merge_allowed({"k": ["a"]}, {"k": ["a", "b"]})["k"] != ["a", "b"]:
        failures.append("merge_allowed duplicated an existing id")

    row = ingest_row("14_microstructure_and_trading", "mt_harris_2003_trading_and_exchanges")
    if list(row.keys()) != ["canonical_path", "command", "out_dir", "source_id"]:
        failures.append(f"ingest_row key order wrong: {list(row.keys())}")
    if row["canonical_path"] != "sources/cfa/pdfs/14_microstructure_and_trading/mt_harris_2003_trading_and_exchanges.pdf":
        failures.append("ingest_row canonical_path wrong")
    if row["out_dir"] != "out/cfa/ingest_per_source/mt_harris_2003_trading_and_exchanges":
        failures.append("ingest_row out_dir wrong")
    if row["command"][:8] != ["cargo", "run", "-p", "cacg-cli", "--bin", "kb", "--", "ingest"]:
        failures.append("ingest_row command prefix wrong")
    if row["command"][8] != row["canonical_path"] or row["command"][-1] != row["out_dir"]:
        failures.append("ingest_row command path/out_dir wrong")

    # serializers round-trip and match the detected on-disk shapes
    obj = {"allowed": {"b": ["2", "1"], "a": ["x"]}, "schema_version": "cacg.v0"}
    if serialize_compact(obj) != b'{"allowed":{"a":["x"],"b":["2","1"]},"schema_version":"cacg.v0"}\n':
        failures.append(f"serialize_compact drift: {serialize_compact(obj)!r}")
    if not serialize_pretty(obj).startswith(b'{\n  "allowed": {\n    "a": [\n'):
        failures.append("serialize_pretty drift")

    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print(f"  - {f}")
        return 1
    print("SELF-TEST PASSED (merge_allowed preserve/extend/sort/idempotent + ingest_row + serializers)")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run hermetic self-test and exit")
    parser.add_argument("--write", action="store_true", help="copy PDFs and write the registries")
    args = parser.parse_args()

    if args.self_test:
        return self_test()

    result = run(write=args.write)
    print(json.dumps(result, ensure_ascii=False, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
