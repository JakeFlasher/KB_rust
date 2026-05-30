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

It then merges the adversarial review-swarm verdicts and applies any recorded
resolutions (de-claim / accept-oracle / re-anchor). Severity vocabulary
(N < W < M < E < H); the AC-5 completion gate requires no UNRESOLVED M/E/H
finding and no UNACCEPTED oracle finding.

Every emitted review row and every open-finding row is validated against a fixed
required-field schema before the artifact is written.

Output: ``release_baseline/anchor_truth_review.json``.

Exit codes:
  2  structural failure (coverage incomplete OR a row missing a required field).
  1  review complete but open/unresolved findings remain (gate not yet satisfied).
  0  gate clean (every M/E/H resolved, every oracle finding accepted, schema ok).
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
OUT = REGISTRY / "release_baseline/anchor_truth_review.json"
SWARM_VERDICTS = REGISTRY / "release_baseline/anchor_truth_swarm_verdicts.json"
RESOLUTIONS = REGISTRY / "release_baseline/anchor_truth_resolutions.json"

REVIEWER = "deterministic-anchor-review/v1"
MIN_ORACLE_ANCHORS = 3  # AC-5: >=3 re-derived anchors per used map/volume

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

FLAG_POLICY = {
    "ok": ("N", "clean: cited chunk on the audited anchor"),
    "wrong_pages_corrected": ("W", "resolved at emit: pages re-anchored (card cites corrected chunk; Layer-2 passes)"),
    "wrong_volume_corrected": ("W", "resolved at emit: volume re-anchored (card cites corrected chunk; Layer-2 passes)"),
    "content_mismatch_noted": ("M", "OPEN: placeholder anchor -- cited chunk does not support the card; re-anchor or de-claim"),
    "re_anchored": ("N", "re-anchored to a supporting source (R4 discovery + adversarial support-verification; independent quote-substring + kb verify)"),
}

SEV_RANK = {"N": 0, "W": 1, "M": 2, "E": 3, "H": 4}

# Every review row -- citation-scope or card-scope -- carries these fields.
REQUIRED_ROW_FIELDS = [
    "card_id", "citation_index", "reading_id", "finding_scope",
    "source_id", "chunk_id", "observed_pdf_page_range",
    "observed_volume", "observed_volume_page",
    "expected_source", "expected_volume", "expected_pdf_page_range",
    "severity", "decision", "reviewer", "audit_flag", "checks",
]


def base_row(**kw) -> dict:
    """A schema-complete row: every required field present (None default)."""
    row: dict = {k: None for k in REQUIRED_ROW_FIELDS}
    row["checks"] = []
    row.update(kw)
    return row


def norm_ws(s: str) -> str:
    return re.sub(r"\s+", " ", s or "").strip()


def max_sev(a: str, b: str) -> str:
    return a if SEV_RANK[a] >= SEV_RANK[b] else b


def load_cards_on_disk(slice_dir: Path) -> list[tuple[str, dict]]:
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
    offsets: dict[str, int | None] = {}
    pat = re.compile(r"legacy_page\s*([+-])\s*(\d+)")
    for entry in per_source_map.get("sources", []):
        rule = entry.get("pdf_coordinate_rule", "")
        m = pat.search(rule)
        if m:
            offsets[entry["source_id"]] = (1 if m.group(1) == "+" else -1) * int(m.group(2))
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


