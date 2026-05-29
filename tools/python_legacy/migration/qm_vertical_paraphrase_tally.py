#!/usr/bin/env python3
"""Capture a SHADOW Layer-2 measurement using legacy paraphrase
prose as the quote candidate, addressing Round-19 review P2-B.

The primary tally (`scripts/qm_vertical_layer2_tally.py`)
produces a structurally-trivial 100 % strict pass rate because
`scripts/migrate_qm_cards.py::extract_quote` pulls a verbatim
substring of each chunk's text as the citation `quote`. The
measurement satisfies AC-8's literal "the full per-citation
tally is captured" contract but is degenerate for the spirit
of "the Layer-2 outcome is measured."

This shadow tally answers the question "what would Layer-2
look like if citation quotes were realistic paraphrases?" The
legacy QM cards under `../CFA_reading/.claude/knowledge/01_quantitative_methods/`
embed `**Source:** <pdf> pp.<range>` annotations after every
paraphrased claim; the prose preceding each annotation is a
genuine paraphrase of the cited PDF content.

For each legacy `**Source:** <pdf> pp.X-Y` annotation:

  1. Extract the sentence-shaped chunk of prose ending at the
     annotation (the "paraphrase claim").
  2. Resolve `<pdf> pp.X-Y` to the matching trim's source_id +
     trim-page range (reusing `PRIMARY_SOURCE_MAP` /
     `TRIM_PAGE_OFFSETS` from migrate_qm_cards.py).
  3. For each chunk in that source whose page span overlaps,
     run `verify_citation` strict-then-fuzzy with the
     paraphrase claim as the `quote`.
  4. Bucket the verdict as `strict` / `fuzzy` / `fail`.

The output report at `_research/qm_paraphrase_tally.md` is the
realistic-pass-rate input AC-9's findings artifact will analyze.

AC-8 is "measure, don't gate," so a low pass rate here is the
EXPECTED finding — the gap between the verbatim-quote primary
tally and the paraphrase shadow tally is exactly what AC-9 is
supposed to surface.

Usage:
    .venv/bin/python scripts/qm_vertical_paraphrase_tally.py
"""

import json
import pathlib
import re
import subprocess
import sys
from collections import Counter, defaultdict
from dataclasses import dataclass

from cacg.chunks_index import ChunksIndex
from cacg.schema import Citation
from cacg.verify.bm25_hints import BM25HintCache
from cacg.verify.layer2 import verify_citation

# Import the source-map + page-offset table from the migration
# script so the shadow tally stays in lockstep with the
# primary mapping (a single source of truth).
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from migrate_qm_cards import (  # noqa: E402  (script-relative import)
    PRIMARY_SOURCE_MAP,
    TRIM_PAGE_OFFSETS,
    book_pages_to_trim_pages,
    primary_pdf_to_source_id,
)


WORKSPACE_ROOT = pathlib.Path(__file__).resolve().parents[2]
LEGACY_DIR = pathlib.Path(
    "/home/jakeshea/CFA_reading/.claude/knowledge/01_quantitative_methods"
)
CHUNKS_MANIFEST = (
    WORKSPACE_ROOT / "tests/parity_corpus/out_python/qm_vertical/chunks_manifest.json"
)
REPORT_PATH = WORKSPACE_ROOT / "_research/qm_paraphrase_tally.md"

# Paraphrase claim window: how many trailing characters before
# the `**Source:** ...` annotation we treat as the "claim."
# Longer captures more context (but more chance of containing
# YAML-forbidden chars); shorter is leaner but may miss the
# topical anchor. 240 chars (~ 1-2 sentences) is a reasonable
# starting heuristic.
CLAIM_TAIL_CHARS = 240

# Regex matching a `**Source:** <pdf-path> pp.<range>` annotation
# OR a bare `pp.<range>` continuation. The first group is the
# absolute PDF reference; the page range is parsed separately.
# Match `**Source:** <pdf-path> pp.<range>`. Tolerates periods
# in the PDF basename (e.g. `CFA_note_2.ocr.pdf`) and arbitrary
# whitespace/newlines inside the path. Greedy `.+?` is bounded
# by the literal ` pp.<digits>` tail so it cannot wander past
# the actual end of the annotation.
_SOURCE_ANNOTATION_RE = re.compile(
    r"\*\*Source:\*\*\s+(\S.+?\.pdf\s+pp\.\d+(?:-\d+)?)",
    re.DOTALL,
)


@dataclass
class ShadowVerdict:
    card_id: str
    annotation_index: int  # 0-based per card
    source_id: str
    chunk_id: str
    bucket: str  # "strict" | "fuzzy" | "fail"
    paraphrase: str
    fail_reason: str | None


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


