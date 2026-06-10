#!/usr/bin/env python3
r"""Emit the Layer-B practitioner cards (the Xueqiu-distilled layer) from a curated spec,
binding every Chinese quote to EXACTLY ONE author-origin corpus chunk and every grounding
quote probe to exactly one grounding chunk — fail-closed, all-or-nothing.

Two citation kinds per card spec (`practitioner_cards.json`):

  * `kind: "xueqiu"` — `{post_id, comment_id?, quote_zh}`. Resolution reuses the AC-3 spike
    resolver (`check_cjk_ingest_spike.resolve_seed`): only `au=1` author chunks are
    candidates, `//@<non-author>` repost spans are attribution-rejected, multi-matches must
    lengthen to a unique author span or fail closed, and `seed_overrides.json` is consulted
    (keyed by the original `(post_id, quote_zh)`) before the heuristic. The stored citation
    quote is the resolver's final (possibly lengthened) span.
  * `kind: "grounding"` — `{source_id, quote_probe}`. Resolution reuses the AC-6 binder
    (`emit_grounding_cards.bind_quote`): tolerant ASCII probe -> exactly-one chunk -> store
    the verbatim chunk slice.

Card-shape discipline encoded at emit time (the faithfulness linter re-checks it on the
written artifacts):

  * the body's `## Thesis` section is written VERBATIM from the spec's `thesis` field, which
    the linter ties back to the candidate's verification verdict (faithful summary or exact
    corrected_summary) — the thesis is never free-typed prose;
  * a `dated-levels` card's FIRST body section must be `## Dated State`;
  * every card gets the deck's mandatory dating tags from the spec.

Fail-closed: any unbound / ambiguous / attribution-rejected citation, or any spec-shape
violation, aborts the WHOLE emit with a non-zero exit; nothing is written. The committed
report lists every binding.

  --emit         bind the spec + write the cards + the report (non-zero exit on any failure)
  --self-test    drive the binder + shape checks hermetically
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "sources/hkex/_registry"))
from deck_paths import guard_card_target  # noqa: E402  # pyright: ignore[reportMissingImports]
from emit_grounding_cards import bind_quote, _yaml_str  # noqa: E402  # pyright: ignore[reportMissingImports]
from check_cjk_ingest_spike import build_index, resolve_seed  # noqa: E402  # pyright: ignore[reportMissingImports]

SPEC = ROOT / "sources/hkex/_registry/practitioner_cards.json"
CHUNKS = ROOT / "out/hkex/chunks_manifest.json"
SEED_OVERRIDES = ROOT / "sources/hkex/_registry/seed_overrides.json"
CARDS_DIR = ROOT / "cards/hkex"
REPORT = ROOT / "sources/hkex/_registry/emit_practitioner_cards_report.json"
CORPUS_SOURCE_ID = "goubujiao_xueqiu_corpus"
LIVE_EDGES = {"supports", "defines"}


def load_overrides() -> dict:
    if not SEED_OVERRIDES.is_file():
        return {}
    ov = json.loads(SEED_OVERRIDES.read_text(encoding="utf-8")).get("overrides", [])
    return {(str(o["post_id"]), o["quote_zh"]): o for o in ov}


def bind_xueqiu(cit: dict, nidx: list[dict], all_nauthors: list[str], overrides: dict) -> dict:
    pid, q = str(cit["post_id"]), cit["quote_zh"]
    o = overrides.get((pid, q))
    cid = (o.get("set_comment_id") if o else None) or cit.get("comment_id")
    quote = (o.get("replace_quote") if o else None) or q
    r = resolve_seed(nidx, all_nauthors, pid, str(cid) if cid else None, quote)
    if r["status"] != "bound":
        raise ValueError(f"xueqiu quote did not bind (status={r['status']}): "
                         f"pid={pid} quote={q[:48]!r} detail={ {k: v for k, v in r.items() if k != 'status'} }")
    chunk_id, page = r["bound_chunk_id"], int(r["page"])
    return {
        "source_id": CORPUS_SOURCE_ID,
        "chunk_id": chunk_id,
        "chunk_hash": r["chunk_hash"],
        "page_range": [page, page],
        "quote": r["final_quote"],
        "_bound_pid": r["bound_pid"],
        "_bound_cid": r["bound_cid"],
        "_lengthened": r["lengthened"],
        "_pid_corrected": r["pid_corrected"],
    }


def validate_card_shape(card: dict) -> None:
    for key in ("id", "reading_id", "title", "summary", "tags", "thesis", "body", "citations"):
        if key not in card:
            raise ValueError(f"spec missing required key {key!r}")
    if not (80 <= len(card["summary"]) <= 400):
        raise ValueError(f"summary length {len(card['summary'])} outside [80,400]")
    if len(card["tags"]) > 10:
        raise ValueError(f"{len(card['tags'])} tags > 10")
    body = card["body"]
    if "## Thesis" not in body:
        raise ValueError("body must contain a '## Thesis' section")
    thesis_idx = body.index("## Thesis")
    thesis_text = body[thesis_idx + len("## Thesis"):].split("##", 1)[0].strip()
    if " ".join(thesis_text.split()) != " ".join(card["thesis"].split()):
        raise ValueError("the '## Thesis' section text must equal the spec `thesis` verbatim "
                         "(whitespace-collapsed)")
    if "dated-levels" in card["tags"]:
        first_heading = next((ln.strip() for ln in body.splitlines() if ln.startswith("##")), "")
        if first_heading != "## Dated State":
            raise ValueError("a dated-levels card's FIRST body section must be '## Dated State'")
    kinds = {c.get("kind") for c in card["citations"]}
    if not card["citations"] or "xueqiu" not in kinds:
        raise ValueError("a practitioner card must carry at least one xueqiu citation")
    for c in card["citations"]:
        if c.get("edge_type") not in LIVE_EDGES:
            raise ValueError(f"edge_type {c.get('edge_type')!r} not in {sorted(LIVE_EDGES)}")
        if c.get("kind") not in ("xueqiu", "grounding"):
            raise ValueError(f"unknown citation kind {c.get('kind')!r}")


def render_card(card: dict, citations: list[dict]) -> str:
    fm = ["---"]
    fm.append(f"schema_version: {_yaml_str('cacg.v0')}")
    fm.append(f"id: {_yaml_str(card['id'])}")
    fm.append(f"title: {_yaml_str(card['title'])}")
    fm.append(f"reading_id: {_yaml_str(card['reading_id'])}")
    fm.append(f"summary: {_yaml_str(card['summary'])}")
    fm.append("tags: [" + ", ".join(_yaml_str(t) for t in card["tags"]) + "]")
    fm.append("citations:")
    for cit, sp in zip(citations, card["citations"]):
        fm.append(f"  - source_id: {_yaml_str(cit['source_id'])}")
        fm.append(f"    chunk_id: {_yaml_str(cit['chunk_id'])}")
        fm.append(f"    chunk_hash: {_yaml_str(cit['chunk_hash'])}")
        fm.append(f"    page_range: [{cit['page_range'][0]}, {cit['page_range'][1]}]")
        fm.append(f"    quote: {_yaml_str(cit['quote'])}")
        fm.append(f"    edge_type: {_yaml_str(sp['edge_type'])}")
    fm.append("---")
    refs = []
    for cit, sp in zip(citations, card["citations"]):
        if sp["kind"] == "xueqiu":
            origin = (f"post {cit['_bound_pid']}" if cit["_bound_cid"] in (None, "NA")
                      else f"author reply c{cit['_bound_cid']} (post {cit['_bound_pid']})")
            refs.append(f"- `{cit['chunk_id']}` — 狗不叫, {origin}, verbatim ★AUTHOR words")
        else:
            refs.append(f"- `{cit['chunk_id']}` — grounding snapshot `{cit['source_id']}`")
    footer = ("\n\n## Sources\n\n"
              "Every Chinese quote above is the author's own verbatim wording from the ingested "
              "Xueqiu corpus (★AUTHOR-tagged utterances only; commenter text is context, never "
              "cited), bound by `kb verify` to the chunk shown. The corpus is a dated snapshot — "
              "see the Dated State / tags for its 2022-H1 weighting.\n\n" + "\n".join(refs) + "\n")
    return "\n".join(fm) + "\n\n" + card["body"].strip() + footer


def emit() -> int:
    spec = json.loads(SPEC.read_text(encoding="utf-8"))
    all_chunks = json.loads(CHUNKS.read_text(encoding="utf-8"))["chunks"]
    corpus_chunks = [c for c in all_chunks if c["source_id"] == CORPUS_SOURCE_ID]
    grounding_chunks = [c for c in all_chunks if c["source_id"] != CORPUS_SOURCE_ID]
    if not corpus_chunks:
        print(f"FAIL: no {CORPUS_SOURCE_ID} chunks in {CHUNKS}", file=sys.stderr)
        return 1
    nidx = build_index(corpus_chunks)
    all_nauthors = [e["nauthor"] for e in nidx]
    overrides = load_overrides()

    report = {"schema_version": "hkex.emit_practitioner_cards.v1", "cards": [], "errors": []}
    rendered: list[tuple[Path, str]] = []
    ids_seen: set[str] = set()
    for card in spec["cards"]:
        try:
            validate_card_shape(card)
            if card["id"] in ids_seen:
                raise ValueError(f"duplicate card id {card['id']}")
            ids_seen.add(card["id"])
            cits = []
            for sp in card["citations"]:
                if sp["kind"] == "xueqiu":
                    cits.append(bind_xueqiu(sp, nidx, all_nauthors, overrides))
                else:
                    b = bind_quote(sp["quote_probe"], grounding_chunks)
                    if b["source_id"] != sp["source_id"]:
                        raise ValueError(f"probe bound to {b['source_id']} != spec {sp['source_id']}")
                    b = dict(b, **{"_bound_pid": None, "_bound_cid": None,
                                   "_lengthened": False, "_pid_corrected": False})
                    cits.append(b)
            text = render_card(card, cits)
            out = guard_card_target(CARDS_DIR / card["reading_id"] / f"{card['id']}.md")
            rendered.append((out, text))
            report["cards"].append({
                "id": card["id"], "reading_id": card["reading_id"],
                "candidate_title": card.get("candidate_title"),
                "path": str(out.relative_to(ROOT)),
                "citations": [{"kind": sp["kind"], "chunk_id": c["chunk_id"],
                               "bound_pid": c["_bound_pid"], "bound_cid": c["_bound_cid"],
                               "lengthened": c["_lengthened"], "pid_corrected": c["_pid_corrected"],
                               "quote": c["quote"]}
                              for c, sp in zip(cits, card["citations"])],
            })
        except (ValueError, KeyError) as e:
            report["errors"].append({"card": card.get("id"), "error": str(e)})
    REPORT.write_text(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
                      encoding="utf-8")
    if report["errors"]:
        print(json.dumps({"verdict": "FAIL", "errors": report["errors"]}, ensure_ascii=False, indent=2))
        return 1
    for out, text in rendered:  # all-or-nothing: write only after every card bound cleanly
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(text, encoding="utf-8")
    print(json.dumps({"verdict": "PASS", "cards_written": len(rendered),
                      "by_reading": {r: sum(1 for c in spec["cards"] if c["reading_id"] == r)
                                     for r in sorted({c["reading_id"] for c in spec["cards"]})}},
                     ensure_ascii=False, indent=2))
    return 0


def self_test() -> int:
    failures: list[str] = []
    base = {
        "id": "gbj-x", "reading_id": "07_derivatives_and_volatility", "title": "t",
        "summary": "s" * 90, "tags": ["xueqiu-2022h1"], "thesis": "He sells around fair value.",
        "body": "## Thesis\n\nHe sells around   fair value.\n\n## Why\n\nbecause.",
        "citations": [{"kind": "xueqiu", "post_id": "1", "quote_zh": "q", "edge_type": "supports"}],
    }
    try:
        validate_card_shape(base)
    except ValueError as e:
        failures.append(f"valid shape rejected: {e}")
    bad = dict(base, body="## Why\n\nno thesis here.")
    try:
        validate_card_shape(bad)
        failures.append("missing ## Thesis not rejected")
    except ValueError:
        pass
    bad = dict(base, body="## Thesis\n\nA DIFFERENT thesis.\n")
    try:
        validate_card_shape(bad)
        failures.append("thesis drift not rejected")
    except ValueError:
        pass
    bad = dict(base, tags=["xueqiu-2022h1", "dated-levels"])
    try:
        validate_card_shape(bad)
        failures.append("dated-levels without leading ## Dated State not rejected")
    except ValueError:
        pass
    ok_dated = dict(base, tags=["xueqiu-2022h1", "dated-levels"],
                    body="## Dated State\n\n2022-H1 levels.\n\n## Thesis\n\nHe sells around fair value.")
    try:
        validate_card_shape(ok_dated)
    except ValueError as e:
        failures.append(f"valid dated-levels card rejected: {e}")
    bad = dict(base, citations=[{"kind": "grounding", "source_id": "s", "quote_probe": "p",
                                 "edge_type": "supports"}])
    try:
        validate_card_shape(bad)
        failures.append("card without a xueqiu citation not rejected")
    except ValueError:
        pass

    # Resolver wiring: a marker-formatted synthetic chunk binds via the spike resolver
    # (au=1 honored, commenter chunk rejected), exercised through bind_xueqiu.
    chunks = [
        {"source_id": CORPUS_SOURCE_ID, "chunk_id": "g:p001:0000", "chunk_hash": "a" * 64,
         "start_page": 1, "end_page": 1,
         "text": "@@HKEX p=1 k=post pid=11 cid=NA au=1@@ 我sell call的原則是不弄丟底倉。"},
        {"source_id": CORPUS_SOURCE_ID, "chunk_id": "g:p002:0000", "chunk_hash": "b" * 64,
         "start_page": 2, "end_page": 2,
         "text": "@@HKEX p=2 k=comment pid=11 cid=77 au=0@@ 商評論者引用：我sell call的原則是另一回事。"},
    ]
    nidx = build_index(chunks)
    nauth = [e["nauthor"] for e in nidx]
    b = bind_xueqiu({"post_id": "11", "quote_zh": "原則是不弄丟底倉"}, nidx, nauth, {})
    if b["chunk_id"] != "g:p001:0000" or b["page_range"] != [1, 1]:
        failures.append(f"author-origin bind wrong: {b}")
    try:
        bind_xueqiu({"post_id": "11", "quote_zh": "原則是另一回事"}, nidx, nauth, {})
        failures.append("commenter-only quote did not fail closed")
    except ValueError:
        pass
    # An override's set_comment_id pin is honored (and a wrong pin fails closed).
    try:
        bind_xueqiu({"post_id": "11", "quote_zh": "原則是不弄丟底倉"}, nidx, nauth,
                    {("11", "原則是不弄丟底倉"): {"set_comment_id": "99"}})
        failures.append("stale override cid pin did not fail closed")
    except ValueError:
        pass

    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print("  -", f)
        return 1
    print("SELF-TEST PASSED (emit_practitioner_cards: shape gates + author-origin binding + "
          "override pin fail-closed)")
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
