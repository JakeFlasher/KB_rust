#!/usr/bin/env python3
"""Emit the 09_portfolio_management_and_asset_pricing slice of CFA legacy cacg.v0 cards.

Scope: all 23 active PM cards, rendered from two curated citation registries:

  * 18 CFA-only cards -> pm_09_cfa_curated_citations.json (each cites
    cfa_2022_l1_combined).
  * 5 multi-source cards -> pm_09_multi_source_citations.json (each cites a
    Pedersen/Cochrane `defines` primary, backed by a single-source v1 offset map,
    plus a CFA-combined/Cochrane `supports` secondary). These 5 were deferred at
    baseline until the single-source offset maps were built.

Why curated (not a heuristic)?  The naive "legacy Vol.N/pp -> v2 offset map ->
smallest overlapping chunk -> first-sentence quote" path does NOT reproduce the
hand-curated on-disk cards: for 14 of the 18 CFA cards the curated anchor
deliberately deviates from the legacy page reference (the legacy ref maps, via the
combined offset map, to a different — often wrong — chunk; curation re-anchored each
to the correct content). Driving emission from curated citations is therefore the
only faithful, reproducible path. The legacy reference + its offset-map range are
recorded as `legacy_provenance` in the CFA curated registry and echoed into the
emitted plan sidecar (pm_09_slice_citation_plan.json) for auditability — they are
NOT written into card frontmatter (cards carry only cacg.v0 fields).

Legacy card bodies are read verbatim from
<legacy-root>/.claude/knowledge/09_.../<card_id>.md, where <legacy-root> defaults to
/home/jakeshea/CFA_reading (override via --legacy-root or the KB_LEGACY_ROOT env).
The on-disk card bodies are byte-identical to these legacy bodies.

EVERY citation is validated before any card is written: the source is authorized for
the reading (source_matrix.json); the chunk exists with a matching source_id;
chunk_hash matches; page_range is within the chunk's [start_page, end_page] span; the
quote is an exact, control-character-free substring of the chunk text; and for
Pedersen/Cochrane the single-source offset map loads with >=3 verified_evidence
triples. Any failure aborts the run (non-zero exit) before any write.

Frontmatter is rendered without card_hash; run `KB_FROZEN_CLOCK=1 kb index`
afterward to recompute card_hash + regenerate manifests/sidecars. With unchanged
inputs the run is a no-op (cards regenerate byte-identically).
"""

from __future__ import annotations

import argparse
import json
import os
import re
import unicodedata
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[3]
# Real legacy-card root (override via --legacy-root or KB_LEGACY_ROOT). The legacy
# bodies live at <legacy-root>/.claude/knowledge/<reading>/. NOTE: an earlier nested
# default (/home/jakeshea/CFA_reading/CFA_reading) was wrong and is corrected here.
DEFAULT_LEGACY_ROOT = Path(os.environ.get("KB_LEGACY_ROOT", "/home/jakeshea/CFA_reading"))
REGISTRY = ROOT / "sources/cfa/_registry"
CARDS_DIR = ROOT / "cards/cfa/09_portfolio_management_and_asset_pricing"
# Scrubbed, in-repo card bodies (preferred over the read-only legacy KB so emission is
# reproducible on a clean checkout that lacks the author's CFA_reading sibling).
LEGACY_REF_DIR = ROOT / "_legacy_reference/cfa/cards/09_portfolio_management_and_asset_pricing"
OUT = ROOT / "out/cfa"
COORD_MAPS = REGISTRY / "page_coordinate_maps"
SOURCE_MATRIX = OUT / "source_matrix.json"
CFA_REGISTRY = REGISTRY / "pm_09_cfa_curated_citations.json"
MULTI_SOURCE_REGISTRY = REGISTRY / "pm_09_multi_source_citations.json"
READING_ID = "09_portfolio_management_and_asset_pricing"
LEGACY_SUBDIR = ".claude/knowledge/09_portfolio_management_and_asset_pricing"

# Expected card sets (a guard against registry drift). Union = the 23 active PM cards.
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
    "pm-tracking-error-and-active-risk",
]
MULTI_SOURCE_CARD_IDS = [
    "pm-active-management-and-alpha",
    "pm-anomalies-and-cross-sectional-pricing",
    "pm-efficient-markets-and-anomalies",
    "pm-multifactor-asset-pricing-intuition",
    "pm-stochastic-discount-factor-intuition",
]

# Sources whose single-source v1 offset map must load with >=3 verified_evidence
# before any citation against them is accepted.
SINGLE_SOURCE_MAP_SOURCES = {
    "pm_pedersen_2015_efficiently_inefficient",
    "pm_cochrane_2005_asset_pricing_revised",
}


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


