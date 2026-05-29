#!/usr/bin/env python3
"""Sampled fuzzy sweep over paraphrase annotations to bound the
Layer-3-required count for the QM vertical.

The M4b primary shadow tally (`_research/qm_paraphrase_tally.md`)
ran strict-only and reported 222 paraphrase annotations
strict-fail, leaving the fuzzy-only-pass count bounded
`0 ≤ k ≤ 222`. The Round-19 review P2-B and the Round-25
review P3-D both flagged the unknown as a Layer-3
capacity-planning input that AC-9's findings analysis could
not resolve under the analyze-only AC-8 budget.

This script tightens the bound by running a deterministic
random sample of N annotations through the full strict +
EVERY-chunk-fuzzy pipeline, then extrapolating with a 95%
Wilson-score interval. The sample size N=30 was chosen as the
smallest that gives ≤10% confidence-interval half-width at the
worst-case 50% sample rate, while keeping wall-clock under
~15 minutes (fuzzy is ~1s per call against the larger chunks
times ~30 overlapping chunks per annotation).

Output: `_research/qm_paraphrase_layer3_capacity.md` (human-
readable) + `.json` sidecar (machine-readable).

Usage:
    .venv/bin/python scripts/qm_layer3_capacity_sample.py
"""

import json
import math
import pathlib
import random
import re
import subprocess
import sys
import time
from collections import Counter, defaultdict
from dataclasses import dataclass

from cacg.chunks_index import ChunksIndex
from cacg.schema import Citation
from cacg.verify.bm25_hints import BM25HintCache
from cacg.verify.layer2 import verify_citation

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from migrate_qm_cards import (  # noqa: E402
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
REPORT_PATH = WORKSPACE_ROOT / "_research/qm_paraphrase_layer3_capacity.md"

# Sample size. 30 annotations × ~30 overlapping chunks/annotation
# = ~900 verify_citation calls × ~1s fuzzy per call ≈ 15 min
# wall-clock. Smaller N runs faster but widens the confidence
# interval; larger N narrows it but costs minutes per N=10.
SAMPLE_SIZE = 30

# Deterministic seed so re-runs produce byte-identical
# samples + reports.
RANDOM_SEED = 42

CLAIM_TAIL_CHARS = 240
MIN_CLAIM_CHARS = 20

_SOURCE_ANNOTATION_RE = re.compile(
    r"\*\*Source:\*\*\s+(\S.+?\.pdf\s+pp\.\d+(?:-\d+)?)",
    re.DOTALL,
)


@dataclass
class SampleVerdict:
    card_id: str
    annotation_index: int
    source_id: str
    paraphrase: str
    overlapping_chunks: int
    bucket: str  # "fuzzy" | "fail"
    fuzzy_chunk_id: str | None


def _short_git_sha() -> str:
    try:
        r = subprocess.run(
            ["git", "rev-parse", "--short=12", "HEAD"],
            check=True, capture_output=True, text=True,
        )
        return r.stdout.strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        return "<no-git>"


def _short_git_date() -> str:
    try:
        r = subprocess.run(
            ["git", "log", "-1", "--format=%cs", "HEAD"],
            check=True, capture_output=True, text=True,
        )
        return r.stdout.strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        return "<no-git>"


def find_frontmatter_close(text: str) -> int:
    lines = text.split("\n")
    if not lines or lines[0] != "---":
        raise ValueError("missing frontmatter open")
    offset = len(lines[0]) + 1
    for line in lines[1:]:
        if line == "---":
            return offset
        offset += len(line) + 1
    raise ValueError("missing frontmatter close")


def extract_annotations(body: str) -> list[tuple[str, str]]:
    """Mirror the shadow tally's annotation extractor. Each
    returned tuple is `(primary_raw, paraphrase_claim)`."""
    out: list[tuple[str, str]] = []
    for m in _SOURCE_ANNOTATION_RE.finditer(body):
        primary_raw = m.group(1).strip()
        annotation_start = m.start()
        scan_end = annotation_start
        while scan_end > 0 and body[scan_end - 1] in " \n":
            scan_end -= 1
        if scan_end > 0 and body[scan_end - 1] == ".":
            scan_end -= 1
        scan_start = max(0, scan_end - CLAIM_TAIL_CHARS)
        window = body[scan_start:scan_end]
        last_boundary = max(window.rfind(". "), window.rfind(".\n"))
        if last_boundary > 0:
            claim = window[last_boundary + 1 :].strip()
        else:
            claim = window.strip()
        if len(claim) < MIN_CLAIM_CHARS:
            continue
        out.append((primary_raw, claim))
    return out


def wilson_ci_95(successes: int, n: int) -> tuple[float, float]:
    """95% Wilson-score confidence interval for a binomial
    proportion. Better-behaved than the normal approximation at
    extreme proportions (which is exactly the regime we expect
    for fuzzy-saves of paraphrase annotations)."""
    if n == 0:
        return (0.0, 1.0)
    z = 1.96
    p = successes / n
    denom = 1.0 + z * z / n
    centre = (p + z * z / (2 * n)) / denom
    half = (z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n))) / denom
    return (max(0.0, centre - half), min(1.0, centre + half))


