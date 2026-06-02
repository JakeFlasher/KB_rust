#!/usr/bin/env python3
"""Validate the migrated-readings bidirectional cross-links (fail-closed gate).

Checks, all re-derived from the committed map + the cards on disk (never trusting
the map's own prose):

  1. The explicit ``released_card_allowlist`` equals the released-card set DERIVED
     from ``documented_pairs``.
  2. Every documented pair is linked in BOTH directions: each endpoint's on-disk
     ``## See Also`` carries a resolving markdown link to the other endpoint.
  3. Every ``## See Also`` markdown link in the migrated cards and the edited
     released cards resolves to an existing card file on disk.
  4. The set of changed pre-existing (released) cards under ``cards/cfa``
     equals the allowlist exactly (an off-allowlist released-card edit fails).
  5. No migrated or edited card declares frontmatter ``card_edges`` (prose only).

Exit 0 iff every check passes; non-zero with a diagnostic otherwise.
"""

from __future__ import annotations

import argparse
import re
import subprocess
from pathlib import Path

from cross_link_map import (
    DEFAULT_MAP,
    NEW_READINGS,
    derive_links,
    has_resolving_link,
    load_map,
    prose_rewrites_by_card,
    reading_of,
    see_also_bounds,
)

ROOT = Path(__file__).resolve().parents[3]
CARDS_ROOT = ROOT / "cards/cfa"
LINK_RE = re.compile(r"\[[^\]]*\]\(([^)]+)\)")


def card_path(card_id: str) -> Path:
    return CARDS_ROOT / reading_of(card_id) / f"{card_id}.md"


def split_card(path: Path) -> tuple[str, str]:
    parts = path.read_text(encoding="utf-8").split("---", 2)
    if len(parts) < 3:
        raise SystemExit(f"{path}: malformed frontmatter")
    return parts[1], parts[2]


def see_also_section(body: str) -> str:
    bounds = see_also_bounds(body)
    return body[bounds[0]:bounds[1]] if bounds else ""


def see_also_link_dests(body: str) -> list[str]:
    return LINK_RE.findall(see_also_section(body))


def migrated_card_paths() -> list[Path]:
    out: list[Path] = []
    for reading in sorted(NEW_READINGS):
        for md in sorted((CARDS_ROOT / reading).glob("*.md")):
            if md.name == "INDEX.md" or md.name.startswith("_") or ".history" in md.name:
                continue
            out.append(md)
    return out


def changed_card_ids(baseline: str) -> set[str]:
    """Card ids whose CONTENT differs from the released baseline (working tree vs
    ``baseline``, default the v0 release tag). Rename-robust: the ``cfa_legacy -> cfa``
    consolidation moved every card path, but a pure rename (git status ``R100``,
    identical bytes) is NOT a content change and is excluded; only in-place
    modifications (``M``) and renames-with-edits (``R<100``) count. Brand-new cards
    (status ``A``) are additions, not changes to released cards, so they are excluded
    too. Measured against the baseline so the check is stable before AND after the edits
    are committed."""
    res = subprocess.run(
        ["git", "-C", str(ROOT), "rev-parse", "--verify", "--quiet", baseline + "^{commit}"],
        capture_output=True, text=True,
    )
    if res.returncode != 0:
        raise SystemExit(f"baseline ref {baseline!r} not found; pass --baseline")
    # Include BOTH the post-rename root and the historical root so git pairs renames
    # (the baseline tree predates the rename and stores cards under cards/cfa_legacy).
    diff = subprocess.run(
        ["git", "-C", str(ROOT), "diff", "--name-status", "-M", baseline,
         "--", "cards/cfa", "cards/cfa_legacy"],
        capture_output=True, text=True, check=True,
    )

    def is_card(p: str) -> bool:
        return p.endswith(".md") and not p.endswith("INDEX.md") and "/_" not in p

    ids: set[str] = set()
    for line in diff.stdout.splitlines():
        parts = line.split("\t")
        status = parts[0]
        if status == "M" and len(parts) >= 2 and is_card(parts[1]):
            ids.add(Path(parts[1]).stem)               # modified in place
        elif status.startswith("R") and len(parts) >= 3:
            sim = int(status[1:]) if status[1:].isdigit() else 100
            if sim < 100 and is_card(parts[2]):
                ids.add(Path(parts[2]).stem)            # renamed AND edited
        # 'A' (new card), 'D', and 'R100' (pure rename, identical content) are not
        # changes to a released card.
    return ids


