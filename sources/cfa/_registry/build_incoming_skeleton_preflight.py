#!/usr/bin/env python3
"""Preflight inventory for the incoming microstructure / performance /
fund-level card skeletons before they are migrated into the cfa corpus.

This is a read-only proof, run BEFORE any PDF is copied or ingested, that every
incoming skeleton is structurally ready to become a real cfa card:

  * the three sets carry exactly their expected card counts;
  * every card declares schema ``cacg.v0`` and carries neither a ``card_hash``
    nor a ``card_edges`` field (those are stamped/attached downstream);
  * every citation still holds the unresolved ``PENDING_INGEST`` placeholder for
    both ``chunk_id`` and ``chunk_hash``;
  * every citation names a ``source_id`` that the set's ``_sources.json`` knows,
    with an integer ``page_range`` inside that source's declared page count;
  * no incoming ``card_id`` collides with the live corpus, and no two incoming
    cards share an id.

The origin lives in a sibling, read-only repository; nothing here writes to it.
The check is fail-closed (any problem exits non-zero), deterministic under
``KB_FROZEN_CLOCK``, and ships a ``--self-test`` that drives every rejection
branch from synthetic inputs so the helpers can be trusted without disk.
"""

from __future__ import annotations

import argparse
import json
import os
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, NamedTuple

import yaml


ROOT = Path(__file__).resolve().parents[3]
REGISTRY = ROOT / "sources/cfa/_registry"
CARDS_MANIFEST = ROOT / "out/cfa/cards_manifest.json"
CARDS_DIR = ROOT / "cards/cfa"
ORIGIN = Path(os.environ.get("KB_DEFERRED_ORIGIN", "/home/jakeshea/CFA_reading/deferred_books"))
CFA_READING = Path(os.environ.get("KB_CFA_READING", "/home/jakeshea/CFA_reading"))
REPORT_PATH = REGISTRY / "incoming_skeleton_preflight.json"
FROZEN_TIMESTAMP = "1970-01-01T00:00:00Z"

PLACEHOLDER = "PENDING_INGEST"
SKELETON_SCHEMA = "cacg.v0"
SKIP_FILES = {"INDEX.md", "README.md", "SCOPE_EXCEPTION.md"}
EDGE_TYPES = {"defines", "supports"}
REQUIRED_CITATION_FIELDS = ("source_id", "chunk_id", "chunk_hash", "page_range", "quote", "edge_type")

# Each incoming set: (origin subdirectory, target reading_id, card-id prefix,
# expected card count). The reading_id doubles as the skeleton subfolder name.
# 14/15/22 live under deferred_books/<subdir>/ with ``_sources.json`` at the set
# root and skeletons under ``_card_skeletons/<reading_id>/``.
INCOMING_SETS = (
    ("14_Microstructure_and_Trading", "14_microstructure_and_trading", "mt-", 73),
    ("15_Performance_and_Attribution", "15_performance_and_attribution", "pa-", 35),
    ("22_Fund_Level_Arbitrage", "22_fund_level_arbitrage", "fa-", 26),
)
# The two template decks (10 Behavioral Finance / 11 Risk Management) live directly
# under CFA_reading/<subdir>/ and differ from the deferred_books layout: their
# ``_sources.json`` sits INSIDE ``_card_skeletons/`` (next to the ``<reading_id>/``
# subfolder), per register_template_decks.py.
TEMPLATE_DECK_SETS = (
    ("10_Behavioral_Finance", "10_behavioral_finance", "be-", 64),
    ("11_Risk_Management", "11_risk_management", "rm-", 28),
)


class SetSpec(NamedTuple):
    """One incoming set with its paths already resolved, so the evaluator is
    layout-agnostic (the deferred_books and template-deck layouts differ only in
    where ``_sources.json`` lives relative to the skeleton subfolder)."""
    reading_id: str
    prefix: str
    expected_count: int
    sources_json: Path
    skeleton_dir: Path


def set_specs() -> list[SetSpec]:
    specs: list[SetSpec] = []
    for origin_subdir, reading_id, prefix, count in INCOMING_SETS:
        set_dir = ORIGIN / origin_subdir
        specs.append(SetSpec(reading_id, prefix, count,
                             set_dir / "_sources.json",
                             set_dir / "_card_skeletons" / reading_id))
    for origin_subdir, reading_id, prefix, count in TEMPLATE_DECK_SETS:
        skel_root = CFA_READING / origin_subdir / "_card_skeletons"
        specs.append(SetSpec(reading_id, prefix, count,
                             skel_root / "_sources.json",
                             skel_root / reading_id))
    return specs


