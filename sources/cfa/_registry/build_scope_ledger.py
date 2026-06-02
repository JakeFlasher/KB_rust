#!/usr/bin/env python3
"""Build and validate the CFA-legacy v0 scope ledger.

The scope ledger is the machine-readable record that partitions every
legacy active card (the universe enumerated in
``card_migration_queue.json``) into exactly one disposition:

  * ``active_emitted``            -- a baseline card present in
                                     ``cards_manifest.json``.
  * ``active_deferred_then_emitted_this_loop``
                                  -- a card deferred at baseline (it needed a
                                     single-source page-offset map) and emitted
                                     during THIS stabilization loop once that map
                                     was built (the 5 Pedersen/Cochrane PM cards).
                                     Present in the manifest, like
                                     ``active_emitted``, but tracked separately so
                                     the deferred->emitted transition is auditable.
  * ``active_deferred_this_loop`` -- a non-tainted card not yet emitted,
                                     slated for emission once its single-
                                     source page-offset map exists (empty now
                                     that the PM stragglers are emitted).
  * ``quarantined``               -- a notes-tainted card held out of the
                                     active corpus under Critical Rule 9.
  * ``excluded``                  -- a legacy card deliberately dropped
                                     from v0 scope for a recorded reason
                                     (empty today; reserved).

The active corpus is ``active_emitted`` + ``active_deferred_then_emitted_this_loop``
(both in the manifest); count reconciliation is against that sum.

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

LEDGER_SCHEMA_VERSION = "cfa_scope_ledger/v1"

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
RESOLVED_BLOCKER = "single-source page-offset map built"

# Cards deferred at baseline (no single-source offset map) and emitted during this
# stabilization loop once the Pedersen/Cochrane v1 offset maps were built. They live
# in cards_manifest.json like any active card, but are tracked as a disjoint bucket so
# the deferred->emitted transition stays auditable. Each MUST now be emitted.
DEFERRED_THEN_EMITTED_IDS: set[str] = {
    "pm-active-management-and-alpha",
    "pm-anomalies-and-cross-sectional-pricing",
    "pm-efficient-markets-and-anomalies",
    "pm-multifactor-asset-pricing-intuition",
    "pm-stochastic-discount-factor-intuition",
}


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

    emitted_all = queue_ids & manifest_set
    unemitted = queue_ids - manifest_set

    # Split the emitted set: cards deferred-at-baseline-but-emitted-this-loop are
    # tracked separately from the baseline active_emitted set. Every id in
    # DEFERRED_THEN_EMITTED_IDS must be a known legacy card AND actually emitted.
    dte_unknown = sorted(DEFERRED_THEN_EMITTED_IDS - queue_ids)
    if dte_unknown:
        failures.append(f"deferred-then-emitted ids absent from the legacy queue: {dte_unknown}")
    dte_not_emitted = sorted(DEFERRED_THEN_EMITTED_IDS - manifest_set)
    if dte_not_emitted:
        failures.append(
            f"deferred-then-emitted ids not present in cards_manifest.json: {dte_not_emitted}"
        )
    deferred_then_emitted = emitted_all & DEFERRED_THEN_EMITTED_IDS
    emitted = emitted_all - deferred_then_emitted

    quarantined = {cid for cid in unemitted if queue_by_id[cid].get("notes_taint") is True}
    deferred = {cid for cid in unemitted if cid not in quarantined}
    excluded = set(EXCLUDED_IDS)

    # Excluded ids, if any, must be carved out of whichever bucket they
    # would otherwise land in so the buckets stay disjoint.
    emitted -= excluded
    deferred_then_emitted -= excluded
    deferred -= excluded
    quarantined -= excluded

    # notes_taint=true cards that were emitted anyway: provenance was
    # superseded by a re-anchor; flag for review, do not quarantine.
    taint_emitted_review = sorted(
        cid for cid in (emitted | deferred_then_emitted)
        if queue_by_id[cid].get("notes_taint") is True
    )

    def reading_of(cid: str) -> str:
        return queue_by_id[cid].get("reading_id", "")

    ledger = {
        "schema_version": LEDGER_SCHEMA_VERSION,
        "legacy_active_total": len(queue_ids),
        "active_corpus_total": len(emitted) + len(deferred_then_emitted),
        "counts": {
            "active_emitted": len(emitted),
            "active_deferred_then_emitted_this_loop": len(deferred_then_emitted),
            "active_deferred_this_loop": len(deferred),
            "quarantined": len(quarantined),
            "excluded": len(excluded),
        },
        "categories": {
            "active_emitted": sorted(emitted),
            "active_deferred_then_emitted_this_loop": [
                {
                    "card_id": cid,
                    "reading_id": reading_of(cid),
                    "emitted": True,
                    "resolved_blocker": RESOLVED_BLOCKER,
                }
                for cid in sorted(deferred_then_emitted)
            ],
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
        "active_deferred_then_emitted_this_loop": deferred_then_emitted,
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
    union = emitted | deferred_then_emitted | deferred | quarantined | excluded
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
                    default=repo / "sources/cfa/_registry/card_migration_queue.json")
    ap.add_argument("--manifest", type=Path,
                    default=repo / "out/cfa/cards_manifest.json")
    ap.add_argument("--cards-dir", type=Path, default=repo / "cards/cfa")
    ap.add_argument("--out", type=Path,
                    default=repo / "sources/cfa/_registry/release_baseline/scope_ledger.json")
    args = ap.parse_args()

    queue = load_queue(args.queue)
    manifest_ids = load_manifest_ids(args.manifest)
    ledger, failures = build_ledger(queue, manifest_ids)

    # Count reconciliation (Invariant 4). The active corpus on disk/in the
    # manifest is the SUM of both emitted buckets (active_emitted +
    # active_deferred_then_emitted_this_loop), not active_emitted alone.
    disk_slugs = find_disk_cards(args.cards_dir)
    disk_count = len(disk_slugs)
    sidecar_count = count_history_sidecars(args.cards_dir)
    manifest_count = len(manifest_ids)
    active_corpus_count = ledger["active_corpus_total"]

    reconciliation = {
        "disk_md": disk_count,
        "manifest": manifest_count,
        "history_sidecars": sidecar_count,
        "ledger_active_corpus": active_corpus_count,
        "ledger_active_emitted": ledger["counts"]["active_emitted"],
        "ledger_active_deferred_then_emitted_this_loop": ledger["counts"][
            "active_deferred_then_emitted_this_loop"
        ],
        "consistent": disk_count == manifest_count == sidecar_count == active_corpus_count,
    }
    ledger["reconciliation"] = reconciliation

    if not reconciliation["consistent"]:
        failures.append(
            "count reconciliation mismatch: "
            f"disk_md={disk_count}, manifest={manifest_count}, "
            f"sidecars={sidecar_count}, ledger_active_corpus={active_corpus_count}"
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
        f"active_deferred_then_emitted_this_loop={ledger['counts']['active_deferred_then_emitted_this_loop']}, "
        f"active_deferred_this_loop={ledger['counts']['active_deferred_this_loop']}, "
        f"quarantined={ledger['counts']['quarantined']}, "
        f"excluded={ledger['counts']['excluded']}, "
        f"active_corpus={ledger['active_corpus_total']}, "
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
