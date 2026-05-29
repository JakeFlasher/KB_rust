# M4 — Pdfium Ingest + QM Vertical Migration: Completion Summary

_captured against HEAD `2c40640` on `2026-05-24`_

This document closes **task-m4-12 / AC-11** and is the M4
exit artifact. AC-11 enumerates the cross-cutting crate-
boundary + determinism invariants that hold at the close of
both M4 sub-phases (M4a ingest core + M4b real-corpus
migration). This file records the verification of each
invariant and the end-state of the milestone.

## AC-11 invariant checklist

### Invariant 1 — `#![forbid(unsafe_code)]` on every crate except `cacg-ingest`

Verified by `grep "^#!\[forbid(unsafe_code)\]" crates/*/src/lib.rs`:

| crate         | declares forbid? | rationale |
|---------------|-----------------:|-----------|
| `cacg-core`     | yes              | trust kernel, no unsafe |
| `cacg-cli`      | yes (lib + main) | thin dispatcher, no unsafe |
| `cacg-search`   | yes              | FTS5 sidecar via rusqlite-safe API |
| `cacg-semantic` | yes              | scaffolded; no unsafe surface |
| `cacg-ingest`   | NO (carve-out)   | Pdfium C++ FFI — the only `unsafe` crate in the workspace, documented at `crates/cacg-ingest/src/lib.rs:8-10` |
| `xtask`         | yes              | parity harness + lint gates, no unsafe |

Carve-out is intentional and documented in-source. The
project's six static gates enforce that no Pdfium symbol or
binding ever leaks past the `cacg-ingest` boundary.

### Invariant 2 — no Pdfium edge into `cacg-core` or any common-path verb

`cargo xtask audit-cacg-core-deps`:

> `xtask audit-cacg-core-deps: 0 forbidden packages in cacg-core's resolved dep closure`

`cargo tree --edges normal` per crate (production deps, ignoring dev):

| crate         | pdfium edges (`--no-default-features`) | pdfium edges (default features) |
|---------------|---------------------------------------:|-------------------------------:|
| `cacg-core`     |                                     0 |                              0 |
| `cacg-cli`      |                                     0 |          1 (through `cacg-ingest` under `--features ingest`) |
| `cacg-search`   |                                     0 |                              0 |
| `cacg-semantic` |                                     0 |                              0 |
| `xtask`         |                                     0 |                              0 |
| `cacg-ingest`   |                                     0 |          1 (under `--features ingest`) |

`cacg-cli`'s pdfium edge under default features is the
documented `kb ingest` path through `cacg-ingest`. With
`--no-default-features` the cli builds without pdfium and
`kb ingest` falls back to the `unimplemented_subcommand`
stub. The lint-trust-leak gate confirms `cacg-cli`'s source
never reaches around `cacg-core` to call trust-critical
primitives directly.

### Invariant 3 — emitted artifacts byte-identical across two `KB_FROZEN_CLOCK=1` runs

Verified empirically against the M4b QM corpus:

| artifact (per run)                                 | byte-identical? |
|----------------------------------------------------|----------------:|
| `cfa_vol1_trim/chunks_manifest.json`               | ✅ |
| `cfa_vol1_trim/sources_manifest.json`              | ✅ |
| `qm_notes_trim/chunks_manifest.json`               | ✅ |
| `qm_notes_trim/sources_manifest.json`              | ✅ |
| Migrated cards (sample: `qm-anova-table.md`, `qm-volatility-model-garch-multivariate.md`) | ✅ across end-to-end migrate + index |
| `cards_manifest.json` (across full migrate + index) | ✅ |
| `summaries.json` (across full migrate + index)      | ✅ |

The xtask matrix locks 6 ingest rows
(`kb_ingest_parity_cfa_vol1_trim` + 5 `kb_ingest_parity_qm_*_trim`)
against the committed Python oracles — those rows wouldn't have
stayed PASS across the M4 review cycle if the ingest were
non-deterministic. The
migration + index round-trip preserves card bytes because
`scripts/migrate_qm_cards.py` is byte-stable on the same
inputs and `kb index` always computes the same `card_hash`
for the same frontmatter+body.

**Subtlety**: the migration script writes cards WITHOUT a
`card_hash` field; `kb index` adds it. Re-running ONLY the
migration script (without re-indexing) reverts the cards to
their no-hash state and they're then no longer byte-equal to
the committed (post-index) state. The end-to-end byte-equality
contract holds for `(migrate + index)` as the atomic
operation, which matches the project's documented "kb index
is the canonical hash authority" pattern (M4-Round 18
review P1.3 closure).

