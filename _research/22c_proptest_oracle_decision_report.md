# AC-6 Proptest Oracle Decision Report

_Task: `task-pyret-19` from `.humanize/.humanize/plans/cacg-python-retirement-plan.md`._

This report fixes the AC-6 path choice for the five Rust proptest oracles that currently spawn `legacy_python_oracle/scripts/*_oracle.py`. The implementation tasks that follow this report must remove the live Python subprocesses from the Rust tests; Python is allowed only for one-time Path-B corpus capture before the resulting fixture is committed.

## Shared AC-6 gate

After `task-pyret-20` through `task-pyret-24`, the common no-spawn proof is:

```bash
rg -n 'legacy_python_oracle/scripts/(canonical_json_oracle|normalize_oracle|bounded_levenshtein_oracle|journal_oracle|history_oracle)\.py|\.venv/bin/python|std::process::Command|Command::new|Stdio::piped|python_oracle|python_distances|resolve_python|cacg_importable|run_oracle_single' \
  crates/cacg-core/tests/canonical_json_proptest.rs \
  crates/cacg-core/tests/normalize_proptest.rs \
  crates/cacg-core/tests/bounded_levenshtein_proptest.rs \
  crates/cacg-core/tests/journal_1000_byte_equal.rs \
  crates/cacg-core/tests/history_parity.rs
```

Expected result after AC-6: no matches. The behavioral gate remains:

```bash
cargo test -p cacg-core --test canonical_json_proptest
cargo test -p cacg-core --test normalize_proptest
cargo test -p cacg-core --test bounded_levenshtein_proptest
cargo test -p cacg-core --test journal_1000_byte_equal
cargo test -p cacg-core --test history_parity
```

## 1. `canonical_json_proptest.rs`

**Path choice: Path A, Rust-native independent oracle.** This is a pure deterministic serialization algorithm with a published behavioral contract: Python `json.dumps(sort_keys=True, separators=(',', ':'), ensure_ascii=False)` over the CACG JSON domain. A frozen corpus would unnecessarily reduce the randomized 10,000-case coverage.

**Independent reference algorithm:** keep the seeded 10,000-case generator, but replace `python_oracle` with a test-local reference writer, for example `mod reference_canonical_json` inside `crates/cacg-core/tests/canonical_json_proptest.rs`. The reference writer recursively serializes `serde_json::Value`: `null`/booleans as literals; integers via decimal formatting; finite floats through an independently written Python-`repr`-compatible formatter; strings with JSON escaping for quote, backslash, named controls, and lowercase `\u00xx` for other C0 controls while leaving non-ASCII scalar values literal; arrays with comma separators and no spaces; objects by collecting key/value pairs, sorting keys by Unicode scalar order, and writing `key:value` pairs without spaces.

**Production function that must not be delegated to:** `cacg_core::canonical_json::canonical_json()` and `canonical_json_from_str()`.

**New oracle test location:** `crates/cacg-core/tests/canonical_json_proptest.rs` as a test-local reference module. Do not move the oracle into `cacg_core` production modules.

**Negative control:** on a sacrificial branch, mutate production canonicalization without touching the test-local reference. Minimum mutations: remove object-key sorting, change C0 control escaping from lowercase `\u00xx` to raw or uppercase, or change float exponent padding. `cargo test -p cacg-core --test canonical_json_proptest` must fail. A static guard should also reject importing or calling `cacg_core::canonical_json::canonical_json` from inside the reference module.

**Validation command to prove no Python spawn remains:**

```bash
rg -n 'legacy_python_oracle/scripts/canonical_json_oracle\.py|\.venv/bin/python|std::process::Command|Command::new|Stdio::piped|python_oracle' crates/cacg-core/tests/canonical_json_proptest.rs
```

Expected: no matches.

## 2. `normalize_proptest.rs`

**Path choice: Path A, Rust-native independent oracle.** Normalization is a closed five-step string algorithm. It should remain randomized at 10,000 cases and should not be compressed into a finite committed corpus.

**Independent reference algorithm:** keep the existing seeded string strategy and implement a test-local reference normalizer, for example `mod reference_normalize` in `crates/cacg-core/tests/normalize_proptest.rs`. The reference algorithm is: Unicode NFC; replace the seven Latin ligatures `U+FB00..U+FB06` with `ff`, `fi`, `fl`, `ffi`, `ffl`, `ft`, `st`; remove hyphenated line breaks matching Python semantics for `-\s*\n\s*` using an explicit 29-codepoint Python `re` whitespace table; collapse one or more Python-whitespace codepoints to a single ASCII space; strip leading and trailing Python-whitespace codepoints. It may use `unicode_normalization` for NFC, but the whitespace predicate and hyphen-linebreak handling must be local to the test oracle.

