# Pdfium Binary Provisioning

**Status:** documented runtime-bind mechanism. Pinned, CI-reproducible
binary provisioning is a future deliverable; the binary-SHA pin
itself is forced by the ingest parity checkpoint when it lands.

## Current Mechanism

`cacg-ingest` (behind the default-off `ingest` Cargo feature) binds
to the Pdfium shared library at runtime via
`pdfium_render::prelude::Pdfium::bind_to_system_library()`.

- Workspace pin: `pdfium-render = "0.9.1"` (root `Cargo.toml`,
  `[workspace.dependencies]`).
- Runtime binding: dynamic. Pdfium itself is not statically linked
  into the cacg-ingest binary; `cacg-ingest` is the only workspace
  crate permitted to perform this FFI call (the audit in
  `cargo xtask audit-cacg-core-deps` enforces this).
- Discovery: `bind_to_system_library` reads the standard dynamic
  loader paths (`LD_LIBRARY_PATH`, `/usr/lib`, `/usr/local/lib`,
  `/etc/ld.so.conf.d/*`) on Linux. The platform-equivalent mechanisms
  apply on macOS and Windows.

If Pdfium is not discoverable, `bind_to_system_library` returns
`PdfiumError::LoadLibraryError(...)`. `cacg-ingest` maps that to
`IngestError::Corrupt { detail: "pdfium bind failed: ..." }` with the
`CACG-INGEST-001` diagnostic — so a missing native dependency surfaces
as a structured diagnostic, not a panic.

Tests that exercise the live binding
(`crates/cacg-ingest/tests/extract_pages_pdfium.rs`) probe for
availability and skip with a console notice when Pdfium is absent, so
the suite stays green on runners without the dependency installed.

**Developer-machine note:** the Python parity oracle's `pypdfium2`
package ships its own `libpdfium.so` under
`.venv/lib/python*/site-packages/pypdfium2_raw/`. On a developer
machine where the project's `.venv` has been activated or where
`pypdfium2` has been installed system-wide, `pdfium-render`'s
`bind_to_system_library` may discover that library even without
explicit operator setup. This is convenient for local development but
is **not** a substitute for a pinned, recorded Pdfium binary in CI:
`pypdfium2`'s bundled Pdfium build can change with every `pypdfium2`
release, which is exactly the cross-machine non-determinism the
forthcoming binary pin is meant to eliminate.

## Operator Setup (Linux)

1. Obtain a Pdfium build for your platform. The community-maintained
   `bblanchon/pdfium-binaries` GitHub releases are the conventional
   source — they package upstream Chromium-source Pdfium for
   Linux / macOS / Windows.
2. Extract `libpdfium.so` from the release archive.
3. Place it where the dynamic loader will find it. The simplest
   sustained setup:

   ```
   sudo install -m 0644 libpdfium.so /usr/local/lib/libpdfium.so
   sudo ldconfig
   ```

4. Verify with `ldconfig -p | grep pdfium` (Linux) or the platform
   equivalent.
5. Re-run `cargo test -p cacg-ingest --features ingest`. The
   `extract_pages_pdfium.rs` fixture tests should now execute against
   the live binding instead of skipping.

## Strict Mode (CI Gate)

By default the cacg-ingest integration tests
(`crates/cacg-ingest/tests/extract_pages_pdfium.rs`) skip with a
console notice when `libpdfium` is unavailable, which keeps local
developer runs green on machines that have not yet completed the
Operator Setup. The drawback is that a CI runner without `libpdfium`
also silently skips — the Pdfium-bound negative-test path would
green-light without exercising a single assertion.

To refuse that silent green-light, set the `CACG_REQUIRE_PDFIUM`
environment variable to `1` before running the test suite:

```bash
CACG_REQUIRE_PDFIUM=1 cargo test -p cacg-ingest --features ingest
```

With the variable set, `skip_if_pdfium_unavailable` panics with a
message pointing operators back to this document instead of returning
silently. CI workflows that have completed the Operator Setup above
should set this variable so a provisioning regression surfaces as a
test failure rather than as an absence of test execution. The
workspace ships no CI step that sets the variable today; that is the
companion deliverable to the CI provisioning step described below.

## CI Gap

The current mechanism requires `libpdfium` to be pre-installed on
every CI runner. The workspace ships no automated fetch step today.
Closing this gap requires a deliberate provisioning deliverable, of
which the two viable shapes are:

- A `build.rs` in `cacg-ingest` (gated on the `ingest` feature) that
  fetches a pinned Pdfium binary, verifies its SHA-256, and stages it
  on the build directory's lib path; or
- A CI workflow step (e.g. in `.github/workflows/parity.yml`) that
  performs the operator setup above against a recorded Pdfium build
  SHA before tests run.

Either path requires pinning the exact Pdfium build SHA. That pin
becomes load-bearing the moment the ingest path is used to produce
hash-pinned chunks intended to be reproducible across machines:
different Pdfium builds can emit byte-different extracted text and
therefore different `chunk_hash` values.

## Pinned Versions

| Component | Pin | Source |
| --- | --- | --- |
| `pdfium-render` (Rust binding) | `0.9.1` | workspace `Cargo.toml` |
| Pdfium native binary | `149.0.7825.0` (`pypdfium2` 5.8.0 bundle) | AC-5 BYTE-EQUAL outcome, resolved Round 9 against `tests/parity_corpus/pdfs/cfa_vol1_trim.pdf`; the committed Python oracle was produced with this build, and Rust `kb ingest` reproduces it byte-for-byte (`crates/cacg-cli/tests/kb_ingest_parity.rs`) |
| `libpdfium.so` SHA-256 | `fcd602cd518476d712f661b08e010700490875288fb17069b5b5a2f8b7724118` | recorded Round 9 against the `pypdfium2 5.8.0` wheel's bundled `pypdfium2_raw/libpdfium.so`; verify with `sha256sum <pypdfium2-install>/pypdfium2_raw/libpdfium.so` |

The Pdfium binary pin is the build the committed Python oracle was
produced with — `pdfium 149.0.7825.0`, shipped inside the
`pypdfium2 5.8.0` wheel under
`<pypdfium2-install>/pypdfium2_raw/libpdfium.so`. Cross-machine
reproducibility of `chunks_manifest.json` content requires every CI
runner to load this exact build. Two practical paths:

1. Install `pypdfium2==5.8.0` in a CI venv and add the wheel's
   `pypdfium2_raw/` directory to `LD_LIBRARY_PATH` before running
   the parity test. This is what the developer-machine setup
   already does implicitly when the project's `.venv` is active.
2. Extract `libpdfium.so` from the `pypdfium2 5.8.0` wheel once
   into a stable path (e.g. `/usr/local/lib/libpdfium.so`) and
   `ldconfig`.

Until a `build.rs` or a CI workflow step pins the binary
fetch + SHA-256 verify automatically, the pin above is operationally
load-bearing: any drift in the bundled Pdfium build will be caught
by the AC-5 byte-equal parity test, surfacing as a failed
`kb_ingest_cfa_vol1_trim_is_byte_equal_with_committed_python_oracle`
assertion rather than silent corpus divergence.

### AC-5 declared text-extraction divergence

Pdfium's `FPDFText_GetText` (the API pypdfium2 calls under the hood)
post-processes the soft-hyphen marker `U+0002` (STX) to the
non-character `U+FFFE`. Pdfium's per-character accessor
`FPDFText_GetUnicode` (the API the safe Rust wrapper
`pdfium-render::PdfPageText::chars().iter()` calls) returns the raw
`U+0002`. The Rust `extract_pages_impl` therefore performs an
explicit `U+0002 → U+FFFE` mapping on each character to match
Python's `pypdfium2` output byte-for-byte. STX never appears in
legitimate text (it is a C0 control byte), so the mapping is safe
on every PDF in the corpus. AC-5 caught this in Round 9; 36
occurrences were observed across 23 of the 34 chunks of
`cfa_vol1_trim.pdf`. The mapping is locked in
`crates/cacg-ingest/src/pdf.rs` and exercised end-to-end by the
parity test.

## Related

- `crates/cacg-ingest/src/pdf.rs` — runtime binding call site +
  per-page panic boundary.
- `crates/cacg-ingest/src/lib.rs` — `IngestError::Corrupt` mapping.
- `crates/cacg-ingest/tests/extract_pages_pdfium.rs` — fixture tests
  that skip cleanly when Pdfium is unavailable.
- `_research/09_dec_proposed_defaults.md` — DEC-2 (Pdfium output
  expectation: BYTE-EQUAL vs HASH-STABLE).
- `_research/07_rust_refactor_research.md` — pdfium-render crate
  selection rationale.
