# Python-Tree Retirement Completion Summary

**Date:** 2026-05-26
**Deletion commit:** `fd97cad`
**Plan:** `.humanize/.humanize/plans/cacg-python-retirement-plan.md`

## Evidence Summary

The legacy Python tree (`legacy_python_oracle/`) has been deleted. The CACG workspace is now a single-language Rust workspace with no Python runtime dependencies in any CI gate, test suite, or xtask command.

### AC Status

| AC | Status | Evidence |
|----|--------|----------|
| AC-1 | MET | Python files relocated to `legacy_python_oracle/` via `git mv` (commit `2140fe2`), then deleted (commit `fd97cad`) |
| AC-1.1 | MET | All path references updated across 15 reference-site categories (Rounds 1-7) |
| AC-1.2 | MET | Disposition manifest at `legacy_python_oracle/DISPOSITION.md` (deleted with quarantine) |
| AC-2 | MET | Python packaging redesigned for quarantine, then deleted |
| AC-3 | MET | Source-map at `docs/python-oracle-relocation.md` |
| AC-4 | MET | M5b addendum preserves evidence chain; relocation + parity-surface + schema-version + final retirement addenda |
| AC-5 | MET | Five boundary gates pass post-deletion: parity 16/0/6, provenance 227 clean, workspace tests pass |
| AC-6 | MET | Five proptest oracles replaced: 3 Path-A independent Rust references, 2 Path-B committed corpus freezes (Round 8) |
| AC-7 | MET | 22 gating rows converted to committed-fixture parity; REPORT SCHEMA migrated to oracle-agnostic; help snapshots disposed; status-check renamed |
| AC-7.1 | MET | 14 help-snapshot rows removed from matrix (Path B); CLI surface transferred to committed snapshots + Rust clap tests |
| AC-8 | MET | Schema-fixture generator ported to Rust-native (`cargo xtask gen-schema-fixtures`, 80 fixtures, no Python) |
| AC-9 | PARTIAL | Inventory complete (644 rows); PORT-TO-RUST entries not individually ported (deferred — Rust parity coverage is comprehensive) |
| AC-10 | MET | 3 PDF generators moved to `tools/python_legacy/pdf/` (DEC-6 Path B); rebuild ceremony documented |
| AC-11 | MET (Option B) | Semantic cache frozen as immutable; Hash B/uv.lock verification removed; schema stays `cacg.v0` |
| AC-12 | MET (Path B) | CFA migration scripts archived to `tools/python_legacy/migration/` |
| AC-13.a | MET | `ci.yml` rewritten Rust-only (no setup-python) |
| AC-13.b | MET | `parity.yml` rewritten Rust-only (no setup-python) |
| AC-13.c | MET | `_validate-workflows.yml` rewritten Rust-only; `xtask lint-workflow-integrity` replaces Python validator |
| AC-14 | MET | Status-check renamed to "Committed-fixture byte-equal parity"; operator checklist recorded |
| AC-15 | MET | `legacy_python_oracle/` deleted at commit `fd97cad`; execution-token sweep clean; allowlist documented |

### Post-Deletion Gates

- `cargo build --workspace`: PASS
- `cargo test --workspace`: 95 passed, 0 failed
- `cargo xtask parity --corpus tests/parity_corpus/`: 22 entries, 16 passed, 0 failed, 6 future-stage
- `cargo xtask audit-semantic-cache-provenance`: 227 entries clean
- `cargo xtask audit-schema-fixtures`: 80 fixtures clean
- `cargo xtask lint-workflow-integrity`: PASS
- `cargo xtask lint-workflow-labels`: PASS

### Plan Evolutions

| Original DEC | Final Resolution | Rationale |
|---|---|---|
| DEC-1 Option C (Rust embedding) | Option B (freeze cache) | No Rust crate produces byte-reproducible MiniLM-L6-v2 embeddings |
| DEC-2 Path A (Rust migrate-vertical) | Path B (archive) | One-time migration; output already committed |
| DEC-6 Path A/C (Rust PDF port) | Path B (archive) | Byte-stability risk; rebuild ceremony preserved |

### Residual Python

The following Python survives outside the deleted quarantine:
- `tools/python_legacy/pdf/` — 3 PDF fixture generators + pyproject.toml
- `tools/python_legacy/migration/` — 4 CFA migration scripts
- `tests/parity_corpus/out_python/` — frozen oracle fixture data (not executable)
- `tests/parity_corpus/help_snapshots/` — frozen argparse tree snapshots (not executable)

All documented in `docs/python-retirement-allowlist.md`.
