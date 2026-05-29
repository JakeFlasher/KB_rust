#!/usr/bin/env python3
"""Build and validate the CFA-legacy v0 scope ledger.

The scope ledger is the machine-readable record that partitions every
legacy active card (the universe enumerated in
``card_migration_queue.json``) into exactly one disposition:

  * ``active_emitted``            -- a card present in ``cards_manifest.json``.
  * ``active_deferred_this_loop`` -- a non-tainted card not yet emitted,
                                     slated for emission once its single-
                                     source page-offset map exists
                                     (the Pedersen/Cochrane PM stragglers).
  * ``quarantined``               -- a notes-tainted card held out of the
                                     active corpus under Critical Rule 9.
  * ``excluded``                  -- a legacy card deliberately dropped
                                     from v0 scope for a recorded reason
                                     (empty today; reserved).

Disposition is derived from EMISSION REALITY (the manifest), not from the
queue's own status booleans, because several queue flags
(``eligible_for_cacg_emission``, the ``quarantine_notes_taint`` bucket)
are stale relative to what was ultimately emitted. The queue's per-card
``notes_taint`` boolean is still trusted as the taint signal, but only to
split the *un-emitted* remainder into quarantined vs deferred.

Invariants enforced (non-zero exit on any failure, so this doubles as a
re-runnable gate check):

  1. The four dispositions are pairwise disjoint.
  2. Their union equals the legacy active universe (274 cards).
  3. Every emitted card id is a known legacy card (no manifest orphan).
  4. Count reconciliation: emitted-card count == on-disk ``*.md`` count
     == ``.history.jsonl`` sidecar count == ``cards_manifest.json`` length.

Cards flagged ``notes_taint=true`` that were nonetheless emitted are
surfaced separately for provenance review (the queue taint flag was
superseded by a re-anchor to a primary source); they are NOT quarantined.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

LEDGER_SCHEMA_VERSION = "cfa_legacy_scope_ledger/v1"

# Cards deliberately dropped from v0 scope for a recorded reason. Empty
# today: every legacy active card is either emitted, deferred for
# emission this loop, or quarantined for notes-taint.
EXCLUDED_IDS: dict[str, str] = {}

QUARANTINE_REASON = "notes-taint (Critical Rule 9): authored from chat-notes-derived material"
QUARANTINE_REAUTHOR_CRITERION = (
    "re-author from a non-notes primary source (or under a sanctioned "
    "notes-provenance policy) before re-admitting to the active corpus"
)
DEFERRED_BLOCKER = "single-source page-offset map required before emission"


def find_disk_cards(cards_dir: Path) -> set[str]:
    """Card slugs on disk, matching the corpus reconciliation filter.

    Mirrors ``find <cards_dir> -name '*.md' ! -name '*.history*'
    ! -name '_*' ! -name 'INDEX.md'``: every ``*.md`` whose basename does
    not start with ``_`` and is not ``INDEX.md``. The slug is the file
    stem, which equals the card id by construction of the emitter.
    """
    slugs: set[str] = set()
    for md in cards_dir.rglob("*.md"):
        name = md.name
        if name == "INDEX.md" or name.startswith("_") or ".history" in name:
            continue
        slugs.add(md.stem)
    return slugs


def count_history_sidecars(cards_dir: Path) -> int:
    return sum(1 for _ in cards_dir.rglob("*.history.jsonl"))


def load_queue(path: Path) -> list[dict]:
    data = json.loads(path.read_text(encoding="utf-8"))
    cards = data.get("cards")
    if not isinstance(cards, list):
        raise SystemExit(f"queue {path} has no 'cards' array")
    return cards


def load_manifest_ids(path: Path) -> list[str]:
    data = json.loads(path.read_text(encoding="utf-8"))
    cards = data.get("cards")
    if not isinstance(cards, list):
        raise SystemExit(f"manifest {path} has no 'cards' array")
    return [c["id"] for c in cards]


def build_ledger(queue: list[dict], manifest_ids: list[str]) -> tuple[dict, list[str]]:
    """Return (ledger, failures). ``failures`` is empty iff every
    invariant holds."""
    failures: list[str] = []

    queue_by_id = {c["card_id"]: c for c in queue}
    queue_ids = set(queue_by_id)
    manifest_set = set(manifest_ids)

    # Manifest orphans: an emitted card the legacy universe does not know.
    orphans = sorted(manifest_set - queue_ids)
    if orphans:
        failures.append(f"manifest cards absent from the legacy queue: {orphans}")

    emitted = queue_ids & manifest_set
    unemitted = queue_ids - manifest_set

    quarantined = {cid for cid in unemitted if queue_by_id[cid].get("notes_taint") is True}
    deferred = {cid for cid in unemitted if cid not in quarantined}
    excluded = set(EXCLUDED_IDS)

    # Excluded ids, if any, must be carved out of whichever bucket they
    # would otherwise land in so the four sets stay disjoint.
    emitted -= excluded
    deferred -= excluded
    quarantined -= excluded

    # notes_taint=true cards that were emitted anyway: provenance was
    # superseded by a re-anchor; flag for review, do not quarantine.
    taint_emitted_review = sorted(
        cid for cid in emitted if queue_by_id[cid].get("notes_taint") is True
    )

    def reading_of(cid: str) -> str:
        return queue_by_id[cid].get("reading_id", "")

    ledger = {
        "schema_version": LEDGER_SCHEMA_VERSION,
        "legacy_active_total": len(queue_ids),
        "counts": {
            "active_emitted": len(emitted),
            "active_deferred_this_loop": len(deferred),
            "quarantined": len(quarantined),
            "excluded": len(excluded),
        },
        "categories": {
            "active_emitted": sorted(emitted),
            "active_deferred_this_loop": [
                {
                    "card_id": cid,
                    "reading_id": reading_of(cid),
                    "emitted": False,
                    "blocker": DEFERRED_BLOCKER,
                }
                for cid in sorted(deferred)
            ],
            "quarantined": [
                {
                    "card_id": cid,
                    "reading_id": reading_of(cid),
                    "reason": QUARANTINE_REASON,
                    "reauthor_criterion": QUARANTINE_REAUTHOR_CRITERION,
                }
                for cid in sorted(quarantined)
            ],
            "excluded": [
                {"card_id": cid, "reason": reason}
                for cid, reason in sorted(EXCLUDED_IDS.items())
            ],
        },
        "notes_taint_flag_emitted_for_review": taint_emitted_review,
    }

    # Invariant 1: pairwise disjoint.
    buckets = {
        "active_emitted": emitted,
        "active_deferred_this_loop": deferred,
        "quarantined": quarantined,
        "excluded": excluded,
    }
    names = list(buckets)
    for i in range(len(names)):
        for j in range(i + 1, len(names)):
            overlap = buckets[names[i]] & buckets[names[j]]
            if overlap:
                failures.append(
                    f"dispositions {names[i]} and {names[j]} overlap: {sorted(overlap)}"
                )

    # Invariant 2: union == universe.
    union = emitted | deferred | quarantined | excluded
    missing = sorted(queue_ids - union)
    extra = sorted(union - queue_ids)
    if missing:
        failures.append(f"legacy cards classified into no disposition: {missing}")
    if extra:
        failures.append(f"dispositions reference unknown card ids: {extra}")

    return ledger, failures


def main() -> int:
    repo = Path(__file__).resolve().parents[3]
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--queue", type=Path,
                    default=repo / "sources/cfa_legacy/_registry/card_migration_queue.json")
    ap.add_argument("--manifest", type=Path,
                    default=repo / "out/cfa_legacy/cards_manifest.json")
    ap.add_argument("--cards-dir", type=Path, default=repo / "cards/cfa_legacy")
    ap.add_argument("--out", type=Path,
                    default=repo / "sources/cfa_legacy/_registry/v0_baseline/scope_ledger.json")
    args = ap.parse_args()

    queue = load_queue(args.queue)
    manifest_ids = load_manifest_ids(args.manifest)
    ledger, failures = build_ledger(queue, manifest_ids)

    # Count reconciliation (Invariant 4).
    disk_slugs = find_disk_cards(args.cards_dir)
    disk_count = len(disk_slugs)
    sidecar_count = count_history_sidecars(args.cards_dir)
    manifest_count = len(manifest_ids)
    emitted_count = ledger["counts"]["active_emitted"]

    reconciliation = {
        "disk_md": disk_count,
        "manifest": manifest_count,
        "history_sidecars": sidecar_count,
        "ledger_active_emitted": emitted_count,
        "consistent": disk_count == manifest_count == sidecar_count == emitted_count,
    }
    ledger["reconciliation"] = reconciliation

    if not reconciliation["consistent"]:
        failures.append(
            "count reconciliation mismatch: "
            f"disk_md={disk_count}, manifest={manifest_count}, "
            f"sidecars={sidecar_count}, ledger_emitted={emitted_count}"
        )

    # On-disk slugs and the manifest id set must agree exactly.
    manifest_set = set(manifest_ids)
    disk_only = sorted(disk_slugs - manifest_set)
    manifest_only = sorted(manifest_set - disk_slugs)
    if disk_only:
        failures.append(f"on-disk cards absent from cards_manifest.json: {disk_only}")
    if manifest_only:
        failures.append(f"cards_manifest.json entries with no card on disk: {manifest_only}")

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(
        json.dumps(ledger, indent=2, sort_keys=True, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )

    print(f"scope ledger written to {args.out}")
    print(
        "counts: "
        f"active_emitted={ledger['counts']['active_emitted']}, "
        f"active_deferred_this_loop={ledger['counts']['active_deferred_this_loop']}, "
        f"quarantined={ledger['counts']['quarantined']}, "
        f"excluded={ledger['counts']['excluded']}, "
        f"universe={ledger['legacy_active_total']}"
    )
    print(
        "reconciliation: "
        f"disk_md={disk_count}, manifest={manifest_count}, "
        f"sidecars={sidecar_count}, consistent={reconciliation['consistent']}"
    )
    if ledger["notes_taint_flag_emitted_for_review"]:
        print(
            "NOTE: notes_taint=true but emitted (provenance review needed): "
            f"{ledger['notes_taint_flag_emitted_for_review']}"
        )

    if failures:
        print("\nSCOPE LEDGER GATE: FAIL", file=sys.stderr)
        for f in failures:
            print(f"  - {f}", file=sys.stderr)
        return 1

    print("\nSCOPE LEDGER GATE: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