def generated_at_utc() -> str:
    if os.environ.get("KB_FROZEN_CLOCK", "1") == "1":
        return FROZEN_TIMESTAMP
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def atomic_write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, tmp_name = tempfile.mkstemp(prefix=f".{path.name}.", suffix=".tmp", dir=path.parent, text=True)
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
    atomic_write_text(path, json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n")


def is_int(value: Any) -> bool:
    """True only for genuine integers; rejects bool (a Python int subclass)."""
    return isinstance(value, int) and not isinstance(value, bool)


def parse_frontmatter(text: str) -> dict[str, Any]:
    """Return the YAML frontmatter block of a card as a dict.

    Raises ValueError if the ``---`` delimited block is missing or is not a
    mapping, so a malformed skeleton fails closed rather than parsing as empty.
    """
    if not text.startswith("---\n"):
        raise ValueError("missing frontmatter opening delimiter")
    end = text.find("\n---", 4)
    if end == -1:
        raise ValueError("missing frontmatter closing delimiter")
    block = text[4:end]
    data = yaml.safe_load(block)
    if not isinstance(data, dict):
        raise ValueError("frontmatter is not a mapping")
    return data


def existing_card_ids() -> tuple[set[str], list[str]]:
    """The live corpus id universe: published-manifest ids unioned with the ids
    found on disk, so a collision against either surface is caught.

    Returns ``(ids, problems)``. A card that cannot be parsed during id
    collection is a hard problem (not a silent skip): a collision proof that
    quietly drops an unreadable existing card could miss a real id clash.
    """
    ids: set[str] = set()
    problems: list[str] = []
    if CARDS_MANIFEST.is_file():
        manifest = json.loads(CARDS_MANIFEST.read_text(encoding="utf-8"))
        cards = manifest["cards"] if isinstance(manifest, dict) else manifest
        for card in cards:
            cid = card.get("id")
            if cid:
                ids.add(str(cid))
    if CARDS_DIR.is_dir():
        for path in CARDS_DIR.glob("*/*.md"):
            if path.name.startswith("_") or path.name in SKIP_FILES:
                continue
            try:
                fm = parse_frontmatter(path.read_text(encoding="utf-8"))
            except ValueError as exc:
                problems.append(f"existing_card_unparseable:{path.relative_to(ROOT)}:{exc}")
                continue
            cid = fm.get("id")
            if cid:
                ids.add(str(cid))
            else:
                problems.append(f"existing_card_missing_id:{path.relative_to(ROOT)}")
    return ids, problems


def load_source_page_counts(sources_json: Path) -> dict[str, int]:
    """Map ``source_id`` -> declared page count from a set's ``_sources.json``.

    Rejects a missing/non-positive/non-integer ``pages`` and a duplicate
    ``source_id`` (a duplicate would let a citation's page bound be validated
    against the wrong source). Raises ValueError so the caller records a
    structural problem rather than trusting malformed source metadata.
    """
    data = json.loads(sources_json.read_text(encoding="utf-8"))
    counts: dict[str, int] = {}
    for source in data["sources"]:
        sid = str(source["source_id"])
        if sid in counts:
            raise ValueError(f"duplicate source_id in _sources.json: {sid}")
        pages = source.get("pages")
        if not is_int(pages) or pages <= 0:
            raise ValueError(f"source {sid}: pages must be a positive integer, got {pages!r}")
        counts[sid] = int(pages)
    return counts


class CitationOutcome:
    """Result of one per-citation check: problems plus a single-page flag.

    A length-1 ``page_range`` (``[p]``) is single-page shorthand. It is NOT a
    problem here (the skeletons are read-only INPUT, not the kernel's final
    two-element artifact; the downstream resolver expands ``[p]`` to ``[p, p]``
    when it binds), but it IS surfaced as a count so the later normalization
    pass has an explicit worklist rather than a silent pass. Anything other than
    length 1 or 2 (e.g. ``[]`` or ``[a, b, c]``) is a hard shape problem.
    """

    __slots__ = ("problems", "single_page")

    def __init__(self, problems: list[str], single_page: bool) -> None:
        self.problems = problems
        self.single_page = single_page


def check_citation(
    cit: Any,
    *,
    allowed_sources: dict[str, int],
    index: int,
) -> CitationOutcome:
    """Pure per-citation check."""
    problems: list[str] = []
    if not isinstance(cit, dict):
        return CitationOutcome([f"citation[{index}]_not_mapping"], False)
    for field in REQUIRED_CITATION_FIELDS:
        if field not in cit:
            problems.append(f"citation[{index}]_missing_{field}")
    if problems:
        return CitationOutcome(problems, False)

    source_id = cit["source_id"]
    page_count = allowed_sources.get(str(source_id))
    if page_count is None:
        problems.append(f"citation[{index}]_unknown_source_id:{source_id}")

    if cit["chunk_id"] != PLACEHOLDER or cit["chunk_hash"] != PLACEHOLDER:
        problems.append(f"citation[{index}]_not_pending_placeholder")

    page_range = cit["page_range"]
    single_page = False
    if not isinstance(page_range, list) or not (1 <= len(page_range) <= 2):
        problems.append(f"citation[{index}]_page_range_bad_shape")
    elif not all(is_int(p) for p in page_range):
        problems.append(f"citation[{index}]_page_range_not_integer")
    else:
        single_page = len(page_range) == 1
        lo = page_range[0]
        hi = page_range[-1]
        if lo > hi:
            problems.append(f"citation[{index}]_page_range_reversed")
        if lo < 1:
            problems.append(f"citation[{index}]_page_range_below_one")
        elif page_count is not None and hi > page_count:
            problems.append(f"citation[{index}]_page_range_exceeds_source_pages")

    if cit["edge_type"] not in EDGE_TYPES:
        problems.append(f"citation[{index}]_unknown_edge_type:{cit['edge_type']}")
    return CitationOutcome(problems, single_page)


def check_card(
    fm: dict[str, Any],
    *,
    prefix: str,
    reading_id: str,
    allowed_sources: dict[str, int],
    existing_ids: set[str],
) -> tuple[list[str], int]:
    """Pure per-card structural check.

    Returns ``(problems, single_page_citation_count)``: a problem list (empty
    when the card is migratable) and the number of its citations that use the
    single-page ``[p]`` shorthand.
    """
    problems: list[str] = []

    if fm.get("schema_version") != SKELETON_SCHEMA:
        problems.append("schema_version_not_cacg_v0")
    if "card_hash" in fm:
        value = fm["card_hash"]
        if isinstance(value, str) and set(value) <= {"0"} and value:
            problems.append("all_zero_card_hash")
        else:
            problems.append("unexpected_card_hash")
    if "card_edges" in fm:
        problems.append("unexpected_card_edges")

    card_id = fm.get("id")
    if not isinstance(card_id, str) or not card_id:
        problems.append("missing_id")
    else:
        if not card_id.startswith(prefix):
            problems.append("id_prefix_mismatch")
        if card_id in existing_ids:
            problems.append("id_collision_with_existing")

    if fm.get("reading_id") != reading_id:
        problems.append("reading_id_mismatch")

    single_page_count = 0
    citations = fm.get("citations")
    if not isinstance(citations, list) or not citations:
        problems.append("citations_missing_or_empty")
    else:
        for i, cit in enumerate(citations):
            outcome = check_citation(cit, allowed_sources=allowed_sources, index=i)
            problems.extend(outcome.problems)
            if outcome.single_page:
                single_page_count += 1
    return problems, single_page_count


def evaluate_set(spec: SetSpec, existing_ids: set[str]) -> dict[str, Any]:
    reading_id = spec.reading_id
    prefix = spec.prefix
    expected_count = spec.expected_count
    sources_json = spec.sources_json
    skeleton_dir = spec.skeleton_dir

    result: dict[str, Any] = {
        "reading_id": reading_id,
        "prefix": prefix,
        "expected_count": expected_count,
        "observed_count": 0,
        "skeleton_dir": str(skeleton_dir),
        "single_page_citations": 0,
        "problems": [],
        "card_ids": [],
    }

    if not sources_json.is_file():
        result["problems"].append(f"missing__sources.json:{sources_json}")
        return result
    if not skeleton_dir.is_dir():
        result["problems"].append(f"missing_skeleton_dir:{skeleton_dir}")
        return result

    try:
        allowed_sources = load_source_page_counts(sources_json)
    except (ValueError, KeyError) as exc:
        result["problems"].append(f"sources_manifest_invalid:{exc}")
        return result
    result["known_source_ids"] = sorted(allowed_sources)

    card_paths = [
        p
        for p in sorted(skeleton_dir.glob("*.md"))
        if p.name not in SKIP_FILES and not p.name.startswith("_")
    ]
    result["observed_count"] = len(card_paths)
    if len(card_paths) != expected_count:
        result["problems"].append(f"card_count_mismatch:expected={expected_count}:observed={len(card_paths)}")

    ids: list[str] = []
    for path in card_paths:
        try:
            fm = parse_frontmatter(path.read_text(encoding="utf-8"))
        except ValueError as exc:
            result["problems"].append(f"{path.name}:frontmatter_error:{exc}")
            continue
        card_problems, single_page = check_card(
            fm,
            prefix=prefix,
            reading_id=reading_id,
            allowed_sources=allowed_sources,
            existing_ids=existing_ids,
        )
        result["single_page_citations"] += single_page
        cid = fm.get("id")
        if isinstance(cid, str) and cid:
            ids.append(cid)
        for problem in card_problems:
            result["problems"].append(f"{path.name}:{problem}")

    result["card_ids"] = sorted(ids)
    return result


def run_checks(readings: set[str] | None = None) -> dict[str, Any]:
    existing_ids, existing_problems = existing_card_ids()
    sets: list[dict[str, Any]] = []
    all_incoming_ids: dict[str, str] = {}
    cross_set_problems: list[str] = list(existing_problems)

    specs = set_specs()
    if readings is not None:
        unknown = sorted(readings - {s.reading_id for s in specs})
        if unknown:
            raise SystemExit(f"--readings names unknown set(s): {unknown}")
        specs = [s for s in specs if s.reading_id in readings]
    for spec in specs:
        set_result = evaluate_set(spec, existing_ids)
        for cid in set_result["card_ids"]:
            if cid in all_incoming_ids:
                cross_set_problems.append(
                    f"incoming_id_collision:{cid}:{all_incoming_ids[cid]}:{spec.reading_id}"
                )
            else:
                all_incoming_ids[cid] = spec.reading_id
        sets.append(set_result)

    total_observed = sum(s["observed_count"] for s in sets)
    total_expected = sum(s["expected_count"] for s in sets)
    total_single_page = sum(s.get("single_page_citations", 0) for s in sets)
    set_problem_count = sum(len(s["problems"]) for s in sets)
    clean = set_problem_count == 0 and not cross_set_problems and total_observed == total_expected

    return {
        "schema_version": "cfa.incoming_skeleton_preflight.v1",
        "generated_at_utc": generated_at_utc(),
        "origin": str(ORIGIN),
        "live_corpus_card_ids": len(existing_ids),
        "totals": {
            "expected": total_expected,
            "observed": total_observed,
            "single_page_citations": total_single_page,
        },
        "cross_set_problems": cross_set_problems,
        "sets": sets,
        "clean": clean,
    }


# --------------------------------------------------------------------------- #
# Self-test: hermetic, exercises every rejection branch from synthetic inputs.
# --------------------------------------------------------------------------- #

def _good_card() -> dict[str, Any]:
    return {
        "schema_version": "cacg.v0",
        "id": "mt-sample-card",
        "title": "Sample",
        "reading_id": "14_microstructure_and_trading",
        "summary": "x" * 90,
        "tags": ["a"],
        "citations": [
            {
                "source_id": "mt_harris_2003_trading_and_exchanges",
                "chunk_id": "PENDING_INGEST",
                "chunk_hash": "PENDING_INGEST",
                "page_range": [10, 11],
                "quote": "some verbatim text",
                "edge_type": "defines",
            }
        ],
    }


def self_test() -> int:
    allowed: dict[str, int] = {"mt_harris_2003_trading_and_exchanges": 664}
    existing: set[str] = {"mt-already-present"}
    failures: list[str] = []

    def card_problems(fm: dict[str, Any]) -> list[str]:
        problems, _ = check_card(
            fm,
            prefix="mt-",
            reading_id="14_microstructure_and_trading",
            allowed_sources=allowed,
            existing_ids=existing,
        )
        return problems

    def card_single_page(fm: dict[str, Any]) -> int:
        _, single_page = check_card(
            fm,
            prefix="mt-",
            reading_id="14_microstructure_and_trading",
            allowed_sources=allowed,
            existing_ids=existing,
        )
        return single_page

    def expect(name: str, fm: dict[str, Any], needle: str) -> None:
        problems = card_problems(fm)
        if not any(needle in p for p in problems):
            failures.append(f"{name}: expected a problem containing {needle!r}, got {problems}")

    def expect_clean(name: str, fm: dict[str, Any]) -> None:
        problems = card_problems(fm)
        if problems:
            failures.append(f"{name}: expected no problems, got {problems}")

    expect_clean("good_card", _good_card())

    # Single-page [p] shorthand is migratable (no problem) but counted.
    fm = _good_card(); fm["citations"][0]["page_range"] = [10]
    if card_problems(fm):
        failures.append(f"single_page_clean: expected no problems, got {card_problems(fm)}")
    if card_single_page(fm) != 1:
        failures.append("single_page_count: [10] should count as one single-page citation")
    if card_single_page(_good_card()) != 0:
        failures.append("two_element_range must not count as single-page")

    fm = _good_card(); fm["card_hash"] = "deadbeef" * 8
    expect("real_card_hash", fm, "unexpected_card_hash")

    fm = _good_card(); fm["card_hash"] = "0" * 64
    expect("all_zero_card_hash", fm, "all_zero_card_hash")

    fm = _good_card(); fm["card_edges"] = []
    expect("card_edges_present", fm, "unexpected_card_edges")

    fm = _good_card(); fm["schema_version"] = "cacg.v1"
    expect("wrong_schema", fm, "schema_version_not_cacg_v0")

    fm = _good_card(); fm["id"] = "pa-wrong-prefix"
    expect("wrong_prefix", fm, "id_prefix_mismatch")

    fm = _good_card(); fm["id"] = "mt-already-present"
    expect("id_collision", fm, "id_collision_with_existing")

    fm = _good_card(); fm["reading_id"] = "99_wrong"
    expect("reading_id_mismatch", fm, "reading_id_mismatch")

    fm = _good_card(); fm["citations"] = []
    expect("no_citations", fm, "citations_missing_or_empty")

    fm = _good_card(); fm["citations"][0]["source_id"] = "mt_unknown_source"
    expect("unknown_source", fm, "unknown_source_id")

    fm = _good_card(); fm["citations"][0]["chunk_id"] = "mt_harris:p0010:0001"
    expect("resolved_chunk_id", fm, "not_pending_placeholder")

    fm = _good_card(); fm["citations"][0]["chunk_hash"] = "0" * 64
    expect("resolved_chunk_hash", fm, "not_pending_placeholder")

    fm = _good_card(); fm["citations"][0]["page_range"] = ["10", "11"]
    expect("non_integer_page", fm, "page_range_not_integer")

    fm = _good_card(); fm["citations"][0]["page_range"] = [True]
    expect("bool_page", fm, "page_range_not_integer")

    fm = _good_card(); fm["citations"][0]["page_range"] = [10, 20, 30]
    expect("page_range_too_long", fm, "page_range_bad_shape")

    fm = _good_card(); fm["citations"][0]["page_range"] = [9999]
    expect("page_over_count", fm, "page_range_exceeds_source_pages")

    fm = _good_card(); fm["citations"][0]["page_range"] = [0]
    expect("page_below_one", fm, "page_range_below_one")

    fm = _good_card(); fm["citations"][0]["page_range"] = [12, 10]
    expect("page_reversed", fm, "page_range_reversed")

    fm = _good_card(); fm["citations"][0]["edge_type"] = "contradicts"
    expect("bad_edge_type", fm, "unknown_edge_type")

    fm = _good_card(); del fm["citations"][0]["quote"]
    expect("missing_field", fm, "missing_quote")

    # is_int guards
    if is_int(True) or not is_int(5) or is_int("5"):
        failures.append("is_int guard wrong")

    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print(f"  - {f}")
        return 1
    print("SELF-TEST PASSED (structural negatives + single-page surfacing + is_int guard)")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run hermetic self-test and exit")
    parser.add_argument("--report", action="store_true", help="write the JSON report artifact")
    parser.add_argument("--readings", type=str, default=None,
                        help="comma-separated reading_id(s) to check (default: every incoming set). "
                             "A migration scopes to its currently-incoming decks, e.g. "
                             "--readings 10_behavioral_finance,11_risk_management (the already-migrated "
                             "14/15/22 sets collide with their now-live cards by design)")
    args = parser.parse_args()

    if args.self_test:
        return self_test()

    readings = {r.strip() for r in args.readings.split(",") if r.strip()} if args.readings else None
    report = run_checks(readings)
    if args.report:
        write_json(REPORT_PATH, report)

    print(
        json.dumps(
            {
                "clean": report["clean"],
                "totals": report["totals"],
                "set_problem_counts": {s["reading_id"]: len(s["problems"]) for s in report["sets"]},
                "cross_set_problems": len(report["cross_set_problems"]),
            },
            ensure_ascii=False,
            sort_keys=True,
        )
    )
    if not report["clean"]:
        for s in report["sets"]:
            for problem in s["problems"][:50]:
                print(f"  {s['reading_id']}: {problem}", file=sys.stderr)
        for problem in report["cross_set_problems"]:
            print(f"  cross-set: {problem}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
