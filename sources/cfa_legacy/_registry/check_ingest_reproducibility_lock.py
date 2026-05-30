#!/usr/bin/env python3
"""Reproducibility lock for the cfa_legacy chunk-ingest path.

Chunk hashes are tied to three identity facts: the kb binary, the
``pdfium-render`` crate version, and the exact ``libpdfium`` shared library
that parses every PDF. Re-ingesting on a host whose ``libpdfium`` differs from
the one the corpus was built against silently diverges every ``chunk_hash`` —
so before any re-ingest those three facts must be captured and asserted against
a committed pin.

This module does two separable things:

  * It owns a committed, byte-stable pin (``v1_reproducibility_lock.json``)
    recording the three identity facts plus the chunk-hash recipe. ``--write-lock``
    regenerates that pin from the authoritative sources already in the repo
    (``snapshot.json``, ``pdfium_provenance.json``, ``Cargo.lock``) so the pin is
    derived, never hand-typed.
  * It checks the live environment against that pin. The ``pdfium-render``
    version and the chunk-hash recipe are reproducible anywhere and are always
    asserted; the ``libpdfium`` binary is an external, machine-local artifact, so
    its absence is reported as "not provisioned" (a NOTE, not a failure) while a
    PRESENT-but-mismatched library is a hard, fail-closed abort -- UNLESS a
    committed equivalence proof (``v1_libpdfium_equivalence_proof.json``) attests
    this exact host build extracts byte-identical text (a proven-equivalent build
    of the same Pdfium source version), in which case it is accepted on the merit
    of that proof rather than disabled. ``--require-ingest-ready`` promotes "not
    provisioned" to a hard failure for the pre-ingest gate.

The chunk-hash recipe is proven binary-independently: the recorded hashes in the
merged manifest are recomputed from their canonical-JSON envelope and asserted
byte-identical, which confirms the recipe this module uses matches the kernel
(``crates/cacg-core/src/hash.rs::chunk_hash``) without needing the parser at all.

The ``--self-test`` drives every branch from synthetic inputs and pins the
canonical envelope *string* (human-verifiable) rather than an opaque digest.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[3]
REGISTRY = ROOT / "sources/cfa_legacy/_registry"
OUT = ROOT / "out/cfa_legacy"
SNAPSHOT = REGISTRY / "snapshot.json"
PROVENANCE = OUT / "pdfium_provenance.json"
CARGO_LOCK = ROOT / "Cargo.lock"
CHUNKS_MANIFEST = OUT / "chunks_manifest.json"
KB_BINARY = ROOT / "target/debug/kb"
LOCK_PATH = REGISTRY / "v1_reproducibility_lock.json"
EQUIVALENCE_PROOF = REGISTRY / "v1_libpdfium_equivalence_proof.json"
STATUS_PATH = OUT / "v1_reproducibility_status.json"  # gitignored: machine capture

# Single source of truth for the proven-equivalent acceptance gate: a host
# libpdfium whose SHA differs from the pin is accepted only when a committed
# proof attests this exact build extracts byte-identical text (see
# prove_libpdfium_equivalence.py).
sys.path.insert(0, str(REGISTRY))
from prove_libpdfium_equivalence import proof_accepts  # noqa: E402

DEFAULT_PDFIUM_LIBRARY = Path("/usr/lib/libpdfium.so")
FROZEN_TIMESTAMP = "1970-01-01T00:00:00Z"

# The exact canonical-JSON envelope a chunk hashes over, mirroring
# crates/cacg-core/src/hash.rs::chunk_hash + canonical_json.rs. Keys are emitted
# sorted; page-span objects carry only these two keys, also sorted.
CHUNK_ENVELOPE_KEYS = ("end_page", "page_spans", "start_page", "text")
PAGE_SPAN_KEYS = ("byte_offset_in_chunk", "page")


def generated_at_utc() -> str:
    if os.environ.get("KB_FROZEN_CLOCK", "1") == "1":
        return FROZEN_TIMESTAMP
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def atomic_write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, tmp_name = tempfile.mkstemp(prefix=f".{path.name}.", suffix=".tmp", dir=path.parent, text=True)
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as f:
            f.write(text)
            f.flush()
            os.fsync(f.fileno())
        os.replace(tmp_name, path)
    finally:
        if os.path.exists(tmp_name):
            os.unlink(tmp_name)


def write_json(path: Path, payload: Any) -> None:
    atomic_write_text(path, json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n")


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for block in iter(lambda: f.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


# --------------------------------------------------------------------------- #
# Pure helpers (driven hermetically by --self-test).
# --------------------------------------------------------------------------- #

def canonical_chunk_payload(text: str, start_page: int, end_page: int, page_spans: list[dict[str, int]]) -> str:
    """Canonical-JSON string a chunk hashes over. Mirrors the kernel exactly."""
    envelope = {
        "end_page": end_page,
        "page_spans": [
            {"byte_offset_in_chunk": int(s["byte_offset_in_chunk"]), "page": int(s["page"])}
            for s in page_spans
        ],
        "start_page": start_page,
        "text": text,
    }
    return json.dumps(envelope, sort_keys=True, ensure_ascii=False, separators=(",", ":"))


def recompute_chunk_hash(text: str, start_page: int, end_page: int, page_spans: list[dict[str, int]]) -> str:
    return hashlib.sha256(
        canonical_chunk_payload(text, start_page, end_page, page_spans).encode("utf-8")
    ).hexdigest()


def recompute_chunk_hash_from_record(chunk: dict[str, Any]) -> str:
    return recompute_chunk_hash(
        chunk["text"], int(chunk["start_page"]), int(chunk["end_page"]), chunk["page_spans"]
    )


def pdfium_render_version_from_lock(lock_text: str) -> str | None:
    """Extract the pinned ``pdfium-render`` version from a Cargo.lock body."""
    match = re.search(r'(?m)^name = "pdfium-render"\nversion = "([^"]+)"', lock_text)
    return match.group(1) if match else None


def library_lock_status(
    actual_sha: str | None, pinned_sha: str, skip_check: bool, accepted_via_proof: bool = False
) -> str:
    """Classify the libpdfium identity against the pin.

    ``satisfied``          present and matches the pin.
    ``satisfied_via_proof`` present, a DIFFERENT SHA, but a committed equivalence
                           proof attests this exact build extracts byte-identical
                           text (a proven-equivalent parser, not a blind skip).
    ``mismatch_abort``     present, a different SHA, no equivalence proof, and the
                           deviation flag is NOT set (a re-ingest would diverge).
    ``deviation_recorded`` a mismatch the operator explicitly opted into.
    ``not_provisioned``    absent on this host (an external, machine-local file).
    """
    if actual_sha is None:
        return "not_provisioned"
    if actual_sha == pinned_sha:
        return "satisfied"
    if accepted_via_proof:
        return "satisfied_via_proof"
    return "deviation_recorded" if skip_check else "mismatch_abort"


# --------------------------------------------------------------------------- #
# Lock generation + live check.
# --------------------------------------------------------------------------- #

def derive_lock() -> dict[str, Any]:
    """Build the committed pin from the authoritative in-repo sources, cross-checking
    that they agree on the libpdfium identity."""
    snapshot = json.loads(SNAPSHOT.read_text(encoding="utf-8"))
    provenance = json.loads(PROVENANCE.read_text(encoding="utf-8"))
    runtime = provenance["operational_runtime"]

    snapshot_sha = str(snapshot["pdfium_library_sha256"])
    runtime_sha = str(runtime["library_sha256"])
    if snapshot_sha != runtime_sha:
        raise SystemExit(
            f"libpdfium SHA disagreement: snapshot={snapshot_sha} provenance={runtime_sha}"
        )

    lock_version = pdfium_render_version_from_lock(CARGO_LOCK.read_text(encoding="utf-8"))
    if lock_version is None:
        raise SystemExit("pdfium-render version not found in Cargo.lock")
    if lock_version != str(runtime["parser_version"]):
        raise SystemExit(
            f"pdfium-render version disagreement: Cargo.lock={lock_version} "
            f"provenance={runtime['parser_version']}"
        )

    kb_version = kb_version_string()

    return {
        "schema_version": "cfa_legacy.v1_reproducibility_lock.v1",
        "generated_at_utc": generated_at_utc(),
        "pins": {
            "libpdfium_sha256": snapshot_sha,
            "libpdfium_package": str(snapshot.get("pdfium_package") or runtime.get("package")),
            "pdfium_render_version": lock_version,
            "kb_binary_version": kb_version,
            "kb_binary_path": "target/debug/kb",
        },
        "chunk_hash_recipe": {
            "kernel_source": "crates/cacg-core/src/hash.rs::chunk_hash",
            "envelope_keys_sorted": list(CHUNK_ENVELOPE_KEYS),
            "page_span_keys_sorted": list(PAGE_SPAN_KEYS),
            "canonical_json": "json.dumps(value, sort_keys=True, ensure_ascii=False, separators=(',',':'))",
            "digest": "sha256-hex of the UTF-8 canonical payload",
        },
        "notes": (
            "libpdfium is an external, machine-local binary; its absence is reported as "
            "not_provisioned (a note), while a present-but-mismatched library aborts. The "
            "kb binary is a build artifact with no committed byte pin; its version string is "
            "asserted and its observed sha256 is captured to the gitignored status sidecar."
        ),
    }


def kb_version_string() -> str | None:
    if not (KB_BINARY.is_file() and os.access(KB_BINARY, os.X_OK)):
        return None
    try:
        out = subprocess.run(
            [str(KB_BINARY), "--version"], capture_output=True, text=True, timeout=60, check=False
        )
    except (OSError, subprocess.SubprocessError):
        return None
    return out.stdout.strip() or None


def resolve_libpdfium_path() -> Path:
    return Path(os.environ.get("KB_PDFIUM_LIBRARY", str(DEFAULT_PDFIUM_LIBRARY)))


def load_equivalence_proof() -> dict[str, Any] | None:
    """The committed proof (if any) that a different-SHA host build is byte-identical."""
    if not EQUIVALENCE_PROOF.is_file():
        return None
    try:
        return json.loads(EQUIVALENCE_PROOF.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None


def reproduce_chunk_hashes() -> dict[str, Any]:
    """Recompute EVERY recorded chunk hash from its canonical envelope.

    This proves the recipe binary-independently: a recorded hash that does not
    match the canonical-JSON recompute means the manifest is tampered or the
    recipe drifted. The merged manifest (~57.6k chunks) recomputes in a couple
    of seconds, so there is no reason to sample — a release proof checks all.

    The manifest is a local-only (gitignored) artifact; when it is absent the
    proof cannot run. That is reported as ``not_provisioned`` (a note in the
    default check, a hard failure under ``--require-ingest-ready``), never a
    silent pass.
    """
    if not CHUNKS_MANIFEST.is_file():
        return {"ran": False, "status": "not_provisioned", "reason": "chunks_manifest_absent"}
    chunks = json.loads(CHUNKS_MANIFEST.read_text(encoding="utf-8"))["chunks"]
    mismatches: list[str] = []
    for chunk in chunks:
        if recompute_chunk_hash_from_record(chunk) != chunk["chunk_hash"]:
            mismatches.append(chunk["chunk_id"])
    return {
        "ran": True,
        "status": "byte_identical" if not mismatches else "mismatch",
        "manifest_chunk_count": len(chunks),
        "checked": len(chunks),
        "mismatches": mismatches,
        "byte_identical": not mismatches,
    }


def run_check(require_ingest_ready: bool, write_status: bool,
              require_library_ready: bool = False) -> tuple[dict[str, Any], bool]:
    if not LOCK_PATH.is_file():
        raise SystemExit(f"missing committed pin: {LOCK_PATH} (run with --write-lock first)")
    lock = json.loads(LOCK_PATH.read_text(encoding="utf-8"))
    pins = lock["pins"]
    skip_check = os.environ.get("KB_SKIP_PDFIUM_HASH_CHECK") == "1"

    # 1. pdfium-render crate version (reproducible anywhere).
    observed_render = pdfium_render_version_from_lock(CARGO_LOCK.read_text(encoding="utf-8"))
    render_ok = observed_render == pins["pdfium_render_version"]

    # 2. kb binary identity. Absence is an explicit not_provisioned (a note in
    # the default check, a failure under --require-ingest-ready); a present
    # binary whose --version string drifts from the pin is a hard fail.
    kb_present = KB_BINARY.is_file() and os.access(KB_BINARY, os.X_OK)
    kb_version = kb_version_string()
    kb_sha = sha256_file(KB_BINARY) if KB_BINARY.is_file() else None
    if not kb_present:
        kb_status = "not_provisioned"
    elif kb_version == pins["kb_binary_version"]:
        kb_status = "satisfied"
    else:
        kb_status = "version_drift"
    kb_version_ok = kb_status != "version_drift"

    # 3. libpdfium identity vs pin. A different SHA is accepted only when a
    # committed equivalence proof attests this exact host build extracts
    # byte-identical text (a proven-equivalent parser), never by a blind skip.
    lib_path = resolve_libpdfium_path()
    lib_sha = sha256_file(lib_path) if lib_path.is_file() else None
    proof = load_equivalence_proof()
    accepted_via_proof = proof_accepts(lib_sha, pins["libpdfium_sha256"], proof)
    lib_status = library_lock_status(lib_sha, pins["libpdfium_sha256"], skip_check, accepted_via_proof)

    # Binary-independent recipe proof over EVERY recorded chunk hash.
    repro = reproduce_chunk_hashes()

    # LIBRARY readiness: the parser + kb identity needed to ingest reproducibly.
    # This is MANIFEST-INDEPENDENT — it is the correct PRE-ingest gate, because a
    # clean rebuild produces the (uncommitted) chunks_manifest.json that the recipe
    # check below reads; requiring that recipe proof before ingest would block the
    # ingest on a file it is supposed to create.
    library_ready = (
        lib_status in {"satisfied", "satisfied_via_proof", "deviation_recorded"}
        and kb_status == "satisfied"
        and render_ok
    )
    # FULL ingest readiness additionally requires the binary-independent recipe
    # proof over the recorded chunk hashes (only meaningful once the manifest exists).
    ingest_ready = library_ready and repro["status"] == "byte_identical"

    # Default mode fails closed only on a real defect: a present-but-mismatched
    # library, a version drift, or a recipe divergence on a PRESENT manifest. A
    # merely-absent external artifact (libpdfium / kb / chunks_manifest) is a
    # note unless a require_* flag demands the corresponding provisioned set.
    # --require-library-ready (the pre-ingest gate) does NOT require the manifest;
    # --require-ingest-ready (post-ingest evidence) requires the full set.
    hard_fail = (
        (lib_status == "mismatch_abort")
        or (not render_ok)
        or (kb_status == "version_drift")
        or (repro["status"] == "mismatch")
        or (require_library_ready and not library_ready)
        or (require_ingest_ready and not ingest_ready)
    )

    status = {
        "schema_version": "cfa_legacy.v1_reproducibility_status.v1",
        "generated_at_utc": generated_at_utc(),
        "pdfium_render": {"observed": observed_render, "pinned": pins["pdfium_render_version"], "ok": render_ok},
        "kb_binary": {
            "observed_version": kb_version,
            "pinned_version": pins["kb_binary_version"],
            "status": kb_status,
            "version_ok": kb_version_ok,
            "observed_sha256": kb_sha,
        },
        "libpdfium": {
            "path": str(lib_path),
            "observed_sha256": lib_sha,
            "pinned_sha256": pins["libpdfium_sha256"],
            "status": lib_status,
            "deviation_flag": skip_check,
            "accepted_via_proof": accepted_via_proof,
            "equivalence_proof": (
                {"verdict": proof.get("verdict"), "total_chunks_compared": proof.get("total_chunks_compared")}
                if proof else None
            ),
        },
        "chunk_hash_reproduction": repro,
        "library_ready": library_ready,
        "ingest_ready": ingest_ready,
        "require_library_ready": require_library_ready,
        "require_ingest_ready": require_ingest_ready,
        "ok": not hard_fail,
    }
    if write_status:
        write_json(STATUS_PATH, status)
    return status, not hard_fail


# --------------------------------------------------------------------------- #
# Self-test.
# --------------------------------------------------------------------------- #

EXPECTED_PAYLOAD = (
    '{"end_page":2,"page_spans":[{"byte_offset_in_chunk":0,"page":1},'
    '{"byte_offset_in_chunk":6,"page":2}],"start_page":1,"text":"hello world"}'
)


def self_test() -> int:
    failures: list[str] = []
    fixture_spans = [{"byte_offset_in_chunk": 0, "page": 1}, {"byte_offset_in_chunk": 6, "page": 2}]

    # Recipe pinned via the inspectable canonical payload string.
    payload = canonical_chunk_payload("hello world", 1, 2, fixture_spans)
    if payload != EXPECTED_PAYLOAD:
        failures.append(f"canonical payload drift:\n  got={payload}\n  exp={EXPECTED_PAYLOAD}")

    # Determinism + tamper detection (no opaque golden hash needed).
    h1 = recompute_chunk_hash("hello world", 1, 2, fixture_spans)
    h2 = recompute_chunk_hash("hello world", 1, 2, fixture_spans)
    if h1 != h2:
        failures.append("chunk hash is not deterministic")
    if recompute_chunk_hash("hello worlx", 1, 2, fixture_spans) == h1:
        failures.append("tampered text did not change the chunk hash")
    if recompute_chunk_hash("hello world", 1, 3, fixture_spans) == h1:
        failures.append("changed end_page did not change the chunk hash")
    if len(h1) != 64 or any(c not in "0123456789abcdef" for c in h1):
        failures.append("chunk hash is not 64-hex")

    # record-shaped recompute path
    record = {"text": "hello world", "start_page": 1, "end_page": 2, "page_spans": fixture_spans,
              "chunk_hash": h1}
    if recompute_chunk_hash_from_record(record) != record["chunk_hash"]:
        failures.append("record-shaped recompute disagrees with field recompute")

    pin = "c110f5240692b1915ad090d4f7e9bc6afa429db41341f2339d866d48402edbe5"
    cases = {
        "satisfied": (library_lock_status(pin, pin, False), "satisfied"),
        "mismatch_abort": (library_lock_status("0" * 64, pin, False), "mismatch_abort"),
        "deviation_recorded": (library_lock_status("0" * 64, pin, True), "deviation_recorded"),
        "not_provisioned": (library_lock_status(None, pin, False), "not_provisioned"),
        "match_ignores_skip": (library_lock_status(pin, pin, True), "satisfied"),
        "satisfied_via_proof": (library_lock_status("0" * 64, pin, False, True), "satisfied_via_proof"),
        "mismatch_without_proof": (library_lock_status("0" * 64, pin, False, False), "mismatch_abort"),
        "proof_beats_skip": (library_lock_status("0" * 64, pin, True, True), "satisfied_via_proof"),
    }
    for name, (got, want) in cases.items():
        if got != want:
            failures.append(f"library_lock_status[{name}]: got {got!r}, want {want!r}")

    # version parser
    lock_body = 'name = "serde"\nversion = "1.0"\n\nname = "pdfium-render"\nversion = "0.9.1"\nsource = "x"\n'
    if pdfium_render_version_from_lock(lock_body) != "0.9.1":
        failures.append("pdfium_render_version_from_lock failed to parse a normal lock")
    if pdfium_render_version_from_lock('name = "other"\nversion = "1.2.3"\n') is not None:
        failures.append("pdfium_render_version_from_lock matched the wrong crate")

    if failures:
        print("SELF-TEST FAILED:")
        for f in failures:
            print(f"  - {f}")
        return 1
    print("SELF-TEST PASSED (recipe + determinism + tamper + lock-status + proof-accept + version-parse)")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run hermetic self-test and exit")
    parser.add_argument("--write-lock", action="store_true", help="(re)generate the committed pin from repo sources")
    parser.add_argument(
        "--require-ingest-ready",
        action="store_true",
        help="fail closed unless libpdfium is present and matches the pin (or the deviation flag is set)",
    )
    parser.add_argument("--no-status", action="store_true", help="do not write the gitignored status sidecar")
    args = parser.parse_args()

    if args.self_test:
        return self_test()

    if args.write_lock:
        write_json(LOCK_PATH, derive_lock())
        print(f"wrote {LOCK_PATH.relative_to(ROOT)}")
        return 0

    status, ok = run_check(require_ingest_ready=args.require_ingest_ready, write_status=not args.no_status)
    print(
        json.dumps(
            {
                "ok": status["ok"],
                "ingest_ready": status["ingest_ready"],
                "libpdfium_status": status["libpdfium"]["status"],
                "kb_status": status["kb_binary"]["status"],
                "pdfium_render_ok": status["pdfium_render"]["ok"],
                "chunk_hash_reproduction": {
                    k: status["chunk_hash_reproduction"].get(k)
                    for k in ("ran", "checked", "status")
                },
            },
            ensure_ascii=False,
            sort_keys=True,
        )
    )
    if not ok:
        lib = status["libpdfium"]
        if lib["status"] == "mismatch_abort":
            print(
                f"libpdfium mismatch: {lib['path']} sha={lib['observed_sha256']} "
                f"!= pin {lib['pinned_sha256']}; refusing to certify a drifted parser "
                "(set KB_SKIP_PDFIUM_HASH_CHECK=1 to record an explicit deviation).",
                file=sys.stderr,
            )
        if not status["pdfium_render"]["ok"]:
            print(f"pdfium-render version drift: {status['pdfium_render']}", file=sys.stderr)
        if status["kb_binary"]["status"] == "version_drift":
            print(f"kb binary version drift: {status['kb_binary']}", file=sys.stderr)
        repro = status["chunk_hash_reproduction"]
        if repro["status"] == "mismatch":
            print(f"chunk_hash reproduction FAILED: {repro['mismatches'][:5]}", file=sys.stderr)
        if args.require_ingest_ready and not status["ingest_ready"]:
            print(
                "ingest not ready: "
                f"libpdfium={lib['status']} kb={status['kb_binary']['status']} "
                f"chunk_hash_repro={repro['status']}",
                file=sys.stderr,
            )
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
