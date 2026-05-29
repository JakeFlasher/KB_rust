# CACG: Content-Addressable Card Graph

A minimal-working framework for authoring small Markdown knowledge cards from PDF sources with **mechanical, hash-pinned content verification** under 100ms per card on the common path.

> **Legacy Python tree relocated.** The Python implementation (`src/cacg/`), oracle/fixture scripts (`scripts/*.py`), and Python test suite (`tests/test_*.py`, `tests/perf/`, `tests/conftest.py`, `tests/fixtures/build_sample_pdf.py`), plus the Python build metadata (`uv.lock`, `pyrightconfig.json`, the original root `pyproject.toml`), have been quarantined under `legacy_python_oracle/`. The legacy CLI lives at `legacy_python_oracle/.venv/bin/kb` after `make install`. The Rust workspace (`crates/`, `xtask/`) is the production target. See [`docs/python-oracle-relocation.md`](docs/python-oracle-relocation.md) for the full old-to-new path mapping and disposition manifest.

## Status

MVP. The `kb` CLI ships every subcommand the implementation plan named (`ingest`, `new`, `lint`, `verify`, `index`, `history`, plus `verify --round-summary` for humanize RLCR integration). End-to-end demo (`make demo`) runs under one second on a laptop; the perf test enforces median lint+verify under 100ms per card with no PDF parsing on the common path.

## Quick Start

```bash
make install                # creates .venv and installs cacg + dev deps
legacy_python_oracle/.venv/bin/python legacy_python_oracle/tests/fixtures/build_sample_pdf.py    # builds the fixture PDF

# Ingest a PDF into a hash-pinned chunk manifest
legacy_python_oracle/.venv/bin/kb ingest tests/fixtures/sample.pdf

# Author a card from the canonical template
legacy_python_oracle/.venv/bin/kb new reading_01 sample-card

# Bootstrap a permissive source_matrix from the just-ingested corpus.
# --source-matrix is MANDATORY on kb lint / kb verify / kb search.
legacy_python_oracle/.venv/bin/kb index cards
legacy_python_oracle/.venv/bin/kb scaffold-matrix \
  --cards-manifest out/cards_manifest.json \
  --chunks-manifest out/chunks_manifest.json \
  --out out/source_matrix.json

# Layer-1 lint (citation structure + card_hash freshness) and Layer-2 verify
# (normalized exact-substring containment against the pinned chunk)
legacy_python_oracle/.venv/bin/kb lint cards/reading_01/sample-card.md \
  --chunks-manifest out/chunks_manifest.json \
  --source-matrix  out/source_matrix.json
legacy_python_oracle/.venv/bin/kb verify cards/reading_01/sample-card.md \
  --chunks-manifest out/chunks_manifest.json \
  --source-matrix  out/source_matrix.json

# Integrate with humanize: verify every card path in a round summary's
# "## Knowledge Consulted" section
legacy_python_oracle/.venv/bin/kb verify --round-summary path/to/round-summary.md \
  --chunks-manifest out/chunks_manifest.json \
  --source-matrix  out/source_matrix.json

# End-to-end demo with deterministic artifacts
make demo
```

`KB_FROZEN_CLOCK=1` collapses timestamps and UUIDs in journals and history files so two identical runs produce byte-identical output. See `docs/schema.md` for the full schema and `docs/lint-codes.md` for every CACG-* diagnostic.

## Three Concentric Loops

```
+------------------------------------------------------------+
|  L3: AUTHORING LOOP (humanize RLCR orchestrated)           |
|     gen-idea -> gen-plan -> start-rlcr-loop                |
|     Per-round summary lists Knowledge Consulted paths      |
+------------------------------------------------------------+
|  L2: CARD-LIFECYCLE LOOP (cacg)                            |
|     kb new -> author -> kb lint -> kb verify -> kb index   |
|     Hash-pinned chunks; append-only tamper-evident history |
+------------------------------------------------------------+
|  L1: VERIFICATION ENGINE (mechanical, deterministic)       |
|     Layer 1: regex/format/structure  (microseconds)        |
|     Layer 2: normalized exact substring  (milliseconds)    |
|     Optional: rank-bm25 "did you mean" hints (diagnostic)  |
+------------------------------------------------------------+
```

## Design Principles

- **Hash-pinning over retrieval.** Cards cite `chunk_id + chunk_hash`; any source or chunk drift mechanically stales the citation (`CACG-HASH-001`). Card-body edits without a `kb index` rerun surface as `CACG-HASH-002`.
- **Normalized exact-substring containment is the verification oracle**, not BM25. BM25 only emits diagnostic "did you mean" hints with `hint_only=true`. `--fuzzy` allows Levenshtein-bounded escape for OCR-grade drift but is opt-in.
- **Atomic, tamper-evident publish.** Manifest writes go through tempfile + Pydantic round-trip validation + `os.replace`. `kb index` stages card updates first, then pair-publishes `cards_manifest.json` + `INDEX.md`, then commits the staged card files. Lint journals and per-card history files use a `prev_checksum` + `event_checksum` chain so same-`seq` rewrites are detectable.
- **Deterministic gates.** Same inputs produce identical outputs. `KB_FROZEN_CLOCK=1` collapses timestamps and UUIDs to zero values for reproducible test fixtures.
- **No PDF parsing on the common path.** `kb lint` and `kb verify` read only manifests and the card itself; `pypdfium2` is invoked exclusively from `kb ingest`. The perf test installs an `__import__` sentinel to enforce this.

## Read-Only References

This framework is prototyped alongside (but never modifies) two read-only sibling repositories:

- `../CFA_reading/` — mature 219-card CFA-curriculum knowledge base; source of the atomic-publish + retraction-log patterns CACG extends.
- `../humanize/` — Claude Code plugin implementing RLCR; CACG integrates via `kb verify --round-summary`.

See `docs/integration-with-humanize.md` for the contract and exit-code matrix.

## Quality Gate

`cargo xtask gate` runs the unified quality gate: formatting, clippy, supply-chain policy, 14+ static-grep lints, and the full test suite. Use `--report` for JSON output.

Key enforcement:
- `clippy::unwrap_used = "deny"` in production crates (test code exempt)
- `lint-unwrap` / `lint-error-swallow` xtask lints for secondary coverage
- `lint-structural` for module size, function length, and nesting depth (blocking)
- `cargo deny check` for supply-chain policy
- All checks blocking in `ci.yml`

See [`docs/rust-quality-gate.md`](docs/rust-quality-gate.md) for the full adapted policy.

## Layout

```
legacy_python_oracle/src/cacg/                 Python package
legacy_python_oracle/src/cacg/lint/            Layer-1 mechanical lint
legacy_python_oracle/src/cacg/verify/          Layer-2 content verification + fuzzy + BM25 hints
legacy_python_oracle/src/cacg/integrate/       humanize round-summary integration
tests/                    pytest suites (unit, golden, adversarial, perf)
tests/fixtures/           deterministic fixture PDF + golden cards
tests/golden/             2 golden cards (pass lint, verify, index)
tests/adversarial/        7 adversarial cards (one per documented failure mode)
legacy_python_oracle/tests/perf/               common-path latency + PDF-import sentinel
scripts/demo.sh           end-to-end demo (under 30s wall-clock)
.humanize/plans/          implementation plans (local, gitignored)
.humanize/drafts/         plan drafts / research briefs (local, gitignored)
docs/schema.md            Pydantic schemas + evolution policy
docs/lint-codes.md        Every CACG-* code with positive/negative examples
docs/integration-with-humanize.md  Round-summary contract
docs/analyses/            T30 + T37 Codex analyses
```

## License

MIT.
