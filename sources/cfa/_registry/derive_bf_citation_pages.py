#!/usr/bin/env python3
"""Re-derive each Behavioral-Finance citation's ``page_range`` to the TRUE PDF
page(s) located by verbatim quote search (decision D1).

The BF skeletons were authored with PRINTED-BOOK page numbers, and each source carries
a different (unknown-to-the-resolver) printed->PDF offset. The incoming resolver has NO
offset map: it binds by clamping candidate chunks to the cited page window, so a printed
page would steer it to the WRONG chunks. This pass rewrites each citation's ``page_range``
to the PDF page where its (kernel-normalized) quote actually appears, so the unchanged
clamp-only resolver binds against the correct candidate chunks. The printed page is a
search hint / recorded provenance only — the verbatim quote is ground truth.

FAIL-CLOSED: a quote that matches zero pages, or more than one page-window, is NOT
silently bound. It is recorded ``unresolved`` / ``ambiguous`` and must be resolved by a
committed reviewed override (``bf_page_derivation_overrides.json``) that names the
page-window and is re-verified (the quote must verbatim-appear, after kernel
normalization, in the chosen window). A committed audit ledger records, per citation:
the page_range read from the skeleton, the derived PDF page-window, the implied offset,
the candidate count, and the status.

The rewrite is LINE-SURGICAL (only each citation's ``page_range:`` line changes; the
body and the verbatim quotes are untouched) and IDEMPOTENT at the deck level: re-deriving
an already-PDF-paged citation yields the same page (the quote is on it), so a second
``--write`` leaves the skeleton deck byte-identical.

Page text comes from ``pdftotext`` (page numbering is the PDF's physical pages, identical
to libpdfium's, so the derived page selects the right ingested chunk); normalization is
the kernel-parity ``normalize_text`` shared with ``resolve_migration_citations.py``.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import statistics
import subprocess
from pathlib import Path
from typing import Any

OFFSET_OUTLIER_PAGES = 5   # flag a resolved citation whose printed->PDF offset deviates this far
                           # from its source's median — a faithfulness-review signal (the verbatim
                           # quote may sit far from the cited printed page)

import yaml

import resolve_migration_citations as rez


ROOT = Path(__file__).resolve().parents[3]
REGISTRY = ROOT / "sources/cfa/_registry"
CFA_READING = Path(os.environ.get("KB_CFA_READING", "/home/jakeshea/CFA_reading"))
DEFAULT_DECK = CFA_READING / "10_Behavioral_Finance/_card_skeletons/10_behavioral_finance"
SOURCES_JSON = CFA_READING / "10_Behavioral_Finance/_card_skeletons/_sources.json"
PDFS_DIR = ROOT / "sources/cfa/pdfs/10_behavioral_finance"
LEDGER_PATH = REGISTRY / "bf_page_derivation_ledger.json"
OVERRIDES_PATH = REGISTRY / "bf_page_derivation_overrides.json"

SKIP_FILES = {"INDEX.md", "README.md", "SCOPE_EXCEPTION.md"}
PAGE_RANGE_RE = re.compile(r'^(\s*)page_range:\s*\[.*\]\s*$')
CITATIONS_KEY_RE = re.compile(r'^citations:\s*$')


# --------------------------------------------------------------------------- #
# PDF page text.
# --------------------------------------------------------------------------- #

# Typographic folds applied ON TOP of the kernel-parity normalize_text, for PAGE
# LOCATION ONLY. The quote is on a physical page; the only thing that stops a verbatim
# match is the extractor rendering typographic variants differently (curly vs straight
# quotes, en/em dashes vs hyphen, a unicode ellipsis vs "..."). Folding these locates
# the page faithfully. The RESOLVER still binds with the unmodified kernel normalize_text
# (kernel parity preserved); this looser form never feeds a chunk_hash.
_TYPO = {
    "“": '"', "”": '"', "„": '"', "‟": '"',   # double curly quotes
    "‘": "'", "’": "'", "‚": "'", "‛": "'",   # single curly quotes
    "′": "'", "″": '"',                                  # prime / double prime
    "–": "-", "—": "-", "―": "-", "−": "-",   # en/em/horizontal/minus
    "…": "...",                                               # ellipsis
}


def loose_normalize(text: str) -> str:
    s = rez.normalize_text(text)
    for k, v in _TYPO.items():
        if k in s:
            s = s.replace(k, v)
    return s


def content_tokens(norm_text: str) -> set[str]:
    """Content words (alphanumeric tokens >= 3 chars) for recall-based location."""
    return {t for t in re.findall(r"[a-z0-9]+", norm_text.lower()) if len(t) >= 3}


def extract_pages(pdf: Path) -> list[str]:
    """Per-page raw text via pdftotext (page breaks on form-feed). Index P-1 is PDF
    page P (1-based)."""
    out = subprocess.run(["pdftotext", str(pdf), "-"], capture_output=True, text=True,
                         timeout=900, check=True).stdout
    pages = out.split("\f")
    if pages and pages[-1] == "":
        pages.pop()
    return pages


def find_windows(quote: str, raw_pages: list[str], loose_pages: list[str]) -> list[list[int]]:
    """All 1-based page-windows whose loosely-normalized text contains the loosely-
    normalized quote: single pages first; only if none, consecutive 2-page spans (a
    quote split across a page boundary)."""
    nq = loose_normalize(quote)
    windows = [[i + 1, i + 1] for i, lp in enumerate(loose_pages) if nq in lp]
    if not windows:
        for i in range(len(raw_pages) - 1):
            if nq in loose_normalize(raw_pages[i] + "\n" + raw_pages[i + 1]):
                windows.append([i + 1, i + 2])
    return windows


RECALL_MIN = 0.80          # >= this fraction of the quote's content words on the page
RECALL_MARGIN = 0.10       # the best page must beat the runner-up by this much (unique location)
RECALL_WINDOW = 10         # recall is searched only within +/- this many pages of the offset-
                           # expected page (printed + the source's learned median offset), so a
                           # bag-of-words match on a far unrelated topic page can never win
OVERRIDE_PROXIMITY = 12    # a reviewed override's page must be within this many pages of the
                           # offset-expected page (catches a reviewer page typo), unless the
                           # override sets ``skip_proximity`` to acknowledge a genuine far page


def recall_locate(quote: str, page_tokens: list[set[str]],
                  lo: int | None = None, hi: int | None = None) -> tuple[int, float] | None:
    """Locate a quote whose typography/math defeats verbatim matching by content-word
    recall: the single page whose token set covers the most of the quote's content
    words, if that page is BOTH above RECALL_MIN AND a clear unique maximum (beats the
    runner-up by RECALL_MARGIN). When ``lo``/``hi`` are given, only those 1-based pages
    are considered (proximity gate). Returns (page_1based, recall) or None."""
    qt = content_tokens(loose_normalize(quote))
    if not qt:
        return None
    rng = (range(len(page_tokens)) if lo is None or hi is None
           else range(max(0, lo - 1), min(len(page_tokens), hi)))
    scored = sorted(((len(qt & page_tokens[i]) / len(qt), i + 1) for i in rng),
                    key=lambda x: (-x[0], x[1]))
    if not scored:
        return None
    best_recall, best_page = scored[0]
    runner = scored[1][0] if len(scored) > 1 else 0.0
    if best_recall >= RECALL_MIN and best_recall - runner >= RECALL_MARGIN:
        return best_page, round(best_recall, 3)
    return None


def window_contains(quote: str, window: list[int], raw_pages: list[str]) -> bool:
    lo, hi = window[0], window[-1]
    if not (1 <= lo <= hi <= len(raw_pages)):
        return False
    joined = "\n".join(raw_pages[lo - 1:hi])
    return loose_normalize(quote) in loose_normalize(joined)


# --------------------------------------------------------------------------- #
# Skeleton IO.
# --------------------------------------------------------------------------- #

def card_paths(deck: Path) -> list[Path]:
    return [p for p in sorted(deck.glob("*.md"))
            if p.name not in SKIP_FILES and not p.name.startswith("_")]


def parse_card(text: str) -> dict[str, Any]:
    """Parse the frontmatter, anchoring on the ``---\\n`` / ``\\n---`` delimiters (NOT a
    bare ``---`` substring, which a quote could contain) so this agrees with
    normalize_template_deck.split_card on where the frontmatter ends."""
    if not text.startswith("---\n"):
        raise ValueError("missing opening frontmatter delimiter")
    end = text.find("\n---", 4)
    if end == -1:
        raise ValueError("missing closing frontmatter delimiter")
    fm = yaml.safe_load(text[4:end])
    if not isinstance(fm, dict):
        raise ValueError("frontmatter not a mapping")
    return fm


def rewrite_page_ranges(text: str, windows: list[list[int]]) -> str:
    """Replace the citations' ``page_range:`` lines (in order) with the derived
    windows. Only lines AFTER the top-level ``citations:`` key are touched; the count
    of page_range lines must equal len(windows) or the card is rejected."""
    lines = text.split("\n")
    in_citations = False
    k = 0
    out: list[str] = []
    for line in lines:
        if CITATIONS_KEY_RE.match(line):
            in_citations = True
            out.append(line)
            continue
        m = PAGE_RANGE_RE.match(line)
        if in_citations and m:
            if k >= len(windows):
                raise ValueError("more page_range lines than derived windows")
            lo, hi = windows[k]
            out.append(f"{m.group(1)}page_range: [{lo}, {hi}]")
            k += 1
            continue
        out.append(line)
    if k != len(windows):
        raise ValueError(f"page_range line count {k} != citation count {len(windows)}")
    return "\n".join(out)


# --------------------------------------------------------------------------- #
# Derivation.
# --------------------------------------------------------------------------- #

def load_overrides() -> dict[tuple[str, str], dict[str, Any]]:
    if not OVERRIDES_PATH.is_file():
        return {}
    doc = json.loads(OVERRIDES_PATH.read_text(encoding="utf-8"))
    return {(o["source_id"], o["quote"]): o for o in doc.get("overrides", [])}


def derive(deck: Path, write: bool) -> dict[str, Any]:
    if not deck.is_dir():
        raise SystemExit(f"deck directory not found: {deck}")
    overrides = load_overrides()
    page_cache: dict[str, tuple[list[str], list[str], list[set[str]]]] = {}

    def pages_for(source_id: str) -> tuple[list[str], list[str], list[set[str]]]:
        if source_id not in page_cache:
            pdf = PDFS_DIR / f"{source_id}.pdf"
            if not pdf.is_file():
                raise SystemExit(f"source PDF not found for {source_id}: {pdf}")
            raw = extract_pages(pdf)
            loose = [loose_normalize(p) for p in raw]
            page_cache[source_id] = (raw, loose, [content_tokens(lp) for lp in loose])
        return page_cache[source_id]

    ledger: list[dict[str, Any]] = []
    rewrites: list[tuple[Path, str]] = []
    blocking: list[str] = []
    counts = {"auto": 0, "reviewed_override": 0, "recall_located": 0, "ambiguous": 0, "unresolved": 0}

    # PASS A — find verbatim windows for every citation and LEARN each source's
    # printed->PDF offset from the unambiguous ``auto`` rows (the offset model used to
    # GATE the non-verbatim paths below).
    parsed: list[tuple[Path, str, dict[str, Any], list[tuple[dict[str, Any], list[list[int]]]]]] = []
    by_src_off: dict[str, list[int]] = {}
    for path in card_paths(deck):
        text = path.read_text(encoding="utf-8")
        fm = parse_card(text)
        cit_found: list[tuple[dict[str, Any], list[list[int]]]] = []
        for cit in fm.get("citations") or []:
            raw_pages, loose_pages, _ = pages_for(cit["source_id"])
            found = find_windows(cit["quote"], raw_pages, loose_pages)
            cit_found.append((cit, found))
            pr = cit.get("page_range")
            if len(found) == 1 and isinstance(pr, list) and pr:
                by_src_off.setdefault(cit["source_id"], []).append(found[0][0] - pr[0])
        parsed.append((path, text, fm, cit_found))
    src_median = {s: int(statistics.median(v)) for s, v in by_src_off.items()}

    # PASS B — resolve each citation. The verbatim ``auto`` path is unconditional; the
    # non-verbatim paths (reviewed_override, recall_located) are GATED by proximity to the
    # offset-expected page (printed + the source's learned median offset), so a reviewer
    # page typo or a bag-of-words match on a far topic page cannot silently bind.
    for path, text, fm, cit_found in parsed:
        windows: list[list[int]] = []
        for cit, found in cit_found:
            sid = cit["source_id"]
            quote = cit["quote"]
            read_pr = cit.get("page_range")
            raw_pages, loose_pages, page_tokens = pages_for(sid)
            read_lo = read_pr[0] if isinstance(read_pr, list) and read_pr else None
            median = src_median.get(sid)
            expected = (read_lo + median) if (read_lo is not None and median is not None) else None
            ov = overrides.get((sid, quote))
            recall: float | None = None

            if ov is not None:
                # A human-reviewed override wins (the AC-3 path for an ambiguous match or a
                # math/elision quote that defeats verbatim search). VERIFIED two ways: its
                # ``evidence_quote`` (a distinctive prose phrase the reviewer read on the page)
                # must loose-verbatim appear in the window, AND the page must lie within
                # OVERRIDE_PROXIMITY of the offset-expected page (unless ``skip_proximity``).
                win = [ov["page_range"][0], ov["page_range"][-1]]
                verify_phrase = ov.get("evidence_quote") or quote
                if not window_contains(verify_phrase, win, raw_pages):
                    raise SystemExit(
                        f"reviewed override for {fm['id']}/{sid} does not verify: evidence "
                        f"{verify_phrase[:60]!r} not found in page window {win}")
                if (expected is not None and not ov.get("skip_proximity")
                        and abs(win[0] - expected) > OVERRIDE_PROXIMITY):
                    raise SystemExit(
                        f"reviewed override for {fm['id']}/{sid} page {win} is "
                        f"{abs(win[0] - expected)} pages from the offset-expected page {expected} "
                        f"(printed {read_lo} + source median {median:+d}); set 'skip_proximity': true "
                        f"in the override to accept a genuine far page")
                status = "reviewed_override"
            elif len(found) == 1:
                win, status = found[0], "auto"
            elif len(found) > 1:
                win, status = (read_pr if isinstance(read_pr, list) else [0, 0]), "ambiguous"
            else:
                # Verbatim (even loose) failed — quote transcribes math/elision the extractor
                # renders differently. Locate by content-word recall, RESTRICTED to a window
                # around the offset-expected page when an offset model exists; accepted only as
                # a UNIQUE clear maximum above threshold, else fail closed (reviewed override).
                if expected is not None:
                    loc = recall_locate(quote, page_tokens, expected - RECALL_WINDOW, expected + RECALL_WINDOW)
                else:
                    loc = recall_locate(quote, page_tokens)
                if loc is not None:
                    win, status, recall = [loc[0], loc[0]], "recall_located", loc[1]
                else:
                    win, status = (read_pr if isinstance(read_pr, list) else [0, 0]), "unresolved"

            counts[status] += 1
            resolved = status in ("auto", "reviewed_override", "recall_located")
            ledger.append({
                "card_id": fm["id"], "source_id": sid,
                "skeleton_page_range_read": read_pr,
                "offset_expected_page": expected,
                "derived_page_range": win if resolved else None,
                "implied_offset": (win[0] - read_lo) if (resolved and read_lo is not None) else None,
                "candidate_count": len(found),
                "candidate_windows": found[:10],
                "recall": recall,
                "status": status,
                "quote_head": quote[:90],
            })
            if resolved:
                windows.append([win[0], win[-1]])
            else:
                windows.append(read_pr if isinstance(read_pr, list) else [1, 1])
                blocking.append(f"{fm['id']} [{sid}] {status} (candidates={len(found)}): {quote[:70]!r}")

        try:
            new_text = rewrite_page_ranges(text, windows)
        except ValueError as exc:
            raise SystemExit(f"{path.name}: {exc}")
        if new_text != text:
            rewrites.append((path, new_text))

    # Flag any resolved row whose offset deviates far from its source's median — a worklist
    # for the faithfulness review (a verbatim-unique quote located far from its cited printed
    # page may be a coincidental match or a mis-cited page, even though the quote IS there).
    offset_outliers = [
        {"card_id": r["card_id"], "source_id": r["source_id"], "status": r["status"],
         "implied_offset": r["implied_offset"], "source_median_offset": src_median.get(r["source_id"]),
         "derived_page_range": r["derived_page_range"], "quote_head": r["quote_head"]}
        for r in ledger
        if r["implied_offset"] is not None and r["source_id"] in src_median
        and abs(r["implied_offset"] - src_median[r["source_id"]]) > OFFSET_OUTLIER_PAGES
    ]

    summary = {
        "schema_version": "cfa.bf_page_derivation_ledger.v1",
        "deck": str(deck),
        "citations_total": len(ledger),
        "status_counts": counts,
        "source_median_offset": dict(sorted(src_median.items())),
        "offset_outliers_for_faithfulness_review": offset_outliers,
        "status_legend": {
            "auto": "loose-verbatim quote found on exactly one page-window — the PDF page is that window",
            "recall_located": "verbatim (even loose) failed because the quote transcribes math notation "
                              "(C_1->C1, gamma->γ, ∈->2) or elides text with '...'; the page is the UNIQUE "
                              "clear maximum of content-word recall (>= 0.80, beats runner-up by >= 0.10). "
                              "The resolver verbatim-binds within this window via its equation/boundary repair.",
            "reviewed_override": "human-reviewed page from bf_page_derivation_overrides.json, verified by a "
                                 "distinctive evidence_quote that loose-verbatim appears in the window",
        },
        "cards_needing_rewrite": len(rewrites),
        "unresolved_or_ambiguous": counts["ambiguous"] + counts["unresolved"],
    }

    if blocking:
        # Fail closed: print the ledger so the unresolved/ambiguous rows can be reviewed,
        # but do NOT write a partial deck.
        if write:
            LEDGER_PATH.write_text(
                json.dumps({**summary, "rows": ledger}, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
                encoding="utf-8")
        raise SystemExit(
            f"{len(blocking)} BF citation(s) unresolved/ambiguous (fail-closed; add a reviewed "
            f"override to {OVERRIDES_PATH.name}):\n  " + "\n  ".join(blocking[:30]))

    written = 0
    if write:
        for path, new_text in rewrites:
            path.write_text(new_text, encoding="utf-8")
            written += 1
        # Write the ledger only when conditioning was actually applied (or it is absent).
        # The ledger's implied_offset is the printed->PDF offset captured during the ONE-TIME
        # conditioning run (reading printed pages); a no-op re-run over the already-conditioned
        # deck reads PDF pages, so rewriting then would zero the offsets — instead it is left
        # intact (the deck is byte-stable, so the conditioning record stays valid).
        if rewrites or not LEDGER_PATH.is_file():
            LEDGER_PATH.write_text(
                json.dumps({**summary, "rows": ledger}, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
                encoding="utf-8")
    summary["cards_rewritten"] = written if write else 0
    summary["wrote_files"] = write
    return summary


# --------------------------------------------------------------------------- #
# Self-test (hermetic).
# --------------------------------------------------------------------------- #

def _self_test() -> int:
    f: list[str] = []
    raw = ["alpha beta gamma", "the quick brown fox jumps", "delta epsilon zeta"]
    loose = [loose_normalize(p) for p in raw]
    # single-page match
    if find_windows("quick brown fox", raw, loose) != [[2, 2]]:
        f.append(f"single-page: {find_windows('quick brown fox', raw, loose)}")
    # zero match
    if find_windows("no such phrase here at all", raw, loose) != []:
        f.append("zero match should be empty")
    # boundary 2-page span (only when no single page matches)
    raw2 = ["ends with quick brown", "fox jumps over"]
    loose2 = [loose_normalize(p) for p in raw2]
    if find_windows("quick brown fox jumps", raw2, loose2) != [[1, 2]]:
        f.append(f"boundary span: {find_windows('quick brown fox jumps', raw2, loose2)}")
    # ambiguous (same phrase on two pages)
    raw3 = ["repeated marker text", "x", "repeated marker text"]
    loose3 = [loose_normalize(p) for p in raw3]
    if find_windows("repeated marker text", raw3, loose3) != [[1, 1], [3, 3]]:
        f.append(f"ambiguous: {find_windows('repeated marker text', raw3, loose3)}")
    # loose_normalize folds curly quotes / em-dash / ellipsis so a verbatim match holds.
    rawq = ['they display “excess volatility” here and — more … end']
    if find_windows('they display "excess volatility" here and - more ... end', rawq,
                    [loose_normalize(rawq[0])]) != [[1, 1]]:
        f.append("loose_normalize did not fold curly quotes / dash / ellipsis")
    # recall_locate: a math-mangled quote still locates its UNIQUE page by content recall.
    rtok = [content_tokens(loose_normalize(p)) for p in
            ["alpha beta gamma delta epsilon", "the cost gamma intensity choice fraction profit", "zzz"]]
    loc = recall_locate("intensity of the choice gamma and the cost fraction profit", rtok)
    if loc is None or loc[0] != 2:
        f.append(f"recall_locate unique page: {loc}")
    # recall ambiguity / low coverage -> None (fail closed).
    rtok_amb = [content_tokens("shared marker token alpha"), content_tokens("shared marker token alpha")]
    if recall_locate("shared marker token alpha", rtok_amb) is not None:
        f.append("recall_locate should be None on a tie")
    # windowed recall: a high-recall page OUTSIDE the proximity window is excluded.
    rtok_win = [content_tokens(loose_normalize(p)) for p in
                ["unrelated topic alpha", "intensity choice gamma cost fraction profit value metric here",
                 "topic three beta", "topic four delta"]]
    q_win = "intensity choice gamma cost fraction profit value metric"
    loc_full = recall_locate(q_win, rtok_win)
    if loc_full is None or loc_full[0] != 2:
        f.append(f"unwindowed recall should find page 2, got {loc_full}")
    if recall_locate(q_win, rtok_win, 3, 4) is not None:
        f.append("windowed recall must exclude the out-of-window high-recall page 2")
    loc_in = recall_locate(q_win, rtok_win, 1, 2)
    if loc_in is None or loc_in[0] != 2:
        f.append("windowed recall should find page 2 when it is in the window")
    # window_contains
    if not window_contains("quick brown", [2, 2], raw):
        f.append("window_contains true case (page 2)")
    if window_contains("quick brown", [1, 1], raw):
        f.append("window_contains false case (page 1)")

    # rewrite_page_ranges: only citation page_range lines change, in order.
    card = ('---\nid: "be-x"\ncitations:\n'
            '  - source_id: "s"\n    chunk_id: "PENDING_INGEST"\n    chunk_hash: "PENDING_INGEST"\n'
            '    page_range: [159, 159]\n    quote: "q1"\n    edge_type: "defines"\n'
            '  - source_id: "s"\n    chunk_id: "PENDING_INGEST"\n    chunk_hash: "PENDING_INGEST"\n'
            '    page_range: [160, 160]\n    quote: "q2"\n    edge_type: "supports"\n---\nbody page_range: [9, 9]\n')
    rewritten = rewrite_page_ranges(card, [[167, 167], [168, 168]])
    if "page_range: [167, 167]" not in rewritten or "page_range: [168, 168]" not in rewritten:
        f.append("rewrite did not apply derived windows")
    if "body page_range: [9, 9]" not in rewritten:
        f.append("rewrite touched a body page_range line")
    if rewritten.count("page_range:") != 3:
        f.append("rewrite changed the number of page_range lines")
    try:
        rewrite_page_ranges(card, [[1, 1]])  # wrong count
        f.append("count mismatch not caught")
    except ValueError:
        pass

    if f:
        print("SELF-TEST FAILED:")
        for x in f:
            print("  -", x)
        return 1
    print("SELF-TEST PASSED (find_windows single/zero/boundary/ambiguous + window_contains + "
          "ordered page_range rewrite + count guard)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--self-test", action="store_true", help="run hermetic self-test and exit")
    ap.add_argument("--deck", type=Path, default=DEFAULT_DECK, help="BF skeleton directory")
    ap.add_argument("--write", action="store_true",
                    help="rewrite each citation's page_range in place and write the audit ledger")
    args = ap.parse_args()
    if args.self_test:
        return _self_test()
    summary = derive(args.deck, write=args.write)
    print(json.dumps(summary, ensure_ascii=False, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
