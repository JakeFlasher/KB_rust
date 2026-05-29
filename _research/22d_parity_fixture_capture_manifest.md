# task-pyret-28: parity fixture capture manifest

Scope: the 16 M2 PASS rows in `xtask/src/parity.rs` covering `kb index`, `kb lint`, `kb verify`, `kb verify --round-summary`, semantic verify, `kb search`, and `kb show`.

The row executors compare Python-side artifacts from their per-row `py-out` directory, plus `kb index` history sidecars from the isolated Python corpus copy. For committed-fixture conversion, use this row-scoped namespace for newly captured artifacts:

`tests/parity_corpus/out_python/parity_rows/<row-name>/...`

`summaries.sqlite` and `.kb_index_cache.json` are runtime byproducts for `kb index` but are explicitly not audited by `is_audited_artifact_name()` and are gitignored. They should not be captured for these 16 rows unless the comparison contract changes.

## Shared Artifact Sets

Per-card lint/verify artifact set for stem `<stem>`:

- `<stem>.stdout`
- `<stem>.stderr`
- `<stem>.exit`
- `<stem>.lint_journal.jsonl`

Per-case read-only CLI artifact set for label `<label>`:

- `<label>.stdout`
- `<label>.stderr`
- `<label>.exit`

Valid-card stems:

- `01-content-addressable-identity`
- `02-determinism-is-a-kept-promise`
- `synthetic-card-01`
- `synthetic-card-02`
- `synthetic-card-03`

Adversarial-card stems:

- `01-malformed-hash`
- `02-reversed-page-range`
- `03-chunk-not-in-manifest`
- `04-stale-card-hash`
- `05-chunk-hash-drift`
- `06-page-disjoint`
- `07-fake-quote`
- `08-auth-unknown-reading`
- `09-auth-unauthorized-source`
- `10-retracted-card`
- `11-retracted-source-cited`
- `12-retracted-chunk-cited`

Round-summary fixture stems:

- `01_clean_na`
- `02_missing_section_non_kb`
- `03_missing_section_kb_relevant`
- `04_na_on_kb_relevant`
- `05_empty_section_kb_relevant`
- `06_sentinel_collision`
- `07_verified_single_card`
- `08_missing_card`
- `09_multiple_sections`

## 1. `kb_index_parity_corpus_reading_01`

Runtime Python artifacts:

- Process stdout/stderr/exit status exist, but `run_kb_index_entry()` only records success/failure and does not persist or byte-compare those streams.
- Audited `py-out` files: `INDEX.md`, `cards_manifest.json`, `summaries.json`.
- Non-audited `py-out` files: `summaries.sqlite`, `.kb_index_cache.json`.
- Audited isolated corpus history sidecars copied from/generated in `cards/reading_01`: five `*.history.jsonl` files.

Already committed under `tests/parity_corpus/out_python/`:

- `INDEX.md` exists at the root and byte-matches this row's current `py-out/INDEX.md`.
- `cards_manifest.json` and `summaries.json` exist at the root, but they are not this row's current bytes: the committed root files record `tests/parity_corpus/cards/reading_01/...` paths, while `run_kb_index_entry()` runs from an isolated corpus root and records `cards/reading_01/...` paths.

Missing capture:

- `tests/parity_corpus/out_python/parity_rows/kb_index_parity_corpus_reading_01/py-out/cards_manifest.json`
- `tests/parity_corpus/out_python/parity_rows/kb_index_parity_corpus_reading_01/py-out/summaries.json`
- `tests/parity_corpus/out_python/parity_rows/kb_index_parity_corpus_reading_01/py-corpus/cards/reading_01/01-content-addressable-identity.history.jsonl`
- `tests/parity_corpus/out_python/parity_rows/kb_index_parity_corpus_reading_01/py-corpus/cards/reading_01/02-determinism-is-a-kept-promise.history.jsonl`
- `tests/parity_corpus/out_python/parity_rows/kb_index_parity_corpus_reading_01/py-corpus/cards/reading_01/synthetic-card-01.history.jsonl`
- `tests/parity_corpus/out_python/parity_rows/kb_index_parity_corpus_reading_01/py-corpus/cards/reading_01/synthetic-card-02.history.jsonl`
- `tests/parity_corpus/out_python/parity_rows/kb_index_parity_corpus_reading_01/py-corpus/cards/reading_01/synthetic-card-03.history.jsonl`

