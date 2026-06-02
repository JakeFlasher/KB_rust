#!/usr/bin/env python3
"""Build a preflight queue for migrating legacy CFA cards to CACG cards.

The output is intentionally conservative: it resolves legacy source paths to
canonical `source_id`s and checks authorization, but it does not pretend legacy
book-page spans are already verified CACG page ranges.
"""

from __future__ import annotations

import hashlib
import json
import os
import re
import tempfile
from collections import Counter, defaultdict
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[3]
LEGACY_ROOT = Path("/home/jakeshea/CFA_reading/CFA_reading")
REGISTRY = ROOT / "sources/cfa/_registry"
EXCLUDED = ROOT / "sources/cfa/excluded"
OUT = ROOT / "out/cfa"
FROZEN_TIMESTAMP = "1970-01-01T00:00:00Z"

SOURCE_REF_RE = re.compile(
    r"(?P<path>.+?\.(?:pdf|PDF))\s+"
    r"(?:(?:Vol\.(?P<volume>\d+)/)?pp\.(?P<start>\d+)(?:-(?P<end>\d+))?)"
)

READING_ID_BY_LEGACY_DIR = {
    "01_quantitative_methods": "01_quantitative_methods",
    "02_economics": "02_economics",
    "03_financial_reporting_analysis": "03_financial_reporting_analysis",
    "05_equity": "05_equity",
    "06_fixed_income_and_credit": "06_fixed_income_and_credit",
    "07_derivatives_and_volatility": "07_derivatives_and_volatility",
    "08_convertible_bonds": "08_convertible_bonds",
    "09_portfolio_management_and_asset_pricing": "09_portfolio_management_and_asset_pricing",
    "10_behavioral_finance": "10_behavioral_finance",
    "11_risk_management": "11_risk_management",
    "17_cross_cutting": "17_cross_cutting",
}


def read_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def display_path(path: Path) -> str:
    try:
        return path.relative_to(ROOT).as_posix()
    except ValueError:
        try:
            return path.relative_to(LEGACY_ROOT).as_posix()
        except ValueError:
            return str(path)


def atomic_write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, tmp_name = tempfile.mkstemp(
        prefix=f".{path.name}.",
        suffix=".tmp",
        dir=path.parent,
        text=True,
    )
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as f:
            f.write(text)
            f.flush()
            os.fsync(f.fileno())
        os.replace(tmp_name, path)
    finally:
        if os.path.exists(tmp_name):
            os.unlink(tmp_name)


def write_json(path: Path, payload: Any) -> None:
    atomic_write_text(
        path,
        json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
    )


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def generated_at_utc() -> str:
    if os.environ.get("KB_FROZEN_CLOCK", "1") == "1":
        return FROZEN_TIMESTAMP
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def file_fingerprint(path: Path) -> dict[str, Any]:
    stat = path.stat()
    return {
        "path": display_path(path),
        "bytes": stat.st_size,
        "sha256": sha256(path),
    }


def active_card_set_fingerprint(
    card_paths: list[Path],
    card_hashes: dict[Path, str],
) -> dict[str, Any]:
    h = hashlib.sha256()
    for path in card_paths:
        rel = path.relative_to(LEGACY_ROOT).as_posix()
        h.update(rel.encode("utf-8"))
        h.update(b"\0")
        h.update(card_hashes[path].encode("ascii"))
        h.update(b"\n")
    return {
        "root": str(LEGACY_ROOT),
        "count": len(card_paths),
        "sha256": h.hexdigest(),
    }


