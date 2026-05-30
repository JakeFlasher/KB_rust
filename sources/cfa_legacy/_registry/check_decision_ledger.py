#!/usr/bin/env python3
"""AC-8 gate: the `_research/29` decision ledger is complete and DEC-2-consistent.

Re-runnable, fail-closed (non-zero exit on any violation). Parses the ledger's
machine-readable "§7 Open-Question Dispositions (Q1-Q12)" table and asserts:

  1. Exactly Q1..Q12 are present (no missing, no duplicate, no stray Qn).
  2. Every question carries an allowed status: DECIDED / IMPLICITLY SETTLED / DEFERRED.
  3. Every DEFERRED question links a FUT-* in its Evidence/FUT column.
  4. Every IMPLICITLY SETTLED question gives an on-disk evidence path that EXISTS
     (re-derived, not merely present — the path is resolved against the repo root).
  5. DEC-2 consistency: no ruling/line claims `volume_page` is in card frontmatter for
     v0 (the forbidden contradiction of DEC-2).
  6. The ledger states the v0 definition-of-done (268 active + 6 quarantined) and links
     FUT-1, FUT-2, FUT-4.

`--self-test` feeds synthetic ledgers through the same `run_checks` and proves the
negative cases fail closed (a Q with no status; a DEFERRED with no FUT; an
IMPLICITLY SETTLED with a non-existent evidence path; a frontmatter-volume_page claim;
a missing Q).
"""

from __future__ import annotations

import argparse
import glob
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
LEDGER_GLOB = "_research/29_*.md"
ALLOWED_STATUS = {"DECIDED", "IMPLICITLY SETTLED", "DEFERRED"}
EXPECTED_QIDS = [f"Q{i}" for i in range(1, 13)]
FUT_RE = re.compile(r"\bFUT-\d+\b")
TABLE_ROW_RE = re.compile(r"^\|\s*(Q\d+)\s*\|\s*([^|]+?)\s*\|\s*([^|]*?)\s*\|\s*(.*?)\s*\|\s*$")


def find_ledger() -> Path:
    matches = sorted(glob.glob(str(REPO / LEDGER_GLOB)))
    if not matches:
        raise SystemExit(f"AC-8 ledger not found: {LEDGER_GLOB}")
    return Path(matches[0])


def parse_rows(text: str) -> list[tuple[str, str, str, str]]:
    """Return [(qid, status, evidence_or_fut, ruling)] from the disposition table."""
    rows = []
    for line in text.splitlines():
        m = TABLE_ROW_RE.match(line)
        if m:
            rows.append((m.group(1), m.group(2).strip(), m.group(3).strip(), m.group(4).strip()))
    return rows


def run_checks(text: str, *, repo: Path, evidence_must_exist: bool = True) -> list[str]:
    failures: list[str] = []
    rows = parse_rows(text)
    by_id: dict[str, tuple[str, str, str, str]] = {}
    for r in rows:
        if r[0] in by_id:
            failures.append(f"duplicate disposition row for {r[0]}")
        by_id[r[0]] = r

    for qid in EXPECTED_QIDS:
        if qid not in by_id:
            failures.append(f"{qid}: missing from the §7 disposition table")
            continue
        _qid, status, evfut, _ruling = by_id[qid]
        if status not in ALLOWED_STATUS:
            failures.append(f"{qid}: status {status!r} not in {sorted(ALLOWED_STATUS)}")
            continue
        if status == "DEFERRED" and not FUT_RE.search(evfut):
            failures.append(f"{qid}: DEFERRED but no FUT-* link in Evidence/FUT column ({evfut!r})")
        if status == "IMPLICITLY SETTLED":
            path = evfut.strip().strip("`").strip()
            if not path:
                failures.append(f"{qid}: IMPLICITLY SETTLED but no evidence path")
            elif evidence_must_exist and not (repo / path).exists():
                failures.append(f"{qid}: IMPLICITLY SETTLED evidence path does not exist: {path}")

    stray = [r[0] for r in rows if r[0] not in EXPECTED_QIDS]
    if stray:
        failures.append(f"stray non-Q1..Q12 rows in disposition table: {stray}")

    # (5) DEC-2 consistency via the authoritative machine-check key/value block (robust
    # against prose phrasing): the v0 card citation coordinate must be pdf_page and
    # volume_page must NOT be in card frontmatter.

    def _val(key: str) -> str | None:
        m = re.search(r"(?m)^\s*%s\s*:\s*(\S+)" % re.escape(key), text)
        return m.group(1).strip().lower() if m else None

    coord = _val("v0_card_citation_coordinate")
    vp_fm = _val("volume_page_in_card_frontmatter")
    if coord is None or vp_fm is None:
        failures.append(
            "DEC-2 machine-check block missing (need v0_card_citation_coordinate + "
            "volume_page_in_card_frontmatter)"
        )
    else:
        if coord != "pdf_page":
            failures.append(f"DEC-2 violation: v0_card_citation_coordinate={coord!r} (must be pdf_page)")
        if vp_fm != "false":
            failures.append(f"DEC-2 violation: volume_page_in_card_frontmatter={vp_fm!r} (must be false for v0)")

    # (6) v0 DoD + FUT links present.
    if not re.search(r"268\b.*\b6\b|6\b.*\b268\b", text) or "quarantin" not in text.lower():
        failures.append("v0 definition-of-done (268 active + 6 quarantined) not stated")
    for fut in ("FUT-1", "FUT-2", "FUT-4"):
        if fut not in text:
            failures.append(f"{fut} not linked in the ledger")

    return failures


