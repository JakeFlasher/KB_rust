#!/usr/bin/env python3
"""Capture the Layer-2 per-citation tally over the migrated QM
vertical, satisfying the AC-8 contract "the full per-citation
tally is captured."

For each of the 17 cards under
`tests/parity_corpus/cards/reading_01_qm/`, this script:

  1. Loads the card via `cacg.card_loader.load_card`.
  2. For each citation in the card, calls
     `cacg.verify.layer2.verify_citation` with
     `fuzzy_enabled=False` (strict Layer-2 substring match).
  3. For citations that fail strict, calls again with
     `fuzzy_enabled=True` to capture the fuzzy-only bucket.
  4. Tallies per-card and aggregate counts under three
     buckets:
       - STRICT pass: substring match against the chunk's
         page-window-restricted text.
       - FUZZY pass: failed strict but the Levenshtein-bounded
         fuzzy matcher accepted it.
       - FAIL: neither strict nor fuzzy accepted.
  5. Writes a Markdown report to
     `_research/qm_layer2_tally.md` with the aggregate +
     per-card breakdown. AC-9 (task-m4-10) consumes this
     report as the input to `_research/18_cfa_real_migration_findings.md`.

AC-8 is "measure, don't gate" — the script ALWAYS exits 0 as
long as every citation produced a determinate verdict (which
`verify_citation` always does, by construction). A low pass
rate is a recorded finding, not a milestone failure.

Usage:
    .venv/bin/python scripts/qm_vertical_layer2_tally.py
"""

import json
import pathlib
import subprocess
import sys
from collections import Counter, defaultdict
from dataclasses import dataclass

from cacg.card_loader import CardLoadError, load_card
from cacg.chunks_index import ChunksIndex
from cacg.verify.layer2 import verify_citation


def _short_git_sha() -> str:
    try:
        r = subprocess.run(
            ["git", "rev-parse", "--short=12", "HEAD"],
            check=True,
            capture_output=True,
            text=True,
        )
        return r.stdout.strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        return "<no-git>"


def _short_git_date() -> str:
    try:
        r = subprocess.run(
            ["git", "log", "-1", "--format=%cs", "HEAD"],
            check=True,
            capture_output=True,
            text=True,
        )
        return r.stdout.strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        return "<no-git>"


WORKSPACE_ROOT = pathlib.Path(__file__).resolve().parents[2]
CARDS_DIR = WORKSPACE_ROOT / "tests/parity_corpus/cards/reading_01_qm"
CHUNKS_MANIFEST = (
    WORKSPACE_ROOT / "tests/parity_corpus/out_python/qm_vertical/chunks_manifest.json"
)
REPORT_PATH = WORKSPACE_ROOT / "_research/qm_layer2_tally.md"


@dataclass
class CitationVerdict:
    card_id: str
    citation_index: int
    source_id: str
    chunk_id: str
    bucket: str  # "strict" | "fuzzy" | "fail"
    fail_reason: str | None  # populated only when bucket == "fail"


def verdict_one(
    card_id: str,
    citation_index: int,
    citation,
    chunks_by_id: dict,
    chunks_by_source: dict[str, list],
    tamper_cache: dict,
) -> CitationVerdict:
    """Verify a single citation under strict, then fuzzy. Return
    the bucket the citation lands in."""
    # Round-19 review P2-A: pass the per-source chunk list (not
    # all_chunks) so BM25 hints computed on the failure path stay
    # scoped to citation.source_id per the bm25_hints contract.
    same_source = chunks_by_source.get(citation.source_id, [])
    strict = verify_citation(
        citation,
        chunks_by_id,
        same_source,
        fuzzy_enabled=False,
        citation_index=citation_index,
        tamper_cache=tamper_cache,
    )
    if strict.verified:
        return CitationVerdict(
            card_id=card_id,
            citation_index=citation_index,
            source_id=citation.source_id,
            chunk_id=citation.chunk_id,
            bucket="strict",
            fail_reason=None,
        )
    fuzzy = verify_citation(
        citation,
        chunks_by_id,
        same_source,
        fuzzy_enabled=True,
        citation_index=citation_index,
        tamper_cache=tamper_cache,
    )
    if fuzzy.verified:
        return CitationVerdict(
            card_id=card_id,
            citation_index=citation_index,
            source_id=citation.source_id,
            chunk_id=citation.chunk_id,
            bucket="fuzzy",
            fail_reason=None,
        )
    # Both fail — capture the strict diagnostic message for the
    # report. `strict.diagnostics[0]` is the CACG-VERIFY-001
    # message that names citations[i] and the failure mode.
    reason = (
        strict.diagnostics[0].message
        if strict.diagnostics
        else "verify_citation returned no diagnostic"
    )
    return CitationVerdict(
        card_id=card_id,
        citation_index=citation_index,
        source_id=citation.source_id,
        chunk_id=citation.chunk_id,
        bucket="fail",
        fail_reason=reason,
    )


