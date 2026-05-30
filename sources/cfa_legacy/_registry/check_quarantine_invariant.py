#!/usr/bin/env python3
"""AC-7 gate: the notes-taint quarantine is frozen and invariant-checked.

Re-runnable, fail-closed (non-zero exit on any violation). Asserts:

  1. The scope ledger's `quarantined` bucket is EXACTLY the 6 canonical notes-taint
     IDs, and each entry carries a Critical-Rule-9 / notes-taint exclusion reason +
     a non-empty re-authoring criterion.
  2. Each of the 6 quarantined IDs is ABSENT from `cards_manifest.json`,
     `summaries.json`, `INDEX.md`, and the on-disk card tree.
  3. No ACTIVE card frontmatter contains a `notes_provenance` field (Rule 9 not
     relaxed via a provenance escape hatch).
  4. No ACTIVE card cites a `notes/` or `scripts/` path (the Rule-9 operational
     invariant on the active corpus).
  5. The canonical Critical Rule 9 statement in the legacy `CLAUDE.md` is intact
     (hard-checked when that read-only sibling is reachable; a NOTE otherwise).

`--self-test` proves the negative tests fail closed: it feeds synthetic violations
(a quarantine ID leaked into the manifest; a `notes_provenance` field on an active
card; a relaxed Rule-9 text) through the same `run_checks` and asserts each is caught.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from pathlib import Path
from typing import Any

REPO = Path(__file__).resolve().parents[3]
OUT = REPO / "out/cfa_legacy"
CARDS_DIR = REPO / "cards/cfa_legacy"
SCOPE_LEDGER = REPO / "sources/cfa_legacy/_registry/v0_baseline/scope_ledger.json"
LEGACY_ROOT = Path(os.environ.get("KB_LEGACY_ROOT", "/home/jakeshea/CFA_reading"))
LEGACY_CLAUDE_MD = LEGACY_ROOT / "CLAUDE.md"

# Canonical AC-7 quarantine set (plan AC-7 positive test).
QUARANTINE_IDS = frozenset({
    "rm-historical-simulation-var",
    "rm-monte-carlo-var",
    "rm-parametric-var",
    "rm-risk-objectives-and-tolerance",
    "rm-sensitivity-versus-simulation",
    "pm-tracking-error-and-active-risk",
})

# Key clauses of Critical Rule 9 that must remain present (unrelaxed) in legacy CLAUDE.md.
RULE9_KEY_CLAUSES = ("User-volatile folders hard-block", "notes/", "hard-block")

# Rule 9 forbids CITING a notes/ or scripts/ path as a source reference. Scope the
# scan to source-reference lines (cacg.v0 `source_id:` + legacy `Primary raw source:`
# / `Supporting sources:` / `**Source:**`) so prose mentioning "scripts/" is not a
# false positive while a cited path still fails.
_NOTES_SCRIPTS_PATH = re.compile(r"(?<![\w./-])(notes|scripts)/")
_SOURCE_REF_LINE = re.compile(
    r"(Primary raw source|Supporting sources|\*\*Source:\*\*|(?<![\w])Source:|source_id)",
    re.IGNORECASE,
)


def find_active_card_files(cards_dir: Path) -> list[Path]:
    """Active card .md files: same filter as the corpus gate / scope ledger."""
    out = []
    for md in cards_dir.rglob("*.md"):
        name = md.name
        if name == "INDEX.md" or name.startswith("_") or ".history" in name:
            continue
        out.append(md)
    return out


def frontmatter_top_keys(text: str) -> set[str]:
    parts = text.split("---", 2)
    if len(parts) < 3:
        return set()
    keys = set()
    for line in parts[1].splitlines():
        m = re.match(r"^([A-Za-z_][\w]*):", line)
        if m:
            keys.add(m.group(1))
    return keys


def load_active_cards(cards_dir: Path) -> list[dict]:
    cards = []
    for md in find_active_card_files(cards_dir):
        text = md.read_text(encoding="utf-8", errors="replace")
        cards.append({
            "id": md.stem,
            "rel": str(md.relative_to(REPO)),
            "frontmatter_keys": frontmatter_top_keys(text),
            "text": text,
        })
    return cards


def run_checks(
    *,
    quarantine_ids: set[str],
    ledger_quarantined: list[dict],
    manifest_ids: set[str],
    summary_ids: set[str],
    index_text: str,
    on_disk_ids: set[str],
    active_cards: list[dict],
    rule9_text: str | None,
) -> list[str]:
    """Return a list of failure strings; empty iff every invariant holds."""
    failures: list[str] = []

    # (1) Scope ledger quarantined bucket == canonical set, with reasons + criteria.
    ledger_ids = {str(e["card_id"]) for e in ledger_quarantined if e.get("card_id")}
    if ledger_ids != quarantine_ids:
        failures.append(
            f"scope-ledger quarantined set != canonical AC-7 set; "
            f"missing={sorted(quarantine_ids - ledger_ids)}, extra={sorted(ledger_ids - quarantine_ids)}"
        )
    for e in ledger_quarantined:
        cid = e.get("card_id")
        reason = (e.get("reason") or "")
        if "Rule 9" not in reason and "notes-taint" not in reason:
            failures.append(f"quarantined {cid}: reason does not cite notes-taint / Critical Rule 9: {reason!r}")
        if not (e.get("reauthor_criterion") or "").strip():
            failures.append(f"quarantined {cid}: missing re-authoring criterion")

    # (2) Each quarantined ID absent from manifest / summaries / INDEX / disk.
    for qid in sorted(quarantine_ids):
        if qid in manifest_ids:
            failures.append(f"quarantined {qid} present in cards_manifest.json")
        if qid in summary_ids:
            failures.append(f"quarantined {qid} present in summaries.json")
        if qid in index_text:
            failures.append(f"quarantined {qid} present in INDEX.md")
        if qid in on_disk_ids:
            failures.append(f"quarantined {qid} present as an on-disk card file")

    # (3) No active card frontmatter carries a notes_provenance field.
    for card in active_cards:
        if "notes_provenance" in card["frontmatter_keys"]:
            failures.append(f"active card {card.get('rel', card['id'])} has a notes_provenance frontmatter field (Rule 9 relaxed)")

    # (4) No active card cites a notes/ or scripts/ path in a source reference.
    for card in active_cards:
        for line in card["text"].splitlines():
            if _SOURCE_REF_LINE.search(line) and _NOTES_SCRIPTS_PATH.search(line):
                failures.append(
                    f"active card {card.get('rel', card['id'])} cites a notes/ or scripts/ path "
                    f"in a source reference (Rule 9 violation): {line.strip()[:80]!r}"
                )
                break

    # (5) Canonical Rule 9 statement intact (hard when the legacy CLAUDE.md is reachable).
    if rule9_text is not None:
        missing = [c for c in RULE9_KEY_CLAUSES if c not in rule9_text]
        if missing:
            failures.append(f"Critical Rule 9 weakened/absent in legacy CLAUDE.md; missing clauses: {missing}")

    return failures


def _self_test() -> int:
    """Prove the negative tests fail closed (synthetic violations are caught)."""
    base: dict[str, Any] = dict(
        quarantine_ids=set(QUARANTINE_IDS),
        ledger_quarantined=[
            {"card_id": q, "reason": "notes-taint (Critical Rule 9): ...", "reauthor_criterion": "re-author ..."}
            for q in QUARANTINE_IDS
        ],
        manifest_ids={"pm-capm-and-sml"},
        summary_ids={"pm-capm-and-sml"},
        index_text="| pm-capm-and-sml | CAPM | 1 | abc |",
        on_disk_ids={"pm-capm-and-sml"},
        active_cards=[{"id": "pm-capm-and-sml", "rel": "x.md", "frontmatter_keys": {"id", "citations"}, "text": "ok"}],
        rule9_text="User-volatile folders hard-block ... notes/ ... hard-block",
    )
    failures = 0

    def expect_clean(name, **over):
        nonlocal failures
        kw = dict(base); kw.update(over)
        res = run_checks(**kw)
        if res:
            failures += 1; print(f"  FAIL (clean case flagged): {name} -> {res}")
        else:
            print(f"  ok    (clean passes): {name}")

    def expect_caught(name, needle, **over):
        nonlocal failures
        kw = dict(base); kw.update(over)
        res = run_checks(**kw)
        if any(needle in f for f in res):
            print(f"  fires (caught): {name}")
        else:
            failures += 1; print(f"  FAIL (violation not caught): {name} -> {res}")

    print("AC-7 self-test (negative probes):")
    expect_clean("baseline synthetic-clean")
    expect_caught("quarantine id leaked into manifest", "cards_manifest",
                  manifest_ids={"pm-capm-and-sml", "rm-monte-carlo-var"})
    expect_caught("quarantine id leaked into summaries", "summaries.json",
                  summary_ids={"rm-parametric-var"})
    expect_caught("quarantine id leaked into INDEX", "INDEX.md",
                  index_text="| rm-monte-carlo-var | VaR | 1 | x |")
    expect_caught("notes_provenance on active card", "notes_provenance",
                  active_cards=[{"id": "c", "rel": "c.md", "frontmatter_keys": {"id", "notes_provenance"}, "text": "ok"}])
    expect_caught("active card cites notes/ path", "notes/ or scripts/",
                  active_cards=[{"id": "c", "rel": "c.md", "frontmatter_keys": {"id"}, "text": "**Source:** notes/foo.pdf"}])
    expect_caught("Rule 9 weakened", "Critical Rule 9 weakened",
                  rule9_text="(rule removed)")
    expect_caught("ledger missing a quarantine id", "quarantined set",
                  ledger_quarantined=base["ledger_quarantined"][:-1])
    expect_caught("ledger entry missing reauthor criterion", "re-authoring criterion",
                  ledger_quarantined=[{**e, "reauthor_criterion": ""} if i == 0 else e
                                      for i, e in enumerate(base["ledger_quarantined"])])
    print()
    if failures:
        print(f"AC-7 SELF-TEST: FAIL ({failures})", file=sys.stderr); return 1
    print("AC-7 SELF-TEST: PASS (all negative probes fail closed)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--self-test", action="store_true", help="run synthetic negative probes and exit")
    args = ap.parse_args()
    if args.self_test:
        return _self_test()

    manifest = json.loads((OUT / "cards_manifest.json").read_text(encoding="utf-8"))
    manifest_ids = {c["id"] for c in manifest["cards"]}
    summaries = json.loads((OUT / "summaries.json").read_text(encoding="utf-8"))
    summary_ids = {s["id"] for s in summaries["summaries"]}
    index_text = (OUT / "INDEX.md").read_text(encoding="utf-8")
    ledger = json.loads(SCOPE_LEDGER.read_text(encoding="utf-8"))
    ledger_quarantined = ledger["categories"]["quarantined"]
    active_cards = load_active_cards(CARDS_DIR)
    on_disk_ids = {c["id"] for c in active_cards}

    rule9_text = None
    if LEGACY_CLAUDE_MD.is_file():
        rule9_text = LEGACY_CLAUDE_MD.read_text(encoding="utf-8", errors="replace")
    else:
        print(f"NOTE: legacy CLAUDE.md not reachable at {LEGACY_CLAUDE_MD}; "
              "Rule-9 text pin skipped (operational checks still enforced).")

    failures = run_checks(
        quarantine_ids=set(QUARANTINE_IDS),
        ledger_quarantined=ledger_quarantined,
        manifest_ids=manifest_ids,
        summary_ids=summary_ids,
        index_text=index_text,
        on_disk_ids=on_disk_ids,
        active_cards=active_cards,
        rule9_text=rule9_text,
    )

    print(f"quarantine IDs: {len(QUARANTINE_IDS)} | active cards scanned: {len(active_cards)} | "
          f"manifest: {len(manifest_ids)} | summaries: {len(summary_ids)} | "
          f"Rule-9 text pin: {'checked' if rule9_text is not None else 'skipped'}")
    if failures:
        print("\nQUARANTINE INVARIANT: FAIL", file=sys.stderr)
        for f in failures:
            print(f"  - {f}", file=sys.stderr)
        return 1
    print("\nQUARANTINE INVARIANT: PASS (6 IDs absent from manifest/summaries/INDEX/disk; "
          "ledger reasons+criteria present; no notes_provenance; no notes//scripts/ citations; Rule 9 intact)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
