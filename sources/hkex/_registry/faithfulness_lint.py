#!/usr/bin/env python3
r"""Deck faithfulness QA linter — the authored-card gate the verbatim kernel cannot see.

`kb verify` proves a quote is verbatim in its pinned chunk; it cannot prove the CARD is
faithful to the EXPERT. This linter closes that gap mechanically over the written card
artifacts, driven entirely by a per-deck config (`faithfulness_config.json`) so the checker
logic is expert-agnostic — a second expert deck supplies its own config, not a rewrite.

Gates (each maps to a research-derived faithfulness rule; all fail closed):

  G1 verdict-link    every practitioner card's spec entry names its `candidate_title`; the
                     candidate's verification verdict must be `faithful` or `overgeneralized`,
                     and the card's `## Thesis` section must equal the candidate's
                     principle_summary (faithful) or corrected_summary (overgeneralized)
                     verbatim (whitespace-collapsed). `weak_evidence` / `misattributed`
                     candidates, and the config's excluded titles, may not be authored.
  G2 attribution     every corpus citation's quote must re-resolve to EXACTLY the chunk the
                     card pins, via the author-origin resolver (au=1 pages only, `//@`
                     repost spans rejected, parent-context parentheticals never rendered).
  G3 risk-spine      a card in the configured option readings whose text trips a trigger
                     term must carry at least one configured risk-spine phrase.
  G4 recurrence      a spec `recurrence` claim must not exceed the re-judged distinct-author-
                     utterance count for its candidate.
  G5 dating          every practitioner card carries the configured mandatory tags; a card
                     with the configured dated tag must open with the configured section.
  G6 naming          configured banned labels must not appear in any title/summary/body.
  G7 dual-cite       a spec entry marked `operational: true` must either carry a grounding
                     citation or contain the configured Xueqiu-only flag phrase in its body.

  --check [--cards-dir DIR]   lint the written practitioner cards against spec + config
  --self-test                 hermetic positive/negative fixtures for every gate
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "sources/hkex/_registry"))
from check_cjk_ingest_spike import build_index, resolve_seed  # noqa: E402  # pyright: ignore[reportMissingImports]

CONFIG = ROOT / "sources/hkex/_registry/faithfulness_config.json"
SPEC = ROOT / "sources/hkex/_registry/practitioner_cards.json"
CHUNKS = ROOT / "out/hkex/chunks_manifest.json"
REPORT = ROOT / "sources/hkex/_registry/faithfulness_lint_report.json"


def collapse(s: str) -> str:
    return " ".join(s.split())


def parse_card(path: Path) -> dict:
    text = path.read_text(encoding="utf-8")
    m = re.match(r"^---\n(.*?)\n---\n(.*)$", text, re.DOTALL)
    if not m:
        raise ValueError(f"{path}: no frontmatter")
    fm_text, body = m.group(1), m.group(2)
    # The frontmatter is emitted by our own tools as flat YAML; parse the fields this
    # linter needs without a YAML dependency (quotes are JSON-escaped double-quoted).
    def field(name: str) -> str | None:
        fm = re.search(rf'^{name}: "((?:[^"\\]|\\.)*)"$', fm_text, re.M)
        return json.loads(f'"{fm.group(1)}"') if fm else None
    tags_m = re.search(r"^tags: \[(.*)\]$", fm_text, re.M)
    tags = re.findall(r'"((?:[^"\\]|\\.)*)"', tags_m.group(1)) if tags_m else []
    cits = []
    for cm in re.finditer(
            r'- source_id: "((?:[^"\\]|\\.)*)"\n'
            r'    chunk_id: "((?:[^"\\]|\\.)*)"\n'
            r'    chunk_hash: "((?:[^"\\]|\\.)*)"\n'
            r'    page_range: \[(\d+), (\d+)\]\n'
            r'    quote: "((?:[^"\\]|\\.)*)"', fm_text):
        cits.append({"source_id": json.loads(f'"{cm.group(1)}"'),
                     "chunk_id": json.loads(f'"{cm.group(2)}"'),
                     "chunk_hash": json.loads(f'"{cm.group(3)}"'),
                     "quote": json.loads(f'"{cm.group(6)}"')})
    return {"path": path, "id": field("id"), "title": field("title") or "",
            "reading_id": field("reading_id") or "", "summary": field("summary") or "",
            "tags": tags, "citations": cits, "body": body}


def thesis_of(body: str) -> str | None:
    i = body.find("## Thesis")
    if i < 0:
        return None
    return collapse(body[i + len("## Thesis"):].split("##", 1)[0])


def first_heading(body: str) -> str:
    for ln in body.splitlines():
        if ln.startswith("##"):
            return ln.strip()
    return ""


def lint_cards(cards: list[dict], specs_by_id: dict, candidates_by_title: dict,
               cfg: dict, nidx: list[dict] | None, all_nauthors: list[str] | None) -> list[dict]:
    errors: list[dict] = []

    def err(card_id, gate, msg):
        errors.append({"card": card_id, "gate": gate, "error": msg})

    corpus_sid = cfg["practitioner_source_id"]
    for card in cards:
        cid = card["id"]
        spec = specs_by_id.get(cid)
        if spec is None:
            err(cid, "G1", "card has no spec entry in practitioner_cards.json")
            continue

        # G1 verdict-link
        ctitle = spec.get("candidate_title")
        cand = candidates_by_title.get(ctitle)
        if cand is None:
            err(cid, "G1", f"candidate_title {ctitle!r} not found in verifications")
        else:
            verdict = cand["verdict"]
            if ctitle in set(cfg.get("excluded_titles", [])):
                err(cid, "G1", f"candidate {ctitle!r} is in the excluded set")
            if verdict not in ("faithful", "overgeneralized"):
                err(cid, "G1", f"verdict {verdict!r} is not authorable")
            expected = (cand["principle_summary"] if verdict == "faithful"
                        else cand["corrected_summary"])
            th = thesis_of(card["body"])
            if th is None:
                err(cid, "G1", "no '## Thesis' section in body")
            elif collapse(expected) != th:
                err(cid, "G1", f"thesis differs from the {verdict} summary "
                               f"(expected {expected[:60]!r}…)")
            # G4 recurrence honesty
            claimed = spec.get("recurrence")
            allowed = cand.get("recurrence_distinct")
            if claimed is not None and allowed is not None and claimed > allowed:
                err(cid, "G4", f"spec recurrence {claimed} > re-judged distinct count {allowed}")

        # G2 attribution: corpus citations must re-resolve to the pinned chunk
        for c in card["citations"]:
            if c["source_id"] != corpus_sid:
                continue
            if nidx is None or all_nauthors is None:
                err(cid, "G2", "corpus chunks unavailable for attribution re-resolution")
                break
            r = resolve_seed(nidx, all_nauthors, pid="0", cid=None, quote=c["quote"])
            # pid "0" never matches, so the resolver falls to the author-word pool with a
            # pid correction — what matters here is author-origin + uniqueness + identity.
            if r["status"] != "bound":
                err(cid, "G2", f"quote no longer author-origin-resolvable "
                               f"(status={r['status']}): {c['quote'][:40]!r}")
            elif r["bound_chunk_id"] != c["chunk_id"]:
                err(cid, "G2", f"quote re-resolves to {r['bound_chunk_id']} but the card pins "
                               f"{c['chunk_id']}")

        # G3 risk-spine
        rs = cfg.get("risk_spine", {})
        text_all = " ".join([card["title"], card["summary"], card["body"]]).lower()
        if card["reading_id"] in set(rs.get("reading_ids", [])):
            if any(t.lower() in text_all for t in rs.get("trigger_terms", [])):
                if not any(p.lower() in text_all for p in rs.get("required_any", [])):
                    err(cid, "G3", "option-overlay card carries no risk-spine phrase")

        # G5 dating
        for t in cfg.get("mandatory_tags", []):
            if t not in card["tags"]:
                err(cid, "G5", f"mandatory tag {t!r} missing")
        dated = cfg.get("dated_levels", {})
        if dated.get("tag") in card["tags"] and first_heading(card["body"]) != dated.get("required_section"):
            err(cid, "G5", f"dated card must open with {dated.get('required_section')!r}")

        # G6 naming
        for ban in cfg.get("naming_bans", []):
            if ban.lower() in text_all:
                err(cid, "G6", f"banned label {ban!r} present")

        # G7 dual-cite
        if spec.get("operational"):
            has_grounding = any(c["source_id"] != corpus_sid for c in card["citations"])
            flag = cfg.get("xueqiu_only_flag", "Xueqiu-only")
            if not has_grounding and flag not in card["body"]:
                err(cid, "G7", f"operational card has no grounding citation and no "
                               f"{flag!r} flag in the body")
    return errors


def load_candidates(cfg: dict) -> dict:
    """candidate_title -> {verdict, principle_summary, corrected_summary, recurrence_distinct}."""
    ver = json.loads((ROOT / cfg["verifications_path"]).read_text(encoding="utf-8"))
    out = {}
    for v in ver:
        vv = v["verification"]
        out[v["card_title_en"]] = {
            "verdict": vv["verdict"],
            "principle_summary": v.get("principle_summary", ""),
            "corrected_summary": vv.get("corrected_summary", ""),
            "recurrence_distinct": vv.get("recurrence_distinct", v.get("recurrence")),
        }
    return out


def check(cards_dir: Path) -> int:
    cfg = json.loads(CONFIG.read_text(encoding="utf-8"))
    spec = json.loads(SPEC.read_text(encoding="utf-8")) if SPEC.is_file() else {"cards": []}
    specs_by_id = {c["id"]: c for c in spec["cards"]}
    candidates = load_candidates(cfg)
    corpus_sid = cfg["practitioner_source_id"]

    all_chunks = json.loads(CHUNKS.read_text(encoding="utf-8"))["chunks"]
    corpus_chunks = [c for c in all_chunks if c["source_id"] == corpus_sid]
    nidx = build_index(corpus_chunks) if corpus_chunks else None
    all_nauthors = [e["nauthor"] for e in nidx] if nidx else None

    cards = []
    for p in sorted(cards_dir.glob("*/*.md")):
        card = parse_card(p)
        if any(c["source_id"] == corpus_sid for c in card["citations"]):
            cards.append(card)

    spec_ids = set(specs_by_id)
    card_ids = {c["id"] for c in cards}
    errors = lint_cards(cards, specs_by_id, candidates, cfg, nidx, all_nauthors)
    for missing in sorted(spec_ids - card_ids):
        errors.append({"card": missing, "gate": "G1",
                       "error": "spec entry has no written card (emit not run?)"})

    report = {"schema_version": "hkex.faithfulness_lint.v1",
              "practitioner_cards_checked": len(cards),
              "spec_entries": len(spec_ids),
              "errors": errors,
              "verdict": "PASS" if not errors else "FAIL"}
    REPORT.write_text(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
                      encoding="utf-8")
    print(json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True))
    return 0 if not errors else 1


def _self_test() -> int:
    failures: list[str] = []
    cfg = {
        "practitioner_source_id": "corp",
        "mandatory_tags": ["xueqiu-2022h1"],
        "dated_levels": {"tag": "dated-levels", "required_section": "## Dated State"},
        "risk_spine": {"reading_ids": ["07_derivatives_and_volatility"],
                       "trigger_terms": ["sell call"],
                       "required_any": ["不弄丟底倉", "would die"]},
        "naming_bans": ["covered-call strategy"],
        "excluded_titles": ["Banned candidate"],
        "xueqiu_only_flag": "Xueqiu-only",
    }
    chunks = [{"source_id": "corp", "chunk_id": "corp:p001:0000", "chunk_hash": "a" * 64,
               "start_page": 1, "end_page": 1,
               "text": "@@HKEX p=1 k=post pid=11 cid=NA au=1@@ 我sell call的原則是不弄丟底倉。"}]
    nidx = build_index(chunks)
    nauth = [e["nauthor"] for e in nidx]
    cand = {"Good candidate": {"verdict": "overgeneralized",
                               "principle_summary": "orig",
                               "corrected_summary": "He sells with the base intact.",
                               "recurrence_distinct": 2},
            "Banned candidate": {"verdict": "faithful", "principle_summary": "x",
                                 "corrected_summary": "", "recurrence_distinct": 1}}
    good_card = {
        "path": Path("x"), "id": "c1", "title": "T", "reading_id": "07_derivatives_and_volatility",
        "summary": "s", "tags": ["xueqiu-2022h1"],
        "citations": [{"source_id": "corp", "chunk_id": "corp:p001:0000",
                       "chunk_hash": "a" * 64, "quote": "原則是不弄丟底倉"}],
        "body": "## Thesis\n\nHe sells with the base intact.\n\n## Why\n\nsell call program — 不弄丟底倉.",
    }
    good_spec = {"c1": {"id": "c1", "candidate_title": "Good candidate", "recurrence": 2,
                        "operational": False}}
    errs = lint_cards([good_card], good_spec, cand, cfg, nidx, nauth)
    if errs:
        failures.append(f"clean card raised: {errs}")

    # G1 thesis drift
    bad = dict(good_card, body=good_card["body"].replace("base intact", "base sometimes intact"))
    if not any(e["gate"] == "G1" for e in lint_cards([bad], good_spec, cand, cfg, nidx, nauth)):
        failures.append("G1 thesis drift not caught")
    # G1 excluded candidate
    sp = {"c1": dict(good_spec["c1"], candidate_title="Banned candidate")}
    bad = dict(good_card, body="## Thesis\n\nx\n\n## Why\n\nsell call — 不弄丟底倉.")
    if not any(e["gate"] == "G1" for e in lint_cards([bad], sp, cand, cfg, nidx, nauth)):
        failures.append("G1 excluded candidate not caught")
    # G2 wrong pin
    bad = dict(good_card)
    bad["citations"] = [dict(good_card["citations"][0], chunk_id="corp:p009:0000")]
    if not any(e["gate"] == "G2" for e in lint_cards([bad], good_spec, cand, cfg, nidx, nauth)):
        failures.append("G2 wrong chunk pin not caught")
    # G2 commenter quote
    bad["citations"] = [dict(good_card["citations"][0], quote="完全不存在的话")]
    if not any(e["gate"] == "G2" for e in lint_cards([bad], good_spec, cand, cfg, nidx, nauth)):
        failures.append("G2 non-author quote not caught")
    # G3 missing risk spine
    bad = dict(good_card, body="## Thesis\n\nHe sells with the base intact.\n\n## Why\n\nsell call program.")
    if not any(e["gate"] == "G3" for e in lint_cards([bad], good_spec, cand, cfg, nidx, nauth)):
        failures.append("G3 missing risk spine not caught")
    # G4 recurrence inflation
    sp = {"c1": dict(good_spec["c1"], recurrence=5)}
    if not any(e["gate"] == "G4" for e in lint_cards([good_card], sp, cand, cfg, nidx, nauth)):
        failures.append("G4 recurrence inflation not caught")
    # G5 missing tag / dated section
    bad = dict(good_card, tags=[])
    if not any(e["gate"] == "G5" for e in lint_cards([bad], good_spec, cand, cfg, nidx, nauth)):
        failures.append("G5 missing mandatory tag not caught")
    bad = dict(good_card, tags=["xueqiu-2022h1", "dated-levels"])
    if not any(e["gate"] == "G5" for e in lint_cards([bad], good_spec, cand, cfg, nidx, nauth)):
        failures.append("G5 dated card without Dated State not caught")
    # G6 naming ban
    bad = dict(good_card, title="A covered-call strategy")
    if not any(e["gate"] == "G6" for e in lint_cards([bad], good_spec, cand, cfg, nidx, nauth)):
        failures.append("G6 banned label not caught")
    # G7 dual-cite
    sp = {"c1": dict(good_spec["c1"], operational=True)}
    if not any(e["gate"] == "G7" for e in lint_cards([good_card], sp, cand, cfg, nidx, nauth)):
        failures.append("G7 operational card without grounding/flag not caught")
    okop = dict(good_card, body=good_card["body"] + "\n\nXueqiu-only practitioner observation.")
    if any(e["gate"] == "G7" for e in lint_cards([okop], sp, cand, cfg, nidx, nauth)):
        failures.append("G7 flagged card wrongly rejected")

    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print("  -", f)
        return 1
    print("SELF-TEST PASSED (faithfulness_lint: G1-G7 positive + negative fixtures)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--check", action="store_true")
    g.add_argument("--self-test", action="store_true")
    ap.add_argument("--cards-dir", type=Path, default=ROOT / "cards/hkex")
    args = ap.parse_args()
    if args.self_test:
        return _self_test()
    return check(args.cards_dir)


if __name__ == "__main__":
    raise SystemExit(main())