**Production function that must not be delegated to:** `cacg_core::normalize::normalize_text()`; the reference must also avoid delegating to `cacg_core::normalize::is_python_re_whitespace()`.

**New oracle test location:** `crates/cacg-core/tests/normalize_proptest.rs` as a test-local reference module.

**Negative control:** on a sacrificial branch, mutate production normalization only: remove one ligature mapping, drop `U+001C..U+001F` from the whitespace predicate, or change hyphen-linebreak consumption so trailing whitespace after the newline is preserved. `cargo test -p cacg-core --test normalize_proptest` must fail while the reference implementation stays unchanged.

**Validation command to prove no Python spawn remains:**

```bash
rg -n 'legacy_python_oracle/scripts/normalize_oracle\.py|\.venv/bin/python|std::process::Command|Command::new|Stdio::piped|python_oracle|cacg_importable' crates/cacg-core/tests/normalize_proptest.rs
```

Expected: no matches.

## 3. `bounded_levenshtein_proptest.rs`

**Path choice: Path A, Rust-native independent oracle.** Levenshtein distance is a pure algorithm, and the current Python oracle is just full O(m*n) dynamic programming. The Rust test should keep its 10,000 randomized cases and compare the bounded production function against an unbounded independent reference.

**Independent reference algorithm:** keep the existing seeded case strategy and add a test-local `reference_levenshtein(a, b) -> u32`. Convert both strings to `Vec<char>` so the unit of edit distance is a Unicode scalar value, not a byte. Use a straightforward Wagner-Fischer matrix or two full rows with insertion, deletion, and substitution cost 1. Do not apply threshold pruning inside the reference. The expected production result is `Some(distance)` when `distance <= threshold`, otherwise `None`.

**Production function that must not be delegated to:** `cacg_core::verify::fuzzy::bounded_levenshtein()`.

**New oracle test location:** `crates/cacg-core/tests/bounded_levenshtein_proptest.rs` as a test-local reference function/module.

**Negative control:** on a sacrificial branch, mutate production `bounded_levenshtein` only: remove the length-difference short-circuit, make substitution cost 2, operate over bytes instead of chars, or return the threshold instead of the actual distance. `cargo test -p cacg-core --test bounded_levenshtein_proptest` must fail. The existing ASCII subset cross-check can stay as an extra guard, but it is not the primary oracle.

**Validation command to prove no Python spawn remains:**

```bash
rg -n 'legacy_python_oracle/scripts/bounded_levenshtein_oracle\.py|\.venv/bin/python|std::process::Command|Command::new|Stdio::piped|python_distances|OracleOut' crates/cacg-core/tests/bounded_levenshtein_proptest.rs
```

Expected: no matches.

## 4. `journal_1000_byte_equal.rs`

**Path choice: Path B, committed cross-language corpus freeze.** Journal append parity is a byte-level data contract over sequencing, checksums, frozen clock fields, canonical JSON field order, and journal validation. The Python behavior should be captured once and then treated as a fixture, not reimplemented as another production-like journal writer in the test.

**Corpus fixture path:** commit a fixture directory at `tests/parity_corpus/proptest_oracles/journal_1000_byte_equal/` with:

- `manifest.json`: schema version, case count, fixture hashes, and branch fingerprints.
- `entries.jsonl`: the 1000 input `JournalEntry`-like dictionaries.
- `expected_lint_journal.jsonl`: the exact Python-produced journal bytes.

**Minimum case count and branch coverage fingerprints:** minimum `case_count == 1000`. The manifest must fingerprint at least these branches: first event has `seq=0` and `prev_checksum=null`; all later events have `prev_checksum == previous.event_checksum`; `seq` spans `0..999`; every line has frozen `event_id=00000000-0000-0000-0000-000000000000` and `timestamp=1970-01-01T00:00:00Z`; verification map covers `fuzzy=true/false`, `layer2=true/false`, and `layer1=true`; `latency_ms` covers `0.0..9.0`; optional card hashes cover both null and non-null values; diagnostics cover empty and non-empty lists. The gate should record SHA-256 for both `entries.jsonl` and `expected_lint_journal.jsonl`.