def build_input_fingerprints(
    card_paths: list[Path],
    card_hashes: dict[Path, str],
) -> dict[str, Any]:
    return {
        "schema_version": "cfa.preflight_input_fingerprints.v1",
        "files": {
            "legacy_path_map": file_fingerprint(REGISTRY / "legacy_path_map.json"),
            "source_matrix": file_fingerprint(OUT / "source_matrix.json"),
            "legacy_notes_taint_manifest": file_fingerprint(
                EXCLUDED / "legacy_notes_taint_manifest.json"
            ),
            "sources_manifest": file_fingerprint(OUT / "sources_manifest.json"),
            "chunks_manifest": file_fingerprint(OUT / "chunks_manifest.json"),
        },
        "legacy_active_card_set": active_card_set_fingerprint(card_paths, card_hashes),
    }


def artifact_set_id(generated_at: str, input_fingerprints: dict[str, Any]) -> str:
    payload = json.dumps(
        {
            "generated_at_utc": generated_at,
            "input_fingerprints": input_fingerprints,
        },
        ensure_ascii=False,
        sort_keys=True,
        separators=(",", ":"),
    )
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()


def active_card_paths() -> list[Path]:
    root = LEGACY_ROOT / ".claude/knowledge"
    cards = []
    for path in sorted(root.glob("*/*.md")):
        if path.name.startswith("_") or path.name in {"README.md", "INDEX.md"}:
            continue
        cards.append(path)
    return cards


def split_legacy_card(path: Path) -> tuple[str, str]:
    text = path.read_text(encoding="utf-8", errors="replace")
    if not text.startswith("---\n"):
        raise ValueError(f"{path}: missing legacy frontmatter delimiter")
    parts = text.split("---", 2)
    if len(parts) < 3:
        raise ValueError(f"{path}: malformed legacy frontmatter delimiter")
    return parts[1].strip("\n"), parts[2].lstrip("\n")


def parse_legacy_frontmatter(fm_text: str) -> dict[str, Any]:
    out: dict[str, Any] = {}
    current_key: str | None = None
    for raw_line in fm_text.splitlines():
        if not raw_line.strip():
            continue
        if not raw_line.startswith((" ", "\t")) and ":" in raw_line:
            key, value = raw_line.split(":", 1)
            current_key = key.strip()
            out[current_key] = value.strip()
            continue
        if current_key is None:
            continue
        stripped = raw_line.strip()
        if stripped.startswith("- "):
            prior = out.get(current_key)
            if not isinstance(prior, list):
                prior = [] if prior == "" else [str(prior)]
                out[current_key] = prior
            prior.append(stripped[2:].strip())
        elif isinstance(out.get(current_key), list) and out[current_key]:
            out[current_key][-1] = f"{out[current_key][-1]} {stripped}".strip()
        else:
            prior = out.get(current_key, "")
            out[current_key] = f"{prior} {stripped}".strip() if prior else stripped
    for list_key in ("Supporting sources", "Repo touchpoints", "edges"):
        if list_key in out and not isinstance(out[list_key], list):
            out[list_key] = [] if out[list_key] == "" else [out[list_key]]
    return out


def title_from_body(body: str, fallback: str) -> str:
    for line in body.splitlines():
        if line.startswith("# "):
            return line[2:].strip()
    return fallback.replace("-", " ").title()


def summary_from_use_when(use_when: str, title: str) -> str:
    base = f"{title}: {use_when}".strip()
    if len(base) < 80:
        base = (
            f"{base} This migrated legacy card preserves the original CFA knowledge "
            "scope pending citation remapping."
        )
    if len(base) > 400:
        base = base[:397].rstrip() + "..."
    return base


def is_empty_source_marker(value: str) -> bool:
    normalized = value.strip().strip(".").lower()
    return normalized in {"", "-", "none", "n/a", "na"}


def normalize_ref_path(path: str) -> str:
    return path.strip().replace("\\", "/")


def build_source_resolver(rows: list[dict[str, Any]]) -> tuple[dict[str, dict[str, Any]], dict[str, list[dict[str, Any]]]]:
    exact: dict[str, dict[str, Any]] = {}
    by_basename: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        if row.get("status") != "active":
            continue
        legacy_path = normalize_ref_path(row["legacy_path"])
        exact[legacy_path] = row
        by_basename[Path(legacy_path).name].append(row)
    return exact, by_basename


