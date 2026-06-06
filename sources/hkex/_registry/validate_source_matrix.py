#!/usr/bin/env python3
r"""Validate the hand-authored `out/hkex/source_matrix.json` (AC-5).

A fail-closed authorization gate that RE-DERIVES its expectations from canonical sources
rather than trusting the matrix (cf. BL-20260530-re-derive-stored-evidence-not-count,
BL-20260530-doc-gate-cross-checks-disk-not-self):

  * every matrix `reading_id` is a known CFA-taxonomy reading (the `cards/cfa` folders);
  * every matrix `source_id` exists in the merged `out/hkex/sources_manifest.json`;
  * the Xueqiu source is authorized for EXACTLY the authorable practitioner readings
    re-derived from `card_inventory.json` + `verifications.json` (verdict ∈
    {faithful, overgeneralized}) — never a blanket all-readings grant, never a missing
    reading;
  * (when `cards/hkex` holds cards) every active card citation's `(reading_id, source_id)`
    is authorized AND every matrix authorization is used by ≥1 active card (no unused/
    overbroad authorization).

SCOPE NOTE (Codex T12 review): this is READING-LEVEL authorization only — exactly what the
kernel `kb lint`/`kb verify --source-matrix` consume. It deliberately does NOT enforce the
research §5.2 named exclusions (e.g. the Tianyi-Cloud misattribution) or per-quote
★AUTHOR(狗不叫) attribution: a reading is authorized for the Xueqiu source when it holds ≥1
AUTHORABLE candidate, so an EXCLUDED candidate in that same reading could still be authored
and pass reading-level auth (`kb verify` proves the quote exists in the rendered PDF, not who
said it). Card-level exclusion + attribution enforcement is the AC-8 deck-local faithfulness
QA linter (verdict / `//@`-and-parenthetical rejection / excluded-set absence). The card-aware
checks here additionally require, once cards exist, that every authorization is USED (no
over-broad grant) and every citation is authorized; `kb lint` is the independent authority on
the latter.

  (default)     validate the real matrix; exit 0 iff clean
  --self-test   drive the validation logic hermetically (each negative must fail)
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[3]
MATRIX = ROOT / "out/hkex/source_matrix.json"
SOURCES_MANIFEST = ROOT / "out/hkex/sources_manifest.json"
INVENTORY = ROOT / "_research/xueqiu_goubujiao/card_inventory.json"
VERIFICATIONS = ROOT / "_research/xueqiu_goubujiao/verifications.json"
CARDS_DIR = ROOT / "cards/hkex"
CFA_CARDS = ROOT / "cards/cfa"
XUEQIU = "goubujiao_xueqiu_corpus"
AUTHORABLE_VERDICTS = {"faithful", "overgeneralized"}



def known_readings() -> set[str]:
    """The CFA reading taxonomy the hkex deck reuses = the cfa card folders."""
    return {p.name for p in CFA_CARDS.iterdir() if p.is_dir() and re.match(r"\d", p.name)}


def manifest_source_ids(path: Path) -> set[str]:
    if not path.is_file():
        raise SystemExit(f"merged sources_manifest missing: {path} (run the runner + merge first)")
    return {str(s["source_id"]) for s in json.loads(path.read_text(encoding="utf-8")).get("sources", [])}


def authorable_readings(inventory: list[dict], verifications: list[dict]) -> set[str]:
    verdict_by_title = {}
    for v in verifications:
        vv = v.get("verification")
        verdict_by_title[v.get("card_title_en")] = vv.get("verdict") if isinstance(vv, dict) else vv
    out = set()
    for c in inventory:
        if verdict_by_title.get(c.get("card_title_en")) in AUTHORABLE_VERDICTS:
            out.add(c.get("reading_folder"))
    return out


def load_cards(cards_dir: Path) -> list[tuple[str, list[str]]]:
    """Active hkex cards -> (reading_id, [cited source_ids]), parsed with a real YAML
    frontmatter loader (not a regex) so a valid YAML variant cannot hide a citation."""
    out = []
    if not cards_dir.is_dir():
        return out
    for p in sorted(cards_dir.rglob("*.md")):
        if p.name == "INDEX.md" or p.name.startswith("_") or ".history" in p.name:
            continue
        text = p.read_text(encoding="utf-8")
        if not text.startswith("---"):
            continue
        end = text.find("\n---", 3)
        if end < 0:
            continue
        try:
            fm = yaml.safe_load(text[3:end])
        except yaml.YAMLError as e:
            raise SystemExit(f"{p}: unparseable frontmatter: {e}")
        if not isinstance(fm, dict) or not fm.get("reading_id"):
            continue
        srcs = [c["source_id"] for c in (fm.get("citations") or [])
                if isinstance(c, dict) and c.get("source_id")]
        out.append((str(fm["reading_id"]), srcs))
    return out


def validate(matrix: dict, src_ids: set[str], auth_readings: set[str],
             known: set[str], cards: list[tuple[str, list[str]]]) -> list[str]:
    errors: list[str] = []
    if matrix.get("schema_version") != "cacg.v0":
        errors.append("schema_version != cacg.v0")
    allowed = matrix.get("allowed")
    if not isinstance(allowed, dict):
        return errors + ["'allowed' is not an object"]

    for reading, srcs in allowed.items():
        if reading not in known:
            errors.append(f"unknown reading_id: {reading}")
        if not isinstance(srcs, list) or not srcs:
            errors.append(f"reading {reading}: source list must be non-empty")
            continue
        if len(set(srcs)) != len(srcs):
            errors.append(f"reading {reading}: duplicate source_id")
        for s in srcs:
            if s not in src_ids:
                errors.append(f"reading {reading}: source_id {s!r} not in merged sources_manifest")

    xueqiu_readings = {r for r, srcs in allowed.items() if isinstance(srcs, list) and XUEQIU in srcs}
    extra = sorted(xueqiu_readings - auth_readings)
    missing = sorted(auth_readings - xueqiu_readings)
    if extra:
        errors.append(f"Xueqiu authorized for non-authorable / blanket reading(s): {extra}")
    if missing:
        errors.append(f"Xueqiu authorization missing for authorable reading(s): {missing}")

    # card-aware checks (only meaningful once cards exist)
    if cards:
        cited_pairs = set()
        for rid, csrcs in cards:
            for cs in csrcs:
                cited_pairs.add((rid, cs))
                if cs not in allowed.get(rid, []):
                    errors.append(f"card citation not authorized: reading {rid} source {cs}")
        for reading, srcs in allowed.items():
            if not isinstance(srcs, list):
                continue
            for s in srcs:
                if (reading, s) in cited_pairs:
                    continue
                # The Xueqiu source is justified by the inventory (practitioner cards may still be
                # pending — e.g. during AC-6 grounding authoring); a GROUNDING source authorization
                # must be cited by an authored card.
                if s == XUEQIU and reading in auth_readings:
                    continue
                errors.append(f"unused matrix authorization (no card cites it): {reading} -> {s}")
    return errors


def self_test() -> int:
    failures: list[str] = []
    known = {"02_economics", "05_equity", "07_derivatives_and_volatility", "14_microstructure_and_trading"}
    src_ids = {XUEQIU, "hk_stamp_duty"}
    auth = {"02_economics", "07_derivatives_and_volatility"}

    def ok(matrix, cards=()):
        return validate(matrix, src_ids, auth, known, list(cards))

    base = {"schema_version": "cacg.v0", "allowed": {
        "02_economics": [XUEQIU], "07_derivatives_and_volatility": [XUEQIU]}}
    if ok(base):
        failures.append(f"clean matrix flagged: {ok(base)}")
    # unknown reading
    if not ok({"schema_version": "cacg.v0", "allowed": {"99_unknown": [XUEQIU], "02_economics": [XUEQIU],
                                                        "07_derivatives_and_volatility": [XUEQIU]}}):
        failures.append("unknown reading not rejected")
    # source not in manifest
    if not ok({"schema_version": "cacg.v0", "allowed": {"02_economics": ["ghost_src"],
                                                        "07_derivatives_and_volatility": [XUEQIU]}}):
        failures.append("matrix source absent from manifest not rejected")
    # blanket/extra Xueqiu reading
    if not ok({"schema_version": "cacg.v0", "allowed": {"02_economics": [XUEQIU], "05_equity": [XUEQIU],
                                                        "07_derivatives_and_volatility": [XUEQIU]}}):
        failures.append("blanket/extra Xueqiu reading not rejected")
    # missing authorable reading
    if not ok({"schema_version": "cacg.v0", "allowed": {"02_economics": [XUEQIU]}}):
        failures.append("missing authorable reading not rejected")
    # card-aware: unauthorized citation
    if not ok(base, cards=[("05_equity", [XUEQIU])]):
        failures.append("unauthorized card citation not rejected")
    # card-aware: an UNUSED *grounding* authorization is rejected (07 authorizes hk_stamp_duty
    # but no card cites it). A grounding grant must be justified by a card, not the inventory.
    g = {"schema_version": "cacg.v0", "allowed": {
        "02_economics": [XUEQIU], "07_derivatives_and_volatility": [XUEQIU, "hk_stamp_duty"]}}
    if not ok(g, cards=[("07_derivatives_and_volatility", [XUEQIU])]):
        failures.append("unused grounding authorization not rejected")
    # card-aware: every grounding authorization cited -> passes (02->XUEQIU here is uncited but
    # inventory-justified, so it must NOT be flagged).
    if ok(g, cards=[("07_derivatives_and_volatility", [XUEQIU, "hk_stamp_duty"])]):
        failures.append(f"fully-covered grounding matrix flagged: {ok(g, cards=[('07_derivatives_and_volatility',[XUEQIU,'hk_stamp_duty'])])}")
    # card-aware: a Xueqiu authorization with no card is OK during grounding rounds (the
    # practitioner cards are pending) -> inventory-justified, never flagged unused.
    if ok(base, cards=[("02_economics", [XUEQIU])]):
        failures.append(f"inventory-justified Xueqiu authorization wrongly flagged: {ok(base, cards=[('02_economics',[XUEQIU])])}")

    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print("  -", f)
        return 1
    print("SELF-TEST PASSED (validate_source_matrix: unknown/absent-source/blanket/missing/"
          "unauthorized-citation/unused all fail; clean + fully-covered pass)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        return self_test()
    if not MATRIX.is_file():
        print(f"FAIL: {MATRIX} missing", file=sys.stderr)
        return 1
    matrix = json.loads(MATRIX.read_text(encoding="utf-8"))
    src_ids = manifest_source_ids(SOURCES_MANIFEST)
    auth = authorable_readings(json.loads(INVENTORY.read_text(encoding="utf-8"))["candidates"],
                               json.loads(VERIFICATIONS.read_text(encoding="utf-8")))
    cards = load_cards(CARDS_DIR)
    errors = validate(matrix, src_ids, auth, known_readings(), cards)
    result = {
        "matrix": str(MATRIX.relative_to(ROOT)),
        "matrix_source_ids": sorted({s for srcs in matrix.get("allowed", {}).values() for s in srcs}),
        "manifest_source_ids": sorted(src_ids),
        "xueqiu_authorized_readings": sorted(r for r, srcs in matrix.get("allowed", {}).items() if XUEQIU in srcs),
        "authorable_readings_from_inventory": sorted(auth),
        "active_hkex_cards": len(cards),
        "errors": errors,
        "verdict": "PASS" if not errors else "FAIL",
    }
    print(json.dumps(result, ensure_ascii=False, indent=2, sort_keys=True))
    return 0 if not errors else 1


if __name__ == "__main__":
    raise SystemExit(main())
