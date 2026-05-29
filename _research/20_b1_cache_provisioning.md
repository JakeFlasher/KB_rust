# B1 Semantic Cache Provisioning

_Sub-milestone M5b-b Phase F (committed QM cache + builder + ceremony + idempotence)._

This document covers the cache-rebuild ceremony for the committed
QM B1 semantic cache (`out/semantic_cache.json`) and its
provenance sidecar (`out/semantic_cache.provenance.json`). The
cache is the frozen lookup table the runtime
`kb verify --semantic <cache>` reads on Layer-3 firing; the
embedding model is NEVER invoked at runtime.

## 1. Inputs (frozen)

| Input | Pin | Reason |
|-------|-----|--------|
| Embedding model | `sentence-transformers/all-MiniLM-L6-v2` @ HuggingFace revision SHA `c9745ed1d9f207416be6d2e6f8de32d1f16199bf` | A specific revision SHA, not just the model name, so a future HuggingFace re-upload cannot silently change the weights. |
| `sentence-transformers` Python package | `==5.5.1` (in `pyproject.toml` `cache-build` extra) | Pinned EXACTLY so two canonical-environment rebuilds produce byte-identical cache output. A version bump requires a deliberate rebuild + new commit. |
| Canonical environment | Linux x86_64 | Verified at build time via `os.uname().sysname == "Linux"` AND `os.uname().machine == "x86_64"`. Other environments hard-error unless `--force-non-canonical` is supplied. |
| Paraphrase annotation pool | The 222 Round-27-frozen QM paraphrase annotations | Extracted by reusing `qm_layer3_capacity_sample.extract_annotations` over the sibling `/home/jakeshea/CFA_reading/.claude/knowledge/01_quantitative_methods/qm-*.md` corpus. The extractor + sort key are deterministic. |
| Chunks manifest | `tests/parity_corpus/out_python/qm_vertical/chunks_manifest.json` | Committed in the repo; the cache pins chunk_hash values against this manifest's chunks. |
| Negative fixtures | 5 hardcoded `(chunk_id, claim_text)` tuples in `scripts/build_semantic_cache.py::NEGATIVE_FIXTURES` | Within the plan budget of 5-10; pinned in source so the negative-fixture set is auditable from the same commit as the rebuild. |
| Decision threshold | Default `--threshold 0.5` | Placeholder until the MiniLM threshold calibration sweep locks the value. The cache verdict (`pass` if score >= threshold else `fail`) flips when the threshold changes, so re-running with a different threshold produces a different `out/semantic_cache.json` and a different Hash C. |

## 2. Outputs

| Path | Content | Schema |
|------|---------|--------|
| `out/semantic_cache.json` | 227 entries (222 paraphrase + 5 negatives), one per `(chunk_hash, claim_window_hash)` pair, sorted lexicographically | `cacg.v0` (matches `cacg_semantic::SemanticCache` + `cacg.verify.semantic.SemanticCache`) |
| `out/semantic_cache.provenance.json` | `hash_b`, `hash_b_components`, `hash_c`, model + threshold metadata, optional `force_non_canonical_override` flag | `cacg.v0` (internal layout; the only consumer is the audit pipeline) |

The cache file is ~45 KB committed (within the plan's `~45-60 KB`
budget). Entries are sorted by `(chunk_hash, claim_window_hash)`
so the on-disk byte order is stable under any shuffled input set.
Scores are rounded to 6 decimal places before serialization to
bound float-representation drift across PyTorch builds.

## 3. Two-hash provenance contract

The provenance JSON commits two SHA-256 hashes:

### Hash B — Canonical-environment binding

SHA-256 over a length-prefixed concatenation, in fixed field
order, of:

1. Raw `uv.lock` bytes.
2. `os.uname().machine` (UTF-8).
3. `os.uname().release` (UTF-8).
4. `python_version` (UTF-8, format `X.Y.Z`).
5. `sentence_transformers.__version__` (UTF-8).

Each field is prefixed by an 8-byte big-endian length; this
prevents field-boundary collisions where two different field
splits could otherwise concatenate to the same byte string.

A bump in any of these five inputs changes Hash B. Sample audit
output:

```jsonc
{
  "hash_b_components": {
    "uv_lock_sha256": "<lowercase-hex>",
    "uname_machine": "x86_64",
    "uname_release": "<kernel-release>",
    "python_version": "3.14.4",
    "sentence_transformers_version": "5.5.1"
  }
}
```

### Hash C — Cache content commitment

SHA-256 over the canonical-JSON bytes of `out/semantic_cache.json`
(sort_keys=True, separators=(",", ":"), ensure_ascii=False,
UTF-8-encoded, trailing newline). A single-entry mutation
(modifying any chunk_hash, claim_window_hash, verdict, or score)
changes Hash C.

### Deliberate omissions (looser contract)

Per the user-ratified looser provenance contract, per-file MiniLM
artifact SHAs are NOT included in Hash B. Model identity is
anchored at the HuggingFace revision SHA. Risk:
`huggingface_hub` could in principle serve different bytes under
the same revision tag; the project accepts this risk because the
revision SHA itself is content-addressable on HuggingFace's side
and the deviation has not been observed in practice. Tracked in
the plan's §Residual Risks (R4).

## 4. Rebuild ceremony

### Prerequisites

1. **Sibling repo** at
   `/home/jakeshea/CFA_reading/.claude/knowledge/01_quantitative_methods/`
   (the legacy QM annotation corpus). The builder hard-errors
   if absent.
