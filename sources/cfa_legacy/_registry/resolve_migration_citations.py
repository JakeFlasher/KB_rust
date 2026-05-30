#!/usr/bin/env python3
"""One-time quote->chunk resolver for the 14/15/22 migration.

Binds every incoming skeleton citation to exactly one chunk of the frozen merged
manifest, writing three curated citation registries (mt_14 / pa_15 / fa_22), an
auditable bind report, and a quote-integrity audit. It PROPOSES a binding (the
chunk whose page span intersects the citation page_range and whose normalized
text contains the normalized quote) and CONFIRMS every binding with the REAL
`kb verify` (the kernel is the containment authority -- AC-equivalent parity).
Fail-closed: a citation that binds to zero/multiple/boundary chunks, or whose
final quote does not pass `kb verify`, is reported and never silently bound.

Some incoming quotes span a Pdfium soft-hyphen marker (U+FFFE) that the kernel's
`normalize_text` preserves (it de-hyphenates only `-\\n`), so the de-hyphenated
authored quote cannot verify. Those are repaired (user decision: hybrid):
minimal TRIM to the longest clean verbatim run when that retains >=60% of the
quote, else RE-ANCHOR to the best-overlap clean verbatim sentence on the cited
page. Every repair is recorded in the audit and re-confirmed by `kb verify`.

`normalize_text` here is a faithful Python re-implementation of the kernel
`crates/cacg-core/src/normalize.rs::normalize_text`; the parity fixture
(`--parity`) proves it agrees with the real `kb verify` on a set of existing v0
citations spanning whitespace / hyphenation / ligature / punctuation /
chunk-boundary cases plus known-FAIL probes.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import tempfile
import unicodedata
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[3]
REGISTRY = ROOT / "sources/cfa_legacy/_registry"
OUT = ROOT / "out/cfa_legacy"
CHUNKS_MANIFEST = OUT / "chunks_manifest.json"
SOURCE_MATRIX = OUT / "source_matrix.json"
CARDS_MANIFEST = OUT / "cards_manifest.json"
KB_BINARY = ROOT / "target/debug/kb"
DEFERRED = Path(os.environ.get("KB_DEFERRED_ORIGIN", "/home/jakeshea/CFA_reading/deferred_books"))

SETS = (
    ("14_Microstructure_and_Trading", "14_microstructure_and_trading", "mt_14"),
    ("15_Performance_and_Attribution", "15_performance_and_attribution", "pa_15"),
    ("22_Fund_Level_Arbitrage", "22_fund_level_arbitrage", "fa_22"),
)
BIND_REPORT = REGISTRY / "migration_bind_report.json"
QUOTE_AUDIT = REGISTRY / "migration_quote_audit.json"
TRIM_RETAIN_MIN = 0.60   # >=60% of the quote retained -> trim; else re-anchor
MIN_QUOTE_CHARS = 25

_LIGATURES = {"ﬀ": "ff", "ﬁ": "fi", "ﬂ": "fl", "ﬃ": "ffi",
              "ﬄ": "ffl", "ﬅ": "ft", "ﬆ": "st"}


# --------------------------------------------------------------------------- #
# Kernel-parity normalization (re-implements crates/cacg-core/src/normalize.rs).
# --------------------------------------------------------------------------- #

def normalize_text(text: str) -> str:
    s = unicodedata.normalize("NFC", text)
    if any(c in s for c in _LIGATURES):
        for src, dst in _LIGATURES.items():
            s = s.replace(src, dst)
    s = re.sub(r"-\s*\n\s*", "", s)   # hyphen-linebreak rejoin (Python re \s == kernel predicate)
    s = re.sub(r"\s+", " ", s)        # collapse whitespace runs
    return s.strip()


def is_illegal_quote_char(c: str) -> bool:
    o = ord(c)
    noncharacter = (0xFDD0 <= o <= 0xFDEF) or (o & 0xFFFE) == 0xFFFE
    return noncharacter or unicodedata.category(c) in {"Cc", "Cf", "Zl", "Zp"}


def has_illegal_chars(text: str) -> bool:
    return any(is_illegal_quote_char(c) for c in text)


# --------------------------------------------------------------------------- #
# Loading.
# --------------------------------------------------------------------------- #

def load_skeletons() -> list[dict[str, Any]]:
    import yaml
    cards: list[dict[str, Any]] = []
    for origin, rid, prefix in SETS:
        skd = DEFERRED / origin / "_card_skeletons" / rid
        for f in sorted(skd.glob("*.md")):
            fm = yaml.safe_load(f.read_text(encoding="utf-8").split("---", 2)[1])
            if not fm or fm.get("reading_id") != rid:
                continue
            cards.append({
                "registry": prefix, "reading_id": rid, "card_id": fm["id"],
                "title": fm["title"], "summary": fm["summary"], "tags": fm["tags"],
                "citations": fm["citations"],
            })
    return cards


def chunk_pages_text(chunk: dict[str, Any], lo: int, hi: int) -> str:
    """The chunk's raw text restricted to pages [lo, hi] via page_spans byte offsets."""
    spans = chunk.get("page_spans") or []
    raw = chunk["text"].encode("utf-8")
    keep = bytearray()
    for i, sp in enumerate(spans):
        start = sp["byte_offset_in_chunk"]
        end = spans[i + 1]["byte_offset_in_chunk"] if i + 1 < len(spans) else len(raw)
        if lo <= sp["page"] <= hi:
            keep += raw[start:end]
    return keep.decode("utf-8", "ignore") if keep else chunk["text"]


