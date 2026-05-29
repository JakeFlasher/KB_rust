# Lint Diagnostic Emission Order Audit (task-vh-5)

Audit produced via `/humanize:ask-codex` (`gpt-5.5:high`) and committed as the port
checklist for task-vh-6 (`cacg-core::lint::layer1::run_layer1_checks`). All citations
below are `file:line` references into the actual Python source as of commit
`3c9a7e8`. Diagnostic ORDER is part of the byte-equal HYBRID parity contract; the
Rust port must mirror this audit literally.

Plan reference: `.humanize/.humanize/plans/cacg-rust-port-m3-verify-hot-path-plan.md` AC-2 (trust-bearing
code subset) and AC-2.1 (auxiliary-codes carve-out). Codex was given the AC-2 subset
and the AC-2.1 carve-out as pre-loaded context.

---

## 1. File overview

Audit target: `src/cacg/lint/layer1.py`, 595 LoC.

High-level structure, top to bottom:

- `_ChunkIndex` is the local lookup wrapper used by citation checks; it stores `by_id`, `retracted_source_ids`, and `retracted_chunk_ids` (`src/cacg/lint/layer1.py:43`).
- `_index_chunks()` builds the `chunk_id -> ChunkRecord` map, rejects duplicate `chunk_id`s by raising `_ManifestLoadError`, and carries source/chunk retraction sets from the manifest (`src/cacg/lint/layer1.py:54`, `src/cacg/lint/layer1.py:66`, `src/cacg/lint/layer1.py:71`, `src/cacg/lint/layer1.py:76`).
- `_ManifestLoadError` is the internal exception mapped to `CACG-MAN-001` by `run_layer1_checks()` (`src/cacg/lint/layer1.py:83`, `src/cacg/lint/layer1.py:331`).
- `_load_chunks_manifest()` reads and Pydantic-validates `chunks_manifest.json`; validation, IO, and JSON/value errors become `_ManifestLoadError` (`src/cacg/lint/layer1.py:92`, `src/cacg/lint/layer1.py:95`, `src/cacg/lint/layer1.py:96`).
- `_check_citations()` is the per-citation trust loop. It emits malformed chunk id/hash, source/chunk retraction, manifest presence, source mismatch, chunk hash mismatch, page disjointness, and optional authorization diagnostics (`src/cacg/lint/layer1.py:100`, `src/cacg/lint/layer1.py:109`, `src/cacg/lint/layer1.py:247`).
- `_check_card_hash()` emits `CACG-HASH-002` when an existing `card_hash` is stale (`src/cacg/lint/layer1.py:250`, `src/cacg/lint/layer1.py:258`, `src/cacg/lint/layer1.py:263`).
- `_diag_failure()` is the shared severity gate for exit/failure state (`src/cacg/lint/layer1.py:276`).
- `freeze_aware_latency()` is local to this file and returns `0.0` when `KB_FROZEN_CLOCK=1` via `clock.frozen()` (`src/cacg/lint/layer1.py:280`, `src/cacg/lint/layer1.py:282`; `src/cacg/clock.py:16`).
- `run_layer1_checks()` is the pure lint entry point for the new Rust parity surface: it loads the card, loads or receives the chunk index, emits diagnostics, and returns `(diagnostics, card_hash_before, doc_or_none)` without journaling (`src/cacg/lint/layer1.py:285`, `src/cacg/lint/layer1.py:306`).
- `lint_card()` is the journal-writing wrapper around `run_layer1_checks()`. It appends exactly one `command="lint"` journal entry for one card visit (`src/cacg/lint/layer1.py:354`, `src/cacg/lint/layer1.py:375`, `src/cacg/lint/layer1.py:381`).
- `lint_directory()` is the batch wrapper: it preloads `ChunksIndex` once when possible, calls `lint_card()` for each non-`SKILL.md` markdown card, then runs deferred aggregate SKILL/ROLE/DEP checks (`src/cacg/lint/layer1.py:396`, `src/cacg/lint/layer1.py:432`, `src/cacg/lint/layer1.py:505`, `src/cacg/lint/layer1.py:508`, `src/cacg/lint/layer1.py:537`, `src/cacg/lint/layer1.py:573`, `src/cacg/lint/layer1.py:585`).

`chunks_manifest.json` load points:

- Single-card or fallback path: `run_layer1_checks()` calls `_load_chunks_manifest()` and `_index_chunks()` when no shared `chunks_index` is supplied (`src/cacg/lint/layer1.py:324`, `src/cacg/lint/layer1.py:326`, `src/cacg/lint/layer1.py:330`).
- Batch path: `lint_directory()` tries `ChunksIndex.from_path(chunks_manifest_path)` once before the card loop (`src/cacg/lint/layer1.py:432`, `src/cacg/lint/layer1.py:433`). If that preload fails, `chunks_index = None`, so each `lint_card()` falls back to the per-card manifest load and can still emit per-card `CACG-MAN-001` (`src/cacg/lint/layer1.py:434`, `src/cacg/lint/layer1.py:435`, `src/cacg/lint/layer1.py:508`).

Journal append point:

- `run_layer1_checks()` explicitly does not append to the journal (`src/cacg/lint/layer1.py:306`).
- `lint_card()` computes diagnostics, derives failure state, and calls `append_entry()` exactly once (`src/cacg/lint/layer1.py:376`, `src/cacg/lint/layer1.py:380`, `src/cacg/lint/layer1.py:381`).
- `append_entry()` serializes diagnostics into the JSONL event payload at `src/cacg/journal.py:268` and `src/cacg/journal.py:277`, then writes one line at `src/cacg/journal.py:293`.

## 2. Diagnostic emission inventory

