#!/usr/bin/env python3
"""Build the negative-test PDF fixtures consumed by
``crates/cacg-ingest/tests/extract_pages_pdfium.rs``.

Outputs:

    tests/parity_corpus/pdfs/zero_pages.pdf
    tests/parity_corpus/pdfs/encrypted.pdf

The bytes are hand-constructed (no PDF library dependency) so the
fixtures are byte-deterministic across regenerations. ``pypdfium2``'s
``PdfDocument.save()`` is explicitly NOT used here because its output
is non-deterministic (creation-time IDs land in the trailer — see
``_research/09_dec_proposed_defaults.md`` line 243).

Run from the workspace root after activating the project ``.venv``:

    .venv/bin/python scripts/build_pdf_negative_fixtures.py
"""
from __future__ import annotations

from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
WORKSPACE_ROOT = Path(__file__).resolve().parents[2]
DEST_DIR = WORKSPACE_ROOT / "tests" / "parity_corpus" / "pdfs"


def _assemble_pdf(objects: list[bytes], trailer_extra: bytes = b"") -> bytes:
    """Assemble a minimal PDF from a list of object bodies.

    Index N in ``objects`` becomes indirect object N+1. Returns
    deterministic bytes: PDF-1.4 header + binary-marker comment + the
    object stream + an xref table whose 10-digit offsets are
    byte-accurate + a trailer carrying ``/Size`` and ``/Root 1 0 R``
    (plus anything in ``trailer_extra`` — used by the encrypted
    fixture to add ``/Encrypt`` and ``/ID``) + the terminating
    ``startxref`` and ``%%EOF`` markers.
    """
    parts: list[bytes] = []
    # PDF header + binary-marker comment (recommended for binary
    # safety so 8-bit-clean tooling treats the file as binary).
    parts.append(b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\n")
    offsets: list[int] = [0]  # index 0 is the free-object head
    for idx, body in enumerate(objects, start=1):
        offsets.append(sum(len(p) for p in parts))
        parts.append(f"{idx} 0 obj\n".encode("ascii"))
        parts.append(body)
        parts.append(b"\nendobj\n")
    xref_offset = sum(len(p) for p in parts)
    parts.append(f"xref\n0 {len(objects) + 1}\n".encode("ascii"))
    parts.append(b"0000000000 65535 f \n")
    for off in offsets[1:]:
        parts.append(f"{off:010d} 00000 n \n".encode("ascii"))
    parts.append(
        f"trailer\n<< /Size {len(objects) + 1} /Root 1 0 R".encode("ascii")
    )
    parts.append(trailer_extra)
    parts.append(b" >>\n")
    parts.append(f"startxref\n{xref_offset}\n%%EOF\n".encode("ascii"))
    return b"".join(parts)


def build_zero_page_pdf_bytes() -> bytes:
    """Return a minimal PDF with a valid xref + catalog + an empty Pages tree.

    The page tree carries ``/Count 0`` and ``/Kids []``. pdfium will
    either load it and report 0 pages or refuse to load it because no
    /MediaBox can be inherited; either outcome surfaces as
    ``IngestError::Corrupt`` (``CACG-INGEST-001``) in ``cacg-ingest``,
    which is what the no-pages negative test asserts.
    """
    return _assemble_pdf(
        [
            b"<< /Type /Catalog /Pages 2 0 R >>",
            b"<< /Type /Pages /Kids [] /Count 0 >>",
        ]
    )


def build_encrypted_pdf_bytes() -> bytes:
    """Return a minimal PDF that pdfium recognizes as encrypted.

    Uses the Standard security handler (algorithm ``/V 1``, revision
    ``/R 2``, RC4-40bit) with all-zero ``/O`` and ``/U`` password
    hashes. The standard handler verifies the user password by
    hashing the supplied password (empty string when no password is
    passed) and comparing to ``/U``; since the hash of the empty
    string is not all zeros, no password attempt — including the
    no-password default — can derive a matching key. pdfium therefore
    rejects the document at the password-verification step, before
    any decryption is attempted.

    Under the standard handler only string and stream content is
    encrypted; dictionary entries (which is all this PDF carries) are
    plain. The single page (object 3) has no ``/Contents`` stream, so
    pdfium's rejection happens at password verification and is not
    masked by a downstream decrypt failure.

    The rejection surfaces as ``IngestError::Corrupt``
    (``CACG-INGEST-001``) in ``cacg-ingest`` — which is what the
    encrypted-PDF negative test asserts.
    """
    # Each of /O and /U must be a 32-byte string per the Standard
    # security handler spec for /V 1 /R 2; b"00" * 32 is 64 hex chars,
    # which the PDF lexer decodes to 32 zero bytes. Do NOT shrink to
    # b"00" * 16 — pdfium would then read a 16-byte hash and produce
    # different (and less stable) reject behavior.
    o_hash = b"<" + b"00" * 32 + b">"
    u_hash = b"<" + b"00" * 32 + b">"
    # /ID entries are 16-byte strings; b"00" * 16 -> 32 hex chars
    # decodes to 16 zero bytes.
    id_half = b"<" + b"00" * 16 + b">"
    encrypt_dict = (
        b"<< /Filter /Standard /V 1 /R 2 "
        + b"/O "
        + o_hash
        + b" /U "
        + u_hash
        + b" /P -4 /Length 40 >>"
    )
    return _assemble_pdf(
        [
            b"<< /Type /Catalog /Pages 2 0 R >>",
            b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
            b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>",
            encrypt_dict,
        ],
        trailer_extra=b" /Encrypt 4 0 R /ID [" + id_half + id_half + b"]",
    )


def main() -> int:
    DEST_DIR.mkdir(parents=True, exist_ok=True)

    fixtures: list[tuple[str, bytes]] = [
        ("zero_pages.pdf", build_zero_page_pdf_bytes()),
        ("encrypted.pdf", build_encrypted_pdf_bytes()),
    ]

    for name, payload in fixtures:
        dest = DEST_DIR / name
        dest.write_bytes(payload)
        print(f"wrote {dest} ({len(payload)} bytes)")

    # Optional sanity check: round-trip through pypdfium2 if available.
    try:
        import pypdfium2 as pdfium  # type: ignore[import-not-found]
    except ImportError:
        print("(pypdfium2 not available; skipping load-back check)")
        return 0
    for name, _ in fixtures:
        path = DEST_DIR / name
        try:
            doc = pdfium.PdfDocument(str(path))
            print(f"pypdfium2 loads {name}: {len(doc)} pages")
        except Exception as err:  # noqa: BLE001 -- diagnostic-only path
            print(f"pypdfium2 cannot load {name} (acceptable): {err}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