No one-time capture needed for `INDEX.md` if the committed-fixture runner maps this row to the existing root-level `out_python/INDEX.md`. If it requires a self-contained row directory, duplicate it under `.../py-out/INDEX.md`.

## 2. `kb_index_parity_corpus_stale_hash_reading_01`

Runtime Python artifacts:

- Process stdout/stderr/exit status exist, but are not persisted or byte-compared by this row.
- Audited `py-out` files: `INDEX.md`, `cards_manifest.json`, `summaries.json`.
- Non-audited `py-out` files: `summaries.sqlite`, `.kb_index_cache.json`.
- Audited isolated corpus history sidecar: `01-content-addressable-identity.history.jsonl`.

Already committed under `tests/parity_corpus/out_python/`:

- `stale_hash/reading_01/01-content-addressable-identity.history.jsonl`

Missing capture:

- `tests/parity_corpus/out_python/parity_rows/kb_index_parity_corpus_stale_hash_reading_01/py-out/INDEX.md`
- `tests/parity_corpus/out_python/parity_rows/kb_index_parity_corpus_stale_hash_reading_01/py-out/cards_manifest.json`
- `tests/parity_corpus/out_python/parity_rows/kb_index_parity_corpus_stale_hash_reading_01/py-out/summaries.json`

No one-time capture needed for the history sidecar if the committed-fixture runner maps this row to the existing `out_python/stale_hash/reading_01/` file. If it requires a self-contained row directory, duplicate it under `.../py-corpus/cards_stale_hash/reading_01/`.

## 3. `kb_lint_parity_golden`

Runtime Python artifacts:

- For each valid-card stem: stdout, stderr, exit code, and `<stem>.lint_journal.jsonl`.

Already committed under `tests/parity_corpus/out_python/`:

- None of this row's runtime stdout/stderr/exit/journal artifacts.
- Existing `out_python/valid/<stem>/lint.json` files are older lint oracles, not artifacts generated by `run_kb_lint_entry_with()`.

Missing capture:

- For every valid-card stem, capture the shared per-card lint/verify artifact set to `tests/parity_corpus/out_python/parity_rows/kb_lint_parity_golden/py-out/`.

## 4. `kb_lint_parity_adversarial`

Runtime Python artifacts:

- For each adversarial-card stem: stdout, stderr, exit code, and `<stem>.lint_journal.jsonl`.

Already committed under `tests/parity_corpus/out_python/`:

- None of this row's runtime stdout/stderr/exit/journal artifacts.
- Existing `out_python/adversarial/<stem>/lint.json` files are older lint oracles, not artifacts generated by `run_kb_lint_entry_with()`.

Missing capture:

- For every adversarial-card stem, capture the shared per-card lint/verify artifact set to `tests/parity_corpus/out_python/parity_rows/kb_lint_parity_adversarial/py-out/`.

## 5. `kb_verify_parity_golden`

Runtime Python artifacts:

- For each valid-card stem: stdout, stderr, exit code, and `<stem>.lint_journal.jsonl`.

Already committed under `tests/parity_corpus/out_python/`:

- None.

Missing capture:

- For every valid-card stem, capture the shared per-card lint/verify artifact set to `tests/parity_corpus/out_python/parity_rows/kb_verify_parity_golden/py-out/`.

## 6. `kb_verify_fuzzy_parity`

Runtime Python artifacts:

- Same five valid-card stems as `kb_verify_parity_golden`, with `--fuzzy` added to the Python invocation.
- For each stem: stdout, stderr, exit code, and `<stem>.lint_journal.jsonl`.

Already committed under `tests/parity_corpus/out_python/`:

- None.

Missing capture:

- For every valid-card stem, capture the shared per-card lint/verify artifact set to `tests/parity_corpus/out_python/parity_rows/kb_verify_fuzzy_parity/py-out/`.

## 7. `kb_verify_skip_lint_parity`

Runtime Python artifacts:

- Same five valid-card stems as `kb_verify_parity_golden`, with `--unsafe-skip-lint` added to the Python invocation.
- For each stem: stdout, stderr, exit code, and `<stem>.lint_journal.jsonl`.