**One-time capture command:** implement the capture hook in `task-pyret-23`, then run:

```bash
KB_FROZEN_CLOCK=1 cargo test -p cacg-core --test journal_1000_byte_equal capture_journal_1000_byte_equal_corpus -- --ignored --nocapture
```

That ignored capture test is the only allowed Python spawn for this oracle after this report. It must call `legacy_python_oracle/scripts/journal_oracle.py`, write the fixture directory above, and be removed or left permanently ignored and excluded from normal gates after the committed fixture is reviewed.

**Mutation-test gate description:** normal tests read `entries.jsonl` and `expected_lint_journal.jsonl`, build the journal with `cacg_core::journal::append_entry()`, and byte-compare against the fixture. A structural fixture test must fail if `case_count`, hashes, or any branch fingerprint is wrong. A mutation test must also fail if a committed expected line is changed, if one `prev_checksum` is broken, or if production append changes field order, `seq`, frozen timestamp/event ID, or checksum calculation.

**Validation command to prove no Python spawn remains:**

```bash
rg -n 'legacy_python_oracle/scripts/journal_oracle\.py|\.venv/bin/python|std::process::Command|Command::new|Stdio::piped|resolve_python|cacg_importable' crates/cacg-core/tests/journal_1000_byte_equal.rs
```

Expected after the normal test rewrite: no matches.

## 5. `history_parity.rs`

**Path choice: Path B, committed cross-language corpus freeze.** History append parity is byte-level and branch-heavy: sequence numbers, checksum chaining, tombstone convention, optional/default fields, and path mapping are the contract. A committed corpus is the right oracle because coverage shape matters more than a second live implementation.

**Corpus fixture path:** commit a fixture directory at `tests/parity_corpus/proptest_oracles/history_parity/` with:

- `manifest.json`: schema version, case count, fixture hashes, and branch fingerprints.
- `cases/50_event/entries.jsonl` and `cases/50_event/expected_card.history.jsonl`.
- `cases/retraction_tombstone/entries.jsonl` and `expected_card.history.jsonl`.
- `cases/empty_new_card_hash/entries.jsonl` and `expected_card.history.jsonl`.
- `cases/omitted_is_retracted_marker/entries.jsonl` and `expected_card.history.jsonl`.

**Minimum case count and branch coverage fingerprints:** minimum `50` entries in the primary chain plus the three singleton branch fixtures above. The manifest must fingerprint at least these branches: first event has `seq=0` and `prev_checksum=null`; chained events have `prev_checksum == previous.event_checksum`; `prev_card_hash` covers null and non-null; `is_retracted` covers true and false; tombstone shape covers `frontmatter_field_changes=["__cacg_retracted__"]`, empty `added`/`removed`, and `new_card_hash == prev_card_hash`; marker-without-`is_retracted` remains false; empty-string `new_card_hash` is accepted and serialized; `cited_chunk_ids_snapshot` preserves array order; `frontmatter_snapshot` covers empty and non-empty objects; `cited_chunk_set_delta` covers empty and non-empty added/removed arrays. The manifest should hash every input and expected-output file.

**One-time capture command:** implement the capture hook in `task-pyret-24`, then run:

```bash
KB_FROZEN_CLOCK=1 cargo test -p cacg-core --test history_parity capture_history_parity_corpus -- --ignored --nocapture
```

That ignored capture test is the only allowed Python spawn for this oracle after this report. It must call `legacy_python_oracle/scripts/history_oracle.py`, write the fixture directory above, and be removed or left permanently ignored and excluded from normal gates after the committed fixture is reviewed.

**Mutation-test gate description:** normal tests read each committed case, append with `cacg_core::history::append_history_event()`, and byte-compare to the fixture. A structural fixture test must fail on missing cases, wrong case count, wrong hashes, or missing branch fingerprints. Mutation tests must fail if a committed expected line changes, if a chained checksum is broken, if tombstone marker behavior changes, if omitted `is_retracted` is promoted to true, if empty `new_card_hash` is rejected, or if array order is sorted during serialization.

**Validation command to prove no Python spawn remains:**

```bash
rg -n 'legacy_python_oracle/scripts/history_oracle\.py|\.venv/bin/python|std::process::Command|Command::new|Stdio::piped|resolve_python|cacg_importable|run_oracle_single' crates/cacg-core/tests/history_parity.rs
```

Expected after the normal test rewrite: no matches.
