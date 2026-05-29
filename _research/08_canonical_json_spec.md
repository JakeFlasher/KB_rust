# CACG Canonical JSON Specification

**Status:** stable.
**Date:** 2026-05-20.
**Plan reference:** trust-kernel-first Rust port plan, canonicalization-spec acceptance criterion.

This spec pins the byte-level behavior of CACG's canonical JSON writer. Every persisted CACG artifact (manifests, journal events, history events, search output, semantic cache, source matrix) is serialized through this writer. Two implementations of the spec (Python and Rust) MUST produce byte-identical output for any input in the CACG artifact JSON domain (defined in §2). The spec is the trust boundary that downstream byte-equal parity gates verify against.

---

## 1. Goal

Pin every byte-level behavior of Python `json.dumps(value, sort_keys=True, separators=(",",":"), ensure_ascii=False)` for inputs drawn from the CACG artifact JSON domain. Two clean-room implementations of the spec — one in Python (the current `cacg.hash.canonical_json`) and one in Rust (the future `cacg_core::canonical_json::canonical_json`) — produce byte-identical output for every valid input.

The downstream contract:

```python
# Python
import json
def canonical_json(obj):
    return json.dumps(obj, sort_keys=True, separators=(",",":"), ensure_ascii=False)
```

```rust
// Rust
pub fn canonical_json(value: &serde_json::Value) -> String { /* implements this spec */ }
```

Byte-equality is asserted across implementations for every fixture committed under `tests/parity_corpus/canonical_json/manifest.json`. The Python implementation is the ORACLE: Rust must match Python's output bytes, not the other way around.

---

## 2. Scope: The CACG Artifact JSON Domain

The CACG artifact JSON domain is the closed set of JSON shapes any CACG persisted artifact may contain. It is INTENTIONALLY narrower than RFC 8259 JSON.

### 2.1 Admissible Types

A canonical JSON document is recursively one of:

- `null`
- `bool` (literal `true` or `false`)
- Integer (Python `int`, Rust `i64` / `u64` as appropriate; serialized as `str(int)`)
- Float — admissible only in two pinned fields (`latency_ms`, `score`) per §2.3.
- UTF-8 string
- Ordered array of admissible types
- String-keyed object (mapping of UTF-8 string to admissible type)

### 2.2 Forbidden Forms

These MUST be rejected at construction time with a typed `CanonicalError`:

- `NaN`, `+Infinity`, `-Infinity` (Python `json.dumps` emits these as `NaN`/`Infinity` literals which are NOT valid JSON; the canonical writer rejects them outright).
- Non-string object keys (Python `json.dumps` coerces ints to strings; the canonical writer rejects to surface input-shape errors at the call site).
- Surrogate halves U+D800..U+DFFF outside a valid UTF-16 surrogate pair.
- Duplicate object keys.
- Unsupported types: bytes, datetime, Decimal, set, custom objects. Pre-convert via Pydantic `model_dump(mode="json")` or Serde `Serialize` upstream.

### 2.3 Float Subdomain

Float rendering is the most fragile part of the byte-equal contract. The canonical writer admits floats ONLY in these pinned fields:

- `LintEvent.latency_ms`, `HistoryEvent.latency_ms` — under `KB_FROZEN_CLOCK=1`, ALWAYS `0.0`. Outside frozen-clock mode, `latency_ms` is EXCLUDED from byte-equal parity gates because Python `repr(float)` and Rust formatters may differ on non-round values; CI parity runs under `KB_FROZEN_CLOCK=1` where `latency_ms = 0.0` everywhere.
- `Diagnostic.hints[i].score`, `SemanticVerdict.score`, `SearchHit.score`, `Bm25Hint.score` — pre-rounded to 6 decimals at the call site (`round(value, 6)` in Python; the Rust call site MUST round identically before invoking the canonical writer).

No other float fields are admissible. Adding a new float field requires a spec amendment.

---

## 3. Byte-Level Rules

### 3.1 Top-Level Output

Single line of bytes with NO trailing newline. Multi-line JSONL artifacts concatenate canonical JSON lines with literal LF (`\n`) separators.

### 3.2 Whitespace

No whitespace between tokens. Python equivalent: `separators=(",",":")`. Rust equivalent: custom writer emits `,` between array/object items and `:` between key/value pairs with no padding.

### 3.3 Key Ordering

Object keys are sorted by Python `sorted()` over the keys' default string ordering, which is codepoint-by-codepoint. Rust MUST replicate codepoint-by-codepoint ordering:

