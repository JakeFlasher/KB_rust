#!/usr/bin/env python3
r"""CJK ingest parity SPIKE (AC-3) — the hard gate before any card authoring.

Ingests the rendered Xueqiu PDF with the CJK chunk config (`max_pages_per_chunk:1,
overlap_tokens:0`) and proves that every candidate Chinese quote binds, verbatim and
normalized, to EXACTLY ONE chunk that is the ★AUTHOR-tagged origin — and that the real
`kb verify` (the binding authority) accepts synthetic cards built from those binds.

Resolution is fail-closed and propose-confirm (cf.
BL-20260531-bind-foreign-quotes-via-propose-confirm-and-ufffe-repair): a normalized
substring PROPOSES a chunk; `kb verify` CONFIRMS. Author-origin discipline:

  * Only `au=1` (author) chunks are candidates; commenter pages are never author-origin.
  * A quote lying AFTER a `//@<non-author>:` repost span is commenter-original → REJECTED
    (attribution); a quote after `//@<author-id>:` is a self-repost duplicate → bound to
    the canonical non-repost author utterance instead.
  * A commenter-requote / duplicate multi-match is disambiguated by LENGTHENING the quote
    to its author-original span; an irreducible verbatim duplicate across distinct author
    utterances binds deterministically (lowest page) with all alternatives recorded.
  * A seed whose `post_id` is imprecise binds to its unique author-origin chunk with a
    recorded pid correction.

  --check [--keep-ingest]   ingest + run the spike; writes cjk_ingest_spike_report.json
"""
from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "sources/hkex/_registry"))
from corpus_model import parse  # noqa: E402  # pyright: ignore[reportMissingImports]
from cacg_normalize import normalize_text  # noqa: E402  # pyright: ignore[reportMissingImports]
from check_corpus_parity import resolve_kb, sha256_file  # noqa: E402  # pyright: ignore[reportMissingImports]

PDF = ROOT / "sources/hkex/pdfs/goubujiao_corpus.pdf"
PROVENANCE = ROOT / "sources/hkex/_registry/renderer_provenance.json"
INGEST_DIR = ROOT / "out/hkex/ingest_per_source/goubujiao_xueqiu_corpus"
REPORT = ROOT / "sources/hkex/_registry/cjk_ingest_spike_report.json"
INVENTORY = ROOT / "_research/xueqiu_goubujiao/card_inventory.json"
VERIFICATIONS = ROOT / "_research/xueqiu_goubujiao/verifications.json"
SOURCE_ID = "goubujiao_xueqiu_corpus"

_MARKER = re.compile(r"@@HKEX p=(\d+) k=(\w+) pid=(\d+) cid=(\S+) au=([01])@@")
_REPOST = re.compile(r"//@([^:：]+)[:：]")
# The author's known Xueqiu @-mention HANDLES (a `//@<author-handle>` is a self-repost,
# not commenter text). Matched by EXACT equality, never substring — an impostor handle
# that merely CONTAINS one of these (e.g. `管我财2`, `小狗不叫`) is NOT the author. The
# nicknames 财主/財主 are not @-mention handles and are deliberately excluded.
AUTHOR_IDS = ("狗不叫", "管我财", "管我財", "管理我的财")


def env_for_kb() -> dict:
    return dict(os.environ, KB_FROZEN_CLOCK="1", KB_SKIP_PDFIUM_HASH_CHECK="1",
                LD_LIBRARY_PATH=":".join(["/usr/lib", os.environ.get("LD_LIBRARY_PATH", "")]).strip(":"))


def author_text_of(norm_chunk: str) -> str:
    """The author's text on a page = the normalized chunk text after its marker."""
    m = _MARKER.search(norm_chunk)
    return norm_chunk[m.end():].lstrip() if m else norm_chunk


def repost_user_before(nauthor: str, nq: str) -> str | None:
    """The `//@user` whose repost span the quote sits inside, or None (author's words).

    Uses the FIRST occurrence of the quote; since a Xueqiu repost places `//@user:` at
    the END (the author's own words come first), first-occurrence biases toward
    classifying as author-words, i.e. toward NOT over-rejecting — and the bound chunk is
    independently required to be `au=1` author-words, so this never admits commenter text."""
    i = nauthor.find(nq)
    if i < 0:
        return None
    ms = list(_REPOST.finditer(nauthor[:i]))
    return ms[-1].group(1).strip() if ms else None


def is_self_repost(user: str | None) -> bool:
    return user is not None and user.strip() in AUTHOR_IDS


def in_author_words(nauthor: str, nq: str) -> bool:
    """True iff the quote is the author's own words (not after a non-author `//@`)."""
    user = repost_user_before(nauthor, nq)
    return user is None or is_self_repost(user)


