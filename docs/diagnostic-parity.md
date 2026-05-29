# Diagnostic Compatibility Parity Contract

**Status:** stable.
**Date:** 2026-05-20.

This document pins the cross-implementation compatibility surface for `Diagnostic` objects emitted by `kb lint` / `kb verify` / `kb index` / `kb search` / `kb show`. The parity contract is the HYBRID policy: structural fields are always byte-equal between Python and Rust; message text is governed by a per-diagnostic snapshot suite with an explicit whitelist of intentional divergences.

---

## 1. Diagnostic Shape

Both implementations emit a `Diagnostic` object with this schema:

```python
class Diagnostic:
    code: str          # CACG-XXX-NNN (canonical code constant)
    severity: str      # "error" | "warning"
    message: str       # Human-readable explanation
    file: Optional[str]  # Repo-relative path or None
    hints: List[Hint]  # Ordered list of structured hint dictionaries
```

```rust
pub struct Diagnostic {
    pub code: String,
    pub severity: Severity,
    pub message: String,
    pub file: Option<String>,
    pub hints: Vec<DiagnosticHint>,
}

pub enum Severity { Error, Warning }
```

Both implementations canonical-JSON-serialize diagnostics via the same byte-equal writer (canonical-JSON acceptance criterion). The Rust struct's `Serialize` impl mirrors Python `Diagnostic.to_dict()` so the serialized JSON shape is identical.

---

## 2. HYBRID Parity Contract

Per Claude's Round-0 PROPOSED-DEFAULT (DEC-3 in `_research/09_dec_proposed_defaults.md`), the parity contract is HYBRID:

### 2.1 Always byte-equal (no exceptions)

These fields MUST byte-match between Python and Rust on every Diagnostic emitted:

- `code` — the CACG diagnostic code constant (e.g., `"CACG-CITE-002"`). Pinned by the constants module at `legacy_python_oracle/src/cacg/lint/codes.py` (Python) and `crates/cacg-core/src/diagnostic.rs` (Rust).
- `severity` — `"error"` or `"warning"`. Case-sensitive.
- `file` — repo-relative path string or `null`. When non-null, paths use forward slashes (POSIX style) on both implementations.

A regression that re-orders these fields' bytes or changes their casing fails the cross-impl parity gate.

### 2.2 Snapshot-tested via the corpus

These fields are NOT byte-equal by default; they are covered by the fixture-based snapshot suite at `tests/parity_corpus/{adversarial,generated_pydantic_errors}/out_python/`:

- `message` — the human-readable error explanation. Each fixture's `out_python/<fixture>.lint.json` carries the exact Python `message` text; the Rust implementation MUST emit either (a) byte-equal `message` text OR (b) a `message` text covered by the intentional-divergences whitelist (§3 below).
- `hints` — the ordered list of structured hint dictionaries. The hint ordering MUST be byte-equal (Python and Rust use the same tiebreak rules). The hint dictionary contents must be byte-equal modulo the whitelist.

The snapshot suite is regenerated when the Python implementation's `message` text intentionally changes (e.g., refactoring a diagnostic for clarity); the new bytes are committed alongside the Python source change. Rust must match the new bytes within one round.

### 2.3 Excluded from byte-equal parity

These fields are NEVER byte-equal between implementations and are deliberately excluded from the parity gate:

- Stack traces (Python tracebacks vs Rust `Debug` impls). Never serialized into the canonical Diagnostic.
- Implementation-specific timing / memory counters. Never serialized into the canonical Diagnostic.

---

## 3. Intentional Divergences Whitelist

The whitelist is INITIALLY EMPTY. Every entry requires:

- The full Python `message` text.
- The full Rust `message` text.
- A diagnostic code OR a fixture-name scope.
- A justification (Python bug being fixed, message-text clarity improvement, performance reason, etc.).
- The round number the entry was added.

Entries are added via a PR that updates BOTH the source-of-truth code AND this table in the same commit. PRs that change `message` text without a corresponding whitelist entry fail the snapshot suite at CI time.

| Entry # | Diagnostic code or fixture scope | Python `message` | Rust `message` | Justification | Added round |
|---------|-----------------------------------|------------------|----------------|---------------|-------------|

_The whitelist is currently empty._ One entry — `CACG-SHOW-003`, the
`kb show --path` traversal / absolute-path rejection — was added as a
declared Rust-side divergence and **removed** once the post-phase
carry-forward patched Python `_cmd_show` to reject the same inputs (a
`..` path component or an absolute path) with the byte-equal
`CACG-SHOW-003` diagnostic. Python and Rust `kb show --path` are now
byte-equal, so no divergence remains.

---

## 3a. AC-2.1: Lint-pass surface auxiliary-codes carve-out

The M3 verify-hot-path plan's AC-2.1 introduces a SCOPED carve-out from the
HYBRID parity contract: certain auxiliary diagnostic-code prefixes are
deferred from the byte-equal gate, but ONLY when emitted from the new
trust-bearing Rust lint-pass functions. Every other Rust surface continues
to honor the §2 HYBRID contract in full.

### 3a.1 Bounded surface

The carve-out applies to exactly these three Rust functions:

- `cacg-core::lint::layer1::run_layer1_checks` (the pure lint pass).
- `cacg-core::lint::layer1::lint_card` (the journal-writing per-card wrapper).
- `cacg-core::lint::layer1::lint_directory` (the batch walker).

