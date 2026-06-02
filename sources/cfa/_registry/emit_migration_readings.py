#!/usr/bin/env python3
"""Emit the three migrated cacg.v0 readings into ``cards/cfa/``.

Renders the cards for

  * ``14_microstructure_and_trading``  (73 ``mt-`` cards),
  * ``15_performance_and_attribution`` (35 ``pa-`` cards),
  * ``22_fund_level_arbitrage``        (26 ``fa-`` cards)

— 134 cards total — from the three committed curated citation registries
(``mt_14_curated_citations.json`` / ``pa_15_curated_citations.json`` /
``fa_22_curated_citations.json``). Each registry already holds REAL,
``kb verify``-confirmed ``chunk_id``/``chunk_hash`` bindings produced by
``resolve_migration_citations.py``; this emitter only renders cards from that
curated truth — it never re-derives or re-matches a quote (see the
curated-registry-not-heuristic discipline used by ``emit_09_pm_slice.py``).

For every card the metadata (``title``/``summary``/``tags``/``citations``) comes
from the registry and the card BODY is read VERBATIM from the read-only authored
skeleton at

    <skeleton-root>/<set-dir>/_card_skeletons/<reading_id>/<card_id>.md

where ``<skeleton-root>`` defaults to ``~/CFA_reading/deferred_books`` (override
with ``--skeleton-root`` or ``$KB_MIGRATION_SKELETON_ROOT``). The skeleton repo
is the read-only origin and is never written.

Trust discipline (fail-closed, every card, BEFORE any write):
  * the registry card-id set for a reading must EQUAL the set of skeleton files
    on disk for that reading AND match the published cardinality (73/35/26);
  * card ids carry the reading's prefix, are unique within the registry, are
    unique across the three readings, and collide with no already-published
    card in ``cards_manifest.json``;
  * every citation validates against the frozen ``chunks_manifest.json`` — the
    source is authorized for the reading and not retracted; the chunk exists,
    is not retracted, and its ``source_id`` matches; ``chunk_hash`` matches; the
    ``page_range`` lies within the chunk's ``[start_page, end_page]`` span; and
    the quote is an exact, artifact-free substring of the chunk text (the same
    raw containment check the published emitters use, which the resolver's
    bindings are built to satisfy).
Any failure aborts the run (non-zero exit) before a single card is written.

Frontmatter is rendered in canonical ``cacg.v0`` key order WITHOUT ``card_hash``;
``card_hash`` is stamped later by ``kb index``. With unchanged inputs the run is
a no-op (cards regenerate byte-identically).

The run also reconciles ``card_migration_queue.json``: it replaces any prior
records for these 134 ids and appends one record per emitted card
(``card_id``, ``reading_id``, ``notes_taint: false``, the ``source_ids`` the card
cites, ``title``, ``target_path``), plus a ``migration_additions`` provenance
block. The legacy preflight records are preserved byte-for-byte and the rewrite
is idempotent.
"""

from __future__ import annotations

import argparse
import json
import os
import unicodedata
from pathlib import Path
from typing import Any, NamedTuple

from cross_link_map import (
    DEFAULT_MAP,
    apply_prose_rewrites,
    derive_links,
    inject_links,
    load_map,
    prose_rewrites_by_card,
)

ROOT = Path(__file__).resolve().parents[3]
REGISTRY = ROOT / "sources/cfa/_registry"
OUT = ROOT / "out/cfa"
CARDS_ROOT = ROOT / "cards/cfa"
SOURCE_MATRIX = OUT / "source_matrix.json"
CHUNKS_MANIFEST = OUT / "chunks_manifest.json"
CARDS_MANIFEST = OUT / "cards_manifest.json"
QUEUE = REGISTRY / "card_migration_queue.json"

CFA_READING = Path(os.environ.get("KB_CFA_READING", str(Path.home() / "CFA_reading")))
DEFAULT_SKELETON_ROOT = Path(
    os.environ.get("KB_MIGRATION_SKELETON_ROOT", str(CFA_READING / "deferred_books"))
)

