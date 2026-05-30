#!/usr/bin/env python3
"""Deterministic negative-probe suite for the 09_PM emitter's trust checks (AC-6).

Re-runnable, no arguments. Proves the validators in `emit_09_pm_slice.py` are
fail-closed:

  * the single-source offset-map validator REJECTS a map whose evidence fails
    re-derivation (`legacy_page + offset != pdf_page`), an unparseable rule, <3
    confirmed triples, and non-integer evidence pages (AC-6 plan-line-68 negative
    test) — while ACCEPTING the real checked-in Pedersen/Cochrane maps;
  * `validate_citation` REJECTS a missing field, hash mismatch, page out of the
    chunk span, malformed page_range, retracted source/chunk, illegal control/format
    chars, and a non-substring quote — while ACCEPTING a real curated citation.

Exit 0 iff every probe behaves as expected; non-zero (with a FAIL line) otherwise.
The map-corruption probes copy the real maps into a temp dir and monkeypatch the
module's COORD_MAPS so the checked-in maps are never mutated.
"""

from __future__ import annotations

import copy
import json
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import emit_09_pm_slice as E  # noqa: E402

REPO = Path(__file__).resolve().parents[3]
OUT = REPO / "out/cfa_legacy"

_FAILURES: list[str] = []


def expect_pass(name: str, fn) -> None:
    try:
        fn()
        print(f"  ok    (accepts): {name}")
    except SystemExit as e:
        _FAILURES.append(f"{name}: expected ACCEPT but raised SystemExit: {e}")
        print(f"  FAIL  (rejected good input): {name} -> {e}")


def expect_reject(name: str, fn) -> None:
    try:
        fn()
        _FAILURES.append(f"{name}: expected REJECT but it PASSED")
        print(f"  FAIL  (accepted bad input): {name}")
    except SystemExit as e:
        print(f"  fires (rejects): {name} -> {str(e)[:72]}")


def main() -> int:
    chunks = {c["chunk_id"]: c for c in json.loads((OUT / "chunks_manifest.json").read_text())["chunks"]}
    allowed = set(json.loads((OUT / "source_matrix.json").read_text())["allowed"][E.READING_ID])
    reg = json.loads((E.MULTI_SOURCE_REGISTRY).read_text())
    good_cit = copy.deepcopy(reg["cards"][0]["citations"][0])  # a Pedersen `defines` citation
    ped = "pm_pedersen_2015_efficiently_inefficient"

    # ---- Offset-map validator: positive control on the real checked-in maps ----
    print("offset-map validator (real maps):")
    for src in sorted(E.SINGLE_SOURCE_MAP_SOURCES):
        expect_pass(f"real map {src}", lambda src=src: E.validate_single_source_map(src))

    # ---- Offset-map validator: negative probes on copied+corrupted maps ----
    print("offset-map validator (corrupted copies):")
    real_coord = E.COORD_MAPS
    with tempfile.TemporaryDirectory() as td:
        tmp = Path(td)
        base = json.loads((real_coord / f"{ped}.json").read_text())
        try:
            E.COORD_MAPS = tmp  # monkeypatch so the real maps are never touched

            def write(doc) -> None:
                (tmp / f"{ped}.json").write_text(json.dumps(doc), encoding="utf-8")

            # 1. one evidence pdf_page corrupted by +999 (Codex's exact probe)
            d = copy.deepcopy(base); d["sources"][0]["verified_evidence"][0]["pdf_page"] += 999; write(d)
            expect_reject("evidence pdf_page +999 (re-derivation mismatch)",
                          lambda: E.validate_single_source_map(ped))
            # 2. unparseable coordinate rule
            d = copy.deepcopy(base); d["sources"][0]["pdf_coordinate_rule"] = "pdf = legacy * 2"; write(d)
            expect_reject("unparseable pdf_coordinate_rule", lambda: E.validate_single_source_map(ped))
            # 3. fewer than 3 evidence triples
            d = copy.deepcopy(base); d["sources"][0]["verified_evidence"] = d["sources"][0]["verified_evidence"][:2]; write(d)
            expect_reject("only 2 verified_evidence triples", lambda: E.validate_single_source_map(ped))
            # 4. non-integer evidence page
            d = copy.deepcopy(base); d["sources"][0]["verified_evidence"][0]["pdf_page"] = "70"; write(d)
            expect_reject("non-integer evidence pdf_page", lambda: E.validate_single_source_map(ped))
            # 5. positive control under the patched dir (uncorrupted copy still passes)
            write(copy.deepcopy(base))
            expect_pass("uncorrupted copy still validates", lambda: E.validate_single_source_map(ped))
        finally:
            E.COORD_MAPS = real_coord

    # ---- validate_citation guards ----
    print("validate_citation guards:")
    vmaps = {s: E.validate_single_source_map(s) for s in sorted(E.SINGLE_SOURCE_MAP_SOURCES)}
    base_kw = dict(card_id="PROBE", allowed=allowed, chunks_by_id=chunks,
                   retracted_chunks=set(), retracted_sources=set(), validated_maps=vmaps)

    def check(cit, **over):
        kw = dict(base_kw); kw.update(over)
        E.validate_citation(cit, **kw)

    expect_pass("good citation", lambda: check(copy.deepcopy(good_cit)))
    c = copy.deepcopy(good_cit); del c["chunk_hash"]
    expect_reject("missing required field", lambda: check(c))
    c = copy.deepcopy(good_cit); c["chunk_hash"] = "0" * 64
    expect_reject("chunk_hash mismatch", lambda: check(c))
    c = copy.deepcopy(good_cit); c["page_range"] = [99999, 99999]
    expect_reject("page out of chunk span", lambda: check(c))
    c = copy.deepcopy(good_cit); c["page_range"] = "48-49"
    expect_reject("malformed page_range", lambda: check(c))
    c = copy.deepcopy(good_cit); c["quote"] = "definitely not a substring zzzqqq"
    expect_reject("quote not a substring", lambda: check(c))
    c = copy.deepcopy(good_cit)
    expect_reject("retracted source", lambda: check(c, retracted_sources={good_cit["source_id"]}))
    c = copy.deepcopy(good_cit)
    expect_reject("retracted chunk", lambda: check(c, retracted_chunks={good_cit["chunk_id"]}))

    print()
    if _FAILURES:
        print(f"NEGATIVE-PROBE SUITE: FAIL ({len(_FAILURES)} probe(s) misbehaved)", file=sys.stderr)
        for f in _FAILURES:
            print(f"  - {f}", file=sys.stderr)
        return 1
    print("NEGATIVE-PROBE SUITE: PASS (all guards fail-closed; real maps + citation accepted)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