Already committed under `tests/parity_corpus/out_python/`:

- None.

Missing capture:

- For every valid-card stem, capture the shared per-card lint/verify artifact set to `tests/parity_corpus/out_python/parity_rows/kb_verify_skip_lint_parity/py-out/`.

## 8. `kb_verify_round_summary_parity_golden`

Runtime Python artifacts:

- For each round-summary fixture stem: stdout, stderr, exit code, and `<stem>.lint_journal.jsonl`.
- The executor pre-touches the journal path, so fixtures that append no entries still produce an empty journal file.

Already committed under `tests/parity_corpus/out_python/`:

- None.

Missing capture:

- For every round-summary fixture stem, capture the shared per-card lint/verify artifact set to `tests/parity_corpus/out_python/parity_rows/kb_verify_round_summary_parity_golden/py-out/`.

## 9. `kb_verify_semantic_parity_golden`

Runtime Python artifacts:

- Single card: `tests/parity_corpus/semantic/card.md`.
- Stem: `card`.
- Artifacts: `card.stdout`, `card.stderr`, `card.exit`, `card.lint_journal.jsonl`.
- The executor pre-touches the journal path.

Already committed under `tests/parity_corpus/out_python/`:

- None.

Missing capture:

- `tests/parity_corpus/out_python/parity_rows/kb_verify_semantic_parity_golden/py-out/card.stdout`
- `tests/parity_corpus/out_python/parity_rows/kb_verify_semantic_parity_golden/py-out/card.stderr`
- `tests/parity_corpus/out_python/parity_rows/kb_verify_semantic_parity_golden/py-out/card.exit`
- `tests/parity_corpus/out_python/parity_rows/kb_verify_semantic_parity_golden/py-out/card.lint_journal.jsonl`

## 10. `kb_verify_semantic_miss_parity_golden`

Runtime Python artifacts:

- Same single card and stem as `kb_verify_semantic_parity_golden`, but with `semantic_cache_empty.json`.
- Artifacts: `card.stdout`, `card.stderr`, `card.exit`, `card.lint_journal.jsonl`.

Already committed under `tests/parity_corpus/out_python/`:

- None.

Missing capture:

- `tests/parity_corpus/out_python/parity_rows/kb_verify_semantic_miss_parity_golden/py-out/card.stdout`
- `tests/parity_corpus/out_python/parity_rows/kb_verify_semantic_miss_parity_golden/py-out/card.stderr`
- `tests/parity_corpus/out_python/parity_rows/kb_verify_semantic_miss_parity_golden/py-out/card.exit`
- `tests/parity_corpus/out_python/parity_rows/kb_verify_semantic_miss_parity_golden/py-out/card.lint_journal.jsonl`

## 11. `kb_verify_round_summary_semantic_parity_golden`

Runtime Python artifacts:

- Single summary: `tests/parity_corpus/round_summary_semantic/summary.md`.
- Stem: `summary`.
- Artifacts: `summary.stdout`, `summary.stderr`, `summary.exit`, `summary.lint_journal.jsonl`.
- The executor pre-touches the journal path.

Already committed under `tests/parity_corpus/out_python/`:

- None.

Missing capture:

- `tests/parity_corpus/out_python/parity_rows/kb_verify_round_summary_semantic_parity_golden/py-out/summary.stdout`
- `tests/parity_corpus/out_python/parity_rows/kb_verify_round_summary_semantic_parity_golden/py-out/summary.stderr`
- `tests/parity_corpus/out_python/parity_rows/kb_verify_round_summary_semantic_parity_golden/py-out/summary.exit`
- `tests/parity_corpus/out_python/parity_rows/kb_verify_round_summary_semantic_parity_golden/py-out/summary.lint_journal.jsonl`

## 12. `kb_search_parity_corpus`

Runtime Python artifacts:

- Read-only `kb search`; no journal and no corpus mutation.
- Per case: stdout, stderr, exit code.

Already committed under `tests/parity_corpus/out_python/`:

- None. `tests/parity_corpus/kb_search/oracle.json` is outside `out_python` and is not the live row artifact set.

Missing capture to `tests/parity_corpus/out_python/parity_rows/kb_search_parity_corpus/py-out/`:

- Case `success_human`: `success_human.stdout`, `success_human.stderr`, `success_human.exit`
- Case `success_json`: `success_json.stdout`, `success_json.stderr`, `success_json.exit`
- Case `success_human_broad`: `success_human_broad.stdout`, `success_human_broad.stderr`, `success_human_broad.exit`
- Case `success_json_broad`: `success_json_broad.stdout`, `success_json_broad.stderr`, `success_json_broad.exit`
- Case `zero_result_human`: `zero_result_human.stdout`, `zero_result_human.stderr`, `zero_result_human.exit`
- Case `zero_result_json`: `zero_result_json.stdout`, `zero_result_json.stderr`, `zero_result_json.exit`
- Case `negative_top_k`: `negative_top_k.stdout`, `negative_top_k.stderr`, `negative_top_k.exit`
- Case `top_k_cap`: `top_k_cap.stdout`, `top_k_cap.stderr`, `top_k_cap.exit`

## 13. `kb_search_parity_cfa_first_bite`

Runtime Python artifacts:

- Read-only `kb search`; no journal and no corpus mutation.
- Per case: stdout, stderr, exit code.

Already committed under `tests/parity_corpus/out_python/`:

- None.

Missing capture to `tests/parity_corpus/out_python/parity_rows/kb_search_parity_cfa_first_bite/py-out/`:

- Case `cfa_title_valuation`: `cfa_title_valuation.stdout`, `cfa_title_valuation.stderr`, `cfa_title_valuation.exit`
- Case `cfa_title_valuation_json`: `cfa_title_valuation_json.stdout`, `cfa_title_valuation_json.stderr`, `cfa_title_valuation_json.exit`
- Case `cfa_tag_multiples`: `cfa_tag_multiples.stdout`, `cfa_tag_multiples.stderr`, `cfa_tag_multiples.exit`
- Case `cfa_summary_earnings`: `cfa_summary_earnings.stdout`, `cfa_summary_earnings.stderr`, `cfa_summary_earnings.exit`
- Case `cfa_zero_result`: `cfa_zero_result.stdout`, `cfa_zero_result.stderr`, `cfa_zero_result.exit`
- Case `cfa_zero_result_json`: `cfa_zero_result_json.stdout`, `cfa_zero_result_json.stderr`, `cfa_zero_result_json.exit`

## 14. `kb_search_parity_fts5_present`

Runtime Python artifacts:

- Read-only `kb search` over `tests/parity_corpus/kb_search_fts5/`; no journal and no corpus mutation.
- The row consumes the committed `summaries.sqlite` sidecar but does not generate a new one.
- Per case: stdout, stderr, exit code.

Already committed under `tests/parity_corpus/out_python/`:

- None.

Missing capture to `tests/parity_corpus/out_python/parity_rows/kb_search_parity_fts5_present/py-out/`:

- Case `cfa_title_valuation`: `cfa_title_valuation.stdout`, `cfa_title_valuation.stderr`, `cfa_title_valuation.exit`
- Case `cfa_title_valuation_json`: `cfa_title_valuation_json.stdout`, `cfa_title_valuation_json.stderr`, `cfa_title_valuation_json.exit`
- Case `cfa_tag_multiples`: `cfa_tag_multiples.stdout`, `cfa_tag_multiples.stderr`, `cfa_tag_multiples.exit`
- Case `cfa_summary_earnings`: `cfa_summary_earnings.stdout`, `cfa_summary_earnings.stderr`, `cfa_summary_earnings.exit`
- Case `cfa_zero_result`: `cfa_zero_result.stdout`, `cfa_zero_result.stderr`, `cfa_zero_result.exit`
- Case `cfa_zero_result_json`: `cfa_zero_result_json.stdout`, `cfa_zero_result_json.stderr`, `cfa_zero_result_json.exit`

## 15. `kb_search_parity_fts5_stale`

Runtime Python artifacts:

- Read-only `kb search` over `tests/parity_corpus/kb_search_fts5_stale/`; no journal and no corpus mutation.
- The row consumes the committed stale `summaries.sqlite` sidecar, emits the `CACG-FTS-001` diagnostic to stderr, then falls back to in-memory BM25.
- Per case: stdout, stderr, exit code.

Already committed under `tests/parity_corpus/out_python/`:

- None.

