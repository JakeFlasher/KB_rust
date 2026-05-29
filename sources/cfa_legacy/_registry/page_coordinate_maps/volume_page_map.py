#!/usr/bin/env python3
"""Volume-page <-> PDF-page conversion for combined-volume CFA PDFs.

Schema: cfa_legacy.page_coordinate_map.v2 (per-volume affine transforms).

The CFA L1/L2/L3 combined PDFs concatenate 6 volumes that each restart printed
page numbering at 1. A bare PDF page index is ambiguous for citations; the
printed-volume-page form `Vol.<N>/pp.<P-Q>` is the publisher-canonical
addressing. This module converts between the two.

Defense:
- v1 schema had a single global `pdf_coordinate_rule` per source; insufficient
  for multi-volume PDFs.
- v2 adds `volume_table` with per-volume `pdf_page_offset` so each volume's
  printed-page-to-PDF-page transform is independent.

Usage:
    from volume_page_map import load_map, vol_page_to_pdf_page

    m = load_map("cfa_2022_l1_combined.json")
    pdf_pg = vol_page_to_pdf_page(m, volume=6, volume_page=343)  # -> 3612
    vol_pg = pdf_page_to_vol_page(m, pdf_page=3612)              # -> (6, 343)
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Optional


def load_map(path: str | Path) -> dict:
    """Load a v2 page-coordinate-map JSON. Raises if schema_version != v2 for
    multi-volume maps (v1 may still be loaded for single-source maps)."""
    p = Path(path)
    if not p.is_absolute():
        p = Path(__file__).parent / p
    doc = json.loads(p.read_text())
    sv = doc.get("schema_version", "")
    accepted = {
        "cfa_legacy.page_coordinate_map.v1",
        "cfa_legacy.page_coordinate_map.v2",
        "cfa_legacy.page_coordinate_map.v2.1",  # adds content_end_pdf_page + junk_pdf_pages_after
    }
    if sv not in accepted:
        raise ValueError(f"unknown schema_version: {sv}")
    if "volume_table" in doc and sv not in {"cfa_legacy.page_coordinate_map.v2", "cfa_legacy.page_coordinate_map.v2.1"}:
        raise ValueError(f"volume_table requires schema v2 or v2.1; got {sv}")
    return doc


def vol_page_to_pdf_page(doc: dict, volume: int, volume_page: int) -> int:
    """Convert (volume, printed page) to PDF page index.

    Raises ValueError if the volume isn't in the table, or if the volume_page
    is outside the volume's verified range.
    """
    table = doc.get("volume_table")
    if not table:
        raise ValueError("map has no volume_table; not a multi-volume source")
    if volume_page < 1:
        raise ValueError(f"volume_page must be >= 1, got {volume_page}")
    for row in table:
        if row["volume"] == volume:
            offset = row["pdf_page_offset"]
            pdf_page = volume_page + offset
            # Use content_end_pdf_page (v2.1) as the hard upper guard when present.
            # The v2.1 schema adds this field to cap the volume to CFA-legitimate
            # content (e.g. V6 of cfa_2022_l1_combined ends at PDF 3841 even
            # though the PDF file extends to 4353 — the trailing 512 pages are
            # non-CFA material from a flawed iLovePDF concatenation).
            hard_upper = row.get("content_end_pdf_page", row["last_pdf_page"])
            if pdf_page < row["first_pdf_page"] or pdf_page > hard_upper:
                detail = (
                    f"Vol.{volume} page {volume_page} -> PDF {pdf_page} is "
                    f"outside the volume's CFA-content PDF range "
                    f"[{row['first_pdf_page']}, {hard_upper}]"
                )
                if "junk_pdf_pages_after" in row and pdf_page > row["junk_pdf_pages_after"]["start_pdf_page"] - 1:
                    detail += (
                        f"; note: PDF pages {row['junk_pdf_pages_after']['start_pdf_page']}"
                        f"-{row['junk_pdf_pages_after']['end_pdf_page']} contain non-CFA material"
                    )
                raise ValueError(detail)
            return pdf_page
    raise ValueError(f"volume {volume} not in table")


def pdf_page_to_vol_page(doc: dict, pdf_page: int) -> tuple[int, int]:
    """Convert PDF page to (volume, printed page)."""
    table = doc.get("volume_table")
    if not table:
        raise ValueError("map has no volume_table; not a multi-volume source")
    for row in table:
        if row["first_pdf_page"] <= pdf_page <= row["last_pdf_page"]:
            volume_page = pdf_page - row["pdf_page_offset"]
            return (row["volume"], volume_page)
    raise ValueError(f"PDF page {pdf_page} not in any volume range")


def _self_test():
    """Self-test: load cfa_2022_l1_combined.json and verify known anchors."""
    here = Path(__file__).parent
    m = load_map("cfa_2022_l1_combined.json")
    # Test known anchor: cc-material card cites Vol.6/pp.343-354
    pdf_343 = vol_page_to_pdf_page(m, volume=6, volume_page=343)
    assert pdf_343 == 3612, f"V6/343 -> PDF expected 3612, got {pdf_343}"
    pdf_354 = vol_page_to_pdf_page(m, volume=6, volume_page=354)
    assert pdf_354 == 3623, f"V6/354 -> PDF expected 3623, got {pdf_354}"
    # Round-trip
    v, p = pdf_page_to_vol_page(m, 3612)
    assert (v, p) == (6, 343), f"PDF 3612 -> ({v}, {p}), expected (6, 343)"
    # Verify all evidence points in the map
    for vol_row in m["volume_table"]:
        for ev in vol_row.get("verified_evidence", []):
            pdf = vol_page_to_pdf_page(m, vol_row["volume"], ev["volume_page"])
            assert pdf == ev["pdf_page"], (
                f"V{vol_row['volume']}/{ev['volume_page']} -> PDF expected "
                f"{ev['pdf_page']}, got {pdf}"
            )
    # Out-of-range should raise
    try:
        vol_page_to_pdf_page(m, volume=1, volume_page=10000)
    except ValueError:
        pass
    else:
        raise AssertionError("expected ValueError for out-of-range volume_page")
    try:
        pdf_page_to_vol_page(m, 999999)
    except ValueError:
        pass
    else:
        raise AssertionError("expected ValueError for out-of-range pdf_page")
    print("OK: volume_page_map self-test passed")


if __name__ == "__main__":
    _self_test()
