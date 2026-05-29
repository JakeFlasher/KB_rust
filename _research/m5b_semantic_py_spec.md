# M5b Semantic Python Oracle Specification (Python → Rust Port)

**Date:** 2026-05-24
**Source oracles:**
- `src/cacg/verify/semantic.py` (244 lines) — `SemanticCache`, `SemanticVerdict`, `claim_window_hash`, `load_semantic_cache`, `dump_semantic_cache`, `_cache_lookup`, `_judge_via_claude`, `run_semantic_check`.
- `src/cacg/cli.py` lines 837–1000 — `_build_semantic_spec`, `_SEMANTIC_LOAD_FAILED` sentinel, semantic-arg handling in `_cmd_verify` and `_cmd_verify_round_summary`.

**Target ports:**
- `crates/cacg-semantic/src/lib.rs` (currently a 26-line `cacg_semantic_version()` placeholder) → full B1 cache-as-oracle runtime in task-m5b-8.
- `crates/cacg-cli/src/main.rs` `dispatch_verify` → `--semantic <path>` argument handling mirroring `_build_semantic_spec` in task-m5b-11.
- B2 LLM-judge trait + injectable client (`LlmJudgeClient`, `HaikuClient`, `MockJudgeClient`) → task-m5b-27..32 (forward-looking only in this spec).

**Authored under:** RLCR Round 7 of the M5b loop (`.humanize/rlcr/2026-05-24_14-58-59`).
**Plan task:** `task-m5b-7` (analyze tag, routed to Codex via `/humanize:ask-codex`).
**Target acceptance criteria:** AC-1, AC-1.1, AC-2 of `.humanize/.humanize/plans/cacg-layer3-semantic-port-plan.md`.
**BitLessons applied:**
- `BL-20260518-reject-duplicate-keys-at-trust-boundary` — § 3 enumerates every object level where duplicate keys must be rejected; § 12 pseudocode shows the custom `Deserializer` + `Visitor` approach (NOT a post-parse `serde_json::Value` scan, per Codex residual risk R1).
- `BL-20260522-port-pydantic-validators-not-just-fields` — § 1 + § 6 enumerate every `@field_validator` / `@model_validator` / `Field(default_factory=...)` so the Rust port reproduces ALL of them, not just the field shapes.
- `BL-20260518-shape-check-fs-inputs` — § 3 documents the `is_file()` precheck for the cache path before any `read_text()` call.

**Convergence status:** This document is the **definitive byte-equality oracle reference** for `task-m5b-8` (port `SemanticCache` + `SemanticVerdict` + `claim_window_hash` to Rust). When Rust behaviour disagrees with this spec, the Python source at the cited line range wins — re-derive from `semantic.py` / `cli.py`, then update this spec to reflect the corrected reading.

---

# M5b Semantic Python Oracle Specification

Target output path: `_research/m5b_semantic_py_spec.md`

Source oracle:

- `src/cacg/verify/semantic.py`, lines 1-244.
- `src/cacg/cli.py`, lines 164-190 and 837-1003.
- Supporting oracle lines:
  - `src/cacg/normalize.py`, lines 12-44.
  - `src/cacg/hash.py`, lines 97-102.
  - `src/cacg/schema.py`, lines 17-18 and 43-47.
  - `src/cacg/verify/layer2.py`, lines 45-62 and 272-349.
  - `src/cacg/verify/runner.py`, lines 39-51 and 169-177.

BitLessons driving this audit:

- `BL-20260518-reject-duplicate-keys-at-trust-boundary`
  - Section 3 explicitly distinguishes the current Python behavior from the Rust port requirement.
  - The Rust port must reject duplicate JSON object keys at every object level before materializing JSON.
  - A post-parse `serde_json::Value` scan is not sufficient.
- `BL-20260522-port-pydantic-validators-not-just-fields`
  - Section 1 enumerates fields, defaults, `Field(...)` constraints, `PrivateAttr`, `@field_validator`s, and inherited `_StrictModel` config.
  - The Rust port must reproduce semantic validation, not only the struct shape.
- `BL-20260518-shape-check-fs-inputs`
  - Section 3 and Section 9 call out the CLI `Path.is_file()` precheck before loading the semantic cache.

Important observed divergences:

- Python rejects duplicate semantic composite keys, meaning duplicate `(chunk_hash, claim_window_hash)` entries, at `semantic.py` lines 115-129.
- Python does not reject duplicate JSON object member names before Pydantic materialization. Observed behavior is last-write-wins for duplicate JSON keys.
- The M5b plan requires Rust to be stricter than current Python on duplicate JSON object member names.
- Python emits `CACG-VERIFY-002` severity `error` for both `fail` and `abstain`, and `warning` for `pass`, at `layer2.py` lines 326-340.
- The M5b plan text says `abstain` should be `info`; that is not current Python behavior.

# 1. Module-Level Types

## 1.1 `_StrictModel`

Oracle location:

- `src/cacg/schema.py`, lines 43-47.
- Imported by `semantic.py` at line 28.

Python shape:

```python
class _StrictModel(BaseModel):
    model_config = ConfigDict(
        extra="forbid",
        frozen=False,
        str_strip_whitespace=False,
    )
```

Semantics:

- Unknown fields are rejected.
- Instances are not frozen.
- String fields are not stripped.
- This model is called `_StrictModel`, but it does not set `strict=True`.
- Pydantic v2 coercions can still occur where field types allow them.
- Observed example: JSON `"score": "0.5"` is accepted and coerced to `0.5`.

Rust equivalent:

- Use `#[serde(deny_unknown_fields)]` on the Rust structs for Pydantic `extra="forbid"`.
- Do not rely only on serde field declarations.
- Add a separate `validate_structurally(&self) -> Result<(), SemanticError>` per `BL-20260522-port-pydantic-validators-not-just-fields`.
- Decide explicitly whether to reproduce Pydantic coercion or enforce strict JSON numeric fields.
- For byte-equality with current Python, score strings are accepted by Python.
- For the M5b plan's "strict canonical-JSON validation", score strings likely should be rejected.
- This is an unresolved policy choice; see Section 11.

## 1.2 `SchemaVersionLiteral`

Oracle location:

- `src/cacg/schema.py`, lines 17-18.
- Used by `SemanticCache` at `semantic.py` line 110.

Python shape:

```python
SCHEMA_VERSION = "cacg.v0"
SchemaVersionLiteral = Literal["cacg.v0"]
```

Semantics:

- Only `"cacg.v0"` is valid.
- `"cacg.v1"` and `"cacg.v9"` are rejected by Pydantic literal validation.
- Tests cover wrong schema version in `tests/test_phase3_semantic_verifier.py` lines 85-87.

Rust equivalent:

- Deserialize as a string.
- Validate exact equality to `"cacg.v0"`.
- Emit a stable semantic-cache load error that CLI maps to `CACG-MAN-001`.
- Keep version-mismatch error text distinct from unknown-field strict-schema error text for AC-1.1.

## 1.3 `SemanticMode`

Oracle location:

- `semantic.py`, line 31.

Python shape:

```python
SemanticMode = Literal["embedding-cache", "llm-judge"]
```

Semantics:

- Valid wire strings:
  - `"embedding-cache"`
  - `"llm-judge"`
- Used by `SemanticVerdict.mode`.
- Used by `run_semantic_check(mode=...)`.

Rust equivalent:

```rust
enum SemanticMode {
    EmbeddingCache,
    LlmJudge,
}
```

Wire mapping:

- `EmbeddingCache` serializes to `"embedding-cache"`.
- `LlmJudge` serializes to `"llm-judge"`.

Validation note:

- Python type hints are not runtime guards for `run_semantic_check`.
- If a caller passes a non-`"embedding-cache"` string to `run_semantic_check`, Python falls through to `_judge_via_claude` at `semantic.py` lines 235-244.
- Rust should use an enum to make this impossible at the type boundary.

## 1.4 `SemanticVerdictKind`

Oracle location:

- `semantic.py`, line 32.

Python shape:

```python
SemanticVerdictKind = Literal["pass", "fail", "abstain"]
```

Wire strings:

- `"pass"`
- `"fail"`
- `"abstain"`

Rust equivalent:

```rust
enum SemanticVerdictKind {
    Pass,
    Fail,
    Abstain,
}
```

Wire mapping:

- `Pass` serializes to `"pass"`.
- `Fail` serializes to `"fail"`.
- `Abstain` serializes to `"abstain"`.

## 1.5 `SemanticVerdict`

Oracle location:

- `semantic.py`, lines 35-58.
- Tests at `tests/test_phase3_semantic_verifier.py` lines 722-739.

Python class shape:

```python
class SemanticVerdict(_StrictModel):
    verdict: SemanticVerdictKind
    score: float = Field(ge=0.0, le=1.0)
    reasoning: str | None = None
    mode: SemanticMode = "embedding-cache"

    @field_validator("score", mode="after")
    @classmethod
    def _reject_nan_and_inf(cls, v: float) -> float:
        import math

        if math.isnan(v) or math.isinf(v):
            raise ValueError(f"score must be a finite number; got {v!r}")
        return v
```

Fields:

| Field | Python type | Required | Default | Constraints |
|---|---:|---:|---:|---|
| `verdict` | `Literal["pass", "fail", "abstain"]` | yes | none | closed-set literal |
| `score` | `float` | yes | none | `ge=0.0`, `le=1.0`, finite validator |
| `reasoning` | `str | None` | no | `None` | no length bound |
| `mode` | `Literal["embedding-cache", "llm-judge"]` | no | `"embedding-cache"` | closed-set literal |

Inherited config:

- `extra="forbid"` from `_StrictModel`.
- Unknown fields rejected.
- `str_strip_whitespace=False`.
- `frozen=False`.

Validators:

- `score` Pydantic field constraints:
  - `score >= 0.0`
  - `score <= 1.0`
- `score` explicit validator:
  - rejects `NaN`.
  - rejects positive infinity.
  - rejects negative infinity.
  - error message: `score must be a finite number; got {v!r}` if this validator is reached.

Observed validation details:

- `score=-0.5` rejected.
- `score=1.5` rejected.
- `score=float("nan")` rejected.
- `score=float("inf")` rejected.
- `verdict="bogus"` rejected.
- `mode="bad-mode"` rejected.
- These are tested at `tests/test_phase3_semantic_verifier.py` lines 728-739.

