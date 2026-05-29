# M5b — Layer-3 Semantic Verifier + Round-Summary Native Port + B2 LLM-Judge Opt-In: Completion Summary

_captured against HEAD `29dccce` on `2026-05-25`, revised at HEAD `e9babd5` to close BS-20_

This document closes **task-m5b-34** and is the M5b exit artifact for the analyze route. M5b covered four sub-milestones: **M5b-a** ported `kb verify --round-summary` natively; **M5b-b** ported the Layer-3 semantic verifier runtime and single-card `--semantic` path; **M5b-c** provisioned the QM B1 semantic cache, provenance, calibration sweep, and `xtask semantic-eval` gate; **M5b-d** added the opt-in B2 LLM-judge feature and the locked end-to-end transcript proof. This file records the acceptance-criterion end-state against committed evidence, not paraphrased intent: paths, test names, parity rows, gate summaries, and known residual risk status are recorded as the source of truth.

## AC verification checklist

### AC-1 — B1 semantic-cache runtime

**Status: MET.**

The committed runtime lives in `crates/cacg-semantic/src/lib.rs`. Round 8 rewrote the crate around `SemanticCache`, `SemanticVerdict`, `SemanticCacheEntry`, and `SemanticError`; Round 9 added a pre-typed duplicate-key scan; Round 10 moved shared semantic types into the core verifier spec so both runtime and callers use the same contract.

| evidence | attestation |
|----------|-------------|
| `crates/cacg-semantic/src/lib.rs` | B1 cache runtime owns cache loading, structural validation, canonical JSON output, and cache entry validation. |
| `crates/cacg-core/src/verify/semantic_spec.rs` | `SemanticVerdictKind`, `SemanticMode`, `SemanticVerdict`, `SemanticEvaluator`, and `claim_window_hash` moved into `cacg-core` in Round 10. |
| `crates/cacg-semantic/tests/committed_cache.rs` | 5 integration tests load committed `out/semantic_cache.json` with 227 entries and round-trip byte-equal. |
| `cargo test -p cacg-semantic` | 44 default lib tests + 5 integration tests = 49/49. |
| `cargo test -p cacg-semantic --features b2-llm-judge` | 62 feature-on lib tests + 5 integration tests = 67/67. |

The runtime rejects malformed cache documents before they can become trusted semantic evidence. Custom `serde` `Deserialize` implementations reject duplicate JSON keys through a `BTreeSet<String>` of seen keys. The Round 9 `DuplicateKeyChecker` `Visitor` plus `DeserializeSeed` walks every JSON object depth before typed deserialization, closing the residual duplicate-key risk rather than relying only on typed struct boundaries. `SemanticCache::to_canonical_json` goes through `cacg_core::canonical_json::canonical_json`, so committed cache round-trips are checked against the same canonicalization substrate used elsewhere.

The structural validator enforces the literal schema version, 64-lowercase-hex hashes, finite scores in `[0,1]`, and cross-entry semantic-key uniqueness. The committed B1 cache is not only loadable; it is tested as an artifact. `crates/cacg-semantic/tests/committed_cache.rs` pins the 227-entry cache and byte-equal round-trip behavior.

### AC-1.1 — Schema-version evolution

**Status: MET.**

The schema-version contract is explicit in `crates/cacg-semantic/src/lib.rs`: the accepted literal is `cacg.v0`. Unknown schema versions produce `SemanticError::UnknownSchemaVersion` with distinct reason text, while unknown fields under the valid schema return a JSON error with strict-schema reason text.

| evidence | attestation |
|----------|-------------|
| `crates/cacg-semantic/src/lib.rs` | Schema version literal is `cacg.v0`; unknown schema version and unknown-field-under-valid-schema have separate error paths. |
| `crates/cacg-semantic/src/lib.rs` unit tests | Reason strings are asserted demonstrably distinct with `assert_ne!`. |

This means future schema evolution has a separate failure mode from simple strict-schema rejection. A consumer seeing an unknown version can distinguish “this cache is from a newer schema” from “this cache has a field that should never have been accepted under the current schema.”

### AC-2 — `kb verify --semantic` single-card

**Status: MET.**

The single-card Layer-3 verifier is wired through the Rust verification pipeline and the CLI dispatcher. Layer-3 firing logic lives in `crates/cacg-core/src/verify/layer2.rs`; CLI evaluator construction and pre-pipeline cache validation live in `crates/cacg-cli/src/main.rs`.

| evidence | attestation |
|----------|-------------|
| `crates/cacg-core/src/verify/layer2.rs` | Layer-3 firing logic, AUTH-suppression mask, HASH-001 short-circuit behavior, and per-verdict severity contract are committed. |
| `crates/cacg-cli/src/main.rs::build_semantic_evaluator` | Builds semantic evaluator for CLI `--semantic` use. |
| `crates/cacg-cli/src/main.rs::dispatch_verify` | Emits `CACG-MAN-001` on missing, directory, or malformed cache before the pipeline starts. |
| clap-level mutex | `--semantic` versus `--semantic-judge` exits at clap level with exit code 2. |
| xtask parity row `kb_verify_semantic_parity_golden` | Golden single-card semantic parity row. |
| xtask parity row `kb_verify_semantic_miss_parity_golden` | Cache-miss abstain parity row added in Round 13. |
| `tests/test_phase3_semantic_verifier.py` | Python parity suite: 30/30. |

Round 11 added the AUTH-suppression mask in `crates/cacg-core/src/verify/layer2.rs` so Layer-3 does not run behind AUTH-000/001/002 short-circuits. Six `layer2::tests` pin AUTH-000/001/002 suppression, HASH-001 short-circuit behavior, and fuzzy-accept non-invocation. Round 13 then locked the per-verdict severity contract: `pass` maps to warning, `fail` maps to error, and `abstain` maps to info. Python parity was updated in `src/cacg/verify/layer2.py` to the same severity map.

