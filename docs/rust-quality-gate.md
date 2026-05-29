# CACG Rust Quality Gate

Adapted quality gate for the Content-Addressable Card Graph Rust workspace, derived from jisilu-rs practices and adjusted for CACG's trust-kernel/parity constraints.

## Enforcement Mechanisms

### Unified Gate Command

`cargo xtask gate` runs all checks in sequence and exits non-zero on any failure. Use `--report` for JSON output with per-check timing.

### CI Workflow (`ci.yml`)

Blocking Linux CI runs separate observable steps:
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo deny check` (full check including advisories; the local gate runs only `bans licenses` — see note below)
- 12 of 14 xtask lint/audit subcommands (all except `lint-platform-cfg` and `lint-workflow-integrity`)
- `cargo run -p xtask -- lint-unwrap`
- `cargo run -p xtask -- lint-error-swallow`
- `cargo run -p xtask -- lint-structural`
- `KB_FROZEN_CLOCK=1 cargo test --workspace --all-targets`

Meta-validation (`_validate-workflows.yml`): `lint-workflow-integrity`, `lint-workflow-labels`, `lint-platform-cfg`.
`lint-workflow-labels` runs in both `ci.yml` and `_validate-workflows.yml`.
All 14 xtask checks are enforced across the two workflows.

> **cargo-deny advisory skip (local gate):** `cargo xtask gate` runs
> `cargo deny check bans licenses`, intentionally omitting the advisories
> category.  The advisory DB now contains CVSS 4.0 entries that
> cargo-deny 0.18.x cannot parse, so the local gate skips advisories to
> avoid false failures on developer machines whose toolchain may lag
> behind CI.  CI runs the full `cargo deny check` with a pinned
> cargo-deny version that handles these entries.

### Clippy Configuration

`clippy.toml` at workspace root:
- `cognitive-complexity-threshold = 25`
- `too-many-lines-threshold = 100`
- `excessive-nesting-threshold = 6`
- `type-complexity-threshold = 300`

Workspace lint config (`Cargo.toml`):
- `clippy::unwrap_used = "deny"` — no `.unwrap()` in production code
- `clippy::wildcard_enum_match_arm = "deny"` — exhaustive matches on internal enums; external types use `#[allow]`
- `clippy::cognitive_complexity = "warn"`
- `clippy::excessive_nesting = "warn"`

## Quality Policy Scope

| Scope | `.unwrap()` | `.expect()` | `.ok()` / `.unwrap_or_default()` |
|-------|-------------|-------------|----------------------------------|
| `crates/*/src/` (non-test) | DENIED | Invariant-only with justification | Audited; `// qg-allow:` if intentional |
| `#[cfg(test)]` + `tests/` | ALLOWED | ALLOWED | ALLOWED |
| `xtask/` | ALLOWED | ALLOWED | ALLOWED |
| Generated data (`casefold_table.rs`) | EXEMPT | EXEMPT | EXEMPT |

## Suppression Conventions

### Clippy Lints
```rust
#[allow(clippy::unwrap_used)]
// Invariant: <reason why this cannot fail>
let x = thing.unwrap();
```

### xtask Lints
```rust
let x = thing.unwrap(); // qg-allow: infallible-unwrap — OnceLock initialization
let x = result.ok(); // qg-allow: intentional-discard — optional preload
```

Suppressions are NOT allowed in trust-critical code paths (`hash.rs`, `canonical_json.rs`, `atomic_publish.rs`, `journal.rs`) without explicit review.

## Structural Limits

Enforced by `xtask lint-structural`:
- Function body: ≤ 300 lines
- Module production lines: ≤ 1,500 (excluding `#[cfg(test)]` and generated files)
- Brace nesting: ≤ 6 levels (clippy `excessive_nesting`)
  - Suppress with `// qg-allow: deep-nesting — <reason>` on the brace line or the first line inside the block (`rustfmt` moves trailing comments off brace lines like `} else {`)

Blocking (promoted from advisory after module decomposition completed in Round 0).

## No-Touch Zones

Modules with correctness constraints that preclude aesthetic refactoring:

- **`bm25.rs`** — Float accumulation order is load-bearing for Python parity
- **`canonical_json.rs`** — Must match Python `json.dumps(sort_keys=True)` exactly
- **`journal.rs`** — Append ordering and checksum chaining are trust-kernel invariants
- **`atomic_publish.rs`** — tmp/bak/replace discipline is the atomicity contract
- **Parity fixture bytes** — `tests/parity_corpus/out_python/` is read-only

## Performance Baselines

iai-callgrind benchmarks (instruction-count deterministic):
- `bm25_iai` — BM25 index construction + scoring
- `verify_iai` — `verify_one_card` on golden card
- `lint_iai` — `lint_card` on golden card
- `index_iai` — `build_index` on 2-card golden corpus

Regression threshold: >5% instruction count increase blocks the change.