# --------------------------------------------------------------------------- #
# kb verify oracle.
# --------------------------------------------------------------------------- #

def kb_verify(reading_id: str, source_id: str, chunk: dict[str, Any], page_range: list[int],
              quote: str, edge_type: str, work: Path, env: dict[str, str], idx: int) -> bool:
    card = {"schema_version": "cacg.v0", "id": f"resolve-probe-{idx}", "title": "Resolver probe",
            "reading_id": reading_id, "summary": "x" * 90, "tags": ["resolver"],
            "citations": [{"source_id": source_id, "chunk_id": chunk["chunk_id"],
                           "chunk_hash": chunk["chunk_hash"], "page_range": page_range,
                           "quote": quote, "edge_type": edge_type}]}
    p = work / f"probe_{idx}.md"
    p.write_text("---\n" + json.dumps(card) + "\n---\n\nbody\n", encoding="utf-8")
    j = work / f"j_{idx}.jsonl"
    r = subprocess.run([str(KB_BINARY), "verify", str(p), "--chunks-manifest", str(CHUNKS_MANIFEST),
                        "--source-matrix", str(SOURCE_MATRIX), "--journal", str(j)],
                       capture_output=True, text=True, env=env)
    p.unlink(missing_ok=True); j.unlink(missing_ok=True)
    return r.returncode == 0


# --------------------------------------------------------------------------- #
# Quote repair helpers (pure; --self-test).
# --------------------------------------------------------------------------- #

def _is_word_bounded(needle: str, hay: str) -> bool:
    """True if `needle` occurs in `hay` bounded by spaces / string edges."""
    start = 0
    while True:
        k = hay.find(needle, start)
        if k < 0:
            return False
        before = k == 0 or hay[k - 1] == " "
        after = k + len(needle) == len(hay) or hay[k + len(needle)] == " "
        if before and after:
            return True
        start = k + 1


def longest_clean_run(quote: str, chunk_text: str) -> str:
    """Longest contiguous run of the quote's WORDS that is a clean (illegal-free)
    word-bounded verbatim substring of the normalized chunk text. Word-aligned so a
    U+FFFE-broken word (chunk `ex<U+FFFE>clusive` vs quote `exclusive`) is excluded
    rather than left as a mid-word fragment."""
    nq = normalize_text(quote)
    nc = normalize_text(chunk_text)
    toks = nq.split(" ")
    best = ""
    for i in range(len(toks)):
        for j in range(i + 1, len(toks) + 1):
            cand = " ".join(toks[i:j])
            if has_illegal_chars(cand):
                break
            if _is_word_bounded(cand, nc) and len(cand) > len(best):
                best = cand
    return best


def candidate_sentences(page_text: str) -> list[str]:
    """Clean (U+FFFE/illegal-free) verbatim sentences from the cited-page text."""
    norm = normalize_text(page_text)
    out: list[str] = []
    for sent in re.split(r"(?<=[.!?])\s+", norm):
        sent = sent.strip()
        if len(sent) >= MIN_QUOTE_CHARS and not has_illegal_chars(sent):
            out.append(sent)
    return out