The single-card CLI path also has failure-mode coverage. Missing, directory, and malformed cache cases are rejected as `CACG-MAN-001` before verification begins, and the feature-gated B2 judge path is mutually exclusive with cache-backed `--semantic`.

### AC-3 — `kb verify --round-summary` native port

**Status: MET.**

The native round-summary verifier is committed in `crates/cacg-cli/src/round_summary.rs`. It replaces the prior CLI stub through `crates/cacg-cli/src/main.rs::dispatch_verify_round_summary`, specifically replacing the `CACG-CLI-002` stub at `main.rs:213-215`.

| evidence | attestation |
|----------|-------------|
| `crates/cacg-cli/src/round_summary.rs` | 679+ lines covering parser, `verify_round_summary`, and `VerifyRoundSummaryError`. |
| `crates/cacg-cli/src/main.rs::dispatch_verify_round_summary` | Native dispatcher replaces the former `CACG-CLI-002` stub at `main.rs:213-215`. |
| `src/cacg/cli.py::_cmd_verify_round_summary` | Python oracle at lines 973-1058. |
| xtask parity row `kb_verify_round_summary_parity_golden` | Runs 9 fixtures and 36 artifact comparisons. |
| `crates/cacg-cli/tests/kb_verify_round_summary.rs` | 12 integration tests. |

The Rust implementation emits `CACG-RS-001`, `CACG-RS-002`, `CACG-RS-003`, and `CACG-RS-004` byte-equal with the Python `_cmd_verify_round_summary` implementation. The parity row runs the native command over the committed fixture corpus and compares the canonical stdout/stderr/exit-code envelopes.

This AC is therefore not just “ported”; it is pinned against the previous Python behavior. The dispatcher now exercises Rust code directly, and the parity matrix holds the command output contract.

### AC-3.1 — Golden parity fixtures

**Status: MET.**

The golden fixture set is committed under `tests/round_summary_fixtures/`: 9 markdown fixtures and 9 paired `*.expected.json` files. Each expected artifact uses the canonical envelope:

```json
{
  "exit_code": "...",
  "fixture_sha256": "...",
  "stderr": "...",
  "stdout": "..."
}
```

The envelope is canonicalized with sorted keys and two-space indentation.

| evidence | attestation |
|----------|-------------|
| `tests/round_summary_fixtures/` | 9 markdown fixtures and 9 paired expected JSON artifacts. |
| `crates/cacg-cli/tests/round_summary_fixtures_parity.rs` | 11 tests: 8 per-fixture parity, directory completeness, schema validity, and fingerprint-mismatch panic. |
| `cacg_core::hash::source_sha256(&bytes)` | Fixture fingerprint computed before running `kb`. |
| Round 4 BS-1 closure | Fingerprint mismatch fails loudly. |

The fingerprint mechanism closes the fixture-drift hole: `source_sha256(&bytes)` is computed before execution, and a mismatch fails loudly rather than allowing an expected JSON artifact to silently describe a different markdown input. The directory-completeness test constrains the fixture count to `5 <= count <= 10`, which keeps the fixture set intentionally bounded while still preventing accidental loss of coverage.

### AC-3.2 — Parser fuzz / edge

**Status: MET.**

Parser edge coverage is committed both as unit tests around the parser and as integration tests around the command behavior.

| evidence | attestation |
|----------|-------------|
| `crates/cacg-cli/src/round_summary.rs::tests` | 52 unit tests cover bullet shapes, CRLF, Windows separators, unquoted spaces, backticks, and parentheses. |
| `crates/cacg-cli/tests/kb_verify_round_summary.rs` | Integration tests cover empty markdown, duplicate paths, and a 10,001-bullet very-large summary under a `<120s` wall-clock expectation. |
| `tests/round_summary_fixtures/09_multiple_sections.md` | Exercises Python parity for first-`Knowledge Consulted`-section-wins behavior. |

The parser’s edge posture is therefore stronger than only golden-path parity. It covers common markdown shape drift, platform path separators, quoting variants, duplicate path handling, empty input, and large input. The `09_multiple_sections.md` fixture explicitly pins the “first Knowledge Consulted section wins” behavior against Python parity.

### AC-4 — `kb verify --round-summary --semantic` batch

**Status: MET.**

Round-summary semantic mode threads a semantic evaluator through every cited card verification. The implementation path is `crates/cacg-cli/src/round_summary.rs::verify_round_summary`, where `Option<&dyn SemanticEvaluator>` is passed into each per-citation `verify_one_card` call.

| evidence | attestation |
|----------|-------------|
| `crates/cacg-cli/src/round_summary.rs::verify_round_summary` | Threads `Option<&dyn SemanticEvaluator>` through each per-cite `verify_one_card`. |
| `crates/cacg-semantic/src/lib.rs::emit_load_trace` | `CACG_SEMANTIC_LOAD_TRACE` emits `load_ok <path> <count>` on success and `load_err <path>` on failure. |
| `crates/cacg-cli/tests/kb_verify_round_summary_semantic.rs::round_summary_semantic_valid_cache_loads_exactly_once` | Subprocess 4-cite batch asserts exactly one `load_ok` line. |
| `crates/cacg-cli/src/round_summary.rs::tests::round_summary_threads_evaluator_through_each_cite` | Synthetic `CountingEvaluator` with `AtomicUsize`: card-A 3x + card-B 1x yields counter `== 4`. |
| xtask parity row `kb_verify_round_summary_semantic_parity_golden` | Semantic round-summary parity row. |
| `tests/parity_corpus/round_summary_semantic/summary.md` | Committed semantic round-summary fixture. |