REGISTRY_SCHEMA_VERSION = "cfa.migration_curated_citations.v1"
SCHEMA_VERSION = "cacg.v0"
KEY_ORDER = ["schema_version", "id", "title", "reading_id", "summary", "tags", "card_edges", "citations", "card_hash"]
CITATION_ORDER = ["source_id", "chunk_id", "chunk_hash", "page_range", "quote", "edge_type"]
REQUIRED_CITATION_FIELDS = ("source_id", "chunk_id", "chunk_hash", "page_range", "quote", "edge_type")


class Reading(NamedTuple):
    reading_id: str
    registry_name: str
    set_dir: str
    prefix: str
    card_count: int
    skeleton_base: Path


# The migrated readings. ``card_count`` is the published cardinality and acts as a
# fixed tripwire: a skeleton directory that silently gains or loses a card is rejected
# even if the registry agrees with it. 14/15/22 live under deferred_books/; the 10/11
# template decks live directly under CFA_reading/ (their skeleton_base differs). The
# read-only skeleton dir for each is <skeleton_base>/<set_dir>/_card_skeletons/<reading_id>/.
READINGS: list[Reading] = [
    Reading("14_microstructure_and_trading", "mt_14_curated_citations.json",
            "14_Microstructure_and_Trading", "mt-", 73, DEFAULT_SKELETON_ROOT),
    Reading("15_performance_and_attribution", "pa_15_curated_citations.json",
            "15_Performance_and_Attribution", "pa-", 35, DEFAULT_SKELETON_ROOT),
    Reading("22_fund_level_arbitrage", "fa_22_curated_citations.json",
            "22_Fund_Level_Arbitrage", "fa-", 26, DEFAULT_SKELETON_ROOT),
    Reading("10_behavioral_finance", "bf_10_curated_citations.json",
            "10_Behavioral_Finance", "be-", 64, CFA_READING),
    Reading("11_risk_management", "rm_11_curated_citations.json",
            "11_Risk_Management", "rm-", 28, CFA_READING),
]

QUEUE_ADDITIONS_SCHEMA = "cfa.migration_queue_additions.v1"


def read_json(path: Path) -> Any:
    if not path.is_file():
        raise SystemExit(f"required input is not a regular file: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def write_atomic(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(content, encoding="utf-8")
    tmp.replace(path)


def split_skeleton(md: Path) -> str:
    """Return the card BODY (everything after the frontmatter) read verbatim."""
    if not md.is_file():
        raise SystemExit(f"missing skeleton card body: {md}")
    text = md.read_text(encoding="utf-8")
    parts = text.split("---", 2)
    if len(parts) < 3:
        raise SystemExit(f"{md}: malformed frontmatter")
    return parts[2].lstrip("\n")


def illegal_text_chars(text: str) -> list[str]:
    """Quote characters that are extraction artifacts / illegal in card text:
    control (Cc), format (Cf, e.g. soft-hyphen U+00AD, ZWSP U+200B), line/
    paragraph separators (Zl/Zp), and Unicode noncharacters (incl. U+FFFE from
    the Pdfium STX->U+FFFE map). Newlines are Cc and thus rejected — quotes must
    be single-line."""
    bad: list[str] = []
    for c in text:
        o = ord(c)
        noncharacter = (0xFDD0 <= o <= 0xFDEF) or (o & 0xFFFE) == 0xFFFE
        if noncharacter or unicodedata.category(c) in {"Cc", "Cf", "Zl", "Zp"}:
            bad.append(hex(o))
    return bad


def render_scalar(value: Any) -> str:
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, str):
        return '"' + value.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n") + '"'
    if isinstance(value, int):
        return str(value)
    raise TypeError(f"unsupported scalar: {value!r}")


