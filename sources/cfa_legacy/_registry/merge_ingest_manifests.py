#!/usr/bin/env python3
"""Deterministically merge per-source ingest manifests for the CFA legacy corpus."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[3]
PLAN_PATH = ROOT / "sources/cfa_legacy/_registry/ingest_plan.json"
OUT_DIR = ROOT / "out/cfa_legacy"


def canonical_json_bytes(value: Any) -> bytes:
    body = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"))
    return body.encode("utf-8")


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def recompute_chunk_hash(chunk: dict[str, Any]) -> str:
    spans = [
        {
            "byte_offset_in_chunk": int(span["byte_offset_in_chunk"]),
            "page": int(span["page"]),
        }
        for span in chunk.get("page_spans", [])
    ]
    envelope = {
        "end_page": int(chunk["end_page"]),
        "page_spans": spans,
        "start_page": int(chunk["start_page"]),
        "text": str(chunk["text"]),
    }
    payload = json.dumps(
        envelope,
        ensure_ascii=False,
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")
    return hashlib.sha256(payload).hexdigest()


def atomic_write(path: Path, data: bytes, *, force: bool) -> str:
    if path.exists():
        old = path.read_bytes()
        if old == data:
            return "unchanged"
        if not force:
            raise SystemExit(f"{path}: exists and differs; rerun with --force to replace")

    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(f"{path.name}.tmp")
    bak = path.with_name(f"{path.name}.bak")
    if tmp.exists() or bak.exists():
        raise SystemExit(f"{path}: sidecar exists ({tmp.name} or {bak.name}); refusing")
    tmp.write_bytes(data)
    os.replace(tmp, path)
    return "written"


def validate_source_manifest(path: Path, expected_id: str, expected_path: str) -> dict[str, Any]:
    payload = load_json(path)
    if payload.get("schema_version") != "cacg.v0":
        raise SystemExit(f"{path}: unexpected schema_version")
    sources = payload.get("sources")
    if not isinstance(sources, list) or len(sources) != 1:
        raise SystemExit(f"{path}: expected exactly one source")
    source = sources[0]
    if source.get("source_id") != expected_id:
        raise SystemExit(f"{path}: source_id mismatch for {expected_id}")
    if source.get("source_path") != expected_path:
        raise SystemExit(
            f"{path}: source_path mismatch for {expected_id}: {source.get('source_path')!r}"
        )
    source_path = ROOT / expected_path
    if not source_path.is_file():
        raise SystemExit(f"{path}: source file missing for {expected_id}: {source_path}")
    actual_sha256 = sha256_file(source_path)
    if source.get("source_sha256") != actual_sha256:
        raise SystemExit(
            f"{path}: source_sha256 mismatch for {expected_id}: "
            f"manifest={source.get('source_sha256')} actual={actual_sha256}"
        )
    return source


def validate_chunks_manifest(path: Path, expected_id: str) -> list[dict[str, Any]]:
    payload = load_json(path)
    if payload.get("schema_version") != "cacg.v0":
        raise SystemExit(f"{path}: unexpected schema_version")
    if payload.get("retracted_source_ids", []) != []:
        raise SystemExit(f"{path}: expected no retracted_source_ids at bootstrap")
    if payload.get("retracted_chunk_ids", []) != []:
        raise SystemExit(f"{path}: expected no retracted_chunk_ids at bootstrap")
    chunks = payload.get("chunks")
    if not isinstance(chunks, list) or not chunks:
        raise SystemExit(f"{path}: expected non-empty chunks")
    for chunk in chunks:
        if chunk.get("source_id") != expected_id:
            raise SystemExit(f"{path}: chunk source_id mismatch for {expected_id}")
        recomputed = recompute_chunk_hash(chunk)
        if chunk.get("chunk_hash") != recomputed:
            raise SystemExit(
                f"{path}: chunk_hash mismatch for {chunk.get('chunk_id')}: "
                f"manifest={chunk.get('chunk_hash')} actual={recomputed}"
            )
    return chunks


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--force", action="store_true", help="replace differing merged manifests")
    parser.add_argument("--out", default=str(OUT_DIR), help="merged out directory")
    parser.add_argument("--require-count", type=int, default=70)
    args = parser.parse_args()

    plan = load_json(PLAN_PATH)
    if not isinstance(plan, list):
        raise SystemExit(f"{PLAN_PATH}: expected a JSON list")
    plan = sorted(plan, key=lambda row: str(row["source_id"]))
    if len(plan) != args.require_count:
        raise SystemExit(f"{PLAN_PATH}: expected {args.require_count} rows, got {len(plan)}")

    sources: list[dict[str, Any]] = []
    chunks: list[dict[str, Any]] = []
    seen_sources: set[str] = set()
    seen_chunks: set[str] = set()

    for row in plan:
        source_id = str(row["source_id"])
        if source_id in seen_sources:
            raise SystemExit(f"duplicate source_id in plan: {source_id}")
        seen_sources.add(source_id)

        out_dir = ROOT / str(row["out_dir"])
        sources_path = out_dir / "sources_manifest.json"
        chunks_path = out_dir / "chunks_manifest.json"
        if not sources_path.is_file() or not chunks_path.is_file():
            raise SystemExit(f"missing per-source manifests for {source_id}: {out_dir}")

        sources.append(
            validate_source_manifest(sources_path, source_id, str(row["canonical_path"]))
        )
        source_chunks = validate_chunks_manifest(chunks_path, source_id)
        for chunk in source_chunks:
            chunk_id = str(chunk.get("chunk_id", ""))
            if chunk_id in seen_chunks:
                raise SystemExit(f"duplicate chunk_id: {chunk_id}")
            seen_chunks.add(chunk_id)
            chunks.append(chunk)

    sources.sort(key=lambda source: str(source["source_id"]))
    chunks.sort(
        key=lambda chunk: (
            str(chunk["source_id"]),
            int(chunk["ordinal"]),
            int(chunk["start_page"]),
            str(chunk["chunk_id"]),
        )
    )

    merged_sources = {"schema_version": "cacg.v0", "sources": sources}
    merged_chunks = {
        "schema_version": "cacg.v0",
        "chunks": chunks,
        "retracted_source_ids": [],
        "retracted_chunk_ids": [],
    }

    out_dir = Path(args.out)
    if not out_dir.is_absolute():
        out_dir = ROOT / out_dir
    source_status = atomic_write(
        out_dir / "sources_manifest.json",
        canonical_json_bytes(merged_sources),
        force=args.force,
    )
    chunk_status = atomic_write(
        out_dir / "chunks_manifest.json",
        canonical_json_bytes(merged_chunks),
        force=args.force,
    )
    print(
        json.dumps(
            {
                "sources_manifest": source_status,
                "chunks_manifest": chunk_status,
                "source_count": len(sources),
                "chunk_count": len(chunks),
                "out_dir": str(out_dir),
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