def resolve_source_ref(
    source_path: str,
    exact: dict[str, dict[str, Any]],
    by_basename: dict[str, list[dict[str, Any]]],
) -> tuple[dict[str, Any] | None, str]:
    normalized = normalize_ref_path(source_path)
    if normalized in exact:
        return exact[normalized], "exact"

    suffix_hits = [row for legacy_path, row in exact.items() if legacy_path.endswith(normalized)]
    if len(suffix_hits) == 1:
        return suffix_hits[0], "suffix"
    if len(suffix_hits) > 1:
        return None, "ambiguous_suffix"

    basename_hits = by_basename.get(Path(normalized).name, [])
    if len(basename_hits) == 1:
        return basename_hits[0], "basename"
    if len(basename_hits) > 1:
        return None, "ambiguous_basename"
    return None, "unmapped"


def parse_source_ref(
    raw_ref: str,
    role: str,
    exact: dict[str, dict[str, Any]],
    by_basename: dict[str, list[dict[str, Any]]],
    chunks_by_source: dict[str, list[dict[str, Any]]],
    source_page_counts: dict[str, int],
) -> dict[str, Any]:
    raw_ref = raw_ref.strip()
    match = SOURCE_REF_RE.search(raw_ref)
    ref: dict[str, Any] = {
        "role": role,
        "raw_ref": raw_ref,
        "parsed": bool(match),
        "source_id": None,
        "resolve_method": None,
        "legacy_source_path": None,
        "volume": None,
        "legacy_page_start": None,
        "legacy_page_end": None,
        "page_coordinate_status": "unparsed",
        "candidate_chunk_count_if_legacy_pages_are_pdf_pages": 0,
        "candidate_chunk_ids_sample_if_legacy_pages_are_pdf_pages": [],
        "source_page_count": None,
        "problems": [],
    }
    if not match:
        ref["problems"].append("source_ref_unparsed")
        return ref

    source_path = normalize_ref_path(match.group("path"))
    start = int(match.group("start"))
    end = int(match.group("end") or match.group("start"))
    volume = match.group("volume")
    ref.update(
        {
            "legacy_source_path": source_path,
            "volume": int(volume) if volume else None,
            "legacy_page_start": start,
            "legacy_page_end": end,
        }
    )
    if end < start:
        ref["problems"].append("legacy_page_range_reversed")

    row, method = resolve_source_ref(source_path, exact, by_basename)
    ref["resolve_method"] = method
    if row is None:
        ref["problems"].append(f"source_ref_{method}")
        return ref

    source_id = row["source_id"]
    ref["source_id"] = source_id
    ref["canonical_path"] = row["canonical_path"]
    ref["source_category"] = row.get("category")
    ref["source_audit_rating"] = row.get("audit_rating")
    ref["source_review_flags"] = row.get("review_flags", [])
    ref["source_page_count"] = source_page_counts.get(source_id)
    if ref["source_page_count"] is not None and end > ref["source_page_count"]:
        ref["problems"].append("legacy_page_range_exceeds_source_page_count")

    if volume is not None and source_id.startswith("cfa_"):
        ref["page_coordinate_status"] = "cfa_volume_book_pages_require_offset"
    else:
        ref["page_coordinate_status"] = "legacy_book_pages_require_offset_verification"

    overlapping_chunks = [
        chunk
        for chunk in chunks_by_source.get(source_id, [])
        if int(chunk["end_page"]) >= start and int(chunk["start_page"]) <= end
    ]
    ref["candidate_chunk_count_if_legacy_pages_are_pdf_pages"] = len(overlapping_chunks)
    ref["candidate_chunk_ids_sample_if_legacy_pages_are_pdf_pages"] = [
        chunk["chunk_id"] for chunk in overlapping_chunks[:5]
    ]
    return ref