| File:Line | CACG code (constant) | Severity | Firing condition | Per-card or per-citation |
|---|---|---:|---|---|
| `src/cacg/lint/layer1.py:113` | `C.CITE_MALFORMED_CHUNK_ID` = `CACG-CITE-001` | error | `cit.chunk_id` does not match `_CHUNK_ID_RE` at `src/cacg/lint/layer1.py:112`; appends then `continue`s at `src/cacg/lint/layer1.py:119`. | Per-citation |
| `src/cacg/lint/layer1.py:124` | `C.CITE_MALFORMED_HASH` = `CACG-CITE-002` | error | `cit.chunk_hash == "0" * 64` placeholder sentinel at `src/cacg/lint/layer1.py:123`; appends then `continue`s at `src/cacg/lint/layer1.py:130`. | Per-citation |
| `src/cacg/lint/layer1.py:132` | `C.CITE_MALFORMED_HASH` = `CACG-CITE-002` | error | `cit.chunk_hash` does not match `_HEX64` at `src/cacg/lint/layer1.py:131`; appends then `continue`s at `src/cacg/lint/layer1.py:138`. Normally precluded for file-loaded cards by schema validation, but present in the helper contract. | Per-citation |
| `src/cacg/lint/layer1.py:146` | `C.RETR_SOURCE_RETRACTED` = `CACG-RETR-002` | error | `cit.source_id` is in `idx.retracted_source_ids` at `src/cacg/lint/layer1.py:145`; appends then `continue`s at `src/cacg/lint/layer1.py:155`. | Per-citation |
| `src/cacg/lint/layer1.py:157` | `C.RETR_CHUNK_RETRACTED` = `CACG-RETR-003` | error | `cit.chunk_id` is in `idx.retracted_chunk_ids` at `src/cacg/lint/layer1.py:156`; appends then `continue`s at `src/cacg/lint/layer1.py:166`. | Per-citation |
| `src/cacg/lint/layer1.py:170` | `C.CITE_CHUNK_NOT_IN_MANIFEST` = `CACG-CITE-004` | error | `idx.by_id.get(cit.chunk_id)` returns `None` at `src/cacg/lint/layer1.py:168`; appends then `continue`s at `src/cacg/lint/layer1.py:176`. | Per-citation |
| `src/cacg/lint/layer1.py:181` | `C.CITE_SOURCE_NOT_IN_MANIFEST` = `CACG-CITE-006` | error | `cit.source_id != chunk.source_id` at `src/cacg/lint/layer1.py:180`; appends then `continue`s at `src/cacg/lint/layer1.py:187`. | Per-citation |
| `src/cacg/lint/layer1.py:190` | `C.HASH_CHUNK_MISMATCH` = `CACG-HASH-001` | error | `cit.chunk_hash != chunk.chunk_hash` at `src/cacg/lint/layer1.py:189`; appends then `continue`s at `src/cacg/lint/layer1.py:199`. | Per-citation |
| `src/cacg/lint/layer1.py:210` | `C.CITE_PAGE_DISJOINT` = `CACG-CITE-005` | error | `cited_pages_set` has no intersection with actual `chunk.page_spans` pages at `src/cacg/lint/layer1.py:206` and `src/cacg/lint/layer1.py:208`; appends then `continue`s at `src/cacg/lint/layer1.py:219`. | Per-citation |
| `src/cacg/lint/layer1.py:241` | dynamic `code` from `is_citation_authorized()`: `CACG-AUTH-001` or `CACG-AUTH-002`; fallback `C.AUTH_SOURCE_UNAUTHORIZED` = `CACG-AUTH-002` | error | `auth` and `auth.matrix` are present at `src/cacg/lint/layer1.py:224`, `is_citation_authorized()` returns `ok == False` at `src/cacg/lint/layer1.py:225` and `src/cacg/lint/layer1.py:230`. Helper returns `CACG-AUTH-001` for unknown reading and `CACG-AUTH-002` for unauthorized source (`src/cacg/source_matrix.py:109-122`, specifically `src/cacg/source_matrix.py:119` for the `CACG-AUTH-001` return and `src/cacg/source_matrix.py:121` for the `CACG-AUTH-002` return). | Per-citation |
| `src/cacg/lint/layer1.py:264` | `C.HASH_CARD_STALE` = `CACG-HASH-002` | error | `_check_card_hash()` sees stored card hash is present and differs from recomputed `card_hash(fm_dict, doc.body)` at `src/cacg/lint/layer1.py:258`, `src/cacg/lint/layer1.py:262`, `src/cacg/lint/layer1.py:263`. | Per-card |
| `src/cacg/lint/layer1.py:315` | carried from `load_card()`: `CACG-CLI-001`, `CACG-FM-001..008`, `CACG-CITE-002`, `CACG-CITE-003`, `CACG-SUM-001..004` | error | `load_card(card_path)` raises `CardLoadError` at `src/cacg/lint/layer1.py:313`; `run_layer1_checks()` extends with `exc.diagnostics` and immediately returns at `src/cacg/lint/layer1.py:316`. `CACG-CLI-001` is built in `card_loader.py` for non-file/unreadable cards (`src/cacg/card_loader.py:38`, `src/cacg/card_loader.py:55`). Frontmatter parse and validation diagnostics are built in `frontmatter.py` (`src/cacg/frontmatter.py:215`, `src/cacg/frontmatter.py:239`, `src/cacg/frontmatter.py:249`, `src/cacg/frontmatter.py:260`, `src/cacg/frontmatter.py:339`). | Per-card |
| `src/cacg/lint/layer1.py:332` | `C.MAN_MALFORMED` = `CACG-MAN-001` | error | Per-card manifest load/index fails: `_load_chunks_manifest()` or `_index_chunks()` raises `_ManifestLoadError`, caught at `src/cacg/lint/layer1.py:331`; appends and returns at `src/cacg/lint/layer1.py:338`. Duplicate `chunk_id`s raise the same path at `src/cacg/lint/layer1.py:71`. | Per-card |
| `src/cacg/lint/layer1.py:343` | `C.AUTH_MATRIX_INVALID` = `CACG-AUTH-000` | error | `auth is not None and auth.load_error is not None` at `src/cacg/lint/layer1.py:342`; this is appended once before citation checks. | Per-card |
| `src/cacg/lint/layer1.py:349` | all `_check_citations()` codes above: `CACG-CITE-001`, `CACG-CITE-002`, `CACG-RETR-002`, `CACG-RETR-003`, `CACG-CITE-004`, `CACG-CITE-006`, `CACG-HASH-001`, `CACG-CITE-005`, `CACG-AUTH-001`, `CACG-AUTH-002` | error | Extends `diagnostics` with the citation-loop result after card load, manifest/index load, and `AUTH-000` handling have completed (`src/cacg/lint/layer1.py:349`). | Per-citation |
| `src/cacg/lint/layer1.py:350` | `C.HASH_CARD_STALE` = `CACG-HASH-002` | error | Extends `diagnostics` with `_check_card_hash()` after all citation diagnostics have been added (`src/cacg/lint/layer1.py:350`). | Per-card |
| `src/cacg/lint/layer1.py:465` | `C.MAN_MALFORMED` = `CACG-MAN-001` | error | Batch-only cards-manifest preflight: `cards_manifest.json` exists but cannot be read or validated, caught at `src/cacg/lint/layer1.py:458`; constructs `_preflight` diagnostic with `file=str(cards_manifest_path)` at `src/cacg/lint/layer1.py:473`. | Per-batch preflight |
| `src/cacg/lint/layer1.py:490` | `_preflight` contents, currently `CACG-MAN-001` from `src/cacg/lint/layer1.py:465` | error | Batch aggregation extends `all_diags` before the card loop when `_preflight` is non-empty (`src/cacg/lint/layer1.py:489`, `src/cacg/lint/layer1.py:490`). | Per-batch preflight |
| `src/cacg/lint/layer1.py:512` | all per-card `lint_card()` diagnostics | error | Batch aggregation extends `all_diags` with one card's `lint_card()` diagnostics immediately after that card's journal append has occurred inside `lint_card()` (`src/cacg/lint/layer1.py:508`, `src/cacg/lint/layer1.py:512`). | Per-card |
| `src/cacg/lint/layer1.py:539` | `CACG-SKILL-001`, `CACG-SKILL-002`, `CACG-SKILL-003` | error | Batch-only auxiliary branch: `validate_skill_routers()` returns diagnostics after all cards have been linted (`src/cacg/lint/layer1.py:537`, `src/cacg/lint/layer1.py:538`). Real construction sites are in `src/cacg/skill_router.py`. | Per-router / batch aggregate |
| `src/cacg/lint/layer1.py:579` | `CACG-ROLE-001`, `CACG-ROLE-002`, `CACG-ROLE-003` | error | Batch-only auxiliary branch: `validate_role_maps()` returns diagnostics after all cards and SKILL checks (`src/cacg/lint/layer1.py:573`, `src/cacg/lint/layer1.py:578`). Real construction sites are in `src/cacg/role_map.py`. | Per-role-map / batch aggregate |
| `src/cacg/lint/layer1.py:592` | `CACG-DEP-001`, `CACG-DEP-002`, `CACG-DEP-003`, `CACG-DEP-004` | error | Batch-only auxiliary branch: `validate_card_dag()` returns diagnostics after role-map validation (`src/cacg/lint/layer1.py:585`, `src/cacg/lint/layer1.py:591`). Real construction sites are in `src/cacg/card_dag.py`. | Per-card-edge / batch aggregate |

