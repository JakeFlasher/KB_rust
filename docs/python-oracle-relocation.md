# Python Oracle Relocation

Frozen oracle. Read-only. Not the production CLI. Reference for Rust parity gates. Removed at the terminal sub-milestone after each live dependency is severed.

## Summary

The legacy Python tree that previously implemented the byte-equal parity oracle (`src/cacg/`), the fixture/oracle generation toolchain (`scripts/*.py`), and the live test suite (`tests/test_*.py`, `tests/perf/`, `tests/conftest.py`, `tests/fixtures/build_sample_pdf.py`) has been relocated under the namespace `legacy_python_oracle/`. The relocation also moved the Python build metadata: `uv.lock` and `pyrightconfig.json` were moved into the quarantine; the workspace-root `pyproject.toml` was deleted, and a redesigned package manifest now lives at `legacy_python_oracle/pyproject.toml` with quarantine-relative paths (`where = ["src"]`, `testpaths = ["tests"]`, `include = ["src"]`).

The relocation is behavior-preserving: the committed `out/semantic_cache.json`, `out/semantic_cache.provenance.json`, every fixture under `tests/parity_corpus/`, and every M5b evidence row remain byte-identical. Rust-owned fixture directories (`tests/parity_corpus/`, `tests/golden/`, `tests/adversarial/`, `tests/round_summary_fixtures/`, `tests/retrieval_eval/`, `tests/semantic_eval/`, `tests/fixtures/*.pdf`) stay in place. Two generated Rust source files (`crates/cacg-core/src/casefold_table.rs`, `crates/cacg-cli/src/nonprintable_table.rs`) also stay in place; their generator-banner headers now point at the relocated generators.

## Full mapping

The complete per-file old-to-new path mapping plus a comprehensive inventory of every reference site (Rust source, CI workflow, doc, research document) that named a relocated Python path lives at:

- [`legacy_python_oracle/MOVED.md`](../legacy_python_oracle/MOVED.md)

The per-Python-tooling-file disposition manifest (35 entries: 34 `scripts/*.py` + 1 `tests/fixtures/build_sample_pdf.py`) categorized by purpose and tagged with terminal disposition (RUST-PORT / FREEZE-WITH-SHA-PIN / MOVE-TO-TOOLS-PYTHON-LEGACY / DELETE-AT-AC-15) lives at:

- [`legacy_python_oracle/DISPOSITION.md`](../legacy_python_oracle/DISPOSITION.md)

## Why the quarantine

The relocation severs the load-bearing Python dependency graph in sequenced sub-milestones while keeping the Python tree available as a byte-equal oracle through the intermediate sub-milestones. Final deletion of `legacy_python_oracle/` happens only after every live Rust runtime, test, xtask command, and CI workflow is independent of it AND a final execution-token sweep proves zero leaks (`python`, `.py`, `pytest`, `ruff`, `pyright`, `pip`, `uv`, `PYTHONPATH`, `cacg.cli`, shebang lines).

## Operator notes

- Installing the legacy Python package (e.g., for the `kb` CLI used by `scripts/demo.sh`):
  - `uv pip install -e './legacy_python_oracle[dev]'`
  - or `python3 -m venv legacy_python_oracle/.venv && legacy_python_oracle/.venv/bin/pip install -e ./legacy_python_oracle[dev]`
- Running the legacy pytest suite: `(cd legacy_python_oracle && .venv/bin/pytest tests)`
- The Rust `cargo xtask parity` resolver expects the venv at `legacy_python_oracle/.venv/bin/python`; if missing, it fails loudly (no silent fallback to system `python3`).
