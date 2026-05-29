# Rust Refactor Research — CACG (Content-Addressable Card Graph)

**Date:** 2026-05-20
**Status:** Research / scoping. No code changes implied.
**Scope:** Clean-slate refactor of the mature Python MVP (~11k LoC, ~640 tests, Pydantic v2 + pypdfium2 + rank-bm25 + sqlite FTS5) to a Rust implementation that crushes the current per-card budgets while preserving the existing trust boundaries, schema, and byte-determinism contracts.

**Method:** Read every source file of the current CACG implementation; surveyed the legacy `../CFA_reading` patterns CACG inherits from; consulted Codex (`gpt-5.5:high`, single-shot architecture consultation, output preserved at `.humanize/skill/2026-05-20_15-22-42-*/output.md`); cross-verified every recommended crate version via the `crates.io` JSON API on 2026-05-20.

---

## 0. TL;DR — Top 3 Architectural Decisions

1. **Pin a Pdfium build, not just a Rust binding.** Chunk hashes in production manifests are SHA-256 over canonical-JSON envelopes that bind extracted PDF text bytes. The chunk-hash determinism oracle is *Pdfium's text extractor*, NOT the language binding. `pdfium-render 0.9.1` is the right crate, but byte-identity with `pypdfium2 5.8.0`'s `get_text_range()` is not automatic — golden byte tests against the existing Python corpus are MANDATORY before any new chunk hashes are emitted from Rust. The `cacg-ingest` crate isolates Pdfium FFI; every other crate forbids the dependency (see §6 workspace).

