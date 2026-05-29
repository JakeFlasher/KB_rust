#!/usr/bin/env python3
"""Emit the 09_portfolio_management_and_asset_pricing slice of CFA legacy cacg.v0 cards.

Scope: the 18 cards that cite ONLY cfa_2022_l1_combined (using the v2 per-volume
offset map built in bucket 1). The 5 multi-source PM cards (citing Pedersen or
Cochrane) are deferred until those single-source offset maps are built.

Pipeline per card:
  1. Read legacy frontmatter -> extract Vol.N/pp.<start>-<end>, "Use when",
     title (H1 in body).
  2. Convert volume_page -> pdf_page via volume_page_map.py.
  3. Find smallest chunk in chunks_manifest with source_id=cfa_2022_l1_combined
     and page range overlapping the cited PDF range.
  4. Pick a verbatim quote: first sentence-like phrase 40-200 chars from chunk
     text, preferring substantive content.
  5. Build cacg.v0 frontmatter (schema_version, id, title, reading_id, summary
     [<=400 chars], tags, citations).
  6. Render card body (preserved verbatim from legacy).
  7. Atomic write to cards/cfa_legacy/09_portfolio_management_and_asset_pricing/.
"""

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
CARDS_DIR = ROOT / "cards/cfa_legacy/09_portfolio_management_and_asset_pricing"
OUT = ROOT / "out/cfa_legacy"
COORD_MAPS = REGISTRY / "page_coordinate_maps"

sys.path.insert(0, str(COORD_MAPS))
from volume_page_map import load_map, vol_page_to_pdf_page  # noqa: E402

# 18 CFA-L1-only PM cards from the migration queue
CARD_IDS = [
    "pm-active-vs-passive-decision",
    "pm-allocation-process",
    "pm-beta-and-factor-exposure",
    "pm-capital-market-line",
    "pm-capm-and-sml",
    "pm-diversification-and-correlation",
    "pm-efficient-frontier",
    "pm-factor-models-intuition",
    "pm-investment-policy-statement",
    "pm-investor-types",
    "pm-market-efficiency-core",
    "pm-performance-ratios-definitions",
    "pm-portfolio-constraints",
    "pm-portfolio-perspective",
    "pm-rebalancing-mechanics",
    "pm-return-and-risk-fundamentals",
    "pm-risk-tolerance-and-objectives",
    "pm-systematic-vs-idiosyncratic-risk",
]

VOL_PAGE_RE = re.compile(r"Vol\.(\d+)/pp\.(\d+)(?:-(\d+))?")
PRIMARY_RE = re.compile(r"^Primary raw source:\s*(.+)$", re.MULTILINE)
USE_WHEN_RE = re.compile(r"^Use when:\s*(.+?)(?=\n[A-Z][a-zA-Z ]+:|\n---)", re.DOTALL | re.MULTILINE)
H1_RE = re.compile(r"^#\s+(.+)$", re.MULTILINE)


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


def extract_use_when(frontmatter: str) -> str:
    m = USE_WHEN_RE.search(frontmatter)
    if not m:
        return ""
    txt = m.group(1).strip()
    # Collapse newlines + whitespace
    txt = re.sub(r"\s+", " ", txt)
    return txt


def extract_title(body: str) -> str:
    m = H1_RE.search(body)
    return m.group(1).strip() if m else "Untitled"


def extract_primary_vol_pages(frontmatter: str) -> tuple[int, int, int]:
    p = PRIMARY_RE.search(frontmatter)
    if not p:
        raise ValueError("no Primary raw source")
    val = p.group(1).strip()
    m = VOL_PAGE_RE.search(val)
    if not m:
        raise ValueError(f"no Vol.N/pp.X-Y pattern in: {val}")
    vol = int(m.group(1))
    page_start = int(m.group(2))
    page_end = int(m.group(3)) if m.group(3) else page_start
    return vol, page_start, page_end


def normalize(text: str) -> str:
    return " ".join(text.split())


def find_smallest_overlap_chunk(chunks: list[dict], source_id: str, pdf_start: int, pdf_end: int) -> dict | None:
    """Return the smallest-page-span chunk whose page range overlaps [pdf_start, pdf_end]."""
    candidates = [
        c for c in chunks
        if c["source_id"] == source_id
        and c["start_page"] <= pdf_end
        and c["end_page"] >= pdf_start
    ]
    if not candidates:
        return None
    # Sort by page-span size (ascending), then by closeness to pdf_start
    candidates.sort(key=lambda c: (c["end_page"] - c["start_page"], abs(c["start_page"] - pdf_start)))
    return candidates[0]