Rust equivalent:

```rust
struct SemanticVerdictWire {
    verdict: SemanticVerdictKind,
    score: f64,
    reasoning: Option<String>,
    mode: SemanticMode,
}

impl SemanticVerdictWire {
    fn validate_structurally(&self) -> Result<(), SemanticError> {
        if !self.score.is_finite() {
            return Err(SemanticError::InvalidScoreFinite);
        }
        if !(0.0..=1.0).contains(&self.score) {
            return Err(SemanticError::InvalidScoreRange);
        }
        Ok(())
    }
}
```

Rust notes:

- Serde alone does not fully reproduce Pydantic validators.
- Reject non-finite values explicitly with `f64::is_finite()`.
- Reject out-of-range values explicitly.
- Enforce enum membership through serde enum deserialization plus structural validation.
- Add `#[serde(default = "default_embedding_cache_mode")]` for `mode`.
- Add `#[serde(default)]` for `reasoning`.
- Add `#[serde(deny_unknown_fields)]`.

## 1.6 `SemanticCacheEntry`

Oracle location:

- `semantic.py`, lines 76-92.
- Tests at `tests/test_phase3_semantic_verifier.py` lines 34-71.

Python class shape:

```python
class SemanticCacheEntry(_StrictModel):
    chunk_hash: str = Field(min_length=64, max_length=64)
    claim_window_hash: str = Field(min_length=64, max_length=64)
    verdict: SemanticVerdictKind
    score: float = Field(ge=0.0, le=1.0)

    @field_validator("chunk_hash", "claim_window_hash")
    @classmethod
    def _is_64_hex(cls, v: str) -> str:
        if not all(c in "0123456789abcdef" for c in v):
            raise ValueError("hash must be 64-hex SHA256")
        return v
```

Fields:

| Field | Python type | Required | Default | Constraints |
|---|---:|---:|---:|---|
| `chunk_hash` | `str` | yes | none | min length 64, max length 64, lowercase hex only |
| `claim_window_hash` | `str` | yes | none | min length 64, max length 64, lowercase hex only |
| `verdict` | `Literal["pass", "fail", "abstain"]` | yes | none | closed-set literal |
| `score` | `float` | yes | none | `ge=0.0`, `le=1.0` |

Inherited config:

- `extra="forbid"` from `_StrictModel`.
- Unknown fields rejected.

Validators:

- `chunk_hash` and `claim_window_hash`:
  - Pydantic length constraints require exactly 64 characters.
  - Validator requires every character to be in `"0123456789abcdef"`.
  - Uppercase hex is rejected.
  - Non-hex lowercase letters such as `z` are rejected.
  - Error message: `hash must be 64-hex SHA256`.

Score validation:

- `Field(ge=0.0, le=1.0)` rejects values outside `[0, 1]`.
- There is no explicit finite validator on `SemanticCacheEntry.score`.
- Observed current Pydantic rejects `NaN` and `Infinity` through the range constraint.
- Rust should still implement explicit finite rejection for portability.

No fields:

- No `reasoning`.
- No `mode`.
- No `evidence`.

Rust equivalent:

```rust
struct SemanticCacheEntryWire {
    chunk_hash: String,
    claim_window_hash: String,
    verdict: SemanticVerdictKind,
    score: f64,
}

impl SemanticCacheEntryWire {
    fn validate_structurally(&self) -> Result<(), SemanticError> {
        validate_sha256_lower_hex(&self.chunk_hash, "chunk_hash")?;
        validate_sha256_lower_hex(&self.claim_window_hash, "claim_window_hash")?;
        validate_score(self.score)?;
        Ok(())
    }
}
```

## 1.7 `SemanticCache`

Oracle location:

- `semantic.py`, lines 95-149.
- Tests at `tests/test_phase3_semantic_verifier.py` lines 74-112 and 742-777.

Python class shape:

```python
class SemanticCache(_StrictModel):
    schema_version: SchemaVersionLiteral
    entries: list[SemanticCacheEntry] = Field(default_factory=list)

    _index: dict[tuple[str, str], "SemanticCacheEntry"] | None = PrivateAttr(default=None)

    @field_validator("entries", mode="after")
    @classmethod
    def _reject_duplicate_keys(cls, v: list[SemanticCacheEntry]) -> list[SemanticCacheEntry]:
        seen: set[tuple[str, str]] = set()
        duplicates: list[tuple[str, str]] = []
        for entry in v:
            key = (entry.chunk_hash, entry.claim_window_hash)
            if key in seen:
                duplicates.append(key)
            seen.add(key)
        if duplicates:
            raise ValueError(
                f"semantic_cache has duplicate keys: {sorted(set(duplicates))}"
            )
        return v

    def lookup(
        self,
        chunk_hash: str,
        claim_window_hash: str,
    ) -> "SemanticCacheEntry | None":
        if self._index is None:
            self._index = {
                (e.chunk_hash, e.claim_window_hash): e for e in self.entries
            }
        return self._index.get((chunk_hash, claim_window_hash))
```

Fields:

| Field | Python type | Required | Default | Constraints |
|---|---:|---:|---:|---|
| `schema_version` | `Literal["cacg.v0"]` | yes | none | exact literal |
| `entries` | `list[SemanticCacheEntry]` | no | `Field(default_factory=list)` | duplicate composite-key validator |

Private attributes:

| Attribute | Python type | Default | Purpose |
|---|---:|---:|---|
| `_index` | `dict[tuple[str, str], SemanticCacheEntry] | None` | `None` | lazy O(1) lookup index |

Inherited config:

- `extra="forbid"` from `_StrictModel`.
- Unknown top-level fields rejected.

Validators:

- `entries` after-validator rejects duplicate semantic composite keys.
- Duplicate key means duplicate tuple:
  - `(entry.chunk_hash, entry.claim_window_hash)`
- Error text prefix:
  - `semantic_cache has duplicate keys: ...`
- The duplicate list is deduplicated through `sorted(set(duplicates))`.
- Python tuple repr appears in the final error text.

Default factory:

- `entries` uses `Field(default_factory=list)`.
- JSON with only `{"schema_version":"cacg.v0"}` loads as an empty cache.
- Dumping that cache emits `"entries":[]`.

Lookup method:

- Builds `_index` on first lookup.
- Reuses the same dict on later lookup calls.
- Keeps index per instance.
- Does not share state across cache instances.
- Does not validate lookup arguments.
- Returns `SemanticCacheEntry | None`.

Rust equivalent:

```rust
pub struct SemanticCache {
    schema_version: String,
    entries: Vec<SemanticCacheEntry>,
    index: HashMap<(String, String), SemanticCacheEntry>,
}
```

Rust validation:

- Validate schema version exactly.
- Validate all entries.
- Validate duplicate semantic composite keys before building the final index.
- Build `HashMap<(String, String), SemanticCacheEntry>` during load.
- Preserve `entries` order for canonical serialization.
- Do not sort entries unless the Python builder does so before constructing the cache.

# 2. `claim_window_hash` Byte-Equality Contract

## 2.1 Function location and signature

Oracle location:

- `semantic.py`, lines 60-73.

Python function:

```python
def _claim_window_hash(quote: str) -> str:
    import hashlib

    normalized = normalize_text(quote)
    return hashlib.sha256(normalized.encode("utf-8")).hexdigest()
```

Rust public name:

- Python name is private: `_claim_window_hash`.
- Rust target should expose public `claim_window_hash`.

Rust signature:

```rust
pub fn claim_window_hash(quote: &str) -> String
```

## 2.2 Normalization dependency

Oracle location:

- `semantic.py`, lines 72-73.
- `normalize.py`, lines 12-44.

The hash input is:

1. Original Python `str` quote.
2. Pass through `normalize_text`.
3. Encode normalized string as UTF-8.
4. SHA-256 over those UTF-8 bytes.
5. Return lowercase hexadecimal digest.

## 2.3 Exact normalization pipeline

Oracle location:

- `normalize.py`, lines 26-44.

Pipeline order:

1. Unicode NFC:
   - `unicodedata.normalize("NFC", text)`
   - Source: `normalize.py` line 38.
2. Common Latin ligature replacement:
   - Source table: `normalize.py` lines 12-20.
   - Applied after NFC.
   - Only runs replacement loop if any ligature char is present.
3. Hyphenated line-break rejoin:
   - Regex: `re.compile(r"-\s*\n\s*")`
   - Replacement: empty string.
   - Source: `normalize.py` lines 22 and 42.
4. Whitespace collapse:
   - Regex: `re.compile(r"\s+")`
   - Replacement: single ASCII space `" "`.
   - Source: `normalize.py` lines 23 and 43.
5. Strip leading and trailing whitespace:
   - Source: `normalize.py` line 44.

## 2.4 Ligature mapping

Oracle location:

- `normalize.py`, lines 12-20.

Mappings:

| Unicode | Name | Replacement |
|---|---|---|
| `ﬀ` | Latin small ligature ff | `ff` |
| `ﬁ` | Latin small ligature fi | `fi` |
| `ﬂ` | Latin small ligature fl | `fl` |
| `ﬃ` | Latin small ligature ffi | `ffi` |
| `ﬄ` | Latin small ligature ffl | `ffl` |
| `ﬅ` | Latin small ligature long st | `ft` |
| `ﬆ` | Latin small ligature st | `st` |

Rust equivalent:

- Use Unicode NFC from `unicode-normalization`.
- Then apply literal replacements for exactly these seven characters.
- Do not use NFKC as a shortcut.
- NFKC would normalize more than the Python oracle and would change the contract.

## 2.5 Whitespace behavior

Oracle location:

- `normalize.py`, lines 22-44.

Whitespace details:

- Python regex `\s+` is Unicode whitespace, not ASCII-only.
- ASCII spaces, tabs, LF, CRLF, form-feed, and other Unicode whitespace are collapsed.
- The replacement is one ASCII space.
- Leading and trailing whitespace are stripped after collapse.
- CRLF and LF normalize to the same final text.
- The hyphenated-line-break regex runs before general whitespace collapse.

ASCII whitespace example:

- Input: `"  Alpha\t\n beta   gamma  "`
- Normalized: `"Alpha beta gamma"`
- Hash: `a53fb55d1133524d9f58398c09e03737736a84d98059537401af8e5e7520791b`

