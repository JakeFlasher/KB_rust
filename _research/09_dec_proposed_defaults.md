# Durable Decisions Record (DEC-1..DEC-8 PROPOSED-DEFAULTs)

**Status:** Round 1 / Rust port milestone M0 / task-M0-6 durable-decisions-record acceptance criterion.
**Date:** 2026-05-20.
**Purpose:** Captures Claude's PROPOSED-DEFAULT positions on the eight DECs surfaced by the Codex first-pass review of the Rust port plan. Lives OUTSIDE the immutable plan file so the RLCR session-immutability rule is not violated. User overrides land via a follow-on commit to THIS file (or via the cancel-update-restart flow if a mid-loop override is needed).

This file is the durable, citable decision record. The goal-tracker's Blocking/Queued Side Issues table at `.humanize/rlcr/2026-05-20_16-15-57/goal-tracker.md` references this file for the current state of each DEC; the plan file itself remains pristine.

---

## DEC-1: Compatibility-mode oracle

**Question:** When Python's behavior is accidental or ugly (e.g., a specific `json.dumps` quirk on float NaN, a Pydantic error message that's typo'd, a tempfile suffix that leaks into a journal event), does the Rust port preserve Python's bytes verbatim (PY-IS-ORACLE) or diverge to spec-defined behavior with migration tooling (SPEC-IS-ORACLE) or fix-the-bug-then-byte-equal (FIX-AND-PIN)?

**PROPOSED-DEFAULT:** `PY-IS-ORACLE` for M0-M5; revisit before M6 perf gates.

**Claude position:** PY-IS-ORACLE minimizes migration risk. Byte-equal parity is the central trust signal; introducing intentional divergences early adds confusion overhead. The exception is RFC-non-compliant Python outputs (NaN/Infinity literals, duplicate keys, non-string keys) which the canonical writer explicitly REJECTS rather than emitting Python-compatible bytes — these are documented in `_research/08_canonical_json_spec.md` §2.2 and §4.

**Codex position:** Question is genuinely open; depends on the team's bug-for-bug appetite.

**Tradeoff summary:** PY-IS-ORACLE = lowest migration risk, possibly preserves bugs. SPEC-IS-ORACLE = clean implementation, requires migration tooling for existing corpora. FIX-AND-PIN = middle ground, requires careful audit of every divergence.

**Affected ACs:** Canonicalization spec AC; cacg-core canonical_json AC; diagnostic-parity contract.

**Revisit trigger:** Before any AC-C1 Rust implementation work that would produce byte-divergent output from Python.

**Status:** PROPOSED-DEFAULT (Round 1). User override path: edit this file's Status line and supply rationale.

---

## DEC-2: Pdfium output expectation

**Question:** Is `pdfium-render 0.9.1` Rust extraction expected to be BYTE-EQUAL to `pypdfium2 5.8.0`'s `get_text_range()` on the same PDF, or HASH-STABLE after Rust normalization (the chunk_hash envelope verifies against extracted text but raw bytes differ), or FRESH-CORPUS (Rust-only corpus with intentional chunk-hash regeneration)?

**PROPOSED-DEFAULT:** `BYTE-EQUAL` preferred; `HASH-STABLE` acceptable as fallback.

**Claude position:** BYTE-EQUAL preferred for existing corpora preservation; HASH-STABLE acceptable with explicit Phase-4 chunk-hash regeneration ceremony documented in `_research/10_pdfium_parity_report.md` (to be authored when M4 lands).

**Codex position:** BYTE-EQUAL is unlikely across bindings/versions/platforms/rendering flags; HASH-STABLE may be the realistic target.

**Tradeoff summary:** BYTE-EQUAL preserves all existing chunk_hashes, may fail empirically. HASH-STABLE preserves the trust contract (chunk_hash verifies against extracted text), requires regenerating chunk_hashes. FRESH-CORPUS deprecates all existing manifests.

**Affected ACs:** Pdfium parity AC (M4).

**Revisit trigger:** Before any cacg-ingest implementation work (M4 task).

