#!/usr/bin/env python3
"""AC-9 gate: the published index is byte-reproducible under the frozen clock.

Re-runnable, fail-closed. Proves it by re-running the production Rust `kb index` binary.
All work happens under the gitignored `out/cfa_legacy/_repro/`; the REAL corpus is never
touched — `kb index <cards_dir>` can mutate that dir (it appends a `.history.jsonl`
event when a card's content changes), so we run against throwaway copies.

The plan's AC-9 names the THREE PUBLISHED artifacts (`cards_manifest.json`,
`summaries.json`, `INDEX.md`) and asks for a byte-identity check plus a non-frozen
negative. This harness operates strictly on those three artifacts and reports the
honest result:

  GATE 1 — POSITIVE (determinism): with a FIXED input dir,
    `KB_FROZEN_CLOCK=1 kb index <cards> --out A` and `--out B` produce byte-identical
    cards_manifest.json / summaries.json / INDEX.md.

  GATE 2 — CLOCK-INVARIANCE (the reconciled negative): a NON-frozen run over the SAME
    input dir (`--out N`, no KB_FROZEN_CLOCK) is ALSO byte-identical to the frozen run
    for all three published artifacts. The production index intentionally carries no
    wall-clock/UUID in these artifacts, so they are clock-invariant — STRONGER than the
    plan's assumption that a non-frozen run would differ. This gate also catches a
    future regression: if a wall-clock/UUID ever leaked into a published artifact,
    frozen != non-frozen would FAIL here.
    (Honest reconciliation: we do NOT claim the published index is "non-reproducible"
    without the frozen clock — it is reproducible. The clock's observable effect is the
    audit sidecar, see GATE 3.)

  GATE 3 — SUPPLEMENTARY, the clock-sensitive surface (NOT the published-index
    negative): `KB_FROZEN_CLOCK` IS load-bearing on the per-card `.history.jsonl` audit
    trail. Modify one card identically in two copies, index one frozen + one
    non-frozen; the appended history event timestamp is the frozen sentinel
    `1970-01-01T00:00:00Z` vs a real wall clock. This proves the frozen clock is not a
    no-op and that the audit trail — not the published artifacts — is where the clock
    lands.

Cross-copy note (corrected): two SEPARATE physical copies of the cards at different
paths produce different manifest/summaries bytes even though `kb index` SORTS paths and
emits cards in `(reading_id, id)` order (same card set, same order). The difference is
the serialized per-card `path` field, which is staging-relative (it records the
as-invoked path), NOT filesystem walk order. AC-9 is therefore tested as
same-input-dir / two-output-dirs, which isolates the comparison from the path prefix.

Writes a DETERMINISTIC proof to
`sources/cfa_legacy/_registry/release_baseline/index_repro.json` (frozen hashes + the epoch
sentinel + boolean assertions only — never the random wall-clock value, so the
committed proof is itself byte-stable across runs).
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
PROOF = REPO / "sources/cfa_legacy/_registry/release_baseline/index_repro.json"
ARTIFACTS = ("cards_manifest.json", "summaries.json", "INDEX.md")
FROZEN_SENTINEL = "1970-01-01T00:00:00Z"
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

    # --- GATES 1 & 2: one fixed input dir; frozen A, frozen B, non-frozen N. ---
    pub_cards = STAGE / "pub_cards"
    shutil.copytree(CARDS, pub_cards)
    out_a, out_b, out_n = STAGE / "out_a", STAGE / "out_b", STAGE / "out_n"
    run_index(pub_cards, out_a, frozen=True)
    run_index(pub_cards, out_b, frozen=True)
    run_index(pub_cards, out_n, frozen=False)
    ha, hb, hn = artifact_hashes(out_a), artifact_hashes(out_b), artifact_hashes(out_n)

    # GATE 1 (positive determinism): two frozen runs byte-identical.
    for name in ARTIFACTS:
        if ha[name] != hb[name]:
            failures.append(f"[determinism] frozen runs differ on {name}: {ha[name][:12]} != {hb[name][:12]}")
    # GATE 2 (clock-invariance): frozen == non-frozen for all three published artifacts.
    for name in ARTIFACTS:
        if ha[name] != hn[name]:
            failures.append(
                f"[clock-invariance] frozen vs non-frozen differ on published {name}: "
                f"{ha[name][:12]} != {hn[name][:12]} (a wall-clock/UUID may have leaked into the published index)"
            )
    frozen_byte_identical = all(ha[n] == hb[n] for n in ARTIFACTS)
    clock_invariant = all(ha[n] == hn[n] for n in ARTIFACTS)

    # --- GATE 3 (supplementary): the frozen clock is load-bearing on the audit sidecar. ---
    neg_f, neg_n = STAGE / "side_frozen", STAGE / "side_nonfrozen"
    shutil.copytree(CARDS, neg_f)
    shutil.copytree(CARDS, neg_n)
    marker = "\n<!-- ac9-clock-probe -->\n"
    for root in (neg_f, neg_n):
        card = root / PROBE_CARD
        card.write_text(card.read_text(encoding="utf-8") + marker, encoding="utf-8")
    run_index(neg_f, STAGE / "out_side_f", frozen=True)
    run_index(neg_n, STAGE / "out_side_n", frozen=False)
    ts_frozen = last_event_timestamp(neg_f)
    ts_nonfrozen = last_event_timestamp(neg_n)
    if ts_frozen != FROZEN_SENTINEL:
        failures.append(f"[sidecar] frozen appended-event timestamp is {ts_frozen!r}, expected {FROZEN_SENTINEL!r}")
    if ts_nonfrozen == FROZEN_SENTINEL:
        failures.append("[sidecar] non-frozen appended-event timestamp is the frozen sentinel "
                        "— KB_FROZEN_CLOCK appears to be a no-op")
    sidecar_clock_load_bearing = (ts_frozen == FROZEN_SENTINEL and ts_nonfrozen != FROZEN_SENTINEL)

    proof = {
        "schema_version": "cfa_legacy_index_repro/v2",
        "kb_binary": str(KB.relative_to(REPO)),
        "cards_dir": str(CARDS.relative_to(REPO)),
        "artifacts": list(ARTIFACTS),
        "method": (
            "GATE 1 determinism: KB_FROZEN_CLOCK=1 kb index <one cards copy> into two --out "
            "dirs, byte-compare the 3 published artifacts. GATE 2 clock-invariance: a "
            "non-frozen run over the SAME input is byte-compared to the frozen run. GATE 3 "
            "supplementary: modify a card, index frozen vs non-frozen, compare the appended "
            "history-event timestamp."
        ),
        "frozen_run_a_hashes": ha,
        "frozen_run_b_hashes": hb,
        "nonfrozen_run_hashes": hn,
        "frozen_byte_identical": frozen_byte_identical,
        "published_artifacts_clock_invariant": clock_invariant,
        "published_artifacts_carry_no_wall_clock": clock_invariant,
        "clock_sensitive_surface": "history sidecar (audit events), not the 3 published artifacts",
        "sidecar_clock_load_bearing": sidecar_clock_load_bearing,
        "frozen_appended_event_timestamp": ts_frozen,  # deterministic: the epoch sentinel
        "cross_copy_difference_cause": (
            "the serialized per-card `path` field (staging-relative; records the as-invoked "
            "path). kb index sorts paths and emits cards in (reading_id, id) order, so two "
            "physical copies have the same card set + order and differ only by the path "
            "prefix — NOT filesystem walk order."
        ),
        "plan_negative_reconciliation": (
            "Plan AC-9's negative assumes a non-frozen index injects a wall-clock timestamp "
            "into the published artifacts. The production kb index does not: the 3 published "
            "artifacts are clock-invariant (GATE 2), a stronger property. The frozen clock's "
            "observable effect is the audit sidecar (GATE 3). This harness therefore asserts "
            "clock-invariance of the published index and load-bearingness on the sidecar, "
            "instead of a published-artifact difference that cannot occur."
        ),
        "passed": not failures,
    }
    PROOF.parent.mkdir(parents=True, exist_ok=True)
    PROOF.write_text(json.dumps(proof, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    shutil.rmtree(STAGE)

    print("GATE 1 frozen A: " + ", ".join(f"{n}={ha[n][:12]}" for n in ARTIFACTS))
    print("GATE 1 frozen B: " + ", ".join(f"{n}={hb[n][:12]}" for n in ARTIFACTS))
    print(f"GATE 1 frozen byte-identical (3 published artifacts): {frozen_byte_identical}")
    print("GATE 2 non-frozen: " + ", ".join(f"{n}={hn[n][:12]}" for n in ARTIFACTS))
    print(f"GATE 2 published artifacts clock-invariant (frozen==non-frozen): {clock_invariant}")
    print(f"GATE 3 sidecar clock load-bearing (frozen={ts_frozen} vs non-frozen=<wall-clock>): {sidecar_clock_load_bearing}")
    print(f"proof written: {PROOF.relative_to(REPO)}")

    if failures:
        print("\nINDEX REPRODUCIBILITY: FAIL", file=sys.stderr)
        for f in failures:
            print(f"  - {f}", file=sys.stderr)
        return 1
    print("\nINDEX REPRODUCIBILITY: PASS (two frozen kb index runs over the same input are "
          "byte-identical for cards_manifest.json + summaries.json + INDEX.md; a non-frozen "
          "run is byte-identical too — the published artifacts are clock-invariant; the "
          "frozen clock is load-bearing on the audit sidecar)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