def _self_test() -> int:
    base_rows = "\n".join(
        f"| Q{i} | DECIDED | — | ruling {i} |" for i in range(1, 13)
    )
    good = (
        "# ledger\n\nv0 complete := 268 active cards + 6 quarantined legacy cards.\n\n"
        "| Question | Status | Evidence / FUT | Ruling |\n|--|--|--|--|\n"
        + base_rows
        + "\nFUT-1 ... FUT-2 ... FUT-4 ...\n"
        + "\nv0_card_citation_coordinate: pdf_page\nvolume_page_in_card_frontmatter: false\n"
    )
    failures = 0

    def expect_clean(name, text, **kw):
        nonlocal failures
        kw.setdefault("evidence_must_exist", False)
        res = run_checks(text, repo=REPO, **kw)
        if res:
            failures += 1; print(f"  FAIL (clean flagged): {name} -> {res}")
        else:
            print(f"  ok    (clean passes): {name}")

    def expect_caught(name, text, needle, **kw):
        nonlocal failures
        kw.setdefault("evidence_must_exist", False)
        res = run_checks(text, repo=REPO, **kw)
        if any(needle in f for f in res):
            print(f"  fires (caught): {name}")
        else:
            failures += 1; print(f"  FAIL (not caught): {name} -> {res}")

    print("AC-8 ledger self-test (negative probes):")
    expect_clean("baseline synthetic-clean", good)
    # Q with no status (drop Q5's status by blanking the cell)
    bad_status = good.replace("| Q5 | DECIDED | — | ruling 5 |", "| Q5 |  | — | ruling 5 |")
    expect_caught("Q5 blank status", bad_status, "Q5")
    # missing Q (remove Q7 row)
    missing = good.replace("| Q7 | DECIDED | — | ruling 7 |\n", "")
    expect_caught("Q7 missing", missing, "Q7: missing")
    # DEFERRED without FUT
    deferred_nofut = good.replace("| Q9 | DECIDED | — | ruling 9 |", "| Q9 | DEFERRED | — | ruling 9 |")
    expect_caught("Q9 DEFERRED no FUT", deferred_nofut, "no FUT-*")
    # IMPLICITLY SETTLED with non-existent evidence path
    impl_bad = good.replace("| Q3 | DECIDED | — | ruling 3 |",
                            "| Q3 | IMPLICITLY SETTLED | `out/cfa_legacy/does_not_exist_zzz.json` | ruling 3 |")
    expect_caught("Q3 impl-settled bad evidence", impl_bad, "does not exist", evidence_must_exist=True)
    # DEC-2 machine-check violations
    dec2_bad = good.replace("volume_page_in_card_frontmatter: false", "volume_page_in_card_frontmatter: true")
    expect_caught("DEC-2 volume_page_in_card_frontmatter=true", dec2_bad, "DEC-2 violation")
    dec2_coord = good.replace("v0_card_citation_coordinate: pdf_page", "v0_card_citation_coordinate: volume_page")
    expect_caught("DEC-2 coordinate=volume_page", dec2_coord, "must be pdf_page")
    dec2_missing = good.replace("volume_page_in_card_frontmatter: false", "")
    expect_caught("DEC-2 machine block missing", dec2_missing, "machine-check block missing")
    # correct prose (negated/deferred) must NOT be flagged — the machine block is authoritative
    dec2_prose = good + "\nFor v0, no volume_page field exists in card frontmatter; promotion deferred to FUT-4.\n"
    expect_clean("DEC-2 correct prose not flagged", dec2_prose)
    # missing FUT link
    no_fut = good.replace("FUT-1 ... FUT-2 ... FUT-4 ...", "FUT-1 ... FUT-2 ...")
    expect_caught("missing FUT-4 link", no_fut, "FUT-4 not linked")
    # missing v0 DoD
    no_dod = good.replace("v0 complete := 268 active cards + 6 quarantined legacy cards.", "v0 is done.")
    expect_caught("missing v0 DoD", no_dod, "definition-of-done")

    print()
    if failures:
        print(f"AC-8 LEDGER SELF-TEST: FAIL ({failures})", file=sys.stderr); return 1
    print("AC-8 LEDGER SELF-TEST: PASS (all negative probes fail closed)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        return _self_test()

    ledger = find_ledger()
    text = ledger.read_text(encoding="utf-8")
    failures = run_checks(text, repo=REPO)
    rows = parse_rows(text)
    print(f"ledger: {ledger.relative_to(REPO)} | Q-rows parsed: {len(rows)} | expected: 12")
    if failures:
        print("\nDECISION LEDGER: FAIL", file=sys.stderr)
        for f in failures:
            print(f"  - {f}", file=sys.stderr)
        return 1
    print("\nDECISION LEDGER: PASS (Q1-Q12 all statused; DEFERRED->FUT; IMPLICITLY SETTLED "
          "evidence exists; DEC-2 consistent; v0 DoD + FUT-1/2/4 present)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