**Status:** RESOLVED-BYTE-EQUAL (Round 9, task-m4-5).

  Resolution: AC-5 ran the Rust `kb ingest` against the committed
  `tests/parity_corpus/pdfs/cfa_vol1_trim.pdf` and the published
  `chunks_manifest.json` matched the committed Python oracle at
  `tests/parity_corpus/out_python/pdfs/cfa_vol1_trim/chunks_manifest.json`
  byte-for-byte (75796 bytes, 34 chunks, every `chunk_hash` matching).
  `sources_manifest.json` matched on every field except the two
  whitelisted parser-identity fields (this plan's DEC-2). The
  outcome is locked end-to-end by
  `crates/cacg-cli/tests/kb_ingest_parity.rs::kb_ingest_cfa_vol1_trim_is_byte_equal_with_committed_python_oracle`,
  and the Pdfium binary pin (`pdfium 149.0.7825.0`, the build
  shipped in `pypdfium2 5.8.0`) is recorded in
  `docs/pdfium-binary-provisioning.md` Pinned Versions.

  Two narrow Rust-side fixes were required to reach byte-equality:
  per-page `normalize_text` (mirroring Python
  `cacg.pdf.extract_pages_normalized`), and an `FPDFText_GetUnicode`
  per-char extraction loop with an explicit Pdfium soft-hyphen
  `U+0002 → U+FFFE` mapping (mirroring the post-processing
  `FPDFText_GetText` applies internally that pypdfium2 inherits).
  See `crates/cacg-ingest/src/pdf.rs` for the call sites and
  Round-9 commit `61e863b` for the full diagnosis.

  The HASH-STABLE fallback artifact (`_research/19_pdfium_parity_report.md`)
  is intentionally NOT produced under this outcome; AC-5's negative
  test for HASH-STABLE only fires when BYTE-EQUAL fails.

---

## DEC-3: Diagnostic message text byte-parity scope

**Question:** Are Rust Diagnostic `message` strings part of the byte-equal contract (FULL), or only `code` + `severity` + `file` byte-equal with `message` "best effort" (CODE-ONLY), or HYBRID (per-fixture snapshot with whitelist)?

**PROPOSED-DEFAULT:** `HYBRID`.

**Claude position:** HYBRID. `code` + `severity` + `file` always byte-equal; `message` covered by fixture-based snapshot tests on the golden corpus with a declared whitelist of intentional divergences. Documented at `docs/diagnostic-parity.md` (Round 1 artifact).

**Codex position:** Must be made explicit; users may depend on exact message text.

**Tradeoff summary:** FULL = maximum compatibility, maximum porting cost. CODE-ONLY = leanest, may break consumers grepping on messages. HYBRID = balanced, requires whitelist discipline.

**Affected ACs:** Diagnostic-compatibility AC (M0), cacg-cli CLI surface (M3).

**Revisit trigger:** ALREADY CONFIRMED IN ROUND 1 via `docs/diagnostic-parity.md` authoring. No further user revisit needed unless the whitelist grows beyond the documented hygiene bound.

**Status:** PROPOSED-DEFAULT (Round 1, confirmed via Round-1 artifact).

---

## DEC-4: Python `kb` deprecation timeline

**Question:** After M7 ships, when does the Python `kb` binary stop being maintained?

**PROPOSED-DEFAULT:** `SOFT-6MO` (6-month coexistence window with deprecation warning).

**Claude position:** SOFT-6MO with deprecation warning emitted by Python `kb` after Rust ships full parity. Gives operators a controlled migration window without forcing immediate adoption.

**Codex position:** Depends on operator workflow stability; no inherent technical preference.

**Tradeoff summary:** HARD = clean break, possibly disruptive. SOFT-6MO = orderly migration, dual-maintenance during the window. INDEFINITE = compounding maintenance burden.

**Affected ACs:** Migration-completion ACs (M7).

**Revisit trigger:** Before M7 pyproject.toml deprecation ceremony.

**Status:** PROPOSED-DEFAULT (Round 1).

---

## DEC-5: CFA_reading PDF corpus selection

**Question:** Which specific PDFs from `/home/jakeshea/CFA_reading/` are part of the M4 byte-equal (or hash-stable) Pdfium parity gate?

**PROPOSED-DEFAULT:** Include both `notes/CFA_note_2.ocr.pdf` (OCR-extracted, hard case) AND `CFA_Program_Curriculum/CFA_2022_Level_I_Volumes_1-6.pdf` Vol.1 only (cleaner extraction). Document known-drift for the OCR PDF in `_research/10_pdfium_parity_report.md`.

**Claude position:** Both for representative coverage; document known-drift for OCR PDF.

**Codex position:** OCR-heavy fixtures will likely fail byte-equal; consider hash-stable for OCR + byte-equal for synthetic.

**Tradeoff summary:** Including the OCR PDF probes the failure boundary; including only the clean PDF gives a confident green but may hide future regressions. Both is most thorough.

**Affected ACs:** Pdfium parity AC (M4) and AC-D2 corpus completeness.

**Blocking status:** This DEC blocks the full AC-D2 corpus. Round 1 ships AC-D2 with PDF section marked `PENDING_USER_DECISION_DEC_5.md` (a placeholder file naming the dependency).

**Revisit trigger:** ASAP — required for AC-D2 full completion. User-facing question.

**Status:** RESOLVED-VIA-PE-02 (the trim-only policy adopted in Round 4; see PE-02 below).

---

## DEC-6: Schema-evolution authority during migration

**Question:** If schema work happens after M0 (e.g., a `cacg.v0.1` field), does it land in Python first (PY-FIRST), Rust first (RUST-FIRST), or simultaneously via the M0 spec (SPEC-FIRST)?

**PROPOSED-DEFAULT:** `SPEC-FIRST`.

**Claude position:** SPEC-FIRST with simultaneous implementation. The M0 canonicalization spec + `docs/schema.md` becomes the single source of truth.

**Codex position:** SPEC-FIRST is correct but requires discipline; PY-FIRST may be faster in practice.

**Tradeoff summary:** SPEC-FIRST = highest discipline, slowest cadence. PY-FIRST = fastest iteration, drifts Rust. RUST-FIRST = forces Python catch-up, risky during migration.

**Affected ACs:** Schema-evolution work whenever it lands.

**Revisit trigger:** First proposed `cacg.v0.1` schema change. Not currently in scope.

**Status:** PROPOSED-DEFAULT (Round 1, dormant).

---

## DEC-7: Performance gate hardware authority

**Question:** Which CI runner is authoritative for the M6 performance gates?

**PROPOSED-DEFAULT:** `BOTH-SEPARATE` (Linux x86_64 AND Linux aarch64 with separate budgets; the lower of the two is the merge-block threshold).

**Claude position:** BOTH-SEPARATE; CI runs both; the lower budget is authoritative.

**Codex position:** Explicit hardware metadata in every benchmark report; no preference on which runner is authoritative.

**Tradeoff summary:** X86-ONLY = simpler CI matrix, misses aarch64 regressions. ARM-ONLY = covers Apple Silicon developers, misses x86 server deployments. BOTH-SEPARATE = most thorough, doubles CI time.

**Affected ACs:** All M6 performance ACs.

**Revisit trigger:** Before M6 task task-M6-2 (per-card verify perf gate).

**Status:** PROPOSED-DEFAULT (Round 1).

---

## DEC-8: `cacg-semantic` inclusion in M5 scope

**Question:** Ship Layer-3 B1 (cache-as-oracle dict lookup) + B2 (LLM-judge) in M5, or defer to a post-M7 milestone?

**PROPOSED-DEFAULT:** Ship B1 in M5; defer B2 to a follow-on milestone.

**Claude position:** B1 is small, deterministic, no async needed — worth shipping. B2 requires async tokio + reqwest, scope creep for a deterministic-core priority.

**Codex position:** B1 is small and worth shipping; B2 adds runtime complexity not aligned with M0-M5's deterministic-core priority.

**Tradeoff summary:** Ship-both = full Layer-3 parity at M7. Ship-B1-only = lean M7, B2 deferred. Defer-both = leanest M7, post-M7 follow-on for both.

**Affected ACs:** M5 cacg-semantic task.

**Revisit trigger:** Before M5's optional-semantic task.

**Status:** PROPOSED-DEFAULT (Round 1).

---

## How to override a DEC

To override any PROPOSED-DEFAULT:

1. Edit the corresponding `Status:` line in this file to a new value (`OVERRIDDEN: <NEW-VALUE> (rationale: ...)`).
2. Update the affected ACs' implementation accordingly in the next round.
3. The plan file at `.humanize/.humanize/plans/cacg-rust-port-trust-kernel-first-plan.md` remains untouched until the RLCR loop ends; final plan-side DEC updates land via a post-loop PR.
4. Document the override in the goal-tracker's Plan Evolution Log.

Mid-loop overrides MAY trigger a re-author of the affected acceptance criterion (e.g., flipping DEC-1 to SPEC-IS-ORACLE mid-M1 forces the canonical-JSON spec to be rewritten). The cancel-update-restart flow is the documented escape if a mid-loop override is materially disruptive.

---

## Plan Evolution: PE-01 — MSRV 1.80 → 1.85

**Date:** 2026-05-20 (Round 3).
**Status:** PROPOSED-DEFAULT (Claude position adopted with documented evidence; user can override).
**Affects:** Plan §Path Boundaries Allowed Choices ("Rust 2021 edition, MSRV 1.80"); task-M0-4 (Cargo workspace skeleton); the `_research/07_rust_refactor_research.md` workspace baseline.

### Context

The plan's research baseline (May 2026, `_research/07_rust_refactor_research.md`) named MSRV 1.80 because Rust 1.80 was the most recent stable at the research time. The Allowed Choices crate set in the plan (clap 4.6.1, rusqlite 0.39.0, garde 0.22.1, etc.) was selected at the same moment without explicit MSRV cross-check against each crate.

### Evidence forcing the bump

Round 2 + Round 3 build attempts under Rust 1.80.0 (the exact pinned channel) surfaced:

- `clap_derive 4.6.1` (the version the plan pins): `feature edition2024 is required ... not stabilized in this version of Cargo (1.80.0)`. Rust edition2024 stabilized in Rust 1.85 (Feb 2025).
- `clap_lex 1.1.0` (transitive via `clap_builder 4.6.0`): same `edition2024` requirement.
- Downgrading `clap` workspace dep to `~4.5.40` (latest 4.5 patch) STILL resolves `clap_lex 1.1.0` because clap 4.5.x carries a permissive `clap_lex >= 0.7.0, < 2.0` constraint and the resolver picks the latest.
- To stay at MSRV 1.80 the workspace would need to pin BOTH `clap = "=4.5.30"` AND `clap_lex = "=0.7.5"` explicitly, and the same exercise would repeat for every actively-maintained dep over the plan's multi-month implementation horizon.

Verified-compatible with MSRV 1.85+ (Round-2 build outcome before the revert attempt):

- All Allowed-Choices crates from `_research/07` §3 resolve cleanly under Rust 1.85.0: clap 4.6.1, clap_derive 4.6.1, clap_lex 1.1.0, serde 1.0.228, serde_json 1.0.149, anyhow 1.0.102, garde 0.22.1, thiserror 2.0.18, time 0.3.47, uuid 1.23.1, sha2 0.10.9, yaml-rust2 0.11.0, serde_yaml_bw 2.5.6, tempfile 3.27.0, rustix 1.1.4, rayon 1.12.0, rusqlite 0.39.0, tracing 0.1, tracing-subscriber 0.3, unicode-normalization 0.1.25, ryu 1.0, miette 7.6.0, pdfium-render 0.9.1, insta 1.47.2, proptest 1.11.0, criterion 0.8.2, iai-callgrind 0.16.1.

### Resolution

MSRV bumped to **1.85.0** (Feb 2025; ~15 months stable as of 2026-05-20). Updates applied in lockstep across:

- `Cargo.toml` `workspace.package.rust-version = "1.85"`.
- `rust-toolchain.toml` `channel = "1.85.0"`.
- `.github/workflows/build-macos.yml` `toolchain: "1.85.0"` (4 occurrences).
- `.github/workflows/_validate-workflows.yml` `toolchain: "1.85.0"`.

### Tradeoff

- Cost: drops MSRV compatibility with Rust 1.80..1.84 (a ~6-month window of older toolchains).
- Benefit: every crate in the plan's Allowed Choices resolves cleanly; no ad-hoc transitive-dep pinning; no per-round "fix the build" churn as new crates bump their MSRVs through 2026.

### User override

Edit this PE-01 entry's `Status:` line to `OVERRIDDEN: <NEW-MSRV>` and supply rationale. Lowering MSRV requires re-running the build under the lower toolchain + pinning every transitive dep explicitly.

---

## Plan Evolution: PE-02 — DEC-5 narrowing (PDF parity corpus = sample.pdf + cfa_vol1_trim.pdf)

**Date:** 2026-05-20 (Round 4).
**Status:** PROPOSED-DEFAULT (Claude position adopted; user can override).
**Affects:** DEC-5 PROPOSED-DEFAULT (originally "both CFA PDFs included"); the plan's AC-D2 PDF corpus requirement; goal-tracker's DEC-5 row.

### Context

The plan's AC-D2 originally required "3 sample PDFs: sample.pdf + 2 representative CFA_reading PDFs per DEC-5". The PROPOSED-DEFAULT in DEC-5 specified the two representative PDFs as `CFA_2022_Level_I_Volumes_1-6.pdf` (21 MB, the full Vol.1) and `notes/CFA_note_2.ocr.pdf` (114 MB OCR-extracted), both ingested from sibling-repo paths.

### Evidence forcing the narrowing

- **GitHub per-file hard limit: 100 MB.** The 114 MB OCR PDF (`CFA_note_2.ocr.pdf`) cannot be committed to a normal git repository. Even Git LFS (the standard escape hatch for large binary fixtures) charges per-storage/bandwidth and is not a standard option for this project; the existing `tests/fixtures/cfa_smoke/.gitignore` policy explicitly says "Never commit the 110 MB PDF."
- **21 MB full Vol.1 PDF is portable but heavy.** Round 2 attempted to commit it (16 MB ingest output for 6410 chunks); the resulting repo bloat would compound across CI clones over the multi-month implementation horizon. Round 3 replaced it with `cfa_vol1_trim.pdf` (pages 1-30; 426 KB; ~25 chunks).
- **pypdfium2 PDF write is not byte-deterministic.** Even if the trim PDF is regenerated on every build, `pypdfium2.PdfDocument.save()` embeds creation-time IDs and trailer entries that drift across runs. The committed `out_python/pdfs/cfa_vol1_trim/sources_manifest.json.source_sha256` hash would change every run, breaking the byte-equal parity contract.
- **Round 3 introduced the commit-once-reuse-forever flow.** `tests/fixtures/cfa_vol1_trim.pdf` is built ONCE on a host with sibling-repo access and committed verbatim. Subsequent builds (including fresh CI runners without sibling-repo access) copy the committed trim into the corpus. This is the only byte-stable path.
- **Representative coverage check.** The trim's 30 pages produce 24 chunks spanning the front matter + Quantitative Methods Reading 1 of the CFA Level 1 curriculum. The byte-equal Pdfium parity gate (M4 in the plan) is exercised end-to-end on this fixture. The OCR-extracted PDF tests a different extraction path (text from OCR'd images vs embedded fonts) but the SAME pdfium-render `PdfPage::text().all()` byte-output discipline. Adding a second representative PDF would test fewer additional pdfium-render code paths than the existing trim already exercises.