def render_card(frontmatter: dict[str, Any], body: str) -> str:
    lines = ["---"]
    for key in KEY_ORDER:
        if key not in frontmatter or frontmatter[key] in (None, [], {}):
            continue
        value = frontmatter[key]
        if key == "tags":
            lines.append("tags: [" + ", ".join(render_scalar(t) for t in value) + "]")
        elif key == "citations":
            lines.append("citations:")
            for cit in value:
                first = True
                for ckey in CITATION_ORDER:
                    if ckey not in cit:
                        continue
                    cv = cit[ckey]
                    rendered = f"[{cv[0]}, {cv[1]}]" if ckey == "page_range" else render_scalar(cv)
                    lines.append(("  - " if first else "    ") + f"{ckey}: {rendered}")
                    first = False
        else:
            lines.append(f"{key}: {render_scalar(value)}")
    lines.append("---")
    return "\n".join(lines) + "\n" + body


def validate_citation(
    cit: dict[str, Any],
    *,
    card_id: str,
    reading_id: str,
    allowed: set[str],
    chunks_by_id: dict[str, dict],
    retracted_chunks: set[str],
    retracted_sources: set[str],
) -> None:
    """Abort the run (non-zero exit) unless every trust check passes for ``cit``."""
    missing = [f for f in REQUIRED_CITATION_FIELDS if f not in cit]
    if missing:
        raise SystemExit(f"{card_id}: citation missing required field(s) {missing}")
    src = cit["source_id"]
    if src not in allowed:
        raise SystemExit(f"{card_id}: source {src!r} not authorized for {reading_id}")
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
    if not (isinstance(pr, list) and len(pr) == 2 and all(isinstance(x, int) and not isinstance(x, bool) for x in pr)):
        raise SystemExit(f"{card_id}: page_range {pr!r} is not an [int, int] pair ({cit['chunk_id']})")
    # Validate WITHIN-SPAN: chunks span multiple pages, so the cited page need
    # not equal the chunk's start_page.
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


def load_registry_cards(reading: Reading) -> dict[str, dict[str, Any]]:
    """Load a reading's registry into a ``{card_id: card}`` map, rejecting
    duplicate ids at this trust boundary (do not silently last-wins)."""
    doc = read_json(REGISTRY / reading.registry_name)
    if doc.get("schema_version") != REGISTRY_SCHEMA_VERSION:
        raise SystemExit(
            f"{reading.registry_name}: schema_version {doc.get('schema_version')!r} != {REGISTRY_SCHEMA_VERSION!r}"
        )
    if doc.get("reading_id") != reading.reading_id:
        raise SystemExit(
            f"{reading.registry_name}: reading_id {doc.get('reading_id')!r} != {reading.reading_id!r}"
        )
    cards = doc.get("cards")
    if not isinstance(cards, list):
        raise SystemExit(f"{reading.registry_name}: no 'cards' array")
    by_id: dict[str, dict[str, Any]] = {}
    duplicates: list[str] = []
    for card in cards:
        cid = card["card_id"]
        if cid in by_id:
            duplicates.append(cid)
            continue
        by_id[cid] = card
    if duplicates:
        raise SystemExit(f"{reading.registry_name}: duplicate card ids {sorted(set(duplicates))}")
    return by_id


def skeleton_dir_for(reading: Reading, override_root: Path | None) -> Path:
    """The read-only skeleton directory for a reading. Each reading defaults to its
    own deck base (14/15/22 -> deferred_books, 10/11 -> CFA_reading); a non-None
    ``override_root`` (--skeleton-root / $KB_MIGRATION_SKELETON_ROOT) overrides the base
    for ALL readings (used by tests / relocated checkouts)."""
    base = override_root if override_root is not None else reading.skeleton_base
    return base / reading.set_dir / "_card_skeletons" / reading.reading_id


def skeleton_ids(reading: Reading, override_root: Path | None) -> set[str]:
    """The authored card-id set on disk for a reading (every ``*.md`` whose name
    is not ``INDEX.md`` and does not start with ``_``)."""
    d = skeleton_dir_for(reading, override_root)
    if not d.is_dir():
        raise SystemExit(f"skeleton directory not found: {d}")
    ids: set[str] = set()
    for md in sorted(d.glob("*.md")):
        if md.name == "INDEX.md" or md.name.startswith("_"):
            continue
        ids.add(md.stem)
    return ids