```rust
let mut entries: Vec<_> = map.iter().collect();
entries.sort_by(|a, b| a.0.cmp(b.0));  // String::cmp is codepoint-by-codepoint
```

Verified byte-equal cases:
- Plain ASCII: `["a", "z", "A"]` sorts to `["A", "a", "z"]` (uppercase before lowercase via codepoint).
- Unicode: `["α", "a"]` sorts to `["a", "α"]` (Latin lowercase a = U+0061; Greek alpha = U+03B1).
- Numeric strings: `["10", "2"]` sorts to `["10", "2"]` (lexicographic, not numeric).

### 3.4 Escape Policy

Strings emit literal UTF-8 bytes EXCEPT for these required escapes:

| Codepoint | Escape | Rationale |
|-----------|--------|-----------|
| U+0022 `"` | `\"` | Required to terminate string |
| U+005C `\` | `\\` | Required to escape escape character |
| U+0008 (BS) | `\b` | Python short form |
| U+0009 (HT) | `\t` | Python short form |
| U+000A (LF) | `\n` | Python short form |
| U+000C (FF) | `\f` | Python short form |
| U+000D (CR) | `\r` | Python short form |
| U+0000..U+0007, U+000B, U+000E..U+001F | `\u00XX` (4-hex lowercase) | Python default `ensure_ascii=False` still escapes these |
| U+007F..U+10FFFF | LITERAL UTF-8 BYTES | `ensure_ascii=False` passes through |

NOT escaped (passes through as literal UTF-8 bytes):
- `/` (forward slash) — `serde_json`'s default escapes this; the canonical writer does NOT.
- U+007F DELETE — neither Python `json.dumps(ensure_ascii=False)` nor the canonical writer escapes this.
- U+2028 LINE SEPARATOR, U+2029 PARAGRAPH SEPARATOR — emitted as literal UTF-8 bytes (`\xe2\x80\xa8` and `\xe2\x80\xa9`). Documented divergence from RFC 8259 §7's recommendation, mirroring Python.

### 3.5 Unicode Normalization

Canonical JSON does NOT normalize Unicode. Input strings are emitted as-is. Upstream normalization is the caller's responsibility (`cacg_core::normalize::normalize_text` for any field that flows into a hash envelope).

### 3.6 Integer Rendering

Integers render as `str(int)` with no leading zeros, no thousands separators, no exponent form. Leading `-` for negatives.

| Value | Canonical bytes |
|-------|-----------------|
| `0` | `0` |
| `-1` | `-1` |
| `42` | `42` |
| `1099511627776` (2^40) | `1099511627776` |
| `9223372036854775807` (i64::MAX) | `9223372036854775807` |
| `18446744073709551615` (u64::MAX) | `18446744073709551615` |

Range: Rust `u64` (0..18446744073709551615). The spec is byte-equal across this range. Values outside this range fall outside the CACG artifact JSON domain and are forbidden.

### 3.7 Float Rendering (Python-Oracle, Pinned by Fixture Suite)

**Python is the oracle. Rust MUST produce bytes IDENTICAL to Python's `json.dumps(x)` for every float in the float subdomain.** The Rust writer MUST NOT default to `f64::to_string()` or `format!("{}", x)` — both produce bytes that diverge from Python on common values (see "Rust implementation guidance" below for verified divergences).

#### Python `json.dumps` float formatter behavior

Python's `json.dumps(x)` calls `float.__repr__(x)`, which since CPython 3.1 uses David Gay's `dtoa` algorithm to produce the SHORTEST string that round-trips to the same `float` value. Concretely:

- `repr(0.0)` = `"0.0"` — always has trailing `.0` for whole floats.
- `repr(-0.0)` = `"-0.0"` — negative zero is distinguished.
- `repr(1.0)` = `"1.0"`, `repr(2.0)` = `"2.0"`, `repr(-1.0)` = `"-1.0"`.
- `repr(0.5)` = `"0.5"`, `repr(0.1)` = `"0.1"`, `repr(0.123457)` = `"0.123457"`.
- `repr(1e-06)` = `"1e-06"` — exp form for `abs(x) < 1e-4`.
- `repr(1e-05)` = `"1e-05"`.
- `repr(0.0001)` = `"0.0001"` — boundary: this stays in decimal form (matches `1e-4` exactly).
- `repr(9999.999999)` = `"9999.999999"` — six-decimal precision preserved.
- `repr(1e16)` = `"1e+16"`, but our domain has no such large floats.
- Exponential form uses lowercase `e` and a `+`/`-` sign and ≥ 2-digit padded exponent (`1e-06` not `1e-6`).

#### Required byte-stable cases (must match exactly between Python and Rust)

Every Rust canonical-JSON implementation MUST emit these exact bytes for these exact `f64` inputs. The fixture suite in §6.1 enforces this.

| f64 input | Required canonical bytes | Notes |
|-----------|--------------------------|-------|
| `0.0` | `0.0` | Trailing `.0` required |
| `-0.0` | `-0.0` | Sign preserved |
| `1.0` | `1.0` | Trailing `.0` |
| `-1.0` | `-1.0` | Trailing `.0` |
| `2.0` | `2.0` | Trailing `.0` |
| `0.5` | `0.5` | Decimal form |
| `0.123457` | `0.123457` | 6-decimal `score` example |
| `0.0001` | `0.0001` | Boundary stays in decimal form |
| `0.00001` | `1e-05` | Exp form below 1e-4 |
| `0.000001` | `1e-06` | Exp form, 6-decimal `score` lower-bound |
| `1e-05` | `1e-05` | Same as `0.00001` |
| `1e-06` | `1e-06` | Same as `0.000001` |
| `9999.999999` | `9999.999999` | Six-decimal preserved |

#### Rust implementation guidance

Rust `f64::to_string()` is NOT byte-equal to Python `repr(float)`. Concrete divergences verified locally:

- `1e-6_f64.to_string()` = `"0.000001"` (Rust) vs `repr(1e-06)` = `"1e-06"` (Python).
- `2.0_f64.to_string()` = `"2"` (Rust) vs `repr(2.0)` = `"2.0"` (Python).
- `(-0.0_f64).to_string()` = `"-0"` (Rust) vs `repr(-0.0)` = `"-0.0"` (Python).

The Rust implementation has two viable paths:

1. **Hand-rolled Python-`repr`-compatible formatter** (~150 LoC): wraps the `ryu` crate's shortest-representation output and applies the Python-specific cosmetic rules (force trailing `.0` for whole floats; switch to exp form at `abs(x) < 1e-4`; lowercase `e` with `+`/`-` sign and 2-digit padded exponent). The fixture suite enforces every case above.
2. **Fixture-proven `ryu` invocation**: `ryu::Buffer::format(x)` produces the shortest round-trip string. The fixture suite is the gate: if a `ryu` invocation produces byte-equal output to Python for every fixture, the Rust implementation may delegate to `ryu`; if any case diverges, the implementation post-processes ryu's output OR falls back to a hand-rolled formatter for the divergent class.

The fixture suite is authoritative. The Rust implementation passes when EVERY fixture in §6.1 round-trips byte-equal. There is no other test.

### 3.8 Bool Rendering

`true` and `false`. Lowercase. No `True`/`False`.

### 3.9 Null Rendering

`null`. Lowercase. No `None`/`Null`/`NULL`.

### 3.10 Array Rendering

`[item0,item1,item2]`. No leading/trailing whitespace, no whitespace around commas. Empty array renders as `[]`.

### 3.11 Object Rendering

`{"key0":value0,"key1":value1}`. Keys sorted per §3.3. No leading/trailing whitespace, no whitespace around colons or commas. Empty object renders as `{}`.

---

## 4. Rejection-at-Construction Rules

The canonical writer MUST reject these inputs with a typed `CanonicalError`. Silent coercion is forbidden — the call site must pre-validate or accept the error.

| Input | Rejection |
|-------|-----------|
| Float that is `NaN`, `+Infinity`, `-Infinity` | `CanonicalError::NonFiniteFloat` |
| Object key that is not a UTF-8 string | `CanonicalError::NonStringKey` |
| Object containing duplicate keys | `CanonicalError::DuplicateKey { key }` |
| String containing an unpaired surrogate half (U+D800..U+DFFF) | `CanonicalError::UnpairedSurrogate { codepoint }` |
| Unsupported type (bytes, datetime, set, custom object) | `CanonicalError::UnsupportedType { type_name }` |

Errors are raised at the writer entry point; no partial-output bytes are emitted.

---

## 5. Compatibility Posture

DEC-1 from the plan was set to `PROPOSED-DEFAULT: PY-IS-ORACLE for M0-M5` in `_research/09_dec_proposed_defaults.md` (the durable decisions record). The canonical writer preserves Python's bytes byte-for-byte for every valid input. Forbidden inputs are rejected at construction rather than emitting Python-compatible-but-non-RFC-compliant bytes (NaN/Infinity, non-string keys, duplicate keys via reserialize).

The user MAY override DEC-1 to `SPEC-IS-ORACLE` (Rust diverges from Python in documented ways) or `FIX-AND-PIN` (Rust fixes specific Python bugs with a documented migration); the override flow is documented in `_research/09_dec_proposed_defaults.md`.

---

## 6. Test Plan

### 6.1 Adversarial Fixture Suite

Committed at `tests/parity_corpus/canonical_json/manifest.json` as a single JSON file mapping fixture names to `{"input": <value>, "expected": "<bytes>"}` entries. At least 200 fixtures across:

- ASCII baseline (strings, ints, bools, nulls).
- Object key ordering (≥ 10 fixtures with explicitly-reordered input keys).
- Escape policy: 1 fixture per row of §3.4's escape table, plus `"`, `\`, `/`, U+007F, U+2028, U+2029.
- Unicode planes: ≥ 50 codepoints across BMP / SMP / SIP / SSP.
- Integer edge cases: 0, ±1, ±2^31, ±2^32, ±2^63, u64::MAX.
- Float byte-stable cases per §3.7 (every row of that table).
- Nested structures (≥ 5 levels).
- Empty containers.