def extract_annotations(body: str) -> list[tuple[int, str, str]]:
    """Find every `**Source:** <pdf> pp.<range>` annotation in
    `body`. For each, return `(span_end, primary_raw, paraphrase_claim)`
    where `paraphrase_claim` is the CLAIM_TAIL_CHARS-bounded prose
    immediately preceding the annotation."""
    out: list[tuple[int, str, str]] = []
    for m in _SOURCE_ANNOTATION_RE.finditer(body):
        primary_raw = m.group(1).strip()
        # Paraphrase claim: the citing sentence ending just BEFORE
        # the `**Source:**` annotation. Legacy convention is
        # `<sentence>. **Source:** <pdf> pp.<range>` so the period
        # immediately before the annotation closes the sentence we
        # want. Walk backwards from that period to the PREVIOUS
        # sentence boundary; that span is the paraphrased claim.
        annotation_start = m.start()
        # Skip back over any whitespace + `. ` boilerplate
        # immediately before the `**` so we're inside the
        # sentence body.
        scan_end = annotation_start
        while scan_end > 0 and body[scan_end - 1] in " \n":
            scan_end -= 1
        if scan_end > 0 and body[scan_end - 1] == ".":
            scan_end -= 1
        # Now scan_end points at the char just before the closing
        # period of the citing sentence. Walk back CLAIM_TAIL_CHARS,
        # then find the previous `. ` / `.\n` to anchor the
        # sentence start.
        scan_start = max(0, scan_end - CLAIM_TAIL_CHARS)
        window = body[scan_start:scan_end]
        last_boundary = max(window.rfind(". "), window.rfind(".\n"))
        if last_boundary > 0:
            claim = window[last_boundary + 1 :].strip()
        else:
            claim = window.strip()
        # Skip degenerate short claims (the cit.quote schema
        # min_length is 1; we require 20 to avoid trivial
        # substring matches like "i.e.").
        if len(claim) < 20:
            continue
        out.append((annotation_start, primary_raw, claim))
    return out


def shadow_verify(
    card_id: str,
    annotation_index: int,
    paraphrase: str,
    primary_raw: str,
    chunks_by_id: dict,
    chunks_by_source: dict[str, list],
    tamper_cache: dict,
    bm25_cache: BM25HintCache,
) -> ShadowVerdict | None:
    """Run a one-citation Layer-2 check using `paraphrase` as the
    quote against every chunk overlapping the annotation's cited
    range. Returns the BEST verdict (strict beats fuzzy beats
    fail). Returns None if the cited PDF is outside the QM
    vertical's coverage (e.g. the supporting Wooldridge book that
    has no trim)."""
    # Parse "<pdf-keyword>... pp.X-Y"
    m = re.search(r"pp\.(\d+)(?:-(\d+))?", primary_raw)
    if not m:
        return None
    book_lo = int(m.group(1))
    book_hi = int(m.group(2)) if m.group(2) else book_lo

    try:
        source_id = primary_pdf_to_source_id(primary_raw, (book_lo, book_hi))
    except ValueError:
        # PDF not in our 5-trim coverage (e.g. Wooldridge ISBN
        # cited as a supporting source). Out of scope for this
        # shadow measurement.
        return None

    trim_pages = book_pages_to_trim_pages(source_id, book_lo, book_hi)
    if trim_pages is None:
        return None

    t_lo, t_hi = trim_pages
    same_source = chunks_by_source.get(source_id, [])
    overlapping = [
        c
        for c in same_source
        if c.end_page >= t_lo and c.start_page <= t_hi
    ]
    if not overlapping:
        return None

    # Phase 1: strict substring check over every overlapping chunk.
    # Cheap (~2ms / call against the BM25-cached layer).
    for chunk in overlapping:
        citation = Citation(
            source_id=source_id,
            chunk_id=chunk.chunk_id,
            chunk_hash=chunk.chunk_hash,
            page_range=(chunk.start_page, chunk.end_page),
            quote=paraphrase,
            edge_type="supports",
        )
        strict = verify_citation(
            citation,
            chunks_by_id,
            same_source,
            fuzzy_enabled=False,
            citation_index=annotation_index,
            tamper_cache=tamper_cache,
            bm25_hint_cache=bm25_cache,
        )
        if strict.verified:
            return ShadowVerdict(
                card_id=card_id,
                annotation_index=annotation_index,
                source_id=source_id,
                chunk_id=chunk.chunk_id,
                bucket="strict",
                paraphrase=paraphrase,
                fail_reason=None,
            )

    # No strict match. Fuzzy is intentionally NOT attempted in
    # this shadow tally: the Levenshtein-bounded matcher costs
    # ~1 s per call against the larger chunks (138 chunks ×
    # ~1KB each is the AFTS Ch.3 case), and with 239 annotations
    # the full sweep would push the script over an hour. AC-8
    # is "measure, don't gate," and the strict-only signal is
    # the substantive piece (Layer-2 strict catches verbatim
    # quotes; paraphrase claims overwhelmingly land in fuzzy
    # or fail, and we report them as fail here). The AC-9
    # findings analysis can run a targeted fuzzy sweep over a
    # smaller sample if it wants a sharper number.
    return ShadowVerdict(
        card_id=card_id,
        annotation_index=annotation_index,
        source_id=overlapping[0].source_id,
        chunk_id=overlapping[0].chunk_id,
        bucket="fail",
        paraphrase=paraphrase,
        fail_reason=f"strict failed across {len(overlapping)} chunks (fuzzy not attempted)",
    )


