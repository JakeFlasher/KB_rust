#!/usr/bin/env python3
"""Register the two template-deck migration sources -- reading
``10_behavioral_finance`` (6 net-new + 2 already-authorized books) and reading
``11_risk_management`` (8 net-new + 1 already-authorized book) -- into the cfa
ingest + authorization + catalog registries.

This REUSES the proven pure helpers from ``register_migration_sources.py``
(``copy_and_verify``, ``merge_allowed``, ``ingest_row``, the serializers) so the
already-shipped 14/15/22 registration path is left completely untouched (Codex
recommendation Q2: minimize risk to the proven path). The two template decks differ
from the ``deferred_books/<set>/`` layout the original registrar assumes -- their
``_sources.json`` lives under ``<deck>/_card_skeletons/`` and the PDFs sit at the
deck root -- so they are registered here instead of by extending the original.

For every source in each deck's ``_sources.json`` this:
  * copies the incoming PDF to ``sources/cfa/pdfs/<reading_id>/<source_id>.pdf``
    (SHA-256-verified, idempotent: an already-present byte-identical copy is left
    alone -- this dedupes the 3 books that are already in-repo);
  * appends one ``ingest_plan.json`` row per not-yet-registered ``source_id``;
  * extends ``source_matrix.json`` (+ ``.pretty`` + ``out/cfa/source_matrix.json``)
    preserving every existing authorization;
  * adds one ``library_catalog.json`` provenance record per net-new source
    (append-only, with a cpp text-quality probe as the audit rating).

Fail-closed (a SHA mismatch / missing PDF aborts before any write) and idempotent
(re-running with the registries already updated is a no-op). ``--self-test`` drives
the pure helpers; ``--write`` performs the registration.
"""

from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any

import register_migration_sources as base

REGISTRY = base.REGISTRY
PDFS_DIR = base.PDFS_DIR
INGEST_PLAN = base.INGEST_PLAN
MATRIX_COMPACT = base.MATRIX_COMPACT
MATRIX_PRETTY = base.MATRIX_PRETTY
MATRIX_OUT = base.MATRIX_OUT
LIBRARY_CATALOG = REGISTRY / "library_catalog.json"

CFA_READING = Path("/home/jakeshea/CFA_reading")

# (reading_id, deck_root, sources_json, origin_subdir for legacy_path)
TEMPLATE_DECKS = (
    (
        "10_behavioral_finance",
        CFA_READING / "10_Behavioral_Finance",
        CFA_READING / "10_Behavioral_Finance/_card_skeletons/_sources.json",
        "10_Behavioral_Finance",
    ),
    (
        "11_risk_management",
        CFA_READING / "11_Risk_Management",
        CFA_READING / "11_Risk_Management/_card_skeletons/_sources.json",
        "11_Risk_Management",
    ),
)