## 2.6 CRLF behavior

CRLF example:

- Input: `"line one\r\nline two"`
- Normalized: `"line one line two"`
- Hash: `e490ed577595f61675761642aa202c2f1f59ef292f58c24fe0b431b05eeb86ec`

LF comparison:

- Input: `"line one\nline two"`
- Normalized: `"line one line two"`
- Hash: `e490ed577595f61675761642aa202c2f1f59ef292f58c24fe0b431b05eeb86ec`

Contract:

- CRLF and LF are byte-equal after normalization.

## 2.7 NFC combining marks

NFD example:

- Input: `"Cafe\u0301"`
- Visual text: `Café`
- Normalized: `"Café"`
- Hash: `73473dcc12b763085904a5279d048c4d5b3b008c46f1f32443b99de04aa83a14`

Composed comparison:

- Input: `"Caf\u00e9"`
- Normalized: `"Café"`
- Hash: `73473dcc12b763085904a5279d048c4d5b3b008c46f1f32443b99de04aa83a14`

Contract:

- NFD and NFC equivalent text hash identically.

## 2.8 Supplementary-plane Unicode

Example:

- Input: `"emoji 😀 and CJK 𠀀"`
- Normalized: `"emoji 😀 and CJK 𠀀"`
- Hash: `24c4ef6df24054f42f626403e11e78a00c58b009970fcce360c49ee3ea91f85c`

Rust implications:

- Rust `&str` stores valid UTF-8 scalar values.
- Python hashes UTF-8 bytes of the normalized `str`.
- Rust must hash UTF-8 bytes of the normalized string.
- No UTF-16 surrogate-pair hashing is involved.

## 2.9 Five golden vector classes

These vectors were computed from the current Python implementation.

| Class | Python input literal | Normalized text | Expected `claim_window_hash` |
|---|---|---|---|
| ASCII whitespace | `"  Alpha\t\n beta   gamma  "` | `"Alpha beta gamma"` | `a53fb55d1133524d9f58398c09e03737736a84d98059537401af8e5e7520791b` |
| Ligature `ﬁ -> fi` | `"ofﬁce file"` | `"office file"` | `8169a01c24806359777ff0f5b8ba513d42c9e21e956a79b589ac6cd5cf759937` |
| NFC combining marks | `"Cafe\u0301"` | `"Café"` | `73473dcc12b763085904a5279d048c4d5b3b008c46f1f32443b99de04aa83a14` |
| CRLF vs LF | `"line one\r\nline two"` | `"line one line two"` | `e490ed577595f61675761642aa202c2f1f59ef292f58c24fe0b431b05eeb86ec` |
| Supplementary-plane | `"emoji 😀 and CJK 𠀀"` | `"emoji 😀 and CJK 𠀀"` | `24c4ef6df24054f42f626403e11e78a00c58b009970fcce360c49ee3ea91f85c` |

Additional equivalence vector:

| Inputs | Expected same hash |
|---|---|
| `"ofﬁce"` and `"office"` | `5cc3f82838ba7260203e4590ce03d00e1663d41f6a5167144f5c95d6be2166a0` |

## 2.10 SHA-256 output format

Oracle location:

- `semantic.py`, line 73.

Format:

- 64 lowercase hexadecimal characters.
- Produced by Python `hashlib.sha256(...).hexdigest()`.
- No prefix.
- No newline.
- No uppercase.

Rust equivalent:

- Use `sha2::Sha256`.
- Lowercase hex encode.
- Ensure exactly 64 hex characters.

# 3. Cache Loading + Duplicate-Key Detection

## 3.1 Function location and signature

Oracle location:

- `semantic.py`, lines 150-154.

Python function:

```python
def load_semantic_cache(path: Path) -> SemanticCache:
    return SemanticCache.model_validate_json(path.read_text(encoding="utf-8"))
```

Rust target:

```rust
impl SemanticCache {
    pub fn load(path: &Path) -> Result<SemanticCache, SemanticError>
}
```

## 3.2 File-shape contract

Oracle location:

- CLI precheck: `cli.py`, lines 864-872.
- Loader itself: `semantic.py`, lines 150-154.

Behavior:

- `load_semantic_cache` does not check `is_file()`.
- `_build_semantic_spec` checks `Path(semantic_arg).is_file()` before loading.
- Missing path is rejected before pipeline start.
- Directory path is rejected before pipeline start.
- Non-regular file path is rejected before pipeline start.

CLI diagnostic:

```text
CACG-MAN-001: semantic cache not found or not a regular file: {cache_path}
```

Rust equivalent:

- `dispatch_verify` should perform `path.is_file()` before `SemanticCache::load`.
- Missing files and directories must not enter the verify pipeline.
- This is required by `BL-20260518-shape-check-fs-inputs`.

## 3.3 UTF-8 reading

Oracle location:

- `semantic.py`, line 154.

Behavior:

- Reads with `path.read_text(encoding="utf-8")`.
- Invalid UTF-8 raises `UnicodeDecodeError`.
- I/O errors raise `OSError` subclasses.
- CLI catches these under `except Exception as exc` at `cli.py` lines 873-880.
- CLI maps them to `CACG-MAN-001`.

Rust equivalent:

- Use `std::fs::read_to_string`.
- Map `std::io::Error` to `SemanticError::Read`.
- CLI prints:
  - `CACG-MAN-001: cannot load semantic cache: {err}`

## 3.4 JSON parsing

Oracle location:

- `semantic.py`, line 154.
- Pydantic `model_validate_json`.

Behavior:

- Malformed JSON raises `pydantic.ValidationError`.
- Observed malformed JSON message begins:
  - `Invalid JSON: EOF while parsing an object...`
- CLI maps it to:
  - `CACG-MAN-001: cannot load semantic cache: {exc}`

Rust equivalent:

- Use `serde_json::Deserializer`.
- Convert syntax errors to `SemanticError::Json`.
- CLI maps to `CACG-MAN-001`.

## 3.5 JSON file shape

Valid top-level semantic cache shape:

```json
{
  "schema_version": "cacg.v0",
  "entries": [
    {
      "chunk_hash": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
      "claim_window_hash": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
      "score": 0.5,
      "verdict": "pass"
    }
  ]
}
```

Required:

- `schema_version`.
- `entries` is optional because of `Field(default_factory=list)`.

Rejected by current Python:

- Top-level array.
- Top-level string.
- Top-level number.
- Top-level object missing `schema_version`.
- `entries: null`.
- `entries: {}`.
- Entry missing any required entry field.
- Entry with extra fields.
- Unknown top-level fields.
- Unknown entry fields.

## 3.6 Schema version validation

Oracle location:

- `SemanticCache.schema_version`: `semantic.py`, line 110.
- `SchemaVersionLiteral`: `schema.py`, lines 17-18.

Behavior:

- `"cacg.v0"` accepted.
- `"cacg.v1"` rejected.
- `"cacg.v9"` rejected.
- Tests at `tests/test_phase3_semantic_verifier.py` lines 85-87.

Rust equivalent:

- Reject any value except `"cacg.v0"`.
- Expose stable version-mismatch reason for AC-1.1.
- CLI maps to `CACG-MAN-001`.

## 3.7 Unknown field validation

Oracle location:

- `_StrictModel`: `schema.py`, lines 43-47.
- `SemanticCache`: `semantic.py`, lines 95-112.
- `SemanticCacheEntry`: `semantic.py`, lines 76-85.

Behavior:

- Unknown top-level fields rejected.
- Unknown entry fields rejected.
- Unknown verdict fields rejected for `SemanticVerdict`.
- Observed top-level error type:
  - `extra_forbidden`
- Observed entry extra field error path:
  - `entries.0.reasoning`

Rust equivalent:

- Use `#[serde(deny_unknown_fields)]`.
- Still run structural validation separately.

## 3.8 Hash field validation

Oracle location:

- `semantic.py`, lines 82-92.

Behavior:

- `chunk_hash` must be exactly 64 characters.
- `claim_window_hash` must be exactly 64 characters.
- Every character must be lowercase `0-9` or `a-f`.
- Uppercase hex is rejected.
- Non-hex is rejected.
- Short and long strings are rejected by Pydantic length constraints.
- Error message from custom validator:
  - `hash must be 64-hex SHA256`

Rust equivalent:

```rust
fn validate_sha256_lower_hex(s: &str) -> Result<(), SemanticError> {
    if s.len() != 64 {
        return Err(SemanticError::InvalidHashLength);
    }
    if !s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        return Err(SemanticError::InvalidHashHex);
    }
    Ok(())
}
```

## 3.9 Verdict validation

Oracle location:

- `SemanticVerdictKind`: `semantic.py`, line 32.
- `SemanticCacheEntry.verdict`: `semantic.py`, line 84.
- `SemanticVerdict.verdict`: `semantic.py`, line 45.

Behavior:

- Valid values:
  - `"pass"`
  - `"fail"`
  - `"abstain"`
- Any other string rejected.
- Tests reject `"maybe"` for cache entry at `tests/test_phase3_semantic_verifier.py` lines 54-61.
- Tests reject `"bogus"` for `SemanticVerdict` at lines 728-729.

Rust equivalent:

- Serde enum with `rename_all` or explicit `#[serde(rename = ...)]`.
- Reject unknown variants.

## 3.10 Score validation

Oracle location:

- `SemanticCacheEntry.score`: `semantic.py`, line 85.
- `SemanticVerdict.score`: `semantic.py`, line 46.
- `SemanticVerdict` finite validator: `semantic.py`, lines 50-57.

Behavior:

- Score range is `[0.0, 1.0]`.
- Python accepts `0.0`.
- Python accepts `1.0`.
- Python rejects `-0.5`.
- Python rejects `1.5`.
- Python rejects `NaN`.
- Python rejects `Infinity`.
- `SemanticVerdict` has explicit finite validator.
- `SemanticCacheEntry` relies on Pydantic range constraints to reject non-finite values.
- Observed JSON `"score": "0.5"` is accepted and coerced by Pydantic.

Rust equivalent:

- Always reject `NaN` and infinities explicitly.
- Always enforce `0.0 <= score <= 1.0`.
- Decide whether numeric strings should be accepted; see Section 11.

## 3.11 Duplicate semantic composite keys

Oracle location:

- `semantic.py`, lines 115-129.