Codes in the AC-2 trust-bearing subset that are not constructed directly by `src/cacg/lint/layer1.py`: `CACG-HASH-003`, `CACG-MAN-002`, `CACG-MAN-003`, `CACG-RETR-001`, and `CACG-JNL-001`. `CACG-CLI-001` is carried through `load_card()` into `run_layer1_checks()` at `src/cacg/lint/layer1.py:315`, but is constructed in `src/cacg/card_loader.py:39` and `src/cacg/card_loader.py:55`. `CACG-JNL-001` is not emitted in `layer1.py`; journal append failures would be raised out of `append_entry()` for a CLI layer to map.

## 3. Per-cited-chunk emission order (the trust contract)

Actual loop entry:

- Citations are visited in source order via `for i, cit in enumerate(doc.frontmatter.citations)` (`src/cacg/lint/layer1.py:109`).
- `loc` is `citations[{i}]`, so messages are index-stable (`src/cacg/lint/layer1.py:110`).
- Every failing branch in `_check_citations()` appends at most one diagnostic for that citation and then `continue`s, except the final auth branch, which appends and then naturally reaches the next citation (`src/cacg/lint/layer1.py:119`, `src/cacg/lint/layer1.py:130`, `src/cacg/lint/layer1.py:138`, `src/cacg/lint/layer1.py:155`, `src/cacg/lint/layer1.py:166`, `src/cacg/lint/layer1.py:176`, `src/cacg/lint/layer1.py:187`, `src/cacg/lint/layer1.py:199`, `src/cacg/lint/layer1.py:219`, `src/cacg/lint/layer1.py:247`).

Plan sequence check:

| Planned item | Actual fire site | Actual order | Confirm/refute |
|---|---|---:|---|
| malformed `chunk_id` | Condition at `src/cacg/lint/layer1.py:112`, append at `src/cacg/lint/layer1.py:113`, `continue` at `src/cacg/lint/layer1.py:119` | 1 | Confirmed. This is first in the loop. |
| placeholder hash | Condition at `src/cacg/lint/layer1.py:123`, append at `src/cacg/lint/layer1.py:124`, `continue` at `src/cacg/lint/layer1.py:130` | 2 | Confirmed. |
| retracted source | Condition at `src/cacg/lint/layer1.py:145`, append at `src/cacg/lint/layer1.py:146`, `continue` at `src/cacg/lint/layer1.py:155` | 4 | Confirmed relative to the plan, but the source contains an extra non-placeholder hash-format branch before this. |
| retracted chunk | Condition at `src/cacg/lint/layer1.py:156`, append at `src/cacg/lint/layer1.py:157`, `continue` at `src/cacg/lint/layer1.py:166` | 5 | Confirmed. |
| missing manifest chunk | Lookup at `src/cacg/lint/layer1.py:168`, condition at `src/cacg/lint/layer1.py:169`, append at `src/cacg/lint/layer1.py:170`, `continue` at `src/cacg/lint/layer1.py:176` | 6 | Confirmed. |
| source mismatch | Condition at `src/cacg/lint/layer1.py:180`, append at `src/cacg/lint/layer1.py:181`, `continue` at `src/cacg/lint/layer1.py:187` | 7 | Confirmed. |
| hash mismatch | Condition at `src/cacg/lint/layer1.py:189`, append at `src/cacg/lint/layer1.py:190`, `continue` at `src/cacg/lint/layer1.py:199` | 8 | Confirmed. |
| page disjoint | Page sets built at `src/cacg/lint/layer1.py:206` and `src/cacg/lint/layer1.py:207`, condition at `src/cacg/lint/layer1.py:208`, append at `src/cacg/lint/layer1.py:210`, `continue` at `src/cacg/lint/layer1.py:219` | 9 | Confirmed. |
| auth | Gate at `src/cacg/lint/layer1.py:224`, helper call at `src/cacg/lint/layer1.py:225`, failure condition at `src/cacg/lint/layer1.py:230`, append at `src/cacg/lint/layer1.py:241` | 10 | Confirmed. Auth is last and only runs after all earlier citation checks pass. |

Actual full per-citation order is therefore:

1. `CACG-CITE-001`: malformed `chunk_id` (`src/cacg/lint/layer1.py:112`, `src/cacg/lint/layer1.py:113`).
2. `CACG-CITE-002`: all-zero placeholder `chunk_hash` (`src/cacg/lint/layer1.py:123`, `src/cacg/lint/layer1.py:124`).
3. `CACG-CITE-002`: non-64-hex `chunk_hash` (`src/cacg/lint/layer1.py:131`, `src/cacg/lint/layer1.py:132`).
4. `CACG-RETR-002`: retracted `source_id` (`src/cacg/lint/layer1.py:145`, `src/cacg/lint/layer1.py:146`).
5. `CACG-RETR-003`: retracted `chunk_id` (`src/cacg/lint/layer1.py:156`, `src/cacg/lint/layer1.py:157`).
6. `CACG-CITE-004`: `chunk_id` absent from manifest (`src/cacg/lint/layer1.py:168`, `src/cacg/lint/layer1.py:170`).
7. `CACG-CITE-006`: citation `source_id` disagrees with manifest chunk `source_id` (`src/cacg/lint/layer1.py:180`, `src/cacg/lint/layer1.py:181`).
8. `CACG-HASH-001`: citation hash differs from manifest chunk hash (`src/cacg/lint/layer1.py:189`, `src/cacg/lint/layer1.py:190`).
9. `CACG-CITE-005`: cited page range is disjoint from actual chunk `page_spans` pages (`src/cacg/lint/layer1.py:206`, `src/cacg/lint/layer1.py:208`, `src/cacg/lint/layer1.py:210`).
10. `CACG-AUTH-001` or `CACG-AUTH-002`: authorization matrix rejects citation (`src/cacg/lint/layer1.py:224`, `src/cacg/lint/layer1.py:230`, `src/cacg/lint/layer1.py:241`).

Drift from the plan summary:

- The plan sequence is correct for the named items, but incomplete. Actual code has an additional `CACG-CITE-002` branch for non-64-hex hash format between "placeholder hash" and "retracted source" (`src/cacg/lint/layer1.py:131`, `src/cacg/lint/layer1.py:132`).
- For normal file-loaded cards, non-64-hex `chunk_hash` is usually caught earlier by schema/frontmatter mapping as `CACG-CITE-002` (`src/cacg/schema.py:267`, `src/cacg/schema.py:270`; `src/cacg/frontmatter.py:306`, `src/cacg/frontmatter.py:307`) and `run_layer1_checks()` returns from the `CardLoadError` branch before the citation loop (`src/cacg/lint/layer1.py:315`, `src/cacg/lint/layer1.py:316`). The Rust port should still mirror the branch if it ports `_check_citations()` as a helper over already-built structs.

## 4. Cross-cutting order (per-card)

`run_layer1_checks()` per-card order:

1. Initialize `diagnostics = []` and `card_hash_before = None` (`src/cacg/lint/layer1.py:310`, `src/cacg/lint/layer1.py:311`).
2. Load and parse the card with `load_card(card_path)` (`src/cacg/lint/layer1.py:313`).
3. If card load fails, extend with `exc.diagnostics` and return immediately (`src/cacg/lint/layer1.py:314`, `src/cacg/lint/layer1.py:315`, `src/cacg/lint/layer1.py:316`). This means frontmatter/CLI diagnostics short-circuit all manifest, auth, citation, and card-hash checks.
4. If card load succeeds, set `card_hash_before = doc.frontmatter.card_hash` (`src/cacg/lint/layer1.py:317`).
5. Build `_ChunkIndex` from supplied batch `chunks_index`, if present (`src/cacg/lint/layer1.py:318`, `src/cacg/lint/layer1.py:319`, `src/cacg/lint/layer1.py:323`).
6. Otherwise load and index `chunks_manifest.json` per-card (`src/cacg/lint/layer1.py:324`, `src/cacg/lint/layer1.py:326`, `src/cacg/lint/layer1.py:330`).
7. If manifest load/index fails, append `CACG-MAN-001` and return immediately (`src/cacg/lint/layer1.py:331`, `src/cacg/lint/layer1.py:332`, `src/cacg/lint/layer1.py:338`). This skips `AUTH-000`, the per-citation loop, and card-hash staleness.
8. If `auth.load_error` is present, append per-card `CACG-AUTH-000` before citations (`src/cacg/lint/layer1.py:342`, `src/cacg/lint/layer1.py:343`). This does not stop citation checks.
9. Extend with all per-citation diagnostics from `_check_citations()` (`src/cacg/lint/layer1.py:349`).
10. Extend with card-hash diagnostics from `_check_card_hash()` (`src/cacg/lint/layer1.py:350`).
11. Return diagnostics, card hash before, and parsed doc (`src/cacg/lint/layer1.py:351`).

