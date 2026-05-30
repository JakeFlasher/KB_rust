#!/usr/bin/env python3
"""v1 existing-corpus invariance gate (re-runnable, fail-closed).

After the v1 `kb index`, the 268 pre-existing (v0) `card_hash` values must be UNCHANGED
except for the recorded cross-link allowlist of released cards; the 6 quarantined ids
must stay out of the active manifest; and the active total must be 402.

The v0 baseline is RE-DERIVED from the committed `v0-candidate` tag's
`out/cfa_legacy/cards_manifest.json` (268 cards); the current state is read from the
working-tree manifest (402). The allowlist of released cards permitted to change is
DERIVED from the committed `migration_cross_links.json` (not hard-coded). Quarantine ids
come from `check_quarantine_invariant.py`'s frozen pin.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
MANIFEST = "out/cfa_legacy/cards_manifest.json"
V0_TAG = "v0-candidate"
CROSS_LINKS = REPO / "sources/cfa_legacy/_registry/migration_cross_links.json"
EXPECTED_ACTIVE = 402
EXPECTED_BASELINE = 268


def git_show(ref: str, path: str) -> str:
    return subprocess.run(["git", "-C", str(REPO), "show", f"{ref}:{path}"],
                          capture_output=True, text=True).stdout


def hashes(text: str) -> dict[str, str]:
    return {c["id"]: c["card_hash"] for c in json.loads(text)["cards"]}


def allowlist() -> set[str]:
    doc = json.loads(CROSS_LINKS.read_text(encoding="utf-8"))
    return set(doc.get("released_card_allowlist", {}))


def quarantine_ids() -> set[str]:
    """The 6 frozen quarantine ids, taken from the quarantine invariant checker's pin."""
    from check_quarantine_invariant import QUARANTINE_IDS
    return set(QUARANTINE_IDS)


def run_checks() -> tuple[list[str], dict]:
    failures: list[str] = []
    base = hashes(git_show(V0_TAG, MANIFEST))
    cur = hashes((REPO / MANIFEST).read_text(encoding="utf-8"))
    allow = allowlist()
    quar = quarantine_ids()

    if len(base) != EXPECTED_BASELINE:
        failures.append(f"{V0_TAG} baseline has {len(base)} cards, expected {EXPECTED_BASELINE}")
    if len(cur) != EXPECTED_ACTIVE:
        failures.append(f"current manifest has {len(cur)} cards, expected {EXPECTED_ACTIVE}")

    changed = sorted(i for i in base if i in cur and base[i] != cur[i])
    missing = sorted(i for i in base if i not in cur)
    if set(changed) != allow:
        failures.append(f"changed pre-existing card_hashes {changed} != released-card allowlist {sorted(allow)}")
    if missing:
        failures.append(f"pre-existing cards dropped from the manifest: {missing}")

    # the 6 quarantined ids must not be active (never in the manifest).
    in_manifest = sorted(q for q in quar if q in cur)
    if in_manifest:
        failures.append(f"quarantined ids present in the active manifest: {in_manifest}")
    if quar and len(quar) != 6:
        failures.append(f"expected 6 quarantined ids, derived {len(quar)}: {sorted(quar)}")

    report = {
        "baseline_tag": V0_TAG, "baseline_cards": len(base), "current_cards": len(cur),
        "changed_preexisting_hashes": changed, "released_card_allowlist": sorted(allow),
        "dropped_preexisting": missing, "quarantined_ids": sorted(quar),
        "quarantined_in_active_manifest": in_manifest, "passed": not failures,
    }
    return failures, report


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--json", action="store_true", help="print the machine-readable report")
    args = ap.parse_args()
    failures, report = run_checks()
    if args.json:
        print(json.dumps(report, indent=2))
    else:
        print(f"baseline({V0_TAG})={report['baseline_cards']} current={report['current_cards']} "
              f"changed={report['changed_preexisting_hashes']} allowlist={report['released_card_allowlist']} "
              f"quarantined={report['quarantined_ids']}")
    if failures:
        print("\nV1 CARD-HASH INVARIANCE: FAIL", file=sys.stderr)
        for f in failures:
            print(f"  - {f}", file=sys.stderr)
        return 1
    print("\nV1 CARD-HASH INVARIANCE: PASS (only the released-card allowlist changed among the 268; "
          "6 quarantined ids absent from the active manifest; 402 active)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