Behavior:

- Duplicate `(chunk_hash, claim_window_hash)` across two entries is rejected.
- This is not JSON object-key detection.
- It is semantic duplicate detection after entries have been parsed.
- Test at `tests/test_phase3_semantic_verifier.py` lines 74-82.
- Error text:
  - `semantic_cache has duplicate keys: [...]`

Rust equivalent:

- Use a `HashSet<(String, String)>` or `BTreeSet<(String, String)>`.
- Reject duplicate tuple while validating entries.
- Do this before or during index construction.
- Preserve the original entry vector for serialization order if needed.

## 3.12 Duplicate JSON object keys: current Python behavior

Critical observation:

- Current Python source uses `SemanticCache.model_validate_json(...)` at `semantic.py` line 154.
- Pydantic materializes JSON into Python object semantics.
- Duplicate JSON object member names are not preserved.
- Observed behavior is last-write-wins.

Observed examples:

- Top-level duplicate `schema_version`:
  - Input: `{"schema_version":"cacg.v9","schema_version":"cacg.v0","entries":[]}`
  - Current Python loads successfully.
- Top-level duplicate `entries`:
  - First `entries` has one entry, second `entries` is empty.
  - Current Python loads successfully with zero entries.
- Duplicate `chunk_hash` inside an entry:
  - First value invalid, second value valid.
  - Current Python loads successfully using the second value.
- Duplicate key inside unknown nested object:
  - Python sees only the last value in that nested object, then rejects the unknown top-level field.
  - It does not report duplicate-key detection.

Conclusion:

- Current Python is not a duplicate-JSON-key oracle.
- It is a semantic composite-key duplicate oracle.

## 3.13 Duplicate JSON object keys: Rust port requirement

Plan and BitLesson requirement:

- `BL-20260518-reject-duplicate-keys-at-trust-boundary`.
- M5b plan lines 26-42 require duplicate key rejection:
  - duplicate top-level JSON keys.
  - duplicate keys inside `entries[i]`.
  - duplicate keys inside any nested object.
- The Rust port must observe keys before materialization.

Required rejection levels:

- Top-level semantic cache object:
  - Duplicate `schema_version`.
  - Duplicate `entries`.
  - Duplicate unknown field names.
- Each `entries[i]` object:
  - Duplicate `chunk_hash`.
  - Duplicate `claim_window_hash`.
  - Duplicate `verdict`.
  - Duplicate `score`.
  - Duplicate unknown field names.
- Any nested object at any depth:
  - Duplicate keys must be rejected.
  - This includes nested objects under unknown fields, even though the unknown field will also be rejected.
  - Duplicate-key detection should happen at the raw JSON object boundary.

Do not implement:

- Do not parse to `serde_json::Value` and then scan.
- Duplicate keys are already lost in normal map materialization.
- A post-parse scan cannot recover them.

Recommended Rust implementation:

- Build a duplicate-key-checking JSON deserialization path.
- Use `serde_json::Deserializer` with a custom `Visitor`.
- For every object visit:
  - Maintain a `BTreeSet<String>` of keys seen in that object.
  - On each key, check `insert`.
  - If insert returns false, return an error immediately.
  - Recurse into values while preserving the duplicate-check stack.
- After duplicate checking, deserialize into typed structs.
- Or deserialize once through a custom `DeserializeSeed` that checks duplicates while building typed structs.
- Simpler acceptable approach:
  - First pass raw token visitor rejects duplicate object keys at every depth.
  - Second pass typed serde deserialization from the original bytes.
  - This is still valid because the first pass observed raw object keys before materialization.

## 3.14 Exceptions mapped to `CACG-MAN-001`

Oracle locations:

- `load_semantic_cache`: `semantic.py`, lines 150-154.
- CLI catch-all: `cli.py`, lines 873-880.

Exceptions during cache load:

| Condition | Python exception | CLI behavior |
|---|---|---|
| Missing file | Not loaded; `is_file()` false | `CACG-MAN-001`, exit 1 |
| Directory path | Not loaded; `is_file()` false | `CACG-MAN-001`, exit 1 |
| Permission/read error | `OSError` subclass | `CACG-MAN-001`, exit 1 |
| Invalid UTF-8 | `UnicodeDecodeError` | `CACG-MAN-001`, exit 1 |
| Malformed JSON | `pydantic.ValidationError` | `CACG-MAN-001`, exit 1 |
| Schema validation error | `pydantic.ValidationError` | `CACG-MAN-001`, exit 1 |
| Duplicate semantic composite key | `pydantic.ValidationError` | `CACG-MAN-001`, exit 1 |

CLI output for load exception:

```text
CACG-MAN-001: cannot load semantic cache: {exc}
```

# 4. Canonical Serialization

## 4.1 Function location and signature

Oracle location:

- `semantic.py`, lines 157-159.

Python function:

```python
def dump_semantic_cache(cache: SemanticCache) -> str:
    return canonical_json(cache.model_dump(mode="json"))
```

Supporting oracle:

- `hash.py`, lines 97-102.

```python
def canonical_json(obj: Any) -> str:
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
```

Rust target:

```rust
impl SemanticCache {
    pub fn to_canonical_json(&self) -> Result<String, SemanticError>
}
```

## 4.2 Exact JSON flags

Python `json.dumps` flags:

| Flag | Value |
|---|---:|
| `sort_keys` | `True` |
| `separators` | `(",", ":")` |
| `ensure_ascii` | `False` |
| `indent` | default `None` |
| `allow_nan` | default `True`, but semantic models validate scores first |

Serialization effects:

- Object keys are sorted lexicographically at every object level.
- No spaces after commas.
- No spaces after colons.
- Non-ASCII characters are emitted as UTF-8 characters, not `\uXXXX`, where Python chooses direct output.
- No trailing newline.
- No pretty-print indentation.

## 4.3 Entry ordering

Oracle location:

- `dump_semantic_cache`: `semantic.py`, lines 157-159.
- No sorting logic elsewhere in `SemanticCache`.

Behavior:

- `entries` are not sorted by `(chunk_hash, claim_window_hash)` during dump.
- `entries` preserve the list order held by the model.
- `json.dumps(sort_keys=True)` sorts object keys, not array elements.
- If the builder wants sorted entries, it must construct `entries` in sorted order before dumping.

Rust equivalent:

- Preserve `Vec<SemanticCacheEntry>` order.
- Do not sort entries inside canonical serialization unless matching a separate builder contract.
- Keep a separate `HashMap` index for lookup.

## 4.4 Example canonical serialization

For:

```python
SemanticCache(
    schema_version="cacg.v0",
    entries=[
        SemanticCacheEntry(
            chunk_hash="a" * 64,
            claim_window_hash="b" * 64,
            verdict="pass",
            score=0.5,
        )
    ],
)
```

Python output:

```json
{"entries":[{"chunk_hash":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","claim_window_hash":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb","score":0.5,"verdict":"pass"}],"schema_version":"cacg.v0"}
```

Notes:

- Top-level key order is `"entries"` then `"schema_version"` because keys are sorted.
- Entry key order is `"chunk_hash"`, `"claim_window_hash"`, `"score"`, `"verdict"`.
- There is no `mode`.
- There is no `reasoning`.
- There is no trailing newline.

## 4.5 Round-trip property

Test oracle:

- `tests/test_phase3_semantic_verifier.py`, lines 90-104.

Python test:

- Construct model.
- Dump to canonical JSON.
- Parse dumped string with `SemanticCache.model_validate_json`.
- Dump again.
- Assert first dump equals second dump.

Exact property:

- `dump(parse(dump(cache))) == dump(cache)`

Not guaranteed:

- `dump(parse(input_bytes)) == input_bytes` for arbitrary valid JSON.
- If input JSON has whitespace, unsorted keys, omitted `entries`, or different numeric spelling, dump will differ.
- If input JSON already uses canonical output and same entry order, parse-then-dump is byte-equal.

Rust equivalent:

- Implement canonical serializer.
- Test canonical committed cache:
  - read bytes.
  - load with duplicate-key checking and structural validation.
  - serialize.
  - compare serialized bytes to original bytes.
- This only passes if committed cache is in canonical format and entry order is stable.

# 5. `_cache_lookup` Semantics

## 5.1 Function location and signature

Oracle location:

- `semantic.py`, lines 162-188.

Python signature:

```python
def _cache_lookup(
    cache: SemanticCache,
    chunk_hash: str,
    claim_window_hash: str,
) -> SemanticVerdict:
```

Return type:

- `SemanticVerdict`.

Rust target:

```rust
impl SemanticCache {
    pub fn lookup(&self, chunk_hash: &str, claim_window_hash: &str) -> SemanticVerdict
}
```

## 5.2 Keying strategy

Oracle locations:

- `SemanticCache.lookup`: `semantic.py`, lines 131-147.
- `_cache_lookup`: `semantic.py`, lines 162-188.

Python behavior:

- `SemanticCache.lookup` lazily builds `_index`.
- `_index` type:
  - `dict[tuple[str, str], SemanticCacheEntry]`
- Key:
  - `(chunk_hash, claim_window_hash)`
- Lookup:
  - `self._index.get((chunk_hash, claim_window_hash))`
- Complexity after index build:
  - Expected O(1).

Rust equivalent:

- Build `HashMap<(String, String), SemanticCacheEntry>` at load time.
- Lookup by borrowed keys if possible.
- Returning an owned `SemanticVerdict` is fine.
- Preserve the `entries` vector separately for canonical serialization.

## 5.3 Cache hit behavior

Oracle location:

- `semantic.py`, lines 175-182.

Python behavior on hit:

```python
return SemanticVerdict(
    verdict=entry.verdict,
    score=entry.score,
    reasoning=None,
    mode="embedding-cache",
)
```

Semantics:

- `verdict` copied verbatim from cache entry.
- `score` copied verbatim from cache entry.
- `reasoning` is always `None`.
- `mode` is always `"embedding-cache"`.
- Cache entry has no `reasoning` field.
- Cache entry has no `mode` field.

Rust equivalent:

```rust
SemanticVerdict {
    verdict: entry.verdict,
    score: entry.score,
    reasoning: None,
    mode: SemanticMode::EmbeddingCache,
}
```

## 5.4 Cache miss behavior

Oracle location:

- `semantic.py`, lines 183-188.
- Test at `tests/test_phase3_semantic_verifier.py`, lines 160-170.

Python behavior on miss:

```python
return SemanticVerdict(
    verdict="abstain",
    score=0.0,
    reasoning=None,
    mode="embedding-cache",
)
```

Semantics:

- Miss is not a load error.
- Miss is not an exception.
- Miss returns an abstain verdict.
- Score is exactly `0.0`.
- Reasoning is `None`.
- Mode is `"embedding-cache"`.

Rust equivalent:

```rust
SemanticVerdict::abstain_embedding_cache()
```

AC mapping:

- This is AC-1 negative test:
  - Cache miss returns `Verdict::Abstain`.
  - It must not fail cache load.
  - It must not abort verification.

# 6. `SemanticVerdict` Enum + Score + Evidence

## 6.1 Verdict string values

Oracle location:

- `semantic.py`, line 32.

Values:

| Wire string | Meaning |
|---|---|
| `"pass"` | Semantic verifier thinks quote is semantically supported |
| `"fail"` | Semantic verifier thinks quote is semantically unsupported |
| `"abstain"` | No semantic verdict available, usually cache miss |

Rust enum:

```rust
enum SemanticVerdictKind {
    Pass,
    Fail,
    Abstain,
}
```

## 6.2 Score range

Oracle locations:

- `SemanticVerdict.score`: `semantic.py`, line 46.
- Finite validator: `semantic.py`, lines 50-57.
- Tests: `tests/test_phase3_semantic_verifier.py`, lines 730-737.

Score contract:

- Inclusive lower bound: `0.0`.
- Inclusive upper bound: `1.0`.
- Reject `NaN`.
- Reject positive infinity.
- Reject negative infinity.
- `0.0` is the miss-abstain convention.
- No invariant requires `abstain` to have score `0.0` when loaded from cache.
- A cache entry with `verdict="abstain"` and `score=0.7` would pass current validation.
- Only cache misses force `score=0.0`.

Rust equivalent:

- Validate `score.is_finite()`.
- Validate `0.0 <= score <= 1.0`.

## 6.3 Evidence field

Current Python:

- There is no field named `evidence`.
- `SemanticVerdict` has optional `reasoning: str | None = None`.
- `SemanticCacheEntry` has no `reasoning`.
- B1 cache hits set `reasoning=None`.
- B1 cache misses set `reasoning=None`.
- B2 judge can return `reasoning`.

Diagnostic hints:

- Layer-2 diagnostic stores semantic information in `hints`.
- Hint shape at `layer2.py` lines 341-347:

```python
{
    "semantic_verdict": verdict.verdict,
    "semantic_score": verdict.score,
    "semantic_mode": verdict.mode,
}
```

No reasoning in hints:

- `reasoning` is included only in the diagnostic message text when truthy.
- It is not included in the hint dict.

Rust equivalent:

- For the verdict object, use `reasoning: Option<String>`.
- Do not invent a separate `evidence` field for B1.
- If future B2 wants structured evidence, that is a schema change, not current Python parity.

## 6.4 Diagnostic wire formatting

Oracle location:

- `layer2.py`, lines 326-340.

Message template:

```python
f"semantic verdict={verdict.verdict} "
f"score={verdict.score:.6f} "
f"mode={verdict.mode}"
+ (
    f" reasoning={verdict.reasoning!r}"
    if verdict.reasoning
    else ""
)
```

Formatting details:

- Score is formatted to exactly six decimals.
- Python f-string format is `:.6f`.
- `reasoning` is appended only if truthy.
- `reasoning=None` is omitted.
- `reasoning=""` is omitted.
- Non-empty reasoning uses Python `repr`, so strings are quoted with Python repr rules.

Examples:

- Pass without reasoning:
  - `semantic verdict=pass score=0.920000 mode=embedding-cache`
- Fail without reasoning:
  - `semantic verdict=fail score=0.050000 mode=embedding-cache`
- B2 with reasoning:
  - `semantic verdict=pass score=0.880000 mode=llm-judge reasoning='mock verdict'`

Rust equivalent:

- Use `format!("{:.6}", score)`.
- Reproduce Python repr for reasoning if byte-equal message parity is required.
- If reasoning can contain quotes, backslashes, or non-ASCII, Rust needs a Python-repr-compatible helper, not Rust `Debug`.

## 6.5 Diagnostic severity

Oracle location:

- `layer2.py`, lines 326-340.

Current Python rule:

```python
severity="error" if verdict.verdict != "pass" else "warning"
```

Observed mapping:

| Verdict | Python severity |
|---|---|
| `"pass"` | `"warning"` |
| `"fail"` | `"error"` |
| `"abstain"` | `"error"` |

Plan divergence:

- M5b plan line 48 says:
  - `pass` -> warning.
  - `fail` -> error.
  - `abstain` -> info.
- Current Python source does not implement `abstain -> info`.
- Byte-equal Python parity means Rust should emit `error` for `abstain`.
- AC text parity means Rust should emit `info` for `abstain`.
- This must be resolved before locking tests.

## 6.6 CLI stderr hint formatting

Oracle location:

- `cli.py`, lines 951-968.

For semantic hints, CLI prints:

```text
  hint semantic_verdict={hint['semantic_verdict']} score={hint['semantic_score']} mode={hint['semantic_mode']}
```

Details:

- Hint score is raw Python `str(float)`.
- It is not formatted to six decimals in the hint line.
- The main diagnostic message already contains the six-decimal score.
- CLI diagnostic line does not print severity.

# 7. `run_semantic_check` Callsite Contract

## 7.1 Function location and signature

Oracle location:

- `semantic.py`, lines 208-244.

Python signature:

```python
def run_semantic_check(
    *,
    quote: str,
    chunk_text: str,
    chunk_hash: str,
    mode: SemanticMode,
    cache: SemanticCache | None = None,
) -> SemanticVerdict:
```

Arguments:

| Argument | Required | Meaning |
|---|---:|---|
| `quote` | yes | Citation quote from the card |
| `chunk_text` | yes | Full normalized chunk text |
| `chunk_hash` | yes | Cited chunk hash |
| `mode` | yes | `"embedding-cache"` or `"llm-judge"` |
| `cache` | no | Required only for `"embedding-cache"` |

Return type:

- `SemanticVerdict`.

## 7.2 Claim hash computation

Oracle location:

- `semantic.py`, line 235.

Behavior:

```python
claim_hash = _claim_window_hash(quote)
```

Semantics:

- Caller passes raw citation quote.
- `run_semantic_check` computes the normalized claim-window hash internally.
- Callers do not pass `claim_window_hash`.
- For B1 lookup, key is:
  - `chunk_hash`
  - `_claim_window_hash(quote)`

Rust equivalent:

- `run_semantic_check` or the caller must compute `claim_window_hash` exactly once using the same function.
- For API parity, prefer computing inside the semantic layer.

## 7.3 Dispatch behavior

Oracle location:

- `semantic.py`, lines 236-244.

Python dispatch:

```python
if mode == "embedding-cache":
    if cache is None:
        raise ValueError(...)
    return _cache_lookup(cache, chunk_hash, claim_hash)
# mode == "llm-judge"
return _judge_via_claude(quote, chunk_text)
```

B1 behavior:

- Requires a loaded cache.
- `cache=None` raises `ValueError`.
- Error message includes:
  - `embedding-cache mode requires a loaded SemanticCache; got None`
- Test at `tests/test_phase3_semantic_verifier.py` lines 173-181.

B2 behavior:

- Ignores `chunk_hash`.
- Ignores computed `claim_hash`.
- Calls `_judge_via_claude(quote, chunk_text)`.
- Returns whatever `_judge_via_claude` returns.
- Does not force returned `mode` to `"llm-judge"`.

Rust equivalent:

```rust
enum SemanticSpec {
    B1(SemanticCache),
    B2(Box<dyn LlmJudgeClient + Send + Sync>),
}
```

Avoid string dispatch in Rust:

- Use enum dispatch.
- B1 variant carries `SemanticCache`.
- B2 variant carries a judge client.

## 7.4 Upstream callsite

Oracle location:

- `layer2.py`, lines 272-349.

Layer-3 fires only inside the Layer-2 failure branch:

- Exact match already failed.
- Fuzzy match either disabled or rejected.
- A `SemanticSpec` was supplied with either:
  - `cache is not None`, or
  - `judge_enabled=True`.

Callsite:

```python
verdict = _run_semantic_check(
    quote=citation.quote,
    chunk_text=chunk_text,
    chunk_hash=citation.chunk_hash,
    mode=mode,
    cache=semantic.cache,
)
```

Mode selection:

```python
mode = "embedding-cache" if semantic.cache is not None else "llm-judge"
```

Misconfiguration:

- If `semantic.cache is not None and semantic.judge_enabled`, Python appends a `CACG-VERIFY-002` diagnostic with severity `error`.
- It does not call `run_semantic_check`.
- Oracle location: `layer2.py`, lines 288-305.

## 7.5 Flow into `verify_one_card`

Oracle locations:

- `runner.py`, lines 39-51.
- `runner.py`, lines 169-177.
- `layer2.py`, lines 358-477.

Flow:

1. CLI builds `semantic_kwarg`.
2. CLI passes `semantic=semantic_kwarg` to `verify_one_card`.
3. `verify_one_card` passes `semantic=semantic` to `verify_card`.
4. `verify_card` passes `semantic=semantic` to each `verify_citation`.
5. `verify_citation` invokes `run_semantic_check` only after Layer-2 exact/fuzzy failure.

Python single-card CLI pass-through:

- `cli.py`, lines 936-946.

Python round-summary CLI pass-through:

- `cli.py`, lines 1001-1013.

## 7.6 `CACG-VERIFY-002` diagnostic format

Oracle location:

- `layer2.py`, lines 326-349.

Diagnostic fields:

| Field | Value |
|---|---|
| `code` | `"CACG-VERIFY-002"` |
| `severity` | `"warning"` for pass, `"error"` otherwise |
| `message` | semantic verdict message |
| `file` | same `file_path` passed into `verify_citation` |
| `hints` | one dict with semantic verdict, score, mode |

Diagnostic message:

```text
semantic verdict={verdict} score={score:.6f} mode={mode}
```

Optional reasoning suffix:

```text
 reasoning={reasoning!r}
```