# Conservative, accurate edition descriptors for the net-new sources (the year is
# already vetted inside each source_id; edition numbers are stated only where the
# filename / source_id carries them explicitly).
EDITIONS = {
    "bf_wakker_2010_pt_risk_ambiguity": "Wakker (2010) Prospect Theory: For Risk and Ambiguity (Cambridge University Press)",
    "bf_hbe_vol1_2018": "Bernheim, DellaVigna & Laibson, eds. (2018) Handbook of Behavioral Economics: Foundations and Applications, Vol. 1 (Elsevier/North-Holland)",
    "bf_hbe_vol2_2019": "Bernheim, DellaVigna & Laibson, eds. (2019) Handbook of Behavioral Economics: Foundations and Applications, Vol. 2 (Elsevier/North-Holland)",
    "bf_compecon_v4_2018_ham": "Hommes & LeBaron, eds. (2018) Handbook of Computational Economics, Vol. 4: Heterogeneous Agent Modeling (Elsevier/North-Holland)",
    "bf_barberis_jin_wang_2021_pt_anomalies": "Barberis, Jin & Wang (2021) Prospect Theory and Stock Market Anomalies, Journal of Finance",
    "bf_pompian_2006_bfwm": "Pompian (2006) Behavioral Finance and Wealth Management, 1st ed. (Wiley) [on-disk 1e; CFA-L3 2e (2012) framing flagged per Critical Rule 6]",
    "rm_hull_2023_rmfi": "Hull (2023) Risk Management and Financial Institutions (Wiley)",
    "rm_christoffersen_2012_elements": "Christoffersen (2012) Elements of Financial Risk Management, 2nd ed. (Academic Press)",
    "rm_embrechts_kluppelberg_mikosch_1997_modelling_extremal_events": "Embrechts, Klüppelberg & Mikosch (1997) Modelling Extremal Events for Insurance and Finance (Springer)",
    "rm_gregory_2020_xva_challenge": "Gregory (2020) The xVA Challenge: Counterparty Risk, Funding, Collateral, Capital and Initial Margin (Wiley)",
    "rm_bouchaud_potters_2003_theory_financial_risk": "Bouchaud & Potters (2003) Theory of Financial Risk and Derivative Pricing, 2nd ed. (Cambridge University Press)",
    "rm_potters_bouchaud_2020_random_matrix_theory": "Potters & Bouchaud (2020) A First Course in Random Matrix Theory (Cambridge University Press)",
    "rm_sornette_2017_why_stock_markets_crash": "Sornette (2017) Why Stock Markets Crash: Critical Events in Complex Financial Systems (Princeton University Press)",
    "rm_follmer_schied_2025_stochastic_finance": "Föllmer & Schied (2025) Stochastic Finance: An Introduction in Discrete Time, 5th ed. (De Gruyter)",
}


# --------------------------------------------------------------------------- #
# Pure helpers.
# --------------------------------------------------------------------------- #

def serialize_catalog(obj: Any) -> bytes:
    """library_catalog.json uses indent=2, raw UTF-8, and PRESERVES top-level key
    order (it is NOT sort_keys) -- match it so the diff stays append-only."""
    return json.dumps(obj, ensure_ascii=False, indent=2).encode("utf-8") + b"\n"


def audit_label(cpp: float) -> str:
    if cpp >= 1000:
        return f"GOOD ({cpp} cpp)"
    if cpp >= 300:
        return f"OK ({cpp} cpp)"
    return f"SCAN ({cpp} cpp)"


def catalog_record(
    *,
    source_id: str,
    reading_id: str,
    sha256: str,
    pages: int,
    size_bytes: int,
    role: str,
    legacy_path: str,
    cpp: float,
) -> dict[str, Any]:
    """One library_catalog active record, keys in the same alphabetical order the
    existing records use."""
    primary = "primary" if role.startswith("primary") else "supporting"
    return {
        "audit_rating": audit_label(cpp),
        "cfa_relevance_hint": "core" if primary == "primary" else "adjacent",
        "citation_method": "pp.<N-M>",
        "edition": EDITIONS.get(source_id, source_id),
        "format": "PDF",
        "legacy_path": legacy_path,
        "library_path": f"pdfs/{reading_id}/{source_id}.pdf",
        "page_count": pages,
        "primary_or_supporting": primary,
        "quotable": "yes",
        "review_flags": (["edition-1e-on-disk-vs-cfa-2e"] if source_id == "bf_pompian_2006_bfwm" else []),
        "sha256": sha256,
        "shared_anchor": False,
        "size_bytes": size_bytes,
        "source_id": source_id,
        "subcorpus_authorizations": [reading_id[:2]],
    }


# --------------------------------------------------------------------------- #
# Registration.
# --------------------------------------------------------------------------- #