def illegal_text_chars(text: str) -> list[str]:
    """Quote characters that are PDF-extraction artifacts / illegal in card text:
    control (Cc), format (Cf — e.g. soft-hyphen U+00AD, ZWSP U+200B), line/paragraph
    separators (Zl/Zp), and Unicode noncharacters (incl. U+FFFE from the Pdfium
    STX->U+FFFE map). Newlines are Cc and thus rejected — quotes must be single-line."""
    bad: list[str] = []
    for c in text:
        o = ord(c)
        noncharacter = (0xFDD0 <= o <= 0xFDEF) or (o & 0xFFFE) == 0xFFFE
        if noncharacter or unicodedata.category(c) in {"Cc", "Cf", "Zl", "Zp"}:
            bad.append(hex(o))
    return bad


_REQUIRED_CITATION_FIELDS = ("source_id", "chunk_id", "chunk_hash", "page_range", "quote", "edge_type")


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


_OFFSET_RULE_RE = re.compile(r"pdf_page\s*=\s*legacy_page\s*([+-])\s*(\d+)\s*$")


def parse_offset_rule(rule: Any, *, source_id: str) -> int:
    """Parse a v1 single-offset rule 'pdf_page = legacy_page + N' (N may be negative)."""
    if not isinstance(rule, str):
        raise SystemExit(f"{source_id}: offset-map pdf_coordinate_rule is not a string: {rule!r}")
    m = _OFFSET_RULE_RE.match(rule.strip())
    if not m:
        raise SystemExit(
            f"{source_id}: unparseable pdf_coordinate_rule {rule!r} "
            "(expected 'pdf_page = legacy_page + N')"
        )
    mag = int(m.group(2))
    return mag if m.group(1) == "+" else -mag


def validate_single_source_map(source_id: str) -> dict[str, Any]:
    """Load + validate a v1 single-source offset map (AC-6 negative test, plan line 68).

    Rejects the map (non-zero exit) unless: it has exactly one source entry whose
    source_id matches; its `pdf_coordinate_rule` parses to `pdf_page = legacy_page + N`;
    EVERY `verified_evidence` triple has integer pages and re-derives
    `legacy_page + N == pdf_page` (ANY failing triple rejects the whole map); and at
    least 3 such confirmed triples exist. Returns {offset, page_count_pdf,
    confirmed_evidence}.
    """
    map_path = COORD_MAPS / f"{source_id}.json"
    if not map_path.exists():
        raise SystemExit(f"single-source offset map missing: {map_path}")
    doc = read_json(map_path)
    sources = doc.get("sources")
    if not isinstance(sources, list) or len(sources) != 1:
        raise SystemExit(f"{source_id}: single-source map must have exactly one `sources` entry")
    src = sources[0]
    if src.get("source_id") != source_id:
        raise SystemExit(
            f"{source_id}: map sources[0].source_id is {src.get('source_id')!r}, expected {source_id!r}"
        )
    offset = parse_offset_rule(src.get("pdf_coordinate_rule"), source_id=source_id)
    evidence = src.get("verified_evidence")
    if not isinstance(evidence, list):
        raise SystemExit(f"{source_id}: verified_evidence is not a list")
    confirmed = 0
    for i, ev in enumerate(evidence):
        lp, pp = ev.get("legacy_page"), ev.get("pdf_page")
        if not (isinstance(lp, int) and not isinstance(lp, bool)
                and isinstance(pp, int) and not isinstance(pp, bool)):
            raise SystemExit(
                f"{source_id}: verified_evidence[{i}] legacy_page/pdf_page not integers: {ev!r}"
            )
        if lp + offset != pp:
            raise SystemExit(
                f"{source_id}: verified_evidence[{i}] fails re-derivation: "
                f"legacy_page {lp} + offset {offset} = {lp + offset} != pdf_page {pp}"
            )
        confirmed += 1
    if confirmed < 3:
        raise SystemExit(
            f"{source_id}: only {confirmed} re-derived verified_evidence triples (need >=3)"
        )
    page_count = src.get("page_count_pdf")
    if not (isinstance(page_count, int) and not isinstance(page_count, bool) and page_count > 0):
        raise SystemExit(
            f"{source_id}: page_count_pdf must be a positive integer, got {page_count!r}"
        )
    return {"offset": offset, "page_count_pdf": page_count, "confirmed_evidence": confirmed}


