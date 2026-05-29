"""Dev-only generator for tests/fixtures/sample.pdf.

Run from project root:
    .venv/bin/python tests/fixtures/build_sample_pdf.py

The output is a deterministic 3-page PDF whose text exercises the
normalization pipeline (NFD characters, hyphenated line breaks, common
ligatures). The output file is committed; this script only needs to be re-run
when the fixture intent changes. fpdf2 produces byte-stable output for the
same input on the same fpdf2 version, which is acceptable: we re-pin in CI.
"""
from __future__ import annotations

from datetime import datetime, timezone
from pathlib import Path

from fpdf import FPDF

FIXED_TIMESTAMP = datetime(1970, 1, 1, 0, 0, 0, tzinfo=timezone.utc)

OUT_PATH = Path(__file__).resolve().parents[3] / "tests" / "fixtures" / "sample.pdf"

PAGE_1_PARAGRAPHS = [
    "Content-addressable identity is the verification primitive at the core of "
    "the framework. Every chunk hashes to a stable 64-hex digest, and every "
    "citation pins exactly one chunk by hash. Drift becomes a mechanical "
    "failure, not a semantic argument.",
    "Hash pinning protects against silent text drift. When a source PDF is "
    "re-issued with a new typesetting pass, page numbers may shift while the "
    "underlying argument stays the same. The hash mismatch surfaces this on "
    "the next lint run.",
    "The verifier is not BM25. BM25 is a retrieval helper that proposes the "
    "three most plausible chunks when a quoted phrase fails containment. The "
    "decision oracle is normalized exact substring containment against the "
    "pinned chunk.",
]

PAGE_2_PARAGRAPHS = [
    "Context engineering trades raw recall for steerable attention. A small "
    "card index is loaded eagerly into the agent's working set. The full "
    "body of each card stays on disk until a specific lookup demands it. "
    "Progressive disclosure is the default.",
    "Subagents amplify isolation. Each per-reading subagent sees only its own "
    "chunks, schema, and style guide. The orchestrator never inherits the "
    "noise of upstream exploration. Tens of thousands of tokens compress to "
    "one focused result.",
    "Round contracts pin the unit of work. One mainline objective per round "
    "and one or two target acceptance criteria keep the loop honest. "
    "Side issues are queued, not promoted, unless they truly block.",
]

PAGE_3_PARAGRAPHS = [
    "Determinism is a kept promise. JSON output sorts keys and uses tight "
    "separators. Frontmatter writers emit fixed key orders. Timestamps and "
    "UUIDs collapse to zero values when KB_FROZEN_CLOCK is set, so artifacts "
    "diff cleanly across runs.",
    "Atomic publish removes the partial-write failure mode. Each manifest is "
    "written to a temp path, validated by Pydantic round-trip, and only then "
    "renamed into the canonical path. A crash mid-write leaves the prior "
    "canonical bytes untouched.",
    "Append-only history per card records what changed and why. Each entry "
    "carries the previous and next card hash plus the delta of cited chunks. "
    "Edits become an auditable timeline, not a git archaeology exercise.",
]


def build() -> None:
    pdf = FPDF()
    pdf.set_creation_date(FIXED_TIMESTAMP)  # deterministic byte output
    pdf.set_auto_page_break(auto=True, margin=15)

    for page_paragraphs in (PAGE_1_PARAGRAPHS, PAGE_2_PARAGRAPHS, PAGE_3_PARAGRAPHS):
        pdf.add_page()
        pdf.set_font("Helvetica", size=11)
        for para in page_paragraphs:
            pdf.multi_cell(0, 6, para)
            pdf.ln(2)

    pdf.output(str(OUT_PATH))
    print(f"wrote {OUT_PATH} ({OUT_PATH.stat().st_size} bytes)")


if __name__ == "__main__":
    build()