The batch path has two distinct proofs. The in-process `CountingEvaluator` proves evaluator invocation is per citation, not per distinct card or per summary. The subprocess load-trace test proves the cache is loaded once for a 4-citation batch, preventing per-citation reload behavior while still invoking semantic evaluation four times.

The committed load trace is intentionally narrow: `load_ok <path> <count>` on success and `load_err <path>` on failure. That gives the test a stable observable without exposing timing, hashes, or unrelated runtime state.

### AC-5 — B2 LLM-judge opt-in feature

**Status: MET.**

The B2 judge is feature-gated and opt-in. The production client, mock client, async bridge, CLI flag, default-build rejection behavior, dependency audit, and wire-level tests are all committed.

| evidence | attestation |
|----------|-------------|
| `crates/cacg-semantic/src/b2.rs` | Defines `LlmJudgeClient`, `JudgeError`, `HaikuClient`, and `MockJudgeClient`. |
| `crates/cacg-cli/Cargo.toml` and `crates/cacg-semantic/Cargo.toml` | `reqwest`, `tokio`, and `async-trait` are optional under `b2-llm-judge`. |
| `crates/cacg-cli/src/main.rs::build_semantic_evaluator` | Under the feature, `--semantic-judge` constructs `B2Evaluator::new(Arc::new(HaikuClient::with_components_using_env_key()))`. |
| `crates/cacg-cli/src/lib.rs` | clap `semantic_judge` field is gated with `#[cfg(feature = b2-llm-judge)]`; default build argparse-rejects `--semantic-judge`. |
| `xtask/src/audit_default_kb_deps.rs` | Default dependency audit bans `tokio`, `reqwest`, `tch`, `ort`, `onnxruntime-rs`, `sentence-transformers-rs`, and `candle-*`. |
| `crates/cacg-cli/tests/kb_verify_semantic_b2_mocked.rs` | 15 tests. |
| `crates/cacg-cli/tests/kb_verify_semantic_b2_wiremock.rs` | 2 tests. |

The feature-on implementation bridges async through a current-thread Tokio runtime, but the default build remains free of the async/network dependency stack. The audit `xtask audit-default-kb-deps` walks `cargo metadata --format-version=1` for the default closure and has 14 unit tests in `xtask/src/audit_default_kb_deps.rs`.

`crates/cacg-semantic/src/b2.rs` has 18 unit tests under `--features b2-llm-judge`. The mocked CLI tests cover panicking-mock default-path proof, malformed response subclasses, lazy missing-key diagnostics, and batch panicking-mock behavior through round-summary verification. The wiremock tests include canary non-leakage on HTTP 500 and `AnthropicRequestBodyMatcher` assertions for model, `max_tokens`, `messages[0].role`, and content substrings including quote and chunk text.

### AC-6 — QM B1 cache + builder + provenance + ceremony + idempotence

**Status: MET.**

The QM B1 cache is committed as `out/semantic_cache.json` with 227 entries: 222 paraphrase entries and 5 negative fixtures. The provenance sidecar is committed as `out/semantic_cache.provenance.json`.

| evidence | attestation |
|----------|-------------|
| `out/semantic_cache.json` | 227 entries: 222 paraphrase + 5 negative fixtures. |
| `out/semantic_cache.provenance.json` | Hash B and Hash C provenance with strict 5-field schema and deny-unknown-fields behavior. |
| `scripts/build_semantic_cache.py` | Builder constants: `EXPECTED_PARAPHRASE_COUNT=222`, `EXPECTED_NEGATIVE_FIXTURE_COUNT=5`, `EXPECTED_ENTRY_COUNT=227`, `DEFAULT_THRESHOLD`; refuses non-canonical environment unless `--force-non-canonical`. |
| `_research/20_b1_cache_provisioning.md` | Rebuild ceremony, HF revision SHA `c9745ed1d9f207416be6d2e6f8de32d1f16199bf`, and `sentence-transformers==5.5.1` pin. |
| `tests/test_semantic_cache_provisioning.py` | 6 passed + 1 skipped; idempotence test gated behind `--build-against-canonical-env`. |
| `xtask audit-semantic-cache-provenance` | PASS: `227 entries clean (222 paraphrase + 5 negative)`. |
| `crates/cacg-semantic/tests/committed_cache.rs` | 5 committed-cache tests. |

Hash B is the 5-field length-prefixed environment/build hash: raw `uv.lock` bytes, `os.uname().machine`, `os.uname().release`, Python version, and `sentence_transformers.__version__`. Hash C is the SHA-256 of the canonical-JSON cache bytes. The provenance schema is strict and has deny-unknown-fields behavior.

The builder refuses non-canonical environments unless explicitly forced, and the research ceremony records the model revision and sentence-transformers pin. The xtask provenance audit has 9 unit tests for negative gates: mutation, `uv.lock` drift, count drift, missing field, unknown field, and wrong `model_revision`.

Gate evidence:

> `xtask audit-semantic-cache-provenance: PASS '227 entries clean (222 paraphrase + 5 negative)'`

The test-count evidence at HEAD also records:

> `pytest tests/test_semantic_cache_provisioning.py: 6 passed + 1 skipped`

### AC-7 — MiniLM threshold calibration sweep

**Status: MET.**

The calibration sweep is committed in `xtask/src/threshold_sweep.rs`. It produces `SweepRows` with pass/fail/abstain partitions and asserts the invariant `pass + fail + abstain == total_entries`.