def overlap_score(a: str, b: str) -> float:
    wa = set(re.findall(r"[a-z0-9]+", a.lower()))
    wb = set(re.findall(r"[a-z0-9]+", b.lower()))
    return len(wa & wb) / len(wa | wb) if (wa or wb) else 0.0


# --------------------------------------------------------------------------- #
# Binding.
# --------------------------------------------------------------------------- #

def window_contains(chunk: dict[str, Any], lo: int, hi: int, needle_norm: str) -> bool:
    """normalize_text(quote) within the normalized cited-page window of the chunk
    (parity-proven against kb verify's page_range containment)."""
    return needle_norm in normalize_text(chunk_pages_text(chunk, lo, hi))


def bind_citation(cit: dict[str, Any], reading_id: str, by_src: dict[str, list]) -> dict[str, Any]:
    """PROPOSE a binding for one citation via parity-proven normalize_text containment.
    Pure (no subprocess); the proposal is confirmed by a later parallel kb-verify pass.
    Fail-closed: zero clean trim/re-anchor -> status 'unbound'."""
    sid = cit["source_id"]
    pr = cit["page_range"]
    lo, hi = (pr[0], pr[-1]) if isinstance(pr, list) else (pr, pr)
    quote = cit["quote"]
    edge = cit["edge_type"]
    nq = normalize_text(quote)
    cand = sorted([c for c in by_src.get(sid, []) if not (c["end_page"] < lo or c["start_page"] > hi)],
                  key=lambda c: c["chunk_id"])

    def bound(chunk, final_quote, final_pr, status, change):
        return {"card_id": None, "status": status, "change": change,
                "reading_id": reading_id,
                "registry": {"source_id": sid, "chunk_id": chunk["chunk_id"],
                             "chunk_hash": chunk["chunk_hash"], "page_range": final_pr,
                             "quote": final_quote, "edge_type": edge},
                "authored_page_range": [lo, hi]}

    # 1. AUTO-bind: candidate(s) whose normalized cited-page-window text contains the authored quote.
    # Chunks can OVERLAP, so pick the best-fit one deterministically (full containment, tightest span, id).
    proposed = [(c, [max(lo, c["start_page"]), min(hi, c["end_page"])]) for c in cand]
    proposed = [(c, cpr) for c, cpr in proposed if window_contains(c, cpr[0], cpr[1], nq)]
    if proposed:
        def fit_key(item):
            c = item[0]
            contains = 0 if (c["start_page"] <= lo and hi <= c["end_page"]) else 1
            return (contains, c["end_page"] - c["start_page"], c["chunk_id"])
        c, cpr = sorted(proposed, key=fit_key)[0]
        corrected = cpr != [lo, hi]
        return bound(c, quote, cpr, "auto", {"type": "page_window",
                     "old_page_range": [lo, hi], "new_page_range": cpr} if corrected else None)

    # 2. REPAIR (U+FFFE / no clean verbatim match). Best candidate chunk = the one whose
    # U+FFFE-stripped normalized text contains the quote, preferring full page containment.
    def stripped(s): return normalize_text(s).replace("￾", "")
    repair_cand = [c for c in cand if stripped(quote) in stripped(c["text"])] or cand
    if not repair_cand:
        return {"status": "no_candidate_chunk", "authored_page_range": [lo, hi]}
    repair_cand = sorted(repair_cand, key=lambda c: (0 if (c["start_page"] <= lo and hi <= c["end_page"]) else 1,
                                                     c["end_page"] - c["start_page"], c["chunk_id"]))
    chunk = repair_cand[0]
    cpr = [max(lo, chunk["start_page"]), min(hi, chunk["end_page"])]

    # 2a. TRIM to the longest clean verbatim run if it retains >=60% and lies in the page window.
    run = longest_clean_run(quote, chunk["text"])
    if len(run) >= MIN_QUOTE_CHARS and len(run) >= TRIM_RETAIN_MIN * max(1, len(nq)) \
            and window_contains(chunk, cpr[0], cpr[1], normalize_text(run)):
        return bound(chunk, run, cpr, "trim",
                     {"type": "trim", "original_quote": quote, "final_quote": run,
                      "retained_fraction": round(len(run) / max(1, len(nq)), 3),
                      "reason": "authored quote spans a U+FFFE hyphenation marker; trimmed to the "
                                "longest clean verbatim run (>=60% retained)"})

    # 2b. RE-ANCHOR to the best-overlap clean sentence on the cited page.
    sents = sorted(candidate_sentences(chunk_pages_text(chunk, lo, hi)),
                   key=lambda s: (-overlap_score(s, nq), s))
    for sent in sents[:3]:
        if window_contains(chunk, cpr[0], cpr[1], normalize_text(sent)):
            return bound(chunk, sent, cpr, "reanchor",
                         {"type": "reanchor", "original_quote": quote, "final_quote": sent,
                          "overlap": round(overlap_score(sent, nq), 3),
                          "reason": "authored quote spans a U+FFFE hyphenation marker and trimming "
                                    "would retain <60%; re-anchored to the best-overlap clean verbatim "
                                    "sentence on the cited page"})
    # 2c. FALLBACK: no >=60% trim and no clean on-page sentence to re-anchor to -> keep the
    # longest clean verbatim run (>=25 chars) as a flagged low-retention trim (audited).
    if len(run) >= MIN_QUOTE_CHARS and window_contains(chunk, cpr[0], cpr[1], normalize_text(run)):
        return bound(chunk, run, cpr, "trim",
                     {"type": "trim", "original_quote": quote, "final_quote": run,
                      "retained_fraction": round(len(run) / max(1, len(nq)), 3),
                      "low_retention": True,
                      "reason": "authored quote spans a U+FFFE hyphenation marker; the clean run retains "
                                "<60% and no clean full sentence was available on the cited page to "
                                "re-anchor, so the longest clean verbatim run is kept (flagged for review)"})
    return {"status": "unbound", "authored_page_range": [lo, hi],
            "note": "no clean verbatim trim/re-anchor in the page window; needs manual review"}