def load_deck_sources() -> list[dict[str, Any]]:
    records: list[dict[str, Any]] = []
    for reading_id, deck_root, sources_json, origin_subdir in TEMPLATE_DECKS:
        data = json.loads(Path(sources_json).read_text(encoding="utf-8"))
        for s in data["sources"]:
            records.append({
                "reading_id": reading_id,
                "source_id": str(s["source_id"]),
                "src_pdf": Path(deck_root) / str(s["file"]),
                "sha256": str(s["sha256"]),
                "pages": int(s["pages"]),
                "role": str(s.get("role", "")),
                "legacy_path": f"{origin_subdir}/{s['file']}",
            })
    return records


def cpp_probe(pdf: Path, pages: int) -> float:
    """Chars-per-page over a mid-book 40-page window (skips front matter) as a
    text-layer quality audit."""
    lo = max(1, int(pages * 0.3))
    hi = min(pages, lo + 39)
    try:
        out = subprocess.run(
            ["pdftotext", "-f", str(lo), "-l", str(hi), "-layout", str(pdf), "-"],
            capture_output=True, text=True, timeout=180,
        ).stdout
    except Exception:
        return 0.0
    span = hi - lo + 1
    return round(len(out) / span, 1) if span else 0.0


def run(write: bool) -> dict[str, Any]:
    records = load_deck_sources()

    pdf_status: dict[str, str] = {}
    for r in records:
        pdf_status[r["source_id"]] = base.copy_and_verify(r) if write else "dry-run"

    # ingest_plan: append rows for any not-yet-registered source_id.
    plan = json.loads(INGEST_PLAN.read_text(encoding="utf-8"))
    existing_ids = {str(row["source_id"]) for row in plan}
    plan_added = 0
    for r in sorted(records, key=lambda x: x["source_id"]):
        if r["source_id"] not in existing_ids:
            plan.append(base.ingest_row(r["reading_id"], r["source_id"]))
            existing_ids.add(r["source_id"])
            plan_added += 1

    # source matrix: extend the two readings, preserving every existing authorization.
    matrix = json.loads(MATRIX_COMPACT.read_text(encoding="utf-8"))
    additions: dict[str, list[str]] = {}
    for r in records:
        additions.setdefault(r["reading_id"], []).append(r["source_id"])
    matrix["allowed"] = base.merge_allowed(matrix.get("allowed", {}), additions)

    # library_catalog: append one record per net-new source (idempotent by id).
    catalog = json.loads(LIBRARY_CATALOG.read_text(encoding="utf-8"))
    cat_added = 0
    for r in records:
        bucket = catalog["active"].setdefault(r["reading_id"], [])
        if any(rec.get("source_id") == r["source_id"] for rec in bucket):
            continue
        cpp = cpp_probe(r["src_pdf"], r["pages"]) if write else 0.0
        bucket.append(catalog_record(
            source_id=r["source_id"], reading_id=r["reading_id"], sha256=r["sha256"],
            pages=r["pages"], size_bytes=r["src_pdf"].stat().st_size, role=r["role"],
            legacy_path=r["legacy_path"], cpp=cpp,
        ))
        cat_added += 1
    active_recs = [rec for v in catalog["active"].values() for rec in v]
    catalog["counts"]["active_files"] = len(active_recs)
    catalog["counts"]["active_unique_source_ids"] = len({rec["source_id"] for rec in active_recs})
    catalog["counts"]["total_files"] = (
        catalog["counts"]["active_files"]
        + catalog["counts"]["deferred_files"]
        + catalog["counts"]["excluded_files"]
    )

    if write:
        base.atomic_write_bytes(INGEST_PLAN, base.serialize_plan(plan))
        base.atomic_write_bytes(MATRIX_COMPACT, base.serialize_compact(matrix))
        base.atomic_write_bytes(MATRIX_PRETTY, base.serialize_pretty(matrix))
        base.atomic_write_bytes(MATRIX_OUT, base.serialize_pretty(matrix))
        base.atomic_write_bytes(LIBRARY_CATALOG, serialize_catalog(catalog))

    return {
        "deck_sources": len(records),
        "pdf_status": pdf_status,
        "ingest_plan_rows_added": plan_added,
        "ingest_plan_total": len(plan),
        "library_catalog_records_added": cat_added,
        "library_catalog_active_total": catalog["counts"]["active_files"],
        "matrix_keys": {rid: matrix["allowed"][rid] for rid, _, _, _ in TEMPLATE_DECKS},
    }