def guard_reading_id_set(reading: Reading, registry_ids: set[str], disk_ids: set[str]) -> None:
    """Fail closed unless the registry id set exactly matches the skeleton id set,
    matches the published cardinality, and every id carries the reading prefix."""
    if registry_ids != disk_ids:
        raise SystemExit(
            f"{reading.reading_id}: registry/skeleton card-id drift — "
            f"registry-only={sorted(registry_ids - disk_ids)} "
            f"skeleton-only={sorted(disk_ids - registry_ids)}"
        )
    if len(registry_ids) != reading.card_count:
        raise SystemExit(
            f"{reading.reading_id}: expected {reading.card_count} cards, found {len(registry_ids)}"
        )
    bad_prefix = sorted(i for i in registry_ids if not i.startswith(reading.prefix))
    if bad_prefix:
        raise SystemExit(f"{reading.reading_id}: card ids without {reading.prefix!r} prefix: {bad_prefix}")


def queue_record(card: dict[str, Any], reading: Reading) -> dict[str, Any]:
    source_ids = sorted({c["source_id"] for c in card["citations"]})
    cid = card["card_id"]
    return {
        "card_id": cid,
        "reading_id": reading.reading_id,
        "notes_taint": False,
        "source_ids": source_ids,
        "title": card["title"],
        "target_path": f"cards/cfa/{reading.reading_id}/{cid}.md",
        "migration_bucket": "deferred_book_skeleton_emitted",
        "migration_origin": "deferred_books_skeleton",
        "problems": [],
    }


def reconcile_queue(queue: dict[str, Any], new_records: list[dict[str, Any]]) -> dict[str, Any]:
    """Return the queue with the migrated records reconciled in: any prior record
    for one of these card ids is removed, then the fresh records are appended in
    card-id order. Legacy records keep their positions and bytes; the rewrite is
    idempotent."""
    mine = {r["card_id"] for r in new_records}
    kept = [c for c in queue["cards"] if c["card_id"] not in mine]
    queue = dict(queue)
    queue["cards"] = kept + sorted(new_records, key=lambda r: r["card_id"])
    reading_counts: dict[str, int] = {}
    for r in new_records:
        reading_counts[r["reading_id"]] = reading_counts.get(r["reading_id"], 0) + 1
    queue["migration_additions"] = {
        "schema_version": QUEUE_ADDITIONS_SCHEMA,
        "generated_by": "emit_migration_readings.py",
        "origin": "deferred_books_skeleton",
        "count": len(new_records),
        "notes_taint": False,
        "reading_counts": dict(sorted(reading_counts.items())),
    }
    return queue