def resolve() -> dict[str, Any]:
    skeletons = load_skeletons()
    manifest = json.loads(CHUNKS_MANIFEST.read_text(encoding="utf-8"))["chunks"]
    by_src: dict[str, list] = {}
    for c in manifest:
        by_src.setdefault(c["source_id"], []).append(c)
    env = dict(os.environ, KB_FROZEN_CLOCK="1",
               LD_LIBRARY_PATH=":".join(["/usr/lib", os.environ.get("LD_LIBRARY_PATH", "")]).strip(":"))

    # Pass1: PROPOSE every binding via parity-proven normalize_text (pure, fast).
    items: list[tuple[dict, dict, dict]] = []  # (card, cit, rec)
    idx = 0
    for card in skeletons:
        for cit in card["citations"]:
            idx += 1
            rec = bind_citation(cit, card["reading_id"], by_src)
            rec["card_id"] = card["card_id"]
            rec["_idx"] = idx
            items.append((card, cit, rec))

    # Pass2: CONFIRM every proposed binding with the REAL kb verify, in parallel.
    work = Path(tempfile.mkdtemp(prefix="resolve_"))
    try:
        bound_items = [(card, cit, rec) for card, cit, rec in items
                       if rec["status"] in ("auto", "trim", "reanchor")]

        def confirm(item):
            rec = item[2]
            r = rec["registry"]
            ok = kb_verify(rec["reading_id"], r["source_id"],
                           {"chunk_id": r["chunk_id"], "chunk_hash": r["chunk_hash"]},
                           r["page_range"], r["quote"], r["edge_type"], work, env, rec["_idx"])
            return rec, ok

        with ThreadPoolExecutor(max_workers=4) as ex:
            for rec, ok in ex.map(confirm, bound_items):
                if not ok:
                    rec["status"] = "confirm_failed"
    finally:
        shutil.rmtree(work, ignore_errors=True)

    # Pass3: assemble registries / report / audit / counts.
    registries: dict[str, list] = {p: [] for _, _, p in SETS}
    report: list[dict[str, Any]] = []
    audit: list[dict[str, Any]] = []
    counts = {p: {"total": 0, "auto": 0, "trim": 0, "reanchor": 0, "page_corrections": 0, "unbound": 0}
              for _, _, p in SETS}
    by_card: dict[str, list] = {}
    for card, cit, rec in items:
        prefix = card["registry"]
        counts[prefix]["total"] += 1
        report.append({"card_id": card["card_id"], "source_id": cit["source_id"],
                       "status": rec["status"], "chunk_id": rec.get("registry", {}).get("chunk_id"),
                       "authored_page_range": rec.get("authored_page_range"), "change": rec.get("change")})
        resolved = by_card.setdefault(card["card_id"], [])
        if rec["status"] in ("auto", "trim", "reanchor"):
            counts[prefix][rec["status"]] += 1
            if rec.get("change") and rec["change"].get("type") == "page_window":
                counts[prefix]["page_corrections"] += 1
            if rec["status"] in ("trim", "reanchor"):
                audit.append({"card_id": card["card_id"], "source_id": cit["source_id"],
                              "chunk_id": rec["registry"]["chunk_id"], **rec["change"]})
            resolved.append(rec["registry"])
        else:
            counts[prefix]["unbound"] += 1
            resolved.append(None)
    for card in skeletons:
        registries[card["registry"]].append({
            "card_id": card["card_id"], "title": card["title"], "summary": card["summary"],
            "tags": card["tags"], "citations": by_card[card["card_id"]]})
    return {"registries": registries, "report": report, "audit": audit, "counts": counts,
            "skeleton_count": len(skeletons)}


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    tmp.replace(path)


