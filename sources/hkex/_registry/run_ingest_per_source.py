#!/usr/bin/env python3
r"""Deck-local per-source ingest runner for the ``hkex`` deck.

Adapted from (never mutating) ``sources/cfa/_registry/run_ingest_per_source.py``. Drives
``ingest_plan.json``: each row is ingested by the Rust ``kb ingest`` into its own
``out/hkex/ingest_per_source/<source_id>/`` directory with a per-source chunk ``--config``.

Fail-closed / no-clobber discipline (cf. BL-20260518-no-clobber-foreign-sidecars):
  * Two plan rows may never share a ``source_id`` or an ``out_dir``.
  * A row is SKIPPED if its ``out_dir`` already holds a COMPLETE ingest of the SAME
    ``source_id`` (resumable); ``--force`` regenerates it.
  * Ingest is REFUSED if the ``out_dir`` already holds a manifest for a DIFFERENT
    ``source_id`` (cross-source clobber) or partial output.

libpdfium is pinned by the documented deviation ``KB_SKIP_PDFIUM_HASH_CHECK=1`` (the same
disposition the AC-2/AC-3 gates use; recorded in the merge report). ``kb`` is resolved via
the shared ``KB_BIN`` -> ``target/debug/kb`` -> ``target/release/kb`` fail-closed policy.
"""
from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
REGISTRY = ROOT / "sources/hkex/_registry"
PLAN_PATH = REGISTRY / "ingest_plan.json"
sys.path.insert(0, str(REGISTRY))
from check_corpus_parity import resolve_kb  # noqa: E402  # pyright: ignore[reportMissingImports]


def load_plan() -> list[dict]:
    plan = json.loads(PLAN_PATH.read_text(encoding="utf-8"))
    if not isinstance(plan, list):
        raise SystemExit(f"{PLAN_PATH}: expected a JSON list")
    ids, dirs = set(), set()
    for row in plan:
        sid, od = str(row["source_id"]), str(row["out_dir"])
        if sid in ids:
            raise SystemExit(f"duplicate source_id in plan: {sid}")
        if od in dirs:
            raise SystemExit(f"two plan rows share out_dir {od}: clobber refused")
        ids.add(sid); dirs.add(od)
    return plan


def manifest_source_id(out_dir: Path) -> str | None:
    sm = out_dir / "sources_manifest.json"
    if not sm.is_file():
        return None
    try:
        srcs = json.loads(sm.read_text(encoding="utf-8")).get("sources", [])
        return str(srcs[0]["source_id"]) if srcs else None
    except (json.JSONDecodeError, KeyError, IndexError):
        return None


def complete(out_dir: Path) -> bool:
    return (out_dir / "sources_manifest.json").is_file() and (out_dir / "chunks_manifest.json").is_file()


def env_for_kb() -> dict:
    env = os.environ.copy()
    env.setdefault("KB_FROZEN_CLOCK", "1")
    env.setdefault("KB_SKIP_PDFIUM_HASH_CHECK", "1")
    parts = [p for p in env.get("LD_LIBRARY_PATH", "").split(":") if p]
    if "/usr/lib" not in parts:
        env["LD_LIBRARY_PATH"] = ":".join(["/usr/lib", *parts])
    return env


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--only", action="append", default=[], help="run only this source_id")
    ap.add_argument("--force", action="store_true", help="regenerate a complete same-source ingest")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    kb = resolve_kb()
    env = env_for_kb()
    plan = load_plan()
    only = set(args.only)
    did = []
    for row in plan:
        sid = str(row["source_id"])
        if only and sid not in only:
            continue
        out_dir = ROOT / str(row["out_dir"])
        pdf = ROOT / str(row["canonical_path"])

        existing = manifest_source_id(out_dir)
        if existing is not None and existing != sid:
            raise SystemExit(f"clobber refused: {out_dir} holds source_id {existing!r}, not {sid!r}")
        if complete(out_dir) and not args.force:
            print(f"skip complete: {sid}")
            continue
        if out_dir.exists() and not complete(out_dir) and any(out_dir.iterdir()) and not args.force:
            raise SystemExit(f"partial output for {sid}: {out_dir}; inspect or --force")
        if not pdf.is_file():
            raise SystemExit(f"source PDF missing for {sid}: {pdf} (render/snapshot it first)")

        # A dry run must NEVER mutate the filesystem: reaching the rmtree below under
        # `--dry-run --force` would clobber a reusable per-source manifest the command was
        # only meant to describe. Gate ALL mutation (rmtree/mkdir/ingest) behind this check.
        if args.dry_run:
            cfgnote = " --config <per-source>" if row.get("config") else ""
            print(f"dry-run: would (re)ingest {sid}: kb ingest {row['canonical_path']} "
                  f"--source-id {sid} --out {row['out_dir']}{cfgnote}  [clobber {row['out_dir']}]")
            continue

        if out_dir.exists():
            import shutil
            shutil.rmtree(out_dir)
        out_dir.mkdir(parents=True)
        with tempfile.TemporaryDirectory() as td:
            extra = []
            if row.get("config"):
                cfg = Path(td) / "cfg.json"
                cfg.write_text(json.dumps(row["config"]), encoding="utf-8")
                extra = ["--config", str(cfg)]
            # Relative paths + cwd=ROOT so the per-source manifest records the canonical
            # (relative) source_path the merge validates against.
            cmd = [str(kb), "ingest", str(row["canonical_path"]), "--source-id", sid,
                   "--out", str(row["out_dir"]), *extra]
            print("run:", " ".join(cmd))
            subprocess.run(cmd, cwd=ROOT, env=env, check=True)
            did.append(sid)
    print(json.dumps({"ingested": did, "kb": str(kb)}, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