SENT_END = re.compile(r"(?<=[.?!])\s+(?=[A-Z])")


def pick_quote(chunk_text: str, min_len: int = 40, max_len: int = 200) -> str:
    """Pick a verbatim quote: first sentence-like phrase in [min_len, max_len]."""
    text = normalize(chunk_text)
    sentences = SENT_END.split(text)
    for s in sentences:
        # Skip boilerplate / headers
        if any(s.upper().startswith(b) for b in ["READING ", "CHAPTER ", "VOLUME ", "CONTENTS", "STUDY SESSION"]):
            continue
        if "STANDARDS OF PRACTICE" in s.upper() and len(s) < 80:
            continue
        if min_len <= len(s) <= max_len:
            return s.strip()
    # Fallback: head of text up to max_len
    return text[:max_len].rstrip()


def derive_tags(card_id: str) -> list[str]:
    """Generate tags from card_id: ['portfolio-management', '<topic-suffix>']."""
    parts = card_id.split("-")
    if len(parts) >= 2:
        # Use first 2-3 words after 'pm' prefix
        suffix = "-".join(parts[1:3]) if len(parts) > 2 else parts[1]
        return ["portfolio-management", suffix]
    return ["portfolio-management"]


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

    coord_map = load_map(COORD_MAPS / "cfa_2022_l1_combined.json")
    chunks_manifest = read_json(OUT / "chunks_manifest.json")
    chunks = chunks_manifest["chunks"]

    emitted = []
    plan = {}
    for card_id in CARD_IDS:
        legacy_md = LEGACY_ROOT / ".claude/knowledge/09_portfolio_management_and_asset_pricing" / f"{card_id}.md"
        if not legacy_md.exists():
            raise SystemExit(f"missing legacy card: {legacy_md}")
        frontmatter, body = split_legacy_card(legacy_md)
        title = extract_title(body)
        use_when = extract_use_when(frontmatter)
        vol, pg_start, pg_end = extract_primary_vol_pages(frontmatter)
        pdf_start = vol_page_to_pdf_page(coord_map, volume=vol, volume_page=pg_start)
        pdf_end = vol_page_to_pdf_page(coord_map, volume=vol, volume_page=pg_end)
        chunk = find_smallest_overlap_chunk(chunks, "cfa_2022_l1_combined", pdf_start, pdf_end)
        if chunk is None:
            raise SystemExit(f"{card_id}: no overlapping chunk for Vol.{vol}/pp.{pg_start}-{pg_end} -> PDF {pdf_start}-{pdf_end}")
        quote = pick_quote(chunk["text"])
        cite_page_range = [max(chunk["start_page"], pdf_start), min(chunk["end_page"], pdf_end)]
        citation = {
            "source_id": "cfa_2022_l1_combined",
            "chunk_id": chunk["chunk_id"],
            "chunk_hash": chunk["chunk_hash"],
            "page_range": cite_page_range,
            "quote": quote,
            "edge_type": "supports",
        }

        # Summary: prefix title + use-when, truncate to 400
        summary = f"{title}: {use_when}".strip()
        if len(summary) > 400:
            summary = summary[:397] + "..."
        if len(summary) < 80:
            summary = (summary + " " * 80)[:80]

        plan[card_id] = {
            "title": title,
            "legacy_primary": f"Vol.{vol}/pp.{pg_start}-{pg_end} -> PDF {pdf_start}-{pdf_end}",
            "chunk_id": chunk["chunk_id"],
            "page_range": cite_page_range,
            "quote_preview": quote[:80],
        }

        if args.plan_only:
            continue

        fm = {
            "schema_version": "cacg.v0",
            "id": card_id,
            "title": title,
            "reading_id": "09_portfolio_management_and_asset_pricing",
            "summary": summary,
            "tags": derive_tags(card_id),
            "citations": [citation],
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
        REGISTRY / "pm_09_slice_citation_plan.json",
        json.dumps({"schema_version": "cfa_legacy.pm_09_slice.v1", "cards": plan}, indent=2, sort_keys=True, ensure_ascii=False) + "\n",
    )
    print(json.dumps({"emitted": [str(p.relative_to(ROOT)) for p in emitted], "plan_only": args.plan_only}, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
