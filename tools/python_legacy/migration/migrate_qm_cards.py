#!/usr/bin/env python3
"""Migrate the 17 legacy `01_quantitative_methods/` cards from the
sibling CFA_reading repo into cacg.v0 schema cards.

Source: `/home/jakeshea/CFA_reading/.claude/knowledge/01_quantitative_methods/qm-*.md`.
Each legacy card carries a natural-prose body with `**Source:**
<pdf> pp.<range>` annotations and a YAML frontmatter block with
`Use when:` / `Primary raw source:` / `Supporting sources:` /
`Repo touchpoints:` / `edges:` / etc.

This script:

  1. Walks the 17 legacy cards.
  2. Parses the legacy frontmatter (`Use when` → summary,
     `Primary raw source` → primary source + page range,
     filename → id, first `#` heading → title).
  3. Maps primary source PDF to the QM-vertical `source_id`
     (one of `qm_notes_trim` / `qm_greene_trim` /
     `qm_afts_trim` / `qm_eslii_trim`).
  4. Translates legacy book-page numbers into trim-PDF page
     space via `TRIM_PAGE_OFFSETS`.
  5. Finds EVERY chunk in the merged QM `chunks_manifest.json`
     whose `source_id` + page span lies fully inside the trim
     range and emits ONE citation per chunk — the cacg.v0
     contract requires a multi-page `pp.N-M` span to fan out
     into one Citation per overlapped chunk. The chunk-fully-
     inside-trim filter avoids selecting chunks that straddle
     the artificial trim-seam boundaries (notably the ESLII
     trim's Ch.3-to-Ch.7 seam at trim page 57/58).
  6. Maps legacy card-to-card references (the `edges:` /
     `Repo touchpoints:` / `## See Also` surfaces) into the
     cacg.v0 `card_edges` list, restricted to the 17 in-vertical
     card ids and deduplicated by `(target, edge_type)`.
  7. Writes the card WITHOUT a `card_hash` field; `kb index`
     is the authoritative hash-computation surface and will
     fill it in. Pre-computing the hash in the migration
     script is dead code — kb index rewrites it on every
     index run anyway — so the migration's output is now
     unambiguously a deterministic predecessor of the indexed
     state.

Determinism: re-running on the same legacy sources + the same
merged manifest produces byte-identical card files. The card
files have NO card_hash; running `kb index` against the output
fills in the canonical hash and emits the cards_manifest. The
cleanly-separated "migration emits cards" + "kb index seals
them" flow keeps the determinism contract honest.

Usage:
    .venv/bin/python scripts/migrate_qm_cards.py
"""

import json
import pathlib
import re
import sys
from dataclasses import dataclass


WORKSPACE_ROOT = pathlib.Path(__file__).resolve().parents[2]
LEGACY_DIR = pathlib.Path(
    "/home/jakeshea/CFA_reading/.claude/knowledge/01_quantitative_methods"
)
OUTPUT_DIR = WORKSPACE_ROOT / "tests/parity_corpus/cards/reading_01_qm"
CHUNKS_MANIFEST = (
    WORKSPACE_ROOT / "tests/parity_corpus/out_python/qm_vertical/chunks_manifest.json"
)
READING_ID = "reading_01_qm"

# Quote bounds for the citation quote-extraction. Min protects
# against trivial fragments; max stays inside the Layer-2
# substring-match window.
MIN_QUOTE_CHARS = 60
MAX_QUOTE_CHARS = 120

# Characters that PyYAML's safe-load rejects in scalar values
# (YAML 1.1 "printable character" minus tab/LF/CR which YAML
# DOES allow). Any quote containing one of these triggers
# CACG-FM-006 at parse time. Pdfium-extracted OCR text frequently
# carries U+0001 (SOH), U+0002 (STX) → U+FFFE soft-hyphen marker,
# and other C0 / C1 / non-character codepoints; the quote
# extractor must skip over any run containing these.
_FORBIDDEN_QUOTE_CHARS = set()
for cp in list(range(0x00, 0x09)) + [0x0B, 0x0C] + list(range(0x0E, 0x20)):
    _FORBIDDEN_QUOTE_CHARS.add(chr(cp))  # C0 minus tab/LF/CR
_FORBIDDEN_QUOTE_CHARS.add("\x7F")  # DEL
for cp in range(0x80, 0xA0):
    _FORBIDDEN_QUOTE_CHARS.add(chr(cp))  # C1 controls
