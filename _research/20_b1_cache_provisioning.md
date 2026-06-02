# B1 Semantic Cache Provisioning

This document records the current operating contract for the committed QM B1
semantic cache:

- `out/semantic_cache.json`
- `out/semantic_cache.provenance.json`

The cache is a frozen lookup table. Runtime verification never runs an
embedding model, never imports Python, and never downloads model weights.

## Current Contract

`cargo run -p xtask -- audit-semantic-cache-provenance` is the active gate. It
checks:

- the cache content hash recorded in the provenance sidecar;
- the frozen entry count: 222 paraphrase fixtures + 5 negative fixtures = 227;
- the pinned model identity and HuggingFace revision;
- the locked decision threshold, currently `0.5`.

Hash B remains in the provenance sidecar as historical build metadata. The
current audit intentionally does not recompute Hash B from `uv.lock`; the
Python builder and root Python lockfile have been retired.

## Rebuild Policy

There is no supported in-repository Python rebuild path. A future regeneration
must be a deliberate migration task that either introduces a maintained Rust
builder or documents an external one-off builder, then commits both cache files
together and updates the audit contract in the same change.

Any regeneration must preserve or deliberately update:

- cache schema version `cacg.v0`;
- model name `sentence-transformers/all-MiniLM-L6-v2`;
- model revision `c9745ed1d9f207416be6d2e6f8de32d1f16199bf`;
- threshold `0.5`, unless `_research/qm_layer3_threshold_sweep.md` is updated;
- the 222 + 5 fixture-count contract, unless the semantic-eval fixture set is
  deliberately revised.

After a regeneration, run:

```bash
cargo run -p xtask -- audit-semantic-cache-provenance
cargo test -p cacg-semantic --test committed_cache
cargo run -p xtask -- semantic-eval
```

## References

- `xtask/src/semantic_cache_provenance.rs`
- `xtask/src/threshold_sweep.rs`
- `crates/cacg-semantic/src/lib.rs`
- `tests/semantic_eval/eval_cases.json`
- `_research/qm_layer3_threshold_sweep.md`
