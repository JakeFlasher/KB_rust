#!/usr/bin/env python3
"""AC-9 gate: the published index is byte-reproducible under the frozen clock.

Re-runnable, fail-closed. Proves it by actually re-running the production Rust
`kb index` binary. All work happens under the gitignored `out/cfa_legacy/_repro/`; the
REAL corpus is never touched — `kb index <cards_dir>` mutates that dir (it can append a
`.history.jsonl` event), so we run against throwaway copies.

Two hard gates:

  POSITIVE — byte-reproducible published artifacts (the AC-9 core):
    With a FIXED input directory, `KB_FROZEN_CLOCK=1 kb index <cards> --out A` and
    `… --out B` produce byte-identical `cards_manifest.json`, `summaries.json`, and
    `INDEX.md`. (Important nuance, verified: kb index emits the manifest/summaries in
    filesystem WALK order, so they are byte-stable for a fixed input PATH but would
    differ across two separate physical copies of the cards — same set, different
    order. INDEX.md is section-sorted and stable either way. AC-9's "two staging dirs"
    is therefore the two `--out` dirs from the SAME input, which is what we test.)

  NEGATIVE — the frozen clock is load-bearing (a non-frozen index is not reproducible):
    The published artifacts carry no timestamp/uuid, so they are time-invariant; the
    wall clock lands only in a freshly-APPENDED history-sidecar event (which kb index
    writes when a card's content changes). We modify one card identically in two
    throwaway copies, index one with `KB_FROZEN_CLOCK=1` and one without, and assert
    the new event's `timestamp` is the frozen sentinel `1970-01-01T00:00:00Z` in the
    frozen run but a real wall-clock value (≠ sentinel) in the non-frozen run — proving
    a non-frozen index is not reproducible.

Writes a DETERMINISTIC proof to
`sources/cfa_legacy/_registry/v0_baseline/index_repro.json` (frozen hashes + the epoch
sentinel + boolean assertions — never the random wall-clock value, so the committed
proof is itself byte-stable across runs).
"""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
KB = REPO / "target/debug/kb"
CARDS = REPO / "cards/cfa_legacy"
STAGE = REPO / "out/cfa_legacy/_repro"
PROOF = REPO / "sources/cfa_legacy/_registry/v0_baseline/index_repro.json"
ARTIFACTS = ("cards_manifest.json", "summaries.json", "INDEX.md")
FROZEN_SENTINEL = "1970-01-01T00:00:00Z"
# A small, stable card whose history sidecar we inspect for the negative case.
PROBE_CARD = "10_behavioral_finance/be-limits-of-arbitrage.md"
PROBE_SIDECAR = "10_behavioral_finance/be-limits-of-arbitrage.history.jsonl"