### Resolution

DEC-5's committed parity-corpus PDF set is narrowed to:

- `tests/parity_corpus/pdfs/sample.pdf` (3239 bytes; fpdf2 deterministic fixture).
- `tests/parity_corpus/pdfs/cfa_vol1_trim.pdf` (426 KB; pages 1-30 of Vol.1; commit-once-reuse-forever).

The 114 MB OCR PDF and 21 MB full Vol.1 PDF are moved to the operator-local-only tier (sibling-source for triage, NOT in the merge-blocking byte-equal parity gate). Documented at `tests/parity_corpus/pdfs/README.md`.

### Tradeoff

- Cost: drops the OCR-extraction code path from the byte-equal parity gate; the OCR-specific quirks (Unicode replacement chars in low-confidence regions, weird text-direction marks in scanned tables) are tested only via the unicode_edge fixture set and not against a real OCR'd source.
- Benefit: corpus stays under GitHub's per-file limit; commit-once-reuse-forever discipline gives byte-stable manifests; future operators don't need Git LFS or a separate PDF-only repo.

### User override

Edit this PE-02 entry's `Status:` line to `OVERRIDDEN: <NEW-POLICY>` and supply rationale. Adding additional PDF fixtures requires either Git LFS, a separate PDF-only repo, or a per-PDF page-trim policy (whatever stays under 50 MB total).