def load_source_page_counts() -> dict[str, int]:
    sources = read_json(OUT / "sources_manifest.json")["sources"]
    return {str(source["source_id"]): int(source["page_count"]) for source in sources}


def load_chunks_index() -> dict[str, list[dict[str, Any]]]:
    chunks = read_json(OUT / "chunks_manifest.json")["chunks"]
    by_source: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for chunk in chunks:
        sid = chunk["source_id"]
        by_source[sid].append(chunk)
    for source_chunks in by_source.values():
        source_chunks.sort(key=lambda c: (int(c["start_page"]), int(c["ordinal"]), c["chunk_id"]))
    return dict(by_source)


def build_queue() -> dict[str, Any]:
    path_map = read_json(REGISTRY / "legacy_path_map.json")
    matrix = read_json(OUT / "source_matrix.json")
    notes_taint = read_json(EXCLUDED / "legacy_notes_taint_manifest.json")
    card_paths = active_card_paths()
    card_hashes = {path: sha256(path) for path in card_paths}
    input_fingerprints = build_input_fingerprints(card_paths, card_hashes)
    tainted_paths = {
        entry["legacy_path"]
        for entry in notes_taint["entries"]
        if entry.get("is_active_card")
    }
    chunks_by_source = load_chunks_index()
    source_page_counts = load_source_page_counts()
    exact, by_basename = build_source_resolver(path_map)

    cards: list[dict[str, Any]] = []
    unique_ref_map: dict[str, dict[str, Any]] = {}
    bucket_counts: Counter[str] = Counter()
    problem_counts: Counter[str] = Counter()
    source_ref_counts: Counter[str] = Counter()
    reading_counts: Counter[str] = Counter()

    for card_path in card_paths:
        rel = card_path.relative_to(LEGACY_ROOT).as_posix()
        legacy_reading_dir = card_path.parent.name
        reading_id = READING_ID_BY_LEGACY_DIR.get(legacy_reading_dir, legacy_reading_dir)
        fm_text, body = split_legacy_card(card_path)
        fm = parse_legacy_frontmatter(fm_text)
        card_id = card_path.stem
        title = title_from_body(body, card_id)

        raw_refs: list[tuple[str, str]] = []
        primary = str(fm.get("Primary raw source", "")).strip()
        if not is_empty_source_marker(primary):
            raw_refs.append(("primary", primary))
        for supporting in fm.get("Supporting sources", []) or []:
            supporting_text = str(supporting).strip()
            if not is_empty_source_marker(supporting_text):
                raw_refs.append(("supporting", supporting_text))

        refs = [
            parse_source_ref(
                raw_ref,
                role,
                exact,
                by_basename,
                chunks_by_source,
                source_page_counts,
            )
            for role, raw_ref in raw_refs
        ]

        source_ids = sorted({ref["source_id"] for ref in refs if ref.get("source_id")})
        allowed = set(matrix["allowed"].get(reading_id, []))
        unauthorized = sorted(sid for sid in source_ids if sid not in allowed)
        unmapped = [ref for ref in refs if ref.get("source_id") is None]
        unparsed = [ref for ref in refs if not ref.get("parsed")]
        has_notes_taint = rel in tainted_paths

        problems = []
        if has_notes_taint:
            problems.append("notes_taint")
        if unparsed:
            problems.append("source_ref_unparsed")
        if unmapped:
            problems.append("source_ref_unmapped")
        if unauthorized:
            problems.append("source_id_unauthorized_for_reading")
        if not refs:
            problems.append("no_legacy_source_refs")
        if any(ref["legacy_page_end"] and ref["source_page_count"] and ref["legacy_page_end"] > ref["source_page_count"] for ref in refs):
            problems.append("legacy_page_range_exceeds_source_page_count")

        if has_notes_taint:
            bucket = "quarantine_notes_taint"
        elif unparsed or unmapped:
            bucket = "blocked_source_ref_resolution"
        elif unauthorized:
            bucket = "blocked_source_authorization"
        elif not refs:
            bucket = "blocked_no_source_refs"
        else:
            bucket = "ready_for_offset_and_quote_mapping"

        for problem in problems:
            problem_counts[problem] += 1
        for ref in refs:
            for problem in ref.get("problems", []):
                problem_counts[problem] += 1
            if ref.get("legacy_source_path"):
                key = ref["legacy_source_path"]
                source_ref_counts[key] += 1
                unique_ref_map.setdefault(
                    key,
                    {
                        "legacy_source_path": key,
                        "source_id": ref.get("source_id"),
                        "resolve_method": ref.get("resolve_method"),
                        "canonical_path": ref.get("canonical_path"),
                    },
                )

        bucket_counts[bucket] += 1
        reading_counts[reading_id] += 1

        legacy_deliverable_ready = str(fm.get("deliverable-ready", "")).lower() == "true"
        cacg_emission_blockers = sorted(
            {
                *problems,
                "page_offset_verification_required",
                "quote_selection_required",
                "chunk_hash_binding_required",
            }
        )

        cards.append(
            {
                "card_id": card_id,
                "legacy_path": rel,
                "legacy_sha256": card_hashes[card_path],
                "legacy_reading_dir": legacy_reading_dir,
                "reading_id": reading_id,
                "target_path": f"cards/cfa/{reading_id}/{card_id}.md",
                "title": title,
                "summary_candidate": summary_from_use_when(str(fm.get("Use when", "")), title),
                "cfa_relevance": fm.get("CFA Relevance"),
                "source_stance": fm.get("Source Stance"),
                "deliverable_ready": legacy_deliverable_ready,
                "deliverable_ready_semantics": "legacy_frontmatter_only_not_a_migration_gate",
                "notes_taint": has_notes_taint,
                "migration_bucket": bucket,
                "eligible_for_offset_quote_mapping": bucket == "ready_for_offset_and_quote_mapping",
                "eligible_for_cacg_emission": False,
                "cacg_emission_blockers": cacg_emission_blockers,
                "problems": problems,
                "source_ids": source_ids,
                "unauthorized_source_ids": unauthorized,
                "source_refs": refs,
                "card_edges_legacy": fm.get("edges", []),
                "repo_touchpoints_legacy": fm.get("Repo touchpoints", []),
                "body_bytes": len(body.encode("utf-8")),
            }
        )

    generated_at = generated_at_utc()
    set_id = artifact_set_id(generated_at, input_fingerprints)
    queue = {
        "schema_version": "cfa.card_migration_preflight.v1",
        "generated_at_utc": generated_at,
        "artifact_set_id": set_id,
        "input_fingerprints": input_fingerprints,
        "legacy_root": str(LEGACY_ROOT),
        "workspace_root": str(ROOT),
        "summary": {
            "active_legacy_cards": len(cards),
            "bucket_counts": dict(sorted(bucket_counts.items())),
            "problem_counts": dict(sorted(problem_counts.items())),
            "reading_counts": dict(sorted(reading_counts.items())),
            "unique_legacy_source_paths_referenced": len(unique_ref_map),
            "unique_resolved_source_ids_referenced": len({sid for c in cards for sid in c["source_ids"]}),
        },
        "cards": cards,
    }
    ref_map = {
        "schema_version": "cfa.legacy_source_ref_map.v1",
        "generated_at_utc": generated_at,
        "artifact_set_id": set_id,
        "input_fingerprints": input_fingerprints,
        "summary": {
            "unique_legacy_source_paths_referenced": len(unique_ref_map),
            "total_source_ref_occurrences": sum(source_ref_counts.values()),
        },
        "entries": [
            {
                **unique_ref_map[path],
                "occurrences": source_ref_counts[path],
            }
            for path in sorted(unique_ref_map)
        ],
    }
    return {"queue": queue, "ref_map": ref_map}


