#!/usr/bin/env python3
"""Deterministic, idempotent normalizer that conditions an incoming template deck
to the incoming-skeleton preflight contract (``build_incoming_skeleton_preflight.py``).

The Behavioral-Finance deck (reading ``10_behavioral_finance``) was authored to a
LOOSER contract than the preflight enforces. This applies three surgical,
FRONTMATTER-ONLY transforms to each skeleton — the card BODY (read verbatim by
``emit_migration_readings.py``) and the verbatim citation quotes are left
byte-identical:

  (a) STRIP ``card_edges`` — 0/408 live cfa cards carry ``card_edges``; the
      card->card relationships are already encoded as ``## See Also`` prose links
      in the body, so nothing is lost.
  (b) INJECT ``chunk_id``/``chunk_hash`` = ``"PENDING_INGEST"`` into every citation
      at the canonical CITATION_ORDER position (immediately after ``source_id``), so
      the preflight's placeholder contract holds and the resolver later replaces the
      placeholders with real bindings.
  (c) REMAP every citation ``edge_type`` outside ``{defines, supports}``
      (``depends_on`` / ``extends`` / ``contrasts_with``) to ``supports`` — the entire
      live corpus uses only ``defines``/``supports`` for card->chunk citations; the
      card->card relationship survives as the ``## See Also`` prose.

The transform is LINE-SURGICAL (it never round-trips the YAML, so the authored
frontmatter formatting and the verbatim quotes are preserved exactly) and IDEMPOTENT
(a second run is a byte no-op). Every rewritten card is re-parsed and validated
against the contract BEFORE any file is written; any violation fails closed and no
card is written. A committed normalization report records the exact
strip/inject/remap counts.
"""

from __future__ import annotations

import argparse
import json
import os
import re
from pathlib import Path
from typing import Any

import yaml


ROOT = Path(__file__).resolve().parents[3]
REGISTRY = ROOT / "sources/cfa/_registry"
CFA_READING = Path(os.environ.get("KB_CFA_READING", "/home/jakeshea/CFA_reading"))
DEFAULT_DECK = CFA_READING / "10_Behavioral_Finance/_card_skeletons/10_behavioral_finance"
REPORT_PATH = REGISTRY / "bf_normalization_report.json"

PLACEHOLDER = "PENDING_INGEST"
REMAP_FROM = {"depends_on", "extends", "contrasts_with"}
REMAP_TO = "supports"
EDGE_TYPES = {"defines", "supports"}
SKIP_FILES = {"INDEX.md", "README.md", "SCOPE_EXCEPTION.md"}

# A citation's first line: ``  - source_id: "..."``. The placeholders are injected
# right after it (CITATION_ORDER puts chunk_id/chunk_hash between source_id and page_range).
SOURCE_ID_RE = re.compile(r'^(\s*)-\s+source_id:\s')
# A standalone citation ``edge_type`` line (NOT a card_edges inline item, which begins
# with ``- {`` and is stripped wholesale before this ever runs).
EDGE_TYPE_RE = re.compile(r'^(\s*)edge_type:\s*"([^"]*)"\s*$')


# --------------------------------------------------------------------------- #
# Pure transform (line-surgical; --self-test drives every branch).
# --------------------------------------------------------------------------- #

_EDGE_TYPE_VALUE_RE = re.compile(r'edge_type:\s*"([^"]*)"')