2. **Treat determinism as a first-class API, not a runtime flag.** Roll our own canonical JSON (recursive sort + custom escape writer matching Python's `json.dumps(sort_keys=True, separators=(",",":"), ensure_ascii=False)` byte-for-byte). Inject a `DeterminismContext { clock, id_gen }` everywhere instead of an ambient `KB_FROZEN_CLOCK=1` env var. Sort outputs everywhere they cross the IO boundary. Snapshot-test journals / manifests / search bytes against the existing Python golden corpus before declaring parity.

3. **Use Rust crates for plumbing; OWN the oracle-critical logic.** Canonical JSON, BM25 scoring + tiebreaks, journal append discipline, multi-file pair-atomic publish, and strict YAML frontmatter validation are all things where a community crate's defaults will silently diverge from the documented CACG contracts. Implement these in `cacg-core` as ~50-200 LoC modules with their own snapshot tests against Python output. Use third-party crates for SHA-256 (`sha2`), Levenshtein DP (`strsim` or vendored), CLI parsing (`clap`), SQLite (`rusqlite`), and so on — but every byte that gets hashed or written to a journal goes through CACG's own code.

**Expected performance gain (rough order):**

| Operation | Python today | Rust target | Multiplier |
|-----------|--------------|-------------|------------|
| `kb verify` per-card median | low ms | sub-millisecond | ~10-30× |
| 1000-card `kb verify --round-summary` | 555 ms | < 50 ms | ~10× |
| `kb search` 1000-card p95 | < 500 ms | < 50 ms | ~10× |
| 10k FTS5 cold-open + query | ~ tens of ms | < 5 ms | ~5-10× |
| Warm-cache `kb index` 10k card | 725 ms | < 100 ms | ~7× |
| Cold `kb ingest` (PDF parse-bound) | ~unchanged | ~unchanged | ~1× |

Rust does NOT speed up `kb ingest` materially — that path is dominated by Pdfium native code (already C++). The wins are in the per-card hot loops where Python interpreter + Pydantic + YAML overhead dominate.

---

## 1. Inventory of the Python Surface to Replicate

### 1.1 CLI Surface (`kb` binary)

13 subcommands, all argparse-based, exit codes uniform (0/1/2):

| Subcommand | Purpose | PDF? | Notes |
|------------|---------|------|-------|
| `ingest <pdf>` | PDF → chunks/sources manifests | YES | pypdfium2 5.8.0 pinned |
| `new <reading> <slug>` | Scaffold card from template | no | |
| `lint [card|--all-readings]` | Layer-1 mechanical lint | no | `--source-matrix` MANDATORY |
| `verify [card|--round-summary]` | Layer-1 + Layer-2 verify; round-summary mode | no | `--source-matrix` MANDATORY, `--fuzzy`, `--semantic`, `--semantic-judge` |
| `index [cards_dir]` | Atomic publish cards_manifest + INDEX + summaries + sqlite | no | content-hashed Phase-D skip |
| `history <card>` | Print/validate per-card history.jsonl | no | |
| `retract <card>` | Card-level retraction | no | tombstone history event |
| `retract-source <id>` | Source-level retraction | no | |
| `retract-chunk <id>` | Chunk-level retraction | no | |
| `scaffold-matrix` | Bootstrap source_matrix.json | no | |
| `scaffold-role-map --reading <id>` | Bootstrap role-map artifact | no | |
| `search <query>` | BM25 / FTS5 retrieval | no | `--source-matrix` MANDATORY |
| `migrate-summaries` | Phase-3 schema migration | no | `--auto-heuristic` / `--strict` |

Planned for Phase 5: `kb show <card_id>` (read primitive, new CACG-SHOW-001/002 codes).

### 1.2 Persistent Artifacts (`cacg.v0` schema, strict Pydantic, `extra="forbid"`)

| Artifact | Lifecycle | Pair-atomic? |
|----------|-----------|--------------|
| `out/sources_manifest.json` | Built by `kb ingest` | Yes (with chunks) |
| `out/chunks_manifest.json` | Built by `kb ingest`; `chunks[]` + `retracted_source_ids[]` + `retracted_chunk_ids[]` with disjointness invariants | Yes (with sources) |
| `cards/<reading>/<slug>.md` | Authored by hand; updated in-place by `kb index` for `card_hash` | No |
| `out/cards_manifest.json` | Built by `kb index`; `cards[]` + `retracted_cards[]` + `dependency_retracted_cards[]` | Yes (4-way) |
| `out/INDEX.md` | Built by `kb index`, human-readable companion | Yes (4-way) |
| `out/summaries.json` | Built by `kb index`, eager-load card-of-cards | Yes (4-way) |
| `out/summaries.sqlite` | Built by `kb index`, FTS5 sidecar with sealed meta table | Yes (4-way) |
| `out/lint_journal.jsonl` | Append-only, tamper-evident chain | n/a |
| `cards/.../<slug>.history.jsonl` | Append-only per card, tamper-evident chain | n/a |
| `out/source_matrix.json` | Authorization artifact, MANDATORY at lint/verify/search | No |
| `out/role_maps/<reading>.json` | Per-reading role-map | No |
| `cards/<reading>/SKILL.md` | Optional per-reading router | No |
| `out/semantic_cache.json` | Optional, frozen offline-built embedding-cache for Layer-3 B1 | No |
| `out/.kb_index_cache.json` | Performance heuristic; trust authority is recomputed hash | No |

### 1.3 Verification Pipeline

```
                   ┌──────────────────────────────────────────────────┐
                   │ Layer 0 (Pydantic + YAML strict parse)           │
                   │  → CACG-FM-001..008, CACG-CITE-002/003 from      │
                   │    citation field validators                     │
                   └────────────────────────┬─────────────────────────┘
                                            │
                   ┌────────────────────────▼─────────────────────────┐
                   │ Layer 1 (mechanical lint, regex/cross-reference) │
                   │  → CACG-CITE-*, CACG-HASH-*, CACG-AUTH-*,        │
                   │    CACG-RETR-*, CACG-DEP-*, CACG-ROLE-*,         │
                   │    CACG-SKILL-*, CACG-MAN-*                      │
                   │  Budget: microseconds per card                   │
                   └────────────────────────┬─────────────────────────┘
                                            │ (only if Layer 1 clean)
                   ┌────────────────────────▼─────────────────────────┐
                   │ Layer 2 (exact-substring containment + fuzzy)    │
                   │  → CACG-VERIFY-001                               │
                   │  Page-window byte-slice of pinned chunk.         │
                   │  Fuzzy = bounded Levenshtein (opt-in).           │
                   │  BM25 hints (rank-bm25) emitted ONLY on failure  │
                   │  with hint_only=true marker.                     │
                   │  Budget: milliseconds                            │
                   └────────────────────────┬─────────────────────────┘
                                            │ (only if Layer 2 fails)
                   ┌────────────────────────▼─────────────────────────┐
                   │ Layer 3 (semantic; opt-in, mutually exclusive)   │
                   │  B1: --semantic <cache.json> dict lookup         │
                   │  B2: --semantic-judge LLM-judge (Claude Haiku)   │
                   │  → CACG-VERIFY-002 (verdict rides in same        │
                   │    journal event as VERIFY-001)                  │
                   └──────────────────────────────────────────────────┘
```

### 1.4 Hashing Primitives (all SHA-256)

```python
source_sha256(path) = sha256(open(path).read())
chunk_hash(text, start_page, end_page, page_spans) =
    sha256(canonical_json({
        "text": normalize_text(text),
        "start_page": start_page,
        "end_page": end_page,
        "page_spans": [{"page": p, "byte_offset_in_chunk": b}, ...]
    }))
card_hash(frontmatter, body) =
    sha256(canonical_json(frontmatter without card_hash) +
           b"\n--BODY--\n" + normalize_text(body))
event_checksum(event) =
    sha256(canonical_json(event without event_checksum))
```

`canonical_json` = `json.dumps(obj, sort_keys=True, separators=(",",":"), ensure_ascii=False)`

`normalize_text` pipeline (deterministic, narrow, locked):
1. Unicode NFC
2. Ligature unification (ﬀ, ﬁ, ﬂ, ﬃ, ﬄ, ﬅ, ﬆ)
3. Hyphen-linebreak rejoin: `-\s*\n\s*` → `""`
4. Whitespace collapse: `\s+` → ` `
5. Strip leading/trailing whitespace

### 1.5 Atomic-Publish Discipline (POSIX)

Single-file: write to `<path>.tmp` → Pydantic round-trip validate the on-disk bytes → `os.replace(tmp, path)`.

Pair / N-file: snapshot each canonical to `.bak` first → write all `.tmp` files validated → `os.replace` each in sequence → on any failure restore from `.bak` and clean up. Symmetric rollback tracks per-canonical `replaced` and `had_prior` flags so partial-success doesn't leak.

Defense in depth: refuse to clobber pre-existing `.tmp` / `.bak` sidecars (`CACG-MAN-002`, `CACG-IDX-006/007/008`).

### 1.6 Append-Only Tamper-Evident Journals

Per append (in `cacg/journal.py:append_entry`):

1. `fcntl.flock(LOCK_EX)` advisory lock over the validate + tail-scan + write + cache-update window.
2. Process-local trust cache keyed on `(st_dev, st_ino, st_size, st_mtime_ns)`. Cache hit → skip the full O(N) `validate_jsonl` + `_scan_tail` pass. Cache miss → full re-validate before append.
3. Compute `event_checksum = sha256(canonical_json(payload sans event_checksum))`, set `prev_checksum = last_event.event_checksum`.
4. Pydantic-validate the assembled event.
5. Serialize to canonical JSON, append via single `os.write(fd, line+"\n")` on a fd opened with `O_APPEND|O_WRONLY|O_CREAT` — POSIX guarantees atomicity for sub-PIPE_BUF (≤ 4096 byte) payloads on the same file descriptor under `O_APPEND`.
6. Update the per-path trust cache with the post-write fingerprint.

Validator (`validate_jsonl`): re-reads every line, parses as `LintEvent`, asserts monotonic `seq`, recomputes `event_checksum` and verifies the prev-checksum chain. Any drift = bad line index.

### 1.7 Search Backends (with FTS5 sidecar)

- **In-memory BM25** (`cacg/search.py`): `rank-bm25` `BM25Okapi` over `[title, summary, *tags]` per card. Pre-filter: lexical token-set overlap. Sort on full-precision scores, tiebreak by `card_id` ASC. Output scores rounded to 6 decimals for byte-stable serialization.
- **SQLite FTS5 sidecar** (`cacg/search_sqlite.py`): `out/summaries.sqlite` built atomically as Phase-D2 step. Schema: `cards_fts(card_id UNINDEXED, reading_id UNINDEXED, path UNINDEXED, card_hash UNINDEXED, title, summary, tags, source_ids)` with `tokenize = 'unicode61 remove_diacritics 1'` + sealed `meta(key, value)` table with `summaries_hash`, `schema_version="cacg.v0.fts1"`, `builder_version="cacg.fts5.builder.v1"`, `summaries_count`. Reader opens via `file:...?mode=ro` URI and verifies the seal. Falls back to in-memory BM25 on stale or missing sidecar (`CACG-FTS-001`) or FTS5 unavailable in build (`CACG-FTS-002`).

### 1.8 Integration with humanize (Claude Code plugin)

Pure subprocess shell-out. humanize's RLCR loop emits a round summary; CACG's `kb verify --round-summary <path> --chunks-manifest <p> --source-matrix <p>` parses the `## Knowledge Consulted` section per the regex contract (`docs/integration-with-humanize.md`), verifies each cited card, and exits 0/1/2 per the documented matrix. STDOUT carries per-card verdicts (VERIFIED / STALE / MISSING). No Python in-process coupling, no PyO3 binding.

---

## 2. Legacy CFA_reading Patterns CACG Inherited

`../CFA_reading` (read-only sibling, 258 cards across 10 closed verticals, ~13k LoC of single-file lint/build scripts) is the *origin* of two patterns that CACG inherits in a cleaner form:

1. **Atomic publish discipline**: `scripts/kb/build_manifest.py:_publish_manifests_atomic` does the tmp + validate + rename dance with FTS5 sidecar + JSON dump in parallel. CACG generalizes this into a per-call combinator with proper rollback and pre-existing-sidecar refusal.

2. **Retraction log**: `.claude/retracted_cards.json` is the legacy single-list-of-paths retraction artifact. CACG replaces this with a proper schema-versioned `cards_manifest.retracted_cards`, `chunks_manifest.retracted_source_ids`, `chunks_manifest.retracted_chunk_ids`, all with disjointness invariants and tombstone history events.

What CACG does NOT inherit from CFA_reading (and what the Rust port should also NOT inherit):

- **Line-based pseudo-YAML frontmatter** (`Use when: ...` / `Primary raw source: <pdf> pp.5-10`). CACG-v0 uses real YAML + strict Pydantic.
- **String-based citations** (`<pdf> pp.5-10`). CACG-v0 uses structured `Citation { source_id, chunk_id, chunk_hash, page_range, quote, edge_type }`.
- **No chunk-level integrity**: CFA_reading hashes ONLY the source PDF bytes, never the extracted text. Drift between cited quote and actual PDF content is undetectable. CACG-v0 closes this by hashing extracted normalized chunk text with page metadata bound in.
- **Per-subcorpus rule branching**: thousands of `if subcorpus == "06"` lines. CACG-v0 keeps all rules generic (source_matrix is data, not code).

**Implication for Rust port:** CFA_reading is the *substrate* (large corpus that the Rust port will eventually verify against), not a code-pattern reference. Don't reach back into its Python scripts for design ideas.

---

## 3. SOTA Rust Crate Recommendations

Every version pin below is verified against `crates.io` JSON API on 2026-05-20. Date in parentheses is the latest release. Crates marked **★** are oracle-critical (their bytes flow into CACG hashes or trust outputs); for those, *implement directly* and use the crate only for low-level primitives.

### 3.1 PDF Text Extraction (★, isolated to `cacg-ingest`)

**Recommended:** `pdfium-render = "0.9.1"` (2026-05-02) + pinned Pdfium binary (chromium-builds release, exact build SHA logged).

**Rationale:** Same Pdfium backend as `pypdfium2`, so a fighting chance of byte-identical `FPDFText_GetText` output across the FFI boundary. `pdfium-render` is mature (1.05M downloads) and the only Rust binding that actually surfaces the per-page text API at the granularity `kb ingest` needs.

**Risks:**
- Byte-identical output to `pypdfium2 5.8.0` is **NOT** automatic. `pypdfium2.PdfTextPage.get_text_range()` uses specific defaults (whole-page UTF-16 LE buffer with explicit terminator handling); `pdfium-render`'s `PdfPage::text()` may differ in handling of: terminator nulls, internal whitespace, hyphenation marks, and Unicode replacement characters. **GATE:** golden byte tests against the existing Python-built `chunks_manifest.json` MUST pass before the Rust port emits any new chunk hashes for an existing corpus.
- Pdfium is C++ FFI: untrusted-PDF DoS / OOM / panic-across-FFI surface. Sandbox `cacg-ingest` accordingly (`-C panic=abort` in the ingest binary, resource-limited subprocess, or wrap each page extraction in a recoverable boundary).
- Cross-version determinism: every Pdfium binary bump must be paired with an explicit chunk-hash re-generation event (same discipline as the current pypdfium2 pin).

**Alternative:** `lopdf = "0.40.0"` (2026-03-19) + hand-rolled text-stream walker. Pure-Rust, no FFI, fully deterministic across Rust edition bumps. Downside: writing a correct PDF text extractor (CMap decoding, font Unicode mapping, ligature reconstruction, layout ordering) is a multi-month subproject. Not recommended unless you commit to owning the extraction stack outright. Documented for completeness only.

**Not recommended:** `pdf-extract = "0.10.0"` (text-only, but its output is heuristic and not deterministic enough); `pdf` crate (immature, parses PDFs but exposes raw streams rather than rendered text).

### 3.2 YAML Frontmatter Parsing (★, strict mode required)

**Recommended:** `yaml-rust2 = "0.11.0"` (2025-12-16) for event-level prescan + custom strict converter. Optional secondary path: `serde_yaml_bw = "2.5.6"` (2026-05-02) for the Serde-derive convenience after the prescan passes.

**Rationale:**
- `yaml-rust2` exposes the event stream so the prescan can reject `AliasEvent`, anchors (`anchor` field on `ScalarEvent` / `MappingStartEvent` / `SequenceStartEvent`), and non-default tags (anything not in `tag:yaml.org,2002:`) BEFORE node construction — matching the Python `_NoDuplicateKeysSafeLoader.compose_node` discipline exactly.
- Duplicate-mapping-key detection rolls into the prescan: maintain a `HashSet<key>` per `MappingStart..MappingEnd` window, surface duplicates as a strict parse error.
- After prescan, deserialize via `serde_yaml_bw` directly to the typed `CardFrontmatter` struct, with `#[serde(deny_unknown_fields)]` providing the `extra="forbid"` equivalent.

**Risks:**
- YAML 1.1 vs 1.2 scalar semantics: `yaml-rust2` is YAML 1.2, `PyYAML.SafeLoader` is YAML 1.1 with `yaml.resolver.BaseResolver.DEFAULT_MAPPING_TAG` defaults. The two differ on bare `yes/no/on/off` booleans and octal/hex number parsing. CACG schemas don't use these (all fields are strings or strict ints), but the test suite must include adversarial fixtures that lock this in.
- Avoid `serde_yml = "0.0.12"` (2024-08-25, last update 21 months ago) — Codex confirmed unsoundness rumors; stale unmaintained crate. Avoid `serde-yaml = "0.9.34"` (2024-03, unmaintained).
- `saphyr = "0.0.6"` (2025-06-11) is the spiritual successor to `yaml-rust2` but at 0.0.x pre-1.0 the API is still churning; revisit when it reaches 1.x.
- `libyaml-safer = "0.3.0"` (2025-12-22) is a hardened C-libyaml port but adds an unsafe-Rust transpile that we don't want in the trust path.

### 3.3 Strict Validation (Pydantic-equivalent)

**Recommended:** `serde = "1.0.228"` (latest) + `garde = "0.22.1"` (2025-11-30) for field-level rules + hand-written `TryFrom<RawFoo> for Foo` validators for cross-field invariants.

**Rationale:**
- `#[serde(deny_unknown_fields)]` is the direct equivalent of Pydantic's `extra="forbid"`.
- Required fields = non-`Option<T>` struct fields. Pydantic's `Field(min_length=...)` maps to `#[garde(length(min=80, max=400))]`. Regex constraints → `#[garde(pattern(...))]`. List uniqueness, sorted-order, disjointness → custom `#[garde(custom(validate_sorted_unique))]` functions.
- Cross-field model_validators (`ChunksManifest.retracted_source_ids` disjoint from `chunks[*].source_id`) belong in an explicit constructor: `pub fn try_new(raw: RawChunksManifest) -> Result<Self, ValidationError>`. Don't try to push this into a derive macro; the invariant is part of the type's identity.

**Risks:**
- `garde` 0.22.x is stable but the ecosystem still hasn't fully converged on it vs `validator = "0.18"`. Either is fine; pick `garde` for its `Context`-aware validation that maps closer to Pydantic's `info.data.get(...)` pattern.
- Diagnostic codes: don't let validation errors leak as raw `garde::Error` strings. Wrap with explicit `CACG-FM-001`..`CACG-FM-008` mapping (mirroring `cacg/frontmatter.py:_map_pydantic_errors`).

### 3.4 Canonical JSON Serialization (★, must match Python json.dumps byte-for-byte)

**Recommended:** Roll our own using `serde_json = "1.0.149"` (latest, with the `preserve_order` feature OFF) as a substrate. Approximately 80-120 LoC.

**Rationale:** Python's `json.dumps(obj, sort_keys=True, separators=(",",":"), ensure_ascii=False)` byte format is:
- Object keys sorted (Python's `sorted(keys)`).
- No whitespace between tokens.
- `ensure_ascii=False` means non-ASCII characters are emitted as literal UTF-8 bytes (no `\uXXXX` escaping for codepoints > U+007F).
- Control characters (` `..``) are escaped via `\u` sequences EXCEPT `\b \t \n \f \r` which use the short forms.
- Double-quote and backslash are escaped (`\"` `\\`).
- Forward slash is NOT escaped.

`serde_json` defaults differ on:
- Object key order is insertion order (with default `Map`); needs explicit sort.
- `serde_json::to_string` may serialize floats slightly differently from Python's `repr(float)` for edge-case values.

**Implementation sketch:**
```rust
// cacg-core/src/canonical_json.rs
pub fn canonical_json(value: &serde_json::Value) -> String {
    let mut out = String::with_capacity(estimate_size(value));
    write_value(&mut out, value);
    out
}

fn write_value(out: &mut String, v: &Value) {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => out.push_str(&n.to_string()), // integers only in our schema
        Value::String(s) => write_string(out, s),         // hand-rolled escape matching Python
        Value::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 { out.push(','); }
                write_value(out, item);
            }
            out.push(']');
        }
        Value::Object(map) => {
            // Sort by key. serde_json::Map's iteration order depends on feature flags;
            // collect into a Vec<(&String, &Value)> and sort_by_key.
            let mut entries: Vec<_> = map.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            out.push('{');
            for (i, (k, v)) in entries.iter().enumerate() {
                if i > 0 { out.push(','); }
                write_string(out, k);
                out.push(':');
                write_value(out, v);
            }
            out.push('}');
        }
    }
}

fn write_string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{08}' => out.push_str("\\b"),
            '\u{09}' => out.push_str("\\t"),
            '\u{0A}' => out.push_str("\\n"),
            '\u{0C}' => out.push_str("\\f"),
            '\u{0D}' => out.push_str("\\r"),
            c if (c as u32) < 0x20 => {
                use std::fmt::Write;
                write!(out, "\\u{:04x}", c as u32).unwrap();
            }
            c => out.push(c), // ensure_ascii=False — non-ASCII passes through
        }
    }
    out.push('"');
}
```

**Risks:**
- **DO NOT** use `serde-jcs = "0.2.0"` (2026-03-25). JCS / RFC 8785 has a different escape policy (escapes more characters) AND different number canonicalization (always uses the shortest float representation per ECMAScript Number.toString). It will NOT match Python `json.dumps` bytes.
- Floats are rare in the CACG schema (only `latency_ms` in journal events; `score: float` in `SemanticVerdict`). Both Python `json.dumps` and serde_json render `0.5` as `0.5`, `0.05` as `0.05`. But `repr(0.1)` = `'0.1'` in Python 3 while `serde_json::to_string(&0.1f64)` may differ at higher precision. **GATE:** snapshot tests covering each numeric field.
- Test the escape policy against the existing Python golden corpus: every quote character, every embedded newline, every Unicode codepoint above U+007F must round-trip identically. The existing `tests/test_round*_review.py` fixtures already exercise many of these; replicate the assertions in Rust.

### 3.5 SHA-256 Hashing (★, compatibility with Python)

**Recommended:** `sha2 = "0.10.x"` (latest 0.11.0, 2026-03-25 — start with 0.10.9 for stability). Enable `asm` feature opportunistically; benchmark before pinning.

**Rationale:**
- Existing CACG corpora have SHA-256 chunk_hashes in production manifests. Rust port MUST produce byte-identical hashes for the same canonical-JSON envelope inputs. `blake3` is faster but a different algorithm; not an option.
- `sha2` 0.10.x and 0.11.x both use the RustCrypto Digest trait. 0.11.x adds runtime CPU dispatch (x86 SHA-NI + aarch64 SHA2). On Apple Silicon and modern Intel/AMD chips, SHA-NI gives ~3-5× over scalar code. On older or non-x86 chips, scalar SHA-256 is still ~2× faster than CPython's hashlib (which itself calls OpenSSL).
- For 5MB-50MB PDF byte streams in `source_sha256(path)`, the speedup matters; for ~1KB canonical JSON envelopes in `chunk_hash` / `card_hash` / `event_checksum`, hashing is already not the bottleneck and any version is fine.

**Risks:** `sha2` 0.11.0 is recent (March 2026); start with 0.10.9 and bump to 0.11 only after benchmarks confirm stability under Pdfium's PDF-byte hashing.

### 3.6 SQLite FTS5 Sidecar

**Recommended:** `rusqlite = "0.39.0"` (2026-03-15) with `features = ["bundled", "modern_sqlite"]` for guaranteed FTS5 availability + Unicode tokenizer.

**Rationale:**
- Existing Python builds use the platform `sqlite3` (whatever ships with the OS). Bundled SQLite in `rusqlite` gives a deterministic SQLite version and FTS5 compile flags across machines.
- The sidecar's seal is `sha256(summaries.json bytes)` plus the `schema_version` / `builder_version` literal strings. Cross-implementation byte-identity of the sidecar file is explicitly NOT a contract (per Phase-4 DEC-8) — what matters is that the SAME query under the SAME tokenizer returns the SAME row set with deterministic `ORDER BY rank, card_id ASC`.
- `unicode61 remove_diacritics 1` tokenizer is universal across modern SQLite FTS5 builds.

**Risks:**
- The Rust port might emit `out/summaries.sqlite` with a slightly different B-tree layout than Python's build. Per DEC-8 this is expected. The seal (`meta.summaries_hash`) is the trust boundary, not the raw bytes.
- Verify: `ORDER BY bm25(cards_fts), card_id ASC` returns identical row ordering across Python and Rust on the same fixture. Snapshot test the row-set hash, not the file bytes.

**Alternative:** `sqlx` is async-first; overkill for this read-mostly use case. Stick with `rusqlite`.

### 3.7 In-Memory BM25 (★, parity with rank-bm25)

**Recommended:** Hand-roll BM25Okapi in `cacg-core/src/bm25.rs` (~50-80 LoC). Use the third-party `bm25 = "2.3.x"` crate for benchmark cross-validation only, not production.

**Rationale:**
- `rank-bm25`'s `BM25Okapi` uses the standard Okapi BM25 formula with `k1=1.5, b=0.75, epsilon=0.25` defaults. Replicating these exactly + the lexical-overlap pre-filter + the `card_id` ASC tiebreak is ~50 LoC.
- Third-party Rust BM25 crates bake in tokenization (stemming, lowercasing) that may differ from `cacg.normalize.normalize_for_lookup` (NFC + lowercase + whitespace split). Owning the tokenizer is non-negotiable for byte-identical search output.

**Implementation sketch:**
```rust
// cacg-core/src/bm25.rs
pub struct Bm25Okapi { /* k1, b, epsilon, avgdl, idfs, corpus */ }

impl Bm25Okapi {
    pub fn new(corpus: Vec<Vec<String>>, k1: f64, b: f64, epsilon: f64) -> Self { ... }
    pub fn get_scores(&self, query: &[String]) -> Vec<f64> { ... }
}

pub fn search(query: &str, summaries: &[SummaryEntry], top_k: usize) -> Vec<SearchHit> {
    let bm25 = Bm25Okapi::new(tokenize_summaries(summaries), 1.5, 0.75, 0.25);
    let q_tokens = tokenize(query);
    let scores = bm25.get_scores(&q_tokens);
    let mut hits: Vec<_> = scores.iter().enumerate()
        .filter(|(i, _)| lexical_overlap(&q_tokens, &summaries[*i].corpus_tokens))
        .map(|(i, s)| (i, *s))
        .collect();
    hits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal)
                            .then_with(|| summaries[a.0].id.cmp(&summaries[b.0].id)));
    hits.into_iter().take(top_k).map(|(i, s)| ...).collect()
}
```

**Risks:** Floating-point determinism. Use `f64` throughout (matching Python's float). The `partial_cmp` + `card_id` tiebreak makes the order deterministic even if two scores are within float epsilon.

### 3.8 Fuzzy Matching (Levenshtein, bounded early-exit)

**Recommended:** Roll our own bounded-Levenshtein DP in `cacg-core/src/fuzzy.rs` (~30-50 LoC) with early-exit when any row exceeds the threshold. Or use `strsim = "0.11.x"` as a baseline.

**Rationale:**
- `rapidfuzz = "0.5.0"` (2023-12-01) is **stale** — not maintained for ~2.5 years as of 2026-05. Codex's recommendation was based on the published version but Codex did not check the freshness; my crates.io fetch confirms it's a dead crate.
- `triple_accel = "0.4.0"` (2021-06-18) is even staler (5 years).
- `strsim = "0.11.x"` is the venerable "boring" choice — small, no SIMD, but lives at the API the CACG fuzzy oracle needs.
- Bounded-Levenshtein with row-min early-exit is ~30 LoC and beats SIMD libraries for the short-quote / short-chunk case typical of CACG (quotes are 80-400 chars, chunks are ~1500-2000 chars per page-window).

**Implementation:** Standard DP with `if min(row) > threshold { return None }` exit condition. The current Python `fuzzy.py` already uses the same approach.

**Risks:** None. This is a settled algorithm; benchmarking only matters if quotes get very long.

### 3.9 File Locking

**Recommended:** `rustix = "1.1.4"` (2026-02-22) for `flock`-compatible POSIX advisory locks: `rustix::fs::flock(fd, FlockOperation::LockExclusive)`.

**Rationale:**
- Python's `fcntl.flock(fd, LOCK_EX)` is POSIX BSD-style advisory locking. `rustix::fs::flock` is a direct equivalent (same syscall semantics: per-fd lock, lock released on fd close, NOT compatible with `fcntl()` record-locks).
- `fd-lock = "4.0.4"` (2025-03-10) is RAII-friendly and uses `flock` internally; either is fine. Prefer `rustix` if you're already using it for `write()` syscall (see §3.11).

**Risks:**
- POSIX-only is acceptable (CACG Phase-4 already documents Windows as non-goal via `CACG-LCK-001`).
- Advisory locks on NFS are unreliable. CACG already documents this as a trust-boundary weakness.

### 3.10 Atomic Single-File Writes

**Recommended:** `tempfile = "3.27.0"` (2026-03-11) via `NamedTempFile::new_in(out_dir).persist(canonical_path)`.

**Rationale:**
- `tempfile::NamedTempFile` is the battle-tested idiom. `persist()` calls `rename(2)` which is atomic on POSIX when source and target are on the same filesystem (guaranteed because `new_in(out_dir)` keeps both in `out_dir`).
- Don't use `atomicwrites` — older, less actively maintained, fewer features.
- After `persist()`, call `File::open(parent_dir).sync_all()` to fsync the directory so the rename hits stable storage. Python's `os.replace` does not do this implicitly either, so byte-equivalent CRASH behavior is preserved; Rust just gives us a better hook to add fsync if we want stronger durability.

### 3.11 Atomic Single-Line Append (★, journal hot path)

**Recommended:** `rustix::io::write` directly on a `rustix::fs::open` fd with `OFlags::APPEND | OFlags::WRONLY | OFlags::CREATE`. Single syscall, single line.

**Rationale:**
- Rust's `std::io::Write::write_all` is implemented as a loop that calls `write(2)` repeatedly until the buffer is consumed. For a < 4096-byte line on an O_APPEND fd this almost always completes in one syscall, but the standard library does NOT guarantee it.
- Python's `os.write(fd, line + b'\n')` is a direct `write(2)` call, no loop. POSIX guarantees:
  - For a fd with `O_APPEND`, the kernel performs the seek-to-end + write atomically as a single operation under the inode lock.
  - For payloads ≤ `PIPE_BUF` (4096 bytes on Linux), the kernel does not interleave with other appenders.
- To get the same guarantee in Rust: use `rustix::io::write(fd, buf)` which is a thin wrapper over `libc::write`. Treat a short write as a fatal error (mirrors Python's `if written != len(payload): raise OSError(...)`).

**Implementation sketch:**
```rust
use rustix::fs::{open, OFlags, Mode};
use rustix::io::write;

fn append_one_line_atomically(path: &Path, line: &str) -> Result<(), Error> {
    let mut payload = line.as_bytes().to_vec();
    payload.push(b'\n');
    if payload.len() > 4096 {
        return Err(Error::LineTooLong); // mirror Python's discipline
    }
    let fd = open(path, OFlags::WRONLY | OFlags::APPEND | OFlags::CREATE,
                  Mode::RUSR | Mode::WUSR | Mode::RGRP | Mode::ROTH)?;
    let written = write(&fd, &payload)?;
    if written != payload.len() {
        return Err(Error::PartialWrite { written, expected: payload.len() });
    }
    Ok(())
}
```

**Risks:** Lines longer than PIPE_BUF (4096) lose the atomicity guarantee. CACG events are typically 600-1200 bytes; preflight a length check (matching Python's existing OSError-on-partial-write).

### 3.12 Multi-File Pair-Atomic Publish

**Recommended:** Roll our own. ~200 LoC in `cacg-core/src/atomic_publish.rs`. No community crate matches the per-file `.bak` snapshot + symmetric-rollback discipline CACG documents.

**Rationale:** The Python `cacg/manifest.py:publish_sources_and_chunks` and `cacg/index.py:_publish_*` functions have been hardened over Rounds 6-9 with specific guarantees:
- Refuse to clobber pre-existing `.tmp` or `.bak` sidecars (`CACG-MAN-002`, `CACG-IDX-006/007/008`).
- Refuse to publish into a non-directory `out_dir`.
- Refuse to overwrite a canonical that exists as a non-regular-file (`CACG-MAN-003`, `CACG-IDX-008`).
- Track per-canonical `replaced` AND `had_prior` flags; rollback removes newly-created canonicals AND restores prior canonicals from `.bak`.

This is a well-understood ~150-200 LoC module; porting to Rust is straightforward with `tempfile` + `rustix::fs::rename` + `rustix::fs::unlink`. Test extensively with failure-injection (mock the rename to fail at every step).

### 3.13 JSONL Streaming Reader

**Recommended:** Stdlib `BufReader::new(File::open(p)).lines()` + `serde_json::from_str` per line. Don't pull in a JSONL wrapper crate.

**Rationale:**
- `serde-jsonlines = "0.7.0"` is fine for ordinary JSONL but the CACG journal validator needs:
  - 1-indexed line numbers in error reports.
  - Per-line `serde_json::from_str` so a single malformed line surfaces as a bad-line entry, not aborts the read.
  - Access to the canonical-JSON re-serialized payload for checksum recomputation.
- All of this is stdlib + serde_json. Pulling in a wrapper crate adds an indirection that obscures the trust-critical inner loop.

### 3.14 Determinism Scaffolding (KB_FROZEN_CLOCK equivalent)

**Recommended:** Explicit `DeterminismContext` struct, passed by reference through every code path that emits timestamps or UUIDs. Construct once at CLI dispatch from `std::env::var("KB_FROZEN_CLOCK").is_ok()`. Use `time = "0.3.x"` and `uuid = "1.x"`.

```rust
// cacg-core/src/determinism.rs
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone)]
pub struct DeterminismContext {
    clock: ClockMode,
    id_gen: IdMode,
}

enum ClockMode { Real, Frozen(OffsetDateTime) }
enum IdMode { Real, Frozen(Uuid) }

impl DeterminismContext {
    pub fn from_env() -> Self {
        if std::env::var_os("KB_FROZEN_CLOCK").is_some() {
            Self { clock: ClockMode::Frozen(OffsetDateTime::UNIX_EPOCH),
                   id_gen: IdMode::Frozen(Uuid::nil()) }
        } else {
            Self { clock: ClockMode::Real, id_gen: IdMode::Real }
        }
    }
    pub fn now(&self) -> OffsetDateTime { match self.clock { ... } }
    pub fn new_uuid(&self) -> Uuid { match self.id_gen { ... } }
}
```

**Rationale:**
- Explicit context-passing is testable. An ambient thread-local + env-var (Python's pattern) makes parallel test runs awkward.
- `time` crate is faster and lighter than `chrono` for our use case (we only need ISO-8601 UTC formatting). Use `chrono` only if you need timezone parsing — not relevant here.
- `Uuid::nil()` = `00000000-0000-0000-0000-000000000000`, matching Python's `uuid.UUID(int=0)`.

**Risks:** None. This is a routine refactor of an env-var-driven global into a properly-injected context.

### 3.15 PyO3 Interop

**Recommendation:** **NONE.** Pure CLI shell-out. No PyO3.

**Rationale:**
- The humanize integration is already shell-out (`subprocess.run(["kb", "verify", "--round-summary", path, ...])`). Exit codes drive the workflow.
- A `pycacg` Python wheel would add: build-system complexity (cibuildwheel / maturin), Python ABI versioning (manylinux), wheel-distribution overhead (50+ MB binaries per platform), AND a subtle determinism risk (PyO3 might let callers reach into the in-process state and bypass the CLI's argument-validation discipline).
- The shell-out boundary IS the trust boundary. Don't blur it.

### 3.16 Async vs Sync

**Recommendation:** Sync everywhere except Layer-3 LLM-judge. `rayon = "1.12.0"` (2026-04-14) for embarrassingly-parallel per-card work.

**Rationale:**
- 99% of CACG operations are CPU-bound (SHA-256, YAML parsing, BM25 scoring) or filesystem-IO bound (manifest reads, journal appends). Tokio adds runtime overhead with no upside.
- `rayon::par_iter()` over the card list in `lint_directory` / `verify_round_summary` is a one-line speedup that takes 10k-card runs from sequential to N-core-parallel. Embarrassingly parallel: each card's verify is independent (the chunks index is read-only across the batch).
- Layer-3 LLM-judge B2 needs HTTP. Isolate this in `cacg-semantic` (optional crate, feature-gated). The async runtime never touches the common path.

**Risks:**
- Determinism with rayon: rayon's parallel iteration order is non-deterministic. Collect results into a Vec then sort by `card_path` ASC before journal append, so the journal's per-card event ORDER is stable. Within each per-card event the data is already deterministic.
- The journal append itself MUST be serialized (one event at a time, behind the `flock` lock). Use rayon for per-card *compute*; serialize the per-card *appends*.

### 3.17 Workspace Structure

**Recommendation:** Cargo workspace.

```
cacg/
├── Cargo.toml            # workspace = { members = ["cacg-core", "cacg-cli", "cacg-ingest", "cacg-search", "cacg-semantic"] }
├── crates/
│   ├── cacg-core/        # schema, hashing, normalize, canonical_json, bm25, fuzzy, journal,
│   │                     # atomic_publish, determinism, diagnostic, all CACG-* codes
│   ├── cacg-ingest/      # ONLY crate that depends on pdfium-render. PDF-isolation contract.
│   ├── cacg-search/      # rusqlite FTS5 sidecar + in-memory BM25 wrapper
│   ├── cacg-semantic/    # OPTIONAL, feature-gated. Layer-3 (B1 cache lookup; B2 LLM judge via reqwest+tokio)
│   └── cacg-cli/         # clap, the `kb` binary, subcommand dispatch
└── xtask/                # custom workspace tasks (snapshot regeneration, perf budget checks)
```

**Rationale:**
- The PDF-isolation contract (`CACG-PERF-001` perf sentinel; common-path verbs must NOT import pypdfium2) maps cleanly to crate boundaries: `cacg-cli` depends on `cacg-core` always and `cacg-ingest` only when dispatching `kb ingest`. `cacg-core` has NO Pdfium dependency in its `Cargo.toml`.
- Defense in depth via CI:
  - `cargo deny check bans` rejects any crate that pulls Pdfium into the wrong target.
  - `cargo tree -e features -p cacg-core` snapshot test: if Pdfium ever appears in `cacg-core`'s dependency graph, the PR fails.
  - `#![forbid(unsafe_code)]` in every crate EXCEPT `cacg-ingest` (which has the Pdfium FFI).
- Feature flags: `cacg-cli` exposes `--features="ingest,semantic"` so a stripped-down release binary (e.g., humanize-bundle) doesn't ship Pdfium.

**Risks:**
- Cargo workspace dependency-version unification: ensure all crates pin the same `serde`, `serde_json`, `time`, `uuid` versions. Use `[workspace.dependencies]` for shared crate versions in the workspace root `Cargo.toml`.
- IDE / rust-analyzer experience: workspaces are well-supported, no friction.

### 3.18 Error Model

**Recommendation:** Roll our own `Diagnostic { code, severity, message, file, hints }` struct (matching the Python `cacg.diagnostic.Diagnostic`). Use `miette = "7.6.0"` (2025-04-27) for CLI-side human rendering only. `thiserror = "2.0.18"` + `anyhow = "1.0.102"` for actual `Result<T, E>` errors.

**Rationale:**
- CACG-* codes are part of the persisted data contract (lint_journal.jsonl events). They are NOT just human-facing diagnostics. `miette::Diagnostic` is built for rich terminal rendering; it does not serialize to canonical-JSON in the shape CACG needs.
- Keep the canonical `Diagnostic` struct in `cacg-core/src/diagnostic.rs`:
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
  pub struct Diagnostic {
      pub code: String,       // "CACG-CITE-002"
      pub severity: Severity, // Error | Warning
      pub message: String,
      pub file: Option<String>,
      pub hints: Vec<DiagnosticHint>,
  }
  ```
- Use `miette` purely as a downstream rendering layer in `cacg-cli`: convert `Diagnostic` → `miette::Report` for `--pretty` output. The default output (machine-readable, matching current Python CLI) bypasses miette entirely.

**Risks:** Don't let `thiserror`-derived enum variants leak into the journal serialization path. Diagnostic codes are NOT error types; they are typed data.

### 3.19 Test Framework

**Recommendation:** `cargo test` + `insta = "1.47.2"` (2026-03-30) + `proptest = "1.11.0"` (2026-03-24).

**Concrete plan:**
- **Snapshot tests** (`insta`): every persistent artifact byte-equivalence. Take the existing Python golden corpus (`tests/golden/*.md`, `tests/fixtures/sample.pdf`), run `kb ingest` + `kb index` + `kb verify` under both Python and Rust, snapshot:
  - `chunks_manifest.json` bytes (gated on Pdfium-output parity; see §3.1 risk)
  - `cards_manifest.json` bytes
  - `summaries.json` bytes
  - `INDEX.md` bytes
  - `lint_journal.jsonl` bytes (under `KB_FROZEN_CLOCK=1`)
  - per-card `history.jsonl` bytes (under `KB_FROZEN_CLOCK=1`)
  - `kb search` stdout (both human and `--json`)
  - `kb verify --round-summary` stdout
- **Property tests** (`proptest`): normalization round-trip (`normalize(normalize(x)) == normalize(x)`), canonical-JSON determinism (`canonical_json(canonical_json::parse(s)) == s` for valid canonical bytes), hash invariance under canonical-JSON serialization.
- **Adversarial fixtures**: port all 12 adversarial cards from `tests/adversarial/01..12-*.md` verbatim. Add Rust-only fixtures: YAML anchor injection, YAML duplicate keys, oversized canonical JSON, partial-write torn journals, rollback-debris from previous crashed runs.
- **Cross-implementation parity tests**: a `xtask parity-check` that runs both `python -m cacg.cli ...` and `cargo run --bin kb -- ...` on the same fixture and asserts byte-equal output. Run this in CI on every PR until the Rust port reaches feature parity.

### 3.20 Performance Instrumentation

**Recommendation:** `criterion = "0.8.2"` (2026-02-04) for microbenchmarks + `iai-callgrind = "0.16.1"` (2025-07-30) for CI-stable instruction budgets + `tracing = "0.1.x"` for structured logs + `pprof = "0.15.0"` for flame graphs on demand.

**Concrete plan:**
- **Criterion** measures wall-clock with statistical analysis. Use for local benchmarks during optimization.
- **iai-callgrind** measures instruction counts deterministically (under Valgrind/Cachegrind). Use for the CI perf gate — `criterion` is too noisy for CI thresholds. The current Python `tests/perf/test_phase*` budgets translate to instruction-count budgets in iai-callgrind.
- **tracing**: emit structured per-card events (`card_path, layer1_ms, layer2_ms, used_fuzzy, diagnostic_count`). At `RUST_LOG=info` they go to stderr; at `RUST_LOG=trace` they go to JSONL for downstream analysis.
- **pprof**: opt-in via env var (`CACG_PPROF=1`) so a single-run flame graph can be captured without leaking overhead into the default path.

CI perf budgets (mirror current Python budgets):
- `bench_verify_one_card` (single card, no fuzzy, no semantic): instruction count budget set at first run + 5% headroom.
- `bench_verify_round_summary_1k` (1000 cards): instruction count budget set at first run + 10%.
- `bench_search_1k` (1000-card BM25 search): instruction count budget set at first run + 5%.
- `bench_index_warm_10k` (warm-cache reindex of 10k stress fixture): instruction count budget.

---

## 4. Architectural Trust Boundaries & Risks

### 4.1 Critical Invariants the Rust Port MUST Preserve

1. **Chunk hash byte-equivalence with Python pypdfium2 5.8.0 output.** If a CACG corpus built by Python is re-verified by the Rust binary, every chunk_hash in the manifest must verify against the recomputed envelope. *Gate:* `tests/parity/test_chunks_manifest_byte_equal.rs` runs both implementations on the canonical sample PDF and asserts `chunks_manifest.json` bytes match.

2. **Canonical JSON byte-equivalence with Python `json.dumps(sort_keys=True, separators=(",",":"), ensure_ascii=False)`.** Same gate. Snapshot every persisted artifact's bytes.

3. **Tamper-evident journal chain integrity.** A journal written by Python and validated by Rust (or vice versa) must produce the same `validate_jsonl` result. *Gate:* `tests/parity/test_journal_cross_lang.rs` cross-validates 1000-event journals built by each.

4. **PDF-isolation contract.** No common-path verb (`lint`, `verify`, `search`, `show`, `new`, `index`, `history`) imports Pdfium. *Gate:* CI checks `cargo tree -e features -p cacg-cli --no-default-features --features=common-path` reports zero Pdfium edges.

5. **Mandatory `--source-matrix` everywhere.** `kb lint`, `kb verify`, `kb search`, `kb show` exit 2 without the flag. *Gate:* `tests/cli/test_mandatory_source_matrix.rs`.

6. **Determinism under KB_FROZEN_CLOCK=1.** Two consecutive runs of `kb index` on the same corpus produce byte-identical `summaries.json`, `cards_manifest.json`, `INDEX.md`, and journal events. *Gate:* `tests/parity/test_kb_index_byte_identical.rs`.

### 4.2 Risks Specific to the Rust Port

| Risk | Severity | Mitigation |
|------|----------|------------|
| Pdfium-render output differs from pypdfium2 at the byte level | HIGH | Golden tests against existing Python corpus before any Rust ingest emits to disk. If divergence detected, document the divergence + version-bump chunk_hash policy. |
| Canonical JSON Unicode escape policy drift | HIGH | Custom serializer matching Python exactly; snapshot tests on every adversarial Unicode codepoint. |
| YAML 1.1 vs 1.2 semantic drift | MEDIUM | yaml-rust2 is YAML 1.2; CACG schemas don't use the divergent constructs (no bare `yes`/`no`, no octal literals), but verify with adversarial fixtures. |
| BM25 floating-point determinism differs across CPU architectures | MEDIUM | Snapshot tests pin scores at 6-decimal precision (matches Python's `round(s, 6)`); `card_id` ASC tiebreak makes ordering deterministic even with float ε drift. |
| SQLite FTS5 tokenizer drift across SQLite versions | MEDIUM | Bundle SQLite in `rusqlite`; pin to a specific SQLite version per CI matrix. |
| rapidfuzz crate is stale (Dec 2023) | LOW | Roll our own Levenshtein DP (~30 LoC). |
| Pdfium native crash on malformed PDF takes down `kb ingest` | LOW | Per-page extraction in a `std::panic::catch_unwind` boundary; report as `CACG-INGEST-001` rather than panicking. |

### 4.3 What NOT to Do

- Don't use PyO3 to expose a Python binding (§3.15).
- Don't use serde-jcs for canonical JSON (§3.4 — different RFC).
- Don't use blake3 for chunk_hash / card_hash (§3.5 — incompatible with existing corpus).
- Don't use a wrapper crate around `os.write(O_APPEND)` (§3.11 — Rust stdlib's `write_all` is a loop, not a single syscall).
- Don't pull Pdfium into `cacg-core` (§3.17 — PDF-isolation contract is enforced by crate boundaries).
- Don't use `chrono` if `time` works (lighter, faster, same correctness for our use case).
- Don't async-ify the common path (§3.16 — sync + rayon is faster).

---

## 5. Performance Budget Analysis

### 5.1 Where Python Spends Its Time Today

Phase-4 Codex Rust-analysis (referenced inline in `cacg/frontmatter.py:_RejectedYAMLConstruct` docstring) measured PyYAML parsing at **91% of `kb verify` wall-clock**. The R5 single-pass loader collapsed the double-parse into one, dropping 1000-card `kb verify --round-summary` from ~2900ms to 555ms. The remaining time:

| Subsystem | Python share | Rust expected share |
|-----------|--------------|---------------------|
| YAML parse + Pydantic validate | ~60% | ~10% (Serde+garde are 10-20× faster) |
| Canonical JSON serialize | ~10% | ~3% |
| SHA-256 hash | ~5% | ~2% |
| Layer-2 substring + page-window slice | ~5% | ~5% (already fast; bottlenecked by memory bandwidth) |
| Layer-1 regex/cross-reference | ~5% | ~2% |
| File IO (read + atomic write) | ~10% | ~10% |
| Journal append (lock + tail-scan amortized) | ~5% | ~5% |

### 5.2 Rust-Only Wins

- **Per-card lint+verify**: dominated by YAML parse in Python. With Serde + yaml-rust2, expect 10-20× speedup. Per-card wall-clock falls from ~0.5ms to ~30-50μs.
- **1000-card `kb verify --round-summary`**: 555ms → 30-60ms.
- **`kb search` 1000-card p95**: dominated by BM25 corpus build (~50ms per call in Python). With pre-tokenized corpus held in `Arc<Bm25Okapi>` across calls, can drop to ~5ms; first call rebuilds.
- **10k FTS5 cold-open**: SQLite is the same; the Rust win here comes from avoiding Python's stdlib `sqlite3` Python-level overhead. Realistic gain: 2-3×.
- **Warm-cache `kb index` 10k**: dominated by Phase-D2 republish even when content unchanged. With the content-hashed Phase-D2 skip + Rust's fast YAML parse, expect 725ms → 100ms (matches the original AC-P9 strict target).
- **`kb ingest`** (cold, PDF-bound): roughly unchanged. The chunker + normalizer are 5-10× faster in Rust but Pdfium dominates the wall-clock.

### 5.3 Where Rust Won't Help Much

- Layer-2 substring containment (already fast; bounded by `chunk.text` length).
- File IO latency (the kernel doesn't care which language sent the syscall).
- SQLite query latency (FTS5 internals are C, same speed).
- LLM-judge B2 (network-bound, dominated by API latency).

---

## 6. Concrete Cargo Workspace Skeleton

```toml
# Cargo.toml (workspace root)
[workspace]
resolver = "2"
members = [
    "crates/cacg-core",
    "crates/cacg-ingest",
    "crates/cacg-search",
    "crates/cacg-semantic",
    "crates/cacg-cli",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.80"
license = "MIT"

[workspace.dependencies]
# Core
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.149"
garde = { version = "0.22.1", features = ["derive"] }
thiserror = "2.0.18"
anyhow = "1.0.102"
time = { version = "0.3.47", features = ["formatting", "parsing"] }
uuid = { version = "1.23.1", features = ["v4"] }

# Hashing
sha2 = "0.10.9"

# YAML
yaml-rust2 = "0.11.0"
serde_yaml_bw = "2.5.6"

# IO / Atomicity
tempfile = "3.27.0"
rustix = { version = "1.1.4", features = ["fs", "io"] }

# Concurrency
rayon = "1.12.0"

# CLI
clap = { version = "4.6.1", features = ["derive", "env", "wrap_help"] }
miette = { version = "7.6.0", features = ["fancy"] }

# SQLite + FTS5
rusqlite = { version = "0.39.0", features = ["bundled", "modern_sqlite"] }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }

# PDF (only in cacg-ingest)
pdfium-render = "0.9.1"

# Layer-3 B2 (only in cacg-semantic feature)
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls", "json"] }
tokio = { version = "1", features = ["macros", "rt"] }

# Dev / test
insta = "1.47.2"
proptest = "1.11.0"
criterion = "0.8.2"
iai-callgrind = "0.16.1"
```

```rust
// crates/cacg-core/src/lib.rs
//! CACG core: schema, hashing, normalize, canonical JSON, BM25, fuzzy,
//! journal, atomic publish, determinism, diagnostics.
//!
//! This crate has NO dependency on Pdfium, SQLite (FTS5 sidecar lives in cacg-search),
//! HTTP clients, or async runtimes. It is the trust kernel.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic, missing_docs)]

pub mod schema;          // CardFrontmatter, ChunkRecord, ChunksManifest, ... (Serde + garde)
pub mod normalize;       // normalize_text + normalize_for_lookup
pub mod hash;            // source_sha256, chunk_hash, card_hash, canonical_json
pub mod canonical_json;  // The byte-equivalent Python json.dumps writer.
pub mod bm25;            // Bm25Okapi parity with rank-bm25
pub mod fuzzy;           // Bounded Levenshtein DP
pub mod journal;         // append-only JSONL with prev_checksum/event_checksum chain
pub mod history;         // per-card history JSONL
pub mod atomic_publish;  // tmp + .bak + replace + rollback combinator
pub mod determinism;     // DeterminismContext (KB_FROZEN_CLOCK)
pub mod diagnostic;      // Diagnostic struct + CACG-* codes constants
pub mod source_matrix;   // SourceMatrix authorization + scaffold helpers
pub mod frontmatter;     // YAML parse + canonical writer
pub mod chunks_index;    // ChunksIndex (in-memory by-id and by-source lookup)
```

```rust
// crates/cacg-ingest/src/lib.rs
//! `kb ingest` path. The ONLY crate that depends on pdfium-render.

#[cfg(feature = "ingest")]
pub mod pdf;             // pdfium-render wrapper, per-page text extract
#[cfg(feature = "ingest")]
pub mod chunker;         // Paragraph-respecting token-budget chunker
#[cfg(feature = "ingest")]
pub mod manifest;        // publish_sources_and_chunks (pair-atomic)
```

```rust
// crates/cacg-cli/src/main.rs
//! `kb` binary: clap dispatch into the per-subcommand handlers.
//!
//! Common-path subcommands (lint, verify, search, show, new, index, history, retract*)
//! do NOT pull in cacg-ingest. The cacg-ingest dependency is feature-gated.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kb", version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    #[cfg(feature = "ingest")]
    Ingest(IngestArgs),
    New(NewArgs),
    Lint(LintArgs),
    Verify(VerifyArgs),
    Index(IndexArgs),
    History(HistoryArgs),
    Retract(RetractArgs),
    RetractSource(RetractSourceArgs),
    RetractChunk(RetractChunkArgs),
    ScaffoldMatrix(ScaffoldMatrixArgs),
    ScaffoldRoleMap(ScaffoldRoleMapArgs),
    Search(SearchArgs),
    Show(ShowArgs),
    MigrateSummaries(MigrateSummariesArgs),
}

fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    let det = cacg_core::determinism::DeterminismContext::from_env();
    match cli.cmd {
        Cmd::Lint(args) => cmd_lint(args, &det),
        Cmd::Verify(args) => cmd_verify(args, &det),
        // ...
    }
}
```

---

## 7. Migration Strategy

### 7.1 Recommended: Clean-Slate Rewrite Behind Identical CLI

Don't try to incrementally swap Python modules for Rust modules. The trust boundaries are at the persisted artifacts (manifest bytes, journal bytes, search output), and those are the right place to assert parity.

**Phase A — Trust kernel (cacg-core):** schema + normalize + hash + canonical_json + diagnostic. Snapshot-test against Python golden corpus. ~4-6 weeks.

**Phase B — Layer 1 + Layer 2 + atomic publish + journal:** can already run `kb lint` / `kb verify` against a Python-built corpus and assert byte-equal journal output. ~4-6 weeks.

**Phase C — Index + search + FTS5 sidecar:** `kb index` + `kb search`. Cross-validate against Python builds (the sidecar BYTES will differ per DEC-8, but the seal `meta.summaries_hash` and query row-set must match). ~3-4 weeks.

**Phase D — Ingest:** `kb ingest` with pdfium-render. The most-risky phase (PDF byte-extraction parity). Golden-test against existing chunk_hashes; if parity holds, ship; if not, document divergence + version-bump chunk_hash policy. ~3-4 weeks.

**Phase E — Operator verbs:** `retract*`, `scaffold-*`, `show`, `migrate-summaries`. ~2-3 weeks.

**Total:** ~16-23 weeks for full feature parity. Each phase ends with a parity gate: `cargo run --bin kb -- <cmd> ...` and `python -m cacg.cli <cmd> ...` produce byte-equal output on the same fixture.

### 7.2 Alternative: Polyglot Coexistence (NOT Recommended)

Could ship `kb-rs` as a Rust binary alongside `kb` (Python) and let humanize choose. Avoid this: two implementations means two trust kernels, and any drift between them is a silent correctness bug. Pick one and commit.

### 7.3 Rollback Path

The Python implementation isn't being deleted. Keep it tagged at the last commit before Rust-port-ships, and bring it back if Phase D parity gates fail. The persisted artifacts (`chunks_manifest.json`, `cards_manifest.json`, etc.) are the contract; both implementations operate on the same files.

---

## 8. Risk Register

| ID | Risk | Probability | Impact | Mitigation |
|----|------|-------------|--------|------------|
| R1 | Pdfium-render output differs from pypdfium2 5.8.0 at byte level | HIGH | HIGH | Golden tests BEFORE shipping; if divergence, document + bump chunk_hash policy with explicit migration |
| R2 | Custom canonical JSON has an undiscovered escape edge case that Python handles differently | MEDIUM | HIGH | Adversarial property tests covering every Unicode plane; cross-impl byte-equal CI gate |
| R3 | BM25 floating-point determinism drifts across architectures (x86_64 vs aarch64 vs riscv) | LOW | MEDIUM | f64 throughout; round to 6 decimals at serialize boundary; card_id tiebreak resolves ε ties |
| R4 | YAML 1.1 vs 1.2 scalar drift in frontmatter parsing | LOW | MEDIUM | Adversarial fixtures + restricted scalar grammar; CACG schemas don't use the divergent constructs |
| R5 | FTS5 tokenizer drift across SQLite versions | LOW | MEDIUM | Bundle SQLite via rusqlite; pin version in CI |
| R6 | `rustix::io::write` short-write on a > PIPE_BUF journal line | LOW | LOW | Preflight length check (CACG-JNL contract already documents this) |
| R7 | rayon parallel-iteration introduces non-determinism in journal order | MEDIUM | LOW | Collect → sort by card_path → append serially under lock |
| R8 | pdfium native crash on malformed PDF panics the process | MEDIUM | LOW | catch_unwind boundary; surface as CACG-INGEST-001 |
| R9 | Pdfium binary version pin diverges from CI host (Ubuntu LTS upgrade) | MEDIUM | MEDIUM | Vendor Pdfium binary into repo (or release artifacts); reproducible-build pin via cargo-vet or vendored .so |
| R10 | clap derive macro generates argument parsing that subtly differs from argparse | LOW | LOW | Snapshot tests on `kb --help` and `kb <cmd> --help`; exit-code regression tests |
| R11 | Stale dependency (rapidfuzz) recommendation gets propagated through ecosystem | LOW | LOW | Don't use it; roll our own fuzzy (30 LoC) |
| R12 | Compile-time feature flag combinatorics let Pdfium leak into common path | MEDIUM | HIGH | `cargo tree -e features` snapshot CI test for each non-ingest binary feature set |

---

## 9. Verification Plan

### 9.1 Parity Tests (Cross-Implementation)

The acceptance criterion for every phase: `python -m cacg.cli <cmd> <args>` and `cargo run --bin kb -- <cmd> <args>` produce byte-identical output on the same fixture under `KB_FROZEN_CLOCK=1`.

```bash
# Concrete parity harness (xtask)
KB_FROZEN_CLOCK=1 python -m cacg.cli verify cards/r01/g.md \
    --chunks-manifest out/chunks_manifest.json \
    --source-matrix out/source_matrix.json > out.python.txt

KB_FROZEN_CLOCK=1 cargo run --bin kb -- verify cards/r01/g.md \
    --chunks-manifest out/chunks_manifest.json \
    --source-matrix out/source_matrix.json > out.rust.txt

diff out.python.txt out.rust.txt  # MUST be empty
sha256sum out/lint_journal.jsonl  # MUST match the pre-recorded golden hash
```

### 9.2 Snapshot Tests (Per Artifact)

Use `insta` for:
- `chunks_manifest.json` bytes (post-ingest)
- `cards_manifest.json` bytes (post-index)
- `summaries.json` bytes (post-index)
- `INDEX.md` bytes (post-index)
- `lint_journal.jsonl` bytes (under FROZEN_CLOCK)
- `<slug>.history.jsonl` bytes (under FROZEN_CLOCK)
- `kb search "duration" --source-matrix m.json --json` stdout

### 9.3 Property Tests (Algorithmic Invariants)

Use `proptest` for:
- `normalize(normalize(x)) == normalize(x)` (idempotence)
- `canonical_json(parse_json(canonical_json(x))) == canonical_json(x)` (round-trip)
- `card_hash(fm, body)` ≠ `card_hash(fm', body)` whenever `fm ≠ fm'` after canonical-JSON
- BM25 ranking is total-ordered by (score DESC, card_id ASC)

### 9.4 Adversarial Fixtures

Port all 12 Python adversarial cards verbatim:
- `01-malformed-hash.md` → CACG-CITE-002
- `02-reversed-page-range.md` → CACG-CITE-003
- ... (see `docs/lint-codes.md` adversarial-coverage map)

Plus Rust-only adversarial:
- YAML anchor injection
- YAML duplicate keys (parses succeed silently if Visitor not strict)
- YAML merge-key gadgets
- Canonical JSON with U+2028 (line separator) and U+2029 (paragraph separator) — JS / JSON edge case
- Partial-write torn journal line (PIPE_BUF + 1 byte payload)
- Rollback debris from previous crashed run (pre-existing .tmp / .bak)
- Concurrent journal appenders racing for the lock

### 9.5 Performance Budgets (iai-callgrind)

Mirror current Python budgets as instruction-count budgets. Set initial values at first green-CI run + 5-10% headroom. CI-blocking on regression beyond budget.

---

## 10. Open Decisions / Things to Benchmark Before Committing

1. **pdfium-render vs lopdf for `kb ingest`.** If Pdfium-render byte-output to pypdfium2 parity holds → pdfium-render. If not, escalate: vendor exact Pdfium build OR commit to lopdf + own the extraction stack. Decision gate: golden-test pass on the existing sample.pdf + a representative CFA_reading PDF (e.g., the McNeil 2015 QRM textbook, which has tables, footnotes, ligatures, and mathematical typography).

2. **sha2 0.10.9 vs 0.11.0.** Benchmark `source_sha256` on a 50MB PDF on at least x86_64-with-SHA-NI and aarch64. If 0.11.0 gives ≥ 2× speedup on supported chips with no regression on others, use 0.11.0. Otherwise 0.10.9 conservative.

3. **rusqlite version of bundled SQLite vs system SQLite.** Run identical FTS5 queries against both; verify row-set equality. Decision gate: snapshot test row-set hash matches Python output.

4. **garde vs validator vs hand-written validators.** Benchmark a 10k-card cards_manifest.json validate pass through both crates. If garde adds > 10% overhead vs hand-written, drop it for the hot path schemas (CardFrontmatter, ChunkRecord) and keep it only for the rare-validation surface (SourceMatrix, role-map).

5. **rayon chunk size for `kb verify --round-summary`.** Default `par_iter` parallelism is one card per task; for very fast per-card work, this may have rayon overhead exceeding the work. Benchmark `chunks(50).par_bridge()` vs `par_iter()`.

6. **Should Layer-3 B1 cache lookup be in `cacg-core` or `cacg-semantic`?** B1 is a strict dict lookup, no model invocation. Putting it in core keeps the optional-feature footprint smaller. Recommend `cacg-core` for B1, `cacg-semantic` for B2 only.

7. **`tracing-subscriber` JSON output vs custom serializer for tracing events.** The CACG journal is canonical-JSON. If we want to also emit `tracing` events with byte-identical formatting, we need a custom subscriber layer. Alternative: emit tracing events in plain text (default) and reserve canonical JSON for the journal proper.

8. **Should the Rust port emit a different `parser_version` on `source_record`?** Currently Python emits `"pypdfium2"` + `"5.8.0+pdfium149.0.7825.0"`. Rust will emit `"pdfium-render"` + `"<crate version>+pdfium<binary version>"`. This changes the `SourceRecord.parser_name` value but NOT the source_sha256 or chunk_hash. The schema accepts arbitrary strings; downstream code only displays them. Safe change.

---

## 11. Sources

- Codex consultation, `gpt-5.5:high`, 2026-05-20 (output preserved at `.humanize/skill/2026-05-20_15-22-42-467929-8b81137b/output.md`).
- Crate version data verified against `crates.io` JSON API on 2026-05-20.
- Current Python implementation: `src/cacg/**/*.py` (~11k LoC).
- Legacy reference: `../CFA_reading/scripts/kb/*.py` (~13k LoC, atomic-publish pattern origin).
- Existing CACG docs: `docs/schema.md`, `docs/lint-codes.md`, `docs/integration-with-humanize.md`, `docs/retrieval.md`, `docs/semantic-verifier.md`, `docs/stress-10k.md`.
- Existing CACG plans: `.humanize/.humanize/plans/cacg-mvp-plan.md`, `.humanize/.humanize/plans/cacg-phase-3-retrieval-semantic-dag-plan.md`, `.humanize/.humanize/plans/cacg-phase-4-trust-perf-plan.md`, `.humanize/.humanize/plans/cacg-phase-5-closure-operator-governance-plan.md`, `.humanize/.humanize/plans/cacg-trust-depth-plan.md`.
- pdfium-render docs: <https://docs.rs/pdfium-render>
- pypdfium2 API: <https://pypdfium2.readthedocs.io/>
- yaml-rust2 events: <https://docs.rs/yaml-rust2>
- saphyr docs: <https://docs.rs/saphyr>
- garde docs: <https://docs.rs/garde>
- rusqlite docs: <https://docs.rs/rusqlite>
- SQLite FTS5: <https://www.sqlite.org/fts5.html>
- rustix docs: <https://docs.rs/rustix>
- tempfile docs: <https://docs.rs/tempfile>
- miette docs: <https://docs.rs/miette>
- insta docs: <https://docs.rs/insta>
- proptest docs: <https://docs.rs/proptest>
- iai-callgrind docs: <https://docs.rs/iai-callgrind>
- RFC 8785 JCS: <https://www.rfc-editor.org/rfc/rfc8785> (NOT recommended for this project — see §3.4)