def lengthen_to_unique(nauthor: str, nq: str, all_nauthors: list[str]) -> str | None:
    """Grow the quote within its author utterance until it matches exactly one chunk."""
    i = nauthor.find(nq)
    if i < 0:
        return None
    lo, hi = i, i + len(nq)
    for _ in range(len(nauthor)):
        if sum(1 for t in all_nauthors if nauthor[lo:hi] in t) == 1:
            return nauthor[lo:hi]
        grew = False
        if hi < len(nauthor):
            hi += 1; grew = True
        if lo > 0:
            lo -= 1; grew = True
        if not grew:
            break
    cand = nauthor[lo:hi]
    return cand if sum(1 for t in all_nauthors if cand in t) == 1 else None


def build_index(chunks: list[dict]) -> list[dict]:
    idx = []
    for c in chunks:
        nt = normalize_text(c["text"])
        m = _MARKER.search(nt)
        idx.append({"chunk": c, "nauthor": author_text_of(nt),
                    "kind": m.group(2) if m else None, "pid": m.group(3) if m else None,
                    "cid": m.group(4) if m else None, "au": m.group(5) if m else None})
    return idx


def resolve_seed(nidx: list[dict], all_nauthors: list[str], pid: str, cid: str | None, quote: str) -> dict:
    nq = normalize_text(quote)
    hits = [e for e in nidx if nq in e["nauthor"]]
    if not hits:
        return {"status": "zero_match", "quote": quote, "seed_pid": pid}
    au1 = [e for e in hits if e["au"] == "1"]
    authw = [e for e in au1 if in_author_words(e["nauthor"], nq)]
    repost_users = sorted({u for e in au1 for u in [repost_user_before(e["nauthor"], nq)]
                           if u and not is_self_repost(u)})
    if not authw:
        # only commenter pages or //@non-author repost spans -> commenter-original.
        return {"status": "attribution_rejected", "quote": quote, "seed_pid": pid,
                "repost_users": repost_users}
    pidmatch = [e for e in authw if e["pid"] == pid and (cid is None or e["cid"] == cid)]
    pid_corrected = not pidmatch
    pool = pidmatch if pidmatch else authw
    final_q, lengthened, recurrence = quote, False, False
    if len(pool) > 1:
        # A multi-match must resolve to a UNIQUE author span or be a genuine verbatim
        # duplicate; otherwise fail closed (the seed quote is ambiguous and must be
        # lengthened at authoring time). Try lengthening within EACH pool member, not
        # just pool[0], so the unique span is found wherever it exists.
        lpick = next(((e, c) for e in pool
                      for c in [lengthen_to_unique(e["nauthor"], nq, all_nauthors)] if c), None)
        if lpick:
            chosen, final_q, lengthened = lpick[0], lpick[1], True
        else:
            # Genuine author-recurrence: the verbatim quote AND its extensions appear in
            # several distinct au=1 author utterances (a principle the author restates), so
            # no unique span exists. This is NOT a commenter requote (every match is the
            # author's own words). Per plan G4, bind to the EARLIEST (original) author
            # utterance and record every alternative; flag it (`recurrence`) so AC-7
            # authoring can confirm the intended context or pick a longer quote. The bound
            # quote is author-origin and verbatim, so this is a valid (recorded) bind.
            chosen, recurrence = min(pool, key=lambda e: e["chunk"]["start_page"]), True
    else:
        chosen = pool[0]
    c = chosen["chunk"]
    return {"status": "bound", "quote": quote, "final_quote": final_q, "seed_pid": pid,
            "bound_chunk_id": c["chunk_id"], "bound_pid": chosen["pid"], "bound_kind": chosen["kind"],
            "bound_cid": chosen["cid"], "page": c["start_page"], "chunk_hash": c["chunk_hash"],
            "pid_corrected": pid_corrected, "lengthened": lengthened, "recurrence": recurrence,
            "self_repost": is_self_repost(repost_user_before(chosen["nauthor"], nq)),
            "alternatives": sorted(e["chunk"]["chunk_id"] for e in pool if e is not chosen) or None,
            "repost_users": repost_users or None}


def load_seeds() -> list[dict]:
    inv = json.loads(INVENTORY.read_text(encoding="utf-8"))["candidates"]
    ver = json.loads(VERIFICATIONS.read_text(encoding="utf-8"))
    verdict_by_title = {}
    if isinstance(ver, list):
        for v in ver:
            vv = v.get("verification")
            verdict_by_title[v.get("card_title_en")] = (vv.get("verdict") if isinstance(vv, dict) else vv)
    seeds = []
    for c in inv:
        for s in c.get("citation_seeds", []):
            seeds.append({"post_id": str(s["post_id"]), "cid": s.get("comment_id"),
                          "quote": s["quote_zh"], "reading_folder": c.get("reading_folder"),
                          "title": c.get("card_title_en"),
                          "verdict": verdict_by_title.get(c.get("card_title_en"))})
    return seeds