| evidence | attestation |
|----------|-------------|
| `xtask/src/threshold_sweep.rs` | Produces sweep rows and asserts partition arithmetic. |
| `cargo xtask threshold-sweep --vertical qm` | Emits 60 rows with `abstain=0`. |
| `_research/qm_layer3_threshold_sweep.md` | Documents locked threshold `0.5`. |
| `xtask/src/semantic_cache_provenance.rs::EXPECTED_THRESHOLD` | Cross-checked against builder `DEFAULT_THRESHOLD`. |
| `scripts/build_semantic_cache.py::DEFAULT_THRESHOLD` | Matched by `builder_default_threshold_matches_expected` unit test. |
| `--provenance` preflight | Defaults to `out/semantic_cache.provenance.json` and checks strict `ProvenanceShape` before row emission. |

The sweep’s abstain result is expected for the committed B1 cache: B1 cache hits never abstain; abstain occurs only on miss. The frozen-label preflight checks the provenance shape, paraphrase count `== 222`, negative fixture count in `[5,10]`, entry-count arithmetic, and `cache.entries.len()` agreement before emitting rows.

The locked threshold is `0.5`, and Round 21 added the cross-check that `xtask/src/semantic_cache_provenance.rs::EXPECTED_THRESHOLD` and `scripts/build_semantic_cache.py::DEFAULT_THRESHOLD` stay aligned.

### AC-8 — `xtask semantic-eval` Layer-3 gate

**Status: MET.**

The Layer-3 semantic evaluation gate is committed in `xtask/src/semantic_eval.rs`. It runs verdict classification on the QM B1 cache plus frozen labels and emits a canonical report format:

```text
<MARKER>  <name:<32>  expected=<x> actual=<y>
```

| evidence | attestation |
|----------|-------------|
| `xtask/src/semantic_eval.rs` | Implements `cargo xtask semantic-eval` Layer-3 verdict classification. |
| `tests/semantic_eval/eval_cases.json` | 3 cases: pass, fail, abstain, with schema version `cacg.v0.semantic-eval1`. |
| `#[serde(deny_unknown_fields)]` | Applied on both semantic-eval and retrieval-eval schemas. |
| cross-rejection tests | Pin semantic-eval and retrieval-eval schema separation. |
| `cargo xtask semantic-eval` | Reports 3/3 cases match expected verdict. |

The fixture schema is intentionally distinct from retrieval-eval. Both schemas use `deny_unknown_fields`, and cross-rejection tests ensure the two fixture families do not accidentally accept one another. The gate output is small and deterministic: three cases, one each for pass, fail, and abstain, all matching expected verdicts.

Gate evidence:

> `xtask semantic-eval: 3/3 cases match`

### AC-9 — Cross-cutting determinism + boundary gate

**Status: MET.**

The workspace-level `#![forbid(unsafe_code)]` posture remains as expected:

| crate / target | evidence |
|----------------|----------|
| `cacg-core` | `crates/cacg-core/src/lib.rs:11` |
| `cacg-cli` | `crates/cacg-cli/src/lib.rs:16` |
| `cacg-search` | `crates/cacg-search/src/lib.rs:10` |
| `cacg-semantic` | `crates/cacg-semantic/src/lib.rs:33` |
| `cacg-ingest` | carve-out preserved |
| `xtask` | `xtask/src/main.rs` top of file |

The static gates after the Round-41 BS-20 closure all hold:

| gate | result |
|------|--------|
| `lint-determinism` | 0 violations under `crates`, `xtask/src`. |
| `lint-trust-leak` | 0 violations under `crates/cacg-cli/src`. |
| `lint-platform-cfg` | 0 violations under `crates/cacg-core/src`. |
| `lint-rename-outside-publisher` | 0 violations under `crates`, `xtask/src`. |
| `lint-runner-bypass` | 0 violations under `crates/cacg-cli/src`. |
| `audit-cacg-core-deps` | 0 forbidden packages in `cacg-core`'s resolved dependency closure. |
| `audit-default-kb-deps` | 0 forbidden packages in `cacg-cli` default-features closure. |

The new `audit-default-kb-deps` gate walks `cargo metadata --format-version=1` for the default `cacg-cli` closure and bans `tokio`, `reqwest`, `tch`, `ort`, `onnxruntime-rs`, `sentence-transformers-rs`, and `candle-*`; it has 14 unit tests in `xtask/src/audit_default_kb_deps.rs`.

**BS-20 closure (Round 41).** The initial Round-40 artifact recorded AC-9 as PARTIAL because `cargo xtask lint-determinism` reported 2 violations: `xtask/src/semantic_eval.rs:262` and `:340` were calling `tempfile::NamedTempFile::new()` inside `#[cfg(test)]` inline tests. The determinism lint exempts only paths containing `/tests/` (filesystem path component), not in-source `#[cfg(test)]` blocks; the violations were introduced in Round 25 commit `3165110` and never surfaced in per-round Codex review checklists. Round 41 closed BS-20 by rewriting both call sites to use `tempfile::TempDir::new()` plus deterministic child filenames (`wrong-schema.json` and `synthetic-semantic-eval.json`). The `TempDir` binding is kept alive for the full test scope so the temporary tree is cleaned at drop. After the fix, `cargo xtask lint-determinism` reports 0 violations and both repaired tests pass under `cargo test -p xtask semantic_eval`.

Additional cross-cutting gate evidence:

> `xtask parity` with `KB_FROZEN_CLOCK=1`: 36 entries, 16 passed, 0 failed, 20 future-stage.

> Provenance audit: PASS, 227 entries clean, `hash_b=57e997076ac8...`, `hash_c=a905e4222404...`.

> `lint-workflow-labels`: 0 violations across 6 files.

> `xtask semantic-eval`: 3/3 cases match.

> `xtask retrieval-eval`: 11/11 cases match, 10/10 expected-hit-at-3.

AC-9 is therefore MET: every static gate in the AC-9 invariant set is green, the dependency firewall is in place, and the parity matrix covers the new semantic and round-summary rows.

### AC-9.1 — End-to-end locked transcript

**Status: MET.**