**Residual gap (Round-25 review P3-D)**: the migration
script's own byte-stability rests on
`pikepdf.save(deterministic_id=True)` + Python's sorted-dict
iteration; neither is continuously gated by the xtask matrix
(the matrix gates the INGEST side, not the MIGRATION side).
A regression in `scripts/migrate_qm_cards.py` or
`scripts/build_qm_trim_fixtures.py` would surface only at
the next manual re-migration of the QM corpus + the
post-migrate git diff. A future hardening round could wire a
test that re-runs both scripts and asserts byte-equality
against the committed cards.

### Invariant 4 — Pdfium binary build SHA logged and pinned

`docs/pdfium-binary-provisioning.md` Pinned Versions table:

| Component | Pin |
|-----------|-----|
| `pdfium-render` (Rust binding) | `0.9.1` |
| Pdfium native binary           | `149.0.7825.0` (`pypdfium2` 5.8.0 bundle) |
| `libpdfium.so` SHA-256         | `fcd602cd518476d712f661b08e010700490875288fb17069b5b5a2f8b7724118` |

The SHA-256 matches the `pypdfium2` 5.8.0 wheel's bundled
`libpdfium.so` (verifiable with `sha256sum
<pypdfium2-install>/pypdfium2_raw/libpdfium.so`). The xtask
matrix's 6 `kb_ingest_parity_*` rows fail the moment a
different libpdfium build is bound (the AC-5 BYTE-EQUAL
contract holds only for this exact binary).

### Invariant 5 — six xtask static gates + parity rows green

The six static gates:

| gate                              | result |
|-----------------------------------|--------|
| `lint-determinism`                | 0 violations |
| `lint-trust-leak`                 | 0 violations |
| `lint-platform-cfg`               | 0 violations |
| `lint-rename-outside-publisher`   | 0 violations |
| `lint-runner-bypass`              | 0 violations |
| `audit-cacg-core-deps`            | 0 forbidden packages |

Plus `audit-schema-fixtures` (80 fixtures clean) which is
not in the AC-11 explicit count but stays green.

`xtask parity` (the M2 / M5 / M4 parity matrix), run with
`LD_LIBRARY_PATH` bound to the pypdfium2 wheel's
`pypdfium2_raw/` directory so the 6 pdfium-dependent ingest
rows execute:

> `xtask parity: 32 entries; 18 passed, 0 failed, 14 future-stage`

Without `LD_LIBRARY_PATH` set, the 6 `kb_ingest_parity_*`
rows degrade to `FUTURE(M4-pdfium-provisioning)` and the
summary changes to `12 passed, 0 failed, 20 future-stage` —
documented as the CI gap below (and at
`docs/pdfium-binary-provisioning.md` §"CI Gap").

The 14 future-stage rows (in the LD_LIBRARY_PATH-bound
run) are all `help_snapshot_*` rows gated to M3 (not M4).
The 18 PASS rows include:
- 2 `kb_index` rows
- 2 `kb_lint` rows
- 3 `kb_verify` rows (golden / fuzzy / skip-lint)
- 4 `kb_search` rows (corpus / cfa-first-bite / fts5-present / fts5-stale)
- 1 `kb_show` row
- 6 `kb_ingest` rows (M4 — cfa_vol1_trim + 5 QM trims)

`xtask retrieval-eval` (the AC-10 fixture gate):

> `xtask retrieval-eval: 11/11 cases passed; expected-hit-at-k = 10/10`

## AC-11 negative-test posture

The two AC-11 negative tests are:

1. **A Pdfium edge reaching `cacg-core` fails the dependency
   audit.** `xtask audit-cacg-core-deps` walks the full
   transitive non-dev closure via `cargo metadata` and matches
   against the resolved package name; any future edit that
   adds `pdfium-render` (or any other forbidden package) to
   `cacg-core`'s Cargo.toml — directly or transitively —
   surfaces here. The gate is in place; nothing to add.