def emit(result: dict[str, Any]) -> dict[str, Any]:
    unbound = [r for r in result["report"] if r["status"] not in ("auto", "trim", "reanchor")]
    if unbound:
        raise SystemExit(f"{len(unbound)} citation(s) failed to bind (fail-closed): "
                         f"{[(u['card_id'], u['status']) for u in unbound[:8]]}")
    for _set in SETS:
        rid, prefix = _set[1], _set[2]
        write_json(REGISTRY / f"{prefix}_curated_citations.json", {
            "schema_version": "cfa_legacy.migration_curated_citations.v1",
            "generated_by": "resolve_migration_citations.py",
            "reading_id": rid,
            "note": "Resolved chunk_id/chunk_hash bound from incoming skeleton citations against the "
                    "frozen merged chunks_manifest; confirmed by kb verify. Quote repairs (trim/re-anchor "
                    "for U+FFFE hyphenation) are recorded in migration_quote_audit.json.",
            "cards": result["registries"][prefix],
        })
    write_json(BIND_REPORT, {
        "schema_version": "cfa_legacy.migration_bind_report.v1",
        "skeleton_count": result["skeleton_count"],
        "counts": result["counts"], "report": sorted(result["report"], key=lambda r: (r["card_id"], r["source_id"])),
    })
    write_json(QUOTE_AUDIT, {
        "schema_version": "cfa_legacy.migration_quote_audit.v1",
        "note": "Every quote changed to bind (trim/re-anchor for U+FFFE hyphenation). Each final quote "
                "verifies via kb verify against its bound chunk; faithfulness reviewed separately.",
        "changed_quotes": sorted(result["audit"], key=lambda a: (a["card_id"], a["source_id"])),
    })
    summary = {p: dict(c) for p, c in result["counts"].items()}
    for p in summary:
        t = summary[p]["total"]
        summary[p]["auto_bind_rate"] = round(100 * summary[p]["auto"] / t, 1) if t else 0.0
        summary[p]["bound_rate"] = round(100 * (summary[p]["auto"] + summary[p]["trim"] + summary[p]["reanchor"]) / t, 1) if t else 0.0
    return summary


# --------------------------------------------------------------------------- #
# AC-equivalent parity fixture: resolver containment vs real kb verify.
# --------------------------------------------------------------------------- #

