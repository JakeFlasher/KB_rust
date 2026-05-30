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
import json
import os
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


def derive_disk_facts(repo: Path) -> dict:
    """Re-derive the on-disk truth the ledger's factual rulings must match — NOT trust
    the ledger's own prose (the R8 errors came from trusting analyze prose).

    Returns: tsay AFTS edition/year (from source_inventory.json) and the set of
    `targeted_books_Chinese` sources that are status:active AND cited by >=1 active card.
    """
    reg = repo / "sources/cfa_legacy/_registry"
    inv_raw = json.loads((reg / "source_inventory.json").read_text(encoding="utf-8"))
    inv = inv_raw["sources"] if isinstance(inv_raw, dict) and "sources" in inv_raw else \
        {e["source_id"]: e for e in inv_raw if isinstance(e, dict) and "source_id" in e}
    tsay = inv.get("qm_tsay_2005_afts_2e", {})
    # The Tsay `edition` is a single descriptive string (e.g. "Tsay 2e 2005 *...*").
    tsay_edition = str(tsay.get("edition", ""))
    chinese_active = [
        sid for sid, e in inv.items()
        if "targeted_books_Chinese" in json.dumps(e, ensure_ascii=False) and e.get("status") == "active"
    ]
    cited = set()
    cards = [
        c for c in glob.glob(str(repo / "cards/cfa_legacy/**/*.md"), recursive=True)
        if ".history" not in c and os.path.basename(c) != "INDEX.md" and not os.path.basename(c).startswith("_")
    ]
    for c in cards:
        parts = Path(c).read_text(encoding="utf-8", errors="replace").split("---", 2)
        fm = parts[1] if len(parts) >= 3 else ""
        for sid in chinese_active:
            if ('source_id: "%s"' % sid) in fm:
                cited.add(sid)
    return {
        "tsay_edition": tsay_edition,
        "tsay_is_2e_2005": ("2e" in tsay_edition and "2005" in tsay_edition),
        "tsay_has_3e_2010": ("3e" in tsay_edition or "2010" in tsay_edition),
        "chinese_active_cited": sorted(cited),
    }


def parse_rows(text: str) -> list[tuple[str, str, str, str]]:
    """Return [(qid, status, evidence_or_fut, ruling)] from the disposition table."""
    rows = []
    for line in text.splitlines():
        m = TABLE_ROW_RE.match(line)
        if m:
            rows.append((m.group(1), m.group(2).strip(), m.group(3).strip(), m.group(4).strip()))
    return rows


def run_checks(text: str, *, repo: Path, evidence_must_exist: bool = True,
               disk_facts: dict | None = None) -> list[str]:
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

    facts = disk_facts if disk_facts is not None else derive_disk_facts(repo)

    # (7) Tsay AFTS edition (Codex R8 gap 2): the machine-check block must assert
    # 2e/2005, the on-disk metadata must AGREE (re-derived, not prose-trusted), and the
    # ledger must not reintroduce the false 3e/2010 finding.
    te, ty = _val("tsay_afts_edition"), _val("tsay_afts_year")
    if te is None or ty is None:
        failures.append("Tsay machine-check block missing (need tsay_afts_edition + tsay_afts_year)")
    elif (te, ty) != ("2e", "2005"):
        failures.append(f"Tsay machine-check must assert 2e/2005, got {te!r}/{ty!r}")
    if not facts.get("tsay_is_2e_2005") or facts.get("tsay_has_3e_2010"):
        failures.append(
            f"on-disk Tsay edition metadata is {facts.get('tsay_edition')!r}; must be 2e/2005 "
            "(PDF title page reads 'Second Edition') and must not say 3e/2010"
        )
    for bad in ("actual edition is Tsay 3e", "suggests 3e/2010", "token falsely claims", "CRITICAL MISMATCH"):
        if bad in text:
            failures.append(f"reintroduced false Tsay 3e/2010 claim: {bad!r}")

    # (8) Q2 targeted-Chinese (Codex R8 gap 1): re-derive the active+cited set from disk;
    # if non-empty the ledger must NOT claim all-excluded, the machine flag must be false,
    # and the Q2 ruling must acknowledge each active+cited source by id.
    active_cited = facts.get("chinese_active_cited", [])
    q2 = by_id.get("Q2")
    q2text = q2[3] if q2 else ""
    all_excl = _val("q2_targeted_chinese_all_excluded")
    if active_cited:
        if all_excl != "false":
            failures.append(
                f"Q2 machine-check q2_targeted_chinese_all_excluded must be false "
                f"({len(active_cited)} targeted_books_Chinese sources are active+cited), got {all_excl!r}"
            )
        for sid in active_cited:
            if sid not in q2text:
                failures.append(f"Q2 ruling does not acknowledge active+cited Chinese source {sid}")
        for bad in ("no active v0 card cites them", "already excluded on disk",
                    "are already excluded"):
            if bad in q2text:
                failures.append(f"Q2 ruling contains a false blanket-exclusion phrase: {bad!r}")

    return failures