_FORBIDDEN_QUOTE_CHARS.add("￾")  # non-character
_FORBIDDEN_QUOTE_CHARS.add("￿")  # non-character


def _next_forbidden(s: str, start: int) -> int:
    """Index of next forbidden char in `s` at or after `start`,
    or `len(s)` if none."""
    for i in range(start, len(s)):
        if s[i] in _FORBIDDEN_QUOTE_CHARS:
            return i
    return len(s)

# Primary source PDF → cacg.v0 source_id. The legacy
# `Primary raw source:` line is matched against the substring
# keys; the value is the source_id in the merged QM chunks_manifest.
# ESLII appears twice: book pages 43-99 (Ch.3) → qm_eslii_ch3_trim,
# book pages 219-260 (Ch.7) → qm_eslii_ch7_trim. Splitting ESLII
# into two trims (instead of one trim with an artificial Ch.3-to-Ch.7
# seam) avoids the seam-straddle problem where one chunker chunk
# would concatenate text from two disjoint book-page ranges.
PRIMARY_SOURCE_MAP: list[tuple[str, str, tuple[int, int] | None]] = [
    # (substring, source_id, optional (book_lo, book_hi) range guard)
    ("notes/CFA_note_2.ocr.pdf", "qm_notes_trim", None),
    ("William H. Greene", "qm_greene_trim", None),
    ("Analysis of Financial Time Series.pdf", "qm_afts_trim", None),
    ("ESLII_print12_toc.pdf", "qm_eslii_ch3_trim", (43, 99)),
    ("ESLII_print12_toc.pdf", "qm_eslii_ch7_trim", (219, 260)),
]

# Book-page to trim-page offsets per source_id. Each tuple is
# `(book_page_start, book_page_end_inclusive, trim_page_start)`.
# The trim PDFs are built by `scripts/build_qm_trim_fixtures.py`;
# the offsets reflect the source-PDF-vs-book-page offsets:
#   - notes: hand-written, book pages == PDF pages.
#   - greene: 1 page of front-matter, book = PDF - 1.
#   - afts:   25 pages of front-matter, book = PDF - 25.
#   - eslii:  19 pages of front-matter, book = PDF - 19.
# The trim re-numbers covered source PDF pages 1..N, so for
# greene's trim of book pp.413-470 (source PDF pp.414-471),
# trim page 1 = book page 413; (413, 470, 1) is the mapping.
TRIM_PAGE_OFFSETS: dict[str, list[tuple[int, int, int]]] = {
    "qm_notes_trim": [(1, 20, 1)],
    "qm_greene_trim": [(413, 470, 1)],
    "qm_afts_trim": [(97, 200, 1)],
    "qm_eslii_ch3_trim": [(43, 99, 1)],
    "qm_eslii_ch7_trim": [(219, 260, 1)],
}


@dataclass
class LegacyCard:
    """Parsed legacy QM card."""

    id: str
    title: str
    summary: str
    primary_pdf: str
    primary_pages: tuple[int, int]  # inclusive (lo, hi) book pages
    tags: list[str]  # derived lowercase-slug tags
    edges: list[str]  # raw legacy `edges:` entries
    repo_touchpoints: list[str]  # raw legacy `Repo touchpoints:` entries
    see_also: list[str]  # paths extracted from a `## See Also` body block
    body: str


def find_frontmatter_close(text: str) -> int:
    """Return the byte offset of the SECOND `^---$` line in `text`,
    treating the first as the frontmatter open. Defensive scan that
    will not be confused by a `\\n---\\n` HR rule appearing later in
    the body."""
    lines = text.split("\n")
    if not lines or lines[0] != "---":
        raise ValueError("missing frontmatter open '---' on the first line")
    offset = len(lines[0]) + 1  # past the opening '---\n'
    for line in lines[1:]:
        if line == "---":
            return offset
        offset += len(line) + 1
    raise ValueError("missing frontmatter close '---' line")


def parse_legacy_frontmatter(text: str) -> dict:
    """Parse the legacy YAML-ish frontmatter block. Each line is
    `key: value` (lists are indented sub-blocks)."""
    close = find_frontmatter_close(text)
    block = text[4:close]
    out: dict[str, object] = {}
    current_list: list[str] | None = None
    for line in block.splitlines():
        if line.startswith("  - "):
            if current_list is None:
                raise ValueError(f"orphan list item: {line!r}")
            current_list.append(line[4:])
            continue
        if line.startswith(("  ", "\t")):
            # Continuation line — ignored (legacy cards never use it).
            continue
        if ":" not in line:
            continue
        key, _, value = line.partition(":")
        key = key.strip()
        value = value.strip()
        if value:
            out[key] = value
            current_list = None
        else:
            current_list = []
            out[key] = current_list
    return out


