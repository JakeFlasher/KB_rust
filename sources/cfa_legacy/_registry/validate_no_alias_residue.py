#!/usr/bin/env python3
"""Bare-alias notes-citation defense (CFA-legacy migration pipeline).

The legacy KB's `NOTES-001/002` lint rules are path-prefix checks (`notes/...`).
Some legacy cards bypass that check by citing the same content via the bare
alias `CFA_note_2 (2026 OCR) pp.115-116` — a citation string with no `notes/`
path prefix. The cacg.v0 source_id regex `^[a-z0-9][a-z0-9_]*$` structurally
rejects this form when it appears in a structured `source_id:` field (capital
`CFA` fails the regex), but the alias can still appear in card BODY prose
where it isn't a citation field but reads like one.

This validator scans cacg.v0 cards and `_legacy_reference/` copies of legacy
cards for the bare-alias pattern. It is a CFA-legacy migration safeguard,
not a framework-generic rule, and intentionally lives outside cacg-core's
neutral lint surface (see `crates/cacg-core/src/lint/mod.rs` for that
surface's scope).

Defense-in-depth layers:
  1. cacg.v0 source_id regex (in `crates/cacg-core/src/schema.rs`) —
     structurally rejects `CFA_note_N` in any `source_id` field.
  2. `legacy_notes_taint_manifest.json` — substring marker `CFA_note`
     quarantines alias-bearing legacy cards before they enter the
     migration queue.
  3. `_legacy_reference/` scrubbed copies — alias residue removed from
     the 4 affected legacy files (cc-material card, _chapter_overviews
     L222, _style_guide L151, _source_role_map L53).
  4. This validator — pipeline check before any per-slice emit.

Usage:
    python3 validate_no_alias_residue.py [--paths PATH [PATH ...]]

Exits 0 if no residue; non-zero with a per-file report otherwise.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ALIAS_RE = re.compile(r"\bCFA_note_\d+\s*\(")

DEFAULT_ROOTS = [
    "_legacy_reference",
    "cards/cfa_legacy",
    "sources/cfa_legacy/_registry/page_coordinate_maps",
]


def scan_file(path: Path) -> list[tuple[int, str]]:
    hits: list[tuple[int, str]] = []
    try:
        text = path.read_text(encoding="utf-8")
    except (UnicodeDecodeError, OSError):
        return hits
    for i, line in enumerate(text.splitlines(), 1):
        if ALIAS_RE.search(line):
            hits.append((i, line.strip()))
    return hits


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--paths",
        nargs="*",
        default=None,
        help="Paths to scan (relative to workspace root). Defaults cover the migration surface.",
    )
    parser.add_argument(
        "--workspace",
        default=str(Path(__file__).resolve().parents[3]),
        help="Workspace root (default: derived from script path).",
    )
    args = parser.parse_args()

    workspace = Path(args.workspace).resolve()
    roots = args.paths or DEFAULT_ROOTS

    total_hits = 0
    file_count = 0
    for root in roots:
        base = workspace / root
        if not base.exists():
            continue
        targets = [base] if base.is_file() else sorted(p for p in base.rglob("*") if p.is_file())
        for p in targets:
            if p.suffix not in {".md", ".json", ".yaml", ".yml", ".txt"}:
                continue
            file_count += 1
            hits = scan_file(p)
            if hits:
                rel = p.relative_to(workspace)
                print(f"\n{rel}:")
                for line_no, line in hits:
                    print(f"  L{line_no}: {line}")
                total_hits += len(hits)

    if total_hits:
        print(f"\nFAIL: {total_hits} bare-alias residue match(es) across {file_count} files.")
        return 1
    print(f"OK: scanned {file_count} files; no bare-alias residue.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
