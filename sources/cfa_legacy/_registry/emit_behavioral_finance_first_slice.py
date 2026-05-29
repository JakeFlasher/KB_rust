#!/usr/bin/env python3
"""Emit the first CFA legacy CACG card slice for behavioral finance."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[3]
LEGACY_ROOT = Path("/home/jakeshea/CFA_reading/CFA_reading")
REGISTRY = ROOT / "sources/cfa_legacy/_registry"
CARDS_DIR = ROOT / "cards/cfa_legacy/10_behavioral_finance"
OUT = ROOT / "out/cfa_legacy"

CARD_IDS = [
    "be-limits-of-arbitrage",
    "be-noise-trader-equilibrium",
    "be-regret-matching-foundations",
    "be-sentiment-vs-fundamentals",
    "be-two-model-mispricing",
]

PAGE_COORDINATE_MAP = {
    "schema_version": "cfa_legacy.page_coordinate_map.v1",
    "reading_id": "10_behavioral_finance",
    "generated_by": "emit_behavioral_finance_first_slice.py",
    "sources": [
        {
            "source_id": "bf_shleifer_2000_inefficient_markets",
            "legacy_coordinate": "printed_book_page",
            "pdf_coordinate_rule": "pdf_page = legacy_page + 9",
            "verified_evidence": [
                {"legacy_page": 28, "pdf_page": 37, "evidence": "chapter 2 opens on PDF page 37"},
                {"legacy_page": 52, "pdf_page": 61, "evidence": "printed page 52 appears on PDF page 61"},
                {"legacy_page": 89, "pdf_page": 98, "evidence": "chapter 4 opens on PDF page 98"},
                {"legacy_page": 111, "pdf_page": 120, "evidence": "printed page 111 appears on PDF page 120"},
                {"legacy_page": 112, "pdf_page": 121, "evidence": "chapter 5 opens on PDF page 121"},
                {"legacy_page": 153, "pdf_page": 162, "evidence": "printed page 153 appears on PDF page 162"},
                {"legacy_page": 174, "pdf_page": 183, "evidence": "printed page 174 appears on PDF page 183"},
            ],
        },
        {
            "source_id": "econ_hart_mascolell_2013_simple_adaptive_strategies",
            "legacy_coordinate": "physical_pdf_page_for_first_slice_refs",
            "pdf_coordinate_rule": "pdf_page = legacy_page",
            "verified_evidence": [
                {
                    "legacy_page": 22,
                    "pdf_page": 22,
                    "evidence": "regret-matching overview appears in the cited PDF-page window",
                },
                {
                    "legacy_page": 23,
                    "pdf_page": 23,
                    "evidence": "Regret Matching Theorem appears in the cited PDF-page window",
                },
                {
                    "legacy_page": 35,
                    "pdf_page": 35,
                    "evidence": "dynamics/equilibria overview appears in the cited PDF-page window",
                },
            ],
        },
    ],
}

CITATIONS = {
    "be-limits-of-arbitrage": [
        {
            "source_id": "bf_shleifer_2000_inefficient_markets",
            "chunk_id": "bf_shleifer_2000_inefficient_markets:p098:0109",
            "page_range": [98, 99],
            "quote": "arbitrageurs can become most constrained precisely when they have the best opportunities",
            "edge_type": "supports",
            "legacy_ref": "Shleifer printed pp.89-111 -> PDF pp.98-120",
        }
    ],
    "be-noise-trader-equilibrium": [
        {
            "source_id": "bf_shleifer_2000_inefficient_markets",
            "chunk_id": "bf_shleifer_2000_inefficient_markets:p037:0039",
            "page_range": [37, 38],
            "quote": "must be borne by any arbitrageur with a short time horizon",
            "edge_type": "supports",
            "legacy_ref": "Shleifer printed pp.28-52 -> PDF pp.37-61",
        },
        {
            "source_id": "econ_hart_mascolell_2013_simple_adaptive_strategies",
            "chunk_id": "econ_hart_mascolell_2013_simple_adaptive_strategies:p022:0024",
            "page_range": [22, 22],
            "quote": "Switch next period to a different action with a probability that is proportional to the regret",
            "edge_type": "supports",
            "legacy_ref": "Hart/Mas-Colell PDF pp.5-30",
        },
    ],
    "be-regret-matching-foundations": [
        {
            "source_id": "econ_hart_mascolell_2013_simple_adaptive_strategies",
            "chunk_id": "econ_hart_mascolell_2013_simple_adaptive_strategies:p023:0026",
            "page_range": [23, 24],
            "quote": "joint distribution of play converges to the set of correlated equilibria",
            "edge_type": "defines",
            "legacy_ref": "Hart/Mas-Colell PDF pp.5-50",
        },
        {
            "source_id": "bf_shleifer_2000_inefficient_markets",
            "chunk_id": "bf_shleifer_2000_inefficient_markets:p121:0138",
            "page_range": [121, 122],
            "quote": "conservatism, defined as the slow updating of models in the face of new evidence",
            "edge_type": "supports",
            "legacy_ref": "Shleifer printed pp.112-153 -> PDF pp.121-162",
        },
    ],
    "be-sentiment-vs-fundamentals": [
        {
            "source_id": "bf_shleifer_2000_inefficient_markets",
            "chunk_id": "bf_shleifer_2000_inefficient_markets:p163:0184",
            "page_range": [163, 164],
            "quote": "It can result from extrapolative expectations about prices, or trend chasing",
            "edge_type": "supports",
            "legacy_ref": "Shleifer printed pp.112-174 -> PDF pp.121-183",
        },
        {
            "source_id": "econ_hart_mascolell_2013_simple_adaptive_strategies",
            "chunk_id": "econ_hart_mascolell_2013_simple_adaptive_strategies:p035:0042",
            "page_range": [35, 35],
            "quote": "correlated equilibria are, in contrast, “dynamically easy.”",
            "edge_type": "supports",
            "legacy_ref": "Hart/Mas-Colell PDF pp.30-50",
        },
    ],
    "be-two-model-mispricing": [
        {
            "source_id": "bf_shleifer_2000_inefficient_markets",
            "chunk_id": "bf_shleifer_2000_inefficient_markets:p120:0137",
            "page_range": [121, 121],
            "quote": "regularities, already mentioned in Chapter 1, are conveniently called underreaction and overreaction",
            "edge_type": "defines",
            "legacy_ref": "Shleifer printed pp.112-153 -> PDF pp.121-162",
        }
    ],
}

TAGS = {
    "be-limits-of-arbitrage": ["behavioral-finance", "arbitrage", "mispricing"],
    "be-noise-trader-equilibrium": ["behavioral-finance", "noise-traders", "mispricing"],
    "be-regret-matching-foundations": ["behavioral-finance", "regret-matching", "game-theory"],
    "be-sentiment-vs-fundamentals": ["behavioral-finance", "sentiment", "mispricing"],
    "be-two-model-mispricing": ["behavioral-finance", "underreaction", "overreaction"],
}


def read_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def split_legacy_card(path: Path) -> tuple[str, str]:
    text = path.read_text(encoding="utf-8", errors="replace")
    parts = text.split("---", 2)
    if len(parts) < 3:
        raise SystemExit(f"{path}: malformed legacy card")
    return parts[1], parts[2].lstrip("\n")


def existing_card_hash(path: Path) -> str | None:
    if not path.is_file():
        return None
    text = path.read_text(encoding="utf-8")
    match = re.search(r'^card_hash:\s*"([0-9a-f]{64})"\s*$', text, flags=re.MULTILINE)
    return match.group(1) if match else None


def render_scalar(value: Any) -> str:
    if isinstance(value, str):
        return '"' + value.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n") + '"'
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, int):
        return str(value)
    raise TypeError(f"unsupported scalar: {value!r}")


def render_card(frontmatter: dict[str, Any], body: str) -> str:
    key_order = [
        "schema_version",
        "id",
        "title",
        "reading_id",
        "summary",
        "tags",
        "card_edges",
        "citations",
        "card_hash",
    ]
    citation_order = ["source_id", "chunk_id", "chunk_hash", "page_range", "quote", "edge_type"]
    lines = ["---"]
    for key in key_order:
        if key not in frontmatter or frontmatter[key] in (None, [], {}):
            continue
        value = frontmatter[key]
        if key == "tags":
            rendered = ", ".join(render_scalar(tag) for tag in value)
            lines.append(f"tags: [{rendered}]")
        elif key == "citations":
            lines.append("citations:")
            for citation in value:
                first = True
                for ckey in citation_order:
                    if ckey not in citation:
                        continue
                    cvalue = citation[ckey]
                    if ckey == "page_range":
                        rendered = f"[{cvalue[0]}, {cvalue[1]}]"
                    else:
                        rendered = render_scalar(cvalue)
                    prefix = "  - " if first else "    "
                    lines.append(f"{prefix}{ckey}: {rendered}")
                    first = False
        else:
            lines.append(f"{key}: {render_scalar(value)}")
    lines.append("---")
    return "\n".join(lines) + "\n" + body


def chunk_index() -> dict[str, dict[str, Any]]:
    chunks = read_json(OUT / "chunks_manifest.json")["chunks"]
    return {chunk["chunk_id"]: chunk for chunk in chunks}


def normalized(text: str) -> str:
    return " ".join(text.split())


def validate_citations(chunks_by_id: dict[str, dict[str, Any]]) -> dict[str, list[dict[str, Any]]]:
    resolved: dict[str, list[dict[str, Any]]] = {}
    for card_id, citations in CITATIONS.items():
        resolved[card_id] = []
        for citation in citations:
            chunk = chunks_by_id.get(citation["chunk_id"])
            if chunk is None:
                raise SystemExit(f"{card_id}: missing chunk {citation['chunk_id']}")
            if chunk["source_id"] != citation["source_id"]:
                raise SystemExit(f"{card_id}: source mismatch for {citation['chunk_id']}")
            if not (chunk["start_page"] <= citation["page_range"][1] and chunk["end_page"] >= citation["page_range"][0]):
                raise SystemExit(f"{card_id}: page_range does not intersect {citation['chunk_id']}")
            if citation["quote"] not in normalized(chunk["text"]):
                raise SystemExit(f"{card_id}: quote not present in {citation['chunk_id']}")
            resolved_citation = {
                "source_id": citation["source_id"],
                "chunk_id": citation["chunk_id"],
                "chunk_hash": chunk["chunk_hash"],
                "page_range": citation["page_range"],
                "quote": citation["quote"],
                "edge_type": citation["edge_type"],
            }
            resolved[card_id].append(resolved_citation)
    return resolved


def build_citation_plan(resolved: dict[str, list[dict[str, Any]]]) -> dict[str, Any]:
    return {
        "schema_version": "cfa_legacy.behavioral_finance_first_slice_citation_plan.v1",
        "reading_id": "10_behavioral_finance",
        "card_ids": CARD_IDS,
        "page_coordinate_map": PAGE_COORDINATE_MAP,
        "cards": {
            card_id: [
                {
                    **citation,
                    "legacy_ref": source["legacy_ref"],
                }
                for citation, source in zip(resolved[card_id], CITATIONS[card_id], strict=True)
            ]
            for card_id in CARD_IDS
        },
    }


def queue_by_id() -> dict[str, dict[str, Any]]:
    queue = read_json(REGISTRY / "card_migration_queue.json")
    return {card["card_id"]: card for card in queue["cards"]}


def emit_cards(resolved: dict[str, list[dict[str, Any]]], *, force: bool) -> list[Path]:
    cards = queue_by_id()
    emitted: list[Path] = []
    CARDS_DIR.mkdir(parents=True, exist_ok=True)
    for card_id in CARD_IDS:
        queue_entry = cards.get(card_id)
        if queue_entry is None:
            raise SystemExit(f"{card_id}: not present in migration queue")
        if not queue_entry.get("eligible_for_offset_quote_mapping"):
            raise SystemExit(f"{card_id}: not eligible for offset/quote mapping")
        legacy_path = LEGACY_ROOT / queue_entry["legacy_path"]
        _, body = split_legacy_card(legacy_path)
        target = CARDS_DIR / f"{card_id}.md"
        frontmatter: dict[str, Any] = {
            "schema_version": "cacg.v0",
            "id": card_id,
            "title": queue_entry["title"],
            "reading_id": queue_entry["reading_id"],
            "summary": queue_entry["summary_candidate"],
            "tags": TAGS[card_id],
            "citations": resolved[card_id],
        }
        card_hash = existing_card_hash(target)
        if card_hash:
            frontmatter["card_hash"] = card_hash
        text = render_card(frontmatter, body)
        if target.exists():
            old = target.read_text(encoding="utf-8")
            if old == text:
                emitted.append(target)
                continue
            if not force:
                raise SystemExit(f"{target}: exists and differs; rerun with --force")
        target.write_text(text, encoding="utf-8")
        emitted.append(target)
    return emitted


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--force", action="store_true")
    parser.add_argument("--plan-only", action="store_true")
    args = parser.parse_args()

    chunks_by_id = chunk_index()
    resolved = validate_citations(chunks_by_id)
    write_json(REGISTRY / "page_coordinate_maps/10_behavioral_finance.json", PAGE_COORDINATE_MAP)
    write_json(REGISTRY / "behavioral_finance_first_slice_citation_plan.json", build_citation_plan(resolved))
    emitted: list[Path] = []
    if not args.plan_only:
        emitted = emit_cards(resolved, force=args.force)
    print(
        json.dumps(
            {
                "cards": [path.relative_to(ROOT).as_posix() for path in emitted],
                "citation_count": sum(len(v) for v in resolved.values()),
                "plan_only": args.plan_only,
            },
            ensure_ascii=False,
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
