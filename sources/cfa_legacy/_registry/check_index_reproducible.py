#!/usr/bin/env python3
"""AC-9 gate: the published index is byte-reproducible under the frozen clock.

Re-runnable, fail-closed. Proves, by actually re-running the production Rust `kb index`
binary, that the three published release artifacts are byte-identical across two
`KB_FROZEN_CLOCK=1` runs, and demonstrates the frozen-clock mechanism is load-bearing.

Method (all work under the gitignored `out/cfa_legacy/_repro/`; the REAL corpus is
never touched — `kb index` appends a `.history.jsonl` event to the cards dir it is run
against, so we run against throwaway copies):

  1. Copy `cards/cfa_legacy` into four independent staging card-trees: two frozen
     (fa, fb), two non-frozen (na, nb).
  2. Run `KB_FROZEN_CLOCK=1 kb index <fa> --out <out_fa>` and `… <fb> --out <out_fb>`;
     run `kb index <na> --out <out_na>` and `… <nb> --out <out_nb>` WITHOUT the frozen
     clock.
  3. POSITIVE (hard gate): SHA-256 of each published artifact
     (`cards_manifest.json`, `summaries.json`, `INDEX.md`) is identical between the two
     frozen runs.
  4. NEGATIVE (hard gate): the frozen clock is load-bearing — the per-card history
     sidecar (`.history.jsonl`), whose appended event timestamp/uuid is controlled by
     `DeterminismContext`, is IDENTICAL between the two frozen runs but DIFFERS between
     the two non-frozen runs (real wall-clock + random uuid each run). This proves a
     non-frozen index is not reproducible.
  5. Records the proof (hashes + the frozen/non-frozen sidecar event fields) to
     `out/cfa_legacy/v0_baseline/index_repro.json`.

Note (honest framing): the three PUBLISHED release artifacts carry no timestamp/uuid,
so they are byte-identical under BOTH frozen and non-frozen runs — a strong inherent
reproducibility property. The wall-clock therefore lands only in the history sidecars
(which are part of the trust chain but not the 3 published artifacts), and that is
where the frozen-clock negative case is observable. The harness asserts both: published
artifacts reproducible (positive) AND frozen-clock-controls-history (negative).
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
# Proof lives with the other re-runnable gate baselines (tracked), not under the
# gitignored out/ tree.
PROOF = REPO / "sources/cfa_legacy/_registry/v0_baseline/index_repro.json"
ARTIFACTS = ("cards_manifest.json", "summaries.json", "INDEX.md")
READING_PROBE = "10_behavioral_finance"  # small slice; pick one card's sidecar to inspect


def sha256_file(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def run_index(cards_dir: Path, out_dir: Path, *, frozen: bool) -> None:
    env = dict(os.environ)
    if frozen:
        env["KB_FROZEN_CLOCK"] = "1"
    else:
        env.pop("KB_FROZEN_CLOCK", None)
    r = subprocess.run(
        [str(KB), "index", str(cards_dir), "--out", str(out_dir)],
        env=env, capture_output=True, text=True,
    )
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


def last_sidecar_event(cards_root: Path) -> dict:
    """Return the last history event for a deterministic probe card."""
    slice_dir = cards_root / READING_PROBE
    sidecars = sorted(slice_dir.glob("*.history.jsonl"))
    if not sidecars:
        raise SystemExit(f"no history sidecar found under {slice_dir}")
    lines = [x for x in sidecars[0].read_text(encoding="utf-8").splitlines() if x.strip()]
    if not lines:
        raise SystemExit(f"empty history sidecar {sidecars[0]}")
    ev = json.loads(lines[-1])
    return {"card": sidecars[0].name, "ts": ev.get("ts"), "timestamp": ev.get("timestamp"),
            "uuid": ev.get("uuid"), "event_id": ev.get("event_id"), "n_events": len(lines)}


def main() -> int:
    if not KB.is_file():
        raise SystemExit(f"kb binary not found: {KB} (build with `cargo build --workspace`)")
    if not CARDS.is_dir():
        raise SystemExit(f"cards dir not found: {CARDS}")

    if STAGE.exists():
        shutil.rmtree(STAGE)
    STAGE.mkdir(parents=True)

    failures: list[str] = []

    # Four independent staging card-trees from the same source.
    trees = {}
    for name in ("fa", "fb", "na", "nb"):
        dst = STAGE / ("cards_" + name)
        shutil.copytree(CARDS, dst)
        trees[name] = dst
    outs = {name: STAGE / ("out_" + name) for name in trees}

    # Run: two frozen, two non-frozen.
    run_index(trees["fa"], outs["fa"], frozen=True)
    run_index(trees["fb"], outs["fb"], frozen=True)
    run_index(trees["na"], outs["na"], frozen=False)
    run_index(trees["nb"], outs["nb"], frozen=False)

    hfa, hfb = artifact_hashes(outs["fa"]), artifact_hashes(outs["fb"])
    hna, hnb = artifact_hashes(outs["na"]), artifact_hashes(outs["nb"])

    # POSITIVE (hard): two frozen runs are byte-identical for all three artifacts.
    for name in ARTIFACTS:
        if hfa[name] != hfb[name]:
            failures.append(f"frozen runs differ on {name}: {hfa[name][:12]} != {hfb[name][:12]}")

    # Informational: the published artifacts are time-invariant (frozen == non-frozen).
    published_time_invariant = all(hfa[n] == hna[n] == hnb[n] for n in ARTIFACTS)

    # NEGATIVE (hard): the frozen clock controls the history sidecar — frozen pair
    # identical, non-frozen pair differs (proving a non-frozen index is not reproducible).
    ev_fa, ev_fb = last_sidecar_event(trees["fa"]), last_sidecar_event(trees["fb"])
    ev_na, ev_nb = last_sidecar_event(trees["na"]), last_sidecar_event(trees["nb"])

    def ev_key(e):
        return (e["ts"], e["timestamp"], e["uuid"], e["event_id"])

    if ev_key(ev_fa) != ev_key(ev_fb):
        failures.append(f"frozen sidecar events differ (should be identical): {ev_fa} != {ev_fb}")
    frozen_is_epoch = ev_fa["ts"] == "1970-01-01T00:00:00Z" and \
        ev_fa["uuid"] == "00000000-0000-0000-0000-000000000000"
    if not frozen_is_epoch:
        failures.append(f"frozen sidecar event is not the epoch/zero-uuid sentinel: {ev_fa}")
    nonfrozen_differs = ev_key(ev_na) != ev_key(ev_nb)
    if not nonfrozen_differs:
        failures.append(
            "non-frozen runs produced IDENTICAL sidecar events — the frozen clock is not "
            f"load-bearing as expected (na={ev_na}, nb={ev_nb})"
        )

    proof = {
        "schema_version": "cfa_legacy_index_repro/v1",
        "kb_binary": str(KB.relative_to(REPO)),
        "cards_dir": str(CARDS.relative_to(REPO)),
        "artifacts": list(ARTIFACTS),
        "frozen_run_a_hashes": hfa,
        "frozen_run_b_hashes": hfb,
        "frozen_byte_identical": all(hfa[n] == hfb[n] for n in ARTIFACTS),
        "published_artifacts_time_invariant": published_time_invariant,
        "frozen_sidecar_event": ev_fa,
        "nonfrozen_sidecar_event_a": ev_na,
        "nonfrozen_sidecar_event_b": ev_nb,
        "nonfrozen_sidecar_nonreproducible": nonfrozen_differs,
        "passed": not failures,
    }
    PROOF.parent.mkdir(parents=True, exist_ok=True)
    PROOF.write_text(json.dumps(proof, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    # Clean the staging tree (gitignored, but keep the tree tidy).
    shutil.rmtree(STAGE)

    print(f"frozen run A hashes: " + ", ".join(f"{n}={hfa[n][:12]}" for n in ARTIFACTS))
    print(f"frozen run B hashes: " + ", ".join(f"{n}={hfb[n][:12]}" for n in ARTIFACTS))
    print(f"frozen byte-identical (all 3 published artifacts): {proof['frozen_byte_identical']}")
    print(f"published artifacts time-invariant (frozen==non-frozen): {published_time_invariant}")
    print(f"frozen sidecar event: ts={ev_fa['ts']} uuid={ev_fa['uuid']}")
    print(f"non-frozen sidecar events differ between runs (non-reproducible): {nonfrozen_differs}")
    print(f"  na: ts={ev_na['ts']} uuid={ev_na['uuid']}")
    print(f"  nb: ts={ev_nb['ts']} uuid={ev_nb['uuid']}")
    print(f"proof written: {PROOF.relative_to(REPO)}")

    if failures:
        print("\nINDEX REPRODUCIBILITY: FAIL", file=sys.stderr)
        for f in failures:
            print(f"  - {f}", file=sys.stderr)
        return 1
    print("\nINDEX REPRODUCIBILITY: PASS (two frozen kb index runs are byte-identical for "
          "cards_manifest.json + summaries.json + INDEX.md; the frozen clock controls the "
          "history sidecar — non-frozen runs are not reproducible)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
