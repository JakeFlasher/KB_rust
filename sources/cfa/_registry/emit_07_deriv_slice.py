#!/usr/bin/env python3
"""Emit the 07_derivatives_and_volatility slice of CFA legacy cacg.v0 cards.

Scope: 20 cards anchored on Hull 11ed (offset 0), Glasserman 2003 (offset +14),
and CFA L1 2022 Vol.5 derivatives readings R45-R46 (offset +2637, with audit
corrections for the legacy Vol.5 pp.380-465 mis-attribution to R47/R48).

Uses hand-curated citations from `deriv_07_slice_curated_citations.json`. Emits
all curated citations per card per the established multi-cite pattern.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[3]
LEGACY_ROOT = Path(__import__("os").environ.get("KB_LEGACY_ROOT", "/home/jakeshea/CFA_reading/CFA_reading"))
REGISTRY = ROOT / "sources/cfa/_registry"
CARDS_DIR = ROOT / "cards/cfa/07_derivatives_and_volatility"
LEGACY_REF_DIR = ROOT / "_legacy_reference/cfa/cards/07_derivatives_and_volatility"
OUT = ROOT / "out/cfa"
CURATED = REGISTRY / "deriv_07_slice_curated_citations.json"

ROLE_TO_EDGE = {"primary": "defines", "supporting": "supports"}

H1_RE = re.compile(r"^#\s+(.+)$", re.MULTILINE)
_STOPWORDS = {"and", "the", "of", "for", "with", "vs", "to", "in", "on", "a", "an"}


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_atomic(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(content, encoding="utf-8")
    tmp.replace(path)


def split_legacy_card(legacy_md: Path) -> tuple[str, str]:
    text = legacy_md.read_text(encoding="utf-8", errors="replace")
    parts = text.split("---", 2)
    if len(parts) < 3:
        raise SystemExit(f"{legacy_md}: malformed frontmatter")
    return parts[1], parts[2].lstrip("\n")


def extract_title(body: str) -> str:
    m = H1_RE.search(body)
    return m.group(1).strip() if m else "Untitled"


def derive_tags(card_id: str) -> list[str]:
    """Generate tags from card_id: ['derivatives', '<topic-suffix>']. Skips
    connective stopwords when building the suffix."""
    parts = card_id.split("-")
    body = [p for p in parts[1:] if p.lower() not in _STOPWORDS]
    if len(body) >= 2:
        return ["derivatives", "-".join(body[:2])]
    if len(body) == 1:
        return ["derivatives", body[0]]
    return ["derivatives"]


def render_scalar(value: Any) -> str:
    if isinstance(value, str):
        return '"' + value.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n") + '"'
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, int):
        return str(value)
    raise TypeError(f"unsupported scalar: {value!r}")


def render_card(frontmatter: dict[str, Any], body: str) -> str:
    key_order = ["schema_version", "id", "title", "reading_id", "summary", "tags", "card_edges", "citations", "card_hash"]
    citation_order = ["source_id", "chunk_id", "chunk_hash", "page_range", "quote", "edge_type"]
    lines = ["---"]
    for key in key_order:
        if key not in frontmatter or frontmatter[key] in (None, [], {}):
            continue
        value = frontmatter[key]
        if key == "tags":
            rendered = ", ".join(render_scalar(t) for t in value)
            lines.append(f"tags: [{rendered}]")
        elif key == "citations":
            lines.append("citations:")
            for cit in value:
                first = True
                for ckey in citation_order:
                    if ckey not in cit:
                        continue
                    cv = cit[ckey]
                    if ckey == "page_range":
                        rendered = f"[{cv[0]}, {cv[1]}]"
                    else:
                        rendered = render_scalar(cv)
                    prefix = "  - " if first else "    "
                    lines.append(f"{prefix}{ckey}: {rendered}")
                    first = False
        else:
            lines.append(f"{key}: {render_scalar(value)}")
    lines.append("---")
    return "\n".join(lines) + "\n" + body


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--plan-only", action="store_true")
    parser.add_argument("--force", action="store_true")
    args = parser.parse_args()

    curated = read_json(CURATED)
    chunks_manifest = read_json(OUT / "chunks_manifest.json")
    chunks_by_id = {c["chunk_id"]: c for c in chunks_manifest["chunks"]}

    emitted = []
    plan = {}

    for card in curated["cards"]:
        card_id = card["card_id"]
        title = card["title"]
        summary = card["summary_curated"]

        if len(summary) > 400:
            summary = summary[:397] + "..."
        if len(summary) < 80:
            summary = (summary + " " * 80)[:80]

        if not card["citations"]:
            raise SystemExit(f"{card_id}: no citations")
        citations: list[dict] = []
        plan_cites: list[dict] = []
        for src in card["citations"]:
            chunk = chunks_by_id.get(src["chunk_id"])
            if chunk is None:
                raise SystemExit(f"{card_id}: chunk_id {src['chunk_id']} not in manifest")
            if chunk["chunk_hash"] != src["chunk_hash"]:
                raise SystemExit(
                    f"{card_id}: chunk_hash mismatch for {src['chunk_id']}: "
                    f"manifest={chunk['chunk_hash'][:12]} curated={src['chunk_hash'][:12]}"
                )
            normalized_chunk_text = " ".join(chunk["text"].split())
            normalized_quote = " ".join(src["quote"].split())
            if normalized_quote not in normalized_chunk_text:
                raise SystemExit(
                    f"{card_id}: quote not found in chunk {src['chunk_id']} (normalized)\n"
                    f"  quote: {normalized_quote[:100]!r}\n"
                    f"  chunk head: {normalized_chunk_text[:200]!r}"
                )
            page_range = [
                max(chunk["start_page"], src["pdf_pages"][0]),
                min(chunk["end_page"], src["pdf_pages"][1]),
            ]
            if page_range[0] > page_range[1]:
                page_range = [chunk["start_page"], chunk["end_page"]]
            role = src.get("role", "supporting")
            edge_type = ROLE_TO_EDGE.get(role, "supports")
            citations.append({
                "source_id": src["source_id"],
                "chunk_id": src["chunk_id"],
                "chunk_hash": src["chunk_hash"],
                "page_range": page_range,
                "quote": src["quote"],
                "edge_type": edge_type,
            })
            plan_cites.append({
                "role": role,
                "source_id": src["source_id"],
                "chunk_id": src["chunk_id"],
                "legacy_pages": src["legacy_pages"],
                "pdf_pages": src["pdf_pages"],
                "page_range_emitted": page_range,
            })

        plan[card_id] = {
            "title": title,
            "source_stance": card["source_stance"],
            "audit_flag": card["audit_flag"],
            "citations": plan_cites,
        }

        if args.plan_only:
            continue

        scrubbed = LEGACY_REF_DIR / f"{card_id}.md"
        legacy_md = LEGACY_ROOT / ".claude/knowledge/07_derivatives_and_volatility" / f"{card_id}.md"
        source_md = scrubbed if scrubbed.exists() else legacy_md
        if not source_md.exists():
            raise SystemExit(f"missing legacy card: {legacy_md} (and no scrubbed copy)")
        _frontmatter, body = split_legacy_card(source_md)

        legacy_title = extract_title(body)
        if title.strip() != legacy_title.strip():
            print(
                f"WARN {card_id}: curated title differs from legacy H1 -- using legacy H1\n"
                f"  curated: {title!r}\n"
                f"  legacy : {legacy_title!r}",
                file=sys.stderr,
            )
            title = legacy_title

        fm = {
            "schema_version": "cacg.v0",
            "id": card_id,
            "title": title,
            "reading_id": "07_derivatives_and_volatility",
            "summary": summary,
            "tags": derive_tags(card_id),
            "citations": citations,
        }
        text = render_card(fm, body)
        target = CARDS_DIR / f"{card_id}.md"
        if target.exists() and not args.force:
            old = target.read_text(encoding="utf-8")
            if old == text:
                emitted.append(target)
                continue
            raise SystemExit(f"{target}: exists and differs; rerun with --force")
        write_atomic(target, text)
        emitted.append(target)

    write_atomic(
        REGISTRY / "deriv_07_slice_citation_plan.json",
        json.dumps({"schema_version": "cfa.deriv_07_slice.v1", "cards": plan}, indent=2, sort_keys=True, ensure_ascii=False) + "\n",
    )
    print(json.dumps({"emitted": [str(p.relative_to(ROOT)) for p in emitted], "plan_only": args.plan_only}, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