Cross-cutting diagnostic interleave:

- Frontmatter and CLI diagnostics occur first and are exclusive: if `load_card()` fails, only loader diagnostics are returned from `run_layer1_checks()` (`src/cacg/lint/layer1.py:313`, `src/cacg/lint/layer1.py:315`, `src/cacg/lint/layer1.py:316`).
- Per-card manifest `CACG-MAN-001` occurs after successful card load but before auth, citations, and card hash (`src/cacg/lint/layer1.py:326`, `src/cacg/lint/layer1.py:332`, `src/cacg/lint/layer1.py:338`).
- Per-card `CACG-AUTH-000` occurs before the citation loop (`src/cacg/lint/layer1.py:342`, `src/cacg/lint/layer1.py:343`, `src/cacg/lint/layer1.py:349`).
- Per-citation `CACG-AUTH-001/002` occurs inside the citation loop, after all mechanical citation checks pass (`src/cacg/lint/layer1.py:224`, `src/cacg/lint/layer1.py:241`).
- Source/chunk retraction checks are inside the citation loop and occur before manifest-presence lookup (`src/cacg/lint/layer1.py:139`, `src/cacg/lint/layer1.py:145`, `src/cacg/lint/layer1.py:156`, `src/cacg/lint/layer1.py:168`).
- Card-hash staleness is always after all citation diagnostics (`src/cacg/lint/layer1.py:349`, `src/cacg/lint/layer1.py:350`).

Journal order:

- `lint_card()` starts timing before calling `run_layer1_checks()` (`src/cacg/lint/layer1.py:375`, `src/cacg/lint/layer1.py:376`).
- It computes `failed` after `run_layer1_checks()` has returned the complete diagnostic list (`src/cacg/lint/layer1.py:380`).
- It appends the journal entry after diagnostics and failure state are finalized (`src/cacg/lint/layer1.py:381`, `src/cacg/lint/layer1.py:388`, `src/cacg/lint/layer1.py:389`).
- In batch mode, each card's journal append occurs inside `lint_card()` during the sorted card loop (`src/cacg/lint/layer1.py:505`, `src/cacg/lint/layer1.py:508`, `src/cacg/lint/layer1.py:512`). Later SKILL/ROLE/DEP aggregate diagnostics are added to the returned `all_diags` after all per-card journal appends and are not journaled by `lint_directory()` (`src/cacg/lint/layer1.py:537`, `src/cacg/lint/layer1.py:539`, `src/cacg/lint/layer1.py:573`, `src/cacg/lint/layer1.py:579`, `src/cacg/lint/layer1.py:585`, `src/cacg/lint/layer1.py:592`).

## 5. KB_FROZEN_CLOCK / freeze_aware_latency integration

`layer1.py` imports `frozen` from `clock` at `src/cacg/lint/layer1.py:25`.

The local lint timing helper is:

- `freeze_aware_latency(start: float) -> float` at `src/cacg/lint/layer1.py:280`.
- It returns `0.0 if frozen() else (time.perf_counter() - start) * 1000.0` at `src/cacg/lint/layer1.py:282`.

`clock.frozen()` is:

- `frozen()` at `src/cacg/clock.py:16`.
- It checks `os.environ.get(FROZEN_ENV, "") == "1"` at `src/cacg/clock.py:17`, where `FROZEN_ENV = "KB_FROZEN_CLOCK"` at `src/cacg/clock.py:13`.

`lint_card()` wiring:

- Captures `start = time.perf_counter()` before running checks (`src/cacg/lint/layer1.py:375`).
- Passes `latency_ms=freeze_aware_latency(start)` into `JournalEntry` at `src/cacg/lint/layer1.py:390`.
- `append_entry()` copies `entry.latency_ms` into the serialized event payload at `src/cacg/journal.py:268` and `src/cacg/journal.py:279`.

Task-vh-14 Rust wiring requirement:

- Implement the Rust equivalent of `freeze_aware_latency(start)` with the exact frozen behavior: if `KB_FROZEN_CLOCK=1`, return JSON/event `latency_ms` as `0.0`, not an elapsed measurement.
- Capture the start time before the lint checks, matching `src/cacg/lint/layer1.py:375`.
- Call the freeze-aware helper at journal-entry construction time, matching `src/cacg/lint/layer1.py:390`.
- The journal path also freezes event UUID and timestamp through `new_uuid()` and `now_iso()` in Python (`src/cacg/journal.py:271`, `src/cacg/journal.py:272`; `src/cacg/clock.py:20`, `src/cacg/clock.py:26`). For byte-equal frozen parity, `latency_ms` must participate in the same frozen-clock discipline.

## 6. Auxiliary-codes branches the Rust port must OMIT (per AC-2.1)