def normalize_frontmatter(fm_lines: list[str]) -> tuple[list[str], dict[str, Any]]:
    """Apply the three transforms to a card's frontmatter lines (the lines BETWEEN
    the ``---`` delimiters). Returns the new lines and per-card stats. The stats also
    tally the edge_types carried on the STRIPPED card_edges (depends_on/extends/...),
    so the report can reconcile every non-{defines,supports} edge_type that was
    eliminated — whether by stripping a card_edge or by remapping a citation."""
    out: list[str] = []
    stats: dict[str, Any] = {"card_edges_stripped": 0, "placeholders_injected": 0,
                             "edge_types_remapped": 0, "card_edge_edge_types": {}}
    i, n = 0, len(fm_lines)
    while i < n:
        line = fm_lines[i]
        # (a) strip a top-level ``card_edges`` key plus its (more-indented) block body.
        if not line.startswith((" ", "\t")) and line.strip().startswith("card_edges:"):
            stats["card_edges_stripped"] += 1
            i += 1
            while i < n and fm_lines[i].startswith((" ", "\t")):
                for et in _EDGE_TYPE_VALUE_RE.findall(fm_lines[i]):
                    stats["card_edge_edge_types"][et] = stats["card_edge_edge_types"].get(et, 0) + 1
                i += 1
            continue
        # (b) inject PENDING_INGEST placeholders right after a citation's source_id line.
        m = SOURCE_ID_RE.match(line)
        if m:
            out.append(line)
            cont = m.group(1) + "  "  # the "- " offsets the continuation keys by two columns
            already = i + 1 < n and fm_lines[i + 1].strip() == f'chunk_id: "{PLACEHOLDER}"'
            if not already:
                out.append(f'{cont}chunk_id: "{PLACEHOLDER}"')
                out.append(f'{cont}chunk_hash: "{PLACEHOLDER}"')
                stats["placeholders_injected"] += 1
            i += 1
            continue
        # (c) remap a citation edge_type outside {defines, supports} to supports.
        e = EDGE_TYPE_RE.match(line)
        if e and e.group(2) in REMAP_FROM:
            out.append(f'{e.group(1)}edge_type: "{REMAP_TO}"')
            stats["edge_types_remapped"] += 1
            i += 1
            continue
        out.append(line)
        i += 1
    return out, stats


def split_card(text: str) -> tuple[list[str], str]:
    """Split a card into (frontmatter_lines, tail) where tail is the closing
    ``---`` line and everything after it, preserved byte-for-byte. Reassembly is
    ``"---\\n" + "\\n".join(fm_lines) + "\\n" + tail``."""
    if not text.startswith("---\n"):
        raise ValueError("missing opening frontmatter delimiter")
    end = text.find("\n---", 4)
    if end == -1:
        raise ValueError("missing closing frontmatter delimiter")
    fm_block = text[4:end]
    return fm_block.split("\n"), text[end + 1:]


def reassemble(fm_lines: list[str], tail: str) -> str:
    return "---\n" + "\n".join(fm_lines) + "\n" + tail


def validate_contract(fm_text: str) -> list[str]:
    """Parse the rewritten frontmatter and re-derive the preflight contract: no
    ``card_edges``, every citation carries both PENDING_INGEST placeholders in
    place, every citation edge_type in {defines, supports}, citations present."""
    problems: list[str] = []
    try:
        fm = yaml.safe_load(fm_text)
    except yaml.YAMLError as exc:
        return [f"frontmatter_not_parseable:{exc}"]
    if not isinstance(fm, dict):
        return ["frontmatter_not_a_mapping"]
    if "card_edges" in fm:
        problems.append("card_edges_still_present")
    cits = fm.get("citations")
    if not isinstance(cits, list) or not cits:
        problems.append("citations_missing_or_empty")
        return problems
    for idx, c in enumerate(cits):
        if not isinstance(c, dict):
            problems.append(f"citation[{idx}]_not_mapping")
            continue
        if c.get("chunk_id") != PLACEHOLDER or c.get("chunk_hash") != PLACEHOLDER:
            problems.append(f"citation[{idx}]_missing_pending_placeholder")
        if c.get("edge_type") not in EDGE_TYPES:
            problems.append(f"citation[{idx}]_edge_type_not_normalized:{c.get('edge_type')}")
    return problems


# --------------------------------------------------------------------------- #
# Deck normalization.
# --------------------------------------------------------------------------- #

def card_paths(deck: Path) -> list[Path]:
    return [p for p in sorted(deck.glob("*.md"))
            if p.name not in SKIP_FILES and not p.name.startswith("_")]


