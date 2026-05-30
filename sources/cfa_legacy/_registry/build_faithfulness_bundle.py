#!/usr/bin/env python3
"""Assemble a compact review bundle for the migration faithfulness pass.

For every repaired citation in ``migration_quote_audit.json`` and every documented
cross-link pair in ``migration_cross_links.json``, emit the small set of facts a
human/independent reviewer needs to judge SEMANTIC faithfulness (which ``kb verify``
cannot): the original vs final quote, a short window of the bound chunk's text
around the final quote (the on-page context), and the path to the card so the
reviewer can read the claim the quote is cited for.

The bundle is written to a throwaway path (default under ``/tmp``); it is a
review input, not a committed artifact, but it is fully reproducible from the
committed audit + map + the (gitignored) chunks manifest.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
REGISTRY = ROOT / "sources/cfa_legacy/_registry"
CARDS = ROOT / "cards/cfa_legacy"
CHUNKS = ROOT / "out/cfa_legacy/chunks_manifest.json"
AUDIT = REGISTRY / "migration_quote_audit.json"
CROSS = REGISTRY / "migration_cross_links.json"

READING_OF_PREFIX = {"mt-": "14_microstructure_and_trading",
                     "pa-": "15_performance_and_attribution",
                     "fa-": "22_fund_level_arbitrage",
                     "be-": "10_behavioral_finance"}
WINDOW = 170


def reading_of(card_id: str) -> str:
    for p, r in READING_OF_PREFIX.items():
        if card_id.startswith(p):
            return r
    raise ValueError(card_id)


def card_rel(card_id: str) -> str:
    return f"cards/cfa_legacy/{reading_of(card_id)}/{card_id}.md"


def chunk_window(text: str, quote: str) -> str:
    i = text.find(quote)
    if i < 0:
        return "(final quote not found verbatim in chunk text)"
    lo = max(0, i - WINDOW)
    hi = min(len(text), i + len(quote) + WINDOW)
    return ("…" if lo > 0 else "") + text[lo:hi] + ("…" if hi < len(text) else "")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", type=Path, default=Path("/tmp/migration_faithfulness_bundle.json"))
    args = ap.parse_args()

    audit = json.loads(AUDIT.read_text(encoding="utf-8"))
    rows = audit["changed_quotes"]
    chunks = {c["chunk_id"]: c for c in json.loads(CHUNKS.read_text(encoding="utf-8"))["chunks"]}

    repairs = []
    for r in rows:
        chunk = chunks.get(r["chunk_id"], {})
        repairs.append({
            "card_id": r["card_id"],
            "reading_id": reading_of(r["card_id"]),
            "card_path": card_rel(r["card_id"]),
            "cause": r["cause"],
            "type": r["type"],
            "overlap": r.get("overlap"),
            "source_id": r["source_id"],
            "chunk_id": r["chunk_id"],
            "original_quote": r["original_quote"],
            "final_quote": r["final_quote"],
            "chunk_context": chunk_window(chunk.get("text", ""), r["final_quote"]),
        })

    cross = json.loads(CROSS.read_text(encoding="utf-8"))
    pairs = [{
        "primary": p["primary"],
        "primary_path": card_rel(p["primary"]),
        "relation": p["relation"],
        "note": p["note"],
        "counterparts": [{"card_id": c, "path": card_rel(c)} for c in p["counterparts"]],
    } for p in cross["documented_pairs"]]

    bundle = {
        "repairs": sorted(repairs, key=lambda x: (x["reading_id"], x["card_id"])),
        "cross_link_pairs": pairs,
        "counts": {
            "repairs": len(repairs),
            "reanchor": sum(1 for r in repairs if r["type"] == "reanchor"),
            "trim": sum(1 for r in repairs if r["type"] == "trim"),
            "pairs": len(pairs),
        },
    }
    args.out.write_text(json.dumps(bundle, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {args.out}: {len(repairs)} repairs, {len(pairs)} cross-link pairs")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
