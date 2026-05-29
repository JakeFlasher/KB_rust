---
schema_version: "cacg.v0"
id: "chunk-hash-drift-card"
title: "Chunk Hash Drift"
reading_id: "reading_01"
summary: "Phase 3 test-fixture summary populated for AC-S1 bounded-summary enforcement; this card carries a non-empty body for retrieval surface tests."
citations:
  - source_id: "sample"
    chunk_id: "sample:p001:0000"
    chunk_hash: "0000000000000000000000000000000000000000000000000000000000000001"
    page_range: [1, 2]
    quote: "Content-addressable identity is the verification primitive at the core of the framework."
    edge_type: "supports"
---
Adversarial: chunk_hash is 64-hex but does not match the manifest hash for
sample:p001:0000. Expected CACG-HASH-001.