def normalize_deck(deck: Path, reading_id: str, write: bool) -> dict[str, Any]:
    if not deck.is_dir():
        raise SystemExit(f"deck directory not found: {deck}")
    paths = card_paths(deck)
    if not paths:
        raise SystemExit(f"no skeleton cards under {deck}")

    totals: dict[str, Any] = {"cards": len(paths), "cards_with_card_edges_stripped": 0,
                              "citations_placeholders_injected": 0, "citation_edge_types_remapped": 0,
                              "card_edge_edge_types_stripped": {}}
    per_card: list[dict[str, Any]] = []
    rewrites: list[tuple[Path, str]] = []
    problems: list[str] = []

    for p in paths:
        original = p.read_text(encoding="utf-8")
        try:
            fm_lines, tail = split_card(original)
        except ValueError as exc:
            problems.append(f"{p.name}:{exc}")
            continue
        new_fm, stats = normalize_frontmatter(fm_lines)
        new_text = reassemble(new_fm, tail)

        # Validate the rewritten card against the contract BEFORE accepting it.
        try:
            new_fm_block = new_text[4:new_text.find("\n---", 4)]
        except Exception:  # pragma: no cover - reassembly always re-emits delimiters
            problems.append(f"{p.name}:reassembly_lost_delimiter")
            continue
        contract_problems = validate_contract(new_fm_block)
        if contract_problems:
            problems.extend(f"{p.name}:{cp}" for cp in contract_problems)
            continue

        totals["cards_with_card_edges_stripped"] += 1 if stats["card_edges_stripped"] else 0
        totals["citations_placeholders_injected"] += stats["placeholders_injected"]
        totals["citation_edge_types_remapped"] += stats["edge_types_remapped"]
        for et, c in stats["card_edge_edge_types"].items():
            totals["card_edge_edge_types_stripped"][et] = totals["card_edge_edge_types_stripped"].get(et, 0) + c
        if new_text != original:
            rewrites.append((p, new_text))
        per_card.append({"card": p.name, **stats, "changed": new_text != original})

    if problems:
        raise SystemExit("normalization contract violations (fail-closed; nothing written):\n  "
                         + "\n  ".join(problems[:50]))

    written = 0
    if write:
        for p, new_text in rewrites:
            p.write_text(new_text, encoding="utf-8")
            written += 1

    # Reconcile every non-{defines,supports} edge_type eliminated: those carried on a
    # stripped card_edge (card->card relationships, now ## See Also prose) PLUS the few
    # carried on a citation (remapped to supports). The plan's draft figure conflated
    # the two (it read "73 citation edge_types"); on disk the 73 split as
    # (stripped card_edges) + (citation remaps).
    stripped_card_edge_total = sum(totals["card_edge_edge_types_stripped"].values())
    totals["non_standard_edge_types_eliminated"] = {
        "via_card_edges_stripped": stripped_card_edge_total,
        "via_citation_remap": totals["citation_edge_types_remapped"],
        "total": stripped_card_edge_total + totals["citation_edge_types_remapped"],
    }

    return {
        "schema_version": "cfa.template_deck_normalization.v1",
        "deck": str(deck),
        "reading_id": reading_id,
        "totals": totals,
        "cards_rewritten": written if write else len(rewrites),
        "cards_needing_rewrite": len(rewrites),
        "wrote_files": write,
        "per_card": sorted(per_card, key=lambda r: r["card"]),
    }


# --------------------------------------------------------------------------- #
# Self-test (hermetic).
# --------------------------------------------------------------------------- #