Missing capture to `tests/parity_corpus/out_python/parity_rows/kb_search_parity_fts5_stale/py-out/`:

- Case `cfa_title_valuation`: `cfa_title_valuation.stdout`, `cfa_title_valuation.stderr`, `cfa_title_valuation.exit`
- Case `cfa_title_valuation_json`: `cfa_title_valuation_json.stdout`, `cfa_title_valuation_json.stderr`, `cfa_title_valuation_json.exit`
- Case `cfa_tag_multiples`: `cfa_tag_multiples.stdout`, `cfa_tag_multiples.stderr`, `cfa_tag_multiples.exit`
- Case `cfa_summary_earnings`: `cfa_summary_earnings.stdout`, `cfa_summary_earnings.stderr`, `cfa_summary_earnings.exit`
- Case `cfa_zero_result`: `cfa_zero_result.stdout`, `cfa_zero_result.stderr`, `cfa_zero_result.exit`
- Case `cfa_zero_result_json`: `cfa_zero_result_json.stdout`, `cfa_zero_result_json.stderr`, `cfa_zero_result_json.exit`

## 16. `kb_show_parity`

Runtime Python artifacts:

- Read-only `kb show`; no journal and no corpus mutation.
- Per case: stdout, stderr, exit code.

Already committed under `tests/parity_corpus/out_python/`:

- None of this row's runtime stdout/stderr/exit artifacts.
- Existing `out_python/dependency_retracted/scenario-01/show_*.json` files are scenario oracles, not the artifacts generated by `run_kb_show_entry_with()`.

Missing capture to `tests/parity_corpus/out_python/parity_rows/kb_show_parity/py-out/`:

- Case `active`: `active.stdout`, `active.stderr`, `active.exit`
- Case `directly_retracted_cli_001`: `directly_retracted_cli_001.stdout`, `directly_retracted_cli_001.stderr`, `directly_retracted_cli_001.exit`
- Case `dependency_retracted_refused`: `dependency_retracted_refused.stdout`, `dependency_retracted_refused.stderr`, `dependency_retracted_refused.exit`
- Case `dependency_retracted_shown`: `dependency_retracted_shown.stdout`, `dependency_retracted_shown.stderr`, `dependency_retracted_shown.exit`
- Case `missing_card`: `missing_card.stdout`, `missing_card.stderr`, `missing_card.exit`
- Case `path_override_match`: `path_override_match.stdout`, `path_override_match.stderr`, `path_override_match.exit`
- Case `path_override_mismatch`: `path_override_mismatch.stdout`, `path_override_mismatch.stderr`, `path_override_mismatch.exit`
- Case `unauthorized_reading`: `unauthorized_reading.stdout`, `unauthorized_reading.stderr`, `unauthorized_reading.exit`
- Case `unauthorized_source`: `unauthorized_source.stdout`, `unauthorized_source.stderr`, `unauthorized_source.exit`
- Case `cfa_search_to_show`: `cfa_search_to_show.stdout`, `cfa_search_to_show.stderr`, `cfa_search_to_show.exit`

## Capture Count Summary

Rows with no missing compared artifacts if existing root mappings are allowed:

- None. The clean `kb index` row still lacks its compared history sidecars under `out_python`.

Rows with partially committed compared artifacts:

- `kb_index_parity_corpus_reading_01`: root-level `INDEX.md` already exists and byte-matches; current-row `cards_manifest.json`, `summaries.json`, and five history sidecars are missing under `out_python`.
- `kb_index_parity_corpus_stale_hash_reading_01`: stale history sidecar already exists; stale row `py-out` manifests are missing.

Rows needing full one-time capture of their compared runtime artifacts:

- `kb_lint_parity_golden`
- `kb_lint_parity_adversarial`
- `kb_verify_parity_golden`
- `kb_verify_fuzzy_parity`
- `kb_verify_skip_lint_parity`
- `kb_verify_round_summary_parity_golden`
- `kb_verify_semantic_parity_golden`
- `kb_verify_semantic_miss_parity_golden`
- `kb_verify_round_summary_semantic_parity_golden`
- `kb_search_parity_corpus`
- `kb_search_parity_cfa_first_bite`
- `kb_search_parity_fts5_present`
- `kb_search_parity_fts5_stale`
- `kb_show_parity`
