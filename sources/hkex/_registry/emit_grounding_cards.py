#!/usr/bin/env python3
r"""Emit the Layer-A grounding cards (AC-6) from a curated spec, binding each hand-authored
quote PROBE to EXACTLY ONE ingested grounding chunk (propose-confirm; `kb verify` is the final
authority).

Authoring a verbatim quote by hand is brittle: the pdfium-extracted chunk text uses curly
apostrophes / en-dashes / collapsed whitespace that a human transcription will not reproduce
byte-for-byte, and a quote that happens to span the chunker's 1-page overlap would bind to TWO
chunks (violating the cacg "one citation -> one chunk" contract). So the spec carries an ASCII
`quote_probe`; this tool:

  * builds a tolerant matcher from the probe (runs of whitespace -> `\s+`; straight/curly
    apostrophes, quotes and hyphen/dash variants -> character classes) and searches the RAW
    chunk text of every chunk in the merged manifest;
  * requires the probe to match EXACTLY ONE chunk (0 -> unbound, >1 -> ambiguous: both fail
    closed) — so a quote sitting in the overlap region of two chunks is rejected, not guessed;
  * stores the EXACT verbatim substring captured from that one chunk (guaranteeing `kb verify`
    exact passes — the stored quote is a literal slice of the chunk), plus the chunk's
    `chunk_id` / `chunk_hash` / `page_range` from the manifest;
  * cross-checks the binding with the kernel-faithful Python normalize port (the stored quote,
    normalized, must be contained in the normalized chunk text) before writing anything;
  * writes `cards/hkex/<reading_id>/<id>.md` with strict `cacg.v0` frontmatter (no `card_hash`
    — absent is valid for not-yet-indexed cards; `kb index` stamps it later) and appends a
    data-driven provenance footer (source url + acquisition/as-of date from
    `grounding_sources.json`) so every HK-official grounding card states its as-of date.

Fail-closed: any unbound / ambiguous / normalize-mismatch citation aborts the whole emit with a
non-zero exit and writes nothing for that card; the report lists every binding.

  --emit         bind the spec + write the cards + the report (exit non-zero on any failure)
  --self-test    drive the binder hermetically (unique binds; ambiguous + missing both rejected)
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "sources/hkex/_registry"))
from cacg_normalize import normalize_text  # noqa: E402  # pyright: ignore[reportMissingImports]

SPEC = ROOT / "sources/hkex/_registry/grounding_cards.json"
CHUNKS = ROOT / "out/hkex/chunks_manifest.json"
GROUNDING = ROOT / "sources/hkex/_registry/grounding_sources.json"
CARDS_DIR = ROOT / "cards/hkex"
REPORT = ROOT / "sources/hkex/_registry/emit_grounding_cards_report.json"
LIVE_EDGES = {"supports", "defines"}

_APOS = "['’‘ʼ`]"          # ' ' ' ʼ `
_QUOTE = '["“”]'                # " " "
_DASH = "[-‐‑‒–—−]"  # - ‐ ‑ ‒ – — −


def probe_to_pattern(probe: str) -> re.Pattern:
    """A tolerant matcher: literal text, but whitespace runs -> \\s+ and apostrophe/quote/dash
    variants -> classes, so the ASCII probe matches the chunk's exact (curly/dashed) bytes. The
    captured group(0) is the verbatim chunk slice we store."""
    out = []
    for ch in probe:
        if ch.isspace():
            out.append(r"\s+")
        elif ch in "'’‘ʼ`":
            out.append(_APOS)
        elif ch in '"“”':
            out.append(_QUOTE)
        elif ch in "-‐‑‒–—−":
            out.append(_DASH)
        else:
            out.append(re.escape(ch))
    # collapse consecutive \s+ tokens that arose from a run of spaces in the probe
    pat = "".join(out)
    pat = re.sub(r"(\\s\+)+", r"\\s+", pat)
    return re.compile(pat, re.DOTALL)


def bind_quote(probe: str, chunks: list[dict]) -> dict:
    """Find the ONE chunk whose raw text contains the probe; return the binding or raise."""
    pat = probe_to_pattern(probe)
    hits = []
    for c in chunks:
        m = pat.search(c["text"])
        if m:
            hits.append((c, m.group(0)))
    if not hits:
        raise ValueError(f"quote_probe matched NO chunk (unbound): {probe!r}")
    if len(hits) > 1:
        ids = ", ".join(c["chunk_id"] for c, _ in hits)
        raise ValueError(f"quote_probe AMBIGUOUS — matched {len(hits)} chunks [{ids}]: {probe!r}")
    chunk, exact = hits[0]
    # belt-and-suspenders: the stored verbatim slice, normalized, must be in the normalized chunk
    if normalize_text(exact) not in normalize_text(chunk["text"]):
        raise ValueError(f"normalize cross-check failed for {chunk['chunk_id']}: {exact!r}")
    return {
        "source_id": chunk["source_id"],
        "chunk_id": chunk["chunk_id"],
        "chunk_hash": chunk["chunk_hash"],
        "page_range": [int(chunk["start_page"]), int(chunk["end_page"])],
        "quote": exact,
    }


def _yaml_str(s: str) -> str:
    """A double-quoted YAML scalar (matches the cfa card style); escape backslash + dquote."""
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def render_card(spec: dict, citations: list[dict], grounding: dict) -> str:
    fm = ["---"]
    fm.append(f"schema_version: {_yaml_str('cacg.v0')}")
    fm.append(f"id: {_yaml_str(spec['id'])}")
    fm.append(f"title: {_yaml_str(spec['title'])}")
    fm.append(f"reading_id: {_yaml_str(spec['reading_id'])}")
    fm.append(f"summary: {_yaml_str(spec['summary'])}")
    fm.append("tags: [" + ", ".join(_yaml_str(t) for t in spec.get("tags", [])) + "]")
    fm.append("citations:")
    for cit, sp in zip(citations, spec["citations"]):
        fm.append(f"  - source_id: {_yaml_str(cit['source_id'])}")
        fm.append(f"    chunk_id: {_yaml_str(cit['chunk_id'])}")
        fm.append(f"    chunk_hash: {_yaml_str(cit['chunk_hash'])}")
        fm.append(f"    page_range: [{cit['page_range'][0]}, {cit['page_range'][1]}]")
        fm.append(f"    quote: {_yaml_str(cit['quote'])}")
        fm.append(f"    edge_type: {_yaml_str(sp['edge_type'])}")
    fm.append("---")
    # data-driven provenance footer: every cited grounding source + its acquisition/as-of date
    gsrc = {s["source_id"]: s for s in grounding.get("sources", [])}
    seen, lines = set(), []
    for cit in citations:
        sid = cit["source_id"]
        if sid in seen:
            continue
        seen.add(sid)
        g = gsrc.get(sid, {})
        lines.append(f"- `{sid}` — {g.get('url', '(url n/a)')} (snapshot acquired {g.get('acquisition_date', 'n/a')}; "
                     f"chunk `{cit['chunk_id']}`)")
    footer = ("\n\n## Grounding sources (as-of)\n\n"
              "Every quote below is verbatim from a free Tier-1 source snapshot, bound by "
              "`kb verify` to the ingested chunk shown; HK-official figures are stated as of the "
              "snapshot date.\n\n" + "\n".join(lines) + "\n")
    return "\n".join(fm) + "\n\n" + spec["body"].strip() + footer


def emit() -> int:
    spec = json.loads(SPEC.read_text(encoding="utf-8"))
    chunks = json.loads(CHUNKS.read_text(encoding="utf-8"))["chunks"]
    grounding = json.loads(GROUNDING.read_text(encoding="utf-8"))
    report = {"schema_version": "hkex.emit_grounding_cards.v1", "cards": [], "errors": []}
    rendered: list[tuple[Path, str]] = []
    for card in spec["cards"]:
        try:
            cits = []
            for sp in card["citations"]:
                if sp["edge_type"] not in LIVE_EDGES:
                    raise ValueError(f"edge_type {sp['edge_type']!r} not in {sorted(LIVE_EDGES)}")
                b = bind_quote(sp["quote_probe"], chunks)
                if b["source_id"] != sp["source_id"]:
                    raise ValueError(f"probe bound to source {b['source_id']} != spec {sp['source_id']}")
                cits.append(b)
            text = render_card(card, cits, grounding)
            out = CARDS_DIR / card["reading_id"] / f"{card['id']}.md"
            rendered.append((out, text))
            report["cards"].append({"id": card["id"], "reading_id": card["reading_id"],
                                    "path": str(out.relative_to(ROOT)),
                                    "citations": [{"source_id": c["source_id"], "chunk_id": c["chunk_id"],
                                                   "page_range": c["page_range"], "quote": c["quote"]} for c in cits]})
        except (ValueError, KeyError) as e:
            report["errors"].append({"card": card.get("id"), "error": str(e)})
    REPORT.write_text(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    if report["errors"]:
        print(json.dumps({"verdict": "FAIL", "errors": report["errors"]}, ensure_ascii=False, indent=2))
        return 1
    for out, text in rendered:   # write only after every card bound cleanly (all-or-nothing)
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(text, encoding="utf-8")
    print(json.dumps({"verdict": "PASS", "cards_written": len(rendered),
                      "by_reading": {r: sum(1 for c in spec["cards"] if c["reading_id"] == r)
                                     for r in sorted({c["reading_id"] for c in spec["cards"]})}},
                     ensure_ascii=False, indent=2))
    return 0


def self_test() -> int:
    failures = []
    chunks = [
        {"source_id": "s1", "chunk_id": "s1:p001:0000", "chunk_hash": "a" * 64,
         "start_page": 1, "end_page": 1, "text": "Alpha the rate is 0.1% per side and the share’s value."},
        {"source_id": "s1", "chunk_id": "s1:p001:0001", "chunk_hash": "b" * 64,
         "start_page": 1, "end_page": 2, "text": "Beta a unique tail sentence over here only."},
        {"source_id": "s2", "chunk_id": "s2:p001:0000", "chunk_hash": "c" * 64,
         "start_page": 1, "end_page": 1, "text": "Gamma a wholly different passage."},
    ]
    # unique bind across whitespace runs; stored slice is the verbatim chunk text
    b = bind_quote("the   rate is 0.1% per side", chunks)
    if b["chunk_id"] != "s1:p001:0000" or "0.1% per side" not in b["quote"]:
        failures.append(f"unique bind wrong: {b}")
    # apostrophe class: ASCII ' in the probe matches the chunk's curly apostrophe (unique to chunk 0)
    b2 = bind_quote("the share's value", chunks)
    if b2["chunk_id"] != "s1:p001:0000" or "value" not in b2["quote"]:
        failures.append(f"apostrophe-class bind wrong: {b2}")
    # ambiguous (same phrase in two chunks, e.g. a chunker overlap) -> reject
    dup = [dict(chunks[0], text="x the shared phrase y"),
           dict(chunks[1], text="z the shared phrase w"), chunks[2]]
    try:
        bind_quote("the shared phrase", dup)
        failures.append("ambiguous quote not rejected")
    except ValueError:
        pass
    # missing -> reject
    try:
        bind_quote("a fabricated control sentence absent everywhere", chunks)
        failures.append("missing quote not rejected")
    except ValueError:
        pass
    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print("  -", f)
        return 1
    print("SELF-TEST PASSED (emit_grounding_cards: unique/whitespace/apostrophe bind; ambiguous + missing rejected)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--emit", action="store_true")
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        return self_test()
    if args.emit:
        return emit()
    ap.error("provide --emit or --self-test")


if __name__ == "__main__":
    raise SystemExit(main())
