---
schema_version: "cacg.v0"
id: "malformed-hash-card"
title: "Malformed Chunk Hash"
reading_id: "reading_01"
summary: "Phase 3 test-fixture summary populated for AC-S1 bounded-summary enforcement; this card carries a non-empty body for retrieval surface tests."
citations:
  - source_id: "sample"
    chunk_id: "sample:p001:0000"
    chunk_hash: "not-a-real-sha256-hash-at-all"
    page_range: [1, 2]
    quote: "Content-addressable identity is the verification primitive at the core of the framework."
    edge_type: "supports"
---
Adversarial: chunk_hash is not a 64-hex SHA256. Expected CACG-CITE-002.