Hint dict:

```python
{
    "semantic_verdict": verdict.verdict,
    "semantic_score": verdict.score,
    "semantic_mode": verdict.mode,
}
```

## 7.7 Semantic failure diagnostic

Oracle location:

- `layer2.py`, lines 316-324.

If `run_semantic_check` raises:

```python
Diagnostic(
    code=VERIFY_SEMANTIC,
    severity="error",
    message=f"semantic verifier failed: {exc}",
    file=file_path,
)
```

Rust equivalent:

- Catch semantic runtime errors at the Layer-2 integration boundary.
- Convert to `CACG-VERIFY-002` with severity `error`.
- Do not abort the process.
- Keep the underlying `CACG-VERIFY-001` diagnostic in the same diagnostics array.

# 8. B2 `_judge_via_claude` Forward-Look

## 8.1 Function location and signature

Oracle location:

- `semantic.py`, lines 191-205.

Python signature:

```python
def _judge_via_claude(quote: str, chunk_text: str) -> SemanticVerdict:
```

Current implementation:

```python
raise NotImplementedError(
    "B2 LLM-judge requires monkey-patching `cacg.verify.semantic._judge_via_claude` "
    "in tests (or the real anthropic SDK at deploy time)"
)
```

## 8.2 Current dependencies

Current Python source:

- Imports no requests library.
- Imports no asyncio.
- Imports no Anthropic SDK.
- Performs no HTTP call.
- Always raises unless monkey-patched.

Test oracle:

- `tests/test_phase3_semantic_verifier.py`, lines 184-213:
  - monkey-patches `_judge_via_claude`.
  - verifies B2 dispatch.
- `tests/test_phase3_semantic_verifier.py`, lines 451-456:
  - default helper raises `NotImplementedError`.

## 8.3 CLI help contract

Oracle location:

- `cli.py`, lines 179-190.

CLI help claims:

- `--semantic-judge` is B2 LLM-judge.
- Invokes `claude-haiku-4-5` via Anthropic SDK.
- Uses structured-output prompt.
- Same firing contract as B1:
  - Layer-2 fails.
  - Fuzzy rejects.
- Mutually exclusive with `--semantic`.
- Tests monkey-patch SDK helper.
- Default `kb verify` path makes zero outbound calls.

Current source gap:

- The actual `_judge_via_claude` is only a stub.
- No HTTP shape is implemented in Python.
- No timeout handling is implemented in Python.
- No API key handling is implemented in Python.
- No request/response schema is implemented in Python.

## 8.4 Rust forward-looking boundary

For task-m5b-8:

- No B2 production implementation required.
- B2 is forward-looking for task-m5b-27 through task-m5b-32.

Future Rust shape from plan:

```rust
#[async_trait]
pub trait LlmJudgeClient {
    async fn judge(
        &self,
        chunk_hash: &str,
        claim_window_hash: &str,
        quote: &str,
        chunk_text: &str,
    ) -> Result<SemanticVerdict, JudgeError>;
}
```

Expected future implementation:

- `HaikuClient`.
- Uses `reqwest` and `tokio`.
- Feature-gated behind `b2-llm-judge`.
- No default dependency leakage into `kb`.
- Tests use a mock client or wiremock server.
- Standard CI must not call live Anthropic API.

# 9. CLI Boundary: `_build_semantic_spec`

## 9.1 Argparse flags

Oracle location:

- `cli.py`, lines 164-190.

Python parser:

```python
semantic_group = verify.add_mutually_exclusive_group()

semantic_group.add_argument("--semantic", ...)
semantic_group.add_argument("--semantic-judge", action="store_true", ...)
```

Behavior:

- `--semantic <CACHE_PATH>` and `--semantic-judge` are mutually exclusive.
- If both are supplied, argparse exits with code `2`.
- This happens before `_build_semantic_spec`.

Rust equivalent:

- Clap `ArgGroup` already exists at `crates/cacg-cli/src/lib.rs` lines 200-204.
- Ensure the group enforces mutual exclusion and returns usage error exit code `2`.

## 9.2 Sentinel

Oracle location:

- `cli.py`, lines 837-840.

Python sentinel:

```python
_SEMANTIC_LOAD_FAILED = object()
```

Purpose:

- Distinguish load failure from no semantic flag.
- `_build_semantic_spec` returns the sentinel when it already printed `CACG-MAN-001`.
- Callers return early with exit 1.

Rust equivalent:

- Prefer `Result<Option<SemanticSpec>, SemanticCliError>`.
- `Ok(None)` means no semantic flag.
- `Ok(Some(spec))` means semantic enabled.
- `Err(err)` means print `CACG-MAN-001` and return failure.
- No sentinel object needed in Rust.

## 9.3 `_build_semantic_spec` signature

Oracle location:

- `cli.py`, lines 843-883.

Python signature:

```python
def _build_semantic_spec(args: argparse.Namespace):
```

Returns:

- `None` if no semantic flag.
- `SemanticSpec(cache=cache, judge_enabled=False)` for B1.
- `SemanticSpec(cache=None, judge_enabled=True)` for B2.
- `_SEMANTIC_LOAD_FAILED` on B1 path load failure.

Imports:

- `SemanticSpec` from `.verify.layer2`, line 855.
- `load_semantic_cache` from `.verify.semantic`, line 856.

## 9.4 Default path

Oracle location:

- `cli.py`, lines 858-862.

Behavior:

```python
semantic_arg = getattr(args, "semantic", None)
judge_arg = bool(getattr(args, "semantic_judge", False))

if semantic_arg is None and not judge_arg:
    return None
```

Semantics:

- Default `kb verify` does not load semantic cache.
- Default `kb verify` does not import/run judge network code.
- Layer-3 disabled.

## 9.5 B1 `--semantic <path>` path

Oracle location:

- `cli.py`, lines 864-881.

Steps:

1. Convert argument to `Path`.
2. Check `cache_path.is_file()`.
3. If false:
   - Print `CACG-MAN-001: semantic cache not found or not a regular file: {cache_path}`.
   - Return sentinel.
4. Try `load_semantic_cache(cache_path)`.
5. If any `Exception`:
   - Print `CACG-MAN-001: cannot load semantic cache: {exc}`.
   - Return sentinel.
6. Return `SemanticSpec(cache=cache, judge_enabled=False)`.

Rust equivalent:

```rust
fn build_semantic_spec(args: &VerifyArgs) -> Result<Option<SemanticSpec>, SemanticCliError> {
    if let Some(path) = &args.semantic {
        if !path.is_file() {
            return Err(SemanticCliError::NotRegularFile(path.clone()));
        }
        let cache = SemanticCache::load(path)?;
        return Ok(Some(SemanticSpec::B1(cache)));
    }
    if args.semantic_judge {
        return Ok(Some(SemanticSpec::B2(...)));
    }
    Ok(None)
}
```

## 9.6 B2 `--semantic-judge` path

Oracle location:

- `cli.py`, line 883.

Behavior:

```python
return SemanticSpec(cache=None, judge_enabled=True)
```

Semantics:

- No cache is loaded.
- Judge mode is enabled.
- Actual judge is invoked later only if Layer-2 fails and fuzzy rejects.

Rust equivalent:

- Construct `SemanticSpec::B2(Box<dyn LlmJudgeClient>)` when feature enabled.
- If feature disabled, clap should reject `--semantic-judge` per M5b plan.
- Current Python always parses the flag.

## 9.7 Single-card return-early pattern

Oracle location:

- `cli.py`, lines 926-928.

Python behavior:

```python
semantic_kwarg = _build_semantic_spec(args)
if semantic_kwarg is _SEMANTIC_LOAD_FAILED:
    return EXIT_FAIL
```

Then pass-through:

- `verify_one_card(... semantic=semantic_kwarg, ...)`
- Oracle location: `cli.py`, lines 936-946.

Rust equivalent:

- Build semantic spec before `verify_one_card`.
- On load failure, return failure before pipeline starts.
- Do not append a journal event for cache load failure.
- Do not run Layer-1 or Layer-2.

## 9.8 Round-summary return-early pattern

Oracle location:

- `cli.py`, lines 1001-1003.

Python behavior:

```python
semantic_kwarg = _build_semantic_spec(args)
if semantic_kwarg is _SEMANTIC_LOAD_FAILED:
    return EXIT_FAIL
```

Then pass-through:

- `verify_round_summary(... semantic=semantic_kwarg)`
- Oracle location: `cli.py`, lines 1005-1013.

Rust equivalent:

- Load cache once at startup.
- If load fails, emit exactly one `CACG-MAN-001`.
- Do not iterate cards.
- Do not append per-card journal events.

## 9.9 Crate-boundary implication

Current Python type:

- `SemanticSpec` lives in `src/cacg/verify/layer2.py`, lines 49-62.
- It stores `cache: object | None` to avoid import cycles.
- It stores `judge_enabled: bool`.

Rust plan:

- `SemanticSpec` should live in `cacg-semantic`.
- `cacg-cli` constructs it.
- `cacg-core::verify::verify_one_card` consumes it.

Crate-boundary options:

1. `cacg-core` depends on `cacg-semantic`.
   - Simple.
   - Direct enum and verdict types.
   - Need ensure no default ML/B2 dependency leakage.
2. `cacg-core` defines a trait.
   - `cacg-semantic` implements the trait.
   - Avoids direct core dependency on semantic crate.
   - More abstraction.
   - More complex lifetime/object management.

Recommendation:

- For B1 M5b, direct `cacg-core -> cacg-semantic` dependency is acceptable only if `cacg-semantic` default features include no ML/B2 network deps.
- Keep B2 behind feature gates.
- Add the default dependency audit per AC-9.

# 10. AC-1 / AC-1.1 / AC-2 Oracle Mapping Table