def extract_pages_one_indexed(spec: str) -> tuple[int, int]:
    """Parse a `pp.X[-Y]` page range. Returns (lo, hi) inclusive."""
    m = re.search(r"pp\.(\d+)(?:-(\d+))?", spec)
    if not m:
        raise ValueError(f"unrecognized page spec: {spec!r}")
    lo = int(m.group(1))
    hi = int(m.group(2)) if m.group(2) else lo
    return (lo, hi)


def primary_pdf_to_source_id(
    primary_raw: str, book_pages: tuple[int, int]
) -> str:
    """Map the legacy `Primary raw source:` value to a cacg.v0
    `source_id`. For sources split across multiple trims (ESLII),
    use the cited book-page range to disambiguate."""
    book_lo, book_hi = book_pages
    for substring, source_id, guard in PRIMARY_SOURCE_MAP:
        if substring not in primary_raw:
            continue
        if guard is None:
            return source_id
        g_lo, g_hi = guard
        if book_lo >= g_lo and book_hi <= g_hi:
            return source_id
    raise ValueError(
        f"no source_id mapping for primary source {primary_raw!r} "
        f"with cited book pages {book_pages}"
    )


def book_pages_to_trim_pages(
    source_id: str, book_lo: int, book_hi: int
) -> tuple[int, int] | None:
    """Translate `(book_lo, book_hi)` legacy book-page numbers into
    the trim-PDF's 1-indexed page numbers. Returns None if the
    legacy range falls outside any of the trim's covered ranges."""
    for b_lo, b_hi, t_lo in TRIM_PAGE_OFFSETS[source_id]:
        if book_lo >= b_lo and book_hi <= b_hi:
            trim_lo = t_lo + (book_lo - b_lo)
            trim_hi = t_lo + (book_hi - b_lo)
            return (trim_lo, trim_hi)
    return None


def extract_see_also_targets(body: str) -> list[str]:
    """Extract the raw target strings under a body `## See Also`
    block. Each target line is `- <path-or-id> -- <description>`
    or `- [text](path)`; we return the raw post-`- ` substring."""
    targets: list[str] = []
    in_section = False
    for line in body.splitlines():
        if line.startswith("## "):
            in_section = line.strip() == "## See Also"
            continue
        if not in_section:
            continue
        stripped = line.strip()
        if stripped.startswith("- "):
            targets.append(stripped[2:].strip())
    return targets


# Topic-keyword tags derived from the card id. Each entry is
# (substring-in-id, tag). Order doesn't matter — duplicates are
# deduped + sorted in `derive_tags`. The schema requires
# lowercase-slug-shaped tags (`^[a-z0-9][a-z0-9-]*$`); these all
# comply. Capped at 10 per card by the schema; the derivation
# typically produces 2-4 per card so the cap is not load-bearing.
_KEYWORD_TAGS: list[tuple[str, str]] = [
    ("regression", "regression"),
    ("anova", "anova"),
    ("goodness-of-fit", "goodness-of-fit"),
    ("hypothesis-tests", "hypothesis-testing"),
    ("assumption-violations", "regression-diagnostics"),
    ("influence-analysis", "regression-diagnostics"),
    ("aic-bic", "model-selection"),
    ("penalized", "shrinkage"),
    ("lasso", "shrinkage"),
    ("projection", "dimensionality-reduction"),
    ("dimensionality-reduction", "dimensionality-reduction"),
    ("decision-trees", "decision-trees"),
    ("structured-data-ml", "supervised-learning"),
    ("time-series", "time-series"),
    ("arch", "garch"),
    # `garch` is omitted: the substring `garch` always contains
    # `arch`, so the `arch → garch` entry above already fires
    # on every card carrying it. M4-Round-23 review P3 noted
    # the redundancy.
    ("conditional-heteroskedasticity", "garch"),
    ("volatility", "garch"),
    ("multivariate", "multivariate"),
    ("multiple-linear-regression", "regression"),
    ("panel", "panel-data"),
    ("factor", "factor-models"),
    ("signal-validation", "cross-validation"),
    ("oos-discipline", "cross-validation"),
    # `qm-cb-arb-factor-construction` is about ridge / lasso /
    # boosting on convertible-bond-arbitrage factors; the
    # `cb-arb → shrinkage` mapping reflects the shrinkage-as-
    # regularization content rather than the convertible-bond
    # domain. Documented here so a future reader doesn't
    # mistake it for a typo.
    ("cb-arb", "shrinkage"),
]


