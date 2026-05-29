#!/usr/bin/env python3
"""Anchor-truth citation review for the six never-swarmed CFA-legacy slices.

Mechanical verification (verbatim quote containment + authorization + hash
freshness) already passes corpus-wide, but it structurally cannot catch the
defect class this review targets: a quote that is verbatim yet anchored to the
wrong volume / wrong source, or a placeholder anchor whose chunk does not
actually support the card's content. This builder produces a machine-readable
per-citation review row for every card in the six slices and classifies each by
severity, joining four sources of truth:

  * the card `.md` frontmatter (observed: source_id, chunk_id, pdf page_range,
    quote) -- what the emitted card actually cites;
  * the slice ``*_slice_curated_citations.json`` audit (audit_flag + audit_notes
    + corrected pdf pages) -- the prior single-pass page/volume/topical audit;
  * the cited chunk in ``chunks_manifest.json`` (source_id, start/end pdf page,
    text) -- the oracle for quote containment and the chunk's true location;
  * the page-coordinate maps + ``volume_page_map`` -- the volume/page oracle.

Severity vocabulary (N < W < M < E < H). The completion gate requires no M/E/H
finding to remain open. The builder is deterministic and re-runnable; the
adversarial review-swarm verdicts are merged in afterward under a separate
``reviewer`` so this file stays a pure mechanical pass.

Output: ``v0_baseline/anchor_truth_review.json`` (rows + oracle re-derivation
evidence + oracle findings + a summary with the open-finding list).

Exit codes:
  2  structural failure (coverage incomplete) -- the review could not complete.
  1  review complete but open M/E/H findings remain (gate not yet satisfied).
  0  review complete and gate clean (no open M/E/H, every oracle anchor matches).
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

import yaml

REGISTRY = Path(__file__).resolve().parent
REPO = REGISTRY.parents[2]
MAPS = REGISTRY / "page_coordinate_maps"
sys.path.insert(0, str(MAPS))
import volume_page_map as vpm  # type: ignore  # noqa: E402  (path injected above)

CHUNKS_MANIFEST = REPO / "out/cfa_legacy/chunks_manifest.json"
SOURCE_MATRIX = REPO / "out/cfa_legacy/source_matrix.json"
CARDS_ROOT = REPO / "cards/cfa_legacy"
OUT = REGISTRY / "v0_baseline/anchor_truth_review.json"
SWARM_VERDICTS = REGISTRY / "v0_baseline/anchor_truth_swarm_verdicts.json"

REVIEWER = "deterministic-anchor-review/v1"

# slice dir, curated-citations file, per-source page map (None => combined-only)
SLICES = [
    ("01_quantitative_methods", "qm_01_slice_curated_citations.json", "01_quantitative_methods.json"),
    ("02_economics", "ec_02_slice_curated_citations.json", "02_economics.json"),
    ("03_financial_reporting_analysis", "fra_03_slice_curated_citations.json", "03_financial_reporting_analysis.json"),
    ("07_derivatives_and_volatility", "deriv_07_slice_curated_citations.json", "07_derivatives_and_volatility.json"),
    ("08_convertible_bonds", "cb_08_slice_curated_citations.json", "08_convertible_bonds.json"),
    ("17_cross_cutting", "cc_17_slice_curated_citations.json", None),
]

COMBINED_SOURCE = "cfa_2022_l1_combined"

# Card-level audit_flag -> (base severity, decision) for the mechanical pass.
# Page/volume corrections are already baked into the emitted card (the card cites
# the corrected chunk and passes Layer-2 verify), so those are resolved (W). A
# content_mismatch_noted card still anchors to a chunk that does not support its
# claim -> open M for the fix pass.
FLAG_POLICY = {
    "ok": ("N", "clean: cited chunk on the audited anchor"),
    "wrong_pages_corrected": ("W", "resolved at emit: pages re-anchored (card cites corrected chunk; Layer-2 passes)"),
    "wrong_volume_corrected": ("W", "resolved at emit: volume re-anchored (card cites corrected chunk; Layer-2 passes)"),
    "content_mismatch_noted": ("M", "OPEN: placeholder anchor -- cited chunk does not support the card; re-anchor or de-claim"),
}

SEV_RANK = {"N": 0, "W": 1, "M": 2, "E": 3, "H": 4}


def norm_ws(s: str) -> str:
    """Whitespace-insensitive normalization for substring containment."""
    return re.sub(r"\s+", " ", s or "").strip()


def max_sev(a: str, b: str) -> str:
    return a if SEV_RANK[a] >= SEV_RANK[b] else b


def load_cards_on_disk(slice_dir: Path) -> list[tuple[str, dict]]:
    """Return [(card_id, frontmatter_dict)] for every emitted card in a slice."""
    out = []
    for md in sorted(slice_dir.rglob("*.md")):
        name = md.name
        if name == "INDEX.md" or name.startswith("_") or ".history" in name:
            continue
        text = md.read_text(encoding="utf-8")
        if not text.startswith("---"):
            raise SystemExit(f"{md}: missing frontmatter delimiter")
        _, fm, _ = text.split("---", 2)
        data = yaml.safe_load(fm)
        out.append((data.get("id", md.stem), data))
    return out


def parse_source_offset(per_source_map: dict) -> dict[str, int | None]:
    """Map source_id -> integer pdf offset parsed from `pdf_coordinate_rule`
    (`pdf_page = legacy_page + N` / `- N` / no addend => 0). None if unparsable."""
    offsets: dict[str, int | None] = {}
    pat = re.compile(r"legacy_page\s*([+-])\s*(\d+)")
    for entry in per_source_map.get("sources", []):
        rule = entry.get("pdf_coordinate_rule", "")
        m = pat.search(rule)
        if m:
            sign = 1 if m.group(1) == "+" else -1
            offsets[entry["source_id"]] = sign * int(m.group(2))
        elif "pdf_page = legacy_page" in rule:
            offsets[entry["source_id"]] = 0
        else:
            offsets[entry["source_id"]] = None
    return offsets


def count_disk_cards(slice_dir: Path) -> int:
    return len([
        md for md in slice_dir.rglob("*.md")
        if md.name != "INDEX.md" and not md.name.startswith("_") and ".history" not in md.name
    ])


def main() -> int:
    structural_failures: list[str] = []

    chunks_raw = json.loads(CHUNKS_MANIFEST.read_text(encoding="utf-8"))
    chunk_by_id = {
        c["chunk_id"]: {
            "source_id": c["source_id"],
            "start_page": c["start_page"],
            "end_page": c["end_page"],
            "text": c["text"],
        }
        for c in chunks_raw["chunks"]
    }

    source_matrix = json.loads(SOURCE_MATRIX.read_text(encoding="utf-8"))
    allowed = source_matrix.get("allowed", {})

    combined_map = vpm.load_map("cfa_2022_l1_combined.json")

    rows: list[dict] = []
    oracle_evidence: list[dict] = []
    oracle_findings: list[dict] = []
    cards_seen: set[str] = set()

    for slice_dir_name, curated_name, per_source_map_name in SLICES:
        slice_dir = CARDS_ROOT / slice_dir_name
        reading_id = slice_dir_name
        curated = json.loads((REGISTRY / curated_name).read_text(encoding="utf-8"))
        curated_by_id = {c["card_id"]: c for c in curated.get("cards", [])}

        per_source_map: dict = {}
        per_source_offsets: dict[str, int | None] = {}
        if per_source_map_name:
            per_source_map = json.loads((MAPS / per_source_map_name).read_text(encoding="utf-8"))
            per_source_offsets = parse_source_offset(per_source_map)

            # oracle re-derivation: >=3 anchors per per-source map used by this slice
            for entry in per_source_map.get("sources", []):
                off = per_source_offsets.get(entry["source_id"])
                if off is None:
                    continue
                for ev in entry.get("verified_evidence", [])[:3]:
                    derived = ev["legacy_page"] + off
                    ok = derived == ev["pdf_page"]
                    oracle_evidence.append({
                        "map": per_source_map_name, "source_id": entry["source_id"],
                        "anchor": {"legacy_page": ev["legacy_page"], "pdf_page": ev["pdf_page"]},
                        "rederived_pdf_page": derived, "match": ok,
                    })
                    if not ok:
                        oracle_findings.append({
                            "map": per_source_map_name, "source_id": entry["source_id"],
                            "severity": "M",
                            "scope": "registry-locator (DEC-2): cards bind by pdf chunk_id; the "
                                     "offset affects only the human legacy-page locator, not card correctness",
                            "detail": f"pdf_coordinate_rule re-derives legacy {ev['legacy_page']} -> "
                                      f"{derived}, but verified_evidence records pdf_page {ev['pdf_page']}",
                            "resolution": "correct pdf_coordinate_rule/verified_evidence to agree (add anchors "
                                          "and pick the evidence-supported offset), or record an explicit "
                                          "accept-rationale (registry-side locator only for v0 per DEC-2)",
                        })

        for card_id, fm in load_cards_on_disk(slice_dir):
            cards_seen.add(card_id)
            cur = curated_by_id.get(card_id)
            flag = (cur or {}).get("audit_flag", "ok" if cur else "MISSING_CURATED")
            base_sev, base_decision = FLAG_POLICY.get(
                flag, ("W", f"no curated audit record for this card (flag={flag})")
            )
            cur_cites = (cur or {}).get("citations", [])

            citations = fm.get("citations", []) or []
            if not citations:
                rows.append({
                    "card_id": card_id, "citation_index": None, "reading_id": reading_id,
                    "source_id": None, "chunk_id": None, "severity": max_sev(base_sev, "M"),
                    "decision": "OPEN: card has no citations", "reviewer": REVIEWER,
                    "audit_flag": flag, "checks": ["no-citations"],
                })
                continue

            for idx, cite in enumerate(citations):
                src = cite.get("source_id")
                chunk_id = cite.get("chunk_id")
                pr = cite.get("page_range") or [None, None]
                quote = cite.get("quote", "")
                chunk = chunk_by_id.get(chunk_id)

                checks: list[str] = []
                sev = base_sev
                # independent check: cited source authorized for the reading
                source_authorized = src in allowed.get(reading_id, [])
                if not source_authorized:
                    sev = max_sev(sev, "E")
                    checks.append("unauthorized-source")
                # independent check: chunk exists and its source matches the citation
                chunk_source_match = bool(chunk) and chunk["source_id"] == src
                if not chunk:
                    sev = max_sev(sev, "E")
                    checks.append("chunk-missing-from-manifest")
                elif not chunk_source_match:
                    sev = max_sev(sev, "E")
                    checks.append(f"chunk-source={chunk['source_id']}!=cite-source={src}")
                # independent check: quote is a verbatim substring of the chunk text
                quote_in_chunk = bool(chunk) and norm_ws(quote) in norm_ws(chunk["text"])
                if chunk and not quote_in_chunk:
                    sev = max_sev(sev, "E")
                    checks.append("quote-not-in-chunk")
                # independent check: the card's cited pdf page_range lies within the
                # chunk's [start_page, end_page] span (a quote may sit on any page the
                # multi-page chunk covers, so within-span -- not == start_page -- is the
                # correct invariant).
                pr0 = pr[0]
                pr1 = pr[1] if pr[1] is not None else pr[0]
                page_match = bool(chunk) and pr0 is not None and pr1 is not None and (
                    chunk["start_page"] <= pr0 <= pr1 <= chunk["end_page"]
                )
                if chunk and not page_match:
                    sev = max_sev(sev, "M")
                    checks.append(
                        f"page_range {pr} outside chunk span [{chunk['start_page']},{chunk['end_page']}]"
                    )

                # observed + expected volume/page (combined source via oracle)
                observed_volume = observed_volume_page = None
                if chunk and src == COMBINED_SOURCE:
                    try:
                        observed_volume, observed_volume_page = vpm.pdf_page_to_vol_page(
                            combined_map, chunk["start_page"]
                        )
                    except ValueError as e:
                        sev = max_sev(sev, "M")
                        checks.append(f"combined-oracle:{e}")
                # expected from curated record (the audited target), matched by index
                if cur_cites and idx < len(cur_cites):
                    cc = cur_cites[idx]
                    expected_source = cc.get("source_id")
                    expected_volume = cc.get("volume")
                    expected_pdf_range = cc.get("pdf_pages")
                else:
                    expected_source = src
                    expected_volume = observed_volume
                    expected_pdf_range = pr

                rows.append({
                    "card_id": card_id,
                    "citation_index": idx,
                    "reading_id": reading_id,
                    "role": cite.get("edge_type"),
                    "source_id": src,
                    "chunk_id": chunk_id,
                    "observed_pdf_page_range": pr,
                    "observed_volume": observed_volume,
                    "observed_volume_page": observed_volume_page,
                    "expected_source": expected_source,
                    "expected_volume": expected_volume,
                    "expected_pdf_page_range": expected_pdf_range,
                    "quote_in_chunk": quote_in_chunk,
                    "source_authorized": source_authorized,
                    "chunk_source_match": chunk_source_match,
                    "page_match": page_match,
                    "audit_flag": flag,
                    "severity": sev,
                    "decision": base_decision if sev == base_sev else f"{base_decision}; checks={checks}",
                    "checks": checks,
                    "reviewer": REVIEWER,
                })

    # ---- combined-map oracle re-derivation: >=3 anchors per cited volume ----
    cited_volumes = sorted({r["observed_volume"] for r in rows if r.get("observed_volume")})
    for vol_row in combined_map["volume_table"]:
        if vol_row["volume"] not in cited_volumes:
            continue
        for ev in vol_row.get("verified_evidence", [])[:3]:
            derived = vpm.vol_page_to_pdf_page(combined_map, vol_row["volume"], ev["volume_page"])
            ok = derived == ev["pdf_page"]
            oracle_evidence.append({
                "map": "cfa_2022_l1_combined.json", "volume": vol_row["volume"],
                "anchor": {"volume_page": ev["volume_page"], "pdf_page": ev["pdf_page"],
                           "header_snippet": ev.get("header_snippet")},
                "rederived_pdf_page": derived, "match": ok,
            })
            if not ok:
                oracle_findings.append({
                    "map": "cfa_2022_l1_combined.json", "volume": vol_row["volume"],
                    "severity": "M",
                    "scope": "combined-volume oracle (the primary CFA locator)",
                    "detail": f"V{vol_row['volume']} vp {ev['volume_page']} -> {derived} "
                              f"!= verified_evidence pdf_page {ev['pdf_page']}",
                    "resolution": "correct the volume_table offset/evidence before relying on the oracle",
                })

    # ---- merge adversarial review-swarm verdicts (if persisted) ----
    # The swarm (per-slice Claude review subagents) confirms/disputes each prior
    # content_mismatch flag and surfaces anchoring defects the single-pass curated
    # audit + mechanical verify both missed. Verdicts attach to the matching rows;
    # a disputed content_mismatch is downgraded; a new finding bumps its row's
    # severity (or adds a card-level synthetic row) so it joins the open set.
    swarm_summary: dict = {"present": False}
    swarm_new_findings: list[dict] = []
    if SWARM_VERDICTS.is_file():
        swarm = json.loads(SWARM_VERDICTS.read_text(encoding="utf-8"))
        row_index: dict[tuple, list[dict]] = {}
        card_index: dict[str, list[dict]] = {}
        for r in rows:
            row_index.setdefault((r["card_id"], r["citation_index"]), []).append(r)
            card_index.setdefault(r["card_id"], []).append(r)
        n_verdict = n_confirm = n_dispute = 0
        for sl in swarm.get("slices", []):
            slice_name = sl.get("slice")
            for v in sl.get("content_mismatch_verdicts", []):
                n_verdict += 1
                n_confirm += v.get("verdict") == "confirm"
                n_dispute += v.get("verdict") == "dispute"
                verdict_obj = {
                    "verdict": v.get("verdict"),
                    "proposed_resolution": v.get("proposed_resolution"),
                    "target_source_id": v.get("target_source_id"),
                    "target_hint": v.get("target_hint"),
                    "reviewer": f"swarm:{slice_name}",
                }
                for r in card_index.get(v["card_id"], []):
                    r["swarm_verdict"] = verdict_obj
                    # A disputed prior content_mismatch (with no independent
                    # mechanical defect) is downgraded out of the open set.
                    if (v.get("verdict") == "dispute"
                            and r.get("audit_flag") == "content_mismatch_noted"
                            and r["severity"] == "M" and not r.get("checks")):
                        r["severity"] = "W"
                        r["decision"] = "swarm dispute: prior content_mismatch flag overturned"
            for f in sl.get("new_findings", []):
                sev = f.get("severity", "W")
                rec = {
                    "card_id": f["card_id"], "citation_index": f.get("citation_index"),
                    "severity": sev, "issue": f.get("issue"),
                    "rationale": f.get("rationale"), "reviewer": f"swarm:{slice_name}",
                }
                swarm_new_findings.append(rec)
                ci = f.get("citation_index")
                targets = row_index.get((f["card_id"], ci)) if ci is not None else None
                if targets:
                    for r in targets:
                        r["severity"] = max_sev(r["severity"], sev)
                        r.setdefault("checks", []).append(f"swarm-finding:{sev}")
                        r["swarm_finding"] = rec
                else:
                    rows.append({
                        "card_id": f["card_id"], "citation_index": ci, "severity": sev,
                        "decision": f"swarm new finding ({sev}): {f.get('issue')}",
                        "reviewer": f"swarm:{slice_name}", "audit_flag": "swarm_new_finding",
                        "checks": [f"swarm-finding:{sev}"], "swarm_finding": rec,
                    })
        swarm_summary = {
            "present": True, "content_mismatch_verdicts": n_verdict,
            "confirmed": n_confirm, "disputed": n_dispute,
            "new_findings": len(swarm_new_findings),
        }

    # ---- coverage check ----
    expected_cards = sum(count_disk_cards(CARDS_ROOT / s[0]) for s in SLICES)
    carded = len(cards_seen)
    if carded != expected_cards:
        structural_failures.append(f"coverage: reviewed {carded} cards but {expected_cards} on disk")

    by_sev: dict[str, int] = {}
    for r in rows:
        by_sev[r["severity"]] = by_sev.get(r["severity"], 0) + 1
    open_rows = [
        {"card_id": r["card_id"], "citation_index": r["citation_index"],
         "severity": r["severity"], "audit_flag": r.get("audit_flag"),
         "decision": r["decision"], "checks": r.get("checks", [])}
        for r in rows if r["severity"] in ("M", "E", "H")
    ]
    open_findings = sorted({r["card_id"] for r in open_rows})
    oracle_all_match = all(e["match"] for e in oracle_evidence)
    gate_clean = (not open_rows) and oracle_all_match and (not oracle_findings)

    artifact = {
        "schema_version": "cfa_legacy_anchor_truth_review/v1",
        "slices": [s[0] for s in SLICES],
        "summary": {
            "cards_reviewed": carded,
            "cards_expected": expected_cards,
            "citations_reviewed": len([r for r in rows if r["citation_index"] is not None]),
            "by_severity": by_sev,
            "open_meh_card_count": len(open_findings),
            "open_meh_row_count": len(open_rows),
            "oracle_anchors_checked": len(oracle_evidence),
            "oracle_all_match": oracle_all_match,
            "oracle_finding_count": len(oracle_findings),
            "swarm": swarm_summary,
            "gate_clean": gate_clean,
        },
        "open_findings_cards": open_findings,
        "open_findings_rows": open_rows,
        "oracle_findings": oracle_findings,
        "swarm_new_findings": swarm_new_findings,
        "oracle_evidence": oracle_evidence,
        "rows": rows,
    }
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(artifact, indent=2, sort_keys=True, ensure_ascii=False) + "\n",
                   encoding="utf-8")

    print(f"anchor-truth review written to {OUT}")
    print(f"cards reviewed={carded}/{expected_cards}; "
          f"citations={artifact['summary']['citations_reviewed']}; "
          f"severity={by_sev}; open M/E/H cards={len(open_findings)} (rows={len(open_rows)}); "
          f"oracle anchors={len(oracle_evidence)} all_match={oracle_all_match}; "
          f"oracle findings={len(oracle_findings)}")
    if open_findings:
        print("open M/E/H cards:", ", ".join(open_findings))
    if oracle_findings:
        print("oracle findings:", "; ".join(f"{o.get('map')}/{o.get('source_id', 'V'+str(o.get('volume')))}" for o in oracle_findings))

    if structural_failures:
        print("\nANCHOR-TRUTH REVIEW: STRUCTURAL FAILURE", file=sys.stderr)
        for f in structural_failures:
            print(f"  - {f}", file=sys.stderr)
        return 2
    if not gate_clean:
        print("\nANCHOR-TRUTH REVIEW: COMPLETE -- open findings remain (AC-5 gate not yet satisfied)")
        return 1
    print("\nANCHOR-TRUTH REVIEW: GATE CLEAN (coverage complete, no open M/E/H, all oracle anchors match)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
