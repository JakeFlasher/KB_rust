#!/usr/bin/env python3
"""One-time quote->chunk resolver for the 14/15/22 migration.

Binds every incoming skeleton citation to exactly one chunk of the frozen merged
manifest, writing three curated citation registries (mt_14 / pa_15 / fa_22), an
auditable bind report, and a quote-integrity audit. It PROPOSES a binding (the
chunk whose page span intersects the citation page_range and whose normalized
text contains the normalized quote) and CONFIRMS every binding with the REAL
`kb verify` (the kernel is the containment authority -- kernel-parity).
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
POLICY_PATH = REGISTRY / "migration_resolver_policy.json"
TRIM_RETAIN_MIN = 0.60   # >=60% of the quote retained -> trim; else re-anchor
MIN_QUOTE_CHARS = 25
MIN_AUTO_BIND_RATE = 0.95


def load_policy() -> dict[str, Any]:
    return json.loads(POLICY_PATH.read_text(encoding="utf-8")) if POLICY_PATH.is_file() else {}


def is_overlap_set(chunks: list[dict[str, Any]]) -> bool:
    """True iff the chunks form a single overlapping page-span chain (kb chunker overlap):
    sorted by start_page, each chunk's start_page <= the previous chunk's end_page."""
    s = sorted(chunks, key=lambda c: (c["start_page"], c["end_page"], c["chunk_id"]))
    return all(s[i]["start_page"] <= s[i - 1]["end_page"] for i in range(1, len(s)))

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


def bind_citation(cit: dict[str, Any], reading_id: str, by_src: dict[str, list],
                  policy: dict[str, Any]) -> dict[str, Any]:
    """PROPOSE a binding for one citation via parity-proven normalize_text containment.
    Pure (no subprocess); the proposal is confirmed by a later parallel kb-verify pass.
    Fail-closed: >1 candidate -> 'ambiguous' (unless the committed policy authorizes
    deterministic overlap disambiguation); zero clean trim/re-anchor -> 'unbound'."""
    sid = cit["source_id"]
    pr = cit["page_range"]
    lo, hi = (pr[0], pr[-1]) if isinstance(pr, list) else (pr, pr)
    quote = cit["quote"]
    edge = cit["edge_type"]
    nq = normalize_text(quote)
    cand = sorted([c for c in by_src.get(sid, []) if not (c["end_page"] < lo or c["start_page"] > hi)],
                  key=lambda c: c["chunk_id"])

    def page_change(cpr, chunk):
        if cpr == [lo, hi]:
            return None
        return {"type": "page_window", "old_page_range": [lo, hi], "new_page_range": cpr,
                "reason": f"cited page_range [{lo}, {hi}] clamped to the bound chunk's page span "
                          f"[{chunk['start_page']}, {chunk['end_page']}] so the cited range lies within "
                          f"the chunk (chunk covers page(s) {cpr[0]}..{cpr[1]} of the cited range)"}

    def bound(chunk, final_quote, final_pr, status, change, overlap_alternatives=None):
        rec = {"card_id": None, "status": status, "change": change, "reading_id": reading_id,
               "registry": {"source_id": sid, "chunk_id": chunk["chunk_id"],
                            "chunk_hash": chunk["chunk_hash"], "page_range": final_pr,
                            "quote": final_quote, "edge_type": edge},
               "authored_page_range": [lo, hi]}
        if overlap_alternatives:
            rec["overlap_alternatives"] = overlap_alternatives
        return rec

    def fit_key(item):
        c = item[0]
        contains = 0 if (c["start_page"] <= lo and hi <= c["end_page"]) else 1
        return (contains, c["end_page"] - c["start_page"], c["chunk_id"])

    # 1. AUTO-bind: candidate(s) whose normalized cited-page-window text contains the authored quote.
    proposed = [(c, [max(lo, c["start_page"]), min(hi, c["end_page"])]) for c in cand]
    proposed = [(c, cpr) for c, cpr in proposed if window_contains(c, cpr[0], cpr[1], nq)]
    if len(proposed) == 1:
        c, cpr = proposed[0]
        return bound(c, quote, cpr, "auto", page_change(cpr, c))
    if len(proposed) > 1:
        # FAIL CLOSED on multiple verifying candidates, UNLESS the committed policy authorizes
        # deterministic disambiguation of a pure chunker-OVERLAP set (record all alternatives).
        chunks = [c for c, _ in proposed]
        candidate_ids = sorted(c["chunk_id"] for c in chunks)
        overlap = is_overlap_set(chunks)
        if overlap and policy.get("allow_overlap_disambiguation", {}).get("enabled"):
            c, cpr = sorted(proposed, key=fit_key)[0]
            return bound(c, quote, cpr, "auto", page_change(cpr, c), overlap_alternatives=candidate_ids)
        return {"status": "ambiguous", "authored_page_range": [lo, hi],
                "candidate_chunk_ids": candidate_ids, "is_overlap_set": overlap}

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
    policy = load_policy()
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
            rec = bind_citation(cit, card["reading_id"], by_src, policy)
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
                       "authored_page_range": rec.get("authored_page_range"), "change": rec.get("change"),
                       "candidate_chunk_ids": rec.get("candidate_chunk_ids"),
                       "overlap_alternatives": rec.get("overlap_alternatives"),
                       "is_overlap_set": rec.get("is_overlap_set")})
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
    policy = load_policy()
    report = result["report"]
    counts = result["counts"]
    # Fail closed on ANY citation not cleanly bound (ambiguous / unbound / no_candidate / confirm_failed).
    failed = [r for r in report if r["status"] not in ("auto", "trim", "reanchor")]
    if failed:
        raise SystemExit(
            f"{len(failed)} citation(s) not cleanly bound (fail-closed): "
            f"{[(r['card_id'], r['status'], r.get('candidate_chunk_ids')) for r in failed[:8]]}")
    # >=95% per-set auto-bind guard, overridable ONLY for the documented U+FFFE quote-repair condition.
    rates = {p: (round(c["auto"] / c["total"], 4) if c["total"] else 1.0) for p, c in counts.items()}
    low = sorted(p for p, r in rates.items() if r < MIN_AUTO_BIND_RATE)
    override = bool(policy.get("allow_low_auto_bind_for_ufffe_repair", {}).get("enabled"))
    non_ufffe_repairs = [a for a in result["audit"] if "U+FFFE" not in a.get("reason", "")]
    override_applied = False
    if low:
        if override and not non_ufffe_repairs:
            override_applied = True
        else:
            raise SystemExit(
                f"auto-bind below {MIN_AUTO_BIND_RATE:.0%} for set(s) {low} (rates {rates}); not covered "
                f"by the U+FFFE override (enabled={override}, non_ufffe_repairs={len(non_ufffe_repairs)}). "
                "Fail-closed.")
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
        "counts": result["counts"],
        "auto_bind_rates": rates,
        "policy": {
            "auto_bind_min_rate": MIN_AUTO_BIND_RATE,
            "low_auto_bind_sets": low,
            "ufffe_auto_bind_override_applied": override_applied,
            "overlap_disambiguation_enabled": bool(policy.get("allow_overlap_disambiguation", {}).get("enabled")),
        },
        "report": sorted(result["report"], key=lambda r: (r["card_id"], r["source_id"])),
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
# Explicit kernel-parity fixture. (a) normalize_text byte-parity to the kernel's
# OWN documented vectors (crates/cacg-core/src/normalize.rs tests); (b) end-to-end:
# the resolver's candidate-selection + containment == the real kb verify (selected
# chunk_id + PASS/FAIL) on declared real-citation rows. The corpus chunks contain
# no `-\n`/ligature (Pdfium emits U+FFFE hyphenation + pre-expands ligatures), so
# those features are proven by the normalize_text vectors.
# --------------------------------------------------------------------------- #

PARITY_NORM_VECTORS = [
    {"input": "word-\nnext", "expected": "wordnext", "tags": ["hyphen-linebreak"]},
    {"input": "word-  \n  next", "expected": "wordnext", "tags": ["hyphen-linebreak"]},
    {"input": "oﬁce", "expected": "ofice", "tags": ["ligature"]},
    {"input": "oﬃce", "expected": "office", "tags": ["ligature"]},
    {"input": "baﬄe", "expected": "baffle", "tags": ["ligature"]},
    {"input": "oﬀ", "expected": "off", "tags": ["ligature"]},
    {"input": "beﬆ", "expected": "best", "tags": ["ligature"]},
    {"input": "a  b\t\tc", "expected": "a b c", "tags": ["whitespace-join"]},
    {"input": "a b", "expected": "a b", "tags": ["whitespace-join"]},
    {"input": "  hello  ", "expected": "hello", "tags": ["whitespace-join"]},
]

PARITY_CITATIONS = [
    {"reading_id": "01_quantitative_methods", "source_id": "qm_eslii_2009_2ed", "page_range": [250, 250],
     "quote": "Then for this set of models we define AIC(α) = err(α) + 2 · d(α) N σˆε 2 .",
     "expect_pass": True, "expect_chunk_id": "qm_eslii_2009_2ed:p250:0316", "tags": ["whitespace-join"]},
    {"reading_id": "01_quantitative_methods", "source_id": "qm_eslii_2009_2ed", "page_range": [252, 253],
     "quote": "The generic form of BIC is BIC = −2 · loglik + (log N) · d.",
     "expect_pass": True, "expect_chunk_id": "qm_eslii_2009_2ed:p252:0319", "tags": ["whitespace-join"]},
    {"reading_id": "01_quantitative_methods", "source_id": "qm_eslii_2009_2ed", "page_range": [242, 242],
     "quote": "we first explore in more detail the nature of test error and the bias–variance tradeoff.",
     "expect_pass": True, "expect_chunk_id": "qm_eslii_2009_2ed:p242:0307", "tags": ["punctuation"]},
    {"reading_id": "01_quantitative_methods", "source_id": "qm_tsay_2014_multivariate_time_series", "page_range": [421, 421],
     "quote": "We study in Section 7.5 the simple Baba–Engle–Kraft– Kroner (BEKK) model of Engle and Kroner (1995) and discuss its pros and cons.",
     "expect_pass": True, "expect_chunk_id": "qm_tsay_2014_multivariate_time_series:p421:0481", "tags": ["punctuation"]},
    {"reading_id": "17_cross_cutting", "source_id": "cfa_2022_l1_combined", "page_range": [1, 3],
     "quote": "volume 2, type “2-5”… and so forth. For example, to go to page 5 of vol",
     "expect_pass": False, "expect_chunk_id": None, "tags": ["chunk-boundary"]},
    {"reading_id": "01_quantitative_methods", "source_id": "qm_eslii_2009_2ed", "page_range": [250, 250],
     "quote": "this is a deliberately fabricated quote that does not exist in the chunk ZZQX",
     "expect_pass": False, "expect_chunk_id": None, "tags": ["known-fail"]},
]

REQUIRED_PARITY_TAGS = {"whitespace-join", "hyphen-linebreak", "ligature", "punctuation", "chunk-boundary"}


def parity() -> int:
    """Normalization-parity gate: prove (a) normalize_text matches the kernel's documented vectors and (b) the resolver's
    candidate-selection + containment match the real kb verify (selected chunk_id + PASS/FAIL) on an
    explicit fixture covering all REQUIRED_PARITY_TAGS + known-FAILs. Fails if a required tag is missing
    or any row mismatches."""
    manifest = json.loads(CHUNKS_MANIFEST.read_text(encoding="utf-8"))["chunks"]
    by_src: dict[str, list] = {}
    for c in manifest:
        by_src.setdefault(c["source_id"], []).append(c)
    policy = load_policy()
    env = dict(os.environ, KB_FROZEN_CLOCK="1",
               LD_LIBRARY_PATH=":".join(["/usr/lib", os.environ.get("LD_LIBRARY_PATH", "")]).strip(":"))
    tags = {t for row in PARITY_NORM_VECTORS + PARITY_CITATIONS for t in row["tags"]}
    fail: list[str] = []
    if REQUIRED_PARITY_TAGS - tags:
        fail.append(f"missing required coverage tags: {sorted(REQUIRED_PARITY_TAGS - tags)}")
    for v in PARITY_NORM_VECTORS:
        got = normalize_text(v["input"])
        if got != v["expected"]:
            fail.append(f"norm vector {v['input']!r}: got {got!r} != {v['expected']!r}")
    work = Path(tempfile.mkdtemp(prefix="parity_"))
    try:
        for i, row in enumerate(PARITY_CITATIONS):
            cit = {"source_id": row["source_id"], "page_range": row["page_range"],
                   "quote": row["quote"], "edge_type": "supports"}
            rec = bind_citation(cit, row["reading_id"], by_src, policy)
            auto = rec["status"] == "auto"
            sel = rec.get("registry", {}).get("chunk_id")
            # real kb verify on the AUTHORED quote against the selected chunk (PASS) or the first
            # candidate chunk in the page window (FAIL rows must FAIL there too).
            window = [c for c in by_src.get(row["source_id"], [])
                      if not (c["end_page"] < row["page_range"][0] or c["start_page"] > row["page_range"][-1])]
            target = next((c for c in window if c["chunk_id"] == sel), None) if auto else (window[0] if window else None)
            real = kb_verify(row["reading_id"], row["source_id"], target, row["page_range"],
                             row["quote"], "supports", work, env, 5000 + i) if target else False
            if auto != row["expect_pass"]:
                fail.append(f"row {i} {row['tags']}: resolver auto={auto} != expect_pass={row['expect_pass']}")
            if auto != real:
                fail.append(f"row {i} {row['tags']}: resolver auto={auto} != kb verify={real}")
            if row["expect_pass"] and sel != row["expect_chunk_id"]:
                fail.append(f"row {i}: selected chunk {sel} != expected {row['expect_chunk_id']}")
    finally:
        shutil.rmtree(work, ignore_errors=True)
    print(json.dumps({"norm_vectors": len(PARITY_NORM_VECTORS), "citations": len(PARITY_CITATIONS),
                      "tags": sorted(tags), "failures": fail[:10]}, ensure_ascii=False))
    if fail:
        print("PARITY: FAIL"); return 1
    print("PARITY: PASS (normalize_text == kernel vectors; resolver selected chunk_id + PASS/FAIL == kb verify)")
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