2. **A non-deterministic emitted artifact fails the
   determinism gate.** The xtask matrix's 18 PASS rows include
   6 `kb_ingest_parity_*` rows that re-run the ingest from
   scratch under `KB_FROZEN_CLOCK=1` and byte-compare against
   the committed Python oracle. The Python oracle is itself
   generated under `KB_FROZEN_CLOCK=1`. Any Rust-side
   non-determinism (random IDs, system-time leaks,
   floating-point order-of-operations drift) would surface as
   a parity FAIL on the next manual `cargo xtask parity` run
   with libpdfium provisioned. The `lint-determinism` static
   gate catches the most common class of accidental
   non-determinism (calls to `OffsetDateTime::now`,
   `SystemTime::now`, `Uuid::new_v4`, etc.) at compile time
   regardless of libpdfium provisioning.

   **CI gap (Round-25 review P2-E)**: today's CI does NOT
   provision libpdfium, so the 6 `kb_ingest_parity_*` rows
   degrade to `FUTURE(M4-pdfium-provisioning)` in CI runs and
   the ingest-determinism gate is operator-bound (catches a
   regression on the next local `cargo xtask parity` invocation
   from a developer-machine setup, not on push). The CI gap is
   documented at `docs/pdfium-binary-provisioning.md` §"CI Gap"
   and will close when a `build.rs` Pdfium-fetch step or a CI
   workflow Pdfium-install step lands. Until then, the
   ingest-determinism enforcement is honest about its
   manual-only reach.

## M4 phase end-state

The M4 plan
(`.humanize/.humanize/plans/cacg-rust-port-m4-ingest-and-migration-pilot-plan.md`)
declared 12 tasks. All 12 are now closed:

| task        | AC      | shipped in                                              |
|-------------|---------|--------------------------------------------------------|
| task-m4-1   | AC-1    | M4a Rounds 0-3 (Pdfium ingest scaffold + fixtures)     |
| task-m4-2   | AC-2    | M4a Round 4 (paragraph-respecting token-budget chunker) |
| task-m4-3   | AC-3    | M4a Round 5 (manifest builder + pair-atomic publish)   |
| task-m4-4   | AC-4    | M4a Rounds 6-7 (kb ingest CLI verb + --config)         |
| task-m4-5   | AC-5    | M4a Round 9 (Pdfium parity checkpoint → BYTE-EQUAL)    |
| task-m4-6   | AC-5    | M4a Rounds 11-12 (parity row in xtask matrix + review) |
| task-m4-7   | AC-6    | M4b Rounds 13-16 (5 trim PDFs + Python supp-plane fix) |
| task-m4-8   | AC-7    | M4b Rounds 17-18 (17 cards, 391 citations, 13 edges)   |
| task-m4-9   | AC-8    | M4b Rounds 19-20 (per-citation Layer-2 tally + shadow) |
| task-m4-10  | AC-9    | M4b Rounds 21-22 (`_research/18` findings artifact)    |
| task-m4-11  | AC-10   | M4b Rounds 23-24 (retrieval-eval QM fixture, 11/11)    |
| task-m4-12  | AC-11   | M4 Rounds 25-26 (this document + review fixes)         |

Plus the major-decisions resolved across the milestone:
- `_research/09` DEC-2 (Pdfium parity policy):
  `RESOLVED-BYTE-EQUAL` (Round 9).
- M4b 13-of-17 vs 17-of-17 scope question: resolved to
  17-of-17 by user sign-off after the Round-13 review
  (Round 15 shipped the 4 deferred trims).

## Round-by-round commit lineage

Rounds 0-25 (Round-0 = the initial `task-m4-1` scaffold). M4a
runs rounds 0-12; M4b runs rounds 13-25:

| round | commit  | content                                                             |
|-------|---------|---------------------------------------------------------------------|
| 0     | 6585d64 | task-m4-1 scaffold (cacg-ingest + pdfium-render wrapper)            |
| 0-fix | a7e19c6 | task-m4-1 Round-0 review fix (per-page panic boundary + docs)       |
| 1     | 72cac6c | task-m4-1 Round 1 (fixture tests + provisioning docs)               |
| 1-fix | 1ea7efe | task-m4-1 Round-2 review fix (terminology + env-gate doc)           |
| 2     | 22919e7 | task-m4-1 Round 2 (zero-page fixture + non-silent gate)             |
| 3     | ee41718 | task-m4-1 Round 3 (encrypted-PDF fixture; AC-1 negative closure)    |
| 3-fix | 5c1bcb6 | task-m4-1 Round-3 review polish                                     |
| 4     | 14e19d1 | task-m4-2 chunker byte-equal port                                   |
| 4-fix | 6c96e46 | task-m4-2 Round-4 review polish                                     |
| 5     | 7955378, 244cb67, f751865 | task-m4-3 manifest builder + Cargo.lock sync + Round-5 review |
| 6-7   | 25793a9, df36bee, d36315d, d712f11 | task-m4-4 CLI verb + dev-dep sync + Round-6 review + --config |
| 8     | (no separate commit; Round-7 review-fix folded into Round 8 below)  |
| 9     | 61e863b | task-m4-5 AC-5 Pdfium parity checkpoint: BYTE-EQUAL outcome         |
| 10    | e251f61 | Round 10: fix Round-9 review findings (DEC-2 status + canonical-bytes) |
| 11    | 5d428f5 | task-m4-6 AC-5 parity gating row in xtask matrix                    |
| 12    | a7cd974 | Round 12: fix Round-11 review findings (lockstep helper + always-build) |
| 13    | 8fa68d1 | task-m4-7 trimmed QM-source PDF + real ingest (AC-6 baseline)       |
| 14    | 34001e1 | Round 14: fix Round-13 review (pikepdf determinism + pin)           |
| 15    | 257ff9e | M4-Round 15: 3 more QM trims + Python supplementary-plane fix       |
| 16    | 8c58952 | M4-Round 16: fix Round-15 review (pyproject + public-API mock + regression) |
| 17    | bcc0c3b | M4-Round 17: task-m4-8 migrate 17 QM cards (AC-7)                   |
| 18    | 67b6ec4 | M4-Round 18: fix Round-17 review (fan-out + card_edges + page offsets) |
| 19    | 1fe8ee9 | M4-Round 19: task-m4-9 Layer-2 per-citation tally (AC-8)            |
| 20    | fbfcce7 | M4-Round 20: fix Round-19 review (BM25 scope + paraphrase shadow tally) |
| 21    | a44cdec | M4-Round 21: task-m4-10 `_research/18` findings artifact (AC-9)     |
| 22    | 8595b27 | M4-Round 22: fix Round-21 review (fuzzy bound + density data + xref) |
| 23    | 595cc16 | M4-Round 23: task-m4-11 retrieval-eval QM fixture (AC-10)           |
| 24    | 2c40640 | M4-Round 24: fix Round-23 review (second auth branch + naming)      |
| 25    | 6cb6775 | M4-Round 25: task-m4-12 / AC-11 cross-cutting invariants + this doc |
| 26    | (this commit) | M4-Round 26: fix Round-25 review (lineage SHAs + CI gap + supp doc) |

## Hand-off to M5+

The next milestone consumes this M4 foundation as fixed
ground:

1. **Ingest is real.** Any future verticals call the same
   `kb ingest` (or its Rust crate) on the same Pdfium pin;
   the BYTE-EQUAL contract carries forward.
2. **The migration pattern is reproducible.** A new vertical
   follows the §4 mapping recipe in
   `_research/18_cfa_real_migration_findings.md`: build
   trims with deterministic `pikepdf.save(deterministic_id=True)`,
   ingest with `KB_FROZEN_CLOCK=1`, parameterize
   `_KEYWORD_TAGS` + `PRIMARY_SOURCE_MAP` +
   `TRIM_PAGE_OFFSETS` for the new corpus, run `kb index`
   to compute card_hashes, drop a retrieval-eval fixture for
   AC-10-equivalent coverage.
3. **Mandatory page-offset verification.** Section 7 of
   `_research/18` flags the single most expensive M4b lesson
   (the book-page-vs-PDF-page off-by-19/25/1 trim bug). Every
   future trim build MUST sample page 1 against the source
   PDF before extracting; the M4 plan's AC-6 wording needs
   tightening as a follow-up.
4. **Verbatim quotes vs paraphrase quotes is a deployment
   choice.** The 100% strict Layer-2 pass rate from the
   primary tally is structural; the 0% strict rate from the
   shadow paraphrase tally is the realistic ground-truth
   measurement. A future "Layer-3 capacity planning" round
   should sweep fuzzy across a sample to bound the
   "Layer-3-required" count above 0 (Round 19 review P2-B
   carry-forward).
5. **`kb index` is the canonical `card_hash` authority.** The
   migration script emits cards WITHOUT `card_hash`; `kb
   index` computes + writes it. Future migration scripts
   MUST NOT compute `card_hash` themselves — Pydantic's
   `model_dump(mode="python")` injects schema defaults
   (`tags: []`, `card_edges: []`) that an external computer
   cannot exactly mirror, producing silent hash drift. Round
   18 review P1.3 documented this as the closure for the
   "deterministic re-run" AC-7 contract; preserved here so a
   future migration team doesn't reinvent the in-script hash
   anti-pattern.

M4 complete.