def emit(override_root: Path | None, *, force: bool, dry_run: bool, cross_links_path: Path,
         readings: set[str] | None = None) -> dict[str, Any]:
    manifest = read_json(CHUNKS_MANIFEST)
    chunks_by_id = {c["chunk_id"]: c for c in manifest["chunks"]}
    retracted_chunks = set(manifest.get("retracted_chunk_ids", []))
    retracted_sources = set(manifest.get("retracted_source_ids", []))
    matrix_allowed = read_json(SOURCE_MATRIX)["allowed"]

    # Scope the run to the requested readings (default: every reading this emitter owns).
    # A migration that lands additively scopes to its own decks so it writes ONLY its
    # net-new cards and never touches the already-released readings (whose on-disk cards
    # carry a `kb index`-stamped `card_hash` the emitter does not render).
    known = {r.reading_id for r in READINGS}
    if readings is not None:
        unknown = sorted(readings - known)
        if unknown:
            raise SystemExit(f"--readings names reading(s) this emitter does not own: {unknown}")
    selected = [r for r in READINGS if readings is None or r.reading_id in readings]
    if not selected:
        raise SystemExit("no readings selected to emit")

    # The exact net-new id set this run owns = the skeleton ids across the selected
    # readings. ``published_ids`` is every already-published id EXCEPT those, so a new
    # card colliding with ANY pre-existing card — INCLUDING a live card in a migrated
    # reading (the 5 live be-/24 live rm- cards) — is caught, while a re-emit of an
    # already-released migrated card is not flagged as a self-collision (its own id is
    # now in the manifest but also in the emitted set, so it is excluded).
    emitted_ids: set[str] = set()
    for reading in selected:
        emitted_ids |= skeleton_ids(reading, override_root)
    published_ids = {c["id"] for c in read_json(CARDS_MANIFEST)["cards"]} - emitted_ids
    # Cross-reading See Also links for the documented overlap pairs are injected
    # into each new card's body from the committed map (the released-card side is
    # applied separately, since those cards are not emitted from skeletons). The map
    # also carries deterministic skeleton-prose rewrites (e.g. to drop a withdrawn
    # counterpart's stale over-claiming prose that link injection cannot remove).
    cross_doc = load_map(cross_links_path)
    new_card_links = derive_links(cross_doc)[0]
    rewrites_by_card = prose_rewrites_by_card(cross_doc)

    # Validate EVERYTHING before writing a single card (fail closed, no partial state).
    planned: list[tuple[Path, str]] = []
    new_records: list[dict[str, Any]] = []
    seen_new: dict[str, str] = {}
    for reading in selected:
        if reading.reading_id not in matrix_allowed:
            raise SystemExit(f"{reading.reading_id}: not authorized in source_matrix")
        allowed = set(matrix_allowed[reading.reading_id])
        by_id = load_registry_cards(reading)
        registry_ids = set(by_id)
        disk_ids = skeleton_ids(reading, override_root)
        guard_reading_id_set(reading, registry_ids, disk_ids)
        for cid in sorted(registry_ids):
            if cid in seen_new:
                raise SystemExit(f"card id {cid!r} appears in both {seen_new[cid]} and {reading.reading_id}")
            if cid in published_ids:
                raise SystemExit(f"card id {cid!r} collides with an already-published card")
            seen_new[cid] = reading.reading_id
            card = by_id[cid]
            citations = card["citations"]
            if not citations:
                raise SystemExit(f"{cid}: registry entry has no citations")
            for cit in citations:
                validate_citation(
                    cit, card_id=cid, reading_id=reading.reading_id, allowed=allowed,
                    chunks_by_id=chunks_by_id, retracted_chunks=retracted_chunks,
                    retracted_sources=retracted_sources,
                )
            body = split_skeleton(skeleton_dir_for(reading, override_root) / f"{cid}.md")
            body = apply_prose_rewrites(body, rewrites_by_card.get(cid, []))
            if cid in new_card_links:
                body = inject_links(body, new_card_links[cid])
            fm = {
                "schema_version": SCHEMA_VERSION,
                "id": cid,
                "title": card["title"],
                "reading_id": reading.reading_id,
                "summary": card["summary"],
                "tags": card["tags"],
                "citations": citations,
            }
            target = CARDS_ROOT / reading.reading_id / f"{cid}.md"
            planned.append((target, render_card(fm, body)))
            new_records.append(queue_record(card, reading))

    written = 0
    if not dry_run:
        for target, text in planned:
            if target.exists() and not force:
                if target.read_text(encoding="utf-8") == text:
                    continue
                raise SystemExit(f"{target}: exists and differs; rerun with --force")
            write_atomic(target, text)
            written += 1
        queue = read_json(QUEUE)
        write_atomic(
            QUEUE,
            json.dumps(reconcile_queue(queue, new_records), ensure_ascii=False, indent=2, sort_keys=True) + "\n",
        )

    return {
        "readings": {r.reading_id: r.card_count for r in selected},
        "cards_planned": len(planned),
        "cards_written": written,
        "queue_records": len(new_records),
        "dry_run": dry_run,
    }


