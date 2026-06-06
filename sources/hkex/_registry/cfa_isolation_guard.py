#!/usr/bin/env python3
"""CFA isolation baseline + guard for the standalone ``hkex`` deck.

The ``hkex`` deck is built alongside the frozen ``cfa`` deck and must never
mutate ``cards/cfa``, ``out/cfa``, or ``sources/cfa``. This guard captures a
committed fingerprint of the pre-work ``cfa`` state and later re-checks that
fingerprint is unchanged.

  --capture  Record the ``cfa`` fingerprint to ``cfa_isolation_baseline.json``
             (active-card count, per-card ``card_hash`` map, the frozen
             ``EXPECTED_ACTIVE``/``EXPECTED_BASELINE`` constants, and the git
             tree object ids of the three ``cfa`` paths at HEAD).
  --check    Fail closed unless: the working tree has no diff under the three
             ``cfa`` paths; the active-card count still equals the recorded
             count; every recorded ``card_hash`` is byte-identical; and each
             recorded git tree id still matches HEAD (no committed drift).

The count derivation mirrors ``run_corpus_gate.sh`` (``*.md`` excluding
history sidecars / ``INDEX.md`` / ``_*``) so the fingerprint is data-driven,
not a hard-coded literal (cf. BL-20260531-milestone-count-bump-via-data-driven-gates).
"""
from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
CFA_PATHS = ("cards/cfa", "out/cfa", "sources/cfa")
BASELINE = ROOT / "sources/hkex/_registry/cfa_isolation_baseline.json"

# Frozen constants for the cfa deck (immutable; the hkex work must not move them).
EXPECTED_ACTIVE = 500
EXPECTED_BASELINE = 268

_FM_ID = re.compile(r'^id:\s*"?([^"\n]+)"?\s*$', re.M)
_FM_HASH = re.compile(r'^card_hash:\s*"?([0-9a-f]{64})"?\s*$', re.M)


def _git(*args: str) -> str:
    return subprocess.run(
        ["git", *args], cwd=ROOT, capture_output=True, text=True, check=True
    ).stdout.strip()


def active_card_files(cards_dir: Path) -> list[Path]:
    """Active card files under ``cards_dir``: ``*.md`` excluding history
    sidecars, ``INDEX.md``, and ``_*`` (matches run_corpus_gate.sh)."""
    out = []
    for p in sorted(cards_dir.rglob("*.md")):
        name = p.name
        if name == "INDEX.md" or name.startswith("_") or ".history" in name:
            continue
        out.append(p)
    return out


def card_hash_map(cards_dir: Path) -> dict[str, str]:
    mapping: dict[str, str] = {}
    for p in active_card_files(cards_dir):
        text = p.read_text(encoding="utf-8")
        mid = _FM_ID.search(text)
        mh = _FM_HASH.search(text)
        if not mid:
            raise SystemExit(f"card without id: {p}")
        cid = mid.group(1).strip()
        if cid in mapping:
            raise SystemExit(f"duplicate card id {cid} at {p}")
        mapping[cid] = mh.group(1) if mh else ""
    return mapping


def tree_ids() -> dict[str, str]:
    ids = {}
    for rel in CFA_PATHS:
        try:
            ids[rel] = _git("rev-parse", f"HEAD:{rel}")
        except subprocess.CalledProcessError:
            ids[rel] = ""  # path not present in HEAD tree
    return ids


def capture() -> dict:
    cards = card_hash_map(ROOT / "cards/cfa")
    fp = {
        "schema_version": "hkex.cfa_isolation_baseline.v1",
        "captured_at_head": _git("rev-parse", "HEAD"),
        "expected_active": EXPECTED_ACTIVE,
        "expected_baseline": EXPECTED_BASELINE,
        "active_card_count": len(cards),
        "cfa_tree_ids": tree_ids(),
        "card_hashes": dict(sorted(cards.items())),
    }
    if len(cards) != EXPECTED_ACTIVE:
        raise SystemExit(
            f"refusing to capture: active cfa cards={len(cards)} != EXPECTED_ACTIVE={EXPECTED_ACTIVE}"
        )
    hashless = sorted(cid for cid, h in cards.items() if not h)
    if hashless:
        raise SystemExit(
            f"refusing to capture: {len(hashless)} active cfa card(s) lack card_hash "
            f"(e.g. {hashless[:3]}); index the cfa deck first"
        )
    BASELINE.write_text(
        json.dumps(fp, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    return fp


def check() -> int:
    if not BASELINE.is_file():
        print(f"FAIL: no baseline at {BASELINE} (run --capture first)", file=sys.stderr)
        return 1
    base = json.loads(BASELINE.read_text(encoding="utf-8"))
    failures: list[str] = []

    # 1a. No working-tree diff (staged or unstaged) under the cfa paths.
    diff = subprocess.run(
        ["git", "diff", "--quiet", "HEAD", "--", *CFA_PATHS], cwd=ROOT
    )
    if diff.returncode != 0:
        failures.append("working-tree diff present under cards/cfa|out/cfa|sources/cfa")
    # 1b. No UNTRACKED files under the cfa paths (git diff ignores untracked; an
    # untracked sidecar / _* / INDEX.md would otherwise slip past every check).
    others = subprocess.run(
        ["git", "ls-files", "--others", "--exclude-standard", "--", *CFA_PATHS],
        cwd=ROOT, capture_output=True, text=True,
    ).stdout.strip()
    if others:
        failures.append(f"untracked file(s) under cfa paths: {others.splitlines()[:3]}")

    # 2. Count unchanged.
    cards = card_hash_map(ROOT / "cards/cfa")
    if len(cards) != base["active_card_count"]:
        failures.append(f"active cfa card count {len(cards)} != baseline {base['active_card_count']}")
    if len(cards) != EXPECTED_ACTIVE:
        failures.append(f"active cfa card count {len(cards)} != EXPECTED_ACTIVE {EXPECTED_ACTIVE}")

    # 3. Per-card hash unchanged.
    base_hashes = base["card_hashes"]
    if set(cards) != set(base_hashes):
        failures.append("cfa card id-set changed vs baseline")
    for cid, h in base_hashes.items():
        if cards.get(cid) != h:
            failures.append(f"cfa card_hash changed: {cid}")
            break

    # 4. Committed tree ids unchanged.
    now = tree_ids()
    for rel, tid in base["cfa_tree_ids"].items():
        if now.get(rel) != tid:
            failures.append(f"cfa tree id changed at HEAD: {rel}")

    if failures:
        print("CFA ISOLATION: FAIL", file=sys.stderr)
        for f in failures:
            print("  -", f, file=sys.stderr)
        return 1
    print(f"CFA ISOLATION: PASS ({len(cards)} active / baseline {base['expected_baseline']}; tree ids stable)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--capture", action="store_true")
    g.add_argument("--check", action="store_true")
    args = ap.parse_args()
    if args.capture:
        fp = capture()
        print(json.dumps({k: v for k, v in fp.items() if k != "card_hashes"}, ensure_ascii=False, indent=2, sort_keys=True))
        print(f"(captured {fp['active_card_count']} card_hashes -> {BASELINE.relative_to(ROOT)})")
        return 0
    return check()


if __name__ == "__main__":
    raise SystemExit(main())
