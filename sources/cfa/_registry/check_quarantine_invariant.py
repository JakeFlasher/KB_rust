#!/usr/bin/env python3
"""AC-7 gate: the notes-taint quarantine is frozen and invariant-checked.

Re-runnable, fail-closed (non-zero exit on any violation). Asserts:

  1. The scope ledger's `quarantined` bucket is EXACTLY the 6 canonical notes-taint
     IDs, and each entry carries a Critical-Rule-9 / notes-taint exclusion reason +
     a non-empty re-authoring criterion.
  2. Each of the 6 quarantined IDs is ABSENT from `cards_manifest.json`,
     `summaries.json`, `INDEX.md`, and the on-disk card tree.
  3. No ACTIVE card frontmatter contains a `notes_provenance` field at ANY nesting
     (Rule 9 not relaxed via a provenance escape hatch).
  4. No ACTIVE card cites a `notes/` or `scripts/` path in a source reference
     (the Rule-9 operational invariant on the active corpus).
  5. The canonical Critical Rule 9 statement is intact. REPRODUCIBLE: validated against
     the TRACKED in-repo pin `rule9_canonical.md` (fail-closed if that committed pin is
     missing or weakened). The live legacy `CLAUDE.md` is an OPTIONAL drift cross-check —
     if present it must also carry the key clauses; if absent it is a NOTE, not a failure,
     so the gate passes on a clean checkout / CI box that lacks the author's legacy
     sibling (no `KB_ALLOW_MISSING_RULE9_SOURCE` opt-in required).

`--self-test` proves the negative tests fail closed: it feeds synthetic violations
(a quarantine ID leaked into the manifest/summaries/INDEX; a top-level AND a nested
`notes_provenance` field; a cited `notes/` path including a non-leading path segment;
a missing tracked pin; a weakened pin; a weakened live legacy source) through the same
`run_checks` and asserts each is caught, while the clean baseline — including the
legacy-absent case — passes.
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
OUT = REPO / "out/cfa"
CARDS_DIR = REPO / "cards/cfa"
SCOPE_LEDGER = REPO / "sources/cfa/_registry/release_baseline/scope_ledger.json"
LEGACY_ROOT = Path(os.environ.get("KB_LEGACY_ROOT", "/home/jakeshea/CFA_reading"))
LEGACY_CLAUDE_MD = LEGACY_ROOT / "CLAUDE.md"
# Tracked, in-repo source-of-record for Critical Rule 9 (makes the gate reproducible off
# the author's machine). The validated text is delimited by these markers.
RULE9_PIN = REPO / "sources/cfa/_registry/rule9_canonical.md"
RULE9_PIN_BEGIN = "<!-- CANONICAL-RULE9-BEGIN -->"
RULE9_PIN_END = "<!-- CANONICAL-RULE9-END -->"

# Canonical notes-taint quarantine set. EMPTY as of the 2026-06 quarantine-absorption:
# all 6 formerly-quarantined notes-taint cards (5 risk-management VaR/risk cards +
# pm-tracking-error-and-active-risk) were re-authored from non-notes primary sources
# (McNeil QRM 2015 / CFA L1) and admitted to the active corpus, so the active corpus is
# now 408 with zero quarantine. The invariant still enforces (fail-closed) that NO active
# card carries a notes_provenance field or cites a notes//scripts/ path and that Critical
# Rule 9 is intact, so the Rule-9 guarantee persists with an empty quarantine set.
QUARANTINE_IDS: frozenset[str] = frozenset()

# Synthetic ids used ONLY by the self-test to prove the negative probes fail closed even
# though the live quarantine set is empty (decoupled from QUARANTINE_IDS by design).
_SELFTEST_PROBE_IDS = frozenset({"rm-monte-carlo-var", "rm-parametric-var"})

# Key clauses of Critical Rule 9 that must remain present (unrelaxed) in legacy CLAUDE.md.
RULE9_KEY_CLAUSES = ("User-volatile folders hard-block", "notes/", "scripts/", "hard-block")

# Rule 9 forbids CITING a notes/ or scripts/ path as a source reference. The lookbehind
# excludes only WORD chars so `notes`/`scripts` as a word SUFFIX (endnotes/, transcripts/)
# is not matched, but a path SEGMENT preceded by `/`, `.`, space, etc. (e.g. ../notes/foo,
# /abs/scripts/x, notes/foo) IS matched. Scoped to source-reference lines so prose is safe.
_NOTES_SCRIPTS_PATH = re.compile(r"(?<![\w-])(notes|scripts)/")
_SOURCE_REF_LINE = re.compile(
    r"(Primary raw source|Supporting sources|\*\*Source:\*\*|(?<![\w])Source:|source_id)",
    re.IGNORECASE,
)
# A notes_provenance key at any frontmatter nesting depth: leading whitespace and/or
# YAML list dashes (`- `) may precede it (top-level, indented map key, or list-item key).
_NOTES_PROVENANCE_KEY = re.compile(r"(?m)^[ \t-]*notes_provenance[ \t]*:")


def extract_pinned_rule9(pin_path: Path) -> str | None:
    """Return the verbatim Rule-9 text between the canonical markers in the tracked pin,
    or None if the file or either marker is absent. Only the delimited block is returned
    so the pin's prose header can never mask a weakening of the rule."""
    if not pin_path.is_file():
        return None
    text = pin_path.read_text(encoding="utf-8", errors="replace")
    b = text.find(RULE9_PIN_BEGIN)
    e = text.find(RULE9_PIN_END)
    if b == -1 or e == -1 or e <= b:
        return None
    return text[b + len(RULE9_PIN_BEGIN):e].strip()


