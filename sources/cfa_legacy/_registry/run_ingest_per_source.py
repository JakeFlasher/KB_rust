#!/usr/bin/env python3
"""Run the CFA legacy per-source PDF ingest plan safely.

This wrapper keeps the generated ingest process resumable: complete per-source
outputs are skipped, while partial outputs stop the run for manual inspection.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
PLAN_PATH = ROOT / "sources/cfa_legacy/_registry/ingest_plan.json"
SNAPSHOT_PATH = ROOT / "sources/cfa_legacy/_registry/snapshot.json"
DEFAULT_PDFIUM_LIBRARY = Path("/usr/lib/libpdfium.so")


def load_plan() -> list[dict[str, object]]:
    with PLAN_PATH.open("r", encoding="utf-8") as f:
        plan = json.load(f)
    if not isinstance(plan, list):
        raise SystemExit(f"{PLAN_PATH}: expected a JSON list")
    return plan


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def expected_pdfium_sha256() -> str | None:
    if not SNAPSHOT_PATH.is_file():
        return None
    with SNAPSHOT_PATH.open("r", encoding="utf-8") as f:
        snapshot = json.load(f)
    value = snapshot.get("pdfium_library_sha256")
    return str(value) if value else None


def verify_pdfium_library() -> None:
    if os.environ.get("KB_SKIP_PDFIUM_HASH_CHECK") == "1":
        print("warning: skipping Pdfium hash check because KB_SKIP_PDFIUM_HASH_CHECK=1")
        return

    expected = expected_pdfium_sha256()
    if expected is None:
        return

    library = Path(os.environ.get("KB_PDFIUM_LIBRARY", str(DEFAULT_PDFIUM_LIBRARY)))
    if not library.is_file():
        raise SystemExit(f"Pdfium library not found for hash check: {library}")

    actual = sha256_file(library)
    if actual != expected:
        raise SystemExit(
            "Pdfium library hash mismatch; refusing to re-ingest with a drifted parser. "
            f"library={library} expected={expected} actual={actual}. "
            "Update the snapshot after an intentional parser upgrade, or set "
            "KB_SKIP_PDFIUM_HASH_CHECK=1 for an explicitly non-reproducible run."
        )


def kb_prefix() -> list[str]:
    if os.environ.get("KB_FORCE_CARGO") == "1":
        return ["cargo", "run", "-p", "cacg-cli", "--bin", "kb", "--"]
    exe = ROOT / "target/debug/kb"
    if exe.is_file() and os.access(exe, os.X_OK):
        return [str(exe)]
    return ["cargo", "run", "-p", "cacg-cli", "--bin", "kb", "--"]


def complete(out_dir: Path) -> bool:
    return (
        (out_dir / "sources_manifest.json").is_file()
        and (out_dir / "chunks_manifest.json").is_file()
    )


def has_partial_output(out_dir: Path) -> bool:
    if not out_dir.exists() or complete(out_dir):
        return False
    return any(out_dir.iterdir())


def filtered_plan(
    plan: list[dict[str, object]],
    only: set[str],
    start_after: str | None,
    limit: int | None,
) -> list[dict[str, object]]:
    rows = []
    seen_start = start_after is None
    for row in plan:
        source_id = str(row["source_id"])
        if not seen_start:
            if source_id == start_after:
                seen_start = True
            continue
        if only and source_id not in only:
            continue
        rows.append(row)
        if limit is not None and len(rows) >= limit:
            break
    if start_after is not None and not seen_start:
        raise SystemExit(f"--start-after source_id not found: {start_after}")
    return rows


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--only", action="append", default=[], help="run only this source_id")
    parser.add_argument("--start-after", help="resume after this source_id in ingest_plan order")
    parser.add_argument("--limit", type=int, help="maximum number of sources to process")
    parser.add_argument("--dry-run", action="store_true", help="print actions without running ingest")
    args = parser.parse_args()

    if args.limit is not None and args.limit < 1:
        raise SystemExit("--limit must be >= 1")

    env = os.environ.copy()
    env.setdefault("KB_FROZEN_CLOCK", "1")
    ld_parts = [part for part in env.get("LD_LIBRARY_PATH", "").split(":") if part]
    if "/usr/lib" not in ld_parts:
        env["LD_LIBRARY_PATH"] = ":".join(["/usr/lib", *ld_parts])

    verify_pdfium_library()

    prefix = kb_prefix()
    plan = filtered_plan(load_plan(), set(args.only), args.start_after, args.limit)
    if not plan:
        print("No matching sources to ingest.")
        return 0

    for row in plan:
        source_id = str(row["source_id"])
        pdf_path = str(row["canonical_path"])
        out_dir = ROOT / str(row["out_dir"])

        if complete(out_dir):
            print(f"skip complete: {source_id}")
            continue
        if has_partial_output(out_dir):
            print(
                f"partial output present for {source_id}: {out_dir}; "
                "inspect or move it before rerunning",
                file=sys.stderr,
            )
            return 2

        cmd = [
            *prefix,
            "ingest",
            pdf_path,
            "--source-id",
            source_id,
            "--out",
            str(Path(str(row["out_dir"]))),
        ]
        print("run:", " ".join(cmd))
        if not args.dry_run:
            subprocess.run(cmd, cwd=ROOT, env=env, check=True)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