The locked transcript proof is committed in `crates/cacg-cli/tests/round_summary_semantic_e2e.rs::round_summary_semantic_e2e_locked_transcript`. Round 38 introduced the end-to-end test; Round 39 repaired it to byte-equal lock behavior.

| evidence | attestation |
|----------|-------------|
| `crates/cacg-cli/tests/round_summary_semantic_e2e.rs::round_summary_semantic_e2e_locked_transcript` | End-to-end round-summary semantic locked transcript. |
| `normalize_paths(text,tmp,workspace)` | Replaces tempdir with `<TMP>` and workspace root with `<WORKSPACE>`. |
| `project_journal_event` | Keeps `card_path`, `verification`, and `diagnostics`; excludes `card_hash_*`, `latency_ms`, `timestamp`, and `command`. |
| `assert_eq!(actual_transcript, expected_transcript)` | Primary lock. |
| `CACG_SEMANTIC_LOAD_TRACE` | Pinned to exactly one `load_ok <TMP>/locked_semantic_cache.json 1` line. |

The expected transcript pins `exit_code=1`, exactly one stderr `STALE` line, alphabetized journal keys, BM25 hint scores `-0.167814` and `-0.181373`, `CACG-VERIFY-001` and `CACG-VERIFY-002` in the same diagnostics array, and the verification dictionary covering fuzzy, layer1, and layer2.

The transcript intentionally normalizes temp paths and workspace paths, and intentionally excludes unstable journal fields. That keeps the lock byte-equal without pretending runtime-specific values are stable.

## Static gate end-state

The static gate end-state after the Round-41 BS-20 closure is fully green across every M5b-relevant gate.

| gate | result | AC impact |
|------|--------|-----------|
| `lint-determinism` | 0 violations under `crates`, `xtask/src` | AC-9 MET. |
| `lint-trust-leak` | 0 violations under `crates/cacg-cli/src` | AC-9 supporting evidence. |
| `lint-platform-cfg` | 0 violations under `crates/cacg-core/src` | AC-9 supporting evidence. |
| `lint-rename-outside-publisher` | 0 violations under `crates`, `xtask/src` | AC-9 supporting evidence. |
| `lint-runner-bypass` | 0 violations under `crates/cacg-cli/src` | AC-9 supporting evidence. |
| `audit-cacg-core-deps` | 0 forbidden packages in `cacg-core`'s resolved dependency closure | AC-9 supporting evidence. |
| `audit-default-kb-deps` | 0 forbidden packages in `cacg-cli` default-features closure | AC-5 and AC-9 supporting evidence. |
| `lint-workflow-labels` | 0 violations across 6 files | M5b ceremony hygiene evidence. |
| `audit-semantic-cache-provenance` | PASS, 227 entries clean | AC-6 evidence. |
| `semantic-eval` | 3/3 cases match | AC-8 evidence. |
| `retrieval-eval` | 11/11 cases match, 10/10 expected-hit-at-3 | Cross-milestone retrieval fixture evidence. |

After the Round-41 BS-20 closure, every M5b gate in the table above is green. The M5b end-state is **13/13 ACs MET**.

## Parity matrix end-state

The parity matrix is green for the M5b rows that are in scope at HEAD.

| parity row / matrix evidence | result |
|------------------------------|--------|
| `kb_verify_round_summary_parity_golden` | 9 fixtures, 36 artifact comparisons. |
| `kb_verify_semantic_parity_golden` | Single-card semantic golden parity. |
| `kb_verify_semantic_miss_parity_golden` | Cache-miss abstain parity. |
| `kb_verify_round_summary_semantic_parity_golden` | Batch round-summary semantic parity. |
| `xtask parity` with `KB_FROZEN_CLOCK=1` | 36 entries, 16 passed, 0 failed, 20 future-stage. |

The 20 future-stage rows are the M3 help snapshots and M4 pdfium-provisioning rows. The M5b semantic and round-summary parity rows are not failing.

## Residual risks (R1–R5) status

| risk | status | evidence / note |
|------|--------|-----------------|
| R1 — AC-1 duplicate-key detection | CLOSED | Round 9 `DuplicateKeyChecker` `Visitor` walks every JSON object depth before typed deserialization. |
| R2 — DEC-1 provenance Hash B canonicalization | CLOSED | `xtask/src/semantic_cache_provenance.rs` recomputes Hash B with explicit 5-field length-prefixed concatenation: raw `uv.lock` bytes, `os.uname().machine`, `os.uname().release`, Python version, and `sentence_transformers.__version__`. Mutation tests fail the audit. |
| R3 — AC-9 audit interpretation | CLOSED | `xtask audit-default-kb-deps` inspects `cargo metadata --format-version=1` without `--all-features` overrides and walks only `cacg-cli`'s resolved default closure. |
| R4 — Model-weight tamper detection | USER-ACCEPTED trade-off | Per-file model artifact SHA-256s are deliberately omitted; identity is anchored at the HF revision SHA. A local model-weight tamper between download and cache-build would not be caught by the provenance file alone. Future tightening would escalate to a three-hash contract. |
| R5 — MiniLM weakness on formula/table-heavy CFA citations | NOT MITIGATED, by design | AC-7 pass-rate is aspirational. M5b ships whatever the locked threshold achieves. Future model swap candidates include BGE-small or jina-embeddings-v2-base-en. |

R1-R3 are closed, R4 is an accepted provenance trade-off, and R5 remains a known model-quality limitation by design.

## M5b phase end-state (13/13 ACs MET)

M5b is fully complete: the Layer-3 semantic verifier port, the round-summary native port, the QM B1 cache/provenance/calibration path, the B2 LLM-judge opt-in feature, the locked end-to-end semantic transcript, and the cross-cutting determinism + boundary gate are all in place.