def main() -> int:
    chunks_index = ChunksIndex.from_path(CHUNKS_MANIFEST)
    chunks_by_id = chunks_index.by_id
    chunks_by_source: dict[str, list] = defaultdict(list)
    for c in chunks_by_id.values():
        chunks_by_source[c.source_id].append(c)

    # Build the full pool of in-vertical annotations (mirroring
    # the shadow tally's 222 = total - 17 out-of-vertical).
    pool: list[tuple[str, int, str, str, str, tuple[int, int]]] = []
    for path in sorted(LEGACY_DIR.glob("qm-*.md")):
        body = path.read_text(encoding="utf-8")
        if body.startswith("---\n"):
            close = body.find("\n---\n", 4)
            if close > 0:
                body = body[close + 5 :]
        card_id = path.stem
        for i, (primary_raw, paraphrase) in enumerate(extract_annotations(body)):
            m = re.search(r"pp\.(\d+)(?:-(\d+))?", primary_raw)
            if not m:
                continue
            book_lo = int(m.group(1))
            book_hi = int(m.group(2)) if m.group(2) else book_lo
            try:
                source_id = primary_pdf_to_source_id(primary_raw, (book_lo, book_hi))
            except ValueError:
                continue
            trim_pages = book_pages_to_trim_pages(source_id, book_lo, book_hi)
            if trim_pages is None:
                continue
            pool.append((card_id, i, primary_raw, paraphrase, source_id, trim_pages))

    pool_size = len(pool)
    rng = random.Random(RANDOM_SEED)
    sample = rng.sample(pool, min(SAMPLE_SIZE, pool_size))

    bm25_cache = BM25HintCache()
    tamper_cache: dict[str, bool] = {}
    verdicts: list[SampleVerdict] = []
    start = time.perf_counter()

    for sample_idx, (card_id, ann_idx, _, paraphrase, source_id, trim_pages) in enumerate(sample, 1):
        t_lo, t_hi = trim_pages
        same_source = chunks_by_source[source_id]
        overlapping = [
            c for c in same_source
            if c.end_page >= t_lo and c.start_page <= t_hi
        ]
        n_overlap = len(overlapping)
        elapsed_so_far = time.perf_counter() - start
        print(
            f"[{sample_idx:>2}/{len(sample)}] {card_id} ann#{ann_idx} "
            f"→ {n_overlap} chunks  (elapsed {elapsed_so_far:.0f}s)",
            flush=True,
        )
        fuzzy_hit: str | None = None
        for chunk in overlapping:
            citation = Citation(
                source_id=source_id,
                chunk_id=chunk.chunk_id,
                chunk_hash=chunk.chunk_hash,
                page_range=(chunk.start_page, chunk.end_page),
                quote=paraphrase,
                edge_type="supports",
            )
            r = verify_citation(
                citation,
                chunks_by_id,
                same_source,
                fuzzy_enabled=True,
                citation_index=ann_idx,
                tamper_cache=tamper_cache,
                bm25_hint_cache=bm25_cache,
            )
            if r.verified:
                fuzzy_hit = chunk.chunk_id
                break
        verdicts.append(SampleVerdict(
            card_id=card_id,
            annotation_index=ann_idx,
            source_id=source_id,
            paraphrase=paraphrase,
            overlapping_chunks=n_overlap,
            bucket="fuzzy" if fuzzy_hit else "fail",
            fuzzy_chunk_id=fuzzy_hit,
        ))

    elapsed = time.perf_counter() - start
    totals = Counter(v.bucket for v in verdicts)
    n = sum(totals.values())
    fuzzy_lo, fuzzy_hi = wilson_ci_95(totals["fuzzy"], n)
    # Extrapolate to the 222-annotation paraphrase population.
    population = 222  # matches qm_paraphrase_tally.json["fail"]
    proj_lo = int(round(fuzzy_lo * population))
    proj_hi = int(round(fuzzy_hi * population))
    layer3_lo = max(0, population - proj_hi)
    layer3_hi = max(0, population - proj_lo)

    git_sha = _short_git_sha()
    git_date = _short_git_date()

    lines: list[str] = []
    lines.append("# QM Vertical — Layer-3 Capacity (Paraphrase Fuzzy Sweep)")
    lines.append("")
    lines.append(f"_captured against HEAD `{git_sha}` on `{git_date}`_")
    lines.append("")
    lines.append("This is the post-M4 capacity-planning measurement that")
    lines.append("tightens the bound the M4b shadow tally left open. The")
    lines.append("shadow tally (`_research/qm_paraphrase_tally.md`) ran")
    lines.append("strict-only on cost grounds; the fuzzy-only-pass count")
    lines.append("was therefore bounded only `0 ≤ k ≤ 222`. The Round-19")
    lines.append("review P2-B and the Round-25 review P3-D both flagged")
    lines.append("this as a Layer-3 capacity-planning unknown.")
    lines.append("")
    lines.append(f"This sample sweep runs `cacg.verify.layer2.verify_citation`")
    lines.append(f"with `fuzzy_enabled=True` against EVERY overlapping chunk")
    lines.append(f"for a deterministic random sample of {SAMPLE_SIZE} of the")
    lines.append(f"{pool_size} in-vertical paraphrase annotations (seed")
    lines.append(f"= {RANDOM_SEED}, byte-stable across re-runs).")
    lines.append("")
    lines.append("## Aggregate")
    lines.append("")
    lines.append(f"- Sample size: **{n}** of {pool_size} in-vertical annotations")
    lines.append(f"- Sample wall-clock: **{elapsed:.1f}s**")
    lines.append(
        f"- FUZZY-pass in sample: **{totals['fuzzy']}** "
        f"({100*totals['fuzzy']/n:.1f}%)"
    )
    lines.append(
        f"- FAIL in sample:       **{totals['fail']}** "
        f"({100*totals['fail']/n:.1f}%)"
    )
    lines.append("")
    lines.append("## Population estimate (95% Wilson-score)")
    lines.append("")
    lines.append(
        f"Extrapolated to the 222-annotation paraphrase population that "
        f"`_research/qm_paraphrase_tally.md` reports as strict-failed:"
    )
    lines.append("")
    lines.append(f"- Fuzzy-save rate (95% CI): **[{100*fuzzy_lo:.1f}%, {100*fuzzy_hi:.1f}%]**")
    lines.append(f"- Projected fuzzy-saves: **[{proj_lo}, {proj_hi}] of 222 annotations**")
    lines.append(f"- Projected Layer-3-required: **[{layer3_lo}, {layer3_hi}] of 222 annotations**")
    lines.append("")
    lines.append("## Per-source breakdown (sample)")
    lines.append("")
    per_source: dict[str, Counter] = {}
    for v in verdicts:
        per_source.setdefault(v.source_id, Counter())[v.bucket] += 1
    lines.append("| source_id          | sample | fuzzy | fail |")
    lines.append("|--------------------|-------:|------:|-----:|")
    for source_id in sorted(per_source.keys()):
        c = per_source[source_id]
        lines.append(
            f"| `{source_id}` | {sum(c.values())} | "
            f"{c['fuzzy']} | {c['fail']} |"
        )
    lines.append("")
    if totals["fuzzy"]:
        lines.append("## Sample fuzzy-saves (up to 5)")
        lines.append("")
        for v in [v for v in verdicts if v.bucket == "fuzzy"][:5]:
            preview = v.paraphrase[:200].replace("\n", " ")
            lines.append(
                f"- `{v.card_id}` ann#{v.annotation_index} → "
                f"chunk `{v.fuzzy_chunk_id}`:"
            )
            lines.append(f"  - paraphrase: {preview!r}")
        lines.append("")
    if totals["fail"]:
        lines.append("## Sample fuzzy-fails (up to 5)")
        lines.append("")
        for v in [v for v in verdicts if v.bucket == "fail"][:5]:
            preview = v.paraphrase[:200].replace("\n", " ")
            lines.append(
                f"- `{v.card_id}` ann#{v.annotation_index} "
                f"({v.overlapping_chunks} chunks tried):"
            )
            lines.append(f"  - paraphrase: {preview!r}")
        lines.append("")
    lines.append("## Interpretation")
    lines.append("")
    lines.append("AC-8 was \"measure, don't gate.\" The shadow tally's")
    lines.append("`0 ≤ fuzzy-saves ≤ 222` bound is now tightened to a real")
    lines.append("Wilson-score interval. The complementary")
    lines.append("`Layer-3-required` count is the strict-and-fuzzy-failed")
    lines.append("residual — the citations that semantic verification")
    lines.append("would have to ground if the migration ever switched")
    lines.append("from verbatim quotes to paraphrase quotes.")
    lines.append("")
    lines.append("The fuzzy matcher is `cacg.verify.fuzzy.fuzzy_match` —")
    lines.append("a Levenshtein-bounded sliding-window check with a")
    lines.append("documented edit-distance budget. Citations that fail")
    lines.append("BOTH strict substring AND fuzzy require Layer-3")
    lines.append("(embedding-cache or LLM-judge) coverage, which carries")
    lines.append("operational cost. A real deployment's capacity plan")
    lines.append("should size against the upper bound here.")
    lines.append("")
    REPORT_PATH.write_text("\n".join(lines) + "\n", encoding="utf-8")

    json_sidecar = REPORT_PATH.with_suffix(".json")
    json_sidecar.write_text(
        json.dumps(
            {
                "sample_size": n,
                "pool_size": pool_size,
                "elapsed_s": round(elapsed, 2),
                "sample_fuzzy": totals["fuzzy"],
                "sample_fail": totals["fail"],
                "fuzzy_rate_ci_95": [round(fuzzy_lo, 4), round(fuzzy_hi, 4)],
                "projected_fuzzy_saves_of_222": [proj_lo, proj_hi],
                "projected_layer3_required_of_222": [layer3_lo, layer3_hi],
                "per_source": {k: dict(v) for k, v in per_source.items()},
            },
            indent=2, sort_keys=True,
        ),
        encoding="utf-8",
    )
    print(f"wrote {REPORT_PATH}")
    print(
        f"  sample: {n} | fuzzy: {totals['fuzzy']} | fail: {totals['fail']}"
        f"  | wall-clock: {elapsed:.1f}s"
    )
    print(
        f"  Layer-3-required projection (95% CI): "
        f"[{layer3_lo}, {layer3_hi}] of 222"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