def run_checks(cross_links_path: Path, baseline: str) -> list[str]:
    failures: list[str] = []
    doc = load_map(cross_links_path)
    released = derive_links(doc)[1]

    # 1. explicit allowlist == derived released set.
    allowlist = set(doc.get("released_card_allowlist", {}))
    if allowlist != set(released):
        failures.append(f"released allowlist {sorted(allowlist)} != derived {sorted(released)}")

    # 2. every documented pair linked both ways (primary <-> each counterpart).
    for pair in doc["documented_pairs"]:
        for c in pair["counterparts"]:
            for src, dst in ((pair["primary"], c), (c, pair["primary"])):
                body = split_card(card_path(src))[1]
                if not has_resolving_link(see_also_section(body), dst):
                    failures.append(f"{src}: missing resolving See Also link to {dst}")

    # 2b. A withdrawn/over-claimed counterpart named in a committed prose rewrite must NOT
    # survive in the migrated card's See Also as a bare code-span / stale prose (link
    # injection only converts/appends links, so it cannot remove an over-claim sentence).
    for cid, rewrites in prose_rewrites_by_card(doc).items():
        section = see_also_section(split_card(card_path(cid))[1])
        for rw in rewrites:
            for forbidden in rw.get("forbidden_see_also_mentions", []):
                if forbidden in section:
                    failures.append(
                        f"{cid}: withdrawn cross-link mention still present in See Also: {forbidden!r}")

    # 3 + 5. links resolve; no card_edges; over the migrated cards + allowlisted released cards.
    to_scan = migrated_card_paths() + [card_path(cid) for cid in sorted(allowlist)]
    for md in to_scan:
        fm, body = split_card(md)
        if re.search(r"(?m)^card_edges:", fm):
            failures.append(f"{md.stem}: frontmatter declares card_edges (prose links only)")
        for dest in see_also_link_dests(body):
            target = dest.split("#", 1)[0]
            if not target.endswith(".md"):
                continue
            if not (md.parent / target).resolve().is_file():
                failures.append(f"{md.stem}: See Also link does not resolve: {dest}")

    # 4. only allowlisted released (non-migrated-reading) cards changed vs baseline.
    changed_released = {
        cid for cid in changed_card_ids(baseline) if reading_of_safe(cid) not in NEW_READINGS
    }
    if changed_released != allowlist:
        failures.append(
            f"changed released cards {sorted(changed_released)} != allowlist {sorted(allowlist)}"
        )
    return failures


def reading_of_safe(card_id: str) -> str:
    try:
        return reading_of(card_id)
    except ValueError:
        return "?"


def _self_test() -> int:
    # has_resolving_link: link present vs only a code-span.
    assert has_resolving_link("- [`x`](../r/x.md) — n", "x")
    assert not has_resolving_link("- `x` (prose only)", "x")
    # see_also_link_dests strips to the section and finds dests; fragments kept.
    body = "# T\n## See Also\n- [`a`](./a.md) — n\n- [`b`](../r/b.md#sec) — n\n## Z\n- [`c`](c.md)\n"
    dests = see_also_link_dests(body)
    assert dests == ["./a.md", "../r/b.md#sec"], dests  # 'c' is past the section
    # reading_of_safe degrades on unknown prefix instead of raising.
    assert reading_of_safe("zz-foo") == "?"
    assert reading_of_safe("mt-foo") == "14_microstructure_and_trading"
    # forbidden-mention grouping + detection logic.
    rw = prose_rewrites_by_card({"skeleton_prose_rewrites": [
        {"card_id": "c", "forbidden_see_also_mentions": ["x-withdrawn"]}]})
    section = "- [`a`](./a.md) — n\n- x-withdrawn still named in prose\n"
    hits = [f for f in rw["c"][0]["forbidden_see_also_mentions"] if f in section]
    assert hits == ["x-withdrawn"], "forbidden-mention detection broken"
    assert prose_rewrites_by_card({}) == {}
    print("check_migration_cross_links self-test: PASS")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--cross-links", type=Path, default=DEFAULT_MAP)
    ap.add_argument("--baseline", default="v0-candidate",
                    help="released-corpus baseline ref to diff released-card edits against "
                         "(default: v0-candidate)")
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        return _self_test()
    failures = run_checks(args.cross_links, args.baseline)
    if failures:
        print("CROSS-LINK GATE: FAIL")
        for f in failures:
            print(f"  - {f}")
        return 1
    print("CROSS-LINK GATE: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
