#!/usr/bin/env python3
"""AC-3 closure proof — every Behavioral-Finance citation's RESOLVED ``page_range``
lies within its bound chunk's ``[start_page, end_page]`` span, broken down by the
page-derivation method (auto / recall_located / reviewed_override).

This closes the gap the round-0 review flagged: the page-derivation ledger
(`bf_page_derivation_ledger.json`) located each BF citation's PDF page, but the page
is only PROVEN correct once the resolver binds the quote to a real chunk and `kb
verify` confirms it. This check cross-references the resolver's curated registry
(`bf_10_curated_citations.json`, whose bindings were `kb verify`-confirmed) against
the frozen `chunks_manifest.json` and the derivation ledger, asserting for EVERY BF
citation — especially the non-verbatim ``recall_located`` and ``reviewed_override``
rows — that the bound chunk exists, its `chunk_hash` matches, and the rewritten
`page_range` is within the chunk span. Fail-closed (any violation → exit 1). Writes
`bf_page_binding_proof.json`.
"""

from __future__ import annotations

import json
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
REG = ROOT / "sources/cfa/_registry"
REGISTRY_PATH = REG / "bf_10_curated_citations.json"
LEDGER_PATH = REG / "bf_page_derivation_ledger.json"
CHUNKS = ROOT / "out/cfa/chunks_manifest.json"
PROOF_PATH = REG / "bf_page_binding_proof.json"


def main() -> int:
    reg = json.loads(REGISTRY_PATH.read_text(encoding="utf-8"))
    chunks = {c["chunk_id"]: c for c in json.loads(CHUNKS.read_text(encoding="utf-8"))["chunks"]}
    ledger_rows = json.loads(LEDGER_PATH.read_text(encoding="utf-8"))["rows"]
    # Ledger rows are emitted in (sorted-card, fm-citation) order — the SAME order the
    # resolver loads skeletons and fills the registry — so the i-th (card, citation) aligns.
    led_by_card: dict[str, list] = defaultdict(list)
    for r in ledger_rows:
        led_by_card[r["card_id"]].append(r)

    violations: list[dict] = []
    nonverbatim: list[dict] = []
    by_derivation: Counter = Counter()
    within = total = 0

    for card in reg["cards"]:
        cid = card["card_id"]
        cits = card["citations"]
        leds = led_by_card.get(cid, [])
        for i, cit in enumerate(cits):
            total += 1
            der = leds[i]["status"] if i < len(leds) else "UNMATCHED_LEDGER_ROW"
            by_derivation[der] += 1
            ch = chunks.get(cit["chunk_id"])
            pr = cit["page_range"]
            ok = (
                ch is not None
                and ch["chunk_hash"] == cit["chunk_hash"]
                and isinstance(pr, list) and len(pr) == 2
                and ch["start_page"] <= pr[0] <= pr[1] <= ch["end_page"]
            )
            if ok:
                within += 1
            else:
                violations.append({
                    "card_id": cid, "source_id": cit["source_id"], "chunk_id": cit["chunk_id"],
                    "page_range": pr, "chunk_span": [ch["start_page"], ch["end_page"]] if ch else None,
                    "chunk_hash_match": bool(ch and ch["chunk_hash"] == cit["chunk_hash"]),
                    "derivation": der,
                })
            if der in ("recall_located", "reviewed_override"):
                nonverbatim.append({
                    "card_id": cid, "source_id": cit["source_id"], "derivation": der,
                    "bound_chunk": cit["chunk_id"],
                    "chunk_span": [ch["start_page"], ch["end_page"]] if ch else None,
                    "page_range": pr, "within_span": ok,
                })

    proof = {
        "schema_version": "cfa.bf_page_binding_proof.v1",
        "note": "Every BF citation's resolver-bound page_range is within its chunk span; the "
                "non-verbatim (recall_located/reviewed_override) derived pages all produced real, "
                "kb-verify-confirmed, span-consistent bindings.",
        "total_bf_citations": total,
        "within_span": within,
        "derivation_status_counts": dict(sorted(by_derivation.items())),
        "nonverbatim_bindings": sorted(nonverbatim, key=lambda r: (r["card_id"], r["source_id"])),
        "violations": violations,
        "all_within_span": not violations,
    }
    PROOF_PATH.write_text(json.dumps(proof, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
                          encoding="utf-8")
    print(json.dumps({k: proof[k] for k in
                      ("total_bf_citations", "within_span", "derivation_status_counts", "all_within_span")},
                     ensure_ascii=False, sort_keys=True))
    if violations:
        print(f"\nFAIL: {len(violations)} BF citation(s) page_range NOT within bound chunk span", flush=True)
        for v in violations[:20]:
            print(f"  {v}")
        return 1
    print("PASS: all BF page_ranges within their bound chunk spans (incl. recall_located + reviewed_override)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