def derive_oracle(rows, chunk_by_id, combined_map, per_source_maps):
    """Re-derive each used map/volume's anchors AND confirm >=3 against a cited
    chunk's page_range. Returns (evidence, findings, coverage).

    AC-5 standard: for every map used as a volume/page oracle, at least
    MIN_ORACLE_ANCHORS deterministically-selected anchors are re-derived to a pdf
    page and confirmed against a cited chunk's page_range; a map whose rule
    disagrees with its own evidence, or that offers fewer than MIN_ORACLE_ANCHORS
    confirmable anchors, is recorded as an open oracle finding.
    """
    evidence: list[dict] = []
    findings: list[dict] = []
    coverage: list[dict] = []

    # cited chunks grouped by source_id (deterministic order by chunk_id)
    cited_by_source: dict[str, list[dict]] = {}
    for r in rows:
        cid = r.get("chunk_id")
        ch = chunk_by_id.get(cid) if cid else None
        if ch:
            cited_by_source.setdefault(ch["source_id"], []).append(
                {"chunk_id": cid, "start_page": ch["start_page"], "end_page": ch["end_page"],
                 "card_id": r["card_id"], "page_range": r.get("observed_pdf_page_range")}
            )
    for lst in cited_by_source.values():
        lst.sort(key=lambda c: c["chunk_id"])

    # ---- combined volume/page oracle (THE AC-5 (volume,page) oracle) ----
    # Standard: >= MIN_ORACLE_ANCHORS re-derived (volume,page) verified-evidence
    # anchors that match, AND >= MIN_ORACLE_ANCHORS cited chunks whose oracle
    # round-trip lands WITHIN the cited chunk's recorded page_range and span
    # (containment, not start_page equality). Enforced map-wide (no per-volume waiver).
    cited_volumes: dict[int, list[dict]] = {}
    for c in cited_by_source.get(COMBINED_SOURCE, []):
        try:
            vol, _ = vpm.pdf_page_to_vol_page(combined_map, c["start_page"])
            cited_volumes.setdefault(vol, []).append(c)
        except ValueError:
            pass
    total_ev_ok = 0
    total_conf = 0
    per_volume: list[dict] = []
    for vol_row in combined_map["volume_table"]:
        vol = vol_row["volume"]
        if vol not in cited_volumes:
            continue
        ev_ok = 0
        for ev in vol_row.get("verified_evidence", []):
            derived = vpm.vol_page_to_pdf_page(combined_map, vol, ev["volume_page"])
            ok = derived == ev["pdf_page"]
            ev_ok += ok
            evidence.append({"map": "cfa_2022_l1_combined.json", "volume": vol, "kind": "map-evidence",
                             "anchor": {"volume_page": ev["volume_page"], "pdf_page": ev["pdf_page"],
                                        "header_snippet": ev.get("header_snippet")},
                             "rederived_pdf_page": derived, "match": ok})
            if not ok:
                findings.append({"map": "cfa_2022_l1_combined.json", "volume": vol, "severity": "M",
                                 "kind": "evidence-mismatch",
                                 "detail": f"V{vol} vp {ev['volume_page']} -> {derived} != {ev['pdf_page']}",
                                 "resolution": "correct the volume_table offset/evidence before relying on the oracle"})
        conf = 0
        for c in cited_volumes[vol][:MIN_ORACLE_ANCHORS]:
            v2, p2 = vpm.pdf_page_to_vol_page(combined_map, c["start_page"])
            back = vpm.vol_page_to_pdf_page(combined_map, v2, p2)
            pr = c["page_range"] or [None, None]
            lo, hi = pr[0], (pr[1] if pr[1] is not None else pr[0])
            contained = (lo is not None and lo <= back <= hi
                         and c["start_page"] <= back <= c["end_page"])
            conf += contained
            evidence.append({"map": "cfa_2022_l1_combined.json", "volume": vol, "kind": "cited-chunk",
                             "chunk_id": c["chunk_id"], "derived_volume_page": [v2, p2],
                             "rederived_pdf_page": back, "cited_page_range": c["page_range"],
                             "chunk_span": [c["start_page"], c["end_page"]], "confirmed": bool(contained)})
        total_ev_ok += ev_ok
        total_conf += conf
        per_volume.append({"volume": vol, "evidence_anchors_ok": ev_ok, "cited_chunk_confirms": conf})
    combined_below = total_ev_ok < MIN_ORACLE_ANCHORS or total_conf < MIN_ORACLE_ANCHORS
    coverage.append({"map": "cfa_2022_l1_combined.json", "ac5_oracle": True,
                     "total_evidence_anchors_ok": total_ev_ok, "total_cited_chunk_confirms": total_conf,
                     "per_volume": per_volume, "below_standard": combined_below})
    if combined_below:
        findings.append({"map": "cfa_2022_l1_combined.json", "severity": "M", "kind": "below-standard-coverage",
                         "detail": f"combined oracle: {total_ev_ok} matching evidence anchors + "
                                   f"{total_conf} cited-chunk confirmations (< {MIN_ORACLE_ANCHORS})",
                         "resolution": "add verified_evidence / cited-chunk confirmations"})

    # ---- per-source single-offset locators (NOT the AC-5 (volume,page) oracle) ----
    # DEC-2: cards bind by pdf chunk_id, so these legacy-page offsets are
    # registry-side human locators outside the AC-5 oracle standard. Coverage is
    # reported transparently (ac5_oracle=false); only an evidence-vs-rule MISMATCH
    # is raised as a finding (a wrong locator), which is then accept-rationalized.
    for map_name, per_source_map in per_source_maps.items():
        offsets = parse_source_offset(per_source_map)
        ev_by_source = {e["source_id"]: e for e in per_source_map.get("sources", [])}
        for source_id in sorted(set(ev_by_source) & set(cited_by_source)):
            entry = ev_by_source[source_id]
            off = offsets.get(source_id)
            ev_ok = 0
            ev_total = 0
            for ev in entry.get("verified_evidence", []):
                ev_total += 1
                if off is None:
                    continue
                derived = ev["legacy_page"] + off
                ok = derived == ev["pdf_page"]
                ev_ok += ok
                evidence.append({"map": map_name, "source_id": source_id, "kind": "map-evidence",
                                 "anchor": {"legacy_page": ev["legacy_page"], "pdf_page": ev["pdf_page"]},
                                 "rederived_pdf_page": derived, "match": ok})
                if not ok:
                    findings.append({"map": map_name, "source_id": source_id, "severity": "M",
                                     "kind": "evidence-mismatch",
                                     "scope": "registry-locator (DEC-2): cards bind by pdf chunk_id; offset is human-locator only",
                                     "detail": f"rule re-derives legacy {ev['legacy_page']} -> {derived} != verified_evidence pdf {ev['pdf_page']}",
                                     "resolution": "correct rule/evidence to agree, or accept-rationale (registry-side locator only for v0, DEC-2)"})
            coverage.append({"map": map_name, "source_id": source_id, "ac5_oracle": False,
                             "evidence_anchors_ok": ev_ok, "evidence_anchors_total": ev_total,
                             "cited_chunks": len(cited_by_source.get(source_id, [])),
                             "note": "single-offset registry-side legacy-page locator (DEC-2): cards bind by "
                                     "pdf chunk_id; outside the AC-5 (volume,page) oracle standard; only an "
                                     "evidence-vs-rule MISMATCH is a finding"})
    return evidence, findings, coverage