def _self_test() -> int:
    # render_card: canonical order, no card_hash, page_range as [a, b].
    fm = {
        "schema_version": "cacg.v0", "id": "mt-x", "title": "T", "reading_id": "14_microstructure_and_trading",
        "summary": "S", "tags": ["a", "b"],
        "citations": [{"source_id": "s", "chunk_id": "s:p1:0001", "chunk_hash": "h",
                       "page_range": [3, 4], "quote": "q", "edge_type": "defines"}],
    }
    out = render_card(fm, "BODY\n")
    assert "card_hash" not in out, "card_hash must not be rendered"
    assert out.index("id:") < out.index("title:") < out.index("citations:"), "key order"
    assert "page_range: [3, 4]" in out and "tags: [\"a\", \"b\"]" in out, "scalar rendering"
    assert out.endswith("---\nBODY\n"), "body appended after frontmatter"

    # illegal_text_chars flags U+FFFE / soft-hyphen / newline, accepts clean text.
    assert illegal_text_chars("clean ascii") == []
    assert illegal_text_chars("a￾b") and illegal_text_chars("a­b") and illegal_text_chars("a\nb")

    chunk = {"chunk_id": "s:p1:0001", "source_id": "s", "chunk_hash": "h",
             "start_page": 1, "end_page": 3, "text": "hello world on page two"}
    chunks = {chunk["chunk_id"]: chunk}
    base: dict[str, Any] = dict(source_id="s", chunk_id="s:p1:0001", chunk_hash="h", page_range=[2, 2],
                                quote="world on page", edge_type="supports")
    kw: dict[str, Any] = dict(card_id="mt-x", reading_id="14_microstructure_and_trading", allowed={"s"},
                              chunks_by_id=chunks, retracted_chunks=set(), retracted_sources=set())
    validate_citation(dict(base), **kw)  # passes

    def rejects(mutation: dict[str, Any], **over) -> bool:
        cit = dict(base); cit.update(mutation)
        try:
            validate_citation(cit, **{**kw, **over})
            return False
        except SystemExit:
            return True

    assert rejects({"source_id": "other"})                       # unauthorized source
    assert rejects({"chunk_id": "s:p9:9999"})                    # missing chunk
    assert rejects({"chunk_hash": "nope"})                       # hash mismatch
    assert rejects({"page_range": [5, 5]})                       # outside chunk span
    assert rejects({"page_range": [2]})                          # malformed page_range
    assert rejects({"quote": "not present verbatim"})            # non-substring
    assert rejects({"quote": "world￾"})                     # illegal char (also non-substring)
    assert rejects({}, allowed=set())                            # source not authorized
    assert rejects({}, retracted_sources={"s"})                  # retracted source
    cit_missing = dict(base); del cit_missing["edge_type"]
    try:
        validate_citation(cit_missing, **kw); raise AssertionError("missing field not caught")
    except SystemExit:
        pass

    # id-set guard: exact match passes; any drift / wrong count / bad prefix fails.
    r = READINGS[0]
    guard_reading_id_set(r, {f"mt-{i}" for i in range(r.card_count)}, {f"mt-{i}" for i in range(r.card_count)})
    for bad in (
        lambda: guard_reading_id_set(r, {"mt-1"}, {"mt-1", "mt-2"}),                       # skeleton-only
        lambda: guard_reading_id_set(r, {"mt-1", "mt-extra"}, {"mt-1"}),                   # registry-only
        lambda: guard_reading_id_set(r, {"mt-1"}, {"mt-1"}),                               # wrong count
        lambda: guard_reading_id_set(r, {f"mt-{i}" for i in range(r.card_count - 1)} | {"pa-x"},
                                     {f"mt-{i}" for i in range(r.card_count - 1)} | {"pa-x"}),  # bad prefix
    ):
        try:
            bad(); raise AssertionError("id-set guard missed a drift")
        except SystemExit:
            pass

    # duplicate-id rejection at registry load (simulated via the same logic).
    dup_seen: dict[str, int] = {}
    dups = []
    for cid in ["mt-a", "mt-a", "mt-b"]:
        if cid in dup_seen:
            dups.append(cid)
        else:
            dup_seen[cid] = 1
    assert dups == ["mt-a"]

    # queue reconcile is idempotent and preserves foreign records.
    q = {"cards": [{"card_id": "legacy-1", "x": 1}], "summary": {"active_legacy_cards": 1}}
    recs = [queue_record({"card_id": "mt-z", "title": "Z",
                          "citations": [{"source_id": "s2"}, {"source_id": "s1"}, {"source_id": "s1"}]}, READINGS[0])]
    once = reconcile_queue(q, recs)
    twice = reconcile_queue(once, recs)
    assert once == twice, "queue reconcile not idempotent"
    assert {"legacy-1", "mt-z"} == {c["card_id"] for c in once["cards"]}
    mt_z = next(c for c in once["cards"] if c["card_id"] == "mt-z")
    assert mt_z["notes_taint"] is False and mt_z["source_ids"] == ["s1", "s2"]
    assert once["migration_additions"]["count"] == 1

    # cross-link injection: convert an existing prose code-span to a resolving link,
    # append a missing one, and stay idempotent on a second pass.
    body = ("# T\n\n## See Also\n- [`mt-x`](./mt-x.md) — intra.\n"
            "- `mt-implementation-shortfall` (reading 14) owns the measure.\n\n"
            "## Escalate\nfoo\n")
    links = [{"target_id": "mt-implementation-shortfall", "note": "n1"},
             {"target_id": "fa-tracking-error-attribution-and-tco", "note": "n2"}]
    once_b = inject_links(body, links)
    assert "[`mt-implementation-shortfall`](../14_microstructure_and_trading/mt-implementation-shortfall.md)" in once_b
    assert "[`fa-tracking-error-attribution-and-tco`](../22_fund_level_arbitrage/fa-tracking-error-attribution-and-tco.md) — n2" in once_b
    assert once_b.count("/mt-implementation-shortfall.md)") == 1, "no double-conversion"
    assert "## Escalate\nfoo" in once_b, "later section preserved"
    assert inject_links(once_b, links) == once_b, "injection not idempotent"

    # prose rewrite: exact find->replace; fail-closed when the find text is absent.
    rb = apply_prose_rewrites("a\n- A and B derive same thing\nz\n",
                              [{"card_id": "c", "find": "- A and B derive same thing", "replace": "- A derives it"}])
    assert "- A derives it" in rb and "and B" not in rb, "rewrite did not apply"
    try:
        apply_prose_rewrites("no match", [{"card_id": "c", "find": "absent line", "replace": "x"}])
        raise AssertionError("missing find not caught")
    except SystemExit:
        pass

    print("emit_migration_readings self-test: PASS")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--skeleton-root", type=Path, default=None,
                    help="override the deck base for ALL readings (each reading otherwise uses its "
                         "own base: 14/15/22 -> deferred_books, 10/11 -> CFA_reading). The skeleton "
                         "dir is <base>/<set-dir>/_card_skeletons/<reading_id>/ (tests / relocated checkouts)")
    ap.add_argument("--readings", type=str, default=None,
                    help="comma-separated reading_id(s) to emit (default: every reading this emitter "
                         "owns). An additive migration scopes to its own decks, e.g. "
                         "--readings 10_behavioral_finance,11_risk_management")
    ap.add_argument("--force", action="store_true",
                    help="overwrite an existing card whose bytes differ from the rendered output")
    ap.add_argument("--dry-run", action="store_true", help="validate everything, write nothing")
    ap.add_argument("--cross-links", type=Path, default=DEFAULT_MAP,
                    help="committed cross-link map injected into new-card See Also sections "
                         "(default: migration_cross_links.json)")
    ap.add_argument("--self-test", action="store_true", help="run hermetic helper tests and exit")
    args = ap.parse_args()
    if args.self_test:
        return _self_test()
    readings = {r.strip() for r in args.readings.split(",") if r.strip()} if args.readings else None
    result = emit(args.skeleton_root, force=args.force, dry_run=args.dry_run,
                  cross_links_path=args.cross_links, readings=readings)
    print(json.dumps(result, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