def rule9_failures(
    *,
    pinned_rule9_text: str | None,
    legacy_rule9_text: str | None,
    key_clauses: tuple[str, ...] = RULE9_KEY_CLAUSES,
) -> list[str]:
    """Pure: validate Critical Rule 9 is intact and unrelaxed.

    Reproducible part (fail-closed): the TRACKED in-repo pin must be present and must
    contain every key clause. The pin is committed, so its absence is a real defect.

    Drift part (optional): if the live legacy `CLAUDE.md` text is provided it must also
    contain the clauses; if it is None (clean checkout without the legacy sibling) that is
    NOT a failure — the tracked pin is authoritative for the gate. This is what makes the
    release gate reproducible off the author's filesystem.
    """
    failures: list[str] = []
    if pinned_rule9_text is None:
        failures.append(
            f"tracked Critical-Rule-9 pin missing/malformed at {RULE9_PIN.name} "
            "(expected the canonical markers); cannot verify Rule 9 reproducibly"
        )
    else:
        missing = [c for c in key_clauses if c not in pinned_rule9_text]
        if missing:
            failures.append(f"tracked Critical-Rule-9 pin weakened/incomplete; missing clauses: {missing}")
    if legacy_rule9_text is not None:
        missing = [c for c in key_clauses if c not in legacy_rule9_text]
        if missing:
            failures.append(f"Critical Rule 9 weakened/absent in legacy CLAUDE.md; missing clauses: {missing}")
    return failures


def find_active_card_files(cards_dir: Path) -> list[Path]:
    """Active card .md files: same filter as the corpus gate / scope ledger."""
    out = []
    for md in cards_dir.rglob("*.md"):
        name = md.name
        if name == "INDEX.md" or name.startswith("_") or ".history" in name:
            continue
        out.append(md)
    return out


def frontmatter_block(text: str) -> str:
    """The raw frontmatter block (between the first two `---`), or '' if absent."""
    parts = text.split("---", 2)
    return parts[1] if len(parts) >= 3 else ""