def derive_tags(card_id: str, fm: dict) -> list[str]:
    """Derive cacg.v0-schema-compliant tags from the legacy
    `card_id` + frontmatter fields. The schema requires lowercase
    slug tags (`^[a-z0-9][a-z0-9-]*$`) and at most 10. We derive
    2-4 typically; output is sorted + deduped for byte-stability."""
    tags: set[str] = set()
    # Topic tags from card-id keywords.
    for substring, tag in _KEYWORD_TAGS:
        if substring in card_id:
            tags.add(tag)
    # Domain tag from `Node type:`.
    node_type = str(fm.get("Node type", "")).strip().lower()
    if node_type:
        # Strip non-slug chars; schema regex is strict.
        slug = re.sub(r"[^a-z0-9-]+", "-", node_type).strip("-")
        if slug:
            tags.add(slug)
    # Cap defensively (the schema enforces ≤ 10).
    out = sorted(tags)
    return out[:10]


def parse_legacy_card(path: pathlib.Path) -> LegacyCard:
    text = path.read_text(encoding="utf-8")
    fm = parse_legacy_frontmatter(text)
    body_start = find_frontmatter_close(text) + 4  # past "\n---\n"
    body = text[body_start:].strip()

    card_id = path.stem

    title_match = re.search(r"^# (.+)$", body, flags=re.MULTILINE)
    if title_match:
        title = title_match.group(1).strip()
    else:
        title = card_id.replace("-", " ").title()

    use_when = str(fm.get("Use when", "")).strip()
    if not use_when:
        use_when = title
    # Hard schema cap is 400; trim defensively at 380.
    if len(use_when) > 380:
        use_when = use_when[:377] + "..."

    primary_raw = str(fm.get("Primary raw source", "")).strip()
    if not primary_raw:
        raise ValueError(f"{path.name}: missing Primary raw source")
    pages = extract_pages_one_indexed(primary_raw)

    edges_raw = fm.get("edges") or []
    repo_touchpoints_raw = fm.get("Repo touchpoints") or []
    edges_list = [e for e in edges_raw if isinstance(e, str)]
    rt_list = [r for r in repo_touchpoints_raw if isinstance(r, str)]
    see_also = extract_see_also_targets(body)
    tags = derive_tags(card_id, fm)

    return LegacyCard(
        id=card_id,
        title=title,
        summary=use_when,
        primary_pdf=primary_raw,
        primary_pages=pages,
        tags=tags,
        edges=edges_list,
        repo_touchpoints=rt_list,
        see_also=see_also,
        body=body,
    )


def find_all_overlapping_chunks(
    chunks: list[dict], source_id: str, trim_pages: tuple[int, int]
) -> list[dict]:
    """Return EVERY chunk from `source_id` whose page span overlaps
    `trim_pages`. A chunk with `[start_page, end_page]` overlaps
    `[t_lo, t_hi]` iff `chunk.end_page >= t_lo AND chunk.start_page
    <= t_hi`. The ESLII Ch.3-to-Ch.7 seam-straddle problem is
    avoided structurally by publishing two separate ESLII trims
    (`qm_eslii_ch3_trim`, `qm_eslii_ch7_trim`) rather than relying
    on the chunk-overlap filter to skip the straddle chunk."""
    t_lo, t_hi = trim_pages
    out: list[dict] = []
    for chunk in chunks:
        if chunk["source_id"] != source_id:
            continue
        if chunk["end_page"] >= t_lo and chunk["start_page"] <= t_hi:
            out.append(chunk)
    return out


