# Python Retirement Allowlist

After the `legacy_python_oracle/` deletion (AC-15), the following
references to Python, the quarantine, or Python tooling are
**intentionally retained** as historical provenance, documentation,
or archival tooling:

## Provenance doc-comments (~100 references across ~45 Rust source files)

Lines like `// Ported from legacy_python_oracle/src/cacg/...` in
`crates/cacg-core/src/`, `crates/cacg-cli/src/`, and
`crates/cacg-semantic/src/`. These document the origin of ported
Rust code and carry no execution dependency.

## Documentation references (~5 files in docs/)

- `docs/python-oracle-relocation.md` — source-map for the quarantine
- `docs/diagnostic-parity.md` — references Python error codes
- `docs/pdf-fixtures-rebuild.md` — rebuild ceremony for PDF fixtures
- `docs/stress-10k.md`, `docs/retrieval.md`, `docs/lint-codes.md`

## Archival Python tooling (tools/python_legacy/)

- `tools/python_legacy/pdf/` — 3 PDF fixture generators + pyproject.toml
- `tools/python_legacy/migration/` — 4 CFA migration scripts

These survive AC-15 deletion in their non-quarantine archival location.
They are not part of CI and require manual Python setup to run.

## Test file provenance comments

Doc-comment headers in `crates/*/tests/*.rs` citing the Python
original they were ported from. No execution dependency.

## Committed parity fixtures

`tests/parity_corpus/out_python/` contains the frozen Python CLI
output that the parity matrix compares Rust output against. These
files are data fixtures, not executable code.

## _research/ documents

Historical research documents referencing the Python tree, the
quarantine, and the retirement plan. These are project records.