Fixtures generated by `scripts/build_parity_corpus.py`; expected bytes computed via the existing Python `cacg.hash.canonical_json`. Re-running the generator on a green tree produces byte-identical manifest output (idempotence verified by the parity-gate workflow).

### 6.2 Reject Fixture Suite

Committed at `tests/parity_corpus/canonical_json_reject/manifest.json`. One fixture per `CanonicalError` variant (§4). The validator script asserts the Python writer raises the expected error type; the future Rust writer must raise the equivalent typed error.

### 6.3 Property Tests

`proptest` (Rust) and `hypothesis` (Python, optional) shrink-tested:

- For all valid `JsonValue` inputs `v`: `canonical_json(parse_json(canonical_json(v))) == canonical_json(v)` (round-trip stability).
- For all integers `i` in `[0, u64::MAX]`: `canonical_json(i)` matches `str(i)`.
- For all `score` floats `s` in `[0.0, 1.0]` rounded to 6 decimals: `canonical_json(s)` matches Python `repr(round(s, 6))`.

### 6.4 Cross-Implementation Parity Gate

`scripts/validate_canonical_json_parity.py` reads `tests/parity_corpus/canonical_json/manifest.json`, runs the Python canonical writer on each `input`, and asserts the output bytes equal the committed `expected` bytes. Exit 0 on parity, 1 on any byte-diff. Wired into `.github/workflows/parity.yml` as the merge-block. Once the Rust workspace lands, the workflow also invokes `cargo xtask parity --module canonical_json` and asserts byte-equal output from both implementations.