def extract_quote(chunk_text: str) -> str:
    """Extract a representative quote from the chunk text. Pick
    a sentence-ish fragment between MIN_QUOTE_CHARS and
    MAX_QUOTE_CHARS that does NOT contain U+FFFE soft-hyphen
    markers — Pdfium emits U+FFFE for soft hyphens (preserved in
    the chunk's text for round-trip fidelity), but YAML 1.1's
    safe-load forbids non-character codepoints in scalars, so a
    quote containing U+FFFE breaks the card's CACG-FM-006 parse.
    Layer-2 substring grounding stays valid because the chosen
    quote is still a verbatim substring of the chunk's text."""
    stripped = chunk_text.strip()
    n = len(stripped)
    start = 0
    while start < n:
        run_end = _next_forbidden(stripped, start)
        run_len = run_end - start
        if run_len >= MIN_QUOTE_CHARS:
            # Cap the window at the smaller of MAX_QUOTE_CHARS and
            # the forbidden-char-free run length so the quote NEVER
            # crosses into the next bad codepoint.
            window_len = min(MAX_QUOTE_CHARS, run_len)
            window = stripped[start : start + window_len]
            cut = window.rfind(".", MIN_QUOTE_CHARS, window_len)
            if cut == -1:
                cut = window.rfind(" ", MIN_QUOTE_CHARS, window_len)
            if cut == -1:
                cut = window_len
            return stripped[start : start + cut].strip()
        if run_end >= n:
            break
        start = run_end + 1
    # Fallback: pathological chunk is forbidden-char-laden
    # throughout. Strip them all and return the first
    # MAX_QUOTE_CHARS chars.
    cleaned = "".join(c for c in stripped if c not in _FORBIDDEN_QUOTE_CHARS)
    return cleaned[:MAX_QUOTE_CHARS].strip()


def build_card_edges(
    legacy: LegacyCard, in_vertical_ids: set[str]
) -> list[dict]:
    """Map legacy card-to-card references into cacg.v0
    `card_edges`. The schema's `CardEdge.edge_type` is restricted
    to `Literal["depends_on", "extends"]` and the field name is
    `target` (not `target_id`). Sources we map:
      - `edges:` frontmatter — entries like
        `applies_to:.claude/knowledge/05_equity/eq-foo.md`. The
        legacy `applies_to` is not a schema edge_type, so all
        legacy `edges:` entries are coerced to `extends` (the
        semantically-loosest schema-allowed value).
      - `Repo touchpoints:` frontmatter — entries are bare paths
        treated as soft dependencies → `extends`.
      - `## See Also` body — entries are arbitrary text; we look
        for `qm-*` substrings as target ids → `extends`.
    Targets are restricted to in-vertical card ids and
    deduplicated by `(target, edge_type)`. The card's own id is
    excluded so a self-citation never produces an edge."""
    seen: set[tuple[str, str]] = set()
    out: list[dict] = []

    def add(target: str, edge_type: str) -> None:
        if target == legacy.id:
            return
        if target not in in_vertical_ids:
            return
        # Break cycles deterministically: emit the edge only when
        # the source id sorts BEFORE the target id alphabetically.
        # The reverse-direction edge would be emitted by the OTHER
        # card's migration if its See Also lists this card too;
        # we drop it to keep the graph acyclic. The original
        # mutual reference is preserved verbatim in each card's
        # `## Original Card (preserved verbatim)` body section.
        if legacy.id >= target:
            return
        key = (target, edge_type)
        if key in seen:
            return
        seen.add(key)
        out.append({"target": target, "edge_type": edge_type})

    for entry in legacy.edges:
        _, _, target_path = entry.partition(":")
        target_id = pathlib.PurePosixPath(target_path.strip()).stem
        add(target_id, "extends")

    for entry in legacy.repo_touchpoints:
        target_id = pathlib.PurePosixPath(entry.strip()).stem
        add(target_id, "extends")

    for entry in legacy.see_also:
        for m in re.findall(r"qm-[a-z0-9-]+", entry):
            add(m, "extends")

    out.sort(key=lambda d: (d["target"], d["edge_type"]))
    return out


def escape_yaml(s: str) -> str:
    """Escape a string for a double-quoted YAML scalar."""
    return s.replace("\\", "\\\\").replace('"', '\\"')