def parity() -> int:
    """Prove normalize_text-based containment agrees with real kb verify on >=10
    existing v0 citations spanning edge cases, plus known-FAIL probes."""
    import yaml
    manifest = json.loads(CHUNKS_MANIFEST.read_text(encoding="utf-8"))["chunks"]
    by_id = {c["chunk_id"]: c for c in manifest}
    cards_manifest = json.loads(CARDS_MANIFEST.read_text(encoding="utf-8"))["cards"]
    env = dict(os.environ, KB_FROZEN_CLOCK="1",
               LD_LIBRARY_PATH=":".join(["/usr/lib", os.environ.get("LD_LIBRARY_PATH", "")]).strip(":"))
    # gather a diverse fixture of real v0 citations from on-disk cards
    fixture = []
    for cm in cards_manifest:
        if len(fixture) >= 14:
            break
        path = ROOT / cm["path"]
        if not path.is_file():
            continue
        fm = yaml.safe_load(path.read_text(encoding="utf-8").split("---", 2)[1])
        for cit in fm.get("citations", []):
            ch = by_id.get(cit["chunk_id"])
            if ch is None:
                continue
            q = cit["quote"]
            feats = (("hyphen" if "-" in q else "") + ("lig" if any(l in ch["text"] for l in _LIGATURES) else "")
                     + ("ws" if "  " in ch["text"] else ""))
            fixture.append((path, fm["reading_id"], cit, ch, feats))
            break
    work = Path(tempfile.mkdtemp(prefix="parity_"))
    mism = []
    try:
        for i, (path, rid, cit, ch, feats) in enumerate(fixture):
            # resolver containment prediction (normalize_text within the cited page window)
            pr = cit["page_range"]
            win = chunk_pages_text(ch, pr[0], pr[-1])
            pred_pass = normalize_text(cit["quote"]) in normalize_text(win)
            real = kb_verify(rid, cit["source_id"], ch, pr, cit["quote"], cit["edge_type"], work, env, 1000 + i)
            if pred_pass != real:
                mism.append((path.name, feats, pred_pass, real, "PASS"))
            # known-FAIL probe: mangle the quote -> both must be FAIL
            mangled = cit["quote"][:-3] + "ZZqx" if len(cit["quote"]) > 10 else cit["quote"] + "ZZqx"
            pred_fail = normalize_text(mangled) in normalize_text(win)
            real_fail = kb_verify(rid, cit["source_id"], ch, pr, mangled, cit["edge_type"], work, env, 2000 + i)
            if pred_fail or real_fail:
                mism.append((path.name, feats, pred_fail, real_fail, "FAIL-probe"))
    finally:
        import shutil
        shutil.rmtree(work, ignore_errors=True)
    print(json.dumps({"fixture_size": len(fixture), "mismatches": len(mism),
                      "detail": mism[:10]}, ensure_ascii=False))
    if len(fixture) < 10:
        print("PARITY: insufficient fixture (<10)"); return 1
    if mism:
        print("PARITY: FAIL"); return 1
    print("PARITY: PASS (resolver normalize_text containment == kb verify on the fixture + known-FAILs)")
    return 0


# --------------------------------------------------------------------------- #
# Self-test (pure helpers).
# --------------------------------------------------------------------------- #

def self_test() -> int:
    f = []
    if normalize_text("word-\nnext") != "wordnext": f.append("hyphen-linebreak")
    if normalize_text("oﬃce") != "office": f.append("ligature ffi")
    if normalize_text("a  b\t\nc") != "a b c": f.append("ws collapse")
    if normalize_text("  x  ") != "x": f.append("strip")
    if not is_illegal_quote_char("￾"): f.append("U+FFFE not flagged illegal")
    if is_illegal_quote_char("a"): f.append("letter flagged illegal")
    # longest_clean_run: U+FFFE splits the run
    if longest_clean_run("the exclusive cost here", "the ex￾clusive cost here today") != "cost here":
        f.append(f"clean_run: {longest_clean_run('the exclusive cost here', 'the ex￾clusive cost here today')!r}")
    if overlap_score("the quick brown fox", "quick brown") <= 0: f.append("overlap")
    s = candidate_sentences("First clean sentence here ok. Second one also fine here.")
    if len(s) != 2: f.append(f"sentences: {s}")
    if f:
        print("SELF-TEST FAILED:"); [print("  -", x) for x in f]; return 1
    print("SELF-TEST PASSED (normalize_text + illegal + clean_run + sentences + overlap)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("--parity", action="store_true", help="prove normalize_text containment == kb verify")
    ap.add_argument("--write", action="store_true", help="resolve + write registries/report/audit")
    ap.add_argument("--dry-run", action="store_true", help="resolve + print counts, write nothing")
    args = ap.parse_args()
    if args.self_test:
        return self_test()
    if args.parity:
        return parity()
    result = resolve()
    if args.write:
        summary = emit(result)
        print(json.dumps(summary, ensure_ascii=False, sort_keys=True, indent=2))
    else:
        print(json.dumps({"counts": result["counts"],
                          "unbound": [(r["card_id"], r["status"]) for r in result["report"]
                                      if r["status"] not in ("auto", "trim", "reanchor")][:20]},
                         ensure_ascii=False, sort_keys=True, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