def _self_test() -> int:
    # Hermetic synthetic disk facts (so the self-test does not depend on live disk):
    # one active+cited Chinese source + Tsay correct at 2e/2005.
    FACTS = {"tsay_edition": "Tsay 2e 2005 *AFTS*", "tsay_is_2e_2005": True,
             "tsay_has_3e_2010": False, "chinese_active_cited": ["cb_probe_active_cited"]}
    q2_good = "| Q2 | DECIDED | `x` | v0 accepts cb_probe_active_cited (active+cited); EPUB excluded |"
    base_rows = "\n".join(
        (q2_good if i == 2 else f"| Q{i} | DECIDED | — | ruling {i} |") for i in range(1, 13)
    )
    machine = ("\nv0_card_citation_coordinate: pdf_page\nvolume_page_in_card_frontmatter: false\n"
               "tsay_afts_edition: 2e\ntsay_afts_year: 2005\n"
               "q2_targeted_chinese_all_excluded: false\n")
    good = (
        "# ledger\n\nv0 complete := 268 active cards + 6 quarantined legacy cards.\n\n"
        "| Question | Status | Evidence / FUT | Ruling |\n|--|--|--|--|\n"
        + base_rows
        + "\nFUT-1 ... FUT-2 ... FUT-4 ...\n"
        + machine
    )
    failures = 0

    def expect_clean(name, text, **kw):
        nonlocal failures
        kw.setdefault("evidence_must_exist", False)
        kw.setdefault("disk_facts", FACTS)
        res = run_checks(text, repo=REPO, **kw)
        if res:
            failures += 1; print(f"  FAIL (clean flagged): {name} -> {res}")
        else:
            print(f"  ok    (clean passes): {name}")

    def expect_caught(name, text, needle, **kw):
        nonlocal failures
        kw.setdefault("evidence_must_exist", False)
        kw.setdefault("disk_facts", FACTS)
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
    # (R9 check 7) Tsay guards
    tsay_block = good.replace("tsay_afts_edition: 2e", "tsay_afts_edition: 3e")
    expect_caught("Tsay machine-block says 3e", tsay_block, "must assert 2e/2005")
    expect_caught("Tsay on-disk metadata is 3e/2010", good, "on-disk Tsay edition metadata",
                  disk_facts={"tsay_edition": "Tsay 3e 2010", "tsay_is_2e_2005": False,
                              "tsay_has_3e_2010": True, "chinese_active_cited": ["cb_probe_active_cited"]})
    tsay_false = good + "\nThe actual edition is Tsay 3e per audit; suggests 3e/2010.\n"
    expect_caught("Tsay false prose reintroduced", tsay_false, "reintroduced false Tsay")
    # (R9 check 8) Q2 Chinese guards
    q2_allexcl = good.replace("q2_targeted_chinese_all_excluded: false", "q2_targeted_chinese_all_excluded: true")
    expect_caught("Q2 flag true while active exist", q2_allexcl, "must be false")
    q2_unack = good.replace(q2_good, "| Q2 | DECIDED | `x` | all four Chinese books excluded |")
    expect_caught("Q2 ruling omits an active+cited source", q2_unack, "does not acknowledge active+cited")
    q2_blanket = good.replace(q2_good,
                              "| Q2 | DECIDED | `x` | cb_probe_active_cited: no active v0 card cites them |")
    expect_caught("Q2 ruling has blanket-exclusion phrase", q2_blanket, "blanket-exclusion phrase")

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