| AC test | Python oracle location | Observed behavior |
|---|---|---|
| AC-1 positive: loads canonical JSON cache file | `semantic.py` lines 150-154; tests lines 107-112 | Reads UTF-8 text and validates with `SemanticCache.model_validate_json`; returns `SemanticCache`. |
| AC-1 positive: in-memory index keyed by `(chunk_hash, claim_window_hash)` | `semantic.py` lines 131-147; tests lines 742-777 | `_index` starts `None`, first lookup builds dict keyed by tuple, later lookup reuses same dict. |
| AC-1 positive: lookup returns pass fixture verdict | `semantic.py` lines 175-182; tests lines 133-158 | Cache hit returns `SemanticVerdict(verdict="pass", score=<entry score>, reasoning=None, mode="embedding-cache")`. |
| AC-1 positive: round-trip canonical serialization | `semantic.py` lines 157-159; `hash.py` lines 97-102; tests lines 90-104 | `dump(parse(dump(cache)))` is byte-equal; entries list order is preserved. |
| AC-1 positive: ASCII whitespace hash vector | `semantic.py` lines 60-73; `normalize.py` lines 26-44 | `"  Alpha\t\n beta   gamma  "` hashes to `a53fb55d1133524d9f58398c09e03737736a84d98059537401af8e5e7520791b`. |
| AC-1 positive: ligature hash vector | `semantic.py` lines 60-73; `normalize.py` lines 12-20 and 38-44 | `"ofﬁce file"` normalizes to `"office file"` and hashes to `8169a01c24806359777ff0f5b8ba513d42c9e21e956a79b589ac6cd5cf759937`. |
| AC-1 positive: NFC combining hash vector | `semantic.py` lines 60-73; `normalize.py` line 38 | `"Cafe\u0301"` normalizes to `"Café"` and hashes to `73473dcc12b763085904a5279d048c4d5b3b008c46f1f32443b99de04aa83a14`. |
| AC-1 positive: CRLF vs LF hash vector | `semantic.py` lines 60-73; `normalize.py` lines 42-44 | CRLF and LF normalize to the same text and hash to `e490ed577595f61675761642aa202c2f1f59ef292f58c24fe0b431b05eeb86ec`. |
| AC-1 positive: supplementary-plane hash vector | `semantic.py` lines 60-73 | `"emoji 😀 and CJK 𠀀"` hashes to `24c4ef6df24054f42f626403e11e78a00c58b009970fcce360c49ee3ea91f85c`. |
| AC-1 negative: malformed JSON | `semantic.py` lines 150-154; `cli.py` lines 873-880 | Pydantic raises `ValidationError`; CLI prints `CACG-MAN-001: cannot load semantic cache: ...`; pipeline does not start. |
| AC-1 negative: duplicate `(chunk_hash, claim_window_hash)` entries | `semantic.py` lines 115-129; tests lines 74-82 | Rejected with `ValidationError` containing `semantic_cache has duplicate keys`. |
| AC-1 negative: duplicate top-level JSON object key | `semantic.py` line 154 | Current Python does not reject; observed last-write-wins. Rust must reject per plan/BitLesson. |
| AC-1 negative: duplicate key within `entries[i]` JSON object | `semantic.py` line 154 | Current Python does not reject; observed last-write-wins before validation. Rust must reject per plan/BitLesson. |
| AC-1 negative: duplicate key within any nested object | `semantic.py` line 154 | Current Python does not reject duplicate itself; unknown nested object is later rejected if under extra field. Rust must reject duplicates at raw JSON level. |
| AC-1 negative: non-hex hash field | `semantic.py` lines 82-92; tests lines 44-52 | Rejected by Pydantic field validator with `hash must be 64-hex SHA256`. |
| AC-1 negative: verdict outside enum | `semantic.py` lines 32, 84; tests lines 54-61 | Rejected by Pydantic literal validation. |
| AC-1 negative: score NaN/inf/out-of-range | `semantic.py` lines 46, 50-57, 85; tests lines 64-71 and 730-737 | Out-of-range rejected; `SemanticVerdict` has finite validator; `SemanticCacheEntry` rejects non-finite through range constraints in current Pydantic. |
| AC-1 negative: unknown top-level fields | `_StrictModel` in `schema.py` lines 43-47; `SemanticCache` lines 95-112 | Rejected due to `extra="forbid"`. |
| AC-1 negative: cache miss returns abstain | `_cache_lookup` lines 183-188; tests lines 160-170 | Returns `SemanticVerdict(verdict="abstain", score=0.0, reasoning=None, mode="embedding-cache")`; no exception. |
| AC-1.1 positive: `schema_version == "cacg.v0"` loads | `schema.py` lines 17-18; `semantic.py` line 110 | Accepted. |
| AC-1.1 negative: unknown schema version | `schema.py` lines 17-18; `semantic.py` line 110; tests lines 85-87 | Rejected by literal validation; CLI maps load failure to `CACG-MAN-001`. |
| AC-1.1 negative: unknown fields under valid schema | `_StrictModel` lines 43-47 | Rejected with `extra_forbidden`; CLI maps to `CACG-MAN-001`. |
| AC-2 positive: `--semantic` constructs B1 spec | `cli.py` lines 843-883 | Loads cache and returns `SemanticSpec(cache=cache, judge_enabled=False)`. |
| AC-2 positive: verify emits `CACG-VERIFY-002` | `layer2.py` lines 285-349; tests lines 569-615 | On Layer-2 failure with semantic cache, diagnostics include both `CACG-VERIFY-001` and `CACG-VERIFY-002`. |
| AC-2 positive: one journal event per card | `runner.py` lines 199-214; tests lines 383-448 | Layer-3 diagnostic rides in same diagnostics array as `CACG-VERIFY-001`. |
| AC-2 positive: score exactly six decimals | `layer2.py` lines 330-333 | Diagnostic message uses `score={verdict.score:.6f}`. |
| AC-2 positive: per-verdict severity | `layer2.py` lines 326-340 | Current Python: pass -> warning, fail -> error, abstain -> error. Plan says abstain -> info, but Python does not. |
| AC-2 negative: no Layer-3 when exact match passes | `layer2.py` lines 224-228; tests lines 270-327 | Exact substring pass returns before semantic branch; no `CACG-VERIFY-002`. |
| AC-2 negative: no Layer-3 when fuzzy accepts | `layer2.py` lines 227-228; tests lines 330-380 | Fuzzy pass returns before semantic branch; no `CACG-VERIFY-002`. |
| AC-2 negative: no Layer-3 on HASH mismatch | `layer2.py` lines 128-145 and 171-202 | Early returns before semantic branch. |
| AC-2 negative: no Layer-3 on source mismatch / CITE trust failure | `layer2.py` lines 147-169 | Early returns before semantic branch. |
| AC-2 negative: no Layer-3 on AUTH failure | `layer2.py` lines 390-425 | Auth diagnostics set failure before per-citation verification; semantic not invoked by that branch. |
| AC-2 negative: no Layer-3 on RETR source/chunk failure | `layer2.py` lines 426-460 | Retracted citations continue without `verify_citation`; semantic not invoked. |
| AC-2 negative: no Layer-3 on manifest load failure | `runner.py` lines 134-161 | Manifest load failure returns before `verify_card`; semantic not invoked. |
| AC-2 negative: missing semantic cache | `cli.py` lines 864-872; tests lines 618-638 | Prints `CACG-MAN-001`, returns exit 1 before verify pipeline. |
| AC-2 negative: both `--semantic` and `--semantic-judge` | `cli.py` lines 164-190; tests lines 780-798 | Argparse exits with code 2. |
| AC-2 negative: pass verdict still overall failed | `layer2.py` lines 262-270 and 326-354 | `CACG-VERIFY-001` remains error in diagnostics, so verify remains failed even if semantic verdict is pass. |

# 11. Risks / Unresolved Ambiguities for the Rust Port

## 11.1 Duplicate JSON object keys

Current Python:

- Does not reject duplicate JSON member names before materialization.
- Last-write-wins behavior was observed.
- This affects top-level objects and entry objects.

M5b requirement:

- Rust must reject duplicate JSON object keys at every level.

Risk:

- Rust will intentionally diverge from current Python for duplicate JSON member names.
- This is required by the plan and BitLesson.
- Tests should document this as a security hardening delta, not accidental byte-parity failure.

## 11.2 `abstain` severity mismatch

Current Python:

- `pass` -> `warning`.
- `fail` -> `error`.
- `abstain` -> `error`.

M5b plan:

- `pass` -> `warning`.
- `fail` -> `error`.
- `abstain` -> `info`.

Risk:

- Byte-equal Python parity conflicts with the plan.
- Resolve before implementing AC-2 tests.
- If Python is the oracle, Rust should emit `error` for abstain.
- If plan is the oracle, Python should be patched or tests should explicitly diverge.

## 11.3 Pydantic is not fully strict

Current `_StrictModel`:

- Forbids extras.
- Does not set `strict=True`.

Observed behavior:

- JSON `"score": "0.5"` is accepted and coerced to float.

Risk:

- Rust strict serde `f64` will reject numeric strings.
- This diverges from current Python.
- The plan says "strict canonical-JSON validation", so rejection may be desired.
- The spec should pin the intended behavior before writing Rust tests.

## 11.4 Unicode regex semantics

Python normalization:

- Uses Python `re` with Unicode semantics for `\s`.
- Rust `regex` crate has Unicode mode by default, but exact whitespace tables may differ across Unicode versions.
- Python version and Rust crate Unicode tables can diverge on obscure whitespace code points.

Risk:

- Golden vectors cover common cases but not every Unicode whitespace scalar.
- For byte-equality, add a small table of whitespace code points if broader coverage is required.

## 11.5 NFC implementation version

Python:

- Uses `unicodedata.normalize("NFC", text)`.
- Behavior depends on Python's bundled Unicode database version.

Rust:

- `unicode-normalization` crate uses its own Unicode data version.

Risk:

- Rare code points may differ across Unicode versions.
- Golden vectors cover common combining accent behavior, not all NFC edge cases.

## 11.6 Do not use NFKC

Temptation:

- Use NFKC to cover ligatures.

Problem:

- Python uses NFC plus a hand-written ligature table.
- NFKC normalizes more characters than Python.
- NFKC would change hashes.

Rust:

- Use NFC.
- Then apply the exact seven ligature replacements.

## 11.7 Hyphenated line-break regex

Python:

- `_HYPHEN_LINEBREAK = re.compile(r"-\s*\n\s*")`
- It requires an actual `\n`.
- It can consume whitespace before and after the newline.
- It runs before whitespace collapse.

Risk:

- Rust implementation must preserve ordering.
- Do not collapse whitespace before hyphen-linebreak rejoin.
- CRLF case:
  - `-\r\n` matches because `\s*` can consume `\r` before `\n`.

## 11.8 Optional fields and nulls