These are Python lint codepaths that emit `CACG-SUM-*`, `CACG-SKILL-*`, `CACG-DEP-*`, or `CACG-ROLE-*`. The new `cacg-core::lint::layer1::run_layer1_checks` surface for task-vh-6 must not emit these from the new lint pass.

`CACG-SUM-*` path, carried through `layer1.py` card-load failure:

- `run_layer1_checks()` calls `load_card()` at `src/cacg/lint/layer1.py:313`.
- On `CardLoadError`, it extends diagnostics at `src/cacg/lint/layer1.py:315` and returns at `src/cacg/lint/layer1.py:316`.
- `frontmatter._map_pydantic_errors()` maps summary length errors:
  - `CACG-SUM-001` for `summary` `string_too_short` at `src/cacg/frontmatter.py:312`, `src/cacg/frontmatter.py:317`, `src/cacg/frontmatter.py:318`.
  - `CACG-SUM-002` for `summary` `string_too_long` at `src/cacg/frontmatter.py:312`, `src/cacg/frontmatter.py:320`, `src/cacg/frontmatter.py:321`.
  - Both are appended at `src/cacg/frontmatter.py:339`.
- Tag errors:
  - `CACG-SUM-004` for oversized tag list by message heuristic at `src/cacg/frontmatter.py:323`, `src/cacg/frontmatter.py:330`, `src/cacg/frontmatter.py:331`.
  - `CACG-SUM-003` for tag non-conformance otherwise at `src/cacg/frontmatter.py:323`, `src/cacg/frontmatter.py:333`, `src/cacg/frontmatter.py:334`.
  - Both are appended at `src/cacg/frontmatter.py:339`.

Reminder for task-vh-6: the existing M1 Rust schema/frontmatter modules may still emit `CACG-SUM-*` from their own parity surface. The omit rule here is bounded to the new lint-pass surface corresponding to `run_layer1_checks()`.

`CACG-SKILL-*` path, batch-only through `lint_directory()`:

- `lint_directory()` imports `validate_skill_routers()` at `src/cacg/lint/layer1.py:424`.
- It skips `SKILL.md` files during normal card lint at `src/cacg/lint/layer1.py:506` and `src/cacg/lint/layer1.py:507`.
- It runs `validate_skill_routers(cards_dir_p, active_ids_by_reading)` after all card linting at `src/cacg/lint/layer1.py:537`.
- It extends aggregate diagnostics at `src/cacg/lint/layer1.py:539`.

Real SKILL construction sites:

- `CACG-SKILL-003` missing YAML frontmatter: `src/cacg/skill_router.py:116`, `src/cacg/skill_router.py:118`.
- `CACG-SKILL-003` YAML parse error: `src/cacg/skill_router.py:129`, `src/cacg/skill_router.py:131`.
- `CACG-SKILL-003` frontmatter top-level not mapping: `src/cacg/skill_router.py:139`, `src/cacg/skill_router.py:141`.
- `CACG-SKILL-003` Pydantic schema violation per error: `src/cacg/skill_router.py:155`, `src/cacg/skill_router.py:157`, `src/cacg/skill_router.py:159`.
- `CACG-SKILL-003` invalid router path placement: `src/cacg/skill_router.py:211`, `src/cacg/skill_router.py:213`.
- `CACG-SKILL-003` unreadable router file: `src/cacg/skill_router.py:226`, `src/cacg/skill_router.py:228`.
- Schema diagnostics are extended into the router validator result at `src/cacg/skill_router.py:236`.
- `CACG-SKILL-002` unknown/retracted/cross-reading `routes_to` target: `src/cacg/skill_router.py:244`, `src/cacg/skill_router.py:246`, `src/cacg/skill_router.py:248`.
- `CACG-SKILL-001` router name collision, one diagnostic per offender path: `src/cacg/skill_router.py:259`, `src/cacg/skill_router.py:263`, `src/cacg/skill_router.py:265`.

`CACG-ROLE-*` path, batch-only through `lint_directory()`:

- `lint_directory()` imports `validate_role_maps()` at `src/cacg/lint/layer1.py:553`.
- It builds `role_active_by_reading` from `cards_manifest.cards` when available at `src/cacg/lint/layer1.py:555`, `src/cacg/lint/layer1.py:560`, `src/cacg/lint/layer1.py:564`.
- It runs `validate_role_maps()` at `src/cacg/lint/layer1.py:573`.
- It extends aggregate diagnostics at `src/cacg/lint/layer1.py:579`.

Real ROLE construction sites:

- `CACG-ROLE-003` role-map schema/load/vocab violation: `src/cacg/role_map.py:195`, `src/cacg/role_map.py:198`, `src/cacg/role_map.py:200`.
- `CACG-ROLE-003` filename stem does not match `reading_id`: `src/cacg/role_map.py:209`, `src/cacg/role_map.py:210`, `src/cacg/role_map.py:212`.
- `CACG-ROLE-001` entry references non-active card, retracted card, or unknown card: `src/cacg/role_map.py:237`, `src/cacg/role_map.py:241`, `src/cacg/role_map.py:245`, `src/cacg/role_map.py:247`.
- `CACG-ROLE-002` active card missing from role map: `src/cacg/role_map.py:259`, `src/cacg/role_map.py:260`, `src/cacg/role_map.py:261`, `src/cacg/role_map.py:263`.

`CACG-DEP-*` path, batch-only through `lint_directory()`:

- `lint_directory()` imports `validate_card_dag()` at `src/cacg/lint/layer1.py:501`.
- It builds `dag_nodes` during the card loop at `src/cacg/lint/layer1.py:504`, `src/cacg/lint/layer1.py:528`.
- It runs `validate_card_dag()` after role-map validation at `src/cacg/lint/layer1.py:585`.
- It extends aggregate diagnostics at `src/cacg/lint/layer1.py:592`.

Real DEP construction sites:

- `CACG-DEP-004` edge target is retracted: `src/cacg/card_dag.py:174`, `src/cacg/card_dag.py:175`, `src/cacg/card_dag.py:177`; this branch `continue`s before unknown-target handling at `src/cacg/card_dag.py:187`.
- `CACG-DEP-001` edge target is not in active nodes: `src/cacg/card_dag.py:188`, `src/cacg/card_dag.py:189`, `src/cacg/card_dag.py:191`.
- `CACG-DEP-002` cycle member: `src/cacg/card_dag.py:207`, `src/cacg/card_dag.py:208`, `src/cacg/card_dag.py:216`, `src/cacg/card_dag.py:218`.
- `CACG-DEP-003` orphan card when `check_orphans` is true: `src/cacg/card_dag.py:234`, `src/cacg/card_dag.py:240`, `src/cacg/card_dag.py:242`.

## 7. Surprises and ordering nuances

- `_check_citations()` emits at most one diagnostic per citation because every mechanical failure branch appends and `continue`s (`src/cacg/lint/layer1.py:119`, `src/cacg/lint/layer1.py:130`, `src/cacg/lint/layer1.py:138`, `src/cacg/lint/layer1.py:155`, `src/cacg/lint/layer1.py:166`, `src/cacg/lint/layer1.py:176`, `src/cacg/lint/layer1.py:187`, `src/cacg/lint/layer1.py:199`, `src/cacg/lint/layer1.py:219`).
- A malformed `chunk_id` suppresses even placeholder hash detection for that same citation because it is first and continues (`src/cacg/lint/layer1.py:112`, `src/cacg/lint/layer1.py:119`).
- All-zero `chunk_hash` is treated as `CACG-CITE-002` even though it is valid 64-hex; it fires before the generic `_HEX64` check (`src/cacg/lint/layer1.py:123`, `src/cacg/lint/layer1.py:131`).
- Non-64-hex hash format exists in `_check_citations()` but is usually unreachable through file-loaded cards because schema validation maps citation `chunk_hash` failures to `CACG-CITE-002` before `run_layer1_checks()` reaches citation checks (`src/cacg/frontmatter.py:306`, `src/cacg/frontmatter.py:307`; `src/cacg/lint/layer1.py:315`, `src/cacg/lint/layer1.py:316`).
- Source/chunk retraction intentionally fires before manifest presence, because retracted chunks are removed from active `chunks_manifest.chunks`; the comment pins that ordering (`src/cacg/lint/layer1.py:139`, `src/cacg/lint/layer1.py:145`, `src/cacg/lint/layer1.py:156`, `src/cacg/lint/layer1.py:168`).
- Manifest load/index failure short-circuits `AUTH-000`, all citation diagnostics, and `HASH-002` (`src/cacg/lint/layer1.py:331`, `src/cacg/lint/layer1.py:338`, `src/cacg/lint/layer1.py:342`, `src/cacg/lint/layer1.py:349`, `src/cacg/lint/layer1.py:350`).
- `AUTH-000` does not suppress citation checks. It is appended once per card, then `_check_citations()` still runs; because `auth.matrix is None`, per-citation auth is skipped (`src/cacg/lint/layer1.py:342`, `src/cacg/lint/layer1.py:343`, `src/cacg/lint/layer1.py:224`).
- `AUTH-001/002` is last in the per-citation loop. A citation with a hash mismatch or page disjointness will not also emit unauthorized-source diagnostics (`src/cacg/lint/layer1.py:189`, `src/cacg/lint/layer1.py:208`, `src/cacg/lint/layer1.py:224`).
- Card-hash staleness is after citation diagnostics, so `HASH-002` follows all per-citation diagnostics for that card (`src/cacg/lint/layer1.py:349`, `src/cacg/lint/layer1.py:350`).
- Cards with no `card_hash` silently pass `_check_card_hash()` (`src/cacg/lint/layer1.py:258`, `src/cacg/lint/layer1.py:259`, `src/cacg/lint/layer1.py:260`).
- Batch `ChunksIndex` preload failure is deliberately swallowed into `chunks_index = None`; then every card retries the manifest load/index path and can emit per-card `CACG-MAN-001` while preserving one journal entry per card (`src/cacg/lint/layer1.py:432`, `src/cacg/lint/layer1.py:434`, `src/cacg/lint/layer1.py:435`, `src/cacg/lint/layer1.py:508`).
- Duplicate `chunk_id`s are mapped to `CACG-MAN-001` through `_index_chunks()` in the per-card path (`src/cacg/lint/layer1.py:66`, `src/cacg/lint/layer1.py:71`, `src/cacg/lint/layer1.py:332`), and through `ChunksIndex.from_path()` in the batch preload path before falling back to per-card loads (`src/cacg/chunks_index.py:75-84` for `from_path` which delegates at line 84, and `src/cacg/chunks_index.py:57-67` for the actual duplicate detection + raise inside `from_manifest`; the raise happens at `src/cacg/chunks_index.py:64`; `src/cacg/lint/layer1.py:434`).
- `cards_manifest.json` preflight failure in `lint_directory()` adds a batch aggregate `CACG-MAN-001` before card diagnostics, with `file` set to the manifest path, not the card path (`src/cacg/lint/layer1.py:464`, `src/cacg/lint/layer1.py:473`, `src/cacg/lint/layer1.py:490`). This diagnostic is not written to per-card journals by `lint_directory()`.
- `SKILL.md` files are filtered out of the normal card walk, so they do not produce card frontmatter diagnostics from `load_card()` (`src/cacg/lint/layer1.py:505`, `src/cacg/lint/layer1.py:506`, `src/cacg/lint/layer1.py:507`).
- SKILL, ROLE, and DEP diagnostics happen only after all cards have already been linted and journaled in batch mode (`src/cacg/lint/layer1.py:508`, `src/cacg/lint/layer1.py:537`, `src/cacg/lint/layer1.py:573`, `src/cacg/lint/layer1.py:585`).
- `lint_directory()` returns auxiliary diagnostics in aggregate but does not append a separate journal event for them (`src/cacg/lint/layer1.py:539`, `src/cacg/lint/layer1.py:579`, `src/cacg/lint/layer1.py:592`, `src/cacg/lint/layer1.py:595`).
- Deterministic batch order comes from sorted card paths, sorted SKILL paths, sorted role-map paths, and sorted DAG node/card ids (`src/cacg/lint/layer1.py:505`; `src/cacg/skill_router.py:205`; `src/cacg/role_map.py:192`; `src/cacg/card_dag.py:166`, `src/cacg/card_dag.py:208`).