---

## 7. Open Questions

- **OQ-1:** dir-fsync after writing canonical JSON. Not the canonical writer's concern; covered by `cacg_core::atomic_publish` per the atomic-publish acceptance criterion.
- **OQ-2:** Cycles in inputs. Pydantic / Serde reject at construction; canonical writer raises `CanonicalError::CircularReference` if a cycle ever reaches it.
- **OQ-3:** New float-bearing fields. No new float field is admitted without an explicit spec amendment.

---

## 8. References

- Python reference implementation: `src/cacg/hash.py:canonical_json` (102 LoC including imports).
- Rust target implementation: `crates/cacg-core/src/canonical_json.rs` (lands when the trust kernel implementation begins).
- David Gay `dtoa` (Python's float-to-string algorithm): <https://www.netlib.org/fp/dtoa.c>.
- `ryu` crate (Rust shortest-float-representation): <https://docs.rs/ryu>. Fixture-proven equivalence required before use; cosmetic post-processing covers Python-specific divergences.
- RFC 8259 JSON: <https://www.rfc-editor.org/rfc/rfc8259>. The canonical writer is a strict subset of RFC 8259 with the documented `U+2028`/`U+2029` literal-passthrough divergence.
- RFC 8785 JSON Canonicalization Scheme (JCS): NOT applicable. JCS has incompatible escape rules and a different number canonicalization.
- Trust-kernel-first Rust port plan at `.humanize/.humanize/plans/cacg-rust-port-trust-kernel-first-plan.md`.
- Durable decisions record at `_research/09_dec_proposed_defaults.md`.