It does NOT apply to any other Rust surface. In particular, the existing M1
schema and frontmatter parity surfaces — `crates/cacg-core/src/schema.rs`,
`crates/cacg-core/src/frontmatter.rs`, and their parity tests at
`crates/cacg-core/tests/schema_parity.rs` — continue to emit `CACG-SUM-*`
diagnostics unchanged and continue to be held to the full HYBRID byte-equal
contract on every code (no carve-out, no annotation, no demotion). A
documentation change that broadens the carve-out outside the three named
lint-pass functions is a policy violation; the `xtask` policy tests at
`xtask/src/parity.rs` (specifically
`ac21_doc_carveout_does_not_broaden_to_m1_surfaces`) fail the build on such
a broadening.

### 3a.2 Deferred code prefixes

From the bounded surface above only, these prefix wildcards are deferred
from the byte-equal gate:

- `CACG-SUM-*` — summary length / tag-slug aggregate diagnostics.
- `CACG-SKILL-*` — `SKILL.md` router validation diagnostics.
- `CACG-DEP-*` — card-edge DAG validation diagnostics.
- `CACG-ROLE-*` — per-reading role-map validation diagnostics.

Python's `lint_directory` emits these via the auxiliary aggregators
`validate_skill_routers`, `validate_role_maps`, and `validate_card_dag`.
The Rust trust-bearing port intentionally omits those aggregators per the
M3 phase's lower bound (`.humanize/plans/cacg-rust-port-m3-verify-hot-path-plan.md`
lines 150-156); their inclusion in the Rust lint-pass surface is M3
authoring-tail follow-on work, NOT a Round 7 deliverable.

### 3a.3 Corpus annotation: `cacg.v0/scope:hot-path`

A parity corpus that exercises only the trust-bearing lint-pass surface
declares the scope via a sidecar JSON file at the corpus root:

```
<corpus_dir>/scope.json
```

Content:

```json
{"schema_version": "cacg.v0", "scope": "hot-path"}
```

When present, the parity harness (`xtask::parity::matrix::run_kb_lint_entry`)
recognizes the corpus as hot-path-scoped and applies the demotion rule
described in §3a.4. When absent (the default for the committed `valid/` and
`adversarial/` corpora as of Round 7), the harness applies the full
byte-equal HYBRID contract with no demotion.

### 3a.4 Harness demotion rule

For `kb_lint_parity_*` rows (the only rows that consult the annotation):

- If the corpus has `scope.json` declaring `scope: "hot-path"`, AND a
  per-card artifact comparison (stdout, stderr, exit, or lint_journal)
  differs ONLY in lines that mention exclusively the deferred code
  prefixes above, the diff is treated as a `FutureStage("M3")` divergence
  rather than a `Fail`.
- If ANY divergent line mentions a code outside the deferred prefixes
  (e.g., `CACG-CITE-*`, `CACG-AUTH-*`, `CACG-MAN-*`, `CACG-HASH-*`,
  `CACG-RETR-*`, `CACG-JNL-*`, `CACG-CLI-*`, or any non-CACG noise), the
  full `Fail` reason stays and the row gates the build.
- The demotion never applies to `MatrixRowKind::KbIndex` or
  `MatrixRowKind::HelpSnapshot` rows; they ignore `scope.json`
  entirely. The policy test
  `ac21_hot_path_annotation_ignored_outside_lint_rows` pins this.

### 3a.5 When the carve-out closes

The carve-out CLOSES when the M3 authoring-tail follow-on phase ports
`validate_skill_routers`, `validate_role_maps`, and `validate_card_dag` to
Rust and re-aligns the lint-pass surface's emitted code set with Python's
`lint_directory`. At that point §3a is removed wholesale from this
document. Until that PR lands, the carve-out remains the source of truth
for the bounded surface and the deferred prefix list.

---

## 4. How parity is enforced

### 4.1 Cross-implementation parity gate

`legacy_python_oracle/scripts/validate_canonical_json_parity.py` (and, once Rust lands, `cargo xtask parity --module diagnostics`) runs the Python and Rust diagnostic emitters against every fixture in `tests/parity_corpus/{adversarial,generated_pydantic_errors}/` and asserts:

1. Every `(code, severity, file)` triple byte-matches between implementations.
2. Every `message` either byte-matches OR is covered by a whitelist entry.
3. Every `hints` array byte-matches modulo the whitelist.

Any failure fails the merge-blocking CI workflow at `.github/workflows/parity.yml`.

### 4.2 Whitelist hygiene

A periodic audit (every 5 rounds via the goal-tracker's Full Alignment Check) verifies:

- Every whitelist entry still applies (the cited Python bug still exists, the cited Rust message still emits, etc.).
- No diagnostic code is on the whitelist for more than 2 milestones without a resolution path.
- The whitelist is < 10 entries at any time; longer than that indicates a process problem, not a justified divergence.

---

## 5. References

- Python `Diagnostic` dataclass: `legacy_python_oracle/src/cacg/diagnostic.py` (48 LoC).
- CACG-* code constants: `legacy_python_oracle/src/cacg/lint/codes.py` (116 LoC).
- Lint-codes documentation: `docs/lint-codes.md` (every CACG-* code with positive/negative examples).
- DEC-3 PROPOSED-DEFAULT (HYBRID): `_research/09_dec_proposed_defaults.md` §DEC-3.
- Plan acceptance criterion: trust-kernel-first Rust port plan §M0 diagnostic-compatibility AC.