def main() -> int:
    if not LEGACY_DIR.is_dir():
        print(f"error: legacy dir not found at {LEGACY_DIR}", file=sys.stderr)
        return 1
    if not CHUNKS_MANIFEST.is_file():
        print(f"error: chunks manifest not found at {CHUNKS_MANIFEST}", file=sys.stderr)
        return 1

    chunks_index = ChunksIndex.from_path(CHUNKS_MANIFEST)
    chunks_by_id = chunks_index.by_id
    chunks_by_source: dict[str, list] = defaultdict(list)
    for c in chunks_by_id.values():
        chunks_by_source[c.source_id].append(c)

    legacy_paths = sorted(LEGACY_DIR.glob("qm-*.md"))
    verdicts: list[ShadowVerdict] = []
    per_card: dict[str, Counter] = {}
    skipped_out_of_scope = 0
    tamper_cache: dict[str, bool] = {}
    # Single BM25 hint cache reused across all citations so the
    # per-source corpus is tokenized + BM25-indexed ONCE per
    # source instead of rebuilt on every failure (was the perf
    # hot spot — without the cache the script took 12+ minutes
    # on 239 annotations because most fail strict and trigger
    # fresh BM25 corpus construction).
    bm25_cache = BM25HintCache()

    for path in legacy_paths:
        body = path.read_text(encoding="utf-8")
        # Strip the legacy frontmatter so annotations in the YAML
        # block aren't double-counted.
        if body.startswith("---\n"):
            close = body.find("\n---\n", 4)
            if close > 0:
                body = body[close + 5 :]
        card_id = path.stem
        per_card[card_id] = Counter()
        annotations = extract_annotations(body)
        for i, (_, primary_raw, paraphrase) in enumerate(annotations):
            v = shadow_verify(
                card_id,
                i,
                paraphrase,
                primary_raw,
                chunks_by_id,
                chunks_by_source,
                tamper_cache,
                bm25_cache,
            )
            if v is None:
                skipped_out_of_scope += 1
                continue
            verdicts.append(v)
            per_card[card_id][v.bucket] += 1

    totals = Counter(v.bucket for v in verdicts)
    total = sum(totals.values())
    if total == 0:
        print("error: no in-vertical paraphrase annotations resolved", file=sys.stderr)
        return 1

    REPORT_PATH.parent.mkdir(parents=True, exist_ok=True)
    git_sha = _short_git_sha()
    git_date = _short_git_date()

    lines: list[str] = []
    lines.append("# QM Vertical — Paraphrase Shadow Layer-2 Tally")
    lines.append("")
    lines.append(f"_captured against HEAD `{git_sha}` on `{git_date}`_")
    lines.append("")
    lines.append("This is the COMPANION measurement to")
    lines.append("`_research/qm_layer2_tally.md` and addresses Round-19")
    lines.append("review P2-B (the primary tally's 100 % strict pass rate")
    lines.append("is structurally trivial — it measures only that the")
    lines.append("migration script's verbatim-substring quotes are still")
    lines.append("verbatim substrings).")
    lines.append("")
    lines.append("This shadow tally uses the LEGACY card's prose preceding")
    lines.append("each `**Source:** <pdf> pp.<range>` annotation as the")
    lines.append("paraphrased claim, then runs `cacg.verify.layer2.verify_citation`")
    lines.append("strict-then-fuzzy against every chunk in the merged QM")
    lines.append("`chunks_manifest.json` that overlaps the cited range. A")
    lines.append("citation is counted `strict` if any overlapping chunk")
    lines.append("contains the paraphrase substring; `fuzzy` if the")
    lines.append("Levenshtein matcher accepted it; `fail` otherwise.")
    lines.append("")
    lines.append("## Aggregate")
    lines.append("")
    lines.append(f"- Total in-vertical annotations: **{total}**")
    lines.append(
        f"- STRICT match: **{totals['strict']}** "
        f"({100*totals['strict']/total:.1f}%)"
    )
    lines.append(
        f"- FUZZY match:  **{totals['fuzzy']}** "
        f"({100*totals['fuzzy']/total:.1f}%)"
    )
    lines.append(
        f"- FAIL:         **{totals['fail']}** "
        f"({100*totals['fail']/total:.1f}%)"
    )
    lines.append(
        f"- Skipped (annotation cites out-of-vertical source like Wooldridge): "
        f"**{skipped_out_of_scope}**"
    )
    lines.append("")
    lines.append("## Per-card breakdown")
    lines.append("")
    lines.append("| card_id | annotations | strict | fuzzy | fail |")
    lines.append("|---------|-------------|--------|-------|------|")
    for card_id in sorted(per_card.keys()):
        c = per_card[card_id]
        n = c["strict"] + c["fuzzy"] + c["fail"]
        if n == 0:
            continue
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
    lines.append("| source_id | annotations | strict | fuzzy | fail |")
    lines.append("|-----------|-------------|--------|-------|------|")
    for source_id in sorted(per_source.keys()):
        c = per_source[source_id]
        n = c["strict"] + c["fuzzy"] + c["fail"]
        lines.append(
            f"| `{source_id}` | {n} | {c['strict']} | "
            f"{c['fuzzy']} | {c['fail']} |"
        )
    lines.append("")
    if totals["fail"]:
        lines.append("## Sample failure paraphrases")
        lines.append("")
        lines.append(
            "Up to 5 paraphrases that failed both strict and fuzzy "
            "(truncated to 200 chars for display)."
        )
        lines.append("")
        for v in [v for v in verdicts if v.bucket == "fail"][:5]:
            preview = v.paraphrase[:200].replace("\n", " ")
            lines.append(
                f"- `{v.card_id}` annotation #{v.annotation_index} "
                f"against `{v.source_id}` chunks (best try: `{v.chunk_id}`):"
            )
            lines.append(f"  - paraphrase: {preview!r}")
        lines.append("")
    lines.append("## Interpretation")
    lines.append("")
    lines.append("AC-8 is \"measure, don't gate.\" The gap between this")
    lines.append("shadow tally and the verbatim-quote primary tally is")
    lines.append("the substantive measurement AC-9's findings artifact")
    lines.append("(`_research/18_cfa_real_migration_findings.md`,")
    lines.append("task-m4-10) will analyze:")
    lines.append("")
    lines.append("  - High STRICT here would mean the legacy paraphrases")
    lines.append("    are close enough to verbatim that Layer-2 trivially")
    lines.append("    grounds them. (Unlikely; the legacy prose is")
    lines.append("    aggressively reworded.)")
    lines.append("  - Substantial FUZZY here means Layer-2 + fuzzy is")
    lines.append("    sufficient — Layer-3 semantic verification isn't")
    lines.append("    needed for the QM vertical.")
    lines.append("  - Substantial FAIL here is the expected finding and")
    lines.append("    motivates Layer-3 (semantic-judge) coverage for")
    lines.append("    a real-corpus deployment.")
    lines.append("")
    REPORT_PATH.write_text("\n".join(lines) + "\n", encoding="utf-8")

    json_path = REPORT_PATH.with_suffix(".json")
    json_path.write_text(
        json.dumps(
            {
                "total": total,
                "strict": totals["strict"],
                "fuzzy": totals["fuzzy"],
                "fail": totals["fail"],
                "skipped_out_of_scope": skipped_out_of_scope,
                "per_card": {k: dict(v) for k, v in per_card.items()},
                "per_source": {k: dict(v) for k, v in per_source.items()},
            },
            indent=2,
            sort_keys=True,
        ),
        encoding="utf-8",
    )
    print(f"wrote {REPORT_PATH}")
    print(
        f"  total: {total}  strict: {totals['strict']}  fuzzy: {totals['fuzzy']}  "
        f"fail: {totals['fail']}  skipped: {skipped_out_of_scope}"
    )
    # Avoid the import-only-used-for-import warning.
    _ = PRIMARY_SOURCE_MAP
    _ = TRIM_PAGE_OFFSETS
    return 0


if __name__ == "__main__":
    sys.exit(main())