def build_page_offset_worklist(queue: dict[str, Any]) -> dict[str, Any]:
    grouped: dict[tuple[str, str], dict[str, Any]] = {}
    for card in queue["cards"]:
        for ref in card["source_refs"]:
            source_id = ref.get("source_id")
            legacy_source_path = ref.get("legacy_source_path")
            if not source_id or not legacy_source_path:
                continue
            key = (source_id, legacy_source_path)
            group = grouped.setdefault(
                key,
                {
                    "source_id": source_id,
                    "legacy_source_path": legacy_source_path,
                    "canonical_path": ref.get("canonical_path"),
                    "source_category": ref.get("source_category"),
                    "source_audit_rating": ref.get("source_audit_rating"),
                    "source_review_flags": ref.get("source_review_flags", []),
                    "source_page_count": ref.get("source_page_count"),
                    "coordinate_statuses": set(),
                    "reading_ids": set(),
                    "card_ids": set(),
                    "ranges": [],
                },
            )
            group["coordinate_statuses"].add(ref["page_coordinate_status"])
            group["reading_ids"].add(card["reading_id"])
            group["card_ids"].add(card["card_id"])
            group["ranges"].append(
                {
                    "card_id": card["card_id"],
                    "legacy_path": card["legacy_path"],
                    "role": ref["role"],
                    "volume": ref.get("volume"),
                    "legacy_page_start": ref.get("legacy_page_start"),
                    "legacy_page_end": ref.get("legacy_page_end"),
                    "candidate_chunk_count_if_legacy_pages_are_pdf_pages": ref.get(
                        "candidate_chunk_count_if_legacy_pages_are_pdf_pages"
                    ),
                    "candidate_chunk_ids_sample_if_legacy_pages_are_pdf_pages": ref.get(
                        "candidate_chunk_ids_sample_if_legacy_pages_are_pdf_pages", []
                    ),
                }
            )

    entries = []
    for group in grouped.values():
        statuses = sorted(group["coordinate_statuses"])
        ranges = group["ranges"]
        starts = [r["legacy_page_start"] for r in ranges if r["legacy_page_start"] is not None]
        ends = [r["legacy_page_end"] for r in ranges if r["legacy_page_end"] is not None]
        volumes = sorted({r["volume"] for r in ranges if r["volume"] is not None})
        category = group.get("source_category") or ""
        flags = group.get("source_review_flags") or []
        if "cfa_volume_book_pages_require_offset" in statuses:
            next_action = "build_cfa_volume_to_pdf_offset_map"
        elif any("current_version_sensitive" in flag for flag in flags):
            next_action = "verify_current_version_and_page_mapping"
        elif category == "china_convertible_bonds":
            next_action = "verify_pdf_page_mapping_for_regulatory_or_research_pdf"
        else:
            next_action = "verify_book_to_pdf_page_offset"
        entries.append(
            {
                "source_id": group["source_id"],
                "legacy_source_path": group["legacy_source_path"],
                "canonical_path": group["canonical_path"],
                "source_category": group.get("source_category"),
                "source_audit_rating": group.get("source_audit_rating"),
                "source_review_flags": flags,
                "source_page_count": group.get("source_page_count"),
                "occurrence_count": len(ranges),
                "card_count": len(group["card_ids"]),
                "reading_ids": sorted(group["reading_ids"]),
                "coordinate_statuses": statuses,
                "legacy_page_min": min(starts) if starts else None,
                "legacy_page_max": max(ends) if ends else None,
                "volumes": volumes,
                "next_action": next_action,
                "sample_ranges": ranges[:12],
            }
        )
    entries.sort(key=lambda e: (e["next_action"], e["source_id"], e["legacy_source_path"]))
    action_counts = Counter(entry["next_action"] for entry in entries)
    return {
        "schema_version": "cfa.page_offset_worklist.v1",
        "generated_at_utc": queue["generated_at_utc"],
        "artifact_set_id": queue["artifact_set_id"],
        "input_fingerprints": queue["input_fingerprints"],
        "summary": {
            "source_entries": len(entries),
            "next_action_counts": dict(sorted(action_counts.items())),
        },
        "entries": entries,
    }