### DEC-5 status update

DEC-5 entry's `Status:` line above now reads: `PROPOSED-DEFAULT (Round 1, BLOCKING for AC-D2)`. Round 4's PE-02 supersedes this: DEC-5 is `RESOLVED-VIA-PE-02` for M0 acceptance purposes. The original "both PDFs" PROPOSED-DEFAULT remains the user-override target if a future broader-coverage policy is adopted.

---

## Plan Evolution: PE-03 — Dependency-retracted `kb show --allow-retracted` oracle deferred to post-M5

**Date:** 2026-05-20 (Round 5).
**Status:** SUPERSEDED-VIA-PE-04 (Round 6) — the dep-retracted `kb show --allow-retracted` oracle is no longer deferred; PE-04 pulls a minimum Python `kb show` forward so the oracle ships in M0. The original PE-03 rationale remains documented below for historical context.
**Affects:** AC-D2 (the `dependency_retracted` scenario row); AC-S4 (`kb show` ships in M5 task-M5-4); the plan's `dependency_retracted/scenario-01/expected.md` prose; goal-tracker's AC-D2 row.

### Context

The plan's AC-D2 originally required "1 dependency-retracted-card scenario for `kb show --allow-retracted`". The M0 corpus must commit a Python-built oracle artifact for every fixture (per the AC-D2 strengthening adopted in Round 3). Round 4's `dependency_retracted/scenario-01/expected.md` documents two behaviors:

