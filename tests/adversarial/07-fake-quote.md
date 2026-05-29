---
schema_version: "cacg.v0"
id: "fake-quote-card"
title: "Fake Quote"
reading_id: "reading_01"
summary: "Phase 3 test-fixture summary populated for AC-S1 bounded-summary enforcement; this card carries a non-empty body for retrieval surface tests."
citations:
  - source_id: "sample"
    chunk_id: "sample:p001:0000"
    chunk_hash: "1694a2598d8402ee06f8c0be6a1defd8a3f85b2d7f896c25fa67b8c671a37895"
    page_range: [1, 2]
    quote: "This sentence does not appear anywhere in the source PDF text."
    edge_type: "supports"
---
Adversarial: chunk_id, chunk_hash, and page_range are all correct, so layer-1
lint passes. The quoted text never appears in the pinned chunk, so layer-2
verification must fail with CACG-VERIFY-001 plus BM25 hint_only=true hints.