def serialize_card(legacy: LegacyCard, chunks: list[dict], in_vertical_ids: set[str]) -> str:
    source_id = primary_pdf_to_source_id(legacy.primary_pdf, legacy.primary_pages)
    trim_pages = book_pages_to_trim_pages(
        source_id, legacy.primary_pages[0], legacy.primary_pages[1]
    )
    if trim_pages is None:
        raise ValueError(
            f"{legacy.id}: primary book pages {legacy.primary_pages} are outside "
            f"the {source_id} trim's covered ranges"
        )
    overlapping = find_all_overlapping_chunks(chunks, source_id, trim_pages)
    if not overlapping:
        raise ValueError(
            f"{legacy.id}: no chunks fully inside trim pages {trim_pages} of {source_id}"
        )

    citations: list[dict] = []
    for chunk in overlapping:
        citations.append({
            "source_id": source_id,
            "chunk_id": chunk["chunk_id"],
            "chunk_hash": chunk["chunk_hash"],
            "page_range": [chunk["start_page"], chunk["end_page"]],
            "quote": extract_quote(chunk["text"]),
            "edge_type": "supports",
        })

    card_edges = build_card_edges(legacy, in_vertical_ids)

    # Body: short intent line + the legacy prose embedded so the
    # engineered content is preserved verbatim for downstream
    # verifier work. Strip the legacy body's leading `# <title>`
    # line so we don't double-render the H1.
    body_inner = re.sub(r"^# .*\n+", "", legacy.body, count=1)
    body = (
        f"{legacy.summary}\n\n"
        f"## Original Card (preserved verbatim)\n\n"
        f"{body_inner}\n"
    )

    out = ["---"]
    out.append(f'schema_version: "cacg.v0"')
    out.append(f'id: "{legacy.id}"')
    out.append(f'title: "{escape_yaml(legacy.title)}"')
    out.append(f'reading_id: "{READING_ID}"')
    out.append(f'summary: "{escape_yaml(legacy.summary)}"')
    if legacy.tags:
        out.append("tags:")
        for t in legacy.tags:
            out.append(f'  - "{t}"')
    out.append("citations:")
    for cit in citations:
        out.append(f'  - source_id: "{cit["source_id"]}"')
        out.append(f'    chunk_id: "{cit["chunk_id"]}"')
        out.append(f'    chunk_hash: "{cit["chunk_hash"]}"')
        out.append(f'    page_range: [{cit["page_range"][0]}, {cit["page_range"][1]}]')
        out.append(f'    quote: "{escape_yaml(cit["quote"])}"')
        out.append(f'    edge_type: "{cit["edge_type"]}"')
    if card_edges:
        out.append("card_edges:")
        for ed in card_edges:
            out.append(f'  - target: "{ed["target"]}"')
            out.append(f'    edge_type: "{ed["edge_type"]}"')
    out.append("---")
    out.append(body)
    return "\n".join(out)


def main() -> int:
    if not LEGACY_DIR.is_dir():
        print(f"error: legacy QM dir not found at {LEGACY_DIR}", file=sys.stderr)
        return 1
    if not CHUNKS_MANIFEST.is_file():
        print(
            f"error: merged QM chunks_manifest not found at {CHUNKS_MANIFEST}; "
            "run scripts/build_qm_vertical_corpus.py first",
            file=sys.stderr,
        )
        return 1

    chunks_doc = json.loads(CHUNKS_MANIFEST.read_text(encoding="utf-8"))
    chunks = chunks_doc["chunks"]

    legacy_paths = sorted(LEGACY_DIR.glob("qm-*.md"))
    if not legacy_paths:
        print(f"error: no qm-*.md cards found under {LEGACY_DIR}", file=sys.stderr)
        return 1
    in_vertical_ids = {p.stem for p in legacy_paths}

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    # Wipe stale cards AND stale history sidecars so a renamed-then-
    # removed legacy card stops appearing here.
    for stale in OUTPUT_DIR.glob("qm-*"):
        if stale.is_file():
            stale.unlink()

    written = 0
    total_citations = 0
    total_edges = 0
    for legacy_path in legacy_paths:
        try:
            legacy = parse_legacy_card(legacy_path)
            card_text = serialize_card(legacy, chunks, in_vertical_ids)
        except Exception as exc:
            print(f"error processing {legacy_path.name}: {exc}", file=sys.stderr)
            return 1
        out_path = OUTPUT_DIR / f"{legacy.id}.md"
        out_path.write_text(card_text, encoding="utf-8")
        written += 1
        # Pull the citation count back out of the written file for
        # the summary line (cheap re-parse via regex).
        total_citations += card_text.count("\n  - source_id:")
        total_edges += card_text.count("\n  - target:")

    print(
        f"migrated {written} cards into {OUTPUT_DIR}: "
        f"{total_citations} citations, {total_edges} card_edges"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