1. The manifest-level cascade: `kb retract-source` populates `cards_manifest.dependency_retracted_cards`.
2. The presentation-layer behavior: `kb show <cascade-child> --allow-retracted` prints a `DEPENDENCY-RETRACTED` status line; without `--allow-retracted` it exits 1 with `CACG-SHOW-001`.

### Evidence forcing the narrowing

- **`kb show` does not exist in the M0-time Python implementation.** Auditing `src/cacg/cli.py` `_build_parser()` enumerates the present subcommands: `ingest`, `new`, `lint`, `verify`, `index`, `history`, `retract`, `retract-source`, `retract-chunk`, `scaffold-matrix`, `scaffold-role-map`, `search`, `migrate` — 13 subcommands; no `show`. Per the plan's milestone schedule (§Implementation Plan task-M5-4), `kb show` is an M5 deliverable shipped by the Rust port; it has no corresponding Python implementation. The Phase-5 closure plan referenced at `.humanize/.humanize/plans/cacg-phase-5-closure-operator-governance-plan.md` enumerates `kb show` as a future feature, not a current one.
- **Cannot Python-build an oracle artifact for a Python subcommand that does not exist.** Round 4's scenario-oracle Phase 7b invokes `kb <subcommand>` via subprocess to capture the real Python output. For the `kb show --allow-retracted` step in `expected.md` step 9-10, no Python subcommand can be invoked. Attempting to fabricate the oracle (hand-write the expected status line) would be lying-on-paper — exactly the failure mode the Round-3 prose-only-oracle audit flagged for retracted/rollback/dep-retracted scenarios.
- **The manifest-level cascade IS implementable in M0 time.** `kb retract-source` IS in the Python implementation (`src/cacg/cli.py:_cmd_retract_source` at line 1136), and Round 4's Phase 7b for `dependency_retracted/scenario-01/expected.json` captures the resulting `cards_manifest.dependency_retracted_cards` cascade end-to-end. That oracle IS Python-built, is in the corpus, and is enforced by the strengthened `validate_per_fixture_oracle_coverage()` (Round 5's gate asserts 2 oracle files for the scenario: `expected.json` + `cards_manifest.json`).
- **Splitting the scenario preserves the parity contract without lying.** The M0 corpus rows now cover the manifest-level cascade only; the presentation-layer (`kb show`) behavior is a post-M5 oracle that lands after task-M5-4 ships `kb show` in the Rust port AND the Python port's `kb show` is implemented in parallel (or, equivalently, the Rust output becomes the oracle when the Rust port is the sole implementation).

