#!/usr/bin/env python3
"""Shared helpers for the migrated-readings cross-link work.

One source of truth (`migration_cross_links.json`, the nine documented cross-set
pairs) is loaded here and the per-card link tables are DERIVED from it, so the
emitter (which injects new-card links), the released-card back-link applier, and
the validator all agree by construction rather than by three hand-kept copies.

Reading is derived from the card-id prefix; all documented links are cross-reading,
so the relative path from any card to a target is ``../<target-reading>/<id>.md``.
Links are rendered as prose markdown (no frontmatter ``card_edges``).
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

REGISTRY = Path(__file__).resolve().parent
DEFAULT_MAP = REGISTRY / "migration_cross_links.json"
MAP_SCHEMA_VERSION = "cfa_legacy.migration_cross_links.v1"

# Card-id prefix -> reading directory under cards/cfa_legacy/.
READING_OF_PREFIX = {
    "fa-": "22_fund_level_arbitrage",
    "mt-": "14_microstructure_and_trading",
    "pa-": "15_performance_and_attribution",
    "be-": "10_behavioral_finance",
}
# The three readings emitted by emit_migration_readings.py (their cards are
# rendered from skeletons, so their cross-links are injected at emit time). Any
# other reading (e.g. 10_behavioral_finance) is a released card edited in place.
NEW_READINGS = {
    "14_microstructure_and_trading",
    "15_performance_and_attribution",
    "22_fund_level_arbitrage",
}

SEE_ALSO_HEADING = "## See Also"


def reading_of(card_id: str) -> str:
    for prefix, reading in READING_OF_PREFIX.items():
        if card_id.startswith(prefix):
            return reading
    raise ValueError(f"unknown card-id prefix: {card_id!r}")


def is_new_card(card_id: str) -> bool:
    return reading_of(card_id) in NEW_READINGS


def relpath_to(target_id: str) -> str:
    """Relative markdown path from any card to ``target_id`` (all cross-reading)."""
    return f"../{reading_of(target_id)}/{target_id}.md"


def render_link(target_id: str, note: str) -> str:
    return f"- [`{target_id}`]({relpath_to(target_id)}) — {note}"


def load_map(path: Path = DEFAULT_MAP) -> dict[str, Any]:
    if not path.is_file():
        raise SystemExit(f"cross-link map is not a regular file: {path}")
    doc = json.loads(path.read_text(encoding="utf-8"))
    if doc.get("schema_version") != MAP_SCHEMA_VERSION:
        raise SystemExit(
            f"{path}: schema_version {doc.get('schema_version')!r} != {MAP_SCHEMA_VERSION!r}"
        )
    if not isinstance(doc.get("documented_pairs"), list) or not doc["documented_pairs"]:
        raise SystemExit(f"{path}: documented_pairs missing/empty")
    return doc


def derive_links(doc: dict[str, Any]) -> tuple[dict[str, list[dict]], dict[str, list[dict]]]:
    """Return (new_card_links, released_backlinks).

    For every documented pair, the primary card and each counterpart are linked in
    BOTH directions; a directional link lands in ``new_card_links`` if its source
    card is in a migrated reading, else in ``released_backlinks``. Each value is an
    ordered, de-duplicated list of ``{"target_id", "note"}``.
    """
    new_links: dict[str, list[dict]] = {}
    released: dict[str, list[dict]] = {}

    def add(src: str, dst: str, note: str) -> None:
        bucket = new_links if is_new_card(src) else released
        lst = bucket.setdefault(src, [])
        if not any(item["target_id"] == dst for item in lst):
            lst.append({"target_id": dst, "note": note})

    for pair in doc["documented_pairs"]:
        primary = pair["primary"]
        note = pair["note"]
        for counterpart in pair["counterparts"]:
            add(primary, counterpart, note)
            add(counterpart, primary, note)
    return new_links, released


def see_also_bounds(text: str) -> tuple[int, int] | None:
    """Return (start, end) byte offsets of the ``## See Also`` section body region
    (from just after the heading line to the next ``## `` heading or EOF), or None
    if the card has no See Also section."""
    i = text.find(SEE_ALSO_HEADING)
    if i < 0:
        return None
    body_start = text.find("\n", i)
    body_start = len(text) if body_start < 0 else body_start + 1
    nxt = text.find("\n## ", body_start)
    end = len(text) if nxt < 0 else nxt + 1
    return body_start, end


def has_resolving_link(section: str, target_id: str) -> bool:
    """True if the section already contains a resolving markdown link to target_id
    (i.e. a ``](...<target_id>.md)`` destination)."""
    return f"/{target_id}.md)" in section or f"]({target_id}.md)" in section


def inject_links(text: str, links: list[dict]) -> str:
    """Idempotently ensure the card's ``## See Also`` section carries a resolving
    link for each target. If a bare code-span ``` `<id>` ``` already references the
    target in prose, convert the first occurrence to a resolving link; otherwise
    append a new bullet. A target already linked is left untouched.

    The card MUST have a ``## See Also`` section (every migrated/edited card does).
    """
    bounds = see_also_bounds(text)
    if bounds is None:
        raise ValueError("card has no '## See Also' section")
    for link in links:
        target = link["target_id"]
        note = link["note"]
        start, end = bounds  # recompute-free: indices shift only at/after `end`
        section = text[start:end]
        if has_resolving_link(section, target):
            continue
        codespan = f"`{target}`"
        if codespan in section:
            new_section = section.replace(codespan, f"[`{target}`]({relpath_to(target)})", 1)
        else:
            block = section.rstrip("\n")
            new_section = (block + "\n" if block else "") + render_link(target, note) + "\n"
        text = text[:start] + new_section + text[end:]
        bounds = (start, start + len(new_section))
    return text
