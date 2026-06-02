#!/usr/bin/env python3
"""Apply the reciprocal cross-link back-links into the already-released cards.

The migrated readings 14/15/22 link out to a small number of already-released
cards (per the documented cross-set pairs); those released cards are not emitted
from skeletons, so their reciprocal ``## See Also`` back-link is added IN PLACE
here. Only cards on the committed allowlist in ``migration_cross_links.json`` are
touched, and the edit is idempotent (a back-link already present is left alone).

Editing a released card's body makes its stored ``card_hash`` stale; ``kb index``
re-stamps it as part of the release move (the single re-index absorbs this churn).
Run with ``--write`` to apply; default is a dry run that reports the planned edits.
"""

from __future__ import annotations

import argparse
from pathlib import Path

from cross_link_map import (
    DEFAULT_MAP,
    derive_links,
    inject_links,
    load_map,
    reading_of,
)

CARDS_ROOT = Path(__file__).resolve().parents[3] / "cards/cfa"


def write_atomic(path: Path, content: str) -> None:
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(content, encoding="utf-8")
    tmp.replace(path)


def card_path(card_id: str) -> Path:
    return CARDS_ROOT / reading_of(card_id) / f"{card_id}.md"


def plan_edits(cross_links_path: Path) -> dict[str, str]:
    """Return {card_id: new_text} for every allowlisted released card whose body
    must change. Fails closed if the explicit allowlist disagrees with the links
    derived from the documented pairs, or a target card is missing."""
    doc = load_map(cross_links_path)
    released = derive_links(doc)[1]
    allowlist = set(doc.get("released_card_allowlist", {}))
    if allowlist != set(released):
        raise SystemExit(
            f"released allowlist {sorted(allowlist)} != derived released cards {sorted(released)}"
        )
    edits: dict[str, str] = {}
    for card_id, links in released.items():
        for link in links:
            if not card_path(link["target_id"]).is_file():
                raise SystemExit(f"{card_id}: back-link target {link['target_id']} not found on disk")
        path = card_path(card_id)
        if not path.is_file():
            raise SystemExit(f"released card not found: {path}")
        text = path.read_text(encoding="utf-8")
        fm = text.split("---", 2)
        if len(fm) < 3:
            raise SystemExit(f"{path}: malformed frontmatter")
        if "card_edges" in fm[1]:
            raise SystemExit(f"{card_id}: frontmatter already declares card_edges (prose links only)")
        body = fm[2]
        new_body = inject_links(body, links)
        if new_body != body:
            edits[card_id] = "---" + fm[1] + "---" + new_body
    return edits


def _self_test() -> int:
    from cross_link_map import see_also_bounds
    body = "# T\n\n## See Also\n- [`x`](./x.md) — intra.\n\n## Escalate\nz\n"
    out = inject_links(body, [{"target_id": "fa-foo-bar", "note": "n"}])
    assert "[`fa-foo-bar`](../22_fund_level_arbitrage/fa-foo-bar.md) — n" in out
    assert inject_links(out, [{"target_id": "fa-foo-bar", "note": "n"}]) == out, "not idempotent"
    assert "## Escalate\nz" in out and see_also_bounds(out) is not None
    print("apply_released_card_backlinks self-test: PASS")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--cross-links", type=Path, default=DEFAULT_MAP)
    ap.add_argument("--write", action="store_true", help="apply the edits (default: dry run)")
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        return _self_test()
    edits = plan_edits(args.cross_links)
    for card_id, text in sorted(edits.items()):
        if args.write:
            write_atomic(card_path(card_id), text)
    print(f"released cards {'edited' if args.write else 'to edit'}: {sorted(edits)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