def _self_test() -> int:
    f: list[str] = []
    sample = [
        'schema_version: "cacg.v0"',
        'id: "be-x"',
        'title: "T"',
        'reading_id: "10_behavioral_finance"',
        'summary: "s"',
        'tags: ["a"]',
        'card_edges:',
        '  - {target: "be-y", edge_type: "depends_on"}',
        '  - {target: "be-z", edge_type: "extends"}',
        'citations:',
        '  - source_id: "bf_hbe_vol1_2018"',
        '    page_range: [159, 159]',
        '    quote: "a verbatim quote here, long enough"',
        '    edge_type: "depends_on"',
        '  - source_id: "bf_hbe_vol1_2018"',
        '    page_range: [160, 160]',
        '    quote: "another verbatim quote here"',
        '    edge_type: "supports"',
    ]
    out, stats = normalize_frontmatter(sample)
    if stats != {"card_edges_stripped": 1, "placeholders_injected": 2, "edge_types_remapped": 1,
                 "card_edge_edge_types": {"depends_on": 1, "extends": 1}}:
        f.append(f"stats wrong: {stats}")
    if any("card_edges" in ln for ln in out):
        f.append("card_edges not stripped")
    if any("target:" in ln for ln in out):
        f.append("card_edges block body not stripped")
    joined = "\n".join(out)
    if joined.count('chunk_id: "PENDING_INGEST"') != 2 or joined.count('chunk_hash: "PENDING_INGEST"') != 2:
        f.append("placeholders not injected exactly twice")
    # placeholders sit between source_id and page_range, indented 4 (continuation column).
    if '  - source_id: "bf_hbe_vol1_2018"\n    chunk_id: "PENDING_INGEST"\n    chunk_hash: "PENDING_INGEST"\n    page_range:' not in joined:
        f.append("placeholder position/indent wrong")
    if 'edge_type: "depends_on"' in joined or 'edge_type: "extends"' in joined:
        f.append("edge_type not remapped")
    # the already-good supports edge_type is untouched (still exactly one occurrence remapped).
    if joined.count('edge_type: "supports"') != 2:
        f.append("supports count wrong after remap")

    # idempotency: a second pass is a no-op.
    out2, stats2 = normalize_frontmatter(out)
    if out2 != out or any(stats2.values()):
        f.append(f"not idempotent: stats2={stats2}")

    # the result satisfies the contract.
    contract = validate_contract("\n".join(out))
    if contract:
        f.append(f"contract not satisfied: {contract}")

    # split/reassemble round-trips a full card byte-for-byte (body preserved).
    card = "---\n" + "\n".join(sample) + "\n---\n\n# Body\n\ntext with --- dashes inside\n"
    fm_lines, tail = split_card(card)
    if reassemble(fm_lines, tail) != card:
        f.append("split/reassemble not byte-identical")
    # a no-card_edges, already-placeholdered citation is left exactly alone.
    clean = ['  - source_id: "s"', '    chunk_id: "PENDING_INGEST"',
             '    chunk_hash: "PENDING_INGEST"', '    page_range: [1, 1]',
             '    quote: "q"', '    edge_type: "defines"']
    out3, stats3 = normalize_frontmatter(clean)
    if out3 != clean or any(stats3.values()):
        f.append(f"already-normalized citation altered: {stats3}")

    if f:
        print("SELF-TEST FAILED:")
        for x in f:
            print("  -", x)
        return 1
    print("SELF-TEST PASSED (strip card_edges + inject placeholders + remap edge_types + "
          "idempotent + contract + byte-identical body)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--self-test", action="store_true", help="run hermetic self-test and exit")
    ap.add_argument("--deck", type=Path, default=DEFAULT_DECK,
                    help="skeleton card directory to normalize (default: the BF deck)")
    ap.add_argument("--reading-id", type=str, default="10_behavioral_finance")
    ap.add_argument("--write", action="store_true",
                    help="rewrite the deck cards in place and write the normalization report")
    args = ap.parse_args()
    if args.self_test:
        return _self_test()

    report = normalize_deck(args.deck, args.reading_id, write=args.write)
    if args.write:
        REPORT_PATH.write_text(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
                               encoding="utf-8")
    print(json.dumps({k: report[k] for k in ("reading_id", "totals", "cards_needing_rewrite",
                                              "cards_rewritten", "wrote_files")},
                     ensure_ascii=False, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