def main() -> int:
    structural_failures: list[str] = []

    chunks_raw = json.loads(CHUNKS_MANIFEST.read_text(encoding="utf-8"))
    chunk_by_id = {
        c["chunk_id"]: {"source_id": c["source_id"], "start_page": c["start_page"],
                        "end_page": c["end_page"], "text": c["text"]}
        for c in chunks_raw["chunks"]
    }
    allowed = json.loads(SOURCE_MATRIX.read_text(encoding="utf-8")).get("allowed", {})
    combined_map = vpm.load_map("cfa_2022_l1_combined.json")

    rows: list[dict] = []
    cards_seen: set[str] = set()
    per_source_maps: dict[str, dict] = {}

    for slice_dir_name, curated_name, per_source_map_name in SLICES:
        slice_dir = CARDS_ROOT / slice_dir_name
        reading_id = slice_dir_name
        curated = json.loads((REGISTRY / curated_name).read_text(encoding="utf-8"))
        curated_by_id = {c["card_id"]: c for c in curated.get("cards", [])}
        if per_source_map_name and per_source_map_name not in per_source_maps:
            per_source_maps[per_source_map_name] = json.loads((MAPS / per_source_map_name).read_text(encoding="utf-8"))

        for card_id, fm in load_cards_on_disk(slice_dir):
            cards_seen.add(card_id)
            cur = curated_by_id.get(card_id)
            flag = (cur or {}).get("audit_flag", "ok" if cur else "MISSING_CURATED")
            base_sev, base_decision = FLAG_POLICY.get(
                flag, ("W", f"no curated audit record for this card (flag={flag})"))
            cur_cites = (cur or {}).get("citations", [])
            citations = fm.get("citations", []) or []

            if not citations:
                rows.append(base_row(
                    card_id=card_id, reading_id=reading_id, finding_scope="card",
                    severity=max_sev(base_sev, "M"), audit_flag=flag,
                    decision="OPEN: card has no citations", reviewer=REVIEWER, checks=["no-citations"]))
                continue

            for idx, cite in enumerate(citations):
                src = cite.get("source_id")
                chunk_id = cite.get("chunk_id")
                pr = cite.get("page_range") or [None, None]
                quote = cite.get("quote", "")
                chunk = chunk_by_id.get(chunk_id)
                checks: list[str] = []
                sev = base_sev

                source_authorized = src in allowed.get(reading_id, [])
                if not source_authorized:
                    sev = max_sev(sev, "E"); checks.append("unauthorized-source")
                chunk_source_match = bool(chunk) and chunk["source_id"] == src
                if not chunk:
                    sev = max_sev(sev, "E"); checks.append("chunk-missing-from-manifest")
                elif not chunk_source_match:
                    sev = max_sev(sev, "E"); checks.append(f"chunk-source={chunk['source_id']}!=cite-source={src}")
                quote_in_chunk = bool(chunk) and norm_ws(quote) in norm_ws(chunk["text"])
                if chunk and not quote_in_chunk:
                    sev = max_sev(sev, "E"); checks.append("quote-not-in-chunk")
                pr0 = pr[0]
                pr1 = pr[1] if pr[1] is not None else pr[0]
                page_match = bool(chunk) and pr0 is not None and pr1 is not None and (
                    chunk["start_page"] <= pr0 <= pr1 <= chunk["end_page"])
                if chunk and not page_match:
                    sev = max_sev(sev, "M")
                    checks.append(f"page_range {pr} outside chunk span [{chunk['start_page']},{chunk['end_page']}]")

                observed_volume = observed_volume_page = None
                if chunk and src == COMBINED_SOURCE:
                    try:
                        observed_volume, observed_volume_page = vpm.pdf_page_to_vol_page(combined_map, chunk["start_page"])
                    except ValueError as e:
                        sev = max_sev(sev, "M"); checks.append(f"combined-oracle:{e}")
                if cur_cites and idx < len(cur_cites):
                    cc = cur_cites[idx]
                    expected_source, expected_volume, expected_pdf = cc.get("source_id"), cc.get("volume"), cc.get("pdf_pages")
                else:
                    expected_source, expected_volume, expected_pdf = src, observed_volume, pr

                rows.append(base_row(
                    card_id=card_id, citation_index=idx, reading_id=reading_id, finding_scope="citation",
                    role=cite.get("edge_type"), source_id=src, chunk_id=chunk_id,
                    observed_pdf_page_range=pr, observed_volume=observed_volume, observed_volume_page=observed_volume_page,
                    expected_source=expected_source, expected_volume=expected_volume, expected_pdf_page_range=expected_pdf,
                    quote_in_chunk=quote_in_chunk, source_authorized=source_authorized,
                    chunk_source_match=chunk_source_match, page_match=page_match,
                    audit_flag=flag, severity=sev,
                    decision=base_decision if sev == base_sev else f"{base_decision}; checks={checks}",
                    checks=checks, reviewer=REVIEWER))

    # ---- oracle re-derivation (coverage + cited-chunk confirmation) ----
    oracle_evidence, oracle_findings, oracle_coverage = derive_oracle(
        rows, chunk_by_id, combined_map, per_source_maps)

    # ---- merge adversarial review-swarm verdicts ----
    swarm_summary: dict = {"present": False}
    swarm_new_findings: list[dict] = []
    if SWARM_VERDICTS.is_file():
        swarm = json.loads(SWARM_VERDICTS.read_text(encoding="utf-8"))
        row_index: dict[tuple, list[dict]] = {}
        card_index: dict[str, list[dict]] = {}
        for r in rows:
            row_index.setdefault((r["card_id"], r["citation_index"]), []).append(r)
            card_index.setdefault(r["card_id"], []).append(r)
        n_v = n_conf = n_disp = 0
        for sl in swarm.get("slices", []):
            sname = sl.get("slice")
            for v in sl.get("content_mismatch_verdicts", []):
                n_v += 1; n_conf += v.get("verdict") == "confirm"; n_disp += v.get("verdict") == "dispute"
                vobj = {"verdict": v.get("verdict"), "proposed_resolution": v.get("proposed_resolution"),
                        "target_source_id": v.get("target_source_id"), "target_hint": v.get("target_hint"),
                        "rationale": v.get("rationale"), "reviewer": f"swarm:{sname}"}
                for r in card_index.get(v["card_id"], []):
                    r["swarm_verdict"] = vobj
                    if (v.get("verdict") == "dispute" and r.get("audit_flag") == "content_mismatch_noted"
                            and r["severity"] == "M" and not r.get("checks")):
                        r["severity"] = "W"; r["decision"] = "swarm dispute: prior content_mismatch flag overturned"
            for f in sl.get("new_findings", []):
                sev = f.get("severity", "W")
                rec = {"card_id": f["card_id"], "citation_index": f.get("citation_index"), "severity": sev,
                       "issue": f.get("issue"), "rationale": f.get("rationale"), "reviewer": f"swarm:{sname}"}
                swarm_new_findings.append(rec)
                ci = f.get("citation_index")
                targets = row_index.get((f["card_id"], ci)) if ci is not None else None
                if targets:
                    for r in targets:
                        r["severity"] = max_sev(r["severity"], sev)
                        r.setdefault("checks", []).append(f"swarm-finding:{sev}")
                        r["swarm_finding"] = rec
                        # repair (3): rewrite decision so an open swarm M no longer
                        # carries the stale "resolved at emit" mechanical text.
                        if SEV_RANK[sev] >= SEV_RANK["M"]:
                            r["decision"] = (f"OPEN(swarm {sev}): {f.get('issue')}"
                                             f" | repair: {f.get('rationale', '')[:200]}")
                else:
                    rows.append(base_row(
                        card_id=f["card_id"], citation_index=ci, reading_id=None, finding_scope="card",
                        severity=sev, audit_flag="swarm_new_finding",
                        decision=f"OPEN(swarm {sev}, card-scope): {f.get('issue')}",
                        reviewer=f"swarm:{sname}", checks=[f"swarm-finding:{sev}"]))
                    rows[-1]["swarm_finding"] = rec
        swarm_summary = {"present": True, "content_mismatch_verdicts": n_v, "confirmed": n_conf,
                         "disputed": n_disp, "new_findings": len(swarm_new_findings)}

    # ---- apply recorded resolutions (de-claim / accept-oracle / re-anchor) ----
    resolutions_applied: list[dict] = []
    if RESOLUTIONS.is_file():
        res = json.loads(RESOLUTIONS.read_text(encoding="utf-8"))
        card_res = {r["target"]: r for r in res.get("resolutions", []) if r.get("target_type") == "card"}
        oracle_res = {r["target"]: r for r in res.get("resolutions", []) if r.get("target_type") == "oracle"}
        ci_map: dict[str, list[dict]] = {}
        for r in rows:
            ci_map.setdefault(r["card_id"], []).append(r)
        # A resolution closes only SEMANTIC findings (content_mismatch / swarm).
        # It must never mask a real MECHANICAL defect (quote not in chunk,
        # unauthorized source, missing/mismatched chunk, page outside span); those
        # are caught here so a bad re-anchor cannot be hidden by a resolution.
        mech_markers = ("unauthorized-source", "chunk-missing", "quote-not-in-chunk",
                        "chunk-source", "outside chunk span", "combined-oracle")

        def has_mechanical_failure(checks: list | None) -> bool:
            return any(any(m in c for m in mech_markers) for c in (checks or []))

        for card_id, rr in card_res.items():
            cite_idx = rr.get("citation_index")
            hit = blocked = False
            for r in ci_map.get(card_id, []):
                if cite_idx is not None and r.get("citation_index") != cite_idx:
                    continue
                if r["severity"] in ("M", "E", "H"):
                    if has_mechanical_failure(r.get("checks")):
                        blocked = True
                        continue
                    r["severity"] = "W"
                    r["decision"] = f"RESOLVED({rr['kind']}): {rr.get('rationale', '')[:220]}"
                    r["resolution"] = {"kind": rr["kind"], "decided_by": rr.get("decided_by")}
                    hit = True
            resolutions_applied.append({"target": card_id, "kind": rr["kind"], "applied": hit, "blocked_by_mechanical": blocked})
            if blocked:
                structural_failures.append(
                    f"resolution for {card_id} cannot apply: an open row still has a MECHANICAL failure "
                    f"(re-anchor did not actually fix the citation)")
            elif not hit:
                structural_failures.append(f"resolution for {card_id} matched no open finding (stale resolution?)")
        for of in oracle_findings:
            key = f"{of.get('map')}/{of.get('source_id', 'V' + str(of.get('volume')))}"
            if key in oracle_res:
                of["accepted"] = True
                of["accept_rationale"] = oracle_res[key].get("rationale")
                of["decided_by"] = oracle_res[key].get("decided_by")
                resolutions_applied.append({"target": key, "kind": "accept-oracle", "applied": True})

    # ---- coverage check ----
    expected_cards = sum(count_disk_cards(CARDS_ROOT / s[0]) for s in SLICES)
    carded = len(cards_seen)
    if carded != expected_cards:
        structural_failures.append(f"coverage: reviewed {carded} cards but {expected_cards} on disk")

    by_sev: dict[str, int] = {}
    for r in rows:
        by_sev[r["severity"]] = by_sev.get(r["severity"], 0) + 1

    open_rows_full = [r for r in rows if r["severity"] in ("M", "E", "H")]
    # open_findings_rows are schema-complete copies (every REQUIRED_ROW_FIELDS key)
    # plus the merge metadata, so the open worklist is fully self-contained.
    open_rows = []
    for r in open_rows_full:
        o = {k: r.get(k) for k in REQUIRED_ROW_FIELDS}
        o["swarm_finding"] = r.get("swarm_finding")
        o["swarm_verdict"] = r.get("swarm_verdict")
        o["resolution"] = r.get("resolution")
        open_rows.append(o)
    open_findings = sorted({r["card_id"] for r in open_rows_full})

    # ---- schema validation: BOTH rows and open_findings_rows carry every field ----
    for r in rows:
        missing = [k for k in REQUIRED_ROW_FIELDS if k not in r]
        if missing:
            structural_failures.append(f"row {r.get('card_id')}[{r.get('citation_index')}] missing fields {missing}")
    for o in open_rows:
        missing = [k for k in REQUIRED_ROW_FIELDS if k not in o]
        if missing:
            structural_failures.append(f"open_findings_row {o.get('card_id')}[{o.get('citation_index')}] missing fields {missing}")

    open_oracle = [o for o in oracle_findings if not o.get("accepted")]
    # gate_clean must be false while any AC-5 (volume,page) oracle coverage entry is
    # below standard without a recorded acceptance.
    ac5_below_unaccepted = [c for c in oracle_coverage
                            if c.get("ac5_oracle") and c.get("below_standard") and not c.get("accepted")]
    gate_clean = (not open_rows_full) and (not open_oracle) and (not ac5_below_unaccepted) and (not structural_failures)

    artifact = {
        "schema_version": "cfa_legacy_anchor_truth_review/v2",
        "slices": [s[0] for s in SLICES],
        "required_row_fields": REQUIRED_ROW_FIELDS,
        "oracle_policy": (
            "The volume/page oracle is cfa_2022_l1_combined.json: >= "
            f"{MIN_ORACLE_ANCHORS} (volume,page) verified-evidence anchors are re-derived per CITED "
            "volume AND >=3 cited chunks are confirmed by round-trip against their page_range. "
            "Per-source maps are single-offset registry-side legacy-page locators (DEC-2): cards "
            "bind by pdf chunk_id, so their coverage is reported transparently in oracle_coverage "
            "but only an evidence-vs-rule MISMATCH is raised as an oracle finding."
        ),
        "summary": {
            "cards_reviewed": carded, "cards_expected": expected_cards,
            "citations_reviewed": len([r for r in rows if r["finding_scope"] == "citation"]),
            "by_severity": by_sev,
            "open_meh_card_count": len(open_findings), "open_meh_row_count": len(open_rows_full),
            "oracle_anchors_checked": len(oracle_evidence),
            "oracle_findings_total": len(oracle_findings), "oracle_findings_open": len(open_oracle),
            "ac5_oracle_below_unaccepted": len(ac5_below_unaccepted),
            "min_oracle_anchors": MIN_ORACLE_ANCHORS,
            "swarm": swarm_summary,
            "resolutions_applied": len(resolutions_applied),
            "gate_clean": gate_clean,
        },
        "open_findings_cards": open_findings,
        "open_findings_rows": open_rows,
        "oracle_findings": oracle_findings,
        "oracle_coverage": oracle_coverage,
        "swarm_new_findings": swarm_new_findings,
        "resolutions_applied": resolutions_applied,
        "oracle_evidence": oracle_evidence,
        "rows": rows,
    }
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(artifact, indent=2, sort_keys=True, ensure_ascii=False) + "\n", encoding="utf-8")

    print(f"anchor-truth review -> {OUT}")
    print(f"cards={carded}/{expected_cards}; citations={artifact['summary']['citations_reviewed']}; "
          f"severity={by_sev}; open M/E/H cards={len(open_findings)} (rows={len(open_rows_full)}); "
          f"oracle: anchors={len(oracle_evidence)}, findings={len(oracle_findings)} (open={len(open_oracle)}); "
          f"resolutions_applied={len(resolutions_applied)}; gate_clean={gate_clean}")
    if open_findings:
        print("open M/E/H cards:", ", ".join(open_findings))
    if open_oracle:
        print("open oracle findings:", "; ".join(f"{o.get('map')}/{o.get('source_id','V'+str(o.get('volume')))}" for o in open_oracle))

    if structural_failures:
        print("\nANCHOR-TRUTH REVIEW: STRUCTURAL FAILURE", file=sys.stderr)
        for f in structural_failures:
            print(f"  - {f}", file=sys.stderr)
        return 2
    if not gate_clean:
        print("\nANCHOR-TRUTH REVIEW: COMPLETE -- open findings remain (AC-5 gate not yet satisfied)")
        return 1
    print("\nANCHOR-TRUTH REVIEW: GATE CLEAN (all M/E/H resolved, all oracle findings accepted, schema ok)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