2. **Canonical Linux x86_64 environment**. macOS arm64 / Windows
   require `--force-non-canonical` and the resulting cache will
   NOT byte-match a canonical rebuild (logged + recorded).
3. **Network access** to HuggingFace Hub on first run (model
   download). Subsequent runs use the local HF cache.
4. **`cache-build` Python extra installed**:

   ```sh
   UV_CACHE_DIR=/tmp/uv-cache uv sync --extra cache-build
   ```

### Build

```sh
unset all_proxy  # if a SOCKS proxy is set without socksio
UV_CACHE_DIR=/tmp/uv-cache uv run --extra cache-build \
    python scripts/build_semantic_cache.py
```

The builder writes `out/semantic_cache.json` and
`out/semantic_cache.provenance.json`. A successful run logs:

```
INFO: sentence-transformers==5.5.1
INFO: model=sentence-transformers/all-MiniLM-L6-v2 @ revision c9745ed1d9f207416be6d2e6f8de32d1f16199bf
INFO: threshold=0.5
INFO: extracted 222 paraphrase annotations from <legacy-dir>
INFO: wrote out/semantic_cache.json (<bytes> bytes, <count> entries) in <s>
INFO: wrote out/semantic_cache.provenance.json (hash_b=..., hash_c=...)
```

Wall-clock is ~7s on a typical Linux x86_64 host (CPU inference;
GPU is unnecessary at 222 paraphrase + 5 negative fixture pairs).

### Commit

Stage and commit:

```sh
git add out/semantic_cache.json out/semantic_cache.provenance.json
git commit -m "regenerate B1 semantic cache (<reason>)"
```

The cache + provenance always commit together. A drift between
the committed cache's Hash C and the provenance's `hash_c` field
is a malformed commit and is caught by the
`xtask audit-semantic-cache-provenance` command:

```sh
cargo xtask audit-semantic-cache-provenance
# expect: "227 entries clean (222 paraphrase + 5 negative), hash_b=…, hash_c=…"
```

The audit recomputes Hash B (from the committed `uv.lock` bytes
plus the four `hash_b_components` strings the builder pinned at
rebuild time) and Hash C (from the committed cache bytes),
cross-checks `hash_b_components.uv_lock_sha256` against
`sha256(uv.lock)`, enforces the frozen 222-paraphrase +
5-negative-fixture count contract, and verifies the pinned
model name + revision SHA. It does NOT re-run the embedding
model and does not need Python or `sentence-transformers`
installed.

## 5. When to rebuild

| Trigger | Action |
|---------|--------|
| `pyproject.toml` `cache-build` extra version pin bumped | Rebuild required; Hash B changes. |
| `uv.lock` regenerated (any pin change anywhere) | Rebuild required; Hash B changes. |
| Decision threshold changed (calibration sweep locks a new value) | Rebuild required; Hash C changes because verdicts flip. |
| Negative fixtures edited (`NEGATIVE_FIXTURES` in the builder) | Rebuild required; Hash C changes. |
| New paraphrase annotation added to the legacy QM corpus | Rebuild required; Hash C changes. The 222-count is FROZEN against Round-27 labels per the plan; any deviation flips the count and the audit gate. |
| `chunks_manifest.json` regenerated (rare; M4-frozen) | Rebuild required; new chunk_hash values mean cache keys move. |
| Routine `cargo` / `cargo-deny` updates that don't change `uv.lock` | No rebuild. |

## 6. Verification

After a rebuild:

1. **Byte-equal idempotence**: run the builder a second time and
   `diff` the cache + provenance. Both must be byte-identical.

   ```sh
   sha256sum out/semantic_cache.json out/semantic_cache.provenance.json > /tmp/before
   uv run --extra cache-build python scripts/build_semantic_cache.py
   sha256sum out/semantic_cache.json out/semantic_cache.provenance.json > /tmp/after
   diff /tmp/before /tmp/after  # must be empty
   ```

2. **Provenance audit**: run
   `cargo xtask audit-semantic-cache-provenance`. This is the
   load-bearing gate — it enforces Hash B + Hash C + the
   frozen 222 + 5 = 227 count contract + the pinned model
   identity. A successful audit prints the entry counts and
   truncated hashes.

3. **Hash consistency** (quick sanity, redundant with the
   audit): the provenance JSON's `hash_c` must match
   `sha256sum out/semantic_cache.json | cut -c1-64`.

4. **Runtime load**: the cache must load cleanly through
   `cacg.verify.semantic.load_semantic_cache` (Python — see
   `tests/test_semantic_cache_provisioning.py`) AND through
   `cacg_semantic::SemanticCache::load` (Rust — see
   `crates/cacg-semantic/tests/committed_cache.rs`). The Rust
   suite additionally asserts the round-trip
   `to_canonical_json` is byte-equal to the committed file.

## 7. References

- `scripts/build_semantic_cache.py` — the builder itself.
- `tests/test_semantic_cache_provisioning.py` — idempotence +
  shape + Hash C change-detection tests.
- `scripts/qm_layer3_capacity_sample.py::extract_annotations` —
  the shared annotation extractor (the 222-count source of
  truth).
- `_research/qm_paraphrase_layer3_capacity.md` — Round-27
  capacity sweep that froze the 222 count.
- The cache schema lives in `cacg-semantic`
  (`crates/cacg-semantic/src/lib.rs::SemanticCache`) and in
  Python (`src/cacg/verify/semantic.py::SemanticCache`). Both
  validate the schema_version literal `cacg.v0`, the 64-hex
  hash format, and the score range `[0.0, 1.0]` at load time.
