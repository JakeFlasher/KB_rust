# Round Summary (round-summary semantic parity fixture)

This synthetic round summary cites a single card whose committed
quote does NOT appear in the pinned chunk text, so Layer-2 emits
`CACG-VERIFY-001` and the Layer-3 cache lookup fires. The
committed `tests/parity_corpus/semantic/semantic_cache.json` pins
a deterministic `fail` verdict for the resulting
`(chunk_hash, claim_window_hash)` pair, which both Python and Rust
report on stderr as `CACG-VERIFY-002` severity = error. This row
is the round-summary batch counterpart of the existing single-card
`kb_verify_semantic_parity_golden` row.

## Knowledge Consulted

- tests/parity_corpus/semantic/card.md -- exercises the round-summary batch path's Layer-3 cache lookup
