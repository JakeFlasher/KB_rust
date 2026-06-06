#!/usr/bin/env python3
"""Deck path discipline for the ``hkex`` deck (scaffold guard).

Every ``hkex`` card, source PDF, ingest output, and registry helper lives under
an ``hkex`` namespace and must NEVER be written into a ``cfa`` path. Authoring
helpers call :func:`guard_card_target` before writing any card so a card can
never land in ``cards/cfa/**`` (AC-1 negative test). Read-only references to the
``cfa`` registry are allowed; writes are not.
"""
from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]

HKEX_CARDS = ROOT / "cards/hkex"
HKEX_SOURCES = ROOT / "sources/hkex"
HKEX_OUT = ROOT / "out/hkex"

# Any write under one of these forbidden roots is rejected.
FORBIDDEN_WRITE_ROOTS = (
    ROOT / "cards/cfa",
    ROOT / "out/cfa",
    ROOT / "sources/cfa",
)


def _is_within(child: Path, parent: Path) -> bool:
    try:
        child.resolve().relative_to(parent.resolve())
        return True
    except ValueError:
        return False


def guard_card_target(path: str | Path) -> Path:
    """Return the resolved card path iff it is under ``cards/hkex``; else raise.

    Rejects any path under a forbidden ``cfa`` root, and any path not under
    ``cards/hkex`` (so a typo'd reading folder outside the deck also fails).
    """
    p = Path(path)
    for root in FORBIDDEN_WRITE_ROOTS:
        if _is_within(p, root):
            raise ValueError(f"refusing to write a card into a cfa path: {p} (under {root})")
    if not _is_within(p, HKEX_CARDS):
        raise ValueError(f"card target must live under cards/hkex/: {p}")
    return p.resolve()


def guard_write_target(path: str | Path) -> Path:
    """Reject any write under a forbidden ``cfa`` root (sources/out/cards)."""
    p = Path(path)
    for root in FORBIDDEN_WRITE_ROOTS:
        if _is_within(p, root):
            raise ValueError(f"refusing to write into a cfa path: {p} (under {root})")
    return p.resolve()


def _self_test() -> int:
    failures: list[str] = []

    # Accept a valid hkex card path.
    try:
        guard_card_target(HKEX_CARDS / "07_derivatives_and_volatility" / "gbj-x.md")
    except ValueError as e:
        failures.append(f"valid hkex card path rejected: {e}")

    # Reject a cfa card path.
    try:
        guard_card_target(ROOT / "cards/cfa" / "07_derivatives_and_volatility" / "x.md")
        failures.append("cfa card path was NOT rejected")
    except ValueError:
        pass

    # Reject a non-hkex, non-cfa card path.
    try:
        guard_card_target(ROOT / "cards" / "stray.md")
        failures.append("stray card path was NOT rejected")
    except ValueError:
        pass

    # Reject writes into cfa sources/out.
    for bad in (ROOT / "out/cfa/x.json", ROOT / "sources/cfa/_registry/x.py"):
        try:
            guard_write_target(bad)
            failures.append(f"cfa write target NOT rejected: {bad}")
        except ValueError:
            pass

    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print("  -", f)
        return 1
    print("SELF-TEST PASSED (deck_paths scaffold guard)")
    return 0


if __name__ == "__main__":
    sys.exit(_self_test())