| AC | status | summary |
|----|--------|---------|
| AC-1 | MET | B1 semantic-cache runtime committed, structurally validated, duplicate-key checked before typed deserialization, canonical JSON round-tripped. |
| AC-1.1 | MET | Schema version `cacg.v0` has distinct unknown-version versus unknown-field failure modes. |
| AC-2 | MET | `kb verify --semantic` single-card path wired through Rust with parity, AUTH suppression, severity contract, and cache-miss abstain behavior. |
| AC-3 | MET | `kb verify --round-summary` native port replaces the CLI stub and matches Python oracle behavior. |
| AC-3.1 | MET | Golden round-summary fixtures and fingerprinted expected envelopes committed. |
| AC-3.2 | MET | Parser fuzz and edge suite covers markdown/path/input-shape edge cases. |
| AC-4 | MET | Round-summary semantic batch path invokes evaluator per citation and loads cache once per batch. |
| AC-5 | MET | B2 LLM-judge is opt-in, feature-gated, mocked, wire-tested, and absent from the default dependency closure. |
| AC-6 | MET | QM B1 cache, provenance sidecar, builder ceremony, audit gate, and idempotence posture are committed. |
| AC-7 | MET | Threshold sweep emits partitioned rows, locks threshold `0.5`, and preflights frozen provenance. |
| AC-8 | MET | `xtask semantic-eval` classifies pass/fail/abstain fixtures and reports 3/3 expected matches. |
| AC-9 | MET | All six baseline static gates plus the new `audit-default-kb-deps` are green; the Round-41 BS-20 closure brought `lint-determinism` back to 0 violations. |
| AC-9.1 | MET | End-to-end locked transcript is byte-equal with normalized paths and exactly one semantic cache load trace. |

Test-count evidence at HEAD `29dccce`:

| command / suite | result |
|-----------------|--------|
| `cargo test -p cacg-core --lib` | 279/279 |
| `cargo test -p cacg-semantic` | 44 + 5 = 49/49 |
| `cargo test -p cacg-semantic --features b2-llm-judge` | 62 + 5 = 67/67 |
| `cargo test -p cacg-cli` | 154 total across 21 result groups |
| `cargo test -p cacg-cli --features b2-llm-judge` | 173 total across 21 result groups |
| `cargo test -p xtask` | 223/223 |
| `pytest tests/test_semantic_cache_provisioning.py` | 6 passed + 1 skipped |

After the Round-41 BS-20 closure, every original-plan acceptance criterion is MET, including AC-9's cross-cutting determinism + boundary gate.

## Round-by-round commit lineage

| round | commit | content |
|-------|--------|---------|
| 0 | `6f8b4da` | M5b kickoff — round-summary parser spec (Codex audit). |
| 1 | `3ff0258` | M5b-a parser port — `round_summary.rs` parser library. |
| 2 | `2ddd1ca` | M5b-a Phase C — `kb verify --round-summary` native dispatcher. |
| 3 | `5b7b60b` | M5b-a Phase D — round-summary golden parity fixtures. |
| 4 | `76113e8` | Close AC-3.1 negative gate — `fixture_sha256` fingerprint. |
| 5 | `d18abb6` | M5b-a Phase E — full AC-3.2 parser/dispatcher edge suite. |
| 6 | `80c33d1` | M5b-a Phase F — xtask parity row for round-summary. |
| 7 | `6922f3b` | M5b-b Phase A — Codex audit of `semantic.py` spec doc. |
| 8 | `a9f5db8` | M5b-b Phase A — port `cacg-semantic` B1 runtime. |
| 8 | `ea7d93d` | Round 8 lockfile sync. |
| 9 | `197c7ac` | Close BS-2 + `task-m5b-9` + `task-m5b-10`. |
| 10 | `bd70cc8` | Wire Layer-3 semantic cache end-to-end through `kb verify`. |
| 11 | `7d78b38` | Suppress Layer-3 on AUTH short-circuits (BS-3). |
| 12 | `13ecd56` | Single-card `--semantic` byte-equal parity row. |
| 13 | `c1f781c` | Per-verdict severity lock + QS-7 abstain alignment. |
| 14 | `e8bec0e` | Rewrite Round-13 workflow-label fragments in domain language. |
| 15 | `94e4075` | BS-5 hygiene closure + QS-10 doc fold-in. |
| 16 | `cfead67` | QM B1 semantic cache + builder + provenance + ceremony. |
| 17 | `3cdfd0f` | Close BS-6: AC-6 audit xtask + frozen-count gates + Rust real-cache test. |
| 18 | `707cd9c` | BS-7 closure. |
| 19 | `1287019` | BS-8 closure + xtask `lint-workflow-labels` static gate. |
| 20 | `174ad9e` | `task-m5b-19/20` — threshold sweep + locked threshold + research doc. |
| 21 | `0ff5fe6` | BS-9 closure: AC-7 sweep contract completion. |
| 22 | `57fe8a1` | `task-m5b-21` — xtask `audit-default-kb-deps` gate. |
| 23 | `3b876f0` | `task-m5b-22/23/24` — AC-4 batch semantic wiring proof artifacts. |
| 24 | `eae3b5e` | BS-10 closure: `task-m5b-23` AC-4 instrumented contract. |
| 25 | `3165110` | `task-m5b-25/26` — xtask semantic-eval + decoupled fixture schema. |
| 26 | `e717c54` | BS-11 closure: `task-m5b-25` AC-8 report format. |
| 27 | `9bbe8a4` | `task-m5b-27` — `b2-llm-judge` feature scaffolding. |
| 28 | `7828b22` | BS-12 closure: `task-m5b-27` doc-comment hygiene. |
| 29 | `673f751` | Close BS-13: `cacg-semantic` load-trace test isolation. |
| 30 | `72d9b9e` | Port B2 LLM-judge client surface. |
| 31 | `e6c3615` | Close BS-14. |
| 32 | `fa77c8f` | Wire B2 LLM-judge end-to-end: `task-m5b-29` + BS-15 closure. |
| 33 | `b6da97c` | Close BS-16 + land `task-m5b-30`. |
| 34 | `d5cdda7` | Close BS-17: round-summary panicking-mock proof. |
| 35 | `ae95ed8` | `task-m5b-31`: comprehensive B2 diagnostic + secret-non-leakage tests. |
| 36 | `93f535a` | BS-18 closure + `task-m5b-32`. |
| 37 | `678208d` | Close BS-19: wiremock body assertions + `b2.rs` stale comment. |
| 38 | `e380e41` | `task-m5b-33`: AC-9.1 locked transcript e2e initial. |
| 39 | `29dccce` | `task-m5b-33`: AC-9.1 byte-equal locked transcript. |
| 40 | `e834da6` | `task-m5b-34`: M5b exit artifact, including BS-20 AC-9 partial finding. |
| 41 | `e9babd5` | BS-20 closure: `xtask/src/semantic_eval.rs` `TempDir` + deterministic child filenames; AC-9 now MET; M5b exit attestation revised to `13/13 ACs MET`. |