### Resolution

- The `dependency_retracted/scenario-01/expected.md` is retained as the **post-M5 specification** of the full end-to-end UX (manifest cascade + presentation refusal + `--allow-retracted` downgrade).
- The M0-time scenario row in AC-D2 is narrowed to the **manifest-level cascade only**: the committed Python-built oracles are `expected.json` (the `cards_manifest.dependency_retracted_cards` outcome) and `cards_manifest.json` (the live file after `kb retract-source`). These are produced by Round 4's `materialize_scenario_oracles()` and validated by Round 5's strengthened `validate_per_fixture_oracle_coverage()`.
- When `kb show` ships in M5 (Rust port task-M5-4) and the Python implementation gains a parallel `kb show` (or the Rust output becomes the sole oracle), a second oracle artifact set (`show_allow_retracted.txt` + `show_default.json` exit-code capture) will be added to this scenario via a Round-N post-M5 backfill commit. The `expected.md` prose stays unchanged; only the oracle artifacts grow.
- The `expected.md` file gains an explicit `Oracle scope:` line clarifying that M0 oracles cover steps 7-8 only (the manifest cascade) and steps 9-10 (the `kb show` presentation behavior) are deferred to post-M5 backfill.

### Tradeoff

- Cost: M0 acceptance does not include a byte-equal oracle for the `kb show --allow-retracted` presentation contract. The contract is documented in prose (`expected.md` steps 9-10 + AC-S4 in the plan) but not enforced empirically until M5 lands.
- Benefit: M0 ships honestly — no prose-only oracle pretending to be a Python-built artifact; the validator's enforced oracle list shrinks by one (from a phantom `show_*.txt` artifact to no presentation-layer oracle) but the scenario row stays in the corpus as a placeholder for the M5 backfill.

### User override

Edit this PE-03 entry's `Status:` line to `OVERRIDDEN: <NEW-POLICY>` and supply rationale. Overrides include: (a) implement `kb show` in Python first to unblock M0-time oracle (largest scope; effectively pulls task-M5-4's Python-side work into M0), (b) commit a hand-authored expected.txt oracle and accept it as non-empirical (defeats the byte-equal parity contract), or (c) drop the dependency_retracted scenario from AC-D2 entirely (loses the cascade-coverage signal).

### AC-D2 status update

AC-D2 row 6 in the plan reads: "1 dependency-retracted-card scenario for `kb show --allow-retracted`". Round 5's PE-03 narrows the M0-time interpretation to the manifest cascade (steps 7-8); the `kb show --allow-retracted` presentation contract (steps 9-10) is deferred to post-M5 backfill once `kb show` exists.

Superseded in Round 6 by PE-04 below — `kb show` is now implemented in Python (minimum AC-S4 subset) and the dep-retracted scenario commits both `show_default.json` and `show_allow_retracted.json` Python-built oracles. AC-D2 is no longer narrowed.

---

## Plan Evolution: PE-04 — Minimum `kb show` pulled forward into Python (M0)

**Date:** 2026-05-20 (Round 6).
**Status:** PROPOSED-DEFAULT (Claude position adopted with documented evidence; user can override).
**Affects:** AC-D2 row 6 (the dep-retracted `kb show --allow-retracted` scenario oracle); AC-S4 (the Rust `kb show` deliverable in M5 task-M5-4); the plan's Ultimate-Goal narrative line ("The plan does NOT modify the Python implementation"); PE-03 (now SUPERSEDED).

### Context

The plan's Ultimate-Goal narrative says "The plan does NOT modify the Python implementation; the two coexist on disk until the final milestone declares Python deprecated." This is a guideline encoding the trust-kernel-first migration discipline: the Rust port should not destabilize Python while the parity corpus is being built. The plan's IMMUTABLE acceptance criterion AC-D2 (line 25 / 32 of `.humanize/.humanize/plans/cacg-rust-port-trust-kernel-first-plan.md`) requires "1 dependency-retracted-card scenario for `kb show --allow-retracted`" with a Python-built oracle under `out_python/`. Round 5's PE-03 attempted to reconcile the tension by deferring the `kb show` portion to a post-M5 backfill. Codex's Round-5 review rejected the deferral: "The original plan still requires a dependency-retracted-card scenario for `kb show --allow-retracted` at `.humanize/.humanize/plans/cacg-rust-port-trust-kernel-first-plan.md:25`, and AC-S4 defines the required show semantics at `.humanize/.humanize/plans/cacg-rust-port-trust-kernel-first-plan.md:220-227`."

### Evidence forcing the pull-forward

