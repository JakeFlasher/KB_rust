# Diagnostic Compatibility Parity Contract

**Status:** stable.
**Date:** 2026-06-01.

This document pins the diagnostic compatibility surface for `kb lint`,
`kb verify`, `kb index`, `kb search`, and `kb show`.

The live implementation is Rust. Compatibility is enforced against frozen
committed fixture bytes under `tests/parity_corpus/out_python/`; no live Python
tree or Python generator is required.

## Diagnostic Shape

Diagnostics serialize as canonical JSON with these fields:

```rust
pub struct Diagnostic {
    pub code: String,
    pub severity: Severity,
    pub message: String,
    pub file: Option<String>,
    pub hints: Vec<DiagnosticHint>,
}

pub enum Severity {
    Error,
    Warning,
}
```

The byte-stable fields are:

- `code`
- `severity`
- `file`

The snapshot-covered fields are:

- `message`
- `hints`

`message` and `hints` must either match the committed fixture bytes or be
covered by an explicit divergence entry below.

## Intentional Divergences

The whitelist is empty.

Any new divergence must include the diagnostic code or fixture scope, the old
fixture text, the new Rust text, the reason, and the release round that added
it. PRs that change diagnostic output without updating this table must fail the
parity gate.

| Entry # | Scope | Fixture text | Rust text | Justification | Added round |
|---------|-------|--------------|-----------|---------------|-------------|

## 3a. AC-2.1: Lint-pass surface auxiliary-codes carve-out

The bounded carve-out from the earlier hot-path port remains in force only for
these Rust functions:

- `cacg-core::lint::layer1::run_layer1_checks`
- `cacg-core::lint::layer1::lint_card`
- `cacg-core::lint::layer1::lint_directory`

Within that bounded surface, these prefixes can be demoted by a hot-path-scoped
parity corpus:

- `CACG-SUM-*`
- `CACG-SKILL-*`
- `CACG-DEP-*`
- `CACG-ROLE-*`

No other diagnostic prefix is demotable. The policy tests in
`xtask/src/parity.rs` keep this carve-out from broadening into schema,
frontmatter, index, search, show, or verify surfaces.

The carve-out does NOT apply to M1 schema/frontmatter parity surfaces:

- `crates/cacg-core/src/schema.rs`
- `crates/cacg-core/src/frontmatter.rs`
- `crates/cacg-core/tests/schema_parity.rs`

A hot-path-scoped corpus declares the annotation literal
`cacg.v0/scope:hot-path` in a `scope.json` sidecar at the corpus root.

## 4. How parity is enforced

Run the committed-fixture parity gate:

```bash
cargo run -p xtask -- parity --corpus tests/parity_corpus/
```

The gate compares Rust CLI output against committed fixture artifacts and fails
on missing artifacts, command failures, byte differences in gating rows, or an
unauthorized demotion.

The merge-blocking workflow is `.github/workflows/parity.yml` with job name
`Committed-fixture byte-equal parity`.

## References

- `crates/cacg-core/src/diagnostic.rs`
- `docs/lint-codes.md`
- `_research/09_dec_proposed_defaults.md`
- `tests/parity_corpus/`
- `xtask/src/parity.rs`