## 8. Port checklist for task-vh-6

- Implement the pure Rust lint surface to match `run_layer1_checks()`, not `lint_directory()` auxiliary behavior. The pure Python contract starts at `src/cacg/lint/layer1.py:285` and explicitly does not append journals at `src/cacg/lint/layer1.py:306`.
- Preserve top-level per-card order exactly: load card, short-circuit on loader diagnostics, set `card_hash_before`, load/index chunks manifest, short-circuit on `CACG-MAN-001`, append `CACG-AUTH-000`, run citation loop, then run card-hash stale check (`src/cacg/lint/layer1.py:313`, `src/cacg/lint/layer1.py:315`, `src/cacg/lint/layer1.py:317`, `src/cacg/lint/layer1.py:326`, `src/cacg/lint/layer1.py:332`, `src/cacg/lint/layer1.py:343`, `src/cacg/lint/layer1.py:349`, `src/cacg/lint/layer1.py:350`).
- Preserve the exact per-citation order: `CITE-001`, `CITE-002` placeholder, `CITE-002` non-hex helper branch, `RETR-002`, `RETR-003`, `CITE-004`, `CITE-006`, `HASH-001`, `CITE-005`, `AUTH-001/002` (`src/cacg/lint/layer1.py:112`, `src/cacg/lint/layer1.py:123`, `src/cacg/lint/layer1.py:131`, `src/cacg/lint/layer1.py:145`, `src/cacg/lint/layer1.py:156`, `src/cacg/lint/layer1.py:168`, `src/cacg/lint/layer1.py:180`, `src/cacg/lint/layer1.py:189`, `src/cacg/lint/layer1.py:208`, `src/cacg/lint/layer1.py:224`).
- Preserve one-diagnostic-per-citation short-circuiting. Do not accumulate multiple mechanical failures for the same citation.
- Use actual `chunk.page_spans[*].page` for `CACG-CITE-005`, not `start_page..end_page` interval approximation (`src/cacg/lint/layer1.py:206`, `src/cacg/lint/layer1.py:207`, `src/cacg/lint/layer1.py:208`).
- Preserve diagnostic `file` fields: citation and card-hash diagnostics use `str(card_path)` passed into helpers (`src/cacg/lint/layer1.py:349`, `src/cacg/lint/layer1.py:350`); per-card manifest and auth-matrix diagnostics also use `str(card_path)` (`src/cacg/lint/layer1.py:336`, `src/cacg/lint/layer1.py:347`).
- Map manifest load, validation, and duplicate `chunk_id` failures to `CACG-MAN-001` and return immediately (`src/cacg/lint/layer1.py:96`, `src/cacg/lint/layer1.py:71`, `src/cacg/lint/layer1.py:331`, `src/cacg/lint/layer1.py:338`).
- Implement source/chunk retraction from `chunks_manifest.retracted_source_ids` and `chunks_manifest.retracted_chunk_ids` before manifest active-chunk lookup (`src/cacg/lint/layer1.py:76`, `src/cacg/lint/layer1.py:145`, `src/cacg/lint/layer1.py:156`, `src/cacg/lint/layer1.py:168`).
- Implement `CACG-AUTH-000` as per-card and before citations; implement `CACG-AUTH-001/002` as per-citation and last in the citation loop (`src/cacg/lint/layer1.py:342`, `src/cacg/lint/layer1.py:343`, `src/cacg/lint/layer1.py:224`, `src/cacg/lint/layer1.py:241`).
- Implement `CACG-HASH-002` only when `card_hash` is present and stale; absent card hash passes silently (`src/cacg/lint/layer1.py:258`, `src/cacg/lint/layer1.py:260`, `src/cacg/lint/layer1.py:263`).
- Do not emit `CACG-SUM-*`, `CACG-SKILL-*`, `CACG-DEP-*`, or `CACG-ROLE-*` from the new Rust lint-pass surface. The Python sources for those deferred branches are identified in section 6.
- Keep journal append outside `run_layer1_checks()`. If Rust also ports `lint_card()`, append exactly once after diagnostics are complete and failure state is computed (`src/cacg/lint/layer1.py:376`, `src/cacg/lint/layer1.py:380`, `src/cacg/lint/layer1.py:381`).
- Wire `freeze_aware_latency` so `KB_FROZEN_CLOCK=1` yields `latency_ms: 0.0` exactly (`src/cacg/lint/layer1.py:280`, `src/cacg/lint/layer1.py:282`, `src/cacg/lint/layer1.py:390`; `src/cacg/clock.py:16`, `src/cacg/clock.py:17`).
- For batch parity later, preserve sorted traversal and per-card journal cardinality: sorted markdown paths, skip `SKILL.md`, call per-card `lint_card()`, then only aggregate SKILL/ROLE/DEP diagnostics after all cards (`src/cacg/lint/layer1.py:505`, `src/cacg/lint/layer1.py:506`, `src/cacg/lint/layer1.py:508`, `src/cacg/lint/layer1.py:537`, `src/cacg/lint/layer1.py:573`, `src/cacg/lint/layer1.py:585`).