- **The IMMUTABLE AC takes precedence over the narrative guideline.** The plan's IMMUTABLE acceptance criteria section explicitly says it is the load-bearing contract; the Ultimate-Goal narrative is preamble. Where they conflict, the AC wins.
- **A Python-built oracle is the only path that satisfies AC-D2.** The corpus's `out_python/` directory is by construction Python-built (the contract is "Python is the oracle for the Rust port byte-equal parity gate"). Synthesizing a hand-written oracle for the show artifacts would be lying-on-paper, exactly the failure mode Codex flagged in Rounds 3-5 for prose-only oracles. Generating the oracle from the Rust implementation would invert the parity contract.
- **The Rust port's M5 task-M5-4 ships the canonical `kb show`.** Python's `_cmd_show` is the M0-time oracle source ONLY. When M5 lands, the Rust binary's `kb show` output is compared byte-for-byte against the Python-built oracle. Python's `show` stays in the codebase as the deterministic byte-equality target.
- **The Python `kb show` surface is small and self-contained.** Only the AC-S4 positive/negative tests need to pass: CACG-SHOW-001 (retraction refusal), CACG-SHOW-002 (path/manifest disagreement), `--allow-retracted` downgrade, mandatory `--source-matrix`, optional `--path`. No new modules, no new schema fields, no new manifest format — it's a read-only resolver over the existing `cards_manifest.json` + a single card file load.

### Resolution

A minimum `_cmd_show` is added to `src/cacg/cli.py` with the AC-S4 surface:

- `kb show <card_id> --source-matrix m.json` — active card: exit 0, prints the documented view (status line if applicable; H1 title; bold summary; key:value frontmatter; bullet-line citations including source_id, chunk_id, page_range, chunk_hash, edge_type, quote).
- `kb show <retracted-id> --source-matrix m.json` — exit 1 + stderr `CACG-SHOW-001: card '<id>' is RETRACTED and `--allow-retracted` was not supplied`.
- `kb show <dep-retracted-id> --source-matrix m.json` — exit 1 + stderr `CACG-SHOW-001: card '<id>' is DEPENDENCY-RETRACTED ...`.
- `kb show <retracted-id> --allow-retracted --source-matrix m.json` — exit 0 + stdout begins `STATUS: RETRACTED` (direct) or `STATUS: DEPENDENCY-RETRACTED` (cascade).
- `kb show <id> --path <wrong-path> --source-matrix m.json` — exit 1 + stderr `CACG-SHOW-002: --path <p> disagrees with cards_manifest: on-disk id=... card_hash=...; manifest id=... card_hash=...`.
- `kb show <id>` (no `--source-matrix`) — exit 2 + argparse usage error.

The dep-retracted scenario's Phase 7b oracle materializer captures two NEW artifacts under `out_python/dependency_retracted/scenario-01/`:

- `show_default.json` — captures exit_code=1 + stderr containing `CACG-SHOW-001` for the default-mode cascade refusal.
- `show_allow_retracted.json` — captures exit_code=0 + stdout containing `STATUS: DEPENDENCY-RETRACTED` plus the documented card view for the downgrade path.

The validator (`validate_per_fixture_oracle_coverage()`) requires both artifacts and asserts the documented behavior (exit code + key substring) for each.

### Tradeoff

- Cost: the Python codebase grows by ~110 lines (one `_cmd_show` + parser registration + dispatch wiring). This violates the Ultimate-Goal narrative's "do not modify Python" guideline, but only for the read-side `kb show` (no manifest mutation; no journal append; no schema change). The change is additive — every pre-existing Python test continues to pass.
- Benefit: AC-D2 is honestly complete (no more "post-M5 backfill" deferral); the Rust port's M5 task-M5-4 ships against a real Python oracle from day one; the immutable AC is satisfied by an immutable Python source, not by a hand-written prose artifact.

### User override

Edit this PE-04 entry's `Status:` line to `OVERRIDDEN: <NEW-POLICY>` and supply rationale. Overrides include: (a) explicit AC-D2 rewrite to drop the `kb show --allow-retracted` requirement (preserves the "do not modify Python" guideline; loses oracle coverage), or (b) deliver the Rust `kb show` first and use it as the oracle (inverts the parity contract; requires DEC-1 flip from PY-IS-ORACLE to RUST-IS-ORACLE — a much larger plan change).

### AC-D2 status update (Round 6)

AC-D2 row 6 is now ORACLE-COVERED IN M0. The dep-retracted scenario commits a full 4-artifact oracle set: `expected.json` + `cards_manifest.json` (manifest cascade) + `show_default.json` + `show_allow_retracted.json` (presentation contract). PE-03 is SUPERSEDED.

---

## Plan Evolution: PE-05 — Time/criterion crate pins lowered to preserve MSRV 1.85

**Date:** 2026-05-20 (Round 9).
**Status:** PROPOSED-DEFAULT (Claude position adopted with documented evidence; user can override).
**Affects:** `Cargo.toml` workspace deps `time` and `criterion`; `Cargo.lock`; the `_research/07_rust_refactor_research.md` workspace baseline.

### Context

