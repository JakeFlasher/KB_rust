---
schema_version: "cacg.v0"
id: "content-addressable-identity"
title: "Content-Addressable Identity"
reading_id: "reading_01"
summary: "Phase 3 test-fixture summary populated for AC-S1 bounded-summary enforcement; this card carries a non-empty body for retrieval surface tests."
citations:
  - source_id: "sample"
    chunk_id: "sample:p001:0000"
    chunk_hash: "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895"
    page_range: [1, 2]
    quote: "Content-addressable identity is the verification primitive at the core of the framework."
    edge_type: "supports"
card_hash: "0000000000000000000000000000000000000000000000000000000000000000"
---
Content-addressable identity gives every chunk a stable 64-hex digest and
every citation pins exactly one chunk by hash. Drift becomes a mechanical
failure rather than a semantic argument.