def load_active_cards(cards_dir: Path) -> list[dict]:
    cards = []
    for md in find_active_card_files(cards_dir):
        text = md.read_text(encoding="utf-8", errors="replace")
        cards.append({
            "id": md.stem,
            "rel": str(md.relative_to(REPO)),
            "frontmatter": frontmatter_block(text),
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
    pinned_rule9_text: str | None,
    legacy_rule9_text: str | None,
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

    # (3) No active card frontmatter carries a notes_provenance field (any nesting).
    for card in active_cards:
        if _NOTES_PROVENANCE_KEY.search(card.get("frontmatter", "")):
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

    # (5) Canonical Rule 9 intact — validated against the TRACKED pin (reproducible,
    # fail-closed) with the live legacy CLAUDE.md as an OPTIONAL drift cross-check.
    failures.extend(rule9_failures(
        pinned_rule9_text=pinned_rule9_text,
        legacy_rule9_text=legacy_rule9_text,
    ))

    return failures


def _self_test() -> int:
    """Prove the negative tests fail closed (synthetic violations are caught)."""
    # Probe against a SYNTHETIC non-empty quarantine set so the negative probes still
    # exercise checks (1)/(2) even though the live QUARANTINE_IDS is now empty.
    base: dict[str, Any] = dict(
        quarantine_ids=set(_SELFTEST_PROBE_IDS),
        ledger_quarantined=[
            {"card_id": q, "reason": "notes-taint (Critical Rule 9): ...", "reauthor_criterion": "re-author ..."}
            for q in _SELFTEST_PROBE_IDS
        ],
        manifest_ids={"pm-capm-and-sml"},
        summary_ids={"pm-capm-and-sml"},
        index_text="| pm-capm-and-sml | CAPM | 1 | abc |",
        on_disk_ids={"pm-capm-and-sml"},
        active_cards=[{"id": "pm-capm-and-sml", "rel": "x.md", "frontmatter": "id: x\ncitations:", "text": "ok"}],
        pinned_rule9_text="User-volatile folders hard-block ... notes/ ... scripts/ ... hard-block",
        legacy_rule9_text="User-volatile folders hard-block ... notes/ ... scripts/ ... hard-block",
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
    # The live post-absorption reality: an EMPTY quarantine set + empty ledger bucket passes.
    expect_clean("empty quarantine set + empty ledger bucket (post-2026-06 absorption)",
                 quarantine_ids=set(), ledger_quarantined=[])
    expect_caught("quarantine id leaked into manifest", "cards_manifest",
                  manifest_ids={"pm-capm-and-sml", "rm-monte-carlo-var"})
    expect_caught("quarantine id leaked into summaries", "summaries.json",
                  summary_ids={"rm-parametric-var"})
    expect_caught("quarantine id leaked into INDEX", "INDEX.md",
                  index_text="| rm-monte-carlo-var | VaR | 1 | x |")
    expect_caught("notes_provenance on active card (top-level)", "notes_provenance",
                  active_cards=[{"id": "c", "rel": "c.md", "frontmatter": "id: c\nnotes_provenance: foo", "text": "ok"}])
    expect_caught("notes_provenance on active card (nested/indented)", "notes_provenance",
                  active_cards=[{"id": "c", "rel": "c.md", "frontmatter": "id: c\ncitations:\n  - notes_provenance: foo", "text": "ok"}])
    expect_caught("active card cites notes/ path (leading)", "notes/ or scripts/",
                  active_cards=[{"id": "c", "rel": "c.md", "frontmatter": "id: c", "text": "**Source:** notes/foo.pdf"}])
    expect_caught("active card cites notes/ path (non-leading segment)", "notes/ or scripts/",
                  active_cards=[{"id": "c", "rel": "c.md", "frontmatter": "id: c", "text": "**Source:** ../user/notes/foo.pdf"}])
    expect_caught("active card cites scripts/ path (abs)", "notes/ or scripts/",
                  active_cards=[{"id": "c", "rel": "c.md", "frontmatter": "id: c", "text": "Primary raw source: /home/u/scripts/kb/x.py"}])
    expect_clean("prose mentioning endnotes/ is NOT a violation",
                 active_cards=[{"id": "c", "rel": "c.md", "frontmatter": "id: c", "text": "**Source:** Smith, endnotes/appendix discussion"}])
    # (5) Rule-9 reproducible-pin + optional-legacy-drift probes.
    expect_caught("tracked Rule-9 pin missing", "pin missing/malformed",
                  pinned_rule9_text=None)
    expect_caught("tracked Rule-9 pin weakened", "pin weakened/incomplete",
                  pinned_rule9_text="(rule removed)")
    expect_caught("tracked Rule-9 pin dropped scripts/ clause", "pin weakened/incomplete",
                  pinned_rule9_text="User-volatile folders hard-block ... notes/ ... hard-block")
    expect_caught("live legacy Rule 9 weakened (drift)", "legacy CLAUDE.md",
                  legacy_rule9_text="(rule removed)")
    expect_clean("legacy source absent is a NOTE, not a failure (reproducible)",
                 legacy_rule9_text=None)
    expect_clean("pin good + legacy absent passes clean", legacy_rule9_text=None)
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

    # Reproducible source-of-record: the TRACKED in-repo pin (required).
    pinned_rule9_text = extract_pinned_rule9(RULE9_PIN)
    # Optional drift cross-check: the live legacy CLAUDE.md (absent on a clean checkout).
    legacy_rule9_text = None
    if LEGACY_CLAUDE_MD.is_file():
        legacy_rule9_text = LEGACY_CLAUDE_MD.read_text(encoding="utf-8", errors="replace")
    else:
        print(f"NOTE: legacy CLAUDE.md not present at {LEGACY_CLAUDE_MD}; Rule-9 drift "
              "cross-check skipped. The tracked pin (rule9_canonical.md) is authoritative "
              "and is still enforced.")

    failures = run_checks(
        quarantine_ids=set(QUARANTINE_IDS),
        ledger_quarantined=ledger_quarantined,
        manifest_ids=manifest_ids,
        summary_ids=summary_ids,
        index_text=index_text,
        on_disk_ids=on_disk_ids,
        active_cards=active_cards,
        pinned_rule9_text=pinned_rule9_text,
        legacy_rule9_text=legacy_rule9_text,
    )

    pin_status = "present" if pinned_rule9_text is not None else "MISSING"
    drift = "checked" if legacy_rule9_text is not None else "skipped (legacy absent)"
    print(f"quarantine IDs: {len(QUARANTINE_IDS)} | active cards scanned: {len(active_cards)} | "
          f"manifest: {len(manifest_ids)} | summaries: {len(summary_ids)} | "
          f"Rule-9 tracked pin: {pin_status} | legacy drift cross-check: {drift}")
    if failures:
        print("\nQUARANTINE INVARIANT: FAIL", file=sys.stderr)
        for f in failures:
            print(f"  - {f}", file=sys.stderr)
        return 1
    print("\nQUARANTINE INVARIANT: PASS (quarantine set empty post-2026-06 absorption; "
          "ledger quarantined bucket empty; no active card carries notes_provenance; "
          "no active card cites a notes//scripts/ path; Rule 9 intact)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