def main() -> int:
    if not CARDS_DIR.is_dir():
        print(f"error: cards dir not found at {CARDS_DIR}", file=sys.stderr)
        return 1
    if not CHUNKS_MANIFEST.is_file():
        print(
            f"error: chunks manifest not found at {CHUNKS_MANIFEST}",
            file=sys.stderr,
        )
        return 1

    chunks_index = ChunksIndex.from_path(CHUNKS_MANIFEST)
    chunks_by_id = chunks_index.by_id
    # Build the per-source chunk lists so verify_citation gets a
    # source-scoped list (per Round-19 review P2-A). The strict
    # path doesn't read it; the fail-path BM25 hint computation
    # does, and must stay scoped to citation.source_id.
    chunks_by_source: dict[str, list] = defaultdict(list)
    for c in chunks_by_id.values():
        chunks_by_source[c.source_id].append(c)

    card_paths = sorted(CARDS_DIR.glob("qm-*.md"))
    if not card_paths:
        print(f"error: no cards found in {CARDS_DIR}", file=sys.stderr)
        return 1

    verdicts: list[CitationVerdict] = []
    per_card: dict[str, Counter] = {}
    tamper_cache: dict[str, bool] = {}

    for path in card_paths:
        try:
            doc = load_card(path)
        except CardLoadError as exc:
            # Round-19 review P3-A: don't let one malformed card
            # abort the entire tally. Synthesize a single "fail"
            # bucket entry so the per-card row is non-empty.
            card_id = path.stem
            per_card[card_id] = Counter({"fail": 1})
            verdicts.append(
                CitationVerdict(
                    card_id=card_id,
                    citation_index=-1,
                    source_id="<unloaded>",
                    chunk_id="<unloaded>",
                    bucket="fail",
                    fail_reason=f"CardLoadError: {exc.diagnostics[0].message if exc.diagnostics else exc}",
                )
            )
            continue
        card_id = doc.frontmatter.id
        per_card[card_id] = Counter()
        for i, citation in enumerate(doc.frontmatter.citations):
            v = verdict_one(
                card_id, i, citation, chunks_by_id, chunks_by_source, tamper_cache
            )
            verdicts.append(v)
            per_card[card_id][v.bucket] += 1

    totals = Counter(v.bucket for v in verdicts)
    total = sum(totals.values())
    if total == 0:
        print("error: no citations across the 17 cards", file=sys.stderr)
        return 1

    # Emit Markdown report.
    REPORT_PATH.parent.mkdir(parents=True, exist_ok=True)
    lines: list[str] = []
    git_sha = _short_git_sha()
    git_date = _short_git_date()
    lines.append("# QM Vertical — Layer-2 Per-Citation Tally")
    lines.append("")
    lines.append(f"_captured against HEAD `{git_sha}` on `{git_date}`_")
    lines.append("")
    lines.append("Captured by `scripts/qm_vertical_layer2_tally.py` over the")
    lines.append("17 cards under `tests/parity_corpus/cards/reading_01_qm/`")
    lines.append("verified against `tests/parity_corpus/out_python/qm_vertical/chunks_manifest.json`.")
    lines.append("Each citation is run through `cacg.verify.layer2.verify_citation`")
    lines.append("first with `fuzzy_enabled=False` (strict substring match), then")
    lines.append("again with `fuzzy_enabled=True` on any strict-fail; the verdict")
    lines.append("buckets the citation into `strict` / `fuzzy` / `fail`.")
    lines.append("")
    lines.append("## Aggregate")
    lines.append("")
    lines.append(f"- Total citations: **{total}**")
    lines.append(
        f"- STRICT pass: **{totals['strict']}** "
        f"({100*totals['strict']/total:.1f}%)"
    )
    lines.append(
        f"- FUZZY pass:  **{totals['fuzzy']}** "
        f"({100*totals['fuzzy']/total:.1f}%)"
    )
    lines.append(
        f"- FAIL:        **{totals['fail']}** "
        f"({100*totals['fail']/total:.1f}%)"
    )
    lines.append("")
    lines.append("## Per-card breakdown")
    lines.append("")
    lines.append("| card_id | citations | strict | fuzzy | fail |")
    lines.append("|---------|-----------|--------|-------|------|")
    for card_id in sorted(per_card.keys()):
        c = per_card[card_id]
        n = c["strict"] + c["fuzzy"] + c["fail"]
        lines.append(
            f"| `{card_id}` | {n} | {c['strict']} | "
            f"{c['fuzzy']} | {c['fail']} |"
        )
    lines.append("")
    lines.append("## Per-source breakdown")
    lines.append("")
    per_source: dict[str, Counter] = {}
    for v in verdicts:
        per_source.setdefault(v.source_id, Counter())[v.bucket] += 1
    lines.append("| source_id | citations | strict | fuzzy | fail |")
    lines.append("|-----------|-----------|--------|-------|------|")
    for source_id in sorted(per_source.keys()):
        c = per_source[source_id]
        n = c["strict"] + c["fuzzy"] + c["fail"]
        lines.append(
            f"| `{source_id}` | {n} | {c['strict']} | "
            f"{c['fuzzy']} | {c['fail']} |"
        )
    lines.append("")
    if totals["fail"]:
        lines.append("## Sample failure diagnostics")
        lines.append("")
        lines.append("Up to 10 representative failure reasons (the full set")
        lines.append("is available by re-running this script).")
        lines.append("")
        sample = [v for v in verdicts if v.bucket == "fail"][:10]
        for v in sample:
            lines.append(
                f"- `{v.card_id}` citation #{v.citation_index} "
                f"(`{v.chunk_id}`):"
            )
            lines.append(f"  - {v.fail_reason}")
        lines.append("")
    lines.append("## AC-8 contract")
    lines.append("")
    lines.append("AC-8 mandates that the tally be captured, NOT that the")
    lines.append("pass rate exceed any threshold. Layer-2 failures driven")
    lines.append("by legitimate paraphrase (the migration script extracts")
    lines.append("a short verbatim sentence from each chunk's text, but")
    lines.append("the card's prose paraphrases that content; only the")
    lines.append("citation `quote` is a verbatim substring) are expected.")
    lines.append("The detailed analysis lives in")
    lines.append("`_research/18_cfa_real_migration_findings.md` (task-m4-10).")
    lines.append("")
    REPORT_PATH.write_text("\n".join(lines) + "\n", encoding="utf-8")

    print(f"wrote {REPORT_PATH}")
    print(f"  total: {total}  strict: {totals['strict']}  fuzzy: {totals['fuzzy']}  fail: {totals['fail']}")

    # Also emit a JSON sidecar for machine-readable consumption.
    json_path = REPORT_PATH.with_suffix(".json")
    json_path.write_text(
        json.dumps(
            {
                "total": total,
                "strict": totals["strict"],
                "fuzzy": totals["fuzzy"],
                "fail": totals["fail"],
                "per_card": {
                    k: dict(v) for k, v in per_card.items()
                },
                "per_source": {
                    k: dict(v) for k, v in per_source.items()
                },
            },
            indent=2,
            sort_keys=True,
        ),
        encoding="utf-8",
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