def validate_citation(
    cit: dict[str, Any],
    *,
    card_id: str,
    allowed: set[str],
    chunks_by_id: dict[str, dict],
    retracted_chunks: set[str],
    retracted_sources: set[str],
    validated_maps: dict[str, dict[str, Any]],
) -> None:
    """Abort the run (non-zero exit) unless every trust check passes for `cit`.

    Checks: every required field present; source authorized for the reading and not
    retracted; chunk exists, not retracted, with matching source_id; chunk_hash
    matches; page_range well-formed and within the chunk's [start_page, end_page]
    span; quote is an exact, artifact-free substring of the chunk text; and for
    Pedersen/Cochrane the single-source offset map was pre-validated (rule re-derives,
    >=3 confirmed triples) and the cited page is within the source's PDF page count.
    `validated_maps` is the output of validate_single_source_map per single-source id.
    """
    missing = [f for f in _REQUIRED_CITATION_FIELDS if f not in cit]
    if missing:
        raise SystemExit(f"{card_id}: citation missing required field(s) {missing}")
    src = cit["source_id"]
    if src not in allowed:
        raise SystemExit(f"{card_id}: source {src!r} not authorized for {READING_ID}")
    if src in retracted_sources:
        raise SystemExit(f"{card_id}: source {src!r} is retracted")
    if cit["chunk_id"] in retracted_chunks:
        raise SystemExit(f"{card_id}: chunk {cit['chunk_id']} is retracted")
    chunk = chunks_by_id.get(cit["chunk_id"])
    if chunk is None:
        raise SystemExit(f"{card_id}: chunk {cit['chunk_id']} not found in chunks_manifest")
    if chunk["source_id"] != src:
        raise SystemExit(
            f"{card_id}: chunk {cit['chunk_id']} source {chunk['source_id']!r} != citation source {src!r}"
        )
    if chunk["chunk_hash"] != cit["chunk_hash"]:
        raise SystemExit(f"{card_id}: chunk_hash mismatch for {cit['chunk_id']}")
    pr = cit["page_range"]
    if not (isinstance(pr, list) and len(pr) == 2 and all(isinstance(x, int) for x in pr)):
        raise SystemExit(f"{card_id}: page_range {pr!r} is not a [int, int] pair ({cit['chunk_id']})")
    if not (chunk["start_page"] <= pr[0] <= pr[1] <= chunk["end_page"]):
        raise SystemExit(
            f"{card_id}: page_range {pr} not within chunk span "
            f"[{chunk['start_page']}, {chunk['end_page']}] for {cit['chunk_id']}"
        )
    if cit["quote"] not in chunk["text"]:
        raise SystemExit(f"{card_id}: quote is not a verbatim substring of chunk {cit['chunk_id']}")
    bad = illegal_text_chars(cit["quote"])
    if bad:
        raise SystemExit(f"{card_id}: quote contains illegal chars {bad} ({cit['chunk_id']})")
    if src in SINGLE_SOURCE_MAP_SOURCES:
        vmap = validated_maps.get(src)
        if vmap is None:
            raise SystemExit(f"{card_id}: single-source offset map for {src} was not validated")
        # page_count_pdf is guaranteed a positive int by validate_single_source_map,
        # so the cited-page bound is always enforced (never silently skipped).
        page_count = vmap["page_count_pdf"]
        if not (1 <= pr[0] <= pr[1] <= page_count):
            raise SystemExit(
                f"{card_id}: cited page_range {pr} outside {src} PDF page count "
                f"[1, {page_count}] ({cit['chunk_id']})"
            )


def emit_card(fm: dict[str, Any], body: str, card_id: str, *, force: bool, emitted: list[Path]) -> None:
    text = render_card(fm, body)
    target = CARDS_DIR / f"{card_id}.md"
    if target.exists() and not force:
        old = target.read_text(encoding="utf-8")
        if old == text:
            emitted.append(target)
            return
        raise SystemExit(f"{target}: exists and differs; rerun with --force")
    write_atomic(target, text)
    emitted.append(target)