# --------------------------------------------------------------------------- #
# Self-test.
# --------------------------------------------------------------------------- #

def self_test() -> int:
    failures: list[str] = []

    # audit_label thresholds
    if audit_label(1500.0) != "GOOD (1500.0 cpp)":
        failures.append("audit_label GOOD")
    if audit_label(500.0) != "OK (500.0 cpp)":
        failures.append("audit_label OK")
    if audit_label(100.0) != "SCAN (100.0 cpp)":
        failures.append("audit_label SCAN")

    # catalog_record key order == the on-disk record key order
    rec = catalog_record(
        source_id="rm_hull_2023_rmfi", reading_id="11_risk_management",
        sha256="ab" * 32, pages=833, size_bytes=123, role="primary-x",
        legacy_path="11_Risk_Management/x.pdf", cpp=1234.5,
    )
    expected_keys = [
        "audit_rating", "cfa_relevance_hint", "citation_method", "edition", "format",
        "legacy_path", "library_path", "page_count", "primary_or_supporting", "quotable",
        "review_flags", "sha256", "shared_anchor", "size_bytes", "source_id",
        "subcorpus_authorizations",
    ]
    if list(rec.keys()) != expected_keys:
        failures.append(f"catalog_record key order: {list(rec.keys())}")
    if rec["library_path"] != "pdfs/11_risk_management/rm_hull_2023_rmfi.pdf":
        failures.append("catalog_record library_path")
    if rec["subcorpus_authorizations"] != ["11"]:
        failures.append("catalog_record subcorpus_authorizations")
    if rec["primary_or_supporting"] != "primary" or rec["cfa_relevance_hint"] != "core":
        failures.append("catalog_record primary mapping")

    # pompian carries the edition review flag
    pom = catalog_record(
        source_id="bf_pompian_2006_bfwm", reading_id="10_behavioral_finance",
        sha256="cd" * 32, pages=338, size_bytes=1, role="primary-pompian",
        legacy_path="10_Behavioral_Finance/x.pdf", cpp=900.0,
    )
    if pom["review_flags"] != ["edition-1e-on-disk-vs-cfa-2e"]:
        failures.append("pompian edition flag missing")

    # catalog serializer preserves top-level order (NOT sort_keys)
    obj = {"active": {}, "counts": {}, "zzz": 1, "aaa": 2}
    out = serialize_catalog(obj).decode("utf-8")
    if out.index('"active"') > out.index('"counts"') or out.index('"zzz"') > out.index('"aaa"'):
        failures.append("serialize_catalog reordered keys (should preserve insertion order)")

    # every declared deck _sources.json exists and parses, every PDF is present on disk
    for deck in TEMPLATE_DECKS:
        deck_root, sources_json = deck[1], deck[2]
        sj = Path(sources_json)
        if not sj.is_file():
            failures.append(f"missing _sources.json: {sj}")
            continue
        data = json.loads(sj.read_text(encoding="utf-8"))
        for s in data["sources"]:
            if not (Path(deck_root) / str(s["file"])).is_file():
                failures.append(f"missing source PDF: {deck_root}/{s['file']}")

    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print(f"  - {f}")
        return 1
    print("SELF-TEST PASSED (audit_label + catalog_record shape + serializer order + deck _sources.json/PDF presence)")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run hermetic self-test and exit")
    parser.add_argument("--write", action="store_true", help="copy PDFs and write the registries")
    args = parser.parse_args()

    if args.self_test:
        return self_test()

    result = run(write=args.write)
    print(json.dumps(result, ensure_ascii=False, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
