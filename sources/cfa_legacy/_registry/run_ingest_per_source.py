#!/usr/bin/env python3
"""Run the CFA legacy per-source PDF ingest plan safely.

This wrapper keeps the generated ingest process resumable: complete per-source
outputs are skipped, while partial outputs stop the run for manual inspection.

Before ingesting it refuses to run unless the libpdfium identity is
reproducibility-ready under the SAME proof-aware gate the reproducibility lock
uses (``check_ingest_reproducibility_lock.run_check``): the host build must match
the pin, or be a proven-equivalent build recorded in the committed equivalence
proof. Sharing that one policy avoids a second, drift-prone SHA-only check.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
REGISTRY = ROOT / "sources/cfa_legacy/_registry"
PLAN_PATH = REGISTRY / "ingest_plan.json"

# Reuse the ONE proof-aware reproducibility gate rather than maintaining a second
# SHA-only policy here (so the wrapper accepts a proven-equivalent host libpdfium
# exactly as the lock does, and still rejects a drifted/unproven one).
sys.path.insert(0, str(REGISTRY))
from check_ingest_reproducibility_lock import run_check  # noqa: E402


def load_plan() -> list[dict[str, object]]:
    with PLAN_PATH.open("r", encoding="utf-8") as f:
        plan = json.load(f)
    if not isinstance(plan, list):
        raise SystemExit(f"{PLAN_PATH}: expected a JSON list")
    return plan


def verify_pdfium_library() -> None:
    """Refuse to ingest unless the libpdfium identity is reproducibility-ready.

    Routes through the shared reproducibility-lock gate in LIBRARY-readiness mode:
    the host build must match the pin OR be a proven-equivalent build recorded in
    the committed equivalence proof (a blind ``KB_SKIP_PDFIUM_HASH_CHECK=1`` is
    honored by that gate as an explicitly recorded deviation). A drifted, unproven
    parser aborts before any chunk is written.

    This is the PRE-ingest gate, so it requires LIBRARY readiness (libpdfium + kb +
    pdfium-render identity) only — NOT the binary-independent recipe proof over
    ``chunks_manifest.json``. In a clean rebuild that manifest does not yet exist
    (it is produced by this very ingest+merge); requiring it here would block the
    rebuild on the file it is supposed to create.
    """
    status, ok = run_check(require_ingest_ready=False, require_library_ready=True, write_status=False)
    if ok:
        return
    lib = status["libpdfium"]
    raise SystemExit(
        "refusing to ingest: libpdfium is not reproducibility-ready. "
        f"libpdfium={lib['status']} (observed={lib['observed_sha256']} pin={lib['pinned_sha256']}); "
        f"kb={status['kb_binary']['status']}; pdfium_render_ok={status['pdfium_render']['ok']}. "
        "Provision the pinned or a proven-equivalent libpdfium (see "
        "check_ingest_reproducibility_lock.py --require-ingest-ready), or set "
        "KB_SKIP_PDFIUM_HASH_CHECK=1 to record an explicit non-reproducible deviation."
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