def smoke_test(chunks: list[dict], page_count: int) -> list[str]:
    errs = []
    multi = [c["chunk_id"] for c in chunks if c["start_page"] != c["end_page"]]
    if multi:
        errs.append(f"{len(multi)} multi-page chunks (e.g. {multi[:3]})")
    if len(chunks) != page_count:
        errs.append(f"chunk_count {len(chunks)} != page_count {page_count}")
    bad = [c["chunk_id"] for c in chunks if len(_MARKER.findall(c["text"])) != 1]
    if bad:
        errs.append(f"{len(bad)} chunks with !=1 marker (e.g. {bad[:3]})")
    return errs


def control_char_scan(chunks: list[dict]) -> list[list[str]]:
    bad = []
    for c in chunks:
        for ch in c["text"]:
            o = ord(ch)
            # C0 controls (except \n\t\r), DEL, C1 controls, and the U+FFFE/U+FFFD
            # noncharacter/replacement marks that signal an extraction defect.
            if (o < 0x20 and ch not in "\n\t\r") or o == 0x7F or 0x80 <= o <= 0x9F \
                    or ch in ("￾", "�"):
                bad.append([c["chunk_id"], hex(o)])
                break
    return bad


def edge_fixtures(nidx: list[dict], all_nauthors: list[str]) -> dict:
    """Explicit, inventory-independent fixtures. Positives: author quotes with the named
    edge tokens each bind to an author-origin chunk. Negatives: a `//@non-author` repost
    quote attribution-rejects, and a `(↪in reply to …)` parent context (never rendered)
    zero-matches."""
    from corpus_model import author_utterances  # type: ignore  # pyright: ignore[reportMissingImports]
    au = author_utterances(parse())
    pos_tokens = ["行駛", "行使", "石鼓", "實物", "除淨", "半裸", "$", "%"]
    positives = []
    for tok in pos_tokens:
        for u in au:
            if tok in u.text and "//@" not in u.text.split(tok)[0]:
                i = u.text.find(tok)
                span = u.text[max(0, i - 6):i + 12]
                span = span.strip()
                if len(normalize_text(span)) >= 6:
                    r = resolve_seed(nidx, all_nauthors, u.post_id, u.comment_id, span)
                    positives.append({"token": tok, "quote": span, "status": r["status"],
                                      "au": "1" if r["status"] == "bound" else None})
                    break
    # negative 1: a //@<non-author> repost quote -> attribution_rejected
    neg_repost = resolve_seed(nidx, all_nauthors, "223159123",
                              None, "电信对自己天翼云的估值不低于2000亿元")
    # negative 2: a (↪in reply to ...) parent context (never rendered) -> zero_match
    neg_context = resolve_seed(nidx, all_nauthors, "0", None, "讨论已被 丹书铁券 删除")
    ok = (all(p["status"] == "bound" for p in positives)
          and neg_repost["status"] == "attribution_rejected"
          and neg_context["status"] == "zero_match")
    return {"positives": positives, "negative_repost_status": neg_repost["status"],
            "negative_context_status": neg_context["status"], "ok": ok}