def write_summary(queue: dict[str, Any], ref_map: dict[str, Any]) -> None:
    cards = queue["cards"]
    by_bucket = Counter(card["migration_bucket"] for card in cards)
    by_problem = Counter(problem for card in cards for problem in card["problems"])
    blocked = [card for card in cards if card["migration_bucket"] != "ready_for_offset_and_quote_mapping"]
    lines = [
        "# CFA Legacy Card Migration Preflight",
        "",
        f"Generated: `{queue['generated_at_utc']}`",
        f"Artifact set: `{queue['artifact_set_id']}`",
        "",
        "## Summary",
        "",
        f"- Active legacy cards scanned: {len(cards)}",
        f"- Ready for offset/quote mapping: {by_bucket.get('ready_for_offset_and_quote_mapping', 0)}",
        f"- Notes-tainted quarantine: {by_bucket.get('quarantine_notes_taint', 0)}",
        f"- Source resolution blocked: {by_bucket.get('blocked_source_ref_resolution', 0)}",
        f"- Source authorization blocked: {by_bucket.get('blocked_source_authorization', 0)}",
        f"- Unique legacy source paths referenced: {ref_map['summary']['unique_legacy_source_paths_referenced']}",
        f"- Total source-reference occurrences: {ref_map['summary']['total_source_ref_occurrences']}",
        f"- Legacy active-card set hash: `{queue['input_fingerprints']['legacy_active_card_set']['sha256']}`",
        "",
        "## Buckets",
        "",
    ]
    for bucket, count in sorted(by_bucket.items()):
        lines.append(f"- `{bucket}`: {count}")
    lines.extend(["", "## Problems", ""])
    if by_problem:
        for problem, count in sorted(by_problem.items()):
            lines.append(f"- `{problem}`: {count}")
    else:
        lines.append("- none")
    lines.extend(
        [
            "",
            "## Blocked Samples",
            "",
        ]
    )
    for card in blocked[:40]:
        problems = ", ".join(card["problems"]) or card["migration_bucket"]
        lines.append(f"- `{card['legacy_path']}`: {problems}")
    if len(blocked) > 40:
        lines.append(f"- ... {len(blocked) - 40} more blocked cards")
    lines.extend(
        [
            "",
            "## Migration Caveat",
            "",
            "A card in `ready_for_offset_and_quote_mapping` is not ready to emit as CACG yet. "
            "It only means its legacy source paths resolve to active `source_id`s and are "
            "authorized for the target `reading_id`. The next pass must verify book-page to "
            "PDF-page offsets and choose verbatim chunk quotes before writing CACG cards.",
            "",
            "`deliverable_ready` is preserved only as legacy frontmatter metadata. Downstream "
            "migration tooling must gate on `migration_bucket`, `notes_taint`, and "
            "`eligible_for_offset_quote_mapping`; every card remains "
            "`eligible_for_cacg_emission: false` until quote and chunk-hash binding finish.",
            "",
            "A source-level worklist for that next pass is written to "
            "`page_offset_worklist.json`.",
            "",
        ]
    )
    atomic_write_text(REGISTRY / "card_migration_preflight_summary.md", "\n".join(lines))


def main() -> int:
    outputs = build_queue()
    offset_worklist = build_page_offset_worklist(outputs["queue"])
    write_json(REGISTRY / "card_migration_queue.json", outputs["queue"])
    write_json(REGISTRY / "legacy_source_ref_map.json", outputs["ref_map"])
    write_json(REGISTRY / "page_offset_worklist.json", offset_worklist)
    write_summary(outputs["queue"], outputs["ref_map"])
    print(json.dumps(outputs["queue"]["summary"], ensure_ascii=False, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
