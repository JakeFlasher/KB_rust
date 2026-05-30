#!/usr/bin/env python3
"""v1 count/constant sweep (structured, fail-closed) — NOT a raw grep.

Two halves:

  (A) DATA AGREEMENT — derive the live v1 counts from the authoritative artifacts and
      assert they equal the expected v1 literals AND that the v1 recipe states them:
        active   = len(cards_manifest.json.cards)            -> 402
        total    = active + |quarantine pin|                  -> 408
        sources  = #rows in ingest_plan.json                  -> 87
        readings = #reading dirs under cards/cfa_legacy        -> 14

  (B) NO STALE LOAD-BEARING CONSTANT — scan the gate scripts for a hardcoded v0
      active/total/source count used in a COMPARISON (e.g. `!= 268`, `== 274`,
      `default=70` for the merge count). Those were refactored to derive from data;
      a re-introduced literal fails here. Historical prose in the v0 research docs +
      self-test fixtures is allowed (it is not load-bearing on a gate).

Run with no args for the real check; `--self-test` for hermetic probes.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
REGISTRY = REPO / "sources/cfa_legacy/_registry"
EXPECTED = {"active": 402, "total": 408, "sources": 87, "readings": 14}

# Gate/recipe scripts that must not hard-code a load-bearing v0 count (comparison or
# default). Includes merge_ingest_manifests.py so the v1 source-count default (87) is
# covered — a reintroduced --require-count default of 70 fails the sweep.
GATE_SCRIPTS = [
    "check_release_cleanliness.py", "check_decision_ledger.py", "build_scope_ledger.py",
    "check_quarantine_invariant.py", "check_index_reproducible.py", "merge_ingest_manifests.py",
]
# Patterns that would be a STALE load-bearing v0 constant (comparison / default).
STALE_PATTERNS = [
    (r"[!=]=\s*268\b", "active-count comparison against 268"),
    (r"[!=]=\s*274\b", "legacy-total comparison against 274"),
    (r"[!=]=\s*70\b", "source-count comparison against 70"),
    (r"default\s*=\s*70\b", "merge --require-count default of 70"),
]


def derive_counts() -> dict[str, int]:
    from check_quarantine_invariant import QUARANTINE_IDS
    manifest = json.loads((REPO / "out/cfa_legacy/cards_manifest.json").read_text(encoding="utf-8"))
    active = len(manifest["cards"])
    ingest = json.loads((REGISTRY / "ingest_plan.json").read_text(encoding="utf-8"))
    rows = ingest if isinstance(ingest, list) else ingest.get("sources") or ingest.get("rows") or ingest.get("plan") or []
    cards_dir = REPO / "cards/cfa_legacy"
    readings = sum(1 for d in cards_dir.iterdir() if d.is_dir() and not d.name.startswith("_"))
    return {"active": active, "total": active + len(set(QUARANTINE_IDS)),
            "sources": len(rows), "readings": readings}


def stale_hits(text: str) -> list[str]:
    hits: list[str] = []
    for pat, desc in STALE_PATTERNS:
        if re.search(pat, text):
            hits.append(desc)
    return hits


def run_checks() -> list[str]:
    failures: list[str] = []
    # (A) data agreement.
    counts = derive_counts()
    for k, want in EXPECTED.items():
        if counts[k] != want:
            failures.append(f"derived {k}={counts[k]} != expected v1 {want}")
    recipe = (REPO / "_research/31_v1_release_and_rebuild_recipe.md").read_text(encoding="utf-8") \
        if (REPO / "_research/31_v1_release_and_rebuild_recipe.md").is_file() else ""
    if not recipe:
        failures.append("v1 recipe _research/31_*.md missing")
    else:
        for k, want in EXPECTED.items():
            if not re.search(rf"\b{want}\b", recipe):
                failures.append(f"v1 recipe does not state {k}={want}")
    # (B) no stale load-bearing constant in the gate scripts.
    for s in GATE_SCRIPTS:
        p = REGISTRY / s
        if not p.is_file():
            failures.append(f"gate script missing: {s}")
            continue
        for desc in stale_hits(p.read_text(encoding="utf-8")):
            failures.append(f"{s}: stale load-bearing v0 constant ({desc})")
    return failures


def _self_test() -> int:
    ok = True
    # stale_hits fires on a hardcoded comparison and is clean on the data-driven form.
    if not stale_hits("if disk_sidecars != 268:"):
        print("  FAIL: stale 268 comparison not caught"); ok = False
    if stale_hits("active = len(manifest['cards'])  # 268 at v0, 402 at v1 (derived)"):
        # a comment mentioning 268 without a comparison is NOT load-bearing -> must be clean
        print("  FAIL: a non-comparison mention of 268 was flagged"); ok = False
    if not stale_hits("parser.add_argument('--require-count', default=70)"):
        print("  FAIL: merge default=70 not caught"); ok = False
    if stale_hits("active_cards = 402"):
        print("  FAIL: a v1 literal was flagged as stale"); ok = False
    print("check_v1_constants self-test:", "PASS" if ok else "FAIL")
    return 0 if ok else 1


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        return _self_test()
    failures = run_checks()
    print("derived counts:", derive_counts(), "| expected:", EXPECTED)
    if failures:
        print("\nV1 CONSTANTS SWEEP: FAIL", file=sys.stderr)
        for f in failures:
            print(f"  - {f}", file=sys.stderr)
        return 1
    print("\nV1 CONSTANTS SWEEP: PASS (derived counts == v1 literals; recipe states them; "
          "no stale load-bearing v0 constant in the gate scripts)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