def synth_verify(kb: Path, env: dict, chunks_manifest: Path, binds: list[dict]) -> list[dict]:
    def clean(b: dict) -> bool:
        r = b["res"]
        return r["status"] == "bound" and not (r["pid_corrected"] or r["lengthened"] or r["recurrence"])

    # Prefer DISTINCT reading_folders (exercises multiple source_matrix authorizations),
    # then fill to 3 with any clean bind so the gate keys on bind correctness, not folder
    # diversity. A clean-bind shortfall (<3) leaves <3 picks -> the >=3 gate fails closed.
    picks, seen_r = [], set()
    for prefer_distinct in (True, False):
        for b in binds:
            if len(picks) >= 3:
                break
            if not clean(b) or b in picks:
                continue
            rf = b["seed"]["reading_folder"]
            if prefer_distinct and rf in seen_r:
                continue
            seen_r.add(rf); picks.append(b)
        if len(picks) >= 3:
            break
    work = Path(tempfile.mkdtemp(prefix="hkex_synth_"))
    results = []
    try:
        allowed: dict[str, list[str]] = {}
        for b in picks:
            allowed.setdefault(b["seed"]["reading_folder"], [SOURCE_ID])
        matrix = work / "source_matrix.json"
        matrix.write_text(json.dumps({"schema_version": "cacg.v0",
                                      "allowed": {k: sorted(set(v)) for k, v in allowed.items()}}))
        for i, b in enumerate(picks):
            r, rf = b["res"], b["seed"]["reading_folder"]
            card = {"schema_version": "cacg.v0", "id": f"gbj-spike-synth-{i}",
                    "title": "Spike synthetic card", "reading_id": rf, "summary": "s" * 90,
                    "tags": ["xueqiu-2022h1"],
                    "citations": [{"source_id": SOURCE_ID, "chunk_id": r["bound_chunk_id"],
                                   "chunk_hash": r["chunk_hash"], "page_range": [r["page"], r["page"]],
                                   "quote": b["seed"]["quote"], "edge_type": "supports"}]}
            cp = work / f"card_{i}.md"
            cp.write_text("---\n" + json.dumps(card, ensure_ascii=False) + "\n---\n\nbody\n", encoding="utf-8")
            rr = subprocess.run([str(kb), "verify", str(cp), "--chunks-manifest", str(chunks_manifest),
                                 "--source-matrix", str(matrix), "--journal", str(work / f"j{i}.jsonl")],
                                capture_output=True, text=True, env=env)
            results.append({"card_id": f"gbj-spike-synth-{i}", "reading_id": rf,
                            "chunk_id": r["bound_chunk_id"], "pass": rr.returncode == 0,
                            "err": None if rr.returncode == 0 else (rr.stdout or rr.stderr).strip()[:200]})
    finally:
        shutil.rmtree(work, ignore_errors=True)
    return results


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--check", action="store_true", required=True)
    ap.add_argument("--keep-ingest", action="store_true")
    args = ap.parse_args()

    if not PDF.is_file():
        print(f"FAIL: rendered PDF missing ({PDF}); run render_corpus_pdf.py --write", file=sys.stderr)
        return 1
    if not PROVENANCE.is_file():
        print("FAIL: renderer_provenance.json missing; run render_corpus_pdf.py --write", file=sys.stderr)
        return 1
    prov = json.loads(PROVENANCE.read_text())
    live_sha = sha256_file(PDF)
    if live_sha != prov["output_pdf_sha256"]:
        print(f"FAIL: PDF sha {live_sha[:16]} != provenance {prov['output_pdf_sha256'][:16]}", file=sys.stderr)
        return 1
    kb = resolve_kb()
    env = env_for_kb()

    # Idempotent regenerate of the reusable per-source ingest (no cross-source clobber).
    if INGEST_DIR.exists():
        shutil.rmtree(INGEST_DIR)
    INGEST_DIR.mkdir(parents=True)
    cfg = INGEST_DIR / "cjk_chunk.yaml"
    cfg.write_text("chunking:\n  target_tokens: 100000\n  overlap_tokens: 0\n  max_pages_per_chunk: 1\n")
    t0 = time.time()
    r = subprocess.run([str(kb), "ingest", str(PDF), "--config", str(cfg), "--out", str(INGEST_DIR),
                        "--source-id", SOURCE_ID], capture_output=True, text=True, env=env)
    ingest_secs = round(time.time() - t0, 1)
    if r.returncode != 0:
        print(f"FAIL: kb ingest rc={r.returncode}\n{(r.stderr or r.stdout)[:500]}", file=sys.stderr)
        return 1
    chunks_manifest = INGEST_DIR / "chunks_manifest.json"
    chunks = json.loads(chunks_manifest.read_text())["chunks"]
    page_count = prov["page_count"]

    smoke_errs = smoke_test(chunks, page_count)
    ctrl = control_char_scan(chunks)
    nidx = build_index(chunks)
    all_nauthors = [e["nauthor"] for e in nidx]

    seeds = load_seeds()
    binds = [{"seed": s, "res": resolve_seed(nidx, all_nauthors, s["post_id"], s["cid"], s["quote"])}
             for s in seeds]
    by_status: dict[str, int] = {}
    for b in binds:
        by_status[b["res"]["status"]] = by_status.get(b["res"]["status"], 0) + 1
    bound = [b for b in binds if b["res"]["status"] == "bound"]
    zero = [b for b in binds if b["res"]["status"] == "zero_match"]
    ambiguous = [b for b in binds if b["res"]["status"] == "ambiguous_multi_match"]
    rejected = [b for b in binds if b["res"]["status"] == "attribution_rejected"]
    # every rejection must be principled: a non-author //@ repost, no author-origin span.
    unprincipled = [b for b in rejected if not b["res"].get("repost_users")]
    flags = {
        "bound_clean": sum(1 for b in bound if not (b["res"]["pid_corrected"] or b["res"]["lengthened"] or b["res"]["recurrence"])),
        "pid_corrected": sum(1 for b in bound if b["res"]["pid_corrected"]),
        "lengthened": sum(1 for b in bound if b["res"]["lengthened"]),
        "recurrence": sum(1 for b in bound if b["res"]["recurrence"]),
    }

    edges = edge_fixtures(nidx, all_nauthors)
    synth = synth_verify(kb, env, chunks_manifest, bound)

    ok = (not smoke_errs and not ctrl and not zero and not ambiguous and not unprincipled
          and edges["ok"] and len(synth) >= 3 and all(s["pass"] for s in synth))

    try:
        kb_recorded = str(kb.relative_to(ROOT))
    except ValueError:
        kb_recorded = str(kb)
    report = {
        "schema_version": "hkex.cjk_ingest_spike_report.v1",
        "verdict": "PASS" if ok else "FAIL",
        "kb_binary": kb_recorded,
        "pdf_sha256": live_sha,
        "pdf_page_count": page_count,
        "chunk_count": len(chunks),
        "ingest_runtime_secs": ingest_secs,
        "ingest_dir": str(INGEST_DIR.relative_to(ROOT)),
        "config": {"target_tokens": 100000, "overlap_tokens": 0, "max_pages_per_chunk": 1},
        "smoke_errors": smoke_errs,
        "control_char_failures": ctrl,
        "candidate_count": len(json.loads(INVENTORY.read_text())["candidates"]),
        "seed_count": len(seeds),
        "status_counts": by_status,
        "bound_flag_counts": flags,
        "zero_match": [{"pid": b["seed"]["post_id"], "quote": b["seed"]["quote"][:40]} for b in zero],
        "ambiguous_multi_match": [{"pid": b["seed"]["post_id"], "quote": b["seed"]["quote"][:40],
                                   "candidates": b["res"].get("candidates")} for b in ambiguous],
        "attribution_rejected": [{"pid": b["seed"]["post_id"], "title": b["seed"]["title"],
                                  "verdict": b["seed"]["verdict"], "repost_users": b["res"].get("repost_users"),
                                  "quote": b["seed"]["quote"][:40]} for b in rejected],
        "unprincipled_rejections": [b["seed"]["post_id"] for b in unprincipled],
        "lengthened_binds": [{"pid": b["seed"]["post_id"], "orig": b["seed"]["quote"][:30],
                              "final": b["res"]["final_quote"][:40]} for b in bound if b["res"]["lengthened"]],
        "pid_corrected_binds": [{"seed_pid": b["seed"]["post_id"], "bound_pid": b["res"]["bound_pid"],
                                 "bound_kind": b["res"]["bound_kind"], "quote": b["seed"]["quote"][:30]}
                                for b in bound if b["res"]["pid_corrected"]],
        "recurrence_binds": [{"pid": b["seed"]["post_id"], "alternatives": b["res"]["alternatives"],
                              "quote": b["seed"]["quote"][:30]} for b in bound if b["res"]["recurrence"]],
        "edge_fixtures": edges,
        "synthetic_verify": synth,
        "note": ("attribution_rejected entries are commenter-original quotes the author "
                 "reposted via //@<non-author> (e.g. post 223159123 / Tianyi-Cloud, research "
                 "§3.1/§5.2 misattributed exclusion); the spike correctly refuses to bind them "
                 "as author-origin. They are excluded from authoring (AC-8)."),
    }
    REPORT.write_text(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    printable = {k: v for k, v in report.items()
                 if k not in ("edge_fixtures", "synthetic_verify", "attribution_rejected",
                              "lengthened_binds", "pid_corrected_binds", "recurrence_binds")}
    print(json.dumps(printable, ensure_ascii=False, indent=2, sort_keys=True))
    if not ok:
        for b in zero:
            print("  ZERO:", b["seed"]["post_id"], b["seed"]["quote"][:40], file=sys.stderr)
        for s in synth:
            if not s["pass"]:
                print("  SYNTH-FAIL:", s, file=sys.stderr)
        if not edges["ok"]:
            print("  EDGE-FAIL:", json.dumps(edges, ensure_ascii=False), file=sys.stderr)
        if unprincipled:
            print("  UNPRINCIPLED-REJECT:", [b["seed"]["post_id"] for b in unprincipled], file=sys.stderr)
    if not args.keep_ingest:
        # keep the reusable manifest (gitignored) but drop the temp config
        cfg.unlink(missing_ok=True)
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