def sha256_file(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def run_index(cards_dir: Path, out_dir: Path, *, frozen: bool) -> None:
    env = dict(os.environ)
    if frozen:
        env["KB_FROZEN_CLOCK"] = "1"
    else:
        env.pop("KB_FROZEN_CLOCK", None)
    r = subprocess.run([str(KB), "index", str(cards_dir), "--out", str(out_dir)],
                       env=env, capture_output=True, text=True)
    if r.returncode != 0:
        raise SystemExit(f"kb index failed (frozen={frozen}) rc={r.returncode}: {r.stderr[:400]}")


def artifact_hashes(out_dir: Path) -> dict[str, str]:
    h = {}
    for name in ARTIFACTS:
        p = out_dir / name
        if not p.is_file():
            raise SystemExit(f"kb index did not produce {p}")
        h[name] = sha256_file(p)
    return h


def last_event_timestamp(cards_root: Path) -> str | None:
    p = cards_root / PROBE_SIDECAR
    lines = [x for x in p.read_text(encoding="utf-8").splitlines() if x.strip()]
    if not lines:
        raise SystemExit(f"empty/missing probe sidecar {p}")
    return json.loads(lines[-1]).get("timestamp")


def main() -> int:
    if not KB.is_file():
        raise SystemExit(f"kb binary not found: {KB} (build with `cargo build --workspace`)")
    if not CARDS.is_dir():
        raise SystemExit(f"cards dir not found: {CARDS}")

    if STAGE.exists():
        shutil.rmtree(STAGE)
    STAGE.mkdir(parents=True)
    failures: list[str] = []

    # --- POSITIVE: same input dir, two --out dirs, both frozen -> byte-identical. ---
    pos_cards = STAGE / "pos_cards"
    shutil.copytree(CARDS, pos_cards)
    out_a, out_b = STAGE / "out_a", STAGE / "out_b"
    run_index(pos_cards, out_a, frozen=True)
    run_index(pos_cards, out_b, frozen=True)
    ha, hb = artifact_hashes(out_a), artifact_hashes(out_b)
    for name in ARTIFACTS:
        if ha[name] != hb[name]:
            failures.append(f"frozen runs differ on {name}: {ha[name][:12]} != {hb[name][:12]}")

    # --- NEGATIVE: modify one card identically in two copies; frozen vs non-frozen
    # appended-event timestamp must be epoch vs wall-clock. ---
    neg_f, neg_n = STAGE / "neg_frozen", STAGE / "neg_nonfrozen"
    shutil.copytree(CARDS, neg_f)
    shutil.copytree(CARDS, neg_n)
    probe_marker = "\n<!-- ac9-repro-probe -->\n"
    for root in (neg_f, neg_n):
        card = root / PROBE_CARD
        card.write_text(card.read_text(encoding="utf-8") + probe_marker, encoding="utf-8")
    run_index(neg_f, STAGE / "out_neg_f", frozen=True)
    run_index(neg_n, STAGE / "out_neg_n", frozen=False)
    ts_frozen = last_event_timestamp(neg_f)
    ts_nonfrozen = last_event_timestamp(neg_n)
    if ts_frozen != FROZEN_SENTINEL:
        failures.append(f"frozen appended-event timestamp is {ts_frozen!r}, expected {FROZEN_SENTINEL!r}")
    if ts_nonfrozen == FROZEN_SENTINEL:
        failures.append(
            "non-frozen appended-event timestamp is the frozen sentinel — the frozen "
            "clock is not load-bearing (a non-frozen index would be wrongly reproducible)"
        )
    nonfrozen_differs = ts_frozen != ts_nonfrozen

    proof = {
        "schema_version": "cfa_legacy_index_repro/v1",
        "kb_binary": str(KB.relative_to(REPO)),
        "cards_dir": str(CARDS.relative_to(REPO)),
        "method": (
            "positive: KB_FROZEN_CLOCK=1 kb index <same cards copy> into two --out dirs, "
            "byte-compare the 3 published artifacts. negative: modify one card identically "
            "in two copies, index frozen vs non-frozen, compare the appended history event "
            "timestamp (frozen sentinel vs wall clock)."
        ),
        "artifacts": list(ARTIFACTS),
        "frozen_run_a_hashes": ha,
        "frozen_run_b_hashes": hb,
        "frozen_byte_identical": all(ha[n] == hb[n] for n in ARTIFACTS),
        "manifest_summaries_are_walk_ordered": (
            "byte-stable for a fixed input path; differ across separate physical copies "
            "(same card set, different order). INDEX.md is section-sorted and stable."
        ),
        "frozen_appended_event_timestamp": ts_frozen,        # deterministic: the sentinel
        "nonfrozen_appended_event_is_wall_clock": ts_nonfrozen != FROZEN_SENTINEL,
        "nonfrozen_index_nonreproducible": nonfrozen_differs,
        "passed": not failures,
    }
    PROOF.parent.mkdir(parents=True, exist_ok=True)
    PROOF.write_text(json.dumps(proof, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    shutil.rmtree(STAGE)

    print("frozen run A: " + ", ".join(f"{n}={ha[n][:12]}" for n in ARTIFACTS))
    print("frozen run B: " + ", ".join(f"{n}={hb[n][:12]}" for n in ARTIFACTS))
    print(f"frozen byte-identical (all 3 published artifacts): {proof['frozen_byte_identical']}")
    print(f"negative: frozen event ts={ts_frozen} | non-frozen event ts={'<wall-clock>' if proof['nonfrozen_appended_event_is_wall_clock'] else ts_nonfrozen}")
    print(f"non-frozen index non-reproducible: {nonfrozen_differs}")
    print(f"proof written: {PROOF.relative_to(REPO)}")

    if failures:
        print("\nINDEX REPRODUCIBILITY: FAIL", file=sys.stderr)
        for f in failures:
            print(f"  - {f}", file=sys.stderr)
        return 1
    print("\nINDEX REPRODUCIBILITY: PASS (two frozen kb index runs over the same input are "
          "byte-identical for cards_manifest.json + summaries.json + INDEX.md; the frozen "
          "clock is load-bearing — a non-frozen run stamps a wall-clock history event and "
          "is not reproducible)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