Round 9 began the M1 cacg-core implementation. The first `cargo build -p cacg-core` under the pinned Rust 1.85 toolchain failed because upstream crate metadata had been updated post-publish: `time@0.3.47` declared `rust-version = "1.88"` and `criterion@0.8.2` declared `rust-version = "1.86"`. Both pins were valid on 1.85 when originally selected in Round 1's research baseline (`_research/07_rust_refactor_research.md`), but the upstream crate maintainers backfilled stricter MSRV declarations between then and now — a known cargo-ecosystem behavior where `rust-version` metadata in the registry index can be tightened post-publish.

### Evidence forcing the pin lowering

- `cargo build -p cacg-core` under Rust 1.85.0 errored: `time@0.3.47 requires rustc 1.88.0` and `time-core@0.1.8 requires rustc 1.88.0`. `time-core 0.1.8` is the transitive `=0.1.8` requirement of `time 0.3.47`.
- Same probe surfaced `criterion@0.8.2 requires rustc 1.86` when the test profile (which pulls in dev-deps) was compiled.
- The `~6-month stable window` argument used in PE-01 (1.80 → 1.85) still applies: 1.85.0 (Feb 2025) has had ~15 months of stable adoption as of 2026-05-20; bumping further to 1.88 (likely May 2025) would re-trigger the toolchain-availability tradeoff PE-01 weighed against.
- Older versions of both crates still resolve cleanly on 1.85.0 and carry no other functional differences relevant to the parity contract:
  - `time 0.3.41` (depends on `time-core 0.1.4`, `num-conv 0.1.0`, `deranged 0.4.0`): same `formatting` + `parsing` features.
  - `criterion 0.5.1` (depends on `criterion-plot 0.5.0`, `itertools 0.10.5`): same benchmark surface used by M6 perf gates (per `_research/07` §3 dev-deps section).

### Resolution

`Cargo.toml` workspace deps updated:
- `time = "0.3.47"` → `time = "0.3.41"` (and `Cargo.lock` downgraded via `cargo update -p time --precise 0.3.41`).
- `criterion = "0.8.2"` → `criterion = "0.5"` (and `Cargo.lock` downgraded via `cargo update -p criterion --precise 0.5.1`).

MSRV stays at 1.85 (no plan amendment needed). The narrower workspace-dep version constraints are documented here so a future operator pinning `time = "0.3.47"` would also need to bump MSRV simultaneously per cargo's enforcement.

### Tradeoff

- Cost: drops the marginal feature additions of `time 0.3.42..0.3.47` and `criterion 0.6..0.8` (mostly internal refactors per their changelogs; none touch the surfaces cacg-core uses).
- Benefit: MSRV stays at 1.85 — the toolchain the rest of the build / CI matrix / CI runners are pinned to. No cascade re-evaluation of every workspace dep against 1.88's MSRV declarations.

### User override

Edit this PE-05 entry's `Status:` line to `OVERRIDDEN: <NEW-MSRV>` and supply rationale. Bumping MSRV to 1.88 unlocks the newer `time` + `criterion` pins but requires updating `rust-toolchain.toml`, the GH Actions toolchain matrix in `.github/workflows/{parity,build-macos,_validate-workflows}.yml`, and re-running PE-01's compatibility scan on every dep.

### Addendum (Round 10): exact pins replace caret ranges

Codex Round-9 review observed that the original PE-05 declared `time = "0.3.41"` and `criterion = "0.5"` as caret-range constraints, which means a future `cargo update` (without `--precise`) could re-resolve to a newer 0.3.x / 0.5.x patch release whose `rust-version` metadata may again exceed MSRV 1.85. The lockfile happens to pin the working versions today, but the workspace constraint itself does not enforce the MSRV claim.

Round 10 tightens both pins to exact versions:

- `time = "=0.3.41"` (exact)
- `criterion = "=0.5.1"` (exact)

With exact pins, `cargo update` cannot regenerate the lockfile to a newer patch release without an explicit workspace-dep edit. The PE-05 "MSRV 1.85 stays load-bearing" rationale is now structurally enforced, not just lockfile-incidentally enforced.

If a future maintainer needs to relax this (e.g., to pick up a security patch within the 0.3.x series), the path is: (a) verify the candidate version's declared `rust-version` is `<= 1.85`, (b) update the exact pin to the new candidate, (c) regenerate `Cargo.lock` and confirm `cargo build` still succeeds on the pinned toolchain.

---

## References

- Plan: `.humanize/.humanize/plans/cacg-rust-port-trust-kernel-first-plan.md` §Pending User Decisions (DEC-1..DEC-8 originally listed there) + §Path Boundaries Allowed Choices (where the MSRV is documented).
- Goal tracker: `.humanize/rlcr/2026-05-20_16-15-57/goal-tracker.md` (references this file for current status).
- Canonical JSON spec (DEC-1 application): `_research/08_canonical_json_spec.md`.
- Diagnostic parity contract (DEC-3 application): `docs/diagnostic-parity.md`.