Current cache:

- `entries` default factory means missing `entries` accepted.
- `entries: null` rejected.
- `SemanticVerdict.reasoning: null` accepted.
- `SemanticCacheEntry` has no `reasoning`; entry `reasoning: null` rejected as extra.

Risk:

- Rust defaults must match:
  - Missing `entries` -> empty vector.
  - `entries: null` -> error.
  - Missing `reasoning` in verdict -> `None`.
  - `reasoning: null` in verdict -> `None`.

## 11.9 Entries object vs list

Observed Python:

- `entries: {}` rejected.
- Error type is list-type validation.
- `entries: []` accepted.
- Missing `entries` accepted.

Rust:

- Ensure `entries` is `Vec`.
- Do not accept map-shaped entries.

## 11.10 Duplicate semantic composite keys

Current Python:

- Rejected by validator.
- Error includes sorted set of duplicate tuples.

Rust:

- Must reject.
- If error text parity matters, mimic stable sorted tuple rendering.
- If only diagnostic code matters, stable Rust error text is sufficient.

## 11.11 Lookup argument validation

Current Python:

- `SemanticCache.lookup` does not validate lookup arguments.
- Non-hex `chunk_hash` lookup simply misses.
- Non-hex `claim_window_hash` lookup simply misses.
- `_cache_lookup` then returns abstain.

Risk:

- Rust should not reject lookup arguments unless the caller boundary requires it.
- For parity, invalid lookup strings should miss and abstain.

## 11.12 B2 returned mode not enforced

Current Python:

- `_judge_via_claude` returns a `SemanticVerdict`.
- `run_semantic_check` does not override `mode`.
- A monkey-patched B2 could return `mode="embedding-cache"` and Python would accept it if the model validates.

Risk:

- Rust B2 client should probably validate returned mode.
- For strict parity, do not override.
- For cleaner semantics, enforce `mode=llm-judge`.
- This is a future B2 decision.

## 11.13 Cache-entry abstain score invariant

Current Python:

- Allows any score in `[0,1]` with `verdict="abstain"` in cache entries.
- Cache miss abstain is always score `0.0`.

Risk:

- Do not add a Rust invariant requiring abstain score `0.0` unless the schema changes.
- Such an invariant would reject cache files Python accepts.

## 11.14 Filesystem path shape

Current Python:

- `_build_semantic_spec` uses `is_file()` before cache load.
- `load_semantic_cache` itself does not.

Risk:

- If Rust library `SemanticCache::load` is used directly, decide whether it should perform `is_file()`.
- CLI definitely must.
- Library-level `load` can also check `is_file()` for defense in depth, but error text should still map cleanly to `CACG-MAN-001`.

## 11.15 Pathological JSON

Current Python:

- No explicit file size limit.
- No explicit JSON depth limit in source.
- Pydantic-core has its own parser behavior.
- Very deep JSON may raise parser or recursion errors.

Rust risk:

- `serde_json` has recursion limits.
- If disabling recursion limit, stack risks increase.
- Keep default recursion safety unless canonical cache needs otherwise.
- Consider file size cap in future hardening; not current Python parity.

## 11.16 Canonical serialization and float rendering

Python:

- Uses `json.dumps`.
- Float rendering follows Python JSON encoder behavior.

Rust:

- `serde_json` float rendering may differ for edge decimal spellings.
- Scores in committed cache should use simple finite decimals.
- Golden round-trip test on committed cache is required.

## 11.17 `reasoning` repr parity

Python diagnostic:

- Uses `{verdict.reasoning!r}`.
- Python repr is not the same as Rust `Debug`.

Risk:

- If B2 tests include reasoning with quotes, backslashes, or non-ASCII, Rust message may diverge.
- For B1 M5b, reasoning is always omitted.
- Future B2 should add Python-repr-compatible formatting or avoid byte-equal reasoning messages.

# 12. Rust Pseudocode Entry Point

This sketch is not production code. It shows the required shape and validation boundaries.

```rust
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub enum SemanticError {
    NotRegularFile,
    Read(std::io::Error),
    Json(serde_json::Error),
    DuplicateJsonKey { key: String },
    UnknownSchemaVersion(String),
    InvalidHash { field: &'static str, value: String },
    InvalidScore { field: &'static str, value: f64 },
    DuplicateSemanticKey { chunk_hash: String, claim_window_hash: String },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticMode {
    #[serde(rename = "embedding-cache")]
    EmbeddingCache,
    #[serde(rename = "llm-judge")]
    LlmJudge,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SemanticVerdictKind {
    Pass,
    Fail,
    Abstain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticVerdict {
    pub verdict: SemanticVerdictKind,
    pub score: f64,
    #[serde(default)]
    pub reasoning: Option<String>,
    #[serde(default = "default_mode")]
    pub mode: SemanticMode,
}

fn default_mode() -> SemanticMode {
    SemanticMode::EmbeddingCache
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SemanticCacheEntryWire {
    chunk_hash: String,
    claim_window_hash: String,
    verdict: SemanticVerdictKind,
    score: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SemanticCacheWire {
    schema_version: String,
    #[serde(default)]
    entries: Vec<SemanticCacheEntryWire>,
}

#[derive(Clone, Debug)]
pub struct SemanticCacheEntry {
    chunk_hash: String,
    claim_window_hash: String,
    verdict: SemanticVerdictKind,
    score: f64,
}

#[derive(Clone, Debug)]
pub struct SemanticCache {
    schema_version: String,
    entries: Vec<SemanticCacheEntry>,
    index: HashMap<(String, String), SemanticCacheEntry>,
}

impl SemanticCache {
    pub fn load(path: &Path) -> Result<Self, SemanticError> {
        if !path.is_file() {
            return Err(SemanticError::NotRegularFile);
        }

        let bytes = fs::read(path).map_err(SemanticError::Read)?;

        reject_duplicate_json_object_keys(&bytes)?;

        let wire: SemanticCacheWire =
            serde_json::from_slice(&bytes).map_err(SemanticError::Json)?;

        if wire.schema_version != "cacg.v0" {
            return Err(SemanticError::UnknownSchemaVersion(wire.schema_version));
        }

        let mut semantic_keys = BTreeSet::<(String, String)>::new();
        let mut entries = Vec::with_capacity(wire.entries.len());
        let mut index = HashMap::with_capacity(wire.entries.len());

        for e in wire.entries {
            validate_hash("chunk_hash", &e.chunk_hash)?;
            validate_hash("claim_window_hash", &e.claim_window_hash)?;
            validate_score("score", e.score)?;

            let key = (e.chunk_hash.clone(), e.claim_window_hash.clone());
            if !semantic_keys.insert(key.clone()) {
                return Err(SemanticError::DuplicateSemanticKey {
                    chunk_hash: key.0,
                    claim_window_hash: key.1,
                });
            }

            let entry = SemanticCacheEntry {
                chunk_hash: e.chunk_hash,
                claim_window_hash: e.claim_window_hash,
                verdict: e.verdict,
                score: e.score,
            };

            index.insert(
                (entry.chunk_hash.clone(), entry.claim_window_hash.clone()),
                entry.clone(),
            );
            entries.push(entry);
        }

        Ok(Self {
            schema_version: "cacg.v0".to_string(),
            entries,
            index,
        })
    }

    pub fn lookup(&self, chunk_hash: &str, claim_window_hash: &str) -> SemanticVerdict {
        match self.index.get(&(chunk_hash.to_string(), claim_window_hash.to_string())) {
            Some(entry) => SemanticVerdict {
                verdict: entry.verdict.clone(),
                score: entry.score,
                reasoning: None,
                mode: SemanticMode::EmbeddingCache,
            },
            None => SemanticVerdict {
                verdict: SemanticVerdictKind::Abstain,
                score: 0.0,
                reasoning: None,
                mode: SemanticMode::EmbeddingCache,
            },
        }
    }

    pub fn to_canonical_json(&self) -> Result<String, SemanticError> {
        // Production code must use the workspace canonical JSON helper.
        // Preserve self.entries order; sort object keys; no trailing newline.
        todo!("serialize with sorted object keys and Python-compatible float rendering")
    }
}

pub fn claim_window_hash(quote: &str) -> String {
    let normalized = normalize_text_python_parity(quote);
    let mut hasher = Sha256::new();
    hasher.update(normalized.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn normalize_text_python_parity(input: &str) -> String {
    // Production code:
    // 1. Unicode NFC.
    // 2. Replace exactly the seven ligatures from normalize.py lines 12-20.
    // 3. Apply r"-\s*\n\s*" -> "" with Python-compatible Unicode whitespace.
    // 4. Apply r"\s+" -> " ".
    // 5. Strip leading/trailing whitespace.
    todo!("implement with golden vectors from Section 2")
}

fn validate_hash(field: &'static str, value: &str) -> Result<(), SemanticError> {
    let ok_len = value.len() == 64;
    let ok_hex = value
        .bytes()
        .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'));

    if ok_len && ok_hex {
        Ok(())
    } else {
        Err(SemanticError::InvalidHash {
            field,
            value: value.to_string(),
        })
    }
}

fn validate_score(field: &'static str, value: f64) -> Result<(), SemanticError> {
    if value.is_finite() && (0.0..=1.0).contains(&value) {
        Ok(())
    } else {
        Err(SemanticError::InvalidScore { field, value })
    }
}

fn reject_duplicate_json_object_keys(bytes: &[u8]) -> Result<(), SemanticError> {
    // Required by BL-20260518-reject-duplicate-keys-at-trust-boundary.
    //
    // This must observe object keys before normal map materialization.
    // Do not parse to serde_json::Value and scan afterward.
    //
    // Production approach:
    // - Drive serde_json::Deserializer with a custom Visitor or DeserializeSeed.
    // - On every JSON object, create a fresh BTreeSet<String>.
    // - For each key, reject if insert(key.clone()) returns false.
    // - Recurse into every value, including arrays and unknown-field objects.
    // - Return DuplicateJsonKey immediately with a stable path if possible.
    //
    // A two-pass approach is acceptable:
    // - Pass 1: raw duplicate-key visitor over tokens.
    // - Pass 2: typed serde deserialization into SemanticCacheWire.
    let _ = bytes;
    todo!("custom serde_json Deserializer/Visitor duplicate-key pass")
}
```