## Hand-off to M5+ / multi-vertical migration

The next milestone inherits the following fixed ground from M5b:

1. **Layer-3 has a shared Rust contract.** `SemanticVerdictKind`, `SemanticMode`, `SemanticVerdict`, `SemanticEvaluator`, and `claim_window_hash` live in `crates/cacg-core/src/verify/semantic_spec.rs`, with `cacg-semantic` and verifier callers using the same spec.

2. **The B1 cache format is strict and canonical.** `cacg.v0` schema handling, duplicate-key rejection before typed deserialization, strict structural validation, canonical JSON output, committed-cache round-trip tests, and provenance audit gates are all in place.

3. **The QM cache is a reproducible baseline.** `out/semantic_cache.json` has 227 entries, the builder records expected counts and threshold, `_research/20_b1_cache_provisioning.md` records the rebuild ceremony, and provenance Hash B / Hash C provide the current environment and cache-byte anchors.

4. **Round-summary verification is native.** `kb verify --round-summary` no longer depends on the old stub path. Parser edge tests, golden fixtures, fingerprint checks, and parity rows define the command contract for future vertical summaries.

5. **Batch semantic verification has a load/invocation contract.** The cache loads once per batch, while the semantic evaluator is invoked per citation. Future verticals can reuse that behavior rather than re-deciding cache lifetime semantics.

6. **The B2 judge path is opt-in.** Default builds reject `--semantic-judge` and keep network/async dependencies out of the default `cacg-cli` closure. Feature-on builds have mocked and wire-level tests, including non-leakage and request-body assertions.

7. **Calibration and eval gates are available.** `threshold-sweep` and `semantic-eval` give future verticals a way to lock threshold behavior and fixture-based verdict classification before turning semantic verification into a release gate.

8. **The locked transcript pattern exists.** AC-9.1 shows how to build an end-to-end semantic transcript that is byte-equal after path normalization while excluding runtime-only fields.

9. **The full static-gate firewall is in place.** Every M5b-relevant static gate (`lint-determinism`, `lint-trust-leak`, `lint-platform-cfg`, `lint-rename-outside-publisher`, `lint-runner-bypass`, `audit-cacg-core-deps`, `audit-default-kb-deps`) reports zero violations. The lint-determinism rule applies to `crates` and `xtask/src` and exempts only paths containing `/tests/`; the inline tests in `xtask/src/semantic_eval.rs` use `TempDir::new()` plus deterministic child filenames so they satisfy the determinism contract without relocation or rule widening.

M5b closes with the semantic verifier port, round-summary native port, cache/provenance/calibration work, B2 opt-in feature, locked transcript, and cross-cutting determinism + boundary gate all shipped. AC-9 is fully MET after the Round-41 BS-20 closure; the M5b end-state is **13/13 ACs MET**.

---

## Relocation Addendum (Python-tree quarantine)

_Added at HEAD `246309d` (M5b post-cleanup), revised when the CACG Python-tree retirement plan's P1 Quarantine sub-milestone landed._

The legacy Python tree referenced throughout this exit artifact has been relocated under the `legacy_python_oracle/` namespace via `git mv` so individual-file history is preserved (verifiable via `git log --follow legacy_python_oracle/<new-path>`). The pre-quarantine evidence sentences above are preserved verbatim; this addendum records the post-quarantine resolvability of every Python path citation in the exit artifact.

Paths relocated under `legacy_python_oracle/` per the quarantine sub-milestone; the underlying evidence remains byte-identical at commit `29dccce` (the original M5b exit attestation SHA) and continues to verify at the new paths post-quarantine.

### Path mapping

The Python paths cited above resolve as follows after the quarantine relocation:

| Pre-quarantine citation | Post-quarantine path |
|---|---|
| `src/cacg/verify/layer2.py` | `legacy_python_oracle/src/cacg/verify/layer2.py` |
| `src/cacg/cli.py::_cmd_verify_round_summary` | `legacy_python_oracle/src/cacg/cli.py::_cmd_verify_round_summary` |
| `scripts/build_semantic_cache.py` | `legacy_python_oracle/scripts/build_semantic_cache.py` |
| `scripts/build_semantic_cache.py::DEFAULT_THRESHOLD` | `legacy_python_oracle/scripts/build_semantic_cache.py::DEFAULT_THRESHOLD` |
| `tests/test_semantic_cache_provisioning.py` | `legacy_python_oracle/tests/test_semantic_cache_provisioning.py` |
| `pytest tests/test_semantic_cache_provisioning.py` | `legacy_python_oracle/.venv/bin/pytest legacy_python_oracle/tests/test_semantic_cache_provisioning.py` |