def emit_registry_card(
    card: dict[str, Any],
    *,
    legacy_root: Path,
    allowed: set[str],
    chunks_by_id: dict[str, dict],
    retracted_chunks: set[str],
    retracted_sources: set[str],
    validated_maps: dict[str, dict[str, Any]],
    force: bool,
    plan_only: bool,
    emitted: list[Path],
    plan: dict[str, Any],
) -> None:
    """Validate + emit one curated card (body verbatim from legacy; metadata from the registry)."""
    card_id = card["card_id"]
    # Prefer the scrubbed in-repo copy (notes-free, tracked) over the read-only legacy
    # KB; fall back to the legacy KB when no scrubbed copy exists.
    scrubbed = LEGACY_REF_DIR / f"{card_id}.md"
    legacy_md = legacy_root / LEGACY_SUBDIR / f"{card_id}.md"
    source_md = scrubbed if scrubbed.exists() else legacy_md
    if not source_md.exists():
        raise SystemExit(f"missing legacy card: {legacy_md} (and no scrubbed copy at {scrubbed})")
    _legacy_fm, body = split_legacy_card(source_md)
    citations = card["citations"]
    if not citations:
        raise SystemExit(f"{card_id}: curated registry entry has no citations")
    for cit in citations:
        validate_citation(
            cit, card_id=card_id, allowed=allowed, chunks_by_id=chunks_by_id,
            retracted_chunks=retracted_chunks, retracted_sources=retracted_sources,
            validated_maps=validated_maps,
        )
    plan[card_id] = {
        "title": card["title"],
        "citations": [
            {"source_id": c["source_id"], "chunk_id": c["chunk_id"], "edge_type": c["edge_type"]}
            for c in citations
        ],
        "legacy_provenance": card.get("legacy_provenance"),
    }
    if plan_only:
        return
    fm = {
        "schema_version": "cacg.v0",
        "id": card_id,
        "title": card["title"],
        "reading_id": READING_ID,
        "summary": card["summary"],
        "tags": card["tags"],
        "citations": citations,
    }
    emit_card(fm, body, card_id, force=force, emitted=emitted)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--plan-only", action="store_true")
    parser.add_argument("--force", action="store_true")
    parser.add_argument(
        "--legacy-root",
        type=Path,
        default=DEFAULT_LEGACY_ROOT,
        help="Root holding .claude/knowledge/<reading>/ legacy card bodies "
        "(default: $KB_LEGACY_ROOT or /home/jakeshea/CFA_reading)",
    )
    args = parser.parse_args()
    legacy_root = args.legacy_root

    manifest = read_json(OUT / "chunks_manifest.json")
    chunks_by_id = {c["chunk_id"]: c for c in manifest["chunks"]}
    retracted_chunks = set(manifest.get("retracted_chunk_ids", []))
    retracted_sources = set(manifest.get("retracted_source_ids", []))
    allowed = set(read_json(SOURCE_MATRIX)["allowed"][READING_ID])

    cfa_reg = read_json(CFA_REGISTRY)
    ms_reg = read_json(MULTI_SOURCE_REGISTRY)
    cfa_ids = [c["card_id"] for c in cfa_reg["cards"]]
    ms_ids = [c["card_id"] for c in ms_reg["cards"]]
    if set(cfa_ids) != set(CARD_IDS):
        raise SystemExit(f"CFA registry card set drift: {set(cfa_ids) ^ set(CARD_IDS)}")
    if set(ms_ids) != set(MULTI_SOURCE_CARD_IDS):
        raise SystemExit(f"multi-source registry card set drift: {set(ms_ids) ^ set(MULTI_SOURCE_CARD_IDS)}")

    # Negative test: every single-source offset map must re-derive each
    # verified_evidence triple and carry >=3 confirmed triples.
    # Validated once up front for both --plan-only and emission.
    validated_maps = {s: validate_single_source_map(s) for s in sorted(SINGLE_SOURCE_MAP_SOURCES)}

    emitted: list[Path] = []
    plan: dict[str, Any] = {}
    for card in cfa_reg["cards"] + ms_reg["cards"]:
        emit_registry_card(
            card,
            legacy_root=legacy_root,
            allowed=allowed,
            chunks_by_id=chunks_by_id,
            retracted_chunks=retracted_chunks,
            retracted_sources=retracted_sources,
            validated_maps=validated_maps,
            force=args.force,
            plan_only=args.plan_only,
            emitted=emitted,
            plan=plan,
        )

    write_atomic(
        REGISTRY / "pm_09_slice_citation_plan.json",
        json.dumps(
            {"schema_version": "cfa.pm_09_slice.v2", "cards": plan},
            indent=2, sort_keys=True, ensure_ascii=False,
        ) + "\n",
    )
    print(json.dumps({
        "emitted": [str(p.relative_to(ROOT)) for p in emitted],
        "card_count": len(plan),
        "cfa_anchor_deviations": cfa_reg.get("legacy_anchor_deviation_count"),
        "offset_maps_validated": {
            s: {"offset": v["offset"], "confirmed_evidence": v["confirmed_evidence"]}
            for s, v in validated_maps.items()
        },
        "plan_only": args.plan_only,
    }, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