Rust-owned paths cited in the original artifact (e.g., `crates/cacg-semantic/src/lib.rs`, `crates/cacg-core/src/verify/semantic_spec.rs`, `crates/cacg-semantic/tests/committed_cache.rs`, `out/semantic_cache.json`, `tests/parity_corpus/`, `tests/golden/`, `tests/adversarial/`) stay at their workspace-root locations; the relocation did not move any Rust source, generated table, committed cache, parity-corpus fixture, golden card, or adversarial fixture.

### Verification

- `git show 29dccce:src/cacg/verify/layer2.py` retrieves the pre-quarantine bytes (verifiable from any clone with the original commit reachable).
- `git log --follow legacy_python_oracle/src/cacg/verify/layer2.py` returns the continuous history including the M5b-era commits.
- `legacy_python_oracle/.venv/bin/pytest legacy_python_oracle/tests/test_semantic_cache_provisioning.py` returns the same `6 passed + 1 skipped` distribution as before the relocation (per AC-5 boundary-gate validation in the quarantine round).
- `cargo xtask audit-semantic-cache-provenance` continues to pass with the relocated `uv.lock` (now at `legacy_python_oracle/uv.lock`); the `--uv-lock` default in `xtask/src/main.rs` was updated in tandem with the relocation per AC-1.1.

### Reverse-resolvability

For each evidence citation in the body of this artifact, the file is reachable at exactly one of the following locations:

- The original workspace-relative path at commit `29dccce` (`git show 29dccce:<path>`), OR
- The post-quarantine path under `legacy_python_oracle/` (resolved at the current HEAD).

Both paths point at byte-identical content; the relocation is a pure-relocation milestone with no semantic behavior change. Any subsequent change to the M5b evidence requires a fresh exit artifact at `_research/22_python_retirement_completion_summary.md` per the terminal sub-milestone contract, NOT in-place edits to this file.

### Quarantine end-state pointer

The terminal Python-retirement sub-milestone will land at a separate exit artifact (`_research/22_python_retirement_completion_summary.md`) once every live Rust runtime, test, xtask command, and CI workflow is independent of `legacy_python_oracle/` AND a final execution-token sweep proves zero leaks. When that lands, this artifact gains a one-line final addendum referencing the new exit artifact and the deletion commit SHA.

## Parity-Surface Contract Evolution Addendum

The `cargo xtask parity` byte-equal parity contract has evolved from "Python ↔ Rust live byte-equal" to "Rust ↔ committed-Python-oracle byte-equal" as part of the Python retirement P3 sub-milestone. The evidence chain is preserved via the frozen oracle fixtures committed under `tests/parity_corpus/out_python/parity_rows/`:

- **REPORT SCHEMA**: All Python-coupled field names (`python_command`, `python_path`, `MissingPython`) have been replaced with oracle-agnostic equivalents (`expected_command`, `expected_path`, `MissingExpected`). The report JSON schema no longer contains any Python-specific symbols.

- **Row executors**: All 16 gating M2 rows (kb_index ×2, kb_lint ×2, kb_verify ×3, kb_verify_round_summary ×1, kb_verify_semantic ×2, kb_verify_round_summary_semantic ×1, kb_search ×4, kb_show ×1) now read expected artifacts from committed fixtures captured once from the Python oracle, rather than spawning Python live. Each row's parity report entry records `expected_command: "committed-fixture: <path>"` and `expected_duration_ms: 0`.

- **Help-snapshot disposition**: The 14 future-stage help-snapshot rows (AC-7.1) have been removed from the matrix (Path B: delete from matrix, CLI-surface contract transferred to committed Python argparse snapshots at `tests/parity_corpus/help_snapshots/` and to the existing Rust clap parity tests). The matrix went from 36 entries to 22 entries (16 gating + 6 future-stage PDF-ingest rows).

- **Status-check name**: The GitHub workflow job name has been updated from `"Python <-> Rust byte-equal parity"` to `"Committed-fixture byte-equal parity"` in both `.github/workflows/parity.yml` and `docs/release-discipline.md`.

The underlying M5b evidence (13/13 ACs MET) remains valid: the parity contract still proves byte-equal CLI output between the Rust implementation and the frozen Python oracle output. The oracle is now committed rather than live.

## Semantic Cache Contract Evolution Addendum (AC-11)

The semantic-cache provenance contract (AC-6 of M5b) has evolved per AC-11 Option B: the cache is frozen as immutable. The audit (`cargo xtask audit-semantic-cache-provenance`) now verifies Hash C (cache content integrity), entry counts (222 paraphrase + 5 negative = 227), model identity (MiniLM-L6-v2 @ pinned revision), and threshold (0.50) only. Hash B (build environment binding) and uv.lock are no longer verified — the build environment is documented in the provenance as a historical record but is not re-checked at audit time, since the cache will not be rebuilt.

Plan evolution: DEC-1 was originally resolved to Option C (full Rust embedding pipeline). Independent ML assessment confirmed that no Rust crate produces byte-reproducible MiniLM-L6-v2 embeddings against `sentence-transformers==5.5.1`, making Option C infeasible near-term. Option B (freeze cache) is the operational baseline that unblocks AC-15 final deletion without requiring a Rust embedding pipeline. The schema_version remains `cacg.v0` (the cache is frozen at this version).

## Final Retirement Addendum

Quarantine retired at commit `fd97cad`. The `legacy_python_oracle/` directory has been deleted — 173 git-tracked files (39,521 lines of Python) removed in a single atomic commit. Evidence re-attested at `_research/22_python_retirement_completion_summary.md` (pending).
